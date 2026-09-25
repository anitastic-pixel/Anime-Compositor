//! B-41: colour key in the core, against D-97.
//!
//! Writes `verification/B-41_color_key_table.md`.
//!
//! Every expected pixel is `Fixtures/color_key/expected_color_key.json`, written by
//! `tools/color_key_reference.py` before this code existed and printed in document 25 as
//! FX-KEY-001 to 023. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, strings, Table, NINE};

use anime_compositor::effects::Effect;

fn key(colors: &[&str], tolerance: f64, softness: f64, match_by: &str) -> Effect {
    Effect::ColorKey {
        colors: strings(colors),
        tolerance,
        softness,
        match_by: match_by.to_string(),
    }
}

const GREEN: &[&str] = &["#00b140"];

#[test]
fn b41_color_key() {
    let mut t = Table::new(
        "color_key",
        "# B-41: colour key\n\nD-97, accepted by the owner on 2026-09-25. Every expected pixel \
         is `Fixtures/color_key/expected_color_key.json`, written by \
         `tools/color_key_reference.py` before this code existed and printed in document 25 as \
         FX-KEY-001 to 023. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-KEY-001 to 023 (document 25)");
    t.fixtures("expected_color_key.json");

    t.heading("How far it reaches");
    let got = key(GREEN, 255.0, 255.0, "hue").bounds_expansion();
    t.row(
        "it grows the drawing's bounds by nothing: each pixel is kept, faded or cleared where it is",
        &got.to_string(),
        got == 0,
    );
    let mut draft = key(GREEN, 20.0, 40.0, "rgb");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: tolerance and softness are colour distances",
        &format!("{draft:?}"),
        draft == key(GREEN, 20.0, 40.0, "rgb"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_key_003.json",
        "fx_key_005.json",
        "fx_key_008.json",
        "fx_key_009.json",
        "fx_key_016.json",
        "fx_key_019.json",
        "fx_key_020.json",
        "fx_key_021.json",
        "fx_key_022.json",
        "fx_key_023.json",
    ]);
    let params = t.saved_parameters("fx_key_011.json");
    t.row(
        "fx_key_011.json, its colour written in capitals, is saved in small letters",
        &params["colors"].to_string(),
        params["colors"] == serde_json::json!(["#00b140"]),
    );
    let params = t.saved_parameters("fx_key_023.json");
    t.row(
        "fx_key_023.json's match \"RGB\" is saved as written, not corrected",
        &params["match"].to_string(),
        params["match"] == serde_json::json!("RGB"),
    );
    t.shape_refused(
        "fx_key_001.json",
        "no `match` at all",
        r##"{"colors": ["#00b140"], "tolerance": 0, "softness": 0}"##,
    );
    t.shape_refused(
        "fx_key_001.json",
        "a softness written as a word",
        r##"{"colors": ["#00b140"], "tolerance": 0, "softness": "soft", "match": "rgb"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_key_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("tolerance 256", set(key(GREEN, 256.0, 0.0, "rgb"))),
            ("tolerance -1", set(key(GREEN, -1.0, 0.0, "rgb"))),
            ("softness 256", set(key(GREEN, 0.0, 256.0, "rgb"))),
            ("softness -1", set(key(GREEN, 0.0, -1.0, "rgb"))),
            ("nine colours", set(key(NINE, 0.0, 0.0, "rgb"))),
            (
                "the colour \"#12345\"",
                set(key(&["#12345"], 0.0, 0.0, "rgb")),
            ),
            ("match \"hsv\"", set(key(GREEN, 0.0, 0.0, "hsv"))),
            ("match \"Hue\"", set(key(GREEN, 0.0, 0.0, "Hue"))),
            (
                "softness keyed to 300",
                keys("softness", &[(0, &[0.0]), (4, &[300.0])]),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_key_001.json",
        vec![
            (
                "tolerance and softness 255 with eight colours, match hue, the top of the ranges,",
                set(key(&NINE[..8], 255.0, 255.0, "hue")),
            ),
            (
                "tolerance and softness 0 with no colour, the bottom,",
                set(key(&[], 0.0, 0.0, "rgb")),
            ),
            (
                "tolerance keyed from 0 to 100",
                keys("tolerance", &[(0, &[0.0]), (4, &[100.0])]),
            ),
            (
                "softness keyed from 0 to 100",
                keys("softness", &[(0, &[0.0]), (4, &[100.0])]),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_key_005.json", 0), ("fx_key_015.json", 3)]);

    t.finish("B-41_color_key_table.md");
}
