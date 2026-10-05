//! B-208: D-327, After Effects' Fast Box Blur.
//!
//! Writes `verification/D-327_fast_box_blur_table.md`.
//!
//! Every expected pixel is `Fixtures/fast_box_blur/expected_fast_box_blur.json`, written by
//! `tools/fast_box_blur_reference.py` before this code existed and printed in document 25 as
//! FX-FASTBOX-001 to 011. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also blurs a white square with Gaussian Blur and with Fast Box Blur, plain and with
//! Exposure +17 after it in a Float composition, into `verification/D-327 pictures/`, over black.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{set, Table};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::{box_weights, Effect};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

fn fast_box(radius: f64, iterations: f64, dimensions: &str) -> Effect {
    Effect::FastBoxBlur { radius, iterations, edges: "transparent".into(), dimensions: dimensions.into() }
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

/// The square with `blur` and, when `stops`, an Exposure after it in a Float composition, over
/// black, 8-bit.
fn picture(dir: &Path, blur: J, stops: Option<f64>) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/fast_box_blur/fx_fastbox_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("square.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(PLATE.0);
    comp["height"] = J::from(PLATE.1);
    comp["float_depth"] = J::from(stops.is_some());
    let layer = &mut comp["layers"][0];
    let middle = json!([PLATE.0 as f64 / 2.0, PLATE.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    let mut effects = vec![blur];
    if let Some(stops) = stops {
        effects.push(json!({ "instance_id": "fx-0-1", "type_id": "core.exposure", "enabled": true, "parameters": { "stops": stops } }));
    }
    layer["effects"] = J::from(effects);
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(effect_table::MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    // Over black in linear light, held to white, so light past 1 shows as white.
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
fn b208_fast_box_blur() {
    let mut t = Table::new(
        "fast_box_blur",
        "# D-327: Fast Box Blur\n\nFrom P-26: tutorials 2 and 3 blur with After Effects' Fast Box \
         Blur, which the replays stood Gaussian Blur in for. After Effects' manual gives its \
         settings, Blur Radius, Iterations, Blur Dimensions and Repeat Edge Pixels: a box laid on \
         several times, ending hard at iterations times the radius, where a Gaussian's faint tail \
         goes on and an Exposure after it shows the difference. Every expected pixel is \
         `Fixtures/fast_box_blur/expected_fast_box_blur.json`, written by \
         `tools/fast_box_blur_reference.py` before the build had it, printed in document 25 as \
         FX-FASTBOX-001 to 011. Tolerance 2e-5.\n",
    );

    t.heading("FX-FASTBOX-001 to 011 (document 25)");
    t.fixtures("expected_fast_box_blur.json");

    t.heading("The kernel");
    for (r, n) in [(4.0, 3.0), (2.5, 1.0), (2.5, 3.0), (0.0, 3.0), (1.0, 50.0), (500.0, 1.0)] {
        let k = box_weights(r, n);
        let sum: f64 = k.iter().map(|&v| v as f64).sum();
        let reach = (n as usize) * (r as f64).ceil() as usize;
        t.row(
            &format!("Radius {r}, iterations {n}: reaches {reach} pixels each side and sums to 1"),
            &format!("{} taps, sum {sum:.7}", k.len()),
            k.len() == 2 * reach + 1 && (sum - 1.0).abs() < 1e-5,
        );
    }

    t.heading("The file");
    t.round_trips(&["fx_fastbox_001.json", "fx_fastbox_003.json", "fx_fastbox_004.json", "fx_fastbox_005.json", "fx_fastbox_009.json", "fx_fastbox_011.json"]);
    let saved = t.saved_parameters("fx_fastbox_004.json");
    t.row("Saved with its radius, iterations, edges and dimensions", &saved.to_string(), saved["radius"] == 4 && saved["iterations"] == 3 && saved["edges"] == "repeat");

    t.heading("Commands");
    let mut document = t.load("fx_fastbox_002.json").document;
    t.refused(
        &mut document,
        vec![
            ("iterations 0", set(fast_box(4.0, 0.0, "both"))),
            ("radius 501", set(fast_box(501.0, 3.0, "both"))),
            ("Blur Dimensions \"Both\", written with a capital", set(fast_box(4.0, 3.0, "Both"))),
        ],
    );
    t.taken(&mut document, "fx_fastbox_002.json", vec![("FX-FASTBOX-002 set to iterations 3,", set(fast_box(4.0, 3.0, "both")))]);
    let mut document = t.load("fx_fastbox_002.json").document;
    document.apply(set(fast_box(4.0, 3.0, "both"))).unwrap();
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_fast_box_blur.json")).unwrap()).unwrap();
    let d = effect_table::largest_difference(&t.render(&document, 0, 64), &expected["cases"]["FX-FASTBOX-001"]["frames"]["0"]);
    t.row("FX-FASTBOX-002 set to iterations 3 by the command draws FX-FASTBOX-001's frame", &format!("largest difference {d:.1e}"), d <= 2e-5);

    t.heading("A half-size draft");
    let mut draft = fast_box(8.0, 3.0, "both");
    draft.scale_distances(|v| v / 2.0);
    t.row("A half-size draft halves the radius and keeps the iterations", &format!("{draft:?}"), draft == fast_box(4.0, 3.0, "both"));

    t.heading("The preview");
    match Gpu::new() {
        Err(why) => t.row("The preview draws Fast Box Blur", &format!("NOT RUN: no usable card ({why})"), false),
        Ok(mut gpu) => {
            for name in ["fx_fastbox_001.json", "fx_fastbox_004.json", "fx_fastbox_007.json"] {
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
                    &format!("{name}: the preview draws the same picture as the CPU, within 1 level of 255"),
                    &format!("{}, largest difference {worst} of 255", if on_cpu { "CPU" } else { "card" }),
                    worst <= 1,
                );
            }
        }
    }

    t.heading("Pictures: a white square, in `verification/D-327 pictures/`, over black");
    let dir = effect_table::repo("verification/D-327 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], bytes).unwrap();
    write("square.png", &square());
    let gaussian = json!({ "instance_id": "fx-0-0", "type_id": "core.gaussian_blur", "enabled": true, "parameters": { "sigma_px": 20, "units": "blurriness" } });
    let boxed = json!({ "instance_id": "fx-0-0", "type_id": "core.fast_box_blur", "enabled": true, "parameters": { "radius": 6, "iterations": 3 } });
    // Along the middle row, 18 and 24 pixels right of the square (its right edge is column 107).
    let row = |bytes: &[u8]| {
        let at = |x: usize| bytes[((h / 2) * w + x) * 4];
        (at(107 + 18), at(107 + 24))
    };
    let mut far = Vec::new();
    for (name, blur, stops, what) in [
        ("1_gaussian_blurriness_20.png", gaussian.clone(), None, "Gaussian Blur, Blurriness 20"),
        ("2_fast_box_radius_6.png", boxed.clone(), None, "Fast Box Blur, radius 6, iterations 3: about as soft"),
        ("3_gaussian_then_exposure_17.png", gaussian, Some(17.0), "the Gaussian with Exposure +17 after it, Float: its faint tail lit up far out"),
        ("4_fast_box_then_exposure_17.png", boxed, Some(17.0), "Fast Box Blur with Exposure +17 after it, Float: ending hard 18 pixels out"),
    ] {
        let (bytes, said) = picture(&dir, blur, stops);
        write(name, &bytes);
        let (near, out) = row(&bytes);
        far.push((near, out));
        t.row(&format!("{name}, {what}; draws cleanly"), &format!("{said:?}, 18 pixels right of the square {near} of 255, 24 pixels right {out}"), said.is_empty());
    }
    t.row(
        "With Exposure +17, the Gaussian still shows 24 pixels out; Fast Box Blur is black there",
        &format!("Gaussian {}, Fast Box Blur {}", far[2].1, far[3].1),
        far[2].1 > 20 && far[3].1 == 0,
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_fastbox_001.json", 0), ("fx_fastbox_004.json", 0), ("fx_fastbox_007.json", 0)]);

    t.finish("D-327_fast_box_blur_table.md");
}
