//! B-81: threshold in the core, against D-138.
//!
//! Writes `verification/B-81_threshold_table.md`.
//!
//! Every expected pixel is `Fixtures/threshold/expected_threshold.json`, written by
//! `tools/threshold_reference.py` before this code existed and printed in document 25 as
//! FX-THRESH-001 to 019. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn threshold(level: f64) -> Effect {
    Effect::Threshold { level }
}

#[test]
fn b81_threshold() {
    let mut t = Table::new(
        "threshold",
        "# B-81: threshold\n\nD-138, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the fifth of the third batch. Every expected pixel is \
         `Fixtures/threshold/expected_threshold.json`, written by `tools/threshold_reference.py` \
         before this code existed and printed in document 25 as FX-THRESH-001 to 019. The \
         build's frame is compared sample by sample; the answer is the largest difference over \
         all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-THRESH-001 to 018 (document 25)");
    t.fixtures_numbered("expected_threshold.json", 1..=18);

    t.heading("FX-THRESH-019, in dispute (D-164, proposed)");
    t.in_dispute("fx_thresh_019.json", "FX-THRESH-019", "D-164, proposed");

    t.heading("How far it reaches");
    let got = threshold(128.0).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = threshold(128.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: it has no distances",
        &format!("{draft:?}"),
        draft == threshold(128.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_thresh_001.json",
        "fx_thresh_002.json",
        "fx_thresh_008.json",
        "fx_thresh_011.json",
        "fx_thresh_012.json",
        "fx_thresh_013.json",
        "fx_thresh_014.json",
        "fx_thresh_015.json",
        "fx_thresh_017.json",
        "fx_thresh_018.json",
    ]);
    t.shape_refused("fx_thresh_001.json", "no `level` at all", r#"{}"#);

    t.heading("Commands");
    let mut document = t.load("fx_thresh_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("level -1", set(threshold(-1.0))),
            ("level 256", set(threshold(256.0))),
            ("level 255.5", set(threshold(255.5))),
            ("level keyed to 300", keys("level", &[(0, &[128.0]), (4, &[300.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_thresh_001.json",
        vec![
            ("level 0, the bottom,", set(threshold(0.0))),
            ("level 255, the top,", set(threshold(255.0))),
            ("level keyed from 0 to 255", keys("level", &[(0, &[0.0]), (4, &[255.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_thresh_001.json", 0), ("fx_thresh_009.json", 0), ("fx_thresh_011.json", 2)]);

    t.finish("B-81_threshold_table.md");
}
