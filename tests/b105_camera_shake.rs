//! B-105: camera shake in the core, against D-162.
//!
//! Writes `verification/B-105_camera_shake_table.md`.
//!
//! Every expected pixel is `Fixtures/camera_shake/expected_camera_shake.json`, written by
//! `tools/camera_shake_reference.py` before this code existed and printed in document 25 as
//! FX-SHAKE-001 to 021. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

/// `[amount, rotation, hold, seed]`.
fn shake(n: [f64; 4]) -> Effect {
    Effect::CameraShake {
        amount: n[0],
        rotation: n[1],
        hold: n[2],
        seed: n[3],
        frame: 0,
    }
}

const START: [f64; 4] = [10.0, 0.0, 1.0, 0.0];

fn with(i: usize, v: f64) -> Effect {
    let mut n = START;
    n[i] = v;
    shake(n)
}

#[test]
fn b105_camera_shake() {
    let mut t = Table::new(
        "camera_shake",
        "# B-105: camera shake\n\nD-162, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the twenty-ninth of the third batch. Every expected pixel is \
         `Fixtures/camera_shake/expected_camera_shake.json`, written by \
         `tools/camera_shake_reference.py` before this code existed and printed in document 25 as \
         FX-SHAKE-001 to 021. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-SHAKE-001 to 021 (document 25)");
    t.fixtures("expected_camera_shake.json");

    t.heading("How far it reaches");
    let got = with(1, 10.0).bounds_expansion();
    t.row(
        "it declares no fixed growth to the card, which never runs it: its turn's reach depends \
         on the size the drawing reaches it at, and is counted as the stack runs, as FX-SHAKE-013 \
         and 014 show",
        &got.to_string(),
        got == 0,
    );
    let mut draft = with(0, 10.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the amount, 10 to 5, and nothing else",
        &format!("{draft:?}"),
        draft == with(0, 5.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_shake_001.json",
        "fx_shake_006.json",
        "fx_shake_008.json",
        "fx_shake_010.json",
        "fx_shake_011.json",
        "fx_shake_012.json",
        "fx_shake_015.json",
        "fx_shake_016.json",
        "fx_shake_017.json",
        "fx_shake_018.json",
        "fx_shake_019.json",
        "fx_shake_020.json",
        "fx_shake_021.json",
    ]);
    t.shape_refused(
        "fx_shake_001.json",
        "no `hold` at all",
        r#"{"amount": 10, "rotation": 0, "seed": 0}"#,
    );
    t.shape_refused(
        "fx_shake_001.json",
        "an amount that is a word",
        r#"{"amount": "lots", "rotation": 0, "hold": 1, "seed": 0}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_shake_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("amount 1001", set(with(0, 1001.0))),
            ("amount -1", set(with(0, -1.0))),
            ("rotation 46", set(with(1, 46.0))),
            ("hold 0", set(with(2, 0.0))),
            ("hold 101", set(with(2, 101.0))),
            ("seed 100001", set(with(3, 100001.0))),
            ("amount keyed to 1500", keys("amount", &[(0, &[10.0]), (4, &[1500.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_shake_001.json",
        vec![
            ("every number at its bottom,", set(shake([0.0, 0.0, 1.0, 0.0]))),
            ("every number at its top,", set(shake([1000.0, 45.0, 100.0, 100000.0]))),
            ("amount keyed from 0 to 4", keys("amount", &[(0, &[0.0]), (4, &[4.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_shake_003.json", 1),
        ("fx_shake_010.json", 2),
        ("fx_shake_014.json", 0),
    ]);

    t.finish("B-105_camera_shake_table.md");
}
