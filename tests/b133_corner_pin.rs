//! B-133: Corner Pin in the core, against D-198.
//!
//! Writes `verification/B-133_corner_pin_table.md`.
//!
//! Every expected pixel is `Fixtures/corner_pin/expected_corner_pin.json`, written by
//! `tools/corner_pin_reference.py` before this code existed and printed in document 25 as
//! FX-PIN-001 to 018. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws a poster, a sky with a sun and a red strip along the bottom, and pins it four
//! ways into `verification/B-133 pictures/`, three times enlarged, over a grey check where it is
//! clear.

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

/// Upper left, upper right, lower left and lower right, in the card's order.
fn pin(p: [[f64; 2]; 4]) -> Effect {
    Effect::CornerPin { upper_left: p[0], upper_right: p[1], lower_left: p[2], lower_right: p[3] }
}

const START: [[f64; 2]; 4] = [[0.0, 0.0], [100.0, 0.0], [0.0, 100.0], [100.0, 100.0]];
const PLATE: (usize, usize) = (160, 100);
const SKY: [u8; 3] = [106, 160, 255];
const SUN: [u8; 3] = [255, 208, 64];
const RED: [u8; 3] = [200, 40, 40];
const LINE: [u8; 3] = [30, 26, 36];

/// `START` with corner `i` moved to `p`.
fn with(i: usize, p: [f64; 2]) -> Effect {
    let mut c = START;
    c[i] = p;
    pin(c)
}

/// The poster: a sky, a sun up on the right, a red strip along the bottom, a dark border.
fn plate() -> Vec<u8> {
    let mut bytes = Vec::new();
    for y in 0..PLATE.1 {
        for x in 0..PLATE.0 {
            let (fx, fy) = (x as f64 + 0.5, y as f64 + 0.5);
            let c = if x < 4 || y < 4 || x >= PLATE.0 - 4 || y >= PLATE.1 - 4 {
                LINE
            } else if y >= 72 {
                RED
            } else if (fx - 120.0).hypot(fy - 30.0) < 16.0 {
                SUN
            } else {
                SKY
            };
            bytes.extend([c[0], c[1], c[2], 255]);
        }
    }
    bytes
}

/// The plate as the composition's one layer, with `effects`, drawn, straight 8-bit; and the same
/// enlarged three times over a grey check where it is clear.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/corner_pin/fx_pin_001.json")).unwrap(),
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
            let check = if (x / 12 + y / 12) % 2 == 0 { 96.0 } else { 128.0 };
            let a = bytes[i + 3] as f64 / 255.0;
            let over = |c: u8| (c as f64 * a + check * (1.0 - a)).round() as u8;
            [over(bytes[i]), over(bytes[i + 1]), over(bytes[i + 2]), 255]
        })
        .collect();
    (bytes, big, said)
}

