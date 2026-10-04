//! B-187: D-302, Curves' alpha curve; and the file keeping a P-26 setting put back to its start.
//!
//! P-26, tutorial 3 (Colorful Glitch) bends Curves' Alpha channel to thin a layer's covering;
//! here Curves bent the colour only. Worked by hand on a 16x16 opaque solid, linear
//! (0.2, 0.4, 0.6):
//! - alpha through [[0, 0], [255, 128]], a straight line, is 128/255 = 0.501961 everywhere,
//!   the colour kept, so the pixel is (0.2, 0.4, 0.6) x 0.501961;
//! - then through [[0, 0], [128, 255]], 128 goes to 255: opaque, and the colour (0.2, 0.4, 0.6)
//!   again;
//! - alpha through [[0, 0], [255, 0]] clears every pixel, and then [[0, 64], [255, 255]] takes
//!   0 to 64: black at 64/255 = 0.250980, since a pixel that did not show has no colour.

use serde_json::{json, Value as J};

use anime_compositor::command::Command;
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, EffectKey};
use anime_compositor::model::{Id, Interp};
use anime_compositor::persist;

const MAIN: &str = "comp-main";

fn curves(alpha: Option<J>) -> J {
    let line = json!([[0, 0], [255, 255]]);
    let mut parameters = json!({"master": line, "red": line, "green": line, "blue": line});
    if let Some(a) = alpha {
        parameters["alpha"] = a;
    }
    json!({"type_id": "core.curves", "enabled": true, "parameters": parameters})
}

fn shot(effects: Vec<J>) -> String {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let effects: Vec<J> = effects
        .into_iter()
        .enumerate()
        .map(|(i, mut e)| {
            e["instance_id"] = json!(format!("f{i}"));
            e
        })
        .collect();
    json!({
        "schema_version": 0, "project_id": "proj-b187",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": MAIN, "name": MAIN, "width": 16, "height": 16, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": ["art"],
            "layers": [{
                "id": "art", "kind": "solid", "name": "art", "enabled": true, "locked": false,
                "in_frame": 0, "out_frame": 1,
                "solid": {"color": [0.2, 0.4, 0.6], "width": 16, "height": 16},
                "transform": {
                    "anchor": t(json!([8, 8])), "position": t(json!([8, 8])),
                    "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                },
                "masks": [], "matte": null, "blend_mode": "normal", "effects": effects
            }]
        }]
    })
    .to_string()
}

fn frame(effects: Vec<J>) -> Vec<[f32; 4]> {
    let loaded = persist::load_str(&shot(effects)).expect("the shot reads");
    let mut log = FrameLog::new(8);
    let f = render_frame(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), 256, &mut log)
        .expect("the frame draws");
    (0..16).flat_map(|y| (0..16).map(move |x| (x, y))).map(|(x, y)| f.pixel(x, y)).collect()
}

fn near(got: &[[f32; 4]], want: [f64; 4], what: &str) {
    for (i, p) in got.iter().enumerate() {
        for c in 0..4 {
            assert!((p[c] as f64 - want[c]).abs() < 1e-5, "{what}, pixel {i}, channel {c}: {} against {}", p[c], want[c]);
        }
    }
}

#[test]
fn the_alpha_curve_bends_the_covering_and_keeps_the_colour() {
    let half = 128.0 / 255.0;
    near(&frame(vec![curves(Some(json!([[0, 0], [255, 128]])))]), [0.2 * half, 0.4 * half, 0.6 * half, half], "thinned");
    near(
        &frame(vec![curves(Some(json!([[0, 0], [255, 128]]))), curves(Some(json!([[0, 0], [128, 255]])))]),
        [0.2, 0.4, 0.6, 1.0],
        "thinned and back",
    );
    near(
        &frame(vec![curves(Some(json!([[0, 0], [255, 0]]))), curves(Some(json!([[0, 64], [255, 255]])))]),
        [0.0, 0.0, 0.0, 64.0 / 255.0],
        "cleared and lifted",
    );
    // A straight alpha curve, or none, changes nothing.
    let bent = json!([[0, 0], [128, 180], [255, 255]]);
    let mut with = curves(Some(json!([[0, 0], [255, 255]])));
    with["parameters"]["master"] = bent.clone();
    let mut without = curves(None);
    without["parameters"]["master"] = bent;
    assert_eq!(frame(vec![with]), frame(vec![without]), "a straight alpha curve is no alpha curve");
    near(&frame(vec![curves(None)]), [0.2, 0.4, 0.6, 1.0], "all straight");
}

