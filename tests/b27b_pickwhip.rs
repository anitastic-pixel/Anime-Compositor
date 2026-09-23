//! B-27b: the pick whip in the core, against D-83.
//!
//! Writes `verification/B-27b_pickwhip_table.md`.
//!
//! # Where the expected values come from
//!
//! `Fixtures/pickwhip/expected_pickwhip.json`, written by `tools/pickwhip_reference.py`, which
//! writes each drop's text by D-83's rule and evaluates it with `tools/expression_reference.py`,
//! D-59's second implementation. Document 25 prints the same text and numbers as FX-WHIP-001 to
//! 020. The tolerance is the catalogue's, 1e-6. Nothing here is a snapshot of a run.
//!
//! # What is deliberately not here
//!
//! The window: the spirals, the line that follows the hand and the lit row are B-27c.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as J;

use anime_compositor::command::{self, Command, Document};
use anime_compositor::expr::{evaluate, link, Target};
use anime_compositor::model::{Expression, Id, Prop, Value};
use anime_compositor::persist;

const COMP: &str = "comp-main";

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
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

fn pair(j: &J) -> (Target, Prop) {
    (target(j[0].as_str().unwrap()), prop(j[1].as_str().unwrap()))
}

/// The one command a drop is: `property.set_expression` with the link's text.
fn drop_command(to: &Target, to_prop: Prop, text: &str) -> Command {
    Command::SetExpression {
        composition: Id::new(COMP),
        target: match to {
            Target::Camera => command::Target::Camera,
            Target::Layer(id) => command::Target::Layer(id.clone()),
        },
        prop: to_prop,
        expression: Some(Expression {
            text: text.to_string(),
            enabled: true,
        }),
    }
}

/// A value in the fixture's units: the model keeps scale as a factor (D-22).
fn file_units(prop: Prop, v: Value) -> Vec<f64> {
    let k = if prop == Prop::Scale { 100.0 } else { 1.0 };
    match v {
        Value::Vec2(x, y) => vec![x * k, y * k],
        Value::Scalar(x) => vec![x * k],
    }
}

fn numbers(j: &J) -> Vec<f64> {
    match j.as_array() {
        Some(a) => a.iter().map(|v| v.as_f64().unwrap()).collect(),
        None => vec![j.as_f64().unwrap()],
    }
}

fn show(v: &[f64]) -> String {
    let each: Vec<String> = v.iter().map(|x| format!("{x}")).collect();
    if each.len() == 1 {
        each[0].clone()
    } else {
        format!("({})", each.join(", "))
    }
}

struct Table {
    out: String,
    checks: usize,
    passed: usize,
}

impl Table {
    fn row(&mut self, what: &str, built: &str, ok: bool) {
        self.checks += 1;
        self.passed += ok as usize;
        let verdict = if ok { "yes" } else { "**NO**" };
        self.out
            .push_str(&format!("| {what} | {built} | {verdict} |\n"));
    }

    fn heading(&mut self, text: &str) {
        self.out.push_str(&format!(
            "\n## {text}\n\n| Check | The build's answer | Matches |\n| --- | --- | --- |\n"
        ));
    }
}

