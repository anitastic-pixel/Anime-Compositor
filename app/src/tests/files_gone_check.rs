use super::*;

/// What the request lists, as "name: file" lines, or "none".
fn listed(viewer: &Mutex<Viewer>) -> String {
    let gone = files_gone(viewer);
    let lines: Vec<String> = gone
        .as_array()
        .expect("files-gone answers a list")
        .iter()
        .map(|f| format!("{}: {}", f["name"].as_str().unwrap_or("?"), f["file"].as_str().unwrap_or("?")))
        .collect();
    if lines.is_empty() { "none".into() } else { lines.join(", ") }
}

#[test]
fn d257_a_deleted_drawing_is_named_and_a_restored_one_is_not() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().expect("the app crate has a parent directory").to_path_buf();
    let copy = repo.join("target/d257_files_gone");
    let _ = std::fs::remove_dir_all(&copy);
    for layer in ["layer1", "layer2", "layer3", "layer4"] {
        std::fs::create_dir_all(copy.join(layer)).expect("make the copy");
        for file in std::fs::read_dir(repo.join("Fixtures/reference_shot").join(layer)).expect("read the reference shot") {
            let file = file.expect("a file").path();
            std::fs::copy(&file, copy.join(layer).join(file.file_name().unwrap())).expect("copy a drawing");
        }
    }
    let viewer = Mutex::new(demo());
    viewer.lock().unwrap().root = copy.clone();
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut row = |what: &str, expected: &str, actual: String| rows.push((what.into(), expected.into(), actual));

    row("the copied reference shot: nothing is gone", "none", listed(&viewer));
    row("drawing 7 of layer3, which never had a file, is a gap and not a gone file", "none", listed(&viewer));
    let drawing = copy.join("layer2/layer2_005.png");
    let kept = std::fs::read(&drawing).expect("read the drawing");
    std::fs::remove_file(&drawing).expect("delete the drawing");
    row("layer2_005.png deleted: it is named, with its footage item", "layer2: layer2/layer2_005.png", listed(&viewer));
    let undo_before = viewer.lock().unwrap().document.undo_labels().len();
    let notes_before = viewer.lock().unwrap().notes.clone();
    files_gone(&viewer);
    let undo_after = viewer.lock().unwrap().document.undo_labels().len();
    let notes_same = viewer.lock().unwrap().notes == notes_before;
    row(
        "asking changes nothing: no undo step, no new note",
        "0 undo steps, notes as before",
        format!("{} undo steps, notes {}", undo_after - undo_before, if notes_same { "as before" } else { "changed" }),
    );
    let gone = files_gone(&viewer);
    row("the answer names the footage item to relink", "asset-layer2", gone[0]["asset"].as_str().unwrap_or("none").to_string());
    std::fs::write(&drawing, kept).expect("put the drawing back");
    row("put back: it is no longer named", "none", listed(&viewer));

    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut text = format!(
        "# D-257: files gone from the disk\n\nWritten by `d257_a_deleted_drawing_is_named_and_a_restored_one_is_not` \
         in `app/src/main.rs`. The reference shot copied to `target/d257_files_gone`, and `files_gone`, the \
         answer to the page's `/files-gone`.\n\n**{passed} of {} checks pass.**\n\n\
         | Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        rows.len()
    );
    for (what, expected, actual) in &rows {
        text.push_str(&format!("| {what} | {expected} | {actual} | {} |\n", if expected == actual { "PASS" } else { "FAIL" }));
    }
    std::fs::write(repo.join("verification/D-257_files_gone_table.md"), text).expect("write the table");
    assert_eq!(passed, rows.len(), "see verification/D-257_files_gone_table.md");
}