#[test]
fn a_wrong_alpha_curve_is_reported() {
    let loaded = persist::load_str(&shot(vec![curves(Some(json!([[0, 0]])))])).expect("the shot reads");
    let e = &loaded.document.project().compositions[0].layer(&Id::new("art")).unwrap().effects[0].effect;
    assert_eq!(e.why_invalid(), "Curves' alpha curve takes 2 to 16 points, and this has 1.");
}

fn saved(document: &anime_compositor::command::Document, preserved: &persist::Preserved) -> J {
    let saved: J = serde_json::from_str(&persist::to_json(document.project(), preserved)).unwrap();
    saved["compositions"][0]["layers"][0]["effects"][0]["parameters"].clone()
}

/// The first effect, changed by `f`, set back through the command.
fn change(document: &mut anime_compositor::command::Document, f: impl FnOnce(&mut Effect)) {
    let mut effect = document.project().compositions[0].layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    f(&mut effect);
    document
        .apply(Command::SetEffectParameters {
            composition: Id::new(MAIN),
            layer_id: Id::new("art"),
            instance_id: Id::new("f0"),
            effect,
        })
        .expect("the change is taken");
}

#[test]
fn the_file_writes_alpha_only_when_needed_and_never_keeps_a_stale_one() {
    let load = |alpha| persist::load_str(&shot(vec![curves(alpha)])).expect("the shot reads");
    let l = load(None);
    assert!(saved(&l.document, &l.preserved).get("alpha").is_none(), "straight is not written, so the file saves as before");
    let l = load(Some(json!([[0, 0], [255, 255]])));
    assert_eq!(saved(&l.document, &l.preserved)["alpha"], json!([[0, 0], [255, 255]]), "a file that wrote it keeps it");
    let mut l = load(Some(json!([[0, 0], [255, 128]])));
    assert_eq!(saved(&l.document, &l.preserved)["alpha"], json!([[0, 0], [255, 128]]), "bent is kept");
    change(&mut l.document, |e| {
        if let Effect::Curves { alpha, .. } = e {
            *alpha = vec![vec![0.0, 0.0], vec![255.0, 255.0]];
        }
    });
    assert_eq!(saved(&l.document, &l.preserved)["alpha"], json!([[0, 0], [255, 255]]), "put back straight, it saves straight, not the bend the file had");
}

/// The same for D-299's and D-300's settings, which had kept the file's old value.
#[test]
fn a_p26_setting_put_back_to_its_start_saves_as_its_start() {
    let noise = json!({"type_id": "core.fractal_noise", "enabled": true, "parameters": {"size": 4, "complexity": 1,
        "contrast": 100, "brightness": 0, "evolution": 30, "speed": 0, "seed": 7, "dark_color": "#000000",
        "light_color": "#ffffff", "opacity": 100, "blend": "normal", "fractal_type": "turbulent", "cycle": 3}});
    let mut l = persist::load_str(&shot(vec![noise])).expect("the shot reads");
    change(&mut l.document, |e| {
        if let Effect::FractalNoise { fractal_type, cycle, .. } = e {
            *fractal_type = "basic".into();
            *cycle = 0.0;
        }
    });
    // A keyed setting whose plain value is at its start keeps its keys.
    l.document
        .apply(Command::SetEffectKeys {
            composition: Id::new(MAIN),
            layer_id: Id::new("art"),
            instance_id: Id::new("f0"),
            setting: "scale_width".into(),
            keys: [(0, 100.0), (4, 150.0)].map(|(frame, v)| EffectKey { frame, value: vec![v], interp: Interp::Linear }).to_vec(),
        })
        .expect("the keys are taken");
    let p = saved(&l.document, &l.preserved);
    assert_eq!(p["scale_width"]["keyframes"].as_array().map(Vec::len), Some(2), "scale width's keys are saved: {p}");
    assert_eq!((&p["fractal_type"], &p["cycle"]), (&json!("basic"), &json!(0)), "both saved at their start: {p}");
    assert!(p.get("noise_type").is_none(), "one never written stays unwritten");

    let bolt = json!({"type_id": "core.lightning_bolt", "enabled": true, "parameters": {"start": [50, 0], "end": [50, 100],
        "jagged": 0, "detail": 2, "branches": 0, "width": 3, "glow": 4, "opacity": 100, "hold": 1, "seed": 0,
        "color": "#ffffff", "glow_color": "#6e8cff", "composite": "off"}});
    let mut l = persist::load_str(&shot(vec![bolt])).expect("the shot reads");
    change(&mut l.document, |e| {
        if let Effect::LightningBolt { composite, .. } = e {
            *composite = "on".into();
        }
    });
    assert_eq!(saved(&l.document, &l.preserved)["composite"], "on", "on saved, not the file's off");
}
