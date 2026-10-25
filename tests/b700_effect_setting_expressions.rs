//! B-700 (D-820): an expression runs on an effect's colour and point, and reads any effect's
//! setting, on the same layer or another.
//!
//! Each check draws the file with the expression and the same file with the value the
//! expression should give written in as a constant, worked out here from After Effects' rules
//! (a colour as four numbers, a point in the layer's pixels, an sRGB colour word from the linear
//! numbers), and asks for the same samples. A third drawing, the file's own constant, must
//! differ, so a check cannot pass by the expression doing nothing.

mod effect_table;

use std::path::PathBuf;

use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::Id;
use anime_compositor::persist;

use effect_table::{repo, same_json, Table};

const MAIN: &str = "comp-main";

fn dir() -> PathBuf {
    repo("Fixtures/effect_mix")
}

/// The bands fixture (16 by 10) with `effects` on its one layer, `art`, and a second layer,
/// `dot`, at (4, 2) with `dot_effects`, drawn under it at a quarter of its opacity.
fn with(effects: J, dot_effects: J) -> J {
    let mut p: J =
        serde_json::from_str(&std::fs::read_to_string(dir().join("fx_mix_001.json")).unwrap())
            .unwrap();
    let comp = &mut p["compositions"][0];
    let mut dot = comp["layers"][0].clone();
    dot["id"] = json!("dot");
    dot["name"] = json!("dot");
    dot["transform"]["position"]["base"] = json!([4, 2]);
    dot["transform"]["opacity"]["base"] = json!(0.25);
    dot["effects"] = dot_effects;
    comp["layers"][0]["effects"] = effects;
    comp["layers"].as_array_mut().unwrap().push(dot);
    comp["layer_order"] = json!(["dot", "art"]);
    p
}

fn held(base: J, text: &str) -> J {
    json!({"base": base, "keyframes": [], "expression": {"text": text, "enabled": true}})
}

fn fx(id: &str, type_id: &str, parameters: J) -> J {
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": parameters})
}

/// The frame's samples and the ids and messages it said.
fn draw(project: &J, frame: i32) -> (Vec<f32>, Vec<String>) {
    let loaded = persist::load_str(&project.to_string())
        .unwrap_or_else(|d| panic!("the project reads: {}", d.message));
    let mut log = FrameLog::new(8);
    let buffer = render_frame(
        loaded.document.project(),
        &Id::new(MAIN),
        frame,
        &dir(),
        64,
        &mut log,
    )
    .expect("the frame draws");
    let said = log
        .finish()
        .iter()
        .map(|d| format!("{} {}", d.id.as_str(), d.message))
        .collect();
    (buffer.data().to_vec(), said)
}

/// sRGB's transfer (IEC 61966-2-1), linear to an 8-bit code, as a colour word holds it.
fn srgb_word(linear: [f64; 3]) -> String {
    let code = |l: f64| {
        let e = if l <= 0.0031308 { 12.92 * l } else { 1.055 * l.powf(1.0 / 2.4) - 0.055 };
        (e.clamp(0.0, 1.0) * 255.0).round() as u8
    };
    format!("#{:02x}{:02x}{:02x}", code(linear[0]), code(linear[1]), code(linear[2]))
}

const CC: [f64; 3] = [0.25, 0.5, 0.75];

fn color_control() -> J {
    fx("fx-cc", "core.color_control", json!({"color": CC}))
}

fn fill(color: J) -> J {
    fx("fx-fill", "core.fill", json!({"mask": 0, "all_masks": "off", "color": color,
        "invert": "off", "horizontal_feather": 0, "vertical_feather": 0, "opacity": 50}))
}

fn shadow(color: J) -> J {
    fx("fx-ds", "core.drop_shadow", json!({"color": color, "opacity": 100, "direction": 135,
        "distance": 3, "softness": 2}))
}

fn tint(color: J) -> J {
    fx("fx-tint", "core.tint", json!({"color": color, "amount": 1.0}))
}

fn twirl(center: J) -> J {
    fx("fx-tw", "core.twirl", json!({"angle": 90, "radius": 40, "center": center}))
}

fn blur(id: &str, sigma: J) -> J {
    fx(id, "core.gaussian_blur", json!({"sigma_px": sigma}))
}

fn slider(v: f64) -> J {
    fx("fx-sl", "core.slider_control", json!({"slider": v}))
}

fn length(v: f64) -> J {
    fx("fx-db", "core.directional_blur", json!({"direction": 90, "length": v}))
}

