#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! The desktop shell: a window, a canvas, and the transport that puts frames in it.
//!
//! Everything about how a frame is *made* lives in `anime_compositor`. What lives here is the
//! part that could not be checked by a fixture — a window, a wall clock, and a request. The
//! division is document 06's ("the rendering core stays independent of the interface") and cargo
//! enforces it: the arrow points this way and there is no way back.
//!
//! Two decisions are implemented rather than described:
//!
//! - **D-36.** Frames reach the webview over a custom URI scheme, not over IPC. A frame is an
//!   ordinary resource with a content type; the IPC channel stays free for commands, so a frame
//!   in flight cannot delay a stop or a scrub.
//! - **D-32.** The wall clock is in the page and the *decision* is in the core. The page asks
//!   "what belongs on screen at this instant", `Playback::at` answers, and the frames between
//!   that answer and the last one are counted as skipped rather than shown late. The page never
//!   computes a frame number.
//!
//! And D-33's indicator is a header on every response, derived from the quality that rendered
//! the frame, so it cannot describe a resolution other than the one on screen.
//!
//! A project reaches the window four ways: named on the command line, dropped on it, chosen in
//! the Open dialog, or picked out of the recent list. All four go through [`open`], which is
//! `persist::load` and nothing else, so what the viewer says about a project is what the core
//! said about it — including everything document 28 asks to be told.
//!
//! It goes back out through [`write`], which is `persist::save` and nothing else, so the atomic
//! replacement SP-01 measured and the unknown-data preservation document 28 requires are the
//! core's, not a second implementation of them living in a window. The shell holds the
//! [`Document`] and its [`Preserved`] rather than a copy of the project, for one reason: a
//! window that saved a project parsed into the parts this build understands would quietly drop
//! the parts it does not, and the person would find out when their masks were gone.
//!
//! The commands are a second custom scheme, `http://project.localhost`, beside the frame one.
//! Not IPC: the frame scheme is already proven in this window and needs no permission surface,
//! and a command here is a request with an answer, which is the shape a fetch already has. IPC
//! stays free, which is what D-36 wanted it for.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::{Diagnostic, DiagnosticId, FrameLog, Severity};
use anime_compositor::effects::{Effect, EffectInstance, EXPOSURE, GAUSSIAN_BLUR, TINT};
use anime_compositor::export::{self, ExportReport, ExportRequest, ExportStatus, MissingSource};
use anime_compositor::media;
use anime_compositor::model::{Asset, Composition, Id, Interp, Layer, Project, Prop, Value};
use anime_compositor::persist::{self, Preserved};
use anime_compositor::preview::{self, Playback, PreviewQuality};
use anime_compositor::time::{ExposureSpan, FrameRate};
use anime_compositor::{AlphaMode, ColorSpace};
use anime_compositor::{OutputAlpha, OutputDepth};
use tauri::http::{Request, Response};
use tauri::{AppHandle, DragDropEvent, Manager, WindowEvent};
use tauri_plugin_dialog::DialogExt;

/// What the shell is looking at.
///
/// One project, one composition, one clock, and whatever the core had to say when it was
/// opened.
struct Viewer {
    /// The project and its undo history, as the core models it. Held whole rather than as a
    /// `Project` so that saving is `persist::save` of the thing that was loaded.
    document: Document,
    /// Everything in the file this build does not model — masks, effects, anything a later
    /// version writes. Opaque here on purpose; the shell's only dealing with it is handing it
    /// back to `persist::save` unread.
    preserved: Preserved,
    /// Where Save writes. `None` until the project has a file of its own, which is the state
    /// [`demo`] is in and the reason Save falls through to Save As rather than overwriting
    /// something in `verification/`.
    path: Option<PathBuf>,
    /// The directory asset paths resolve against. Normally the project file's own directory,
    /// which is the rule `persist::load` checks media against — see [`demo`] for the one
    /// exception and why it exists.
    root: PathBuf,
    composition: Id,
    quality: PreviewQuality,
    playback: Playback,
    /// What to call the open project on screen.
    name: String,
    /// Document 28's warnings from opening it, verbatim, or empty when there were none.
    /// Never summarised and never dropped: a project that opened with missing media is not the
    /// same project as one that opened cleanly, and the window has to say which one is on
    /// screen.
    notes: Vec<String>,
    /// What the last command did, in one sentence, or empty. Separate from [`notes`](Self::notes)
    /// because a warning is about the project and this is about the last thing the person asked
    /// for: "Saved to shot.json" is not something wrong with the project, and a save that failed
    /// must not be filed away among warnings that were already there.
    status: String,
    /// The recovery snapshots found beside this project when it was opened, newest first, or
    /// empty. Read once at open rather than on every frame: a header is written for every frame
    /// and asking the file system five times a frame for files that change every two minutes
    /// would be a cost paid sixty times a second for nothing.
    recovery: Vec<PathBuf>,
    /// What the autosave timer last did, in one sentence, or empty. Its own line rather than
    /// [`status`](Self::status), which belongs to the last thing the *person* asked for: an
    /// autosave happens on a clock while somebody is doing something else, and overwriting
    /// "Saved to shot.json" with it would take away the answer they were waiting for.
    autosaved: String,
    /// When this document first became dirty, as far as the timer has seen. `None` while it is
    /// clean. Document 07 asks for a snapshot "after two minutes of dirty activity", so the two
    /// minutes are measured from here.
    dirty_since: Option<Instant>,
    /// Document 05 line 31's alpha-only view and transparency grid, and document 21 line 97's
    /// rule about both: they are presentation. Neither is in the project, neither is undoable
    /// (document 24 says so in its own column), and neither may change a cached pixel or
    /// anything exported. The grid is drawn by the page behind a canvas that already has
    /// alpha, so it touches no pixel at all; the alpha view is applied to the copy of the
    /// frame that is about to leave, after the cache has been given the frame it keeps.
    alpha_only: bool,
    checkerboard: bool,
    /// W-02's relink, worked out and not yet agreed to. See [`PendingRelink`]. At most one at a
    /// time: a person is answering one question, and a second proposal replaces the first rather
    /// than queueing behind it.
    relink: Option<PendingRelink>,
    /// B-08b: the decoded cels this preview has already paid for. Belongs to the viewer rather
    /// than to a frame because its whole purpose is to outlive one, and it is replaced along with
    /// everything else when a different project is opened, so nothing from the old one survives.
    /// Behind a lock of its own since P-04, and held by the viewer only so that opening a
    /// different project still replaces it along with everything else. A frame is rendered
    /// outside the viewer lock, so the one thing a render holds for its whole length has to
    /// be something a command never needs, and no command touches this.
    cache: Arc<Mutex<CelCache>>,
}

/// What the page is asking for.
///
/// Split out from the request handling so it can be tested without a window, which is the only
/// part of the transport a headless test can reach.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ask {
    /// The frame the playback clock says belongs on screen this many milliseconds after
    /// playback began. The page supplies the instant; the core decides the frame.
    At(u64),
    /// One named frame, for stepping. The clock is not consulted and nothing is skipped.
    Frame(i32),
    /// Start the clock over from this frame and answer it (W-09). What the page asks for when
    /// Play is pressed, so that playback begins under the playhead.
    Play(i32),
}

/// Read `/at/<milliseconds>`, `/frame/<n>` or `/play/<n>`, with an optional `?q=draft|full`.
///
/// Returns `None` for anything else, which the handler answers with a 404 rather than guessing.
/// An unreadable quality is `None` in the second slot, meaning "leave it as it is": a typo in a
/// query string should not silently switch the preview to a resolution nobody asked for.
fn parse(path: &str, query: Option<&str>) -> Option<(Ask, Option<PreviewQuality>)> {
    let mut parts = path.trim_matches('/').split('/');
    let ask = match (parts.next()?, parts.next()?, parts.next()) {
        ("at", ms, None) => Ask::At(ms.parse().ok()?),
        ("frame", n, None) => Ask::Frame(n.parse().ok()?),
        ("play", n, None) => Ask::Play(n.parse().ok()?),
        _ => return None,
    };
    Some((ask, quality_asked(query)))
}

/// The `?q=draft|full` half of a frame request, shared by `/frame`, `/at` and `/boxes`.
///
/// `None` means "leave it as it is": a typo in a query string should not silently switch the
/// preview to a resolution nobody asked for, and a selection outline asking for the wrong one
/// would draw its boxes at four times the size of the picture under them.
fn quality_asked(query: Option<&str>) -> Option<PreviewQuality> {
    query
        .and_then(|q| q.split('&').find_map(|pair| pair.strip_prefix("q=")))
        .and_then(|value| match value {
            "draft" => Some(PreviewQuality::Draft),
            "full" => Some(PreviewQuality::Full),
            _ => None,
        })
}

/// Where each layer's drawing lands on the frame that is on screen, as four corners.
///
/// The window is the only thing that can answer this. A layer's box is its source rectangle
/// carried through its transform, and the page has neither: document 07's project format records
/// no pixel size for a drawing, so the page cannot know how big a cel is, and the transform it
/// does hold is a set of numbers rather than a matrix. Asking the renderer is not a shortcut
/// around that — it is the only place the answer exists.
///
/// The corners are in the frame's own pixels, which is what the canvas is drawn in, so a draft
/// preview's boxes are a quarter the size of a full one's and land on the same drawing either
/// way. That is why the plan goes through [`preview::scale_plan`] here exactly as it does when
/// the frame is rendered.
///
/// A layer with no corners is a layer the plan does not contain — hidden, out of its own
/// `[in_frame, out_frame)`, or missing its drawing at this frame — and having no box is the
/// truthful answer for it rather than an empty one.
///
/// This is the preview path and holds the preview's cache, so it costs a plan the frame beside
/// it has usually already paid for. ADR-015 is untouched: nothing here is reachable from export.
/// One property of one layer, evaluated at every frame of a range, for the graph editor.
///
/// The graph editor draws a curve, and it has to know where the curve goes. It could work that
/// out from the keyframes the page already holds, which is what a graph editor usually does -
/// but then the shape on screen would be a second implementation of document 20, free to
/// disagree with the one that renders the frame, and a disagreement would look exactly like a
/// correct curve. So the page asks, and what it draws is what `Property::value_at` says.
///
/// This rides on the frame scheme beside `boxes` and for the same reason: it is a question about
/// what is on screen, whose answer changes nothing and needs no undo record. Unlike `boxes` it
/// costs no render - a property is evaluated without planning a frame.
fn curve(viewer: &Mutex<Viewer>, query: Option<&str>) -> Response<Vec<u8>> {
    let viewer = viewer.lock().expect("the viewer lock was poisoned");
    let number = |name: &str, fallback: i32| {
        parameter(query, name)
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(fallback)
    };
    let answer = (|| {
        let prop = property(parameter(query, "prop").as_deref()?)?;
        let layer = viewer
            .document
            .project()
            .composition(&viewer.composition)?
            .layer(&Id::new(&parameter(query, "layer")?))?;
        let (from, to) = (number("from", 0), number("to", 0));
        // A range wider than the longest composition the envelope allows is a page with a bug,
        // not a person with a problem, so it is cut short rather than refused.
        let to = to.clamp(from, from.saturating_add(10_000));
        // D-22: a scale is a percentage in the panels, so the graph is drawn in the numbers the
        // inspector beside it shows.
        let factor = if prop == Prop::Scale { 100.0 } else { 1.0 };
        let samples: Vec<serde_json::Value> = (from..=to)
            .map(|f| match layer.transform.get(prop).value_at(f) {
                Value::Scalar(v) => serde_json::json!([v * factor]),
                Value::Vec2(x, y) => serde_json::json!([x * factor, y * factor]),
            })
            .collect();
        Some(serde_json::json!({ "from": from, "to": to, "samples": samples }))
    })();
    // A layer or property that is not there is a page asking about something the person has just
    // deleted. No samples is the truthful answer to that, and the graph draws nothing.
    let body = answer.unwrap_or_else(|| serde_json::json!({ "from": 0, "to": 0, "samples": [] }));
    allow_the_page_to_read_this(Response::builder())
        .header("content-type", "application/json; charset=utf-8")
        .body(body.to_string().into_bytes())
        .expect("build the curve response")
}

fn boxes(viewer: &Mutex<Viewer>, frame: i32, quality: Option<PreviewQuality>) -> Response<Vec<u8>> {
    let taken = {
        let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
        if let Some(quality) = quality {
            viewer.quality = quality;
        }
        Snapshot {
            project: viewer.document.project().clone(),
            composition: viewer.composition.clone(),
            root: viewer.root.clone(),
            quality: viewer.quality,
            alpha_only: viewer.alpha_only,
            cache: Arc::clone(&viewer.cache),
            frame,
            reply: Response::builder(),
        }
    };
    let mut log = FrameLog::new(0);
    let plan = match anime_compositor::compose::plan_frame_cached(
        &taken.project,
        &taken.composition,
        taken.frame,
        &taken.root,
        &mut log,
        &mut taken.cache.lock().expect("the cel cache lock was poisoned"),
    ) {
        Ok(plan) => preview::scale_plan(plan, taken.quality),
        // The frame's own request will have said why in words. A selection outline is not the
        // place to say it a second time, so this answers with no boxes rather than an error.
        Err(_) => anime_compositor::render::FramePlan {
            width: 0,
            height: 0,
            layers: Vec::new(),
        },
    };
    let found: Vec<serde_json::Value> = plan
        .layers
        .iter()
        .map(|layer| {
            let (w, h) = (layer.source.width() as f64, layer.source.height() as f64);
            let corners: Vec<f64> = [(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)]
                .iter()
                .flat_map(|&(x, y)| {
                    let (x, y) = layer.transform.apply(x, y);
                    [x, y]
                })
                .collect();
            serde_json::json!({ "layer": layer.id.as_str(), "corners": corners })
        })
        .collect();
    // W-10: what each layer's five properties are on this frame, in the units the file holds,
    // so that the inspector can show a keyframed property's value under the playhead rather
    // than a base value nothing is drawn from. Every layer of the composition is here, not only
    // the ones the plan drew: a layer outside its own life still has a panel.
    let values: serde_json::Map<String, serde_json::Value> = taken
        .project
        .composition(&taken.composition)
        .map(|comp| {
            comp.layers_in_order()
                .map(|layer| {
                    let at: serde_json::Map<String, serde_json::Value> = layer
                        .transform
                        .value_at(frame)
                        .into_iter()
                        .map(|(prop, value)| {
                            // D-22: the file holds a scale as a percentage, and a whole
                            // number as a whole number, so the panel reads 100 and not 100.0.
                            let factor = if prop == Prop::Scale { 100.0 } else { 1.0 };
                            let num = |v: f64| match v * factor {
                                w if w.fract() == 0.0 && w.abs() < 1e15 => {
                                    serde_json::json!(w as i64)
                                }
                                w => serde_json::json!(w),
                            };
                            let value = match value {
                                Value::Scalar(v) => num(v),
                                Value::Vec2(x, y) => serde_json::json!([num(x), num(y)]),
                            };
                            (prop.as_str().to_string(), value)
                        })
                        .collect();
                    (layer.id.as_str().to_string(), serde_json::Value::Object(at))
                })
                .collect()
        })
        .unwrap_or_default();
    allow_the_page_to_read_this(taken.reply)
        .header("content-type", "application/json; charset=utf-8")
        .body(
            serde_json::json!({ "frame": frame, "layers": found, "values": values })
                .to_string()
                .into_bytes(),
        )
        .expect("build the boxes response")
}

/// Percent-encode a string so it can travel in an HTTP header and arrive unharmed.
///
/// Header values are bytes, and a project called `背景_日本語` or a diagnostic quoting a path
/// with a Japanese directory in it is not ASCII. Sending those bytes raw would either be
/// rejected or arrive as mojibake, and mojibake in a *diagnostic* is worse than no diagnostic:
/// the person is told the wrong filename. The page reads these with `decodeURIComponent`.
///
/// Everything outside printable ASCII is encoded, and so is `%` itself, so decoding is exact.
fn for_a_header(text: &str) -> String {
    let mut encoded = String::with_capacity(text.len());
    for byte in text.bytes() {
        match byte {
            b'%' | 0x00..=0x1f | 0x7f..=0xff => {
                encoded.push_str(&format!("%{byte:02X}"));
            }
            _ => encoded.push(byte as char),
        }
    }
    encoded
}

/// Let the page read the headers, not only receive them.
///
/// The webview treats `http://frame.localhost` as a different origin from the page, so a
/// response without both of these is fetched successfully and then withheld: the body fails
/// CORS and every `x-` header reads as absent. Both are needed; either alone is silence.
fn allow_the_page_to_read_this(
    response: tauri::http::response::Builder,
) -> tauri::http::response::Builder {
    response
        .header("access-control-allow-origin", "*")
        .header("access-control-expose-headers", "*")
}

/// Replace a frame's colour with its own alpha, in place, so the page draws the alpha channel.
///
/// Document 05 line 31 asks for an alpha-only view and document 21 line 97 requires it to be
/// presentation: this runs on the bytes that are about to be sent, which
/// `WorkingBuffer::to_srgb8_straight` has just allocated, so the cached frame and the export path
/// never see it. Grey rather than false colour, and opaque rather than transparent, because the
/// question the view answers is "how transparent is this pixel" and an answer that is itself
/// transparent cannot be looked at.
fn as_alpha_only(pixels: &mut [u8]) {
    for pixel in pixels.chunks_exact_mut(4) {
        pixel[0] = pixel[3];
        pixel[1] = pixel[3];
        pixel[2] = pixel[3];
        pixel[3] = 255;
    }
}

/// One frame's worth of viewer, copied out under the lock so that the render can run without it.
///
/// P-04. Everything here is either cheap to copy - a project is layer and asset records, not
/// pixels - or already shared, and the whole point of it is that none of it is borrowed from the
/// viewer: the lock is released the instant this exists.
struct Snapshot {
    project: Project,
    composition: Id,
    root: PathBuf,
    quality: PreviewQuality,
    alpha_only: bool,
    /// The decoded cels, which a render holds for its whole length and no command touches.
    cache: Arc<Mutex<CelCache>>,
    frame: i32,
    /// Everything the window has to say about this frame, already written into the response.
    /// Built under the lock rather than after the render, so that it describes the document the
    /// pixels were made from even if an edit lands while they are being made.
    reply: tauri::http::response::Builder,
}

/// Everything the window says about a frame, as headers, read under the lock that took the
/// snapshot beside it.
fn said_about(
    viewer: &Viewer,
    ask: Ask,
    frame: i32,
    skipped: u32,
    exporting: bool,
    exported: &str,
) -> tauri::http::response::Builder {
    allow_the_page_to_read_this(Response::builder())
        .header("content-type", "application/octet-stream")
        .header("x-frame", frame.to_string())
        .header("x-skipped", skipped.to_string())
        .header("x-quality", viewer.quality.label())
        // What the person is looking through. The page needs both: one to draw the grid behind
        // the canvas, and one so the buttons show which view is on.
        // W-02's proposal, if one is waiting to be agreed to. It travels with the frame like
        // everything else the window has to say, so the panel offering it cannot outlive it.
        .header(
            "x-relink",
            for_a_header(match &viewer.relink {
                Some(pending) => &pending.said,
                None => "",
            }),
        )
        .header(
            "x-relink-asset",
            for_a_header(match &viewer.relink {
                Some(pending) => pending.asset.as_str(),
                None => "",
            }),
        )
        .header("x-alpha", viewer.alpha_only.to_string())
        .header("x-checkerboard", viewer.checkerboard.to_string())
        .header(
            "x-differs",
            viewer.quality.differs_from_export().to_string(),
        )
        .header("x-project", for_a_header(&viewer.name))
        .header("x-notes", for_a_header(&viewer.notes.join("\n")))
        .header("x-status", for_a_header(&viewer.status))
        // What the timer last wrote, and what there is to recover. Both here rather than in
        // `x-status` so that neither can take the status line away from a command's answer.
        .header("x-autosaved", for_a_header(&viewer.autosaved))
        // The export, which belongs to the window rather than to the project on screen: it is
        // still running, and still cancellable, after another project has been opened.
        .header("x-exporting", exporting.to_string())
        .header("x-export", for_a_header(exported))
        .header(
            "x-recovery",
            for_a_header(
                &viewer
                    .recovery
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
        )
        // Document 26's dirty flag, as the person sees it. It is the core's answer, not a guess
        // by the window: `Document::is_dirty` is false again when the state matches the last
        // successful save, which is the rule a window cannot reimplement correctly.
        .header("x-dirty", viewer.document.is_dirty().to_string())
        .header(
            "x-path",
            for_a_header(
                &viewer
                    .path
                    .as_ref()
                    .map_or(String::new(), |p| p.display().to_string()),
            ),
        )
        // The playback report belongs to playback. A stepped frame did not come from the clock
        // and saying "played 0 frames" beside it would be a sentence about nothing.
        .header(
            "x-report",
            match ask {
                Ask::At(_) => viewer.playback.report(),
                Ask::Frame(_) | Ask::Play(_) => String::new(),
            },
        )
}

/// Render what was asked for and hand it back as raw display-ready pixels.
///
/// The body is `WorkingBuffer::to_srgb8_straight` — the same bytes an 8-bit export writes, minus
/// the PNG container. Encoding a PNG here would cost ten to thirty milliseconds against a frame
/// budget the latency measurement put at eighty-two, to be immediately undone by the browser.
/// The page draws these straight into an `ImageData`.
///
/// Everything the page needs to *say* about the frame travels in headers beside it, so the
/// number on screen and the pixels on screen always came from the same render.
fn serve(
    viewer: &Mutex<Viewer>,
    export: &Mutex<Export>,
    ask: Ask,
    quality: Option<PreviewQuality>,
) -> Response<Vec<u8>> {
    let (exporting, exported) = {
        let export = export.lock().expect("the export lock was poisoned");
        (export.cancel.is_some(), export.said.clone())
    };
    // P-01: how long the page waited for the frame already in flight. The guard outlives this
    // statement, so the duration is recorded rather than wrapped around a closure.
    let waited = std::time::Instant::now();
    // P-04, and document 06's snapshot contract applied to the viewer: everything this frame
    // needs is copied out under the lock, and the lock is given back before any of the work
    // begins. What the window has to say about the frame is written here as well, into the
    // response itself, so the sentence on screen and the pixels on screen still came from one
    // instant of one document - which is what holding the lock across the render used to buy,
    // and is the only thing it bought.
    let taken = {
        let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
        anime_compositor::perf::record(
            anime_compositor::perf::Stage::LockWait,
            waited.elapsed().as_nanos() as u64,
        );
        if let Some(quality) = quality {
            viewer.quality = quality;
        }
        let (frame, skipped) = match ask {
            Ask::At(ms) => {
                let shown = viewer.playback.at(Duration::from_millis(ms));
                (shown.frame, shown.skipped)
            }
            // Stepping stops at the ends of the work area rather than running off them: a frame
            // outside the composition is not a frame, and the viewer has nowhere to go from
            // there.
            Ask::Frame(n) => {
                let first = viewer.playback.at_rest();
                let last = first + viewer.playback.length() as i32 - 1;
                (n.clamp(first, last), 0)
            }
            // W-09: playback begins at the playhead. The clock is started over from this frame
            // and answers it at once, so the first frame played is the one that was on screen.
            Ask::Play(n) => {
                viewer.playback.start_from(n);
                let shown = viewer.playback.at(Duration::ZERO);
                (shown.frame, shown.skipped)
            }
        };
        Snapshot {
            project: viewer.document.project().clone(),
            composition: viewer.composition.clone(),
            root: viewer.root.clone(),
            quality: viewer.quality,
            alpha_only: viewer.alpha_only,
            cache: Arc::clone(&viewer.cache),
            frame,
            reply: said_about(viewer, ask, frame, skipped, exporting, &exported),
        }
    };

    let mut log = FrameLog::new(3);
    let buffer = match preview::preview_frame_cached(
        &taken.project,
        &taken.composition,
        taken.frame,
        &taken.root,
        taken.quality,
        DEFAULT_TILE_SIZE,
        &mut log,
        &mut taken.cache.lock().expect("the cel cache lock was poisoned"),
    ) {
        Ok(buffer) => buffer,
        // Document 28: a frame that cannot be made is reported, never replaced by something
        // that looks like a frame. The page shows this sentence instead of a picture.
        Err(diagnostic) => {
            return allow_the_page_to_read_this(Response::builder().status(500))
                .header("content-type", "text/plain; charset=utf-8")
                .body(diagnostic.message.into_bytes())
                .expect("build the diagnostic response")
        }
    };

    let image = buffer.as_image();
    let (width, height) = (image.width(), image.height());
    taken
        .reply
        // The only two things about a frame the window cannot say until it has been made.
        .header("x-width", width.to_string())
        .header("x-height", height.to_string())
        .body({
            let mut pixels = buffer.to_srgb8_straight();
            if taken.alpha_only {
                as_alpha_only(&mut pixels);
            }
            pixels
        })
        .expect("build the frame response")
}

/// Open a project file.
///
/// This is `persist::load` and a composition to look at. Media resolves against the project
/// file's own directory, which is the rule the core checks media against, so what the viewer
/// renders and what the core warned about are the same set of files.
fn open(path: &Path) -> Result<Viewer, Diagnostic> {
    let loaded = persist::load(path)?;
    // The first composition that has anything in it, and the first composition otherwise.
    //
    // Until B-12d a project this window edited had exactly one composition and `first` was the
    // whole rule. `composition.create` makes a second one reachable, and `verification/
    // B-12b_w01_walkthrough.md` immediately found what that costs: the walk made a composition,
    // filled it, saved, closed and reopened, and came back looking at the empty one the file had
    // started with. Nothing was lost, but a person cannot tell that from a blank viewer.
    //
    // This is a rule of thumb and not the answer. Document 07's project format has no field for
    // which composition was open, so there is nothing to restore; adding one is a schema change
    // and the owner's decision, and it is written up in
    // `verification/B-12d_new_composition_table.md`. Until then, an empty composition is never
    // the one somebody was working in.
    let composition = loaded
        .document
        .project()
        .compositions
        .iter()
        .find(|c| !c.is_empty())
        .or_else(|| loaded.document.project().compositions.first())
        .ok_or_else(|| {
            Diagnostic::new(
                DiagnosticId::ProjectSchemaInvalid,
                Severity::Error,
                "This project has no composition to show.",
                format!("{} contains an empty compositions array.", path.display()),
            )
            .with_remediation("The project that was open is still open. Nothing was changed.")
        })?;
    let first = composition.start_frame;
    let last = first + composition.duration_frames as i32 - 1;
    let playback = Playback::new(first, last, composition.frame_rate);
    let composition = composition.id.clone();

    // Document 28's PROJECT_RECOVERY_AVAILABLE, at the one moment it can be acted on. A person
    // who is told about unsaved work an hour after opening the project has already redone it.
    let candidates = persist::recovery_candidates(path);
    let mut notes: Vec<String> = loaded.warnings.iter().map(sentence).collect();
    if let Some(diagnostic) = persist::recovery_diagnostic(&candidates) {
        notes.push(sentence(&diagnostic));
    }

    Ok(Viewer {
        document: loaded.document,
        preserved: loaded.preserved,
        path: Some(path.to_path_buf()),
        root: path.parent().unwrap_or(Path::new(".")).to_path_buf(),
        composition,
        quality: PreviewQuality::default(),
        playback,
        name: path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.display().to_string()),
        notes,
        status: String::new(),
        recovery: candidates.into_iter().map(|c| c.path).collect(),
        autosaved: String::new(),
        dirty_since: None,
        alpha_only: false,
        // On, because a transparent frame that reads as black is a frame a person
        // misjudges, and every photograph of this window so far was taken with the grid there.
        checkerboard: true,
        relink: None,
        cache: Arc::new(Mutex::new(CelCache::viewer())),
    })
}

/// One diagnostic as the window says it: what happened, then what to do about it.
fn sentence(diagnostic: &Diagnostic) -> String {
    match &diagnostic.remediation {
        Some(next) => format!("{} {}", diagnostic.message, next),
        None => diagnostic.message.clone(),
    }
}

/// The project the window opens on when it was not given one.
///
/// The reference shot, resolved against this crate's source directory, so it only works from a
/// checkout. Its media root is overridden because this one project is not where a project
/// normally is: `verification/B-08a_project.json` is written by `cargo test` into `verification`
/// while the cels it names live under `Fixtures/reference_shot`. The load warnings are dropped
/// with it, because they describe the directory this project is *not* rendered against and
/// would name files that are present. That is a development convenience and the only one; a
/// project the person actually opens goes through [`open`] unaltered.
fn demo() -> Viewer {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .to_path_buf();
    let path = repo.join("verification/B-08a_project.json");
    let mut viewer =
        open(&path).unwrap_or_else(|d| panic!("open {}: {}", path.display(), d.message));
    viewer.root = repo.join("Fixtures/reference_shot");
    viewer.name = "the reference shot".to_string();
    viewer.notes.clear();
    // With no save path there is nothing to recover *into*, so offering a snapshot would offer
    // something that could not be finished.
    viewer.recovery.clear();
    // No save path, deliberately. This project's file is written into `verification/` by
    // `cargo test` and CI checks that directory has not changed; a Save that landed there would
    // fail the build for a reason nobody would connect to a button. Save therefore asks where.
    viewer.path = None;
    viewer
}

/// Save the open project to `path`, and say what happened in one sentence.
///
/// This is `persist::save` with the preserved data handed back to it, and nothing else. On
/// failure the file at `path` is untouched and the document stays dirty, which is the core's
/// guarantee rather than this function's.
fn write(viewer: &mut Viewer, path: &Path) -> Result<(), Diagnostic> {
    persist::save(path, &mut viewer.document, &viewer.preserved)
}

/// Save to the file the project came from, or say why there is not one.
fn save(viewer: &Mutex<Viewer>) -> String {
    let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
    let Some(path) = viewer.path.clone() else {
        return "This project has no file of its own yet. Use Save As.".to_string();
    };
    match write(viewer, &path) {
        Ok(()) => format!("Saved to {}", path.display()),
        Err(diagnostic) => sentence(&diagnostic),
    }
}

/// Save to a file the person chose, then open that file.
///
/// Reopening is the point. After Save As the window is showing a project that lives somewhere
/// else, and media paths in a project file are relative to it, so a project saved into another
/// directory may no longer find its cels. Loading the file back means the window shows what
/// somebody opening it tomorrow would see, warnings and all, instead of a picture that only
/// works because the old media root is still in memory.
fn save_as(viewer: &Mutex<Viewer>, path: &Path) -> String {
    {
        let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
        if let Err(diagnostic) = write(viewer, path) {
            return sentence(&diagnostic);
        }
    }
    let said = format!("Saved to {}", path.display());
    take(viewer, path);
    said
}

/// Load a dropped or named file into the viewer, or report why it could not be.
///
/// A file that cannot be opened leaves the project that was open exactly as it was and adds the
/// reason to what the window is saying. Closing a working project because the next one was
/// unreadable would lose the person their place to punish them for a bad drop.
fn take(viewer: &Mutex<Viewer>, path: &Path) {
    let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
    match open(path) {
        Ok(opened) => {
            *viewer = opened;
            viewer.status = format!("Opened {}", path.display());
        }
        Err(diagnostic) => {
            viewer.notes = vec![sentence(&diagnostic)];
            viewer.status = format!("{} could not be opened.", path.display());
        }
    }
}

// -------------------------------------------------------------------------------------------
// Autosave and recovery
// -------------------------------------------------------------------------------------------

/// Document 07: "save a recovery snapshot after two minutes of dirty activity".
const AUTOSAVE_AFTER: Duration = Duration::from_secs(120);

/// How often the timer looks. Short enough that the two minutes above are two minutes and not
/// two and a half, long enough that a window nobody is touching costs nothing.
const AUTOSAVE_TICK: Duration = Duration::from_secs(10);

/// One look at the clock. Returns what to say if a snapshot was written, and nothing otherwise.
///
/// `now` is a parameter so this can be checked without waiting two minutes; the window passes
/// `Instant::now()`.
///
/// Document 26: "Autosave does not clear user-facing dirty state and does not replace the
/// canonical manual-save path." Nothing here calls `mark_saved` and nothing here can — the core's
/// `autosave` takes the document by shared reference for exactly that reason.
fn autosave_tick(viewer: &mut Viewer, now: Instant) -> Option<String> {
    // A project with no file of its own has nowhere to put a snapshot: recovery files live beside
    // the project, and there is no project. The built-in reference shot is in this state.
    let Some(path) = viewer.path.clone() else {
        viewer.dirty_since = None;
        return None;
    };
    if !viewer.document.is_dirty() {
        viewer.dirty_since = None;
        return None;
    }
    let since = *viewer.dirty_since.get_or_insert(now);
    if now.duration_since(since) < AUTOSAVE_AFTER {
        return None;
    }
    // Measured from this snapshot, not from when the work began, so a document left dirty writes
    // one snapshot every two minutes rather than one on every tick after the first two.
    viewer.dirty_since = Some(now);
    Some(
        match persist::autosave(&path, &viewer.document, &viewer.preserved) {
            Ok(written) => {
                viewer.recovery = persist::recovery_candidates(&path)
                    .into_iter()
                    .map(|c| c.path)
                    .collect();
                format!("Recovery snapshot written to {}", written.display())
            }
            // A failed autosave is said out loud rather than swallowed. It is the one moment the
            // person can still do something about it — the work is in memory and nowhere else.
            Err(diagnostic) => sentence(&diagnostic),
        },
    )
}

/// Open a recovery snapshot as the project it belongs to.
///
/// The snapshot's *contents* are loaded and the project's *identity* is kept: the window is left
/// pointing at the project file, so Save writes the recovered work into the project rather than
/// back into the snapshot. Document 07 requires that recovering not overwrite the last manual
/// save, and it does not: nothing is written here at all. The project file is still on disk
/// exactly as it was, which is why the document opens dirty — the difference between what is on
/// screen and what is in the file is the work being recovered.
fn recover(viewer: &Mutex<Viewer>, snapshot: &Path) -> String {
    let project = match viewer
        .lock()
        .expect("the viewer lock was poisoned")
        .path
        .clone()
    {
        Some(path) => path,
        None => return "There is no project to recover into.".to_string(),
    };
    let mut taken = match open(snapshot) {
        Ok(taken) => taken,
        Err(diagnostic) => return sentence(&diagnostic),
    };
    // The project as the file has it, which is the baseline the recovered document is dirty
    // against. If it cannot be read, the snapshot is not opened either: a window that could not
    // say what the file holds cannot say what is outstanding.
    let saved = match persist::load(&project) {
        Ok(saved) => saved,
        Err(diagnostic) => return sentence(&diagnostic),
    };
    taken.document = Document::recovered(
        taken.document.project().clone(),
        saved.document.project().clone(),
    );
    taken.name = project
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| project.display().to_string());
    taken.recovery = persist::recovery_candidates(&project)
        .into_iter()
        .map(|c| c.path)
        .collect();
    taken.notes.push(format!(
        "This is the recovery snapshot {}, not the saved project. Nothing has been written to \
         {} yet; Save writes this into it.",
        snapshot.display(),
        project.display()
    ));
    taken.path = Some(project);
    let said = format!("Recovered {}", snapshot.display());
    *viewer.lock().expect("the viewer lock was poisoned") = taken;
    said
}

/// Say something in the window's status line, replacing whatever it said before.
fn announce(viewer: &Mutex<Viewer>, said: String) {
    viewer.lock().expect("the viewer lock was poisoned").status = said;
}

// -------------------------------------------------------------------------------------------
// Editing
// -------------------------------------------------------------------------------------------

/// Everything the editing panels draw, as one JSON answer.
///
/// The project half of it is `persist::to_json` - the same text a save writes, preserved records
/// and all - rather than a summary shaped for the page. Two reasons. What a panel shows is then
/// what the file would hold, so a panel cannot quietly disagree with a save. And a record this
/// build does not model still reaches the page, so an effect from a later version is listed as
/// present in the stack instead of being absent from the only view the person has of it.
///
/// Selection is not here. Which layer is being worked on is not a fact about the project and does
/// not belong in the file; it lives in the page, and the page is the only thing that needs it.
fn state(viewer: &Mutex<Viewer>) -> String {
    let viewer = &*viewer.lock().expect("the viewer lock was poisoned");
    let project: serde_json::Value = serde_json::from_str(&persist::to_json(
        viewer.document.project(),
        &viewer.preserved,
    ))
    .expect("the project text the core just wrote is JSON");
    serde_json::json!({
        "project": project,
        "composition": viewer.composition.as_str(),
        "revision": viewer.document.revision(),
        // What Undo and Redo would do next, in the words document 26 requires each record to
        // carry. The buttons say it rather than saying "Undo", because a person who has been
        // away from the window for a minute cannot otherwise know what is about to be taken back.
        "undo": viewer.document.undo_labels(),
        "redo": viewer.document.redo_labels(),
    })
    .to_string()
}

/// Apply one command and say what happened, in the history record's own words.
///
/// Document 26: a rejected command changes nothing at all. That is the core's guarantee and not
/// this function's - what happens here on a refusal is that the diagnostic becomes the sentence
/// on the status line, so the person is told which rule stopped them rather than watching a
/// control spring back with no explanation.
fn edit(viewer: &Mutex<Viewer>, command: Command) -> String {
    let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
    match viewer.document.apply(command) {
        Ok(record) => record.label.clone(),
        Err(diagnostic) => sentence(&diagnostic),
    }
}

/// Put a composition of the open project on screen, with a playback clock to match it.
///
/// The clock is rebuilt rather than kept, because it carries the composition's first and last
/// frame and its rate: a viewer that switched composition without it would play the new shot to
/// the old one's length.
fn show(viewer: &Mutex<Viewer>, id: &Id) {
    let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
    let Some(comp) = held.document.project().composition(id) else {
        return;
    };
    let first = comp.start_frame;
    let last = first + comp.duration_frames as i32 - 1;
    held.playback = Playback::new(first, last, comp.frame_rate);
    held.composition = id.clone();
}

/// Put the window on the composition an undo or a redo moved, and never on one that is gone.
///
/// `touched` is the record's own affected list. Three rules in order of preference, and they are
/// one rule: show the person the change they just asked for.
///
/// If the record names a composition that is in the project, that is where the change was, and
/// that is where the window goes -- which for a redone `composition.create` is the composition
/// that has just come back, and for any layer edit is the composition it happened in. If it
/// names none that exist, the window stays where it is. If where it is has itself been taken
/// away -- which is what undoing `composition.create` does -- it falls back to the first
/// composition, which is where [`open`] starts. Without that last rule the window keeps an ID
/// nothing answers, every frame request finds nothing, and the person is left looking at a
/// viewer that says there is no composition on screen with no way back to one there is.
fn settle(viewer: &Mutex<Viewer>, touched: &[Id]) {
    let go = {
        let held = viewer.lock().expect("the viewer lock was poisoned");
        let project = held.document.project();
        touched
            .iter()
            .find(|id| project.composition(id).is_some())
            .or_else(|| project.composition(&held.composition).map(|c| &c.id))
            .or_else(|| project.compositions.first().map(|c| &c.id))
            .cloned()
    };
    if let Some(id) = go {
        show(viewer, &id);
    }
}

/// `edit.undo` and `edit.redo` from document 24.
///
/// Both name what they moved. "Undone" alone would be true and useless: the whole reason undo is
/// trusted is that the person can see it took back the thing they meant.
fn undo(viewer: &Mutex<Viewer>) -> String {
    let (said, touched) = {
        let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
        match held.document.undo() {
            Some(record) => (format!("Undone: {}", record.label), record.affected.clone()),
            None => ("There is nothing to undo.".to_string(), Vec::new()),
        }
    };
    settle(viewer, &touched);
    said
}

fn redo(viewer: &Mutex<Viewer>) -> String {
    let (said, touched) = {
        let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
        match held.document.redo() {
            Some(record) => (format!("Redone: {}", record.label), record.affected.clone()),
            None => ("There is nothing to redo.".to_string(), Vec::new()),
        }
    };
    settle(viewer, &touched);
    said
}

/// Which transform property a request names, or `None` for a word that is not one of them.
///
/// The five names are document 19's own, which are also the keys `persist` writes, so the page
/// sends back the word it was given rather than a number this file would have to keep in step.
fn property(name: &str) -> Option<Prop> {
    [
        Prop::Anchor,
        Prop::Position,
        Prop::Scale,
        Prop::Rotation,
        Prop::Opacity,
    ]
    .into_iter()
    .find(|p| p.as_str() == name)
}

/// A property value from the page: `x,y` for the three vec2 properties, one number for the two
/// scalar ones.
///
/// Shaped by the property rather than guessed from the text, so that sending one number for
/// position is refused by the core's own rule about kinds instead of being quietly read as a
/// scalar. Whether the number is in range is not decided here either - opacity clamps, and that
/// is document 19's decision and is made in one place.
fn property_value(prop: Prop, text: &str) -> Option<Value> {
    let number = |t: &str| t.trim().parse::<f64>().ok();
    // D-22: the file holds a scale as a percentage and the model holds the factor document 21
    // composes with, and `persist` divides on the way in and multiplies on the way out. The panel
    // is given the file's numbers and sends them back in the same units, so the same division
    // belongs here. Without it, somebody who typed the 100 the field was already showing got a
    // layer a hundred times its size, and the walkthrough in `acceptance` is what found that:
    // every earlier test of this function set a position or checked a scale being refused.
    let percent = if prop == Prop::Scale { 100.0 } else { 1.0 };
    match prop.kind() {
        "vec2" => {
            let (x, y) = text.split_once(',')?;
            Some(Value::Vec2(number(x)? / percent, number(y)? / percent))
        }
        _ => Some(Value::Scalar(number(text)? / percent)),
    }
}

/// Release a drag: one history entry covering everything the scrub passed through.
///
/// A drag that ended where it started is not an edit. The core returns `None` for it and puts
/// the value back, and this says so rather than reporting a change nobody made.
fn end_drag(viewer: &Mutex<Viewer>) -> String {
    let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
    match viewer.document.end_drag() {
        Some(record) => record.label.clone(),
        None => "Nothing moved.".to_string(),
    }
}

/// Escape during a drag. Document 24: "Escape restores the pre-drag value."
fn cancel_drag(viewer: &Mutex<Viewer>) -> String {
    let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
    let was = viewer.document.drag_in_progress();
    viewer.document.cancel_drag();
    if was {
        "Cancelled; the value is back where the drag started.".to_string()
    } else {
        "Nothing is being dragged.".to_string()
    }
}

/// One intermediate value of a scrub, starting the transaction if this is the first one.
///
/// Document 26 wants a drag to be one history entry, which means something has to open the
/// transaction. Doing it here rather than on a separate request from the page removes the state
/// the page would otherwise have to keep in step with the document - a page that forgot to open
/// one would write a history entry per pixel of mouse movement, and nothing would look wrong
/// until somebody pressed Ctrl+Z.
fn drag_update(viewer: &Mutex<Viewer>, command: Command) -> String {
    let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
    if !viewer.document.drag_in_progress() {
        if let Err(diagnostic) = viewer.document.begin_drag() {
            return sentence(&diagnostic);
        }
    }
    match viewer.document.update_drag(command) {
        // Not the label: a scrub sends these several times a second and the status line is for
        // reading. What it moved to is the useful half.
        Ok(()) => "Dragging.".to_string(),
        Err(diagnostic) => sentence(&diagnostic),
    }
}

/// A layer ID nothing in this project is using.
///
/// Counted rather than random, so that running the same steps twice writes the same file and a
/// person reading one can tell which layer is which. It takes one past the highest number in
/// use rather than the first gap: a gap means a layer was deleted, and giving its number to
/// something else makes two different layers indistinguishable in a diff of two saves.
fn unused_layer_id(project: &Project) -> Id {
    let highest = project
        .compositions
        .iter()
        .flat_map(|c| c.layer_order())
        .filter_map(|id| id.as_str().strip_prefix("layer-"))
        .filter_map(|n| n.parse::<u64>().ok())
        .max();
    Id::new(format!("layer-{}", highest.map_or(1, |n| n + 1)))
}

/// An effect instance ID nothing in this project is using, counted the way layer IDs are.
///
/// Counted across the whole project rather than the one layer, so that an effect keeps its
/// identity if it is ever moved between layers and two saves of the same steps match.
fn unused_effect_id(project: &Project) -> Id {
    let highest = project
        .compositions
        .iter()
        .flat_map(|c| c.layers_in_order())
        .flat_map(|l| l.effects.iter())
        .filter_map(|e| e.instance_id.as_str().strip_prefix("fx-"))
        .filter_map(|n| n.parse::<u64>().ok())
        .max();
    Id::new(format!("fx-{}", highest.map_or(1, |n| n + 1)))
}

/// A composition ID nothing in this project is using, counted the way layer IDs are.
fn unused_composition_id(project: &Project) -> Id {
    let highest = project
        .compositions
        .iter()
        .filter_map(|c| c.id.as_str().strip_prefix("comp-"))
        .filter_map(|n| n.parse::<u64>().ok())
        .max();
    Id::new(format!("comp-{}", highest.map_or(1, |n| n + 1)))
}

/// An asset ID nothing in this project is using, counted the way layer IDs are.
fn unused_asset_id(project: &Project) -> Id {
    let highest = project
        .assets
        .iter()
        .filter_map(|a| a.id.as_str().strip_prefix("asset-"))
        .filter_map(|n| n.parse::<u64>().ok())
        .max();
    Id::new(format!("asset-{}", highest.map_or(1, |n| n + 1)))
}

/// A list of numbers as somebody would read it out: `7`, `7 and 9`, `7, 9 and 11`.
fn spoken(numbers: &[u32]) -> String {
    let text: Vec<String> = numbers.iter().map(u32::to_string).collect();
    match text.split_last() {
        None => String::new(),
        Some((last, [])) => last.clone(),
        Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
    }
}

/// Document 24's `media.import`: group the chosen files into one sequence and add it.
///
/// The selection is the person's and is never widened here. Document 07: "Search only
/// user-selected locations" — B-03's importer takes the files it is given and does not look in
/// the folder around them, and this passes on what it was handed.
///
/// Everything the importer said travels to the notes list unchanged, because a gap, a duplicate
/// number or a file that was left out is a fact about the project now on screen and not about
/// the button that was just pressed. The status line gets the summary a person checks a scan
/// against: how many drawings arrived, which numbers they run between, and what is missing.
fn import(viewer: &Mutex<Viewer>, files: &[PathBuf]) -> String {
    let result = media::import_sequence(files);
    let told: Vec<String> = result.diagnostics.iter().map(sentence).collect();
    let Some(sequence) = result.asset else {
        let mut said = match files.len() {
            1 => "The chosen file does not form an image sequence, so nothing was imported."
                .to_string(),
            n => format!(
                "None of the {n} chosen files form an image sequence, so nothing was imported."
            ),
        };
        for note in told {
            said.push(' ');
            said.push_str(&note);
        }
        return said;
    };

    let asset = {
        let held = viewer.lock().expect("the viewer lock was poisoned");
        let frames = sequence
            .frames()
            .iter()
            .map(|(n, p)| (*n, persist::stored_path(&held.root, p)))
            .collect();
        // The pattern is the name, because it is what the folder shows and what a person
        // recognises. It is a description of the naming, never the authority on which files
        // exist; the frame list is that, and it is what the record carries.
        Asset::sequence(
            unused_asset_id(held.document.project()),
            sequence.pattern(),
            sequence.pattern(),
        )
        .with_frames(frames)
    };
    let id = asset.id.clone();
    let said = edit(viewer, Command::AddAsset { asset });

    {
        let mut held = viewer.lock().expect("the viewer lock was poisoned");
        if !held.document.project().assets.iter().any(|a| a.id == id) {
            return said;
        }
        held.notes.extend(told);
    }
    let (lo, hi) = sequence
        .range()
        .expect("an imported sequence has at least one drawing");
    let count = sequence.frames().len();
    match sequence.missing().as_slice() {
        [] => format!("{said}: {count} drawings, numbered {lo} to {hi}."),
        [one] => format!(
            "{said}: {count} drawings, numbered {lo} to {hi}, and drawing {one} is missing."
        ),
        many => format!(
            "{said}: {count} drawings, numbered {lo} to {hi}, and {} drawings are missing: {}.",
            many.len(),
            spoken(many)
        ),
    }
}

/// A relink that has been worked out and is waiting to be agreed to.
///
/// W-02 requires the window to "present changed dimensions, frame range or alpha interpretation
/// before applying the change", so choosing the replacement drawings and relinking to them cannot
/// be one action: the first works out what would happen and leaves it here, and the second either
/// applies it or throws it away. Nothing about the project changes while one of these exists.
struct PendingRelink {
    asset: Id,
    /// The description the person is being asked to agree to, in the words the panel shows.
    said: String,
    candidate: persist::RelinkCandidate,
}

/// The size of the drawings an asset points at now, or `None` when they cannot be read.
///
/// Relinking is what a person does when the media has gone, so the usual case is that there is
/// nothing to compare against. Saying that is the point: an unanswerable comparison presented as
/// "unchanged" would be a lie in the one place W-02 asks for the truth.
fn size_now(root: &Path, asset: &Asset) -> Option<(usize, usize)> {
    asset
        .frames
        .values()
        .find_map(|relative| media::decode_png(&root.join(relative)).ok())
        .map(|image| (image.width(), image.height()))
}

/// Work out what relinking `asset` to `files` would do, and hold it until it is agreed to.
fn propose_relink(viewer: &Mutex<Viewer>, asset: &Id, files: &[PathBuf]) -> String {
    let mut held = viewer.lock().expect("the viewer lock was poisoned");
    let candidate =
        match persist::relink_candidate(held.document.project(), asset, files, &held.root) {
            Ok(candidate) => candidate,
            Err(diagnostic) => return sentence(&diagnostic),
        };
    let existing = held
        .document
        .project()
        .assets
        .iter()
        .find(|a| &a.id == asset)
        .cloned()
        .expect("relink_candidate refuses an asset that is not there");
    let was = size_now(&held.root, &existing);

    let count = candidate.asset.frames.len();
    let numbered = match candidate.range {
        Some((lo, hi)) => format!("{count} drawings, numbered {lo} to {hi}"),
        None => format!("{count} drawings"),
    };
    let gaps = match candidate.missing.as_slice() {
        [] => String::new(),
        [one] => format!(", and drawing {one} would be missing"),
        many => format!(
            ", and {} drawings would be missing: {}",
            many.len(),
            spoken(many)
        ),
    };
    // The three things W-02 names, each stated whether or not it changed. "Unchanged" is as much
    // of an answer as "1920 by 1080 becomes 1280 by 720", and a person about to replace the
    // artwork in a shot needs to be told both.
    let size = match was {
        Some((w, h)) if (w, h) == (candidate.width as usize, candidate.height as usize) => format!(
            "The drawings are {}x{}, the same size as the ones it points at now.",
            candidate.width, candidate.height
        ),
        Some((w, h)) => format!(
            "The drawings are {}x{}, where the ones it points at now are {w}x{h}.",
            candidate.width, candidate.height
        ),
        None => format!(
            "The drawings are {}x{}. What it points at now cannot be read, so there is nothing \
             to compare that with.",
            candidate.width, candidate.height
        ),
    };
    let interpretation = format!(
        "The colour is read as {} with {} alpha, which is what this project already recorded: a \
         PNG does not state either, so relinking does not change how the pixels are read.",
        match candidate.interpretation.color_space {
            ColorSpace::Srgb => "sRGB",
            ColorSpace::LinearLight => "linear light",
        },
        match candidate.interpretation.alpha {
            AlphaMode::Straight => "straight",
            AlphaMode::Premultiplied => "premultiplied",
        }
    );
    let mut said = format!(
        "Relinking \"{}\" to {} would give it {numbered}{gaps}. {size} {interpretation} The \
         layers using it keep their stacking, transforms, exposures, masks and effects. Nothing \
         has changed yet.",
        existing.name, candidate.pattern,
    );
    for note in candidate.diagnostics.iter().map(sentence) {
        said.push(' ');
        said.push_str(&note);
    }
    held.relink = Some(PendingRelink {
        asset: asset.clone(),
        said: said.clone(),
        candidate,
    });
    said
}

/// The three effects of document 21, at the settings that change no pixels.
///
/// Adding an effect and setting it are two commands rather than one, so that a stack can be
/// built before any of it is tuned; starting each one at its identity means the picture does
/// not jump the instant an effect is added, and the change a person then sees is the one they
/// typed.
fn new_effect(type_id: &str) -> Option<Effect> {
    match type_id {
        EXPOSURE => Some(Effect::Exposure { stops: 0.0 }),
        GAUSSIAN_BLUR => Some(Effect::GaussianBlur { sigma_px: 0.0 }),
        TINT => Some(Effect::Tint {
            color: [0.0, 0.0, 0.0],
            amount: 0.0,
        }),
        _ => None,
    }
}

/// The settings for an effect of `type_id`, read from the request.
///
/// Every parameter of the type has to be present. The alternative -- filling a missing one in
/// from a default -- would let a request that named one parameter quietly reset the others, and
/// this is the request a panel sends after somebody types in one field. Whether the numbers are
/// in range is document 21's decision and is made in the core, so a negative sigma is refused
/// there, in its words, rather than judged twice.
fn effect_parameters(type_id: &str, query: Option<&str>) -> Result<Effect, String> {
    let number = |name: &str| -> Result<f64, String> {
        let Some(text) = parameter(query, name) else {
            return Err(format!("What should {name} be set to?"));
        };
        text.trim()
            .parse::<f64>()
            .map_err(|_| format!("{name} needs a number. Not \"{text}\"."))
    };
    match type_id {
        EXPOSURE => Ok(Effect::Exposure {
            stops: number("stops")?,
        }),
        GAUSSIAN_BLUR => Ok(Effect::GaussianBlur {
            sigma_px: number("sigma_px")?,
        }),
        TINT => {
            let Some(text) = parameter(query, "color") else {
                return Err("What colour should the tint be?".to_string());
            };
            let parts: Vec<f64> = text
                .split(',')
                .filter_map(|p| p.trim().parse::<f64>().ok())
                .collect();
            let [r, g, b] = parts[..] else {
                return Err(format!(
                    "color needs three numbers, like 1, 0.5, 0. Not \"{text}\"."
                ));
            };
            Ok(Effect::Tint {
                color: [r, g, b],
                amount: number("amount")?,
            })
        }
        // Document 19 keeps an effect this build does not have rather than dropping it, and
        // keeping it means keeping its settings as they were written. There is no schema here
        // to read them against, so they are left alone and said to be left alone.
        _ => Err(format!(
            "{type_id} is not an effect this build has, so its settings are kept as the file \
             wrote them and cannot be changed here."
        )),
    }
}

/// Everything the page can do to the open document, keyed by document 24's own command IDs.
///
/// The IDs are the URL paths, so what the page asks for and what the map lists are the same
/// string, and a command that is not in the map cannot be reached by spelling one. `None` means
/// this path was not one of them, which the caller answers with its 404.
///
/// Every answer, including a refusal, is a sentence for the status line. The list a button was
/// clicked in can be older than the document, so a layer that is no longer there is told about
/// rather than silently doing nothing.
/// The identifiers `edit_command` answers, and the only ones.
///
/// `None` is how this function says it has never heard of an identifier, and `fn command` reads
/// that as permission to try the routes it answers itself -- opening, saving, exporting. Without
/// this list an unknown identifier did not reach that `None`: it fell through to the layer
/// lookup at the bottom of the function and was answered "Which layer? Choose one in the layer
/// list.", which is a sentence, which is `Some`. Every route the shell answers was therefore
/// swallowed on its way past, and Open, Save, Save As, Export, the recent list and recovery all
/// stopped working in the built window while every test kept passing, because the tests call
/// those functions directly and only the window goes through here.
///
/// `verification/B-12b_page_table.md` checks that everything the page sends is in this list, and
/// `verification/B-12b_command_map_table.md` checks that nothing outside it is answered.
const ANSWERS: &[&str] = &[
    "composition.create",
    "composition.open",
    "edit.redo",
    "edit.undo",
    "effect.add",
    "effect.delete",
    "effect.move",
    "effect.move_down",
    "effect.move_up",
    "effect.set_parameters",
    "effect.toggle_bypass",
    "exposure.set_span",
    "keyframe.add_remove",
    "keyframe.move",
    "keyframe.set_interp",
    "layer.create",
    "layer.delete",
    "layer.move_down",
    "layer.move_up",
    "layer.rename",
    "layer.set_matte",
    "layer.shift",
    "layer.toggle_lock",
    "layer.toggle_visibility",
    "layer.trim",
    "media.import",
    "media.relink",
    "property.drag_cancel",
    "property.drag_end",
    "property.drag_update",
    "property.set_base",
    "viewer.toggle_alpha",
    "viewer.toggle_checkerboard",
];

fn edit_command(viewer: &Mutex<Viewer>, id: &str, query: Option<&str>) -> Option<String> {
    if !ANSWERS.contains(&id) {
        return None;
    }
    match id {
        "edit.undo" => return Some(undo(viewer)),
        "edit.redo" => return Some(redo(viewer)),
        // The two ends of an interaction transaction. Neither names a layer: the drag already
        // knows which value it is holding, and asking the page to name it again would be asking
        // it to be right about something it has no way to check.
        // The two ways of looking at a frame rather than changing it. Document 24's table gives
        // both "no" under undoable, and document 21 line 97 calls them presentation, so neither
        // touches the document: they set a flag the next frame is drawn through. That also means
        // neither marks the project dirty, which is the part a person would notice if it were
        // got wrong - looking at the alpha channel is not unsaved work.
        "viewer.toggle_alpha" => {
            let mut held = viewer.lock().expect("the viewer lock was poisoned");
            held.alpha_only = !held.alpha_only;
            return Some(match held.alpha_only {
                true => "Alpha-only inspection is on. The picture is the alpha channel, white \
                         where the frame is opaque and black where it is empty; what is exported \
                         is unchanged."
                    .to_string(),
                false => "Alpha-only inspection is off.".to_string(),
            });
        }
        "viewer.toggle_checkerboard" => {
            let mut held = viewer.lock().expect("the viewer lock was poisoned");
            held.checkerboard = !held.checkerboard;
            return Some(match held.checkerboard {
                true => "The transparency grid is on. It is drawn behind the frame and is not \
                         part of it."
                    .to_string(),
                false => "The transparency grid is off.".to_string(),
            });
        }
        "property.drag_end" => return Some(end_drag(viewer)),
        "property.drag_cancel" => return Some(cancel_drag(viewer)),
        // The one command that names files rather than anything in the project. Without any,
        // the request never reaches here: the window answers it with the operating system's
        // file dialog, which only the app handle can open.
        // W-02, in three requests rather than one: choose the drawings, read what would change,
        // then apply it or drop it. The middle step is the requirement - nothing about the
        // project moves until `apply` arrives, and `cancel` leaves it pointing where it did.
        "media.relink" => {
            let Some(asset) = parameter(query, "asset") else {
                return Some(
                    "Which sequence should be relinked? Choose one in the media bin.".to_string(),
                );
            };
            let asset = Id::new(&asset);
            if parameter(query, "cancel").is_some() {
                let waiting = viewer
                    .lock()
                    .expect("the viewer lock was poisoned")
                    .relink
                    .take();
                return Some(match waiting {
                    Some(_) => "The relink was dropped. The sequence still points at the \
                                drawings it did."
                        .to_string(),
                    None => "There is no relink waiting to be dropped.".to_string(),
                });
            }
            if parameter(query, "apply").is_some() {
                let waiting = {
                    let mut held = viewer.lock().expect("the viewer lock was poisoned");
                    match held.relink.as_ref().is_some_and(|p| p.asset == asset) {
                        true => held.relink.take(),
                        false => None,
                    }
                };
                let Some(waiting) = waiting else {
                    return Some(
                        "There is no relink waiting for that sequence. Choose the replacement \
                         drawings first, so that what would change can be read before it happens."
                            .to_string(),
                    );
                };
                return Some(edit(viewer, persist::relink_command(&waiting.candidate)));
            }
            let files: Vec<PathBuf> = parameters(query, "file")
                .into_iter()
                .map(PathBuf::from)
                .collect();
            return Some(match files.is_empty() {
                true => "Which drawings should the sequence point at? Choose the files \
                         themselves, not the folder they are in."
                    .to_string(),
                false => propose_relink(viewer, &asset, &files),
            });
        }
        // W-01 step 3, which had no command until B-12d. Answered here rather than below
        // because everything below reads the composition on screen first, and this is the one
        // command whose whole purpose is that there may not be one worth working in yet.
        //
        // The five fields are document 19 line 15's own: name, width, height, frame rate, and a
        // length in frames. A start frame is not asked for. Document 19 has one and every
        // fixture in this build starts at 0; a field nobody in W-01 is told what to put in is a
        // field somebody puts the wrong thing in, and `SetExposureSpans` can move the work
        // afterwards if a shot ever needs it.
        "composition.create" => {
            let name = parameter(query, "name").unwrap_or_else(|| "New composition".to_string());
            let number = |field: &str, fallback: u32| -> Result<u32, String> {
                match parameter(query, field) {
                    None => Ok(fallback),
                    Some(text) => text.trim().parse::<u32>().map_err(|_| {
                        format!(
                            "\"{}\" is not a number of {field}. A composition needs whole \
                             numbers for its width, its height, its frame rate and its length.",
                            text.trim()
                        )
                    }),
                }
            };
            let (width, height, rate, frames) = match (
                number("width", 1920),
                number("height", 1080),
                number("fps", 24),
                number("frames", 240),
            ) {
                (Ok(w), Ok(h), Ok(r), Ok(f)) => (w, h, r, f),
                (Err(said), _, _, _)
                | (_, Err(said), _, _)
                | (_, _, Err(said), _)
                | (_, _, _, Err(said)) => return Some(said),
            };
            // The rate goes through the core's own constructor, so a zero is refused by the
            // rule that owns it rather than by a second copy of that rule written here.
            let Ok(frame_rate) = FrameRate::new(rate, 1) else {
                return Some(
                    "A frame rate of zero is not a frame rate. Twenty-four is the one the \
                     reference shot uses."
                        .to_string(),
                );
            };
            let composition = {
                let held = viewer.lock().expect("the viewer lock was poisoned");
                Composition::new(
                    unused_composition_id(held.document.project()),
                    name,
                    width,
                    height,
                    frame_rate,
                    0,
                    frames,
                )
            };
            let id = composition.id.clone();
            let said = edit(
                viewer,
                Command::AddComposition {
                    composition: Box::new(composition),
                },
            );
            // Only if the core took it. A refused command must leave the window looking at what
            // it was looking at, and `show` would otherwise move it to a composition that is
            // not in the project.
            let made = viewer
                .lock()
                .expect("the viewer lock was poisoned")
                .document
                .project()
                .composition(&id)
                .is_some();
            if made {
                show(viewer, &id);
                return Some(format!(
                    "{said}, {width}x{height} at {rate} fps, {frames} frames. It is empty; \
                     import drawings and add layers to fill it."
                ));
            }
            return Some(said);
        }
        // Which composition the window is looking at. Not an edit, for the same reason the two
        // inspection toggles above are not: nothing about the project changes, the window looks
        // somewhere else. It is also the way back out of a composition that has just been made,
        // and without it `composition.create` is a one-way door - the owner found that the first
        // time they used it, with no way to return to the shot they had been working on.
        "composition.open" => {
            let Some(asked) = parameter(query, "composition") else {
                return Some(
                    "Which composition should be opened? Choose one in the project panel."
                        .to_string(),
                );
            };
            let id = Id::new(&asked);
            let named = viewer
                .lock()
                .expect("the viewer lock was poisoned")
                .document
                .project()
                .composition(&id)
                .map(|comp| comp.name.clone());
            return Some(match named {
                None => format!("There is no composition {asked} in this project."),
                Some(name) => {
                    show(viewer, &id);
                    format!("{name} is on screen.")
                }
            });
        }
        "media.import" => {
            let files: Vec<PathBuf> = parameters(query, "file")
                .into_iter()
                .map(PathBuf::from)
                .collect();
            return Some(match files.is_empty() {
                true => "Which drawings should be imported? Choose the files themselves, not \
                         the folder they are in."
                    .to_string(),
                false => import(viewer, &files),
            });
        }
        _ => {}
    }
    // A drawing number an exposure names that its sequence has not got, filled in by the arm
    // that can see both and said at the end, once the core has accepted the change.
    let mut absent: Option<u32> = None;
    let command = {
        let held = viewer.lock().expect("the viewer lock was poisoned");
        let composition = held.composition.clone();
        let project = held.document.project();
        let Some(comp) = project.composition(&composition) else {
            return Some("There is no composition on screen to edit.".to_string());
        };
        if id == "layer.create" {
            // The one layer command that names no existing layer. It names a drawing instead,
            // because document 19's layer has an `asset_id` and no state in which it has none.
            let Some(asset) = parameter(query, "asset").map(Id::new) else {
                return Some("Which drawing should the new layer show?".to_string());
            };
            if !project.assets.iter().any(|a| a.id == asset) {
                return Some(format!("There is no imported drawing called {asset}."));
            }
            let layer = Layer::new(
                unused_layer_id(project),
                parameter(query, "name").unwrap_or_else(|| "New layer".to_string()),
                asset,
                comp.start_frame,
                comp.start_frame + comp.duration_frames as i32,
            );
            // The end of the order is the front of the picture -- `layers_in_order` is bottom
            // first -- and the front is where somebody who has just added a layer looks for it.
            Command::AddLayer {
                composition,
                layer: Box::new(layer),
                index: comp.len(),
            }
        } else {
            let Some(layer_id) = parameter(query, "layer").map(Id::new) else {
                return Some("Which layer? Choose one in the layer list.".to_string());
            };
            let Some(layer) = comp.layer(&layer_id) else {
                return Some(format!("{layer_id} is not a layer in this composition."));
            };
            let at = comp
                .index_of(&layer_id)
                .expect("a layer that is here has a place");
            match id {
                "layer.delete" => Command::RemoveLayer {
                    composition,
                    layer_id,
                },
                // Not refused for being the name it already has. Document 26 makes that a
                // history entry that changes nothing, which is honest: somebody pressed F2 and
                // pressed return, and undo should take them back to before they did.
                "layer.rename" => match parameter(query, "name") {
                    Some(name) => Command::RenameLayer {
                        composition,
                        layer_id,
                        name,
                    },
                    None => return Some("A layer needs a name.".to_string()),
                },
                // Document 24 calls both of these a toggle, so the new value is read from the
                // document rather than sent by the page. A page that sent it would be deciding
                // what the layer currently is from a list that may be stale.
                "layer.toggle_visibility" => Command::SetLayerEnabled {
                    composition,
                    layer_id,
                    value: !layer.enabled,
                },
                "layer.toggle_lock" => Command::SetLayerLocked {
                    composition,
                    layer_id,
                    value: !layer.locked,
                },
                // Toward the front is later in the order. Asking to move the front layer further
                // forward is not an error and not a history entry: it is somebody pressing the
                // key one more time, and it is told so rather than given an undo item that
                // undoes nothing.
                "layer.move_up" if at + 1 < comp.len() => Command::ReorderLayer {
                    composition,
                    layer_id,
                    to_index: at + 1,
                },
                "layer.move_down" if at > 0 => Command::ReorderLayer {
                    composition,
                    layer_id,
                    to_index: at - 1,
                },
                "layer.move_up" => return Some(format!("{} is already at the front.", layer.name)),
                "layer.move_down" => {
                    return Some(format!("{} is already at the back.", layer.name))
                }
                // W-05: the two halves of document 20's sentence, as the bar on the timeline
                // sends them. Both take frames rather than a distance, so that a drag which
                // sends the same numbers twice has asked for the same thing twice.
                "layer.shift" => Command::ShiftLayer {
                    composition,
                    layer_id,
                    in_frame: match frame_parameter(query, "in") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    },
                },
                "layer.trim" => Command::TrimLayer {
                    composition,
                    layer_id,
                    in_frame: match frame_parameter(query, "in") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    },
                    out_frame: match frame_parameter(query, "out") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    },
                },
                // W-10: document 24's toggle. Where the property has no key on the frame, one
                // is set holding the value the property already has there, so pressing the
                // diamond changes nothing on screen and only begins to hold it; where it has
                // one, that key is removed. Which of the two is read from the document, as the
                // other toggles are, rather than sent by a page that may be describing a key
                // that has since been undone.
                "keyframe.add_remove" => {
                    let Some(prop) = parameter(query, "prop").as_deref().and_then(property) else {
                        return Some(
                            "Which property? Say anchor, position, scale, rotation or opacity."
                                .to_string(),
                        );
                    };
                    let frame = match frame_parameter(query, "frame") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    };
                    let property = layer.transform.get(prop);
                    match property.keyframe_at(frame) {
                        Some(_) => Command::RemoveKeyframe {
                            composition,
                            layer_id,
                            prop,
                            frame,
                        },
                        None => Command::SetKeyframe {
                            composition,
                            layer_id,
                            prop,
                            frame,
                            value: property.value_at(frame),
                            interp: Interp::Linear,
                        },
                    }
                }
                // W-11: a key taken hold of on its property's row and put down on another frame.
                // The page names both frames rather than a distance, as `layer.shift` does, so
                // that what arrives is the whole move however many pointer moves drew it.
                //
                // No `drag=1` on this one, and that is deliberate: a drag record is replayed by
                // redo against the state the drag began in, and a move names the frame it starts
                // from, so an intermediate one would name a frame the key had already left. The
                // page follows the pointer with a mark of its own and sends this once, on
                // release, which is one history entry either way.
                "keyframe.move" => {
                    let Some(prop) = parameter(query, "prop").as_deref().and_then(property) else {
                        return Some(
                            "Which property? Say anchor, position, scale, rotation or opacity."
                                .to_string(),
                        );
                    };
                    let from_frame = match frame_parameter(query, "from") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    };
                    let to_frame = match frame_parameter(query, "to") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    };
                    Command::MoveKeyframe {
                        composition,
                        layer_id,
                        prop,
                        from_frame,
                        to_frame,
                    }
                }
                // D-52's preset, which is the third thing a keyframe can be set to. Document 20
                // says the mode belongs to the segment that *starts* at the keyframe, so this
                // changes what happens between this key and the next one and nothing else - it
                // is not After Effects' F9, which eases both sides of the key it is pressed on.
                // The page's label says so.
                //
                // The value is read out of the document rather than sent, for the same reason
                // `keyframe.add_remove` reads which of its two halves to do: a page can be
                // describing a key that has since been undone, and the value is not the thing
                // being changed here.
                "keyframe.set_interp" => {
                    let Some(prop) = parameter(query, "prop").as_deref().and_then(property) else {
                        return Some(
                            "Which property? Say anchor, position, scale, rotation or opacity."
                                .to_string(),
                        );
                    };
                    let frame = match frame_parameter(query, "frame") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    };
                    let interp = match parameter(query, "mode").as_deref() {
                        Some("hold") => Interp::Hold,
                        Some("linear") => Interp::Linear,
                        // Without `curve`, this is the preset button beside the diamond and
                        // the curve is easy ease. With it, it is the graph editor sending the
                        // four numbers a handle was just dragged to.
                        Some("ease") => match parameter(query, "curve") {
                            None => Interp::EASY,
                            Some(text) => match handles(&text) {
                                Ok(interp) => interp,
                                Err(said) => return Some(said),
                            },
                        },
                        other => {
                            return Some(format!(
                                "An interpolation is hold, linear or ease. Not \"{}\".",
                                other.unwrap_or("")
                            ))
                        }
                    };
                    let property = layer.transform.get(prop);
                    let Some(key) = property.keyframe_at(frame) else {
                        return Some(format!(
                            "{prop} has no keyframe at frame {frame}, so there is no segment to \
                             ease. Add a keyframe first."
                        ));
                    };
                    Command::SetKeyframe {
                        composition,
                        layer_id,
                        prop,
                        frame,
                        value: key.value,
                        interp,
                    }
                }
                // Typed into a field, or scrubbed on its label. The same command either way;
                // what differs is whether it becomes a history entry on its own or joins the one
                // the drag will commit at release.
                //
                // W-10: on a property that has keyframes the base is not what is drawn, so the
                // same request sets a key at the frame the page names instead, keeping the
                // interpolation a key already there was given. This is what After Effects does
                // when a value is changed on an animated property, and it is why the page sends
                // the frame with every value: the window decides which of the two it means.
                "property.set_base" | "property.drag_update" => {
                    let Some(prop) = parameter(query, "prop").as_deref().and_then(property) else {
                        return Some(
                            "Which property? Say anchor, position, scale, rotation or opacity."
                                .to_string(),
                        );
                    };
                    let Some(text) = parameter(query, "value") else {
                        return Some(format!("What should {prop} be set to?"));
                    };
                    let Some(value) = property_value(prop, &text) else {
                        // Named before the core sees it, because the core's refusal is about
                        // kinds and this one is about the text not being numbers at all.
                        return Some(match prop.kind() {
                            "vec2" => {
                                format!("{prop} needs two numbers, like 12, -4. Not \"{text}\".")
                            }
                            _ => format!("{prop} needs a number. Not \"{text}\"."),
                        });
                    };
                    let property = layer.transform.get(prop);
                    if !property.is_animated() {
                        Command::SetPropertyBase {
                            composition,
                            layer_id,
                            prop,
                            value,
                        }
                    } else {
                        let frame = match frame_parameter(query, "frame") {
                            Ok(frame) => frame,
                            Err(_) => {
                                return Some(format!(
                                    "{prop} is keyframed, so a value belongs to a frame. Say \
                                     frame=<frame>."
                                ))
                            }
                        };
                        Command::SetKeyframe {
                            composition,
                            layer_id,
                            prop,
                            frame,
                            value,
                            interp: property
                                .keyframe_at(frame)
                                .map_or(Interp::Linear, |k| k.interp),
                        }
                    }
                }
                // One command for all three of choosing a matte, changing whether the matte
                // layer is still drawn in its own right, and clearing it. Document 19 holds
                // them in one record and the core takes them in one command, so splitting them
                // here would only give the page a way to send half of one.
                //
                // No layer named means no matte, which is what the "none" entry in the list
                // sends. A layer that has gone is not the same thing and is refused by the
                // core, which is the reader that knows what is in the composition.
                "layer.set_matte" => Command::SetMatte {
                    composition,
                    layer_id,
                    matte: parameter(query, "matte")
                        .filter(|m| !m.is_empty())
                        .map(Id::new),
                    matte_only: parameter(query, "only").as_deref() == Some("true"),
                },
                // Document 24's `exposure.set_span`, which assigns one span. The core takes the
                // whole ordered list, so the list is built here out of the one the layer has:
                // a span starting where an existing one starts replaces it, which is what
                // editing a row does, and anything else is inserted in frame order. Whether the
                // result is legal is document 20's rule and stays in the core.
                //
                // No drawing number means the row is being cleared, which is the only way to
                // remove an exposure and is not the same as setting it to drawing zero.
                "exposure.set_span" => {
                    let Some(start) = parameter(query, "start") else {
                        return Some("Which frame does the exposure start at?".to_string());
                    };
                    let Ok(start) = start.parse::<i32>() else {
                        return Some(format!(
                            "An exposure starts at a whole frame number. Not \"{start}\"."
                        ));
                    };
                    // Which row this is, when it is not the frame the row is moving to. An
                    // exposure moved to another frame is one row edited, not one added and one
                    // left behind, and the page is the only thing that knows which row was
                    // being typed in.
                    let at = match parameter(query, "was") {
                        None => start,
                        Some(was) => match was.parse::<i32>() {
                            Ok(was) => was,
                            Err(_) => {
                                return Some(format!(
                                    "An exposure starts at a whole frame number. Not \"{was}\"."
                                ))
                            }
                        },
                    };
                    let mut spans = layer.exposure_spans.clone();
                    let held = spans.iter().position(|s| s.start_frame == at);
                    if let Some(index) = held {
                        spans.remove(index);
                    }
                    match parameter(query, "drawing").filter(|d| !d.is_empty()) {
                        None if held.is_none() => {
                            return Some(format!(
                                "There is no exposure starting at frame {at} to clear."
                            ))
                        }
                        None => {}
                        Some(drawing) => {
                            let Ok(drawing_number) = drawing.parse::<u32>() else {
                                return Some(format!(
                                    "A drawing number is a whole number, counting from zero. \
                                     Not \"{drawing}\"."
                                ));
                            };
                            let Some(end) = parameter(query, "end") else {
                                return Some(
                                    "Which frame does the exposure end before?".to_string(),
                                );
                            };
                            let Ok(end_frame_exclusive) = end.parse::<i32>() else {
                                return Some(format!(
                                    "An exposure ends before a whole frame number. Not \"{end}\"."
                                ));
                            };
                            let span = ExposureSpan {
                                start_frame: start,
                                end_frame_exclusive,
                                drawing_number,
                            };
                            spans.retain(|s| s.start_frame != start);
                            spans.push(span);
                            spans.sort_by_key(|s| s.start_frame);
                            // Document 28 renders a frame whose drawing is absent as nothing
                            // and substitutes no neighbour. That is a decision somebody should
                            // meet while typing the number, not three hundred frames into an
                            // export, so the answer says it.
                            if !project
                                .assets
                                .iter()
                                .find(|a| a.id == layer.asset_id)
                                .is_some_and(|a| a.frames.contains_key(&drawing_number))
                            {
                                absent = Some(drawing_number);
                            }
                        }
                    }
                    Command::SetExposureSpans {
                        composition,
                        layer_id,
                        spans,
                    }
                }
                // A new effect goes on the end of the stack, which document 21 evaluates last,
                // because that is where somebody who has just added one looks for it.
                "effect.add" => {
                    let Some(type_id) = parameter(query, "type") else {
                        return Some(
                            "Which effect? Say core.gaussian_blur, core.exposure or core.tint."
                                .to_string(),
                        );
                    };
                    let Some(effect) = new_effect(&type_id) else {
                        return Some(format!(
                            "This build has no effect called {type_id}. It has \
                             core.gaussian_blur, core.exposure and core.tint."
                        ));
                    };
                    Command::AddEffect {
                        composition,
                        layer_id,
                        effect: EffectInstance::new(unused_effect_id(project), effect),
                        index: None,
                    }
                }
                // The other six all name an instance that is already on the layer, so the
                // lookup and its refusal are written once.
                "effect.delete"
                | "effect.toggle_bypass"
                | "effect.set_parameters"
                | "effect.move"
                | "effect.move_up"
                | "effect.move_down" => {
                    let Some(instance_id) = parameter(query, "effect").map(Id::new) else {
                        return Some("Which effect? Choose one in the effects list.".to_string());
                    };
                    let Some(existing) =
                        layer.effects.iter().find(|e| e.instance_id == instance_id)
                    else {
                        return Some(format!("{instance_id} is not an effect on this layer."));
                    };
                    // Where in the stack it is now. The list in the window is drawn in
                    // evaluation order, first at the top, so up is one step earlier.
                    let at = layer
                        .effects
                        .iter()
                        .position(|e| e.instance_id == instance_id)
                        .unwrap_or(0);
                    match id {
                        "effect.delete" => Command::RemoveEffect {
                            composition,
                            layer_id,
                            instance_id,
                        },
                        // Refused in words at either end rather than clamped: an arrow that
                        // silently did nothing would read as a window that had stopped
                        // listening, which is the thing the first sitting with this build
                        // complained of.
                        "effect.move_up" if at == 0 => {
                            return Some(format!("{instance_id} is already first."));
                        }
                        "effect.move_down" if at + 1 >= layer.effects.len() => {
                            return Some(format!("{instance_id} is already last."));
                        }
                        "effect.move_up" | "effect.move_down" => Command::ReorderEffect {
                            composition,
                            layer_id,
                            instance_id,
                            to_index: if id == "effect.move_up" {
                                at - 1
                            } else {
                                at + 1
                            },
                        },
                        // W-07: a card dragged up or down the stack lands at one position,
                        // sent once at the drop. Past the end is the core's refusal; the place
                        // it already has is refused here, because the core would write it as
                        // a change and it would be an undo step that undoes nothing.
                        "effect.move" => {
                            let Some(to_index) =
                                parameter(query, "to").and_then(|to| to.parse::<usize>().ok())
                            else {
                                return Some(
                                    "Where to? Send the position in the stack, counted from \
                                     nought."
                                        .to_string(),
                                );
                            };
                            if to_index == at {
                                return Some(format!(
                                    "{instance_id} is already at position {to_index}."
                                ));
                            }
                            Command::ReorderEffect {
                                composition,
                                layer_id,
                                instance_id,
                                to_index,
                            }
                        }
                        "effect.toggle_bypass" => Command::SetEffectEnabled {
                            composition,
                            layer_id,
                            instance_id,
                            enabled: !existing.enabled,
                        },
                        // The type is read off the effect rather than sent by the page. The
                        // core refuses settings of the wrong kind for an instance, and this
                        // makes that mistake unreachable from here instead of catchable.
                        _ => match effect_parameters(existing.type_id(), query) {
                            Ok(effect) => Command::SetEffectParameters {
                                composition,
                                layer_id,
                                instance_id,
                                effect,
                            },
                            Err(said) => return Some(said),
                        },
                    }
                }
                _ => return None,
            }
        }
    };
    let before = viewer
        .lock()
        .expect("the viewer lock was poisoned")
        .document
        .undo_depth();
    // W-05: any edit joins the running drag when the page says so. A bar pulled along the
    // timeline sends `layer.shift` or `exposure.set_span` many times a second, and one press
    // has to be one thing to undo whichever command is inside it, which is the rule
    // `property.drag_update` already has for the transform fields.
    let said = if id == "property.drag_update" || parameter(query, "drag").is_some() {
        drag_update(viewer, command)
    } else {
        edit(viewer, command)
    };
    // Only when the change landed. A command the core refused changed nothing, and a warning
    // about what it would have done reads as though it had happened.
    let landed = viewer
        .lock()
        .expect("the viewer lock was poisoned")
        .document
        .undo_depth()
        > before;
    if let (Some(drawing), true) = (absent, landed) {
        return Some(format!(
            "{said} Drawing {drawing} is not in this sequence, so the frames exposing it stay \
             empty; no neighbouring drawing is put there instead."
        ));
    }
    // Deleting a layer that another layer was using as its matte leaves that reference behind.
    // The project loader treats it as a warning and keeps it rather than clearing it, so this
    // is a legal state and not a fault -- but nothing else would tell the person at the moment
    // it happened, and they would meet it as a warning the next time the file was opened.
    Some(match dangling_mattes(viewer) {
        names if id == "layer.delete" && !names.is_empty() => format!(
            "{said} {} now shaped by a layer that is not here; undo puts it back.",
            match names.as_slice() {
                [one] => format!("\"{one}\" is"),
                many => format!("{} layers are", many.len()),
            }
        ),
        _ => said,
    })
}

/// The names of layers whose matte points at a layer the composition does not have.
fn dangling_mattes(viewer: &Mutex<Viewer>) -> Vec<String> {
    let held = viewer.lock().expect("the viewer lock was poisoned");
    let Some(comp) = held.document.project().composition(&held.composition) else {
        return Vec::new();
    };
    comp.layers_in_order()
        .filter(|l| {
            l.matte
                .as_ref()
                .is_some_and(|m| comp.layer(&m.layer_id).is_none())
        })
        .map(|l| l.name.clone())
        .collect()
}

// -------------------------------------------------------------------------------------------
// Export
// -------------------------------------------------------------------------------------------

/// The export the window has running, and what the last one did.
///
/// Beside the viewer rather than inside it, and for one reason: an export outlives the project it
/// came from. It works on a snapshot taken when the person asked for it, so opening another
/// project or recovering a snapshot while it runs replaces the [`Viewer`] and must not take away
/// the Cancel button or the report of a job still writing files.
#[derive(Default)]
struct Export {
    /// Set while a job is running. Setting it true is the whole of Cancel — `export_sequence`
    /// reads it between frames, which is what R-09's "cancellation between frames" means and why
    /// a cancelled job's files are whole ones.
    cancel: Option<Arc<AtomicBool>>,
    /// What the running job is doing, or what the last one did, in the core's words.
    said: String,
}

/// PNG depth and alpha for an export from the window.
///
/// R-09 makes both a choice and `ExportRequest` carries them; the window does not offer the
/// choice yet, so it states the default it is taking rather than leaving the reader to guess.
/// Eight bits and straight alpha is what document 21 line 31 asks for and what `T-08_frames/`
/// was written with, so what this window exports is comparable with what the fixtures committed.
const WINDOW_DEPTH: OutputDepth = OutputDepth::Eight;
const WINDOW_ALPHA: OutputAlpha = OutputAlpha::Straight;

/// Everything a job needs, taken from the viewer under one lock and owned from then on.
///
/// This is B-10's immutable export snapshot, and it is a `clone` rather than a lock held for four
/// minutes: what gets written is the shot as it was at the instant the person asked, whatever
/// happens to the open document while it is being written.
///
/// The range is the work area, which is the same first and last frame the transport steps between,
/// so what is exported is what the viewer plays.
fn export_job(
    viewer: &Viewer,
    into: &Path,
    missing: MissingSource,
) -> (Project, PathBuf, ExportRequest) {
    let first = viewer.playback.at_rest();
    let last = first + viewer.playback.length() as i32 - 1;
    // The project's own name, so two shots exported into one folder do not overwrite each other.
    let stem = Path::new(&viewer.name)
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "shot".to_string());
    (
        viewer.document.project().clone(),
        viewer.root.clone(),
        ExportRequest {
            composition: viewer.composition.clone(),
            first_frame: first,
            last_frame: last,
            output_dir: into.to_path_buf(),
            naming: format!("{stem}_%04d.png"),
            depth: WINDOW_DEPTH,
            alpha: WINDOW_ALPHA,
            tile_size: DEFAULT_TILE_SIZE,
            missing,
        },
    )
}

/// Run a job to the end and say what it did in the window's one line.
fn run_export(
    project: &Project,
    root: &Path,
    request: &ExportRequest,
    cancel: &AtomicBool,
) -> String {
    what_the_export_did(
        &export::export_sequence(project, root, request, cancel),
        &request.output_dir,
    )
}

/// An export report as a sentence, with every diagnostic the core produced kept.
///
/// Nothing here summarises: the core's `FrameLog` has already capped repeated warnings at three
/// and a count (D-25), so what arrives is bounded, and folding it further would be this window
/// deciding what the person is allowed to know about their own render.
fn what_the_export_did(report: &ExportReport, into: &Path) -> String {
    let mut lines = vec![match report.status {
        ExportStatus::Completed => format!(
            "Exported {} frames into {}.",
            report.written.len(),
            into.display()
        ),
        ExportStatus::Blocked => "Nothing was exported.".to_string(),
        ExportStatus::Cancelled => format!(
            "The {} frames that finished are in {}.",
            report.written.len(),
            into.display()
        ),
        ExportStatus::Failed => format!(
            "The export stopped on a problem after {} of {} frames, in {}.",
            report.written.len(),
            report.frames_requested,
            into.display()
        ),
    }];
    // Document 28: output produced with something left out says so, in the report as well as in
    // the file's own tag.
    if report.fidelity_incomplete {
        lines.push(
            "These frames are not a faithful render: something this build does not support was \
             left out of them."
                .to_string(),
        );
    }
    lines.extend(report.diagnostics.iter().map(sentence));
    lines.join(" ")
}

/// Start an export into `into`, or say why not. Returns what the status line should say now.
fn start_export(app: &AppHandle, into: &Path, missing: MissingSource) -> String {
    let state = app.state::<Mutex<Export>>();
    if state
        .lock()
        .expect("the export lock was poisoned")
        .cancel
        .is_some()
    {
        return "An export is already running. Cancel it, or wait for it to finish.".to_string();
    }
    let (project, root, request) = {
        let viewer = app.state::<Mutex<Viewer>>();
        let viewer = viewer.lock().expect("the viewer lock was poisoned");
        export_job(&viewer, into, missing)
    };
    let said = format!(
        "Exporting {} frames into {}. The window stays usable while it writes.",
        request.last_frame - request.first_frame + 1,
        into.display()
    );
    let cancel = Arc::new(AtomicBool::new(false));
    {
        let mut export = state.lock().expect("the export lock was poisoned");
        export.cancel = Some(Arc::clone(&cancel));
        export.said = said.clone();
    }
    // A thread, so the window keeps answering for frames while a shot is being written: an
    // export of the reference shot takes minutes, and a viewer frozen for minutes is a viewer
    // that looks broken.
    let handle = app.clone();
    std::thread::spawn(move || {
        let done = run_export(&project, &root, &request, &cancel);
        let state = handle.state::<Mutex<Export>>();
        let mut export = state.lock().expect("the export lock was poisoned");
        export.cancel = None;
        export.said = done;
    });
    said
}

/// Ask the operating system which folder the frames go in, then start writing them there.
fn ask_where_to_export(app: &AppHandle, missing: MissingSource) {
    let handle = app.clone();
    app.dialog()
        .file()
        .set_title("Export the frames into a folder")
        .pick_folder(move |chosen| {
            let Some(into) = chosen.and_then(|c| c.into_path().ok()) else {
                return;
            };
            let said = start_export(&handle, &into, missing);
            announce(&handle.state::<Mutex<Viewer>>(), said);
            refresh(&handle);
        });
}

/// Ask a running export to stop. It stops between frames, so the file being written finishes.
fn cancel_export(app: &AppHandle) -> String {
    let state = app.state::<Mutex<Export>>();
    let export = state.lock().expect("the export lock was poisoned");
    match &export.cancel {
        Some(flag) => {
            flag.store(true, Ordering::SeqCst);
            "Stopping the export. The frame being written will be finished first.".to_string()
        }
        None => "No export is running.".to_string(),
    }
}

// -------------------------------------------------------------------------------------------
// The recent list
// -------------------------------------------------------------------------------------------

/// How many projects the recent list remembers. Long enough to cover a working day's shots,
/// short enough to read without scrolling.
const RECENT_LIMIT: usize = 8;

/// The recent list after opening `opened`: most recent first, no duplicates, capped.
///
/// Pure, so it can be checked without a window. Comparison is case-insensitive because this is
/// a Windows-only product per ADR-001 and `C:\Shots\a.json` and `c:\shots\A.JSON` are one file;
/// two entries for it would be two ways to open the same project and one of them would look
/// like a different one.
fn remembered(existing: &[String], opened: &str) -> Vec<String> {
    let mut list = vec![opened.to_string()];
    for entry in existing {
        if !entry.eq_ignore_ascii_case(opened) && !entry.trim().is_empty() {
            list.push(entry.clone());
        }
    }
    list.truncate(RECENT_LIMIT);
    list
}

/// Where the recent list is kept: one absolute path per line, in the application's own
/// configuration directory.
///
/// Beside the application rather than beside a project, because it is about the person and not
/// about any one shot. Plain text rather than JSON because a path cannot contain a newline on
/// this platform, so lines are enough, and because a person who wants to clear the list should
/// be able to look at the file and see what it is.
fn recent_file(app: &AppHandle) -> Option<PathBuf> {
    let directory = app.path().app_config_dir().ok()?;
    std::fs::create_dir_all(&directory).ok()?;
    Some(directory.join("recent.txt"))
}

/// The recent list, newest first, with everything that is no longer on disk left out.
///
/// Offering a file that has been moved or deleted would turn the list into a source of failed
/// opens; a project that comes back is simply offered again the next time it is opened.
fn recent(app: &AppHandle) -> Vec<String> {
    let Some(file) = recent_file(app) else {
        return Vec::new();
    };
    std::fs::read_to_string(file)
        .unwrap_or_default()
        .lines()
        .filter(|line| !line.trim().is_empty() && Path::new(line).is_file())
        .map(str::to_string)
        .collect()
}

/// Put `path` at the top of the recent list. A failure to write it is not worth telling anyone
/// about: the list is a convenience and nothing depends on it.
fn remember(app: &AppHandle, path: &Path) {
    let Some(file) = recent_file(app) else { return };
    let absolute = std::fs::canonicalize(path)
        .unwrap_or_else(|_| path.to_path_buf())
        .display()
        .to_string();
    // Windows canonicalisation returns the \\?\ form, which is correct and unreadable. The list
    // is shown to a person, so the prefix comes off; it is not part of the path's identity.
    let absolute = absolute
        .strip_prefix(r"\\?\")
        .unwrap_or(&absolute)
        .to_string();
    let list = remembered(&recent(app), &absolute);
    let _ = std::fs::write(file, list.join("\n"));
}

// -------------------------------------------------------------------------------------------
// The commands
// -------------------------------------------------------------------------------------------

/// The value of `name` in a query string, percent-decoded.
/// Every value given for one parameter, in the order they appear.
///
/// [`parameter`] takes the first, which is what a setting with one value wants. An import names
/// one file per value: a selection is a list, and joining it into a single string would need a
/// separator, which is a character a Japanese file name is entitled to contain.
fn parameters(query: Option<&str>, name: &str) -> Vec<String> {
    let prefix = format!("{name}=");
    query
        .into_iter()
        .flat_map(|q| q.split('&'))
        .filter_map(|pair| pair.strip_prefix(&prefix))
        .map(from_a_query)
        .collect()
}

/// Whether this export writes the frames whose drawings are missing.
///
/// Document 07's default is that a missing drawing blocks a final export. `?missing=write` is the
/// person overriding it in front of the checkbox that says what it does, which is document 28's
/// recorded override rather than a silent fallback. Written down as its own function because it
/// is the one place in this file where a query string decides what reaches the disk: everything
/// that is not exactly `write` blocks, and that has a table row of its own.
fn missing_source(query: Option<&str>) -> MissingSource {
    match parameter(query, "missing").as_deref() {
        Some("write") => MissingSource::RenderTransparent,
        _ => MissingSource::Block,
    }
}

fn parameter(query: Option<&str>, name: &str) -> Option<String> {
    let prefix = format!("{name}=");
    query
        .and_then(|q| q.split('&').find_map(|pair| pair.strip_prefix(&prefix)))
        .map(from_a_query)
}

/// The four numbers of a D-52 curve as the page sends them, or the sentence to answer with.
///
/// Document 19 bounds `x` to the segment and leaves `y` free for overshoot; `persist` refuses a
/// file that breaks that, and this refuses a drag that would write one, so the same rule is
/// enforced on the way in as on the way back.
fn handles(text: &str) -> Result<Interp, String> {
    let numbers: Vec<f64> = text
        .split(',')
        .filter_map(|n| n.trim().parse().ok())
        .collect();
    let [x1, y1, x2, y2] = numbers[..] else {
        return Err(format!(
            "A curve is four numbers, x1,y1,x2,y2. Not \"{text}\"."
        ));
    };
    if !(0.0..=1.0).contains(&x1) || !(0.0..=1.0).contains(&x2) {
        return Err(format!(
            "A curve handle cannot reach outside its own segment, so x1 and x2 are between 0 \
             and 1. Not {x1} and {x2}."
        ));
    }
    Ok(Interp::Ease { x1, y1, x2, y2 })
}

/// A frame number the page named, or the sentence to answer with when it did not.
fn frame_parameter(query: Option<&str>, name: &str) -> Result<i32, String> {
    let Some(value) = parameter(query, name) else {
        return Err(format!("Which frame? Say {name}=<frame>."));
    };
    value
        .parse::<i32>()
        .map_err(|_| format!("A frame is a whole number. Not \"{value}\"."))
}

/// Undo [`for_a_header`]: percent-decoded back into the string that was encoded.
///
/// The recent list travels to the page and a chosen entry travels back, and both directions
/// carry paths that can be Japanese. A half-decoded path is a path to a file that does not
/// exist, so this is the other half of the same contract and is checked against the same
/// hand-worked UTF-8 vectors.
fn from_a_query(text: &str) -> String {
    let raw = text.as_bytes();
    let mut bytes = Vec::with_capacity(raw.len());
    let mut i = 0;
    while i < raw.len() {
        let hex = |b: u8| (b as char).to_digit(16);
        match (raw[i], raw.get(i + 1).copied(), raw.get(i + 2).copied()) {
            (b'%', Some(h), Some(l)) => match (hex(h), hex(l)) {
                (Some(h), Some(l)) => {
                    bytes.push((h * 16 + l) as u8);
                    i += 3;
                }
                // Not an escape after all. Keep the % rather than eating the next two
                // characters, which would silently shorten a path.
                _ => {
                    bytes.push(b'%');
                    i += 1;
                }
            },
            _ => {
                bytes.push(raw[i]);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

/// Ask the operating system where a project is, then open it.
///
/// The dialog is opened from Rust and its answer never passes through the page, which is why
/// the page has no dialog permission and cannot ask for a file on its own. A cancelled dialog
/// says nothing and changes nothing.
fn ask_to_open(app: &AppHandle) {
    let handle = app.clone();
    app.dialog()
        .file()
        .set_title("Open a project")
        .add_filter("Anime Compositor project", &["json"])
        .pick_file(move |chosen| {
            let Some(path) = chosen.and_then(|c| c.into_path().ok()) else {
                return;
            };
            take(&handle.state::<Mutex<Viewer>>(), &path);
            remember(&handle, &path);
            refresh(&handle);
        });
}

/// Ask the operating system which drawings to import, then import them.
///
/// The dialog takes many files at once, because a sequence is a selection and B-03 groups the
/// files it is given. It filters to PNG, which is the format G1 reads; a person who selects
/// something else is told by the importer rather than by a dialog that will not let them.
fn ask_what_to_import(app: &AppHandle) {
    let handle = app.clone();
    app.dialog()
        .file()
        .set_title("Import drawings")
        .add_filter("PNG drawings", &["png"])
        .pick_files(move |chosen| {
            let files: Vec<PathBuf> = chosen
                .unwrap_or_default()
                .into_iter()
                .filter_map(|c| c.into_path().ok())
                .collect();
            if files.is_empty() {
                return;
            }
            let viewer = handle.state::<Mutex<Viewer>>();
            let said = import(&viewer, &files);
            announce(&viewer, said);
            refresh(&handle);
        });
}

/// Ask which drawings a sequence should point at instead, and work out what that would do.
///
/// Document 07: "Search only user-selected locations." Nothing here scans for a replacement; the
/// files are the ones a person chose, and what comes back is a proposal rather than a change.
fn ask_what_to_relink_to(app: &AppHandle, asset: Id) {
    let handle = app.clone();
    app.dialog()
        .file()
        .set_title("Relink to these drawings")
        .add_filter("PNG drawings", &["png"])
        .pick_files(move |chosen| {
            let files: Vec<PathBuf> = chosen
                .unwrap_or_default()
                .into_iter()
                .filter_map(|c| c.into_path().ok())
                .collect();
            if files.is_empty() {
                return;
            }
            let viewer = handle.state::<Mutex<Viewer>>();
            let said = propose_relink(&viewer, &asset, &files);
            announce(&viewer, said);
            refresh(&handle);
        });
}

/// Ask the operating system where to write the project, then write it there.
fn ask_where_to_save(app: &AppHandle) {
    let handle = app.clone();
    let suggestion = {
        let viewer = app.state::<Mutex<Viewer>>();
        let viewer = viewer.lock().expect("the viewer lock was poisoned");
        match &viewer.path {
            Some(path) => path.file_name().map(|n| n.to_string_lossy().into_owned()),
            None => None,
        }
        .unwrap_or_else(|| "project.json".to_string())
    };
    app.dialog()
        .file()
        .set_title("Save the project as")
        .set_file_name(suggestion)
        .add_filter("Anime Compositor project", &["json"])
        .save_file(move |chosen| {
            let Some(path) = chosen.and_then(|c| c.into_path().ok()) else {
                return;
            };
            let said = save_as(&handle.state::<Mutex<Viewer>>(), &path);
            announce(&handle.state::<Mutex<Viewer>>(), said);
            remember(&handle, &path);
            refresh(&handle);
        });
}

/// Reload the page, which is the whole of the update after a command that changed what is open.
///
/// The page holds no state about the project — everything it says arrives with a frame — so
/// there is nothing to keep in step and nothing to invalidate.
fn refresh(app: &AppHandle) {
    if let Some(page) = app.get_webview_window("main") {
        let _ = page.eval("location.reload()");
    }
}

/// Answer one command from the page. The body is what the status line should say, or the recent
/// list, one path per line.
fn command(app: &AppHandle, path: &str, query: Option<&str>) -> Response<Vec<u8>> {
    let viewer = app.state::<Mutex<Viewer>>();
    let path = path.trim_matches('/');
    // The one answer that is not a sentence. It is large, it is asked for after every edit, and
    // it is JSON, so it leaves here rather than through the status line the rest of these share.
    if path == "state" {
        return allow_the_page_to_read_this(Response::builder())
            .header("content-type", "application/json; charset=utf-8")
            .body(state(&viewer).into_bytes())
            .expect("build the state response");
    }
    // An import with no files named is the button in the media bin, and what it needs is the
    // operating system's file dialog, which belongs to the app handle and not to the viewer.
    // Answered before `edit_command`, which would otherwise refuse it for naming no files.
    // The same for the first step of a relink, which needs the dialog for the same reason. The
    // other two steps carry `apply` or `cancel` and are answered by `edit_command` below.
    if path == "media.relink"
        && parameter(query, "file").is_none()
        && parameter(query, "apply").is_none()
        && parameter(query, "cancel").is_none()
    {
        if let Some(asset) = parameter(query, "asset") {
            ask_what_to_relink_to(app, Id::new(&asset));
            return allow_the_page_to_read_this(Response::builder())
                .header("content-type", "text/plain; charset=utf-8")
                .body(Vec::new())
                .expect("build the relink response");
        }
    }
    if path == "media.import" && parameter(query, "file").is_none() {
        ask_what_to_import(app);
        return allow_the_page_to_read_this(Response::builder())
            .header("content-type", "text/plain; charset=utf-8")
            .body(Vec::new())
            .expect("build the import response");
    }
    // An edit answers here rather than falling through the match below, because its answer is
    // also what the status line should say: the page asks for the frame again straight
    // afterwards, and that answer carries `x-status`, so a sentence not written into the viewer
    // would be replaced by the one before it before anybody could read it.
    if let Some(said) = edit_command(&viewer, path, query) {
        announce(&viewer, said.clone());
        return allow_the_page_to_read_this(Response::builder())
            .header("content-type", "text/plain; charset=utf-8")
            .body(said.into_bytes())
            .expect("build the command response");
    }
    let said = match path {
        "recent" => recent(app).join("\n"),
        // With a path, the recent list chose it. Without one, ask. Both end at `take`.
        "open" => match parameter(query, "path") {
            Some(chosen) => {
                let chosen = PathBuf::from(chosen);
                take(&viewer, &chosen);
                remember(app, &chosen);
                viewer
                    .lock()
                    .expect("the viewer lock was poisoned")
                    .status
                    .clone()
            }
            None => {
                ask_to_open(app);
                String::new()
            }
        },
        "save" => {
            let said = save(&viewer);
            announce(&viewer, said.clone());
            // A project with nowhere to go asks where, rather than refusing and leaving the
            // person to find the other button.
            if viewer
                .lock()
                .expect("the viewer lock was poisoned")
                .path
                .is_none()
            {
                ask_where_to_save(app);
            }
            said
        }
        "save-as" => {
            ask_where_to_save(app);
            String::new()
        }
        // The page only ever offers a path it was given in `x-recovery`, but this checks anyway:
        // a command scheme is reachable by anything running in the page.
        "recover" => match parameter(query, "path") {
            Some(chosen) => {
                let chosen = PathBuf::from(chosen);
                let known = viewer
                    .lock()
                    .expect("the viewer lock was poisoned")
                    .recovery
                    .contains(&chosen);
                if known {
                    recover(&viewer, &chosen)
                } else {
                    format!(
                        "{} is not a recovery snapshot for this project.",
                        chosen.display()
                    )
                }
            }
            None => "Which snapshot? Choose one from the recovery list.".to_string(),
        },
        // Document 07's default is that a missing drawing blocks a final export. `?missing=write`
        // is the person overriding it in front of the checkbox that says what it does, which is
        // document 28's recorded override rather than a silent fallback.
        "export" => {
            ask_where_to_export(app, missing_source(query));
            String::new()
        }
        "cancel-export" => cancel_export(app),
        _ => {
            return allow_the_page_to_read_this(Response::builder().status(404))
                .header("content-type", "text/plain; charset=utf-8")
                .body(
                    b"ask for /state, /open, /save, /save-as, /recover, /export, \
                      /cancel-export, /recent, or one of document 24's command IDs"
                        .to_vec(),
                )
                .expect("build the not-found response")
        }
    };
    allow_the_page_to_read_this(Response::builder())
        .header("content-type", "text/plain; charset=utf-8")
        .body(said.into_bytes())
        .expect("build the command response")
}

/// R-11 asks that nothing leaves the device. Nothing in this program tries to; the web view
/// component Windows supplies does, on its own account, and `tools/offline_check.ps1` caught it
/// holding connections to Microsoft addresses in twenty idle seconds. That is not this
/// application's code and it cannot be removed. These are the switches Chromium offers for it —
/// no background networking, no component updates, no reporting, no pings, no sync — and they are
/// set because leaving them unset would be a choice too.
///
/// They do not close it. The same twenty seconds with these on still showed connections, which is
/// recorded in `verification/B-11_offline_run.md` and registered as a decision for the owner
/// rather than described as solved. Do not delete these arguments on the grounds that they did not
/// work; they narrow what is left to explain.
///
/// Anything already in the variable is kept and appended to, because `tools/capture_window.ps1`
/// uses it to photograph the window at other display scales and overwriting it would silently
/// undo that.
fn quieten_the_web_view() {
    const OURS: &str = "--disable-background-networking --disable-component-update \
                        --disable-domain-reliability --no-pings --disable-sync \
                        --disable-features=DnsOverHttps,msSmartScreenProtection,msWebOOUI,msPdfOOUI";
    const NAME: &str = "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS";
    let value = match std::env::var(NAME) {
        Ok(existing) if !existing.trim().is_empty() => format!("{OURS} {existing}"),
        _ => OURS.to_string(),
    };
    std::env::set_var(NAME, value);
}

fn main() {
    quieten_the_web_view();
    // A project named on the command line goes through the same `take` a dropped one does, so
    // the two ways in cannot behave differently. It is also the only one a script can drive,
    // which is how the photographs under `verification/` are taken.
    let viewer = Mutex::new(demo());
    if let Some(named) = std::env::args_os().nth(1) {
        take(&viewer, Path::new(&named));
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(viewer)
        .manage(Mutex::new(Export::default()))
        // The autosave timer. A thread rather than anything cleverer: it sleeps for all but a
        // few microseconds of its life, it must run whether or not the page is asking for
        // frames, and it holds the viewer lock only for as long as the check takes. It writes
        // nothing to the page — a reload while somebody is working would be a worse
        // interruption than the one it is protecting them from — so what it did appears with
        // the next frame, which is within a sixtieth of a second of it happening.
        .setup(|app| {
            let handle = app.handle().clone();
            std::thread::spawn(move || loop {
                std::thread::sleep(AUTOSAVE_TICK);
                let state = handle.state::<Mutex<Viewer>>();
                let mut viewer = state.lock().expect("the viewer lock was poisoned");
                if let Some(said) = autosave_tick(&mut viewer, Instant::now()) {
                    viewer.autosaved = said;
                }
            });
            Ok(())
        })
        .on_window_event(|window, event| {
            let WindowEvent::DragDrop(DragDropEvent::Drop { paths, .. }) = event else {
                return;
            };
            let Some(path) = paths.first() else { return };
            take(&window.state::<Mutex<Viewer>>(), path);
            remember(window.app_handle(), path);
            refresh(window.app_handle());
        })
        .register_uri_scheme_protocol("project", |ctx, request: Request<Vec<u8>>| {
            command(
                ctx.app_handle(),
                request.uri().path(),
                request.uri().query(),
            )
        })
        .register_uri_scheme_protocol("frame", |ctx, request: Request<Vec<u8>>| {
            let viewer = ctx.app_handle().state::<Mutex<Viewer>>();
            let export = ctx.app_handle().state::<Mutex<Export>>();
            // `/boxes/<n>` rides on the frame scheme rather than the command one because it is
            // the same kind of thing a frame is: a question about what is on screen, whose
            // answer changes nothing and needs no undo record.
            if let Some(n) = request
                .uri()
                .path()
                .trim_matches('/')
                .strip_prefix("boxes/")
                .and_then(|n| n.parse::<i32>().ok())
            {
                return boxes(&viewer, n, quality_asked(request.uri().query()));
            }
            // `/curve?layer=&prop=&from=&to=`, the graph editor's samples: the same kind of
            // question as `boxes` above, on the same scheme, for the same reason.
            if request.uri().path().trim_matches('/') == "curve" {
                return curve(&viewer, request.uri().query());
            }
            match parse(request.uri().path(), request.uri().query()) {
                Some((ask, quality)) => serve(&viewer, &export, ask, quality),
                None => allow_the_page_to_read_this(Response::builder().status(404))
                    .header("content-type", "text/plain; charset=utf-8")
                    .body(b"ask for /at/<milliseconds>, /frame/<number> or /play/<number>".to_vec())
                    .expect("build the not-found response"),
            }
        })
        .run(tauri::generate_context!())
        .expect("the window could not be created");
}

/// What saving from the window does, checked without a window.
///
/// The dialogs cannot be reached from a test — a file dialog is the operating system's and a
/// test has no hands — but everything on this side of the person's answer can be, and that is
/// where a save can go wrong quietly. The three questions worth asking are whether the file that
/// arrives is the project that was open, whether the parts of it this build does not model
/// survive the trip, and whether a failed save leaves the previous file alone.
///
/// Writes `verification/B-09_save_table.md`.
#[cfg(test)]
mod saving {
    use super::*;

    struct Report {
        rows: Vec<(String, String, String)>,
        /// This machine's temporary directory, which appears in most of the values here and is
        /// different on every machine. CI checks that the committed artifact still matches what
        /// the tests produce, so a real path in a cell would fail the build on the runner for a
        /// reason that has nothing to do with saving.
        scratch: String,
    }

    impl Report {
        fn check(&mut self, check: &str, expected: impl ToString, actual: impl ToString) {
            let short = |text: String| text.replace(&self.scratch, "<a temporary directory>");
            self.rows.push((
                check.to_string(),
                short(expected.to_string()),
                short(actual.to_string()),
            ));
        }
    }

    fn repo(rel: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("the app crate has a parent directory")
            .join(rel)
    }

    /// A scratch directory of this test's own, emptied first so a previous run cannot make a
    /// later one pass.
    fn scratch() -> PathBuf {
        let directory = std::env::temp_dir().join("anime_compositor_b09_save");
        let _ = std::fs::remove_dir_all(&directory);
        std::fs::create_dir_all(&directory).expect("make the scratch directory");
        directory
    }

    fn viewer_on(path: &Path) -> Mutex<Viewer> {
        Mutex::new(open(path).unwrap_or_else(|d| panic!("open {}: {}", path.display(), d.message)))
    }

    #[test]
    fn what_the_window_writes_is_the_project_it_opened() {
        let scratch = scratch();
        let mut report = Report {
            rows: Vec::new(),
            scratch: scratch.display().to_string(),
        };

        // ---- a project this build does not fully understand -----------------------------------
        // Chosen on purpose: `unknown_effect_project.json` names an effect no version of this
        // build has. Everything B-09 promises about saving is visible in what happens to that
        // effect, and nothing else in the fixtures makes the promise checkable.
        let source = repo("Fixtures/projects/unknown_effect_project.json");
        let viewer = viewer_on(&source);

        let elsewhere = scratch.join("saved_elsewhere.json");
        let said = save_as(&viewer, &elsewhere);
        report.check(
            "Save As says where the project went",
            format!("Saved to {}", elsewhere.display()),
            &said,
        );
        report.check(
            "the file the person chose is now on disk",
            true,
            elsewhere.is_file(),
        );

        let written = std::fs::read_to_string(&elsewhere).expect("read what was written");
        report.check(
            "the effect this build does not have is still in the saved file",
            true,
            written.contains("vendor.future.effect"),
        );
        // Not "the file contains the string" twice: this is the whole project's worth of the
        // same question. `to_json` of the reopened file is what a *second* save would write, so
        // agreement means the trip through this build changed nothing at all.
        let reopened = persist::load(&elsewhere).expect("reopen what was written");
        report.check(
            "reopening the saved file and saving it again would write the same bytes",
            written.len(),
            persist::to_json(reopened.document.project(), &reopened.preserved).len(),
        );
        report.check(
            "and the same text, not merely the same length",
            true,
            persist::to_json(reopened.document.project(), &reopened.preserved) == written,
        );

        // ---- what the window is showing afterwards --------------------------------------------
        {
            let viewer = viewer.lock().expect("the viewer lock was poisoned");
            report.check(
                "after Save As the window is showing the file that was written",
                elsewhere.display().to_string(),
                viewer
                    .path
                    .as_ref()
                    .map_or(String::new(), |p| p.display().to_string()),
            );
            report.check(
                "and calls it by its new name",
                "saved_elsewhere.json",
                &viewer.name,
            );
            report.check(
                "with no unsaved work outstanding",
                false,
                viewer.document.is_dirty(),
            );
            // Media paths in a project file are relative to that file, so this is what decides
            // whether the reopened project can find its drawings at all. A window that kept the
            // old directory would show the picture correctly and hand somebody else a file whose
            // cels resolve to nothing — the failure Save As reopens the file to avoid.
            report.check(
                "and looks for its drawings beside the file it wrote, not beside the one it came \
                 from",
                "beside the file it wrote",
                if Some(viewer.root.as_path()) == elsewhere.parent() {
                    "beside the file it wrote"
                } else if Some(viewer.root.as_path()) == source.parent() {
                    "still beside the project it came from"
                } else {
                    "somewhere that is neither"
                },
            );
        }

        // ---- Save, with a file of its own ------------------------------------------------------
        let said = save(&viewer);
        report.check(
            "Save writes to the file the project came from",
            format!("Saved to {}", elsewhere.display()),
            &said,
        );
        report.check(
            "and writing it a second time changes nothing in it",
            written.clone(),
            std::fs::read_to_string(&elsewhere).expect("read it again"),
        );

        // ---- Save, without one -----------------------------------------------------------------
        // The state the window opens in: the built-in reference shot has no file of its own, and
        // Save must ask rather than choose somewhere.
        {
            let mut held = viewer.lock().expect("the viewer lock was poisoned");
            held.path = None;
        }
        report.check(
            "a project with no file of its own is not saved anywhere; the window asks",
            "This project has no file of its own yet. Use Save As.",
            save(&viewer),
        );

        // ---- a save that cannot be done --------------------------------------------------------
        // Document 25's FX-IO-002 in the shape a window can reach: somewhere unwritable. The
        // question is not whether it fails, it is what it leaves behind.
        let viewer = viewer_on(&elsewhere);
        let nowhere = scratch.join("no_such_directory").join("shot.json");
        let said = save_as(&viewer, &nowhere);
        report.check(
            "a save that cannot be done says so in the core's words",
            true,
            said.contains("could not be saved") || said.contains("not be written"),
        );
        report.check(
            "and does not leave a half-written file behind",
            false,
            nowhere.exists(),
        );
        report.check(
            "and the file that was already good is untouched",
            written,
            std::fs::read_to_string(&elsewhere).expect("read the good file"),
        );
        report.check(
            "and the window is still showing the project it had",
            elsewhere.display().to_string(),
            viewer
                .lock()
                .expect("the viewer lock was poisoned")
                .path
                .as_ref()
                .map_or(String::new(), |p| p.display().to_string()),
        );

        // The same failure reached by Ctrl+S instead of Save As. It is a second path through the
        // window and it fails on its own occasions — a project file gone read-only, a drive no
        // longer mounted — which Save As never touches, because Save As is always given a new
        // destination and Save is never given one at all.
        {
            let mut held = viewer.lock().expect("the viewer lock was poisoned");
            held.path = Some(nowhere.clone());
        }
        let said = save(&viewer);
        report.check(
            "and Save fails the same way, rather than naming a file it did not write",
            true,
            said.contains("could not be saved") || said.contains("not be written"),
        );

        write_artifact(&report);
        let failed: Vec<&(String, String, String)> =
            report.rows.iter().filter(|(_, e, a)| e != a).collect();
        assert!(
            failed.is_empty(),
            "{} of {} checks failed, see verification/B-09_save_table.md: {:#?}",
            failed.len(),
            report.rows.len(),
            failed
        );
    }

    fn write_artifact(report: &Report) {
        let passed = report.rows.iter().filter(|(_, e, a)| e == a).count();
        let mut out = String::new();
        out.push_str("# B-09, saving from the window\n\n");
        out.push_str(
            "The window could open a project and not write one back. It can now, and this is what \
             the writing does. Produced by `cargo test -p anime_compositor_app`, from \
             `app/src/main.rs`.\n\n",
        );
        out.push_str(
            "Every row here is about one promise: **what comes back off the disk is the project \
             that was open, including the parts this build does not understand.** The fixture is \
             `Fixtures/projects/unknown_effect_project.json`, which names an effect no version of \
             this build has. A window that saved only what it could model would drop that effect, \
             the file would still open, the picture would still look right, and the loss would be \
             found much later by the person who made the mask. That is the failure this table \
             exists to catch.\n\n",
        );
        out.push_str("| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
        for (check, expected, actual) in &report.rows {
            out.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                check,
                cell(expected),
                cell(actual),
                if expected == actual { "pass" } else { "FAIL" }
            ));
        }
        out.push_str(&format!(
            "\n**{} of {} checks pass.**\n",
            passed,
            report.rows.len()
        ));
        out.push_str(
            "\n## What this does not cover\n\nThe dialogs. A file dialog belongs to the operating \
             system and a test has no hands to answer one, so what is checked here begins at the \
             path the person chose. Choosing a file, and the Open and Save As dialogs that do the \
             choosing, are still unphotographed; the two photographs beside this table show a \
             Ctrl+S save of a project that already had a file, which is the one path a script can \
             drive from end to end.\n\nWhere a row says *a temporary directory*, \
             the real value was this machine's scratch directory, which is different on every \
             machine and on every run. The destination is shown rather than hidden — a save that \
             reports the wrong one is exactly the failure worth seeing — but the machine-specific \
             part of it is not, because this file is committed and checked.\n",
        );
        let path = repo("verification/B-09_save_table.md");
        std::fs::write(path, out).expect("write the artifact");
    }

    /// A path in a table cell, with the scratch directory's own separators left alone but the
    /// table's separator escaped, so one row cannot silently become two columns.
    fn cell(text: &str) -> String {
        text.replace('|', r"\|")
    }
}

/// What the editing panels can do to the open document, checked without a window.
///
/// B-12a's first half. The core already knows how to add, delete, rename, reorder and hide a
/// layer, and `tests/` checks that it does. What could go wrong *here* is everything between a
/// row in a list and that core: a button that names a layer which is no longer there, a new
/// layer given an identifier something else is already using, a toggle that decides what a layer
/// currently is from a list drawn a minute ago, a refusal that leaves the person with a control
/// that sprang back and no sentence. None of those look like a failure at the time.
///
/// Writes `verification/B-12a_editing_table.md`.
#[cfg(test)]
mod editing {
    use super::*;

    struct Report {
        rows: Vec<(String, String, String)>,
    }

    impl Report {
        fn check(&mut self, check: &str, expected: impl ToString, actual: impl ToString) {
            self.rows
                .push((check.to_string(), expected.to_string(), actual.to_string()));
        }
    }

    fn repo(rel: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("the app crate has a parent directory")
            .join(rel)
    }

    /// One command, by the document 24 identifier the page would send.
    ///
    /// `expect` rather than a fallback: `None` means the identifier is not one this window
    /// answers, which is a mistake in the test and not a result worth tabulating.
    fn run(viewer: &Mutex<Viewer>, what: &str) -> String {
        let (id, query) = match what.split_once('?') {
            Some((id, query)) => (id, Some(query)),
            None => (what, None),
        };
        edit_command(viewer, id, query).expect("a command document 24 lists")
    }

    fn held(viewer: &Mutex<Viewer>) -> std::sync::MutexGuard<'_, Viewer> {
        viewer.lock().expect("the viewer lock was poisoned")
    }

    /// The layers of the composition on screen, front last, as names.
    fn names(viewer: &Mutex<Viewer>) -> String {
        let held = held(viewer);
        let comp = held
            .document
            .project()
            .composition(&held.composition)
            .expect("the composition on screen");
        comp.layers_in_order()
            .map(|l| l.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn layer_ids(viewer: &Mutex<Viewer>) -> String {
        let held = held(viewer);
        let comp = held
            .document
            .project()
            .composition(&held.composition)
            .expect("the composition on screen");
        comp.layer_order()
            .iter()
            .map(|id| id.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn layer<T>(viewer: &Mutex<Viewer>, id: &str, read: impl FnOnce(&Layer) -> T) -> Option<T> {
        let held = held(viewer);
        let comp = held
            .document
            .project()
            .composition(&held.composition)
            .expect("the composition on screen");
        comp.layer(&Id::new(id)).map(read)
    }

    #[test]
    fn the_panels_change_the_document_and_only_through_commands() {
        let mut report = Report { rows: Vec::new() };

        // The fixture names an effect no version of this build has. Every check below runs on a
        // project this build only partly understands, on purpose: a panel is a second reader of
        // the project, and a second reader is a second chance to drop what it cannot model.
        let source = repo("Fixtures/projects/unknown_effect_project.json");
        let viewer = Mutex::new(
            open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
        );

        // ---- what the page is given ----------------------------------------------------------
        let answer: serde_json::Value =
            serde_json::from_str(&state(&viewer)).expect("the state answer is JSON");
        {
            let held = held(&viewer);
            let written = persist::to_json(held.document.project(), &held.preserved);
            let same: serde_json::Value =
                serde_json::from_str(&written).expect("what a save would write is JSON");
            // Not "the panels got a project" but "the panels got *this* project". If the two
            // ever differ, the window is drawing one thing and saving another, and the person
            // finds out when they reopen the file.
            report.check(
                "the project the panels draw is the project a save would write",
                true,
                answer["project"] == same,
            );
        }
        report.check(
            "an effect this build does not have reaches the panels rather than vanishing",
            true,
            state(&viewer).contains("vendor.future.effect"),
        );
        report.check(
            "the panels are told which composition is on screen",
            "comp-main",
            answer["composition"].as_str().unwrap_or("(none)"),
        );
        report.check(
            "a project just opened has nothing to undo",
            "0 to undo, 0 to redo",
            format!(
                "{} to undo, {} to redo",
                answer["undo"].as_array().map_or(0, |a| a.len()),
                answer["redo"].as_array().map_or(0, |a| a.len())
            ),
        );

        // ---- adding a layer ------------------------------------------------------------------
        report.check(
            "adding a layer without saying which drawing is refused, and asks",
            "Which drawing should the new layer show?",
            run(&viewer, "layer.create"),
        );
        report.check(
            "a drawing that is not in the project is refused by name",
            "There is no imported drawing called asset-nothing.",
            run(&viewer, "layer.create?asset=asset-nothing"),
        );
        report.check(
            "neither refusal put anything in the undo history",
            0,
            held(&viewer).document.undo_depth(),
        );

        run(&viewer, "layer.create?asset=asset-cel&name=Shadow");
        report.check(
            "a new layer is added in front of the ones already there",
            "Cel, Shadow",
            names(&viewer),
        );
        report.check(
            "its identifier is not one the project was already using",
            "layer-cel, layer-1",
            layer_ids(&viewer),
        );
        report.check(
            "it covers the whole composition, which is five frames from zero",
            "0 to 5",
            layer(&viewer, "layer-1", |l| {
                format!("{} to {}", l.in_frame, l.out_frame)
            })
            .unwrap_or_default(),
        );
        run(&viewer, "layer.create?asset=asset-cel&name=Highlight");
        report.check(
            "a second new layer gets an identifier of its own",
            "layer-cel, layer-1, layer-2",
            layer_ids(&viewer),
        );

        // ---- naming --------------------------------------------------------------------------
        report.check(
            "renaming with no name is refused",
            "A layer needs a name.",
            run(&viewer, "layer.rename?layer=layer-1"),
        );
        run(&viewer, "layer.rename?layer=layer-1&name=Cast shadow");
        report.check(
            "renaming changes the name and nothing else",
            "Cel, Cast shadow, Highlight",
            names(&viewer),
        );
        run(&viewer, "layer.rename?layer=layer-1&name=%E5%BD%B1");
        report.check(
            "a name in Japanese survives the trip through the query string",
            "\u{5f71}",
            layer(&viewer, "layer-1", |l| l.name.clone()).unwrap_or_default(),
        );

        // ---- hiding and locking ----------------------------------------------------------------
        // Document 24 calls both of these a toggle, so what they do depends on what the layer is
        // now, not on what the page last drew. Each is pressed twice and must come back.
        run(&viewer, "layer.toggle_visibility?layer=layer-1");
        report.check(
            "hiding a visible layer switches it off",
            false,
            layer(&viewer, "layer-1", |l| l.enabled).unwrap_or(true),
        );
        run(&viewer, "layer.toggle_visibility?layer=layer-1");
        report.check(
            "and pressing it again switches it back on",
            true,
            layer(&viewer, "layer-1", |l| l.enabled).unwrap_or(false),
        );
        run(&viewer, "layer.toggle_lock?layer=layer-1");
        report.check(
            "locking a layer locks it",
            true,
            layer(&viewer, "layer-1", |l| l.locked).unwrap_or(false),
        );
        run(&viewer, "layer.toggle_lock?layer=layer-1");
        report.check(
            "and pressing it again unlocks it",
            false,
            layer(&viewer, "layer-1", |l| l.locked).unwrap_or(true),
        );

        // ---- order ------------------------------------------------------------------------------
        run(&viewer, "layer.move_up?layer=layer-1");
        report.check(
            "moving a layer forward puts it one place nearer the front",
            "Cel, Highlight, \u{5f71}",
            names(&viewer),
        );
        run(&viewer, "layer.move_down?layer=layer-1");
        report.check(
            "moving it back puts it where it was",
            "Cel, \u{5f71}, Highlight",
            names(&viewer),
        );
        report.check(
            "the front layer cannot go further forward, and is told so by name",
            "Highlight is already at the front.",
            run(&viewer, "layer.move_up?layer=layer-2"),
        );
        report.check(
            "the back layer cannot go further back, and is told so by name",
            "Cel is already at the back.",
            run(&viewer, "layer.move_down?layer=layer-cel"),
        );
        report.check(
            "neither refusal put an entry in the history that would undo nothing",
            "Cel, \u{5f71}, Highlight",
            names(&viewer),
        );

        // ---- a list older than the document ------------------------------------------------------
        report.check(
            "a command naming a layer that is not there is refused by name",
            "layer-gone is not a layer in this composition.",
            run(&viewer, "layer.delete?layer=layer-gone"),
        );
        report.check(
            "a command naming no layer at all asks which one",
            "Which layer? Choose one in the layer list.",
            run(&viewer, "layer.delete"),
        );

        // ---- undo ---------------------------------------------------------------------------------
        let before_delete = names(&viewer);
        let depth = held(&viewer).document.undo_depth();
        run(&viewer, "layer.delete?layer=layer-1");
        report.check(
            "deleting a layer removes it",
            "Cel, Highlight",
            names(&viewer),
        );
        report.check(
            "and the two refusals before it left the history exactly as deep as it was",
            depth + 1,
            held(&viewer).document.undo_depth(),
        );
        let answer: serde_json::Value =
            serde_json::from_str(&state(&viewer)).expect("the state answer is JSON");
        report.check(
            "the Undo button is told what it would take back",
            "Delete layer layer-1",
            answer["undo"]
                .as_array()
                .and_then(|a| a.last())
                .and_then(|v| v.as_str())
                .unwrap_or("(nothing)"),
        );
        report.check(
            "undoing says what it took back",
            "Undone: Delete layer layer-1",
            undo(&viewer),
        );
        report.check(
            "and the layer is back where it was, with the name it had",
            before_delete,
            names(&viewer),
        );
        report.check(
            "redoing says what it put back",
            "Redone: Delete layer layer-1",
            redo(&viewer),
        );
        report.check(
            "and the layer is gone again",
            "Cel, Highlight",
            names(&viewer),
        );

        // ---- dirty and clean -----------------------------------------------------------------------
        report.check(
            "a document that has been edited is unsaved work",
            true,
            held(&viewer).document.is_dirty(),
        );
        // Document 26: dirty is a comparison against the last save, not a flag, so undoing all
        // the way back to the file makes it clean again. Nothing in the window decides this.
        while held(&viewer).document.undo_depth() > 0 {
            undo(&viewer);
        }
        report.check(
            "undoing back to the file makes it not unsaved work again",
            false,
            held(&viewer).document.is_dirty(),
        );
        report.check(
            "and the composition is the one that was opened",
            "Cel",
            names(&viewer),
        );
        report.check(
            "there is nothing left to undo, and it says so",
            "There is nothing to undo.",
            undo(&viewer),
        );

        // ---- and out to the disk ---------------------------------------------------------------------
        // The last question, and the one the others are for: after all of that, is what a save
        // would write still the file that was opened? The effect this build cannot model is in
        // that comparison, so this is also the check that no panel dropped it along the way.
        {
            let held = held(&viewer);
            let opened = std::fs::read_to_string(&source)
                .expect("read the fixture")
                .replace("\r\n", "\n");
            let now = persist::to_json(held.document.project(), &held.preserved);
            // Both texts are thousands of characters and a table is for reading, so the row
            // carries the answer rather than the two files.
            let same = "identical, including the effect this build cannot model";
            report.check(
                "after all of the above, a save would write the file that was opened",
                same,
                if opened == now {
                    same
                } else {
                    "what a save would write is no longer what was opened"
                },
            );
        }

        write_artifact(
            &report,
            "verification/B-12a_editing_table.md",
            "B-12a: what the layer panel does",
            LAYER_INTRO,
            LAYER_NOTES,
        );
        let failed: Vec<&String> = report
            .rows
            .iter()
            .filter(|(_, e, a)| e != a)
            .map(|(c, _, _)| c)
            .collect();
        assert!(failed.is_empty(), "these checks failed: {failed:#?}");
    }

    /// One table with its own prose around it.
    ///
    /// Two tests in this module write one each, and everything they have in common - the header
    /// row, the escaping, the count at the bottom - is written here rather than twice.
    fn write_artifact(report: &Report, file: &str, title: &str, intro: &[&str], notes: &[&str]) {
        let passed = report.rows.iter().filter(|(_, e, a)| e == a).count();
        let mut out = format!("# {title}\n\n");
        out.push_str(
            "Generated by `app/src/main.rs`, module `editing`. Re-run with `cargo test \
             --workspace`.\n\n",
        );
        for paragraph in intro.iter().chain(notes) {
            out.push_str(paragraph);
            out.push_str("\n\n");
        }
        out.push_str("| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
        for (check, expected, actual) in &report.rows {
            out.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                check,
                cell(expected),
                cell(actual),
                if expected == actual { "pass" } else { "FAIL" }
            ));
        }
        out.push_str(&format!(
            "\n**{} of {} checks pass.**\n",
            passed,
            report.rows.len()
        ));
        std::fs::write(repo(file), out).expect("write the artifact");
    }

    /// The prose around the layer table.
    ///
    /// Out here rather than inline so that the test above reads as the sequence of steps it is,
    /// which is the thing worth checking against document 24.
    const LAYER_INTRO: &[&str] = &[
        "This is the window's half of the layer commands in document 24. The core's half - \
             that adding, deleting, renaming and reordering a layer are correct and undoable - is \
             checked in `verification/B-05_model_table.md` and is not repeated here. What is \
             checked here is the part between a row in a list and that core, which is where a \
             window goes wrong: a button that names a layer no longer there, a new layer given an \
             identifier something else is already using, a toggle that decides what a layer \
             currently is from a list drawn a minute ago, a refusal that leaves a control \
             springing back with nothing said.",
        "The fixture is `Fixtures/projects/unknown_effect_project.json`, which names an \
             effect no version of this build has. It is used for every row on purpose: a panel is \
             a second reader of the project, and a second reader is a second chance to lose what \
             it cannot model. The last row is the one that would catch that - after every edit \
             above and every undo of them, what a save would write is compared against the file \
             that was opened, byte for byte.",
    ];

    const LAYER_NOTES: &[&str] = &[
        "## What to look at\n\n- **A refusal is a sentence, never silence.** Six rows here \
             are commands that were turned down, and each one is checked for the words the person \
             would read - which drawing, which layer, and that the front layer is already at the \
             front.\n- **A refused command changes nothing.** Document 26 requires it, and the \
             rows after each refusal check the history is no deeper and the layers are in the same \
             order.\n- **Undo names what it takes back.** \"Undone\" on its own would be true and \
             useless.\n- **Unsaved work is a comparison, not a flag.** Undoing every edit back to \
             the file makes the window say the project is saved again, because the core compares \
             the document against the last save rather than counting edits.",
        "## What this does not cover\n\nThe page. Every row here calls the same function \
             the window's URL scheme calls, with the same text the page would put in it, so what \
             is checked is everything from the request inwards. That a button is wired to the \
             right request, that the list is drawn front-first, and that the keyboard reaches all \
             of it are in the photographs beside this table, not in it.\n\nThe transform and \
             the effect stack are edited from the same inspector and are checked in \
             `verification/B-12a_transform_table.md` and `verification/B-12a_effects_table.md`. \
             The matte and the mask are shown and cannot yet be changed; that is later work and \
             is named as missing rather than quietly absent.",
    ];

    /// The transform property a layer carries, as a string, from the same JSON the panels get.
    fn base(viewer: &Mutex<Viewer>, layer_id: &str, prop: &str) -> String {
        let answer: serde_json::Value =
            serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
        answer["project"]["compositions"][0]["layers"]
            .as_array()
            .expect("a composition has layers")
            .iter()
            .find(|l| l["id"] == layer_id)
            .map(|l| l["transform"][prop]["base"].to_string())
            .unwrap_or_else(|| "(no such layer)".to_string())
    }

    /// A property's keyframes as the panels get them, one `value@frame interp` per key.
    fn keys(viewer: &Mutex<Viewer>, layer_id: &str, prop: &str) -> String {
        let answer: serde_json::Value =
            serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
        answer["project"]["compositions"][0]["layers"]
            .as_array()
            .expect("a composition has layers")
            .iter()
            .find(|l| l["id"] == layer_id)
            .and_then(|l| l["transform"][prop]["keyframes"].as_array())
            .map(|keys| {
                keys.iter()
                    .map(|k| {
                        // D-52: an eased key's four numbers go in the row. The word "ease" on
                        // its own would pass whatever curve it was given, which is the one
                        // thing a table about easing has to be able to tell apart.
                        let curve = match &k["ease"] {
                            serde_json::Value::Null => String::new(),
                            ease => format!(" {ease}"),
                        };
                        format!(
                            "{}@{} {}{curve}",
                            k["value"],
                            k["frame"],
                            k["interp"].as_str().unwrap_or("?")
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_else(|| "(no such layer)".to_string())
    }

    /// A property's value on one frame, out of the `/boxes` answer the inspector shows it from.
    /// What the graph editor is told to draw: the first component of one property, frame by
    /// frame, out of the route the page asks rather than out of the model beside it.
    fn plotted(viewer: &Mutex<Viewer>, query: &str) -> Vec<f64> {
        let answer = curve(viewer, Some(query));
        let said: serde_json::Value =
            serde_json::from_slice(answer.body()).expect("the route answers with JSON");
        said["samples"]
            .as_array()
            .expect("samples is a list")
            .iter()
            .map(|s| s[0].as_f64().expect("a sample is a number"))
            .collect()
    }

    fn value_at(viewer: &Mutex<Viewer>, frame: i32, layer_id: &str, prop: &str) -> String {
        let body = boxes(viewer, frame, None).into_body();
        let answer: serde_json::Value =
            serde_json::from_slice(&body).expect("the boxes answer is JSON");
        answer["values"][layer_id][prop].to_string()
    }

    #[test]
    fn the_inspector_changes_a_transform_and_a_drag_is_one_history_entry() {
        let mut report = Report { rows: Vec::new() };
        let source = repo("Fixtures/projects/unknown_effect_project.json");
        let viewer = Mutex::new(
            open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
        );
        let l = "layer-cel";

        // ---- typing a number ------------------------------------------------------------------
        report.check(
            "the fixture's layer starts where document 19's default puts it",
            "[0,0]",
            base(&viewer, l, "position"),
        );
        report.check(
            "typing a position says what it set, in the words undo will use",
            "Set position to (120, -40)",
            run(
                &viewer,
                "property.set_base?layer=layer-cel&prop=position&value=120,-40",
            ),
        );
        report.check(
            "and the panels are given the new value back",
            "[120,-40]",
            base(&viewer, l, "position"),
        );
        report.check("a scalar property takes one number", "45", {
            run(
                &viewer,
                "property.set_base?layer=layer-cel&prop=rotation&value=45",
            );
            base(&viewer, l, "rotation")
        });
        // Document 19 makes opacity nought to one and the core clamps rather than refuses, so a
        // person who typed 5 gets the fully opaque layer they were reaching for.
        report.check("opacity above one is clamped rather than refused", "1", {
            run(
                &viewer,
                "property.set_base?layer=layer-cel&prop=opacity&value=5",
            );
            base(&viewer, l, "opacity")
        });

        // ---- what the panel refuses before the core sees it -------------------------------------
        let depth = held(&viewer).document.undo_depth();
        report.check(
            "a property name that is not one of the five is refused, and the five are named",
            "Which property? Say anchor, position, scale, rotation or opacity.",
            run(
                &viewer,
                "property.set_base?layer=layer-cel&prop=wobble&value=1",
            ),
        );
        report.check(
            "no value at all is refused, and asks",
            "What should scale be set to?",
            run(&viewer, "property.set_base?layer=layer-cel&prop=scale"),
        );
        report.check(
            "text that is not numbers is refused in the shape the property wants",
            "position needs two numbers, like 12, -4. Not \"over there\".",
            run(
                &viewer,
                "property.set_base?layer=layer-cel&prop=position&value=over%20there",
            ),
        );
        report.check(
            "one number for a two-number property is refused the same way",
            "position needs two numbers, like 12, -4. Not \"7\".",
            run(
                &viewer,
                "property.set_base?layer=layer-cel&prop=position&value=7",
            ),
        );
        report.check(
            "two numbers for a one-number property never reach the core, and are named here",
            "rotation needs a number. Not \"1,2\".",
            run(
                &viewer,
                "property.set_base?layer=layer-cel&prop=rotation&value=1,2",
            ),
        );
        report.check(
            // The percent typed in is divided into the factor the model holds before the core
            // sees it, which is why the second number comes back as 0.01 rather than the 1 that
            // was typed. The refusal names the model's value, not the field's; that the two
            // spellings differ is D-22's, and `verification/B-12b_w01_walkthrough.md` writes up
            // what it costs a person reading the status line.
            "a value that is not a finite number is refused",
            "scale cannot be set to (NaN, 0.01).",
            run(
                &viewer,
                "property.set_base?layer=layer-cel&prop=scale&value=NaN,1",
            ),
        );
        report.check(
            "none of those six refusals put anything in the history",
            depth,
            held(&viewer).document.undo_depth(),
        );
        report.check(
            "and the position is the one that was typed",
            "[120,-40]",
            base(&viewer, l, "position"),
        );

        // ---- a drag ------------------------------------------------------------------------------
        // Document 26: a drag previews without creating hundreds of history entries and commits
        // one command at release. This is the check that the window opens the transaction at all -
        // a page that forgot to would look identical until somebody pressed Ctrl+Z.
        let depth = held(&viewer).document.undo_depth();
        for x in [130, 150, 180, 210] {
            run(
                &viewer,
                &format!("property.drag_update?layer=layer-cel&prop=position&value={x},-40"),
            );
        }
        report.check(
            "the value follows the drag while it is being dragged",
            "[210,-40]",
            base(&viewer, l, "position"),
        );
        report.check(
            "and four intermediate values are not four history entries",
            depth,
            held(&viewer).document.undo_depth(),
        );
        report.check(
            "releasing commits one entry, named for what it set",
            "Set position to (210, -40)",
            run(&viewer, "property.drag_end"),
        );
        report.check(
            "which is one entry, not four",
            depth + 1,
            held(&viewer).document.undo_depth(),
        );
        report.check(
            "and undoing it goes back to before the drag, not to one step inside it",
            "[120,-40]",
            {
                undo(&viewer);
                base(&viewer, l, "position")
            },
        );
        redo(&viewer);

        // ---- Escape during a drag ------------------------------------------------------------------
        let depth = held(&viewer).document.undo_depth();
        for x in [220, 260, 300] {
            run(
                &viewer,
                &format!("property.drag_update?layer=layer-cel&prop=position&value={x},-40"),
            );
        }
        report.check(
            "Escape during a drag says the value has gone back",
            "Cancelled; the value is back where the drag started.",
            run(&viewer, "property.drag_cancel"),
        );
        report.check(
            "and it has - document 24: Escape restores the pre-drag value",
            "[210,-40]",
            base(&viewer, l, "position"),
        );
        report.check(
            "a cancelled drag leaves no history entry",
            depth,
            held(&viewer).document.undo_depth(),
        );

        // ---- the two ends on their own ---------------------------------------------------------------
        report.check(
            "releasing when nothing is being dragged is said, not silently ignored",
            "Nothing moved.",
            run(&viewer, "property.drag_end"),
        );
        report.check(
            "and so is Escape when nothing is being dragged",
            "Nothing is being dragged.",
            run(&viewer, "property.drag_cancel"),
        );
        // A drag that ends where it started is somebody who thought better of it. Document 26
        // says that is not an edit, and an undo item that undoes nothing is worse than none.
        let depth = held(&viewer).document.undo_depth();
        run(
            &viewer,
            "property.drag_update?layer=layer-cel&prop=position&value=260,-40",
        );
        run(
            &viewer,
            "property.drag_update?layer=layer-cel&prop=position&value=210,-40",
        );
        report.check(
            "a drag that ends where it started is not an edit",
            "Nothing moved.",
            run(&viewer, "property.drag_end"),
        );
        report.check(
            "and adds no history entry",
            depth,
            held(&viewer).document.undo_depth(),
        );

        // ---- a locked layer ---------------------------------------------------------------------------
        run(&viewer, "layer.toggle_lock?layer=layer-cel");
        report.check(
            "a locked layer refuses a transform edit, and says which rule stopped it",
            "The layer \"Cel\" is locked, so it was not changed. Unlock the layer to edit it.",
            run(
                &viewer,
                "property.set_base?layer=layer-cel&prop=position&value=0,0",
            ),
        );
        report.check(
            "and the value is untouched",
            "[210,-40]",
            base(&viewer, l, "position"),
        );
        run(&viewer, "layer.toggle_lock?layer=layer-cel");

        // ---- keyframes (W-10) ---------------------------------------------------------------------------
        // Document 24's toggle, and what a typed or dragged value becomes once a property has
        // keys. The value between two keys is read through `/boxes`, which is the answer the
        // inspector shows it from, so the row is the number the artist would see.
        let depth = held(&viewer).document.undo_depth();
        report.check(
            "the diamond on a property with no keys sets one holding the value it has",
            "Keyframe position at frame 12 to (210, -40)",
            run(
                &viewer,
                "keyframe.add_remove?layer=layer-cel&prop=position&frame=12",
            ),
        );
        report.check(
            "and the file now holds that one key, linear as document 19 defaults",
            "[210,-40]@12 linear",
            keys(&viewer, l, "position"),
        );
        report.check(
            "a value typed at another frame becomes a second key rather than a base",
            "Keyframe position at frame 36 to (0, 0)",
            run(
                &viewer,
                "property.set_base?layer=layer-cel&prop=position&value=0,0&frame=36",
            ),
        );
        report.check(
            "so there are two keys",
            "[210,-40]@12 linear, [0,0]@36 linear",
            keys(&viewer, l, "position"),
        );
        report.check(
            "and the base is the value from before either",
            "[210,-40]",
            base(&viewer, l, "position"),
        );
        report.check(
            "halfway between the keys the window answers the halfway value",
            "[105,-20]",
            value_at(&viewer, 24, l, "position"),
        );
        report.check(
            "a value on a keyframed property with no frame named is refused",
            "position is keyframed, so a value belongs to a frame. Say frame=<frame>.",
            run(
                &viewer,
                "property.set_base?layer=layer-cel&prop=position&value=0,0",
            ),
        );
        report.check(
            "the diamond with no frame named is refused",
            "Which frame? Say frame=<frame>.",
            run(&viewer, "keyframe.add_remove?layer=layer-cel&prop=position"),
        );
        report.check(
            "the diamond on a frame that has a key removes it",
            "Remove position keyframe at frame 36",
            run(
                &viewer,
                "keyframe.add_remove?layer=layer-cel&prop=position&frame=36",
            ),
        );
        report.check(
            "leaving the first",
            "[210,-40]@12 linear",
            keys(&viewer, l, "position"),
        );
        // A corner drag on the picture sends a position and a scale for one layer at once.
        // Keyed, both must survive the drag: the transaction keeps the last of each control,
        // and a key on another property is another control.
        run(
            &viewer,
            "keyframe.add_remove?layer=layer-cel&prop=scale&frame=12",
        );
        for (x, s) in [(240, 120), (270, 160), (300, 200)] {
            run(
                &viewer,
                &format!(
                    "property.drag_update?layer=layer-cel&prop=position&value={x},-40&frame=12"
                ),
            );
            run(
                &viewer,
                &format!("property.drag_update?layer=layer-cel&prop=scale&value={s},{s}&frame=12"),
            );
        }
        report.check(
            "a drag on two keyed properties commits one entry",
            "Keyframe position at frame 12 to (300, -40) and 1 more",
            run(&viewer, "property.drag_end"),
        );
        report.check(
            "the position key took the last value dragged",
            "[300,-40]@12 linear",
            keys(&viewer, l, "position"),
        );
        report.check(
            "and the scale key beside it survived the position's drag",
            "[200,200]@12 linear",
            keys(&viewer, l, "scale"),
        );
        report.check(
            "five history entries for the five edits: two keys set, one typed, one removed, one drag",
            5,
            held(&viewer).document.undo_depth() - depth,
        );

        // ---- moving a key (W-11) ------------------------------------------------------------
        // The mark on a property's row, taken hold of and put down on another frame. One command
        // rather than a remove and a set, so it is one entry to undo and the key cannot be lost
        // between the two halves of it; the value and the interpolation mode go with it.
        let depth = held(&viewer).document.undo_depth();
        report.check(
            "a key put down on another frame moves, and says which frame it came from",
            "Move position keyframe from frame 12 to frame 20",
            run(
                &viewer,
                "keyframe.move?layer=layer-cel&prop=position&from=12&to=20",
            ),
        );
        report.check(
            "the key is on the new frame with the value and the interpolation it had",
            "[300,-40]@20 linear",
            keys(&viewer, l, "position"),
        );
        report.check(
            "and the key on another property stayed where it was",
            "[200,200]@12 linear",
            keys(&viewer, l, "scale"),
        );
        run(
            &viewer,
            "keyframe.add_remove?layer=layer-cel&prop=position&frame=30",
        );
        report.check(
            "a key put down where that property already has one is refused, not allowed to \
             overwrite it",
            "There is already a position keyframe on frame 30.",
            run(
                &viewer,
                "keyframe.move?layer=layer-cel&prop=position&from=20&to=30",
            ),
        );
        report.check(
            "and both keys are still there",
            "[300,-40]@20 linear, [300,-40]@30 linear",
            keys(&viewer, l, "position"),
        );
        report.check(
            "a key that is not on the frame named cannot be moved",
            "There is no position keyframe at frame 99 to move. The edit was not applied. \
             Nothing in the project changed.",
            run(
                &viewer,
                "keyframe.move?layer=layer-cel&prop=position&from=99&to=40",
            ),
        );
        report.check(
            "a move onto the frame the key is already on is not an edit",
            "That keyframe is already on that frame.",
            run(
                &viewer,
                "keyframe.move?layer=layer-cel&prop=position&from=20&to=20",
            ),
        );
        report.check(
            "two history entries for the two edits that were allowed, and none for the three \
             that were refused",
            2,
            held(&viewer).document.undo_depth() - depth,
        );
        undo(&viewer);
        undo(&viewer);
        report.check(
            "undoing puts the key back on the frame it was moved from",
            "[300,-40]@12 linear",
            keys(&viewer, l, "position"),
        );

        // ---- easing a segment (D-52) --------------------------------------------------------
        // The preset beside the diamond. Document 20 says a keyframe's mode belongs to the
        // segment that starts at it, so what these rows check is the value *between* two keys
        // moving while the keys themselves do not - which is what an ease is and the only way to
        // see one without drawing a graph.
        let depth = held(&viewer).document.undo_depth();
        run(
            &viewer,
            "keyframe.add_remove?layer=layer-cel&prop=rotation&frame=0",
        );
        run(
            &viewer,
            "property.set_base?layer=layer-cel&prop=rotation&value=120&frame=24",
        );
        report.check(
            "two linear rotation keys, and the halfway frame is halfway between them",
            "82.5",
            value_at(&viewer, 12, l, "rotation"),
        );
        report.check(
            "pressing the curve says which segment it changed",
            "Keyframe rotation at frame 0 to 45",
            run(
                &viewer,
                "keyframe.set_interp?layer=layer-cel&prop=rotation&frame=0&mode=ease",
            ),
        );
        report.check(
            "the key carries the four numbers of the curve, not just the word",
            "45@0 ease [0.3333333333333333,0,0.6666666666666666,1], 120@24 linear",
            keys(&viewer, l, "rotation"),
        );
        report.check(
            "the halfway frame is still halfway, because easy ease is symmetric",
            "82.5",
            value_at(&viewer, 12, l, "rotation"),
        );
        report.check(
            "a quarter of the way along it is behind where linear would have it: 63.75 becomes",
            "56.71875",
            value_at(&viewer, 6, l, "rotation"),
        );
        report.check(
            "and three quarters along it is ahead: 101.25 becomes",
            "108.28125",
            value_at(&viewer, 18, l, "rotation"),
        );
        report.check(
            "neither keyframe moved",
            "45",
            value_at(&viewer, 0, l, "rotation"),
        );
        report.check(
            "nor the one at the end",
            "120",
            value_at(&viewer, 24, l, "rotation"),
        );
        report.check(
            "pressing it again puts the segment back to linear",
            "Keyframe rotation at frame 0 to 45",
            run(
                &viewer,
                "keyframe.set_interp?layer=layer-cel&prop=rotation&frame=0&mode=linear",
            ),
        );
        report.check(
            "and the four numbers are gone from the file rather than left behind",
            "45@0 linear, 120@24 linear",
            keys(&viewer, l, "rotation"),
        );
        report.check(
            "easing a frame that has no key is refused, and says what to do first",
            "rotation has no keyframe at frame 7, so there is no segment to ease. Add a \
             keyframe first.",
            run(
                &viewer,
                "keyframe.set_interp?layer=layer-cel&prop=rotation&frame=7&mode=ease",
            ),
        );
        report.check(
            "a curve this window does not offer is refused rather than guessed at",
            "An interpolation is hold, linear or ease. Not \"bouncy\".",
            run(
                &viewer,
                "keyframe.set_interp?layer=layer-cel&prop=rotation&frame=0&mode=bouncy",
            ),
        );
        report.check(
            "four history entries for the four edits, and none for the two refusals",
            4,
            held(&viewer).document.undo_depth() - depth,
        );
        undo(&viewer);
        report.check(
            "undoing the straightening gives the curve back",
            "45@0 ease [0.3333333333333333,0,0.6666666666666666,1], 120@24 linear",
            keys(&viewer, l, "rotation"),
        );

        // ---- the graph editor (D-52) --------------------------------------------------------
        // The panel that shares the timeline's place draws a line and two handles, and the line
        // is the one thing about it that could quietly be wrong: a graph that solved the curve
        // itself would go on looking like a perfectly good graph on the day it stopped agreeing
        // with the picture. So the page asks the window, and these rows are what it is told.
        let drawn = plotted(&viewer, "layer=layer-cel&prop=rotation&from=0&to=24");
        report.check(
            "the graph is given one sample per frame of the range",
            25,
            drawn.len(),
        );
        report.check(
            "and every one of them is the value the renderer would draw on that frame",
            "all 25 agree",
            match (0..=24).all(|f| {
                let shown: f64 = value_at(&viewer, f, l, "rotation")
                    .parse()
                    .expect("a rotation is a number");
                (shown - drawn[f as usize]).abs() < 1e-12
            }) {
                true => "all 25 agree",
                false => "at least one disagrees",
            },
        );
        report.check(
            "so the eased middle of the segment is drawn where the ease puts it",
            "82.5",
            drawn[12],
        );
        report.check(
            "and a quarter along, behind where a straight line would have it",
            "56.71875",
            drawn[6],
        );
        report.check(
            "pulling the first handle to the far end of the segment is one edit",
            "Keyframe rotation at frame 0 to 45",
            run(
                &viewer,
                "keyframe.set_interp?layer=layer-cel&prop=rotation&frame=0&mode=ease\
                 &curve=0.75,0,1,0.25",
            ),
        );
        report.check(
            "and the four numbers it was dropped at are the ones in the file",
            "45@0 ease [0.75,0,1,0.25], 120@24 linear",
            keys(&viewer, l, "rotation"),
        );
        report.check(
            "and the middle of the segment is held right back: 82.5 becomes",
            "49.3993",
            format!(
                "{:.4}",
                plotted(&viewer, "layer=layer-cel&prop=rotation&from=0&to=24")[12]
            ),
        );
        report.check(
            "a handle dragged outside its own segment is refused rather than clamped",
            "A curve handle cannot reach outside its own segment, so x1 and x2 are between 0 \
             and 1. Not 1.2 and 0.1.",
            run(
                &viewer,
                "keyframe.set_interp?layer=layer-cel&prop=rotation&frame=0&mode=ease\
                 &curve=1.2,0,0.1,1",
            ),
        );
        report.check(
            "and so is a curve that is not four numbers",
            "A curve is four numbers, x1,y1,x2,y2. Not \"0.4,0.1\".",
            run(
                &viewer,
                "keyframe.set_interp?layer=layer-cel&prop=rotation&frame=0&mode=ease\
                 &curve=0.4,0.1",
            ),
        );
        report.check(
            "asking about a layer that has gone draws nothing rather than failing",
            0,
            plotted(&viewer, "layer=gone&prop=rotation&from=0&to=24").len(),
        );
        undo(&viewer);

        // ---- back to the file ---------------------------------------------------------------------------
        while held(&viewer).document.undo_depth() > 0 {
            undo(&viewer);
        }
        let held = held(&viewer);
        let opened = std::fs::read_to_string(&source)
            .expect("read the fixture")
            .replace("\r\n", "\n");
        let now = persist::to_json(held.document.project(), &held.preserved);
        let same = "identical, including the effect this build cannot model";
        report.check(
            "undoing every transform edit gives back the file that was opened",
            same,
            if opened == now {
                same
            } else {
                "what a save would write is no longer what was opened"
            },
        );
        drop(held);

        write_artifact(
            &report,
            "verification/B-12a_transform_table.md",
            "B-12a: what the transform inspector does",
            TRANSFORM_INTRO,
            TRANSFORM_NOTES,
        );
        let failed: Vec<&String> = report
            .rows
            .iter()
            .filter(|(_, e, a)| e != a)
            .map(|(c, _, _)| c)
            .collect();
        assert!(failed.is_empty(), "these checks failed: {failed:#?}");
    }

    const TRANSFORM_INTRO: &[&str] = &[
        "Document 19's five transform properties, edited the two ways an inspector offers: \
         typing a number into a field, and dragging the handle beside it. Both send the same \
         command ID. What differs is that typing commits an entry of its own and a drag commits \
         one entry for the whole gesture, which is document 26's interaction transaction.",
        "The core's half - that setting a property is correct, undoable, and clamped or refused \
         by document 19's rules - is checked in `verification/B-05_model_table.md`. What is \
         checked here is the part the window owns: that the text a field sends becomes the right \
         kind of value, that a drag opens a transaction at all, and that every refusal is a \
         sentence naming the rule.",
    ];

    const TRANSFORM_NOTES: &[&str] = &[
        "## What to look at\n\n- **Four drag steps are one history entry.** The rows check the \
         value follows the drag, that the history does not deepen while it is following, and \
         that one entry appears at release. A window that forgot to open the transaction would \
         look identical on screen and would put four entries in the history, and nobody would \
         find out until they pressed Ctrl+Z.\n- **A drag that ends where it started is not an \
         edit.** It is somebody who thought better of it, and an undo item that undoes nothing \
         is worse than no undo item.\n- **Escape puts the value back.** Document 24 requires it \
         and the row after it checks the value, not just the sentence.\n- **A value is read in \
         the shape the property wants.** Sending one number for position is refused rather than \
         read as a scalar the core would then reject for its kind, which means a wrong-kind \
         value cannot reach the model at all. The row that would have shown the core's own \
         refusal instead shows the window's, and that is the correct outcome.",
        "## What this does not cover\n\nThe gesture. Every row here sends the requests a scrub \
         sends, in the order it sends them, but that a pointer dragged across the handle produces \
         those requests - and that the field stops being rebuilt underneath it while it does - is \
         in the photographs beside this table.\n\nBlend mode is shown in the inspector and cannot \
         be changed from it. There is no command in the core for changing one, W-01 does not ask \
         to change one, and adding a command to the model to fill a gap in a panel is a decision \
         about the project format rather than about this window.\n\nKeyframes, since W-10. The \
         diamond beside a property is document 24's `keyframe.add_remove`, and a value typed or \
         dragged on a keyframed property becomes a key at the frame under the playhead rather \
         than a base nothing is drawn from. The rows under \"keyframes\" are that, and the \
         interpolated value between two keys is read back through the same `/boxes` answer the \
         inspector shows it from.\n\nMoving a key along the bar is `keyframe.move`, built by \
         W-11 on 2026-09-12: one command rather than a remove and a set, so a key put on the \
         wrong frame costs one entry to undo and cannot be lost between the two halves of the \
         gesture. The rows under \"moving a key\" are that, including the two refusals - a frame \
         that already carries a key of the same property, and a frame that carries none to move. \
         What the rows cannot cover is the gesture itself: that the mark follows the pointer and \
         that nothing is sent until it is let go is in the photographs beside this table.",
    ];

    /// The effect records of one layer, out of the same JSON the panels are drawn from.
    fn effect_records(viewer: &Mutex<Viewer>, layer_id: &str) -> Vec<serde_json::Value> {
        let answer: serde_json::Value =
            serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
        answer["project"]["compositions"][0]["layers"]
            .as_array()
            .expect("a composition has layers")
            .iter()
            .find(|l| l["id"] == layer_id)
            .and_then(|l| l["effects"].as_array().cloned())
            .unwrap_or_default()
    }

    /// The stack in the order document 21 evaluates it: what each effect is and whether it runs.
    fn stack(viewer: &Mutex<Viewer>, layer_id: &str) -> String {
        effect_records(viewer, layer_id)
            .iter()
            .map(|e| {
                format!(
                    "{} {} {}",
                    e["instance_id"].as_str().unwrap_or("?"),
                    e["type_id"].as_str().unwrap_or("?"),
                    if e["enabled"] == serde_json::Value::Bool(true) {
                        "on"
                    } else {
                        "bypassed"
                    }
                )
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// One effect's settings, as they would be written to the file.
    fn settings(viewer: &Mutex<Viewer>, layer_id: &str, instance: &str) -> String {
        effect_records(viewer, layer_id)
            .iter()
            .find(|e| e["instance_id"] == instance)
            .map(|e| e["parameters"].to_string())
            .unwrap_or_else(|| "(no such effect)".to_string())
    }

    #[test]
    fn the_effects_panel_builds_a_stack_and_keeps_the_one_it_cannot_draw() {
        let mut report = Report { rows: Vec::new() };
        let source = repo("Fixtures/projects/unknown_effect_project.json");
        let viewer = Mutex::new(
            open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
        );
        let l = "layer-cel";

        // ---- what was in the file ---------------------------------------------------------------
        report.check(
            "the layer arrives with the effect no version of this build has",
            "fx-unknown-1 vendor.future.effect on",
            stack(&viewer, l),
        );

        // ---- adding ----------------------------------------------------------------------------
        report.check(
            "adding a blur says what was added, in the words undo will use",
            "Add core.gaussian_blur",
            run(
                &viewer,
                "effect.add?layer=layer-cel&type=core.gaussian_blur",
            ),
        );
        report.check(
            "and it goes on the end of the stack, which is where it is evaluated last",
            "fx-unknown-1 vendor.future.effect on, fx-1 core.gaussian_blur on",
            stack(&viewer, l),
        );
        report.check(
            "a new effect starts at the setting that changes no pixels",
            r#"{"sigma_px":0}"#,
            settings(&viewer, l, "fx-1"),
        );

        // ---- settings ---------------------------------------------------------------------------
        report.check(
            "setting the radius says which effect's settings changed",
            "Change core.gaussian_blur settings",
            run(
                &viewer,
                "effect.set_parameters?layer=layer-cel&effect=fx-1&sigma_px=4.5",
            ),
        );
        report.check(
            "and the panels are given the new radius back",
            r#"{"sigma_px":4.5}"#,
            settings(&viewer, l, "fx-1"),
        );
        let depth = held(&viewer).document.undo_depth();
        report.check(
            "a negative radius is refused in document 21's own words, not clamped",
            "A Gaussian blur needs a sigma of zero or more, and this is -1. Choose a value \
             inside the range.",
            run(
                &viewer,
                "effect.set_parameters?layer=layer-cel&effect=fx-1&sigma_px=-1",
            ),
        );
        report.check(
            "text that is not a number is refused by the window, before the core sees it",
            "sigma_px needs a number. Not \"wide\".",
            run(
                &viewer,
                "effect.set_parameters?layer=layer-cel&effect=fx-1&sigma_px=wide",
            ),
        );
        report.check(
            "a setting left out is refused rather than filled in from a default",
            "What should sigma_px be set to?",
            run(&viewer, "effect.set_parameters?layer=layer-cel&effect=fx-1"),
        );
        report.check(
            "none of those three refusals changed the radius",
            r#"{"sigma_px":4.5}"#,
            settings(&viewer, l, "fx-1"),
        );
        report.check(
            "and none of them put anything in the history",
            depth,
            held(&viewer).document.undo_depth(),
        );

        // ---- an effect with more than one setting -------------------------------------------------
        run(&viewer, "effect.add?layer=layer-cel&type=core.tint");
        report.check(
            "a tint takes a colour and an amount together",
            r#"{"amount":0.25,"color":[1,0,0]}"#,
            {
                run(
                    &viewer,
                    "effect.set_parameters?layer=layer-cel&effect=fx-2&color=1,0,0&amount=0.25",
                );
                settings(&viewer, l, "fx-2")
            },
        );
        report.check(
            "an amount outside document 21's range is refused",
            "A tint amount runs from 0 to 1, and this is 1.5. Choose a value inside the range.",
            run(
                &viewer,
                "effect.set_parameters?layer=layer-cel&effect=fx-2&color=1,0,0&amount=1.5",
            ),
        );
        report.check(
            "a colour that is not three numbers is refused by the window",
            "color needs three numbers, like 1, 0.5, 0. Not \"1,0\".",
            run(
                &viewer,
                "effect.set_parameters?layer=layer-cel&effect=fx-2&color=1,0&amount=0.25",
            ),
        );
        report.check(
            "and the tint is as it was set",
            r#"{"amount":0.25,"color":[1,0,0]}"#,
            settings(&viewer, l, "fx-2"),
        );

        // ---- bypass ------------------------------------------------------------------------------
        // Document 24 calls this a bypass rather than a delete: the effect stays in the file with
        // its settings, and the picture is drawn without it.
        report.check(
            "bypassing an effect says so",
            "Bypass effect fx-1",
            run(&viewer, "effect.toggle_bypass?layer=layer-cel&effect=fx-1"),
        );
        report.check(
            "and the stack says which one is not running",
            "fx-unknown-1 vendor.future.effect on, fx-1 core.gaussian_blur bypassed, \
             fx-2 core.tint on",
            stack(&viewer, l),
        );
        report.check(
            "a bypassed effect keeps its settings",
            r#"{"sigma_px":4.5}"#,
            settings(&viewer, l, "fx-1"),
        );
        report.check(
            "switching it back on says that instead",
            "Switch effect fx-1 on",
            run(&viewer, "effect.toggle_bypass?layer=layer-cel&effect=fx-1"),
        );

        // ---- the effect this build does not have ---------------------------------------------------
        // Document 19: an unknown effect record must survive load and save and may not be
        // silently discarded. It can be switched off and it can be removed, because both are
        // things a person decided; what cannot happen is this window writing settings for a
        // schema it has never seen.
        report.check(
            "the effect from another version can be bypassed like any other",
            "Bypass effect fx-unknown-1",
            run(
                &viewer,
                "effect.toggle_bypass?layer=layer-cel&effect=fx-unknown-1",
            ),
        );
        run(
            &viewer,
            "effect.toggle_bypass?layer=layer-cel&effect=fx-unknown-1",
        );
        report.check(
            "but its settings cannot be changed from here, and it says why",
            "vendor.future.effect is not an effect this build has, so its settings are kept as \
             the file wrote them and cannot be changed here.",
            run(
                &viewer,
                "effect.set_parameters?layer=layer-cel&effect=fx-unknown-1&strength=0.9",
            ),
        );
        report.check(
            "and its settings are the ones the file wrote",
            r#"{"strength":0.5}"#,
            settings(&viewer, l, "fx-unknown-1"),
        );

        // ---- deleting ------------------------------------------------------------------------------
        run(&viewer, "effect.add?layer=layer-cel&type=core.exposure");
        report.check(
            "deleting an effect from the middle of the stack names the one that went",
            "Remove effect fx-2",
            run(&viewer, "effect.delete?layer=layer-cel&effect=fx-2"),
        );
        report.check(
            "and the rest of the stack is untouched, in the order it was in",
            "fx-unknown-1 vendor.future.effect on, fx-1 core.gaussian_blur on, \
             fx-3 core.exposure on",
            stack(&viewer, l),
        );
        report.check(
            "the gap a deleted effect left is not filled by the next one",
            "fx-unknown-1 vendor.future.effect on, fx-1 core.gaussian_blur on, \
             fx-3 core.exposure on, fx-4 core.gaussian_blur on",
            {
                run(
                    &viewer,
                    "effect.add?layer=layer-cel&type=core.gaussian_blur",
                );
                stack(&viewer, l)
            },
        );
        // ---- the order, which is the picture ---------------------------------------------
        // ADR-017 evaluates the stack in order, so a blur then an exposure is not an exposure
        // then a blur. Until W-03 the only way to change that order was to delete an effect and
        // add it again, which loses its settings on the way.
        report.check(
            "moving an effect earlier says where it went",
            "Move effect fx-3 to position 1",
            run(&viewer, "effect.move_up?layer=layer-cel&effect=fx-3"),
        );
        report.check(
            "and the stack is in the new order, with nothing else disturbed",
            "fx-unknown-1 vendor.future.effect on, fx-3 core.exposure on, \
             fx-1 core.gaussian_blur on, fx-4 core.gaussian_blur on",
            stack(&viewer, l),
        );
        report.check(
            "moving it back later puts the stack where it was",
            "fx-unknown-1 vendor.future.effect on, fx-1 core.gaussian_blur on, \
             fx-3 core.exposure on, fx-4 core.gaussian_blur on",
            {
                run(&viewer, "effect.move_down?layer=layer-cel&effect=fx-3");
                stack(&viewer, l)
            },
        );
        // W-07: a card dragged up or down the stack lands at one position, sent once.
        report.check(
            "moving an effect straight to a position says where it went",
            "Move effect fx-4 to position 0",
            run(&viewer, "effect.move?layer=layer-cel&effect=fx-4&to=0"),
        );
        report.check(
            "and it is there, with the others closed up behind it",
            "fx-4 core.gaussian_blur on, fx-unknown-1 vendor.future.effect on, \
             fx-1 core.gaussian_blur on, fx-3 core.exposure on",
            stack(&viewer, l),
        );
        report.check(
            "a position past the end is refused in words",
            "Position 9 is past the end of a stack of 4 effects.",
            run(&viewer, "effect.move?layer=layer-cel&effect=fx-4&to=9"),
        );
        report.check(
            "and so is the position it already has, rather than written as a change",
            "fx-4 is already at position 0.",
            run(&viewer, "effect.move?layer=layer-cel&effect=fx-4&to=0"),
        );
        run(&viewer, "effect.move?layer=layer-cel&effect=fx-4&to=3");
        report.check(
            "an effect this build does not have moves like any other",
            "fx-1 core.gaussian_blur on, fx-unknown-1 vendor.future.effect on, \
             fx-3 core.exposure on, fx-4 core.gaussian_blur on",
            {
                run(
                    &viewer,
                    "effect.move_down?layer=layer-cel&effect=fx-unknown-1",
                );
                stack(&viewer, l)
            },
        );
        run(
            &viewer,
            "effect.move_up?layer=layer-cel&effect=fx-unknown-1",
        );
        run(&viewer, "effect.delete?layer=layer-cel&effect=fx-4");
        run(&viewer, "effect.delete?layer=layer-cel&effect=fx-3");

        // ---- what the window refuses before the core sees it ------------------------------------------
        let depth = held(&viewer).document.undo_depth();
        report.check(
            "an effect that is not on this layer is said, not silently ignored",
            "fx-99 is not an effect on this layer.",
            run(&viewer, "effect.delete?layer=layer-cel&effect=fx-99"),
        );
        report.check(
            "no effect named at all is asked for",
            "Which effect? Choose one in the effects list.",
            run(&viewer, "effect.toggle_bypass?layer=layer-cel"),
        );
        report.check(
            "an effect type this build does not have is refused, and the three are named",
            "This build has no effect called core.warp. It has core.gaussian_blur, \
             core.exposure and core.tint.",
            run(&viewer, "effect.add?layer=layer-cel&type=core.warp"),
        );
        report.check(
            "adding without saying which effect asks",
            "Which effect? Say core.gaussian_blur, core.exposure or core.tint.",
            run(&viewer, "effect.add?layer=layer-cel"),
        );
        report.check(
            "and a layer that is not in this composition is named",
            "layer-gone is not a layer in this composition.",
            run(&viewer, "effect.add?layer=layer-gone&type=core.exposure"),
        );
        report.check(
            "the first effect in the stack refuses to go earlier, in words",
            "fx-unknown-1 is already first.",
            run(
                &viewer,
                "effect.move_up?layer=layer-cel&effect=fx-unknown-1",
            ),
        );
        report.check(
            "and the last one refuses to go later",
            "fx-1 is already last.",
            run(&viewer, "effect.move_down?layer=layer-cel&effect=fx-1"),
        );
        report.check(
            "none of those seven refusals put anything in the history",
            depth,
            held(&viewer).document.undo_depth(),
        );

        // ---- a locked layer ----------------------------------------------------------------------------
        run(&viewer, "layer.toggle_lock?layer=layer-cel");
        let depth = held(&viewer).document.undo_depth();
        report.check(
            "a locked layer refuses an effect, and says which rule stopped it",
            "The layer \"Cel\" is locked, so it was not changed. Unlock the layer to edit it.",
            run(&viewer, "effect.add?layer=layer-cel&type=core.exposure"),
        );
        report.check(
            "and refuses a settings change too",
            "The layer \"Cel\" is locked, so it was not changed. Unlock the layer to edit it.",
            run(
                &viewer,
                "effect.set_parameters?layer=layer-cel&effect=fx-1&sigma_px=2",
            ),
        );
        report.check(
            "and neither did the two the lock stopped",
            depth,
            held(&viewer).document.undo_depth(),
        );
        run(&viewer, "layer.toggle_lock?layer=layer-cel");
        report.check(
            "and the stack is the one that was built",
            "fx-unknown-1 vendor.future.effect on, fx-1 core.gaussian_blur on",
            stack(&viewer, l),
        );

        // ---- back to the file -----------------------------------------------------------------------------
        while held(&viewer).document.undo_depth() > 0 {
            undo(&viewer);
        }
        let held = held(&viewer);
        let opened = std::fs::read_to_string(&source)
            .expect("read the fixture")
            .replace("\r\n", "\n");
        let now = persist::to_json(held.document.project(), &held.preserved);
        let same = "identical, including the effect this build cannot model";
        report.check(
            "undoing every effect edit gives back the file that was opened",
            same,
            if opened == now {
                same
            } else {
                "what a save would write is no longer what was opened"
            },
        );
        drop(held);

        write_artifact(
            &report,
            "verification/B-12a_effects_table.md",
            "B-12a: what the effects panel does",
            EFFECTS_INTRO,
            EFFECTS_NOTES,
        );
        let failed: Vec<&String> = report
            .rows
            .iter()
            .filter(|(_, e, a)| e != a)
            .map(|(c, _, _)| c)
            .collect();
        assert!(failed.is_empty(), "these checks failed: {failed:#?}");
    }

    const EFFECTS_INTRO: &[&str] = &[
        "Document 21's three effects, built into a stack from the panel: added, bypassed, \
         deleted, and their settings typed. That each effect draws what document 21 says it \
         draws is checked in `verification/B-07_effects_table.md` and is not repeated here. What \
         is checked here is the part between a field in a panel and that arithmetic - that the \
         right effect is reached, that every setting travels on every change, and that a number \
         outside the range is refused in words rather than quietly turned into a different one.",
        "Every row runs on `Fixtures/projects/unknown_effect_project.json`, whose one layer \
         already carries an effect no version of this build has. It is there for the whole \
         table on purpose: a panel that lists effects is the place where an effect it cannot \
         draw is most likely to be dropped, and the last row saves the project back and compares \
         it to the file that was opened.",
    ];

    const EFFECTS_NOTES: &[&str] = &[
        "## What to look at\n\n- **An effect this build does not have is kept, and is honest \
         about it.** It can be bypassed and it can be deleted, because both are things a person \
         decided. Its settings cannot be changed, because this build has never seen the schema \
         they belong to, and the panel says so in a sentence rather than showing empty \
         fields.\n- **A new effect starts at the setting that changes no pixels.** Adding a blur \
         does not make the picture jump; the change a person then sees is the one they \
         typed.\n- **Every setting travels on every change.** A tint is a colour and an amount, \
         and a request that named only one would leave the other to a default - which would \
         reset it. A missing setting is refused instead.\n- **Where a refusal comes from is \
         visible in its words.** \"needs a number\" is the window, refusing text. \"needs a sigma \
         of zero or more\" is the core, refusing a number document 21 has no meaning for. Both \
         reach the status line the same way, and neither changes anything.\n- **A gap left by a \
         deleted effect is not filled.** Identifiers are counted from the highest in use, so an \
         effect in the middle of a stack cannot be replaced by a different one wearing its \
         identifier, and two runs of the same steps write the same file.",
        "## What this does not cover\n\nThe picture. These rows check what the panel does to the \
         project, not what the renderer then draws; the blur, exposure and tint themselves are \
         checked against independently generated weights in `verification/B-07_effects_table.md`, \
         and the frame with an effect on it is in the photographs beside this table.\n\nReordering \
         the stack. Document 21 evaluates effects in order and this panel adds each new one at \
         the end; there is no command in document 24 for moving one, and W-01 does not ask to \
         move one.\n\nA parameter over time. The settings here are constants, which is what \
         document 19 calls a parameter with no keyframes on it.",
    ];

    /// What shapes a layer, as the panels are given it.
    fn matte(viewer: &Mutex<Viewer>, layer_id: &str) -> String {
        let answer: serde_json::Value =
            serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
        answer["project"]["compositions"][0]["layers"]
            .as_array()
            .expect("a composition has layers")
            .iter()
            .find(|l| l["id"] == layer_id)
            .map(|l| l["matte"].to_string())
            .unwrap_or_else(|| "(no such layer)".to_string())
    }

    #[test]
    fn the_inspector_chooses_a_matte_and_refuses_the_ones_that_would_not_work() {
        let mut report = Report { rows: Vec::new() };
        let source = repo("Fixtures/projects/unknown_effect_project.json");
        let viewer = Mutex::new(
            open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
        );

        report.check(
            "the fixture's layer is shaped by nothing",
            "null",
            matte(&viewer, "layer-cel"),
        );
        // A matte is one layer shaping another, so there has to be another. This is the second
        // layer a person would draw the shape on.
        run(&viewer, "layer.create?asset=asset-cel&name=Shape");

        // ---- choosing one ------------------------------------------------------------------------
        report.check(
            "choosing a matte says which layer was chosen",
            "Set matte to layer-1",
            run(
                &viewer,
                "layer.set_matte?layer=layer-cel&matte=layer-1&only=false",
            ),
        );
        report.check(
            "and the panels are given it back, with the matte layer still drawn in its own right",
            r#"{"layer_id":"layer-1","matte_only":false,"mode":"alpha"}"#,
            matte(&viewer, "layer-cel"),
        );
        // D-42: the matte layer being kept out of the visible stack is a setting of its own,
        // rather than the matte layer being switched off, so that a file cannot say one thing
        // and mean another.
        report.check(
            "keeping the matte layer out of the picture is said in the words undo will use",
            "Set matte to layer-1, matte only",
            run(
                &viewer,
                "layer.set_matte?layer=layer-cel&matte=layer-1&only=true",
            ),
        );
        report.check(
            "and it is that setting that changed, not the matte layer's own switch",
            r#"{"layer_id":"layer-1","matte_only":true,"mode":"alpha"}"#,
            matte(&viewer, "layer-cel"),
        );
        report.check(
            "the layer used as a matte is still switched on, which is D-42's whole point",
            true,
            layer(&viewer, "layer-1", |l| l.enabled).unwrap_or(false),
        );

        // ---- clearing it ---------------------------------------------------------------------------
        report.check(
            "clearing the matte says so",
            "Clear matte",
            run(&viewer, "layer.set_matte?layer=layer-cel&matte=&only=false"),
        );
        report.check(
            "and the layer is shaped by nothing again",
            "null",
            matte(&viewer, "layer-cel"),
        );
        report.check(
            "undo puts back the matte that was cleared, and its setting with it",
            r#"{"layer_id":"layer-1","matte_only":true,"mode":"alpha"}"#,
            {
                undo(&viewer);
                matte(&viewer, "layer-cel")
            },
        );

        // ---- what is refused ------------------------------------------------------------------------
        let depth = held(&viewer).document.undo_depth();
        report.check(
            "a layer that is not in this composition cannot be a matte, and is named",
            "The layer chosen as a matte, layer-gone, is not in this composition.",
            run(
                &viewer,
                "layer.set_matte?layer=layer-cel&matte=layer-gone&only=false",
            ),
        );
        // Document 19 forbids the cycle and the core refuses it. The list in the panel does not
        // offer a layer itself, so this is the second line of defence rather than the first.
        report.check(
            "a layer cannot be its own matte",
            "That matte would make two layers depend on each other. Choose a layer that does \
             not already use this one as its matte.",
            run(
                &viewer,
                "layer.set_matte?layer=layer-cel&matte=layer-cel&only=false",
            ),
        );
        report.check(
            "and two layers cannot shape each other",
            "That matte would make two layers depend on each other. Choose a layer that does \
             not already use this one as its matte.",
            run(
                &viewer,
                "layer.set_matte?layer=layer-1&matte=layer-cel&only=false",
            ),
        );
        report.check(
            "no layer named at all is asked for",
            "Which layer? Choose one in the layer list.",
            run(&viewer, "layer.set_matte?matte=layer-1"),
        );
        report.check(
            "none of those four refusals put anything in the history",
            depth,
            held(&viewer).document.undo_depth(),
        );
        report.check(
            "and the matte is the one that was chosen",
            r#"{"layer_id":"layer-1","matte_only":true,"mode":"alpha"}"#,
            matte(&viewer, "layer-cel"),
        );

        // ---- a locked layer ---------------------------------------------------------------------------
        run(&viewer, "layer.toggle_lock?layer=layer-cel");
        let depth = held(&viewer).document.undo_depth();
        report.check(
            "a locked layer refuses a matte, and says which rule stopped it",
            "The layer \"Cel\" is locked, so it was not changed. Unlock the layer to edit it.",
            run(&viewer, "layer.set_matte?layer=layer-cel&matte=&only=false"),
        );
        report.check(
            "which changed nothing",
            depth,
            held(&viewer).document.undo_depth(),
        );
        run(&viewer, "layer.toggle_lock?layer=layer-cel");

        // ---- the matte layer going away ----------------------------------------------------------------
        // A matte names a layer, and a layer can be deleted. The reference is kept rather than
        // cleared -- that is the project loader's rule, which reports it as a warning and leaves
        // it alone -- so what this window owes the person is to say it at the moment it happens
        // rather than the next time the file is opened.
        report.check(
            "deleting the layer used as a matte says what it did to the layer it was shaping",
            "Delete layer layer-1 \"Cel\" is now shaped by a layer that is not here; undo puts \
             it back.",
            run(&viewer, "layer.delete?layer=layer-1"),
        );
        report.check(
            "and the reference is kept, which is what the project loader expects to find",
            r#"{"layer_id":"layer-1","matte_only":true,"mode":"alpha"}"#,
            matte(&viewer, "layer-cel"),
        );
        report.check(
            "and undo brings the layer back, with the matte pointing at it again",
            "Cel, Shape",
            {
                undo(&viewer);
                names(&viewer)
            },
        );

        // ---- back to the file ---------------------------------------------------------------------------
        while held(&viewer).document.undo_depth() > 0 {
            undo(&viewer);
        }
        let held = held(&viewer);
        let opened = std::fs::read_to_string(&source)
            .expect("read the fixture")
            .replace("\r\n", "\n");
        let now = persist::to_json(held.document.project(), &held.preserved);
        let same = "identical, including the effect this build cannot model";
        report.check(
            "undoing everything gives back the file that was opened",
            same,
            if opened == now {
                same
            } else {
                "what a save would write is no longer what was opened"
            },
        );
        drop(held);

        write_artifact(
            &report,
            "verification/B-12a_matte_table.md",
            "B-12a: what the matte chooser does",
            MATTE_INTRO,
            MATTE_NOTES,
        );
        let failed: Vec<&String> = report
            .rows
            .iter()
            .filter(|(_, e, a)| e != a)
            .map(|(c, _, _)| c)
            .collect();
        assert!(failed.is_empty(), "these checks failed: {failed:#?}");
    }

    const MATTE_INTRO: &[&str] = &[
        "W-01 asks the artist to apply a matte: one layer shaping another, which is how a cel is \
         held inside a shape rather than being cut with a pair of scissors. That the matte is \
         then drawn the way document 21 says - the alpha of one layer multiplying the other's - \
         is checked in `verification/B-06_mask_table.md` and is not repeated here. What is \
         checked here is the part between a chooser in a panel and that arithmetic: that the \
         layer chosen is the layer used, that the settings the file holds together travel \
         together, and that the arrangements document 19 forbids are refused in a sentence.",
        "One command carries all three of choosing a matte, clearing it, and choosing whether \
         the matte layer is still drawn in its own right. `layer.set_matte` is added to \
         document 24 for it, which is written up beside the row.",
    ];

    const MATTE_NOTES: &[&str] = &[
        "## What to look at\n\n- **The matte layer is not switched off.** D-42 made \"keep this \
         layer out of the visible stack\" a setting of its own rather than reusing the layer's \
         own switch, so that a file showing a layer switched off while it visibly shapes the \
         picture cannot happen. The row after the one that sets it checks the matte layer is \
         still on.\n- **A cycle is refused, twice over.** The chooser does not offer a layer \
         itself, and the core refuses it as well; the rows here go through the request the \
         chooser sends, so what they check is the second line of defence.\n- **Deleting the \
         layer that was being used as a matte keeps the reference, and says so.** That is the \
         project loader's rule - it reports such a reference as a warning and leaves it alone, \
         so that a layer deleted by mistake can be undone back into place - and this window \
         says it in the status line at the moment it happens rather than leaving it to be met \
         the next time the file is opened.\n- **Clearing is the same command with no layer named**, which is what the \
         \"none\" entry in the list sends. A layer that has gone is a different thing and is \
         refused.",
        "## What this does not cover\n\nThe shape. This panel chooses which layer is the matte; \
         drawing an outline to be the matte is `SetMask` in the core and has no gesture in this \
         window yet. A matte is a whole layer here, which is what an artist with a scanned shape \
         has and what W-01 describes.\n\nWhich channel the matte uses. Document 21 defines an \
         alpha matte and this build has one kind; a luminance matte is not in G1.",
    ];

    /// A layer's exposures as an exposure sheet reads them: which frames, and which drawing.
    fn exposures(viewer: &Mutex<Viewer>, layer_id: &str) -> String {
        layer(viewer, layer_id, |l| {
            l.exposure_spans
                .iter()
                .map(|s| {
                    format!(
                        "{}-{}:{}",
                        s.start_frame, s.end_frame_exclusive, s.drawing_number
                    )
                })
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_else(|| "(no such layer)".to_string())
    }

    /// The project's drawings, as the media bin lists them.
    fn bin(viewer: &Mutex<Viewer>) -> String {
        held(viewer)
            .document
            .project()
            .assets
            .iter()
            .map(|a| format!("{} \"{}\" {} drawings", a.id, a.name, a.frames.len()))
            .collect::<Vec<_>>()
            .join("; ")
    }

    /// Where one drawing of an imported sequence ended up in the project record.
    fn stored(viewer: &Mutex<Viewer>, asset: &str, drawing: u32) -> String {
        held(viewer)
            .document
            .project()
            .assets
            .iter()
            .find(|a| a.id == Id::new(asset))
            .and_then(|a| a.frames.get(&drawing).cloned())
            .unwrap_or_else(|| "(no such drawing)".to_string())
    }

    fn notes_mention(viewer: &Mutex<Viewer>, words: &str) -> bool {
        held(viewer).notes.iter().any(|n| n.contains(words))
    }

    /// The PNG files in one folder of the reference shot, which is the selection a person makes
    /// in the dialog. Sorted, because a file dialog's order is its own business.
    fn drawings_in(folder: &str) -> Vec<PathBuf> {
        let dir = repo(&format!("Fixtures/reference_shot/{folder}"));
        let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter(|p| p.extension().is_some_and(|x| x == "png"))
            .collect();
        files.sort();
        files
    }

    /// The request the media bin sends once the file dialog has been answered.
    fn import_of(files: &[PathBuf]) -> String {
        let parts: Vec<String> = files
            .iter()
            .map(|p| format!("file={}", for_a_header(&p.display().to_string())))
            .collect();
        format!("media.import?{}", parts.join("&"))
    }

    #[test]
    fn the_media_bin_imports_a_sequence_and_the_exposures_it_is_given() {
        let mut report = Report { rows: Vec::new() };
        let source = repo("Fixtures/projects/cel_holds_project.json");
        let viewer = Mutex::new(
            open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
        );

        // ---- the exposures the file already holds -------------------------------------------
        report.check(
            "the fixture holds drawing 1 for two frames and drawing 2 for three",
            "0-2:1, 2-5:2",
            exposures(&viewer, "layer-cel"),
        );
        {
            let answer: serde_json::Value =
                serde_json::from_str(&state(&viewer)).expect("the state answer is JSON");
            report.check(
                "and the panels are given them to draw",
                r#"[{"drawing_number":1,"end_frame_exclusive":2,"start_frame":0},"#.to_string()
                    + r#"{"drawing_number":2,"end_frame_exclusive":5,"start_frame":2}]"#,
                answer["project"]["compositions"][0]["layers"][0]["exposure_spans"].to_string(),
            );
        }

        // ---- changing one ------------------------------------------------------------------
        report.check(
            "shortening a hold says how many exposures the layer now has",
            "Set 2 exposures",
            run(
                &viewer,
                "exposure.set_span?layer=layer-cel&was=2&start=2&end=4&drawing=2",
            ),
        );
        report.check(
            "and the hold is one frame shorter, with the other one untouched",
            "0-2:1, 2-4:2",
            exposures(&viewer, "layer-cel"),
        );
        // Document 20's own example: drawing numbers are under no ordering constraint, because a
        // re-exposure of an earlier drawing is ordinary cel work.
        report.check(
            "a later exposure may show an earlier drawing again",
            "Set 3 exposures",
            run(
                &viewer,
                "exposure.set_span?layer=layer-cel&start=4&end=5&drawing=1",
            ),
        );
        report.check(
            "and the sheet reads in frame order, not drawing order",
            "0-2:1, 2-4:2, 4-5:1",
            exposures(&viewer, "layer-cel"),
        );
        // Moving a row is one row edited. The request carries the frame the row starts at now as
        // well as the one being typed, so that an exposure dragged along the sheet does not
        // leave a copy of itself where it was.
        report.check(
            "moving an exposure to another frame moves it rather than copying it",
            "0-2:1, 2-4:2, 5-6:1",
            {
                run(
                    &viewer,
                    "exposure.set_span?layer=layer-cel&was=4&start=5&end=6&drawing=1",
                );
                exposures(&viewer, "layer-cel")
            },
        );
        report.check(
            "clearing one says how many are left",
            "Set 2 exposures",
            run(
                &viewer,
                "exposure.set_span?layer=layer-cel&was=5&start=5&drawing=",
            ),
        );
        report.check(
            "and it is the cleared one that has gone",
            "0-2:1, 2-4:2",
            exposures(&viewer, "layer-cel"),
        );

        // ---- a drawing the sequence has not got -----------------------------------------------
        // Document 28: a frame whose drawing is missing renders as nothing and no neighbouring
        // drawing is put there instead. The command is allowed - a person may be exposing a
        // drawing that has not been scanned yet - and is told what it will look like.
        report.check(
            "exposing a drawing that is not in the sequence is allowed and said",
            "Set 3 exposures Drawing 7 is not in this sequence, so the frames exposing it stay \
             empty; no neighbouring drawing is put there instead.",
            run(
                &viewer,
                "exposure.set_span?layer=layer-cel&start=4&end=5&drawing=7",
            ),
        );
        report.check(
            "and it was applied, not refused",
            "0-2:1, 2-4:2, 4-5:7",
            exposures(&viewer, "layer-cel"),
        );
        report.check(
            "undo takes that exposure back off the sheet",
            "0-2:1, 2-4:2",
            {
                undo(&viewer);
                exposures(&viewer, "layer-cel")
            },
        );

        // ---- what is refused ------------------------------------------------------------------
        let depth = held(&viewer).document.undo_depth();
        // Document 20's rule, in the core's words. The window does not have a copy of it.
        report.check(
            "two exposures cannot cover one frame",
            "Those exposures cannot be used: exposure spans overlap or are out of order: a span \
             ends at 2 and the next starts at 1.",
            run(
                &viewer,
                "exposure.set_span?layer=layer-cel&start=1&end=3&drawing=2",
            ),
        );
        report.check(
            "an exposure that covers no frame is not an exposure",
            "Those exposures cannot be used: exposure span [6, 6) covers no frame.",
            run(
                &viewer,
                "exposure.set_span?layer=layer-cel&start=6&end=6&drawing=1",
            ),
        );
        report.check(
            "clearing an exposure that is not there says so",
            "There is no exposure starting at frame 9 to clear.",
            run(
                &viewer,
                "exposure.set_span?layer=layer-cel&start=9&drawing=",
            ),
        );
        report.check(
            "a frame number that is not a number is named before the core sees it",
            "An exposure starts at a whole frame number. Not \"two\".",
            run(
                &viewer,
                "exposure.set_span?layer=layer-cel&start=two&end=4&drawing=1",
            ),
        );
        report.check(
            "and so is a drawing number that is not one",
            "A drawing number is a whole number, counting from zero. Not \"-1\".",
            run(
                &viewer,
                "exposure.set_span?layer=layer-cel&start=6&end=7&drawing=-1",
            ),
        );
        report.check(
            "an exposure with no end is asked for rather than guessed at",
            "Which frame does the exposure end before?",
            run(
                &viewer,
                "exposure.set_span?layer=layer-cel&start=6&drawing=1",
            ),
        );
        report.check(
            "none of those six refusals put anything in the history",
            depth,
            held(&viewer).document.undo_depth(),
        );
        report.check(
            "and the sheet is the one that was there",
            "0-2:1, 2-4:2",
            exposures(&viewer, "layer-cel"),
        );
        run(&viewer, "layer.toggle_lock?layer=layer-cel");
        let depth = held(&viewer).document.undo_depth();
        report.check(
            "a locked layer refuses an exposure, and says which rule stopped it",
            "The layer \"Cel\" is locked, so it was not changed. Unlock the layer to edit it.",
            run(
                &viewer,
                "exposure.set_span?layer=layer-cel&start=6&end=7&drawing=1",
            ),
        );
        report.check(
            "which changed nothing",
            depth,
            held(&viewer).document.undo_depth(),
        );
        run(&viewer, "layer.toggle_lock?layer=layer-cel");

        // ---- importing drawings -----------------------------------------------------------------
        report.check(
            "the media bin starts with the one sequence the file names",
            "asset-cel \"Cel\" 2 drawings",
            bin(&viewer),
        );
        report.check(
            "an import that names no file asks for one rather than importing nothing",
            "Which drawings should be imported? Choose the files themselves, not the folder \
             they are in.",
            run(&viewer, "media.import"),
        );
        // Layer 3 of the reference shot is the sequence with a hole in it: eleven files
        // numbered 0 to 11, and no drawing 7.
        let layer3 = drawings_in("layer3");
        report.check(
            "importing a sequence with a gap says how many drawings arrived and what is missing",
            "Import layer3_%03d.png: 11 drawings, numbered 0 to 11, and drawing 7 is missing.",
            run(&viewer, &import_of(&layer3)),
        );
        report.check(
            "the drawing that is missing is missing from the record, not filled in",
            "0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11",
            held(&viewer)
                .document
                .project()
                .assets
                .iter()
                .find(|a| a.id == Id::new("asset-1"))
                .map(|a| {
                    a.frames
                        .keys()
                        .map(u32::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .unwrap_or_else(|| "(the sequence was not imported)".to_string()),
        );
        report.check(
            "the importer's own warning about the gap is kept, in its words",
            true,
            notes_mention(&viewer, "One drawing is missing from layer3_%03d.png: 7."),
        );
        report.check(
            "media chosen from outside the project's folder keeps its whole path",
            true,
            stored(&viewer, "asset-1", 0)
                .ends_with("Fixtures/reference_shot/layer3/layer3_000.png"),
        );
        report.check(
            "and it is written with forward slashes, so the project can be handed over",
            true,
            !stored(&viewer, "asset-1", 0).contains('\\'),
        );
        // Layer 2 is the other trap: drawing 13 is called `layer2_桜_013.png`, which the pattern
        // does not generate. It is a complete sequence and must not be reported as one with a
        // hole at 13.
        let layer2 = drawings_in("layer2");
        report.check(
            "a sequence whose names are not all alike is complete, not full of holes",
            "Import layer2_%03d.png: 24 drawings, numbered 0 to 23.",
            run(&viewer, &import_of(&layer2)),
        );
        report.check(
            "the drawing with the Japanese name is stored under the name it has",
            true,
            stored(&viewer, "asset-2", 13).ends_with("layer2_桜_013.png"),
        );
        report.check(
            "and it was noticed rather than passed over in silence",
            true,
            notes_mention(
                &viewer,
                "One file does not match the pattern layer2_%03d.png but carries a clear number",
            ),
        );
        report.check(
            "both sequences are in the bin, beside the one the file came with",
            "asset-cel \"Cel\" 2 drawings; asset-1 \"layer3_%03d.png\" 11 drawings; \
             asset-2 \"layer2_%03d.png\" 24 drawings",
            bin(&viewer),
        );
        report.check(
            "an import can be undone",
            "Undone: Import layer2_%03d.png",
            undo(&viewer),
        );
        report.check(
            "and the drawings go with it",
            "asset-cel \"Cel\" 2 drawings; asset-1 \"layer3_%03d.png\" 11 drawings",
            bin(&viewer),
        );
        // Not every file is a drawing. A selection that forms no sequence imports nothing and
        // says what the importer made of it.
        report.check(
            "a file that is not a numbered drawing imports nothing and says why",
            "The chosen file does not form an image sequence, so nothing was imported. One \
             selected file has no number in its name and was not imported. Import it as a still \
             image, or rename it so it carries a drawing number.",
            run(&viewer, &import_of(std::slice::from_ref(&source))),
        );

        // ---- back to the file ---------------------------------------------------------------
        while held(&viewer).document.undo_depth() > 0 {
            undo(&viewer);
        }
        let held = held(&viewer);
        let opened = std::fs::read_to_string(&source)
            .expect("read the fixture")
            .replace("\r\n", "\n");
        let now = persist::to_json(held.document.project(), &held.preserved);
        let same = "identical, drawings, exposures and all";
        report.check(
            "undoing everything gives back the file that was opened",
            same,
            if opened == now {
                same
            } else {
                "what a save would write is no longer what was opened"
            },
        );
        drop(held);

        write_artifact(
            &report,
            "verification/B-12a_media_table.md",
            "B-12a: what the media bin and the exposure sheet do",
            MEDIA_INTRO,
            MEDIA_NOTES,
        );
        let failed: Vec<&String> = report
            .rows
            .iter()
            .filter(|(_, e, a)| e != a)
            .map(|(c, _, _)| c)
            .collect();
        assert!(failed.is_empty(), "these checks failed: {failed:#?}");
    }

    const MEDIA_INTRO: &[&str] = &[
        "W-01 begins before any layer exists: the artist \"imports media, reviews sequence \
         grouping and missing-frame warnings, creates a composition and assigns exposures\". \
         This is the window's half of those steps. That B-03 groups a selection correctly is \
         checked in `verification/B-03_import_table.md`, and that the exposure sheet resolves \
         to the right drawing at each frame in `verification/B-04_exposure_table.md`; neither \
         is repeated here. What is checked here is the part between a panel and those two: that \
         what the importer found reaches the person in words, that the record written into the \
         project is the one the importer described, and that a row typed into the exposure \
         sheet becomes the span the file holds.",
        "The exposures run on `Fixtures/projects/cel_holds_project.json`, which already holds a \
         drawing for two frames and another for three. The imports run on the reference shot, \
         whose layer 3 is missing drawing 7 and whose layer 2 carries drawing 13 under a \
         Japanese name the pattern does not generate. Those two folders are the reason the \
         importer was written the way it was, and they are the two cases a media bin can most \
         easily lie about.",
        "`exposure.set_span` is in document 24's table and had no command behind it. \
         `SetExposureSpans` is added to the core by this unit, taking the whole ordered list \
         the way `SetMask` takes a whole outline, and the reason is written beside it.",
    ];

    const MEDIA_NOTES: &[&str] = &[
        "## What to look at\n\n- **The gap is a fact, not an absence.** Layer 3 imports as \
         eleven drawings numbered 0 to 11 with no 7, and the record says so. An importer that \
         renumbered what it found would slide every later drawing one frame early and nothing \
         would look wrong until somebody counted.\n- **The complete sequence is not reported as \
         a broken one.** Layer 2's drawing 13 is called `layer2_桜_013.png`. It is imported \
         under its own name, the sequence is complete, and the mismatch is mentioned rather \
         than treated as a hole.\n- **Exposing a drawing that does not exist is allowed and \
         said.** Document 28 renders such a frame as nothing and substitutes no neighbour. A \
         person exposing a drawing that has not been scanned yet is told what it will look \
         like; they are not stopped.\n- **The rule about overlapping exposures is the core's.** \
         The two refusals quote document 20's own words through the same check the loader and \
         the renderer use. The window has no copy of that rule to fall out of step with.\n- \
         **Moving an exposure moves it.** The row carries the frame it starts at now as well as \
         the frame being typed, so an exposure dragged along the sheet does not leave a copy of \
         itself behind.",
        "## What this does not cover\n\nThe file dialog. Choosing files belongs to the \
         operating system and a test has no hands to answer one, so every row here begins at \
         the selection a person made. The button that opens it is in the media bin and is in \
         the photographs.\n\nStills. `media.import` builds an image sequence, because that is \
         what W-01 imports and what the reference shot is. A single still image is a record \
         this build's model already has and its own import is not built.\n\nRelinking, which is \
         W-02 and its own table.",
    ];

    /// The request the media bin sends once the relink dialog has been answered.
    fn relink_of(asset: &str, files: &[PathBuf]) -> String {
        let parts: Vec<String> = files
            .iter()
            .map(|p| format!("file={}", for_a_header(&p.display().to_string())))
            .collect();
        format!("media.relink?asset={asset}&{}", parts.join("&"))
    }

    /// Two drawings of a size nothing in the reference shot has.
    fn tiny_drawings() -> Vec<PathBuf> {
        let directory = std::env::temp_dir().join("anime_compositor_b12a_relink");
        let _ = std::fs::remove_dir_all(&directory);
        std::fs::create_dir_all(&directory).expect("make the scratch directory");
        (0..2)
            .map(|n| {
                let file = directory.join(format!("tiny_{n:03}.png"));
                anime_compositor::png_out::write_rgba(
                    &file,
                    8,
                    8,
                    OutputDepth::Eight,
                    &[],
                    &[0u8; 8 * 8 * 4],
                )
                .expect("write a tiny drawing");
                file
            })
            .collect()
    }

    /// One sentence out of a proposal, so that a row can show the part it is about.
    fn sentence_about(said: &str, opening: &str) -> String {
        said.split(". ")
            .find(|s| s.starts_with(opening))
            .map(|s| format!("{}.", s.trim_end_matches('.')))
            .unwrap_or_else(|| format!("(nothing was said starting \"{opening}\")"))
    }

    /// What the frame carries about a relink waiting to be agreed to, which is all the panel
    /// offering it ever knows. It rides on the frame rather than on an answer of its own so that
    /// a panel cannot outlive the proposal it is showing.
    fn on_the_frame(viewer: &Mutex<Viewer>) -> (String, String) {
        let export = Mutex::new(Export::default());
        let (ask, quality) = parse("/frame/0", None).expect("a readable request");
        let response = serve(viewer, &export, ask, quality);
        let text = |name: &str| {
            response.headers().get(name).map_or_else(String::new, |v| {
                from_a_query(v.to_str().unwrap_or_default())
            })
        };
        (text("x-relink"), text("x-relink-asset"))
    }

    #[test]
    fn relinking_says_what_would_change_before_anything_changes() {
        let mut report = Report { rows: Vec::new() };
        let source = repo("Fixtures/projects/cel_holds_project.json");
        let viewer = Mutex::new(
            open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
        );
        let replacements = drawings_in("layer3");

        // ---- what cannot be relinked -------------------------------------------------------
        report.check(
            "relinking without naming a sequence asks which one",
            "Which sequence should be relinked? Choose one in the media bin.",
            run(&viewer, "media.relink"),
        );
        report.check(
            "naming a sequence but no drawings asks for the drawings themselves",
            "Which drawings should the sequence point at? Choose the files themselves, not the \
             folder they are in.",
            run(&viewer, "media.relink?asset=asset-cel"),
        );
        report.check(
            "a sequence that is not in the project is refused",
            "That media is not in this project. Nothing was changed.",
            run(&viewer, &relink_of("asset-gone", &replacements)),
        );
        report.check(
            "and agreeing to a relink nobody proposed is refused",
            "There is no relink waiting for that sequence. Choose the replacement drawings \
             first, so that what would change can be read before it happens.",
            run(&viewer, "media.relink?asset=asset-cel&apply=1"),
        );

        // ---- what would change, said before it does -----------------------------------------
        let said = run(&viewer, &relink_of("asset-cel", &replacements));
        report.check(
            "choosing the replacement drawings answers with what relinking would do",
            "Relinking \"Cel\" to layer3_%03d.png would give it 11 drawings, numbered 0 to 11, \
             and drawing 7 would be missing. The drawings are 1920x1080. What it points at now \
             cannot be read, so there is nothing to compare that with. The colour is read as \
             sRGB with straight alpha, which is what this project already recorded: a PNG does \
             not state either, so relinking does not change how the pixels are read. The layers \
             using it keep their stacking, transforms, exposures, masks and effects. Nothing \
             has changed yet. One drawing is missing from layer3_%03d.png: 7. Add the missing \
             files to the folder and relink the sequence, or leave the gap if the hole is \
             intended.",
            &said,
        );
        let (waiting, about) = on_the_frame(&viewer);
        report.check(
            "the panel offering it reads it off the frame, word for word",
            &said,
            &waiting,
        );
        report.check(
            "and the frame says which sequence it is about",
            "asset-cel",
            &about,
        );
        report.check(
            "the drawings the sequence points at have not moved",
            "asset-cel \"Cel\" 2 drawings",
            bin(&viewer),
        );
        report.check(
            "there is nothing to undo, because nothing was done",
            0,
            held(&viewer).document.undo_depth(),
        );

        // ---- and it can be walked away from -------------------------------------------------
        report.check(
            "leaving it as it is says so",
            "The relink was dropped. The sequence still points at the drawings it did.",
            run(&viewer, "media.relink?asset=asset-cel&cancel=1"),
        );
        report.check("the panel goes with it", "", on_the_frame(&viewer).0);
        report.check(
            "the sequence still points where it did",
            "asset-cel \"Cel\" 2 drawings",
            bin(&viewer),
        );
        report.check(
            "and dropping a second time has nothing to drop",
            "There is no relink waiting to be dropped.",
            run(&viewer, "media.relink?asset=asset-cel&cancel=1"),
        );

        // ---- agreeing to it ------------------------------------------------------------------
        run(&viewer, &relink_of("asset-cel", &replacements));
        report.check(
            "a proposal about one sequence cannot be applied to another",
            "There is no relink waiting for that sequence. Choose the replacement drawings \
             first, so that what would change can be read before it happens.",
            run(&viewer, "media.relink?asset=asset-1&apply=1"),
        );
        report.check(
            "and the proposal it was not about is still waiting",
            "asset-cel",
            on_the_frame(&viewer).1,
        );
        report.check(
            "agreeing names what was done, so that undo can be recognised",
            "Relink Cel",
            run(&viewer, "media.relink?asset=asset-cel&apply=1"),
        );
        report.check(
            "the sequence now points at the drawings that were chosen",
            "asset-cel \"Cel\" 11 drawings",
            bin(&viewer),
        );
        report.check(
            "the gap is still a gap: the drawing that is not there was not invented",
            "(no such drawing)",
            stored(&viewer, "asset-cel", 7),
        );
        report.check(
            "the layer that used it is the same layer, holding the same drawings for the same \
             frames",
            "0-2:1, 2-5:2",
            exposures(&viewer, "layer-cel"),
        );
        report.check(
            "the panel is gone, because there is nothing left to agree to",
            "",
            on_the_frame(&viewer).0,
        );
        report.check(
            "and it is one thing to undo",
            "Undone: Relink Cel",
            undo(&viewer),
        );
        report.check(
            "which puts the drawings back",
            "asset-cel \"Cel\" 2 drawings",
            bin(&viewer),
        );

        // ---- the comparison W-02 asks for, when there is something to compare -----------------
        run(&viewer, &import_of(&drawings_in("layer3")));
        let said = run(&viewer, &relink_of("asset-1", &tiny_drawings()));
        report.check(
            "relinking a sequence that can be read says what size it is now and what size it \
             would become",
            "The drawings are 8x8, where the ones it points at now are 1920x1080.",
            sentence_about(&said, "The drawings are"),
        );
        report.check(
            "and says the alpha is read the way this project already reads it, because a PNG \
             does not say",
            "The colour is read as sRGB with straight alpha, which is what this project \
             already recorded: a PNG does not state either, so relinking does not change how \
             the pixels are read.",
            sentence_about(&said, "The colour is read"),
        );

        write_artifact(
            &report,
            "verification/B-12a_relink_table.md",
            "B-12a: relinking, and what is said before anything changes",
            RELINK_INTRO,
            RELINK_NOTES,
        );
        let failed: Vec<&String> = report
            .rows
            .iter()
            .filter(|(_, e, a)| e != a)
            .map(|(c, _, _)| c)
            .collect();
        assert!(failed.is_empty(), "these checks failed: {failed:#?}");
    }

    const RELINK_INTRO: &[&str] = &[
        "W-02 is one sentence: relink \"by explicit user choice; preserve layer identity, \
         timing, masks and effects\", and \"present changed dimensions, frame range or alpha \
         interpretation before applying the change\". The presenting is the part that cannot be \
         done by a core command, because it happens before there is a command, so it is built \
         here and checked here. That relinking preserves layer identity once it is applied is \
         the core's own behaviour and is checked in \
         `verification/B-09_persistence_table.md`; this table checks the window's half.",
        "Relinking is three requests rather than one, and that is the requirement rather than a \
         convenience: choosing the drawings works out what would happen and says it, and a \
         second request either agrees to it or drops it. Between the two, nothing about the \
         project has moved - the rows below check the drawings, the history and the file rather \
         than trusting the sentence that says so.",
    ];

    const RELINK_NOTES: &[&str] = &[
        "## What to look at\n\n- **The proposal is on the frame.** The sentence a person is \
         being asked to agree to travels back with the picture, like everything else this \
         window says, so a panel offering a relink cannot outlive the proposal it describes. \
         Cancel it, apply it, or open another project, and the panel goes with the next \
         frame.\n- **All three of W-02's changes are stated whether or not they changed.** The \
         size is given even when it is the same, the range and the gaps are given in full, and \
         the alpha reading is stated as carried over. \"Unchanged\" is an answer; silence is \
         not.\n- **What cannot be read is said to be unreadable.** Relinking is what a person \
         does when the media has gone, so the usual case is that there is nothing to compare \
         the new size against. That case says so rather than reporting the drawings as the \
         same size.\n- **A proposal belongs to one sequence.** Agreeing to it while another \
         sequence is selected is refused, and the proposal is still there afterwards.\n- **The \
         gap survives.** Relinking to a sequence missing drawing 7 leaves drawing 7 missing, \
         which document 28 renders as nothing.",
        "## What this does not cover\n\nThe file dialog, as everywhere else in this window: a \
         test has no hands to answer one, so every row begins at the selection a person \
         made.\n\nMasks. W-02 names them among what a relink preserves, and it does preserve \
         them - relinking replaces one asset record and cannot reach a layer - but this build \
         has no gesture for drawing a mask, so there is no mask here to photograph surviving \
         one.\n\nRelinking to a sequence in a different colour space. A PNG states neither the \
         colour space nor the alpha mode, so nothing read off the new drawings could \
         contradict the project; a format that states them is not in G1.",
    ];

    /// Every composition in the project, in the order the file holds them.
    fn comp_ids(viewer: &Mutex<Viewer>) -> String {
        held(viewer)
            .document
            .project()
            .compositions
            .iter()
            .map(|c| c.id.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// The composition the window is showing, written out the way a person would read it.
    fn on_screen(viewer: &Mutex<Viewer>) -> String {
        let held = held(viewer);
        match held.document.project().composition(&held.composition) {
            None => format!("{} is not in the project", held.composition),
            Some(comp) => format!(
                "{} {} {}x{} at {} fps, {} frames",
                comp.id,
                comp.name,
                comp.width,
                comp.height,
                comp.frame_rate.numerator() / comp.frame_rate.denominator(),
                comp.duration_frames
            ),
        }
    }

    /// W-01's third step, which had no command in this build until B-12d.
    ///
    /// Writes `verification/B-12d_new_composition_table.md`.
    #[test]
    fn a_composition_can_be_made_and_the_window_moves_into_it() {
        let mut report = Report { rows: Vec::new() };
        // `minimal_project.json` is the one fixture whose only composition is empty, which is
        // the state this command exists for: a project somebody can open with nothing in it to
        // work in.
        let source = repo("Fixtures/projects/minimal_project.json");
        let viewer = Mutex::new(
            open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
        );
        let before = held(&viewer).document.undo_depth();

        // ---- what somebody who fills in nothing gets -----------------------------------------
        let made = run(&viewer, "composition.create");
        report.check(
            "creating one with nothing filled in says what was made and that it is empty",
            "New composition New composition, 1920x1080 at 24 fps, 240 frames. It is empty; \
             import drawings and add layers to fill it.",
            &made,
        );
        report.check(
            "and the window is showing the composition that was just made",
            "comp-1 New composition 1920x1080 at 24 fps, 240 frames",
            on_screen(&viewer),
        );
        report.check(
            "and the one the file already held is still in the project",
            "comp-main, comp-1",
            comp_ids(&viewer),
        );
        report.check(
            "and the transport is the new composition's length, not the old one's",
            240,
            held(&viewer).playback.length(),
        );
        report.check(
            "and it is one history record, so one undo takes it back",
            before + 1,
            held(&viewer).document.undo_depth(),
        );

        // ---- the four refusals, each a sentence rather than a control springing back ----------
        let where_it_was = on_screen(&viewer);
        report.check(
            "a composition with no width is refused, and the refusal says which three matter",
            "A composition needs a width, a height and a length, and one of them was zero.",
            run(&viewer, "composition.create?width=0"),
        );
        report.check(
            "a composition larger than this build will make is refused with the limit in it",
            "20000x1080 is larger than this build will make: no side past 16384 and no more \
             than 67108864 pixels in all.",
            run(&viewer, "composition.create?width=20000"),
        );
        report.check(
            "a composition with no length is refused by the same sentence as one with no width",
            "A composition needs a width, a height and a length, and one of them was zero.",
            run(&viewer, "composition.create?frames=0"),
        );
        report.check(
            "a composition inside both side limits but past the pixel budget is still refused",
            "16384x16384 is larger than this build will make: no side past 16384 and no more \
             than 67108864 pixels in all. At 16384 wide the tallest this build will make is \
             4096.",
            run(&viewer, "composition.create?width=16384&height=16384"),
        );
        report.check(
            "a composition longer than this build will make is refused with the limit in it",
            "50000 frames is longer than this build will make; the limit is 10000.",
            run(&viewer, "composition.create?frames=50000"),
        );
        report.check(
            "a width that is not a number is refused before the core is asked",
            "\"wide\" is not a number of width. A composition needs whole numbers for its \
             width, its height, its frame rate and its length.",
            run(&viewer, "composition.create?width=wide"),
        );
        report.check(
            "a frame rate of zero is refused by the rule that owns frame rates",
            "A frame rate of zero is not a frame rate. Twenty-four is the one the reference \
             shot uses.",
            run(&viewer, "composition.create?fps=0"),
        );
        report.check(
            "and after seven refusals the project has exactly what it had before them",
            "comp-main, comp-1",
            comp_ids(&viewer),
        );
        report.check(
            "and the window is still looking at what it was looking at",
            where_it_was,
            on_screen(&viewer),
        );

        // ---- undo, which takes away the composition the window is showing --------------------
        report.check(
            "undo names the composition it took back rather than saying \"undone\"",
            "Undone: New composition New composition",
            undo(&viewer),
        );
        report.check(
            "and the window has moved off the composition that is no longer there",
            "comp-main Main 1920x1080 at 24 fps, 24 frames",
            on_screen(&viewer),
        );
        report.check(
            "and the project is back to the one composition the file held",
            "comp-main",
            comp_ids(&viewer),
        );
        report.check(
            "redo puts it back",
            "Redone: New composition New composition",
            redo(&viewer),
        );
        report.check(
            "and the window is showing it again",
            "comp-1 New composition 1920x1080 at 24 fps, 240 frames",
            on_screen(&viewer),
        );

        // ---- a second one, which must not be given the first one's identifier ----------------
        run(&viewer, "composition.create?name=Second&frames=48");
        report.check(
            "a second composition gets an identifier the first one is not using",
            "comp-main, comp-1, comp-2",
            comp_ids(&viewer),
        );
        report.check(
            "and a composition whose identifier is already in the project is refused",
            "A composition with the ID comp-1 is already in the project.",
            {
                let composition = Composition::new(
                    Id::new("comp-1"),
                    "A second comp-1",
                    1920,
                    1080,
                    FrameRate::new(24, 1).expect("24 fps"),
                    0,
                    240,
                );
                edit(
                    &viewer,
                    Command::AddComposition {
                        composition: Box::new(composition),
                    },
                )
            },
        );

        // ---- which composition a reopened project shows --------------------------------------
        // The defect the W-01 walkthrough found. Everything above happens in a project whose
        // first composition is the empty one the file came with, so before B-12d's rule of
        // thumb a save, close and reopen came back looking at that empty one and the work
        // looked lost.
        let folder = std::env::temp_dir().join("anime_compositor_b12d");
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("make the scratch directory");
        let drawings = drawings_in("layer1");
        run(&viewer, &import_of(&drawings));
        run(&viewer, "layer.create?asset=asset-1&name=Background");
        let file = folder.join("reopened.json");
        save_as(&viewer, &file);
        // A row rather than a panic. A project this test wrote that will not open again is a
        // line in the table the owner reads, not a stack trace: the hardening pass broke the
        // rule that a composition has a positive length and the schema caught it here, on the
        // way back through the disk, which ended the run before the table was written at all.
        let (showing, still_there) = match open(&file) {
            Ok(viewer) => {
                let reopened = Mutex::new(viewer);
                (on_screen(&reopened), comp_ids(&reopened))
            }
            Err(d) => (d.message.clone(), d.message),
        };
        report.check(
            "a saved project reopens looking at the composition with work in it",
            "comp-2 Second 1920x1080 at 24 fps, 48 frames",
            showing,
        );
        report.check(
            "and the empty compositions are still there, because none of them was thrown away",
            "comp-main, comp-1, comp-2",
            still_there,
        );

        // ---- the largest one this build will make, which must be allowed ---------------------
        // The limit rows above all check that something too large is refused. Without this one
        // the same rows pass a build that refuses everything, and the row a person would hit is
        // the one where a legal size is turned down for no reason they can see.
        report.check(
            "a composition exactly at the pixel budget is made rather than refused",
            "New composition Biggest, 16384x4096 at 24 fps, 240 frames. It is empty; import \
             drawings and add layers to fill it.",
            run(
                &viewer,
                "composition.create?name=Biggest&width=16384&height=4096",
            ),
        );

        // ---- getting back out of one ---------------------------------------------------------
        // The owner found this the first time they made a composition: making one moves the
        // window into it, and until `composition.open` existed there was no way back to the shot
        // they had been working on. Nothing in the window even named the other compositions.
        let depth = held(&viewer).document.undo_depth();
        report.check(
            "the window can be put back on a composition it left",
            "comp-main Main 1920x1080 at 24 fps, 24 frames",
            {
                run(&viewer, "composition.open?composition=comp-main");
                on_screen(&viewer)
            },
        );
        report.check(
            "and the transport is that composition's length, not the one it came from",
            24,
            held(&viewer).playback.length(),
        );
        report.check(
            "and looking somewhere else is not an edit, so there is nothing new to undo",
            depth,
            held(&viewer).document.undo_depth(),
        );
        report.check(
            "a composition that is not in the project is refused",
            "There is no composition comp-9 in this project.",
            run(&viewer, "composition.open?composition=comp-9"),
        );
        report.check(
            "and the window is still looking at the one it was",
            "comp-main Main 1920x1080 at 24 fps, 24 frames",
            on_screen(&viewer),
        );
        report.check(
            "asking with no composition named says what to do rather than moving the window",
            "Which composition should be opened? Choose one in the project panel.",
            run(&viewer, "composition.open"),
        );

        write_artifact(
            &report,
            "verification/B-12d_new_composition_table.md",
            "B-12d: making a composition",
            NEW_COMPOSITION_INTRO,
            NEW_COMPOSITION_NOTES,
        );
        let failed: Vec<&String> = report
            .rows
            .iter()
            .filter(|(_, e, a)| e != a)
            .map(|(c, _, _)| c)
            .collect();
        assert!(failed.is_empty(), "these checks failed: {failed:#?}");
    }

    const NEW_COMPOSITION_INTRO: &[&str] = &[
        "W-01 lists \"creates a composition\" third of its thirteen steps, and until B-12d this \
         build could not take that step. There was no `composition.create` in document 24 and \
         no command in the core behind it: the composition somebody worked in was whichever one \
         their project file already held, and `verification/B-12_acceptance_run.md` recorded \
         step 3 as **not built**. This table is that step, made to work and then checked.",
        "It is a new capability rather than a correction of an omission, which is written up \
         beside the new row in document 24. The owner may cut it; nothing else in the build \
         depends on it existing.",
    ];

    const NEW_COMPOSITION_NOTES: &[&str] = &[
        "## What to look at\n\n- **The size limits are this build's, not the \
         specification's.** Document 19 line 52 says a composition is \"bounded by \
         implementation safety limits\" and never says what they are. This build chose no side \
         past 16384, no more than 67108864 pixels in all, and no more than 10000 frames, and \
         the rows that check them are the only place those numbers are visible. They are \
         provisional and are the owner's to change.\n- **The two size limits are not one \
         limit, and the refusal says so.** 16384 a side and 67108864 pixels in all is 8192 by \
         8192, or 16384 by 4096, and not 16384 square, which is four times the pixel ceiling. \
         The owner read the first half of that sentence and asked for 16384 square on \
         2026-09-08, so the refusal now finishes the arithmetic and names the tallest height \
         the requested width allows, and the Biggest row above makes exactly that shape. \
         The ceiling is memory: document 21 works in float32 RGBA, sixteen bytes a pixel, so \
         67108864 pixels is a gibibyte for one layer buffer and several for a frame.\n- **A refusal is a sentence and changes \
         nothing.** Seven commands here are turned down, and the two rows after them check that \
         the project holds exactly what it held before and that the window is still looking at \
         the same composition. A control that springs back with nothing said is the failure \
         this guards against.\n- **Undo takes away the composition on screen.** That is the one \
         genuinely new way this command can break a window, because no other command could \
         remove what the viewer was pointed at. Undo and redo both put the window on the \
         composition the record touched, and fall back to the first composition when that one \
         has been taken away. Before that rule a redo said \"Redone\" and left the person \
         looking at a different composition, which is what the row for it caught.",
        "## What this does not cover\n\n**Which composition a reopened project shows is a rule \
         of thumb, not a restored setting.** Document 07's project format has no field for \
         which composition was open, so there is nothing on disk to come back to. This build \
         opens the first composition that has anything in it, and the first composition \
         otherwise. The last two rows check that rule, and they pass, but the rule is wrong for \
         anyone who deliberately leaves an empty composition ready to work in and expects to \
         find it. Fixing it properly means adding a field to the project format, which is a \
         schema change and the owner's decision.\n\n**There is no `project.new`.** Document 24 \
         names one on Ctrl+N and this build does not have it, so every composition here is made \
         inside a project that was opened from a file. Ctrl+Shift+N is what this build binds, \
         leaving Ctrl+N free for the command the table already promises.\n\n**The page.** Every \
         row calls the same function the window's URL scheme calls. That the New composition \
         button and its five fields send it, and that Ctrl+Shift+N reaches the button, are in \
         `verification/B-12b_page_table.md`, `verification/B-12c_keyboard_table.md` and the \
         photograph beside this table.",
    ];

    fn cell(text: &str) -> String {
        text.replace('|', r"\|")
    }
}

/// What the autosave timer and the recovery path do, checked without a window.
///
/// B-09's other half. The core already knows how to write a recovery snapshot and how to find
/// one; what could go wrong here is everything around that — a timer that writes too early or
/// never, a snapshot that quietly becomes the file Save writes to, a recovered project the
/// window calls saved. None of those would look like a failure at the time. They would look
/// like a person's afternoon disappearing later.
///
/// Writes `verification/B-09_recovery_table.md`.
#[cfg(test)]
mod recovery_and_autosave {
    use super::*;
    use anime_compositor::command::Command;

    fn repo(rel: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("the app crate has a parent directory")
            .join(rel)
    }

    /// A copy of a fixture in a scratch directory of its own, emptied first. A copy because
    /// autosave writes beside the project, and `Fixtures/` is not somewhere a test may write.
    fn a_project_to_work_on() -> PathBuf {
        let directory = std::env::temp_dir().join("anime_compositor_b09_recovery");
        let _ = std::fs::remove_dir_all(&directory);
        std::fs::create_dir_all(&directory).expect("make the scratch directory");
        let path = directory.join("shot.json");
        std::fs::copy(repo("Fixtures/projects/unknown_effect_project.json"), &path)
            .expect("copy the fixture");
        path
    }

    /// Rename the one layer, which is the smallest real change to a project this build can make.
    fn change_something(viewer: &mut Viewer, to: &str) {
        let composition = viewer.composition.clone();
        let layer_id = viewer.document.project().compositions[0].layer_order()[0].clone();
        viewer
            .document
            .apply(Command::RenameLayer {
                composition,
                layer_id,
                name: to.to_string(),
            })
            .expect("rename the layer");
    }

    fn layer_name(viewer: &Viewer) -> String {
        let composition = &viewer.document.project().compositions[0];
        composition
            .layer(&composition.layer_order()[0])
            .expect("the layer the order names")
            .name
            .clone()
    }

    #[test]
    fn nothing_is_written_early_nothing_is_lost_late() {
        let project = a_project_to_work_on();
        let mut rows: Vec<(String, String, String)> = Vec::new();
        let mut check = |what: &str, expected: &dyn ToString, actual: &dyn ToString| {
            rows.push((what.to_string(), expected.to_string(), actual.to_string()));
        };

        let untouched = std::fs::read_to_string(&project).expect("read the project");
        let viewer = Mutex::new(open(&project).expect("open the project"));
        let start = Instant::now();

        // ---- the timer ------------------------------------------------------------------------
        {
            let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
            check(
                "a project with nothing outstanding writes no snapshot, however long it sits",
                &"nothing",
                &autosave_tick(viewer, start + Duration::from_secs(3600))
                    .unwrap_or_else(|| "nothing".to_string()),
            );

            change_something(viewer, "Renamed while nobody was saving");
            check(
                "there is now unsaved work",
                &true,
                &viewer.document.is_dirty(),
            );
            check(
                "one minute of unsaved work is not enough (document 07 asks for two)",
                &"nothing",
                &autosave_tick(viewer, start + Duration::from_secs(60))
                    .unwrap_or_else(|| "nothing".to_string()),
            );
            check(
                "and no file has appeared beside the project",
                &false,
                &persist::autosave_path(&project, 0).exists(),
            );

            // Two minutes after the tick above, which is when the timer first saw the work: the
            // clock starts at the sight of it, not at the moment it was done.
            let said = autosave_tick(viewer, start + Duration::from_secs(181))
                .unwrap_or_else(|| "nothing".to_string());
            check(
                "after two minutes a snapshot is written, in the first free slot",
                &format!(
                    "Recovery snapshot written to {}",
                    persist::autosave_path(&project, 0).display()
                ),
                &said,
            );
            check(
                "the work is still unsaved afterwards (document 26)",
                &true,
                &viewer.document.is_dirty(),
            );
            check(
                "and the project file has not been touched (document 07)",
                &"unchanged",
                &if std::fs::read_to_string(&project).expect("read the project") == untouched {
                    "unchanged"
                } else {
                    "changed"
                },
            );
            check(
                "the snapshot keeps the effect this build does not understand",
                &true,
                &std::fs::read_to_string(persist::autosave_path(&project, 0))
                    .expect("read the snapshot")
                    .contains("vendor.future.effect"),
            );

            // Five more, each two minutes after the last. The sixth has to reuse a slot.
            for minute in 1..=5 {
                change_something(viewer, &format!("Renamed again, {minute}"));
                autosave_tick(viewer, start + Duration::from_secs(181 + 121 * minute))
                    .expect("a snapshot every two minutes");
            }
            check(
                "six snapshots leave five files, not six",
                &persist::AUTOSAVE_SLOTS,
                &persist::recovery_candidates(&project).len(),
            );
            check(
                "and the window is offering all five to recover from",
                &persist::AUTOSAVE_SLOTS,
                &viewer.recovery.len(),
            );

            // Every row above ticks at a moment that is itself the timer's first sight of the
            // work, and the clock starts at that sight — so they read zero and would pass for
            // any waiting time at all, including none. These three straddle the boundary
            // instead: the last tick of the loop above was at 181 + 121 * 5, and the work is
            // still unsaved, so the clock is running from there.
            let last = 181 + 121 * 5;
            check(
                "one look short of two minutes writes nothing",
                &"nothing",
                &autosave_tick(viewer, start + Duration::from_secs(last + 119))
                    .unwrap_or_else(|| "nothing".to_string()),
            );
            let said = autosave_tick(viewer, start + Duration::from_secs(last + 121))
                .unwrap_or_else(|| "nothing".to_string());
            check(
                "and one look past it writes a snapshot",
                &"a snapshot",
                &if said.starts_with("Recovery snapshot written to") {
                    "a snapshot"
                } else {
                    said.as_str()
                },
            );
            check(
                "and the two minutes start again there, so ten seconds later there is nothing",
                &"nothing",
                &autosave_tick(viewer, start + Duration::from_secs(last + 131))
                    .unwrap_or_else(|| "nothing".to_string()),
            );
        }

        // ---- recovering -------------------------------------------------------------------------
        let newest = persist::recovery_candidates(&project)[0].path.clone();
        let said = recover(&viewer, &newest);
        check(
            "recovering says which snapshot was opened",
            &format!("Recovered {}", newest.display()),
            &said,
        );
        {
            let viewer = viewer.lock().expect("the viewer lock was poisoned");
            check(
                "the recovered work is what was in the snapshot",
                &"Renamed again, 5",
                &layer_name(&viewer),
            );
            check(
                "Save would write to the project, not back into the snapshot",
                &project.display().to_string(),
                &viewer
                    .path
                    .as_ref()
                    .map_or(String::new(), |p| p.display().to_string()),
            );
            check(
                "and the window calls the project by its own name",
                &"shot.json",
                &viewer.name,
            );
            check(
                "the recovered work counts as unsaved, because the project file does not have it",
                &true,
                &viewer.document.is_dirty(),
            );
            // The note is the only thing on screen that says the project file has not been
            // written yet. Without it a recovered window is indistinguishable from a saved one.
            check(
                "and the window says on screen that this is a snapshot and the project is untouched",
                &true,
                &viewer.notes.iter().any(|note| {
                    note.contains(&newest.display().to_string())
                        && note.contains("Nothing has been written to")
                }),
            );
        }
        // Compared whole, reported as a word: the fixture is two hundred lines and a table cell
        // holding it twice is a table nobody reads.
        check(
            "recovering wrote nothing: the project file is still byte for byte what it was",
            &"unchanged",
            &if std::fs::read_to_string(&project).expect("read the project") == untouched {
                "unchanged"
            } else {
                "changed"
            },
        );

        // ---- and then saving it ------------------------------------------------------------------
        let said = save(&viewer);
        check(
            "saving after a recovery writes the project",
            &format!("Saved to {}", project.display()),
            &said,
        );
        check(
            "the project file now holds the recovered work",
            &true,
            &std::fs::read_to_string(&project)
                .expect("read the project")
                .contains("Renamed again, 5"),
        );
        {
            let viewer = viewer.lock().expect("the viewer lock was poisoned");
            check(
                "and there is nothing outstanding any more",
                &false,
                &viewer.document.is_dirty(),
            );
        }

        // ---- the case with nowhere to put anything -------------------------------------------------
        {
            let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
            viewer.path = None;
            change_something(viewer, "Changed with no file to save to");
            check(
                "a project with no file of its own writes no snapshot; there is nowhere beside it",
                &"nothing",
                &autosave_tick(viewer, start + Duration::from_secs(7200))
                    .unwrap_or_else(|| "nothing".to_string()),
            );
            // The tick above is the first sight of this change, so its clock reads zero and it
            // would write nothing wherever the project lived. This one is two minutes later,
            // which is the tick that would put the snapshot somewhere chosen for it.
            check(
                "and none two minutes later either, when there would otherwise be one",
                &"nothing",
                &autosave_tick(viewer, start + Duration::from_secs(7200 + 121))
                    .unwrap_or_else(|| "nothing".to_string()),
            );
        }
        check(
            "and it cannot be recovered into either",
            &"There is no project to recover into.",
            &recover(&viewer, &newest),
        );

        let scratch = project
            .parent()
            .expect("the project has a directory")
            .display()
            .to_string();
        write_artifact(&rows, &scratch);
        let failed: Vec<&(String, String, String)> =
            rows.iter().filter(|(_, e, a)| e != a).collect();
        assert!(
            failed.is_empty(),
            "{} of {} checks failed, see verification/B-09_recovery_table.md: {:#?}",
            failed.len(),
            rows.len(),
            failed
        );
    }

    fn write_artifact(rows: &[(String, String, String)], scratch: &str) {
        let passed = rows.iter().filter(|(_, e, a)| e == a).count();
        let mut out = String::new();
        out.push_str("# B-09, autosave and recovery in the window\n\n");
        out.push_str(
            "The core has known how to write a recovery snapshot for some time; until now nothing \
             called it. This is the window's half — a clock that decides when, and a way back in \
             from a snapshot. Produced by `cargo test -p anime_compositor_app`, from \
             `app/src/main.rs`.\n\n",
        );
        out.push_str(
            "The promise is document 07's, in two parts. **A snapshot is written after two \
             minutes of unsaved work, and it never overwrites the last manual save.** And **\
             recovering from a snapshot does not save it**: the window comes back pointing at the \
             project file with the recovered work outstanding, so the person decides whether it \
             becomes the project. The two minutes are a parameter here rather than a wait, which \
             is the only thing about this table that is not what the window does.\n\n",
        );
        out.push_str("| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
        for (check, expected, actual) in rows {
            let short = |text: &str| {
                text.replace(scratch, "<a temporary directory>")
                    .replace('|', r"\|")
            };
            out.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                check,
                short(expected),
                short(actual),
                if expected == actual { "pass" } else { "FAIL" }
            ));
        }
        out.push_str(&format!(
            "\n**{} of {} checks pass.**\n",
            passed,
            rows.len()
        ));
        out.push_str(
            "\n## What this does not cover\n\nThe two minutes passing. The timer is a thread that \
             sleeps ten seconds at a time and asks the same question this table asks; what is \
             checked here is the question, with the clock supplied. A thread that never started \
             would not fail this table, and only the running window shows that it did.\n\n\
             **Nothing in this window makes a project dirty yet.** It is a viewer: it opens, \
             shows, plays and saves, and no control in it changes a project. So in ordinary use \
             today the timer has nothing to write, and it stays quiet. What it protects is the \
             editing that B-13 onwards adds, and it is built now because the alternative is \
             building it after the first afternoon somebody loses.\n\n\
             Where a row says *a temporary directory*, the real value was this machine's scratch \
             directory, which differs on every machine and every run.\n",
        );
        std::fs::write(repo("verification/B-09_recovery_table.md"), out)
            .expect("write the artifact");
    }
}

/// What the page is handed when it asks for a frame, checked without a page.
///
/// The renderer's picture is checked pixel by pixel elsewhere, and the playback clock is checked
/// in `verification/B-08_preview_table.md`. What is new here is the transport between them: which
/// frame comes back for a given request, how many bytes the picture is, whether the page is
/// allowed to read the answer at all, and whether everything the window has to *say* about the
/// frame - its number, its resolution, the project's name, the notes, the dirty flag - arrives
/// beside the same picture it describes.
///
/// The window itself cannot be started in a test, so `serve` is called directly with the same
/// arguments the request handler passes it. What that leaves out is named at the end of the table.
///
/// Writes `verification/B-08_shell_table.md`.
#[cfg(test)]
mod serving {
    use super::*;
    use anime_compositor::command::Command;

    fn repo(rel: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("the app crate has a parent directory")
            .join(rel)
    }

    /// One header as text, or the empty string if it is not there at all. A header the page cannot
    /// read is the same to the page as a header that was never sent, so both read as absent here.
    fn header(response: &Response<Vec<u8>>, name: &str) -> String {
        response.headers().get(name).map_or_else(String::new, |v| {
            v.to_str().unwrap_or("<not readable as text>").to_string()
        })
    }

    /// Percent-decoding, written out here rather than borrowed from the window.
    ///
    /// The window encodes; this decodes. Using the window's own `from_a_query` would check one
    /// function against itself and would pass just as happily if both halves were wrong together.
    fn decode(text: &str) -> String {
        let bytes = text.as_bytes();
        let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
        let mut at = 0;
        while at < bytes.len() {
            if bytes[at] == b'%' && at + 3 <= bytes.len() {
                let hex = std::str::from_utf8(&bytes[at + 1..at + 3]).expect("ascii");
                out.push(u8::from_str_radix(hex, 16).expect("two hex digits"));
                at += 3;
            } else {
                out.push(bytes[at]);
                at += 1;
            }
        }
        String::from_utf8(out).expect("what went in was utf-8")
    }

    /// A project of sixteen pixels whose answer is known before anything renders it.
    ///
    /// Every pixel in the reference shot is either opaque or empty, and those two look the same
    /// whichever way alpha is carried — so the shot cannot tell straight alpha and premultiplied
    /// apart, and the page draws straight. This cel is four pixels of half-transparent red, which
    /// can: carried straight it stays 255 red, carried premultiplied it comes back at about 188.
    fn a_half_transparent_red_project() -> PathBuf {
        let directory = std::env::temp_dir().join("anime_compositor_b08_shell");
        let _ = std::fs::remove_dir_all(&directory);
        std::fs::create_dir_all(directory.join("media")).expect("make the scratch directory");
        let samples: Vec<u8> = std::iter::repeat_n([255u8, 0, 0, 128], 16)
            .flatten()
            .collect();
        anime_compositor::png_out::write_rgba(
            &directory.join("media/cel_0001.png"),
            4,
            4,
            OutputDepth::Eight,
            &[],
            &samples,
        )
        .expect("write the cel");
        let project = directory.join("shot.json");
        std::fs::write(
            &project,
            r#"{
  "schema_version": 0,
  "project_id": "proj-one-pixel",
  "color_settings": { "working_space": "linear-srgb", "alpha_mode": "premultiplied" },
  "assets": [
    {
      "id": "asset-cel",
      "kind": "image_sequence",
      "name": "Cel",
      "pattern": "cel_####.png",
      "frames": { "1": "media/cel_0001.png" },
      "interpretation": { "color_space": "srgb", "alpha": "straight" }
    }
  ],
  "compositions": [
    {
      "id": "comp-main",
      "name": "Main",
      "width": 4,
      "height": 4,
      "pixel_aspect_ratio": 1,
      "frame_rate": { "numerator": 24, "denominator": 1 },
      "start_frame": 0,
      "duration_frames": 1,
      "work_area": { "start_frame": 0, "end_frame_exclusive": 1 },
      "layer_order": ["layer-cel"],
      "layers": [
        {
          "id": "layer-cel",
          "kind": "raster",
          "name": "Cel",
          "asset_id": "asset-cel",
          "enabled": true,
          "locked": false,
          "in_frame": 0,
          "out_frame": 1,
          "source_offset_frames": 0,
          "transform": {
            "anchor": { "base": [0, 0], "keyframes": [] },
            "position": { "base": [0, 0], "keyframes": [] },
            "scale": { "base": [100, 100], "keyframes": [] },
            "rotation": { "base": 0, "keyframes": [] },
            "opacity": { "base": 1, "keyframes": [] }
          },
          "exposure_spans": [
            { "start_frame": 0, "end_frame_exclusive": 1, "drawing_number": 1 }
          ],
          "mask": null,
          "matte": null,
          "blend_mode": "normal",
          "effects": []
        }
      ]
    }
  ]
}
"#,
        )
        .expect("write the project");
        project
    }

    /// Ask the transport for something, the way the request handler does.
    fn ask(
        viewer: &Mutex<Viewer>,
        export: &Mutex<Export>,
        path: &str,
        query: Option<&str>,
    ) -> Response<Vec<u8>> {
        let (ask, quality) = parse(path, query).expect("the test asks for something readable");
        serve(viewer, export, ask, quality)
    }

    /// One document 24 command, by the identifier the page would send.
    fn run(viewer: &Mutex<Viewer>, what: &str) -> String {
        let (id, query) = match what.split_once('?') {
            Some((id, query)) => (id, Some(query)),
            None => (what, None),
        };
        edit_command(viewer, id, query).expect("a command document 24 lists")
    }

    fn held(viewer: &Mutex<Viewer>) -> std::sync::MutexGuard<'_, Viewer> {
        viewer.lock().expect("the viewer lock was poisoned")
    }

    /// A directory of this test's own, emptied first so a previous run cannot make a later one
    /// pass.
    fn scratch(name: &str) -> PathBuf {
        let directory = std::env::temp_dir().join(name);
        let _ = std::fs::remove_dir_all(&directory);
        std::fs::create_dir_all(&directory).expect("make the scratch directory");
        directory
    }

    #[test]
    fn what_the_page_gets_is_the_frame_it_asked_for() {
        let mut rows: Vec<(String, String, String)> = Vec::new();
        let mut check = |what: &str, expected: &dyn ToString, actual: &dyn ToString| {
            rows.push((what.to_string(), expected.to_string(), actual.to_string()));
        };

        let viewer = Mutex::new(demo());
        let export = Mutex::new(Export::default());

        // ---- one frame, asked for by number ------------------------------------------------------
        // The reference shot is 240 frames of 1920 by 1080 at 24 frames a second, starting at 0,
        // and D-33 makes the preview open at draft resolution, which DRAFT_DIVISOR puts at a
        // quarter of each side: 480 by 270.
        let got = ask(&viewer, &export, "/frame/0", None);
        check(
            "asking for frame 0 is answered",
            &200,
            &got.status().as_u16(),
        );
        check(
            "with raw pixels rather than an image file, which is what the page draws",
            &"application/octet-stream",
            &header(&got, "content-type"),
        );
        check(
            "and the answer says which frame it is",
            &"0",
            &header(&got, "x-frame"),
        );
        check(
            "at draft resolution, which is what the preview opens at",
            &"Draft",
            &header(&got, "x-quality"),
        );
        check(
            "and says so, so nobody mistakes it for what an export would write",
            &"true",
            &header(&got, "x-differs"),
        );
        check("480 wide", &"480", &header(&got, "x-width"));
        check("270 high", &"270", &header(&got, "x-height"));
        check(
            "and the picture is exactly that many pixels, four bytes each",
            &(480 * 270 * 4),
            &got.body().len(),
        );
        // Without both of these the page fetches the answer successfully and is then refused it:
        // the picture fails and every header reads as absent. Either one alone is silence.
        check(
            "the page is allowed to read the answer",
            &"*",
            &header(&got, "access-control-allow-origin"),
        );
        check(
            "and allowed to read the headers beside it, not only receive them",
            &"*",
            &header(&got, "access-control-expose-headers"),
        );
        check(
            "a frame asked for by number did not come from the clock, so there is no playback \
             report beside it",
            &"",
            &header(&got, "x-report"),
        );

        // ---- stepping stops at the ends of the work area ------------------------------------------
        let got = ask(&viewer, &export, "/frame/999", None);
        check(
            "stepping past the end of the shot stops at the last frame",
            &"239",
            &header(&got, "x-frame"),
        );
        let got = ask(&viewer, &export, "/frame/-5", None);
        check(
            "and stepping before the beginning stops at the first",
            &"0",
            &header(&got, "x-frame"),
        );

        // ---- and asked for by the clock ----------------------------------------------------------
        let got = ask(&viewer, &export, "/at/0", None);
        check(
            "the first frame of a playback is the one at the start of the shot",
            &"0",
            &header(&got, "x-frame"),
        );
        check(
            "and this time the playback report is beside it",
            &"Played 1 frames in real time. No frames were dropped.",
            &header(&got, "x-report"),
        );
        // One second at 24 frames a second is frame 24. The 23 in between were never asked for,
        // and D-32 requires that to be said rather than hidden.
        let got = ask(&viewer, &export, "/at/1000", None);
        check(
            "a second into the shot the clock asks for frame 24, not the next one along",
            &"24",
            &header(&got, "x-frame"),
        );
        check(
            "and says the 23 frames in between were passed over",
            &"23",
            &header(&got, "x-skipped"),
        );
        // A page whose clock jumps backwards is a page with a bug, and the answer to that is to
        // hold still, not to invent a negative skip count and put it in front of the owner.
        let got = ask(&viewer, &export, "/at/500", None);
        check(
            "time running backwards holds the frame rather than rewinding",
            &"24",
            &header(&got, "x-frame"),
        );
        check(
            "and counts nothing as skipped",
            &"0",
            &header(&got, "x-skipped"),
        );

        // ---- the resolution the page asks for -----------------------------------------------------
        let got = ask(&viewer, &export, "/frame/0", Some("q=full"));
        check(
            "asking for full resolution gets it",
            &"Full",
            &header(&got, "x-quality"),
        );
        check("at the shot's own size", &"1920", &header(&got, "x-width"));
        check(
            "and full resolution is what an export would write, so the warning goes away",
            &"false",
            &header(&got, "x-differs"),
        );
        // A typo in a query string must not quietly change what the person is looking at.
        let got = ask(&viewer, &export, "/frame/0", Some("q=sideways"));
        check(
            "a resolution nobody recognises changes nothing rather than guessing",
            &"Full",
            &header(&got, "x-quality"),
        );
        let got = ask(&viewer, &export, "/frame/0", Some("q=draft"));
        check(
            "and asking for draft again gets draft",
            &"Draft",
            &header(&got, "x-quality"),
        );

        // ---- what the window has to say about the project ------------------------------------------
        {
            let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
            held.name = "背景_日本語.json".to_string();
            held.notes = vec!["One note.".to_string(), "And another.".to_string()];
            held.status = "Saved to 背景_日本語.json".to_string();
        }
        let got = ask(&viewer, &export, "/frame/0", None);
        check(
            "a Japanese project name arrives at the page as the name it started as",
            &"背景_日本語.json",
            &decode(&header(&got, "x-project")),
        );
        check(
            "and so does a sentence with Japanese in it",
            &"Saved to 背景_日本語.json",
            &decode(&header(&got, "x-status")),
        );
        check(
            "the notes travel whole, one to a line",
            &"One note.\nAnd another.",
            &decode(&header(&got, "x-notes")),
        );
        check(
            "nothing is being exported, and the page is told so",
            &"false",
            &header(&got, "x-exporting"),
        );
        check(
            "there is nothing outstanding in the project yet",
            &"false",
            &header(&got, "x-dirty"),
        );
        {
            let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
            let composition = held.composition.clone();
            let layer_id = held.document.project().compositions[0].layer_order()[0].clone();
            held.document
                .apply(Command::RenameLayer {
                    composition,
                    layer_id,
                    name: "Renamed while the page watched".to_string(),
                })
                .expect("rename the layer");
        }
        let got = ask(&viewer, &export, "/frame/0", None);
        check(
            "and once something is changed the page is told that too",
            &"true",
            &header(&got, "x-dirty"),
        );

        // ---- a request the window does not understand ------------------------------------------------
        check(
            "a path the window does not understand is not answered with a guess",
            &"nothing",
            &parse("/frames/3", None).map_or("nothing", |_| "something"),
        );
        check(
            "and neither is a time that is not a number",
            &"nothing",
            &parse("/at/soon", None).map_or("nothing", |_| "something"),
        );

        // ---- a frame that cannot be made --------------------------------------------------------------
        // Document 28: a frame that cannot be drawn is reported in words, never replaced by
        // something that looks like a frame.
        let broken = Mutex::new(
            open(&repo("Fixtures/projects/missing_media_project.json"))
                .expect("open the project whose drawing is missing"),
        );
        let got = ask(&broken, &export, "/frame/0", None);
        check(
            "a missing drawing does not take the rest of the shot down with it: the frame still \
             comes back",
            &200,
            &got.status().as_u16(),
        );
        check(
            "and it is honestly empty rather than filled in with a guess",
            &"every pixel transparent",
            &if got.body().iter().all(|byte| *byte == 0) {
                "every pixel transparent"
            } else {
                "something was drawn"
            },
        );
        check(
            "and the page is carrying the sentence that explains why it is empty",
            &"2 of the files for \"Cel\" are not where the project expects them. The reference is \
              kept as it is. Relink the asset to point it at the files, or put them back. Frames \
              that cannot be found render as nothing rather than as a guess.",
            &decode(&header(&got, "x-notes")),
        );
        check(
            "which the page can only read because the refusal carries the same permission the \
             picture does",
            &"*",
            &header(&got, "access-control-expose-headers"),
        );

        // ---- the pixels themselves ------------------------------------------------------------------
        // Everything above counts the bytes without reading one. The page draws these straight
        // into an ImageData, which is straight alpha and nothing else, so a picture handed over
        // premultiplied would be the right size, the right frame and the wrong colour.
        let known = Mutex::new(
            open(&a_half_transparent_red_project()).expect("open the four-pixel project"),
        );
        let got = ask(&known, &export, "/frame/0", Some("q=full"));
        check(
            "a four by four picture is sixty-four bytes",
            &64,
            &got.body().len(),
        );
        check(
            "and a half-transparent red one comes back as the red it was drawn as, not the \
             darker red premultiplying it would give",
            &"255, 0, 0, 128",
            &got.body()[..4]
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        );
        check(
            "and every pixel of it is that same red",
            &true,
            &got.body().chunks_exact(4).all(|px| *px == got.body()[..4]),
        );

        write_artifact(&rows);
        let failed: Vec<&(String, String, String)> =
            rows.iter().filter(|(_, e, a)| e != a).collect();
        assert!(
            failed.is_empty(),
            "{} of {} checks failed, see verification/B-08_shell_table.md: {:#?}",
            failed.len(),
            rows.len(),
            failed
        );
    }

    /// B-12a item 7: the two ways of looking at a frame, and the promise that neither changes it.
    ///
    /// Writes `verification/B-12a_inspect_table.md`.
    #[test]
    fn looking_at_the_alpha_channel_changes_nothing_but_the_looking() {
        let mut rows: Vec<(String, String, String)> = Vec::new();
        let mut check = |what: &str, expected: &dyn ToString, actual: &dyn ToString| {
            rows.push((what.to_string(), expected.to_string(), actual.to_string()));
        };
        let pixel = |body: &[u8]| {
            body[..4]
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        };

        let project = a_half_transparent_red_project();
        let viewer = Mutex::new(open(&project).expect("open the four-pixel project"));
        let export = Mutex::new(Export::default());

        let colour = ask(&viewer, &export, "/frame/0", Some("q=full"));
        check(
            "the window opens looking at the picture itself",
            &"false",
            &header(&colour, "x-alpha"),
        );
        check(
            "half-transparent red arrives as the red it was drawn as",
            &"255, 0, 0, 128",
            &pixel(colour.body()),
        );
        check(
            "and the transparency grid starts on, because a transparent frame that reads as \
             black is a frame somebody misjudges",
            &"true",
            &header(&colour, "x-checkerboard"),
        );

        // ---- the alpha-only view -------------------------------------------------------------
        check(
            "turning alpha inspection on says what will be on screen and what will not change",
            &"Alpha-only inspection is on. The picture is the alpha channel, white where the \
              frame is opaque and black where it is empty; what is exported is unchanged.",
            &run(&viewer, "viewer.toggle_alpha"),
        );
        let alpha = ask(&viewer, &export, "/frame/0", Some("q=full"));
        check(
            "and the frame that comes back says it is being looked at that way",
            &"true",
            &header(&alpha, "x-alpha"),
        );
        check(
            "a pixel that is half transparent is drawn as the grey half way up",
            &"128, 128, 128, 255",
            &pixel(alpha.body()),
        );
        check(
            "the alpha view is opaque, so what is being measured cannot itself be see-through",
            &true,
            &alpha.body().chunks_exact(4).all(|px| px[3] == 255),
        );
        check(
            "it is the same picture, at the same size",
            &format!(
                "{}x{}",
                header(&colour, "x-width"),
                header(&colour, "x-height")
            ),
            &format!(
                "{}x{}",
                header(&alpha, "x-width"),
                header(&alpha, "x-height")
            ),
        );

        // Document 21 line 97: presentation must not alter cached final pixels. The frame was
        // just rendered and cached while the alpha view was on; asking for it again with the
        // view off is what proves the cache was given the picture and not the grey.
        check(
            "turning it off says so",
            &"Alpha-only inspection is off.",
            &run(&viewer, "viewer.toggle_alpha"),
        );
        let again = ask(&viewer, &export, "/frame/0", Some("q=full"));
        check(
            "and the frame comes back byte for byte as it was, so what the cache kept was the \
             picture and never the view",
            &true,
            &(again.body() == colour.body()),
        );

        // ---- neither is a change to the project -----------------------------------------------
        check(
            "looking at the alpha channel is not unsaved work",
            &"false",
            &header(&again, "x-dirty"),
        );
        check(
            "and it is not in the undo history either, which is what document 24's own table \
             says: undoable, no",
            &0,
            &held(&viewer).document.undo_depth(),
        );

        // ---- the grid ---------------------------------------------------------------------------
        check(
            "turning the grid off says what it was",
            &"The transparency grid is off.",
            &run(&viewer, "viewer.toggle_checkerboard"),
        );
        let plain = ask(&viewer, &export, "/frame/0", Some("q=full"));
        check(
            "the frame says the grid is off",
            &"false",
            &header(&plain, "x-checkerboard"),
        );
        check(
            "and not one byte of the frame is different, because the grid was never in it",
            &true,
            &(plain.body() == colour.body()),
        );
        check(
            "turning it back on says so too",
            &"The transparency grid is on. It is drawn behind the frame and is not part of it.",
            &run(&viewer, "viewer.toggle_checkerboard"),
        );

        // ---- R-10: inspection must not alter export ---------------------------------------------
        // The comparison is two exports of the same frame, one taken while the alpha view is on,
        // rather than a claim about what an export ought to contain. Byte for byte, or the view
        // reached the file.
        let ordinary = scratch("anime_compositor_b12a_export_plain");
        let (p, root, request) = export_job(&held(&viewer), &ordinary, MissingSource::Block);
        run_export(&p, &root, &request, &AtomicBool::new(false));
        run(&viewer, "viewer.toggle_alpha");
        let inspecting = scratch("anime_compositor_b12a_export_alpha");
        let (p, root, request) = export_job(&held(&viewer), &inspecting, MissingSource::Block);
        run_export(&p, &root, &request, &AtomicBool::new(false));
        let read = |dir: &Path| {
            let mut files: Vec<PathBuf> = std::fs::read_dir(dir)
                .expect("read the export directory")
                .filter_map(|e| e.ok().map(|e| e.path()))
                .collect();
            files.sort();
            files
                .iter()
                .map(|f| std::fs::read(f).expect("read an exported frame"))
                .collect::<Vec<_>>()
        };
        let written = read(&ordinary);
        check(
            "an export writes the frames of the work area",
            &1,
            &written.len(),
        );
        check(
            "and exporting while looking at the alpha channel writes the same file, byte for \
             byte, which is what R-10 asks for",
            &true,
            &(read(&inspecting) == written),
        );
        run(&viewer, "viewer.toggle_alpha");

        write_inspect_artifact(&rows);
        let failed: Vec<&(String, String, String)> =
            rows.iter().filter(|(_, e, a)| e != a).collect();
        assert!(
            failed.is_empty(),
            "{} of {} checks failed, see verification/B-12a_inspect_table.md: {:#?}",
            failed.len(),
            rows.len(),
            failed
        );
    }

    fn write_inspect_artifact(rows: &[(String, String, String)]) {
        let passed = rows.iter().filter(|(_, e, a)| e == a).count();
        let mut out = String::new();
        out.push_str("# B-12a: looking at the alpha channel, and the grid behind the frame\n\n");
        out.push_str(
            "W-01 ends with the artist inspecting alpha, and document 05 line 31 lists the two \
             views this is about: the transparency grid, and the alpha-only display. Both are in \
             the viewer now, as document 24's `viewer.toggle_checkerboard` and \
             `viewer.toggle_alpha`. Produced by `cargo test -p anime_compositor_app`, from \
             `app/src/main.rs`.\n\n",
        );
        out.push_str(
            "The promise these rows are about is document 21 line 97's, which R-10 states as a \
             requirement: **checkerboard and alpha-only inspection must not alter export.** A \
             view that quietly leaked into the file, or into the frames the preview keeps, would \
             be found by whoever opened the exported sequence, long after the person who turned \
             it on had forgotten it was on. So the grid is drawn by the page behind a canvas that \
             already carries transparency and touches no pixel at all, and the alpha view is \
             applied to the copy of the frame that is on its way to the screen, after the cache \
             has been handed the picture it keeps.\n\n",
        );
        out.push_str(
            "The fixture is four pixels of half-transparent red, drawn by this test. The \
             reference shot cannot be used for this: every pixel in it is either opaque or empty, \
             so an alpha view of it that was subtly wrong would still look right.\n\n",
        );
        out.push_str("| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
        for (check, expected, actual) in rows {
            out.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                check,
                expected.replace('|', r"\|"),
                actual.replace('|', r"\|"),
                if expected == actual { "pass" } else { "FAIL" }
            ));
        }
        out.push_str(&format!(
            "\n**{} of {} checks pass.**\n",
            passed,
            rows.len()
        ));
        out.push_str(
            "\n## What this does not cover\n\nWhat the grid looks like. It is eight-pixel squares \
             of two greys in the page's stylesheet, fixed to the screen rather than to the \
             picture so that zooming does not stretch them, and no test can see it. The \
             photographs of the window are where it is judged.\n\nZoom and pan, which document 05 \
             lists in the same line: since W-06 the page zooms with the wheel and scrolls the \
             stage around it, and the grid, fixed to the screen, is what says the picture and \
             not the window was zoomed. No test can see that either.\n",
        );
        std::fs::write(repo("verification/B-12a_inspect_table.md"), out).expect("write it");
    }

    fn write_artifact(rows: &[(String, String, String)]) {
        let passed = rows.iter().filter(|(_, e, a)| e == a).count();
        let checkout = repo("").display().to_string();
        let mut out = String::new();
        out.push_str("# B-08, what the page is handed when it asks for a frame\n\n");
        out.push_str(
            "Between the renderer and the picture on screen there is a short journey: the page \
             asks for a frame over a local address, and gets back raw pixels with everything the \
             window has to say about them in the headers beside. Nothing else in this project \
             looks at that journey. Produced by `cargo test -p anime_compositor_app`, from \
             `app/src/main.rs`.\n\n",
        );
        out.push_str(
            "The promise is that **the picture and the words about it always came from the same \
             render**. The frame number, the resolution, the project's name, the notes and the \
             unsaved-work flag are attached to the picture they describe, so the number on screen \
             can never belong to a different drawing than the one under it.\n\n",
        );
        out.push_str("| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
        for (check, expected, actual) in rows {
            let short = |text: &str| {
                text.replace(&checkout, "<the checkout>")
                    .replace('|', r"\|")
                    .replace('\n', "⏎")
            };
            out.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                check,
                short(expected),
                short(actual),
                if expected == actual { "pass" } else { "FAIL" }
            ));
        }
        out.push_str(&format!(
            "\n**{} of {} checks pass.**\n",
            passed,
            rows.len()
        ));
        out.push_str(
            "\n## What this does not cover\n\nThe window itself. These rows call the same function \
             the request handler calls, with the same arguments, but no window is opened and no \
             page is loaded, so a build that never registered the handler at all would pass this \
             table. The photographs in `verification/B-08_window_shell.md` are what show that the \
             window exists and draws; this table is what shows that what it draws is described \
             correctly.\n\n\
             Nor does it cover whether the *picture* is right. Whether frame 24 looks like the \
             shot is H-01's question and B-08a's; the question here is only whether the frame \
             that comes back is the frame that was asked for, whether it is the size it says it \
             is, and whether its colours arrive the way the page draws them. The last of those \
             needs a picture the shot cannot supply, so the four-pixel rows draw their own.\n\n\
             Where a row says *the checkout*, the real value was this machine's copy of the \
             repository, whose path differs on every machine. A line break inside a value is \
             shown as ⏎ so that one row stays one row.\n",
        );
        std::fs::write(repo("verification/B-08_shell_table.md"), out).expect("write the artifact");
    }
}

/// What exporting from the window does, checked without a window.
///
/// The renderer's export path is already checked to the frame in `T-08_export_table.md` and to
/// the whole shot in `B-10_full_shot_table.md`. What is new here, and what nothing else can see,
/// is the part between the button and that path: which frames the window asks for, what it names
/// them, whether the project it writes is the one that was open when the person asked, and
/// whether a refusal, a cancellation and a failure each reach the person in words.
///
/// Writes `verification/B-10_export_table.md`.
#[cfg(test)]
mod exporting {
    use super::*;
    use anime_compositor::command::Command;

    fn repo(rel: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("the app crate has a parent directory")
            .join(rel)
    }

    /// A scratch directory of this test's own, emptied first so a previous run's frames cannot
    /// make a later one pass.
    fn scratch(name: &str) -> PathBuf {
        let directory = std::env::temp_dir().join(name);
        let _ = std::fs::remove_dir_all(&directory);
        std::fs::create_dir_all(&directory).expect("make the scratch directory");
        directory
    }

    /// The PNG files in a directory, in name order.
    fn written(dir: &Path) -> Vec<String> {
        let mut names: Vec<String> = std::fs::read_dir(dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().into_owned())
                    .collect()
            })
            .unwrap_or_default();
        names.sort();
        names
    }

    /// The width and height a PNG declares, read out of its IHDR rather than trusted from the
    /// request: the question is what is on the disk.
    fn size_of(png: &Path) -> String {
        let bytes = std::fs::read(png).expect("read the frame that was written");
        let number = |at: usize| u32::from_be_bytes(bytes[at..at + 4].try_into().expect("four"));
        format!("{}x{}", number(16), number(20))
    }

    #[test]
    fn what_the_window_exports_is_the_shot_it_had() {
        let mut rows: Vec<(String, String, String)> = Vec::new();
        let mut check = |check: &str, expected: &dyn std::fmt::Display, actual: &dyn ToString| {
            rows.push((check.to_string(), expected.to_string(), actual.to_string()));
        };

        // A row about a sentence the person reads records the words themselves rather than a
        // bare true, which tells a reader nothing. The phrase asked for is the Expected column;
        // if it was in what the window said, that is the Actual column too, and if it was not,
        // the Actual column is everything the window did say instead. The core owns the exact
        // wording of its diagnostics and is checked on it elsewhere — pinning every word twice
        // would make an improvement to a message look like a regression here.
        let says = |phrase: &str, said: &str| {
            if said.contains(phrase) {
                phrase.to_string()
            } else {
                format!("the window said: {said}")
            }
        };

        // The project the window opens on: the reference shot, which has a deliberate gap —
        // layer 3 has no drawing 7, so frames 14 and 15 of every cycle ask for a drawing that is
        // not there. That gap is the whole reason document 07 blocks an export by default, and it
        // is the only fixture in this repository that can show the block happening.
        let mut viewer = demo();
        let into = scratch("anime_compositor_b10_export");

        // ---- what the window asks for -----------------------------------------------------------
        let (snapshot, root, request) = export_job(&viewer, &into, MissingSource::Block);
        check(
            "the range offered is the work area, first to last frame inclusive",
            &"0 to 239",
            &format!("{} to {}", request.first_frame, request.last_frame),
        );
        check(
            "the files are named for the project and carry the frame number, four digits wide",
            &"the reference shot_%04d.png",
            &request.naming,
        );
        check(
            "at eight bits with straight alpha, which is what the export fixtures were written \
             with",
            &"Eight, Straight",
            &format!("{:?}, {:?}", request.depth, request.alpha),
        );

        // ---- the snapshot -------------------------------------------------------------------------
        // B-10 asks for an *immutable* export snapshot. The check is that editing the open
        // document after the job was made does not reach the job: what is written is the shot as
        // it was when the person asked.
        let composition = viewer.composition.clone();
        let layer_id = viewer.document.project().compositions[0].layer_order()[0].clone();
        let before = snapshot.compositions[0]
            .layer(&layer_id)
            .expect("the layer the order names")
            .name
            .clone();
        viewer
            .document
            .apply(Command::RenameLayer {
                composition,
                layer_id: layer_id.clone(),
                name: "Renamed while the export was running".to_string(),
            })
            .expect("rename the layer");
        check(
            "the open project has been changed since the export was asked for",
            &"Renamed while the export was running",
            &viewer.document.project().compositions[0]
                .layer(&layer_id)
                .expect("the layer the order names")
                .name,
        );
        check(
            "and the export is still writing the project as it was when it was asked for",
            &before,
            &snapshot.compositions[0]
                .layer(&layer_id)
                .expect("the layer the order names")
                .name,
        );

        // ---- a missing drawing, with document 07's default ---------------------------------------
        // Two frames rather than 240: which frames are asked for is checked above, and what a
        // whole shot does is `B-10_full_shot_table.md`. Frames 14 and 15 are the two the gap
        // falls on.
        let short = |missing| {
            let (_, _, mut request) = export_job(&viewer, &into, missing);
            request.first_frame = 14;
            request.last_frame = 15;
            request
        };
        let blocked = short(MissingSource::Block);
        let said = run_export(&snapshot, &root, &blocked, &AtomicBool::new(false));
        check(
            "by default a frame whose drawing is missing stops the export before anything is \
             written",
            &"Nothing was exported.",
            &says("Nothing was exported.", &said),
        );
        check(
            "and the person is told how many frames it is",
            &"2 of the 2 frames asked for have a drawing that is missing",
            &says(
                "2 of the 2 frames asked for have a drawing that is missing",
                &said,
            ),
        );
        check(
            "and what to do about it",
            &"Relink or restore the missing drawings",
            &says("Relink or restore the missing drawings", &said),
        );
        check(
            "nothing at all is on the disk",
            &"[]",
            &format!("{:?}", written(&into)),
        );

        // ---- the same two frames, written on purpose ----------------------------------------------
        let anyway = short(MissingSource::RenderTransparent);
        let said = run_export(&snapshot, &root, &anyway, &AtomicBool::new(false));
        check(
            "asked to write them anyway, the window writes them and says where",
            &format!("Exported 2 frames into {}.", into.display()),
            &says(
                &format!("Exported 2 frames into {}.", into.display()),
                &said,
            ),
        );
        check(
            "the two files are named for the frames that were asked for",
            &"[\"the reference shot_0014.png\", \"the reference shot_0015.png\"]",
            &format!("{:?}", written(&into)),
        );
        check(
            "an exported frame is full size, whatever resolution the preview was showing",
            &"1920x1080",
            &size_of(&into.join("the reference shot_0014.png")),
        );
        check(
            "and the drawing that is missing is still reported rather than passed over in silence",
            &"Frame 14 exposes drawing 7 of layer3_%03d.png, which is missing.",
            &says(
                "Frame 14 exposes drawing 7 of layer3_%03d.png, which is missing.",
                &said,
            ),
        );
        check(
            "once for each frame it was missing on, not once for the export",
            &"Frame 15 exposes drawing 7 of layer3_%03d.png, which is missing.",
            &says(
                "Frame 15 exposes drawing 7 of layer3_%03d.png, which is missing.",
                &said,
            ),
        );

        // ---- cancelling ----------------------------------------------------------------------------
        // R-09 asks for cancellation between frames. Asked before the first one, the answer is
        // that nothing was written and the job does not claim to have succeeded.
        let elsewhere = scratch("anime_compositor_b10_cancel");
        let mut cancelled = short(MissingSource::RenderTransparent);
        cancelled.output_dir = elsewhere.clone();
        let said = run_export(&snapshot, &root, &cancelled, &AtomicBool::new(true));
        check(
            "a cancelled export says how far it got, and does not claim to have succeeded",
            &"Export stopped at your request after 0 of 2 frames",
            &says("Export stopped at your request after 0 of 2 frames", &said),
        );
        // The sentence above is the core's diagnostic. This one is the window's own opening line,
        // and it is checked separately because the two can disagree: a window that says "Exported
        // 0 frames" and then reports a cancellation underneath has told the person two different
        // things about the same job, and the diagnostic row alone cannot see it.
        // This row's folder is a second scratch directory, and `write_artifact` below hides only
        // the first one, so the machine-specific part is replaced here instead. Without this the
        // committed table would carry the path of whichever machine last ran the test.
        let finished = format!("The 0 frames that finished are in {}.", elsewhere.display());
        let anywhere =
            |text: &str| text.replace(&elsewhere.display().to_string(), "<a temporary directory>");
        check(
            "and the window's own first sentence says what is there, not that it exported them",
            &anywhere(&finished),
            &anywhere(&says(&finished, &said)),
        );
        check(
            "and left no half-written file behind",
            &"[]",
            &format!("{:?}", written(&elsewhere)),
        );
        check(
            "asking a window with nothing running to cancel is not an error either",
            &"No export is running.",
            &match Export::default().cancel {
                Some(_) => "a job was running".to_string(),
                None => "No export is running.".to_string(),
            },
        );

        // ---- somewhere the frames cannot go ---------------------------------------------------------
        let nowhere = into.join("no_such_directory");
        let mut refused = short(MissingSource::RenderTransparent);
        refused.output_dir = nowhere.clone();
        let said = run_export(&snapshot, &root, &refused, &AtomicBool::new(false));
        check(
            "an export into a folder that is not there says how far it got before it stopped",
            &"The export stopped on a problem after 0 of 2 frames",
            &says("The export stopped on a problem after 0 of 2 frames", &said),
        );
        let could_not = format!(
            "Frame 14 could not be written to {}. Check that the folder exists, is writable and \
             has room",
            nowhere.join("the reference shot_0014.png").display()
        );
        check(
            "and names the file it could not write, rather than only that something went wrong",
            &could_not,
            &says(&could_not, &said),
        );
        check(
            "and the folder is still not there: nothing was created to hold a failure",
            &false,
            &nowhere.exists(),
        );

        write_artifact(&rows, &into.display().to_string());
        let failed: Vec<&(String, String, String)> =
            rows.iter().filter(|(_, e, a)| e != a).collect();
        assert!(
            failed.is_empty(),
            "{} of {} checks failed, see verification/B-10_export_table.md: {:#?}",
            failed.len(),
            rows.len(),
            failed
        );
    }

    fn write_artifact(rows: &[(String, String, String)], scratch: &str) {
        let passed = rows.iter().filter(|(_, e, a)| e == a).count();
        let mut out = String::new();
        out.push_str("# B-10, exporting from the window\n\n");
        out.push_str(
            "The renderer could export a shot and the window could not ask it to. It can now, and \
             this is what the asking does. Produced by `cargo test -p anime_compositor_app`, from \
             `app/src/main.rs`.\n\n",
        );
        out.push_str(
            "What the frames themselves look like is not this table's question — `T-08_export_\
             table.md` checks the pixels and the naming to the frame, and `B-10_full_shot_table.md` \
             exports the whole shot twice and requires the two to be identical byte for byte. What \
             is new here is everything between the button and that: **which frames the window asks \
             for, what it names them, that what is written is the project as it was when the \
             person asked rather than as it is when the job finishes, and that a refusal, a \
             cancellation and a failure each arrive in words instead of in silence.**\n\n",
        );
        out.push_str(
            "The project is the reference shot, chosen because of its deliberate gap: layer 3 has \
             no drawing 7, so frames 14 and 15 ask for a drawing that is not there. Document 07 \
             blocks an export on that by default, and the checkbox beside the Export button is the \
             person overriding it in front of a sentence that says what the override does.\n\n",
        );
        out.push_str(
            "Rows about a sentence quote it. The Expected column is the words that had to reach \
             the person; the Actual column is those words if they did, and everything the window \
             said instead if they did not.\n\n",
        );
        out.push_str("| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
        for (check, expected, actual) in rows {
            let short = |text: &str| {
                text.replace(scratch, "<a temporary directory>")
                    .replace('|', r"\|")
            };
            out.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                check,
                short(expected),
                short(actual),
                if expected == actual { "pass" } else { "FAIL" }
            ));
        }
        out.push_str(&format!(
            "\n**{} of {} checks pass.**\n",
            passed,
            rows.len()
        ));
        out.push_str(
            "\n## What this does not cover\n\nThe folder dialog, which belongs to the operating \
             system and which a test has no hands to answer, so what is checked here begins at the \
             folder the person chose.\n\n\
             **Two frames, not two hundred and forty.** Which frames the window asks for is a row \
             above; what a whole shot does is `B-10_full_shot_table.md`, which runs for four \
             minutes and is not part of an ordinary build.\n\n\
             **There is no progress bar.** R-09 asks for cancellation between frames and for \
             failure to be reported, and both are here; it does not ask for a count of frames as \
             they are written, and the core has no hook to report one without a change to its \
             signature that only a window wants. What the window shows while a job runs is what it \
             is doing and a Cancel button, and what it shows afterwards is the core's report.\n\n\
             **A second export while one is running** is refused by `start_export`, which needs a \
             running application to reach — the refusal is one branch above the part this table \
             can call.\n\n\
             Where a row says *a temporary directory*, the real value was this machine's scratch \
             directory, which differs on every machine and every run.\n",
        );
        std::fs::write(repo("verification/B-10_export_table.md"), out).expect("write the artifact");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_both_forms_and_the_quality_beside_them() {
        assert_eq!(parse("/at/0", None), Some((Ask::At(0), None)));
        assert_eq!(parse("/at/16683", None), Some((Ask::At(16683), None)));
        assert_eq!(parse("/frame/100", None), Some((Ask::Frame(100), None)));
        assert_eq!(parse("/play/30", None), Some((Ask::Play(30), None)));
        assert_eq!(
            parse("/frame/-3", Some("q=full")),
            Some((Ask::Frame(-3), Some(PreviewQuality::Full)))
        );
        assert_eq!(
            parse("/at/1000", Some("t=9&q=draft")),
            Some((Ask::At(1000), Some(PreviewQuality::Draft)))
        );
    }

    #[test]
    fn refuses_what_it_does_not_understand() {
        for path in ["/", "/at", "/at/", "/at/soon", "/frame/1/2", "/frames/1"] {
            assert_eq!(parse(path, None), None, "{path} should not be understood");
        }
    }

    /// A quality that cannot be read leaves the preview alone rather than changing it. Silently
    /// switching resolution on a typo would put a frame on screen at a resolution nobody asked
    /// for, with an indicator that agreed with it.
    #[test]
    fn an_unreadable_quality_changes_nothing() {
        assert_eq!(parse("/at/0", Some("q=fastest")), Some((Ask::At(0), None)));
        assert_eq!(
            parse("/at/0", Some("quality=full")),
            Some((Ask::At(0), None))
        );
    }

    /// Expected bytes worked out from UTF-8 by hand, not read off this function: 夜 is
    /// E5 A4 9C and 空 is E7 A9 BA. A wrong encoder that agreed with itself would still fail
    /// here.
    #[test]
    fn japanese_survives_the_journey_into_a_header() {
        assert_eq!(for_a_header("夜空"), "%E5%A4%9C%E7%A9%BA");
        assert_eq!(
            for_a_header("media/背景/夜空.png"),
            "media/%E8%83%8C%E6%99%AF/%E5%A4%9C%E7%A9%BA.png"
        );
    }

    /// The other direction, against the same hand-worked UTF-8: E5 A4 9C is 夜, E7 A9 BA is 空.
    /// A path that arrives half-decoded is a path to a file that is not there.
    #[test]
    fn japanese_survives_the_journey_back() {
        assert_eq!(from_a_query("%E5%A4%9C%E7%A9%BA"), "夜空");
        assert_eq!(
            from_a_query("C:%5CShots%5C%E5%A4%9C%E7%A9%BA.json"),
            r"C:\Shots\夜空.json"
        );
        assert_eq!(from_a_query("100%25 sure"), "100% sure");
        // A stray percent is kept rather than eating what follows it. Shortening a path by two
        // characters would open the wrong file or none.
        assert_eq!(from_a_query("50% of it"), "50% of it");
    }

    /// Both directions, on the strings this window actually carries.
    #[test]
    fn the_two_encodings_are_each_other() {
        for text in [
            "夜空",
            r"C:\Shots\背景\shot.json",
            "2 of the files for \"layer3\" are missing.",
            "100% sure",
        ] {
            assert_eq!(from_a_query(&for_a_header(text)), text);
        }
    }

    /// Newest first, and a project opened twice appears once. The second rule is what makes the
    /// list a list of projects rather than a log of openings.
    #[test]
    fn the_recent_list_is_newest_first_and_holds_each_project_once() {
        let list = remembered(&[], r"C:\a.json");
        assert_eq!(list, vec![r"C:\a.json"]);

        let list = remembered(&list, r"C:\b.json");
        assert_eq!(list, vec![r"C:\b.json", r"C:\a.json"]);

        let list = remembered(&list, r"C:\a.json");
        assert_eq!(list, vec![r"C:\a.json", r"C:\b.json"]);
    }

    /// ADR-001 is Windows only, where these are one file. Two entries for one project would be
    /// two ways to open the same thing, one of them looking like something else.
    #[test]
    fn one_file_spelled_two_ways_is_one_entry() {
        let list = remembered(&[r"C:\Shots\A.JSON".to_string()], r"c:\shots\a.json");
        assert_eq!(list, vec![r"c:\shots\a.json"]);
    }

    /// The cap holds, and it is the oldest that goes.
    #[test]
    fn the_recent_list_forgets_the_oldest_first() {
        let mut list: Vec<String> = Vec::new();
        for n in 0..12 {
            list = remembered(&list, &format!(r"C:\shot{n}.json"));
        }
        assert_eq!(list.len(), RECENT_LIMIT);
        assert_eq!(list[0], r"C:\shot11.json");
        assert_eq!(list[RECENT_LIMIT - 1], r"C:\shot4.json");
    }

    /// Newlines separate the notes and `%` would otherwise make decoding ambiguous, so both are
    /// encoded. Ordinary punctuation is left alone, because a diagnostic is meant to be read in
    /// the header as well as after it.
    #[test]
    fn encodes_only_what_a_header_cannot_carry() {
        assert_eq!(for_a_header("one\ntwo"), "one%0Atwo");
        assert_eq!(for_a_header("100% sure"), "100%25 sure");
        assert_eq!(
            for_a_header("2 of the files for \"layer3\" are missing."),
            "2 of the files for \"layer3\" are missing."
        );
    }
}

/// The window against document 24, in both directions.
///
/// Every other table under `verification/` checks what a command *does*. These two check that
/// the command is *reachable*: that the page asks for the identifier document 24 names, that
/// something answers every identifier the page asks for, and that a shortcut the document
/// promises is bound to something. Both read the real files rather than a copy of them -
/// `app/ui/index.html` and `Markdown/24_UI_Command_and_Interaction_Map.md` - so a rename on one
/// side and not the other is what they are for.
///
/// This is the first test in this project that reads the page at all. It is string matching over
/// JavaScript and not a browser, which bounds it exactly: it can see which identifier a handler
/// names, and it cannot see whether the button that handler is attached to is on the screen. The
/// photographs beside it are what say that.
#[cfg(test)]
mod contract {
    use super::*;

    struct Report {
        rows: Vec<(String, String, String)>,
    }

    impl Report {
        fn check(&mut self, check: &str, expected: impl ToString, actual: impl ToString) {
            self.rows
                .push((check.to_string(), expected.to_string(), actual.to_string()));
        }
    }

    fn repo(rel: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("the app crate has a parent directory")
            .join(rel)
    }

    fn page() -> String {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui/index.html");
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
    }

    fn source() -> String {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/main.rs");
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
    }

    /// Everything that follows `needle`, up to the first character a route cannot contain.
    ///
    /// A route is written in the page in one of three shapes, and this reads all three by being
    /// given each opening in turn: a quoted string beginning with a slash, the identifier handed
    /// to `onSelected`, and the one handed to `send`.
    fn after(text: &str, needle: &str) -> Vec<String> {
        text.match_indices(needle)
            .map(|(at, _)| {
                let rest = &text[at + needle.len()..];
                let end = rest
                    .find(|c: char| !(c.is_ascii_lowercase() || c == '.' || c == '_' || c == '-'))
                    .unwrap_or(rest.len());
                rest[..end].to_string()
            })
            .filter(|found| !found.is_empty())
            .collect()
    }

    fn sorted(mut list: Vec<String>) -> Vec<String> {
        list.sort();
        list.dedup();
        list
    }

    /// Every route the page can ask the window for.
    fn asked_for(page: &str) -> Vec<String> {
        let mut found = after(page, "'/");
        found.extend(after(page, "onSelected('"));
        found.extend(after(page, "send('"));
        sorted(found)
    }

    /// The global accelerator handler, which is the last `keydown` listener in the page.
    fn accelerators(page: &str) -> String {
        let at = page
            .find("addEventListener('keydown', (e) => {")
            .expect("the page has a global keydown handler");
        page[at..].to_string()
    }

    // ---- the page ----------------------------------------------------------------------------

    /// Every command identifier written into the page, in the order a sorted list puts them.
    ///
    /// Pinned rather than counted: a new `send` of an identifier nobody listed is a change to
    /// what the window can do, and it should have to be written down here as well as there.
    const SENT: &[&str] = &[
        "composition.create",
        "composition.open",
        "edit.redo",
        "edit.undo",
        "effect.add",
        "effect.delete",
        "effect.move",
        "effect.move_down",
        "effect.move_up",
        "effect.set_parameters",
        "effect.toggle_bypass",
        "exposure.set_span",
        "keyframe.add_remove",
        "keyframe.move",
        "keyframe.set_interp",
        "layer.create",
        "layer.delete",
        "layer.move_down",
        "layer.move_up",
        "layer.rename",
        "layer.set_matte",
        "layer.shift",
        "layer.toggle_lock",
        "layer.toggle_visibility",
        "layer.trim",
        "media.import",
        "media.relink",
        "property.drag_cancel",
        "property.drag_end",
        "property.drag_update",
        "property.set_base",
        "viewer.toggle_alpha",
        "viewer.toggle_checkerboard",
    ];

    /// The routes that are not commands: the shell's own, and the two the transport uses.
    const ROUTES: &[&str] = &[
        // The frame scheme's four, which are not commands and are not answered by the shell:
        // `frame` is a numbered frame, `at` is the frame playback has reached by a given number
        // of milliseconds, `play` starts the clock over from the playhead (W-09), and `boxes`
        // is where the selected layers landed on that frame, which only the renderer knows
        // because an asset records no pixel size. `curve` is the fifth, added by D-52: one
        // property evaluated across a range, so the graph editor draws what the renderer will
        // do rather than its own idea of it. The clock is checked in
        // `verification/B-08_preview_table.md`.
        "at",
        "boxes",
        "cancel-export",
        "curve",
        "export",
        "frame",
        "open",
        "play",
        "recent",
        "recover",
        "save",
        "save-as",
        "state",
    ];

    /// A control, the identifier it must send, and the text in the page that says it does.
    ///
    /// The third column is what makes this more than a list: it anchors the identifier to the
    /// handler that sends it, so moving `layer.delete` onto the button that moves a layer
    /// forward fails here rather than passing because the string is still somewhere in the file.
    const WIRING: &[(&str, &str, &str)] = &[
        (
            "Delete layer",
            "layer.delete",
            "$('dellayer').onclick = onSelected('layer.delete')",
        ),
        (
            "Forward",
            "layer.move_up",
            "$('up').onclick = onSelected('layer.move_up')",
        ),
        (
            "Back",
            "layer.move_down",
            "$('down').onclick = onSelected('layer.move_down')",
        ),
        (
            "Undo",
            "edit.undo",
            "$('undo').onclick = () => command('/edit.undo')",
        ),
        (
            "Redo",
            "edit.redo",
            "$('redo').onclick = () => command('/edit.redo')",
        ),
        (
            "Alpha only",
            "viewer.toggle_alpha",
            "$('alpha').onclick = () => command('/viewer.toggle_alpha')",
        ),
        (
            "the transparency grid",
            "viewer.toggle_checkerboard",
            "$('checker').onclick = () => command('/viewer.toggle_checkerboard')",
        ),
        (
            "Import drawings",
            "media.import",
            "$('import').onclick = () => command('/media.import')",
        ),
        (
            "Add an exposure",
            "exposure.set_span",
            "command('/exposure.set_span?layer='",
        ),
        ("Add effect", "effect.add", "command('/effect.add?layer='"),
        (
            "Add layer",
            "layer.create",
            "command('/layer.create?asset='",
        ),
        (
            "Relink drawings",
            "media.relink",
            "command('/media.relink?asset=' + encodeURIComponent(selectedAsset))",
        ),
        (
            "Apply the relink",
            "media.relink",
            "encodeURIComponent(relinking) + '&apply=1'",
        ),
        (
            "Leave it as it is",
            "media.relink",
            "encodeURIComponent(relinking) + '&cancel=1'",
        ),
    ];

    #[test]
    fn the_page_asks_for_nothing_the_window_cannot_answer() {
        let mut report = Report { rows: Vec::new() };
        let page = page();
        let source = source();
        let asked = asked_for(&page);

        let (commands, routes): (Vec<String>, Vec<String>) =
            asked.into_iter().partition(|found| found.contains('.'));

        report.check(
            "the identifiers the page sends are the ones written down here",
            SENT.join(", "),
            commands.join(", "),
        );
        report.check(
            "the routes the page asks for that are not commands are the ones written down here",
            ROUTES.join(", "),
            routes.join(", "),
        );

        // Every command identifier, answered. `None` is the window saying it has never heard of
        // the identifier, which is what a page sending `layer.remove` for `layer.delete` would
        // look like: a button that does nothing at all, silently.
        let viewer = Mutex::new(demo());
        for id in &commands {
            report.check(
                &format!("the window answers `{id}`"),
                "a sentence",
                match edit_command(&viewer, id, None) {
                    Some(said) if said.is_empty() => "an empty answer".to_string(),
                    Some(_) => "a sentence".to_string(),
                    None => "nothing - the window has never heard of it".to_string(),
                },
            );
        }
        // The two inspection toggles were flipped by the loop above, because answering them is
        // what they do. Put them back, so that nothing after this reads a viewer this test left
        // looking at the alpha channel.
        for id in ["viewer.toggle_alpha", "viewer.toggle_checkerboard"] {
            edit_command(&viewer, id, None);
        }

        // The routes are answered by `fn command` rather than by `edit_command`, so what is
        // checked is that the shell has an arm of that name. `frame` and `state` are not in
        // that match: one is the other scheme, one is answered before it.
        for route in &routes {
            // `frame`, `at`, `play`, `boxes` and `curve` belong to the other scheme and are served
            // beside `fn frame`, not by the command shell, so there is no arm of that name to
            // look for.
            if matches!(route.as_str(), "frame" | "at" | "play" | "boxes" | "curve") {
                continue;
            }
            let arm = format!("\"{route}\"");
            report.check(
                &format!("the window's shell answers `/{route}`"),
                true,
                source.contains(&arm),
            );
        }

        for (control, id, wiring) in WIRING {
            report.check(
                &format!("the {control} control sends `{id}`"),
                true,
                page.contains(wiring),
            );
        }

        // Every check above this line reads the script and none of them reads the markup, which
        // is how renaming `id="up"` to `id="upward"` and leaving the handler saying `$('up')`
        // got through the whole table once. A handler hung on a name nothing is called is a
        // button that does nothing, silently, exactly like an identifier nobody answers.
        let defined: Vec<&str> = page
            .match_indices("id=\"")
            .map(|(at, _)| {
                let rest = &page[at + 4..];
                &rest[..rest.find('"').unwrap_or(0)]
            })
            .collect();
        let missing: Vec<String> = sorted(after(&page, "$('"))
            .into_iter()
            .filter(|name| !defined.contains(&name.as_str()))
            .collect();
        report.check(
            "every control the script reaches for is one the markup defines",
            "none missing",
            match missing.is_empty() {
                true => "none missing".to_string(),
                false => missing.join(", "),
            },
        );

        write_artifact(
            &report,
            "verification/B-12b_page_table.md",
            "B-12b: what the page asks the window for",
            PAGE_INTRO,
            PAGE_NOTES,
        );
        let failed: Vec<&String> = report
            .rows
            .iter()
            .filter(|(_, e, a)| e != a)
            .map(|(c, _, _)| c)
            .collect();
        assert!(failed.is_empty(), "these checks failed: {failed:#?}");
    }

    const PAGE_INTRO: &[&str] = &[
        "`app/ui/index.html` is the one file in this project that no test has ever read. Every \
         B-12a table calls the same function the window's URL scheme calls, which checks \
         everything from the request inwards and nothing outwards of it; what the page does with \
         a click has been checked only by looking at photographs. This table is the outward half, \
         as far as reading a file can take it.",
        "It reads the page and the window's own source, and asks three questions. Which \
         identifiers does the page send? Does anything answer them? And is each named control \
         wired to the identifier document 24 gives it?",
    ];

    const PAGE_NOTES: &[&str] = &[
        "## What to look at\n\n- **The first two rows are a list, not a count.** They name every \
         identifier and every route the page can ask for, in one cell each. A button added \
         tomorrow that sends something new fails this row until the new identifier is written \
         down beside the others, which is the point: what the window can be asked to do should \
         be a thing somebody wrote down.\n- **`nothing - the window has never heard of it`** is \
         the answer this table exists to catch. A page sending an identifier the window does not \
         answer is a button that does nothing, with no error, no status line and nothing in the \
         log.\n- **The wiring rows anchor an identifier to its handler.** Checking only that \
         `layer.delete` appears somewhere in the page would pass if Delete layer and Forward \
         swapped commands.\n- **The last row reads the markup rather than the script.** Every \
         other row here reads the handlers; that one checks that each control a handler reaches \
         for is a control this page actually contains, because renaming a button and forgetting \
         its handler leaves the handler attached to nothing.",
        "## What this cannot cover\n\nThis is string matching over JavaScript, not a browser. It \
         can see that a handler names an identifier. It cannot see that the handler is attached \
         to a button, that the button is on the screen, that it is enabled, or that clicking it \
         reaches the handler. `verification/B-12a_window_and_keyboard.md` and its photographs are \
         what say those, and the owner's run under B-12 is what says it for real.\n\nIt also \
         cannot see the shell routes actually working. `/save` is checked here only as far as \
         `fn command` having an arm of that name; what a save writes is \
         `verification/B-09_save_table.md`.",
    ];

    // ---- document 24 -------------------------------------------------------------------------

    /// How each of document 24's command identifiers is reached in this build.
    ///
    /// Written down rather than derived, because the interesting half is the identifiers nothing
    /// reaches: an identifier that is simply absent from a build looks exactly like one nobody
    /// has noticed is absent. The test measures which of the four this build actually does and
    /// compares; a row that says `nothing yet` is checked to be true as hard as the others.
    const REACHED: &[(&str, &str)] = &[
        ("project.new", "nothing yet"),
        ("project.open", "a route the shell answers"),
        ("project.save", "a route the shell answers"),
        ("project.save_as", "a route the shell answers"),
        ("composition.create", "a command the window answers"),
        ("composition.open", "a command the window answers"),
        ("edit.undo", "a command the window answers"),
        ("edit.redo", "a command the window answers"),
        ("media.import", "a command the window answers"),
        ("media.relink", "a command the window answers"),
        ("layer.create", "a command the window answers"),
        ("layer.delete", "a command the window answers"),
        ("layer.rename", "a command the window answers"),
        ("layer.move_up", "a command the window answers"),
        ("layer.move_down", "a command the window answers"),
        ("layer.toggle_visibility", "a command the window answers"),
        ("layer.toggle_lock", "a command the window answers"),
        ("layer.set_matte", "a command the window answers"),
        ("layer.shift", "a command the window answers"),
        ("layer.trim", "a command the window answers"),
        ("timeline.previous_frame", "the page, with no request"),
        ("timeline.next_frame", "the page, with no request"),
        ("timeline.play_pause", "the page, with no request"),
        ("timeline.set_work_start", "nothing yet"),
        ("timeline.set_work_end", "nothing yet"),
        ("exposure.set_span", "a command the window answers"),
        ("property.set_base", "a command the window answers"),
        ("keyframe.add_remove", "a command the window answers"),
        ("keyframe.move", "a command the window answers"),
        ("keyframe.set_interp", "a command the window answers"),
        ("effect.add", "a command the window answers"),
        ("effect.delete", "a command the window answers"),
        ("effect.toggle_bypass", "a command the window answers"),
        ("effect.set_parameters", "a command the window answers"),
        ("effect.move_up", "a command the window answers"),
        ("effect.move_down", "a command the window answers"),
        ("effect.move", "a command the window answers"),
        ("viewer.fit", "the page, with no request"),
        ("viewer.zoom_100", "the page, with no request"),
        ("viewer.toggle_checkerboard", "a command the window answers"),
        ("viewer.toggle_alpha", "a command the window answers"),
        ("render.preview_current", "the page, with no request"),
        ("export.sequence", "a route the shell answers"),
        ("app.command_palette", "nothing yet"),
    ];

    /// Document 24's identifier, and the text in the page that binds the shortcut it promises.
    ///
    /// Only the identifiers document 24 gives a G1 shortcut appear here. `none` in that column
    /// is not a gap and is not listed.
    const SHORTCUTS: &[(&str, &str, &str)] = &[
        ("project.new", "Ctrl+N", ""),
        ("project.open", "Ctrl+O", "e.ctrlKey && (e.key === 'o'"),
        ("project.save", "Ctrl+S", "e.ctrlKey && (e.key === 's'"),
        ("project.save_as", "Ctrl+Shift+S", "e.shiftKey ? '/save-as'"),
        (
            "composition.create",
            "Ctrl+Shift+N",
            "e.shiftKey && (e.key === 'n'",
        ),
        ("edit.undo", "Ctrl+Z", "e.key === 'z'"),
        ("edit.redo", "Ctrl+Shift+Z", "e.shiftKey ? $('redo')"),
        ("media.import", "Ctrl+I", "e.key === 'i'"),
        ("layer.create", "Ctrl+Alt+L", "e.altKey && (e.key === 'l'"),
        ("layer.delete", "Delete", "e.key === 'Delete'"),
        ("layer.rename", "F2", "e.key === 'F2'"),
        ("layer.move_up", "Ctrl+]", "e.key === ']'"),
        ("layer.move_down", "Ctrl+[", "e.key === '['"),
        ("layer.shift", "[ or ]", "e.key === '[' || e.key === ']'"),
        (
            "layer.trim",
            "Alt+[ or Alt+]",
            "e.key === '[' || e.key === ']'",
        ),
        ("timeline.previous_frame", "Left", "e.key === 'ArrowLeft'"),
        ("timeline.next_frame", "Right", "e.key === 'ArrowRight'"),
        ("timeline.play_pause", "Space", "e.key === ' '"),
        ("timeline.set_work_start", "B", ""),
        ("timeline.set_work_end", "N", ""),
        ("viewer.fit", "Shift+/", "e.key === '?'"),
        ("viewer.zoom_100", "Ctrl+1", "e.key === '1'"),
        ("export.sequence", "Ctrl+M", "e.key === 'm'"),
        ("app.command_palette", "Ctrl+Shift+P", ""),
    ];

    /// The accelerators that work by pressing a button, and the button each one presses.
    ///
    /// The table above only sees that a key is tested for somewhere in the handler. Moving
    /// `Delete` onto the Forward button went straight through it, because the key test was still
    /// there and only what it did had changed. This reads the arm each key opens - the text from
    /// its test to the next `else if` - and says which control that arm reaches for.
    const PRESSES: &[(&str, &str, &str)] = &[
        ("Ctrl+M", "e.key === 'm'", "$('export')"),
        ("Ctrl+I", "e.key === 'i'", "$('import')"),
        ("Ctrl+Shift+N", "e.key === 'n'", "$('newcomp')"),
        ("Ctrl+Alt+L", "e.key === 'l'", "$('addlayer')"),
        ("Ctrl+]", "e.key === ']'", "$('up')"),
        ("Ctrl+[", "e.key === '['", "$('down')"),
        ("Delete", "e.key === 'Delete'", "$('dellayer')"),
        ("Space", "e.key === ' ') {", "$('play')"),
        ("D", "e.key === 'd'", "$('toggle')"),
        ("Alt+A", "e.key === 'a'", "$('alpha')"),
        ("G", "e.key === 'g'", "$('checker')"),
        ("Shift+/", "e.key === '?'", "$('fit')"),
    ];

    /// Document 24's command table, read out of the document itself.
    fn document_24() -> Vec<(String, String)> {
        let text = std::fs::read_to_string(repo("Markdown/24_UI_Command_and_Interaction_Map.md"))
            .expect("read document 24");
        text.lines()
            .filter(|line| line.starts_with('|'))
            .map(|line| {
                line.trim_matches('|')
                    .split('|')
                    .map(str::trim)
                    .collect::<Vec<_>>()
            })
            .filter(|cells| cells.len() == 4 && cells[0].contains('.'))
            .map(|cells| (cells[0].to_string(), cells[2].to_string()))
            .collect()
    }

    #[test]
    fn every_command_document_24_names_is_accounted_for() {
        let mut report = Report { rows: Vec::new() };
        let page = page();
        let keys = accelerators(&page);
        let listed = document_24();
        let viewer = Mutex::new(demo());
        let source = source();
        let asked = asked_for(&page);

        report.check(
            "document 24 names the identifiers this table walks",
            REACHED.len(),
            listed.len(),
        );
        report.check(
            "and they are the same identifiers, in the same order",
            REACHED
                .iter()
                .map(|(id, _)| *id)
                .collect::<Vec<_>>()
                .join(", "),
            listed
                .iter()
                .map(|(id, _)| id.as_str())
                .collect::<Vec<_>>()
                .join(", "),
        );

        for (id, expected) in REACHED {
            // Measured four ways, in the order a request would find them: the shell answers a
            // route before `edit_command` sees it, `edit_command` answers a command, the page
            // may do something with no request at all, and otherwise nothing does.
            let route = match *id {
                "project.open" => "open",
                "project.save" => "save",
                "project.save_as" => "save-as",
                "export.sequence" => "export",
                _ => "",
            };
            let reached = if !route.is_empty() && source.contains(&format!("\"{route}\"")) {
                "a route the shell answers"
            } else if edit_command(&viewer, id, None).is_some() {
                "a command the window answers"
            } else if asked.iter().any(|sent| sent == id) {
                "the page sends it and nothing answers"
            } else if PAGE_SIDE.contains(id) {
                "the page, with no request"
            } else {
                "nothing yet"
            };
            report.check(&format!("`{id}` is reached by"), expected, reached);
        }
        for id in ["viewer.toggle_alpha", "viewer.toggle_checkerboard"] {
            edit_command(&viewer, id, None);
        }

        for (id, shortcut, binding) in SHORTCUTS {
            let promised = listed
                .iter()
                .find(|(listed_id, _)| listed_id == id)
                .map(|(_, shortcut)| shortcut.clone())
                .unwrap_or_else(|| "(not in document 24)".to_string());
            report.check(
                &format!("document 24 gives `{id}` the shortcut"),
                shortcut,
                promised,
            );
            let reached = REACHED
                .iter()
                .find(|(reached_id, _)| reached_id == id)
                .map(|(_, how)| *how)
                .unwrap_or("nothing yet");
            report.check(
                &format!("and {shortcut} is bound"),
                match reached {
                    "nothing yet" => "no, and the command is not built either",
                    _ => "yes",
                },
                match (binding.is_empty(), keys.contains(binding)) {
                    (true, _) => "no, and the command is not built either",
                    (false, true) => "yes",
                    (false, false) => "no, though the command is built",
                },
            );
        }

        for (shortcut, key, control) in PRESSES {
            let arm = match keys.find(key) {
                Some(at) => {
                    let rest = &keys[at..];
                    &rest[..rest.find("else if").unwrap_or(rest.len())]
                }
                None => "",
            };
            report.check(
                &format!("and {shortcut} presses"),
                *control,
                match arm.split_once("$('") {
                    Some((_, rest)) => format!("$('{}", &rest[..rest.find(')').unwrap_or(0) + 1]),
                    None => format!("nothing - no arm tests for {key}"),
                },
            );
        }

        write_artifact(
            &report,
            "verification/B-12b_command_map_table.md",
            "B-12b: document 24's command map against the window",
            MAP_INTRO,
            MAP_NOTES,
        );
        let failed: Vec<&String> = report
            .rows
            .iter()
            .filter(|(_, e, a)| e != a)
            .map(|(c, _, _)| c)
            .collect();
        assert!(failed.is_empty(), "these checks failed: {failed:#?}");
    }

    /// The three document 24 identifiers the page carries out itself.
    ///
    /// Stepping and playback change nothing in the project - document 24 marks all three not
    /// undoable - so they are a number the page holds and a frame it asks for, and there is no
    /// command for them to send. `render.preview_current` is the same thing said differently:
    /// every frame this window shows is rendered when it is asked for.
    const PAGE_SIDE: &[&str] = &[
        "timeline.previous_frame",
        "timeline.next_frame",
        "timeline.play_pause",
        "render.preview_current",
        "viewer.fit",
        "viewer.zoom_100",
    ];

    const MAP_INTRO: &[&str] = &[
        "Document 24 names thirty-six command identifiers. This walks all thirty-six against \
         the build and says, for each, what actually reaches it - and then walks the twenty-one \
         it gives a keyboard shortcut and says whether that shortcut is bound. The identifiers \
         and the shortcuts are read out of `Markdown/24_UI_Command_and_Interaction_Map.md` \
         itself, so an edit to the document that nothing implements fails here.",
        "There are four ways a command is reached in this build, and the second column of every \
         row is one of them. **A route the shell answers** is one that needs the operating system \
         - opening, saving, exporting - and is answered before the command layer sees it. **A \
         command the window answers** goes through `edit_command`, which is the command layer \
         document 24's first paragraph describes. **The page, with no request** is the transport: \
         stepping and playing change nothing in the project, so there is nothing to send. \
         **Nothing yet** is a command this build does not have.",
    ];

    const MAP_NOTES: &[&str] = &[
        "## What to look at\n\n- **`nothing yet` is checked as hard as the rest.** A row that \
         says a command is not built is a claim, and the test confirms nothing answers that \
         identifier. This is what stops the list quietly going stale after somebody builds \
         one.\n- **`the page sends it and nothing answers`** is a value no row expects, and \
         seeing it in the Actual column would mean a button that does nothing.\n- **The shortcut \
         rows come in pairs.** The first says what document 24 promises, read from the document. \
         The second says whether the page binds it.\n- **The `presses` rows say what the key \
         does, not only that it is bound.** A shortcut moved onto the wrong button keeps its key \
         test and stops doing its job; those rows are the ones that would say so.",
        "## The six commands this build does not have\n\nEach is a deliberate absence, and none \
         of them is a step of W-01.\n\n- **`project.new`** - a new project is an empty window and \
         this build always opens on something: the reference shot when it is given nothing, or \
         the project it was given. Making a new one is Save As over a copy.\n- \
         **`timeline.set_work_start` and `set_work_end`** - the work area is the whole \
         composition in this build, which is what `verification/B-08_preview_table.md` measures \
         and what B-10 exports. Narrowing it is a setting nothing yet reads.\n- \
         **`keyframe.add_remove`** - since W-10 the diamond beside each transform property, \
         checked in `verification/B-12a_transform_table.md`.\n- **`viewer.fit` and `viewer.zoom_100`** are the page's own since W-06: a \
         zoom is a size the page gives the canvas and a scroll of the stage around it, and \
         nothing in the project changes, so neither sends a request.\n- \
         **`app.command_palette`** - a search over commands, which needs the commands to be \
         worth searching first.",
        "## What this cannot cover\n\nThat a bound shortcut reaches the command. The binding is \
         read as text in the page's accelerator handler; that pressing the key really runs it is \
         `verification/B-12a_window_and_keyboard.md`, where a Q-03 photograph found exactly that \
         defect - rows that Tab reached and no key could operate.\n\nWhether document 24's \
         shortcuts conflict with Windows or with the web view, which document 24 itself asks for \
         and which needs a person at the keyboard.",
    ];

    /// One table with its own prose around it.
    fn write_artifact(report: &Report, file: &str, title: &str, intro: &[&str], notes: &[&str]) {
        let passed = report.rows.iter().filter(|(_, e, a)| e == a).count();
        let mut out = format!("# {title}\n\n");
        out.push_str(
            "Generated by `app/src/main.rs`, module `contract`. Re-run with `cargo test \
             --workspace`.\n\n",
        );
        for paragraph in intro.iter().chain(notes) {
            out.push_str(paragraph);
            out.push_str("\n\n");
        }
        out.push_str("| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
        for (check, expected, actual) in &report.rows {
            out.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                check,
                cell(expected),
                cell(actual),
                if expected == actual { "pass" } else { "FAIL" }
            ));
        }
        out.push_str(&format!(
            "\n**{} of {} checks pass.**\n",
            passed,
            report.rows.len()
        ));
        std::fs::write(repo(file), out).expect("write the artifact");
    }

    // ---- document 26, the text half of coalescing ---------------------------------------------

    /// The three places in the page where somebody types into a field and the window is sent a
    /// command: the control, the text that wires it, and what commits it.
    ///
    /// The layer's name uses `onblur`, and since W-14 both numbers use the one rule every
    /// number in this window now keeps: what is typed is sent when it is committed, by Enter or
    /// by leaving the field, and never per keystroke - "a half-typed `-` is not a value anybody
    /// meant", which is what the transform fields have said since W-04. That is document 26's
    /// sentence about text, "committing/focus exit ends the transaction", and it is what After
    /// Effects does, which is what the fifth sitting asked these numbers to be like.
    ///
    /// Until W-14 an effect's setting was the one that went per keystroke, inside the drag
    /// transaction the picture's drags use. The second sitting's finding 10 was that the effect
    /// "did not show" until a click elsewhere took the focus; Enter now commits it where before
    /// only a click elsewhere did, so the complaint that bought that liveness is answered
    /// without it. Swapping a commit for a keystroke is a one-word edit, which is why the line
    /// that commits is pinned rather than trusted.
    const TYPED_FIELDS: &[(&str, &str, &str)] = &[
        (
            "a layer's name is committed by losing focus",
            "layer.rename",
            "box.onblur = () => finish(true);",
        ),
        (
            "an effect's settings are sent when the number is committed, by Enter or by leaving \
             the field",
            "effect.set_parameters",
            "commit: () => sendParameters(),",
        ),
        (
            "an exposure's frames are committed by Enter or by leaving the field",
            "exposure.set_span",
            "commit: sendSpan,",
        ),
    ];

    #[test]
    fn typing_into_a_field_is_one_history_entry_when_it_is_committed() {
        let mut report = Report { rows: Vec::new() };
        let page = page();

        // The page half. Nothing here can press a key -- what it can say is that no handler in
        // the page attached to the event that fires per keystroke sends a command. Since W-06
        // the zoom slider has such a handler, and a zoom asks the window for nothing, so the
        // statement each handler is in is read for a request rather than the handler counted.
        let per_keystroke: Vec<&str> = page
            .match_indices("oninput")
            .chain(page.match_indices("addEventListener('input'"))
            .map(|(at, _)| {
                let rest = &page[at..];
                let end = [rest.find(";\n"), rest.find(";\r")]
                    .into_iter()
                    .flatten()
                    .min();
                &rest[..end.map_or(rest.len(), |end| end + 1)]
            })
            .filter(|statement| statement.contains("command("))
            .collect();
        report.check(
            "no field in the page sends anything while a key is being pressed",
            "no input event handler sends a command",
            match per_keystroke.is_empty() {
                true => "no input event handler sends a command".to_string(),
                false => format!("{} do: {}", per_keystroke.len(), per_keystroke.join(" / ")),
            },
        );
        for (control, id, wiring) in TYPED_FIELDS {
            report.check(
                &format!("{control}, and sends `{id}`"),
                true,
                page.contains(wiring),
            );
        }
        report.check(
            "what an effect's field sends per keystroke is inside a drag",
            true,
            page.contains(
                "const live = () => { open = true; dragging = true; sendParameters('&drag=1'); };",
            ),
        );
        report.check(
            "and a name committed unchanged sends nothing at all",
            true,
            page.contains("if (commit && wanted !== layer.name) {"),
        );

        // The window half. One committed field, one entry in the undo list, named in the words
        // the person will read on the Undo button.
        let viewer = Mutex::new(demo());
        let before = held(&viewer).document.undo_depth();
        run(&viewer, "layer.rename?layer=layer-1&name=the background");
        report.check(
            "committing a new name is one entry in the undo list",
            format!("undo list {}", before + 1),
            format!("undo list {}", held(&viewer).document.undo_depth()),
        );
        report.check(
            "and the entry says what it will undo",
            "Rename layer to the background",
            held(&viewer)
                .document
                .undo_labels()
                .last()
                .unwrap_or(&"(nothing)")
                .to_string(),
        );

        run(&viewer, "effect.add?layer=layer-1&type=core.gaussian_blur");
        let before = held(&viewer).document.undo_depth();
        for sigma in [2, 6, 9] {
            run(
                &viewer,
                &format!("effect.set_parameters?layer=layer-1&effect=fx-1&sigma_px={sigma}"),
            );
        }
        report.check(
            "three settings committed one after another are three entries, not one and not thirty",
            format!("undo list {}", before + 3),
            format!("undo list {}", held(&viewer).document.undo_depth()),
        );
        run(&viewer, "edit.undo");
        report.check(
            "so undoing once goes back one commit, not back to before the field was touched",
            "GaussianBlur { sigma_px: 6.0 }",
            layer(&viewer, "layer-1", |l| {
                l.effects
                    .iter()
                    .map(|e| format!("{:?}", e.effect))
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_else(|| "(no such layer)".to_string()),
        );

        // W-07: the same field typed live. Three keystrokes travel inside one drag and losing
        // the focus closes it, so the three are one entry and the entry holds the last of them.
        let before = held(&viewer).document.undo_depth();
        for sigma in ["3", "3.", "3.5"] {
            run(
                &viewer,
                &format!("effect.set_parameters?layer=layer-1&effect=fx-1&sigma_px={sigma}&drag=1"),
            );
        }
        run(&viewer, "property.drag_end");
        report.check(
            "a setting typed in three keystrokes, each sent as it lands, is one entry",
            format!("undo list {}", before + 1),
            format!("undo list {}", held(&viewer).document.undo_depth()),
        );
        report.check(
            "and the entry holds the last keystroke, not the first",
            "GaussianBlur { sigma_px: 3.5 }",
            layer(&viewer, "layer-1", |l| {
                l.effects
                    .iter()
                    .map(|e| format!("{:?}", e.effect))
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_else(|| "(no such layer)".to_string()),
        );

        // The numeric half of the same rule, for contrast: a drag is many requests and one
        // entry, because document 26 makes the release the commit.
        let before = held(&viewer).document.undo_depth();
        for x in [100.0_f64, 140.0, 180.0] {
            run(
                &viewer,
                &format!("property.drag_update?layer=layer-2&prop=position&value={x}, 0"),
            );
        }
        run(&viewer, "property.drag_end?layer=layer-2&prop=position");
        report.check(
            "and a drag of three steps is one entry, which is the same rule for a number",
            format!("undo list {}", before + 1),
            format!("undo list {}", held(&viewer).document.undo_depth()),
        );

        // Two layers selected on the picture and dragged together. Still one press, still one
        // release, so still one entry - and both layers have to be in it, or undoing would put
        // one of them back and leave the other where the pointer left it.
        let before = held(&viewer).document.undo_depth();
        for x in [200.0_f64, 240.0, 300.0] {
            for layer in ["layer-1", "layer-2"] {
                run(
                    &viewer,
                    &format!("property.drag_update?layer={layer}&prop=position&value={x}, 0"),
                );
            }
        }
        run(&viewer, "property.drag_end");
        // Each of these takes the lock and gives it straight back. Two `held(&viewer)` calls
        // inside one `format!` would both still be alive when the second one asked for it, and
        // the second would wait for the first for ever.
        let depth = held(&viewer).document.undo_depth();
        let entry = held(&viewer)
            .document
            .undo_labels()
            .last()
            .unwrap_or(&"(nothing)")
            .to_string();
        report.check(
            "a drag holding two layers is one entry that says it moved more than one",
            format!(
                "undo list {}, Set position to (300, 0) and 1 more",
                before + 1
            ),
            format!("undo list {depth}, {entry}"),
        );
        report.check(
            "and undoing that one entry puts both layers back, not just the first",
            "layer-1 (0, 0), layer-2 (180, 0)",
            {
                run(&viewer, "edit.undo");
                ["layer-1", "layer-2"]
                    .iter()
                    .map(|id| {
                        format!(
                            "{id} {}",
                            layer(&viewer, id, |l| l.transform.position.base().to_string())
                                .unwrap_or_else(|| "(no such layer)".to_string())
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            },
        );

        // W-05: a layer's bar dragged along the timeline. The same transaction with a different
        // command inside it, and the flag in the query is what puts it there.
        let before = held(&viewer).document.undo_depth();
        let (in_0, out_0) = layer(&viewer, "layer-2", |l| (l.in_frame, l.out_frame))
            .expect("layer-2 is in the demo");
        for at in [4, 9, 12] {
            run(
                &viewer,
                &format!("layer.shift?layer=layer-2&in={at}&drag=1"),
            );
        }
        run(&viewer, "property.drag_end");
        let depth = held(&viewer).document.undo_depth();
        let frames = |viewer: &Mutex<Viewer>| {
            layer(viewer, "layer-2", |l| {
                format!("{} to {}", l.in_frame, l.out_frame)
            })
            .unwrap_or_else(|| "(no such layer)".to_string())
        };
        report.check(
            "a layer's bar dragged three steps along the timeline is one entry",
            format!("undo list {}, 12 to {}", before + 1, 12 + out_0 - in_0),
            format!("undo list {depth}, {}", frames(&viewer)),
        );
        run(&viewer, "edit.undo");
        report.check(
            "and undoing it puts the bar back where it was",
            format!("{in_0} to {out_0}"),
            frames(&viewer),
        );

        write_artifact(
            &report,
            "verification/B-12b_text_coalescing_table.md",
            "B-12b: one entry in the undo list per thing typed",
            TEXT_INTRO,
            TEXT_NOTES,
        );
        let failed: Vec<&String> = report
            .rows
            .iter()
            .filter(|(_, e, a)| e != a)
            .map(|(c, _, _)| c)
            .collect();
        assert!(failed.is_empty(), "these checks failed: {failed:#?}");
    }

    fn held(viewer: &Mutex<Viewer>) -> std::sync::MutexGuard<'_, Viewer> {
        viewer.lock().expect("the viewer lock was poisoned")
    }

    fn run(viewer: &Mutex<Viewer>, what: &str) -> String {
        let (id, query) = match what.split_once('?') {
            Some((id, query)) => (id, Some(query)),
            None => (what, None),
        };
        edit_command(viewer, id, query).expect("a command document 24 lists")
    }

    fn layer<T>(viewer: &Mutex<Viewer>, id: &str, read: impl FnOnce(&Layer) -> T) -> Option<T> {
        let held = held(viewer);
        let comp = held
            .document
            .project()
            .composition(&held.composition)
            .expect("the composition on screen");
        comp.layer(&Id::new(id)).map(read)
    }

    const TEXT_INTRO: &[&str] = &[
        "Document 26: \"Text edits may coalesce while one field has focus; committing or focus \
         exit ends the transaction.\" In plain terms, the promise is that typing `1`, `2`, `0` \
         into a field is one thing to undo and not three, and that leaving a field you did not \
         change is not a thing to undo at all.",
        "There are three fields in this window a person types into: a layer's name, an effect's \
         settings, and the frames of an exposure. All three keep that promise the same way, and \
         it is worth saying plainly because it is not code anybody in this project wrote: they \
         are wired to the browser's `change` event, which fires when a field is committed **and** \
         its value is not the one it had when it took focus. The alternative event, `input`, \
         fires on every keystroke, and a field wired to it would put one entry in the undo list \
         per letter. The first row below is the check that no field in this page is.",
    ];

    const TEXT_NOTES: &[&str] = &[
        "## What this cannot cover\n\nNo test in this project presses a key. The rows about the \
         page read the file and say which event each field is attached to; what the browser then \
         does with that event is the browser's, and is documented behaviour rather than \
         something measured here. The rows about the window send the command a committed field \
         would send and count what lands in the undo list, which is the half that is this \
         project's own.\n\nThe last three rows are not about text at all. They drag a position \
         through several values and end the drag, and are here because they are the same \
         sentence of document 26 read the other way: many requests, one entry. Without them a \
         reader has no way to see that three entries for three committed settings is the \
         intended answer rather than the same defect in the other direction. The last two of \
         them have two layers under the pointer at once, which is that promise again with more \
         than one thing being moved: one entry, both layers named in it, and undoing brings both \
         of them back rather than the first.",
    ];

    // ---- what the panels draw ------------------------------------------------------------------

    /// The variables the page reads project data out of, and where each one comes from in the
    /// answer `/state` gives.
    ///
    /// The fields themselves are not written down here. They are read out of the page, so a panel
    /// that starts reading something new is checked from the moment it does rather than when
    /// somebody remembers to add a row.
    const READ_FROM: &[(&str, &str)] = &[
        ("doc", "the whole answer"),
        ("comp", "the composition on screen"),
        ("layer", "one of its layers"),
        ("asset", "one sequence in the media bin"),
        ("span", "one exposure on a layer"),
        ("fx", "one effect in a layer's stack"),
    ];

    /// Words that follow a dot in this page and are not fields of the project.
    ///
    /// Every one of them is JavaScript's own - a method or a property of the language rather than
    /// of the document. Kept short deliberately: anything not on this list is treated as a field
    /// the panels read, so the mistake this list can make is letting a check through, never
    /// inventing one.
    const NOT_OURS: &[&str] = &[
        "concat", "every", "filter", "find", "forEach", "includes", "indexOf", "join", "length",
        "map", "push", "slice", "some", "sort", "split", "has", "trim", "toFixed", "value",
    ];

    /// Every `<var>.<field>` the page reads, for one variable name.
    ///
    /// The character in front of the name is what separates a field read from a command
    /// identifier: `layer.move_up` inside `onSelected('layer.move_up')` follows a quote, and
    /// `'/layer.create?asset='` follows a slash. Neither is a field.
    fn fields_read(page: &str, var: &str) -> Vec<String> {
        let needle = format!("{var}.");
        let mut found = Vec::new();
        for (at, _) in page.match_indices(&needle) {
            let before = page[..at].chars().next_back().unwrap_or(' ');
            if before == '/'
                || before == '\''
                || before == '.'
                || before == '_'
                || before.is_alphanumeric()
            {
                continue;
            }
            let rest = &page[at + needle.len()..];
            let end = rest
                .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
                .unwrap_or(rest.len());
            let field = &rest[..end];
            if field.is_empty() || NOT_OURS.contains(&field) {
                continue;
            }
            found.push(field.to_string());
        }
        sorted(found)
    }

    /// A layer of the answer, by identifier.
    fn layer_of<'a>(answer: &'a serde_json::Value, id: &str) -> &'a serde_json::Value {
        let comp = answer["project"]["compositions"]
            .as_array()
            .expect("the answer holds a list of compositions")
            .iter()
            .find(|c| c["id"] == answer["composition"])
            .expect("the composition the viewer says is on screen");
        comp["layers"]
            .as_array()
            .expect("the composition holds a list of layers")
            .iter()
            .find(|l| l["id"] == id)
            .unwrap_or_else(|| panic!("no layer {id} in the composition"))
    }

    #[test]
    fn every_field_the_panels_read_is_in_the_answer_the_window_gives() {
        let mut report = Report { rows: Vec::new() };
        let page = page();
        let viewer = Mutex::new(demo());
        // The reference shot carries no effect and no matte, because nothing in it needs one, and
        // those are the two panels most likely to be reading a name that has moved. They are put
        // there with the same commands a person would use.
        for kind in ["core.gaussian_blur", "core.exposure", "core.tint"] {
            run(&viewer, &format!("effect.add?layer=layer-3&type={kind}"));
        }
        run(
            &viewer,
            "layer.set_matte?layer=layer-3&matte=layer-2&only=false",
        );
        let answer: serde_json::Value =
            serde_json::from_str(&state(&viewer)).expect("the state answer is JSON");
        let layer = layer_of(&answer, "layer-3");

        let node = |var: &str| -> serde_json::Value {
            match var {
                "doc" => answer.clone(),
                "comp" => {
                    let mut comp = answer["project"]["compositions"]
                        .as_array()
                        .expect("compositions")
                        .iter()
                        .find(|c| c["id"] == answer["composition"])
                        .expect("the composition on screen")
                        .clone();
                    // The page reads `comp.layers`, which is there; nothing else of the
                    // composition is read by name today.
                    comp["layers"] = comp["layers"].clone();
                    comp
                }
                "layer" => layer.clone(),
                "asset" => answer["project"]["assets"][0].clone(),
                "span" => layer["exposure_spans"][0].clone(),
                "fx" => layer["effects"][0].clone(),
                other => panic!("no node for {other}"),
            }
        };

        for (var, where_from) in READ_FROM {
            let holds = node(var);
            let fields = fields_read(&page, var);
            report.check(
                &format!(
                    "the panels read {} field{} out of {where_from}",
                    fields.len(),
                    match fields.len() {
                        1 => "",
                        _ => "s",
                    }
                ),
                true,
                !fields.is_empty(),
            );
            for field in fields {
                report.check(
                    &format!("`{var}.{field}` is in the answer"),
                    "present",
                    match holds.get(&field) {
                        Some(_) => "present".to_string(),
                        None => format!("missing - the panel would draw nothing for {field}"),
                    },
                );
            }
        }

        // Three reads the loop above cannot see, because they are one level further in and the
        // page reaches them through a local name rather than through `layer`.
        for prop in ["anchor", "position", "scale", "rotation", "opacity"] {
            report.check(
                &format!("`layer.transform.{prop}.base` is in the answer"),
                "present",
                match layer["transform"][prop].get("base") {
                    Some(_) => "present",
                    None => "missing - the inspector would show an empty field",
                },
            );
        }
        for field in ["layer_id", "matte_only"] {
            report.check(
                &format!("`layer.matte.{field}` is in the answer"),
                "present",
                match layer["matte"].get(field) {
                    Some(_) => "present",
                    None => "missing - the matte row would forget which layer it is",
                },
            );
        }

        // The mask, which no command in this build can make: the panel says how many points a
        // layer's mask has, and the only way to have one is to open a project that carries one.
        // A wrong name here would not be a blank field, it would be `undefined points`.
        let with_a_mask = std::fs::read_to_string(repo("Fixtures/projects/cel_holds_project.json"))
            .expect("read the cel-holds fixture")
            .replace(
                "\"mask\": null",
                "\"mask\": { \"vertices\": [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]], \
                 \"enabled\": true, \"inverted\": false }",
            );
        let loaded = persist::load_str(&with_a_mask).expect("a project carrying a mask opens");
        let written: serde_json::Value = serde_json::from_str(&persist::to_json(
            loaded.document.project(),
            &loaded.preserved,
        ))
        .expect("what a save would write is JSON");
        report.check(
            "`layer.mask.vertices` is in a saved project that has a mask",
            "present",
            match written["compositions"][0]["layers"][0]["mask"].get("vertices") {
                Some(_) => "present",
                None => "missing - the inspector would say `undefined points`",
            },
        );

        // The effect panel's own vocabulary. These are not fields of the answer, they are the
        // names the panel puts in a request, and a name the command does not read is a field a
        // person can type into that changes nothing.
        for (type_id, params) in EFFECT_KINDS {
            report.check(
                &format!("the panel's `{type_id}` is an effect this build has"),
                "added",
                match run(&viewer, &format!("effect.add?layer=layer-4&type={type_id}")) {
                    said if said.starts_with("Add ") => "added".to_string(),
                    said => said,
                },
            );
            let effects = layer_of(
                &serde_json::from_str::<serde_json::Value>(&state(&viewer)).expect("JSON"),
                "layer-4",
            )["effects"]
                .clone();
            let instance = effects[effects.as_array().map_or(0, |e| e.len() - 1)]["instance_id"]
                .as_str()
                .expect("the effect just added has an identifier")
                .to_string();
            let sent = params
                .iter()
                .map(|(name, value)| format!("&{name}={value}"))
                .collect::<String>();
            report.check(
                &format!(
                    "and the settings it sends for it - {} - are the ones the command reads",
                    params
                        .iter()
                        .map(|(name, _)| *name)
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                "accepted",
                match run(
                    &viewer,
                    &format!("effect.set_parameters?layer=layer-4&effect={instance}{sent}"),
                ) {
                    said if said.starts_with("Set ") || said.starts_with("Change ") => {
                        "accepted".to_string()
                    }
                    said => said,
                },
            );
        }

        write_artifact(
            &report,
            "verification/B-12b_state_fields_table.md",
            "B-12b: what the panels read, against what the window sends them",
            FIELDS_INTRO,
            FIELDS_NOTES,
        );
        let failed: Vec<&String> = report
            .rows
            .iter()
            .filter(|(_, e, a)| e != a)
            .map(|(c, _, _)| c)
            .collect();
        assert!(failed.is_empty(), "these checks failed: {failed:#?}");
    }

    /// The effect identifiers the page's own tables name, and one full set of settings for each.
    ///
    /// Read off `EFFECT_PARAMS` and `EFFECT_NAMES` in the page by eye rather than by parser: this
    /// is three effects and the parser to read a JavaScript object literal would be longer than
    /// the list. The check is that the command reads these names, so an effect renamed on one
    /// side and not the other fails here.
    const EFFECT_KINDS: &[(&str, &[(&str, &str)])] = &[
        ("core.gaussian_blur", &[("sigma_px", "4")]),
        ("core.exposure", &[("stops", "0.5")]),
        (
            "core.tint",
            &[("color", "0.9, 0.7, 0.5"), ("amount", "0.3")],
        ),
    ];

    const FIELDS_INTRO: &[&str] = &[
        "The five panels draw themselves out of one JSON answer, `/state`, whose project half is \
         the same text a save writes. Nothing has ever checked that the names the panels read are \
         the names the window sends. A field renamed on one side is a panel that draws a blank \
         space: no error, no status line, nothing in the log, and a person who assumes the layer \
         simply has no name.",
        "This is that check, and the names are read out of `app/ui/index.html` itself rather than \
         written down here. Every `doc.`, `comp.`, `layer.`, `asset.`, `span.` and `fx.` the page \
         reads is looked for in a real answer from a real project, with an effect stack and a \
         matte added by command first, because the reference shot has neither.",
    ];

    const FIELDS_NOTES: &[&str] = &[
        "## What to look at\n\n- **`missing`** in the Actual column is the failure this table \
         exists to catch, and it names the field. Every other row is a name the panels read and \
         the window sends.\n- **The transform, matte and mask rows are one level further in.** \
         The page reaches those through a local name rather than through `layer`, so the scan \
         cannot see them and they are asked for by name.\n- **The last six rows are the other \
         direction**: not what the panel reads, but what it sends. The effect panel builds a \
         request out of the parameter names in its own table, so a setting renamed in the core \
         and not in the page is a field a person can type into that changes nothing.",
        "## What this cannot cover\n\nThat the panel draws the value correctly once it has it. \
         This says the name resolves, not that the number is put in the right box, formatted the \
         right way, or updated when it changes. `verification/B-12a_window_and_keyboard.md` and \
         the photographs are what say that, and the owner's run under B-12 is what says it for \
         real.\n\nIt also cannot see a field that is present and always null. `mask` is null on \
         every layer of the reference shot, which is why the mask row opens a project that \
         carries one.",
    ];

    // ---- the routes the shell owns ---------------------------------------------------------

    #[test]
    fn the_windows_own_routes_are_not_swallowed_on_the_way_past() {
        let mut report = Report { rows: Vec::new() };
        let source = source();
        let viewer = Mutex::new(demo());

        // The defect this test exists for. `fn command` offers every request to `edit_command`
        // first and reads `None` as permission to try its own routes, so an `edit_command` that
        // answers something it has never heard of takes Open, Save, Save As, Export, the recent
        // list and recovery with it. That is what happened, it reached a photograph, and every
        // test that saves calls `save` directly - only the window goes through this path.
        for route in ROUTES {
            if *route == "frame" || *route == "at" {
                continue;
            }
            report.check(
                &format!("`/{route}` is left alone by the command layer"),
                "not mine - the shell answers it",
                match edit_command(&viewer, route, None) {
                    None => "not mine - the shell answers it".to_string(),
                    Some(said) => format!("swallowed, and answered: {said}"),
                },
            );
        }

        // A word that is neither. Nothing may claim it, or the shell's 404 - the only thing that
        // tells a page it asked for something that does not exist - never happens.
        for nonsense in ["layer.remove", "save-it", "", "state/../save"] {
            report.check(
                &format!("`/{nonsense}`, which is nothing this window has, is refused"),
                "not mine",
                match edit_command(&viewer, nonsense, None) {
                    None => "not mine".to_string(),
                    Some(said) => format!("answered: {said}"),
                },
            );
        }

        // The sentence a 404 carries is the only list of routes a person or a page ever sees, and
        // it is written by hand. This is the one check that it still names what the shell has.
        let listed: Vec<String> = {
            let at = source
                .find("ask for /state,")
                .expect("the not-found answer is in this file");
            let rest = &source[at..];
            sorted(after(
                &rest[..rest.find("command IDs").unwrap_or(rest.len())],
                "/",
            ))
        };
        let shell: Vec<String> = ROUTES
            .iter()
            .filter(|route| !matches!(**route, "frame" | "at" | "play" | "boxes" | "curve"))
            .map(|route| route.to_string())
            .collect();
        report.check(
            "the answer to a route that does not exist names the routes that do",
            shell.join(", "),
            listed.join(", "),
        );

        // Query strings, which every command in this window arrives with. The page builds them
        // with `encodeURIComponent`, so what comes back has to be what was typed.
        let cases: &[(&str, &str, &str)] = &[
            ("layer=layer%201", "layer", "layer 1"),
            ("name=a%26b", "name", "a&b"),
            ("name=%E7%8C%AB", "name", "猫"),
            ("layers=1&layer=2", "layer", "2"),
            // A name that ends with the name being asked for. `matte_layer` is a real one, and
            // a search that looks for `layer=` anywhere in a pair rather than at its start
            // answers this with the wrong sequence's identifier.
            ("matte_layer=layer-9&layer=layer-2", "layer", "layer-2"),
            ("start=12&drawing=", "drawing", ""),
        ];
        for (query, name, expected) in cases {
            report.check(
                &format!("`?{query}` gives `{name}`"),
                *expected,
                parameter(Some(query), name).unwrap_or_else(|| "(nothing)".to_string()),
            );
        }
        report.check(
            "and a name that is not in the query gives nothing at all",
            "(nothing)",
            parameter(Some("layer=layer-1"), "name").unwrap_or_else(|| "(nothing)".to_string()),
        );

        // The one query string in this file that decides what reaches the disk.
        for (query, expected) in [
            (Some("missing=write"), "RenderTransparent"),
            (Some("missing=Write"), "Block"),
            (Some("missing=1"), "Block"),
            (Some("missing="), "Block"),
            (None, "Block"),
        ] {
            report.check(
                &format!(
                    "what an export asked for with `{}` does with a missing drawing",
                    query.unwrap_or("no query at all")
                ),
                expected,
                format!("{:?}", missing_source(query)),
            );
        }

        write_artifact(
            &report,
            "verification/B-12b_routes_table.md",
            "B-12b: the routes the window answers itself",
            ROUTES_INTRO,
            ROUTES_NOTES,
        );
        let failed: Vec<&String> = report
            .rows
            .iter()
            .filter(|(_, e, a)| e != a)
            .map(|(c, _, _)| c)
            .collect();
        assert!(failed.is_empty(), "these checks failed: {failed:#?}");
    }

    const ROUTES_INTRO: &[&str] = &[
        "Every request from the page arrives at one function, which offers it to the command \
         layer first and answers it itself if the command layer has never heard of it. That \
         hand-off is the whole of the window's routing, and on 2026-09-07 it was broken: the \
         command layer answered everything, including identifiers it did not have, so Open, \
         Save, Save As, Export, the recent list and recovery were all swallowed on the way past. \
         The editing window worked perfectly and the project could not be written.",
        "No test saw it, because every test that saves calls the save function directly and only \
         the window goes through the hand-off. This table is that path: for each route the shell \
         owns, that the command layer leaves it alone; that a word neither of them has is refused \
         by both; that the sentence a person gets when they ask for something that does not exist \
         still names what does; and that a query string is read back as what was typed into it.",
    ];

    const ROUTES_NOTES: &[&str] = &[
        "## What to look at\n\n- **`swallowed, and answered:`** in the Actual column is the \
         defect this table was written for, and the sentence beside it is what the person would \
         have got instead of their project being saved.\n- **The empty route** is in the list on \
         purpose. A page can ask for `/` and something has to refuse it.\n- **`missing=Write` \
         blocks.** The override that writes frames with drawings missing is exactly the word \
         `write`; everything else takes document 07's default and refuses the job. A comparison \
         that ignored case would let a typo become an export nobody asked for.",
        "## What this cannot cover\n\nThe three routes that open a dialog. Import, Save As and \
         Export hand the request to Windows before anything of ours runs, and no test in this \
         project has hands to answer a file dialog. What is checked here is that the request \
         reaches the shell at all; what happens after the dialog is \
         `verification/B-09_save_table.md` and `verification/B-10_export_table.md`.\n\nAnd the \
         function itself. It needs a running application to be called, so what is checked is the \
         decision it makes rather than the response it builds: the hand-off, the list, the query \
         reading and the export override are each called directly.",
    ];

    // ---- document 05 line 69, the keyboard claim ----------------------------------------------

    /// The name a `$('id')` in the page reaches for, where that lookup is followed by something
    /// that makes it a control: a handler, a synthetic click, or a checkbox being read.
    fn controls(page: &str) -> Vec<String> {
        page.match_indices("$('")
            .filter_map(|(at, _)| {
                let rest = &page[at + 3..];
                let end = rest.find('\'')?;
                let after = &rest[end + 2..];
                let is_a_control = after.starts_with(".on")
                    || after.starts_with(".click")
                    || after.starts_with(".checked");
                is_a_control.then(|| rest[..end].to_string())
            })
            .collect()
    }

    /// The tag the markup gives an element, read backwards from its identifier.
    fn tag_of(page: &str, id: &str) -> String {
        let needle = format!("id=\"{id}\"");
        match page.find(&needle) {
            None => "not in the markup".to_string(),
            Some(at) => {
                let before = &page[..at];
                let open = before.rfind('<').map(|o| &before[o + 1..]).unwrap_or("");
                open.split(|c: char| c.is_whitespace() || c == '>')
                    .next()
                    .unwrap_or("")
                    .to_string()
            }
        }
    }

    /// The page with every mouse-only handler cut out of it.
    ///
    /// A double click and a pointer drag are the two gestures a keyboard cannot make. Whatever
    /// a handler for one of them sends is unreachable without a mouse unless the same request is
    /// sent from somewhere else as well, so the question "can this be done without a mouse" is
    /// the question "does this identifier still appear once those handlers are gone".
    ///
    /// A handler is cut from its name to the end of its statement, which is the first semicolon
    /// at depth zero - handlers here are written both as a braced body and as a one-line arrow.
    fn without_the_mouse(page: &str) -> String {
        const MOUSE: [&str; 4] = [
            "ondblclick",
            "onpointerdown",
            "onpointermove",
            "onpointerup",
        ];
        let mut out = String::with_capacity(page.len());
        let bytes: Vec<char> = page.chars().collect();
        let mut i = 0;
        while i < bytes.len() {
            let rest: String = bytes[i..].iter().take(20).collect();
            match MOUSE.iter().find(|m| rest.starts_with(**m)) {
                None => {
                    out.push(bytes[i]);
                    i += 1;
                }
                Some(_) => {
                    let mut depth = 0i32;
                    while i < bytes.len() {
                        match bytes[i] {
                            '(' | '{' | '[' => depth += 1,
                            ')' | '}' | ']' => depth -= 1,
                            ';' if depth <= 0 => {
                                i += 1;
                                break;
                            }
                            _ => {}
                        }
                        i += 1;
                    }
                }
            }
        }
        out
    }

    /// The keys the window answers when nothing in particular is focused, as the page writes
    /// them: every `e.key === '...'` in the one global handler.
    fn key_names(page: &str) -> Vec<String> {
        let block = accelerators(page);
        let block = block.as_str();
        let mut keys: Vec<String> = block
            .match_indices("e.key === '")
            .filter_map(|(at, _)| {
                let rest = &block[at + 11..];
                let end = rest.find('\'')?;
                Some(rest[..end].to_string())
            })
            // A letter is written in the page in both cases and is one key; anything longer
            // is already the name of a key and is left as it is.
            .map(|key| match key.as_str() {
                " " => "Space".to_string(),
                one if one.chars().count() == 1 => one.to_uppercase(),
                other => other.to_string(),
            })
            .collect();
        keys.sort();
        keys.dedup();
        keys
    }

    /// Document 05 line 69: "complete W-01 without assistance, with all required controls
    /// reachable by keyboard."
    ///
    /// Every other artifact in B-12 checks what a request does. This one checks that a person
    /// with no mouse can make the request at all, by reading the page the way the claim is
    /// worded: which controls exist, whether each is a thing the Tab key stops at, which keys
    /// the window answers on its own, and - the row this table is really for - whether every
    /// command the page can send survives having the mouse-only handlers cut out of it.
    ///
    /// Writes `verification/B-12c_keyboard_table.md`.
    #[test]
    fn every_command_the_page_sends_can_be_asked_for_without_a_mouse() {
        let page = page();
        let mut report = Report { rows: Vec::new() };

        let mut named = controls(&page);
        named.sort();
        named.dedup();
        report.check(
            "the controls the page wires are the ones written down here",
            CONTROLS.join(", "),
            named.join(", "),
        );
        // A button, a select and a checkbox are stops on the Tab order because the browser makes
        // them so. A span or a div with a click handler is not, and that is the shape this row
        // exists to catch.
        let not_focusable: Vec<String> = named
            .iter()
            .map(|id| (id, tag_of(&page, id)))
            .filter(|(_, tag)| !matches!(tag.as_str(), "button" | "select" | "input"))
            .map(|(id, tag)| format!("{id} is a {tag}"))
            .collect();
        report.check(
            "and every one of them is a control the Tab key stops at on its own",
            "none of them is anything else",
            match not_focusable.is_empty() {
                true => "none of them is anything else".to_string(),
                false => not_focusable.join(", "),
            },
        );

        // The two lists are built out of `li`, which nothing focuses by itself.
        for (what, marker) in [
            (
                "a row in the media bin or the layer list",
                "  li.tabIndex = 0;",
            ),
            ("the number a drag changes", "  handle.tabIndex = 0;"),
        ] {
            report.check(
                &format!("{what} is put into the Tab order by hand"),
                true,
                page.contains(marker),
            );
        }
        report.check(
            "and a focused row is chosen with Enter or Space, which is what a click does",
            true,
            page.contains(
                "li.dispatchEvent(new MouseEvent('click', { bubbles: true, shiftKey: e.shiftKey }));",
            ),
        );
        report.check(
            "and a focused drag handle is moved with the arrow keys, which is what a drag does",
            true,
            page.contains("const by = { ArrowLeft: -1, ArrowRight: 1, ArrowDown: -1, ArrowUp: 1 }"),
        );

        report.check(
            "the keys the window answers with nothing focused are the ones written down here",
            KEYS.join(", "),
            key_names(&page).join(", "),
        );

        // The row this table is for. Everything the page can ask the window to do, asked of a
        // copy of the page with the double clicks and the pointer drags cut out.
        let without = asked_for(&without_the_mouse(&page));
        for id in asked_for(&page) {
            if matches!(id.as_str(), "frame" | "at" | "state") {
                continue; // Not a control: the page asks for these itself, to draw with.
            }
            let reachable = without.contains(&id);
            report.check(
                &format!("`{id}` can be asked for without a mouse"),
                match MOUSE_ONLY.contains(&id.as_str()) {
                    true => "no - it is a drag, and a drag is a mouse",
                    false => "yes",
                },
                match reachable {
                    true => "yes",
                    false => "no - it is a drag, and a drag is a mouse",
                },
            );
        }

        // Every gesture that needs a mouse, each with the thing that does the same job without one.
        for (gesture, does, instead) in MOUSE_GESTURES {
            report.check(
                &format!("{gesture} is not the only way to {does}"),
                true,
                page.contains(instead),
            );
        }

        write_artifact(
            &report,
            "verification/B-12c_keyboard_table.md",
            "B-12c: the keyboard claim, and what is mouse-only",
            KEYBOARD_INTRO,
            KEYBOARD_NOTES,
        );
        assert!(
            report.rows.iter().all(|(_, e, a)| e == a),
            "the keyboard claim does not hold: {:?}",
            report
                .rows
                .iter()
                .filter(|(_, e, a)| e != a)
                .collect::<Vec<_>>()
        );
    }

    /// Every control the page wires a handler to, or clicks for the person, or reads.
    const CONTROLS: [&str; 35] = [
        "addeffect",
        "addexposure",
        "addlayer",
        "alpha",
        "anyway",
        "applyrelink",
        "back",
        "cancelcomp",
        "cancelexport",
        "cancelrelink",
        "checker",
        "dellayer",
        "down",
        "export",
        "fit",
        "fit100",
        "fwd",
        "graphprop",
        "import",
        "makecomp",
        "newcomp",
        "open",
        "play",
        "recent",
        "recovery",
        "redo",
        "relink",
        "save",
        "saveas",
        "tabgraph",
        "tabsheet",
        "toggle",
        "undo",
        "up",
        "zoomer",
    ];

    /// Document 24's shortcuts, as keys rather than as chords: the modifiers live in the same
    /// branch as the key and `verification/B-12b_command_map_table.md` is what checks the pair.
    const KEYS: [&str; 23] = [
        "1",
        "?",
        "A",
        "ArrowLeft",
        "ArrowRight",
        "D",
        "Delete",
        "F2",
        "G",
        "I",
        "L",
        "M",
        "N",
        "O",
        "P",
        "R",
        "S",
        "Space",
        "T",
        "U",
        "Z",
        "[",
        "]",
    ];

    /// The identifiers that are a drag and cannot be anything else.
    ///
    /// It was the three `property.drag_*` identifiers until W-04. A nudge with the arrow keys
    /// now moves every selected layer, and one press moving three layers has to be one thing
    /// to undo, so the nudge opens and commits the same transaction a drag does:
    /// `property.drag_update` once per layer and then `property.drag_end`. The cancel was the
    /// last of them, and W-07 took it too: an effect's setting is sent as it is typed inside
    /// that same transaction, and Escape in the field abandons it.
    ///
    /// `effect.move` is what is left, since W-07. It is the drop at the end of a card dragged
    /// up or down the stack, and the two arrows on the card send `effect.move_up` and
    /// `effect.move_down` to the same end a step at a time, which is the pairing the rows
    /// below are for.
    const MOUSE_ONLY: [&str; 1] = ["effect.move"];

    /// A mouse gesture, what it does, and the text in the page that does the same job without one.
    const MOUSE_GESTURES: [(&str, &str, &str); 16] = [
        (
            "dragging the border between two panels",
            "give one of them more of the window",
            "split.onkeydown = (e) => {",
        ),
        (
            "double clicking a drawing sequence in the media bin",
            "make a layer out of it",
            "$('addlayer').click();",
        ),
        (
            "double clicking a layer",
            "rename it",
            "else if (e.key === 'F2') { e.preventDefault(); beginRename(); }",
        ),
        (
            "dragging a transform value",
            "change it",
            "held += by * step * (e.shiftKey ? 10 : 1);",
        ),
        (
            "dragging a layer on the picture",
            "move it",
            "const by = { ArrowLeft: [-1, 0], ArrowRight: [1, 0],",
        ),
        (
            "pulling a corner or the rotation arm on the picture",
            "scale or turn the layer",
            "held += by * step * (e.shiftKey ? 10 : 1);",
        ),
        (
            "dragging along the ruler",
            "go to a frame",
            "else if (e.key === 'ArrowRight') step(1);",
        ),
        (
            "dragging a layer's bar along the exposure sheet",
            "move the layer in time",
            "e.key === '[' || e.key === ']'",
        ),
        (
            "pulling either end of a layer's bar",
            "trim it",
            "e.key === '[' || e.key === ']'",
        ),
        (
            "dragging the seam between two exposure blocks",
            "retime the exposures on either side of it",
            "commit: sendSpan,",
        ),
        (
            "dragging a box across the picture",
            "select several layers",
            "shiftKey: e.shiftKey",
        ),
        (
            "dragging the anchor mark",
            "move the anchor",
            "propRow(dl, layer, 'anchor'",
        ),
        (
            "turning the wheel over the picture",
            "zoom",
            "e.key === '1'",
        ),
        (
            "dragging the handle beside an effect's setting",
            "change the setting",
            "held += by * step * (e.shiftKey ? 10 : 1);",
        ),
        (
            "moving over the tint's colour picker",
            "choose the colour",
            "fields[name].push(input);",
        ),
        (
            "dragging an effect's name up or down the stack",
            "reorder the effects",
            "command('/effect.move_up' + where)",
        ),
    ];

    const KEYBOARD_INTRO: &[&str] = &[
        "Document 05 line 69 sets B-12's bar: \"complete W-01 without assistance, with all \
         required controls reachable by keyboard.\" `verification/B-11_display_and_keyboard.md` \
         is a photograph of the Tab key reaching the Open button, which says the order exists \
         and that the control holding it is visible. This is the other half, and it is the half \
         a photograph cannot take: whether there is any command in this window that a person \
         without a mouse cannot ask for.",
        "It reads `app/ui/index.html`. The question it asks is not \"is there a keyboard \
         shortcut\" but \"is this request sent from anywhere that is not a mouse gesture\", and \
         it asks it by cutting every double-click and pointer-drag handler out of a copy of the \
         page and looking for the request in what is left.",
    ];

    const KEYBOARD_NOTES: &[&str] = &[
        "## What to look at\n\nThe one row that says **no**. `effect.move` is the drop of a \
         card dragged up or down the effect stack, and the arrows on the card do the same job a \
         step at a time. It used to be `property.drag_cancel`, Escape during a drag, and before \
         W-04 three: `property.drag_update` and `property.drag_end` are the running transaction \
         and the coalescing document 26 asks for, and until W-04 only a pointer opened one. \
         Nudging the picture with the arrow keys now moves every selected layer, and one press \
         moving three layers has to be one thing to undo, so it opens and commits that same \
         transaction; and since W-07 an effect's setting is sent as it is typed inside that same \
         transaction, so Escape in the field abandons it, which is the cancel without a pointer. \
         The rest of the keyboard changes numbers rather than dragging them, and the last sixteen rows \
         are the mouse gestures in this \r
         window each paired with the thing that does the same job without one: the arrow keys on a \r
         focused handle send `property.set_base`, which is one undo step per press rather than one per \r
         drag. The picture itself works the same way: a layer is dragged, and it is nudged by the \r
         arrow keys while the canvas holds the focus; a corner handle scales and the rotation arm \r
         turns, and both numbers are typed or stepped in the inspector; the playhead is dragged along \r
         the exposure sheet, and it is stepped by the arrow keys everywhere the canvas does not hold \r
         the focus. Since W-06 a box dragged across the picture selects the layers inside it, and \
         Shift with Space or Enter on a row in the layer list adds that row the way Shift with a \
         click does; the anchor mark is dragged, and its two numbers are in the inspector like \
         the rest; and the wheel zooms, where Ctrl+1 and Shift+/ set the two zooms document 24 \
         names. Since W-07 every value box in the effects panel has the handle the transform rows \
         have, and its arrow keys step the value the same way; the tint's colour picker sets the \
         three numbers beside it, which are typed like any other; and an effect card is dragged \
         up or down the stack, where the arrows on it move it a step at a time. Since W-12 the \
         After Effects property keys A, P, S, R, T and U open a selected layer's properties under \
         it on the timeline, Shift with a key adds one, and the arrow beside the layer's name is \
         a button that opens all five; alpha-only inspection moved from A to Alt+A to make room.          Since W-13 the borders between the panels are dragged, and each border is a stop on the          Tab order that the arrow keys move and Home puts back.",
        "The two lists are the part that had to be built rather than inherited. Every button \
         here is a `button` and every chooser a `select`, so the Tab order is the browser's and \
         nothing had to be arranged; the rows of the media bin and the layer list are `li` \
         elements, which nothing focuses, and they carry a tab stop and an Enter/Space handler \
         put there by hand. The row that checks the list of controls is a list rather than a \
         count for the same reason as the one in `verification/B-12b_page_table.md`: a control \
         added tomorrow fails this table until somebody writes it down beside the others, and \
         the row underneath then asks what kind of element it is.",
        "## What this cannot cover\n\nThat the Tab order is a sensible one. It says every \
         control is in it; it does not say the order walks the window in the order a person \
         reads it, and nothing but a person pressing Tab can say that. The photograph in \
         `verification/B-11_display_and_keyboard.md` counts six presses to Open, which is one \
         sample of it.",
        "Whether a focused control can be seen. The focus ring is a colour in a stylesheet, and \
         this reads the page as text. That is the photograph's job as well.",
        "And the file dialogs. Import, Open, Save As and Export hand over to Windows, which \
         brings its own keyboard handling and is not this project's to check.",
    ];

    fn cell(text: &str) -> String {
        text.replace('|', r"\|")
    }
}

/// W-01 walked end to end, from an empty composition to an exported sequence.
///
/// B-12 is the owner's acceptance run of this workflow by hand. This is the same thirteen steps
/// driven by script first, for one reason: a defect found here costs a re-run of a test, and the
/// same defect found in the middle of the owner's run costs the run. It is not a substitute for
/// it -- a script cannot say whether the window was pleasant to use, whether a sentence read
/// well, or whether the picture looked right -- and it does not touch the page, which is
/// `verification/B-12b_page_table.md` and the photographs beside it.
///
/// Every step goes through the same function the window's URL scheme calls, with the same text
/// the page would put in it, so what is walked is the window from the request inwards.
///
/// Writes `verification/B-12b_w01_walkthrough.md` and `verification/B-12b_w01_frame_12.png`,
/// and leaves the exported sequence in `target/b12b_w01/frames`.
#[cfg(test)]
mod acceptance {
    use super::*;
    use anime_compositor::time::ExposureMap;

    struct Report {
        rows: Vec<(String, String, String)>,
    }

    impl Report {
        fn check(&mut self, check: &str, expected: impl ToString, actual: impl ToString) {
            self.rows
                .push((check.to_string(), expected.to_string(), actual.to_string()));
        }
    }

    fn repo(rel: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("the app crate has a parent directory")
            .join(rel)
    }

    /// The walkthrough's own directory under `target/`, emptied first so that a previous run's
    /// frames cannot make a later one pass. Under `target/` rather than the system temporary
    /// directory because the frames are half the artifact: they are meant to be opened.
    fn workspace(name: &str) -> PathBuf {
        let directory = repo("target/b12b_w01").join(name);
        let _ = std::fs::remove_dir_all(&directory);
        std::fs::create_dir_all(&directory).expect("make the walkthrough directory");
        directory
    }

    /// One command, by the document 24 identifier the page would send.
    fn run(viewer: &Mutex<Viewer>, what: &str) -> String {
        let (id, query) = match what.split_once('?') {
            Some((id, query)) => (id, Some(query)),
            None => (what, None),
        };
        edit_command(viewer, id, query).expect("a command document 24 lists")
    }

    fn held(viewer: &Mutex<Viewer>) -> std::sync::MutexGuard<'_, Viewer> {
        viewer.lock().expect("the viewer lock was poisoned")
    }

    /// The phrase asked for, when it was in what the window said, and everything the window did
    /// say when it was not. The exact wording of a diagnostic belongs to the core and is pinned
    /// there; what matters here is that the person was told the thing at all.
    fn says(phrase: &str, said: &str) -> String {
        match said.contains(phrase) {
            true => phrase.to_string(),
            false => format!("the window said: {said}"),
        }
    }

    /// The files in a directory, in name order.
    fn written(dir: &Path) -> Vec<String> {
        let mut names: Vec<String> = std::fs::read_dir(dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .map(|e| e.file_name().to_string_lossy().into_owned())
                    .collect()
            })
            .unwrap_or_default();
        names.sort();
        names
    }

    fn layer<T>(viewer: &Mutex<Viewer>, id: &str, read: impl FnOnce(&Layer) -> T) -> Option<T> {
        let held = held(viewer);
        let comp = held
            .document
            .project()
            .composition(&held.composition)
            .expect("the composition on screen");
        comp.layer(&Id::new(id)).map(read)
    }

    /// The layers of the composition, back to front, as `name(identifier)`.
    fn stack(viewer: &Mutex<Viewer>) -> String {
        let held = held(viewer);
        let comp = held
            .document
            .project()
            .composition(&held.composition)
            .expect("the composition on screen");
        comp.layers_in_order()
            .map(|l| format!("{}({})", l.name, l.id))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn exposures(viewer: &Mutex<Viewer>, layer_id: &str) -> String {
        layer(viewer, layer_id, |l| {
            l.exposure_spans
                .iter()
                .map(|s| {
                    format!(
                        "{}-{}:{}",
                        s.start_frame, s.end_frame_exclusive, s.drawing_number
                    )
                })
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_else(|| "(no such layer)".to_string())
    }

    /// Which drawing of its sequence a layer shows at every frame of the composition, read the
    /// way the renderer reads it -- through `ExposureMap` and the layer's own timing -- rather
    /// than by re-reading the spans that were typed in. This is W-01's acceptance line, "cel
    /// identity per frame matches the exposure reference", and [`CEL_REFERENCE`] is the
    /// reference.
    fn cel_identity(viewer: &Mutex<Viewer>, layer_id: &str, frames: i32) -> String {
        let Some((spans, timing)) =
            layer(viewer, layer_id, |l| (l.exposure_spans.clone(), l.timing()))
        else {
            return "(no such layer)".to_string();
        };
        let map = match ExposureMap::new(spans) {
            Ok(map) => map,
            // Document 20's rules about spans are the core's, and a sheet that breaks one is a
            // failed row here rather than a panic: the table is the thing that has to say so.
            Err(error) => return format!("(not a legal sheet: {error:?})"),
        };
        (0..frames)
            .map(|frame| match timing.local_frame(frame) {
                None => "-".to_string(),
                Some(local) => match map.drawing_at(local) {
                    None => "-".to_string(),
                    Some(number) => number.to_string(),
                },
            })
            .collect::<Vec<_>>()
            .join(",")
    }

    /// The effect stack of a layer, in evaluation order.
    fn effects(viewer: &Mutex<Viewer>, layer_id: &str) -> String {
        layer(viewer, layer_id, |l| {
            l.effects
                .iter()
                .map(|e| e.type_id().to_string())
                .collect::<Vec<_>>()
                .join(" then ")
        })
        .unwrap_or_else(|| "(no such layer)".to_string())
    }

    /// One number of a transform, as a number rather than as a spelling of one. `110` and
    /// `110.0` are the same value, and a table that failed on which of the two a serialiser
    /// chose would be failing on something nobody can see in a panel.
    fn number(value: &serde_json::Value) -> String {
        match value.as_f64() {
            Some(number) => format!("{number}"),
            None => value.to_string(),
        }
    }

    /// The transform of a layer, read out of the same JSON the inspector is given rather than
    /// out of the model, so that what is checked is what a person would read in the panel --
    /// including its units, which is where this walk found a defect.
    fn transform(viewer: &Mutex<Viewer>, layer_id: &str) -> String {
        let answer: serde_json::Value =
            serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
        // The composition named by `composition`, not the first one in the list. Since B-12d this
        // walk makes its own composition and the project holds two, and the page picks the same
        // way -- `doc.project.compositions.find((c) => c.id === doc.composition)`.
        let Some(layer) = answer["project"]["compositions"]
            .as_array()
            .expect("a project has compositions")
            .iter()
            .find(|c| c["id"] == answer["composition"])
            .expect("the composition on screen")["layers"]
            .as_array()
            .expect("a composition has layers")
            .iter()
            .find(|l| l["id"] == layer_id)
            .cloned()
        else {
            return "(no such layer)".to_string();
        };
        ["anchor", "position", "scale", "rotation", "opacity"]
            .iter()
            .map(|prop| {
                let base = &layer["transform"][prop]["base"];
                let numbers = match base.as_array() {
                    Some(pair) => pair.iter().map(number).collect::<Vec<_>>(),
                    None => vec![number(base)],
                };
                format!("{prop} {}", numbers.join(", "))
            })
            .collect::<Vec<_>>()
            .join("; ")
    }

    fn matte(viewer: &Mutex<Viewer>, layer_id: &str) -> String {
        layer(viewer, layer_id, |l| match &l.matte {
            None => "none".to_string(),
            Some(m) => format!("{} matte_only={}", m.layer_id, m.matte_only),
        })
        .unwrap_or_else(|| "(no such layer)".to_string())
    }

    /// The drawings of one folder of the reference shot, sorted, which is the selection a person
    /// makes in the import dialog.
    fn drawings_in(folder: &str) -> Vec<PathBuf> {
        let dir = repo(&format!("Fixtures/reference_shot/{folder}"));
        let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter(|p| p.extension().is_some_and(|x| x == "png"))
            .collect();
        files.sort();
        files
    }

    /// The request the media bin sends once the file dialog has been answered.
    fn import_of(files: &[PathBuf]) -> String {
        let parts: Vec<String> = files
            .iter()
            .map(|p| format!("file={}", for_a_header(&p.display().to_string())))
            .collect();
        format!("media.import?{}", parts.join("&"))
    }

    fn header(response: &Response<Vec<u8>>, name: &str) -> String {
        response
            .headers()
            .get(name)
            .map(|v| v.to_str().unwrap_or("(not text)").to_string())
            .unwrap_or_else(|| "(no such header)".to_string())
    }

    /// The exposure sheet this walk assigns to the cel layer, as somebody would type it into the
    /// exposure panel: start frame, end frame exclusive, drawing number. Threes and twos with
    /// two single-frame accents at the end, which is what a sheet looks like and what the
    /// on-twos fixtures in this repository do not exercise.
    const SHEET: &[(i32, i32, u32)] = &[
        (0, 3, 0),
        (3, 6, 1),
        (6, 8, 2),
        (8, 10, 3),
        (10, 13, 4),
        (13, 16, 5),
        (16, 19, 6),
        (19, 22, 7),
        (22, 23, 8),
        (23, 24, 9),
    ];

    /// The same sheet read out one composition frame at a time, written by hand from the spans
    /// above rather than from a run of the code (ADR-009). This is the exposure reference W-01's
    /// acceptance criterion compares cel identity against.
    const CEL_REFERENCE: &str = "0,0,0,1,1,1,2,2,3,3,4,4,4,5,5,5,6,6,6,7,7,7,8,9";

    /// The transform typed into the inspector at step 6, in the units the panel reads back.
    /// `app/ui/index.html` shows the project file's own numbers and sends them back unconverted,
    /// so scale is the percentage document 19 stores and opacity the nought-to-one it stores.
    const TRANSFORM_TYPED_IN: &str =
        "anchor 960, 540; position 980, 520; scale 110, 110; rotation 6; opacity 0.8";

    #[test]
    // Slow: twenty-five full-resolution previews and a twenty-four frame export. Run by name,
    // exactly as B-10's whole shot is, and its artifact is committed rather than made again on
    // every build.
    #[ignore = "renders and exports full-resolution frames; run by name"]
    fn the_thirteen_steps_of_w01_end_in_a_sequence_on_the_disk() {
        let mut report = Report { rows: Vec::new() };
        let folder = workspace("project");
        let source = repo("Fixtures/projects/minimal_project.json");
        let project_file = folder.join("my_shot.json");
        std::fs::copy(&source, &project_file).expect("copy the empty project to work in");

        let viewer = Mutex::new(
            open(&project_file)
                .unwrap_or_else(|d| panic!("open {}: {}", project_file.display(), d.message)),
        );
        let export_state = Mutex::new(Export::default());

        // ---- step 3, taken first: the composition -------------------------------------------
        // W-01 lists "create a composition" third, and until B-12d nothing in this build could
        // take that step: the composition a person worked in was whichever one their project
        // file already held. `composition.create` is that step, and the twenty-four frames at
        // 1920x1080 and 24 fps are the reference shot's own shape, so everything below happens
        // in a composition this walkthrough made rather than one it was handed.
        let made = run(
            &viewer,
            "composition.create?name=My shot&width=1920&height=1080&fps=24&frames=24",
        );
        report.check(
            "step 3: a composition, made by the command document 24 names",
            "New composition My shot, 1920x1080 at 24 fps, 24 frames",
            says(
                "New composition My shot, 1920x1080 at 24 fps, 24 frames",
                &made,
            ),
        );
        report.check(
            "and the person is told it is empty, rather than left looking at a blank frame",
            "It is empty; import drawings and add layers to fill it.",
            says(
                "It is empty; import drawings and add layers to fill it.",
                &made,
            ),
        );
        report.check(
            "and the window is showing the one that was just made, not the one the file held",
            "comp-1 My shot 1920x1080 at 24 fps, 24 frames",
            {
                let held = held(&viewer);
                let comp = held
                    .document
                    .project()
                    .composition(&held.composition)
                    .expect("the composition on screen");
                format!(
                    "{} {} {}x{} at {} fps, {} frames",
                    comp.id,
                    comp.name,
                    comp.width,
                    comp.height,
                    comp.frame_rate.numerator() / comp.frame_rate.denominator(),
                    comp.duration_frames
                )
            },
        );
        report.check(
            "and the one the file held is still there, because making one is not replacing one",
            "comp-main, comp-1",
            held(&viewer)
                .document
                .project()
                .compositions
                .iter()
                .map(|c| c.id.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        );

        // ---- step 1: import the drawings ----------------------------------------------------
        let background = run(&viewer, &import_of(&drawings_in("layer1")));
        let cel = run(&viewer, &import_of(&drawings_in("layer2")));
        let shape = run(&viewer, &import_of(&drawings_in("layer3")));
        let overlay = run(&viewer, &import_of(&drawings_in("layer4")));
        report.check(
            "step 1: four sequences imported into a project that had none",
            "asset-1, asset-2, asset-3, asset-4",
            held(&viewer)
                .document
                .project()
                .assets
                .iter()
                .map(|a| a.id.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        );
        report.check(
            "and the person is told what each one holds",
            "24 drawings, numbered 0 to 23",
            says("24 drawings, numbered 0 to 23", &cel),
        );

        // ---- step 2: grouping, and the warning about the gap --------------------------------
        report.check(
            "step 2: twenty-four files chosen at once are one sequence, not twenty-four",
            "layer2_%03d.png",
            held(&viewer)
                .document
                .project()
                .assets
                .iter()
                .find(|a| a.id == Id::new("asset-2"))
                .map(|a| a.name.clone())
                .unwrap_or_else(|| "(no asset-2)".to_string()),
        );
        report.check(
            "and the sequence with a hole in it says so at import, not at export",
            "drawing 7 is missing",
            says("drawing 7 is missing", &shape),
        );
        report.check(
            "the one-drawing sequence and the twenty-drawing one are read as they are",
            "1 drawings, numbered 0 to 0 | 20 drawings, numbered 0 to 19",
            format!(
                "{} | {}",
                says("1 drawings, numbered 0 to 0", &background),
                says("20 drawings, numbered 0 to 19", &overlay)
            ),
        );

        // ---- step 5, taken before step 4: stack the layers -----------------------------------
        // The exposures of step 4 are assigned to a layer, so the layers have to exist first.
        // W-01's order is the order a person thinks in, not an order the build enforces.
        run(&viewer, "layer.create?asset=asset-1&name=background");
        run(&viewer, "layer.create?asset=asset-2&name=cel");
        run(&viewer, "layer.create?asset=asset-4&name=overlay");
        run(&viewer, "layer.create?asset=asset-3&name=shape");
        report.check(
            "step 5: four layers, the newest at the front",
            "background(layer-1), cel(layer-2), overlay(layer-3), shape(layer-4)",
            stack(&viewer),
        );
        let said = run(&viewer, "layer.move_down?layer=layer-4");
        report.check(
            "and one of them moved back a place, which is what the reorder button sends",
            "background(layer-1), cel(layer-2), shape(layer-4), overlay(layer-3)",
            stack(&viewer),
        );
        report.check(
            "the window says what moved, in the words undo will use",
            "Move layer to position 2",
            says("Move layer to position 2", &said),
        );

        // ---- step 4: assign exposures --------------------------------------------------------
        // A layer with no exposures renders transparent (document 20), so this step is not
        // decoration: without it the export below writes twenty-four empty frames.
        for (start, end, drawing) in SHEET {
            run(
                &viewer,
                &format!(
                    "exposure.set_span?layer=layer-2&start={start}&end={end}&drawing={drawing}"
                ),
            );
        }
        report.check(
            "step 4: the cel layer holds the sheet that was typed into it",
            SHEET
                .iter()
                .map(|(s, e, d)| format!("{s}-{e}:{d}"))
                .collect::<Vec<_>>()
                .join(", "),
            exposures(&viewer, "layer-2"),
        );
        report.check(
            "and the drawing on screen at each of the twenty-four frames is the reference sheet",
            CEL_REFERENCE,
            cel_identity(&viewer, "layer-2", 24),
        );
        // The other three layers, so that every one of them draws something. The layer whose
        // sequence has a hole in it is deliberately never given drawing 7.
        run(
            &viewer,
            "exposure.set_span?layer=layer-1&start=0&end=24&drawing=0",
        );
        for (start, end, drawing) in [(0, 8, 0), (8, 16, 3), (16, 24, 9)] {
            run(
                &viewer,
                &format!(
                    "exposure.set_span?layer=layer-4&start={start}&end={end}&drawing={drawing}"
                ),
            );
        }
        for (start, end, drawing) in [(0, 12, 0), (12, 24, 10)] {
            run(
                &viewer,
                &format!(
                    "exposure.set_span?layer=layer-3&start={start}&end={end}&drawing={drawing}"
                ),
            );
        }
        report.check(
            "the sequence with the hole is given exposures that step around it",
            "0-8:0, 8-16:3, 16-24:9",
            exposures(&viewer, "layer-4"),
        );
        // And the panel says so when one does not. Undone straight away: what is being checked
        // is the warning, not a change to the shot.
        let onto_the_hole = run(
            &viewer,
            "exposure.set_span?layer=layer-4&start=8&end=16&drawing=7",
        );
        report.check(
            "and typing the missing drawing into the panel is answered there and then",
            "Drawing 7 is not in this sequence",
            says("Drawing 7 is not in this sequence", &onto_the_hole),
        );
        run(&viewer, "edit.undo");
        report.check(
            "undo puts the exposure back, so meeting the warning cost the shot nothing",
            "0-8:0, 8-16:3, 16-24:9",
            exposures(&viewer, "layer-4"),
        );

        // ---- step 6: anchors and transforms ---------------------------------------------------
        for (prop, value) in [
            ("anchor", "960, 540"),
            ("position", "980, 520"),
            ("scale", "110, 110"),
            ("rotation", "6"),
            ("opacity", "0.8"),
        ] {
            run(
                &viewer,
                &format!("property.set_base?layer=layer-3&prop={prop}&value={value}"),
            );
        }
        report.check(
            "step 6: the anchor and the transform, in the units the panel shows and sends",
            TRANSFORM_TYPED_IN,
            transform(&viewer, "layer-3"),
        );

        // ---- step 7: apply a matte ------------------------------------------------------------
        let said = run(
            &viewer,
            "layer.set_matte?layer=layer-3&matte=layer-4&only=true",
        );
        report.check(
            "step 7: the overlay is shaped by the layer under it",
            "layer-4 matte_only=true",
            matte(&viewer, "layer-3"),
        );
        report.check(
            "and the window says which layer is shaping it, and that it is a matte only",
            "Set matte to layer-4, matte only",
            says("Set matte to layer-4, matte only", &said),
        );

        // ---- steps 8 and 9: one blur and one colour operation ---------------------------------
        run(&viewer, "effect.add?layer=layer-3&type=core.gaussian_blur");
        run(
            &viewer,
            "effect.set_parameters?layer=layer-3&effect=fx-1&sigma_px=4",
        );
        run(&viewer, "effect.add?layer=layer-3&type=core.tint");
        run(
            &viewer,
            "effect.set_parameters?layer=layer-3&effect=fx-2&color=0.2,0.4,1&amount=0.5",
        );
        report.check(
            "steps 8 and 9: a blur, then a colour operation, in the order they were added",
            "core.gaussian_blur then core.tint",
            effects(&viewer, "layer-3"),
        );

        // ---- step 10: inspect the alpha -------------------------------------------------------
        let revision_before = held(&viewer).document.revision();
        let ordinary = serve(&viewer, &export_state, Ask::Frame(12), None);
        let said = run(&viewer, "viewer.toggle_alpha");
        let inspected = serve(&viewer, &export_state, Ask::Frame(12), None);
        report.check(
            "step 10: alpha-only inspection says what it is showing",
            "the picture is the alpha channel",
            says("the picture is the alpha channel", &said.to_lowercase()),
        );
        report.check(
            "and what comes back is grey everywhere, and the same size as the picture was",
            "a different picture, every pixel grey, same size",
            {
                let same_size = ordinary.body().len() == inspected.body().len();
                let changed = ordinary.body() != inspected.body();
                let grey = inspected
                    .body()
                    .chunks_exact(4)
                    .all(|p| p[0] == p[1] && p[1] == p[2]);
                match (same_size, changed, grey) {
                    (true, true, true) => {
                        "a different picture, every pixel grey, same size".to_string()
                    }
                    _ => format!("same size {same_size}, changed {changed}, all grey {grey}"),
                }
            },
        );
        report.check(
            "and looking at it changed nothing about the project, which is what makes it looking",
            format!("revision {revision_before}"),
            format!("revision {}", held(&viewer).document.revision()),
        );
        run(&viewer, "viewer.toggle_alpha");

        // ---- step 11: preview the work area ---------------------------------------------------
        let mut stepped = Vec::new();
        for frame in 0..24 {
            let response = serve(&viewer, &export_state, Ask::Frame(frame), None);
            stepped.push(format!(
                "{}:{}",
                response.status().as_u16(),
                header(&response, "x-frame")
            ));
        }
        report.check(
            "step 11: every frame of the work area answers, and answers with itself",
            (0..24)
                .map(|f| format!("200:{f}"))
                .collect::<Vec<_>>()
                .join(" "),
            stepped.join(" "),
        );
        let played = serve(&viewer, &export_state, Ask::At(0), None);
        report.check(
            "and playing rather than stepping asks the clock, which starts at the first frame",
            "200:0",
            format!(
                "{}:{}",
                played.status().as_u16(),
                header(&played, "x-frame")
            ),
        );

        // ---- step 12: save, close, reopen ------------------------------------------------------
        let said = save_as(&viewer, &project_file);
        report.check(
            "step 12: the shot is written to the file it was opened from",
            "Saved to",
            says("Saved to", &said),
        );
        report.check(
            "and the window no longer holds unsaved work",
            "false",
            held(&viewer).document.is_dirty().to_string(),
        );
        drop(viewer);
        let viewer = Mutex::new(
            open(&project_file)
                .unwrap_or_else(|d| panic!("reopen {}: {}", project_file.display(), d.message)),
        );
        report.check(
            "reopened, the layers are in the order they were left in",
            "background(layer-1), cel(layer-2), shape(layer-4), overlay(layer-3)",
            stack(&viewer),
        );
        report.check(
            "the effect order survives the reopening, which W-01 asks for by name",
            "core.gaussian_blur then core.tint",
            effects(&viewer, "layer-3"),
        );
        report.check(
            "so does the matte",
            "layer-4 matte_only=true",
            matte(&viewer, "layer-3"),
        );
        report.check(
            "so does the transform",
            TRANSFORM_TYPED_IN,
            transform(&viewer, "layer-3"),
        );
        report.check(
            "and the cel identity at every frame is still the reference sheet",
            CEL_REFERENCE,
            cel_identity(&viewer, "layer-2", 24),
        );
        report.check(
            "the reopened project had nothing to warn about",
            "[]",
            format!("{:?}", held(&viewer).notes),
        );

        // ---- step 13: export a PNG sequence -----------------------------------------------------
        let into = workspace("frames");
        let (snapshot, root, request) = {
            let held = held(&viewer);
            export_job(&held, &into, MissingSource::Block)
        };
        report.check(
            "step 13: the range offered is the whole composition, first to last inclusive",
            "0 to 23",
            format!("{} to {}", request.first_frame, request.last_frame),
        );
        let said = run_export(&snapshot, &root, &request, &AtomicBool::new(false));
        report.check(
            "the export finishes under the default missing-drawing policy and says where",
            format!("Exported 24 frames into {}.", into.display()),
            says(
                &format!("Exported 24 frames into {}.", into.display()),
                &said,
            ),
        );
        report.check(
            "twenty-four files are on the disk, named for the project and numbered from zero",
            "24 files, my_shot_0000.png to my_shot_0023.png",
            {
                let names = written(&into);
                format!(
                    "{} files, {} to {}",
                    names.len(),
                    names.first().map(String::as_str).unwrap_or("(none)"),
                    names.last().map(String::as_str).unwrap_or("(none)")
                )
            },
        );
        report.check(
            "and not one of them is empty",
            "true",
            written(&into)
                .iter()
                .all(|name| {
                    std::fs::metadata(into.join(name))
                        .map(|m| m.len() > 0)
                        .unwrap_or(false)
                })
                .to_string(),
        );

        // One frame kept beside the table, so that the end of the walk is something to look at
        // rather than a count of files. The rest stay under `target/b12b_w01/frames`.
        std::fs::copy(
            into.join("my_shot_0012.png"),
            repo("verification/B-12b_w01_frame_12.png"),
        )
        .expect("keep one exported frame");

        write_artifact(&report);
        let failed: Vec<&(String, String, String)> =
            report.rows.iter().filter(|(_, e, a)| e != a).collect();
        assert!(
            failed.is_empty(),
            "W-01 did not walk end to end: {failed:#?}"
        );
    }

    fn write_artifact(report: &Report) {
        write_report(
            report,
            "verification/B-12b_w01_walkthrough.md",
            "# W-01 walked end to end, before the owner walks it\n\nGenerated by \
             `app/src/main.rs`, module `acceptance`. Slow, so it does not run with the rest: \
             `cargo test -p anime_compositor_app the_thirteen_steps -- --ignored`.\n\n",
            INTRO,
            NOTES,
        );
    }

    /// One walkthrough's table, with its own heading, opening paragraphs and closing notes.
    fn write_report(report: &Report, path: &str, heading: &str, intro: &[&str], notes: &[&str]) {
        let passed = report.rows.iter().filter(|(_, e, a)| e == a).count();
        let mut out = String::from(heading);
        for paragraph in intro {
            out.push_str(paragraph);
            out.push_str("\n\n");
        }
        out.push_str("| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
        for (check, expected, actual) in &report.rows {
            out.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                check,
                cell(expected),
                cell(actual),
                if expected == actual { "pass" } else { "FAIL" }
            ));
        }
        out.push_str(&format!(
            "\n**{} of {} checks pass.**\n",
            passed,
            report.rows.len()
        ));
        for paragraph in notes {
            out.push('\n');
            out.push_str(paragraph);
            out.push('\n');
        }
        std::fs::write(repo(path), out).expect("write the artifact");
    }

    const INTRO: &[&str] = &[
        "B-12 is the owner's acceptance run of W-01: thirteen steps by hand, from importing \
         drawings to an exported sequence. This table is the same thirteen steps driven by \
         script first. The reason for doing it twice is arithmetic -- a defect found here costs \
         a re-run of a test, and the same defect found on step twelve of the owner's run costs \
         the run. Two were found this way. One of them, a release-blocking one, is written up in \
         `verification/B-12b_save_works.md`; the other is below.",
        "The shot is built from nothing: a copy of `Fixtures/projects/minimal_project.json`, \
         which is one empty 1920 by 1080 composition of twenty-four frames with no drawings and \
         no layers in it. Everything after that goes through the same function the window's URL \
         scheme calls, with the same text the page would put in it.",
        "**Step 3 used to be the one step that could not be walked.** W-01 says \"create a \
         composition\", and until B-12d this build had no command that made one: document 24 had \
         no `composition.create` row, and the composition somebody worked in was the one their \
         project file already held. This table recorded that as missing rather than working \
         around it. B-12d built it, and the four rows below are that step actually taken -- the \
         composition everything after step 3 happens in is one this walk made, not one it was \
         handed. The composition the file came with is still in the project, untouched, because \
         making one is not replacing one.",
    ];

    const NOTES: &[&str] = &[
        "## The defect this walk found\n\nTyping into the scale field made the layer a hundred \
         times too big. The panel is given the project file's own numbers and sends them back in \
         the same units; a scale in the file is a percentage -- `100` for full size -- while the \
         model holds the factor document 21 composes with. Nothing between the two divided by a \
         hundred. So somebody who typed the `100` the field was already showing, or who dragged \
         the label one step, got a layer a hundred times its size. It survived because every \
         earlier test of that function either set a position, where the two units are the same, \
         or checked a scale being refused. The division now happens once, where typing and \
         dragging both pass through, and the two transform rows above are what check it.\n\nOne \
         part of it is left for the owner rather than quietly reworded: the status line reports \
         what the *model* was set to, so typing 110 into scale is answered \"Set scale to (1.1, \
         1.1)\". That sentence belongs to the core, which is right about its own units, and \
         changing it is a decision rather than a fix.",
        "## What to look at\n\n- **The exposure sheet is threes and twos with two single-frame \
         accents**, not the on-twos cadence every fixture in this repository uses. The reference \
         it is checked against was written out by hand from the spans, not from a run of the \
         code, and it is compared twice: once after it is typed in, and once after the project \
         has been saved, closed and reopened. That is W-01's acceptance line.\n- **The layer \
         whose sequence has a hole in it is given exposures that step around the hole**, and the \
         row after that types the missing drawing in on purpose, to check the panel says so at \
         the moment it is typed rather than three hundred frames into an export. The undo after \
         it costs the shot nothing.\n- **The export runs under the default policy**, which \
         refuses a frame whose drawing is missing, and it finishes -- because nothing in this \
         shot exposes a drawing that is not there.",
        "## What this cannot cover\n\n- **The page.** Every row calls the request rather than \
         clicking the control that sends it. What is wired to what is \
         `verification/B-12b_page_table.md`; that it is on the screen and reachable by keyboard \
         is `verification/B-12a_window_and_keyboard.md`.\n- **The three dialogs.** Import, Save \
         As and Export each open a window Windows draws, and no script can answer one. What is \
         walked here is the request underneath.\n- **Whether it was pleasant.** A script cannot \
         say whether a sentence read well, whether the picture looked right, or whether a step \
         was hard to find. That is what B-12 is for, and it is why this is a rehearsal and not a \
         replacement.\n- **The 240-frame shot.** This composition is twenty-four frames. W-01's \
         \"exactly 240 correctly named files\" is `verification/B-10_full_shot_table.md`, which \
         exports the reference shot twice and compares the two byte for byte.",
        "## Where the frames are\n\nAll twenty-four are written to `target/b12b_w01/frames` and \
         are not cleaned between runs, so they can be opened or flipped through straight after \
         the test. One of them, frame 12, is kept beside this table as \
         `B-12b_w01_frame_12.png`: the background, the cel on its fifth drawing, and the overlay \
         blurred, tinted, moved off centre and shaped by the layer under it.",
    ];

    // ---- W-02, walked the same way ---------------------------------------------------------

    /// One sequence of the media bin, by identifier.
    fn sequence(viewer: &Mutex<Viewer>, asset: &str) -> String {
        held(viewer)
            .document
            .project()
            .assets
            .iter()
            .find(|a| a.id == Id::new(asset))
            .map(|a| {
                let mut numbers: Vec<u32> = a.frames.keys().copied().collect();
                numbers.sort_unstable();
                format!(
                    "{} {} drawings, numbered {}, drawing 0 at {}",
                    a.pattern.clone().unwrap_or_default(),
                    numbers.len(),
                    runs(&numbers),
                    a.frames
                        .get(&0)
                        .map(|f| f.replace('\\', "/"))
                        .unwrap_or_else(|| "(no drawing 0)".to_string())
                )
            })
            .unwrap_or_else(|| "(no such sequence)".to_string())
    }

    /// The drawing numbers a sequence holds, as the runs they fall into, so that a gap in the
    /// middle of eleven drawings is something a person reads rather than counts.
    fn runs(numbers: &[u32]) -> String {
        let mut out: Vec<String> = Vec::new();
        let mut at = 0;
        while at < numbers.len() {
            let mut end = at;
            while end + 1 < numbers.len() && numbers[end + 1] == numbers[end] + 1 {
                end += 1;
            }
            out.push(match end == at {
                true => numbers[at].to_string(),
                false => format!("{} to {}", numbers[at], numbers[end]),
            });
            at = end + 1;
        }
        out.join(", ")
    }

    /// A picture, short enough to put in a table cell and long enough that two different
    /// pictures cannot share one. Not a cryptographic digest: this is a table a person reads,
    /// and what it has to do is differ when the drawings differ and match when they are the
    /// same file rendered twice.
    fn picture(response: &Response<Vec<u8>>) -> String {
        let body = response.body();
        let mut sum: u64 = 1469598103934665603;
        for byte in body {
            sum ^= *byte as u64;
            sum = sum.wrapping_mul(1099511628211);
        }
        format!("{} bytes, {sum:016x}", body.len())
    }

    /// Copy one folder of drawings into the walkthrough's own directory, so that the project
    /// being walked has its own media and nothing here can write into `Fixtures/`.
    fn copy_drawings(from: &Path, into: &Path, rename: Option<&str>) {
        std::fs::create_dir_all(into).expect("make the drawings directory");
        for entry in std::fs::read_dir(from).expect("read the drawings folder") {
            let path = entry.expect("a directory entry").path();
            if path.extension().is_some_and(|x| x == "png") {
                let name = path
                    .file_name()
                    .expect("a file has a name")
                    .to_string_lossy()
                    .into_owned();
                let name = match rename {
                    // `layer4_004.png` delivered as the revised `layer3_004.png`: the drawing
                    // number is what a relink matches on, so the revised artwork has to arrive
                    // under the numbers the exposure sheet already names.
                    Some(stem) => match name.split_once('_') {
                        Some((_, number)) => format!("{stem}_{number}"),
                        None => name,
                    },
                    None => name,
                };
                std::fs::copy(&path, into.join(name)).expect("copy a drawing");
            }
        }
    }

    /// The five things W-02 asks for, walked in one go on the reference shot.
    ///
    /// W-02 is a sentence and a half: relink "by explicit user choice", "present changed
    /// dimensions, frame range or alpha interpretation before applying the change", "media
    /// replacement invalidates affected cache entries and preserves intentional holds", "if
    /// frames are absent, report them", "never silently slide subsequent drawings to fill a
    /// gap", and "undo restores the prior reference".
    ///
    /// `verification/B-12a_relink_table.md` checks the window's half of that on a small project
    /// built for it: what is said, what is refused, and what has not moved yet. What it cannot
    /// check is the picture, and the picture is where two of those clauses live. Invalidating a
    /// cache entry is invisible unless the frame that comes back afterwards is a different one,
    /// and not sliding drawings to fill a gap is invisible unless the frame at the gap is
    /// looked at. This walk renders the same frame before the relink, after it and after the
    /// undo, on the 240-frame reference shot with its holds, and compares the three.
    ///
    /// Writes `verification/B-12c_w02_walkthrough.md`.
    #[test]
    // Slow: it copies the reference shot's drawings and renders full-resolution previews.
    #[ignore = "copies the reference shot and renders previews; run by name"]
    fn the_revised_drawings_replace_the_old_ones_and_the_shot_keeps_its_timing() {
        let mut report = Report { rows: Vec::new() };
        let folder = workspace("w02");
        // The shot as delivered: the reference project and its four folders of drawings, copied
        // so that the media sits beside the project file exactly as a person's would.
        let project_file = folder.join("revised_shot.json");
        std::fs::copy(repo("verification/B-08a_project.json"), &project_file)
            .expect("copy the reference shot project");
        for name in ["layer1", "layer2", "layer3", "layer4"] {
            copy_drawings(
                &repo(&format!("Fixtures/reference_shot/{name}")),
                &folder.join(name),
                None,
            );
        }
        // The revised artwork, delivered in a folder of its own the way a revision arrives:
        // layer 3 redrawn. It is layer 4's drawings under layer 3's numbers, so every drawing
        // is a different picture from the one it replaces, and drawing 7 is not delivered -
        // the hole in this sequence is deliberate and stays a hole.
        let revised = folder.join("revised");
        copy_drawings(
            &repo("Fixtures/reference_shot/layer4"),
            &revised,
            Some("layer3"),
        );
        for number in [7, 12, 13, 14, 15, 16, 17, 18, 19] {
            let _ = std::fs::remove_file(revised.join(format!("layer3_{number:03}.png")));
        }
        let mut delivered: Vec<PathBuf> = std::fs::read_dir(&revised)
            .expect("read the revised folder")
            .filter_map(|e| e.ok().map(|e| e.path()))
            .collect();
        delivered.sort();

        let viewer = Mutex::new(
            open(&project_file)
                .unwrap_or_else(|d| panic!("open {}: {}", project_file.display(), d.message)),
        );
        let export_state = Mutex::new(Export::default());

        // ---- the shot as it stands ---------------------------------------------------------
        report.check(
            "the shot opens with the four sequences and the four layers it was left with",
            "layer3_%03d.png 11 drawings, numbered 0 to 6, 8 to 11, drawing 0 at layer3/layer3_000.png",
            sequence(&viewer, "asset-layer3"),
        );
        report.check(
            "the layer that uses the sequence being revised holds its drawings on twos",
            "0-2:0, 2-4:1, 4-6:2",
            exposures(&viewer, "layer-3")
                .split(", ")
                .take(3)
                .collect::<Vec<_>>()
                .join(", "),
        );
        let before = picture(&serve(&viewer, &export_state, Ask::Frame(8), None));
        let at_the_gap_before = picture(&serve(&viewer, &export_state, Ask::Frame(14), None));
        report.check(
            "and a frame of it can be looked at, which is what a relink has to change",
            true,
            before != "0 bytes, 0000000000000000",
        );

        // ---- W-02 step 1: choose the revised drawings --------------------------------------
        let said = run(
            &viewer,
            &format!(
                "media.relink?asset=asset-layer3&{}",
                delivered
                    .iter()
                    .map(|p| format!("file={}", for_a_header(&p.display().to_string())))
                    .collect::<Vec<_>>()
                    .join("&")
            ),
        );
        report.check(
            "step 1: choosing the revised drawings says what the sequence would become",
            "would give it 11 drawings, numbered 0 to 11, and drawing 7 would be missing",
            says(
                "would give it 11 drawings, numbered 0 to 11, and drawing 7 would be missing",
                &said,
            ),
        );
        report.check(
            "and states the size, which W-02 asks for whether or not it changed",
            "The drawings are 1920x1080, the same size as the ones it points at now.",
            says(
                "The drawings are 1920x1080, the same size as the ones it points at now.",
                &said,
            ),
        );
        report.check(
            "and how the colour and the alpha will be read, which W-02 also asks for",
            "The colour is read as sRGB with straight alpha",
            says("The colour is read as sRGB with straight alpha", &said),
        );
        report.check(
            "and that the drawing that was not delivered is missing, at the moment of choosing",
            "One drawing is missing from layer3_%03d.png: 7.",
            says("One drawing is missing from layer3_%03d.png: 7.", &said),
        );

        // ---- step 2: nothing has moved yet -------------------------------------------------
        report.check(
            "step 2: the sequence still points at the drawings it did",
            "layer3_%03d.png 11 drawings, numbered 0 to 6, 8 to 11, drawing 0 at layer3/layer3_000.png",
            sequence(&viewer, "asset-layer3"),
        );
        report.check(
            "and there is nothing to undo, because nothing has been done",
            0,
            held(&viewer).document.undo_depth(),
        );
        report.check(
            "and the frame that comes back is the frame that came back before",
            &before,
            picture(&serve(&viewer, &export_state, Ask::Frame(8), None)),
        );

        // ---- step 3: agree to it -----------------------------------------------------------
        let said = run(&viewer, "media.relink?asset=asset-layer3&apply=1");
        report.check(
            "step 3: agreeing names what was done, in the words undo will use",
            "Relink layer3",
            says("Relink layer3", &said),
        );
        report.check(
            "the sequence now points at the drawings that were chosen",
            "layer3_%03d.png 11 drawings, numbered 0 to 6, 8 to 11, drawing 0 at revised/layer3_000.png",
            sequence(&viewer, "asset-layer3"),
        );
        report.check(
            "the layer is the same layer, holding the same drawings for the same frames",
            "0-2:0, 2-4:1, 4-6:2",
            exposures(&viewer, "layer-3")
                .split(", ")
                .take(3)
                .collect::<Vec<_>>()
                .join(", "),
        );
        report.check(
            "and its whole exposure sheet is untouched, which is W-02's intentional holds",
            120,
            layer(&viewer, "layer-3", |l| l.exposure_spans.len()).unwrap_or(0),
        );
        report.check(
            "the other three sequences were not touched by a relink of this one",
            "layer2_%03d.png 24 drawings, numbered 0 to 23, drawing 0 at layer2/layer2_000.png",
            sequence(&viewer, "asset-layer2"),
        );

        // ---- step 4: the picture, which is where the cache lives ---------------------------
        let after = picture(&serve(&viewer, &export_state, Ask::Frame(8), None));
        report.check(
            "step 4: the frame that had been looked at before the relink comes back different",
            "a different picture",
            match after == before {
                true => "the same picture, so a cached frame outlived the drawings it was made \
                         from"
                    .to_string(),
                false => "a different picture".to_string(),
            },
        );
        report.check(
            "and the frame at the gap is the picture it was, because nothing was slid \
             into it",
            &at_the_gap_before,
            picture(&serve(&viewer, &export_state, Ask::Frame(14), None)),
        );
        report.check(
            "and frames 14 to 17 still ask for drawings 7, 7, 8, 8 - the two frames over the \
             gap keep asking for the drawing nobody drew, rather than being given the next one",
            "7,7,8,8",
            cel_identity(&viewer, "layer-3", 20)
                .split(',')
                .skip(14)
                .take(4)
                .collect::<Vec<_>>()
                .join(","),
        );

        // ---- step 5: undo restores the prior reference -------------------------------------
        let said = run(&viewer, "edit.undo");
        report.check(
            "step 5: undo says which thing it undid",
            "Undone: Relink layer3",
            says("Undone: Relink layer3", &said),
        );
        report.check(
            "the sequence points at the drawings it pointed at before",
            "layer3_%03d.png 11 drawings, numbered 0 to 6, 8 to 11, drawing 0 at layer3/layer3_000.png",
            sequence(&viewer, "asset-layer3"),
        );
        report.check(
            "and the frame is the picture it was, byte for byte, not a cached revised one",
            &before,
            picture(&serve(&viewer, &export_state, Ask::Frame(8), None)),
        );

        // ---- and it survives being written down --------------------------------------------
        run(&viewer, "edit.redo");
        let said = save_as(&viewer, &project_file);
        report.check(
            "redone and saved, the file names the drawings that were chosen",
            "Saved to",
            says("Saved to", &said),
        );
        let reopened = Mutex::new(
            open(&project_file)
                .unwrap_or_else(|d| panic!("reopen {}: {}", project_file.display(), d.message)),
        );
        report.check(
            "and reopening it finds the revised sequence, with the gap still a gap",
            "layer3_%03d.png 11 drawings, numbered 0 to 6, 8 to 11, drawing 0 at revised/layer3_000.png",
            sequence(&reopened, "asset-layer3"),
        );
        report.check(
            "and the exposure sheet that was typed before the revision, unchanged",
            120,
            layer(&reopened, "layer-3", |l| l.exposure_spans.len()).unwrap_or(0),
        );

        write_report(
            &report,
            "verification/B-12c_w02_walkthrough.md",
            "# W-02 walked end to end, on the shot W-01 built\n\n\
             Generated by `app/src/main.rs`, module `acceptance`. Slow, so it does not run with \
             the rest: `cargo test -p anime_compositor_app the_revised_drawings -- --ignored`.\n\n",
            W02_INTRO,
            W02_NOTES,
        );
        let failed: Vec<&(String, String, String)> =
            report.rows.iter().filter(|(_, e, a)| e != a).collect();
        assert!(
            failed.is_empty(),
            "W-02 did not walk end to end: {failed:#?}"
        );
    }

    const W02_INTRO: &[&str] = &[
        "W-02 is the other acceptance run: an existing shot, a folder of redrawn artwork, and \
         the question of whether replacing one costs the other. The window's half of it - what \
         is said before anything changes, what is refused, what has not moved yet - is \
         `verification/B-12a_relink_table.md`, on a small project built for it. This is the same \
         workflow on the 240-frame reference shot, and it is here for the two clauses of W-02 \
         that table cannot reach, because both of them are about the picture.",
        "**\"Media replacement invalidates affected cache entries.\"** A frame is decoded once \
         and kept. The failure is invisible in the project file and visible only in the window: \
         the sequence says it points at the new drawings, and the frame that comes back is the \
         old ones. So the same frame is rendered before the relink, after it, and again after \
         the undo, and the three are compared. The last of those is the sharper check - after \
         undoing, the picture has to be the first one back byte for byte.",
        "**\"Never silently slide subsequent drawings to fill a gap.\"** The revised folder is \
         delivered one drawing short, which is what happens. Nothing is renumbered to close the \
         hole: the frames that exposed drawing 7 still ask for drawing 7, and document 28 draws \
         a drawing that is not there as nothing.",
    ];

    const W02_NOTES: &[&str] = &[
        "## What to look at\n\n- **The two pictures that must differ and the two that must \
         match.** Frame 8 before the relink and after it are different pictures; frame 8 before \
         the relink and after the undo are the same picture, to the byte. A build that cached \
         the decoded drawing and never dropped it would fail the first; a build that dropped \
         everything and re-decoded from the wrong reference would fail the second.\n- **The \
         exposure sheet is not touched.** 120 spans before, 120 spans after - the 240 frames of \
         the shot held on twos - and the drawing \
         each frame asks for either side of the gap is what it was. Relinking replaces one \
         record in the media bin and cannot reach a layer, which is the design; these rows are \
         what says so on the shot rather than in the abstract.\n- **The revised artwork arrives \
         in a folder of its own.** It is layer4's drawings delivered under layer3's numbers, \
         which is what a revision looks like: same names, same numbers, different pictures, one \
         of them late.",
        "## What this cannot cover\n\n- **The file dialog.** As everywhere in this window, the \
         walk begins at the files a person chose.\n- **Whether the revised artwork is the right \
         artwork.** Nothing here can say the redraw was the one that was asked for; it can say \
         the shot now shows it and kept its timing.\n- **Masks.** W-02 names them among what a \
         relink preserves. It does preserve them, for the same reason it preserves exposures - \
         it replaces an asset record and cannot reach a layer - but this build has no gesture \
         for drawing a mask, so there is no mask in this shot to watch survive one.\n- **The \
         page.** Every row calls the request the panel sends rather than clicking the panel. \
         What is wired to what is `verification/B-12b_page_table.md`.",
    ];

    fn cell(text: &str) -> String {
        text.replace('|', r"\|")
    }
}

/// The fixture document 08 line 41 declares, built the way every other timing test builds it.
/// Included rather than copied: `tests/common/mod.rs` is where this project keeps the fixture
/// builder, and P-04 is the fourth measurement to want it.
#[cfg(test)]
#[path = "../../tests/common/mod.rs"]
mod common;

/// P-04: whether the editor answers while the renderer works.
///
/// The question is not how fast a frame is. It is whether a person pressing a key during one
/// gets an answer, and until P-04 they did not: `serve` took the viewer lock and held it through
/// planning, rendering and the encode, so every command queued behind the frame.
///
/// Both tests here put a render thread and a command thread on the same viewer. The first runs
/// on every build and proves the property without a stopwatch: a command answered *strictly
/// inside* one frame's own interval cannot have waited for that frame. The second is the
/// measurement the entry asks for and is `#[ignore]`d, because a number is only worth reading
/// from a release build on the recorded machine.
#[cfg(test)]
mod responsiveness {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    /// A span of wall clock: when something started and when it was answered.
    type Span = (Instant, Instant);

    fn repo(rel: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("the app crate has a parent directory")
            .join(rel)
    }

    /// The declared ten-layer fixture as a project this window can open, written beside its own
    /// drawings under `target/` so that opening it resolves media the way any project does.
    fn declared_fixture() -> Viewer {
        let (_project, root, text) = common::build_fixture("p04");
        let path = root.join("p04_project.json");
        std::fs::write(&path, &text).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
        open(&path).unwrap_or_else(|d| panic!("open {}: {}", path.display(), d.message))
    }

    /// Render frames on a thread of its own until it is told to stop, recording what each frame
    /// cost. Returns the spans, so the commands measured against them can be placed inside them.
    fn while_rendering(
        viewer: Arc<Mutex<Viewer>>,
        quality: PreviewQuality,
        frames: usize,
        command: impl Fn(&Mutex<Viewer>) -> Span,
    ) -> (Vec<Span>, Vec<Span>) {
        let export = Arc::new(Mutex::new(Export::default()));
        let stop = Arc::new(AtomicBool::new(false));
        let rendering = {
            let (viewer, export, stop) = (viewer.clone(), export.clone(), stop.clone());
            std::thread::spawn(move || {
                let mut spans = Vec::new();
                // A scatter across the work area rather than one frame over and over, so the
                // cache is not answering every render out of the same entry.
                for i in 0..frames {
                    let at = Instant::now();
                    let response = serve(
                        &viewer,
                        &export,
                        Ask::Frame((i as i32 * 97) % 240),
                        Some(quality),
                    );
                    assert_eq!(response.status(), 200, "a frame the window could not make");
                    spans.push((at, Instant::now()));
                }
                stop.store(true, Ordering::Release);
                spans
            })
        };
        let mut answered = Vec::new();
        while !stop.load(Ordering::Acquire) {
            answered.push(command(&viewer));
            // A person presses a key about this often at their fastest. A hot loop would
            // measure the lock's throughput under one thread's whole attention, which is not a
            // question anybody asked.
            std::thread::sleep(Duration::from_millis(1));
        }
        let frames = rendering.join().expect("the render thread panicked");
        (frames, answered)
    }

    /// One command, timed: how long the window took to answer it.
    fn toggle(viewer: &Mutex<Viewer>) -> Span {
        let at = Instant::now();
        let said = edit_command(viewer, "viewer.toggle_checkerboard", None);
        let answered = Instant::now();
        assert!(
            said.is_some(),
            "the window did not answer a command document 24 lists"
        );
        (at, answered)
    }

    /// Milliseconds, nearest-rank p50 and p95, of a list of spans.
    fn spread(spans: &[Span]) -> (f64, f64, f64) {
        let mut ms: Vec<f64> = spans
            .iter()
            .map(|(at, to)| to.duration_since(*at).as_secs_f64() * 1000.0)
            .collect();
        ms.sort_by(|a, b| a.partial_cmp(b).expect("a duration is never NaN"));
        let rank = |p: f64| ms[(((ms.len() as f64) * p).ceil() as usize).clamp(1, ms.len()) - 1];
        (
            rank(0.5),
            rank(0.95),
            *ms.last().expect("at least one command"),
        )
    }

    /// How many of the answered commands began and ended inside one frame that was still being
    /// rendered. On the build before P-04 this is zero by construction: the render held the lock
    /// the command needed, so no command could be answered until the frame it waited on was
    /// finished.
    fn inside_a_frame(frames: &[Span], answered: &[Span]) -> usize {
        answered
            .iter()
            .filter(|(at, to)| frames.iter().any(|(from, until)| at > from && to < until))
            .count()
    }

    /// The property, on every build, without a stopwatch: a command is answered while a frame is
    /// being made, not after it.
    ///
    /// The reference shot rather than the declared fixture, because this one runs in the normal
    /// suite and the point it makes does not need a quarter-second frame to make it.
    ///
    /// Writes `verification/P-04_responsiveness_table.md`.
    #[test]
    fn a_command_is_answered_while_a_frame_is_being_made() {
        let viewer = Arc::new(Mutex::new(demo()));
        // Draft, and four frames: this one runs in the normal suite, on a debug build, and the
        // point it makes does not need a slow frame to make it.
        let (frames, answered) = while_rendering(viewer, PreviewQuality::Draft, 4, toggle);
        let inside = inside_a_frame(&frames, &answered);
        let (_, _, worst) = spread(&answered);
        let (fp50, _, _) = spread(&frames);

        let mut table = String::new();
        table.push_str(
            "# P-04: is the editor answering while the renderer works?\n\n\
             Written by `a_command_is_answered_while_a_frame_is_being_made` in \
             `app/src/main.rs`, on every build. One thread asks the window for frames of the \
             reference shot as fast as it can answer; another presses a key over and over and \
             times how long each press waits. A press that begins *and* ends between the start \
             and the end of one frame cannot have waited for that frame.\n\n\
             Nothing on this page is a duration. How many presses fit inside a frame, and how \
             long each one took, depend on the build and the machine the suite happened to run \
             on, and a page rewritten with different numbers on every run is a page nobody can \
             tell a change from. The durations are the measurement, and they are dated: \
             `verification/P-04_responsiveness.md`.\n\n",
        );
        table.push_str("| Question | Expected | Found | Verdict |\n|---|---|---|---|\n");
        let verdict = |ok: bool| if ok { "pass" } else { "FAIL" };
        let said = |ok: bool, yes: &'static str, no: &'static str| if ok { yes } else { no };
        let made = !frames.is_empty() && !answered.is_empty();
        table.push_str(&format!(
            "| were frames being made while the keys were pressed | {} frames, and at least one \
             press | {} | {} |\n",
            frames.len(),
            said(made, "yes", "no"),
            verdict(made),
        ));
        table.push_str(&format!(
            "| presses answered inside a frame that was still being rendered | more than none | \
             {} | {} |\n",
            said(inside > 0, "more than none", "none"),
            verdict(inside > 0),
        ));
        table.push_str(&format!(
            "| the slowest press against the time one frame takes | shorter | {} | {} |\n",
            said(worst < fp50, "shorter", "not shorter"),
            verdict(worst < fp50),
        ));
        let out = repo("verification/P-04_responsiveness_table.md");
        std::fs::write(&out, table).unwrap_or_else(|e| panic!("write {}: {e}", out.display()));

        assert!(
            inside > 0,
            "no command was answered inside a frame: {} presses against {} frames, so the \
             render is still holding the lock a command needs",
            answered.len(),
            frames.len()
        );
        assert!(
            worst < fp50,
            "the slowest press took {worst:.3} ms against a frame of {fp50:.3} ms, which is the \
             shape of a press that waited for a frame"
        );
    }

    /// The measurement the entry asks for: what a keystroke waits for while a frame of the
    /// declared ten-layer fixture is being made, at both qualities.
    ///
    /// `#[ignore]`d and release-only, like every other timing test here. Writes
    /// `verification/P-04_responsiveness.md`.
    #[test]
    #[ignore = "a measurement; run it deliberately, in release, on the recorded machine"]
    fn p04_responsiveness() {
        // What the short lock copies out. A frame that used to borrow the project now clones
        // it, and a page reporting a saving has to say what it spent to get it.
        let copied = {
            let viewer = declared_fixture();
            let mut each = Vec::new();
            for _ in 0..200 {
                let at = Instant::now();
                let copy = viewer.document.project().clone();
                each.push((at, Instant::now()));
                drop(copy);
            }
            spread(&each)
        };
        let mut rows = String::new();
        for quality in [PreviewQuality::Draft, PreviewQuality::Full] {
            let viewer = Arc::new(Mutex::new(declared_fixture()));
            let (frames, answered) = while_rendering(viewer, quality, 12, toggle);
            let (p50, p95, worst) = spread(&answered);
            let (fp50, fp95, _) = spread(&frames);
            let inside = inside_a_frame(&frames, &answered);
            rows.push_str(&format!(
                "| {} | {:.3} | {:.3} | **{p50:.3}** | **{p95:.3}** | {worst:.3} | {inside} of \
                 {} |\n",
                quality.label(),
                fp50,
                fp95,
                answered.len(),
            ));
        }
        let out = repo("verification/P-04_measured.md");
        let mut text = String::new();
        text.push_str(
            "Written by `p04_responsiveness` in `app/src/main.rs`. One thread asks for frames \
             of the declared ten-layer fixture; another presses a key over and over and times \
             the answer.\n\n\
             | Quality | Frame p50 (ms) | Frame p95 (ms) | Press p50 (ms) | Press p95 (ms) | \
             Worst press (ms) | Answered inside a frame |\n|---|---|---|---|---|---|---|\n",
        );
        text.push_str(&rows);
        text.push_str(&format!(
            "\nCopying the project out under the short lock, which is what a frame now does \
             instead of borrowing it: p50 {:.4} ms, p95 {:.4} ms, worst {:.4} ms over 200.\n",
            copied.0, copied.1, copied.2
        ));
        std::fs::write(&out, text).unwrap_or_else(|e| panic!("write {}: {e}", out.display()));
    }
}
