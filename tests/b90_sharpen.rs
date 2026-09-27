//! B-90: sharpen in the core, against D-147.
//!
//! Writes `verification/B-90_sharpen_table.md`.
//!
//! Every expected pixel is `Fixtures/sharpen/expected_sharpen.json`, written by
//! `tools/sharpen_reference.py` before this code existed and printed in document 25 as
//! FX-SHARPEN-001 to 018. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn sharpen(amount: f64, radius: f64) -> Effect {
    Effect::Sharpen { amount, radius }
}

#[test]
fn b90_sharpen() {
    let mut t = Table::new(
        "sharpen",
        "# B-90: sharpen\n\nD-147, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the fourteenth of the third batch. Every expected pixel is \
         `Fixtures/sharpen/expected_sharpen.json`, written by `tools/sharpen_reference.py` \
         before this code existed and printed in document 25 as FX-SHARPEN-001 to 018. The \
         build's frame is compared sample by sample; the answer is the largest difference over \
         all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-SHARPEN-001 to 018 (document 25)");
    t.fixtures("expected_sharpen.json");

    t.heading("How far it reaches");
    let got = sharpen(500.0, 100.0).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = sharpen(100.0, 3.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the radius, 3 to 1.5, and nothing else",
        &format!("{draft:?}"),
        draft == sharpen(100.0, 1.5),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_sharpen_001.json",
        "fx_sharpen_006.json",
        "fx_sharpen_008.json",
        "fx_sharpen_010.json",
        "fx_sharpen_013.json",
        "fx_sharpen_014.json",
        "fx_sharpen_015.json",
        "fx_sharpen_016.json",
        "fx_sharpen_017.json",
        "fx_sharpen_018.json",
    ]);
    t.shape_refused("fx_sharpen_001.json", "no `radius` at all", r#"{"amount": 100}"#);
    t.shape_refused(
        "fx_sharpen_001.json",
        "an amount that is a word",
        r#"{"amount": "lots", "radius": 1}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_sharpen_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("amount 501", set(sharpen(501.0, 1.0))),
            ("amount -1", set(sharpen(-1.0, 1.0))),
            ("radius 101", set(sharpen(100.0, 101.0))),
            ("radius -0.5", set(sharpen(100.0, -0.5))),
            ("amount keyed to 600", keys("amount", &[(0, &[100.0]), (4, &[600.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_sharpen_001.json",
        vec![
            ("amount 0 and radius 0, the bottoms,", set(sharpen(0.0, 0.0))),
            ("amount 500 and radius 100, the tops,", set(sharpen(500.0, 100.0))),
            ("radius keyed from 0 to 2", keys("radius", &[(0, &[0.0]), (4, &[2.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_sharpen_001.json", 0),
        ("fx_sharpen_007.json", 0),
        ("fx_sharpen_011.json", 3),
    ]);

    t.finish("B-90_sharpen_table.md");
}
