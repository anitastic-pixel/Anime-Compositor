//! Wave 12 lane X: the page's items from the tutorial follow-alongs (D-1072 on). Each test runs
//! the page's own marked code under Node, sends what it asks for to the window's own command
//! handler, pins the lines that wire it in, and writes its table under `verification/`.

use super::*;

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .join(rel)
}

fn page() -> String {
    std::fs::read_to_string(repo("app/ui/index.html")).expect("read the page")
}

/// Runs `body` under Node with the names `names` taken out of the page's code between
/// `// ---- {marker} begin` and `// ---- {marker} end`. `body` prints with `out(value)`.
fn node(marker: &str, names: &str, body: &str) -> serde_json::Value {
    let script = format!(
        "const fs = require('fs');\n\
         const page = fs.readFileSync(process.argv[2], 'utf8');\n\
         const from = page.indexOf('// ---- {marker} begin');\n\
         const to = page.indexOf('// ---- {marker} end');\n\
         if (from < 0 || to < from) throw new Error('the markers are not in the page');\n\
         const out = (v) => console.log(JSON.stringify(v));\n\
         const {{ {names} }} = new Function(page.slice(from, to) + '\\nreturn {{ {names} }};')();\n\
         {body}\n"
    );
    let path = std::env::temp_dir().join(format!("w12x_{}.js", marker.replace([' ', '/'], "_")));
    std::fs::write(&path, script).expect("write the check");
    let out = std::process::Command::new("node")
        .arg(&path)
        .arg(repo("app/ui/index.html"))
        .output()
        .expect("the page's code runs under Node, which must be installed");
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    serde_json::from_slice(&out.stdout).expect("JSON")
}

type Rows = Vec<(String, String, String)>;

fn table(file: &str, title: &str, test: &str, intro: &str, rows: &Rows) {
    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut text = format!(
        "# {title}\n\nWritten by `{test}` in `app/src/tests/w12x.rs`.\n\n{intro}\n\n\
         **{passed} of {} checks pass.**\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        rows.len()
    );
    for (check, expected, actual) in rows {
        text.push_str(&format!(
            "| {check} | {expected} | {actual} | {} |\n",
            if expected == actual { "pass" } else { "FAIL" }
        ));
    }
    std::fs::write(repo(&format!("verification/{file}")), text).expect("write the table");
    assert!(rows.iter().all(|(_, e, a)| e == a), "{rows:#?}");
}

