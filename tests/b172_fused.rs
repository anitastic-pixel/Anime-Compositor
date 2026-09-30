//! B-172: colour effects next to each other on a layer, each of which reads only its own pixel,
//! drawn on the card in one pass rather than one pass each: the same maths in the same order, the
//! pixel handed from one to the next as the texture between them held it (GPU plan G13, D-244).
//!
//! Three shots of the reference shot, every such effect the card draws put in runs on its first
//! three layers: runs of five to eight; runs broken by blurs, an Offset and a Drop Shadow; and
//! runs beginning with the two that only begin a run (Paraffin, HSV Key). At Full and Draft, five
//! frames each, drawn from nothing (the card and the cache emptied first):
//!
//! 1. with runs drawn in one pass, the frame the viewer shows is, byte for byte, the frame drawn
//!    with every effect in a pass of its own;
//! 2. and the card runs fewer passes: every run of two or more is one.
//!
//! Writes `verification/B-172_fused_table.md`.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};
use serde_json::{json, Value};

mod common;
use common::repo;

fn effect(name: &str) -> Value {
    let (type_id, parameters) = match name {
        "Curves" => ("core.curves", json!({"master": [[0, 0], [128, 150], [255, 255]], "red": [[0, 0], [64, 40], [192, 215], [255, 255]], "green": [[0, 0], [255, 255]], "blue": [[0, 20], [255, 235]]})),
        "Levels" => ("core.levels", json!({"input_black": 20, "input_white": 230, "gamma": 1.6, "output_black": 10, "output_white": 245})),
        "Hue/Saturation" => ("core.hue_saturation", json!({"hue": 40, "saturation": 30, "lightness": -10})),
        "Gradient" => ("core.gradient", json!({"shape": "radial", "start": [50, 50], "end": [100, 100], "start_color": "#ffcc00", "end_color": "#3050ff", "start_opacity": 80, "end_opacity": 40, "blend": "multiply"})),
        "Noise" => ("core.noise", json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"})),
        "Exposure Flicker" => ("core.exposure_flicker", json!({"amount": 0.5, "hold": 3, "seed": 4})),
        "Color Balance" => ("core.color_balance", json!({"shadows": [-20, 10, 30], "midtones": [15, -5, -10], "highlights": [10, 5, -25]})),
        "Gradient Map" => ("core.gradient_map", json!({"shadow_color": "#1a0a40", "midtone_color": "#c05060", "highlight_color": "#fff0c0", "midpoint": 40, "amount": 80})),
        "Vignette" => ("core.vignette", json!({"amount": 60, "color": "#201030", "size": 90, "roundness": 50, "softness": 60, "center": [45, 55]})),
        "Fractal Noise" => ("core.fractal_noise", json!({"size": 80, "complexity": 5, "contrast": 130, "brightness": 5, "evolution": 0, "speed": 15, "seed": 3, "dark_color": "#102030", "light_color": "#ffe8c0", "opacity": 60, "blend": "screen"})),
        "Invert" => ("core.invert", json!({"channel": "rgb", "amount": 80})),
        "Invert Alpha" => ("core.invert", json!({"channel": "alpha", "amount": 30})),
        "Brightness & Contrast" => ("core.brightness_contrast", json!({"brightness": 30, "contrast": 40})),
        "Black & White" => ("core.black_white", json!({"reds": 120, "yellows": 110, "greens": -10, "cyans": -50, "blues": -50, "magentas": 120})),
        "Posterize" => ("core.posterize", json!({"levels": 6})),
        "Threshold" => ("core.threshold", json!({"level": 128})),
        "Channel Mixer" => ("core.channel_mixer", json!({"red": [0, 0, 100, 0], "green": [0, 100, 0, 0], "blue": [100, 0, 0, 0], "monochrome": "off"})),
        "Vibrance" => ("core.vibrance", json!({"vibrance": 40, "saturation": 20})),
        "Leave Color" => ("core.leave_color", json!({"color": "#ff0000", "tolerance": 15, "softness": 10, "amount": 100})),
        "Solarize" => ("core.solarize", json!({"threshold": 128})),
        "Halftone" => ("core.halftone", json!({"size": 8, "angle": 45, "ink": "#000000", "paper": "#ffffff", "amount": 100})),
        "Color Lookup" => ("core.color_lookup", json!({"lut": "asset-lut"})),
        "HSV Key" => ("core.hsv_key", json!({"hue": 30, "hue_range": 40, "saturation": 50, "saturation_range": 50, "value": 60, "value_range": 40, "invert": "off"})),
        "Paraffin" => ("core.paraffin", json!({"color": "#ffc890", "direction": 45, "spread": 60, "opacity": 70, "blend": "overlay"})),
        "Gaussian Blur" => ("core.gaussian_blur", json!({"sigma_px": 4})),
        "Directional Blur" => ("core.directional_blur", json!({"direction": 45, "length": 30})),
        "Offset" => ("core.offset", json!({"shift": [37.5, -21.25]})),
        "Drop Shadow" => ("core.drop_shadow", json!({"color": "#102040", "opacity": 70, "direction": 135, "distance": 12, "softness": 9})),
        _ => panic!("no settings for {name}"),
    };
    json!({"instance_id": format!("b172-{name}"), "type_id": type_id, "enabled": true, "parameters": parameters})
}

/// The reference shot with `stacks` on its first three layers.
fn shot(stacks: [&[&str]; 3]) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: Value = serde_json::from_str(&text).expect("the reference shot is JSON");
    for (i, stack) in stacks.into_iter().enumerate() {
        j["compositions"][0]["layers"][i]["effects"] = stack.iter().map(|n| effect(n)).collect();
    }
    // Color Lookup's file, from its own fixtures.
    j["assets"].as_array_mut().expect("the reference shot's assets").push(json!({"id": "asset-lut", "kind": "lut", "name": "warm_17", "path": "../cube_lut/luts/warm_17.cube"}));
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("{}", d.message)).document.project().clone()
}

