//! B-94: twirl in the core, against D-151.
//!
//! Writes `verification/B-94_twirl_table.md`.
//!
//! Every expected pixel is `Fixtures/twirl/expected_twirl.json`, written by
//! `tools/twirl_reference.py` before this code existed and printed in document 25 as
//! FX-TWIRL-001 to 021. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn twirl(angle: f64, radius: f64, center: [f64; 2]) -> Effect {
    Effect::Twirl {
        angle,
        radius,
        center,
    }
}

const MIDDLE: [f64; 2] = [50.0, 50.0];

#[test]
fn b94_twirl() {
    let mut t = Table::new(
        "twirl",
        "# B-94: twirl\n\nD-151, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the eighteenth of the third batch. Every expected pixel is \
         `Fixtures/twirl/expected_twirl.json`, written by `tools/twirl_reference.py` before \
         this code existed and printed in document 25 as FX-TWIRL-001 to 021. The build's \
         frame is compared sample by sample; the answer is the largest difference over all of \
         them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-TWIRL-001 to 021 (document 25)");
    t.fixtures("expected_twirl.json");

    t.heading("How far it reaches");
    let got = twirl(3600.0, 10000.0, MIDDLE).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = twirl(90.0, 50.0, MIDDLE);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the radius, 50 to 25, and nothing else",
        &format!("{draft:?}"),
        draft == twirl(90.0, 25.0, MIDDLE),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_twirl_001.json",
        "fx_twirl_009.json",
        "fx_twirl_010.json",
        "fx_twirl_012.json",
        "fx_twirl_013.json",
        "fx_twirl_016.json",
        "fx_twirl_017.json",
        "fx_twirl_018.json",
        "fx_twirl_019.json",
        "fx_twirl_020.json",
        "fx_twirl_021.json",
    ]);
    t.shape_refused(
        "fx_twirl_001.json",
        "no `center` at all",
        r#"{"angle": 90, "radius": 50}"#,
    );
    t.shape_refused(
        "fx_twirl_001.json",
        "an angle that is a word",
        r#"{"angle": "right", "radius": 50, "center": [50, 50]}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_twirl_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("angle 3601", set(twirl(3601.0, 50.0, MIDDLE))),
            ("angle -3601", set(twirl(-3601.0, 50.0, MIDDLE))),
            ("radius -1", set(twirl(90.0, -1.0, MIDDLE))),
            ("radius 10001", set(twirl(90.0, 10001.0, MIDDLE))),
            ("centre 50, 1001", set(twirl(90.0, 50.0, [50.0, 1001.0]))),
            ("radius keyed to 20000", keys("radius", &[(0, &[50.0]), (4, &[20000.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_twirl_001.json",
        vec![
            ("every number at its bottom,", set(twirl(-3600.0, 0.0, [-1000.0, -1000.0]))),
            ("every number at its top,", set(twirl(3600.0, 10000.0, [1000.0, 1000.0]))),
            ("angle keyed from 0 to 180", keys("angle", &[(0, &[0.0]), (4, &[180.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_twirl_001.json", 0),
        ("fx_twirl_007.json", 0),
        ("fx_twirl_014.json", 3),
        ("fx_twirl_015.json", 0),
    ]);

    t.finish("B-94_twirl_table.md");
}
