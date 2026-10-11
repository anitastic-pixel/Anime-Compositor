//! B-399: D-519, CC Spotlight's four Gel renders and its Gel Layer (D-507's `core.cc_spotlight`,
//! after CycoreFX's CC Spotlight (Perspective)): the light projects a layer through the cone like
//! a slide. CycoreFX describes the renders but publishes no formula: the rule is ours, stated in
//! `tools/cc_spotlight_gel_reference.py`.
//!
//! Writes `verification/D-519_cc_spotlight_gel_table.md` and draws pictures into
//! `verification/D-519 pictures/`.
//!
//! Every expected pixel is `Fixtures/cc_spotlight_gel/expected_cc_spotlight_gel.json`, written by
//! the tool before this code existed and printed in document 25 as FX-SPOTGEL-001 to 021.
//! Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{repo, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::Command;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
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
    matches!(c, render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::CcSpotlight { .. }))
}

/// The CC Spotlights the card's plan leaves to the card, over every layer.
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

/// A CC Spotlight with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"from": [25, 25], "to": [50, 50], "height": 50, "cone_angle": 40, "edge_softness": 20,
        "color": "#ffffff", "intensity": 100, "render": "light_shadow", "gel_layer": ""});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.cc_spotlight", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` leave nothing for the card.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=21 {
        let file = format!("fx_spotgel_{n:03}.json");
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
/// and Draft, on the card against the processor; 3 on the card each frame.
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J)]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p) in settings {
        let project = reference(|id| json!([fx(&format!("b399-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, CC Spotlight {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, CC Spotlight {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

/// `effects` on the layer showing `1_street.png` (from `dir`), with a hidden layer `gel`
/// showing `2_gel.png` beneath it, drawn on the processor, straight 8-bit, with what it
/// warned of.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("1_street.png");
    let mut asset = project["assets"][0].clone();
    asset["id"] = J::from("asset-gel");
    asset["name"] = J::from("gel");
    asset["path"] = J::from("2_gel.png");
    project["assets"].as_array_mut().unwrap().push(asset);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    let layer = &mut comp["layers"][0];
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let mut gel = layer.clone();
    gel["id"] = J::from("gel");
    gel["name"] = J::from("gel");
    gel["asset_id"] = J::from("asset-gel");
    gel["enabled"] = J::from(false);
    gel["effects"] = json!([]);
    comp["layers"].as_array_mut().unwrap().insert(0, gel);
    comp["layer_order"].as_array_mut().unwrap().push(J::from("gel"));
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
fn b399_cc_spotlight_gel() {
    let mut t = Table::new(
        "cc_spotlight_gel",
        "# D-519: CC Spotlight's Gel renders\n\nB-399: the four Gel renders (Gel Only, Gel Add, \
         Gel Add+, Gel Shadow) and the Gel Layer of `core.cc_spotlight` (D-507), after CycoreFX's \
         CC Spotlight (Perspective): the light projects a layer through its cone like a slide. \
         CycoreFX describes the renders but publishes no formula, so the rule is ours, stated in \
         `tools/cc_spotlight_gel_reference.py`: the gel layer (D-189's layer setting, centred on \
         the drawing) is carried across the cone, its longer side spanning the opening; the light \
         let through is the gel over white, (1 - gel alpha) + gel colour; each Gel render is its \
         Light render with the light's colour multiplied by that. With no gel layer a Gel render \
         is its Light render, and a Light render never reads the gel layer. Every expected pixel \
         is `Fixtures/cc_spotlight_gel/expected_cc_spotlight_gel.json`, written by the tool \
         before this code existed and printed in document 25 as FX-SPOTGEL-001 to 021. \
         Tolerance 2e-5.\n",
    );

    t.heading("FX-SPOTGEL-001 to 021 (document 25)");
    t.fixtures_numbered("expected_cc_spotlight_gel.json", 1..=21);

    t.heading("Files whose layers read each other (document 25)");
    let expected: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/cc_spotlight_gel/expected_cc_spotlight_gel.json")).unwrap()).unwrap();
    for (file, case) in expected["loads"].as_object().unwrap() {
        let says = case["says"].as_str().unwrap();
        let want = case["refused"].as_str();
        let got = persist::load(&t.root.join(file)).err();
        t.row(
            &format!("{file}: {says}"),
            &got.as_ref().map_or("opened".to_string(), |d| format!("{} {}", d.id.as_str(), d.message)),
            got.map(|d| d.id.as_str().to_string()).as_deref() == want,
        );
    }

    t.heading("The file");
    let files: Vec<String> = (1..=21).map(|n| format!("fx_spotgel_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_spotgel_011.json");
    t.row(
        "fx_spotgel_011.json is saved with its nine settings, render gel add, the slide as the gel",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 9 && saved["render"] == "gel_add" && saved["gel_layer"] == "slide" && saved["color"] == "#ff8000",
    );
    let old = persist::load(&repo("Fixtures/cc_spotlight/fx_spot_009.json")).expect("D-507's file opens");
    let old_effect = old.document.project().composition(&Id::new(MAIN)).unwrap().layers_in_order().flat_map(|l| &l.effects).next().unwrap().effect.clone();
    let old_saved = effect_table::saved(&old);
    let old_params = old_saved["compositions"][0]["layers"].as_array().unwrap().iter().find_map(|l| l["effects"].as_array().and_then(|e| e.first())).unwrap()["parameters"].clone();
    t.row(
        "D-507's fx_spot_009.json, written before the Gel Layer: opens with no gel layer and is saved with its eight settings, as it was",
        &format!("{old_effect:?}; saved {old_params}"),
        matches!(&old_effect, Effect::CcSpotlight { gel_layer, .. } if *gel_layer == "") && old_params.as_object().unwrap().len() == 8 && old.warnings.is_empty(),
    );
    for (file, want) in [("fx_spotgel_019.json", "gel layer"), ("fx_spotgel_020.json", "render"), ("fx_spotgel_021.json", "render")] {
        let why = holder_effect(&t, file).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }

    t.heading("Commands");
    let mut document = t.load("fx_spotgel_002.json").document;
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = holder_effect(&t, "fx_spotgel_002.json");
        f(&mut e);
        e
    };
    let glow = with(&|e| if let Effect::CcSpotlight { render, .. } = e { *render = "gel_glow".to_string() });
    let number = with(&|e| if let Effect::CcSpotlight { gel_layer, .. } = e { *gel_layer = J::from(3) });
    let small = with(&|e| if let Effect::CcSpotlight { render, gel_layer, .. } = e {
        *render = "gel_only".to_string();
        *gel_layer = J::from("small");
    });
    // Nothing is a distance: a draft changes nothing.
    let mut draft = with(&|_| {});
    draft.scale_distances(|d| d * 0.5);
    let same = draft == with(&|_| {});
    refused(
        &mut t,
        &mut document,
        vec![
            ("render \"gel_glow\"", set_holder(glow)),
            ("a gel layer written as the number 3", set_holder(number)),
        ],
    );
    taken(
        &mut t,
        &mut document,
        "fx_spotgel_002.json",
        vec![
            ("render gel only, the small paint as the gel", set_holder(small)),
            ("intensity keyed from 0 to 250", keys_holder("intensity", &[(0, 0.0), (4, 250.0)])),
        ],
    );
    t.row("a half-size draft preview changes no setting (none is a distance)", &format!("{draft:?}"), same);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_spotgel_001.json", 0),
        ("fx_spotgel_002.json", 0),
        ("fx_spotgel_003.json", 0),
        ("fx_spotgel_004.json", 0),
        ("fx_spotgel_007.json", 0),
        ("fx_spotgel_008.json", 4),
        ("fx_spotgel_010.json", 0),
        ("fx_spotgel_011.json", 0),
        ("fx_spotgel_015.json", 2),
        ("fx_spotgel_018.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // A gel layer not in the composition (009) and refused (019 to 021) leave nothing for the card.
    card_fixtures(&mut t, &mut gpu, &[9, 19, 20, 21]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("gel add, layer 4 as the gel, opening 90", json!({"render": "gel_add", "gel_layer": "layer-4", "cone_angle": 90})),
            ("gel only, layer 4 as the gel, low and wide", json!({"render": "gel_only", "gel_layer": "layer-4", "height": 10, "cone_angle": 90})),
            ("gel shadow, layer 4 as the gel, From [50, 50], orange", json!({"render": "gel_shadow", "gel_layer": "layer-4", "from": [50, 50], "color": "#ff8000"})),
            ("gel add plus, no gel layer", json!({"render": "gel_add_plus"})),
        ],
    );

    t.heading("Pictures: in `verification/D-519 pictures/`");
    let dir = repo("verification/D-519 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], bytes).unwrap();
    // The street, and as the gel, a slide of four coloured panes on clear.
    let street = town();
    let mut slide = vec![0u8; w * h * 4];
    for y in h / 4..3 * h / 4 {
        for x in w / 4..3 * w / 4 {
            let pane: [u8; 4] = match (x < w / 2, y < h / 2) {
                (true, true) => [255, 40, 40, 255],
                (false, true) => [40, 200, 40, 255],
                (true, false) => [40, 80, 255, 255],
                (false, false) => [255, 220, 40, 255],
            };
            slide[(y * w + x) * 4..(y * w + x) * 4 + 4].copy_from_slice(&pane);
        }
    }
    write("1_street.png", &street);
    write("2_gel.png", &slide);
    // Gel Only leaves the drawing clear outside the light: its pictures are laid on black, so
    // the pool of light shows in any viewer.
    let on_black = |b: &[u8]| -> Vec<u8> {
        b.chunks(4).flat_map(|p| {
            let a = p[3] as f64 / 255.0;
            [(p[0] as f64 * a).round() as u8, (p[1] as f64 * a).round() as u8, (p[2] as f64 * a).round() as u8, 255]
        }).collect()
    };
    let down = json!({"from": [50, 50], "to": [50, 50], "cone_angle": 90});
    let with_p = |extra: J| {
        let mut p = down.clone();
        for (k, v) in extra.as_object().unwrap() {
            p[k] = v.clone();
        }
        p
    };

    let (light_only, _) = picture(&dir, json!([fx("fx-0-0", &with_p(json!({"render": "light_only"})))]));
    let (gel_only, said) = picture(&dir, json!([fx("fx-0-0", &with_p(json!({"render": "gel_only", "gel_layer": "gel"})))]));
    write("3_gel_only.png", &on_black(&gel_only));
    t.row(
        "3_gel_only.png, gel only, the four panes as the gel, the light straight down on the middle: the street gone, the panes projected in a round pool of light on black (the clear outside laid on black in the picture), red top left, green top right, blue bottom left, yellow bottom right, white light round them; draws cleanly",
        &format!("{said:?}, {} pixels differ from light only", distance(&gel_only, &light_only).1),
        said.is_empty() && distance(&gel_only, &light_only).1 > 0,
    );

    let (gel_add, said) = picture(&dir, json!([fx("fx-0-0", &with_p(json!({"render": "gel_add", "gel_layer": "gel"})))]));
    write("4_gel_add.png", &gel_add);
    t.row(
        "4_gel_add.png, gel add: the street with the coloured panes' light added in the pool; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&gel_add, &street).1),
        said.is_empty() && distance(&gel_add, &street).1 > 0,
    );

    let (shadow, said) = picture(&dir, json!([fx("fx-0-0", &with_p(json!({"render": "gel_shadow", "gel_layer": "gel"})))]));
    write("5_gel_shadow.png", &shadow);
    t.row(
        "5_gel_shadow.png, gel shadow: black outside the pool, the street inside tinted by each pane; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&shadow, &street).1),
        said.is_empty() && distance(&shadow, &street).1 > 0,
    );

    let (low, said) = picture(&dir, json!([fx("fx-0-0", &json!({"render": "gel_only", "gel_layer": "gel", "from": [10, 10], "height": 20, "cone_angle": 60}))]));
    write("6_low_light.png", &on_black(&low));
    t.row(
        "6_low_light.png, gel only, the light low at the top left: the panes and the pool drawn out into a long slanted shape, on black; draws cleanly",
        &format!("{said:?}, {} pixels differ from 3", distance(&low, &gel_only).1),
        said.is_empty() && distance(&low, &gel_only).1 > 0,
    );

    let (light_add, _) = picture(&dir, json!([fx("fx-0-0", &with_p(json!({"render": "light_add", "gel_layer": "gel"})))]));
    let (clear, said) = picture(&dir, json!([fx("fx-0-0", &with_p(json!({"render": "gel_add"})))]));
    t.row(
        "gel add with no gel layer is byte for byte light add with the gel named (a Light render ignores it)",
        &format!("{said:?}, {} pixels differ", distance(&clear, &light_add).1),
        said.is_empty() && clear == light_add,
    );

    let (gone, said) = picture(&dir, json!([fx("fx-0-0", &with_p(json!({"render": "gel_add", "gel_layer": "gone"})))]));
    t.row(
        "a gel layer not in the composition: the street exactly as it was, with the warning",
        &format!("{said:?}, {} pixels changed", distance(&gone, &street).1),
        gone == street && said.iter().any(|s| s.starts_with("EFFECT_LAYER_MISSING")),
    );

    t.finish("D-519_cc_spotlight_gel_table.md");
}

/// The effect in `file` (on `holder`, or the adjustment layer in 016).
fn holder_effect(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    let layer = d.project().composition(&Id::new(MAIN)).unwrap().layers_in_order().find(|l| l.effects.iter().any(|e| matches!(e.effect, Effect::CcSpotlight { .. })));
    layer.expect("a layer with a CC Spotlight").effects[0].effect.clone()
}

/// The whole composition, to see that a refused command changed nothing.
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
/// With `B399_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-399: a measurement, run deliberately with --release --ignored"]
fn b399_cc_spotlight_gel_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B399_CPU").is_ok();
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
    let shots: [(&str, Option<J>); 4] = [
        ("Noise alone", None),
        ("Noise, then CC Spotlight, gel add, layer 4 as the gel, opening 90", Some(json!({"render": "gel_add", "gel_layer": "layer-4", "cone_angle": 90}))),
        ("Noise, then CC Spotlight, gel shadow, layer 4 as the gel", Some(json!({"render": "gel_shadow", "gel_layer": "layer-4"}))),
        ("Noise, then CC Spotlight, gel only, layer 4 as the gel, low and wide", Some(json!({"render": "gel_only", "gel_layer": "layer-4", "height": 10, "cone_angle": 90}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}g"), p));
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
    let out = std::env::var("B399_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-399_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
