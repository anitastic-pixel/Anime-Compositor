//! D-261 (W-46): Sketch, notes drawn over the cut that are never exported.
//!
//! Writes `verification/D-261_sketch_table.md`.
//!
//! The owner accepted it on 2026-10-02 as choice (a), with four checks:
//!
//! 1. A project with two sketch layers saves them and reads them back exactly.
//! 2. Its exports are byte for byte the same as without the sketches: the engine never reads them.
//! 3. Today's build keeps the sketches when it opens and saves the project. It keeps lines it
//!    does not know, so this row passes before the change too, and must still pass after it,
//!    down to a line a later build might add to a stroke.
//! 4. A project without sketches is unchanged: every project under `Fixtures/` that opens saves
//!    byte for byte as on e395d8b, the hash D-253 pinned.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

use anime_compositor::command::Command;
use anime_compositor::export::{export_sequence, ExportRequest, MissingSource, OutputFormat};
use anime_compositor::model::{Id, Project, Stroke};
use anime_compositor::persist::Preserved;
use anime_compositor::{persist, sha256, OutputAlpha, OutputDepth};

/// Taken on e395d8b (D-253's check): how many fixture projects open, and the SHA-256 of their
/// saved text, one after another in path order.
const FIXTURES_OPENED: usize = 2297;
const FIXTURES_SAVED: &str = "aeadb2e51777a56efb28e3113b652eaa65c03d10cf2ba66b8d4798f0c86dae2d";

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn projects(dir: &Path, found: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            projects(&path, found);
        } else if path.extension().is_some_and(|e| e == "json") {
            found.push(path);
        }
    }
}

/// Frames 8 to 12 of the project's first composition as PNG, every file by name with its bytes.
fn exported(project: &Project, name: &str) -> Vec<(String, Vec<u8>)> {
    let dir = repo("target/d261_sketch").join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let request = ExportRequest {
        composition: project.compositions[0].id.clone(),
        first_frame: 8,
        last_frame: 12,
        output_dir: dir.clone(),
        naming: "d261_%04d.png".to_string(),
        depth: OutputDepth::Eight,
        alpha: OutputAlpha::Straight,
        tile_size: 128,
        missing: MissingSource::Block,
        format: OutputFormat::Png,
        choices: Default::default(),
    };
    let report = export_sequence(project, &repo("Fixtures/reference_shot"), &request, &AtomicBool::new(false));
    assert!(report.succeeded(), "the export did not complete: {:?}", report.diagnostics);
    let mut files: Vec<(String, Vec<u8>)> = fs::read_dir(&dir)
        .unwrap()
        .flatten()
        .map(|e| (e.file_name().to_string_lossy().into_owned(), fs::read(e.path()).unwrap()))
        .collect();
    files.sort();
    files
}

/// The reference shot with two sketch layers written as text, the way this build will write
/// them, plus a line on a layer and on a stroke that no build writes yet. Whole numbers are
/// written without a point, as the project writer has always written them.
fn with_sketches_as_text() -> (String, serde_json::Value) {
    let mut root: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(repo("verification/B-08a_project.json")).unwrap()).unwrap();
    let sketches = serde_json::json!([
        { "id": "sketch-1", "name": "Timing notes", "visible": true, "whole_cut": false,
          "strokes": [
            { "frame": 10, "tool": "pencil", "size": 2, "colour": "#e84a5f",
              "points": [[120.5, 300], [180.25, 320], [240, 360.75]] },
            { "frame": 10, "tool": "eraser", "size": 12, "colour": "#ffffff",
              "points": [[150, 310]], "pressure": [0.4] } ] },
        { "id": "sketch-2", "name": "Layout", "visible": false, "whole_cut": true, "opacity": 0.5,
          "strokes": [
            { "tool": "brush", "size": 5, "colour": "#3a8ee8",
              "points": [[960, 0], [960, 1080]] } ] }
    ]);
    root["compositions"][0]["sketches"] = sketches.clone();
    (root.to_string(), sketches)
}

