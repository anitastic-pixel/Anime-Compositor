//! B-835: a Half preview builds the layer an effect reads as a map (Displacement Map here) at
//! Half's own size. It used to take Half for Draft and build the map at a quarter, so the
//! displaced picture at Half was coarser than Half should be. The check: the Half picture is
//! near the Full picture shrunk by two, nearer than the old quarter-size map left it.

use std::fs;

use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};
use serde_json::{json, Value};

mod common;
use common::repo;

fn frame(project: &anime_compositor::model::Project, quality: PreviewQuality) -> (usize, usize, Vec<u8>) {
    let mut log = FrameLog::new(3);
    let b = preview::preview_frame(
        project,
        &Id::new("comp-reference-shot"),
        100,
        &repo("Fixtures/reference_shot"),
        quality,
        DEFAULT_TILE_SIZE,
        &mut log,
    )
    .unwrap_or_else(|d| panic!("frame 100 at {quality:?}: {}", d.message));
    (b.width(), b.height(), b.to_srgb8_straight())
}

#[test]
fn b835_half_builds_its_maps_at_half() {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: Value = serde_json::from_str(&text).expect("the reference shot is JSON");
    let dm = json!({"instance_id": "b835", "type_id": "core.displacement_map", "enabled": true, "parameters":
        {"layer": "layer-4", "fit": "stretch", "horizontal": "red", "max_horizontal": 12.0, "vertical": "green", "max_vertical": 12.0, "wrap": "off", "expand": "off"}});
    j["compositions"][0]["layers"][0]["effects"] = json!([dm]);
    let project = persist::load_str(&j.to_string()).expect("loads").document.project().clone();

    let (fw, fh, full) = frame(&project, PreviewQuality::Full);
    let (hw, hh, half) = frame(&project, PreviewQuality::Half);
    // Full shrunk by two, a plain 2 by 2 average, beside Half pixel for pixel.
    let (w, h) = (hw.min(fw / 2), hh.min(fh / 2));
    let mut sum = 0.0;
    for y in 0..h {
        for x in 0..w {
            for c in 0..4 {
                let f = |dx: usize, dy: usize| full[((2 * y + dy) * fw + 2 * x + dx) * 4 + c] as f64;
                let shrunk = (f(0, 0) + f(1, 0) + f(0, 1) + f(1, 1)) / 4.0;
                sum += (shrunk - half[(y * hw + x) * 4 + c] as f64).abs();
            }
        }
    }
    let mean = sum / (w * h * 4) as f64;
    println!("B-835: Half {hw}x{hh} against Full {fw}x{fh} shrunk by two: mean difference {mean:.3} levels of 255");
    assert!(mean < BAR, "Half is {mean:.3} levels from Full shrunk by two, the bar is {BAR}");
}

/// Measured 2026-11-03: 0.195 with the fix, 0.208 with the quarter-size map (the reference
/// shot's map is smooth, so the gap is small). Set between the two: it fails without the fix.
const BAR: f64 = 0.2;
