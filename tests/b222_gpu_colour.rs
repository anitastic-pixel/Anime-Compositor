//! B-222 (D-341): ten colour effects of their own pixel on the GPU, against the CPU.
//!
//! Exposure, Tint, Shift Channels, Solid Composite, Change to Color, Color Key, Select Color,
//! Line Recolor, Colorama and Extract each read only the pixel they write, so the card draws them
//! in its one-pixel `tone` pass, and inside a run of colour effects drawn in one pass (B-172).
//! Every fixture of the ten, at every frame it has, and the reference shot with each, is drawn by
//! the CPU and by the card, alone and with neighbours in one run, at Full and Draft. The rule:
//! no channel of any pixel more than 1 level of 255 apart (ADR-006, D-100).
//!
//! Writes `verification/B-222_gpu_colour_table.md` and `verification/B-222 pictures/`.

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

/// The three that choose by an 8-bit rounding and so only begin a run on the card (D-224, D-341).
const LOOKS_FIRST: [&str; 3] = ["Color Key", "Select Color", "Line Recolor"];

struct Shot {
    effect: &'static str,
    name: String,
    project: Project,
    root: PathBuf,
    comp: Id,
    frames: Vec<i32>,
    reference: bool,
    /// The effects the first layer's run must leave to the card at Full, for a reference shot.
    first_layer: Option<usize>,
    /// Drawn with neighbours, so in one pass with them.
    fused: bool,
}

/// Each effect's name and type.
const TEN: [(&str, &str); 10] = [
    ("Exposure", "core.exposure"),
    ("Tint", "core.tint"),
    ("Shift Channels", "core.shift_channels"),
    ("Solid Composite", "core.solid_composite"),
    ("Change to Color", "core.change_to_color"),
    ("Color Key", "core.color_key"),
    ("Select Color", "core.select_color"),
    ("Line Recolor", "core.line_recolor"),
    ("Colorama", "core.colorama"),
    ("Extract", "core.extract"),
];

/// The settings each effect is given in the reference shot; Shift Channels and Colorama, which
/// have no fixtures, and the ones with more than one way of working, more than once.
fn settings() -> Vec<(&'static str, Value)> {
    let colorama = |phase: &str, shift: f64, reps: f64, stops: u32, blend: f64| {
        json!({"get_phase": phase, "layer": "", "fit": "stretch", "phase_shift": shift, "cycle_repetitions": reps, "stops": stops,
            "color_1": "#102060", "color_2": "#e04080", "color_3": "#ffd040", "color_4": "#40c0a0", "color_5": "#f0f0ff", "blend_with_original": blend})
    };
    vec![
        ("Exposure", json!({"stops": 0.7, "offset": 0.0, "gamma": 1.0, "bypass": "off"})),
        ("Exposure", json!({"stops": -0.4, "offset": 0.03, "gamma": 1.3, "bypass": "off"})),
        ("Tint", json!({"color": [0.9, 0.5, 0.2], "amount": 0.7})),
        ("Shift Channels", json!({"take_alpha": "alpha", "take_red": "green", "take_green": "blue", "take_blue": "red"})),
        ("Shift Channels", json!({"take_alpha": "lightness", "take_red": "hue", "take_green": "saturation", "take_blue": "luminance"})),
        ("Shift Channels", json!({"take_alpha": "half", "take_red": "full", "take_green": "off", "take_blue": "alpha"})),
        ("Solid Composite", json!({"source_opacity": 70, "color": "#3050a0", "opacity": 80, "blend": "normal"})),
        ("Solid Composite", json!({"source_opacity": 100, "color": "#ffa040", "opacity": 60, "blend": "multiply"})),
        ("Solid Composite", json!({"source_opacity": 90, "color": "#203080", "opacity": 50, "blend": "screen"})),
        ("Solid Composite", json!({"source_opacity": 80, "color": "#402010", "opacity": 70, "blend": "add"})),
        ("Change to Color", json!({"from": "#c04040", "to": "#40a0ff", "change": "hue_lightness_saturation", "change_by": "transforming",
            "hue_tolerance": 20, "lightness_tolerance": 60, "saturation_tolerance": 60, "softness": 40, "view_matte": "off"})),
        ("Change to Color", json!({"from": "#e0b090", "to": "#80ff40", "change": "hue", "change_by": "setting",
            "hue_tolerance": 15, "lightness_tolerance": 50, "saturation_tolerance": 50, "softness": 30, "view_matte": "on"})),
        ("Color Key", json!({"colors": ["#1e1a24", "#f0d0b0"], "tolerance": 30, "softness": 10, "match": "rgb"})),
        ("Color Key", json!({"colors": ["#d04030"], "tolerance": 25, "softness": 15, "match": "hue"})),
        ("Select Color", json!({"colors": ["#1e1a24"], "tolerance": 20, "keep": "chosen"})),
        ("Select Color", json!({"colors": ["#f0d0b0", "#3060c0"], "tolerance": 35, "keep": "others"})),
        ("Line Recolor", json!({"colors": ["#1e1a24"], "tolerance": 20, "new_color": "#ff3000"})),
        ("Colorama", colorama("intensity", 40.0, 1.5, 4, 20.0)),
        ("Colorama", colorama("luminance", -90.0, 0.75, 5, 0.0)),
        ("Colorama", colorama("blue", 0.0, 3.0, 2, 50.0)),
        ("Colorama", colorama("alpha", 10.0, 1.0, 3, 0.0)),
        ("Extract", json!({"channel": "luminance", "black_point": 30, "white_point": 220, "black_softness": 20, "white_softness": 30, "invert": "off"})),
        ("Extract", json!({"channel": "red", "black_point": 60, "white_point": 200, "black_softness": 0, "white_softness": 0, "invert": "on"})),
    ]
}

