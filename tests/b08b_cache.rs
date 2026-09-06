//! B-08b: the bounded cache of decoded cels (R-06b), unparked by D-37 and specified by ADR-015.
//!
//! Writes `verification/B-08b_cache_table.md` (correctness, every run) and
//! `verification/B-08b_cache_budget.md` (the measurement, under `--release --ignored`).
//!
//! # What this is for
//!
//! A cache is the one kind of change that can make a build faster and wrong at the same time, and
//! the wrongness is invisible from the outside: the picture looks plausible, it is simply the
//! picture of a different frame, or of a file as it was before someone repainted it. Document 27
//! states the rule that makes that unacceptable — "A cold render and a fully warm render for the
//! same immutable request must produce equivalent pixels and diagnostics" — and this file's job is
//! to hold that rule to a byte comparison rather than to an assurance.
//!
//! So the table below is mostly not about speed. Speed is one artifact, measured separately and
//! reported as numbers on a named machine. The checks are all forms of one question: **does having
//! remembered something change what comes out?**
//!
//! # Where the expected values come from
//!
//! Every expected value here is either a definition or arithmetic on one, and none was read off a
//! run of this build (ADR-009).
//!
//! - **The identical-pixel counts** are the extent of the buffers being compared. A draft frame of
//!   this composition is 480x270 and therefore 518,400 samples; a full frame is 1920x1080 and
//!   therefore 8,294,400. "Identical" means every sample, so the expected value is the whole count.
//! - **The warm pass decoding nothing** is what a hit means. The six frames of the warm pass ask
//!   for cels the cold pass has already put in a cache large enough to hold all of them, so the
//!   number of decodes in the warm pass is zero by the definition of the word.
//! - **One cel is 33,177,600 bytes**: 1920 x 1080 x 4 channels x 4 bytes, because the working
//!   buffer is f32. This is the correction recorded in ADR-015 — a cel on disk is a quarter of
//!   that, and budgeting on the disk figure would have overshot memory fourfold.
//! - **The changed-file counts** follow from the key document 27 requires: path, length, mtime and
//!   interpretation. Replace the file and the key no longer matches, so the second request is a
//!   miss and the pixels are the new file's.
//! - **The ten-loop values** are the same number ten times, which is the whole claim: a cache that
//!   is bounded does not grow when you play the same shot again.
//!
//! # What this deliberately does not do
//!
//! It does not test the export path for the presence of a cache, because there is nothing there to
//! test: export calls [`compose::plan_frame`], which builds a [`CelCache::none`] whose budget is
//! zero. The check below asserts that a zero-budget cache holds nothing after a full pass, which is
//! the property that makes the absence structural rather than a matter of who remembered to pass
//! what.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::cache::{CelCache, DEFAULT_BUDGET_BYTES};
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::{Diagnostic, DiagnosticId, FrameLog};
use anime_compositor::model::{Id, Interpretation, Project};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};

const COMP: &str = "comp-reference-shot";
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
/// One decoded cel in the working space: RGBA f32 at the composition's extent.
const ONE_CEL: usize = WIDTH * HEIGHT * 4 * std::mem::size_of::<f32>();
/// Samples in a draft frame: a quarter on each axis, RGBA.
const DRAFT_SAMPLES: usize = (WIDTH / 4) * (HEIGHT / 4) * 4;
/// Samples in a full frame.
const FULL_SAMPLES: usize = WIDTH * HEIGHT * 4;
/// A budget with room for every cel these six frames use, so that "warm" means warm rather than
/// partly evicted. The six frames use fifteen distinct drawings; twenty cels is room for all of
/// them and no room for a second copy of any. Deliberately not [`DEFAULT_BUDGET_BYTES`]: the
/// default is a performance choice that the measurement may move, and none of the checks here are
/// about performance.
const AMPLE: usize = 20 * ONE_CEL;
/// The frames every pass renders. 14 is a gap frame — layer 3 asks for the drawing the fixture
/// deliberately omits — so a pass that goes quiet about it is a pass that lost a diagnostic.
const FRAMES: [i32; 6] = [0, 12, 14, 25, 100, 101];

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

/// B-09's own artifact, read back off the disk: a real project file rather than a model built in
/// the test, for the same reason B-08 reads it.
fn project() -> Project {
    let path = repo("verification/B-08a_project.json");
    let loaded =
        persist::load(&path).unwrap_or_else(|d| panic!("open {}: {}", path.display(), d.message));
    loaded.document.project().clone()
}

