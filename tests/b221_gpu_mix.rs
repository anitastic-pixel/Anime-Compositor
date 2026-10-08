//! B-221: an effect mixed below 100 drawn on the card (D-340, EFFECTS.md P0-2). Made from
//! `tests/b155_gpu_chain.rs`, with the same rules.
//!
//! An effect's Mix lays its result back over what it was given, `before + m (after - before)` on
//! all four channels, `before` where it was in the drawing the effect grew and clear outside it
//! (D-202, document 21; `effects::mix_back`). Until now a layer with an effect mixed below 100
//! was the CPU's from that effect on. Now the card keeps what the effect was given, runs the
//! effect, and mixes, in one small pass; inside a run of colour effects drawn in one pass
//! (B-172), the same sum in the run's own pass.
//!
//! Every effect the card can draw is put last on the reference shot's first three layers (after
//! Levels and a Gaussian Blur, a Drop Shadow and Hue/Saturation, and Curves), at Mix 0, 37 and
//! 100. Then a layer of colour effects drawn in one pass with mixed members among them, a mixed
//! Glow and a mixed Gaussian Blur (which grow the drawing), and an adjustment layer whose effects
//! are mixed.
//!
//! What is compared is what the page receives, eight-bit straight sRGB: the CPU's frame through
//! `preview_frame_cached`, and the card's through `preview_frame_srgb8`. The CPU stays the
//! authority (ADR-006, D-100); the rule stays 1 level of 255 (D-165).
//!
//! Writes `verification/B-221_gpu_mix_table.md` and, for the worst frame, three pictures in
//! `verification/B-221 pictures/`.

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

/// D-165's tolerance, in levels of 255.
const LIMIT: u8 = 1;

