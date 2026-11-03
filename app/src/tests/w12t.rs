//! Wave 12 lane T: the page's quality-of-life items from the playlist follow-alongs (D-988 on).
//! Each test runs the page's own marked code under Node, sends what it asks for to the window's
//! own command handler, pins the lines that wire it in, and writes its table under `verification/`.

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
    let path = std::env::temp_dir().join(format!("w12t_{}.js", marker.replace([' ', '/'], "_")));
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

fn pins(rows: &mut Rows, lines: &[(&str, &str)]) {
    let page = page();
    for (what, line) in lines {
        rows.push((what.to_string(), "true".into(), page.contains(line).to_string()));
    }
}

fn table(file: &str, title: &str, test: &str, intro: &str, rows: &Rows) {
    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut text = format!(
        "# {title}\n\nWritten by `{test}` in `app/src/tests/w12t.rs`.\n\n{intro}\n\n\
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

/// Sends a page request to the window's own command handler.
fn send(viewer: &Mutex<Viewer>, request: &str) {
    let request = request.trim_start_matches('/');
    let (what, query) = request.split_once('?').unwrap_or((request, ""));
    edit_command(viewer, what, Some(query));
}

/// D-988 / B-868: Layer > Flip the layer horizontally or vertically turns the Scale's width or
/// height over, as one entry to undo.
#[test]
fn b868_flip_layer() {
    use anime_compositor::model::Value;
    let mut rows: Rows = Vec::new();
    let viewer = Mutex::new(demo());
    let id = {
        let held = viewer.lock().expect("lock");
        let comp = held.document.project().composition(&held.composition).expect("comp");
        let out = comp.layers_in_order().find(|l| l.kind == LayerKind::Raster).expect("a drawn layer").id.clone();
        out
    };
    let scale = || -> String {
        let held = viewer.lock().expect("lock");
        let comp = held.document.project().composition(&held.composition).expect("comp");
        let layer = comp.layers_in_order().find(|l| l.id == id).expect("the layer");
        let out = match layer.transform.scale.value_at(0) {
            // The file keeps a fraction; the page and the table read percent.
            Value::Vec2(x, y) => format!("{}, {}", x * 100.0, y * 100.0),
            Value::Scalar(v) => format!("{}", v * 100.0),
        };
        out
    };
    let depth = || viewer.lock().expect("lock").document.undo_depth();
    let start = scale();
    let asked = node(
        "B-868",
        "flipSends",
        &format!(
            "globalThis.transformValue = (l) => l.s;\n\
             globalThis.number = (n) => String(Math.round(n * 1e6) / 1e6);\n\
             const one = {{ id: '{id}', s: [{start}] }};\n\
             const two = [{{ id: 'a', s: [50, -80] }}, {{ id: 'b b', s: [-100, 100, 100] }}];\n\
             out({{\n\
               h: flipSends([one], 0, 0)[0],\n\
               v: flipSends([one], 1, 0)[0],\n\
               two: flipSends(two, 1, 12).join(' then '),\n\
               none: flipSends([], 0, 0).length,\n\
             }});",
            id = id.as_str()
        ),
    );
    let get = |k: &str| asked[k].as_str().unwrap_or("?").to_string();
    rows.push((
        "two layers flipped vertically at frame 12: each Scale's height turned over, depth kept".into(),
        "/property.set_base?layer=a&frame=12&drag=1&prop=scale&value=50%2C80 then \
         /property.set_base?layer=b%20b&frame=12&drag=1&prop=scale&value=-100%2C-100%2C100"
            .into(),
        get("two"),
    ));
    rows.push(("nothing selected: nothing sent".into(), "0".into(), asked["none"].to_string()));
    rows.push(("the drawn layer's Scale to begin with".into(), "100, 100".into(), start.clone()));
    let d0 = depth();
    send(&viewer, &get("h"));
    send(&viewer, "/property.drag_end");
    rows.push((
        "Flip horizontally: Scale and the entries to undo".into(),
        "-100, 100; 1".into(),
        format!("{}; {}", scale(), depth() - d0),
    ));
    viewer.lock().expect("lock").document.undo();
    rows.push(("one Ctrl+Z: as it started".into(), start.clone(), scale()));
    send(&viewer, &get("v"));
    send(&viewer, "/property.drag_end");
    rows.push(("Flip vertically".into(), "100, -100".into(), scale()));
    viewer.lock().expect("lock").document.undo();
    pins(
        &mut rows,
        &[
            ("in the Layer menu: horizontally", "<button data-run=\"Flip the layer horizontally\"></button>"),
            ("in the Layer menu: vertically", "<button data-run=\"Flip the layer vertically\"></button>"),
            ("in the command finder: horizontally", "['Flip the layer horizontally', '', () => flipLayers(0)],"),
            ("in the command finder: vertically", "['Flip the layer vertically', '', () => flipLayers(1)],"),
            ("the selected unlocked layers, at the playhead", "const sends = flipSends(movable(), axis, frame);"),
        ],
    );
    table(
        "B-868_flip_layer_table.md",
        "B-868: Flip the layer horizontally or vertically (D-988)",
        "b868_flip_layer",
        "After Effects' Layer > Transform > Flip Horizontal and Flip Vertical (lO54j6So7dA 2:23). \
         `flipSends` is the page's own code run under Node; its requests go to the window's own \
         command handler on the reference shot's first drawn layer. The private-app check is \
         `verification/B-868_private_app_table.md`.",
        &rows,
    );
}

/// D-989 / B-869: a path copied and pasted into Position: a key on each point over two seconds,
/// back to the first on a closed path, the middle keys roving, the path's handles on each key,
/// as one entry to undo.
#[test]
fn b869_paste_path_into_position() {
    use anime_compositor::model::{Keyframe, Value};
    let mut rows: Rows = Vec::new();
    let viewer = Mutex::new(demo());
    let id = {
        let held = viewer.lock().expect("lock");
        let comp = held.document.project().composition(&held.composition).expect("comp");
        let out = comp
            .layers_in_order()
            .find(|l| l.kind == LayerKind::Raster && l.transform.position.keyframes().is_empty())
            .expect("a drawn layer with no Position keys")
            .id
            .clone();
        out
    };
    let asked = node(
        "B-869",
        "motionSends",
        &format!(
            "globalThis.number = (n) => String(Math.round(n * 1e6) / 1e6);\n\
             const square = {{ closed: true, points: [[100, 100, 0, 0, 0, 0], [500, 100, 0, -40, 0, 40], \
             [500, 300, 0, 0, 0, 0], [100, 300, 0, 0, 0, 0]] }};\n\
             const open = {{ closed: false, points: [[10, 20, 0, 0, 0, 0], [30, 40, 0, 0, 0, 0]] }};\n\
             out({{\n\
               square: motionSends(square, '{id}', 0, 24, true),\n\
               open: motionSends(open, 'a', 10, 24, false, [5]).join(' then '),\n\
               one: motionSends({{ closed: true, points: [[1, 2, 0, 0, 0, 0]] }}, 'a', 0, 24, true).length,\n\
             }});",
            id = id.as_str()
        ),
    );
    rows.push((
        "an open path of two points at frame 10, with a depth of 5: two keys two seconds apart, no roving".into(),
        "/property.set_base?layer=a&frame=10&drag=1&prop=position&value=10%2C20%2C5 then \
         /property.set_base?layer=a&frame=58&drag=1&prop=position&value=30%2C40%2C5"
            .into(),
        asked["open"].as_str().unwrap_or("?").into(),
    ));
    rows.push(("a path of one point: nothing sent".into(), "0".into(), asked["one"].to_string()));
    let depth = || viewer.lock().expect("lock").document.undo_depth();
    let d0 = depth();
    for request in asked["square"].as_array().expect("requests") {
        send(&viewer, request.as_str().unwrap_or("?"));
    }
    send(&viewer, "/property.drag_end");
    let keys: Vec<Keyframe> = {
        let held = viewer.lock().expect("lock");
        let comp = held.document.project().composition(&held.composition).expect("comp");
        let layer = comp.layers_in_order().find(|l| l.id == id).expect("the layer");
        let out = layer.transform.position.keyframes().to_vec();
        out
    };
    let shown = |k: &Keyframe| match k.value {
        Value::Vec2(x, y) => format!("{x},{y}"),
        Value::Scalar(v) => format!("{v}"),
    };
    rows.push((
        "a closed square pasted at frame 0: a key on each corner and back to the first; entries to undo".into(),
        "100,100 500,100 500,300 100,300 100,100; 1".into(),
        format!("{}; {}", keys.iter().map(shown).collect::<Vec<_>>().join(" "), depth() - d0),
    ));
    rows.push((
        "the first and last keys two seconds apart (24 frames a second)".into(),
        "0, 48".into(),
        format!("{}, {}", keys.first().map_or(-1, |k| k.frame), keys.last().map_or(-1, |k| k.frame)),
    ));
    rows.push((
        "which keys rove".into(),
        "false, true, true, true, false".into(),
        keys.iter().map(|k| k.roving.to_string()).collect::<Vec<_>>().join(", "),
    ));
    rows.push((
        "the second corner keeps the path's handles".into(),
        "Some([0.0, -40.0, 0.0, 40.0])".into(),
        format!("{:?}", keys.get(1).and_then(|k| k.spatial)),
    ));
    viewer.lock().expect("lock").document.undo();
    let after: usize = {
        let held = viewer.lock().expect("lock");
        let comp = held.document.project().composition(&held.composition).expect("comp");
        let out = comp.layers_in_order().find(|l| l.id == id).expect("the layer").transform.position.keyframes().len();
        out
    };
    rows.push(("one Ctrl+Z: no Position keys again".into(), "0".into(), after.to_string()));
    pins(
        &mut rows,
        &[
            ("pressing a property's name remembers it", "    namedProp = { layer: layer.id, prop };\n"),
            (
                "pressing a mask's or shape's name names its path",
                "namedProp = { layer: layer.id, prop: (shapes ? 'shape:' : 'mask:') + at };",
            ),
            ("Ctrl+C on a named path with no keys copies the path", "namedPathCopy() && (e.key === 'c' || e.key === 'C')"),
            (
                "Ctrl+V with Position's name pressed pastes the path as motion",
                "if (copiedPath && namedProp && namedProp.prop === 'position' && selection.includes(namedProp.layer)) return pasteMotion(start);",
            ),
        ],
    );
    table(
        "B-869_paste_path_table.md",
        "B-869: a path pasted into Position (D-989)",
        "b869_paste_path_into_position",
        "After Effects' paste of a Path into Position (tQ5wqg10x_I 1:12). `motionSends` is the \
         page's own code run under Node; the square's requests go to the window's own command \
         handler on the reference shot's first drawn layer without Position keys. The private-app \
         check is `verification/B-869_private_app_table.md`.",
        &rows,
    );
}
