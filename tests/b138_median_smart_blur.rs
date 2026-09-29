//! B-138: Median and Smart Blur in the core, against D-203.
//!
//! Writes `verification/B-138_median_smart_blur_table.md`.
//!
//! Every expected pixel is `Fixtures/median_smart_blur/expected_median_smart_blur.json`, written
//! by `tools/median_smart_blur_reference.py` before this code existed and printed in document 25
//! as FX-MEDIAN-001 to 011 and FX-SMART-001 to 012. Tolerance 2e-5. Nothing here is a snapshot
//! of a run.
//!
//! It also draws a face with grainy skin, specks and pinholes, cleans and smooths it into
//! `verification/B-138 pictures/`, three times enlarged, over a grey check where it is clear.

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

fn median(radius: f64, operate_on_alpha: &str) -> Effect {
    Effect::Median { radius, operate_on_alpha: operate_on_alpha.to_string() }
}

fn smart(radius: f64, threshold: f64) -> Effect {
    Effect::SmartBlur { radius, threshold }
}

const PLATE: (usize, usize) = (120, 80);
const MIDDLE: (f64, f64) = (60.0, 42.0);
const SKIN: [u8; 3] = [246, 214, 190];
const LINE: [u8; 3] = [30, 26, 36];

/// What the face's pixel is: clear, the line round it and its eyes, a strand of hair one pixel
/// wide, grainy skin, or a speck.
#[derive(Clone, Copy, PartialEq)]
enum Part {
    Clear,
    Line,
    Hair,
    Skin,
    Speck,
}

/// How far a pixel's middle is from the face's.
fn from_middle((x, y): (usize, usize)) -> f64 {
    (x as f64 + 0.5 - MIDDLE.0).hypot(y as f64 + 0.5 - MIDDLE.1)
}

/// A drawn face as a scan gives it: skin with grain of ten levels either way, a line three
/// pixels wide round it, two eyes, a one-pixel strand of hair, and sixty specks of white, of
/// line or of nothing.
fn face() -> (Vec<u8>, Vec<Part>) {
    let mut seed = 138u32;
    let mut rnd = |n: u32| {
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        (seed >> 8) % n
    };
    let (w, h) = PLATE;
    let mut parts = Vec::new();
    let mut bytes = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let d = from_middle((x, y));
            let eye = |ex: f64| (x as f64 + 0.5 - ex).hypot(y as f64 + 0.5 - 38.0) < 4.0;
            let (part, c) = if d > 34.0 {
                (Part::Clear, [0, 0, 0, 0])
            } else if d > 31.0 || eye(48.0) || eye(72.0) {
                (Part::Line, [LINE[0], LINE[1], LINE[2], 255])
            } else if y == 14 + (x.saturating_sub(40)) / 3 && (40..82).contains(&x) {
                (Part::Hair, [LINE[0], LINE[1], LINE[2], 255])
            } else {
                let g = rnd(21) as i32 - 10;
                let c = |v: u8| (v as i32 + g).clamp(0, 255) as u8;
                (Part::Skin, [c(SKIN[0]), c(SKIN[1]), c(SKIN[2]), 255])
            };
            parts.push(part);
            bytes.extend(c);
        }
    }
    let mut placed = 0;
    while placed < 60 {
        let i = (rnd(h as u32) as usize) * w + rnd(w as u32) as usize;
        // Specks away from the line and the eyes, so a check of the skin sees each whole.
        if parts[i] == Part::Skin && from_middle((i % w, i / w)) < 27.0 {
            parts[i] = Part::Speck;
            let c = [[255, 255, 255, 255], [LINE[0], LINE[1], LINE[2], 255], [0, 0, 0, 0]][rnd(3) as usize];
            bytes[4 * i..4 * i + 4].copy_from_slice(&c);
            placed += 1;
        }
    }
    (bytes, parts)
}

