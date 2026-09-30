//! B-156: an adjustment layer drawn on the card (D-225, GPU plan G5), and the check that it
//! draws as the CPU does. Made from `tests/b155_gpu_chain.rs`, with the same rules.
//!
//! An adjustment layer runs its stack on the whole frame drawn beneath it and mixes the result
//! back by its cover, `B + c*(E(B) - B)` (D-66, `render::adjust_frame`). Until now a frame with
//! one was the CPU's, whole (B-44). Now the card takes the frame it has drawn so far, runs the
//! stack on it as it runs a layer's (D-224), and mixes it back by the layer's cover.
//!
//! Each effect the card draws is put alone on an adjustment layer over the reference shot. Then
//! runs of three: over the whole frame, over part of it (scaled, turned and at 60% opacity, so
//! the cover is between none and all at its edges), and in the middle of the layers, so the
//! layers above it are drawn onto the adjusted frame. Bloom, Glow, Paraffin and Kira-kira look
//! at their drawing before the card is asked, which an adjustment layer's drawing (the frame
//! beneath) is not yet; an HSV Key only begins a run (D-224), and a frame drawn by the card is
//! never the start of one; Kaleidoscope the card does not draw (D-240). An adjustment layer with
//! any of these stays the CPU's, and the frame must then be the CPU's own byte for byte.
//!
//! What is compared is what the page receives, eight-bit straight sRGB: the CPU's frame through
//! `preview_frame_cached`, and the card's through `preview_frame_srgb8`. The rule stays 1 level
//! of 255 (D-165).
//!
//! Writes `verification/B-156_gpu_adjust_table.md` and, for the worst frame, three pictures in
//! `verification/B-156 pictures/`.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::{Diagnostic, DiagnosticId, FrameLog};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::png_out;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::OutputDepth;
use serde_json::{json, Value};

mod common;
use common::repo;

/// D-165's tolerance, in levels of 255.
const LIMIT: u8 = 1;

/// Every effect the card draws, with the settings its own card check gives it.
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

/// Of those, the ones an adjustment layer keeps on the CPU: the four that look at their drawing
/// before the card is asked, and HSV Key, which only begins a run (D-224).
const CPU_ONLY: [&str; 5] = ["Bloom", "Glow", "Paraffin", "Kira-kira", "HSV Key"];

/// Where the adjustment layer is and what it covers.
#[derive(Clone, Copy)]
enum Cover {
    /// Over every layer, the whole frame.
    Whole,
    /// Over every layer, scaled, turned and at 60% opacity.
    Part,
    /// Between the second and third layers, scaled, turned and at 60% opacity.
    Middle,
}

struct Shot {
    name: String,
    project: Project,
    root: PathBuf,
    comp: Id,
    /// Whether the card is to draw it.
    card: bool,
}

fn fx(id: &str, type_id: &str, parameters: &Value) -> Value {
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": parameters})
}

fn kaleidoscope() -> Value {
    json!({"instance_id": "b156-k", "type_id": "core.kaleidoscope", "enabled": true, "parameters": {"segments": 6, "rotation": 0, "size": 100, "center": [50, 50], "mode": "mirror"}})
}

