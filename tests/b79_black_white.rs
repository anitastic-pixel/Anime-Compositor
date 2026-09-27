//! B-79: black and white in the core, against D-136.
//!
//! Writes `verification/B-79_black_white_table.md`.
//!
//! Every expected pixel is `Fixtures/black_white/expected_black_white.json`, written by
//! `tools/black_white_reference.py` before this code existed and printed in document 25 as
//! FX-BW-001 to 022. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn bw(w: [f64; 6]) -> Effect {
    Effect::BlackWhite {
        reds: w[0],
        yellows: w[1],
        greens: w[2],
        cyans: w[3],
        blues: w[4],
        magentas: w[5],
    }
}

const START: [f64; 6] = [40.0, 60.0, 40.0, 60.0, 20.0, 80.0];

fn with(i: usize, v: f64) -> Effect {
    let mut w = START;
    w[i] = v;
    bw(w)
}

#[test]
fn b79_black_white() {
    let mut t = Table::new(
        "black_white",
        "# B-79: black and white\n\nD-136, accepted on 2026-09-26 by the owner's message asking \
         for thirty more effects, the third of the third batch. Every expected pixel is \
         `Fixtures/black_white/expected_black_white.json`, written by \
         `tools/black_white_reference.py` before this code existed and printed in document 25 \
         as FX-BW-001 to 022. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-BW-001 to 022 (document 25)");
    t.fixtures("expected_black_white.json");

    t.heading("How far it reaches");
    let got = bw(START).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = bw(START);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: it has no distances",
        &format!("{draft:?}"),
        draft == bw(START),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_bw_001.json",
        "fx_bw_004.json",
        "fx_bw_011.json",
        "fx_bw_012.json",
        "fx_bw_013.json",
        "fx_bw_014.json",
        "fx_bw_016.json",
        "fx_bw_017.json",
        "fx_bw_020.json",
        "fx_bw_022.json",
    ]);
    t.shape_refused(
        "fx_bw_001.json",
        "no `magentas` at all",
        r#"{"reds": 40, "yellows": 60, "greens": 40, "cyans": 60, "blues": 20}"#,
    );
    t.shape_refused(
        "fx_bw_001.json",
        "a reds that is a word",
        r#"{"reds": "lots", "yellows": 60, "greens": 40, "cyans": 60, "blues": 20, "magentas": 80}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_bw_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("reds 301", set(with(0, 301.0))),
            ("yellows -201", set(with(1, -201.0))),
            ("blues 1000", set(with(4, 1000.0))),
            ("magentas keyed to -250", keys("magentas", &[(0, &[80.0]), (4, &[-250.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_bw_001.json",
        vec![
            ("all six at -200, the bottom,", set(bw([-200.0; 6]))),
            ("all six at 300, the top,", set(bw([300.0; 6]))),
            ("cyans keyed from 60 to -200", keys("cyans", &[(0, &[60.0]), (4, &[-200.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_bw_001.json", 0), ("fx_bw_011.json", 0), ("fx_bw_012.json", 2)]);

    t.finish("B-79_black_white_table.md");
}
