//! B-101: iris wipe in the core, against D-158.
//!
//! Writes `verification/B-101_iris_wipe_table.md`.
//!
//! Every expected pixel is `Fixtures/iris_wipe/expected_iris_wipe.json`, written by
//! `tools/iris_wipe_reference.py` before this code existed and printed in document 25 as
//! FX-IRIS-001 to 027. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn iris(completion: f64, center: [f64; 2], feather: f64, invert: &str) -> Effect {
    Effect::IrisWipe {
        completion,
        center,
        feather,
        invert: invert.to_string(),
    }
}

const MIDDLE: [f64; 2] = [50.0, 50.0];

#[test]
fn b101_iris_wipe() {
    let mut t = Table::new(
        "iris_wipe",
        "# B-101: iris wipe\n\nD-158, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the twenty-fifth of the third batch. Every expected pixel is \
         `Fixtures/iris_wipe/expected_iris_wipe.json`, written by `tools/iris_wipe_reference.py` \
         before this code existed and printed in document 25 as FX-IRIS-001 to 027. The build's \
         frame is compared sample by sample; the answer is the largest difference over all of \
         them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-IRIS-001 to 027 (document 25)");
    t.fixtures("expected_iris_wipe.json");

    t.heading("How far it reaches");
    let got = iris(50.0, MIDDLE, 10000.0, "on").bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = iris(50.0, MIDDLE, 8.0, "off");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the feather, 8 to 4, and nothing else",
        &format!("{draft:?}"),
        draft == iris(50.0, MIDDLE, 4.0, "off"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_iris_001.json",
        "fx_iris_009.json",
        "fx_iris_015.json",
        "fx_iris_016.json",
        "fx_iris_018.json",
        "fx_iris_020.json",
        "fx_iris_021.json",
        "fx_iris_022.json",
        "fx_iris_023.json",
        "fx_iris_024.json",
        "fx_iris_025.json",
        "fx_iris_026.json",
        "fx_iris_027.json",
    ]);
    t.shape_refused(
        "fx_iris_001.json",
        "no `invert` at all",
        r#"{"completion": 0, "center": [50, 50], "feather": 0}"#,
    );
    t.shape_refused(
        "fx_iris_001.json",
        "a centre that is a word",
        r#"{"completion": 0, "center": "middle", "feather": 0, "invert": "off"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_iris_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("completion -1", set(iris(-1.0, MIDDLE, 0.0, "off"))),
            ("completion 101", set(iris(101.0, MIDDLE, 0.0, "off"))),
            ("feather -1", set(iris(50.0, MIDDLE, -1.0, "off"))),
            ("feather 10001", set(iris(50.0, MIDDLE, 10001.0, "off"))),
            ("centre 1001, 50", set(iris(50.0, [1001.0, 50.0], 0.0, "off"))),
            ("invert \"yes\"", set(iris(50.0, MIDDLE, 0.0, "yes"))),
            ("invert \"On\"", set(iris(50.0, MIDDLE, 0.0, "On"))),
            ("completion keyed to 150", keys("completion", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_iris_001.json",
        vec![
            ("every number at its bottom,", set(iris(0.0, [-1000.0, -1000.0], 0.0, "on"))),
            ("every number at its top,", set(iris(100.0, [1000.0, 1000.0], 10000.0, "off"))),
            ("completion keyed from 0 to 100", keys("completion", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_iris_008.json", 0),
        ("fx_iris_013.json", 0),
        ("fx_iris_019.json", 0),
    ]);

    t.finish("B-101_iris_wipe_table.md");
}
