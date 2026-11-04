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

/// D-1116: after every command of a run of edits, the page's copy built by the page's own
/// `mergeState` from "only what changed" equals a full `/state`, as JSON values, for the
/// reference shot and the 200-layer copy. Run under Node, the page's code taken from its markers.
#[test]
fn d1116_the_page_copy_built_from_changes_equals_a_full_read() {
    for layers in [4, 200] {
        let viewer = viewer(layers);
        let ids: Vec<String> = {
            let held = viewer.lock().unwrap();
            let comp = held.document.project().composition(&held.composition).unwrap();
            let ids = comp.layers_in_order().map(|l| l.id.as_str().to_string()).collect();
            ids
        };
        let (a, b) = (&ids[0], &ids[1]);
        let edits: Vec<(&str, String)> = vec![
            ("layer.rename", format!("layer={a}&name=p27")),
            ("property.set_base", format!("layer={a}&frame=0&prop=position&value=120,300")),
            ("layer.set_label", format!("layer={b}&label=16")),
            ("layer.add_solid", "name=p27%20solid".into()),
            ("layer.move_down", format!("layer={a}")),
            ("layer.duplicate", format!("layer={b}")),
            ("layer.delete", format!("layer={b}&frame=0")),
            ("edit.undo", String::new()),
            ("edit.undo", String::new()),
            ("edit.redo", String::new()),
            ("layer.set_solo", format!("layer={a}")),
            ("edit.undo", String::new()),
            ("edit.undo", String::new()),
            ("edit.undo", String::new()),
        ];
        let first: serde_json::Value = serde_json::from_str(&state_since(&viewer, None)).unwrap();
        let mut token = first["token"].as_u64().unwrap();
        let mut steps = Vec::new();
        for (name, query) in &edits {
            let _ = edit_command(&viewer, name, Some(query.as_str()).filter(|q| !q.is_empty()));
            let text = state_since(&viewer, Some(token));
            let answer: serde_json::Value = serde_json::from_str(&text).unwrap();
            token = answer["token"].as_u64().unwrap();
            let expected: serde_json::Value =
                serde_json::from_str(&state_before_d1115(&viewer)).unwrap();
            steps.push(serde_json::json!({
                "what": name, "bytes": text.len(), "answer": answer, "expected": expected,
            }));
        }
        let data = std::env::temp_dir().join(format!("p27_d1116_{layers}.json"));
        std::fs::write(&data, serde_json::json!({"first": first, "steps": steps}).to_string()).unwrap();
        let script = std::env::temp_dir().join("p27_d1116.js");
        std::fs::write(
            &script,
            "const fs = require('fs');\n\
             const page = fs.readFileSync(process.argv[2], 'utf8');\n\
             const from = page.indexOf('// ---- D-1116 merge begin');\n\
             const to = page.indexOf('// ---- D-1116 merge end');\n\
             if (from < 0 || to < from) throw new Error('the markers are not in the page');\n\
             const { mergeState } = new Function(page.slice(from, to) + '\\nreturn { mergeState };')();\n\
             const sorted = (v) => Array.isArray(v) ? v.map(sorted) : v && typeof v === 'object'\n\
               ? Object.fromEntries(Object.keys(v).sort().map((k) => [k, sorted(v[k])])) : v;\n\
             const same = (a, b) => JSON.stringify(sorted(a)) === JSON.stringify(sorted(b));\n\
             const data = JSON.parse(fs.readFileSync(process.argv[3], 'utf8'));\n\
             let doc = data.first;\n\
             const rows = data.steps.map((s) => {\n\
               const merged = mergeState(doc, s.answer);\n\
               if (!merged) return [s.what, 'refused', false, s.bytes];\n\
               doc = merged;\n\
               const { token, ...rest } = merged;\n\
               return [s.what, s.answer.project ? 'whole' : 'changes', same(rest, s.expected), s.bytes];\n\
             });\n\
             console.log(JSON.stringify(rows));\n",
        )
        .unwrap();
        let out = std::process::Command::new("node")
            .arg(&script)
            .arg(repo("app/ui/index.html"))
            .arg(&data)
            .output()
            .expect("the page's code runs under Node, which must be installed");
        assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
        let rows: Vec<(String, String, bool, usize)> = serde_json::from_slice(&out.stdout).unwrap();
        let _ = std::fs::remove_file(&data);
        for row in &rows {
            println!("{layers} layers: {row:?}");
            assert!(row.2, "{layers} layers: the page's copy differs after {}", row.0);
        }
        // Edits inside the composition are sent as changes, not as the whole project.
        assert_eq!(rows[0].1, "changes", "{layers}: a rename");
        assert_eq!(rows[1].1, "changes", "{layers}: a value");
        if layers == 200 {
            assert!(rows[0].3 < 100_000, "a rename at 200 layers sends {} bytes", rows[0].3);
        }
        // A copy the window did not send last is answered with the whole project.
        let stale: serde_json::Value =
            serde_json::from_str(&state_since(&viewer, Some(token + 1000))).unwrap();
        assert!(stale.get("project").is_some() && stale.get("changes").is_none());
    }
}

/// D-1119, by B-780's check: the 240 frames of the reference shot made ahead, then one edit, then
/// every frame the frame memory sends is byte for byte the frame made by hand after the same
/// edit. An edit that draws nothing keeps all 240; an edit that draws (hiding a layer), or a rename (names stay
/// in the comparison), keeps none, and no frame from memory is ever stale.
#[test]
fn d1119_frames_made_ahead_survive_an_edit_that_draws_nothing() {
    let export = Mutex::new(Export::default());
    let ask = |v: &Mutex<Viewer>, a: Ask| serve_logged(v, &export, None, None, a, Some(PreviewQuality::Draft));
    let cached = |r: &Response<Vec<u8>>| r.headers().get("x-cached").is_some_and(|v| v == "1");
    let shot = repo("Fixtures/reference_shot");
    let viewer_on = || {
        let mut viewer = demo();
        viewer.root = shot.clone();
        Mutex::new(viewer)
    };
    let edits: [(&str, &str, usize); 7] = [
        ("layer.set_label", "layer=layer-1&label=3", 240),
        ("layer.toggle_lock", "layer=layer-1", 240),
        ("layer.toggle_shy", "layer=layer-1", 240),
        ("layer.set_markers", "layer=layer-1&marker=2|beat", 240),
        ("timeline.set_markers", "marker=2|hit", 240),
        ("layer.rename", "layer=layer-1&name=p27", 0),
        ("layer.toggle_visibility", "layer=layer-1", 0),
    ];
    for (name, query, keeps) in edits {
        let ahead = viewer_on();
        ask(&ahead, Ask::Play(0));
        render_ahead_by(&ahead, None, 1);
        let cold = viewer_on();
        for v in [&ahead, &cold] {
            edit_command(v, name, Some(query)).expect(name);
        }
        let (mut kept, mut same) = (0, 0);
        for n in 0..240 {
            let from = ask(&ahead, Ask::Frame(n));
            if cached(&from) {
                kept += 1;
                same += usize::from(from.body() == ask(&cold, Ask::Frame(n)).body());
            }
        }
        println!("{name}: {kept} of 240 sent from memory, {same} of them byte for byte the frame made by hand");
        assert_eq!(same, kept, "{name}: a frame from memory is not the frame made by hand");
        assert_eq!(kept, keeps, "{name}: frames kept");
    }
}
