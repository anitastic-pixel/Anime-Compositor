//! B-61: the outline in the core, against D-118.
//!
//! Writes `verification/B-61_outline_table.md`.
//!
//! Every expected pixel is `Fixtures/outline/expected_outline.json`, written by
//! `tools/outline_reference.py` before this code existed and printed in document 25 as
//! FX-OUTLINE-001 to 020. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn outline(color: &str, width: f64, softness: f64, opacity: f64) -> Effect {
    Effect::Outline {
        color: color.into(),
        width,
        softness,
        opacity,
    }
}

#[test]
fn b61_outline() {
    let mut t = Table::new(
        "outline",
        "# B-61: outline\n\nD-118, accepted by the owner on 2026-09-26 in the batch of ten. \
         Every expected pixel is `Fixtures/outline/expected_outline.json`, written by \
         `tools/outline_reference.py` before this code existed and printed in document 25 as \
         FX-OUTLINE-001 to 020. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-OUTLINE-001 to 020 (document 25)");
    t.fixtures("expected_outline.json");

    t.heading("How far it reaches");
    for (what, e, want) in [
        ("the defaults, width 3, grow the drawing by 3 on every side", outline("#ffffff", 3.0, 0.0, 100.0), 3),
        ("width 2.5 and softness 6 grow it by 3, the width rounded up, and 6, the blur's reach", outline("#ffffff", 2.5, 6.0, 100.0), 9),
        ("width 0 does not grow it, whatever the softness", outline("#ffffff", 0.0, 10.0, 100.0), 0),
    ] {
        let got = e.bounds_expansion();
        t.row(what, &got.to_string(), got == want);
    }
    let mut draft = outline("#ffffff", 4.0, 2.0, 100.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the width and the softness, and nothing else",
        &format!("{draft:?}"),
        draft == outline("#ffffff", 2.0, 1.0, 100.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_outline_001.json",
        "fx_outline_005.json",
        "fx_outline_006.json",
        "fx_outline_010.json",
        "fx_outline_011.json",
        "fx_outline_015.json",
        "fx_outline_016.json",
        "fx_outline_017.json",
        "fx_outline_018.json",
        "fx_outline_019.json",
        "fx_outline_020.json",
    ]);
    let colour = t.saved_parameters("fx_outline_009.json")["color"].clone();
    t.row(
        "fx_outline_009.json, its colour in capitals, is saved with it in small letters",
        &colour.to_string(),
        colour == "#c82828",
    );
    t.shape_refused(
        "fx_outline_001.json",
        "no `opacity` at all",
        r##"{"color": "#ffffff", "width": 3, "softness": 0}"##,
    );
    t.shape_refused(
        "fx_outline_001.json",
        "a width written as a word",
        r##"{"color": "#ffffff", "width": "three", "softness": 0, "opacity": 100}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_outline_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("width 101", set(outline("#ffffff", 101.0, 0.0, 100.0))),
            ("width -1", set(outline("#ffffff", -1.0, 0.0, 100.0))),
            ("softness 101", set(outline("#ffffff", 3.0, 101.0, 100.0))),
            ("opacity -1", set(outline("#ffffff", 3.0, 0.0, -1.0))),
            ("opacity 101", set(outline("#ffffff", 3.0, 0.0, 101.0))),
            ("colour \"white\"", set(outline("white", 3.0, 0.0, 100.0))),
            ("width keyed to 150", keys("width", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_outline_001.json",
        vec![
            ("width, softness and opacity 100, the tops,", set(outline("#c82828", 100.0, 100.0, 100.0))),
            ("width, softness and opacity 0, the bottoms,", set(outline("#ffffff", 0.0, 0.0, 0.0))),
            ("width keyed from 0 to 2", keys("width", &[(0, &[0.0]), (4, &[2.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_outline_006.json", 0), ("fx_outline_010.json", 2)]);

    t.finish("B-61_outline_table.md");
}
