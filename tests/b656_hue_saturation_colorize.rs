//! B-656: Hue/Saturation's Colorize, against D-776.
//!
//! Writes `verification/B-656_hue_saturation_colorize_table.md`.
//!
//! Every expected pixel is `Fixtures/hue_saturation/expected_hue_saturation.json`, written by
//! `tools/hue_saturation_reference.py` before this code existed and printed in document 25 as
//! FX-HUESAT-027 to 036. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn cz(word: &str, hue: f64, saturation: f64, lightness: f64) -> Effect {
    Effect::HueSaturation {
        hue: 0.0,
        saturation: 0.0,
        lightness: 0.0,
        ranges: [[0.0; 3]; 6],
        colorize: word.into(),
        colorize_hue: hue,
        colorize_saturation: saturation,
        colorize_lightness: lightness,
    }
}

#[test]
fn b656_hue_saturation_colorize() {
    let mut t = Table::new(
        "hue_saturation",
        "# B-656: Hue/Saturation's Colorize\n\nD-776, accepted on 2026-10-24. Every expected \
         pixel is `Fixtures/hue_saturation/expected_hue_saturation.json`, written by \
         `tools/hue_saturation_reference.py` before this code existed and printed in document \
         25 as FX-HUESAT-027 to 036. The build's frame is compared sample by sample; the answer \
         is the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-HUESAT-027 to 036 (document 25)");
    t.fixtures_numbered("expected_hue_saturation.json", 27..=36);

    t.heading("The file");
    let old = t.saved_parameters("fx_huesat_001.json");
    let quiet = ["colorize", "colorize_hue", "colorize_saturation", "colorize_lightness"]
        .iter()
        .all(|k| old.get(*k).is_none());
    t.row(
        "fx_huesat_001.json, a file from before Colorize, opens with it off and is saved without \
         its words",
        &old.to_string(),
        quiet,
    );
    t.round_trips(&[
        "fx_huesat_001.json",
        "fx_huesat_024.json",
        "fx_huesat_027.json",
        "fx_huesat_028.json",
        "fx_huesat_029.json",
        "fx_huesat_031.json",
        "fx_huesat_032.json",
        "fx_huesat_033.json",
        "fx_huesat_034.json",
        "fx_huesat_036.json",
    ]);
    t.shape_refused(
        "fx_huesat_027.json",
        "a colorize hue written as a word",
        r#"{"hue": 0, "saturation": 0, "lightness": 0, "colorize": "on", "colorize_hue": "red"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_huesat_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("colorize \"yes\"", set(cz("yes", 0.0, 25.0, 0.0))),
            ("colorize hue 361", set(cz("on", 361.0, 25.0, 0.0))),
            ("colorize hue -1", set(cz("on", -1.0, 25.0, 0.0))),
            ("colorize saturation 101", set(cz("on", 0.0, 101.0, 0.0))),
            ("colorize saturation -1", set(cz("on", 0.0, -1.0, 0.0))),
            ("colorize lightness 101", set(cz("on", 0.0, 25.0, 101.0))),
            ("colorize lightness -101", set(cz("on", 0.0, 25.0, -101.0))),
            (
                "colorize hue keyed to 400",
                keys("colorize_hue", &[(0, &[0.0]), (4, &[400.0])]),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_huesat_001.json",
        vec![
            (
                "colorize on, hue 360, saturation 100 and lightness 100, the tops of the ranges,",
                set(cz("on", 360.0, 100.0, 100.0)),
            ),
            (
                "colorize on, hue 0, saturation 0 and lightness -100, the bottoms,",
                set(cz("on", 0.0, 0.0, -100.0)),
            ),
            (
                "colorize hue keyed from 0 to 360",
                keys("colorize_hue", &[(0, &[0.0]), (4, &[360.0])]),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_huesat_028.json", 0), ("fx_huesat_031.json", 2)]);

    t.finish("B-656_hue_saturation_colorize_table.md");
}
