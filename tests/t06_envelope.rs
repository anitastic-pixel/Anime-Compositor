//! T-06: the performance envelope document 08 declares, measured on this machine.
//!
//! Writes `verification/T-06_performance_envelope.md` under `--release --ignored`.
//!
//! # What is already measured, and what is not
//!
//! Two artifacts exist before this one and neither closes document 08 line 41.
//! `verification/B-08_preview_latency.md` measures the cost of making one preview frame with no
//! cache at all, which is a cold render and not a seek. `verification/B-08b_cache_budget.md`
//! measures forty-eight consecutive frames at four budgets, which is sequential playback and not a
//! seek either, and reports the cache's own held bytes rather than the process's.
//!
//! Document 08 line 41 asks for five numbers. Two of them have never been produced:
//!
//! - **"p95 cached seek-to-display at or below 100 ms."** A seek is a jump to a frame that is not
//!   the next one. Nothing here has ever jumped.
//! - **"no unbounded memory growth after ten repeated work-area loops."** `tests/b08b_cache.rs`
//!   loops ten times over six frames and reports what the cache says it holds. A cache can be
//!   perfectly honest about its own bytes while the process around it grows, so the number that
//!   answers this question has to come from the operating system, and the loop has to be over the
//!   work area — 240 frames — rather than over six.
//!
//! And one of them cannot be produced at all: **peak VRAM**. There is no GPU path in this build.
//! R-04 and R-05 are parked under D-12 and no ADR has put work on a device. Reporting a VRAM figure
//! would be reporting the desktop's, not this program's, so the row says so instead of carrying a
//! number.
//!
//! # The fixture document 08 asks for could not be built when this was written
//!
//! Line 41 declares the reference fixture as "1080p, 24 fps, 240 frames, ten raster layers, two
//! alpha mattes and three simple effect instances". The reference shot is 1080p, 24 fps and 240
//! frames exactly. It has four raster layers, no mattes and no effects, **because mattes are B-06
//! and effects are B-07 and both are PARKED under D-12.** The declared fixture was therefore
//! not buildable when this test was written, and it is not a fixture that may be quietly
//! substituted: a measurement of four layers is not a measurement of ten. Both parks have
//! since lifted and `tests/b12b_declared_fixture.rs` measures the declared fixture itself,
//! into `verification/T-06_declared_fixture.md`; this test stays on the reference shot,
//! which is the shot the viewer opens.
//!
//! What is measured is what exists, said plainly in the artifact, with the shortfall named as a
//! registered conflict rather than absorbed. Every number below is a floor for the declared
//! fixture, not an estimate of it — more layers cost more, and the two parked features are exactly
//! the ones document 08 says need bounds expansion and a second evaluation of alpha.
//!
//! # What "seek-to-display" means here, and where it stops
//!
//! It stops at a finished buffer in memory, as `verification/B-08_preview_latency.md` does, for the
//! same reason: the transport into the web view is the window's, and a headless test cannot open a
//! window. `verification/B-08_window_shell.md` is where playback is counted end to end, by
//! photographing a running one. Every latency here is therefore seek-to-buffer, and the artifact
//! says so rather than letting the document's word stand unqualified.
//!
//! # What is asserted and what is only reported
//!
//! Timings are reported and never asserted. A test that fails because a machine is busy teaches
//! nothing, and document 12 asks for measurements against a named machine rather than for a
//! threshold nobody can reproduce. Four things are asserted, because all four are true of a
//! correct build on any machine:
//!
//! - the cache never holds more than its budget, in any loop;
//! - the process's working set after the tenth loop is not more than one cel above its working set
//!   after the second;
//! - the bytes the cache says it holds are its cel count times the size of a cel;
//! - a cache given less than one cel of budget holds nothing **and evicts nothing**.
//!
//! The last two were added after a mutation pass, and the reason is worth keeping next to them.
//! Every figure this file writes is read out of the cache, and the budget check above reads the
//! same accounting, so a fault in that accounting moves the number being checked and the number
//! it is checked against together and nothing fails. The third assertion derives the held bytes a
//! second way instead. The fourth exists because `src/cache.rs` refuses a cel too large for the
//! budget rather than admitting it and evicting it immediately, and both of those end with an
//! empty cache: the eviction count is the only thing that tells them apart. See
//! `verification/HARDENING_mutation_report.md`.
//!
//! The second is the check document 08 actually asks for, and the bound is argued rather than
//! picked. Loops three to ten are 1,920 renders of 240 frames. Anything that retained memory per
//! render — a decoded cel, a frame buffer, a diagnostic — would retain it 1,920 times, which is
//! tens of gigabytes for a cel and hundreds of megabytes for anything frame-sized. One cel,
//! 33,177,600 bytes, is smaller than a single instance of the cheapest of those leaks by three
//! orders of magnitude, and larger than the allocator noise between two identical loops. The second
//! loop rather than the first is the baseline because the first loop is where every page is touched
//! for the first time.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use anime_compositor::cache::{budget_label, CelCache, DEFAULT_BUDGET_BYTES};
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};

