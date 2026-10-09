//! B-64: Lens Blur's iris and highlights in the core, against D-121.
//!
//! Writes `verification/B-64_lens_iris_table.md`.
//!
//! Every expected pixel is `Fixtures/lens_blur/expected_lens_blur.json`, written by
//! `tools/lens_blur_reference.py` before this code existed and printed in document 25 as
//! FX-LENS-019 to 044. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

/// A lens blur with the iris words and numbers of D-121.
fn lens(radius: f64, edges: &str, iris: &str, numbers: [f64; 5]) -> Effect {
    let [roundness, rotation, aspect, highlight_gain, highlight_threshold] = numbers;
    Effect::LensBlur {
        radius,
        edges: edges.into(),
        iris: iris.into(),
        roundness,
        rotation,
        aspect,
        highlight_gain,
        highlight_threshold,
        layer: "".into(),
        fit: "center".into(),
        channel: "luminance".into(),
        focal_distance: 0.0,
        invert: "off".into(),
        map: None,
    }
}

const START: [f64; 5] = [0.0, 0.0, 1.0, 0.0, 100.0];

#[test]
fn b64_lens_iris() {
    let mut t = Table::new(
        "lens_blur",
        "# B-64: lens blur iris and highlights\n\nD-121, proposed on 2026-09-26. Every expected \
         pixel is `Fixtures/lens_blur/expected_lens_blur.json`, written by \
         `tools/lens_blur_reference.py` before this code existed and printed in document 25 as \
         FX-LENS-019 to 044. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5. \
         FX-LENS-001 to 018, the round iris of D-116, are walked by B-59 and must not move.\n",
    );

    t.heading("FX-LENS-019 to 044 (document 25)");
    t.fixtures_numbered("expected_lens_blur.json", 19..=44);

    t.heading("How far it reaches");
    for (what, e, want) in [
        (
            "radius 10, aspect 1, grows the drawing by 10 on every side, as before",
            lens(10.0, "transparent", "hexagon", START),
            10,
        ),
        (
            "radius 1.5, aspect 2, grows it by 3: the radius times the square root of 2, rounded up",
            lens(1.5, "transparent", "circle", [0.0, 0.0, 2.0, 0.0, 100.0]),
            3,
        ),
        (
            "radius 1.5, aspect 0.5, grows it by 3 as well",
            lens(1.5, "transparent", "circle", [0.0, 0.0, 0.5, 0.0, 100.0]),
            3,
        ),
        (
            "radius 10, aspect 2, with the edge pixels repeated does not grow it",
            lens(10.0, "repeat", "square", [0.0, 0.0, 2.0, 0.0, 100.0]),
            0,
        ),
    ] {
        let got = e.bounds_expansion();
        t.row(what, &got.to_string(), got == want);
    }
    let settings = [50.0, 45.0, 2.0, 3.0, 80.0];
    let mut draft = lens(10.0, "repeat", "hexagon", settings);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the radius, and keeps the iris, its turn, its stretch \
         and the highlights",
        &format!("{draft:?}"),
        draft == lens(5.0, "repeat", "hexagon", settings),
    );

    t.heading("The file");
    let old = t.saved_parameters("fx_lens_001.json");
    let clean = ["iris", "roundness", "rotation", "aspect", "highlight_gain", "highlight_threshold"]
        .iter()
        .all(|k| old.get(k).is_none());
    t.row(
        "fx_lens_001.json, from before D-121, is saved without any of the new settings",
        &old.to_string(),
        clean,
    );
    t.round_trips(&[
        "fx_lens_019.json",
        "fx_lens_031.json",
        "fx_lens_032.json",
        "fx_lens_035.json",
        "fx_lens_037.json",
        "fx_lens_038.json",
        "fx_lens_039.json",
        "fx_lens_040.json",
        "fx_lens_041.json",
        "fx_lens_042.json",
        "fx_lens_043.json",
        "fx_lens_044.json",
    ]);
    t.shape_refused(
        "fx_lens_001.json",
        "an iris written as a number",
        r#"{"radius": 10, "edges": "transparent", "iris": 6}"#,
    );
    t.shape_refused(
        "fx_lens_001.json",
        "a rotation written as a word",
        r#"{"radius": 10, "edges": "transparent", "rotation": "up"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_lens_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("iris \"star\"", set(lens(10.0, "transparent", "star", START))),
            ("iris \"Hexagon\"", set(lens(10.0, "transparent", "Hexagon", START))),
            ("roundness 101", set(lens(10.0, "transparent", "square", [101.0, 0.0, 1.0, 0.0, 100.0]))),
            ("rotation 3601", set(lens(10.0, "transparent", "square", [0.0, 3601.0, 1.0, 0.0, 100.0]))),
            ("aspect 0.05", set(lens(10.0, "transparent", "square", [0.0, 0.0, 0.05, 0.0, 100.0]))),
            ("highlight gain -1", set(lens(10.0, "transparent", "square", [0.0, 0.0, 1.0, -1.0, 100.0]))),
            ("highlight threshold 101", set(lens(10.0, "transparent", "square", [0.0, 0.0, 1.0, 0.0, 101.0]))),
            ("aspect keyed to 20", keys("aspect", &[(0, &[1.0]), (4, &[20.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_lens_001.json",
        vec![
            (
                "a hexagon, roundness 20, turned 15 degrees, aspect 1.5, gain 3 over 90",
                set(lens(6.0, "repeat", "hexagon", [20.0, 15.0, 1.5, 3.0, 90.0])),
            ),
            (
                "the far ends: decagon, roundness 100, rotation -3600, aspect 10, gain 100 over 0",
                set(lens(4.0, "transparent", "decagon", [100.0, -3600.0, 10.0, 100.0, 0.0])),
            ),
            ("rotation keyed from 0 to 3600", keys("rotation", &[(0, &[0.0]), (4, &[3600.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_lens_021.json", 0),
        ("fx_lens_028.json", 0),
        ("fx_lens_030.json", 0),
        ("fx_lens_033.json", 0),
        ("fx_lens_034.json", 0),
    ]);

    t.finish("B-64_lens_iris_table.md");
}
