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
use anime_compositor::command::Document;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::{Diagnostic, DiagnosticId, FrameLog, Severity};
use anime_compositor::export::{self, ExportReport, ExportRequest, ExportStatus, MissingSource};
use anime_compositor::model::{Id, Project};
use anime_compositor::persist::{self, Preserved};
use anime_compositor::preview::{self, Playback, PreviewQuality};
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
                    b"ask for /open, /save, /save-as, /recover, /export, /cancel-export or \
                      /recent"
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
        let finished = format!("The 0 frames that finished are in {}.", elsewhere.display());
        check(
            "and the window's own first sentence says what is there, not that it exported them",
            &finished,
            &says(&finished, &said),
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
