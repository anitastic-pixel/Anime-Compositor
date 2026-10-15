use super::*;

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .join(rel)
}

fn header(response: &Response<Vec<u8>>, name: &str) -> String {
    response.headers().get(name).map_or_else(String::new, |v| {
        v.to_str().unwrap_or("<not readable as text>").to_string()
    })
}

/// "identical", or how far apart two answers are.
fn same(a: &Response<Vec<u8>>, b: &Response<Vec<u8>>) -> String {
    if a.status() == 200 && a.body() == b.body() {
        return "identical".into();
    }
    let differ = a.body().iter().zip(b.body()).filter(|(x, y)| x != y).count();
    format!("{differ} bytes differ, {} against {} bytes", a.body().len(), b.body().len())
}

/// What the page is told about a frame besides its pixels.
fn said(r: &Response<Vec<u8>>) -> String {
    ["x-width", "x-height", "x-drawn-on", "x-frame", "x-quality"]
        .iter()
        .map(|h| header(r, h))
        .collect::<Vec<_>>()
        .join(" ")
}

fn copy_dir(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("make the copy's folder");
    for entry in std::fs::read_dir(from).expect("read the fixture folder") {
        let entry = entry.expect("a folder entry");
        let path = entry.path();
        if path.is_dir() {
            copy_dir(&path, &to.join(entry.file_name()));
        } else {
            std::fs::copy(&path, to.join(entry.file_name())).expect("copy a fixture file");
        }
    }
}

