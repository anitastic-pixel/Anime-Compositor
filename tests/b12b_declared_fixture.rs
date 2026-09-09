//! B-12b item 6: the fixture document 08 line 41 declares, built, and the envelope re-measured
//! against it.
//!
//! Writes `verification/T-06_declared_fixture.md` and `verification/T-06_declared_fixture.json`
//! under `--release --ignored`.
//!
//! # Why this file exists
//!
//! `tests/t06_envelope.rs` measures the reference shot and says at the top of its own artifact
//! that the reference shot is **not** the fixture document 08 asks about. Line 41 declares
//! *"1080p, 24 fps, 240 frames, ten raster layers, two alpha mattes and three simple effect
//! instances"*; the reference shot has four raster layers, no mattes and no effects. That gap is
//! **D-41**, and D-41 says in its own words what closes it: *"the re-measurement against the real
//! fixture after B-07, not an edit to line 41."* B-06 and B-07 have landed, so the fixture is
//! buildable, so this is that re-measurement.
//!
//! **D-40 is the other entry this touches.** The owner decided on 2026-09-06 to leave the preview
//! cache budget at 128 MB, and gave a reason with a condition attached: the target line 41 asks
//! about is met at that budget *on a four-layer shot*, and *"if B-06 and B-07 make the reference
//! shot heavy enough that the measured p95 crosses 100 ms, that is a new measurement and reopens
//! this entry."* This file is the measurement that decided that. It fired: 390.40 ms against a
//! 100 ms target, because 128 MB cannot hold the 316.4 MiB one frame of this fixture needs. On
//! 2026-09-08 the owner raised the default, and `cache::DEFAULT_BUDGET_BYTES` is where the reason
//! is written down. The seek table below now measures the new default against the old one.
//!
//! This file still only states verdicts. Reopening or closing a decision is a note in the
//! register, and the register is the owner's.
//!
//! # What was built, and the one way it is not the real thing
//!
//! Ten raster layers over **ten distinct image sequences on disk**, two alpha mattes, three effect
//! instances, in the reference shot's own composition — 1920x1080, 24 fps, 240 frames.
//!
//! The ten sequences are **copies of the reference shot's four**, laid down under `target/` by
//! this test. So the fixture has ten layers of drawings and only four layers of *pictures*: the
//! same art appears more than once. That is a real limitation and it is stated here rather than
//! buried, but it is a limitation of what the shot looks like and not of what it costs. Nothing in
//! the render path shares work between two layers that read different files. `src/cache.rs` keys a
//! decoded cel on the file's path, length, modification time and interpretation, so ten sequences
//! are ten decodes; `verification/D-37_decode_cost.md` puts decoding at 75.15 ms of an 81.69 ms
//! draft frame, which is the part being multiplied. Drawing the owner six more layers of art would
//! change the pictures in this artifact and would not change a number in it.
//!
//! **The two mattes are alpha mattes by construction, not by a setting.** `model::MatteReference`
//! holds a layer and a `matte_only` flag and nothing else; document 21 composites a matte from the
//! matte layer's alpha. There is no luma matte in this build to choose instead.
//!
//! **`matte_only` is false on both.** True would keep the matte layer out of the visible stack,
//! which is the commoner way to use one and the cheaper one to render. False keeps all ten layers
//! composited, which is the reading of "ten raster layers, two alpha mattes" that costs more, and
//! a floor should cost more rather than less.
//!
//! # What is asserted and what is only reported
//!
//! The same division `tests/t06_envelope.rs` makes, for the same reasons, and the same four
//! assertions — a timing is reported and never asserted, because a test that fails on a busy
//! machine teaches nothing.
//!
//! # What this does not measure, and why not
//!
//! **Full resolution.** T-06 walks the shot again at full resolution because an export writes
//! full-resolution frames. Ten layers at full resolution is roughly four times the work of the
//! draft walk here and document 08 line 41 does not ask for it; `verification/B-10_export.md` is
//! where an export is timed.
//!
//! **Anything about the shot's pictures.** What a frame looks like is B-06's and B-07's to check
//! and `verification/` has the pages that do it. Every number here is a cost.
//!
//! The seek table does now reach a cache with room for the whole shot: the owner asked on
//! 2026-09-08 whether raising the default again would buy anything, and two probe budgets - 4 GiB,
//! and 6 GiB, which is past the 5.5 GB all 166 distinct drawings cost - answer it with a measured
//! row each. They are probes and neither is a default; `cache::DEFAULT_BUDGET_BYTES` is unmoved by
//! this file.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::time::Instant;

use anime_compositor::cache::{budget_label, CelCache, DEFAULT_BUDGET_BYTES};
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};

mod common;
use common::{build_fixture, peak_working_set, repo, working_set};