fn shots() -> Vec<(&'static str, Project)> {
    vec![
        (
            "every colour effect, in runs of eight, eight and five",
            shot([
                &["Curves", "Levels", "Hue/Saturation", "Gradient", "Noise", "Exposure Flicker", "Color Balance", "Gradient Map"],
                &["Vignette", "Fractal Noise", "Invert", "Brightness & Contrast", "Black & White", "Posterize", "Channel Mixer", "Vibrance"],
                &["Leave Color", "Solarize", "Halftone", "Color Lookup", "Threshold"],
            ]),
        ),
        (
            "runs broken by a blur, an Offset and a Drop Shadow",
            shot([
                &["Levels", "Hue/Saturation", "Gaussian Blur", "Vibrance", "Channel Mixer", "Noise"],
                &["Invert", "Directional Blur", "Curves", "Color Balance", "Offset", "Posterize", "Invert Alpha"],
                &["Drop Shadow", "Hue/Saturation", "Brightness & Contrast", "Gradient"],
            ]),
        ),
        (
            "runs beginning with Paraffin and HSV Key",
            shot([
                &["Paraffin", "Gradient Map", "Vibrance", "Solarize"],
                &["HSV Key", "Levels", "Noise"],
                &["Exposure Flicker", "Leave Color", "Fractal Noise"],
            ]),
        ),
    ]
}

/// One frame drawn from nothing: the bytes the viewer shows and the passes the card ran.
fn frame(gpu: &mut Gpu, project: &Project, root: &PathBuf, n: i32, quality: PreviewQuality) -> (Vec<u8>, u64) {
    gpu.forget();
    let mut cache = CelCache::viewer();
    let mut log = FrameLog::new(3);
    let before = gpu.dispatched();
    let (bytes, ..) = preview::preview_frame_srgb8(project, &Id::new("comp-reference-shot"), n, root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, gpu).expect("a card frame");
    (bytes, gpu.dispatched() - before)
}

#[test]
fn b172_fused() {
    let mut gpu = Gpu::new().expect("a usable card");
    let root = repo("Fixtures/reference_shot");
    let mut s = String::from(
        "# B-172: runs of colour effects drawn in one pass on the card\n\n\
         Written by `tests/b172_fused.rs`. Each frame is drawn from nothing twice on the card: once with\n\
         every effect in a pass of its own, as before, and once with each run of colour effects next to\n\
         each other drawn in one pass. \"Same\" means every byte the viewer shows is the same.\n\n\
         | Shot | Quality | Frame | Passes, one each | Passes, runs in one | Same |\n|---|---|---:|---:|---:|---|\n",
    );
    let (mut total, mut same, mut fewer) = (0, 0, 0);
    for (name, project) in shots() {
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for n in [0, 50, 101, 173, 239] {
                gpu.fuse(false);
                let (apart, passes_apart) = frame(&mut gpu, &project, &root, n, quality);
                gpu.fuse(true);
                let (together, passes_together) = frame(&mut gpu, &project, &root, n, quality);
                let ok = apart == together;
                total += 1;
                same += ok as usize;
                fewer += (passes_together < passes_apart) as usize;
                let _ = writeln!(s, "| {name} | {} | {n} | {passes_apart} | {passes_together} | {} |", quality.label(), if ok { "yes" } else { "NO" });
            }
        }
    }
    let _ = write!(s, "\n**{same} of {total} frames the same; {fewer} of {total} drawn in fewer passes.**\n");
    fs::write(repo("verification/B-172_fused_table.md"), s).expect("write the table");
    assert_eq!(same, total, "a frame drawn with runs in one pass differs");
    assert_eq!(fewer, total, "a frame with runs of colour effects was not drawn in fewer passes");
}
