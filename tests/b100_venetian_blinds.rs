//! B-100: venetian blinds in the core, against D-157.
//!
//! Writes `verification/B-100_venetian_blinds_table.md`.
//!
//! Every expected pixel is `Fixtures/venetian_blinds/expected_venetian_blinds.json`, written by
//! `tools/venetian_blinds_reference.py` before this code existed and printed in document 25 as
//! FX-BLINDS-001 to 026. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn blinds(completion: f64, angle: f64, width: f64, feather: f64) -> Effect {
    Effect::VenetianBlinds {
        completion,
        angle,
        width,
        feather,
    }
}

#[test]
fn b100_venetian_blinds() {
    let mut t = Table::new(
        "venetian_blinds",
        "# B-100: venetian blinds\n\nD-157, accepted on 2026-09-26 by the owner's message asking \
         for thirty more effects, the twenty-fourth of the third batch. Every expected pixel is \
         `Fixtures/venetian_blinds/expected_venetian_blinds.json`, written by \
         `tools/venetian_blinds_reference.py` before this code existed and printed in document \
         25 as FX-BLINDS-001 to 026. The build's frame is compared sample by sample; the answer \
         is the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-BLINDS-001 to 025 (document 25)");
    t.fixtures_numbered("expected_venetian_blinds.json", 1..=25);

    t.heading("FX-BLINDS-026, in dispute (D-164, proposed)");
    t.in_dispute("fx_blinds_026.json", "FX-BLINDS-026", "D-164, proposed");

    t.heading("How far it reaches");
    let got = blinds(50.0, 30.0, 10000.0, 10000.0).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = blinds(50.0, 0.0, 20.0, 6.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the width, 20 to 10, and the feather, 6 to 3",
        &format!("{draft:?}"),
        draft == blinds(50.0, 0.0, 10.0, 3.0),
    );
    let mut draft = blinds(50.0, 0.0, 1.5, 0.0);
    draft.scale_distances(|d| d * 0.25);
    t.row(
        "a quarter-size draft of width 1.5 holds the width at 1, the least the command takes, \
         rather than dropping the effect",
        &format!("{draft:?}"),
        draft == blinds(50.0, 0.0, 1.0, 0.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_blinds_001.json",
        "fx_blinds_012.json",
        "fx_blinds_014.json",
        "fx_blinds_015.json",
        "fx_blinds_016.json",
        "fx_blinds_017.json",
        "fx_blinds_019.json",
        "fx_blinds_020.json",
        "fx_blinds_021.json",
        "fx_blinds_022.json",
        "fx_blinds_023.json",
        "fx_blinds_024.json",
        "fx_blinds_025.json",
    ]);
    t.shape_refused(
        "fx_blinds_001.json",
        "no `width` at all",
        r#"{"completion": 0, "angle": 0, "feather": 0}"#,
    );
    t.shape_refused(
        "fx_blinds_001.json",
        "a feather that is a list",
        r#"{"completion": 0, "angle": 0, "width": 20, "feather": [0]}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_blinds_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("completion -1", set(blinds(-1.0, 0.0, 20.0, 0.0))),
            ("completion 100.5", set(blinds(100.5, 0.0, 20.0, 0.0))),
            ("width 0.5", set(blinds(50.0, 0.0, 0.5, 0.0))),
            ("width 10001", set(blinds(50.0, 0.0, 10001.0, 0.0))),
            ("feather -1", set(blinds(50.0, 0.0, 20.0, -1.0))),
            ("angle 3601", set(blinds(50.0, 3601.0, 20.0, 0.0))),
            ("completion keyed to 150", keys("completion", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_blinds_001.json",
        vec![
            ("every number at its bottom,", set(blinds(0.0, -3600.0, 1.0, 0.0))),
            ("every number at its top,", set(blinds(100.0, 3600.0, 10000.0, 10000.0))),
            ("completion keyed from 0 to 100", keys("completion", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_blinds_008.json", 0),
        ("fx_blinds_012.json", 0),
        ("fx_blinds_018.json", 3),
    ]);

    t.finish("B-100_venetian_blinds_table.md");
}
