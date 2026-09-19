//! T-12 / R-13 / B-14b: expressions, against D-59.
//!
//! Writes `verification/B-14b_expression_table.md`.
//!
//! # Where the expected values come from
//!
//! `Markdown/25_Test_Fixture_Catalog.md`, FX-EXPR-001 to 016, every number of which is printed by
//! `tools/expression_reference.py`, a second implementation of document 09's language in Python
//! that shares nothing with `src/expr.rs`. The tables are read out of document 25 itself rather
//! than copied here, so this test and the catalogue cannot drift apart: each table is filled in
//! again by the build, cell by cell, and every cell is compared.
//!
//! # What else this proves
//!
//! That an expression reaches the picture (Fade is drawn at half opacity at frame 12), that a
//! failing one is reported on the frame while the layer is still drawn at its keys, and that an
//! export meeting one is refused before anything is written.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

use anime_compositor::compose::plan_frame;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::export::{export_sequence, ExportRequest, ExportStatus, MissingSource, OutputFormat};
use anime_compositor::expr::{evaluate, Target};
use anime_compositor::model::{Id, Project, Prop, Value};
use anime_compositor::persist;
use anime_compositor::{OutputAlpha, OutputDepth};
use serde_json::Value as J;

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn fixture() -> J {
    let text = fs::read_to_string(repo("Fixtures/projects/expression_project.json"))
        .expect("the expression fixture");
    serde_json::from_str(&text).expect("the fixture is JSON")
}

fn open(data: &J) -> Project {
    match persist::load_str(&data.to_string()) {
        Ok(l) => l.document.project().clone(),
        Err(d) => panic!("the project did not open: {} {}", d.message, d.detail),
    }
}

fn target(name: &str) -> Target {
    match name {
        "camera" => Target::Camera,
        id => Target::Layer(Id::new(id)),
    }
}

fn prop(name: &str) -> Prop {
    [
        Prop::Anchor,
        Prop::Position,
        Prop::Scale,
        Prop::Rotation,
        Prop::Opacity,
        Prop::Depth,
        Prop::Zoom,
    ]
    .into_iter()
    .find(|p| p.as_str() == name)
    .expect("a property name")
}

/// The fixture with one property's expression replaced, as document 25's cases are.
fn with(data: &J, owner: &str, name: &str, text: &str) -> J {
    let mut data = data.clone();
    let comp = &mut data["compositions"][0];
    let record = if owner == "camera" {
        &mut comp["camera"][name]
    } else {
        let layer = comp["layers"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|l| l["id"] == owner)
            .expect("a fixture layer");
        &mut layer["transform"][name]
    };
    record["expression"] = serde_json::json!({ "text": text, "enabled": true });
    data
}

/// Document 25's shape: the value in the file's units and the identifier, `none` for none.
/// A failure is drawn at the keys, so that is the value shown for one.
fn outcome(data: &J, owner: &str, name: &str, frame: i32) -> (String, String) {
    let project = open(data);
    let comp = &project.compositions[0];
    let (t, p) = (target(owner), prop(name));
    let (value, code) = match evaluate(comp, &t, p, frame) {
        Ok(v) => (v, "none".to_string()),
        Err(e) => (keyed(comp, &t, p, frame), e.id.as_str().to_string()),
    };
    let k = if p == Prop::Scale { 100.0 } else { 1.0 };
    let shown = match value {
        Value::Scalar(x) => show(x * k),
        Value::Vec2(x, y) => format!("({}, {})", show(x * k), show(y * k)),
    };
    (shown, code)
}

fn keyed(comp: &anime_compositor::model::Composition, t: &Target, p: Prop, frame: i32) -> Value {
    match t {
        Target::Camera => {
            let camera = comp.camera.as_ref().expect("the fixture has a camera");
            let cp = anime_compositor::model::CameraProp::from_str(p.as_str()).unwrap();
            camera.get(cp).value_at(frame)
        }
        Target::Layer(id) => {
            let layer = comp.layer(id).expect("the layer is there");
            layer
                .transform
                .get(p)
                .expect("a transform property")
                .value_at(frame)
        }
    }
}

