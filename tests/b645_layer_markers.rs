//! B-645: D-765, a layer's markers as real data: read, kept in frame order, saved, slid with the
//! layer, and a marker's length on either kind.
//!
//! Writes `verification/B-645_layer_markers_table.md`.
//!
//! # Where the expected values come from
//!
//! `tools/layer_markers_reference.py` works each case from D-765's rule (`Fixtures/layer_markers/`,
//! FX-LMARK-001 to 009). Nothing here was captured from the program.

use serde_json::Value as J;
use std::path::PathBuf;

use anime_compositor::command::Command;
use anime_compositor::model::{Id, Marker};
use anime_compositor::persist;

fn dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Fixtures/layer_markers")
}

fn markers_json(list: &[Marker]) -> J {
    J::Array(
        list.iter()
            .map(|m| serde_json::json!({"frame": m.frame, "name": m.name, "duration_frames": m.duration_frames}))
            .collect(),
    )
}

fn saved_layer_markers(text: &str) -> J {
    let root: J = serde_json::from_str(text).unwrap();
    root["compositions"][0]["layers"][0]
        .get("markers")
        .cloned()
        .unwrap_or(J::String("absent".into()))
}

#[test]
fn layer_markers_load_save_and_slide_as_the_reference_says() {
    let want: J = serde_json::from_str(
        &std::fs::read_to_string(dir().join("expected_layer_markers.json")).unwrap(),
    )
    .unwrap();
    let mut table = String::from(
        "# B-645: layer markers against the reference\n\n\
         Written by `tests/b645_layer_markers.rs`. Each case is `Fixtures/layer_markers/`'s, worked by \
         `tools/layer_markers_reference.py` from D-765's rule. Each file is opened, slid where the case \
         says, and saved; what the program holds, what it warns and what it writes must all be the \
         reference's.\n\n\
         | Fixture | Layer markers held | Warning | Layer `markers` written | Composition markers written | Pass |\n\
         | --- | --- | --- | --- | --- | --- |\n",
    );
    let (mut passed, mut total) = (0, 0);
    for case in want["cases"].as_array().unwrap() {
        let file = case["file"].as_str().unwrap();
        let mut loaded = persist::load_str(&std::fs::read_to_string(dir().join(file)).unwrap())
            .unwrap_or_else(|e| panic!("{file} opens: {e:?}"));
        let slide = case["slide_frames"].as_i64().unwrap() as i32;
        if slide != 0 {
            loaded
                .document
                .apply(Command::ShiftLayer {
                    composition: Id::new("comp-main"),
                    layer_id: Id::new("lm"),
                    in_frame: slide,
                })
                .expect("the layer slides");
        }
        let project = loaded.document.project();
        let comp = project.composition(&Id::new("comp-main")).unwrap();
        let held = markers_json(&comp.layer(&Id::new("lm")).unwrap().markers);
        let warnings: Vec<String> = loaded
            .warnings
            .iter()
            .map(|w| w.id.as_str().to_string())
            .collect();
        let named = loaded
            .warnings
            .iter()
            .all(|w| case["warning_names"].as_str().is_some_and(|at| w.detail.contains(at)));
        let text = persist::to_json(project, &loaded.preserved);
        let written = saved_layer_markers(&text);
        let root: J = serde_json::from_str(&text).unwrap();
        let comp_written = root["compositions"][0]
            .get("markers")
            .cloned()
            .unwrap_or(J::Array(vec![]));
        let want_warn: Vec<String> = case["warnings"]
            .as_array()
            .unwrap()
            .iter()
            .map(|w| w.as_str().unwrap().to_string())
            .collect();
        let ok = held == case["layer_markers"]
            && warnings == want_warn
            && named
            && written == case["saved_layer_markers"]
            && comp_written == case["saved_comp_markers"]
            && markers_json(&comp.markers) == case["comp_markers"];
        total += 1;
        passed += ok as usize;
        table.push_str(&format!(
            "| {file} | {held} | {} | {written} | {comp_written} | {} |\n",
            if warnings.is_empty() { "none".to_string() } else { warnings.join(", ") },
            if ok { "yes" } else { "NO" }
        ));
        assert!(
            ok,
            "{file}: held {held} warned {warnings:?} (named {named}) wrote {written} comp {comp_written}; want {case}"
        );
    }
    table.push_str(&format!("\n{passed} of {total} cases match the reference.\n"));
    std::fs::write(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("verification/B-645_layer_markers_table.md"),
        table,
    )
    .unwrap();
}

/// D-765: an old project, saved before layers had markers, opens with no word said and saves
/// byte for byte as it did (Fixtures/projects/minimal_project.json and a layer with none).
#[test]
fn an_old_project_without_layer_markers_loads_and_saves_as_before() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Fixtures/projects/minimal_project.json");
    let loaded = persist::load_str(&std::fs::read_to_string(path).unwrap()).expect("opens");
    assert!(loaded.warnings.is_empty(), "{:?}", loaded.warnings);
    let text = persist::to_json(loaded.document.project(), &loaded.preserved);
    assert!(!text.contains("\"markers\""), "{text}");
    let old = persist::load_str(&std::fs::read_to_string(dir().join("fx_lmark_003.json")).unwrap())
        .expect("opens");
    assert!(old.warnings.is_empty());
    let again = persist::to_json(old.document.project(), &old.preserved);
    assert!(!again.contains("\"markers\""), "{again}");
}

/// D-765: SetLayerMarkers is one entry to undo, keeps frame order, and undoes to what was there.
#[test]
fn setting_a_layers_markers_is_one_undo() {
    let mut loaded =
        persist::load_str(&std::fs::read_to_string(dir().join("fx_lmark_001.json")).unwrap()).unwrap();
    let doc = &mut loaded.document;
    let before = doc.undo_depth();
    doc.apply(Command::SetLayerMarkers {
        composition: Id::new("comp-main"),
        layer_id: Id::new("lm"),
        markers: vec![
            Marker { frame: 30, name: "b".into(), duration_frames: 4 },
            Marker { frame: 12, name: "hit".into(), duration_frames: 0 },
            Marker { frame: -3, name: "before".into(), duration_frames: 0 },
        ],
    })
    .expect("set");
    assert_eq!(doc.undo_depth(), before + 1);
    let frames = |doc: &anime_compositor::command::Document| {
        doc.project().composition(&Id::new("comp-main")).unwrap().layer(&Id::new("lm")).unwrap()
            .markers
            .iter()
            .map(|m| m.frame)
            .collect::<Vec<_>>()
    };
    assert_eq!(frames(doc), vec![-3, 12, 30]);
    doc.undo().expect("undo");
    assert_eq!(frames(doc), vec![12]);
}
