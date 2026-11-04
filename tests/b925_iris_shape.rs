//! B-925: D-1045, Iris Wipe's Iris Points, Outer Radius, Use Inner Radius, Inner Radius and
//! Rotation, after After Effects' Iris Wipe in rsGGCshMDr8 at 9:12; Adobe gives no formula, so
//! the rule is ours.
//!
//! Writes `verification/B-925_iris_shape_table.md`.
//!
//! Every expected pixel is `Fixtures/iris_wipe/expected_iris_shape.json`, written by
//! `tools/iris_shape_reference.py` before this code existed and printed in document 25 as
//! FX-IRIS-028 to 045. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table, MAIN};

use anime_compositor::effects::Effect;
use anime_compositor::model::Id;

fn star(points: f64, outer_radius: f64, use_inner: &str, inner_radius: f64, rotation: f64) -> Effect {
    Effect::IrisWipe {
        completion: 0.0,
        center: [50.0, 50.0],
        feather: 0.0,
        invert: "off".to_string(),
        points,
        outer_radius,
        use_inner: use_inner.to_string(),
        inner_radius,
        rotation,
    }
}

/// The Iris Wipe in `file`, as written when it is refused.
fn iris(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    let layer = d.project().composition(&Id::new(MAIN)).unwrap().layers_in_order().next().unwrap();
    let fx = &layer.effects[0];
    fx.invalid().unwrap_or_else(|| fx.effect.clone())
}

#[test]
fn b925_iris_shape() {
    let mut t = Table::new(
        "iris_wipe",
        "# B-925: Iris Wipe's shape\n\nD-1045, accepted on 2026-10-27 under the owner's request \
         for the tutorial follow-alongs. Every expected pixel is \
         `Fixtures/iris_wipe/expected_iris_shape.json`, written by `tools/iris_shape_reference.py` \
         before this code existed and printed in document 25 as FX-IRIS-028 to 045. The build's \
         frame is compared sample by sample; the answer is the largest difference over all of \
         them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-IRIS-028 to 045 (document 25)");
    t.fixtures("expected_iris_shape.json");

    t.heading("A wrong word is refused (what it warns)");
    let why = iris(&t, "fx_iris_044.json").why_invalid();
    t.row("fx_iris_044.json: the warning names use inner radius", &why, why.contains("use inner radius"));

    t.heading("A half-size draft");
    let mut draft = star(8.0, 400.0, "on", 100.0, 30.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "halves the outer and inner radius, and leaves the points and the rotation",
        &format!("{draft:?}"),
        draft == star(8.0, 200.0, "on", 50.0, 30.0),
    );

    t.heading("The file");
    t.round_trips(&["fx_iris_028.json", "fx_iris_031.json", "fx_iris_036.json", "fx_iris_044.json"]);
    let saved = t.saved_parameters("fx_iris_001.json");
    t.row(
        "fx_iris_001.json, from before them, is saved without the five settings",
        &saved.to_string(),
        ["points", "outer_radius", "use_inner", "inner_radius", "rotation"]
            .iter()
            .all(|k| saved.get(k).is_none()),
    );

    t.heading("Commands");
    let mut document = t.load("fx_iris_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("points 33", set(star(33.0, 0.0, "off", 0.0, 0.0))),
            ("outer radius -1", set(star(8.0, -1.0, "off", 0.0, 0.0))),
            ("inner radius 100001", set(star(8.0, 4.0, "on", 100001.0, 0.0))),
            ("use inner radius \"yes\"", set(star(8.0, 4.0, "yes", 0.0, 0.0))),
            ("rotation keyed to 100001", keys("rotation", &[(0, &[0.0]), (4, &[100001.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_iris_001.json",
        vec![
            ("every new number at its bottom,", set(star(0.0, 0.0, "off", 0.0, -100000.0))),
            ("every new number at its top,", set(star(32.0, 100000.0, "on", 100000.0, 100000.0))),
            ("outer radius keyed from 0 to 1800", keys("outer_radius", &[(0, &[0.0]), (4, &[1800.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_iris_028.json", 0), ("fx_iris_031.json", 0), ("fx_iris_033.json", 0)]);

    t.finish("B-925_iris_shape_table.md");
}
