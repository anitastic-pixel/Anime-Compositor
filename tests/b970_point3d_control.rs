//! D-1090 / B-970: After Effects' 3D Point Control, a point of three numbers for an expression.
//!
//! Writes `verification/B-970_point3d_control_table.md`.
//!
//! The expected values are document 25's FX-EXPR-023, printed by `tools/expression_reference.py`,
//! which shares nothing with `src/expr.rs`; this reads them from the document's table. Each row
//! is one expression in `Fixtures/projects/point3d_control_project.json`.

use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::command::{Command, Document};
use anime_compositor::effects::{Effect, EffectInstance};
use anime_compositor::expr::{evaluate, Target};
use anime_compositor::model::{Id, Layer, Project, Prop, Value};
use anime_compositor::persist;
use serde_json::Value as J;

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn fixture() -> J {
    serde_json::from_str(
        &fs::read_to_string(repo("Fixtures/projects/point3d_control_project.json")).unwrap(),
    )
    .unwrap()
}

fn open(data: &J) -> Project {
    match persist::load_str(&data.to_string()) {
        Ok(l) => l.document.project().clone(),
        Err(d) => panic!("the project did not open: {} {}", d.message, d.detail),
    }
}

/// Python's `repr` for a float, and a whole number without its point, as the reference prints.
fn show(x: f64) -> String {
    if x.fract() == 0.0 && x.abs() < 1e15 {
        format!("{}", x as i64)
    } else {
        format!("{x:?}")
    }
}

fn shown(v: Value) -> String {
    match v {
        Value::Scalar(x) => show(x),
        Value::Vec2(x, y) => format!("({}, {})", show(x), show(y)),
    }
}

/// FX-EXPR-023 from document 25: the layer's name, the property, the expression and the cells.
fn table() -> Vec<(String, String, String, Vec<String>)> {
    let doc = fs::read_to_string(repo("Markdown/25_Test_Fixture_Catalog.md")).unwrap();
    let start = doc.find("FX-EXPR-023: ").expect("FX-EXPR-023 in document 25");
    doc[start..]
        .lines()
        .skip_while(|l| !l.starts_with("| property"))
        .skip(2)
        .take_while(|l| l.starts_with('|'))
        .map(|l| {
            let cells: Vec<String> =
                l.trim().trim_matches('|').split(" | ").map(|c| c.trim().to_string()).collect();
            let (who, prop) = cells[0].split_once(' ').unwrap();
            let text = cells[1].trim_matches('`').to_string();
            (who.to_string(), prop.to_string(), text, cells[2..].to_vec())
        })
        .collect()
}

