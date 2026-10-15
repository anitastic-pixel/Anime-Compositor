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

/// The parts asked for: the page's fractions, and what they are in words.
const PARTS: [(&str, &str); 5] = [
    ("0,0,0.5,1", "the left half"),
    ("0.3,0.3,0.7,0.6", "a box in the middle"),
    ("0.8,0.9,1,1", "the bottom right corner"),
    ("0.5,0.5,0.5,0.5", "a single point"),
    ("0.25,0.25,0.75,0.75", "the middle quarter, as at 200%"),
];

/// The pixels `(x, y, w, h)` of an eight-bit picture `width` across.
fn cut(pixels: &[u8], width: usize, (x, y, w, h): (usize, usize, usize, usize)) -> Vec<u8> {
    (y..y + h).flat_map(|row| pixels[(row * width + x) * 4..(row * width + x + w) * 4].to_vec()).collect()
}

/// The `x-part` header, `x,y,w,h` in the frame's pixels.
fn part_of(r: &Response<Vec<u8>>) -> Option<(usize, usize, usize, usize)> {
    let n: Vec<usize> = header(r, "x-part").split(',').filter_map(|v| v.parse().ok()).collect();
    (n.len() == 4).then(|| (n[0], n[1], n[2], n[3]))
}

/// The pixels a part must be: every pixel the fractions touch, and one more on every side,
/// inside the frame. `None` when that is the whole frame.
fn expected(seen: &str, w: usize, h: usize) -> Option<(usize, usize, usize, usize)> {
    let f: Vec<f64> = seen.split(',').map(|v| v.parse().unwrap()).collect();
    let x0 = ((f[0] * w as f64).floor() as usize).saturating_sub(1);
    let y0 = ((f[1] * h as f64).floor() as usize).saturating_sub(1);
    let x1 = ((f[2] * w as f64).ceil() as usize + 1).min(w);
    let y1 = ((f[3] * h as f64).ceil() as usize + 1).min(h);
    ((x0, y0, x1, y1) != (0, 0, w, h)).then_some((x0, y0, x1 - x0, y1 - y0))
}

/// One part checked against the whole frame: `None` when it passes, or what is wrong.
fn wrong(whole: &Response<Vec<u8>>, part: &Response<Vec<u8>>, seen: &str) -> Option<String> {
    if whole.status() != 200 || part.status() != 200 {
        return Some(format!("status {} and {}", whole.status(), part.status()));
    }
    let (w, h): (usize, usize) = (header(whole, "x-width").parse().unwrap_or(0), header(whole, "x-height").parse().unwrap_or(0));
    let want = expected(seen, w, h);
    if part_of(part) != want {
        return Some(format!("part {:?} sent, {want:?} expected, of {w} by {h}", part_of(part)));
    }
    let checks = [
        (header(part, "x-width") == header(whole, "x-width") && header(part, "x-height") == header(whole, "x-height"), "the page is told the whole frame's size"),
        (header(part, "x-drawn-on") == header(whole, "x-drawn-on"), "the same processor drew it"),
    ];
    if let Some((_, what)) = checks.iter().find(|(ok, _)| !ok) {
        return Some(format!("{what}: no"));
    }
    let should = want.map_or_else(|| whole.body().clone(), |p| cut(whole.body(), w, p));
    if should != *part.body() {
        let differ = should.iter().zip(part.body()).filter(|(a, b)| a != b).count();
        return Some(format!("{} bytes sent, {} expected, {differ} differ", part.body().len(), should.len()));
    }
    None
}

struct Shot {
    group: &'static str,
    name: String,
    path: Option<PathBuf>,
    frames: Vec<i32>,
}

fn viewer_of(shot: &Shot) -> Mutex<Viewer> {
    match &shot.path {
        Some(path) => Mutex::new(open(path).unwrap_or_else(|d| panic!("open {}: {}", path.display(), d.message))),
        None => Mutex::new(demo()),
    }
}

fn shots() -> Vec<Shot> {
    let mut shots = vec![Shot { group: "reference shot", name: "the reference shot".into(), path: None, frames: vec![0, 100, 239] }];
    // Adjustment layers and Light Wraps draw over the whole frame so far; motion blur, frame
    // blending and the card's own blurs are made before the picture is laid.
    for (group, folder) in [
        ("adjustment layers", "adjust"),
        ("Light Wrap", "light_wrap"),
        ("motion blur", "motion_blur"),
        ("frame blending", "frame_blending"),
        ("Bloom", "bloom"),
        ("Directional Blur", "directional_blur"),
        ("Glow", "glow"),
    ] {
        let mut files: Vec<PathBuf> = std::fs::read_dir(repo(&format!("Fixtures/{folder}")))
            .expect("read a fixture folder")
            .map(|e| e.expect("a folder entry").path())
            .filter(|p| p.extension().is_some_and(|x| x == "json") && !p.file_name().unwrap().to_string_lossy().starts_with("expected"))
            .collect();
        files.sort();
        for path in files {
            if open(&path).is_err() {
                continue;
            }
            let name = path.file_stem().unwrap().to_string_lossy().into_owned();
            shots.push(Shot { group, name, path: Some(path), frames: vec![0, 2] });
        }
    }
    shots
}

