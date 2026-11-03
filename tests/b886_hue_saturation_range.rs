//! B-886: Hue/Saturation's Channel Range handles, against D-1006.
//!
//! Writes `verification/B-886_hue_saturation_range_table.md`.
//!
//! Every expected pixel is `Fixtures/hue_saturation/expected_hue_saturation_range.json`,
//! written by `tools/hue_saturation_range_reference.py` before this code existed and printed in
//! document 25 as FX-HUESAT-037 to 047. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{set, Table};

use anime_compositor::effects::{hue_range_default, Effect};

fn reds(bounds: Vec<f64>) -> Effect {
    let mut all: [Vec<f64>; 6] = std::array::from_fn(hue_range_default);
    all[0] = bounds;
    Effect::HueSaturation {
        hue: 0.0,
        saturation: 0.0,
        lightness: 0.0,
        ranges: [[0.0, -100.0, 0.0], [0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3]],
        bounds: all,
        colorize: "off".into(),
        colorize_hue: 0.0,
        colorize_saturation: 25.0,
        colorize_lightness: 0.0,
    }
}

#[test]
fn b886_hue_saturation_range() {
    let mut t = Table::new(
        "hue_saturation",
        "# B-886: Hue/Saturation's Channel Range\n\nD-1006, accepted on 2026-10-27. Every \
         expected pixel is `Fixtures/hue_saturation/expected_hue_saturation_range.json`, written \
         by `tools/hue_saturation_range_reference.py` before this code existed and printed in \
         document 25 as FX-HUESAT-037 to 047. The build's frame is compared sample by sample; \
         the answer is the largest difference over all of them, against the catalogue's \
         tolerance.\n",
    );

    t.heading("FX-HUESAT-037 to 047 (document 25)");
    t.fixtures_numbered("expected_hue_saturation_range.json", 37..=47);

    t.heading("The file");
    let old = t.saved_parameters("fx_huesat_024.json");
    let quiet = anime_compositor::effects::HUE_RANGE_BOUNDS
        .iter()
        .all(|k| old.get(*k).is_none());
    t.row(
        "fx_huesat_024.json, a file from before the handles, opens at After Effects' own and is \
         saved without them",
        &old.to_string(),
        quiet,
    );
    t.round_trips(&[
        "fx_huesat_001.json",
        "fx_huesat_024.json",
        "fx_huesat_037.json",
        "fx_huesat_038.json",
        "fx_huesat_043.json",
        "fx_huesat_044.json",
        "fx_huesat_047.json",
    ]);
    t.shape_refused(
        "fx_huesat_038.json",
        "a handle written as a word",
        r#"{"hue": 0, "saturation": 0, "lightness": 0, "reds_hsl": [0, -100, 0], "reds_range": [275, "a", 35, 85]}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_huesat_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("reds handles backwards, more than a turn", set(reds(vec![45.0, 15.0, 345.0, 315.0]))),
            ("a reds handle at 361", set(reds(vec![315.0, 345.0, 15.0, 361.0]))),
            ("a reds handle at -1", set(reds(vec![-1.0, 345.0, 15.0, 45.0]))),
            ("reds with three handles", set(reds(vec![315.0, 345.0, 15.0]))),
            ("reds with five handles", set(reds(vec![315.0, 345.0, 15.0, 45.0, 50.0]))),
        ],
    );
    t.taken(
        &mut document,
        "fx_huesat_001.json",
        vec![
            ("reds widened to 275, 320, 35, 85", set(reds(vec![275.0, 320.0, 35.0, 85.0]))),
            ("reds as one hue, 0, 0, 0, 0", set(reds(vec![0.0, 0.0, 0.0, 0.0]))),
            ("reds at 0 and 360, the same place", set(reds(vec![300.0, 360.0, 0.0, 60.0]))),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_huesat_038.json", 0), ("fx_huesat_042.json", 0)]);

    t.finish("B-886_hue_saturation_range_table.md");
}
