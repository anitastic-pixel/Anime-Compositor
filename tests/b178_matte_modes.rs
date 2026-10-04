//! B-178: D-293, After Effects' four track matte modes.
//!
//! P-26 found only the alpha matte; the tutorials' luma mattes could not be built. A white frame
//! is matted by a grey square over its left half, on nothing, so the frame's alpha is the cover.
//! The grey is the one whose display value is exactly 0.5, so every expected number is worked
//! by hand: alpha 1 and 0, luma 0.5 and 0, and one minus each for the inverted modes.

use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::preview::{preview_frame, PreviewQuality};

const MAIN: &str = "comp-main";

/// The linear value whose sRGB encoding is 0.5.
fn half_grey() -> f64 {
    ((0.5 + 0.055) / 1.055f64).powf(2.4)
}

fn solid(id: &str, color: [f64; 3], (w, h): (u32, u32), at: [f64; 2]) -> J {
    json!({
        "id": id, "kind": "solid", "name": id, "enabled": true, "locked": false,
        "in_frame": 0, "out_frame": 1,
        "solid": {"color": color, "width": w, "height": h},
        "transform": {
            "anchor": {"base": [w as f64 / 2.0, h as f64 / 2.0], "keyframes": []},
            "position": {"base": at, "keyframes": []},
            "scale": {"base": [100, 100], "keyframes": []},
            "rotation": {"base": 0, "keyframes": []},
            "opacity": {"base": 1, "keyframes": []}
        },
        "mask": null, "matte": null, "blend_mode": "normal", "effects": []
    })
}

fn shot(mode: &str) -> J {
    let g = half_grey();
    let mut picture = solid("picture", [1.0, 1.0, 1.0], (64, 32), [32.0, 16.0]);
    picture["matte"] = json!({"layer_id": "square", "mode": mode, "matte_only": true});
    let layers = vec![picture, solid("square", [g, g, g], (32, 32), [16.0, 16.0])];
    json!({
        "schema_version": 0, "project_id": "proj-b178",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": MAIN, "name": MAIN, "width": 64, "height": 32, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": ["picture", "square"], "layers": layers
        }]
    })
}

/// The cover at the middle of the left half and of the right half, at Full or at Draft.
fn cover(mode: &str, quality: PreviewQuality) -> (f32, f32) {
    let loaded = persist::load_str(&shot(mode).to_string()).expect("the shot reads");
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Fixtures");
    let mut log = FrameLog::new(8);
    let frame = match quality {
        PreviewQuality::Full => render_frame(loaded.document.project(), &Id::new(MAIN), 0, &root, 64, &mut log),
        _ => preview_frame(loaded.document.project(), &Id::new(MAIN), 0, &root, quality, 64, &mut log),
    }
    .expect("the frame draws");
    assert!(log.finish().is_empty(), "the shot draws without a word");
    let (w, h) = (frame.width(), frame.height());
    (frame.pixel(w / 4, h / 2)[3], frame.pixel(3 * w / 4, h / 2)[3])
}

#[test]
fn each_mode_covers_by_its_own_rule() {
    for (mode, left, right) in [
        ("alpha", 1.0, 0.0),
        ("alpha_inverted", 0.0, 1.0),
        ("luma", 0.5, 0.0),
        ("luma_inverted", 0.5, 1.0),
    ] {
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            let (l, r) = cover(mode, quality);
            assert!((l - left).abs() < 1e-4 && (r - right).abs() < 1e-4, "{mode} {quality:?}: ({l}, {r})");
        }
    }
}

#[test]
fn the_file_keeps_the_mode_and_refuses_one_it_does_not_know() {
    let loaded = persist::load_str(&shot("luma_inverted").to_string()).unwrap();
    let saved = persist::to_json(loaded.document.project(), &loaded.preserved);
    let again: J = serde_json::from_str(&saved).unwrap();
    assert_eq!(again["compositions"][0]["layers"][0]["matte"]["mode"], "luma_inverted");
    let refused = persist::load_str(&shot("lumen").to_string()).err().expect("an unknown mode is refused");
    assert!(refused.detail.contains("alpha, alpha_inverted, luma, luma_inverted"), "{}", refused.detail);
}