fn value(data: &J, owner: &str, name: &str, frame: i32) -> String {
    outcome(data, owner, name, frame).0
}

/// Python's `repr` for a float, and a whole number without its point, as the reference prints.
fn show(x: f64) -> String {
    if x.fract() == 0.0 && x.abs() < 1e15 {
        format!("{}", x as i64)
    } else {
        format!("{x:?}")
    }
}

/// One of document 25's tables: its heading line and its rows, cells trimmed.
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
    let end = start
        + text[start..]
            .find("## Persistence fixtures")
            .expect("the next section");
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
        tables.push(Table {
            title,
            header,
            rows,
        });
    }
    tables
}

fn numbers(cell: &str) -> Option<Vec<f64>> {
    cell.trim_matches(|c| c == '(' || c == ')')
        .split(", ")
        .map(|n| n.parse::<f64>().ok())
        .collect()
}

/// Whether two cells agree: numbers to the tolerance, anything else exactly.
fn agrees(expected: &str, built: &str, tolerance: f64) -> bool {
    match (numbers(expected), numbers(built)) {
        (Some(a), Some(b)) => {
            a.len() == b.len() && a.iter().zip(&b).all(|(x, y)| (x - y).abs() <= tolerance)
        }
        _ => expected == built,
    }
}

fn code(text: &str) -> &str {
    text.trim_matches('`')
}

