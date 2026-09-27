//! B-89: find edges in the core, against D-146.
//!
//! Writes `verification/B-89_find_edges_table.md`.
//!
//! Every expected pixel is `Fixtures/find_edges/expected_find_edges.json`, written by
//! `tools/find_edges_reference.py` before this code existed and printed in document 25 as
//! FX-FINDEDGES-001 to 019. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn find_edges(invert: &str, amount: f64) -> Effect {
    Effect::FindEdges {
        invert: invert.to_string(),
        amount,
    }
}

#[test]
fn b89_find_edges() {
    let mut t = Table::new(
        "find_edges",
        "# B-89: find edges\n\nD-146, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the thirteenth of the third batch. Every expected pixel is \
         `Fixtures/find_edges/expected_find_edges.json`, written by \
         `tools/find_edges_reference.py` before this code existed and printed in document 25 as \
         FX-FINDEDGES-001 to 019. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-FINDEDGES-001 to 019 (document 25)");
    t.fixtures("expected_find_edges.json");

    t.heading("How far it reaches");
    let got = find_edges("on", 100.0).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = find_edges("off", 100.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: it has no distances",
        &format!("{draft:?}"),
        draft == find_edges("off", 100.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_findedges_001.json",
        "fx_findedges_003.json",
        "fx_findedges_007.json",
        "fx_findedges_009.json",
        "fx_findedges_015.json",
        "fx_findedges_016.json",
        "fx_findedges_017.json",
        "fx_findedges_018.json",
        "fx_findedges_019.json",
    ]);
    t.shape_refused("fx_findedges_001.json", "no `invert` at all", r#"{"amount": 100}"#);
    t.shape_refused(
        "fx_findedges_001.json",
        "an amount that is a word",
        r#"{"invert": "off", "amount": "all"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_findedges_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("amount 101", set(find_edges("off", 101.0))),
            ("amount -1", set(find_edges("off", -1.0))),
            ("invert \"yes\"", set(find_edges("yes", 100.0))),
            ("invert \"ON\"", set(find_edges("ON", 100.0))),
            ("amount keyed to 150", keys("amount", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_findedges_001.json",
        vec![
            ("amount 0, the bottom,", set(find_edges("on", 0.0))),
            ("amount 100, the top,", set(find_edges("off", 100.0))),
            ("amount keyed from 0 to 100", keys("amount", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_findedges_001.json", 0),
        ("fx_findedges_003.json", 0),
        ("fx_findedges_013.json", 3),
    ]);

    t.finish("B-89_find_edges_table.md");
}