#[test]
fn b158_part_on_screen_is_the_same_pixels_as_the_whole_frame() {
    use PreviewQuality::{Draft, Full};
    let export = Mutex::new(Export::default());
    let card = Mutex::new(Card::default());
    let said_card = card.lock().unwrap().set(DrawOn::Gpu);
    let on_card = card.lock().unwrap().on;
    let shots = shots();

    // Per group and processor: parts checked, parts passed, and the first failure.
    let mut groups: Vec<(String, &str, usize, usize, String, usize)> = Vec::new();
    let mut rows = String::new();
    let processors: Vec<(&str, Option<&Mutex<Card>>)> =
        if on_card { vec![("CPU", None), ("card", Some(&card))] } else { vec![("CPU", None)] };
    for (processor, card) in &processors {
        for shot in &shots {
            let key = format!("{} ({processor})", shot.group);
            if !groups.iter().any(|g| g.0 == key) {
                groups.push((key.clone(), processor, 0, 0, String::new(), 0));
            }
            for quality in [Full, Draft] {
                for &n in &shot.frames {
                    // The whole frame, made by a viewer that has remembered nothing.
                    let whole = serve_seen(&viewer_of(shot), &export, None, *card, Ask::Frame(n), Some(quality), None);
                    let viewer = viewer_of(shot);
                    for (seen, words) in PARTS {
                        let part = serve_seen(&viewer, &export, None, *card, Ask::Frame(n), Some(quality), Some(seen));
                        let result = wrong(&whole, &part, seen);
                        let g = groups.iter_mut().find(|g| g.0 == key).unwrap();
                        // A part that grows to the whole frame is sent whole; counted apart.
                        let w: usize = header(&whole, "x-width").parse().unwrap_or(0);
                        let h: usize = header(&whole, "x-height").parse().unwrap_or(0);
                        if expected(seen, w, h).is_none() {
                            g.5 += 1;
                        }
                        g.2 += 1;
                        match &result {
                            None => g.3 += 1,
                            Some(why) if g.4.is_empty() => g.4 = format!("{} frame {n}, {}, {words}: {why}", shot.name, quality.label()),
                            Some(_) => {}
                        }
                        if shot.path.is_none() {
                            rows.push_str(&format!(
                                "| {processor} | frame {n}, {} | {words} | {} | {} |\n",
                                quality.label(),
                                header(&part, "x-part"),
                                match &result { None => "PASS".to_string(), Some(why) => format!("FAIL: {why}") },
                            ));
                        }
                    }
                }
            }
        }
    }

    // Memory (B-154): a part is cut from a whole frame already made, and is never kept as one.
    let mut memory: Vec<(&str, String, String)> = Vec::new();
    let viewer = Mutex::new(demo());
    let seen = Some(PARTS[4].0);
    let first_part = serve_seen(&viewer, &export, None, None, Ask::Frame(100), Some(Full), seen);
    let whole = serve_seen(&viewer, &export, None, None, Ask::Frame(100), Some(Full), None);
    memory.push(("after only a part of frame 100 was made, the whole frame is made, not sent from memory", "0".into(), header(&whole, "x-cached")));
    let again = serve_seen(&viewer, &export, None, None, Ask::Frame(100), Some(Full), seen);
    memory.push(("with the whole frame remembered, a part of it is cut from memory", "1".into(), header(&again, "x-cached")));
    memory.push(("and is the same pixels as the part made", "identical".into(), if again.body() == first_part.body() && part_of(&again).is_some() { "identical".into() } else { "different".into() }));
    let everything = serve_seen(&viewer, &export, None, None, Ask::Frame(100), Some(Full), Some("0,0,1,1"));
    memory.push(("all of the picture on screen is the whole frame, with no part header", "none".into(), if header(&everything, "x-part").is_empty() { "none".into() } else { header(&everything, "x-part") }));
    memory.push(("and is byte for byte the whole frame", "identical".into(), if everything.body() == whole.body() { "identical".into() } else { "different".into() }));
    let playing = serve_seen(&viewer, &export, None, None, Ask::Play(0), Some(Full), seen);
    memory.push(("playback is sent whole, whatever is on screen", "none".into(), if header(&playing, "x-part").is_empty() { "none".into() } else { header(&playing, "x-part") }));
    let unreadable = serve_seen(&viewer, &export, None, None, Ask::Frame(100), Some(Full), Some("left,top"));
    memory.push(("an unreadable part is the whole frame, never a guess", "none".into(), if header(&unreadable, "x-part").is_empty() { "none".into() } else { header(&unreadable, "x-part") }));
    memory.push(("exports never read a part: the export code does not name it", "does not".into(), if include_str!("../../../src/export.rs").contains("render_part") { "does".into() } else { "does not".into() }));

    let checks = groups.iter().map(|g| g.2).sum::<usize>() + memory.len();
    let passed = groups.iter().map(|g| g.3).sum::<usize>() + memory.iter().filter(|(_, e, a)| e == a).count();
    let mut text = format!(
        "# B-158: only the part of the picture on screen\n\nWritten by `cargo test -p anime_compositor_app part_on_screen`. \
         Zoomed in, the page asks for the part of the picture it can show, as fractions of the picture across and down, \
         and the window makes and sends only those pixels, with one pixel spare on every side. **The rule: every part is \
         byte for byte the same pixels of the whole frame**, made by a viewer that has remembered nothing, with the same \
         size told to the page and the same processor. Parts asked for, as fractions (left, top, right, bottom): {}. \
         Card: {}.\n\n**{passed} of {checks} checks pass.**\n\n## Each group\n\n| Group | Parts checked | Of those, sent whole (with its spare pixel the part is all of the frame) | Pass | First failure |\n|---|---:|---:|---:|---|\n",
        PARTS.iter().map(|(f, w)| format!("{w} ({f})")).collect::<Vec<_>>().join("; "),
        if on_card { card.lock().unwrap().about() } else { format!("none, card rows NOT RUN: {said_card}") },
    );
    for (group, _, checked, ok, first, whole) in &groups {
        text.push_str(&format!("| {group} | {checked} | {whole} | {ok} of {checked} | {} |\n", if first.is_empty() { "none" } else { first }));
    }
    text.push_str("\n## Memory, playback and export\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
    for (what, expected, actual) in &memory {
        text.push_str(&format!("| {what} | {expected} | {actual} | {} |\n", if expected == actual { "PASS" } else { "FAIL" }));
    }
    text.push_str("\n## The reference shot, every part\n\n| Drawn on | Frame | Part | Pixels sent (x, y, across, down) | Result |\n|---|---|---|---|---|\n");
    text.push_str(&rows);
    std::fs::write(repo("verification/B-158_part_table.md"), text).expect("write the table");
    assert_eq!(passed, checks, "see verification/B-158_part_table.md");
}

/// The saving: the reference shot and the declared ten-layer fixture, each frame asked for
/// whole and then as the middle quarter a 200% zoom shows, by viewers that remember nothing.
///
/// Writes `verification/B-158_timing_raw.md`; the committed table is made from three runs.
#[test]
#[ignore = "a measurement; run it deliberately, in release, on the recorded machine"]
fn b158_part_timing() {
    let export = Mutex::new(Export::default());
    let card = Mutex::new(Card::default());
    let said_card = card.lock().unwrap().set(DrawOn::Gpu);
    let on_card = card.lock().unwrap().on;
    let declared = || {
        let (_project, root, text) = common::build_fixture("b158");
        let path = root.join("b158_project.json");
        std::fs::write(&path, &text).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
        Mutex::new(open(&path).unwrap_or_else(|d| panic!("open {}: {}", path.display(), d.message)))
    };
    let reference = || Mutex::new(demo());
    let shots: [(&str, &dyn Fn() -> Mutex<Viewer>); 2] =
        [("reference shot", &reference), ("declared ten-layer fixture", &declared)];
    let mut rows = String::new();
    for (processor, card) in [("CPU", None), ("card", Some(&card))] {
        if card.is_some() && !on_card {
            rows.push_str(&format!("| card | | | NOT RUN: {said_card} | |\n"));
            continue;
        }
        for (shot, make) in shots {
            for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
                let mut median = Vec::new();
                for seen in [None, Some(PARTS[4].0)] {
                    let viewer = make();
                    // Warm: the cels read once, so the files are not what is timed.
                    serve_seen(&viewer, &export, None, card, Ask::Frame(0), Some(quality), seen);
                    let mut ms: Vec<f64> = (1..49)
                        .map(|n| {
                            let at = Instant::now();
                            let reply = serve_seen(&viewer, &export, None, card, Ask::Frame(n), Some(quality), seen);
                            assert_eq!(reply.status(), 200);
                            at.elapsed().as_secs_f64() * 1000.0
                        })
                        .collect();
                    ms.sort_by(f64::total_cmp);
                    median.push(ms[ms.len() / 2]);
                }
                rows.push_str(&format!("| {shot} | {processor} | {} | {:.1} | {:.1} |\n", quality.label(), median[0], median[1]));
            }
        }
    }
    let text = format!(
        "# B-158: whole frame and the part on screen, one run\n\nWritten by `b158_part_timing`, release build. Card: {}. \
         Median of frames 1 to 48, each made (not from memory), in milliseconds of the window's whole work for a frame.\n\n\
         | Shot | Drawn on | Quality | Whole frame ms | Middle quarter ms |\n|---|---|---|---:|---:|\n{rows}",
        if on_card { card.lock().unwrap().about() } else { said_card.clone() },
    );
    std::fs::write(repo("verification/B-158_timing_raw.md"), text).expect("write the table");
}
