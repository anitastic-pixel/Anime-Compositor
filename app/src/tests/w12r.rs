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

/// D-947 / B-827: every number field takes a sum, and +, * or / first works on the value there.
#[test]
fn b827_sums_in_number_fields() {
    let mut rows: Rows = Vec::new();
    let cases: &[(&str, f64, &str)] = &[
        ("+400", 960.0, "1360"),
        ("-400", 960.0, "-400"),
        ("960-400", 960.0, "560"),
        ("*2", 960.0, "1920"),
        ("/2", 960.0, "480"),
        ("+ 400", 960.0, "1360"),
        ("2*(3+4)", 0.0, "14"),
        ("1+2*3", 0.0, "7"),
        ("-(5-8)", 0.0, "3"),
        ("+10", -5.0, "5"),
        ("*-1", 30.0, "-30"),
        ("12.5", 3.0, "12.5"),
        (".5", 3.0, "0.5"),
        ("1e3", 0.0, "1000"),
        ("/0", 5.0, "not taken"),
        ("abc", 5.0, "not taken"),
        ("2+", 5.0, "not taken"),
        ("(2", 5.0, "not taken"),
        ("alert(1)", 5.0, "not taken"),
        ("", 5.0, "not taken"),
    ];
    let js: Vec<String> = cases
        .iter()
        .map(|(t, now, _)| format!("[{}, {now}]", serde_json::to_string(t).expect("text")))
        .collect();
    let said = node(
        "B-827",
        "typedSum",
        &format!(
            "out([{}].map(([t, now]) => {{ const v = typedSum(t, now); \
             return Number.isFinite(v) ? String(Math.round(v * 1e6) / 1e6) : 'not taken'; }}));",
            js.join(", ")
        ),
    );
    for ((text, now, want), got) in cases.iter().zip(said.as_array().expect("a list")) {
        rows.push((
            format!("`{text}` typed over {now}"),
            want.to_string(),
            got.as_str().unwrap_or("?").to_string(),
        ));
    }
    pins(
        &mut rows,
        &[(
            "the one number field every panel uses reads what is typed through typedSum",
            "      const said = typedSum(box.value, held); // D-947",
        )],
    );
    table(
        "B-827_typed_sums_table.md",
        "B-827: sums typed into number fields (D-947)",
        "b827_sums_in_number_fields",
        "Each row runs the page's own `typedSum` under Node: the text typed, the value the field \
         held, and the number it becomes (\"not taken\" leaves the field as it was). The last row \
         pins that `scrubValue`, the number field every panel uses, reads through it. The \
         private-app check is `verification/B-827_private_app_table.md`.",
        &rows,
    );
}

/// D-948 / B-828: After Effects' sixteen label colours, in its order with its names; the first
/// eight unchanged so a saved label shows as it did.
#[test]
fn b828_sixteen_label_colours() {
    let mut rows: Rows = Vec::new();
    let said = node(
        "B-828",
        "LABELS, LABEL_NAMES",
        "out({ n: LABELS.length, names: LABEL_NAMES.slice(1).join(', '), first: LABELS.slice(1, 9).join(' '), \
           rest: LABELS.slice(9).join(' ') });",
    );
    rows.push(("label colours besides none".into(), "16".into(), (said["n"].as_i64().unwrap_or(0) - 1).to_string()));
    rows.push((
        "their names, in After Effects' order".into(),
        "red, yellow, aqua, pink, lavender, peach, sea foam, blue, green, purple, orange, brown, \
         fuchsia, cyan, sandstone, dark green"
            .into(),
        said["names"].as_str().unwrap_or("?").into(),
    ));
    rows.push((
        "the first eight keep the colours they had (a saved 1 to 8 looks as before)".into(),
        "#c8554f #d9c75b #79b8b0 #d99bb5 #9f96c9 #d9a878 #8fb58f #6d86d9".into(),
        said["first"].as_str().unwrap_or("?").into(),
    ));
    rows.push((
        "the new eight, After Effects' 9 to 16".into(),
        "#4aa44c #8e2c9a #e8920d #7f452a #f46dd6 #3da2a5 #a89677 #1e401e".into(),
        said["rest"].as_str().unwrap_or("?").into(),
    ));

    // The window and the file.
    let viewer = Mutex::new(demo());
    let first = {
        let held = viewer.lock().expect("lock");
        let comp = held.document.project().composition(&held.composition).expect("comp");
        let id = comp.layers_in_order().next().expect("a layer").id.as_str().to_string();
        id
    };
    let label = |v: &Mutex<Viewer>| {
        let held = v.lock().expect("lock");
        let comp = held.document.project().composition(&held.composition).expect("comp");
        let n = comp.layer(&Id::new(first.as_str())).expect("layer").label;
        n
    };
    let _ = edit_command(&viewer, "layer.set_label", Some(&format!("layer={first}&label=16")));
    rows.push((
        "the window: layer.set_label label=16 (dark green)".into(),
        "label 16".into(),
        format!("label {}", label(&viewer)),
    ));
    let seventeen = edit_command(&viewer, "layer.set_label", Some(&format!("layer={first}&label=17")));
    rows.push((
        "the window: label=17 is refused and the label stays".into(),
        "refused (1 to 16), label 16".into(),
        format!("{}, label {}", if seventeen.as_deref().unwrap_or("").contains("1 to 16") { "refused (1 to 16)" } else { "taken" }, label(&viewer)),
    ));
    let saved = {
        let held = viewer.lock().expect("lock");
        persist::to_json(held.document.project(), &held.preserved)
    };
    let reopened = persist::load_str(&saved).expect("the saved project reads");
    let comp_id = viewer.lock().expect("lock").composition.clone();
    let read = |l: &persist::Loaded| {
        l.document
            .project()
            .composition(&comp_id)
            .and_then(|c| c.layer(&Id::new(first.as_str())))
            .map(|l| l.label)
            .unwrap_or(0)
    };
    rows.push(("saved and opened again, the layer's label".into(), "16".into(), read(&reopened).to_string()));
    let swap = |to: &str| saved.replace("\"label\": 16", &format!("\"label\": {to}")).replace("\"label\":16", &format!("\"label\":{to}"));
    let old = persist::load_str(&swap("8")).map(|l| read(&l).to_string()).unwrap_or_else(|d| d.message);
    rows.push(("an older file's label 8 (blue) opens as 8".into(), "8".into(), old));
    let past = persist::load_str(&swap("17")).is_err();
    rows.push(("a file with label 17 is refused, not quietly changed".into(), "true".into(), past.to_string()));

    pins(
        &mut rows,
        &[("the label squares sit nine to a row", ".menu .swatches { display: grid; grid-template-columns: repeat(9, 16px);")],
    );
    table(
        "B-828_label_colours_table.md",
        "B-828: After Effects' sixteen label colours (D-948)",
        "b828_sixteen_label_colours",
        "The first rows run the page's own `LABELS` and `LABEL_NAMES` under Node; then the real \
         window sets and refuses labels and the project is saved and opened again; the last row \
         pins the menu's layout. The private-app check is `verification/B-828_private_app_table.md`.",
        &rows,
    );
}

