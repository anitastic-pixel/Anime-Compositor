//! B-192: D-307, Hue/Saturation's colour ranges.
//!
//! P-26, tutorial 3 (the colourful glitch) turns only the reds of a picture with After Effects'
//! Hue/Saturation Channel Control; here it turned every colour. A range turns a colour on its
//! centre as the master does, half as much 30 degrees off it, and not at all 60 degrees off it
//! or on a grey.

#![recursion_limit = "256"]

use serde_json::{json, Value};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::Id;
use anime_compositor::persist;

const MAIN: &str = "comp-main";

fn shot(color: [f64; 3], parameters: Value) -> String {
    let t = |v: Value| json!({"base": v, "keyframes": []});
    let mut p = json!({"hue": 0, "saturation": 0, "lightness": 0});
    for (k, v) in parameters.as_object().unwrap() {
        p[k] = v.clone();
    }
    json!({
        "schema_version": 0, "project_id": "proj-b192",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": MAIN, "name": MAIN, "width": 4, "height": 4, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": ["art"],
            "layers": [{
                "id": "art", "kind": "solid", "name": "art", "enabled": true, "locked": false,
                "in_frame": 0, "out_frame": 1,
                "solid": {"color": color, "width": 4, "height": 4},
                "transform": {
                    "anchor": t(json!([2, 2])), "position": t(json!([2, 2])),
                    "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                },
                "masks": [], "matte": null, "blend_mode": "normal",
                "effects": [{"instance_id": "hs", "type_id": "core.hue_saturation", "enabled": true,
                    "parameters": p}]
            }]
        }]
    })
    .to_string()
}

fn pixel(color: [f64; 3], parameters: Value) -> [f32; 4] {
    let loaded = persist::load_str(&shot(color, parameters)).expect("the shot reads");
    let mut log = FrameLog::new(8);
    render_frame(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), 256, &mut log)
        .expect("the frame draws")
        .pixel(1, 1)
}

fn close(a: [f32; 4], b: [f32; 4]) -> bool {
    a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-4)
}

const RED: [f64; 3] = [1.0, 0.0, 0.0];
// Solids are linear; this green is 0.5 once encoded, which the effect works in: hue 30.
const ORANGE: [f64; 3] = [1.0, 0.214_041_14, 0.0];
const YELLOW: [f64; 3] = [1.0, 1.0, 0.0];
const GREY: [f64; 3] = [0.4, 0.4, 0.4];

#[test]
fn the_reds_turn_only_red() {
    let reds = json!({"reds_hsl": [120, 0, 0]});
    let red = pixel(RED, reds.clone());
    assert!(red[1] > 0.9 && red[0] < 0.1, "red turns green: {red:?}");
    assert!(close(red, pixel(RED, json!({"hue": 120}))), "as the master turns it");
    // Halfway out of the range, half as far.
    assert!(close(pixel(ORANGE, reds.clone()), pixel(ORANGE, json!({"hue": 60}))), "orange turns half as far");
    for c in [YELLOW, GREY, [0.0, 0.0, 1.0]] {
        assert!(close(pixel(c, reds.clone()), pixel(c, json!({}))), "{c:?} is left alone");
    }
    // Saturation and lightness the same way, with the master's added.
    let a = pixel(RED, json!({"reds_hsl": [0, -60, 20], "saturation": -20}));
    assert!(close(a, pixel(RED, json!({"saturation": -80, "lightness": 20}))), "{a:?}");
    // The other ranges sit 60 degrees apart: the yellows turn yellow, not red.
    let y = pixel(YELLOW, json!({"yellows_hsl": [-60, 0, 0], "reds_hsl": [90, 0, 0]}));
    assert!(close(y, pixel(RED, json!({}))), "yellow turns red: {y:?}");
}

#[test]
fn the_file_keeps_a_moved_range_only() {
    let saved = |p: Value| {
        let loaded = persist::load_str(&shot(RED, p)).expect("the shot reads");
        let s: Value = serde_json::from_str(&persist::to_json(loaded.document.project(), &loaded.preserved)).unwrap();
        s["compositions"][0]["layers"][0]["effects"][0]["parameters"].clone()
    };
    let old = saved(json!({}));
    assert!(old.get("reds_hsl").is_none(), "an old file stays as it was: {old}");
    let new = saved(json!({"blues_hsl": [10, -20, 30]}));
    assert_eq!(new["blues_hsl"], json!([10, -20, 30]));
    assert!(new.get("reds_hsl").is_none(), "{new}");
    let e = persist::load_str(&shot(RED, json!({"reds_hsl": [1, 2]}))).err().expect("two numbers are refused");
    assert!(format!("{e:?}").contains("three numbers, hue, saturation and lightness"), "{e:?}");
}
