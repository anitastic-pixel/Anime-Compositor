//! P-19: the session log, checked without a window.
//!
//! Each frame here is drawn the way `serve` in `app/src/main.rs` draws one: the stopwatch
//! collects on this thread, the frame is rendered through the viewer's cache, the next frame's
//! drawings are read ahead on a thread of their own, the pixels are encoded, and a row is
//! written. The read-ahead is joined only after the row is written, so its work is running
//! beside the frame the whole time, which is the case the per-thread collection is for.
//!
//! `p19_session_log` runs in the ordinary suite and asserts only what is true on any machine. It
//! writes `verification/P-19_session_log_table.md`. `p19_session_log_cost` is a measurement, run
//! deliberately with `--release --ignored`; it writes `verification/P-19_session_log_cost_table.md`
//! and an example saved copy, `verification/P-19_session_log_example.md`.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{Id, Project};
use anime_compositor::perf;
use anime_compositor::persist;
use anime_compositor::preview::{self, Playback, PreviewQuality};
use anime_compositor::session_log::{Row, SessionLog, KEEPS};
use anime_compositor::sha256;

mod common;
use common::repo;

const COMP: &str = "comp-reference-shot";

fn reference() -> (Project, PathBuf) {
    let loaded = persist::load(&repo("verification/B-08a_project.json"))
        .unwrap_or_else(|d| panic!("the reference shot: {}", d.message));
    (loaded.document.project().clone(), repo("Fixtures/reference_shot"))
}

/// One frame, as the window's `serve` makes it. Returns the SHA-256 of the pixels and the
/// frame's time as the window measures it, which stops before the read-ahead is waited for.
#[allow(clippy::too_many_arguments)]
fn draw(
    project: &Project,
    root: &Path,
    frame: i32,
    dropped: u32,
    ahead: Option<i32>,
    quality: PreviewQuality,
    cache: &Arc<Mutex<CelCache>>,
    log: &mut SessionLog,
) -> (String, f64) {
    let began = Instant::now();
    let logging = log.is_on();
    if logging {
        perf::begin_capture();
    }
    let comp = Id::new(COMP);
    let mut frame_log = FrameLog::new(3);
    let (buffer, counts) = {
        let mut c = cache.lock().unwrap();
        let before = (c.hits(), c.misses(), c.effect_hits());
        let buffer = preview::preview_frame_cached(
            project,
            &comp,
            frame,
            root,
            quality,
            DEFAULT_TILE_SIZE,
            &mut frame_log,
            &mut c,
        )
        .unwrap_or_else(|d| panic!("frame {frame}: {}", d.message));
        let after = (c.hits(), c.misses(), c.effect_hits());
        (buffer, (after.0 - before.0, after.1 - before.1, after.2 - before.2))
    };
    let reading = ahead.map(|next| {
        preview::read_ahead(project.clone(), comp.clone(), next, root.to_path_buf(), cache.clone())
    });
    let pixels = buffer.to_srgb8_straight();
    let ms = began.elapsed().as_secs_f64() * 1000.0;
    let stages = perf::end_capture();
    if logging {
        log.push(Row {
            since_on_ms: 0,
            frame,
            quality,
            playing: ahead.is_some(),
            ms,
            stages: stages.expect("the capture begun above"),
            dropped,
            read_ahead: ahead.is_some(),
            exporting: false,
            cel_hits: counts.0,
            cel_misses: counts.1,
            effect_hits: counts.2,
            warnings: frame_log
                .finish()
                .iter()
                .map(|d| format!("{}: {}", d.id.as_str(), d.message))
                .collect(),
        });
    }
    if let Some(h) = reading {
        h.join().unwrap();
    }
    (sha256::hex(&pixels), ms)
}

/// Play `frames` frames of the reference shot from its first, as the 24 fps clock would hand
/// them out with every frame arriving on time. Returns each frame's number and picture.
fn play(
    project: &Project,
    root: &Path,
    frames: usize,
    quality: PreviewQuality,
    log: &mut SessionLog,
) -> Vec<(i32, String)> {
    let comp = project.composition(&Id::new(COMP)).expect("the reference composition");
    let (first, last) = comp.work_frames();
    let mut clock = Playback::new(first, last, comp.frame_rate);
    let cache = Arc::new(Mutex::new(CelCache::viewer()));
    (0..frames)
        .map(|i| {
            let shown = clock.at(Duration::from_secs_f64(i as f64 / 24.0));
            let next = clock.after(shown.frame);
            let (sha, _) =
                draw(project, root, shown.frame, shown.skipped, Some(next), quality, &cache, log);
            (shown.frame, sha)
        })
        .collect()
}

fn row(frame: i32) -> Row {
    Row {
        since_on_ms: 0,
        frame,
        quality: PreviewQuality::Draft,
        playing: true,
        ms: frame as f64,
        stages: Vec::new(),
        dropped: 0,
        read_ahead: true,
        exporting: false,
        cel_hits: 0,
        cel_misses: 0,
        effect_hits: 0,
        warnings: Vec::new(),
    }
}

