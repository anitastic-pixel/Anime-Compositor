//! B-114 (D-49, D-174): the performance envelope measured the way the window's viewer runs.
//!
//! Writes `verification/B-114_viewer_envelope.md` under `--release --ignored`.
//!
//! `tests/t06_envelope.rs` and `tests/b12b_declared_fixture.rs` measure with
//! `CelCache::with_budget(DEFAULT_BUDGET_BYTES)`: 1 GiB of drawings, no room for effect results,
//! composited on the processor. The window has not run that way since P-11 split the cache, D-105
//! made its size a quarter of the machine's memory, and B-44 moved compositing to the card. This
//! measures the same two shots as the window builds them:
//!
//! - the cache is `CelCache::viewer_sized`, at the Automatic setting's size on this machine and at
//!   1 GiB, the least Automatic ever gives, which is what a machine with 4 GB or less gets;
//! - a frame is drawn on the card through `preview::preview_frame_srgb8`, as the window does with
//!   **Draw on: Auto** and a usable card, and on the processor otherwise; the page says which.
//!
//! What it still does not do, as the older pages do not: the window's read-ahead, which decodes
//! the next frames on other threads during playback, so a playback figure here is a ceiling; and
//! the transport into the window, so every figure is seek-to-buffer.
//!
//! Timings are reported, never asserted. Asserted, as T-06 does: neither half of the cache holds
//! more than its budget, and the process's working set grows by no more than one cel from the
//! second loop to the tenth.
//!
//! ```text
//! cargo test --release --test b114_viewer_envelope -- --ignored --nocapture
//! ```

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use anime_compositor::cache::{automatic_budget, budget_label, installed_memory, CelCache, DEFAULT_BUDGET_BYTES};
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};

mod common;
use common::{build_fixture, peak_working_set, repo, working_set};

const COMP: &str = "comp-reference-shot";
/// One decoded cel at 1920 by 1080, RGBA f32.
const ONE_CEL: usize = 1920 * 1080 * 4 * 4;
const FRAMES: i32 = 240;
const LOOPS: usize = 10;
/// Coprime with 240, so the walk meets every frame once, far from in order, the same way every run.
const SCATTER: i32 = 97;
const FRAME_BUDGET_MS: f64 = 1000.0 / 24.0;
const MIB: f64 = 1024.0 * 1024.0;

fn frame_ms(project: &Project, root: &Path, frame: i32, cache: &mut CelCache, gpu: &mut Option<Gpu>) -> f64 {
    let comp = Id::new(COMP);
    let mut log = FrameLog::new(3);
    let at = Instant::now();
    let q = PreviewQuality::Draft;
    match gpu {
        Some(g) => drop(
            preview::preview_frame_srgb8(project, &comp, frame, root, q, DEFAULT_TILE_SIZE, &mut log, cache, g)
                .unwrap_or_else(|d| panic!("frame {frame} on the card: {}", d.message)),
        ),
        None => drop(
            preview::preview_frame_cached(project, &comp, frame, root, q, DEFAULT_TILE_SIZE, &mut log, cache)
                .unwrap_or_else(|d| panic!("frame {frame}: {}", d.message)),
        ),
    }
    at.elapsed().as_secs_f64() * 1000.0
}

fn sorted(mut v: Vec<f64>) -> Vec<f64> {
    v.sort_by(|a, b| a.partial_cmp(b).expect("no NaN in a duration"));
    v
}

/// Nearest rank, as T-06 does it.
fn pct(sorted: &[f64], p: f64) -> f64 {
    sorted[((p * sorted.len() as f64).ceil() as usize).clamp(1, sorted.len()) - 1]
}

struct Row {
    cold_total: f64,
    warm: Vec<f64>,
    decodes: u64,
    effects_reused: u64,
    seek: Vec<f64>,
    again: Vec<f64>,
    held_mib: f64,
    ws_second: usize,
    ws_tenth: usize,
}

fn measure(project: &Project, root: &Path, total: usize, gpu: &mut Option<Gpu>) -> Row {
    let mut cache = CelCache::viewer_sized(total);
    if let Some(g) = gpu {
        g.forget();
    }
    let _ = frame_ms(project, root, 0, &mut cache, gpu);
    let (mut loops, mut ws) = (Vec::new(), Vec::new());
    let (mut decodes, mut effects_reused) = (0, 0);
    for _ in 0..LOOPS {
        let (misses, effect_hits) = (cache.misses(), cache.effect_hits());
        loops.push(sorted((0..FRAMES).map(|f| frame_ms(project, root, f, &mut cache, gpu)).collect()));
        (decodes, effects_reused) = (cache.misses() - misses, cache.effect_hits() - effect_hits);
        ws.push(working_set());
        assert!(cache.held_bytes() <= cache.budget(), "drawings over budget: {} of {}", cache.held_bytes(), cache.budget());
        assert!(
            cache.effect_held_bytes() <= cache.effect_budget(),
            "effect results over budget: {} of {}",
            cache.effect_held_bytes(),
            cache.effect_budget()
        );
    }
    assert!(
        ws[LOOPS - 1] <= ws[1] + ONE_CEL,
        "the working set grew from {} to {} bytes between the second loop and the tenth",
        ws[1],
        ws[LOOPS - 1]
    );
    let (mut seek, mut again) = (Vec::new(), Vec::new());
    for i in 0..FRAMES {
        let f = (i * SCATTER) % FRAMES;
        seek.push(frame_ms(project, root, f, &mut cache, gpu));
        again.push(frame_ms(project, root, f, &mut cache, gpu));
    }
    Row {
        cold_total: loops[0].iter().sum(),
        warm: loops.pop().expect("ten loops"),
        decodes,
        effects_reused,
        seek: sorted(seek),
        again: sorted(again),
        held_mib: (cache.held_bytes() + cache.effect_held_bytes()) as f64 / MIB,
        ws_second: ws[1],
        ws_tenth: ws[LOOPS - 1],
    }
}

