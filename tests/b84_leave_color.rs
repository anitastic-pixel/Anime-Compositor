//! B-84: leave colour in the core, against D-141.
//!
//! Writes `verification/B-84_leave_color_table.md`.
//!
//! Every expected pixel is `Fixtures/leave_color/expected_leave_color.json`, written by
//! `tools/leave_color_reference.py` before this code existed and printed in document 25 as
//! FX-LEAVE-001 to 022. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn leave(color: &str, tolerance: f64, softness: f64, amount: f64) -> Effect {
    Effect::LeaveColor {
        color: color.to_string(),
        tolerance,
        softness,
        amount,
    }
}

#[test]
fn b84_leave_color() {
    let mut t = Table::new(
        "leave_color",
        "# B-84: leave colour\n\nD-141, accepted on 2026-09-26 by the owner's message asking \
         for thirty more effects, the eighth of the third batch. Every expected pixel is \
         `Fixtures/leave_color/expected_leave_color.json`, written by \
         `tools/leave_color_reference.py` before this code existed and printed in document 25 \
         as FX-LEAVE-001 to 022. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-LEAVE-001 to 022 (document 25)");
    t.fixtures("expected_leave_color.json");

    t.heading("How far it reaches");
    let got = leave("#ff0000", 100.0, 100.0, 100.0).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = leave("#ff0000", 15.0, 10.0, 100.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: it has no distances",
        &format!("{draft:?}"),
        draft == leave("#ff0000", 15.0, 10.0, 100.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_leave_001.json",
        "fx_leave_008.json",
        "fx_leave_013.json",
        "fx_leave_015.json",
        "fx_leave_016.json",
        "fx_leave_017.json",
        "fx_leave_018.json",
        "fx_leave_020.json",
        "fx_leave_021.json",
        "fx_leave_022.json",
    ]);
    let saved = t.saved_parameters("fx_leave_010.json")["color"].clone();
    t.row(
        "fx_leave_010.json's colour, written #4060FF, is saved in small letters",
        &saved.to_string(),
        saved == "#4060ff",
    );
    t.shape_refused(
        "fx_leave_001.json",
        "no `color` at all",
        r#"{"tolerance": 15, "softness": 10, "amount": 100}"#,
    );
    t.shape_refused(
        "fx_leave_001.json",
        "a colour that is a number",
        r#"{"color": 255, "tolerance": 15, "softness": 10, "amount": 100}"#,
    );
    t.shape_refused(
        "fx_leave_001.json",
        "an amount that is a word",
        r##"{"color": "#ff0000", "tolerance": 15, "softness": 10, "amount": "all"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_leave_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("tolerance 101", set(leave("#ff0000", 101.0, 10.0, 100.0))),
            ("softness -1", set(leave("#ff0000", 15.0, -1.0, 100.0))),
            ("amount 101", set(leave("#ff0000", 15.0, 10.0, 101.0))),
            ("the colour \"red\"", set(leave("red", 15.0, 10.0, 100.0))),
            ("the colour \"#ff00\"", set(leave("#ff00", 15.0, 10.0, 100.0))),
            ("tolerance keyed to 150", keys("tolerance", &[(0, &[15.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_leave_001.json",
        vec![
            ("every number at 0, the bottom,", set(leave("#4060ff", 0.0, 0.0, 0.0))),
            ("every number at 100, the top,", set(leave("#808080", 100.0, 100.0, 100.0))),
            ("amount keyed from 0 to 100", keys("amount", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_leave_001.json", 0),
        ("fx_leave_007.json", 0),
        ("fx_leave_013.json", 2),
    ]);

    t.finish("B-84_leave_color_table.md");
}
