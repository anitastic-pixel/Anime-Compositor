use super::*;

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .join(rel)
}

fn header(r: &Response<Vec<u8>>, name: &str) -> String {
    r.headers().get(name).map_or_else(String::new, |v| {
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

/// A 20 px feathered rectangle on layer 2, the kind of mask the L25 lag shot drags.
fn mask_on_layer_2(right: f64) -> Command {
    Command::SetMasks {
        composition: Id::new("comp-reference-shot"),
        layer_id: Id::new("layer-2"),
        masks: vec![anime_compositor::mask::Mask {
            feather_px: 20.0,
            ..anime_compositor::mask::Mask::polygon(vec![
                (200.0, 150.0),
                (right, 150.0),
                (right, 900.0),
                (200.0, 900.0),
            ])
        }],
    }
}

/// D-898 / B-778: the In motion picture. While the hand drags or scrubs, the page asks for
/// `?q=motion` whatever quality is chosen: an eighth of the composition each way, masks drawn at
/// that size, at most four moments of any shutter. It is labelled, never kept in the frame
/// memory, leaves the chosen quality alone, and the picture asked for once the hand stops is byte
/// for byte the one that quality makes on its own, after an edit, an undo, a frame change, a
/// quality change and a switch of composition.
///
/// Writes `verification/B-778_in_motion_table.md`.
#[test]
fn b778_in_motion_is_labelled_never_kept_and_replaced_by_the_chosen_picture() {
    use PreviewQuality::{Draft, Full, Half, Motion};
    let export = Mutex::new(Export::default());
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut row = |what: &str, expected: &str, actual: String| {
        rows.push((what.to_string(), expected.to_string(), actual));
    };
    let ask = |v: &Mutex<Viewer>, card: Option<&Mutex<Card>>, n: i32, q: PreviewQuality| {
        serve_logged(v, &export, None, card, Ask::Frame(n), Some(q))
    };
    let held = |v: &Mutex<Viewer>| {
        let ram = Arc::clone(&v.lock().unwrap().ram);
        let bytes = ram.lock().unwrap().frames.held_bytes();
        bytes
    };

    // `moving` is the viewer the hand is on; `alone` is asked only for the chosen picture.
    let moving = Mutex::new(demo());
    let alone = Mutex::new(demo());
    let full = ask(&moving, None, 30, Full);
    let before = held(&moving);
    let rough = ask(&moving, None, 30, Motion);
    row(
        "frame 30 In motion: labelled, not final, an eighth each way of 1920 by 1080",
        "In motion true 240 135",
        format!(
            "{} {} {} {}",
            header(&rough, "x-quality"),
            header(&rough, "x-differs"),
            header(&rough, "x-width"),
            header(&rough, "x-height")
        ),
    );
    row("asked again, it is made again, not sent from memory", "0", header(&ask(&moving, None, 30, Motion), "x-cached"));
    row(
        "the frame memory holds no more than before it",
        &format!("{before} bytes"),
        format!("{} bytes", held(&moving)),
    );
    row(
        "the quality chosen is still Full",
        "Full",
        moving.lock().unwrap().quality.label().to_string(),
    );
    let again = ask(&moving, None, 30, Full);
    row("Full afterwards is the Full frame remembered from before", "1", header(&again, "x-cached"));
    row(
        "and is Full, the same as export, byte for byte frame 30 at Full asked for alone",
        "Full false identical",
        format!(
            "{} {} {}",
            header(&again, "x-quality"),
            header(&again, "x-differs"),
            same(&again, &ask(&alone, None, 30, Full))
        ),
    );
    row("which is the Full frame from before", "identical", same(&again, &full));

    // The hand moves over each change; the picture after it must be the chosen one, made fresh.
    let mut step = |what: &str, n: i32, q: PreviewQuality, moving: &Mutex<Viewer>, alone: &Mutex<Viewer>| {
        let rough = ask(moving, None, n, Motion);
        let sharp = ask(moving, None, n, q);
        let cold = ask(alone, None, n, q);
        let cold_rough = ask(alone, None, n, Motion);
        row(
            &format!("{what}: In motion, then {} once still, is that {} asked for alone; the In motion frame is the one asked for alone", q.label(), q.label()),
            "In motion identical identical",
            format!("{} {} {}", header(&rough, "x-quality"), same(&sharp, &cold), same(&rough, &cold_rough)),
        );
    };
    for v in [&moving, &alone] {
        edit(v, mask_on_layer_2(1500.0));
    }
    step("a feathered mask added to layer 2", 30, Full, &moving, &alone);
    for v in [&moving, &alone] {
        edit(v, mask_on_layer_2(1100.0));
    }
    step("the mask dragged narrower", 30, Full, &moving, &alone);
    for v in [&moving, &alone] {
        undo(v);
    }
    step("undone", 30, Full, &moving, &alone);
    step("a frame change", 60, Full, &moving, &alone);
    step("a quality change (Half)", 60, Half, &moving, &alone);
    step("a quality change (Draft)", 61, Draft, &moving, &alone);
    for v in [&moving, &alone] {
        frame_boxes::switch(v);
    }
    step("a switch of composition", 61, Full, &moving, &alone);

    // On the card, when there is one.
    let card = Mutex::new(Card::default());
    card.lock().unwrap().set(DrawOn::Gpu);
    if card.lock().unwrap().on {
        let on_card = Mutex::new(demo());
        let cold = Mutex::new(demo());
        for v in [&on_card, &cold] {
            edit(v, mask_on_layer_2(1500.0));
        }
        let rough = ask(&on_card, Some(&card), 30, Motion);
        row(
            "card: frame 30 asked for in motion, with the mask, is drawn at Draft there (In motion measured slower on the card), labelled so, 480 by 270",
            "Draft true 480 270",
            format!(
                "{} {} {} {}",
                header(&rough, "x-quality"),
                header(&rough, "x-differs"),
                header(&rough, "x-width"),
                header(&rough, "x-height")
            ),
        );
        row(
            "card: Full afterwards is byte for byte the card's Full asked for alone",
            "identical",
            same(&ask(&on_card, Some(&card), 30, Full), &ask(&cold, Some(&card), 30, Full)),
        );
        row(
            "card: the frame asked for in motion is the card's Draft asked for alone",
            "identical",
            same(&rough, &ask(&cold, Some(&card), 30, Draft)),
        );
        row(
            "card: the quality chosen is still Full",
            "Full",
            on_card.lock().unwrap().quality.label().to_string(),
        );
    } else {
        row("card: there is no card on this machine, so these rows are skipped", "skipped", "skipped".into());
    }

    // The drawing taken down for a frame in motion, held beside the drawing in the cel cache.
    {
        use anime_compositor::cache::CelCache;
        let path = repo("Fixtures/reference_shot/layer1/layer1_000.png");
        let interp = anime_compositor::model::Interpretation::default();
        let mut cache = CelCache::with_budget(512 * 1024 * 1024);
        let whole = cache.decoded(&path, interp).expect("decode the cel");
        let held_whole = cache.held_bytes();
        let small = |w: &anime_compositor::WorkingBuffer| {
            anime_compositor::WorkingBuffer::opaque(w.width() / 8, w.height() / 8)
        };
        let taken = cache.smaller(&path, interp, 8, || small(&whole));
        row(
            "cache: the drawing taken down an eighth is counted in the memory budget",
            &format!("{} bytes more", taken.width() * taken.height() * 4 * 4),
            format!("{} bytes more", cache.held_bytes() - held_whole),
        );
        let mut made = false;
        let again = cache.smaller(&path, interp, 8, || {
            made = true;
            small(&whole)
        });
        row(
            "cache: asked again, it is the one held, not made again",
            "true false",
            format!("{} {made}", std::sync::Arc::ptr_eq(&taken, &again)),
        );
        let asked = cache.decoded(&path, interp).expect("decode the cel");
        row(
            "cache: a request that is not in motion still gets the whole drawing, never the eighth",
            &format!("true {} {}", whole.width(), whole.height()),
            format!("{} {} {}", std::sync::Arc::ptr_eq(&asked, &whole), asked.width(), asked.height()),
        );
        let mut made = false;
        cache.smaller(&path, interp, 4, || {
            made = true;
            small(&whole)
        });
        row("cache: another size is its own, never the eighth", "true", made.to_string());
    }

    // Fewer moments of a shutter, in motion only.
    let mut project = moving.lock().unwrap().document.project().clone();
    for c in &mut project.compositions {
        c.motion_blur.enabled = false;
    }
    row(
        "with motion blur switched off its 16 samples cost nothing, so nothing is copied",
        "none",
        anime_compositor::preview::fewer_samples(&project).map_or("none".into(), |_| "a copy".into()),
    );
    for c in &mut project.compositions {
        c.motion_blur.enabled = true;
    }
    row(
        "the reference shot's 16 motion blur samples are cut to 4 in motion",
        "16 4",
        format!(
            "{} {}",
            project.compositions[0].motion_blur.samples,
            anime_compositor::preview::fewer_samples(&project)
                .map_or(0, |p| p.compositions[0].motion_blur.samples)
        ),
    );
    for c in &mut project.compositions {
        c.motion_blur.samples = 4;
    }
    row(
        "a shot with 4 or fewer is drawn as it is, uncopied",
        "none",
        anime_compositor::preview::fewer_samples(&project).map_or("none".into(), |_| "a copy".into()),
    );

    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut out = format!(
        "# B-778: In motion (D-898)\n\n\
         Written by `b778_in_motion_is_labelled_never_kept_and_replaced_by_the_chosen_picture` in \
         `app/src/tests/in_motion.rs`.\n\n\
         While the hand drags or scrubs, the page asks for each frame In motion, whatever quality \
         is chosen: an eighth of the composition each way, masks drawn at that size, at most four \
         moments of any shutter. Each row checks that it is labelled, never kept, and that the \
         picture asked for once the hand stops is byte for byte the one the chosen quality makes \
         on a second viewer given the same changes and never moved. On the reference shot.\n\n\
         **{passed} of {} checks pass.**\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        rows.len()
    );
    for (check, expected, actual) in &rows {
        out.push_str(&format!(
            "| {check} | {expected} | {actual} | {} |\n",
            if expected == actual { "pass" } else { "FAIL" }
        ));
    }
    out.push_str(
        "\n## What this does not cover\n\nHow quick it is and how it feels: the delay is measured \
         by the lag harness (D-898's evidence) and judged in `verification/B-778_playtest.md`.\n",
    );
    std::fs::write(repo("verification/B-778_in_motion_table.md"), out).expect("write the table");
    assert!(rows.iter().all(|(_, e, a)| e == a), "{rows:#?}");
}
