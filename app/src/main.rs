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

use std::path::PathBuf;
use std::sync::Mutex;
use std::time::Duration;

use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::{self, Playback, PreviewQuality};
use tauri::http::{Request, Response};
use tauri::Manager;

/// What the shell is looking at.
///
/// One project, one composition, one clock. There is no file-open interface yet, so the project
/// is the reference shot and the path to it is a development convenience — see [`open`].
struct Viewer {
    project: Project,
    /// The directory asset paths resolve against, which is the fixture root and not the
    /// project file's own directory.
    root: PathBuf,
    composition: Id,
    quality: PreviewQuality,
    playback: Playback,
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

/// The page and the frames are different origins, so the page can only read a frame if the
/// response says so. Exposing the headers is the half that is easy to forget: without it the
/// pixels arrive and every `x-` header beside them reads as absent, which would leave the frame
/// number and the resolution indicator describing nothing.
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
            return allow_the_page_to_read_this(Response::builder())
                .status(500)
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

/// The project the window opens on.
///
/// Hard-coded, deliberately and temporarily: there is no open dialog yet, and a viewer with
/// nothing in it cannot be looked at. It is the reference shot, resolved against this crate's
/// source directory, so the shell only runs from a checkout. When B-09 brings a real open
/// command this function is what it replaces.
fn open() -> Viewer {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .to_path_buf();
    let path = repo.join("verification/B-08a_project.json");
    let loaded =
        persist::load(&path).unwrap_or_else(|d| panic!("open {}: {}", path.display(), d.message));
    let project = loaded.document.project().clone();
    let composition = project
        .compositions
        .first()
        .expect("the reference shot has a composition");
    let first = composition.start_frame;
    let last = first + composition.duration_frames as i32 - 1;
    let playback = Playback::new(first, last, composition.frame_rate);
    let composition = composition.id.clone();
    Viewer {
        project,
        root: repo.join("Fixtures/reference_shot"),
        composition,
        quality: PreviewQuality::default(),
        playback,
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(open()))
        .register_uri_scheme_protocol("frame", |ctx, request: Request<Vec<u8>>| {
            let viewer = ctx.app_handle().state::<Mutex<Viewer>>();
            match parse(request.uri().path(), request.uri().query()) {
                Some((ask, quality)) => serve(&viewer, ask, quality),
                None => allow_the_page_to_read_this(Response::builder())
                    .status(404)
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
}
