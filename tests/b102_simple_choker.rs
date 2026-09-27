//! B-102: simple choker in the core, against D-159.
//!
//! Writes `verification/B-102_simple_choker_table.md`.
//!
//! Every expected pixel is `Fixtures/simple_choker/expected_simple_choker.json`, written by
//! `tools/simple_choker_reference.py` before this code existed and printed in document 25 as
//! FX-CHOKE-001 to 017. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn choker(choke: f64) -> Effect {
    Effect::SimpleChoker { choke }
}

#[test]
fn b102_simple_choker() {
    let mut t = Table::new(
        "simple_choker",
        "# B-102: simple choker\n\nD-159, accepted on 2026-09-26 by the owner's message asking \
         for thirty more effects, the twenty-sixth of the third batch. Every expected pixel is \
         `Fixtures/simple_choker/expected_simple_choker.json`, written by \
         `tools/simple_choker_reference.py` before this code existed and printed in document 25 \
         as FX-CHOKE-001 to 017. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-CHOKE-001 to 017 (document 25)");
    t.fixtures("expected_simple_choker.json");

    t.heading("How far it reaches");
    let got = choker(-2.5).bounds_expansion();
    t.row("a spread of 2.5 grows the drawing's bounds by 2 pixels", &got.to_string(), got == 2);
    let got = choker(-100.0).bounds_expansion();
    t.row("a spread of 100 grows them by 100", &got.to_string(), got == 100);
    let got = choker(100.0).bounds_expansion();
    t.row("a shrink grows them by nothing", &got.to_string(), got == 0);
    let got = choker(0.0).bounds_expansion();
    t.row("choke 0 grows them by nothing", &got.to_string(), got == 0);
    let mut draft = choker(-4.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the choke, -4 to -2",
        &format!("{draft:?}"),
        draft == choker(-2.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_choke_001.json",
        "fx_choke_003.json",
        "fx_choke_009.json",
        "fx_choke_010.json",
        "fx_choke_013.json",
        "fx_choke_014.json",
        "fx_choke_015.json",
        "fx_choke_016.json",
        "fx_choke_017.json",
    ]);
    t.shape_refused("fx_choke_001.json", "no `choke` at all", r#"{}"#);
    t.shape_refused("fx_choke_001.json", "a choke that is a list", r#"{"choke": [1]}"#);

    t.heading("Commands");
    let mut document = t.load("fx_choke_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("choke 101", set(choker(101.0))),
            ("choke -101", set(choker(-101.0))),
            ("choke keyed to 150", keys("choke", &[(0, &[1.0]), (4, &[150.0])])),
            ("choke keyed from -120", keys("choke", &[(0, &[-120.0]), (4, &[-1.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_choke_001.json",
        vec![
            ("choke at its bottom, -100,", set(choker(-100.0))),
            ("choke at its top, 100,", set(choker(100.0))),
            ("choke keyed from 0 to -100", keys("choke", &[(0, &[0.0]), (4, &[-100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_choke_003.json", 0),
        ("fx_choke_008.json", 0),
        ("fx_choke_010.json", 4),
    ]);

    t.finish("B-102_simple_choker_table.md");
}