/// The reference shot with its drawings in a folder of its own, so one can be replaced.
fn copied_shot(name: &str) -> PathBuf {
    let to = std::env::temp_dir().join(format!("b154-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&to);
    copy_dir(&repo("Fixtures/reference_shot"), &to);
    to
}

fn viewer_on(root: &Path) -> Mutex<Viewer> {
    let mut viewer = demo();
    viewer.root = root.to_path_buf();
    Mutex::new(viewer)
}

fn hide_layer_1() -> Command {
    Command::SetLayerEnabled {
        composition: Id::new("comp-reference-shot"),
        layer_id: Id::new("layer-1"),
        value: false,
    }
}

/// A Light Wrap at Mix 50, which the card hands to the CPU whole (B-137b).
fn mixed_light_wrap_on_layer_2() -> Command {
    Command::AddEffect {
        composition: Id::new("comp-reference-shot"),
        layer_id: Id::new("layer-2"),
        effect: EffectInstance {
            mix: 50.0,
            ..EffectInstance::new(
                Id::new("b154-light-wrap"),
                Effect::LightWrap { width: 20.0, intensity: 100.0, blend: "screen".to_string() },
            )
        },
        index: None,
    }
}

#[test]
fn b154_a_frame_made_once_is_sent_from_memory_and_never_stale() {
    use PreviewQuality::{Draft, Full};
    let export = Mutex::new(Export::default());
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut row = |what: &str, expected: &str, actual: String| {
        rows.push((what.to_string(), expected.to_string(), actual));
    };
    let ask = |v: &Mutex<Viewer>, card: Option<&Mutex<Card>>, n: i32, q: PreviewQuality| {
        serve_logged(v, &export, None, card, Ask::Frame(n), Some(q))
    };

    // On the CPU.
    let root = copied_shot("cpu");
    let viewer = viewer_on(&root);
    let first = ask(&viewer, None, 100, Full);
    row("CPU: frame 100, asked for the first time, is made", "0", header(&first, "x-cached"));
    let again = ask(&viewer, None, 100, Full);
    row("asked for again, it is sent from memory", "1", header(&again, "x-cached"));
    row("byte for byte the frame made the first time", "identical", same(&first, &again));
    row("and the page is told the same about it", &said(&first), said(&again));
    row("frame 101 is not frame 100", "0", header(&ask(&viewer, None, 101, Full), "x-cached"));
    row("Draft is not Full", "0", header(&ask(&viewer, None, 100, Draft), "x-cached"));
    row("Full again is still remembered", "1", header(&ask(&viewer, None, 100, Full), "x-cached"));

    viewer.lock().unwrap().solo = vec![Id::new("layer-2")];
    let soloed = ask(&viewer, None, 100, Full);
    row("soloing a layer is not the frame remembered without it", "0", header(&soloed, "x-cached"));
    let cold = viewer_on(&root);
    cold.lock().unwrap().solo = vec![Id::new("layer-2")];
    row("and is the soloed frame made cold", "identical", same(&soloed, &ask(&cold, None, 100, Full)));
    viewer.lock().unwrap().solo.clear();
    row("unsoloed, the first frame is remembered", "1", header(&ask(&viewer, None, 100, Full), "x-cached"));

    edit(&viewer, hide_layer_1());
    let edited = ask(&viewer, None, 100, Full);
    row("after an edit (layer 1 hidden) the frame is made again", "0", header(&edited, "x-cached"));
    let cold = viewer_on(&root);
    edit(&cold, hide_layer_1());
    row("and is the edited frame made cold", "identical", same(&edited, &ask(&cold, None, 100, Full)));
    undo(&viewer);
    let undone = ask(&viewer, None, 100, Full);
    row("after undo the frame from before the edit is sent from memory", "1", header(&undone, "x-cached"));
    row("and is that frame", "identical", same(&first, &undone));

    // A drawing replaced on disk: every drawing of layer 1 becomes layer 2's first.
    let other = std::fs::read_dir(root.join("layer2"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "png"))
        .min()
        .expect("layer 2 has a drawing");
    let bytes = std::fs::read(other).unwrap();
    for entry in std::fs::read_dir(root.join("layer1")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|x| x == "png") {
            std::fs::write(&path, &bytes).unwrap();
        }
    }
    let replaced = ask(&viewer, None, 100, Full);
    row("after layer 1's drawings are replaced on disk the frame is made again", "0", header(&replaced, "x-cached"));
    row("and shows the new drawings: it is not the frame from before", "different", if replaced.body() == first.body() { "the same".into() } else { "different".into() });
    row("and is the frame made cold from the new drawings", "identical", same(&replaced, &ask(&viewer_on(&root), None, 100, Full)));

    // Made ahead while playing.
    let ahead = viewer_on(&repo("Fixtures/reference_shot"));
    row("with nothing playing nothing is made ahead", "0 frames", format!("{} frames", render_ahead(&ahead, None)));
    serve_logged(&ahead, &export, None, None, Ask::Play(0), Some(Draft));
    row("Draft, after play is pressed at frame 0: the other 239 frames of the loop are made ahead", "239 frames", format!("{} frames", render_ahead(&ahead, None)));
    let remembered = (0..240).filter(|&n| header(&ask(&ahead, None, n, Draft), "x-cached") == "1").count();
    row("and all 240 are then sent from memory", "240 of 240", format!("{remembered} of 240"));
    // The cels have what the remembered frames do not: fitted each time the cels are wanted.
    let (cels, ram) = {
        let v = ahead.lock().unwrap();
        (Arc::clone(&v.cache), Arc::clone(&v.ram))
    };
    let loop_holds = ram.lock().unwrap().frames.held_bytes();
    ask(&ahead, None, 100, Full);
    let gib = |bytes: usize| format!("{:.2} GiB", bytes as f64 / (1u64 << 30) as f64);
    let cels = cels.lock().unwrap();
    row(
        "the next frame not remembered gives the cels the memory setting less the loop",
        &gib(ram_ceiling() - loop_holds),
        gib(cels.budget() + cels.effect_budget()),
    );
    drop(cels);
    let cold = viewer_on(&repo("Fixtures/reference_shot"));
    row("a frame made ahead (frame 50) is byte for byte the one made when asked", "identical", same(&ask(&ahead, None, 50, Draft), &ask(&cold, None, 50, Draft)));

    // On the graphics card.
    let card = Mutex::new(Card::default());
    let said_card = card.lock().unwrap().set(DrawOn::Gpu);
    let on_card = card.lock().unwrap().on;
    if on_card {
        let gpu_first = ask(&viewer, Some(&card), 100, Full);
        row("card: frame 100 is not the CPU's remembered frame", "0", header(&gpu_first, "x-cached"));
        row("and is drawn on the card", "GPU", header(&gpu_first, "x-drawn-on"));
        let gpu_again = ask(&viewer, Some(&card), 100, Full);
        row("asked for again, it is sent from memory", "1", header(&gpu_again, "x-cached"));
        row("byte for byte the card's frame", "identical", same(&gpu_first, &gpu_again));
        row("and still says the card drew it", "GPU", header(&gpu_again, "x-drawn-on"));

        // A frame the card leaves to the CPU keeps saying so from memory, with its warning.
        let session = Mutex::new(SessionLog::default());
        session.lock().unwrap().switch(true);
        edit(&viewer, mixed_light_wrap_on_layer_2());
        let logged = |v: &Mutex<Viewer>| serve_logged(v, &export, Some(&session), Some(&card), Ask::Frame(100), Some(Full));
        let warned = |s: &Mutex<SessionLog>| s.lock().unwrap().rows().back().map_or(String::new(), |r| r.warnings.join(" / "));
        let left = logged(&viewer);
        let left_said = warned(&session);
        row("the first time, the session log says the CPU drew it", "says so", if left_said.contains("The CPU drew this frame") { "says so".into() } else { format!("says {left_said:?}") });
        let left_again = logged(&viewer);
        row("a frame the card leaves to the CPU (a Light Wrap at Mix 50), asked for again, is sent from memory", "1", header(&left_again, "x-cached"));
        row("and still says the CPU drew it, not the card", "CPU, not GPU", header(&left_again, "x-drawn-on"));
        row("and the session log gets the same warning again", &left_said, warned(&session));
        row("byte for byte the frame the CPU drew", "identical", same(&left, &left_again));

        // The card paints a remembered frame from the same bytes it drew it into.
        let mut held = card.lock().unwrap();
        let gpu = held.gpu.as_mut().unwrap().as_mut().unwrap();
        let project = shown(&demo());
        let mut cels = CelCache::viewer();
        let mut log = FrameLog::new(3);
        let (w, h) = preview::preview_frame_held(&project, &Id::new("comp-reference-shot"), 60, &repo("Fixtures/reference_shot"), Full, DEFAULT_TILE_SIZE, &mut log, &mut cels, gpu).unwrap();
        let drawn = gpu.picture().unwrap();
        gpu.hold(&drawn, w, h);
        row("the card's picture, put back on the card from memory, is the same bytes", "identical", if gpu.picture().unwrap() == drawn { "identical".into() } else { "different".into() });
        drop(held);

        let ahead = viewer_on(&repo("Fixtures/reference_shot"));
        serve_logged(&ahead, &export, None, Some(&card), Ask::Play(0), Some(Draft));
        row("card, Draft: after play is pressed the other 239 frames are made ahead", "239 frames", format!("{} frames", render_ahead(&ahead, Some(&card))));
        let remembered = (0..240).filter(|&n| header(&ask(&ahead, Some(&card), n, Draft), "x-cached") == "1").count();
        row("and all 240 are then sent from memory", "240 of 240", format!("{remembered} of 240"));
        let cold = viewer_on(&repo("Fixtures/reference_shot"));
        row("a frame the card made ahead (frame 50) is byte for byte the one it makes when asked", "identical", same(&ask(&ahead, Some(&card), 50, Draft), &ask(&cold, Some(&card), 50, Draft)));
    }

    row(
        "exports never read remembered frames: the export code does not name them",
        "does not",
        if include_str!("../../../src/export.rs").contains("FrameCache") { "does".into() } else { "does not".into() },
    );

    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut text = format!(
        "# B-154: RAM preview\n\nWritten by `cargo test -p anime_compositor_app ram_preview`. \
         The reference shot, 1920 by 1080, through `serve_logged`, the function every frame \
         on screen comes from. \"Cold\" is a viewer that has remembered nothing. `x-cached` \
         is 1 when the frame was sent from memory. Card: {}.\n\n**{passed} of {} checks \
         pass.**\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        if on_card { card.lock().unwrap().about() } else { format!("none, card rows NOT RUN: {said_card}") },
        rows.len()
    );
    for (what, expected, actual) in &rows {
        text.push_str(&format!(
            "| {what} | {expected} | {actual} | {} |\n",
            if expected == actual { "PASS" } else { "FAIL" }
        ));
    }
    std::fs::write(repo("verification/B-154_ram_preview_table.md"), text).expect("write the table");
    let _ = std::fs::remove_dir_all(&root);
    assert_eq!(passed, rows.len(), "see verification/B-154_ram_preview_table.md");
}

/// The plan's proof: the 240-frame loop, asked for frame by frame the first time and again,
/// and made ahead from a press of play. Milliseconds are `serve_logged`'s own, the window's
/// whole work for a frame; the web view's transport is not in them (P-15).
///
/// Writes `verification/B-154_timing_table.md`.
#[test]
#[ignore = "a measurement; run it deliberately, in release, on the recorded machine"]
fn b154_loop_timing() {
    let export = Mutex::new(Export::default());
    let budget = 1000.0 / 24.0;
    let card = Mutex::new(Card::default());
    let said_card = card.lock().unwrap().set(DrawOn::Gpu);
    let on_card = card.lock().unwrap().on;
    let spread = |ms: &mut Vec<f64>| {
        ms.sort_by(f64::total_cmp);
        let late = ms.iter().filter(|&&m| m > budget).count();
        format!("{:.1} | {:.1} | {:.1} | {late}", ms[ms.len() / 2], ms[ms.len() * 95 / 100], ms[ms.len() - 1])
    };
    // Document 08's declared ten-layer fixture, blurred and graded, beside the reference shot.
    let declared = || {
        let (_project, root, text) = common::build_fixture("b154");
        let path = root.join("b154_project.json");
        std::fs::write(&path, &text).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
        Mutex::new(open(&path).unwrap_or_else(|d| panic!("open {}: {}", path.display(), d.message)))
    };
    let reference = || viewer_on(&repo("Fixtures/reference_shot"));
    let shots: [(&str, &dyn Fn() -> Mutex<Viewer>); 2] =
        [("reference shot", &reference), ("declared ten-layer fixture", &declared)];
    let mut rows = String::new();
    for (processor, card) in [("CPU", None), ("card", Some(&card))] {
        if card.is_some() && !on_card {
            rows.push_str(&format!("| card | | | NOT RUN: {said_card} | | | | |\n"));
            continue;
        }
        for (shot, make) in shots {
        for quality in [PreviewQuality::Draft, PreviewQuality::Full] {
            let viewer = make();
            for pass in ["first pass, each frame made", "second pass, from memory"] {
                let mut ms: Vec<f64> = (0..240)
                    .map(|n| {
                        let at = Instant::now();
                        let reply = serve_logged(&viewer, &export, None, card, Ask::Frame(n), Some(quality));
                        assert_eq!(reply.status(), 200);
                        at.elapsed().as_secs_f64() * 1000.0
                    })
                    .collect();
                rows.push_str(&format!("| {shot} | {processor} | {} | {pass} | {} |\n", quality.label(), spread(&mut ms)));
            }
            let held = viewer.lock().unwrap().ram.lock().unwrap().frames.held_bytes();
            let ahead = make();
            serve_logged(&ahead, &export, None, card, Ask::Play(0), Some(quality));
            let at = Instant::now();
            let made = render_ahead(&ahead, card);
            rows.push_str(&format!(
                "| {shot} | {processor} | {} | made ahead after play: {made} frames in {:.1} s; the loop holds {:.2} GiB | | | | |\n",
                quality.label(),
                at.elapsed().as_secs_f64(),
                held as f64 / (1u64 << 30) as f64,
            ));
        }
        }
    }
    let text = format!(
        "# B-154: the 240-frame loop, made and from memory\n\nWritten by `b154_loop_timing` in \
         `app/src/main.rs`, release build. Both shots are 1920 by 1080, 240 frames at 24 a \
         second, so a frame is due every {budget:.1} ms. Card: {}. Milliseconds are the window's \
         whole work for a frame (`serve_logged`), not the web view's transport.\n\n\
         | Shot | Drawn on | Quality | Pass | Median ms | 95th percentile ms | Slowest ms | Frames over {budget:.1} ms |\n\
         |---|---|---|---|---|---|---|---|\n{rows}",
        if on_card { card.lock().unwrap().about() } else { said_card.clone() },
    );
    std::fs::write(repo("verification/B-154_timing_table.md"), text).expect("write the table");
}
