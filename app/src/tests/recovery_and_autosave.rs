use super::*;
use anime_compositor::command::Command;

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .join(rel)
}

/// A copy of a fixture in a scratch directory of its own, emptied first. A copy because
/// autosave writes beside the project, and `Fixtures/` is not somewhere a test may write.
fn a_project_to_work_on() -> PathBuf {
    let directory = crate::test_temp().join("anime_compositor_b09_recovery");
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("make the scratch directory");
    let path = directory.join("shot.json");
    std::fs::copy(repo("Fixtures/projects/unknown_effect_project.json"), &path)
        .expect("copy the fixture");
    path
}

/// Rename the one layer, which is the smallest real change to a project this build can make.
fn change_something(viewer: &mut Viewer, to: &str) {
    let composition = viewer.composition.clone();
    let layer_id = viewer.document.project().compositions[0].layer_order()[0].clone();
    viewer
        .document
        .apply(Command::RenameLayer {
            composition,
            layer_id,
            name: to.to_string(),
        })
        .expect("rename the layer");
}

fn layer_name(viewer: &Viewer) -> String {
    let composition = &viewer.document.project().compositions[0];
    composition
        .layer(&composition.layer_order()[0])
        .expect("the layer the order names")
        .name
        .clone()
}

#[test]
fn nothing_is_written_early_nothing_is_lost_late() {
    let project = a_project_to_work_on();
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut check = |what: &str, expected: &dyn ToString, actual: &dyn ToString| {
        rows.push((what.to_string(), expected.to_string(), actual.to_string()));
    };

    let untouched = std::fs::read_to_string(&project).expect("read the project");
    let viewer = Mutex::new(open(&project).expect("open the project"));
    let start = Instant::now();

    // ---- the timer ------------------------------------------------------------------------
    {
        let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
        check(
            "a project with nothing outstanding writes no snapshot, however long it sits",
            &"nothing",
            &autosave_tick(viewer, start + Duration::from_secs(3600))
                .unwrap_or_else(|| "nothing".to_string()),
        );

        change_something(viewer, "Renamed while nobody was saving");
        check(
            "there is now unsaved work",
            &true,
            &viewer.document.is_dirty(),
        );
        check(
            "one minute of unsaved work is not enough (document 07 asks for two)",
            &"nothing",
            &autosave_tick(viewer, start + Duration::from_secs(60))
                .unwrap_or_else(|| "nothing".to_string()),
        );
        check(
            "and no file has appeared beside the project",
            &false,
            &persist::autosave_path(&project, 0).exists(),
        );

        // Two minutes after the tick above, which is when the timer first saw the work: the
        // clock starts at the sight of it, not at the moment it was done.
        let said = autosave_tick(viewer, start + Duration::from_secs(181))
            .unwrap_or_else(|| "nothing".to_string());
        check(
            "after two minutes a snapshot is written, in the first free slot",
            &format!(
                "Recovery snapshot written to {}",
                persist::autosave_path(&project, 0).display()
            ),
            &said,
        );
        check(
            "the work is still unsaved afterwards (document 26)",
            &true,
            &viewer.document.is_dirty(),
        );
        check(
            "and the project file has not been touched (document 07)",
            &"unchanged",
            &if std::fs::read_to_string(&project).expect("read the project") == untouched {
                "unchanged"
            } else {
                "changed"
            },
        );
        check(
            "the snapshot keeps the effect this build does not understand",
            &true,
            &std::fs::read_to_string(persist::autosave_path(&project, 0))
                .expect("read the snapshot")
                .contains("vendor.future.effect"),
        );

        // Five more, each two minutes after the last. The sixth has to reuse a slot.
        for minute in 1..=5 {
            change_something(viewer, &format!("Renamed again, {minute}"));
            autosave_tick(viewer, start + Duration::from_secs(181 + 121 * minute))
                .expect("a snapshot every two minutes");
        }
        check(
            "six snapshots leave five files, not six",
            &persist::AUTOSAVE_SLOTS,
            &persist::recovery_candidates(&project).len(),
        );
        check(
            "and the window is offering all five to recover from",
            &persist::AUTOSAVE_SLOTS,
            &viewer.recovery.len(),
        );

        // Every row above ticks at a moment that is itself the timer's first sight of the
        // work, and the clock starts at that sight — so they read zero and would pass for
        // any waiting time at all, including none. These three straddle the boundary
        // instead: the last tick of the loop above was at 181 + 121 * 5, and the work is
        // still unsaved, so the clock is running from there.
        let last = 181 + 121 * 5;
        check(
            "one look short of two minutes writes nothing",
            &"nothing",
            &autosave_tick(viewer, start + Duration::from_secs(last + 119))
                .unwrap_or_else(|| "nothing".to_string()),
        );
        let said = autosave_tick(viewer, start + Duration::from_secs(last + 121))
            .unwrap_or_else(|| "nothing".to_string());
        check(
            "and one look past it writes a snapshot",
            &"a snapshot",
            &if said.starts_with("Recovery snapshot written to") {
                "a snapshot"
            } else {
                said.as_str()
            },
        );
        check(
            "and the two minutes start again there, so ten seconds later there is nothing",
            &"nothing",
            &autosave_tick(viewer, start + Duration::from_secs(last + 131))
                .unwrap_or_else(|| "nothing".to_string()),
        );
    }

    // ---- recovering -------------------------------------------------------------------------
    let newest = persist::recovery_candidates(&project)[0].path.clone();
    let said = recover(&viewer, &newest);
    check(
        "recovering says which snapshot was opened",
        &format!("Recovered {}", newest.display()),
        &said,
    );
    {
        let viewer = viewer.lock().expect("the viewer lock was poisoned");
        check(
            "the recovered work is what was in the snapshot",
            &"Renamed again, 5",
            &layer_name(&viewer),
        );
        check(
            "Save would write to the project, not back into the snapshot",
            &project.display().to_string(),
            &viewer
                .path
                .as_ref()
                .map_or(String::new(), |p| p.display().to_string()),
        );
        check(
            "and the window calls the project by its own name",
            &"shot.json",
            &viewer.name,
        );
        check(
            "the recovered work counts as unsaved, because the project file does not have it",
            &true,
            &viewer.document.is_dirty(),
        );
        // The note is the only thing on screen that says the project file has not been
        // written yet. Without it a recovered window is indistinguishable from a saved one.
        check(
            "and the window says on screen that this is a snapshot and the project is untouched",
            &true,
            &viewer.notes.iter().any(|note| {
                note.contains(&newest.display().to_string())
                    && note.contains("Nothing has been written to")
            }),
        );
    }
    // Compared whole, reported as a word: the fixture is two hundred lines and a table cell
    // holding it twice is a table nobody reads.
    check(
        "recovering wrote nothing: the project file is still byte for byte what it was",
        &"unchanged",
        &if std::fs::read_to_string(&project).expect("read the project") == untouched {
            "unchanged"
        } else {
            "changed"
        },
    );

    // ---- and then saving it ------------------------------------------------------------------
    let said = save(&viewer);
    check(
        "saving after a recovery writes the project",
        &format!("Saved to {}", project.display()),
        &said,
    );
    check(
        "the project file now holds the recovered work",
        &true,
        &std::fs::read_to_string(&project)
            .expect("read the project")
            .contains("Renamed again, 5"),
    );
    {
        let viewer = viewer.lock().expect("the viewer lock was poisoned");
        check(
            "and there is nothing outstanding any more",
            &false,
            &viewer.document.is_dirty(),
        );
    }

    // ---- the case with nowhere to put anything -------------------------------------------------
    {
        let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
        viewer.path = None;
        change_something(viewer, "Changed with no file to save to");
        check(
            "a project with no file of its own writes no snapshot; there is nowhere beside it",
            &"nothing",
            &autosave_tick(viewer, start + Duration::from_secs(7200))
                .unwrap_or_else(|| "nothing".to_string()),
        );
        // The tick above is the first sight of this change, so its clock reads zero and it
        // would write nothing wherever the project lived. This one is two minutes later,
        // which is the tick that would put the snapshot somewhere chosen for it.
        check(
            "and none two minutes later either, when there would otherwise be one",
            &"nothing",
            &autosave_tick(viewer, start + Duration::from_secs(7200 + 121))
                .unwrap_or_else(|| "nothing".to_string()),
        );
    }
    check(
        "and it cannot be recovered into either",
        &"There is no project to recover into.",
        &recover(&viewer, &newest),
    );

    let scratch = project
        .parent()
        .expect("the project has a directory")
        .display()
        .to_string();
    write_artifact(&rows, &scratch);
    let failed: Vec<&(String, String, String)> = rows.iter().filter(|(_, e, a)| e != a).collect();
    assert!(
        failed.is_empty(),
        "{} of {} checks failed, see verification/B-09_recovery_table.md: {:#?}",
        failed.len(),
        rows.len(),
        failed
    );
}