/// The reference shot with an adjustment layer running `stack`.
fn shot(name: String, stack: Vec<Value>, cover: Cover, card: bool) -> Shot {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: Value = serde_json::from_str(&text).expect("the reference shot is JSON");
    let (scale, rotation, opacity) = match cover {
        Cover::Whole => (100, 0, 1.0),
        Cover::Part | Cover::Middle => (60, 12, 0.6),
    };
    let key = |v: Value| json!({"base": v, "keyframes": []});
    let adjustment = json!({
        "id": "b156-adj",
        "kind": "adjustment",
        "name": "b156 adjustment",
        "enabled": true,
        "locked": false,
        "in_frame": 0,
        "out_frame": 240,
        "transform": {
            "anchor": key(json!([960, 540])),
            "position": key(json!([900, 500])),
            "scale": key(json!([scale, scale + 10])),
            "rotation": key(json!(rotation)),
            "opacity": key(json!(opacity)),
        },
        "matte": null,
        "blend_mode": "normal",
        "effects": stack,
    });
    let comp = &mut j["compositions"][0];
    let at = match cover {
        Cover::Middle => 2,
        _ => comp["layers"].as_array().expect("the reference shot's layers").len(),
    };
    comp["layers"].as_array_mut().expect("the reference shot's layers").insert(at, adjustment);
    comp["layer_order"].as_array_mut().expect("the reference shot's layer order").insert(at, json!("b156-adj"));
    // Color Lookup's file, from its own fixtures.
    j["assets"].as_array_mut().expect("the reference shot's assets").push(json!({"id": "asset-lut", "kind": "lut", "name": "warm_17", "path": "../cube_lut/luts/warm_17.cube"}));
    let loaded = persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("{name}: {}", d.message));
    Shot {
        name,
        project: loaded.document.project().clone(),
        root: repo("Fixtures/reference_shot"),
        comp: Id::new("comp-reference-shot"),
        card,
    }
}

