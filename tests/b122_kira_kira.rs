//! B-122: kira-kira in the core, against D-186.
//!
//! Writes `verification/B-122_kira_kira_table.md`.
//!
//! Every expected pixel is `Fixtures/kira_kira/expected_kira_kira.json`, written by
//! `tools/kira_kira_reference.py` before this code existed and printed in document 25 as
//! FX-KIRA-001 to 029. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws a night scene, the proposal's, at a quarter of 1920 by 1080, and sparkles it
//! six ways into `verification/B-122 pictures/`.

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

/// `[threshold, spacing, density, size, angle, twinkle, period, seed, opacity]`.
fn kira(n: [f64; 9], shape: &str, color: &str) -> Effect {
    Effect::KiraKira {
        threshold: n[0],
        spacing: n[1],
        density: n[2],
        size: n[3],
        shape: shape.to_string(),
        angle: n[4],
        twinkle: n[5],
        period: n[6],
        seed: n[7],
        opacity: n[8],
        color: color.to_string(),
        frame: 0,
    }
}

const START: [f64; 9] = [95.0, 64.0, 60.0, 40.0, 0.0, 100.0, 24.0, 0.0, 100.0];

fn with(i: usize, v: f64) -> Effect {
    let mut n = START;
    n[i] = v;
    kira(n, "star", "#ffffff")
}

const NIGHT: (usize, usize) = (480, 270);

/// The proposal's night scene at a quarter of 1920 by 1080: a sky, a sea with a moon path of
/// white dashes, a pale moon, a face whose eyes each hold two white glints, a pale collar, and a
/// sword with a white edge and a white point on its gem.
fn night() -> Vec<u8> {
    let (w, h) = NIGHT;
    let mut dashes = Vec::new();
    let mut r: u64 = 3;
    let mut next = |n: u64| {
        r = r.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (r >> 33) % n
    };
    for _ in 0..60 {
        let (x, y) = (300 + next(120) as usize, 205 + next(60) as usize);
        dashes.push((x, y, 1 + next(5) as usize));
    }
    // The distance from (px, py) to the segment (ax, ay) to (bx, by).
    let seg = |px: f64, py: f64, (ax, ay): (f64, f64), (bx, by): (f64, f64)| {
        let (vx, vy) = (bx - ax, by - ay);
        let t = (((px - ax) * vx + (py - ay) * vy) / (vx * vx + vy * vy)).clamp(0.0, 1.0);
        (px - ax - t * vx).hypot(py - ay - t * vy)
    };
    let mut bytes = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let (fx, fy) = (x as f64 + 0.5, y as f64 + 0.5);
            let oval = |x0: f64, y0: f64, x1: f64, y1: f64| {
                ((fx - (x0 + x1) / 2.0) / ((x1 - x0) / 2.0)).powi(2) + ((fy - (y0 + y1) / 2.0) / ((y1 - y0) / 2.0)).powi(2) <= 1.0
            };
            let t = y as f64 / h as f64;
            let mut c = [(20.0 + 40.0 * t) as u8, (24.0 + 20.0 * t) as u8, (70.0 + 40.0 * t) as u8];
            if y >= 200 {
                c = [16, 30, 70];
            }
            if dashes.iter().any(|&(dx, dy, l)| y == dy && (dx..=dx + l).contains(&x)) {
                c = [255, 255, 255];
            }
            if oval(340.0, 30.0, 380.0, 70.0) {
                c = [240, 236, 210];
            }
            if oval(60.0, 40.0, 200.0, 200.0) {
                c = [70, 50, 60];
            }
            if oval(78.0, 70.0, 182.0, 190.0) {
                c = [246, 214, 190];
            }
            for ex in [98.0, 142.0] {
                if oval(ex, 110.0, ex + 26.0, 150.0) {
                    c = [40, 30, 80];
                }
                if oval(ex + 5.0, 120.0, ex + 21.0, 146.0) {
                    c = [90, 60, 150];
                }
                if oval(ex + 4.0, 114.0, ex + 12.0, 122.0) || oval(ex + 16.0, 138.0, ex + 20.0, 142.0) {
                    c = [255, 255, 255];
                }
            }
            let t = (fy - 200.0) / 70.0;
            if (0.0..1.0).contains(&t) && fx >= 70.0 - 20.0 * t && fx <= 190.0 + 20.0 * t {
                c = [232, 232, 238];
            }
            if seg(fx, fy, (256.0, 254.0), (396.0, 107.0)) < 5.0 {
                c = [150, 160, 185];
            }
            if seg(fx, fy, (256.0, 250.0), (396.0, 104.0)) < 1.0 {
                c = [255, 255, 255];
            }
            if seg(fx, fy, (243.0, 234.0), (274.0, 265.0)) < 3.5 {
                c = [90, 70, 40];
            }
            if (fx - 262.0).abs() + (fy - 242.0).abs() < 8.0 {
                c = [200, 30, 50];
            }
            if (260..262).contains(&x) && (238..240).contains(&y) {
                c = [255, 255, 255];
            }
            bytes.extend([c[0], c[1], c[2], 255]);
        }
    }
    bytes
}

