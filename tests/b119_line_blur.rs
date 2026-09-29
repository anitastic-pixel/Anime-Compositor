//! B-119: line blur in the core, against D-183.
//!
//! Writes `verification/B-119_line_blur_table.md`.
//!
//! Every expected pixel is `Fixtures/line_blur/expected_line_blur.json`, written by
//! `tools/line_blur_reference.py` before this code existed and printed in document 25 as
//! FX-LBLUR-001 to 016. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws a jagged ink drawing, black strokes on white paper with no smoothing, and
//! puts it through Line Blur three ways into `verification/B-119 pictures/`, three times
//! enlarged, for the owner to see the steps smooth out.

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

fn blur(length: f64, strength: f64, lines_only: &str) -> Effect {
    Effect::LineBlur {
        length,
        strength,
        lines_only: lines_only.to_string(),
    }
}

const PAPER: (usize, usize) = (200, 120);

/// Black strokes on white paper, each pixel wholly one or the other: a ring, a shallow and a
/// steep line, and a wave, the steps a scan or a hard pen leaves.
fn drawing() -> Vec<u8> {
    let segment = |x: f64, y: f64, (ax, ay): (f64, f64), (bx, by): (f64, f64)| {
        let (dx, dy) = (bx - ax, by - ay);
        let t = (((x - ax) * dx + (y - ay) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
        (x - ax - t * dx).hypot(y - ay - t * dy)
    };
    let mut bytes = Vec::new();
    for y in 0..PAPER.1 {
        for x in 0..PAPER.0 {
            let (x, y) = (x as f64, y as f64);
            let ink = ((x - 50.0).hypot(y - 60.0) - 35.0).abs() < 1.0
                || segment(x, y, (110.0, 12.0), (190.0, 36.0)) < 0.9
                || segment(x, y, (112.0, 48.0), (140.0, 110.0)) < 0.9
                || (x >= 150.0 && x <= 192.0 && (y - 80.0 - 14.0 * (x / 9.0).sin()).abs() < 1.0);
            let v = if ink { 0 } else { 255 };
            bytes.extend([v, v, v, 255]);
        }
    }
    bytes
}

/// The drawing as the composition's one layer, with `effects`, drawn and enlarged three times.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/line_blur/fx_lblur_002.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("drawing.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(PAPER.0);
    comp["height"] = J::from(PAPER.1);
    let layer = &mut comp["layers"][0];
    let middle = serde_json::json!([PAPER.0 as f64 / 2.0, PAPER.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(effect_table::MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    let bytes = frame.to_srgb8_straight();
    let big: Vec<u8> = (0..PAPER.1 * 3)
        .flat_map(|y| (0..PAPER.0 * 3).map(move |x| (y / 3 * PAPER.0 + x / 3) * 4))
        .flat_map(|i| bytes[i..i + 4].to_vec())
        .collect();
    (bytes, big, said)
}

#[test]
fn b119_line_blur() {
    let mut t = Table::new(
        "line_blur",
        "# B-119: line blur\n\nD-183, accepted by the owner on 2026-09-28. Every expected pixel \
         is `Fixtures/line_blur/expected_line_blur.json`, written by \
         `tools/line_blur_reference.py` before this code existed and printed in document 25 as \
         FX-LBLUR-001 to 016. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-LBLUR-001 to 016 (document 25)");
    t.fixtures("expected_line_blur.json");

    t.heading("How far it reaches");
    for (length, strength, want) in [(0.0, 100.0, 0), (0.5, 100.0, 3), (4.0, 0.0, 3), (50.0, 100.0, 3)] {
        let got = blur(length, strength, "off").bounds_expansion();
        t.row(
            &format!("length {length} at strength {strength} grows the drawing's bounds by {want}"),
            &got.to_string(),
            got == want,
        );
    }
    let mut draft = blur(8.0, 60.0, "on");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the length and keeps the strength and lines only",
        &format!("{draft:?}"),
        draft == blur(4.0, 60.0, "on"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_lblur_002.json",
        "fx_lblur_005.json",
        "fx_lblur_007.json",
        "fx_lblur_011.json",
        "fx_lblur_013.json",
        "fx_lblur_016.json",
    ]);
    t.shape_refused(
        "fx_lblur_002.json",
        "no `lines_only` at all",
        r##"{"length": 4, "strength": 100}"##,
    );
    t.shape_refused(
        "fx_lblur_002.json",
        "a length that is a word",
        r##"{"length": "long", "strength": 100, "lines_only": "off"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_lblur_002.json").document;
    t.refused(
        &mut document,
        vec![
            ("length 51", set(blur(51.0, 100.0, "off"))),
            ("length -1", set(blur(-1.0, 100.0, "off"))),
            ("strength 101", set(blur(4.0, 101.0, "off"))),
            ("lines only \"yes\"", set(blur(4.0, 100.0, "yes"))),
            ("length keyed to 60", keys("length", &[(0, &[0.0]), (4, &[60.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_lblur_002.json",
        vec![
            ("length 50 at strength 100, the top of the ranges,", set(blur(50.0, 100.0, "on"))),
            ("length 0 at strength 0, the bottom,", set(blur(0.0, 0.0, "off"))),
            ("strength keyed from 0 to 100", keys("strength", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("Pictures: a jagged ink drawing, in `verification/B-119 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/B-119 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PAPER;
    png_out::write_rgba(&dir.join("drawing.png"), w, h, OutputDepth::Eight, &[], &drawing()).unwrap();
    let (before, big, said) = picture(&dir, J::Array(vec![]));
    png_out::write_rgba(&dir.join("before.png"), w * 3, h * 3, OutputDepth::Eight, &[], &big).unwrap();
    t.row("before.png, the drawing with no effect, draws cleanly", &format!("{said:?}"), said.is_empty());
    for (name, length, lines_only, what) in [
        ("length_4", 4, "off", "the settings it is added with"),
        ("length_12", 12, "off", "a longer pass"),
        ("lines_only", 4, "on", "lines only on"),
    ] {
        let effects = serde_json::json!([{
            "instance_id": "fx-0-0", "type_id": "core.line_blur", "enabled": true,
            "parameters": { "length": length, "strength": 100, "lines_only": lines_only },
        }]);
        let (bytes, big, said) = picture(&dir, effects);
        png_out::write_rgba(&dir.join(format!("{name}.png")), w * 3, h * 3, OutputDepth::Eight, &[], &big)
            .unwrap();
        let changed = bytes.chunks(4).zip(before.chunks(4)).filter(|(a, b)| a != b).count();
        // Two pixels of paper, each well away from every stroke and from the paper's edge.
        let paper = [(60, 5), (100, 110)].iter().all(|&(x, y)| {
            let i = (y * w + x) * 4;
            bytes[i..i + 4] == before[i..i + 4]
        });
        t.row(
            &format!("{name}.png, {what}, draws cleanly, changes the strokes and leaves the empty paper white"),
            &format!("{said:?}, {changed} of {} pixels changed", w * h),
            said.is_empty() && changed > 0 && paper,
        );
    }

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_lblur_003.json", 0), ("fx_lblur_008.json", 3)]);

    t.finish("B-119_line_blur_table.md");
}