fn shots() -> Vec<Shot> {
    let all = effects();
    let get = |name: &str| all.iter().find(|e| e.0 == name).map(|e| fx(&format!("b156-{name}"), e.1, &e.2)).expect("a named effect");
    let mut shots = Vec::new();
    for (name, type_id, p) in &all {
        let card = !CPU_ONLY.contains(name);
        shots.push(shot(format!("{name} alone, over the whole frame"), vec![fx("b156-a", type_id, p)], Cover::Whole, card));
    }
    let three = || vec![get("Levels"), get("Gaussian Blur"), get("Hue/Saturation")];
    shots.push(shot("Levels, Gaussian Blur and Hue/Saturation, over the whole frame".into(), three(), Cover::Whole, true));
    shots.push(shot("Levels, Gaussian Blur and Hue/Saturation, over part of it".into(), three(), Cover::Part, true));
    shots.push(shot("Levels, Gaussian Blur and Hue/Saturation, between the second and third layers".into(), three(), Cover::Middle, true));
    shots.push(shot(
        "Camera Shake, Motion Tile and Corner Pin, which grow by the frame's size, over part of it".into(),
        vec![get("Camera Shake"), get("Motion Tile"), get("Corner Pin")],
        Cover::Part,
        true,
    ));
    shots.push(shot(
        "Directional Blur, Curves and a moving Noise, between the second and third layers".into(),
        vec![get("Directional Blur"), get("Curves"), get("Noise")],
        Cover::Middle,
        true,
    ));
    shots.push(shot("Kaleidoscope alone, over the whole frame".into(), vec![kaleidoscope()], Cover::Whole, false));
    shots.push(shot("Levels, then Kaleidoscope, then Curves, over part of it".into(), vec![get("Levels"), kaleidoscope(), get("Curves")], Cover::Part, false));
    shots.push(shot("Levels, then a Glow, over part of it".into(), vec![get("Levels"), get("Glow")], Cover::Part, false));
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

#[test]
fn b156_gpu_adjust() {
    let out = repo("verification/B-156_gpu_adjust_table.md");
    let mut gpu = match Gpu::new() {
        Ok(gpu) => gpu,
        Err(why) => {
            fs::write(&out, format!("# B-156: adjustment layers on the GPU\n\n**NOT RUN.** No usable card: {why}\n\nNo check in this table was run, so none of them passes.\n"))
                .expect("write the B-156 table");
            return;
        }
    };
    let (mut rows, mut checks, mut passed) = (String::new(), 0, 0);
    let mut by_kind: BTreeMap<&str, (usize, usize, u8)> = BTreeMap::new();
    let mut worst: Option<((u8, usize), String, Vec<u8>, Vec<u8>, usize, usize)> = None;
    for shot in &shots() {
        let kind = match (shot.card, shot.name.contains(" alone,")) {
            (true, true) => "one effect the card draws",
            (true, false) => "a run of three the card draws",
            (false, _) => "kept on the CPU",
        };
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100] {
                let mut cache = CelCache::viewer();
                let mut log = FrameLog::new(3);
                let c = preview::preview_frame_cached(&shot.project, &shot.comp, frame, &shot.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache)
                    .unwrap_or_else(|d| panic!("{} frame {frame} on the CPU: {}", shot.name, d.message));
                let (w, h) = (c.width(), c.height());
                let said = |log: &[Diagnostic]| {
                    let mut ids: Vec<&str> = log.iter().map(|d| d.id.as_str()).filter(|id| *id != DiagnosticId::GpuPreviewOnCpu.as_str()).collect();
                    ids.sort();
                    ids.dedup();
                    ids.join(", ")
                };
                let said_cpu = said(&log.finish());
                let c = c.to_srgb8_straight();
                let mut log = FrameLog::new(3);
                let (g, ..) = preview::preview_frame_srgb8(&shot.project, &shot.comp, frame, &shot.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu)
                    .unwrap_or_else(|d| panic!("{} frame {frame} on the GPU: {}", shot.name, d.message));
                let log = log.finish();
                let on_cpu = log.iter().any(|d| d.id == DiagnosticId::GpuPreviewOnCpu);
                let said_gpu = said(&log);
                let d = distance(&c, &g);
                let same_words = said_cpu == said_gpu;
                // On the card: within the tolerance. Kept on the CPU: the CPU's frame exactly.
                let pass = same_words && on_cpu != shot.card && if shot.card { d.0 <= LIMIT } else { d.0 == 0 };
                checks += 1;
                passed += pass as usize;
                let e = by_kind.entry(kind).or_default();
                e.0 += 1;
                e.1 += pass as usize;
                e.2 = e.2.max(d.0);
                let case = format!("{} frame {frame}, {}", shot.name, quality.label());
                let _ = writeln!(
                    rows,
                    "| {case} | {} | {} | {} | {} | {} | {} |",
                    if shot.card { "GPU" } else { "CPU" },
                    if on_cpu { "CPU" } else { "GPU" },
                    d.0,
                    d.1,
                    match (said_gpu.is_empty(), same_words) {
                        (true, true) => "none".to_string(),
                        (false, true) => format!("{said_gpu}, on both"),
                        (_, false) => format!("CPU: {said_cpu}; GPU: {said_gpu}"),
                    },
                    match (pass, on_cpu == shot.card, same_words) {
                        (true, ..) => "PASS",
                        (false, true, _) if shot.card => "FAIL: the CPU drew it",
                        (false, true, _) => "FAIL: the card drew it",
                        (false, false, false) => "FAIL: different warnings",
                        (false, false, true) => "FAIL: the pictures differ",
                    }
                );
                if shot.card && worst.as_ref().is_none_or(|w| d > w.0) {
                    worst = Some((d, case, c, g, w, h));
                }
            }
        }
    }

    let pictures = repo("verification/B-156 pictures");
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
    for (kind, (n, p, most)) in &by_kind {
        let _ = writeln!(summary, "| {kind} | {n} | {most} | {p} of {n} |");
    }
    let s = format!(
        "# B-156: adjustment layers on the GPU against the CPU\n\n\
         Written by `tests/b156_gpu_adjust.rs`. The card: {}.\n\n\
         An adjustment layer runs its effects on the whole frame drawn beneath it and mixes the \
         result back by what it covers (D-66). Each of the {} effects the card can draw is put \
         alone on an adjustment layer over the reference shot. Then runs of three: over the \
         whole frame, over part of it (scaled, turned and at 60% opacity, so its edges cover \
         part of a pixel), and between the second and third layers, so the two above it are \
         drawn onto the adjusted frame. Bloom, Glow, Paraffin and Kira-kira look at their \
         drawing before the card is asked, and an HSV Key only begins a run (D-224), so an \
         adjustment layer with one stays the CPU's; so does one with a Kaleidoscope, which the \
         card does not draw (D-240).\n\n\
         Each row compares the eight-bit picture the page receives, drawn by the CPU and by the \
         GPU. **The rule: drawn where the table says; on the card no channel of any pixel more \
         than {LIMIT} level of 255 apart; kept on the CPU the CPU's frame byte for byte**; the \
         same warnings on both.\n\n\
         **{passed} of {checks} checks pass.**\n\n\
         The worst comparison drawn on the card is \"{worst_case}\": largest difference {largest} of 255, pixels differing: {count}. \
         Its pictures are in `verification/B-156 pictures/`: `cpu.png`, `gpu.png`, and \
         `difference.png`, black where the two agree and a white 7 by 7 square around every pixel \
         where they do not.\n\n\
         ## By kind\n\n\
         | Kind | Frames compared | Largest difference (of 255) | Pass |\n|---|---:|---:|---|\n{summary}\n\
         ## Every frame\n\n\
         | Case | Must be drawn on | Drawn on | Largest difference (of 255) | Pixels differing | Warnings | Result |\n|---|---|---|---:|---:|---|---|\n{rows}",
        gpu.about(),
        effects().len() - CPU_ONLY.len(),
    );
    fs::write(&out, s).expect("write the B-156 table");
    assert_eq!(passed, checks, "B-156: {passed} of {checks} checks pass");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times on the card of the reference shot under an adjustment layer: a still run of
