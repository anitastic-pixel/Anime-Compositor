//! B-216: D-337, Glow and Solid Composite on display values in After Effects' depths, from
//! P-26's tutorial 2.
//!
//! After Effects at 8 bpc, and at its own 32 bpc without a linear working space, lays Glow and
//! Solid Composite on display values; D-333 moved only the blurs there. Every expected pixel is
//! `Fixtures/glow_display/expected_glow_display.json`, written by
//! `tools/glow_display_reference.py` before this code existed.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::Id;
use anime_compositor::{persist, png_out, OutputDepth};

const SIZE: (usize, usize) = (480, 270);

/// A gold line 3 pixels wide from the top left to the bottom right on nothing, tutorial 2's bolt
/// colour #ba9a3c.
fn line(dir: &Path) {
    let (w, h) = SIZE;
    let mut px = vec![0u8; w * h * 4];
    for y in 0..h {
        for x in 0..w {
            let d = (x as f64 * h as f64 - y as f64 * w as f64).abs() / ((w * w + h * h) as f64).sqrt();
            if d <= 1.5 {
                px[(y * w + x) * 4..][..4].copy_from_slice(&[0xba, 0x9a, 0x3c, 255]);
            }
        }
    }
    png_out::write_rgba(&dir.join("line.png"), w, h, OutputDepth::Eight, &[], &px).unwrap();
}

/// Tutorial 2's glow layer, a quarter size: the line on black with Solid Composite, then three
/// Glows in After Effects units, threshold 0, intensity 0.1, radius 10, 30 and 68 (the
/// tutorial's 39, 119 and 274 over four). `depth` is "float" or "ae32". The frame as 8-bit
/// straight pixels, and the warnings.
fn glow_layer(dir: &Path, depth: &str) -> (Vec<u8>, Vec<String>) {
    let (w, h) = SIZE;
    let t = |v: J| json!({"base": v, "keyframes": []});
    let glow = |n: usize, radius: f64| {
        json!({"instance_id": format!("fx-0-{n}"), "type_id": "core.glow", "enabled": true, "parameters": {
            "based_on": "bright", "threshold": 0, "colors": [], "tolerance": 0, "radius": radius,
            "intensity": 0.1, "operation": "add", "tint": "", "units": "after_effects"}})
    };
    let mut comp = json!({
        "id": MAIN, "name": "Main", "width": w, "height": h, "pixel_aspect_ratio": 1,
        "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 24,
        "work_area": {"start_frame": 0, "end_frame_exclusive": 24},
        "layer_order": ["line"], "float_depth": true,
        "layers": [{
            "id": "line", "kind": "raster", "name": "line", "asset_id": "asset-line", "enabled": true,
            "locked": false, "in_frame": 0, "out_frame": 24, "source_offset_frames": 0,
            "transform": {
                "anchor": t(json!([w as f64 / 2.0, h as f64 / 2.0])), "position": t(json!([w as f64 / 2.0, h as f64 / 2.0])),
                "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
            },
            "exposure_spans": [], "mask": null, "matte": null, "blend_mode": "normal",
            "effects": [
                {"instance_id": "fx-0-0", "type_id": "core.solid_composite", "enabled": true,
                 "parameters": {"source_opacity": 100, "color": "#000000", "opacity": 100, "blend": "normal"}},
                glow(1, 10.0), glow(2, 30.0), glow(3, 68.0)
            ]
        }]
    });
    if depth == "ae32" {
        comp["ae_32bpc"] = J::from(true);
    }
    let project = json!({
        "schema_version": 0, "project_id": "proj-b216-picture",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-line", "kind": "still", "name": "line", "path": "line.png",
            "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [comp]
    });
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
}

#[test]
fn b216_glow_display() {
    let mut t = Table::new(
        "glow_display",
        "# B-216: Glow and Solid Composite on display values\n\nD-337, from P-26's tutorial 2. \
         Every expected pixel is `Fixtures/glow_display/expected_glow_display.json`, written by \
         `tools/glow_display_reference.py` before this code existed and printed in document 25 \
         as FX-GLDISP-001 to 008. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5. \
         FX-AE32-001 and 002, which D-337 changed, are checked by B-214.\n",
    );

    t.heading("FX-GLDISP-001 to 008 (document 25)");
    t.fixtures("expected_glow_display.json");
    t.round_trips(&["fx_gldisp_002.json", "fx_gldisp_006.json", "fx_gldisp_008.json"]);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_gldisp_002.json", 0), ("fx_gldisp_006.json", 0)]);

    t.heading("Pictures: tutorial 2's glow layer, a quarter size, in `verification/D-337 pictures/`");
    let dir = effect_table::repo("verification/D-337 pictures");
    fs::create_dir_all(&dir).unwrap();
    line(&dir);
    let (w, h) = SIZE;
    let mut shots = Vec::new();
    for (name, depth, what) in [
        ("1_float.png", "float", "Float, linear light: a thin line with a faint halo"),
        ("2_ae_32bpc.png", "ae32", "32 bpc (After Effects), D-337: the halo laid on in display values"),
    ] {
        let (bytes, said) = glow_layer(&dir, depth);
        png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        t.row(&format!("{name}, {what}; draws cleanly"), &format!("{said:?}"), said.is_empty());
        shots.push(bytes);
    }
    // 30 pixels straight below the line's middle, inside the halo.
    let at = |b: &[u8]| {
        let i = ((h / 2 + 30) * w + w / 2) * 4;
        [b[i], b[i + 1], b[i + 2]]
    };
    let (f, a) = (at(&shots[0]), at(&shots[1]));
    t.row(
        "30 pixels from the line the 32 bpc (After Effects) halo is brighter than Float's and still gold, green under red, blue under green",
        &format!("Float {f:?}, 32 bpc (After Effects) {a:?}"),
        a[0] > f[0] && a[1] < a[0] && a[2] < a[1],
    );

    t.finish("D-337_glow_display_table.md");
}