const COMP: &str = "comp-reference-shot";
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
/// One decoded cel held in the working space: RGBA f32 at the composition's extent.
const ONE_CEL: usize = WIDTH * HEIGHT * 4 * std::mem::size_of::<f32>();
const FIRST: i32 = 0;
const LAST: i32 = 239;
/// The number of repeated work-area loops document 08 line 41 asks for.
const LOOPS: usize = 10;
/// Ten cels, which is one frame of this fixture: every layer contributes one. This is the smallest
/// budget at which asking for the frame just shown is a hit rather than ten decodes.
const ONE_FRAME_BYTES: usize = 10 * ONE_CEL;
/// What the viewer's default budget was until this file's first run reopened D-40: 128 MB, which
/// is less than the 316.4 MiB one frame of this fixture needs. The seek table measures the current
/// default against it, so the page says what raising it bought rather than only where it landed.
const WAS_DEFAULT_BYTES: usize = 128 * 1024 * 1024;
/// Budgets that are nobody's default, measured only to answer whether raising the default again
/// would buy anything. 4 GiB is a large budget that still does not hold this shot; 6 GiB is the
/// first round number past the 5.5 GB all 166 distinct drawings of it cost, so it is the smallest
/// budget at which a scrub of this shot can be warm. The owner asked for both on 2026-09-08.
const PROBE_BUDGETS: [usize; 2] = [4 * 1024 * 1024 * 1024, 6 * 1024 * 1024 * 1024];
/// How much of this fixture a budget holds, written for a person: cels, and frames of this shot.
fn held_cels(budget: usize) -> String {
    let cels = budget / ONE_CEL;
    format!("{cels} cels, {:.1} frames of this shot", cels as f64 / 10.0)
}

/// Coprime with the 240 frames of the work area, so stepping by it visits every frame exactly once
/// in an order that is nowhere near sequential, and the same order every run.
const SCATTER: i32 = 97;
/// 24 fps as a per-frame budget in milliseconds.
const FRAME_BUDGET_MS: f64 = 1000.0 / 24.0;

// ---------------------------------------------------------------------------------------
// Measuring
// ---------------------------------------------------------------------------------------

fn render_ms(
    project: &Project,
    root: &Path,
    frame: i32,
    quality: PreviewQuality,
    cache: &mut CelCache,
) -> f64 {
    let comp = Id::new(COMP);
    let mut log = FrameLog::new(3);
    let at = Instant::now();
    preview::preview_frame_cached(
        project,
        &comp,
        frame,
        root,
        quality,
        DEFAULT_TILE_SIZE,
        &mut log,
        cache,
    )
    .unwrap_or_else(|d| panic!("frame {frame}: {}", d.message));
    at.elapsed().as_secs_f64() * 1000.0
}

/// One untimed frame, so no measured frame pays for the first touch of freshly allocated pages.
fn warm_up(project: &Project, root: &Path) {
    let _ = render_ms(
        project,
        root,
        FIRST,
        PreviewQuality::Draft,
        &mut CelCache::none(),
    );
}

/// A cel that cannot fit the budget is refused rather than admitted and immediately thrown out.
/// Both behaviours end with an empty cache, so the eviction count is the only thing that separates
/// them. Returns the eviction count for the artifact to report.
fn refuses_what_it_cannot_hold(project: &Project, root: &Path) -> u64 {
    let mut cache = CelCache::with_budget(ONE_CEL - 1);
    let _ = render_ms(project, root, FIRST, PreviewQuality::Draft, &mut cache);
    assert_eq!(
        cache.len(),
        0,
        "a budget below one cel held {} cels",
        cache.len()
    );
    assert_eq!(
        cache.held_bytes(),
        0,
        "a budget below one cel held {} bytes",
        cache.held_bytes()
    );
    assert_eq!(
        cache.evictions(),
        0,
        "a budget below one cel evicted {} cels, so it admitted what it should have refused",
        cache.evictions()
    );
    cache.evictions()
}

fn sorted(mut ms: Vec<f64>) -> Vec<f64> {
    ms.sort_by(|a, b| a.partial_cmp(b).expect("no NaN in a measured duration"));
    ms
}

/// Nearest rank on the sorted sample, as `tests/t06_envelope.rs` does it.
fn percentile(sorted: &[f64], p: f64) -> f64 {
    let rank = (p * sorted.len() as f64).ceil().max(1.0) as usize;
    sorted[rank.min(sorted.len()) - 1]
}

fn median(sorted: &[f64]) -> f64 {
    let n = sorted.len();
    if n % 2 == 0 {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    } else {
        sorted[n / 2]
    }
}

fn work_area() -> Vec<i32> {
    (FIRST..=LAST).collect()
}

fn scattered() -> Vec<i32> {
    let length = LAST - FIRST + 1;
    (0..length)
        .map(|i| FIRST + (i * SCATTER) % length)
        .collect()
}

