//! B-66: the distance gradation in the core, against D-123.
//!
//! Writes `verification/B-66_distance_gradation_table.md`.
//!
//! Every expected pixel is `Fixtures/distance_gradation/expected_distance_gradation.json`,
//! written by `tools/distance_gradation_reference.py` before this code existed and printed in
//! document 25 as FX-DISTGRAD-001 to 024. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn shade(color: &str, width: f64, opacity: f64, invert: &str, blend: &str) -> Effect {
    Effect::DistanceGradation {
        color: color.to_string(),
        width,
        opacity,
        invert: invert.to_string(),
        blend: blend.to_string(),
    }
}

#[test]
fn b66_distance_gradation() {
    let mut t = Table::new(
        "distance_gradation",
        "# B-66: distance gradation\n\nD-123, accepted by the owner on 2026-09-26, the first of \
         the second batch of ten. Every expected pixel is \
         `Fixtures/distance_gradation/expected_distance_gradation.json`, written by \
         `tools/distance_gradation_reference.py` before this code existed and printed in \
         document 25 as FX-DISTGRAD-001 to 024. The build's frame is compared sample by sample; \
         the answer is the largest difference over all of them, against the catalogue's \
         tolerance of 2e-5.\n",
    );

    t.heading("FX-DISTGRAD-001 to 024 (document 25)");
    t.fixtures("expected_distance_gradation.json");

    t.heading("How far it reaches");
    let got = shade("#6450a0", 1000.0, 100.0, "on", "add").bounds_expansion();
    t.row(
        "it grows the drawing's bounds by nothing: the shading stays on the covering",
        &got.to_string(),
        got == 0,
    );
    let mut draft = shade("#6450a0", 12.0, 50.0, "off", "multiply");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the width, a distance, and leaves the opacity",
        &format!("{draft:?}"),
        draft == shade("#6450a0", 6.0, 50.0, "off", "multiply"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_distgrad_001.json",
        "fx_distgrad_008.json",
        "fx_distgrad_011.json",
        "fx_distgrad_013.json",
        "fx_distgrad_015.json",
        "fx_distgrad_016.json",
        "fx_distgrad_017.json",
        "fx_distgrad_018.json",
        "fx_distgrad_021.json",
        "fx_distgrad_022.json",
        "fx_distgrad_023.json",
        "fx_distgrad_024.json",
    ]);
    t.shape_refused(
        "fx_distgrad_001.json",
        "no `width` at all",
        r##"{"color": "#6450a0", "opacity": 50, "invert": "off", "blend": "multiply"}"##,
    );
    t.shape_refused(
        "fx_distgrad_001.json",
        "invert written as true rather than a word",
        r##"{"color": "#6450a0", "width": 10, "opacity": 50, "invert": true, "blend": "multiply"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_distgrad_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("width 1001", set(shade("#6450a0", 1001.0, 50.0, "off", "multiply"))),
            ("width -1", set(shade("#6450a0", -1.0, 50.0, "off", "multiply"))),
            ("opacity 101", set(shade("#6450a0", 10.0, 101.0, "off", "multiply"))),
            ("invert \"yes\"", set(shade("#6450a0", 10.0, 50.0, "yes", "multiply"))),
            ("blend \"overlay\"", set(shade("#6450a0", 10.0, 50.0, "off", "overlay"))),
            ("colour \"#12345\"", set(shade("#12345", 10.0, 50.0, "off", "multiply"))),
            ("opacity keyed to 150", keys("opacity", &[(0, &[50.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_distgrad_001.json",
        vec![
            ("width 1000 and opacity 100, the tops,", set(shade("#6450a0", 1000.0, 100.0, "off", "multiply"))),
            ("width 0, opacity 0, invert on and screen", set(shade("#ffffff", 0.0, 0.0, "on", "screen"))),
            ("width keyed from 0 to 4", keys("width", &[(0, &[0.0]), (4, &[4.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_distgrad_001.json", 0), ("fx_distgrad_017.json", 3)]);

    t.finish("B-66_distance_gradation_table.md");
}
