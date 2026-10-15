//! B-398: D-518, Warp Blend under `core.warp_blend`, after CycoreFX's CC WarpoMatic (Transition),
//! under our own name: the layer and another layer are pulled out of shape along what differs
//! between them while one blends into the other. CycoreFX names the controls but publishes no
//! formula and no defaults: the rule and the settings when added are ours, stated in
//! `tools/warp_blend_reference.py`.
//!
//! Writes `verification/D-518_warp_blend_table.md` and draws pictures into
//! `verification/D-518 pictures/`.
//!
//! Every expected pixel is `Fixtures/warp_blend/expected_warp_blend.json`, written by the tool
//! before this code existed and printed in document 25 as FX-WARPB-001 to 039. Tolerance 2e-5.
//! Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{both, distance, median, reference, repo, time_passes, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::Command;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, EffectKey};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Interp, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

fn is_ours(c: &render::OnCard) -> bool {
    matches!(c, render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::WarpBlend { .. }))
}

/// The Warp Blends the card's plan leaves to the card, over every layer.
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

/// A Warp Blend with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"completion": 0, "layer_to_reveal": "", "reactor": "brightness", "smoothness": 10,
        "warp_amount": 50, "warp_direction": "joint", "blend_span": 50});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.warp_blend", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` leave nothing for the card.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=39 {
        let file = format!("fx_warpb_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b398-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Warp Blend {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Warp Blend {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

/// `effects` on the layer showing `1_street.png` (from `dir`), with a hidden layer `reveal`
/// showing `2_reveal.png` beneath it, drawn on the processor, straight 8-bit, with what it
/// warned of.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("1_street.png");
    let mut asset = project["assets"][0].clone();
    asset["id"] = J::from("asset-reveal");
    asset["name"] = J::from("reveal");
    asset["path"] = J::from("2_reveal.png");
    project["assets"].as_array_mut().unwrap().push(asset);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    let layer = &mut comp["layers"][0];
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let mut reveal = layer.clone();
    reveal["id"] = J::from("reveal");
    reveal["name"] = J::from("reveal");
    reveal["asset_id"] = J::from("asset-reveal");
    reveal["enabled"] = J::from(false);
    reveal["effects"] = json!([]);
    comp["layers"].as_array_mut().unwrap().insert(0, reveal);
    comp["layer_order"].as_array_mut().unwrap().push(J::from("reveal"));
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
fn b398_warp_blend() {
    let mut t = Table::new(
        "warp_blend",
        "# D-518: Warp Blend\n\nB-398: `core.warp_blend`, after CycoreFX's CC WarpoMatic \
         (Transition), under our own name: the layer and the Layer to Reveal (D-189's layer \
         setting, centred on the drawing; nothing when none is chosen) are pulled out of shape \
         along what differs between them while one blends into the other. CycoreFX names the \
         controls but publishes no formula and no defaults, so the rule and the settings when \
         added are ours, stated in `tools/warp_blend_reference.py`: a reactor worked from the two \
         layers (Brightness, Contrast Differences, Brightness Differences or Local Differences), \
         blurred by the Smoothness; its slope times the Warp Amount is the pull, turned a quarter \
         when Twisting; the layer is read pulled by the completion's share of it and the reveal \
         by the rest, the same way (Joint, Twisting) or the other (Opposing); and the reveal's \
         share rises across the Blend Span, centred on the middle of the transition. Every \
         expected pixel is `Fixtures/warp_blend/expected_warp_blend.json`, written by the tool \
         before this code existed and printed in document 25 as FX-WARPB-001 to 039. Tolerance \
         2e-5.\n",
    );

    t.heading("FX-WARPB-001 to 039 (document 25)");
    t.fixtures_numbered("expected_warp_blend.json", 1..=39);

    t.heading("Files whose layers read each other (document 25)");
    let expected: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/warp_blend/expected_warp_blend.json")).unwrap()).unwrap();
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
    let files: Vec<String> = (1..=39).map(|n| format!("fx_warpb_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_warpb_007.json");
    t.row(
        "fx_warpb_007.json is saved with its seven settings, the layer and the words as written",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 7
            && saved["layer_to_reveal"] == "paint"
            && saved["reactor"] == "brightness"
            && saved["warp_direction"] == "opposing"
            && saved["completion"] == 50.0
            && saved["smoothness"] == 2.0
            && saved["warp_amount"] == 8.0
            && saved["blend_span"] == 50.0,
    );
    for (file, want) in [
        ("fx_warpb_028.json", "completion"),
        ("fx_warpb_029.json", "completion"),
        ("fx_warpb_030.json", "smoothness"),
        ("fx_warpb_031.json", "smoothness"),
        ("fx_warpb_032.json", "warp amount"),
        ("fx_warpb_033.json", "warp amount"),
        ("fx_warpb_034.json", "blend span"),
        ("fx_warpb_035.json", "blend span"),
        ("fx_warpb_036.json", "reactor"),
        ("fx_warpb_037.json", "warp direction"),
        ("fx_warpb_038.json", "layer to reveal"),
    ] {
        let why = holder_effect(&t, file).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    let whole = r#""layer_to_reveal": "", "reactor": "brightness", "smoothness": 10, "warp_amount": 50, "warp_direction": "joint", "blend_span": 50"#;
    t.shape_refused("fx_warpb_001.json", "a Warp Blend whose completion is a word", &format!(r#"{{{whole}, "completion": "half"}}"#));
    t.shape_refused("fx_warpb_001.json", "a Warp Blend without its completion", &format!(r#"{{{whole}}}"#));
    t.shape_refused(
        "fx_warpb_001.json",
        "a Warp Blend without its layer to reveal",
        r#"{"completion": 0, "reactor": "brightness", "smoothness": 10, "warp_amount": 50, "warp_direction": "joint", "blend_span": 50}"#,
    );
    t.shape_refused(
        "fx_warpb_001.json",
        "a Warp Blend whose reactor is a number",
        r#"{"completion": 0, "layer_to_reveal": "", "reactor": 2, "smoothness": 10, "warp_amount": 50, "warp_direction": "joint", "blend_span": 50}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_warpb_005.json").document;
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = holder_effect(&t, "fx_warpb_005.json");
        f(&mut e);
        e
    };
    let over = with(&|e| if let Effect::WarpBlend { completion, .. } = e { *completion = 101.0 });
    let word = with(&|e| if let Effect::WarpBlend { reactor, .. } = e { *reactor = "hue".to_string() });
    let bare = with(&|e| if let Effect::WarpBlend { completion, layer_to_reveal, warp_direction, .. } = e {
        *completion = 70.0;
        *layer_to_reveal = J::from("");
        *warp_direction = "twisting".to_string();
    });
    // Draft halves the smoothness and the pull, as every other distance; the completion and the
    // span are not distances.
    let mut draft = with(&|e| if let Effect::WarpBlend { smoothness, warp_amount, .. } = e {
        (*smoothness, *warp_amount) = (8.0, -40.0);
    });
    draft.scale_distances(|d| d * 0.5);
    let halved = draft == with(&|e| if let Effect::WarpBlend { smoothness, warp_amount, .. } = e {
        (*smoothness, *warp_amount) = (4.0, -20.0);
    });
    refused(
        &mut t,
        &mut document,
        vec![
            ("completion 101", set_holder(over)),
            ("reactor \"hue\"", set_holder(word)),
            ("warp amount keyed to 600", keys_holder("warp_amount", &[(0, 0.0), (4, 600.0)])),
        ],
    );
    taken(
        &mut t,
        &mut document,
        "fx_warpb_005.json",
        vec![
            ("completion 70, twisting, revealing nothing", set_holder(bare)),
            ("completion keyed from 0 to 100", keys_holder("completion", &[(0, 0.0), (4, 100.0)])),
        ],
    );
    t.row(
        "a half-size draft preview halves the smoothness and the warp amount and keeps the rest",
        &format!("{draft:?}"),
        halved,
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_warpb_003.json", 0),
        ("fx_warpb_004.json", 0),
        ("fx_warpb_005.json", 0),
        ("fx_warpb_006.json", 0),
        ("fx_warpb_007.json", 0),
        ("fx_warpb_008.json", 0),
        ("fx_warpb_012.json", 0),
        ("fx_warpb_014.json", 0),
        ("fx_warpb_016.json", 0),
        ("fx_warpb_020.json", 0),
        ("fx_warpb_021.json", 0),
        ("fx_warpb_022.json", 0),
        ("fx_warpb_023.json", 0),
        ("fx_warpb_024.json", 2),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Completion 0 (001), a layer not in the composition (018) and refused (028 to 039) leave
    // nothing for the card.
    let none: Vec<u32> = [1, 18].into_iter().chain(28..=39).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("completion 40, revealing layer 4 (as added otherwise)", json!({"completion": 40, "layer_to_reveal": "layer-4"})),
            ("completion 60, local differences, twisting, smoothness 4, amount -30, revealing layer 4", json!({"completion": 60, "layer_to_reveal": "layer-4", "reactor": "local_differences", "warp_direction": "twisting", "smoothness": 4, "warp_amount": -30})),
            ("completion 50, contrast differences, opposing, smoothness 0, span 0, revealing nothing", json!({"completion": 50, "reactor": "contrast_differences", "warp_direction": "opposing", "smoothness": 0, "blend_span": 0})),
            ("completion 100, revealing layer 4", json!({"completion": 100, "layer_to_reveal": "layer-4"})),
        ],
    );

    t.heading("Pictures: in `verification/D-518 pictures/`");
    let dir = repo("verification/D-518 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], bytes).unwrap();
    // The street, and to reveal, the street in its opposite colours.
    let street = town();
    let opposite: Vec<u8> = street.chunks_exact(4).flat_map(|p| [255 - p[0], 255 - p[1], 255 - p[2], p[3]]).collect();
    write("1_street.png", &street);
    write("2_reveal.png", &opposite);
    let half: Vec<u8> = street.iter().zip(&opposite).map(|(a, b)| ((*a as u16 + *b as u16) / 2) as u8).collect();

    let (begun, said) = picture(&dir, json!([fx("fx-0-0", &json!({"completion": 25, "layer_to_reveal": "reveal"}))]));
    write("3_begun.png", &begun);
    t.row(
        "3_begun.png, completion 25: the street alone, pulled out of shape along its edges, before the blend begins; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&begun, &street).1),
        said.is_empty() && distance(&begun, &street).1 > 0,
    );

    let (joint, said) = picture(&dir, json!([fx("fx-0-0", &json!({"completion": 50, "layer_to_reveal": "reveal"}))]));
    write("4_halfway.png", &joint);
    t.row(
        "4_halfway.png, completion 50: half the street and half its opposite, both pulled the same way; draws cleanly",
        &format!("{said:?}, {} pixels differ from an even mix without the pull", distance(&joint, &half).1),
        said.is_empty() && distance(&joint, &half).1 > 0,
    );

    let (opposing, said) = picture(&dir, json!([fx("fx-0-0", &json!({"completion": 50, "layer_to_reveal": "reveal", "warp_direction": "opposing"}))]));
    write("5_opposing.png", &opposing);
    t.row(
        "5_opposing.png, completion 50, opposing: the two pulled opposite ways; draws cleanly",
        &format!("{said:?}, {} pixels differ from 4", distance(&opposing, &joint).1),
        said.is_empty() && distance(&opposing, &joint).1 > 0,
    );

    let (twisting, said) = picture(&dir, json!([fx("fx-0-0", &json!({"completion": 50, "layer_to_reveal": "reveal", "warp_direction": "twisting", "warp_amount": 150}))]));
    write("6_twisting.png", &twisting);
    t.row(
        "6_twisting.png, completion 50, twisting, amount 150: the two swirled round along the edges; draws cleanly",
        &format!("{said:?}, {} pixels differ from 4", distance(&twisting, &joint).1),
        said.is_empty() && distance(&twisting, &joint).1 > 0,
    );

    let (done, said) = picture(&dir, json!([fx("fx-0-0", &json!({"completion": 100, "layer_to_reveal": "reveal"}))]));
    write("7_done.png", &done);
    let d = distance(&done, &opposite);
    t.row(
        "7_done.png, completion 100: the street's opposite exactly; draws cleanly",
        &format!("{said:?}, largest difference {} of 255 from the reveal, {} pixels", d.0, d.1),
        said.is_empty() && d.0 <= 1,
    );

    let (gone, said) = picture(&dir, json!([fx("fx-0-0", &json!({"completion": 50, "layer_to_reveal": "gone"}))]));
    t.row(
        "a layer to reveal not in the composition: the street exactly as it was, with the warning",
        &format!("{said:?}, {} pixels changed", distance(&gone, &street).1),
        gone == street && said.iter().any(|s| s.starts_with("EFFECT_LAYER_MISSING")),
    );

    t.finish("D-518_warp_blend_table.md");
}

/// The effect in `file` (on `holder`, or the adjustment layer in 022).
fn holder_effect(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    let layer = d.project().composition(&Id::new(MAIN)).unwrap().layers_in_order().find(|l| l.effects.iter().any(|e| matches!(e.effect, Effect::WarpBlend { .. })));
    layer.expect("a layer with a Warp Blend").effects[0].effect.clone()
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

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B398_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-398: a measurement, run deliberately with --release --ignored"]
fn b398_warp_blend_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B398_CPU").is_ok();
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
        ("Noise, then Warp Blend, completion 50, revealing layer 4 (as added otherwise)", Some(json!({"completion": 50, "layer_to_reveal": "layer-4"}))),
        ("Noise, then Warp Blend, completion 50, local differences, twisting, smoothness 30, revealing layer 4", Some(json!({"completion": 50, "layer_to_reveal": "layer-4", "reactor": "local_differences", "warp_direction": "twisting", "smoothness": 30}))),
        ("Noise, then Warp Blend, completion 50, smoothness 0, revealing nothing", Some(json!({"completion": 50, "smoothness": 0}))),
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
        let (first, times) = time_passes(&mut gpu, cpu, passes, &project, &comp, &root, PreviewQuality::Full);
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B398_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-398_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