#[test]
#[ignore = "timing; run under --release with --ignored"]
fn b114_viewer_envelope() {
    let reference = persist::load(&repo("verification/B-08a_project.json"))
        .unwrap_or_else(|d| panic!("the reference shot: {}", d.message))
        .document
        .project()
        .clone();
    let (declared, declared_root, _) = build_fixture("b114");
    let shots: [(&str, Project, PathBuf); 2] = [
        ("The reference shot: four layers, no mattes, no effects", reference, repo("Fixtures/reference_shot")),
        ("Document 08's shot: ten layers, two mattes, three effects", declared, declared_root),
    ];
    let mut gpu = Gpu::new().ok();
    let drawn_on = gpu.as_ref().map_or("the processor: no usable card was found".to_string(), |g| format!("the card, {}", g.about()));
    let automatic = automatic_budget();
    let budgets = [
        (format!("{} (Automatic on this machine)", budget_label(automatic)), automatic),
        (format!("{} (the least Automatic gives)", budget_label(DEFAULT_BUDGET_BYTES)), DEFAULT_BUDGET_BYTES),
    ];

    let mut s = format!(
        "# B-114: the envelope, measured the way the viewer runs\n\n\
         Written by `tests/b114_viewer_envelope.rs` (`cargo test --release --test b114_viewer_envelope -- --ignored`), \
         for D-174. Timings are this machine's on the day; run it again and they will move a little.\n\n\
         ## Machine, build and configuration\n\n\
         - Processor: {}, {} threads\n\
         - Memory: {:.1} GB\n\
         - Frames drawn on {}\n\
         - Build: {}\n\
         - Preview: draft, frames 0 to 239, one frame at a time\n\
         - Cache: built as the window builds it, `CelCache::viewer_sized`: nine sixteenths for drawings, seven for effect results\n\n\
         The older pages, `verification/T-06_performance_envelope.md` and `verification/T-06_declared_fixture.md`, \
         measured a cache the window no longer builds: 1 GiB, drawings only, composited on the processor. They stay as \
         the record of that. This page is what the envelope now rests on.\n\n\
         ## What came back\n\n\
         Times in ms. A 24 fps clock allows {FRAME_BUDGET_MS:.1} ms a frame; a frame over it is one the viewer drops rather \
         than play slowly (D-32). **Cold** is the whole first time round the shot, reading every drawing from disk. **Warm** is the tenth time round the whole shot. **Seek** is a jump to a frame far from \
         the last, after those ten loops. **Again** is asking for the frame just shown.\n\n\
         | Shot | Cache | Cold, first loop in all | Warm median | Warm p95 | Warm frames over {FRAME_BUDGET_MS:.1} ms | Seek p95 | Again p95 | Drawings read from disk, tenth loop | Effect results reused, tenth loop | Cache holds (MiB) | Working set, loop 2 → 10 (MiB) |\n\
         |---|---|---|---|---|---|---|---|---|---|---|---|\n",
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        installed_memory().unwrap_or(0) as f64 / 1e9,
        drawn_on,
        if cfg!(debug_assertions) { "debug, and its numbers describe the compiler" } else { "release" },
    );
    for (shot, project, root) in &shots {
        for (label, total) in &budgets {
            let r = measure(project, root, *total, &mut gpu);
            let _ = writeln!(
                s,
                "| {shot} | {label} | {:.0} | {:.2} | {:.2} | {} of 240 | {:.2} | {:.2} | {} | {} | {:.0} | {:.0} → {:.0} |",
                r.cold_total,
                pct(&r.warm, 0.5),
                pct(&r.warm, 0.95),
                r.warm.iter().filter(|&&ms| ms > FRAME_BUDGET_MS).count(),
                pct(&r.seek, 0.95),
                pct(&r.again, 0.95),
                r.decodes,
                r.effects_reused,
                r.held_mib,
                r.ws_second as f64 / MIB,
                r.ws_tenth as f64 / MIB,
            );
        }
    }
    let _ = write!(
        s,
        "\nPeak working set of the whole run: {:.0} MiB. That is the largest cache above plus the program, not \
         something the window would add to it.\n\n\
         **Document 08's target** is a p95 seek at or below 100 ms and playback at 24 fps. Read them off the Seek p95 \
         and the frames-over column. Peak memory on the card is not measured.\n",
        peak_working_set() as f64 / MIB
    );
    fs::write(repo("verification/B-114_viewer_envelope.md"), s).expect("write the page");
}
