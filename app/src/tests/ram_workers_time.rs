//! B-780 (D-900): the first playback of a shot, measured as the page plays it. Run deliberately,
//! in release, on the recorded machine (`--ignored`). Rows go to standard output and, when
//! `B780_OUT` names a file, are appended to it as JSON lines, labelled `B780_LABEL`.
//!
//! Shots: `B780_L25` (the lag harness's 25-layer reference shot, drawn from
//! `Fixtures/reference_shot`), and document 08's declared ten-layer fixture, blurred and graded.
//! For each: how long after Play the render-ahead holds 24, 120 and 239 frames, and how many
//! frames a first play of the 240-frame loop drops, asking `/at` as the page does, at most once a
//! screen refresh and never while a frame is in flight. `rest` plays after the hand has rested on
//! frame 0 for two seconds (`B780_REST`, milliseconds). `B780_WORKERS` sets the workers on the processor.
use super::*;

/// The frame requests that start the render-ahead, as the window's request handler starts it.
fn starts_ahead(ask: Ask) -> bool {
    ahead_wanted(ask)
}

fn l25() -> Option<Mutex<Viewer>> {
    let path = PathBuf::from(std::env::var_os("B780_L25")?);
    let mut viewer =
        open(&path).unwrap_or_else(|d| panic!("open {}: {}", path.display(), d.message));
    viewer.root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("Fixtures/reference_shot");
    Some(Mutex::new(viewer))
}

fn declared() -> Option<Mutex<Viewer>> {
    let (_project, root, text) = common::build_fixture("b780");
    let path = root.join("b780_project.json");
    std::fs::write(&path, &text).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
    Some(Mutex::new(open(&path).unwrap_or_else(|d| {
        panic!("open {}: {}", path.display(), d.message)
    })))
}

fn kept(viewer: &Mutex<Viewer>, card: Option<&Mutex<Card>>) -> usize {
    ready_frames(viewer, card)["frames"]
        .as_array()
        .map_or(0, Vec::len)
}

/// One run: optionally rest on frame 0, press Play, play one loop as the page does.
fn run(
    make: &dyn Fn() -> Option<Mutex<Viewer>>,
    card: Option<&Mutex<Card>>,
    q: PreviewQuality,
    rest_ms: u64,
) -> Option<serde_json::Value> {
    let viewer = make()?;
    let export = Mutex::new(Export::default());
    let running = AtomicBool::new(false);
    let out = std::thread::scope(|s| {
        let ask = |a: Ask| {
            let reply = serve_logged(&viewer, &export, None, card, a, Some(q));
            if starts_ahead(a) && !running.swap(true, Ordering::SeqCst) {
                let (viewer, running) = (&viewer, &running);
                s.spawn(move || {
                    render_ahead(viewer, card);
                    running.store(false, Ordering::SeqCst);
                });
            }
            reply
        };
        if rest_ms > 0 {
            ask(Ask::Frame(0));
            std::thread::sleep(Duration::from_millis(rest_ms));
        }
        let rested = kept(&viewer, card);
        let began = Instant::now();
        ask(Ask::Play(0));
        let mut reached: [Option<f64>; 3] = [None; 3];
        let mut asked = 0;
        let mut cached = 0;
        let tick = Duration::from_micros(16_667);
        let mut next = began + tick;
        loop {
            let now = Instant::now();
            if next > now {
                std::thread::sleep(next - now);
            }
            let ms = began.elapsed().as_millis() as u64;
            if ms >= 10_000 {
                break;
            }
            let reply = ask(Ask::At(ms));
            asked += 1;
            if reply.headers().get("x-cached").is_some_and(|v| v == "1") {
                cached += 1;
            }
            let n = kept(&viewer, card);
            for (i, want) in [24, 120, 239].iter().enumerate() {
                if reached[i].is_none() && n >= *want {
                    reached[i] = Some(began.elapsed().as_secs_f64());
                }
            }
            // The page asks at the next refresh after the answer came.
            next = Instant::now().max(next) + tick;
            while next < Instant::now() {
                next += tick;
            }
        }
        let (shown, skipped) = {
            let v = viewer.lock().unwrap();
            (v.playback.frames_shown(), v.playback.skipped())
        };
        // Stop playing and see how far it got; nothing is made at rest after that.
        ask(Ask::Frame(0));
        viewer.lock().unwrap().ram.lock().unwrap().idle = None;
        let kept_after = kept(&viewer, card);
        serde_json::json!({
            "rest_ms": rest_ms, "rested_kept": rested,
            "s_to_24": reached[0], "s_to_120": reached[1], "s_to_239": reached[2],
            "frames_shown": shown, "frames_dropped": skipped, "asked": asked, "from_memory": cached,
            "kept_at_end": kept_after,
        })
    });
    Some(out)
}

#[test]
#[ignore = "a measurement; run it deliberately, in release, on the recorded machine"]
fn b780_first_play_timing() {
    let label = std::env::var("B780_LABEL").unwrap_or_else(|_| "unlabelled".into());
    let rest: u64 = std::env::var("B780_REST")
        .ok()
        .and_then(|r| r.parse().ok())
        .unwrap_or(2000);
    if let Some(k) = std::env::var("B780_WORKERS")
        .ok()
        .and_then(|k| k.parse().ok())
    {
        AHEAD_WORKERS.store(k, Ordering::SeqCst);
    }
    // D-900's proposal: `B780_AT_REST=1` makes the loop at rest as well, as the program does not.
    if std::env::var("B780_AT_REST").as_deref() == Ok("1") {
        AHEAD_AT_REST.store(true, Ordering::SeqCst);
    }
    let card = Mutex::new(Card::default());
    let said_card = card.lock().unwrap().set(DrawOn::Gpu);
    let on_card = card.lock().unwrap().on;
    let shots: [(&str, &dyn Fn() -> Option<Mutex<Viewer>>); 2] =
        [("L25", &l25), ("declared ten-layer", &declared)];
    let only = std::env::var("B780_ONLY").unwrap_or_default();
    let mut out = std::env::var_os("B780_OUT").map(|p| {
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(p)
            .unwrap()
    });
    for (processor, card) in [("CPU", None), ("card", Some(&card))] {
        if card.is_some() && !on_card {
            println!("card NOT RUN: {said_card}");
            continue;
        }
        for (shot, make) in shots {
            for q in [PreviewQuality::Draft, PreviewQuality::Full] {
                for rest_ms in [0, rest] {
                    let key = format!("{processor}/{shot}/{}/{rest_ms}", q.label());
                    if !only.is_empty() && !key.contains(&only) {
                        continue;
                    }
                    let Some(mut row) = run(make, card, q, rest_ms) else {
                        println!("{key}: NOT RUN (no B780_L25)");
                        continue;
                    };
                    row["label"] = label.clone().into();
                    row["key"] = key.clone().into();
                    println!("{row}");
                    if let Some(f) = out.as_mut() {
                        use std::io::Write as _;
                        writeln!(f, "{row}").unwrap();
                    }
                }
            }
        }
    }
}
