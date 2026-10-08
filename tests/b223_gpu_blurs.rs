//! B-223 (D-342): five blurs on the GPU, against the CPU.
//!
//! Fast Box Blur, Channel Blur, Compound Blur, Selective Color Blur and CC Vector Blur, each the
//! CPU's rule on the card: the same taps, edges, growth and order of sums. Compound Blur and CC
//! Vector Blur read another layer as a map, which the card is handed as a picture of its own.
//! Every fixture of the five, at every frame it has, and the reference shot with each, is drawn
//! by the CPU and by the card at Full and Draft. The rule: no channel of any pixel more than 1
//! level of 255 apart (ADR-006, D-100).
//!
//! Writes `verification/B-223_gpu_blurs_table.md` and `verification/B-223 pictures/`.

use std::collections::BTreeMap;
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
use anime_compositor::render;
use anime_compositor::OutputDepth;
use serde_json::{json, Value};

mod common;
use common::repo;

/// ADR-006 as D-100 has it, in levels of 255.
const LIMIT: u8 = 1;

struct Shot {
    effect: &'static str,
    name: String,
    project: Project,
    root: PathBuf,
    comp: Id,
    frames: Vec<i32>,
    reference: bool,
}

/// Each effect's name and type.
const FIVE: [(&str, &str); 5] = [
    ("Fast Box Blur", "core.fast_box_blur"),
    ("Channel Blur", "core.channel_blur"),
    ("Compound Blur", "core.compound_blur"),
    ("Selective Color Blur", "core.selective_color_blur"),
    ("CC Vector Blur", "core.vector_blur"),
];

/// The settings each effect is given in the reference shot, each way of working at least once.
/// Channel Blur has no fixtures, so it has the most. A map is the shot's fourth layer.
fn settings() -> Vec<(&'static str, Value)> {
    let fbb = |radius: f64, iterations: f64, edges: &str, dimensions: &str| {
        json!({"radius": radius, "iterations": iterations, "edges": edges, "dimensions": dimensions})
    };
    let cb = |s: [f64; 4], edges: &str, dimensions: &str| {
        json!({"red_blurriness": s[0], "green_blurriness": s[1], "blue_blurriness": s[2], "alpha_blurriness": s[3], "edges": edges, "dimensions": dimensions})
    };
    let vb = |kind: &str, amount: f64, angle: f64, ridge: f64, layer: &str, property: &str, softness: f64| {
        json!({"type": kind, "amount": amount, "angle_offset": angle, "ridge_smoothness": ridge, "layer": layer, "fit": "stretch", "property": property, "map_softness": softness})
    };
    let cbl = |layer: &str, fit: &str, max: f64, invert: &str, edges: &str| {
        json!({"layer": layer, "fit": fit, "max_blur": max, "invert": invert, "edges": edges})
    };
    vec![
        ("Fast Box Blur", fbb(6.0, 3.0, "transparent", "both")),
        ("Fast Box Blur", fbb(12.5, 1.0, "repeat", "horizontal")),
        ("Fast Box Blur", fbb(2.0, 4.0, "transparent", "vertical")),
        ("Channel Blur", cb([8.0, 2.0, 0.0, 4.0], "transparent", "both")),
        ("Channel Blur", cb([0.0, 5.0, 5.0, 5.0], "repeat", "horizontal")),
        ("Channel Blur", cb([3.0, 3.0, 10.0, 0.0], "transparent", "vertical")),
        ("Channel Blur", cb([6.0, 6.0, 6.0, 6.0], "transparent", "both")),
        ("Compound Blur", cbl("layer-4", "stretch", 30.0, "off", "transparent")),
        ("Compound Blur", cbl("layer-4", "center", 12.0, "on", "repeat")),
        ("Selective Color Blur", json!({"blur": 12, "colors": ["#1e1a24", "#f0d0b0"], "tolerance": 30})),
        ("Selective Color Blur", json!({"blur": 60, "colors": ["#d04030"], "tolerance": 40})),
        ("CC Vector Blur", vb("natural", 10.0, 0.0, 10.0, "", "lightness", 20.0)),
        ("CC Vector Blur", vb("constant", 6.0, 30.0, 0.0, "", "alpha", 4.0)),
        ("CC Vector Blur", vb("perpendicular", 8.0, 0.0, 0.0, "", "luminance", 10.0)),
        ("CC Vector Blur", vb("direction_center", 5.0, 0.0, 0.5, "", "hue", 0.0)),
        ("CC Vector Blur", vb("direction_fading", 7.0, 45.0, 0.25, "layer-4", "saturation", 6.0)),
        ("CC Vector Blur", vb("natural", 12.0, -20.0, 5.0, "layer-4", "red", 12.0)),
    ]
}

