use super::*;

/// Every file in a folder, by name, with its bytes.
pub(super) fn folder(dir: &Path) -> Vec<(String, Vec<u8>)> {
    let mut files: Vec<(String, Vec<u8>)> = std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|e| {
                    (
                        e.file_name().to_string_lossy().into_owned(),
                        std::fs::read(e.path()).unwrap_or_default(),
                    )
                })
                .collect()
        })
        .unwrap_or_default();
    files.sort();
    files
}

pub(super) fn same(a: &[(String, Vec<u8>)], b: &[(String, Vec<u8>)]) -> String {
    if a.is_empty() {
        "empty".into()
    } else if a == b {
        format!(
            "{} {}, every byte the same",
            a.len(),
            if a.len() == 1 { "file" } else { "files" }
        )
    } else {
        format!("{} files against {}, different", a.len(), b.len())
    }
}

/// An MP4 carries the second it was written, twice in each of its `mvhd`, `tkhd` and `mdhd`
/// boxes, so two films of the same frames written a second apart differ in those bytes alone.
/// They are blanked here, and nothing else is.
fn unstamped(mut files: Vec<(String, Vec<u8>)>) -> Vec<(String, Vec<u8>)> {
    for (_, bytes) in &mut files {
        for name in [&b"mvhd"[..], b"tkhd", b"mdhd"] {
            if let Some(at) = bytes.windows(4).position(|w| w == name) {
                if bytes.get(at + 4) == Some(&0) && bytes.len() >= at + 16 {
                    bytes[at + 8..at + 16].fill(0);
                }
            }
        }
    }
    files
}