/// Every check in one test, because the stopwatch's switch is shared by the whole process and
/// the checks turn it on and off.
#[test]
fn p19_session_log() {
    let (project, root) = reference();
    let played = 48;
    let mut s = String::from(
        "# P-19: the session log, checked\n\n\
         Written by `tests/p19_session_log.rs` under `cargo test --test p19_session_log`. Each \
         frame is drawn the way the window draws one: the viewer's cache, the next frame's \
         drawings read ahead beside it, then encoded. Timings are not asserted and not printed \
         here; they differ from run to run and machine to machine (document 12). The cost of the \
         log is measured in `verification/P-19_session_log_cost_table.md`.\n\n\
         | # | Check | Result |\n|---|---|---|\n",
    );
    let mut n = 0;
    let mut check = |s: &mut String, what: &str, result: String| {
        n += 1;
        let _ = writeln!(s, "| {n} | {what} | {result} |");
    };

    // 1. Off: nothing is recorded and the stopwatch stays off.
    let mut log = SessionLog::default();
    assert!(!log.is_on(), "a new log must start switched off (ADR-012)");
    let off = play(&project, &root, played, PreviewQuality::Draft, &mut log);
    assert_eq!(log.rows().len(), 0, "the log recorded rows while switched off");
    assert!(!perf::is_enabled(), "the stopwatch was on with the log switched off");
    check(
        &mut s,
        "A new log is off. Playing the reference shot for 48 frames with it off records nothing, \
         and the stopwatch stays off.",
        "0 rows; stopwatch off".into(),
    );

    // 2. On: one row per frame drawn.
    log.switch(true);
    assert!(perf::is_enabled(), "switching the log on did not start the stopwatch");
    let on = play(&project, &root, played, PreviewQuality::Draft, &mut log);
    assert_eq!(log.rows().len(), played, "{played} frames drawn, rows kept");
    check(
        &mut s,
        "Switched on, playing the same 48 frames records one row for each.",
        format!("{} rows", log.rows().len()),
    );

    // 3. Each row's stages fit inside its own frame time.
    for r in log.rows() {
        let sum = r.stages.iter().map(|&(_, n)| n).sum::<u64>() as f64 / 1e6;
        assert!(
            sum <= r.ms,
            "frame {}: stages add up to {sum:.3} ms, more than the frame's {:.3} ms",
            r.frame,
            r.ms
        );
    }
    assert!(log.rows().iter().all(|r| r.stages.iter().any(|&(_, n)| n > 0)), "a row had no stage");
    check(
        &mut s,
        "In every row, the stages add up to no more than the frame's own time, while the next \
         frame's drawings are being read on another thread at the same moment.",
        "48 of 48".into(),
    );

    // 4. The same pictures with the log on and off.
    assert_eq!(on, off, "the log changed a picture");
    check(
        &mut s,
        "Every frame drawn with the log on is byte-identical to the same frame with it off.",
        "48 of 48 identical".into(),
    );

    // 5. Draft and Full are told apart, and a stepped frame is not playing.
    let cache = Arc::new(Mutex::new(CelCache::viewer()));
    draw(&project, &root, 12, 0, None, PreviewQuality::Full, &cache, &mut log);
    let last = log.rows().back().unwrap();
    assert_eq!((last.quality, last.playing, last.read_ahead), (PreviewQuality::Full, false, false));
    check(
        &mut s,
        "A frame stepped to at Full quality is logged as Full, not playing, not read ahead.",
        format!("{}, playing {}, read ahead {}", last.quality.label(), last.playing, last.read_ahead),
    );

    // 6. Switched off: the rows stay, nothing new is added, the stopwatch stops.
    log.switch(false);
    let kept = log.rows().len();
    play(&project, &root, 4, PreviewQuality::Draft, &mut log);
    assert_eq!(log.rows().len(), kept, "rows added after switching off");
    assert!(!perf::is_enabled(), "the stopwatch kept running after switching off");
    check(
        &mut s,
        "Switched off again, the 49 rows stay to be read or saved, 4 more frames add none, and \
         the stopwatch stops.",
        format!("{kept} rows; stopwatch off"),
    );

    // 7. The saved copy.
    let heading = vec![
        "Saved: 2026-09-25 12:00".to_string(),
        "Operating system: windows".to_string(),
        "Processor cores: 24".to_string(),
        "Build: 0.1.0, release".to_string(),
    ];
    let copy = log.to_markdown(&heading);
    for line in &heading {
        assert!(copy.contains(&format!("- {line}\n")), "the saved copy lost '{line}'");
    }
    let table: Vec<&str> = copy.lines().filter(|l| l.starts_with('|')).collect();
    assert_eq!(table.len(), kept + 2, "a header, a rule and one line per row");
    check(
        &mut s,
        "A saved copy is headed with the date, operating system, core count, version and build, \
         and has one table line per row after its header and rule.",
        format!("{} lines for {kept} rows", table.len() - 2),
    );

    // 8. Clear empties it.
    log.clear();
    assert_eq!((log.rows().len(), log.summary().frames), (0, 0));
    check(&mut s, "Clear empties the log.", "0 rows".into());

    // 9. The cap.
    log.switch(true);
    for f in 0..(KEEPS + 3) as i32 {
        log.push(row(f));
    }
    log.switch(false);
    let summary = log.summary();
    assert_eq!(log.rows().len(), KEEPS);
    assert_eq!(summary.let_go, 3);
    assert_eq!(log.rows().front().unwrap().frame, 3, "the oldest three go first");
    check(
        &mut s,
        "5,003 rows pushed keep the newest 5,000 and count the 3 let go.",
        format!("{} kept, {} let go, oldest kept is row {}", log.rows().len(), summary.let_go, 3),
    );

    // 10. The summary's arithmetic, on rows whose times are 3 to 5002 ms.
    assert_eq!(summary.median_ms, 2502.0);
    assert_eq!(summary.slowest_5_percent_ms, 4752.0);
    assert_eq!(log.slowest(20).len(), 20);
    assert_eq!(log.slowest(20)[0].frame, 5002);
    check(
        &mut s,
        "On those rows, taking 3 ms to 5,002 ms, the median is 2,502 ms, the slowest 5% take \
         4,752 ms or more, and the slowest listed first is the 5,002 ms one.",
        format!(
            "{} / {} / {}",
            summary.median_ms,
            summary.slowest_5_percent_ms,
            log.slowest(20)[0].ms
        ),
    );

    fs::write(repo("verification/P-19_session_log_table.md"), s).expect("write the artifact");
}

