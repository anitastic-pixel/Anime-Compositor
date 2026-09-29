//! B-140: Kaleidoscope in the core, against D-205.
//!
//! Writes `verification/B-140_kaleidoscope_table.md`, and pictures in `verification/B-140 pictures/`.
//!
//! Every expected pixel is `Fixtures/kaleidoscope/expected_kaleidoscope.json`, written by
//! `tools/kaleidoscope_reference.py` before this code existed and printed in document 25 as
//! FX-KALEIDO-001 to 020. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// `[segments, rotation, size]`, the centre and the mirroring.
fn kaleidoscope(n: [f64; 3], center: [f64; 2], mode: &str) -> Effect {
    Effect::Kaleidoscope { segments: n[0], rotation: n[1], size: n[2], center, mode: mode.to_string() }
}

const START: [f64; 3] = [6.0, 0.0, 100.0];
const PLATE: (usize, usize) = (160, 100);

/// A small made-up drawing with nothing symmetric in it: a sky darkening upward, a sun, a hill
/// and a red kite on a dark string.
fn drawing() -> Vec<u8> {
    let (w, h) = PLATE;
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let (fx, fy) = (x as f64 + 0.5, y as f64 + 0.5);
            let mut c = [(40 + y) as u8, (90 + y) as u8, (200 - y) as u8];
            if (fx - 124.0).hypot(fy - 24.0) < 13.0 {
                c = [255, 210, 80];
            }
            if fy > 62.0 + 12.0 * (fx / 26.0).sin() {
                c = [60, 140, 70];
            }
            if ((fx - 92.0) * 2.5 + (fy - 30.0)).abs() < 2.5 && (76.0..92.0).contains(&fx) {
                c = [30, 30, 30];
            }
            if (fx - 92.0).abs() / 8.0 + (fy - 18.0).abs() / 12.0 < 1.0 {
                c = [220, 40, 60];
            }
            bytes.extend([c[0], c[1], c[2], 255]);
        }
    }
    bytes
}

/// The drawing as the composition's one layer, with `effects`, drawn, straight 8-bit; and the
/// same enlarged three times.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/kaleidoscope/fx_kaleido_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("drawing.png");
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
            bytes[i..i + 4].to_vec()
        })
        .collect();
    (bytes, big, said)
}

/// One Kaleidoscope, or none, as a layer's `effects`.
fn stack(steps: &[([f64; 3], [f64; 2], &str)]) -> J {
    J::Array(
        steps
            .iter()
            .enumerate()
            .map(|(i, (n, center, mode))| {
                serde_json::json!({
                    "instance_id": format!("fx-0-{i}"), "type_id": "core.kaleidoscope", "enabled": true,
                    "parameters": { "segments": n[0], "rotation": n[1], "size": n[2], "center": center, "mode": mode },
                })
            })
            .collect(),
    )
}

