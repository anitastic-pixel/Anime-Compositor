//! P-01: where one preview frame actually goes, stage by stage.
//!
//! Writes `verification/P-01_frame_trace.md` under `--release --ignored`.
//!
//! # Why this exists
//!
//! `verification/T-06_declared_fixture.md` measures the fixture document 08 line 41 declares at
//! 264.17 ms a frame at the median of the tenth loop, against the 41.667 ms a 24 fps clock
//! allows. That is D-47, and D-47 is OPEN. Two research documents were written about it,
//! `Markdown/32_Performance_Architecture_Investigation.md` and
//! `Markdown/33_Hardware_Optimization_and_Future_Architecture_Research.md`, and both of them
//! reach the same wall: document 32 section 1.2 could account for about half of a frame and
//! attributed the rest to "everything else". Nobody had measured it.
//!
//! This file measures it. It changes no picture, proposes nothing and optimises nothing.
//! Document 15's P-01 entry says so in the strongest terms it has: **"Not in scope: any
//! optimisation, however obvious it looks while the timers go in."** Every later entry in that
//! section is ranked by the table this writes and not by either research document's estimates.
//!
//! # What is measured
//!
//! `src/perf.rs` holds one atomic nanosecond counter and one call count per named stage, off
//! unless a caller switches it on, which only this file does. The stages are disjoint by
//! construction — no stage is inside another — so the table can be summed and subtracted from
//! the frame time to leave a residual, and that residual is printed rather than hidden.
//!
//! The frame measured is the preview path end to end: `preview::preview_frame_cached`, then
//! `WorkingBuffer::to_srgb8_straight`, which is what `app/src/main.rs` line 351 does before the
//! bytes reach the page. It stops there, at a `Vec<u8>` in memory, for the same reason
//! `verification/B-08_preview_latency.md` stops there: the transport into the web view belongs
//! to the window, and a headless test cannot open one. The `wait for the viewer lock` row is in
//! the table for the same reason it is zero in it — there is no second thread here to wait for.
//! P-04 is the entry that measures that row, in a running window.
//!
//! # The three cache states, and the one this machine cannot establish
//!
//! Document 33's correction to document 32 is that T-06's "cold" loop was never cold: the
//! operating system's file cache held the drawings from the loop before. Three states are
//! therefore reported and they are not all separable:
//!
//! - **application cache cold, files first read by this process.** A [`CelCache::none`] pass over
//!   frames this process has not touched.
//! - **application cache cold, operating system file cache warm.** The same pass again, with a
//!   fresh [`CelCache::none`].
//! - **everything warm.** A cache large enough to hold the pass, on the second time through, with
//!   the eviction count reported so that "large enough" is checked rather than claimed.
//!
//! The first two differ only in what Windows holds, and **an unprivileged test process cannot
//! empty the standby list**, so if they come out equal that is the honest reading: this run did
//! not establish a cold file cache. Emptying it needs `SeProfileSingleProcessPrivilege` and an
//! elevated helper, which is a tool this repository does not have and P-01 is not the unit that
//! adds one. The artifact says which of the two happened rather than presenting the first row as
//! a cold-disk figure.
//!
//! # What is asserted
//!
//! Timings are reported and never asserted, for document 12's reason: a threshold that fails
//! because the machine was busy teaches nothing. Three things are asserted, because all three
//! are true of a correct run on any machine, and each one is a way this file could print a
//! confident wrong number:
//!
//! - every stage's total is at most the frame time it was measured inside, so no stage is
//!   secretly nested in another and double-counted;
//! - the residual is not negative, which is the same property checked from the other side;
//! - the warm pass evicted nothing, so the "everything warm" row is warm.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{Id, Project};
use anime_compositor::perf::{self, Stage};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};

mod common;
use common::{build_fixture, repo};

