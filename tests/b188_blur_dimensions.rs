//! B-188: D-303, Gaussian Blur's Blur Dimensions: both, horizontal or vertical.
//!
//! P-26, tutorials 3 (Colorful Glitch) and 4 (Lightsaber) blur along one axis only, for streaks;
//! here a Gaussian Blur blurred both ways. Worked by hand on one opaque white pixel at (8, 8) of
//! a 16x16 frame, sigma 1, whose taps `w[0..=6]` are document 21's for radius 3:
//! - horizontal: pixel (8 + d, 8) is `w[3 + d]` covering, every other row clear;
//! - vertical: pixel (8, 8 + d) is `w[3 + d]`, every other column clear;
//! - both: pixel (8 + dx, 8 + dy) is `w[3 + dx] w[3 + dy]`, as before D-303.

use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::gaussian_weights;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};

const MAIN: &str = "comp-main";

fn shot(extra: J) -> String {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let mut parameters = json!({"sigma_px": 1});
    for (k, v) in extra.as_object().unwrap() {
        parameters[k] = v.clone();
    }
    json!({
        "schema_version": 0, "project_id": "proj-b188",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": MAIN, "name": MAIN, "width": 16, "height": 16, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": ["dot"],
            "layers": [{
                "id": "dot", "kind": "solid", "name": "dot", "enabled": true, "locked": false,
                "in_frame": 0, "out_frame": 1,
                "solid": {"color": [1, 1, 1], "width": 1, "height": 1},
                "transform": {
                    "anchor": t(json!([0.5, 0.5])), "position": t(json!([8.5, 8.5])),
                    "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                },
                "masks": [], "matte": null, "blend_mode": "normal",
                "effects": [{"instance_id": "f", "type_id": "core.gaussian_blur", "enabled": true, "parameters": parameters}]
            }]
        }]
    })
    .to_string()
}

/// Each pixel's alpha, row by row.
fn frame(extra: J) -> Vec<f32> {
    let loaded = persist::load_str(&shot(extra)).expect("the shot reads");
    let mut log = FrameLog::new(8);
    let f = render_frame(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), 256, &mut log)
        .expect("the frame draws");
    (0..16).flat_map(|y| (0..16).map(move |x| (x, y))).map(|(x, y)| f.pixel(x, y)[3]).collect()
}

/// The tap `d` pixels from the dot, or 0 past the radius.
fn tap(d: i32) -> f64 {
    let w = gaussian_weights(1.0);
    assert_eq!(w.len(), 7, "sigma 1 reaches 3 pixels");
    if d.abs() > 3 { 0.0 } else { w[(3 + d) as usize] as f64 }
}

#[test]
fn each_dimension_spreads_the_dot_along_its_own_axes() {
    let cases: [(&str, fn(i32, i32) -> f64); 3] = [
        ("horizontal", |dx, dy| if dy == 0 { tap(dx) } else { 0.0 }),
        ("vertical", |dx, dy| if dx == 0 { tap(dy) } else { 0.0 }),
        ("both", |dx, dy| tap(dx) * tap(dy)),
    ];
    for (dims, want) in cases {
        let got = frame(json!({"dimensions": dims}));
        for (i, a) in got.iter().enumerate() {
            let (dx, dy) = ((i % 16) as i32 - 8, (i / 16) as i32 - 8);
            assert!((*a as f64 - want(dx, dy)).abs() < 1e-6, "{dims}, pixel ({}, {}): {a} against {}", dx + 8, dy + 8, want(dx, dy));
        }
    }
    assert_eq!(frame(json!({})), frame(json!({"dimensions": "both"})), "a file without it blurs both ways, as before");
    // Repeating the edge of a one-pixel layer one way only still draws the dot alone, the taps
    // summing to 1 as 32-bit numbers do.
    let held = frame(json!({"dimensions": "horizontal", "edges": "repeat"}));
    for (i, a) in held.iter().enumerate() {
        let want = if i == 8 * 16 + 8 { 1.0 } else { 0.0 };
        assert!((*a - want).abs() < 1e-6, "repeat, pixel {i}: {a} against {want}");
    }
}

#[test]
fn the_card_hands_a_one_way_blur_to_the_cpu_and_draws_the_same() {
    let Ok(mut gpu) = Gpu::new() else {
        eprintln!("no usable card; nothing to check");
        return;
    };
    let loaded = persist::load_str(&shot(json!({"dimensions": "horizontal"}))).expect("the shot reads");
    let mut log = FrameLog::new(8);
    let mut cache = CelCache::viewer();
    let (px, w, _) = preview::preview_frame_srgb8(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), PreviewQuality::Full, 256, &mut log, &mut cache, &mut gpu)
        .expect("the frame draws");
    for y in 0..16 {
        let lit = (0..16).filter(|x| px[(y * w + x) * 4 + 3] > 0).count();
        assert_eq!(lit, if y == 8 { 7 } else { 0 }, "row {y}: only the dot's row is lit, across the radius");
    }
}

#[test]
fn the_file_keeps_it_and_reports_a_wrong_one() {
    let saved = |extra: J| {
        let loaded = persist::load_str(&shot(extra)).expect("the shot reads");
        let saved: J = serde_json::from_str(&persist::to_json(loaded.document.project(), &loaded.preserved)).unwrap();
        saved["compositions"][0]["layers"][0]["effects"][0]["parameters"].clone()
    };
    assert!(saved(json!({})).get("dimensions").is_none(), "both is not written, so the file saves as before");
    assert_eq!(saved(json!({"dimensions": "both"}))["dimensions"], "both", "a file that wrote it keeps it");
    assert_eq!(saved(json!({"dimensions": "vertical"}))["dimensions"], "vertical", "vertical is kept");
    let loaded = persist::load_str(&shot(json!({"dimensions": "diagonal"}))).expect("the shot reads");
    let e = &loaded.document.project().compositions[0].layer(&Id::new("dot")).unwrap().effects[0].effect;
    assert_eq!(e.why_invalid(), "Gaussian Blur's dimensions are \"both\", \"horizontal\" or \"vertical\", and this is \"diagonal\".");
}
