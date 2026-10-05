//! B-176: D-291, an expression on an effect's number setting.
//!
//! P-26 found the tutorials' `wiggle` and `time*` on effect settings refused on load. Each check
//! here compares the expression's picture with the picture of the plain number it should come to,
//! drawn by the code that existed before D-291, so no expected pixel comes from the new code.

use std::path::PathBuf;
use std::sync::atomic::AtomicBool;

use serde_json::{json, Value as J};

use anime_compositor::command::Command;
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::EffectKey;
use anime_compositor::export::{export_sequence, ExportRequest, ExportStatus, MissingSource, OutputFormat};
use anime_compositor::model::{Expression, Id, Interp};
use anime_compositor::{persist, OutputAlpha, OutputDepth};

const MAIN: &str = "comp-main";

fn dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Fixtures/effect_mix")
}

/// The bands fixture with one Gaussian Blur whose `sigma_px` is `sigma`, as the file writes it.
fn with_sigma(sigma: J) -> J {
    let mut p: J =
        serde_json::from_str(&std::fs::read_to_string(dir().join("fx_mix_001.json")).unwrap()).unwrap();
    p["compositions"][0]["layers"][0]["effects"] = json!([{
        "instance_id": "fx-0-0", "type_id": "core.gaussian_blur", "enabled": true,
        "parameters": {"sigma_px": sigma, "edges": "transparent"}
    }]);
    p
}

fn expression(base: f64, keys: J, text: &str) -> J {
    json!({"base": base, "keyframes": keys, "expression": {"text": text, "enabled": true}})
}

