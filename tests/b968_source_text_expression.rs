//! D-1088 / B-968: Source Text expressions, a text layer's words worked out each frame.
//!
//! Writes `verification/B-968_source_text_expression_table.md`.
//!
//! The expected words are document 25's FX-EXPR-022, printed by `tools/expression_reference.py`,
//! which shares nothing with `src/expr.rs`; this reads them from the document's table. Each row
//! is the Source Text expression of Words in
//! `Fixtures/projects/source_text_expression_project.json`, at frames 0, 7, 12 and 48.

use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::expr::source_text;
use anime_compositor::model::{Id, Project};
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

fn fixture() -> J {
    serde_json::from_str(
        &fs::read_to_string(repo("Fixtures/projects/source_text_expression_project.json"))
            .unwrap(),
    )
    .unwrap()
}

fn words(data: &mut J) -> &mut J {
    data["compositions"][0]["layers"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|l| l["id"] == "layer-words")
        .expect("the fixture's text layer")
}

/// FX-EXPR-022 from document 25: the expression (`None` for the tutorial's switched off) and the
/// four cells as written.
fn table() -> Vec<(Option<String>, Vec<String>)> {
    let doc = fs::read_to_string(repo("Markdown/25_Test_Fixture_Catalog.md")).unwrap();
    let start = doc.find("FX-EXPR-022: ").expect("FX-EXPR-022 in document 25");
    doc[start..]
        .lines()
        .skip_while(|l| !l.starts_with("| Words"))
        .skip(2)
        .take_while(|l| l.starts_with('|'))
        .map(|l| {
            let cells: Vec<String> = l
                .trim()
                .trim_matches('|')
                .split(" | ")
                .map(|c| c.trim().to_string())
                .collect();
            let text = cells[0].strip_prefix('`').map(|c| c.trim_end_matches('`').to_string());
            (text, cells[1..].to_vec())
        })
        .collect()
}

#[test]
fn fx_expr_022_source_text_expressions() {
    let base = fixture();
    let rows = table();
    assert_eq!(rows.len(), 17, "FX-EXPR-022 has seventeen rows");
    let frames = [0, 7, 12, 48];
    let layer = Id::new("layer-words");
    let mut out = String::from(
        "# B-968: Source Text expressions (D-1088)\n\n\
         Document 25's FX-EXPR-022, from `tools/expression_reference.py`, against the build \
         (`tests/b968_source_text_expression.rs`). Each row is the Source Text expression of the \
         text layer Words in `Fixtures/projects/source_text_expression_project.json`, whose own \
         words are \"25%\"; each cell is the words drawn at that frame, and the diagnostic if the \
         expression fails. A cell passes when the build's is exactly the expected.\n\n\
         | Words Source Text expression | frame | expected | build | result |\n\
         | --- | --- | --- | --- | --- |\n",
    );
    let (mut pass, mut all) = (0, 0);
    for (text, want) in &rows {
        let mut data = base.clone();
        let tutorial = words(&mut data)["source_text"]["expression"]["text"].clone();
        words(&mut data)["source_text"]["expression"] = match text {
            Some(t) => serde_json::json!({ "text": t, "enabled": true }),
            None => serde_json::json!({ "text": tutorial, "enabled": false }),
        };
        let project = open(&data);
        let comp = &project.compositions[0];
        let own = comp.layer(&layer).unwrap().text.as_ref().unwrap().text.clone();
        for (f, want) in frames.iter().zip(want) {
            let got = match source_text(comp, &layer, *f) {
                None => format!("\"{own}\""),
                Some(Ok(s)) => format!("\"{s}\""),
                Some(Err(e)) => format!("\"{own}\", {}", e.id.as_str()),
            };
            let ok = got == *want;
            pass += ok as usize;
            all += 1;
            out += &format!(
                "| {} | {f} | {want} | {got} | {} |\n",
                text.as_ref()
                    .map_or("the tutorial's, switched off".to_string(), |t| format!("`{t}`")),
                if ok { "pass" } else { "FAIL" }
            );
        }
    }
    out += &format!("\n{pass} of {all} cells pass.\n");
    fs::write(repo("verification/B-968_source_text_expression_table.md"), &out).unwrap();
    assert_eq!(pass, all, "{out}");
}

#[test]
fn source_text_expression_is_saved_and_read_back() {
    let project = open(&fixture());
    let text = persist::to_json(&project, &persist::Preserved::none());
    let again: J = serde_json::from_str(&text).unwrap();
    let layer = again["compositions"][0]["layers"]
        .as_array()
        .unwrap()
        .iter()
        .find(|l| l["id"] == "layer-words")
        .unwrap();
    assert_eq!(
        layer["source_text"]["expression"],
        serde_json::json!({
            "text": "thisComp.layer(\"Controls\").effect(\"Slider Control\")(\"Slider\")",
            "enabled": true
        })
    );
    assert_eq!(open(&again), project);
}

#[test]
fn an_older_text_layer_loads_without_one_and_saves_as_it_was() {
    let file = fs::read_to_string(repo("Fixtures/text_outlines/fx_txo_020.json")).unwrap();
    let project = open(&serde_json::from_str(&file).unwrap());
    let comp = &project.compositions[0];
    let art = Id::new("art");
    assert_eq!(comp.layer(&art).unwrap().text.as_ref().unwrap().expression, None);
    assert!(source_text(comp, &art, 5).is_none());
    let again: J = serde_json::from_str(&persist::to_json(&project, &persist::Preserved::none()))
        .unwrap();
    assert!(again["compositions"][0]["layers"][0]["source_text"]
        .get("expression")
        .is_none());
}

#[test]
fn a_source_text_expression_that_is_not_one_refuses_the_file() {
    let mut data = fixture();
    words(&mut data)["source_text"]["expression"] = serde_json::json!({ "text": "1" });
    let refused = persist::load_str(&data.to_string()).err().expect("refused");
    assert_eq!(refused.id.as_str(), "PROJECT_SCHEMA_INVALID");
    assert!(refused.message.contains("source_text/expression") || refused.detail.contains("source_text/expression"),
        "{} {}", refused.message, refused.detail);
}
