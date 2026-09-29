//! B-137: Mix on every effect, against D-202.
//!
//! Writes `verification/B-137_effect_mix_table.md`.
//!
//! Every expected pixel is `Fixtures/effect_mix/expected_effect_mix.json`, written by
//! `tools/effect_mix_reference.py` before this code existed and printed in document 25 as
//! FX-MIX-001 to 017. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws a street with an Invert and a Gaussian Blur at several Mixes, and an Invert
//! whose Mix is keyed from 0 to 100, into `verification/B-137 pictures/`.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, same_json, saved, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth, WorkingBuffer};

/// Set the Mix of `instance` on `layer`.
fn mix(layer: &str, instance: &str, mix: f64) -> Command {
    Command::SetEffectMix {
        composition: Id::new(MAIN),
        layer_id: Id::new(layer),
        instance_id: Id::new(instance),
        mix,
    }
}

fn fixture(t: &Table, file: &str) -> J {
    serde_json::from_str(&fs::read_to_string(t.root.join(file)).unwrap()).unwrap()
}

/// One effect as a layer's `effects`, with a Mix when one is given.
fn one(type_id: &str, parameters: J, mix: Option<J>) -> J {
    let mut e = json!({"instance_id": "fx-0-0", "type_id": type_id, "enabled": true, "parameters": parameters});
    if let Some(m) = mix {
        e["mix"] = m;
    }
    json!([e])
}

/// The street as the composition's one layer, with `effects`, at `frame`; and what it warned of.
fn street(dir: &Path, effects: J, frame: i32) -> (WorkingBuffer, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/effect_mix/fx_mix_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the street's project reads");
    let mut log = FrameLog::new(3);
    let buffer = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log)
        .expect("the street draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (buffer, said)
}

/// The largest difference between `got` and `a + m (b - a)`, sample by sample.
fn off_the_line(got: &WorkingBuffer, a: &WorkingBuffer, b: &WorkingBuffer, m: f32) -> f32 {
    got.data()
        .iter()
        .zip(a.data().iter().zip(b.data()))
        .map(|(g, (a, b))| (g - (a + m * (b - a))).abs())
        .fold(0.0, f32::max)
}

/// Panels side by side, a dark bar of 4 pixels between them.
fn strip(panels: &[Vec<u8>]) -> (Vec<u8>, usize) {
    let (w, h) = TOWN;
    let wide = panels.len() * w + (panels.len() - 1) * 4;
    let mut out = Vec::with_capacity(wide * h * 4);
    for y in 0..h {
        for (i, p) in panels.iter().enumerate() {
            if i > 0 {
                (0..4).for_each(|_| out.extend([24, 24, 28, 255]));
            }
            out.extend_from_slice(&p[y * w * 4..(y + 1) * w * 4]);
        }
    }
    (out, wide)
}

/// Every effect record in a file, and how many of them write a Mix.
fn mixes(j: &J) -> (usize, usize) {
    match j {
        J::Object(map) => {
            let here = map.contains_key("instance_id") && map.contains_key("type_id");
            map.values().map(mixes).fold(
                (here as usize, (here && map.contains_key("mix")) as usize),
                |(a, b), (c, d)| (a + c, b + d),
            )
        }
        J::Array(items) => items.iter().map(mixes).fold((0, 0), |(a, b), (c, d)| (a + c, b + d)),
        _ => (0, 0),
    }
}

/// How many layers of the card's plan have an effect left for the card.
fn left_to_card(project: &Project, root: &Path) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(
        project,
        &Id::new(MAIN),
        0,
        root,
        PreviewQuality::Full,
        &mut log,
        &mut CelCache::viewer(),
    )
    .expect("plan the frame");
    plan.layers.iter().filter(|l| l.on_card.is_some()).count()
}

