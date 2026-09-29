//! B-145: Optics Compensation in the core, against D-210.
//!
//! Writes `verification/B-145_optics_compensation_table.md`, and pictures in
//! `verification/B-145 pictures/`.
//!
//! Every expected pixel is `Fixtures/optics_compensation/expected_optics_compensation.json`,
//! written by `tools/optics_compensation_reference.py` before this code existed and printed in
//! document 25 as FX-OPTICS-001 to 020. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// The field of view, reverse, orientation and centre.
fn optics(fov: f64, reverse: &str, orientation: &str, center: [f64; 2]) -> Effect {
    Effect::OpticsCompensation {
        field_of_view: fov,
        reverse: reverse.to_string(),
        orientation: orientation.to_string(),
        center,
    }
}

const PLATE: (usize, usize) = (160, 100);

/// A small made-up drawing: a cream card ruled every ten pixels, a blue frame round it, and a
/// red ball in the middle, so every bend shows in the lines.
fn drawing() -> Vec<u8> {
    let (w, h) = PLATE;
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let (fx, fy) = (x as f64 + 0.5, y as f64 + 0.5);
            let mut c = [240, 232, 210];
            if x % 10 == 0 || y % 10 == 0 {
                c = [70, 70, 90];
            }
            if x < 3 || y < 3 || x >= w - 3 || y >= h - 3 {
                c = [40, 80, 200];
            }
            if (fx - 80.0).hypot(fy - 50.0) < 14.0 {
                c = [220, 50, 60];
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
        &fs::read_to_string(effect_table::repo("Fixtures/optics_compensation/fx_optics_001.json")).unwrap(),
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

/// Optics Compensations, in order, as a layer's `effects`.
fn stack(steps: &[(f64, &str, &str, [f64; 2])]) -> J {
    J::Array(
        steps
            .iter()
            .enumerate()
            .map(|(i, (fov, reverse, orientation, center))| {
                serde_json::json!({
                    "instance_id": format!("fx-0-{i}"), "type_id": "core.optics_compensation", "enabled": true,
                    "parameters": { "field_of_view": fov, "reverse": reverse, "orientation": orientation, "center": center },
                })
            })
            .collect(),
    )
}

#[test]
fn b145_optics_compensation() {
    let mut t = Table::new(
        "optics_compensation",
        "# B-145: Optics Compensation\n\nD-210, accepted on 2026-09-28 with the After Effects \
         picks (B8). Every expected pixel is \
         `Fixtures/optics_compensation/expected_optics_compensation.json`, written by \
         `tools/optics_compensation_reference.py` before this code existed and printed in \
         document 25 as FX-OPTICS-001 to 020. The build's frame is compared sample by sample; the \
         answer is the largest difference over all of them, against the catalogue's tolerance of \
         2e-5.\n",
    );

    t.heading("FX-OPTICS-001 to 020 (document 25)");
    t.fixtures("expected_optics_compensation.json");

    t.heading("How far it reaches");
    let got = optics(90.0, "off", "horizontal", [50.0, 50.0]).bounds_expansion();
    t.row("it never grows the layer: it declares no growth", &got.to_string(), got == 0);
    let mut draft = optics(120.0, "on", "diagonal", [25.0, 75.0]);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview keeps every setting, as none is a distance in pixels",
        &format!("{draft:?}"),
        draft == optics(120.0, "on", "diagonal", [25.0, 75.0]),
    );

    t.heading("The file");
    t.round_trips(&(1..=20).map(|n| format!("fx_optics_{n:03}.json")).collect::<Vec<_>>().iter().map(String::as_str).collect::<Vec<_>>());
    t.shape_refused(
        "fx_optics_001.json",
        "no `orientation` at all",
        r#"{"field_of_view": 90, "reverse": "off", "center": [50, 50]}"#,
    );
    t.shape_refused(
        "fx_optics_001.json",
        "a centre of one number",
        r#"{"field_of_view": 90, "reverse": "off", "orientation": "horizontal", "center": [50]}"#,
    );
    t.shape_refused(
        "fx_optics_001.json",
        "a field of view that is a word",
        r#"{"field_of_view": "wide", "reverse": "off", "orientation": "horizontal", "center": [50, 50]}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_optics_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("field of view -1", set(optics(-1.0, "off", "horizontal", [50.0, 50.0]))),
            ("field of view 181", set(optics(181.0, "off", "horizontal", [50.0, 50.0]))),
            ("centre down -1001", set(optics(90.0, "off", "horizontal", [50.0, -1001.0]))),
            ("reverse \"yes\"", set(optics(90.0, "yes", "horizontal", [50.0, 50.0]))),
            ("orientation \"sideways\"", set(optics(90.0, "off", "sideways", [50.0, 50.0]))),
            ("orientation \"Horizontal\", written with a capital", set(optics(90.0, "off", "Horizontal", [50.0, 50.0]))),
            ("field of view keyed to 200", keys("field_of_view", &[(0, &[90.0]), (4, &[200.0])])),
            ("centre keyed to 2000 across", keys("center", &[(0, &[50.0, 50.0]), (4, &[2000.0, 50.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_optics_001.json",
        vec![
            ("every number at its bottom,", set(optics(0.0, "on", "vertical", [-1000.0, -1000.0]))),
            ("every number at its top,", set(optics(180.0, "off", "diagonal", [1000.0, 1000.0]))),
            ("field of view keyed from 0 to 120", keys("field_of_view", &[(0, &[0.0]), (4, &[120.0])])),
            ("centre keyed across the drawing", keys("center", &[(0, &[0.0, 50.0]), (4, &[100.0, 50.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_optics_001.json", 0),
        ("fx_optics_006.json", 0),
        ("fx_optics_010.json", 2),
        ("fx_optics_014.json", 0),
    ]);

    t.heading("Pictures: a made-up drawing, in `verification/B-145 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/B-145 pictures");
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
    let dist = |(x, y): (usize, usize), (cx, cy): (f64, f64)| (x as f64 + 0.5 - cx).hypot(y as f64 + 0.5 - cy);
    let clear = |bytes: &[u8]| every().filter(|&p| px(bytes, p)[3] == 0).count();
    // The four pixels round a point, which a lens there keeps.
    let round = |bytes: &[u8], other: &[u8], (cx, cy): (usize, usize)| {
        [(cx - 1, cy - 1), (cx, cy - 1), (cx - 1, cy), (cx, cy)].iter().all(|&p| near(px(bytes, p), px(other, p)))
    };
    // How far, on average over the pixels that show in both, one picture is from another.
    let apart = |a: &[u8], b: &[u8]| {
        let shown: Vec<_> = every().filter(|&p| px(a, p)[3] > 0 && px(b, p)[3] > 0).collect();
        let sum: u32 = shown.iter().map(|&p| (0..3).map(|c| px(a, p)[c].abs_diff(px(b, p)[c]) as u32).sum::<u32>()).sum();
        sum as f64 / (3 * shown.len()) as f64
    };
    let middle = (w as f64 / 2.0, h as f64 / 2.0);

    let (before, big, said) = picture(&dir, stack(&[]));
    write("before.png", &big, 3);
    t.row("before.png, the drawing with no effect, draws cleanly", &format!("{said:?}"), said.is_empty());

    let (fish, big, said) = picture(&dir, stack(&[(90.0, "off", "horizontal", [50.0, 50.0])]));
    write("fov_90.png", &big, 3);
    let edges = (0..h).filter(|&y| px(&fish, (0, y))[3] == 0 && px(&fish, (w - 1, y))[3] == 0).count();
    t.row(
        "fov_90.png, field of view 90: a fisheye, the middle four pixels kept, the lines bowed, the \
         left and right edges emptied from top to bottom, as they read from past the drawing; draws \
         cleanly",
        &format!("{said:?}, middle kept {}, {edges} of {h} rows empty at both ends, {} clear", round(&fish, &before, (80, 50)), clear(&fish)),
        said.is_empty() && round(&fish, &before, (80, 50)) && edges == h && fish != before,
    );

    let (wide, big, said) = picture(&dir, stack(&[(180.0, "off", "horizontal", [50.0, 50.0])]));
    write("fov_180.png", &big, 3);
    let out = every().filter(|&p| dist(p, middle) >= 80.0).count();
    let gone = every().filter(|&p| dist(p, middle) >= 80.0 && px(&wide, p)[3] == 0).count();
    t.row(
        "fov_180.png, field of view 180, the most: only the round part within 80 pixels of the middle \
         shows, every pixel farther out empty; draws cleanly",
        &format!("{said:?}, {gone} of {out} farther out empty"),
        said.is_empty() && out > 0 && gone == out && round(&wide, &before, (80, 50)),
    );

    let (back, big, said) = picture(&dir, stack(&[(90.0, "on", "horizontal", [50.0, 50.0])]));
    write("reverse.png", &big, 3);
    t.row(
        "reverse.png, reverse on: the edges stretched out, so the blue frame is pushed off the \
         sides; the middle kept, nothing clear; draws cleanly",
        &format!("{said:?}, middle kept {}, {} clear, left edge {:?}", round(&back, &before, (80, 50)), clear(&back), px(&back, (0, 50))),
        said.is_empty() && round(&back, &before, (80, 50)) && clear(&back) == 0 && px(&back, (0, 50)) != px(&before, (0, 50)),
    );

    let (tall, big, said) = picture(&dir, stack(&[(90.0, "off", "vertical", [50.0, 50.0])]));
    write("vertical.png", &big, 3);
    let (diag, big2, said2) = picture(&dir, stack(&[(90.0, "off", "diagonal", [50.0, 50.0])]));
    write("diagonal.png", &big2, 3);
    t.row(
        "vertical.png and diagonal.png: the 90 degrees across the height bends harder than across \
         the width, and across the diagonal more gently, so more of vertical.png is empty than of \
         fov_90.png, and less of diagonal.png; both draw cleanly",
        &format!("{said:?} {said2:?}, empty pixels: vertical {}, horizontal {}, diagonal {}", clear(&tall), clear(&fish), clear(&diag)),
        said.is_empty() && said2.is_empty() && clear(&tall) > clear(&fish) && clear(&fish) > clear(&diag),
    );

    let (aside, big, said) = picture(&dir, stack(&[(90.0, "off", "horizontal", [25.0, 50.0])]));
    write("centre_25.png", &big, 3);
    t.row(
        "centre_25.png, the centre a quarter of the way across: the lens's middle moves there and \
         the four pixels round it are kept; the picture differs from fov_90.png; draws cleanly",
        &format!("{said:?}, kept {}", round(&aside, &before, (40, 50))),
        said.is_empty() && round(&aside, &before, (40, 50)) && aside != fish,
    );

    let (lens, big, said) = picture(&dir, stack(&[(60.0, "off", "horizontal", [50.0, 50.0])]));
    write("fov_60.png", &big, 3);
    let (undone, big2, said2) =
        picture(&dir, stack(&[(60.0, "off", "horizontal", [50.0, 50.0]), (60.0, "on", "horizontal", [50.0, 50.0])]));
    write("in_then_out.png", &big2, 3);
    let (a, b) = (apart(&lens, &before), apart(&undone, &before));
    t.row(
        "fov_60.png, then in_then_out.png, the same lens put in and taken out again: back to the \
         drawing, softer, as each thin ruled line is sampled twice, but on average closer to \
         before.png than the fisheye alone; both draw cleanly",
        &format!("{said:?} {said2:?}, average difference: fisheye {a:.2}, put in and taken out {b:.2}"),
        said.is_empty() && said2.is_empty() && b < a,
    );

    t.finish("B-145_optics_compensation_table.md");
}
