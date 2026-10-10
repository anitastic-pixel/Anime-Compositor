//! B-305: D-426 Lens Flare, after After Effects' Lens Flare ("Generate" in
//! `docs/effects/EFFECTS.md`): the flare a bright light makes in a camera lens, over the layer.
//!
//! Writes `verification/D-426_lens_flare_table.md`.
//!
//! Every expected pixel is `Fixtures/lens_flare/expected_lens_flare.json`, written by
//! `tools/lens_flare_reference.py` before this code existed and printed in document 25 as
//! FX-FLARE-001 to 019. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-426 pictures/`.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{repo, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
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

/// The effects `pick` takes that the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality, pick: fn(&Effect) -> bool) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if pick(&f.instance.effect)))
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

/// An effect of `type_id` with the settings `p`; for the ones here, every setting `p` leaves
/// out is the one a new one takes, since the file holds them all.
fn fx(type_id: &str, id: &str, p: &J) -> J {
    let mut all = match type_id {
        "core.lens_flare" => json!({"flare_center": [30, 30], "flare_brightness": 100, "lens_type": "zoom", "blend_with_original": 0}),
        _ => json!({}),
    };
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` change nothing or are left out with a warning, so the card is not asked.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, stem: &str, count: u32, none: &[u32], pick: fn(&Effect) -> bool) {
    let comp = Id::new(MAIN);
    for n in 1..=count {
        let file = format!("{stem}_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, pick);
            }
        }
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor.
fn card_reference(t: &mut Table, gpu: &mut Gpu, name: &str, type_id: &str, settings: &[(&str, J)], pick: fn(&Effect) -> bool) {
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
        let project = reference(|id| json!([fx(type_id, &format!("b305-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, {name} {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, pick);
                t.row(
                    &format!("the reference shot, {name} {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

fn changed(a: &[u8], b: &[u8]) -> bool {
    distance(a, b).1 > 0
}

fn files(stem: &str, count: u32) -> Vec<String> {
    (1..=count).map(|n| format!("{stem}_{n:03}.json")).collect()
}

// --- Lens Flare --------------------------------------------------------------------------------

fn is_flare(e: &Effect) -> bool {
    matches!(e, Effect::LensFlare { .. })
}

/// A Lens Flare as a new one starts, changed by `change`.
fn flare(change: impl FnOnce(&mut Effect)) -> Effect {
    let mut e = Effect::LensFlare {
        flare_center: [30.0, 30.0],
        flare_brightness: 100.0,
        lens_type: "zoom".to_string(),
        blend_with_original: 0.0,
    };
    change(&mut e);
    e
}

/// Set the `holder` layer's Lens Flare; its fixtures' drawing is `holder`, not `art`.
fn set_holder(effect: Effect) -> Command {
    Command::SetEffectParameters { composition: Id::new(MAIN), layer_id: Id::new("holder"), instance_id: Id::new("fx-1"), effect }
}

/// Key a setting of the `holder` layer's Lens Flare, linearly.
fn keys_holder(setting: &str, values: &[(i32, &[f64])]) -> Command {
    Command::SetEffectKeys {
        composition: Id::new(MAIN),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-1"),
        setting: setting.to_string(),
        keys: values.iter().map(|&(frame, v)| EffectKey { frame, value: v.to_vec(), interp: Interp::Linear }).collect(),
    }
}

/// Each command is refused with a sentence and leaves the `holder` layer's effect as it was.
fn refused_holder(t: &mut Table, document: &mut Document, commands: Vec<(&str, Command)>) {
    let held = |d: &Document| format!("{:?}", d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0]);
    let before = held(document);
    for (what, command) in commands {
        let refused = document.apply(command).err();
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && held(document) == before,
        );
    }
}

fn why_holder(t: &mut Table, cases: &[(&str, &str)]) {
    for (file, want) in cases {
        let doc = t.load(file).document;
        let why = doc.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0].effect.why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == *want);
    }
}

/// `effects` on the street at frame 0, drawn, straight 8-bit, with what it warned of.
fn street(dir: &Path, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

#[test]
fn b305_lens_flare() {
    let mut t = Table::new(
        "lens_flare",
        "# D-426: Lens Flare\n\nB-305, after After Effects' Lens Flare: the flare a bright light \
         makes shining into a camera lens, added over the layer. Its controls are After Effects' \
         (Flare Center, Flare Brightness, Lens Type: 50-300mm Zoom, 35mm Prime or 105mm Prime, \
         Blend With Original); each lens is a fixed set of glows, a halo ring, a star of rays and \
         coloured ghost discs placed along the line from the flare through the layer's middle, \
         after the parts of Video Copilot's Optical Flares. The parts, sizes, colours, ranges and \
         defaults are ours. Every expected pixel is `Fixtures/lens_flare/expected_lens_flare.json`, \
         written by `tools/lens_flare_reference.py` before this code existed and printed in \
         document 25 as FX-FLARE-001 to 019. Tolerance 2e-5.\n",
    );

    t.heading("FX-FLARE-001 to 019 (document 25)");
    t.fixtures("expected_lens_flare.json");

    t.heading("How far it reaches");
    let got = flare(|_| {}).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing: the flare is drawn on the layer as it is", &got.to_string(), got == 0);
    let mut draft = flare(|_| {});
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes no setting: the flare's place is a share of the drawing and its parts' sizes shares of its diagonal",
        &format!("{draft:?}"),
        draft == flare(|_| {}),
    );

    t.heading("The file");
    let all = files("fx_flare", 19);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_flare_014.json");
    t.row(
        "fx_flare_014.json is saved with all 4 settings",
        &saved.to_string(),
        saved["flare_center"] == json!([70, 20])
            && saved["flare_brightness"] == 150
            && saved["lens_type"] == "105mm"
            && saved["blend_with_original"] == 25
            && saved.as_object().unwrap().len() == 4,
    );
    let keyed = t.saved_parameters("fx_flare_011.json");
    t.row(
        "fx_flare_011.json is saved with the centre's two keys",
        &keyed["flare_center"].to_string(),
        keyed["flare_center"]["keyframes"].as_array().is_some_and(|k| k.len() == 2),
    );
    why_holder(
        &mut t,
        &[
            ("fx_flare_015.json", "Lens Flare's flare brightness runs from 0 to 300, and this is 301."),
            ("fx_flare_016.json", "Lens Flare's flare brightness runs from 0 to 300, and this is -1."),
            ("fx_flare_017.json", "Lens Flare's blend with original runs from 0 to 100, and this is 101."),
            ("fx_flare_018.json", "Lens Flare's lens type is \"zoom\", \"35mm\" or \"105mm\", and this is \"200mm\"."),
            ("fx_flare_019.json", "Lens Flare's flare center runs from -1000 to 1000, and this is 1001."),
        ],
    );
    let rest = r#""flare_brightness": 100, "lens_type": "zoom", "blend_with_original": 0"#;
    t.shape_refused("fx_flare_001.json", "a Lens Flare with no `flare_center`", &format!("{{{rest}}}"));
    t.shape_refused("fx_flare_001.json", "a Lens Flare whose centre is one number", &format!("{{\"flare_center\": 30, {rest}}}"));
    t.shape_refused("fx_flare_001.json", "a Lens Flare whose brightness is a word", &format!("{{\"flare_center\": [30, 30], {}}}", rest.replace("100", "\"100\"")));
    t.shape_refused("fx_flare_001.json", "a Lens Flare whose lens type is a number", &format!("{{\"flare_center\": [30, 30], {}}}", rest.replace("\"zoom\"", "50")));
    t.shape_refused("fx_flare_001.json", "a Lens Flare with no `lens_type`", &format!("{{\"flare_center\": [30, 30], {}}}", rest.replace(", \"lens_type\": \"zoom\"", "")));

    t.heading("Commands");
    let mut document = t.load("fx_flare_001.json").document;
    refused_holder(
        &mut t,
        &mut document,
        vec![
            ("brightness 300.5", set_holder(flare(|e| if let Effect::LensFlare { flare_brightness, .. } = e { *flare_brightness = 300.5 }))),
            ("brightness -0.5", set_holder(flare(|e| if let Effect::LensFlare { flare_brightness, .. } = e { *flare_brightness = -0.5 }))),
            ("blend 100.5", set_holder(flare(|e| if let Effect::LensFlare { blend_with_original, .. } = e { *blend_with_original = 100.5 }))),
            ("lens type \"fisheye\"", set_holder(flare(|e| if let Effect::LensFlare { lens_type, .. } = e { *lens_type = "fisheye".to_string() }))),
            ("centre -1001, 50", set_holder(flare(|e| if let Effect::LensFlare { flare_center, .. } = e { *flare_center = [-1001.0, 50.0] }))),
            ("brightness keyed to 301", keys_holder("flare_brightness", &[(0, &[0.0]), (4, &[301.0])])),
            ("centre keyed to 50, 1001", keys_holder("flare_center", &[(0, &[0.0, 0.0]), (4, &[50.0, 1001.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_flare_001.json",
        vec![
            (
                "the tops: centre 1000, 1000, brightness 300, blend 100, the 35mm prime",
                set_holder(flare(|e| {
                    if let Effect::LensFlare { flare_center, flare_brightness, lens_type, blend_with_original } = e {
                        *flare_center = [1000.0, 1000.0];
                        *flare_brightness = 300.0;
                        *lens_type = "35mm".to_string();
                        *blend_with_original = 100.0;
                    }
                })),
            ),
            (
                "the bottoms: centre -1000, -1000, brightness 0, blend 0, the 105mm prime",
                set_holder(flare(|e| {
                    if let Effect::LensFlare { flare_center, flare_brightness, lens_type, blend_with_original } = e {
                        *flare_center = [-1000.0, -1000.0];
                        *flare_brightness = 0.0;
                        *lens_type = "105mm".to_string();
                        *blend_with_original = 0.0;
                    }
                })),
            ),
            ("brightness keyed from 0 to 300", keys_holder("flare_brightness", &[(0, &[0.0]), (4, &[300.0])])),
            ("centre keyed from 0, 0 to 100, 100", keys_holder("flare_center", &[(0, &[0.0, 0.0]), (4, &[100.0, 100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_flare_001.json", 0),
        ("fx_flare_005.json", 0),
        ("fx_flare_006.json", 0),
        ("fx_flare_009.json", 0),
        ("fx_flare_010.json", 0),
        ("fx_flare_011.json", 2),
        ("fx_flare_013.json", 0),
        ("fx_flare_014.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_flare", 19, &[15, 16, 17, 18, 19], is_flare);
    let across = json!({"base": [0, 0], "keyframes": [{"frame": 0, "value": [10, 20], "interp": "linear"}, {"frame": 239, "value": [90, 80], "interp": "linear"}]});
    card_reference(
        &mut t,
        &mut gpu,
        "Lens Flare",
        "core.lens_flare",
        &[
            ("as it starts (the 50-300mm zoom at 30, 30, brightness 100)", json!({})),
            ("the 35mm prime at 80, 60, brightness 200, blend 30", json!({"lens_type": "35mm", "flare_center": [80, 60], "flare_brightness": 200, "blend_with_original": 30})),
            ("the 105mm prime, its centre keyed from 10, 20 to 90, 80", json!({"lens_type": "105mm", "flare_center": across})),
        ],
        is_flare,
    );

    t.heading("Pictures: the street at frame 0, in `verification/D-426 pictures/`");
    let dir = repo("verification/D-426 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (before, said) = street(&dir, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let shots: [(&str, J, &str); 5] = [
        ("as_added", json!({}), "as it starts: the 50-300mm zoom at 30, 30, a hot glow and rays up and left, a blue halo, ghosts down to the right"),
        ("prime_35mm", json!({"lens_type": "35mm"}), "the 35mm prime: a smaller, whiter glow with eight rays and three ghosts"),
        ("prime_105mm", json!({"lens_type": "105mm", "flare_center": [75, 25]}), "the 105mm prime at 75, 25: a large white glow, twelve long rays, a faint warm halo"),
        ("bright", json!({"flare_brightness": 250, "flare_center": [50, 20]}), "brightness 250 at 50, 20: the glow washes the sky white"),
        ("blended", json!({"blend_with_original": 60}), "blend 60: the zoom flare at 40% strength"),
    ];
    for (i, (name, p, what)) in shots.iter().enumerate() {
        let file = format!("{}_{name}.png", i + 2);
        let (bytes, said) = street(&dir, json!([fx("core.lens_flare", "fx-0-0", p)]));
        write(&file, &bytes);
        t.row(
            &format!("{file}, {what}; draws cleanly"),
            &format!("{said:?}, {} pixels changed", distance(&bytes, &before).1),
            said.is_empty() && changed(&bytes, &before),
        );
    }

    t.finish("D-426_lens_flare_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B305_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-305: a measurement, run deliberately with --release --ignored"]
fn b305_lens_flare_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B305_CPU").is_ok();
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
    let shots: [(&str, Option<(&str, J)>); 4] = [
        ("Noise alone", None),
        ("Noise, then Lens Flare as added (the 50-300mm zoom, ten parts)", Some(("core.lens_flare", json!({})))),
        ("Noise, then Lens Flare, the 35mm prime (six parts), brightness 200", Some(("core.lens_flare", json!({"lens_type": "35mm", "flare_brightness": 200})))),
        ("Noise, then Lens Flare, the 105mm prime at 75, 25, blend 30", Some(("core.lens_flare", json!({"lens_type": "105mm", "flare_center": [75, 25], "blend_with_original": 30})))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx("core.noise", &format!("{id}n"), &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some((type_id, p)) = &e {
                v.push(fx(type_id, &format!("{id}c"), p));
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
    let out = std::env::var("B305_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-305_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
