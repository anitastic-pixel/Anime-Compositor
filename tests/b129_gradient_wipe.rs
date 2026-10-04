//! B-129: gradient wipe in the core, against D-194.
//!
//! Writes `verification/B-129_gradient_wipe_table.md`.
//!
//! Every expected pixel is `Fixtures/gradient_wipe/expected_gradient_wipe.json`, written by
//! `tools/gradient_wipe_reference.py` before this code existed and printed in document 25 as
//! FX-GWIPE-001 to 028. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws B-127's street over a plum solid and two maps for it, a spot and clouds, and
//! wipes the street five ways into `verification/B-129 pictures/`.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{town, Table, TOWN};
use serde_json::Value as J;

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, EffectInstance, EffectKey};
use anime_compositor::model::{Id, Interp};
use anime_compositor::{persist, png_out, OutputDepth};

fn gwipe(layer: J, fit: &str, completion: f64, softness: f64, invert: &str) -> Effect {
    Effect::GradientWipe {
        layer,
        fit: fit.to_string(),
        completion,
        softness,
        invert: invert.to_string(),
        map: None,
    }
}

/// The ramp, stretched, at `completion` and `softness`.
fn ramp(completion: f64, softness: f64) -> Effect {
    gwipe(J::from("ramp"), "stretch", completion, softness, "off")
}

/// A Gradient Wipe naming `named`, half done.
fn naming(named: &str) -> Effect {
    gwipe(J::from(named), "stretch", 50.0, 0.0, "off")
}

fn main_comp() -> Id {
    Id::new(effect_table::MAIN)
}

/// Set the effect `instance` on `layer`.
fn set(layer: &str, instance: &str, effect: Effect) -> Command {
    Command::SetEffectParameters {
        composition: main_comp(),
        layer_id: Id::new(layer),
        instance_id: Id::new(instance),
        effect,
    }
}

/// Add `effect` to the end of `layer`'s stack.
fn add(layer: &str, instance: &str, effect: Effect) -> Command {
    Command::AddEffect {
        composition: main_comp(),
        layer_id: Id::new(layer),
        effect: EffectInstance::new(Id::new(instance), effect),
        index: None,
    }
}

/// The whole composition, to see that a refused command changed nothing.
fn everything(document: &Document) -> String {
    format!("{:?}", document.project().composition(&main_comp()).unwrap())
}

fn refused(t: &mut Table, document: &mut Document, commands: Vec<(&str, Command, Option<&str>)>) {
    let held = everything(document);
    for (what, command, id) in commands {
        let refused = document.apply(command).err();
        let untouched = everything(document) == held;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| format!("{} {}", d.id.as_str(), d.message)),
            refused.as_ref().is_some_and(|d| id.is_none_or(|id| d.id.as_str() == id)) && untouched,
        );
    }
}

/// Where the spot is darkest, and how far out it turns white, in pixels.
const SPOT: (f64, f64, f64) = (480.0 * 0.4, 270.0 * 0.5, 480.0 * 0.6);

/// A grey map, one byte a pixel from `grey`, opaque.
fn grey_map(grey: impl Fn(f64, f64) -> f64) -> Vec<u8> {
    let (w, h) = TOWN;
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let g = (255.0 * grey(x as f64 + 0.5, y as f64 + 0.5).clamp(0.0, 1.0)).round() as u8;
            bytes.extend([g, g, g, 255]);
        }
    }
    bytes
}

/// Black at the spot, white `SPOT.2` pixels away and beyond.
fn spot() -> Vec<u8> {
    let (cx, cy, r) = SPOT;
    grey_map(|x, y| (x - cx).hypot(y - cy) / r)
}

/// Soft clouds from a few sines, stretched from black to white.
fn clouds() -> Vec<u8> {
    let (w, h) = TOWN;
    let g = |x: f64, y: f64| {
        let (x, y) = (x / w as f64, y / w as f64);
        (9.0 * x + 3.0 * (7.0 * y).sin()).sin() + (13.0 * y + 2.0 * (11.0 * x).sin() + 1.0).sin() + 0.5 * (23.0 * x + 19.0 * y).sin()
    };
    let all: Vec<f64> = (0..w * h).map(|i| g((i % w) as f64 + 0.5, (i / w) as f64 + 0.5)).collect();
    let (lo, hi) = all.iter().fold((f64::MAX, f64::MIN), |(lo, hi), &v| (lo.min(v), hi.max(v)));
    grey_map(|x, y| (g(x, y) - lo) / (hi - lo))
}

