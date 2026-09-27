//! B-85: solarize in the core, against D-142.
//!
//! Writes `verification/B-85_solarize_table.md`.
//!
//! Every expected pixel is `Fixtures/solarize/expected_solarize.json`, written by
//! `tools/solarize_reference.py` before this code existed and printed in document 25 as
//! FX-SOLAR-001 to 016. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn solarize(threshold: f64) -> Effect {
    Effect::Solarize { threshold }
}

#[test]
fn b85_solarize() {
    let mut t = Table::new(
        "solarize",
        "# B-85: solarize\n\nD-142, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the ninth of the third batch. Every expected pixel is \
         `Fixtures/solarize/expected_solarize.json`, written by `tools/solarize_reference.py` \
         before this code existed and printed in document 25 as FX-SOLAR-001 to 016. The \
         build's frame is compared sample by sample; the answer is the largest difference over \
         all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-SOLAR-001 to 016 (document 25)");
    t.fixtures("expected_solarize.json");

    t.heading("How far it reaches");
    let got = solarize(0.0).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = solarize(128.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: it has no distances",
        &format!("{draft:?}"),
        draft == solarize(128.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_solar_001.json",
        "fx_solar_008.json",
        "fx_solar_009.json",
        "fx_solar_010.json",
        "fx_solar_011.json",
        "fx_solar_012.json",
        "fx_solar_013.json",
        "fx_solar_014.json",
        "fx_solar_015.json",
        "fx_solar_016.json",
    ]);
    t.shape_refused("fx_solar_001.json", "no `threshold` at all", r#"{}"#);
    t.shape_refused(
        "fx_solar_001.json",
        "a threshold that is a word",
        r#"{"threshold": "half"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_solar_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("threshold -1", set(solarize(-1.0))),
            ("threshold 255.5", set(solarize(255.5))),
            ("threshold keyed to 300", keys("threshold", &[(0, &[128.0]), (4, &[300.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_solar_001.json",
        vec![
            ("threshold 0, the bottom,", set(solarize(0.0))),
            ("threshold 255, the top,", set(solarize(255.0))),
            ("threshold keyed from 0 to 255", keys("threshold", &[(0, &[0.0]), (4, &[255.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_solar_001.json", 0), ("fx_solar_007.json", 0), ("fx_solar_009.json", 2)]);

    t.finish("B-85_solarize_table.md");
}
