//! B-139: Snowfall in the core, against D-204.
//!
//! Writes `verification/B-139_snowfall_table.md`, and pictures in `verification/B-139 pictures/`.
//!
//! Every expected pixel is `Fixtures/snowfall/expected_snowfall.json`, written by
//! `tools/snowfall_reference.py` before this code existed and printed in document 25 as
//! FX-SNOW-001 to 029. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// `[density, spacing, size, depth, speed, wind, wiggle, period, seed, opacity]`.
fn snowfall(n: [f64; 10], color: &str) -> Effect {
    Effect::Snowfall {
        color: color.to_string(),
        density: n[0],
        spacing: n[1],
        size: n[2],
        depth: n[3],
        speed: n[4],
        wind: n[5],
        wiggle: n[6],
        period: n[7],
        seed: n[8],
        opacity: n[9],
        frame: 0,
    }
}

const START: [f64; 10] = [50.0, 32.0, 6.0, 50.0, 2.0, 0.5, 3.0, 48.0, 0.0, 100.0];

fn with(i: usize, v: f64) -> Effect {
    let mut n = START;
    n[i] = v;
    snowfall(n, "#ffffff")
}

/// The picture's size: a dark sky with a clear margin round it.
const PLATE: (usize, usize) = (160, 90);
const SKY: [u8; 3] = [30, 26, 36];

fn inside(x: usize, y: usize) -> bool {
    (10..150).contains(&x) && (10..80).contains(&y)
}

