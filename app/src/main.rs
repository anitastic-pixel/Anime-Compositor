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
use std::sync::Mutex;
use std::time::Duration;

use anime_compositor::cache::{CelCache, DEFAULT_BUDGET_BYTES};
use anime_compositor::command::Document;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::{Diagnostic, DiagnosticId, FrameLog, Severity};
use anime_compositor::model::Id;
use anime_compositor::persist::{self, Preserved};
use anime_compositor::preview::{self, Playback, PreviewQuality};
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
        notes: loaded.warnings.iter().map(sentence).collect(),
        status: String::new(),
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

/// Say something in the window's status line, replacing whatever it said before.
fn announce(viewer: &Mutex<Viewer>, said: String) {
    viewer.lock().expect("the viewer lock was poisoned").status = said;
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
    let said = match path.trim_matches('/') {
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
        _ => {
            return allow_the_page_to_read_this(Response::builder().status(404))
                .header("content-type", "text/plain; charset=utf-8")
                .body(b"ask for /open, /save, /save-as or /recent".to_vec())
                .expect("build the not-found response")
        }
    };
    allow_the_page_to_read_this(Response::builder())
        .header("content-type", "text/plain; charset=utf-8")
        .body(said.into_bytes())
        .expect("build the command response")
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
        .plugin(tauri_plugin_dialog::init())
        .manage(viewer)
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