/// D-1072 / B-952: Composition > Crop comp to region of interest. The composition takes the
/// box's size and every unparented layer moves by the box's corner, so the picture in the box is
/// the same before and after.
#[test]
fn b952_crop_to_region() {
    let mut rows: Rows = Vec::new();
    // The page's own code on made-up compositions, 200 by 100.
    let asked = node(
        "B-952",
        "cropSteps",
        "const layers = [\n\
           { id: 'a', name: 'A', transform: { position: { base: [100, 50] } } },\n\
           { id: 'k', name: 'K', transform: { position: { base: [0, 0], keyframes: [{ frame: 0, value: [10, 20] }, { frame: 12, value: [30, 40, 5] }] } } },\n\
           { id: 's', name: 'S', transform: { position: { x: { base: 5 }, y: { base: 6, keyframes: [{ frame: 3, value: 60 }] } } } },\n\
           { id: 'c', name: 'C', parent: 'a', transform: { position: { base: [1, 1] } } },\n\
         ];\n\
         const comp = (more) => Object.assign({ width: 200, height: 100, layers }, more);\n\
         const box = [0.25, 0.1, 0.755, 0.9];\n\
         const all = cropSteps(comp({}), box);\n\
         out({\n\
           steps: all.steps.join(' then '),\n\
           size: all.width + ' by ' + all.height,\n\
           corner: cropSteps(comp({}), [0, 0, 0.5, 0.5]).steps.join(' then '),\n\
           tiny: cropSteps(comp({}), [0.5, 0.5, 0.5001, 0.5001]).steps.join(' then '),\n\
           locked: cropSteps(comp({ layers: [...layers, { id: 'l', name: 'Sky', locked: true, transform: { position: { base: [0, 0] } } }] }), box).why,\n\
           lockedChild: cropSteps(comp({ layers: [...layers, { id: 'l', name: 'Sky', parent: 'a', locked: true, transform: { position: { base: [0, 0] } } }] }), box).steps.length,\n\
           camera: cropSteps(comp({ camera: { position: { base: [100, 50] } } }), box).why,\n\
         });",
    );
    let get = |k: &str| asked[k].as_str().map(str::to_string).unwrap_or_else(|| asked[k].to_string());
    for (what, field, want) in [
        (
            "a box from (50, 10) to (151, 90): each unparented Position, every key, moved by -50, -10, in one drag, then the size",
            "steps",
            "/property.set_base?layer=a&prop=position&drag=1&value=50%2C40 then \
             /property.set_base?layer=k&prop=position&drag=1&frame=0&value=-40%2C10 then \
             /property.set_base?layer=k&prop=position&drag=1&frame=12&value=-20%2C30%2C5 then \
             /property.set_base?layer=s&prop=position_x&drag=1&value=-45 then \
             /property.set_base?layer=s&prop=position_y&drag=1&frame=3&value=50 then \
             /property.drag_end then /composition.set_settings?width=101&height=80",
        ),
        ("its new size, rounded to whole pixels", "size", "101 by 80"),
        ("a box from the top-left corner moves nothing, only the size", "corner", "/composition.set_settings?width=100&height=50"),
        ("a box under a pixel still leaves a 1 by 1 composition", "tiny", "/property.set_base?layer=a&prop=position&drag=1&value=0%2C0 then /property.set_base?layer=k&prop=position&drag=1&frame=0&value=-90%2C-30 then /property.set_base?layer=k&prop=position&drag=1&frame=12&value=-70%2C-10%2C5 then /property.set_base?layer=s&prop=position_x&drag=1&value=-95 then /property.set_base?layer=s&prop=position_y&drag=1&frame=3&value=10 then /property.drag_end then /composition.set_settings?width=1&height=1"),
        ("an unparented locked layer: refused, nothing sent", "locked", "Unlock Sky first: a locked layer cannot be moved, so nothing is cropped."),
        ("a locked layer with a parent is not moved, so it does not stop the crop", "lockedChild", "7"),
        ("a composition with a camera: refused, nothing sent", "camera", "This composition has a camera, which a crop does not move, so it is not cropped."),
    ] {
        rows.push((what.into(), want.into(), get(field)));
    }

    // The window: the page's crop on the reference shot, drawn before and after.
    let viewer = Mutex::new(demo());
    let draw = |viewer: &Mutex<Viewer>, frame: i32| {
        let (project, comp, root) = {
            let held = viewer.lock().expect("lock");
            (held.document.project().clone(), held.composition.clone(), held.root.clone())
        };
        anime_compositor::compose::render_frame(&project, &comp, frame, &root, DEFAULT_TILE_SIZE, &mut FrameLog::new(0))
            .expect("the frame draws")
    };
    let comp_json = || -> serde_json::Value {
        let s: serde_json::Value = serde_json::from_str(&state(&viewer)).expect("the state is JSON");
        let id = s["composition"].clone();
        s["project"]["compositions"].as_array().expect("compositions").iter().find(|c| c["id"] == id).expect("the comp").clone()
    };
    let comp = comp_json();
    let (w, h) = (comp["width"].as_u64().unwrap() as usize, comp["height"].as_u64().unwrap() as usize);
    let box_ = [0.3, 0.25, 0.8, 0.85];
    let asked = node(
        "B-952",
        "cropSteps",
        &format!("out(cropSteps({comp}, {box_:?}));"),
    );
    let steps: Vec<String> = asked["steps"].as_array().map(|a| a.iter().map(|s| s.as_str().unwrap().to_string()).collect()).unwrap_or_default();
    rows.push(("the reference shot: no refusal".into(), "null".into(), asked["why"].to_string().replace("\"", "")));
    let (x0, y0) = ((box_[0] * w as f64).round() as usize, (box_[1] * h as f64).round() as usize);
    let (nw, nh) = (asked["width"].as_u64().unwrap_or(0) as usize, asked["height"].as_u64().unwrap_or(0) as usize);
    let frames = [0, 12, 24];
    let before: Vec<_> = frames.iter().map(|&f| draw(&viewer, f)).collect();
    let depth = viewer.lock().expect("lock").document.undo_depth();
    for request in &steps {
        let request = request.trim_start_matches('/');
        let (what, query) = request.split_once('?').unwrap_or((request, ""));
        edit_command(&viewer, what, Some(query));
    }
    let after_comp = comp_json();
    rows.push((
        format!("the {w} by {h} composition cropped to the box from ({x0}, {y0}): its new size"),
        format!("{nw} by {nh}"),
        format!("{} by {}", after_comp["width"], after_comp["height"]),
    ));
    rows.push((
        "entries to undo".into(),
        "2".into(),
        (viewer.lock().expect("lock").document.undo_depth() - depth).to_string(),
    ));
    let after: Vec<_> = frames.iter().map(|&f| draw(&viewer, f)).collect();
    for (n, f) in frames.iter().enumerate() {
        let mut worst = 0.0f32;
        for y in 0..nh {
            for x in 0..nw {
                let (a, b) = (before[n].pixel(x0 + x, y0 + y), after[n].pixel(x, y));
                for c in 0..4 {
                    worst = worst.max((a[c] - b[c]).abs());
                }
            }
        }
        rows.push((
            format!("frame {f}: the most any pixel inside the box differs from the cropped composition's, 0 to 1"),
            "0".into(),
            format!("{worst}"),
        ));
    }
    picture(&before[1], &after[1], (x0, y0, nw, nh));
    rows.push((
        "crop.png: the reference shot at frame 12 with the box drawn on it, and the cropped composition beside it at the same scale".into(),
        "drawn".into(),
        "drawn".into(),
    ));
    viewer.lock().expect("lock").document.undo();
    viewer.lock().expect("lock").document.undo();
    let back = comp_json();
    rows.push((
        "two Ctrl+Z: the size and the layers back".into(),
        "true".into(),
        (back == comp).to_string(),
    ));

    let page = page();
    for (what, line) in [
        ("in the Composition menu", "<button data-run=\"Crop comp to region of interest\"></button>"),
        ("in the command finder", "['Crop comp to region of interest', '', () => cropToRegion()],"),
        ("the box is cleared and the steps sent in order", "  setRegion(null);\n  for (const request of steps) await command(request);"),
    ] {
        rows.push((what.into(), "true".into(), page.contains(line).to_string()));
    }
    table(
        "B-952_crop_to_region_table.md",
        "B-952: Crop comp to region of interest (D-1072)",
        "b952_crop_to_region",
        "After Effects' Composition > Crop Comp to Region of Interest (4GeGLgK2BBk, about 4:40). \
         `cropSteps` is the page's own code run under Node; for the window rows its requests go \
         to the window's own command handler on the reference shot, and the frames are drawn \
         before and after. The picture is `verification/B-952 pictures/crop.png`.",
        &rows,
    );
}

