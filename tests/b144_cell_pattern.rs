//! B-144: Cell Pattern in the core, against D-209.
//!
//! Writes `verification/B-144_cell_pattern_table.md`, and pictures in
//! `verification/B-144 pictures/`.
//!
//! Every expected pixel is `Fixtures/cell_pattern/expected_cell_pattern.json`, written by
//! `tools/cell_pattern_reference.py` before this code existed and printed in document 25 as
//! FX-CELL-001 to 033. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// The pattern, invert, the six numbers (contrast, disperse, size, evolution, seed, opacity), the
/// two colours and the blend.
fn cells(pattern: &str, invert: &str, n: [f64; 6], dark: &str, light: &str, blend: &str) -> Effect {
    Effect::CellPattern {
        pattern: pattern.to_string(),
        invert: invert.to_string(),
        contrast: n[0],
        disperse: n[1],
        size: n[2],
        evolution: n[3],
        seed: n[4],
        dark_color: dark.to_string(),
        light_color: light.to_string(),
        opacity: n[5],
        blend: blend.to_string(),
    }
}

/// Size 16, the rest as it starts.
const START: [f64; 6] = [100.0, 1.0, 16.0, 0.0, 0.0, 100.0];
const PLATE: (usize, usize) = (160, 100);

/// Bubbles, black to white, normal, with other numbers.
fn plain(n: [f64; 6]) -> Effect {
    cells("bubbles", "off", n, "#000000", "#ffffff", "normal")
}

/// START with the `k`th number changed to `v`.
fn with(k: usize, v: f64) -> Effect {
    let mut n = START;
    n[k] = v;
    plain(n)
}

/// A small made-up cel: a grey card with a dark line round it, a round head of skin on it with a
/// dark line round that and its right part in shadow, and nothing around the card.
fn drawing() -> Vec<u8> {
    let (w, h) = PLATE;
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let r = ((x as f64 - 80.0).powi(2) + (y as f64 - 50.0).powi(2)).sqrt();
            bytes.extend(if !(12..=148).contains(&x) || !(10..=90).contains(&y) {
                [0, 0, 0, 0]
            } else if x <= 13 || x >= 147 || y <= 11 || y >= 89 || (27.0..30.0).contains(&r) {
                [30, 26, 36, 255]
            } else if r < 27.0 && x >= 88 {
                [220, 160, 140, 255]
            } else if r < 27.0 {
                [246, 214, 190, 255]
            } else {
                [190, 200, 214, 255]
            });
        }
    }
    bytes
}

/// The drawing as the composition's one layer, with `effects`, drawn, straight 8-bit; and the
/// same enlarged three times.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/cell_pattern/fx_cell_001.json")).unwrap(),
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

/// One Cell Pattern, or none, as a layer's `effects`.
fn stack(steps: &[&Effect]) -> J {
    J::Array(
        steps
            .iter()
            .enumerate()
            .map(|(i, e)| {
                let Effect::CellPattern {
                    pattern,
                    invert,
                    contrast,
                    disperse,
                    size,
                    evolution,
                    seed,
                    dark_color,
                    light_color,
                    opacity,
                    blend,
                } = e
                else {
                    unreachable!()
                };
                serde_json::json!({
                    "instance_id": format!("fx-0-{i}"), "type_id": "core.cell_pattern", "enabled": true,
                    "parameters": { "pattern": pattern, "invert": invert, "contrast": contrast,
                        "disperse": disperse, "size": size, "evolution": evolution, "seed": seed,
                        "dark_color": dark_color, "light_color": light_color, "opacity": opacity,
                        "blend": blend },
                })
            })
            .collect(),
    )
}

