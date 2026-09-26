//! B-48, D-105: how much memory the viewer may use, and whether more of it plays faster.
//!
//! `b48_memory` writes `verification/B-48_memory_table.md`: the machine's memory as Windows
//! reports it, what Automatic gives the viewer and the card, the largest a Custom setting may
//! give, that a cache made smaller lets go of what no longer fits, and that a bigger cache draws
//! the same bytes (document 27: a cache never defines correctness).
//!
//! `b48_memory_timing`, run deliberately in release, writes `verification/B-48_memory_timing_table.md`:
//! the reference shot with B-47's three Blooms played twice on the CPU and on the card, with D-40's
//! gibibyte and with Automatic.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use anime_compositor::cache::{self, CelCache};
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};
use serde_json::json;

mod common;
use common::repo;

const GIB: usize = 1024 * 1024 * 1024;

/// The reference shot with B-47's three Blooms, the shot whose CPU loop played again no faster
/// than the first time (`verification/B-47_gpu_bloom_timing_table.md`).
fn bloomed_reference() -> (Project, PathBuf, Id) {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: serde_json::Value = serde_json::from_str(&text).expect("the reference shot is JSON");
    let bloom = |id: &str, threshold: f64, radius: f64, intensity: f64, streaks: &str, length: f64, angle: f64| {
        json!({"instance_id": id, "type_id": "core.bloom", "enabled": true,
               "parameters": {"threshold": threshold, "radius": radius, "intensity": intensity,
                              "streaks": streaks, "length": length, "angle": angle}})
    };
    let layers = &mut j["compositions"][0]["layers"];
    layers[0]["effects"] = json!([bloom("b47-a", 80.0, 20.0, 1.0, "none", 0.0, 0.0)]);
    layers[1]["effects"] = json!([
        {"instance_id": "b47-b", "type_id": "core.gaussian_blur", "enabled": true, "parameters": {"sigma_px": 4.0}},
        bloom("b47-c", 60.0, 10.0, 1.0, "star", 60.0, 15.0)
    ]);
    layers[2]["effects"] = json!([bloom("b47-d", 30.0, 6.0, 2.0, "cross", 25.0, 30.0)]);
    let loaded = persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the bloomed reference shot: {}", d.message));
    (loaded.document.project().clone(), repo("Fixtures/reference_shot"), Id::new("comp-reference-shot"))
}

fn gb(bytes: impl Into<u64>) -> String {
    format!("{:.1} GB", bytes.into() as f64 / 1e9)
}

