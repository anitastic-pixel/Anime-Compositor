//! B-08's first piece: the preview path — resolution selection and the playback clock.
//!
//! Writes `verification/B-08_preview_table.md` and `verification/B-08_preview_draft_100.png`.
//!
//! # What this is for
//!
//! B-08a joined a saved project to a rendered frame and stopped there, because everything after
//! it needed decisions nobody had made. The owner made two of them on 2026-09-05, and this is
//! those two decisions turned into something that can be checked rather than described.
//!
//! - **D-33** — the preview starts at draft resolution, with a visible indication that it is not
//!   final pixels. R-06a requires that indication whenever preview quality differs from export.
//! - **D-32** — playback holds real time and drops the frames it cannot deliver, and reports how
//!   many it dropped. An unreported drop would be a silent fidelity fallback, which document 28
//!   forbids.
//!
//! There is deliberately no window here. A window is checked by looking at it, and the two rules
//! above are checked by a table, so they are separated: what a fixture can hold is held by a
//! fixture, and the screenshots follow with the transport.
//!
//! # Where the expected values come from
//!
//! **The draft extent** is SP-05's, not this build's. The spike transported 480×270 against the
//! reference shot's 1920×1080 and measured 145 fps against 24.9 at full
//! (`spikes/B-01_G0_spike_report.md`). A quarter on each axis is therefore a measurement, and
//! 480×270 is arithmetic on the composition the fixture declares.
//!
//! **The playback frames** are hand-derived from the definition D-32 chose, that frame `k`
//! occupies the half-open interval `[k/rate, (k+1)/rate)` in wall-clock time. At 24 fps one
//! frame is 1/24 of a second, which is 41666666.66… nanoseconds, so 41666666 ns is still frame 0
//! and 41666667 ns is frame 1. At 24000/1001 one frame is 1001/24000 of a second, which is
//! 41708333.33… nanoseconds, so 41708333 ns is still frame 0 and 41708334 ns is frame 1. Both
//! boundaries are computed by hand above and written as literals below; neither was read off a
//! run of this build, per ADR-009.
//!
//! **The skip counts** follow from the same definition. A caller that answers every third frame
//! time has advanced three frames each time, of which one is the frame it draws and two came and
//! went undrawn.
//!
//! **Layer 3's colour**, `(255, 220, 0, 255)`, and **the gap frames** are the reference shot's
//! own, taken from `Fixtures/reference_shot/README.md` and its declared cadence, and are the
//! same values B-10 checks the full export against. Frame 14 is a gap frame — layer 3 asks for
//! drawing 7, which the fixture deliberately does not contain — and frame 12 is not.
//!
//! # What this deliberately does not do
//!
//! It does not open a window, drive a real clock, or sleep. Nothing here waits: the playback
//! clock is asked what belongs on screen at an instant the test supplies, which is the only way
//! a decision about dropped frames can be checked by a table rather than by watching it and
//! forming an impression.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::export::{export_sequence, ExportRequest, MissingSource};
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::png_out;
use anime_compositor::preview::{self, Playback, PreviewQuality};
use anime_compositor::{OutputAlpha, OutputDepth, WorkingBuffer};

const COMP: &str = "comp-reference-shot";
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
/// The frame the preview and the export are compared at. Not a gap frame: 100/2 is 50 and
/// 50 % 12 is 2, so layer 3 exposes drawing 2 and every layer of the shot is present.
const FRAME: i32 = 100;
/// A frame where layer 3 asks for the drawing the fixture deliberately omits: 14/2 is 7.
const GAP_FRAME: i32 = 14;
/// A frame either side of that gap, where layer 3 does paint: 12/2 is 6.
const PAINTED_FRAME: i32 = 12;
/// Layer 3's colour in the reference shot, from the fixture's own README.
const LAYER3: [u8; 4] = [255, 220, 0, 255];

// ---------------------------------------------------------------------------------------
// Reporting
// ---------------------------------------------------------------------------------------

struct Row {
    check: String,
    expected: String,
    actual: String,
}

impl Row {
    fn pass(&self) -> bool {
        self.expected == self.actual
    }
}

#[derive(Default)]
struct Report {
    rows: Vec<Row>,
}

impl Report {
    fn check(&mut self, check: &str, expected: impl ToString, actual: impl ToString) {
        self.rows.push(Row {
            check: check.to_string(),
            expected: expected.to_string(),
            actual: actual.to_string(),
        });
    }
}

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn root() -> PathBuf {
    repo("Fixtures/reference_shot")
}

