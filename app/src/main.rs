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

use anime_compositor::cache::{CelCache, DEFAULT_BUDGET_BYTES};
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::{Diagnostic, DiagnosticId, FrameLog, Severity};
use anime_compositor::effects::{Effect, EffectInstance, EXPOSURE, GAUSSIAN_BLUR, TINT};
use anime_compositor::export::{self, ExportReport, ExportRequest, ExportStatus, MissingSource};
use anime_compositor::media;
use anime_compositor::model::{Asset, Id, Layer, Project, Prop, Value};
use anime_compositor::persist::{self, Preserved};
use anime_compositor::preview::{self, Playback, PreviewQuality};
use anime_compositor::time::ExposureSpan;
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
    /// B-08b: the decoded cels this preview has already paid for. Belongs to the viewer rather
    /// than to a frame because its whole purpose is to outlive one, and it is replaced along with
    /// everything else when a different project is opened, so nothing from the old one survives.
    cache: CelCache,
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
}

/// Read `/at/<milliseconds>` or `/frame/<n>`, with an optional `?q=draft|full`.
///
/// Returns `None` for anything else, which the handler answers with a 404 rather than guessing.
/// An unreadable quality is `None` in the second slot, meaning "leave it as it is": a typo in a
/// query string should not silently switch the preview to a resolution nobody asked for.
fn parse(path: &str, query: Option<&str>) -> Option<(Ask, Option<PreviewQuality>)> {
    let mut parts = path.trim_matches('/').split('/');
    let ask = match (parts.next()?, parts.next()?, parts.next()) {
        ("at", ms, None) => Ask::At(ms.parse().ok()?),
        ("frame", n, None) => Ask::Frame(n.parse().ok()?),
        _ => return None,
    };
    let quality = query
        .and_then(|q| q.split('&').find_map(|pair| pair.strip_prefix("q=")))
        .and_then(|value| match value {
            "draft" => Some(PreviewQuality::Draft),
            "full" => Some(PreviewQuality::Full),
            _ => None,
        });
    Some((ask, quality))
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
    let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
    if let Some(quality) = quality {
        viewer.quality = quality;
    }
    let (frame, skipped) = match ask {
        Ask::At(ms) => {
            let shown = viewer.playback.at(Duration::from_millis(ms));
            (shown.frame, shown.skipped)
        }
        // Stepping stops at the ends of the work area rather than running off them: a frame
        // outside the composition is not a frame, and the viewer has nowhere to go from there.
        Ask::Frame(n) => {
            let first = viewer.playback.at_rest();
            let last = first + viewer.playback.length() as i32 - 1;
            (n.clamp(first, last), 0)
        }
    };

    let mut log = FrameLog::new(3);
    let buffer = match preview::preview_frame_cached(
        viewer.document.project(),
        &viewer.composition,
        frame,
        &viewer.root,
        viewer.quality,
        DEFAULT_TILE_SIZE,
        &mut log,
        &mut viewer.cache,
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
    allow_the_page_to_read_this(Response::builder())
        .header("content-type", "application/octet-stream")
        .header("x-frame", frame.to_string())
        .header("x-skipped", skipped.to_string())
        .header("x-width", width.to_string())
        .header("x-height", height.to_string())
        .header("x-quality", viewer.quality.label())
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
        .header("x-export", for_a_header(&exported))
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
                Ask::Frame(_) => String::new(),
            },
        )
        .body(buffer.to_srgb8_straight())
        .expect("build the frame response")
}

