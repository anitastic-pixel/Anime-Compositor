use super::*;
use anime_compositor::command::Command;

fn picture(project: &Project, composition: &Id, root: &Path) -> Vec<u8> {
    anime_compositor::compose::render_frame(project, composition, 10, root, DEFAULT_TILE_SIZE, &mut FrameLog::new(0))
        .map(|p| p.to_srgb8_straight())
        .unwrap_or_default()
}

#[test]
fn d256_a_never_saved_project_is_copied_and_offered_again() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().expect("the app crate has a parent directory").to_path_buf();
    let shot = repo.join("Fixtures/reference_shot");
    let folder = repo.join("target/d256_unsaved");
    let elsewhere = repo.join("target/d256_saved_elsewhere");
    let _ = std::fs::remove_dir_all(&folder);
    let _ = std::fs::remove_dir_all(&elsewhere);
    std::fs::create_dir_all(&elsewhere).expect("make the scratch folder");
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut row = |what: &str, expected: &str, actual: String| rows.push((what.into(), expected.into(), actual));

    // A new project, as File > New makes it, with two layers: a drawing from the reference
    // shot and a solid.
    let mut viewer = blank(shot.clone());
    let composition = viewer.composition.clone();
    let reference = demo();
    let drawing = reference.document.project().compositions[0].layers_in_order().find(|l| !l.asset_id.as_str().is_empty()).expect("a drawing layer").clone();
    let asset = reference.document.project().assets.iter().find(|a| a.id == drawing.asset_id).expect("its drawing").clone();
    let solid = Layer::solid(Id::new("layer-solid"), "Solid 1", Solid { color: [0.2, 0.4, 0.9], width: 400, height: 300 }, 1920, 1080, 0, 240);
    for command in [
        Command::AddAsset { asset },
        Command::AddLayer { composition: composition.clone(), layer: Box::new(solid), index: 0 },
        Command::AddLayer { composition: composition.clone(), layer: Box::new(drawing), index: 0 },
    ] {
        viewer.document.apply(command).expect("build the new project");
    }
    let start = Instant::now();
    let at = |seconds: u64| start + Duration::from_secs(seconds);
    let copies = |folder: &Path| persist::recovery_candidates(&unsaved_copy(folder));

    row(
        "with no folder for unsaved copies, nothing is written, as before",
        "nothing",
        autosave_tick_in(&mut viewer, at(7200), None).unwrap_or_else(|| "nothing".into()),
    );
    viewer.dirty_since = None;
    row("the first look at the new, unsaved project writes nothing yet", "nothing", autosave_tick_in(&mut viewer, at(0), Some(&folder)).unwrap_or_else(|| "nothing".into()));
    row("nor 119 seconds later", "nothing", autosave_tick_in(&mut viewer, at(119), Some(&folder)).unwrap_or_else(|| "nothing".into()));
    let said = autosave_tick_in(&mut viewer, at(121), Some(&folder)).unwrap_or_else(|| "nothing".into());
    let first = persist::autosave_path(&unsaved_copy(&folder), 0);
    row(
        "two minutes after the first change, a copy is written into the app's own folder",
        &format!("Recovery snapshot written to {}", first.display()),
        said,
    );
    row("the copy is on the disk", "true", first.exists().to_string());
    let beside_the_drawings = std::fs::read_dir(&shot).expect("read the reference shot").filter_map(Result::ok).filter(|e| e.file_name().to_string_lossy().contains(".autosave-")).count();
    row("nothing is written beside the drawings", "0 copies there", format!("{beside_the_drawings} copies there"));
    row("the project is still unsaved: the copy is not a save", "true", viewer.document.is_dirty().to_string());
    for n in 2..=7u64 {
        autosave_tick_in(&mut viewer, at(121 * n), Some(&folder));
    }
    row("seven copies later, five are kept", "5", copies(&folder).len().to_string());

    // The window closes without saving. The next start opens the reference shot, which has no
    // file of its own either, and looks in the app's folder.
    let newest = copies(&folder).first().map(|c| c.path.clone()).unwrap_or_default();
    let mut restarted = demo();
    offer_unsaved(&mut restarted, &folder);
    row(
        "the next start offers the copies on the recovery list, newest first",
        &format!("5, newest {}", newest.display()),
        format!("{}, newest {}", restarted.recovery.len(), restarted.recovery.first().map(|p| p.display().to_string()).unwrap_or_default()),
    );
    row(
        "and says so",
        "true",
        restarted.notes.iter().any(|n| n.contains("recovery snapshots")).to_string(),
    );
    let restarted = Mutex::new(restarted);
    row(
        "a snapshot from anywhere else is still refused, as before",
        "There is no project to recover into.",
        recover_in(&restarted, &repo.join("verification/B-08a_project.json"), Some(&folder)),
    );
    row(
        "recovering the newest copy",
        &format!("Recovered {}", newest.display()),
        recover_in(&restarted, &newest, Some(&folder)),
    );
    let recovered = restarted.lock().unwrap();
    row(
        "gives an unsaved project: no file of its own, and work outstanding",
        "no file, unsaved",
        format!("{}, {}", if recovered.path.is_none() { "no file" } else { "a file" }, if recovered.document.is_dirty() { "unsaved" } else { "saved" }),
    );
    let saved = |project: &Project, root: &Path| persist::to_json(&persist::rebased(project, root, &elsewhere), &Preserved::none());
    let original = saved(viewer.document.project(), &viewer.root);
    let again = saved(recovered.document.project(), &recovered.root);
    row(
        "saved into the same folder, it is the same project as the original, byte for byte",
        &format!("{} bytes, the same", original.len()),
        if again == original { format!("{} bytes, the same", again.len()) } else { format!("{} bytes against {}, different", again.len(), original.len()) },
    );
    let before = picture(viewer.document.project(), &composition, &viewer.root);
    let after = picture(recovered.document.project(), &recovered.composition, &recovered.root);
    row(
        "and its frame 10 is the original's, every byte",
        &format!("{} bytes, the same", before.len()),
        if !before.is_empty() && after == before { format!("{} bytes, the same", after.len()) } else { format!("{} bytes against {}, different", after.len(), before.len()) },
    );
    drop(recovered);
    forget_unsaved(&folder);
    row("once it has been saved, the copies are removed", "0", copies(&folder).len().to_string());

    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut text = format!(
        "# D-256: recovery copies of a project that was never saved\n\nWritten by \
         `d256_a_never_saved_project_is_copied_and_offered_again` in `app/src/main.rs`. The app's own folder is \
         stood in for by `target/d256_unsaved`; the window uses `%LOCALAPPDATA%\\dev.anitastic.anime-compositor\\Unsaved`.\n\n\
         **{passed} of {} checks pass.**\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        rows.len()
    );
    let tidy = |s: &str| s.replace(&repo.display().to_string(), "").replace(['|', '\\'], "/");
    for (what, expected, actual) in &rows {
        text.push_str(&format!("| {what} | {} | {} | {} |\n", tidy(expected), tidy(actual), if expected == actual { "PASS" } else { "FAIL" }));
    }
    std::fs::write(repo.join("verification/D-256_unsaved_recovery_table.md"), text).expect("write the table");
    assert_eq!(passed, rows.len(), "see verification/D-256_unsaved_recovery_table.md");
}
