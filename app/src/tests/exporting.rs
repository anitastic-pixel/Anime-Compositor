use super::*;
use anime_compositor::command::Command;

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .join(rel)
}

/// A scratch directory of this test's own, emptied first so a previous run's frames cannot
/// make a later one pass.
fn scratch(name: &str) -> PathBuf {
    let directory = crate::test_temp().join(name);
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("make the scratch directory");
    directory
}

/// The PNG files in a directory, in name order.
fn written(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}

/// The width and height a PNG declares, read out of its IHDR rather than trusted from the
/// request: the question is what is on the disk.
fn size_of(png: &Path) -> String {
    let bytes = std::fs::read(png).expect("read the frame that was written");
    let number = |at: usize| u32::from_be_bytes(bytes[at..at + 4].try_into().expect("four"));
    format!("{}x{}", number(16), number(20))
}

#[test]
fn what_the_window_exports_is_the_shot_it_had() {
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut check = |check: &str, expected: &dyn std::fmt::Display, actual: &dyn ToString| {
        rows.push((check.to_string(), expected.to_string(), actual.to_string()));
    };

    // A row about a sentence the person reads records the words themselves rather than a
    // bare true, which tells a reader nothing. The phrase asked for is the Expected column;
    // if it was in what the window said, that is the Actual column too, and if it was not,
    // the Actual column is everything the window did say instead. The core owns the exact
    // wording of its diagnostics and is checked on it elsewhere — pinning every word twice
    // would make an improvement to a message look like a regression here.
    let says = |phrase: &str, said: &str| {
        if said.contains(phrase) {
            phrase.to_string()
        } else {
            format!("the window said: {said}")
        }
    };

    // The project the window opens on: the reference shot, which has a deliberate gap —
    // layer 3 has no drawing 7, so frames 14 and 15 of every cycle ask for a drawing that is
    // not there. That gap is the whole reason document 07 blocks an export by default, and it
    // is the only fixture in this repository that can show the block happening.
    let mut viewer = demo();
    let into = scratch("anime_compositor_b10_export");

    // ---- what the window asks for -----------------------------------------------------------
    let (snapshot, root, request) =
        export_job(&viewer, &into, MissingSource::Block, OutputFormat::Png);
    check(
        "the range offered is the work area, first to last frame inclusive",
        &"0 to 239",
        &format!("{} to {}", request.first_frame, request.last_frame),
    );
    check(
        "the files are named for the project and carry the frame number, four digits wide",
        &"the reference shot_%04d.png",
        &request.naming,
    );
    check(
        "at eight bits with straight alpha, which is what the export fixtures were written \
         with",
        &"Eight, Straight",
        &format!("{:?}, {:?}", request.depth, request.alpha),
    );

    // ---- the snapshot -------------------------------------------------------------------------
    // B-10 asks for an *immutable* export snapshot. The check is that editing the open
    // document after the job was made does not reach the job: what is written is the shot as
    // it was when the person asked.
    let composition = viewer.composition.clone();
    let layer_id = viewer.document.project().compositions[0].layer_order()[0].clone();
    let before = snapshot.compositions[0]
        .layer(&layer_id)
        .expect("the layer the order names")
        .name
        .clone();
    viewer
        .document
        .apply(Command::RenameLayer {
            composition,
            layer_id: layer_id.clone(),
            name: "Renamed while the export was running".to_string(),
        })
        .expect("rename the layer");
    check(
        "the open project has been changed since the export was asked for",
        &"Renamed while the export was running",
        &viewer.document.project().compositions[0]
            .layer(&layer_id)
            .expect("the layer the order names")
            .name,
    );
    check(
        "and the export is still writing the project as it was when it was asked for",
        &before,
        &snapshot.compositions[0]
            .layer(&layer_id)
            .expect("the layer the order names")
            .name,
    );

    // ---- a missing drawing, with document 07's default ---------------------------------------
    // Two frames rather than 240: which frames are asked for is checked above, and what a
    // whole shot does is `B-10_full_shot_table.md`. Frames 14 and 15 are the two the gap
    // falls on.
    let short = |missing| {
        let (_, _, mut request) = export_job(&viewer, &into, missing, OutputFormat::Png);
        request.first_frame = 14;
        request.last_frame = 15;
        request
    };
    let blocked = short(MissingSource::Block);
    let said = run_export(&snapshot, &root, &blocked, &AtomicBool::new(false));
    check(
        "by default a frame whose drawing is missing stops the export before anything is \
         written",
        &"Nothing was exported.",
        &says("Nothing was exported.", &said),
    );
    check(
        "and the person is told how many frames it is",
        &"2 of the 2 frames asked for have a drawing that is missing",
        &says(
            "2 of the 2 frames asked for have a drawing that is missing",
            &said,
        ),
    );
    check(
        "and what to do about it",
        &"Relink or restore the missing drawings",
        &says("Relink or restore the missing drawings", &said),
    );
    check(
        "nothing at all is on the disk",
        &"[]",
        &format!("{:?}", written(&into)),
    );

    // ---- the same two frames, written on purpose ----------------------------------------------
    let anyway = short(MissingSource::RenderTransparent);
    let said = run_export(&snapshot, &root, &anyway, &AtomicBool::new(false));
    check(
        "asked to write them anyway, the window writes them and says where",
        &format!("Exported 2 frames into {}.", into.display()),
        &says(
            &format!("Exported 2 frames into {}.", into.display()),
            &said,
        ),
    );
    check(
        "the two files are named for the frames that were asked for",
        &"[\"the reference shot_0014.png\", \"the reference shot_0015.png\"]",
        &format!("{:?}", written(&into)),
    );
    check(
        "an exported frame is full size, whatever resolution the preview was showing",
        &"1920x1080",
        &size_of(&into.join("the reference shot_0014.png")),
    );
    check(
        "and the drawing that is missing is still reported rather than passed over in silence",
        &"Frame 14 exposes drawing 7 of layer3_%03d.png, which is missing.",
        &says(
            "Frame 14 exposes drawing 7 of layer3_%03d.png, which is missing.",
            &said,
        ),
    );
    check(
        "once for each frame it was missing on, not once for the export",
        &"Frame 15 exposes drawing 7 of layer3_%03d.png, which is missing.",
        &says(
            "Frame 15 exposes drawing 7 of layer3_%03d.png, which is missing.",
            &said,
        ),
    );

    // ---- cancelling ----------------------------------------------------------------------------
    // R-09 asks for cancellation between frames. Asked before the first one, the answer is
    // that nothing was written and the job does not claim to have succeeded.
    let elsewhere = scratch("anime_compositor_b10_cancel");
    let mut cancelled = short(MissingSource::RenderTransparent);
    cancelled.output_dir = elsewhere.clone();
    let said = run_export(&snapshot, &root, &cancelled, &AtomicBool::new(true));
    check(
        "a cancelled export says how far it got, and does not claim to have succeeded",
        &"Export stopped at your request after 0 of 2 frames",
        &says("Export stopped at your request after 0 of 2 frames", &said),
    );
    // The sentence above is the core's diagnostic. This one is the window's own opening line,
    // and it is checked separately because the two can disagree: a window that says "Exported
    // 0 frames" and then reports a cancellation underneath has told the person two different
    // things about the same job, and the diagnostic row alone cannot see it.
    // This row's folder is a second scratch directory, and `write_artifact` below hides only
    // the first one, so the machine-specific part is replaced here instead. Without this the
    // committed table would carry the path of whichever machine last ran the test.
    let finished = format!("The 0 frames that finished are in {}.", elsewhere.display());
    let anywhere =
        |text: &str| text.replace(&elsewhere.display().to_string(), "<a temporary directory>");
    check(
        "and the window's own first sentence says what is there, not that it exported them",
        &anywhere(&finished),
        &anywhere(&says(&finished, &said)),
    );
    check(
        "and left no half-written file behind",
        &"[]",
        &format!("{:?}", written(&elsewhere)),
    );
    check(
        "asking a window with nothing running to cancel is not an error either",
        &"No export is running.",
        &match Export::default().cancel {
            Some(_) => "a job was running".to_string(),
            None => "No export is running.".to_string(),
        },
    );

    // ---- somewhere the frames cannot go ---------------------------------------------------------
    let nowhere = into.join("no_such_directory");
    let mut refused = short(MissingSource::RenderTransparent);
    refused.output_dir = nowhere.clone();
    let said = run_export(&snapshot, &root, &refused, &AtomicBool::new(false));
    check(
        "an export into a folder that is not there says how far it got before it stopped",
        &"The export stopped on a problem after 0 of 2 frames",
        &says("The export stopped on a problem after 0 of 2 frames", &said),
    );
    let could_not = format!(
        "Frame 14 could not be written to {}. Check that the folder exists, is writable and \
         has room",
        nowhere.join("the reference shot_0014.png").display()
    );
    check(
        "and names the file it could not write, rather than only that something went wrong",
        &could_not,
        &says(&could_not, &said),
    );
    check(
        "and the folder is still not there: nothing was created to hold a failure",
        &false,
        &nowhere.exists(),
    );

    write_artifact(&rows, &into.display().to_string());
    let failed: Vec<&(String, String, String)> =
        rows.iter().filter(|(_, e, a)| e != a).collect();
    assert!(
        failed.is_empty(),
        "{} of {} checks failed, see verification/B-10_export_table.md: {:#?}",
        failed.len(),
        rows.len(),
        failed
    );
}