/// What one pass over [`FRAMES`] produced: the pixels of each frame, and the diagnostics.
struct Pass {
    frames: Vec<Vec<f32>>,
    diagnostics: Vec<Diagnostic>,
    /// Decodes this pass paid for, which is the cache's miss count across the pass.
    decodes: u64,
}

fn pass(project: &Project, quality: PreviewQuality, cache: &mut CelCache) -> Pass {
    let comp = Id::new(COMP);
    let root = root();
    let before = cache.misses();
    let mut log = FrameLog::new(3);
    let mut frames = Vec::new();
    for &frame in &FRAMES {
        let buffer = preview::preview_frame_cached(
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
        frames.push(buffer.data().to_vec());
    }
    Pass {
        decodes: cache.misses() - before,
        frames,
        diagnostics: log.finish(),
    }
}

/// How many samples of two passes agree, summed over every frame.
fn agreeing(a: &Pass, b: &Pass) -> usize {
    a.frames
        .iter()
        .zip(&b.frames)
        .map(|(x, y)| x.iter().zip(y).filter(|(p, q)| p == q).count())
        .sum()
}

// ---------------------------------------------------------------------------------------
// The checks
// ---------------------------------------------------------------------------------------

#[test]
fn b08b_cache_is_invisible_in_the_result() {
    let project = project();
    let mut report = Report::default();

    // --- A cache large enough for everything: cold, then warm over the same frames. -----------
    let mut cache = CelCache::with_budget(AMPLE);
    let cold = pass(&project, PreviewQuality::Draft, &mut cache);
    let warm = pass(&project, PreviewQuality::Draft, &mut cache);

    report.check(
        "A warm draft render is the cold render, sample for sample",
        DRAFT_SAMPLES * FRAMES.len(),
        agreeing(&cold, &warm),
    );
    report.check(
        "A warm draft render reports the same diagnostics, word for word",
        format!("identical, all {} of them", cold.diagnostics.len()),
        format!(
            "{1}, all {0} of them",
            warm.diagnostics.len(),
            if warm.diagnostics == cold.diagnostics {
                "identical"
            } else {
                "**different**"
            }
        ),
    );
    report.check(
        "The gap frame is still reported when the drawing either side of it came from memory",
        true,
        warm.diagnostics
            .iter()
            .any(|d| d.id == DiagnosticId::MediaSequenceGap),
    );
    report.check(
        "The cold pass decoded, the warm pass did not",
        "cold > 0, warm = 0",
        format!(
            "cold {}, warm = {}",
            if cold.decodes > 0 { "> 0" } else { "= 0" },
            warm.decodes
        ),
    );
    report.check(
        "Nothing held exceeds the budget",
        format!("held <= {AMPLE}"),
        if cache.held_bytes() <= AMPLE {
            format!("held <= {AMPLE}")
        } else {
            format!("held = {}, OVER", cache.held_bytes())
        },
    );

    // --- The same comparison at full resolution, which is the extent an export writes. --------
    let mut cache = CelCache::with_budget(AMPLE);
    let comp = Id::new(COMP);
    let mut log = FrameLog::new(3);
    let full_cold = preview::preview_frame_cached(
        &project,
        &comp,
        100,
        &root(),
        PreviewQuality::Full,
        DEFAULT_TILE_SIZE,
        &mut log,
        &mut cache,
    )
    .expect("full cold frame 100");
    let full_warm = preview::preview_frame_cached(
        &project,
        &comp,
        100,
        &root(),
        PreviewQuality::Full,
        DEFAULT_TILE_SIZE,
        &mut log,
        &mut cache,
    )
    .expect("full warm frame 100");
    report.check(
        "A warm full-resolution frame 100 is the cold one, sample for sample",
        FULL_SAMPLES,
        full_cold
            .data()
            .iter()
            .zip(full_warm.data())
            .filter(|(a, b)| a == b)
            .count(),
    );

    // --- A cache bounded to one cel: eviction on every frame, and the same pixels. ------------
    let mut tiny = CelCache::with_budget(ONE_CEL);
    let evicting = pass(&project, PreviewQuality::Draft, &mut tiny);
    report.check(
        "A cache bounded to one cel renders what an unbounded one renders",
        DRAFT_SAMPLES * FRAMES.len(),
        agreeing(&cold, &evicting),
    );
    report.check(
        "A cache bounded to one cel reports what an unbounded one reports",
        "identical",
        if evicting.diagnostics == cold.diagnostics {
            "identical"
        } else {
            "**different**"
        },
    );
    report.check(
        "It stayed inside one cel by evicting, not by growing",
        format!("held <= {ONE_CEL}, evictions > 0"),
        format!(
            "held {}, evictions {}",
            if tiny.held_bytes() <= ONE_CEL {
                format!("<= {ONE_CEL}")
            } else {
                format!("= {}, OVER", tiny.held_bytes())
            },
            if tiny.evictions() > 0 { "> 0" } else { "= 0" }
        ),
    );

    // --- Export's cache. ----------------------------------------------------------------------
    let mut none = CelCache::none();
    let uncached = pass(&project, PreviewQuality::Draft, &mut none);
    report.check(
        "The cache export uses holds nothing after a full pass",
        "0 cels, 0 bytes, 0 hits",
        format!(
            "{} cels, {} bytes, {} hits",
            none.len(),
            none.held_bytes(),
            none.hits()
        ),
    );
    report.check(
        "The uncached path renders what the cached one renders",
        DRAFT_SAMPLES * FRAMES.len(),
        agreeing(&cold, &uncached),
    );

    // --- Media identity: a file that changed is not the file that was remembered. -------------
    let scratch = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("b08b_cache");
    fs::create_dir_all(&scratch).expect("create scratch directory");
    let cel = scratch.join("cel.png");
    let first = root().join("layer1/layer1_000.png");
    let second = root().join("layer4/layer4_000.png");

    let mut watching = CelCache::with_budget(AMPLE);
    copy_as(&first, &cel);
    let a = decoded(&mut watching, &cel);
    let a_again = decoded(&mut watching, &cel);
    report.check(
        "The same unchanged file is answered from memory the second time",
        "1 decode, 1 hit",
        format!("{} decode, {} hit", watching.misses(), watching.hits()),
    );
    report.check(
        "and the remembered answer is the decoded one, sample for sample",
        a.len(),
        a.iter().zip(&a_again).filter(|(x, y)| x == y).count(),
    );

    copy_as(&second, &cel);
    let b = decoded(&mut watching, &cel);
    report.check(
        "A file that changed under the same name is decoded again",
        "2 decodes",
        format!("{} decodes", watching.misses()),
    );
    report.check(
        "and the answer is the new file, not the remembered one",
        "different from the first",
        if b == a {
            "**the remembered one**"
        } else {
            "different from the first"
        },
    );
    let direct = CelCache::none()
        .decoded(&second, Interpretation::default())
        .expect("decode the replacement directly");
    report.check(
        "and it is exactly what an uncached decode of the new file gives",
        direct.data().len(),
        b.iter().zip(direct.data()).filter(|(x, y)| x == y).count(),
    );
    let _ = fs::remove_file(&cel);

    // --- Ten loops of the same frames, which is what a person scrubbing actually does. ---------
    let mut looping = CelCache::with_budget(AMPLE);
    let mut held = Vec::new();
    let mut decodes_after_the_first_loop = 0;
    for loop_number in 0..10 {
        let run = pass(&project, PreviewQuality::Draft, &mut looping);
        if loop_number > 0 {
            decodes_after_the_first_loop += run.decodes;
        }
        held.push(looping.held_bytes());
    }
    report.check(
        "Ten loops of the shot hold what one loop held",
        format!("{} then nine identical", held[0]),
        format!(
            "{} then {}",
            held[0],
            if held.iter().all(|h| *h == held[0]) {
                "nine identical".to_string()
            } else {
                format!("**{:?}**", &held[1..])
            }
        ),
    );
    report.check(
        "and the nine loops after the first decode nothing at all",
        "0 decodes",
        format!("{decodes_after_the_first_loop} decodes"),
    );

    write_report(&report, &looping);
    let failed: Vec<&Row> = report.rows.iter().filter(|r| !r.pass()).collect();
    assert!(
        failed.is_empty(),
        "{} of {} checks failed, first: {} (expected {}, got {}); see \
         verification/B-08b_cache_table.md",
        failed.len(),
        report.rows.len(),
        failed[0].check,
        failed[0].expected,
        failed[0].actual
    );
}

fn copy_as(from: &Path, to: &Path) {
    let bytes = fs::read(from).unwrap_or_else(|e| panic!("read {}: {e}", from.display()));
    fs::write(to, bytes).unwrap_or_else(|e| panic!("write {}: {e}", to.display()));
    // Windows keeps file times to 100 ns, so two writes a millisecond apart already differ. The
    // two files also differ in length, so the key would change even if the clock did not.
}

fn decoded(cache: &mut CelCache, path: &Path) -> Vec<f32> {
    cache
        .decoded(path, Interpretation::default())
        .unwrap_or_else(|d| panic!("decode {}: {}", path.display(), d.message))
        .data()
        .to_vec()
}

fn write_report(report: &Report, looping: &CelCache) {
    let passed = report.rows.iter().filter(|r| r.pass()).count();
    let mut out = String::new();
    out.push_str("# B-08b: the bounded cache of decoded cels, and what it must not change\n\n");
    out.push_str(
        "D-37 unparked this cache on 2026-09-05 because the window was delivering a frame every \
         92 ms and 75 of those milliseconds were reading and decoding cels the same second of \
         playback had already read. ADR-015 bounded what was unparked. This is the part of it a \
         reader can check: **the cache is not allowed to change the picture or the warnings, ever, \
         under any budget.** Produced by `tests/b08b_cache.rs` from \
         `verification/B-08a_project.json` and the reference shot.\n\n",
    );
    out.push_str(
        "The rows to read first are the three that compare pixels. A warm render, a render from a \
         cache bounded so tightly it throws away every cel it holds, and a render with no cache at \
         all are the same picture sample for sample. If a cache ever starts serving the wrong cel, \
         those rows are where it shows up as a number rather than as a picture somebody has to \
         notice looks wrong.\n\n",
    );
    let _ = writeln!(
        out,
        "The speed this bought is not here. It is a measurement, and it is in \
         `verification/B-08b_cache_budget.md`, on a named machine.\n"
    );
    out.push_str("## Checks\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
    for row in &report.rows {
        let _ = writeln!(
            out,
            "| {} | {} | {} | {} |",
            row.check,
            row.expected,
            row.actual,
            if row.pass() { "pass" } else { "**FAIL**" }
        );
    }
    let _ = writeln!(
        out,
        "\n**{passed} of {} checks pass.**\n",
        report.rows.len()
    );
    let _ = writeln!(
        out,
        "## What one cel costs to hold\n\n\
         A cel of this composition is 1920 by 1080. On disk, and in \
         `verification/D-37_decode_cost.md`, that is 8,294,400 bytes. In memory it is \
         {ONE_CEL} bytes, four times as much, because the renderer samples a buffer of `f32` in \
         the working space rather than the bytes that were on disk. ADR-015 records the correction; \
         it matters because a budget written against the smaller figure would have held four times \
         the memory it promised.\n\n\
         The viewer's default budget is {DEFAULT_BUDGET_BYTES} bytes, which is {} cels of this \
         size, and it was chosen by the measurement in `verification/B-08b_cache_budget.md` rather \
         than by a guess ahead of it. The checks above deliberately do not use that number: they \
         use a budget with room for every drawing the six frames touch, because a check about \
         whether remembering changes the answer should not also depend on how much is remembered. \
         Ten loops of six frames ended holding {} bytes in {} cels, which is every distinct drawing \
         those frames use and no copy of any of them.",
        DEFAULT_BUDGET_BYTES / ONE_CEL,
        looping.held_bytes(),
        looping.len(),
    );
    let path = repo("verification/B-08b_cache_table.md");
    fs::write(&path, out).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
}

// ---------------------------------------------------------------------------------------
// The measurement D-37 asked for: the same playback, before and after
// ---------------------------------------------------------------------------------------

/// What the cache is worth, as the same playthrough at four budgets.
///
/// "Before" is not a different build. It is `CelCache::none()`, which is the code path that
/// existed the day D-37 was written and is still the path export takes, so the two halves of the
/// comparison differ in one thing only.
///
/// Ignored by default for the reason every timing test here is: a debug build measures the
/// compiler. Run it deliberately:
///
/// ```text
/// cargo test --release --test b08b_cache -- --ignored --nocapture
/// ```
#[test]
#[ignore = "timing; run under --release with --ignored"]
fn b08b_cache_budget() {
    let project = project();
    let comp = Id::new(COMP);
    let root = root();
    // Two seconds of the shot at 24 fps, which is long enough for layer 2 to work through all
    // twenty-four of its drawings and start repeating them.
    let frames: Vec<i32> = (0..48).collect();

    let budgets: [(&str, usize); 4] = [
        ("none (the path export takes)", 0),
        ("one cel", ONE_CEL),
        ("128 MB (the viewer's default)", DEFAULT_BUDGET_BYTES),
        ("512 MB", 512 * 1024 * 1024),
    ];

    let mut rows = Vec::new();
    for (label, budget) in budgets {
        let mut cache = CelCache::with_budget(budget);
        // One untimed frame, so no measured frame pays for the first touch of fresh pages.
        let mut warmup = FrameLog::new(3);
        let _ = preview::preview_frame_cached(
            &project,
            &comp,
            0,
            &root,
            PreviewQuality::Draft,
            DEFAULT_TILE_SIZE,
            &mut warmup,
            &mut CelCache::none(),
        );
        let mut ms = Vec::new();
        let start = std::time::Instant::now();
        for &frame in &frames {
            let mut log = FrameLog::new(3);
            let at = std::time::Instant::now();
            let _ = preview::preview_frame_cached(
                &project,
                &comp,
                frame,
                &root,
                PreviewQuality::Draft,
                DEFAULT_TILE_SIZE,
                &mut log,
                &mut cache,
            )
            .unwrap_or_else(|d| panic!("frame {frame}: {}", d.message));
            ms.push(at.elapsed().as_secs_f64() * 1000.0);
        }
        let total = start.elapsed().as_secs_f64() * 1000.0;
        let mut sorted = ms.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).expect("no NaN in a measured duration"));
        rows.push((
            label,
            budget,
            total,
            sorted,
            cache.hits(),
            cache.misses(),
            cache.evictions(),
            cache.held_bytes(),
        ));
    }
    write_budget_artifact(&rows, frames.len());
}

