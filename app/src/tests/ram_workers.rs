//! B-780 (D-900): the RAM preview made by several workers at once, and at rest once the hand is
//! still. Writes `verification/B-780_ram_workers_table.md`.
use super::*;
use PreviewQuality::{Draft, Full};

/// D-900: the workers the rows below run; the program runs `AHEAD_WORKERS`.
const SEVERAL: usize = 4;

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
        "frames made ahead at rest in the program (D-900, PROPOSED: off, it slowed the next drag at Full on the processor)",
        "off",
        if AHEAD_AT_REST.load(Ordering::SeqCst) { "on" } else { "off" }.into(),
    );
    // The rows below are the proposal's: made at rest as well.
    AHEAD_AT_REST.store(true, Ordering::SeqCst);

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

    AHEAD_AT_REST.store(false, Ordering::SeqCst);
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