fn write_artifact(rows: &[(String, String, String)], scratch: &str) {
    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut out = String::new();
    out.push_str("# B-10, exporting from the window\n\n");
    out.push_str(
        "The renderer could export a shot and the window could not ask it to. It can now, and \
         this is what the asking does. Produced by `cargo test -p anime_compositor_app`, from \
         `app/src/main.rs`.\n\n",
    );
    out.push_str(
        "What the frames themselves look like is not this table's question — `T-08_export_\
         table.md` checks the pixels and the naming to the frame, and `B-10_full_shot_table.md` \
         exports the whole shot twice and requires the two to be identical byte for byte. What \
         is new here is everything between the button and that: **which frames the window asks \
         for, what it names them, that what is written is the project as it was when the \
         person asked rather than as it is when the job finishes, and that a refusal, a \
         cancellation and a failure each arrive in words instead of in silence.**\n\n",
    );
    out.push_str(
        "The project is the reference shot, chosen because of its deliberate gap: layer 3 has \
         no drawing 7, so frames 14 and 15 ask for a drawing that is not there. Document 07 \
         blocks an export on that by default, and the checkbox beside the Export button is the \
         person overriding it in front of a sentence that says what the override does.\n\n",
    );
    out.push_str(
        "Rows about a sentence quote it. The Expected column is the words that had to reach \
         the person; the Actual column is those words if they did, and everything the window \
         said instead if they did not.\n\n",
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
        "\n## What this does not cover\n\nThe folder dialog, which belongs to the operating \
         system and which a test has no hands to answer, so what is checked here begins at the \
         folder the person chose.\n\n\
         **Two frames, not two hundred and forty.** Which frames the window asks for is a row \
         above; what a whole shot does is `B-10_full_shot_table.md`, which runs for four \
         minutes and is not part of an ordinary build.\n\n\
         **There is no progress bar.** R-09 asks for cancellation between frames and for \
         failure to be reported, and both are here; it does not ask for a count of frames as \
         they are written, and the core has no hook to report one without a change to its \
         signature that only a window wants. What the window shows while a job runs is what it \
         is doing and a Cancel button, and what it shows afterwards is the core's report.\n\n\
         **A second export while one is running** is refused by `start_export`, which needs a \
         running application to reach — the refusal is one branch above the part this table \
         can call.\n\n\
         Where a row says *a temporary directory*, the real value was this machine's scratch \
         directory, which differs on every machine and every run.\n",
    );
    std::fs::write(repo("verification/B-10_export_table.md"), out).expect("write the artifact");
}