/// The sky with `settings` on it at `frame`, straight 8-bit; and the same enlarged three times
/// over a grey check where it is clear.
fn picture(dir: &Path, settings: J, frame: i32) -> (Vec<u8>, Vec<u8>) {
    let mut project: J =
        serde_json::from_str(&fs::read_to_string(effect_table::repo("Fixtures/snowfall/fx_snow_001.json")).unwrap())
            .unwrap();
    project["assets"][0]["path"] = J::from("sky.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(PLATE.0);
    comp["height"] = J::from(PLATE.1);
    comp["duration_frames"] = J::from(48);
    comp["work_area"]["end_frame_exclusive"] = J::from(48);
    let layer = &mut comp["layers"][0];
    layer["out_frame"] = J::from(48);
    let middle = serde_json::json!([PLATE.0 as f64 / 2.0, PLATE.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"][0]["parameters"] = settings;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &Id::new(effect_table::MAIN), frame, dir, 64, &mut log)
        .expect("the picture draws");
    assert!(log.finish().is_empty(), "the picture draws without a warning");
    let bytes = drawn.to_srgb8_straight();
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
    (bytes, big)
}

fn settings(changes: &[(&str, J)]) -> J {
    let mut s = serde_json::json!({
        "color": "#ffffff", "density": 50, "spacing": 32, "size": 6, "depth": 50, "speed": 2,
        "wind": 0.5, "wiggle": 3, "period": 48, "seed": 0, "opacity": 100
    });
    for (k, v) in changes {
        s[*k] = v.clone();
    }
    s
}

#[test]
fn b139_snowfall() {
    let mut t = Table::new(
        "snowfall",
        "# B-139: Snowfall\n\nD-204, accepted on 2026-09-28 by the owner's \"take everything\", \
         which took in the After Effects picks B1 to B12; this is B2. Every expected pixel is \
         `Fixtures/snowfall/expected_snowfall.json`, written by `tools/snowfall_reference.py` \
         before this code existed and printed in document 25 as FX-SNOW-001 to 029. The build's \
         frame is compared sample by sample; the answer is the largest difference over all of \
         them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-SNOW-001 to 029 (document 25)");
    t.fixtures("expected_snowfall.json");

    t.heading("How far it reaches");
    let got = with(2, 100.0).bounds_expansion();
    t.row("size 100 grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = snowfall([50.0, 32.0, 6.0, 50.0, 2.0, -1.0, 3.0, 48.0, 5.0, 100.0], "#ffffff");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the spacing, size, speed, wind and wiggle, and nothing else",
        &format!("{draft:?}"),
        draft == snowfall([50.0, 16.0, 3.0, 50.0, 1.0, -0.5, 1.5, 48.0, 5.0, 100.0], "#ffffff"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_snow_001.json",
        "fx_snow_015.json",
        "fx_snow_017.json",
        "fx_snow_018.json",
        "fx_snow_021.json",
        "fx_snow_022.json",
        "fx_snow_023.json",
        "fx_snow_024.json",
        "fx_snow_025.json",
        "fx_snow_026.json",
        "fx_snow_027.json",
        "fx_snow_028.json",
        "fx_snow_029.json",
    ]);
    let params = t.saved_parameters("fx_snow_016.json");
    t.row(
        "fx_snow_016.json, its colour written in capitals, is saved in small letters, as Rain's is",
        &params["color"].to_string(),
        params["color"] == serde_json::json!("#a0c8ff"),
    );
    t.shape_refused(
        "fx_snow_001.json",
        "no `wind` at all",
        r##"{"color": "#ffffff", "density": 50, "spacing": 32, "size": 6, "depth": 50, "speed": 2,
            "wiggle": 3, "period": 48, "seed": 0, "opacity": 100}"##,
    );
    t.shape_refused(
        "fx_snow_001.json",
        "a size that is a word",
        r##"{"color": "#ffffff", "density": 50, "spacing": 32, "size": "big", "depth": 50, "speed": 2,
            "wind": 0.5, "wiggle": 3, "period": 48, "seed": 0, "opacity": 100}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_snow_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("density 101", set(with(0, 101.0))),
            ("spacing 1", set(with(1, 1.0))),
            ("size 101", set(with(2, 101.0))),
            ("scene depth -1", set(with(3, -1.0))),
            ("speed 1001", set(with(4, 1001.0))),
            ("wind -1001", set(with(5, -1001.0))),
            ("wiggle 101", set(with(6, 101.0))),
            ("period 0.5", set(with(7, 0.5))),
            ("seed 100001", set(with(8, 100001.0))),
            ("opacity 101", set(with(9, 101.0))),
            ("colour \"#fffff\"", set(snowfall(START, "#fffff"))),
            ("colour \"white\"", set(snowfall(START, "white"))),
            ("opacity keyed to 150", keys("opacity", &[(0, &[100.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_snow_001.json",
        vec![
            (
                "every number at its bottom,",
                set(snowfall([0.0, 2.0, 0.0, 0.0, 0.0, -1000.0, 0.0, 1.0, 0.0, 0.0], "#000000")),
            ),
            (
                "every number at its top,",
                set(snowfall(
                    [100.0, 1000.0, 100.0, 100.0, 1000.0, 1000.0, 100.0, 1000.0, 100000.0, 100.0],
                    "#ffffff",
                )),
            ),
            ("opacity keyed from 0 to 100", keys("opacity", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_snow_002.json", 0), ("fx_snow_010.json", 4), ("fx_snow_020.json", 0)]);

    t.heading("Pictures: snow on a night sky, in `verification/B-139 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/B-139 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let write = |name: &str, bytes: &[u8], scale: usize| {
        png_out::write_rgba(&dir.join(name), w * scale, h * scale, OutputDepth::Eight, &[], bytes).unwrap()
    };
    let sky: Vec<u8> = (0..w * h)
        .flat_map(|i| if inside(i % w, i / w) { [SKY[0], SKY[1], SKY[2], 255] } else { [0; 4] })
        .collect();
    write("sky.png", &sky, 1);
    // How many pixels of the sky the snow lights, and whether any outside it shows.
    let lit = |bytes: &[u8]| (0..w * h).filter(|&i| bytes[4 * i] > SKY[0] + 8).count();
    let spilt = |bytes: &[u8]| (0..w * h).any(|i| !inside(i % w, i / w) && bytes[4 * i + 3] != 0);
    let shot = |name: &str, what: &str, s: J, frame: i32| {
        let (bytes, big) = picture(&dir, s, frame);
        write(name, &big, 3);
        (bytes, what.to_string())
    };
    let pictures = [
        shot("start_frame_0.png", "as it starts, frame 0", settings(&[]), 0),
        shot("start_frame_24.png", "as it starts, frame 24", settings(&[]), 24),
        shot("depth_0.png", "scene depth 0", settings(&[("depth", J::from(0))]), 0),
        shot("depth_100.png", "scene depth 100", settings(&[("depth", J::from(100))]), 0),
        shot(
            "blizzard.png",
            "a blizzard: density 100, spacing 12, size 3, wind 4, speed 6",
            settings(&[
                ("density", J::from(100)),
                ("spacing", J::from(12)),
                ("size", J::from(3)),
                ("wind", J::from(4)),
                ("speed", J::from(6)),
            ]),
            0,
        ),
    ];
    for (bytes, what) in &pictures {
        let n = lit(bytes);
        t.row(
            &format!("{what}: snow lights some of the sky, and nothing shows outside it"),
            &format!("{n} of {} sky pixels lit; spilt: {}", 140 * 70, spilt(bytes)),
            n > 0 && !spilt(bytes),
        );
    }
    t.row(
        "frame 24 is not frame 0: the snow has fallen on",
        "compared",
        pictures[0].0 != pictures[1].0,
    );
    t.row(
        "the blizzard lights more of the sky than the start",
        &format!("{} against {}", lit(&pictures[4].0), lit(&pictures[0].0)),
        lit(&pictures[4].0) > 3 * lit(&pictures[0].0),
    );

    t.finish("B-139_snowfall_table.md");
}