#[test]
fn b137_effect_mix() {
    let mut t = Table::new(
        "effect_mix",
        "# B-137: Mix on every effect\n\nD-202, accepted on 2026-09-28 with the After Effects \
         picks (A12). Every expected pixel is `Fixtures/effect_mix/expected_effect_mix.json`, \
         written by `tools/effect_mix_reference.py` before this code existed and printed in \
         document 25 as FX-MIX-001 to 017. The build's frame is compared sample by sample; the \
         answer is the largest difference over all of them, against the catalogue's tolerance of \
         2e-5.\n",
    );

    t.heading("FX-MIX-001 to 017 (document 25)");
    t.fixtures("expected_effect_mix.json");

    t.heading("The file");
    t.round_trips(&[
        "fx_mix_001.json",
        "fx_mix_002.json",
        "fx_mix_004.json",
        "fx_mix_005.json",
        "fx_mix_007.json",
        "fx_mix_008.json",
        "fx_mix_010.json",
        "fx_mix_012.json",
        "fx_mix_013.json",
        "fx_mix_014.json",
        "fx_mix_015.json",
        "fx_mix_016.json",
        "fx_mix_017.json",
    ]);
    let three = saved(&t.load("fx_mix_003.json"));
    let two = saved(&t.load("fx_mix_002.json"));
    t.row(
        "fx_mix_003.json, Mix 100 written, is saved as FX-MIX-002 is, with no Mix: a plain 100 is \
         never written",
        &format!("its layers the same as fx_mix_002.json's saved: {}", same_json(&three["compositions"], &two["compositions"])),
        same_json(&three["compositions"], &two["compositions"]) && mixes(&three) == (1, 0),
    );
    for (what, value) in [
        ("a Mix written as a word", json!("half")),
        ("a Mix that is true", json!(true)),
        ("a Mix with an expression", json!({"base": 50, "expression": "time * 10"})),
        ("a Mix whose base is a word", json!({"base": "half", "keyframes": []})),
    ] {
        let mut j = fixture(&t, "fx_mix_001.json");
        j["compositions"][0]["layers"][0]["effects"][0]["mix"] = value;
        let refused = persist::load_str(&j.to_string()).err();
        t.row(
            &format!("a file with {what} is refused as a fault in its shape"),
            &refused.as_ref().map_or("opened".to_string(), |d| format!("{}: {}", d.id.as_str(), d.message)),
            refused.is_some(),
        );
    }
    for (what, value) in [
        ("a Mix of 30", json!(30)),
        (
            "a Mix keyed from 30 to 150",
            json!({"base": 30, "keyframes": [{"frame": 0, "value": 30, "interp": "linear"}, {"frame": 4, "value": 150, "interp": "linear"}]}),
        ),
    ] {
        let mut j = fixture(&t, "fx_mix_001.json");
        let e = &mut j["compositions"][0]["layers"][0]["effects"][0];
        e["type_id"] = json!("vendor.soft_focus");
        e["parameters"] = json!({"radius": 3});
        e["mix"] = value;
        let loaded = persist::load_str(&j.to_string()).expect("a file with an effect this build does not have opens");
        let kept = same_json(&saved(&loaded), &j);
        t.row(
            &format!(
                "an effect this build does not have, with {what}, opened and saved holds what it \
                 held, the Mix included"
            ),
            if kept { "the same" } else { "differs" },
            kept,
        );
    }
    // Every project in `Fixtures/` but these, as the earlier builds read them.
    let (mut files, mut records, mut gained) = (0, 0, Vec::new());
    let mut folders: Vec<_> = fs::read_dir(effect_table::repo("Fixtures")).unwrap().map(|e| e.unwrap().path()).collect();
    folders.sort();
    for folder in folders.iter().filter(|f| f.is_dir() && !f.ends_with("effect_mix")) {
        let mut paths: Vec<_> = fs::read_dir(folder).unwrap().map(|e| e.unwrap().path()).collect();
        paths.sort();
        for path in paths.iter().filter(|p| p.extension().is_some_and(|e| e == "json")) {
            let Some(original) = fs::read_to_string(path).ok().and_then(|s| serde_json::from_str::<J>(&s).ok()) else {
                continue;
            };
            if original.get("schema_version").is_none() {
                continue;
            }
            let Ok(loaded) = persist::load(path) else { continue };
            files += 1;
            let (n, written) = mixes(&saved(&loaded));
            records += n;
            if written != mixes(&original).1 {
                gained.push(path.display().to_string());
            }
        }
    }
    t.row(
        "every other project in Fixtures/ that opens, opened and saved, writes a Mix on no effect \
         that did not have one: every file before D-202 is saved as it was",
        &format!("{files} files, {records} effects, {} gained a Mix {gained:?}", gained.len()),
        files > 0 && records > 0 && gained.is_empty(),
    );

    t.heading("Commands");
    let mut document = t.load("fx_mix_002.json").document;
    t.refused(
        &mut document,
        vec![
            ("Mix 101", mix("art", "fx-0-0", 101.0)),
            ("Mix -1", mix("art", "fx-0-0", -1.0)),
            ("Mix not a number", mix("art", "fx-0-0", f64::NAN)),
            ("Mix keyed from 0 to 150", keys("mix", &[(0, &[0.0]), (4, &[150.0])])),
            ("Mix keyed with two numbers a key", keys("mix", &[(0, &[0.0, 0.0]), (4, &[100.0, 100.0])])),
            ("Mix on an effect the layer does not have", mix("art", "fx-9-9", 50.0)),
        ],
    );
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_effect_mix.json")).unwrap()).unwrap();
    document.apply(mix("art", "fx-0-0", 50.0)).expect("Mix 50 is taken");
    let d = effect_table::largest_difference(&t.render(&document, 0, 64), &expected["cases"]["FX-MIX-001"]["frames"]["0"]);
    let label = document.undo_labels().last().copied().unwrap_or_default().to_string();
    t.row(
        "fx_mix_002.json, the Invert at 100, set to Mix 50 draws FX-MIX-001, and the step to undo \
         says what it was",
        &format!("largest difference {d:.1e}; \"{label}\""),
        d <= 2e-5 && label.contains("Mix to 50%"),
    );
    document.undo();
    let depth = document.undo_depth();
    document.begin_drag().unwrap();
    for m in [80.0, 60.0, 40.0] {
        document.update_drag(mix("art", "fx-0-0", m)).unwrap();
    }
    document.end_drag();
    let one_step = document.undo_depth() == depth + 1;
    document.undo();
    let back = t.render(&document, 0, 64).data() == t.render(&t.load("fx_mix_002.json").document, 0, 64).data();
    t.row(
        "a drag of the Mix from 100 through 80 and 60 to 40 is one step to undo, and undoing it \
         gives back the frame at 100",
        &format!("steps added {}; frame back: {back}", document.undo_depth() + 1 - depth),
        one_step && back,
    );
    let mut document = t.load("fx_mix_002.json").document;
    t.taken(
        &mut document,
        "fx_mix_002.json",
        vec![
            ("Mix 50,", mix("art", "fx-0-0", 50.0)),
            ("Mix keyed from 0 to 100,", keys("mix", &[(0, &[0.0]), (4, &[100.0])])),
            ("Mix 0, with keys on it,", mix("art", "fx-0-0", 0.0)),
        ],
    );
    let mut posterize = t.load("fx_mix_017.json").document;
    for (what, command) in [
        ("Mix 100 on Posterize Time", mix("holder", "fx-1", 100.0)),
        ("Mix 50 on Posterize Time", mix("holder", "fx-1", 50.0)),
        (
            "keys on Posterize Time's Mix",
            Command::SetEffectKeys {
                composition: Id::new(MAIN),
                layer_id: Id::new("holder"),
                instance_id: Id::new("fx-1"),
                setting: "mix".into(),
                keys: Vec::new(),
            },
        ),
    ] {
        let refused = posterize.apply(command).err();
        t.row(
            &format!("{what} is refused with a sentence: it has no Mix"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some(),
        );
    }
    let mut j = fixture(&t, "fx_mix_001.json");
    j["compositions"][0]["layers"][0]["effects"][0]["type_id"] = json!("vendor.soft_focus");
    let mut unknown = persist::load_str(&j.to_string()).unwrap().document;
    let refused = unknown.apply(mix("art", "fx-0-0", 50.0)).err();
    t.row(
        "Mix 50 on an effect this build does not have is refused with a sentence: it is kept as \
         it was written",
        &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
        refused.is_some(),
    );

    t.heading("The preview's memory and the graphics card");
    let root = t.root.clone();
    let mut document = t.load("fx_mix_006.json").document;
    let mut cache = CelCache::viewer();
    let preview = |document: &Document, cache: &mut CelCache| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(document.project(), &Id::new(MAIN), 0, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, cache)
            .expect("the preview draws")
            .to_srgb8_straight()
    };
    let at_50 = preview(&document, &mut cache);
    document.apply(mix("art", "fx-0-0", 100.0)).unwrap();
    let remembered = preview(&document, &mut cache);
    let fresh = preview(&document, &mut CelCache::viewer());
    t.row(
        "fx_mix_006.json, the Gaussian Blur at Mix 50, drawn by the preview, then set to 100 and \
         drawn again with what the preview remembers: the second is the blur at 100, not the \
         remembered 50",
        &format!("same as a fresh 100: {}; same as the 50: {}", remembered == fresh, remembered == at_50),
        remembered == fresh && remembered != at_50,
    );
    let mixed = t.load("fx_mix_006.json").document;
    let (card_50, card_100) = (left_to_card(mixed.project(), &root), left_to_card(document.project(), &root));
    t.row(
        "the Gaussian Blur at Mix 50 is drawn by the CPU, not left to the card, which does not mix \
         yet; at 100 it is left to the card as before",
        &format!("left to the card at 50: {card_50}; at 100: {card_100}"),
        card_50 == 0 && card_100 == 1,
    );
    match Gpu::new() {
        Err(why) => t.row("the graphics card's checks", &format!("not run: no usable card, {why}"), false),
        Ok(mut gpu) => {
            for (file, mixed) in [("fx_mix_006.json", true), ("fx_mix_013.json", true), ("fx_mix_002.json", false)] {
                let mut j = fixture(&t, file);
                let mut project = t.load(file).document.project().clone();
                if !mixed {
                    j["compositions"][0]["layers"][0]["effects"] = one("core.gaussian_blur", json!({"sigma_px": 1, "edges": "transparent"}), None);
                    project = persist::load_str(&j.to_string()).unwrap().document.project().clone();
                }
                let mut cache = CelCache::viewer();
                let mut log = FrameLog::new(3);
                let cpu = preview::preview_frame_cached(&project, &Id::new(MAIN), 0, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache)
                    .unwrap()
                    .to_srgb8_straight();
                let mut log = FrameLog::new(3);
                let (card, ..) = preview::preview_frame_srgb8(&project, &Id::new(MAIN), 0, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu)
                    .unwrap();
                let said: Vec<String> = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
                let on_cpu = said.iter().any(|s| s.starts_with(DiagnosticId::GpuPreviewOnCpu.as_str()));
                let apart = cpu.iter().zip(&card).map(|(a, b)| a.abs_diff(*b)).max().unwrap_or(0);
                let (what, ok) = match file {
                    "fx_mix_006.json" => (
                        "fx_mix_006.json, the Gaussian Blur at Mix 50, through the card: the CPU blurs and mixes, the card lays it, no message, within 1 level of the CPU's frame",
                        !on_cpu && apart <= 1,
                    ),
                    "fx_mix_013.json" => (
                        "fx_mix_013.json, the Light Wrap at Mix 50, through the card: the CPU draws the whole frame, says so in the warning panel, and it is the CPU's frame exactly",
                        on_cpu && apart == 0,
                    ),
                    _ => (
                        "a Gaussian Blur with no Mix written, through the card: drawn by the card as before, no message, within 1 level of the CPU's frame",
                        !on_cpu && apart <= 1,
                    ),
                };
                t.row(what, &format!("{said:?}; largest difference {apart} of 255"), ok);
            }
        }
    }

    t.heading("Pictures: a street, in `verification/B-137 pictures/`");
    let dir = effect_table::repo("verification/B-137 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    png_out::write_rgba(&dir.join("town.png"), w, h, OutputDepth::Eight, &[], &effect_table::town()).unwrap();
    let write = |name: &str, (bytes, wide): (Vec<u8>, usize)| {
        png_out::write_rgba(&dir.join(name), wide, h, OutputDepth::Eight, &[], &bytes).unwrap()
    };
    let mut warned = Vec::new();
    let mut draw = |effects: J, frame: i32| {
        let (buffer, said) = street(&dir, effects, frame);
        warned.extend(said);
        buffer
    };
    let invert = |m: Option<J>| one("core.invert", json!({"channel": "rgb", "amount": 100}), m);
    let plain = draw(json!([]), 0);
    let full = draw(invert(None), 0);
    let invert_mixes = [0.0, 25.0, 50.0, 75.0, 100.0];
    let panels: Vec<WorkingBuffer> = invert_mixes.iter().map(|&m| draw(invert(Some(json!(m))), 0)).collect();
    let off = invert_mixes.iter().zip(&panels).map(|(&m, p)| off_the_line(p, &plain, &full, m as f32 / 100.0)).fold(0.0, f32::max);
    write("invert_mix.png", strip(&panels.iter().map(WorkingBuffer::to_srgb8_straight).collect::<Vec<_>>()));
    t.row(
        "invert_mix.png, an Invert at Mix 0, 25, 50, 75 and 100, left to right: the street, fading \
         to its negative; 0 is the street exactly and 100 the Invert with no Mix exactly; every \
         sample of each on the straight line between the two, in linear light, within 1e-6",
        &format!(
            "0 is the street: {}; 100 is the Invert: {}; furthest off the line {off:.1e}",
            panels[0].data() == plain.data(),
            panels[4].data() == full.data()
        ),
        panels[0].data() == plain.data() && panels[4].data() == full.data() && off <= 1e-6,
    );
    let blur = |m: Option<J>| one("core.gaussian_blur", json!({"sigma_px": 6, "edges": "repeat"}), m);
    let soft = draw(blur(None), 0);
    let blurs: Vec<WorkingBuffer> = [0.0, 50.0, 100.0].iter().map(|&m| draw(blur(Some(json!(m))), 0)).collect();
    let off = off_the_line(&blurs[1], &plain, &soft, 0.5);
    write("blur_mix.png", strip(&blurs.iter().map(WorkingBuffer::to_srgb8_straight).collect::<Vec<_>>()));
    t.row(
        "blur_mix.png, a Gaussian Blur of 6 pixels at Mix 0, 50 and 100: the sharp street, a soft \
         glow of the blur laid half over the sharp drawing, the blur; every sample of the middle \
         half-way between the other two, within 1e-6",
        &format!("0 is the street: {}; 100 is the blur: {}; furthest off {off:.1e}", blurs[0].data() == plain.data(), blurs[2].data() == soft.data()),
        blurs[0].data() == plain.data() && blurs[2].data() == soft.data() && off <= 1e-6,
    );
    let keyed = invert(Some(json!({"base": 0, "keyframes": [
        {"frame": 0, "value": 0, "interp": "linear"}, {"frame": 4, "value": 100, "interp": "linear"}]})));
    let frames: Vec<WorkingBuffer> = (0..5).map(|f| draw(keyed.clone(), f)).collect();
    let same = frames.iter().zip(&panels).all(|(f, p)| f.data() == p.data());
    write("invert_fade_frames.png", strip(&frames.iter().map(WorkingBuffer::to_srgb8_straight).collect::<Vec<_>>()));
    t.row(
        "invert_fade_frames.png, frames 0 to 4 of an Invert whose Mix is keyed from 0 at frame 0 to \
         100 at frame 4: the effect fading in, each frame exactly the panel of invert_mix.png with \
         the Mix it has there, 0, 25, 50, 75 and 100",
        &format!("every frame its panel: {same}"),
        same,
    );
    t.row("every picture draws cleanly", &format!("{warned:?}"), warned.is_empty());

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_mix_001.json", 0), ("fx_mix_006.json", 0), ("fx_mix_007.json", 2), ("fx_mix_012.json", 0), ("fx_mix_013.json", 0)]);

    t.finish("B-137_effect_mix_table.md");
}
