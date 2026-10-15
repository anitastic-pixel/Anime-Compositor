use super::*;

/// The frames `/ready` lists, as runs: "none", "0-239" or "5, 100-101".
fn listed(viewer: &Mutex<Viewer>, card: Option<&Mutex<Card>>) -> String {
    let answer = ready_frames(viewer, card);
    let frames: Vec<i64> = answer["frames"].as_array().map_or_else(Vec::new, |a| a.iter().filter_map(|f| f.as_i64()).collect());
    let mut runs: Vec<(i64, i64)> = Vec::new();
    for f in frames {
        match runs.last_mut() {
            Some((_, last)) if *last + 1 == f => *last = f,
            _ => runs.push((f, f)),
        }
    }
    if runs.is_empty() {
        return "none".into();
    }
    runs.iter()
        .map(|&(a, b)| if a == b { a.to_string() } else { format!("{a}-{b}") })
        .collect::<Vec<_>>()
        .join(", ")
}

fn bytes(viewer: &Mutex<Viewer>, field: &str) -> String {
    format!("{} bytes", ready_frames(viewer, None)[field].as_u64().unwrap_or(0))
}

fn hide_layer_1() -> Command {
    Command::SetLayerEnabled {
        composition: Id::new("comp-reference-shot"),
        layer_id: Id::new("layer-1"),
        value: false,
    }
}

#[test]
fn w40c_ready_frames_are_the_ones_memory_would_send() {
    use PreviewQuality::{Draft, Full};
    let export = Mutex::new(Export::default());
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut row = |what: &str, expected: &str, actual: String| {
        rows.push((what.to_string(), expected.to_string(), actual));
    };
    let ask = |v: &Mutex<Viewer>, card: Option<&Mutex<Card>>, n: i32, q: PreviewQuality| {
        serve_logged(v, &export, None, card, Ask::Frame(n), Some(q))
    };

    let viewer = Mutex::new(demo());
    row("a viewer that has made nothing: no frame is ready", "none", listed(&viewer, None));
    ask(&viewer, None, 100, Full);
    ask(&viewer, None, 101, Full);
    row("frames 100 and 101 made at Full: those two are ready", "100-101", listed(&viewer, None));
    row("the memory holds those two frames: 1920 by 1080, four bytes a pixel, twice", "16588800 bytes", bytes(&viewer, "held"));
    row("the memory setting given is the Preferences setting", &format!("{} bytes", ram_ceiling()), bytes(&viewer, "setting"));
    let before = ready_frames(&viewer, None).to_string();
    row("asking changes nothing: asked again, the same answer", &before, ready_frames(&viewer, None).to_string());

    ask(&viewer, None, 5, Draft);
    row("frame 5 made at Draft: at Draft only frame 5 is ready, not the Full frames", "5", listed(&viewer, None));
    ask(&viewer, None, 100, Full);
    row("back at Full: 100 and 101 are ready, not the Draft frame", "100-101", listed(&viewer, None));

    viewer.lock().unwrap().solo = vec![Id::new("layer-2")];
    row("a layer soloed: nothing is ready, no frame was made with it soloed", "none", listed(&viewer, None));
    viewer.lock().unwrap().solo.clear();
    row("unsoloed: 100 and 101 are ready again", "100-101", listed(&viewer, None));

    edit(&viewer, hide_layer_1());
    row("after an edit (layer 1 hidden) nothing is ready", "none", listed(&viewer, None));
    undo(&viewer);
    row("after undo the frames from before the edit are ready again", "100-101", listed(&viewer, None));

    let ahead = Mutex::new(demo());
    serve_logged(&ahead, &export, None, None, Ask::Play(0), Some(Draft));
    render_ahead(&ahead, None);
    row("Draft, play pressed at frame 0 and the loop made ahead: every frame is ready", "0-239", listed(&ahead, None));
    row("the loop holds 240 Draft frames: 480 by 270, four bytes a pixel", "124416000 bytes", bytes(&ahead, "held"));

    let card = Mutex::new(Card::default());
    let said_card = card.lock().unwrap().set(DrawOn::Gpu);
    let on_card = card.lock().unwrap().on;
    if on_card {
        row("card: the CPU's frames are not ready for the card", "none", listed(&viewer, Some(&card)));
        ask(&viewer, Some(&card), 100, Full);
        row("frame 100 drawn on the card: it is ready for the card", "100", listed(&viewer, Some(&card)));
        row("and the CPU's 100 and 101 are still ready for the CPU", "100-101", listed(&viewer, None));
    }

    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut text = format!(
        "# W-40c: the ready frames\n\nWritten by `w40c_ready_frames_are_the_ones_memory_would_send` \
         in `app/src/main.rs`. The reference shot, 1920 by 1080, through `serve_logged`, the \
         function every frame on screen comes from, and `ready_frames`, the answer to the \
         page's `/ready`. Frames are listed as runs. Card: {}.\n\n**{passed} of {} checks \
         pass.**\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        if on_card { card.lock().unwrap().about() } else { format!("none, card rows NOT RUN: {said_card}") },
        rows.len()
    );
    for (what, expected, actual) in &rows {
        text.push_str(&format!("| {what} | {expected} | {actual} | {} |\n", if expected == actual { "PASS" } else { "FAIL" }));
    }
    let out = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .join("verification/W-40c_ready_frames_table.md");
    std::fs::write(out, text).expect("write the table");
    assert_eq!(passed, rows.len(), "see verification/W-40c_ready_frames_table.md");
}
