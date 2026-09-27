//! B-86: halftone in the core, against D-143.
//!
//! Writes `verification/B-86_halftone_table.md`.
//!
//! Every expected pixel is `Fixtures/halftone/expected_halftone.json`, written by
//! `tools/halftone_reference.py` before this code existed and printed in document 25 as
//! FX-HALFTONE-001 to 025. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn halftone(size: f64, angle: f64, ink: &str, paper: &str, amount: f64) -> Effect {
    Effect::Halftone {
        size,
        angle,
        ink: ink.to_string(),
        paper: paper.to_string(),
        amount,
    }
}

fn start() -> Effect {
    halftone(8.0, 45.0, "#000000", "#ffffff", 100.0)
}

#[test]
fn b86_halftone() {
    let mut t = Table::new(
        "halftone",
        "# B-86: halftone\n\nD-143, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the tenth of the third batch. Every expected pixel is \
         `Fixtures/halftone/expected_halftone.json`, written by `tools/halftone_reference.py` \
         before this code existed and printed in document 25 as FX-HALFTONE-001 to 025. The \
         build's frame is compared sample by sample; the answer is the largest difference over \
         all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-HALFTONE-001 to 025 (document 25)");
    t.fixtures("expected_halftone.json");

    t.heading("How far it reaches");
    let got = halftone(200.0, 45.0, "#000000", "#ffffff", 100.0).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = start();
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the size, 8 to 4, and nothing else",
        &format!("{draft:?}"),
        draft == halftone(4.0, 45.0, "#000000", "#ffffff", 100.0),
    );
    let mut draft = halftone(3.0, 45.0, "#000000", "#ffffff", 100.0);
    draft.scale_distances(|d| d * 0.25);
    t.row(
        "a quarter-size draft of size 3 holds the size at its smallest, 2, rather than dropping \
         the effect",
        &format!("{draft:?}"),
        draft == halftone(2.0, 45.0, "#000000", "#ffffff", 100.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_halftone_001.json",
        "fx_halftone_008.json",
        "fx_halftone_012.json",
        "fx_halftone_013.json",
        "fx_halftone_014.json",
        "fx_halftone_016.json",
        "fx_halftone_018.json",
        "fx_halftone_020.json",
        "fx_halftone_023.json",
        "fx_halftone_024.json",
        "fx_halftone_025.json",
    ]);
    let saved = t.saved_parameters("fx_halftone_009.json");
    t.row(
        "fx_halftone_009.json's colours, written #1E1A24 and #F6D6BE, are saved in small letters",
        &format!("{} and {}", saved["ink"], saved["paper"]),
        saved["ink"] == "#1e1a24" && saved["paper"] == "#f6d6be",
    );
    t.shape_refused(
        "fx_halftone_001.json",
        "no `paper` at all",
        r##"{"size": 8, "angle": 45, "ink": "#000000", "amount": 100}"##,
    );
    t.shape_refused(
        "fx_halftone_001.json",
        "a size that is a word",
        r##"{"size": "big", "angle": 45, "ink": "#000000", "paper": "#ffffff", "amount": 100}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_halftone_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("size 1", set(halftone(1.0, 45.0, "#000000", "#ffffff", 100.0))),
            ("size 201", set(halftone(201.0, 45.0, "#000000", "#ffffff", 100.0))),
            ("angle 3601", set(halftone(8.0, 3601.0, "#000000", "#ffffff", 100.0))),
            ("amount 101", set(halftone(8.0, 45.0, "#000000", "#ffffff", 101.0))),
            ("the ink \"black\"", set(halftone(8.0, 45.0, "black", "#ffffff", 100.0))),
            ("the paper \"#fffff\"", set(halftone(8.0, 45.0, "#000000", "#fffff", 100.0))),
            ("size keyed to 250", keys("size", &[(0, &[8.0]), (4, &[250.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_halftone_001.json",
        vec![
            (
                "size 2, angle -3600 and amount 0, the bottoms,",
                set(halftone(2.0, -3600.0, "#1e1a24", "#f6d6be", 0.0)),
            ),
            (
                "size 200, angle 3600 and amount 100, the tops,",
                set(halftone(200.0, 3600.0, "#ffffff", "#000000", 100.0)),
            ),
            ("angle keyed from 0 to 90", keys("angle", &[(0, &[0.0]), (4, &[90.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_halftone_001.json", 0),
        ("fx_halftone_016.json", 0),
        ("fx_halftone_013.json", 2),
    ]);

    t.finish("B-86_halftone_table.md");
}