/// D-949 / B-829: an eyedropper after every colour field in the panels, with colour management on
/// or off. Off, the window answers a pick with the pixel's hex; a plain question still says off.
#[test]
fn b829_eyedropper_beside_every_colour_field() {
    let mut rows: Rows = Vec::new();
    let said = node(
        "B-829",
        "wantsDropper",
        "const field = (o) => Object.assign({ tagName: 'INPUT', type: 'color', closest: () => null, nextElementSibling: null }, o);\n\
         out({\n\
           panel: wantsDropper(field({})),\n\
           dialog: wantsDropper(field({ closest: (s) => (s.includes('dialog') ? {} : null) })),\n\
           twice: wantsDropper(field({ nextElementSibling: { classList: { contains: (c) => c === 'dropbtn' } } })),\n\
           number: wantsDropper(field({ type: 'number' })),\n\
           button: wantsDropper(field({ tagName: 'BUTTON' })),\n\
         });",
    );
    for (what, field, want) in [
        ("a colour field in a panel gets a dropper", "panel", "true"),
        ("one in a dialog does not (the picture cannot be clicked under it)", "dialog", "false"),
        ("a field that has its dropper gets no second one", "twice", "false"),
        ("a number field gets none", "number", "false"),
        ("nor a button", "button", "false"),
    ] {
        rows.push((what.into(), want.into(), said[field].to_string()));
    }

    // The window, with colour management off: the pick's hex is the pixel the screen shows.
    let dir = repo("Fixtures/colour/view");
    let viewer = Mutex::new(open(&dir.join("hdr_scene_off.json")).unwrap_or_else(|d| panic!("{}", d.message)));
    let ask = |v: &Mutex<Viewer>, q: &str| colour_info(v, Some(q)).unwrap_or_else(|why| panic!("{q}: {why}"));
    rows.push((
        "off: a plain question is still answered only with off (FX-COLC7-004)".into(),
        r#"{"managed":false}"#.into(),
        ask(&viewer, "frame=0&x=10&y=10").to_string(),
    ));
    rows.push((
        "off: a colour with pick=1 likewise (only a pixel is picked)".into(),
        r#"{"managed":false}"#.into(),
        ask(&viewer, "rgb=1,0,0&pick=1").to_string(),
    ));
    let project = viewer.lock().expect("lock").document.project().clone();
    let comp = viewer.lock().expect("lock").composition.clone();
    let shown = preview::preview_frame(&project, &comp, 0, &dir, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut FrameLog::new(0))
        .expect("the frame");
    let width = shown.width();
    let bytes = shown.to_srgb8_straight();
    for (x, y) in [(10usize, 10usize), (width / 2, shown.height() / 2), (width - 5, shown.height() - 5)] {
        let i = (y * width + x) * 4;
        let want = format!("#{:02x}{:02x}{:02x}", bytes[i], bytes[i + 1], bytes[i + 2]);
        let a = ask(&viewer, &format!("frame=0&x={x}&y={y}&pick=1"));
        rows.push((
            format!("off: picking {x}, {y} gives the colour shown there"),
            format!("{want}, managed false"),
            format!("{}, managed {}", a["hex"].as_str().unwrap_or("?"), a["managed"]),
        ));
    }
    let on = Mutex::new(open(&dir.join("hdr_scene_aces.json")).unwrap_or_else(|d| panic!("{}", d.message)));
    let a = ask(&on, "frame=0&x=10&y=10&pick=1");
    let b = ask(&on, "frame=0&x=10&y=10");
    rows.push((
        "on: pick=1 changes nothing in the answer".into(),
        "true".into(),
        (a == b && a["managed"] == true).to_string(),
    ));
    pins(
        &mut rows,
        &[
            ("every colour field added to the page is given its dropper", "new MutationObserver((ms) => ms.forEach((m) => m.addedNodes.forEach(addDroppers)))"),
            ("a pick asks with pick=1 and takes any answer with a hex", "if (!a || !a.hex) { $('status').textContent = 'Nothing to pick there.'; return; }"),
            ("colour management off no longer stops a pick", "  if (!on) return;   // D-949"),
        ],
    );
    table(
        "B-829_eyedropper_table.md",
        "B-829: an eyedropper beside every colour field (D-949)",
        "b829_eyedropper_beside_every_colour_field",
        "The first rows run the page's own `wantsDropper` under Node on made-up fields; then the \
         real window answers picks from `Fixtures/colour/view` with colour management off and on, \
         each compared with the bytes the screen shows; the last rows pin the wiring. The \
         private-app check is `verification/B-829_private_app_table.md`.",
        &rows,
    );
}

