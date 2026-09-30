//! B-164 (D-235): big Gaussian Blurs, Glows and Blooms worked small on the graphics card, and
//! the check that the viewer's frame stays within 1 level of the CPU's exact one.
//!
//! D-235's safety rule: a blur of sigma s may be worked by shrinking the picture f times (f = 8,
//! 4 or 2), blurring the small picture, and enlarging it back; f is halved, down to 1 (exact),
//! until the small sigma is at least 6. D-221: the shrink is a tent-weighted mean rather than the
//! block's plain one, so the small sigma is sqrt(s^2 - (2f^2 + 1) / 12 - f^2 / 6) / f. The rule
//! is written out again here rather than taken from the card's code, so the check does not trust it.
//!
//! Each row is the reference shot at frame 100 with one effect on its first three layers, at one
//! size, at Full and at Draft, compared as the page receives it, eight-bit straight sRGB. A row
//! passes when the card drew it, both sides said the same, no pixel is more than 1 level off,
//! and the card worked small exactly the blurs the rule says (the rule's blurs times the layers
//! left to the card). Export never takes this shortcut: only the viewer's card does.
//!
//! Writes `verification/B-164_downsampled_blurs_table.md`.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::png_out;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::OutputDepth;
use serde_json::json;

mod common;
use common::repo;

/// D-107's tolerance, in levels of 255.
const LIMIT: u8 = 1;
const SIZES: [f64; 4] = [20.0, 50.0, 100.0, 200.0];
const FRAME: i32 = 100;

/// D-235's rule: the largest factor up to 8 that keeps the small sigma at 6 or more.
fn factor(sigma: f64) -> usize {
    let small = |f: f64| (sigma * sigma - (2.0 * f * f + 1.0) / 12.0 - f * f / 6.0).max(0.0).sqrt() / f;
    let mut f = 8;
    while f > 1 && small(f as f64) < 6.0 {
        f /= 2;
    }
    f
}

/// The effect, its blurs' sigmas as the CPU works them, and its JSON.
fn effect(kind: &str, size: f64) -> (Vec<f64>, serde_json::Value) {
    match kind {
        "Gaussian Blur" => (vec![size], json!({"type_id": "core.gaussian_blur", "parameters": {"sigma_px": size}})),
        "Glow" => (
            vec![size / 3.0],
            json!({"type_id": "core.glow", "parameters": {"based_on": "bright", "threshold": 60, "colors": [], "tolerance": 0,
                   "radius": size, "intensity": 1, "operation": "add", "tint": ""}}),
        ),
        _ => (
            [1.0, 0.5, 0.25, 0.125].iter().map(|s| size / 3.0 * s).collect(),
            json!({"type_id": "core.bloom", "parameters": {"threshold": 80, "radius": size, "intensity": 1.0,
                   "streaks": "none", "length": 0.0, "angle": 0.0}}),
        ),
    }
}

fn shot(e: &serde_json::Value) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: serde_json::Value = serde_json::from_str(&text).expect("the reference shot is JSON");
    for (i, layer) in j["compositions"][0]["layers"].as_array_mut().expect("layers").iter_mut().take(3).enumerate() {
        let mut e = e.clone();
        e["instance_id"] = json!(format!("b164-{i}"));
        e["enabled"] = json!(true);
        layer["effects"] = json!([e]);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot with {e}: {}", d.message)).document.project().clone()
}

fn distance(a: &[u8], b: &[u8]) -> (u8, usize) {
    assert_eq!(a.len(), b.len(), "the two pictures are different sizes");
    let mut largest = 0;
    let mut pixels = 0;
    for (p, q) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
        let d = p.iter().zip(q).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0);
        largest = largest.max(d);
        pixels += (d > 0) as usize;
    }
    (largest, pixels)
}

