//! B-780 (D-900): the RAM preview made by several workers at once, and at rest once the hand is
//! still. Writes `verification/B-780_ram_workers_table.md`.
use super::*;
use PreviewQuality::{Draft, Full};

/// D-900: the workers the rows below run; the program runs `AHEAD_WORKERS`.
const SEVERAL: usize = 4;

/// B-787: the at-rest switches are the program's; the tests that turn them run one at a time.
static SWITCHES: Mutex<()> = Mutex::new(());

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .join(rel)
}

fn cached(r: &Response<Vec<u8>>) -> bool {
    r.headers().get("x-cached").is_some_and(|v| v == "1")
}

fn viewer_on(root: &Path) -> Mutex<Viewer> {
    let mut viewer = demo();
    viewer.root = root.to_path_buf();
    Mutex::new(viewer)
}

fn held(viewer: &Mutex<Viewer>) -> usize {
    ready_frames(viewer, None)["frames"]
        .as_array()
        .map_or(0, Vec::len)
}

fn hide_layer_1() -> Command {
    Command::SetLayerEnabled {
        composition: Id::new("comp-reference-shot"),
        layer_id: Id::new("layer-1"),
        value: false,
    }
}

#[test]
fn b780_frames_made_by_several_workers_are_the_frames_made_by_one() {
    let _switches = SWITCHES.lock().unwrap_or_else(|e| e.into_inner());
    let export = Mutex::new(Export::default());
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut row = |what: &str, expected: &str, actual: String| {
        rows.push((what.to_string(), expected.to_string(), actual));
    };
    let ask = |v: &Mutex<Viewer>, a: Ask, q: PreviewQuality| {
        serve_logged(v, &export, None, None, a, Some(q))
    };
    // Every frame of `frames` sent from memory by `ahead`, against `cold` asked for one at a time
    // with nothing made ahead: "N of N" when all are both.
    let all_same =
        |ahead: &Mutex<Viewer>, cold: &Mutex<Viewer>, frames: &[i32], q: PreviewQuality| {
            let good = frames
                .iter()
                .filter(|&&n| {
                    let kept = ask(ahead, Ask::Frame(n), q);
                    cached(&kept) && kept.body() == ask(cold, Ask::Frame(n), q).body()
                })
                .count();
            format!("{good} of {}", frames.len())
        };
    // Frames sent from memory that are not the cold frame: what a stale frame would be.
    let stale = |ahead: &Mutex<Viewer>, cold: &Mutex<Viewer>, frames: &[i32], q: PreviewQuality| {
        frames
            .iter()
            .filter(|&&n| {
                let kept = ask(ahead, Ask::Frame(n), q);
                cached(&kept) && kept.body() != ask(cold, Ask::Frame(n), q).body()
            })
            .count()
            .to_string()
    };
    let shot = repo("Fixtures/reference_shot");
    let every: Vec<i32> = (0..240).collect();
    row(
        "workers making frames ahead on the processor in the program (D-900: several measured no faster)",
        "1",
        AHEAD_WORKERS.load(Ordering::SeqCst).to_string(),
    );
    row(
        "frames made ahead at rest on the processor in the program (D-900, D-908: off, it slowed the next drag at Full on the processor)",
        "off",
        if AHEAD_AT_REST_CPU.load(Ordering::SeqCst) { "on" } else { "off" }.into(),
    );
    // The rows below are D-900's proposal: made at rest on the processor as well.
    AHEAD_AT_REST_CPU.store(true, Ordering::SeqCst);

    // Byte for byte the frames one worker makes, which are the frames asked for by hand.
    for q in [Draft, Full] {
        let ahead = viewer_on(&shot);
        ask(&ahead, Ask::Play(0), q);
        row(
            &format!(
                "reference shot, {}: after play at frame 0 the other 239 frames are made ahead",
                q.label()
            ),
            "239 frames",
            format!("{} frames", render_ahead_by(&ahead, None, SEVERAL)),
        );
        row(
            &format!("reference shot, {}: every frame is sent from memory and is byte for byte the frame made by hand", q.label()),
            "240 of 240",
            all_same(&ahead, &viewer_on(&shot), &every, q),
        );
    }
    let (_project, declared_root, text) = common::build_fixture("b780-check");
    let declared_path = declared_root.join("b780_project.json");
    std::fs::write(&declared_path, &text).expect("write the declared fixture");
    let declared = || Mutex::new(open(&declared_path).expect("open the declared fixture"));
    let ahead = declared();
    ask(&ahead, Ask::Play(0), Draft);
    row(
        "document 08's ten layers, blurred and graded, Draft: the other 239 frames are made ahead",
        "239 frames",
        format!("{} frames", render_ahead_by(&ahead, None, SEVERAL)),
    );
    let sampled: Vec<i32> = (0..240).step_by(4).collect();
    row(
        "and every fourth frame is byte for byte the frame made by hand",
        "60 of 60",
        all_same(&ahead, &declared(), &sampled, Draft),
    );
    row(
        "the workers' own cels are given back when they finish",
        "0 bytes",
        format!("{} bytes", AHEAD_CELS.load(Ordering::SeqCst)),
    );

    // An edit, then an undo, while the loop is being made ahead.
    let ahead = viewer_on(&shot);
    ask(&ahead, Ask::Play(0), Draft);
    std::thread::scope(|s| {
        let worker = s.spawn(|| render_ahead_by(&ahead, None, SEVERAL));
        while held(&ahead) < 30 && !worker.is_finished() {
            std::thread::sleep(Duration::from_millis(2));
        }
        edit(&ahead, hide_layer_1());
        worker.join().unwrap();
    });
    let cold_edited = viewer_on(&shot);
    edit(&cold_edited, hide_layer_1());
    row(
        "layer 1 hidden while the loop is being made ahead: every frame is then the edited frame made by hand, from memory",
        "240 of 240",
        all_same(&ahead, &cold_edited, &every, Draft),
    );
    undo(&ahead);
    row(
        "after undo, frames sent from memory that are not the original frame made by hand",
        "0",
        stale(&ahead, &viewer_on(&shot), &every, Draft),
    );

    // Another composition while the loop is being made ahead.
    let ahead = viewer_on(&shot);
    let cold_switched = viewer_on(&shot);
    let original = ahead.lock().unwrap().composition.clone();
    for v in [&ahead, &cold_switched] {
        edit_command(v, "composition.create", Some("name=Other&frames=48"));
    }
    let other = ahead.lock().unwrap().composition.clone();
    row(
        "a second composition, 48 frames long, is made and opened",
        "another",
        if other == original {
            "the same".into()
        } else {
            "another".into()
        },
    );
    show(&ahead, &original);
    ask(&ahead, Ask::Play(0), Draft);
    std::thread::scope(|s| {
        let worker = s.spawn(|| render_ahead_by(&ahead, None, SEVERAL));
        while held(&ahead) < 30 && !worker.is_finished() {
            std::thread::sleep(Duration::from_millis(2));
        }
        show(&ahead, &other);
        worker.join().unwrap();
    });
    let short: Vec<i32> = (0..48).collect();
    row(
        "another composition opened while the loop is being made ahead: its frames from memory that are not its own",
        "0",
        stale(&ahead, &cold_switched, &short, Draft),
    );
    show(&ahead, &original);
    show(&cold_switched, &original);
    row(
        "and back on the first, its frames from memory that are not its own",
        "0",
        stale(&ahead, &cold_switched, &every, Draft),
    );

    // At rest: made once the hand has been still for half a second.
    // With the program's one worker and with several.
    let mut asked = Vec::new();
    for k in [1, SEVERAL] {
        let rest = viewer_on(&shot);
        std::thread::scope(|s| {
            ask(&rest, Ask::Frame(100), Draft);
            let worker = s.spawn(|| render_ahead_by(&rest, None, k));
            std::thread::sleep(Duration::from_millis(250));
            row(
                &format!(
                    "{k} worker(s): at rest on frame 100, a quarter of a second later: frames held"
                ),
                "1",
                held(&rest).to_string(),
            );
            // The hand keeps stepping: nothing is made between its frames.
            for n in 101..=110 {
                ask(&rest, Ask::Frame(n), Draft);
                std::thread::sleep(Duration::from_millis(100));
            }
            row(
            &format!("{k} worker(s): stepping frames 101 to 110 a tenth of a second apart: frames held, only those asked for"),
            "11",
            held(&rest).to_string(),
        );
            row(
                &format!("{k} worker(s): once the hand is still, the rest of the loop is made"),
                "229 frames",
                format!("{} frames", worker.join().unwrap()),
            );
        });
        row(
        &format!("{k} worker(s): and every frame is sent from memory, byte for byte the frame made by hand"),
        "240 of 240",
        all_same(&rest, &viewer_on(&shot), &every, Draft),
    );

        // The hand stops frames being made ahead: on the declared ten layers at Full, each slow.
        let busy = declared();
        let (asked_ms, before, after, _made) = std::thread::scope(|s| {
            ask(&busy, Ask::Play(0), Full);
            let worker = s.spawn(|| render_ahead_by(&busy, None, k));
            while held(&busy) < 3 {
                std::thread::sleep(Duration::from_millis(2));
            }
            let t = Instant::now();
            ask(&busy, Ask::Frame(200), Full);
            let asked_ms = t.elapsed().as_secs_f64() * 1000.0;
            let before = held(&busy);
            std::thread::sleep(Duration::from_millis(300));
            let after = held(&busy);
            (asked_ms, before, after, worker.join().unwrap())
        });
        row(
        &format!("{k} worker(s): playing the ten layers at Full, a frame asked for by hand: frames made ahead in the 0.3 s after it"),
        "0",
        (after - before).to_string(),
    );
        row(
            &format!("{k} worker(s): and once the hand is still the loop is finished"),
            "240 held",
            format!("{} held", held(&busy)),
        );
        asked.push(asked_ms);
    }
    // The picture below the last edit (B-159) is still there after the loop is made at rest:
    // layer 3 of frame 100 at Full set to 80% opacity, then 60%, with the loop made between or not.
    let keeps_below = |fill: bool| {
        let kept = viewer_on(&shot);
        let reused = || kept.lock().unwrap().cache.lock().unwrap().below_reused();
        let opacity = |value: &str| {
            let query = format!("layer=layer-3&prop=opacity&value={value}&frame=100");
            edit_command(&kept, "property.set_base", Some(&query));
            ask(&kept, Ask::Frame(100), Full);
        };
        ask(&kept, Ask::Frame(100), Full);
        opacity("0.8");
        let before = reused();
        let made = if fill { render_ahead_by(&kept, None, 1) } else { 0 };
        opacity("0.6");
        let how = if reused() > before { "started from them" } else { "drawn from nothing" };
        format!("{made} made, {how}")
    };
    row(
        "frame 100 at Full, layer 3 set to 80% then 60% opacity, nothing made between: the 60% frame starts from the layers below as kept",
        "0 made, started from them",
        keeps_below(false),
    );
    row(
        "the same with the loop made at rest between the two: it still starts from them",
        "239 made, started from them",
        keeps_below(true),
    );

    let t = Instant::now();
    ask(&declared(), Ask::Frame(200), Full);
    let alone_ms = t.elapsed().as_secs_f64() * 1000.0;

    // Memory: room for 50 Draft frames.
    let full = viewer_on(&shot);
    // Draft draws a quarter of the width and of the height: 480 by 270.
    let one = 480 * 270 * 4;
    full.lock().unwrap().ram.lock().unwrap().frames = FrameCache::with_budget(50 * one);
    ask(&full, Ask::Play(0), Draft);
    render_ahead_by(&full, None, SEVERAL);
    let bytes = full.lock().unwrap().ram.lock().unwrap().frames.held_bytes();
    row(
        "with room for 50 Draft frames, the workers stop when it is full",
        "50 frames, 25920000 bytes",
        format!("{} frames, {bytes} bytes", held(&full)),
    );
    row(
        "the workers' own cels while they run: a quarter of the memory setting, taken from the viewer's",
        "the setting",
        if frame_share() + ram_ceiling() / 4 <= ram_ceiling() {
            "the setting".into()
        } else {
            "more than the setting".into()
        },
    );

    AHEAD_AT_REST_CPU.store(false, Ordering::SeqCst);
    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut text = format!(
        "# B-780: RAM preview made by several workers, and at rest\n\nWritten by `cargo test \
         --release -p anime_compositor_app b780_frames`. Frames go through `serve_logged`, the \
         function every frame on screen comes from; \"by hand\" is a viewer that has made \
         nothing ahead, asked for one frame at a time. Rows not marked with a worker count run \
         `{SEVERAL}` workers on the processor, the setting D-900 leaves off (the program runs \
         one); the card keeps B-154's one. Rows after the second run with frames made at \
         rest switched on, D-900's proposal; the program leaves it off.\n\nA frame asked for by hand while the ten layers \
         were made ahead at Full took {:.0} ms with one worker and {:.0} ms with {SEVERAL}, \
         and {alone_ms:.0} ms on a viewer with nothing made ahead (one run each, not a timing \
         claim).\n\n**{passed} of {} checks pass.**\n\n| Check | Expected | Actual | Result \
         |\n|---|---|---|---|\n",
        asked[0],
        asked[1],
        rows.len()
    );
    for (what, expected, actual) in &rows {
        text.push_str(&format!(
            "| {what} | {expected} | {actual} | {} |\n",
            if expected == actual { "PASS" } else { "FAIL" }
        ));
    }
    std::fs::write(repo("verification/B-780_ram_workers_table.md"), text).expect("write the table");
    assert_eq!(
        passed,
        rows.len(),
        "see verification/B-780_ram_workers_table.md"
    );
}