const S: usize = 4;

/// Frame 12 before (a quarter of its size, the box outlined in orange) and the crop after it, at the same
/// size, side by side on grey.
fn picture(before: &anime_compositor::WorkingBuffer, after: &anime_compositor::WorkingBuffer, (x0, y0, nw, nh): (usize, usize, usize, usize)) {
    let (bw, bh) = (before.width() / S, before.height() / S);
    let gap = 24;
    let (ow, oh) = (bw + gap + nw / S, bh);
    let mut out = vec![90u8; ow * oh * 4];
    let put = |out: &mut Vec<u8>, x: usize, y: usize, p: [u8; 4]| {
        let o = (y * ow + x) * 4;
        out[o..o + 4].copy_from_slice(&p);
    };
    let srgb = |b: &anime_compositor::WorkingBuffer| b.to_srgb8_straight();
    let (b8, a8) = (srgb(before), srgb(after));
    let px = |v: &[u8], w: usize, x: usize, y: usize| {
        let o = (y * w + x) * 4;
        let a = v[o + 3] as u32;
        // Over the grey, so empty parts read as grey.
        let mix = |c: u8| ((c as u32 * a + 90 * (255 - a)) / 255) as u8;
        [mix(v[o]), mix(v[o + 1]), mix(v[o + 2]), 255]
    };
    for y in 0..bh {
        for x in 0..bw {
            put(&mut out, x, y, px(&b8, before.width(), x * S, y * S));
        }
    }
    for y in 0..nh / S {
        for x in 0..nw / S {
            put(&mut out, bw + gap + x, y, px(&a8, after.width(), x * S, y * S));
        }
    }
    let (l, t, r, b) = (x0 / S, y0 / S, (x0 + nw) / S, (y0 + nh) / S);
    for x in l..r.min(bw) {
        for y in [t, b.min(bh) - 1] {
            put(&mut out, x, y, [255, 160, 40, 255]);
        }
    }
    for y in t..b.min(bh) {
        for x in [l, r.min(bw) - 1] {
            put(&mut out, x, y, [255, 160, 40, 255]);
        }
    }
    let dir = repo("verification/B-952 pictures");
    std::fs::create_dir_all(&dir).expect("the folder");
    anime_compositor::png_out::write_rgba(&dir.join("crop.png"), ow, oh, OutputDepth::Eight, &[], &out)
        .expect("write the picture");
}

