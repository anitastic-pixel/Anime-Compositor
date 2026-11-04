//! Wave 12 lane V: the page's items from the tutorial follow-alongs (D-1027 on). Each test runs
//! the page's own marked code under Node, pins the lines that wire it in, and writes its table
//! under `verification/`.

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
    let path = std::env::temp_dir().join(format!("w12v_{}.js", marker.replace([' ', '/'], "_")));
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
        rows.push((format!("The page holds: {what}"), "true".into(), page.contains(line).to_string()));
    }
}

fn table(file: &str, title: &str, test: &str, intro: &str, rows: &Rows) {
    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut text = format!(
        "# {title}\n\nWritten by `{test}` in `app/src/tests/w12v.rs`.\n\n{intro}\n\n\
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

/// D-1027 / B-906: Shift keeps a dragged rectangle or ellipse square or round, Ctrl draws it out
/// from where the press began.
#[test]
fn b906_drawing_keys() {
    let mut rows: Rows = Vec::new();
    let said = node(
        "B-906",
        "dragCorners",
        "const k = (s, c) => ({ shiftKey: s, ctrlKey: c });\n\
         out([\n\
           dragCorners(100, 100, 160, 130, k(false, false)),\n\
           dragCorners(100, 100, 160, 130, k(true, false)),\n\
           dragCorners(100, 100, 70, 180, k(true, false)),\n\
           dragCorners(100, 100, 160, 130, k(false, true)),\n\
           dragCorners(100, 100, 160, 130, k(true, true)),\n\
           dragCorners(100, 100, 40, 90, k(true, true)),\n\
         ]);",
    );
    let cases = [
        ("No key: corner to corner, 60 by 30", "[100,100,160,130]"),
        ("Shift: a square, the longer side (60) winning", "[100,100,160,160]"),
        ("Shift dragged up-left and down: still a square, 80 a side, the way the pointer went", "[100,100,20,180]"),
        ("Ctrl: out from the press, 120 by 60 round (100, 100)", "[40,70,160,130]"),
        ("Ctrl and Shift: a square 120 a side round (100, 100)", "[40,40,160,160]"),
        ("Ctrl and Shift dragged up-left: the same square size, 120 a side round (100, 100)", "[160,160,40,40]"),
    ];
    for (i, (what, want)) in cases.iter().enumerate() {
        rows.push((what.to_string(), want.to_string(), said[i].to_string()));
    }
    pins(
        &mut rows,
        &[
            (
                "the rectangle, rounded rectangle and ellipse drag read the keys",
                "const box = kind === 'polygon' || kind === 'star' ? [ox, oy, mx, my] : dragCorners(ox, oy, mx, my, m);",
            ),
            ("the shape's settings come from those corners", "settings = shapeSettings(kind, ...box);"),
            ("the mask's outline comes from those corners", "shapePoints(kind, ...box)"),
        ],
    );
    table(
        "B-906_drawing_keys_table.md",
        "B-906: Shift and Ctrl while drawing a rectangle or an ellipse (D-1027)",
        "b906_drawing_keys",
        "After Effects: Shift while dragging a Rectangle or Ellipse tool keeps it a square or a circle; \
         Ctrl draws it out from its centre. The rows below run the page's own drag arithmetic; the \
         numbers are the two corners drawn between, x0, y0, x1, y1, in the layer's pixels.",
        &rows,
    );
}

/// D-1028 / B-907: Ctrl+D with a mask chosen copies the mask, keys and mode included, just below
/// it, named as a new mask is.
#[test]
fn b907_mask_duplicate() {
    let mut rows: Rows = Vec::new();
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(open(&source).unwrap_or_else(|d| panic!("open: {}", d.message)));
    let cel_of = || -> serde_json::Value {
        let answer: serde_json::Value = serde_json::from_str(&state(&viewer)).expect("JSON");
        answer["project"]["compositions"][0]["layers"]
            .as_array()
            .expect("layers")
            .iter()
            .find(|l| l["name"] == "Cel")
            .cloned()
            .expect("the Cel layer")
    };
    let cel = cel_of()["id"].as_str().unwrap_or_default().to_string();
    let run = |what: &str| -> String {
        let (id, query) = what.split_once('?').expect("a query");
        edit_command(&viewer, id, Some(query)).expect("a command the window answers")
    };
    let masks = || -> String {
        cel_of()["masks"]
            .as_array()
            .cloned()
            .unwrap_or_default()
            .iter()
            .map(|m| {
                format!(
                    "{} {} {} points {} keys, feather {}{}",
                    m["name"].as_str().unwrap_or("?"),
                    m["mode"].as_str().unwrap_or("?"),
                    m["path"]["base"]["points"].as_array().map_or(0, |p| p.len()),
                    m["path"]["keyframes"].as_array().map_or(0, |k| k.len()),
                    m["feather_px"],
                    if m["inverted"] == serde_json::Value::Bool(true) { ", inverted" } else { "" },
                )
            })
            .collect::<Vec<_>>()
            .join("; ")
    };
    let square = "0,0,0,0,0,0;100,0,0,0,0,0;100,100,0,0,0,0;0,100,0,0,0,0";
    run(&format!("mask.add?layer={cel}&points={square}"));
    run(&format!("mask.add?layer={cel}&points={square}"));
    run(&format!("mask.set?layer={cel}&mask=0&mode=subtract&feather=8&inverted=1"));
    run(&format!("mask.add_remove_key?layer={cel}&mask=0&frame=0"));
    run(&format!(
        "mask.set_path?layer={cel}&mask=0&frame=10&points=10,0,0,0,0,0;100,0,0,0,0,0;100,100,0,0,0,0;0,100,0,0,0,0"
    ));
    let before = masks();
    rows.push((
        "Two masks to start: Mask 1 subtract, feathered, inverted, its path keyed at 0 and 10".into(),
        "Mask 1 subtract 4 points 2 keys, feather 8, inverted; Mask 2 add 4 points 0 keys, feather 0".into(),
        before.clone(),
    ));
    run(&format!("mask.duplicate?layer={cel}&mask=0"));
    rows.push((
        "Ctrl+D on Mask 1: the copy sits just below it, named Mask 3 (the next free), mode, feather, inverted and keys kept"
            .into(),
        "Mask 1 subtract 4 points 2 keys, feather 8, inverted; Mask 3 subtract 4 points 2 keys, feather 8, inverted; \
         Mask 2 add 4 points 0 keys, feather 0"
            .into(),
        masks(),
    ));
    rows.push(("The copy's path is the original's, key for key".into(), "true".into(), {
        let m = cel_of()["masks"].clone();
        (m[0]["path"] == m[1]["path"]).to_string()
    }));
    rows.push(("One entry to undo, named for the new mask".into(), "Draw Mask 3".into(), held_labels(&viewer)));
    let _ = edit_command(&viewer, "edit.undo", None);
    rows.push(("Undo takes the copy back".into(), before.clone(), masks()));
    rows.push((
        "A mask that is not there is refused, with how many there are".into(),
        "Which mask? \"Cel\" has 2.".into(),
        run(&format!("mask.duplicate?layer={cel}&mask=7")),
    ));
    rows.push(("and nothing changed".into(), before, masks()));
    pins(
        &mut rows,
        &[
            ("Ctrl+D (and the finder line that presses it) copies the chosen mask first", "  if (duplicateMask()) return;"),
            (
                "only with a mask chosen, one layer chosen, and a layer of masks rather than shapes",
                "if (!maskChosen || selection.length !== 1 || !layer || drawsShapes(layer) || !masksOf(layer)[maskPicked]) return false;",
            ),
            ("the copy becomes the mask chosen", "    maskPicked = at + 1;"),
            ("a mask's name in the timeline chooses it (a shape's does not)", "maskPicked = at; maskChosen = !shapes;"),
            ("a double-click on its outline chooses it", "  maskChosen = !drawsShapes(layer); // D-1028"),
            ("choosing a layer lets the mask go", "function select(id, add) {\n  maskChosen = false;"),
            ("and so does choosing several", "function choose(ids) {\n  maskChosen = false;"),
        ],
    );
    table(
        "B-907_mask_duplicate_table.md",
        "B-907: Ctrl+D with a mask chosen copies the mask (D-1028)",
        "b907_mask_duplicate",
        "After Effects: with a mask selected, Edit > Duplicate (Ctrl+D) copies it, everything \
         included, just below it. The first rows run the window's own command on \
         `Fixtures/projects/cel_holds_project.json`; the last rows pin how the page sends it.",
        &rows,
    );
}

fn held_labels(viewer: &Mutex<Viewer>) -> String {
    viewer
        .lock()
        .expect("the viewer lock")
        .document
        .undo_labels()
        .last()
        .cloned()
        .unwrap_or_default()
        .to_string()
}

/// D-1030 / B-909: Ctrl+Shift+X cuts the composition down to its work area.
#[test]
fn b909_trim_to_work_area() {
    let mut rows: Rows = Vec::new();
    let source = repo("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(open(&source).unwrap_or_else(|d| panic!("open: {}", d.message)));
    let comp = || -> String {
        let answer: serde_json::Value = serde_json::from_str(&state(&viewer)).expect("JSON");
        let c = &answer["project"]["compositions"][0];
        let cel = &c["layers"][0];
        format!(
            "first frame {}, {} frames, work area {}, markers [{}], Cel {} to {}",
            c["start_frame"],
            c["duration_frames"],
            if c["work_area"].is_null() {
                "none".to_string()
            } else {
                format!("{} to {}", c["work_area"]["start_frame"], c["work_area"]["end_frame_exclusive"])
            },
            c["markers"]
                .as_array()
                .cloned()
                .unwrap_or_default()
                .iter()
                .map(|m| m["frame"].to_string())
                .collect::<Vec<_>>()
                .join(", "),
            cel["in_frame"],
            cel["out_frame"],
        )
    };
    let run = |id: &str, query: Option<&str>| -> String {
        edit_command(&viewer, id, query).expect("a command the window answers")
    };
    let trim = || run("composition.trim_to_work_area", None);
    let whole = comp();
    rows.push((
        "The project as saved: five frames, the work area the whole of them".into(),
        "first frame 0, 5 frames, work area 0 to 5, markers [], Cel 0 to 5".into(),
        whole.clone(),
    ));
    rows.push((
        "Trimming with the work area the whole composition is refused, with what to do".into(),
        "The work area is already the whole composition: set it with B and N first.".into(),
        trim(),
    ));
    rows.push(("and nothing changed".into(), whole.clone(), comp()));
    run("timeline.set_markers", Some("marker=0|a&marker=2|b&marker=4|c"));
    run("timeline.set_work_start", Some("frame=1"));
    run("timeline.set_work_end", Some("frame=3"));
    let before = comp();
    rows.push((
        "B at frame 1, N at frame 3, markers on 0, 2 and 4".into(),
        "first frame 0, 5 frames, work area 1 to 4, markers [0, 2, 4], Cel 0 to 5".into(),
        before.clone(),
    ));
    trim();
    rows.push((
        "Ctrl+Shift+X: the composition is frames 1 to 3, the work area all of it, the marker inside kept, the layer untouched"
            .into(),
        "first frame 1, 3 frames, work area 1 to 4, markers [2], Cel 0 to 5".into(),
        comp(),
    ));
    rows.push((
        "One entry to undo".into(),
        "Trim the composition to the work area".into(),
        held_labels(&viewer),
    ));
    rows.push((
        "A second Ctrl+Shift+X has no work area to trim to and is refused".into(),
        "The work area is already the whole composition: set it with B and N first.".into(),
        trim(),
    ));
    let _ = edit_command(&viewer, "edit.undo", None);
    rows.push(("Undo puts the composition back as it was".into(), before, comp()));
    pins(
        &mut rows,
        &[
            ("Ctrl+Shift+X sends it", "e.code === 'KeyX') { e.preventDefault(); trimToWork(); }"),
            ("the command it sends", "await command('/composition.trim_to_work_area');"),
            ("the Composition menu has it", "<button data-run=\"Trim comp to work area\"></button>"),
            (
                "the command finder has it, under After Effects' key",
                "['Trim comp to work area', 'Ctrl+Shift+X', { key: 'X', code: 'KeyX', ctrlKey: true, shiftKey: true }],",
            ),
            (
                "the timeline's right-click menu has it, greyed with no work area",
                "['Trim comp to work area', 'Ctrl+Shift+X', trimToWork, !!composition()?.work_area],",
            ),
        ],
    );
    table(
        "B-909_trim_to_work_area_table.md",
        "B-909: Trim Comp to Work Area, Ctrl+Shift+X (D-1030)",
        "b909_trim_to_work_area",
        "After Effects: Composition > Trim Comp to Work Area (Ctrl+Shift+X) makes the work area the \
         whole composition. The first rows run the window's own commands on \
         `Fixtures/projects/cel_holds_project.json` and read the composition back; the last rows pin \
         how the page sends it.",
        &rows,
    );
}