const COMP: &str = "comp-reference-shot";
/// How many frames each row is measured over. Twenty is enough for a median and a nearest-rank
/// p95 to mean something, and small enough that the whole file is under a minute even where a
/// frame costs a quarter of a second.
const SAMPLES: usize = 20;
/// A stride coprime with the 240 frames of the work area, so the sample is spread across the
/// shot rather than taken from one run of consecutive drawings. The same constant and the same
/// reason as `tests/t06_envelope.rs`.
const SCATTER: i32 = 97;
/// Room for every distinct drawing a twenty-frame pass of the ten-layer fixture touches. Checked
/// rather than assumed: the warm row asserts the cache evicted nothing.
const WARM_BUDGET_BYTES: usize = 6 * 1024 * 1024 * 1024;
/// 24 fps as a frame budget in milliseconds, which is what every number here is against.
const FRAME_BUDGET_MS: f64 = 1000.0 / 24.0;

/// The frames a row is measured over: twenty spread across the whole shot, deterministically.
fn sample_frames() -> Vec<i32> {
    (0..SAMPLES as i32).map(|i| (i * SCATTER) % 240).collect()
}

/// One workload: a name for the artifact, a project, and the root its drawings sit under.
struct Workload {
    name: &'static str,
    project: Project,
    root: PathBuf,
    layers: usize,
}

fn reference_shot() -> Workload {
    let path = repo("verification/B-08a_project.json");
    let loaded =
        persist::load(&path).unwrap_or_else(|d| panic!("open {}: {}", path.display(), d.message));
    Workload {
        name: "the reference shot",
        project: loaded.document.project().clone(),
        root: repo("Fixtures/reference_shot"),
        layers: 4,
    }
}

fn declared_fixture() -> Workload {
    let (project, root, _text) = build_fixture("p01");
    Workload {
        name: "the declared ten-layer fixture",
        project,
        root,
        layers: 10,
    }
}

/// One measured frame: the total, and every stage's nanoseconds within it.
struct Measured {
    total_ns: u64,
    stages: Vec<u64>,
}

/// Render one frame with the timers on and return what each stage cost.
///
/// The encode is inside the measured span because it is inside the frame: `app/src/main.rs`
/// converts the working buffer to sRGB bytes before the page ever sees it, and a frame time that
/// stopped at the working buffer would be a number no one waits for.
fn measure(
    workload: &Workload,
    frame: i32,
    quality: PreviewQuality,
    cache: &mut CelCache,
) -> Measured {
    let comp = Id::new(COMP);
    let mut log = FrameLog::new(3);
    perf::reset();
    perf::enable();
    let at = Instant::now();
    let buffer = preview::preview_frame_cached(
        &workload.project,
        &comp,
        frame,
        &workload.root,
        quality,
        DEFAULT_TILE_SIZE,
        &mut log,
        cache,
    )
    .unwrap_or_else(|d| panic!("{} frame {frame}: {}", workload.name, d.message));
    let pixels = buffer.to_srgb8_straight();
    let total_ns = at.elapsed().as_nanos() as u64;
    perf::disable();
    assert_eq!(
        pixels.len(),
        buffer.width() * buffer.height() * 4,
        "the encode produced a buffer of the wrong shape, so the frame it timed was not the \
         frame it rendered"
    );
    let stages: Vec<u64> = perf::snapshot().into_iter().map(|(_, ns, _)| ns).collect();
    let summed: u64 = stages.iter().sum();
    assert!(
        summed <= total_ns,
        "{} frame {frame}: the stages sum to {summed} ns inside a frame of {total_ns} ns, so a \
         stage is nested in another and is being counted twice",
        workload.name
    );
    Measured { total_ns, stages }
}

/// One row of the artifact: a workload, a quality, a cache state, and the frames behind it.
struct Row {
    cache_state: &'static str,
    frames: Vec<Measured>,
    evictions: u64,
    hits: u64,
    misses: u64,
}

impl Row {
    fn totals_ms(&self) -> Vec<f64> {
        sorted(self.frames.iter().map(|m| ns_ms(m.total_ns)).collect())
    }

    fn stage_ms(&self, index: usize) -> Vec<f64> {
        sorted(self.frames.iter().map(|m| ns_ms(m.stages[index])).collect())
    }

    /// Per frame: the time the stage table could not account for.
    fn residual_ms(&self) -> Vec<f64> {
        sorted(
            self.frames
                .iter()
                .map(|m| {
                    let summed: u64 = m.stages.iter().sum();
                    ns_ms(m.total_ns - summed)
                })
                .collect(),
        )
    }

