//! D-1033 / B-911: `thisComp.layer("Name")`, `thisComp.layer(n)` and a name holding a layer.
//!
//! Writes `verification/B-911_expression_layer_names_table.md`.
//!
//! The expected values are document 25's FX-EXPR-021, printed by
//! `tools/expression_reference.py`, which shares nothing with `src/expr.rs`. Each row is Spin's
//! rotation expression in `Fixtures/projects/expression_project.json`, at frame 12.

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

fn layer<'a>(data: &'a mut J, id: &str) -> &'a mut J {
    data["compositions"][0]["layers"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|l| l["id"] == id)
        .expect("a fixture layer")
}

fn show(x: f64) -> String {
    if x.fract() == 0.0 && x.abs() < 1e15 {
        format!("{}", x as i64)
    } else {
        format!("{x:?}")
    }
}

/// Spin's rotation at frame 12 with `text` as its expression, and the identifier (`none` for
/// none). A failure is drawn at its keys, so that is the value shown for one.
fn outcome(base: &J, text: &str) -> (f64, String) {
    let mut data = base.clone();
    layer(&mut data, "layer-spin")["transform"]["rotation"]["expression"] =
        serde_json::json!({ "text": text, "enabled": true });
    let project = open(&data);
    let comp = &project.compositions[0];
    let spin = Id::new("layer-spin");
    let (value, code) = match evaluate(comp, &Target::Layer(spin.clone()), Prop::Rotation, 12) {
        Ok(v) => (v, "none".to_string()),
        Err(e) => {
            let at = comp.layer(&spin).unwrap().transform.get(Prop::Rotation).unwrap();
            (at.value_at(12), e.id.as_str().to_string())
        }
    };
    let Value::Scalar(x) = value else {
        panic!("rotation is one number")
    };
    (x, code)
}

#[test]
fn fx_expr_021_layer_by_name_number_and_held() {
    let base: J = serde_json::from_str(
        &fs::read_to_string(repo("Fixtures/projects/expression_project.json")).unwrap(),
    )
    .unwrap();
    let mut twins = base.clone();
    layer(&mut twins, "layer-fade")["name"] = J::from("Lead");
    let missing = "EXPRESSION_REFERENCE_MISSING";
    let wrong = "EXPRESSION_TYPE";
    // FX-EXPR-021, row by row: the expression, the file, a note, the value and the identifier.
    let rows: Vec<(&str, &J, &str, f64, &str)> = vec![
        (r#"thisComp.layer("Lead").position[0]"#, &base, "", 480.0, "none"),
        (r#"thisComp.layer("layer-lead").position[0]"#, &base, "", 480.0, "none"),
        ("thisComp.layer(5).position[0]", &base, "", 480.0, "none"),
        ("thisComp.layer(1).opacity", &base, "", 100.0, "none"),
        ("thisComp.layer(9).position[1]", &base, "", 540.0, "none"),
        ("var L = thisComp.layer(\"Lead\");\nL.position[0] / 10", &base, "", 48.0, "none"),
        (
            "target = thisComp.layer(\"Lead\");\n\
             (target.position[0] - (thisComp.width / 2)) / thisComp.width",
            &base,
            "",
            -0.25,
            "none",
        ),
        (
            "let L = thisComp.layer(5); L.transform.position.valueAtTime(0)[0]",
            &base,
            "",
            0.0,
            "none",
        ),
        ("me = thisLayer; me.position[0]", &base, "", 960.0, "none"),
        ("c = thisComp.activeCamera; c.zoom / 100", &base, "", 19.2, "none"),
        (
            r#"thisComp.layer("Lead").position[0]"#,
            &twins,
            " (Fade renamed Lead, above the real Lead)",
            960.0,
            "none",
        ),
        (r#"thisComp.layer("Nobody").position[0]"#, &base, "", 0.0, missing),
        ("thisComp.layer(0).position[0]", &base, "", 0.0, missing),
        ("thisComp.layer(10).position[0]", &base, "", 0.0, missing),
        ("thisComp.layer(2.5).position[0]", &base, "", 0.0, missing),
        ("thisComp.layer([1, 2]).position[0]", &base, "", 0.0, wrong),
        ("L = thisComp.layer(5); L", &base, "", 0.0, wrong),
        ("L = thisComp.layer(5); L + 1", &base, "", 0.0, wrong),
    ];
    let mut out = String::from(
        "# B-911: a layer by its name or number, and a name holding a layer (D-1033)\n\n\
         Document 25's FX-EXPR-021, from `tools/expression_reference.py`, against the build \
         (`tests/b911_expression_layer_names.rs`). Each row is Spin's rotation expression in \
         `Fixtures/projects/expression_project.json` at frame 12. Tolerance 1e-9.\n\n\
         | Spin rotation expression | expected | build | diagnostic expected | build | result |\n\
         | --- | --- | --- | --- | --- | --- |\n",
    );
    let mut pass = 0;
    for (text, data, note, want, want_code) in &rows {
        let (got, code) = outcome(data, text);
        let ok = (got - want).abs() <= 1e-9 && code == *want_code;
        pass += ok as usize;
        out += &format!(
            "| `{}`{note} | {} | {} | {want_code} | {code} | {} |\n",
            text.replace('\n', "\\n"),
            show(*want),
            show(got),
            if ok { "pass" } else { "FAIL" }
        );
    }
    out += &format!("\n{pass} of {} rows pass.\n", rows.len());
    fs::write(
        repo("verification/B-911_expression_layer_names_table.md"),
        &out,
    )
    .unwrap();
    assert_eq!(pass, rows.len(), "{out}");
}
