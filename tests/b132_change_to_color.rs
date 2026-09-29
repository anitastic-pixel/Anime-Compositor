//! B-132: Change to Color in the core, against D-197.
//!
//! Writes `verification/B-132_change_to_color_table.md`.
//!
//! Every expected pixel is `Fixtures/change_to_color/expected_change_to_color.json`, written by
//! `tools/change_to_color_reference.py` before this code existed and printed in document 25 as
//! FX-CTC-001 to 027. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws a figure in a red jacket, with its shadow and highlight, a pink ribbon and a
//! shaded face, and turns the jacket blue four ways into `verification/B-132 pictures/`, three
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

/// From, to, change, change by, the three tolerances and softness, and the matte, in the card's
/// order.
fn change(words: [&str; 4], n: [f64; 4], matte: &str) -> Effect {
    Effect::ChangeToColor {
        from: words[0].to_string(),
        to: words[1].to_string(),
        change: words[2].to_string(),
        change_by: words[3].to_string(),
        hue_tolerance: n[0],
        lightness_tolerance: n[1],
        saturation_tolerance: n[2],
        softness: n[3],
        view_matte: matte.to_string(),
    }
}

const WORDS: [&str; 4] = ["#ff0000", "#0080ff", "hue", "setting"];
const ADDED: [f64; 4] = [5.0, 50.0, 50.0, 50.0];
const PLATE: (usize, usize) = (160, 100);
const PAPER: [u8; 3] = [244, 236, 216];
const RED: [u8; 3] = [200, 40, 40];
const SHADOW: [u8; 3] = [140, 30, 40];
const LIGHT: [u8; 3] = [240, 140, 140];
const PINK: [u8; 3] = [255, 0, 64];
const SKIN: [u8; 3] = [246, 214, 190];
const SHADE: [u8; 3] = [220, 160, 140];
const LINE: [u8; 3] = [30, 26, 36];

/// A figure in a red jacket on paper: the jacket's highlight down its left side and its shadow
/// down its right, a pink ribbon at the collar, and a face shaded on its right.
fn plate() -> Vec<u8> {
    let mut bytes = Vec::new();
    for y in 0..PLATE.1 {
        for x in 0..PLATE.0 {
            let (fx, fy) = (x as f64, y as f64);
            let head = (fx - 80.0).hypot(fy - 30.0);
            let ribbon = (fx - 80.0).hypot(fy - 56.0);
            let t = (fy - 50.0) / 42.0;
            let (l, r) = (64.0 - 12.0 * t, 96.0 + 12.0 * t);
            let body = (0.0..=1.0).contains(&t) && fx >= l && fx <= r;
            let c = if head < 18.0 && fx > 88.0 {
                SHADE
            } else if head < 18.0 {
                SKIN
            } else if head < 20.0 || (body && (fx - l < 2.0 || r - fx < 2.0 || t * 42.0 < 2.0 || t * 42.0 > 40.0)) {
                LINE
            } else if body && ribbon < 5.0 {
                PINK
            } else if body && fx - l < 6.0 {
                LIGHT
            } else if body && r - fx < 10.0 {
                SHADOW
            } else if body {
                RED
            } else {
                PAPER
            };
            bytes.extend([c[0], c[1], c[2], 255]);
        }
    }
    bytes
}

/// The plate as the composition's one layer, with `effects`, drawn; and the same enlarged three
/// times.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/change_to_color/fx_ctc_001.json")).unwrap(),
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
            [bytes[i], bytes[i + 1], bytes[i + 2], 255]
        })
        .collect();
    (bytes, big, said)
}

