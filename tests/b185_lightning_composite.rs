//! B-185: D-300, Lightning Bolt's Composite on Original, as After Effects has it.
//!
//! P-26, tutorial 2 (Advanced Electric): After Effects' Advanced Lightning starts with Composite
//! on Original off, so on a black solid the layer is the bolt alone and Add puts it over the
//! shot. Here the bolt was always painted onto the solid, which hid everything under it.
//! Worked by hand from D-190's rule on an opaque red 16x16 solid: the bolt adds its light `L` to
//! a pixel's colour and covers by `A`. On, a lit pixel is red + L with alpha 1; off, it is L with
//! alpha A on a clear layer, and an unlit pixel is clear. So on minus off is the red, exactly
//! where off's alpha is 1 - A short of on's.

use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::Id;
use anime_compositor::persist;

const MAIN: &str = "comp-main";

fn shot(extra: J) -> String {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let mut parameters = json!({"start": [50, 0], "end": [50, 100], "jagged": 0, "detail": 2, "branches": 0,
        "width": 3, "glow": 4, "opacity": 100, "hold": 1, "seed": 0, "color": "#ffffff", "glow_color": "#6e8cff"});
    for (k, v) in extra.as_object().unwrap() {
        parameters[k] = v.clone();
    }
    json!({
        "schema_version": 0, "project_id": "proj-b185",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": MAIN, "name": MAIN, "width": 16, "height": 16, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": ["red"],
            "layers": [{
                "id": "red", "kind": "solid", "name": "red", "enabled": true, "locked": false,
                "in_frame": 0, "out_frame": 1,
                "solid": {"color": [1, 0, 0], "width": 16, "height": 16},
                "transform": {
                    "anchor": t(json!([8, 8])), "position": t(json!([8, 8])),
                    "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                },
                "masks": [], "matte": null, "blend_mode": "normal",
                "effects": [{"instance_id": "f", "type_id": "core.lightning_bolt", "enabled": true, "parameters": parameters}]
            }]
        }]
    })
    .to_string()
}

fn frame(extra: J) -> Vec<[f32; 4]> {
    let loaded = persist::load_str(&shot(extra)).expect("the shot reads");
    let mut log = FrameLog::new(8);
    let f = render_frame(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), 256, &mut log)
        .expect("the frame draws");
    (0..16).flat_map(|y| (0..16).map(move |x| (x, y))).map(|(x, y)| f.pixel(x, y)).collect()
}

#[test]
fn off_is_the_bolt_alone_and_on_is_as_before() {
    let before = frame(json!({}));
    let on = frame(json!({"composite": "on"}));
    let off = frame(json!({"composite": "off"}));
    assert_eq!(before, on, "a file without the setting draws as before, which is on");
    let (mut lit, mut clear) = (0, 0);
    for (i, (a, b)) in on.iter().zip(&off).enumerate() {
        if b[3] == 0.0 {
            assert_eq!(*b, [0.0; 4], "pixel {i}: off and unlit is clear");
            assert_eq!(*a, [1.0, 0.0, 0.0, 1.0], "pixel {i}: on and unlit is the red");
            clear += 1;
            continue;
        }
        lit += 1;
        assert!((a[0] - b[0] - 1.0).abs() < 1e-6, "pixel {i}: on's red is off's plus the solid's: {a:?} {b:?}");
        assert!((a[1] - b[1]).abs() < 1e-6 && (a[2] - b[2]).abs() < 1e-6, "pixel {i}: the same light: {a:?} {b:?}");
        assert_eq!(a[3], 1.0, "pixel {i}: on stays opaque");
        assert!(b[3] > 0.0 && b[3] <= 1.0, "pixel {i}: off covers by the bolt's own share: {}", b[3]);
    }
    assert!(lit > 16 && clear > 16, "the bolt lights some pixels and not others: {lit} lit, {clear} clear");
}

#[test]
fn the_file_keeps_it_and_reports_a_wrong_one() {
    let saved = |text: &str| {
        let loaded = persist::load_str(text).expect("the shot reads");
        let saved: J = serde_json::from_str(&persist::to_json(loaded.document.project(), &loaded.preserved)).unwrap();
        saved["compositions"][0]["layers"][0]["effects"][0]["parameters"].clone()
    };
    assert!(saved(&shot(json!({}))).get("composite").is_none(), "on is not written, so the file saves as before");
    assert_eq!(saved(&shot(json!({"composite": "on"})))["composite"], "on", "a file that wrote on keeps it as written");
    assert_eq!(saved(&shot(json!({"composite": "off"})))["composite"], "off", "off is kept");
    let loaded = persist::load_str(&shot(json!({"composite": "maybe"}))).expect("the shot reads");
    let e = &loaded.document.project().compositions[0].layer(&Id::new("red")).unwrap().effects[0].effect;
    assert_eq!(e.why_invalid(), "Lightning Bolt's composite on original is \"off\" or \"on\", and this is \"maybe\".");
}
