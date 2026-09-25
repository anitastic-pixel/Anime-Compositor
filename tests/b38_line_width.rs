//! B-38: line width in the core, against D-94.
//!
//! Writes `verification/B-38_line_width_table.md`.
//!
//! Every expected pixel is `Fixtures/line_width/expected_line_width.json`, written by
//! `tools/line_width_reference.py` before this code existed and printed in document 25 as
//! FX-WIDTH-001 to 019. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, strings, Table, NINE};

use anime_compositor::effects::Effect;

fn width(width: f64, based_on: &str, colors: &[&str]) -> Effect {
    Effect::LineWidth {
        width,
        based_on: based_on.to_string(),
        colors: strings(colors),
        tolerance: 0.0,
    }
}

const LINE: &[&str] = &["#1e1a24"];

#[test]
fn b38_line_width() {
    let mut t = Table::new(
        "line_width",
        "# B-38: line width\n\nD-94, accepted by the owner on 2026-09-25. Every expected pixel \
         is `Fixtures/line_width/expected_line_width.json`, written by \
         `tools/line_width_reference.py` before this code existed and printed in document 25 as \
         FX-WIDTH-001 to 019. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-WIDTH-001 to 019 (document 25)");
    t.fixtures("expected_line_width.json");

    t.heading("How far it reaches");
    for (w, want) in [(0.0, 0), (1.0, 1), (1.5, 2), (20.0, 20), (-5.0, 0)] {
        let got = width(w, "shape", &[]).bounds_expansion();
        t.row(
            &format!("width {w} grows the drawing's bounds by {want}"),
            &got.to_string(),
            got == want,
        );
    }
    let mut draft = width(-6.0, "colors", LINE);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the width, thinning included, and keeps the rest",
        &format!("{draft:?}"),
        draft == width(-3.0, "colors", LINE),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_width_001.json",
        "fx_width_005.json",
        "fx_width_008.json",
        "fx_width_013.json",
        "fx_width_014.json",
        "fx_width_015.json",
        "fx_width_016.json",
        "fx_width_017.json",
        "fx_width_018.json",
        "fx_width_019.json",
    ]);
    t.shape_refused(
        "fx_width_001.json",
        "no `based_on` at all",
        r##"{"width": 1, "colors": [], "tolerance": 0}"##,
    );
    t.shape_refused(
        "fx_width_001.json",
        "a width that is a word",
        r##"{"width": "thick", "based_on": "shape", "colors": [], "tolerance": 0}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_width_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("width 21", set(width(21.0, "shape", LINE))),
            ("width -21", set(width(-21.0, "shape", LINE))),
            ("based on \"line\"", set(width(1.0, "line", LINE))),
            ("based on \"Shape\"", set(width(1.0, "Shape", LINE))),
            ("nine colours", set(width(1.0, "colors", NINE))),
            (
                "the colour \"#12345\"",
                set(width(1.0, "colors", &["#12345"])),
            ),
            (
                "width keyed to 25",
                keys("width", &[(0, &[0.0]), (4, &[25.0])]),
            ),
            (
                "tolerance keyed to 300",
                keys("tolerance", &[(0, &[0.0]), (4, &[300.0])]),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_width_001.json",
        vec![
            (
                "width 20 with eight colours, the top of the ranges,",
                set(width(20.0, "colors", &NINE[..8])),
            ),
            (
                "width -20 with no colour, the bottom,",
                set(width(-20.0, "shape", &[])),
            ),
            (
                "width keyed from -4 to 4",
                keys("width", &[(0, &[-4.0]), (4, &[4.0])]),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_width_002.json", 0), ("fx_width_009.json", 3)]);

    t.finish("B-38_line_width_table.md");
}
