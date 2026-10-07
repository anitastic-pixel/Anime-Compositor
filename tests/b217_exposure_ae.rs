//! B-217: D-335, Exposure's Offset, Gamma Correction and Bypass Linear Light Conversion, from
//! P-26's tutorial 2.
//!
//! Tutorial 2 lifts its ground texture with Exposure 2.47 and Gamma Correction 1.69. Every
//! expected pixel is `Fixtures/exposure_ae/expected_exposure_ae.json`, written by
//! `tools/exposure_ae_reference.py` before this code existed.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::Id;
use anime_compositor::{persist, png_out, OutputDepth};

/// The town in a composition at 32 bpc (After Effects) with one Exposure of `parameters`, or
/// none. The frame as 8-bit straight pixels, and the warnings.
fn town(dir: &Path, parameters: Option<J>) -> (Vec<u8>, Vec<String>) {
    let (w, h) = TOWN;
    let t = |v: J| json!({"base": v, "keyframes": []});
    let effects: Vec<J> = parameters
        .into_iter()
        .map(|p| json!({"instance_id": "fx-0-0", "type_id": "core.exposure", "enabled": true, "parameters": p}))
        .collect();
    let project = json!({
        "schema_version": 0, "project_id": "proj-b217-picture",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-town", "kind": "still", "name": "town", "path": "town.png",
            "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
            "id": MAIN, "name": "Main", "width": w, "height": h, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 24,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 24},
            "layer_order": ["town"], "float_depth": true, "ae_32bpc": true,
            "layers": [{
                "id": "town", "kind": "raster", "name": "town", "asset_id": "asset-town", "enabled": true,
                "locked": false, "in_frame": 0, "out_frame": 24, "source_offset_frames": 0,
                "transform": {
                    "anchor": t(json!([w as f64 / 2.0, h as f64 / 2.0])), "position": t(json!([w as f64 / 2.0, h as f64 / 2.0])),
                    "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                },
                "exposure_spans": [], "mask": null, "matte": null, "blend_mode": "normal",
                "effects": effects
            }]
        }]
    });
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
}

#[test]
fn b217_exposure_ae() {
    let mut t = Table::new(
        "exposure_ae",
        "# B-217: Exposure's Offset, Gamma Correction and Bypass\n\nD-335, from P-26's tutorial 2. \
         Every expected pixel is `Fixtures/exposure_ae/expected_exposure_ae.json`, written by \
         `tools/exposure_ae_reference.py` before this code existed and printed in document 25 \
         as FX-EXPAE-001 to 016. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-EXPAE-001 to 016 (document 25)");
    t.fixtures("expected_exposure_ae.json");
    t.round_trips(&["fx_expae_004.json", "fx_expae_006.json", "fx_expae_010.json", "fx_expae_011.json", "fx_expae_015.json", "fx_expae_016.json"]);
    let saved = t.saved_parameters("fx_expae_001.json");
    t.row(
        "fx_expae_001.json, which writes no offset and no bypass, is saved without them",
        &saved.to_string(),
        saved.get("offset").is_none() && saved.get("bypass").is_none(),
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_expae_005.json", 0), ("fx_expae_009.json", 0)]);

    t.heading("Pictures: the town at 32 bpc (After Effects), in `verification/D-335 pictures/`");
    let dir = effect_table::repo("verification/D-335 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    png_out::write_rgba(&dir.join("town.png"), w, h, OutputDepth::Eight, &[], &effect_table::town()).unwrap();
    let mut shots = Vec::new();
    for (name, parameters, what) in [
        ("1_none.png", None, "no Exposure"),
        ("2_exposure.png", Some(json!({"stops": 2.47})), "Exposure 2.47 alone, as before D-335"),
        ("3_gamma.png", Some(json!({"stops": 2.47, "gamma": 1.69})), "Exposure 2.47 and Gamma Correction 1.69, tutorial 2's ground"),
        ("4_offset.png", Some(json!({"stops": 0, "offset": -0.1})), "Offset -0.1 alone: the darks go black"),
    ] {
        let (bytes, said) = town(&dir, parameters);
        png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        t.row(&format!("{name}, {what}; draws cleanly"), &format!("{said:?}"), said.is_empty());
        shots.push(bytes);
    }
    // A dark window and the road, which the gamma lifts more than the gain alone does.
    let at = |b: &[u8], x: usize, y: usize| b[(y * w + x) * 4];
    let road = (5, 250);
    t.row(
        "On the road the gamma lifts the grey above Exposure alone, and Offset -0.1 darkens it",
        &format!(
            "red: none {}, Exposure {}, with gamma {}, offset {}",
            at(&shots[0], road.0, road.1), at(&shots[1], road.0, road.1), at(&shots[2], road.0, road.1), at(&shots[3], road.0, road.1)
        ),
        at(&shots[2], road.0, road.1) > at(&shots[1], road.0, road.1) && at(&shots[3], road.0, road.1) < at(&shots[0], road.0, road.1),
    );

    t.finish("D-335_exposure_ae_table.md");
}
