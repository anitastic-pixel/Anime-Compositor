//! B-35: line recolour in the core, against D-91.
//!
//! Writes `verification/B-35_line_recolor_table.md`.
//!
//! Every expected pixel is `Fixtures/recolor/expected_recolor.json`, written by
//! `tools/recolor_reference.py` before this code existed and printed in document 25 as
//! FX-RECOLOR-001 to 018. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, strings, Table, NINE};

use anime_compositor::effects::Effect;

fn recolor(colors: &[&str], tolerance: f64, new_color: &str) -> Effect {
    Effect::LineRecolor {
        colors: strings(colors),
        tolerance,
        new_color: new_color.to_string(),
    }
}

const LINE: &[&str] = &["#1e1a24"];

#[test]
fn b35_line_recolor() {
    let mut t = Table::new(
        "recolor",
        "# B-35: line recolour\n\nD-91, accepted by the owner on 2026-09-25. Every expected \
         pixel is `Fixtures/recolor/expected_recolor.json`, written by \
         `tools/recolor_reference.py` before this code existed and printed in document 25 as \
         FX-RECOLOR-001 to 018. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-RECOLOR-001 to 018 (document 25)");
    t.fixtures("expected_recolor.json");

    t.heading("How far it reaches");
    let got = recolor(LINE, 255.0, "#ff0000").bounds_expansion();
    t.row(
        "it grows the drawing's bounds by nothing: each pixel is recoloured where it is",
        &got.to_string(),
        got == 0,
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_recolor_001.json",
        "fx_recolor_005.json",
        "fx_recolor_007.json",
        "fx_recolor_012.json",
        "fx_recolor_014.json",
        "fx_recolor_015.json",
        "fx_recolor_016.json",
        "fx_recolor_017.json",
        "fx_recolor_018.json",
    ]);
    let params = t.saved_parameters("fx_recolor_006.json");
    t.row(
        "fx_recolor_006.json, written in capitals, is saved in small letters",
        &format!("{} and {}", params["colors"], params["new_color"]),
        params["colors"] == serde_json::json!(["#1e1a24"])
            && params["new_color"] == serde_json::json!("#ff0000"),
    );
    t.shape_refused(
        "fx_recolor_001.json",
        "no `new_color` at all",
        r##"{"colors": ["#1e1a24"], "tolerance": 0}"##,
    );
    t.shape_refused(
        "fx_recolor_001.json",
        "a new colour that is a number",
        r##"{"colors": ["#1e1a24"], "tolerance": 0, "new_color": 16711680}"##,
    );
    t.shape_refused(
        "fx_recolor_001.json",
        "a tolerance that is a word",
        r##"{"colors": ["#1e1a24"], "tolerance": "some", "new_color": "#ff0000"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_recolor_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("tolerance 256", set(recolor(LINE, 256.0, "#ff0000"))),
            ("tolerance -1", set(recolor(LINE, -1.0, "#ff0000"))),
            ("nine colours", set(recolor(NINE, 0.0, "#ff0000"))),
            (
                "the colour \"#12345\"",
                set(recolor(&["#12345"], 0.0, "#ff0000")),
            ),
            ("the new colour \"red\"", set(recolor(LINE, 0.0, "red"))),
            ("an empty new colour", set(recolor(LINE, 0.0, ""))),
            (
                "tolerance keyed to 300",
                keys("tolerance", &[(0, &[0.0]), (4, &[300.0])]),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_recolor_001.json",
        vec![
            (
                "tolerance 255 with eight colours, the top of the ranges,",
                set(recolor(&NINE[..8], 255.0, "#3060ff")),
            ),
            (
                "tolerance 0 with no colour, the bottom,",
                set(recolor(&[], 0.0, "#000000")),
            ),
            (
                "the new colour in capitals \"#FF4000\"",
                set(recolor(LINE, 0.0, "#FF4000")),
            ),
            (
                "tolerance keyed from 0 to 20",
                keys("tolerance", &[(0, &[0.0]), (4, &[20.0])]),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_recolor_005.json", 0), ("fx_recolor_008.json", 3)]);

    t.finish("B-35_line_recolor_table.md");
}
