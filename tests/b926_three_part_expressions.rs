//! D-1046 / B-926: three-part position expressions on flat layers, `[x, y, z]`,
//! `wiggle(...)[2]` and `.length`, as After Effects runs them.
//!
//! Writes `verification/B-926_three_part_expressions_table.md`.
//!
//! The expected values are document 25's FX-EXPR-021, printed by
//! `tools/expression_reference.py --three-part`, which shares nothing with `src/expr.rs`. As in
//! B-646, the table is read out of document 25 and filled in again by the build, cell by cell.

use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::expr::{evaluate, Target};
use anime_compositor::model::{Id, Project, Prop, Value};
use anime_compositor::persist;
use serde_json::Value as J;

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn open(data: &J) -> Project {
    match persist::load_str(&data.to_string()) {
        Ok(l) => l.document.project().clone(),
        Err(d) => panic!("the project did not open: {} {}", d.message, d.detail),
    }
}

/// The fixture with one layer property's expression replaced.
fn with(data: &J, owner: &str, name: &str, text: &str) -> J {
    let mut data = data.clone();
    let layer = data["compositions"][0]["layers"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|l| l["id"] == owner)
        .expect("a fixture layer");
    layer["transform"][name]["expression"] = serde_json::json!({ "text": text, "enabled": true });
    data
}

/// The value in the file's units, and the identifier (`none` for none). A failure is drawn at
/// its keys, so that is the value shown for one.
fn outcome(data: &J, owner: &str, p: Prop, frame: i32) -> (String, String) {
    let project = open(data);
    let comp = &project.compositions[0];
    let (value, code) = match evaluate(comp, &Target::Layer(Id::new(owner)), p, frame) {
        Ok(v) => (v, "none".to_string()),
        Err(e) => (
            comp.layer(&Id::new(owner)).unwrap().transform.get(p).unwrap().value_at(frame),
            e.id.as_str().to_string(),
        ),
    };
    let shown = match value {
        Value::Scalar(x) => show(x),
        Value::Vec2(x, y) => format!("({}, {})", show(x), show(y)),
    };
    (shown, code)
}

/// Python's `repr` for a float, and a whole number without its point, as the reference prints.
fn show(x: f64) -> String {
    if x.fract() == 0.0 && x.abs() < 1e15 {
        format!("{}", x as i64)
    } else {
        format!("{x:?}")
    }
}

fn cells(line: &str) -> Vec<String> {
    line.trim().trim_matches('|').split(" | ").map(|c| c.trim().to_string()).collect()
}

fn numbers(cell: &str) -> Option<Vec<f64>> {
    cell.trim_matches(|c| c == '(' || c == ')')
        .split(", ")
        .map(|n| n.parse::<f64>().ok())
        .collect()
}

fn agrees(expected: &str, built: &str) -> bool {
    match (numbers(expected), numbers(built)) {
        (Some(a), Some(b)) => {
            a.len() == b.len() && a.iter().zip(&b).all(|(x, y)| (x - y).abs() <= 1e-9)
        }
        _ => expected == built,
    }
}

#[test]
fn b926_three_part_expressions() {
    let text = fs::read_to_string(repo("Markdown/25_Test_Fixture_Catalog.md")).expect("doc 25");
    let start = text
        .find("### Three-part position expressions on flat layers (D-1046)")
        .expect("the section");
    let lines: Vec<&str> = text[start..].lines().collect();
    let at = lines.iter().position(|l| l.starts_with("FX-EXPR-021")).expect("the table");
    let mut rows = lines[at + 1..]
        .iter()
        .skip_while(|l| !l.starts_with('|'))
        .take_while(|l| l.starts_with('|'))
        .map(|l| cells(l));
    let header = rows.next().unwrap();
    let rows: Vec<Vec<String>> = rows.skip(1).collect();
    assert_eq!(rows.len(), 12, "FX-EXPR-021 has twelve expressions");

    let data: J = serde_json::from_str(
        &fs::read_to_string(repo("Fixtures/projects/expression_project.json")).expect("fixture"),
    )
    .expect("JSON");
    let (mut total, mut ok) = (0, 0);
    let mut body = format!("| {} | matches document 25 |\n", header.join(" | "));
    body.push_str(&format!("|{}\n", " --- |".repeat(header.len() + 1)));
    for row in &rows {
        let (owner, name) = row[0].split_once(' ').expect("layer and property");
        let p = match name {
            "position" => Prop::Position,
            "rotation" => Prop::Rotation,
            other => panic!("{other}"),
        };
        let code = row[1].trim_matches('`').replace("\\n", "\n");
        let d = with(&data, owner, name, &code);
        let mut built = vec![row[0].clone(), row[1].clone()];
        let mut diagnostic = String::new();
        for (i, f) in [0, 12, 24].into_iter().enumerate() {
            let (v, c) = outcome(&d, owner, p, f);
            built.push(v);
            if i == 0 {
                diagnostic = c;
            }
        }
        built.push(diagnostic);
        let mut wrong = Vec::new();
        for (c, (e, b)) in row.iter().zip(&built).enumerate().skip(2) {
            total += 1;
            if agrees(e, b) {
                ok += 1;
            } else {
                wrong.push(format!("{} should be {e}", header[c]));
            }
        }
        let verdict = if wrong.is_empty() {
            "yes".to_string()
        } else {
            format!("**NO**: {}", wrong.join("; "))
        };
        body.push_str(&format!("| {} | {verdict} |\n", built.join(" | ")));
    }

    let out = format!(
        "# B-926 — three-part position expressions on flat layers (D-1046)\n\n\
         **{ok} of {total} numbers and diagnostics match document 25.**\n\n\
         Generated by `tests/b926_three_part_expressions.rs` from \
         `Fixtures/projects/expression_project.json`.\n\n\
         ## How to read this\n\n\
         Each row puts one expression on Shake's position (kept at (960, 540) without it) or \
         Spin's rotation (0) and shows what the build gives at frames 0, 12 and 24, and the \
         warning. The last column says whether every cell matches what \
         `tools/expression_reference.py --three-part` worked out separately in Python (to \
         1e-9). The first row is the tutorial nlgsV151IVU's shake at 2:10; the first six are \
         let through by D-1046, the last six are still refused. `\\n` is a line break.\n\n\
         ## FX-EXPR-021\n\n{body}"
    );
    fs::write(repo("verification/B-926_three_part_expressions_table.md"), &out).expect("write");
    assert_eq!(ok, total, "see verification/B-926_three_part_expressions_table.md");
}