#[test]
fn b132_change_to_color() {
    let mut t = Table::new(
        "change_to_color",
        "# B-132: Change to Color\n\nD-197, accepted on 2026-09-28 with the After Effects picks \
         (A7). Every expected pixel is `Fixtures/change_to_color/expected_change_to_color.json`, \
         written by `tools/change_to_color_reference.py` before this code existed and printed in \
         document 25 as FX-CTC-001 to 027. The build's frame is compared sample by sample; the \
         answer is the largest difference over all of them, against the catalogue's tolerance \
         of 2e-5.\n",
    );

    t.heading("FX-CTC-001 to 027 (document 25)");
    t.fixtures("expected_change_to_color.json");

    t.heading("How far it reaches");
    let got = change(WORDS, ADDED, "off").bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = change(WORDS, ADDED, "on");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview keeps every setting, none being a distance",
        &format!("{draft:?}"),
        draft == change(WORDS, ADDED, "on"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_ctc_002.json",
        "fx_ctc_007.json",
        "fx_ctc_012.json",
        "fx_ctc_016.json",
        "fx_ctc_017.json",
        "fx_ctc_021.json",
        "fx_ctc_022.json",
        "fx_ctc_023.json",
        "fx_ctc_025.json",
        "fx_ctc_027.json",
    ]);
    let written = t.saved_parameters("fx_ctc_019.json");
    t.row(
        "fx_ctc_019.json, its colours written in capitals, is saved with them in small letters, as \
         Leave Color's colour is",
        &format!("{} and {}", written["from"], written["to"]),
        written["from"] == "#c82828" && written["to"] == "#0080ff",
    );
    t.shape_refused(
        "fx_ctc_001.json",
        "no `view_matte` at all",
        r##"{"from": "#ff0000", "to": "#0080ff", "change": "hue", "change_by": "setting", "hue_tolerance": 5, "lightness_tolerance": 50, "saturation_tolerance": 50, "softness": 50}"##,
    );
    t.shape_refused(
        "fx_ctc_001.json",
        "a hue tolerance that is a word",
        r##"{"from": "#ff0000", "to": "#0080ff", "change": "hue", "change_by": "setting", "hue_tolerance": "five", "lightness_tolerance": 50, "saturation_tolerance": 50, "softness": 50, "view_matte": "off"}"##,
    );
    t.shape_refused(
        "fx_ctc_001.json",
        "a from colour that is a number",
        r##"{"from": 255, "to": "#0080ff", "change": "hue", "change_by": "setting", "hue_tolerance": 5, "lightness_tolerance": 50, "saturation_tolerance": 50, "softness": 50, "view_matte": "off"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_ctc_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("hue tolerance 101", set(change(WORDS, [101.0, 50.0, 50.0, 50.0], "off"))),
            ("lightness tolerance -1", set(change(WORDS, [5.0, -1.0, 50.0, 50.0], "off"))),
            ("saturation tolerance 100.5", set(change(WORDS, [5.0, 50.0, 100.5, 50.0], "off"))),
            ("softness 101", set(change(WORDS, [5.0, 50.0, 50.0, 101.0], "off"))),
            ("from \"#ff00\"", set(change(["#ff00", "#0080ff", "hue", "setting"], ADDED, "off"))),
            ("to \"blue\"", set(change(["#ff0000", "blue", "hue", "setting"], ADDED, "off"))),
            ("change \"saturation\"", set(change(["#ff0000", "#0080ff", "saturation", "setting"], ADDED, "off"))),
            ("change by \"shift\"", set(change(["#ff0000", "#0080ff", "hue", "shift"], ADDED, "off"))),
            ("view matte \"yes\"", set(change(WORDS, ADDED, "yes"))),
            ("softness keyed to 150", keys("softness", &[(0, &[50.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_ctc_001.json",
        vec![
            (
                "every setting at the top of its range, the last words,",
                set(change(["#ffffff", "#000000", "hue_lightness_saturation", "transforming"], [100.0; 4], "on")),
            ),
            (
                "every setting at the bottom, the other words,",
                set(change(["#000000", "#ffffff", "hue_lightness", "setting"], [0.0; 4], "off")),
            ),
            ("hue tolerance keyed from 0 to 100", keys("hue_tolerance", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("Pictures: a figure in a red jacket, in `verification/B-132 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/B-132 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let drawn = plate();
    png_out::write_rgba(&dir.join("plate.png"), w, h, OutputDepth::Eight, &[], &drawn).unwrap();
    let (_, big, said) = picture(&dir, J::Array(vec![]));
    png_out::write_rgba(&dir.join("before.png"), w * 3, h * 3, OutputDepth::Eight, &[], &big).unwrap();
    t.row("before.png, the plate with no effect, draws cleanly", &format!("{said:?}"), said.is_empty());
    let jacket = [(80, 75), (98, 75), (60, 75)];
    let rest = [(80, 56), (75, 30), (94, 30), (10, 10), (80, 12)];
    let px = |bytes: &[u8], (x, y): (usize, usize)| {
        let i = (y * w + x) * 4;
        [bytes[i], bytes[i + 1], bytes[i + 2]]
    };
    let near = |a: [u8; 3], b: [u8; 3]| a.iter().zip(b).all(|(&u, v)| u.abs_diff(v) <= 1);
    for (name, what, how, matte) in [
        ("hue", "Hue: the jacket, its shadow and highlight turn blue, each as light or dark as it was", ["hue", "setting"], "off"),
        ("transforming", "Transforming To Color: each turned as far round as #0080ff is from the red", ["hue", "transforming"], "off"),
        ("flat", "Hue, Lightness & Saturation: the jacket one flat #0080ff", ["hue_lightness_saturation", "setting"], "off"),
        ("matte", "View Correction Matte: white where the colour changes, black elsewhere", ["hue", "setting"], "on"),
    ] {
        let effects = serde_json::json!([{
            "instance_id": "fx-0-0", "type_id": "core.change_to_color", "enabled": true,
            "parameters": {
                "from": "#c82828", "to": "#0080ff", "change": how[0], "change_by": how[1],
                "hue_tolerance": 5, "lightness_tolerance": 50, "saturation_tolerance": 50, "softness": 50,
                "view_matte": matte,
            },
        }]);
        let (bytes, big, said) = picture(&dir, effects);
        png_out::write_rgba(&dir.join(format!("{name}.png")), w * 3, h * 3, OutputDepth::Eight, &[], &big)
            .unwrap();
        let changed: Vec<[u8; 3]> = jacket.iter().map(|&p| px(&bytes, p)).collect();
        let kept: Vec<[u8; 3]> = rest.iter().map(|&p| px(&bytes, p)).collect();
        let ok = match name {
            "flat" => changed.iter().all(|&c| near(c, [0, 128, 255])),
            "matte" => changed.iter().all(|&c| c == [255; 3]) && kept.iter().all(|&c| c == [0; 3]),
            _ => {
                changed.iter().all(|c| c[2] > c[0] + 40)
                    && changed.iter().zip(jacket).all(|(c, p)| c != &px(&drawn, p))
            }
        } && (name == "matte" || kept.iter().zip(rest).all(|(c, p)| c == &px(&drawn, p)));
        t.row(
            &format!(
                "{name}.png, {what}; draws cleanly; the jacket's red, shadow and highlight at {jacket:?}, \
                 and the ribbon, the face, its shade, the paper and the line at {rest:?}"
            ),
            &format!("{said:?}, jacket {changed:?}, the rest {kept:?}"),
            said.is_empty() && ok,
        );
    }

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_ctc_002.json", 0), ("fx_ctc_011.json", 0), ("fx_ctc_018.json", 3)]);

    t.finish("B-132_change_to_color_table.md");
}
