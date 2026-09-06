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
//! A project reaches the window two ways: named on the command line, or dropped on it. Both go
//! through [`open`], which is `persist::load` and nothing else, so what the viewer says about a
//! project is what the core said about it — including everything document 28 asks to be told.

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Duration;

use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::{Diagnostic, DiagnosticId, FrameLog, Severity};
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::{self, Playback, PreviewQuality};
use tauri::http::{Request, Response};
use tauri::{DragDropEvent, Manager, WindowEvent};

/// What the shell is looking at.
///
/// One project, one composition, one clock, and whatever the core had to say when it was
/// opened.
struct Viewer {
    project: Project,
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
fn serve(viewer: &Mutex<Viewer>, ask: Ask, quality: Option<PreviewQuality>) -> Response<Vec<u8>> {
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
    let buffer = match preview::preview_frame(
        &viewer.project,
        &viewer.composition,
        frame,
        &viewer.root,
        viewer.quality,
        DEFAULT_TILE_SIZE,
        &mut log,
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
    let project = loaded.document.project().clone();
    let composition = project.compositions.first().ok_or_else(|| {
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
    Ok(Viewer {
        project,
        root: path.parent().unwrap_or(Path::new(".")).to_path_buf(),
        composition,
        quality: PreviewQuality::default(),
        playback,
        name: path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.display().to_string()),
        notes: loaded.warnings.iter().map(sentence).collect(),
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
    viewer
}

/// Load a dropped or named file into the viewer, or report why it could not be.
///
/// A file that cannot be opened leaves the project that was open exactly as it was and adds the
/// reason to what the window is saying. Closing a working project because the next one was
/// unreadable would lose the person their place to punish them for a bad drop.
fn take(viewer: &Mutex<Viewer>, path: &Path) {
    let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
    match open(path) {
        Ok(opened) => *viewer = opened,
        Err(diagnostic) => viewer.notes = vec![sentence(&diagnostic)],
    }
}

fn main() {
    // A project named on the command line goes through the same `take` a dropped one does, so
    // the two ways in cannot behave differently. It is also the only one a script can drive,
    // which is how the photographs under `verification/` are taken.
    let viewer = Mutex::new(demo());
    if let Some(named) = std::env::args_os().nth(1) {
        take(&viewer, Path::new(&named));
    }

    tauri::Builder::default()
        .manage(viewer)
        .on_window_event(|window, event| {
            let WindowEvent::DragDrop(DragDropEvent::Drop { paths, .. }) = event else {
                return;
            };
            let Some(path) = paths.first() else { return };
            take(&window.state::<Mutex<Viewer>>(), path);
            // The page is stateless about the project — everything it says comes back with a
            // frame — so reloading it is the whole of the update.
            if let Some(page) = window.get_webview_window("main") {
                let _ = page.eval("location.reload()");
            }
        })
        .register_uri_scheme_protocol("frame", |ctx, request: Request<Vec<u8>>| {
            let viewer = ctx.app_handle().state::<Mutex<Viewer>>();
            match parse(request.uri().path(), request.uri().query()) {
                Some((ask, quality)) => serve(&viewer, ask, quality),
                None => allow_the_page_to_read_this(Response::builder().status(404))
                    .header("content-type", "text/plain; charset=utf-8")
                    .body(b"ask for /at/<milliseconds> or /frame/<number>".to_vec())
                    .expect("build the not-found response"),
            }
        })
        .run(tauri::generate_context!())
        .expect("the window could not be created");
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
