//! B-392: D-512, Channel Wipe under `core.channel_wipe`, after CycoreFX's CC Image Wipe
//! (Transition), under our own name: the layer wiped away in the order of a chosen channel of a
//! gradient layer, the lowest values first. A second effect over Gradient Wipe's engine (D-383,
//! D-394), which takes the extra settings; Gradient Wipe draws as before, and its own fixtures
//! and card comparisons are run again here unchanged. CycoreFX names the controls but publishes
//! no formula and no settings when added: the rule and those settings are ours, stated in
//! `tools/channel_wipe_reference.py`.
//!
//! Writes `verification/D-512_channel_wipe_table.md` and draws pictures into
//! `verification/D-512 pictures/`.
//!
//! Every expected pixel is `Fixtures/channel_wipe/expected_channel_wipe.json`, written by the
//! tool before this code existed and printed in document 25 as FX-CWIPE-001 to 033, or Gradient
//! Wipe's own `Fixtures/gradient_wipe/expected_gradient_wipe.json` (FX-GWIPE-001 to 028).
//! Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{distance, median, reference, repo, town, Table, MAIN, TOWN};
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

/// What a frame warned of, the card's own `GPU_PREVIEW_ON_CPU` left out, and whether it said that.
fn said(log: FrameLog) -> (String, bool) {
    let all = log.finish();
    let refused = all.iter().any(|d| d.id.as_str() == DiagnosticId::GpuPreviewOnCpu.as_str());
    let mut ids: Vec<&str> = all.iter().map(|d| d.id.as_str()).filter(|id| *id != DiagnosticId::GpuPreviewOnCpu.as_str()).collect();
    ids.sort();
    ids.dedup();
    (ids.join(", "), refused)
}

/// A Channel Wipe or, for Gradient Wipe's own rows, a Gradient Wipe.
fn is_ours(c: &render::OnCard) -> bool {
    matches!(c, render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::ChannelWipe { .. } | Effect::GradientWipe { .. }))
}

/// The wipes the card's plan leaves to the card, over every layer.
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
    let (said_cpu, _) = said(log);
    let mut log = FrameLog::new(3);
    let (g, ..) = preview::preview_frame_srgb8(project, comp, frame, root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, gpu)
        .unwrap_or_else(|d| panic!("frame {frame} on the GPU: {}", d.message));
    let (said_gpu, refused) = said(log);
    (distance(&c.to_srgb8_straight(), &g), refused, said_cpu, said_gpu)
}

/// A Channel Wipe with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"layer": "", "fit": "stretch", "completion": 0, "softness": 10, "property": "luminance", "blur": 0, "invert": "off"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.channel_wipe", "enabled": true, "parameters": all})
}

/// A Gradient Wipe with all five of its settings `p`.
fn gwipe(id: &str, p: &J) -> J {
    json!({"instance_id": id, "type_id": "core.gradient_wipe", "enabled": true, "parameters": p})
}

/// Every file `{prefix}_{n:03}.json` of `t.root` for n in 1 to `count` on the card against the
/// processor, frames 0 to 4 at Full and Draft. The files numbered in `none` leave nothing for the
/// card; with `none` None the count on the card is shown but not required, as B-226 has it for
/// Gradient Wipe, and a frame the card refuses must be the processor's exactly.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, prefix: &str, count: u32, none: Option<&[u32]>) {
    let comp = Id::new(MAIN);
    for n in 1..=count {
        let file = format!("{prefix}_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card, mut exact) = (0, false, true, 0, true);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                exact &= !r || d.0 == 0;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality);
            }
        }
        let ok = match none {
            Some(none) => largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
            None => largest <= 1 && exact && agree,
        };
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            ok,
        );
    }
}