/// D-950 / B-830: the Align panel: line the chosen layers up by an edge or middle, to the
/// selection or the composition, or space three or more evenly; one entry to undo.
#[test]
fn b830_align_panel() {
    let mut rows: Rows = Vec::new();
    let said = node(
        "B-830",
        "alignMoves",
        "const A = { id: 'A', x0: 0, y0: 0, x1: 100, y1: 50 }, B = { id: 'B', x0: 200, y0: 100, x1: 260, y1: 200 },\n\
               C = { id: 'C', x0: 500, y0: 20, x1: 540, y1: 60 }, comp = { width: 1920, height: 1080 };\n\
         const go = (rects, how, to) => alignMoves(rects, how, to, comp).map((m) => m.id + ' ' + m.d.join(',')).join('; ');\n\
         out({\n\
           left: go([A, B, C], 'left', 'selection'), right: go([A, B, C], 'right', 'comp'),\n\
           hcentre: go([A, B, C], 'hcentre', 'selection'), vcentre: go([A, B, C], 'vcentre', 'comp'),\n\
           bottom: go([A, B, C], 'bottom', 'selection'), top: go([A, B, C], 'top', 'comp'),\n\
           distleft: go([C, A, B], 'distleft', 'selection'), disthcentre: go([A, B, C], 'disthcentre', 'comp'),\n\
           distvcentre: go([A, B, C], 'distvcentre', 'selection'), two: go([A, B], 'distleft', 'selection'),\n\
           one: go([B], 'left', 'selection'),\n\
         });",
    );
    for (what, field, want) in [
        ("left edges to the selection's left", "left", "A 0,0; B -200,0; C -500,0"),
        ("right edges to the composition's right (1920)", "right", "A 1820,0; B 1660,0; C 1380,0"),
        ("centres to the selection's centre (270)", "hcentre", "A 220,0; B 40,0; C -250,0"),
        ("middles to the composition's middle (540)", "vcentre", "A 0,515; B 0,390; C 0,500"),
        ("bottoms to the selection's bottom (200)", "bottom", "A 0,150; B 0,0; C 0,140"),
        ("tops to the composition's top", "top", "A 0,0; B 0,-100; C 0,-20"),
        ("distribute left edges: the outer two stay, the middle one goes halfway", "distleft", "A 0,0; B 50,0; C 0,0"),
        ("distribute centres (50, 230, 520 become 50, 285, 520)", "disthcentre", "A 0,0; B 55,0; C 0,0"),
        ("distribute middles (25, 40, 150 become 25, 87.5, 150)", "distvcentre", "A 0,0; C 0,47.5; B 0,0"),
        ("distribute with two layers does nothing", "two", ""),
        ("one layer and Selection aligns to the composition", "one", "B -200,0"),
    ] {
        rows.push((what.into(), want.into(), said[field].as_str().unwrap_or("?").into()));
    }

    // The window: three positions sent as the panel sends them are one entry to undo.
    let viewer = Mutex::new(demo());
    let depth = |v: &Mutex<Viewer>| v.lock().expect("lock").document.undo_depth();
    let (ids, before_project) = {
        let held = viewer.lock().expect("lock");
        let comp = held.document.project().composition(&held.composition).expect("comp");
        let ids: Vec<String> = comp.layers_in_order().map(|l| l.id.as_str().to_string()).take(3).collect();
        (ids, held.document.project().clone())
    };
    let before = depth(&viewer);
    for (n, id) in ids.iter().enumerate() {
        edit_command(&viewer, "property.set_base", Some(&format!("layer={id}&frame=0&drag=1&prop=position&value={},300", 100 * (n + 1))));
    }
    edit_command(&viewer, "property.drag_end", None);
    let moved = viewer.lock().expect("lock").document.project().clone() != before_project;
    let entries = depth(&viewer) - before;
    viewer.lock().expect("lock").document.undo();
    let back = viewer.lock().expect("lock").document.project().clone() == before_project;
    rows.push((
        format!("the window: {} layers moved as the panel sends, then one Ctrl+Z", ids.len()),
        "moved, 1 entry, all back".into(),
        format!("{}, {entries} entry, {}", if moved { "moved" } else { "not moved" }, if back { "all back" } else { "not back" }),
    ));
    pins(
        &mut rows,
        &[
            ("the panel sits in the Effect controls column", "<h2 data-fold=\"align\">"),
            ("to the selection or the composition", "<select id=\"alignto\"><option value=\"selection\">Selection</option><option value=\"comp\">Composition</option></select>"),
            ("a parented layer is left out (its Position is in its parent's space)", "const chosen = movable().filter((l) => !l.parent);"),
            ("each button moves through alignPanel", "b.onclick = () => alignPanel(row + how);"),
        ],
    );
    table(
        "B-830_align_panel_table.md",
        "B-830: the Align panel (D-950)",
        "b830_align_panel",
        "The first rows run the page's own `alignMoves` under Node on three made-up boxes in a \
         1920 by 1080 composition (A: 0 to 100 across, 0 to 50 down; B: 200 to 260, 100 to 200; \
         C: 500 to 540, 20 to 60), each answer the distance each layer moves; one row asks the real \
         window; the rest pin the wiring. The private-app check is `verification/B-830_private_app_table.md`.",
        &rows,
    );
}

