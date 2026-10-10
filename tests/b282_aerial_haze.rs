//! B-282: D-403, Aerial Haze under `core.aerial_haze`, PLUGINS.md's pick #8: each pixel moved
//! toward a haze colour by an amount, evenly or through a matte layer (D-189's layer setting).
//!
//! Writes `verification/D-403_aerial_haze_table.md` and draws pictures into
//! `verification/D-403 pictures/`.
//!
//! Every expected pixel is `Fixtures/aerial_haze/expected_aerial_haze.json`, written by
//! `tools/aerial_haze_reference.py` before this code existed and printed in document 25 as
//! FX-HAZE-001 to 025. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{repo, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::command::Command;
use anime_compositor::effects::{Effect, EffectKey};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Interp, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

fn distance(a: &[u8], b: &[u8]) -> (u8, usize) {
    assert_eq!(a.len(), b.len(), "the two pictures are different sizes");
    let (mut largest, mut pixels) = (0, 0);
    for (p, q) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
        if p[3] == 0 && q[3] == 0 {
            continue;
        }
        let d = p.iter().zip(q).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0);
        largest = largest.max(d);
        pixels += (d > 0) as usize;
    }
    (largest, pixels)
}

fn said(log: FrameLog) -> String {
    let mut ids: Vec<&str> = log.finish().iter().map(|d| d.id.as_str()).collect();
    ids.sort();
    ids.dedup();
    ids.join(", ")
}

/// The Aerial Hazes the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::AerialHaze { .. })))
        .count()
        // An adjustment layer's run is made from its stack when the card draws it.
        + plan.layers
            .iter()
            .filter_map(|l| l.adjust.as_ref())
            .filter_map(|stack| compose::adjust_run(stack, (plan.width, plan.height)))
            .flatten()
            .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::AerialHaze { .. })))
            .count()
}

