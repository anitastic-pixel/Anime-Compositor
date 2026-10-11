//! B-381: D-501, Difference Matte under `core.difference_matte`, after After Effects' Difference
//! Matte (Keying): each pixel compared with a clean plate, another layer named by D-189's layer
//! setting, and keyed out where they match, the rule and the settings when added ours.
//!
//! Writes `verification/D-501_difference_matte_table.md` and draws pictures into
//! `verification/D-501 pictures/`.
//!
//! Every expected pixel is `Fixtures/difference_matte/expected_difference_matte.json`, written by
//! `tools/difference_matte_reference.py` before this code existed and printed in document 25 as
//! FX-DMATTE-001 to 026. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

fn is_ours(c: &render::OnCard) -> bool {
    matches!(c, render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::DifferenceMatte { .. }))
}

/// The Difference Mattes the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers.iter().flat_map(|l| &l.on_card).filter(|c| is_ours(c.unmixed())).count()
        // An adjustment layer's run is made from its stack when the card draws it.
        + plan.layers
            .iter()
            .filter_map(|l| l.adjust.as_ref())
            .filter_map(|stack| compose::adjust_run(stack, (plan.width, plan.height)))
            .flatten()
            .filter(|c| is_ours(c.unmixed()))
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

/// A Difference Matte with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"view": "final", "layer": "", "fit": "center", "matching_tolerance": 15,
        "matching_softness": 0, "blur_before_difference": 0});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.difference_matte", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning or stay on the processor.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=26 {
        let file = format!("fx_dmatte_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b381-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Difference Matte {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Difference Matte {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the layer showing `1_shot.png` (from `dir`), with a hidden layer `plate` showing
/// `plate_png` beneath it, drawn on the processor, straight 8-bit, with what it warned of.
fn picture(dir: &Path, plate_png: &str, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("1_shot.png");
    let mut asset = project["assets"][0].clone();
    asset["id"] = J::from("asset-plate");
    asset["name"] = J::from("plate");
    asset["path"] = J::from(plate_png);
    project["assets"].as_array_mut().unwrap().push(asset);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    let layer = &mut comp["layers"][0];
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let mut plate = layer.clone();
    plate["id"] = J::from("plate");
    plate["name"] = J::from("plate");
    plate["asset_id"] = J::from("asset-plate");
    plate["enabled"] = J::from(false);
    plate["effects"] = json!([]);
    comp["layers"].as_array_mut().unwrap().insert(0, plate);
    comp["layer_order"].as_array_mut().unwrap().push(J::from("plate"));
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

#[test]
fn b381_difference_matte() {
    let mut t = Table::new(
        "difference_matte",
        "# D-501: Difference Matte\n\nB-381: `core.difference_matte`, after After Effects' Difference \
         Matte (Keying): each pixel compared with a clean plate, another layer of the composition \
         named by D-189's layer setting (on the card through `map_texture`), and keyed out where \
         they match; the rule and the settings when added are ours. Every expected pixel is \
         `Fixtures/difference_matte/expected_difference_matte.json`, written by \
         `tools/difference_matte_reference.py` before this code existed and printed in document 25 \
         as FX-DMATTE-001 to 026. Tolerance 2e-5.\n",
    );

    t.heading("FX-DMATTE-001 to 026 (document 25)");
    t.fixtures_numbered("expected_difference_matte.json", 1..=26);

    t.heading("Files whose layers read each other (document 25)");
    let expected: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/difference_matte/expected_difference_matte.json")).unwrap()).unwrap();
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
    let files: Vec<String> = (1..=26).map(|n| format!("fx_dmatte_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_dmatte_010.json");
    t.row(
        "fx_dmatte_010.json is saved with its six settings, the layer as written",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 6 && saved["layer"] == "patch" && saved["fit"] == "center" && saved["matching_tolerance"] == 15.0,
    );
    for (file, want) in [
        ("fx_dmatte_018.json", "matching tolerance"),
        ("fx_dmatte_019.json", "matching tolerance"),
        ("fx_dmatte_020.json", "matching softness"),
        ("fx_dmatte_021.json", "blur before difference"),
        ("fx_dmatte_022.json", "blur before difference"),
        ("fx_dmatte_023.json", "view"),
        ("fx_dmatte_024.json", "fit"),
        ("fx_dmatte_025.json", "difference layer"),
    ] {
        let why = holder_effect(&t, file).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    let whole = r#""view": "final", "fit": "center", "matching_softness": 0, "blur_before_difference": 0"#;
    t.shape_refused("fx_dmatte_001.json", "a Difference Matte whose tolerance is a word", &format!(r#"{{{whole}, "layer": "", "matching_tolerance": "some"}}"#));
    t.shape_refused("fx_dmatte_001.json", "a Difference Matte without its layer", &format!(r#"{{{whole}, "matching_tolerance": 15}}"#));
    t.shape_refused(
        "fx_dmatte_001.json",
        "a Difference Matte whose view is a number",
        r#"{"view": 7, "layer": "", "fit": "center", "matching_tolerance": 15, "matching_softness": 0, "blur_before_difference": 0}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_dmatte_001.json").document;
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = holder_effect(&t, "fx_dmatte_001.json");
        f(&mut e);
        e
    };
    let over = with(&|e| if let Effect::DifferenceMatte { matching_tolerance, .. } = e { *matching_tolerance = 101.0 });
    let alpha = with(&|e| if let Effect::DifferenceMatte { view, .. } = e { *view = "alpha".into() });
    let tile = with(&|e| if let Effect::DifferenceMatte { fit, .. } = e { *fit = "tile".into() });
    let plate = with(&|e| {
        if let Effect::DifferenceMatte { layer, matching_tolerance, .. } = e {
            *layer = J::from("plate");
            *matching_tolerance = 5.0;
        }
    });
    let blurred = |b: f64| {
        with(&|e| {
            if let Effect::DifferenceMatte { matching_tolerance, matching_softness, blur_before_difference, .. } = e {
                (*matching_tolerance, *matching_softness, *blur_before_difference) = (5.0, 10.0, b);
            }
        })
    };
    let (mut draft, half) = (blurred(4.0), blurred(2.0));
    refused(
        &mut t,
        &mut document,
        vec![
            ("tolerance 101", set_holder(over)),
            ("view alpha", set_holder(alpha)),
            ("fit tile", set_holder(tile)),
            ("tolerance keyed to 150", keys_holder("matching_tolerance", &[(0, 0.0), (4, 150.0)])),
        ],
    );
    taken(
        &mut t,
        &mut document,
        "fx_dmatte_001.json",
        vec![
            ("the plate as the difference layer at tolerance 5", set_holder(plate)),
            ("tolerance keyed from 0 to 10", keys_holder("matching_tolerance", &[(0, 0.0), (4, 10.0)])),
        ],
    );

    // B-381b: Draft halves the blur, as every other distance; the tolerance and softness stay.
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves Blur Before Difference and keeps the tolerance and softness (B-381b)",
        &format!("{draft:?}"),
        draft == half,
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_dmatte_002.json", 0),
        ("fx_dmatte_003.json", 0),
        ("fx_dmatte_006.json", 0),
        ("fx_dmatte_008.json", 0),
        ("fx_dmatte_009.json", 0),
        ("fx_dmatte_010.json", 0),
        ("fx_dmatte_011.json", 0),
        ("fx_dmatte_014.json", 4),
        ("fx_dmatte_015.json", 2),
        ("fx_dmatte_016.json", 0),
        ("fx_dmatte_017.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // As added (001), a layer not in the composition (013) and refused (018 to 026) leave
    // nothing for the card.
    let none: Vec<u32> = [1, 13].into_iter().chain(18..=26).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("against layer 4", json!({"layer": "layer-4"}), 3),
            ("against layer 4 stretched, as the matte, tolerance 8, softness 12", json!({"layer": "layer-4", "fit": "stretch", "view": "matte", "matching_tolerance": 8, "matching_softness": 12}), 3),
            ("against layer 4, tolerance 5, softness 10, blurred at 2", json!({"layer": "layer-4", "matching_tolerance": 5, "matching_softness": 10, "blur_before_difference": 2}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-501 pictures/`");
    let dir = repo("verification/D-501 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    // The plate is the street; the shot is the street with a magenta disc in front of it.
    let street = town();
    let (cx, cy, r) = (TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0, TOWN.1 as f64 / 4.0);
    let inside = |i: usize| {
        let (x, y) = ((i % TOWN.0) as f64 + 0.5, (i / TOWN.0) as f64 + 0.5);
        (x - cx).hypot(y - cy) < r
    };
    let shot: Vec<u8> = street
        .chunks_exact(4)
        .enumerate()
        .flat_map(|(i, p)| if inside(i) { [230, 40, 160, 255] } else { [p[0], p[1], p[2], p[3]] })
        .collect();
    write("1_shot.png", &shot);
    write("2_plate.png", &street);
    let disc = (0..TOWN.0 * TOWN.1).filter(|i| inside(*i)).count();

    let (keyed, said) = picture(&dir, "2_plate.png", json!([fx("fx-0-0", &json!({"layer": "plate"}))]));
    write("3_keyed.png", &keyed);
    let kept = keyed.chunks_exact(4).enumerate().filter(|(i, p)| inside(*i) && p[3] == 255).count();
    let leaked = keyed.chunks_exact(4).enumerate().filter(|(i, p)| !inside(*i) && p[3] > 0).count();
    t.row(
        "3_keyed.png, against the plate as added (tolerance 15): only the disc is left, every pixel of the street keyed out; draws cleanly",
        &format!("{said:?}, {kept} of the disc's {disc} pixels whole, {leaked} pixels of the street showing"),
        said.is_empty() && kept == disc && leaked == 0,
    );

    let (matte, said) = picture(&dir, "2_plate.png", json!([fx("fx-0-0", &json!({"layer": "plate", "view": "matte"}))]));
    write("4_matte.png", &matte);
    let white = matte.chunks_exact(4).enumerate().filter(|(i, p)| inside(*i) && p[..3] == [255, 255, 255]).count();
    let black = matte.chunks_exact(4).enumerate().filter(|(i, p)| !inside(*i) && p[..3] == [0, 0, 0] && p[3] == 255).count();
    t.row(
        "4_matte.png, as the matte only: the disc white, the street black; draws cleanly",
        &format!("{said:?}, {white} of {disc} disc pixels white, {black} of {} street pixels black", TOWN.0 * TOWN.1 - disc),
        said.is_empty() && white == disc && black == TOWN.0 * TOWN.1 - disc,
    );

    // A plate 6 levels lighter, as if the light changed between the two takes.
    let lighter: Vec<u8> = street.chunks_exact(4).flat_map(|p| [p[0].saturating_add(6), p[1].saturating_add(6), p[2].saturating_add(6), p[3]]).collect();
    write("5_lighter_plate.png", &lighter);
    let (tight, said_tight) = picture(&dir, "5_lighter_plate.png", json!([fx("fx-0-0", &json!({"layer": "plate", "matching_tolerance": 1}))]));
    let (loose, said_loose) = picture(&dir, "5_lighter_plate.png", json!([fx("fx-0-0", &json!({"layer": "plate", "matching_tolerance": 5}))]));
    write("6_lighter_tolerance_1.png", &tight);
    write("7_lighter_tolerance_5.png", &loose);
    let showing = |p: &[u8]| p.chunks_exact(4).enumerate().filter(|(i, q)| !inside(*i) && q[3] > 0).count();
    let whole = |p: &[u8]| p.chunks_exact(4).enumerate().filter(|(i, q)| inside(*i) && q[3] == 255).count();
    t.row(
        "6 and 7: against a plate 6 levels lighter (about 2.4 per cent), tolerance 1 keys almost none of the street and tolerance 5 keys all of it, the disc whole in both; draws cleanly",
        &format!("{said_tight:?} {said_loose:?}, street pixels showing {} at 1 and {} at 5, disc pixels whole {} and {} of {disc}", showing(&tight), showing(&loose), whole(&tight), whole(&loose)),
        said_tight.is_empty() && said_loose.is_empty() && showing(&tight) > (TOWN.0 * TOWN.1 - disc) * 9 / 10 && showing(&loose) == 0 && whole(&tight) == disc && whole(&loose) == disc,
    );

    let (gone, said) = picture(&dir, "2_plate.png", json!([fx("fx-0-0", &json!({"layer": "gone"}))]));
    t.row(
        "a difference layer not in the composition: the shot exactly as it was, with the warning",
        &format!("{said:?}, {} pixels changed", distance(&gone, &shot).1),
        gone == shot && said.iter().any(|s| s.starts_with("EFFECT_LAYER_MISSING")),
    );

    t.finish("D-501_difference_matte_table.md");
}

/// The `holder` layer's effect in `file`.
fn holder_effect(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    let layer = d.project().composition(&Id::new(MAIN)).unwrap().layers_in_order().find(|l| l.effects.iter().any(|e| matches!(e.effect, Effect::DifferenceMatte { .. })));
    layer.expect("a layer with a Difference Matte").effects[0].effect.clone()
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
/// With `B381_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-381: a measurement, run deliberately with --release --ignored"]
fn b381_difference_matte_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B381_CPU").is_ok();
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
        ("Noise, then Difference Matte against layer 4 (tolerance 15)", Some(json!({"layer": "layer-4"}))),
        (
            "Noise, then Difference Matte against layer 4, softness 10, blurred at 2",
            Some(json!({"layer": "layer-4", "matching_tolerance": 5, "matching_softness": 10, "blur_before_difference": 2})),
        ),
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
    let out = std::env::var("B381_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-381_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