/// The build's cells for one of document 25's rows, the first column copied.
fn build_row(data: &J, n: u32, i: usize, row: &[String]) -> Vec<String> {
    let first = row[0].clone();
    let frame = || first.parse::<i32>().expect("a frame");
    let rest: Vec<String> = match n {
        1 => vec![
            value(data, "layer-spin", "rotation", frame()),
            value(data, "layer-follow", "rotation", frame()),
        ],
        2 => vec![value(data, "layer-shake", "position", frame())],
        3 => vec![value(data, "layer-shake-twos", "position", frame())],
        4 => vec![match i {
            0 => value(data, "layer-shake", "position", 12),
            1 => value(
                &with(
                    data,
                    "layer-shake",
                    "position",
                    "seedRandom(5)\nwiggle(2, 30)",
                ),
                "layer-shake",
                "position",
                12,
            ),
            _ => value(
                &with(data, "layer-spin", "rotation", "wiggle(2, 30)"),
                "layer-spin",
                "rotation",
                12,
            ),
        }],
        5 => vec![
            value(data, "layer-lead", "position", frame()),
            value(data, "layer-trail", "position", frame()),
        ],
        6 => ["cycle", "pingpong", "offset", "continue"]
            .iter()
            .map(|k| {
                let text = format!("loopOut(\"{k}\")");
                value(
                    &with(data, "layer-loop", "rotation", &text),
                    "layer-loop",
                    "rotation",
                    frame(),
                )
            })
            .collect(),
        7 => vec![
            value(data, "layer-fade", "opacity", frame()),
            value(
                &with(data, "layer-fade", "opacity", "ease(time, 0, 1, 0, 100)"),
                "layer-fade",
                "opacity",
                frame(),
            ),
        ],
        8 => {
            let owner = match row[0].as_str() {
                "rotation" => "layer-spin",
                "position" => "layer-shake",
                "opacity" => "layer-fade",
                _ => "camera",
            };
            let text = code(&row[1]);
            let (v, c) = outcome(&with(data, owner, &row[0], text), owner, &row[0], 12);
            vec![row[1].clone(), v, c]
        }
        9 => {
            let mut case = data.clone();
            let comp = &mut case["compositions"][0];
            if i == 1 {
                comp["layers"][0]["name"] = J::from("Something else entirely");
            } else if i == 2 {
                comp["layers"]
                    .as_array_mut()
                    .unwrap()
                    .retain(|l| l["id"] != "layer-spin");
                comp["layer_order"]
                    .as_array_mut()
                    .unwrap()
                    .retain(|l| l != "layer-spin");
            }
            let (v, c) = outcome(&case, "layer-follow", "rotation", 12);
            vec![v, c]
        }
        10 => {
            let (text, owner) = match i {
                0 => ("thisComp.layer(\"layer-follow\").rotation", "layer-spin"),
                1 => ("thisComp.layer(\"layer-follow\").rotation", "layer-follow"),
                2 => ("thisLayer.rotation + 1", "layer-spin"),
                _ => ("thisProperty.valueAtTime(time - 1) + value", "layer-spin"),
            };
            let case = with(data, "layer-spin", "rotation", text);
            let (v, c) = outcome(&case, owner, "rotation", 12);
            vec![row[1].clone(), v, c]
        }
        11 => {
            let (case, frame) = if i < 2 {
                let chase = with(
                    data,
                    "layer-spin",
                    "rotation",
                    "thisComp.layer(\"layer-follow\").rotation.valueAtTime(time - 1/24) + 1",
                );
                let chase = with(
                    &chase,
                    "layer-follow",
                    "rotation",
                    "thisComp.layer(\"layer-spin\").rotation.valueAtTime(time) + 1",
                );
                (chase, if i == 0 { 40 } else { 3 })
            } else {
                let long = format!("1{}", " + 1".repeat(1100));
                assert_eq!(first, format!("`1 + 1 + ...`, {} bytes", long.len()));
                (with(data, "layer-spin", "rotation", &long), 40)
            };
            let (v, c) = outcome(&case, "layer-spin", "rotation", frame);
            vec![v, c]
        }
        12 => {
            let case = with(data, "layer-spin", "rotation", code(&row[0]));
            let (v, c) = outcome(&case, "layer-spin", "rotation", 12);
            vec![v, c]
        }
        13 => [
            "random()",
            "random(10)",
            "random(5, 10)",
            "seedRandom(3, 1)\nrandom()",
        ]
        .iter()
        .map(|t| {
            value(
                &with(data, "layer-spin", "rotation", t),
                "layer-spin",
                "rotation",
                frame(),
            )
        })
        .collect(),
        14 => vec![
            value(data, "camera", "position", frame()),
            value(
                &with(
                    data,
                    "layer-spin",
                    "rotation",
                    "thisComp.activeCamera.zoom / 100",
                ),
                "layer-spin",
                "rotation",
                frame(),
            ),
        ],
        15 => vec![value(
            &with(
                data,
                "layer-shake",
                "position",
                "bob = Math.sin(time * Math.PI * 2) * 20\nvalue + [0, bob]",
            ),
            "layer-shake",
            "position",
            frame(),
        )],
        16 => {
            let (case, owner, name) = match i {
                0 => (data.clone(), "layer-off", "opacity"),
                1 => (
                    with(
                        data,
                        "layer-spin",
                        "rotation",
                        "thisComp.layer(\"layer-off\").opacity",
                    ),
                    "layer-spin",
                    "rotation",
                ),
                _ => (
                    with(
                        data,
                        "layer-spin",
                        "rotation",
                        "thisComp.layer(\"layer-fade\").opacity",
                    ),
                    "layer-spin",
                    "rotation",
                ),
            };
            let (v, c) = outcome(&case, owner, name, 12);
            vec![v, c]
        }
        _ => panic!("document 25 has a table this test does not know: {n}"),
    };
    let mut out = vec![first];
    out.extend(rest);
    out
}

