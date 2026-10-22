//! B-619: the page's lists of the names a shape, a mask, a group, a repeater and a path operation
//! answer to are the core's, so a name added to one and not the other is caught here rather than
//! by a row that will not draw or a key the core refuses.
//!
//! Writes `verification/B-619_name_lists_table.md`.
//!
//! # Where the expected values come from
//!
//! The core's own constants: `shape::SHAPE_PROPERTIES`, `ParametricKind::settings`,
//! `GROUP_PROPERTIES`, `REPEATER_PROPERTIES`, `OPERATIONS` and `mask::MASK_PROPERTIES`. The page
//! is the copy, read from the same `index.html` the window shows.
use anime_compositor::mask::MASK_PROPERTIES;
use anime_compositor::shape::{
    ParametricKind, GROUP_PROPERTIES, OPERATIONS, REPEATER_PROPERTIES, SHAPE_PROPERTIES,
};
use serde_json::{json, Value as J};
use std::path::PathBuf;

const PAGE: &str = include_str!("../../ui/index.html");

/// The page's `const <name> = ` literal as JSON: quotes made double, keys quoted and trailing
/// commas dropped. The lists hold only plain words, numbers and null.
fn literal(name: &str) -> J {
    let from = PAGE
        .find(&format!("const {name} = "))
        .unwrap_or_else(|| panic!("the page has {name}"))
        + name.len()
        + 9;
    // The literal's lines, its comment lines left out, up to where its brackets close.
    let (mut depth, mut quoted, mut js) = (0, false, String::new());
    'lines: for line in PAGE[from..].lines() {
        if line.trim_start().starts_with("//") {
            continue;
        }
        for c in line.chars() {
            js.push(c);
            match c {
                '\'' => quoted = !quoted,
                '[' | '{' if !quoted => depth += 1,
                ']' | '}' if !quoted => depth -= 1,
                _ => {}
            }
            if depth == 0 {
                break 'lines;
            }
        }
        js.push('\n');
    }
    let js = js.replace('\'', "\"");
    let mut out = String::new();
    for line in js.lines() {
        let body = line.trim_start();
        match body.split_once(':') {
            Some((key, _)) if key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') => {
                out.push_str(&format!("\"{key}\"{}\n", &body[key.len()..]))
            }
            _ => out.push_str(&format!("{body}\n")),
        }
    }
    let mut json = String::new();
    for c in out.chars() {
        if c == ']' || c == '}' {
            let kept = json.trim_end().len();
            if json[..kept].ends_with(',') {
                json.truncate(kept - 1);
            }
        }
        json.push(c);
    }
    serde_json::from_str(&json).unwrap_or_else(|e| panic!("{name}: {e}\n{json}"))
}

fn bound(x: f64) -> J {
    if x.is_finite() {
        json!(x)
    } else {
        J::Null
    }
}

fn same(a: &J, b: &J) -> bool {
    match (a, b) {
        (J::Number(x), J::Number(y)) => x.as_f64() == y.as_f64(),
        (J::Array(x), J::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(a, b)| same(a, b))
        }
        _ => a == b,
    }
}

#[test]
fn the_page_names_what_the_core_names() {
    let mut rows: Vec<(String, J, J)> = Vec::new();
    for (list, core) in [
        ("SHAPE_PROPS", &SHAPE_PROPERTIES[..]),
        ("MASK_PROPS", &MASK_PROPERTIES[..]),
        ("GROUP_PROPS", &GROUP_PROPERTIES[..]),
        ("REPEATER_PROPS", &REPEATER_PROPERTIES[..]),
    ] {
        rows.push((list.into(), json!(core), literal(list)));
    }
    let page = literal("PARAMETRIC_SETTINGS");
    let kinds: Vec<&str> = page
        .as_object()
        .unwrap()
        .keys()
        .map(|k| k.as_str())
        .collect();
    // The page looks a kind up by name, so the order is not compared.
    let mut names = ParametricKind::NAMES.to_vec();
    names.sort();
    rows.push((
        "PARAMETRIC_SETTINGS, the kinds".into(),
        json!(names),
        json!(kinds),
    ));
    for name in ParametricKind::NAMES {
        let kind = ParametricKind::named(name).unwrap();
        rows.push((
            format!("PARAMETRIC_SETTINGS, {name}"),
            json!(kind.settings()),
            page[name].clone(),
        ));
    }
    let page = literal("OPERATIONS");
    let types: Vec<&str> = page
        .as_object()
        .unwrap()
        .keys()
        .map(|k| k.as_str())
        .collect();
    let mut names: Vec<&str> = OPERATIONS.iter().map(|o| o.0).collect();
    names.sort();
    rows.push(("OPERATIONS, the types".into(), json!(names), json!(types)));
    for (kind, numbers, choice) in OPERATIONS {
        let line = &page[kind];
        let held: Vec<J> = line[1]
            .as_array()
            .map(|ns| ns.iter().map(|n| json!([n[0], n[1], n[2]])).collect())
            .unwrap_or_default();
        rows.push((
            format!("OPERATIONS, {kind}: numbers and ranges"),
            json!(numbers
                .iter()
                .map(|(n, lo, hi, _)| json!([n, bound(*lo), bound(*hi)]))
                .collect::<Vec<_>>()),
            json!(held),
        ));
        rows.push((
            format!("OPERATIONS, {kind}: choice and words"),
            choice.map_or(J::Null, |(c, words)| json!([c, words])),
            line.get(2).cloned().unwrap_or(J::Null),
        ));
    }

    let mut out = String::from(
        "# B-619: the page's name lists against the core's\n\n\
         Written by `app/src/tests/name_lists.rs`. Each line is one list the window keeps a copy \
         of: what the core says, what the page says, and whether they are the same. A range \
         written `null` has no end.\n\n\
         | List | The core | The page | Same |\n| --- | --- | --- | --- |\n",
    );
    let mut passed = 0;
    for (list, core, page) in &rows {
        let ok = same(core, page);
        passed += ok as usize;
        let verdict = if ok { "yes" } else { "**NO**" };
        out.push_str(&format!("| {list} | {core} | {page} | {verdict} |\n"));
    }
    out.push_str(&format!(
        "\nB-619: {passed} of {} lists match.\n",
        rows.len()
    ));
    let table =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../verification/B-619_name_lists_table.md");
    std::fs::write(table, &out).unwrap();
    assert_eq!(
        passed,
        rows.len(),
        "see verification/B-619_name_lists_table.md"
    );
}