/// D-1073 / B-953: Interpret Footage's Loop. A sequence set to loop 3 times gives a new layer its
/// drawings three times over, and New Comp from Selection a composition three times as long.
#[test]
fn b953_interpret_loop() {
    let mut rows: Rows = Vec::new();
    let viewer = Mutex::new(demo());
    let (asset, still, depth) = {
        let held = viewer.lock().expect("lock");
        let project = held.document.project();
        let asset = project.assets.iter().filter(|a| a.kind == AssetKind::ImageSequence).max_by_key(|a| a.frames.len()).expect("a sequence").clone();
        let still = project.assets.iter().find(|a| a.kind == AssetKind::Still).map(|a| a.id.as_str().to_string());
        (asset, still, held.document.undo_depth())
    };
    let n = asset.frames.len();
    let id = asset.id.as_str().to_string();
    let ask = |q: &str| {
        let (what, query) = q.split_once('?').unwrap_or((q, ""));
        edit_command(&viewer, what, Some(query)).unwrap_or_default()
    };
    let loop_of = |which: &str| viewer.lock().expect("lock").document.project().assets.iter().find(|a| a.id.as_str() == which).map_or(0, |a| a.loop_count);
    let loop_now = || loop_of(&id);
    let said = |answer: &str| if answer.is_empty() { "silent" } else { "answered" };
    rows.push((format!("the reference shot's longest sequence ({n} drawings), before: Loop"), "1".into(), loop_now().to_string()));
    ask(&format!("asset.set_loop?asset={id}&times=3"));
    rows.push(("after asset.set_loop times=3: Loop".into(), "3".into(), loop_now().to_string()));
    rows.push(("entries to undo".into(), "1".into(), (viewer.lock().expect("lock").document.undo_depth() - depth).to_string()));
    let saved = {
        let held = viewer.lock().expect("lock");
        persist::to_json(held.document.project(), &Preserved::default())
    };
    let json: serde_json::Value = serde_json::from_str(&saved).expect("the saved project is JSON");
    let list = json["assets"].as_array().expect("assets");
    let entry = list.iter().find(|a| a["id"] == id.as_str()).expect("the asset");
    rows.push(("the saved project: the sequence's \"loop\"".into(), "3".into(), entry["loop"].to_string()));
    let others = list.iter().filter(|a| a["id"] != id.as_str() && a.get("loop").is_some()).count();
    rows.push(("the saved project: other assets with a \"loop\" written".into(), "0".into(), others.to_string()));
    let reopened = persist::load_str(&saved).expect("it opens");
    let back = reopened.document.project().assets.iter().find(|a| a.id == asset.id).map(|a| a.loop_count);
    rows.push(("saved and opened again: Loop".into(), "Some(3)".into(), format!("{back:?}")));
    let bad = saved.replace("\"loop\": 3", "\"loop\": 0");
    rows.push((
        "a file saying \"loop\": 0 is refused".into(),
        "true".into(),
        (bad != saved && persist::load_str(&bad).is_err()).to_string(),
    ));

    // Refusals: each answers and changes nothing.
    for (what, q) in [
        ("0 times", format!("asset.set_loop?asset={id}&times=0")),
        ("100 times", format!("asset.set_loop?asset={id}&times=100")),
        ("not a number", format!("asset.set_loop?asset={id}&times=two")),
        ("an asset not in the project", "asset.set_loop?asset=nothing-here&times=2".to_string()),
    ] {
        let answer = ask(&q);
        rows.push((format!("refused, {what}: Loop stays 3 and the window answers"), "3, answered".into(), format!("{}, {}", loop_now(), said(&answer))));
    }
    if let Some(still) = &still {
        let answer = ask(&format!("asset.set_loop?asset={still}&times=2"));
        rows.push(("refused, a still: its Loop stays 1 and the window answers".into(), "1, answered".into(), format!("{}, {}", loop_of(still), said(&answer))));
    }
    rows.push(("entries to undo after the refusals".into(), "1".into(), (viewer.lock().expect("lock").document.undo_depth() - depth).to_string()));

    // A composition from the sequence, then a layer from it.
    ask(&format!("composition.create?name=Looped&footage={id}&fps=24/1&drag=1"));
    ask(&format!("layer.create?asset={id}&name=Run&drag=1"));
    ask("property.drag_end");
    let (frames, spans, project, comp, root) = {
        let held = viewer.lock().expect("lock");
        let project = held.document.project().clone();
        let comp = project.composition(&held.composition).expect("the new composition");
        let layer = comp.layers_in_order().next().expect("the layer").clone();
        (comp.duration_frames, layer.exposure_spans.clone(), project.clone(), held.composition.clone(), held.root.clone())
    };
    rows.push(("New Comp from Selection: its length in frames, 3 times the drawings".into(), (3 * n).to_string(), frames.to_string()));
    let numbers: Vec<u32> = asset.frames.keys().copied().collect();
    let want: Vec<String> = numbers.repeat(3).iter().enumerate().map(|(f, d)| format!("{f}:{d}")).collect();
    let got: Vec<String> = spans.iter().map(|s| format!("{}:{}", s.start_frame, s.drawing_number)).collect();
    rows.push(("the new layer's sheet: one frame per drawing, the drawings 3 times over".into(), "true".into(), (want == got).to_string()));
    rows.push(("the sheet's length in frames".into(), (3 * n).to_string(), spans.last().map_or(0, |s| s.end_frame_exclusive).to_string()));

    let draw = |frame: i32| {
        anime_compositor::compose::render_frame(&project, &comp, frame, &root, DEFAULT_TILE_SIZE, &mut FrameLog::new(0)).expect("the frame draws")
    };
    let picks = [0, n / 2, n - 1];
    let mut grid = Vec::new();
    for pass in 0..3 {
        for &p in &picks {
            grid.push(draw((pass * n + p) as i32));
        }
    }
    for (c, &p) in picks.iter().enumerate() {
        let mut worst = 0.0f32;
        for pass in 1..3 {
            let (a, b) = (&grid[c], &grid[pass * 3 + c]);
            for y in 0..a.height() {
                for x in 0..a.width() {
                    let (pa, pb) = (a.pixel(x, y), b.pixel(x, y));
                    for k in 0..4 {
                        worst = worst.max((pa[k] - pb[k]).abs());
                    }
                }
            }
        }
        rows.push((format!("frames {p}, {} and {}: the most any pixel differs, 0 to 1", n + p, 2 * n + p), "0".into(), format!("{worst}")));
    }
    let differ = {
        let (a, b) = (&grid[0], &grid[2]);
        (0..a.height()).any(|y| (0..a.width()).any(|x| a.pixel(x, y) != b.pixel(x, y)))
    };
    rows.push(("the first and last drawing differ (so the rows above are a real check)".into(), "true".into(), differ.to_string()));
    loop_picture(&grid);
    rows.push(("loop.png: three rows, one per pass, each the first, middle and last drawing".into(), "drawn".into(), "drawn".into()));

    // Undo: the layer and composition, then the Loop.
    viewer.lock().expect("lock").document.undo();
    viewer.lock().expect("lock").document.undo();
    rows.push(("two Ctrl+Z: Loop back to".into(), "1".into(), loop_now().to_string()));

    let page = page();
    for (what, line) in [
        ("the Project panel's line for a sequence has Loop", "const lab = el('label', 'pdim', 'Loop ');"),
        ("only for a sequence", "if (asset.kind === 'image_sequence') {"),
        ("a change sends asset.set_loop", "times.onchange = () => command('/asset.set_loop?asset=' + encodeURIComponent(asset.id)"),
    ] {
        rows.push((what.into(), "true".into(), page.contains(line).to_string()));
    }
    table(
        "B-953_interpret_loop_table.md",
        "B-953: Interpret Footage Loop (D-1073)",
        "b953_interpret_loop",
        "After Effects' Interpret Footage > Loop (nTpT2PELCbA, about 1:30). The requests go to the \
         window's own command handler on the reference shot, with its longest sequence; the frames \
         are drawn by the renderer. The picture is `verification/B-953 pictures/loop.png`.",
        &rows,
    );
}