fn preview_and_export(data: &J, out: &mut String) -> bool {
    let mut ok = true;
    let mut line = |out: &mut String, check: &str, expected: &str, built: String| {
        let pass = expected == built;
        ok &= pass;
        out.push_str(&format!(
            "| {check} | {expected} | {built} | {} |\n",
            if pass { "yes" } else { "**NO**" }
        ));
    };
    out.push_str(
        "\n## In the picture, and at export\n\n\
         The fixture's one drawing is the first cel of the reference shot, copied to where the \
         project expects it.\n\n\
         | check | expected | built | matches |\n| --- | --- | --- | --- |\n",
    );
    let root = repo("target/b14b-scratch");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("media")).unwrap();
    fs::copy(
        repo("Fixtures/reference_shot/layer1/layer1_000.png"),
        root.join("media/cel_0001.png"),
    )
    .unwrap();
    let comp = Id::new("comp-main");

    let project = open(data);
    let mut log = FrameLog::new(3);
    let plan = plan_frame(&project, &comp, 12, &root, &mut log).expect("frame 12 plans");
    let fade = plan
        .layers
        .iter()
        .find(|l| l.id.as_str() == "layer-fade")
        .expect("Fade drawn");
    line(
        out,
        "Fade is drawn at the opacity its expression gives at frame 12",
        "0.5",
        fade.opacity.to_string(),
    );
    let off = plan
        .layers
        .iter()
        .find(|l| l.id.as_str() == "layer-off")
        .expect("drawn");
    line(
        out,
        "Switched off is drawn at its keyed opacity, its expression being off",
        "1",
        off.opacity.to_string(),
    );
    line(
        out,
        "the fixture as written reports nothing at frame 12",
        "none",
        ids(&log.ids_at(12)),
    );

    let broken = with(data, "layer-spin", "rotation", "[1, 2]");
    let project = open(&broken);
    let mut log = FrameLog::new(3);
    let plan = plan_frame(&project, &comp, 12, &root, &mut log).expect("frame 12 plans");
    line(
        out,
        "Spin's rotation set to `[1, 2]`: frame 12 reports it on Spin, and on Follow, which reads          Spin's rotation",
        "EXPRESSION_TYPE, EXPRESSION_TYPE",
        ids(&log.ids_at(12)),
    );
    line(
        out,
        "and Spin is still drawn, at its keys",
        "yes",
        if plan.layers.iter().any(|l| l.id.as_str() == "layer-spin") {
            "yes"
        } else {
            "no"
        }
        .to_string(),
    );

    for (policy, label) in [
        (MissingSource::Block, "the default"),
        (
            MissingSource::RenderTransparent,
            "leave out missing drawings",
        ),
    ] {
        let dir = root.join(format!("out-{label}"));
        fs::create_dir_all(&dir).unwrap();
        let request = ExportRequest {
            composition: comp.clone(),
            first_frame: 10,
            last_frame: 14,
            output_dir: dir.clone(),
            naming: "shot_%04d.png".to_string(),
            depth: OutputDepth::Eight,
            alpha: OutputAlpha::Straight,
            tile_size: 256,
            missing: policy,
            format: OutputFormat::Png,
            choices: Default::default(),
        };
        let report = export_sequence(&project, &root, &request, &AtomicBool::new(false));
        line(
            out,
            &format!("exporting frames 10 to 14 with that expression ({label}): the job is"),
            "refused",
            if report.status == ExportStatus::Blocked {
                "refused"
            } else {
                "not refused"
            }
            .to_string(),
        );
        line(
            out,
            "files written",
            "0",
            fs::read_dir(&dir).unwrap().count().to_string(),
        );
        let first = report.diagnostics.first();
        line(
            out,
            "the reason given",
            "EXPRESSION_TYPE: The expression on Spin's rotation does not work at frame 10, so \
             nothing was exported. It fails on frames 10 to 14.",
            first.map_or("none".to_string(), |d| {
                let ranges = d.detail.rsplit(". ").next().unwrap_or("").to_string();
                format!("{}: {} {}", d.id, d.message, ranges)
            }),
        );
    }

    let mut off = broken.clone();
    off["compositions"][0]["layers"][0]["transform"]["rotation"]["expression"]["enabled"] =
        J::from(false);
    let project = open(&off);
    let dir = root.join("out-off");
    fs::create_dir_all(&dir).unwrap();
    let request = ExportRequest {
        composition: comp.clone(),
        first_frame: 12,
        last_frame: 12,
        output_dir: dir.clone(),
        naming: "shot_%04d.png".to_string(),
        depth: OutputDepth::Eight,
        alpha: OutputAlpha::Straight,
        tile_size: 256,
        missing: MissingSource::Block,
        format: OutputFormat::Png,
        choices: Default::default(),
    };
    let report = export_sequence(&project, &root, &request, &AtomicBool::new(false));
    line(
        out,
        "the same expression switched off: frame 12 exports",
        "1 file",
        format!(
            "{} file{}",
            report.written.len(),
            if report.written.len() == 1 { "" } else { "s" }
        ),
    );
    ok
}

