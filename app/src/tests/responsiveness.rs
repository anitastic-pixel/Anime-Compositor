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