/// The frame's samples and what it warned of.
fn draw(project: &J, frame: i32) -> (Vec<f32>, Vec<String>) {
    let loaded = persist::load_str(&project.to_string()).expect("the project reads");
    let mut log = FrameLog::new(3);
    let buffer = render_frame(loaded.document.project(), &Id::new(MAIN), frame, &dir(), 64, &mut log)
        .expect("the frame draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (buffer.data().to_vec(), said)
}

#[test]
fn an_expression_draws_the_number_it_comes_to() {
    let (two, _) = draw(&with_sigma(json!(2.0)), 2);
    let (three, _) = draw(&with_sigma(json!(3.0)), 3);
    let (five, _) = draw(&with_sigma(json!(5.0)), 2);
    assert_ne!(two, three, "the blur must change with sigma for these checks to mean anything");
    // `value` is the constant when there are no keys.
    let (got, said) = draw(&with_sigma(expression(1.0, json!([]), "value*2")), 2);
    assert!(said.is_empty(), "{said:?}");
    assert_eq!(got, two, "value*2 on 1 draws as sigma 2");
    // `time` is seconds: frame 3 at 24 fps is 0.125, so time*24 is 3.
    let (got, _) = draw(&with_sigma(expression(9.0, json!([]), "time*24")), 3);
    assert_eq!(got, three, "time*24 at frame 3 draws as sigma 3");
    // With keys, `value` is the keyed value: 2 at 0 to 6 at 4 is 4 at frame 2, plus 1 is 5.
    let keys = json!([{"frame": 0, "value": 2.0, "interp": "linear"},
                      {"frame": 4, "value": 6.0, "interp": "linear"}]);
    let (got, _) = draw(&with_sigma(expression(9.0, keys, "value+1")), 2);
    assert_eq!(got, five, "value+1 on keys at 4 draws as sigma 5");
    // Past the setting's range it is held at the end, as a typed number is: 0 to 500.
    let (zero, _) = draw(&with_sigma(json!(0.0)), 2);
    let (got, _) = draw(&with_sigma(expression(1.0, json!([]), "-40")), 2);
    assert_eq!(got, zero, "-40 is held at sigma 0");
    // Switched off, the setting is its own value.
    let mut off = with_sigma(expression(2.0, json!([]), "value*100"));
    off["compositions"][0]["layers"][0]["effects"][0]["parameters"]["sigma_px"]["expression"]["enabled"] =
        J::from(false);
    assert_eq!(draw(&off, 2).0, two, "a switched-off expression draws the base");
}

#[test]
fn the_file_keeps_it_and_refuses_one_on_a_colour() {
    let project = with_sigma(expression(1.0, json!([]), "wiggle(2, 1)"));
    let loaded = persist::load_str(&project.to_string()).expect("the project reads");
    let saved = persist::to_json(loaded.document.project(), &loaded.preserved);
    let again: J = serde_json::from_str(&saved).unwrap();
    assert_eq!(
        again["compositions"][0]["layers"][0]["effects"][0]["parameters"]["sigma_px"]["expression"],
        json!({"text": "wiggle(2, 1)", "enabled": true}),
    );
    let reloaded = persist::load_str(&saved).expect("the saved project reads");
    assert_eq!(persist::to_json(reloaded.document.project(), &reloaded.preserved), saved);

    let mut tint = with_sigma(json!(1.0));
    tint["compositions"][0]["layers"][0]["effects"] = json!([{
        "instance_id": "fx-0-0", "type_id": "core.tint", "enabled": true,
        "parameters": {"color": {"base": [0, 0, 1], "keyframes": [],
            "expression": {"text": "[1, 0, 0]", "enabled": true}}, "amount": 1.0}
    }]);
    let refused = persist::load_str(&tint.to_string()).err().expect("a colour's expression is refused");
    assert!(refused.detail.contains("a colour or a point takes keys only"), "{}", refused.detail);
}

#[test]
fn a_broken_expression_is_said_and_blocks_export() {
    let project = with_sigma(expression(2.0, json!([]), "nonsense("));
    let (got, said) = draw(&project, 2);
    let (two, _) = draw(&with_sigma(json!(2.0)), 2);
    assert_eq!(got, two, "the frame is drawn at the setting's own value");
    assert!(said.iter().any(|s| s.contains("art's Gaussian Blur sigma_px does not work at frame 2")), "{said:?}");

    let loaded = persist::load_str(&project.to_string()).unwrap();
    let out = std::env::temp_dir().join(format!("b176-{}", std::process::id()));
    std::fs::create_dir_all(&out).unwrap();
    let request = ExportRequest {
        composition: Id::new(MAIN),
        first_frame: 0,
        last_frame: 2,
        output_dir: out.clone(),
        naming: "f_%04d.png".to_string(),
        depth: OutputDepth::Eight,
        alpha: OutputAlpha::Straight,
        tile_size: 64,
        missing: MissingSource::Block,
        format: OutputFormat::Png,
        choices: Default::default(),
    };
    let report = export_sequence(loaded.document.project(), &dir(), &request, &AtomicBool::new(false));
    assert_eq!(report.status, ExportStatus::Blocked);
    assert_eq!(std::fs::read_dir(&out).unwrap().count(), 0);
    let first = &report.diagnostics[0];
    assert!(first.message.contains("art's Gaussian Blur sigma_px does not work at frame 0"), "{}", first.message);
    let _ = std::fs::remove_dir_all(&out);
}

#[test]
fn keys_and_expression_keep_each_other() {
    let loaded = persist::load_str(&with_sigma(json!(2.0)).to_string()).unwrap();
    let mut document = loaded.document;
    let (comp, layer, fx) = (Id::new(MAIN), Id::new("art"), Id::new("fx-0-0"));
    let set = |text: Option<&str>, setting: &str| Command::SetEffectExpression {
        composition: comp.clone(),
        layer_id: layer.clone(),
        instance_id: fx.clone(),
        setting: setting.to_string(),
        expression: text.map(|t| Expression { text: t.to_string(), enabled: true }),
    };
    let keys = |keys: Vec<EffectKey>| Command::SetEffectKeys {
        composition: comp.clone(),
        layer_id: layer.clone(),
        instance_id: fx.clone(),
        setting: "sigma_px".to_string(),
        keys,
    };
    let instance = |d: &anime_compositor::command::Document| {
        d.project().composition(&comp).unwrap().layer(&layer).unwrap().effects[0].clone()
    };
    let key = |frame, v| EffectKey { frame, value: vec![v], interp: Interp::Linear };

    document.apply(keys(vec![key(0, 1.0), key(4, 3.0)])).unwrap();
    document.apply(set(Some("value*2"), "sigma_px")).unwrap();
    assert_eq!(instance(&document).expression("sigma_px").map(|e| e.text.as_str()), Some("value*2"));
    assert_eq!(instance(&document).tracks["sigma_px"][0].keyframes().len(), 2, "the keys are kept");
    document.apply(keys(vec![key(1, 5.0)])).unwrap();
    assert!(instance(&document).expression("sigma_px").is_some(), "new keys keep the expression");
    document.apply(keys(vec![])).unwrap();
    assert!(instance(&document).expression("sigma_px").is_some(), "no keys keeps the expression");
    document.apply(set(None, "sigma_px")).unwrap();
    assert!(instance(&document).tracks.is_empty(), "no keys and no expression is the plain setting");

    assert!(document.apply(set(Some("1"), "mix")).is_err(), "the Mix takes keys only");
    assert!(document.apply(set(Some("1"), "edges")).is_err(), "a choice is not a number setting");
}
