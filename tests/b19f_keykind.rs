//! B-19f: separate X and Y for position, and auto, continuous and roving keys, in the core,
//! against D-69.
//!
//! Writes `verification/B-19f_keykind_table.md`.
//!
//! Every expected number is `Fixtures/keykind/expected_keykind.json`, written by
//! `tools/keykind_reference.py` before this code existed and printed in document 25 as FX-SEP,
//! FX-KIND and FX-ROVE. Tolerance 1e-9. Nothing here is a snapshot of a run.
//!
//! Not here: the key menu, the key shapes and the X and Y rows in the window are B-19g.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value as J};

use anime_compositor::command::{Command, Document, Target};
use anime_compositor::expr::evaluate;
use anime_compositor::model::{Expression, Id, Interp, Kind, Prop, Value};
use anime_compositor::persist;

const MAIN: &str = "comp-main";
const LAYER: &str = "bg";

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
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

/// The same thing within the fixture's tolerance. A number is compared as a number.
fn close(a: &J, b: &J, tolerance: f64) -> bool {
    match (a, b) {
        (J::Number(x), J::Number(y)) => {
            (x.as_f64().unwrap() - y.as_f64().unwrap()).abs() <= tolerance
        }
        (J::Array(x), J::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(u, v)| close(u, v, tolerance))
        }
        (J::Object(x), J::Object(y)) => {
            x.len() == y.len()
                && x.iter()
                    .all(|(k, u)| y.get(k).is_some_and(|v| close(u, v, tolerance)))
        }
        _ => a == b,
    }
}

/// The fixture's keys as the file writes a property.
fn to_file(keys: &J) -> J {
    let keys = keys.as_array().unwrap();
    let file: Vec<J> = keys
        .iter()
        .map(|k| {
            let mut k = k.clone();
            if k["interp"].is_array() {
                k["ease"] = k["interp"].clone();
                k["interp"] = json!("ease");
            }
            k
        })
        .collect();
    json!({"base": keys[0]["value"], "keyframes": file})
}

/// The file's property as the fixture writes keys.
fn from_file(property: &J) -> J {
    if property.get("x").is_some() {
        return json!({"x": from_file(&property["x"]), "y": from_file(&property["y"])});
    }
    J::Array(
        property["keyframes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|k| {
                let mut k = k.clone();
                if let Some(ease) = k.as_object_mut().unwrap().remove("ease") {
                    k["interp"] = ease;
                }
                k
            })
            .collect(),
    )
}

/// FX-SEP-001's project with one property of its layer replaced, as text.
fn project_with(prop: &str, property: J) -> String {
    let text = fs::read_to_string(repo("Fixtures/keykind/fx_sep_001.json")).unwrap();
    let mut project: J = serde_json::from_str(&text).unwrap();
    project["compositions"][0]["layers"][0]["transform"][prop] = property;
    project.to_string()
}

/// A document whose layer holds the fixture's `before`, and the property's name in the file.
fn before(case: &J) -> (Document, &'static str) {
    let b = &case["before"];
    let (prop, property) = if b.get("x").is_some() {
        (
            "position",
            json!({"x": to_file(&b["x"]), "y": to_file(&b["y"])}),
        )
    } else if b[0]["value"].is_array() {
        ("position", to_file(b))
    } else {
        ("rotation", to_file(b))
    };
    let loaded = persist::load_str(&project_with(prop, property))
        .unwrap_or_else(|d| panic!("the before state opens: {}", d.message));
    (loaded.document, prop)
}

/// What a save would write for the property, as the fixture writes keys.
fn saved(document: &Document, prop: &str) -> J {
    let text = persist::to_json(document.project(), &persist::Preserved::none());
    let project: J = serde_json::from_str(&text).unwrap();
    from_file(&project["compositions"][0]["layers"][0]["transform"][prop])
}

fn layer() -> Target {
    Target::Layer(Id::new(LAYER))
}

fn set_kind(prop: Prop, frames: &[i32], kind: Kind) -> Command {
    Command::SetKeyKind {
        composition: Id::new(MAIN),
        target: layer(),
        prop,
        frames: frames.to_vec(),
        kind,
    }
}