/// Every effect the card draws, with the settings B-155 gives it.
fn effects() -> Vec<(&'static str, &'static str, Value)> {
    vec![
        ("Radial Blur", "core.radial_blur", json!({"type": "spin", "amount": 12, "center": [35, 60]})),
        ("Bloom", "core.bloom", json!({"threshold": 60, "radius": 10, "intensity": 1, "streaks": "star", "length": 60, "angle": 15})),
        ("Directional Blur", "core.directional_blur", json!({"direction": 45, "length": 30})),
        ("Gaussian Blur", "core.gaussian_blur", json!({"sigma_px": 4})),
        ("Glow", "core.glow", json!({"based_on": "bright", "threshold": 60, "colors": [], "tolerance": 0, "radius": 30, "intensity": 1, "operation": "add", "tint": ""})),
        ("Curves", "core.curves", json!({"master": [[0, 0], [128, 150], [255, 255]], "red": [[0, 0], [64, 40], [192, 215], [255, 255]], "green": [[0, 0], [255, 255]], "blue": [[0, 20], [255, 235]]})),
        ("Levels", "core.levels", json!({"input_black": 20, "input_white": 230, "gamma": 1.6, "output_black": 10, "output_white": 245})),
        ("Hue/Saturation", "core.hue_saturation", json!({"hue": 40, "saturation": 30, "lightness": -10})),
        ("Gradient", "core.gradient", json!({"shape": "radial", "start": [50, 50], "end": [100, 100], "start_color": "#ffcc00", "end_color": "#3050ff", "start_opacity": 80, "end_opacity": 40, "blend": "multiply"})),
        ("Drop Shadow", "core.drop_shadow", json!({"color": "#102040", "opacity": 70, "direction": 135, "distance": 12, "softness": 9})),
        ("Lens Blur", "core.lens_blur", json!({"radius": 12, "edges": "transparent", "iris": "hexagon", "roundness": 20, "rotation": 15, "aspect": 1.3, "highlight_gain": 2, "highlight_threshold": 80})),
        ("Rim Light", "core.rim_light", json!({"color": "#ffe0a0", "direction": 45, "width": 4, "softness": 3, "intensity": 80, "blend": "screen"})),
        ("Outline", "core.outline", json!({"color": "#ffffff", "width": 3.5, "softness": 2, "opacity": 90})),
        ("Noise", "core.noise", json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"})),
        ("Chromatic Aberration", "core.chromatic_aberration", json!({"amount": 5, "center": [40, 60]})),
        ("Distance Gradation", "core.distance_gradation", json!({"color": "#3040a0", "width": 20, "opacity": 70, "invert": "off", "blend": "multiply"})),
        ("Light Rays", "core.light_rays", json!({"center": [50, 40], "length": 40, "threshold": 60, "intensity": 1.5, "color": "#ffe0b0"})),
        ("Exposure Flicker", "core.exposure_flicker", json!({"amount": 0.5, "hold": 3, "seed": 4})),
        ("Vignette", "core.vignette", json!({"amount": 60, "color": "#201030", "size": 90, "roundness": 50, "softness": 60, "center": [45, 55]})),
        ("Turbulent Displace", "core.turbulent_displace", json!({"amount": 12, "size": 50, "complexity": 3, "evolution": 30, "speed": 10, "seed": 5, "edges": "transparent"})),
        ("Fractal Noise", "core.fractal_noise", json!({"size": 80, "complexity": 5, "contrast": 130, "brightness": 5, "evolution": 0, "speed": 15, "seed": 3, "dark_color": "#102030", "light_color": "#ffe8c0", "opacity": 60, "blend": "screen"})),
        ("Gradient Map", "core.gradient_map", json!({"shadow_color": "#1a0a40", "midtone_color": "#c05060", "highlight_color": "#fff0c0", "midpoint": 40, "amount": 80})),
        ("Color Balance", "core.color_balance", json!({"shadows": [-20, 10, 30], "midtones": [15, -5, -10], "highlights": [10, 5, -25]})),
        ("Offset", "core.offset", json!({"shift": [37.5, -21.25]})),
        ("Invert", "core.invert", json!({"channel": "rgb", "amount": 80})),
        ("Brightness & Contrast", "core.brightness_contrast", json!({"brightness": 30, "contrast": 40})),
        ("Black & White", "core.black_white", json!({"reds": 120, "yellows": 110, "greens": -10, "cyans": -50, "blues": -50, "magentas": 120})),
        ("Posterize", "core.posterize", json!({"levels": 6})),
        ("Threshold", "core.threshold", json!({"level": 128})),
        ("Channel Mixer", "core.channel_mixer", json!({"red": [0, 0, 100, 0], "green": [0, 100, 0, 0], "blue": [100, 0, 0, 0], "monochrome": "off"})),
        ("Vibrance", "core.vibrance", json!({"vibrance": 40, "saturation": 20})),
        ("Leave Color", "core.leave_color", json!({"color": "#ff0000", "tolerance": 15, "softness": 10, "amount": 100})),
        ("Solarize", "core.solarize", json!({"threshold": 128})),
        ("Halftone", "core.halftone", json!({"size": 8, "angle": 45, "ink": "#000000", "paper": "#ffffff", "amount": 100})),
        ("Mosaic", "core.mosaic", json!({"size": 10})),
        ("Emboss", "core.emboss", json!({"direction": 135, "relief": 1, "contrast": 100, "mode": "grey"})),
        ("Find Edges", "core.find_edges", json!({"invert": "off", "amount": 100})),
        ("Sharpen", "core.sharpen", json!({"amount": 100, "radius": 1})),
        ("Diffusion", "core.diffusion", json!({"radius": 10, "amount": 50, "blend": "screen"})),
        ("Wave Warp", "core.wave_warp", json!({"shape": "sine", "height": 10, "width": 40, "direction": 90, "speed": 1, "phase": 0, "edges": "transparent"})),
        ("Ripple", "core.ripple", json!({"center": [50, 50], "amplitude": 5, "wavelength": 30, "speed": 20, "phase": 0, "fade": 0})),
        ("Twirl", "core.twirl", json!({"angle": 90, "radius": 50, "center": [50, 50]})),
        ("Bulge", "core.bulge", json!({"center": [50, 50], "radius": 50, "height": 1})),
        ("Mirror", "core.mirror", json!({"center": [50, 50], "angle": 0})),
        ("Linear Wipe", "core.linear_wipe", json!({"completion": 50, "angle": 90, "feather": 0})),
        ("Radial Wipe", "core.radial_wipe", json!({"completion": 50, "start_angle": 0, "center": [50, 50], "wipe": "clockwise", "feather": 0})),
        ("Venetian Blinds", "core.venetian_blinds", json!({"completion": 50, "angle": 0, "width": 20, "feather": 0})),
        ("Iris Wipe", "core.iris_wipe", json!({"completion": 50, "center": [50, 50], "feather": 0, "invert": "off"})),
        ("Simple Choker", "core.simple_choker", json!({"choke": -3})),
        ("Speed Lines", "core.speed_lines", json!({"center": [50, 50], "color": "#000000", "count": 120, "thickness": 1.5, "inner": 150, "inner_jitter": 40, "angle_jitter": 50, "seed": 0, "hold": 2, "opacity": 100})),
        ("Cross Glare", "core.cross_glare", json!({"threshold": 80, "length": 40, "points": 4, "angle": 45, "intensity": 1, "color": "#ffffff"})),
        ("Camera Shake", "core.camera_shake", json!({"amount": 10, "rotation": 0, "hold": 1, "seed": 0})),
        ("Rain", "core.rain", json!({"color": "#c8d8ff", "density": 30, "spacing": 24, "length": 20, "width": 1, "direction": 170, "speed": 30, "seed": 0, "opacity": 60})),
        ("Motion Tile", "core.motion_tile", json!({"output_width": 150, "output_height": 130, "mirror": "on"})),
        ("Color Lookup", "core.color_lookup", json!({"lut": "asset-lut"})),
        ("Line Blur", "core.line_blur", json!({"length": 12, "strength": 80, "lines_only": "off"})),
        ("HSV Key", "core.hsv_key", json!({"hue": 30, "hue_range": 40, "saturation": 50, "saturation_range": 50, "value": 60, "value_range": 40, "invert": "off"})),
        ("Paraffin", "core.paraffin", json!({"color": "#ffc890", "direction": 45, "spread": 60, "opacity": 70, "blend": "overlay"})),
        ("Kira-kira", "core.kira_kira", json!({"threshold": 60, "spacing": 24, "density": 80, "size": 12, "angle": 15, "twinkle": 50, "period": 24, "seed": 7, "opacity": 90, "shape": "star", "color": "#fff0c0"})),
        ("Median", "core.median", json!({"radius": 4, "operate_on_alpha": "on"})),
        ("Smart Blur", "core.smart_blur", json!({"radius": 4, "threshold": 40})),
        ("Roughen Edges", "core.roughen_edges", json!({"edge_type": "roughen_color", "edge_color": "#8a3c14", "border": 6, "size": 8, "complexity": 3, "evolution": 30, "speed": 10, "seed": 3})),
        ("Radial Shadow", "core.radial_shadow", json!({"color": "#203040", "opacity": 70, "light": [30, 10], "distance": 15, "softness": 8, "render": "glass_edge", "color_influence": 60, "shadow_only": "off"})),
        ("Bevel Alpha", "core.bevel_alpha", json!({"edge_thickness": 4, "light_angle": -45, "light_color": "#fff0d0", "light_intensity": 0.6})),
        ("Snowfall", "core.snowfall", json!({"color": "#ffffff", "density": 60, "spacing": 40, "size": 5, "depth": 40, "speed": 3, "wind": 0.5, "wiggle": 3, "period": 48, "seed": 5, "opacity": 90})),
        ("Cell Pattern", "core.cell_pattern", json!({"pattern": "crystals", "invert": "off", "contrast": 120, "disperse": 0.8, "size": 30, "evolution": 20, "seed": 2, "opacity": 60, "dark_color": "#102040", "light_color": "#f0e0c0", "blend": "screen"})),
        ("Polar Coordinates", "core.polar_coordinates", json!({"interpolation": 70, "conversion": "rect_to_polar"})),
        ("Optics Compensation", "core.optics_compensation", json!({"field_of_view": 60, "reverse": "off", "orientation": "diagonal", "center": [45, 55]})),
        ("Corner Pin", "core.corner_pin", json!({"upper_left": [5, 3], "upper_right": [92, 8], "lower_left": [0, 100], "lower_right": [110, 96]})),
    ]
}

/// The ones that only begin a run on the card (D-224): last in a stack, the card has only them.
const LOOKS_FIRST: [&str; 5] = ["Bloom", "Glow", "Paraffin", "Kira-kira", "HSV Key"];

struct Shot {
    name: String,
    project: Project,
    root: PathBuf,
    comp: Id,
    /// How many effects the card must be left on the first layer at Full, where that is held.
    first_layer: Option<usize>,
    /// Whether an effect is mixed below 100, so the card's plan must say `Mix`.
    mixed: bool,
}

/// `name`'s settings at `mix`, as instance `id`.
fn fx(id: &str, name: &str, mix: f64) -> Value {
    let all = effects();
    let (_, type_id, parameters) = all.iter().find(|e| e.0 == name).unwrap_or_else(|| panic!("no settings for {name}"));
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "mix": mix, "parameters": parameters})
}

