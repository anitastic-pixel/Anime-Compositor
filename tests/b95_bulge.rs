//! B-95: bulge in the core, against D-152.
//!
//! Writes `verification/B-95_bulge_table.md`.
//!
//! Every expected pixel is `Fixtures/bulge/expected_bulge.json`, written by
//! `tools/bulge_reference.py` before this code existed and printed in document 25 as
//! FX-BULGE-001 to 023. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn bulge(center: [f64; 2], radius: f64, height: f64) -> Effect {
    Effect::Bulge {
        center,
        radius,
        height,
    }
}

const MIDDLE: [f64; 2] = [50.0, 50.0];

#[test]
fn b95_bulge() {
    let mut t = Table::new(
        "bulge",
        "# B-95: bulge\n\nD-152, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the nineteenth of the third batch. Every expected pixel is \
         `Fixtures/bulge/expected_bulge.json`, written by `tools/bulge_reference.py` before \
         this code existed and printed in document 25 as FX-BULGE-001 to 023. The build's \
         frame is compared sample by sample; the answer is the largest difference over all of \
         them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-BULGE-001 to 023 (document 25)");
    t.fixtures("expected_bulge.json");

    t.heading("How far it reaches");
    let got = bulge(MIDDLE, 10000.0, -4.0).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = bulge(MIDDLE, 50.0, 1.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the radius, 50 to 25, and nothing else",
        &format!("{draft:?}"),
        draft == bulge(MIDDLE, 25.0, 1.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_bulge_001.json",
        "fx_bulge_009.json",
        "fx_bulge_011.json",
        "fx_bulge_013.json",
        "fx_bulge_014.json",
        "fx_bulge_018.json",
        "fx_bulge_019.json",
        "fx_bulge_020.json",
        "fx_bulge_021.json",
        "fx_bulge_022.json",
        "fx_bulge_023.json",
    ]);
    t.shape_refused(
        "fx_bulge_001.json",
        "no `height` at all",
        r#"{"center": [50, 50], "radius": 50}"#,
    );
    t.shape_refused(
        "fx_bulge_001.json",
        "a height that is a word",
        r#"{"center": [50, 50], "radius": 50, "height": "big"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_bulge_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("radius -1", set(bulge(MIDDLE, -1.0, 1.0))),
            ("radius 10001", set(bulge(MIDDLE, 10001.0, 1.0))),
            ("height 4.5", set(bulge(MIDDLE, 50.0, 4.5))),
            ("height -4.5", set(bulge(MIDDLE, 50.0, -4.5))),
            ("centre 50, 1001", set(bulge([50.0, 1001.0], 50.0, 1.0))),
            ("height keyed to 5", keys("height", &[(0, &[1.0]), (4, &[5.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_bulge_001.json",
        vec![
            ("every number at its bottom,", set(bulge([-1000.0, -1000.0], 0.0, -4.0))),
            ("every number at its top,", set(bulge([1000.0, 1000.0], 10000.0, 4.0))),
            ("height keyed from 0 to 2", keys("height", &[(0, &[0.0]), (4, &[2.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_bulge_004.json", 0),
        ("fx_bulge_007.json", 0),
        ("fx_bulge_015.json", 0),
    ]);

    t.finish("B-95_bulge_table.md");
}