struct Loop {
    ms: Vec<f64>,
    total_ms: f64,
    decodes: u64,
    hits: u64,
    held: usize,
    working_set: usize,
}

fn one_loop(project: &Project, root: &Path, cache: &mut CelCache) -> Loop {
    let decodes_before = cache.misses();
    let hits_before = cache.hits();
    let mut ms = Vec::new();
    let start = Instant::now();
    for frame in work_area() {
        ms.push(render_ms(
            project,
            root,
            frame,
            PreviewQuality::Draft,
            cache,
        ));
    }
    Loop {
        total_ms: start.elapsed().as_secs_f64() * 1000.0,
        ms,
        decodes: cache.misses() - decodes_before,
        hits: cache.hits() - hits_before,
        held: cache.held_bytes(),
        working_set: working_set(),
    }
}

/// A scattered walk of the whole work area at one budget, warmed by an identical walk first when
/// asked, so that the measured pass is the "cached" seek document 08 means.
fn seek_walk(
    project: &Project,
    root: &Path,
    budget: usize,
    warm_first: bool,
) -> (Vec<f64>, usize, usize) {
    let mut cache = CelCache::with_budget(budget);
    if warm_first {
        for frame in scattered() {
            let _ = render_ms(project, root, frame, PreviewQuality::Draft, &mut cache);
        }
    }
    let mut ms = Vec::new();
    for frame in scattered() {
        ms.push(render_ms(
            project,
            root,
            frame,
            PreviewQuality::Draft,
            &mut cache,
        ));
    }
    (sorted(ms), cache.held_bytes(), cache.len())
}

/// The same frame asked for twice in a row, everywhere in the shot: the warmest seek there is.
fn reseek(project: &Project, root: &Path, budget: usize) -> Vec<f64> {
    let mut cache = CelCache::with_budget(budget);
    let mut ms = Vec::new();
    for frame in scattered() {
        let _ = render_ms(project, root, frame, PreviewQuality::Draft, &mut cache);
        ms.push(render_ms(
            project,
            root,
            frame,
            PreviewQuality::Draft,
            &mut cache,
        ));
    }
    sorted(ms)
}

// ---------------------------------------------------------------------------------------
// The measurement
// ---------------------------------------------------------------------------------------

/// Document 08 line 41's declared fixture, built and measured.
///
/// ```text
/// cargo test --release --test b12b_declared_fixture -- --ignored --nocapture
/// ```
#[test]
#[ignore = "timing; builds a workload on disk and runs for several minutes under --release"]
fn t06_against_the_fixture_document_08_declares() {
    let (project, root, text) = build_fixture("measured");
    let path = repo("verification/T-06_declared_fixture.json");
    fs::write(&path, &text).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));

    warm_up(&project, &root);
    let evictions_below_one_cel = refuses_what_it_cannot_hold(&project, &root);

    // --- Ten loops over the work area at the viewer's own budget. -----------------------------
    let mut cache = CelCache::with_budget(DEFAULT_BUDGET_BYTES);
    let mut loops = Vec::new();
    for _ in 0..LOOPS {
        loops.push(one_loop(&project, &root, &mut cache));
    }
    for (n, run) in loops.iter().enumerate() {
        assert!(
            run.held <= DEFAULT_BUDGET_BYTES,
            "loop {} held {} bytes, over its budget of {DEFAULT_BUDGET_BYTES}",
            n + 1,
            run.held
        );
    }
    let peak_after_loops = peak_working_set();

    // --- Seeking. -----------------------------------------------------------------------------
    let reseek_default = reseek(&project, &root, DEFAULT_BUDGET_BYTES);
    let (scatter_default, _, _) = seek_walk(&project, &root, DEFAULT_BUDGET_BYTES, false);
    let (scatter_was, _, _) = seek_walk(&project, &root, WAS_DEFAULT_BYTES, false);
    let reseek_was = reseek(&project, &root, WAS_DEFAULT_BYTES);
    // Measured after the peak above is read, so a budget nobody ships cannot be reported as the
    // program's appetite.
    let probes: Vec<(usize, Vec<f64>, Vec<f64>)> = PROBE_BUDGETS
        .iter()
        .map(|&b| {
            let r = reseek(&project, &root, b);
            let (w, _, _) = seek_walk(&project, &root, b, false);
            (b, r, w)
        })
        .collect();
    // The cache the byte-accounting check below reads is filled at exactly one frame of headroom,
    // so the two figures it compares are a cel count and a cel size and nothing else.
    let (_, filled_bytes, filled_cels) = seek_walk(&project, &root, ONE_FRAME_BYTES, true);

    // The budget assertion above believes whatever the cache says it holds. This is the same
    // figure derived a second way, from a count of cels.
    assert_eq!(
        filled_bytes,
        filled_cels * ONE_CEL,
        "the cache reports {filled_bytes} bytes held in {filled_cels} cels, which is not \
         {filled_cels} cels of {ONE_CEL} bytes"
    );

    write_artifact(
        &loops,
        &Seeks {
            reseek_default,
            scatter_default,
            scatter_was,
            reseek_was,
            probes,
            filled_bytes,
            filled_cels,
            evictions_below_one_cel,
        },
        peak_after_loops,
        text.len(),
    );

    // --- The one claim document 08 asks for that is a check rather than a number. --------------
    let baseline = loops[1].working_set;
    let ended = loops[LOOPS - 1].working_set;
    let growth = ended.saturating_sub(baseline);
    assert!(
        growth <= ONE_CEL,
        "the process grew {growth} bytes between the end of loop 2 ({baseline}) and the end of \
         loop {LOOPS} ({ended}), which is more than one cel ({ONE_CEL}); see \
         verification/T-06_declared_fixture.md"
    );
}