/// The reference shot with `stacks` on its first three layers and, when given, an adjustment
/// layer over them all running `adjust`.
fn shot(name: String, stacks: [Vec<Value>; 3], adjust: Option<Vec<Value>>, first_layer: Option<usize>, mixed: bool) -> Shot {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: Value = serde_json::from_str(&text).expect("the reference shot is JSON");
    for (i, stack) in stacks.into_iter().enumerate() {
        j["compositions"][0]["layers"][i]["effects"] = Value::Array(stack);
    }
    if let Some(stack) = adjust {
        // B-156's adjustment layer, scaled, turned and at 60% opacity.
        let key = |v: Value| json!({"base": v, "keyframes": []});
        let comp = &mut j["compositions"][0];
        let at = comp["layers"].as_array().expect("the reference shot's layers").len();
        comp["layers"].as_array_mut().expect("the layers").push(json!({
            "id": "b221-adj", "kind": "adjustment", "name": "b221 adjustment", "enabled": true, "locked": false,
            "in_frame": 0, "out_frame": 240,
            "transform": {"anchor": key(json!([960, 540])), "position": key(json!([900, 500])), "scale": key(json!([60, 70])), "rotation": key(json!(12)), "opacity": key(json!(0.6))},
            "matte": null, "blend_mode": "normal", "effects": stack,
        }));
        comp["layer_order"].as_array_mut().expect("the layer order").insert(at, json!("b221-adj"));
    }
    // Color Lookup's file, from its own fixtures.
    j["assets"].as_array_mut().expect("the reference shot's assets").push(json!({"id": "asset-lut", "kind": "lut", "name": "warm_17", "path": "../cube_lut/luts/warm_17.cube"}));
    let loaded = persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("{name}: {}", d.message));
    Shot { name, project: loaded.document.project().clone(), root: repo("Fixtures/reference_shot"), comp: Id::new("comp-reference-shot"), first_layer, mixed }
}

