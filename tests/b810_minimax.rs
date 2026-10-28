//! B-810: D-930, after After Effects' Minimax (Channel): each chosen channel becomes the
//! largest or smallest over a square, row or column about the pixel, so a matte or a line grows
//! or shrinks by whole pixels.
//!
//! Writes `verification/B-810_minimax_table.md`.
//!
//! Every expected pixel is `Fixtures/minimax/expected_minimax.json`, written by
//! `tools/minimax_reference.py` (every window searched in full) before this code existed and
//! printed in document 25 as FX-MINIMAX-001 to 020. Tolerance 2e-5. Nothing here is a snapshot
//! of a run.

mod effect_table;

use effect_table::{changed, Table};

use anime_compositor::effects::Effect;

#[test]
fn b810_minimax() {
    let mut t = Table::new(
        "minimax",
        "# B-810: Minimax\n\nD-930, accepted on 2026-10-27 under the owner's request for the \
         tutorial follow-alongs. Every expected pixel is `Fixtures/minimax/expected_minimax.json`, \
         written by `tools/minimax_reference.py` before this code existed and printed in \
         document 25 as FX-MINIMAX-001 to 020. The reference searches every window in full; the \
         build runs a row pass then a column pass. The build's frame is compared sample by \
         sample; the answer is the largest difference over all of them, against the catalogue's \
         tolerance of 2e-5.\n",
    );

    t.heading("FX-MINIMAX-001 to 020 (document 25)");
    t.fixtures_numbered("expected_minimax.json", 1..=20);

    t.heading("How far it reaches");
    let e = changed(&t, "fx_minimax_002.json", |_| {});
    t.row(
        "it grows the drawing's bounds by nothing: the layer keeps its size",
        &e.bounds_expansion().to_string(),
        e.bounds_expansion() == 0,
    );
    let mut draft = changed(&t, "fx_minimax_002.json", |e| {
        if let Effect::Minimax { radius, .. } = e {
            *radius = 4.0;
        }
    });
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the radius, 4 to 2",
        &format!("{draft:?}"),
        matches!(draft, Effect::Minimax { radius, .. } if radius == 2.0),
    );

    t.heading("Wrong words are refused (what each warns)");
    for (file, want) in [
        ("fx_minimax_017.json", "operation"),
        ("fx_minimax_018.json", "channel"),
        ("fx_minimax_019.json", "direction"),
        ("fx_minimax_020.json", "don't shrink edges"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file}: the warning names the {want}"), &why, why.contains(want));
    }

    t.heading("The file");
    t.round_trips(&[
        "fx_minimax_001.json",
        "fx_minimax_002.json",
        "fx_minimax_005.json",
        "fx_minimax_009.json",
        "fx_minimax_012.json",
        "fx_minimax_014.json",
        "fx_minimax_017.json",
    ]);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_minimax_002.json", 0), ("fx_minimax_005.json", 0), ("fx_minimax_012.json", 2)]);

    t.finish("B-810_minimax_table.md");
}
