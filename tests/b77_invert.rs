//! B-77: the invert in the core, against D-134.
//!
//! Writes `verification/B-77_invert_table.md`.
//!
//! Every expected pixel is `Fixtures/invert/expected_invert.json`, written by
//! `tools/invert_reference.py` before this code existed and printed in document 25 as
//! FX-INVERT-001 to 020. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn invert(channel: &str, amount: f64) -> Effect {
    Effect::Invert {
        channel: channel.to_string(),
        amount,
    }
}

#[test]
fn b77_invert() {
    let mut t = Table::new(
        "invert",
        "# B-77: invert\n\nD-134, accepted on 2026-09-26 by the owner's message asking for thirty \
         more effects, the first of the third batch. Every expected pixel is \
         `Fixtures/invert/expected_invert.json`, written by `tools/invert_reference.py` before \
         this code existed and printed in document 25 as FX-INVERT-001 to 020. The build's frame \
         is compared sample by sample; the answer is the largest difference over all of them, \
         against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-INVERT-001 to 020 (document 25)");
    t.fixtures("expected_invert.json");

    t.heading("How far it reaches");
    let got = invert("alpha", 100.0).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = invert("rgb", 60.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: it has no distances",
        &format!("{draft:?}"),
        draft == invert("rgb", 60.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_invert_001.json",
        "fx_invert_005.json",
        "fx_invert_008.json",
        "fx_invert_010.json",
        "fx_invert_012.json",
        "fx_invert_015.json",
        "fx_invert_016.json",
        "fx_invert_018.json",
        "fx_invert_019.json",
        "fx_invert_020.json",
    ]);
    t.shape_refused("fx_invert_001.json", "no `channel` at all", r#"{"amount": 100}"#);
    t.shape_refused(
        "fx_invert_001.json",
        "an amount that is a word",
        r#"{"channel": "rgb", "amount": "all"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_invert_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("amount 101", set(invert("rgb", 101.0))),
            ("amount -1", set(invert("rgb", -1.0))),
            ("channel \"luma\"", set(invert("luma", 100.0))),
            ("channel \"Red\", in a capital", set(invert("Red", 100.0))),
            ("amount keyed to 150", keys("amount", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_invert_001.json",
        vec![
            ("channel alpha at amount 0, the bottom,", set(invert("alpha", 0.0))),
            ("channel blue at amount 100, the top,", set(invert("blue", 100.0))),
            ("amount keyed from 0 to 100", keys("amount", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_invert_001.json", 0), ("fx_invert_009.json", 0), ("fx_invert_010.json", 2)]);

    t.finish("B-77_invert_table.md");
}
