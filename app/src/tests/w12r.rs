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

/// D-946 / B-826: a parent or a track matte set with several layers chosen is set on all of them,
/// as one entry to undo; a matte chosen where there was none is matte only (After Effects turns
/// the matte layer's eye off); and the track matte has a pick whip.
#[test]
fn b826_parent_and_matte_on_every_chosen_layer() {
    let mut rows: Rows = Vec::new();
    let said = node(
        "B-826",
        "manyRequests, parentRequest, matteRequest, matteOnly",
        "const all = [{ id: 'a' }, { id: 'b' }, { id: 'c' }, { id: 'p', parent: 'b' }, { id: 'm' },\n\
                       { id: 's', kind: 'audio' }, { id: 'n', kind: 'null' }];\n\
         const L = (id) => all.find((l) => l.id === id);\n\
         const par = (on, chosen, to) => manyRequests(L(on), chosen, all, (l) => parentRequest(l, to, 12, all)).join(' ');\n\
         const mat = (on, chosen, to) => manyRequests(L(on), chosen, all, (l) => matteRequest(l, to, true, all)).join(' ');\n\
         out({\n\
           three: par('a', ['a', 'c', 'b'], 'm'),\n\
           alone: par('a', ['a'], 'm'),\n\
           notChosen: par('a', ['b', 'c'], 'm'),\n\
           withItself: par('a', ['a', 'm'], 'm'),\n\
           loop: par('a', ['a', 'b'], 'p'),\n\
           sound: par('a', ['a', 's'], 'm'),\n\
           none: par('a', ['a', 'c'], ''),\n\
           mattes: mat('a', ['a', 'c', 'm', 's'], 'm'),\n\
           nullMatte: mat('a', ['a'], 'n'),\n\
           firstMatte: matteOnly(null, 'm', false),\n\
           changed: matteOnly({ layer_id: 'x', matte_only: false }, 'm', false),\n\
           kept: matteOnly({ layer_id: 'x', matte_only: true }, 'm', true),\n\
           cleared: matteOnly(null, '', false),\n\
         });",
    );
    for (what, field, want) in [
        ("parent M set on A with A, C, B chosen: all three, in the order chosen",
         "three", "/layer.set_parent?layer=a&parent=m&frame=12 /layer.set_parent?layer=c&parent=m&frame=12 /layer.set_parent?layer=b&parent=m&frame=12"),
        ("set on A chosen alone: A only", "alone", "/layer.set_parent?layer=a&parent=m&frame=12"),
        ("set on A when A is not among the chosen: A only", "notChosen", "/layer.set_parent?layer=a&parent=m&frame=12"),
        ("the parent itself among the chosen is left out", "withItself", "/layer.set_parent?layer=a&parent=m&frame=12"),
        ("a chosen layer the parent rides on is left out (no loop)", "loop", "/layer.set_parent?layer=a&parent=p&frame=12"),
        ("a sound among the chosen is left out", "sound", "/layer.set_parent?layer=a&parent=m&frame=12"),
        ("None clears every chosen layer's parent", "none", "/layer.set_parent?layer=a&parent=&frame=12 /layer.set_parent?layer=c&parent=&frame=12"),
        ("matte M on A, C, M and a sound: A and C", "mattes", "/layer.set_matte?layer=a&matte=m&only=true /layer.set_matte?layer=c&matte=m&only=true"),
        ("a null is never a matte", "nullMatte", ""),
    ] {
        rows.push((what.into(), want.into(), said[field].as_str().unwrap_or("?").into()));
    }
    for (what, field, want) in [
        ("a first matte is matte only (its eye off)", "firstMatte", "true"),
        ("changing a matte keeps the tick as it was (off)", "changed", "false"),
        ("changing a matte keeps the tick as it was (on)", "kept", "true"),
        ("clearing a matte keeps the tick as it was", "cleared", "false"),
    ] {
        rows.push((what.into(), want.into(), said[field].to_string()));
    }

    // The window: layers given one parent as the page sends several are one entry to undo.
    let viewer = Mutex::new(demo());
    let depth = |v: &Mutex<Viewer>| v.lock().expect("lock").document.undo_depth();
    let ids: Vec<String> = {
        let held = viewer.lock().expect("lock");
        let comp = held.document.project().composition(&held.composition).expect("comp");
        comp.layers_in_order().map(|l| l.id.as_str().to_string()).collect()
    };
    let (parent, kids) = (&ids[0], &ids[1..ids.len().min(4)]);
    let before = depth(&viewer);
    for kid in kids {
        edit_command(&viewer, "layer.set_parent", Some(&format!("layer={kid}&parent={parent}&frame=0&drag=1")));
    }
    edit_command(&viewer, "property.drag_end", None);
    let set = {
        let held = viewer.lock().expect("lock");
        let comp = held.document.project().composition(&held.composition).expect("comp");
        kids.iter()
            .filter(|k| comp.layer(&Id::new(k.as_str())).is_some_and(|l| l.parent.as_ref().is_some_and(|p| p.as_str() == parent)))
            .count()
    };
    rows.push((
        format!("the window: {} layers given one parent with drag=1, then drag_end", kids.len()),
        format!("{} parented, 1 entry to undo", kids.len()),
        format!("{set} parented, {} entry to undo", depth(&viewer) - before),
    ));

    pins(
        &mut rows,
        &[
            ("the Parent list sends for every chosen layer", "chooser.onchange = () => sendMany(layer, (l) => parentRequest(l, chooser.value, frame, layers()));"),
            ("the Track matte list too, hiding a first matte", "chooser.onchange = () => { only.checked = matteOnly(held, chooser.value, only.checked); send(); };"),
            ("several requests are one entry to undo", "for (const request of sends) await command(request + '&drag=1');"),
            ("the track matte has a pick whip", "const pick = whip(layer, null, true);"),
            ("which sends for every chosen layer", "return ['Track matte: ' + parent.name, () => sendMany(layer, (l) => matteRequest(l, parent.id, only, layers()))];"),
            ("and so does the parent whip", "return ['Parent: ' + parent.name, () => sendMany(layer, (l) => parentRequest(l, parent.id, frame, layers()))];"),
        ],
    );
    table(
        "B-826_parent_and_matte_table.md",
        "B-826: parent and track matte on every chosen layer, and a matte pick whip (D-946)",
        "b826_parent_and_matte_on_every_chosen_layer",
        "The first rows run the page's own `manyRequests`, `parentRequest`, `matteRequest` and \
         `matteOnly` under Node on made-up layers; one row asks the real window; the rest pin the \
         lines that wire them in. The private-app check is `verification/B-826_private_app_table.md`.",
        &rows,
    );
}