    /// A stage's share of every frame in the row, summed both ways so that it is exact.
    ///
    /// Medians do not add up — the median of a sum is not the sum of the medians — so a column of
    /// p50s that summed to the frame time would be a coincidence rather than a result. This is
    /// the figure that is allowed to be read as a share of the frame.
    fn share(&self, index: usize) -> f64 {
        let part: u64 = self.frames.iter().map(|m| m.stages[index]).sum();
        let whole: u64 = self.frames.iter().map(|m| m.total_ns).sum();
        if whole == 0 {
            0.0
        } else {
            100.0 * part as f64 / whole as f64
        }
    }

    fn residual_share(&self) -> f64 {
        100.0 - (0..Stage::ALL.len()).map(|i| self.share(i)).sum::<f64>()
    }

    /// How many times a stage ran across the whole row, so a per-call cost can be derived.
    fn calls(&self) -> u64 {
        self.frames.len() as u64
    }
}

fn ns_ms(ns: u64) -> f64 {
    ns as f64 / 1_000_000.0
}

fn sorted(mut ms: Vec<f64>) -> Vec<f64> {
    ms.sort_by(|a, b| a.partial_cmp(b).expect("no NaN in a measured duration"));
    ms
}

/// Nearest rank, the same convention `tests/t06_envelope.rs` states and for the same reason: the
/// artifact has to say which one it used.
fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return f64::NAN;
    }
    sorted[(p * (sorted.len() - 1) as f64).round() as usize]
}

fn median(sorted: &[f64]) -> f64 {
    let n = sorted.len();
    if n == 0 {
        f64::NAN
    } else if n.is_multiple_of(2) {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    } else {
        sorted[n / 2]
    }
}

/// The three cache states of one workload at one quality.
fn three_states(workload: &Workload, quality: PreviewQuality) -> Vec<Row> {
    let frames = sample_frames();

    // 1. Application cache cold, and the first time this process reads these files.
    let mut cold = CelCache::none();
    let first: Vec<Measured> = frames
        .iter()
        .map(|&f| measure(workload, f, quality, &mut cold))
        .collect();

    // 2. Application cache cold again. Only the operating system's file cache has changed.
    let mut again = CelCache::none();
    let second: Vec<Measured> = frames
        .iter()
        .map(|&f| measure(workload, f, quality, &mut again))
        .collect();

    // 3. Everything warm: one untimed pass to fill the cache, then the measured one.
    let mut warm = CelCache::with_budget(WARM_BUDGET_BYTES);
    for &f in &frames {
        let _ = measure(workload, f, quality, &mut warm);
    }
    let before_hits = warm.hits();
    let third: Vec<Measured> = frames
        .iter()
        .map(|&f| measure(workload, f, quality, &mut warm))
        .collect();
    assert_eq!(
        warm.evictions(),
        0,
        "{} at {}: the warm pass evicted {} cels, so the row labelled warm was not warm",
        workload.name,
        quality.label(),
        warm.evictions()
    );

    vec![
        Row {
            cache_state: "application cache cold, files first read by this process",
            frames: first,
            evictions: 0,
            hits: 0,
            misses: 0,
        },
        Row {
            cache_state: "application cache cold, operating system file cache warm",
            frames: second,
            evictions: 0,
            hits: 0,
            misses: 0,
        },
        Row {
            cache_state: "everything warm",
            frames: third,
            evictions: warm.evictions(),
            hits: warm.hits() - before_hits,
            misses: warm.misses(),
        },
    ]
}

#[test]
#[ignore = "P-01: a measurement, run deliberately with --release --ignored"]
fn p01_frame_trace() {
    let workloads = [reference_shot(), declared_fixture()];
    let mut sections: Vec<(String, PreviewQuality, Vec<Row>)> = Vec::new();
    for workload in &workloads {
        for quality in [PreviewQuality::Draft, PreviewQuality::Full] {
            let rows = three_states(workload, quality);
            for row in &rows {
                let residual = row.residual_ms();
                assert!(
                    residual.iter().all(|&ms| ms >= 0.0),
                    "{} at {}, {}: a negative residual means a stage was counted twice",
                    workload.name,
                    quality.label(),
                    row.cache_state
                );
            }
            sections.push((
                format!("{} ({} layers)", workload.name, workload.layers),
                quality,
                rows,
            ));
        }
    }
    write_artifact(&sections);
}

