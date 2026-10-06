//! B-211: D-329, Lightning Bolt going round shapes and a negative Alpha Obstacle, from P-26.
//!
//! After Effects' Advanced Lightning goes round what blocks it, and a negative Alpha Obstacle
//! keeps it inside a shape; D-324's bolt could only stop. Every expected pixel is
//! `Fixtures/lightning_around/expected_lightning_around.json`, written by
//! `tools/lightning_around_reference.py` before this code existed.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, set, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::model::Id;
use anime_compositor::{persist, png_out, OutputDepth};

/// `base` with `path` and Alpha Obstacle `obstacle`.
fn around(base: &Effect, path: &str, obstacle: f64) -> Effect {
    let mut e = base.clone();
    if let Effect::LightningBolt { path: p, obstacle: o, .. } = &mut e {
        (*p, *o) = (path.to_string(), obstacle);
    }
    e
}

const SIZE: (usize, usize) = (480, 270);
const ROCK: [u8; 4] = [70, 52, 40, 255];

/// A night sky, a round rock in the middle of it, and a ring of rock, each a drawing of its own.
fn drawings() -> [Vec<u8>; 3] {
    let (w, h) = SIZE;
    let (mut sky, mut disc, mut ring) = (Vec::new(), Vec::new(), Vec::new());
    for y in 0..h {
        for x in 0..w {
            let (fx, fy) = (x as f64 + 0.5, y as f64 + 0.5);
            let t = y as f64 / h as f64;
            sky.extend([(26.0 + 30.0 * t) as u8, (28.0 + 22.0 * t) as u8, (58.0 + 40.0 * t) as u8, 255]);
            let r = (fx - 240.0).hypot(fy - 135.0);
            disc.extend(if r <= 70.0 { ROCK } else { [0; 4] });
            ring.extend(if (80.0..=110.0).contains(&r) { ROCK } else { [0; 4] });
        }
    }
    [sky, disc, ring]
}

/// The sky, and over it the drawing `rock` carrying `parameters`' bolt, at frame 0.
fn picture(dir: &Path, rock: &str, parameters: Option<J>) -> (Vec<u8>, Vec<String>) {
    let (w, h) = SIZE;
    let t = |v: J| json!({"base": v, "keyframes": []});
    let layer = |id: &str, effects: J| {
        json!({
            "id": id, "kind": "raster", "name": id, "asset_id": format!("asset-{id}"), "enabled": true,
            "locked": false, "in_frame": 0, "out_frame": 24, "source_offset_frames": 0,
            "transform": {
                "anchor": t(json!([w as f64 / 2.0, h as f64 / 2.0])), "position": t(json!([w as f64 / 2.0, h as f64 / 2.0])),
                "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
            },
            "exposure_spans": [], "mask": null, "matte": null, "blend_mode": "normal", "effects": effects
        })
    };
    let asset = |id: &str| {
        json!({"id": format!("asset-{id}"), "kind": "still", "name": id, "path": format!("{id}.png"),
            "interpretation": {"color_space": "srgb", "alpha": "straight"}})
    };
    let effects = parameters.map_or(json!([]), |p| {
        json!([{"instance_id": "fx-0-0", "type_id": "core.lightning_bolt", "enabled": true, "parameters": p}])
    });
    let project = json!({
        "schema_version": 0, "project_id": "proj-b211-picture",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [asset("sky"), asset(rock)],
        "compositions": [{
            "id": MAIN, "name": "Main", "width": w, "height": h, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 24,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 24},
            "layer_order": ["sky", rock],
            "layers": [layer("sky", json!([])), layer(rock, effects)]
        }]
    });
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
}

