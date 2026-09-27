//! B-80: posterize in the core, against D-137.
//!
//! Writes `verification/B-80_posterize_table.md`.
//!
//! Every expected pixel is `Fixtures/posterize/expected_posterize.json`, written by
//! `tools/posterize_reference.py` before this code existed and printed in document 25 as
//! FX-POSTER-001 to 018. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn posterize(levels: f64) -> Effect {
    Effect::Posterize { levels }
}

#[test]
fn b80_posterize() {
    let mut t = Table::new(
        "posterize",
        "# B-80: posterize\n\nD-137, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the fourth of the third batch. Every expected pixel is \
         `Fixtures/posterize/expected_posterize.json`, written by `tools/posterize_reference.py` \
         before this code existed and printed in document 25 as FX-POSTER-001 to 018. The \
         build's frame is compared sample by sample; the answer is the largest difference over \
         all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-POSTER-001 to 018 (document 25)");
    t.fixtures("expected_posterize.json");

    t.heading("How far it reaches");
    let got = posterize(2.0).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = posterize(6.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: it has no distances",
        &format!("{draft:?}"),
        draft == posterize(6.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_poster_001.json",
        "fx_poster_005.json",
        "fx_poster_007.json",
        "fx_poster_008.json",
        "fx_poster_010.json",
        "fx_poster_011.json",
        "fx_poster_012.json",
        "fx_poster_013.json",
        "fx_poster_015.json",
        "fx_poster_018.json",
    ]);
    t.shape_refused("fx_poster_001.json", "no `levels` at all", r#"{}"#);
    t.shape_refused("fx_poster_001.json", "levels that is a word", r#"{"levels": "six"}"#);

    t.heading("Commands");
    let mut document = t.load("fx_poster_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("levels 1", set(posterize(1.0))),
            ("levels 257", set(posterize(257.0))),
            ("levels 256.5", set(posterize(256.5))),
            ("levels keyed to 300", keys("levels", &[(0, &[6.0]), (4, &[300.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_poster_001.json",
        vec![
            ("levels 2, the bottom,", set(posterize(2.0))),
            ("levels 256, the top,", set(posterize(256.0))),
            ("levels keyed from 2 to 10", keys("levels", &[(0, &[2.0]), (4, &[10.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_poster_001.json", 0), ("fx_poster_004.json", 0), ("fx_poster_008.json", 2)]);

    t.finish("B-80_posterize_table.md");
}