#[test]
fn b140_kaleidoscope() {
    let mut t = Table::new(
        "kaleidoscope",
        "# B-140: Kaleidoscope\n\nD-205, accepted on 2026-09-28 with the After Effects picks \
         (B3). Every expected pixel is `Fixtures/kaleidoscope/expected_kaleidoscope.json`, \
         written by `tools/kaleidoscope_reference.py` before this code existed and printed in \
         document 25 as FX-KALEIDO-001 to 020. The build's frame is compared sample by sample; \
         the answer is the largest difference over all of them, against the catalogue's tolerance \
         of 2e-5.\n",
    );

    t.heading("FX-KALEIDO-001 to 020 (document 25)");
    t.fixtures("expected_kaleidoscope.json");

    t.heading("How far it reaches");
    let got = kaleidoscope(START, [50.0, 50.0], "mirror").bounds_expansion();
    t.row("it never grows the layer: it declares no growth", &got.to_string(), got == 0);
    let mut draft = kaleidoscope([8.0, 30.0, 200.0], [25.0, 75.0], "repeat");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview keeps every setting, as none is a distance in pixels",
        &format!("{draft:?}"),
        draft == kaleidoscope([8.0, 30.0, 200.0], [25.0, 75.0], "repeat"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_kaleido_001.json",
        "fx_kaleido_003.json",
        "fx_kaleido_008.json",
        "fx_kaleido_009.json",
        "fx_kaleido_010.json",
        "fx_kaleido_011.json",
        "fx_kaleido_013.json",
        "fx_kaleido_014.json",
        "fx_kaleido_015.json",
        "fx_kaleido_016.json",
        "fx_kaleido_017.json",
        "fx_kaleido_018.json",
        "fx_kaleido_019.json",
        "fx_kaleido_020.json",
    ]);
    t.shape_refused(
        "fx_kaleido_001.json",
        "no `mode` at all",
        r#"{"segments": 6, "rotation": 0, "size": 100, "center": [50, 50]}"#,
    );
    t.shape_refused(
        "fx_kaleido_001.json",
        "a centre of one number",
        r#"{"segments": 6, "rotation": 0, "size": 100, "center": [50], "mode": "mirror"}"#,
    );
    t.shape_refused(
        "fx_kaleido_001.json",
        "segments that are a word",
        r#"{"segments": "six", "rotation": 0, "size": 100, "center": [50, 50], "mode": "mirror"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_kaleido_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("segments 1", set(kaleidoscope([1.0, 0.0, 100.0], [50.0, 50.0], "mirror"))),
            ("segments 33", set(kaleidoscope([33.0, 0.0, 100.0], [50.0, 50.0], "mirror"))),
            ("rotation -3601", set(kaleidoscope([6.0, -3601.0, 100.0], [50.0, 50.0], "mirror"))),
            ("size 9", set(kaleidoscope([6.0, 0.0, 9.0], [50.0, 50.0], "mirror"))),
            ("size 1001", set(kaleidoscope([6.0, 0.0, 1001.0], [50.0, 50.0], "mirror"))),
            ("centre down -1001", set(kaleidoscope(START, [50.0, -1001.0], "mirror"))),
            ("mirroring \"flower\"", set(kaleidoscope(START, [50.0, 50.0], "flower"))),
            ("mirroring \"Mirror\", written with a capital", set(kaleidoscope(START, [50.0, 50.0], "Mirror"))),
            ("segments keyed to 40", keys("segments", &[(0, &[6.0]), (4, &[40.0])])),
            ("centre keyed to 2000 across", keys("center", &[(0, &[50.0, 50.0]), (4, &[2000.0, 50.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_kaleido_001.json",
        vec![
            ("every number at its bottom,", set(kaleidoscope([2.0, -3600.0, 10.0], [-1000.0, -1000.0], "repeat"))),
            ("every number at its top,", set(kaleidoscope([32.0, 3600.0, 1000.0], [1000.0, 1000.0], "mirror"))),
            ("segments keyed from 2 to 12", keys("segments", &[(0, &[2.0]), (4, &[12.0])])),
            ("centre keyed across the drawing", keys("center", &[(0, &[0.0, 50.0]), (4, &[100.0, 50.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_kaleido_001.json", 0), ("fx_kaleido_007.json", 0), ("fx_kaleido_010.json", 2), ("fx_kaleido_013.json", 0)]);

    t.heading("Pictures: a made-up drawing, in `verification/B-140 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/B-140 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let write = |name: &str, bytes: &[u8], scale: usize| {
        png_out::write_rgba(&dir.join(name), w * scale, h * scale, OutputDepth::Eight, &[], bytes).unwrap()
    };
    write("drawing.png", &drawing(), 1);
    let px = |bytes: &[u8], (x, y): (usize, usize)| {
        let i = (y * w + x) * 4;
        [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]
    };
    let near = |a: [u8; 4], b: [u8; 4]| a.iter().zip(b).all(|(&u, v)| u.abs_diff(v) <= 1);
    let every = || (0..h).flat_map(move |y| (0..w).map(move |x| (x, y)));
    // The pixels whose middle lies well inside the wedge clockwise from `from` degrees, `span`
    // wide, round the point (cx, cy): the wedge the pattern is made of.
    let inside = |from: f64, span: f64, (cx, cy): (f64, f64)| {
        every().filter(move |&(x, y)| {
            let (dx, dy) = (x as f64 + 0.5 - cx, y as f64 + 0.5 - cy);
            let a = (dx.atan2(-dy).to_degrees() - from).rem_euclid(360.0);
            (1.0..span - 1.0).contains(&a) && dx.hypot(dy) > 2.0
        })
    };
    let clear = |bytes: &[u8]| every().filter(|&p| px(bytes, p)[3] < 255).count();
    let middle = (w as f64 / 2.0, h as f64 / 2.0);

    let (before, big, said) = picture(&dir, stack(&[]));
    write("before.png", &big, 3);
    t.row("before.png, the drawing with no effect, draws cleanly", &format!("{said:?}"), said.is_empty());

    let (six, big, said) = picture(&dir, stack(&[(START, [50.0, 50.0], "mirror")]));
    write("six_mirror.png", &big, 3);
    let (kept, all) = (inside(0.0, 60.0, middle).filter(|&p| near(px(&six, p), px(&before, p))).count(), inside(0.0, 60.0, middle).count());
    let halves = every().filter(|&(x, y)| near(px(&six, (x, y)), px(&six, (w - 1 - x, y)))).count();
    t.row(
        "six_mirror.png, as it starts, six wedges mirrored: the wedge from straight up clockwise \
         through 60 degrees is the drawing itself, every pixel inside it within 1 of before.png; \
         the left half the right half mirrored, every pixel; nothing clear; draws cleanly",
        &format!("{said:?}, {kept} of {all} in the wedge, {halves} of {} mirrored, {} clear", w * h, clear(&six)),
        said.is_empty() && all > 0 && kept == all && halves == w * h && clear(&six) == 0,
    );

    let (repeat, big, said) = picture(&dir, stack(&[(START, [50.0, 50.0], "repeat")]));
    write("six_repeat.png", &big, 3);
    let same = inside(0.0, 60.0, middle).filter(|&p| px(&repeat, p) == px(&six, p)).count();
    let next = inside(60.0, 60.0, middle).filter(|&p| px(&repeat, p) != px(&six, p)).count();
    t.row(
        "six_repeat.png, Repeat: the first wedge the same as six_mirror.png, every pixel, and the \
         next wedge different from it in most pixels, as it is turned rather than mirrored; nothing \
         clear; draws cleanly",
        &format!("{said:?}, first wedge {same} of {all} the same, next wedge {next} of {} different, {} clear", inside(60.0, 60.0, middle).count(), clear(&repeat)),
        said.is_empty() && same == all && next * 2 > inside(60.0, 60.0, middle).count() && clear(&repeat) == 0,
    );

    let (twelve, big, said) = picture(&dir, stack(&[([12.0, 0.0, 100.0], [50.0, 50.0], "mirror")]));
    write("twelve.png", &big, 3);
    let (kept, all) = (inside(0.0, 30.0, middle).filter(|&p| near(px(&twelve, p), px(&before, p))).count(), inside(0.0, 30.0, middle).count());
    t.row(
        "twelve.png, 12 segments: a finer pattern, its first wedge, 30 degrees wide, the drawing \
         itself; nothing clear; draws cleanly",
        &format!("{said:?}, {kept} of {all} in the wedge, {} clear", clear(&twelve)),
        said.is_empty() && all > 0 && kept == all && clear(&twelve) == 0 && twelve != six,
    );

    let (spun, big, said) = picture(&dir, stack(&[([6.0, 30.0, 100.0], [50.0, 50.0], "mirror")]));
    write("rotation_30.png", &big, 3);
    let (kept, all) = (inside(30.0, 60.0, middle).filter(|&p| near(px(&spun, p), px(&before, p))).count(), inside(30.0, 60.0, middle).count());
    t.row(
        "rotation_30.png, rotation 30: the pattern turned, the wedge from 30 degrees clockwise \
         through 90 now the drawing itself; draws cleanly",
        &format!("{said:?}, {kept} of {all} in the wedge"),
        said.is_empty() && all > 0 && kept == all && spun != six,
    );

    let (small, big, said) = picture(&dir, stack(&[([6.0, 0.0, 50.0], [50.0, 50.0], "mirror")]));
    write("size_50.png", &big, 3);
    t.row(
        "size_50.png, size 50: the pattern half as large, reaching past the drawing's edges, where \
         it reads the drawing mirrored back, so nothing is clear; draws cleanly",
        &format!("{said:?}, {} clear", clear(&small)),
        said.is_empty() && clear(&small) == 0 && small != six,
    );

    let (aside, big, said) = picture(&dir, stack(&[(START, [25.0, 50.0], "mirror")]));
    write("centre_25.png", &big, 3);
    let halves = every().filter(|&(x, y)| x < 80 && near(px(&aside, (x, y)), px(&aside, (79 - x, y)))).count();
    t.row(
        "centre_25.png, the centre a quarter of the way across: the pattern turns round that point, \
         the strip left of it the strip right of it mirrored, every pixel of the left half; nothing \
         clear; draws cleanly",
        &format!("{said:?}, {halves} of {} mirrored, {} clear", w * h / 2, clear(&aside)),
        said.is_empty() && halves == w * h / 2 && clear(&aside) == 0,
    );

    t.finish("B-140_kaleidoscope_table.md");
}