fn ids(ids: &[DiagnosticId]) -> String {
    if ids.is_empty() {
        return "none".to_string();
    }
    ids.iter()
        .map(|i| i.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

#[test]
fn b14b_expressions() {
    let data = fixture();
    let tables = catalogue();
    assert_eq!(
        tables.len(),
        16,
        "document 25 has sixteen expression tables"
    );
    let mut out = String::new();
    let (mut cells_total, mut cells_ok) = (0, 0);
    let mut body = String::new();
    for (n, table) in (1u32..).zip(&tables) {
        assert!(table.title.starts_with(&format!("FX-EXPR-{n:03}")));
        let tolerance = if [2, 3, 4, 13, 14].contains(&n) {
            1e-12
        } else {
            1e-9
        };
        body.push_str(&format!("\n## {}\n\n", table.title));
        body.push_str(&format!(
            "| {} | matches document 25 |\n",
            table.header.join(" | ")
        ));
        body.push_str(&format!("|{}\n", " --- |".repeat(table.header.len() + 1)));
        for (i, row) in table.rows.iter().enumerate() {
            let built = build_row(&data, n, i, row);
            assert_eq!(built.len(), row.len(), "{} row {i}", table.title);
            let mut wrong = Vec::new();
            for (c, (e, b)) in row.iter().zip(&built).enumerate().skip(1) {
                cells_total += 1;
                if agrees(e, b, tolerance) {
                    cells_ok += 1;
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
    let mut extra = String::new();
    let extra_ok = preview_and_export(&data, &mut extra);
    out.push_str(&format!(
        "# B-14b — expressions (T-12)\n\n\
         **{cells_ok} of {cells_total} numbers and diagnostics match document 25**, and the \
         picture and export checks at the end {}.\n\n\
         Generated by `tests/b14b_expressions.rs`. Covers R-13, test T-12 and the expression \
         fixtures FX-EXPR-001 to 016 of document 25.\n\n\
         ## How to read this\n\n\
         Each table below is document 25's table, filled in again by the build. The numbers are \
         the build's own; the last column says whether every one of them matches the number \
         document 25 gives, which `tools/expression_reference.py` worked out separately in \
         Python. Numbers are compared to 1e-9, and the wiggle and random tables (002, 003, 004, \
         013 and 014) to 1e-12. A diagnostic column must match exactly, and `none` means the \
         property reported nothing. A failed expression shows the value the property is drawn \
         with instead, which is its keyed value.\n\n\
         Numbers whose last digit or two differ from document 25 are the same number printed \
         differently, and still match; a number that does not match says what it should be.\n",
        if extra_ok {
            "all pass"
        } else {
            "do **not** all pass"
        }
    ));
    out.push_str(&body);
    out.push_str(&extra);
    fs::write(repo("verification/B-14b_expression_table.md"), &out).expect("write the table");
    assert_eq!(
        cells_ok, cells_total,
        "see verification/B-14b_expression_table.md"
    );
    assert!(extra_ok, "see verification/B-14b_expression_table.md");
}