/// D-951 / B-831: New Comp from Selection: a composition the size and length of the chosen
/// composition or drawings, holding it as its one layer, as one entry to undo.
#[test]
fn b831_new_comp_from_selection() {
    let mut rows: Rows = Vec::new();
    let said = node(
        "B-831",
        "newCompSteps",
        "const comp = { id: 'c1', name: 'Shot 1', layers: [], width: 1280, height: 720, duration_frames: 48,\n\
                        frame_rate: { numerator: 24000, denominator: 1001 } };\n\
         out({\n\
           comp: newCompSteps(comp, '24/1').join(' then '),\n\
           seq: newCompSteps({ id: 'a1', name: 'Run', kind: 'image_sequence' }, '12/1').join(' then '),\n\
           sound: newCompSteps({ id: 'a2', name: 'Voice', kind: 'audio' }, '24/1').length,\n\
         });",
    );
    for (what, field, want) in [
        (
            "a composition: its own size, length and rate, then it as a layer, as one drag",
            "comp",
            "/composition.create?name=Shot%201&width=1280&height=720&frames=48&fps=24000/1001&drag=1 then \
             /layer.add_composition?of=c1&drag=1 then /property.drag_end",
        ),
        (
            "drawings: the app reads their size and count; the rate is the open composition's",
            "seq",
            "/composition.create?name=Run&footage=a1&fps=12/1&drag=1 then \
             /layer.create?asset=a1&name=Run&drag=1 then /property.drag_end",
        ),
    ] {
        rows.push((what.into(), want.into(), said[field].as_str().unwrap_or("?").into()));
    }
    rows.push(("a sound makes nothing".into(), "0".into(), said["sound"].to_string()));

    // The window: the page's steps run as the page sends them.
    let run = |viewer: &Mutex<Viewer>, steps: &[String]| {
        for step in steps {
            let (id, q) = step[1..].split_once('?').map_or((&step[1..], None), |(i, q)| (i, Some(q)));
            edit_command(viewer, id, q);
        }
    };
    let viewer = Mutex::new(demo());
    let (asset, size, count, before, depth, shown) = {
        let held = viewer.lock().expect("lock");
        let project = held.document.project();
        let asset = project.assets.iter().filter(|a| a.kind == AssetKind::ImageSequence).max_by_key(|a| a.frames.len()).expect("a sequence").clone();
        let image = media::decode(&held.root.join(&asset.files()[0])).expect("decode");
        let out = (
            asset.clone(),
            format!("{}x{}", image.width(), image.height()),
            asset.frames.len(),
            project.clone(),
            held.document.undo_depth(),
            held.composition.clone(),
        );
        out
    };
    let steps = vec![
        format!("/composition.create?name={}&footage={}&fps=24/1&drag=1", asset.id.as_str(), asset.id.as_str()),
        format!("/layer.create?asset={}&name=Run&drag=1", asset.id.as_str()),
        "/property.drag_end".to_string(),
    ];
    run(&viewer, &steps);
    let made = {
        let held = viewer.lock().expect("lock");
        let comp = held.document.project().composition(&held.composition).expect("the new composition");
        let layers: Vec<&Layer> = comp.layers_in_order().collect();
        let out = format!(
            "{}x{}, {} frames, {} layer showing {}, {} entry",
            comp.width,
            comp.height,
            comp.duration_frames,
            layers.len(),
            layers.first().map_or("nothing".to_string(), |l| l.asset_id.as_str().to_string()),
            held.document.undo_depth() - depth,
        );
        out
    };
    rows.push((
        "the window: New Comp from Selection on the reference shot's drawings".into(),
        format!("{size}, {count} frames, 1 layer showing {}, 1 entry", asset.id.as_str()),
        made,
    ));
    viewer.lock().expect("lock").document.undo();
    let back = {
        let held = viewer.lock().expect("lock");
        let out = held.document.project().clone() == before;
        out
    };
    rows.push(("one Ctrl+Z takes the composition and its layer away".into(), "true".into(), back.to_string()));

    // A composition: the new one holds the shot as a layer, the shot's size and length.
    let (w, h, frames) = {
        let held = viewer.lock().expect("lock");
        let c = held.document.project().composition(&shown).expect("the shot");
        let out = (c.width, c.height, c.duration_frames);
        out
    };
    run(
        &viewer,
        &[
            format!("/composition.create?name=Shot&width={w}&height={h}&frames={frames}&fps=24/1&drag=1"),
            format!("/layer.add_composition?of={}&drag=1", shown.as_str()),
            "/property.drag_end".to_string(),
        ],
    );
    let nested = {
        let held = viewer.lock().expect("lock");
        let comp = held.document.project().composition(&held.composition).expect("the new composition");
        let out = format!(
            "{} layers, the first a {:?}, {}",
            comp.len(),
            comp.layers_in_order().next().map(|l| l.kind),
            held.composition != shown
        );
        out
    };
    rows.push((
        "the window: from the open composition".into(),
        "1 layers, the first a Some(Composition), true".into(),
        nested,
    ));
    let refused = edit_command(&viewer, "composition.create", Some("footage=nothing-here")).unwrap_or_default();
    rows.push((
        "a drawing that is not there is refused".into(),
        "true".into(),
        refused.contains("Choose a drawing").to_string(),
    ));
    pins(
        &mut rows,
        &[
            ("in a Project item's right-click menu", "[['New comp from selection', '', () => newCompFromSelection(id)]]"),
            ("in the Composition menu", "<button data-run=\"New comp from selection\"></button>"),
        ],
    );
    table(
        "B-831_new_comp_from_selection_table.md",
        "B-831: New Comp from Selection (D-951)",
        "b831_new_comp_from_selection",
        "The first rows run the page's own `newCompSteps` under Node on a made-up composition and \
         two made-up items; the window rows send those steps to a real window on the reference \
         shot; the rest pin the wiring. The private-app check is \
         `verification/B-831_private_app_table.md`.",
        &rows,
    );
}

