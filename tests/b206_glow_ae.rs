//! B-206: D-322, Glow in After Effects' own numbers.
//!
//! Writes `verification/D-322_glow_ae_table.md`.
//!
//! Every expected pixel is `Fixtures/glow_ae/expected_glow_ae.json`, written by
//! `tools/glow_ae_reference.py` before this code existed and printed in document 25 as
//! FX-GLOW-AE-001 to 010. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also glows a white square the old way and the new into `verification/D-322 pictures/`,
//! over black.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{set, Table};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

fn glow(units: &str) -> Effect {
    Effect::Glow {
        based_on: "bright".into(),
        threshold: 60.0,
        colors: Vec::new(),
        tolerance: 0.0,
        radius: 10.0,
        intensity: 1.0,
        operation: "add".into(),
        tint: String::new(),
        units: units.into(),
    }
}

/// A 16:9 plate holding a white square 24 pixels across in its middle, clear round it.
const PLATE: (usize, usize) = (192, 108);

fn square() -> Vec<u8> {
    let (w, h) = PLATE;
    (0..w * h)
        .flat_map(|i| {
            let (x, y) = (i % w, i / w);
            if (84..108).contains(&x) && (42..66).contains(&y) { [255; 4] } else { [0; 4] }
        })
        .collect()
}