#[test]
fn d261_sketches_are_kept_and_never_drawn_into_a_frame() {
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut row = |what: &str, expected: &str, actual: String| rows.push((what.into(), expected.into(), actual));

    // Row 1, through the commands the page sends.
    let mut doc = persist::load(&repo("verification/B-08a_project.json")).unwrap_or_else(|d| panic!("{}", d.message)).document;
    let without = doc.project().clone();
    let composition = without.compositions[0].id.clone();
    let apply = |doc: &mut anime_compositor::command::Document, command: Command| doc.apply(command).map(|_| ()).map_err(|d| d.message);
    let notes = Id::new("sketch-1");
    let layout = Id::new("sketch-2");
    let stroke = |frame: Option<i32>, tool: &str, size: f64, colour: &str, points: Vec<[f64; 2]>| Stroke {
        frame,
        tool: tool.into(),
        size,
        colour: colour.into(),
        points,
        ..Default::default()
    };
    let steps = [
        Command::SetSketchLayer { composition: composition.clone(), sketch: notes.clone(), name: "Timing notes".into(), visible: true, whole_cut: false },
        Command::SetSketchLayer { composition: composition.clone(), sketch: layout.clone(), name: "Layout".into(), visible: false, whole_cut: true },
        Command::AddSketchStroke {
            composition: composition.clone(),
            sketch: notes.clone(),
            stroke: stroke(Some(10), "pencil", 2.0, "#e84a5f", vec![[120.5, 300.0], [180.25, 320.0], [240.0, 360.75]]),
        },
        Command::AddSketchStroke { composition: composition.clone(), sketch: notes.clone(), stroke: stroke(Some(10), "eraser", 12.0, "#ffffff", vec![[150.0, 310.0]]) },
        Command::AddSketchStroke { composition: composition.clone(), sketch: notes.clone(), stroke: stroke(Some(11), "brush", 5.0, "#f2c94c", vec![[10.0, 10.0], [20.0, 20.0]]) },
        Command::AddSketchStroke { composition: composition.clone(), sketch: layout.clone(), stroke: stroke(None, "brush", 5.0, "#3a8ee8", vec![[960.0, 0.0], [960.0, 1080.0]]) },
    ];
    let mut refused = Vec::new();
    for step in steps {
        if let Err(e) = apply(&mut doc, step) {
            refused.push(e);
        }
    }
    row("Two sketch layers and four strokes are taken", "none refused", if refused.is_empty() { "none refused".into() } else { format!("{refused:?}") });
    row(
        "A stroke with a point that is not a number is refused",
        "refused",
        if apply(&mut doc, Command::AddSketchStroke { composition: composition.clone(), sketch: notes.clone(), stroke: stroke(Some(3), "pencil", 2.0, "#e84a5f", vec![[f64::NAN, 1.0]]) }).is_err() { "refused" } else { "taken" }.into(),
    );
    row(
        "A tool other than brush, pencil and eraser is refused",
        "refused",
        if apply(&mut doc, Command::AddSketchStroke { composition: composition.clone(), sketch: notes.clone(), stroke: stroke(Some(3), "spray", 2.0, "#e84a5f", vec![[1.0, 1.0]]) }).is_err() { "refused" } else { "taken" }.into(),
    );
    row(
        "A stroke on a sketch layer that is not there is refused",
        "refused",
        if apply(&mut doc, Command::AddSketchStroke { composition: composition.clone(), sketch: Id::new("sketch-9"), stroke: stroke(Some(3), "pencil", 2.0, "#e84a5f", vec![[1.0, 1.0]]) }).is_err() { "refused" } else { "taken" }.into(),
    );
    let strokes = |p: &Project| p.compositions[0].sketches.iter().map(|s| s.strokes.len().to_string()).collect::<Vec<_>>().join(" + ");
    row("Before Clear: strokes per layer", "3 + 1", strokes(doc.project()));
    let _ = apply(&mut doc, Command::ClearSketch { composition: composition.clone(), sketch: notes.clone(), frame: Some(11) });
    row("Clear on frame 11 takes that frame's stroke only", "2 + 1", strokes(doc.project()));
    doc.undo();
    row("Undo puts it back", "3 + 1", strokes(doc.project()));
    doc.undo();
    row("Undo again takes back the last stroke drawn", "3 + 0", strokes(doc.project()));
    doc.redo();
    let drawn = doc.project().clone();
    let text = persist::to_json(&drawn, &Preserved::none());
    let back = persist::load_str(&text).unwrap_or_else(|d| panic!("{}", d.message));
    row(
        "Saved and opened again, the two layers are the same, every point",
        "the same",
        if back.document.project().compositions[0].sketches == drawn.compositions[0].sketches { "the same".into() } else { format!("{:?}", back.document.project().compositions[0].sketches) },
    );
    row(
        "Saved again, the text is the same, byte for byte",
        "the same",
        if persist::to_json(back.document.project(), &back.preserved) == text { "the same" } else { "different" }.into(),
    );

    // Row 2: the engine never reads them.
    let plain = exported(&without, "without");
    let sketched = exported(&drawn, "with");
    row("Frames 8 to 12 exported without sketches", "5 files", format!("{} files", plain.len()));
    row(
        "With the sketches (on frames 10 and 11, and the whole cut, one layer hidden), the same files, every byte",
        "the same",
        if plain == sketched { "the same".into() } else { format!("{} files, differing", sketched.len()) },
    );

    // Row 3: today's build, and this one, keep what they are given.
    let (written, sketches) = with_sketches_as_text();
    let kept = persist::load_str(&written).unwrap_or_else(|d| panic!("{}", d.message));
    let saved: serde_json::Value = serde_json::from_str(&persist::to_json(kept.document.project(), &kept.preserved)).unwrap();
    row(
        "Sketches in a file, with a line on a layer and a stroke no build writes yet, are kept through open and save",
        "kept as they were",
        if saved["compositions"][0]["sketches"] == sketches { "kept as they were".into() } else { saved["compositions"][0]["sketches"].to_string() },
    );

    // Row 4.
    let plain_text = persist::to_json(&without, &Preserved::none());
    row(
        "The reference shot without sketches saves no sketches line",
        "no line",
        if plain_text.contains("\"sketches\"") { "a line" } else { "no line" }.into(),
    );
    let mut found = Vec::new();
    projects(&repo("Fixtures"), &mut found);
    found.sort();
    // The files that were there on e395d8b; fixtures added since (D-318 on) are new behaviour.
    let pinned = include_str!("fixtures_e395d8b.txt");
    found.retain(|p| {
        let rel = p.strip_prefix(repo("")).unwrap().to_string_lossy().replace(std::path::MAIN_SEPARATOR, "/");
        pinned.lines().any(|l| l == rel)
    });
    let mut saved = String::new();
    let mut opened = 0;
    for path in &found {
        let Ok(text) = fs::read_to_string(path) else { continue };
        if let Ok(loaded) = persist::load_str(&text) {
            opened += 1;
            saved.push_str(&persist::to_json(loaded.document.project(), &loaded.preserved));
        }
    }
    row("Fixture projects that open", &FIXTURES_OPENED.to_string(), opened.to_string());
    row("Their saved text, all of it, is byte for byte as before (SHA-256)", FIXTURES_SAVED, sha256::hex(saved.as_bytes()));

    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut md = format!(
        "# D-261: Sketch, kept and never drawn into a frame\n\nWritten by `tests/d261_sketch.rs`. The fixture hash was taken on e395d8b, before D-253.\n\n**{passed} of {} checks pass.**\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        rows.len()
    );
    for (check, e, a) in &rows {
        let _ = writeln!(md, "| {check} | {e} | {a} | {} |", if e == a { "pass" } else { "FAIL" });
    }
    fs::write(repo("verification/D-261_sketch_table.md"), md).unwrap();
    let failed: Vec<_> = rows.iter().filter(|(_, e, a)| e != a).collect();
    assert!(failed.is_empty(), "{failed:?}");
}