fn write_artifact(sections: &[(String, PreviewQuality, Vec<Row>)]) {
    let mut s = String::from("# P-01: where the frame goes\n\n");
    s.push_str(
        "Document 15's P-01. A per-stage timer through one preview frame, on both fixtures, at \
         both preview qualities, in three cache states. Produced by `tests/p01_frame_trace.rs`, \
         which is `#[ignore]`d in normal runs and writes this file under `--release --ignored`.\n\n\
         **This unit optimises nothing.** It measures, and every later entry in document 15's \
         performance section is ranked by the tables below rather than by an estimate in \
         `Markdown/32_Performance_Architecture_Investigation.md` or \
         `Markdown/33_Hardware_Optimization_and_Future_Architecture_Research.md`. Where a number \
         here disagrees with one of those documents, this file is the measurement and they are \
         the research.\n\n",
    );

    s.push_str("## Machine, build and configuration\n\n");
    let _ = writeln!(
        s,
        "- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads\n\
         - OS: Microsoft Windows 11 Education, 10.0.26200\n\
         - Toolchain: cargo release profile, `opt-level = 3`\n\
         - Tile size: `compose::DEFAULT_TILE_SIZE`\n\
         - Threads rayon was given: {}\n\
         - Sample: {SAMPLES} frames a row, stepping by {SCATTER} through the 240-frame work area, \
         so the sample is spread across the shot rather than taken from one run of drawings\n\
         - Percentiles are by nearest rank on the sorted sample\n\
         - Frame budget at 24 fps: {FRAME_BUDGET_MS:.3} ms\n\n\
         Debug assertions in this build: {}. A run with `true` there is a debug build and its \
         numbers say more about the compiler than about the renderer.\n",
        std::thread::available_parallelism().map_or("unknown".to_string(), |n| n.to_string()),
        cfg!(debug_assertions)
    );

    s.push_str(
        "\n## What a frame means here, and where it stops\n\n\
         One frame is `preview::preview_frame_cached` followed by \
         `WorkingBuffer::to_srgb8_straight`, which is what `app/src/main.rs` does before any byte \
         reaches the page. It stops at a `Vec<u8>` in memory. The transport into the web view is \
         the window's and a headless test cannot open one, which is the same boundary \
         `verification/B-08_preview_latency.md` draws and for the same reason.\n\n\
         The `wait for the viewer lock` row is therefore **0.000 ms in every table below, and \
         that zero is a property of this harness rather than of the program.** In the running \
         window, `app/src/main.rs` takes the viewer mutex before planning and holds it through \
         the render and the encode, so a command arriving mid-frame waits for all of it. There is \
         no second thread here to do the waiting. **P-04 is the entry that measures that row**, in \
         a window, and this file is not evidence that the wait is small.\n\n",
    );

    s.push_str(
        "## The three cache states, and what this machine could not establish\n\n\
         | Row | Application cache | Operating system file cache |\n|---|---|---|\n\
         | application cache cold, files first read by this process | empty, `CelCache::none` | \
         whatever Windows happened to hold |\n\
         | application cache cold, operating system file cache warm | empty, `CelCache::none` | \
         warm: every file was read by the row above |\n\
         | everything warm | large enough to hold the pass, second time through | warm |\n\n\
         **The first two rows differ only in something this process cannot control.** Emptying \
         the Windows standby list needs `SeProfileSingleProcessPrivilege` and an elevated helper, \
         which this repository does not have and P-01 is not the unit that adds one. If the two \
         rows come out equal, the honest reading is that **this run did not establish a cold file \
         cache**, not that a cold disk is free. Document 33's correction to document 32 — that \
         T-06's \"cold\" loop was never cold — applies to this file as well, and it is stated \
         here rather than discovered later.\n\n",
    );

    for (name, quality, rows) in sections {
        let _ = writeln!(s, "## {name}, {} resolution\n", quality.label());
        for row in rows {
            let totals = row.totals_ms();
            let residual = row.residual_ms();
            let _ = writeln!(
                s,
                "### {} — {}, {}\n",
                name,
                quality.label(),
                row.cache_state
            );
            let _ = writeln!(
                s,
                "Frame time: p50 **{:.3} ms**, p95 **{:.3} ms**, over {} frames. That is \
                 {:.2}x the {FRAME_BUDGET_MS:.3} ms a 24 fps clock allows.\n",
                median(&totals),
                percentile(&totals, 0.95),
                row.calls(),
                median(&totals) / FRAME_BUDGET_MS
            );
            if row.hits > 0 || row.misses > 0 {
                let _ = writeln!(
                    s,
                    "Cache over the measured pass: {} hits, {} misses in the cache's whole life, \
                     {} evictions.\n",
                    row.hits, row.misses, row.evictions
                );
            }
            s.push_str("| Stage | p50 ms | p95 ms | share of the frame |\n|---|---|---|---|\n");
            for (index, stage) in Stage::ALL.iter().enumerate() {
                let ms = row.stage_ms(index);
                let _ = writeln!(
                    s,
                    "| {} | {:.3} | {:.3} | {:.1}% |",
                    stage.label(),
                    median(&ms),
                    percentile(&ms, 0.95),
                    row.share(index)
                );
            }
            let _ = writeln!(
                s,
                "| **unaccounted for** | {:.3} | {:.3} | {:.1}% |",
                median(&residual),
                percentile(&residual, 0.95),
                row.residual_share()
            );
            s.push('\n');
        }
    }

    s.push_str(
        "## What this table ranks\n\n\
         The three stages that cost the most in each row, by share, largest first. This is the \
         ordering document 15's P-01 entry says every later entry is ranked by, and it is \
         generated from the tables above rather than typed, so a re-run cannot leave it \
         stale.\n\n\
         | Workload | Quality | Cache state | First | Second | Third |\n|---|---|---|---|---|---|\n",
    );
    for (name, quality, rows) in sections {
        for row in rows {
            let mut ranked: Vec<(usize, f64)> =
                (0..Stage::ALL.len()).map(|i| (i, row.share(i))).collect();
            ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).expect("a share is never NaN"));
            let top: Vec<String> = ranked
                .iter()
                .take(3)
                .map(|&(i, share)| format!("{} — {share:.1}%", Stage::ALL[i].label()))
                .collect();
            let _ = writeln!(
                s,
                "| {} | {} | {} | {} | {} | {} |",
                name,
                quality.label(),
                row.cache_state,
                top[0],
                top[1],
                top[2]
            );
        }
    }
    s.push('\n');

    s.push_str(
        "## How to read these tables\n\n\
         **The share column is the one that adds up.** A median is not additive — the median of a \
         sum is not the sum of the medians — so the p50 column does not total the frame time and \
         is not meant to. The share column is every frame's nanoseconds in that stage over every \
         frame's nanoseconds altogether, which is exact, and it sums to a hundred with the \
         unaccounted-for row.\n\n\
         **Unaccounted for is real work, not measurement error.** It is the frame minus every \
         named stage: resolving each layer's exposure to a path, checking the file is where the \
         project says, evaluating the animated properties, allocating and dropping the buffers, \
         and the plan structure itself. A large figure there is a finding and names the next \
         thing to instrument; it is not a licence to guess.\n\n\
         **The stages are disjoint and the harness checks it.** Every row asserts that its stages \
         sum to no more than the frame they were measured inside, and that the residual is not \
         negative. A nested pair of timers would fail both.\n\n\
         ## What was not measured\n\n\
         - **The viewer lock.** Zero here by construction; P-04 measures it.\n\
         - **A cold operating system file cache.** Not establishable from an unprivileged process; \
         see the section above.\n\
         - **Export.** ADR-015 keeps the cel cache off the export path, and the export path is not \
         what D-47 is about.\n\
         - **Anything about a graphics card.** ADR-006 is the owner's and P-07 is the entry that \
         would put one number in front of it.\n\n\
         No expected value in `Fixtures/` or document 25 was read or written by this file.\n",
    );

    let path = repo("verification/P-01_frame_trace.md");
    write_lf(&path, &s);
}

/// LF endings whatever this machine's checkout does, because `.gitattributes` declares
/// `verification/**/*.md text eol=lf` and CI compares the committed copy against a fresh run.
fn write_lf(path: &Path, text: &str) {
    fs::write(path, text.replace("\r\n", "\n")).unwrap_or_else(|e| {
        panic!("write {}: {e}", path.display());
    });
}