#[test]
fn b27b_pickwhip() {
    let root = repo("Fixtures/pickwhip");
    let expected: J =
        serde_json::from_str(&fs::read_to_string(root.join("expected_pickwhip.json")).unwrap())
            .unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();
    let open = || {
        persist::load(&root.join(expected["project"].as_str().unwrap()))
            .unwrap_or_else(|d| panic!("the project opens: {} {}", d.message, d.detail))
    };
    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };

    for (fx, case) in expected["cases"].as_object().unwrap() {
        t.heading(&format!("{fx}: {}", case["says"].as_str().unwrap()));
        let mut document = open().document;
        let drops = case["drops"].as_array().unwrap();
        for drop in drops {
            let (from, from_prop) = pair(&drop["source"]);
            let (to, to_prop) = pair(&drop["destination"]);
            let want = drop["text"].as_str().unwrap();
            let comp = document.project().composition(&Id::new(COMP)).unwrap();
            let got = link(comp, &from, from_prop, &to, to_prop);
            let text = got.clone().unwrap_or_default();
            t.row(
                &format!("The text written, which is `{}`", want.replace('\n', " ")),
                &format!("`{}`", got.unwrap_or_else(|e| e).replace('\n', " ")),
                text == want,
            );
            let depth = document.undo_depth();
            let applied = document.apply(drop_command(&to, to_prop, &text)).is_ok();
            t.row(
                "The drop is one step of history",
                &format!("{} step(s)", document.undo_depth() - depth),
                applied && document.undo_depth() == depth + 1,
            );
        }
        let (to, to_prop) = pair(&drops[0]["destination"]);
        let comp = document.project().composition(&Id::new(COMP)).unwrap();
        for (frame, v) in case["frames"].as_object().unwrap() {
            let f: i32 = frame.parse().unwrap();
            let want = numbers(&v["linked"]);
            let got = evaluate(comp, &to, to_prop, f);
            let error = v["error"].as_str();
            let (built, ok) = match (&got, error) {
                (Ok(value), None) => {
                    let value = file_units(to_prop, *value);
                    let close = value.len() == want.len()
                        && value
                            .iter()
                            .zip(&want)
                            .all(|(a, b)| (a - b).abs() <= tolerance);
                    (show(&value), close)
                }
                (Err(e), Some(code)) => (e.id.as_str().to_string(), e.id.as_str() == code),
                (Ok(value), Some(_)) => (show(&file_units(to_prop, *value)), false),
                (Err(e), None) => (e.id.as_str().to_string(), false),
            };
            let what = match error {
                Some(code) => format!("Frame {f}: {code}, falling back to the keys"),
                None => format!("Frame {f}: the linked value, {}", show(&want)),
            };
            t.row(&what, &built, ok);
        }
        // The drop comes back off Undo, whatever was there before.
        let before = open().document;
        while document.undo_depth() > 0 {
            document.undo();
        }
        t.row(
            "Undo brings back the property as it was",
            if document.project() == before.project() {
                "as it was"
            } else {
                "changed"
            },
            document.project() == before.project(),
        );
    }

    t.heading("Where nothing is written");
    for (fx, case) in expected["nothing"].as_object().unwrap() {
        let document = open().document;
        let comp = document.project().composition(&Id::new(COMP)).unwrap();
        let (from, from_prop) = pair(&case["source"]);
        let (to, to_prop) = pair(&case["destination"]);
        let got = link(comp, &from, from_prop, &to, to_prop);
        t.row(
            &format!("{fx}: {}", case["says"].as_str().unwrap()),
            &got.clone().unwrap_or_else(|e| format!("refused: \"{e}\"")),
            got.is_err(),
        );
    }

    t.heading("What D-83 says a link keeps");
    let loaded = open();
    let mut document = loaded.document;
    let comp_id = Id::new(COMP);
    let (null, target_layer) = (target("null-1"), target("layer-target"));
    let text = link(
        document.project().composition(&comp_id).unwrap(),
        &null,
        Prop::Position,
        &target_layer,
        Prop::Position,
    )
    .unwrap();
    document
        .apply(drop_command(&target_layer, Prop::Position, &text))
        .unwrap();
    document
        .apply(Command::RenameLayer {
            composition: comp_id.clone(),
            layer_id: Id::new("null-1"),
            name: "Renamed".to_string(),
        })
        .unwrap();
    let at = |document: &Document| {
        let comp = document.project().composition(&comp_id).unwrap();
        (
            evaluate(comp, &target_layer, Prop::Position, 24).ok(),
            evaluate(comp, &null, Prop::Position, 24).ok(),
        )
    };
    let (linked, source) = at(&document);
    t.row(
        "Renaming Null 1 leaves the link reading it, because it names the id",
        &format!(
            "{} against {}",
            linked.map_or("no value".into(), |v| show(&file_units(Prop::Position, v))),
            source.map_or("no value".into(), |v| show(&file_units(Prop::Position, v)))
        ),
        linked.is_some() && linked == source,
    );
    let json = persist::to_json(document.project(), &loaded.preserved);
    let reopened = persist::load_str(&json).map(|l| l.document);
    let same = reopened.as_ref().is_ok_and(|d| at(d) == (linked, source));
    t.row(
        "Saved and reopened, the link still reads Null 1",
        if same { "the same value" } else { "different" },
        same,
    );

    let text = format!(
        "# B-27b: the pick whip in the core\n\nGenerated by `tests/b27b_pickwhip.rs` against \
         `Fixtures/pickwhip/expected_pickwhip.json` (D-83, document 25's FX-WHIP-001 to 020). \
         Each drop is `expr::link`'s text set as the property's expression, one step of \
         history, then evaluated on frames 0, 12, 24, 36 and 48. Values are in the file's \
         units, so an opacity reads 0 to 1 and a scale in percent. Tolerance {tolerance}.\n\n\
         **{} of {} checks match.**\n{}",
        t.passed, t.checks, t.out
    );
    fs::write(repo("verification/B-27b_pickwhip_table.md"), text).unwrap();
    assert_eq!(
        t.passed, t.checks,
        "see verification/B-27b_pickwhip_table.md"
    );
}