/// The night scene as the composition's one layer, with `effects`, drawn at `frame`.
fn picture(dir: &Path, effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/kira_kira/fx_kira_001.json")).unwrap(),
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
fn b122_kira_kira() {
    let mut t = Table::new(
        "kira_kira",
        "# B-122: kira-kira\n\nD-186, accepted by the owner on 2026-09-28. Every expected pixel \
         is `Fixtures/kira_kira/expected_kira_kira.json`, written by \
         `tools/kira_kira_reference.py` before this code existed and printed in document 25 as \
         FX-KIRA-001 to 029. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-KIRA-001 to 029 (document 25)");
    t.fixtures("expected_kira_kira.json");

    t.heading("How far it reaches");
    for (what, e, want) in [
        ("as added, size 40, grows the drawing's bounds by 40", with(3, 40.0), 40),
        ("size 12.5 grows them by 13, the size rounded up", with(3, 12.5), 13),
        ("size 0 grows them by nothing", with(3, 0.0), 0),
        ("density 0 grows them by nothing", with(2, 0.0), 0),
        ("opacity 0 grows them by nothing", with(8, 0.0), 0),
    ] {
        let got = e.bounds_expansion();
        t.row(what, &got.to_string(), got == want);
    }
    let mut draft = kira(START, "star", "#ffffff");
    draft.scale_distances(|d| d * 0.5);
    let mut halved = START;
    (halved[1], halved[3]) = (32.0, 20.0);
    t.row(
        "a half-size draft preview halves the spacing and the size, and nothing else",
        &format!("{draft:?}"),
        draft == kira(halved, "star", "#ffffff"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_kira_001.json",
        "fx_kira_006.json",
        "fx_kira_007.json",
        "fx_kira_016.json",
        "fx_kira_019.json",
        "fx_kira_020.json",
        "fx_kira_021.json",
        "fx_kira_022.json",
        "fx_kira_026.json",
        "fx_kira_027.json",
        "fx_kira_028.json",
    ]);
    let params = t.saved_parameters("fx_kira_017.json");
    t.row(
        "fx_kira_017.json, its colour written in capitals, is saved in small letters, as Rain's is",
        &params["color"].to_string(),
        params["color"] == serde_json::json!("#ff8000"),
    );
    t.row(
        "the frame the twinkle is worked at is never saved",
        &format!("{:?}", params.get("frame")),
        params.get("frame").is_none(),
    );
    t.shape_refused(
        "fx_kira_001.json",
        "no `period` at all",
        r##"{"threshold": 95, "spacing": 64, "density": 60, "size": 40, "shape": "star", "angle": 0,
            "twinkle": 100, "seed": 0, "opacity": 100, "color": "#ffffff"}"##,
    );
    t.shape_refused(
        "fx_kira_001.json",
        "a size that is a word",
        r##"{"threshold": 95, "spacing": 64, "density": 60, "size": "big", "shape": "star", "angle": 0,
            "twinkle": 100, "period": 24, "seed": 0, "opacity": 100, "color": "#ffffff"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_kira_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("threshold 101", set(with(0, 101.0))),
            ("spacing 1", set(with(1, 1.0))),
            ("density -1", set(with(2, -1.0))),
            ("size 1001", set(with(3, 1001.0))),
            ("angle 3601", set(with(4, 3601.0))),
            ("twinkle 101", set(with(5, 101.0))),
            ("period 0", set(with(6, 0.0))),
            ("seed 100001", set(with(7, 100001.0))),
            ("opacity 101", set(with(8, 101.0))),
            ("shape \"circle\"", set(kira(START, "circle", "#ffffff"))),
            ("colour \"#12345\"", set(kira(START, "star", "#12345"))),
            ("twinkle keyed to 150", keys("twinkle", &[(0, &[100.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_kira_001.json",
        vec![
            (
                "every number at its bottom, a cross,",
                set(kira([0.0, 2.0, 0.0, 0.0, -3600.0, 0.0, 1.0, 0.0, 0.0], "cross", "#000000")),
            ),
            (
                "every number at its top,",
                set(kira([100.0, 1000.0, 100.0, 1000.0, 3600.0, 100.0, 1000.0, 100000.0, 100.0], "star", "#ffffff")),
            ),
            ("period keyed from 1 to 1000", keys("period", &[(0, &[1.0]), (4, &[1000.0])])),
        ],
    );

    t.heading("Pictures: a night scene at a quarter of 1920 by 1080, in `verification/B-122 pictures/`");
    let dir = effect_table::repo("verification/B-122 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = NIGHT;
    png_out::write_rgba(&dir.join("night.png"), w, h, OutputDepth::Eight, &[], &night()).unwrap();
    let (before, said) = picture(&dir, J::Array(vec![]), 0);
    png_out::write_rgba(&dir.join("before.png"), w, h, OutputDepth::Eight, &[], &before).unwrap();
    t.row("before.png, the scene with no effect, draws cleanly", &format!("{said:?}"), said.is_empty());
    // A quarter-size picture takes a quarter of every distance, as a quarter-size draft does.
    let quarter = |n: [f64; 9]| [n[0], n[1] / 4.0, n[2], n[3] / 4.0, n[4], n[5], n[6], n[7], n[8]];
    let (moon, cheek, collar, sky) = ((360, 50), (130, 175), (130, 240), (20, 20));
    let (glint, small_glint) = ((106, 118), (160, 140));
    let mut shots: Vec<(String, Vec<u8>)> = Vec::new();
    for (name, n, shape, color, frame, what, lit) in [
        ("as_added_frame_0", START, "star", "#ffffff", 0, "as it is added, at frame 0", vec![]),
        ("as_added_frame_12", START, "star", "#ffffff", 12, "as it is added, at frame 12, half a twinkle on", vec![]),
        (
            "density_100",
            { let mut n = START; n[2] = 100.0; n },
            "star",
            "#ffffff",
            0,
            "density 100, a star in every lit cell",
            vec![glint, small_glint],
        ),
        (
            "small_and_close",
            { let mut n = START; (n[1], n[3]) = (32.0, 24.0); n },
            "star",
            "#ffffff",
            0,
            "spacing 32 and size 24",
            vec![],
        ),
        (
            "warm_cross",
            { let mut n = START; n[4] = 45.0; n },
            "cross",
            "#ffe0a0",
            0,
            "a warm cross at angle 45",
            vec![],
        ),
    ] {
        let q = quarter(n);
        let effects = serde_json::json!([{
            "instance_id": "fx-0-0", "type_id": "core.kira_kira", "enabled": true,
            "parameters": {"threshold": q[0], "spacing": q[1], "density": q[2], "size": q[3], "shape": shape,
                "angle": q[4], "twinkle": q[5], "period": q[6], "seed": q[7], "opacity": q[8], "color": color},
        }]);
        let (bytes, said) = picture(&dir, effects, frame);
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let at = |b: &[u8], (x, y): (usize, usize)| b[(y * w + x) * 4..(y * w + x) * 4 + 4].to_vec();
        let moved = |p| at(&bytes, p) != at(&before, p);
        // A white glint's own pixels stay white; its star shows on the pixels round it.
        let near = |(px, py): (usize, usize)| (py - 10..py + 10).any(|y| (px - 10..px + 10).any(|x| moved((x, y))));
        let changed = (0..w * h).filter(|&i| bytes[i * 4..i * 4 + 4] != before[i * 4..i * 4 + 4]).count();
        let kept = [moon, cheek, collar, sky];
        t.row(
            &format!(
                "{name}.png, {what}, draws cleanly, changes some pixels, some within 10 of each of {lit:?}, \
                 and leaves the moon, the cheek, the collar and the sky, {kept:?}, exactly"
            ),
            &format!(
                "{said:?}, {changed} changed, lit {:?}, left {:?}",
                lit.iter().map(|&p| near(p)).collect::<Vec<_>>(),
                kept.iter().map(|&p| !moved(p)).collect::<Vec<_>>()
            ),
            said.is_empty() && changed > 0 && lit.iter().all(|&p| near(p)) && kept.iter().all(|&p| !moved(p)),
        );
        shots.push((name.to_string(), bytes));
    }
    t.row(
        "the twinkle moves: frame 12 is not frame 0",
        &format!("{}", shots[0].1 != shots[1].1),
        shots[0].1 != shots[1].1,
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_kira_001.json", 4), ("fx_kira_007.json", 0), ("fx_kira_021.json", 0)]);

    t.finish("B-122_kira_kira_table.md");
}
