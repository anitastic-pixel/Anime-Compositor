//! B-182: D-297, an adjustment layer's blend mode.
//!
//! P-26, tutorial 3 (Colorful Glitch) puts an adjustment layer in Add mode; D-66 refused every mode
//! but Normal. After Effects lays the adjusted frame over the frame beneath in the layer's mode.
//! A solid of linear grey g = 0.2 under an adjustment layer with Exposure +1 (E = 2g = 0.4), every
//! number worked by hand, both opaque so the modes are their plain formulas:
//!
//! - normal: 0.4, as D-66 always gave.
//! - multiply: 0.4 * 0.2 = 0.08.
//! - screen: 0.4 + 0.2 - 0.08 = 0.52.
//! - add: 0.4 + 0.2 = 0.6.
//!
//! At opacity 50% each is halfway back to 0.2: B + c (M - B).

use serde_json::{json, Value as J};

use anime_compositor::command::Command;
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{BlendMode, Id};
use anime_compositor::persist;

const MAIN: &str = "comp-main";
const G: f64 = 0.2;

fn transform(opacity: f64) -> J {
    json!({
        "anchor": {"base": [2, 2], "keyframes": []},
        "position": {"base": [2, 2], "keyframes": []},
        "scale": {"base": [100, 100], "keyframes": []},
        "rotation": {"base": 0, "keyframes": []},
        "opacity": {"base": opacity, "keyframes": []}
    })
}

fn shot(mode: &str, opacity: f64) -> String {
    json!({
        "schema_version": 0, "project_id": "proj-b182",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": MAIN, "name": MAIN, "width": 4, "height": 4, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": ["grey", "adj"],
            "layers": [
                {
                    "id": "grey", "kind": "solid", "name": "grey", "enabled": true, "locked": false,
                    "in_frame": 0, "out_frame": 1,
                    "solid": {"color": [G, G, G], "width": 4, "height": 4},
                    "transform": transform(1.0),
                    "mask": null, "matte": null, "blend_mode": "normal", "effects": []
                },
                {
                    "id": "adj", "kind": "adjustment", "name": "adj", "enabled": true, "locked": false,
                    "in_frame": 0, "out_frame": 1, "transform": transform(opacity),
                    "mask": null, "matte": null, "blend_mode": mode,
                    "effects": [{"instance_id": "fx-1", "type_id": "core.exposure", "enabled": true, "parameters": {"stops": 1}}]
                }
            ]
        }]
    })
    .to_string()
}

fn red_at_middle(mode: &str, opacity: f64) -> f64 {
    let loaded = persist::load_str(&shot(mode, opacity)).expect("the shot reads");
    let mut log = FrameLog::new(8);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), 64, &mut log).expect("the frame draws");
    assert!(log.finish().is_empty(), "the shot draws without a word");
    frame.pixel(2, 2)[0] as f64
}

#[test]
fn each_mode_lays_the_adjusted_frame_on_the_one_beneath() {
    for (mode, full) in [("normal", 0.4), ("multiply", 0.08), ("screen", 0.52), ("add", 0.6)] {
        let got = red_at_middle(mode, 1.0);
        assert!((got - full).abs() < 1e-6, "{mode} at 100%: {got} against {full}");
        let half = G + 0.5 * (full - G);
        let got = red_at_middle(mode, 0.5);
        assert!((got - half).abs() < 1e-6, "{mode} at 50%: {got} against {half}");
    }
}

#[test]
fn the_file_and_the_command_take_a_mode() {
    let loaded = persist::load_str(&shot("screen", 1.0)).expect("an adjustment layer in screen opens");
    let saved: J = serde_json::from_str(&persist::to_json(loaded.document.project(), &loaded.preserved)).unwrap();
    assert_eq!(saved["compositions"][0]["layers"][1]["blend_mode"], "screen");
    let mut document = persist::load_str(&shot("normal", 1.0)).unwrap().document;
    document
        .apply(Command::SetBlendMode { composition: Id::new(MAIN), layer_id: Id::new("adj"), mode: BlendMode::Add })
        .expect("an adjustment layer takes add");
    assert_eq!(document.project().compositions[0].layer(&Id::new("adj")).unwrap().blend_mode, BlendMode::Add);
}
