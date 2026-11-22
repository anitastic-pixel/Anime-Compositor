use super::*;

/// "yes" when the folder is there, "no" when it is not.
fn there(folder: &Path) -> &'static str {
    if folder.exists() {
        "yes"
    } else {
        "no"
    }
}

/// What `export_into` answered, as words: the folder it will write into, none, or its refusal.
fn answer(result: &Result<Option<PathBuf>, String>) -> String {
    match result {
        Ok(Some(folder)) => format!("into {}", folder.display()),
        Ok(None) => "the folder dialog".into(),
        Err(refused) => format!("refused: {refused}"),
    }
}

fn query(folder: &Path) -> String {
    format!("format=png&into={}", folder.display())
}

#[test]
fn d1140_export_into_only_in_a_window_started_for_testing() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .to_path_buf();
    let base = crate::test_temp().join("d1140_export_into");
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).expect("make the scratch folder");
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut row = |what: &str, expected: &str, actual: String| {
        rows.push((what.into(), expected.into(), actual))
    };
    let refused = |r: &Result<Option<PathBuf>, String>| {
        if r.is_err() {
            "refused".to_string()
        } else {
            answer(r)
        }
    };

    // ---- a window the person started: into= is refused, nothing is made -------------------
    let absent = base.join("not_started_for_testing");
    let said = export_into(Some(&query(&absent)), false);
    row(
        "a person's window: into= is refused",
        "refused",
        refused(&said),
    );
    row(
        "a person's window: the refusal says why",
        "only in a window started for testing",
        match &said {
            Err(s) if s.contains("only in a window started for testing") => {
                "only in a window started for testing".into()
            }
            other => answer(other),
        },
    );
    row(
        "a person's window: the folder named is not made",
        "no",
        there(&absent).into(),
    );
    row(
        "a person's window without into=: the folder dialog, as before",
        "the folder dialog",
        answer(&export_into(Some("format=png"), false)),
    );

    // ---- a window started for testing ------------------------------------------------------
    row(
        "a testing window without into=: the folder dialog, as before",
        "the folder dialog",
        answer(&export_into(Some("format=png"), true)),
    );
    let fresh = base.join("fresh");
    let said = export_into(Some(&query(&fresh)), true);
    row(
        "a testing window, a folder not there yet: accepted",
        &format!("into {}", fresh.display()),
        answer(&said),
    );
    row(
        "a testing window: the window makes the folder",
        "yes",
        there(&fresh).into(),
    );
    let empty = base.join("empty");
    std::fs::create_dir_all(&empty).expect("make the empty folder");
    row(
        "a testing window, an empty folder: accepted",
        &format!("into {}", empty.display()),
        answer(&export_into(Some(&query(&empty)), true)),
    );
    let full = base.join("full");
    std::fs::create_dir_all(&full).expect("make the full folder");
    std::fs::write(full.join("keep.txt"), "kept").expect("write a file to keep");
    row(
        "a testing window, a folder with a file in it: refused",
        "refused",
        refused(&export_into(Some(&query(&full)), true)),
    );
    row(
        "a testing window: the file already there is untouched",
        "kept",
        std::fs::read_to_string(full.join("keep.txt")).unwrap_or_else(|e| e.to_string()),
    );
    row(
        "a testing window, a relative folder: refused",
        "refused",
        refused(&export_into(Some("format=png&into=out%2Fframes"), true)),
    );
    row(
        "a testing window, into= with nothing after it: refused",
        "refused",
        refused(&export_into(Some("format=png&into="), true)),
    );
    let spaced = base.join("with a space");
    let escaped = format!("format=png&into={}", spaced.display()).replace(' ', "%20");
    row(
        "a testing window, a folder name sent escaped (%20): read as the page sends it",
        &format!("into {}", spaced.display()),
        answer(&export_into(Some(&escaped), true)),
    );

    // ---- and the folder is written by the same export job the dialog starts ------------------
    let viewer = demo();
    let (snapshot, root, mut request) =
        export_job(&viewer, &fresh, MissingSource::RenderTransparent, OutputFormat::Png);
    request.last_frame = request.first_frame + 1;
    let said = run_export(&snapshot, &root, &request, &AtomicBool::new(false));
    row(
        "two frames of the reference shot exported into the folder the window made",
        "Exported 2 frames",
        if said.contains("Exported 2 frames") {
            "Exported 2 frames".into()
        } else {
            format!("the window said: {said}")
        },
    );
    let mut files: Vec<String> = std::fs::read_dir(&fresh)
        .map(|d| {
            d.filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    files.sort();
    row(
        "the files in it",
        "the reference shot_0000.png, the reference shot_0001.png",
        files.join(", "),
    );
    let _ = std::fs::remove_dir_all(&base);

    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut text = format!(
        "# D-1140: export into a named folder, only in a window started for testing\n\n\
         Written by `d1140_export_into_only_in_a_window_started_for_testing` in `app/src/main.rs`. \
         `export_into` is what `/export` reads `into=` with; the window is started for testing when \
         `ANIME_COMPOSITOR_TEST_EXPORT` is set. Without `into=` the route asks for the folder in the \
         system's dialog, as before. Folder names below are this machine's scratch folder.\n\n\
         **{passed} of {} checks pass.**\n\n\
         | Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        rows.len()
    );
    let scratch = base.display().to_string();
    for (what, expected, actual) in &rows {
        text.push_str(&format!(
            "| {what} | {} | {} | {} |\n",
            expected.replace(&scratch, "<scratch>"),
            actual.replace(&scratch, "<scratch>"),
            if expected == actual { "PASS" } else { "FAIL" }
        ));
    }
    std::fs::write(repo.join("verification/D-1140_export_into_table.md"), text)
        .expect("write the table");
    assert_eq!(
        passed,
        rows.len(),
        "see verification/D-1140_export_into_table.md"
    );
}
