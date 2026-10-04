//! B-183: D-298, a mask's feather, opacity and expansion take keys, as After Effects' do.
//!
//! P-26, tutorial 4 (Chris Connor's lightsaber) and tutorial 1 (Shockwave) animate a mask's
//! feather and opacity; here they were one number for the whole shot. Worked by hand: a white
//! 8x8 solid masked to the square (1,1)-(7,7), whose middle pixel (4,4) is wholly inside it.
//! Opacity keyed 1 at frame 0 and 0 at frame 10 is 0.5 at frame 5, so the middle pixel is 0.5.
//! Feather keyed 0 to 8 is 4 at frame 5, and expansion keyed -2 to 2 is 0 there: frame 5 is
//! then the same picture, pixel for pixel, as a mask with those plain numbers.

#![recursion_limit = "256"]

use serde_json::{json, Value as J};

use anime_compositor::command::Command;
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{Id, Interp, Keyframe, Value};
use anime_compositor::persist;

const MAIN: &str = "comp-main";

fn keys(a: f64, b: f64) -> J {
    json!({"base": a, "keyframes": [
        {"frame": 0, "value": a, "interp": "linear"},
        {"frame": 10, "value": b, "interp": "linear"}
    ]})
}

fn shot(opacity: J, feather: J, expansion: J) -> String {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let c = |x: i32, y: i32| json!({"point": [x, y], "in": [0, 0], "out": [0, 0]});
    json!({
        "schema_version": 0, "project_id": "proj-b183",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": MAIN, "name": MAIN, "width": 8, "height": 8, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": 11, "work_area": {"start_frame": 0, "end_frame_exclusive": 11},
            "layer_order": ["white"],
            "layers": [{
                "id": "white", "kind": "solid", "name": "white", "enabled": true, "locked": false,
                "in_frame": 0, "out_frame": 11,
                "solid": {"color": [1, 1, 1], "width": 8, "height": 8},
                "transform": {
                    "anchor": t(json!([4, 4])), "position": t(json!([4, 4])),
                    "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                },
                "masks": [{
                    "name": "m", "mode": "add", "opacity": opacity, "feather_px": feather,
                    "expansion_px": expansion,
                    "enabled": true, "inverted": false,
                    "path": {"base": {"points": [c(1, 1), c(7, 1), c(7, 7), c(1, 7)]}, "keyframes": []}
                }],
                "matte": null, "blend_mode": "normal", "effects": []
            }]
        }]
    })
    .to_string()
}

fn frame(text: &str, at: i32) -> Vec<[f32; 4]> {
    let loaded = persist::load_str(text).expect("the shot reads");
    let mut log = FrameLog::new(8);
    let f = render_frame(loaded.document.project(), &Id::new(MAIN), at, std::path::Path::new("."), 64, &mut log)
        .expect("the frame draws");
    (0..8).flat_map(|y| (0..8).map(move |x| (x, y))).map(|(x, y)| f.pixel(x, y)).collect()
}

#[test]
fn a_keyed_opacity_is_halfway_at_the_halfway_frame() {
    let text = shot(keys(1.0, 0.0), json!(0), json!(0));
    for (at, want) in [(0, 1.0), (5, 0.5), (10, 0.0)] {
        let got = frame(&text, at)[4 * 8 + 4][0] as f64;
        assert!((got - want).abs() < 1e-6, "frame {at}: {got} against {want}");
    }
}

#[test]
fn keyed_feather_and_expansion_draw_as_their_plain_values() {
    let keyed = frame(&shot(json!(1), keys(0.0, 8.0), keys(-2.0, 2.0)), 5);
    let plain = frame(&shot(json!(1), json!(4), json!(0)), 5);
    assert_eq!(keyed, plain, "frame 5 is feather 4 and expansion 0");
    let start = frame(&shot(json!(1), keys(0.0, 8.0), keys(-2.0, 2.0)), 0);
    assert_eq!(start, frame(&shot(json!(1), json!(0), json!(-2)), 0), "frame 0 is the first keys");
}

#[test]
fn the_file_keeps_the_keys_and_refuses_what_d77_refuses() {
    let loaded = persist::load_str(&shot(keys(1.0, 0.0), json!(3), json!(0))).expect("keys open");
    let saved: J = serde_json::from_str(&persist::to_json(loaded.document.project(), &loaded.preserved)).unwrap();
    let mask = &saved["compositions"][0]["layers"][0]["masks"][0];
    assert_eq!(mask["opacity"]["keyframes"][1]["value"].as_f64(), Some(0.0));
    assert_eq!(mask["feather_px"].as_f64(), Some(3.0), "a number without keys stays plain");
    for bad in [
        shot(keys(1.0, 1.5), json!(0), json!(0)),
        shot(json!(1), keys(0.0, -1.0), json!(0)),
        shot(json!(1), json!(0), keys(0.0, 9000.0)),
        shot(json!({"base": 1, "keyframes": [], "expression": {"text": "1", "enabled": true}}), json!(0), json!(0)),
    ] {
        assert!(persist::load_str(&bad).is_err(), "refused: {bad}");
    }
}

#[test]
fn the_command_takes_keys_and_refuses_one_out_of_range() {
    let mut document = persist::load_str(&shot(json!(1), json!(0), json!(0))).unwrap().document;
    let mut masks = document.project().compositions[0].layer(&Id::new("white")).unwrap().masks.clone();
    let mut track = masks[0].channel("opacity").unwrap();
    let key = |frame, v| Keyframe {
        frame,
        value: Value::Scalar(v),
        interp: Interp::Linear,
        spatial: None,
        kind: Default::default(),
        roving: false,
    };
    track.set_keyframe(key(0, 1.0));
    track.set_keyframe(key(10, -0.1));
    masks[0].set_channel("opacity", track.clone());
    let set = |masks| Command::SetMasks { composition: Id::new(MAIN), layer_id: Id::new("white"), masks };
    let refused = document.apply(set(masks.clone())).err().expect("an opacity of -0.1 is refused");
    assert_eq!(refused.message, "Mask \"m\" was given an opacity of -0.1, which is not from 0 to 1.");
    track.set_keyframe(key(10, 0.0));
    masks[0].set_channel("opacity", track);
    document.apply(set(masks)).expect("keys inside the range are taken");
    let held = &document.project().compositions[0].layer(&Id::new("white")).unwrap().masks[0];
    assert_eq!(held.at(5).opacity, 0.5);
}