/// three over the whole frame, then the same over part of it, then a run beginning with a Noise
/// that changes every frame. Every eighth frame is asked for as the viewer asks; one loop fills
/// the caches, then seven are timed; the median of their 210 frames, in ms. The same test run
/// on the checks-first build gives the "before" column, when the CPU drew such a frame whole.
#[test]
#[ignore]
fn b156_gpu_adjust_timing() {
    let mut gpu = Gpu::new().expect("a usable card");
    let all = effects();
    let get = |name: &str| all.iter().find(|e| e.0 == name).map(|e| fx(&format!("b156-{name}"), e.1, &e.2)).expect("a named effect");
    let three = || vec![get("Levels"), get("Gaussian Blur"), get("Hue/Saturation")];
    let timed = [
        shot("Levels, Gaussian Blur and Hue/Saturation, over the whole frame".into(), three(), Cover::Whole, true),
        shot("the same, over part of it".into(), three(), Cover::Part, true),
        shot("a moving Noise, then the same, over the whole frame".into(), vec![get("Noise"), get("Levels"), get("Gaussian Blur"), get("Hue/Saturation")], Cover::Whole, true),
    ];
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n| Shot | Quality | Drawn on | GPU median ms |\n|---|---|---|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    for shot in timed {
        for quality in [PreviewQuality::Draft, PreviewQuality::Full] {
            let mut cache = CelCache::viewer();
            gpu.forget();
            let mut times = Vec::new();
            let mut on_cpu = false;
            for pass in 0..8 {
                for frame in (0..240).step_by(8) {
                    let mut log = FrameLog::new(3);
                    let t = std::time::Instant::now();
                    drop(preview::preview_frame_srgb8(&shot.project, &shot.comp, frame, &shot.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).expect("GPU frame"));
                    if pass > 0 {
                        times.push(t.elapsed().as_secs_f64() * 1000.0);
                    }
                    on_cpu |= log.finish().iter().any(|d| d.id == DiagnosticId::GpuPreviewOnCpu);
                }
            }
            let _ = writeln!(s, "| {} | {} | {} | {:.1} |", shot.name, quality.label(), if on_cpu { "CPU" } else { "GPU" }, median(times));
        }
    }
    fs::write(repo("verification/B-156_timing_raw.md"), s).expect("write the timing table");
}
