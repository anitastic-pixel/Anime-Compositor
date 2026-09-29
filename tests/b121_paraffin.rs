//! B-121: paraffin in the core, against D-185.
//!
//! Writes `verification/B-121_paraffin_table.md`.
//!
//! Every expected pixel is `Fixtures/paraffin/expected_paraffin.json`, written by
//! `tools/paraffin_reference.py` before this code existed and printed in document 25 as
//! FX-PARA-001 to 023. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws a standing figure on a clear cel and washes it four ways into
//! `verification/B-121 pictures/`, over a pale blue background, three times enlarged.

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

fn wash(color: &str, direction: f64, spread: f64, opacity: f64, blend: &str) -> Effect {
    Effect::Paraffin {
        color: color.to_string(),
        direction,
        spread,
        opacity,
        blend: blend.to_string(),
    }
}

fn added() -> Effect {
    wash("#6450a0", 180.0, 70.0, 50.0, "multiply")
}

const CEL: (usize, usize) = (100, 120);
const BACK: [f64; 3] = [200.0, 214.0, 226.0];

/// A standing figure on a clear cel: hair, a face and neck, a white shirt, a blue skirt and two
/// legs, each part outlined where it meets another part or the clear.
fn cel() -> Vec<u8> {
    let part = |x: i64, y: i64| -> usize {
        let (fx, fy) = (x as f64 + 0.5, y as f64 + 0.5);
        let inside = |cx: f64, cy: f64, rx: f64, ry: f64| ((fx - cx) / rx).powi(2) + ((fy - cy) / ry).powi(2) <= 1.0;
        let across = |top: f64, bottom: f64, l0: f64, r0: f64, l1: f64, r1: f64| {
            let t = (fy - top) / (bottom - top);
            (0.0..1.0).contains(&t) && fx >= l0 + t * (l1 - l0) && fx <= r0 + t * (r1 - r0)
        };
        if inside(50.0, 27.0, 12.0, 13.0) {
            1
        } else if inside(50.0, 23.0, 16.0, 17.0) {
            2
        } else if (44..=56).contains(&x) && (40..44).contains(&y) {
            3
        } else if across(44.0, 72.0, 32.0, 68.0, 36.0, 64.0) {
            4
        } else if across(72.0, 92.0, 36.0, 64.0, 28.0, 72.0) {
            5
        } else if (92..114).contains(&y) && ((40..=46).contains(&x) || (54..=60).contains(&x)) {
            6
        } else {
            0
        }
    };
    let colors = [[0, 0, 0], [246, 214, 190], [70, 50, 60], [246, 214, 190], [240, 240, 245], [60, 80, 150], [246, 214, 190]];
    let mut bytes = Vec::new();
    for y in 0..CEL.1 as i64 {
        for x in 0..CEL.0 as i64 {
            let p = part(x, y);
            let edge = [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().any(|(dx, dy)| part(x + dx, y + dy) != p);
            let c = if p == 0 { [0, 0, 0] } else if edge { [30, 26, 36] } else { colors[p] };
            bytes.extend([c[0], c[1], c[2], if p == 0 { 0 } else { 255 }]);
        }
    }
    bytes
}

/// The cel as the composition's one layer, with `effects`, drawn; and the same over the pale
/// blue background, enlarged three times.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/paraffin/fx_para_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("cel.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(CEL.0);
    comp["height"] = J::from(CEL.1);
    let layer = &mut comp["layers"][0];
    let middle = serde_json::json!([CEL.0 as f64 / 2.0, CEL.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(effect_table::MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    let bytes = frame.to_srgb8_straight();
    let lin = |v: f64| {
        let v = v / 255.0;
        if v <= 0.04045 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
    };
    let enc = |v: f64| {
        let v = v.clamp(0.0, 1.0);
        255.0 * if v <= 0.0031308 { v * 12.92 } else { 1.055 * v.powf(1.0 / 2.4) - 0.055 }
    };
    let big: Vec<u8> = (0..CEL.1 * 3)
        .flat_map(|y| (0..CEL.0 * 3).map(move |x| (x, y)))
        .flat_map(|(x, y)| {
            let i = (y / 3 * CEL.0 + x / 3) * 4;
            let a = f64::from(bytes[i + 3]) / 255.0;
            let mix = |c: usize| enc(lin(f64::from(bytes[i + c])) * a + lin(BACK[c]) * (1.0 - a)).round() as u8;
            [mix(0), mix(1), mix(2), 255]
        })
        .collect();
    (bytes, big, said)
}

#[test]
fn b121_paraffin() {
    let mut t = Table::new(
        "paraffin",
        "# B-121: paraffin\n\nD-185, accepted by the owner on 2026-09-28. Every expected pixel \
         is `Fixtures/paraffin/expected_paraffin.json`, written by `tools/paraffin_reference.py` \
         before this code existed and printed in document 25 as FX-PARA-001 to 023. The build's \
         frame is compared sample by sample; the answer is the largest difference over all of \
         them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-PARA-001 to 023 (document 25)");
    t.fixtures("expected_paraffin.json");

    t.heading("How far it reaches");
    let got = wash("#6450a0", 90.0, 100.0, 100.0, "add").bounds_expansion();
    t.row("it grows the drawing's bounds by nothing: the wash stays on the covering", &got.to_string(), got == 0);
    let mut draft = added();
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview keeps every setting, none being a distance",
        &format!("{draft:?}"),
        draft == added(),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_para_001.json",
        "fx_para_007.json",
        "fx_para_008.json",
        "fx_para_013.json",
        "fx_para_015.json",
        "fx_para_016.json",
        "fx_para_017.json",
        "fx_para_018.json",
        "fx_para_021.json",
        "fx_para_022.json",
        "fx_para_023.json",
    ]);
    t.shape_refused(
        "fx_para_001.json",
        "no `spread` at all",
        r##"{"color": "#6450a0", "direction": 180, "opacity": 50, "blend": "multiply"}"##,
    );
    t.shape_refused(
        "fx_para_001.json",
        "a direction that is a word",
        r##"{"color": "#6450a0", "direction": "up", "spread": 70, "opacity": 50, "blend": "multiply"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_para_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("direction 361", set(wash("#6450a0", 361.0, 70.0, 50.0, "multiply"))),
            ("direction -1", set(wash("#6450a0", -1.0, 70.0, 50.0, "multiply"))),
            ("spread 101", set(wash("#6450a0", 180.0, 101.0, 50.0, "multiply"))),
            ("opacity 101", set(wash("#6450a0", 180.0, 70.0, 101.0, "multiply"))),
            ("blend \"color_burn\"", set(wash("#6450a0", 180.0, 70.0, 50.0, "color_burn"))),
            ("colour \"#12345\"", set(wash("#12345", 180.0, 70.0, 50.0, "multiply"))),
            ("spread keyed to 150", keys("spread", &[(0, &[50.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_para_001.json",
        vec![
            ("direction 360, spread 100 and opacity 100, the tops, soft light,", set(wash("#ffffff", 360.0, 100.0, 100.0, "soft_light"))),
            ("direction 0, spread 0, opacity 0, overlay,", set(wash("#000000", 0.0, 0.0, 0.0, "overlay"))),
            ("direction keyed from 0 to 360", keys("direction", &[(0, &[0.0]), (4, &[360.0])])),
        ],
    );

    t.heading("Pictures: a figure on a clear cel, in `verification/B-121 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/B-121 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = CEL;
    png_out::write_rgba(&dir.join("cel.png"), w, h, OutputDepth::Eight, &[], &cel()).unwrap();
    let (before, big, said) = picture(&dir, J::Array(vec![]));
    png_out::write_rgba(&dir.join("before.png"), w * 3, h * 3, OutputDepth::Eight, &[], &big).unwrap();
    t.row("before.png, the cel with no effect, draws cleanly", &format!("{said:?}"), said.is_empty());
    let (hair, face, shirt, shirt_right, skirt_left, leg, clear) =
        ((50, 9), (50, 27), (50, 58), (62, 58), (37, 85), (43, 110), (5, 5));
    for (name, e, what, changed, kept) in [
        ("as_added", added(), "as it is added, violet multiplied up from below", vec![shirt, leg], vec![hair, face, clear]),
        (
            "warm_from_above",
            wash("#ffb070", 0.0, 50.0, 70.0, "screen"),
            "a warm light screened down from above, spread 50, opacity 70",
            vec![hair, face],
            vec![leg, clear],
        ),
        (
            "blue_from_right",
            wash("#4060c0", 90.0, 60.0, 60.0, "multiply"),
            "a blue shade multiplied in from the right, spread 60, opacity 60",
            vec![shirt_right],
            vec![skirt_left, leg, clear],
        ),
        (
            "soft_light_from_below",
            wash("#6450a0", 180.0, 100.0, 100.0, "soft_light"),
            "violet in soft light from below, spread 100, opacity 100",
            vec![leg, face],
            vec![clear],
        ),
    ] {
        let Effect::Paraffin { color, direction, spread, opacity, blend } = &e else { unreachable!() };
        let effects = serde_json::json!([{
            "instance_id": "fx-0-0", "type_id": "core.paraffin", "enabled": true,
            "parameters": {"color": color, "direction": direction, "spread": spread, "opacity": opacity, "blend": blend},
        }]);
        let (bytes, big, said) = picture(&dir, effects);
        png_out::write_rgba(&dir.join(format!("{name}.png")), w * 3, h * 3, OutputDepth::Eight, &[], &big)
            .unwrap();
        let at = |b: &[u8], (x, y): (usize, usize)| b[(y * w + x) * 4..(y * w + x) * 4 + 4].to_vec();
        let moved = |p| at(&bytes, p) != at(&before, p);
        t.row(
            &format!("{name}.png, {what}, draws cleanly, changes {changed:?} and leaves {kept:?} exactly"),
            &format!(
                "{said:?}, changed {:?}, left {:?}",
                changed.iter().map(|&p| moved(p)).collect::<Vec<_>>(),
                kept.iter().map(|&p| !moved(p)).collect::<Vec<_>>()
            ),
            said.is_empty() && changed.iter().all(|&p| moved(p)) && kept.iter().all(|&p| !moved(p)),
        );
    }

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_para_001.json", 0), ("fx_para_017.json", 3)]);

    t.finish("B-121_paraffin_table.md");
}
