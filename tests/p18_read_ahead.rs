//! P-18: a first playthrough in real time, with and without reading the next frame's drawings
//! ahead.
//!
//! P-03(b)'s harness samples twenty frames scattered across the shot, which is right for what it
//! measures and cannot show this at all: reading ahead helps only when the next request is for the
//! next frame. So this plays the shot the way the window does. A 24 fps clock (D-32) says which
//! frame is due, the frame is rendered and encoded, and then the page's own work - receiving the
//! bytes, drawing them and waiting for the screen - is stood in for by a fixed gap before the next
//! request. Nobody has measured that gap, so it is a stated assumption and there are two of it:
//! 0 ms, the worst case for reading ahead, and 8 ms, the average wait for a 60 Hz screen.
//!
//! Each pass starts from an empty viewer cache, so every drawing is met for the first time: the
//! first playthrough after opening a project. Two rounds, alternating, because the first pass of
//! a process also reads the files from disk for the first time.
//!
//! Asserted, because it is true on any machine: every frame shown with reading ahead on is
//! byte-identical to the same frame rendered with no cache at all. Timings are reported and
//! never asserted (document 12).
//!
//! It writes `verification/P-18_read_ahead_table.md`.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::{self, Playback, PreviewQuality};
use anime_compositor::sha256;

mod common;
use common::{build_fixture, repo};

const COMP: &str = "comp-reference-shot";
const FRAME_BUDGET_MS: f64 = 1000.0 / 24.0;

struct Workload {
    name: &'static str,
    project: Project,
    root: PathBuf,
}

/// One pass through the work area, once, in real time.
struct Pass {
    shown: u32,
    skipped: u32,
    /// Request to pixels, per frame shown, in ms and sorted.
    ms: Vec<f64>,
    /// Frame number and SHA-256 of what was shown.
    frames: Vec<(i32, String)>,
}

fn play(w: &Workload, ahead: bool, gap: Duration) -> Pass {
    let comp = Id::new(COMP);
    let c = w.project.composition(&comp).expect("the fixture has its composition");
    let (first, last) = c.work_frames();
    let mut clock = Playback::new(first, last, c.frame_rate);
    let length = Duration::from_secs_f64((last - first + 1) as f64 / 24.0);
    let cache = Arc::new(Mutex::new(CelCache::viewer()));
    let mut pass = Pass { shown: 0, skipped: 0, ms: Vec::new(), frames: Vec::new() };
    let began = Instant::now();
    let mut pending = None;
    while began.elapsed() < length {
        let shown = clock.at(began.elapsed());
        let at = Instant::now();
        let buffer = preview::preview_frame_cached(
            &w.project,
            &comp,
            shown.frame,
            &w.root,
            PreviewQuality::Draft,
            DEFAULT_TILE_SIZE,
            &mut FrameLog::new(3),
            &mut cache.lock().unwrap(),
        )
        .unwrap_or_else(|d| panic!("{} frame {}: {}", w.name, shown.frame, d.message));
        // What `app/src/main.rs` does between the render and the encode.
        if ahead {
            let next = clock.after(shown.frame);
            let handle =
                preview::read_ahead(w.project.clone(), comp.clone(), next, w.root.clone(), cache.clone());
            pending = Some(handle);
        }
        let pixels = buffer.to_srgb8_straight();
        pass.ms.push(at.elapsed().as_secs_f64() * 1000.0);
        pass.frames.push((shown.frame, sha256::hex(&pixels)));
        std::thread::sleep(gap);
    }
    if let Some(h) = pending {
        h.join().unwrap();
    }
    pass.shown = clock.frames_shown();
    pass.skipped = clock.skipped();
    pass.ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
    pass
}

fn pct(sorted: &[f64], p: f64) -> f64 {
    sorted[(p * (sorted.len() - 1) as f64).round() as usize]
}

#[test]
#[ignore = "P-18: a measurement, run deliberately with --release --ignored"]
fn p18_read_ahead() {
    let reference = persist::load(&repo("verification/B-08a_project.json"))
        .unwrap_or_else(|d| panic!("the reference shot: {}", d.message));
    let (declared, declared_root, _) = build_fixture("p18");
    let workloads = [
        Workload {
            name: "the reference shot (4 layers)",
            project: reference.document.project().clone(),
            root: repo("Fixtures/reference_shot"),
        },
        Workload { name: "the declared ten-layer fixture (10 layers)", project: declared, root: declared_root },
    ];

    let mut s = String::from(
        "# P-18: a first playthrough in real time, reading ahead off and on\n\n\
         Written by `tests/p18_read_ahead.rs` under `cargo test --release --test p18_read_ahead -- \
         --ignored --nocapture`. Draft quality, the viewer's own cache (`CelCache::viewer`), \
         emptied before every pass, and one pass through the work area in real time at 24 fps. \
         **Gap** stands in for what the page does between receiving one frame and asking for the \
         next; it is an assumption, not a measurement. **Wait** is from asking for a frame to \
         having its pixels, so it includes any time spent waiting for the read-ahead to finish.\n\n",
    );
    for w in &workloads {
        let _ = writeln!(s, "## {}\n", w.name);
        s.push_str("| Gap | Reading ahead | Round | Frames shown | Frames dropped | Wait p50 ms | Wait p95 ms |\n|---|---|---|---|---|---|---|\n");
        let mut checked = None;
        for gap_ms in [0u64, 8] {
            let gap = Duration::from_millis(gap_ms);
            for round in 1..=2 {
                for ahead in [false, true] {
                    let pass = play(w, ahead, gap);
                    let _ = writeln!(
                        s,
                        "| {gap_ms} ms | {} | {round} | {} | {} | {:.1} | {:.1} |",
                        if ahead { "on" } else { "off" },
                        pass.shown,
                        pass.skipped,
                        pct(&pass.ms, 0.5),
                        pct(&pass.ms, 0.95),
                    );
                    if ahead && checked.is_none() {
                        // Document 27: a cache may change the clock and nothing else.
                        for (frame, sha) in &pass.frames {
                            let alone = preview::preview_frame(
                                &w.project,
                                &Id::new(COMP),
                                *frame,
                                &w.root,
                                PreviewQuality::Draft,
                                DEFAULT_TILE_SIZE,
                                &mut FrameLog::new(3),
                            )
                            .unwrap();
                            assert_eq!(
                                &sha256::hex(&alone.to_srgb8_straight()),
                                sha,
                                "{} frame {frame}: reading ahead changed the picture",
                                w.name
                            );
                        }
                        checked = Some(pass.frames.len());
                    }
                }
            }
        }
        let _ = writeln!(
            s,
            "\nEvery one of the {} frames shown by the first pass with reading ahead on is \
             byte-identical to the same frame rendered with no cache. A frame is due every \
             {FRAME_BUDGET_MS:.1} ms; a dropped frame is one the clock passed over because the one \
             before it was not ready in time (D-32).\n",
            checked.expect("a pass with reading ahead on ran")
        );
    }
    fs::write(repo("verification/P-18_read_ahead_table.md"), s).expect("write the artifact");
}
