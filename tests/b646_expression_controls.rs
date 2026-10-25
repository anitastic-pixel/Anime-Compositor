//! D-766 / B-646: After Effects' Expression Controls, and `if`/`else`, comparisons and logic.
//!
//! Writes `verification/B-646_expression_controls_table.md`.
//!
//! The expected values are document 25's FX-EXPR-017 to 020, printed by
//! `tools/expression_reference.py`, which shares nothing with `src/expr.rs`. As in B-14b, the
//! tables are read out of document 25 and filled in again by the build, cell by cell.
//!
//! FX-EXPR-020 is lane q's two tutorial cases (a slider driving a wiggle, a checkbox driving
//! opacity) with real controls. The test also proves the new file opens and saves back to the
//! same project, and that the older expression fixture still opens and saves unchanged.

use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::expr::{evaluate, Target};
use anime_compositor::model::{Id, Project, Prop, Value};
use anime_compositor::persist;
use serde_json::Value as J;

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn read(rel: &str) -> J {
    serde_json::from_str(&fs::read_to_string(repo(rel)).expect("the fixture")).expect("JSON")
}

fn open(data: &J) -> Project {
    match persist::load_str(&data.to_string()) {
        Ok(l) => l.document.project().clone(),
        Err(d) => panic!("the project did not open: {} {}", d.message, d.detail),
    }
}

fn layer<'a>(data: &'a mut J, id: &str) -> &'a mut J {
    data["compositions"][0]["layers"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|l| l["id"] == id)
        .expect("a fixture layer")
}

/// The fixture with Controls' rotation carrying `text`, as document 25's cases are.
fn with(data: &J, text: &str) -> J {
    let mut data = data.clone();
    layer(&mut data, "layer-ctl")["transform"]["rotation"]["expression"] =
        serde_json::json!({ "text": text, "enabled": true });
    data
}