/// Nine frames at an eighth of their size: a row per pass through the drawings.
fn loop_picture(grid: &[anime_compositor::WorkingBuffer]) {
    const E: usize = 8;
    let (tw, th) = (grid[0].width() / E, grid[0].height() / E);
    let gap = 8;
    let (ow, oh) = (3 * tw + 4 * gap, 3 * th + 4 * gap);
    let mut out = vec![90u8; ow * oh * 4];
    for (i, b) in grid.iter().enumerate() {
        let v = b.to_srgb8_straight();
        let (ox, oy) = (gap + (i % 3) * (tw + gap), gap + (i / 3) * (th + gap));
        for y in 0..th {
            for x in 0..tw {
                let s = ((y * E) * b.width() + x * E) * 4;
                let a = v[s + 3] as u32;
                let mix = |c: u8| ((c as u32 * a + 90 * (255 - a)) / 255) as u8;
                let o = ((oy + y) * ow + ox + x) * 4;
                out[o..o + 4].copy_from_slice(&[mix(v[s]), mix(v[s + 1]), mix(v[s + 2]), 255]);
            }
        }
    }
    let dir = repo("verification/B-953 pictures");
    std::fs::create_dir_all(&dir).expect("the folder");
    anime_compositor::png_out::write_rgba(&dir.join("loop.png"), ow, oh, OutputDepth::Eight, &[], &out)
        .expect("write the picture");
}

