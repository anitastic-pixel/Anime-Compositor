//! B-184: D-299, Fractal Noise's Fractal Type, Noise Type, Invert, Offset Turbulence, Scale
//! Width and Height, and Cycle Evolution, as After Effects has them.
//!
//! P-26, tutorials 1 (Shockwave) and 3 (Colorful Glitch) set each of these; here there were none.
//! Worked by hand on a white 8x8 solid with black-to-white noise, contrast 100 and brightness 0,
//! so a pixel's grey `v` is 0.5 + 0.5 n for the noise `n`, unclamped:
//! - Invert turns `n` over, so its grey is 1 - v.
//! - With one octave, Turbulent's noise is 2|n| - 1, so its grey is |2 v - 1|.
//! - Block noise of one octave at size 4 is one grey over each 4 by 4 cell.
//! - Offset (3, 0) is the plain noise moved 3 pixels right.
//! - Scale Width 50% of size 100 is 50 across and 100 down: Size 50 at Scale Height 200%.
//! - Cycle 1 repeats every turn, so evolution 90 and 450 degrees draw the same; Cycle 2 repeats
//!   every two, so 90 and 810 do.
//! The defaults draw D-128's noise unchanged: `tests/b71_fractal_noise.rs`'s fixtures still pass.

use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::Id;
use anime_compositor::persist;

const MAIN: &str = "comp-main";

fn shot(extra: J) -> String {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let mut parameters = json!({"size": 4, "complexity": 1, "contrast": 100, "brightness": 0, "evolution": 30,
        "speed": 0, "seed": 7, "dark_color": "#000000", "light_color": "#ffffff", "opacity": 100, "blend": "normal"});
    for (k, v) in extra.as_object().unwrap() {
        parameters[k] = v.clone();
    }
    json!({
        "schema_version": 0, "project_id": "proj-b184",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": MAIN, "name": MAIN, "width": 8, "height": 8, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": ["white"],
            "layers": [{
                "id": "white", "kind": "solid", "name": "white", "enabled": true, "locked": false,
                "in_frame": 0, "out_frame": 1,
                "solid": {"color": [1, 1, 1], "width": 8, "height": 8},
                "transform": {
                    "anchor": t(json!([4, 4])), "position": t(json!([4, 4])),
                    "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                },
                "masks": [], "matte": null, "blend_mode": "normal",
                "effects": [{"instance_id": "f", "type_id": "core.fractal_noise", "enabled": true, "parameters": parameters}]
            }]
        }]
    })
    .to_string()
}

/// Each pixel's red, row by row, in linear light.
fn frame(extra: J) -> Vec<f32> {
    let loaded = persist::load_str(&shot(extra)).expect("the shot reads");
    let mut log = FrameLog::new(8);
    let f = render_frame(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), 64, &mut log)
        .expect("the frame draws");
    (0..8).flat_map(|y| (0..8).map(move |x| (x, y))).map(|(x, y)| f.pixel(x, y)[0]).collect()
}

/// The grey as encoded, document 21's sRGB.
fn grey(linear: f32) -> f64 {
    let c = linear as f64;
    if c <= 0.0031308 { c * 12.92 } else { 1.055 * c.powf(1.0 / 2.4) - 0.055 }
}

#[test]
fn invert_and_turbulent_are_the_greys_worked_by_hand() {
    let plain = frame(json!({}));
    let inverted = frame(json!({"invert": "on"}));
    let turbulent = frame(json!({"fractal_type": "turbulent"}));
    for i in 0..64 {
        let v = grey(plain[i]);
        assert!((grey(inverted[i]) - (1.0 - v)).abs() < 1e-5, "pixel {i}: inverted {} against {}", grey(inverted[i]), 1.0 - v);
        let want = (2.0 * v - 1.0).abs();
        assert!((grey(turbulent[i]) - want).abs() < 1e-5, "pixel {i}: turbulent {} against {want}", grey(turbulent[i]));
    }
    assert!(plain.iter().any(|v| *v != plain[0]), "the plain noise is not one grey");
}