fn header(r: &Response<Vec<u8>>, name: &str) -> String {
    r.headers().get(name).map_or_else(String::new, |v| v.to_str().unwrap_or("?").to_string())
}

/// D-952 / B-832: the viewer's resolution list, Full, Half, Third, Quarter and Auto, with Third
/// drawn by the window at a third of the size each way.
#[test]
fn b832_resolution_list() {
    let mut rows: Rows = Vec::new();
    let said = node(
        "B-832",
        "RESOLUTIONS, resolutionShown, resolutionSet",
        "out({\n\
           names: RESOLUTIONS.map(([, n]) => n).join(', '),\n\
           auto: JSON.stringify(resolutionSet('auto')), third: JSON.stringify(resolutionSet('third')),\n\
           lit: [['full', false], ['full', true], ['half', false], ['third', true], ['draft', true]]\n\
             .map(([q, f]) => resolutionShown(q, f)).join(', '),\n\
         });",
    );
    for (what, field, want) in [
        ("the list, as After Effects names it", "names", "Full, Half, Third, Quarter, Auto"),
        ("Auto is Full drawn at the size shown (Full pixels unticked)", "auto", "{\"quality\":\"full\",\"fullPixels\":false}"),
        ("a size from the list is that size at any zoom (Full pixels ticked)", "third", "{\"quality\":\"third\",\"fullPixels\":true}"),
        ("the line lit for Full unticked, Full ticked, Half unticked, Third, Quarter", "lit", "auto, full, half, third, draft"),
    ] {
        rows.push((what.into(), want.into(), said[field].as_str().unwrap_or("?").into()));
    }
    let sized = node(
        "B-162 / B-163",
        "sizedAs, askedAs",
        "out([sizedAs('/frame/7?q=full', 0.33), sizedAs('/frame/7?q=full', 0.4), sizedAs('/frame/7?q=third', 0.25),\n\
              sizedAs('/frame/7?q=third', 1), askedAs('/frame/7?q=third', true)].join(' '));",
    );
    rows.push((
        "Auto at 33% draws Third, at 40% Half; Third chosen at 25% draws Quarter, at 100% Third; a drag on Third is In motion".into(),
        "/frame/7?q=third /frame/7?q=half /frame/7?q=draft /frame/7?q=third /frame/7?q=motion".into(),
        sized.as_str().unwrap_or("?").into(),
    ));

    // The window draws Third: a third each way, labelled not final, and close to Full averaged
    // over each three by three square.
    let export = Mutex::new(Export::default());
    let viewer = Mutex::new(demo());
    let ask = |q| serve_logged(&viewer, &export, None, None, Ask::Frame(30), Some(q));
    let full = ask(PreviewQuality::Full);
    let third = ask(PreviewQuality::Third);
    rows.push((
        "frame 30 at Third: its size, its label and not final".into(),
        "640 360 Third true, 921600 bytes".into(),
        format!(
            "{} {} {} {}, {} bytes",
            header(&third, "x-width"),
            header(&third, "x-height"),
            header(&third, "x-quality"),
            header(&third, "x-differs"),
            third.body().len()
        ),
    ));
    let (f, t) = (full.body(), third.body());
    let mut total = 0.0;
    for y in 0..360usize {
        for x in 0..640usize {
            for c in 0..4usize {
                let mut sum = 0.0;
                for dy in 0..3 {
                    for dx in 0..3 {
                        sum += f[((y * 3 + dy) * 1920 + x * 3 + dx) * 4 + c] as f64;
                    }
                }
                total += (sum / 9.0 - t[(y * 640 + x) * 4 + c] as f64).abs();
            }
        }
    }
    let mean = total / (640.0 * 360.0 * 4.0);
    rows.push((
        "Third against Full averaged over each 3x3 square: mean difference under 3 levels of 255".into(),
        "true".into(),
        format!("{}", mean < 3.0),
    ));
    rows.push((
        "and Full asked after it is still Full, the same as export".into(),
        "Full false".into(),
        {
            let again = ask(PreviewQuality::Full);
            format!("{} {}", header(&again, "x-quality"), header(&again, "x-differs"))
        },
    ));
    pins(
        &mut rows,
        &[
            ("the list sits above the D key's Full/Draft line in the quality menu", "  $('toggle').before(b);"),
            ("the lit line follows each frame that comes back", "    litResolution(); // D-952"),
            ("and the Full pixels tick", "sizeStillRight(); litResolution(); };"),
        ],
    );
    table(
        "B-832_resolution_list_table.md",
        "B-832: the viewer's resolution list (D-952)",
        "b832_resolution_list",
        &format!(
            "The first rows run the page's own list and its sizing rule under Node; the window rows \
             ask the reference shot's frame 30 at Third and Full (Third's mean difference from Full \
             averaged over 3x3 squares was {mean:.2} levels of 255); the rest pin the wiring. The \
             private-app check is `verification/B-832_private_app_table.md`."
        ),
        &rows,
    );
}

