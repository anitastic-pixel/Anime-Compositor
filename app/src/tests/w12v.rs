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
                "const box = kind === 'polygon' || kind === 'star' ? [lx, ly, mx, my] : dragCorners(lx, ly, mx, my, m);",
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