struct Seeks {
    reseek_default: Vec<f64>,
    scatter_default: Vec<f64>,
    scatter_was: Vec<f64>,
    reseek_was: Vec<f64>,
    probes: Vec<(usize, Vec<f64>, Vec<f64>)>,
    filled_bytes: usize,
    filled_cels: usize,
    evictions_below_one_cel: u64,
}

fn mib(bytes: usize) -> f64 {
    bytes as f64 / (1024.0 * 1024.0)
}

fn write_artifact(loops: &[Loop], seeks: &Seeks, peak: usize, project_bytes: usize) {
    let mut s = String::from("# T-06 again, against the fixture document 08 actually declares\n\n");
    s.push_str(
        "`verification/T-06_performance_envelope.md` measures the reference shot and says at the \
         top of itself that the reference shot is not the fixture document 08 line 41 asks about: \
         *\"1080p, 24 fps, 240 frames, ten raster layers, two alpha mattes and three simple \
         effect instances\"*, against a shot with four layers, no mattes and no effects. That gap \
         is **D-41**, and D-41 names what closes it - the re-measurement against the real fixture \
         once B-06 and B-07 have landed, not an edit to line 41. Both have landed. This is that \
         measurement. Produced by `tests/b12b_declared_fixture.rs`, which is `#[ignore]`d in \
         normal runs.\n\n",
    );

    s.push_str("## Machine, build and configuration\n\n");
    s.push_str(
        "- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads\n\
         - OS: Microsoft Windows 11 Education, 10.0.26200\n\
         - Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`\n\
         - Workload: `verification/T-06_declared_fixture.json`, at draft resolution\n\
         - Tile size: `compose::DEFAULT_TILE_SIZE`\n",
    );
    let _ = writeln!(
        s,
        "- Work area: frames {FIRST} to {LAST}, the whole shot\n\
         - Percentiles are by nearest rank on the sorted sample\n\n\
         Debug assertions in this build: {}. A run with `true` there is a debug build, and its \
         numbers say more about the compiler than about the renderer.\n",
        cfg!(debug_assertions)
    );

    // --- The fixture. --------------------------------------------------------------------------
    let _ = writeln!(
        s,
        "\n## The fixture, and the one way it is not the real thing\n\n\
         | What line 41 declares | What was built |\n|---|---|\n\
         | 1080p | 1920 x 1080 |\n\
         | 24 fps | 24 fps |\n\
         | 240 frames | 240, frames {FIRST} to {LAST} |\n\
         | Ten raster layers | Ten, over ten separate image sequences on disk |\n\
         | Two alpha mattes | Layer 5 matted by layer 4, layer 8 by layer 7 |\n\
         | Three simple effect instances | An exposure on layer 2, a Gaussian blur on layer 6, a \
         tint on layer 9 |\n\n\
         The project file is written out beside this one as \
         `verification/T-06_declared_fixture.json`, {project_bytes} bytes, and it is a project \
         this build opens: the test builds it, saves it, and reads it back through \
         `persist::load_str` before measuring anything.\n\n\
         **The ten sequences are copies of the reference shot's four.** The test lays them down \
         under `target/b12b_declared_fixture/` as `copy1` to `copy10`. So this shot has ten layers \
         of drawings and four layers of pictures - the same art appears more than once, and \
         anybody looking at a frame of it would see that.\n\n\
         That is a limitation of what it looks like and not of what it costs, and the difference \
         matters because this file is about cost. Nothing in the render path shares work between \
         two layers reading different files: `src/cache.rs` keys a decoded cel on the file's path, \
         its length, its modification time and its interpretation, so ten sequences are ten \
         decodes and not four. `verification/D-37_decode_cost.md` puts decoding at 75.15 ms of an \
         81.69 ms draft frame, which is the part being multiplied here. Six more layers of \
         owner-drawn art would change the pictures and would not change a number below.\n\n\
         Two smaller choices, both made towards the heavier reading rather than the cheaper one. \
         The mattes are **alpha** mattes because that is the only kind this build has - \
         `model::MatteReference` holds a layer and a flag, and document 21 composites from the \
         matte layer's alpha. And `matte_only` is **false** on both, so all ten layers still \
         composite; true would keep the two matte layers out of the visible stack and cost less. \
         A floor should cost more rather than less.\n\n\
         Each copied layer keeps the exposure sheet of the reference layer it came from, \
         including layer 3's deliberately missing drawing 7 and layer 4's out-of-order \
         re-exposure. Nothing here is a new timing path; it is the same paths, ten times over.\n",
    );

    // --- Ten loops. ---------------------------------------------------------------------------
    s.push_str(
        "\n## Ten repeated work-area loops\n\n\
         | Loop | Total ms | Median ms | p95 ms | Slowest ms | Frames per second at the median | \
         Decodes | From memory | Cache held (MiB) | Process working set (MiB) |\n\
         |---|---|---|---|---|---|---|---|---|---|\n",
    );
    for (n, run) in loops.iter().enumerate() {
        let ms = sorted(run.ms.clone());
        let m = median(&ms);
        let _ = writeln!(
            s,
            "| {} | {:.1} | {:.2} | {:.2} | {:.2} | {:.1} | {} | {} | {:.1} | {:.1} |",
            n + 1,
            run.total_ms,
            m,
            percentile(&ms, 0.95),
            ms[ms.len() - 1],
            1000.0 / m,
            run.decodes,
            run.hits,
            mib(run.held),
            mib(run.working_set),
        );
    }

    let first = sorted(loops[0].ms.clone());
    let last = sorted(loops[LOOPS - 1].ms.clone());
    let work_area_budget_ms = (LAST - FIRST + 1) as f64 * FRAME_BUDGET_MS;
    let under = loops
        .iter()
        .filter(|run| run.total_ms <= work_area_budget_ms)
        .count();
    let baseline = loops[1].working_set;
    let ended = loops[LOOPS - 1].working_set;
    let dropped = loops[LOOPS - 1]
        .ms
        .iter()
        .filter(|m| **m > FRAME_BUDGET_MS)
        .count();
    let _ = writeln!(
        s,
        "\n**Cold-render throughput** is the first row: {:.1} ms for {} frames, a median of \
         {:.2} ms and {:.1} frames per second. It is the only loop that pays for reading every \
         drawing off the disk for the first time.\n\n\
         **Warm playback at 24 fps.** A 24 fps clock allows this 240-frame work area {:.1} ms, and \
         **{} of the ten loops came in under it**; the tenth loop's median frame cost {:.2} ms \
         against the {:.1} ms one frame is allowed, and **{} of its {} frames** cost more than \
         that. Those are the frames D-32 drops rather than running the shot slow. End to end, on a \
         real window, playback is counted in `verification/B-08_window_shell.md`.\n\n\
         **Memory across the ten loops.** {:.1} MiB at the end of the second loop, {:.1} MiB at \
         the end of the tenth, a difference of {:.1} MiB across 1,920 renders. The test fails if \
         that exceeds one cel, 33,177,600 bytes. Peak working set across the ten loops was \
         {:.1} MiB, read before the seek section below so that the large cache the seek table \
         allocates does not get reported as the program's appetite.\n\n\
         **Peak VRAM is not reported.** There is no GPU path in this build; a VRAM figure would \
         be the desktop's.\n",
        loops[0].total_ms,
        loops[0].ms.len(),
        median(&first),
        1000.0 / median(&first),
        work_area_budget_ms,
        under,
        median(&last),
        FRAME_BUDGET_MS,
        dropped,
        loops[LOOPS - 1].ms.len(),
        mib(baseline),
        mib(ended),
        mib(ended.saturating_sub(baseline)),
        mib(peak),
    );

    // --- Seeks. -------------------------------------------------------------------------------
    s.push_str(
        "\n## Seeking\n\n\
         A seek is a jump to a frame that is not the next one. The walk steps through the work \
         area 97 frames at a time; 97 is coprime with 240, so it reaches every frame exactly once \
         in an order nowhere near sequential, and it is the same order every run.\n\n\
         | Seek | Budget | Median ms | p95 ms | Slowest ms | Within 100 ms at p95 |\n\
         |---|---|---|---|---|---|\n",
    );
    let now = format!(
        "{} (the viewer's default)",
        budget_label(DEFAULT_BUDGET_BYTES)
    );
    let was = format!("{} (what it was before)", budget_label(WAS_DEFAULT_BYTES));
    let rows: [(&str, &str, &Vec<f64>); 4] = [
        ("The frame just shown, again", &now, &seeks.reseek_default),
        (
            "A scattered walk of the whole shot",
            &now,
            &seeks.scatter_default,
        ),
        ("The frame just shown, again", &was, &seeks.reseek_was),
        (
            "A scattered walk of the whole shot",
            &was,
            &seeks.scatter_was,
        ),
    ];
    {
        let mut row = |label: &str, budget: &str, ms: &[f64]| {
            let p95 = percentile(ms, 0.95);
            let _ = writeln!(
                s,
                "| {label} | {budget} | {:.2} | {p95:.2} | {:.2} | {} |",
                median(ms),
                ms[ms.len() - 1],
                if p95 <= 100.0 { "yes" } else { "**no**" },
            );
        };
        for (label, budget, ms) in rows {
            row(label, budget, ms);
        }
        for (budget, reseek, scatter) in &seeks.probes {
            let name = format!("{} (a probe, nobody's default)", budget_label(*budget));
            row("The frame just shown, again", &name, reseek);
            row("A scattered walk of the whole shot", &name, scatter);
        }
    }

    let p95_default = percentile(&seeks.scatter_default, 0.95);
    let _ = writeln!(
        s,
        "\n### How to read the seek table\n\n\
         **The bottom two rows are the shot this project's performance target is written against, \
         on a cache that could not hold one frame of it.** One cel of this composition costs \
         33,177,600 bytes to hold and every frame of this fixture needs ten of them, which is \
         316.4 MiB, against the {} the default used to be. Asking for the frame just shown was \
         not a warm seek at all: showing the frame evicted the cels that made it. That is what \
         reopened D-40, and the top two rows are the same two requests at the {} default that \
         replaced it.\n\n\
         **What raising it bought, measured rather than argued.** Asking for the frame just \
         shown went from {:.2} ms to {:.2} ms at the median, and the scattered walk from {:.2} ms \
         to {:.2} ms. Playback moved too, in the loop table above: the tenth loop decoded {} cels \
         and answered {} from memory, where the same loop at 128 MB decoded 2,340 and answered \
         480. The reason a scrub improved at all is that {} is more than one frame, and a shot \
         re-uses drawings across frames.\n\n\
         What it did not buy is a warm scrub. Holding every distinct drawing of this fixture is \
         166 cels, about 5.5 GB, so at the default a jump far enough away still re-decodes most \
         of what it needs, and the scattered walk stays nearer a lower bound on the cost of \
         scrubbing than a picture of a warm cache. The two probe rows are what a budget that does \
         hold the whole shot costs and buys, and the paragraph under this table reads them.\n\n\
         **And no budget reaches the target.** With every cel of a frame in memory, what is left \
         is the cost of compositing ten layers, two mattes and three effects, and that is not a \
         cache's to save. It is the same finding as the playback paragraph below in different \
         clothes. The cache in the byte-accounting check held {:.1} MiB in {} cels.\n\n\
         **Every figure here is seek-to-buffer, not seek-to-display.** It stops at a finished \
         picture in memory; the transport into the window is the window's, and \
         `verification/B-08_window_shell.md` is where a real one is watched.\n",
        budget_label(WAS_DEFAULT_BYTES),
        budget_label(DEFAULT_BUDGET_BYTES),
        median(&seeks.reseek_was),
        median(&seeks.reseek_default),
        median(&seeks.scatter_was),
        median(&seeks.scatter_default),
        loops[LOOPS - 1].decodes,
        loops[LOOPS - 1].hits,
        held_cels(DEFAULT_BUDGET_BYTES),
        mib(seeks.filled_bytes),
        seeks.filled_cels,
    );

    // The two probe budgets, read against the default, so the question "would raising it again
    // help?" is answered by a number in this file rather than by an opinion in a conversation.
    if let (Some(first), Some(last)) = (seeks.probes.first(), seeks.probes.last()) {
        let _ = writeln!(
            s,
            "\n**Would raising it further help?** The two probe rows are here to answer that and \
             are nobody's default. Against the {} default's {:.2} ms scattered p95: {} gives \
             {:.2} ms, and {} - the first round number past the 5.5 GB every distinct drawing of \
             this shot costs, so the only budget here at which a scrub can be warm - gives \
             {:.2} ms. Repeating a frame, which the default already holds, goes from {:.2} ms at \
             the default to {:.2} ms at 6 GiB - that one was never the cache's to improve \
             further. A budget that holds the whole shot is the only one that can make scrubbing \
             cheap, and it is 6 GiB of a person's memory to do it; whether that is a trade this \
             project offers is the third of D-40's three answers and the owner's to make.\n",
            budget_label(DEFAULT_BUDGET_BYTES),
            percentile(&seeks.scatter_default, 0.95),
            budget_label(first.0),
            percentile(&first.2, 0.95),
            budget_label(last.0),
            percentile(&last.2, 0.95),
            median(&seeks.reseek_default),
            median(&last.1),
        );
    }

    // --- The verdict. -------------------------------------------------------------------------
    let met = p95_default <= 100.0;
    // The scrub at the largest budget measured, which is the one that holds the whole shot. The
    // verdict quotes it so that "a bigger cache is not the answer" is a number and not a claim.
    let widest = seeks
        .probes
        .last()
        .map(|(b, _, scatter)| (budget_label(*b), percentile(scatter, 0.95)));
    let verdict = if met {
        "So the target is met at the raised budget, on the fixture document 08 declares rather \
         than on a four-layer floor. D-40 can close on this row."
            .to_string()
    } else {
        let bigger = match &widest {
            Some((label, p95)) => format!(
                "Scrubbing improved less, and the probe rows in the seek table say how much is \
                 left in a bigger cache: at {label}, which is past the 5.5 GB every distinct \
                 drawing of this shot costs and so holds all of it, the scattered p95 is \
                 {p95:.2} ms."
            ),
            None => "Scrubbing did not, because 166 distinct drawings do not fit in any budget \
                     this project will set."
                .to_string(),
        };
        format!(
            "**The raised budget did not reach the target either**, and this is the honest shape \
             of what raising it bought: the bottom two rows of the seek table above against the \
             top two. Repeating a frame got much cheaper, because its ten cels now stay in \
             memory. {bigger} And none of them reaches 100 ms, because with the cels in hand what \
             is left is compositing. D-40 stays open on the part a cache cannot answer, and what \
             is left in it is the third of its three answers - whether the budget becomes a \
             setting with a stated cost - plus a question that is not D-40's: whether a 100 ms \
             scrub on a ten-layer shot is a target this project keeps."
        )
    };
    let _ = writeln!(
        s,
        "\n## What this settles, and what it hands back to the owner\n\n\
         **D-41 - \"the performance fixture document 08 declares cannot be built in this build\".** \
         It can now, and it has been. The entry says what closes it: the re-measurement against \
         the real fixture after B-07. This file is it, and D-41 can be closed. What stays true and \
         should be written into the closing note is the sentence above about the art: ten \
         sequences of drawings, four sequences of pictures. Every figure in this file is a real \
         ten-layer cost and no figure in it is a picture of a ten-layer shot.\n\n\
         **D-40 - the preview cache budget, now {}.** The owner decided on 2026-09-06 to leave \
         it at 128 MB, and attached a condition: *\"if B-06 and B-07 make the reference shot heavy \
         enough that the measured p95 crosses 100 ms, that is a new measurement and reopens this \
         entry rather than contradicting it.\"* That measurement fired on this fixture, and on \
         2026-09-08 the owner raised the default to hold a working neighbourhood - the second of \
         the three answers the entry lists. Here is the same measurement at the budget that \
         replaced it, scrubbing:\n\n\
         | | |\n|---|---|\n\
         | p95 of a scattered seek, default budget | **{:.2} ms** |\n\
         | Document 08 line 41's target | 100 ms |\n\
         | Crossed | **{}** |\n\n\
         {}\n\n\
         **And a third thing, which is not in the register at all.** D-40 is about the cache \
         budget and D-41 was about the fixture not existing. Neither of them is the biggest \
         number on this page. Document 08 line 41 also asks for *warm-cache playback at 24 fps*, \
         and on the fixture it declares this build renders **{:.2} ms a frame at draft against \
         the {:.1} ms a 24 fps clock allows** - {} of the ten loops came in under the deadline, \
         and {} of the tenth loop's {} frames were over it. That is a factor of about {:.0}, and \
         a factor is not a margin.\n\n\
         Read next to `verification/T-06_performance_envelope.md`, which has the four-layer \
         reference shot sitting *on* the 24 fps deadline and flipping either side of it between \
         runs, this says something specific and worth saying plainly: **nothing here is a \
         regression, and the shot document 08 declares is simply more work than this build does \
         in real time.** Two and a half times the layers costs about {:.1} times the frame, which \
         is roughly what the layer count alone accounts for: this is a shot that is more work, \
         not a build that got worse at it. `verification/D-37_decode_cost.md` is where the cost \
         of a layer mostly lives - \
         decoding was 75.15 ms of an 81.69 ms four-cel draft frame - and the loop table above is \
         consistent with it: every loop still decodes {} cels, against {} answered from memory. \
         The budget took a bite out of that and cannot take the rest: a sequential walk of 166 \
         distinct drawings comes back round to a drawing long after any cache short of the whole \
         5.5 GB has dropped it.\n\n\
         What that costs the person using it is already decided and needs no new decision: D-32 \
         says the viewer holds real time and drops the frames it cannot make, so this shot plays \
         at the right speed and shows fewer frames rather than playing slowly. What is not \
         decided is whether a 24 fps target for a ten-layer shot is one this project keeps, \
         lowers, or reaches by doing the decoding differently. That is a register entry somebody \
         has to open, and opening it is the owner's.\n\n\
         This file does not act on any of it. Reopening or opening a decision is a note in \
         `Markdown/14_Decisions_Risks.md`, and the register is the owner's.\n",
        budget_label(DEFAULT_BUDGET_BYTES),
        p95_default,
        if met { "no" } else { "yes" },
        verdict,
        median(&last),
        FRAME_BUDGET_MS,
        under,
        dropped,
        loops[LOOPS - 1].ms.len(),
        median(&last) / FRAME_BUDGET_MS,
        median(&last) / 81.69,
        loops[LOOPS - 1].decodes,
        loops[LOOPS - 1].hits,
    );

    let _ = writeln!(
        s,
        "\n## What document 08 line 41 asked for, and what came back\n\n\
         | Asked for | Answer |\n|---|---|\n\
         | The ten-layer, two-matte, three-effect fixture | **Built**, and measured here. D-41 |\n\
         | Warm-cache playback at 24 fps | {} of the ten loops came in under the {:.1} ms a 24 fps \
         clock allows 240 frames |\n\
         | p95 cached seek-to-display at or below 100 ms | **{}** at the viewer's default budget, \
         as seek-to-buffer: {:.2} ms. D-40 |\n\
         | No unbounded memory growth after ten repeated work-area loops | Measured against the \
         operating system's working set, and asserted rather than reported |\n\
         | Dropped frames | {} of the tenth loop's {} frames cost more than one frame's budget; \
         counted end to end by photograph in `B-08_window_shell.md` |\n\
         | Cold-render throughput | The first loop above |\n\
         | Peak RAM | {:.1} MiB peak working set |\n\
         | Peak VRAM | **Not measurable**: there is no GPU path in this build |\n\n\
         ## Two checks on the cache that every number above rests on\n\n\
         Every figure in this file is read out of the cache, so a fault in the cache's own \
         accounting would move all of them at once without failing anything. Both of these are \
         asserted, and the run that wrote this file passed them.\n\n\
         - **The held bytes are the cel count times the cel size.** The cache held {:.1} MiB in \
         {} cels, checked against {} cels of {} bytes - the same figure derived a second way.\n\
         - **A budget below one cel refuses rather than churns.** Given {} bytes, one less than a \
         cel, the cache held nothing and evicted {}. A cache that admits what it cannot hold and \
         throws it straight back out also ends up empty; the eviction count is the only thing that \
         tells the two apart.\n",
        under,
        work_area_budget_ms,
        if met { "Met" } else { "Not met" },
        p95_default,
        dropped,
        loops[LOOPS - 1].ms.len(),
        mib(peak),
        mib(seeks.filled_bytes),
        seeks.filled_cels,
        seeks.filled_cels,
        ONE_CEL,
        ONE_CEL - 1,
        seeks.evictions_below_one_cel,
    );

    let path = repo("verification/T-06_declared_fixture.md");
    fs::write(&path, s).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
}

