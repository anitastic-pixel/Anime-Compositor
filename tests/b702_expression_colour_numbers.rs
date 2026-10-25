//! B-702 (D-822): an expression's colours are After Effects' numbers, 0 to 1 sRGB, linear only
//! in Float without AE 32 bpc (After Effects' Linearize Working Space).
//!
//! As B-700 does, each check draws the file with the expression and the same file with the value
//! the expression should give written in, worked out here from After Effects' rule, and the
//! file's own constant must draw differently. A row also shows D-820's linear reading drawing
//! otherwise, so the check cannot pass on the old rule.

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

/// The bands fixture (16 by 10) with `effects` on its one layer and the composition's depth
/// switches `depth` (`{}` is Display).
fn with(effects: J, depth: J) -> J {
    let mut p: J =
        serde_json::from_str(&std::fs::read_to_string(dir().join("fx_mix_001.json")).unwrap())
            .unwrap();
    let comp = &mut p["compositions"][0];
    comp["layers"][0]["effects"] = effects;
    for (k, v) in depth.as_object().unwrap() {
        comp[k] = v.clone();
    }
    p
}

fn held(base: J, text: &str) -> J {
    json!({"base": base, "keyframes": [], "expression": {"text": text, "enabled": true}})
}

fn fx(id: &str, type_id: &str, parameters: J) -> J {
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": parameters})
}