#[test]
fn b700_effect_setting_expressions() {
    let mut t = Table::new(
        "effect_mix",
        "# D-820: expressions on an effect's colour and point, and reading any effect's setting\n\n\
         B-700. Each frame is the file with the expression; the expected frame is the same file \
         with the value the expression should give written in, worked out by this test from \
         After Effects' rules: a colour is four numbers, red, green, blue and alpha (in this \
         build's linear numbers, FX-EXPR-017); a point is in the layer's pixels, which the file \
         holds as per cent of the layer's width and height (bands is 16 by 10); a colour word is \
         the sRGB code of the linear numbers. The file's own constant must draw differently, so \
         no row passes by the expression doing nothing. The Color Control holds the linear \
         colour (0.25, 0.5, 0.75).\n",
    );

    let cc_word = srgb_word(CC);
    let runs: Vec<(&str, J, J, J)> = vec![
        (
            "Fill's Color (a colour word) from a Color Control, `effect(\"Color Control\")(\"Color\")`",
            with(json!([color_control(), fill(held(json!("#ff0000"), "effect(\"Color Control\")(\"Color\")"))]), json!([])),
            with(json!([color_control(), fill(json!(cc_word))]), json!([])),
            with(json!([color_control(), fill(json!("#ff0000"))]), json!([])),
        ),
        (
            "Drop Shadow's Color, Smertimba's neon, as lane w7e typed it",
            with(json!([color_control(), shadow(held(json!("#00ff00"), "effect(\"Color Control\")(\"Color\")"))]), json!([])),
            with(json!([color_control(), shadow(json!(cc_word))]), json!([])),
            with(json!([color_control(), shadow(json!("#00ff00"))]), json!([])),
        ),
        (
            "Drop Shadow's Color written as four numbers, `[1, 0, 0, 1]`",
            with(json!([shadow(held(json!("#00ff00"), "[1, 0, 0, 1]"))]), json!([])),
            with(json!([shadow(json!("#ff0000"))]), json!([])),
            with(json!([shadow(json!("#00ff00"))]), json!([])),
        ),
        (
            "Tint's Color (linear numbers) from a Color Control on another layer, `thisComp.layer(\"dot\").effect(\"Color Control\")(\"Color\")`",
            with(json!([tint(held(json!([0, 0, 1]), "thisComp.layer(\"dot\").effect(\"Color Control\")(\"Color\")"))]), json!([color_control()])),
            with(json!([tint(json!(CC))]), json!([color_control()])),
            with(json!([tint(json!([0, 0, 1]))]), json!([color_control()])),
        ),
        (
            "Twirl's Center (a point) from another layer's position, `thisComp.layer(\"dot\").transform.position`: (4, 2) is 25 % and 20 %",
            with(json!([twirl(held(json!([50, 50]), "thisComp.layer(\"dot\").transform.position"))]), json!([])),
            with(json!([twirl(json!([25, 20]))]), json!([])),
            with(json!([twirl(json!([50, 50]))]), json!([])),
        ),
        (
            "Twirl's Center from its own value, `value + [4, 0]`: 50 % of 16 is 8, and 12 is 75 %",
            with(json!([twirl(held(json!([50, 50]), "value + [4, 0]"))]), json!([])),
            with(json!([twirl(json!([75, 50]))]), json!([])),
            with(json!([twirl(json!([50, 50]))]), json!([])),
        ),
        (
            "Gaussian Blur's sigma from a Slider Control, `effect(\"Slider Control\")(\"Slider\") / 2`",
            with(json!([slider(3.0), blur("fx-b", held(json!(0.5), "effect(\"Slider Control\")(\"Slider\") / 2"))]), json!([])),
            with(json!([slider(3.0), blur("fx-b", json!(1.5))]), json!([])),
            with(json!([slider(3.0), blur("fx-b", json!(0.5))]), json!([])),
        ),
        (
            "Gaussian Blur's sigma from another layer's Directional Blur, `thisComp.layer(\"dot\").effect(\"Directional Blur\")(\"Length\")`",
            with(json!([blur("fx-b", held(json!(0.5), "thisComp.layer(\"dot\").effect(\"Directional Blur\")(\"Length\")"))]), json!([length(2.0)])),
            with(json!([blur("fx-b", json!(2.0))]), json!([length(2.0)])),
            with(json!([blur("fx-b", json!(0.5))]), json!([length(2.0)])),
        ),
        (
            "Gaussian Blur's sigma from the second effect's second setting, `effect(2)(2)`",
            with(json!([blur("fx-b", held(json!(0.5), "effect(2)(2)")), length(1.25)]), json!([])),
            with(json!([blur("fx-b", json!(1.25)), length(1.25)]), json!([])),
            with(json!([blur("fx-b", json!(0.5)), length(1.25)]), json!([])),
        ),
    ];
    t.heading("It runs: the frame is the file with the expected value written in, and nothing is said");
    for (what, project, want, plain) in &runs {
        let (got, said) = draw(project, 2);
        let (expected, _) = draw(want, 2);
        let (own, _) = draw(plain, 2);
        t.row(
            &format!("{what}: frame 2"),
            &format!(
                "{}, {} the file's constant; said {said:?}",
                if got == expected { "sample-identical to the expected value" } else { "differs from the expected value" },
                if got != own { "unlike" } else { "the same as" }
            ),
            got == expected && got != own && said.is_empty(),
        );
    }

    let fails: Vec<(&str, J, J, &str)> = vec![
        (
            "Two Gaussian Blurs each reading the other's sigma",
            with(json!([blur("fx-b", held(json!(1), "effect(\"Gaussian Blur 2\")(\"sigma_px\")")),
                        blur("fx-b2", held(json!(1), "effect(\"Gaussian Blur\")(\"sigma_px\")"))]), json!([])),
            with(json!([blur("fx-b", json!(1)), blur("fx-b2", json!(1))]), json!([])),
            "EXPRESSION_CYCLE",
        ),
        (
            "Drop Shadow's Color given three numbers, `[1, 0, 0]` (After Effects asks for four)",
            with(json!([shadow(held(json!("#00ff00"), "[1, 0, 0]"))]), json!([])),
            with(json!([shadow(json!("#00ff00"))]), json!([])),
            "EXPRESSION_TYPE",
        ),
        (
            "Twirl's Center reading another layer's Twirl Center, whose pixels need that layer's size",
            with(json!([twirl(held(json!([50, 50]), "thisComp.layer(\"dot\").effect(\"Twirl\")(\"Center\")"))]), json!([twirl(json!([25, 20]))])),
            with(json!([twirl(json!([50, 50]))]), json!([twirl(json!([25, 20]))])),
            "EXPRESSION_TYPE",
        ),
        (
            "`effect(\"Gaussian Blur\")(\"Blurriness\")`, which this blur holds as a sigma",
            with(json!([blur("fx-b", json!(1)), fx("fx-sl", "core.slider_control", json!({"slider":
                held(json!(0), "effect(\"Gaussian Blur\")(\"Blurriness\")")}))]), json!([])),
            with(json!([blur("fx-b", json!(1)), slider(0.0)]), json!([])),
            "EXPRESSION_REFERENCE_MISSING",
        ),
        (
            "Fill's All Masks, a choice from a list, `1`",
            with(json!([fx("fx-fill", "core.fill", json!({"mask": 0, "all_masks": held(json!("off"), "1"),
                "color": "#ff0000", "invert": "off", "horizontal_feather": 0, "vertical_feather": 0,
                "opacity": 50}))]), json!([])),
            with(json!([fill(json!("#ff0000"))]), json!([])),
            "EXPRESSION_TYPE",
        ),
    ];
    t.heading("It fails: the frame is the file's constant, and the frame says why");
    for (what, project, plain, code) in &fails {
        let (got, said) = draw(project, 2);
        let (want, _) = draw(plain, 2);
        t.row(
            &format!("{what}: frame 2 is the file's constant and says {code}"),
            &format!("{}; said {said:?}", if got == want { "sample-identical" } else { "differs" }),
            got == want && said.iter().any(|s| s.starts_with(code)),
        );
    }

    t.heading("Saved as read, and old projects as they were");
    let project = &runs[0].1;
    let loaded = persist::load_str(&project.to_string()).unwrap();
    let saved = persist::to_json(loaded.document.project(), &loaded.preserved);
    let again: J = serde_json::from_str(&saved).unwrap();
    let art = again["compositions"][0]["layers"]
        .as_array()
        .unwrap()
        .iter()
        .find(|l| l["id"] == "art")
        .unwrap();
    let written = &art["effects"][1]["parameters"]["color"];
    let read = &project["compositions"][0]["layers"][0]["effects"][1]["parameters"]["color"];
    t.row(
        "Fill's Color with its expression: saved as read, expression text and all",
        &written.to_string(),
        same_json(written, read),
    );
    for old in [
        "Fixtures/effect_mix/fx_mix_001.json",
        "Fixtures/projects/expression_controls_project.json",
    ] {
        let text = std::fs::read_to_string(repo(old)).unwrap();
        let loaded = persist::load_str(&text).unwrap();
        let saved: J =
            serde_json::from_str(&persist::to_json(loaded.document.project(), &loaded.preserved))
                .unwrap();
        let same = same_json(&saved, &serde_json::from_str(&text).unwrap());
        t.row(
            &format!("{old}, written before D-820, opens and saves holding what it held"),
            if same { "the same" } else { "differs" },
            same,
        );
    }
    t.finish("B-700_effect_setting_expressions_table.md");
}
