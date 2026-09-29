//! B-120: the HSV key in the core, against D-184.
//!
//! Writes `verification/B-120_hsv_key_table.md`.
//!
//! Every expected pixel is `Fixtures/hsv_key/expected_hsv_key.json`, written by
//! `tools/hsv_key_reference.py` before this code existed and printed in document 25 as
//! FX-HSV-001 to 016. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws a figure in front of a green screen and keys it three ways into
//! `verification/B-120 pictures/`, over a grey checkerboard where pixels are taken out, three
//! times enlarged.

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

/// Hue, saturation, value, their three ranges, and invert, in the card's order.
fn key(n: [f64; 6], invert: &str) -> Effect {
    Effect::HsvKey {
        hue: n[0],
        saturation: n[1],
        value: n[2],
        hue_range: n[3],
        saturation_range: n[4],
        value_range: n[5],
        invert: invert.to_string(),
    }
}

const ADDED: [f64; 6] = [120.0, 60.0, 60.0, 40.0, 40.0, 40.0];
const PLATE: (usize, usize) = (160, 100);
const GREEN: [u8; 3] = [0, 177, 64];
const SHADOW: [u8; 3] = [0, 100, 36];
const SPILL: [u8; 3] = [90, 160, 100];
const SKIN: [u8; 3] = [246, 214, 190];
const LINE: [u8; 3] = [30, 26, 36];
const SHIRT: [u8; 3] = [60, 90, 200];
const GREY: [u8; 3] = [128, 128, 128];

/// A figure in front of a green screen that darkens to its shadow at the floor: a head, a shirt
/// with a grey button, and green light spilt down the shirt's left side.
fn plate() -> Vec<u8> {
    let mut bytes = Vec::new();
    for y in 0..PLATE.1 {
        for x in 0..PLATE.0 {
            let (fx, fy) = (x as f64, y as f64);
            let head = (fx - 80.0).hypot(fy - 30.0);
            let button = (fx - 80.0).hypot(fy - 66.0);
            let t = (fy - 50.0) / 42.0;
            let (l, r) = (64.0 - 12.0 * t, 96.0 + 12.0 * t);
            let body = (0.0..=1.0).contains(&t) && fx >= l && fx <= r;
            let c = if head < 18.0 {
                SKIN
            } else if head < 20.0 || (body && (fx - l < 2.0 || r - fx < 2.0 || t * 42.0 < 2.0 || t * 42.0 > 40.0)) {
                LINE
            } else if body && fx - l < 5.0 {
                SPILL
            } else if body && button < 3.0 {
                GREY
            } else if body && button < 4.0 {
                LINE
            } else if body {
                SHIRT
            } else if y >= 80 {
                SHADOW
            } else {
                GREEN
            };
            bytes.extend([c[0], c[1], c[2], 255]);
        }
    }
    bytes
}

