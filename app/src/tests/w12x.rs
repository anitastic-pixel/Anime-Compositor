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
