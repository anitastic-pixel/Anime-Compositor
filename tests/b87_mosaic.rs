//! B-87: mosaic in the core, against D-144.
//!
//! Writes `verification/B-87_mosaic_table.md`.
//!
//! Every expected pixel is `Fixtures/mosaic/expected_mosaic.json`, written by
//! `tools/mosaic_reference.py` before this code existed and printed in document 25 as
//! FX-MOSAIC-001 to 020. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn mosaic(size: f64) -> Effect {
    Effect::Mosaic { size }
}

#[test]
fn b87_mosaic() {
    let mut t = Table::new(
        "mosaic",
        "# B-87: mosaic\n\nD-144, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the eleventh of the third batch. Every expected pixel is \
         `Fixtures/mosaic/expected_mosaic.json`, written by `tools/mosaic_reference.py` before \
         this code existed and printed in document 25 as FX-MOSAIC-001 to 020. The build's \
         frame is compared sample by sample; the answer is the largest difference over all of \
         them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-MOSAIC-001 to 020 (document 25)");
    t.fixtures("expected_mosaic.json");

    t.heading("How far it reaches");
    let got = mosaic(1000.0).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = mosaic(10.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the size, 10 to 5",
        &format!("{draft:?}"),
        draft == mosaic(5.0),
    );
    let mut draft = mosaic(1.5);
    draft.scale_distances(|d| d * 0.25);
    t.row(
        "a quarter-size draft of size 1.5 holds the size at 1, which changes nothing, rather \
         than dropping the effect",
        &format!("{draft:?}"),
        draft == mosaic(1.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_mosaic_001.json",
        "fx_mosaic_006.json",
        "fx_mosaic_010.json",
        "fx_mosaic_011.json",
        "fx_mosaic_013.json",
        "fx_mosaic_015.json",
        "fx_mosaic_016.json",
        "fx_mosaic_018.json",
        "fx_mosaic_019.json",
        "fx_mosaic_020.json",
    ]);
    t.shape_refused("fx_mosaic_001.json", "no `size` at all", r#"{}"#);
    t.shape_refused("fx_mosaic_001.json", "a size that is a word", r#"{"size": "big"}"#);

    t.heading("Commands");
    let mut document = t.load("fx_mosaic_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("size 0.5", set(mosaic(0.5))),
            ("size 1001", set(mosaic(1001.0))),
            ("size keyed to 0", keys("size", &[(0, &[10.0]), (4, &[0.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_mosaic_001.json",
        vec![
            ("size 1, the bottom,", set(mosaic(1.0))),
            ("size 1000, the top,", set(mosaic(1000.0))),
            ("size keyed from 2 to 6", keys("size", &[(0, &[2.0]), (4, &[6.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_mosaic_005.json", 0),
        ("fx_mosaic_006.json", 0),
        ("fx_mosaic_013.json", 0),
    ]);

    t.finish("B-87_mosaic_table.md");
}