/// The square with one Glow of `settings` (After Effects' defaults for the rest), in a Float
/// composition when `float`, over black, 8-bit.
fn picture(dir: &Path, settings: J, float: bool) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/glow_ae/fx_glow_ae_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("square.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(PLATE.0);
    comp["height"] = J::from(PLATE.1);
    comp["float_depth"] = J::from(float);
    let layer = &mut comp["layers"][0];
    let middle = json!([PLATE.0 as f64 / 2.0, PLATE.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    let parameters = &mut layer["effects"][0]["parameters"];
    parameters.as_object_mut().unwrap().remove("units");
    for (k, v) in settings.as_object().unwrap() {
        parameters[k] = v.clone();
    }
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(effect_table::MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    // Over black in linear light: the premultiplied colour itself, held to white.
    let black = frame
        .data()
        .chunks(4)
        .flat_map(|p| {
            let over = |c: f32| (anime_compositor::color::linear_to_srgb(c.clamp(0.0, 1.0)) * 255.0).round() as u8;
            [over(p[0]), over(p[1]), over(p[2]), 255]
        })
        .collect();
    (black, said)
}

#[test]
fn b206_glow_ae() {
    let mut t = Table::new(
        "glow_ae",
        "# D-322: Glow in After Effects' own numbers\n\nFrom D-308, held back for want of an \
         After Effects frame, then settled on 2026-10-04 from the Creative COW thread \"Glow \
         Effect and transparent background mechanics\", where After Effects' Glow was measured on \
         white shapes: the radius is a Gaussian Blur (here Blurriness, D-321), and the glow's \
         brightness is GI*(GT/100) + GI*16*(1-GT/100), its covering untouched by intensity. Glow \
         gains Units: After Effects, which a new one takes, and Classic, what a file without \
         units means, D-89's rule as before. Every expected pixel is \
         `Fixtures/glow_ae/expected_glow_ae.json`, written by `tools/glow_ae_reference.py` before \
         the build had it, printed in document 25 as FX-GLOW-AE-001 to 010. Tolerance 2e-5.\n",
    );

    t.heading("FX-GLOW-AE-001 to 010 (document 25)");
    t.fixtures("expected_glow_ae.json");

    t.heading("The file");
    t.round_trips(&["fx_glow_ae_001.json", "fx_glow_ae_002.json", "fx_glow_ae_003.json", "fx_glow_ae_004.json", "fx_glow_ae_009.json"]);
    let saved = t.saved_parameters("fx_glow_ae_002.json");
    t.row("fx_glow_ae_002.json, a glow with no units as before D-322, is saved without units", &saved.to_string(), saved.get("units").is_none());
    let saved = t.saved_parameters("fx_glow_ae_001.json");
    t.row("A glow in After Effects units is saved as units: after_effects", &saved.to_string(), saved["units"] == "after_effects");

    t.heading("Commands");
    let mut document = t.load("fx_glow_ae_002.json").document;
    t.refused(
        &mut document,
        vec![("units \"ae\"", set(glow("ae"))), ("units \"After_Effects\", written with capitals", set(glow("After_Effects")))],
    );
    t.taken(&mut document, "fx_glow_ae_002.json", vec![("FX-GLOW-AE-002 set to After Effects units,", set(glow("after_effects")))]);
    let mut document = t.load("fx_glow_ae_002.json").document;
    document.apply(set(glow("after_effects"))).unwrap();
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_glow_ae.json")).unwrap()).unwrap();
    let d = effect_table::largest_difference(&t.render(&document, 0, 64), &expected["cases"]["FX-GLOW-AE-001"]["frames"]["0"]);
    t.row("FX-GLOW-AE-002 set to After Effects units by the command draws FX-GLOW-AE-001's frame", &format!("largest difference {d:.1e}"), d <= 2e-5);

    t.heading("The preview card");
    match Gpu::new() {
        Err(why) => t.row("The card draws the glow", &format!("NOT RUN: no usable card ({why})"), false),
        Ok(mut gpu) => {
            for name in ["fx_glow_ae_001.json", "fx_glow_ae_002.json", "fx_glow_ae_004.json", "fx_glow_ae_006.json", "fx_glow_ae_007.json", "fx_glow_ae_008.json"] {
                let project = t.load(name).document.project().clone();
                let comp = Id::new("comp-main");
                let mut cache = CelCache::viewer();
                let mut log = FrameLog::new(3);
                let cpu = preview::preview_frame_cached(&project, &comp, 0, &t.root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache).unwrap().to_srgb8_straight();
                let mut log = FrameLog::new(3);
                let (card, ..) = preview::preview_frame_srgb8(&project, &comp, 0, &t.root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).unwrap();
                let on_cpu = log.finish().iter().any(|d| d.id.as_str() == DiagnosticId::GpuPreviewOnCpu.as_str());
                let worst = cpu.iter().zip(&card).map(|(a, b)| a.abs_diff(*b)).max().unwrap_or(0);
                t.row(
                    &format!("{name}: drawn on the card, within 1 level of 255 of the CPU"),
                    &format!("{}, largest difference {worst} of 255", if on_cpu { "CPU" } else { "card" }),
                    !on_cpu && worst <= 1,
                );
            }
        }
    }

    t.heading("Pictures: a white square, in `verification/D-322 pictures/`, over black");
    let dir = effect_table::repo("verification/D-322 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], bytes).unwrap();
    write("square.png", &square());
    // Along the middle row, 6 and 20 pixels right of the square.
    let row = |bytes: &[u8]| {
        let at = |x: usize| bytes[((h / 2) * w + x) * 4];
        (at(113), at(127))
    };
    let mut near = Vec::new();
    for (name, settings, float, what) in [
        ("1_square.png", json!({ "intensity": 0 }), false, "the square with no glow"),
        ("2_old_glow_defaults.png", json!({}), false, "Glow with After Effects' defaults (threshold 60, radius 10, intensity 1) in an older project (Classic)"),
        ("3_ae_glow_defaults.png", json!({ "units": "after_effects" }), false, "the same in After Effects units: a tighter, far brighter glow"),
        ("4_old_tutorial_glow.png", json!({ "threshold": 0, "radius": 39, "intensity": 0.1 }), true, "tutorial 2's first Glow (threshold 0, radius 39, intensity 0.1) in Float, Classic: barely there"),
        ("5_ae_tutorial_glow.png", json!({ "threshold": 0, "radius": 39, "intensity": 0.1, "units": "after_effects" }), true, "the same in After Effects units: a clear soft glow, 1.6 times the light"),
    ] {
        let (bytes, said) = picture(&dir, settings, float);
        write(name, &bytes);
        let (six, twenty) = row(&bytes);
        near.push((six, twenty));
        t.row(&format!("{name}, {what}; draws cleanly"), &format!("{said:?}, 6 pixels right of the square {six} of 255, 20 pixels right {twenty}"), said.is_empty());
    }
    t.row(
        "After Effects units glow brighter near the square than Classic at the same settings",
        &format!("defaults {} against {}; tutorial {} against {}", near[2].0, near[1].0, near[4].0, near[3].0),
        near[2].0 > near[1].0 + 40 && near[4].0 > near[3].0 + 40,
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_glow_ae_001.json", 0), ("fx_glow_ae_004.json", 0), ("fx_glow_ae_008.json", 0)]);

    t.finish("D-322_glow_ae_table.md");
}
