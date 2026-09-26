//! B-59: the lens blur in the core, against D-116.
//!
//! Writes `verification/B-59_lens_blur_table.md`.
//!
//! Every expected pixel is `Fixtures/lens_blur/expected_lens_blur.json`, written by
//! `tools/lens_blur_reference.py` before this code existed and printed in document 25 as
//! FX-LENS-001 to 018. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn lens(radius: f64, edges: &str) -> Effect {
    Effect::LensBlur {
        radius,
        edges: edges.into(),
    }
}

#[test]
fn b59_lens_blur() {
    let mut t = Table::new(
        "lens_blur",
        "# B-59: lens blur\n\nD-116, accepted by the owner on 2026-09-26 in the batch of ten. \
         Every expected pixel is `Fixtures/lens_blur/expected_lens_blur.json`, written by \
         `tools/lens_blur_reference.py` before this code existed and printed in document 25 as \
         FX-LENS-001 to 018. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-LENS-001 to 018 (document 25)");
    t.fixtures("expected_lens_blur.json");

    t.heading("How far it reaches");
    for (what, e, want) in [
        ("radius 10 grows the drawing by 10 on every side", lens(10.0, "transparent"), 10),
        ("radius 2.5 grows it by 3, the radius rounded up", lens(2.5, "transparent"), 3),
        ("radius 0.9 grows it by 1, though it changes no pixel", lens(0.9, "transparent"), 1),
        ("radius 0 does not grow it", lens(0.0, "transparent"), 0),
        ("radius 10 with the edge pixels repeated does not grow it", lens(10.0, "repeat"), 0),
    ] {
        let got = e.bounds_expansion();
        t.row(what, &got.to_string(), got == want);
    }
    let mut draft = lens(10.0, "repeat");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the radius, and keeps the edges",
        &format!("{draft:?}"),
        draft == lens(5.0, "repeat"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_lens_001.json",
        "fx_lens_003.json",
        "fx_lens_008.json",
        "fx_lens_010.json",
        "fx_lens_011.json",
        "fx_lens_014.json",
        "fx_lens_015.json",
        "fx_lens_016.json",
        "fx_lens_017.json",
        "fx_lens_018.json",
    ]);
    t.shape_refused("fx_lens_001.json", "no `edges` at all", r#"{"radius": 10}"#);
    t.shape_refused(
        "fx_lens_001.json",
        "a radius written as a word",
        r#"{"radius": "ten", "edges": "transparent"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_lens_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("radius 201", set(lens(201.0, "transparent"))),
            ("radius -1", set(lens(-1.0, "transparent"))),
            ("edges \"wrap\"", set(lens(10.0, "wrap"))),
            ("edges \"Repeat\"", set(lens(10.0, "Repeat"))),
            ("radius keyed to 250", keys("radius", &[(0, &[0.0]), (4, &[250.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_lens_001.json",
        vec![
            ("radius 200, the top,", set(lens(200.0, "transparent"))),
            ("radius 0, edges repeat", set(lens(0.0, "repeat"))),
            ("radius keyed from 0 to 4", keys("radius", &[(0, &[0.0]), (4, &[4.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_lens_008.json", 0), ("fx_lens_010.json", 2)]);

    t.finish("B-59_lens_blur_table.md");
}