/// D-953 / B-833: the small shortcuts: F4 for the switches or the modes, Ctrl+Shift+Y for the
/// selected solid's settings, angles as turns and degrees, and a wiggle on Position driven by
/// Speed and Amount sliders.
#[test]
fn b833_small_shortcuts() {
    let mut rows: Rows = Vec::new();
    let angles = node(
        "B-833",
        "angleText, typedTurns",
        "out({\n\
           shown: [390, -30, -390, 0, 720, 45.5].map(angleText).join(', '),\n\
           typed: ['1x+30', '-1x-30', '-1x30', '2x0', '0x+45.5\u{b0}', '45'].map(typedTurns).join(', '),\n\
         });",
    );
    rows.push((
        "angles shown as turns and degrees: 390, -30, -390, 0, 720, 45.5".into(),
        "1x+30\u{b0}, 0x-30\u{b0}, -1x-30\u{b0}, 0x+0\u{b0}, 2x+0\u{b0}, 0x+45.5\u{b0}".into(),
        angles["shown"].as_str().unwrap_or("?").into(),
    ));
    rows.push((
        "typed as turns and degrees: 1x+30, -1x-30, -1x30, 2x0, 0x+45.5\u{b0}; a plain 45 is left to the sum reader".into(),
        "390, -390, -390, 720, 45.5, ".into(),
        angles["typed"].as_str().unwrap_or("?").into(),
    ));
    let said = node(
        "B-833b",
        "cyclePanes, solidSteps, wigglePreset",
        "const cls = new Set();\n\
         const el = { textContent: '', classList: { toggle: (c, on) => (on ? cls.add(c) : cls.delete(c)) } };\n\
         globalThis.$ = () => el;\n\
         const panes = [];\n\
         for (let i = 0; i < 3; i++) { cyclePanes(); panes.push([...cls].join('+') || 'none'); }\n\
         const solid = { id: 's1', name: 'Red' };\n\
         const none = wigglePreset({ effects: [] }), one = wigglePreset({ effects: [{ type_id: 'core.slider_control' }] });\n\
         out({\n\
           panes: panes.join(', '),\n\
           same: solidSteps(solid, 'Red', 640, 360, [1, 0, 0]).join(' then '),\n\
           renamed: solidSteps(solid, 'Sky', 640, 360, [0, 0, 1]).join(' then '),\n\
           none: none.transform.position.expression.text.split('\\n')[1],\n\
           one: one.transform.position.expression.text.split('\\n')[1],\n\
           sliders: none.effects.map((e) => e.parameters.slider).join(', '),\n\
           query: encodeURIComponent(JSON.stringify(none)),\n\
         });",
    );
    for (what, field, want) in [
        ("F4 three times: the switches alone, the modes alone, both", "panes", "nomodes, noswitches, none"),
        (
            "Solid Settings OK on a solid: its size and colour as one drag",
            "same",
            "/solid.set?layer=s1&drag=1&width=640&height=360&color=1,0,0 then /property.drag_end",
        ),
        (
            "and a new name in the same drag",
            "renamed",
            "/solid.set?layer=s1&drag=1&width=640&height=360&color=0,0,1 then /layer.rename?layer=s1&name=Sky&drag=1 then /property.drag_end",
        ),
        (
            "Wiggle position: the expression reads the two new sliders",
            "none",
            "wiggle(effect(\"Slider Control\")(\"Slider\"), effect(\"Slider Control 2\")(\"Slider\"))",
        ),
        (
            "on a layer that already has a slider, the new ones are numbered after it",
            "one",
            "wiggle(effect(\"Slider Control 2\")(\"Slider\"), effect(\"Slider Control 3\")(\"Slider\"))",
        ),
        ("Speed and Amount start at 2 wiggles a second and 30 pixels", "sliders", "2, 30"),
    ] {
        rows.push((what.into(), want.into(), said[field].as_str().unwrap_or("?").into()));
    }

    // The window: the page's preset on the reference shot's front drawn layer.
    use anime_compositor::expr::Target;
    use anime_compositor::model::Prop;
    let viewer = Mutex::new(demo());
    let (id, before, depth) = {
        let held = viewer.lock().expect("lock");
        let comp = held.document.project().composition(&held.composition).expect("comp");
        let id = comp.layers_in_order().find(|l| l.kind == LayerKind::Raster).expect("a drawn layer").id.clone();
        let out = (id, held.document.project().clone(), held.document.undo_depth());
        out
    };
    let query = said["query"].as_str().expect("the preset");
    let refused = edit_command(&viewer, "preset.apply", Some(&format!("layer={}&frame=0&preset={query}", id.as_str())));
    let positions = |viewer: &Mutex<Viewer>| -> (Vec<(f64, f64)>, (f64, f64)) {
        let held = viewer.lock().expect("lock");
        let comp = held.document.project().composition(&held.composition).expect("comp");
        let layer = comp.layer(&id).expect("the layer");
        let base = layer.transform.position.value_at(0).as_vec2().expect("a 2D position");
        let out = (
            (0..48)
                .map(|f| {
                    anime_compositor::expr::evaluate(comp, &Target::Layer(id.clone()), Prop::Position, f)
                        .expect("the expression runs")
                        .as_vec2()
                        .expect("2D")
                })
                .collect(),
            base,
        );
        out
    };
    let (moved, base) = positions(&viewer);
    let furthest = moved.iter().map(|(x, y)| (x - base.0).abs().max((y - base.1).abs())).fold(0.0, f64::max);
    let (sliders, entries) = {
        let held = viewer.lock().expect("lock");
        let comp = held.document.project().composition(&held.composition).expect("comp");
        let out = (
            comp.layer(&id)
                .expect("the layer")
                .effects
                .iter()
                .filter_map(|e| match e.effect {
                    Effect::SliderControl { slider } => Some(slider.to_string()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join(", "),
            held.document.undo_depth() - depth,
        );
        out
    };
    rows.push((
        "the window: Wiggle position on the front drawn layer".into(),
        "Set the position expression and 2 more, sliders 2, 30, 1 entry".into(),
        format!("{}, sliders {sliders}, {entries} entry", refused.unwrap_or_default()),
    ));
    rows.push((
        "over 48 frames Position wanders, never more than 30 pixels from where it was".into(),
        "true, true".into(),
        format!("{}, {}", furthest > 1.0, furthest <= 30.0),
    ));
    let amount = {
        let held = viewer.lock().expect("lock");
        let comp = held.document.project().composition(&held.composition).expect("comp");
        let out = comp.layer(&id).expect("the layer").effects.last().expect("Amount").instance_id.clone();
        out
    };
    edit_command(&viewer, "effect.set_parameters", Some(&format!("layer={}&effect={}&slider=0", id.as_str(), amount.as_str())));
    let (still, _) = positions(&viewer);
    rows.push((
        "Amount set to 0: Position stays where it was".into(),
        "true".into(),
        still.iter().all(|p| *p == base).to_string(),
    ));
    viewer.lock().expect("lock").document.undo();
    viewer.lock().expect("lock").document.undo();
    let back = {
        let held = viewer.lock().expect("lock");
        let out = held.document.project().clone() == before;
        out
    };
    rows.push(("two Ctrl+Z (the Amount, then the preset) put the layer back".into(), "true".into(), back.to_string()));
    pins(
        &mut rows,
        &[
            ("every degree number shows turns and degrees", "handle.textContent = suffix === '\u{b0}' ? angleText(round(held)) : number(round(held)) + suffix; // D-953"),
            ("F4 on the keyboard", "else if (e.key === 'F4' && !e.ctrlKey && !e.altKey && !e.shiftKey) { e.preventDefault(); cyclePanes(); }"),
            ("Ctrl+Shift+Y on the keyboard", "    solidSettings();\n"),
            ("the columns F4 hides", "#timeline.nomodes .layerhead > :nth-child(10), #timeline.nomodes .layerhead > :nth-child(11) { display: none; }"),
            ("Wiggle position in the Animation menu", "<button data-run=\"Wiggle position\"></button>"),
            ("the command finder's lines", "['Solid settings', 'Ctrl+Shift+Y', { key: 'Y', ctrlKey: true, shiftKey: true }],\n    ['Switches or modes', 'F4', { key: 'F4' }],"),
        ],
    );
    table(
        "B-833_small_shortcuts_table.md",
        "B-833: F4, Ctrl+Shift+Y, angles in turns, and a wiggle with sliders (D-953)",
        "b833_small_shortcuts",
        "The first rows run the page's own code under Node; the window rows apply the page's own \
         Wiggle position preset to the reference shot's front drawn layer and read Position after \
         the expression on frames 0 to 47; the rest pin the wiring. The private-app check is \
         `verification/B-833_private_app_table.md`.",
        &rows,
    );
}

/// D-954 / B-834: Ctrl+click on a key, found by the private-app check: turning an auto key back
/// to linear straightens the side coming into it as well, and the whole click is one entry.
#[test]
fn b834_ctrl_click_back_to_linear() {
    use anime_compositor::model::{Interp, Keyframe};
    let mut rows: Rows = Vec::new();
    let viewer = Mutex::new(demo());
    {
        // Three plain linear Rotation keys, 0, 30, 0, on the first drawn layer, as layer3 has them
        // in the reference shot.
        use anime_compositor::model::{Prop, Value};
        let held = &mut *viewer.lock().expect("lock");
        let composition = held.composition.clone();
        let comp = held.document.project().composition(&composition).expect("comp");
        let id = comp.layers_in_order().find(|l| l.kind == LayerKind::Raster).expect("a drawn layer").id.clone();
        for (frame, value) in [(0, 0.0), (40, 30.0), (80, 0.0)] {
            held.document
                .apply(anime_compositor::command::Command::SetKeyframe {
                    composition: composition.clone(),
                    target: anime_compositor::command::Target::Layer(id.clone()),
                    prop: Prop::Rotation,
                    frame,
                    value: Value::Scalar(value),
                    interp: Interp::Linear,
                    spatial: None,
                })
                .expect("a key");
        }
    }
    let keys = |viewer: &Mutex<Viewer>| -> (String, Vec<Keyframe>) {
        let held = viewer.lock().expect("lock");
        let comp = held.document.project().composition(&held.composition).expect("comp");
        let layer = comp
            .layers_in_order()
            .find(|l| l.transform.rotation.keyframes().len() >= 3)
            .expect("a layer with three Rotation keys");
        let out = (layer.id.as_str().to_string(), layer.transform.rotation.keyframes().to_vec());
        out
    };
    let (id, start) = keys(&viewer);
    let middle = start[1].frame;
    let depth = || viewer.lock().expect("lock").document.undo_depth();
    let send = |request: &str| {
        let request = request.trim_start_matches('/');
        let (what, query) = request.split_once('?').unwrap_or((request, ""));
        edit_command(&viewer, what, Some(query));
    };
    let d0 = depth();
    // The page's first Ctrl+click on the plain linear middle key: auto, in one drag.
    send(&format!("/keyframe.set_kind?kind=auto&key={id}|rotation|{middle}&drag=1"));
    send("/property.drag_end");
    let (_, auto) = keys(&viewer);
    let eased = |k: &Keyframe| matches!(k.interp, Interp::Ease { .. });
    rows.push((
        "Ctrl+click on the plain middle key: auto, easing both sides, one entry".into(),
        "true, true, 1".into(),
        format!("{}, {}, {}", eased(&auto[0]), eased(&auto[1]), depth() - d0),
    ));
    // What the page sends for the second Ctrl+click: the side coming in, from the key before.
    let js_keys: Vec<serde_json::Value> = auto
        .iter()
        .map(|k| match k.interp {
            Interp::Ease { x1, y1, x2, y2 } => {
                serde_json::json!({ "frame": k.frame, "interp": "ease", "ease": [x1, y1, x2, y2] })
            }
            Interp::Hold => serde_json::json!({ "frame": k.frame, "interp": "hold" }),
            _ => serde_json::json!({ "frame": k.frame, "interp": "linear" }),
        })
        .collect();
    let layer = serde_json::json!({ "id": id, "transform": { "rotation": { "keyframes": js_keys } } });
    let said = node(
        "B-834",
        "intoKey",
        &format!(
            "globalThis.whose = (l) => 'layer=' + l.id;\n\
             const layer = {layer};\n\
             out({{ into: intoKey(layer, 'rotation', {middle}), first: intoKey(layer, 'rotation', {first}),\n\
               straight: intoKey({{ id: 'x', transform: {{ rotation: {{ keyframes: [{{ frame: 0, interp: 'linear' }}] }} }} }}, 'rotation', 10) }});",
            first = start[0].frame
        ),
    );
    let into = said["into"].as_str().unwrap_or("?").to_string();
    rows.push((
        "the side coming in is sent as the key before's curve, straightened".into(),
        format!("/keyframe.set_interp?layer={id}&prop=rotation&frame={}&drag=1&mode=linear", start[0].frame),
        into.clone(),
    ));
    rows.push((
        "nothing to send for the first key, or for a side already straight".into(),
        "null, null".into(),
        format!("{}, {}", said["first"], said["straight"]),
    ));
    let d1 = depth();
    send(&format!("/keyframe.set_kind?kind=bezier&key={id}|rotation|{middle}&drag=1"));
    send(&format!("/keyframe.set_interp?layer={id}&prop=rotation&frame={middle}&mode=linear&drag=1"));
    send(&into);
    send("/property.drag_end");
    let (_, back) = keys(&viewer);
    rows.push((
        "Ctrl+click again: every Rotation key as it started, plain and linear, one entry".into(),
        "true, 1".into(),
        format!("{}, {}", back == start, depth() - d1),
    ));
    viewer.lock().expect("lock").document.undo();
    rows.push(("one Ctrl+Z: the auto key again".into(), "true".into(), (keys(&viewer).1 == auto).to_string()));
    viewer.lock().expect("lock").document.undo();
    rows.push(("and one more: as it started".into(), "true".into(), (keys(&viewer).1 == start).to_string()));
    pins(
        &mut rows,
        &[(
            "the second Ctrl+click sends the side coming in",
            "  const before = to === 'linear' ? intoKey(layer, prop, key.frame) : null;\n  if (before) sends.push(before);\n",
        )],
    );
    table(
        "B-834_ctrl_click_linear_table.md",
        "B-834: Ctrl+click back to linear straightens both sides, as one entry (D-954)",
        "b834_ctrl_click_back_to_linear",
        "Found by B-825's check in a private copy of the app: the second Ctrl+click left the side \
         coming into the key eased, and each click was two or three entries to undo. The rows send \
         the page's requests to the window's own command handler on the reference shot's keyed \
         Rotation; `intoKey` is the page's own code run under Node. The private-app check is \
         `verification/B-825_private_app_table.md`.",
        &rows,
    );
}
