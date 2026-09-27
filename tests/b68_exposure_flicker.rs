//! B-68: the exposure flicker in the core, against D-125.
//!
//! Writes `verification/B-68_exposure_flicker_table.md`.
//!
//! Every expected pixel is `Fixtures/exposure_flicker/expected_exposure_flicker.json`, written
//! by `tools/exposure_flicker_reference.py` before this code existed and printed in document 25
//! as FX-FLICKER-001 to 022. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn flicker(amount: f64, hold: f64, seed: f64) -> Effect {
    Effect::ExposureFlicker {
        amount,
        hold,
        seed,
        frame: 0,
    }
}

#[test]
fn b68_exposure_flicker() {
    let mut t = Table::new(
        "exposure_flicker",
        "# B-68: exposure flicker\n\nD-125, accepted by the owner on 2026-09-26, the third of \
         the second batch of ten. Every expected pixel is \
         `Fixtures/exposure_flicker/expected_exposure_flicker.json`, written by \
         `tools/exposure_flicker_reference.py` before this code existed and printed in document \
         25 as FX-FLICKER-001 to 022. The build's frame is compared sample by sample; the answer \
         is the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-FLICKER-001 to 022 (document 25)");
    t.fixtures("expected_exposure_flicker.json");

    t.heading("How far it reaches");
    let got = flicker(4.0, 1.0, 0.0).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = flicker(1.0, 3.0, 7.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: it has no distances",
        &format!("{draft:?}"),
        draft == flicker(1.0, 3.0, 7.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_flicker_001.json",
        "fx_flicker_004.json",
        "fx_flicker_006.json",
        "fx_flicker_009.json",
        "fx_flicker_010.json",
        "fx_flicker_012.json",
        "fx_flicker_013.json",
        "fx_flicker_014.json",
        "fx_flicker_015.json",
        "fx_flicker_016.json",
        "fx_flicker_018.json",
        "fx_flicker_020.json",
        "fx_flicker_022.json",
    ]);
    t.shape_refused(
        "fx_flicker_001.json",
        "no `amount` at all",
        r##"{"hold": 1, "seed": 0}"##,
    );
    t.shape_refused(
        "fx_flicker_001.json",
        "a hold that is a word",
        r##"{"amount": 0.25, "hold": "one", "seed": 0}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_flicker_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("amount 4.5", set(flicker(4.5, 1.0, 0.0))),
            ("amount -0.1", set(flicker(-0.1, 1.0, 0.0))),
            ("hold 0.5", set(flicker(0.25, 0.5, 0.0))),
            ("hold 101", set(flicker(0.25, 101.0, 0.0))),
            ("seed -1", set(flicker(0.25, 1.0, -1.0))),
            ("seed 100001", set(flicker(0.25, 1.0, 100001.0))),
            ("amount keyed to 6", keys("amount", &[(0, &[0.25]), (4, &[6.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_flicker_001.json",
        vec![
            (
                "amount 4, hold 100 and seed 100000, the tops,",
                set(flicker(4.0, 100.0, 100000.0)),
            ),
            ("amount 0, hold 1 and seed 0, the bottoms,", set(flicker(0.0, 1.0, 0.0))),
            ("hold keyed from 1 to 2", keys("hold", &[(0, &[1.0]), (4, &[2.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_flicker_001.json", 1), ("fx_flicker_015.json", 3)]);

    t.finish("B-68_exposure_flicker_table.md");
}
