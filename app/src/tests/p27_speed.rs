//! P-27's speed fixes (D-1115 to D-1120): each one changes how fast the window answers, never
//! what it answers. The checks here compare the new answer with the old one, on the reference
//! shot and on the 200-layer copy `tests/p20_edit_cost.rs` measures.

use super::*;

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .join(rel)
}

/// The reference shot with `layers` layers, as `tests/p20_edit_cost.rs` builds it: the original
/// four when `layers` is 4, otherwise copies of layer 2, each with 25 position keys.
fn project_text(layers: usize) -> String {
    let text = std::fs::read_to_string(repo("verification/B-08a_project.json")).unwrap();
    if layers == 4 {
        return text;
    }
    let mut p: serde_json::Value = serde_json::from_str(&text).unwrap();
    let comp = &mut p["compositions"][0];
    let model = comp["layers"][1].clone();
    let mut list = Vec::new();
    let mut order = Vec::new();
    for n in 0..layers {
        let mut l = model.clone();
        let id = format!("layer-p20-{n}");
        l["id"] = serde_json::json!(id);
        l["name"] = serde_json::json!(format!("copy {n}"));
        l["transform"]["position"]["keyframes"] = serde_json::Value::Array(
            (0..25)
                .map(|k| {
                    serde_json::json!({"frame": k * 9, "value": [k as f64 * 3.0, n as f64], "interp": "linear"})
                })
                .collect(),
        );
        order.push(serde_json::json!(id));
        list.push(l);
    }
    comp["layers"] = serde_json::Value::Array(list);
    comp["layer_order"] = serde_json::Value::Array(order);
    serde_json::to_string_pretty(&p).unwrap()
}

/// A window on the reference shot holding the `layers`-layer project.
fn viewer(layers: usize) -> Mutex<Viewer> {
    let mut viewer = open(&repo("verification/B-08a_project.json")).expect("the shot opens");
    let loaded = persist::load_str(&project_text(layers)).expect("the copy opens");
    viewer.document = loaded.document;
    viewer.preserved = Arc::new(loaded.preserved);
    Mutex::new(viewer)
}

/// `/state` as it was written before D-1115, step for step, kept here as the reference.
fn state_before_d1115(viewer: &Mutex<Viewer>) -> String {
    let viewer = &*viewer.lock().unwrap();
    let project: serde_json::Value =
        serde_json::from_str(&persist::to_json(viewer.document.project(), &viewer.preserved))
            .unwrap();
    serde_json::json!({
        "project": project,
        "composition": viewer.composition.as_str(),
        "revision": viewer.document.revision(),
        "solo": viewer.solo.iter().map(|id| id.as_str()).collect::<Vec<_>>(),
        "undo": viewer.document.undo_labels(),
        "redo": viewer.document.redo_labels(),
    })
    .to_string()
}

/// D-1115: `/state` written outside the lock and spliced is the same answer as before, every
/// field, read as JSON, for the reference shot and the 200-layer copy, before and after an edit.
#[test]
fn d1115_state_is_the_same_json_as_before() {
    for layers in [4, 200] {
        let viewer = viewer(layers);
        for step in 0..2 {
            let (new_text, old_text) = (state(&viewer), state_before_d1115(&viewer));
            let new: serde_json::Value = serde_json::from_str(&new_text).unwrap();
            let old: serde_json::Value = serde_json::from_str(&old_text).unwrap();
            assert_eq!(new, old, "{layers} layers, step {step}");
            // Not a longer text to send: written spaced it was twice the size.
            assert!(new_text.len() <= old_text.len(), "{layers}: {} > {}", new_text.len(), old_text.len());
            println!("{layers} layers, step {step}: {} bytes, before {}", new_text.len(), old_text.len());
            let first = {
                let held = viewer.lock().unwrap();
                let comp = held.document.project().composition(&held.composition).unwrap();
                let id = comp.layers_in_order().next().unwrap().id.clone();
                id
            };
            edit_command(
                &viewer,
                "layer.rename",
                Some(&format!("layer={}&name=p27", first.as_str())),
            )
            .unwrap();
        }
    }
}

/// D-1120: the copy autosave writes outside the lock is the file it wrote under the lock, byte
/// for byte, on the 200-layer copy; and the window can take an edit between the two halves.
#[test]
fn d1120_autosave_writes_the_same_file_outside_the_lock() {
    let folder = repo("target/p27_d1120");
    let _ = std::fs::remove_dir_all(&folder);
    std::fs::create_dir_all(&folder).unwrap();
    let viewer = viewer(200);
    viewer.lock().unwrap().path = Some(folder.join("copies.json"));
    let first = {
        let held = viewer.lock().unwrap();
        let comp = held.document.project().composition(&held.composition).unwrap();
        let id = comp.layers_in_order().next().unwrap().id.clone();
        id
    };
    let rename = |name: &str| {
        edit_command(&viewer, "layer.rename", Some(&format!("layer={}&name={name}", first.as_str())))
            .unwrap();
    };
    rename("before");
    let start = Instant::now();
    assert!(autosave_due(&mut viewer.lock().unwrap(), start, None).is_none(), "not yet due");
    let due = autosave_due(&mut viewer.lock().unwrap(), start + AUTOSAVE_AFTER, None).expect("due");
    // What the old code wrote: the project as it stood at the look at the clock.
    let expected = {
        let held = viewer.lock().unwrap();
        persist::to_json(held.document.project(), &held.preserved)
    };
    // An edit while the copy is being written: the lock is free, and the copy is unchanged by it.
    rename("during");
    let (said, recovery) = autosave_write(due);
    let written = recovery.expect("written").into_iter().next().expect("one copy");
    assert!(said.starts_with("Recovery snapshot written to"), "{said}");
    assert_eq!(std::fs::read_to_string(&written).unwrap(), expected);
    assert!(expected.contains("\"before\"") && !expected.contains("\"during\""));
    let _ = std::fs::remove_dir_all(&folder);
}