fn type_of(effect: &str) -> &'static str {
    TEN.iter().find(|t| t.0 == effect).expect("one of the ten").1
}

/// The neighbours of a run: Hue/Saturation before (not before the three that only begin a run)
/// and Levels after, B-221's settings.
fn neighbours(effect: &str, id: &str) -> (Option<Value>, Value) {
    let before = (!LOOKS_FIRST.contains(&effect)).then(|| {
        json!({"instance_id": format!("{id}-hs"), "type_id": "core.hue_saturation", "enabled": true, "parameters": {"hue": 40, "saturation": 30, "lightness": -10}})
    });
    let after = json!({"instance_id": format!("{id}-lv"), "type_id": "core.levels", "enabled": true,
        "parameters": {"input_black": 20, "input_white": 230, "gamma": 1.6, "output_black": 10, "output_white": 245}});
    (before, after)
}

/// Each instance of `type_id` in the project given its neighbours.
fn with_neighbours(j: &mut Value, effect: &str, type_id: &str) {
    for comp in j["compositions"].as_array_mut().expect("compositions") {
        for layer in comp["layers"].as_array_mut().expect("layers") {
            let Some(stack) = layer["effects"].as_array_mut() else { continue };
            let mut out = Vec::new();
            for e in stack.drain(..) {
                if e["type_id"] == type_id {
                    let id = e["instance_id"].as_str().unwrap_or("x").to_string();
                    let (before, after) = neighbours(effect, &id);
                    out.extend(before);
                    out.push(e);
                    out.push(after);
                } else {
                    out.push(e);
                }
            }
            *stack = out;
        }
    }
}

/// The reference shot with `parameters` on its first three layers, the second after a Drop
/// Shadow, as B-151 has it; `fused`, with its neighbours.
fn reference(effect: &'static str, case: usize, parameters: &Value, fused: bool) -> Shot {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: Value = serde_json::from_str(&text).expect("the reference shot is JSON");
    let type_id = type_of(effect);
    let fx = |id: &str| json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": parameters});
    let shadow = json!({"instance_id": "b222-s", "type_id": "core.drop_shadow", "enabled": true, "parameters": {"color": "#000000", "opacity": 60, "direction": 200, "distance": 6, "softness": 4}});
    let layers = &mut j["compositions"][0]["layers"];
    layers[0]["effects"] = json!([fx("b222-a")]);
    layers[1]["effects"] = json!([shadow, fx("b222-b")]);
    layers[2]["effects"] = json!([fx("b222-c")]);
    if fused {
        with_neighbours(&mut j, effect, type_id);
    }
    let loaded = persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot with {effect}: {}", d.message));
    let run = if fused { if LOOKS_FIRST.contains(&effect) { 2 } else { 3 } } else { 1 };
    Shot {
        effect,
        name: format!("the reference shot with {effect} ({}){}", case + 1, if fused { ", in a run" } else { "" }),
        project: loaded.document.project().clone(),
        root: repo("Fixtures/reference_shot"),
        comp: Id::new("comp-reference-shot"),
        frames: vec![0, 100, 239],
        reference: true,
        first_layer: Some(run),
        fused,
    }
}