const MIXES: [f64; 3] = [0.0, 37.0, 100.0];

/// The run of colour effects drawn in one pass with mixed members, the growing ones mixed, and
/// the adjustment layer.
fn special() -> Vec<Shot> {
    vec![
        shot(
            "colour effects in one pass, three of them mixed (Hue/Saturation 37, Vibrance 0, Invert 50)".into(),
            [
                vec![fx("a1", "Levels", 100.0), fx("a2", "Hue/Saturation", 37.0), fx("a3", "Curves", 100.0), fx("a4", "Vibrance", 0.0), fx("a5", "Color Balance", 100.0)],
                vec![fx("b1", "Invert", 50.0), fx("b2", "Brightness & Contrast", 100.0), fx("b3", "Gradient Map", 37.0)],
                vec![fx("c1", "Noise", 37.0), fx("c2", "Posterize", 63.0)],
            ],
            None,
            Some(5),
            true,
        ),
        shot(
            "a Glow at 50, a Gaussian Blur at 37, then Levels at 37 (growing effects mixed)".into(),
            [
                vec![fx("a1", "Glow", 50.0), fx("a2", "Gaussian Blur", 37.0), fx("a3", "Levels", 37.0)],
                vec![fx("b1", "Drop Shadow", 37.0), fx("b2", "Directional Blur", 50.0), fx("b3", "Outline", 37.0)],
                vec![fx("c1", "Lens Blur", 50.0), fx("c2", "Motion Tile", 37.0)],
            ],
            None,
            Some(3),
            true,
        ),
        shot(
            "an adjustment layer: Gaussian Blur 37, Levels 50, Curves 37, Hue/Saturation".into(),
            [vec![], vec![], vec![]],
            Some(vec![fx("j1", "Gaussian Blur", 37.0), fx("j2", "Levels", 50.0), fx("j3", "Curves", 37.0), fx("j4", "Hue/Saturation", 100.0)]),
            None,
            true,
        ),
    ]
}

