//! B-76: the second batch of ten on the graphics card (D-133), and the check that each draws as
//! the CPU does: Distance Gradation, Light Rays, Exposure Flicker, Vignette, Turbulent Displace,
//! Fractal Noise, Gradient Map, Color Balance, Offset and Light Wrap.
//!
//! What is compared is what the page receives, eight-bit straight sRGB: the CPU's frame through
//! `preview_frame_cached`, and the card's through `preview_frame_srgb8`, whose plan leaves a
//! layer's last effect of the nine to the card, and whose drawing does a layer's Light Wraps
//! wherever they are. The CPU stays the authority (ADR-006, D-100).
//!
//! Each row also says how many layers had one in fact left to the card, since a row where none
//! was compares the CPU with itself. One that changes nothing is not left, nor is one whose
//! settings are invalid (the CPU reports and skips it); on those rows any difference is the
//! card's layering, which D-100 holds to 1 level.
//!
//! Writes `verification/B-76_gpu_fx_table.md` and, for the worst frame, three pictures in
//! `verification/B-76 pictures/`. `b76_gpu_fx_timing`, run deliberately in release, writes
//! `verification/B-76_gpu_fx_timing_table.md`.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::png_out;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::render;
use anime_compositor::OutputDepth;
use serde_json::json;

mod common;
use common::repo;

/// D-133's tolerance, in levels of 255.
const LIMIT: u8 = 1;

struct Shot {
    effect: &'static str,
    name: String,
    project: Project,
    root: PathBuf,
    comp: Id,
    frames: Vec<i32>,
}

/// Each effect's name, its fixture folder and file prefix, its type and the settings it is
/// given in the reference shot.
fn effects() -> Vec<(&'static str, &'static str, &'static str, &'static str, serde_json::Value)> {
    vec![
        ("Distance Gradation", "distance_gradation", "fx_distgrad", "core.distance_gradation", json!({"color": "#3040a0", "width": 20, "opacity": 70, "invert": "off", "blend": "multiply"})),
        ("Light Rays", "light_rays", "fx_rays", "core.light_rays", json!({"center": [50, 40], "length": 40, "threshold": 60, "intensity": 1.5, "color": "#ffe0b0"})),
        ("Exposure Flicker", "exposure_flicker", "fx_flicker", "core.exposure_flicker", json!({"amount": 0.5, "hold": 3, "seed": 4})),
        ("Vignette", "vignette", "fx_vignette", "core.vignette", json!({"amount": 60, "color": "#201030", "size": 90, "roundness": 50, "softness": 60, "center": [45, 55]})),
        ("Turbulent Displace", "turbulent_displace", "fx_turb", "core.turbulent_displace", json!({"amount": 12, "size": 50, "complexity": 3, "evolution": 30, "speed": 10, "seed": 5, "edges": "transparent"})),
        ("Fractal Noise", "fractal_noise", "fx_fractal", "core.fractal_noise", json!({"size": 80, "complexity": 5, "contrast": 130, "brightness": 5, "evolution": 0, "speed": 15, "seed": 3, "dark_color": "#102030", "light_color": "#ffe8c0", "opacity": 60, "blend": "screen"})),
        ("Gradient Map", "gradient_map", "fx_gradmap", "core.gradient_map", json!({"shadow_color": "#1a0a40", "midtone_color": "#c05060", "highlight_color": "#fff0c0", "midpoint": 40, "amount": 80})),
        ("Color Balance", "color_balance", "fx_balance", "core.color_balance", json!({"shadows": [-20, 10, 30], "midtones": [15, -5, -10], "highlights": [10, 5, -25]})),
        ("Offset", "offset", "fx_offset", "core.offset", json!({"shift": [37.5, -21.25]})),
        ("Light Wrap", "light_wrap", "fx_wrap", "core.light_wrap", json!({"width": 15, "intensity": 150, "blend": "screen"})),
    ]
}

/// The reference shot with `type_id` on its first three layers, the second after a Drop Shadow
/// that grows it first, so the effect runs from a corner that is not the buffer's.
fn reference(effect: &'static str, type_id: &str, parameters: &serde_json::Value) -> Shot {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: serde_json::Value = serde_json::from_str(&text).expect("the reference shot is JSON");
    let fx = |id: &str| json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": parameters});
    let shadow = json!({"instance_id": "b76-s", "type_id": "core.drop_shadow", "enabled": true, "parameters": {"color": "#000000", "opacity": 60, "direction": 200, "distance": 6, "softness": 4}});
    let layers = &mut j["compositions"][0]["layers"];
    layers[0]["effects"] = json!([fx("b76-a")]);
    layers[1]["effects"] = json!([shadow, fx("b76-b")]);
    layers[2]["effects"] = json!([fx("b76-c")]);
    let loaded = persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot with {effect}: {}", d.message));
    Shot {
        effect,
        name: format!("the reference shot with {effect}"),
        project: loaded.document.project().clone(),
        root: repo("Fixtures/reference_shot"),
        comp: Id::new("comp-reference-shot"),
        frames: vec![0, 100, 239],
    }
}