/// The value in the file's units, and the identifier (`none` for none). A failure is drawn at
/// its keys, so that is the value shown for one.
fn outcome(data: &J, owner: &str, p: Prop, frame: i32) -> (String, String) {
    let project = open(data);
    let comp = &project.compositions[0];
    let t = Target::Layer(Id::new(owner));
    let (value, code) = match evaluate(comp, &t, p, frame) {
        Ok(v) => (v, "none".to_string()),
        Err(e) => (
            comp.layer(&Id::new(owner))
                .unwrap()
                .transform
                .get(p)
                .unwrap()
                .value_at(frame),
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

struct Table {
    title: String,
    header: Vec<String>,
    rows: Vec<Vec<String>>,
}

fn cells(line: &str) -> Vec<String> {
    line.trim()
        .trim_matches('|')
        .split(" | ")
        .map(|c| c.trim().to_string())
        .collect()
}

fn catalogue() -> Vec<Table> {
    let text = fs::read_to_string(repo("Markdown/25_Test_Fixture_Catalog.md")).expect("doc 25");
    let start = text.find("## Expression fixtures").expect("the section");
    let end = start + text[start..].find("## Packaging fixtures").expect("the next section");
    let mut tables = Vec::new();
    let mut lines = text[start..end].lines().peekable();
    while let Some(line) = lines.next() {
        let Some(rest) = line.strip_prefix("FX-EXPR-") else {
            continue;
        };
        let title = format!("FX-EXPR-{rest}");
        while !lines.peek().expect("a table follows").starts_with('|') {
            lines.next();
        }
        let header = cells(lines.next().unwrap());
        lines.next();
        let mut rows = Vec::new();
        while lines.peek().is_some_and(|l| l.starts_with('|')) {
            rows.push(cells(lines.next().unwrap()));
        }
        tables.push(Table { title, header, rows });
    }
    tables
}

fn numbers(cell: &str) -> Option<Vec<f64>> {
    cell.trim_matches(|c| c == '(' || c == ')')
        .split(", ")
        .map(|n| n.parse::<f64>().ok())
        .collect()
}

fn agrees(expected: &str, built: &str, tolerance: f64) -> bool {
    match (numbers(expected), numbers(built)) {
        (Some(a), Some(b)) => {
            a.len() == b.len() && a.iter().zip(&b).all(|(x, y)| (x - y).abs() <= tolerance)
        }
        _ => expected == built,
    }
}

/// A case cell's expression: backticks off, `\n` a line break.
fn code(cell: &str) -> String {
    cell.trim_matches('`').replace("\\n", "\n")
}

fn build_row(data: &J, n: u32, _i: usize, row: &[String]) -> Vec<String> {
    let mut built = vec![row[0].clone()];
    match n {
        17 | 18 => {
            let reader = n == 17 && row[0].starts_with("Reader");
            let d = if reader { data.clone() } else { with(data, &code(&row[0])) };
            for f in [0, 12, 24, 48] {
                let (v, c) = if reader {
                    outcome(&d, "layer-reader", Prop::Rotation, f)
                } else {
                    outcome(&d, "layer-ctl", Prop::Rotation, f)
                };
                assert_eq!(c, "none", "{} at frame {f}", row[0]);
                built.push(v);
            }
        }
        19 => {
            let d = match row[0].as_str() {
                r if r.starts_with("Slider Control 2's own") => {
                    let mut d = with(data, "effect(\"Slider Control 2\")(\"Slider\")");
                    layer(&mut d, "layer-ctl")["effects"][6]["parameters"]["slider"]
                        ["expression"]["text"] = J::from("effect(\"Slider Control 2\")(\"Slider\") + 1");
                    d
                }
                r if r.starts_with("the Layer Control set") => {
                    let mut d = with(data, "effect(\"Layer Control\")(\"Layer\").position[0]");
                    layer(&mut d, "layer-ctl")["effects"][5]["parameters"]["layer"] = J::from("");
                    d
                }
                _ => with(data, &code(&row[0])),
            };
            let (v, c) = outcome(&d, "layer-ctl", Prop::Rotation, 36);
            built.push(format!("{v}, {c}"));
        }
        20 => {
            let f = row[0].parse::<i32>().expect("a frame");
            let (w, wc) = outcome(data, "layer-wiggle", Prop::Position, f);
            let (b, bc) = outcome(data, "layer-box", Prop::Opacity, f);
            assert_eq!((wc.as_str(), bc.as_str()), ("none", "none"), "frame {f}");
            built.push(w);
            built.push(b);
        }
        _ => unreachable!(),
    }
    built
}

#[test]
fn b646_expression_controls() {
    let data = read("Fixtures/projects/expression_controls_project.json");
    let tables: Vec<Table> = catalogue().into_iter().skip(16).collect();
    assert_eq!(tables.len(), 4, "document 25 has FX-EXPR-017 to 020");
    let (mut total, mut ok) = (0, 0);
    let mut body = String::new();
    for (n, table) in (17u32..).zip(&tables) {
        assert!(table.title.starts_with(&format!("FX-EXPR-{n:03}")));
        let tolerance = if n == 20 { 1e-12 } else { 1e-9 };
        body.push_str(&format!("\n## {}\n\n", table.title));
        body.push_str(&format!("| {} | matches document 25 |\n", table.header.join(" | ")));
        body.push_str(&format!("|{}\n", " --- |".repeat(table.header.len() + 1)));
        for (i, row) in table.rows.iter().enumerate() {
            let built = build_row(&data, n, i, row);
            assert_eq!(built.len(), row.len(), "{} row {i}", table.title);
            let mut wrong = Vec::new();
            for (c, (e, b)) in row.iter().zip(&built).enumerate().skip(1) {
                total += 1;
                if agrees(e, b, tolerance) {
                    ok += 1;
                } else {
                    wrong.push(format!("{} should be {e}", table.header[c]));
                }
            }
            let verdict = if wrong.is_empty() {
                "yes".to_string()
            } else {
                format!("**NO**: {}", wrong.join("; "))
            };
            body.push_str(&format!("| {} | {verdict} |\n", built.join(" | ")));
        }
    }

    // Saving: the new file opens and saves back to the same project, twice over the same
    // bytes; the older expression fixture does too, so old projects load unchanged.
    let mut saves = String::from(
        "\n## Opening and saving\n\n| file | saved again equals the file | second save, same bytes |\n| --- | --- | --- |\n",
    );
    let mut saves_ok = true;
    for rel in [
        "Fixtures/projects/expression_controls_project.json",
        "Fixtures/projects/expression_project.json",
    ] {
        let original = read(rel);
        let loaded = persist::load_str(&original.to_string()).expect("opens");
        let first = persist::to_json(loaded.document.project(), &loaded.preserved);
        let again = persist::load_str(&first).expect("reopens");
        let second = persist::to_json(again.document.project(), &again.preserved);
        let same = serde_json::from_str::<J>(&first).unwrap() == original;
        let stable = first == second;
        saves_ok &= same && stable;
        saves.push_str(&format!(
            "| `{rel}` | {} | {} |\n",
            if same { "yes" } else { "**NO**" },
            if stable { "yes" } else { "**NO**" }
        ));
    }

    let out = format!(
        "# B-646 — After Effects' Expression Controls (D-766)\n\n\
         **{ok} of {total} numbers and diagnostics match document 25**, and opening and saving \
         {}.\n\n\
         Generated by `tests/b646_expression_controls.rs` from \
         `Fixtures/projects/expression_controls_project.json`.\n\n\
         ## How to read this\n\n\
         Each table is document 25's table filled in again by the build. The last column says \
         whether every number matches what `tools/expression_reference.py` worked out separately \
         in Python. Numbers are compared to 1e-9, and lane q's wiggle (FX-EXPR-020) to 1e-12. \
         A refused case shows the keyed value it is drawn with, then its diagnostic.\n{body}{saves}",
        if saves_ok { "all pass" } else { "do **not** all pass" }
    );
    fs::write(repo("verification/B-646_expression_controls_table.md"), &out).expect("write");
    assert_eq!(ok, total, "see verification/B-646_expression_controls_table.md");
    assert!(saves_ok, "see verification/B-646_expression_controls_table.md");
}
