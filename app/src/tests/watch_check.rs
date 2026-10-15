use super::render_queue_check::{folder, same};
use super::*;

#[test]
fn d252_paused_it_waits_then_writes_what_an_unpaused_export_writes() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().expect("the app crate has a parent directory").to_path_buf();
    let scratch = repo.join("target/d252_watch");
    let _ = std::fs::remove_dir_all(&scratch);
    let dir = |name: &str| {
        let d = scratch.join(name);
        std::fs::create_dir_all(&d).expect("make a scratch folder");
        d
    };
    let viewer = Mutex::new(demo());
    {
        // Sixty frames: enough to pause at 40 with frames still to come.
        let held = &mut *viewer.lock().unwrap();
        let composition = held.composition.clone();
        held.document.apply(Command::SetWorkArea { composition, start_frame: 0, end_frame_exclusive: 60 }).expect("set the work area");
    }
    let job = |into: &Path, missing: MissingSource| export_job(&viewer.lock().unwrap(), into, missing, OutputFormat::Png);
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut row = |what: &str, expected: &str, actual: String| rows.push((what.into(), expected.into(), actual));

    // What an export that is never paused writes.
    let (project, root, request) = job(&dir("whole"), MissingSource::RenderTransparent);
    run_export(&project, &root, &request, &AtomicBool::new(false));
    let whole = folder(&scratch.join("whole"));
    row("an export never paused writes sixty frames", "60", whole.len().to_string());

    let export = Mutex::new(Export::default());
    row("Pause with nothing running", "No export is running.", pause_export(&export));

    // The same export, paused once frame 40 is on the disk.
    let (project, root, request) = job(&dir("paused"), MissingSource::RenderTransparent);
    let cancel = Arc::new(AtomicBool::new(false));
    {
        let held = &mut *export.lock().unwrap();
        held.cancel = Some(Arc::clone(&cancel));
        held.done = Arc::new(AtomicUsize::new(0));
        held.total = 60;
        held.watch = Arc::new(export::Watch::default());
    }
    let (done, watch) = {
        let held = export.lock().unwrap();
        (Arc::clone(&held.done), Arc::clone(&held.watch))
    };
    let paused = scratch.join("paused");
    std::thread::scope(|s| {
        let writer = s.spawn(|| export::export_sequence_watched(&project, &root, &request, &cancel, &done, &watch));
        while done.load(Ordering::SeqCst) < 41 && !writer.is_finished() {
            std::thread::sleep(Duration::from_millis(1));
        }
        row("Pause is said", "Pausing after the frame being written. Press Pause again to go on.", pause_export(&export));
        std::thread::sleep(Duration::from_secs(1));
        let at_pause = folder(&paused).len();
        row(
            "it waits within a few frames of 40: 41 to 45 frames written",
            "41 to 45",
            if (41..=45).contains(&at_pause) { "41 to 45".into() } else { at_pause.to_string() },
        );
        std::thread::sleep(Duration::from_secs(5));
        row("for five seconds the folder does not grow", &format!("{at_pause} files"), format!("{} files", folder(&paused).len()));
        let seen = watching(&export);
        row("the window says it is paused", "true", seen["paused"].to_string());
        row(
            "the strip holds the five frames last written",
            &format!("{:?}", (at_pause as i64 - 5..at_pause as i64).collect::<Vec<_>>()),
            format!("{:?}", seen["frames"].as_array().map(|f| f.iter().filter_map(|n| n.as_i64()).collect::<Vec<_>>()).unwrap_or_default()),
        );
        // The picture shown for frame 40 against the file written for frame 40, shrunk the same way.
        let written = folder(&paused).get(40).map(|(name, _)| paused.join(name));
        let file = written
            .and_then(|path| anime_compositor::media::decode(&path).ok())
            .map(|decoded| export::small(&decoded.into_working(), 480));
        let shown = watched_frame(&export, 40);
        row(
            "the picture shown for frame 40 is the written frame 40, shrunk: no byte more than 1 apart",
            "480 x 270, at most 1 apart",
            match (file, shown) {
                (Some((fw, fh, f)), Some((w, h, p))) if (fw, fh) == (w, h) && f.len() == p.len() => {
                    let most = f.iter().zip(&p).map(|(a, b)| a.abs_diff(*b)).max().unwrap_or(0);
                    format!("{w} x {h}, {}", if most <= 1 { "at most 1 apart".to_string() } else { format!("{most} apart") })
                }
                (file, shown) => format!("file {:?}, shown {:?}", file.map(|f| (f.0, f.1)), shown.map(|s| (s.0, s.1))),
            },
        );
        row("Pause again goes on", "Going on.", pause_export(&export));
        writer.join().expect("the export thread");
    });
    row("after going on, the folder is the never-paused export's", &same(&whole, &whole), same(&folder(&paused), &whole));

    // The bar's orange frames: those an export that refuses missing drawings names.
    let (project, root, request) = job(&dir("blocked"), MissingSource::Block);
    let blocked = export::export_sequence(&project, &root, &request, &AtomicBool::new(false));
    let named = blocked
        .diagnostics
        .iter()
        .find(|d| d.id == DiagnosticId::ExportBlockedMissingMedia)
        .map_or("none".to_string(), |d| d.detail.clone());
    let missing: Vec<i32> = watching(&export)["missing"].as_array().map(|f| f.iter().filter_map(|n| n.as_i64()).map(|n| n as i32).collect()).unwrap_or_default();
    row(
        "the frames marked orange are the ones whose drawing is missing",
        &named,
        if missing.is_empty() { "none".into() } else { format!("Frames {}.", export::ranges(&missing)) },
    );

    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut text = format!(
        "# D-252: watch it write, and Pause\n\nWritten by `d252_paused_it_waits_then_writes_what_an_unpaused_export_writes` in \
         `app/src/main.rs`. The reference shot, frames 0 to 59, as PNG, with missing drawings written as transparent.\n\n\
         **{passed} of {} checks pass.**\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        rows.len()
    );
    for (what, expected, actual) in &rows {
        text.push_str(&format!("| {what} | {expected} | {actual} | {} |\n", if expected == actual { "PASS" } else { "FAIL" }));
    }
    std::fs::write(repo.join("verification/D-252_watch_and_pause_table.md"), text).expect("write the table");
    assert_eq!(passed, rows.len(), "see verification/D-252_watch_and_pause_table.md");
}