/// `plate` as the composition's one layer, with `effects`, drawn, straight 8-bit; and the same
/// enlarged three times over a grey check where it is clear.
fn picture(dir: &Path, plate: &str, effects: J) -> (Vec<u8>, Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/median_smart_blur/fx_median_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from(plate);
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

/// One effect as a layer's `effects`.
fn one(type_id: &str, parameters: J) -> J {
    serde_json::json!([{ "instance_id": "fx-0-0", "type_id": type_id, "enabled": true, "parameters": parameters }])
}

#[test]
fn b138_median_smart_blur() {
    let mut t = Table::new(
        "median_smart_blur",
        "# B-138: Median and Smart Blur\n\nD-203, accepted on 2026-09-28 with the After Effects \
         picks (B1). Every expected pixel is \
         `Fixtures/median_smart_blur/expected_median_smart_blur.json`, written by \
         `tools/median_smart_blur_reference.py` before this code existed and printed in document \
         25 as FX-MEDIAN-001 to 011 and FX-SMART-001 to 012. The build's frame is compared \
         sample by sample; the answer is the largest difference over all of them, against the \
         catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-MEDIAN-001 to 011 and FX-SMART-001 to 012 (document 25)");
    t.fixtures("expected_median_smart_blur.json");

    t.heading("How far they reach");
    let got = [median(10.0, "on").bounds_expansion(), smart(10.0, 255.0).bounds_expansion()];
    t.row("neither ever grows the layer: each declares no growth", &format!("{got:?}"), got == [0, 0]);
    let mut draft = median(4.0, "on");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves Median's radius, 4 to 2, and keeps the word",
        &format!("{draft:?}"),
        draft == median(2.0, "on"),
    );
    let mut draft = smart(3.0, 64.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves Smart Blur's radius, 3 to 1.5, and keeps the \
         threshold, which is colour, not distance",
        &format!("{draft:?}"),
        draft == smart(1.5, 64.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_median_004.json",
        "fx_median_005.json",
        "fx_median_006.json",
        "fx_median_008.json",
        "fx_median_009.json",
        "fx_median_010.json",
        "fx_median_011.json",
        "fx_smart_003.json",
        "fx_smart_006.json",
        "fx_smart_007.json",
        "fx_smart_009.json",
        "fx_smart_010.json",
        "fx_smart_011.json",
        "fx_smart_012.json",
    ]);
    t.shape_refused("fx_median_001.json", "a Median with no `radius`", r#"{"operate_on_alpha": "off"}"#);
    t.shape_refused("fx_median_001.json", "a Median with no `operate_on_alpha`", r#"{"radius": 2}"#);
    t.shape_refused(
        "fx_median_001.json",
        "a Median whose operate on alpha is a number",
        r#"{"radius": 2, "operate_on_alpha": 1}"#,
    );
    t.shape_refused("fx_smart_001.json", "a Smart Blur with no `threshold`", r#"{"radius": 3}"#);
    t.shape_refused(
        "fx_smart_001.json",
        "a Smart Blur whose threshold is a word",
        r#"{"radius": 3, "threshold": "high"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_median_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("Median radius 10.5", set(median(10.5, "off"))),
            ("Median radius -0.5", set(median(-0.5, "off"))),
            ("operate on alpha \"sometimes\"", set(median(2.0, "sometimes"))),
            ("operate on alpha \"On\", written with a capital", set(median(2.0, "On"))),
            ("Median radius keyed to 20", keys("radius", &[(0, &[0.0]), (4, &[20.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_median_001.json",
        vec![
            ("Median radius 0,", set(median(0.0, "off"))),
            ("operate on alpha on,", set(median(2.0, "on"))),
            ("Median radius keyed from 0 to 10", keys("radius", &[(0, &[0.0]), (4, &[10.0])])),
        ],
    );
    let mut document = t.load("fx_smart_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("Smart Blur radius 11", set(smart(11.0, 64.0))),
            ("threshold 256", set(smart(3.0, 256.0))),
            ("threshold -1", set(smart(3.0, -1.0))),
            ("threshold keyed to 300", keys("threshold", &[(0, &[0.0]), (4, &[300.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_smart_001.json",
        vec![
            ("threshold 0,", set(smart(3.0, 0.0))),
            ("threshold 255,", set(smart(3.0, 255.0))),
            ("threshold keyed from 0 to 64", keys("threshold", &[(0, &[0.0]), (4, &[64.0])])),
        ],
    );

    t.heading("Pictures: a scanned face cleaned and smoothed, in `verification/B-138 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/B-138 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let write = |name: &str, bytes: &[u8], scale: usize| {
        png_out::write_rgba(&dir.join(name), w * scale, h * scale, OutputDepth::Eight, &[], bytes).unwrap()
    };
    let (plate, parts) = face();
    write("face.png", &plate, 1);
    let px = |bytes: &[u8], i: usize| [bytes[4 * i], bytes[4 * i + 1], bytes[4 * i + 2], bytes[4 * i + 3]];
    let far = |p: [u8; 4], c: [u8; 3]| p[3] < 255 || (0..3).any(|k| p[k].abs_diff(c[k]) > 24);
    // The skin well inside the line, where a speck is the only thing far from skin.
    let inside = |i: usize| from_middle((i % w, i / w)) < 27.0 && !matches!(parts[i], Part::Line | Part::Hair);
    let off_skin = |bytes: &[u8]| (0..w * h).filter(|&i| inside(i) && far(px(bytes, i), SKIN)).count();
    let holes = |bytes: &[u8]| (0..w * h).filter(|&i| from_middle((i % w, i / w)) < 31.0 && px(bytes, i)[3] == 0).count();
    let dark = |bytes: &[u8], part: Part| {
        let of: Vec<usize> = (0..w * h).filter(|&i| parts[i] == part).collect();
        (of.iter().filter(|&&i| !far(px(bytes, i), LINE)).count(), of.len())
    };
    // How much the skin changes from one pixel to the next, across grain only.
    let grain = |bytes: &[u8]| {
        let pairs: Vec<usize> = (0..w * h - 1).filter(|&i| parts[i] == Part::Skin && parts[i + 1] == Part::Skin).collect();
        let d: u32 = pairs.iter().map(|&i| (0..3).map(|k| bytes[4 * i + k].abs_diff(bytes[4 * i + 4 + k]) as u32).sum::<u32>()).sum();
        d as f64 / pairs.len() as f64
    };
    let spill = |bytes: &[u8]| (0..w * h).filter(|&i| parts[i] == Part::Clear && px(bytes, i)[3] > 0).count();

    let (before, big, said) = picture(&dir, "face.png", J::Array(vec![]));
    write("before.png", &big, 3);
    t.row(
        "before.png, the face with no effect: specks off the skin, pinholes, grain; draws cleanly",
        &format!("{said:?}, {} pixels off the skin, {} pinholes, grain {:.1}", off_skin(&before), holes(&before), grain(&before)),
        said.is_empty() && off_skin(&before) > 0 && holes(&before) > 0,
    );

    let (cleaned, big, said) =
        picture(&dir, "face.png", one("core.median", serde_json::json!({ "radius": 2, "operate_on_alpha": "off" })));
    write("median_2.png", &big, 3);
    let (line, of) = dark(&cleaned, Part::Line);
    // The hair where it is more than three pixels from the line it runs into.
    let loose: Vec<usize> = (0..w * h).filter(|&i| parts[i] == Part::Hair && from_middle((i % w, i / w)) < 28.0).collect();
    let (hair, hairs) = (loose.iter().filter(|&&i| !far(px(&cleaned, i), LINE)).count(), loose.len());
    t.row(
        "median_2.png, Median at its default radius of 2: no white or dark speck left on the skin, \
         the pinholes still clear as the covering is each pixel's own, the line round the face and \
         the eyes kept, at least 19 of their pixels in 20 still line-dark, and the one-pixel hair \
         taken away, as a line thinner than the radius is, wherever it is more than three pixels \
         from the line it runs into; draws cleanly",
        &format!(
            "{said:?}, {} off the skin of which {} pinholes, line {line} of {of}, hair {hair} of {hairs}",
            off_skin(&cleaned),
            holes(&cleaned)
        ),
        said.is_empty()
            && off_skin(&cleaned) == holes(&cleaned)
            && holes(&cleaned) == holes(&before)
            && line * 20 >= of * 19
            && hair == 0,
    );

    let (filled, big, said) =
        picture(&dir, "face.png", one("core.median", serde_json::json!({ "radius": 2, "operate_on_alpha": "on" })));
    write("median_2_alpha_on.png", &big, 3);
    t.row(
        "median_2_alpha_on.png, Median 2 with Operate on Alpha on: the pinholes filled too, nothing \
         off the skin and nothing clear inside the line; nothing spills past the face; draws cleanly",
        &format!("{said:?}, {} off the skin, {} pinholes, {} spilt", off_skin(&filled), holes(&filled), spill(&filled)),
        said.is_empty() && off_skin(&filled) == 0 && holes(&filled) == 0 && spill(&filled) == 0,
    );

    let (smooth, big, said) =
        picture(&dir, "face.png", one("core.smart_blur", serde_json::json!({ "radius": 3, "threshold": 64 })));
    write("smart_blur_64.png", &big, 3);
    let (line, of) = dark(&smooth, Part::Line);
    let same_line = (0..w * h).filter(|&i| parts[i] == Part::Line && px(&smooth, i) == px(&before, i)).count();
    t.row(
        "smart_blur_64.png, Smart Blur at its defaults, radius 3 and threshold 64: the grain less \
         than half what it was, and every pixel of the line and the eyes exactly as it was, as the \
         skin is too far from them to be mixed in; draws cleanly",
        &format!("{said:?}, grain {:.1} from {:.1}, line {same_line} of {of} unchanged", grain(&smooth), grain(&before)),
        said.is_empty() && grain(&smooth) * 2.0 < grain(&before) && same_line == of && line == of,
    );

    let (blurred, big, said) =
        picture(&dir, "face.png", one("core.smart_blur", serde_json::json!({ "radius": 3, "threshold": 255 })));
    write("smart_blur_255.png", &big, 3);
    let (line, of) = dark(&blurred, Part::Line);
    t.row(
        "smart_blur_255.png, Smart Blur at threshold 255 mixes everything that shows, a plain blur \
         inside the drawing: the line softened into the skin, fewer than 9 of its pixels in 10 still \
         line-dark, and still nothing past the face's edge, as clear pixels are never mixed in; \
         draws cleanly",
        &format!("{said:?}, line {line} of {of} still dark, {} spilt", spill(&blurred)),
        said.is_empty() && line * 10 < of * 9 && spill(&blurred) == 0,
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_median_001.json", 0),
        ("fx_median_004.json", 0),
        ("fx_median_005.json", 2),
        ("fx_smart_001.json", 0),
        ("fx_smart_007.json", 2),
    ]);

    t.finish("B-138_median_smart_blur_table.md");
}