fn type_of(effect: &str) -> &'static str {
    FIVE.iter().find(|t| t.0 == effect).expect("one of the five").1
}

/// The reference shot with `parameters` on its first three layers, the second after a Drop
/// Shadow, as B-151 and B-222 have it.
fn reference(effect: &'static str, case: usize, parameters: &Value) -> Shot {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: Value = serde_json::from_str(&text).expect("the reference shot is JSON");
    let type_id = type_of(effect);
    let fx = |id: &str| json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": parameters});
    let shadow = json!({"instance_id": "b223-s", "type_id": "core.drop_shadow", "enabled": true, "parameters": {"color": "#000000", "opacity": 60, "direction": 200, "distance": 6, "softness": 4}});
    let layers = &mut j["compositions"][0]["layers"];
    layers[0]["effects"] = json!([fx("b223-a")]);
    layers[1]["effects"] = json!([shadow, fx("b223-b")]);
    layers[2]["effects"] = json!([fx("b223-c")]);
    let loaded = persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot with {effect}: {}", d.message));
    Shot {
        effect,
        name: format!("the reference shot with {effect} ({})", case + 1),
        project: loaded.document.project().clone(),
        root: repo("Fixtures/reference_shot"),
        comp: Id::new("comp-reference-shot"),
        frames: vec![0, 100, 239],
        reference: true,
    }
}