#[test]
fn fx_expr_023_3d_point_control() {
    let base = fixture();
    let rows = table();
    assert_eq!(rows.len(), 16, "FX-EXPR-023 has sixteen rows");
    let frames = [0, 12, 24, 48];
    let mut out = String::from(
        "# B-970: 3D Point Control (D-1090)\n\n\
         Document 25's FX-EXPR-023, from `tools/expression_reference.py`, against the build \
         (`tests/b970_point3d_control.rs`). Point3D's 3D Point Control runs from (0, 0, 0) at \
         frame 0 to (480, -240, 960) at frame 48. Each row is one expression on the property \
         named; each cell is the value at that frame in the file's units, and the diagnostic if \
         the expression fails. A cell passes when the build's is exactly the expected.\n\n\
         | property | expression | frame | expected | build | result |\n\
         | --- | --- | --- | --- | --- | --- |\n",
    );
    let (mut pass, mut all) = (0, 0);
    for (who, prop_name, text, want) in &rows {
        let id = match who.as_str() {
            "Point3D" => "layer-p3",
            "Controls" => "layer-ctl",
            other => panic!("no layer {other}"),
        };
        let (p, key) = match prop_name.as_str() {
            "rotation" => (Prop::Rotation, "rotation"),
            "position" => (Prop::Position, "position"),
            "depth" => (Prop::Depth, "depth"),
            other => panic!("no property {other}"),
        };
        let mut data = base.clone();
        let layer = data["compositions"][0]["layers"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|l| l["id"] == id)
            .unwrap();
        let record = if key == "depth" { &mut layer["depth"] } else { &mut layer["transform"][key] };
        record["expression"] = serde_json::json!({ "text": text, "enabled": true });
        let project = open(&data);
        let comp = &project.compositions[0];
        let l = comp.layer(&Id::new(id)).unwrap();
        for (f, want) in frames.iter().zip(want) {
            let got = match evaluate(comp, &Target::Layer(Id::new(id)), p, *f) {
                Ok(v) => shown(v),
                Err(e) => {
                    let keyed = match p {
                        Prop::Depth => l.depth.as_ref().unwrap().value_at(*f),
                        _ => l.transform.get(p).unwrap().value_at(*f),
                    };
                    format!("{}, {}", shown(keyed), e.id.as_str())
                }
            };
            let ok = got == *want;
            pass += ok as usize;
            all += 1;
            out += &format!(
                "| {who} {prop_name} | `{text}` | {f} | {want} | {got} | {} |\n",
                if ok { "pass" } else { "FAIL" }
            );
        }
    }
    out += &format!("\n{pass} of {all} cells pass.\n");
    fs::write(repo("verification/B-970_point3d_control_table.md"), &out).unwrap();
    assert_eq!(pass, all, "{out}");
}

#[test]
fn the_point_saves_as_three_numbers_and_reads_back() {
    let project = open(&fixture());
    let text = persist::to_json(&project, &persist::Preserved::none());
    let again: J = serde_json::from_str(&text).unwrap();
    let p3 = again["compositions"][0]["layers"]
        .as_array()
        .unwrap()
        .iter()
        .find(|l| l["id"] == "layer-p3")
        .unwrap();
    let fx = &p3["effects"][0];
    assert_eq!(fx["type_id"], "core.point3d_control");
    assert_eq!(fx["parameters"]["point"]["keyframes"][1]["value"], serde_json::json!([480, -240, 960]));
    assert_eq!(open(&again), project);

    // Without a point it starts at (0, 0, 0); two numbers are refused, never padded.
    let mut d = fixture();
    let p = &mut d["compositions"][0]["layers"][0]["effects"][0]["parameters"];
    p.as_object_mut().unwrap().remove("point");
    let none = open(&d);
    let fx = &none.compositions[0].layer(&Id::new("layer-p3")).unwrap().effects[0];
    assert!(matches!(fx.effect, Effect::Point3DControl { point: [0.0, 0.0, 0.0] }));
    d["compositions"][0]["layers"][0]["effects"][0]["parameters"]["point"] = serde_json::json!([1, 2]);
    let err = persist::load_str(&d.to_string()).err().expect("two numbers refused");
    assert_eq!(err.id.as_str(), "PROJECT_SCHEMA_INVALID");
}

#[test]
fn an_older_project_is_unchanged_and_a_null_may_carry_one() {
    // D-766's file, which has no 3D Point Control, opens and saves as it did.
    let path = repo("Fixtures/projects/expression_controls_project.json");
    let loaded = persist::load(&path).unwrap();
    let saved = persist::to_json(loaded.document.project(), &loaded.preserved);
    assert!(!saved.contains("point3d"));
    let first: J = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(open(&serde_json::from_str(&saved).unwrap()), open(&first));

    // D-1089: a null may carry Expression Controls, this one among them.
    let project = open(&fixture());
    let comp = project.compositions[0].id.clone();
    let mut null = Layer::null(Id::new("layer-null"), "Rig", 1920, 1080, 0, 49);
    null.effects =
        vec![EffectInstance::new(Id::new("fx-n"), Effect::Point3DControl { point: [1.0, 2.0, 3.0] })];
    let mut doc = Document::new(project);
    doc.apply(Command::AddLayer { composition: comp, layer: Box::new(null), index: 0 })
        .expect("a null may carry a 3D Point Control");
}