/// Every fixture of the ten, each at every frame it has, the ones the CPU reports as invalid
/// included.
fn fixtures() -> Vec<Shot> {
    let mut shots = Vec::new();
    for (effect, folder, prefix, ..) in effects() {
        let mut files: Vec<String> = fs::read_dir(repo(&format!("Fixtures/{folder}")))
            .expect("read a fixture folder")
            .map(|e| e.expect("a fixture file").file_name().to_string_lossy().into_owned())
            .filter(|n| n.starts_with(prefix) && n.ends_with(".json"))
            .collect();
        files.sort();
        for file in files {
            let loaded = persist::load(&repo(&format!("Fixtures/{folder}/{file}"))).unwrap_or_else(|d| panic!("{file}: {}", d.message));
            let project = loaded.document.project().clone();
            let comp = &project.compositions[0];
            let (id, frames) = (comp.id.clone(), (comp.start_frame..comp.start_frame + comp.duration_frames as i32).collect());
            let name = file.trim_end_matches(".json").to_string();
            shots.push(Shot { effect, name, project, root: repo(&format!("Fixtures/{folder}")), comp: id, frames });
        }
    }
    shots
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

/// How many layers of the card's plan have one of the nine left for the card, or a Light Wrap
/// that runs, which the card draws wherever it is; and whether it has an adjustment layer, which
/// has the CPU draw the whole frame (B-44).
fn left(shot: &Shot, frame: i32, quality: PreviewQuality) -> (usize, bool) {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(&shot.project, &shot.comp, frame, &shot.root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    let n = plan
        .layers
        .iter()
        .filter(|l| l.on_card.iter().any(|c| matches!(c, render::OnCard::Fx(_))) || l.wrap.iter().any(|i| i.enabled && i.effect.is_valid()))
        .count();
    (n, plan.layers.iter().any(|l| l.adjust.is_some()))
}

#[test]
fn b76_gpu_fx() {
    let out = repo("verification/B-76_gpu_fx_table.md");
    let mut gpu = match Gpu::new() {
        Ok(gpu) => gpu,
        Err(why) => {
            fs::write(&out, format!("# B-76: the second batch of ten on the GPU\n\n**NOT RUN.** No usable card: {why}\n\nNo check in this table was run, so none of them passes.\n"))
                .expect("write the B-76 table");
            return;
        }
    };
    let (mut rows, mut checks, mut passed) = (String::new(), 0, 0);
    // Each effect's checks, passes, frames left to the card and largest difference.
    let mut by_effect: BTreeMap<&str, (usize, usize, usize, u8)> = BTreeMap::new();
    let mut worst: Option<((u8, usize), String, Vec<u8>, Vec<u8>, usize, usize)> = None;
    let mut shots = fixtures();
    let references: Vec<Shot> = effects().iter().map(|(effect, _, _, type_id, p)| reference(effect, type_id, p)).collect();
    shots.extend(references);
    for shot in &shots {
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for &frame in &shot.frames {
                let mut cache = CelCache::viewer();
                let mut log = FrameLog::new(3);
                let c = preview::preview_frame_cached(&shot.project, &shot.comp, frame, &shot.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache)
                    .unwrap_or_else(|d| panic!("{} frame {frame} on the CPU: {}", shot.name, d.message));
                let (w, h) = (c.width(), c.height());
                let said = |log: FrameLog| {
                    let mut ids: Vec<&str> = log.finish().iter().map(|d| d.id.as_str()).collect();
                    ids.sort();
                    ids.dedup();
                    ids.join(", ")
                };
                let said_cpu = said(log);
                let c = c.to_srgb8_straight();
                let mut log = FrameLog::new(3);
                let (g, ..) = preview::preview_frame_srgb8(&shot.project, &shot.comp, frame, &shot.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu)
                    .unwrap_or_else(|d| panic!("{} frame {frame} on the GPU: {}", shot.name, d.message));
                let said_gpu = said(log);
                let on_cpu = said_gpu.contains(DiagnosticId::GpuPreviewOnCpu.as_str());
                let (n, adjusted) = left(shot, frame, quality);
                let d = distance(&c, &g);
                // A frame with an adjustment layer is the CPU's by B-44's rule, and must then be
                // the CPU's picture exactly, with the CPU's warnings and the card's one message.
                let pass = if adjusted {
                    let expected = if said_cpu.is_empty() {
                        DiagnosticId::GpuPreviewOnCpu.as_str().to_string()
                    } else {
                        let mut ids: Vec<&str> = said_cpu.split(", ").chain([DiagnosticId::GpuPreviewOnCpu.as_str()]).collect();
                        ids.sort();
                        ids.join(", ")
                    };
                    on_cpu && d.0 == 0 && said_gpu == expected
                } else {
                    !on_cpu && said_cpu == said_gpu && d.0 <= LIMIT
                };
                checks += 1;
                passed += pass as usize;
                let e = by_effect.entry(shot.effect).or_default();
                e.0 += 1;
                e.1 += pass as usize;
                e.2 += (n > 0) as usize;
                e.3 = e.3.max(d.0);
                let case = format!("{} frame {frame}, {}", shot.name, quality.label());
                let _ = writeln!(
                    rows,
                    "| {case} | {n} | {} | {} | {} | {} |",
                    d.0,
                    d.1,
                    match (said_gpu.is_empty(), said_cpu == said_gpu) {
                        (true, true) => "none".to_string(),
                        (false, true) => format!("{said_gpu}, on both"),
                        (_, false) => format!("CPU: {said_cpu}; GPU: {said_gpu}"),
                    },
                    match (adjusted, on_cpu, pass) {
                        (true, _, true) => "PASS: an adjustment layer, so the CPU drew it (B-44)",
                        (true, _, false) => "FAIL",
                        (false, true, _) => "FAIL: the CPU drew it",
                        (false, false, true) => "PASS",
                        (false, false, false) => "FAIL",
                    }
                );
                if worst.as_ref().is_none_or(|w| d > w.0) {
                    worst = Some((d, case, c, g, w, h));
                }
            }
        }
    }

    // The CPU drawing a plan made for the card, as it does when the card refuses a frame, draws
    // the same frame as a plan made for the CPU, byte for byte.
    for reference in shots.iter().filter(|s| s.name.starts_with("the reference shot")) {
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            let mut log = FrameLog::new(3);
            let tile = match quality {
                PreviewQuality::Full => DEFAULT_TILE_SIZE,
                PreviewQuality::Draft => compose::DRAFT_TILE_SIZE,
            };
            let plan = |card: bool, log: &mut FrameLog| {
                let make = if card { compose::plan_frame_for_card } else { compose::plan_frame_at };
                let p = make(&reference.project, &reference.comp, 100, &reference.root, quality, log, &mut CelCache::viewer()).expect("plan");
                preview::scale_plan(p, quality)
            };
            let a = render::render(&plan(false, &mut log), tile);
            let b = render::render(&plan(true, &mut log), tile);
            let same = a.data() == b.data();
            checks += 1;
            passed += same as usize;
            let _ = writeln!(
                rows,
                "| {} frame 100, {}: the plan made for the card, drawn by the CPU | — | — | {} | — | {} |",
                reference.name,
                quality.label(),
                if same { "byte-identical".to_string() } else { "the frame differs".to_string() },
                if same { "PASS" } else { "FAIL" }
            );
        }
    }

    let pictures = repo("verification/B-76 pictures");
    fs::create_dir_all(&pictures).expect("make the pictures folder");
    let ((largest, count), worst_case, c, g, w, h) = worst.expect("something was compared");
    let mut diff = vec![0u8; c.len()];
    for p in (0..w * h).filter(|&p| c[p * 4..p * 4 + 4] != g[p * 4..p * 4 + 4]) {
        let (x, y) = ((p % w) as isize, (p / w) as isize);
        for yy in (y - 3).max(0)..(y + 4).min(h as isize) {
            for xx in (x - 3).max(0)..(x + 4).min(w as isize) {
                let i = (yy as usize * w + xx as usize) * 4;
                diff[i..i + 3].fill(255);
            }
        }
    }
    for px in diff.chunks_exact_mut(4) {
        px[3] = 255;
    }
    for (name, bytes) in [("cpu.png", &c), ("gpu.png", &g), ("difference.png", &diff)] {
        png_out::write_rgba(&pictures.join(name), w, h, OutputDepth::Eight, &[], bytes).expect("write a picture");
    }

    let mut summary = String::new();
    for (effect, ..) in effects() {
        let (n, p, on_card, most) = by_effect[effect];
        let _ = writeln!(summary, "| {effect} | {n} | {on_card} | {most} | {p} of {n} |");
    }
    let s = format!(
        "# B-76: the second batch of ten on the GPU against the CPU\n\n\
         Written by `tests/b76_gpu_fx.rs`. The card: {}.\n\n\
         Each row compares the eight-bit picture the page receives, drawn by the CPU and by the \
         GPU, with the layer's last effect of the nine done on the card and its Light Wraps \
         wherever they are. **The rule: no channel of any pixel more than {LIMIT} level of 255 \
         apart** (D-133). An effect that changes nothing or whose settings are invalid is not \
         left to the card; on those rows any difference is the card's layering, held to the same \
         1 level by D-100. On every row both paths must give the same warnings, and the card must \
         draw the frame itself, except a frame with an adjustment layer, which the CPU draws by \
         B-44's rule: that one must be the CPU's picture exactly, the card's message \
         `GPU_PREVIEW_ON_CPU` its only extra warning.\n\n\
         **{passed} of {checks} checks pass.**\n\n\
         The worst comparison is \"{worst_case}\": largest difference {largest} of 255, pixels differing: {count}. \
         Its pictures are in `verification/B-76 pictures/`: `cpu.png`, `gpu.png`, and \
         `difference.png`, black where the two agree and a white 7 by 7 square around every pixel \
         where they do not.\n\n\
         ## Each effect\n\n\
         Every fixture frame of the effect and the reference shot with it, at Full and Draft.\n\n\
         | Effect | Frames compared | Frames with it on the card | Largest difference (of 255) | Pass |\n|---|---:|---:|---:|---|\n{summary}\n\
         ## Every frame\n\n\
         | Case | Effects left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |\n|---|---:|---:|---:|---|---|\n{rows}",
        gpu.about(),
    );
    fs::write(&out, s).expect("write the B-76 table");
    assert_eq!(passed, checks, "B-76: {passed} of {checks} checks pass");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

#[test]
#[ignore = "B-76: a measurement, run deliberately with --release --ignored"]
fn b76_gpu_fx_timing() {
    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "# B-76: frame times with each of the second ten, CPU and GPU\n\n\
         Written by `tests/b76_gpu_fx.rs` (`cargo test --release --test b76_gpu_fx -- \
         --ignored`).\n\n\
         - Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n\
         Each shot is the reference shot with one of the ten on three layers, as in the B-76 \
         table. Every fourth frame, 60 in all, is asked for as the viewer asks, whole: planning, \
         the effects, drawing, and the eight-bit picture. On the CPU the effects run inside \
         planning; on the GPU the card runs the last effect of each layer. Each path starts with \
         empty caches and plays the frames twice: the first loop fills the caches, the second is \
         what playing it again costs. Medians in ms.\n\n\
         | Effect | Quality | CPU, first loop | CPU, again | GPU, first loop | GPU, again |\n|---|---|---:|---:|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    for (effect, _, _, type_id, parameters) in effects() {
        let shot = reference(effect, type_id, &parameters);
        for quality in [PreviewQuality::Draft, PreviewQuality::Full] {
            let tile = DEFAULT_TILE_SIZE;
            let mut row = format!("| {effect} | {} |", quality.label());
            for on_card in [false, true] {
                let mut cache = CelCache::viewer();
                gpu.forget();
                for _ in 0..2 {
                    let mut times = Vec::new();
                    for frame in (0..240).step_by(4) {
                        let mut log = FrameLog::new(3);
                        let t = Instant::now();
                        if on_card {
                            drop(preview::preview_frame_srgb8(&shot.project, &shot.comp, frame, &shot.root, quality, tile, &mut log, &mut cache, &mut gpu).expect("GPU frame"));
                        } else {
                            drop(preview::preview_frame_cached(&shot.project, &shot.comp, frame, &shot.root, quality, tile, &mut log, &mut cache).expect("CPU frame").to_srgb8_straight());
                        }
                        times.push(t.elapsed().as_secs_f64() * 1000.0);
                    }
                    let _ = write!(row, " {:.1} |", median(times));
                }
            }
            let _ = writeln!(s, "{row}");
        }
    }
    fs::write(repo("verification/B-76_gpu_fx_timing_table.md"), s).expect("write the timing table");
}