/// The reference shot with each effect `make` makes on its first three layers, frames 0, 100
/// and 239 at Full and Draft, on the card against the processor; 3 on the card each frame.
fn card_reference(t: &mut Table, gpu: &mut Gpu, name: &str, make: fn(&str, &J) -> J, settings: &[(&str, J)]) {
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
        let project = reference(|id| json!([make(&format!("b392-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, {name} {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, {name} {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

/// The files of `t.root`'s `expected` whose layers read each other, each refused as written.
fn loads(t: &mut Table, expected: &str) {
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join(expected)).unwrap()).unwrap();
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
fn b392_channel_wipe() {
    let mut t = Table::new(
        "channel_wipe",
        "# D-512: Channel Wipe\n\nB-392: `core.channel_wipe`, after CycoreFX's CC Image Wipe \
         (Transition), under our own name: the layer wiped away in the order of a chosen channel \
         of a gradient layer, the lowest values first. By D-383 and D-394, as D-402 Tritone, it \
         is a second effect over Gradient Wipe's engine (D-194), which takes the extra settings \
         (Property, Blur, the layer itself as the gradient); Gradient Wipe is that engine with \
         luminance, no blur and nothing wiped when no layer is named, and draws as before: its \
         own fixtures and card comparisons are run again below, unchanged. CycoreFX names the \
         controls but publishes no formula and no settings when added, so the rule and those \
         settings are ours, stated in `tools/channel_wipe_reference.py`: the gradient (the layer \
         itself when none is chosen) is read in the chosen property (red, green, blue, alpha, \
         luminance, lightness, hue or saturation), blurred at sigma Blur / 2 with the outer \
         pixels repeated, and Gradient Wipe's completion and softness rule keeps or clears each \
         pixel. Every expected pixel is `Fixtures/channel_wipe/expected_channel_wipe.json`, \
         written by the tool before this code existed and printed in document 25 as \
         FX-CWIPE-001 to 033. Tolerance 2e-5. CWIPE-002 to 006 are Gradient Wipe's own rendered \
         numbers (FX-GWIPE-016, 003, 004, 008, 012), which shows the one engine.\n",
    );

    t.heading("FX-CWIPE-001 to 033 (document 25)");
    t.fixtures_numbered("expected_channel_wipe.json", 1..=33);

    t.heading("Files whose layers read each other (document 25)");
    loads(&mut t, "expected_channel_wipe.json");

    t.heading("The file");
    let files: Vec<String> = (1..=33).map(|n| format!("fx_cwipe_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_cwipe_016.json");
    t.row(
        "fx_cwipe_016.json is saved with its seven settings, the layer as written, blur 3",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 7
            && saved["layer"] == "card"
            && saved["property"] == "luminance"
            && saved["blur"] == json!(3)
            && saved["softness"] == json!(20),
    );
    t.row(
        "the map the effect reads is never saved",
        &format!("{:?}", saved.get("map")),
        saved.get("map").is_none(),
    );
    for (file, want) in [
        ("fx_cwipe_025.json", "completion"),
        ("fx_cwipe_026.json", "softness"),
        ("fx_cwipe_027.json", "blur"),
        ("fx_cwipe_028.json", "blur"),
        ("fx_cwipe_029.json", "property"),
        ("fx_cwipe_030.json", "fit"),
        ("fx_cwipe_031.json", "invert"),
        ("fx_cwipe_032.json", "layer"),
    ] {
        let why = holder_effect(&t, file).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    let whole = r#""layer": "", "fit": "stretch", "softness": 10, "property": "luminance", "blur": 0, "invert": "off""#;
    t.shape_refused("fx_cwipe_001.json", "a Channel Wipe whose completion is a word", &format!(r#"{{{whole}, "completion": "half"}}"#));
    t.shape_refused("fx_cwipe_001.json", "a Channel Wipe without its completion", &format!(r#"{{{whole}}}"#));
    t.shape_refused(
        "fx_cwipe_001.json",
        "a Channel Wipe without its property",
        r#"{"layer": "", "fit": "stretch", "completion": 0, "softness": 10, "blur": 0, "invert": "off"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_cwipe_008.json").document;
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = holder_effect(&t, "fx_cwipe_008.json");
        f(&mut e);
        e
    };
    let over = with(&|e| if let Effect::ChannelWipe { completion, .. } = e { *completion = 101.0 });
    let blurred = with(&|e| if let Effect::ChannelWipe { blur, .. } = e { *blur = 120.0 });
    let value = with(&|e| if let Effect::ChannelWipe { property, .. } = e { *property = "value".to_string() });
    let hue = with(&|e| if let Effect::ChannelWipe { layer, property, blur, .. } = e {
        *layer = J::from("");
        *property = "hue".to_string();
        *blur = 6.0;
    });
    // Draft halves the blur, a distance; the completion and the softness are not distances.
    let mut draft = with(&|e| if let Effect::ChannelWipe { blur, softness, .. } = e {
        (*blur, *softness) = (8.0, 30.0);
    });
    draft.scale_distances(|d| d * 0.5);
    let halved = draft == with(&|e| if let Effect::ChannelWipe { blur, softness, .. } = e {
        (*blur, *softness) = (4.0, 30.0);
    });
    let grows = with(&|_| {}).bounds_expansion();
    refused(
        &mut t,
        &mut document,
        vec![
            ("completion 101", set_holder(over)),
            ("blur 120", set_holder(blurred)),
            ("property \"value\"", set_holder(value)),
            ("blur keyed to 600", keys_holder("blur", &[(0, 0.0), (4, 600.0)])),
        ],
    );
    taken(
        &mut t,
        &mut document,
        "fx_cwipe_008.json",
        vec![
            ("the layer itself read in hue, blur 6", set_holder(hue)),
            ("completion keyed from 0 to 100", keys_holder("completion", &[(0, 0.0), (4, 100.0)])),
        ],
    );
    t.row(
        "a half-size draft preview halves the blur and keeps the completion and the softness",
        &format!("{draft:?}"),
        halved,
    );
    t.row("a wipe never grows the drawing's bounds", &grows.to_string(), grows == 0);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_cwipe_002.json", 0),
        ("fx_cwipe_003.json", 0),
        ("fx_cwipe_006.json", 0),
        ("fx_cwipe_008.json", 0),
        ("fx_cwipe_010.json", 0),
        ("fx_cwipe_011.json", 0),
        ("fx_cwipe_012.json", 0),
        ("fx_cwipe_013.json", 0),
        ("fx_cwipe_014.json", 0),
        ("fx_cwipe_015.json", 0),
        ("fx_cwipe_016.json", 0),
        ("fx_cwipe_017.json", 0),
        ("fx_cwipe_020.json", 2),
        ("fx_cwipe_023.json", 0),
        ("fx_cwipe_024.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Completion 0 (001), a layer not in the composition (021) and refused (025 to 033) leave
    // nothing for the card.
    let none: Vec<u32> = [1, 21].into_iter().chain(25..=33).collect();
    card_fixtures(&mut t, &mut gpu, "fx_cwipe", 33, Some(&none));
    card_reference(
        &mut t,
        &mut gpu,
        "Channel Wipe",
        fx,
        &[
            ("the layer itself, completion 40 (as added otherwise)", json!({"completion": 40})),
            ("layer 4 in hue, completion 50, softness 20, blur 6", json!({"layer": "layer-4", "property": "hue", "completion": 50, "softness": 20, "blur": 6})),
            ("layer 4 centred in alpha, completion 60, inverted", json!({"layer": "layer-4", "fit": "center", "property": "alpha", "completion": 60, "invert": "on"})),
            ("the layer itself in saturation, tiled layer 4 not used, completion 30, softness 0, blur 20", json!({"property": "saturation", "completion": 30, "softness": 0, "blur": 20})),
            ("completion 100", json!({"completion": 100})),
        ],
    );

    t.heading("Gradient Wipe unchanged: FX-GWIPE-001 to 028 (document 25), as B-129 runs them");
    let own = t.root.clone();
    t.root = repo("Fixtures/gradient_wipe");
    t.fixtures_numbered("expected_gradient_wipe.json", 1..=28);
    loads(&mut t, "expected_gradient_wipe.json");

    t.heading("Gradient Wipe unchanged on the card: its files and B-226's four reference settings");
    card_fixtures(&mut t, &mut gpu, "fx_gwipe", 28, None);
    t.root = own;
    let wipe = |fit: &str, completion: f64, softness: f64, invert: &str| {
        json!({"layer": "layer-4", "fit": fit, "completion": completion, "softness": softness, "invert": invert})
    };
    card_reference(
        &mut t,
        &mut gpu,
        "Gradient Wipe",
        gwipe,
        &[
            ("layer 4 stretched, completion 50, softness 0", wipe("stretch", 50.0, 0.0, "off")),
            ("layer 4 stretched, completion 30, softness 25, inverted", wipe("stretch", 30.0, 25.0, "on")),
            ("layer 4 centred, completion 70, softness 100", wipe("center", 70.0, 100.0, "off")),
            ("layer 4 tiled, completion 45, softness 10", wipe("tile", 45.0, 10.0, "off")),
        ],
    );

    t.heading("Pictures: in `verification/D-512 pictures/`");
    let dir = repo("verification/D-512 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], bytes).unwrap();
    // The street, and as another gradient, the street in its opposite colours.
    let street = town();
    let opposite: Vec<u8> = street.chunks_exact(4).flat_map(|p| [255 - p[0], 255 - p[1], 255 - p[2], p[3]]).collect();
    write("1_street.png", &street);
    write("2_reveal.png", &opposite);
    let px = |v: &[u8], i: usize| v[i * 4..i * 4 + 4].to_vec();
    let kept = |v: &[u8]| (0..w * h).filter(|&i| px(v, i) == px(&street, i)).count();
    let clear = |v: &[u8]| (0..w * h).filter(|&i| v[i * 4 + 3] == 0).count();

    let (half, said) = picture(&dir, json!([fx("fx-0-0", &json!({"completion": 50, "softness": 0}))]));
    write("3_itself_luminance_half.png", &half);
    t.row(
        "3_itself_luminance_half.png, the street read in its own luminance at completion 50, softness 0: the dark half of the street clear, the light half exactly as it was, nothing between; draws cleanly",
        &format!("{said:?}, {} pixels kept, {} clear, of {}", kept(&half), clear(&half), w * h),
        said.is_empty() && kept(&half) > w * h / 10 && clear(&half) > w * h / 10 && kept(&half) + clear(&half) == w * h,
    );

    let (hue, said) = picture(&dir, json!([fx("fx-0-0", &json!({"completion": 50, "softness": 20, "property": "hue"}))]));
    write("4_itself_hue_soft.png", &hue);
    t.row(
        "4_itself_hue_soft.png, the same read in hue, softness 20: other parts go first (reds and oranges before blues), soft edges between; draws cleanly",
        &format!("{said:?}, {} pixels differ from 3", distance(&hue, &half).1),
        said.is_empty() && distance(&hue, &half).1 > 0,
    );

    let (sharp, _) = picture(&dir, json!([fx("fx-0-0", &json!({"completion": 50}))]));
    let (blurred, said) = picture(&dir, json!([fx("fx-0-0", &json!({"completion": 50, "blur": 20}))]));
    write("5_itself_luminance_blur.png", &blurred);
    t.row(
        "5_itself_luminance_blur.png, luminance at completion 50 with blur 20: the cleared shapes rounded into soft blobs, not following every detail; draws cleanly",
        &format!("{said:?}, {} pixels differ from the same without blur", distance(&blurred, &sharp).1),
        said.is_empty() && distance(&blurred, &sharp).1 > 0,
    );

    let settings = json!({"layer": "reveal", "fit": "stretch", "completion": 50, "softness": 10, "invert": "off"});
    let (ours, said) = picture(&dir, json!([fx("fx-0-0", &settings)]));
    let (theirs, _) = picture(&dir, json!([gwipe("fx-0-0", &settings)]));
    write("6_other_layer.png", &ours);
    t.row(
        "6_other_layer.png, the opposite-colours layer as the gradient, luminance, completion 50: the light parts of the street go first; byte for byte Gradient Wipe's picture with the same settings, one engine; draws cleanly",
        &format!("{said:?}, {} pixels differ from Gradient Wipe's, {} clear", distance(&ours, &theirs).1, clear(&ours)),
        said.is_empty() && ours == theirs && clear(&ours) > w * h / 10,
    );

    let (done, said) = picture(&dir, json!([fx("fx-0-0", &json!({"completion": 100}))]));
    t.row(
        "completion 100: every pixel clear; draws cleanly",
        &format!("{said:?}, {} of {} clear", clear(&done), w * h),
        said.is_empty() && clear(&done) == w * h,
    );

    let (gone, said) = picture(&dir, json!([fx("fx-0-0", &json!({"completion": 50, "layer": "gone"}))]));
    t.row(
        "a gradient layer not in the composition: the street exactly as it was, with the warning",
        &format!("{said:?}, {} pixels changed", distance(&gone, &street).1),
        gone == street && said.iter().any(|s| s.starts_with("EFFECT_LAYER_MISSING")),
    );

    t.finish("D-512_channel_wipe_table.md");
}

/// The effect in `file` (on `holder`, or the adjustment layer in 024).
fn holder_effect(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    let layer = d.project().composition(&Id::new(MAIN)).unwrap().layers_in_order().find(|l| l.effects.iter().any(|e| matches!(e.effect, Effect::ChannelWipe { .. })));
    layer.expect("a layer with a Channel Wipe").effects[0].effect.clone()
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
/// With `B392_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-392: a measurement, run deliberately with --release --ignored"]
fn b392_channel_wipe_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B392_CPU").is_ok();
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
        ("Noise, then Channel Wipe, completion 50 (as added otherwise)", Some(json!({"completion": 50}))),
        ("Noise, then Channel Wipe, completion 50, layer 4 in hue, softness 20, blur 6", Some(json!({"completion": 50, "layer": "layer-4", "property": "hue", "softness": 20, "blur": 6}))),
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
    let out = std::env::var("B392_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-392_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