fn pct(sorted: &[f64], p: f64) -> f64 {
    sorted[(p * (sorted.len() - 1) as f64).round() as usize]
}

#[test]
#[ignore = "P-19: a measurement, run deliberately with --release --ignored"]
fn p19_session_log_cost() {
    let (project, root) = reference();
    let frames = 240;
    let mut s = String::from(
        "# P-19: what having the session log on costs\n\n\
         Written by `tests/p19_session_log.rs` under `cargo test --release --test p19_session_log \
         -- --ignored --nocapture`. The reference shot, 240 frames a pass from its first, each \
         drawn the way the window draws one (the viewer's cache, emptied before every pass; the \
         next frame's drawings read ahead). Passes alternate off and on, three rounds each. **ms** \
         is one frame from asking to encoded pixels, P-15's measure and the one a row records; \
         the read-ahead it starts is not waited for, as in the window.\n\n\
         | Quality | Log | Round | ms p50 | ms p95 |\n|---|---|---|---|---|\n",
    );
    let mut example = None;
    for quality in [PreviewQuality::Draft, PreviewQuality::Full] {
        for round in 1..=3 {
            for on in [false, true] {
                let mut log = SessionLog::default();
                log.switch(on);
                let mut times = Vec::with_capacity(frames);
                let comp = project.composition(&Id::new(COMP)).unwrap();
                let (first, last) = comp.work_frames();
                let mut clock = Playback::new(first, last, comp.frame_rate);
                let cache = Arc::new(Mutex::new(CelCache::viewer()));
                for i in 0..frames {
                    let shown = clock.at(Duration::from_secs_f64(i as f64 / 24.0));
                    let next = clock.after(shown.frame);
                    let (_, ms) =
                        draw(&project, &root, shown.frame, 0, Some(next), quality, &cache, &mut log);
                    times.push(ms);
                }
                log.switch(false);
                times.sort_by(f64::total_cmp);
                let _ = writeln!(
                    s,
                    "| {} | {} | {round} | {:.2} | {:.2} |",
                    quality.label(),
                    if on { "on" } else { "off" },
                    pct(&times, 0.5),
                    pct(&times, 0.95)
                );
                if on && round == 1 && quality == PreviewQuality::Draft {
                    example = Some(log.to_markdown(&[
                        "An example saved copy, written by `tests/p19_session_log.rs`: the first \
                         Draft pass with the log on"
                            .to_string(),
                        format!("Operating system: {}", std::env::consts::OS),
                        format!(
                            "Processor cores: {}",
                            std::thread::available_parallelism().map_or(0, |n| n.get())
                        ),
                        format!(
                            "Build: {}, {}",
                            env!("CARGO_PKG_VERSION"),
                            if cfg!(debug_assertions) { "debug" } else { "release" }
                        ),
                    ]));
                }
            }
        }
    }
    fs::write(repo("verification/P-19_session_log_cost_table.md"), s).expect("write the table");
    fs::write(repo("verification/P-19_session_log_example.md"), example.unwrap())
        .expect("write the example");
}