/// The plate as the composition's one layer, with `effects`, drawn; and the same over a grey
/// checkerboard, enlarged three times.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/hsv_key/fx_hsv_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("plate.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(PLATE.0);
    comp["height"] = J::from(PLATE.1);
    let layer = &mut comp["layers"][0];
    let middle = serde_json::json!([PLATE.0 as f64 / 2.0, PLATE.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(effect_table::MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    let bytes = frame.to_srgb8_straight();
    let big: Vec<u8> = (0..PLATE.1 * 3)
        .flat_map(|y| (0..PLATE.0 * 3).map(move |x| (x, y)))
        .flat_map(|(x, y)| {
            let i = (y / 3 * PLATE.0 + x / 3) * 4;
            let board = if (x / 15 + y / 15) % 2 == 1 { 200.0 } else { 150.0 };
            let a = f64::from(bytes[i + 3]) / 255.0;
            let mix = |c: u8| (f64::from(c) * a + board * (1.0 - a)).round() as u8;
            [mix(bytes[i]), mix(bytes[i + 1]), mix(bytes[i + 2]), 255]
        })
        .collect();
    (bytes, big, said)
}

#[test]
fn b120_hsv_key() {
    let mut t = Table::new(
        "hsv_key",
        "# B-120: HSV key\n\nD-184, accepted by the owner on 2026-09-28. Every expected pixel \
         is `Fixtures/hsv_key/expected_hsv_key.json`, written by `tools/hsv_key_reference.py` \
         before this code existed and printed in document 25 as FX-HSV-001 to 016. The build's \
         frame is compared sample by sample; the answer is the largest difference over all of \
         them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-HSV-001 to 016 (document 25)");
    t.fixtures("expected_hsv_key.json");

    t.heading("How far it reaches");
    let got = key(ADDED, "off").bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = key(ADDED, "on");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview keeps every setting, none being a distance",
        &format!("{draft:?}"),
        draft == key(ADDED, "on"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_hsv_002.json",
        "fx_hsv_004.json",
        "fx_hsv_009.json",
        "fx_hsv_011.json",
        "fx_hsv_015.json",
        "fx_hsv_016.json",
    ]);
    t.shape_refused(
        "fx_hsv_001.json",
        "no `invert` at all",
        r##"{"hue": 120, "saturation": 60, "value": 60, "hue_range": 40, "saturation_range": 40, "value_range": 40}"##,
    );
    t.shape_refused(
        "fx_hsv_001.json",
        "a hue that is a word",
        r##"{"hue": "green", "saturation": 60, "value": 60, "hue_range": 40, "saturation_range": 40, "value_range": 40, "invert": "off"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_hsv_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("hue 361", set(key([361.0, 60.0, 60.0, 40.0, 40.0, 40.0], "off"))),
            ("hue range 181", set(key([120.0, 60.0, 60.0, 181.0, 40.0, 40.0], "off"))),
            ("saturation 101", set(key([120.0, 101.0, 60.0, 40.0, 40.0, 40.0], "off"))),
            ("value range -1", set(key([120.0, 60.0, 60.0, 40.0, 40.0, -1.0], "off"))),
            ("invert \"yes\"", set(key(ADDED, "yes"))),
            ("hue range keyed to 200", keys("hue_range", &[(0, &[40.0]), (4, &[200.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_hsv_001.json",
        vec![
            ("every setting at the top of its range,", set(key([360.0, 100.0, 100.0, 180.0, 100.0, 100.0], "on"))),
            ("every setting at the bottom,", set(key([0.0; 6], "off"))),
            ("hue keyed from 0 to 360", keys("hue", &[(0, &[0.0]), (4, &[360.0])])),
        ],
    );

    t.heading("Pictures: a figure in front of a green screen, in `verification/B-120 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/B-120 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    png_out::write_rgba(&dir.join("plate.png"), w, h, OutputDepth::Eight, &[], &plate()).unwrap();
    let (_, big, said) = picture(&dir, J::Array(vec![]));
    png_out::write_rgba(&dir.join("before.png"), w * 3, h * 3, OutputDepth::Eight, &[], &big).unwrap();
    t.row("before.png, the plate with no effect, draws cleanly", &format!("{said:?}"), said.is_empty());
    let (screen, shadow, spill, skin, shirt) = ((10, 10), (10, 90), (61, 70), (80, 30), (90, 75));
    for (name, n, invert, what, gone, kept) in [
        ("as_added", ADDED, "off", "as it is added", vec![screen, shadow, spill], vec![skin, shirt]),
        (
            "narrow",
            [142.0, 100.0, 50.0, 5.0, 10.0, 50.0],
            "off",
            "hue 142 within 5, saturation 100 within 10, value 50 within 50",
            vec![screen, shadow],
            vec![spill, skin, shirt],
        ),
        (
            "inverted",
            [142.0, 100.0, 50.0, 5.0, 10.0, 50.0],
            "on",
            "the same inverted",
            vec![spill, skin, shirt],
            vec![screen, shadow],
        ),
    ] {
        let effects = serde_json::json!([{
            "instance_id": "fx-0-0", "type_id": "core.hsv_key", "enabled": true,
            "parameters": {
                "hue": n[0], "saturation": n[1], "value": n[2],
                "hue_range": n[3], "saturation_range": n[4], "value_range": n[5], "invert": invert,
            },
        }]);
        let (bytes, big, said) = picture(&dir, effects);
        png_out::write_rgba(&dir.join(format!("{name}.png")), w * 3, h * 3, OutputDepth::Eight, &[], &big)
            .unwrap();
        let alpha = |(x, y): (usize, usize)| bytes[(y * w + x) * 4 + 3];
        t.row(
            &format!("{name}.png, {what}, draws cleanly, takes out {gone:?} and keeps {kept:?} whole"),
            &format!("{said:?}, covering {:?} and {:?}", gone.iter().map(|&p| alpha(p)).collect::<Vec<_>>(),
                kept.iter().map(|&p| alpha(p)).collect::<Vec<_>>()),
            said.is_empty() && gone.iter().all(|&p| alpha(p) == 0) && kept.iter().all(|&p| alpha(p) == 255),
        );
    }

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_hsv_001.json", 0), ("fx_hsv_010.json", 3)]);

    t.finish("B-120_hsv_key_table.md");
}
