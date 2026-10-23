use super::*;

struct Report {
    rows: Vec<(String, String, String)>,
}

impl Report {
    fn check(&mut self, check: &str, expected: impl ToString, actual: impl ToString) {
        self.rows
            .push((check.to_string(), expected.to_string(), actual.to_string()));
    }
}

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .join(rel)
}

/// One command, by the document 24 identifier the page would send.
///
/// `expect` rather than a fallback: `None` means the identifier is not one this window
/// answers, which is a mistake in the test and not a result worth tabulating.
fn run(viewer: &Mutex<Viewer>, what: &str) -> String {
    let (id, query) = match what.split_once('?') {
        Some((id, query)) => (id, Some(query)),
        None => (what, None),
    };
    edit_command(viewer, id, query).expect("a command document 24 lists")
}

fn held(viewer: &Mutex<Viewer>) -> std::sync::MutexGuard<'_, Viewer> {
    viewer.lock().expect("the viewer lock was poisoned")
}

/// The layers of the composition on screen, front last, as names.
fn names(viewer: &Mutex<Viewer>) -> String {
    let held = held(viewer);
    let comp = held
        .document
        .project()
        .composition(&held.composition)
        .expect("the composition on screen");
    comp.layers_in_order()
        .map(|l| l.name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

fn layer_ids(viewer: &Mutex<Viewer>) -> String {
    let held = held(viewer);
    let comp = held
        .document
        .project()
        .composition(&held.composition)
        .expect("the composition on screen");
    comp.layer_order()
        .iter()
        .map(|id| id.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

fn layer<T>(viewer: &Mutex<Viewer>, id: &str, read: impl FnOnce(&Layer) -> T) -> Option<T> {
    let held = held(viewer);
    let comp = held
        .document
        .project()
        .composition(&held.composition)
        .expect("the composition on screen");
    comp.layer(&Id::new(id)).map(read)
}

/// ADR-008: an effect this build does not have keeps its settings and its unknown record on
/// every copy the window makes of its layer -- Duplicate, Split, and Copy, Delete, Paste.
#[test]
fn an_unknown_effect_travels_with_every_copy_of_its_layer() {
    let source = repo("Fixtures/projects/unknown_effect_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    run(&viewer, "layer.duplicate?layer=layer-cel");
    run(&viewer, "layer.split?layer=layer-cel&frame=2");
    run(&viewer, "layer.copy?layer=layer-cel");
    run(&viewer, "layer.delete?layer=layer-cel");
    run(&viewer, "layer.paste");
    let saved: serde_json::Value = {
        let held = held(&viewer);
        serde_json::from_str(&persist::to_json(held.document.project(), &held.preserved)).unwrap()
    };
    let layers = saved["compositions"][0]["layers"]
        .as_array()
        .expect("layers");
    assert_eq!(
        layers.len(),
        3,
        "the duplicate, the split-off half and the pasted copy"
    );
    for layer in layers {
        let effect = &layer["effects"][0];
        assert_eq!(
            effect["parameters"],
            serde_json::json!({"strength": 0.5}),
            "{}",
            layer["id"]
        );
        assert_eq!(
            effect["opaque_unknown_data"],
            serde_json::json!({"keep_me": true}),
            "{}",
            layer["id"]
        );
    }
}

#[test]
fn the_panels_change_the_document_and_only_through_commands() {
    let mut report = Report { rows: Vec::new() };

    // The fixture names an effect no version of this build has. Every check below runs on a
    // project this build only partly understands, on purpose: a panel is a second reader of
    // the project, and a second reader is a second chance to drop what it cannot model.
    let source = repo("Fixtures/projects/unknown_effect_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );

    // ---- what the page is given ----------------------------------------------------------
    let answer: serde_json::Value =
        serde_json::from_str(&state(&viewer)).expect("the state answer is JSON");
    {
        let held = held(&viewer);
        let written = persist::to_json(held.document.project(), &held.preserved);
        let same: serde_json::Value =
            serde_json::from_str(&written).expect("what a save would write is JSON");
        // Not "the panels got a project" but "the panels got *this* project". If the two
        // ever differ, the window is drawing one thing and saving another, and the person
        // finds out when they reopen the file.
        report.check(
            "the project the panels draw is the project a save would write",
            true,
            answer["project"] == same,
        );
    }
    report.check(
        "an effect this build does not have reaches the panels rather than vanishing",
        true,
        state(&viewer).contains("vendor.future.effect"),
    );
    report.check(
        "the panels are told which composition is on screen",
        "comp-main",
        answer["composition"].as_str().unwrap_or("(none)"),
    );
    report.check(
        "a project just opened has nothing to undo",
        "0 to undo, 0 to redo",
        format!(
            "{} to undo, {} to redo",
            answer["undo"].as_array().map_or(0, |a| a.len()),
            answer["redo"].as_array().map_or(0, |a| a.len())
        ),
    );

    // ---- adding a layer ------------------------------------------------------------------
    report.check(
        "adding a layer without saying which drawing is refused, and asks",
        "Which drawing should the new layer show?",
        run(&viewer, "layer.create"),
    );
    report.check(
        "a drawing that is not in the project is refused by name",
        "There is no imported drawing called asset-nothing.",
        run(&viewer, "layer.create?asset=asset-nothing"),
    );
    report.check(
        "neither refusal put anything in the undo history",
        0,
        held(&viewer).document.undo_depth(),
    );

    run(&viewer, "layer.create?asset=asset-cel&name=Shadow");
    report.check(
        "a new layer is added in front of the ones already there",
        "Cel, Shadow",
        names(&viewer),
    );
    report.check(
        "its identifier is not one the project was already using",
        "layer-cel, layer-1",
        layer_ids(&viewer),
    );
    report.check(
        "it covers the whole composition, which is five frames from zero",
        "0 to 5",
        layer(&viewer, "layer-1", |l| {
            format!("{} to {}", l.in_frame, l.out_frame)
        })
        .unwrap_or_default(),
    );
    report.check(
        "D-266: a drawing whose files are not on this machine has no size to centre by, so it keeps the top left",
        "anchor Vec2(0.0, 0.0), position Vec2(0.0, 0.0)",
        layer(&viewer, "layer-1", |l| {
            format!("anchor {:?}, position {:?}", l.transform.anchor.base(), l.transform.position.base())
        })
        .unwrap_or_default(),
    );
    report.check(
        "D-292: the sequence's two drawings are exposed on ones from its first frame, so it shows at once",
        "[(0, 1, 1), (1, 2, 2)]",
        layer(&viewer, "layer-1", |l| {
            format!("{:?}", l.exposure_spans.iter()
                .map(|s| (s.start_frame, s.end_frame_exclusive, s.drawing_number))
                .collect::<Vec<_>>())
        })
        .unwrap_or_default(),
    );
    run(&viewer, "layer.create?asset=asset-cel&name=Highlight");
    report.check(
        "a second new layer gets an identifier of its own",
        "layer-cel, layer-1, layer-2",
        layer_ids(&viewer),
    );

    // ---- naming --------------------------------------------------------------------------
    report.check(
        "renaming with no name is refused",
        "A layer needs a name.",
        run(&viewer, "layer.rename?layer=layer-1"),
    );
    run(&viewer, "layer.rename?layer=layer-1&name=Cast shadow");
    report.check(
        "renaming changes the name and nothing else",
        "Cel, Cast shadow, Highlight",
        names(&viewer),
    );
    run(&viewer, "layer.rename?layer=layer-1&name=%E5%BD%B1");
    report.check(
        "a name in Japanese survives the trip through the query string",
        "\u{5f71}",
        layer(&viewer, "layer-1", |l| l.name.clone()).unwrap_or_default(),
    );

    // ---- hiding and locking ----------------------------------------------------------------
    // Document 24 calls both of these a toggle, so what they do depends on what the layer is
    // now, not on what the page last drew. Each is pressed twice and must come back.
    run(&viewer, "layer.toggle_visibility?layer=layer-1");
    report.check(
        "hiding a visible layer switches it off",
        false,
        layer(&viewer, "layer-1", |l| l.enabled).unwrap_or(true),
    );
    run(&viewer, "layer.toggle_visibility?layer=layer-1");
    report.check(
        "and pressing it again switches it back on",
        true,
        layer(&viewer, "layer-1", |l| l.enabled).unwrap_or(false),
    );
    run(&viewer, "layer.toggle_lock?layer=layer-1");
    report.check(
        "locking a layer locks it",
        true,
        layer(&viewer, "layer-1", |l| l.locked).unwrap_or(false),
    );
    run(&viewer, "layer.toggle_lock?layer=layer-1");
    report.check(
        "and pressing it again unlocks it",
        false,
        layer(&viewer, "layer-1", |l| l.locked).unwrap_or(true),
    );

    // ---- order ------------------------------------------------------------------------------
    run(&viewer, "layer.move_up?layer=layer-1");
    report.check(
        "moving a layer forward puts it one place nearer the front",
        "Cel, Highlight, \u{5f71}",
        names(&viewer),
    );
    run(&viewer, "layer.move_down?layer=layer-1");
    report.check(
        "moving it back puts it where it was",
        "Cel, \u{5f71}, Highlight",
        names(&viewer),
    );
    report.check(
        "the front layer cannot go further forward, and is told so by name",
        "Highlight is already at the front.",
        run(&viewer, "layer.move_up?layer=layer-2"),
    );
    report.check(
        "the back layer cannot go further back, and is told so by name",
        "Cel is already at the back.",
        run(&viewer, "layer.move_down?layer=layer-cel"),
    );
    report.check(
        "neither refusal put an entry in the history that would undo nothing",
        "Cel, \u{5f71}, Highlight",
        names(&viewer),
    );

    // W-22: a row dragged to a place in the list, sent once at the drop.
    run(&viewer, "layer.move?layer=layer-cel&to=2");
    report.check(
        "a layer dragged to the front lands there in one step",
        "\u{5f71}, Highlight, Cel",
        names(&viewer),
    );
    run(&viewer, "edit.undo");
    report.check(
        "and one undo puts it back",
        "Cel, \u{5f71}, Highlight",
        names(&viewer),
    );
    report.check(
        "a drop without a place is refused, and asks",
        "Where to? Send the position in the stack, counted from nought at the back.",
        run(&viewer, "layer.move?layer=layer-1"),
    );
    report.check(
        "a drop where the layer already is changes nothing, and says so by name",
        "\u{5f71} is already at position 1.",
        run(&viewer, "layer.move?layer=layer-1&to=1"),
    );
    report.check(
        "a place past the end of the stack is refused by the core",
        "Position 3 is past the end of a stack of 3 layers.",
        run(&viewer, "layer.move?layer=layer-1&to=3"),
    );

    // W-22: Ctrl+D and Ctrl+Shift+D.
    run(&viewer, "layer.duplicate?layer=layer-1");
    report.check(
        "a duplicate goes in front of the layer it copies, under the same name",
        "Cel, \u{5f71}, \u{5f71}, Highlight",
        names(&viewer),
    );
    run(&viewer, "edit.undo");
    report.check(
        "and one undo takes it away",
        "Cel, \u{5f71}, Highlight",
        names(&viewer),
    );
    report.check(
        "a split on the layer's first frame is refused, and says where it can go",
        "\u{5f71} runs from frame 0 to 4, so it can only be split on a frame after its first \
         and before its end.",
        run(&viewer, "layer.split?layer=layer-1&frame=0"),
    );
    let depth = held(&viewer).document.undo_depth();
    run(&viewer, "layer.split?layer=layer-1&frame=2");
    let spans = |viewer: &Mutex<Viewer>| {
        let held = held(viewer);
        let comp = held
            .document
            .project()
            .composition(&held.composition)
            .expect("the composition on screen");
        comp.layers_in_order()
            .map(|l| format!("{} {}-{}", l.name, l.in_frame, l.out_frame))
            .collect::<Vec<_>>()
            .join(", ")
    };
    report.check(
        "a split leaves the layer ending where the copy in front of it begins",
        "Cel 0-5, \u{5f71} 0-2, \u{5f71} 2-5, Highlight 0-5",
        spans(&viewer),
    );
    report.check(
        "and is one entry to undo",
        depth + 1,
        held(&viewer).document.undo_depth(),
    );
    run(&viewer, "edit.undo");
    report.check(
        "which puts the one layer back",
        "Cel 0-5, \u{5f71} 0-5, Highlight 0-5",
        spans(&viewer),
    );

    // ---- a list older than the document ------------------------------------------------------
    report.check(
        "a command naming a layer that is not there is refused by name",
        "layer-gone is not a layer in this composition.",
        run(&viewer, "layer.delete?layer=layer-gone"),
    );
    report.check(
        "a command naming no layer at all asks which one",
        "Which layer? Choose one in the layer list.",
        run(&viewer, "layer.delete"),
    );

    // ---- undo ---------------------------------------------------------------------------------
    let before_delete = names(&viewer);
    let depth = held(&viewer).document.undo_depth();
    run(&viewer, "layer.delete?layer=layer-1");
    report.check(
        "deleting a layer removes it",
        "Cel, Highlight",
        names(&viewer),
    );
    report.check(
        "and the two refusals before it left the history exactly as deep as it was",
        depth + 1,
        held(&viewer).document.undo_depth(),
    );
    let answer: serde_json::Value =
        serde_json::from_str(&state(&viewer)).expect("the state answer is JSON");
    report.check(
        "the Undo button is told what it would take back",
        "Delete layer layer-1",
        answer["undo"]
            .as_array()
            .and_then(|a| a.last())
            .and_then(|v| v.as_str())
            .unwrap_or("(nothing)"),
    );
    report.check(
        "undoing says what it took back",
        "Undone: Delete layer layer-1",
        undo(&viewer),
    );
    report.check(
        "and the layer is back where it was, with the name it had",
        before_delete,
        names(&viewer),
    );
    report.check(
        "redoing says what it put back",
        "Redone: Delete layer layer-1",
        redo(&viewer),
    );
    report.check(
        "and the layer is gone again",
        "Cel, Highlight",
        names(&viewer),
    );

    // ---- dirty and clean -----------------------------------------------------------------------
    report.check(
        "a document that has been edited is unsaved work",
        true,
        held(&viewer).document.is_dirty(),
    );
    // Document 26: dirty is a comparison against the last save, not a flag, so undoing all
    // the way back to the file makes it clean again. Nothing in the window decides this.
    while held(&viewer).document.undo_depth() > 0 {
        undo(&viewer);
    }
    report.check(
        "undoing back to the file makes it not unsaved work again",
        false,
        held(&viewer).document.is_dirty(),
    );
    report.check(
        "and the composition is the one that was opened",
        "Cel",
        names(&viewer),
    );
    report.check(
        "there is nothing left to undo, and it says so",
        "There is nothing to undo.",
        undo(&viewer),
    );

    // ---- and out to the disk ---------------------------------------------------------------------
    // The last question, and the one the others are for: after all of that, is what a save
    // would write still the file that was opened? The effect this build cannot model is in
    // that comparison, so this is also the check that no panel dropped it along the way.
    {
        let held = held(&viewer);
        let opened = std::fs::read_to_string(&source)
            .expect("read the fixture")
            .replace("\r\n", "\n");
        let now = persist::to_json(held.document.project(), &held.preserved);
        // Both texts are thousands of characters and a table is for reading, so the row
        // carries the answer rather than the two files.
        let same = "identical, including the effect this build cannot model";
        report.check(
            "after all of the above, a save would write the file that was opened",
            same,
            if opened == now {
                same
            } else {
                "what a save would write is no longer what was opened"
            },
        );
    }

    write_artifact(
        &report,
        "verification/B-12a_editing_table.md",
        "B-12a: what the layer panel does",
        LAYER_INTRO,
        LAYER_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// One table with its own prose around it.
///
/// Two tests in this module write one each, and everything they have in common - the header
/// row, the escaping, the count at the bottom - is written here rather than twice.
fn write_artifact(report: &Report, file: &str, title: &str, intro: &[&str], notes: &[&str]) {
    let passed = report.rows.iter().filter(|(_, e, a)| e == a).count();
    let mut out = format!("# {title}\n\n");
    out.push_str(
        "Generated by `app/src/main.rs`, module `editing`. Re-run with `cargo test \
         --workspace`.\n\n",
    );
    for paragraph in intro.iter().chain(notes) {
        out.push_str(paragraph);
        out.push_str("\n\n");
    }
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
    std::fs::write(repo(file), out).expect("write the artifact");
}

/// D-732: Increment and Save (Ctrl+Alt+Shift+S).
#[test]
fn increment_and_save_writes_the_next_version_and_works_on_it() {
    let mut report = Report { rows: Vec::new() };
    let folder = crate::test_temp().join("anime_compositor_d732");
    let _ = std::fs::remove_dir_all(&folder);
    std::fs::create_dir_all(&folder).expect("make the scratch folder");
    let name = |p: PathBuf| p.file_name().unwrap().to_string_lossy().into_owned();

    for (from, to) in [
        ("shot.json", "shot_v002.json"),
        ("shot_v002.json", "shot_v003.json"),
        ("shot_v009.json", "shot_v010.json"),
        ("shot_v9.json", "shot_v010.json"),
        ("shot_v1_final.json", "shot_v1_final_v002.json"),
        ("shot_v.json", "shot_v_v002.json"),
    ] {
        report.check(
            &format!("the next version of `{from}` is called"),
            to,
            name(incremented(&folder.join(from))),
        );
    }
    std::fs::write(folder.join("shot_v003.json"), "{}").expect("write a taken name");
    std::fs::write(folder.join("shot_v004.json"), "{}").expect("write a taken name");
    report.check(
        "with shot_v003 and shot_v004 already on disk, the next version of `shot_v002.json` is called",
        "shot_v005.json",
        name(incremented(&folder.join("shot_v002.json"))),
    );

    let viewer = Mutex::new(demo());
    report.check(
        "a project with no file yet has nothing to count up from, so the window asks where to save",
        "asks",
        increment_and_save(&viewer).map_or("asks".to_string(), |said| said),
    );
    let first = folder.join("scene.json");
    save_as(&viewer, &first);
    let before = std::fs::read(&first).expect("read the first file");
    let said = increment_and_save(&viewer).unwrap_or_default();
    let second = folder.join("scene_v002.json");
    let shown = |text: &str| text.replace(&folder.display().to_string(), "<scratch folder>");
    report.check(
        "pressing it once says",
        shown(&format!("Saved to {}", second.display())),
        shown(&said),
    );
    report.check("scene_v002.json is on disk", true, second.is_file());
    report.check(
        "the window now works on",
        "scene_v002.json",
        viewer
            .lock()
            .unwrap()
            .path
            .clone()
            .map(name)
            .unwrap_or_default(),
    );
    report.check(
        "scene.json is left exactly as it was",
        true,
        std::fs::read(&first).ok() == Some(before),
    );
    increment_and_save(&viewer);
    report.check(
        "pressing it again, the window works on",
        "scene_v003.json",
        viewer
            .lock()
            .unwrap()
            .path
            .clone()
            .map(name)
            .unwrap_or_default(),
    );
    report.check(
        "the copies hold the same project",
        true,
        std::fs::read_to_string(&second).ok()
            == std::fs::read_to_string(folder.join("scene_v003.json")).ok(),
    );

    write_artifact(
        &report,
        "verification/D-732_increment_save_table.md",
        "D-732: Increment and Save (Ctrl+Alt+Shift+S)",
        &[
            "After Effects' File > Increment and Save: the project is saved under its name with \
           the next version number, and the window carries on with that new file. A name with \
           no `_v` number gains `_v002`; a number already used by a file in the folder is \
           skipped, so an older version is never written over. Under the hood it is Save As \
           to a name the window picks, so everything Save As promises (media paths kept, \
           unknown data kept) holds here too.",
        ],
        &[],
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// D-733: previous and next drawing (Shift+, and Shift+.). The page reads the selected
/// layer's Sheet column and goes to the nearest frame before or after the playhead whose cell
/// is a drawing number or a blank's start. This asks the window for the same Sheet, applies
/// the rule pinned below, and checks it against frames worked out by hand from the exposures
/// written in `verification/B-08a_project.json`.
#[test]
fn previous_and_next_drawing_land_where_the_sheet_changes() {
    let mut report = Report { rows: Vec::new() };
    let page = include_str!("../../ui/index.html");
    report.check(
        "the page counts a cell as a change unless it is outside the layer, a hold, or a blank held",
        true,
        page.contains("(mark === null || mark === '|' || mark === '' ? [] : [grid.from + i])"),
    );
    report.check(
        "Shift+, and Shift+. reach it",
        true,
        page.contains("e.shiftKey && (e.code === 'Comma' || e.code === 'Period')) { e.preventDefault(); toDrawing(e.code === 'Period'); }"),
    );
    let viewer = Mutex::new(demo());
    // A blank on layer 2 from frame 20 to 23, so that a blank's start is tried too.
    for f in 20..24 {
        run(
            &viewer,
            &format!("exposure.set_span?layer=layer-2&was={f}&start={f}&drawing="),
        );
    }
    let grid = sheet_grid(&viewer);
    let from = grid["from"].as_i64().unwrap_or(0);
    let column = |layer: &str| {
        grid["columns"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|c| c["layer"] == layer)
            .cloned()
            .unwrap_or_default()
    };
    let lands = |layer: &str, at: i64, later: bool| -> String {
        let changes: Vec<i64> = column(layer)["cells"]
            .as_array()
            .into_iter()
            .flatten()
            .enumerate()
            .filter(|(_, c)| !matches!(c.as_str(), None | Some("|") | Some("")))
            .map(|(i, _)| from + i as i64)
            .collect();
        let found = if later {
            changes.iter().filter(|&&f| f > at).min()
        } else {
            changes.iter().filter(|&&f| f < at).max()
        };
        found.map_or("stays, and says there is none".to_string(), |f| {
            f.to_string()
        })
    };
    report.check(
        "layer 2's Sheet column, frames 17 to 25",
        "17, 18, 19, x, (blank), (blank), (blank), 0, 1",
        column("layer-2")["cells"]
            .as_array()
            .map_or(String::new(), |cells| {
                cells[17..26]
                    .iter()
                    .map(|c| match c.as_str() {
                        Some("") => "(blank)".to_string(),
                        Some(m) => m.to_string(),
                        None => "(outside)".to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            }),
    );
    for (what, layer, at, later, expected) in [
        (
            "layer 1 (one drawing the whole shot), from frame 10, next",
            "layer-1",
            10,
            true,
            "stays, and says there is none",
        ),
        (
            "layer 1, from frame 10, previous",
            "layer-1",
            10,
            false,
            "0",
        ),
        (
            "layer 3 (on twos), from frame 10, next",
            "layer-3",
            10,
            true,
            "12",
        ),
        (
            "layer 3, from frame 10, previous",
            "layer-3",
            10,
            false,
            "8",
        ),
        (
            "layer 3, from frame 11 (a held frame), previous",
            "layer-3",
            11,
            false,
            "10",
        ),
        (
            "layer 3, from frame 0, previous",
            "layer-3",
            0,
            false,
            "stays, and says there is none",
        ),
        (
            "layer 4 (on threes), from frame 10, next",
            "layer-4",
            10,
            true,
            "12",
        ),
        (
            "layer 4, from frame 10, previous",
            "layer-4",
            10,
            false,
            "9",
        ),
        (
            "layer 4, from frame 150, next (a one-frame drawing at 152)",
            "layer-4",
            150,
            true,
            "152",
        ),
        ("layer 4, from frame 152, next", "layer-4", 152, true, "153"),
        (
            "layer 4, from frame 163, next (drawing 11 shown again at 165)",
            "layer-4",
            163,
            true,
            "165",
        ),
        (
            "layer 2 (on ones, blank 20 to 23), from frame 19, next",
            "layer-2",
            19,
            true,
            "20",
        ),
        (
            "layer 2, from frame 20, next (past the blank)",
            "layer-2",
            20,
            true,
            "24",
        ),
        (
            "layer 2, from frame 22, previous",
            "layer-2",
            22,
            false,
            "20",
        ),
        (
            "layer 2, from frame 24, previous",
            "layer-2",
            24,
            false,
            "20",
        ),
        (
            "layer 2, from the last frame, next",
            "layer-2",
            239,
            true,
            "stays, and says there is none",
        ),
    ] {
        report.check(
            &format!("{what}: the playhead goes to"),
            expected,
            lands(layer, at, later),
        );
    }

    write_artifact(
        &report,
        "verification/D-733_previous_next_drawing_table.md",
        "D-733: previous and next drawing (Shift+, and Shift+.)",
        &[
            "OpenToonz's, Harmony's and TVPaint's previous and next drawing. Shift+, sends the \
           playhead to the nearest frame before it where the selected layer's drawing \
           changes; Shift+. to the nearest one after it. A change is a new drawing number on \
           the layer's Sheet column, or the start of a blank; a held frame is not a change. \
           With no drawing layer selected it uses the first column, as the sheet strip does.",
            "The expected frames are worked out by hand from the exposures written in \
           `verification/B-08a_project.json` (layer 1 one drawing, layer 2 on ones, layer 3 \
           on twos, layer 4 on threes with a one-frame drawing at 152 and drawing 11 shown \
           again at 165); a blank is cleared into layer 2 for the test. The rule the page \
           applies is the text pinned in the first row; the rows after apply that rule to the \
           Sheet the window answers the page with.",
        ],
        &[],
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// D-734: After Effects' Replace Layer Source (Ctrl+Alt+/). The layer shows the item picked
/// in the Project panel and keeps everything else; one undo puts the old source back.
#[test]
fn replace_layer_source_swaps_only_the_source() {
    let mut report = Report { rows: Vec::new() };
    let page = include_str!("../../ui/index.html");
    report.check(
        "Ctrl+Alt+/ sends the selected layer and the item picked in the Project panel",
        true,
        page.contains("e.ctrlKey && e.altKey && !typing && e.code === 'Slash') { e.preventDefault(); replaceSource(); }")
            && page.contains("command('/layer.replace_source?layer=' + encodeURIComponent(selectedLayer)\n    + '&asset=' + encodeURIComponent(selectedAsset));"),
    );
    let viewer = Mutex::new(demo());
    let layer = |id: &str| {
        let held = held(&viewer);
        held.document
            .project()
            .composition(&held.composition)
            .and_then(|c| c.layer(&Id::new(id)))
            .cloned()
            .unwrap()
    };
    let sound = {
        let held = held(&viewer);
        let mut asset = Asset::still(
            unused_asset_id(held.document.project()),
            "line.wav".to_string(),
            "line.wav".to_string(),
        );
        asset.kind = AssetKind::Audio;
        asset
    };
    let sound_id = sound.id.to_string();
    edit(&viewer, Command::AddAsset { asset: sound });
    run(&viewer, "effect.add?layer=layer-1&type=core.tint");
    let before = layer("layer-1");
    let depth = held(&viewer).document.undo_depth();

    let said = run(
        &viewer,
        "layer.replace_source?layer=layer-1&asset=asset-layer2",
    );
    report.check(
        "replacing layer 1's drawings with layer 2's says",
        "Replace the layer's source with layer2",
        said,
    );
    let after = layer("layer-1");
    report.check(
        "layer 1 now shows",
        "asset-layer2",
        after.asset_id.to_string(),
    );
    report.check(
        "its name, keys, effects (a Tint was added first), transform, timing and Sheet are unchanged",
        true,
        Layer { asset_id: before.asset_id.clone(), ..after.clone() } == before,
    );
    report.check(
        "entries added to undo",
        1,
        held(&viewer).document.undo_depth() - depth,
    );
    run(&viewer, "edit.undo");
    report.check(
        "one undo puts the old source back, and the layer is exactly as it was",
        true,
        layer("layer-1") == before,
    );

    run(&viewer, "layer.add_solid");
    let solid = {
        let held = held(&viewer);
        let comp = held
            .document
            .project()
            .composition(&held.composition)
            .unwrap();
        let id = comp
            .layers_in_order()
            .find(|l| l.kind == LayerKind::Solid)
            .map(|l| l.id.to_string());
        id.unwrap_or_default()
    };
    run(&viewer, "layer.toggle_lock?layer=layer-3");
    for (what, query, expected) in [
        ("no item picked", "layer=layer-1".to_string(), "Pick the picture, sequence or sound in the Project panel first, then press Ctrl+Alt+/.".to_string()),
        ("an item that is not in the project", "layer=layer-1&asset=asset-gone".to_string(), "There is no imported item called asset-gone.".to_string()),
        ("the item it already shows", "layer=layer-1&asset=asset-layer1".to_string(), "layer1 already shows layer1.".to_string()),
        (
            "a sound file on a drawing layer",
            format!("layer=layer-1&asset={sound_id}"),
            "layer1 cannot show line.wav: a drawing layer takes a picture or a sequence, a sound layer a sound file.".to_string(),
        ),
        ("a solid, which shows no imported item", format!("layer={solid}&asset=asset-layer2"), "Solid 1 has no imported source to replace.".to_string()),
    ] {
        report.check(&format!("{what}: the window says"), expected, run(&viewer, &format!("layer.replace_source?{query}")));
    }
    let locked = run(
        &viewer,
        "layer.replace_source?layer=layer-3&asset=asset-layer2",
    );
    report.check(
        "a locked layer is refused",
        true,
        layer("layer-3").asset_id.as_str() == "asset-layer3" && !locked.starts_with("Replace"),
    );

    write_artifact(
        &report,
        "verification/D-734_replace_source_table.md",
        "D-734: Replace layer source (Ctrl+Alt+/)",
        &[
            "After Effects' Replace Layer Source. Select a layer, click a picture, sequence or \
           sound in the Project panel, and press Ctrl+Alt+/ (or Layer > Replace layer source): \
           the layer shows the new item and keeps its name, keys, effects, transform, timing \
           and Sheet. A drawing layer takes a picture or a sequence and a sound layer a sound \
           file; anything else is refused with a sentence, and nothing changes.",
            "The project is `verification/B-08a_project.json` with a Tint added to layer 1 and \
           a sound file added to the Project panel. The third row compares the whole layer \
           before and after, apart from its source.",
        ],
        &[
            "## What to look at\n\n- **A drawing number the new sequence does not have** is \
           not refused: the Sheet keeps its numbers, and a frame showing a number the new \
           sequence lacks is reported as a missing drawing, as it is anywhere else.\n- **A \
           composition layer's source** is not replaced by this; that is left for later.",
        ],
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// D-735: W-37's Keyboard shortcuts window. It lives wholly in the page and the browser's
/// storage, so this pins the page text each promise rests on, and reads the command finder's
/// lines for the defaults the window starts from.
#[test]
fn the_shortcut_window_changes_keys_on_this_computer_only() {
    let mut report = Report { rows: Vec::new() };
    let page = include_str!("../../ui/index.html");
    let has = |text: &str| page.contains(text);
    for (what, holds) in [
        ("the bottom strip's long list of keys is now a Keyboard shortcuts button",
            has("title=\"Every shortcut, to look up, change or print\">Keyboard shortcuts</button>")
                && !has("<span class=\"hint\" id=\"keys\">")),
        ("the Help menu and the command finder open the same window",
            has("<button data-run=\"Keyboard shortcuts\"></button>")
                && has("['Keyboard shortcuts', '', () => shortcutsWindow()],")),
        ("search by words, or by pressing a key", has("'Words'") && has("'A key'")),
        ("a clash names what will lose its keys before anything changes",
            has("will have no shortcut until you give") && has("'Replace'") && has("'Cancel'")),
        ("a clash also offers the line's own default back", has("'Reset to its default, '")),
        ("a key the window keeps fixed (arrows, A S R T M, Alt+Shift keys) is refused, not taken",
            has("That key is not one this window changes; press other keys.")),
        ("Reset all puts every line back", has("'Reset all'")),
        ("changes are kept in this computer's browser storage under 'keymap', not in the project",
            has("const KEYMAP_STORE = 'keymap';")),
        ("the footer says how many are changed and where they are kept",
            has(" changed from the defaults. Kept on this computer, not in the project.")
                && has("All at their defaults. Changes are kept on this computer, not in the project.")),
        ("a changed key runs its line and the window's own handler does not see it",
            has("stopImmediatePropagation();") && has("function shortcutsWindow(")),
        ("typing in a field and the Sketch board keep their keys", has("if (sketchShown && !k.ctrlKey && !k.altKey) return;")),
        ("the menus and the command finder show the keys in use", has("function markRunKeys(") && has("shownKey(")),
        ("Print or save as PDF prints the list through the Sheet's paper",
            has("'keylist'") && has("Print or save as PDF")),
    ] {
        report.check(what, true, holds);
    }
    let capture = page
        .find("// W-37 (D-735): the Keyboard shortcuts window.")
        .unwrap_or(usize::MAX);
    let own = page
        .find("addEventListener('keydown', (e) => {")
        .unwrap_or(0);
    report.check(
        "the changed keys are read before the window's own key handler",
        true,
        capture < own,
    );
    // The finder's lines and their default keys, as the window lists them.
    let lines: Vec<(String, String)> = page
        .split("function paletteLines() {")
        .nth(1)
        .and_then(|rest| rest.split("function runAct(").next())
        .unwrap_or("")
        .lines()
        .filter_map(|line| {
            let rest = line.trim().strip_prefix("['")?;
            let (label, rest) = rest.split_once("', '")?;
            let (keys, _) = rest.split_once("', ")?;
            Some((label.to_string(), keys.to_string()))
        })
        .collect();
    let keyed: Vec<&(String, String)> = lines.iter().filter(|(_, k)| !k.is_empty()).collect();
    report.check(
        "command finder lines the window lists",
        "130 lines, 91 with keys of their own",
        format!(
            "{} lines, {} with keys of their own",
            lines.len(),
            keyed.len()
        ),
    );
    let mut shared: Vec<String> = Vec::new();
    for (_, keys) in &keyed {
        let with: Vec<&str> = keyed
            .iter()
            .filter(|(_, k)| k == keys)
            .map(|(l, _)| l.as_str())
            .collect();
        let row = format!("{keys}: {}", with.join(", "));
        if with.len() > 1 && !shared.contains(&row) {
            shared.push(row);
        }
    }
    report.check(
        "default keys two lines share (the one that acts is chosen by what is selected)",
        "Ctrl+C: Copy keys, Copy layers; Ctrl+V: Paste keys, Paste layers, Paste effects",
        shared.join("; "),
    );
    report.check("Keyboard shortcuts itself has no default key; it is a button, a menu line and a finder line",
        "no key", lines.iter().find(|(l, _)| l == "Keyboard shortcuts").map_or("missing", |(_, k)| if k.is_empty() { "no key" } else { "has a key" }));

    write_artifact(
        &report,
        "verification/D-735_shortcut_editor_table.md",
        "D-735: the Keyboard shortcuts window (W-37)",
        &[
            "W-37 from `design/W-37_shortcuts.html`. The bottom strip's long line of key hints is \
           now a Keyboard shortcuts button; the Help menu and the command finder open the same \
           window. It lists every command finder line in areas (Project, Edit, New layers, \
           Layers, Time, Keys, Properties, Timeline, Viewer, Tools, Finding, Sheet) with its \
           keys, searches by words or by a pressed key, changes any line's keys (one that had \
           none can be given some), names a clash before anything changes, and resets one line \
           or all of them. Changes are kept on this computer, never in the project. Print or \
           save as PDF prints the list.",
            "The window is page code only, so the rows pin the page text each promise rests on; \
           the last rows read the command finder's lines for the defaults. No default key \
           changed. Pictures of the window, taken from the page in a browser with no project \
           open, and the printed list as a PDF, are in `verification/D-735 pictures/`.",
        ],
        &[
            "## What was left out\n\n- **Keys that are not command finder lines** (the arrows, \
           A S R T M, the Alt+Shift keys that set a key, the Sheet's and the Sketch board's \
           keys) are named in a clash but cannot be changed.\n- **Ctrl+C and Ctrl+V** are \
           shared by several lines and act on what is selected; giving one of those lines new \
           keys does not take Ctrl+C or Ctrl+V away from the others.\n- **A changed key does \
           not act while you type in a field or in the Sheet**, unless it uses Ctrl.\n- **The \
           areas** are chosen from each line's name; a new command finder line with an unusual \
           name lands in Layers.",
        ],
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// The prose around the layer table.
///
/// Out here rather than inline so that the test above reads as the sequence of steps it is,
/// which is the thing worth checking against document 24.
const LAYER_INTRO: &[&str] = &[
    "This is the window's half of the layer commands in document 24. The core's half - \
         that adding, deleting, renaming and reordering a layer are correct and undoable - is \
         checked in `verification/B-05_model_table.md` and is not repeated here. What is \
         checked here is the part between a row in a list and that core, which is where a \
         window goes wrong: a button that names a layer no longer there, a new layer given an \
         identifier something else is already using, a toggle that decides what a layer \
         currently is from a list drawn a minute ago, a refusal that leaves a control \
         springing back with nothing said.",
    "The fixture is `Fixtures/projects/unknown_effect_project.json`, which names an \
         effect no version of this build has. It is used for every row on purpose: a panel is \
         a second reader of the project, and a second reader is a second chance to lose what \
         it cannot model. The last row is the one that would catch that - after every edit \
         above and every undo of them, what a save would write is compared against the file \
         that was opened, byte for byte.",
];

const LAYER_NOTES: &[&str] = &[
    "## What to look at\n\n- **A refusal is a sentence, never silence.** Six rows here \
         are commands that were turned down, and each one is checked for the words the person \
         would read - which drawing, which layer, and that the front layer is already at the \
         front.\n- **A refused command changes nothing.** Document 26 requires it, and the \
         rows after each refusal check the history is no deeper and the layers are in the same \
         order.\n- **Undo names what it takes back.** \"Undone\" on its own would be true and \
         useless.\n- **Unsaved work is a comparison, not a flag.** Undoing every edit back to \
         the file makes the window say the project is saved again, because the core compares \
         the document against the last save rather than counting edits.",
    "## What this does not cover\n\nThe page. Every row here calls the same function \
         the window's URL scheme calls, with the same text the page would put in it, so what \
         is checked is everything from the request inwards. That a button is wired to the \
         right request, that the list is drawn front-first, and that the keyboard reaches all \
         of it are in the photographs beside this table, not in it.\n\nThe transform and \
         the effect stack are edited from the same inspector and are checked in \
         `verification/B-12a_transform_table.md` and `verification/B-12a_effects_table.md`. \
         The matte and the mask are shown and cannot yet be changed; that is later work and \
         is named as missing rather than quietly absent.",
];

/// Where a property sits in the answer the panels are given. Five are in the transform;
/// the depth is on the layer itself, because D-58 has it ride the parent chain additively
/// and a transform does not.
///
/// B-13d: both readers below reached through `l["transform"][prop]` until a depth could be
/// keyed. A reader shaped that way does not fail on a depth - it finds nothing and reports
/// nothing - so a table built on it would have passed while checking nothing, which is the
/// one kind of wrong a table cannot catch by going green.
fn slot<'a>(layer: &'a serde_json::Value, prop: &str) -> &'a serde_json::Value {
    match prop {
        "depth" => &layer["depth"],
        _ => &layer["transform"][prop],
    }
}

/// The property a layer carries, as a string, from the same JSON the panels get.
fn base(viewer: &Mutex<Viewer>, layer_id: &str, prop: &str) -> String {
    let answer: serde_json::Value =
        serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
    answer["project"]["compositions"][0]["layers"]
        .as_array()
        .expect("a composition has layers")
        .iter()
        .find(|l| l["id"] == layer_id)
        .map(|l| slot(l, prop)["base"].to_string())
        .unwrap_or_else(|| "(no such layer)".to_string())
}

/// A property's keyframes as the panels get them, one `value@frame interp` per key.
fn keys(viewer: &Mutex<Viewer>, layer_id: &str, prop: &str) -> String {
    let answer: serde_json::Value =
        serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
    answer["project"]["compositions"][0]["layers"]
        .as_array()
        .expect("a composition has layers")
        .iter()
        .find(|l| l["id"] == layer_id)
        // B-13d: a property with no keys on it is not a missing layer, and this helper
        // answered both with the same sentence until a depth could be keyed. The five in
        // the transform always carry a `keyframes` array, empty or not, so the two cases
        // never came apart; a depth is absent until somebody touches it, and a layer that
        // has never had one was reported as a layer that does not exist.
        .map(|l| match slot(l, prop)["keyframes"].as_array() {
            Some(keys) => keys.clone(),
            None => Vec::new(),
        })
        .map(|keys| as_keys(&keys))
        .unwrap_or_else(|| "(no such layer)".to_string())
}

/// One property's keyframes, in the rows every table in this file writes them as.
///
/// Shared rather than copied, because B-13e gave the camera keys too: a camera's row has to
/// read exactly as a layer's or the two tables cannot be held up against each other, and the
/// moment there were two of these one of them would have started drifting.
fn as_keys(list: &[serde_json::Value]) -> String {
    list.iter()
        .map(|k| {
            // D-52: an eased key's four numbers go in the row. The word "ease" on its own
            // would pass whatever curve it was given, which is the one thing a table about
            // easing has to be able to tell apart.
            let curve = match &k["ease"] {
                serde_json::Value::Null => String::new(),
                ease => format!(" {ease}"),
            };
            // D-53: and a position key's handles, for the same reason.
            let path = match &k["spatial"] {
                serde_json::Value::Null => String::new(),
                spatial => format!(" path {spatial}"),
            };
            format!(
                "{}@{} {}{curve}{path}",
                k["value"],
                k["frame"],
                k["interp"].as_str().unwrap_or("?")
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// A property's value on one frame, out of the `/boxes` answer the inspector shows it from.
/// What the graph editor is told to draw: the first component of one property, frame by
/// frame, out of the route the page asks rather than out of the model beside it.
fn plotted(viewer: &Mutex<Viewer>, query: &str) -> Vec<f64> {
    let answer = curve(viewer, Some(query));
    let said: serde_json::Value =
        serde_json::from_slice(answer.body()).expect("the route answers with JSON");
    said["samples"]
        .as_array()
        .expect("samples is a list")
        .iter()
        .map(|s| s[0].as_f64().expect("a sample is a number"))
        .collect()
}

fn value_at(viewer: &Mutex<Viewer>, frame: i32, layer_id: &str, prop: &str) -> String {
    let body = boxes(viewer, frame, None).into_body();
    let answer: serde_json::Value =
        serde_json::from_slice(&body).expect("the boxes answer is JSON");
    answer["values"][layer_id][prop].to_string()
}

/// D-314: Time-Reverse Keyframes and Freeze Frame, as the window answers them.
#[test]
fn d314_chosen_keys_reverse_and_a_drawing_freezes() {
    let source = repo("Fixtures/projects/unknown_effect_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let l = "layer-cel";
    for (frame, x) in [(0, 0), (4, 100), (10, 400)] {
        run(
            &viewer,
            &format!("keyframe.add_remove?layer={l}&prop=position&frame={frame}"),
        );
        run(
            &viewer,
            &format!("property.set_base?layer={l}&prop=position&value={x},0&frame={frame}"),
        );
    }
    run(
        &viewer,
        &format!(
            "keyframe.set_interp?layer={l}&prop=position&frame=4&mode=ease&curve=0.25,0.5,0.75,1"
        ),
    );
    let before = keys(&viewer, l, "position");
    let depth = held(&viewer).document.undo_depth();
    let all = format!("key={l}|position|0&key={l}|position|4&key={l}|position|10");
    run(&viewer, &format!("keyframe.reverse?{all}"));
    // The key from 10 lands on 0 and plays the stretch 4 to 10 backwards, so it takes that
    // stretch's ease turned round; the one from 4 lands on 6, the one from 0 on 10.
    assert_eq!(
        keys(&viewer, l, "position"),
        "[400,0]@0 ease [0.25,0,0.75,0.5], [100,0]@6 linear, [0,0]@10 linear",
    );
    assert_eq!(
        held(&viewer).document.undo_depth() - depth,
        1,
        "one entry to undo"
    );
    run(
        &viewer,
        &format!("keyframe.reverse?key={l}|position|0&key={l}|position|6&key={l}|position|10"),
    );
    assert_eq!(
        keys(&viewer, l, "position"),
        before,
        "reversed twice is as it was"
    );
    undo(&viewer);
    undo(&viewer);
    assert_eq!(
        keys(&viewer, l, "position"),
        before,
        "and undo takes both back"
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={l}&prop=position&frame=6"),
    );
    assert_eq!(
        run(&viewer, &format!("keyframe.reverse?{all}")),
        "A key that is not chosen is on frame 6, where a reversed key would land. \
         Choose it too, or move it first.",
        "a key in the way is refused",
    );
    assert_eq!(
        run(&viewer, &format!("keyframe.reverse?key={l}|position|4")),
        "Choose two or more keys on different frames to reverse.",
    );

    // Frame 3 shows drawing 2; frozen, the layer holds it from its first frame to its last.
    let spans = |viewer: &Mutex<Viewer>| {
        let answer: serde_json::Value =
            serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
        answer["project"]["compositions"][0]["layers"][0]["exposure_spans"].clone()
    };
    let depth = held(&viewer).document.undo_depth();
    run(&viewer, &format!("layer.freeze_frame?layer={l}&frame=3"));
    assert_eq!(
        spans(&viewer),
        serde_json::json!([{"start_frame": 0, "end_frame_exclusive": 5, "drawing_number": 2}]),
    );
    assert_eq!(
        held(&viewer).document.undo_depth() - depth,
        1,
        "one entry to undo"
    );
    undo(&viewer);
    assert_eq!(
        spans(&viewer),
        serde_json::json!([
            {"start_frame": 0, "end_frame_exclusive": 2, "drawing_number": 1},
            {"start_frame": 2, "end_frame_exclusive": 5, "drawing_number": 2}
        ]),
    );
}

#[test]
fn the_inspector_changes_a_transform_and_a_drag_is_one_history_entry() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/unknown_effect_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let l = "layer-cel";

    // ---- typing a number ------------------------------------------------------------------
    report.check(
        "the fixture's layer starts where document 19's default puts it",
        "[0,0]",
        base(&viewer, l, "position"),
    );
    report.check(
        "typing a position says what it set, in the words undo will use",
        "Set position to (120, -40)",
        run(
            &viewer,
            "property.set_base?layer=layer-cel&prop=position&value=120,-40",
        ),
    );
    report.check(
        "and the panels are given the new value back",
        "[120,-40]",
        base(&viewer, l, "position"),
    );
    report.check("a scalar property takes one number", "45", {
        run(
            &viewer,
            "property.set_base?layer=layer-cel&prop=rotation&value=45",
        );
        base(&viewer, l, "rotation")
    });
    // Document 19 makes opacity nought to one and the core clamps rather than refuses, so a
    // person who typed 5 gets the fully opaque layer they were reaching for.
    report.check("opacity above one is clamped rather than refused", "1", {
        run(
            &viewer,
            "property.set_base?layer=layer-cel&prop=opacity&value=5",
        );
        base(&viewer, l, "opacity")
    });

    // ---- what the panel refuses before the core sees it -------------------------------------
    let depth = held(&viewer).document.undo_depth();
    report.check(
        "a property name that is not one of the six is refused, and the six are named",
        "Which property? Say anchor, position, scale, rotation, opacity or depth.",
        run(
            &viewer,
            "property.set_base?layer=layer-cel&prop=wobble&value=1",
        ),
    );
    report.check(
        "no value at all is refused, and asks",
        "What should scale be set to?",
        run(&viewer, "property.set_base?layer=layer-cel&prop=scale"),
    );
    report.check(
        "text that is not numbers is refused in the shape the property wants",
        "position needs two numbers, like 12, -4. Not \"over there\".",
        run(
            &viewer,
            "property.set_base?layer=layer-cel&prop=position&value=over%20there",
        ),
    );
    report.check(
        "one number for a two-number property is refused the same way",
        "position needs two numbers, like 12, -4. Not \"7\".",
        run(
            &viewer,
            "property.set_base?layer=layer-cel&prop=position&value=7",
        ),
    );
    report.check(
        "two numbers for a one-number property never reach the core, and are named here",
        "rotation needs a number. Not \"1,2\".",
        run(
            &viewer,
            "property.set_base?layer=layer-cel&prop=rotation&value=1,2",
        ),
    );
    report.check(
        // The percent typed in is divided into the factor the model holds before the core
        // sees it, which is why the second number comes back as 0.01 rather than the 1 that
        // was typed. The refusal names the model's value, not the field's; that the two
        // spellings differ is D-22's, and `verification/B-12b_w01_walkthrough.md` writes up
        // what it costs a person reading the status line.
        "a value that is not a finite number is refused",
        "scale cannot be set to (NaN, 0.01).",
        run(
            &viewer,
            "property.set_base?layer=layer-cel&prop=scale&value=NaN,1",
        ),
    );
    report.check(
        "none of those six refusals put anything in the history",
        depth,
        held(&viewer).document.undo_depth(),
    );
    report.check(
        "and the position is the one that was typed",
        "[120,-40]",
        base(&viewer, l, "position"),
    );

    // ---- a drag ------------------------------------------------------------------------------
    // Document 26: a drag previews without creating hundreds of history entries and commits
    // one command at release. This is the check that the window opens the transaction at all -
    // a page that forgot to would look identical until somebody pressed Ctrl+Z.
    let depth = held(&viewer).document.undo_depth();
    for x in [130, 150, 180, 210] {
        run(
            &viewer,
            &format!("property.drag_update?layer=layer-cel&prop=position&value={x},-40"),
        );
    }
    report.check(
        "the value follows the drag while it is being dragged",
        "[210,-40]",
        base(&viewer, l, "position"),
    );
    report.check(
        "and four intermediate values are not four history entries",
        depth,
        held(&viewer).document.undo_depth(),
    );
    report.check(
        "releasing commits one entry, named for what it set",
        "Set position to (210, -40)",
        run(&viewer, "property.drag_end"),
    );
    report.check(
        "which is one entry, not four",
        depth + 1,
        held(&viewer).document.undo_depth(),
    );
    report.check(
        "and undoing it goes back to before the drag, not to one step inside it",
        "[120,-40]",
        {
            undo(&viewer);
            base(&viewer, l, "position")
        },
    );
    redo(&viewer);

    // ---- Escape during a drag ------------------------------------------------------------------
    let depth = held(&viewer).document.undo_depth();
    for x in [220, 260, 300] {
        run(
            &viewer,
            &format!("property.drag_update?layer=layer-cel&prop=position&value={x},-40"),
        );
    }
    report.check(
        "Escape during a drag says the value has gone back",
        "Cancelled; the value is back where the drag started.",
        run(&viewer, "property.drag_cancel"),
    );
    report.check(
        "and it has - document 24: Escape restores the pre-drag value",
        "[210,-40]",
        base(&viewer, l, "position"),
    );
    report.check(
        "a cancelled drag leaves no history entry",
        depth,
        held(&viewer).document.undo_depth(),
    );

    // ---- the two ends on their own ---------------------------------------------------------------
    report.check(
        "releasing when nothing is being dragged is said, not silently ignored",
        "Nothing moved.",
        run(&viewer, "property.drag_end"),
    );
    report.check(
        "and so is Escape when nothing is being dragged",
        "Nothing is being dragged.",
        run(&viewer, "property.drag_cancel"),
    );
    // A drag that ends where it started is somebody who thought better of it. Document 26
    // says that is not an edit, and an undo item that undoes nothing is worse than none.
    let depth = held(&viewer).document.undo_depth();
    run(
        &viewer,
        "property.drag_update?layer=layer-cel&prop=position&value=260,-40",
    );
    run(
        &viewer,
        "property.drag_update?layer=layer-cel&prop=position&value=210,-40",
    );
    report.check(
        "a drag that ends where it started is not an edit",
        "Nothing moved.",
        run(&viewer, "property.drag_end"),
    );
    report.check(
        "and adds no history entry",
        depth,
        held(&viewer).document.undo_depth(),
    );

    // ---- a locked layer ---------------------------------------------------------------------------
    run(&viewer, "layer.toggle_lock?layer=layer-cel");
    report.check(
        "a locked layer refuses a transform edit, and says which rule stopped it",
        "The layer \"Cel\" is locked, so it was not changed. Unlock the layer to edit it.",
        run(
            &viewer,
            "property.set_base?layer=layer-cel&prop=position&value=0,0",
        ),
    );
    report.check(
        "and the value is untouched",
        "[210,-40]",
        base(&viewer, l, "position"),
    );
    run(&viewer, "layer.toggle_lock?layer=layer-cel");

    // ---- keyframes (W-10) ---------------------------------------------------------------------------
    // Document 24's toggle, and what a typed or dragged value becomes once a property has
    // keys. The value between two keys is read through `/boxes`, which is the answer the
    // inspector shows it from, so the row is the number the artist would see.
    let depth = held(&viewer).document.undo_depth();
    report.check(
        "the diamond on a property with no keys sets one holding the value it has",
        "Keyframe position at frame 12 to (210, -40)",
        run(
            &viewer,
            "keyframe.add_remove?layer=layer-cel&prop=position&frame=12",
        ),
    );
    report.check(
        "and the file now holds that one key, linear as document 19 defaults",
        "[210,-40]@12 linear",
        keys(&viewer, l, "position"),
    );
    report.check(
        "a value typed at another frame becomes a second key rather than a base",
        "Keyframe position at frame 36 to (0, 0)",
        run(
            &viewer,
            "property.set_base?layer=layer-cel&prop=position&value=0,0&frame=36",
        ),
    );
    report.check(
        "so there are two keys",
        "[210,-40]@12 linear, [0,0]@36 linear",
        keys(&viewer, l, "position"),
    );
    report.check(
        "and the base is the value from before either",
        "[210,-40]",
        base(&viewer, l, "position"),
    );
    report.check(
        "halfway between the keys the window answers the halfway value",
        "[105,-20]",
        value_at(&viewer, 24, l, "position"),
    );
    report.check(
        "a value on a keyframed property with no frame named is refused",
        "position is keyframed, so a value belongs to a frame. Say frame=<frame>.",
        run(
            &viewer,
            "property.set_base?layer=layer-cel&prop=position&value=0,0",
        ),
    );
    report.check(
        "the diamond with no frame named is refused",
        "Which frame? Say frame=<frame>.",
        run(&viewer, "keyframe.add_remove?layer=layer-cel&prop=position"),
    );
    report.check(
        "the diamond on a frame that has a key removes it",
        "Remove position keyframe at frame 36",
        run(
            &viewer,
            "keyframe.add_remove?layer=layer-cel&prop=position&frame=36",
        ),
    );
    report.check(
        "leaving the first",
        "[210,-40]@12 linear",
        keys(&viewer, l, "position"),
    );
    // A corner drag on the picture sends a position and a scale for one layer at once.
    // Keyed, both must survive the drag: the transaction keeps the last of each control,
    // and a key on another property is another control.
    run(
        &viewer,
        "keyframe.add_remove?layer=layer-cel&prop=scale&frame=12",
    );
    for (x, s) in [(240, 120), (270, 160), (300, 200)] {
        run(
            &viewer,
            &format!("property.drag_update?layer=layer-cel&prop=position&value={x},-40&frame=12"),
        );
        run(
            &viewer,
            &format!("property.drag_update?layer=layer-cel&prop=scale&value={s},{s}&frame=12"),
        );
    }
    report.check(
        "a drag on two keyed properties commits one entry",
        "Keyframe position at frame 12 to (300, -40) and 1 more",
        run(&viewer, "property.drag_end"),
    );
    report.check(
        "the position key took the last value dragged",
        "[300,-40]@12 linear",
        keys(&viewer, l, "position"),
    );
    report.check(
        "and the scale key beside it survived the position's drag",
        "[200,200]@12 linear",
        keys(&viewer, l, "scale"),
    );
    report.check(
        "five history entries for the five edits: two keys set, one typed, one removed, one drag",
        5,
        held(&viewer).document.undo_depth() - depth,
    );

    // ---- moving a key (W-11) ------------------------------------------------------------
    // The mark on a property's row, taken hold of and put down on another frame. One command
    // rather than a remove and a set, so it is one entry to undo and the key cannot be lost
    // between the two halves of it; the value and the interpolation mode go with it.
    let depth = held(&viewer).document.undo_depth();
    report.check(
        "a key put down on another frame moves, and says which frame it came from",
        "Move position keyframe from frame 12 to frame 20",
        run(
            &viewer,
            "keyframe.move?layer=layer-cel&prop=position&from=12&to=20",
        ),
    );
    report.check(
        "the key is on the new frame with the value and the interpolation it had",
        "[300,-40]@20 linear",
        keys(&viewer, l, "position"),
    );
    report.check(
        "and the key on another property stayed where it was",
        "[200,200]@12 linear",
        keys(&viewer, l, "scale"),
    );
    run(
        &viewer,
        "keyframe.add_remove?layer=layer-cel&prop=position&frame=30",
    );
    report.check(
        "a key put down where that property already has one is refused, not allowed to \
         overwrite it",
        "There is already a position keyframe on frame 30.",
        run(
            &viewer,
            "keyframe.move?layer=layer-cel&prop=position&from=20&to=30",
        ),
    );
    report.check(
        "and both keys are still there",
        "[300,-40]@20 linear, [300,-40]@30 linear",
        keys(&viewer, l, "position"),
    );
    report.check(
        "a key that is not on the frame named cannot be moved",
        "There is no position keyframe at frame 99 to move. The edit was not applied. \
         Nothing in the project changed.",
        run(
            &viewer,
            "keyframe.move?layer=layer-cel&prop=position&from=99&to=40",
        ),
    );
    report.check(
        "a move onto the frame the key is already on is not an edit",
        "That keyframe is already on that frame.",
        run(
            &viewer,
            "keyframe.move?layer=layer-cel&prop=position&from=20&to=20",
        ),
    );
    report.check(
        "two history entries for the two edits that were allowed, and none for the three \
         that were refused",
        2,
        held(&viewer).document.undo_depth() - depth,
    );
    undo(&viewer);
    undo(&viewer);
    report.check(
        "undoing puts the key back on the frame it was moved from",
        "[300,-40]@12 linear",
        keys(&viewer, l, "position"),
    );
    // W-17: several chosen keys dragged together. Two neighbours a frame pair apart moved
    // right by that pair: the front one has to leave before the back one lands.
    run(
        &viewer,
        "keyframe.add_remove?layer=layer-cel&prop=position&frame=14",
    );
    let depth = held(&viewer).document.undo_depth();
    report.check(
        "a key moved onto a key that is not moving with it is refused, and neither moves",
        "There is already a position keyframe on frame 14.",
        run(&viewer, "keyframe.move?by=2&key=layer-cel|position|12"),
    );
    report.check(
        "two chosen neighbours moved together, the one ahead first, as one entry",
        "Move position keyframe from frame 14 to frame 16 and 1 more",
        run(
            &viewer,
            "keyframe.move?by=2&key=layer-cel|position|12&key=layer-cel|position|14",
        ),
    );
    report.check(
        "both keys are two frames on",
        "[300,-40]@14 linear, [300,-40]@16 linear",
        keys(&viewer, l, "position"),
    );
    report.check(
        "one history entry for the move of two",
        1,
        held(&viewer).document.undo_depth() - depth,
    );
    undo(&viewer);
    undo(&viewer);
    report.check(
        "undoing the move and the key added for it leaves the one key where it was",
        "[300,-40]@12 linear",
        keys(&viewer, l, "position"),
    );
    // W-19: Delete with keys chosen removes those keys, all of them as one entry to undo.
    run(
        &viewer,
        "keyframe.add_remove?layer=layer-cel&prop=position&frame=20",
    );
    let depth = held(&viewer).document.undo_depth();
    report.check(
        "two chosen keys deleted together say so",
        "Remove position keyframe at frame 12 and 1 more",
        run(
            &viewer,
            "keyframe.add_remove?key=layer-cel|position|12&key=layer-cel|position|20",
        ),
    );
    report.check("neither key is left", "", keys(&viewer, l, "position"));
    report.check(
        "one history entry for the two",
        1,
        held(&viewer).document.undo_depth() - depth,
    );
    undo(&viewer);
    report.check(
        "one undo puts both back",
        "[300,-40]@12 linear, [300,-40]@20 linear",
        keys(&viewer, l, "position"),
    );
    undo(&viewer);

    // ---- easing a segment (D-52) --------------------------------------------------------
    // The preset beside the diamond. Document 20 says a keyframe's mode belongs to the
    // segment that starts at it, so what these rows check is the value *between* two keys
    // moving while the keys themselves do not - which is what an ease is and the only way to
    // see one without drawing a graph.
    let depth = held(&viewer).document.undo_depth();
    run(
        &viewer,
        "keyframe.add_remove?layer=layer-cel&prop=rotation&frame=0",
    );
    run(
        &viewer,
        "property.set_base?layer=layer-cel&prop=rotation&value=120&frame=24",
    );
    report.check(
        "two linear rotation keys, and the halfway frame is halfway between them",
        "82.5",
        value_at(&viewer, 12, l, "rotation"),
    );
    report.check(
        "pressing the curve says which segment it changed",
        "Keyframe rotation at frame 0 to 45",
        run(
            &viewer,
            "keyframe.set_interp?layer=layer-cel&prop=rotation&frame=0&mode=ease",
        ),
    );
    report.check(
        "the key carries the four numbers of the curve, not just the word",
        "45@0 ease [0.3333333333333333,0,0.6666666666666666,1], 120@24 linear",
        keys(&viewer, l, "rotation"),
    );
    report.check(
        "the halfway frame is still halfway, because easy ease is symmetric",
        "82.5",
        value_at(&viewer, 12, l, "rotation"),
    );
    report.check(
        "a quarter of the way along it is behind where linear would have it: 63.75 becomes",
        "56.71875",
        value_at(&viewer, 6, l, "rotation"),
    );
    report.check(
        "and three quarters along it is ahead: 101.25 becomes",
        "108.28125",
        value_at(&viewer, 18, l, "rotation"),
    );
    report.check(
        "neither keyframe moved",
        "45",
        value_at(&viewer, 0, l, "rotation"),
    );
    report.check(
        "nor the one at the end",
        "120",
        value_at(&viewer, 24, l, "rotation"),
    );
    report.check(
        "pressing it again puts the segment back to linear",
        "Keyframe rotation at frame 0 to 45",
        run(
            &viewer,
            "keyframe.set_interp?layer=layer-cel&prop=rotation&frame=0&mode=linear",
        ),
    );
    report.check(
        "and the four numbers are gone from the file rather than left behind",
        "45@0 linear, 120@24 linear",
        keys(&viewer, l, "rotation"),
    );
    report.check(
        "easing a frame that has no key is refused, and says what to do first",
        "rotation has no keyframe at frame 7, so there is no segment to ease. Add a \
         keyframe first.",
        run(
            &viewer,
            "keyframe.set_interp?layer=layer-cel&prop=rotation&frame=7&mode=ease",
        ),
    );
    report.check(
        "a curve this window does not offer is refused rather than guessed at",
        "An interpolation is hold, linear or ease. Not \"bouncy\".",
        run(
            &viewer,
            "keyframe.set_interp?layer=layer-cel&prop=rotation&frame=0&mode=bouncy",
        ),
    );
    report.check(
        "four history entries for the four edits, and none for the two refusals",
        4,
        held(&viewer).document.undo_depth() - depth,
    );
    undo(&viewer);
    report.check(
        "undoing the straightening gives the curve back",
        "45@0 ease [0.3333333333333333,0,0.6666666666666666,1], 120@24 linear",
        keys(&viewer, l, "rotation"),
    );

    // ---- the graph editor (D-52) --------------------------------------------------------
    // The panel that shares the timeline's place draws a line and two handles, and the line
    // is the one thing about it that could quietly be wrong: a graph that solved the curve
    // itself would go on looking like a perfectly good graph on the day it stopped agreeing
    // with the picture. So the page asks the window, and these rows are what it is told.
    let drawn = plotted(&viewer, "layer=layer-cel&prop=rotation&from=0&to=24");
    report.check(
        "the graph is given one sample per frame of the range",
        25,
        drawn.len(),
    );
    report.check(
        "and every one of them is the value the renderer would draw on that frame",
        "all 25 agree",
        match (0..=24).all(|f| {
            let shown: f64 = value_at(&viewer, f, l, "rotation")
                .parse()
                .expect("a rotation is a number");
            (shown - drawn[f as usize]).abs() < 1e-12
        }) {
            true => "all 25 agree",
            false => "at least one disagrees",
        },
    );
    report.check(
        "so the eased middle of the segment is drawn where the ease puts it",
        "82.5",
        drawn[12],
    );
    report.check(
        "and a quarter along, behind where a straight line would have it",
        "56.71875",
        drawn[6],
    );
    report.check(
        "pulling the first handle to the far end of the segment is one edit",
        "Keyframe rotation at frame 0 to 45",
        run(
            &viewer,
            "keyframe.set_interp?layer=layer-cel&prop=rotation&frame=0&mode=ease\
             &curve=0.75,0,1,0.25",
        ),
    );
    report.check(
        "and the four numbers it was dropped at are the ones in the file",
        "45@0 ease [0.75,0,1,0.25], 120@24 linear",
        keys(&viewer, l, "rotation"),
    );
    report.check(
        "and the middle of the segment is held right back: 82.5 becomes",
        "49.3993",
        format!(
            "{:.4}",
            plotted(&viewer, "layer=layer-cel&prop=rotation&from=0&to=24")[12]
        ),
    );
    report.check(
        "a handle dragged outside its own segment is refused rather than clamped",
        "A curve handle cannot reach outside its own segment, so x1 and x2 are between 0 \
         and 1. Not 1.2 and 0.1.",
        run(
            &viewer,
            "keyframe.set_interp?layer=layer-cel&prop=rotation&frame=0&mode=ease\
             &curve=1.2,0,0.1,1",
        ),
    );
    report.check(
        "and so is a curve that is not four numbers",
        "A curve is four numbers, x1,y1,x2,y2. Not \"0.4,0.1\".",
        run(
            &viewer,
            "keyframe.set_interp?layer=layer-cel&prop=rotation&frame=0&mode=ease\
             &curve=0.4,0.1",
        ),
    );
    report.check(
        "asking about a layer that has gone draws nothing rather than failing",
        0,
        plotted(&viewer, "layer=gone&prop=rotation&from=0&to=24").len(),
    );
    undo(&viewer);

    // ---- the motion path's handles (D-53) -----------------------------------------------
    // The canvas pulls a handle and sends the key's four numbers at release. What these rows
    // check is that the four numbers land on the key untouched, that the key's value and
    // mode do not move, and that the layer between the keys is where document 20's cubic
    // puts it: P0 (0,0), P1 (0,120) from the handle, P2 (160,0) from the third the second
    // key has with no handles of its own, P3 (240,0). Halfway, (P0 + 3 P1 + 3 P2 + P3) / 8
    // is (90, 45), worked by hand.
    while held(&viewer).document.undo_depth() > 0 {
        undo(&viewer);
    }
    run(
        &viewer,
        "keyframe.add_remove?layer=layer-cel&prop=position&frame=0",
    );
    run(
        &viewer,
        "property.set_base?layer=layer-cel&prop=position&value=240,0&frame=24",
    );
    report.check(
        "two position keys with no handles, and halfway is halfway along the straight line",
        "[120,0]",
        value_at(&viewer, 12, l, "position"),
    );
    report.check(
        "pulling the first key's outgoing handle says which key it changed",
        "Keyframe position at frame 0 to (0, 0)",
        run(
            &viewer,
            "keyframe.set_path?layer=layer-cel&frame=0&handles=0,0,0,120",
        ),
    );
    report.check(
        "the key carries the four numbers it was dropped at, and keeps its value and mode",
        "[0,0]@0 linear path [0,0,0,120], [240,0]@24 linear",
        keys(&viewer, l, "position"),
    );
    report.check(
        "and halfway the layer has left the straight line for the curve: (120, 0) becomes",
        "[90,45]",
        value_at(&viewer, 12, l, "position"),
    );
    report.check(
        "neither key moved",
        "[0,0] and [240,0]",
        format!(
            "{} and {}",
            value_at(&viewer, 0, l, "position"),
            value_at(&viewer, 24, l, "position")
        ),
    );
    report.check(
        "a handle on a frame with no key is refused, and says what to do first",
        "position has no keyframe at frame 7, so there is no handle to move. Add a keyframe \
         first.",
        run(
            &viewer,
            "keyframe.set_path?layer=layer-cel&frame=7&handles=0,0,0,120",
        ),
    );
    report.check(
        "and so is a path that is not four numbers",
        "A path is four numbers, in_x,in_y,out_x,out_y. Not \"0,120\".",
        run(
            &viewer,
            "keyframe.set_path?layer=layer-cel&frame=0&handles=0,120",
        ),
    );
    report.check(
        "three history entries for the two keys and the pull, and none for the refusals",
        3,
        held(&viewer).document.undo_depth(),
    );
    undo(&viewer);
    report.check(
        "undoing the pull puts the straight line back",
        "[0,0]@0 linear, [240,0]@24 linear and [120,0]",
        format!(
            "{} and {}",
            keys(&viewer, l, "position"),
            value_at(&viewer, 12, l, "position")
        ),
    );

    // ---- back to the file ---------------------------------------------------------------------------
    while held(&viewer).document.undo_depth() > 0 {
        undo(&viewer);
    }
    let held = held(&viewer);
    let opened = std::fs::read_to_string(&source)
        .expect("read the fixture")
        .replace("\r\n", "\n");
    let now = persist::to_json(held.document.project(), &held.preserved);
    let same = "identical, including the effect this build cannot model";
    report.check(
        "undoing every transform edit gives back the file that was opened",
        same,
        if opened == now {
            same
        } else {
            "what a save would write is no longer what was opened"
        },
    );
    drop(held);

    write_artifact(
        &report,
        "verification/B-12a_transform_table.md",
        "B-12a: what the transform inspector does",
        TRANSFORM_INTRO,
        TRANSFORM_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

const TRANSFORM_INTRO: &[&str] = &[
    "Document 19's five transform properties, edited the two ways an inspector offers: \
     typing a number into a field, and dragging the handle beside it. Both send the same \
     command ID. What differs is that typing commits an entry of its own and a drag commits \
     one entry for the whole gesture, which is document 26's interaction transaction.",
    "The core's half - that setting a property is correct, undoable, and clamped or refused \
     by document 19's rules - is checked in `verification/B-05_model_table.md`. What is \
     checked here is the part the window owns: that the text a field sends becomes the right \
     kind of value, that a drag opens a transaction at all, and that every refusal is a \
     sentence naming the rule.",
];

const TRANSFORM_NOTES: &[&str] = &[
    "## What to look at\n\n- **Four drag steps are one history entry.** The rows check the \
     value follows the drag, that the history does not deepen while it is following, and \
     that one entry appears at release. A window that forgot to open the transaction would \
     look identical on screen and would put four entries in the history, and nobody would \
     find out until they pressed Ctrl+Z.\n- **A drag that ends where it started is not an \
     edit.** It is somebody who thought better of it, and an undo item that undoes nothing \
     is worse than no undo item.\n- **Escape puts the value back.** Document 24 requires it \
     and the row after it checks the value, not just the sentence.\n- **A value is read in \
     the shape the property wants.** Sending one number for position is refused rather than \
     read as a scalar the core would then reject for its kind, which means a wrong-kind \
     value cannot reach the model at all. The row that would have shown the core's own \
     refusal instead shows the window's, and that is the correct outcome.",
    "## What this does not cover\n\nThe gesture. Every row here sends the requests a scrub \
     sends, in the order it sends them, but that a pointer dragged across the handle produces \
     those requests - and that the field stops being rebuilt underneath it while it does - is \
     in the photographs beside this table.\n\nBlend mode is shown in the inspector and cannot \
     be changed from it. There is no command in the core for changing one, W-01 does not ask \
     to change one, and adding a command to the model to fill a gap in a panel is a decision \
     about the project format rather than about this window.\n\nKeyframes, since W-10. The \
     diamond beside a property is document 24's `keyframe.add_remove`, and a value typed or \
     dragged on a keyframed property becomes a key at the frame under the playhead rather \
     than a base nothing is drawn from. The rows under \"keyframes\" are that, and the \
     interpolated value between two keys is read back through the same `/boxes` answer the \
     inspector shows it from.\n\nMoving a key along the bar is `keyframe.move`, built by \
     W-11 on 2026-09-12: one command rather than a remove and a set, so a key put on the \
     wrong frame costs one entry to undo and cannot be lost between the two halves of the \
     gesture. The rows under \"moving a key\" are that, including the two refusals - a frame \
     that already carries a key of the same property, and a frame that carries none to move. \
     What the rows cannot cover is the gesture itself: that the mark follows the pointer and \
     that nothing is sent until it is let go is in the photographs beside this table.",
];

/// The effect records of one layer, out of the same JSON the panels are drawn from.
fn effect_records(viewer: &Mutex<Viewer>, layer_id: &str) -> Vec<serde_json::Value> {
    let answer: serde_json::Value =
        serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
    answer["project"]["compositions"][0]["layers"]
        .as_array()
        .expect("a composition has layers")
        .iter()
        .find(|l| l["id"] == layer_id)
        .and_then(|l| l["effects"].as_array().cloned())
        .unwrap_or_default()
}

/// The stack in the order document 21 evaluates it: what each effect is and whether it runs.
fn stack(viewer: &Mutex<Viewer>, layer_id: &str) -> String {
    effect_records(viewer, layer_id)
        .iter()
        .map(|e| {
            format!(
                "{} {} {}",
                e["instance_id"].as_str().unwrap_or("?"),
                e["type_id"].as_str().unwrap_or("?"),
                if e["enabled"] == serde_json::Value::Bool(true) {
                    "on"
                } else {
                    "bypassed"
                }
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// One effect's settings, as they would be written to the file.
fn settings(viewer: &Mutex<Viewer>, layer_id: &str, instance: &str) -> String {
    effect_records(viewer, layer_id)
        .iter()
        .find(|e| e["instance_id"] == instance)
        .map(|e| e["parameters"].to_string())
        .unwrap_or_else(|| "(no such effect)".to_string())
}

/// D-291: an effect's number setting takes an expression by the route a transform's does,
/// and a colour, which takes keys only, is refused in a sentence.
#[test]
fn an_effect_setting_takes_an_expression() {
    let source = repo("Fixtures/projects/unknown_effect_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    run(&viewer, "effect.add?layer=layer-cel&type=core.tint");
    // D-401: the older Tint, whose colour is a keyable number setting.
    run(
        &viewer,
        "effect.set_parameters?layer=layer-cel&effect=fx-1&color=0,0,0&amount=0",
    );
    let expression = || {
        let held = held(&viewer);
        let comp = held
            .document
            .project()
            .composition(&held.composition)
            .cloned()
            .unwrap();
        let fx = comp
            .layer(&Id::new("layer-cel"))
            .unwrap()
            .effects
            .iter()
            .find(|e| e.instance_id.as_str() == "fx-1")
            .cloned()
            .unwrap();
        fx.expression("amount").map(|e| (e.text.clone(), e.enabled))
    };
    assert_eq!(
        run(
            &viewer,
            "property.set_expression?layer=layer-cel&prop=fx:fx-1:amount&text=time&on=true"
        ),
        "Change the expression of Tint's amount"
    );
    assert_eq!(expression(), Some(("time".to_string(), true)));
    run(
        &viewer,
        "property.set_expression?layer=layer-cel&prop=fx:fx-1:amount&text=time&on=false",
    );
    assert_eq!(
        expression(),
        Some(("time".to_string(), false)),
        "switched off and kept"
    );
    run(
        &viewer,
        "property.set_expression?layer=layer-cel&prop=fx:fx-1:amount",
    );
    assert_eq!(expression(), None, "no text removes it");
    assert_eq!(
        run(
            &viewer,
            "property.set_expression?layer=layer-cel&prop=fx:fx-1:color&text=1&on=true"
        ),
        "Tint's color cannot carry an expression; it takes keys only."
    );
    assert_eq!(
        run(
            &viewer,
            "property.set_expression?layer=layer-cel&prop=fx:fx-9:amount&text=1&on=true"
        ),
        "fx-9 is not an effect on this layer."
    );
}

/// W-31: effects pasted from a copy or a preset arrive whole, keys and all, as one entry to
/// undo, and what this build would drop is refused rather than pasted in part.
#[test]
fn pasted_effects_arrive_whole_or_not_at_all() {
    let source = repo("Fixtures/projects/unknown_effect_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let l = "layer-cel";
    let paste = |effects: &str| {
        run(
            &viewer,
            &format!("effect.paste?layer={l}&effects={effects}"),
        )
    };
    run(&viewer, "effect.add?layer=layer-cel&type=core.tint");
    run(
        &viewer,
        "effect.set_parameters?layer=layer-cel&effect=fx-1&color=1,0,0&amount=0.25",
    );
    run(
        &viewer,
        "keyframe.add_remove?layer=layer-cel&prop=fx:fx-1:amount&frame=0",
    );
    run(
        &viewer,
        "effect.add?layer=layer-cel&type=core.gaussian_blur",
    );
    let records = effect_records(&viewer, l);
    let [unknown, tint, blur] = &records[..] else {
        panic!("three effects: {records:?}")
    };
    let copied = serde_json::json!([tint, blur]).to_string();
    let depth = held(&viewer).document.undo_depth();

    assert_eq!(paste(&copied), "Add Tint and 1 more");
    assert_eq!(
        held(&viewer).document.undo_depth(),
        depth + 1,
        "one entry to undo"
    );
    let after = effect_records(&viewer, l);
    assert_eq!(after.len(), 5);
    // The copies are the originals under new identifiers, the tint's key with them.
    for (copy, original, id) in [(&after[3], tint, "fx-3"), (&after[4], blur, "fx-4")] {
        assert_eq!(copy["instance_id"], id);
        let mut same = copy.clone();
        same["instance_id"] = original["instance_id"].clone();
        assert_eq!(&same, original);
    }
    run(&viewer, "edit.undo");
    assert_eq!(
        effect_records(&viewer, l).len(),
        3,
        "one undo takes both away"
    );

    // Refused whole, with nothing changed.
    let depth = held(&viewer).document.undo_depth();
    let said = paste(&serde_json::json!([blur, unknown]).to_string());
    assert!(
        said.contains("vendor.future.effect cannot be pasted: this build does not have it."),
        "{said}"
    );
    let mut odd = tint.clone();
    odd["parameters"]["sparkle"] = serde_json::json!(3);
    let said = paste(&serde_json::json!([odd]).to_string());
    assert!(said.contains("sparkle"), "{said}");
    assert_eq!(held(&viewer).document.undo_depth(), depth);
    assert_eq!(effect_records(&viewer, l).len(), 3);
}

/// D-380: a new Channel Blur starts in After Effects' Blurriness, as Gaussian Blur (D-321).
#[test]
fn a_new_channel_blur_starts_in_blurriness() {
    let source = repo("Fixtures/projects/unknown_effect_project.json");
    let viewer = Mutex::new(open(&source).unwrap_or_else(|d| panic!("open: {}", d.message)));
    assert_eq!(
        run(&viewer, "effect.add?layer=layer-cel&type=core.channel_blur"),
        "Add Channel Blur"
    );
    let said = settings(&viewer, "layer-cel", "fx-1");
    assert!(said.contains(r#""units":"blurriness""#), "{said}");
    let all = "effect.set_parameters?layer=layer-cel&effect=fx-1&red_blurriness=10&green_blurriness=0&blue_blurriness=0&alpha_blurriness=0";
    let said = run(&viewer, &format!("{all}&units=sigma"));
    assert!(said.starts_with("Change "), "{said}");
    assert!(!settings(&viewer, "layer-cel", "fx-1").contains("units"));
    let said = run(&viewer, &format!("{all}&units=pixels"));
    assert!(
        said.contains(r#"units are "sigma" or "blurriness""#),
        "{said}"
    );
}

/// B-638 (D-759): a Stroke added from now on starts at After Effects' Brush Hardness 79 with
/// Stroke Sequentially on; a project saved before keeps the 75 and off its file holds, and saves
/// back byte for byte. Writes `verification/B-638_stroke_start_table.md`.
#[test]
fn b638_a_new_stroke_starts_at_after_effects_values() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/unknown_effect_project.json");
    let viewer = Mutex::new(open(&source).unwrap_or_else(|d| panic!("open: {}", d.message)));
    let said = run(&viewer, "effect.add?layer=layer-cel&type=core.stroke");
    report.check("Add a Stroke from the Effects panel", "Add Path Stroke", &said);
    let added = settings(&viewer, "layer-cel", "fx-1");
    let has = |text: &str, key: &str| text.contains(key);
    report.check(
        "The new Stroke's Brush Hardness",
        "\"brush_hardness\":79",
        if has(&added, r#""brush_hardness":79"#) { "\"brush_hardness\":79" } else { added.as_str() },
    );
    report.check(
        "The new Stroke's Stroke Sequentially",
        "\"stroke_sequentially\":\"on\"",
        if has(&added, r#""stroke_sequentially":"on""#) { "\"stroke_sequentially\":\"on\"" } else { added.as_str() },
    );
    // Typed back in the panel (every setting at once, as the page sends them), 75 and off are kept.
    let typed = run(
        &viewer,
        "effect.set_parameters?layer=layer-cel&effect=fx-1&mask=1&all_masks=off\
         &stroke_sequentially=off&color=%23ffffff&brush_size=2&brush_hardness=75&opacity=100\
         &start=0&end=100&spacing=15&paint_style=on_original",
    );
    let now = settings(&viewer, "layer-cel", "fx-1");
    report.check(
        "Typed in the panel: Brush Hardness 75, Stroke Sequentially off",
        "kept",
        if typed.starts_with("Change ")
            && has(&now, r#""brush_hardness":75"#)
            && has(&now, r#""stroke_sequentially":"off""#)
        {
            "kept".to_string()
        } else {
            format!("{typed}: {now}")
        },
    );

    // A project saved before D-759: FX-STROKE-001's file, written with 75 and off.
    let old = repo("Fixtures/stroke/fx_stroke_001.json");
    let viewer = Mutex::new(open(&old).unwrap_or_else(|d| panic!("open: {}", d.message)));
    let read = settings(&viewer, "art", "fx-0-0");
    report.check(
        "An older project's Stroke opens at what it saved",
        "75 and off",
        if has(&read, r#""brush_hardness":75"#) && has(&read, r#""stroke_sequentially":"off""#) {
            "75 and off".to_string()
        } else {
            read.clone()
        },
    );
    let text = std::fs::read_to_string(&old).expect("read the older project");
    let first = persist::load_str(&text).expect("the older project reads");
    let saved = persist::to_json(first.document.project(), &first.preserved);
    let again = persist::load_str(&saved).expect("the saved project reads");
    let resaved = persist::to_json(again.document.project(), &again.preserved);
    report.check(
        "Saved, opened and saved again, the older project's file",
        "the same bytes",
        if saved == resaved { "the same bytes".to_string() } else { "different bytes".to_string() },
    );
    let written: serde_json::Value = serde_json::from_str(&saved).expect("the saved file is JSON");
    let fx = &written["compositions"][0]["layers"][0]["effects"][0]["parameters"];
    report.check(
        "The older project's saved Brush Hardness and Stroke Sequentially",
        "75, \"off\"",
        format!("{}, {}", fx["brush_hardness"], fx["stroke_sequentially"]),
    );

    let failed = report.rows.iter().filter(|(_, e, a)| e != a).count();
    let mut out = String::from(
        "# B-638: a Stroke added from now on starts at After Effects' values (D-759)\n\n\
         Written by `b638_a_new_stroke_starts_at_after_effects_values` in `app/src/tests/editing.rs`. \
         A Stroke added in the window starts at Brush Hardness 79 with Stroke Sequentially on, as \
         After Effects' panel shows a fresh Stroke (5 of 5 tutorial panels, wave 3 lane J). A \
         project saved before keeps the 75 and off its file holds. No picture of an older project \
         changes, and no fixture is added or changed.\n\n\
         | Check | Expected | Actual | Result |\n|---|---|---|---|\n",
    );
    for (check, expected, actual) in &report.rows {
        let result = if expected == actual { "pass" } else { "FAIL" };
        out.push_str(&format!("| {check} | {} | {} | {result} |\n", cell(expected), cell(actual)));
    }
    out.push_str(&format!("\n{} of {} checks pass.\n", report.rows.len() - failed, report.rows.len()));
    std::fs::write(repo("verification/B-638_stroke_start_table.md"), out).expect("write the table");
    assert_eq!(failed, 0, "{:#?}", report.rows);
}

/// B-640 (D-761): a Radio Waves added from now on lives After Effects' 10 seconds and fades over
/// its last 5, in the composition's frames; a project saved before keeps the 96 and 48 its file
/// holds, and saves back byte for byte. Writes `verification/B-640_radio_waves_start_table.md`.
#[test]
fn b640_a_new_radio_waves_starts_at_after_effects_times() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/unknown_effect_project.json");
    let text = std::fs::read_to_string(&source).expect("read the project");
    let pick = |s: &str, key: &str| {
        let v: serde_json::Value = serde_json::from_str(s).unwrap_or_default();
        format!("{}", v[key])
    };
    // The same project at other frame rates, written beside the temporary files.
    for (numerator, denominator, label, life, fade) in [
        (24, 1, "24", "240", "120"),
        (30000, 1001, "29.97", "300", "150"),
        (25, 1, "25", "250", "125"),
        (120, 1, "120", "1000", "600"),
    ] {
        let path = if numerator == 24 {
            source.clone()
        } else {
            let at = std::env::temp_dir().join(format!("b640_radio_waves_{numerator}_{denominator}.json"));
            let moved = text.replace("\r\n", "\n").replacen(
                "\"numerator\": 24,\n        \"denominator\": 1",
                &format!("\"numerator\": {numerator},\n        \"denominator\": {denominator}"),
                1,
            );
            assert_ne!(moved, text.replace("\r\n", "\n"), "the project's frame rate was found");
            std::fs::write(&at, moved).expect("write the project at another rate");
            at
        };
        let viewer = Mutex::new(open(&path).unwrap_or_else(|d| panic!("open: {}", d.message)));
        let said = run(&viewer, "effect.add?layer=layer-cel&type=core.radio_waves");
        report.check(&format!("{label} fps: add a Radio Waves"), "Add Radio Waves", &said);
        let added = settings(&viewer, "layer-cel", "fx-1");
        report.check(
            &format!("{label} fps: the new Radio Waves' Lifespan and Fade-out Time, frames"),
            format!("{life}, {fade}"),
            format!("{}, {}", pick(&added, "lifespan"), pick(&added, "fade_out_time")),
        );
    }

    // Typed back in the panel (every setting at once, as the page sends them), 96 and 48 are kept.
    let viewer = Mutex::new(open(&source).unwrap_or_else(|d| panic!("open: {}", d.message)));
    run(&viewer, "effect.add?layer=layer-cel&type=core.radio_waves");
    let typed = run(
        &viewer,
        "effect.set_parameters?layer=layer-cel&effect=fx-1&producer_point=50,50&sides=64\
         &interval=24&expansion=5&orientation=0&direction=90&velocity=0&spin=0&lifespan=96\
         &opacity=100&fade_in_time=0&fade_out_time=48&start_width=5&end_width=5&profile=square\
         &color=%23ffffff",
    );
    let now = settings(&viewer, "layer-cel", "fx-1");
    report.check(
        "Typed in the panel: Lifespan 96, Fade-out Time 48",
        "kept",
        if typed.starts_with("Change ") && pick(&now, "lifespan") == "96" && pick(&now, "fade_out_time") == "48" {
            "kept".to_string()
        } else {
            format!("{typed}: {now}")
        },
    );

    // A project saved before D-761: FX-RWAVE-001's file, written with 96 and 48.
    let old = repo("Fixtures/radio_waves/fx_rwave_001.json");
    let viewer = Mutex::new(open(&old).unwrap_or_else(|d| panic!("open: {}", d.message)));
    let read = settings(&viewer, "art", "fx-0-0");
    report.check(
        "An older project's Radio Waves opens at what it saved",
        "96, 48",
        format!("{}, {}", pick(&read, "lifespan"), pick(&read, "fade_out_time")),
    );
    let text = std::fs::read_to_string(&old).expect("read the older project");
    let first = persist::load_str(&text).expect("the older project reads");
    let saved = persist::to_json(first.document.project(), &first.preserved);
    let again = persist::load_str(&saved).expect("the saved project reads");
    let resaved = persist::to_json(again.document.project(), &again.preserved);
    report.check(
        "Saved, opened and saved again, the older project's file",
        "the same bytes",
        if saved == resaved { "the same bytes".to_string() } else { "different bytes".to_string() },
    );
    let written: serde_json::Value = serde_json::from_str(&saved).expect("the saved file is JSON");
    let fx = &written["compositions"][0]["layers"][0]["effects"][0]["parameters"];
    report.check(
        "The older project's saved Lifespan and Fade-out Time",
        "96, 48",
        format!("{}, {}", fx["lifespan"], fx["fade_out_time"]),
    );

    let failed = report.rows.iter().filter(|(_, e, a)| e != a).count();
    let mut out = String::from(
        "# B-640: a Radio Waves added from now on starts at After Effects' times (D-761)\n\n\
         Written by `b640_a_new_radio_waves_starts_at_after_effects_times` in \
         `app/src/tests/editing.rs`. A Radio Waves added in the window lives 10 seconds and fades \
         over its last 5, as After Effects' panel shows a fresh Radio Waves (Lifespan 10 s in 5 \
         tutorials, Fade-out Time 5 s in 6, wave 3 lane n), worked in the composition's frames \
         to the nearest frame and held inside 1 to 1000. A project saved before keeps the 96 and \
         48 its file holds. No picture of an older project changes, and no fixture is added or \
         changed.\n\n\
         | Check | Expected | Actual | Result |\n|---|---|---|---|\n",
    );
    for (check, expected, actual) in &report.rows {
        let result = if expected == actual { "pass" } else { "FAIL" };
        out.push_str(&format!("| {check} | {} | {} | {result} |\n", cell(expected), cell(actual)));
    }
    out.push_str(&format!("\n{} of {} checks pass.\n", report.rows.len() - failed, report.rows.len()));
    std::fs::write(repo("verification/B-640_radio_waves_start_table.md"), out).expect("write the table");
    assert_eq!(failed, 0, "{:#?}", report.rows);
}

/// D-743: a new Fractal Noise starts at After Effects' Sub Influence 70 and Sub Scaling 56, and
/// each newly shown setting typed in the panel is kept.
#[test]
fn b622_fractal_noise_sub_settings_typed_in_the_panel() {
    let source = repo("Fixtures/projects/unknown_effect_project.json");
    let viewer = Mutex::new(open(&source).unwrap_or_else(|d| panic!("open: {}", d.message)));
    assert_eq!(
        run(
            &viewer,
            "effect.add?layer=layer-cel&type=core.fractal_noise"
        ),
        "Add Fractal Noise"
    );
    let said = settings(&viewer, "layer-cel", "fx-1");
    assert!(
        said.contains(r#""sub_influence":70"#) && said.contains(r#""sub_scaling":56"#),
        "{said}"
    );
    assert!(
        !said.contains("rotation") && !said.contains("overflow"),
        "{said}"
    );
    // What the panel sends: every number field and every switch at once.
    let all =
        "effect.set_parameters?layer=layer-cel&effect=fx-1&size=100&complexity=4&contrast=100\
               &brightness=0&evolution=0&speed=0&seed=0&opacity=100&offset=0,0&scale_width=100\
               &scale_height=100&cycle=0&dark_color=%23000000&light_color=%23ffffff&blend=normal\
               &fractal_type=basic&noise_type=smooth&invert=off";
    for (typed, kept) in [
        ("&rotation=36&sub_influence=70&sub_scaling=56&sub_rotation=0&sub_offset=0,0&overflow=allow_hdr&perspective_offset=off&center_subscale=off", r#""rotation":36"#),
        ("&rotation=0&sub_influence=77.5&sub_scaling=56&sub_rotation=0&sub_offset=0,0&overflow=allow_hdr&perspective_offset=off&center_subscale=off", r#""sub_influence":77.5"#),
        ("&rotation=0&sub_influence=70&sub_scaling=59.7&sub_rotation=0&sub_offset=0,0&overflow=allow_hdr&perspective_offset=off&center_subscale=off", r#""sub_scaling":59.7"#),
        ("&rotation=0&sub_influence=70&sub_scaling=56&sub_rotation=45&sub_offset=0,0&overflow=allow_hdr&perspective_offset=off&center_subscale=off", r#""sub_rotation":45"#),
        ("&rotation=0&sub_influence=70&sub_scaling=56&sub_rotation=0&sub_offset=5,3&overflow=allow_hdr&perspective_offset=off&center_subscale=off", r#""sub_offset":[5,3]"#),
        ("&rotation=0&sub_influence=70&sub_scaling=56&sub_rotation=0&sub_offset=0,0&overflow=clip&perspective_offset=off&center_subscale=off", r#""overflow":"clip""#),
        ("&rotation=0&sub_influence=70&sub_scaling=56&sub_rotation=0&sub_offset=0,0&overflow=allow_hdr&perspective_offset=on&center_subscale=off", r#""perspective_offset":"on""#),
        ("&rotation=0&sub_influence=70&sub_scaling=56&sub_rotation=0&sub_offset=0,0&overflow=allow_hdr&perspective_offset=off&center_subscale=on", r#""center_subscale":"on""#),
    ] {
        let said = run(&viewer, &format!("{all}{typed}"));
        assert!(said.starts_with("Change "), "{typed}: {said}");
        let now = settings(&viewer, "layer-cel", "fx-1");
        assert!(now.contains(kept), "{typed}: {now}");
    }
    // Out of range is refused in Fractal Noise's own words.
    let said = run(&viewer, &format!("{all}&sub_scaling=5"));
    assert!(
        said.contains("sub_scaling")
            || said.contains("Sub Scaling")
            || said.contains("sub scaling"),
        "{said}"
    );
    let said = run(&viewer, &format!("{all}&overflow=hdr"));
    assert!(said.contains("Fractal Noise's overflow"), "{said}");
}

#[test]
fn the_effects_panel_builds_a_stack_and_keeps_the_one_it_cannot_draw() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/unknown_effect_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let l = "layer-cel";

    // ---- what was in the file ---------------------------------------------------------------
    report.check(
        "the layer arrives with the effect no version of this build has",
        "fx-unknown-1 vendor.future.effect on",
        stack(&viewer, l),
    );

    // ---- adding ----------------------------------------------------------------------------
    report.check(
        "adding a blur says what was added, in the words undo will use",
        "Add Gaussian Blur",
        run(
            &viewer,
            "effect.add?layer=layer-cel&type=core.gaussian_blur",
        ),
    );
    report.check(
        "and it goes on the end of the stack, which is where it is evaluated last",
        "fx-unknown-1 vendor.future.effect on, fx-1 core.gaussian_blur on",
        stack(&viewer, l),
    );
    report.check(
        "a new effect starts at the setting that changes no pixels, in After Effects' Blurriness (D-321)",
        r#"{"sigma_px":0,"units":"blurriness"}"#,
        settings(&viewer, l, "fx-1"),
    );

    // ---- settings ---------------------------------------------------------------------------
    report.check(
        "setting the radius says which effect's settings changed",
        "Change Gaussian Blur settings",
        run(
            &viewer,
            "effect.set_parameters?layer=layer-cel&effect=fx-1&sigma_px=4.5",
        ),
    );
    report.check(
        "and the panels are given the new radius back",
        r#"{"sigma_px":4.5}"#,
        settings(&viewer, l, "fx-1"),
    );
    let depth = held(&viewer).document.undo_depth();
    report.check(
        "a negative radius is refused in document 21's own words, not clamped",
        "A Gaussian blur's sigma runs from 0 to 500, and this is -1. Choose a value inside \
         the range.",
        run(
            &viewer,
            "effect.set_parameters?layer=layer-cel&effect=fx-1&sigma_px=-1",
        ),
    );
    report.check(
        "text that is not a number is refused by the window, before the core sees it",
        "sigma_px needs a number. Not \"wide\".",
        run(
            &viewer,
            "effect.set_parameters?layer=layer-cel&effect=fx-1&sigma_px=wide",
        ),
    );
    report.check(
        "a setting left out is refused rather than filled in from a default",
        "What should sigma_px be set to?",
        run(&viewer, "effect.set_parameters?layer=layer-cel&effect=fx-1"),
    );
    report.check(
        "none of those three refusals changed the radius",
        r#"{"sigma_px":4.5}"#,
        settings(&viewer, l, "fx-1"),
    );
    report.check(
        "and none of them put anything in the history",
        depth,
        held(&viewer).document.undo_depth(),
    );

    // ---- an effect with more than one setting -------------------------------------------------
    run(&viewer, "effect.add?layer=layer-cel&type=core.tint");
    report.check(
        "a tint takes a colour and an amount together",
        r#"{"amount":0.25,"color":[1,0,0]}"#,
        {
            run(
                &viewer,
                "effect.set_parameters?layer=layer-cel&effect=fx-2&color=1,0,0&amount=0.25",
            );
            settings(&viewer, l, "fx-2")
        },
    );
    report.check(
        "an amount outside document 21's range is refused",
        "A tint amount runs from 0 to 1, and this is 1.5. Choose a value inside the range.",
        run(
            &viewer,
            "effect.set_parameters?layer=layer-cel&effect=fx-2&color=1,0,0&amount=1.5",
        ),
    );
    report.check(
        "a colour that is not three numbers is refused by the window",
        "color needs three numbers, like 1, 0.5, 0. Not \"1,0\".",
        run(
            &viewer,
            "effect.set_parameters?layer=layer-cel&effect=fx-2&color=1,0&amount=0.25",
        ),
    );
    // The audit of 2026-09-25: a part that was not a number used to be dropped before the
    // count, so four parts with one word among them passed as three numbers.
    report.check(
        "a colour with a word among its numbers is refused, not read as the other three",
        "color needs three numbers, like 1, 0.5, 0. Not \"1,abc,0,0.5\".",
        run(
            &viewer,
            "effect.set_parameters?layer=layer-cel&effect=fx-2&color=1,abc,0,0.5&amount=0.25",
        ),
    );
    report.check(
        "and the tint is as it was set",
        r#"{"amount":0.25,"color":[1,0,0]}"#,
        settings(&viewer, l, "fx-2"),
    );

    // ---- bypass ------------------------------------------------------------------------------
    // Document 24 calls this a bypass rather than a delete: the effect stays in the file with
    // its settings, and the picture is drawn without it.
    report.check(
        "bypassing an effect says so",
        "Bypass Gaussian Blur",
        run(&viewer, "effect.toggle_bypass?layer=layer-cel&effect=fx-1"),
    );
    report.check(
        "and the stack says which one is not running",
        "fx-unknown-1 vendor.future.effect on, fx-1 core.gaussian_blur bypassed, \
         fx-2 core.tint on",
        stack(&viewer, l),
    );
    report.check(
        "a bypassed effect keeps its settings",
        r#"{"sigma_px":4.5}"#,
        settings(&viewer, l, "fx-1"),
    );
    report.check(
        "switching it back on says that instead",
        "Switch Gaussian Blur on",
        run(&viewer, "effect.toggle_bypass?layer=layer-cel&effect=fx-1"),
    );

    // ---- the effect this build does not have ---------------------------------------------------
    // Document 19: an unknown effect record must survive load and save and may not be
    // silently discarded. It can be switched off and it can be removed, because both are
    // things a person decided; what cannot happen is this window writing settings for a
    // schema it has never seen.
    report.check(
        "the effect from another version can be bypassed like any other",
        "Bypass vendor.future.effect",
        run(
            &viewer,
            "effect.toggle_bypass?layer=layer-cel&effect=fx-unknown-1",
        ),
    );
    run(
        &viewer,
        "effect.toggle_bypass?layer=layer-cel&effect=fx-unknown-1",
    );
    report.check(
        "but its settings cannot be changed from here, and it says why",
        "vendor.future.effect is not an effect this build has, so its settings are kept as \
         the file wrote them and cannot be changed here.",
        run(
            &viewer,
            "effect.set_parameters?layer=layer-cel&effect=fx-unknown-1&strength=0.9",
        ),
    );
    report.check(
        "and its settings are the ones the file wrote",
        r#"{"strength":0.5}"#,
        settings(&viewer, l, "fx-unknown-1"),
    );

    // ---- deleting ------------------------------------------------------------------------------
    run(&viewer, "effect.add?layer=layer-cel&type=core.exposure");
    report.check(
        "deleting an effect from the middle of the stack names the one that went",
        "Remove Tint",
        run(&viewer, "effect.delete?layer=layer-cel&effect=fx-2"),
    );
    report.check(
        "and the rest of the stack is untouched, in the order it was in",
        "fx-unknown-1 vendor.future.effect on, fx-1 core.gaussian_blur on, \
         fx-3 core.exposure on",
        stack(&viewer, l),
    );
    report.check(
        "the gap a deleted effect left is not filled by the next one",
        "fx-unknown-1 vendor.future.effect on, fx-1 core.gaussian_blur on, \
         fx-3 core.exposure on, fx-4 core.gaussian_blur on",
        {
            run(
                &viewer,
                "effect.add?layer=layer-cel&type=core.gaussian_blur",
            );
            stack(&viewer, l)
        },
    );
    // ---- the order, which is the picture ---------------------------------------------
    // ADR-017 evaluates the stack in order, so a blur then an exposure is not an exposure
    // then a blur. Until W-03 the only way to change that order was to delete an effect and
    // add it again, which loses its settings on the way.
    report.check(
        "moving an effect earlier says where it went",
        "Move Exposure to position 1",
        run(&viewer, "effect.move_up?layer=layer-cel&effect=fx-3"),
    );
    report.check(
        "and the stack is in the new order, with nothing else disturbed",
        "fx-unknown-1 vendor.future.effect on, fx-3 core.exposure on, \
         fx-1 core.gaussian_blur on, fx-4 core.gaussian_blur on",
        stack(&viewer, l),
    );
    report.check(
        "moving it back later puts the stack where it was",
        "fx-unknown-1 vendor.future.effect on, fx-1 core.gaussian_blur on, \
         fx-3 core.exposure on, fx-4 core.gaussian_blur on",
        {
            run(&viewer, "effect.move_down?layer=layer-cel&effect=fx-3");
            stack(&viewer, l)
        },
    );
    // W-07: a card dragged up or down the stack lands at one position, sent once.
    report.check(
        "moving an effect straight to a position says where it went",
        "Move Gaussian Blur to position 0",
        run(&viewer, "effect.move?layer=layer-cel&effect=fx-4&to=0"),
    );
    report.check(
        "and it is there, with the others closed up behind it",
        "fx-4 core.gaussian_blur on, fx-unknown-1 vendor.future.effect on, \
         fx-1 core.gaussian_blur on, fx-3 core.exposure on",
        stack(&viewer, l),
    );
    report.check(
        "a position past the end is refused in words",
        "Position 9 is past the end of a stack of 4 effects.",
        run(&viewer, "effect.move?layer=layer-cel&effect=fx-4&to=9"),
    );
    report.check(
        "and so is the position it already has, rather than written as a change",
        "fx-4 is already at position 0.",
        run(&viewer, "effect.move?layer=layer-cel&effect=fx-4&to=0"),
    );
    run(&viewer, "effect.move?layer=layer-cel&effect=fx-4&to=3");
    report.check(
        "an effect this build does not have moves like any other",
        "fx-1 core.gaussian_blur on, fx-unknown-1 vendor.future.effect on, \
         fx-3 core.exposure on, fx-4 core.gaussian_blur on",
        {
            run(
                &viewer,
                "effect.move_down?layer=layer-cel&effect=fx-unknown-1",
            );
            stack(&viewer, l)
        },
    );
    run(
        &viewer,
        "effect.move_up?layer=layer-cel&effect=fx-unknown-1",
    );
    run(&viewer, "effect.delete?layer=layer-cel&effect=fx-4");
    run(&viewer, "effect.delete?layer=layer-cel&effect=fx-3");
    // D-86: smoothing finds steps a blur before it would hide, so it is added at the top.
    report.check(
        "line smoothing is added at the top of the stack, not the bottom",
        "fx-2 core.line_smooth on, fx-unknown-1 vendor.future.effect on, \
         fx-1 core.gaussian_blur on",
        {
            run(&viewer, "effect.add?layer=layer-cel&type=core.line_smooth");
            stack(&viewer, l)
        },
    );
    report.check(
        "and it starts at softness 50 and threshold 10",
        r#"{"softness":50,"threshold":10}"#,
        settings(&viewer, l, "fx-2"),
    );
    // D-87: selective colour blur matches exact colours, so it goes above even smoothing.
    report.check(
        "selective colour blur is added at the very top, above line smoothing",
        "fx-3 core.selective_color_blur on, fx-2 core.line_smooth on, \
         fx-unknown-1 vendor.future.effect on, fx-1 core.gaussian_blur on",
        {
            run(
                &viewer,
                "effect.add?layer=layer-cel&type=core.selective_color_blur",
            );
            stack(&viewer, l)
        },
    );
    report.check(
        "and it starts at blur 12 with no colour chosen",
        r#"{"blur":12,"colors":[]}"#,
        settings(&viewer, l, "fx-3"),
    );
    run(&viewer, "effect.delete?layer=layer-cel&effect=fx-2");
    report.check(
        "line smoothing added now goes below the selective colour blur, not above it",
        "fx-3 core.selective_color_blur on, fx-4 core.line_smooth on, \
         fx-unknown-1 vendor.future.effect on, fx-1 core.gaussian_blur on",
        {
            run(&viewer, "effect.add?layer=layer-cel&type=core.line_smooth");
            stack(&viewer, l)
        },
    );
    run(&viewer, "effect.delete?layer=layer-cel&effect=fx-4");
    run(&viewer, "effect.delete?layer=layer-cel&effect=fx-3");

    // ---- what the window refuses before the core sees it ------------------------------------------
    let depth = held(&viewer).document.undo_depth();
    report.check(
        "an effect that is not on this layer is said, not silently ignored",
        "fx-99 is not an effect on this layer.",
        run(&viewer, "effect.delete?layer=layer-cel&effect=fx-99"),
    );
    report.check(
        "no effect named at all is asked for",
        "Which effect? Choose one in the effects list.",
        run(&viewer, "effect.toggle_bypass?layer=layer-cel"),
    );
    report.check(
        "an effect type this build does not have is refused, and the ninety-three are named",
        "This build has no effect called core.not_yet_made. It has core.gaussian_blur, \
         core.exposure, core.tint, core.line_smooth, core.selective_color_blur, core.glow, \
         core.line_recolor, core.directional_blur, core.select_color, core.line_width, \
         core.radial_blur, core.bloom, core.color_key, core.curves, core.levels, \
         core.hue_saturation, core.gradient, core.drop_shadow, core.lens_blur, \
         core.rim_light, core.outline, core.noise, core.chromatic_aberration, \
         core.distance_gradation, core.light_rays, core.exposure_flicker, core.vignette, \
         core.turbulent_displace, core.fractal_noise, core.gradient_map, \
         core.color_balance, core.offset, core.light_wrap, core.invert, \
         core.brightness_contrast, core.black_white, core.posterize, core.threshold, \
         core.channel_mixer, core.vibrance, core.leave_color, core.solarize, core.halftone, \
         core.mosaic, core.emboss, core.find_edges, core.sharpen, core.diffusion, \
         core.wave_warp, core.ripple, core.twirl, core.bulge, core.mirror, \
         core.motion_tile, core.linear_wipe, core.radial_wipe, core.venetian_blinds, \
         core.iris_wipe, core.simple_choker, core.speed_lines, core.cross_glare, \
         core.camera_shake, core.rain, core.color_lookup, core.line_blur, \
         core.hsv_key, core.paraffin, core.kira_kira, core.lightning_bolt, \
         core.compound_blur, core.displacement_map, core.gradient_wipe, core.glass_melt, core.tile_wipe, core.teeth_wipe, core.glow_wipe, core.stripe_sweep, core.push_iris, core.stretch_wipe, core.twister, core.warp_blend, core.channel_wipe, core.halation, core.echo, core.cc_wide_time, core.force_motion_blur, core.cc_rainfall, core.wave_world, core.caustics, core.card_dance, core.shatter, core.time_difference, core.posterize_time, core.pixel_motion_blur, core.timewarp, core.warp_stabilizer, core.rolling_shutter_repair, \
         core.change_to_color, core.corner_pin, core.light_sweep, core.radio_waves, core.polar_coordinates, core.median, core.smart_blur, core.snowfall, core.kaleidoscope, core.roughen_edges, core.beam, core.four_color_gradient, core.cell_pattern, core.optics_compensation, core.radial_shadow, core.extract, core.bevel_alpha, core.bevel_edges, core.block_dissolve, core.shift_channels, core.solid_composite, core.channel_blur, core.fast_box_blur, core.fast_blur_legacy, core.gaussian_blur_legacy, core.colorama, core.glass, core.vector_blur, core.moment_map, core.pass_extract, core.depth_key, core.id_key, core.cryptomatte, core.depth_of_field, core.fog_3d, core.identifier, core.numbers, core.timecode, core.basic_text, core.text_animator, core.stretch_levels, core.stretch_contrast, core.stretch_color, core.spread_tones, core.matte_choker, core.refine_hard_matte, core.refine_soft_matte, core.stroke, core.bilateral_blur, core.cross_blur, core.spin_zoom_blur, core.fast_zoom_blur, core.broadcast_safe, core.color_neutralizer, core.color_offset, core.kernel, core.toner, core.change_color, core.bend_it, core.bender, core.blobbylize, core.glue_gun, core.threads, core.lens_flare, core.color_balance_hls, core.color_link, core.color_stabilizer, core.gamma_pedestal_gain, core.flow_motion, core.griddler, core.fisheye, core.levels_individual, core.photo_filter, core.color_grade, core.page_turn, core.power_pin, core.bezier_warp, core.mesh_warp, core.warp, core.liquify, core.reshape, core.mask_smear, core.ripple_pulse, core.slant, core.smear, core.split, core.split_2, core.tiles, core.magnify, core.spherize, core.checkerboard, core.circle, core.ellipse, core.detail_upscale, core.audio_spectrum, core.audio_waveform, core.fractal, core.grid, core.light_burst, core.fill, core.eyedropper_fill, core.paint_bucket, core.write_on, core.scribble, core.energy_stroke, core.add_grain, core.vegas, core.curl_noise, core.turbulent_noise, core.noise_alpha, core.noise_hls, core.noise_hls_auto, core.dust_scratches, core.match_grain, core.remove_grain, core.brush_strokes, core.cartoon, core.deband, core.advanced_spill_suppressor, core.color_difference_key, core.color_range, core.difference_matte, core.inner_outer_key, core.key_cleaner, core.linear_color_key, core.screen_key, core.spill_suppressor, core.reduce_interlace_flicker, core.unmult, core.three_d_glasses, core.cc_spotlight, core.cc_cylinder, core.cc_ball_action, core.particle_space, core.bubbles, core.drizzle, core.particle_plane, core.particle_playground, core.hair, core.particle_emitter, core.particle_grid, core.mercury, core.polygon_shards, core.foam, core.scatterize, core.star_burst, core.cc_environment, core.cc_sphere, core.card_wipe, core.simple_wire_removal, core.block_load, core.burn_film, core.hex_tile, core.mr_smoothie, core.plastic, core.repeat_tile, core.scatter, core.strobe_light, core.texturize, core.channel_threshold, core.threshold_rgb, core.contour_paint, core.cc_vignette, core.arbitrary_map, core.selective_color, core.shadow_highlight, core.tritone, core.aerial_haze, core.transform, core.noise_gate, core.compressor and core.distortion.",
        run(&viewer, "effect.add?layer=layer-cel&type=core.not_yet_made"),
    );
    report.check(
        "adding without saying which effect asks",
        "Which effect? Say core.gaussian_blur, core.exposure, core.tint, core.line_smooth, \
         core.selective_color_blur, core.glow, core.line_recolor, core.directional_blur, \
         core.select_color, core.line_width, core.radial_blur, core.bloom, core.color_key, \
         core.curves, core.levels, core.hue_saturation, core.gradient, core.drop_shadow, \
         core.lens_blur, core.rim_light, core.outline, core.noise, \
         core.chromatic_aberration, core.distance_gradation, core.light_rays, \
         core.exposure_flicker, core.vignette, core.turbulent_displace, core.fractal_noise, \
         core.gradient_map, core.color_balance, core.offset, core.light_wrap, core.invert, \
         core.brightness_contrast, core.black_white, core.posterize, core.threshold, \
         core.channel_mixer, core.vibrance, core.leave_color, core.solarize, core.halftone, \
         core.mosaic, core.emboss, core.find_edges, core.sharpen, core.diffusion, \
         core.wave_warp, core.ripple, core.twirl, core.bulge, core.mirror, \
         core.motion_tile, core.linear_wipe, core.radial_wipe, core.venetian_blinds, \
         core.iris_wipe, core.simple_choker, core.speed_lines, core.cross_glare, \
         core.camera_shake, core.rain, core.color_lookup, core.line_blur, \
         core.hsv_key, core.paraffin, core.kira_kira, core.lightning_bolt, \
         core.compound_blur, core.displacement_map, core.gradient_wipe, core.glass_melt, core.tile_wipe, core.teeth_wipe, core.glow_wipe, core.stripe_sweep, core.push_iris, core.stretch_wipe, core.twister, core.warp_blend, core.channel_wipe, core.halation, core.echo, core.cc_wide_time, core.force_motion_blur, core.cc_rainfall, core.wave_world, core.caustics, core.card_dance, core.shatter, core.time_difference, core.posterize_time, core.pixel_motion_blur, core.timewarp, core.warp_stabilizer, core.rolling_shutter_repair, \
         core.change_to_color, core.corner_pin, core.light_sweep, core.radio_waves, core.polar_coordinates, core.median, core.smart_blur, core.snowfall, core.kaleidoscope, core.roughen_edges, core.beam, core.four_color_gradient, core.cell_pattern, core.optics_compensation, core.radial_shadow, core.extract, core.bevel_alpha, core.bevel_edges, core.block_dissolve, core.shift_channels, core.solid_composite, core.channel_blur, core.fast_box_blur, core.fast_blur_legacy, core.gaussian_blur_legacy, core.colorama, core.glass, core.vector_blur, core.moment_map, core.pass_extract, core.depth_key, core.id_key, core.cryptomatte, core.depth_of_field, core.fog_3d, core.identifier, core.numbers, core.timecode, core.basic_text, core.text_animator, core.stretch_levels, core.stretch_contrast, core.stretch_color, core.spread_tones, core.matte_choker, core.refine_hard_matte, core.refine_soft_matte, core.stroke, core.bilateral_blur, core.cross_blur, core.spin_zoom_blur, core.fast_zoom_blur, core.broadcast_safe, core.color_neutralizer, core.color_offset, core.kernel, core.toner, core.change_color, core.bend_it, core.bender, core.blobbylize, core.glue_gun, core.threads, core.lens_flare, core.color_balance_hls, core.color_link, core.color_stabilizer, core.gamma_pedestal_gain, core.flow_motion, core.griddler, core.fisheye, core.levels_individual, core.photo_filter, core.color_grade, core.page_turn, core.power_pin, core.bezier_warp, core.mesh_warp, core.warp, core.liquify, core.reshape, core.mask_smear, core.ripple_pulse, core.slant, core.smear, core.split, core.split_2, core.tiles, core.magnify, core.spherize, core.checkerboard, core.circle, core.ellipse, core.detail_upscale, core.audio_spectrum, core.audio_waveform, core.fractal, core.grid, core.light_burst, core.fill, core.eyedropper_fill, core.paint_bucket, core.write_on, core.scribble, core.energy_stroke, core.add_grain, core.vegas, core.curl_noise, core.turbulent_noise, core.noise_alpha, core.noise_hls, core.noise_hls_auto, core.dust_scratches, core.match_grain, core.remove_grain, core.brush_strokes, core.cartoon, core.deband, core.advanced_spill_suppressor, core.color_difference_key, core.color_range, core.difference_matte, core.inner_outer_key, core.key_cleaner, core.linear_color_key, core.screen_key, core.spill_suppressor, core.reduce_interlace_flicker, core.unmult, core.three_d_glasses, core.cc_spotlight, core.cc_cylinder, core.cc_ball_action, core.particle_space, core.bubbles, core.drizzle, core.particle_plane, core.particle_playground, core.hair, core.particle_emitter, core.particle_grid, core.mercury, core.polygon_shards, core.foam, core.scatterize, core.star_burst, core.cc_environment, core.cc_sphere, core.card_wipe, core.simple_wire_removal, core.block_load, core.burn_film, core.hex_tile, core.mr_smoothie, core.plastic, core.repeat_tile, core.scatter, core.strobe_light, core.texturize, core.channel_threshold, core.threshold_rgb, core.contour_paint, core.cc_vignette, core.arbitrary_map, core.selective_color, core.shadow_highlight, core.tritone, core.aerial_haze, core.transform, core.noise_gate, core.compressor or core.distortion.",
        run(&viewer, "effect.add?layer=layer-cel"),
    );
    report.check(
        "and a layer that is not in this composition is named",
        "layer-gone is not a layer in this composition.",
        run(&viewer, "effect.add?layer=layer-gone&type=core.exposure"),
    );
    report.check(
        "the first effect in the stack refuses to go earlier, in words",
        "fx-unknown-1 is already first.",
        run(
            &viewer,
            "effect.move_up?layer=layer-cel&effect=fx-unknown-1",
        ),
    );
    report.check(
        "and the last one refuses to go later",
        "fx-1 is already last.",
        run(&viewer, "effect.move_down?layer=layer-cel&effect=fx-1"),
    );
    report.check(
        "none of those seven refusals put anything in the history",
        depth,
        held(&viewer).document.undo_depth(),
    );

    // ---- a locked layer ----------------------------------------------------------------------------
    run(&viewer, "layer.toggle_lock?layer=layer-cel");
    let depth = held(&viewer).document.undo_depth();
    report.check(
        "a locked layer refuses an effect, and says which rule stopped it",
        "The layer \"Cel\" is locked, so it was not changed. Unlock the layer to edit it.",
        run(&viewer, "effect.add?layer=layer-cel&type=core.exposure"),
    );
    report.check(
        "and refuses a settings change too",
        "The layer \"Cel\" is locked, so it was not changed. Unlock the layer to edit it.",
        run(
            &viewer,
            "effect.set_parameters?layer=layer-cel&effect=fx-1&sigma_px=2",
        ),
    );
    report.check(
        "and neither did the two the lock stopped",
        depth,
        held(&viewer).document.undo_depth(),
    );
    run(&viewer, "layer.toggle_lock?layer=layer-cel");
    report.check(
        "and the stack is the one that was built",
        "fx-unknown-1 vendor.future.effect on, fx-1 core.gaussian_blur on",
        stack(&viewer, l),
    );

    // ---- back to the file -----------------------------------------------------------------------------
    while held(&viewer).document.undo_depth() > 0 {
        undo(&viewer);
    }
    let held = held(&viewer);
    let opened = std::fs::read_to_string(&source)
        .expect("read the fixture")
        .replace("\r\n", "\n");
    let now = persist::to_json(held.document.project(), &held.preserved);
    let same = "identical, including the effect this build cannot model";
    report.check(
        "undoing every effect edit gives back the file that was opened",
        same,
        if opened == now {
            same
        } else {
            "what a save would write is no longer what was opened"
        },
    );
    drop(held);

    write_artifact(
        &report,
        "verification/B-12a_effects_table.md",
        "B-12a: what the effects panel does",
        EFFECTS_INTRO,
        EFFECTS_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

const EFFECTS_INTRO: &[&str] = &[
    "Document 21's three effects, built into a stack from the panel: added, bypassed, \
     deleted, and their settings typed. That each effect draws what document 21 says it \
     draws is checked in `verification/B-07_effects_table.md` and is not repeated here. What \
     is checked here is the part between a field in a panel and that arithmetic - that the \
     right effect is reached, that every setting travels on every change, and that a number \
     outside the range is refused in words rather than quietly turned into a different one.",
    "Every row runs on `Fixtures/projects/unknown_effect_project.json`, whose one layer \
     already carries an effect no version of this build has. It is there for the whole \
     table on purpose: a panel that lists effects is the place where an effect it cannot \
     draw is most likely to be dropped, and the last row saves the project back and compares \
     it to the file that was opened.",
];

const EFFECTS_NOTES: &[&str] = &[
    "## What to look at\n\n- **An effect this build does not have is kept, and is honest \
     about it.** It can be bypassed and it can be deleted, because both are things a person \
     decided. Its settings cannot be changed, because this build has never seen the schema \
     they belong to, and the panel says so in a sentence rather than showing empty \
     fields.\n- **A new effect starts at the setting that changes no pixels.** Adding a blur \
     does not make the picture jump; the change a person then sees is the one they \
     typed.\n- **Every setting travels on every change.** A tint is a colour and an amount, \
     and a request that named only one would leave the other to a default - which would \
     reset it. A missing setting is refused instead.\n- **Where a refusal comes from is \
     visible in its words.** \"needs a number\" is the window, refusing text. \"sigma runs from \
     0 to 500\" is the core, refusing a number document 21 has no meaning for. Both \
     reach the status line the same way, and neither changes anything.\n- **A gap left by a \
     deleted effect is not filled.** Identifiers are counted from the highest in use, so an \
     effect in the middle of a stack cannot be replaced by a different one wearing its \
     identifier, and two runs of the same steps write the same file.",
    "## What this does not cover\n\nThe picture. These rows check what the panel does to the \
     project, not what the renderer then draws; the blur, exposure and tint themselves are \
     checked against independently generated weights in `verification/B-07_effects_table.md`, \
     and the frame with an effect on it is in the photographs beside this table.\n\nReordering \
     the stack. Document 21 evaluates effects in order and this panel adds each new one at \
     the end; there is no command in document 24 for moving one, and W-01 does not ask to \
     move one.\n\nA parameter over time. The settings here are constants, which is what \
     document 19 calls a parameter with no keyframes on it.",
];

/// What shapes a layer, as the panels are given it.
fn matte(viewer: &Mutex<Viewer>, layer_id: &str) -> String {
    let answer: serde_json::Value =
        serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
    answer["project"]["compositions"][0]["layers"]
        .as_array()
        .expect("a composition has layers")
        .iter()
        .find(|l| l["id"] == layer_id)
        .map(|l| l["matte"].to_string())
        .unwrap_or_else(|| "(no such layer)".to_string())
}

#[test]
fn the_inspector_chooses_a_matte_and_refuses_the_ones_that_would_not_work() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/unknown_effect_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );

    report.check(
        "the fixture's layer is shaped by nothing",
        "null",
        matte(&viewer, "layer-cel"),
    );
    // A matte is one layer shaping another, so there has to be another. This is the second
    // layer a person would draw the shape on.
    run(&viewer, "layer.create?asset=asset-cel&name=Shape");

    // ---- choosing one ------------------------------------------------------------------------
    report.check(
        "choosing a matte says which layer was chosen",
        "Set matte to layer-1",
        run(
            &viewer,
            "layer.set_matte?layer=layer-cel&matte=layer-1&only=false",
        ),
    );
    report.check(
        "and the panels are given it back, with the matte layer still drawn in its own right",
        r#"{"layer_id":"layer-1","matte_only":false,"mode":"alpha"}"#,
        matte(&viewer, "layer-cel"),
    );
    // D-42: the matte layer being kept out of the visible stack is a setting of its own,
    // rather than the matte layer being switched off, so that a file cannot say one thing
    // and mean another.
    report.check(
        "keeping the matte layer out of the picture is said in the words undo will use",
        "Set matte to layer-1, matte only",
        run(
            &viewer,
            "layer.set_matte?layer=layer-cel&matte=layer-1&only=true",
        ),
    );
    report.check(
        "and it is that setting that changed, not the matte layer's own switch",
        r#"{"layer_id":"layer-1","matte_only":true,"mode":"alpha"}"#,
        matte(&viewer, "layer-cel"),
    );
    report.check(
        "the layer used as a matte is still switched on, which is D-42's whole point",
        true,
        layer(&viewer, "layer-1", |l| l.enabled).unwrap_or(false),
    );
    // D-293: the Track Matte mode, its own switch.
    report.check(
        "D-293: choosing luma says so in the words undo will use",
        "Set matte mode to luma inverted",
        run(
            &viewer,
            "layer.set_matte_mode?layer=layer-cel&mode=luma_inverted",
        ),
    );
    report.check(
        "D-293: changing the matte layer keeps the mode",
        r#"{"layer_id":"layer-1","matte_only":false,"mode":"luma_inverted"}"#,
        {
            run(
                &viewer,
                "layer.set_matte?layer=layer-cel&matte=layer-1&only=false",
            );
            matte(&viewer, "layer-cel")
        },
    );
    report.check(
        "D-293: a word that is not a mode is refused, not read as alpha",
        "\"lumen\" is not a matte mode. Choose alpha, alpha_inverted, luma or luma_inverted.",
        run(&viewer, "layer.set_matte_mode?layer=layer-cel&mode=lumen"),
    );
    run(&viewer, "layer.set_matte_mode?layer=layer-cel&mode=alpha");
    run(
        &viewer,
        "layer.set_matte?layer=layer-cel&matte=layer-1&only=true",
    );

    // ---- clearing it ---------------------------------------------------------------------------
    report.check(
        "clearing the matte says so",
        "Clear matte",
        run(&viewer, "layer.set_matte?layer=layer-cel&matte=&only=false"),
    );
    report.check(
        "and the layer is shaped by nothing again",
        "null",
        matte(&viewer, "layer-cel"),
    );
    report.check(
        "undo puts back the matte that was cleared, and its setting with it",
        r#"{"layer_id":"layer-1","matte_only":true,"mode":"alpha"}"#,
        {
            undo(&viewer);
            matte(&viewer, "layer-cel")
        },
    );

    // ---- what is refused ------------------------------------------------------------------------
    let depth = held(&viewer).document.undo_depth();
    report.check(
        "a layer that is not in this composition cannot be a matte, and is named",
        "The layer chosen as a matte, layer-gone, is not in this composition.",
        run(
            &viewer,
            "layer.set_matte?layer=layer-cel&matte=layer-gone&only=false",
        ),
    );
    // Document 19 forbids the cycle and the core refuses it. The list in the panel does not
    // offer a layer itself, so this is the second line of defence rather than the first.
    report.check(
        "a layer cannot be its own matte",
        "That matte would make two layers depend on each other. Choose a layer that does \
         not already use this one as its matte.",
        run(
            &viewer,
            "layer.set_matte?layer=layer-cel&matte=layer-cel&only=false",
        ),
    );
    report.check(
        "and two layers cannot shape each other",
        "That matte would make two layers depend on each other. Choose a layer that does \
         not already use this one as its matte.",
        run(
            &viewer,
            "layer.set_matte?layer=layer-1&matte=layer-cel&only=false",
        ),
    );
    report.check(
        "no layer named at all is asked for",
        "Which layer? Choose one in the layer list.",
        run(&viewer, "layer.set_matte?matte=layer-1"),
    );
    report.check(
        "none of those four refusals put anything in the history",
        depth,
        held(&viewer).document.undo_depth(),
    );
    report.check(
        "and the matte is the one that was chosen",
        r#"{"layer_id":"layer-1","matte_only":true,"mode":"alpha"}"#,
        matte(&viewer, "layer-cel"),
    );

    // ---- a locked layer ---------------------------------------------------------------------------
    run(&viewer, "layer.toggle_lock?layer=layer-cel");
    let depth = held(&viewer).document.undo_depth();
    report.check(
        "a locked layer refuses a matte, and says which rule stopped it",
        "The layer \"Cel\" is locked, so it was not changed. Unlock the layer to edit it.",
        run(&viewer, "layer.set_matte?layer=layer-cel&matte=&only=false"),
    );
    report.check(
        "which changed nothing",
        depth,
        held(&viewer).document.undo_depth(),
    );
    run(&viewer, "layer.toggle_lock?layer=layer-cel");

    // ---- the matte layer going away ----------------------------------------------------------------
    // A matte names a layer, and a layer can be deleted. The reference is kept rather than
    // cleared -- that is the project loader's rule, which reports it as a warning and leaves
    // it alone -- so what this window owes the person is to say it at the moment it happens
    // rather than the next time the file is opened.
    report.check(
        "deleting the layer used as a matte says what it did to the layer it was shaping",
        "Delete layer layer-1 \"Cel\" is now shaped by a layer that is not here; undo puts \
         it back.",
        run(&viewer, "layer.delete?layer=layer-1"),
    );
    report.check(
        "and the reference is kept, which is what the project loader expects to find",
        r#"{"layer_id":"layer-1","matte_only":true,"mode":"alpha"}"#,
        matte(&viewer, "layer-cel"),
    );
    report.check(
        "and undo brings the layer back, with the matte pointing at it again",
        "Cel, Shape",
        {
            undo(&viewer);
            names(&viewer)
        },
    );

    // ---- back to the file ---------------------------------------------------------------------------
    while held(&viewer).document.undo_depth() > 0 {
        undo(&viewer);
    }
    let held = held(&viewer);
    let opened = std::fs::read_to_string(&source)
        .expect("read the fixture")
        .replace("\r\n", "\n");
    let now = persist::to_json(held.document.project(), &held.preserved);
    let same = "identical, including the effect this build cannot model";
    report.check(
        "undoing everything gives back the file that was opened",
        same,
        if opened == now {
            same
        } else {
            "what a save would write is no longer what was opened"
        },
    );
    drop(held);

    write_artifact(
        &report,
        "verification/B-12a_matte_table.md",
        "B-12a: what the matte chooser does",
        MATTE_INTRO,
        MATTE_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// What a layer rides on, as the panels are given it.
fn parent(viewer: &Mutex<Viewer>, layer_id: &str) -> String {
    let answer: serde_json::Value =
        serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
    answer["project"]["compositions"][0]["layers"]
        .as_array()
        .expect("a composition has layers")
        .iter()
        .find(|l| l["id"] == layer_id)
        .map(|l| l["parent"].to_string())
        .unwrap_or_else(|| "(no such layer)".to_string())
}

#[test]
fn the_inspector_chooses_a_parent_and_says_when_a_layer_cannot_keep_its_shape() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/unknown_effect_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );

    report.check(
        "the fixture's layer rides on nothing",
        "null",
        parent(&viewer, "layer-cel"),
    );
    // Parenting is one layer riding on another, so there has to be another. This is the
    // layer a person would draw the head on, with the cel riding on it.
    run(&viewer, "layer.create?asset=asset-cel&name=Head");

    // ---- choosing one ------------------------------------------------------------------------
    report.check(
        "choosing a parent says which layer this one now rides on",
        "Set parent to layer-1",
        run(
            &viewer,
            "layer.set_parent?layer=layer-cel&parent=layer-1&frame=0",
        ),
    );
    report.check(
        "and the panels are given it back",
        "\"layer-1\"",
        parent(&viewer, "layer-cel"),
    );
    // D-57: a parent carries the transform and nothing else. A layer that has just been
    // given a parent is shaped by no more than it was before.
    report.check(
        "a parent is not a matte, and choosing one shaped the layer by nothing",
        "null",
        matte(&viewer, "layer-cel"),
    );

    // ---- clearing it ---------------------------------------------------------------------------
    report.check(
        "clearing the parent says so",
        "Clear parent",
        run(&viewer, "layer.set_parent?layer=layer-cel&parent=&frame=0"),
    );
    report.check(
        "and the layer rides on nothing again",
        "null",
        parent(&viewer, "layer-cel"),
    );
    report.check(
        "undo puts back the parent that was cleared",
        "\"layer-1\"",
        {
            undo(&viewer);
            parent(&viewer, "layer-cel")
        },
    );

    // ---- what is refused ------------------------------------------------------------------------
    let depth = held(&viewer).document.undo_depth();
    report.check(
        "a layer that is not in this composition cannot be a parent, and is named",
        "The layer chosen as a parent, layer-gone, is not in this composition.",
        run(
            &viewer,
            "layer.set_parent?layer=layer-cel&parent=layer-gone&frame=0",
        ),
    );
    // D-57 forbids the loop and the core refuses it. The list in the panel does not offer a
    // layer itself, so this is the second line of defence rather than the first.
    report.check(
        "a layer cannot ride on itself",
        "That parent would make two layers ride on each other. Choose a layer that is not \
         already riding on this one.",
        run(
            &viewer,
            "layer.set_parent?layer=layer-cel&parent=layer-cel&frame=0",
        ),
    );
    report.check(
        "and two layers cannot ride on each other",
        "That parent would make two layers ride on each other. Choose a layer that is not \
         already riding on this one.",
        run(
            &viewer,
            "layer.set_parent?layer=layer-1&parent=layer-cel&frame=0",
        ),
    );
    // The frame is what document 21 works the keep-place conversion at, so a request that
    // does not carry one has no answer rather than a default one.
    report.check(
        "a request that does not say which frame is refused rather than assuming one",
        "Which frame? Say frame=<frame>.",
        run(&viewer, "layer.set_parent?layer=layer-cel&parent=layer-1"),
    );
    report.check(
        "no layer named at all is asked for",
        "Which layer? Choose one in the layer list.",
        run(&viewer, "layer.set_parent?parent=layer-1&frame=0"),
    );
    report.check(
        "none of those five refusals put anything in the history",
        depth,
        held(&viewer).document.undo_depth(),
    );
    report.check(
        "and the parent is the one that was chosen",
        "\"layer-1\"",
        parent(&viewer, "layer-cel"),
    );

    // ---- a locked layer ---------------------------------------------------------------------------
    run(&viewer, "layer.toggle_lock?layer=layer-cel");
    let depth = held(&viewer).document.undo_depth();
    report.check(
        "a locked layer refuses a parent, and says which rule stopped it",
        "The layer \"Cel\" is locked, so it was not changed. Unlock the layer to edit it.",
        run(
            &viewer,
            "layer.set_parent?layer=layer-cel&parent=layer-1&frame=0",
        ),
    );
    report.check(
        "which changed nothing",
        depth,
        held(&viewer).document.undo_depth(),
    );
    run(&viewer, "layer.toggle_lock?layer=layer-cel");

    // ---- keeping its place, and saying when it could not keep its shape ----------------------
    // Document 21: the conversion is exact when every layer in the chain is scaled evenly,
    // or when nothing involved is turned. Otherwise the exact answer is a slant no layer can
    // hold: the anchor lands where it was and the rest of the layer moves. Both are allowed;
    // what separates them is whether the window says so.
    run(&viewer, "layer.set_parent?layer=layer-cel&parent=&frame=0");
    run(
        &viewer,
        "property.set_base?layer=layer-1&prop=scale&value=200,200",
    );
    report.check(
        "a parent scaled evenly keeps the layer exactly, and the answer says only what it did",
        "Set parent to layer-1",
        run(
            &viewer,
            "layer.set_parent?layer=layer-cel&parent=layer-1&frame=0",
        ),
    );
    undo(&viewer);
    run(
        &viewer,
        "property.set_base?layer=layer-1&prop=scale&value=200,50",
    );
    run(
        &viewer,
        "property.set_base?layer=layer-cel&prop=rotation&value=45",
    );
    report.check(
        "a parent stretched one way, under a turned layer, says the shape could not be kept",
        "Set parent to layer-1 It could not keep its shape exactly: the parent is stretched \
         more one way than the other and something in the chain is turned, so the anchor is \
         where it was and the rest of the layer has moved. Ctrl+Z undoes it.",
        run(
            &viewer,
            "layer.set_parent?layer=layer-cel&parent=layer-1&frame=0",
        ),
    );
    report.check(
        "and it did happen: the sentence is a warning about a change that landed",
        "\"layer-1\"",
        parent(&viewer, "layer-cel"),
    );

    // ---- the parent going away ----------------------------------------------------------------
    // This is the one place parenting does not follow the matte. A matte reference is kept
    // when its layer is deleted; a kept parent reference would move the layer across the
    // screen, so the children let go where they stand, in the same entry.
    // The children are converted before the layer goes, and they are converted by a
    // command the lock refuses, so a locked child stops the whole deletion rather than being
    // left pointing at a layer that is not there. That is the existing lock rule reaching a
    // new command rather than a rule parenting invented, and it is written on the playtest
    // sheet, so it is checked here rather than reasoned about.
    run(&viewer, "layer.toggle_lock?layer=layer-cel");
    report.check(
        "a locked layer riding on it stops the deletion rather than being left behind",
        "The layer \"Cel\" is locked, so it was not changed. Unlock the layer to edit it.",
        run(&viewer, "layer.delete?layer=layer-1&frame=0"),
    );
    report.check(
        "and neither layer went anywhere",
        "Cel, Head",
        names(&viewer),
    );
    run(&viewer, "layer.toggle_lock?layer=layer-cel");

    let depth = held(&viewer).document.undo_depth();
    report.check(
        "deleting a layer lets go of what was riding on it",
        "Clear parent and 1 more",
        run(&viewer, "layer.delete?layer=layer-1&frame=0"),
    );
    report.check(
        "the layer that was riding on it is still here, riding on nothing",
        "null",
        parent(&viewer, "layer-cel"),
    );
    report.check(
        "and letting go and deleting are one entry in the history, not two",
        depth + 1,
        held(&viewer).document.undo_depth(),
    );
    report.check("so one undo brings the layer back", "Cel, Head", {
        undo(&viewer);
        names(&viewer)
    });
    report.check(
        "with the layer riding on it again",
        "\"layer-1\"",
        parent(&viewer, "layer-cel"),
    );

    // ---- back to the file ---------------------------------------------------------------------------
    while held(&viewer).document.undo_depth() > 0 {
        undo(&viewer);
    }
    let held = held(&viewer);
    let opened = std::fs::read_to_string(&source)
        .expect("read the fixture")
        .replace("\r\n", "\n");
    let now = persist::to_json(held.document.project(), &held.preserved);
    let same = "identical, including the effect this build cannot model";
    report.check(
        "undoing everything gives back the file that was opened",
        same,
        if opened == now {
            same
        } else {
            "what a save would write is no longer what was opened"
        },
    );
    drop(held);

    write_artifact(
        &report,
        "verification/B-13b_panel_table.md",
        "B-13b: what the parent chooser does",
        PARENT_INTRO,
        PARENT_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

const PARENT_INTRO: &[&str] = &[
    "D-57 lets a layer ride on another layer's transform, so that a mouth cel moves with the \
     head cel it belongs to rather than being keyed twice. That the arithmetic is document \
     21's - the parent's whole transform applied after the child's own, up the chain - is \
     checked against independently generated numbers in \
     `verification/B-13b_parenting_table.md` and is not repeated here. What is checked here \
     is the part between a chooser in a panel and that arithmetic: that the layer chosen is \
     the layer used, that the arrangements D-57 forbids are refused in a sentence, and that \
     the two things this window owes a person at the moment they happen - a shape that could \
     not be kept exactly, and a parent being deleted - are said rather than left to be found \
     in the picture.",
    "One command carries choosing a parent, changing it and clearing it. \
     `layer.set_parent` is added to document 24 for it, and it is the only layer command \
     that carries the frame, because document 21 works the keep-place conversion at one \
     frame and on an animated parent the answer differs from frame to frame.",
];

const PARENT_NOTES: &[&str] = &[
    "## What to look at\n\n- **The layer keeps where it is on screen.** Choosing a parent is \
     a statement about what moves with what, not an instruction to move the layer, so the \
     values are converted through the parent's chain and the layer stays put. That is After \
     Effects' default and it is what the rows above check the answers of.\n- **When the \
     shape cannot be kept, the window says so rather than hiding it.** A parent stretched \
     more one way than the other, under a layer that is turned, asks for a slant no layer \
     can hold; the anchor lands exactly where it was and the rest of the layer moves. The \
     change is allowed and a sentence says what happened, with Ctrl+Z named in it.\n- \
     **Deleting a parent is the one place parenting does not copy the matte.** A deleted \
     matte leaves its reference behind, because the layer it shaped can wait to be undone \
     without moving. A child pointing at a deleted parent would jump instead, so the \
     children let go where they stand - and in the same entry, so that one undo brings back \
     the layer and everything that was riding on it.\n- **A loop is refused, twice over.** \
     The chooser does not offer a layer itself, and the core refuses the whole loop of which \
     that is only the shortest; the rows here go through the request the chooser sends, so \
     what they check is the second line of defence.\n- **Clearing is the same command with \
     no layer named**, which is what the \"none\" entry in the list sends. A layer that is \
     not in this composition is a different thing and is refused.",
    "## What this does not cover\n\nThe numbers. These rows check what the chooser does to \
     the project; where a parented layer's pixels actually land is FX-PARENT-001 to 008 in \
     `verification/B-13b_parenting_table.md`, worked from document 21's four steps by a \
     generator that never builds a matrix.\n\nParenting without keeping place. After Effects \
     offers it on a modifier key; D-57 leaves it undecided and nothing in W-04 asks for it, \
     so this window always keeps place.\n\nThe camera. B-13c is a separate contract, and \
     parenting does not imply it.",
];

/// D-58's depth on one layer, as the panels are given it: the base, or `null` where the
/// layer has never been given one and sits on the plane the composition is drawn at.
fn depth_of(viewer: &Mutex<Viewer>, layer_id: &str) -> String {
    let answer: serde_json::Value =
        serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
    answer["project"]["compositions"][0]["layers"]
        .as_array()
        .expect("a composition has layers")
        .iter()
        .find(|l| l["id"] == layer_id)
        .map(|l| match l.get("depth") {
            None | Some(serde_json::Value::Null) => "null".to_string(),
            Some(depth) => depth["base"].to_string(),
        })
        .unwrap_or_else(|| "(no such layer)".to_string())
}

/// One of the camera's three properties as the panels are given it, or `null` where the
/// composition has no camera at all -- which is what every composition says until one of
/// the three is touched.
fn camera_of(viewer: &Mutex<Viewer>, prop: &str) -> String {
    let answer: serde_json::Value =
        serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
    match answer["project"]["compositions"][0].get("camera") {
        None | Some(serde_json::Value::Null) => "null".to_string(),
        Some(camera) => camera[prop]["base"].to_string(),
    }
}

/// One of the camera's three properties' keyframes, written as `keys` writes a layer's.
/// Empty where there are none, and empty where there is no camera at all, which is the same
/// thing to a reader: nobody has keyed it.
fn camera_keys(viewer: &Mutex<Viewer>, prop: &str) -> String {
    let answer: serde_json::Value =
        serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
    match answer["project"]["compositions"][0]["camera"][prop]["keyframes"].as_array() {
        Some(list) => as_keys(list),
        None => String::new(),
    }
}

/// The lens the panel would show back, in millimetres, from the pixels the file holds.
/// This is the conversion of `drawCamera` worked the other way, and it is the round trip
/// that matters: a person types 50 and has to be shown 50 again.
fn lens_mm(viewer: &Mutex<Viewer>, width: f64) -> String {
    match camera_of(viewer, "zoom").parse::<f64>() {
        Ok(pixels) => format!("{}", pixels * 36.0 / width),
        Err(_) => "no camera".to_string(),
    }
}

#[test]
fn the_inspector_sets_a_depth_and_places_the_camera() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/unknown_effect_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );

    // ---- what a project without a camera says ------------------------------------------------
    report.check(
        "the fixture's layer sits on the plane the composition is drawn at",
        "null",
        depth_of(&viewer, "layer-cel"),
    );
    report.check(
        "and the composition has no camera at all until one is touched",
        "null",
        camera_of(&viewer, "zoom"),
    );

    // ---- a depth ---------------------------------------------------------------------------
    report.check(
        "setting a depth says which plane the layer was put on",
        "Set depth to 640",
        run(&viewer, "layer.set_depth?layer=layer-cel&depth=640"),
    );
    report.check(
        "and the panels are given it back",
        "640",
        depth_of(&viewer, "layer-cel"),
    );
    report.check(
        "a depth in front of the composition's plane is a negative number, not a refusal",
        "-200",
        {
            run(&viewer, "layer.set_depth?layer=layer-cel&depth=-200");
            depth_of(&viewer, "layer-cel")
        },
    );
    report.check("undo puts back the depth that was replaced", "640", {
        undo(&viewer);
        depth_of(&viewer, "layer-cel")
    });
    report.check(
        "and undoing the first one puts the layer back on the composition's own plane",
        "null",
        {
            undo(&viewer);
            depth_of(&viewer, "layer-cel")
        },
    );

    // ---- what a depth refuses ----------------------------------------------------------------
    let depth = held(&viewer).document.undo_depth();
    report.check(
        "a request with no depth in it is asked for one rather than given a default",
        "How far back? A depth is a number of pixels, and 0 is the plane the composition \
         is drawn at.",
        run(&viewer, "layer.set_depth?layer=layer-cel"),
    );
    report.check(
        "words are not a depth, and are quoted back",
        "\"six hundred\" is not a depth. A depth is a number of pixels.",
        run(&viewer, "layer.set_depth?layer=layer-cel&depth=six hundred"),
    );
    report.check(
        "no layer named at all is asked for",
        "Which layer? Choose one in the layer list.",
        run(&viewer, "layer.set_depth?depth=640"),
    );
    report.check(
        "none of those three refusals put anything in the history",
        depth,
        held(&viewer).document.undo_depth(),
    );

    // ---- a locked layer -----------------------------------------------------------------------
    run(&viewer, "layer.toggle_lock?layer=layer-cel");
    let depth = held(&viewer).document.undo_depth();
    report.check(
        "a locked layer refuses a depth, and says which rule stopped it",
        "The layer \"Cel\" is locked, so it was not changed. Unlock the layer to edit it.",
        run(&viewer, "layer.set_depth?layer=layer-cel&depth=640"),
    );
    report.check(
        "which changed nothing",
        depth,
        held(&viewer).document.undo_depth(),
    );
    run(&viewer, "layer.toggle_lock?layer=layer-cel");

    // ---- the camera ---------------------------------------------------------------------------
    report.check(
        "touching the lens says which of the camera's properties was set",
        "Set the camera's zoom",
        run(&viewer, "camera.set_property?property=zoom&value=50"),
    );
    // D-22: the millimetres are converted once, at this boundary. What the file holds is
    // the pixels the arithmetic uses, which for a 1920-wide composition is 1920 * 50 / 36.
    report.check(
        "the file holds the lens in pixels, not in millimetres",
        "2666.6666666666665",
        camera_of(&viewer, "zoom"),
    );
    report.check(
        "and the panel shows back the 50 mm that was typed",
        "50",
        lens_mm(&viewer, 1920.0),
    );
    report.check(
        "a camera that was not there is created by touching it, with the rest at its default",
        "[960,540]",
        camera_of(&viewer, "position"),
    );
    report.check(
        "including the depth, which is the default lens's own zoom, behind the frame",
        "-2666.6666666666665",
        camera_of(&viewer, "depth"),
    );
    report.check(
        "placing the camera takes both numbers in one request",
        "[760,540]",
        {
            run(
                &viewer,
                "camera.set_property?property=position&value=760,540",
            );
            camera_of(&viewer, "position")
        },
    );
    report.check("and its depth is one", "-1920", {
        run(&viewer, "camera.set_property?property=depth&value=-1920");
        camera_of(&viewer, "depth")
    });
    report.check(
        "undo takes back the camera one property at a time",
        "[960,540]",
        {
            undo(&viewer);
            undo(&viewer);
            camera_of(&viewer, "position")
        },
    );

    // ---- what the camera refuses ---------------------------------------------------------------
    let depth = held(&viewer).document.undo_depth();
    report.check(
        "a request naming no property is asked which one",
        "Which of the camera's properties? Its position, its depth or its zoom.",
        run(&viewer, "camera.set_property?value=50"),
    );
    report.check(
        "a property the camera does not have is named back with the three it does",
        "\"tilt\" is not one of the camera's properties. They are position, depth and zoom.",
        run(&viewer, "camera.set_property?property=tilt&value=50"),
    );
    report.check(
        "a place needs both numbers, and says so with an example",
        "The camera's position is two numbers, across and down: value=960,540.",
        run(&viewer, "camera.set_property?property=position&value=960"),
    );
    report.check(
        "a lens is one number, and says what it is measured on",
        "The camera's zoom is one number, a lens in millimetres on a 36 mm back: value=50.",
        run(&viewer, "camera.set_property?property=zoom&value=50,50"),
    );
    report.check(
        "words are not a number, and are quoted back",
        "\"wide\" is not a number.",
        run(&viewer, "camera.set_property?property=zoom&value=wide"),
    );
    report.check(
        "none of those five refusals put anything in the history",
        depth,
        held(&viewer).document.undo_depth(),
    );

    // ---- back to the file -----------------------------------------------------------------------
    while held(&viewer).document.undo_depth() > 0 {
        undo(&viewer);
    }
    let held = held(&viewer);
    let opened = std::fs::read_to_string(&source)
        .expect("read the fixture")
        .replace("\r\n", "\n");
    let now = persist::to_json(held.document.project(), &held.preserved);
    let same = "identical, with no camera and no depth anywhere in it";
    report.check(
        "undoing everything gives back the file that was opened",
        same,
        if opened == now {
            same
        } else {
            "what a save would write is no longer what was opened"
        },
    );
    drop(held);

    write_artifact(
        &report,
        "verification/B-13c_panel_table.md",
        "B-13c: what the depth row and the camera block do",
        CAMERA_INTRO,
        CAMERA_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// B-13d: a depth keys like any other number, which document 24 line 91 said before there
/// was anything to do it with. These are the gestures a person makes on the Depth row, in
/// the order `verification/B-13d_depth_keys_playtest.md` walks them.
#[test]
fn a_depth_keys_like_any_other_property() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/unknown_effect_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );

    // The bar goes to nought first, so the shift further down moves it a distance this
    // table can name rather than one that depends on where the fixture happened to leave it.
    run(&viewer, "layer.shift?layer=layer-cel&in=0");

    report.check(
        "a layer nobody has given a depth has no depth keys",
        "",
        keys(&viewer, "layer-cel", "depth"),
    );
    report.check(
        "the diamond puts a key where the playhead is, holding the value already there",
        "0@0 linear",
        {
            run(
                &viewer,
                "keyframe.add_remove?layer=layer-cel&prop=depth&frame=0",
            );
            keys(&viewer, "layer-cel", "depth")
        },
    );
    report.check(
        "changing the depth on another frame writes a second key rather than a base",
        "0@0 linear, 640@24 linear",
        {
            run(
                &viewer,
                "property.set_base?layer=layer-cel&prop=depth&value=640&frame=24",
            );
            keys(&viewer, "layer-cel", "depth")
        },
    );
    report.check(
        "and the base it did not touch is still the base",
        "0",
        base(&viewer, "layer-cel", "depth"),
    );
    report.check(
        "a depth key is taken hold of and put down on another frame like any other key",
        "0@0 linear, 640@30 linear",
        {
            run(
                &viewer,
                "keyframe.move?layer=layer-cel&prop=depth&from=24&to=30",
            );
            keys(&viewer, "layer-cel", "depth")
        },
    );
    report.check(
        "undo puts it back on the frame it came from",
        "0@0 linear, 640@24 linear",
        {
            undo(&viewer);
            keys(&viewer, "layer-cel", "depth")
        },
    );

    // The defect this unit went looking for. `ShiftLayer` moved the five transform tracks
    // and left the depth where it was, which nothing could see until a depth could key: a
    // layer dragged along the timeline would have desynchronised from its own depth.
    report.check(
        "dragging the layer's bar takes its depth keys with it, as it takes the others",
        "0@6 linear, 640@30 linear",
        {
            run(&viewer, "layer.shift?layer=layer-cel&in=6");
            keys(&viewer, "layer-cel", "depth")
        },
    );
    report.check(
        "and the position keys it always moved are still moved by the same distance",
        keys(&viewer, "layer-cel", "position"),
        keys(&viewer, "layer-cel", "position"),
    );
    report.check(
        "undoing the drag brings the depth keys back with the bar",
        "0@0 linear, 640@24 linear",
        {
            undo(&viewer);
            keys(&viewer, "layer-cel", "depth")
        },
    );
    report.check(
        "the diamond pressed again on a frame that has a key takes that key away",
        "640@24 linear",
        {
            run(
                &viewer,
                "keyframe.add_remove?layer=layer-cel&prop=depth&frame=0",
            );
            keys(&viewer, "layer-cel", "depth")
        },
    );
    report.check(
        "a property the window does not have is refused by name, and the six are named",
        "Which property? Say anchor, position, scale, rotation, opacity or depth.",
        run(
            &viewer,
            "property.set_base?layer=layer-cel&prop=wobble&value=1",
        ),
    );

    write_artifact(
        &report,
        "verification/B-13d_depth_keys_table.md",
        "B-13d: what a keyed depth does",
        DEPTH_KEYS_INTRO,
        DEPTH_KEYS_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

const DEPTH_KEYS_INTRO: &[&str] = &[
    "The owner played `verification/B-13c_camera_playtest.md` on 2026-09-15, reported that \
     the camera and the parallax work, and asked for one thing about the depth: that it be \
     \"keyable and drag-click like all the other changable values\". The drag was built \
     the same day. This table is the keying.",
    "There is no new contract behind it. Document 24 line 91 already said a depth keys like \
     any other number and document 19 line 25 already said it carries a base and keyframes \
     like every other animatable property, so what was missing was the building and not the \
     deciding. A depth became one of the property names the command layer takes, which is \
     why no row below names a command written for the depth: every one of them is a command \
     that already existed, reaching a depth because a depth is now a property.",
];

const DEPTH_KEYS_NOTES: &[&str] = &[
    "## What to look at\n\n- **The diamond holds the value that was already there.** The \
     first row after the empty one adds a key and the picture does not move, which is what \
     a stopwatch does everywhere else in this window.\n- **The second key writes \
     itself.** Changing a depth on a property that already has one key writes a key at the \
     frame the page names rather than setting the base, which is what After Effects does \
     and what the window already did for the other five. The row after it checks the base \
     was left alone, because that is the half of the rule a person cannot see.\n- **The \
     keys travel with the bar.** This is the row worth knowing about: the code that shifts \
     a layer moved its five transform tracks and left a depth behind, and nothing could see \
     it until a depth could hold a key. A layer dragged along the timeline would have come \
     apart from its own depth.\n- **Undo is one gesture at a time**, as document 26 has \
     it, and three rows here are undos.",
    "## What this does not cover\n\nWhere a layer at a depth actually lands. That is \
     D-58's arithmetic and it is checked against independently generated numbers in \
     `verification/B-13c_camera_table.md`, 105 of 105. A keyed depth is sampled at the \
     frame and then goes through the same two lines, so keying changed nothing about \
     it.\n\nEasing a depth key. F9 and the graph editor reach a depth because it is a \
     property, and `verification/D-52_ease_table.md` and `verification/D-53_path_table.md` \
     are where that machinery is checked; this table does not repeat their rendering of a \
     curve. Step 6 of the playtest sheet is where a person confirms it by \
     hand.\n\nWhether animating a depth is comfortable. No table can say that, which is \
     what `verification/B-13d_depth_keys_playtest.md` is for.\n\nThe camera, which was the \
     remaining half of D-58 when this was written and was built the same day: B-13e gave its \
     three properties the same diamonds, and `verification/B-13e_camera_keys_table.md` is \
     where they are checked.",
];

/// B-13e: the camera keys like any other property, which is what document 24 line 91 said
/// would happen once a command could name it as a target. These are the gestures a person
/// makes on the Camera rows, in the order `verification/B-13e_camera_keys_playtest.md`
/// walks them.
#[test]
fn the_camera_keys_like_any_other_property() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/unknown_effect_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );

    // D-58's default lens on this fixture's 1920-wide composition: 50 mm on a 36 mm back is
    // 1920 * 50 / 36 pixels, and the camera is pulled back by its own zoom. Every number
    // below is that one or a number typed in place of it.
    let lens = "2666.6666666666665";

    report.check(
        "a composition nobody has pointed a camera at has no camera keys",
        "",
        camera_keys(&viewer, "zoom"),
    );
    report.check(
        "the diamond puts a key where the playhead is, holding the lens already shown",
        format!("{lens}@0 linear"),
        {
            run(
                &viewer,
                "keyframe.add_remove?target=camera&prop=zoom&frame=0",
            );
            camera_keys(&viewer, "zoom")
        },
    );
    report.check(
        "keying a camera that is not in the file writes the default one the panel showed",
        "[960,540]",
        camera_of(&viewer, "position"),
    );
    report.check(
        "changing the lens on another frame writes a second key rather than a base",
        format!("{lens}@0 linear, 1920@24 linear"),
        {
            run(
                &viewer,
                "property.set_base?target=camera&prop=zoom&value=1920&frame=24",
            );
            camera_keys(&viewer, "zoom")
        },
    );
    report.check(
        "and the base it did not touch is still the base",
        lens,
        camera_of(&viewer, "zoom"),
    );
    // The key beside this one reads 1920, which is the lens in the pixels the wire carries.
    // This row is the other half of that: the base was not touched, so the panel still shows
    // the 50 mm it always showed, and a person watching the box sees no ghost movement.
    report.check(
        "and the millimetres the panel reads from that base are the 50 mm it always showed",
        "50",
        lens_mm(&viewer, 1920.0),
    );
    report.check(
        "a camera key is taken hold of and put down on another frame like any other key",
        format!("{lens}@0 linear, 1920@30 linear"),
        {
            run(
                &viewer,
                "keyframe.move?target=camera&prop=zoom&from=24&to=30",
            );
            camera_keys(&viewer, "zoom")
        },
    );
    report.check(
        "undo puts it back on the frame it came from",
        format!("{lens}@0 linear, 1920@24 linear"),
        {
            undo(&viewer);
            camera_keys(&viewer, "zoom")
        },
    );
    report.check(
        "a camera key holds rather than travels when it is told to",
        format!("{lens}@0 hold, 1920@24 linear"),
        {
            run(
                &viewer,
                "keyframe.set_interp?target=camera&prop=zoom&frame=0&mode=hold",
            );
            camera_keys(&viewer, "zoom")
        },
    );

    // ---- the place, which is the one of the three that can carry handles ------------------
    report.check(
        "the camera's place keys as a pair of numbers, as a layer's position does",
        "[960,540]@0 linear",
        {
            run(
                &viewer,
                "keyframe.add_remove?target=camera&prop=position&frame=0",
            );
            camera_keys(&viewer, "position")
        },
    );
    report.check(
        "and takes motion path handles, from a route that names no property at all",
        "[960,540]@0 linear path [-40,0,40,0]",
        {
            run(
                &viewer,
                "keyframe.set_path?target=camera&frame=0&handles=-40,0,40,0",
            );
            camera_keys(&viewer, "position")
        },
    );
    report.check(
        "a depth nobody has keyed is still set as a base, which is W-10's rule",
        "-1920",
        {
            run(
                &viewer,
                "property.set_base?target=camera&prop=depth&value=-1920",
            );
            camera_of(&viewer, "depth")
        },
    );

    // ---- what the camera refuses ------------------------------------------------------------
    let depth = held(&viewer).document.undo_depth();
    report.check(
        "a length is not a path, and is told so in the words of what was asked for",
        "The camera's zoom is a number, not a path. Only its place can carry handles.",
        run(
            &viewer,
            "keyframe.set_path?target=camera&prop=zoom&frame=0&handles=0,0,0,0",
        ),
    );
    report.check(
        "a property the camera does not have is named back with the three it does",
        "\"tilt\" is not one of the camera's properties. They are position, depth and zoom.",
        run(
            &viewer,
            "property.set_base?target=camera&prop=tilt&value=50",
        ),
    );
    report.check(
        "a request naming no property is asked which one",
        "Which of the camera's properties? Its position, its depth or its zoom.",
        run(&viewer, "property.set_base?target=camera&value=50"),
    );
    report.check(
        "a keyed property needs the frame the value belongs to, and says which to send",
        "The camera's zoom is keyframed, so a value belongs to a frame. Say frame=<frame>.",
        run(
            &viewer,
            "property.set_base?target=camera&prop=zoom&value=50",
        ),
    );
    report.check(
        "an interpolation the window does not have is quoted back with the three it does",
        "An interpolation is hold, linear or ease. Not \"wobble\".",
        run(
            &viewer,
            "keyframe.set_interp?target=camera&prop=zoom&frame=0&mode=wobble",
        ),
    );
    report.check(
        "a command that only a layer has is refused by name rather than asking which layer",
        "layer.rename changes a layer, not the camera. The camera takes a value, a \
         keyframe, a move, an ease and a path.",
        run(
            &viewer,
            "layer.rename?target=camera&prop=position&name=Camera",
        ),
    );
    report.check(
        "none of those six refusals put anything in the history",
        depth,
        held(&viewer).document.undo_depth(),
    );

    // ---- back to the file ------------------------------------------------------------------
    while held(&viewer).document.undo_depth() > 0 {
        undo(&viewer);
    }
    report.check(
        "undoing everything takes the camera back out of a file that never had one",
        "null",
        camera_of(&viewer, "zoom"),
    );

    write_artifact(
        &report,
        "verification/B-13e_camera_keys_table.md",
        "B-13e: what a keyed camera does",
        CAMERA_KEYS_INTRO,
        CAMERA_KEYS_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

const CAMERA_KEYS_INTRO: &[&str] = &[
    "The owner played `verification/B-13c_camera_playtest.md` on 2026-09-15 and asked for \
     the camera's values to update as they change, to be keyframable, and to drag like \
     every other changeable value. The first was already true. B-13d gave the depth the \
     other two the next day and this is the camera, which is the last piece of D-58.",
    "There is no new command behind it and no new contract. Document 24 line 91 said that \
     once a command could name the camera as its target instead of a layer, every property \
     command that already exists would reach it unchanged - a base, a key, a move, an ease, \
     a path - and that is what the rows below are. Every request in them is a command that \
     was already there, carrying `target=camera` where it usually carries a layer's id. The \
     owner chose the shape when asked: key it where it is, with the camera staying a \
     property of the composition rather than becoming a layer.",
];

const CAMERA_KEYS_NOTES: &[&str] = &[
    "## What to look at\n\n- **A camera nobody has pointed is keyed at the camera the \
     renderer would have used.** The first two rows key a composition whose file holds no \
     camera at all, and what gets written is D-58's default: the 50 mm lens on a 36 mm \
     back, in the middle, pulled back by its own zoom. Pressing the diamond holds what the \
     panel was already showing, which is what a stopwatch does everywhere else.\n- **The \
     lens crosses the wire in pixels.** `property.set_base` has always carried what the \
     file holds - an opacity goes over it as nought to one - so a lens goes over it as the \
     pixels D-58's arithmetic uses, and the millimetres a person reads are turned over at \
     the control beside opacity's conversion. The row that sets 1920 and reads back 36 mm \
     is that boundary in one line.\n- **The second key writes itself**, as it does for \
     every other animated property: changing a keyed camera value writes a key at the frame \
     the page names rather than moving the base. The row after it checks the base was left \
     alone, which is the half a person cannot see, and it is the reason the Camera block's \
     three boxes no longer send `camera.set_property` - that command sets a base, and on a \
     keyed camera it would have moved a number nothing is drawn from.\n- **Only the place \
     can carry handles.** A camera's place is a point that travels and its depth and lens \
     are lengths, so D-53's handles reach one of the three and are refused by name on the \
     other two.\n- **Six refusals and none of them in the history**, which is document \
     26's rule and is checked in one row after them.\n- **Undo takes the camera back out \
     of the file.** The last row reopens the question B-13c answered for the panel: a \
     project made before the camera existed is not given one by being looked at.",
    "## What this does not cover\n\nThe numbers. Where a layer at a depth actually lands is \
     D-58's arithmetic, checked against independently generated values in \
     `verification/B-13c_camera_table.md`. A keyed camera is sampled at the frame and then \
     goes through the same two lines, so keying changed nothing about it.\n\nEasing a \
     camera key. F9 reaches the camera's keys and `verification/D-52_ease_table.md` is where \
     that machinery is checked; the hold row above is enough to show the camera reaches it. \
     B-13e's notes said the graph editor reached it too, and that was wrong: the graph draws \
     whichever layer is selected, and the camera is deliberately not in the layer list, so \
     there is no way to point the graph at it. B-13f fixed F9 - which was sending a layer id \
     for a camera key, and being refused - and left the graph editor as work the owner has \
     not been asked about. Step 7 of the playtest sheet is where a person confirms the \
     rest.\n\nWhether animating a camera is comfortable, and whether the Camera group \
     belongs at the top of the timeline. No table can say either, which is what \
     `verification/B-13e_camera_keys_playtest.md` is for.\n\nA camera that is a layer. The \
     owner raised it as a question and chose to key the camera where it is instead, so the \
     camera is deliberately not in the layer list: not in the parent chooser, not in the \
     matte chooser, not selected by Select All and not deleted by Delete. D-58 names the \
     camera-layer rig as a later contract and it stays one.",
];

const CAMERA_INTRO: &[&str] = &[
    "D-58 gives a composition a camera and a layer a depth, so that a pan across layers set \
     at different distances parts the way it does under a rostrum: the near layer crosses \
     the frame faster than the far one, from one camera move rather than a key on every \
     layer. That the arithmetic is D-58's - one scale per layer, folded into document 21's \
     step 4 so the picture is still resampled once - is checked against independently \
     generated numbers in `verification/B-13c_camera_table.md` and is not repeated here. \
     What is checked here is the part between the controls and that arithmetic: that the \
     number typed is the number stored, that the millimetres a person reads are the pixels \
     the file holds, and that a composition nobody has pointed a camera at still saves \
     without one.",
    "Two commands carried it when this was written and neither is sent by the window any \
     more, which is worth saying because both still answer and both are still in document \
     24. `layer.set_depth` stopped on 2026-09-16, when B-13d made a depth one of the \
     properties the command layer takes by name, and it is kept for the keep-place \
     conversion parenting needs, which is not built. `camera.set_property` stopped the same \
     day, when B-13e made the camera something a property command can name as its target: a \
     box that set a base would have been a trap on a camera with keys, because the base is \
     not what is drawn on any frame but the first. The rows below are still the base values \
     and are unchanged. What a keyed depth does is \
     `verification/B-13d_depth_keys_table.md`, and what a keyed camera does is \
     `verification/B-13e_camera_keys_table.md`.",
];

const CAMERA_NOTES: &[&str] = &[
    "## What to look at\n\n- **A composition with no camera is shown the default, not \
     blanks.** The panel fills the three boxes with the 50 mm lens on a 36 mm back that \
     D-58 takes from After Effects, placed in the middle and pulled back by its own zoom, \
     because that is what the renderer would use. Nothing is written until one of them is \
     changed - the last row checks that undoing everything gives back a file with no \
     camera in it at all, which is why a project made before the camera existed opens and \
     saves unchanged.\n- **The lens is millimetres here and pixels in the file.** D-22 puts \
     a unit conversion at the boundary rather than in the middle, and this is a second \
     boundary of the same kind: 50 mm on a 1920-wide composition is stored as 2666.67 \
     pixels, and the panel converts it back so the person is shown the 50 they typed.\n- \
     **The depth was a typed box and is a scrub now.** It was typed here because a depth \
     was not a property the core took by name and could not be keyed, so a control that \
     dragged would have promised an animation this build could not write. B-13d removed the \
     reason on 2026-09-16, after the owner played B-13c and asked for exactly this: a depth \
     is a property like the five above it, it drags, and it keys.\n- **A \
     depth in front of the composition's plane is a negative number**, not a refusal. The \
     plane the composition is drawn at is 0 and a layer may sit either side of it; what is \
     refused is a layer at or behind the camera, which is `CAMERA_PLANE_BEHIND` and is \
     shown in the other table.\n- **A locked layer refuses a depth like any other edit**, \
     in the same sentence, naming the rule.",
    "## What this does not cover\n\nThe numbers. These rows check what the controls do to \
     the project; where a layer at a depth actually lands is FX-CAM-001 to 011 in \
     `verification/B-13c_camera_table.md`, worked from D-58 by a generator that never \
     builds a matrix.\n\nAnimating, which is no longer missing and is no longer here. The \
     depth got the diamond from B-13d and the camera from B-13e, both on 2026-09-16, and \
     the two tables are `verification/B-13d_depth_keys_table.md` and \
     `verification/B-13e_camera_keys_table.md`. The rows below are the base values, which \
     is what a property has before anybody keys it and what these controls still set on a \
     camera nobody has keyed.\n\nDragging the camera in the picture. The camera is placed \
     by dragging its numbers, not by pulling it about in the viewer; W-04 asks for a camera \
     move and not for a handle to make it with.",
];

const MATTE_INTRO: &[&str] = &[
    "W-01 asks the artist to apply a matte: one layer shaping another, which is how a cel is \
     held inside a shape rather than being cut with a pair of scissors. That the matte is \
     then drawn the way document 21 says - the alpha of one layer multiplying the other's - \
     is checked in `verification/B-06_mask_table.md` and is not repeated here. What is \
     checked here is the part between a chooser in a panel and that arithmetic: that the \
     layer chosen is the layer used, that the settings the file holds together travel \
     together, and that the arrangements document 19 forbids are refused in a sentence.",
    "One command carries all three of choosing a matte, clearing it, and choosing whether \
     the matte layer is still drawn in its own right. `layer.set_matte` is added to \
     document 24 for it, which is written up beside the row.",
];

const MATTE_NOTES: &[&str] = &[
    "## What to look at\n\n- **The matte layer is not switched off.** D-42 made \"keep this \
     layer out of the visible stack\" a setting of its own rather than reusing the layer's \
     own switch, so that a file showing a layer switched off while it visibly shapes the \
     picture cannot happen. The row after the one that sets it checks the matte layer is \
     still on.\n- **A cycle is refused, twice over.** The chooser does not offer a layer \
     itself, and the core refuses it as well; the rows here go through the request the \
     chooser sends, so what they check is the second line of defence.\n- **Deleting the \
     layer that was being used as a matte keeps the reference, and says so.** That is the \
     project loader's rule - it reports such a reference as a warning and leaves it alone, \
     so that a layer deleted by mistake can be undone back into place - and this window \
     says it in the status line at the moment it happens rather than leaving it to be met \
     the next time the file is opened.\n- **Clearing is the same command with no layer named**, which is what the \
     \"none\" entry in the list sends. A layer that has gone is a different thing and is \
     refused.",
    "## What this does not cover\n\nThe shape. This panel chooses which layer is the matte; \
     drawing an outline to be the matte is `SetMask` in the core and has no gesture in this \
     window yet. A matte is a whole layer here, which is what an artist with a scanned shape \
     has and what W-01 describes.\n\nWhich channel the matte uses. Document 21 defines an \
     alpha matte and this build has one kind; a luminance matte is not in G1.",
];

/// A layer's exposures as an exposure sheet reads them: which frames, and which drawing.
fn exposures(viewer: &Mutex<Viewer>, layer_id: &str) -> String {
    layer(viewer, layer_id, |l| {
        l.exposure_spans
            .iter()
            .map(|s| {
                format!(
                    "{}-{}:{}",
                    s.start_frame, s.end_frame_exclusive, s.drawing_number
                )
            })
            .collect::<Vec<_>>()
            .join(", ")
    })
    .unwrap_or_else(|| "(no such layer)".to_string())
}

/// The project's drawings, as the media bin lists them.
fn bin(viewer: &Mutex<Viewer>) -> String {
    held(viewer)
        .document
        .project()
        .assets
        .iter()
        .map(|a| format!("{} \"{}\" {} drawings", a.id, a.name, a.frames.len()))
        .collect::<Vec<_>>()
        .join("; ")
}

/// Where one drawing of an imported sequence ended up in the project record.
fn stored(viewer: &Mutex<Viewer>, asset: &str, drawing: u32) -> String {
    held(viewer)
        .document
        .project()
        .assets
        .iter()
        .find(|a| a.id == Id::new(asset))
        .and_then(|a| a.frames.get(&drawing).cloned())
        .unwrap_or_else(|| "(no such drawing)".to_string())
}

fn notes_mention(viewer: &Mutex<Viewer>, words: &str) -> bool {
    held(viewer).notes.iter().any(|n| n.contains(words))
}

/// The PNG files in one folder of the reference shot, which is the selection a person makes
/// in the dialog. Sorted, because a file dialog's order is its own business.
fn drawings_in(folder: &str) -> Vec<PathBuf> {
    let dir = repo(&format!("Fixtures/reference_shot/{folder}"));
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "png"))
        .collect();
    files.sort();
    files
}

/// The request the media bin sends once the file dialog has been answered.
fn import_of(files: &[PathBuf]) -> String {
    let parts: Vec<String> = files
        .iter()
        .map(|p| format!("file={}", for_a_header(&p.display().to_string())))
        .collect();
    format!("media.import?{}", parts.join("&"))
}

#[test]
fn the_media_bin_imports_a_sequence_and_the_exposures_it_is_given() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );

    // ---- the exposures the file already holds -------------------------------------------
    report.check(
        "the fixture holds drawing 1 for two frames and drawing 2 for three",
        "0-2:1, 2-5:2",
        exposures(&viewer, "layer-cel"),
    );
    {
        let answer: serde_json::Value =
            serde_json::from_str(&state(&viewer)).expect("the state answer is JSON");
        report.check(
            "and the panels are given them to draw",
            r#"[{"drawing_number":1,"end_frame_exclusive":2,"start_frame":0},"#.to_string()
                + r#"{"drawing_number":2,"end_frame_exclusive":5,"start_frame":2}]"#,
            answer["project"]["compositions"][0]["layers"][0]["exposure_spans"].to_string(),
        );
    }

    // ---- changing one ------------------------------------------------------------------
    report.check(
        "shortening a hold says how many exposures the layer now has",
        "Set 2 exposures",
        run(
            &viewer,
            "exposure.set_span?layer=layer-cel&was=2&start=2&end=4&drawing=2",
        ),
    );
    report.check(
        "and the hold is one frame shorter, with the other one untouched",
        "0-2:1, 2-4:2",
        exposures(&viewer, "layer-cel"),
    );
    // Document 20's own example: drawing numbers are under no ordering constraint, because a
    // re-exposure of an earlier drawing is ordinary cel work.
    report.check(
        "a later exposure may show an earlier drawing again",
        "Set 3 exposures",
        run(
            &viewer,
            "exposure.set_span?layer=layer-cel&start=4&end=5&drawing=1",
        ),
    );
    report.check(
        "and the sheet reads in frame order, not drawing order",
        "0-2:1, 2-4:2, 4-5:1",
        exposures(&viewer, "layer-cel"),
    );
    // Moving a row is one row edited. The request carries the frame the row starts at now as
    // well as the one being typed, so that an exposure dragged along the sheet does not
    // leave a copy of itself where it was.
    report.check(
        "moving an exposure to another frame moves it rather than copying it",
        "0-2:1, 2-4:2, 5-6:1",
        {
            run(
                &viewer,
                "exposure.set_span?layer=layer-cel&was=4&start=5&end=6&drawing=1",
            );
            exposures(&viewer, "layer-cel")
        },
    );
    report.check(
        "clearing one says how many are left",
        "Set 2 exposures",
        run(
            &viewer,
            "exposure.set_span?layer=layer-cel&was=5&start=5&drawing=",
        ),
    );
    report.check(
        "and it is the cleared one that has gone",
        "0-2:1, 2-4:2",
        exposures(&viewer, "layer-cel"),
    );

    // ---- a drawing the sequence has not got -----------------------------------------------
    // Document 28: a frame whose drawing is missing renders as nothing and no neighbouring
    // drawing is put there instead. The command is allowed - a person may be exposing a
    // drawing that has not been scanned yet - and is told what it will look like.
    report.check(
        "exposing a drawing that is not in the sequence is allowed and said",
        "Set 3 exposures Drawing 7 is not in this sequence, so the frames exposing it stay \
         empty; no neighbouring drawing is put there instead.",
        run(
            &viewer,
            "exposure.set_span?layer=layer-cel&start=4&end=5&drawing=7",
        ),
    );
    report.check(
        "and it was applied, not refused",
        "0-2:1, 2-4:2, 4-5:7",
        exposures(&viewer, "layer-cel"),
    );
    report.check(
        "undo takes that exposure back off the sheet",
        "0-2:1, 2-4:2",
        {
            undo(&viewer);
            exposures(&viewer, "layer-cel")
        },
    );

    // ---- what is refused ------------------------------------------------------------------
    let depth = held(&viewer).document.undo_depth();
    // Document 20's rule, in the core's words. The window does not have a copy of it.
    report.check(
        "two exposures cannot cover one frame",
        "Those exposures cannot be used: exposure spans overlap or are out of order: a span \
         ends at 2 and the next starts at 1.",
        run(
            &viewer,
            "exposure.set_span?layer=layer-cel&start=1&end=3&drawing=2",
        ),
    );
    report.check(
        "an exposure that covers no frame is not an exposure",
        "Those exposures cannot be used: exposure span [6, 6) covers no frame.",
        run(
            &viewer,
            "exposure.set_span?layer=layer-cel&start=6&end=6&drawing=1",
        ),
    );
    report.check(
        "clearing an exposure that is not there says so",
        "There is no exposure starting at frame 9 to clear.",
        run(
            &viewer,
            "exposure.set_span?layer=layer-cel&start=9&drawing=",
        ),
    );
    report.check(
        "a frame number that is not a number is named before the core sees it",
        "An exposure starts at a whole frame number. Not \"two\".",
        run(
            &viewer,
            "exposure.set_span?layer=layer-cel&start=two&end=4&drawing=1",
        ),
    );
    report.check(
        "and so is a drawing number that is not one",
        "A drawing number is a whole number, counting from zero. Not \"-1\".",
        run(
            &viewer,
            "exposure.set_span?layer=layer-cel&start=6&end=7&drawing=-1",
        ),
    );
    report.check(
        "an exposure with no end is asked for rather than guessed at",
        "Which frame does the exposure end before?",
        run(
            &viewer,
            "exposure.set_span?layer=layer-cel&start=6&drawing=1",
        ),
    );
    report.check(
        "none of those six refusals put anything in the history",
        depth,
        held(&viewer).document.undo_depth(),
    );
    report.check(
        "and the sheet is the one that was there",
        "0-2:1, 2-4:2",
        exposures(&viewer, "layer-cel"),
    );
    run(&viewer, "layer.toggle_lock?layer=layer-cel");
    let depth = held(&viewer).document.undo_depth();
    report.check(
        "a locked layer refuses an exposure, and says which rule stopped it",
        "The layer \"Cel\" is locked, so it was not changed. Unlock the layer to edit it.",
        run(
            &viewer,
            "exposure.set_span?layer=layer-cel&start=6&end=7&drawing=1",
        ),
    );
    report.check(
        "which changed nothing",
        depth,
        held(&viewer).document.undo_depth(),
    );
    run(&viewer, "layer.toggle_lock?layer=layer-cel");

    // ---- importing drawings -----------------------------------------------------------------
    report.check(
        "the media bin starts with the one sequence the file names",
        "asset-cel \"Cel\" 2 drawings",
        bin(&viewer),
    );
    report.check(
        "an import that names no file asks for one rather than importing nothing",
        "Which drawings should be imported? Choose the files themselves, not the folder \
         they are in.",
        run(&viewer, "media.import"),
    );
    // Layer 3 of the reference shot is the sequence with a hole in it: eleven files
    // numbered 0 to 11, and no drawing 7.
    let layer3 = drawings_in("layer3");
    report.check(
        "importing a sequence with a gap says how many drawings arrived and what is missing",
        "Import layer3_%03d.png: 11 drawings, numbered 0 to 11, and drawing 7 is missing.",
        run(&viewer, &import_of(&layer3)),
    );
    report.check(
        "the drawing that is missing is missing from the record, not filled in",
        "0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11",
        held(&viewer)
            .document
            .project()
            .assets
            .iter()
            .find(|a| a.id == Id::new("asset-1"))
            .map(|a| {
                a.frames
                    .keys()
                    .map(u32::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_else(|| "(the sequence was not imported)".to_string()),
    );
    report.check(
        "the importer's own warning about the gap is kept, in its words",
        true,
        notes_mention(&viewer, "One drawing is missing from layer3_%03d.png: 7."),
    );
    report.check(
        "media chosen from outside the project's folder keeps its whole path",
        true,
        stored(&viewer, "asset-1", 0).ends_with("Fixtures/reference_shot/layer3/layer3_000.png"),
    );
    report.check(
        "and it is written with forward slashes, so the project can be handed over",
        true,
        !stored(&viewer, "asset-1", 0).contains('\\'),
    );
    // Layer 2 is the other trap: drawing 13 is called `layer2_桜_013.png`, which the pattern
    // does not generate. It is a complete sequence and must not be reported as one with a
    // hole at 13.
    let layer2 = drawings_in("layer2");
    report.check(
        "a sequence whose names are not all alike is complete, not full of holes",
        "Import layer2_%03d.png: 24 drawings, numbered 0 to 23.",
        run(&viewer, &import_of(&layer2)),
    );
    report.check(
        "the drawing with the Japanese name is stored under the name it has",
        true,
        stored(&viewer, "asset-2", 13).ends_with("layer2_桜_013.png"),
    );
    report.check(
        "and it was noticed rather than passed over in silence",
        true,
        notes_mention(
            &viewer,
            "One file does not match the pattern layer2_%03d.png but carries a clear number",
        ),
    );
    report.check(
        "both sequences are in the bin, beside the one the file came with",
        "asset-cel \"Cel\" 2 drawings; asset-1 \"layer3_%03d.png\" 11 drawings; \
         asset-2 \"layer2_%03d.png\" 24 drawings",
        bin(&viewer),
    );
    report.check(
        "an import can be undone",
        "Undone: Import layer2_%03d.png",
        undo(&viewer),
    );
    report.check(
        "and the drawings go with it",
        "asset-cel \"Cel\" 2 drawings; asset-1 \"layer3_%03d.png\" 11 drawings",
        bin(&viewer),
    );
    // Not every file is a drawing. A selection that forms no sequence imports nothing and
    // says what the importer made of it.
    report.check(
        "a file that is not a numbered drawing imports nothing and says why",
        "The chosen file does not form an image sequence, so nothing was imported. One \
         selected file has no number in its name and was not imported. Import it as a still \
         image, or rename it so it carries a drawing number.",
        run(&viewer, &import_of(std::slice::from_ref(&source))),
    );

    // ---- back to the file ---------------------------------------------------------------
    while held(&viewer).document.undo_depth() > 0 {
        undo(&viewer);
    }
    let held = held(&viewer);
    let opened = std::fs::read_to_string(&source)
        .expect("read the fixture")
        .replace("\r\n", "\n");
    let now = persist::to_json(held.document.project(), &held.preserved);
    let same = "identical, drawings, exposures and all";
    report.check(
        "undoing everything gives back the file that was opened",
        same,
        if opened == now {
            same
        } else {
            "what a save would write is no longer what was opened"
        },
    );
    drop(held);

    write_artifact(
        &report,
        "verification/B-12a_media_table.md",
        "B-12a: what the media bin and the exposure sheet do",
        MEDIA_INTRO,
        MEDIA_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

const MEDIA_INTRO: &[&str] = &[
    "W-01 begins before any layer exists: the artist \"imports media, reviews sequence \
     grouping and missing-frame warnings, creates a composition and assigns exposures\". \
     This is the window's half of those steps. That B-03 groups a selection correctly is \
     checked in `verification/B-03_import_table.md`, and that the exposure sheet resolves \
     to the right drawing at each frame in `verification/B-04_exposure_table.md`; neither \
     is repeated here. What is checked here is the part between a panel and those two: that \
     what the importer found reaches the person in words, that the record written into the \
     project is the one the importer described, and that a row typed into the exposure \
     sheet becomes the span the file holds.",
    "The exposures run on `Fixtures/projects/cel_holds_project.json`, which already holds a \
     drawing for two frames and another for three. The imports run on the reference shot, \
     whose layer 3 is missing drawing 7 and whose layer 2 carries drawing 13 under a \
     Japanese name the pattern does not generate. Those two folders are the reason the \
     importer was written the way it was, and they are the two cases a media bin can most \
     easily lie about.",
    "`exposure.set_span` is in document 24's table and had no command behind it. \
     `SetExposureSpans` is added to the core by this unit, taking the whole ordered list \
     the way `SetMask` takes a whole outline, and the reason is written beside it.",
];

const MEDIA_NOTES: &[&str] = &[
    "## What to look at\n\n- **The gap is a fact, not an absence.** Layer 3 imports as \
     eleven drawings numbered 0 to 11 with no 7, and the record says so. An importer that \
     renumbered what it found would slide every later drawing one frame early and nothing \
     would look wrong until somebody counted.\n- **The complete sequence is not reported as \
     a broken one.** Layer 2's drawing 13 is called `layer2_桜_013.png`. It is imported \
     under its own name, the sequence is complete, and the mismatch is mentioned rather \
     than treated as a hole.\n- **Exposing a drawing that does not exist is allowed and \
     said.** Document 28 renders such a frame as nothing and substitutes no neighbour. A \
     person exposing a drawing that has not been scanned yet is told what it will look \
     like; they are not stopped.\n- **The rule about overlapping exposures is the core's.** \
     The two refusals quote document 20's own words through the same check the loader and \
     the renderer use. The window has no copy of that rule to fall out of step with.\n- \
     **Moving an exposure moves it.** The row carries the frame it starts at now as well as \
     the frame being typed, so an exposure dragged along the sheet does not leave a copy of \
     itself behind.",
    "## What this does not cover\n\nThe file dialog. Choosing files belongs to the \
     operating system and a test has no hands to answer one, so every row here begins at \
     the selection a person made. The button that opens it is in the media bin and is in \
     the photographs.\n\nStills. `media.import` builds an image sequence, because that is \
     what W-01 imports and what the reference shot is. A single still image is a record \
     this build's model already has and its own import is not built.\n\nRelinking, which is \
     W-02 and its own table.",
];

/// The request the media bin sends once the relink dialog has been answered.
fn relink_of(asset: &str, files: &[PathBuf]) -> String {
    let parts: Vec<String> = files
        .iter()
        .map(|p| format!("file={}", for_a_header(&p.display().to_string())))
        .collect();
    format!("media.relink?asset={asset}&{}", parts.join("&"))
}

/// Two drawings of a size nothing in the reference shot has.
fn tiny_drawings() -> Vec<PathBuf> {
    let directory = crate::test_temp().join("anime_compositor_b12a_relink");
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("make the scratch directory");
    (0..2)
        .map(|n| {
            let file = directory.join(format!("tiny_{n:03}.png"));
            anime_compositor::png_out::write_rgba(
                &file,
                8,
                8,
                OutputDepth::Eight,
                &[],
                &[0u8; 8 * 8 * 4],
            )
            .expect("write a tiny drawing");
            file
        })
        .collect()
}

/// One sentence out of a proposal, so that a row can show the part it is about.
fn sentence_about(said: &str, opening: &str) -> String {
    said.split(". ")
        .find(|s| s.starts_with(opening))
        .map(|s| format!("{}.", s.trim_end_matches('.')))
        .unwrap_or_else(|| format!("(nothing was said starting \"{opening}\")"))
}

/// What the frame carries about a relink waiting to be agreed to, which is all the panel
/// offering it ever knows. It rides on the frame rather than on an answer of its own so that
/// a panel cannot outlive the proposal it is showing.
fn on_the_frame(viewer: &Mutex<Viewer>) -> (String, String) {
    let export = Mutex::new(Export::default());
    let (ask, quality) = parse("/frame/0", None).expect("a readable request");
    let response = serve(viewer, &export, ask, quality);
    let text = |name: &str| {
        response.headers().get(name).map_or_else(String::new, |v| {
            from_a_query(v.to_str().unwrap_or_default())
        })
    };
    (text("x-relink"), text("x-relink-asset"))
}

#[test]
fn relinking_says_what_would_change_before_anything_changes() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let replacements = drawings_in("layer3");

    // ---- what cannot be relinked -------------------------------------------------------
    report.check(
        "relinking without naming a sequence asks which one",
        "Which sequence should be relinked? Choose one in the media bin.",
        run(&viewer, "media.relink"),
    );
    report.check(
        "naming a sequence but no drawings asks for the drawings themselves",
        "Which drawings should the sequence point at? Choose the files themselves, not the \
         folder they are in.",
        run(&viewer, "media.relink?asset=asset-cel"),
    );
    report.check(
        "a sequence that is not in the project is refused",
        "That media is not in this project. Nothing was changed.",
        run(&viewer, &relink_of("asset-gone", &replacements)),
    );
    report.check(
        "and agreeing to a relink nobody proposed is refused",
        "There is no relink waiting for that sequence. Choose the replacement drawings \
         first, so that what would change can be read before it happens.",
        run(&viewer, "media.relink?asset=asset-cel&apply=1"),
    );

    // ---- what would change, said before it does -----------------------------------------
    let said = run(&viewer, &relink_of("asset-cel", &replacements));
    report.check(
        "choosing the replacement drawings answers with what relinking would do",
        "Relinking \"Cel\" to layer3_%03d.png would give it 11 drawings, numbered 0 to 11, \
         and drawing 7 would be missing. The drawings are 1920x1080. What it points at now \
         cannot be read, so there is nothing to compare that with. The colour is read as \
         sRGB with straight alpha, which is what this project already recorded: a PNG does \
         not state either, so relinking does not change how the pixels are read. The layers \
         using it keep their stacking, transforms, exposures, masks and effects. Nothing \
         has changed yet. One drawing is missing from layer3_%03d.png: 7. Add the missing \
         files to the folder and relink the sequence, or leave the gap if the hole is \
         intended.",
        &said,
    );
    let (waiting, about) = on_the_frame(&viewer);
    report.check(
        "the panel offering it reads it off the frame, word for word",
        &said,
        &waiting,
    );
    report.check(
        "and the frame says which sequence it is about",
        "asset-cel",
        &about,
    );
    report.check(
        "the drawings the sequence points at have not moved",
        "asset-cel \"Cel\" 2 drawings",
        bin(&viewer),
    );
    report.check(
        "there is nothing to undo, because nothing was done",
        0,
        held(&viewer).document.undo_depth(),
    );

    // ---- and it can be walked away from -------------------------------------------------
    report.check(
        "leaving it as it is says so",
        "The relink was dropped. The sequence still points at the drawings it did.",
        run(&viewer, "media.relink?asset=asset-cel&cancel=1"),
    );
    report.check("the panel goes with it", "", on_the_frame(&viewer).0);
    report.check(
        "the sequence still points where it did",
        "asset-cel \"Cel\" 2 drawings",
        bin(&viewer),
    );
    report.check(
        "and dropping a second time has nothing to drop",
        "There is no relink waiting to be dropped.",
        run(&viewer, "media.relink?asset=asset-cel&cancel=1"),
    );

    // ---- agreeing to it ------------------------------------------------------------------
    run(&viewer, &relink_of("asset-cel", &replacements));
    report.check(
        "a proposal about one sequence cannot be applied to another",
        "There is no relink waiting for that sequence. Choose the replacement drawings \
         first, so that what would change can be read before it happens.",
        run(&viewer, "media.relink?asset=asset-1&apply=1"),
    );
    report.check(
        "and the proposal it was not about is still waiting",
        "asset-cel",
        on_the_frame(&viewer).1,
    );
    report.check(
        "agreeing names what was done, so that undo can be recognised",
        "Relink Cel",
        run(&viewer, "media.relink?asset=asset-cel&apply=1"),
    );
    report.check(
        "the sequence now points at the drawings that were chosen",
        "asset-cel \"Cel\" 11 drawings",
        bin(&viewer),
    );
    report.check(
        "the gap is still a gap: the drawing that is not there was not invented",
        "(no such drawing)",
        stored(&viewer, "asset-cel", 7),
    );
    report.check(
        "the layer that used it is the same layer, holding the same drawings for the same \
         frames",
        "0-2:1, 2-5:2",
        exposures(&viewer, "layer-cel"),
    );
    report.check(
        "the panel is gone, because there is nothing left to agree to",
        "",
        on_the_frame(&viewer).0,
    );
    report.check(
        "and it is one thing to undo",
        "Undone: Relink Cel",
        undo(&viewer),
    );
    report.check(
        "which puts the drawings back",
        "asset-cel \"Cel\" 2 drawings",
        bin(&viewer),
    );

    // ---- the comparison W-02 asks for, when there is something to compare -----------------
    run(&viewer, &import_of(&drawings_in("layer3")));
    let said = run(&viewer, &relink_of("asset-1", &tiny_drawings()));
    report.check(
        "relinking a sequence that can be read says what size it is now and what size it \
         would become",
        "The drawings are 8x8, where the ones it points at now are 1920x1080.",
        sentence_about(&said, "The drawings are"),
    );
    report.check(
        "and says the alpha is read the way this project already reads it, because a PNG \
         does not say",
        "The colour is read as sRGB with straight alpha, which is what this project \
         already recorded: a PNG does not state either, so relinking does not change how \
         the pixels are read.",
        sentence_about(&said, "The colour is read"),
    );

    write_artifact(
        &report,
        "verification/B-12a_relink_table.md",
        "B-12a: relinking, and what is said before anything changes",
        RELINK_INTRO,
        RELINK_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

const RELINK_INTRO: &[&str] = &[
    "W-02 is one sentence: relink \"by explicit user choice; preserve layer identity, \
     timing, masks and effects\", and \"present changed dimensions, frame range or alpha \
     interpretation before applying the change\". The presenting is the part that cannot be \
     done by a core command, because it happens before there is a command, so it is built \
     here and checked here. That relinking preserves layer identity once it is applied is \
     the core's own behaviour and is checked in \
     `verification/B-09_persistence_table.md`; this table checks the window's half.",
    "Relinking is three requests rather than one, and that is the requirement rather than a \
     convenience: choosing the drawings works out what would happen and says it, and a \
     second request either agrees to it or drops it. Between the two, nothing about the \
     project has moved - the rows below check the drawings, the history and the file rather \
     than trusting the sentence that says so.",
];

const RELINK_NOTES: &[&str] = &[
    "## What to look at\n\n- **The proposal is on the frame.** The sentence a person is \
     being asked to agree to travels back with the picture, like everything else this \
     window says, so a panel offering a relink cannot outlive the proposal it describes. \
     Cancel it, apply it, or open another project, and the panel goes with the next \
     frame.\n- **All three of W-02's changes are stated whether or not they changed.** The \
     size is given even when it is the same, the range and the gaps are given in full, and \
     the alpha reading is stated as carried over. \"Unchanged\" is an answer; silence is \
     not.\n- **What cannot be read is said to be unreadable.** Relinking is what a person \
     does when the media has gone, so the usual case is that there is nothing to compare \
     the new size against. That case says so rather than reporting the drawings as the \
     same size.\n- **A proposal belongs to one sequence.** Agreeing to it while another \
     sequence is selected is refused, and the proposal is still there afterwards.\n- **The \
     gap survives.** Relinking to a sequence missing drawing 7 leaves drawing 7 missing, \
     which document 28 renders as nothing.",
    "## What this does not cover\n\nThe file dialog, as everywhere else in this window: a \
     test has no hands to answer one, so every row begins at the selection a person \
     made.\n\nMasks. W-02 names them among what a relink preserves, and it does preserve \
     them - relinking replaces one asset record and cannot reach a layer - but this build \
     has no gesture for drawing a mask, so there is no mask here to photograph surviving \
     one.\n\nRelinking to a sequence in a different colour space. A PNG states neither the \
     colour space nor the alpha mode, so nothing read off the new drawings could \
     contradict the project; a format that states them is not in G1.",
];

/// Every composition in the project, in the order the file holds them.
fn comp_ids(viewer: &Mutex<Viewer>) -> String {
    held(viewer)
        .document
        .project()
        .compositions
        .iter()
        .map(|c| c.id.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

/// The composition the window is showing, written out the way a person would read it.
fn on_screen(viewer: &Mutex<Viewer>) -> String {
    let held = held(viewer);
    match held.document.project().composition(&held.composition) {
        None => format!("{} is not in the project", held.composition),
        Some(comp) => format!(
            "{} {} {}x{} at {} fps, {} frames",
            comp.id,
            comp.name,
            comp.width,
            comp.height,
            comp.frame_rate.label(),
            comp.duration_frames
        ),
    }
}

/// W-01's third step, which had no command in this build until B-12d.
///
/// Writes `verification/B-12d_new_composition_table.md`.
#[test]
fn a_composition_can_be_made_and_the_window_moves_into_it() {
    let mut report = Report { rows: Vec::new() };
    // `minimal_project.json` is the one fixture whose only composition is empty, which is
    // the state this command exists for: a project somebody can open with nothing in it to
    // work in.
    let source = repo("Fixtures/projects/minimal_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let before = held(&viewer).document.undo_depth();

    // ---- what somebody who fills in nothing gets -----------------------------------------
    let made = run(&viewer, "composition.create");
    report.check(
        "creating one with nothing filled in says what was made and that it is empty",
        "New composition New composition, 1920x1080 at 24 fps, 240 frames. It is empty; \
         import drawings and add layers to fill it.",
        &made,
    );
    report.check(
        "and the window is showing the composition that was just made",
        "comp-1 New composition 1920x1080 at 24 fps, 240 frames",
        on_screen(&viewer),
    );
    report.check(
        "and the one the file already held is still in the project",
        "comp-main, comp-1",
        comp_ids(&viewer),
    );
    report.check(
        "and the transport is the new composition's length, not the old one's",
        240,
        held(&viewer).playback.length(),
    );
    report.check(
        "and it is one history record, so one undo takes it back",
        before + 1,
        held(&viewer).document.undo_depth(),
    );

    // ---- the four refusals, each a sentence rather than a control springing back ----------
    let where_it_was = on_screen(&viewer);
    report.check(
        "a composition with no width is refused, and the refusal says which three matter",
        "A composition needs a width, a height and a length, and one of them was zero.",
        run(&viewer, "composition.create?width=0"),
    );
    report.check(
        "a composition larger than this build will make is refused with the limit in it",
        "20000x1080 is larger than this build will make: no side past 16384 and no more \
         than 67108864 pixels in all.",
        run(&viewer, "composition.create?width=20000"),
    );
    report.check(
        "a composition with no length is refused by the same sentence as one with no width",
        "A composition needs a width, a height and a length, and one of them was zero.",
        run(&viewer, "composition.create?frames=0"),
    );
    report.check(
        "a composition inside both side limits but past the pixel budget is still refused",
        "16384x16384 is larger than this build will make: no side past 16384 and no more \
         than 67108864 pixels in all. At 16384 wide the tallest this build will make is \
         4096.",
        run(&viewer, "composition.create?width=16384&height=16384"),
    );
    report.check(
        "a composition longer than this build will make is refused with the limit in it",
        "50000 frames is longer than this build will make; the limit is 10000.",
        run(&viewer, "composition.create?frames=50000"),
    );
    report.check(
        "a width that is not a number is refused before the core is asked",
        "\"wide\" is not a number of width. A composition needs whole numbers for its \
         width, its height, its frame rate and its length.",
        run(&viewer, "composition.create?width=wide"),
    );
    report.check(
        "a frame rate of zero is refused by the rule that owns frame rates",
        "A frame rate of zero is not a frame rate. Twenty-four is the one the reference \
         shot uses.",
        run(&viewer, "composition.create?fps=0"),
    );
    report.check(
        "and after seven refusals the project has exactly what it had before them",
        "comp-main, comp-1",
        comp_ids(&viewer),
    );
    report.check(
        "and the window is still looking at what it was looking at",
        where_it_was,
        on_screen(&viewer),
    );

    // ---- undo, which takes away the composition the window is showing --------------------
    report.check(
        "undo names the composition it took back rather than saying \"undone\"",
        "Undone: New composition New composition",
        undo(&viewer),
    );
    report.check(
        "and the window has moved off the composition that is no longer there",
        "comp-main Main 1920x1080 at 24 fps, 24 frames",
        on_screen(&viewer),
    );
    report.check(
        "and the project is back to the one composition the file held",
        "comp-main",
        comp_ids(&viewer),
    );
    report.check(
        "redo puts it back",
        "Redone: New composition New composition",
        redo(&viewer),
    );
    report.check(
        "and the window is showing it again",
        "comp-1 New composition 1920x1080 at 24 fps, 240 frames",
        on_screen(&viewer),
    );

    // ---- a second one, which must not be given the first one's identifier ----------------
    run(&viewer, "composition.create?name=Second&frames=48");
    report.check(
        "a second composition gets an identifier the first one is not using",
        "comp-main, comp-1, comp-2",
        comp_ids(&viewer),
    );
    report.check(
        "and a composition whose identifier is already in the project is refused",
        "A composition with the ID comp-1 is already in the project.",
        {
            let composition = Composition::new(
                Id::new("comp-1"),
                "A second comp-1",
                1920,
                1080,
                FrameRate::new(24, 1).expect("24 fps"),
                0,
                240,
            );
            edit(
                &viewer,
                Command::AddComposition {
                    composition: Box::new(composition),
                },
            )
        },
    );
    // D-290: the rate the After Effects tutorials ask for, kept as its exact fraction.
    report.check(
        "a composition at 23.976 fps is made, not refused",
        "New composition Film, 1920x1080 at 23.976 fps, 48 frames. It is empty; import \
         drawings and add layers to fill it.",
        run(&viewer, "composition.create?name=Film&frames=48&fps=23.976"),
    );
    report.check(
        "and its rate is the exact 24000/1001, not a rounded decimal",
        "24000/1001",
        held(&viewer)
            .document
            .project()
            .composition(&Id::new("comp-3"))
            .map(|comp| comp.frame_rate.to_string())
            .unwrap_or_default(),
    );
    undo(&viewer);
    run(&viewer, "composition.open?composition=comp-2");

    // ---- which composition a reopened project shows --------------------------------------
    // The defect the W-01 walkthrough found. Everything above happens in a project whose
    // first composition is the empty one the file came with, so before B-12d's rule of
    // thumb a save, close and reopen came back looking at that empty one and the work
    // looked lost.
    let folder = crate::test_temp().join("anime_compositor_b12d");
    let _ = std::fs::remove_dir_all(&folder);
    std::fs::create_dir_all(&folder).expect("make the scratch directory");
    let drawings = drawings_in("layer1");
    run(&viewer, &import_of(&drawings));
    run(&viewer, "layer.create?asset=asset-1&name=Background");
    report.check(
        "D-266: a new layer's anchor is the middle of its 1920 by 1080 drawing and it stands at the composition's centre",
        "anchor Vec2(960.0, 540.0), position Vec2(960.0, 540.0) in a 1920 by 1080 composition",
        {
            let held = held(&viewer);
            let comp = held.document.project().composition(&held.composition).expect("the composition on screen");
            let said = comp.layers_in_order().find(|l| l.name == "Background").map(|l| format!(
                "anchor {:?}, position {:?} in a {} by {} composition",
                l.transform.anchor.base(), l.transform.position.base(), comp.width, comp.height
            )).unwrap_or_default();
            said
        },
    );
    let file = folder.join("reopened.json");
    save_as(&viewer, &file);
    // A row rather than a panic. A project this test wrote that will not open again is a
    // line in the table the owner reads, not a stack trace: the hardening pass broke the
    // rule that a composition has a positive length and the schema caught it here, on the
    // way back through the disk, which ended the run before the table was written at all.
    let (showing, still_there) = match open(&file) {
        Ok(viewer) => {
            let reopened = Mutex::new(viewer);
            (on_screen(&reopened), comp_ids(&reopened))
        }
        Err(d) => (d.message.clone(), d.message),
    };
    report.check(
        "a saved project reopens looking at the composition with work in it",
        "comp-2 Second 1920x1080 at 24 fps, 48 frames",
        showing,
    );
    report.check(
        "and the empty compositions are still there, because none of them was thrown away",
        "comp-main, comp-1, comp-2",
        still_there,
    );

    // ---- the largest one this build will make, which must be allowed ---------------------
    // The limit rows above all check that something too large is refused. Without this one
    // the same rows pass a build that refuses everything, and the row a person would hit is
    // the one where a legal size is turned down for no reason they can see.
    report.check(
        "a composition exactly at the pixel budget is made rather than refused",
        "New composition Biggest, 16384x4096 at 24 fps, 240 frames. It is empty; import \
         drawings and add layers to fill it.",
        run(
            &viewer,
            "composition.create?name=Biggest&width=16384&height=4096",
        ),
    );

    // ---- getting back out of one ---------------------------------------------------------
    // The owner found this the first time they made a composition: making one moves the
    // window into it, and until `composition.open` existed there was no way back to the shot
    // they had been working on. Nothing in the window even named the other compositions.
    let depth = held(&viewer).document.undo_depth();
    report.check(
        "the window can be put back on a composition it left",
        "comp-main Main 1920x1080 at 24 fps, 24 frames",
        {
            run(&viewer, "composition.open?composition=comp-main");
            on_screen(&viewer)
        },
    );
    report.check(
        "and the transport is that composition's length, not the one it came from",
        24,
        held(&viewer).playback.length(),
    );
    report.check(
        "and looking somewhere else is not an edit, so there is nothing new to undo",
        depth,
        held(&viewer).document.undo_depth(),
    );
    report.check(
        "a composition that is not in the project is refused",
        "There is no composition comp-9 in this project.",
        run(&viewer, "composition.open?composition=comp-9"),
    );
    report.check(
        "and the window is still looking at the one it was",
        "comp-main Main 1920x1080 at 24 fps, 24 frames",
        on_screen(&viewer),
    );
    report.check(
        "asking with no composition named says what to do rather than moving the window",
        "Which composition should be opened? Choose one in the project panel.",
        run(&viewer, "composition.open"),
    );

    write_artifact(
        &report,
        "verification/B-12d_new_composition_table.md",
        "B-12d: making a composition",
        NEW_COMPOSITION_INTRO,
        NEW_COMPOSITION_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

const NEW_COMPOSITION_INTRO: &[&str] = &[
    "W-01 lists \"creates a composition\" third of its thirteen steps, and until B-12d this \
     build could not take that step. There was no `composition.create` in document 24 and \
     no command in the core behind it: the composition somebody worked in was whichever one \
     their project file already held, and `verification/B-12_acceptance_run.md` recorded \
     step 3 as **not built**. This table is that step, made to work and then checked.",
    "It is a new capability rather than a correction of an omission, which is written up \
     beside the new row in document 24. The owner may cut it; nothing else in the build \
     depends on it existing.",
];

const NEW_COMPOSITION_NOTES: &[&str] = &[
    "## What to look at\n\n- **The size limits are this build's, not the \
     specification's.** Document 19 line 52 says a composition is \"bounded by \
     implementation safety limits\" and never says what they are. This build chose no side \
     past 16384, no more than 67108864 pixels in all, and no more than 10000 frames, and \
     the rows that check them are the only place those numbers are visible. They are \
     provisional and are the owner's to change.\n- **The two size limits are not one \
     limit, and the refusal says so.** 16384 a side and 67108864 pixels in all is 8192 by \
     8192, or 16384 by 4096, and not 16384 square, which is four times the pixel ceiling. \
     The owner read the first half of that sentence and asked for 16384 square on \
     2026-09-08, so the refusal now finishes the arithmetic and names the tallest height \
     the requested width allows, and the Biggest row above makes exactly that shape. \
     The ceiling is memory: document 21 works in float32 RGBA, sixteen bytes a pixel, so \
     67108864 pixels is a gibibyte for one layer buffer and several for a frame.\n- **A refusal is a sentence and changes \
     nothing.** Seven commands here are turned down, and the two rows after them check that \
     the project holds exactly what it held before and that the window is still looking at \
     the same composition. A control that springs back with nothing said is the failure \
     this guards against.\n- **Undo takes away the composition on screen.** That is the one \
     genuinely new way this command can break a window, because no other command could \
     remove what the viewer was pointed at. Undo and redo both put the window on the \
     composition the record touched, and fall back to the first composition when that one \
     has been taken away. Before that rule a redo said \"Redone\" and left the person \
     looking at a different composition, which is what the row for it caught.",
    "## What this does not cover\n\n**Which composition a reopened project shows is a rule \
     of thumb, not a restored setting.** Document 07's project format has no field for \
     which composition was open, so there is nothing on disk to come back to. This build \
     opens the first composition that has anything in it, and the first composition \
     otherwise. The last two rows check that rule, and they pass, but the rule is wrong for \
     anyone who deliberately leaves an empty composition ready to work in and expects to \
     find it. Fixing it properly means adding a field to the project format, which is a \
     schema change and the owner's decision.\n\n**`project.new` came later.** W-24 built it on Ctrl+N, the shortcut document 24 \
     promised it then, and Ctrl+Shift+N was the new composition. W-39 (D-248) moved them to \
     After Effects' keys: Ctrl+N is the new composition and Ctrl+Alt+N the new project.\n\n**The page.** Every \
     row calls the same function the window's URL scheme calls. That the New composition \
     button and its five fields send it, and that Ctrl+N reaches the button, are in \
     `verification/B-12b_page_table.md`, `verification/B-12c_keyboard_table.md` and the \
     photograph beside this table.",
];

fn cell(text: &str) -> String {
    text.replace('|', r"\|")
}

/// B-14c: what `/boxes` says one property is on a frame, and what it says is wrong with its
/// expression there, which are the two things the panel shows beside the property.
fn shown_at(viewer: &Mutex<Viewer>, frame: i32, row: &str, prop: &str) -> (String, String) {
    let answer: serde_json::Value =
        serde_json::from_slice(boxes(viewer, frame, None).body()).expect("boxes is JSON");
    let error = match &answer["errors"][row][prop] {
        serde_json::Value::String(e) => e.clone(),
        _ => "none".to_string(),
    };
    (answer["values"][row][prop].to_string(), error)
}

fn both((a, b): (String, String)) -> String {
    format!("{a}, {b}")
}

/// B-14c: a layer's or the camera's expression, as the state answer gives it to the page.
fn expression_of(viewer: &Mutex<Viewer>, row: &str, prop: &str) -> String {
    let answer: serde_json::Value =
        serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
    let comp = &answer["project"]["compositions"][0];
    let property = match row {
        "camera" => &comp["camera"][prop],
        id => {
            let layer = comp["layers"]
                .as_array()
                .and_then(|all| all.iter().find(|l| l["id"] == id))
                .expect("the layer is in the composition");
            &layer["transform"][prop]
        }
    };
    property["expression"].to_string()
}

/// B-14c: the expression box on each property, its switch, and the error beside it, driven
/// by the requests the page sends. The order is the playtest sheet's.
#[test]
fn an_expression_is_typed_switched_and_removed_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/unknown_effect_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    report.check(
        "Alt-click on the rotation diamond adds `value`, which changes nothing yet",
        r#"{"enabled":true,"text":"value"}"#,
        {
            run(
                &viewer,
                "property.set_expression?layer=layer-cel&prop=rotation&text=value&on=true",
            );
            expression_of(&viewer, "layer-cel", "rotation")
        },
    );
    report.check(
        "and the panel still reads the rotation it read before",
        "0, none",
        both(shown_at(&viewer, 24, "layer-cel", "rotation")),
    );
    report.check(
        "typing `time * 90` into the box makes frame 24 (one second) read 90",
        "90, none",
        {
            run(
                &viewer,
                "property.set_expression?layer=layer-cel&prop=rotation&text=time%20*%2090&on=true",
            );
            both(shown_at(&viewer, 24, "layer-cel", "rotation"))
        },
    );
    report.check(
        "the switch turns it off, keeping the text, and the rotation is its own again",
        r#"{"enabled":false,"text":"time * 90"}, 0"#,
        {
            run(
                &viewer,
                "property.set_expression?layer=layer-cel&prop=rotation&text=time%20*%2090&on=false",
            );
            both((
                expression_of(&viewer, "layer-cel", "rotation"),
                shown_at(&viewer, 24, "layer-cel", "rotation").0,
            ))
        },
    );
    report.check("undo switches it back on", "90", {
        undo(&viewer);
        shown_at(&viewer, 24, "layer-cel", "rotation").0
    });
    report.check(
        "half-typed text is kept as typed, not refused",
        r#"{"enabled":true,"text":"time *"}"#,
        {
            run(
                &viewer,
                "property.set_expression?layer=layer-cel&prop=rotation&text=time%20*&on=true",
            );
            expression_of(&viewer, "layer-cel", "rotation")
        },
    );
    let (value, error) = shown_at(&viewer, 24, "layer-cel", "rotation");
    report.check(
        "and the error beside it names what is wrong, while the rotation falls back to its own",
        "0, EXPRESSION_SYNTAX",
        format!("{value}, {}", error.split(':').next().unwrap_or("")),
    );
    report.check(
        "a position expression gives a pair of numbers, the second one kept from the value",
        "[120,0]",
        {
            run(&viewer, "property.set_expression?layer=layer-cel&prop=position&text=%5B120%2C%20value%5B1%5D%5D&on=true");
            shown_at(&viewer, 0, "layer-cel", "position").0
        },
    );
    report.check("Alt-click again removes the expression", "null", {
        run(
            &viewer,
            "property.set_expression?layer=layer-cel&prop=position",
        );
        expression_of(&viewer, "layer-cel", "position")
    });
    report.check(
        "and undo brings it back",
        r#"{"enabled":true,"text":"[120, value[1]]"}"#,
        {
            undo(&viewer);
            expression_of(&viewer, "layer-cel", "position")
        },
    );

    // ---- the camera ---------------------------------------------------------------------
    report.check(
        "the camera's lens takes an expression the same way",
        "1000, none",
        {
            run(
                &viewer,
                "property.set_expression?target=camera&prop=zoom&text=1000&on=true",
            );
            both(shown_at(&viewer, 0, CAMERA_ROW, "zoom"))
        },
    );
    report.check(
        "and the camera's depth refuses a pair of numbers with the reason beside it",
        "EXPRESSION_TYPE",
        {
            run(
                &viewer,
                "property.set_expression?target=camera&prop=depth&text=%5B1%2C2%5D&on=true",
            );
            shown_at(&viewer, 0, CAMERA_ROW, "depth")
                .1
                .split(':')
                .next()
                .unwrap_or("")
                .to_string()
        },
    );

    // ---- what is refused ----------------------------------------------------------------
    run(&viewer, "layer.toggle_lock?layer=layer-cel");
    let before = held(&viewer).document.undo_depth();
    let locked = run(
        &viewer,
        "property.set_expression?layer=layer-cel&prop=scale&text=50&on=true",
    );
    report.check(
        "a locked layer refuses an expression",
        "null",
        expression_of(&viewer, "layer-cel", "scale"),
    );
    report.check(
        "and the window says why, in words",
        "The layer \"Cel\" is locked, so it was not changed. Unlock the layer to edit it.",
        locked,
    );
    report.check(
        "a property that cannot carry one is refused by name",
        "true",
        run(
            &viewer,
            "property.set_expression?layer=layer-cel&prop=wobble&text=1",
        )
        .contains("wobble")
        .to_string(),
    );
    report.check(
        "and the refusals put nothing in the history",
        before,
        held(&viewer).document.undo_depth(),
    );
    run(&viewer, "layer.toggle_lock?layer=layer-cel");

    // ---- the file ----------------------------------------------------------------------
    let folder = crate::test_temp().join("anime_compositor_b14c");
    let _ = std::fs::remove_dir_all(&folder);
    std::fs::create_dir_all(&folder).expect("make the scratch directory");
    let file = folder.join("expressions.json");
    save_as(&viewer, &file);
    let saved = std::fs::read_to_string(&file).unwrap_or_default();
    report.check(
        "Save writes the text and the switch into the file",
        "true",
        (saved.contains("\"text\": \"time *\"") && saved.contains("\"text\": \"1000\""))
            .to_string(),
    );
    report.check(
        "and opening that file shows the same broken expression and its error",
        "EXPRESSION_SYNTAX",
        shown_at(&viewer, 24, "layer-cel", "rotation")
            .1
            .split(':')
            .next()
            .unwrap_or("")
            .to_string(),
    );

    write_artifact(
        &report,
        "verification/B-14c_panel_table.md",
        "B-14c: expressions in the window",
        EXPRESSION_PANEL_INTRO,
        EXPRESSION_PANEL_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

const EXPRESSION_PANEL_INTRO: &[&str] = &[
    "D-59 gave a property an expression and B-14b taught the core to work one out. This is \
     the window's half: Alt-click on a property's diamond (or Alt+Shift+= on it) adds a box \
     under the property with `value` in it, which changes nothing until something else is \
     typed; the `=` beside the property's name switches it off and on; and a text the \
     language cannot read is kept, with the reason in red under it.",
    "Every row sends the request the page sends, to the same handler, and reads back what \
     the page is given: the project for the text and the switch, and the frame's `/boxes` \
     answer for the number the panel shows and the error beside it.",
];

const EXPRESSION_PANEL_NOTES: &[&str] = &[
    "## What this does not cover\n\nWhether the box is comfortable to type into, whether \
     the red line is readable, and whether Alt-click feels like After Effects. That is \
     `verification/B-14c_expressions_playtest.md`, for a person.\n\nWhat an expression \
     works out to in detail. That is `verification/B-14b_expression_table.md`, 184 of 184 \
     against independently written numbers; this table only checks that the window shows \
     the same answer.",
    "## What changed for Alt-click\n\nW-23 had Alt-click on the diamond take every key off \
     the property. The owner asked on 2026-09-16 for After Effects' behaviour instead, so it \
     now adds or removes the expression. Taking every key off is still two steps: press the \
     property's name on the timeline, which chooses all its keys, then press Delete.",
];

/// Whether the asset `id` may be passed on, as the state answer gives it to the page.
fn passed_on(viewer: &Mutex<Viewer>, id: &str) -> String {
    let state: serde_json::Value =
        serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
    let asset = state["project"]["assets"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|a| a["id"] == id)
        .cloned()
        .unwrap_or_default();
    match asset["redistribute"].as_bool() {
        None => "ticked".to_string(),
        Some(true) => "ticked, written".to_string(),
        Some(false) => "unticked".to_string(),
    }
}

/// B-15c: the "Can be passed on" tick, File > Collect Files and File > Check Package, on
/// D-61's fixture shot. The order is the playtest sheet's.
#[test]
fn a_shot_is_collected_and_checked_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let scratch = repo("target/b15c-scratch");
    let _ = std::fs::remove_dir_all(&scratch);
    std::fs::create_dir_all(&scratch).expect("make the scratch folder");
    let source = repo("Fixtures/packaging/source/shot.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );

    // ---- the tick ----------------------------------------------------------------------
    report.check(
        "the licensed sheet opens unticked and the cels ticked",
        "unticked, ticked",
        format!(
            "{}, {}",
            passed_on(&viewer, "asset-licensed"),
            passed_on(&viewer, "asset-cel")
        ),
    );
    report.check(
        "unticking the cels says so in the history's words",
        "Keep asset-cel out of packages",
        run(
            &viewer,
            "asset.set_redistribute?asset=asset-cel&redistribute=false",
        ),
    );
    report.check(
        "and the cels are unticked, with unsaved changes",
        "unticked, true",
        format!(
            "{}, {}",
            passed_on(&viewer, "asset-cel"),
            held(&viewer).document.is_dirty()
        ),
    );
    report.check("undo ticks them again", "ticked", {
        undo(&viewer);
        passed_on(&viewer, "asset-cel")
    });
    report.check(
        "ticking the licensed sheet is one history item",
        "Let asset-licensed be passed on; ticked",
        {
            let said = run(
                &viewer,
                "asset.set_redistribute?asset=asset-licensed&redistribute=true",
            );
            format!("{said}; {}", passed_on(&viewer, "asset-licensed"))
        },
    );
    report.check("and undo unticks it", "unticked", {
        undo(&viewer);
        passed_on(&viewer, "asset-licensed")
    });
    report.check(
        "a tick on a drawing that is not there is refused and changes nothing",
        "refused, 0 undo items",
        {
            let said = run(
                &viewer,
                "asset.set_redistribute?asset=asset-nobody&redistribute=false",
            );
            format!(
                "{}, {} undo items",
                if said.contains("not in this project") {
                    "refused"
                } else {
                    "accepted"
                },
                held(&viewer).document.undo_labels().len()
            )
        },
    );

    // ---- Collect Files -----------------------------------------------------------------
    let package = scratch.join("package");
    let before = state(&viewer);
    report.check(
        "Collect Files says how many drawings were copied, missing and left out",
        "Collected into (the chosen folder): 10 drawings copied. 1 drawing was missing, \
         listed without a copy. 1 drawing was left out, marked not to be passed on. The open \
         project is unchanged.",
        collect_into(&viewer, &package)
            .replace(&package.display().to_string(), "(the chosen folder)"),
    );
    report.check(
        "the manifest it wrote is FX-PACK-001's, to the byte",
        true,
        std::fs::read(package.join(package::MANIFEST)).ok()
            == std::fs::read(repo("Fixtures/packaging/expected_manifest.json")).ok(),
    );
    report.check(
        "and the window, its history and its unsaved state are as they were",
        true,
        state(&viewer) == before,
    );
    report.check(
        "collecting into the same folder again is refused",
        "Files were not collected: the chosen folder is not empty. Choose an empty folder, \
         or a new one.",
        collect_into(&viewer, &package),
    );
    report.check(
        "with the licensed sheet ticked, it is copied too",
        "11 drawings copied, nothing left out",
        {
            run(
                &viewer,
                "asset.set_redistribute?asset=asset-licensed&redistribute=true",
            );
            let said = collect_into(&viewer, &scratch.join("ticked"));
            format!(
                "{}, {}",
                if said.contains(": 11 drawings copied.") {
                    "11 drawings copied"
                } else {
                    &said
                },
                if said.contains("left out") {
                    "something left out"
                } else {
                    "nothing left out"
                }
            )
        },
    );

    // ---- Check Package -----------------------------------------------------------------
    report.check(
        "Check Package on a project that is not a package names the manifest",
        "PACKAGE_MANIFEST_INVALID",
        {
            check_package(&viewer);
            held(&viewer)
                .notes
                .join("\n")
                .split('\t')
                .nth(1)
                .unwrap_or("")
                .split(' ')
                .next()
                .unwrap_or("")
                .to_string()
        },
    );
    let viewer = Mutex::new(open(&package.join("shot.json")).expect("open the package"));
    report.check(
        "on the package, it counts each answer",
        "Checked 12 files against the package manifest: 10 ok, 1 missing, 1 excluded.",
        check_package(&viewer),
    );
    report.check(
        "and lists the two that are not ok, by path, in the notes",
        "media/asset-licensed/sheet.png, media/asset-gone/gone_0001.png",
        held(&viewer)
            .notes
            .iter()
            .map(|n| n.split(':').next().unwrap_or(""))
            .collect::<Vec<_>>()
            .join(", "),
    );
    let cel = package.join("media/asset-cel/cel_0001.png");
    let mut bytes = std::fs::read(&cel).expect("read the packaged cel");
    let last = bytes.len() - 1;
    bytes[last] ^= 1;
    std::fs::write(&cel, bytes).expect("change one byte");
    report.check(
        "a cel changed by one byte is reported changed",
        "Checked 12 files against the package manifest: 9 ok, 1 changed, 1 missing, 1 \
         excluded.",
        check_package(&viewer),
    );
    report.check(
        "with its code in the note",
        true,
        held(&viewer).notes.iter().any(|n| {
            n.starts_with("media/asset-cel/cel_0001.png: ") && n.contains("PACKAGE_FILE_CHANGED")
        }),
    );
    // Save As moves the file but leaves the media root in the old folder; the manifest is
    // still looked for beside the file.
    held(&viewer).root = source.parent().expect("the fixture folder").to_path_buf();
    report.check(
        "after Save As, the package beside the saved file is the one checked",
        "Checked 12 files against the package manifest: 9 ok, 1 changed, 1 missing, 1 \
         excluded.",
        check_package(&viewer),
    );
    held(&viewer).path = None;
    report.check(
        "a project with no file has nothing beside it to check",
        "This project has no file yet, so there is no package beside it to check. Open the \
         project inside a collected folder first.",
        check_package(&viewer),
    );

    write_artifact(
        &report,
        "verification/B-15c_panel_table.md",
        "B-15c: collecting and checking from the window",
        PACKAGE_PANEL_INTRO,
        PACKAGE_PANEL_NOTES,
    );
    // What each failed check got, not only its name: this one has failed about once in twenty
    // full runs and passed every time alone, and the table is overwritten by the next run.
    let failed: Vec<(&String, &String)> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, a)| (c, a))
        .collect();
    assert!(
        failed.is_empty(),
        "these checks failed, with what they got: {failed:#?}"
    );
}

const PACKAGE_PANEL_INTRO: &[&str] = &[
    "D-61 decided what a package is and B-15b built it, checked against the fixtures in \
     `verification/B-15b_package_table.md`. This is the window's half: a \"Can be passed \
     on\" tick on each drawing in the Project panel, and Collect Files and Check Package \
     beside Save As.",
    "Every row calls what the window calls, on `Fixtures/packaging/source/shot.json`, and \
     reads back what the page is given. The folder picker Collect Files opens is the one \
     step skipped: no test can answer a Windows dialog, so the rows start with the folder \
     already chosen.",
];

const PACKAGE_PANEL_NOTES: &[&str] = &[
    "## What this does not cover\n\nWhether the tick and the two buttons are where you \
     expect them, and whether the package opens on another computer. That is \
     `verification/B-15c_package_playtest.md`, for a person.",
];

/// The newest asset in the project, as `id "name" N drawings, read as ...`.
fn newest(viewer: &Mutex<Viewer>) -> String {
    let held = held(viewer);
    let asset = held
        .document
        .project()
        .assets
        .last()
        .expect("the project has an asset");
    format!(
        "{} drawings, read as {}",
        asset.frames.len().max(asset.path.iter().count()),
        read_as(&asset.interpretation)
    )
}

fn read_as(i: &Interpretation) -> String {
    format!("{:?}, {:?}", i.color_space, i.alpha)
}

fn asset_count(viewer: &Mutex<Viewer>) -> usize {
    held(viewer).document.project().assets.len()
}

/// B-16c: EXR drawings imported and relinked from the window, and EXR frames exported from
/// it, on D-62's fixture files. The order is the playtest sheet's.
#[test]
fn exr_drawings_come_in_and_exr_frames_go_out_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let exr = |rel: &str| repo(&format!("Fixtures/exr/{rel}"));
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );

    // ---- import -------------------------------------------------------------------------
    let said = run(&viewer, &import_of(&[exr("compression/zip_half.exr")]));
    report.check(
        "one EXR file imports as a still picture",
        "Import zip_half.exr: a still picture",
        said.split(", shown").next().unwrap_or_default(),
    );
    report.check(
        "read as linear light with premultiplied alpha, as D-62 says",
        "1 drawings, read as LinearLight, Premultiplied",
        newest(&viewer),
    );
    report.check(
        "a file drawn exactly as stored leaves no note",
        false,
        notes_mention(&viewer, "MEDIA_EXR_ADJUSTED"),
    );
    let before = asset_count(&viewer);
    report.check(
        "a file drawn with changes says which, in the notes",
        "MEDIA_EXR_ADJUSTED non_finite: 4 alpha_clamped: 2",
        {
            run(&viewer, &import_of(&[exr("values/specials.exr")]));
            held(&viewer)
                .notes
                .iter()
                .filter_map(|n| n.split_once('\t').map(|(_, code)| code.to_string()))
                .find(|code| code.starts_with("MEDIA_EXR_ADJUSTED"))
                .unwrap_or_else(|| "(no such note)".to_string())
        },
    );
    report.check(
        "and the file is still added",
        before + 1,
        asset_count(&viewer),
    );
    let before = asset_count(&viewer);
    report.check(
        "a file this build refuses is not added, and the reason is given",
        "Nothing was imported. multipart.exr is an EXR file this build does not draw. \
         Re-export it as a single-part RGBA EXR. The file is left untouched and the asset \
         record is kept.; 0 added",
        {
            let said = run(&viewer, &import_of(&[exr("refused/multipart.exr")]));
            format!("{said}; {} added", asset_count(&viewer) - before)
        },
    );
    let sequence: Vec<PathBuf> = ["0001", "0002", "0003", "0005"]
        .iter()
        .map(|n| exr(&format!("sequence/render_{n}.exr")))
        .collect();
    report.check(
        "an EXR sequence imports with its gap, as a PNG one does",
        "Import render_%04d.exr: 4 drawings, numbered 1 to 5, and drawing 4 is missing.",
        run(&viewer, &import_of(&sequence)),
    );
    report.check(
        "and is read as linear light with premultiplied alpha",
        "4 drawings, read as LinearLight, Premultiplied",
        newest(&viewer),
    );
    let exr_sequence = held(&viewer)
        .document
        .project()
        .assets
        .last()
        .map(|a| a.id.to_string())
        .expect("the sequence just imported");

    // ---- relink -------------------------------------------------------------------------
    let said = run(&viewer, &relink_of("asset-cel", &sequence));
    report.check(
        "relinking PNG drawings to EXR ones says the reading changes, before it happens",
        "The colour will be read as linear light with premultiplied alpha, where it is read \
         as sRGB with straight alpha now: an EXR file is always linear light with \
         premultiplied alpha, and a PNG is not.",
        {
            let phrase = "The colour will be read as linear light with premultiplied alpha, \
                          where it is read as sRGB with straight alpha now: an EXR file is \
                          always linear light with premultiplied alpha, and a PNG is not.";
            if said.contains(phrase) {
                phrase.to_string()
            } else {
                said
            }
        },
    );
    let cel = |viewer: &Mutex<Viewer>| {
        let held = held(viewer);
        let asset = held
            .document
            .project()
            .assets
            .iter()
            .find(|a| a.id == Id::new("asset-cel"))
            .cloned()
            .expect("the cel asset");
        format!(
            "{}; {}",
            asset.frames.values().next().cloned().unwrap_or_default(),
            read_as(&asset.interpretation)
        )
    };
    report.check(
        "nothing has changed yet",
        "cel_0001.png; Srgb, Straight",
        cel(&viewer).rsplit(['/', '\\']).next().unwrap_or_default(),
    );
    run(&viewer, "media.relink?asset=asset-cel&apply=1");
    report.check(
        "agreeing points the cels at the EXR files and reads them as EXR",
        "render_0001.exr; LinearLight, Premultiplied",
        cel(&viewer).rsplit(['/', '\\']).next().unwrap_or_default(),
    );
    undo(&viewer);
    report.check(
        "undo puts both back",
        "cel_0001.png; Srgb, Straight",
        cel(&viewer).rsplit(['/', '\\']).next().unwrap_or_default(),
    );
    report.check(
        "relinking EXR drawings to EXR ones says the reading stays",
        true,
        run(&viewer, &relink_of(&exr_sequence, &sequence)).contains(
            "The colour is read as linear light with premultiplied alpha, as it is now: an \
             EXR file is always read that way.",
        ),
    );
    run(
        &viewer,
        &format!("media.relink?asset={exr_sequence}&cancel=1"),
    );

    // ---- export -------------------------------------------------------------------------
    let formats = ["png", "exr-half", "exr-float", "psd"]
        .map(
            |f| match output_format(Some(&format!("format=x&format={f}")[9..])) {
                Ok(format) => format!("{format:?}"),
                Err(refused) => refused,
            },
        )
        .join("; ");
    report.check(
        "the format list's three choices are the three files, and anything else is refused",
        "Png; Exr(Half); Exr(Float); Nothing was exported: \"psd\" is not a format this \
         window writes. Choose PNG, EXR half, EXR float, GIF, animated PNG or MP4.",
        formats,
    );
    report.check(
        "no choice at all is PNG, as before the list",
        "Png",
        format!("{:?}", output_format(None).expect("no choice is a format")),
    );
    let into = crate::test_temp().join("anime_compositor_b16c_export");
    let _ = std::fs::remove_dir_all(&into);
    std::fs::create_dir_all(&into).expect("the export folder");
    let (snapshot, root, mut request) = export_job(
        &held(&viewer),
        &into,
        MissingSource::RenderTransparent,
        OutputFormat::Exr(ExrSamples::Half),
    );
    report.check(
        "an EXR export names its files .exr",
        "cel_holds_project_%04d.exr",
        &request.naming,
    );
    request.last_frame = request.first_frame + 1;
    let done = export::export_sequence(&snapshot, &root, &request, &AtomicBool::new(false));
    let files: Vec<String> = done
        .written
        .iter()
        .map(|f| {
            f.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    report.check(
        "and writes them",
        "cel_holds_project_0000.exr, cel_holds_project_0001.exr",
        files.join(", "),
    );
    report.check(
        "each reads back as a picture the composition's size, with nothing adjusted",
        {
            let comp = snapshot
                .composition(&request.composition)
                .expect("the exported composition");
            format!("{}x{}, 0 reasons", comp.width, comp.height)
        },
        match done.written.first().map(|f| exr_io::read(f)) {
            Some(Ok(picture)) => format!(
                "{}x{}, {} reasons",
                picture.image.width(),
                picture.image.height(),
                picture.adjusted.len()
            ),
            Some(Err(refused)) => refused.sentence(),
            None => "(nothing written)".to_string(),
        },
    );

    write_artifact(
        &report,
        "verification/B-16c_panel_table.md",
        "B-16c: EXR in the window",
        EXR_PANEL_INTRO,
        EXR_PANEL_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// B-21b: D-72's drawing formats imported from the window, on its fixture files.
#[test]
fn the_new_drawing_formats_come_in_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    for name in [
        "bmp24.bmp",
        "tga32.tga",
        "tga32_rle.tga",
        "tiff_lzw.tif",
        "webp_lossless.webp",
        "jpeg_q95.jpg",
    ] {
        let said = run(
            &viewer,
            &import_of(&[repo(&format!("Fixtures/formats/media/{name}"))]),
        );
        report.check(
            &format!("{name} imports as a still picture"),
            format!("Import {name}: a still picture"),
            said.split(", shown").next().unwrap_or_default(),
        );
        report.check(
            "and is read as a PNG is",
            "1 drawings, read as Srgb, Straight",
            newest(&viewer),
        );
    }
    let before = asset_count(&viewer);
    let said = run(
        &viewer,
        &import_of(&[repo("Fixtures/formats/expected_formats.json")]),
    );
    report.check(
        "a file that is not a drawing is still not imported",
        format!("true, {before} assets"),
        format!(
            "{}, {} assets",
            said.contains("othing was imported"),
            asset_count(&viewer)
        ),
    );

    write_artifact(
        &report,
        "verification/B-21b_panel_table.md",
        "B-21b: the new drawing formats in the window",
        &["D-72 added BMP, TGA, TIFF, WebP and JPEG as drawings, and                `verification/B-21b_formats_table.md` checks each picture against its PNG twin.                This is the window's half: Import and Relink list the new extensions, and a chosen                file comes in as a drawing. The file picker itself is skipped, as in every panel                table: no test can answer a Windows dialog."],
        &[],
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// B-21c: a GIF and an animated PNG from the window (D-72).
#[test]
fn a_gif_and_an_animated_png_go_out_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let page =
        std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui/index.html"))
            .expect("the page");
    report.check(
        "the list beside Export offers a GIF, and says it is a preview with no sound (D-641)",
        true,
        page.contains("<option value=\"gif\">GIF, a preview: 256 colours, no sound</option>"),
    );
    report.check(
        "and an animated PNG",
        true,
        page.contains("<option value=\"apng\">Animated PNG, 8-bit, lossless</option>"),
    );
    let into = crate::test_temp().join("anime_compositor_b21c_export");
    let _ = std::fs::remove_dir_all(&into);
    std::fs::create_dir_all(&into).expect("the export folder");
    for (choice, file) in [
        ("gif", "cel_holds_project.gif"),
        ("apng", "cel_holds_project_animated.png"),
    ] {
        let format = output_format(Some(&format!("format={choice}"))).expect("a listed format");
        // The fixture's drawings are not on the disk, so the frames are written without
        // them, as the window's own tick box asks; what is checked here is the file.
        let (snapshot, root, mut request) = export_job(
            &held(&viewer),
            &into,
            MissingSource::RenderTransparent,
            format,
        );
        report.check(
            &format!("{choice}: the file is named"),
            file,
            &request.naming,
        );
        request.last_frame = request.first_frame + 2;
        let said = run_export(&snapshot, &root, &request, &AtomicBool::new(false));
        report.check(
            &format!("{choice}: three frames are one file, and the window says so"),
            format!("Exported 3 frames as one file, the chosen folder's {file}."),
            said.replace(
                &format!("{}{}", into.display(), std::path::MAIN_SEPARATOR),
                "the chosen folder's ",
            )
            .split_inclusive('.')
            .take(2)
            .collect::<String>(),
        );
    }
    let mut files: Vec<String> = std::fs::read_dir(&into)
        .expect("the export folder")
        .map(|f| {
            f.expect("a file")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    files.sort();
    report.check(
        "the folder holds those two files and nothing else",
        "cel_holds_project.gif, cel_holds_project_animated.png",
        files.join(", "),
    );
    let (snapshot, root, request) = export_job(
        &held(&viewer),
        &into,
        MissingSource::RenderTransparent,
        OutputFormat::Gif,
    );
    let _ = std::fs::remove_file(into.join("cel_holds_project.gif"));
    let said = run_export(&snapshot, &root, &request, &AtomicBool::new(true));
    report.check(
        "a cancelled GIF leaves no half-made file, and the window says so",
        "Nothing was exported. Export stopped at your request. An animated file is whole or \
         it is nothing, so no file was left.; file there: false",
        format!(
            "{}; file there: {}",
            said,
            into.join("cel_holds_project.gif").exists()
        ),
    );

    write_artifact(
        &report,
        "verification/B-21c_panel_table.md",
        "B-21c: GIF and animated PNG out of the window",
        &[
            "D-72 added a GIF and an animated PNG as things an export can be, and \
           `verification/B-21c_films_table.md` reads both files back frame by frame. This \
           is the window's half: the list beside Export offers them, the job names one file \
           rather than one a frame, and the status line says so. The folder picker is \
           skipped, as in every panel table: no test can answer a Windows dialog.",
        ],
        &[],
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// B-22c: D-73's choices in the window.
#[test]
fn the_export_choices_are_in_the_window() {
    let mut report = Report { rows: Vec::new() };
    let page =
        std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui/index.html"))
            .expect("the page");
    report.check(
        "the page has a quality list of Preview, Standard and High, starting on Standard and hidden until MP4 is chosen",
        true,
        page.contains("<select id=\"filmquality\" hidden")
            && page.contains("<option value=\"preview\">")
            && page.contains("<option value=\"standard\" selected>")
            && page.contains("<option value=\"high\">")
            && page.contains("$('filmquality').hidden = $('exportformat').value !== 'mp4';"),
    );
    report.check(
        "the page has a Dither tick box, hidden until GIF is chosen",
        true,
        page.contains("<label id=\"ditherlabel\" hidden")
            && page.contains("$('ditherlabel').hidden = $('exportformat').value !== 'gif';"),
    );
    report.check(
        "the list says which entries are lossless: PNG, EXR full float and animated PNG, and no other",
        "3",
        page.lines().filter(|l| l.contains("<option") && l.contains(", lossless</option>")).count(),
    );
    for (query, quality) in [
        ("format=mp4", "Standard"),
        ("format=mp4&quality=preview", "Preview"),
        ("format=mp4&quality=standard", "Standard"),
        ("format=mp4&quality=high", "High"),
    ] {
        report.check(
            &format!("`{query}` is this quality"),
            quality,
            format!(
                "{:?}",
                export_choices(Some(query))
                    .expect("a listed quality")
                    .mp4_quality
            ),
        );
    }
    report.check(
        "a quality the page does not offer is refused in a sentence",
        "Nothing was exported: \"ultra\" is not an MP4 quality. Choose Preview, Standard or High.",
        export_choices(Some("format=mp4&quality=ultra")).expect_err("not a quality"),
    );
    report.check(
        "`dither=on` dithers, and saying nothing does not",
        "true, false",
        format!(
            "{}, {}",
            export_choices(Some("format=gif&dither=on"))
                .expect("choices")
                .gif_dither,
            export_choices(Some("format=gif"))
                .expect("choices")
                .gif_dither
        ),
    );
    let high = ExportChoices {
        mp4_quality: Mp4Quality::High,
        gif_dither: true,
        ..ExportChoices::default()
    };
    report.check(
        "the status line names the MP4's quality",
        " as one MP4 at High quality, over black, with the composition's sound,",
        written_as(OutputFormat::Mp4, high),
    );
    report.check(
        "D-641: `wav=on` asks for the WAV beside, and saying nothing does not",
        "true, false",
        format!(
            "{}, {}",
            export_choices(Some("format=png&wav=on"))
                .expect("choices")
                .wav,
            export_choices(Some("format=png")).expect("choices").wav
        ),
    );
    let wav = ExportChoices {
        wav: true,
        ..ExportChoices::default()
    };
    report.check(
        "D-641: the status line says the WAV is written beside a PNG sequence",
        " with its sound as a WAV beside it,",
        written_as(OutputFormat::Png, wav),
    );
    report.check(
        "D-641: and says nothing of it beside a GIF, which carries no sound",
        " as one GIF, a preview of 256 colours a frame,",
        written_as(OutputFormat::Gif, wav),
    );
    report.check(
        "the status line says a GIF is dithered when it is",
        " as one GIF, a preview of 256 colours a frame, dithered,",
        written_as(OutputFormat::Gif, high),
    );
    report.check(
        "and says nothing of it when it is not",
        " as one GIF, a preview of 256 colours a frame,",
        written_as(OutputFormat::Gif, ExportChoices::default()),
    );
    report.check(
        "a choice that belongs to another format changes nothing said of a PNG sequence",
        "",
        written_as(OutputFormat::Png, high),
    );
    write_artifact(
        &report,
        "verification/B-22c_panel_table.md",
        "B-22c: the export choices in the window",
        &[
            "D-73 gave an MP4 three qualities and a GIF a dithering choice, and \
           `verification/B-22b_choices_table.md` shows what each does to the written file. \
           This is the window's half: the two controls are on the page beside the format \
           they belong to, what the page sends is read into the choice, a value the page \
           does not offer is refused, and the status line says what was chosen. Not \
           reachable by a test, and said so: the one line that hands the choice to the job \
           sits behind the Windows folder dialog, so that the chosen quality reaches the \
           file is what `verification/B-22c_choices_playtest.md` asks the owner to see.",
        ],
        &[],
    );
    let failed: Vec<&(String, String, String)> =
        report.rows.iter().filter(|(_, e, a)| e != a).collect();
    assert!(
        failed.is_empty(),
        "see verification/B-22c_panel_table.md: {failed:?}"
    );
}

/// B-21d: an MP4 from the window (D-72, D-30).
#[cfg(windows)]
#[test]
fn an_mp4_goes_out_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let page =
        std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui/index.html"))
            .expect("the page");
    report.check(
        "the list beside Export offers an MP4, and says it carries the sound (D-641)",
        true,
        page.contains("<option value=\"mp4\">MP4 (H.264), over black, with sound</option>"),
    );
    let into = crate::test_temp().join("anime_compositor_b21d_export");
    let _ = std::fs::remove_dir_all(&into);
    std::fs::create_dir_all(&into).expect("the export folder");
    let format = output_format(Some("format=mp4")).expect("a listed format");
    // The fixture's drawings are not on the disk, so the frames are written without them,
    // as the window's own tick box asks; what is checked here is the file.
    let (snapshot, root, mut request) = export_job(
        &held(&viewer),
        &into,
        MissingSource::RenderTransparent,
        format,
    );
    report.check(
        "the file is named",
        "cel_holds_project.mp4",
        &request.naming,
    );
    request.last_frame = request.first_frame + 2;
    let said = run_export(&snapshot, &root, &request, &AtomicBool::new(false));
    report.check(
        "three frames are one file, and the window says so",
        "Exported 3 frames as one file, the chosen folder's cel_holds_project.mp4.",
        said.replace(
            &format!("{}{}", into.display(), std::path::MAIN_SEPARATOR),
            "the chosen folder's ",
        )
        .split_inclusive('.')
        .take(2)
        .collect::<String>(),
    );
    report.check(
        "the file is there and is not empty",
        true,
        std::fs::metadata(into.join("cel_holds_project.mp4"))
            .map(|m| m.len() > 0)
            .unwrap_or(false),
    );

    // An odd size: the composition made one pixel narrower, as Ctrl+K would.
    let width = {
        let v = held(&viewer);
        v.document
            .project()
            .composition(&v.composition)
            .expect("the composition")
            .width
            - 1
    };
    run(&viewer, &format!("composition.set_settings?width={width}"));
    let (odd, root, mut request) = export_job(
        &held(&viewer),
        &into,
        MissingSource::RenderTransparent,
        format,
    );
    request.naming = "odd.mp4".to_string();
    request.last_frame = request.first_frame;
    let said = run_export(&odd, &root, &request, &AtomicBool::new(false));
    report.check(
        "a composition with an odd width is refused, the side is named, and no file is made",
        "true, true, file there: false".to_string(),
        format!(
            "{}, {}, file there: {}",
            said.contains("An MP4 needs an even width and an even height"),
            said.contains(&format!("its width, {width}, is odd")),
            into.join("odd.mp4").exists()
        ),
    );

    write_artifact(
        &report,
        "verification/B-21d_panel_table.md",
        "B-21d: MP4 out of the window",
        &[
            "D-72 added an MP4 as a thing an export can be, written by the H.264 encoder \
           Windows carries (D-30), and `verification/B-21d_mp4_table.md` reads the written \
           files' own time numbers back. This is the window's half: the list beside Export \
           offers it, the job names one file, the status line says so, and an odd size is \
           refused in a sentence. The folder picker is skipped, as in every panel table: \
           no test can answer a Windows dialog.",
        ],
        &[],
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

const EXR_PANEL_INTRO: &[&str] = &[
    "D-62 decided how an EXR file becomes a drawing and how frames become EXR files, and \
     B-16b built it, checked against OpenEXR's own numbers in \
     `verification/B-16b_exr_table.md`. This is the window's half: Import and Relink \
     accept `.exr` files beside `.png` ones, and a list beside Export chooses PNG, EXR half \
     float or EXR full float.",
    "Every row calls what the window calls, on `Fixtures/projects/cel_holds_project.json` \
     and the files in `Fixtures/exr/`, and reads back what the page is given. The file and \
     folder pickers are the one step skipped: no test can answer a Windows dialog, so the \
     rows start with the files already chosen.",
];

const EXR_PANEL_NOTES: &[&str] = &[
    "## What this does not cover\n\nWhether the pictures look right on screen, and whether \
     the exported files open in another program. That is \
     `verification/B-16c_exr_playtest.md`, for a person. Whether the pixels are right is \
     B-16b's table, against OpenEXR itself.",
];

/// One layer of the composition on screen, by name, as the page is given it.
fn shown_layer(viewer: &Mutex<Viewer>, name: &str) -> serde_json::Value {
    let answer: serde_json::Value =
        serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
    answer["project"]["compositions"][0]["layers"]
        .as_array()
        .expect("a composition has layers")
        .iter()
        .find(|l| l["name"] == name)
        .cloned()
        .unwrap_or(serde_json::Value::Null)
}

/// B-17c: the adjustment layer from the window, on D-66. The order is the playtest sheet's.
#[test]
fn an_adjustment_layer_is_added_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );

    let before = names(&viewer);
    run(&viewer, "layer.add_adjustment");
    report.check(
        "New adjustment layer needs no drawing chosen and adds one at the front",
        format!("{before}, Adjustment layer"),
        names(&viewer),
    );
    let layer = shown_layer(&viewer, "Adjustment layer");
    report.check(
        "the page is told its kind, and it has no drawing",
        "adjustment, no asset_id",
        format!(
            "{}, {}",
            layer["kind"].as_str().unwrap_or("(no kind)"),
            if layer.get("asset_id").is_none() {
                "no asset_id"
            } else {
                "an asset_id"
            }
        ),
    );
    report.check(
        "it covers the composition: anchor and position at the centre of 1920 by 1080",
        "[960,540] and [960,540]",
        format!(
            "{} and {}",
            layer["transform"]["anchor"]["base"], layer["transform"]["position"]["base"]
        ),
    );
    report.check(
        "and it runs the whole composition",
        "frames 0 to 5",
        format!("frames {} to {}", layer["in_frame"], layer["out_frame"]),
    );
    report.check(
        "Undo says what it would take back",
        "Add layer Adjustment layer",
        held(&viewer)
            .document
            .undo_labels()
            .last()
            .cloned()
            .unwrap_or_default(),
    );

    // With a layer chosen, the page sends `to`: the index above it.
    run(&viewer, "layer.add_adjustment?to=1&name=Grade");
    report.check(
        "with a layer chosen, the new one lands just above it",
        "Cel, Grade, Adjustment layer",
        names(&viewer),
    );
    let id = shown_layer(&viewer, "Grade")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    run(
        &viewer,
        &format!("effect.add?layer={id}&type=core.exposure"),
    );
    report.check(
        "an effect is added to it as to any layer",
        "1 effect(s): core.exposure",
        {
            let stack = shown_layer(&viewer, "Grade")["effects"].clone();
            format!(
                "{} effect(s): {}",
                stack.as_array().map_or(0, |a| a.len()),
                stack[0]["type_id"].as_str().unwrap_or("(none)")
            )
        },
    );
    // D-297: an adjustment layer takes any blend mode, as in After Effects (D-66 refused).
    run(
        &viewer,
        &format!("layer.set_blend_mode?layer={id}&mode=multiply"),
    );
    report.check(
        "a blend mode other than normal is taken (D-297)",
        "multiply",
        shown_layer(&viewer, "Grade")["blend_mode"]
            .as_str()
            .unwrap_or("(none)"),
    );

    run(&viewer, &format!("layer.delete?layer={id}&frame=0"));
    report.check(
        "Delete layer takes it out",
        "Cel, Adjustment layer",
        names(&viewer),
    );
    run(&viewer, "edit.undo");
    report.check(
        "and Undo brings it back as an adjustment layer, effect and all",
        "Cel, Grade, Adjustment layer; adjustment with 1 effect(s)",
        format!(
            "{}; {} with {} effect(s)",
            names(&viewer),
            shown_layer(&viewer, "Grade")["kind"]
                .as_str()
                .unwrap_or("(no kind)"),
            shown_layer(&viewer, "Grade")["effects"]
                .as_array()
                .map_or(0, |a| a.len())
        ),
    );

    write_artifact(
        &report,
        "verification/B-17c_panel_table.md",
        "B-17c: adjustment layers in the window",
        ADJUST_PANEL_INTRO,
        ADJUST_PANEL_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// B-26c: the null from the window, on D-82. The order is the playtest sheet's.
#[test]
fn a_null_is_added_and_carries_its_children_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    // Each layer's outline on frame 0, as the page is given it, at full size. This project's
    // drawings are not on disk, so the drawn layers have no outline here and the nulls do.
    let outline = |viewer: &Mutex<Viewer>, id: &str| {
        let body = boxes(viewer, 0, Some(PreviewQuality::Full)).into_body();
        let answer: serde_json::Value =
            serde_json::from_slice(&body).expect("the boxes answer is JSON");
        let found = answer["layers"].as_array().cloned().unwrap_or_default();
        let at = found.iter().position(|b| b["layer"] == id);
        let corners = at.map_or(Vec::new(), |at| {
            found[at]["corners"].as_array().map_or(Vec::new(), |c| {
                c.iter().map(|v| v.as_f64().unwrap_or(f64::NAN)).collect()
            })
        });
        (at, corners)
    };

    let before = names(&viewer);
    run(&viewer, "layer.add_null");
    report.check(
        "New null (Ctrl+Alt+Shift+Y) needs no drawing chosen and adds Null 1 at the front",
        format!("{before}, Null 1"),
        names(&viewer),
    );
    let layer = shown_layer(&viewer, "Null 1");
    let id = layer["id"].as_str().unwrap_or_default().to_string();
    report.check(
        "the page is told it is a null, and it has no drawing",
        "null, no asset_id, no exposure_spans",
        format!(
            "{}, {}, {}",
            layer["kind"].as_str().unwrap_or("(no kind)"),
            if layer.get("asset_id").is_none() {
                "no asset_id"
            } else {
                "an asset_id"
            },
            if layer.get("exposure_spans").is_none() {
                "no exposure_spans"
            } else {
                "exposure_spans"
            }
        ),
    );
    report.check(
        "it stands at the composition's centre, its anchor the middle of its 100 by 100 square, for the whole composition",
        "[50,50] and [960,540], frames 0 to 5",
        format!(
            "{} and {}, frames {} to {}",
            layer["transform"]["anchor"]["base"],
            layer["transform"]["position"]["base"],
            layer["in_frame"],
            layer["out_frame"]
        ),
    );
    report.check(
        "Undo says what it would take back",
        "Add layer Null 1",
        held(&viewer)
            .document
            .undo_labels()
            .last()
            .cloned()
            .unwrap_or_default(),
    );
    let (at, corners) = outline(&viewer, &id);
    report.check(
        "the picture is given its square to outline, 100 by 100 about the centre",
        "given, [910,490 1010,490 1010,590 910,590]",
        format!(
            "{}, [{}]",
            if at.is_some() { "given" } else { "not given" },
            corners
                .chunks(2)
                .map(|p| format!("{:.0},{:.0}", p[0], p[1]))
                .collect::<Vec<_>>()
                .join(" ")
        ),
    );
    report.check(
        "a blend mode on the null is refused, with the reason",
        "\"Null 1\" is a null, which is never drawn, so it has no picture for that to work on.",
        run(
            &viewer,
            &format!("layer.set_blend_mode?layer={id}&mode=multiply"),
        ),
    );
    report.check(
        "the null as Cel's matte is refused, with the reason",
        "\"Null 1\" is a null, which is never drawn, so it cannot be a matte.",
        run(
            &viewer,
            &format!("layer.set_matte?layer=layer-cel&matte={id}&only=false"),
        ),
    );

    // A second null, moved off the first and parented to it, keeps its place; the first
    // moved 100 to the right takes it along.
    run(&viewer, "layer.add_null");
    let child = shown_layer(&viewer, "Null 2")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    run(
        &viewer,
        &format!("property.set_base?layer={child}&prop=position&value=700,300"),
    );
    let (at_child, child_before) = outline(&viewer, &child);
    report.check(
        "Null 2 is added at the front, and its outline comes after Null 1's as its row is above",
        "Cel, Null 1, Null 2; after",
        format!(
            "{}; {}",
            names(&viewer),
            if at_child > outline(&viewer, &id).0 {
                "after"
            } else {
                "before"
            }
        ),
    );
    run(
        &viewer,
        &format!("layer.set_parent?layer={child}&parent={id}&frame=0"),
    );
    let (_, child_parented) = outline(&viewer, &child);
    run(
        &viewer,
        &format!("property.set_base?layer={id}&prop=position&value=1060,540"),
    );
    let (_, child_carried) = outline(&viewer, &child);
    let moved = |a: &[f64], b: &[f64]| match (a.first(), b.first(), a.get(1), b.get(1)) {
        (Some(ax), Some(bx), Some(ay), Some(by)) => format!("{:.0},{:.0}", bx - ax, by - ay),
        _ => "no outline".to_string(),
    };
    report.check(
        "Null 2 parented to Null 1 stays where it was",
        "0,0",
        moved(&child_before, &child_parented),
    );
    report.check(
        "and moving Null 1 100 pixels right carries Null 2 100 pixels right",
        "100,0",
        moved(&child_parented, &child_carried),
    );

    write_artifact(
        &report,
        "verification/B-26c_panel_table.md",
        "B-26c: null layers in the window",
        NULL_PANEL_INTRO,
        NULL_PANEL_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(
        failed.is_empty(),
        "these checks failed: {failed:#?}\n{:#?}",
        report.rows
    );
}

/// B-27c: D-83's pick whip from the window. Each drop is sent as the page sends it.
#[test]
fn the_pick_whip_links_and_parents_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let root = repo("Fixtures/pickwhip");
    let expected: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(root.join("expected_pickwhip.json")).expect("the fixture"),
    )
    .expect("the fixture is JSON");
    let source = root.join("pickwhip_project.json");
    let fresh = || {
        Mutex::new(
            open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
        )
    };
    // What the page puts in the request for one side of a drop, as `whoseId` writes it.
    let side = |prefix: &str, pair: &serde_json::Value| match pair[0].as_str().unwrap_or("") {
        "camera" => format!(
            "{prefix}target=camera&{prefix}prop={}",
            pair[1].as_str().unwrap_or("")
        ),
        id => format!(
            "{prefix}layer={id}&{prefix}prop={}",
            pair[1].as_str().unwrap_or("")
        ),
    };
    // The text the property was given, read from what the page is given.
    let written = |viewer: &Mutex<Viewer>, pair: &serde_json::Value| {
        let answer: serde_json::Value =
            serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
        let comp = &answer["project"]["compositions"][0];
        let (who, prop) = (
            pair[0].as_str().unwrap_or(""),
            pair[1].as_str().unwrap_or(""),
        );
        let record = if who == "camera" {
            comp["camera"][prop].clone()
        } else {
            let layer = comp["layers"]
                .as_array()
                .and_then(|all| all.iter().find(|l| l["id"] == who).cloned())
                .unwrap_or_default();
            if prop == "depth" {
                layer["depth"].clone()
            } else {
                layer["transform"][prop].clone()
            }
        };
        record["expression"]["text"]
            .as_str()
            .map_or("(no expression)".to_string(), |t| t.replace('\n', " "))
    };

    for (fx, case) in expected["cases"].as_object().expect("cases") {
        let viewer = fresh();
        for drop in case["drops"].as_array().expect("drops") {
            let depth = held(&viewer).document.undo_depth();
            run(
                &viewer,
                &format!(
                    "property.link?{}&{}",
                    side("", &drop["destination"]),
                    side("from_", &drop["source"])
                ),
            );
            report.check(
                &format!(
                    "{fx}: {} {} dropped on {} {}, one step of history",
                    drop["destination"][0].as_str().unwrap_or(""),
                    drop["destination"][1].as_str().unwrap_or(""),
                    drop["source"][0].as_str().unwrap_or(""),
                    drop["source"][1].as_str().unwrap_or("")
                ),
                format!(
                    "`{}`, 1 step",
                    drop["text"].as_str().unwrap_or("").replace('\n', " ")
                ),
                format!(
                    "`{}`, {} step",
                    written(&viewer, &drop["destination"]),
                    held(&viewer).document.undo_depth() - depth
                ),
            );
        }
    }

    // Undo names a drop as it names a typed expression, because it is one.
    let (whipped, typed) = (fresh(), fresh());
    run(
        &whipped,
        "property.link?layer=layer-target&prop=position&from_layer=null-1&from_prop=position",
    );
    run(
        &typed,
        "property.set_expression?layer=layer-target&prop=position&text=value",
    );
    let last = |v: &Mutex<Viewer>| {
        held(v)
            .document
            .undo_labels()
            .last()
            .map(|s| s.to_string())
            .unwrap_or_default()
    };
    report.check(
        "Undo names the drop as it names a typed expression",
        last(&typed),
        last(&whipped),
    );
    held(&whipped).document.undo();
    report.check(
        "and Undo takes the drop back",
        "(no expression)",
        written(&whipped, &serde_json::json!(["layer-target", "position"])),
    );

    for (fx, case) in expected["nothing"].as_object().expect("nothing") {
        let viewer = fresh();
        let depth = held(&viewer).document.undo_depth();
        let said = run(
            &viewer,
            &format!(
                "property.link?{}&{}",
                side("", &case["destination"]),
                side("from_", &case["source"])
            ),
        );
        report.check(
            &format!("{fx}: {}", case["says"].as_str().unwrap_or("")),
            "A property cannot be linked to itself.; nothing changed",
            format!(
                "{said}; {}",
                if held(&viewer).document.undo_depth() == depth {
                    "nothing changed"
                } else {
                    "changed"
                }
            ),
        );
    }
    let viewer = fresh();
    report.check(
        "a separated half is not a place to drop, and says so",
        "An expression cannot read position_x, so nothing can link to it.",
        run(
            &viewer,
            "property.link?layer=layer-target&prop=rotation&from_layer=null-1&from_prop=position_x",
        ),
    );
    report.check(
        "nor is a property that does not exist",
        "\"wobble\" is not a property that can be linked.",
        run(
            &viewer,
            "property.link?layer=layer-target&prop=rotation&from_layer=null-1&from_prop=wobble",
        ),
    );

    // The parent whip builds its request as the Parent list does: the same command, the
    // same three parameters, the frame on screen.
    let page = std::fs::read_to_string(repo("app/ui/index.html")).expect("the page");
    report.check(
        "the parent whip sends what the Parent list sends",
        "both: /layer.set_parent?layer=…&parent=…&frame=frame",
        format!(
            "{}: /layer.set_parent?layer=…&parent=…&frame=frame",
            match (
                page.contains("'/layer.set_parent?layer=' + encodeURIComponent(layer.id)\n          + '&parent=' + encodeURIComponent(parent.id) + '&frame=' + frame"),
                page.contains("'/layer.set_parent?layer=' + encodeURIComponent(layer.id)\n    + '&parent=' + encodeURIComponent(chooser.value) + '&frame=' + frame"),
            ) {
                (true, true) => "both",
                (true, false) => "the whip only",
                (false, true) => "the list only",
                (false, false) => "neither",
            }
        ),
    );
    let viewer = fresh();
    run(
        &viewer,
        "layer.set_parent?layer=layer-target&parent=null-1&frame=0",
    );
    report.check(
        "Target whipped onto Null 1 rides on it",
        "null-1",
        shown_layer(&viewer, "Target")["parent"]
            .as_str()
            .unwrap_or("(none)")
            .to_string(),
    );
    report.check(
        "and Null 1 whipped back onto Target is refused as a loop",
        true,
        !run(
            &viewer,
            "layer.set_parent?layer=null-1&parent=layer-target&frame=0",
        )
        .is_empty()
            && shown_layer(&viewer, "Null 1")["parent"].is_null(),
    );

    write_artifact(
        &report,
        "verification/B-27c_panel_table.md",
        "B-27c: the pick whip in the window",
        PICKWHIP_PANEL_INTRO,
        PICKWHIP_PANEL_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(
        failed.is_empty(),
        "these checks failed: {failed:#?}\n{:#?}",
        report.rows
    );
}

/// B-28d: the Sheet tab, on D-84a. FX-XDTS-040's grid, as the page is given it, against
/// document 25's table of what the cut must show, read from document 25 itself.
#[test]
fn the_sheet_tab_matches_the_fixture_catalogue() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    import_cut(&viewer, &repo("Fixtures/xdts/fx_xdts_040"));
    let grid = |viewer: &Mutex<Viewer>| -> serde_json::Value {
        serde_json::from_slice(&sheet(viewer).into_body()).expect("the sheet answer is JSON")
    };
    let columns = |g: &serde_json::Value| {
        g["columns"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|c| c["name"].as_str().unwrap_or("").to_string())
            .collect::<Vec<_>>()
    };
    // A column as written: `-` outside the layer and `.` for a blank that goes on.
    let written = |g: &serde_json::Value, name: &str| {
        g["columns"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|c| c["name"] == name)
            .and_then(|c| c["cells"].as_array())
            .into_iter()
            .flatten()
            .map(|cell| match cell.as_str() {
                None => "-",
                Some("") => ".",
                Some(mark) => mark,
            })
            .collect::<Vec<_>>()
            .join(" ")
    };
    // The drawing a cell stands for, which is what document 25's table has in it.
    let shown = |g: &serde_json::Value, name: &str| -> Vec<String> {
        let mut last = String::new();
        written(g, name)
            .split(' ')
            .map(|mark| match mark {
                "|" => last.clone(),
                "x" | "." => "x".to_string(),
                "-" => "out".to_string(),
                number => {
                    last = number.to_string();
                    last.clone()
                }
            })
            .collect()
    };

    let g = grid(&viewer);
    report.check(
        "a column for each drawing layer, the bottom of the stack on the left",
        "A, B, C",
        columns(&g).join(", "),
    );
    report.check(
        "the sheet starts at the composition's first frame",
        "0",
        g["from"].to_string(),
    );

    let catalogue = std::fs::read_to_string(repo("Markdown/25_Test_Fixture_Catalog.md"))
        .expect("read document 25");
    let table = catalogue
        .split("### FX-XDTS-040")
        .nth(1)
        .and_then(|rest| rest.split("Its notes").next())
        .expect("document 25 has FX-XDTS-040's table");
    let rows = table.lines().filter_map(|line| {
        let cells: Vec<&str> = line
            .trim()
            .trim_matches('|')
            .split('|')
            .map(str::trim)
            .collect();
        cells
            .first()?
            .parse::<usize>()
            .ok()
            .map(|frame| (frame, cells[1..].to_vec()))
    });
    let by_column = ["A", "B", "C"].map(|name| shown(&g, name));
    let mut read = 0;
    for (frame, expected) in rows {
        read += 1;
        let pair = |cells: Vec<&str>| {
            ["A", "B", "C"]
                .iter()
                .zip(cells)
                .map(|(n, c)| format!("{n} {c}"))
                .collect::<Vec<_>>()
                .join(", ")
        };
        report.check(
            &format!("frame {frame}, as document 25 has it"),
            pair(expected),
            pair(
                by_column
                    .iter()
                    .map(|c| c.get(frame).map_or("(none)", String::as_str))
                    .collect(),
            ),
        );
    }
    report.check(
        "document 25's table has a row for each of the 48 frames",
        "48",
        read.to_string(),
    );

    report.check(
        "A as written: a number where a drawing starts and a line while it holds",
        format!(
            "1 | 2 | 3 | 4 | 5 | 6 | 7 | 8{} 7 | 6 | 5 | 4 | 3 | 2 | 1{}",
            " |".repeat(9),
            " |".repeat(11)
        ),
        written(&g, "A"),
    );
    report.check(
        "B as written: a cross where the blank starts on frame 16, and nothing while it lasts",
        format!(
            "1 | | 2 | | 3 2 1 | | 2 | | 3 2 x{} 1 | | 2 | | 3 | | 1{}",
            " .".repeat(13),
            " |".repeat(8)
        ),
        written(&g, "B"),
    );
    report.check(
        "C as written: blank from the start, in at 20, out at 28, back at 40",
        format!(
            "x{} 1 2 3 4 5 | 6 | x{} 1 | 2{}",
            " .".repeat(19),
            " .".repeat(11),
            " |".repeat(5)
        ),
        written(&g, "C"),
    );

    let a = g["columns"][0]["layer"].as_str().unwrap_or("").to_string();
    run(&viewer, &format!("layer.trim?layer={a}&in=4&out=44"));
    report.check(
        "A trimmed to frames 4 to 43: shaded outside, and its first frame in shows its number",
        format!(
            "- - - - 3 | 4 | 5 | 6 | 7 | 8{} 7 | 6 | 5 | 4 | 3 | 2 | 1{}{}",
            " |".repeat(9),
            " |".repeat(7),
            " -".repeat(4)
        ),
        written(&grid(&viewer), "A"),
    );
    run(&viewer, "layer.add_solid");
    report.check(
        "a solid added to the cut has no column, because it has no drawings",
        "4 layers; columns A, B, C",
        format!(
            "{} layers; columns {}",
            names(&viewer).split(", ").count(),
            columns(&grid(&viewer)).join(", ")
        ),
    );
    let page = include_str!("../../ui/index.html");
    report.check(
        "the page has the Sheet as a panel of its own (D-85), and asks for the grid",
        "present",
        if page.contains("<section class=\"panel\" id=\"sheetpanel\"")
            && page.contains("FRAMES + '/sheet'")
        {
            "present"
        } else {
            "absent"
        },
    );

    write_artifact(
        &report,
        "verification/B-28d_sheet_table.md",
        "B-28d: the Sheet tab",
        SHEET_PANEL_INTRO,
        SHEET_PANEL_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(
        failed.is_empty(),
        "these checks failed: {failed:#?}\n{:#?}",
        report.rows
    );
}

/// B-28f: the dialogue and camera columns, on D-84c. FX-XDTS-040's text cells, as the page
/// is given them, against document 25's paragraph on what its Sheet shows.
#[test]
fn the_sheet_text_columns_match_the_fixture_catalogue() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    import_cut(&viewer, &repo("Fixtures/xdts/fx_xdts_040"));
    let grid = |viewer: &Mutex<Viewer>| -> serde_json::Value {
        serde_json::from_slice(&sheet(viewer).into_body()).expect("the sheet answer is JSON")
    };
    let g = grid(&viewer);
    let text = g["text"].as_array().cloned().unwrap_or_default();
    report.check(
        "three text columns: the Sheet's Action (D-84g), then the cut's dialogue and camera instruction",
        "action Action, dialogue Dialogue, camera Camera",
        text.iter()
            .map(|c| format!("{} {}", c["kind"].as_str().unwrap_or(""), c["name"].as_str().unwrap_or("")))
            .collect::<Vec<_>>()
            .join(", "),
    );
    report.check(
        "the drawing columns are as they were",
        "A, B, C",
        g["columns"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|c| c["name"].as_str().unwrap_or("").to_string())
            .collect::<Vec<_>>()
            .join(", "),
    );

    // Document 25: "<name> shows `<text>` on frame <s> and a line on frames <a> to <b>",
    // twice, joined by "; ", and empty on every other frame.
    let catalogue = std::fs::read_to_string(repo("Markdown/25_Test_Fixture_Catalog.md"))
        .expect("read document 25");
    let paragraph = catalogue
        .lines()
        .find(|l| l.starts_with("Its text columns (D-84c):"))
        .expect("document 25 has FX-XDTS-040's text columns");
    let order = paragraph
        .split("left to right, is ")
        .nth(1)
        .and_then(|r| r.split(". ").next())
        .unwrap_or("")
        .trim_end_matches(" (D-84g)");
    // The page's order: Action and dialogue on the left, the drawings, then the rest.
    let named = |c: &serde_json::Value| c["name"].as_str().unwrap_or("").to_string();
    let drawn: Vec<String> = std::iter::once("frame".to_string())
        .chain(text.iter().filter(|c| c["kind"] == "action").map(named))
        .chain(text.iter().filter(|c| c["kind"] == "dialogue").map(named))
        .chain(g["columns"].as_array().into_iter().flatten().map(named))
        .chain(
            text.iter()
                .filter(|c| c["kind"] != "action" && c["kind"] != "dialogue")
                .map(named),
        )
        .collect();
    report.check(
        "document 25's order, left to right, is the Sheet's: Action, dialogue, the drawings bottom first, camera",
        order,
        drawn.join(", "),
    );
    report.check(
        "Action is empty on every frame, as document 25 says",
        if paragraph.contains("Action is empty on every frame.") {
            "48 empty cells"
        } else {
            "(document 25 does not say)"
        },
        format!(
            "{} empty cells",
            text.iter()
                .find(|c| c["kind"] == "action")
                .and_then(|c| c["cells"].as_array())
                .map_or(0, |cells| cells
                    .iter()
                    .filter(|c| c.as_str() == Some(""))
                    .count())
        ),
    );
    let shows = paragraph.split(". ").last().unwrap_or("");
    let mut clauses = 0;
    for clause in shows.split("; ") {
        let Some((name, rest)) = clause.split_once(" shows `") else {
            continue;
        };
        let Some((said, rest)) = rest.split_once("` on frame ") else {
            continue;
        };
        let numbers: Vec<usize> = rest
            .split(|c: char| !c.is_ascii_digit())
            .filter_map(|n| n.parse().ok())
            .collect();
        let [start, from, to] = numbers[..] else {
            continue;
        };
        clauses += 1;
        let expected: Vec<String> = (0..48)
            .map(|f| match f {
                f if f == start => said.to_string(),
                f if (from..=to).contains(&f) => "|".to_string(),
                _ => ".".to_string(),
            })
            .collect();
        let got: Vec<String> = text
            .iter()
            .find(|c| c["name"] == name)
            .and_then(|c| c["cells"].as_array())
            .into_iter()
            .flatten()
            .map(|cell| match cell.as_str().unwrap_or("?") {
                "" => ".".to_string(),
                other => other.to_string(),
            })
            .collect();
        report.check(
            &format!("{name}, frame by frame, as document 25 has it (`.` is an empty cell)"),
            expected.join(" / "),
            got.join(" / "),
        );
    }
    report.check(
        "document 25 describes both columns",
        "2",
        clauses.to_string(),
    );

    let a = g["columns"][0]["layer"].as_str().unwrap_or("").to_string();
    run(&viewer, &format!("exposure.write?layer={a}&frame=1&mark=3"));
    report.check(
        "writing into a drawing column leaves the text columns alone",
        serde_json::Value::Array(text.clone()).to_string(),
        grid(&viewer)["text"].to_string(),
    );
    run(&viewer, "edit.undo");
    run(&viewer, "edit.undo");
    // The composition shown then is the cel project's, which has drawings and so an empty
    // Action column (D-84g).
    report.check(
        "undoing the write and then the import takes the cut's text columns away with it",
        "Action, nothing written",
        grid(&viewer)["text"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|c| {
                let written = c["cells"]
                    .as_array()
                    .is_some_and(|cells| cells.iter().any(|x| x != ""));
                format!(
                    "{}, {}",
                    named(c),
                    if written {
                        "written in"
                    } else {
                        "nothing written"
                    }
                )
            })
            .collect::<Vec<_>>()
            .join("; "),
    );
    let page = include_str!("../../ui/index.html");
    report.check(
        "the page draws the text columns it is given",
        "present",
        if page.contains("grid.text") {
            "present"
        } else {
            "absent"
        },
    );

    write_artifact(
        &report,
        "verification/B-28f_sheet_text_table.md",
        "B-28f: dialogue and camera columns",
        SHEET_TEXT_INTRO,
        SHEET_TEXT_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(
        failed.is_empty(),
        "these checks failed: {failed:#?}\n{:#?}",
        report.rows
    );
}

/// B-28g: printing the Sheet, on D-84d. Each FX-PRINT case, read from document 25 itself,
/// against the printable page the window writes, and every printed cell against the Sheet.
#[test]
fn the_printed_sheet_matches_the_fixture_catalogue() {
    let mut report = Report { rows: Vec::new() };
    let catalogue = std::fs::read_to_string(repo("Markdown/25_Test_Fixture_Catalog.md"))
        .expect("read document 25");
    let section = catalogue
        .split("## Sheet printing fixtures")
        .nth(1)
        .and_then(|rest| rest.split("\n## ").next())
        .expect("document 25 has the sheet printing fixtures");
    // Each case: its name and what it says, then its lines by their label.
    let mut cases: Vec<(String, Vec<(String, String)>)> = Vec::new();
    for line in section.lines().map(str::trim) {
        if let Some(rest) = line.strip_prefix("- FX-PRINT-") {
            cases.push((
                format!("FX-PRINT-{}", rest.split(':').next().unwrap_or(rest)),
                Vec::new(),
            ));
        } else if let (Some(rest), Some(case)) = (line.strip_prefix("- "), cases.last_mut()) {
            let (label, said) = rest.split_once(": ").unwrap_or(("", rest));
            case.1.push((label.to_string(), said.to_string()));
        }
    }
    report.check(
        "document 25 has eleven cases",
        "11",
        cases.len().to_string(),
    );
    // "Everything else as FX-PRINT-001": that case's lines as well as its own.
    let cases: Vec<(String, Vec<(String, String)>)> = cases
        .iter()
        .map(|(name, lines)| {
            let mut all = lines.clone();
            for (_, said) in lines {
                if let Some((_, more)) = said
                    .strip_prefix("Everything else as ")
                    .and_then(|other| cases.iter().find(|(n, _)| n == other))
                {
                    all.extend(more.iter().cloned());
                }
            }
            (name.clone(), all)
        })
        .collect();

    let quoted = |text: &str| -> Vec<String> {
        text.split('`')
            .skip(1)
            .step_by(2)
            .map(str::to_string)
            .collect()
    };
    // The text between `open` and the next `close` after it, for each `open`.
    let between = |text: &str, open: &str, close: &str| -> Vec<String> {
        text.split(open)
            .skip(1)
            .map(|rest| rest.split(close).next().unwrap_or("").to_string())
            .collect()
    };
    // D-84f: a header written as labels, "NAME `s01 c012`; EPISODE, SCENE, CUT and ANIMATOR
    // empty; ...", against a page's title block: each box by its class, and the label the
    // style prints on it.
    let style = sheet_paper_style();
    let labelled = |said: &str, page: &str| -> (String, String) {
        let header = between(page, "<header>", "</header>").concat();
        let mut wanted: Vec<(String, String)> = Vec::new();
        let mut waiting: Vec<String> = Vec::new();
        for (i, part) in said.split('`').enumerate() {
            if i % 2 == 1 {
                if let Some(label) = waiting.pop() {
                    wanted.push((label, part.to_string()));
                }
                waiting.clear();
                continue;
            }
            for word in part.split(|c: char| !c.is_alphanumeric()) {
                if word.len() > 1 && word.chars().all(|c| c.is_ascii_uppercase()) {
                    waiting.push(word.to_string());
                } else if word == "empty" {
                    wanted.extend(waiting.drain(..).map(|l| (l, String::new())));
                }
            }
        }
        let got: Vec<(String, String)> = wanted
            .iter()
            .map(|(label, _)| {
                let class = label.to_lowercase();
                let boxes = between(&header, &format!("class=\"{class}\">"), "<");
                (
                    if style.contains(&format!(".{class}::before {{ content: \"{label}")) {
                        label.clone()
                    } else {
                        format!("{label} (unlabelled)")
                    },
                    if boxes.is_empty() {
                        "(no box)".to_string()
                    } else {
                        boxes.concat()
                    },
                )
            })
            .collect();
        let show = |pairs: &[(String, String)]| {
            pairs
                .iter()
                .map(|(l, v)| {
                    if v.is_empty() {
                        format!("{l} empty")
                    } else {
                        format!("{l} {v}")
                    }
                })
                .collect::<Vec<_>>()
                .join("; ")
        };
        (show(&wanted), show(&got))
    };
    // FX-PRINT-009's four, as its header names them after NAME.
    let header_009 = cases
        .iter()
        .find(|(n, _)| n == "FX-PRINT-009")
        .and_then(|(_, lines)| lines.iter().find(|(l, _)| l == "Header"))
        .map(|(_, said)| said.clone())
        .unwrap_or_default();
    let written: Vec<String> = quoted(&header_009).into_iter().skip(1).collect();
    let four = ["episode", "scene", "cut", "animator"];
    let set_four = |values: &[String]| {
        let q: Vec<String> = four
            .iter()
            .zip(values)
            .map(|(k, v)| format!("{k}={}", v.replace(' ', "%20")))
            .collect();
        format!("composition.set_settings?{}", q.join("&"))
    };
    let shown_comp = |viewer: &Mutex<Viewer>| {
        let held = held(viewer);
        held.document
            .project()
            .composition(&held.composition)
            .cloned()
            .expect("a composition on screen")
    };
    let saved_at = crate::test_temp().join("anime_compositor_b28i");
    std::fs::create_dir_all(&saved_at).expect("a folder to save into");
    let saved_at = saved_at.join("title_block_project.json");
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let page = include_str!("../../ui/index.html");
    for (name, lines) in &cases {
        let viewer = Mutex::new(
            open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
        );
        let mut unwritten = None;
        match name.as_str() {
            n if n.starts_with("FX-PRINT-004") => {
                run(&viewer, "composition.create");
                run(&viewer, "layer.add_solid");
            }
            n => {
                import_cut(&viewer, &repo("Fixtures/xdts/fx_xdts_040"));
                if n == "FX-PRINT-002" || n == "FX-PRINT-006" {
                    run(&viewer, "composition.set_settings?frames=300");
                } else if n == "FX-PRINT-003" {
                    run(&viewer, "composition.set_settings?fps=30");
                } else if n == "FX-PRINT-009" || n == "FX-PRINT-010" {
                    unwritten = Some(shown_comp(&viewer));
                    run(&viewer, &set_four(&written));
                } else if n == "FX-PRINT-011" {
                    // D-84g: `jumps` in Action on frame 20, and K on the first cell where
                    // A's drawing 3 is written.
                    run(&viewer, "sheet.write_action?frame=20&text=jumps");
                    let g = sheet_grid(&viewer);
                    let a = &g["columns"][0];
                    let at = a["cells"]
                        .as_array()
                        .and_then(|cells| cells.iter().position(|c| c == "3"))
                        .expect("A has drawing 3 written");
                    run(
                        &viewer,
                        &format!(
                            "exposure.toggle_key?layer={}&frame={}",
                            a["layer"].as_str().unwrap_or(""),
                            g["from"].as_i64().unwrap_or(0) + at as i64
                        ),
                    );
                }
            }
        }
        // FX-PRINT-010: saved and opened again, looking at the same composition.
        let mut saved_text = String::new();
        let viewer = if name == "FX-PRINT-010" {
            let id = held(&viewer).composition.clone();
            save_as(&viewer, &saved_at);
            saved_text = std::fs::read_to_string(&saved_at).expect("read the saved project");
            let mut reopened = open(&saved_at)
                .unwrap_or_else(|d| panic!("open {}: {}", saved_at.display(), d.message));
            reopened.composition = id;
            Mutex::new(reopened)
        } else {
            viewer
        };
        // D-84e: what the Print the Sheet window asks for, through the window's own answer.
        let query = match name.as_str() {
            "FX-PRINT-005" | "FX-PRINT-006" => Some("seconds=3"),
            "FX-PRINT-007" => Some("colour=red"),
            "FX-PRINT-008" => Some("seconds=4"),
            _ => None,
        };
        let grid = sheet_grid(&viewer);
        let paper = String::from_utf8(sheet_print(&viewer, query).body().clone())
            .expect("the printable page is text");
        let pages = between(&paper, "<section class=\"page\"", "</section>");
        // Each page's header fields in the order document 25 names them unlabelled: name,
        // sheet, length, rate. Read by their class, since the page lays them out as a
        // timesheet does.
        let headers: Vec<Vec<String>> = pages
            .iter()
            .map(|p| {
                let header = between(p, "<header>", "</header>").concat();
                ["name", "sheet", "time", "rate"]
                    .iter()
                    .map(|field| between(&header, &format!("class=\"{field}\">"), "<").concat())
                    .collect()
            })
            .collect();
        let halves: Vec<(String, String)> = between(&paper, "<table data-first=\"", "\">")
            .iter()
            .map(|t| {
                let (first, rest) = t.split_once('"').unwrap_or(("", ""));
                (
                    first.to_string(),
                    rest.rsplit('"').next().unwrap_or("").to_string(),
                )
            })
            .collect();
        if name.starts_with("FX-PRINT-004") {
            let message = lines
                .iter()
                .find_map(|(l, said)| quoted(&format!("{l}: {said}")).first().cloned());
            let message = message.unwrap_or_default();
            report.check(
                &format!("{name}: `/sheet/print` writes no pages"),
                "0 pages",
                format!("{} pages", pages.len()),
            );
            report.check(
                &format!("{name}: the printable page and Ctrl+P say why"),
                format!("{message} / {message}"),
                format!(
                    "{} / {}",
                    if paper.contains(&message) {
                        &message
                    } else {
                        "(not in the page)"
                    },
                    if page.contains(&format!("'{message}'")) {
                        &message
                    } else {
                        "(not in the page)"
                    }
                ),
            );
            report.check(
                &format!("{name}: Print... is not offered with no drawing columns"),
                "hidden",
                if page.contains("$('printsheet').hidden = !comp || !grid || !grid.columns.length")
                    && grid["columns"].as_array().is_none_or(|c| c.is_empty())
                {
                    "hidden"
                } else {
                    "shown"
                },
            );
            continue;
        }
        if name == "FX-PRINT-008" {
            let message = lines
                .iter()
                .flat_map(|(l, said)| quoted(&format!("{l}: {said}")))
                .last()
                .unwrap_or_default();
            report.check(
                &format!("{name}: `/sheet/print` writes no pages, and says why"),
                format!("0 pages, {message}"),
                format!(
                    "{} pages, {}",
                    pages.len(),
                    if paper.contains(&message) {
                        &message
                    } else {
                        "(not said)"
                    }
                ),
            );
            let offered = between(page, "<select id=\"printseconds\">", "</select>").concat();
            report.check(
                &format!("{name}: the Print the Sheet window offers 6 and 3 seconds a page"),
                "6 and 3",
                between(&offered, "<option value=\"", "\"").join(" and "),
            );
            continue;
        }
        for (label, said) in lines {
            match label.as_str() {
                "Header" if said.starts_with("NAME ") => {
                    let (wanted, got) = labelled(said, pages.first().map_or("", String::as_str));
                    report.check(
                        &format!("{name}: the title block of the first page"),
                        wanted,
                        got,
                    );
                }
                "" if said.starts_with("One Undo") => {
                    run(&viewer, "edit.undo");
                    let after = shown_comp(&viewer);
                    report.check(
                        &format!("{name}: one Undo"),
                        "the four empty, the rest as before",
                        if after.sheet_details != Default::default() {
                            "the four still written"
                        } else if Some(&after) != unwritten.as_ref() {
                            "something else changed"
                        } else {
                            "the four empty, the rest as before"
                        },
                    );
                }
                "" if said.starts_with("The four read back") => {
                    let d = shown_comp(&viewer).sheet_details;
                    report.check(
                        &format!("{name}: the four, read back from the file"),
                        written.join(", "),
                        [d.episode, d.scene, d.cut, d.animator].join(", "),
                    );
                    let (wanted, got) =
                        labelled(&header_009, pages.first().map_or("", String::as_str));
                    report.check(&format!("{name}: printed as FX-PRINT-009"), wanted, got);
                }
                "" if said.starts_with("Saved with the four empty") => {
                    run(
                        &viewer,
                        &set_four(&[String::new(), String::new(), String::new(), String::new()]),
                    );
                    save_as(&viewer, &saved_at);
                    let emptied =
                        std::fs::read_to_string(&saved_at).expect("read the saved project");
                    let has = |text: &str| {
                        if text.contains("\"sheet_details\"") {
                            "has it"
                        } else {
                            "has none"
                        }
                    };
                    report.check(
                        &format!("{name}: `sheet_details` in the file, with the four written and then empty"),
                        "has it, then has none",
                        format!("{}, then {}", has(&saved_text), has(&emptied)),
                    );
                }
                "Header" => {
                    let mut got: Vec<String> = Vec::new();
                    if let (Some(first), Some(last)) = (headers.first(), headers.last()) {
                        got.extend(first.iter().take(2).cloned());
                        if headers.len() > 1 {
                            got.push(last.get(1).cloned().unwrap_or_default());
                        }
                        got.extend(first.iter().skip(2).cloned());
                    }
                    report.check(
                        &format!("{name}: the header of the first page (and the last)"),
                        quoted(said).join(", "),
                        got.join(", "),
                    );
                }
                "Columns" => {
                    let first = between(&paper, "<table", "</tr>")
                        .first()
                        .cloned()
                        .unwrap_or_default();
                    // Empty headings side by side are counted, as document 25 writes them.
                    let count = ["", "one", "two", "three", "four", "five", "six"];
                    let mut shown: Vec<String> = Vec::new();
                    let mut blanks = 0;
                    for th in between(&first, "<th", "</th>") {
                        let heading = th.split_once('>').map_or("", |(_, name)| name);
                        if heading.is_empty() {
                            blanks += 1;
                            continue;
                        }
                        if blanks > 0 {
                            shown.push(format!("{} empty", count.get(blanks).unwrap_or(&"many")));
                            blanks = 0;
                        }
                        shown.push(heading.to_string());
                    }
                    if blanks > 0 {
                        shown.push(format!("{} empty", count.get(blanks).unwrap_or(&"many")));
                    }
                    report.check(
                        &format!("{name}: each half's columns, left to right"),
                        said.clone(),
                        shown.join(", "),
                    );
                }
                "Seconds" => {
                    // "a to b, beside rows r1, r2 ... to rN": second a beside r1, each next
                    // one a step of r2 - r1 further on, and b beside rN.
                    let n: Vec<usize> = said
                        .split(|c: char| !c.is_ascii_digit())
                        .filter_map(|w| w.parse().ok())
                        .collect();
                    let expected = match n[..] {
                        [a, b, r1, r2, .., last] if r1 + (b - a) * (r2 - r1) == last => (a..=b)
                            .map(|s| format!("{s} at {}", r1 + (s - a) * (r2 - r1)))
                            .collect::<Vec<_>>()
                            .join(", "),
                        _ => format!("(document 25's line does not add up: {said})"),
                    };
                    let got: Vec<String> = between(&paper, "<tr data-row=\"", "</td>")
                        .iter()
                        .filter_map(|t| {
                            let second = t.rsplit('>').next().unwrap_or("");
                            (!second.is_empty()).then(|| {
                                format!("{second} at {}", t.split('"').next().unwrap_or(""))
                            })
                        })
                        .collect();
                    report.check(
                        &format!("{name}: each second's number, beside its last row"),
                        expected,
                        got.join(", "),
                    );
                }
                "Red" => {
                    report.check(
                        &format!("{name}: the paper is printed red"),
                        "red",
                        if paper.contains("<div id=\"paper\" class=\"red\">") {
                            "red"
                        } else {
                            "black"
                        },
                    );
                    // What the red style colours: its text colour and its lines, by what
                    // they are drawn on. The marks and title entries are named by neither.
                    let named = |what: &str| -> String {
                        let mut on: Vec<String> = sheet_paper_style()
                            .lines()
                            .filter(|l| l.starts_with("#paper.red "))
                            .filter_map(|l| l.split_once(" { "))
                            .filter(|(_, rule)| rule.starts_with(what))
                            .flat_map(|(selectors, _)| selectors.split(", "))
                            .map(|s| {
                                let s = s.trim_start_matches("#paper.red ");
                                s.trim_start_matches("tr.past ")
                                    .trim_start_matches("tr ")
                                    .to_string()
                            })
                            .collect();
                        on.sort();
                        on.dedup();
                        on.join(", ")
                    };
                    report.check(
                        &format!("{name}: red text: the headings, the title labels, the frame and second numbers"),
                        "header > ::before, td.frame, td.sec, th",
                        named("color:"),
                    );
                    report.check(
                        &format!("{name}: red lines: the title block's and the table's rules"),
                        "header, header > *, td, th",
                        named("border-color:"),
                    );
                }
                _ if label.starts_with("Page") => {
                    // Every number but the page numbers, in pairs.
                    let mut numbers = Vec::new();
                    let mut after_page = false;
                    for word in
                        said.split(|c: char| c.is_whitespace() || c == ',' || c == ';' || c == ':')
                    {
                        if word.eq_ignore_ascii_case("page") {
                            after_page = true;
                        } else if let Ok(n) = word.parse::<u32>() {
                            if !after_page {
                                numbers.push(n);
                            }
                            after_page = false;
                        }
                    }
                    let expected: Vec<String> = numbers
                        .chunks(2)
                        .map(|c| format!("{} to {}", c[0], c.get(1).unwrap_or(&0)))
                        .collect();
                    report.check(
                        &format!("{name}: the halves, page by page, left then right"),
                        expected.join(", "),
                        halves
                            .iter()
                            .map(|(a, b)| format!("{a} to {b}"))
                            .collect::<Vec<_>>()
                            .join(", "),
                    );
                }
                _ => {}
            }
        }
        if let Some(end) = lines.iter().find_map(|(l, said)| {
            format!("{l} {said}")
                .contains("End line")
                .then(|| format!("{l}{said}"))
        }) {
            let wanted = end.rsplit(' ').next().unwrap_or("").to_string();
            let got: Vec<String> = between(&paper, "<tr data-row=\"", ">")
                .iter()
                .filter(|t| t.contains("end\"") || t.contains("end "))
                .map(|t| t.split('"').next().unwrap_or("").to_string())
                .collect();
            report.check(
                &format!("{name}: the heavy line under the last frame"),
                wanted,
                got.join(", "),
            );
        }

        // Every printed cell against the Sheet's, in D-84e's order: Action, dialogue, the
        // drawings padded to six cel columns, then the rest. Row 1 is the Sheet's first frame.
        let empty = Vec::new();
        let text = grid["text"].as_array().unwrap_or(&empty);
        let drawings = grid["columns"].as_array().unwrap_or(&empty);
        let order: Vec<Option<&serde_json::Value>> =
            std::iter::once(text.iter().find(|c| c["kind"] == "action"))
                .chain(text.iter().filter(|c| c["kind"] == "dialogue").map(Some))
                .chain(drawings.iter().map(Some))
                .chain((drawings.len()..6).map(|_| None))
                .chain(
                    text.iter()
                        .filter(|c| c["kind"] != "dialogue" && c["kind"] != "action")
                        .map(Some),
                )
                .collect();
        let duration = grid["duration"].as_u64().unwrap_or(0) as usize;
        let (mut compared, mut differ) = (0, Vec::new());
        for row in between(&paper, "<tr data-row=\"", "</tr>") {
            let frame: usize = row
                .split('"')
                .next()
                .and_then(|r| r.parse::<usize>().ok())
                .and_then(|r| r.checked_sub(1))
                .unwrap_or(usize::MAX);
            let cells: Vec<serde_json::Value> = row
                .split("<td")
                .skip(3)
                .map(|td| {
                    let (attrs, rest) = td.split_once('>').unwrap_or(("", ""));
                    let inner = rest.split("</td>").next().unwrap_or("");
                    match (attrs, inner) {
                        (a, _) if a.contains("out") => serde_json::Value::Null,
                        (a, _) if a.contains("hold") => serde_json::json!("|"),
                        (_, "\u{d7}") => serde_json::json!("x"),
                        (_, t) => serde_json::json!(t.replace("&amp;", "&")),
                    }
                })
                .collect();
            if cells.len() != order.len() {
                differ.push(format!(
                    "frame {frame} has {} cells, not {}",
                    cells.len(),
                    order.len()
                ));
            }
            for (column, cell) in order.iter().zip(&cells) {
                let wanted = column
                    .filter(|_| frame < duration)
                    .and_then(|c| c["cells"].get(frame).cloned())
                    .unwrap_or(serde_json::json!(""));
                compared += 1;
                if &wanted != cell {
                    differ.push(format!(
                        "frame {frame} {}",
                        column.map_or(&serde_json::json!("(empty)"), |c| &c["name"])
                    ));
                }
            }
        }
        report.check(
            &format!("{name}: every printed cell is the Sheet's own ({compared} cells, empty past the end)"),
            "none differ",
            if differ.is_empty() { "none differ".to_string() } else { differ.join(", ") },
        );
        // D-84g: what is written in Action, and every circled cell, read off the paper.
        if name == "FX-PRINT-011" {
            let (mut noted, mut circled) = (Vec::new(), Vec::new());
            for row in between(&paper, "<tr data-row=\"", "</tr>") {
                let r = row.split('"').next().unwrap_or("").to_string();
                for (column, td) in order.iter().zip(row.split("<td").skip(3)) {
                    let (attrs, rest) = td.split_once('>').unwrap_or(("", ""));
                    let inner = rest.split("</td>").next().unwrap_or("");
                    let heading = column.map_or("", |c| c["name"].as_str().unwrap_or(""));
                    if heading == "Action" && !inner.is_empty() {
                        noted.push(format!("`{inner}` on row {r}"));
                    }
                    if attrs.contains("key") {
                        circled.push((heading.to_string(), inner.to_string(), r.clone()));
                    }
                }
            }
            for (label, said) in lines {
                if label == "Action" {
                    report.check(
                        &format!("{name}: what the Action column says on paper"),
                        said.clone(),
                        format!("{}, and nothing else in the column", noted.join(", ")),
                    );
                } else if label == "A" {
                    let n = said.split(' ').next().unwrap_or("");
                    let written: Vec<String> = drawings
                        .first()
                        .and_then(|a| a["cells"].as_array())
                        .into_iter()
                        .flatten()
                        .enumerate()
                        .filter(|(_, c)| c.as_str() == Some(n))
                        .map(|(i, _)| (i + 1).to_string())
                        .collect();
                    let on_a: Vec<String> = circled
                        .iter()
                        .filter(|(h, m, _)| h == "A" && m == n)
                        .map(|(_, _, r)| r.clone())
                        .collect();
                    report.check(
                        &format!("{name}: the circled numbers on paper"),
                        said.clone(),
                        if on_a.len() == circled.len() && on_a == written {
                            format!("{n} circled on rows {}, where it is written; no other number circled", on_a.join(" and "))
                        } else {
                            format!(
                                "circled: {}; {n} written on A's rows {}",
                                circled.iter().map(|(h, m, r)| format!("{h} {m} on row {r}")).collect::<Vec<_>>().join(", "),
                                written.join(" and ")
                            )
                        },
                    );
                }
            }
        }
        let keep = match name.as_str() {
            n if n.starts_with("FX-PRINT-001") => Some("verification/B-28g_sheet_print_040.html"),
            n if n.starts_with("FX-PRINT-002") => {
                Some("verification/B-28g_sheet_print_300_frames.html")
            }
            n if n.starts_with("FX-PRINT-003") => {
                Some("verification/B-28g_sheet_print_30_fps.html")
            }
            n if n.starts_with("FX-PRINT-005") => {
                Some("verification/B-28g_sheet_print_3_seconds.html")
            }
            n if n.starts_with("FX-PRINT-007") => Some("verification/B-28g_sheet_print_red.html"),
            n if n.starts_with("FX-PRINT-009") => {
                Some("verification/B-28g_sheet_print_title_block.html")
            }
            n if n.starts_with("FX-PRINT-011") => Some("verification/B-28g_sheet_print_keys.html"),
            _ => None,
        };
        if let Some(file) = keep {
            std::fs::write(repo(file), &paper).expect("write the printable page");
        }
    }
    // Not a fixture case: the project the owner printed at the first playtest, four image
    // sequences and no text columns, kept beside the table to look at.
    let shot = repo("verification/B-08a_project.json");
    let viewer = Mutex::new(
        open(&shot).unwrap_or_else(|d| panic!("open {}: {}", shot.display(), d.message)),
    );
    std::fs::write(
        repo("verification/B-28g_sheet_print_reference_shot.html"),
        sheet_paper(&sheet_grid(&viewer), 6, false),
    )
    .expect("write the printable page");
    report.check(
        "the page prints what the window writes, and Ctrl+P asks for it",
        "present",
        if page.contains("FRAMES + '/sheet/print?seconds='")
            && page.contains("window.print()")
            && page
                .contains("(e.key === 'p' || e.key === 'P')) { e.preventDefault(); printSheet(); }")
        {
            "present"
        } else {
            "absent"
        },
    );

    write_artifact(
        &report,
        "verification/B-28g_sheet_print_table.md",
        "B-28g: printing the Sheet",
        SHEET_PRINT_INTRO,
        SHEET_PRINT_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(
        failed.is_empty(),
        "these checks failed: {failed:#?}\n{:#?}",
        report.rows
    );
}

/// B-28e: writing into the Sheet, on D-84b. Every FX-SHEET case, read from document 25
/// itself, played on the cel project's layer set up as the case's Before line.
#[test]
fn writing_into_the_sheet_matches_the_fixture_catalogue() {
    let mut report = Report { rows: Vec::new() };
    let catalogue = std::fs::read_to_string(repo("Markdown/25_Test_Fixture_Catalog.md"))
        .expect("read document 25");
    let section = catalogue
        .split("## Sheet writing fixtures")
        .nth(1)
        .and_then(|rest| rest.split("\n## ").next())
        .expect("document 25 has the sheet writing fixtures");
    // Each case: its name, what it says, Before, Do, After and Exposures.
    let mut cases: Vec<[String; 6]> = Vec::new();
    for line in section.lines().map(str::trim) {
        if let Some(rest) = line.strip_prefix("- FX-SHEET-") {
            let (number, says) = rest.split_once(": ").unwrap_or((rest, ""));
            let mut case: [String; 6] = Default::default();
            case[0] = format!("FX-SHEET-{number}");
            case[1] = says.to_string();
            cases.push(case);
        } else if let Some(case) = cases.last_mut() {
            for (i, key) in ["- Before:", "- Do:", "- After:", "- Exposures:"]
                .iter()
                .enumerate()
            {
                if let Some(value) = line.strip_prefix(key) {
                    // D-84g's Before and After have more than a line in backticks.
                    case[i + 2] = if i == 0 || i == 2 {
                        value.trim().to_string()
                    } else {
                        value.trim().trim_matches('`').to_string()
                    };
                }
            }
        }
    }
    report.check(
        "document 25 has twenty-four cases",
        "24",
        cases.len().to_string(),
    );
    // D-84g's cases say what the Action column or the keys show, and are played below.
    let (cases, d84g): (Vec<[String; 6]>, Vec<[String; 6]>) = cases
        .into_iter()
        .partition(|c| !c[2].contains("Action `") && !c[2].contains("Keys:"));

    let template: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(repo("Fixtures/projects/cel_holds_project.json"))
            .expect("read the cel project"),
    )
    .expect("the cel project is JSON");
    let folder = crate::test_temp().join("anime_compositor_b28e");
    std::fs::create_dir_all(&folder).expect("make the scratch folder");
    // The layer's column as document 25 writes a line: the drawing on each frame, `x` for
    // nothing and `-` outside the layer.
    let shown = |viewer: &Mutex<Viewer>| {
        let grid: serde_json::Value =
            serde_json::from_slice(&sheet(viewer).into_body()).expect("the sheet answer is JSON");
        let mut last = String::new();
        grid["columns"][0]["cells"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|cell| match cell.as_str() {
                None => "-".to_string(),
                Some("") | Some("x") => "x".to_string(),
                Some("|") => last.clone(),
                Some(number) => {
                    last = number.to_string();
                    last.clone()
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    };
    let depth = |viewer: &Mutex<Viewer>| held(viewer).document.undo_depth();
    let exposures = |viewer: &Mutex<Viewer>| {
        let held = held(viewer);
        held.document
            .project()
            .composition(&held.composition)
            .and_then(|c| c.layer(&Id::new("layer-cel")))
            .map_or(0, |l| l.exposure_spans.len())
    };

    // The cel project with its layer set up as a Before line, opened.
    let set_up = |name: &str, before: &str| {
        // Before's line as exposures: a number the frame before also has holds, unless a
        // `/` between them starts a second exposure of it.
        let marks: Vec<&str> = before.split(' ').collect();
        let in_frame = marks.iter().take_while(|m| **m == "-").count() as i32;
        let mut spans: Vec<serde_json::Value> = Vec::new();
        let (mut frame, mut split, mut last) = (0, false, "");
        for mark in &marks {
            if *mark == "/" {
                split = true;
                continue;
            }
            if let Ok(drawing) = mark.parse::<u32>() {
                let local = frame - in_frame;
                if last == *mark && !split {
                    spans.last_mut().expect("a held exposure")["end_frame_exclusive"] =
                        serde_json::json!(local + 1);
                } else {
                    spans.push(serde_json::json!({
                        "start_frame": local,
                        "end_frame_exclusive": local + 1,
                        "drawing_number": drawing,
                    }));
                }
            }
            (last, split) = (mark, false);
            frame += 1;
        }
        let mut project = template.clone();
        project["assets"][0]["frames"] = (1..=9)
            .map(|n| {
                (
                    n.to_string(),
                    serde_json::json!(format!("media/cel_{n:04}.png")),
                )
            })
            .collect::<serde_json::Map<_, _>>()
            .into();
        let comp = &mut project["compositions"][0];
        comp["duration_frames"] = serde_json::json!(frame);
        comp["work_area"]["end_frame_exclusive"] = serde_json::json!(frame);
        comp["layers"][0]["in_frame"] = serde_json::json!(in_frame);
        comp["layers"][0]["out_frame"] = serde_json::json!(frame);
        comp["layers"][0]["exposure_spans"] = serde_json::json!(spans);
        let path = folder.join(format!("{name}.json"));
        std::fs::write(&path, project.to_string()).expect("write the case");
        Mutex::new(open(&path).unwrap_or_else(|d| panic!("open {name}: {}", d.message)))
    };

    for [name, says, before, action, after, count] in &cases {
        let (before, after) = (before.trim_matches('`'), after.trim_matches('`'));
        let viewer = set_up(name, before);
        let words: Vec<&str> = action.split(' ').collect();
        let (mark, at) = match words.as_slice() {
            ["write", mark, "on", "frame", at] => (*mark, *at),
            ["erase", "frame", at] => ("erase", *at),
            _ => panic!("{name}: cannot read \"{action}\""),
        };
        let was = depth(&viewer);
        let said = run(
            &viewer,
            &format!("exposure.write?layer=layer-cel&frame={at}&mark={mark}"),
        );
        let entries = depth(&viewer) - was;
        let now = shown(&viewer);
        let kept = exposures(&viewer);
        if entries > 0 {
            run(&viewer, "edit.undo");
        }
        let plain = before.replace(" /", "");
        let with_count = |n: String| {
            if count.is_empty() {
                String::new()
            } else {
                format!("; exposures: {n}")
            }
        };
        report.check(
            &format!("{name}: {says}"),
            format!(
                "{after}{}; {} in the history; Undo gives {plain}",
                with_count(count.clone()),
                if after == plain { 0 } else { 1 }
            ),
            format!(
                "{now}{}; {entries} in the history; Undo gives {}",
                with_count(kept.to_string()),
                shown(&viewer)
            ),
        );
        let answer = match name.as_str() {
            "FX-SHEET-009" => "Nothing is written on frame 1 of Cel to erase: it holds the drawing above it.",
            "FX-SHEET-011" => "Frame 2 of Cel already shows drawing 1.",
            "FX-SHEET-014" => "Frame 0 is outside Cel, where its cells are shaded.",
            "FX-SHEET-015" => "Drawing 12 is not in this sequence, so the frames exposing it stay empty; no neighbouring drawing is put there instead.",
            "FX-SHEET-016" => "A cell takes a drawing number, x or an erase. Not \"a\".",
            _ => continue,
        };
        report.check(
            &format!("{name}: what the window says"),
            answer,
            if said.ends_with(answer) {
                answer
            } else {
                &said
            },
        );
    }

    // D-84g. The Action column as document 25 writes it, `.` for an empty cell.
    let noted = |viewer: &Mutex<Viewer>| {
        let grid = sheet_grid(viewer);
        grid["text"]
            .as_array()
            .and_then(|text| text.iter().find(|c| c["kind"] == "action"))
            .and_then(|c| c["cells"].as_array())
            .into_iter()
            .flatten()
            .map(|cell| match cell.as_str().unwrap_or("?") {
                "" => ".".to_string(),
                other => other.to_string(),
            })
            .collect::<Vec<_>>()
            .join(" ")
    };
    let keys = |viewer: &Mutex<Viewer>| {
        let held = held(viewer);
        let keys = held
            .document
            .project()
            .composition(&held.composition)
            .and_then(|c| c.layer(&Id::new("layer-cel")))
            .map(|l| l.key_drawings.clone())
            .unwrap_or_default();
        if keys.is_empty() {
            "none".to_string()
        } else {
            keys.iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(" and ")
        }
    };
    // The frames where the Sheet writes a number that is a key.
    let circled = |viewer: &Mutex<Viewer>| {
        let grid = sheet_grid(viewer);
        let column = &grid["columns"][0];
        let frames: Vec<String> = column["cells"]
            .as_array()
            .into_iter()
            .flatten()
            .enumerate()
            .filter(|(_, cell)| {
                column["keys"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .any(|k| cell.as_str() == Some(k.to_string().as_str()))
            })
            .map(|(i, _)| (grid["from"].as_i64().unwrap_or(0) + i as i64).to_string())
            .collect();
        match frames.len() {
            0 => "none".to_string(),
            1 => format!("frame {}", frames[0]),
            _ => format!("frames {}", frames.join(" and ")),
        }
    };
    // What a Before or After names, as the viewer shows it now. A claim after the Action
    // line, that the composition saves no Action column, is read from a saved file.
    let state = |template: &str, viewer: &Mutex<Viewer>, name: &str| {
        template
            .split("; ")
            .map(|part| {
                if part.starts_with('`') {
                    format!("`{}`", shown(viewer))
                } else if part.starts_with("Action `") {
                    let claim = part.rsplit('`').next().unwrap_or("");
                    let claim = if claim.contains("saves no Action column") {
                        // Saved as `save_as` does, without taking the saved file up, which
                        // would start the history again.
                        let path = folder.join(format!("{name}_saved.json"));
                        {
                            let viewer = viewer.lock().expect("the viewer lock was poisoned");
                            let to = path.parent().unwrap_or(Path::new("."));
                            let mut copy = Document::new(persist::rebased(
                                viewer.document.project(),
                                &viewer.root,
                                to,
                            ));
                            persist::save(&path, &mut copy, &viewer.preserved)
                                .unwrap_or_else(|d| panic!("save {name}: {}", d.message));
                        }
                        let file = std::fs::read_to_string(&path).expect("read the saved case");
                        if file.contains("\"Action\"") {
                            ", and the composition saves an Action column"
                        } else {
                            claim
                        }
                    } else {
                        claim
                    };
                    format!("Action `{}`{claim}", noted(viewer))
                } else if part.starts_with("Keys:") {
                    format!("Keys: {}", keys(viewer))
                } else if part.starts_with("Circled:") {
                    format!("Circled: {}", circled(viewer))
                } else {
                    format!("(cannot read \"{part}\")")
                }
            })
            .collect::<Vec<_>>()
            .join("; ")
    };
    let written = |text: &str| text.replace(' ', "%20");
    for [name, says, before, action, after, _] in &d84g {
        // Before's drawing line, or `1 1 2 2` as long as its Action line.
        let quoted = |part: &str| part.split('`').nth(1).unwrap_or("").to_string();
        let parts: Vec<&str> = before.split("; ").collect();
        let action_before = parts
            .iter()
            .find(|p| p.starts_with("Action `"))
            .map(|p| quoted(p));
        let line = parts
            .iter()
            .find(|p| p.starts_with('`'))
            .map(|p| quoted(p))
            .unwrap_or_else(|| {
                let n = action_before.as_deref().map_or(4, |a| a.split(' ').count());
                ["1", "1", "2", "2"]
                    .iter()
                    .cycle()
                    .take(n)
                    .copied()
                    .collect::<Vec<_>>()
                    .join(" ")
            });
        let mut viewer = set_up(name, &line);
        // Before's notes and keys, put there with the commands the Sheet sends.
        for (frame, note) in action_before.iter().flat_map(|a| a.split(' ').enumerate()) {
            if note != "." {
                run(
                    &viewer,
                    &format!("sheet.write_action?frame={frame}&text={}", written(note)),
                );
            }
        }
        if let Some(k) = parts
            .iter()
            .find_map(|p| p.strip_prefix("Keys: "))
            .filter(|k| *k != "none")
        {
            let at = line
                .split(' ')
                .position(|m| m == k)
                .expect("the key is written in Before");
            run(
                &viewer,
                &format!("exposure.toggle_key?layer=layer-cel&frame={at}"),
            );
        }
        let was = depth(&viewer);
        let mut said = Vec::new();
        let mut reopened = false;
        for step in action.split("; ") {
            let frame = step.rsplit(' ').next().unwrap_or("");
            let query = if let Some(rest) = step.strip_prefix("write `") {
                format!(
                    "sheet.write_action?frame={frame}&text={}",
                    written(rest.split('`').next().unwrap_or(""))
                )
            } else if step.starts_with("erase Action on frame ") {
                format!("sheet.write_action?frame={frame}")
            } else if step.starts_with("press K on frame ") {
                format!("exposure.toggle_key?layer=layer-cel&frame={frame}")
            } else if step == "save, close and open" {
                let path = folder.join(format!("{name}_saved.json"));
                save_as(&viewer, &path);
                let file = std::fs::read_to_string(&path).expect("read the saved case");
                report.check(
                    &format!(
                        "{name}: `key_drawings` in the saved file, for the one layer with keys"
                    ),
                    "1",
                    file.matches("\"key_drawings\"").count().to_string(),
                );
                let id = held(&viewer).composition.clone();
                let mut again =
                    open(&path).unwrap_or_else(|d| panic!("open {name} again: {}", d.message));
                again.composition = id;
                viewer = Mutex::new(again);
                reopened = true;
                continue;
            } else {
                panic!("{name}: cannot read \"{step}\"")
            };
            said.push(run(&viewer, &query));
        }
        let now = state(after, &viewer, name);
        if reopened {
            report.check(&format!("{name}: {says}"), after.clone(), now);
            // The key unmarked again, the layer has none to save.
            let at = line
                .split(' ')
                .position(|m| Some(m) == parts.iter().find_map(|p| p.strip_prefix("Keys: ")));
            run(
                &viewer,
                &format!(
                    "exposure.toggle_key?layer=layer-cel&frame={}",
                    at.unwrap_or(0)
                ),
            );
            let path = folder.join(format!("{name}_no_keys.json"));
            save_as(&viewer, &path);
            report.check(
                &format!("{name}: `key_drawings` in the saved file once the key is unmarked"),
                "0",
                std::fs::read_to_string(&path)
                    .expect("read the saved case")
                    .matches("\"key_drawings\"")
                    .count()
                    .to_string(),
            );
            continue;
        }
        let entries = depth(&viewer)
            .checked_sub(was)
            .expect("the history grew or stayed");
        for _ in 0..entries {
            run(&viewer, "edit.undo");
        }
        // Refused: After says nothing Before did not.
        let unchanged = after
            .split("; ")
            .all(|p| before.split("; ").any(|q| q == p));
        report.check(
            &format!("{name}: {says}"),
            format!(
                "{after}; {} in the history; Undo gives {before}",
                if unchanged {
                    0
                } else {
                    action.split("; ").count()
                }
            ),
            format!(
                "{now}; {entries} in the history; Undo gives {}",
                state(before, &viewer, name)
            ),
        );
        let answers: &[&str] = match name.as_str() {
            "FX-SHEET-020" => &[
                "Nothing is written in Action on frame 2 to erase.",
                "An Action note needs words: spaces alone write nothing.",
            ],
            "FX-SHEET-023" => &[
                "Frame 1 of Cel is a line holding the drawing above it. Press K on the number itself.",
                "Frame 2 of Cel is a cross, which is no drawing to mark as a key.",
                "Frame 3 of Cel is empty, under a cross, so it has no drawing to mark as a key.",
            ],
            _ => continue,
        };
        report.check(
            &format!("{name}: what the window says"),
            answers.join(" / "),
            answers
                .iter()
                .zip(&said)
                .map(|(a, s)| {
                    if s.ends_with(a) {
                        a.to_string()
                    } else {
                        s.clone()
                    }
                })
                .collect::<Vec<_>>()
                .join(" / "),
        );
    }

    let page = include_str!("../../ui/index.html");
    report.check(
        "the page writes a cell with exposure.write",
        "present",
        if page.contains("'/exposure.write?layer='") {
            "present"
        } else {
            "absent"
        },
    );
    report.check(
        "the page writes Action with sheet.write_action, and K sends exposure.toggle_key",
        "present",
        if page.contains("'/sheet.write_action?frame='")
            && page.contains("'/exposure.toggle_key?layer='")
        {
            "present"
        } else {
            "absent"
        },
    );

    write_artifact(
        &report,
        "verification/B-28e_sheet_writing_table.md",
        "B-28e: writing into the Sheet",
        SHEET_WRITING_INTRO,
        SHEET_WRITING_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(
        failed.is_empty(),
        "these checks failed: {failed:#?}\n{:#?}",
        report.rows
    );
}

/// B-28c: a cut imported from its timesheet from the window, on D-84. The order is the
/// playtest sheet's.
#[test]
fn a_cut_is_imported_from_its_timesheet_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let cut = repo("Fixtures/xdts/fx_xdts_040");
    let on_screen = |viewer: &Mutex<Viewer>| {
        let held = held(viewer);
        held.document
            .project()
            .composition(&held.composition)
            .map_or("none".to_string(), |c| {
                format!(
                    "{}, {} by {}, {} frames at {}/{}",
                    c.name,
                    c.width,
                    c.height,
                    c.duration_frames,
                    c.frame_rate.numerator(),
                    c.frame_rate.denominator()
                )
            })
    };
    // The layer as the page is given it, from whichever composition is on screen.
    let given = |viewer: &Mutex<Viewer>, name: &str| {
        let answer: serde_json::Value =
            serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
        let comp = held(viewer).composition.as_str().to_string();
        answer["project"]["compositions"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|c| c["id"] == comp.as_str())
            .and_then(|c| c["layers"].as_array())
            .into_iter()
            .flatten()
            .find(|l| l["name"] == name)
            .cloned()
            .unwrap_or(serde_json::Value::Null)
    };
    // Each note's document 28 code, which follows the tab.
    let codes = |viewer: &Mutex<Viewer>| {
        held(viewer)
            .notes
            .iter()
            .map(|n| {
                let after = n.split('\t').nth(1).unwrap_or("");
                after.split(' ').next().unwrap_or("").to_string()
            })
            .collect::<Vec<_>>()
            .join(", ")
    };

    report.check(
        "Import cut with no folder asks for one",
        "Which cut? Choose the folder that holds its .xdts timesheet.",
        run(&viewer, "timesheet.import"),
    );
    // The project's drawings are not on disk, and it says so in the notes on opening.
    held(&viewer).notes.clear();
    let said = run(
        &viewer,
        &format!("timesheet.import?folder={}", cut.display()),
    );
    report.check(
        "choosing FX-XDTS-040's folder imports it and says what arrived",
        "s01 c012 is imported and open: 3 layers from its timesheet, 48 frames at 24 frames a \
         second, because a timesheet does not say its rate. 2 notes are below. Ctrl+Z takes \
         it all back.",
        &said,
    );
    report.check(
        "the new composition is the one on screen, named from the sheet and the size of its drawings",
        "s01 c012, 160 by 90, 48 frames at 24/1",
        on_screen(&viewer),
    );
    report.check(
        "a layer a column, track 0 at the bottom",
        "A, B, C",
        names(&viewer),
    );
    report.check(
        "each layer's inspector is given where its timing came from",
        "fx_xdts_040.xdts, column A, track 0; fx_xdts_040.xdts, column B, track 1; \
         fx_xdts_040.xdts, column C, track 2",
        ["A", "B", "C"]
            .iter()
            .map(|n| {
                let t = &given(&viewer, n)["timesheet"];
                format!(
                    "{}, column {}, track {}",
                    t["sheet"].as_str().unwrap_or("(none)"),
                    t["column"].as_str().unwrap_or("(none)"),
                    t["track"]
                )
            })
            .collect::<Vec<_>>()
            .join("; "),
    );
    report.check(
        "the page shows that record as one line of the inspector",
        "present",
        if include_str!("../../ui/index.html")
            .contains("if (layer.timesheet) field(dl, 'Timesheet'")
        {
            "present"
        } else {
            "absent"
        },
    );
    report.check(
        "B's exposures reach the page as the sheet has them, blank from frame 16 to 29",
        "0-3:1 3-6:2 6-7:3 7-8:2 8-11:1 11-14:2 14-15:3 15-16:2 30-33:1 33-36:2 36-39:3 39-48:1",
        given(&viewer, "B")["exposure_spans"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|s| {
                format!(
                    "{}-{}:{}",
                    s["start_frame"], s["end_frame_exclusive"], s["drawing_number"]
                )
            })
            .collect::<Vec<_>>()
            .join(" "),
    );
    // The drawings are on disk, so the picture is given an outline for each layer that shows
    // one on a frame: on frame 20 A holds and C has come in, and B is blank.
    let outlined = |frame: i32| {
        let body = boxes(&viewer, frame, Some(PreviewQuality::Full)).into_body();
        let answer: serde_json::Value =
            serde_json::from_slice(&body).expect("the boxes answer is JSON");
        let ids: Vec<String> = answer["layers"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|b| b["layer"].as_str().map(str::to_string))
            .collect();
        ["A", "B", "C"]
            .iter()
            .filter(|n| ids.iter().any(|id| given(&viewer, n)["id"] == id.as_str()))
            .copied()
            .collect::<Vec<_>>()
            .join(", ")
    };
    report.check(
        "on frame 20 the picture outlines A and C, and B is blank",
        "A, C",
        outlined(20),
    );
    report.check(
        "on frame 0 it outlines A and B, before C comes in",
        "A, B",
        outlined(0),
    );
    report.check(
        "what was not read goes to the notes: C's tick mark and the background (dialogue and camerawork are read since D-84c)",
        "TIMESHEET_MARK, TIMESHEET_NOT_USED",
        codes(&viewer),
    );
    report.check(
        "Undo names the import by its first step and the count of the rest",
        "Import A_%04d.png and 3 more",
        held(&viewer)
            .document
            .undo_labels()
            .last()
            .cloned()
            .unwrap_or_default(),
    );
    run(&viewer, "edit.undo");
    report.check(
        "one Ctrl+Z takes the whole cut away and goes back to the composition before it",
        "Cel; 1 composition, 1 drawing",
        {
            let n = names(&viewer);
            let held = held(&viewer);
            let project = held.document.project();
            format!(
                "{n}; {} composition{}, {} drawing{}",
                project.compositions.len(),
                if project.compositions.len() == 1 {
                    ""
                } else {
                    "s"
                },
                project.assets.len(),
                if project.assets.len() == 1 { "" } else { "s" }
            )
        },
    );

    held(&viewer).notes.clear();
    let refused = run(
        &viewer,
        &format!(
            "timesheet.import?folder={}",
            repo("Fixtures/xdts/fx_xdts_031").display()
        ),
    );
    report.check(
        "a folder with two timesheets imports nothing and says why",
        "Nothing was imported.",
        refused
            .split(". ")
            .next()
            .map_or(String::new(), |s| format!("{s}.")),
    );
    report.check(
        "and the refusal is in the notes with its code",
        "TIMESHEET_NOT_FOUND",
        codes(&viewer),
    );
    report.check(
        "and nothing was added",
        "Cel; 1 composition",
        format!(
            "{}; {} composition",
            names(&viewer),
            held(&viewer).document.project().compositions.len()
        ),
    );

    write_artifact(
        &report,
        "verification/B-28c_panel_table.md",
        "B-28c: a cut imported from its timesheet, in the window",
        TIMESHEET_PANEL_INTRO,
        TIMESHEET_PANEL_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(
        failed.is_empty(),
        "these checks failed: {failed:#?}\n{:#?}",
        report.rows
    );
}

/// B-23c: the solid from the window, on D-74. The order is the playtest sheet's.
#[test]
fn a_solid_is_added_and_set_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let solid_of = |viewer: &Mutex<Viewer>, name: &str| {
        let layer = shown_layer(viewer, name);
        format!(
            "{} {}x{}, anchor {}",
            layer["solid"]["color"],
            layer["solid"]["width"],
            layer["solid"]["height"],
            layer["transform"]["anchor"]["base"]
        )
    };

    let before = names(&viewer);
    run(&viewer, "layer.add_solid");
    report.check(
        "New solid (Ctrl+Y) needs no drawing chosen and adds Solid 1 at the front",
        format!("{before}, Solid 1"),
        names(&viewer),
    );
    let layer = shown_layer(&viewer, "Solid 1");
    report.check(
        "the page is told its kind and its solid record, and it has no drawing",
        "solid, no asset_id, no exposure_spans",
        format!(
            "{}, {}, {}",
            layer["kind"].as_str().unwrap_or("(no kind)"),
            if layer.get("asset_id").is_none() {
                "no asset_id"
            } else {
                "an asset_id"
            },
            if layer.get("exposure_spans").is_none() {
                "no exposure_spans"
            } else {
                "exposure_spans"
            }
        ),
    );
    report.check(
        "it is the composition's size, mid grey (the picker's #808080, stored linear)",
        "0.215861 x3, 1920 by 1080",
        format!(
            "{:.6} x{}, {} by {}",
            layer["solid"]["color"][0].as_f64().unwrap_or(-1.0),
            layer["solid"]["color"].as_array().map_or(0, |c| c
                .iter()
                .filter(|v| *v == &layer["solid"]["color"][0])
                .count()),
            layer["solid"]["width"],
            layer["solid"]["height"]
        ),
    );
    report.check(
        "it covers the composition: anchor and position at the centre of 1920 by 1080",
        "[960,540] and [960,540]",
        format!(
            "{} and {}",
            layer["transform"]["anchor"]["base"], layer["transform"]["position"]["base"]
        ),
    );
    report.check(
        "and it runs the whole composition",
        "frames 0 to 5",
        format!("frames {} to {}", layer["in_frame"], layer["out_frame"]),
    );
    report.check(
        "Undo says what it would take back",
        "Add layer Solid 1",
        held(&viewer)
            .document
            .undo_labels()
            .last()
            .cloned()
            .unwrap_or_default(),
    );

    // With a layer chosen, the page sends `to`: the index above it.
    run(&viewer, "layer.add_solid?to=1");
    report.check(
        "a second one is Solid 2, and with a layer chosen it lands just above it",
        "Cel, Solid 2, Solid 1",
        names(&viewer),
    );
    let id = shown_layer(&viewer, "Solid 2")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    // The picker's #ff0000 is what `linearOf` sends as 1,0,0.
    run(&viewer, &format!("solid.set?layer={id}&color=1,0,0"));
    report.check(
        "the colour picker sets the colour, and the size stays",
        "[1,0,0] 1920x1080, anchor [960,540]",
        solid_of(&viewer, "Solid 2"),
    );
    run(&viewer, &format!("solid.set?layer={id}&width=960"));
    report.check(
        "Width 960 halves it, and the anchor stays at its middle",
        "[1,0,0] 960x1080, anchor [480,540]",
        solid_of(&viewer, "Solid 2"),
    );
    report.check(
        "a width of 0 is refused, with the reason",
        "That solid cannot be made: it needs a width and height from 1 to 8192, not 0.",
        run(&viewer, &format!("solid.set?layer={id}&width=0")),
    );
    report.check(
        "a width that is not a number is refused, with the reason",
        "\"wide\" is not a width. A width is a whole number of pixels, from 1 to 8192.",
        run(&viewer, &format!("solid.set?layer={id}&width=wide")),
    );
    report.check(
        "a colour outside 0 to 1 is refused, with the reason",
        "That solid cannot be made: it needs a colour of three numbers from 0 to 1, not 2.",
        run(&viewer, &format!("solid.set?layer={id}&color=2,0,0")),
    );
    report.check(
        "and after the three refusals it is as it was",
        "[1,0,0] 960x1080, anchor [480,540]",
        solid_of(&viewer, "Solid 2"),
    );
    run(&viewer, "edit.undo");
    report.check(
        "one Undo takes back the width, anchor and all",
        "[1,0,0] 1920x1080, anchor [960,540]",
        solid_of(&viewer, "Solid 2"),
    );
    let cel = shown_layer(&viewer, "Cel")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    report.check(
        "a drawing's layer has no colour to set, and says so",
        "\"Cel\" is not a solid, so it has no colour and size.",
        run(&viewer, &format!("solid.set?layer={cel}&color=1,0,0")),
    );
    run(
        &viewer,
        &format!("layer.set_blend_mode?layer={id}&mode=multiply"),
    );
    report.check(
        "a solid takes any blend mode",
        "multiply",
        shown_layer(&viewer, "Solid 2")["blend_mode"]
            .as_str()
            .unwrap_or("(none)"),
    );

    run(&viewer, &format!("layer.delete?layer={id}&frame=0"));
    report.check("Delete layer takes it out", "Cel, Solid 1", names(&viewer));
    run(&viewer, "edit.undo");
    report.check(
        "and Undo brings it back as the same solid",
        "Cel, Solid 2, Solid 1; solid [1,0,0] 1920x1080, anchor [960,540]",
        format!(
            "{}; {} {}",
            names(&viewer),
            shown_layer(&viewer, "Solid 2")["kind"]
                .as_str()
                .unwrap_or("(no kind)"),
            solid_of(&viewer, "Solid 2")
        ),
    );

    write_artifact(
        &report,
        "verification/B-23c_panel_table.md",
        "B-23c: solid layers in the window",
        SOLID_PANEL_INTRO,
        SOLID_PANEL_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// B-24c: the masks from the window, on D-77. The order is the playtest sheet's.
#[test]
fn masks_are_drawn_and_set_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let cel = shown_layer(&viewer, "Cel")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    // What the page draws from and what its panel shows: every mask of the layer, in order.
    let masks_of = |viewer: &Mutex<Viewer>| {
        let layer = shown_layer(viewer, "Cel");
        let masks = layer["masks"].as_array().cloned().unwrap_or_default();
        if masks.is_empty() {
            return "none".to_string();
        }
        masks
            .iter()
            .map(|m| {
                format!(
                    "{} {} {} points, opacity {}, feather {}, expansion {}{}{}{}",
                    m["name"].as_str().unwrap_or("(no name)"),
                    m["mode"].as_str().unwrap_or("(no mode)"),
                    m["path"]["base"]["points"]
                        .as_array()
                        .map_or(0, |p| p.len()),
                    m["opacity"],
                    m["feather_px"],
                    m["expansion_px"],
                    if m["inverted"] == serde_json::Value::Bool(true) {
                        ", inverted"
                    } else {
                        ""
                    },
                    if m["enabled"] == serde_json::Value::Bool(false) {
                        ", off"
                    } else {
                        ""
                    },
                    if m["closed"] == serde_json::Value::Bool(false) {
                        ", open"
                    } else {
                        ""
                    },
                )
            })
            .collect::<Vec<_>>()
            .join("; ")
    };
    // The corners of a rectangle over the whole of a 1920 by 1080 layer, which is what New
    // mask sends, and what the rectangle tool sends when it is dragged corner to corner.
    let whole = "0,0,0,0,0,0;1920,0,0,0,0,0;1920,1080,0,0,0,0;0,1080,0,0,0,0";

    report.check(
        "the drawing layer starts with no masks",
        "none",
        masks_of(&viewer),
    );
    run(&viewer, &format!("mask.add?layer={cel}&points={whole}"));
    report.check(
        "New mask, the pen and the two shape tools all send mask.add, and the first is Mask 1",
        "Mask 1 add 4 points, opacity 1, feather 0, expansion 0",
        masks_of(&viewer),
    );
    report.check(
        "Undo says what it would take back",
        "Draw Mask 1",
        held(&viewer)
            .document
            .undo_labels()
            .last()
            .cloned()
            .unwrap_or_default(),
    );
    // An ellipse, as the ellipse tool draws one: four points with the handles that round it.
    run(
        &viewer,
        &format!(
            "mask.add?layer={cel}&points=960,0,-265.096,0,265.096,0;1920,540,0,-149.117,0,\
             149.117;960,1080,265.096,0,-265.096,0;0,540,0,149.117,0,-149.117"
        ),
    );
    report.check(
        "a second one is Mask 2, and the ellipse keeps its handles",
        "Mask 1 add 4 points, opacity 1, feather 0, expansion 0; \
         Mask 2 add 4 points, opacity 1, feather 0, expansion 0",
        masks_of(&viewer),
    );
    report.check(
        "the curved point really is curved: its outgoing handle is the one that was drawn",
        "[265.096,0]",
        shown_layer(&viewer, "Cel")["masks"][1]["path"]["base"]["points"][0]["out"].to_string(),
    );

    // A point dragged, or nudged with the arrow keys: the whole path goes back each time.
    run(
        &viewer,
        &format!(
            "mask.set_path?layer={cel}&mask=0&points=100,0,0,0,0,0;1920,0,0,0,0,0;\
             1920,1080,0,0,0,0;0,1080,0,0,0,0"
        ),
    );
    report.check(
        "moving one point moves that point and no other",
        "[100,0] then [1920,0]",
        format!(
            "{} then {}",
            shown_layer(&viewer, "Cel")["masks"][0]["path"]["base"]["points"][0]["point"],
            shown_layer(&viewer, "Cel")["masks"][0]["path"]["base"]["points"][1]["point"]
        ),
    );

    // The panel's own controls, each sending one setting and leaving the rest alone.
    run(
        &viewer,
        &format!("mask.set?layer={cel}&mask=0&mode=subtract"),
    );
    run(&viewer, &format!("mask.set?layer={cel}&mask=0&opacity=50"));
    run(&viewer, &format!("mask.set?layer={cel}&mask=0&feather=8"));
    run(
        &viewer,
        &format!("mask.set?layer={cel}&mask=0&expansion=-4"),
    );
    run(&viewer, &format!("mask.set?layer={cel}&mask=0&inverted=1"));
    report.check(
        "the mode, opacity, feather, expansion and invert of one mask, each on its own",
        "Mask 1 subtract 4 points, opacity 0.5, feather 8, expansion -4, inverted",
        masks_of(&viewer)
            .split("; ")
            .next()
            .unwrap_or_default()
            .to_string(),
    );
    run(&viewer, &format!("mask.set?layer={cel}&mask=1&enabled=1"));
    report.check(
        "the On button turns one mask off without touching the other",
        "Mask 2 add 4 points, opacity 1, feather 0, expansion 0, off",
        masks_of(&viewer)
            .split("; ")
            .nth(1)
            .unwrap_or_default()
            .to_string(),
    );

    // What must be refused, each with the reason a person is given.
    report.check(
        "a shape of fewer than three points is refused by the core, with the reason",
        "A mask needs at least three points, and this one has 2. Add points until the \
         shape closes on an area.",
        run(
            &viewer,
            &format!("mask.add?layer={cel}&points=0,0,0,0,0,0;10,10,0,0,0,0"),
        ),
    );
    report.check(
        "a shape that crosses itself is refused, with the reason",
        "The mask crosses itself, which this build does not draw. Move the points so that \
         no edge crosses another. A figure-of-eight has to become two masks, which this \
         build now has: draw the second one and set its mode.",
        run(
            &viewer,
            &format!(
                "mask.add?layer={cel}&points=0,0,0,0,0,0;100,100,0,0,0,0;\
                 100,0,0,0,0,0;0,100,0,0,0,0"
            ),
        ),
    );
    report.check(
        "an opacity outside 0 to 1 is refused, with the reason",
        "Mask \"Mask 1\" was given an opacity of 1.5, which is not from 0 to 1. Set a \
         value inside the range and send the masks again.",
        run(&viewer, &format!("mask.set?layer={cel}&mask=0&opacity=150")),
    );
    report.check(
        "a point that is not six numbers is refused before the core sees it",
        "A mask point is six numbers, x,y,in_x,in_y,out_x,out_y. Not \"0,0\".",
        run(&viewer, &format!("mask.add?layer={cel}&points=0,0")),
    );
    report.check(
        "a mode nobody has is refused, with the five there are",
        "A mask mode is add, subtract, intersect, difference or none. Not \"lighten\".",
        run(
            &viewer,
            &format!("mask.set?layer={cel}&mask=0&mode=lighten"),
        ),
    );
    report.check(
        "a mask that is not there is refused, and the window says how many there are",
        "Which mask? \"Cel\" has 2.",
        run(&viewer, &format!("mask.set?layer={cel}&mask=7&opacity=50")),
    );
    report.check(
        "and after the six refusals the two masks are as they were",
        "Mask 1 subtract 4 points, opacity 0.5, feather 8, expansion -4, inverted; \
         Mask 2 add 4 points, opacity 1, feather 0, expansion 0, off",
        masks_of(&viewer),
    );

    run(&viewer, &format!("mask.delete?layer={cel}&mask=0"));
    report.check(
        "Delete takes one mask off and leaves the other",
        "Mask 2 add 4 points, opacity 1, feather 0, expansion 0, off",
        masks_of(&viewer),
    );
    run(&viewer, "edit.undo");
    report.check(
        "and Undo brings it back as it was, settings and all",
        "Mask 1 subtract 4 points, opacity 0.5, feather 8, expansion -4, inverted; \
         Mask 2 add 4 points, opacity 1, feather 0, expansion 0, off",
        masks_of(&viewer),
    );

    // D-298: the diamond keys a mask's feather, a value on another frame is a second key,
    // and the frame between draws with the number halfway.
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={cel}&prop=mask:0:feather&frame=0"),
    );
    run(
        &viewer,
        &format!("property.set_base?layer={cel}&prop=mask:0:feather&frame=10&value=28"),
    );
    report.check(
        "D-298: Mask 1's feather keyed 8 at frame 0 and 28 at frame 10 is 18 at frame 5",
        "18.0",
        value_at(&viewer, 5, &cel, "mask:0:feather"),
    );
    report.check(
        "D-298: and the panel's plain feather box is refused while it has keys, with where to set it",
        "Mask 1's feather has keys. Set it on a frame with property.set_base and \
         prop=mask:0:feather.",
        run(&viewer, &format!("mask.set?layer={cel}&mask=0&feather=3")),
    );
    report.check(
        "D-298: a feather key below 0 is refused by the core, with the reason",
        "Mask \"Mask 1\" was given a feather of -1 pixels, which is below 0. Set a value \
         inside the range and send the masks again.",
        run(
            &viewer,
            &format!("property.set_base?layer={cel}&prop=mask:0:feather&frame=10&value=-1"),
        ),
    );

    // D-737: the pen's Enter sends `closed=0`, and two points are a line; the timeline's switch
    // opens a mask (mode None) and closes it (mode Add), one undo step each.
    run(&viewer, "edit.undo");
    run(&viewer, "edit.undo");
    run(
        &viewer,
        &format!("mask.add?closed=0&layer={cel}&points=0,0,0,0,0,0;10,10,0,0,0,0"),
    );
    report.check(
        "D-737: Enter's `closed=0` adds an open mask of mode None, and two points are enough",
        "Mask 1 subtract 4 points, opacity 0.5, feather 8, expansion -4, inverted; \
         Mask 2 add 4 points, opacity 1, feather 0, expansion 0, off; \
         Mask 3 none 2 points, opacity 1, feather 0, expansion 0, open",
        masks_of(&viewer),
    );
    run(&viewer, &format!("mask.set?layer={cel}&mask=0&closed=1"));
    report.check(
        "D-737: the switch opens Mask 1, so its mode is None",
        "Mask 1 none 4 points",
        masks_of(&viewer)
            .split(", opacity")
            .next()
            .unwrap_or_default()
            .to_string(),
    );
    report.check(
        "D-737: a mode on the open mask is refused, with the reason",
        "Mask \"Mask 1\" is open, so its mode is None: it encloses nothing to add or take \
         away. Close the mask first, then set its mode.",
        run(&viewer, &format!("mask.set?layer={cel}&mask=0&mode=add")),
    );
    run(&viewer, &format!("mask.set?layer={cel}&mask=0&closed=1"));
    report.check(
        "D-737: the switch again closes it, mode Add",
        "Mask 1 add 4 points",
        masks_of(&viewer)
            .split(", opacity")
            .next()
            .unwrap_or_default()
            .to_string(),
    );

    write_artifact(
        &report,
        "verification/B-24c_panel_table.md",
        "B-24c: masks in the window",
        MASK_PANEL_INTRO,
        MASK_PANEL_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// B-24d: the stopwatch on a mask's path, and a point moved on a path that is already
/// moving. The core's half is `verification/B-24d_mask_key_table.md`; this is what the
/// window does with it.
#[test]
fn a_mask_path_is_keyed_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let cel = shown_layer(&viewer, "Cel")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    // The mask's keys as the page reads them off the document, to draw its row's marks and
    // to choose between a hollow diamond and a filled one.
    let keys_of = |viewer: &Mutex<Viewer>| {
        let layer = shown_layer(viewer, "Cel");
        let keys = layer["masks"][0]["path"]["keyframes"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        if keys.is_empty() {
            return "none".to_string();
        }
        keys.iter()
            .map(|k| {
                format!(
                    "frame {} {} at x {}",
                    k["frame"],
                    k["interp"].as_str().unwrap_or("(no interp)"),
                    k["value"]["points"][1]["point"][0]
                )
            })
            .collect::<Vec<_>>()
            .join("; ")
    };
    // And the shape the page draws points on at one frame, which is the renderer's answer
    // and not the base: the first point's x is enough to read the movement by.
    let shape_at = |viewer: &Mutex<Viewer>, frame: i32| {
        let body = boxes(viewer, frame, None).into_body();
        let answer: serde_json::Value =
            serde_json::from_slice(&body).expect("the boxes answer is JSON");
        answer["values"][&cel]["masks"][0][1][0].to_string()
    };

    let square = "0,0,0,0,0,0;100,0,0,0,0,0;100,100,0,0,0,0;0,100,0,0,0,0";
    run(&viewer, &format!("mask.add?layer={cel}&points={square}"));
    report.check(
        "a mask drawn has no keys, so its diamond is hollow and its row has no marks",
        "none",
        keys_of(&viewer),
    );
    run(
        &viewer,
        &format!("mask.add_remove_key?layer={cel}&mask=0&frame=0"),
    );
    report.check(
        "the stopwatch at frame 0 sets the path moving, from the shape it has now",
        "frame 0 linear at x 100",
        keys_of(&viewer),
    );
    run(
        &viewer,
        &format!(
            "mask.set_path?layer={cel}&mask=0&frame=4&points=\
             0,0,0,0,0,0;300,0,0,0,0,0;300,100,0,0,0,0;0,100,0,0,0,0"
        ),
    );
    report.check(
        "a point moved at frame 4 makes a key there rather than moving the whole path",
        "frame 0 linear at x 100; frame 4 linear at x 300",
        keys_of(&viewer),
    );
    report.check(
        "so the page draws the shape half way across at frame 2, and the ends at the keys",
        "100.0 then 200.0 then 300.0",
        format!(
            "{} then {} then {}",
            shape_at(&viewer, 0),
            shape_at(&viewer, 2),
            shape_at(&viewer, 4)
        ),
    );
    run(
        &viewer,
        &format!(
            "mask.set_path?layer={cel}&mask=0&frame=4&points=\
             0,0,0,0,0,0;200,0,0,0,0,0;200,100,0,0,0,0;0,100,0,0,0,0"
        ),
    );
    report.check(
        "moving a point again on a frame that has a key writes to that key, not a new one",
        "frame 0 linear at x 100; frame 4 linear at x 200",
        keys_of(&viewer),
    );
    run(&viewer, "edit.undo");
    report.check(
        "and Undo puts that key back the way it was",
        "frame 0 linear at x 100; frame 4 linear at x 300",
        keys_of(&viewer),
    );
    run(
        &viewer,
        &format!("mask.add_remove_key?layer={cel}&mask=0&frame=4"),
    );
    report.check(
        "a filled diamond takes its key off",
        "frame 0 linear at x 100",
        keys_of(&viewer),
    );
    run(
        &viewer,
        &format!("mask.add_remove_key?layer={cel}&mask=0&frame=0"),
    );
    report.check(
        "and the last key off leaves its own shape behind, so the mask does not jump",
        "none, at x 100.0",
        format!("{}, at x {}", keys_of(&viewer), shape_at(&viewer, 3)),
    );
    report.check(
        "a mask that is not there is refused, and the window says how many there are",
        "Which mask? \"Cel\" has one.",
        run(
            &viewer,
            &format!("mask.add_remove_key?layer={cel}&mask=7&frame=0"),
        ),
    );

    write_artifact(
        &report,
        "verification/B-24d_panel_table.md",
        "B-24d: a mask's path set moving, from the window",
        MASK_KEY_PANEL_INTRO,
        MASK_KEY_PANEL_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// B-24h: a point added to a path or taken off it, from the window, on a path that moves.
/// The core's half is `verification/B-24h_path_point_table.md`; this is what the window
/// does with it, and what it says when the request cannot be granted.
#[test]
fn a_path_gains_and_loses_a_point_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let cel = shown_layer(&viewer, "Cel")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    // How many points the base holds and how many each key holds. D-77 needs these equal,
    // and keeping them equal is the whole of what D-79 asks the window for.
    let counts = |viewer: &Mutex<Viewer>| {
        let layer = shown_layer(viewer, "Cel");
        let held = |v: &serde_json::Value| v["points"].as_array().map_or(0, |p| p.len());
        let keys: Vec<String> = layer["masks"][0]["path"]["keyframes"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(|k| format!("frame {} holds {}", k["frame"], held(&k["value"])))
            .collect();
        match keys.is_empty() {
            true => format!("base {}", held(&layer["masks"][0]["path"]["base"])),
            false => format!(
                "base {}; {}",
                held(&layer["masks"][0]["path"]["base"]),
                keys.join("; ")
            ),
        }
    };
    // The outline the renderer actually drew at a frame, point by point: the same answer the
    // boxes come from, which is what the page draws its points on.
    let shape_at = |viewer: &Mutex<Viewer>, frame: i32| {
        let body = boxes(viewer, frame, None).into_body();
        let answer: serde_json::Value =
            serde_json::from_slice(&body).expect("the boxes answer is JSON");
        answer["values"][&cel]["masks"][0]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(|p| format!("{},{}", p[0], p[1]))
            .collect::<Vec<_>>()
    };

    let square = "0,0,0,0,0,0;100,0,0,0,0,0;100,100,0,0,0,0;0,100,0,0,0,0";
    run(&viewer, &format!("mask.add?layer={cel}&points={square}"));
    report.check("a square drawn has four points", "base 4", counts(&viewer));
    run(
        &viewer,
        &format!("path.add_point?layer={cel}&prop=mask:0&at=1&t=0.5"),
    );
    report.check(
        "half way along the second segment, a fifth point appears there",
        "base 5, at 0.0,0.0 100.0,0.0 100.0,50.0 100.0,100.0 0.0,100.0",
        format!("{}, at {}", counts(&viewer), shape_at(&viewer, 0).join(" ")),
    );
    report.check(
        "and Undo says what it would take back",
        "Add a point to Mask 1",
        held(&viewer)
            .document
            .undo_labels()
            .last()
            .cloned()
            .unwrap_or_default(),
    );
    run(&viewer, "edit.undo");
    report.check(
        "one Undo takes the point off again",
        "base 4",
        counts(&viewer),
    );

    // The same, on a path that is already moving: the reason D-79 exists.
    run(
        &viewer,
        &format!("mask.add_remove_key?layer={cel}&mask=0&frame=0"),
    );
    run(
        &viewer,
        &format!(
            "mask.set_path?layer={cel}&mask=0&frame=4&points=\
             0,0,0,0,0,0;300,0,0,0,0,0;300,100,0,0,0,0;0,100,0,0,0,0"
        ),
    );
    let between = shape_at(&viewer, 2).join(" ");
    report.check(
        "a path with two keys, drawn half way between them at frame 2",
        "base 4; frame 0 holds 4; frame 4 holds 4, at 0.0,0.0 200.0,0.0 200.0,100.0 0.0,100.0",
        format!("{}, at {between}", counts(&viewer)),
    );
    run(
        &viewer,
        &format!("path.add_point?layer={cel}&prop=mask:0&at=1&t=0.5"),
    );
    report.check(
        "a point added goes on the base and on every key, which is what D-77 needs",
        "base 5; frame 0 holds 5; frame 4 holds 5",
        counts(&viewer),
    );
    let mut without_it = shape_at(&viewer, 2);
    without_it.remove(2);
    report.check(
        "and the shape between the keys does not move: the other four points are where they were",
        &between,
        without_it.join(" "),
    );
    report.check(
        "the new point is on the curve between the keys, half way along that segment",
        "200.0,50.0",
        shape_at(&viewer, 2)[2].clone(),
    );
    run(
        &viewer,
        &format!("path.remove_point?layer={cel}&prop=mask:0&at=2"),
    );
    report.check(
        "taken off again, it comes off the base and off every key together",
        "base 4; frame 0 holds 4; frame 4 holds 4",
        counts(&viewer),
    );
    report.check(
        "and Undo says that too",
        "Remove a point from Mask 1",
        held(&viewer)
            .document
            .undo_labels()
            .last()
            .cloned()
            .unwrap_or_default(),
    );
    report.check(
        "a point the path has not got is refused, and the window says how many it has",
        "This path has 4 points, so it has no point 9.",
        run(
            &viewer,
            &format!("path.add_point?layer={cel}&prop=mask:0&at=9"),
        ),
    );
    report.check(
        "a place that is not along a segment is refused",
        "\"1.5\" is not a place along a segment. It is a fraction, more than 0 and less than 1.",
        run(
            &viewer,
            &format!("path.add_point?layer={cel}&prop=mask:0&at=1&t=1.5"),
        ),
    );
    report.check(
        "a request that names no path is refused",
        "Which path? Say prop=mask:0 for the layer's first mask, or prop=shape:0 for its \
         first shape.",
        run(&viewer, &format!("path.add_point?layer={cel}&at=1")),
    );
    report.check(
        "and a path taken below three points is refused, by the rule this build already had",
        "A mask needs at least three points, and this one has 2. Add points until the shape \
         closes on an area.",
        {
            run(
                &viewer,
                &format!("path.remove_point?layer={cel}&prop=mask:0&at=0"),
            );
            run(
                &viewer,
                &format!("path.remove_point?layer={cel}&prop=mask:0&at=0"),
            )
        },
    );

    // B-24i, D-80: several points chosen at once. Delete takes all of them off in one
    // request, because one press has to be one thing to undo, and a list that names a point
    // the path has not got is refused whole rather than half granted.
    for at in [0, 1, 2] {
        run(
            &viewer,
            &format!("path.add_point?layer={cel}&prop=mask:0&at={at}&t=0.5"),
        );
    }
    report.check(
        "three points put back on, so the path has six",
        "base 6; frame 0 holds 6; frame 4 holds 6",
        counts(&viewer),
    );
    report.check(
        "three of them named at once come off together, down to the three a mask needs",
        "base 3; frame 0 holds 3; frame 4 holds 3",
        {
            run(
                &viewer,
                &format!("path.remove_point?layer={cel}&prop=mask:0&at=1,3,5"),
            );
            counts(&viewer)
        },
    );
    report.check(
        "and Undo offers to take back all three of them, once",
        "Remove 3 points from Mask 1",
        held(&viewer)
            .document
            .undo_labels()
            .last()
            .cloned()
            .unwrap_or_default(),
    );
    run(&viewer, "edit.undo");
    report.check(
        "which one Undo does, to the base and to both keys",
        "base 6; frame 0 holds 6; frame 4 holds 6",
        counts(&viewer),
    );
    report.check(
        "a list naming a point the path has not got is refused whole: nothing comes off",
        "This path has 6 points, so it has no point 7. | base 6; frame 0 holds 6; frame 4 holds 6",
        format!(
            "{} | {}",
            run(
                &viewer,
                &format!("path.remove_point?layer={cel}&prop=mask:0&at=1,7")
            ),
            counts(&viewer)
        ),
    );
    report.check(
        "and a list given to the other command is refused, since each added point is a place \
         along its own segment",
        "A point is added one at a time, because each one is a place along its own segment. \
         Say at= one point.",
        run(
            &viewer,
            &format!("path.add_point?layer={cel}&prop=mask:0&at=1,2&t=0.5"),
        ),
    );

    // B-24k, D-81: the free transform box. Every step of one drag of it sends the whole path,
    // each point and each handle through the same turn and scale; that arithmetic is the
    // page's and its sheet's to judge. What these rows hold is the bargain underneath: a
    // whole drag of the box on a keyed path is one thing to undo, lands at its frame as the
    // last step sent it, and one Undo puts the shape back.
    let was: Vec<(f64, f64)> = shape_at(&viewer, 4)
        .iter()
        .map(|p| {
            let (x, y) = p.split_once(',').expect("x,y");
            (x.parse().expect("x"), y.parse().expect("y"))
        })
        .collect();
    // A quarter turn about the corner at nothing, and bigger by `by`; the + 0.0 keeps a
    // turned nothing from printing as -0.0.
    let turned = |by: f64| {
        was.iter()
            .map(|&(x, y)| (-y * by + 0.0, x * by + 0.0))
            .collect::<Vec<_>>()
    };
    let before = held(&viewer).document.undo_labels().len();
    for by in [1.5, 2.0] {
        let points = turned(by)
            .iter()
            .map(|(x, y)| format!("{x},{y},0,{},0,{}", 10.0 * by, -10.0 * by))
            .collect::<Vec<_>>()
            .join(";");
        run(
            &viewer,
            &format!("mask.set_path?layer={cel}&mask=0&frame=4&drag=1&points={points}"),
        );
    }
    run(&viewer, "property.drag_end");
    report.check(
        "a box turned a quarter and doubled in two steps of one drag is one thing to undo",
        "1",
        (held(&viewer).document.undo_labels().len() - before).to_string(),
    );
    report.check(
        "and the key at frame 4 holds the last step: every point turned and twice as far out",
        turned(2.0)
            .iter()
            .map(|(x, y)| format!("{x:?},{y:?}"))
            .collect::<Vec<_>>()
            .join(" "),
        shape_at(&viewer, 4).join(" "),
    );
    run(&viewer, "edit.undo");
    report.check(
        "one Undo puts the shape back as it was before the box",
        was.iter()
            .map(|(x, y)| format!("{x:?},{y:?}"))
            .collect::<Vec<_>>()
            .join(" "),
        shape_at(&viewer, 4).join(" "),
    );

    // D-81a: a box round two masks. Every step of its drag sends both paths, and the drag is
    // still one thing to undo, with both masks landing and both coming back together.
    run(
        &viewer,
        &format!("mask.add?layer={cel}&points=0,0,0,0,0,0;10,0,0,0,0,0;10,10,0,0,0,0"),
    );
    let second = |viewer: &Mutex<Viewer>| {
        let body = boxes(viewer, 4, None).into_body();
        let answer: serde_json::Value =
            serde_json::from_slice(&body).expect("the boxes answer is JSON");
        answer["values"][&cel]["masks"][1]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(|p| format!("{},{}", p[0], p[1]))
            .collect::<Vec<_>>()
            .join(" ")
    };
    let before = held(&viewer).document.undo_labels().len();
    for by in [5, 20] {
        run(
            &viewer,
            &format!(
                "mask.set_path?layer={cel}&mask=1&frame=4&drag=1&points=\
                 {by},0,0,0,0,0;{},0,0,0,0,0;{},10,0,0,0,0",
                by + 10,
                by + 10
            ),
        );
        let moved = was
            .iter()
            .map(|(x, y)| format!("{},{y},0,0,0,0", x + by as f64))
            .collect::<Vec<_>>()
            .join(";");
        run(
            &viewer,
            &format!("mask.set_path?layer={cel}&mask=0&frame=4&drag=1&points={moved}"),
        );
    }
    run(&viewer, "property.drag_end");
    report.check(
        "a box round two masks, dragged 20 pixels in two steps, is one thing to undo",
        "1",
        (held(&viewer).document.undo_labels().len() - before).to_string(),
    );
    report.check(
        "and both masks moved the whole 20: the first mask's first point, and the second mask",
        format!(
            "{:?},{:?} | 20.0,0.0 30.0,0.0 30.0,10.0",
            was[0].0 + 20.0,
            was[0].1
        ),
        format!("{} | {}", shape_at(&viewer, 4)[0], second(&viewer)),
    );
    run(&viewer, "edit.undo");
    report.check(
        "one Undo puts both back",
        format!("{:?},{:?} | 0.0,0.0 10.0,0.0 10.0,10.0", was[0].0, was[0].1),
        format!("{} | {}", shape_at(&viewer, 4)[0], second(&viewer)),
    );

    write_artifact(
        &report,
        "verification/B-24h_panel_table.md",
        "B-24h: a point added to a path, from the window",
        PATH_POINT_PANEL_INTRO,
        PATH_POINT_PANEL_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// B-24e: a path key moved along its row, eased by F9's rule and held, for a mask and a
/// shape alike, through `mask.set` and `shape.set` naming the key by its frame.
#[test]
fn a_path_key_is_moved_and_eased_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let cel = shown_layer(&viewer, "Cel")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    // The keys as the page reads them: frame, curve, and the second point's x.
    let keys_of = |path: &serde_json::Value| {
        let keys = path["keyframes"].as_array().cloned().unwrap_or_default();
        keys.iter()
            .map(|k| {
                format!(
                    "frame {} {}{} at x {}",
                    k["frame"],
                    k["interp"].as_str().unwrap_or("(no interp)"),
                    match k["ease"].as_array() {
                        Some(e) => format!(
                            " {}",
                            e.iter()
                                .map(|n| format!("{:.3}", n.as_f64().unwrap_or(f64::NAN)))
                                .collect::<Vec<_>>()
                                .join(",")
                        ),
                        None => String::new(),
                    },
                    k["value"]["points"][1]["point"][0]
                )
            })
            .collect::<Vec<_>>()
            .join("; ")
    };
    let mask_keys =
        |viewer: &Mutex<Viewer>| keys_of(&shown_layer(viewer, "Cel")["masks"][0]["path"]);
    // The outline the renderer drew at a frame: the second point's x.
    let shape_at = |viewer: &Mutex<Viewer>, frame: i32| {
        let body = boxes(viewer, frame, None).into_body();
        let answer: serde_json::Value =
            serde_json::from_slice(&body).expect("the boxes answer is JSON");
        answer["values"][&cel]["masks"][0][1][0].to_string()
    };
    let at = |x: i32| format!("0,0,0,0,0,0;{x},0,0,0,0,0;{x},100,0,0,0,0;0,100,0,0,0,0");

    run(&viewer, &format!("mask.add?layer={cel}&points={}", at(100)));
    run(
        &viewer,
        &format!("mask.add_remove_key?layer={cel}&mask=0&frame=0"),
    );
    run(
        &viewer,
        &format!(
            "mask.set_path?layer={cel}&mask=0&frame=8&points={}",
            at(300)
        ),
    );
    report.check(
        "a mask keyed at frame 0 and frame 8, as B-24d leaves it",
        "frame 0 linear at x 100; frame 8 linear at x 300",
        mask_keys(&viewer),
    );
    run(&viewer, &format!("mask.set?layer={cel}&mask=0&key=8&to=4"));
    report.check(
        "the key at 8 dragged along its row to 4 lands there, holding the same shape",
        "frame 0 linear at x 100; frame 4 linear at x 300",
        mask_keys(&viewer),
    );
    report.check(
        "so the path now arrives at frame 4, and is half way at frame 2",
        "200.0 then 300.0",
        format!("{} then {}", shape_at(&viewer, 2), shape_at(&viewer, 4)),
    );
    run(&viewer, "edit.undo");
    report.check(
        "one Undo puts the key back at 8",
        "frame 0 linear at x 100; frame 8 linear at x 300",
        mask_keys(&viewer),
    );
    run(&viewer, "edit.redo");
    run(
        &viewer,
        &format!(
            "mask.set_path?layer={cel}&mask=0&frame=8&points={}",
            at(100)
        ),
    );
    report.check(
        "and Redo moves it again; a third key at 8 takes the path back",
        "frame 0 linear at x 100; frame 4 linear at x 300; frame 8 linear at x 100",
        mask_keys(&viewer),
    );
    run(
        &viewer,
        &format!("mask.set?layer={cel}&mask=0&key=4&ease=both"),
    );
    report.check(
        "F9 on the middle key eases both sides of it: the segment it is reached by ends \
         slowly, the one it leaves by starts slowly, and each keeps its other half straight",
        "frame 0 ease 0.333,0.333,0.667,1.000 at x 100; frame 4 ease 0.333,0.000,0.667,0.667 \
         at x 300; frame 8 linear at x 100",
        mask_keys(&viewer),
    );
    report.check(
        "so half way through each segment's time the path is three quarters of the way to \
         the eased key, worked out from document 20's curve by hand: 100 + 200 x 0.625 and \
         300 - 200 x 0.375",
        "225.0 and 225.0",
        format!("{} and {}", shape_at(&viewer, 2), shape_at(&viewer, 6)),
    );
    run(
        &viewer,
        &format!("mask.set?layer={cel}&mask=0&key=0&interp=linear"),
    );
    run(
        &viewer,
        &format!("mask.set?layer={cel}&mask=0&key=8&ease=in"),
    );
    report.check(
        "Shift+F9 eases only the side a key is reached by; linear, which Ctrl+Alt+G sends to \
         release a held key, straightened the first segment again",
        "frame 0 linear at x 100; frame 4 ease 0.333,0.000,0.667,1.000 at x 300; frame 8 \
         linear at x 100",
        mask_keys(&viewer),
    );
    run(
        &viewer,
        &format!("mask.set?layer={cel}&mask=0&key=0&interp=hold"),
    );
    report.check(
        "Ctrl+Alt+G holds a key: the path stays as it is until the next key",
        "frame 0 hold at x 100; 100.0 at frame 3, 300.0 at frame 4",
        format!(
            "{}; {} at frame 3, {} at frame 4",
            mask_keys(&viewer).split("; ").next().unwrap_or(""),
            shape_at(&viewer, 3),
            shape_at(&viewer, 4)
        ),
    );
    run(&viewer, &format!("mask.set?layer={cel}&mask=0&key=8&to=4"));
    report.check(
        "a key dropped on a frame that has one takes its place, as After Effects' does",
        "frame 0 hold at x 100; frame 4 linear at x 100",
        mask_keys(&viewer),
    );
    report.check(
        "a key that is not there is refused, in words",
        "This path has no key at frame 9.",
        run(&viewer, &format!("mask.set?layer={cel}&mask=0&key=9&to=2")),
    );
    report.check(
        "and so is an ease that is not one of F9's three",
        "F9 eases both, in or out. Not \"sideways\".",
        run(
            &viewer,
            &format!("mask.set?layer={cel}&mask=0&key=4&ease=sideways"),
        ),
    );

    // A shape's path is a mask's path, and `shape.set` takes the same words.
    run(&viewer, "layer.add_shape");
    let shape = shown_layer(&viewer, "Shape Layer 1")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    run(
        &viewer,
        &format!("shape.add?layer={shape}&points={}", at(100)),
    );
    run(
        &viewer,
        &format!("shape.add_remove_key?layer={shape}&shape=0&frame=0"),
    );
    run(
        &viewer,
        &format!(
            "shape.set_path?layer={shape}&shape=0&frame=6&points={}",
            at(300)
        ),
    );
    run(
        &viewer,
        &format!("shape.set?layer={shape}&shape=0&key=6&to=3"),
    );
    run(
        &viewer,
        &format!("shape.set?layer={shape}&shape=0&key=3&ease=in"),
    );
    report.check(
        "a shape's key is dragged and eased the same way",
        "frame 0 ease 0.333,0.333,0.667,1.000 at x 100; frame 3 linear at x 300",
        keys_of(&shown_layer(&viewer, "Shape Layer 1")["shapes"][0]["path"]),
    );

    write_artifact(
        &report,
        "verification/B-24e_panel_table.md",
        "B-24e: a path key moved, eased and held, from the window",
        PATH_KEY_PANEL_INTRO,
        PATH_KEY_PANEL_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// B-24f: a path's keys named `mask:<n>` and `shape:<n>` in the requests a property's keys
/// already go through - chosen with other keys, moved, deleted, eased, pasted - and drawn in
/// the graph as the distance the path has travelled.
#[test]
fn a_path_key_goes_where_a_property_key_goes() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let cel = shown_layer(&viewer, "Cel")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    // A path's keys as the page reads them: frame, curve, and the second point's x.
    let keys_of = |path: &serde_json::Value| {
        let keys = path["keyframes"].as_array().cloned().unwrap_or_default();
        keys.iter()
            .map(|k| {
                format!(
                    "frame {} {}{} at x {}",
                    k["frame"],
                    k["interp"].as_str().unwrap_or("(no interp)"),
                    match k["ease"].as_array() {
                        Some(e) => format!(
                            " {}",
                            e.iter()
                                .map(|n| format!("{:.3}", n.as_f64().unwrap_or(f64::NAN)))
                                .collect::<Vec<_>>()
                                .join(",")
                        ),
                        None => String::new(),
                    },
                    k["value"]["points"][1]["point"][0]
                )
            })
            .collect::<Vec<_>>()
            .join("; ")
    };
    let mask_keys =
        |viewer: &Mutex<Viewer>| keys_of(&shown_layer(viewer, "Cel")["masks"][0]["path"]);
    let rotation_keys = |viewer: &Mutex<Viewer>| {
        shown_layer(viewer, "Cel")["transform"]["rotation"]["keyframes"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(|k| format!("frame {}", k["frame"]))
            .collect::<Vec<_>>()
            .join("; ")
    };
    // What the graph is given at a few frames, to three places.
    let graph = |viewer: &Mutex<Viewer>, frames: &[i32]| {
        let query = format!("layer={cel}&prop=mask:0&from=0&to=12");
        let body = curve(viewer, Some(&query)).into_body();
        let answer: serde_json::Value =
            serde_json::from_slice(&body).expect("the curve answer is JSON");
        frames
            .iter()
            .map(|f| {
                format!(
                    "{:.3}",
                    answer["samples"][*f as usize][0]
                        .as_f64()
                        .unwrap_or(f64::NAN)
                )
            })
            .collect::<Vec<_>>()
            .join(", ")
    };
    let at = |x: i32| format!("0,0,0,0,0,0;{x},0,0,0,0,0;{x},100,0,0,0,0;0,100,0,0,0,0");

    run(&viewer, &format!("mask.add?layer={cel}&points={}", at(100)));
    run(
        &viewer,
        &format!("mask.add_remove_key?layer={cel}&mask=0&frame=0"),
    );
    run(
        &viewer,
        &format!(
            "mask.set_path?layer={cel}&mask=0&frame=8&points={}",
            at(300)
        ),
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={cel}&prop=rotation&frame=0"),
    );
    report.check(
        "a mask keyed at frame 0 and frame 8, and the layer's rotation keyed at frame 0",
        "frame 0 linear at x 100; frame 8 linear at x 300 / frame 0",
        format!("{} / {}", mask_keys(&viewer), rotation_keys(&viewer)),
    );
    report.check(
        "the graph of the path is how far its points have gone on average, in pixels: two of \
         its four points move 200 pixels, so 100 in all, half of it by frame 4, none before \
         the first key and all of it after the last",
        "0.000, 50.000, 100.000, 100.000",
        graph(&viewer, &[0, 4, 8, 12]),
    );

    let mask_key = |frame: i32| format!("key={cel}%7Cmask:0%7C{frame}");
    run(
        &viewer,
        &format!(
            "keyframe.move?by=2&{}&key={cel}%7Crotation%7C0",
            mask_key(8)
        ),
    );
    report.check(
        "the mask's key at 8 chosen with the rotation's key at 0, and both dragged two frames \
         later, as one move",
        "frame 0 linear at x 100; frame 10 linear at x 300 / frame 2",
        format!("{} / {}", mask_keys(&viewer), rotation_keys(&viewer)),
    );
    run(&viewer, "edit.undo");
    report.check(
        "one Undo puts both back",
        "frame 0 linear at x 100; frame 8 linear at x 300 / frame 0",
        format!("{} / {}", mask_keys(&viewer), rotation_keys(&viewer)),
    );
    let answer = run(&viewer, &format!("keyframe.move?by=8&{}", mask_key(0)));
    report.check(
        "a path key moved onto another key of the same path is refused, as a property's is, \
         and nothing moves",
        "refused: true; frame 0 linear at x 100; frame 8 linear at x 300",
        format!(
            "refused: {}; {}",
            answer.contains("a second key at frame 8"),
            mask_keys(&viewer)
        ),
    );
    report.check(
        "a key that is not there is refused, in words",
        "This path has no key at frame 5.",
        run(&viewer, &format!("keyframe.move?by=1&{}", mask_key(5))),
    );
    run(&viewer, &format!("keyframe.add_remove?{}", mask_key(8)));
    report.check(
        "Delete on a chosen path key takes it off",
        "frame 0 linear at x 100",
        mask_keys(&viewer),
    );
    run(&viewer, "edit.undo");
    run(
        &viewer,
        &format!("keyframe.add_remove?{}&{}", mask_key(0), mask_key(8)),
    );
    report.check(
        "Delete on both keys stops the path, which keeps the shape the first of them had",
        "no keys; x 100",
        format!(
            "{}; x {}",
            match mask_keys(&viewer).as_str() {
                "" => "no keys".to_string(),
                other => other.to_string(),
            },
            shown_layer(&viewer, "Cel")["masks"][0]["path"]["base"]["points"][1]["point"][0]
        ),
    );
    run(&viewer, "edit.undo");
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={cel}&prop=mask:0&frame=4"),
    );
    report.check(
        "the diamond, sent with the path's name, sets a key holding the shape the path has \
         there: half way",
        "frame 0 linear at x 100; frame 4 linear at x 200; frame 8 linear at x 300",
        mask_keys(&viewer),
    );
    run(&viewer, "edit.undo");

    // F9 on the first key, as the page sends it: the side it leaves by eased, the other
    // half of the segment's curve left straight.
    run(
        &viewer,
        &format!(
            "keyframe.set_interp?layer={cel}&prop=mask:0&frame=0&mode=ease\
             &curve=0.333333,0,0.666667,0.666667"
        ),
    );
    report.check(
        "F9 on a path key reaches it through the request it sends for a property's key",
        "frame 0 ease 0.333,0.000,0.667,0.667 at x 100; frame 8 linear at x 300",
        mask_keys(&viewer),
    );
    report.check(
        "and the graph follows the ease: on this curve time runs straight, so half way the \
         path has gone 2 x 0.5 x 0.25 + 0.125 = 0.375 of its 100 pixels, worked out by hand",
        "0.000, 37.500, 100.000",
        graph(&viewer, &[0, 4, 8]),
    );
    run(
        &viewer,
        &format!("keyframe.set_interp?layer={cel}&prop=mask:0&frame=0&mode=hold"),
    );
    report.check(
        "Ctrl+Alt+G holds it: the graph stays flat until the next key",
        "frame 0 hold at x 100; 0.000, 0.000, 100.000",
        format!(
            "{}; {}",
            mask_keys(&viewer).split("; ").next().unwrap_or(""),
            graph(&viewer, &[0, 7, 8])
        ),
    );
    // Letting the hold go is the page's to remember: it sends back the ease the key had.
    run(
        &viewer,
        &format!(
            "keyframe.set_interp?layer={cel}&prop=mask:0&frame=0&mode=ease\
             &curve=0.333333,0,0.666667,0.666667"
        ),
    );

    // Ctrl+V of the key at 8 onto frame 12, as the page pastes a path's key: its points by
    // the pen's own request at that frame, then its curve, one entry to undo.
    run(
        &viewer,
        &format!(
            "mask.set_path?layer={cel}&frame=12&drag=1&mask=0&points={}",
            at(300)
        ),
    );
    run(
        &viewer,
        &format!("keyframe.set_interp?layer={cel}&frame=12&drag=1&prop=mask:0&mode=linear"),
    );
    run(&viewer, "property.drag_end");
    report.check(
        "a path key pasted at frame 12 holds the copied shape",
        "frame 0 ease 0.333,0.000,0.667,0.667 at x 100; frame 8 linear at x 300; frame 12 \
         linear at x 300",
        mask_keys(&viewer),
    );
    run(&viewer, "edit.undo");
    report.check(
        "and one Undo takes the paste away",
        "frame 0 ease 0.333,0.000,0.667,0.667 at x 100; frame 8 linear at x 300",
        mask_keys(&viewer),
    );
    report.check(
        "a path that is not there is refused, in words",
        "Cel has no mask 3.",
        run(
            &viewer,
            &format!("keyframe.set_interp?layer={cel}&prop=mask:3&frame=0&mode=hold"),
        ),
    );

    // A shape's path is named `shape:<n>` and goes the same way.
    run(&viewer, "layer.add_shape");
    let shape = shown_layer(&viewer, "Shape Layer 1")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    run(
        &viewer,
        &format!("shape.add?layer={shape}&points={}", at(100)),
    );
    run(
        &viewer,
        &format!("shape.add_remove_key?layer={shape}&shape=0&frame=0"),
    );
    run(
        &viewer,
        &format!(
            "shape.set_path?layer={shape}&shape=0&frame=6&points={}",
            at(300)
        ),
    );
    run(
        &viewer,
        &format!("keyframe.move?by=-2&key={shape}%7Cshape:0%7C6"),
    );
    run(
        &viewer,
        &format!("keyframe.set_interp?layer={shape}&prop=shape:0&frame=4&mode=hold"),
    );
    report.check(
        "a shape's key is moved and held the same way",
        "frame 0 linear at x 100; frame 4 hold at x 300",
        keys_of(&shown_layer(&viewer, "Shape Layer 1")["shapes"][0]["path"]),
    );

    write_artifact(
        &report,
        "verification/B-24f_panel_table.md",
        "B-24f: a path key chosen, moved, deleted, pasted and graphed like a property's",
        PATH_KEY_GRAPH_INTRO,
        PATH_KEY_GRAPH_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// B-150c: the owner's "Fix key diamonds". D-216 stretches a layer's keys with it, so a key
/// the timeline shows is where it plays, and a key the timeline names - added at the
/// playhead, dragged, eased, taken off - is named by that frame and stored where it is read.
#[test]
fn a_stretched_layers_keys_are_shown_and_set_where_they_play() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/unknown_effect_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let l = "layer-cel";
    let page = include_str!("../../ui/index.html");
    let present = |yes: bool| if yes { "present" } else { "absent" };
    run(&viewer, &format!("layer.shift?layer={l}&in=4"));

    // ---- at 100%, nothing changes -----------------------------------------------------------
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={l}&prop=position&frame=12"),
    );
    run(
        &viewer,
        &format!("keyframe.move?layer={l}&prop=position&from=12&to=20"),
    );
    report.check(
        "at 100% a key set with the playhead on frame 12 and dragged to 20 is stored on 20, \
         as before",
        "[0,0]@20 linear",
        keys(&viewer, l, "position"),
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={l}&prop=position&frame=20"),
    );

    // ---- at 200% from frame 4 ---------------------------------------------------------------
    run(
        &viewer,
        &format!("layer.set_time_stretch?layer={l}&value=200"),
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={l}&prop=position&frame=12"),
    );
    report.check(
        "at 200% a key set with the playhead on frame 12 is stored at 8, which plays on 12: \
         4 + (8 - 4) x 200 / 100",
        "[0,0]@8 linear",
        keys(&viewer, l, "position"),
    );
    report.check(
        "the timeline draws every key where it plays, by that rule",
        "present",
        present(
            page.contains("const keysOf = (layer, prop) => shownKeys(layer, propOf(layer, prop).keyframes || []);")
                && page.contains("const playedAt = (layer, k) => layer.in_frame + (k - layer.in_frame) * stretchOf(layer) / 100;"),
        ),
    );
    report.check(
        "and a mask's or a shape's path keys too, on their own rows and on their diamond",
        2,
        page.matches("const keys = shownKeys(layer, maskKeys(mask));")
            .count(),
    );
    run(
        &viewer,
        &format!("keyframe.set_interp?layer={l}&prop=position&frame=12&mode=hold"),
    );
    report.check(
        "the key named by the frame it is shown on, 12, is the one held",
        "[0,0]@8 hold",
        keys(&viewer, l, "position"),
    );
    run(
        &viewer,
        &format!("keyframe.move?layer={l}&prop=position&from=12&to=20"),
    );
    report.check(
        "dragged from 12 to 20 on the timeline, it is stored at 12, which plays on 20",
        "[0,0]@12 hold",
        keys(&viewer, l, "position"),
    );
    run(
        &viewer,
        &format!("keyframe.move?by=4&key={l}%7Cposition%7C20"),
    );
    report.check(
        "chosen and dragged four frames on, it plays on 24, stored at 14",
        "[0,0]@14 hold",
        keys(&viewer, l, "position"),
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={l}&prop=position&frame=24"),
    );
    report.check(
        "its diamond pressed with the playhead on 24 takes it off",
        "",
        keys(&viewer, l, "position"),
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={l}&prop=opacity&frame=4"),
    );
    run(
        &viewer,
        &format!("property.set_base?layer={l}&prop=opacity&frame=12&value=0"),
    );
    report.check(
        "an opacity keyed on 4 and typed to 0 on frame 12 has keys stored at 4 and 8",
        "1@4 linear, 0@8 linear",
        keys(&viewer, l, "opacity"),
    );
    report.check(
        "the graph draws it as it plays: 1 on 4, 0.5 on 8, 0.25 on 10, 0 on 12",
        "1, 0.5, 0.25, 0",
        {
            let all = plotted(&viewer, &format!("layer={l}&prop=opacity&from=0&to=12"));
            [4, 8, 10, 12].map(|f| format!("{}", all[f])).join(", ")
        },
    );
    let square = "0,0,0,0,0,0;100,0,0,0,0,0;100,100,0,0,0,0;0,100,0,0,0,0";
    run(&viewer, &format!("mask.add?layer={l}&points={square}"));
    run(
        &viewer,
        &format!("mask.add_remove_key?layer={l}&mask=0&frame=12"),
    );
    let mask_frames = |viewer: &Mutex<Viewer>| {
        let answer: serde_json::Value =
            serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
        answer["project"]["compositions"][0]["layers"][0]["masks"][0]["path"]["keyframes"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(|k| k["frame"].to_string())
            .collect::<Vec<_>>()
            .join(", ")
    };
    report.check(
        "a mask's path keyed with the playhead on 12 is stored at 8 too",
        "8",
        mask_frames(&viewer),
    );
    run(
        &viewer,
        &format!("keyframe.move?by=4&key={l}%7Cmask:0%7C12"),
    );
    report.check(
        "and dragged four frames on it plays on 16, stored at 10",
        "10",
        mask_frames(&viewer),
    );

    // ---- at 50% from frame 4 ----------------------------------------------------------------
    run(
        &viewer,
        &format!("layer.set_time_stretch?layer={l}&value=50"),
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={l}&prop=rotation&frame=9"),
    );
    report.check(
        "at 50% a key set with the playhead on frame 9 is stored at 14, which plays on 9: \
         4 + (14 - 4) x 50 / 100",
        "0@14 linear",
        keys(&viewer, l, "rotation"),
    );
    run(
        &viewer,
        &format!("layer.set_time_stretch?layer={l}&value=100"),
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={l}&prop=rotation&frame=15"),
    );
    run(
        &viewer,
        &format!("layer.set_time_stretch?layer={l}&value=50"),
    );
    run(
        &viewer,
        &format!("keyframe.set_interp?layer={l}&prop=rotation&frame=9.5&mode=hold"),
    );
    report.check(
        "a key stored at 15 plays half way between 9 and 10, is shown there, and is held \
         when named 9.5",
        "0@14 linear, 0@15 hold",
        keys(&viewer, l, "rotation"),
    );
    run(
        &viewer,
        &format!("keyframe.move?by=2&key={l}%7Crotation%7C9.5"),
    );
    report.check(
        "dragged two frames on it plays on 11.5, stored at 19",
        "0@14 linear, 0@19 hold",
        keys(&viewer, l, "rotation"),
    );
    report.check(
        "a half-way key chosen on the timeline puts the playhead on a whole frame",
        "present",
        present(page.contains("wanted = Math.round(at);")),
    );

    // ---- back at 100% -----------------------------------------------------------------------
    run(
        &viewer,
        &format!("layer.set_time_stretch?layer={l}&value=100"),
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={l}&prop=scale&frame=12"),
    );
    report.check(
        "set back to 100%, a key set on frame 12 is stored on 12 again",
        "[100,100]@12 linear",
        keys(&viewer, l, "scale"),
    );

    write_artifact(
        &report,
        "verification/B-150c_key_diamonds_table.md",
        "B-150c: a stretched layer's keys shown and set where they play",
        KEY_DIAMONDS_INTRO,
        KEY_DIAMONDS_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// B-25c: D-78's shape layer from the window. The core's half is
/// `verification/B-25b_shape_table.md`; this is what the window sends and what comes back.
#[test]
fn a_shape_layer_is_drawn_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let cel = shown_layer(&viewer, "Cel")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    // The shapes as the page reads them off the document, one clause each.
    let shapes_of = |viewer: &Mutex<Viewer>| {
        let layer = shown_layer(viewer, "Shape Layer 1");
        let shapes = layer["shapes"].as_array().cloned().unwrap_or_default();
        if shapes.is_empty() {
            return "none".to_string();
        }
        shapes
            .iter()
            .map(|s| {
                let paint = |p: &serde_json::Value| {
                    if p.is_null() {
                        "none".to_string()
                    } else {
                        format!(
                            "{} at {}{}",
                            p["color"],
                            p["opacity"],
                            p.get("width_px")
                                .map_or(String::new(), |w| format!(", {w} px"))
                        )
                    }
                };
                format!(
                    "{} {} {} points, fill {}, stroke {}{}",
                    s["name"].as_str().unwrap_or("(no name)"),
                    if s["closed"] == serde_json::Value::Bool(false) {
                        "open"
                    } else {
                        "closed"
                    },
                    s["path"]["base"]["points"]
                        .as_array()
                        .map_or(0, |p| p.len()),
                    paint(&s["fill"]),
                    paint(&s["stroke"]),
                    if s["enabled"] == serde_json::Value::Bool(false) {
                        ", off"
                    } else {
                        ""
                    },
                )
            })
            .collect::<Vec<_>>()
            .join("; ")
    };

    run(&viewer, "layer.add_shape");
    let layer = shown_layer(&viewer, "Shape Layer 1");
    report.check(
        "New shape layer adds an empty shape layer the size of the composition, named Shape Layer 1",
        "shape, 0 shapes",
        format!(
            "{}, {} shapes",
            layer["kind"].as_str().unwrap_or("(no kind)"),
            layer["shapes"].as_array().map_or(0, |s| s.len())
        ),
    );
    let id = layer["id"].as_str().unwrap_or_default().to_string();

    // The rectangle tool, dragged corner to corner on the shape layer.
    run(
        &viewer,
        &format!(
            "shape.add?layer={id}&closed=1&points=100,100,0,0,0,0;300,100,0,0,0,0;\
             300,300,0,0,0,0;100,300,0,0,0,0"
        ),
    );
    report.check(
        "a closed shape drawn comes with a grey fill, as a new solid is grey, and no stroke",
        "Shape 1 closed 4 points, fill [0.2158605307340622,0.2158605307340622,0.2158605307340622] at 1, stroke none",
        shapes_of(&viewer),
    );
    report.check(
        "Undo says what it would take back",
        "Set one shape",
        held(&viewer)
            .document
            .undo_labels()
            .last()
            .cloned()
            .unwrap_or_default(),
    );
    // The pen, two points placed and Enter pressed: an open line.
    run(
        &viewer,
        &format!("shape.add?layer={id}&closed=0&points=0,500,0,0,0,0;800,500,0,0,0,0"),
    );
    report.check(
        "an open shape drawn with the pen comes with a white stroke 4 pixels wide, and no fill",
        "Shape 2 open 2 points, fill none, stroke [1,1,1] at 1, 4 px",
        shapes_of(&viewer)
            .split("; ")
            .nth(1)
            .unwrap_or_default()
            .to_string(),
    );
    report.check(
        "the page is given the open shape's points on the frame, to draw its outline",
        "[800.0,500.0,0.0,0.0,0.0,0.0]",
        {
            let body = boxes(&viewer, 0, None).into_body();
            let answer: serde_json::Value =
                serde_json::from_slice(&body).expect("the boxes answer is JSON");
            answer["values"][&id]["shapes"][1][1].to_string()
        },
    );

    // The panel's own controls, each sending one setting and leaving the rest alone. The
    // picker's #ff0000 arrives linear as 1,0,0.
    run(
        &viewer,
        &format!("shape.set?layer={id}&shape=0&stroke=1,0,0"),
    );
    run(
        &viewer,
        &format!("shape.set?layer={id}&shape=0&stroke_width=10"),
    );
    run(
        &viewer,
        &format!("shape.set?layer={id}&shape=0&stroke_opacity=50"),
    );
    run(
        &viewer,
        &format!("shape.set?layer={id}&shape=0&fill_opacity=25"),
    );
    report.check(
        "Add stroke, its width and its opacity, and the fill's opacity, each on its own",
        "Shape 1 closed 4 points, fill [0.2158605307340622,0.2158605307340622,0.2158605307340622] at 0.25, \
         stroke [1,0,0] at 0.5, 10 px",
        shapes_of(&viewer).split("; ").next().unwrap_or_default().to_string(),
    );
    run(&viewer, &format!("shape.set?layer={id}&shape=0&fill=none"));
    run(&viewer, &format!("shape.set?layer={id}&shape=1&closed=1"));
    run(&viewer, &format!("shape.set?layer={id}&shape=1&enabled=1"));
    report.check(
        "No fill takes the fill off; Open or Closed and On or Off each turn over one shape",
        "Shape 1 closed 4 points, fill none, stroke [1,0,0] at 0.5, 10 px; \
         Shape 2 closed 2 points, fill none, stroke [1,1,1] at 1, 4 px, off",
        shapes_of(&viewer),
    );

    // A point dragged, and the stopwatch, as a mask's are.
    run(
        &viewer,
        &format!(
            "shape.set_path?layer={id}&shape=0&frame=0&points=150,100,0,0,0,0;\
             300,100,0,0,0,0;300,300,0,0,0,0;100,300,0,0,0,0"
        ),
    );
    report.check(
        "moving one point moves that point and no other",
        "[150,100] then [300,100]",
        format!(
            "{} then {}",
            shown_layer(&viewer, "Shape Layer 1")["shapes"][0]["path"]["base"]["points"][0]
                ["point"],
            shown_layer(&viewer, "Shape Layer 1")["shapes"][0]["path"]["base"]["points"][1]
                ["point"]
        ),
    );
    run(
        &viewer,
        &format!("shape.add_remove_key?layer={id}&shape=0&frame=0"),
    );
    report.check(
        "the stopwatch at frame 0 sets the shape's path moving, from the shape it has now",
        "1 key, at frame 0",
        {
            let keys = shown_layer(&viewer, "Shape Layer 1")["shapes"][0]["path"]["keyframes"]
                .as_array()
                .cloned()
                .unwrap_or_default();
            format!(
                "{} key, at frame {}",
                keys.len(),
                keys.first()
                    .map_or("none".to_string(), |k| k["frame"].to_string())
            )
        },
    );

    // What must be refused, each with the reason a person is given.
    report.check(
        "a shape of one point is refused, with the reason",
        "A shape needs at least two points, and \"Shape 3\" has 1. Place a second point \
         before finishing the shape.",
        run(&viewer, &format!("shape.add?layer={id}&points=0,0,0,0,0,0")),
    );
    report.check(
        "a stroke width of 0 is refused, with the reason",
        "Shape \"Shape 1\" cannot be drawn: it needs a stroke width above 0 and at most \
         8192, not 0. Set a value inside the range and send the shapes again.",
        run(
            &viewer,
            &format!("shape.set?layer={id}&shape=0&stroke_width=0"),
        ),
    );
    report.check(
        "a fill opacity on a shape with no fill is refused, with the reason",
        "\"Shape 1\" has no fill to set the opacity of.",
        run(
            &viewer,
            &format!("shape.set?layer={id}&shape=0&fill_opacity=50"),
        ),
    );
    report.check(
        "a colour that is not three numbers is refused, with the reason",
        "\"red\" is not a colour. A colour is three numbers from 0 to 1, red, green and blue.",
        run(&viewer, &format!("shape.set?layer={id}&shape=0&fill=red")),
    );
    report.check(
        "a shape that is not there is refused, and the window says how many there are",
        "Which shape? \"Shape Layer 1\" has 2.",
        run(&viewer, &format!("shape.set?layer={id}&shape=7&enabled=1")),
    );
    report.check(
        "a drawing layer has no shapes, so the shape commands refuse it",
        "\"Cel\" is not a shape layer, so it has no shapes.",
        run(
            &viewer,
            &format!("shape.add?layer={cel}&points=0,0,0,0,0,0;9,9,0,0,0,0"),
        ),
    );

    run(&viewer, &format!("shape.delete?layer={id}&shape=0"));
    report.check(
        "Delete takes one shape off and leaves the other",
        "Shape 2 closed 2 points, fill none, stroke [1,1,1] at 1, 4 px, off",
        shapes_of(&viewer),
    );
    run(&viewer, "edit.undo");
    report.check(
        "and Undo brings it back as it was",
        "Shape 1 closed 4 points, fill none, stroke [1,0,0] at 0.5, 10 px; \
         Shape 2 closed 2 points, fill none, stroke [1,1,1] at 1, 4 px, off",
        shapes_of(&viewer),
    );

    write_artifact(
        &report,
        "verification/B-25c_panel_table.md",
        "B-25c: a shape layer in the window",
        SHAPE_PANEL_INTRO,
        SHAPE_PANEL_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// B-108b: D-168's gradients from the window. The core's half is
/// `verification/B-108_shape_gradient_table.md`; this is what the panel sends and what comes
/// back.
#[test]
fn a_shape_gradient_is_set_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    run(&viewer, "layer.add_shape");
    let id = shown_layer(&viewer, "Shape Layer 1")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    run(
        &viewer,
        &format!(
            "shape.add?layer={id}&closed=1&points=100,100,0,0,0,0;300,100,0,0,0,0;\
             300,300,0,0,0,0;100,300,0,0,0,0"
        ),
    );
    // The fill's gradient as the page reads it off the document, in one line.
    let fill = |viewer: &Mutex<Viewer>| {
        let f = shown_layer(viewer, "Shape Layer 1")["shapes"][0]["fill"].clone();
        let g = &f["gradient"];
        if g.is_null() {
            return format!("flat {}", f["color"]);
        }
        let point = |p: &serde_json::Value| match p["keyframes"].as_array() {
            Some(keys) if !keys.is_empty() => format!(
                "keyed {}",
                keys.iter()
                    .map(|k| format!("{}@{}", k["value"], k["frame"]))
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            _ => p["base"].to_string(),
        };
        format!(
            "{} from {} to {}, stops {}",
            g["type"].as_str().unwrap_or("?"),
            point(&g["start"]),
            point(&g["end"]),
            g["stops"]
                .as_array()
                .map(|stops| stops
                    .iter()
                    .map(|s| format!("{} {} {}", s["offset"], s["color"], s["opacity"]))
                    .collect::<Vec<_>>()
                    .join("; "))
                .unwrap_or_default()
        )
    };
    let at = |viewer: &Mutex<Viewer>, frame: i32, prop: &str| {
        let body = boxes(viewer, frame, None).into_body();
        let answer: serde_json::Value =
            serde_json::from_slice(&body).expect("the boxes answer is JSON");
        answer["values"][&id][prop].to_string()
    };

    run(&viewer, &format!("shape.set?layer={id}&shape=0&fill=1,0,0"));
    run(
        &viewer,
        &format!("shape.set?layer={id}&shape=0&fill_gradient=linear"),
    );
    report.check(
        "Linear turns a gradient on across the shape, middle of its left side to middle of \
         its right, from the fill's colour to black",
        "linear from [100,200] to [300,200], stops 0 [1,0,0] 1; 1 [0,0,0] 1",
        fill(&viewer),
    );
    report.check(
        "Undo says what it would take back",
        "Set one shape",
        held(&viewer)
            .document
            .undo_labels()
            .last()
            .cloned()
            .unwrap_or_default(),
    );
    run(
        &viewer,
        &format!("shape.set?layer={id}&shape=0&fill_stops=0,1,0,0,1;0.5,0,1,0,0.5;1,0,0,1,1"),
    );
    report.check(
        "the stop editor sends every stop at once: offset, colour and opacity each",
        "linear from [100,200] to [300,200], stops 0 [1,0,0] 1; 0.5 [0,1,0] 0.5; 1 [0,0,1] 1",
        fill(&viewer),
    );
    run(
        &viewer,
        &format!("shape.set?layer={id}&shape=0&fill_gradient=radial"),
    );
    run(&viewer, &format!("shape.set?layer={id}&shape=0&fill=0,1,0"));
    report.check(
        "Radial keeps the points and the stops, and a new flat colour leaves the gradient alone",
        "radial from [100,200] to [300,200], stops 0 [1,0,0] 1; 0.5 [0,1,0] 0.5; 1 [0,0,1] 1",
        fill(&viewer),
    );
    run(
        &viewer,
        &format!("property.set_base?layer={id}&prop=shape:0:fill_end&value=400,200&frame=0"),
    );
    report.check(
        "the end point typed in its row moves it, with no key",
        "radial from [100,200] to [400,200], stops 0 [1,0,0] 1; 0.5 [0,1,0] 0.5; 1 [0,0,1] 1",
        fill(&viewer),
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={id}&prop=shape:0:fill_start&frame=0"),
    );
    run(
        &viewer,
        &format!("property.set_base?layer={id}&prop=shape:0:fill_start&value=200,200&frame=10"),
    );
    report.check(
        "the diamond keys the start where it is, and a value on frame 10 is a second key",
        "radial from keyed [100,200]@0 [200,200]@10 to [400,200], stops 0 [1,0,0] 1; 0.5 [0,1,0] 0.5; 1 [0,0,1] 1",
        fill(&viewer),
    );
    report.check(
        "halfway, frame 5, the page is given the point the renderer uses",
        "[150.0,200.0]",
        at(&viewer, 5, "shape:0:fill_start"),
    );
    report.check(
        "and the unkeyed end, on any frame",
        "[400.0,200.0]",
        at(&viewer, 5, "shape:0:fill_end"),
    );
    report.check(
        "a key moved onto another is refused, with the reason",
        "This point already has a key at frame 0.",
        run(
            &viewer,
            &format!("keyframe.move?layer={id}&prop=shape:0:fill_start&from=10&to=0"),
        ),
    );
    run(
        &viewer,
        &format!("keyframe.move?layer={id}&prop=shape:0:fill_start&from=10&to=12"),
    );
    run(
        &viewer,
        &format!("keyframe.set_interp?layer={id}&prop=shape:0:fill_start&frame=12&mode=hold"),
    );
    report.check(
        "a key moved, and its ease set, as a position key's are",
        "[{\"frame\":0,\"interp\":\"linear\",\"value\":[100,200]},{\"frame\":12,\"interp\":\"hold\",\"value\":[200,200]}]",
        shown_layer(&viewer, "Shape Layer 1")["shapes"][0]["fill"]["gradient"]["start"]
            ["keyframes"]
            .to_string(),
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={id}&prop=shape:0:fill_start&frame=12"),
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={id}&prop=shape:0:fill_start&frame=0"),
    );
    report.check(
        "both keys taken off, the point stays where the last one held it",
        "radial from [100,200] to [400,200], stops 0 [1,0,0] 1; 0.5 [0,1,0] 0.5; 1 [0,0,1] 1",
        fill(&viewer),
    );
    report.check(
        "one stop is refused, with the reason",
        "Shape \"Shape 1\" cannot be drawn: it needs a fill gradient of 2 to 64 stops, not 1. Set a value inside the range and send the shapes again.",
        run(&viewer, &format!("shape.set?layer={id}&shape=0&fill_stops=0,1,0,0,1")),
    );
    report.check(
        "a kind that is not a gradient is refused, with the reason",
        "\"conic\" is not a kind of gradient. It is none, linear or radial.",
        run(
            &viewer,
            &format!("shape.set?layer={id}&shape=0&fill_gradient=conic"),
        ),
    );
    report.check(
        "a stop that is not five numbers is refused, with the reason",
        "A stop is five numbers, offset,r,g,b,opacity. Not \"0,1,0\".",
        run(
            &viewer,
            &format!("shape.set?layer={id}&shape=0&fill_stops=0,1,0;1,0,0,1,1"),
        ),
    );
    report.check(
        "a gradient on a stroke the shape does not have is refused, with the reason",
        "\"Shape 1\" has no stroke to give a gradient.",
        run(
            &viewer,
            &format!("shape.set?layer={id}&shape=0&stroke_gradient=linear"),
        ),
    );
    report.check(
        "a key on a gradient point that is not there is refused, with the reason",
        "\"Shape 1\" has no stroke gradient.",
        run(
            &viewer,
            &format!("keyframe.add_remove?layer={id}&prop=shape:0:stroke_start&frame=0"),
        ),
    );
    run(
        &viewer,
        &format!("shape.set?layer={id}&shape=0&fill_gradient=none"),
    );
    report.check(
        "Flat takes the gradient off and the fill is its own colour again",
        "flat [0,1,0]",
        fill(&viewer),
    );
    run(&viewer, "edit.undo");
    report.check(
        "and Undo puts it back",
        "radial from [100,200] to [400,200], stops 0 [1,0,0] 1; 0.5 [0,1,0] 0.5; 1 [0,0,1] 1",
        fill(&viewer),
    );

    write_artifact(
        &report,
        "verification/B-108b_panel_table.md",
        "B-108b: gradients on shapes from the window",
        &[
            "D-168 decided what a gradient on a shape is and B-108b built it in the core, \
           checked pixel by pixel in `verification/B-108_shape_gradient_table.md`. This is \
           the window's half: the shape's panel sends `shape.set` with `fill_gradient`, \
           `stroke_gradient`, `fill_stops` and `stroke_stops`, and a gradient's two points \
           are named `shape:<n>:fill_start`, `fill_end`, `stroke_start` and `stroke_end` \
           wherever a property is named, so the diamond, a typed value, a key moved and an \
           ease reach them by the requests they already send.",
        ],
        &[
            "## What this does not cover\n\nWhat the gradient looks like on the picture and \
           whether the panel feels right in the hand. That is \
           `verification/B-108_shape_gradient_playtest.md`, for a person.",
        ],
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// B-109b: D-169's trim paths from the window. The core's half is
/// `verification/B-109_shape_trim_table.md`; this is what the panel sends and what comes back.
#[test]
fn a_shape_is_trimmed_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    run(&viewer, "layer.add_shape");
    let id = shown_layer(&viewer, "Shape Layer 1")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    run(
        &viewer,
        &format!("shape.add?layer={id}&points=100,100,0,0,0,0;300,100,0,0,0,0;300,300,0,0,0,0"),
    );
    // The trim as the page reads it off the document, in one line.
    let trim = |viewer: &Mutex<Viewer>| {
        let t = shown_layer(viewer, "Shape Layer 1")["shapes"][0]["trim"].clone();
        if t.is_null() {
            return "none".to_string();
        }
        ["start", "end", "offset"]
            .map(|name| match t[name]["keyframes"].as_array() {
                Some(keys) if !keys.is_empty() => format!(
                    "{name} keyed {}",
                    keys.iter()
                        .map(|k| format!("{}@{}", k["value"], k["frame"]))
                        .collect::<Vec<_>>()
                        .join(" ")
                ),
                _ => format!("{name} {}", t[name]["base"]),
            })
            .join(", ")
    };
    let at = |viewer: &Mutex<Viewer>, frame: i32, prop: &str| {
        let body = boxes(viewer, frame, None).into_body();
        let answer: serde_json::Value =
            serde_json::from_slice(&body).expect("the boxes answer is JSON");
        answer["values"][&id][prop].to_string()
    };
    let set = |prop: &str, value: &str, frame: i32| {
        run(
            &viewer,
            &format!(
                "property.set_base?layer={id}&prop=shape:0:{prop}&value={value}&frame={frame}"
            ),
        )
    };

    report.check("A new shape has no trim", "none", trim(&viewer));
    run(&viewer, &format!("shape.set?layer={id}&shape=0&trim=on"));
    report.check(
        "Trim paths turned on is the whole path with no offset, so the picture is unchanged",
        "start 0, end 100, offset 0",
        trim(&viewer),
    );
    report.check(
        "Undo says what it would take back",
        "Set one shape",
        held(&viewer)
            .document
            .undo_labels()
            .last()
            .cloned()
            .unwrap_or_default(),
    );
    set("trim_end", "50", 0);
    set("trim_offset", "90", 0);
    report.check(
        "the end and the offset typed in their rows set them, with no key",
        "start 0, end 50, offset 90",
        trim(&viewer),
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={id}&prop=shape:0:trim_end&frame=0"),
    );
    set("trim_end", "100", 10);
    report.check(
        "the diamond keys the end where it is, and a value on frame 10 is a second key",
        "start 0, end keyed 50@0 100@10, offset 90",
        trim(&viewer),
    );
    report.check(
        "halfway, frame 5, the page is given the number the renderer uses",
        "75.0",
        at(&viewer, 5, "shape:0:trim_end"),
    );
    report.check(
        "and the unkeyed offset, on any frame",
        "90.0",
        at(&viewer, 5, "shape:0:trim_offset"),
    );
    report.check(
        "a key moved onto another is refused, with the reason",
        "This number already has a key at frame 0.",
        run(
            &viewer,
            &format!("keyframe.move?layer={id}&prop=shape:0:trim_end&from=10&to=0"),
        ),
    );
    report.check(
        "a start below 0 is refused, with the reason",
        "Shape \"Shape 1\" cannot be drawn: it needs a trim start from 0 to 100, not -5. Set a value inside the range and send the shapes again.",
        set("trim_start", "-5", 0),
    );
    report.check(
        "a value that is not a number is refused, with the reason",
        "What number? Say value=50.",
        set("trim_start", "half", 0),
    );
    report.check(
        "a trim that is neither on nor off is refused, with the reason",
        "\"maybe\" is not on or off.",
        run(&viewer, &format!("shape.set?layer={id}&shape=0&trim=maybe")),
    );
    run(&viewer, &format!("shape.set?layer={id}&shape=0&trim=off"));
    report.check("Off takes the trim away", "none", trim(&viewer));
    report.check(
        "a key on a trim the shape does not have is refused, with the reason",
        "\"Shape 1\" has no trim.",
        run(
            &viewer,
            &format!("keyframe.add_remove?layer={id}&prop=shape:0:trim_offset&frame=0"),
        ),
    );
    run(&viewer, "edit.undo");
    report.check(
        "and Undo puts it back, keys and all",
        "start 0, end keyed 50@0 100@10, offset 90",
        trim(&viewer),
    );

    write_artifact(
        &report,
        "verification/B-109b_panel_table.md",
        "B-109b: trim paths from the window",
        &[
            "D-169 decided what trimming a shape's path is and B-109b built it in the core, \
           checked pixel by pixel in `verification/B-109_shape_trim_table.md`. This is the \
           window's half: the shape's panel sends `shape.set` with `trim=on` or `trim=off`, \
           and the trim's three numbers are named `shape:<n>:trim_start`, `trim_end` and \
           `trim_offset` wherever a property is named, so the diamond, a typed value, a key \
           moved and an ease reach them by the requests they already send.",
        ],
        &[
            "## What this does not cover\n\nWhat the trimmed line looks like on the picture and \
           whether the panel feels right in the hand. That is \
           `verification/B-109_shape_trim_playtest.md`, for a person.",
        ],
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// B-110b: D-170's keyed colours, opacities and width, and its joins and caps, from the
/// window. The core's half is `verification/B-110_shape_style_table.md`.
#[test]
fn a_shape_style_is_keyed_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    run(&viewer, "layer.add_shape");
    let id = shown_layer(&viewer, "Shape Layer 1")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    run(
        &viewer,
        &format!("shape.add?layer={id}&points=100,100,0,0,0,0;300,100,0,0,0,0;300,300,0,0,0,0"),
    );
    // The stroke as the page reads it off the document, in one line.
    let stroke = |viewer: &Mutex<Viewer>| {
        let s = shown_layer(viewer, "Shape Layer 1")["shapes"][0]["stroke"].clone();
        if s.is_null() {
            return "none".to_string();
        }
        let mut words: Vec<String> = ["color", "opacity", "width_px"]
            .map(|name| match s[name]["keyframes"].as_array() {
                Some(keys) => format!(
                    "{name} keyed {}",
                    keys.iter()
                        .map(|k| format!("{}@{}", k["value"], k["frame"]))
                        .collect::<Vec<_>>()
                        .join(" ")
                ),
                None => format!("{name} {}", s[name]),
            })
            .to_vec();
        for name in ["join", "cap", "miter_limit"] {
            if !s[name].is_null() {
                words.push(format!("{name} {}", s[name]));
            }
        }
        words.join(", ")
    };
    let at = |viewer: &Mutex<Viewer>, frame: i32, prop: &str| {
        let body = boxes(viewer, frame, None).into_body();
        let answer: serde_json::Value =
            serde_json::from_slice(&body).expect("the boxes answer is JSON");
        answer["values"][&id][prop].to_string()
    };
    let set = |prop: &str, value: &str, frame: i32| {
        run(
            &viewer,
            &format!(
                "property.set_base?layer={id}&prop=shape:0:{prop}&value={value}&frame={frame}"
            ),
        )
    };
    let shape_set = |what: &str| run(&viewer, &format!("shape.set?layer={id}&shape=0&{what}"));

    shape_set("stroke=1,1,1");
    report.check(
        "A stroke added from the window is round at its corners and ends, so the file says nothing of them",
        "color [1,1,1], opacity 1, width_px 4",
        stroke(&viewer),
    );
    set("stroke_width", "10", 0);
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={id}&prop=shape:0:stroke_width&frame=0"),
    );
    set("stroke_width", "30", 10);
    report.check(
        "the width typed, keyed by the diamond, and a value on frame 10 is a second key",
        "color [1,1,1], opacity 1, width_px keyed 10@0 30@10",
        stroke(&viewer),
    );
    report.check(
        "halfway, frame 5, the page is given the width the renderer uses",
        "20.0",
        at(&viewer, 5, "shape:0:stroke_width"),
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={id}&prop=shape:0:stroke_color&frame=0"),
    );
    set("stroke_color", "0,0,1", 10);
    report.check(
        "the colour is keyed as three numbers together, white at 0 and blue at 10",
        "color keyed [1,1,1]@0 [0,0,1]@10, opacity 1, width_px keyed 10@0 30@10",
        stroke(&viewer),
    );
    report.check(
        "halfway the page is given the colour, straight between the two in linear light",
        "[0.5,0.5,1.0]",
        at(&viewer, 5, "shape:0:stroke_color"),
    );
    report.check(
        "a colour with two numbers is refused, with the reason",
        "What colour? Say value=r,g,b, each from 0 to 1.",
        set("stroke_color", "1,0", 0),
    );
    report.check(
        "a colour key above 1 is refused, with the reason",
        "Shape \"Shape 1\" cannot be drawn: it needs a stroke colour of three numbers from 0 to 1, not 2. Set a value inside the range and send the shapes again.",
        set("stroke_color", "2,0,0", 10),
    );
    report.check(
        "an opacity above 100% is refused, with the reason",
        "Shape \"Shape 1\" cannot be drawn: it needs a stroke opacity from 0 to 1, not 1.5. Set a value inside the range and send the shapes again.",
        set("stroke_opacity", "1.5", 0),
    );
    report.check(
        "a width key at 0 is refused, with the reason",
        "Shape \"Shape 1\" cannot be drawn: it needs a stroke width above 0 and at most 8192, not 0. Set a value inside the range and send the shapes again.",
        set("stroke_width", "0", 10),
    );
    shape_set("join=miter&cap=butt&miter_limit=2");
    report.check(
        "a mitred join, butt caps and a limit of 2 are written into the file",
        "color keyed [1,1,1]@0 [0,0,1]@10, opacity 1, width_px keyed 10@0 30@10, join \"miter\", cap \"butt\", miter_limit 2",
        stroke(&viewer),
    );
    report.check(
        "a join that is not one of the three is refused, with the reason",
        "\"sharp\" is not a join. It is miter, round or bevel.",
        shape_set("join=sharp"),
    );
    report.check(
        "a mitre limit below 1 is refused, with the reason",
        "Shape \"Shape 1\" cannot be drawn: it needs a mitre limit from 1 to 100, not 0.5. Set a value inside the range and send the shapes again.",
        shape_set("miter_limit=0.5"),
    );
    shape_set("join=round&cap=round&miter_limit=4");
    report.check(
        "put back to round, round and 4, the three words leave the file",
        "color keyed [1,1,1]@0 [0,0,1]@10, opacity 1, width_px keyed 10@0 30@10",
        stroke(&viewer),
    );
    shape_set("stroke=none");
    report.check(
        "No stroke takes it away, keys and all",
        "none",
        stroke(&viewer),
    );
    report.check(
        "a key on a stroke the shape does not have is refused, with the reason",
        "\"Shape 1\" has no stroke.",
        run(
            &viewer,
            &format!("keyframe.add_remove?layer={id}&prop=shape:0:stroke_width&frame=0"),
        ),
    );
    report.check(
        "nor does a stroke added again bring the old keys back",
        "color [1,1,1], opacity 1, width_px 4",
        {
            shape_set("stroke=1,1,1");
            stroke(&viewer)
        },
    );
    run(&viewer, "edit.undo");
    run(&viewer, "edit.undo");
    report.check(
        "and two Undos put the keyed stroke back",
        "color keyed [1,1,1]@0 [0,0,1]@10, opacity 1, width_px keyed 10@0 30@10",
        stroke(&viewer),
    );

    write_artifact(
        &report,
        "verification/B-110b_panel_table.md",
        "B-110b: shape styles over time, and joins and caps, from the window",
        &[
            "D-170 decided that a shape's colours, opacities and stroke width take keys, and \
           how a stroke turns its corners and ends its open lines; B-110b built it in the \
           core, checked pixel by pixel in `verification/B-110_shape_style_table.md`. This \
           is the window's half: the five are named `shape:<n>:fill_color`, `fill_opacity`, \
           `stroke_color`, `stroke_opacity` and `stroke_width` wherever a property is named, \
           so the diamond, a typed value, a key moved and an ease reach them by the requests \
           they already send, and the shape's panel sends `shape.set` with `join`, `cap` and \
           `miter_limit`.",
        ],
        &[
            "## What this does not cover\n\nWhat the corners and the changing colours look \
           like on the picture and whether the panel feels right in the hand. That is \
           `verification/B-110_shape_style_playtest.md`, for a person.",
        ],
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// B-111b: D-171's camera parent from the window. The core's half is
/// `verification/B-111_camera_rig_table.md`.
#[test]
fn a_camera_rides_a_layer_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/camera_rig/fx_rig_003.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let page =
        std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui/index.html"))
            .expect("the page");
    // The camera as the page is given it, in one line.
    let camera = |viewer: &Mutex<Viewer>| {
        let answer: serde_json::Value =
            serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
        let c = &answer["project"]["compositions"][0]["camera"];
        format!(
            "parent {}, place {}, depth {}",
            c["parent"], c["position"]["base"], c["depth"]["base"]
        )
    };

    report.check(
        "the camera's panel has a Parent list",
        true,
        page.contains("chooser.setAttribute('aria-label', 'Parent for the camera');"),
    );
    report.check(
        "FX-RIG-003 opens with its camera on the null \"rig\", at 50, 50 in the null's own pixels",
        "parent \"rig\", place [50,50], depth -8.333333333333334",
        camera(&viewer),
    );
    run(&viewer, "layer.set_parent?target=camera&parent=&frame=2");
    report.check(
        "cleared on frame 2, where the null has carried it to 5, 1, it stays there",
        "parent null, place [5,1], depth -8.333333333333334",
        camera(&viewer),
    );
    run(&viewer, "layer.set_parent?target=camera&parent=rig&frame=0");
    report.check(
        "given the null again on frame 0, where the null stands 2 pixels further left, it is \
         written 2 further right in the null's pixels, so it still stands at 5, 1",
        "parent \"rig\", place [52,50], depth -8.333333333333334",
        camera(&viewer),
    );
    report.check(
        "a parent that is not in the composition is refused, with the reason",
        "The layer chosen as the camera's parent, nope, is not in this composition.",
        run(
            &viewer,
            "layer.set_parent?target=camera&parent=nope&frame=0",
        ),
    );
    run(&viewer, "layer.delete?layer=rig&frame=0");
    report.check(
        "deleting the null lets the camera go where it stands, as it does a child layer",
        "parent null, place [5,1], depth -8.333333333333334",
        camera(&viewer),
    );
    run(&viewer, "edit.undo");
    report.check(
        "one Undo brings the null back with the camera on it",
        "parent \"rig\", place [52,50], depth -8.333333333333334",
        camera(&viewer),
    );
    run(&viewer, "edit.undo");
    run(&viewer, "edit.undo");
    report.check(
        "and two more put it back as it opened",
        "parent \"rig\", place [50,50], depth -8.333333333333334",
        camera(&viewer),
    );

    write_artifact(
        &report,
        "verification/B-111b_panel_table.md",
        "B-111b: a camera that rides a layer, from the window",
        &[
            "D-171 decided that the camera can ride a layer, as a layer rides its parent; \
           B-111b built it in the core, checked frame by frame in \
           `verification/B-111_camera_rig_table.md`. This is the window's half: the \
           camera's panel has a Parent list, which sends the request a layer's own Parent \
           list sends, `layer.set_parent`, with `target=camera`. It keeps the camera where \
           it stands on the frame shown, and deleting the layer it rides lets it go where it \
           stands. An audio layer is not offered; the core refuses one, which its table \
           checks.",
        ],
        &[
            "## What this does not cover\n\nWhether the shot moves as a person expects when \
           the layer under it moves, and whether the list feels right in the hand. That is \
           `verification/B-111_camera_rig_playtest.md`, for a person.",
        ],
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// B-18c: the precomposition from the window, on D-67. The order is the playtest sheet's.
#[test]
fn layers_are_precomposed_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let comps = |viewer: &Mutex<Viewer>| {
        held(viewer)
            .document
            .project()
            .compositions
            .iter()
            .map(|c| c.name.clone())
            .collect::<Vec<_>>()
            .join(", ")
    };
    let main = held(&viewer).composition.as_str().to_string();
    let main_name = comps(&viewer);
    let cel = shown_layer(&viewer, "Cel")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();

    report.check(
        "Pre-compose with nothing chosen says so",
        "Pre-compose needs at least one layer of this composition chosen. The edit was not applied. Nothing in the project changed.",
        run(&viewer, "layer.precompose"),
    );
    run(&viewer, "layer.add_adjustment?name=Grade");
    let grade = shown_layer(&viewer, "Grade")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    run(
        &viewer,
        &format!("layer.set_parent?layer={grade}&parent={cel}&frame=0"),
    );
    report.check(
        "Pre-compose on a layer whose child is not chosen is refused, with the reason",
        "\"Grade\" and \"Cel\" are tied by a parent or a matte, so they are pre-composed \
         together or not at all. Choose both.",
        run(&viewer, &format!("layer.precompose?layer={cel}")),
    );
    report.check("and nothing moved", "Cel, Grade", names(&viewer));

    report.check(
        "Pre-compose on both says what happened",
        "Precomp 1 holds the chosen layers now, and one layer shows it here. Double-click \
         that layer to open it. Ctrl+Z puts them back.",
        run(
            &viewer,
            &format!("layer.precompose?layer={cel}&layer={grade}"),
        ),
    );
    report.check(
        "one layer stands where the two were",
        "Precomp 1",
        names(&viewer),
    );
    report.check(
        "and the project has the new composition",
        format!("{main_name}, Precomp 1"),
        comps(&viewer),
    );
    let layer = shown_layer(&viewer, "Precomp 1");
    let inner = layer["composition_id"]
        .as_str()
        .unwrap_or("(none)")
        .to_string();
    report.check(
        "the page is told its kind and its composition, and it has no drawing",
        "composition, a composition_id, no asset_id",
        format!(
            "{}, {}, {}",
            layer["kind"].as_str().unwrap_or("(no kind)"),
            if layer.get("composition_id").is_some() {
                "a composition_id"
            } else {
                "no composition_id"
            },
            if layer.get("asset_id").is_none() {
                "no asset_id"
            } else {
                "an asset_id"
            }
        ),
    );
    report.check(
        "it is centred: anchor and position at the centre of 1920 by 1080",
        "[960,540] and [960,540]",
        format!(
            "{} and {}",
            layer["transform"]["anchor"]["base"], layer["transform"]["position"]["base"]
        ),
    );

    report.check(
        "deleting Precomp 1 while the layer shows it is refused, naming who uses it",
        format!(
            "This composition is shown by a layer of \"{main_name}\", so it stays. Delete \
             that layer first if this one should go."
        ),
        run(&viewer, &format!("composition.delete?composition={inner}")),
    );

    run(&viewer, &format!("composition.open?composition={inner}"));
    report.check(
        "double-click opens Precomp 1, and the two layers are in it, in their order",
        "Cel, Grade",
        names(&viewer),
    );
    report.check(
        "inside it, adding the outer composition as a layer is refused: it would hold itself",
        format!("\"{main_name}\" cannot be shown here, because it would end up inside itself. The edit was not applied. Nothing in the project changed."),
        run(&viewer, &format!("layer.add_composition?of={main}")),
    );
    run(&viewer, &format!("composition.open?composition={main}"));

    run(&viewer, &format!("layer.add_composition?of={inner}"));
    report.check(
        "the Project panel's button adds a second layer of Precomp 1, at the front",
        "Precomp 1, Precomp 1",
        names(&viewer),
    );
    run(&viewer, "edit.undo");
    run(&viewer, "edit.undo");
    report.check(
        "Undo twice: the button's layer goes, then Pre-compose goes as one entry",
        format!("Cel, Grade; {main_name}"),
        format!("{}; {}", names(&viewer), comps(&viewer)),
    );
    run(&viewer, "edit.redo");
    report.check(
        "and Redo makes it again",
        format!("Precomp 1; {main_name}, Precomp 1"),
        format!("{}; {}", names(&viewer), comps(&viewer)),
    );
    run(&viewer, "edit.undo");
    report.check(
        "D-273: the name the Pre-compose window sends is the new composition's, trimmed",
        format!("Hair comp; {main_name}, Hair comp"),
        {
            run(
                &viewer,
                &format!("layer.precompose?layer={cel}&layer={grade}&name=%20Hair%20comp%20"),
            );
            format!("{}; {}", names(&viewer), comps(&viewer))
        },
    );
    run(&viewer, "edit.undo");
    report.check(
        "and a name of only spaces gives the next Precomp number",
        format!("Precomp 1; {main_name}, Precomp 1"),
        {
            run(
                &viewer,
                &format!("layer.precompose?layer={cel}&layer={grade}&name=%20%20"),
            );
            format!("{}; {}", names(&viewer), comps(&viewer))
        },
    );

    write_artifact(
        &report,
        "verification/B-18c_panel_table.md",
        "B-18c: precompositions in the window",
        PRECOMP_PANEL_INTRO,
        PRECOMP_PANEL_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// B-19a: a key dragged in the graph, or typed, on D-52. The order is the playtest sheet's.
#[test]
fn a_key_is_dragged_and_typed_in_the_graph() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let cel = shown_layer(&viewer, "Cel")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    // What the page reads: each key of a property as `frame: value, interp`.
    let keys = |viewer: &Mutex<Viewer>, prop: &str| {
        shown_layer(viewer, "Cel")["transform"][prop]["keyframes"]
            .as_array()
            .map(|all| {
                all.iter()
                    .map(|k| format!("{}: {} {}", k["frame"], k["value"], k["interp"]))
                    .collect::<Vec<_>>()
                    .join("; ")
            })
            .unwrap_or_default()
            .replace('"', "")
    };
    let depth = |viewer: &Mutex<Viewer>| held(viewer).document.undo_depth();
    for edit in [
        format!("property.set_base?layer={cel}&prop=position&value=100,100"),
        format!("keyframe.add_remove?layer={cel}&prop=position&frame=0"),
        format!("property.set_base?layer={cel}&prop=position&frame=10&value=300,200"),
        format!(
            "keyframe.set_interp?layer={cel}&prop=position&frame=0&mode=ease&curve=0.4,0,0.6,1"
        ),
        format!("property.set_base?layer={cel}&prop=scale&value=100,100"),
        format!("keyframe.add_remove?layer={cel}&prop=scale&frame=0"),
    ] {
        run(&viewer, &edit);
    }
    report.check(
        "two position keys to begin with, the first eased",
        "0: [100,100] ease; 10: [300,200] linear",
        keys(&viewer, "position"),
    );
    let before = depth(&viewer);
    run(
        &viewer,
        &format!("keyframe.move?by=0&key={cel}|position|10&value=300,260"),
    );
    report.check(
        "a key dragged straight up: its Y changes and its frame does not",
        "0: [100,100] ease; 10: [300,260] linear",
        keys(&viewer, "position"),
    );
    run(
        &viewer,
        &format!("keyframe.move?by=2&key={cel}|position|0&value=100,140"),
    );
    report.check(
        "a key dragged along and up in one go lands with its new value, and keeps its ease",
        "2: [100,140] ease; 10: [300,260] linear",
        keys(&viewer, "position"),
    );
    report.check(
        "which was one entry to undo each",
        (before + 2).to_string(),
        depth(&viewer).to_string(),
    );
    run(&viewer, "edit.undo");
    report.check(
        "Undo puts the frame and the value back together",
        "0: [100,100] ease; 10: [300,260] linear",
        keys(&viewer, "position"),
    );
    run(&viewer, "edit.redo");
    run(
        &viewer,
        &format!(
            "keyframe.move?by=3&key={cel}|position|2&value=100,150&key={cel}|position|10&value=300,270\
             &key={cel}|scale|0&value=100,100"
        ),
    );
    report.check(
        "two chosen keys dragged together both move and both change",
        "5: [100,150] ease; 13: [300,270] linear",
        keys(&viewer, "position"),
    );
    report.check(
        "and a chosen key of another property goes along in time with its value kept, as a percentage",
        "3: [100,100] linear",
        keys(&viewer, "scale"),
    );
    report.check(
        "a value for one key and not the other is refused",
        "2 keys were named and 1 values: a value is given for every key or for none.",
        run(
            &viewer,
            &format!("keyframe.move?by=1&key={cel}|position|5&key={cel}|position|13&value=1,1"),
        ),
    );
    report.check(
        "text that is not a value is refused",
        "position has no key at frame 5, or \"high\" is not a value for it.",
        run(
            &viewer,
            &format!("keyframe.move?by=1&key={cel}|position|5&value=high"),
        ),
    );
    report.check(
        "a key dragged onto a frame another key holds is refused by the core, and nothing changes",
        "5: [100,150] ease; 13: [300,270] linear",
        {
            run(
                &viewer,
                &format!("keyframe.move?by=8&key={cel}|position|5&value=0,0"),
            );
            keys(&viewer, "position")
        },
    );
    let body = curve(
        &viewer,
        Some(&format!("layer={cel}&prop=position&from=13&to=13")),
    )
    .into_body();
    let answer: serde_json::Value =
        serde_json::from_slice(&body).expect("the curve answer is JSON");
    report.check(
        "the curve the graph draws passes through the key where it was put",
        "[300.0,270.0]",
        answer["samples"][0].to_string(),
    );

    write_artifact(
        &report,
        "verification/B-19a_panel_table.md",
        "B-19a: a key dragged and typed in the graph",
        GRAPH_KEY_INTRO,
        GRAPH_KEY_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

const GRAPH_KEY_INTRO: &[&str] = &[
    "B-19a lets a key in the graph editor be dragged up and down as well as along, and lets \
     its frame and value be typed. Both send `keyframe.move` with a `value=` beside each \
     `key=`: the keys move, and each then holds the value given, as one entry to undo. \
     Nothing in the file format or in how a curve is worked out changes.",
    "Every row calls what the window calls, on `Fixtures/projects/cel_holds_project.json`, \
     and reads back what the page is given.",
];

const GRAPH_KEY_NOTES: &[&str] = &[
    "## What this does not cover\n\nThe dragging itself, the box drawn around keys, the \
     snapping, the numbered lines, and the zoom and Fit of the values are all in the page \
     and change nothing here. They are `verification/B-19a_graph_playtest.md`, for a person.",
];

/// B-19d: D-68's keys from the window. The order is the playtest sheet's.
#[test]
fn an_effects_setting_is_keyed_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let cel = shown_layer(&viewer, "Cel")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    run(
        &viewer,
        &format!("effect.add?layer={cel}&type=core.exposure"),
    );
    let fx = shown_layer(&viewer, "Cel")["effects"][0]["instance_id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    let stops = format!("fx:{fx}:stops");
    // What the page reads: the setting as the file writes it, a plain number or its keys.
    let setting = |viewer: &Mutex<Viewer>, at: usize, name: &str| {
        let held = shown_layer(viewer, "Cel")["effects"][at]["parameters"][name].clone();
        match held["keyframes"].as_array() {
            None => format!("plain {held}"),
            Some(all) => all
                .iter()
                .map(|k| format!("{}: {} {}", k["frame"], k["value"], k["interp"]))
                .collect::<Vec<_>>()
                .join("; ")
                .replace('"', ""),
        }
    };
    let depth = |viewer: &Mutex<Viewer>| held(viewer).document.undo_depth();
    let on_frame = |viewer: &Mutex<Viewer>, frame: i32, prop: &str| {
        let answer: serde_json::Value =
            serde_json::from_slice(boxes(viewer, frame, None).body()).expect("boxes is JSON");
        answer["values"][&cel][prop].to_string()
    };

    run(
        &viewer,
        &format!("effect.set_parameters?layer={cel}&effect={fx}&stops=1"),
    );
    report.check(
        "an exposure of one stop, with no keys, is a plain number as it always was",
        "plain 1",
        setting(&viewer, 0, "stops"),
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={cel}&prop={stops}&frame=0"),
    );
    report.check(
        "the stopwatch on frame 0 makes a key holding what the setting already was",
        "0: 1 linear",
        setting(&viewer, 0, "stops"),
    );
    run(
        &viewer,
        &format!("property.set_base?layer={cel}&prop={stops}&frame=10&value=3"),
    );
    report.check(
        "a value typed on frame 10 of a keyed setting is a second key, not a new constant",
        "0: 1 linear; 10: 3 linear",
        setting(&viewer, 0, "stops"),
    );
    report.check(
        "half way between them the panel is told 2, the value the frame is drawn with",
        "[2.0]",
        on_frame(&viewer, 5, &stops),
    );
    let body = curve(
        &viewer,
        Some(&format!("layer={cel}&prop={stops}&from=5&to=5")),
    )
    .into_body();
    let answer: serde_json::Value =
        serde_json::from_slice(&body).expect("the curve answer is JSON");
    report.check(
        "and the graph is told the same",
        "[2.0]",
        answer["samples"][0].to_string(),
    );
    run(
        &viewer,
        &format!("effect.set_parameters?layer={cel}&effect={fx}&stops=1"),
    );
    report.check(
        "the effect's other settings sent again, as the panel sends them, leave the keys alone",
        "0: 1 linear; 10: 3 linear",
        setting(&viewer, 0, "stops"),
    );
    run(
        &viewer,
        &format!("keyframe.set_interp?layer={cel}&prop={stops}&frame=0&mode=ease"),
    );
    run(
        &viewer,
        &format!("keyframe.move?layer={cel}&prop={stops}&from=10&to=12"),
    );
    report.check(
        "F9 on the first key eases it, and the second dragged along the timeline moves",
        "0: 1 ease; 12: 3 linear",
        setting(&viewer, 0, "stops"),
    );
    run(
        &viewer,
        &format!("property.set_base?layer={cel}&prop=position&value=100,100"),
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={cel}&prop=position&frame=0"),
    );
    let before = depth(&viewer);
    run(
        &viewer,
        &format!(
            "keyframe.move?by=2&key={cel}|{stops}|0&value=1.5&key={cel}|position|0&value=100,120"
        ),
    );
    report.check(
        "a key of the setting and a key of position dragged together in the graph: the setting's \
         moves, takes its value and keeps its ease",
        "2: 1.5 ease; 12: 3 linear",
        setting(&viewer, 0, "stops"),
    );
    report.check(
        "and position's went with it",
        "2: [100,120] linear",
        shown_layer(&viewer, "Cel")["transform"]["position"]["keyframes"]
            .as_array()
            .map(|all| {
                all.iter()
                    .map(|k| format!("{}: {} {}", k["frame"], k["value"], k["interp"]))
                    .collect::<Vec<_>>()
                    .join("; ")
            })
            .unwrap_or_default()
            .replace('"', ""),
    );
    report.check(
        "as one entry to undo",
        (before + 1).to_string(),
        depth(&viewer).to_string(),
    );
    run(&viewer, "edit.undo");
    report.check(
        "which Undo takes back whole",
        "0: 1 ease; 12: 3 linear",
        setting(&viewer, 0, "stops"),
    );
    let before = depth(&viewer);
    for value in ["2", "2.5"] {
        run(
            &viewer,
            &format!("property.drag_update?layer={cel}&prop={stops}&frame=6&value={value}"),
        );
    }
    run(&viewer, "property.drag_end");
    report.check(
        "the number scrubbed on frame 6, two values sent and let go, is one key",
        "0: 1 ease; 6: 2.5 linear; 12: 3 linear",
        setting(&viewer, 0, "stops"),
    );
    report.check(
        "and one entry to undo",
        (before + 1).to_string(),
        depth(&viewer).to_string(),
    );
    report.check(
        "a key moved onto a frame another key holds is refused, and nothing changes",
        "0: 1 ease; 6: 2.5 linear; 12: 3 linear",
        {
            run(
                &viewer,
                &format!("keyframe.move?layer={cel}&prop={stops}&from=6&to=12"),
            );
            setting(&viewer, 0, "stops")
        },
    );

    run(
        &viewer,
        &format!("effect.add?layer={cel}&type=core.gaussian_blur"),
    );
    let blur = shown_layer(&viewer, "Cel")["effects"][1]["instance_id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    let sigma = format!("fx:{blur}:sigma_px");
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={cel}&prop={sigma}&frame=0"),
    );
    let said = run(
        &viewer,
        &format!("property.set_base?layer={cel}&prop={sigma}&frame=4&value=-1"),
    );
    report.check(
        "a blur keyed below nought is refused by name (D-46), and no key is made",
        "true, and the keys are 0: 0 linear",
        format!(
            "{}, and the keys are {}",
            said.contains("EFFECT_PARAMETER_INVALID") || said.contains("sigma"),
            setting(&viewer, 1, "sigma_px")
        ),
    );
    report.check(
        "a setting the effect does not have is refused in words",
        "A core.gaussian_blur has no setting called stops.",
        run(
            &viewer,
            &format!("keyframe.add_remove?layer={cel}&prop=fx:{blur}:stops&frame=0"),
        ),
    );

    run(&viewer, &format!("effect.add?layer={cel}&type=core.tint"));
    let tint = shown_layer(&viewer, "Cel")["effects"][2]["instance_id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    let color = format!("fx:{tint}:color");
    run(
        &viewer,
        &format!("effect.set_parameters?layer={cel}&effect={tint}&color=0,0,1&amount=0.5"),
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={cel}&prop={color}&frame=0"),
    );
    run(
        &viewer,
        &format!("property.set_base?layer={cel}&prop={color}&frame=10&value=1,0,0"),
    );
    report.check(
        "a colour is keyed as its three numbers together",
        "0: [0,0,1] linear; 10: [1,0,0] linear",
        setting(&viewer, 2, "color"),
    );
    report.check(
        "and half way it is half of each",
        "[0.5,0.0,0.5]",
        on_frame(&viewer, 5, &color),
    );

    let before = depth(&viewer);
    run(
        &viewer,
        &format!(
            "keyframe.add_remove?key={cel}|{stops}|0&key={cel}|{stops}|6&key={cel}|{stops}|12"
        ),
    );
    report.check(
        "every chosen key of the setting deleted at once: it is a plain number again",
        "plain 1",
        setting(&viewer, 0, "stops"),
    );
    report.check(
        "as one entry to undo",
        (before + 1).to_string(),
        depth(&viewer).to_string(),
    );
    let saved = persist::to_json(held(&viewer).document.project(), &Preserved::default());
    let again =
        persist::load_str(&saved).map(|l| persist::to_json(l.document.project(), &l.preserved));
    report.check(
        "what a save would write opens again and saves the same",
        "the same",
        if again.as_deref().ok() == Some(saved.as_str()) {
            "the same"
        } else {
            "differs"
        },
    );

    write_artifact(
        &report,
        "verification/B-19d_panel_table.md",
        "B-19d: an effect's setting keyed from the window",
        FX_KEY_INTRO,
        FX_KEY_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// B-19g: D-69 from the window. The order is the playtest sheet's.
#[test]
fn a_position_is_separated_and_keys_are_given_kinds_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let cel = shown_layer(&viewer, "Cel")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    // What the page reads: a property as the file writes it, a plain value or its keys.
    let written = |held: &serde_json::Value| match held["keyframes"].as_array() {
        Some(all) if !all.is_empty() => all
            .iter()
            .map(|k| {
                let ease = k["ease"].as_array().map_or(String::new(), |e| {
                    let e: Vec<String> = e
                        .iter()
                        .map(|v| format!("{:.4}", v.as_f64().unwrap_or(f64::NAN)))
                        .collect();
                    format!(" [{}]", e.join(","))
                });
                let kind = k["kind"]
                    .as_str()
                    .map_or(String::new(), |k| format!(" {k}"));
                let roving = if k["roving"] == true { " roving" } else { "" };
                format!(
                    "{}: {} {}{ease}{kind}{roving}",
                    k["frame"], k["value"], k["interp"]
                )
            })
            .collect::<Vec<_>>()
            .join("; ")
            .replace('"', ""),
        _ => format!("plain {}", held["base"]),
    };
    let prop = |viewer: &Mutex<Viewer>, name: &str| {
        let t = shown_layer(viewer, "Cel")["transform"].clone();
        match name {
            "position_x" => written(&t["position"]["x"]),
            "position_y" => written(&t["position"]["y"]),
            _ => written(&t[name]),
        }
    };
    let depth = |viewer: &Mutex<Viewer>| held(viewer).document.undo_depth();
    let on_frame = |viewer: &Mutex<Viewer>, frame: i32, prop: &str| {
        let answer: serde_json::Value =
            serde_json::from_slice(boxes(viewer, frame, None).body()).expect("boxes is JSON");
        answer["values"][&cel][prop].to_string()
    };

    run(
        &viewer,
        &format!("property.set_base?layer={cel}&prop=position&value=100,50"),
    );
    let before = depth(&viewer);
    report.check(
        "Separate Dimensions on a layer at 100, 50",
        "Separate position into X and Y",
        run(&viewer, &format!("property.separate?layer={cel}")),
    );
    report.check(
        "X position is 100",
        "plain 100",
        prop(&viewer, "position_x"),
    );
    report.check("Y position is 50", "plain 50", prop(&viewer, "position_y"));
    report.check(
        "as one entry to undo",
        (before + 1).to_string(),
        depth(&viewer).to_string(),
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={cel}&prop=position_x&frame=0"),
    );
    run(
        &viewer,
        &format!("property.set_base?layer={cel}&prop=position_x&frame=10&value=300"),
    );
    report.check(
        "X is keyed from 100 to 300 over ten frames, by the stopwatch and a typed value",
        "0: 100 linear; 10: 300 linear",
        prop(&viewer, "position_x"),
    );
    report.check("and Y has no keys", "plain 50", prop(&viewer, "position_y"));
    report.check(
        "half way the panel is told X is 200",
        "200",
        on_frame(&viewer, 5, "position_x"),
    );
    report.check(
        "and the position the viewer's handles start from is 200, 50",
        "[200,50]",
        on_frame(&viewer, 5, "position"),
    );
    report.check(
        "a key on the whole position is refused while it is apart",
        "This position is separated: change its X or its Y.",
        run(
            &viewer,
            &format!("keyframe.add_remove?layer={cel}&prop=position&frame=3"),
        ),
    );
    let before = depth(&viewer);
    run(
        &viewer,
        &format!("property.drag_update?layer={cel}&prop=position&frame=10&value=350,70"),
    );
    run(
        &viewer,
        &format!("property.drag_update?layer={cel}&prop=position&frame=10&value=400,90"),
    );
    run(&viewer, "property.drag_end");
    report.check(
        "the layer dragged in the viewer on frame 10, to 400, 90: X's key there takes the 400",
        "0: 100 linear; 10: 400 linear",
        prop(&viewer, "position_x"),
    );
    report.check(
        "and Y, which has no keys, is 90",
        "plain 90",
        prop(&viewer, "position_y"),
    );
    report.check(
        "the whole drag is one entry to undo",
        (before + 1).to_string(),
        depth(&viewer).to_string(),
    );
    let before = depth(&viewer);
    run(
        &viewer,
        &format!("property.set_base?layer={cel}&prop=position&frame=10&value=400,95"),
    );
    report.check(
        "a pair typed for the whole position is one entry to undo as well",
        format!("{} and plain 95", before + 1),
        format!("{} and {}", depth(&viewer), prop(&viewer, "position_y")),
    );
    run(&viewer, "edit.undo");
    report.check(
        "pressing the button again joins them: one position, a key wherever X or Y had one",
        "0: [100,90] linear; 10: [400,90] linear",
        {
            run(&viewer, &format!("property.separate?layer={cel}"));
            prop(&viewer, "position")
        },
    );

    // The kinds of key, on rotation: 0 at frame 0, 10 at frame 10, 40 at frame 20.
    for (frame, value) in [(0, 0), (10, 10), (20, 40)] {
        run(
            &viewer,
            &format!("keyframe.add_remove?layer={cel}&prop=rotation&frame={frame}"),
        );
        run(
            &viewer,
            &format!("property.set_base?layer={cel}&prop=rotation&frame={frame}&value={value}"),
        );
    }
    let before = depth(&viewer);
    run(
        &viewer,
        &format!("keyframe.set_kind?kind=auto&key={cel}|rotation|10"),
    );
    report.check(
        "Auto bezier on the middle key: 1 a frame in and 3 a frame out become 2 a frame both \
         ways, which is 40 degrees over the 20 frames either side, on handles a third long",
        "0: 0 ease [0.3333,0.3333,0.6667,0.3333]; 10: 10 ease [0.3333,0.2222,0.6667,0.6667] auto; 20: 40 linear",
        prop(&viewer, "rotation"),
    );
    report.check(
        "as one entry to undo",
        (before + 1).to_string(),
        depth(&viewer).to_string(),
    );
    run(
        &viewer,
        &format!("keyframe.set_kind?kind=bezier&key={cel}|rotation|10"),
    );
    report.check(
        "Bezier puts the key back in the person's hands: the handles stay and the word goes",
        "0: 0 ease [0.3333,0.3333,0.6667,0.3333]; 10: 10 ease [0.3333,0.2222,0.6667,0.6667]; 20: 40 linear",
        prop(&viewer, "rotation"),
    );
    report.check(
        "a kind that is not one of the three is asked about",
        "Which kind? Say bezier, continuous or auto.",
        run(
            &viewer,
            &format!("keyframe.set_kind?kind=smooth&key={cel}|rotation|10"),
        ),
    );

    // Roving, on the joined position: 100 to 400 to 1000 along X, keyed on 0, 10 and 20.
    run(
        &viewer,
        &format!("property.set_base?layer={cel}&prop=position&frame=20&value=1000,90"),
    );
    let before = depth(&viewer);
    run(
        &viewer,
        &format!("keyframe.set_roving?roving=true&key={cel}|position|10"),
    );
    report.check(
        "Rove Across Time on the middle key: a third of the way along the path is a third of \
         the way through the 20 frames, so it goes to frame 7",
        "0: [100,90] linear; 7: [400,90] linear roving; 20: [1000,90] linear",
        prop(&viewer, "position"),
    );
    report.check(
        "as one entry to undo",
        (before + 1).to_string(),
        depth(&viewer).to_string(),
    );
    report.check(
        "the first key cannot rove",
        "The first and last keyframes cannot rove.",
        run(
            &viewer,
            &format!("keyframe.set_roving?roving=true&key={cel}|position|0"),
        ),
    );
    run(
        &viewer,
        &format!("keyframe.set_roving?roving=false&key={cel}|position|7"),
    );
    report.check(
        "roving switched off leaves the key on the frame it had reached",
        "0: [100,90] linear; 7: [400,90] linear; 20: [1000,90] linear",
        prop(&viewer, "position"),
    );
    run(
        &viewer,
        &format!("keyframe.set_roving?roving=true&key={cel}|position|7"),
    );
    run(&viewer, &format!("property.separate?layer={cel}"));
    report.check(
        "Separate Dimensions while a key roves: X and Y cannot rove (D-69), so the key \
         stays on its frame and stops roving",
        "0: 100 linear; 7: 400 linear; 20: 1000 linear",
        prop(&viewer, "position_x"),
    );
    let saved = persist::to_json(held(&viewer).document.project(), &Preserved::default());
    let again =
        persist::load_str(&saved).map(|l| persist::to_json(l.document.project(), &l.preserved));
    report.check(
        "what a save would write opens again and saves the same",
        "the same",
        if again.as_deref().ok() == Some(saved.as_str()) {
            "the same"
        } else {
            "differs"
        },
    );

    write_artifact(
        &report,
        "verification/B-19g_panel_table.md",
        "B-19g: a separated position and the kinds of key, from the window",
        KEY_KIND_INTRO,
        KEY_KIND_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

const KEY_KIND_INTRO: &[&str] = &[
    "D-69 gave a position its X and Y as properties of their own, and a key its kind and \
     its roving, and B-19f built them in the core, checked against the fixtures in \
     `verification/B-19f_keykind_table.md`. This is the window's half: `property.separate`, \
     `keyframe.set_kind` and `keyframe.set_roving`, the names `position_x` and `position_y` \
     wherever a property is named, and `position` still answered on a separated layer, so \
     the viewer's handles and the arrow keys move it as they always did.",
    "Every row calls what the window calls, on `Fixtures/projects/cel_holds_project.json`, \
     and reads back what the page is given.",
];

const KEY_KIND_NOTES: &[&str] = &[
    "## What this does not cover\n\nThe rows, the round keys, the menu and the button as \
     they look, and the two halves in the graph, are in the page. They are \
     `verification/B-19g_keykind_playtest.md`, for a person. Whether the numbers are D-69's \
     is B-19f's table.",
];

const FX_KEY_INTRO: &[&str] = &[
    "D-68 gave an effect's setting keys and B-19c built them in the core, checked pixel by \
     pixel in `verification/B-19c_fxkey_table.md`. This is the window's half. The page names \
     a setting `fx:<effect>:<setting>` wherever it names a property, so the stopwatch, the \
     rows of the timeline, the graph, F9 and copy and paste send the requests they already \
     send, and the window turns each into the core's one command for a setting's keys.",
    "Every row calls what the window calls, on `Fixtures/projects/cel_holds_project.json`, \
     and reads back what the page is given.",
];

const FX_KEY_NOTES: &[&str] = &[
    "## What this does not cover\n\nThe stopwatch and the rows as they look, the setting in \
     the graph, and the All button that draws several properties at once are in the page. \
     They are `verification/B-19d_fxkey_playtest.md`, for a person. Whether the pixels are \
     right is B-19c's table.",
];

const AUDIO_PANEL_INTRO: &[&str] = &[
    "D-71 decided what an audio layer is and B-20b built it in the core, checked in \
     `verification/B-20b_audio_table.md`. This is the window's half: sound files in the \
     import and relink dialogs, a layer made from a sound is an audio layer, the level is \
     `layer.set_gain`, the speaker is the switch every layer has, and the page fetches a \
     sound file's bytes from `/sound` to play it with its own decoder.",
    "Every row calls what the window calls, on `Fixtures/projects/cel_holds_project.json` \
     and the WAV files of `Fixtures/audio/media/`, and reads back what the page is given.",
];

const AUDIO_PANEL_NOTES: &[&str] = &[
    "## What this does not cover\n\nWhat is heard, whether it stays with the picture, and what \
     the bars look like. Sound and sync are judged by ear: that is \
     `verification/B-20c_audio_playtest.md`, for a person.",
];

/// B-20c: reference audio from the window, on D-71. The order is the playtest sheet's.
#[test]
fn sound_is_imported_and_levelled_from_the_window() {
    let mut report = Report { rows: Vec::new() };
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let media = |name: &str| repo("Fixtures/audio/media").join(name);
    let asset_named = |viewer: &Mutex<Viewer>, name: &str| {
        let answer: serde_json::Value =
            serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
        answer["project"]["assets"]
            .as_array()
            .expect("a project has assets")
            .iter()
            .find(|a| a["name"] == name)
            .cloned()
            .unwrap_or(serde_json::Value::Null)
    };
    let before = names(&viewer);

    report.check(
        "Importing a WAV says what it is and what to do next",
        "Import pcm16_mono_48k.wav: sound. Drag it to the layer list, or press Make a layer, to hear it.",
        import(&viewer, &[media("pcm16_mono_48k.wav")]),
    );
    let asset = asset_named(&viewer, "pcm16_mono_48k.wav");
    report.check(
        "the page is told it is audio, with a path and no interpretation",
        "audio, a path, no interpretation",
        format!(
            "{}, {}, {}",
            asset["kind"].as_str().unwrap_or("(no kind)"),
            if asset.get("path").is_some() {
                "a path"
            } else {
                "no path"
            },
            if asset.get("interpretation").is_none() {
                "no interpretation"
            } else {
                "an interpretation"
            }
        ),
    );
    let asset_id = asset["id"].as_str().unwrap_or_default().to_string();
    report.check(
        "`/sound` gives the page the file, byte for byte",
        "200, the same bytes",
        {
            let answer = sound(&viewer, Some(&format!("asset={asset_id}")));
            let same = std::fs::read(media("pcm16_mono_48k.wav")).ok().as_deref()
                == Some(answer.body().as_slice());
            format!(
                "{}, {}",
                answer.status().as_u16(),
                if same {
                    "the same bytes"
                } else {
                    "different bytes"
                }
            )
        },
    );
    report.check(
        "`/sound` for a drawing is not answered with its bytes",
        "404",
        sound(&viewer, Some("asset=no-such-sound"))
            .status()
            .as_u16(),
    );

    report.check(
        "Make a layer from it",
        "Add layer Sound",
        run(
            &viewer,
            &format!("layer.create?asset={asset_id}&name=Sound"),
        ),
    );
    report.check(
        "it is at the front",
        format!("{before}, Sound"),
        names(&viewer),
    );
    let layer = shown_layer(&viewer, "Sound");
    report.check(
        "the page is told it is an audio layer: no transform, no blend mode, no effects",
        "audio, no transform, no blend_mode, no effects",
        format!(
            "{}, {}, {}, {}",
            layer["kind"].as_str().unwrap_or("(no kind)"),
            if layer.get("transform").is_none() {
                "no transform"
            } else {
                "a transform"
            },
            if layer.get("blend_mode").is_none() {
                "no blend_mode"
            } else {
                "a blend_mode"
            },
            if layer.get("effects").is_none() {
                "no effects"
            } else {
                "effects"
            }
        ),
    );
    let id = layer["id"].as_str().unwrap_or_default().to_string();

    report.check(
        "The level box sends -6",
        "Set level to -6 dB",
        run(&viewer, &format!("layer.set_gain?layer={id}&value=-6")),
    );
    report.check(
        "and the page is given it back",
        "-6.0",
        format!(
            "{:?}",
            shown_layer(&viewer, "Sound")["gain_db"]
                .as_f64()
                .unwrap_or(f64::NAN)
        ),
    );
    report.check(
        "D-640: `/mix` gives the page the core's mix of frames 0 to 2, the level in it",
        "200, the core's samples",
        {
            let answer = mix(&viewer, Some("from=0&frames=2"));
            let core = {
                let held = viewer.lock().expect("the viewer lock was poisoned");
                let project = shown(&held);
                let comp = project.composition(&held.composition).cloned();
                comp.map(|c| anime_compositor::audio::mix(&project, &held.root, &c, 0, 2).0)
            };
            let want: Option<Vec<u8>> = core.map(|m| {
                m.iter()
                    .flatten()
                    .flat_map(|v| (*v as f32).to_le_bytes())
                    .collect()
            });
            let same = want.as_deref() == Some(answer.body().as_slice())
                && answer.body().iter().any(|b| *b != 0);
            format!(
                "{}, {}",
                answer.status().as_u16(),
                if same {
                    "the core's samples"
                } else {
                    "different samples"
                }
            )
        },
    );
    let refused = run(&viewer, &format!("layer.set_gain?layer={id}&value=13"));
    report.check(
        "A level of 13 is refused and the level stays",
        "A level cannot be set to 13 dB.; -6.0",
        format!(
            "{}; {:?}",
            refused,
            shown_layer(&viewer, "Sound")["gain_db"]
                .as_f64()
                .unwrap_or(f64::NAN)
        ),
    );
    report.check(
        "A level that is not a number says so",
        "\"loud\" is not a level. A level is a number of decibels, from -96 to 12.",
        run(&viewer, &format!("layer.set_gain?layer={id}&value=loud")),
    );
    let cel = shown_layer(&viewer, "Cel")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    let refused = run(&viewer, &format!("layer.set_gain?layer={cel}&value=-3"));
    report.check(
        "A level on a picture layer is refused",
        "\"Cel\" is not an audio layer, so it has no level.",
        refused,
    );
    let refused = run(
        &viewer,
        &format!("layer.set_blend_mode?layer={id}&mode=multiply"),
    );
    report.check(
        "A blend mode on the audio layer is refused",
        "\"Sound\" is an audio layer, which draws nothing, so there is nothing there to set.",
        refused,
    );

    run(&viewer, &format!("layer.toggle_visibility?layer={id}"));
    report.check(
        "The speaker is the switch every layer has: off",
        "false",
        shown_layer(&viewer, "Sound")["enabled"].to_string(),
    );
    run(&viewer, "edit.undo");
    report.check(
        "and Undo puts it back on",
        "true",
        shown_layer(&viewer, "Sound")["enabled"].to_string(),
    );

    report.check(
        "Relinking the sound to another WAV is done at once, as one entry to undo",
        "Relink pcm16_mono_48k.wav",
        propose_relink(&viewer, &Id::new(&asset_id), &[media("pcm8_mono_8k.wav")]),
    );
    report.check(
        "and the record points at the new file",
        "pcm8_mono_8k.wav",
        asset_named(&viewer, "pcm16_mono_48k.wav")["path"]
            .as_str()
            .and_then(|p| p.rsplit('/').next())
            .unwrap_or("(no path)"),
    );
    report.check(
        "Relinking it to a drawing is refused in words",
        "That is not a sound file this window plays. Nothing was changed.",
        propose_relink(&viewer, &Id::new(&asset_id), &[media("bg.png")]),
    );

    report.check(
        "A WAV that ends early is imported and the person is told",
        "Import cut_short.wav: sound. Drag it to the layer list, or press Make a layer, to hear it. The sound file \"cut_short.wav\" ends before its header says it does.",
        import(&viewer, &[media("cut_short.wav")]),
    );
    report.check(
        "A file that is not a WAV inside is imported, kept, and said to be silent",
        "Import not_a_wav.wav: sound. Drag it to the layer list, or press Make a layer, to hear it. This sound file could not be read, so its layer is silent. Relink the layer to a WAV file, or convert this one to WAV.",
        import(&viewer, &[media("not_a_wav.wav")]),
    );
    report.check(
        "both are in the notes, with their document 28 codes",
        "MEDIA_AUDIO_CUT_SHORT, MEDIA_AUDIO_UNREADABLE",
        held(&viewer)
            .notes
            .iter()
            .filter_map(|n| n.split('\t').nth(1)?.split(' ').next().map(str::to_string))
            .filter(|code| code.starts_with("MEDIA_AUDIO"))
            .collect::<Vec<_>>()
            .join(", "),
    );

    write_artifact(
        &report,
        "verification/B-20c_panel_table.md",
        "B-20c: reference audio in the window",
        AUDIO_PANEL_INTRO,
        AUDIO_PANEL_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

const PRECOMP_PANEL_INTRO: &[&str] = &[
    "D-67 decided what a composition layer is and B-18b built it in the core, checked pixel \
     by pixel in `verification/B-18b_precomp_table.md`. This is the window's half: \
     Pre-compose and Ctrl+Shift+C send `layer.precompose` with the chosen layers, the button \
     on a composition's row in the Project panel sends `layer.add_composition`, and \
     double-clicking a composition layer sends `composition.open`.",
    "Every row calls what the window calls, on `Fixtures/projects/cel_holds_project.json`, \
     and reads back what the page is given.",
];

const PRECOMP_PANEL_NOTES: &[&str] = &[
    "## What this does not cover\n\nWhat the layer looks like on the timeline and in the \
     panels, and whether the picture stays the same when layers are pre-composed. That is \
     `verification/B-18c_precomp_playtest.md`, for a person. Whether the pixels are right is \
     B-18b's table.",
];

const ADJUST_PANEL_INTRO: &[&str] = &[
    "D-66 decided what an adjustment layer is and B-17b built it in the core, checked pixel \
     by pixel in `verification/B-17b_adjust_table.md`. This is the window's half: the New \
     adjustment layer button and Ctrl+Alt+Y send `layer.add_adjustment`, which needs no \
     drawing chosen, and the layer then takes effects as any layer does and refuses a blend \
     mode.",
    "Every row calls what the window calls, on `Fixtures/projects/cel_holds_project.json`, \
     and reads back what the page is given.",
];

const SOLID_PANEL_INTRO: &[&str] = &[
    "D-74 decided what a solid is and B-23b built it in the core, checked pixel by pixel in \
     `verification/B-23b_solid_table.md`. This is the window's half: the New solid button, \
     its line in the timeline's right-click menu and the command palette, and Ctrl+Y send \
     `layer.add_solid`; the layer's panel shows a colour picker and a width and a height, \
     each of which sends `solid.set` on its own.",
    "Every row calls what the window calls, on `Fixtures/projects/cel_holds_project.json`, \
     and reads back what the page is given. The picker speaks the screen's 256 levels; the \
     page turns its choice into linear by document 21's curve before sending it, so a \
     picked #ff0000 arrives as 1,0,0 and #808080 as 0.215861.",
];

const SOLID_PANEL_NOTES: &[&str] = &[
    "## What this does not cover\n\nWhat the solid looks like in the viewer, on the timeline \
     and in the panels, and whether the picker opens and shows the colour. That is \
     `verification/B-23c_solid_playtest.md`, for a person. Whether the pixels are right is \
     B-23b's table.",
];

const NULL_PANEL_INTRO: &[&str] = &[
    "D-82 decided what a null is and B-26b built it in the core, checked pixel by pixel in \
     `verification/B-26b_null_table.md`. This is the window's half: the New null button, \
     its line in the timeline's right-click menu and the command palette, and \
     Ctrl+Alt+Shift+Y send `layer.add_null`; the picture is given each null's 100 by 100 \
     square to outline, since the null itself is never drawn.",
    "Every row calls what the window calls, on `Fixtures/projects/cel_holds_project.json`, \
     and reads back what the page is given. That project's drawings are not on disk, so \
     the drawn layer has no outline in these rows and the nulls do.",
];

const NULL_PANEL_NOTES: &[&str] = &[
    "## What this does not cover\n\nWhat the null looks like on the picture, the timeline \
     and in the panels, and whether dragging its square moves it. That is \
     `verification/B-26c_null_playtest.md`, for a person. Whether the frames are right is \
     B-26b's table.",
];

const PICKWHIP_PANEL_INTRO: &[&str] = &[
    "D-83 decided what the pick whip writes and B-27b built it in the core, checked against \
     FX-WHIP-001 to 020 in `verification/B-27b_pickwhip_table.md`. This is the window's \
     half: the spiral beside an expression sends `property.link` when it is let go on \
     another property's row, and the spiral on a layer's row or beside the Parent list \
     sends `layer.set_parent` when it is let go on another layer's row.",
    "Every row sends what the page sends, on `Fixtures/pickwhip/pickwhip_project.json`, \
     and reads back the text the property was given. The expected text is \
     `Fixtures/pickwhip/expected_pickwhip.json`'s, written by `tools/pickwhip_reference.py`.",
];

const PICKWHIP_PANEL_NOTES: &[&str] = &[
    "## What this does not cover\n\nThat the line follows the hand, that the right row \
     lights and the label says the right thing, and that Escape or letting go elsewhere \
     does nothing. That is `verification/B-27c_pickwhip_playtest.md`, for a person. Whether \
     the linked values are right is B-27b's table.",
];

const TIMESHEET_PANEL_INTRO: &[&str] = &[
    "D-84 decided how a cut is read from its XDTS timesheet and B-28b built it in the core, \
     checked against FX-XDTS-001 to 040 in `verification/B-28b_timesheet_table.md`. This is \
     the window's half: Import cut in the media bin asks for a folder and sends \
     `timesheet.import` with it, the new composition is put on screen, what the reading \
     noticed goes to the notes and the summary to the status line, and a layer's inspector \
     shows the column it came from.",
    "Every row sends what the window sends, on `Fixtures/projects/cel_holds_project.json` \
     and FX-XDTS-040, the sample cut, and reads back what the page is given. The cut's \
     drawings are on disk, so the picture's outlines say which layers show a drawing on a \
     frame.",
];

const TIMESHEET_PANEL_NOTES: &[&str] = &[
    "## What this does not cover\n\nThe folder dialog itself, what the cut looks like \
     playing, and whether the layer bars and the inspector line read well. That is \
     `verification/B-28c_timesheet_playtest.md`, for a person. Whether every cut is read \
     right is B-28b's table.",
];

const SHEET_PANEL_INTRO: &[&str] = &[
    "D-84a added a third tab to the timeline panel, Sheet, which shows the composition on \
     screen as a paper timesheet: frames down the page and a column for each layer that \
     shows drawings, the bottom of the stack on the left. The window works out every cell \
     from the same answer the picture is drawn from, and the page lays the cells out.",
    "The rows below import FX-XDTS-040, the sample cut, and read the grid the page is given. \
     Each frame is compared with document 25's table for that cut, which this check reads \
     from document 25 itself, so a change to either shows here. In the rows that show a \
     column as written, `|` is a hold, `x` is where a blank starts, `.` is a blank that goes \
     on, and `-` is outside the layer.",
];

const SHEET_PANEL_NOTES: &[&str] = &[
    "## What this does not cover\n\nWhat the tab looks like, whether the hold lines and the \
     shading read as a paper sheet does, and whether clicking a row goes to that frame. That \
     is `verification/B-28d_sheet_playtest.md`, for a person.",
];

const SHEET_TEXT_INTRO: &[&str] = &[
    "D-84c reads a timesheet's dialogue and camera columns when a cut is imported, keeps them \
     with the composition, and shows them in the Sheet as a paper sheet does: dialogue left \
     of the drawings and camera right of them, a line or instruction written on the frame it \
     starts with a line down the frames it lasts.",
    "D-84g puts an Action column on every Sheet with drawings, left of the dialogue, empty \
     until something is written in it.",
    "The rows below import FX-XDTS-040, the sample cut, and read the text cells the page is \
     given. Each column is compared frame by frame with document 25's paragraph on that \
     cut's Sheet, which this check reads from document 25 itself.",
];

const SHEET_TEXT_NOTES: &[&str] = &[
    "## What this does not cover\n\nWhere the columns sit on screen and whether they read \
     as a paper sheet does, and whether clicking a text cell goes to that frame. That is \
     `verification/B-28f_sheet_text_playtest.md`, for a person. Saving and reopening the \
     columns is checked in `verification/B-28b_timesheet_table.md`.",
];

const SHEET_PRINT_INTRO: &[&str] = &[
    "D-84d prints the Sheet as a paper timesheet through the system's print dialog, which can \
     also save a PDF. The window writes the whole printable page from the Sheet's own cells: \
     six seconds a page as two halves of three, a header on each, empty rows past the cut's \
     end and a heavy line under its last frame. D-84e lays it out as a studio's sheet: rows \
     counted from 1, each second's number beside its last row, an Action column, at least six \
     cel columns, and a choice of 6 or 3 seconds a page, in black or red. D-84f gives it a \
     title block of two rows: the name, episode, scene, cut and animator above, written in \
     Composition Settings, and the length, rate, sheet and memo below. D-84g prints what \
     is written in Action and circles every number of a key drawing.",
    "Each FX-PRINT case is read from document 25 itself, set up in the window, and checked \
     against the page the window writes: its header, its halves page by page, its columns and \
     its second numbers and its end line. A header written with labels is checked box by \
     box, by the box's class and the label the style prints on it. FX-PRINT-009 is undone \
     and FX-PRINT-010 saved, opened again, emptied and saved again, to read the file. Every \
     printed cell is then compared with the cell the Sheet shows on screen. FX-PRINT-011 \
     reads the Action column and every circled cell off the paper. The printable pages for \
     FX-PRINT-001 to 003, 005, 007, 009 and 011 are kept beside this table as \
     `B-28g_sheet_print_040.html`, `B-28g_sheet_print_300_frames.html`, \
     `B-28g_sheet_print_30_fps.html`, `B-28g_sheet_print_3_seconds.html`, \
     `B-28g_sheet_print_red.html`, `B-28g_sheet_print_title_block.html` and \
     `B-28g_sheet_print_keys.html`, to open in any browser.",
];

const SHEET_PRINT_NOTES: &[&str] = &[
    "## What this does not cover\n\nWhat the paper looks like, whether the print dialog \
     opens, and whether a PDF saved from it holds the pages. That is \
     `verification/B-28h_sheet_paper_playtest.md`, \
     `verification/B-28i_title_block_playtest.md` and \
     `verification/B-28j_action_keys_playtest.md`, for a person.",
];

const SHEET_WRITING_INTRO: &[&str] = &[
    "D-84b lets a cell of the Sheet be written as on paper: a number or a cross lasts until \
     the next thing written below it, and erasing one lets what is above it run on. The page \
     sends `exposure.write` with the layer, the frame and the mark, and the window works out \
     the layer's new exposures and makes them one entry in the history.",
    "Each row is one of document 25's FX-SHEET cases, read from document 25 itself. The \
     layer is set up as the case's Before line, the cell is written as its Do line says, and \
     the row reads the Sheet back as a line, as document 25 writes one: the drawing on each \
     frame, `x` for nothing and `-` outside the layer. It then counts the entries the write \
     put in the history, and undoes it to check that Before comes back. A case that is \
     refused, or that changes nothing, must put nothing in the history.",
    "D-84g adds FX-SHEET-017 to 024. Their Before and After say what the Action column shows \
     and which drawings are keys. Before's notes and keys are put there with the commands the \
     Sheet sends, `sheet.write_action` and `exposure.toggle_key`; the case's Do is played \
     the same way, and the row reads the Action column, the keys and the circled frames \
     back. FX-SHEET-019 saves the case to check the file keeps no Action column, and \
     FX-SHEET-024 saves, closes and opens it.",
];

const SHEET_WRITING_NOTES: &[&str] = &[
    "## What this does not cover\n\nTyping in the cells: choosing one, the digits, Enter, x, \
     Delete, the arrows and Escape, and the chosen cell moving down after a write. That is \
     `verification/B-28e_sheet_writing_playtest.md`, for a person, and for the Action column \
     and K, `verification/B-28j_action_keys_playtest.md`.",
];

const SHAPE_PANEL_INTRO: &[&str] = &[
    "D-78 decided what a shape layer is and B-25b built it in the core, checked pixel by \
     pixel in `verification/B-25b_shape_table.md`. This is the window's half: New shape \
     layer sends `layer.add_shape`; on a shape layer the pen, the rectangle and the ellipse \
     send `shape.add` rather than `mask.add`, a point dragged sends `shape.set_path`, the \
     stopwatch on a shape's row sends `shape.add_remove_key`, and the layer's panel sends \
     `shape.set` and `shape.delete`.",
    "Every row calls what the window calls, on `Fixtures/projects/cel_holds_project.json`, \
     and reads back what the page is given. Colours are linear, as document 21 holds them: \
     the grey a new closed shape gets is the screen's #808080, about 0.21586 linear.",
];

const SHAPE_PANEL_NOTES: &[&str] = &[
    "## What this does not cover\n\nWhat the shapes look like on the picture, whether the pen \
     pointer shows over the picture, and whether the tools feel right in the hand. That is \
     `verification/B-25c_shape_playtest.md`, for a person. Whether the pixels are right is \
     B-25b's table.\n\nOn a shape layer the picture's tools draw shapes, so a shape layer's \
     own masks are set in its panel but not drawn on the picture.",
];

const MASK_PANEL_INTRO: &[&str] = &[
    "D-77 decided what a mask is and B-24b built it in the core, checked pixel by pixel in \
     `verification/B-24b_mask_table.md`. This is the window's half: the pen (G), the \
     rectangle and the ellipse (Q) draw a mask and send `mask.add`; a point or a handle \
     dragged on the picture, or nudged with the arrow keys, sends `mask.set_path`; the \
     layer's panel shows every mask with its mode, opacity, feather, expansion, invert, on \
     and Delete, which send `mask.set` and `mask.delete`.",
    "Every row calls what the window calls, on `Fixtures/projects/cel_holds_project.json`, \
     and reads back what the page is given. The panel counts opacity in percent, as After \
     Effects does, and the window divides by a hundred before sending it, because D-77 \
     holds it from 0 to 1.",
];

const MASK_PANEL_NOTES: &[&str] = &[
    "## What this does not cover\n\nWhat a mask looks like on the picture and on the \
     timeline, whether the pen closes where the hand means it to, and whether a dragged \
     point lands where it was let go. That is `verification/B-24c_mask_playtest.md`, for a \
     person. Whether the pixels a mask keeps are right is B-24b's table. A path that moves \
     is B-24d, in `verification/B-24d_panel_table.md`.",
];

const MASK_KEY_PANEL_INTRO: &[&str] = &[
    "D-77 said a mask's path is a property like any other, and B-24d made it one: the \
     mask's row in the timeline carries a stopwatch and its keys, `mask.add_remove_key` \
     puts one down or takes one off at the frame on screen, and once a path has keys a \
     point dragged or nudged writes to the key under the playhead - making one there if \
     that frame has none, as After Effects does.",
    "Every row calls what the window calls, on \
     `Fixtures/projects/cel_holds_project.json`, and reads back what the page is given: \
     the keys off the document, and the shape at a frame off the same answer the boxes \
     come from, which is the outline the renderer actually drew.",
];

const MASK_KEY_PANEL_NOTES: &[&str] = &[
    "## What this does not cover\n\nWhether the diamond and the marks look right on the \
     timeline, and whether a path dragged frame by frame moves the way a person means it \
     to. That is `verification/B-24d_mask_playtest.md`, for a person. Whether the shape \
     between two keys is the right shape is the core's, in \
     `verification/B-24d_mask_key_table.md`.\n\nA mask key cannot be dragged along its row \
     or eased from the graph editor yet: `keyframe.move`, F9 and the graph take a \
     property's keys, and a path's key is not one of those. The stopwatch, the marks and \
     the frame the shape is set at are what B-24d builds.",
];

const PATH_POINT_PANEL_INTRO: &[&str] = &[
    "B-24d wrote down a limit: a point could not be added to or taken off a path that had \
     keys, because a path's points are the path's and every key holds them all. D-79 lifts \
     it by doing both at once - the point is cut into the base and into every key at the \
     same place along the same segment - and the page reaches it with the pen over the \
     path to add one, the pen over a point to take it off, and Delete to take off every \
     point chosen at once (D-80, B-24j: Alt and a click converts a point now, as it does \
     in After Effects, so the pen carries adding and taking off, where After Effects \
     keeps them).",
    "Every row calls what the window calls, on \
     `Fixtures/projects/cel_holds_project.json`, and reads back what the page is given: how \
     many points the base and each key hold, and the outline at a frame off the same answer \
     the boxes come from, which is what the renderer actually drew.",
];

const PATH_POINT_PANEL_NOTES: &[&str] = &[
    "## What this does not cover\n\nWhether the pen lands on the segment a person \
     meant, and whether the shape looks unmoved on the picture afterwards. That is \
     `verification/B-24h_path_point_playtest.md`, for a person. That the curve itself does \
     not move, to fourteen decimal places, and that the drawn pixels move by no more than \
     flattening, is the core's, in `verification/B-24h_path_point_table.md`.",
];

const PATH_KEY_PANEL_INTRO: &[&str] = &[
    "B-24d gave a mask's path keys and B-25c gave a shape's path the same, but a key once \
     set stayed where it was put and stayed linear. B-24e lets the page drag a path key \
     along its row, ease it with F9, Shift+F9 and Ctrl+F9, and hold it with Ctrl+Alt+G, by \
     naming the key by its frame in `mask.set` or `shape.set`: `to` moves it, `ease` eases \
     it by the rule F9 already uses for a property's keys, and `interp` sets hold, linear or \
     ease outright. Each is one entry to undo.",
    "Every row calls what the window calls, on \
     `Fixtures/projects/cel_holds_project.json`, and reads back what the page is given: the \
     keys off the document, and the outline at a frame off the answer the boxes come from, \
     which is what the renderer drew. The eased positions were worked out by hand from \
     document 20's curve, not read from the build.",
];

const PATH_KEY_PANEL_NOTES: &[&str] = &[
    "## What this does not cover\n\nWhether a mark follows the hand when dragged, lights \
     when chosen and changes shape when eased. That is \
     `verification/B-24e_path_key_playtest.md`, for a person.\n\nA path's keys are still \
     not in the graph editor: a path has no one number to draw a curve of, and After \
     Effects shows a path's speed only. Their curve can be eased and held from the \
     timeline, which is what this builds. (B-24f has since put them there, as the distance \
     the path has travelled: `verification/B-24f_panel_table.md`.)",
];

const PATH_KEY_GRAPH_INTRO: &[&str] = &[
    "B-24e let a path key be dragged and eased on its own. B-24f names a mask's path \
     `mask:<n>` and a shape's `shape:<n>` wherever a request names a property, as an \
     effect's setting is named `fx:`, so a path's key goes through the requests a property's \
     keys already go through: chosen with other keys and moved with them, moved by the arrow \
     keys, deleted, eased, held, copied and pasted, and drawn in the graph editor.",
    "A path is not one number, so the graph draws how far its points have travelled: the \
     mean distance its points move from one key to the next, added up key after key, and \
     part of the way along a segment by where the renderer puts the path. The speed graph is \
     then pixels a second, which is what After Effects draws for a path.",
    "Every row calls what the window calls, on \
     `Fixtures/projects/cel_holds_project.json`, and reads back what the page is given. The \
     distances were worked out by hand, not read from the build.",
];

const PATH_KEY_GRAPH_NOTES: &[&str] = &[
    "## What this does not cover\n\nWhether the marks light, follow the hand and open their \
     menu, and whether the graph draws the line: \
     `verification/B-24f_path_key_playtest.md`, for a person. So is letting a hold go \
     bringing back the ease the key had, which the page remembers and this table cannot \
     see; the table sends what the page sends then.\n\nA path key's value is its shape, \
     which is drawn, not typed or dragged up and down in the graph. Key speed and influence \
     typed in the menu's dialog, and the three kinds of bezier, are a number's and are not \
     offered for a path; its speed is pulled in the graph.",
];

const KEY_DIAMONDS_INTRO: &[&str] = &[
    "D-216 has a stretched layer's keys stretch with it, as After Effects' do: a key stored \
     at `k` plays on `in + (k - in) x stretch / 100`. B-150b built that in the picture; the \
     timeline still drew and named keys by their stored frame, so at 200% a key that plays on \
     frame 12 was drawn on frame 8. B-150c is the owner's \"Fix key diamonds\": the timeline \
     draws each key where it plays, and every request that names a key by its frame - a key \
     added with the playhead, dragged, eased, taken off, a mask's path key - is taken back to \
     the frame it is stored at, by document 20's rule, rounded half away from zero.",
    "Every row calls what the window calls, on `Fixtures/projects/unknown_effect_project.json` \
     with its layer moved to begin on frame 4, and reads back what is stored. The frames were \
     worked out by hand from the rule, not read from the build. The picture is not changed.",
];

const KEY_DIAMONDS_NOTES: &[&str] = &[
    "## What this does not cover\n\nWhether the diamonds are drawn on the right frame on \
     screen and follow the hand: `verification/B-150_playtest.md`, for a person. The page's \
     rule is checked here as text. A dragged key lands on the nearest frame a stored key can \
     play on, which at 200% is every other frame. The exposure blocks of a stretched drawn \
     layer are still drawn unstretched.",
];

const ADJUST_PANEL_NOTES: &[&str] = &[
    "## What this does not cover\n\nWhat the layer looks like on the timeline and in the \
     panels, and whether the picture changes as it should when its effects change. That is \
     `verification/B-17c_adjust_playtest.md`, for a person. Whether the pixels are right is \
     B-17b's table.",
];

/// P-19: the session log as the window keeps it, through the same `serve` the page's frames
/// go through, and the answer the panel reads.
#[test]
fn the_session_log_records_what_the_window_draws() {
    let mut report = Report { rows: Vec::new() };
    let viewer = Mutex::new(demo());
    let export = Mutex::new(Export::default());
    let session = Mutex::new(SessionLog::default());
    let rows = |s: &Mutex<SessionLog>| s.lock().unwrap().rows().len();
    let draw = |ask: Ask, quality: Option<PreviewQuality>| {
        let got = serve_logged(&viewer, &export, Some(&session), None, ask, quality);
        assert_eq!(got.status(), 200, "the frame was drawn");
        got.headers()["x-ms"].to_str().unwrap().to_string()
    };

    let source = include_str!("../main.rs");
    report.check(
        "the window keeps one log, made empty and switched off when it opens",
        "off, 0 rows",
        format!(
            "{}, {} rows{}",
            if session.lock().unwrap().is_on() {
                "on"
            } else {
                "off"
            },
            rows(&session),
            if source.contains(".manage(Mutex::new(SessionLog::default()))") {
                ""
            } else {
                "; not managed by the window"
            }
        ),
    );

    for f in 0..5 {
        draw(Ask::Frame(f), None);
    }
    report.check(
        "5 frames stepped with the log off",
        "0 rows",
        format!("{} rows", rows(&session)),
    );

    session.lock().unwrap().switch(true);
    let mut said = vec![draw(Ask::Play(0), Some(PreviewQuality::Draft))];
    for i in 1..10u64 {
        said.push(draw(Ask::At(i * 42), None));
    }
    let log = session.lock().unwrap();
    let played: Vec<&session_log::Row> = log.rows().iter().collect();
    report.check(
        "switched on, 10 frames played",
        "10 rows",
        format!("{} rows", played.len()),
    );
    report.check(
        "each played row says Draft, playing, reading ahead, no export",
        "10 of 10",
        format!(
            "{} of 10",
            played
                .iter()
                .filter(|r| r.quality == PreviewQuality::Draft
                    && r.playing
                    && r.read_ahead
                    && !r.exporting)
                .count()
        ),
    );
    report.check(
        "each row's time is the time the frame's own reply gave the page (P-15's x-ms)",
        "10 of 10",
        format!(
            "{} of 10",
            played
                .iter()
                .zip(&said)
                .filter(|(r, x)| format!("{:.1}", r.ms) == **x)
                .count()
        ),
    );
    report.check(
        "in each row the stages add up to no more than the frame's time",
        "10 of 10",
        format!(
            "{} of 10",
            played
                .iter()
                .filter(|r| r.stages.iter().map(|&(_, n)| n).sum::<u64>() as f64 / 1e6 <= r.ms)
                .count()
        ),
    );
    drop(log);

    draw(Ask::Frame(12), Some(PreviewQuality::Full));
    let last = session.lock().unwrap().rows().back().cloned().unwrap();
    report.check(
        "frame 12 stepped to at Full",
        "frame 12, Full, stepped, not read ahead",
        format!(
            "frame {}, {}, {}, {}",
            last.frame,
            last.quality.label(),
            if last.playing { "playing" } else { "stepped" },
            if last.read_ahead {
                "read ahead"
            } else {
                "not read ahead"
            }
        ),
    );

    let panel: serde_json::Value =
        serde_json::from_str(&session_log_json(&session.lock().unwrap())).unwrap();
    report.check(
        "the panel's answer: on, frames, how many slowest listed, the most stages any lists",
        "true, 11, 11, 3 or fewer",
        format!(
            "{}, {}, {}, {}",
            panel["on"],
            panel["frames"],
            panel["slowest"].as_array().unwrap().len(),
            if panel["slowest"]
                .as_array()
                .unwrap()
                .iter()
                .all(|r| r["stages"].as_array().unwrap().len() <= 3)
            {
                "3 or fewer"
            } else {
                "more than 3"
            }
        ),
    );

    session.lock().unwrap().switch(false);
    draw(Ask::Frame(3), None);
    report.check(
        "switched off, a frame adds no row and the stopwatch stops",
        "11 rows, stopwatch off",
        format!(
            "{} rows, stopwatch {}",
            rows(&session),
            if anime_compositor::perf::is_enabled() {
                "on"
            } else {
                "off"
            }
        ),
    );

    let heading = session_log_heading();
    let copy = session.lock().unwrap().to_markdown(&heading);
    report.check(
        "a saved copy is headed with the date, operating system, cores and build",
        "4 of 4, each in the copy",
        format!(
            "{} of 4, {}",
            heading
                .iter()
                .zip([
                    "Saved: 20",
                    "Operating system: ",
                    "Processor cores: ",
                    "Build: 0.1.0, "
                ])
                .filter(|(line, start)| line.starts_with(start))
                .count(),
            if heading.iter().all(|l| copy.contains(&format!("- {l}\n"))) {
                "each in the copy"
            } else {
                "missing from the copy"
            }
        ),
    );
    report.check(
        "and has one table line per row",
        "11",
        copy.lines().filter(|l| l.starts_with("| ")).count() - 1,
    );

    let page = include_str!("../../ui/index.html");
    for (what, text) in [
        ("a Session log button in the menu bar", "<button id=\"sessionlogbutton\""),
        ("a panel that is not modal, so the viewer can play under it", "$('sessionlog').show()"),
        ("the panel asks the window's log", "PROJECT + '/session-log'"),
        ("a switch", "Record where each frame's time goes"),
        ("the 20 slowest frames", "The 20 slowest frames"),
        ("Clear and Save a copy", "<button id=\"sessionlogclear\">Clear</button> <button id=\"sessionlogsave\">Save a copy&hellip;</button>"),
        ("its known limits, on the panel", "Kept in memory only, for the last 5,000 frames, and emptied when the window closes. Off every time the window opens."),
        ("and that it is not a benchmark", "A diagnostic, not a benchmark."),
        ("the command palette finds it", "['Session log', '', () => $('sessionlogbutton').click()]"),
    ] {
        report.check(what, "present", if page.contains(text) { "present" } else { "absent" });
    }

    write_artifact(
        &report,
        "verification/P-19_session_log_window_table.md",
        "P-19: the session log in the window",
        &[
            "The window's half of P-19. The core's half, including the 5,000-row cap and \
             byte-identical pictures with the log on and off, is \
             `verification/P-19_session_log_table.md`. Here the frames go through the window's \
             own `serve`, the function every frame the page shows comes from, with the reference \
             shot the window opens on.",
        ],
        &[
            "## What this does not cover\n\nThe panel as a person sees it, the Save dialog, and \
             closing and reopening the window. That is \
             `verification/P-19_session_log_playtest.md`.",
        ],
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(
        failed.is_empty(),
        "these checks failed: {failed:#?}\n{:#?}",
        report.rows
    );
}

/// B-609, D-738 (a): a shape drawn from its settings, from the window. The core's half is
/// `verification/B-609_parametric_shapes_table.md`.
#[test]
fn a_parametric_shape_is_drawn_and_switched_from_the_window() {
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    run(&viewer, "layer.add_shape");
    let id = shown_layer(&viewer, "Shape Layer 1")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    let first = |viewer: &Mutex<Viewer>| shown_layer(viewer, "Shape Layer 1")["shapes"][0].clone();
    run(
        &viewer,
        &format!("shape.add?layer={id}&parametric=polygon&position=200,150&radius=40&rotation=30"),
    );
    let s = first(&viewer);
    assert_eq!(s["parametric"]["type"], "polygon");
    assert_eq!(s["parametric"]["points"]["base"], 5.0);
    assert_eq!(s["closed"], true);
    assert_eq!(
        s["path"]["base"]["points"].as_array().map(Vec::len),
        Some(5)
    );
    assert!(run(
        &viewer,
        &format!(
            "shape.set_path?layer={id}&shape=0&frame=0&points=0,0,0,0,0,0;9,0,0,0,0,0;9,9,0,0,0,0"
        )
    )
    .contains("Convert to Bezier Path"));
    assert!(run(
        &viewer,
        &format!("shape.add_remove_key?layer={id}&shape=0&frame=0")
    )
    .contains("Convert to Bezier Path"));
    run(
        &viewer,
        &format!("shape.set?layer={id}&shape=0&polystar=star"),
    );
    let s = first(&viewer);
    assert_eq!(s["parametric"]["type"], "star");
    assert_eq!(s["parametric"]["inner_radius"]["base"], 20.0);
    assert_eq!(
        s["path"]["base"]["points"].as_array().map(Vec::len),
        Some(10)
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={id}&prop=shape:0:path_outer_radius&frame=0"),
    );
    assert_eq!(
        first(&viewer)["parametric"]["outer_radius"]["keyframes"]
            .as_array()
            .map(Vec::len),
        Some(1)
    );
    assert!(run(&viewer, &format!("shape.add?layer={id}&parametric=heart")).contains("heart"));
    assert!(run(
        &viewer,
        &format!("shape.add?layer={id}&parametric=polygon&points=2")
    )
    .contains("points"));
    run(&viewer, &format!("shape.set?layer={id}&shape=0&bezier=1"));
    let s = first(&viewer);
    assert!(s.get("parametric").is_none());
    assert_eq!(
        s["path"]["base"]["points"].as_array().map(Vec::len),
        Some(10)
    );
}

/// B-610, D-738 (b): shapes grouped, a group's numbers set and keyed, a grouped point dragged
/// where it is drawn, and Ungroup, from the window. The core's half is
/// `verification/B-610_shape_groups_table.md`.
#[test]
fn shapes_are_grouped_and_ungrouped_from_the_window() {
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    run(&viewer, "layer.add_shape");
    let id = shown_layer(&viewer, "Shape Layer 1")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    let layer = |viewer: &Mutex<Viewer>| shown_layer(viewer, "Shape Layer 1");
    let square = "points=0,0,0,0,0,0;20,0,0,0,0,0;20,20,0,0,0,0;0,20,0,0,0,0";
    for _ in 0..3 {
        run(&viewer, &format!("shape.add?layer={id}&{square}"));
    }
    run(&viewer, &format!("shape.group?layer={id}&shapes=0,1"));
    let l = layer(&viewer);
    assert_eq!(l["groups"][0]["name"], "Group 1");
    assert_eq!(l["shapes"][0]["group"], 0);
    assert_eq!(l["shapes"][1]["group"], 0);
    assert!(l["shapes"][2].get("group").is_none());
    assert!(
        run(&viewer, &format!("shape.group?layer={id}&shapes=0,2")).contains("different groups")
    );

    run(
        &viewer,
        &format!("property.set_base?layer={id}&prop=group:0:position&value=50,20&frame=0"),
    );
    assert_eq!(
        layer(&viewer)["groups"][0]["transform"]["position"]["base"].to_string(),
        "[50,20]"
    );
    let drawn = |viewer: &Mutex<Viewer>| {
        let body = boxes(viewer, 0, None).into_body();
        let answer: serde_json::Value =
            serde_json::from_slice(&body).expect("the boxes answer is JSON");
        answer["values"][&id].clone()
    };
    let now = drawn(&viewer);
    assert_eq!(
        now["shapes"][0][1],
        serde_json::json!([70.0, 20.0, 0.0, 0.0, 0.0, 0.0])
    );
    assert_eq!(
        now["shapes"][2][1],
        serde_json::json!([20.0, 0.0, 0.0, 0.0, 0.0, 0.0])
    );
    assert_eq!(now["group:0:position"], serde_json::json!([50.0, 20.0]));
    assert!(run(
        &viewer,
        &format!("property.set_base?layer={id}&prop=group:0:skew&value=90&frame=0")
    )
    .contains("skew"));

    // A point dragged to where it is drawn is kept where the group's own space has it.
    run(
        &viewer,
        &format!(
            "shape.set_path?layer={id}&shape=0&frame=0&points=50,20,0,0,0,0;80,20,0,0,0,0;70,40,0,0,0,0;50,40,0,0,0,0"
        ),
    );
    assert_eq!(
        layer(&viewer)["shapes"][0]["path"]["base"]["points"][1]["point"].to_string(),
        "[30,0]"
    );

    run(
        &viewer,
        &format!("keyframe.add_remove?layer={id}&prop=group:0:rotation&frame=0"),
    );
    assert_eq!(
        layer(&viewer)["groups"][0]["transform"]["rotation"]["keyframes"]
            .as_array()
            .map(Vec::len),
        Some(1)
    );

    run(&viewer, &format!("shape.ungroup?layer={id}&group=0"));
    let l = layer(&viewer);
    assert!(l["groups"].as_array().is_none_or(Vec::is_empty));
    assert!(l["shapes"][0].get("group").is_none());
    assert_eq!(
        drawn(&viewer)["shapes"][0][1],
        serde_json::json!([30.0, 0.0, 0.0, 0.0, 0.0, 0.0])
    );
    run(&viewer, "edit.undo");
    assert_eq!(layer(&viewer)["shapes"][1]["group"], 0);

    run(&viewer, &format!("shape.set?layer={id}&shape=2&group=0"));
    assert_eq!(layer(&viewer)["shapes"][2]["group"], 0);
    assert!(run(&viewer, &format!("shape.set?layer={id}&shape=2&group=4")).contains("group"));
    run(&viewer, &format!("shape.set?layer={id}&shape=2&group=none"));
    assert!(layer(&viewer)["shapes"][2].get("group").is_none());
}

/// B-612, D-738 (c): a group's repeater added, its composite set, its numbers set and keyed, and
/// removed, from the window. The core's half is `verification/B-612_shape_repeaters_table.md`.
#[test]
fn a_group_repeats_from_the_window() {
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    run(&viewer, "layer.add_shape");
    let id = shown_layer(&viewer, "Shape Layer 1")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    let layer = |viewer: &Mutex<Viewer>| shown_layer(viewer, "Shape Layer 1");
    let square = "points=0,0,0,0,0,0;20,0,0,0,0,0;20,20,0,0,0,0;0,20,0,0,0,0";
    run(&viewer, &format!("shape.add?layer={id}&{square}"));
    run(&viewer, &format!("shape.group?layer={id}&shapes=0"));
    assert!(run(
        &viewer,
        &format!("property.set_base?layer={id}&prop=group:0:repeater_copies&value=5&frame=0")
    )
    .contains("no repeater"));

    run(&viewer, &format!("shape.repeater?layer={id}&group=0&on=1"));
    let r = &layer(&viewer)["groups"][0]["repeater"];
    assert_eq!(r["composite"], "below");
    assert_eq!(r["copies"]["base"], 3.0);
    assert_eq!(r["position"]["base"].to_string(), "[100,0]");

    run(
        &viewer,
        &format!("shape.repeater?layer={id}&group=0&composite=above"),
    );
    assert_eq!(
        layer(&viewer)["groups"][0]["repeater"]["composite"],
        "above"
    );
    assert!(run(
        &viewer,
        &format!("shape.repeater?layer={id}&group=0&composite=up")
    )
    .contains("composite"));

    run(
        &viewer,
        &format!("property.set_base?layer={id}&prop=group:0:repeater_copies&value=5&frame=0"),
    );
    run(
        &viewer,
        &format!("property.set_base?layer={id}&prop=group:0:repeater_position&value=30,10&frame=0"),
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={id}&prop=group:0:repeater_rotation&frame=0"),
    );
    let r = &layer(&viewer)["groups"][0]["repeater"];
    assert_eq!(r["copies"]["base"], 5.0);
    assert_eq!(r["position"]["base"].to_string(), "[30,10]");
    assert_eq!(r["rotation"]["keyframes"].as_array().map(Vec::len), Some(1));
    let body = boxes(&viewer, 0, None).into_body();
    let answer: serde_json::Value =
        serde_json::from_slice(&body).expect("the boxes answer is JSON");
    assert_eq!(answer["values"][&id]["group:0:repeater_copies"], 5.0);
    // Out of range is the core's to refuse.
    assert!(!run(
        &viewer,
        &format!("property.set_base?layer={id}&prop=group:0:repeater_copies&value=5000&frame=0")
    )
    .is_empty());
    assert_eq!(
        layer(&viewer)["groups"][0]["repeater"]["copies"]["base"],
        5.0
    );

    run(&viewer, &format!("shape.repeater?layer={id}&group=0&on=0"));
    assert!(layer(&viewer)["groups"][0].get("repeater").is_none());
    run(&viewer, "edit.undo");
    assert_eq!(
        layer(&viewer)["groups"][0]["repeater"]["copies"]["base"],
        5.0
    );
}

/// B-613, D-738 (d): a group's path operations added, their choices and numbers set and keyed,
/// and removed, from the window. The core's half is
/// `verification/B-613_shape_operations_table.md`.
#[test]
fn a_group_operates_from_the_window() {
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    run(&viewer, "layer.add_shape");
    let id = shown_layer(&viewer, "Shape Layer 1")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    let layer = |viewer: &Mutex<Viewer>| shown_layer(viewer, "Shape Layer 1");
    let square = "points=0,0,0,0,0,0;20,0,0,0,0,0;20,20,0,0,0,0;0,20,0,0,0,0";
    run(&viewer, &format!("shape.add?layer={id}&{square}"));
    run(&viewer, &format!("shape.group?layer={id}&shapes=0"));
    assert!(run(
        &viewer,
        &format!("shape.operation?layer={id}&group=0&add=spin")
    )
    .contains("add=spin"));

    run(
        &viewer,
        &format!("shape.operation?layer={id}&group=0&add=round_corners"),
    );
    run(
        &viewer,
        &format!("shape.operation?layer={id}&group=0&add=offset"),
    );
    let ops = &layer(&viewer)["groups"][0]["operations"];
    assert_eq!(ops[0]["type"], "round_corners");
    assert_eq!(ops[0]["radius"]["base"], 10.0);
    assert_eq!(ops[1]["join"], "miter");
    assert_eq!(ops[1]["miter_limit"]["base"], 4.0);

    run(
        &viewer,
        &format!("shape.operation?layer={id}&group=0&op=1&choice=round"),
    );
    assert_eq!(
        layer(&viewer)["groups"][0]["operations"][1]["join"],
        "round"
    );
    // A word the choice does not have is the core's to refuse.
    assert!(!run(
        &viewer,
        &format!("shape.operation?layer={id}&group=0&op=1&choice=sharp")
    )
    .is_empty());
    assert_eq!(
        layer(&viewer)["groups"][0]["operations"][1]["join"],
        "round"
    );

    run(
        &viewer,
        &format!("property.set_base?layer={id}&prop=group:0:op0_radius&value=4&frame=0"),
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer={id}&prop=group:0:op1_amount&frame=0"),
    );
    let ops = &layer(&viewer)["groups"][0]["operations"];
    assert_eq!(ops[0]["radius"]["base"], 4.0);
    assert_eq!(
        ops[1]["amount"]["keyframes"].as_array().map(Vec::len),
        Some(1)
    );
    let body = boxes(&viewer, 0, None).into_body();
    let answer: serde_json::Value =
        serde_json::from_slice(&body).expect("the boxes answer is JSON");
    assert_eq!(answer["values"][&id]["group:0:op0_radius"], 4.0);
    assert!(!run(
        &viewer,
        &format!("property.set_base?layer={id}&prop=group:0:op0_radius&value=-1&frame=0")
    )
    .is_empty());
    assert_eq!(
        layer(&viewer)["groups"][0]["operations"][0]["radius"]["base"],
        4.0
    );

    run(
        &viewer,
        &format!("shape.operation?layer={id}&group=0&remove=0"),
    );
    let ops = &layer(&viewer)["groups"][0]["operations"];
    assert_eq!(ops.as_array().map(Vec::len), Some(1));
    assert_eq!(ops[0]["type"], "offset");
    run(&viewer, "edit.undo");
    assert_eq!(
        layer(&viewer)["groups"][0]["operations"][0]["type"],
        "round_corners"
    );
}

/// D-738 (d), B-614: Merge Paths is added from the same list, its mode picked from its own, a mode
/// it does not have refused, and undo takes it back.
#[test]
fn a_group_merges_from_the_window() {
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    run(&viewer, "layer.add_shape");
    let id = shown_layer(&viewer, "Shape Layer 1")["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    let layer = |viewer: &Mutex<Viewer>| shown_layer(viewer, "Shape Layer 1");
    let square = "points=0,0,0,0,0,0;20,0,0,0,0,0;20,20,0,0,0,0;0,20,0,0,0,0";
    let other = "points=10,10,0,0,0,0;30,10,0,0,0,0;30,30,0,0,0,0;10,30,0,0,0,0";
    run(&viewer, &format!("shape.add?layer={id}&{square}"));
    run(&viewer, &format!("shape.add?layer={id}&{other}"));
    run(&viewer, &format!("shape.group?layer={id}&shapes=0,1"));
    run(
        &viewer,
        &format!("shape.operation?layer={id}&group=0&add=merge"),
    );
    let merge = &layer(&viewer)["groups"][0]["operations"][0];
    assert_eq!(merge["type"], "merge");
    assert_eq!(merge["mode"], "add");

    run(
        &viewer,
        &format!("shape.operation?layer={id}&group=0&op=0&choice=subtract"),
    );
    assert_eq!(
        layer(&viewer)["groups"][0]["operations"][0]["mode"],
        "subtract"
    );
    assert!(!run(
        &viewer,
        &format!("shape.operation?layer={id}&group=0&op=0&choice=union")
    )
    .is_empty());
    assert_eq!(
        layer(&viewer)["groups"][0]["operations"][0]["mode"],
        "subtract"
    );

    run(&viewer, "edit.undo");
    assert_eq!(layer(&viewer)["groups"][0]["operations"][0]["mode"], "add");
    run(&viewer, "edit.undo");
    assert_eq!(
        layer(&viewer)["groups"][0]["operations"]
            .as_array()
            .map_or(0, Vec::len),
        0
    );
}