fn set_roving(frames: &[i32], roving: bool) -> Command {
    Command::SetKeyRoving {
        composition: Id::new(MAIN),
        target: layer(),
        prop: Prop::Position,
        frames: frames.to_vec(),
        roving,
    }
}

fn set_key(prop: Prop, frame: i32, value: Value, interp: Interp) -> Command {
    Command::SetKeyframe {
        composition: Id::new(MAIN),
        target: layer(),
        prop,
        frame,
        value,
        interp,
        spatial: None,
    }
}

fn separate(on: bool) -> Command {
    Command::SeparatePosition {
        composition: Id::new(MAIN),
        layer_id: Id::new(LAYER),
        separate: on,
    }
}

fn move_key(prop: Prop, from_frame: i32, to_frame: i32) -> Command {
    Command::MoveKeyframe {
        composition: Id::new(MAIN),
        target: layer(),
        prop,
        from_frame,
        to_frame,
    }
}

/// The command each fixture's `edit` sentence means.
fn edit(id: &str, prop: Prop) -> Command {
    let ease = Interp::Ease {
        x1: 0.25,
        y1: 0.5,
        x2: 0.75,
        y2: 1.0,
    };
    match id {
        "FX-KIND-001" | "FX-KIND-003" | "FX-KIND-007" => set_kind(prop, &[4], Kind::Auto),
        "FX-KIND-002" => set_kind(Prop::Rotation, &[0, 4], Kind::Auto),
        "FX-KIND-004" => set_key(Prop::Rotation, 12, Value::Scalar(12.0), Interp::Linear),
        "FX-KIND-005" => set_kind(Prop::Rotation, &[4], Kind::Continuous),
        "FX-KIND-006" => set_key(Prop::Rotation, 4, Value::Scalar(8.0), ease),
        "FX-ROVE-001" => set_roving(&[2], true),
        "FX-ROVE-002" => set_roving(&[1, 2], true),
        "FX-ROVE-003" => move_key(Prop::Position, 4, 2),
        "FX-ROVE-004" => set_roving(&[5], true),
        "FX-SEP-003" => separate(true),
        "FX-SEP-004" => separate(false),
        other => panic!("no edit is written here for {other}"),
    }
}

/// Whether opening the project is refused as PROJECT_SCHEMA_INVALID, and what was said.
fn refused(prop: &str, property: J) -> (bool, String) {
    match persist::load_str(&project_with(prop, property)) {
        Ok(_) => (false, "it opened".to_string()),
        Err(d) => (
            d.id.as_str() == "PROJECT_SCHEMA_INVALID",
            d.id.as_str().to_string(),
        ),
    }
}

