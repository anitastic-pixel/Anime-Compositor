//! B-771: Echo's Blend draws as light as its Add, and its Minimum keeps the whole trail with the
//! darker colour where copies overlap, as After Effects does (D-891). Writes
//! `verification/B-771_echo_blend_minimum_table.md`.

mod effect_table;

use std::fs;

use effect_table::Table;
use serde_json::Value as J;

use anime_compositor::{persist, WorkingBuffer};

#[test]
fn b771_echo_blend_minimum() {
    let mut t = Table::new(
        "echo",
        "# B-771: Echo's Blend and Minimum\n\nD-891, accepted on 2026-10-26. Blend now draws as \
         Add, the copies summed and held at 1; Minimum takes the darker straight colour of the \
         copies that cover a pixel, at the largest covering. Every expected pixel is \
         `Fixtures/echo/expected_echo.json`, written by `tools/echo_reference.py`; the answer is \
         the largest difference against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-ECHO-008 and 012 (document 25), the two cases D-891 changed");
    t.fixtures_numbered("expected_echo.json", 8..=8);
    t.fixtures_numbered("expected_echo.json", 12..=12);

    t.heading("What the two operators draw, row 2");
    let px = |b: &WorkingBuffer, x: usize| -> [f32; 4] {
        let i = (2 * b.width() + x) * 4;
        let d = b.data();
        [d[i], d[i + 1], d[i + 2], d[i + 3]]
    };
    let plain = t.load("fx_echo_002.json");
    let blend = t.load("fx_echo_012.json");
    let drawing = t.render(&plain.document, 4, 64);
    let blended = t.render(&blend.document, 4, 64);
    let lighter = (0..blended.width()).all(|x| {
        let (b, d) = (px(&blended, x), px(&drawing, x));
        (0..4).all(|c| b[c] >= d[c] - 2e-5)
    });
    t.row(
        "Blend, frame 4: every pixel at least as light as the present drawing alone, channel by \
         channel, as a sum is (an average of four was darker)",
        &format!("column 9 {:?} against the drawing's {:?}", px(&blended, 9), px(&drawing, 9)),
        lighter,
    );
    let minimum = t.load("fx_echo_008.json");
    let min3 = t.render(&minimum.document, 3, 64);
    let covered: Vec<usize> = (0..min3.width()).filter(|&x| px(&min3, x)[3] > 0.0).collect();
    t.row(
        "Minimum, frame 3: drawings 4 and 3 both drawn, columns 4 to 9 (before D-891 only the \
         two columns where they overlapped, 6 and 7)",
        &format!("covered columns {covered:?}"),
        covered == (4..=9).collect::<Vec<_>>(),
    );
    let c7 = px(&min3, 7);
    t.row(
        "Minimum, frame 3, column 7, where both balls are whole: opaque, the darker green of \
         drawing 3 and the darker blue of drawing 4",
        &format!("{c7:?}"),
        (c7[3] - 1.0).abs() <= 2e-5 && (c7[1] - 0.3515326).abs() <= 2e-5 && (c7[2] - 0.03189603).abs() <= 2e-5,
    );

    t.heading("A project saved before D-891");
    // fx_echo_008.json and 012.json were saved before D-891 and are not changed by it.
    for file in ["fx_echo_008.json", "fx_echo_012.json"] {
        let path = t.root.join(file);
        let original: J = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        let loaded = persist::load(&path).expect("opens");
        let warnings: Vec<&str> = loaded.warnings.iter().map(|d| d.id.as_str()).collect();
        let same = effect_table::same_json(&effect_table::saved(&loaded), &original);
        t.row(
            &format!("{file} opens with no warning and saves holding what it held, operator word included"),
            &format!("warnings {warnings:?}, saved {}", if same { "the same" } else { "differs" }),
            warnings.is_empty() && same,
        );
    }

    t.finish("B-771_echo_blend_minimum_table.md");
}