#[test]
fn block_noise_is_one_grey_a_cell() {
    let block = frame(json!({"noise_type": "block"}));
    for (cx, cy) in [(0, 0), (4, 0), (0, 4), (4, 4)] {
        let first = block[cy * 8 + cx];
        for y in cy..cy + 4 {
            for x in cx..cx + 4 {
                assert_eq!(block[y * 8 + x], first, "pixel ({x}, {y}) in the cell at ({cx}, {cy})");
            }
        }
    }
    let corners = [block[0], block[4], block[32], block[36]];
    assert!(corners.iter().any(|v| *v != corners[0]), "the four cells are not one grey: {corners:?}");
}

#[test]
fn offset_and_scale_move_and_stretch_the_clouds() {
    let plain = frame(json!({}));
    let moved = frame(json!({"offset": [3, 0]}));
    for y in 0..8 {
        for x in 0..5 {
            assert_eq!(moved[y * 8 + x + 3], plain[y * 8 + x], "({x}, {y}) moved 3 right");
        }
    }
    let wide = frame(json!({"size": 100, "scale_width": 50, "complexity": 4}));
    let tall = frame(json!({"size": 50, "scale_height": 200, "complexity": 4}));
    assert_eq!(wide, tall, "50 across and 100 down, both ways");
    assert_ne!(wide, frame(json!({"size": 100, "complexity": 4})), "Scale Width changes the picture");
}

#[test]
fn cycle_repeats_after_its_turns() {
    let at = |evolution: f64, cycle: f64| frame(json!({"evolution": evolution, "cycle": cycle, "complexity": 4}));
    assert_eq!(at(90.0, 1.0), at(450.0, 1.0), "cycle 1: one turn on is the same");
    assert_eq!(at(90.0, 2.0), at(810.0, 2.0), "cycle 2: two turns on is the same");
    assert_ne!(at(90.0, 2.0), at(450.0, 2.0), "cycle 2: one turn on is not");
    assert_ne!(at(90.0, 0.0), at(450.0, 0.0), "no cycle: it never repeats");
}

#[test]
fn the_file_keeps_the_look_and_reports_a_wrong_one() {
    let effect = |text: &str| {
        let loaded = persist::load_str(text).expect("the shot reads");
        let saved: J = serde_json::from_str(&persist::to_json(loaded.document.project(), &loaded.preserved)).unwrap();
        let e = loaded.document.project().compositions[0].layer(&Id::new("white")).unwrap().effects[0].effect.clone();
        (saved["compositions"][0]["layers"][0]["effects"][0]["parameters"].clone(), e)
    };
    let (saved, _) = effect(&shot(json!({})));
    for key in ["fractal_type", "noise_type", "invert", "offset", "scale_width", "scale_height", "cycle"] {
        assert!(saved.get(key).is_none(), "{key} is not written at its start, so the file saves as before");
    }
    let look = json!({"fractal_type": "turbulent", "noise_type": "block", "invert": "on", "offset": [12.5, -3],
        "scale_width": 430, "scale_height": 3, "cycle": 4});
    let (saved, _) = effect(&shot(look.clone()));
    for (k, v) in look.as_object().unwrap() {
        assert_eq!(saved[k].to_string(), v.to_string(), "{k} is kept");
    }
    let (_, wrong) = effect(&shot(json!({"fractal_type": "wavy"})));
    assert_eq!(wrong.why_invalid(), "Fractal Noise's fractal type is \"basic\" or \"turbulent\", and this is \"wavy\".");
    let (_, wrong) = effect(&shot(json!({"scale_width": 0})));
    assert!(!wrong.is_valid(), "a Scale Width of 0 is refused: {}", wrong.why_invalid());
    let (_, mut draft) = effect(&shot(json!({"offset": [40, -10]})));
    draft.scale_distances(|d| d * 0.5);
    let (_, half) = effect(&shot(json!({"size": 2, "offset": [20, -5]})));
    assert_eq!(draft, half, "a half-size draft halves the size and the offset");
}