/// D-1074 / B-954: Ctrl + wheel over an expression sizes its text, 8 to 32 pixels.
#[test]
fn b954_expression_text_size() {
    let mut rows: Rows = Vec::new();
    let asked = node(
        "B-954",
        "exprSizeStep",
        "const run = (start, deltas) => deltas.reduce((s, d) => exprSizeStep(s, d), start);\n\
         out({\n\
           first: exprSizeStep(undefined, -100),\n\
           down: exprSizeStep(undefined, 100),\n\
           five: run(12, [-1, -1, -1, -1, -1]),\n\
           top: run(12, Array(40).fill(-120)),\n\
           bottom: run(12, Array(40).fill(120)),\n\
           none: exprSizeStep(15, 0),\n\
           junk: exprSizeStep('big', 0),\n\
         });",
    );
    for (what, field, want) in [
        ("no size kept yet, one notch up (Ctrl + wheel away from you)", "first", "13"),
        ("no size kept yet, one notch down", "down", "11"),
        ("five notches up from 12", "five", "17"),
        ("forty notches up: no bigger than", "top", "32"),
        ("forty notches down: no smaller than", "bottom", "8"),
        ("no turn: the kept size", "none", "15"),
        ("a kept size that is not a number: the 12 every box starts at", "junk", "12"),
    ] {
        rows.push((what.into(), want.into(), asked[field].to_string()));
    }
    let page = page();
    for (what, line) in [
        ("the expression box listens for Ctrl + wheel and only for it", "  text.onwheel = (e) => {\n    if (!e.ctrlKey) return;\n    e.preventDefault();"),
        ("the size is kept with the window's preferences", "    prefs.exprSize = exprSizeStep(prefs.exprSize, e.deltaY);\n    savePrefs();"),
        ("every expression box takes the size", "font: var(--exprsize, 12px)/1.35 Consolas"),
        ("the kept size is used when the window opens", "showExprSize();\n"),
    ] {
        rows.push((what.into(), "true".into(), page.contains(line).to_string()));
    }
    table(
        "B-954_expression_text_size_table.md",
        "B-954: Ctrl + wheel sizes the expression text (D-1074)",
        "b954_expression_text_size",
        "After Effects' Ctrl + wheel over an expression (4GeGLgK2BBk, about 3:40). `exprSizeStep` is \
         the page's own code run under Node; the other rows read the page.",
        &rows,
    );
}

/// D-1075 / B-955: Shift on letting go of the parent whip parents and jumps: the child's Position
/// becomes the parent's Anchor Point, so the child's anchor lands on the parent's.
#[test]
fn b955_parent_and_jump() {
    let mut rows: Rows = Vec::new();
    let asked = node(
        "B-955",
        "jumpRequests",
        "const parent = { id: 'p', transform: { anchor: { base: [40, 25] } } };\n\
         out({\n\
           plain: jumpRequests({ id: 'c', transform: { position: { base: [900, 500] } } }, parent).join(' then '),\n\
           keyed: jumpRequests({ id: 'c', transform: { position: { base: [0, 0], keyframes: [{ frame: 0, value: [1, 2] }] } } }, parent).length,\n\
           split: jumpRequests({ id: 'c', transform: { position: { x: { base: 1 }, y: { base: 2 } } } }, parent).length,\n\
           bare: jumpRequests({ id: 'c' }, { id: 'p' }).join(' then '),\n\
         });",
    );
    let get = |k: &str| asked[k].as_str().map(str::to_string).unwrap_or_else(|| asked[k].to_string());
    for (what, field, want) in [
        ("a child at (900, 500), a parent whose anchor is (40, 25): the child's Position becomes (40, 25)", "plain", "/property.set_base?layer=c&prop=position&value=40%2C25"),
        ("a keyed Position is not moved", "keyed", "0"),
        ("a separated Position is not moved", "split", "0"),
        ("no transform written: the anchor is (0, 0)", "bare", "/property.set_base?layer=c&prop=position&value=0%2C0"),
    ] {
        rows.push((what.into(), want.into(), get(field)));
    }

    // The window: the page's steps on the reference shot, then drawn.
    let viewer = Mutex::new(demo());
    let (child, parent, depth) = {
        let held = viewer.lock().expect("lock");
        let comp = held.document.project().composition(&held.composition).expect("the comp");
        let order: Vec<&Layer> = comp.layers_in_order().filter(|l| l.parent.is_none()).collect();
        (order[0].id.as_str().to_string(), order[1].id.as_str().to_string(), held.document.undo_depth())
    };
    // Away from the corner, so a jump that did nothing would show.
    for (id, prop, value) in [(&parent, "position", "700,300"), (&parent, "anchor", "40,25"), (&child, "position", "900,500")] {
        edit_command(&viewer, "property.set_base", Some(&format!("layer={id}&prop={prop}&value={value}")));
    }
    let depth = viewer.lock().expect("lock").document.undo_depth().max(depth);
    let parent_json = {
        let s: serde_json::Value = serde_json::from_str(&state(&viewer)).expect("JSON");
        let id = s["composition"].clone();
        let comp = s["project"]["compositions"].as_array().unwrap().iter().find(|c| c["id"] == id).unwrap().clone();
        comp["layers"].as_array().unwrap().iter().find(|l| l["id"] == parent.as_str()).unwrap().clone()
    };
    let steps = node(
        "B-955",
        "jumpRequests",
        &format!("out(jumpRequests({{ id: '{child}', transform: {{ position: {{ base: [0, 0] }} }} }}, {parent_json}));"),
    );
    edit_command(&viewer, "layer.set_parent", Some(&format!("layer={child}&parent={parent}&frame=0&drag=1")));
    for request in steps.as_array().expect("steps") {
        let request = request.as_str().unwrap().trim_start_matches('/');
        let (what, query) = request.split_once('?').unwrap();
        edit_command(&viewer, what, Some(&format!("{query}&drag=1")));
    }
    edit_command(&viewer, "property.drag_end", None);
    let (child_anchor, parent_anchor, entries) = {
        let held = viewer.lock().expect("lock");
        let comp = held.document.project().composition(&held.composition).expect("the comp");
        let at = |id: &str| {
            let l = comp.layers_in_order().find(|l| l.id.as_str() == id).expect("the layer");
            let world = anime_compositor::compose::world_transform(comp, &l.id, 0);
            let a = match l.transform.anchor.value_at(0) {
                Value::Vec2(x, y) => (x, y),
                Value::Scalar(_) => (0.0, 0.0),
            };
            world.apply(a.0, a.1)
        };
        (at(&child), at(&parent), held.document.undo_depth() - depth)
    };
    let near = (child_anchor.0 - parent_anchor.0).abs() < 1e-6 && (child_anchor.1 - parent_anchor.1).abs() < 1e-6;
    rows.push((
        format!("the reference shot: {child} parented and jumped to {parent}: its anchor on the frame is the parent's ({:.2}, {:.2})", parent_anchor.0, parent_anchor.1),
        "true".into(),
        format!("{near}"),
    ));
    rows.push(("entries to undo".into(), "1".into(), entries.to_string()));

    let page = page();
    for (what, line) in [
        ("Shift on letting go picks parent and jump; the label says so", "if (shift) return ['Parent and jump to: ' + parent.name, () => jumpTo(layer, parent)];"),
        ("Shift is read when letting go, too", "const up = (u) => { if (shift !== u.shiftKey) move(u); stop(true); };"),
        ("every chosen layer, as one entry to undo", "for (const request of sends) await command(request + '&drag=1');"),
    ] {
        rows.push((what.into(), "true".into(), page.contains(line).to_string()));
    }
    table(
        "B-955_parent_and_jump_table.md",
        "B-955: Shift + parent whip, parent and jump (D-1075)",
        "b955_parent_and_jump",
        "After Effects' Shift-drag of the parent pick whip (4GeGLgK2BBk, about 3:20). `jumpRequests` \
         is the page's own code run under Node; for the window rows its requests go to the window's \
         own command handler on the reference shot.",
        &rows,
    );
}