#[test]
fn b133_corner_pin() {
    let mut t = Table::new(
        "corner_pin",
        "# B-133: Corner Pin\n\nD-198, accepted on 2026-09-28 with the After Effects picks (A8). \
         Every expected pixel is `Fixtures/corner_pin/expected_corner_pin.json`, written by \
         `tools/corner_pin_reference.py` before this code existed and printed in document 25 as \
         FX-PIN-001 to 018. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-PIN-001 to 018 (document 25)");
    t.fixtures("expected_corner_pin.json");

    t.heading("How far it reaches");
    let got = with(0, [-25.0, 0.0]).bounds_expansion();
    t.row(
        "it declares no fixed growth to the card, which never runs it: how far it grows depends \
         on the size the drawing reaches it at, and is counted as the stack runs, as FX-PIN-006, \
         013 and 015 show",
        &got.to_string(),
        got == 0,
    );
    let mut draft = with(3, [60.0, 60.0]);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview keeps every corner, each being a share of the drawing",
        &format!("{draft:?}"),
        draft == with(3, [60.0, 60.0]),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_pin_002.json",
        "fx_pin_004.json",
        "fx_pin_008.json",
        "fx_pin_011.json",
        "fx_pin_012.json",
        "fx_pin_014.json",
        "fx_pin_015.json",
        "fx_pin_016.json",
        "fx_pin_017.json",
        "fx_pin_018.json",
    ]);
    t.shape_refused(
        "fx_pin_001.json",
        "no `lower_right` at all",
        r#"{"upper_left": [0, 0], "upper_right": [100, 0], "lower_left": [0, 100]}"#,
    );
    t.shape_refused(
        "fx_pin_001.json",
        "a corner of one number",
        r#"{"upper_left": [0], "upper_right": [100, 0], "lower_left": [0, 100], "lower_right": [100, 100]}"#,
    );
    t.shape_refused(
        "fx_pin_001.json",
        "a corner that is a word",
        r#"{"upper_left": "top", "upper_right": [100, 0], "lower_left": [0, 100], "lower_right": [100, 100]}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_pin_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("upper left x -401", set(with(0, [-401.0, 0.0]))),
            ("upper right y 501", set(with(1, [100.0, 501.0]))),
            ("lower left x 500.5", set(with(2, [500.5, 100.0]))),
            ("lower right y -400.5", set(with(3, [100.0, -400.5]))),
            ("lower right keyed to (600, 100)", keys("lower_right", &[(0, &[100.0, 100.0]), (4, &[600.0, 100.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_pin_001.json",
        vec![
            ("every corner at the bottom of its range,", set(pin([[-400.0; 2]; 4]))),
            ("every corner at the top of its range,", set(pin([[500.0; 2]; 4]))),
            ("a bow tie, which draws nothing,", set(pin([[0.0, 0.0], [100.0, 100.0], [0.0, 100.0], [100.0, 0.0]]))),
            ("upper left keyed from (0, 0) to (-400, 500)", keys("upper_left", &[(0, &[0.0, 0.0]), (4, &[-400.0, 500.0])])),
        ],
    );

    t.heading("Pictures: a poster, in `verification/B-133 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/B-133 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let drawn = plate();
    png_out::write_rgba(&dir.join("plate.png"), w, h, OutputDepth::Eight, &[], &drawn).unwrap();
    let (_, big, said) = picture(&dir, J::Array(vec![]));
    png_out::write_rgba(&dir.join("before.png"), w * 3, h * 3, OutputDepth::Eight, &[], &big).unwrap();
    t.row("before.png, the poster with no effect, draws cleanly", &format!("{said:?}"), said.is_empty());
    let px = |bytes: &[u8], (x, y): (usize, usize)| {
        let i = (y * w + x) * 4;
        [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]
    };
    let near = |a: [u8; 4], b: [u8; 4]| a.iter().zip(b).all(|(&u, v)| u.abs_diff(v) <= 1);
    for (name, what, corners) in [
        (
            "keystone",
            "Upper Left (25, 0) and Upper Right (75, 0): the top drawn in, the bottom kept; the top \
             corners clear, the bottom ones and the middle as they were",
            [[25.0, 0.0], [75.0, 0.0], [0.0, 100.0], [100.0, 100.0]],
        ),
        (
            "wall",
            "pinned to (52, 18), (88, 8), (52, 70) and (88, 86), a wall on the right leaning \
             away: the left half clear, the poster's sky inside the four corners",
            [[52.0, 18.0], [88.0, 8.0], [52.0, 70.0], [88.0, 86.0]],
        ),
        (
            "flipped",
            "the left corners and the right ones swapped: the poster turned over left to right, \
             the sun on the left",
            [[100.0, 0.0], [0.0, 0.0], [100.0, 100.0], [0.0, 100.0]],
        ),
        (
            "crossed",
            "the bottom corners swapped, a bow tie: nothing drawn, as crossed corners draw nothing",
            [[0.0, 0.0], [100.0, 0.0], [100.0, 100.0], [0.0, 100.0]],
        ),
    ] {
        let effects = serde_json::json!([{
            "instance_id": "fx-0-0", "type_id": "core.corner_pin", "enabled": true,
            "parameters": {
                "upper_left": corners[0], "upper_right": corners[1],
                "lower_left": corners[2], "lower_right": corners[3],
            },
        }]);
        let (bytes, big, said) = picture(&dir, effects);
        png_out::write_rgba(&dir.join(format!("{name}.png")), w * 3, h * 3, OutputDepth::Eight, &[], &big)
            .unwrap();
        let clear = |p| px(&bytes, p)[3] == 0;
        let kept = |p| near(px(&bytes, p), px(&drawn, p));
        let (ok, seen) = match name {
            "keystone" => (
                clear((2, 2)) && clear((157, 2)) && kept((2, 97)) && kept((157, 97)) && kept((80, 50)),
                format!("{:?}", [(2, 2), (157, 2), (2, 97), (157, 97), (80, 50)].map(|p| px(&bytes, p))),
            ),
            "wall" => (
                clear((10, 50)) && clear((70, 5)) && near(px(&bytes, (112, 40)), [SKY[0], SKY[1], SKY[2], 255]),
                format!("{:?}", [(10, 50), (70, 5), (112, 40)].map(|p| px(&bytes, p))),
            ),
            "flipped" => {
                let all = (0..h).all(|y| (0..w).all(|x| near(px(&bytes, (x, y)), px(&drawn, (w - 1 - x, y)))));
                (all, format!("every pixel the poster's across from it: {all}"))
            }
            _ => {
                let all = (0..h).all(|y| (0..w).all(|x| clear((x, y))));
                (all, format!("every pixel clear: {all}"))
            }
        };
        t.row(
            &format!("{name}.png, {what}; draws cleanly"),
            &format!("{said:?}, {seen}"),
            said.is_empty() && ok,
        );
    }

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_pin_005.json", 0), ("fx_pin_012.json", 2), ("fx_pin_015.json", 0)]);

    t.finish("B-133_corner_pin_table.md");
}
