//! B-205: D-321, Gaussian Blur in After Effects' Blurriness.
//!
//! Writes `verification/D-321_blurriness_table.md`.
//!
//! Every expected pixel is `Fixtures/blurriness/expected_blurriness.json`, written by
//! `tools/blurriness_reference.py` before this code existed and printed in document 25 as
//! FX-BLURRY-001 to 009. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also blurs a white square the old way and the new, plain and with Exposure +20 after it in
//! a Float composition, into `verification/D-321 pictures/`, over black.

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

fn blur(number: f64, units: &str) -> Effect {
    Effect::GaussianBlur { sigma_px: number, edges: "transparent".into(), dimensions: "both".into(), units: units.into() }
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

/// The square with `effects`, in a Float composition when `float`, over black, 8-bit.
fn picture(dir: &Path, effects: J, float: bool) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/blurriness/fx_blurry_007.json")).unwrap(),
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
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(effect_table::MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    // Over black in linear light: the premultiplied colour itself, held to white, so a Float
    // value past 1 shows as white rather than being lost with the straight colour's clip.
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

fn stack(number: f64, units: &str, stops: Option<f64>) -> J {
    let mut parameters = json!({ "sigma_px": number });
    if !units.is_empty() {
        parameters["units"] = J::from(units);
    }
    let mut effects = vec![json!({ "instance_id": "fx-0-0", "type_id": "core.gaussian_blur", "enabled": true, "parameters": parameters })];
    if let Some(stops) = stops {
        effects.push(json!({ "instance_id": "fx-0-1", "type_id": "core.exposure", "enabled": true, "parameters": { "stops": stops } }));
    }
    J::from(effects)
}

#[test]
fn b205_blurriness() {
    let mut t = Table::new(
        "blurriness",
        "# D-321: Gaussian Blur in After Effects' Blurriness\n\nFrom D-308, held back for want of \
         an After Effects frame, then settled on 2026-10-04 from two players of After Effects' \
         own files: lottie-web (\"Empirical value, matching AE's blur appearance\", 0.3) and Skia's \
         Skottie (\"Close-enough to AE\", 0.3). Gaussian Blur gains Units: Blurriness, which a new \
         one takes, is After Effects' number, sigma 0.3 times it, its kernel reaching 6.5 sigmas \
         so a strong Exposure after it finds no box; Sigma, what a file without units means, is \
         document 21's rule as before. Every expected pixel is \
         `Fixtures/blurriness/expected_blurriness.json`, written by \
         `tools/blurriness_reference.py` before the build had Blurriness, printed in document 25 \
         as FX-BLURRY-001 to 009. Tolerance 2e-5.\n",
    );

    t.heading("FX-BLURRY-001 to 009 (document 25)");
    t.fixtures("expected_blurriness.json");

    t.heading("The file");
    t.round_trips(&["fx_blurry_001.json", "fx_blurry_002.json", "fx_blurry_003.json", "fx_blurry_007.json", "fx_blurry_008.json"]);
    let saved = t.saved_parameters("fx_blurry_002.json");
    t.row("fx_blurry_002.json, a blur with no units as before D-321, is saved without units", &saved.to_string(), saved.get("units").is_none());
    let saved = t.saved_parameters("fx_blurry_001.json");
    t.row("A blur in Blurriness is saved as units: blurriness", &saved.to_string(), saved["units"] == "blurriness");

    t.heading("Commands");
    let mut document = t.load("fx_blurry_002.json").document;
    t.refused(
        &mut document,
        vec![
            ("units \"pixels\"", set(blur(10.0, "pixels"))),
            ("units \"Blurriness\", written with a capital", set(blur(10.0, "Blurriness"))),
        ],
    );
    t.taken(&mut document, "fx_blurry_002.json", vec![("FX-BLURRY-002 set to Blurriness,", set(blur(10.0, "blurriness")))]);
    let mut document = t.load("fx_blurry_002.json").document;
    document.apply(set(blur(10.0, "blurriness"))).unwrap();
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_blurriness.json")).unwrap()).unwrap();
    let d = effect_table::largest_difference(&t.render(&document, 0, 64), &expected["cases"]["FX-BLURRY-001"]["frames"]["0"]);
    t.row("FX-BLURRY-002 set to Blurriness by the command draws FX-BLURRY-001's frame", &format!("largest difference {d:.1e}"), d <= 2e-5);

    t.heading("The preview card");
    match Gpu::new() {
        Err(why) => t.row("The card draws Blurriness", &format!("NOT RUN: no usable card ({why})"), false),
        Ok(mut gpu) => {
            for name in ["fx_blurry_001.json", "fx_blurry_002.json", "fx_blurry_004.json", "fx_blurry_006.json"] {
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

    t.heading("Pictures: a white square, in `verification/D-321 pictures/`, over black");
    let dir = effect_table::repo("verification/D-321 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], bytes).unwrap();
    write("square.png", &square());
    // Along the middle row, right of the square: the middle's level, and the last lit pixel's.
    let row = |bytes: &[u8]| {
        let at = |x: usize| bytes[((h / 2) * w + x) * 4];
        let last = (96..w).take_while(|&x| at(x) > 0).last().map_or(0, at);
        (at(96), at(70), last)
    };
    let mut middles = Vec::new();
    for (name, effects, float, what) in [
        ("1_square.png", json!([]), false, "the square with no effect"),
        ("2_old_blur_20.png", stack(20.0, "", None), false, "Gaussian Blur 20 in an old file (sigma 20): very soft, the square's middle well below white"),
        ("3_blurriness_20.png", stack(20.0, "blurriness", None), false, "Gaussian Blur at Blurriness 20, as After Effects reads it (sigma 6): the square kept, its edges softened"),
        ("4_old_cut_exposure_20.png", stack(6.0, "sigma", Some(20.0)), true, "the same sigma 6 cut at three sigmas, then Exposure +20 in Float: the hard-edged box P-26 saw"),
        ("5_blurriness_20_exposure_20.png", stack(20.0, "blurriness", Some(20.0)), true, "Blurriness 20 then Exposure +20 in Float: the glow fades out with no box"),
    ] {
        let (bytes, said) = picture(&dir, effects, float);
        write(name, &bytes);
        let (middle, outside, last) = row(&bytes);
        middles.push(middle);
        let ok = match name {
            "4_old_cut_exposure_20.png" => last == 255,
            "5_blurriness_20_exposure_20.png" => last <= 8 && outside > 0,
            _ => true,
        };
        t.row(
            &format!("{name}, {what}; draws cleanly"),
            &format!("{said:?}, the middle {middle} of 255, 14 pixels left of the square {outside}, the last lit pixel right of it {last}"),
            said.is_empty() && ok,
        );
    }
    t.row(
        "Blurriness 20 keeps the square brighter than the old Blur 20 (After Effects' 20 is a third as soft)",
        &format!("{} against {}", middles[2], middles[1]),
        middles[2] > middles[1] + 40,
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_blurry_001.json", 0), ("fx_blurry_004.json", 0), ("fx_blurry_007.json", 0)]);

    t.finish("D-321_blurriness_table.md");
}