// ---------------------------------------------------------------------------------------
// The project: B-08a's own artifact, read back off the disk
// ---------------------------------------------------------------------------------------

/// `verification/B-08a_project.json` is a real project file, written by B-08a and committed.
/// Reading it rather than rebuilding the model in code is deliberate: the preview path is
/// supposed to work from a saved project, and a project assembled in the test would not prove
/// that.
fn project() -> Project {
    let path = repo("verification/B-08a_project.json");
    let loaded =
        persist::load(&path).unwrap_or_else(|d| panic!("open {}: {}", path.display(), d.message));
    loaded.document.project().clone()
}

fn preview_at(project: &Project, frame: i32, quality: PreviewQuality) -> WorkingBuffer {
    let mut log = FrameLog::new(3);
    preview::preview_frame(
        project,
        &Id::new(COMP),
        frame,
        &root(),
        quality,
        DEFAULT_TILE_SIZE,
        &mut log,
    )
    .unwrap_or_else(|d| panic!("preview frame {frame}: {}", d.message))
}

/// Export one frame through the ordinary export path, and hand back the samples it wrote.
///
/// This is the export half of the comparison, and it is a real export: the same
/// [`export_sequence`] B-10 runs the whole shot through, at the document 07 default for a
/// missing drawing, which frame 100 never reaches because nothing is missing there.
fn exported(project: &Project) -> Result<Vec<u8>, String> {
    let dir = repo("target/b08_preview");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap_or_else(|e| panic!("make {}: {e}", dir.display()));
    let request = ExportRequest {
        composition: Id::new(COMP),
        first_frame: FRAME,
        last_frame: FRAME,
        output_dir: dir.clone(),
        naming: "shot_%04d.png".to_string(),
        depth: OutputDepth::Eight,
        alpha: OutputAlpha::Straight,
        tile_size: DEFAULT_TILE_SIZE,
        missing: MissingSource::Block,
    };
    let report = export_sequence(project, &root(), &request, &AtomicBool::new(false));
    if !report.succeeded() {
        // A sentence, not a panic. B-10 found two rows that could only ever have failed by
        // crashing, and a crashed table is a table the owner cannot read.
        return Err(format!(
            "the export of frame {FRAME} did not succeed: {:?}",
            report.status
        ));
    }
    decode(&dir.join(format!("shot_{FRAME:04}.png")))
}

/// The RGBA bytes of a written PNG, so the comparison is against the file rather than against
/// the buffer the file was made from. A sentence rather than a panic, for the same reason.
fn decode(path: &Path) -> Result<Vec<u8>, String> {
    let file =
        fs::File::open(path).map_err(|e| format!("{} was not written: {e}", path.display()))?;
    let mut reader = png::Decoder::new(std::io::BufReader::new(file))
        .read_info()
        .map_err(|e| format!("{} is not a readable PNG: {e}", path.display()))?;
    let mut samples = vec![0u8; reader.output_buffer_size().expect("a sized buffer")];
    let info = reader
        .next_frame(&mut samples)
        .map_err(|e| format!("{} does not decode: {e}", path.display()))?;
    samples.truncate(info.buffer_size());
    Ok(samples)
}

/// How many samples of two equal-length images differ, and nothing about how much by: at full
/// resolution the answer has to be zero, so a magnitude would be a number nobody reads.
fn differing(a: &[u8], b: &Result<Vec<u8>, String>) -> String {
    let b = match b {
        Err(why) => return why.clone(),
        Ok(b) => b,
    };
    if a.len() != b.len() {
        return format!("different lengths: {} and {}", a.len(), b.len());
    }
    let n = a.iter().zip(b).filter(|(x, y)| x != y).count();
    format!("{n} of {} samples differ", a.len())
}

/// Whether layer 3's paint appears anywhere in an image, scanned whole rather than sampled.
///
/// B-10 learned this the expensive way: a one-pixel sample passed a deliberate fault that
/// substituted the nearest drawing for a missing one, because a substituted drawing still paints
/// a bar, only elsewhere. The whole frame is scanned here for the same reason.
fn has_layer3(samples: &[u8]) -> &'static str {
    if samples.chunks_exact(4).any(|p| p == LAYER3) {
        "layer 3 paint present"
    } else {
        "no layer 3 paint"
    }
}

// ---------------------------------------------------------------------------------------
// The playback clock, at 24 fps unless a row says otherwise
// ---------------------------------------------------------------------------------------

fn rate(numerator: u32, denominator: u32) -> anime_compositor::time::FrameRate {
    anime_compositor::time::FrameRate::new(numerator, denominator).expect("a real frame rate")
}