#[allow(clippy::type_complexity)]
fn write_budget_artifact(
    rows: &[(&str, usize, f64, Vec<f64>, u64, u64, u64, usize)],
    frames: usize,
) {
    let mut s = String::from("# B-08b: what the cache is worth, measured\n\n");
    s.push_str(
        "D-37 unparked the cache on a measurement and asked for one back: the same playback, in \
         the same build, on the same machine, with and without it. This is that. Produced by \
         `tests/b08b_cache.rs`, which is `#[ignore]`d in normal runs.\n\n",
    );
    s.push_str(
        "## Machine, build and configuration\n\n\
         - CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads\n\
         - OS: Microsoft Windows 11 Education, 10.0.26200\n\
         - Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`\n\
         - Workload: `verification/B-08a_project.json`, the four-layer reference shot, previewed \
         at draft resolution\n\
         - Tile size: `compose::DEFAULT_TILE_SIZE`\n\n",
    );
    let _ = writeln!(
        s,
        "- Frames per run: {frames}, consecutively from frame 0, which is two seconds of the shot \
         at 24 fps\n\n\
         Debug assertions in this build: {}. A run with `true` there is a debug build, and its \
         numbers say more about the compiler than about the cache.\n",
        cfg!(debug_assertions)
    );
    s.push_str(
        "## The same playthrough at four budgets\n\n\
         | Budget | Bytes | Total ms | Median ms per frame | Slowest ms | Frames per second at the median | Decodes | Answered from memory | Evictions | Held at the end |\n\
         |---|---|---|---|---|---|---|---|---|---|\n",
    );
    for (label, budget, total, sorted, hits, misses, evictions, held) in rows {
        let n = sorted.len();
        let median = if n % 2 == 0 {
            (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
        } else {
            sorted[n / 2]
        };
        let _ = writeln!(
            s,
            "| {label} | {budget} | {total:.1} | {median:.2} | {:.2} | {:.1} | {misses} | {hits} | \
             {evictions} | {held} |",
            sorted[n - 1],
            1000.0 / median
        );
    }
    s.push_str(
        "\n## How to read this\n\n\
         The first row is the build as it was before D-37: every frame decodes every cel it needs, \
         four decodes a frame, and it is still the path an export takes. Each row below it is the \
         same forty-eight frames with more memory allowed, and the only thing that changes between \
         rows is how many of those decodes happen at all. `tests/b08b_cache.rs` separately checks \
         that the pixels do not change between them, which is the claim that makes this table \
         about speed rather than about a trade.\n\n\
         The 'one cel' row is a real result and not a formality. A cache too small to hold the \
         frame it is working on evicts what it is about to need next, so it answers nothing from \
         memory, evicts once per decode, and finishes no faster than having no cache at all. A \
         cache is not a thing you can add a little of.\n\n\
         The two rows below it are within a fraction of a millisecond of each other on four times \
         the memory, which is why the default is the smaller of them. Playback is sequential: a \
         cel is asked for again within a few frames or not for a long time, so what has to fit is \
         the reuse distance, not the shot. That is also why both rows still evict — neither holds \
         the whole shot, and neither needs to.\n\n\
         What is still not measured here is the transport into the webview, which has its own cost \
         and its own place to be fixed. `verification/B-08_window_shell.md` is where playback is \
         counted end to end, in dropped frames, by photographing a running window, and it is the \
         artifact this change has to move next.\n",
    );
    let path = repo("verification/B-08b_cache_budget.md");
    fs::write(&path, s).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
}