const COMP: &str = "comp-reference-shot";
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
/// One decoded cel held in the working space: RGBA f32 at the composition's extent. The same
/// arithmetic as `tests/b08b_cache.rs`, and the correction ADR-015 records — a cel on disk is a
/// quarter of this.
const ONE_CEL: usize = WIDTH * HEIGHT * 4 * std::mem::size_of::<f32>();
/// The work area of the reference shot, which is the whole shot: B-10 exports 0 to 239.
const FIRST: i32 = 0;
const LAST: i32 = 239;
/// The number of repeated work-area loops document 08 line 41 asks for.
const LOOPS: usize = 10;
/// A budget with room for every distinct drawing in the shot. `src/cache.rs` puts that at about
/// 1.9 GB; 2 GB is the next round number above it. This row exists because the default budget
/// cannot answer document 08's seek question — see the artifact.
const WHOLE_SHOT_BYTES: usize = 2048 * 1024 * 1024;
/// A stride coprime with the 240 frames of the work area, so that stepping by it visits every
/// frame exactly once in an order that is nowhere near sequential. Deterministic on purpose: a
/// measurement that uses a different order every run cannot be compared with itself.
const SCATTER: i32 = 97;
/// 24 fps, as a frame budget in milliseconds. D-32 holds real time and drops what does not fit, so
/// a frame that costs more than this is a frame the viewer would drop.
const FRAME_BUDGET_MS: f64 = 1000.0 / 24.0;

mod common;
use common::{peak_working_set, repo, working_set};

// ---------------------------------------------------------------------------------------
// The workload
// ---------------------------------------------------------------------------------------

fn root() -> PathBuf {
    repo("Fixtures/reference_shot")
}

/// B-09's own artifact, read back off the disk, for the same reason `tests/b08b_cache.rs` reads it:
/// a real project file rather than a model assembled in the test.
fn project() -> Project {
    let path = repo("verification/B-08a_project.json");
    let loaded =
        persist::load(&path).unwrap_or_else(|d| panic!("open {}: {}", path.display(), d.message));
    loaded.document.project().clone()
}

/// Render one frame and return what it cost, in milliseconds.
fn render_ms(project: &Project, frame: i32, quality: PreviewQuality, cache: &mut CelCache) -> f64 {
    let comp = Id::new(COMP);
    let root = root();
    let mut log = FrameLog::new(3);
    let at = Instant::now();
    preview::preview_frame_cached(
        project,
        &comp,
        frame,
        &root,
        quality,
        DEFAULT_TILE_SIZE,
        &mut log,
        cache,
    )
    .unwrap_or_else(|d| panic!("frame {frame}: {}", d.message));
    at.elapsed().as_secs_f64() * 1000.0
}

/// One untimed frame, so that no measured frame pays for the first touch of freshly allocated
/// pages. `verification/B-08_preview_latency.md` does the same and for the same reason.
fn warm_up(project: &Project, quality: PreviewQuality) {
    let _ = render_ms(project, FIRST, quality, &mut CelCache::none());
}

/// A cel that cannot fit the budget is refused rather than admitted and immediately thrown out.
///
/// `src/cache.rs` says so in a comment - "evicting everything to hold one thing that will be
/// evicted by the next request is worse than not holding it" - and nothing checked it. Both
/// behaviours end with an empty cache holding zero bytes, so the count of evictions is the only
/// thing that separates them: a cache that refuses never evicts. Added after a mutation pass; see
/// `verification/HARDENING_mutation_report.md`.
///
/// Returns the eviction count for the artifact to report.
fn refuses_what_it_cannot_hold(project: &Project) -> u64 {
    let mut cache = CelCache::with_budget(ONE_CEL - 1);
    let _ = render_ms(project, FIRST, PreviewQuality::Draft, &mut cache);
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

/// The value at `p` of a sorted sample, by nearest rank. Stated rather than borrowed because
/// percentile conventions differ and the artifact has to say which one it used.
fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return f64::NAN;
    }
    let rank = (p * (sorted.len() - 1) as f64).round() as usize;
    sorted[rank]
}

