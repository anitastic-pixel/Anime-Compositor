//! B-190: D-305, Shift Channels.
//!
//! P-26, tutorial 3 (Colorful Glitch) takes a layer's alpha from its brightness with After
//! Effects' Shift Channels; here there was no such effect. Worked by hand on a 4x4 solid in an
//! 8x8 frame, read at its pixel (4, 4); a solid is opaque, so its straight colour is its colour.
//! With `lin(v)` document 21's sRGB-to-linear of an encoded `v`, the frame's premultiplied pixel:
//! - red from green and green from red, on red: green, `[0, 1, 0, 1]`;
//! - blue from full, on red: `[1, 0, 1, 1]`;
//! - red from half: `[lin(0.5), 0, 0, 1]`;
//! - alpha from half: red at half covering, `[0.5, 0, 0, 0.5]`;
//! - alpha from green, on red: nothing;
//! - red from luminance, on red: `lin(0.2126)`, the luma's red weight;
//! - red from hue, on green: `lin(1 / 3)`, a third of a turn;
//! - red from lightness, on red: `lin(0.5)`; from saturation, on red: 1; on a grey: 0;
//! - alpha from full where Linear Wipe cleared the drawing: black, `[0, 0, 0, 1]`.

#![recursion_limit = "256"]

use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};

const MAIN: &str = "comp-main";

fn shot(color: [f64; 3], wipe: f64, take: J) -> String {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let mut parameters = json!({"take_alpha": "alpha", "take_red": "red", "take_green": "green", "take_blue": "blue"});
    for (k, v) in take.as_object().unwrap() {
        parameters[k] = v.clone();
    }
    json!({
        "schema_version": 0, "project_id": "proj-b190",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": MAIN, "name": MAIN, "width": 8, "height": 8, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": ["art"],
            "layers": [{
                "id": "art", "kind": "solid", "name": "art", "enabled": true, "locked": false,
                "in_frame": 0, "out_frame": 1,
                "solid": {"color": color, "width": 4, "height": 4},
                "transform": {
                    "anchor": t(json!([2, 2])), "position": t(json!([4, 4])),
                    "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                },
                "masks": [], "matte": null, "blend_mode": "normal",
                "effects": [
                    {"instance_id": "wipe", "type_id": "core.linear_wipe", "enabled": true,
                     "parameters": {"completion": wipe, "angle": 90, "feather": 0}},
                    {"instance_id": "shift", "type_id": "core.shift_channels", "enabled": true, "parameters": parameters}
                ]
            }]
        }]
    })
    .to_string()
}

fn pixel(color: [f64; 3], wipe: f64, take: J) -> [f64; 4] {
    let loaded = persist::load_str(&shot(color, wipe, take)).expect("the shot reads");
    let mut log = FrameLog::new(8);
    let f = render_frame(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), 256, &mut log)
        .expect("the frame draws");
    f.pixel(4, 4).map(|v| v as f64)
}

/// Document 21's sRGB decode.
fn lin(v: f64) -> f64 {
    if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
}

fn near(got: [f64; 4], want: [f64; 4], what: &str) {
    for c in 0..4 {
        assert!((got[c] - want[c]).abs() < 1e-5, "{what}: {got:?} against {want:?}");
    }
}

const RED: [f64; 3] = [1.0, 0.0, 0.0];

#[test]
fn each_channel_comes_from_where_it_is_told() {
    near(pixel(RED, 0.0, json!({})), [1., 0., 0., 1.], "as it starts, nothing changes");
    near(pixel(RED, 0.0, json!({"take_red": "green", "take_green": "red"})), [0., 1., 0., 1.], "red and green swapped");
    near(pixel(RED, 0.0, json!({"take_blue": "full"})), [1., 0., 1., 1.], "blue full");
    near(pixel(RED, 0.0, json!({"take_red": "half"})), [lin(0.5), 0., 0., 1.], "red half");
    near(pixel(RED, 0.0, json!({"take_alpha": "half"})), [0.5, 0., 0., 0.5], "alpha half");
    near(pixel(RED, 0.0, json!({"take_alpha": "green"})), [0., 0., 0., 0.], "alpha from an empty green");
    near(pixel(RED, 0.0, json!({"take_red": "luminance"})), [lin(0.2126), 0., 0., 1.], "red from luminance");
    near(pixel([0.0, 1.0, 0.0], 0.0, json!({"take_red": "hue"})), [lin(1.0 / 3.0), 1., 0., 1.], "red from green's hue");
    near(pixel(RED, 0.0, json!({"take_red": "lightness"})), [lin(0.5), 0., 0., 1.], "red from lightness");
    near(pixel(RED, 0.0, json!({"take_green": "saturation"})), [1., 1., 0., 1.], "green from red's saturation");
    let grey = lin(0.5);
    near(pixel([grey; 3], 0.0, json!({"take_red": "saturation"})), [0., grey, grey, 1.], "a grey has none");
    near(pixel(RED, 0.0, json!({"take_green": "off"})), [1., 0., 0., 1.], "green off");
    // The wipe clears the drawing's left half, frame columns 2 to 3; (4, 4) is kept, (2, 4) is not.
    let loaded = persist::load_str(&shot(RED, 50.0, json!({"take_alpha": "full"}))).expect("the shot reads");
    let mut log = FrameLog::new(8);
    let f = render_frame(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), 256, &mut log).unwrap();
    near(f.pixel(2, 4).map(|v| v as f64), [0., 0., 0., 1.], "a cleared pixel made solid is black");
    near(f.pixel(4, 4).map(|v| v as f64), [1., 0., 0., 1.], "a kept one keeps its colour");
}

#[test]
fn the_card_draws_it_the_same() {
    let Ok(mut gpu) = Gpu::new() else {
        eprintln!("no usable card; nothing to check");
        return;
    };
    let loaded = persist::load_str(&shot(RED, 0.0, json!({"take_red": "green", "take_green": "red", "take_blue": "full"})))
        .expect("the shot reads");
    let mut log = FrameLog::new(8);
    let mut cache = CelCache::viewer();
    let (px, w, _) = preview::preview_frame_srgb8(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), PreviewQuality::Full, 256, &mut log, &mut cache, &mut gpu)
        .expect("the frame draws");
    assert_eq!(&px[(4 * w + 4) * 4..][..4], [0, 255, 255, 255], "cyan");
}

#[test]
fn the_file_keeps_it_and_reports_a_wrong_word() {
    let loaded = persist::load_str(&shot(RED, 0.0, json!({"take_alpha": "luminance"}))).expect("the shot reads");
    let saved: J = serde_json::from_str(&persist::to_json(loaded.document.project(), &loaded.preserved)).unwrap();
    let p = &saved["compositions"][0]["layers"][0]["effects"][1]["parameters"];
    assert_eq!(p, &json!({"take_alpha": "luminance", "take_red": "red", "take_green": "green", "take_blue": "blue"}));
    let loaded = persist::load_str(&shot(RED, 0.0, json!({"take_blue": "purple"}))).expect("the shot reads");
    let e = &loaded.document.project().compositions[0].layer(&Id::new("art")).unwrap().effects[1].effect;
    assert_eq!(
        e.why_invalid(),
        "Shift Channels takes blue from \"alpha\", \"red\", \"green\", \"blue\", \"luminance\", \"hue\", \"lightness\", \"saturation\", \"full\", \"half\" or \"off\", and this is \"purple\"."
    );
}
