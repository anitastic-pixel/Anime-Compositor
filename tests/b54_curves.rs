//! B-54: curves in the core, against D-111.
//!
//! Writes `verification/B-54_curves_table.md`.
//!
//! Every expected pixel is `Fixtures/curves/expected_curves.json`, written by
//! `tools/curves_reference.py` before this code existed and printed in document 25 as
//! FX-CURVES-001 to 020. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn line(points: &[[f64; 2]]) -> Vec<Vec<f64>> {
    points.iter().map(|p| p.to_vec()).collect()
}

const STRAIGHT: &[[f64; 2]] = &[[0.0, 0.0], [255.0, 255.0]];

fn master(points: Vec<Vec<f64>>) -> Effect {
    Effect::Curves {
        master: points,
        red: line(STRAIGHT),
        green: line(STRAIGHT),
        blue: line(STRAIGHT),
    }
}

/// `n` points rising evenly from 0 to 255.
fn rising(n: usize) -> Vec<Vec<f64>> {
    (0..n)
        .map(|i| {
            let v = 255.0 * i as f64 / (n - 1) as f64;
            vec![v, v]
        })
        .collect()
}

#[test]
fn b54_curves() {
    let mut t = Table::new(
        "curves",
        "# B-54: curves\n\nD-111, accepted by the owner on 2026-09-26 in the batch of ten. Every \
         expected pixel is `Fixtures/curves/expected_curves.json`, written by \
         `tools/curves_reference.py` before this code existed and printed in document 25 as \
         FX-CURVES-001 to 020. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-CURVES-001 to 020 (document 25)");
    t.fixtures("expected_curves.json");

    t.heading("How far it reaches");
    let lift = master(line(&[[0.0, 0.0], [128.0, 180.0], [255.0, 255.0]]));
    let got = lift.bounds_expansion();
    t.row(
        "it grows the drawing's bounds by nothing: each pixel is regraded where it is",
        &got.to_string(),
        got == 0,
    );
    let mut draft = lift.clone();
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: a curve is colours, not distances",
        &format!("{draft:?}"),
        draft == lift,
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_curves_003.json",
        "fx_curves_006.json",
        "fx_curves_011.json",
        "fx_curves_013.json",
        "fx_curves_015.json",
        "fx_curves_016.json",
        "fx_curves_017.json",
        "fx_curves_018.json",
        "fx_curves_019.json",
        "fx_curves_020.json",
    ]);
    let params = t.saved_parameters("fx_curves_013.json");
    t.row(
        "fx_curves_013.json's points written with fractions are saved with them",
        &params["master"].to_string(),
        params["master"] == serde_json::json!([[0, 0], [127.5, 200.25], [255, 255]]),
    );
    t.shape_refused(
        "fx_curves_001.json",
        "no `blue` curve at all",
        r#"{"master": [[0, 0], [255, 255]], "red": [[0, 0], [255, 255]], "green": [[0, 0], [255, 255]]}"#,
    );
    t.shape_refused(
        "fx_curves_001.json",
        "a point's out written as a word",
        r#"{"master": [[0, 0], [255, "white"]], "red": [[0, 0], [255, 255]], "green": [[0, 0], [255, 255]], "blue": [[0, 0], [255, 255]]}"#,
    );
    t.shape_refused(
        "fx_curves_001.json",
        "a curve written as one number",
        r#"{"master": 255, "red": [[0, 0], [255, 255]], "green": [[0, 0], [255, 255]], "blue": [[0, 0], [255, 255]]}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_curves_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("a master of one point", set(master(line(&[[128.0, 128.0]])))),
            ("a master of 17 points", set(master(rising(17)))),
            (
                "an out of 256",
                set(master(line(&[[0.0, 0.0], [255.0, 256.0]]))),
            ),
            (
                "an in of -1",
                set(master(line(&[[-1.0, 0.0], [255.0, 255.0]]))),
            ),
            (
                "two points at in 128",
                set(master(line(&[[0.0, 0.0], [128.0, 100.0], [128.0, 200.0], [255.0, 255.0]]))),
            ),
            (
                "a point of three numbers",
                set(master(vec![vec![0.0, 0.0], vec![128.0, 128.0, 128.0], vec![255.0, 255.0]])),
            ),
            (
                "the master curve keyed, which a list of points cannot be",
                keys("master", &[(0, &[0.0, 0.0]), (4, &[255.0, 255.0])]),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_curves_001.json",
        vec![
            ("a master of 16 points, the most", set(master(rising(16)))),
            (
                "a master of 2 points at the corners of the range, inverting,",
                set(master(line(&[[0.0, 255.0], [255.0, 0.0]]))),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_curves_011.json", 0), ("fx_curves_014.json", 0)]);

    t.finish("B-54_curves_table.md");
}
