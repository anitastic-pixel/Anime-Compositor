//! D-271: a pen's pressure in the Sketch workspace.
//!
//! Writes `verification/D-271_sketch_pen_table.md`.
//!
//! The owner asked for it on 2026-10-03 ("pressure sensitivity when using a pen"). A stroke drawn
//! with a pen keeps one pressure from 0 to 1 per point and saves it as `pressure`; a stroke drawn
//! with a mouse keeps none and saves as before. The engine still never reads a sketch, which
//! D-261's check shows byte for byte; this check fails to build on 076359e, which has no pressure.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use anime_compositor::command::{Command, Document};
use anime_compositor::model::{Id, Stroke};
use anime_compositor::persist;
use anime_compositor::persist::Preserved;

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

#[test]
fn d271_a_pen_strokes_pressure_is_kept() {
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut row = |what: &str, expected: &str, actual: String| rows.push((what.into(), expected.into(), actual));

    let mut doc = persist::load(&repo("verification/B-08a_project.json")).unwrap_or_else(|d| panic!("{}", d.message)).document;
    let composition = doc.project().compositions[0].id.clone();
    let sketch = Id::new("sketch-1");
    let apply = |doc: &mut Document, command: Command| doc.apply(command).map(|_| ()).map_err(|d| d.message);
    let stroke = |points: Vec<[f64; 2]>, pressure: Vec<f64>| Stroke {
        frame: Some(10),
        tool: "brush".into(),
        size: 10.0,
        colour: "#c8302c".into(),
        points,
        pressure,
        ..Default::default()
    };
    let add = |doc: &mut Document, s: Stroke| {
        apply(doc, Command::AddSketchStroke { composition: composition.clone(), sketch: sketch.clone(), stroke: s })
    };
    apply(&mut doc, Command::SetSketchLayer { composition: composition.clone(), sketch: sketch.clone(), name: "Pen".into(), visible: true, whole_cut: false }).unwrap();

    let pen = stroke(vec![[100.0, 100.0], [140.0, 110.0], [180.0, 130.0]], vec![0.1, 0.55, 1.0]);
    row("A pen stroke, light to firm, is taken", "taken", if add(&mut doc, pen.clone()).is_ok() { "taken" } else { "refused" }.into());
    row("A mouse stroke, with no pressure, is taken", "taken", if add(&mut doc, stroke(vec![[10.0, 10.0], [20.0, 20.0]], vec![])).is_ok() { "taken" } else { "refused" }.into());
    row(
        "A stroke with fewer pressures than points is refused",
        "refused",
        if add(&mut doc, stroke(vec![[1.0, 1.0], [2.0, 2.0]], vec![0.5])).is_err() { "refused" } else { "taken" }.into(),
    );
    row(
        "A pressure past 1 is refused",
        "refused",
        if add(&mut doc, stroke(vec![[1.0, 1.0]], vec![1.5])).is_err() { "refused" } else { "taken" }.into(),
    );
    row(
        "A pressure that is not a number is refused",
        "refused",
        if add(&mut doc, stroke(vec![[1.0, 1.0]], vec![f64::NAN])).is_err() { "refused" } else { "taken" }.into(),
    );

    let drawn = doc.project().clone();
    let text = persist::to_json(&drawn, &Preserved::none());
    let saved: serde_json::Value = serde_json::from_str(&text).unwrap();
    let strokes = &saved["compositions"][0]["sketches"][0]["strokes"];
    row("The pen stroke is saved with its pressure", "[0.1,0.55,1]", strokes[0]["pressure"].to_string());
    row("The mouse stroke is saved with no pressure line", "no line", if strokes[1].get("pressure").is_some() { "a line" } else { "no line" }.into());
    let back = persist::load_str(&text).unwrap_or_else(|d| panic!("{}", d.message));
    row(
        "Opened again, both strokes are the same, every pressure",
        "the same",
        if back.document.project().compositions[0].sketches == drawn.compositions[0].sketches { "the same".into() } else { format!("{:?}", back.document.project().compositions[0].sketches) },
    );
    row(
        "Saved again, the text is the same, byte for byte",
        "the same",
        if persist::to_json(back.document.project(), &back.preserved) == text { "the same" } else { "different" }.into(),
    );
    doc.undo();
    doc.undo();
    row("Undo twice takes back both strokes", "0", doc.project().compositions[0].sketches[0].strokes.len().to_string());

    // A file whose pressure does not fit its points is refused with where and why, not opened
    // with the pressure quietly dropped (document 28).
    let mut bad = saved.clone();
    bad["compositions"][0]["sketches"][0]["strokes"][0]["pressure"] = serde_json::json!([0.5]);
    row(
        "A file with one pressure for three points is refused, saying where",
        "refused at /strokes/0/pressure",
        match persist::load_str(&bad.to_string()) {
            Err(d) if d.message.contains("strokes/0/pressure") || d.detail.contains("strokes/0/pressure") => "refused at /strokes/0/pressure".into(),
            Err(d) => format!("refused: {} / {}", d.message, d.detail),
            Ok(_) => "opened".into(),
        },
    );

    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut md = format!(
        "# D-271: a pen's pressure in a sketch\n\nWritten by `tests/d271_sketch_pen.rs`.\n\n**{passed} of {} checks pass.**\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        rows.len()
    );
    for (what, expected, actual) in &rows {
        let _ = writeln!(md, "| {what} | {expected} | {actual} | {} |", if expected == actual { "PASS" } else { "FAIL" });
    }
    fs::write(repo("verification/D-271_sketch_pen_table.md"), md).unwrap();
    assert_eq!(passed, rows.len(), "{rows:#?}");
}
