//! B-88: emboss in the core, against D-145.
//!
//! Writes `verification/B-88_emboss_table.md`.
//!
//! Every expected pixel is `Fixtures/emboss/expected_emboss.json`, written by
//! `tools/emboss_reference.py` before this code existed and printed in document 25 as
//! FX-EMBOSS-001 to 026. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn emboss(direction: f64, relief: f64, contrast: f64, mode: &str) -> Effect {
    Effect::Emboss {
        direction,
        relief,
        contrast,
        mode: mode.to_string(),
    }
}

#[test]
fn b88_emboss() {
    let mut t = Table::new(
        "emboss",
        "# B-88: emboss\n\nD-145, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the twelfth of the third batch. Every expected pixel is \
         `Fixtures/emboss/expected_emboss.json`, written by `tools/emboss_reference.py` before \
         this code existed and printed in document 25 as FX-EMBOSS-001 to 026. The build's \
         frame is compared sample by sample; the answer is the largest difference over all of \
         them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-EMBOSS-001 to 026 (document 25)");
    t.fixtures("expected_emboss.json");

    t.heading("How far it reaches");
    let got = emboss(135.0, 100.0, 1000.0, "grey").bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = emboss(135.0, 3.0, 100.0, "grey");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the relief, 3 to 1.5, and nothing else",
        &format!("{draft:?}"),
        draft == emboss(135.0, 1.5, 100.0, "grey"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_emboss_001.json",
        "fx_emboss_005.json",
        "fx_emboss_011.json",
        "fx_emboss_013.json",
        "fx_emboss_015.json",
        "fx_emboss_017.json",
        "fx_emboss_019.json",
        "fx_emboss_023.json",
        "fx_emboss_024.json",
        "fx_emboss_025.json",
        "fx_emboss_026.json",
    ]);
    t.shape_refused(
        "fx_emboss_001.json",
        "no `mode` at all",
        r#"{"direction": 135, "relief": 1, "contrast": 100}"#,
    );
    t.shape_refused(
        "fx_emboss_001.json",
        "a relief that is a word",
        r#"{"direction": 135, "relief": "deep", "contrast": 100, "mode": "grey"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_emboss_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("relief 101", set(emboss(135.0, 101.0, 100.0, "grey"))),
            ("relief -1", set(emboss(135.0, -1.0, 100.0, "grey"))),
            ("contrast 1001", set(emboss(135.0, 1.0, 1001.0, "grey"))),
            ("direction 3601", set(emboss(3601.0, 1.0, 100.0, "grey"))),
            ("mode \"gray\"", set(emboss(135.0, 1.0, 100.0, "gray"))),
            ("relief keyed to 150", keys("relief", &[(0, &[1.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_emboss_001.json",
        vec![
            (
                "direction -3600, relief 0 and contrast 0, the bottoms,",
                set(emboss(-3600.0, 0.0, 0.0, "color")),
            ),
            (
                "direction 3600, relief 100 and contrast 1000, the tops,",
                set(emboss(3600.0, 100.0, 1000.0, "grey")),
            ),
            ("direction keyed from 0 to 180", keys("direction", &[(0, &[0.0]), (4, &[180.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_emboss_001.json", 0),
        ("fx_emboss_005.json", 0),
        ("fx_emboss_011.json", 0),
    ]);

    t.finish("B-88_emboss_table.md");
}
