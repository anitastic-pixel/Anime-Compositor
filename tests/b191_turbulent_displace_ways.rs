//! B-191: D-306, Turbulent Displace's Displacement and Pinning.
//!
//! P-26, tutorials 4 and 5 (the lightsabers) push a blade's glow sideways only and keep a
//! layer's edges still with After Effects' Turbulent Displace; here it always pushed both ways
//! and everywhere. Worked on a 64x64 solid in the middle of a 96x96 frame (frame columns and
//! rows 16 to 79), pushed up to 8 pixels, 30 pixels a cell:
//! - a solid whose bottom half Linear Wipe cleared is the same along every row, so pushing it
//!   sideways only changes nothing away from its sides, where pushing both ways moves its edge;
//! - pushed both ways, the solid's outer pixels read past its edge and lose some covering;
//!   pinned, they stay put; its middle, more than 30 pixels from every edge, is pushed the same.

#![recursion_limit = "256"]

use serde_json::json;

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::WorkingBuffer as Frame;

const MAIN: &str = "comp-main";

/// The frame, with the wipe at `wipe`% and, when `ways` is given, the displace with those
/// displacement and pinning words.
fn frame(wipe: f64, ways: Option<(&str, &str)>) -> Frame {
    let t = |v: serde_json::Value| json!({"base": v, "keyframes": []});
    let mut effects = vec![json!({"instance_id": "wipe", "type_id": "core.linear_wipe", "enabled": true,
        "parameters": {"completion": wipe, "angle": 180, "feather": 0}})];
    if let Some((displacement, pinning)) = ways {
        effects.push(json!({"instance_id": "push", "type_id": "core.turbulent_displace", "enabled": true,
            "parameters": {"amount": 8, "size": 30, "complexity": 2, "evolution": 0, "speed": 20, "seed": 0,
                "edges": "transparent", "displacement": displacement, "pinning": pinning}}));
    }
    let shot = json!({
        "schema_version": 0, "project_id": "proj-b191",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": MAIN, "name": MAIN, "width": 96, "height": 96, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": ["art"],
            "layers": [{
                "id": "art", "kind": "solid", "name": "art", "enabled": true, "locked": false,
                "in_frame": 0, "out_frame": 1,
                "solid": {"color": [1, 1, 1], "width": 64, "height": 64},
                "transform": {
                    "anchor": t(json!([32, 32])), "position": t(json!([48, 48])),
                    "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                },
                "masks": [], "matte": null, "blend_mode": "normal", "effects": effects
            }]
        }]
    });
    let loaded = persist::load_str(&shot.to_string()).expect("the shot reads");
    let mut log = FrameLog::new(8);
    render_frame(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), 256, &mut log)
        .expect("the frame draws")
}

fn alpha(f: &Frame, x: usize, y: usize) -> f32 {
    f.pixel(x, y)[3]
}

#[test]
fn sideways_only_leaves_horizontal_bands_alone() {
    let plain = frame(50.0, None);
    let column: Vec<f32> = (0..96).map(|y| alpha(&plain, 48, y)).collect();
    assert!(column.contains(&0.0) && column.contains(&1.0), "the band is drawn: {column:?}");
    let sideways = frame(50.0, Some(("horizontal", "none")));
    let both = frame(50.0, Some(("turbulent", "none")));
    let mut moved = 0;
    for y in 0..96 {
        for x in 28..68 {
            let (a, s) = (plain.pixel(x, y), sideways.pixel(x, y));
            for c in 0..4 {
                assert!((a[c] - s[c]).abs() < 1e-5, "({x}, {y}) moved sideways: {s:?} against {a:?}");
            }
            moved += usize::from((alpha(&both, x, y) - alpha(&plain, x, y)).abs() > 0.01);
        }
    }
    assert!(moved > 0, "pushed both ways, the band's edge moves");
    // Up and down only, the band's edge moves as well.
    let upright = frame(50.0, Some(("vertical", "none")));
    assert!((0..96).any(|y| (alpha(&upright, 48, y) - column[y]).abs() > 0.01), "pushed up and down, it moves");
}

#[test]
fn pinned_edges_stay_put() {
    let free = frame(0.0, Some(("turbulent", "none")));
    let pinned = frame(0.0, Some(("turbulent", "all")));
    let edge = |f: &Frame| {
        (16..80)
            .flat_map(|i| [(16, i), (79, i), (i, 16), (i, 79)])
            .map(|(x, y)| alpha(f, x, y))
            .fold(1.0f32, f32::min)
    };
    assert!(edge(&free) < 0.9, "unpinned, the edge frays: {}", edge(&free));
    assert!(edge(&pinned) > 0.99, "pinned, it does not: {}", edge(&pinned));
    // Nothing is pushed past the drawing's edge when pinned.
    assert!((0..96).all(|i| alpha(&pinned, 15, i) < 0.01 && alpha(&pinned, i, 80) < 0.01), "nothing spills");
    assert_eq!(free.pixel(48, 48), pinned.pixel(48, 48), "the middle is pushed the same");
}

#[test]
fn the_file_keeps_the_ways_and_reports_a_wrong_word() {
    let effect = |p: serde_json::Value| {
        let mut parameters = json!({"amount": 8, "size": 30, "complexity": 2, "evolution": 0, "speed": 20, "seed": 0,
            "edges": "transparent"});
        for (k, v) in p.as_object().unwrap() {
            parameters[k] = v.clone();
        }
        parameters
    };
    // Through a project made from a frame's file: start from the plain shot's text.
    let shot = |p: serde_json::Value| {
        json!({
            "schema_version": 0, "project_id": "proj-b191",
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
                    "solid": {"color": [1, 1, 1], "width": 4, "height": 4},
                    "transform": {
                        "anchor": {"base": [2, 2], "keyframes": []}, "position": {"base": [4, 4], "keyframes": []},
                        "scale": {"base": [100, 100], "keyframes": []}, "rotation": {"base": 0, "keyframes": []},
                        "opacity": {"base": 1, "keyframes": []}
                    },
                    "masks": [], "matte": null, "blend_mode": "normal",
                    "effects": [{"instance_id": "push", "type_id": "core.turbulent_displace", "enabled": true,
                        "parameters": effect(p)}]
                }]
            }]
        })
        .to_string()
    };
    let saved = |p: serde_json::Value| {
        let loaded = persist::load_str(&shot(p)).expect("the shot reads");
        let s: serde_json::Value =
            serde_json::from_str(&persist::to_json(loaded.document.project(), &loaded.preserved)).unwrap();
        s["compositions"][0]["layers"][0]["effects"][0]["parameters"].clone()
    };
    let old = saved(json!({}));
    assert!(old.get("displacement").is_none() && old.get("pinning").is_none(), "an old file stays as it was: {old}");
    let new = saved(json!({"displacement": "vertical", "pinning": "all"}));
    assert_eq!((&new["displacement"], &new["pinning"]), (&json!("vertical"), &json!("all")));
    let loaded = persist::load_str(&shot(json!({"pinning": "some"}))).expect("the shot reads");
    let e = &loaded.document.project().compositions[0].layer(&Id::new("art")).unwrap().effects[0].effect;
    assert_eq!(e.why_invalid(), "Turbulent Displace's pinning is \"none\" or \"all\", and this is \"some\".");
}
