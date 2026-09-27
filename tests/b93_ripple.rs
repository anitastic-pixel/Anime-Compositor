//! B-93: ripple in the core, against D-150.
//!
//! Writes `verification/B-93_ripple_table.md`.
//!
//! Every expected pixel is `Fixtures/ripple/expected_ripple.json`, written by
//! `tools/ripple_reference.py` before this code existed and printed in document 25 as
//! FX-RIPPLE-001 to 026. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

/// The centre, then amplitude, wavelength, speed, phase and fade.
fn ripple(center: [f64; 2], n: [f64; 5]) -> Effect {
    Effect::Ripple {
        center,
        amplitude: n[0],
        wavelength: n[1],
        speed: n[2],
        phase: n[3],
        fade: n[4],
        frame: 0,
    }
}

const MIDDLE: [f64; 2] = [50.0, 50.0];
const START: [f64; 5] = [5.0, 30.0, 20.0, 0.0, 0.0];

/// The settings as they start with one number changed.
fn with(i: usize, v: f64) -> Effect {
    let mut n = START;
    n[i] = v;
    ripple(MIDDLE, n)
}

#[test]
fn b93_ripple() {
    let mut t = Table::new(
        "ripple",
        "# B-93: ripple\n\nD-150, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the seventeenth of the third batch. Every expected pixel is \
         `Fixtures/ripple/expected_ripple.json`, written by `tools/ripple_reference.py` before \
         this code existed and printed in document 25 as FX-RIPPLE-001 to 026. The build's \
         frame is compared sample by sample; the answer is the largest difference over all of \
         them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-RIPPLE-001 to 026 (document 25)");
    t.fixtures("expected_ripple.json");

    t.heading("How far it reaches");
    let got = with(0, 1000.0).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = ripple(MIDDLE, [5.0, 30.0, 20.0, 40.0, 60.0]);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the amplitude, the wavelength and the fade, and \
         nothing else",
        &format!("{draft:?}"),
        draft == ripple(MIDDLE, [2.5, 15.0, 20.0, 40.0, 30.0]),
    );
    let mut draft = with(1, 1.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft of wavelength 1 holds the wavelength at 1, its range's bottom, \
         rather than leaving the effect out",
        &format!("{draft:?}"),
        draft == ripple(MIDDLE, [2.5, 1.0, 20.0, 0.0, 0.0]),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_ripple_001.json",
        "fx_ripple_009.json",
        "fx_ripple_010.json",
        "fx_ripple_012.json",
        "fx_ripple_014.json",
        "fx_ripple_019.json",
        "fx_ripple_020.json",
        "fx_ripple_021.json",
        "fx_ripple_022.json",
        "fx_ripple_023.json",
        "fx_ripple_024.json",
        "fx_ripple_025.json",
        "fx_ripple_026.json",
    ]);
    t.shape_refused(
        "fx_ripple_001.json",
        "no `fade` at all",
        r#"{"center": [50, 50], "amplitude": 5, "wavelength": 30, "speed": 20, "phase": 0}"#,
    );
    t.shape_refused(
        "fx_ripple_001.json",
        "a centre of one number",
        r#"{"center": 50, "amplitude": 5, "wavelength": 30, "speed": 20, "phase": 0, "fade": 0}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_ripple_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("amplitude 1001", set(with(0, 1001.0))),
            ("amplitude -1", set(with(0, -1.0))),
            ("wavelength 0", set(with(1, 0.0))),
            ("speed 361", set(with(2, 361.0))),
            ("phase -100001", set(with(3, -100001.0))),
            ("fade -1", set(with(4, -1.0))),
            ("centre 50, 1001", set(ripple([50.0, 1001.0], START))),
            ("amplitude keyed to 1500", keys("amplitude", &[(0, &[5.0]), (4, &[1500.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_ripple_001.json",
        vec![
            (
                "every number at its bottom,",
                set(ripple([-1000.0, -1000.0], [0.0, 1.0, -360.0, -100000.0, 0.0])),
            ),
            (
                "every number at its top,",
                set(ripple([1000.0, 1000.0], [1000.0, 10000.0, 360.0, 100000.0, 100000.0])),
            ),
            (
                "the centre keyed from 50, 50 to 0, 0",
                keys("center", &[(0, &[50.0, 50.0]), (4, &[0.0, 0.0])]),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_ripple_001.json", 2),
        ("fx_ripple_008.json", 0),
        ("fx_ripple_015.json", 2),
    ]);

    t.finish("B-93_ripple_table.md");
}
