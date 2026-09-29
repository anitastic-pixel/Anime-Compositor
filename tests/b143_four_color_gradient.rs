//! B-143: 4-Color Gradient in the core, against D-208.
//!
//! Writes `verification/B-143_four_color_gradient_table.md`, and pictures in
//! `verification/B-143 pictures/`.
//!
//! Every expected pixel is `Fixtures/four_color_gradient/expected_four_color_gradient.json`,
//! written by `tools/four_color_gradient_reference.py` before this code existed and printed in
//! document 25 as FX-4CG-001 to 026. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// The four points, the four colours, blend, opacity and the blending mode.
fn gradient(p: [[f64; 2]; 4], c: [&str; 4], blend: f64, opacity: f64, mode: &str) -> Effect {
    Effect::FourColorGradient {
        point_1: p[0],
        point_2: p[1],
        point_3: p[2],
        point_4: p[3],
        color_1: c[0].to_string(),
        color_2: c[1].to_string(),
        color_3: c[2].to_string(),
        color_4: c[3].to_string(),
        blend,
        opacity,
        blending_mode: mode.to_string(),
    }
}

const CORNERS: [[f64; 2]; 4] = [[10.0, 10.0], [90.0, 10.0], [10.0, 90.0], [90.0, 90.0]];
const AE: [&str; 4] = ["#ffff00", "#00ff00", "#ff00ff", "#0000ff"];
const PLATE: (usize, usize) = (160, 100);

