//! B-99: radial wipe in the core, against D-156.
//!
//! Writes `verification/B-99_radial_wipe_table.md`.
//!
//! Every expected pixel is `Fixtures/radial_wipe/expected_radial_wipe.json`, written by
//! `tools/radial_wipe_reference.py` before this code existed and printed in document 25 as
//! FX-RWIPE-001 to 029. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn radial_wipe(completion: f64, start_angle: f64, center: [f64; 2], wipe: &str, feather: f64) -> Effect {
    Effect::RadialWipe {
        completion,
        start_angle,
        center,
        wipe: wipe.to_string(),
        feather,
    }
}

const MIDDLE: [f64; 2] = [50.0, 50.0];

#[test]
fn b99_radial_wipe() {
    let mut t = Table::new(
        "radial_wipe",
        "# B-99: radial wipe\n\nD-156, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the twenty-third of the third batch. Every expected pixel is \
         `Fixtures/radial_wipe/expected_radial_wipe.json`, written by \
         `tools/radial_wipe_reference.py` before this code existed and printed in document 25 \
         as FX-RWIPE-001 to 029. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-RWIPE-001 to 029 (document 25)");
    t.fixtures("expected_radial_wipe.json");

    t.heading("How far it reaches");
    let got = radial_wipe(50.0, 0.0, MIDDLE, "both", 360.0).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = radial_wipe(50.0, 30.0, MIDDLE, "clockwise", 90.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: the feather is in degrees, not pixels",
        &format!("{draft:?}"),
        draft == radial_wipe(50.0, 30.0, MIDDLE, "clockwise", 90.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_rwipe_001.json",
        "fx_rwipe_012.json",
        "fx_rwipe_014.json",
        "fx_rwipe_016.json",
        "fx_rwipe_018.json",
        "fx_rwipe_019.json",
        "fx_rwipe_021.json",
        "fx_rwipe_022.json",
        "fx_rwipe_023.json",
        "fx_rwipe_024.json",
        "fx_rwipe_025.json",
        "fx_rwipe_026.json",
        "fx_rwipe_027.json",
        "fx_rwipe_028.json",
        "fx_rwipe_029.json",
    ]);
    t.shape_refused(
        "fx_rwipe_001.json",
        "no `wipe` at all",
        r#"{"completion": 0, "start_angle": 0, "center": [50, 50], "feather": 0}"#,
    );
    t.shape_refused(
        "fx_rwipe_001.json",
        "a centre of one number",
        r#"{"completion": 0, "start_angle": 0, "center": [50], "wipe": "clockwise", "feather": 0}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_rwipe_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("completion 101", set(radial_wipe(101.0, 0.0, MIDDLE, "clockwise", 0.0))),
            ("start angle -3601", set(radial_wipe(0.0, -3601.0, MIDDLE, "clockwise", 0.0))),
            ("centre 1001, 50", set(radial_wipe(0.0, 0.0, [1001.0, 50.0], "clockwise", 0.0))),
            ("feather 361", set(radial_wipe(0.0, 0.0, MIDDLE, "clockwise", 361.0))),
            ("feather -1", set(radial_wipe(0.0, 0.0, MIDDLE, "clockwise", -1.0))),
            ("wipe \"spiral\"", set(radial_wipe(0.0, 0.0, MIDDLE, "spiral", 0.0))),
            ("wipe \"Clockwise\"", set(radial_wipe(0.0, 0.0, MIDDLE, "Clockwise", 0.0))),
            ("completion keyed to 150", keys("completion", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_rwipe_001.json",
        vec![
            (
                "every number at its bottom,",
                set(radial_wipe(0.0, -3600.0, [-1000.0, -1000.0], "counterclockwise", 0.0)),
            ),
            (
                "every number at its top,",
                set(radial_wipe(100.0, 3600.0, [1000.0, 1000.0], "both", 360.0)),
            ),
            ("completion keyed from 0 to 100", keys("completion", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_rwipe_007.json", 0),
        ("fx_rwipe_010.json", 0),
        ("fx_rwipe_020.json", 3),
        ("fx_rwipe_021.json", 0),
    ]);

    t.finish("B-99_radial_wipe_table.md");
}
