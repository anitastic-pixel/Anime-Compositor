//! B-37: select colour in the core, against D-93.
//!
//! Writes `verification/B-37_select_color_table.md`.
//!
//! Every expected pixel is `Fixtures/select_color/expected_select_color.json`, written by
//! `tools/select_color_reference.py` before this code existed and printed in document 25 as
//! FX-SELECT-001 to 016. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, strings, Table, NINE};

use anime_compositor::effects::Effect;

fn select(colors: &[&str], tolerance: f64, keep: &str) -> Effect {
    Effect::SelectColor {
        colors: strings(colors),
        tolerance,
        keep: keep.to_string(),
    }
}

const LINE: &[&str] = &["#1e1a24"];

#[test]
fn b37_select_color() {
    let mut t = Table::new(
        "select_color",
        "# B-37: select colour\n\nD-93, accepted by the owner on 2026-09-25. Every expected \
         pixel is `Fixtures/select_color/expected_select_color.json`, written by \
         `tools/select_color_reference.py` before this code existed and printed in document 25 \
         as FX-SELECT-001 to 016. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-SELECT-001 to 016 (document 25)");
    t.fixtures("expected_select_color.json");

    t.heading("How far it reaches");
    let got = select(LINE, 255.0, "others").bounds_expansion();
    t.row(
        "it grows the drawing's bounds by nothing: each pixel is kept or cleared where it is",
        &got.to_string(),
        got == 0,
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_select_001.json",
        "fx_select_002.json",
        "fx_select_007.json",
        "fx_select_010.json",
        "fx_select_012.json",
        "fx_select_013.json",
        "fx_select_014.json",
        "fx_select_015.json",
        "fx_select_016.json",
    ]);
    let params = t.saved_parameters("fx_select_006.json");
    t.row(
        "fx_select_006.json, its colour written in capitals, is saved in small letters",
        &params["colors"].to_string(),
        params["colors"] == serde_json::json!(["#1e1a24"]),
    );
    let params = t.saved_parameters("fx_select_016.json");
    t.row(
        "fx_select_016.json's keep \"Chosen\" is saved as written, not corrected",
        &params["keep"].to_string(),
        params["keep"] == serde_json::json!("Chosen"),
    );
    t.shape_refused(
        "fx_select_001.json",
        "no `keep` at all",
        r##"{"colors": ["#1e1a24"], "tolerance": 0}"##,
    );
    t.shape_refused(
        "fx_select_001.json",
        "a keep that is a number",
        r##"{"colors": ["#1e1a24"], "tolerance": 0, "keep": 1}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_select_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("tolerance 256", set(select(LINE, 256.0, "chosen"))),
            ("tolerance -1", set(select(LINE, -1.0, "chosen"))),
            ("nine colours", set(select(NINE, 0.0, "chosen"))),
            (
                "the colour \"#12345\"",
                set(select(&["#12345"], 0.0, "chosen")),
            ),
            ("keep \"both\"", set(select(LINE, 0.0, "both"))),
            ("keep \"Others\"", set(select(LINE, 0.0, "Others"))),
            (
                "tolerance keyed to 300",
                keys("tolerance", &[(0, &[0.0]), (4, &[300.0])]),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_select_001.json",
        vec![
            (
                "tolerance 255 with eight colours, keep others, the top of the ranges,",
                set(select(&NINE[..8], 255.0, "others")),
            ),
            (
                "tolerance 0 with no colour, the bottom,",
                set(select(&[], 0.0, "chosen")),
            ),
            (
                "tolerance keyed from 0 to 20",
                keys("tolerance", &[(0, &[0.0]), (4, &[20.0])]),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_select_002.json", 0), ("fx_select_008.json", 3)]);

    t.finish("B-37_select_color_table.md");
}
