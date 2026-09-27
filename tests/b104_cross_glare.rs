//! B-104: cross glare in the core, against D-161.
//!
//! Writes `verification/B-104_cross_glare_table.md`.
//!
//! Every expected pixel is `Fixtures/cross_glare/expected_cross_glare.json`, written by
//! `tools/cross_glare_reference.py` before this code existed and printed in document 25 as
//! FX-GLARE-001 to 029. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

/// `[threshold, length, points, angle, intensity]`.
fn glare(n: [f64; 5], color: &str) -> Effect {
    Effect::CrossGlare {
        threshold: n[0],
        length: n[1],
        points: n[2],
        angle: n[3],
        intensity: n[4],
        color: color.to_string(),
    }
}

const START: [f64; 5] = [80.0, 40.0, 4.0, 45.0, 1.0];

fn with(i: usize, v: f64) -> Effect {
    let mut n = START;
    n[i] = v;
    glare(n, "#ffffff")
}

#[test]
fn b104_cross_glare() {
    let mut t = Table::new(
        "cross_glare",
        "# B-104: cross glare\n\nD-161, accepted on 2026-09-26 by the owner's message asking for \
         thirty more effects, the twenty-eighth of the third batch. Every expected pixel is \
         `Fixtures/cross_glare/expected_cross_glare.json`, written by \
         `tools/cross_glare_reference.py` before this code existed and printed in document 25 as \
         FX-GLARE-001 to 029. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-GLARE-001 to 029 (document 25)");
    t.fixtures("expected_cross_glare.json");

    t.heading("How far it reaches");
    let got = with(1, 40.7).bounds_expansion();
    t.row("length 40.7 grows the drawing's bounds by 40 pixels", &got.to_string(), got == 40);
    let got = with(1, 0.9).bounds_expansion();
    t.row("length 0.9, taken down to 0, grows them by nothing", &got.to_string(), got == 0);
    let got = with(4, 0.0).bounds_expansion();
    t.row("intensity 0 grows them by nothing", &got.to_string(), got == 0);
    let mut draft = with(1, 40.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the length, 40 to 20, and nothing else",
        &format!("{draft:?}"),
        draft == with(1, 20.0),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_glare_001.json",
        "fx_glare_005.json",
        "fx_glare_015.json",
        "fx_glare_018.json",
        "fx_glare_019.json",
        "fx_glare_020.json",
        "fx_glare_022.json",
        "fx_glare_023.json",
        "fx_glare_024.json",
        "fx_glare_025.json",
        "fx_glare_026.json",
        "fx_glare_027.json",
        "fx_glare_028.json",
        "fx_glare_029.json",
    ]);
    t.shape_refused(
        "fx_glare_001.json",
        "no `points` at all",
        r##"{"threshold": 80, "length": 40, "angle": 45, "intensity": 1, "color": "#ffffff"}"##,
    );
    t.shape_refused(
        "fx_glare_001.json",
        "a length that is a word",
        r##"{"threshold": 80, "length": "long", "points": 4, "angle": 45, "intensity": 1,
            "color": "#ffffff"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_glare_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("threshold 101", set(with(0, 101.0))),
            ("length 1001", set(with(1, 1001.0))),
            ("points 0", set(with(2, 0.0))),
            ("points 9", set(with(2, 9.0))),
            ("angle -3601", set(with(3, -3601.0))),
            ("intensity 11", set(with(4, 11.0))),
            ("colour \"#12345\"", set(glare(START, "#12345"))),
            ("colour \"white\"", set(glare(START, "white"))),
            ("intensity keyed to 11", keys("intensity", &[(0, &[1.0]), (4, &[11.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_glare_001.json",
        vec![
            ("every number at its bottom,", set(glare([0.0, 0.0, 1.0, -3600.0, 0.0], "#000000"))),
            ("every number at its top,", set(glare([100.0, 1000.0, 8.0, 3600.0, 10.0], "#ffffff"))),
            ("length keyed from 0 to 8", keys("length", &[(0, &[0.0]), (4, &[8.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_glare_005.json", 0),
        ("fx_glare_011.json", 0),
        ("fx_glare_018.json", 4),
    ]);

    t.finish("B-104_cross_glare_table.md");
}