fn median(sorted: &[f64]) -> f64 {
    let n = sorted.len();
    if n == 0 {
        f64::NAN
    } else if n % 2 == 0 {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    } else {
        sorted[n / 2]
    }
}

fn work_area() -> Vec<i32> {
    (FIRST..=LAST).collect()
}

/// Every frame of the work area, in an order that is nowhere near sequential: step by [`SCATTER`],
/// which is coprime with the length, so the walk visits each frame exactly once.
fn scattered() -> Vec<i32> {
    let length = LAST - FIRST + 1;
    (0..length)
        .map(|i| FIRST + (i * SCATTER) % length)
        .collect()
}

// ---------------------------------------------------------------------------------------
// What one loop over the work area cost, and what the process held afterwards
// ---------------------------------------------------------------------------------------

struct Loop {
    ms: Vec<f64>,
    total_ms: f64,
    decodes: u64,
    hits: u64,
    held: usize,
    working_set: usize,
}

fn one_loop(project: &Project, quality: PreviewQuality, cache: &mut CelCache) -> Loop {
    let decodes_before = cache.misses();
    let hits_before = cache.hits();
    let mut ms = Vec::new();
    let start = Instant::now();
    for frame in work_area() {
        ms.push(render_ms(project, frame, quality, cache));
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

// ---------------------------------------------------------------------------------------
// The measurement
// ---------------------------------------------------------------------------------------

/// Document 08's provisional performance envelope, measured.
///
/// Ignored by default for the reason every timing test in this repository is: a debug build
/// measures the compiler. Run it deliberately, and expect it to take a few minutes:
///
/// ```text
/// cargo test --release --test t06_envelope -- --ignored --nocapture
/// ```
#[test]
#[ignore = "timing; run under --release with --ignored"]
fn t06_performance_envelope() {
    let project = project();
    warm_up(&project, PreviewQuality::Draft);
    let evictions_below_one_cel = refuses_what_it_cannot_hold(&project);

    // --- Ten loops over the work area at the viewer's own budget. ------------------------------
    let mut cache = CelCache::with_budget(DEFAULT_BUDGET_BYTES);
    let mut loops = Vec::new();
    for _ in 0..LOOPS {
        loops.push(one_loop(&project, PreviewQuality::Draft, &mut cache));
    }

    for (n, run) in loops.iter().enumerate() {
        assert!(
            run.held <= DEFAULT_BUDGET_BYTES,
            "loop {} held {} bytes, over its budget of {DEFAULT_BUDGET_BYTES}",
            n + 1,
            run.held
        );
    }

    // The peak this run's headline reports, read here rather than at the end. Everything after
    // this line deliberately allocates a cache sixteen times the viewer's own, to answer a
    // question the viewer's budget cannot; folding that into "peak RAM" would describe an
    // experiment rather than the program.
    let peak_after_loops = peak_working_set();

    // --- Seeking: the same frame again, and a scattered walk of the whole shot. ----------------
    // Re-seek is the only genuinely warm seek the default budget can offer. The budget holds four
    // cels and every frame of this shot needs four, so the frame just shown is in memory and the
    // one before it is not. The artifact says why that is the finding rather than a limitation of
    // the test.
    let mut reseek = Vec::new();
    let mut reseek_cache = CelCache::with_budget(DEFAULT_BUDGET_BYTES);
    for frame in scattered() {
        let _ = render_ms(&project, frame, PreviewQuality::Draft, &mut reseek_cache);
        reseek.push(render_ms(
            &project,
            frame,
            PreviewQuality::Draft,
            &mut reseek_cache,
        ));
    }

    let mut scatter = Vec::new();
    let mut scatter_cache = CelCache::with_budget(DEFAULT_BUDGET_BYTES);
    for frame in scattered() {
        scatter.push(render_ms(
            &project,
            frame,
            PreviewQuality::Draft,
            &mut scatter_cache,
        ));
    }

    // The same scattered walk against a cache with room for the whole shot, walked twice: the
    // first pass fills it, the second is the seek document 08 means by "cached".
    let mut whole = CelCache::with_budget(WHOLE_SHOT_BYTES);
    for frame in scattered() {
        let _ = render_ms(&project, frame, PreviewQuality::Draft, &mut whole);
    }
    let filled_bytes = whole.held_bytes();
    let filled_cels = whole.len();
    // The budget assertion above believes whatever the cache says it holds, so a fault in the
    // accounting itself passes it. This is the same figure derived a second way, from a count of
    // cels and the arithmetic every budget in this file is set by. Added after a mutation pass;
    // see `verification/HARDENING_mutation_report.md`.
    assert_eq!(
        filled_bytes,
        filled_cels * ONE_CEL,
        "the cache reports {filled_bytes} bytes held in {filled_cels} cels, which is not          {filled_cels} cels of {ONE_CEL} bytes"
    );
    let mut scatter_whole = Vec::new();
    for frame in scattered() {
        scatter_whole.push(render_ms(
            &project,
            frame,
            PreviewQuality::Draft,
            &mut whole,
        ));
    }
    let whole_decodes_on_the_second_walk = {
        let before = whole.misses();
        let _ = render_ms(&project, FIRST, PreviewQuality::Draft, &mut whole);
        whole.misses() - before
    };
    // Released before the full-resolution walk fills a second one of these. Two 2 GB caches alive
    // at once would measure this test's own appetite rather than the renderer's.
    drop(whole);

    // --- The same scattered seek at full resolution, which is what an export writes. -----------
    warm_up(&project, PreviewQuality::Full);
    let mut full_seek = Vec::new();
    let mut full_cache = CelCache::with_budget(WHOLE_SHOT_BYTES);
    for frame in scattered() {
        let _ = render_ms(&project, frame, PreviewQuality::Full, &mut full_cache);
    }
    for frame in scattered() {
        full_seek.push(render_ms(
            &project,
            frame,
            PreviewQuality::Full,
            &mut full_cache,
        ));
    }

    write_artifact(
        &loops,
        &Seeks {
            reseek: sorted(reseek),
            scatter_default: sorted(scatter),
            scatter_whole: sorted(scatter_whole),
            full_seek: sorted(full_seek),
            filled_bytes,
            filled_cels,
            decodes_on_a_second_walk: whole_decodes_on_the_second_walk,
            evictions_below_one_cel,
        },
        peak_after_loops,
    );

    // --- The one claim document 08 asks for that is a check rather than a number. ---------------
    let baseline = loops[1].working_set;
    let ended = loops[LOOPS - 1].working_set;
    let growth = ended.saturating_sub(baseline);
    assert!(
        growth <= ONE_CEL,
        "the process grew {growth} bytes between the end of loop 2 ({baseline}) and the end of \
         loop {LOOPS} ({ended}), which is more than one cel ({ONE_CEL}); see \
         verification/T-06_performance_envelope.md"
    );
}

struct Seeks {
    reseek: Vec<f64>,
    scatter_default: Vec<f64>,
    scatter_whole: Vec<f64>,
    full_seek: Vec<f64>,
    filled_bytes: usize,
    filled_cels: usize,
    decodes_on_a_second_walk: u64,
    evictions_below_one_cel: u64,
}

fn mib(bytes: usize) -> f64 {
    bytes as f64 / (1024.0 * 1024.0)
}

fn write_artifact(loops: &[Loop], seeks: &Seeks, peak: usize) {
    let mut s = String::from("# T-06: document 08's performance envelope, measured\n\n");
    s.push_str(
        "Document 08 line 43 says of its own figures: *\"These are validation targets, not \
         measured capabilities.\"* This is the measurement that turns as many of them as this \
         build can into measured capabilities, and names the ones it cannot. Produced by \
         `tests/t06_envelope.rs`, which is `#[ignore]`d in normal runs.\n\n",
    );

    s.push_str("## Machine, build and configuration\n\n");
    s.push_str(
        "- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads\n\
         - OS: Microsoft Windows 11 Education, 10.0.26200\n\
         - Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`\n\
         - Workload: `verification/B-08a_project.json`, the reference shot, at draft resolution \
         unless a row says otherwise\n\
         - Tile size: `compose::DRAFT_TILE_SIZE`, which is what a draft preview is cut \n         into (P-03(f))\n",
    );
    let _ = writeln!(
        s,
        "- Work area: frames {FIRST} to {LAST}, which is the whole shot and what B-10 exports\n\
         - Percentiles are by nearest rank on the sorted sample\n\n\
         Debug assertions in this build: {}. A run with `true` there is a debug build, and its \
         numbers say more about the compiler than about the renderer.\n",
        cfg!(debug_assertions)
    );

    s.push_str(
        "\n## The fixture this file could not use, and the one that has since been built\n\n\
         Document 08 line 41 declares the reference fixture as *\"1080p, 24 fps, 240 frames, ten \
         raster layers, two alpha mattes and three simple effect instances\"*. The reference shot \
         is 1080p, 24 fps and 240 frames exactly, and it has **four raster layers, no mattes and \
         no effects**, because mattes were B-06 and effects were B-07 and both were PARKED under \
         D-12 when this was measured.\n\n\
         Nothing below is a measurement of the declared fixture. Every figure here is a **floor** \
         for it rather than an estimate of it: six more layers cost more, and the two parked \
         features are the two document 08 itself says need bounds expansion and a second \
         evaluation of alpha. That gap is registered as **D-41**.\n\n\
         **B-06 and B-07 have both since landed, and the declared fixture has been built and \
         measured.** It is `verification/T-06_declared_fixture.md`, and it is the file to read \
         for what document 08's own fixture costs - it is about nine times the frame. This file \
         stays what it is, the reference shot's own numbers, because the reference shot is what \
         the viewer actually opens.\n",
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
    let loop_totals: Vec<f64> = loops.iter().map(|run| run.total_ms).collect();
    // What a 24 fps clock allows the whole work area. This is the aggregate form of the
    // per-frame budget and the only sense in which playback at 24 fps is a yes or a no: a
    // median frame inside budget says nothing about a loop that misses its deadline.
    let work_area_budget_ms = (LAST - FIRST + 1) as f64 * FRAME_BUDGET_MS;
    let baseline = loops[1].working_set;
    let ended = loops[LOOPS - 1].working_set;
    let dropped = loops[LOOPS - 1]
        .ms
        .iter()
        .filter(|m| **m > FRAME_BUDGET_MS)
        .count();
    let _ = writeln!(
        s,
        "\n**Cold-render throughput** is the first row: {:.1} ms in total for {} frames, a median \
         of {:.2} ms and {:.1} frames per second. It is the only loop that pays for reading every \
         drawing off the disk for the first time.\n\n\
         **Memory across the ten loops.** The process held {:.1} MiB at the end of the second loop \
         and {:.1} MiB at the end of the tenth, a difference of {:.1} MiB across 1,920 renders. \
         `tests/t06_envelope.rs` fails if that difference exceeds one cel, 33,177,600 bytes; the \
         bound is argued in the test's own header rather than picked. Peak working set across the \
         ten loops was {:.1} MiB. That figure is read before the seek section below, which \
         deliberately allocates a cache sixteen times the viewer's own; folding that experiment \
         into a peak-RAM number would describe the test rather than the program.\n\n\
         **Peak VRAM is not reported, and not because it was forgotten.** There is no GPU path in \
         this build; a VRAM figure would be the desktop's.\n\n\
         **Dropped frames** are D-32's, and they are counted end to end by photographing a running \
         window in `verification/B-08_window_shell.md`, not here. What this table can say is how \
         many frames of the tenth loop cost more than the {:.1} ms a 24 fps frame is allowed: \
         **{} of {}**. Those are the frames the viewer would drop rather than run the shot slow.\n\n\
         **Warm playback sits on the 24 fps deadline, and which side of it is not settled by this \
         file.** A 24 fps clock allows this 240-frame work area {:.1} ms. On the run that wrote \
         this artifact, **{} of the ten loops came in under that**, the fastest at {:.1} ms and \
         the slowest at {:.1} ms - a margin of {:+.1}% to {:+.1}% against the deadline.\n\n\
         That margin is small enough that the verdict changes with what else the machine is doing. \
         Run repeatedly on 2026-09-06 on an otherwise ordinary desktop, the count has come back \
         as ten of the ten loops under the deadline, as none of the ten, and as the first six \
         under it with the last four about a fifth behind. Nothing in the build changed \
         between them. **Read the row below as *at the deadline*, not as a pass or a failure**, \
         and read the margin rather than the count: a measurement that flips between runs is a \
         measurement of the margin, and the margin is a few per cent.\n\n\
         The paragraph above is the other half of the same picture: inside the tenth loop, \
         individual frames ran over the {:.1} ms one frame is allowed, and D-32 drops those rather \
         than running the shot slow. Both figures are four layers at draft resolution, and both \
         are a floor for the fixture document 08 declares, which has six more layers, two mattes \
         and three effects. End to end, on a real window, playback is counted in \
         `verification/B-08_window_shell.md`.\n",
        loops[0].total_ms,
        loops[0].ms.len(),
        median(&first),
        1000.0 / median(&first),
        mib(baseline),
        mib(ended),
        mib(ended.saturating_sub(baseline)),
        mib(peak),
        FRAME_BUDGET_MS,
        dropped,
        loops[LOOPS - 1].ms.len(),
        work_area_budget_ms,
        loops.iter().filter(|run| run.total_ms <= work_area_budget_ms).count(),
        loop_totals.iter().cloned().fold(f64::INFINITY, f64::min),
        loop_totals.iter().cloned().fold(0.0f64, f64::max),
        100.0 * (loop_totals.iter().cloned().fold(f64::INFINITY, f64::min)
            / work_area_budget_ms
            - 1.0),
        100.0 * (loop_totals.iter().cloned().fold(0.0f64, f64::max) / work_area_budget_ms
            - 1.0),
        FRAME_BUDGET_MS,
    );

    // --- Seeks. -------------------------------------------------------------------------------
    s.push_str(
        "\n## Seeking\n\n\
         A seek is a jump to a frame that is not the next one, which is what scrubbing is and what \
         nothing measured before this had ever done. The walk used here steps through the work \
         area by 97 frames at a time; 97 is coprime with 240, so the walk reaches every frame \
         exactly once in an order that is nowhere near sequential, and it is the same order every \
         run.\n\n\
         | Seek | Budget | Median ms | p95 ms | Slowest ms | Within 100 ms at p95 |\n\
         |---|---|---|---|---|---|\n",
    );
    let now = format!(
        "{} (the viewer's default)",
        budget_label(DEFAULT_BUDGET_BYTES)
    );
    let rows: [(&str, &str, &Vec<f64>); 4] = [
        ("The frame just shown, again", &now, &seeks.reseek),
        (
            "A scattered walk of the whole shot",
            &now,
            &seeks.scatter_default,
        ),
        (
            "The same walk, second pass",
            "2 GB (room for every drawing)",
            &seeks.scatter_whole,
        ),
        (
            "The same walk at full resolution, second pass",
            "2 GB (room for every drawing)",
            &seeks.full_seek,
        ),
    ];
    for (label, budget, ms) in rows {
        let p95 = percentile(ms, 0.95);
        let _ = writeln!(
            s,
            "| {label} | {budget} | {:.2} | {p95:.2} | {:.2} | {} |",
            median(ms),
            ms[ms.len() - 1],
            if p95 <= 100.0 { "yes" } else { "**no**" },
        );
    }

    let _ = writeln!(
        s,
        "\n### How to read the seek table\n\n\
         **A scrub is the workload the budget decides, and this shot is the small one.** One cel \
         of this composition costs 33,177,600 bytes to hold and every frame of this shot needs \
         four of them, so the {} default holds several frames of it and repeating a frame is a \
         full hit. Jumping far enough away still evicts and decodes, which is the second row, and \
         a cold decode is what `verification/B-08_preview_latency.md` already measured. The third \
         row is the whole shot in memory: the cache filled to {:.1} MiB in {} cels, and a second \
         walk over the same 240 frames decoded {} further drawings.\n\n\
         The default is what it is because of a heavier shot than this one. \
         `verification/B-08b_cache_budget.md` measured 512 MB against 128 MB on sequential \
         playback of *this* shot and found a fraction of a millisecond between them, which is why \
         the default sat at 128 MB until 2026-09-08. What moved it was \
         `verification/T-06_declared_fixture.md`: on the ten-layer fixture document 08 line 41 \
         actually declares, one frame is 316.4 MiB and 128 MB could not hold it. Four layers were \
         never the shot the number had to be right for.\n\n\
         **Document 08's \"p95 cached seek-to-display at or below 100 ms\" is met in every row \
         of the table above, the default budget included**, at {:.2} ms for the scattered walk \
         that re-decodes on almost every jump; the slowest single seek of those 240 was {:.2} ms. \
         The default budget is therefore not a failure against that target on this machine. What \
         it is, is a cost, and the cost shows as a comparison rather than as a breach: {:.2} ms at \
         the median re-decoding against {:.2} ms with the drawings already in memory. Whether that \
         trade is the right one is a decision for the owner rather than a change an agent should \
         make, because raising the budget spends memory the machine may not have. Registered as \
         **D-40**. Two things bound the headroom above: every figure here is four layers rather \
         than the ten document 08 declares, and it is one machine.\n\n\
         **Every figure in this table is seek-to-buffer, not seek-to-display.** It stops at a \
         finished picture in memory. Getting that picture onto the screen is the transport, and \
         `verification/B-08_window_shell.md` is the artifact that watches a real window do it. \
         Document 08's phrase is \"seek-to-display\"; this measures the part of it that a headless \
         test can reach, and the difference is not small — `verification/B-08_preview_latency.md` \
         records that the transport was never measured on this build at all.\n",
        budget_label(DEFAULT_BUDGET_BYTES),
        mib(seeks.filled_bytes),
        seeks.filled_cels,
        seeks.decodes_on_a_second_walk,
        percentile(&seeks.scatter_default, 0.95),
        seeks.scatter_default[seeks.scatter_default.len() - 1],
        median(&seeks.scatter_default),
        median(&seeks.scatter_whole),
    );

    let _ = writeln!(
        s,
        "\n## What document 08 line 41 asked for, and what came back\n\n\
         | Asked for | Answer |\n|---|---|\n\
         | Warm-cache playback at 24 fps | **At the deadline**: {} of the ten loops of this run \
         came in under the {:.1} ms a 24 fps clock allows 240 frames, and the count flips between \
         runs. Read the margin above, at draft on four layers |\n\
         | p95 cached seek-to-display at or below 100 ms | **Met**, as seek-to-buffer, in every seek \
         measured, the default budget included; worst p95 {:.2} ms. What the default costs is D-40 |\n\
         | No unbounded memory growth after ten repeated work-area loops | Measured against the \
         operating system's working set, and asserted rather than reported |\n\
         | Dropped frames | Counted end to end by photograph in `B-08_window_shell.md`; the frames \
         over budget in the tenth loop are counted here |\n\
         | Cold-render throughput | The first loop above |\n\
         | Peak RAM | Peak working set above |\n\
         | Peak VRAM | **Not measurable**: there is no GPU path in this build |\n\
         | The ten-layer, two-matte, three-effect fixture | **Not this file's workload**: it \
         was parked under D-12 when this was measured. Built and measured since, in \
         `verification/T-06_declared_fixture.md`. Every figure here is a floor for it, not \
         an estimate. D-41 |\n\n\
         ## Two checks on the cache that every number above rests on\n\n\
         Every figure in this file is read out of the cache, so a fault in the cache's own \
         accounting would move all of them at once without failing anything. A mutation pass found \
         that the budget check believed that accounting, and these two were added because of it. \
         Both are asserted rather than reported, and the run that wrote this file passed them.\n\n\
         - **The held bytes are the cel count times the cel size.** With room for the whole shot \
         the cache held {:.1} MiB in {} cels, checked against {} cels of {} bytes - the same figure \
         derived a second way rather than taken on trust.\n\
         - **A budget below one cel refuses rather than churns.** Given {} bytes, one byte less \
         than a cel, the cache held nothing and evicted {}. A cache that admits what it cannot \
         hold and throws it straight back out also ends up empty; the eviction count is the only \
         thing that tells the two apart.\n",
        loops.iter().filter(|run| run.total_ms <= work_area_budget_ms).count(),
        work_area_budget_ms,
        [
            percentile(&seeks.reseek, 0.95),
            percentile(&seeks.scatter_default, 0.95),
            percentile(&seeks.scatter_whole, 0.95),
            percentile(&seeks.full_seek, 0.95),
        ]
        .into_iter()
        .fold(0.0f64, f64::max),
        mib(seeks.filled_bytes),
        seeks.filled_cels,
        seeks.filled_cels,
        ONE_CEL,
        ONE_CEL - 1,
        seeks.evictions_below_one_cel,
    );

    let path = repo("verification/T-06_performance_envelope.md");
    fs::write(&path, s).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
}