fn write_artifact(rows: &[(String, String, String)], scratch: &str) {
    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut out = String::new();
    out.push_str("# B-09, autosave and recovery in the window\n\n");
    out.push_str(
        "The core has known how to write a recovery snapshot for some time; until now nothing \
         called it. This is the window's half — a clock that decides when, and a way back in \
         from a snapshot. Produced by `cargo test -p anime_compositor_app`, from \
         `app/src/main.rs`.\n\n",
    );
    out.push_str(
        "The promise is document 07's, in two parts. **A snapshot is written after two \
         minutes of unsaved work, and it never overwrites the last manual save.** And **\
         recovering from a snapshot does not save it**: the window comes back pointing at the \
         project file with the recovered work outstanding, so the person decides whether it \
         becomes the project. The two minutes are a parameter here rather than a wait, which \
         is the only thing about this table that is not what the window does.\n\n",
    );
    out.push_str("| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
    for (check, expected, actual) in rows {
        let short = |text: &str| {
            text.replace(scratch, "<a temporary directory>")
                .replace('|', r"\|")
        };
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            check,
            short(expected),
            short(actual),
            if expected == actual { "pass" } else { "FAIL" }
        ));
    }
    out.push_str(&format!(
        "\n**{} of {} checks pass.**\n",
        passed,
        rows.len()
    ));
    out.push_str(
        "\n## What this does not cover\n\nThe two minutes passing. The timer is a thread that \
         sleeps ten seconds at a time and asks the same question this table asks; what is \
         checked here is the question, with the clock supplied. A thread that never started \
         would not fail this table, and only the running window shows that it did.\n\n\
         **Nothing in this window makes a project dirty yet.** It is a viewer: it opens, \
         shows, plays and saves, and no control in it changes a project. So in ordinary use \
         today the timer has nothing to write, and it stays quiet. What it protects is the \
         editing that B-13 onwards adds, and it is built now because the alternative is \
         building it after the first afternoon somebody loses.\n\n\
         Where a row says *a temporary directory*, the real value was this machine's scratch \
         directory, which differs on every machine and every run.\n",
    );
    std::fs::write(repo("verification/B-09_recovery_table.md"), out).expect("write the artifact");
}
