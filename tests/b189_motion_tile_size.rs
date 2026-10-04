//! B-189: D-304, Motion Tile's Tile Center, Tile Width and Tile Height.
//!
//! P-26, tutorial 3 (Colorful Glitch) shrinks Motion Tile's tiles to 28 per cent across to
//! repeat a grid; here a tile was always the drawing at its own size. Worked by hand on an 8x8
//! white solid whose left half Linear Wipe clears (completion 50, angle 90, feather 0), so the
//! drawing's columns 0 to 3 are clear and 4 to 7 solid, and every row alike:
//! - tile width 50: each tile is 4 wide, set round the middle (columns 2 to 5), two output
//!   points a pixel; a row reads 1 1 0 0 1 1 0 0;
//! - with mirror, the tiles either side turn over: 0 0 0 0 1 1 1 1;
//! - the centre at 25 per cent puts a tile at columns 0 to 3: 0 0 1 1 0 0 1 1;
//! - output width 200 grows 4 a side, and the same 4-wide tiles carry on: 16 pixels of
//!   1 1 0 0;
//! - tile width 200 shows the drawing's middle half doubled, one bilinear point a pixel:
//!   column `j` reads the drawing at `j / 2 + 2.25`, so 0 0 0 0.25 0.75 1 1 1.
//! Tile height works the same down, checked with the wipe turned to 180.

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

fn shot(angle: f64, tile: J) -> String {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let mut parameters = json!({"output_width": 100, "output_height": 100, "mirror": "off"});
    for (k, v) in tile.as_object().unwrap() {
        parameters[k] = v.clone();
    }
    json!({
        "schema_version": 0, "project_id": "proj-b189",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": MAIN, "name": MAIN, "width": 16, "height": 16, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": ["art"],
            "layers": [{
                "id": "art", "kind": "solid", "name": "art", "enabled": true, "locked": false,
                "in_frame": 0, "out_frame": 1,
                "solid": {"color": [1, 1, 1], "width": 8, "height": 8},
                "transform": {
                    "anchor": t(json!([4, 4])), "position": t(json!([8, 8])),
                    "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                },
                "masks": [], "matte": null, "blend_mode": "normal",
                "effects": [
                    {"instance_id": "wipe", "type_id": "core.linear_wipe", "enabled": true,
                     "parameters": {"completion": 50, "angle": angle, "feather": 0}},
                    {"instance_id": "tile", "type_id": "core.motion_tile", "enabled": true, "parameters": parameters}
                ]
            }]
        }]
    })
    .to_string()
}

/// Alpha of the frame's row 8 (`angle` 90) or column 8 (`angle` 180), from `from` for `n` pixels.
fn line(angle: f64, tile: J, from: usize, n: usize) -> Vec<f64> {
    let loaded = persist::load_str(&shot(angle, tile)).expect("the shot reads");
    let mut log = FrameLog::new(8);
    let f = render_frame(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), 256, &mut log)
        .expect("the frame draws");
    (from..from + n)
        .map(|i| if angle == 90.0 { f.pixel(i as _, 8) } else { f.pixel(8, i as _) }[3] as f64)
        .collect()
}

fn near(got: &[f64], want: &[f64], what: &str) {
    assert_eq!(got.len(), want.len());
    for (i, (g, w)) in got.iter().zip(want).enumerate() {
        assert!((g - w).abs() < 1e-6, "{what}, pixel {i}: {got:?} against {want:?}");
    }
}

#[test]
fn a_sized_tile_repeats_the_drawing_shrunk_or_grown_round_its_centre() {
    // The drawing sits at frame columns 4 to 11.
    near(&line(90.0, json!({}), 4, 8), &[0., 0., 0., 0., 1., 1., 1., 1.], "the drawing, as before");
    near(&line(90.0, json!({"tile_width": 50}), 4, 8), &[1., 1., 0., 0., 1., 1., 0., 0.], "width 50");
    near(&line(90.0, json!({"tile_width": 50, "mirror": "on"}), 4, 8), &[0., 0., 0., 0., 1., 1., 1., 1.], "width 50, mirror");
    near(&line(90.0, json!({"tile_width": 50, "tile_center": [25, 50]}), 4, 8), &[0., 0., 1., 1., 0., 0., 1., 1.], "centre 25");
    near(&line(90.0, json!({"tile_width": 50, "output_width": 200}), 0, 16), &[1., 1., 0., 0.].repeat(4), "output 200");
    near(&line(90.0, json!({"tile_width": 200}), 4, 8), &[0., 0., 0., 0.25, 0.75, 1., 1., 1.], "width 200");
    near(&line(180.0, json!({"tile_height": 50}), 4, 8), &[1., 1., 0., 0., 1., 1., 0., 0.], "height 50, down");
    // Width alone leaves the column untouched.
    near(&line(180.0, json!({"tile_width": 50}), 4, 8), &[0., 0., 0., 0., 1., 1., 1., 1.], "width 50, down");
}

#[test]
fn the_card_hands_a_sized_tile_to_the_cpu_and_draws_the_same() {
    let Ok(mut gpu) = Gpu::new() else {
        eprintln!("no usable card; nothing to check");
        return;
    };
    let loaded = persist::load_str(&shot(90.0, json!({"tile_width": 50}))).expect("the shot reads");
    let mut log = FrameLog::new(8);
    let mut cache = CelCache::viewer();
    let (px, w, _) = preview::preview_frame_srgb8(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), PreviewQuality::Full, 256, &mut log, &mut cache, &mut gpu)
        .expect("the frame draws");
    let row: Vec<u8> = (4..12).map(|x| px[(8 * w + x) * 4 + 3]).collect();
    assert_eq!(row, [255, 255, 0, 0, 255, 255, 0, 0], "row 8");
}

#[test]
fn the_file_keeps_them_only_when_set() {
    let saved = |tile: J| {
        let loaded = persist::load_str(&shot(90.0, tile)).expect("the shot reads");
        let saved: J = serde_json::from_str(&persist::to_json(loaded.document.project(), &loaded.preserved)).unwrap();
        saved["compositions"][0]["layers"][0]["effects"][1]["parameters"].clone()
    };
    let p = saved(json!({}));
    for k in ["tile_center", "tile_width", "tile_height"] {
        assert!(p.get(k).is_none(), "{k} at its start is not written, so the file saves as before");
    }
    let p = saved(json!({"tile_center": [25, 40], "tile_width": 28}));
    assert_eq!((&p["tile_center"], &p["tile_width"]), (&json!([25, 40]), &json!(28)), "set ones are kept");
    assert!(p.get("tile_height").is_none(), "one never set stays unwritten");
    assert_eq!(saved(json!({"tile_width": 100}))["tile_width"], 100, "a file that wrote it keeps it");
}
