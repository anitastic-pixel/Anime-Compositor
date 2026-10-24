//! B-648: Echo's Composite In Back puts the present copy at the back and In Front puts it in
//! front, as After Effects does (D-768). Writes `verification/B-648_echo_stacking_table.md`.

mod effect_table;

use std::fs;

use effect_table::Table;
use serde_json::Value as J;

use anime_compositor::persist;

#[test]
fn b648_echo_stacking() {
    let mut t = Table::new(
        "echo",
        "# B-648: Echo's Composite In Back and In Front\n\nD-768, accepted on 2026-10-24. \
         Composite In Back now puts the present copy at the back and each echo farther forward; \
         Composite In Front puts the present copy in front. Every expected pixel is \
         `Fixtures/echo/expected_echo.json`, written by `tools/echo_reference.py`; the answer is \
         the largest difference against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-ECHO-010 and 011 (document 25), the two cases D-768 changed");
    t.fixtures_numbered("expected_echo.json", 10..=11);

    t.heading("What each operator puts on top, frame 4, row 2");
    let loaded_back = t.load("fx_echo_010.json");
    let loaded_front = t.load("fx_echo_011.json");
    let plain = t.load("fx_echo_002.json");
    let back = t.render(&loaded_back.document, 4, 64);
    let front = t.render(&loaded_front.document, 4, 64);
    let drawing = t.render(&plain.document, 4, 64);
    let width = back.width();
    let px = |b: &anime_compositor::WorkingBuffer, x: usize| -> [f32; 4] {
        let i = (2 * width + x) * 4;
        let d = b.data();
        [d[i], d[i + 1], d[i + 2], d[i + 3]]
    };
    // Column 9 is the present ball's left column (drawing 5, columns 9 to 11), opaque.
    let near = |a: [f32; 4], b: [f32; 4]| a.iter().zip(b).all(|(x, y)| (x - y).abs() <= 2e-5);
    t.row(
        "In Front: the present ball's opaque column 9 is the drawing itself, whole on top",
        &format!("{:?} against the drawing's {:?}", px(&front, 9), px(&drawing, 9)),
        near(px(&front, 9), px(&drawing, 9)),
    );
    t.row(
        "In Back: the same column is covered by the echoes, so it differs from the drawing",
        &format!("{:?} against the drawing's {:?}", px(&back, 9), px(&drawing, 9)),
        !near(px(&back, 9), px(&drawing, 9)),
    );

    t.heading("A project saved before D-768");
    // fx_echo_010.json and 011.json were saved before D-768 and are not changed by it.
    for file in ["fx_echo_010.json", "fx_echo_011.json"] {
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

    t.finish("B-648_echo_stacking_table.md");
}
