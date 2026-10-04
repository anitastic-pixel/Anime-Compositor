//! B-196: D-315, Displacement Map's Expand Output.
//!
//! P-26, tutorial 3 (Colorful Glitch) pushes slices of a picture past the layer's edge with
//! After Effects' Expand Output; here the push stopped at the edge. A white 8 by 6 solid at
//! (4, 2) in a 16 by 10 composition is pushed 4 pixels right by a map that is red all over.

use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::Id;
use anime_compositor::persist;

fn solid(id: &str, enabled: bool, color: [f64; 3], (w, h): (usize, usize), at: [f64; 2], effects: J) -> J {
    json!({
        "id": id, "kind": "solid", "name": id, "enabled": enabled, "locked": false,
        "in_frame": 0, "out_frame": 1,
        "transform": {
            "anchor": {"base": [0, 0], "keyframes": []},
            "position": {"base": at, "keyframes": []},
            "scale": {"base": [100, 100], "keyframes": []},
            "rotation": {"base": 0, "keyframes": []},
            "opacity": {"base": 1, "keyframes": []}
        },
        "mask": null, "matte": null, "blend_mode": "normal", "effects": effects,
        "solid": {"color": color, "width": w, "height": h}
    })
}

fn project(params: J) -> J {
    let effect = json!([{"instance_id": "fx-1", "type_id": "core.displacement_map", "enabled": true, "parameters": params}]);
    json!({
        "schema_version": 0,
        "project_id": "proj-b196",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": "comp-main", "name": "comp-main", "width": 16, "height": 10, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 1,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": ["red", "card"],
            "layers": [
                solid("red", false, [1.0, 0.0, 0.0], (16, 10), [0.0, 0.0], json!([])),
                solid("card", true, [1.0, 1.0, 1.0], (8, 6), [4.0, 2.0], effect),
            ]
        }]
    })
}

fn params(expand: Option<&str>, wrap: &str) -> J {
    let mut p = json!({"layer": "red", "fit": "stretch", "horizontal": "red", "max_horizontal": -4,
        "vertical": "off", "max_vertical": 0, "wrap": wrap});
    if let Some(e) = expand {
        p["expand"] = J::from(e);
    }
    p
}

/// The alpha of each pixel of row 4, as bytes.
fn row(params: J) -> Vec<u8> {
    let loaded = persist::load_str(&project(params).to_string()).expect("the project reads");
    let mut log = FrameLog::new(3);
    let dir = std::env::temp_dir();
    let drawn = render_frame(loaded.document.project(), &Id::new("comp-main"), 0, &dir, 64, &mut log).expect("it draws");
    assert!(log.finish().is_empty(), "nothing to report");
    drawn.to_srgb8_straight().chunks(4).skip(4 * 16).take(16).map(|p| p[3]).collect()
}

#[test]
fn expand_output_carries_the_push_past_the_edge() {
    let (o, x) = (0u8, 255u8);
    // Off, as before D-315: the card moves right inside its own 8 pixels and is cut at 12.
    let kept = [o, o, o, o, o, o, o, o, x, x, x, x, o, o, o, o];
    assert_eq!(row(params(None, "off")), kept);
    assert_eq!(row(params(Some("off"), "off")), kept);
    // On: the card's last 4 pixels land on 12 to 15.
    assert_eq!(row(params(Some("on"), "off")), [o, o, o, o, o, o, o, o, x, x, x, x, x, x, x, x]);
    // Wrapping, the push comes round inside the card, and expand changes nothing.
    let wrapped = row(params(None, "on"));
    assert_eq!(wrapped, [o, o, o, o, x, x, x, x, x, x, x, x, o, o, o, o]);
    assert_eq!(row(params(Some("on"), "on")), wrapped);
}

#[test]
fn expand_output_is_saved_only_when_on_and_a_wrong_word_is_reported() {
    let saved = |p: J| {
        let loaded = persist::load_str(&project(p).to_string()).expect("the project reads");
        let text = persist::to_json(loaded.document.project(), &loaded.preserved);
        let back: J = serde_json::from_str(&text).unwrap();
        back["compositions"][0]["layers"][1]["effects"][0]["parameters"].clone()
    };
    assert_eq!(saved(params(None, "off")).get("expand"), None, "a file without it stays without it");
    assert_eq!(saved(params(Some("on"), "off"))["expand"], "on");
    assert_eq!(saved(params(Some("off"), "off"))["expand"], "off", "one that had it keeps it");

    let loaded = persist::load_str(&project(params(Some("sideways"), "off")).to_string()).expect("the project reads");
    let mut log = FrameLog::new(3);
    let dir = std::env::temp_dir();
    render_frame(loaded.document.project(), &Id::new("comp-main"), 0, &dir, 64, &mut log).expect("it draws");
    let said: Vec<String> = log.finish().iter().map(|d| d.message.clone()).collect();
    assert!(said.iter().any(|m| m.contains("not drawn")), "{said:?}");
    let e = &loaded.document.project().compositions[0].layer(&Id::new("card")).unwrap().effects[0].effect;
    assert_eq!(e.why_invalid(), "Displacement Map's expand output is \"off\" or \"on\", and this is \"sideways\".");
}
