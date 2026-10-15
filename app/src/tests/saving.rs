use super::*;

struct Report {
    rows: Vec<(String, String, String)>,
    /// This machine's temporary directory, which appears in most of the values here and is
    /// different on every machine. CI checks that the committed artifact still matches what
    /// the tests produce, so a real path in a cell would fail the build on the runner for a
    /// reason that has nothing to do with saving.
    scratch: String,
}

impl Report {
    fn check(&mut self, check: &str, expected: impl ToString, actual: impl ToString) {
        let short = |text: String| bare(&text).replace(&self.scratch, "<a temporary directory>");
        self.rows.push((
            check.to_string(),
            short(expected.to_string()),
            short(actual.to_string()),
        ));
    }
}

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .join(rel)
}

/// `text` with this checkout's own directory taken out. Save As writes a drawing outside the
/// new folder as a whole path, which is correct and names wherever the repository happens to
/// be, so a committed artifact has to say it some other way.
fn bare(text: &str) -> String {
    let root = repo("").display().to_string().replace('\\', "/");
    text.replace(root.trim_end_matches('/'), "<the repository>")
}

/// A scratch directory of this test's own, emptied first so a previous run cannot make a
/// later one pass.
fn scratch() -> PathBuf {
    let directory = crate::test_temp().join("anime_compositor_b09_save");
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("make the scratch directory");
    directory
}

fn viewer_on(path: &Path) -> Mutex<Viewer> {
    Mutex::new(open(path).unwrap_or_else(|d| panic!("open {}: {}", path.display(), d.message)))
}

#[test]
fn one_file_imported_on_its_own_is_a_still() {
    let viewer = viewer_on(&repo("Fixtures/projects/unknown_effect_project.json"));
    import(
        &viewer,
        &[repo("Fixtures/reference_shot/layer1/layer1_000.png")],
    );
    let held = viewer.lock().expect("the viewer lock was poisoned");
    let last = held
        .document
        .project()
        .assets
        .last()
        .expect("an asset was added");
    assert_eq!(last.name, "layer1_000.png");
    assert!(
        last.path.is_some() && last.frames.is_empty(),
        "a still, not a sequence"
    );
}

#[test]
fn what_the_window_writes_is_the_project_it_opened() {
    let scratch = scratch();
    let mut report = Report {
        rows: Vec::new(),
        scratch: scratch.display().to_string(),
    };

    // ---- a project this build does not fully understand -----------------------------------
    // Chosen on purpose: `unknown_effect_project.json` names an effect no version of this
    // build has. Everything B-09 promises about saving is visible in what happens to that
    // effect, and nothing else in the fixtures makes the promise checkable.
    let source = repo("Fixtures/projects/unknown_effect_project.json");
    let viewer = viewer_on(&source);

    let elsewhere = scratch.join("saved_elsewhere.json");
    let said = save_as(&viewer, &elsewhere);
    report.check(
        "Save As says where the project went",
        format!("Saved to {}", elsewhere.display()),
        &said,
    );
    report.check(
        "the file the person chose is now on disk",
        true,
        elsewhere.is_file(),
    );

    let written = std::fs::read_to_string(&elsewhere).expect("read what was written");
    report.check(
        "the effect this build does not have is still in the saved file",
        true,
        written.contains("vendor.future.effect"),
    );
    // Not "the file contains the string" twice: this is the whole project's worth of the
    // same question. `to_json` of the reopened file is what a *second* save would write, so
    // agreement means the trip through this build changed nothing at all.
    let reopened = persist::load(&elsewhere).expect("reopen what was written");
    report.check(
        "reopening the saved file and saving it again would write the same bytes",
        bare(&written).len(),
        bare(&persist::to_json(
            reopened.document.project(),
            &reopened.preserved,
        ))
        .len(),
    );
    report.check(
        "and the same text, not merely the same length",
        true,
        persist::to_json(reopened.document.project(), &reopened.preserved) == written,
    );

    // ---- what the window is showing afterwards --------------------------------------------
    {
        let viewer = viewer.lock().expect("the viewer lock was poisoned");
        report.check(
            "after Save As the window is showing the file that was written",
            elsewhere.display().to_string(),
            viewer
                .path
                .as_ref()
                .map_or(String::new(), |p| p.display().to_string()),
        );
        report.check(
            "and calls it by its new name",
            "saved_elsewhere.json",
            &viewer.name,
        );
        report.check(
            "with no unsaved work outstanding",
            false,
            viewer.document.is_dirty(),
        );
        // Media paths in a project file are relative to that file, so this is what decides
        // whether the reopened project can find its drawings at all. A window that kept the
        // old directory would show the picture correctly and hand somebody else a file whose
        // cels resolve to nothing — the failure Save As reopens the file to avoid.
        report.check(
            "and looks for its drawings beside the file it wrote, not beside the one it came \
             from",
            "beside the file it wrote",
            if Some(viewer.root.as_path()) == elsewhere.parent() {
                "beside the file it wrote"
            } else if Some(viewer.root.as_path()) == source.parent() {
                "still beside the project it came from"
            } else {
                "somewhere that is neither"
            },
        );
    }

    // ---- Save, with a file of its own ------------------------------------------------------
    let said = save(&viewer);
    report.check(
        "Save writes to the file the project came from",
        format!("Saved to {}", elsewhere.display()),
        &said,
    );
    report.check(
        "and writing it a second time changes nothing in it",
        written.clone(),
        std::fs::read_to_string(&elsewhere).expect("read it again"),
    );

    // ---- Save, without one -----------------------------------------------------------------
    // The state the window opens in: the built-in reference shot has no file of its own, and
    // Save must ask rather than choose somewhere.
    {
        let mut held = viewer.lock().expect("the viewer lock was poisoned");
        held.path = None;
    }
    report.check(
        "a project with no file of its own is not saved anywhere; the window asks",
        "This project has no file of its own yet. Use Save As.",
        save(&viewer),
    );

    // ---- a save that cannot be done --------------------------------------------------------
    // Document 25's FX-IO-002 in the shape a window can reach: somewhere unwritable. The
    // question is not whether it fails, it is what it leaves behind.
    let viewer = viewer_on(&elsewhere);
    let nowhere = scratch.join("no_such_directory").join("shot.json");
    let said = save_as(&viewer, &nowhere);
    report.check(
        "a save that cannot be done says so in the core's words",
        true,
        said.contains("could not be saved") || said.contains("not be written"),
    );
    report.check(
        "and does not leave a half-written file behind",
        false,
        nowhere.exists(),
    );
    report.check(
        "and the file that was already good is untouched",
        written,
        std::fs::read_to_string(&elsewhere).expect("read the good file"),
    );
    report.check(
        "and the window is still showing the project it had",
        elsewhere.display().to_string(),
        viewer
            .lock()
            .expect("the viewer lock was poisoned")
            .path
            .as_ref()
            .map_or(String::new(), |p| p.display().to_string()),
    );

    // The same failure reached by Ctrl+S instead of Save As. It is a second path through the
    // window and it fails on its own occasions — a project file gone read-only, a drive no
    // longer mounted — which Save As never touches, because Save As is always given a new
    // destination and Save is never given one at all.
    {
        let mut held = viewer.lock().expect("the viewer lock was poisoned");
        held.path = Some(nowhere.clone());
    }
    let said = save(&viewer);
    report.check(
        "and Save fails the same way, rather than naming a file it did not write",
        true,
        said.contains("could not be saved") || said.contains("not be written"),
    );

    write_artifact(&report);
    let failed: Vec<&(String, String, String)> =
        report.rows.iter().filter(|(_, e, a)| e != a).collect();
    assert!(
        failed.is_empty(),
        "{} of {} checks failed, see verification/B-09_save_table.md: {:#?}",
        failed.len(),
        report.rows.len(),
        failed
    );
}

