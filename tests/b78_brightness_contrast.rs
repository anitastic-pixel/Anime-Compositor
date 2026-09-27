//! B-78: brightness and contrast in the core, against D-135.
//!
//! Writes `verification/B-78_brightness_contrast_table.md`.
//!
//! Every expected pixel is `Fixtures/brightness_contrast/expected_brightness_contrast.json`,
//! written by `tools/brightness_contrast_reference.py` before this code existed and printed in
//! document 25 as FX-BRICON-001 to 022. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn bricon(brightness: f64, contrast: f64) -> Effect {
    Effect::BrightnessContrast {
        brightness,
        contrast,
    }
}

#[test]
fn b78_brightness_contrast() {
    let mut t = Table::new(
        "brightness_contrast",
        "# B-78: brightness and contrast\n\nD-135, accepted on 2026-09-26 by the owner's message \
         asking for thirty more effects, the second of the third batch. Every expected pixel is \
         `Fixtures/brightness_contrast/expected_brightness_contrast.json`, written by \
         `tools/brightness_contrast_reference.py` before this code existed and printed in \
         document 25 as FX-BRICON-001 to 022. The build's frame is compared sample by sample; \
         the answer is the largest difference over all of them, against the catalogue's \
         tolerance of 2e-5.\n",
    );

    t.heading("FX-BRICON-001 to 022 (document 25)");
    t.fixtures("expected_brightness_contrast.json");

    t.heading("How far it reaches");
    let got = bricon(150.0, 100.0).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = bricon(30.0, 40.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: it has no distances",
        &format!("{draft:?}"),
        draft == bricon(30.0, 40.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_bricon_001.json",
        "fx_bricon_004.json",
        "fx_bricon_009.json",
        "fx_bricon_010.json",
        "fx_bricon_013.json",
        "fx_bricon_014.json",
        "fx_bricon_015.json",
        "fx_bricon_016.json",
        "fx_bricon_017.json",
        "fx_bricon_022.json",
    ]);
    t.shape_refused("fx_bricon_001.json", "no `contrast` at all", r#"{"brightness": 0}"#);
    t.shape_refused(
        "fx_bricon_001.json",
        "a brightness that is a word",
        r#"{"brightness": "bright", "contrast": 0}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_bricon_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("brightness 151", set(bricon(151.0, 0.0))),
            ("brightness -151", set(bricon(-151.0, 0.0))),
            ("contrast 101", set(bricon(0.0, 101.0))),
            ("contrast -101", set(bricon(0.0, -101.0))),
            ("contrast keyed to 120", keys("contrast", &[(0, &[0.0]), (4, &[120.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_bricon_001.json",
        vec![
            ("brightness -150 and contrast 100, the ends,", set(bricon(-150.0, 100.0))),
            ("brightness 150 and contrast -100, the other ends,", set(bricon(150.0, -100.0))),
            ("brightness keyed from 0 to 150", keys("brightness", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_bricon_006.json", 0),
        ("fx_bricon_010.json", 0),
        ("fx_bricon_012.json", 2),
    ]);

    t.finish("B-78_brightness_contrast_table.md");
}
