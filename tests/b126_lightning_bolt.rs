//! B-126: lightning bolt in the core, against D-190.
//!
//! Writes `verification/B-126_lightning_bolt_table.md`.
//!
//! Every expected pixel is `Fixtures/lightning_bolt/expected_lightning_bolt.json`, written by
//! `tools/lightning_bolt_reference.py` before this code existed and printed in document 25 as
//! FX-BOLT-001 to 032. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws a night scene, the proposal's, at a quarter of 1920 by 1080, and strikes it
//! six ways into `verification/B-126 pictures/`.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, set, Table};
use serde_json::Value as J;

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::model::Id;
use anime_compositor::{persist, png_out, OutputDepth};

/// `[start x, start y, end x, end y, jagged, detail, branches, width, glow, opacity, hold, seed]`.
fn bolt(n: [f64; 12], color: &str, glow_color: &str) -> Effect {
    Effect::LightningBolt {
        start: [n[0], n[1]],
        end: [n[2], n[3]],
        jagged: n[4],
        detail: n[5],
        branches: n[6],
        width: n[7],
        glow: n[8],
        opacity: n[9],
        hold: n[10],
        seed: n[11],
        color: color.to_string(),
        glow_color: glow_color.to_string(),
        composite: "on".to_string(),
        kind: "direction".to_string(),
        turbulence: 0.0,
        decay: 0.0,
        conductivity: 0.0,
        obstacle: 0.0,
        frame: 0,
    }
}

const START: [f64; 12] = [40.0, 0.0, 60.0, 100.0, 40.0, 6.0, 30.0, 3.0, 24.0, 100.0, 2.0, 0.0];

fn with(i: usize, v: f64) -> Effect {
    let mut n = START;
    n[i] = v;
    bolt(n, "#ffffff", "#6e8cff")
}

const NIGHT: (usize, usize) = (480, 270);

/// The proposal's night scene at a quarter of 1920 by 1080: a sky darkening upward, a few
/// clouds, a dark hill along the bottom and three lit windows in it.
fn night() -> Vec<u8> {
    let (w, h) = NIGHT;
    let clouds = [(40.0, 20.0, 90.0, 30.0), (150.0, 10.0, 110.0, 36.0), (260.0, 26.0, 100.0, 28.0), (380.0, 14.0, 120.0, 34.0)];
    let mut bytes = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let (fx, fy) = (x as f64 + 0.5, y as f64 + 0.5);
            let t = y as f64 / h as f64;
            let mut c = [(26.0 + 30.0 * t) as u8, (28.0 + 22.0 * t) as u8, (58.0 + 40.0 * t) as u8];
            if clouds.iter().any(|&(cx, cy, rx, ry)| ((fx - cx) / rx).powi(2) + ((fy - cy) / ry).powi(2) <= 1.0) {
                c = [58, 58, 78];
            }
            if fy >= 212.0 - 10.0 * (fx / 60.0).sin() {
                c = [14, 16, 28];
            }
            if [60, 70, 82].iter().any(|&wx| (wx..wx + 4).contains(&x)) && (236..240).contains(&y) {
                c = [250, 210, 120];
            }
            bytes.extend([c[0], c[1], c[2], 255]);
        }
    }
    bytes
}

