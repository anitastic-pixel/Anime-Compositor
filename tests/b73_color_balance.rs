//! B-73: the colour balance in the core, against D-130.
//!
//! Writes `verification/B-73_color_balance_table.md`.
//!
//! Every expected pixel is `Fixtures/color_balance/expected_color_balance.json`, written by
//! `tools/color_balance_reference.py` before this code existed and printed in document 25 as
//! FX-BALANCE-001 to 019. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn balance(shadows: &[f64], midtones: &[f64], highlights: &[f64]) -> Effect {
    Effect::ColorBalance {
        shadows: shadows.to_vec(),
        midtones: midtones.to_vec(),
        highlights: highlights.to_vec(),
    }
}

const ZERO: &[f64] = &[0.0, 0.0, 0.0];

#[test]
fn b73_color_balance() {
    let mut t = Table::new(
        "color_balance",
        "# B-73: colour balance\n\nD-130, accepted by the owner on 2026-09-26, the eighth of the \
         second batch of ten. Every expected pixel is \
         `Fixtures/color_balance/expected_color_balance.json`, written by \
         `tools/color_balance_reference.py` before this code existed and printed in document 25 \
         as FX-BALANCE-001 to 019. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-BALANCE-001 to 019 (document 25)");
    t.fixtures("expected_color_balance.json");

    t.heading("How far it reaches");
    let got = balance(&[100.0, 0.0, 0.0], ZERO, ZERO).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = balance(&[0.0, 0.0, 40.0], &[-10.0, 5.0, 0.0], &[30.0, 10.0, -20.0]);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: it has no distances",
        &format!("{draft:?}"),
        draft == balance(&[0.0, 0.0, 40.0], &[-10.0, 5.0, 0.0], &[30.0, 10.0, -20.0]),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_balance_001.json",
        "fx_balance_002.json",
        "fx_balance_004.json",
        "fx_balance_009.json",
        "fx_balance_011.json",
        "fx_balance_013.json",
        "fx_balance_014.json",
        "fx_balance_015.json",
        "fx_balance_018.json",
        "fx_balance_019.json",
    ]);
    t.shape_refused(
        "fx_balance_001.json",
        "no `midtones` at all",
        r#"{"shadows": [0, 0, 0], "highlights": [0, 0, 0]}"#,
    );
    t.shape_refused(
        "fx_balance_001.json",
        "a shadow red that is a word",
        r#"{"shadows": ["red", 0, 0], "midtones": [0, 0, 0], "highlights": [0, 0, 0]}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_balance_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("shadows 101, 0, 0", set(balance(&[101.0, 0.0, 0.0], ZERO, ZERO))),
            ("midtones 0, -101, 0", set(balance(ZERO, &[0.0, -101.0, 0.0], ZERO))),
            ("highlights 0, 0, 150", set(balance(ZERO, ZERO, &[0.0, 0.0, 150.0]))),
            ("shadows of two numbers, 100, 0", set(balance(&[100.0, 0.0], ZERO, ZERO))),
            ("highlights of four numbers", set(balance(ZERO, ZERO, &[0.0, 0.0, 0.0, 0.0]))),
            (
                "highlights keyed to 0, 0, 150",
                keys("highlights", &[(0, &[0.0, 0.0, 0.0]), (4, &[0.0, 0.0, 150.0])]),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_balance_001.json",
        vec![
            (
                "all nine at the tops, 100,",
                set(balance(&[100.0; 3], &[100.0; 3], &[100.0; 3])),
            ),
            (
                "all nine at the bottoms, -100,",
                set(balance(&[-100.0; 3], &[-100.0; 3], &[-100.0; 3])),
            ),
            (
                "shadows keyed from 0, 0, 0 to 100, 0, 0",
                keys("shadows", &[(0, &[0.0, 0.0, 0.0]), (4, &[100.0, 0.0, 0.0])]),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_balance_009.json", 0), ("fx_balance_012.json", 4)]);

    t.finish("B-73_color_balance_table.md");
}
