//! B-72: the gradient map in the core, against D-129.
//!
//! Writes `verification/B-72_gradient_map_table.md`.
//!
//! Every expected pixel is `Fixtures/gradient_map/expected_gradient_map.json`, written by
//! `tools/gradient_map_reference.py` before this code existed and printed in document 25 as
//! FX-GRADMAP-001 to 023. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn map(colors: [&str; 3], midpoint: f64, amount: f64) -> Effect {
    Effect::GradientMap {
        shadow_color: colors[0].to_string(),
        midtone_color: colors[1].to_string(),
        highlight_color: colors[2].to_string(),
        midpoint,
        amount,
    }
}

const GREY: [&str; 3] = ["#000000", "#808080", "#ffffff"];

#[test]
fn b72_gradient_map() {
    let mut t = Table::new(
        "gradient_map",
        "# B-72: gradient map\n\nD-129, accepted by the owner on 2026-09-26, the seventh of the \
         second batch of ten. Every expected pixel is \
         `Fixtures/gradient_map/expected_gradient_map.json`, written by \
         `tools/gradient_map_reference.py` before this code existed and printed in document 25 \
         as FX-GRADMAP-001 to 023. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-GRADMAP-001 to 023 (document 25)");
    t.fixtures("expected_gradient_map.json");

    t.heading("How far it reaches");
    let got = map(GREY, 50.0, 100.0).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = map(GREY, 40.0, 80.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: it has no distances",
        &format!("{draft:?}"),
        draft == map(GREY, 40.0, 80.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_gradmap_001.json",
        "fx_gradmap_004.json",
        "fx_gradmap_007.json",
        "fx_gradmap_010.json",
        "fx_gradmap_012.json",
        "fx_gradmap_013.json",
        "fx_gradmap_014.json",
        "fx_gradmap_016.json",
        "fx_gradmap_017.json",
        "fx_gradmap_021.json",
        "fx_gradmap_022.json",
        "fx_gradmap_023.json",
    ]);
    t.shape_refused(
        "fx_gradmap_001.json",
        "no `midtone_color` at all",
        r##"{"shadow_color": "#000000", "highlight_color": "#ffffff", "midpoint": 50, "amount": 100}"##,
    );
    t.shape_refused(
        "fx_gradmap_001.json",
        "a midpoint that is a word",
        r##"{"shadow_color": "#000000", "midtone_color": "#808080", "highlight_color": "#ffffff", "midpoint": "half", "amount": 100}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_gradmap_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("midpoint 0", set(map(GREY, 0.0, 100.0))),
            ("midpoint 100", set(map(GREY, 100.0, 100.0))),
            ("amount 101", set(map(GREY, 50.0, 101.0))),
            ("amount -1", set(map(GREY, 50.0, -1.0))),
            ("shadow colour \"#12345\"", set(map(["#12345", "#808080", "#ffffff"], 50.0, 100.0))),
            ("midtone colour \"grey\"", set(map(["#000000", "grey", "#ffffff"], 50.0, 100.0))),
            ("highlight colour \"white\"", set(map(["#000000", "#808080", "white"], 50.0, 100.0))),
            ("midpoint keyed to 150", keys("midpoint", &[(0, &[50.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_gradmap_001.json",
        vec![
            (
                "midpoint 99 and amount 100, the tops,",
                set(map(["#2a1650", "#c85a50", "#ffe6b4"], 99.0, 100.0)),
            ),
            ("midpoint 1 and amount 0, the bottoms,", set(map(GREY, 1.0, 0.0))),
            ("amount keyed from 0 to 100", keys("amount", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_gradmap_004.json", 0), ("fx_gradmap_013.json", 2)]);

    t.finish("B-72_gradient_map_table.md");
}
