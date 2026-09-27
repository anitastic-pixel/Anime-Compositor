//! B-63: the chromatic aberration in the core, against D-120.
//!
//! Writes `verification/B-63_chromatic_aberration_table.md`.
//!
//! Every expected pixel is `Fixtures/chromatic_aberration/expected_chromatic_aberration.json`,
//! written by `tools/chromatic_aberration_reference.py` before this code existed and printed in
//! document 25 as FX-CHROMA-001 to 016. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn split(amount: f64, center: [f64; 2]) -> Effect {
    Effect::ChromaticAberration { amount, center }
}

#[test]
fn b63_chromatic_aberration() {
    let mut t = Table::new(
        "chromatic_aberration",
        "# B-63: chromatic aberration\n\nD-120, accepted by the owner on 2026-09-26 in the batch \
         of ten. Every expected pixel is \
         `Fixtures/chromatic_aberration/expected_chromatic_aberration.json`, written by \
         `tools/chromatic_aberration_reference.py` before this code existed and printed in \
         document 25 as FX-CHROMA-001 to 016. The build's frame is compared sample by sample; \
         the answer is the largest difference over all of them, against the catalogue's \
         tolerance of 2e-5.\n",
    );

    t.heading("FX-CHROMA-001 to 016 (document 25)");
    t.fixtures("expected_chromatic_aberration.json");

    t.heading("How far it reaches");
    let e = split(100.0, [0.0, 50.0]);
    let got = e.bounds_expansion();
    t.row(
        "it grows the drawing's bounds by nothing: a fringe past the layer's edge is cut",
        &got.to_string(),
        got == 0,
    );
    let mut draft = split(6.0, [30.0, 70.0]);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the amount, a distance, and leaves the centre, a share \
         of the drawing",
        &format!("{draft:?}"),
        draft == split(3.0, [30.0, 70.0]),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_chroma_001.json",
        "fx_chroma_004.json",
        "fx_chroma_005.json",
        "fx_chroma_006.json",
        "fx_chroma_007.json",
        "fx_chroma_009.json",
        "fx_chroma_012.json",
        "fx_chroma_013.json",
        "fx_chroma_014.json",
        "fx_chroma_015.json",
        "fx_chroma_016.json",
    ]);
    t.shape_refused(
        "fx_chroma_001.json",
        "no `center` at all",
        r##"{"amount": 3}"##,
    );
    t.shape_refused(
        "fx_chroma_001.json",
        "a centre of one number",
        r##"{"amount": 3, "center": 50}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_chroma_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("amount 101", set(split(101.0, [50.0, 50.0]))),
            ("amount -1", set(split(-1.0, [50.0, 50.0]))),
            ("centre 1001, 50", set(split(3.0, [1001.0, 50.0]))),
            ("centre 50, -1001", set(split(3.0, [50.0, -1001.0]))),
            ("amount keyed to 150", keys("amount", &[(0, &[3.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_chroma_001.json",
        vec![
            ("amount 100 and centre 1000, 1000, the tops,", set(split(100.0, [1000.0, 1000.0]))),
            ("amount 0 and centre -1000, -1000, the bottoms,", set(split(0.0, [-1000.0, -1000.0]))),
            ("centre keyed from 50, 50 to 0, 50", keys("center", &[(0, &[50.0, 50.0]), (4, &[0.0, 50.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_chroma_001.json", 0), ("fx_chroma_007.json", 3)]);

    t.finish("B-63_chromatic_aberration_table.md");
}
