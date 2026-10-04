//! B-56: hue and saturation in the core, against D-113.
//!
//! Writes `verification/B-56_hue_saturation_table.md`.
//!
//! Every expected pixel is `Fixtures/hue_saturation/expected_hue_saturation.json`, written by
//! `tools/hue_saturation_reference.py` before this code existed and printed in document 25 as
//! FX-HUESAT-001 to 023. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn hs(hue: f64, saturation: f64, lightness: f64) -> Effect {
    Effect::HueSaturation {
        hue,
        saturation,
        lightness,
        ranges: [[0.0; 3]; 6],
    }
}

#[test]
fn b56_hue_saturation() {
    let mut t = Table::new(
        "hue_saturation",
        "# B-56: hue/saturation\n\nD-113, accepted by the owner on 2026-09-26 in the batch of \
         ten. Every expected pixel is `Fixtures/hue_saturation/expected_hue_saturation.json`, \
         written by `tools/hue_saturation_reference.py` before this code existed and printed in \
         document 25 as FX-HUESAT-001 to 023. The build's frame is compared sample by sample; \
         the answer is the largest difference over all of them, against the catalogue's \
         tolerance of 2e-5.\n",
    );

    t.heading("FX-HUESAT-001 to 023 (document 25)");
    t.fixtures("expected_hue_saturation.json");

    t.heading("How far it reaches");
    let all = hs(60.0, -50.0, 20.0);
    let got = all.bounds_expansion();
    t.row(
        "it grows the drawing's bounds by nothing: each pixel is regraded where it is",
        &got.to_string(),
        got == 0,
    );
    let mut draft = all.clone();
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: hue and saturation are colours, not distances",
        &format!("{draft:?}"),
        draft == all,
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_huesat_002.json",
        "fx_huesat_004.json",
        "fx_huesat_008.json",
        "fx_huesat_013.json",
        "fx_huesat_014.json",
        "fx_huesat_015.json",
        "fx_huesat_016.json",
        "fx_huesat_018.json",
        "fx_huesat_022.json",
        "fx_huesat_023.json",
    ]);
    t.shape_refused(
        "fx_huesat_001.json",
        "no `saturation` at all",
        r#"{"hue": 0, "lightness": 0}"#,
    );
    t.shape_refused(
        "fx_huesat_001.json",
        "a lightness written as a word",
        r#"{"hue": 0, "saturation": 0, "lightness": "one hundred and one"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_huesat_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("hue 181", set(hs(181.0, 0.0, 0.0))),
            ("hue -181", set(hs(-181.0, 0.0, 0.0))),
            ("saturation 101", set(hs(0.0, 101.0, 0.0))),
            ("saturation -101", set(hs(0.0, -101.0, 0.0))),
            ("lightness 101", set(hs(0.0, 0.0, 101.0))),
            ("lightness -101", set(hs(0.0, 0.0, -101.0))),
            ("hue keyed to 200", keys("hue", &[(0, &[0.0]), (4, &[200.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_huesat_001.json",
        vec![
            ("hue 180, saturation and lightness 100, the tops of the ranges,", set(hs(180.0, 100.0, 100.0))),
            ("hue -180, saturation and lightness -100, the bottoms,", set(hs(-180.0, -100.0, -100.0))),
            ("hue keyed from -180 to 180", keys("hue", &[(0, &[-180.0]), (4, &[180.0])])),
            ("lightness keyed from 0 to 50", keys("lightness", &[(0, &[0.0]), (4, &[50.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_huesat_014.json", 0), ("fx_huesat_017.json", 3)]);

    t.finish("B-56_hue_saturation_table.md");
}
