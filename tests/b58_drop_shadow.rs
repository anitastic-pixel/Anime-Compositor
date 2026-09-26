//! B-58: the drop shadow in the core, against D-115.
//!
//! Writes `verification/B-58_drop_shadow_table.md`.
//!
//! Every expected pixel is `Fixtures/drop_shadow/expected_drop_shadow.json`, written by
//! `tools/drop_shadow_reference.py` before this code existed and printed in document 25 as
//! FX-SHADOW-001 to 023. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn shadow(color: &str, opacity: f64, direction: f64, distance: f64, softness: f64) -> Effect {
    Effect::DropShadow {
        color: color.into(),
        opacity,
        direction,
        distance,
        softness,
    }
}

#[test]
fn b58_drop_shadow() {
    let mut t = Table::new(
        "drop_shadow",
        "# B-58: drop shadow\n\nD-115, accepted by the owner on 2026-09-26 in the batch of ten. \
         Every expected pixel is `Fixtures/drop_shadow/expected_drop_shadow.json`, written by \
         `tools/drop_shadow_reference.py` before this code existed and printed in document 25 \
         as FX-SHADOW-001 to 023. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-SHADOW-001 to 023 (document 25)");
    t.fixtures("expected_drop_shadow.json");

    t.heading("How far it reaches");
    for (what, e, want) in [
        ("the defaults, distance 5, grow the drawing by 5 on every side", shadow("#000000", 50.0, 135.0, 5.0, 0.0), 5),
        ("distance 2.5 and softness 6 grow it by 3, the distance rounded up, and 6, the blur's reach", shadow("#000000", 0.0, 90.0, 2.5, 6.0), 9),
        ("distance 0, softness 0 does not grow it", shadow("#000000", 50.0, 135.0, 0.0, 0.0), 0),
    ] {
        let got = e.bounds_expansion();
        t.row(&format!("{what}"), &got.to_string(), got == want);
    }
    let mut draft = shadow("#000000", 50.0, 135.0, 5.0, 6.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the distance and the softness, and nothing else",
        &format!("{draft:?}"),
        draft == shadow("#000000", 50.0, 135.0, 2.5, 3.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_shadow_001.json",
        "fx_shadow_011.json",
        "fx_shadow_013.json",
        "fx_shadow_014.json",
        "fx_shadow_015.json",
        "fx_shadow_018.json",
        "fx_shadow_019.json",
        "fx_shadow_020.json",
        "fx_shadow_021.json",
        "fx_shadow_022.json",
        "fx_shadow_023.json",
    ]);
    let colour = t.saved_parameters("fx_shadow_012.json")["color"].clone();
    t.row(
        "fx_shadow_012.json, its colour in capitals, is saved with it in small letters",
        &colour.to_string(),
        colour == "#2040a0",
    );
    t.shape_refused(
        "fx_shadow_001.json",
        "no `softness` at all",
        r##"{"color": "#000000", "opacity": 50, "direction": 135, "distance": 5}"##,
    );
    t.shape_refused(
        "fx_shadow_001.json",
        "a distance written as a word",
        r##"{"color": "#000000", "opacity": 50, "direction": 135, "distance": "five",
            "softness": 0}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_shadow_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("opacity 101", set(shadow("#000000", 101.0, 135.0, 5.0, 0.0))),
            ("opacity -1", set(shadow("#000000", -1.0, 135.0, 5.0, 0.0))),
            ("direction 3601", set(shadow("#000000", 50.0, 3601.0, 5.0, 0.0))),
            ("direction -3601", set(shadow("#000000", 50.0, -3601.0, 5.0, 0.0))),
            ("distance -1", set(shadow("#000000", 50.0, 135.0, -1.0, 0.0))),
            ("distance 1001", set(shadow("#000000", 50.0, 135.0, 1001.0, 0.0))),
            ("softness 501", set(shadow("#000000", 50.0, 135.0, 5.0, 501.0))),
            ("colour \"black\"", set(shadow("black", 50.0, 135.0, 5.0, 0.0))),
            ("opacity keyed to 150", keys("opacity", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_shadow_001.json",
        vec![
            ("opacity 100, distance 1000 and softness 500, the tops,", set(shadow("#2040a0", 100.0, 3600.0, 1000.0, 500.0))),
            ("direction -3600, the bottom,", set(shadow("#000000", 50.0, -3600.0, 0.0, 0.0))),
            ("distance keyed from 0 to 4", keys("distance", &[(0, &[0.0]), (4, &[4.0])])),
            ("direction keyed from 0 to 180", keys("direction", &[(0, &[0.0]), (4, &[180.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_shadow_009.json", 0), ("fx_shadow_013.json", 2)]);

    t.finish("B-58_drop_shadow_table.md");
}
