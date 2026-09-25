//! B-39: radial blur in the core, against D-95.
//!
//! Writes `verification/B-39_radial_blur_table.md`.
//!
//! Every expected pixel is `Fixtures/radial_blur/expected_radial_blur.json`, written by
//! `tools/radial_blur_reference.py` before this code existed and printed in document 25 as
//! FX-RADIAL-001 to 018. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn radial(kind: &str, amount: f64, center: [f64; 2]) -> Effect {
    Effect::RadialBlur {
        kind: kind.to_string(),
        amount,
        center,
    }
}

#[test]
fn b39_radial_blur() {
    let mut t = Table::new(
        "radial_blur",
        "# B-39: radial blur\n\nD-95, accepted by the owner on 2026-09-25. Every expected pixel \
         is `Fixtures/radial_blur/expected_radial_blur.json`, written by \
         `tools/radial_blur_reference.py` before this code existed and printed in document 25 as \
         FX-RADIAL-001 to 018. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-RADIAL-001 to 018 (document 25)");
    t.fixtures("expected_radial_blur.json");

    t.heading("How far it reaches");
    let got = radial("spin", 100.0, [-1000.0, 1000.0]).bounds_expansion();
    t.row(
        "the most spin, far off the drawing, does not grow its bounds",
        &got.to_string(),
        got == 0,
    );
    let mut draft = radial("zoom", 40.0, [20.0, 70.0]);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: the amount and the centre are shares",
        &format!("{draft:?}"),
        draft == radial("zoom", 40.0, [20.0, 70.0]),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_radial_001.json",
        "fx_radial_002.json",
        "fx_radial_008.json",
        "fx_radial_009.json",
        "fx_radial_012.json",
        "fx_radial_013.json",
        "fx_radial_015.json",
        "fx_radial_016.json",
        "fx_radial_017.json",
        "fx_radial_018.json",
    ]);
    t.shape_refused(
        "fx_radial_001.json",
        "no `type` at all",
        r##"{"amount": 30, "center": [50, 50]}"##,
    );
    t.shape_refused(
        "fx_radial_001.json",
        "a centre of three numbers",
        r##"{"type": "spin", "amount": 30, "center": [50, 50, 50]}"##,
    );
    t.shape_refused(
        "fx_radial_001.json",
        "a keyed centre whose key holds one number",
        r##"{"type": "spin", "amount": 30, "center": {"base": [50, 50], "keyframes": [
            {"frame": 0, "value": [50, 50], "interp": "linear"},
            {"frame": 4, "value": 0, "interp": "linear"}]}}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_radial_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("amount 101", set(radial("spin", 101.0, [50.0, 50.0]))),
            ("amount -1", set(radial("spin", -1.0, [50.0, 50.0]))),
            (
                "centre -1001, 50",
                set(radial("spin", 30.0, [-1001.0, 50.0])),
            ),
            ("centre 50, 1001", set(radial("spin", 30.0, [50.0, 1001.0]))),
            ("type \"twist\"", set(radial("twist", 30.0, [50.0, 50.0]))),
            ("type \"Zoom\"", set(radial("Zoom", 30.0, [50.0, 50.0]))),
            (
                "amount keyed to 150",
                keys("amount", &[(0, &[0.0]), (4, &[150.0])]),
            ),
            (
                "centre keyed to 50, 2000",
                keys("center", &[(0, &[50.0, 50.0]), (4, &[50.0, 2000.0])]),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_radial_001.json",
        vec![
            (
                "zoom 100 about centre 1000, -1000, the tops of the ranges,",
                set(radial("zoom", 100.0, [1000.0, -1000.0])),
            ),
            (
                "spin 0 about centre -1000, 1000, the bottoms,",
                set(radial("spin", 0.0, [-1000.0, 1000.0])),
            ),
            (
                "centre keyed from 50, 50 to 0, 100",
                keys("center", &[(0, &[50.0, 50.0]), (4, &[0.0, 100.0])]),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_radial_004.json", 0), ("fx_radial_012.json", 0)]);

    t.finish("B-39_radial_blur_table.md");
}