#[test]
fn b19f_keykind() {
    let expected: J = serde_json::from_str(
        &fs::read_to_string(repo("Fixtures/keykind/expected_keykind.json")).unwrap(),
    )
    .unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();
    let cases = expected["cases"].as_object().unwrap();
    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };

    t.heading("A separated position, frame by frame (FX-SEP-001, 002)");
    for id in ["FX-SEP-001", "FX-SEP-002"] {
        let case = &cases[id];
        let file = format!(
            "Fixtures/keykind/{}.json",
            id.to_lowercase().replace('-', "_")
        );
        let text = fs::read_to_string(repo(&file)).unwrap();
        let loaded = persist::load_str(&text).unwrap_or_else(|d| panic!("{file}: {}", d.message));
        let comp = loaded
            .document
            .project()
            .composition(&Id::new(MAIN))
            .unwrap();
        let mut worst: f64 = 0.0;
        for (frame, want) in case["position"].as_object().unwrap() {
            let got = evaluate(
                comp,
                &anime_compositor::expr::Target::Layer(Id::new(LAYER)),
                Prop::Position,
                frame.parse().unwrap(),
            )
            .unwrap_or_else(|_| panic!("{id} frame {frame} evaluates"));
            let (x, y) = got.as_vec2().unwrap();
            worst = worst
                .max((x - want[0].as_f64().unwrap()).abs())
                .max((y - want[1].as_f64().unwrap()).abs());
        }
        t.row(
            &format!("{id}: {}", case["says"].as_str().unwrap()),
            &format!("largest difference {worst:.1e}"),
            worst <= tolerance,
        );
        let again = persist::to_json(loaded.document.project(), &loaded.preserved);
        let (a, b): (J, J) = (
            serde_json::from_str(&text).unwrap(),
            serde_json::from_str(&again).unwrap(),
        );
        let same = close(
            &a["compositions"][0]["layers"][0]["transform"]["position"],
            &b["compositions"][0]["layers"][0]["transform"]["position"],
            0.0,
        );
        t.row(
            &format!("{id}: saved again, the position is written as it was read"),
            if same {
                "the same x and y"
            } else {
                "different"
            },
            same,
        );
    }

    t.heading("Edits, against the reference's keys (FX-KIND, FX-ROVE, FX-SEP-003, 004)");
    for (id, case) in cases {
        if case.get("edit").is_none() {
            continue;
        }
        let (mut document, prop) = before(case);
        let named = if prop == "position" {
            Prop::Position
        } else {
            Prop::Rotation
        };
        let result = document
            .apply(edit(id, named))
            .map(|_| ())
            .map_err(|d| d.id.as_str().to_string());
        let what = format!(
            "{id}: {}. The edit: {}",
            case["says"].as_str().unwrap(),
            case["edit"].as_str().unwrap()
        );
        if case["after"].is_null() {
            let unchanged = close(&saved(&document, prop), &case["before"], 0.0);
            let ok = result == Err("COMMAND_INVALID_VALUE".to_string()) && unchanged;
            let built = format!(
                "{}, keys {}",
                result.err().unwrap_or_else(|| "accepted".to_string()),
                if unchanged { "unchanged" } else { "changed" }
            );
            t.row(&what, &built, ok);
            continue;
        }
        let ok = result.is_ok() && close(&saved(&document, prop), &case["after"], tolerance);
        let built = match (&result, ok) {
            (Err(e), _) => format!("refused: {e}"),
            (_, true) => "the reference's keys, within 1e-9".to_string(),
            (_, false) => format!("`{}`", saved(&document, prop)),
        };
        t.row(&what, &built, ok);
    }

    t.heading("What a file may not say");
    let key = |frame: i32, value: J, more: J| {
        let mut k = json!({"frame": frame, "value": value, "interp": "linear"});
        for (name, v) in more.as_object().unwrap() {
            k[name] = v.clone();
        }
        k
    };
    let pair = |n: i32| json!([n, 0]);
    let refusals = [
        (
            "roving on a first key",
            "position",
            json!({"base": [0, 0], "keyframes": [
                key(0, pair(0), json!({"roving": true})), key(4, pair(4), json!({}))]}),
        ),
        (
            "roving on a last key",
            "position",
            json!({"base": [0, 0], "keyframes": [
                key(0, pair(0), json!({})), key(4, pair(4), json!({"roving": true}))]}),
        ),
        (
            "roving on a rotation key",
            "rotation",
            json!({"base": 0, "keyframes": [
                key(0, json!(0), json!({})), key(2, json!(1), json!({"roving": true})),
                key(4, json!(2), json!({}))]}),
        ),
        (
            "a kind that is not one of the three",
            "rotation",
            json!({"base": 0, "keyframes": [key(0, json!(0), json!({"kind": "smooth"}))]}),
        ),
        (
            "path handles inside x",
            "position",
            json!({"x": {"base": 0, "keyframes": [key(0, json!(0), json!({"spatial": [0, 0, 1, 1]}))]},
                   "y": {"base": 0, "keyframes": []}}),
        ),
        (
            "a pair inside x",
            "position",
            json!({"x": {"base": [0, 0], "keyframes": []}, "y": {"base": 0, "keyframes": []}}),
        ),
        (
            "roving inside x",
            "position",
            json!({"x": {"base": 0, "keyframes": [
                key(0, json!(0), json!({})), key(2, json!(1), json!({"roving": true})),
                key(4, json!(2), json!({}))]},
                   "y": {"base": 0, "keyframes": []}}),
        ),
        (
            "x and y on an anchor",
            "anchor",
            json!({"x": {"base": 0, "keyframes": []}, "y": {"base": 0, "keyframes": []}}),
        ),
    ];
    for (what, prop, property) in refusals {
        let (ok, said) = refused(prop, property);
        t.row(&format!("A file with {what} is refused"), &said, ok);
    }

    t.heading("Kinds and roving through a save, and undo");
    {
        let case = &cases["FX-ROVE-002"];
        let (mut document, prop) = before(case);
        document.apply(edit("FX-ROVE-002", Prop::Position)).unwrap();
        document
            .apply(set_kind(Prop::Position, &[0], Kind::Continuous))
            .unwrap();
        let text = persist::to_json(document.project(), &persist::Preserved::none());
        let again = persist::load_str(&text).unwrap().document;
        let same = close(&saved(&document, prop), &saved(&again, prop), 0.0)
            && text.contains("\"roving\"")
            && text.contains("\"continuous\"");
        t.row(
            "Roving keys and a continuous key are written, read, and written the same again",
            if same { "the same keys" } else { "different" },
            same,
        );

        // Dragging a roving key in time stops it roving.
        let roving_at = saved(&document, prop)[1]["frame"].as_i64().unwrap() as i32;
        let free = (1..40)
            .find(|f| {
                document
                    .project()
                    .composition(&Id::new(MAIN))
                    .unwrap()
                    .layer(&Id::new(LAYER))
                    .unwrap()
                    .transform
                    .position
                    .keyframe_at(*f)
                    .is_none()
            })
            .unwrap();
        let moved = document
            .apply(move_key(Prop::Position, roving_at, free))
            .is_ok();
        let keys = saved(&document, prop);
        let stopped = keys
            .as_array()
            .unwrap()
            .iter()
            .find(|k| k["frame"] == json!(free))
            .is_some_and(|k| k.get("roving").is_none());
        t.row(
            "A roving key moved in time by hand stops roving",
            &format!(
                "moved from frame {roving_at} to {free}, roving {}",
                if stopped { "gone" } else { "kept" }
            ),
            moved && stopped,
        );
    }
    {
        let case = &cases["FX-KIND-001"];
        let (mut document, prop) = before(case);
        document.apply(edit("FX-KIND-001", Prop::Rotation)).unwrap();
        // Pull the out handle of the auto key at frame 4.
        document
            .apply(set_key(
                Prop::Rotation,
                4,
                case["before"][1]["value"]
                    .as_f64()
                    .map(Value::Scalar)
                    .unwrap(),
                Interp::Ease {
                    x1: 0.5,
                    y1: 0.1,
                    x2: 0.6,
                    y2: 0.9,
                },
            ))
            .unwrap();
        let kind = saved(&document, prop)[1]["kind"].clone();
        t.row(
            "Pulling an auto key's handle makes it continuous",
            &format!("kind {kind}"),
            kind == json!("continuous"),
        );
    }
    {
        let case = &cases["FX-SEP-003"];
        let (mut document, prop) = before(case);
        document.apply(separate(true)).unwrap();
        let label = document
            .undo_labels()
            .last()
            .map(|s| s.to_string())
            .unwrap_or_default();
        document.undo();
        let back = close(&saved(&document, prop), &case["before"], 0.0);
        t.row(
            "Separate is one entry to undo, and undo gives back the path handles",
            &format!(
                "\"{label}\", then {}",
                if back {
                    "the keys as before"
                } else {
                    "different keys"
                }
            ),
            back,
        );
        let again = document
            .apply(separate(false))
            .map(|_| ())
            .map_err(|d| d.id.as_str().to_string());
        t.row(
            "Joining a position that is not separated is refused",
            &format!("{again:?}"),
            again == Err("COMMAND_INVALID_VALUE".to_string()),
        );
    }

    t.heading("A separated position under commands");
    {
        let case = &cases["FX-SEP-003"];
        let (mut document, _) = before(case);
        let expression = |prop: Prop, text: Option<&str>| Command::SetExpression {
            composition: Id::new(MAIN),
            target: layer(),
            prop,
            expression: text.map(|t| Expression {
                text: t.to_string(),
                enabled: true,
            }),
        };
        document
            .apply(expression(Prop::Position, Some("value")))
            .unwrap();
        let with = document
            .apply(separate(true))
            .map(|_| ())
            .map_err(|d| d.id.as_str().to_string());
        t.row(
            "Separate is refused while the position has an expression",
            &format!("{with:?}"),
            with == Err("COMMAND_INVALID_VALUE".to_string()),
        );
        document.apply(expression(Prop::Position, None)).unwrap();
        document.apply(separate(true)).unwrap();

        let whole = document
            .apply(set_key(
                Prop::Position,
                2,
                Value::Vec2(1.0, 1.0),
                Interp::Linear,
            ))
            .map(|_| ())
            .map_err(|d| d.id.as_str().to_string());
        t.row(
            "A key on `position` is refused while it is separated",
            &format!("{whole:?}"),
            whole == Err("COMMAND_INVALID_VALUE".to_string()),
        );

        let half = document
            .apply(set_key(
                Prop::PositionX,
                2,
                Value::Scalar(100.0),
                Interp::Linear,
            ))
            .is_ok();
        let comp = document.project().composition(&Id::new(MAIN)).unwrap();
        let at = evaluate(
            comp,
            &anime_compositor::expr::Target::Layer(Id::new(LAYER)),
            Prop::Position,
            2,
        )
        .ok()
        .and_then(|v| v.as_vec2());
        let y_before = cases["FX-SEP-001"]["position"]["2"][1].as_f64().unwrap();
        t.row(
            "A key on `position_x` at frame 2 moves X there and leaves Y alone",
            &format!("{at:?}"),
            half && at.is_some_and(|(x, y)| x == 100.0 && (y - y_before).abs() <= tolerance),
        );

        document
            .apply(expression(Prop::PositionY, Some("value + 10")))
            .unwrap();
        let comp = document.project().composition(&Id::new(MAIN)).unwrap();
        let at = evaluate(
            comp,
            &anime_compositor::expr::Target::Layer(Id::new(LAYER)),
            Prop::Position,
            2,
        )
        .ok()
        .and_then(|v| v.as_vec2());
        t.row(
            "An expression on `position_y`, value + 10, is in the position",
            &format!("{at:?}"),
            at.is_some_and(|(x, y)| x == 100.0 && (y - y_before - 10.0).abs() <= tolerance),
        );
        let join = document
            .apply(separate(false))
            .map(|_| ())
            .map_err(|d| d.id.as_str().to_string());
        t.row(
            "Joining is refused while X or Y has an expression",
            &format!("{join:?}"),
            join == Err("COMMAND_INVALID_VALUE".to_string()),
        );
        let roving = document
            .apply(set_roving(&[2], true))
            .map(|_| ())
            .map_err(|d| d.id.as_str().to_string());
        t.row(
            "Roving is refused on a separated position",
            &format!("{roving:?}"),
            roving == Err("COMMAND_INVALID_VALUE".to_string()),
        );
    }

    let report = format!(
        "# B-19f: separate X and Y, and auto, continuous and roving keys, in the core (D-69)\n\n\
         Written by `cargo test --test b19f_keykind`. Every expected number is from\n\
         `Fixtures/keykind/expected_keykind.json`, which `tools/keykind_reference.py` wrote before\n\
         this code existed. Tolerance 1e-9.\n\n\
         **{} of {} checks match.**{}\n{}",
        t.passed,
        t.checks,
        " FX-KIND-005's `before` was corrected in the fixture on 2026-09-19 by the owner's word; \
         until then this table was 36 of 37.",
        t.out
    );
    fs::write(repo("verification/B-19f_keykind_table.md"), report).unwrap();
    assert_eq!(
        t.passed,
        t.checks,
        "see verification/B-19f_keykind_table.md"
    );
}