fn shots() -> Vec<Shot> {
    let mut shots = Vec::new();
    for (name, ..) in effects() {
        for mix in MIXES {
            shots.push(shot(
                format!("{name} at Mix {mix}"),
                [
                    vec![fx("l1", "Levels", 100.0), fx("g1", "Gaussian Blur", 100.0), fx("x1", name, mix)],
                    vec![fx("d2", "Drop Shadow", 100.0), fx("h2", "Hue/Saturation", 100.0), fx("x2", name, mix)],
                    vec![fx("c3", "Curves", 100.0), fx("x3", name, mix)],
                ],
                None,
                Some(if LOOKS_FIRST.contains(&name) { 1 } else { 3 }),
                mix < 100.0,
            ));
        }
    }
    shots.extend(special());
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

/// How many effects the card's plan leaves to the card on each of the first three layers, and
/// whether any of the plan's effects is mixed.
fn left(shot: &Shot, frame: i32, quality: PreviewQuality) -> (Vec<usize>, bool) {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(&shot.project, &shot.comp, frame, &shot.root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    let kinds = ["Radial(Radial", "Bloom(Bloom", "Directional(Directional", "Gaussian(Gaussian", "Glow(Glow", "Fx(Fx"];
    let counts = plan.layers.iter().take(3).map(|l| {
        let said = format!("{:?}", l.on_card);
        kinds.iter().map(|k| said.matches(k).count()).sum()
    });
    // An adjustment layer's run is made from its stack when the card draws it.
    let size = (plan.width, plan.height);
    let runs: Vec<_> = plan.layers.iter().filter_map(|l| l.adjust.as_ref()).map(|stack| compose::adjust_run(stack, size)).collect();
    let said = format!("{:?}{:?}", plan.layers, runs);
    let mixed = ["[Mix(", " Mix("].iter().any(|m| said.contains(m));
    (counts.collect(), mixed)
}

#[test]
fn b221_gpu_mix() {
    let out = repo("verification/B-221_gpu_mix_table.md");
    let mut gpu = Gpu::new().expect("a usable card");
    let (mut rows, mut checks, mut passed) = (String::new(), 0, 0);
    let mut by_mix: BTreeMap<String, (usize, usize, u8)> = BTreeMap::new();
    let mut worst: Option<((u8, usize), String, Vec<u8>, Vec<u8>, usize, usize)> = None;
    let shots = shots();
    for shot in &shots {
        let group = match shot.name.rsplit_once(" at Mix ") {
            Some((_, mix)) => format!("every card effect at Mix {mix}"),
            None => shot.name.clone(),
        };
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100] {
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
                let (n, mixed) = left(shot, frame, quality);
                let d = distance(&c, &g);
                let close = !on_cpu && said_cpu == said_gpu && d.0 <= LIMIT;
                // At Full the first layer's whole run is the card's; at Draft a distance the draft
                // cannot take keeps an effect on the CPU (B-107), so only the pictures are held.
                let whole = quality == PreviewQuality::Draft || shot.first_layer.is_none_or(|k| n.first() == Some(&k));
                let planned = quality == PreviewQuality::Draft || mixed == shot.mixed;
                let pass = close && whole && planned;
                checks += 1;
                passed += pass as usize;
                let e = by_mix.entry(group.clone()).or_default();
                e.0 += 1;
                e.1 += pass as usize;
                e.2 = e.2.max(d.0);
                let case = format!("{} frame {frame}, {}", shot.name, quality.label());
                let counts = n.iter().map(|k| k.to_string()).collect::<Vec<_>>().join(" / ");
                let expected = match (quality, shot.first_layer) {
                    (PreviewQuality::Full, Some(k)) => k.to_string(),
                    _ => "—".into(),
                };
                let _ = writeln!(
                    rows,
                    "| {case} | {counts} | {expected} | {} | {} | {} | {} | {} |",
                    if mixed { "yes" } else { "no" },
                    d.0,
                    d.1,
                    match (said_gpu.is_empty(), said_cpu == said_gpu) {
                        (true, true) => "none".to_string(),
                        (false, true) => format!("{said_gpu}, on both"),
                        (_, false) => format!("CPU: {said_cpu}; GPU: {said_gpu}"),
                    },
                    match (on_cpu, close, whole, planned) {
                        (true, ..) => "FAIL: the CPU drew it",
                        (false, false, ..) => "FAIL: the pictures differ",
                        (false, true, false, _) => "FAIL: not the whole run on the card",
                        (false, true, true, false) => "FAIL: the Mix is not the card's",
                        (false, true, true, true) => "PASS",
                    }
                );
                if worst.as_ref().is_none_or(|w| d > w.0) {
                    worst = Some((d, case, c, g, w, h));
                }
            }
        }
    }

    // B-172: a run with mixed members drawn in one pass is, byte for byte, the run drawn with
    // every effect in a pass of its own, each mixed by the Mix pass.
    let mut fused = (0, 0);
    for shot in &shots[shots.len() - 3..shots.len() - 1] {
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100] {
                let draw = |gpu: &mut Gpu| {
                    gpu.forget();
                    let mut log = FrameLog::new(3);
                    preview::preview_frame_srgb8(&shot.project, &shot.comp, frame, &shot.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer(), gpu).expect("a card frame").0
                };
                gpu.fuse(false);
                let apart = draw(&mut gpu);
                gpu.fuse(true);
                let together = draw(&mut gpu);
                let same = apart == together;
                checks += 1;
                passed += same as usize;
                fused.0 += 1;
                fused.1 += same as usize;
                if !same {
                    let _ = writeln!(rows, "| {} frame {frame}, {}: drawn in one pass and in a pass each | — | — | — | — | — | the frames differ | FAIL |", shot.name, quality.label());
                }
            }
        }
    }

    // The CPU drawing a plan made for the card, as it does when the card refuses a frame, draws
    // the same frame as a plan made for the CPU, byte for byte.
    let mut identical = (0, 0);
    for shot in &shots {
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            let mut log = FrameLog::new(3);
            let tile = match quality {
                PreviewQuality::Full | PreviewQuality::Half => DEFAULT_TILE_SIZE,
                PreviewQuality::Draft => compose::DRAFT_TILE_SIZE,
            };
            let plan = |card: bool, log: &mut FrameLog| {
                let make = if card { compose::plan_frame_for_card } else { compose::plan_frame_at };
                let p = make(&shot.project, &shot.comp, 100, &shot.root, quality, log, &mut CelCache::viewer()).expect("plan");
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
                let _ = writeln!(rows, "| {} frame 100, {}: the plan made for the card, drawn by the CPU | — | — | — | — | — | the frame differs | FAIL |", shot.name, quality.label());
            }
        }
    }

    let pictures = repo("verification/B-221 pictures");
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
    for (group, (n, p, most)) in &by_mix {
        let _ = writeln!(summary, "| {group} | {n} | {most} | {p} of {n} |");
    }
    let s = format!(
        "# B-221: effects mixed below 100 on the GPU against the CPU\n\n\
         Written by `tests/b221_gpu_mix.rs`. The card: {}.\n\n\
         An effect's Mix lays its result back over what it was given (D-202). Until B-221 a layer \
         with an effect mixed below 100 was drawn by the CPU from that effect on; now the card \
         keeps what the effect was given, runs the effect and mixes, in one small pass, or inside \
         the pass of a run of colour effects drawn together (B-172) (D-340).\n\n\
         Each of the {} effects the card can draw is put last on the reference shot's first three \
         layers, at Mix 0, 37 and 100. Then three more shots: a run of colour effects drawn in one \
         pass with mixed members, growing effects (Glow, Gaussian Blur, Drop Shadow, Directional \
         Blur, Outline, Lens Blur, Motion Tile) mixed, and an adjustment layer with mixed effects.\n\n\
         Each row compares the eight-bit picture the page receives, drawn by the CPU and by the \
         GPU, at frames 0 and 100, Full and Draft. **The rule: no channel of any pixel more than \
         {LIMIT} level of 255 apart**, the same warnings on both, the card drawing the frame \
         itself, at Full the first layer's whole run on the card, and at Full the card's plan \
         holding the Mix exactly when an effect is mixed below 100.\n\n\
         **{passed} of {checks} checks pass.**\n\n\
         The runs with mixed members drawn in one pass match the same runs drawn a pass each, byte \
         for byte, in {} of {}. The CPU drawing each plan made for the card, as it does when the \
         card refuses a frame, draws the plan made for the CPU byte for byte in {} of {} (frame \
         100, Full and Draft). A failing one is listed below.\n\n\
         The worst comparison is \"{worst_case}\": largest difference {largest} of 255, pixels \
         differing: {count}. Its pictures are in `verification/B-221 pictures/`: `cpu.png`, \
         `gpu.png`, and `difference.png`, black where the two agree and a white 7 by 7 square \
         around every pixel where they do not.\n\n\
         ## By Mix and shot\n\n\
         | Shot | Frames compared | Largest difference (of 255) | Pass |\n|---|---:|---:|---|\n{summary}\n\
         ## Every frame\n\n\
         Effects left to the card on the first three layers, the number the first layer must have \
         at Full, and whether the card's plan holds a Mix.\n\n\
         | Case | Left to the card | First layer must have | Mix on the card | Largest difference (of 255) | Pixels differing | Warnings | Result |\n|---|---:|---:|---|---:|---:|---|---|\n{rows}",
        gpu.about(),
        effects().len(),
        fused.1,
        fused.0,
        identical.1,
        identical.0,
    );
    fs::write(&out, s).expect("write the B-221 table");
    assert_eq!(passed, checks, "B-221: {passed} of {checks} checks pass");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-151's way: the reference shot with a Glow at Mix 50 on its first three