#[test]
fn b211_lightning_around() {
    let mut t = Table::new(
        "lightning_around",
        "# B-211: Lightning Bolt round shapes and a negative Alpha Obstacle\n\nD-329, from P-26's \
         tutorial 2. Every expected pixel is `Fixtures/lightning_around/expected_lightning_around.json`, \
         written by `tools/lightning_around_reference.py` before this code existed and printed in \
         document 25 as FX-LIGHTA-001 to 014. The build's frame is compared sample by sample; the \
         answer is the largest difference over all of them, against the catalogue's tolerance of \
         2e-5. D-324's FX-LIGHTX-001 to 024 are checked again, unchanged, by B-203, and D-190's \
         FX-BOLT-001 to 032 by B-126.\n",
    );

    t.heading("FX-LIGHTA-001 to 014 (document 25)");
    t.fixtures("expected_lightning_around.json");

    t.heading("The file");
    let files: Vec<String> = (1..=14).map(|n| format!("fx_lighta_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let old = persist::load(&effect_table::repo("Fixtures/lightning_extras/fx_lightx_004.json")).unwrap();
    let saved = effect_table::saved(&old);
    let params = &saved["compositions"][0]["layers"][0]["effects"][0]["parameters"];
    t.row(
        "fx_lightx_004.json, a file from before D-329, is saved without `path`",
        &format!("{:?}", params.get("path")),
        params.get("path").is_none(),
    );
    let mut document = t.load("fx_lighta_001.json").document;
    let base = document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    let wrong = t.load("fx_lighta_013.json");
    let why = wrong.document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.why_invalid();
    t.row(
        "fx_lighta_013.json's path \"sideways\" is named in a sentence",
        &why,
        why == "Lightning Bolt's path at an obstacle is \"split\" or \"around\", and this is \"sideways\".",
    );
    t.shape_refused(
        "fx_lighta_001.json",
        "a path that is a number",
        r##"{"start": [50, 0], "end": [50, 100], "jagged": 40, "detail": 6, "branches": 0, "width": 1, "glow": 0,
            "opacity": 100, "hold": 1, "seed": 0, "color": "#ffffff", "glow_color": "#6e8cff", "path": 3}"##,
    );

    t.heading("How far it reaches");
    let mut draft = around(&base, "around", -50.0);
    let full = draft.clone();
    draft.scale_distances(|d| d * 0.5);
    let mut halved = full.clone();
    if let Effect::LightningBolt { width, glow, .. } = &mut halved {
        (*width, *glow) = (*width / 2.0, *glow / 2.0);
    }
    t.row(
        "a half-size draft preview halves the width and the glow, and neither new setting",
        &format!("{draft:?}"),
        draft == halved,
    );
    let grows = around(&base, "around", -100.0).bounds_expansion();
    t.row("going round, it never grows the drawing's bounds", &grows.to_string(), grows == 0);

    t.heading("Commands");
    t.refused(
        &mut document,
        vec![
            ("path \"sideways\"", set(around(&base, "sideways", 50.0))),
            ("Alpha Obstacle -101", set(around(&base, "split", -101.0))),
            ("Alpha Obstacle keyed to -150", keys("obstacle", &[(0, &[0.0]), (4, &[-150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_lighta_001.json",
        vec![
            ("path around,", set(around(&base, "around", 50.0))),
            ("Alpha Obstacle -100,", set(around(&base, "split", -100.0))),
            ("Alpha Obstacle keyed from 100 to -100", keys("obstacle", &[(0, &[100.0]), (4, &[-100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_lighta_004.json", 0), ("fx_lighta_005.json", 0), ("fx_lighta_010.json", 0)]);

    t.heading("Pictures: a night sky with rock over it, in `verification/D-329 pictures/`");
    let dir = effect_table::repo("verification/D-329 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = SIZE;
    let [sky, disc, ring] = drawings();
    for (name, bytes) in [("sky", &sky), ("disc", &disc), ("ring", &ring)] {
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], bytes).unwrap();
    }
    let bolt = |start: [f64; 2], end: [f64; 2], obstacle: f64, path: &str| {
        Some(json!({"start": start, "end": end, "jagged": 40, "detail": 6, "branches": 30,
            "width": 1.5, "glow": 8, "opacity": 100, "hold": 2, "seed": 0, "color": "#ffffff",
            "glow_color": "#6e8cff", "obstacle": obstacle, "path": path}))
    };
    let lit = |b: &[u8], before: &[u8], x: usize, y: usize| b[(y * w + x) * 4..][..4] != before[(y * w + x) * 4..][..4];
    let count = |b: &[u8], before: &[u8], keep: &dyn Fn(f64) -> bool| {
        (0..w * h).filter(|&i| keep((((i % w) as f64 + 0.5) - 240.0).hypot((i / w) as f64 + 0.5 - 135.0)) && lit(b, before, i % w, i / w)).count()
    };
    let (disc_before, said) = picture(&dir, "disc", None);
    png_out::write_rgba(&dir.join("disc_before.png"), w, h, OutputDepth::Eight, &[], &disc_before).unwrap();
    t.row("disc_before.png, the sky with the round rock, draws cleanly", &format!("{said:?}"), said.is_empty());
    let mut shots = Vec::new();
    for (name, what, rock, params) in [
        ("1_disc_stop", "the bolt from the top edge to the bottom, Alpha Obstacle 50, Stop: it ends on the rock", "disc", bolt([50.0, 0.0], [50.0, 100.0], 50.0, "split")),
        ("2_disc_go_round", "the same with Go Round: the bolt goes round the rock and on to the bottom edge", "disc", bolt([50.0, 0.0], [50.0, 100.0], 50.0, "around")),
    ] {
        let (bytes, said) = picture(&dir, rock, params);
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let bottom = (0..w).filter(|&x| lit(&bytes, &disc_before, x, h - 1)).count();
        let inside = count(&bytes, &disc_before, &|r| r <= 60.0);
        t.row(
            &format!("{name}.png, {what}"),
            &format!("{said:?}, {bottom} of the bottom row lit, {inside} lit more than 10 pixels inside the rock"),
            said.is_empty() && inside == 0 && (bottom > 0) == (name == "2_disc_go_round"),
        );
        shots.push(bytes);
    }
    let (ring_before, said) = picture(&dir, "ring", None);
    png_out::write_rgba(&dir.join("ring_before.png"), w, h, OutputDepth::Eight, &[], &ring_before).unwrap();
    t.row("ring_before.png, the sky with the ring of rock, draws cleanly", &format!("{said:?}"), said.is_empty());
    // From the ring's left side to its right, both inside the rock, 95 pixels from the middle.
    let (left, right) = ([(240.0 - 95.0) / 4.8, 50.0], [(240.0 + 95.0) / 4.8, 50.0]);
    for (name, what, params) in [
        ("3_ring_inside_stop", "a bolt from the ring's left side to its right, Alpha Obstacle -50, Stop: it ends where it leaves the rock", bolt(left, right, -50.0, "split")),
        ("4_ring_inside_go_round", "the same with Go Round: the bolt follows the ring round to its right side, never crossing the hole", bolt(left, right, -50.0, "around")),
    ] {
        let (bytes, said) = picture(&dir, "ring", params);
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let hole = count(&bytes, &ring_before, &|r| r <= 70.0);
        let outside = count(&bytes, &ring_before, &|r| r >= 120.0);
        let far = (0..h).any(|y| (330..w).any(|x| lit(&bytes, &ring_before, x, y)));
        t.row(
            &format!("{name}.png, {what}"),
            &format!("{said:?}, {hole} lit more than 10 pixels into the hole, {outside} more than 10 outside the ring, right side reached: {far}"),
            said.is_empty() && hole == 0 && outside == 0 && far == (name == "4_ring_inside_go_round"),
        );
        shots.push(bytes);
    }
    t.row(
        "Stop and Go Round draw different pictures, on the disc and on the ring",
        &format!("{} pictures", shots.len()),
        shots[0] != shots[1] && shots[2] != shots[3],
    );

    t.finish("D-329_lightning_around_table.md");
}
