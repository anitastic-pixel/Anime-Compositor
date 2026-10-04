//! D-282 to D-287: taking strokes off, moving them, and a sketch layer's opacity, lock and order.
//!
//! Writes `verification/D-282_sketch_edits_table.md`.
//!
//! The owner accepted D-282 to D-284 on 2026-10-03 and, in the same message, liked the rest of the
//! sketch list (D-285 to D-289). Four commands are new: `sketch.remove_strokes`,
//! `sketch.move_strokes`, `sketch.set_look` and `sketch.move_layer`; a layer saves `opacity` and
//! `locked`, a stroke `filled` and `pressure_opacity`, each only when it is not the default, so a
//! file without them saves byte for byte as before. This check fails to build on cc14398.

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
fn d282_sketch_strokes_and_layers_are_edited() {
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut row = |what: &str, expected: &str, actual: String| rows.push((what.into(), expected.into(), actual));
    let said = |r: Result<(), String>| match r {
        Ok(()) => "taken".to_string(),
        Err(_) => "refused".to_string(),
    };

    let mut doc = persist::load(&repo("verification/B-08a_project.json")).unwrap_or_else(|d| panic!("{}", d.message)).document;
    let composition = doc.project().compositions[0].id.clone();
    let (a, b) = (Id::new("sketch-a"), Id::new("sketch-b"));
    let apply = |doc: &mut Document, command: Command| doc.apply(command).map(|_| ()).map_err(|d| d.message);
    let stroke = |x: f64| Stroke {
        frame: Some(10),
        tool: "brush".into(),
        size: 10.0,
        colour: "#c8302c".into(),
        points: vec![[x, 100.0], [x + 20.0, 120.0]],
        ..Default::default()
    };
    let layer = |doc: &mut Document, id: &Id, name: &str| {
        apply(doc, Command::SetSketchLayer { composition: composition.clone(), sketch: id.clone(), name: name.into(), visible: true, whole_cut: false }).unwrap()
    };
    layer(&mut doc, &a, "Rough");
    layer(&mut doc, &b, "Clean");
    for x in [0.0, 100.0, 200.0, 300.0] {
        apply(&mut doc, Command::AddSketchStroke { composition: composition.clone(), sketch: a.clone(), stroke: stroke(x) }).unwrap();
    }
    let filled = Stroke { filled: true, pressure: vec![0.2, 0.9], pressure_opacity: true, ..stroke(400.0) };
    apply(&mut doc, Command::AddSketchStroke { composition: composition.clone(), sketch: a.clone(), stroke: filled }).unwrap();
    let starts = |doc: &Document| -> String {
        doc.project().compositions[0].sketches[0].strokes.iter().map(|k| format!("{}", k.points[0][0])).collect::<Vec<_>>().join(",")
    };

    // D-282: the stroke eraser takes whole strokes off, by their places, in one undo step.
    let remove = |doc: &mut Document, strokes: Vec<usize>| apply(doc, Command::RemoveSketchStrokes { composition: composition.clone(), sketch: a.clone(), strokes });
    row("Taking strokes 1 and 2 off leaves 0, 300 and 400", "0,300,400", { remove(&mut doc, vec![1, 2]).unwrap(); starts(&doc) });
    doc.undo();
    row("One undo puts both back where they were", "0,100,200,300,400", starts(&doc));
    row("A place past the last stroke is refused", "refused", said(remove(&mut doc, vec![9])));
    row("The same place twice is refused", "refused", said(remove(&mut doc, vec![1, 1])));
    row("No places at all is refused", "refused", said(remove(&mut doc, vec![])));

    // D-284: the lasso moves the chosen strokes by one amount, in one undo step.
    let shift = |doc: &mut Document, strokes: Vec<usize>, by: [f64; 2]| {
        apply(doc, Command::MoveSketchStrokes { composition: composition.clone(), sketch: a.clone(), strokes, by })
    };
    shift(&mut doc, vec![0, 3], [5.0, -2.5]).unwrap();
    let k = &doc.project().compositions[0].sketches[0].strokes;
    row("Moving strokes 0 and 3 by 5, -2.5 moves every point of both", "[5,97.5] [25,117.5] [305,97.5] [325,117.5]",
        format!("{:?} {:?} {:?} {:?}", k[0].points[0], k[0].points[1], k[3].points[0], k[3].points[1]).replace(".0", "").replace(", ", ","));
    row("and leaves the others where they were", "100,200,400", format!("{},{},{}", k[1].points[0][0], k[2].points[0][0], k[4].points[0][0]));
    doc.undo();
    row("One undo puts them back", "0,100,200,300,400", starts(&doc));
    row("A move by an amount that is not a number is refused", "refused", said(shift(&mut doc, vec![0], [f64::NAN, 0.0])));

    // D-283: opacity, lock and order.
    let look = |doc: &mut Document, id: &Id, opacity: f64, locked: bool| {
        apply(doc, Command::SetSketchLook { composition: composition.clone(), sketch: id.clone(), opacity, locked })
    };
    row("An opacity of 0.4 is taken", "taken", said(look(&mut doc, &a, 0.4, false)));
    row("An opacity of 1.5 is refused", "refused", said(look(&mut doc, &a, 1.5, false)));
    row("An opacity below 0 is refused", "refused", said(look(&mut doc, &a, -0.1, false)));
    look(&mut doc, &a, 0.4, true).unwrap();
    row("On a locked layer, a stroke is refused", "refused",
        said(apply(&mut doc, Command::AddSketchStroke { composition: composition.clone(), sketch: a.clone(), stroke: stroke(9.0) })));
    row("taking strokes off is refused", "refused", said(remove(&mut doc, vec![0])));
    row("moving strokes is refused", "refused", said(shift(&mut doc, vec![0], [1.0, 1.0])));
    row("Clear is refused", "refused", said(apply(&mut doc, Command::ClearSketch { composition: composition.clone(), sketch: a.clone(), frame: Some(10) })));
    row("deleting the layer is refused", "refused", said(apply(&mut doc, Command::RemoveSketchLayer { composition: composition.clone(), sketch: a.clone() })));
    row("and the refusal says how to go on", "says Unlock it",
        match remove(&mut doc, vec![0]) { Err(m) if m.contains("Unlock it") => "says Unlock it".into(), other => format!("{other:?}") });
    row("Unlocked, a stroke is taken again", "taken", {
        look(&mut doc, &a, 0.4, false).unwrap();
        said(apply(&mut doc, Command::AddSketchStroke { composition: composition.clone(), sketch: a.clone(), stroke: stroke(500.0) }))
    });
    look(&mut doc, &a, 0.4, true).unwrap();
    let order = |doc: &Document| doc.project().compositions[0].sketches.iter().map(|s| s.name.clone()).collect::<Vec<_>>().join(",");
    let to = |doc: &mut Document, id: &Id, to: usize| apply(doc, Command::MoveSketchLayer { composition: composition.clone(), sketch: id.clone(), to });
    row("Moving Rough to the top puts Clean under it", "Clean,Rough", { to(&mut doc, &a, 1).unwrap(); order(&doc) });
    row("Moving a layer past the top is refused", "refused", said(to(&mut doc, &b, 2)));

    // The saved file: the new fields are written only when set, and read back the same.
    let drawn = doc.project().clone();
    let text = persist::to_json(&drawn, &Preserved::none());
    let saved: serde_json::Value = serde_json::from_str(&text).unwrap();
    let rough = &saved["compositions"][0]["sketches"][1];
    row("Rough is saved with its opacity and lock", "0.4 true", format!("{} {}", rough["opacity"], rough["locked"]));
    row("Clean, at full opacity and unlocked, saves neither", "neither",
        if saved["compositions"][0]["sketches"][0].get("opacity").is_none() && saved["compositions"][0]["sketches"][0].get("locked").is_none() { "neither".into() } else { "a line".into() });
    row("The filled pen stroke saves filled and pressure_opacity", "true true", format!("{} {}", rough["strokes"][4]["filled"], rough["strokes"][4]["pressure_opacity"]));
    row("A plain stroke saves neither", "neither",
        if rough["strokes"][0].get("filled").is_none() && rough["strokes"][0].get("pressure_opacity").is_none() { "neither".into() } else { "a line".into() });
    let back = persist::load_str(&text).unwrap_or_else(|d| panic!("{}", d.message));
    row("Opened again, both layers are the same", "the same",
        if back.document.project().compositions[0].sketches == drawn.compositions[0].sketches { "the same".into() } else { format!("{:?}", back.document.project().compositions[0].sketches) });
    row("Saved again, the text is the same, byte for byte", "the same",
        if persist::to_json(back.document.project(), &back.preserved) == text { "the same" } else { "different" }.into());

    // A file with a value that cannot be is refused at its place, not opened with it dropped.
    for (pointer, value, what) in [
        ("opacity", serde_json::json!(2), "/sketches/1/opacity"),
        ("locked", serde_json::json!("yes"), "/sketches/1/locked"),
    ] {
        let mut bad = saved.clone();
        bad["compositions"][0]["sketches"][1][pointer] = value.clone();
        row(&format!("A file with {pointer} {value} is refused, saying where"), &format!("refused at {what}"), match persist::load_str(&bad.to_string()) {
            Err(d) if d.message.contains(what) || d.detail.contains(what) => format!("refused at {what}"),
            Err(d) => format!("refused: {} / {}", d.message, d.detail),
            Ok(_) => "opened".into(),
        });
    }
    let mut bad = saved.clone();
    bad["compositions"][0]["sketches"][1]["strokes"][0]["filled"] = serde_json::json!(1);
    row("A file with filled 1 is refused, saying where", "refused at /strokes/0/filled", match persist::load_str(&bad.to_string()) {
        Err(d) if d.message.contains("strokes/0/filled") || d.detail.contains("strokes/0/filled") => "refused at /strokes/0/filled".into(),
        Err(d) => format!("refused: {} / {}", d.message, d.detail),
        Ok(_) => "opened".into(),
    });

    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut md = format!(
        "# D-282 to D-287: sketch strokes and layers\n\nWritten by `tests/d282_sketch_edits.rs`.\n\n**{passed} of {} checks pass.**\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        rows.len()
    );
    for (what, expected, actual) in &rows {
        let _ = writeln!(md, "| {what} | {expected} | {actual} | {} |", if expected == actual { "PASS" } else { "FAIL" });
    }
    fs::write(repo("verification/D-282_sketch_edits_table.md"), md).unwrap();
    assert_eq!(passed, rows.len(), "{rows:#?}");
}