/// B-787 (D-908): the loop made at rest only while the card draws the viewer, with the program's
/// switches. Writes `verification/B-787_card_at_rest_table.md`.
#[test]
fn b787_frames_made_at_rest_on_the_card_only() {
    let _switches = SWITCHES.lock().unwrap_or_else(|e| e.into_inner());
    let export = Mutex::new(Export::default());
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut row = |what: &str, expected: &str, actual: String| {
        rows.push((what.to_string(), expected.to_string(), actual));
    };
    let ask = |v: &Mutex<Viewer>, c: Option<&Mutex<Card>>, a: Ask, q: PreviewQuality| {
        serve_logged(v, &export, None, c, a, Some(q))
    };
    let held_on = |v: &Mutex<Viewer>, c: Option<&Mutex<Card>>| {
        ready_frames(v, c)["frames"].as_array().map_or(0, Vec::len)
    };
    // Frames of `frames` sent from memory that are not the frame `cold` makes by hand.
    let stale = |ahead: &Mutex<Viewer>,
                 cold: &Mutex<Viewer>,
                 c: Option<&Mutex<Card>>,
                 frames: &[i32],
                 q: PreviewQuality| {
        frames
            .iter()
            .filter(|&&n| {
                let kept = ask(ahead, c, Ask::Frame(n), q);
                cached(&kept) && kept.body() != ask(cold, c, Ask::Frame(n), q).body()
            })
            .count()
    };
    // Frames of `frames` sent from memory and byte for byte the frame `cold` makes by hand.
    let same = |ahead: &Mutex<Viewer>,
                cold: &Mutex<Viewer>,
                c: Option<&Mutex<Card>>,
                frames: &[i32],
                q: PreviewQuality| {
        let good = frames
            .iter()
            .filter(|&&n| {
                let kept = ask(ahead, c, Ask::Frame(n), q);
                cached(&kept) && kept.body() == ask(cold, c, Ask::Frame(n), q).body()
            })
            .count();
        format!("{good} of {}", frames.len())
    };
    let shot = repo("Fixtures/reference_shot");
    let every: Vec<i32> = (0..240).collect();
    row(
        "frames made ahead at rest while the card draws the viewer, in the program (D-908 PROPOSED)",
        "off",
        if AHEAD_AT_REST.load(Ordering::SeqCst) { "on" } else { "off" }.into(),
    );
    row(
        "and while the processor draws it (D-900: the next drag at Full was slower)",
        "off",
        if AHEAD_AT_REST_CPU.load(Ordering::SeqCst) { "on" } else { "off" }.into(),
    );
    // The rows below are D-908's proposal: the card's switch on, the processor's left off.
    AHEAD_AT_REST.store(true, Ordering::SeqCst);

    // The processor: nothing is made at rest, with no card and with the card switched off.
    let off = Mutex::new(Card::default());
    for (what, c) in [("no card", None), ("the card switched off", Some(&off))] {
        let rest = viewer_on(&shot);
        ask(&rest, c, Ask::Frame(100), Draft);
        std::thread::sleep(AHEAD_IDLE + Duration::from_millis(100));
        row(
            &format!("processor ({what}), at rest on frame 100 past the half second: frames made ahead"),
            "0 frames",
            format!("{} frames", render_ahead(&rest, c)),
        );
    }

    row(
        "a frame asked by hand on the processor starts the frames made ahead (as before B-780, it does not)",
        "no",
        if ahead_wanted(Ask::Frame(100), None) || ahead_wanted(Ask::Frame(100), Some(&off)) { "yes" } else { "no" }.into(),
    );
    let card = Mutex::new(Card::default());
    let said_card = card.lock().unwrap().set(DrawOn::Gpu);
    let on_card = card.lock().unwrap().on;
    if on_card {
        let c = Some(&card);
        row(
            "a frame asked by hand on the card starts them, to be made once the hand is still",
            "yes",
            if ahead_wanted(Ask::Frame(100), c) { "yes" } else { "no" }.into(),
        );
        // At rest on frame 100: nothing for the first half second, then the rest of the loop.
        for q in [Draft, Full] {
            let rest = viewer_on(&shot);
            let made = std::thread::scope(|s| {
                ask(&rest, c, Ask::Frame(100), q);
                let worker = s.spawn(|| render_ahead(&rest, c));
                std::thread::sleep(Duration::from_millis(250));
                row(
                    &format!("card, {}: at rest on frame 100, a quarter of a second later: frames held", q.label()),
                    "1",
                    held_on(&rest, c).to_string(),
                );
                worker.join().unwrap()
            });
            let sampled: Vec<i32> = if q == Draft { every.clone() } else { (0..240).step_by(8).collect() };
            row(
                &format!("card, {}: once the hand is still the rest of the loop is made", q.label()),
                "239 frames",
                format!("{made} frames"),
            );
            row(
                &format!("card, {}: frames sent from memory, byte for byte the card's frame made by hand", q.label()),
                &format!("{0} of {0}", sampled.len()),
                same(&rest, &viewer_on(&shot), c, &sampled, q),
            );
            if q == Draft {
                // A quality change: the Draft loop is never sent for Full.
                row(
                    "card: the Draft loop made, then each eighth frame asked at Full: Draft frames sent",
                    "0",
                    stale(&rest, &viewer_on(&shot), c, &(0..240).step_by(8).collect::<Vec<_>>(), Full).to_string(),
                );
                // The card switched off: the card's frames are never sent for the processor's.
                card.lock().unwrap().set(DrawOn::Cpu);
                row(
                    "card switched off after the loop was made: each eighth frame on the processor, card frames sent",
                    "0",
                    stale(&rest, &viewer_on(&shot), Some(&card), &(0..240).step_by(8).collect::<Vec<_>>(), Draft).to_string(),
                );
                card.lock().unwrap().set(DrawOn::Gpu);
            }
        }

        // The hand moves to another frame while the loop is made: work starts round the new one.
        let moved = viewer_on(&shot);
        std::thread::scope(|s| {
            ask(&moved, c, Ask::Frame(100), Draft);
            let worker = s.spawn(|| render_ahead(&moved, c));
            while held_on(&moved, c) < 30 && !worker.is_finished() {
                std::thread::sleep(Duration::from_millis(2));
            }
            ask(&moved, c, Ask::Frame(20), Draft);
            worker.join().unwrap();
        });
        row(
            "card: the hand moves from frame 100 to 20 while the loop is made: frames sent from memory that are not the frame made by hand",
            "0",
            stale(&moved, &viewer_on(&shot), c, &every, Draft).to_string(),
        );
        row(
            "and every frame is then held",
            "240 held",
            format!("{} held", held_on(&moved, c)),
        );

        // An edit while the loop is made at rest, then an undo.
        let edited = viewer_on(&shot);
        std::thread::scope(|s| {
            ask(&edited, c, Ask::Frame(100), Draft);
            let worker = s.spawn(|| render_ahead(&edited, c));
            while held_on(&edited, c) < 30 && !worker.is_finished() {
                std::thread::sleep(Duration::from_millis(2));
            }
            edit(&edited, hide_layer_1());
            worker.join().unwrap();
        });
        let cold_edited = viewer_on(&shot);
        edit(&cold_edited, hide_layer_1());
        row(
            "card: layer 1 hidden while the loop is made at rest: frames from memory, the edited frame made by hand",
            "240 of 240",
            same(&edited, &cold_edited, c, &every, Draft),
        );
        undo(&edited);
        row(
            "card: after undo, frames sent from memory that are not the original frame made by hand",
            "0",
            stale(&edited, &viewer_on(&shot), c, &every, Draft).to_string(),
        );

        // Another composition opened while the loop is made at rest, and back.
        let ahead = viewer_on(&shot);
        let cold_switched = viewer_on(&shot);
        let original = ahead.lock().unwrap().composition.clone();
        for v in [&ahead, &cold_switched] {
            edit_command(v, "composition.create", Some("name=Other&frames=48"));
        }
        let other = ahead.lock().unwrap().composition.clone();
        show(&ahead, &original);
        std::thread::scope(|s| {
            ask(&ahead, c, Ask::Frame(100), Draft);
            let worker = s.spawn(|| render_ahead(&ahead, c));
            while held_on(&ahead, c) < 30 && !worker.is_finished() {
                std::thread::sleep(Duration::from_millis(2));
            }
            show(&ahead, &other);
            worker.join().unwrap();
        });
        let short: Vec<i32> = (0..48).collect();
        row(
            "card: another composition opened while the loop is made at rest: its frames from memory that are not its own",
            "0",
            stale(&ahead, &cold_switched, c, &short, Draft).to_string(),
        );
        show(&ahead, &original);
        show(&cold_switched, &original);
        row(
            "and back on the first, its frames from memory that are not its own",
            "0",
            stale(&ahead, &cold_switched, c, &every, Draft).to_string(),
        );
    }

    AHEAD_AT_REST.store(false, Ordering::SeqCst);
    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut text = format!(
        "# B-787: RAM preview made at rest on the card only\n\nWritten by `cargo test --release \
         -p anime_compositor_app b787_frames`. The program is built with both at-rest switches off (D-908 PROPOSED); the rows below switch the card's on, as proposed: frames \
         are made ahead at rest, once the hand has been still for half a second, only while the \
         graphics card draws the viewer. Frames go through `serve_logged`, the function every \
         frame on screen comes from; \"by hand\" is a viewer that has made nothing ahead, asked \
         for one frame at a time. Card: {}.\n\n**{passed} of {} checks pass.**\n\n| Check | \
         Expected | Actual | Result |\n|---|---|---|---|\n",
        if on_card {
            card.lock().unwrap().about()
        } else {
            format!("none, card rows NOT RUN: {said_card}")
        },
        rows.len()
    );
    for (what, expected, actual) in &rows {
        text.push_str(&format!(
            "| {what} | {expected} | {actual} | {} |\n",
            if expected == actual { "PASS" } else { "FAIL" }
        ));
    }
    std::fs::write(repo("verification/B-787_card_at_rest_table.md"), text)
        .expect("write the table");
    assert!(on_card, "the card rows did not run: {said_card}");
    assert_eq!(passed, rows.len(), "see verification/B-787_card_at_rest_table.md");
}