/// The night scene as the composition's one layer, with `effects`, drawn at `frame`.
fn picture(dir: &Path, effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/lightning_bolt/fx_bolt_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("night.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(NIGHT.0);
    comp["height"] = J::from(NIGHT.1);
    comp["duration_frames"] = J::from(24);
    comp["work_area"]["end_frame_exclusive"] = J::from(24);
    let layer = &mut comp["layers"][0];
    layer["out_frame"] = J::from(24);
    let middle = serde_json::json!([NIGHT.0 as f64 / 2.0, NIGHT.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &Id::new(effect_table::MAIN), frame, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
}

#[test]
fn b126_lightning_bolt() {
    let mut t = Table::new(
        "lightning_bolt",
        "# B-126: lightning bolt\n\nD-190, accepted by the owner on 2026-09-28. Every expected \
         pixel is `Fixtures/lightning_bolt/expected_lightning_bolt.json`, written by \
         `tools/lightning_bolt_reference.py` before this code existed and printed in document 25 \
         as FX-BOLT-001 to 032. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-BOLT-001 to 032 (document 25)");
    t.fixtures("expected_lightning_bolt.json");

    t.heading("How far it reaches");
    for (what, e) in [
        ("as added, it never grows the drawing's bounds", with(4, 40.0)),
        ("glow 500, the most, does not grow them either", with(8, 500.0)),
        ("width 100, the most, does not grow them either", with(7, 100.0)),
    ] {
        let got = e.bounds_expansion();
        t.row(what, &got.to_string(), got == 0);
    }
    let mut draft = bolt(START, "#ffffff", "#6e8cff");
    draft.scale_distances(|d| d * 0.5);
    let mut halved = START;
    (halved[7], halved[8]) = (1.5, 12.0);
    t.row(
        "a half-size draft preview halves the width and the glow, and nothing else",
        &format!("{draft:?}"),
        draft == bolt(halved, "#ffffff", "#6e8cff"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_bolt_001.json",
        "fx_bolt_004.json",
        "fx_bolt_009.json",
        "fx_bolt_010.json",
        "fx_bolt_014.json",
        "fx_bolt_017.json",
        "fx_bolt_018.json",
        "fx_bolt_019.json",
        "fx_bolt_020.json",
        "fx_bolt_021.json",
        "fx_bolt_022.json",
        "fx_bolt_029.json",
        "fx_bolt_030.json",
        "fx_bolt_031.json",
        "fx_bolt_032.json",
    ]);
    let params = t.saved_parameters("fx_bolt_015.json");
    t.row(
        "fx_bolt_015.json, its colours written in capitals, is saved in small letters",
        &format!("{} {}", params["color"], params["glow_color"]),
        params["color"] == serde_json::json!("#ff8000") && params["glow_color"] == serde_json::json!("#00ff00"),
    );
    t.row(
        "the frame the bolt is drawn at is never saved",
        &format!("{:?}", params.get("frame")),
        params.get("frame").is_none(),
    );
    t.shape_refused(
        "fx_bolt_001.json",
        "no `hold` at all",
        r##"{"start": [40, 0], "end": [60, 100], "jagged": 40, "detail": 6, "branches": 30, "width": 3,
            "glow": 24, "opacity": 100, "seed": 0, "color": "#ffffff", "glow_color": "#6e8cff"}"##,
    );
    t.shape_refused(
        "fx_bolt_001.json",
        "a start that is one number",
        r##"{"start": 40, "end": [60, 100], "jagged": 40, "detail": 6, "branches": 30, "width": 3,
            "glow": 24, "opacity": 100, "hold": 2, "seed": 0, "color": "#ffffff", "glow_color": "#6e8cff"}"##,
    );
    t.shape_refused(
        "fx_bolt_001.json",
        "a width that is a word",
        r##"{"start": [40, 0], "end": [60, 100], "jagged": 40, "detail": 6, "branches": 30, "width": "thin",
            "glow": 24, "opacity": 100, "hold": 2, "seed": 0, "color": "#ffffff", "glow_color": "#6e8cff"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_bolt_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("start x 1001", set(with(0, 1001.0))),
            ("end y -1001", set(with(3, -1001.0))),
            ("jaggedness 101", set(with(4, 101.0))),
            ("detail 0", set(with(5, 0.0))),
            ("detail 9", set(with(5, 9.0))),
            ("branches -1", set(with(6, -1.0))),
            ("width 101", set(with(7, 101.0))),
            ("glow 501", set(with(8, 501.0))),
            ("opacity 101", set(with(9, 101.0))),
            ("hold 0", set(with(10, 0.0))),
            ("seed 100001", set(with(11, 100001.0))),
            ("colour \"#12345\"", set(bolt(START, "#12345", "#6e8cff"))),
            ("glow colour \"blue\"", set(bolt(START, "#ffffff", "blue"))),
            ("branches keyed to 150", keys("branches", &[(0, &[30.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_bolt_001.json",
        vec![
            (
                "every number at its bottom,",
                set(bolt([-1000.0, -1000.0, -1000.0, -1000.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0], "#000000", "#000000")),
            ),
            (
                "every number at its top,",
                set(bolt([1000.0, 1000.0, 1000.0, 1000.0, 100.0, 8.0, 100.0, 100.0, 500.0, 100.0, 100.0, 100000.0], "#ffffff", "#ffffff")),
            ),
            ("detail keyed from 1 to 8", keys("detail", &[(0, &[1.0]), (4, &[8.0])])),
            ("the start keyed across the layer", keys("start", &[(0, &[0.0, 0.0]), (4, &[100.0, 50.0])])),
        ],
    );

    t.heading("Pictures: a night scene at a quarter of 1920 by 1080, in `verification/B-126 pictures/`");
    let dir = effect_table::repo("verification/B-126 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = NIGHT;
    png_out::write_rgba(&dir.join("night.png"), w, h, OutputDepth::Eight, &[], &night()).unwrap();
    let (before, said) = picture(&dir, J::Array(vec![]), 0);
    png_out::write_rgba(&dir.join("before.png"), w, h, OutputDepth::Eight, &[], &before).unwrap();
    t.row("before.png, the scene with no effect, draws cleanly", &format!("{said:?}"), said.is_empty());
    // The proposal's bolt, from the cloud to the hill. A quarter-size picture takes a quarter of
    // the width and the glow, as a quarter-size draft does.
    let base = [30.0, 14.0, 64.0, 76.0, 40.0, 6.0, 30.0, 3.0, 24.0, 100.0, 2.0, 0.0];
    let (s, e) = ((144, 37), (307, 205));
    let kept = [(2, 2), (2, 267), (477, 2)];
    let mut shots: Vec<(String, Vec<u8>)> = Vec::new();
    for (name, change, color, glow_color, frame, what) in [
        ("as_added_frame_0", vec![], "#ffffff", "#6e8cff", 0, "the proposal's bolt at frame 0"),
        ("as_added_frame_2", vec![], "#ffffff", "#6e8cff", 2, "at frame 2, a new bolt"),
        ("branches_80_detail_8", vec![(6, 80.0), (5, 8.0)], "#ffffff", "#6e8cff", 0, "branches 80 and detail 8"),
        ("jaggedness_20", vec![(4, 20.0)], "#ffffff", "#6e8cff", 0, "jaggedness 20"),
        ("red_and_gold", vec![(7, 8.0), (8, 60.0)], "#fff2c0", "#ff4a1a", 0, "width 8, glow 60, a gold core in red"),
    ] {
        let mut n = base;
        for (i, v) in change {
            n[i] = v;
        }
        let effects = serde_json::json!([{
            "instance_id": "fx-0-0", "type_id": "core.lightning_bolt", "enabled": true,
            "parameters": {"start": [n[0], n[1]], "end": [n[2], n[3]], "jagged": n[4], "detail": n[5],
                "branches": n[6], "width": n[7] / 4.0, "glow": n[8] / 4.0, "opacity": n[9], "hold": n[10],
                "seed": n[11], "color": color, "glow_color": glow_color},
        }]);
        let (bytes, said) = picture(&dir, effects, frame);
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let at = |b: &[u8], (x, y): (usize, usize)| b[(y * w + x) * 4..(y * w + x) * 4 + 4].to_vec();
        let moved = |p| at(&bytes, p) != at(&before, p);
        let changed = (0..w * h).filter(|&i| bytes[i * 4..i * 4 + 4] != before[i * 4..i * 4 + 4]).count();
        t.row(
            &format!(
                "{name}.png, {what}, draws cleanly, changes some pixels, lights its two ends {s:?} and \
                 {e:?}, and leaves the corners {kept:?} exactly"
            ),
            &format!(
                "{said:?}, {changed} changed, ends lit {:?}, corners left {:?}",
                [moved(s), moved(e)],
                kept.iter().map(|&p| !moved(p)).collect::<Vec<_>>()
            ),
            said.is_empty() && changed > 0 && moved(s) && moved(e) && kept.iter().all(|&p| !moved(p)),
        );
        shots.push((name.to_string(), bytes));
    }
    t.row(
        "the bolt changes with the hold: frame 2 is not frame 0",
        &format!("{}", shots[0].1 != shots[1].1),
        shots[0].1 != shots[1].1,
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_bolt_001.json", 0), ("fx_bolt_010.json", 0), ("fx_bolt_019.json", 0), ("fx_bolt_021.json", 0)]);

    t.finish("B-126_lightning_bolt_table.md");
}