/// `n` whole frames of wall-clock time at 24 fps, exactly, as nanoseconds are not divisible by 24.
///
/// 24 frames is one second, so `n` frames is `n * 1_000_000_000 / 24` nanoseconds and that
/// division is not exact for most `n`. The boundary rows below use literal nanosecond values
/// derived by hand; this helper is only for the rows where `n` is a multiple of 3, for which
/// the division is exact: 3/24 second is 125000000 ns exactly.
fn frames24(n: u64) -> Duration {
    assert!(n % 3 == 0, "only multiples of three divide exactly");
    Duration::from_nanos(n / 3 * 125_000_000)
}

/// Where each layer's transform carries the composition's far corner, rounded to whole pixels.
///
/// The corner is the cheapest place to see a scale: it is the one point a uniform scale about
/// the origin moves the furthest, so a scale that was not applied is visible at a glance.
fn corners(plan: &anime_compositor::render::FramePlan) -> String {
    plan.layers
        .iter()
        .map(|layer| {
            let (x, y) = layer.transform.apply(WIDTH as f64, HEIGHT as f64);
            format!("{},{}", x.round(), y.round())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn play(shown: &[(i32, u32)]) -> String {
    shown
        .iter()
        .map(|(frame, skipped)| format!("{frame}+{skipped}"))
        .collect::<Vec<_>>()
        .join(" ")
}

// ---------------------------------------------------------------------------------------
// The check
// ---------------------------------------------------------------------------------------

#[test]
fn b08_previews_at_a_chosen_resolution_and_plays_at_real_time() {
    let mut report = Report::default();
    let project = project();

    // -----------------------------------------------------------------------------------
    // D-33: resolution selection, and the indicator that makes a draft default honest
    // -----------------------------------------------------------------------------------

    report.check(
        "the preview opens at the resolution D-33 chose",
        "Draft",
        PreviewQuality::default().label(),
    );
    report.check(
        "at draft, R-06a's indication that preview differs from export is on",
        "Draft, differs from export: true",
        format!(
            "{}, differs from export: {}",
            PreviewQuality::Draft.label(),
            PreviewQuality::Draft.differs_from_export()
        ),
    );
    report.check(
        "at full, there is nothing to indicate, because nothing differs",
        "Full, differs from export: false",
        format!(
            "{}, differs from export: {}",
            PreviewQuality::Full.label(),
            PreviewQuality::Full.differs_from_export()
        ),
    );
    report.check(
        "draft is SP-05's measured extent for this composition",
        "480x270",
        {
            let (w, h) = PreviewQuality::Draft.extent(WIDTH, HEIGHT);
            format!("{w}x{h}")
        },
    );
    report.check(
        "full is the composition's own extent, unchanged",
        "1920x1080",
        {
            let (w, h) = PreviewQuality::Full.extent(WIDTH, HEIGHT);
            format!("{w}x{h}")
        },
    );
    report.check(
        "a composition that does not divide by four keeps its last column and row",
        "481x271",
        {
            let (w, h) = PreviewQuality::Draft.extent(WIDTH + 1, HEIGHT + 1);
            format!("{w}x{h}")
        },
    );

    // A preview at a smaller extent is only a preview if the picture shrank with it. An extent
    // that changed while the layers did not would be a crop of the top-left corner, at the right
    // size and of the wrong thing, and every extent row above would still pass. Each layer of
    // this shot sits at identity, so a quarter scale must carry the composition's far corner,
    // (1920, 1080), to the draft frame's far corner, (480, 270) - arithmetic, not a measurement.
    let mut log = FrameLog::new(3);
    let plan =
        anime_compositor::compose::plan_frame(&project, &Id::new(COMP), FRAME, &root(), &mut log)
            .expect("frame 100 plans");
    report.check(
        "at draft, the layers are scaled with the extent rather than cropped by it",
        "480,270 480,270 480,270 480,270",
        corners(&preview::scale_plan(plan.clone(), PreviewQuality::Draft)),
    );
    report.check(
        "at full, the layers are left exactly as the export sees them",
        "1920,1080 1920,1080 1920,1080 1920,1080",
        corners(&preview::scale_plan(plan, PreviewQuality::Full)),
    );

    // -----------------------------------------------------------------------------------
    // The exit condition: a full-resolution preview is the exported frame, sample for sample
    // -----------------------------------------------------------------------------------

    let export = exported(&project);
    let full = preview_at(&project, FRAME, PreviewQuality::Full);
    let full_samples = full.encode(OutputDepth::Eight, OutputAlpha::Straight);

    report.check(
        "a full-resolution preview of frame 100 is the exported frame, sample for sample",
        format!("0 of {} samples differ", WIDTH * HEIGHT * 4),
        differing(&full_samples, &export),
    );
    report.check(
        "the frame that comparison ran on is the composition's own extent",
        "1920x1080",
        format!("{}x{}", full.width(), full.height()),
    );

    // The positive control. If the row above passed because both sides came from the same place,
    // or because `differing` cannot see a difference, this row would pass too — and it must not.
    let draft = preview_at(&project, FRAME, PreviewQuality::Draft);
    let draft_samples = draft.encode(OutputDepth::Eight, OutputAlpha::Straight);
    report.check(
        "a draft preview of the same frame is not the exported frame",
        "different lengths: 518400 and 8294400",
        differing(&draft_samples, &export),
    );
    report.check(
        "the draft frame is the extent draft claims",
        "480x270",
        format!("{}x{}", draft.width(), draft.height()),
    );

    // -----------------------------------------------------------------------------------
    // Draft renders the real scene, and does not fill a gap to look better at speed
    // -----------------------------------------------------------------------------------

    report.check(
        "at draft, a frame with a missing drawing has no layer 3 paint anywhere in it",
        "no layer 3 paint",
        has_layer3(
            &preview_at(&project, GAP_FRAME, PreviewQuality::Draft)
                .encode(OutputDepth::Eight, OutputAlpha::Straight),
        ),
    );
    report.check(
        "at draft, the frame either side of that gap does paint layer 3",
        "layer 3 paint present",
        has_layer3(
            &preview_at(&project, PAINTED_FRAME, PreviewQuality::Draft)
                .encode(OutputDepth::Eight, OutputAlpha::Straight),
        ),
    );

    // -----------------------------------------------------------------------------------
    // D-32: the playback clock holds real time
    // -----------------------------------------------------------------------------------

    report.check(
        "at rest, before playback, the work area's first frame is shown",
        "10",
        Playback::new(10, 13, rate(24, 1)).at_rest(),
    );

    // One frame at 24 fps is 41666666.66… ns. The boundary is between these two values, and
    // which side it falls on is the difference between playback that is right and playback that
    // runs half a frame early for its whole length.
    let mut clock = Playback::new(0, 239, rate(24, 1));
    report.check(
        "a nanosecond before the first frame ends, frame 0 is still on screen",
        "0",
        clock.at(Duration::from_nanos(41_666_666)).frame,
    );
    let mut clock = Playback::new(0, 239, rate(24, 1));
    report.check(
        "a nanosecond after it ends, frame 1 is",
        "1",
        clock.at(Duration::from_nanos(41_666_667)).frame,
    );
    // Document 20's own seconds-to-frame conversion rounds half away from zero, which would put
    // frame 1 on screen here. Playback is a different question and answers it differently.
    let mut clock = Playback::new(0, 239, rate(24, 1));
    report.check(
        "just past halfway through frame 0, frame 0 is still on screen, not frame 1",
        "0",
        clock.at(Duration::from_nanos(20_833_334)).frame,
    );

    // 24000/1001 is 23.976. One frame is 1001/24000 second, or 41708333.33… ns.
    let mut clock = Playback::new(0, 239, rate(24000, 1001));
    report.check(
        "at 23.976 the frame boundary is exact, not nearly right",
        "0 then 1",
        format!(
            "{} then {}",
            clock.at(Duration::from_nanos(41_708_333)).frame,
            Playback::new(0, 239, rate(24000, 1001))
                .at(Duration::from_nanos(41_708_334))
                .frame
        ),
    );

    // A machine that keeps up: asked once per frame time, it shows every frame and drops none.
    let mut clock = Playback::new(0, 239, rate(24, 1));
    let kept_up: Vec<(i32, u32)> = [0, 3, 6, 9]
        .iter()
        .map(|&n| {
            let shown = clock.at(frames24(n));
            (shown.frame, shown.skipped)
        })
        .collect();
    report.check(
        "asked once every third frame time, each answer is three frames on and two were passed over undrawn",
        "0+0 3+2 6+2 9+2",
        play(&kept_up),
    );
    report.check(
        "and that run reports what it cost",
        "Played 4 frames in real time and dropped 6 to keep the timing true. Step through the \
         frames to see every drawing, or switch the preview to draft resolution.",
        clock.report(),
    );

    let mut clock = Playback::new(0, 239, rate(24, 1));
    for n in [0, 3, 6, 9] {
        // Answered at the top of each frame it is asked for: 3/24 second apart, three frames
        // apart, so each answer is the frame after the two that were skipped.
        clock.at(frames24(n));
    }
    report.check(
        "the dropped frames are counted, not hidden",
        "shown 4, skipped 6",
        format!(
            "shown {}, skipped {}",
            clock.frames_shown(),
            clock.skipped()
        ),
    );

    let mut clock = Playback::new(0, 239, rate(24, 1));
    clock.at(Duration::ZERO);
    clock.at(Duration::from_nanos(41_666_667));
    clock.at(Duration::from_nanos(83_333_334));
    report.check(
        "a machine that does keep up drops nothing, and says so",
        "Played 3 frames in real time. No frames were dropped.",
        clock.report(),
    );

    // Work-area playback loops, which is what R-06a means by a work area.
    let mut clock = Playback::new(10, 13, rate(24, 1));
    let looped: Vec<i32> = [0, 3, 6, 9, 12]
        .iter()
        .map(|&n| clock.at(frames24(n)).frame)
        .collect();
    report.check(
        "playback loops inside the work area rather than running past its end",
        "10 13 12 11 10",
        looped
            .iter()
            .map(i32::to_string)
            .collect::<Vec<_>>()
            .join(" "),
    );

    // Time running backwards is the caller's fault. Reporting a skip count invented from it
    // would put a wrong number in front of the owner, which is worse than holding the frame.
    let mut clock = Playback::new(0, 239, rate(24, 1));
    clock.at(frames24(6));
    let back = clock.at(frames24(3));
    report.check(
        "if the clock the caller supplies runs backwards, the frame is held and nothing is \
         counted as dropped",
        "frame 6, skipped 0",
        format!("frame {}, skipped {}", back.frame, back.skipped),
    );

    // -----------------------------------------------------------------------------------
    // The artifact to look at
    // -----------------------------------------------------------------------------------

    let path = repo("verification/B-08_preview_draft_100.png");
    png_out::write_rgba(
        &path,
        draft.width(),
        draft.height(),
        OutputDepth::Eight,
        &[(
            "Source",
            "frame 100 of verification/B-08a_project.json at draft resolution, \
             by tests/b08_preview.rs"
                .to_string(),
        )],
        &draft_samples,
    )
    .unwrap_or_else(|e| panic!("write {}: {e}", path.display()));

    write_report(&report);
    let failed: Vec<&Row> = report.rows.iter().filter(|r| !r.pass()).collect();
    assert!(
        failed.is_empty(),
        "{} of {} checks failed, first: {} (expected {}, got {}); see \
         verification/B-08_preview_table.md",
        failed.len(),
        report.rows.len(),
        failed[0].check,
        failed[0].expected,
        failed[0].actual
    );
}

fn write_report(report: &Report) {
    let passed = report.rows.iter().filter(|r| r.pass()).count();
    let mut out = String::new();
    out.push_str("# B-08 preview: resolution selection and the playback clock\n\n");
    out.push_str(
        "The two decisions the owner made on 2026-09-05, turned into behaviour a table can \
         hold. **D-33**: the preview opens at draft resolution and says so. **D-32**: playback \
         holds real time, drops the frames it cannot deliver, and reports how many. Produced by \
         `tests/b08_preview.rs` from `verification/B-08a_project.json`, which is a real project \
         file.\n\n",
    );
    out.push_str(
        "The row that matters most is the fourth from the top of the second group: **a \
         full-resolution preview of frame 100 is the exported frame, sample for sample**. The \
         preview is composed through the display path and the export is written by the export \
         path, and all 8,294,400 samples agree. The row after it is the control that makes that \
         mean something — the same comparison against a draft preview, which must not agree.\n\n",
    );
    out.push_str(
        "`verification/B-08_preview_draft_100.png` is that draft frame, 480 by 270. Beside \
         `verification/B-08a_frames/frame_100.png`, which is the same frame at full size, it is \
         the same picture and a quarter of the width. There is still no window; that is the rest \
         of B-08.\n\n",
    );
    out.push_str("## Checks\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
    for row in &report.rows {
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            row.check,
            row.expected,
            row.actual,
            if row.pass() { "pass" } else { "**FAIL**" }
        ));
    }
    out.push_str(&format!(
        "\n**{passed} of {} checks pass.**\n",
        report.rows.len()
    ));
    let path = repo("verification/B-08_preview_table.md");
    fs::write(&path, out).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
}