fn write_artifact(report: &Report) {
    let passed = report.rows.iter().filter(|(_, e, a)| e == a).count();
    let mut out = String::new();
    out.push_str("# B-09, saving from the window\n\n");
    out.push_str(
        "The window could open a project and not write one back. It can now, and this is what \
         the writing does. Produced by `cargo test -p anime_compositor_app`, from \
         `app/src/main.rs`.\n\n",
    );
    out.push_str(
        "Every row here is about one promise: **what comes back off the disk is the project \
         that was open, including the parts this build does not understand.** The fixture is \
         `Fixtures/projects/unknown_effect_project.json`, which names an effect no version of \
         this build has. A window that saved only what it could model would drop that effect, \
         the file would still open, the picture would still look right, and the loss would be \
         found much later by the person who made the mask. That is the failure this table \
         exists to catch.\n\n",
    );
    out.push_str("| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
    for (check, expected, actual) in &report.rows {
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            check,
            cell(expected),
            cell(actual),
            if expected == actual { "pass" } else { "FAIL" }
        ));
    }
    out.push_str(&format!(
        "\n**{} of {} checks pass.**\n",
        passed,
        report.rows.len()
    ));
    out.push_str(
        "\n## What this does not cover\n\nThe dialogs. A file dialog belongs to the operating \
         system and a test has no hands to answer one, so what is checked here begins at the \
         path the person chose. Choosing a file, and the Open and Save As dialogs that do the \
         choosing, are still unphotographed; the two photographs beside this table show a \
         Ctrl+S save of a project that already had a file, which is the one path a script can \
         drive from end to end.\n\nWhere a row says *a temporary directory*, \
         the real value was this machine's scratch directory, which is different on every \
         machine and on every run; *the repository* is this checkout's own directory, which \
         Save As writes whole for a drawing that is not under the folder the project moved \
         to. The destination is shown rather than hidden — a save that \
         reports the wrong one is exactly the failure worth seeing — but the machine-specific \
         part of it is not, because this file is committed and checked.\n",
    );
    let path = repo("verification/B-09_save_table.md");
    std::fs::write(path, out).expect("write the artifact");
}

/// A path in a table cell, with the scratch directory's own separators left alone but the
/// table's separator escaped, so one row cannot silently become two columns.
fn cell(text: &str) -> String {
    text.replace('|', r"\|")
}
