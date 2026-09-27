//! B-98: linear wipe in the core, against D-155.
//!
//! Writes `verification/B-98_linear_wipe_table.md`.
//!
//! Every expected pixel is `Fixtures/linear_wipe/expected_linear_wipe.json`, written by
//! `tools/linear_wipe_reference.py` before this code existed and printed in document 25 as
//! FX-LWIPE-001 to 032. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn linear_wipe(completion: f64, angle: f64, feather: f64) -> Effect {
    Effect::LinearWipe {
        completion,
        angle,
        feather,
    }
}

#[test]
fn b98_linear_wipe() {
    let mut t = Table::new(
        "linear_wipe",
        "# B-98: linear wipe\n\nD-155, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the twenty-second of the third batch. Every expected pixel is \
         `Fixtures/linear_wipe/expected_linear_wipe.json`, written by \
         `tools/linear_wipe_reference.py` before this code existed and printed in document 25 \
         as FX-LWIPE-001 to 032. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-LWIPE-001 to 031 (document 25)");
    t.fixtures_numbered("expected_linear_wipe.json", 1..=31);

    t.heading("FX-LWIPE-032, in dispute (D-164, proposed)");
    t.in_dispute("fx_lwipe_032.json", "FX-LWIPE-032", "D-164, proposed");

    t.heading("How far it reaches");
    let got = linear_wipe(50.0, 45.0, 10000.0).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = linear_wipe(50.0, 90.0, 8.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the feather, 8 to 4, and nothing else",
        &format!("{draft:?}"),
        draft == linear_wipe(50.0, 90.0, 4.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_lwipe_001.json",
        "fx_lwipe_010.json",
        "fx_lwipe_015.json",
        "fx_lwipe_016.json",
        "fx_lwipe_017.json",
        "fx_lwipe_021.json",
        "fx_lwipe_025.json",
        "fx_lwipe_026.json",
        "fx_lwipe_027.json",
        "fx_lwipe_028.json",
        "fx_lwipe_029.json",
        "fx_lwipe_030.json",
        "fx_lwipe_031.json",
    ]);
    t.shape_refused(
        "fx_lwipe_001.json",
        "no `feather` at all",
        r#"{"completion": 0, "angle": 90}"#,
    );
    t.shape_refused(
        "fx_lwipe_001.json",
        "an angle that is a word",
        r#"{"completion": 0, "angle": "left", "feather": 0}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_lwipe_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("completion -1", set(linear_wipe(-1.0, 90.0, 0.0))),
            ("completion 100.5", set(linear_wipe(100.5, 90.0, 0.0))),
            ("angle 3601", set(linear_wipe(25.0, 3601.0, 0.0))),
            ("angle -3601", set(linear_wipe(25.0, -3601.0, 0.0))),
            ("feather -1", set(linear_wipe(50.0, 90.0, -1.0))),
            ("feather 10001", set(linear_wipe(50.0, 90.0, 10001.0))),
            ("completion keyed to 150", keys("completion", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_lwipe_001.json",
        vec![
            ("every number at its bottom,", set(linear_wipe(0.0, -3600.0, 0.0))),
            ("every number at its top,", set(linear_wipe(100.0, 3600.0, 10000.0))),
            ("completion keyed from 0 to 100", keys("completion", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_lwipe_005.json", 0),
        ("fx_lwipe_010.json", 0),
        ("fx_lwipe_020.json", 3),
        ("fx_lwipe_022.json", 0),
    ]);

    t.finish("B-98_linear_wipe_table.md");
}