#[test]
fn b164_downsampled_blurs() {
    let out = repo("verification/B-164_downsampled_blurs_table.md");
    let mut gpu = match Gpu::new() {
        Ok(gpu) => gpu,
        Err(why) => {
            fs::write(&out, format!("# B-164: big blurs worked small on the card\n\n**NOT RUN.** No usable card: {why}\n\nNo check in this table was run, so none of them passes.\n"))
                .expect("write the B-164 table");
            return;
        }
    };
    let (root, comp): (PathBuf, Id) = (repo("Fixtures/reference_shot"), Id::new("comp-reference-shot"));
    let (mut rows, mut checks, mut passed) = (String::new(), 0, 0);
    let mut worst: Option<((u8, usize), String, Vec<u8>, Vec<u8>, usize, usize)> = None;
    for kind in ["Gaussian Blur", "Glow", "Bloom"] {
        for size in SIZES {
            let (sigmas, e) = effect(kind, size);
            let project = shot(&e);
            for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
                // Draft works a layer's effects on a smaller drawing, its distances divided.
                let factors: Vec<usize> = sigmas.iter().map(|&s| factor(s / quality.divisor() as f64)).collect();
                let small = factors.iter().filter(|&&f| f > 1).count();
                let mut cache = CelCache::viewer();
                let mut log = FrameLog::new(3);
                let c = preview::preview_frame_cached(&project, &comp, FRAME, &root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache)
                    .unwrap_or_else(|d| panic!("{kind} {size} on the CPU: {}", d.message));
                let said = |log: FrameLog| {
                    let mut ids: Vec<&str> = log.finish().iter().map(|d| d.id.as_str()).collect();
                    ids.sort();
                    ids.dedup();
                    ids.join(", ")
                };
                let said_cpu = said(log);
                let (w, h) = (c.width(), c.height());
                let c = c.to_srgb8_straight();
                let mut plan_log = FrameLog::new(3);
                let on_card = compose::plan_frame_for_card(&project, &comp, FRAME, &root, quality, &mut plan_log, &mut CelCache::viewer())
                    .expect("plan the frame")
                    .layers
                    .iter()
                    .filter(|l| !l.on_card.is_empty())
                    .count();
                // Nothing kept from the last row: Draft works a layer's effects at the same size
                // as Full, so the card could reuse Full's blur and work nothing small.
                gpu.forget();
                let before = gpu.shrunk();
                let mut log = FrameLog::new(3);
                let (g, ..) = preview::preview_frame_srgb8(&project, &comp, FRAME, &root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu)
                    .unwrap_or_else(|d| panic!("{kind} {size} on the GPU: {}", d.message));
                let worked = gpu.shrunk() - before;
                let expected = (small * on_card) as u64;
                let said_gpu = said(log);
                let on_cpu = said_gpu.contains(DiagnosticId::GpuPreviewOnCpu.as_str());
                let d = distance(&c, &g);
                let pass = !on_cpu && said_cpu == said_gpu && d.0 <= LIMIT && worked == expected;
                checks += 1;
                passed += pass as usize;
                let factors: Vec<String> = factors.iter().map(|f| f.to_string()).collect();
                let _ = writeln!(
                    rows,
                    "| {kind} {size}, {} | {} | {on_card} | {expected} | {worked} | {} | {} | {} | {} |",
                    quality.label(),
                    factors.join("/"),
                    d.0,
                    d.1,
                    match (said_gpu.is_empty(), said_cpu == said_gpu) {
                        (true, true) => "none".to_string(),
                        (false, true) => format!("{said_gpu}, on both"),
                        (_, false) => format!("CPU: {said_cpu}; GPU: {said_gpu}"),
                    },
                    match (on_cpu, pass) {
                        (true, _) => "FAIL: the CPU drew it",
                        (false, true) => "PASS",
                        (false, false) => "FAIL",
                    }
                );
                if worked > 0 && worst.as_ref().is_none_or(|w| d > w.0) {
                    worst = Some((d, format!("{kind} {size}, {}", quality.label()), c, g, w, h));
                }
            }
        }
    }
    let pictures = repo("verification/B-164 pictures");
    fs::create_dir_all(&pictures).expect("make the pictures folder");
    let ((largest, count), worst_case, c, g, w, h) = worst.expect("something was worked small");
    // Each channel's difference times 64, so 1 level shows as a quarter grey.
    let diff: Vec<u8> = c.iter().zip(&g).enumerate().map(|(i, (x, y))| if i % 4 == 3 { 255 } else { x.abs_diff(*y).saturating_mul(64) }).collect();
    for (name, bytes) in [("cpu.png", &c), ("gpu.png", &g), ("difference x64.png", &diff)] {
        png_out::write_rgba(&pictures.join(name), w, h, OutputDepth::Eight, &[], bytes).expect("write a picture");
    }
    let table = format!(
        "# B-164: big blurs worked small on the card (D-235)\n\n\
         Card: {}.\n\n\
         The reference shot at frame {FRAME}, with the effect on its first three layers. \"Factors\" is D-235's rule for each of \
         the effect's blurs (Bloom has four): 1 is the exact blur, 2, 4 or 8 the block the picture is shrunk by. \"Small, \
         expected\" is the blurs the rule works small times the layers left to the card; \"small, worked\" is what the card \
         did. The difference is in levels of 255 against the CPU's exact frame, as the page receives it; the limit is {LIMIT}. \
         Export never takes this shortcut.\n\n\
         **{passed} of {checks} checks pass.**\n\n\
         Of the rows worked small, the furthest from the CPU is \"{worst_case}\": largest difference {largest} of 255, pixels \
         differing: {count}. Its pictures are in `verification/B-164 pictures/`: `cpu.png`, `gpu.png`, and \
         `difference x64.png`, each channel's difference times 64, so black where the two agree and a dark grey where they \
         are 1 level apart.\n\n\
         | Case | Factors | Layers on the card | Small, expected | Small, worked | Largest difference | Pixels differing | Diagnostics | Result |\n\
         |---|---|---:|---:|---:|---:|---:|---|---|\n{rows}",
        gpu.about()
    );
    fs::write(&out, table).expect("write the B-164 table");
    assert_eq!(passed, checks, "B-164: {} of {checks} checks fail; see {}", checks - passed, out.display());
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// Run deliberately, in release: how long the card takes to draw frame 100 with each big blur,
/// the card first made to let go of what it kept, so the blur is worked every time.
#[test]
#[ignore]
fn b164_downsampled_blurs_timing() {
    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- Build: {}\n\n| Shot | Quality | GPU |\n|---|---|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let (root, comp) = (repo("Fixtures/reference_shot"), Id::new("comp-reference-shot"));
    for (kind, size) in [("none", 0.0), ("Gaussian Blur", 20.0), ("Gaussian Blur", 50.0), ("Gaussian Blur", 100.0), ("Gaussian Blur", 200.0), ("Glow", 100.0), ("Glow", 200.0), ("Bloom", 100.0), ("Bloom", 200.0)] {
        let project = if kind == "none" {
            let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
            persist::load_str(&text).expect("the reference shot").document.project().clone()
        } else {
            shot(&effect(kind, size).1)
        };
        for quality in [PreviewQuality::Draft, PreviewQuality::Full] {
            let mut cache = CelCache::viewer();
            let mut times = Vec::new();
            for pass in 0..16 {
                gpu.forget();
                let mut log = FrameLog::new(3);
                let t = std::time::Instant::now();
                drop(preview::preview_frame_srgb8(&project, &comp, FRAME, &root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).expect("GPU frame"));
                if pass > 0 {
                    times.push(t.elapsed().as_secs_f64() * 1000.0);
                }
            }
            let _ = writeln!(s, "| {kind} {size} | {} | {:.1} |", quality.label(), median(times));
        }
    }
    fs::write(repo("verification/B-164_timing_raw.md"), s).expect("write the timing table");
}
