use super::*;
use anime_compositor::WorkingBuffer;

/// Each `by` by `by` block of the picture averaged, premultiplied in linear light, then made
/// 8-bit sRGB as every picture the window sends is.
fn shrunk_here(full: &WorkingBuffer, by: usize) -> (usize, usize, Vec<u8>) {
    let (w, h) = (full.width() / by, full.height() / by);
    let mut out = WorkingBuffer::transparent(w, h);
    for y in 0..h {
        for x in 0..w {
            for c in 0..4 {
                let mut sum = 0.0f32;
                for dy in 0..by {
                    for dx in 0..by {
                        sum += full.data()[((y * by + dy) * full.width() + x * by + dx) * 4 + c];
                    }
                }
                out.data_mut()[(y * w + x) * 4 + c] = sum / (by * by) as f32;
            }
        }
    }
    (w, h, out.to_srgb8_straight())
}

fn same(a: &(usize, usize, Vec<u8>), b: &(usize, usize, Vec<u8>)) -> String {
    if a == b {
        format!("{} by {}, every byte the same", a.0, a.1)
    } else {
        let differ = a.2.iter().zip(&b.2).filter(|(x, y)| x != y).count();
        format!(
            "{} by {} against {} by {}, {differ} bytes differ",
            a.0, a.1, b.0, b.1
        )
    }
}

#[test]
fn d260_a_small_picture_is_its_full_picture_shrunk() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .to_path_buf();
    let viewer = Mutex::new(demo());
    let root = viewer.lock().unwrap().root.clone();
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut row = |what: &str, expected: &str, actual: String| {
        rows.push((what.into(), expected.into(), actual))
    };

    let undo_before = viewer.lock().unwrap().document.undo_labels().len();
    let shown_before = viewer.lock().unwrap().composition.clone();
    let ready_before = ready_frames(&viewer, None);

    let drawing = anime_compositor::media::decode(&root.join("layer2/layer2_000.png"))
        .expect("decode the drawing")
        .into_working();
    let small = thumbnail(&viewer, &Id::new("asset-layer2"), 0);
    row(
        "layer2's small picture is its first drawing, layer2_000.png, a twelfth each way",
        "160 by 90, every byte the same",
        small
            .as_ref()
            .map_or_else(|e| e.clone(), |s| same(s, &shrunk_here(&drawing, 12))),
    );

    let project = viewer.lock().unwrap().document.project().clone();
    let mut log = FrameLog::new(10);
    let frame = anime_compositor::compose::render_frame(
        &project,
        &Id::new("comp-reference-shot"),
        10,
        &root,
        DEFAULT_TILE_SIZE,
        &mut log,
    )
    .expect("render frame 10");
    let small = thumbnail(&viewer, &Id::new("comp-reference-shot"), 10);
    row(
        "the reference shot's small picture at frame 10 is its full frame 10, a twelfth each way",
        "160 by 90, every byte the same",
        small
            .as_ref()
            .map_or_else(|e| e.clone(), |s| same(s, &shrunk_here(&frame, 12))),
    );
    row(
        "and it is a picture, not an empty one",
        "some pixels drawn",
        small.as_ref().map_or_else(
            |e| e.clone(),
            |s| {
                if s.2.chunks(4).any(|p| p[3] > 0) {
                    "some pixels drawn".into()
                } else {
                    "all transparent".into()
                }
            },
        ),
    );
    row(
        "a name that is no item is refused",
        "refused",
        match thumbnail(&viewer, &Id::new("asset-nobody"), 0) {
            Ok(_) => "a picture".into(),
            Err(_) => "refused".into(),
        },
    );

    row(
        "asking changes nothing: the same composition on screen, no undo step, the same frames ready",
        "unchanged",
        if viewer.lock().unwrap().composition == shown_before
            && viewer.lock().unwrap().document.undo_labels().len() == undo_before
            && ready_frames(&viewer, None) == ready_before
        {
            "unchanged".into()
        } else {
            "changed".into()
        },
    );

    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut text = format!(
        "# D-260: the Project panel's small pictures\n\nWritten by `d260_a_small_picture_is_its_full_picture_shrunk` \
         in `app/src/main.rs`. `thumbnail` is the answer to the page's `/thumb`; the expected pictures are \
         shrunk here by the test's own loop, each 12 by 12 block of pixels averaged.\n\n**{passed} of {} checks pass.**\n\n\
         | Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        rows.len()
    );
    for (what, expected, actual) in &rows {
        text.push_str(&format!(
            "| {what} | {expected} | {actual} | {} |\n",
            if expected == actual { "PASS" } else { "FAIL" }
        ));
    }
    std::fs::write(repo.join("verification/D-260_thumbnails_table.md"), text)
        .expect("write the table");
    assert_eq!(
        passed,
        rows.len(),
        "see verification/D-260_thumbnails_table.md"
    );
}