/// The street as the holder over a plum solid, and `map` as the switched-off `ramp` layer, with
/// `effects` on the street, drawn at frame 0.
fn picture(dir: &Path, map: &str, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/gradient_wipe/fx_gwipe_003.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    project["assets"][1]["path"] = J::from(map);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let layers = comp["layers"].as_array_mut().unwrap();
    for layer in layers.iter_mut() {
        if layer["id"] == "holder" {
            layer["effects"] = effects.clone();
        }
    }
    // Plum, sRGB 70, 40 and 90, in the working space.
    let mut plum = layers.iter().find(|l| l["id"] == "white").unwrap().clone();
    plum["id"] = J::from("plum");
    plum["name"] = J::from("plum");
    plum["enabled"] = J::from(true);
    plum["solid"] = serde_json::json!({"color": [0.0612, 0.0212, 0.1022], "width": TOWN.0, "height": TOWN.1});
    layers.insert(0, plum);
    comp["layer_order"].as_array_mut().unwrap().insert(0, J::from("plum"));
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &main_comp(), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
}

#[test]
fn b129_gradient_wipe() {
    let mut t = Table::new(
        "gradient_wipe",
        "# B-129: gradient wipe\n\nD-194, accepted by the owner on 2026-09-28 (\"take \
         everything\"). Every expected pixel is \
         `Fixtures/gradient_wipe/expected_gradient_wipe.json`, written by \
         `tools/gradient_wipe_reference.py` before this code existed and printed in document 25 \
         as FX-GWIPE-001 to 028. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-GWIPE-001 to 028 (document 25)");
    t.fixtures_numbered("expected_gradient_wipe.json", 1..=28);

    t.heading("Files whose layers read each other (document 25)");
    let expected: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/gradient_wipe/expected_gradient_wipe.json")).unwrap(),
    )
    .unwrap();
    for (file, case) in expected["loads"].as_object().unwrap() {
        let says = case["says"].as_str().unwrap();
        let id = case["refused"].as_str().unwrap();
        let got = persist::load(&effect_table::repo(&format!("Fixtures/gradient_wipe/{file}"))).err();
        t.row(
            &format!("{file}: {says}"),
            &got.as_ref().map_or("opened".to_string(), |d| format!("{} {}", d.id.as_str(), d.message)),
            got.is_some_and(|d| d.id.as_str() == id),
        );
    }

    t.heading("How far it reaches");
    let got = ramp(50.0, 100.0).bounds_expansion();
    t.row("a wipe never grows the drawing's bounds", &got.to_string(), got == 0);
    let mut draft = ramp(40.0, 30.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview leaves every setting as it is, since none is a distance",
        &format!("{draft:?}"),
        draft == ramp(40.0, 30.0),
    );

    t.heading("The file");
    let files: Vec<String> = (1..=28).map(|n| format!("fx_gwipe_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let params = t.saved_parameters("fx_gwipe_028.json");
    t.row(
        "fx_gwipe_028.json, its layer written as the number 3, is saved as the number 3",
        &params["layer"].to_string(),
        params["layer"] == serde_json::json!(3),
    );
    let params = t.saved_parameters("fx_gwipe_003.json");
    t.row(
        "the map the effect reads is never saved",
        &format!("{:?}", params.get("map")),
        params.get("map").is_none(),
    );
    for (what, parameters) in [
        ("no `fit` at all", r#"{"layer": "ramp", "completion": 50, "softness": 0, "invert": "off"}"#),
        ("no `layer` at all", r#"{"fit": "stretch", "completion": 50, "softness": 0, "invert": "off"}"#),
        ("no `invert` at all", r#"{"layer": "ramp", "fit": "stretch", "completion": 50, "softness": 0}"#),
        (
            "a completion that is a word",
            r#"{"layer": "ramp", "fit": "stretch", "completion": "half", "softness": 0, "invert": "off"}"#,
        ),
    ] {
        t.shape_refused("fx_gwipe_003.json", what, parameters);
    }

    t.heading("Commands");
    let mut document = t.load("fx_gwipe_003.json").document;
    refused(
        &mut t,
        &mut document,
        vec![
            ("Transition Completion 101", set("holder", "fx-1", ramp(101.0, 0.0)), None),
            ("Transition Softness -1", set("holder", "fx-1", ramp(50.0, -1.0)), None),
            (
                "Transition Completion keyed to 200 at frame 4",
                Command::SetEffectKeys {
                    composition: main_comp(),
                    layer_id: Id::new("holder"),
                    instance_id: Id::new("fx-1"),
                    setting: "completion".to_string(),
                    keys: [(0, 30.0), (4, 200.0)]
                        .map(|(frame, v)| EffectKey { frame, value: vec![v], interp: Interp::Linear })
                        .to_vec(),
                },
                None,
            ),
            ("fit \"fill\"", set("holder", "fx-1", gwipe(J::from("ramp"), "fill", 50.0, 0.0, "off")), None),
            ("invert \"yes\"", set("holder", "fx-1", gwipe(J::from("ramp"), "stretch", 50.0, 0.0, "yes")), None),
            ("a layer that is the number 3", set("holder", "fx-1", gwipe(J::from(3), "stretch", 50.0, 0.0, "off")), None),
            (
                "a Gradient Wipe on `ramp` naming `holder`, which names `ramp`",
                add("ramp", "fx-2", naming("holder")),
                Some("EFFECT_LAYER_CYCLE"),
            ),
            (
                "a Displacement Map on `ramp` naming `holder`, whose Gradient Wipe names `ramp`",
                add(
                    "ramp",
                    "fx-2",
                    Effect::DisplacementMap {
                        layer: J::from("holder"),
                        fit: "stretch".to_string(),
                        horizontal: "red".to_string(),
                        max_horizontal: 5.0,
                        vertical: "green".to_string(),
                        max_vertical: 5.0,
                        wrap: "off".to_string(),
                        expand: "off".to_string(),
                        map: None,
                    },
                ),
                Some("EFFECT_LAYER_CYCLE"),
            ),
        ],
    );
    let taken = [
        ("naming the holder itself, which is no circle,", set("holder", "fx-1", naming("holder"))),
        (
            "Invert Gradient on, the checker tiled, softness 100",
            set("holder", "fx-1", gwipe(J::from("card"), "tile", 50.0, 100.0, "on")),
        ),
        ("a Gradient Wipe on `ramp` naming `white`", add("ramp", "fx-2", naming("white"))),
        ("a Gradient Wipe on `white` naming no layer", add("white", "fx-3", naming(""))),
    ];
    let n = taken.len();
    for (what, command) in taken {
        let taken = document.apply(command).is_ok();
        t.row(&format!("{what} is taken"), if taken { "taken" } else { "refused" }, taken);
    }
    refused(
        &mut t,
        &mut document,
        vec![(
            "`white`'s Gradient Wipe changed to name `ramp`, which names `white`",
            set("white", "fx-3", naming("ramp")),
            Some("EFFECT_LAYER_CYCLE"),
        )],
    );
    let before = t.render(&t.load("fx_gwipe_003.json").document, 0, 64);
    for _ in 0..n {
        document.undo();
    }
    let same = t.render(&document, 0, 64).data() == before.data();
    t.row(&format!("undo {n} times: frame 0 is the frame it was"), if same { "byte-identical" } else { "differ" }, same);

    t.heading("Pictures: B-127's street over a plum solid, in `verification/B-129 pictures/`");
    let dir = effect_table::repo("verification/B-129 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    png_out::write_rgba(&dir.join("town.png"), w, h, OutputDepth::Eight, &[], &town()).unwrap();
    png_out::write_rgba(&dir.join("map_spot.png"), w, h, OutputDepth::Eight, &[], &spot()).unwrap();
    png_out::write_rgba(&dir.join("map_clouds.png"), w, h, OutputDepth::Eight, &[], &clouds()).unwrap();
    let (before, said) = picture(&dir, "map_spot.png", J::Array(vec![]));
    png_out::write_rgba(&dir.join("before.png"), w, h, OutputDepth::Eight, &[], &before).unwrap();
    t.row("before.png, the street with no effect, draws cleanly", &format!("{said:?}"), said.is_empty());
    let (plum, _) = picture(&dir, "map_spot.png", serde_json::json!([{
        "instance_id": "fx-1", "type_id": "core.gradient_wipe", "enabled": true,
        "parameters": {"layer": "ramp", "fit": "stretch", "completion": 100, "softness": 0, "invert": "off"},
    }]));
    let px = |bytes: &[u8], i: usize| bytes[i * 4..i * 4 + 4].to_vec();
    let is_street = |bytes: &[u8], i: usize| px(bytes, i) == px(&before, i);
    let is_plum = |bytes: &[u8], i: usize| px(bytes, i) == px(&plum, i);
    let count = |f: &dyn Fn(usize) -> bool| (0..w * h).filter(|&i| f(i)).count();
    let (cx, cy, r) = SPOT;
    let from_spot = |i: usize| ((i % w) as f64 + 0.5 - cx).hypot((i / w) as f64 + 0.5 - cy) / r;
    let mut gone_before = 0;
    for (name, map, completion, softness, invert, what) in [
        ("spot_30", "map_spot.png", 30.0, 0.0, "off", "the spot at completion 30%, softness 0: a hard hole round the spot, the street whole outside it"),
        ("spot_60_soft", "map_spot.png", 60.0, 25.0, "off", "completion 60%, softness 25: a wider hole with a soft edge"),
        ("spot_30_invert", "map_spot.png", 30.0, 0.0, "on", "completion 30% with Invert Gradient: wiped from the outside in, the street kept within 70% of the way out"),
        ("clouds_40", "map_clouds.png", 40.0, 10.0, "off", "the clouds at completion 40%, softness 10: soft patches gone"),
        ("clouds_75", "map_clouds.png", 75.0, 10.0, "off", "the clouds at 75%: most of the street gone"),
    ] {
        let effects = serde_json::json!([{
            "instance_id": "fx-1", "type_id": "core.gradient_wipe", "enabled": true,
            "parameters": {"layer": "ramp", "fit": "stretch", "completion": completion, "softness": softness, "invert": invert},
        }]);
        let (bytes, said) = picture(&dir, map, effects);
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let gone = count(&|i| is_plum(&bytes, i) && !is_street(&bytes, i));
        let between = count(&|i| !is_plum(&bytes, i) && !is_street(&bytes, i));
        // With a hard edge each pixel is the street or the plum; at the spot's 30% the edge is
        // 30% of the way out, so pixels well inside it are gone and well outside it kept, and
        // inverted it is 70% of the way out, the other way round.
        let (check, ok) = match name {
            "spot_30" => {
                let wrong = count(&|i| {
                    (from_spot(i) < 0.28 && !is_plum(&bytes, i)) || (from_spot(i) > 0.32 && !is_street(&bytes, i))
                });
                (format!("{gone} pixels gone, {between} between, {wrong} on the wrong side of 30%"), gone > 0 && between == 0 && wrong == 0)
            }
            "spot_60_soft" => (format!("{gone} pixels gone, {between} fading"), gone > gone_before && between > 0),
            "spot_30_invert" => {
                let wrong = count(&|i| {
                    (from_spot(i) < 0.68 && !is_street(&bytes, i)) || (from_spot(i) > 0.72 && !is_plum(&bytes, i))
                });
                (format!("{between} between, {wrong} on the wrong side of 70%"), between == 0 && wrong == 0)
            }
            "clouds_40" => (format!("{gone} pixels gone, {between} fading"), gone > 0 && between > 0),
            _ => (format!("{gone} pixels gone, more than at 40%"), gone > gone_before),
        };
        gone_before = gone;
        t.row(&format!("{name}.png, {what}, draws cleanly"), &format!("{said:?}, {check}"), said.is_empty() && ok);
    }

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_gwipe_003.json", 0),
        ("fx_gwipe_009.json", 0),
        ("fx_gwipe_014.json", 0),
        ("fx_gwipe_015.json", 0),
        ("fx_gwipe_016.json", 0),
        ("fx_gwipe_021.json", 0),
    ]);

    t.finish("B-129_gradient_wipe_table.md");
}
