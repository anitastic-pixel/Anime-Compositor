//! B-950: D-1070, Motion Tile's Phase and Horizontal Phase Shift (lHQvi94dpvM 4:00, G6 of the
//! wave 12 report). Worked by hand as B-189's: an 8x8 white solid half cleared by Linear Wipe,
//! tiled at 50 per cent across and down, so tiles are 4 pixels and a line reads 1 1 0 0 1 1 0 0.
//! The tile round the middle covers the drawing's columns (or rows) 2 to 5; the drawing's 0, 1,
//! 6 and 7 are in the tiles either side, the odd ones, which Phase 180 slides by half a tile, 2
//! pixels, so they read 0 0 1 1 0 0 1 1.

#![recursion_limit = "256"]

mod effect_table;

use effect_table::{repo, Table};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth, WorkingBuffer};

const MAIN: &str = "comp-main";

/// A `side`-pixel white solid in a `comp`-pixel comp, wiped at `angle` (90 clears the left half,
/// 180 the top), then Motion Tile with `tile` over its defaults.
fn shot(comp: usize, side: usize, angle: f64, tile: &J) -> String {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let mut parameters = json!({"output_width": 100, "output_height": 100, "mirror": "off"});
    for (k, v) in tile.as_object().unwrap() {
        parameters[k] = v.clone();
    }
    json!({
        "schema_version": 0, "project_id": "proj-b950",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": MAIN, "name": MAIN, "width": comp, "height": comp, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": ["art"],
            "layers": [{
                "id": "art", "kind": "solid", "name": "art", "enabled": true, "locked": false,
                "in_frame": 0, "out_frame": 1,
                "solid": {"color": [1, 1, 1], "width": side, "height": side},
                "transform": {
                    "anchor": t(json!([side / 2, side / 2])), "position": t(json!([comp / 2, comp / 2])),
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

fn draw(comp: usize, side: usize, angle: f64, tile: &J) -> WorkingBuffer {
    let loaded = persist::load_str(&shot(comp, side, angle, tile)).expect("the shot reads");
    let mut log = FrameLog::new(8);
    render_frame(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), 256, &mut log)
        .expect("the frame draws")
}

/// The small shot's alpha along the drawing's column `c` (`down`) or row `c`, its 8 pixels.
fn line(angle: f64, tile: &J, down: bool, c: usize) -> Vec<f64> {
    let f = draw(16, 8, angle, tile);
    (4..12)
        .map(|i| if down { f.pixel(4 + c, i) } else { f.pixel(i, 4 + c) }[3] as f64)
        .collect()
}

fn show(v: &[f64]) -> String {
    v.iter().map(|a| format!("{a}")).collect::<Vec<_>>().join(" ")
}

fn is(v: &[f64], want: [f64; 8]) -> bool {
    v.iter().zip(want).all(|(a, b)| (a - b).abs() < 1e-6)
}

const EVEN: [f64; 8] = [1., 1., 0., 0., 1., 1., 0., 0.];
const ODD: [f64; 8] = [0., 0., 1., 1., 0., 0., 1., 1.];

#[test]
fn b950_motion_tile_phase() {
    let mut t = Table::new(
        "motion_tile",
        "# D-1070: Motion Tile, Phase and Horizontal Phase Shift\n\nB-950. An 8 by 8 white solid, \
         half cleared by Linear Wipe, tiled at 50 per cent across and down: each tile is 4 pixels, \
         and a line through it reads 1 1 0 0 1 1 0 0 (1 is solid, 0 clear). The middle tile covers \
         the drawing's columns and rows 2 to 5; 0, 1, 6 and 7 are in the tiles either side. Phase \
         slides those by Phase / 360 of a tile, so at 180 by 2 pixels: 0 0 1 1 0 0 1 1.\n",
    );
    let half = |p: J| {
        let mut j = json!({"tile_width": 50, "tile_height": 50});
        for (k, v) in p.as_object().unwrap() {
            j[k] = v.clone();
        }
        j
    };

    t.heading("Phase slides every other column down (top half cleared, read down a column)");
    for (p, c, want, what) in [
        (0.0, 0, EVEN, "Phase 0, column 0: as before"),
        (180.0, 4, EVEN, "Phase 180, column 4 (the middle tile): not moved"),
        (180.0, 0, ODD, "Phase 180, column 0: slid down half a tile"),
        (180.0, 7, ODD, "Phase 180, column 7: slid down half a tile"),
        (-180.0, 0, ODD, "Phase -180, column 0: half a tile up, which looks the same"),
        (360.0, 0, EVEN, "Phase 360, column 0: a whole tile, as Phase 0"),
        (90.0, 0, [0., 1., 1., 0., 0., 1., 1., 0.], "Phase 90, column 0: a quarter tile, 1 pixel down"),
    ] {
        let v = line(180.0, &half(json!({"phase": p})), true, c);
        t.row(what, &show(&v), is(&v, want));
    }
    let across = line(90.0, &half(json!({"phase": 180})), false, 0);
    t.row(
        "Phase 180, read across row 0 (left half cleared): rows are not slid sideways",
        &show(&across),
        is(&across, EVEN),
    );

    t.heading("Horizontal Phase Shift on: every other row slides right (left half cleared, read across)");
    for (c, want, what) in [
        (0, ODD, "row 0: slid right half a tile"),
        (3, EVEN, "row 3 (the middle tile): not moved"),
        (6, ODD, "row 6: slid right half a tile"),
    ] {
        let v = line(90.0, &half(json!({"phase": 180, "horizontal_phase_shift": "on"})), false, c);
        t.row(&format!("Phase 180, {what}"), &show(&v), is(&v, want));
    }
    let down = line(180.0, &half(json!({"phase": 180, "horizontal_phase_shift": "on"})), true, 0);
    t.row("Phase 180, read down column 0 (top half cleared): columns are not slid", &show(&down), is(&down, EVEN));
    let mirrored = line(90.0, &half(json!({"phase": 180, "horizontal_phase_shift": "on", "mirror": "on"})), false, 0);
    t.row(
        "with Mirror on, row 0 reads mirror alone's 0 0 0 0 1 1 1 1 with each tile turned over where it lands, 2 pixels on: 1 1 0 0 0 0 1 1",
        &show(&mirrored),
        is(&mirrored, [1., 1., 0., 0., 0., 0., 1., 1.]),
    );

    t.heading("At the drawing's own size, grown, and on the card");
    let f = draw(16, 8, 90.0, &json!({"phase": 180, "horizontal_phase_shift": "on", "output_width": 200, "output_height": 200}));
    let row = |y: usize| (0..16).map(|x| f.pixel(x, y)[3] as f64).collect::<Vec<_>>();
    let (top, middle) = (row(0), row(4));
    t.row(
        "the drawing's own tile, output 200, Phase 180 across: the middle row of tiles reads 1111 0000 1111 0000 (the drawing in the middle)",
        &show(&middle),
        middle == [1., 1., 1., 1., 0., 0., 0., 0., 1., 1., 1., 1., 0., 0., 0., 0.],
    );
    t.row(
        "and the row of tiles above is slid half a tile, 4 pixels: 0000 1111 0000 1111",
        &show(&top),
        top == [0., 0., 0., 0., 1., 1., 1., 1., 0., 0., 0., 0., 1., 1., 1., 1.],
    );
    let card = match Gpu::new() {
        Ok(mut gpu) => {
            let loaded = persist::load_str(&shot(16, 8, 90.0, &half(json!({"phase": 180, "horizontal_phase_shift": "on"})))).unwrap();
            let mut log = FrameLog::new(8);
            let mut cache = CelCache::viewer();
            let (px, w, _) = preview::preview_frame_srgb8(
                loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."),
                PreviewQuality::Full, 256, &mut log, &mut cache, &mut gpu,
            )
            .expect("the frame draws");
            let row: Vec<u8> = (4..12).map(|x| px[(4 * w + x) * 4 + 3]).collect();
            (format!("{row:?}"), row == [0, 0, 255, 255, 0, 0, 255, 255])
        }
        Err(_) => ("no usable card here".to_string(), true),
    };
    t.row("the viewer with the card draws row 0 the same (it hands a Phase to the CPU)", &card.0, card.1);

    t.heading("Saved and refused");
    let saved = |tile: &J| {
        let loaded = persist::load_str(&shot(16, 8, 90.0, tile)).unwrap();
        let j: J = serde_json::from_str(&persist::to_json(loaded.document.project(), &loaded.preserved)).unwrap();
        j["compositions"][0]["layers"][0]["effects"][1]["parameters"].clone()
    };
    let p = saved(&json!({}));
    t.row(
        "a file without them saves without them",
        "no phase words",
        p.get("phase").is_none() && p.get("horizontal_phase_shift").is_none(),
    );
    let p = saved(&json!({"phase": 180, "horizontal_phase_shift": "on"}));
    t.row(
        "set ones are written",
        &format!("phase {}, horizontal {}", p["phase"], p["horizontal_phase_shift"]),
        p["phase"] == 180.0 && p["horizontal_phase_shift"] == "on",
    );
    let bad = persist::load_str(&shot(16, 8, 90.0, &json!({"horizontal_phase_shift": "sideways"}))).unwrap();
    let valid = bad.document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[1].is_valid();
    t.row("an unknown Horizontal Phase Shift word switches the effect off", &format!("runs: {valid}"), !valid);

    picture();
    t.row(
        "bricks.png: a quarter-size tile of a half-cleared square, Phase 0, then 180 with Horizontal Phase Shift, then 90 down",
        "drawn",
        true,
    );
    t.finish("B-950_motion_tile_phase_table.md");
}

/// Three shots of a 240-pixel solid wiped at 45 degrees and tiled at 25 per cent, side by side.
fn picture() {
    let s = 240;
    let shots = [
        json!({"tile_width": 25, "tile_height": 25}),
        json!({"tile_width": 25, "tile_height": 25, "phase": 180, "horizontal_phase_shift": "on"}),
        json!({"tile_width": 25, "tile_height": 25, "phase": 90}),
    ];
    let gap = 16;
    let ow = 3 * s + 2 * gap;
    let mut out = WorkingBuffer::opaque(ow, s);
    for (n, tile) in shots.iter().enumerate() {
        let b = draw(s, s, 45.0, tile);
        for y in 0..s {
            for x in 0..s {
                let p = b.pixel(x, y);
                let o = (y * ow + n * (s + gap) + x) * 4;
                let d = out.data_mut();
                for k in 0..3 {
                    d[o + k] = p[k] + (1.0 - p[3]) * 0.05;
                }
                d[o + 3] = 1.0;
            }
        }
    }
    let dir = repo("verification/B-950 pictures");
    std::fs::create_dir_all(&dir).unwrap();
    png_out::write_rgba(&dir.join("bricks.png"), ow, s, OutputDepth::Eight, &[], &out.to_srgb8_straight()).unwrap();
}