/// The gradient as it starts, with another blend, opacity and mode.
fn plain(blend: f64, opacity: f64, mode: &str) -> Effect {
    gradient(CORNERS, AE, blend, opacity, mode)
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
        &fs::read_to_string(effect_table::repo("Fixtures/four_color_gradient/fx_4cg_001.json")).unwrap(),
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

/// One 4-Color Gradient, or none, as a layer's `effects`.
fn stack(steps: &[&Effect]) -> J {
    J::Array(
        steps
            .iter()
            .enumerate()
            .map(|(i, e)| {
                let Effect::FourColorGradient {
                    point_1,
                    point_2,
                    point_3,
                    point_4,
                    color_1,
                    color_2,
                    color_3,
                    color_4,
                    blend,
                    opacity,
                    blending_mode,
                } = e
                else {
                    unreachable!()
                };
                serde_json::json!({
                    "instance_id": format!("fx-0-{i}"), "type_id": "core.four_color_gradient", "enabled": true,
                    "parameters": { "point_1": point_1, "point_2": point_2, "point_3": point_3,
                        "point_4": point_4, "color_1": color_1, "color_2": color_2,
                        "color_3": color_3, "color_4": color_4, "blend": blend,
                        "opacity": opacity, "blending_mode": blending_mode },
                })
            })
            .collect(),
    )
}

#[test]
fn b143_four_color_gradient() {
    let mut t = Table::new(
        "four_color_gradient",
        "# B-143: 4-Color Gradient\n\nD-208, accepted on 2026-09-28 with the After Effects picks \
         (B6). Every expected pixel is \
         `Fixtures/four_color_gradient/expected_four_color_gradient.json`, written by \
         `tools/four_color_gradient_reference.py` before this code existed and printed in \
         document 25 as FX-4CG-001 to 026. The build's frame is compared sample by sample; the \
         answer is the largest difference over all of them, against the catalogue's tolerance of \
         2e-5.\n",
    );

    t.heading("FX-4CG-001 to 026 (document 25)");
    t.fixtures("expected_four_color_gradient.json");

    t.heading("How far it reaches");
    let got = plain(100.0, 100.0, "normal").bounds_expansion();
    t.row("it never grows the layer: it declares no growth", &got.to_string(), got == 0);
    let mut draft = plain(60.0, 80.0, "multiply");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: the points are in per cent, and blend and opacity are not distances",
        &format!("{draft:?}"),
        draft == plain(60.0, 80.0, "multiply"),
    );

    t.heading("The file");
    let files: Vec<String> = (1..=26).filter(|&n| n != 10).map(|n| format!("fx_4cg_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let params = t.saved_parameters("fx_4cg_010.json");
    t.row(
        "fx_4cg_010.json, its colours written in capitals, is saved in small letters, as Snowfall's is",
        &format!("{} {} {} {}", params["color_1"], params["color_2"], params["color_3"], params["color_4"]),
        params["color_1"] == serde_json::json!("#ff0000")
            && params["color_2"] == serde_json::json!("#00ff00")
            && params["color_3"] == serde_json::json!("#0000ff")
            && params["color_4"] == serde_json::json!("#ffffff"),
    );
    t.shape_refused(
        "fx_4cg_001.json",
        "no `blending_mode` at all",
        r##"{"point_1": [10, 10], "point_2": [90, 10], "point_3": [10, 90], "point_4": [90, 90], "color_1": "#ffff00", "color_2": "#00ff00", "color_3": "#ff00ff", "color_4": "#0000ff", "blend": 100, "opacity": 100}"##,
    );
    t.shape_refused(
        "fx_4cg_001.json",
        "a point 1 that is one number",
        r##"{"point_1": 10, "point_2": [90, 10], "point_3": [10, 90], "point_4": [90, 90], "color_1": "#ffff00", "color_2": "#00ff00", "color_3": "#ff00ff", "color_4": "#0000ff", "blend": 100, "opacity": 100, "blending_mode": "normal"}"##,
    );
    t.shape_refused(
        "fx_4cg_001.json",
        "a colour 3 that is a number",
        r##"{"point_1": [10, 10], "point_2": [90, 10], "point_3": [10, 90], "point_4": [90, 90], "color_1": "#ffff00", "color_2": "#00ff00", "color_3": 3, "color_4": "#0000ff", "blend": 100, "opacity": 100, "blending_mode": "normal"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_4cg_001.json").document;
    let moved = |k: usize, p: [f64; 2]| {
        let mut points = CORNERS;
        points[k] = p;
        gradient(points, AE, 100.0, 100.0, "normal")
    };
    let painted = |k: usize, c: &'static str| {
        let mut colors = AE;
        colors[k] = c;
        gradient(CORNERS, colors, 100.0, 100.0, "normal")
    };
    t.refused(
        &mut document,
        vec![
            ("blend 0", set(plain(0.0, 100.0, "normal"))),
            ("blend 1001", set(plain(1001.0, 100.0, "normal"))),
            ("opacity -1", set(plain(100.0, -1.0, "normal"))),
            ("opacity 101", set(plain(100.0, 101.0, "normal"))),
            ("point 1 -1001 across", set(moved(0, [-1001.0, 10.0]))),
            ("point 4 1001 down", set(moved(3, [90.0, 1001.0]))),
            ("colour 2 \"#12345\"", set(painted(1, "#12345"))),
            ("colour 4 \"blue\"", set(painted(3, "blue"))),
            ("blending mode \"darken\"", set(plain(100.0, 100.0, "darken"))),
            ("blending mode \"Normal\", written with a capital", set(plain(100.0, 100.0, "Normal"))),
            ("opacity keyed to 101", keys("opacity", &[(0, &[100.0]), (4, &[101.0])])),
            ("blend keyed to 0", keys("blend", &[(0, &[100.0]), (4, &[0.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_4cg_001.json",
        vec![
            (
                "every number at its bottom,",
                set(gradient([[-1000.0; 2]; 4], ["#000000"; 4], 1.0, 0.0, "add")),
            ),
            (
                "every number at its top,",
                set(gradient([[1000.0; 2]; 4], ["#FFFFFF"; 4], 1000.0, 100.0, "screen")),
            ),
            ("blend keyed from 1 to 1000", keys("blend", &[(0, &[1.0]), (4, &[1000.0])])),
            ("point 2 keyed from (90, 10) to (10, 90)", keys("point_2", &[(0, &[90.0, 10.0]), (4, &[10.0, 90.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_4cg_001.json", 0), ("fx_4cg_002.json", 0), ("fx_4cg_007.json", 0), ("fx_4cg_013.json", 2)]);

    t.heading("Pictures: a made-up cel, in `verification/B-143 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/B-143 pictures");
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
    // Within `d` of a colour, the covering aside.
    let close = |p: [u8; 4], c: [u8; 3], d: u8| p[..3].iter().zip(c).all(|(&u, v)| u.abs_diff(v) <= d);

    let (start, big, said) = picture(&dir, stack(&[&plain(100.0, 100.0, "normal")]));
    write("as_it_starts.png", &big, 3);
    let [tl, tr, bl, br] = [(16, 14), (143, 14), (16, 85), (143, 85)].map(|p| px(&start, p));
    t.row(
        "as_it_starts.png, as it starts: yellow in the top-left corner of the card, green in the top-right, \
         magenta in the bottom-left and blue in the bottom-right, the drawing's lines painted over; every \
         covering kept, nothing around the card; draws cleanly",
        &format!("{said:?}, corners {tl:?} {tr:?} {bl:?} {br:?}"),
        said.is_empty()
            && kept(&start)
            && tl[0] > 200 && tl[1] > 200 && tl[2] < 60
            && tr[1] > 200 && tr[0] < 100 && tr[2] < 100
            && bl[0] > 200 && bl[2] > 200 && bl[1] < 60
            && br[2] > 200 && br[0] < 60 && br[1] < 60,
    );

    let (hard, big, said) = picture(&dir, stack(&[&plain(1.0, 100.0, "normal")]));
    write("blend_1.png", &big, 3);
    let quarters = [((40, 30), [255, 255, 0]), ((120, 30), [0, 255, 0]), ((40, 70), [255, 0, 255]), ((120, 70), [0, 0, 255])];
    t.row(
        "blend_1.png, blend 1: four flat patches, each point's own colour, yellow, green, magenta and blue, \
         to within 1 in each quarter; draws cleanly",
        &format!("{said:?}, {:?}", quarters.map(|(p, _)| px(&hard, p))),
        said.is_empty() && kept(&hard) && quarters.iter().all(|&(p, c)| close(px(&hard, p), c, 1)),
    );

    let (even, big, said) = picture(&dir, stack(&[&plain(1000.0, 100.0, "normal")]));
    write("blend_1000.png", &big, 3);
    let range = |bytes: &[u8]| {
        (0..3).map(|c| {
            let v: Vec<u8> = shown().map(|p| px(bytes, p)[c]).collect();
            v.iter().max().unwrap() - v.iter().min().unwrap()
        }).max().unwrap()
    };
    t.row(
        "blend_1000.png, blend 1000: the four mixed nearly evenly, every channel across the card spanning \
         under half of what it spans as it starts; draws cleanly",
        &format!("{said:?}, widest channel {} against {}", range(&even), range(&start)),
        said.is_empty() && kept(&even) && u16::from(range(&even)) * 2 < u16::from(range(&start)),
    );

    let (half, big, said) = picture(&dir, stack(&[&plain(100.0, 50.0, "normal")]));
    write("opacity_50.png", &big, 3);
    let ring = (80, 21);
    t.row(
        "opacity_50.png, opacity 50: the colours laid on at half strength, the drawing's dark line \
         showing through, darker at the head's top than as it starts; draws cleanly",
        &format!("{said:?}, {:?} against {:?} at the head's top", px(&half, ring), px(&start, ring)),
        said.is_empty() && kept(&half) && (0..3).all(|c| px(&half, ring)[c] < px(&start, ring)[c]),
    );

    for (file, what, mode, opacity, darker) in [
        ("multiply.png", "multiply: tinted and darkened, the lines kept dark, no pixel lighter than the drawing", "multiply", 100.0, true),
        ("screen.png", "screen: lightened, no pixel darker than the drawing", "screen", 100.0, false),
        ("add_30.png", "add at opacity 30: lightened, no pixel darker than the drawing", "add", 30.0, false),
    ] {
        let (after, big, said) = picture(&dir, stack(&[&plain(100.0, opacity, mode)]));
        write(file, &big, 3);
        let moved = shown().filter(|&p| px(&after, p) != px(&before, p)).count();
        let wrong = shown()
            .filter(|&p| {
                (0..3).any(|c| {
                    let step = i32::from(px(&after, p)[c]) - i32::from(px(&before, p)[c]);
                    if darker { step > 1 } else { step < -1 }
                })
            })
            .count();
        t.row(
            &format!("{file}, {what}; every covering kept; draws cleanly"),
            &format!("{said:?}, {moved} pixels changed, {wrong} the wrong way"),
            said.is_empty() && kept(&after) && moved > 0 && wrong == 0,
        );
    }

    let (middle, big, said) = picture(&dir, stack(&[&gradient([[50.0, 50.0], CORNERS[1], CORNERS[2], CORNERS[3]], AE, 100.0, 100.0, "normal")]));
    write("point_1_middle.png", &big, 3);
    t.row(
        "point_1_middle.png, point 1 moved to the middle: yellow in the middle of the head, at 80, 50; draws cleanly",
        &format!("{said:?}, {:?} at 80, 50", px(&middle, (80, 50))),
        said.is_empty() && kept(&middle) && close(px(&middle, (80, 50)), [255, 255, 0], 12),
    );

    let sunset = gradient([[20.0, 0.0], [80.0, 0.0], [20.0, 100.0], [80.0, 100.0]], ["#ffd060", "#ff6040", "#6040a0", "#203070"], 100.0, 100.0, "multiply");
    let (dusk, big, said) = picture(&dir, stack(&[&sunset]));
    write("sunset_multiply.png", &big, 3);
    let (top, bottom) = (px(&dusk, (80, 14)), px(&dusk, (80, 86)));
    t.row(
        "sunset_multiply.png, gold, orange, purple and navy laid on by multiply: warm along the top, red over \
         blue, and cool along the bottom, blue over red; draws cleanly",
        &format!("{said:?}, {top:?} at the top, {bottom:?} at the bottom"),
        said.is_empty() && kept(&dusk) && top[0] > top[2] && bottom[2] > bottom[0],
    );

    t.finish("B-143_four_color_gradient_table.md");
}