/// Open a project file.
///
/// This is `persist::load` and a composition to look at. Media resolves against the project
/// file's own directory, which is the rule the core checks media against, so what the viewer
/// renders and what the core warned about are the same set of files.
fn open(path: &Path) -> Result<Viewer, Diagnostic> {
    let loaded = persist::load(path)?;
    let composition = loaded
        .document
        .project()
        .compositions
        .first()
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
        cache: CelCache::with_budget(DEFAULT_BUDGET_BYTES),
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

/// `edit.undo` and `edit.redo` from document 24.
///
/// Both name what they moved. "Undone" alone would be true and useless: the whole reason undo is
/// trusted is that the person can see it took back the thing they meant.
fn undo(viewer: &Mutex<Viewer>) -> String {
    let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
    match viewer.document.undo() {
        Some(record) => format!("Undone: {}", record.label),
        None => "There is nothing to undo.".to_string(),
    }
}

fn redo(viewer: &Mutex<Viewer>) -> String {
    let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
    match viewer.document.redo() {
        Some(record) => format!("Redone: {}", record.label),
        None => "There is nothing to redo.".to_string(),
    }
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
    match prop.kind() {
        "vec2" => {
            let (x, y) = text.split_once(',')?;
            Some(Value::Vec2(number(x)?, number(y)?))
        }
        _ => Some(Value::Scalar(number(text)?)),
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
fn edit_command(viewer: &Mutex<Viewer>, id: &str, query: Option<&str>) -> Option<String> {
    match id {
        "edit.undo" => return Some(undo(viewer)),
        "edit.redo" => return Some(redo(viewer)),
        // The two ends of an interaction transaction. Neither names a layer: the drag already
        // knows which value it is holding, and asking the page to name it again would be asking
        // it to be right about something it has no way to check.
        "property.drag_end" => return Some(end_drag(viewer)),
        "property.drag_cancel" => return Some(cancel_drag(viewer)),
        // The one command that names files rather than anything in the project. Without any,
        // the request never reaches here: the window answers it with the operating system's
        // file dialog, which only the app handle can open.
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
                // Typed into a field, or scrubbed on its label. The same command either way;
                // what differs is whether it becomes a history entry on its own or joins the one
                // the drag will commit at release.
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
                    Command::SetPropertyBase {
                        composition,
                        layer_id,
                        prop,
                        value,
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
                // The other three all name an instance that is already on the layer, so the
                // lookup and its refusal are written once.
                "effect.delete" | "effect.toggle_bypass" | "effect.set_parameters" => {
                    let Some(instance_id) = parameter(query, "effect").map(Id::new) else {
                        return Some("Which effect? Choose one in the effects list.".to_string());
                    };
                    let Some(existing) =
                        layer.effects.iter().find(|e| e.instance_id == instance_id)
                    else {
                        return Some(format!("{instance_id} is not an effect on this layer."));
                    };
                    match id {
                        "effect.delete" => Command::RemoveEffect {
                            composition,
                            layer_id,
                            instance_id,
                        },
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
    let said = if id == "property.drag_update" {
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

fn parameter(query: Option<&str>, name: &str) -> Option<String> {
    let prefix = format!("{name}=");
    query
        .and_then(|q| q.split('&').find_map(|pair| pair.strip_prefix(&prefix)))
        .map(from_a_query)
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
            let missing = match parameter(query, "missing").as_deref() {
                Some("write") => MissingSource::RenderTransparent,
                _ => MissingSource::Block,
            };
            ask_where_to_export(app, missing);
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
            match parse(request.uri().path(), request.uri().query()) {
                Some((ask, quality)) => serve(&viewer, &export, ask, quality),
                None => allow_the_page_to_read_this(Response::builder().status(404))
                    .header("content-type", "text/plain; charset=utf-8")
                    .body(b"ask for /at/<milliseconds> or /frame/<number>".to_vec())
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
             checked in `verification/B-05_command_table.md` and is not repeated here. What is \
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
            "a value that is not a finite number is refused",
            "scale cannot be set to (NaN, 1).",
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
         by document 19's rules - is checked in `verification/B-05_command_table.md`. What is \
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
         about the project format rather than about this window.\n\nKeyframes. The inspector sets \
         a property's base value, which is what document 19 calls the value with no keyframes on \
         it. `keyframe.add_remove` is in document 24 and is not built.",
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
            "none of those five refusals put anything in the history",
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
         draws is checked in `verification/B-07_effect_table.md` and is not repeated here. What \
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
         checked against independently generated weights in `verification/B-07_effect_table.md`, \
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
         is checked in `verification/B-06_matte_table.md` and is not repeated here. What is \
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
