//! B-83: vibrance in the core, against D-140.
//!
//! Writes `verification/B-83_vibrance_table.md`.
//!
//! Every expected pixel is `Fixtures/vibrance/expected_vibrance.json`, written by
//! `tools/vibrance_reference.py` before this code existed and printed in document 25 as
//! FX-VIBRANCE-001 to 018. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn vib(vibrance: f64, saturation: f64) -> Effect {
    Effect::Vibrance {
        vibrance,
        saturation,
    }
}

#[test]
fn b83_vibrance() {
    let mut t = Table::new(
        "vibrance",
        "# B-83: vibrance\n\nD-140, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the seventh of the third batch. Every expected pixel is \
         `Fixtures/vibrance/expected_vibrance.json`, written by `tools/vibrance_reference.py` \
         before this code existed and printed in document 25 as FX-VIBRANCE-001 to 018. The \
         build's frame is compared sample by sample; the answer is the largest difference over \
         all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-VIBRANCE-001 to 018 (document 25)");
    t.fixtures("expected_vibrance.json");

    t.heading("How far it reaches");
    let got = vib(100.0, 100.0).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = vib(40.0, 20.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: it has no distances",
        &format!("{draft:?}"),
        draft == vib(40.0, 20.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_vibrance_001.json",
        "fx_vibrance_009.json",
        "fx_vibrance_010.json",
        "fx_vibrance_012.json",
        "fx_vibrance_013.json",
        "fx_vibrance_014.json",
        "fx_vibrance_016.json",
        "fx_vibrance_018.json",
    ]);
    t.shape_refused("fx_vibrance_001.json", "no `saturation` at all", r#"{"vibrance": 0}"#);
    t.shape_refused(
        "fx_vibrance_001.json",
        "a vibrance that is a word",
        r#"{"vibrance": "lots", "saturation": 0}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_vibrance_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("vibrance 101", set(vib(101.0, 0.0))),
            ("vibrance -101", set(vib(-101.0, 0.0))),
            ("saturation 101", set(vib(0.0, 101.0))),
            ("saturation -101", set(vib(0.0, -101.0))),
            ("vibrance keyed to -150", keys("vibrance", &[(0, &[0.0]), (4, &[-150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_vibrance_001.json",
        vec![
            ("vibrance -100 and saturation 100, the ends,", set(vib(-100.0, 100.0))),
            ("vibrance 100 and saturation -100, the other ends,", set(vib(100.0, -100.0))),
            ("saturation keyed from -100 to 100", keys("saturation", &[(0, &[-100.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_vibrance_002.json", 0),
        ("fx_vibrance_009.json", 0),
        ("fx_vibrance_011.json", 2),
    ]);

    t.finish("B-83_vibrance_table.md");
}