/// layers (the second after a Drop Shadow), then a Noise at Mix 50 that changes every frame, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; then seven loops are timed, the median of their 210 frames
/// "again", in ms. The same test run on the build before B-221 gives the "before" columns, the
/// mixed effect then drawn by the CPU. Writes `B221_OUT`, or `verification/B-221_timing_raw.md`.
#[test]
#[ignore = "B-221: a measurement, run deliberately with --release --ignored"]
fn b221_gpu_mix_timing() {
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
    let on = |name: &str| {
        let x = |id: &str| fx(id, name, 50.0);
        shot(format!("{name} at Mix 50"), [vec![x("x1")], vec![fx("d2", "Drop Shadow", 100.0), x("x2")], vec![x("x3")]], None, None, true)
    };
    for shot in [on("Glow"), on("Noise")] {
        for quality in [PreviewQuality::Draft, PreviewQuality::Full] {
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
            let n = left(&shot, 100, quality).0.iter().map(|k| k.to_string()).collect::<Vec<_>>().join(" / ");
            let _ = writeln!(s, "| {} | {} | {n} | {:.1} | {:.1} |", shot.name, quality.label(), median(first), median(times));
        }
    }
    let out = std::env::var("B221_OUT").map(PathBuf::from).unwrap_or_else(|_| repo("verification/B-221_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
