//! Wave 12 lane R: the page's quality-of-life items from the tutorial follow-alongs (D-945 on).
//! Each test runs the page's own marked code under Node, pins the lines that wire it in, and
//! writes its table under `verification/`.

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
    let path = std::env::temp_dir().join(format!("w12r_{}.js", marker.replace([' ', '/'], "_")));
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
        "# {title}\n\nWritten by `{test}` in `app/src/tests/w12r.rs`.\n\n{intro}\n\n\
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

/// D-945 / B-825: the F9 family read in one place, Ctrl+M as After Effects' Add to Render Queue
/// with the export on Ctrl+Alt+M, and Ctrl+click on a key toggling linear and auto bezier.
#[test]
fn b825_shortcut_fixes() {
    let mut rows: Rows = Vec::new();
    let said = node(
        "B-825",
        "easeSide, ctrlClickTo",
        "out({\n\
           f9: easeSide({}), shift: easeSide({ shiftKey: true }),\n\
           ctrlShift: easeSide({ ctrlKey: true, shiftKey: true }), ctrl: easeSide({ ctrlKey: true }),\n\
           plain: ctrlClickTo({ frame: 0 }), linear: ctrlClickTo({ interp: 'linear', kind: 'bezier' }),\n\
           ease: ctrlClickTo({ interp: 'ease' }), hold: ctrlClickTo({ interp: 'hold' }),\n\
           auto: ctrlClickTo({ interp: 'linear', kind: 'auto' }), cont: ctrlClickTo({ kind: 'continuous' }),\n\
         });",
    );
    for (what, field, want) in [
        ("F9 eases", "f9", "both"),
        ("Shift+F9 eases the side reached from", "shift", "in"),
        ("Ctrl+Shift+F9 eases the side left by (the defect named: it is out, not read as anything else)", "ctrlShift", "out"),
        ("Ctrl+F9 alone stays out, as before", "ctrl", "out"),
        ("Ctrl+click on a key with nothing set: auto bezier", "plain", "auto"),
        ("Ctrl+click on a plain linear key: auto bezier", "linear", "auto"),
        ("Ctrl+click on an eased key: linear", "ease", "linear"),
        ("Ctrl+click on a held key: linear", "hold", "linear"),
        ("Ctrl+click on an auto bezier key: linear", "auto", "linear"),
        ("Ctrl+click on a continuous key: linear", "cont", "linear"),
    ] {
        rows.push((what.into(), want.into(), said[field].as_str().unwrap_or("?").into()));
    }
    pins(
        &mut rows,
        &[
            ("F9 goes through easeSide", "easeKeys(easeSide(e));"),
            ("the key menu names Ctrl+Shift+F9", "['Easy ease out', 'Ctrl+Shift+F9', () => easeKeys('out'), some],"),
            ("the palette has Easy ease out", "['Easy ease out', 'Ctrl+Shift+F9', { key: 'F9', ctrlKey: true, shiftKey: true }],"),
            ("Ctrl+M adds to the render queue", "(e.key === 'm' || e.key === 'M')) { e.preventDefault(); $('addtoqueue').click(); }"),
            ("Ctrl+Alt+M exports, tested first", "else if (e.ctrlKey && e.altKey && e.code === 'KeyM') { e.preventDefault(); $('export').click(); }"),
            ("the Export menu line says Ctrl+Alt+M", "data-line=\"Export PNG sequence\" data-key=\"Ctrl+Alt+M\""),
            ("the Add to Render queue line says Ctrl+M", "<button id=\"addtoqueue\" data-key=\"Ctrl+M\""),
            ("the Render button no longer claims Ctrl+M", "'Render the open composition: asks where, then writes it';"),
            ("Shift adds a key; Ctrl alone no longer does", "    const add = e.shiftKey;\n"),
            ("a Ctrl click that did not move toggles", "else if (to === key.frame && toggle) ctrlClickKey(layer, prop, key);"),
        ],
    );
    table(
        "B-825_shortcut_fixes_table.md",
        "B-825: three shortcut fixes (D-945)",
        "b825_shortcut_fixes",
        "The first rows run the page's own `easeSide` and `ctrlClickTo` under Node; the rest pin \
         the lines that wire them in. The private-app check is `verification/B-825_private_app_table.md`.",
        &rows,
    );
}

