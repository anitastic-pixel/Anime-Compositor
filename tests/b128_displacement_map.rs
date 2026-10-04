//! B-128: displacement map in the core, against D-193.
//!
//! Writes `verification/B-128_displacement_map_table.md`.
//!
//! Every expected pixel is `Fixtures/displacement_map/expected_displacement_map.json`, written by
//! `tools/displacement_map_reference.py` before this code existed and printed in document 25 as
//! FX-DMAP-001 to 031. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws B-127's street and two maps for it, a pattern of waves and a glass ball, and
//! moves the street four ways into `verification/B-128 pictures/`.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{town, Table, TOWN};
use serde_json::Value as J;

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, EffectInstance};
use anime_compositor::model::Id;
use anime_compositor::{persist, png_out, OutputDepth};

fn dmap(layer: J, fit: &str, horizontal: &str, max_horizontal: f64, vertical: &str, max_vertical: f64, wrap: &str) -> Effect {
    Effect::DisplacementMap {
        layer,
        fit: fit.to_string(),
        horizontal: horizontal.to_string(),
        max_horizontal,
        vertical: vertical.to_string(),
        max_vertical,
        wrap: wrap.to_string(),
        expand: "off".to_string(),
        map: None,
    }
}

/// The ramp, red across and green down, at most `across` and `down`.
fn ramp(across: f64, down: f64) -> Effect {
    dmap(J::from("ramp"), "stretch", "red", across, "green", down, "off")
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

/// A Displacement Map naming `named`, as added otherwise.
fn naming(named: &str) -> Effect {
    dmap(J::from(named), "stretch", "red", 5.0, "green", 5.0, "off")
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

/// A map of waves: red in bands down the picture, green in waves across it.
fn waves() -> Vec<u8> {
    let (w, h) = TOWN;
    let tau = std::f64::consts::TAU;
    let byte = |v: f64| (255.0 * v).round() as u8;
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let (x, y) = (x as f64, y as f64);
            let r = 0.5 + 0.5 * (tau * y / 48.0).sin();
            let g = 0.5 + 0.5 * (tau * x / 70.0 + y / 40.0).sin();
            bytes.extend([byte(r), byte(g), 128, 255]);
        }
    }
    bytes
}

/// The glass ball: its middle and size, in pixels.
const BALL: (f64, f64, f64) = (480.0 * 0.62, 270.0 * 0.55, 270.0 * 0.38);

/// A map of a glass ball: inside it red and green point back to its middle, and outside it the
/// map is clear.
fn ball() -> Vec<u8> {
    let (w, h) = TOWN;
    let (cx, cy, r) = BALL;
    let byte = |v: f64| (255.0 * v.clamp(0.0, 1.0)).round() as u8;
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let (px, py) = (x as f64 + 0.5, y as f64 + 0.5);
            let inside = (px - cx).hypot(py - cy) <= r;
            bytes.extend([byte(0.5 - 0.5 * (px - cx) / r), byte(0.5 - 0.5 * (py - cy) / r), 128, if inside { 255 } else { 0 }]);
        }
    }
    bytes
}

/// The street as the holder and `map` as the switched-off `ramp` layer, with `effects` on the
/// street, drawn at frame 0.
fn picture(dir: &Path, map: &str, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/displacement_map/fx_dmap_002.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    project["assets"][1]["path"] = J::from(map);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    for layer in comp["layers"].as_array_mut().unwrap() {
        if layer["id"] == "holder" {
            layer["effects"] = effects.clone();
        }
    }
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &main_comp(), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
}