/// D-1076 / B-956: Space held while dragging out a shape or mask moves it rather than sizing it.
#[test]
fn b956_space_moves_while_drawing() {
    let mut rows: Rows = Vec::new();
    let asked = node(
        "B-956",
        "spaceMove",
        "// A drag from (10, 10): to (50, 40) sizing, to (70, 60) with Space held, to (80, 80) after.\n\
         const steps = [[[50, 40], false], [[70, 60], true], [[80, 80], false]];\n\
         let start = [10, 10], last = null; const seen = [];\n\
         for (const [now, held] of steps) { start = spaceMove(start, last, now, held); last = now; seen.push(start.join(', ') + ' to ' + now.join(', ')); }\n\
         out({ seen, first: spaceMove([5, 5], null, [9, 9], true).join(', ') });",
    );
    let seen: Vec<String> = asked["seen"].as_array().unwrap().iter().map(|s| s.as_str().unwrap().to_string()).collect();
    for (what, want, got) in [
        ("dragged from (10, 10) to (50, 40): the shape is sized, its corner stays", "10, 10 to 50, 40", &seen[0]),
        ("Space held, on to (70, 60): the whole shape moves by (20, 20), its size kept", "30, 30 to 70, 60", &seen[1]),
        ("Space let go, on to (80, 80): sized again from where it was moved to", "30, 30 to 80, 80", &seen[2]),
    ] {
        rows.push((what.into(), want.into(), got.clone()));
    }
    rows.push(("Space already held at the first step: nothing to move by yet".into(), "5, 5".into(), asked["first"].as_str().unwrap().to_string()));
    let page = page();
    for (what, line) in [
        ("Space during the drag is the drag's, so it does not play", "if (k.type === 'keydown') { k.preventDefault(); k.stopPropagation(); }"),
        ("every step of the drag goes through it", "from = spaceMove(from, last, [mx, my], space);"),
        ("the shape is drawn from where it was moved to", "settings = shapeSettings(kind, ox, oy, mx, my);"),
        ("the keys are let go of when the drag ends", "window.removeEventListener('keyup', spaceKey, true);"),
    ] {
        rows.push((what.into(), "true".into(), page.contains(line).to_string()));
    }
    table(
        "B-956_space_moves_while_drawing_table.md",
        "B-956: Space moves a shape or mask while drawing it (D-1076)",
        "b956_space_moves_while_drawing",
        "After Effects' Space held while dragging out a shape (4GeGLgK2BBk, about 3:00). `spaceMove` is \
         the page's own code run under Node, in layer pixels; the other rows read the page.",
        &rows,
    );
}