/// The frame's samples (linear, premultiplied RGBA) and what it said.
fn draw(project: &J) -> (Vec<f32>, Vec<String>) {
    let loaded = persist::load_str(&project.to_string())
        .unwrap_or_else(|d| panic!("the project reads: {}", d.message));
    let mut log = FrameLog::new(8);
    let buffer = render_frame(
        loaded.document.project(),
        &Id::new(MAIN),
        2,
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

/// sRGB's transfer (IEC 61966-2-1), linear to encoded, and back.
fn encode(l: f64) -> f64 {
    if l <= 0.0031308 { 12.92 * l } else { 1.055 * l.powf(1.0 / 2.4) - 0.055 }
}
fn decode(e: f64) -> f64 {
    if e <= 0.04045 { e / 12.92 } else { ((e + 0.055) / 1.055).powf(2.4) }
}

const CC: [f64; 3] = [0.25, 0.5, 0.75];

fn color_control() -> J {
    fx("fx-cc", "core.color_control", json!({"color": CC}))
}

fn fill(color: J) -> J {
    fx("fx-fill", "core.fill", json!({"mask": 0, "all_masks": "off", "color": color,
        "invert": "off", "horizontal_feather": 0, "vertical_feather": 0, "opacity": 100}))
}

fn tint(color: J) -> J {
    fx("fx-tint", "core.tint", json!({"color": color, "amount": 1.0}))
}

fn blur(sigma: J) -> J {
    fx("fx-b", "core.gaussian_blur", json!({"sigma_px": sigma}))
}

const ORANGE: &str = "[1, 0.5, 0, 1]";

#[test]
fn b702_expression_colour_numbers() {
    let mut t = Table::new(
        "effect_mix",
        "# D-822: an expression's colours are After Effects' numbers\n\n\
         B-702. A colour in an expression is four numbers 0 to 1, After Effects' sRGB numbers \
         (Display, 8 bpc and AE 32 bpc, which are After Effects with Linearize Working Space \
         off), and this build's linear numbers only in Float without AE 32 bpc (Linearize Working \
         Space on). Each frame is the file with the expression; the expected frame is the file \
         with the value the expression should give written in. The file's own constant, and \
         D-820's linear reading, must draw differently. The Color Control holds the linear \
         colour (0.25, 0.5, 0.75), which After Effects shows as about (0.537, 0.735, 0.881).\n",
    );

    let display = json!({});
    let eight = json!({"eight_bpc": true});
    let ae32 = json!({"float_depth": true, "ae_32bpc": true});
    let float = json!({"float_depth": true});
    let typed = |depth: &J| with(json!([fill(held(json!("#0000ff"), ORANGE))]), depth.clone());
    let word = |w: &str, depth: &J| with(json!([fill(json!(w))]), depth.clone());

    // (what, with the expression, the expected value written in, D-820's reading or the constant)
    let runs: Vec<(String, J, J, J)> = vec![
        (
            format!("The tutorial's typed orange on Fill's Color, `{ORANGE}`, in Display: #ff8000 (D-820 gave #ffbc00)"),
            typed(&display),
            word("#ff8000", &display),
            word("#ffbc00", &display),
        ),
        (
            "The same in 8 bpc: #ff8000".to_string(),
            typed(&eight),
            word("#ff8000", &eight),
            word("#ffbc00", &eight),
        ),
        (
            "The same in AE 32 bpc: #ff8000".to_string(),
            typed(&ae32),
            word("#ff8000", &ae32),
            word("#ffbc00", &ae32),
        ),
        (
            "The same in Float, Linearize Working Space on: linear numbers, #ffbc00".to_string(),
            typed(&float),
            word("#ffbc00", &float),
            word("#ff8000", &float),
        ),
        (
            format!("Tint's Color (stored linear) given `{ORANGE}` in Display: green 0.5 is linear {:.6}", decode(0.5)),
            with(json!([tint(held(json!([0, 0, 1]), ORANGE))]), display.clone()),
            with(json!([tint(json!([1.0, decode(0.5), 0.0]))]), display.clone()),
            with(json!([tint(json!([1.0, 0.5, 0.0]))]), display.clone()),
        ),
        (
            format!("A Color Control read, `effect(\"Color Control\")(\"Color\")[1] * 4` on Gaussian Blur's sigma: green 0.5 reads {:.6}", encode(0.5)),
            with(json!([color_control(), blur(held(json!(0.5), "effect(\"Color Control\")(\"Color\")[1] * 4"))]), display.clone()),
            with(json!([color_control(), blur(json!(encode(0.5) * 4.0))]), display.clone()),
            with(json!([color_control(), blur(json!(2.0))]), display.clone()),
        ),
        (
            format!("A Fill colour read, `effect(\"Fill\")(\"Color\")[1] * 4` with Fill at #ff8000: 128 / 255 = {:.6}", 128.0f32 / 255.0),
            with(json!([fill(json!("#ff8000")), blur(held(json!(0.5), "effect(\"Fill\")(\"Color\")[1] * 4"))]), display.clone()),
            with(json!([fill(json!("#ff8000")), blur(json!((128.0f32 / 255.0) as f64 * 4.0))]), display.clone()),
            with(json!([fill(json!("#ff8000")), blur(json!(decode(128.0 / 255.0) * 4.0))]), display.clone()),
        ),
        (
            "A Color Control linked into Tint, `effect(\"Color Control\")(\"Color\")`: the same colour as before, out and back".to_string(),
            with(json!([color_control(), tint(held(json!([0, 0, 1]), "effect(\"Color Control\")(\"Color\")"))]), display.clone()),
            with(json!([color_control(), tint(json!(CC.map(|c| decode(encode(c)))))]), display.clone()),
            with(json!([color_control(), tint(json!([0, 0, 1]))]), display.clone()),
        ),
    ];
    t.heading("It runs: the frame is the file with the expected value written in, and nothing is said");
    for (what, project, want, other) in &runs {
        let (got, said) = draw(project);
        let (expected, _) = draw(want);
        let (old, _) = draw(other);
        t.row(
            &format!("{what}: frame 2"),
            &format!(
                "{}, {} the other reading; said {said:?}",
                if got == expected { "sample-identical to the expected value" } else { "differs from the expected value" },
                if got != old { "unlike" } else { "the same as" }
            ),
            got == expected && got != old && said.is_empty(),
        );
    }

    t.heading("The orange on screen");
    let (px, _) = draw(&typed(&display));
    let at = (8 * 16 + 12) * 4;
    let code = |c: f32| (encode((c / px[at + 3]) as f64).clamp(0.0, 1.0) * 255.0).round() as u8;
    let rgb = [code(px[at]), code(px[at + 1]), code(px[at + 2])];
    t.row(
        &format!("`{ORANGE}` on Fill's Color in Display, pixel (12, 8) in sRGB codes: After Effects' orange 255, 128, 0"),
        &format!("{}, {}, {} at alpha {}", rgb[0], rgb[1], rgb[2], px[at + 3]),
        rgb == [255, 128, 0],
    );

    t.heading("Old projects as they were");
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
            &format!("{old}, written before D-822, opens and saves holding what it held"),
            if same { "the same" } else { "differs" },
            same,
        );
    }
    t.finish("B-702_expression_colour_numbers_table.md");
}
