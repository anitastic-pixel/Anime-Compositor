//! B-127: compound blur in the core, against D-191.
//!
//! Writes `verification/B-127_compound_blur_table.md`.
//!
//! Every expected pixel is `Fixtures/compound_blur/expected_compound_blur.json`, written by
//! `tools/compound_blur_reference.py` before this code existed and printed in document 25 as
//! FX-CBLUR-001 to 026. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws a street at a quarter of 1920 by 1080 and a depth map for it, and blurs the
//! street five ways into `verification/B-127 pictures/`.

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

fn blur(layer: J, fit: &str, max_blur: f64, invert: &str, edges: &str) -> Effect {
    Effect::CompoundBlur {
        layer,
        fit: fit.to_string(),
        max_blur,
        invert: invert.to_string(),
        edges: edges.to_string(),
        map: None,
    }
}

fn ramp(max_blur: f64) -> Effect {
    blur(J::from("ramp"), "stretch", max_blur, "off", "transparent")
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

/// Add a Compound Blur naming `named` to the end of `layer`'s stack.
fn add(layer: &str, instance: &str, named: &str) -> Command {
    Command::AddEffect {
        composition: main_comp(),
        layer_id: Id::new(layer),
        effect: EffectInstance::new(Id::new(instance), blur(J::from(named), "stretch", 6.0, "off", "transparent")),
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

/// The depth map: black along the house fronts, rows 170 to 210, which stay sharp, whitening
/// above and below them.
fn depth() -> Vec<u8> {
    let (w, h) = TOWN;
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        let v = ((((y as f64 - 190.0).abs() - 20.0) / 70.0).clamp(0.0, 1.0) * 255.0).round() as u8;
        for _ in 0..w {
            bytes.extend([v, v, v, 255]);
        }
    }
    bytes
}

/// The street as the holder and the depth map as the switched-off `ramp` layer, with `effects`
/// on the street, drawn at frame 0.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/compound_blur/fx_cblur_002.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    project["assets"][1]["path"] = J::from("depth.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    comp["layers"][0]["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &main_comp(), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
}

#[test]
fn b127_compound_blur() {
    let mut t = Table::new(
        "compound_blur",
        "# B-127: compound blur\n\nD-191, accepted by the owner on 2026-09-28 (\"take \
         everything\"). Every expected pixel is \
         `Fixtures/compound_blur/expected_compound_blur.json`, written by \
         `tools/compound_blur_reference.py` before this code existed and printed in document 25 \
         as FX-CBLUR-001 to 026. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-CBLUR-001 to 026 (document 25)");
    t.fixtures_numbered("expected_compound_blur.json", 1..=18);
    t.fixtures_numbered("expected_compound_blur.json", 20..=26);
    // FX-CBLUR-019 says its adjustment layer is above the holder, and its file puts it beneath:
    // `layer_order` is drawn first to last, and the reference writes `[adjust] + layers`. Beneath
    // the holder it has nothing to work on. Pending D-192, proposed.
    let expected: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/compound_blur/expected_compound_blur.json")).unwrap(),
    )
    .unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();
    let pixels = &expected["cases"]["FX-CBLUR-019"]["frames"]["0"];
    let unblurred = effect_table::largest_difference(&t.render(&t.load("fx_cblur_001.json").document, 0, 64), pixels);
    let as_written = t.render(&t.load("fx_cblur_019.json").document, 0, 64);
    let plain = t.render(&t.load("fx_cblur_001.json").document, 0, 64);
    t.row(
        "FX-CBLUR-019 frame 0, in dispute (D-192, proposed): as its file is written, the adjustment \
         layer is drawn beneath the holder, so the frame is the holder unblurred, FX-CBLUR-001's",
        &format!(
            "{}; the case's pixels are {unblurred:.1e} from that",
            if as_written.data() == plain.data() { "byte-identical to FX-CBLUR-001" } else { "differs from FX-CBLUR-001" }
        ),
        as_written.data() == plain.data(),
    );
    let mut above: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/compound_blur/fx_cblur_019.json")).unwrap(),
    )
    .unwrap();
    above["compositions"][0]["layer_order"].as_array_mut().unwrap().swap(0, 1);
    let above = persist::load_str(&above.to_string()).expect("FX-CBLUR-019 above reads");
    let d = effect_table::largest_difference(&t.render(&above.document, 0, 64), pixels);
    t.row(
        "FX-CBLUR-019 frame 0 with the adjustment layer moved above the holder, as the case says it \
         is: the case's pixels",
        &format!("largest difference {d:.1e}"),
        d <= tolerance,
    );

    t.heading("Files whose layers read each other (document 25)");
    for (file, case) in expected["loads"].as_object().unwrap() {
        let says = case["says"].as_str().unwrap();
        let opened = persist::load(&effect_table::repo(&format!("Fixtures/compound_blur/{file}")));
        match (case.get("refused").and_then(J::as_str), opened) {
            (Some(id), opened) => {
                let got = opened.err();
                t.row(
                    &format!("{file}: {says}"),
                    &got.as_ref().map_or("opened".to_string(), |d| format!("{} {}", d.id.as_str(), d.message)),
                    got.is_some_and(|d| d.id.as_str() == id),
                );
            }
            (None, Ok(loaded)) => {
                for (frame, pixels) in case["frames"].as_object().unwrap() {
                    let frame: i32 = frame.parse().unwrap();
                    let d = effect_table::largest_difference(&t.render(&loaded.document, frame, 64), pixels);
                    t.row(&format!("{file} frame {frame}: {says}"), &format!("largest difference {d:.1e}"), d <= tolerance);
                }
            }
            (None, Err(d)) => t.row(&format!("{file}: {says}"), &format!("refused: {}", d.message), false),
        }
    }

    t.heading("How far it reaches");
    let got = ramp(500.0).bounds_expansion();
    t.row("Maximum Blur 500, the most, never grows the drawing's bounds", &got.to_string(), got == 0);
    let mut draft = ramp(20.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the Maximum Blur, and nothing else",
        &format!("{draft:?}"),
        draft == ramp(10.0),
    );

    t.heading("The file");
    let files: Vec<String> = (1..=26).map(|n| format!("fx_cblur_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let params = t.saved_parameters("fx_cblur_026.json");
    t.row(
        "fx_cblur_026.json, its layer written as the number 3, is saved as the number 3",
        &params["layer"].to_string(),
        params["layer"] == serde_json::json!(3),
    );
    let params = t.saved_parameters("fx_cblur_002.json");
    t.row(
        "the map the effect reads is never saved",
        &format!("{:?}", params.get("map")),
        params.get("map").is_none(),
    );
    t.shape_refused(
        "fx_cblur_002.json",
        "no `fit` at all",
        r#"{"layer": "ramp", "max_blur": 6, "invert": "off", "edges": "transparent"}"#,
    );
    t.shape_refused(
        "fx_cblur_002.json",
        "no `layer` at all",
        r#"{"fit": "stretch", "max_blur": 6, "invert": "off", "edges": "transparent"}"#,
    );
    t.shape_refused(
        "fx_cblur_002.json",
        "a Maximum Blur that is a word",
        r#"{"layer": "ramp", "fit": "stretch", "max_blur": "lots", "invert": "off", "edges": "transparent"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_cblur_002.json").document;
    refused(
        &mut t,
        &mut document,
        vec![
            ("Maximum Blur 501", set("holder", "fx-1", ramp(501.0)), None),
            ("Maximum Blur -1", set("holder", "fx-1", ramp(-1.0)), None),
            ("fit \"fill\"", set("holder", "fx-1", blur(J::from("ramp"), "fill", 6.0, "off", "transparent")), None),
            ("invert \"yes\"", set("holder", "fx-1", blur(J::from("ramp"), "stretch", 6.0, "yes", "transparent")), None),
            ("edges \"wrap\"", set("holder", "fx-1", blur(J::from("ramp"), "stretch", 6.0, "off", "wrap")), None),
            ("a layer that is the number 3", set("holder", "fx-1", blur(J::from(3), "stretch", 6.0, "off", "transparent")), None),
            (
                "a Compound Blur on `ramp` naming `holder`, which names `ramp`",
                add("ramp", "fx-2", "holder"),
                Some("EFFECT_LAYER_CYCLE"),
            ),
        ],
    );
    let n = 3;
    for (what, command) in [
        ("naming the holder itself, which is no circle,", set("holder", "fx-1", blur(J::from("holder"), "stretch", 6.0, "off", "transparent"))),
        ("a Compound Blur on `ramp` naming `white`", add("ramp", "fx-2", "white")),
        ("a Compound Blur on `white` naming no layer", add("white", "fx-3", "")),
    ] {
        let taken = document.apply(command).is_ok();
        t.row(&format!("{what} is taken"), if taken { "taken" } else { "refused" }, taken);
    }
    refused(
        &mut t,
        &mut document,
        vec![
            (
                "`white`'s Compound Blur changed to name `ramp`, which names `white`",
                set("white", "fx-3", blur(J::from("ramp"), "stretch", 6.0, "off", "transparent")),
                Some("EFFECT_LAYER_CYCLE"),
            ),
            (
                "a second Compound Blur on `white`, switched off, naming `ramp`",
                Command::AddEffect {
                    composition: main_comp(),
                    layer_id: Id::new("white"),
                    effect: EffectInstance { enabled: false, ..EffectInstance::new(Id::new("fx-4"), ramp(6.0)) },
                    index: None,
                },
                Some("EFFECT_LAYER_CYCLE"),
            ),
        ],
    );
    let before = t.render(&t.load("fx_cblur_002.json").document, 0, 64);
    for _ in 0..n {
        document.undo();
    }
    let same = t.render(&document, 0, 64).data() == before.data();
    t.row(
        &format!("undo {n} times: frame 0 is the frame it was"),
        if same { "byte-identical" } else { "differ" },
        same,
    );

    let mut document = t.load("fx_cblur_015.json").document;
    let mut gone = document.project().composition(&main_comp()).unwrap().layer(&Id::new("white")).unwrap().clone();
    gone.id = Id::new("gone");
    gone.name = "gone".to_string();
    gone.effects = vec![EffectInstance::new(Id::new("fx-9"), blur(J::from("holder"), "stretch", 6.0, "off", "transparent"))];
    refused(
        &mut t,
        &mut document,
        vec![(
            "adding a layer `gone`, which the holder names, with a Compound Blur naming the holder",
            Command::AddLayer { composition: main_comp(), layer: Box::new(gone), index: 0 },
            Some("EFFECT_LAYER_CYCLE"),
        )],
    );

    t.heading("Pictures: a street at a quarter of 1920 by 1080, in `verification/B-127 pictures/`");
    let dir = effect_table::repo("verification/B-127 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    png_out::write_rgba(&dir.join("town.png"), w, h, OutputDepth::Eight, &[], &town()).unwrap();
    png_out::write_rgba(&dir.join("depth.png"), w, h, OutputDepth::Eight, &[], &depth()).unwrap();
    let (before, said) = picture(&dir, J::Array(vec![]));
    png_out::write_rgba(&dir.join("before.png"), w, h, OutputDepth::Eight, &[], &before).unwrap();
    t.row("before.png, the street with no effect, draws cleanly", &format!("{said:?}"), said.is_empty());
    let rows_kept = |bytes: &[u8], rows: std::ops::RangeInclusive<usize>| {
        rows.clone().all(|y| bytes[y * w * 4..(y + 1) * w * 4] == before[y * w * 4..(y + 1) * w * 4])
    };
    let rows_changed = |bytes: &[u8], rows: std::ops::RangeInclusive<usize>| {
        rows.clone().any(|y| bytes[y * w * 4..(y + 1) * w * 4] != before[y * w * 4..(y + 1) * w * 4])
    };
    let alpha = |bytes: &[u8], (x, y): (usize, usize)| bytes[(y * w + x) * 4 + 3];
    // A quarter-size picture takes a quarter of the blur, as a quarter-size draft does: 10 here
    // is 40 at full size.
    for (name, layer, invert, edges, what) in [
        ("depth_of_field", "ramp", "off", "repeat", "the depth map, Maximum Blur 10, edges repeated: the house fronts sharp, the sky and the road blurred"),
        ("depth_inverted", "ramp", "on", "repeat", "the same, inverted: the house fronts blurred, the sky above row 100 sharp"),
        ("edges_transparent", "ramp", "off", "transparent", "the depth map with the edges left transparent: the frame's rim fades into clear"),
        ("own_brightness", "holder", "off", "repeat", "the street naming itself: bright sky and lit windows blurred most, the dark road least"),
    ] {
        let effects = serde_json::json!([{
            "instance_id": "fx-1", "type_id": "core.compound_blur", "enabled": true,
            "parameters": {"layer": layer, "fit": "stretch", "max_blur": 10, "invert": invert, "edges": edges},
        }]);
        let (bytes, said) = picture(&dir, effects);
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let (check, ok) = match name {
            "depth_of_field" => (
                format!("rows 170 to 210 kept {}, sky changed {}, corner alpha {}", rows_kept(&bytes, 170..=210), rows_changed(&bytes, 0..=60), alpha(&bytes, (0, 0))),
                rows_kept(&bytes, 170..=210) && rows_changed(&bytes, 0..=60) && alpha(&bytes, (0, 0)) == 255,
            ),
            "depth_inverted" => (
                format!("rows 0 to 100 kept {}, house fronts changed {}", rows_kept(&bytes, 0..=100), rows_changed(&bytes, 170..=210)),
                rows_kept(&bytes, 0..=100) && rows_changed(&bytes, 170..=210),
            ),
            "edges_transparent" => (
                format!("rows 170 to 210 kept {}, corner alpha {}", rows_kept(&bytes, 170..=210), alpha(&bytes, (0, 0))),
                rows_kept(&bytes, 170..=210) && alpha(&bytes, (0, 0)) < 255,
            ),
            _ => (format!("houses changed {}", rows_changed(&bytes, 110..=200)), rows_changed(&bytes, 110..=200)),
        };
        t.row(
            &format!("{name}.png, {what}, draws cleanly"),
            &format!("{said:?}, {check}"),
            said.is_empty() && ok,
        );
    }

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_cblur_002.json", 0), ("fx_cblur_008.json", 0), ("fx_cblur_011.json", 0), ("fx_cblur_013.json", 0), ("fx_cblur_019.json", 0)]);

    t.finish("B-127_compound_blur_table.md");
}