#[test]
fn b128_displacement_map() {
    let mut t = Table::new(
        "displacement_map",
        "# B-128: displacement map\n\nD-193, accepted by the owner on 2026-09-28 (\"take \
         everything\"). Every expected pixel is \
         `Fixtures/displacement_map/expected_displacement_map.json`, written by \
         `tools/displacement_map_reference.py` before this code existed and printed in document \
         25 as FX-DMAP-001 to 031. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-DMAP-001 to 031 (document 25)");
    t.fixtures_numbered("expected_displacement_map.json", 1..=31);

    t.heading("Files whose layers read each other (document 25)");
    let expected: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/displacement_map/expected_displacement_map.json")).unwrap(),
    )
    .unwrap();
    for (file, case) in expected["loads"].as_object().unwrap() {
        let says = case["says"].as_str().unwrap();
        let id = case["refused"].as_str().unwrap();
        let got = persist::load(&effect_table::repo(&format!("Fixtures/displacement_map/{file}"))).err();
        t.row(
            &format!("{file}: {says}"),
            &got.as_ref().map_or("opened".to_string(), |d| format!("{} {}", d.id.as_str(), d.message)),
            got.is_some_and(|d| d.id.as_str() == id),
        );
    }

    t.heading("How far it reaches");
    let got = ramp(1000.0, -1000.0).bounds_expansion();
    t.row("maxima of 1000 and -1000, the most, never grow the drawing's bounds", &got.to_string(), got == 0);
    let mut draft = ramp(20.0, -8.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves both maxima, and nothing else",
        &format!("{draft:?}"),
        draft == ramp(10.0, -4.0),
    );

    t.heading("The file");
    let files: Vec<String> = (1..=31).map(|n| format!("fx_dmap_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let params = t.saved_parameters("fx_dmap_031.json");
    t.row(
        "fx_dmap_031.json, its layer written as the number 3, is saved as the number 3",
        &params["layer"].to_string(),
        params["layer"] == serde_json::json!(3),
    );
    let params = t.saved_parameters("fx_dmap_002.json");
    t.row(
        "the map the effect reads is never saved",
        &format!("{:?}", params.get("map")),
        params.get("map").is_none(),
    );
    for (what, parameters) in [
        (
            "no `fit` at all",
            r#"{"layer": "ramp", "horizontal": "red", "max_horizontal": 2, "vertical": "green", "max_vertical": 2, "wrap": "off"}"#,
        ),
        (
            "no `layer` at all",
            r#"{"fit": "stretch", "horizontal": "red", "max_horizontal": 2, "vertical": "green", "max_vertical": 2, "wrap": "off"}"#,
        ),
        (
            "no `wrap` at all",
            r#"{"layer": "ramp", "fit": "stretch", "horizontal": "red", "max_horizontal": 2, "vertical": "green", "max_vertical": 2}"#,
        ),
        (
            "a maximum across that is a word",
            r#"{"layer": "ramp", "fit": "stretch", "horizontal": "red", "max_horizontal": "far", "vertical": "green", "max_vertical": 2, "wrap": "off"}"#,
        ),
    ] {
        t.shape_refused("fx_dmap_002.json", what, parameters);
    }

    t.heading("Commands");
    let mut document = t.load("fx_dmap_002.json").document;
    let word = |h: &str, v: &str, fit: &str, wrap: &str| dmap(J::from("ramp"), fit, h, 2.0, v, 2.0, wrap);
    refused(
        &mut t,
        &mut document,
        vec![
            ("Max Horizontal Displacement 1001", set("holder", "fx-1", ramp(1001.0, 2.0)), None),
            ("Max Vertical Displacement -1001", set("holder", "fx-1", ramp(2.0, -1001.0)), None),
            ("across \"half\"", set("holder", "fx-1", word("half", "green", "stretch", "off")), None),
            ("down \"purple\"", set("holder", "fx-1", word("red", "purple", "stretch", "off")), None),
            ("fit \"fill\"", set("holder", "fx-1", word("red", "green", "fill", "off")), None),
            ("wrap \"yes\"", set("holder", "fx-1", word("red", "green", "stretch", "yes")), None),
            (
                "a layer that is the number 3",
                set("holder", "fx-1", dmap(J::from(3), "stretch", "red", 2.0, "green", 2.0, "off")),
                None,
            ),
            (
                "a Displacement Map on `ramp` naming `holder`, which names `ramp`",
                add("ramp", "fx-2", naming("holder")),
                Some("EFFECT_LAYER_CYCLE"),
            ),
            (
                "a Compound Blur on `ramp` naming `holder`, whose Displacement Map names `ramp`",
                add(
                    "ramp",
                    "fx-2",
                    Effect::CompoundBlur {
                        layer: J::from("holder"),
                        fit: "stretch".to_string(),
                        max_blur: 6.0,
                        invert: "off".to_string(),
                        edges: "transparent".to_string(),
                        map: None,
                    },
                ),
                Some("EFFECT_LAYER_CYCLE"),
            ),
        ],
    );
    let taken = [
        ("naming the holder itself, which is no circle,", set("holder", "fx-1", naming("holder"))),
        ("luminance across instead of red", set("holder", "fx-1", word("luminance", "green", "stretch", "off"))),
        ("a Displacement Map on `ramp` naming `white`", add("ramp", "fx-2", naming("white"))),
        ("a Displacement Map on `white` naming no layer", add("white", "fx-3", naming(""))),
    ];
    let n = taken.len();
    for (what, command) in taken {
        let taken = document.apply(command).is_ok();
        t.row(&format!("{what} is taken"), if taken { "taken" } else { "refused" }, taken);
    }
    let every = ["red", "green", "blue", "alpha", "luminance", "hue", "lightness", "saturation", "full", "off"];
    let all_taken = every.iter().all(|w| {
        let taken = document.apply(set("holder", "fx-1", word(w, w, "tile", "on"))).is_ok();
        document.undo();
        taken
    });
    t.row(
        "each of the ten channel words, across and down, with Tile Map and Wrap on, is taken",
        &every.join(", "),
        all_taken,
    );
    refused(
        &mut t,
        &mut document,
        vec![(
            "`white`'s Displacement Map changed to name `ramp`, which names `white`",
            set("white", "fx-3", naming("ramp")),
            Some("EFFECT_LAYER_CYCLE"),
        )],
    );
    let before = t.render(&t.load("fx_dmap_002.json").document, 0, 64);
    for _ in 0..n {
        document.undo();
    }
    let same = t.render(&document, 0, 64).data() == before.data();
    t.row(&format!("undo {n} times: frame 0 is the frame it was"), if same { "byte-identical" } else { "differ" }, same);

    t.heading("Pictures: B-127's street, in `verification/B-128 pictures/`");
    let dir = effect_table::repo("verification/B-128 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    png_out::write_rgba(&dir.join("town.png"), w, h, OutputDepth::Eight, &[], &town()).unwrap();
    png_out::write_rgba(&dir.join("map_waves.png"), w, h, OutputDepth::Eight, &[], &waves()).unwrap();
    png_out::write_rgba(&dir.join("map_ball.png"), w, h, OutputDepth::Eight, &[], &ball()).unwrap();
    let (before, said) = picture(&dir, "map_waves.png", J::Array(vec![]));
    png_out::write_rgba(&dir.join("before.png"), w, h, OutputDepth::Eight, &[], &before).unwrap();
    t.row("before.png, the street with no effect, draws cleanly", &format!("{said:?}"), said.is_empty());
    let alpha = |bytes: &[u8], i: usize| bytes[i * 4 + 3];
    let changed = |bytes: &[u8], i: usize| bytes[i * 4..i * 4 + 4] != before[i * 4..i * 4 + 4];
    let count = |f: &dyn Fn(usize) -> bool| (0..w * h).filter(|&i| f(i)).count();
    let rim = |i: usize| i % w == 0 || i % w == w - 1 || i / w == 0 || i / w == h - 1;
    let (cx, cy, r) = BALL;
    let from_ball = |i: usize| ((i % w) as f64 + 0.5 - cx).hypot((i / w) as f64 + 0.5 - cy) / r;
    // A quarter-size picture takes a quarter of the distance, as a quarter-size draft does: 3
    // here is 12 at full size.
    for (name, map, h_word, v_word, most, wrap, what) in [
        ("waves", "map_waves.png", "red", "green", 3.0, "off", "the waves, red across and green down at most 3: the street ripples, and clear comes in at the edges"),
        ("waves_wrapped", "map_waves.png", "red", "green", 10.0, "on", "the waves at most 10 with Wrap Pixels Around: stronger, and the edges filled from the far side"),
        ("glass_ball", "map_ball.png", "red", "green", 0.4 * r, "off", "the glass ball, at most 40% of its size: the street magnified inside it and untouched outside"),
        ("heat_haze", "", "luminance", "off", 4.0, "off", "the street naming itself, luminance across at most 4: bright parts pushed sideways more than dark"),
    ] {
        let layer = if map.is_empty() { "holder" } else { "ramp" };
        let effects = serde_json::json!([{
            "instance_id": "fx-1", "type_id": "core.displacement_map", "enabled": true,
            "parameters": {"layer": layer, "fit": "stretch", "horizontal": h_word, "max_horizontal": most,
                           "vertical": v_word, "max_vertical": most, "wrap": wrap},
        }]);
        let (bytes, said) = picture(&dir, if map.is_empty() { "map_waves.png" } else { map }, effects);
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let moved = count(&|i| changed(&bytes, i));
        let clear_rim = count(&|i| rim(i) && alpha(&bytes, i) < 255);
        let (check, ok) = match name {
            "waves" => (format!("{moved} pixels changed, {clear_rim} rim pixels less than opaque"), moved > w * h / 10 && clear_rim > 0),
            "waves_wrapped" => (
                format!("{moved} pixels changed, {} pixels less than opaque", count(&|i| alpha(&bytes, i) < 255)),
                moved > w * h / 10 && count(&|i| alpha(&bytes, i) < 255) == 0,
            ),
            "glass_ball" => {
                let outside = count(&|i| from_ball(i) > 1.0 && changed(&bytes, i));
                let inside = count(&|i| from_ball(i) < 0.9 && changed(&bytes, i));
                (format!("{outside} pixels outside the ball changed, {inside} inside it"), outside == 0 && inside > 0)
            }
            _ => (format!("{moved} pixels changed"), moved > 0),
        };
        t.row(&format!("{name}.png, {what}, draws cleanly"), &format!("{said:?}, {check}"), said.is_empty() && ok);
    }

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_dmap_002.json", 0),
        ("fx_dmap_008.json", 0),
        ("fx_dmap_013.json", 0),
        ("fx_dmap_015.json", 0),
        ("fx_dmap_022.json", 0),
        ("fx_dmap_023.json", 0),
    ]);

    t.finish("B-128_displacement_map_table.md");
}