/// Every fixture naming one of the five, each at every frame it has. A file is the first
/// effect's it names; one that must not open (a malformed setting) has no frame to compare.
fn fixtures() -> Vec<Shot> {
    let mut files: Vec<PathBuf> = Vec::new();
    for folder in fs::read_dir(repo("Fixtures")).expect("read Fixtures").map(|e| e.expect("a folder").path()).filter(|p| p.is_dir()) {
        for f in fs::read_dir(&folder).expect("read a fixture folder").map(|e| e.expect("a file").path()) {
            let name = f.file_name().unwrap_or_default().to_string_lossy().into_owned();
            if name.ends_with(".json") && !name.starts_with("expected") && name != "exposure_bypass_word.json" {
                files.push(f);
            }
        }
    }
    files.sort();
    let mut shots = Vec::new();
    for file in files {
        let Ok(text) = fs::read_to_string(&file) else { continue };
        let Some(&(effect, _)) = FIVE.iter().find(|t| text.contains(&format!("\"{}\"", t.1))) else { continue };
        let Ok(loaded) = persist::load(&file) else { continue };
        let root = file.parent().expect("a folder").to_path_buf();
        let folder = root.file_name().unwrap_or_default().to_string_lossy().into_owned();
        let project = loaded.document.project().clone();
        let comp = &project.compositions[0];
        let (id, frames) = (comp.id.clone(), (comp.start_frame..comp.start_frame + comp.duration_frames as i32).collect());
        shots.push(Shot {
            effect,
            name: format!("{folder}/{}", file.file_stem().unwrap_or_default().to_string_lossy()),
            project,
            root,
            comp: id,
            frames,
            reference: false,
        });
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

/// How many effects the card's plan leaves to the card on each layer.
fn left(shot: &Shot, frame: i32, quality: PreviewQuality) -> Vec<usize> {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(&shot.project, &shot.comp, frame, &shot.root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers.iter().map(|l| l.on_card.iter().filter(|c| matches!(c, render::OnCard::Fx(_))).count()).collect()
}

fn said(log: FrameLog) -> String {
    let mut ids: Vec<&str> = log.finish().iter().map(|d| d.id.as_str()).collect();
    ids.sort();
    ids.dedup();
    ids.join(", ")
}

#[test]
fn b223_gpu_blurs() {
    let out = repo("verification/B-223_gpu_blurs_table.md");
    let mut gpu = Gpu::new().expect("a usable card");
    let (mut rows, mut checks, mut passed) = (String::new(), 0, 0);
    // Each effect's checks, passes, frames with it on the card, frames the CPU drew, largest difference.
    let mut by_effect: BTreeMap<&str, (usize, usize, usize, usize, u8)> = BTreeMap::new();
    let mut worst: Option<((u8, usize), String, Vec<u8>, Vec<u8>, usize, usize)> = None;
    let mut shots = fixtures();
    let n_fixtures = shots.len();
    for (case, (effect, p)) in settings().iter().enumerate() {
        let k = settings()[..case].iter().filter(|s| s.0 == *effect).count();
        shots.push(reference(effect, k, p));
    }
    for shot in &shots {
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for &frame in &shot.frames {
                let mut cache = CelCache::viewer();
                let mut log = FrameLog::new(3);
                let c = preview::preview_frame_cached(&shot.project, &shot.comp, frame, &shot.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache)
                    .unwrap_or_else(|d| panic!("{} frame {frame} on the CPU: {}", shot.name, d.message));
                let (w, h) = (c.width(), c.height());
                let said_cpu = said(log);
                let c = c.to_srgb8_straight();
                let mut log = FrameLog::new(3);
                let (g, ..) = preview::preview_frame_srgb8(&shot.project, &shot.comp, frame, &shot.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu)
                    .unwrap_or_else(|d| panic!("{} frame {frame} on the GPU: {}", shot.name, d.message));
                let said_gpu = said(log);
                let on_cpu = said_gpu.contains(DiagnosticId::GpuPreviewOnCpu.as_str());
                let n = left(shot, frame, quality);
                let on_card = n.iter().sum::<usize>();
                let d = distance(&c, &g);
                // A frame the card refuses (an 8 bpc or Float composition, an adjustment layer
                // with an effect it does not draw there) must be the CPU's exactly, saying so.
                let expected_cpu = {
                    let mut ids: Vec<&str> = said_cpu.split(", ").filter(|s| !s.is_empty()).chain([DiagnosticId::GpuPreviewOnCpu.as_str()]).collect();
                    ids.sort();
                    ids.join(", ")
                };
                // At Full the first layer's blur is the card's; at Draft a blur shrunk to nothing is not left.
                let first = quality == PreviewQuality::Draft || !shot.reference || n.first() == Some(&1);
                let pass = if on_cpu {
                    !shot.reference && d.0 == 0 && said_gpu == expected_cpu
                } else {
                    said_cpu == said_gpu && d.0 <= LIMIT && (!shot.reference || on_card > 0) && first
                };
                checks += 1;
                passed += pass as usize;
                let e = by_effect.entry(shot.effect).or_default();
                e.0 += 1;
                e.1 += pass as usize;
                e.2 += (on_card > 0 && !on_cpu) as usize;
                e.3 += on_cpu as usize;
                e.4 = e.4.max(d.0);
                let case = format!("{} frame {frame}, {}", shot.name, quality.label());
                let counts = n.iter().take(3).map(|k| k.to_string()).collect::<Vec<_>>().join(" / ");
                let _ = writeln!(
                    rows,
                    "| {case} | {counts} | {} | {} | {} | {} |",
                    d.0,
                    d.1,
                    match (said_gpu.is_empty(), said_cpu == said_gpu) {
                        (true, true) => "none".to_string(),
                        (false, true) => format!("{said_gpu}, on both"),
                        (_, false) => format!("CPU: {said_cpu}; GPU: {said_gpu}"),
                    },
                    match (on_cpu, pass) {
                        (true, true) => "PASS: the card refused the frame and said so; the CPU's picture exactly",
                        (true, false) => "FAIL: the CPU drew it",
                        (false, true) => "PASS",
                        (false, false) if d.0 > LIMIT => "FAIL: the pictures differ",
                        (false, false) if !first => "FAIL: the first layer's blur not on the card",
                        (false, false) if shot.reference && on_card == 0 => "FAIL: the effect was not left to the card",
                        (false, false) => "FAIL: the warnings differ",
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
    let mut identical = (0, 0);
    for reference in shots.iter().filter(|s| s.reference) {
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            let mut log = FrameLog::new(3);
            let tile = match quality {
                PreviewQuality::Full | PreviewQuality::Half => DEFAULT_TILE_SIZE,
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
            identical.0 += 1;
            identical.1 += same as usize;
            if !same {
                let _ = writeln!(rows, "| {} frame 100, {}: the plan made for the card, drawn by the CPU | — | — | — | the frame differs | FAIL |", reference.name, quality.label());
            }
        }
    }

    let pictures = repo("verification/B-223 pictures");
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
    for (effect, _) in FIVE {
        let (n, p, on_card, cpu, most) = by_effect[effect];
        let _ = writeln!(summary, "| {effect} | {n} | {on_card} | {cpu} | {most} | {p} of {n} |");
    }
    let s = format!(
        "# B-223: five blurs on the GPU against the CPU\n\n\
         Written by `tests/b223_gpu_blurs.rs`. The card: {}.\n\n\
         Fast Box Blur, Channel Blur, Compound Blur, Selective Color Blur and CC Vector Blur, \
         each the CPU's rule on the card (D-342). Compound Blur and CC Vector Blur read another \
         layer as a map, which the card is handed as a picture of its own. Selective Color Blur \
         chooses pixels by an 8-bit rounding, so, like an HSV Key, it only begins a run.\n\n\
         The cases: every fixture naming one of the five ({n_fixtures} files), at every frame it \
         has; and the reference shot with each effect on its first three layers (the second \
         after a Drop Shadow), {} settings, at frames 0, 100 and 239. Each at Full and Draft.\n\n\
         Each row compares the eight-bit picture the page receives, drawn by the CPU and by the \
         GPU. **The rule: no channel of any pixel more than {LIMIT} level of 255 apart** \
         (ADR-006, D-100), the same warnings on both, and on a reference shot row the effect in \
         fact on the card, at Full the first layer's blur. 8 bpc and After Effects 32 bpc \
         compositions give the card no effect (D-330, D-333); a few frames the card refuses \
         whole (Float depth, an adjustment layer): those must be the CPU's picture exactly, with \
         the card's message `GPU_PREVIEW_ON_CPU` its only extra warning.\n\n\
         **{passed} of {checks} checks pass.**\n\n\
         The CPU drawing each plan made for the card draws the plan made for the CPU byte for \
         byte in {} of {}.\n\n\
         The worst comparison is \"{worst_case}\": largest difference {largest} of 255, pixels \
         differing: {count}. Its pictures are in `verification/B-223 pictures/`: `cpu.png`, \
         `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square \
         around every pixel where they do not.\n\n\
         ## Each effect\n\n\
         | Effect | Frames compared | Frames with an effect on the card | Frames the card refused | Largest difference (of 255) | Pass |\n|---|---:|---:|---:|---:|---|\n{summary}\n\
         ## Every frame\n\n\
         Effects left to the card on the first three layers.\n\n\
         | Case | Left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |\n|---|---|---:|---:|---|---|\n{rows}",
        gpu.about(),
        settings().len(),
        identical.1,
        identical.0,
    );
    fs::write(&out, s).expect("write the B-223 table");
    assert_eq!(passed, checks, "B-223: {passed} of {checks} checks pass");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-222's way: the reference shot with a blur on its first three layers (the
/// second after a Drop Shadow), most after a Noise that changes every frame so nothing is kept,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; then seven loops are timed, the median of their
/// 210 frames "again", in ms. The same test run on the build before B-223 gives the "before"
/// columns. Writes `B223_OUT`, or `verification/B-223_timing_raw.md`.
#[test]
#[ignore = "B-223: a measurement, run deliberately with --release --ignored"]
fn b223_gpu_blurs_timing() {
    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n\
         | Shot | Quality | Left to the card | GPU first | GPU again |\n|---|---|---|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let fx = |id: &str, type_id: &str, p: Value| json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": p});
    let shots: [(&str, bool, &str, Value); 6] = [
        ("Fast Box Blur 30 x 3", false, "core.fast_box_blur", json!({"radius": 30, "iterations": 3, "edges": "transparent", "dimensions": "both"})),
        ("Noise, then Fast Box Blur 30 x 3", true, "core.fast_box_blur", json!({"radius": 30, "iterations": 3, "edges": "transparent", "dimensions": "both"})),
        ("Noise, then CC Vector Blur", true, "core.vector_blur",
            json!({"type": "natural", "amount": 20, "angle_offset": 0, "ridge_smoothness": 10, "layer": "", "fit": "stretch", "property": "lightness", "map_softness": 20})),
        ("Noise, then Channel Blur", true, "core.channel_blur",
            json!({"red_blurriness": 12, "green_blurriness": 4, "blue_blurriness": 0, "alpha_blurriness": 6, "edges": "transparent", "dimensions": "both"})),
        ("Noise, then Compound Blur", true, "core.compound_blur", json!({"layer": "layer-4", "fit": "stretch", "max_blur": 30, "invert": "off", "edges": "transparent"})),
        ("Selective Color Blur", false, "core.selective_color_blur", json!({"blur": 30, "colors": ["#1e1a24", "#f0d0b0"], "tolerance": 30})),
    ];
    for (name, noise, type_id, p) in shots {
        let stack = |id: &str| {
            let mut v = Vec::new();
            if noise {
                v.push(fx(&format!("{id}n"), "core.noise", json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"})));
            }
            v.push(fx(&format!("{id}b"), type_id, p.clone()));
            v
        };
        let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
        let mut j: Value = serde_json::from_str(&text).expect("the reference shot is JSON");
        let shadow = fx("d2", "core.drop_shadow", json!({"color": "#000000", "opacity": 60, "direction": 200, "distance": 6, "softness": 4}));
        let layers = &mut j["compositions"][0]["layers"];
        layers[0]["effects"] = Value::Array(stack("a"));
        layers[1]["effects"] = Value::Array([vec![shadow], stack("b")].concat());
        layers[2]["effects"] = Value::Array(stack("c"));
        let loaded = persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("{name}: {}", d.message));
        let shot = Shot {
            effect: "Fast Box Blur",
            name: name.to_string(),
            project: loaded.document.project().clone(),
            root: repo("Fixtures/reference_shot"),
            comp: Id::new("comp-reference-shot"),
            frames: Vec::new(),
            reference: true,
        };
        let quality = PreviewQuality::Full;
        let mut cache = CelCache::viewer();
        gpu.forget();
        let (mut first, mut times) = (Vec::new(), Vec::new());
        for pass in 0..8 {
            for frame in (0..240).step_by(8) {
                let mut log = FrameLog::new(3);
                let t = std::time::Instant::now();
                drop(preview::preview_frame_srgb8(&shot.project, &shot.comp, frame, &shot.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).expect("GPU frame"));
                let ms = t.elapsed().as_secs_f64() * 1000.0;
                if pass > 0 { times.push(ms) } else { first.push(ms) }
            }
        }
        let n = left(&shot, 100, quality).iter().take(3).map(|k| k.to_string()).collect::<Vec<_>>().join(" / ");
        let _ = writeln!(s, "| {} | {} | {n} | {:.1} | {:.1} |", shot.name, quality.label(), median(first), median(times));
    }
    let out = std::env::var("B223_OUT").map(PathBuf::from).unwrap_or_else(|_| repo("verification/B-223_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