/// Every fixture naming one of the ten, each at every frame it has, alone and with neighbours. A
/// file is the first effect's it names; one that must not open (a malformed setting) has no frame
/// to compare. `exposure_bypass_word.json` is not yet a fixture (it is not in the repository).
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
        let Some(&(effect, type_id)) = TEN.iter().find(|t| text.contains(&format!("\"{}\"", t.1))) else { continue };
        let Ok(loaded) = persist::load(&file) else { continue };
        let root = file.parent().expect("a folder").to_path_buf();
        let folder = root.file_name().unwrap_or_default().to_string_lossy().into_owned();
        let name = format!("{folder}/{}", file.file_stem().unwrap_or_default().to_string_lossy());
        for fused in [false, true] {
            let project = if fused {
                let mut j: Value = serde_json::from_str(&text).expect("a fixture is JSON");
                with_neighbours(&mut j, effect, type_id);
                match persist::load_str(&j.to_string()) {
                    Ok(l) => l.document.project().clone(),
                    Err(_) => continue,
                }
            } else {
                loaded.document.project().clone()
            };
            let comp = &project.compositions[0];
            let (id, frames) = (comp.id.clone(), (comp.start_frame..comp.start_frame + comp.duration_frames as i32).collect());
            shots.push(Shot {
                effect,
                name: if fused { format!("{name}, in a run") } else { name.clone() },
                project,
                root: root.clone(),
                comp: id,
                frames,
                reference: false,
                first_layer: None,
                fused,
            });
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
fn b222_gpu_colour() {
    let out = repo("verification/B-222_gpu_colour_table.md");
    let mut gpu = Gpu::new().expect("a usable card");
    let (mut rows, mut checks, mut passed) = (String::new(), 0, 0);
    // Each effect's checks, passes, frames with it on the card, frames the CPU drew, largest difference.
    let mut by_effect: BTreeMap<&str, (usize, usize, usize, usize, u8)> = BTreeMap::new();
    let mut worst: Option<((u8, usize), String, Vec<u8>, Vec<u8>, usize, usize)> = None;
    let mut shots = fixtures();
    let n_fixtures = shots.iter().filter(|s| !s.fused).count();
    for (case, (effect, p)) in settings().iter().enumerate() {
        let k = settings()[..case].iter().filter(|s| s.0 == *effect).count();
        for fused in [false, true] {
            shots.push(reference(effect, k, p, fused));
        }
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
                let whole = quality == PreviewQuality::Draft || shot.first_layer.is_none_or(|k| n.first() == Some(&k));
                let pass = if on_cpu {
                    !shot.reference && d.0 == 0 && said_gpu == expected_cpu
                } else {
                    said_cpu == said_gpu && d.0 <= LIMIT && (!shot.reference || on_card > 0) && whole
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
                        (false, false) if !whole => "FAIL: not the whole run on the card",
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

    // B-172: a run drawn in one pass is, byte for byte, the run drawn a pass each, in fewer passes.
    let mut fused = (0, 0);
    for shot in shots.iter().filter(|s| s.reference && s.fused) {
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            let frame = 100;
            let draw = |gpu: &mut Gpu| {
                gpu.forget();
                let mut log = FrameLog::new(3);
                let before = gpu.dispatched();
                let bytes = preview::preview_frame_srgb8(&shot.project, &shot.comp, frame, &shot.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer(), gpu).expect("a card frame").0;
                (bytes, gpu.dispatched() - before)
            };
            gpu.fuse(false);
            let (apart, passes_apart) = draw(&mut gpu);
            gpu.fuse(true);
            let (together, passes_together) = draw(&mut gpu);
            let ok = apart == together && passes_together < passes_apart;
            checks += 1;
            passed += ok as usize;
            fused.0 += 1;
            fused.1 += ok as usize;
            let _ = writeln!(
                rows,
                "| {} frame {frame}, {}: one pass against a pass each | — | {} | — | passes {passes_apart} apart, {passes_together} together | {} |",
                shot.name,
                quality.label(),
                if apart == together { "byte-identical".to_string() } else { "the frames differ".to_string() },
                if ok { "PASS" } else { "FAIL" }
            );
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

    let pictures = repo("verification/B-222 pictures");
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
    for (effect, _) in TEN {
        let (n, p, on_card, cpu, most) = by_effect[effect];
        let _ = writeln!(summary, "| {effect} | {n} | {on_card} | {cpu} | {most} | {p} of {n} |");
    }
    let s = format!(
        "# B-222: ten colour effects on the GPU against the CPU\n\n\
         Written by `tests/b222_gpu_colour.rs`. The card: {}.\n\n\
         Exposure, Tint, Shift Channels, Solid Composite, Change to Color, Color Key, Select \
         Color, Line Recolor, Colorama and Extract each read only the pixel they write, so the \
         card draws them, and inside a run of colour effects drawn in one pass (B-172) (D-341). \
         Color Key, Select Color and Line Recolor choose pixels by an 8-bit rounding, so, like \
         an HSV Key, they only begin a run. A Colorama that adds another layer's phase stays the \
         CPU's.\n\n\
         The cases: every fixture naming one of the ten ({n_fixtures} files), at every frame it \
         has, alone and with each instance given neighbours (a Hue/Saturation before, except \
         before the three that only begin a run, and a Levels after); and the reference shot with \
         each effect on its first three layers (the second after a Drop Shadow), {} settings, \
         alone and with the same neighbours, at frames 0, 100 and 239. Each at Full and Draft.\n\n\
         Each row compares the eight-bit picture the page receives, drawn by the CPU and by the \
         GPU. **The rule: no channel of any pixel more than {LIMIT} level of 255 apart** \
         (ADR-006, D-100), the same warnings on both, and on a reference shot row the effect in \
         fact on the card, at Full the first layer's whole run. Most fixtures are 8 bpc or \
         After Effects 32 bpc compositions, which the card is given no effect in (D-330, D-333); \
         a few the card refuses whole (Float depth, an adjustment layer): those must be the \
         CPU's picture exactly, with the card's message `GPU_PREVIEW_ON_CPU` its only extra \
         warning.\n\n\
         **{passed} of {checks} checks pass.**\n\n\
         The reference shot's runs drawn in one pass match the same runs drawn a pass each, byte \
         for byte and in fewer passes, in {} of {} (frame 100, Full and Draft). The CPU drawing \
         each plan made for the card draws the plan made for the CPU byte for byte in {} of {}.\n\n\
         The worst comparison is \"{worst_case}\": largest difference {largest} of 255, pixels \
         differing: {count}. Its pictures are in `verification/B-222 pictures/`: `cpu.png`, \
         `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square \
         around every pixel where they do not.\n\n\
         ## Each effect\n\n\
         | Effect | Frames compared | Frames with an effect on the card | Frames the card refused | Largest difference (of 255) | Pass |\n|---|---:|---:|---:|---:|---|\n{summary}\n\
         ## Every frame\n\n\
         Effects left to the card on the first three layers.\n\n\
         | Case | Left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |\n|---|---|---:|---:|---|---|\n{rows}",
        gpu.about(),
        settings().len(),
        fused.1,
        fused.0,
        identical.1,
        identical.0,
    );
    fs::write(&out, s).expect("write the B-222 table");
    assert_eq!(passed, checks, "B-222: {passed} of {checks} checks pass");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-221's way: the reference shot with a Color Key, an Exposure and a Tint on
/// its first three layers (the second after a Drop Shadow), and a Noise that changes every frame
/// then an Exposure and a Tint, every eighth frame asked for as the viewer asks, whole. The first loop
/// starts with empty caches and its 30 frames' median is "first"; then seven loops are timed, the
/// median of their 210 frames "again", in ms. The same test run on the build before B-222 gives
/// the "before" columns. Writes `B222_OUT`, or `verification/B-222_timing_raw.md`.
#[test]
#[ignore = "B-222: a measurement, run deliberately with --release --ignored"]
fn b222_gpu_colour_timing() {
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
    let stack = |id: &str, noise: bool| {
        let mut v = Vec::new();
        if noise {
            v.push(fx(&format!("{id}n"), "core.noise", json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"})));
        }
        // A Colour Key only begins a run, so the changing one has none: Noise, Exposure, Tint all the card's.
        if !noise {
            v.push(fx(&format!("{id}k"), "core.color_key", json!({"colors": ["#1e1a24", "#f0d0b0"], "tolerance": 30, "softness": 10, "match": "rgb"})));
        }
        v.push(fx(&format!("{id}e"), "core.exposure", json!({"stops": -0.4, "offset": 0.03, "gamma": 1.3, "bypass": "off"})));
        v.push(fx(&format!("{id}t"), "core.tint", json!({"color": [0.9, 0.5, 0.2], "amount": 0.7})));
        v
    };
    let shot = |name: &str, noise: bool| {
        let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
        let mut j: Value = serde_json::from_str(&text).expect("the reference shot is JSON");
        let shadow = fx("d2", "core.drop_shadow", json!({"color": "#000000", "opacity": 60, "direction": 200, "distance": 6, "softness": 4}));
        let layers = &mut j["compositions"][0]["layers"];
        layers[0]["effects"] = Value::Array(stack("a", noise));
        layers[1]["effects"] = Value::Array([vec![shadow], stack("b", noise)].concat());
        layers[2]["effects"] = Value::Array(stack("c", noise));
        let loaded = persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("{name}: {}", d.message));
        Shot {
            effect: "Color Key",
            name: name.to_string(),
            project: loaded.document.project().clone(),
            root: repo("Fixtures/reference_shot"),
            comp: Id::new("comp-reference-shot"),
            frames: Vec::new(),
            reference: true,
            first_layer: None,
            fused: true,
        }
    };
    for shot in [shot("Color Key, Exposure, Tint", false), shot("Noise, then Exposure, Tint", true)] {
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
    let out = std::env::var("B222_OUT").map(PathBuf::from).unwrap_or_else(|_| repo("verification/B-222_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
