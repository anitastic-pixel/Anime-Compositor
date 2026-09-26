//! B-55: levels in the core, against D-112.
//!
//! Writes `verification/B-55_levels_table.md`.
//!
//! Every expected pixel is `Fixtures/levels/expected_levels.json`, written by
//! `tools/levels_reference.py` before this code existed and printed in document 25 as
//! FX-LEVELS-001 to 024. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn levels(input_black: f64, input_white: f64, gamma: f64, output_black: f64, output_white: f64) -> Effect {
    Effect::Levels {
        input_black,
        input_white,
        gamma,
        output_black,
        output_white,
    }
}

#[test]
fn b55_levels() {
    let mut t = Table::new(
        "levels",
        "# B-55: levels\n\nD-112, accepted by the owner on 2026-09-26 in the batch of ten. Every \
         expected pixel is `Fixtures/levels/expected_levels.json`, written by \
         `tools/levels_reference.py` before this code existed and printed in document 25 as \
         FX-LEVELS-001 to 024. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-LEVELS-001 to 024 (document 25)");
    t.fixtures("expected_levels.json");

    t.heading("How far it reaches");
    let all = levels(32.0, 224.0, 1.5, 16.0, 240.0);
    let got = all.bounds_expansion();
    t.row(
        "it grows the drawing's bounds by nothing: each pixel is regraded where it is",
        &got.to_string(),
        got == 0,
    );
    let mut draft = all.clone();
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: levels are colours, not distances",
        &format!("{draft:?}"),
        draft == all,
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_levels_008.json",
        "fx_levels_009.json",
        "fx_levels_011.json",
        "fx_levels_013.json",
        "fx_levels_014.json",
        "fx_levels_015.json",
        "fx_levels_018.json",
        "fx_levels_020.json",
        "fx_levels_024.json",
    ]);
    t.shape_refused(
        "fx_levels_001.json",
        "no `gamma` at all",
        r#"{"input_black": 0, "input_white": 255, "output_black": 0, "output_white": 255}"#,
    );
    t.shape_refused(
        "fx_levels_001.json",
        "an output white written as a word",
        r#"{"input_black": 0, "input_white": 255, "gamma": 1, "output_black": 0, "output_white": "white"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_levels_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("input black 256", set(levels(256.0, 255.0, 1.0, 0.0, 255.0))),
            ("input white -1", set(levels(0.0, -1.0, 1.0, 0.0, 255.0))),
            ("gamma 0.05", set(levels(0.0, 255.0, 0.05, 0.0, 255.0))),
            ("gamma 11", set(levels(0.0, 255.0, 11.0, 0.0, 255.0))),
            ("output black -1", set(levels(0.0, 255.0, 1.0, -1.0, 255.0))),
            ("output white 256", set(levels(0.0, 255.0, 1.0, 0.0, 256.0))),
            (
                "input black keyed to 300",
                keys("input_black", &[(0, &[0.0]), (4, &[300.0])]),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_levels_001.json",
        vec![
            (
                "input 255 to 0 and output 255 to 0 with gamma 10, the ends of the ranges,",
                set(levels(255.0, 0.0, 10.0, 255.0, 0.0)),
            ),
            (
                "input black and white both 120, a threshold,",
                set(levels(120.0, 120.0, 0.1, 0.0, 255.0)),
            ),
            (
                "gamma keyed from 1 to 3",
                keys("gamma", &[(0, &[1.0]), (4, &[3.0])]),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_levels_008.json", 0), ("fx_levels_017.json", 3)]);

    t.finish("B-55_levels_table.md");
}