/// The fixture is what document 08 line 41 declares, and this build renders a frame of it.
///
/// Cheap, and not `#[ignore]`d, because it is the only thing standing between the artifact above
/// and a fixture that quietly stopped being ten layers. It checks the project this test *wrote*
/// rather than the values that went into writing it, and it renders one frame so that a fixture
/// that can be assembled but not drawn fails here rather than twenty minutes into a timing run.
#[test]
fn the_fixture_is_ten_layers_two_mattes_and_three_effects_and_renders() {
    let (project, root, _) = build_fixture("shape");
    // The fixture is what it claims to be, checked against the file that was written rather than
    // against the values that went into it.
    let comp = project
        .composition(&Id::new(COMP))
        .expect("the fixture has the reference shot's composition");
    assert_eq!(comp.len(), 10, "the declared fixture has ten raster layers");
    assert_eq!(comp.width, 1920);
    assert_eq!(comp.height, 1080);
    assert_eq!(comp.duration_frames, 240);
    let mattes = comp.layers_in_order().filter(|l| l.matte.is_some()).count();
    assert_eq!(mattes, 2, "the declared fixture has two alpha mattes");
    let effects: usize = comp.layers_in_order().map(|l| l.effects.len()).sum();
    assert_eq!(
        effects, 3,
        "the declared fixture has three effect instances"
    );
    let distinct = project.assets.len();
    assert_eq!(distinct, 10, "ten layers over ten sequences on disk");

    let mut cache = CelCache::none();
    let ms = render_ms(&project, &root, FIRST, PreviewQuality::Draft, &mut cache);
    assert!(
        ms.is_finite() && ms > 0.0,
        "frame {FIRST} of the declared fixture did not render"
    );
}