#[test]
fn b144_cell_pattern() {
    let mut t = Table::new(
        "cell_pattern",
        "# B-144: Cell Pattern\n\nD-209, accepted on 2026-09-28 with the After Effects picks \
         (B7). Every expected pixel is `Fixtures/cell_pattern/expected_cell_pattern.json`, \
         written by `tools/cell_pattern_reference.py` before this code existed and printed in \
         document 25 as FX-CELL-001 to 033. The build's frame is compared sample by sample; the \
         answer is the largest difference over all of them, against the catalogue's tolerance of \
         2e-5.\n",
    );

    t.heading("FX-CELL-001 to 033 (document 25)");
    t.fixtures("expected_cell_pattern.json");

    t.heading("How far it reaches");
    let got = plain(START).bounds_expansion();
    t.row("it never grows the layer: it declares no growth", &got.to_string(), got == 0);
    let mut draft = plain(START);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the size, 16 to 8, and nothing else",
        &format!("{draft:?}"),
        draft == with(2, 8.0),
    );
    let mut draft = with(2, 1.5);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a draft never takes the size under 1: 1.5 is held at 1, as Fractal Noise's is",
        &format!("{draft:?}"),
        draft == with(2, 1.0),
    );

    t.heading("The file");
    let files: Vec<String> = (1..=33).filter(|&n| n != 14).map(|n| format!("fx_cell_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let params = t.saved_parameters("fx_cell_014.json");
    t.row(
        "fx_cell_014.json, its light colour written in capitals, is saved in small letters, as Snowfall's is",
        &format!("{} {}", params["dark_color"], params["light_color"]),
        params["dark_color"] == serde_json::json!("#203070") && params["light_color"] == serde_json::json!("#ffd060"),
    );
    t.shape_refused(
        "fx_cell_001.json",
        "no `blend` at all",
        r##"{"pattern": "bubbles", "invert": "off", "contrast": 100, "disperse": 1, "size": 4, "evolution": 0, "seed": 0, "dark_color": "#000000", "light_color": "#ffffff", "opacity": 100}"##,
    );
    t.shape_refused(
        "fx_cell_001.json",
        "a size that is a word",
        r##"{"pattern": "bubbles", "invert": "off", "contrast": 100, "disperse": 1, "size": "big", "evolution": 0, "seed": 0, "dark_color": "#000000", "light_color": "#ffffff", "opacity": 100, "blend": "normal"}"##,
    );
    t.shape_refused(
        "fx_cell_001.json",
        "an invert that is a number",
        r##"{"pattern": "bubbles", "invert": 1, "contrast": 100, "disperse": 1, "size": 4, "evolution": 0, "seed": 0, "dark_color": "#000000", "light_color": "#ffffff", "opacity": 100, "blend": "normal"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_cell_001.json").document;
    let worded = |pattern: &str, invert: &str, dark: &str, light: &str, blend: &str| {
        cells(pattern, invert, START, dark, light, blend)
    };
    t.refused(
        &mut document,
        vec![
            ("contrast -1", set(with(0, -1.0))),
            ("contrast 1001", set(with(0, 1001.0))),
            ("disperse -0.1", set(with(1, -0.1))),
            ("disperse 1.6", set(with(1, 1.6))),
            ("size 0", set(with(2, 0.0))),
            ("size 1001", set(with(2, 1001.0))),
            ("evolution 100001", set(with(3, 100001.0))),
            ("seed -1", set(with(4, -1.0))),
            ("seed 100001", set(with(4, 100001.0))),
            ("opacity 101", set(with(5, 101.0))),
            ("pattern \"tubular\"", set(worded("tubular", "off", "#000000", "#ffffff", "normal"))),
            ("pattern \"Bubbles\", written with a capital", set(worded("Bubbles", "off", "#000000", "#ffffff", "normal"))),
            ("invert \"yes\"", set(worded("bubbles", "yes", "#000000", "#ffffff", "normal"))),
            ("dark colour \"#12345\"", set(worded("bubbles", "off", "#12345", "#ffffff", "normal"))),
            ("light colour \"white\"", set(worded("bubbles", "off", "#000000", "white", "normal"))),
            ("blend \"darken\"", set(worded("bubbles", "off", "#000000", "#ffffff", "darken"))),
            ("disperse keyed to 1.6", keys("disperse", &[(0, &[1.0]), (4, &[1.6])])),
            ("size keyed to 0", keys("size", &[(0, &[4.0]), (4, &[0.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_cell_001.json",
        vec![
            (
                "every number at its bottom,",
                set(cells("static_plates", "on", [0.0, 0.0, 1.0, -100000.0, 0.0, 0.0], "#000000", "#000000", "add")),
            ),
            (
                "every number at its top,",
                set(cells("plates", "off", [1000.0, 1.5, 1000.0, 100000.0, 100000.0, 100.0], "#FFFFFF", "#FFFFFF", "screen")),
            ),
            ("evolution keyed from 0 to 3600", keys("evolution", &[(0, &[0.0]), (4, &[3600.0])])),
            ("seed keyed from 0 to 100", keys("seed", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_cell_001.json", 0),
        ("fx_cell_004.json", 0),
        ("fx_cell_013.json", 0),
        ("fx_cell_019.json", 2),
        ("fx_cell_022.json", 0),
    ]);

    t.heading("Pictures: a made-up cel, in `verification/B-144 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/B-144 pictures");
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
    let every = || (0..h).flat_map(move |y| (0..w).map(move |x| (x, y)));
    let (before, big, said) = picture(&dir, stack(&[]));
    write("before.png", &big, 3);
    t.row("before.png, the drawing with no effect, draws cleanly", &format!("{said:?}"), said.is_empty());

    // Every pixel keeps its covering: the empty ones stay empty and the card's edge stays where it is.
    let kept = |after: &[u8]| every().all(|p| px(after, p)[3] == px(&before, p)[3]);
    let shown = || every().filter(|&p| px(&before, p)[3] > 0);
    let grey = |bytes: &[u8]| shown().all(|p| {
        let q = px(bytes, p);
        q[0].abs_diff(q[1]) <= 1 && q[1].abs_diff(q[2]) <= 1
    });
    let mean = |bytes: &[u8]| {
        let v: Vec<f64> = shown().map(|p| f64::from(px(bytes, p)[1])).collect();
        v.iter().sum::<f64>() / v.len() as f64
    };
    let count = |bytes: &[u8], f: &dyn Fn(u8) -> bool| shown().filter(|&p| f(px(bytes, p)[1])).count();

    let (start, big, said) = picture(&dir, stack(&[&plain(START)]));
    write("bubbles.png", &big, 3);
    let (lo, hi) = shown().fold((255, 0), |(lo, hi), p| (px(&start, p)[1].min(lo), px(&start, p)[1].max(hi)));
    t.row(
        "bubbles.png, bubbles sixteen pixels a cell: grey balls over the whole card, white at their \
         middles and near black where they meet, the drawing under them gone; every covering kept, \
         nothing around the card; draws cleanly",
        &format!("{said:?}, greys from {lo} to {hi}"),
        said.is_empty() && kept(&start) && grey(&start) && lo < 40 && hi > 230,
    );

    let (crystal, big, said) = picture(&dir, stack(&[&cells("crystals", "off", START, "#000000", "#ffffff", "normal")]));
    write("crystals.png", &big, 3);
    t.row(
        "crystals.png, crystals: faceted cones, the same cells darker on the whole than the bubbles; draws cleanly",
        &format!("{said:?}, average grey {:.0} against the bubbles' {:.0}", mean(&crystal), mean(&start)),
        said.is_empty() && kept(&crystal) && grey(&crystal) && mean(&crystal) + 20.0 < mean(&start),
    );

    let (plate, big, said) = picture(&dir, stack(&[&cells("plates", "off", START, "#000000", "#ffffff", "normal")]));
    write("plates.png", &big, 3);
    let white = count(&plate, &|v| v == 255);
    let total = shown().count();
    t.row(
        "plates.png, plates: flat white plates, dark only in the seams, over four in ten pixels pure white, \
         against under one in ten in the bubbles; draws cleanly",
        &format!("{said:?}, {white} of {total} pixels white, the bubbles {}", count(&start, &|v| v == 255)),
        said.is_empty() && kept(&plate) && grey(&plate) && white * 10 > total * 4
            && count(&start, &|v| v == 255) * 10 < total && count(&plate, &|v| v < 40) > 0,
    );

    let (flat, big, said) = picture(&dir, stack(&[&cells("static_plates", "off", START, "#000000", "#ffffff", "normal")]));
    write("static_plates.png", &big, 3);
    let pairs = (0..h).flat_map(|y| (0..w - 1).map(move |x| (x, y)))
        .filter(|&(x, y)| px(&before, (x, y))[3] > 0 && px(&before, (x + 1, y))[3] > 0);
    let (same, all) = pairs.fold((0, 0), |(s, a), (x, y)| (s + usize::from(px(&flat, (x, y)) == px(&flat, (x + 1, y))), a + 1));
    t.row(
        "static_plates.png, static plates: each cell one flat grey, so nine in ten pixels match the one to their right; draws cleanly",
        &format!("{said:?}, {same} of {all} matching"),
        said.is_empty() && kept(&flat) && grey(&flat) && same * 10 > all * 9,
    );

    let (turned, big, said) = picture(&dir, stack(&[&cells("bubbles", "on", START, "#000000", "#ffffff", "normal")]));
    write("invert.png", &big, 3);
    let off = shown().map(|p| (i32::from(px(&turned, p)[1]) + i32::from(px(&start, p)[1]) - 255).abs()).max().unwrap();
    t.row(
        "invert.png, invert on: the bubbles turned over, dark balls on light seams, each pixel and the bubbles' same pixel adding to white within 2; draws cleanly",
        &format!("{said:?}, furthest {off}"),
        said.is_empty() && kept(&turned) && off <= 2,
    );

    let (hard, big, said) = picture(&dir, stack(&[&with(0, 300.0)]));
    write("contrast_300.png", &big, 3);
    let ends = |bytes: &[u8]| count(bytes, &|v| v == 0 || v == 255);
    t.row(
        "contrast_300.png, contrast 300: the greys pushed out to black and white, far more pixels at one or the other; draws cleanly",
        &format!("{said:?}, {} pixels at black or white against {}", ends(&hard), ends(&start)),
        said.is_empty() && kept(&hard) && ends(&hard) > 2 * ends(&start),
    );

    let (grid, big, said) = picture(&dir, stack(&[&with(1, 0.0)]));
    write("disperse_0.png", &big, 3);
    let repeats = shown().filter(|&(x, y)| x + 16 < w && px(&before, (x + 16, y))[3] > 0)
        .all(|(x, y)| px(&grid, (x, y)) == px(&grid, (x + 16, y)));
    t.row(
        "disperse_0.png, disperse 0: every point in its cell's middle, the balls in a square grid repeating every 16 pixels across; draws cleanly",
        &format!("{said:?}, repeats {repeats}"),
        said.is_empty() && kept(&grid) && repeats,
    );

    let (moved, big, said) = picture(&dir, stack(&[&with(3, 90.0)]));
    write("evolution_90.png", &big, 3);
    let changed = shown().filter(|&p| px(&moved, p) != px(&start, p)).count();
    t.row(
        "evolution_90.png, evolution 90: the points a quarter of the way round their circles, the balls moved, most pixels changed from the bubbles; draws cleanly",
        &format!("{said:?}, {changed} of {total} changed"),
        said.is_empty() && kept(&moved) && changed * 2 > total,
    );

    let (dusk, big, said) = picture(&dir, stack(&[&cells("bubbles", "off", START, "#203070", "#ffd060", "multiply")]));
    write("navy_gold_multiply.png", &big, 3);
    let lightest = shown().filter(|&p| px(&before, p) == [190, 200, 214, 255]).max_by_key(|&p| px(&start, p)[1]).unwrap();
    let darkest = shown().filter(|&p| px(&before, p) == [190, 200, 214, 255]).min_by_key(|&p| px(&start, p)[1]).unwrap();
    let (l, d) = (px(&dusk, lightest), px(&dusk, darkest));
    t.row(
        "navy_gold_multiply.png, navy to gold laid on by multiply: the card warm in the balls, red over blue, \
         and cool in the seams, blue over red, the drawing's lines still dark; draws cleanly",
        &format!("{said:?}, {l:?} in a ball, {d:?} in a seam"),
        said.is_empty() && kept(&dusk) && l[0] > l[2] && d[2] > d[0] && px(&dusk, (12, 50))[0] < 40,
    );

    t.finish("B-144_cell_pattern_table.md");
}