#[test]
fn b48_memory() {
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let installed = cache::installed_memory();
    rows.push(("Windows says how much memory the machine has".into(), "yes".into(),
        installed.map_or("no".into(), |_| "yes".into())));
    let m = installed.unwrap_or(0) as usize;

    let viewer = CelCache::viewer();
    rows.push(("the cache every earlier table measured is unchanged: D-40's 1 GiB, 448 MiB of it for effects".into(),
        "576 MiB + 448 MiB".into(),
        format!("{} + {}", cache::budget_label(viewer.budget()), cache::budget_label(viewer.effect_budget()))));
    rows.push(("Automatic gives the viewer a quarter of the machine's memory, never less than 1 GiB".into(),
        gb((m / 4).max(GIB) as u64), gb(cache::automatic_budget() as u64)));
    rows.push(("Custom may give it at most three quarters".into(),
        gb((m / 4 * 3).max(GIB) as u64), gb(cache::largest_budget() as u64)));
    let sized = CelCache::viewer_sized(cache::automatic_budget());
    rows.push(("Automatic splits as the 1 GiB does: seven sixteenths for effects".into(),
        "7/16".into(),
        if sized.effect_budget() == cache::automatic_budget() / 16 * 7 && sized.budget() + sized.effect_budget() == cache::automatic_budget() {
            "7/16".into()
        } else {
            format!("{} of {}", sized.effect_budget(), sized.budget() + sized.effect_budget())
        }));

    // A cache that has played part of the shot, then made smaller, holds no more than it may.
    let (project, root, comp) = bloomed_reference();
    let draft = PreviewQuality::Draft;
    let mut big = CelCache::viewer_sized(cache::automatic_budget());
    let mut first = Vec::new();
    for frame in [0, 100, 239] {
        let mut log = FrameLog::new(3);
        first.push(preview::preview_frame_cached(&project, &comp, frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut big).expect("frame").to_srgb8_straight());
    }
    for frame in 0..48 {
        let mut log = FrameLog::new(3);
        drop(preview::preview_frame_cached(&project, &comp, frame, &root, draft, DEFAULT_TILE_SIZE, &mut log, &mut big).expect("frame"));
    }
    let before = big.held_bytes() + big.effect_held_bytes();
    big.resize(64 * 1024 * 1024);
    rows.push((format!("made 64 MiB after holding {}, it lets go of what does not fit", gb(before as u64)),
        "at most 64 MiB, split 36 + 28".into(),
        if big.held_bytes() <= big.budget() && big.effect_held_bytes() <= big.effect_budget() && big.budget() == 36 << 20 && big.effect_budget() == 28 << 20 {
            "at most 64 MiB, split 36 + 28".into()
        } else {
            format!("{} of {}, {} of {}", big.held_bytes(), big.budget(), big.effect_held_bytes(), big.effect_budget())
        }));

    // Document 27: the same frames, asked again from a big cache and from D-40's, are the same bytes.
    let mut small = CelCache::viewer();
    let mut big = CelCache::viewer_sized(cache::automatic_budget());
    let mut same = true;
    for (frame, cold) in [0, 100, 239].into_iter().zip(&first) {
        for cache in [&mut small, &mut big] {
            for _ in 0..2 {
                let mut log = FrameLog::new(3);
                let warm = preview::preview_frame_cached(&project, &comp, frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, cache).expect("frame").to_srgb8_straight();
                same &= &warm == cold;
            }
        }
    }
    rows.push(("frames 0, 100 and 239 at Full, twice each from each cache, are the same bytes as the first time".into(),
        "identical".into(), if same { "identical".into() } else { "different".into() }));

    let card = match Gpu::new() {
        Ok(gpu) => {
            let memory = gpu.memory().unwrap_or(0) as usize;
            rows.push(("Automatic gives the card half its own memory, as before".into(),
                gb((memory / 2) as u64), gb(gpu.automatic_budget() as u64)));
            rows.push(("and it opens with that".into(), gb(gpu.automatic_budget() as u64), gb(gpu.budget as u64)));
            rows.push(("Custom may give the card at most 85% of its memory".into(),
                gb((memory / 20 * 17) as u64), gb(gpu.largest_budget() as u64)));
            format!("{}", gpu.about())
        }
        Err(why) => format!("none ({why}); the card's rows are not run"),
    };

    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut s = format!(
        "# B-48: how much memory the viewer may use\n\n\
         Written by `cargo test --test b48_memory`. Machine memory: {}. Card: {card}.\n\n\
         **{passed} of {} checks pass.**\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        installed.map_or("not reported".into(), gb),
        rows.len(),
    );
    for (check, expected, actual) in &rows {
        let _ = writeln!(s, "| {check} | {expected} | {actual} | {} |", if expected == actual { "pass" } else { "FAIL" });
    }
    fs::write(repo("verification/B-48_memory_table.md"), s).expect("write the B-48 table");
    assert_eq!(passed, rows.len(), "B-48: {passed} of {} checks pass", rows.len());
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

#[test]
#[ignore = "B-48: a measurement, run deliberately with --release --ignored"]
fn b48_memory_timing() {
    let (project, root, comp) = bloomed_reference();
    let automatic = cache::automatic_budget();
    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "# B-48: frame times with D-40's 1 GiB and with Automatic, on the CPU and the card\n\n\
         Written by `tests/b48_memory.rs` (`cargo test --release --test b48_memory -- \
         --ignored`).\n\n\
         - Machine memory: {}; Automatic gives the viewer {}\n- Card: {}; it may hold {} of drawings\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n\
         The reference shot with B-47's three Blooms, every frame asked for as the viewer asks, \
         whole: reading the drawings, the effects, drawing, and the eight-bit picture. On the CPU \
         the three blooms run inside planning; on the card the card runs them, and the Gaussian \
         Blur before one of them stays on the CPU. Both keep what they read and what the CPU's \
         effects make in the viewer's cache, whose size is the Memory column. Each row starts with \
         empty caches and plays the shot twice: the first loop fills them, the second is what \
         playing it again costs. Medians over all 240 frames, in ms, and what the viewer's cache \
         held at the end.\n\n\
         | Quality | Memory | Drawn on | First loop | Again | Held at the end |\n|---|---|---|---:|---:|---:|\n",
        cache::installed_memory().map_or("not reported".into(), gb),
        gb(automatic as u64),
        gpu.about(),
        gb(gpu.budget as u64),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    for quality in [PreviewQuality::Draft, PreviewQuality::Full] {
        for (label, total) in [("1 GiB (before)", GIB), ("Automatic", automatic)] {
            for on_card in [false, true] {
                let mut cache = CelCache::viewer_sized(total);
                gpu.forget();
                let mut row = format!("| {} | {label} | {} |", quality.label(), if on_card { "GPU" } else { "CPU" });
                for _ in 0..2 {
                    let mut times = Vec::new();
                    for frame in 0..240 {
                        let mut log = FrameLog::new(3);
                        let t = Instant::now();
                        if on_card {
                            drop(preview::preview_frame_srgb8(&project, &comp, frame, &root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).expect("GPU frame"));
                        } else {
                            drop(preview::preview_frame_cached(&project, &comp, frame, &root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache).expect("CPU frame").to_srgb8_straight());
                        }
                        times.push(t.elapsed().as_secs_f64() * 1000.0);
                    }
                    let _ = write!(row, " {:.1} |", median(times));
                }
                let _ = writeln!(s, "{row} {} |", gb((cache.held_bytes() + cache.effect_held_bytes()) as u64));
            }
        }
    }
    fs::write(repo("verification/B-48_memory_timing_table.md"), s).expect("write the timing table");
}