fn statuses(export: &Mutex<Export>, viewer: &Mutex<Viewer>) -> String {
    queue_list(export, viewer)
        .as_array()
        .map(|rows| {
            rows.iter()
                .map(|r| {
                    format!(
                        "{} {}",
                        if r["ticked"] == true { "[x]" } else { "[ ]" },
                        r["status"].as_str().unwrap_or("?")
                    )
                })
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_default()
}

#[test]
fn d251_the_queue_writes_what_one_export_writes() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .to_path_buf();
    let scratch = repo.join("target/d251_queue");
    let _ = std::fs::remove_dir_all(&scratch);
    let dir = |name: &str| {
        let d = scratch.join(name);
        std::fs::create_dir_all(&d).expect("make a scratch folder");
        d
    };
    let viewer = Mutex::new(demo());
    {
        // Twelve frames, so the check takes seconds rather than minutes.
        let held = &mut *viewer.lock().unwrap();
        let composition = held.composition.clone();
        held.document
            .apply(Command::SetWorkArea {
                composition,
                start_frame: 0,
                end_frame_exclusive: 12,
            })
            .expect("set the work area");
    }
    let write = MissingSource::RenderTransparent;
    let standard = ExportChoices::default();
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut row = |what: &str, expected: &str, actual: String| {
        rows.push((what.into(), expected.into(), actual))
    };

    // What one Export writes today, for each of the three formats.
    let once = |format: OutputFormat, into: &Path| {
        let (project, root, mut request) = export_job(&viewer.lock().unwrap(), into, write, format);
        request.choices = standard;
        run_export(&project, &root, &request, &AtomicBool::new(false));
        folder(into)
    };
    let one_png = once(OutputFormat::Png, &dir("one_png"));
    let one_gif = once(OutputFormat::Gif, &dir("one_gif"));
    let one_mp4 = once(OutputFormat::Mp4, &dir("one_mp4"));
    row(
        "one Export writes twelve PNG frames, one GIF and one MP4",
        "12, 1, 1",
        format!("{}, {}, {}", one_png.len(), one_gif.len(), one_mp4.len()),
    );

    let export = Mutex::new(Export::default());
    for (format, name) in [
        (OutputFormat::Png, "png"),
        (OutputFormat::Gif, "gif"),
        (OutputFormat::Mp4, "mp4"),
        (OutputFormat::Png, "unticked"),
    ] {
        queue_add(
            &export,
            &viewer.lock().unwrap(),
            &dir(name),
            write,
            format,
            standard,
        );
    }
    let ids: Vec<u64> = queue_list(&export, &viewer)
        .as_array()
        .map(|r| r.iter().filter_map(|r| r["id"].as_u64()).collect())
        .unwrap_or_default();
    row("four rows added", "4", ids.len().to_string());
    let fourth = ids.get(3).copied().unwrap_or(0);
    queue_change(
        &export,
        &viewer,
        Some(&format!("do=tick&row={fourth}&on=0")),
    );
    row(
        "before Render: three ticked, one not, all queued",
        "[x] Queued, [x] Queued, [x] Queued, [ ] Queued",
        statuses(&export, &viewer),
    );

    let said = render_queue(&export, &AtomicBool::new(false));
    row(
        "Render writes the ticked rows in turn",
        "Rendered 3 of 3 rows.",
        said,
    );
    row(
        "row 1, PNG, is what one Export writes",
        &same(&one_png, &one_png),
        same(&folder(&scratch.join("png")), &one_png),
    );
    row(
        "row 2, GIF, is what one Export writes",
        &same(&one_gif, &one_gif),
        same(&folder(&scratch.join("gif")), &one_gif),
    );
    let one_mp4 = unstamped(one_mp4);
    row(
        "row 3, MP4, is what one Export writes, but for the second each was written at",
        &same(&one_mp4, &one_mp4),
        same(&unstamped(folder(&scratch.join("mp4"))), &one_mp4),
    );
    row(
        "the unticked row writes nothing",
        "empty",
        same(&folder(&scratch.join("unticked")), &one_png),
    );
    row(
        "after: the three are done and unticked, the fourth still waits",
        "[ ] Done, [ ] Done, [ ] Done, [ ] Queued",
        statuses(&export, &viewer),
    );

    // Stop during row 2.
    let stopping = Mutex::new(Export::default());
    for name in ["stop_1", "stop_2", "stop_3"] {
        queue_add(
            &stopping,
            &viewer.lock().unwrap(),
            &dir(name),
            write,
            OutputFormat::Png,
            standard,
        );
    }
    let second = queue_list(&stopping, &viewer)
        .as_array()
        .and_then(|r| r.get(1))
        .and_then(|r| r["id"].as_u64())
        .unwrap_or(0);
    let cancel = AtomicBool::new(false);
    let finished = AtomicBool::new(false);
    let said = std::thread::scope(|s| {
        s.spawn(|| {
            while !finished.load(Ordering::SeqCst) {
                {
                    let held = stopping.lock().unwrap();
                    if held.row == Some(second) && held.done.load(Ordering::SeqCst) >= 2 {
                        cancel.store(true, Ordering::SeqCst);
                        return;
                    }
                }
                std::thread::sleep(Duration::from_millis(2));
            }
        });
        let said = render_queue(&stopping, &cancel);
        finished.store(true, Ordering::SeqCst);
        said
    });
    let count = |name: &str| folder(&scratch.join(name)).len();
    row(
        "Stop during row 2 is said",
        "Stopped during row 2 of 3.",
        said,
    );
    row(
        "row 1 is whole",
        &same(&one_png, &one_png),
        same(&folder(&scratch.join("stop_1")), &one_png),
    );
    row(
        "row 2 keeps the frames that finished, fewer than twelve",
        "some, fewer than 12",
        match count("stop_2") {
            1..=11 => "some, fewer than 12".into(),
            n => n.to_string(),
        },
    );
    row(
        "row 3 is not started",
        "0 files",
        format!("{} files", count("stop_3")),
    );
    row(
        "and the rows say so",
        "[ ] Done, [x] Stopped, [x] Not started",
        statuses(&stopping, &viewer),
    );

    // Order, removal, and edits made after a row was added.
    let listed = |e: &Mutex<Export>| {
        queue_list(e, &viewer)
            .as_array()
            .map(|r| {
                r.iter()
                    .filter_map(|r| r["id"].as_u64())
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .unwrap_or_default()
    };
    let before = listed(&export);
    queue_change(
        &export,
        &viewer,
        Some(&format!("do=move&row={fourth}&to=0")),
    );
    row(
        "a row dragged to the top is first",
        &format!(
            "{fourth} {}",
            before.split(' ').take(3).collect::<Vec<_>>().join(" ")
        ),
        listed(&export),
    );
    queue_change(&export, &viewer, Some(&format!("do=remove&row={}", ids[0])));
    row(
        "a row removed with its × is gone",
        "3",
        queue_list(&export, &viewer)
            .as_array()
            .map_or(0, Vec::len)
            .to_string(),
    );
    row(
        "a row added before any edit is not marked",
        "false",
        queue_list(&export, &viewer)[0]["stale"].to_string(),
    );
    {
        let held = &mut *viewer.lock().unwrap();
        let composition = held.composition.clone();
        held.document
            .apply(Command::RenameLayer {
                composition,
                layer_id: Id::new("layer-2"),
                name: "Edited after".into(),
            })
            .expect("rename");
    }
    row(
        "after an edit, the row is marked as added before it",
        "true",
        queue_list(&export, &viewer)[0]["stale"].to_string(),
    );
    queue_change(&export, &viewer, Some(&format!("do=refresh&row={fourth}")));
    row(
        "Refresh takes the project again",
        "false",
        queue_list(&export, &viewer)[0]["stale"].to_string(),
    );

    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut text = format!(
        "# D-251: the render queue\n\nWritten by `d251_the_queue_writes_what_one_export_writes` in `app/src/main.rs`. \
         The reference shot, frames 0 to 11, with missing drawings written as transparent. \"One Export\" is the \
         window's Export of today, into a folder of its own.\n\n**{passed} of {} checks pass.**\n\n\
         | Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        rows.len()
    );
    for (what, expected, actual) in &rows {
        text.push_str(&format!(
            "| {what} | {expected} | {actual} | {} |\n",
            if expected == actual { "PASS" } else { "FAIL" }
        ));
    }
    std::fs::write(repo.join("verification/D-251_render_queue_table.md"), text)
        .expect("write the table");
    assert_eq!(
        passed,
        rows.len(),
        "see verification/D-251_render_queue_table.md"
    );
}