/// D-1077 / B-957: Output templates, the export choices kept under a name and chosen again.
#[test]
fn b957_output_templates() {
    let mut rows: Rows = Vec::new();
    let asked = node(
        "B-957",
        "TEMPLATE_KEYS, keepTemplate",
        "const test = { exportformat: 'exr-float', filmquality: 'standard', gifdither: false, soundwav: true, cardexport: false };\n\
         const one = keepTemplate(undefined, 'test', test);\n\
         const two = keepTemplate(one, 'film', { ...test, exportformat: 'mp4', filmquality: 'high' });\n\
         const again = keepTemplate(two, 'test', { ...test, exportformat: 'png' });\n\
         out({\n\
           one: one.map((t) => t.name + ' ' + t.exportformat + (t.soundwav ? ' with WAV' : '')).join('; '),\n\
           two: two.map((t) => t.name + ' ' + t.exportformat).join('; '),\n\
           again: again.map((t) => t.name + ' ' + t.exportformat).join('; '),\n\
           keys: TEMPLATE_KEYS.map((k) => k[0]).join(', '),\n\
         });",
    );
    for (what, field, want) in [
        ("Make template \"test\" on EXR full float with the WAV ticked", "one", "test exr-float with WAV"),
        ("then \"film\" on MP4 High: both kept", "two", "test exr-float; film mp4"),
        ("\"test\" made again on PNG: replaced, not doubled", "again", "film mp4; test png"),
        ("what a template keeps (never the missing-drawing tick)", "keys", "exportformat, filmquality, gifdither, soundwav, cardexport"),
    ] {
        rows.push((what.into(), want.into(), asked[field].as_str().unwrap_or_default().to_string()));
    }
    let page = page();
    for (what, line) in [
        ("the Template list and Make template stand with the export choices", "<span><select id=\"outputtemplate\""),
        ("kept with the window's preferences", "prefs.outputTemplates = keepTemplate(outputTemplates(), name, Object.fromEntries(TEMPLATE_KEYS.map(([id, key]) => [id, $(id)[key]])));"),
        ("choosing one sets each choice as if picked by hand", "    $(id)[key] = t[id];\n    $(id).dispatchEvent(new Event('change'));"),
        ("Export and Add to queue read the same choices", "$('addtoqueue').onclick = () => askQueue('do=add&' + exportChoices());"),
    ] {
        rows.push((what.into(), "true".into(), page.contains(line).to_string()));
    }
    table(
        "B-957_output_templates_table.md",
        "B-957: Output templates (D-1077)",
        "b957_output_templates",
        "After Effects' Output Module > Make Template (nTpT2PELCbA, about 0:40). `keepTemplate` is the \
         page's own code run under Node; the other rows read the page.",
        &rows,
    );
}

/// D-1078 / B-958: Noise's Clip Result Values, chosen in Effect Controls and sent with the others.
#[test]
fn b958_noise_clip_control() {
    let mut rows: Rows = Vec::new();
    let clip = |q: &str| match effect_parameters("core.noise", Some(q)) {
        Ok(e @ Effect::Noise { .. }) => {
            let valid = e.is_valid();
            match e {
                Effect::Noise { clip, .. } => format!("{clip}, {}", if valid { "drawn" } else { "refused" }),
                _ => unreachable!(),
            }
        }
        other => format!("{other:?}"),
    };
    let base = "amount=40&mode=mono&seed=0&animate=off";
    for (what, q, want) in [
        ("a window from before D-1078 sends no clip: Clip on", base.to_string(), "on, drawn"),
        ("Clip Result Values On", format!("{base}&clip=on"), "on, drawn"),
        ("Clip Result Values Off", format!("{base}&clip=off"), "off, drawn"),
        ("a word that is not a choice", format!("{base}&clip=wrap"), "wrap, refused"),
    ] {
        rows.push((what.into(), want.into(), clip(&q)));
    }
    let made = match new_effect("core.noise") {
        Some(Effect::Noise { clip, .. }) => clip,
        other => format!("{other:?}"),
    };
    rows.push(("a Noise just added: Clip Result Values".into(), "on".into(), made));
    let page = page();
    rows.push((
        "Effect Controls shows Clip Result Values, On first (a file without it reads On)".into(),
        "true".into(),
        page.contains("['clip', 'Clip Result Values', [['on', 'On'], ['off', 'Off']]]]").to_string(),
    ));
    table(
        "B-958_noise_clip_control_table.md",
        "B-958: Noise, Clip Result Values in Effect Controls (D-1078)",
        "b958_noise_clip_control",
        "After Effects' Noise > Clip Result Values (lHQvi94dpvM about 4:20 on, f80 about 4:00 off). \
         The window's own reading of what Effect Controls sends; the drawing itself is \
         `verification/B-958_noise_clip_table.md`.",
        &rows,
    );
}