/// The processor and the card, each drawing the frame the page receives: the largest
/// difference, the pixels differing, whether the card refused the frame, and each one's warnings.
fn both(gpu: &mut Gpu, project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> ((u8, usize), bool, String, String) {
    let mut cache = CelCache::viewer();
    let mut log = FrameLog::new(3);
    let c = preview::preview_frame_cached(project, comp, frame, root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache)
        .unwrap_or_else(|d| panic!("frame {frame} on the CPU: {}", d.message));
    let said_cpu = said(log);
    let mut log = FrameLog::new(3);
    let (g, ..) = preview::preview_frame_srgb8(project, comp, frame, root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, gpu)
        .unwrap_or_else(|d| panic!("frame {frame} on the GPU: {}", d.message));
    let said_gpu = said(log);
    let refused = said_gpu.contains(DiagnosticId::GpuPreviewOnCpu.as_str());
    (distance(&c.to_srgb8_straight(), &g), refused, said_cpu, said_gpu)
}

/// The reference shot (1920 by 1080) with `stack` on its first three layers.
fn reference(stack: impl Fn(&str) -> J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        layers[i]["effects"] = stack(id);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

/// An Aerial Haze with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"haze_color": "#b4c8dc", "amount": 30, "layer": "", "fit": "stretch"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.aerial_haze", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning or stay on the processor.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=25 {
        let file = format!("fx_haze_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality);
            }
        }
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor; `want` of 3 on the card each frame.
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J, usize)]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p, want) in settings {
        let project = reference(|id| json!([fx(&format!("b282-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Aerial Haze {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Aerial Haze {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the still `asset` (in `dir`, `size` pixels), drawn on the processor, straight
/// 8-bit, with what it warned of.
fn picture(dir: &Path, asset: &str, size: (usize, usize), effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from(asset);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(size.0);
    comp["height"] = J::from(size.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([size.0 as f64 / 2.0, size.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// Set the `holder` layer's first effect (the fixtures' own names).
fn set_holder(effect: Effect) -> Command {
    Command::SetEffectParameters { composition: Id::new(MAIN), layer_id: Id::new("holder"), instance_id: Id::new("fx-1"), effect }
}

/// Key a setting of the `holder` layer's first effect, linearly.
fn keys_holder(setting: &str, values: &[(i32, f64)]) -> Command {
    Command::SetEffectKeys {
        composition: Id::new(MAIN),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-1"),
        setting: setting.to_string(),
        keys: values.iter().map(|&(frame, v)| EffectKey { frame, value: vec![v], interp: Interp::Linear }).collect(),
    }
}

/// The street with `effects`, and a hidden layer `matte` showing `matte.png` (from `dir`).
fn picture_matte(dir: &Path, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let mut asset = project["assets"][0].clone();
    asset["id"] = J::from("asset-matte");
    asset["name"] = J::from("matte");
    asset["path"] = J::from("matte.png");
    project["assets"].as_array_mut().unwrap().push(asset);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    let layer = &mut comp["layers"][0];
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let mut matte = layer.clone();
    matte["id"] = J::from("matte");
    matte["name"] = J::from("matte");
    matte["asset_id"] = J::from("asset-matte");
    matte["enabled"] = J::from(false);
    matte["effects"] = json!([]);
    comp["layers"].as_array_mut().unwrap().insert(0, matte);
    comp["layer_order"].as_array_mut().unwrap().push(J::from("matte"));
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

#[test]
fn b282_aerial_haze() {
    let mut t = Table::new(
        "aerial_haze",
        "# D-403: Aerial Haze\n\nB-282: `core.aerial_haze`, PLUGINS.md's pick #8: each pixel moved \
         toward a haze colour by an amount, evenly or by the brightness of a matte layer (D-189's \
         layer setting, on the card through `map_texture`). Every expected pixel is \
         `Fixtures/aerial_haze/expected_aerial_haze.json`, written by \
         `tools/aerial_haze_reference.py` before this code existed and printed in document 25 as \
         FX-HAZE-001 to 025. Tolerance 2e-5.\n",
    );

    t.heading("FX-HAZE-001 to 025 (document 25)");
    t.fixtures_numbered("expected_aerial_haze.json", 1..=25);

    t.heading("Files whose layers read each other (document 25)");
    let expected: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/aerial_haze/expected_aerial_haze.json")).unwrap()).unwrap();
    for (file, case) in expected["loads"].as_object().unwrap() {
        let says = case["says"].as_str().unwrap();
        let id = case["refused"].as_str().unwrap();
        let got = persist::load(&t.root.join(file)).err();
        t.row(
            &format!("{file}: {says}"),
            &got.as_ref().map_or("opened".to_string(), |d| format!("{} {}", d.id.as_str(), d.message)),
            got.is_some_and(|d| d.id.as_str() == id),
        );
    }

    t.heading("The file");
    // 005, its colour in capitals, is saved in small letters (the row after).
    let files: Vec<String> = (1..=25).filter(|n| *n != 5).map(|n| format!("fx_haze_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let caps = t.saved_parameters("fx_haze_005.json");
    t.row(
        "fx_haze_005.json, its colour written in capitals, is saved in small letters, as Gradient Map's are",
        &caps["haze_color"].to_string(),
        caps["haze_color"] == "#b4c8dc",
    );
    let saved = t.saved_parameters("fx_haze_011.json");
    t.row(
        "fx_haze_011.json is saved with its four settings, the layer as written",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 4 && saved["layer"] == "card" && saved["fit"] == "center" && saved["amount"] == 100.0,
    );
    for (file, want) in [
        ("fx_haze_020.json", "amount"),
        ("fx_haze_021.json", "amount"),
        ("fx_haze_023.json", "Haze Color"),
        ("fx_haze_024.json", "fit"),
        ("fx_haze_025.json", "matte layer"),
    ] {
        let why = holder_effect(&t, file).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    t.shape_refused("fx_haze_001.json", "an Aerial Haze whose amount is a word", r##"{"haze_color": "#b4c8dc", "amount": "some", "layer": "", "fit": "stretch"}"##);
    t.shape_refused("fx_haze_001.json", "an Aerial Haze without its layer", r##"{"haze_color": "#b4c8dc", "amount": 30, "fit": "stretch"}"##);
    t.shape_refused("fx_haze_001.json", "an Aerial Haze whose colour is a number", r##"{"haze_color": 7, "amount": 30, "layer": "", "fit": "stretch"}"##);

    t.heading("Commands");
    let mut document = t.load("fx_haze_001.json").document;
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = holder_effect(&t, "fx_haze_001.json");
        f(&mut e);
        e
    };
    let over = with(&|e| if let Effect::AerialHaze { amount, .. } = e { *amount = 101.0 });
    let bad = with(&|e| if let Effect::AerialHaze { haze_color, .. } = e { *haze_color = "#abc".into() });
    let fill = with(&|e| if let Effect::AerialHaze { fit, .. } = e { *fit = "fill".into() });
    let ramp = with(&|e| {
        if let Effect::AerialHaze { layer, amount, .. } = e {
            *layer = J::from("ramp");
            *amount = 60.0;
        }
    });
    refused(
        &mut t,
        &mut document,
        vec![
            ("amount 101", set_holder(over)),
            ("Haze Color #abc", set_holder(bad)),
            ("fit fill", set_holder(fill)),
            ("amount keyed to 150", keys_holder("amount", &[(0, 0.0), (4, 150.0)])),
        ],
    );
    taken(
        &mut t,
        &mut document,
        "fx_haze_001.json",
        vec![("the ramp as the matte at 60", set_holder(ramp)), ("amount keyed from 0 to 100", keys_holder("amount", &[(0, 0.0), (4, 100.0)]))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_haze_001.json", 0),
        ("fx_haze_007.json", 0),
        ("fx_haze_011.json", 0),
        ("fx_haze_012.json", 0),
        ("fx_haze_013.json", 0),
        ("fx_haze_015.json", 2),
        ("fx_haze_018.json", 0),
        ("fx_haze_019.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Amount 0 (002), a layer not in the composition (016) and refused (020 to 025) leave
    // nothing for the card.
    let none: Vec<u32> = [2, 16].into_iter().chain(20..=25).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added (even)", json!({}), 3),
            ("through layer 4 as the matte at 80", json!({"amount": 80, "layer": "layer-4"}), 3),
            ("white haze through layer 4 centred at 50", json!({"haze_color": "#ffffff", "amount": 50, "layer": "layer-4", "fit": "center"}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-403 pictures/`");
    let dir = repo("verification/D-403 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    // The matte: white at the top (far away) to black at the bottom (near).
    let matte: Vec<u8> = (0..TOWN.1)
        .flat_map(|y| {
            let v = (255.0 * (1.0 - y as f64 / (TOWN.1 - 1) as f64)).round() as u8;
            (0..TOWN.0).flat_map(move |_| [v, v, v, 255])
        })
        .collect();
    write("matte.png", TOWN, &matte);
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]));
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (same, said) = street(json!({"amount": 0}));
    t.row(
        "amount 0 changes nothing: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );

    // Contrast: the spread between the darkest and lightest red channel.
    let spread = |p: &[u8]| {
        let reds = p.chunks_exact(4).filter(|q| q[3] > 0).map(|q| q[0]);
        reds.clone().max().unwrap() as i32 - reds.min().unwrap() as i32
    };
    let (even, said) = street(json!({}));
    write("2_as_added.png", TOWN, &even);
    let darkest = |p: &[u8]| p.chunks_exact(4).filter(|q| q[3] > 0).map(|q| q[0] as u32 + q[1] as u32 + q[2] as u32).min().unwrap();
    t.row(
        "2_as_added.png, as added (pale sky blue, 30, even): the whole street paler and bluer, its blacks lifted and its contrast lower; draws cleanly",
        &format!("{said:?}, darkest pixel {} against {}, red spread {} against {}", darkest(&even), darkest(&before), spread(&even), spread(&before)),
        said.is_empty() && darkest(&even) > darkest(&before) && spread(&even) < spread(&before),
    );

    let (thick, said) = street(json!({"amount": 70}));
    write("3_amount_70.png", TOWN, &thick);
    t.row(
        "3_amount_70.png, amount 70: a thick fog, nearer the sky blue than 2 is; draws cleanly",
        &format!("{said:?}, red spread {} against 2's {}", spread(&thick), spread(&even)),
        said.is_empty() && spread(&thick) < spread(&even),
    );

    let row_change = |a: &[u8], y: usize| {
        let w = TOWN.0 * 4;
        a[y * w..(y + 1) * w].iter().zip(&before[y * w..(y + 1) * w]).map(|(p, q)| p.abs_diff(*q) as u32).sum::<u32>()
    };
    let (faded, said) = picture_matte(&dir, json!([fx("fx-0-0", &json!({"amount": 80, "layer": "matte"}))]));
    write("4_matte_top_far.png", TOWN, &faded);
    let (top, bottom) = (row_change(&faded, 0), row_change(&faded, TOWN.1 - 1));
    t.row(
        "4_matte_top_far.png, amount 80 through a matte white at the top and black at the bottom: the top of the street deep in haze, the bottom row exactly as it was; draws cleanly",
        &format!("{said:?}, change in the top row {top}, in the bottom row {bottom}"),
        said.is_empty() && top > 0 && bottom == 0,
    );

    let (gone, said) = street(json!({"amount": 80, "layer": "gone"}));
    t.row(
        "a matte layer not in the composition: the street exactly as it was, with the warning",
        &format!("{said:?}, {} pixels changed", distance(&gone, &before).1),
        gone == before && said.iter().any(|s| s.starts_with("EFFECT_LAYER_MISSING")),
    );

    t.finish("D-403_aerial_haze_table.md");
}

/// The `holder` layer's effect in `file`.
fn holder_effect(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    let layer = d.project().composition(&Id::new(MAIN)).unwrap().layers_in_order().find(|l| l.effects.iter().any(|e| matches!(e.effect, Effect::AerialHaze { .. })));
    layer.expect("a layer with an Aerial Haze").effects[0].effect.clone()
}


/// The whole composition, to see that a refused command changed nothing (the effect is on
/// `holder`, not on the table's `art` layer).
fn everything(document: &anime_compositor::command::Document) -> String {
    format!("{:?}", document.project().composition(&Id::new(MAIN)).unwrap())
}

fn refused(t: &mut Table, document: &mut anime_compositor::command::Document, commands: Vec<(&str, Command)>) {
    let held = everything(document);
    for (what, command) in commands {
        let refused = document.apply(command).err();
        let untouched = everything(document) == held;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && untouched,
        );
    }
}

fn taken(t: &mut Table, document: &mut anime_compositor::command::Document, file: &str, commands: Vec<(&str, Command)>) {
    let n = commands.len();
    for (what, command) in commands {
        let taken = document.apply(command).is_ok();
        t.row(&format!("{what} is taken"), if taken { "taken" } else { "refused" }, taken);
    }
    let before = t.render(&t.load(file).document, 0, 64);
    for _ in 0..n {
        document.undo();
    }
    let same = t.render(document, 0, 64).data() == before.data();
    t.row(&format!("undo {n} times: frame 0 is the frame it was"), if same { "byte-identical" } else { "differ" }, same);
}
fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B282_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-282: a measurement, run deliberately with --release --ignored"]
fn b282_aerial_haze_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B282_CPU").is_ok();
    let passes = if cpu { 3 } else { 8 };
    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n- Drawn by: {}\n- Loops: {passes}\n\n\
         | Shot | Quality | First | Again |\n|---|---|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
        if cpu { "the processor" } else { "the card" },
    );
    let shots: [(&str, Option<J>); 3] = [
        ("Noise alone", None),
        ("Noise, then Aerial Haze as added (pale sky blue, 30, even)", Some(json!({}))),
        ("Noise, then Aerial Haze at 80 through layer 4 as the matte", Some(json!({"amount": 80, "layer": "layer-4"}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}c"), p));
            }
            J::Array(v)
        });
        let (comp, root) = (Id::new("comp-reference-shot"), repo("Fixtures/reference_shot"));
        let mut cache = CelCache::viewer();
        gpu.forget();
        let (mut first, mut times) = (Vec::new(), Vec::new());
        for pass in 0..passes {
            for frame in (0..240).step_by(8) {
                let mut log = FrameLog::new(3);
                let t = std::time::Instant::now();
                if cpu {
                    drop(preview::preview_frame_cached(&project, &comp, frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache).expect("CPU frame"));
                } else {
                    drop(preview::preview_frame_srgb8(&project, &comp, frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).expect("GPU frame"));
                }
                let ms = t.elapsed().as_secs_f64() * 1000.0;
                if pass > 0 { times.push(ms) } else { first.push(ms) }
            }
        }
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B282_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-282_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
