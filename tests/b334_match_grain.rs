//! B-334: D-454 Match Grain, after After Effects' Match Grain ("Noise & Grain" in
//! `docs/effects/EFFECTS.md`): the grain of a whole source layer measured per channel by
//! Immerkaer's fast noise estimate, then laid by D-443's Add Grain at that strength.
//!
//! Writes `verification/D-454_match_grain_table.md`.
//!
//! Every expected pixel is `Fixtures/match_grain/expected_match_grain.json`, written by
//! `tools/match_grain_reference.py` before this code existed and printed in document 25 as
//! FX-MATCHGRAIN-001 to 023. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-454 pictures/`.

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

/// A Match Grain reading `layer`, Add Grain's starting values but `intensity` and `blending_mode`.
fn matched(layer: J, intensity: f64, blending_mode: &str) -> Effect {
    Effect::MatchGrain {
        layer,
        grain: Box::new(Effect::AddGrain {
            intensity,
            size: 1.0,
            softness: 0.0,
            aspect_ratio: 1.0,
            red_intensity: 1.0,
            green_intensity: 1.0,
            blue_intensity: 1.0,
            monochromatic: "off".into(),
            saturation: 1.0,
            blending_mode: blending_mode.into(),
            shadows: 1.0,
            midtones: 1.0,
            highlights: 1.0,
            midpoint: 0.5,
            animation_speed: 1.0,
            animate_smoothly: "on".into(),
            random_seed: 0.0,
            frame: 0,
        }),
        map: None,
    }
}

fn set_holder(effect: Effect) -> Command {
    Command::SetEffectParameters { composition: Id::new(MAIN), layer_id: Id::new("holder"), instance_id: Id::new("fx-0-0"), effect }
}

fn keys_holder(setting: &str, values: &[(i32, &[f64])]) -> Command {
    Command::SetEffectKeys {
        composition: Id::new(MAIN),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-0-0"),
        setting: setting.to_string(),
        keys: values.iter().map(|&(frame, v)| EffectKey { frame, value: v.to_vec(), interp: Interp::Linear }).collect(),
    }
}

fn holder_effect(d: &Document) -> Effect {
    d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0].effect.clone()
}

/// Each command on the `holder` file is refused with a sentence and leaves it as it was; then
/// each taken one is taken, and as many undos give back `file`'s frame 0 exactly.
fn holder_commands(t: &mut Table, file: &str, refused: Vec<(&str, Command)>, taken: Vec<(&str, Command)>) {
    let mut document = t.load(file).document;
    let held = format!("{:?}", document.project());
    for (what, command) in refused {
        let why = document.apply(command).err();
        let untouched = format!("{:?}", document.project()) == held;
        t.row(&format!("{what} is refused with a sentence, and nothing changes"), &why.as_ref().map_or("taken".into(), |d| d.message.clone()), why.is_some() && untouched);
    }
    let n = taken.len();
    for (what, command) in taken {
        let ok = document.apply(command).is_ok();
        t.row(&format!("{what} is taken"), if ok { "taken" } else { "refused" }, ok);
    }
    let before = t.render(&t.load(file).document, 0, 64);
    for _ in 0..n {
        document.undo();
    }
    let same = t.render(&document, 0, 64).data() == before.data();
    t.row(&format!("undo {n} times: frame 0 is the frame it was"), if same { "byte-identical" } else { "differ" }, same);
}

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

/// Add Grain's settings as it is added, and the layer.
const STARTING: &str = r#"{"intensity": 1, "size": 1, "softness": 0, "aspect_ratio": 1, "red_intensity": 1,
    "green_intensity": 1, "blue_intensity": 1, "monochromatic": "off", "saturation": 1, "blending_mode": "film",
    "shadows": 1, "midtones": 1, "highlights": 1, "midpoint": 0.5, "animation_speed": 1, "animate_smoothly": "on",
    "random_seed": 0}"#;

/// An effect of `type_id` with the settings `p`; for Match Grain and Add Grain every setting `p`
/// leaves out is the one a new one takes, since the file holds them all.
fn fx(type_id: &str, id: &str, p: &J) -> J {
    let mut all = match type_id {
        "core.add_grain" => serde_json::from_str(STARTING).unwrap(),
        "core.match_grain" => {
            let mut s: J = serde_json::from_str(STARTING).unwrap();
            s["layer"] = J::from("");
            s
        }
        _ => json!({}),
    };
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": all})
}

/// A setting keyed linearly through `points`, each a frame and a value.
fn keyed(points: &[(i32, J)]) -> J {
    let frames: Vec<J> = points.iter().map(|(f, v)| json!({"frame": f, "value": v, "interp": "linear"})).collect();
    json!({"base": points[0].1, "keyframes": frames})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` change nothing, are left out with a warning, or are refused, so the
/// card is not asked.
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
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor.
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J)], pick: fn(&Effect) -> bool) {
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
        let project = reference(|id| json!([fx("core.match_grain", &format!("b334-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Match Grain {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, pick);
                t.row(
                    &format!("the reference shot, Match Grain {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

/// `effects` on the street at frame 0, over a hidden plate: the street again with `plate` (an
/// Add Grain) on it, which a Match Grain reads as `plate`. Drawn, straight 8-bit, with what it
/// warned of.
fn picture(dir: &Path, effects: J, plate: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    let mut hidden = layer.clone();
    layer["effects"] = effects;
    hidden["id"] = J::from("plate");
    hidden["name"] = J::from("plate");
    hidden["enabled"] = J::from(false);
    hidden["effects"] = plate;
    comp["layers"].as_array_mut().unwrap().push(hidden);
    comp["layer_order"].as_array_mut().unwrap().push(J::from("plate"));
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// The spread of each channel's difference from `before`, in 8-bit levels: how strong the grain is.
fn spread(a: &[u8], before: &[u8]) -> [f64; 3] {
    let mut s = [0.0; 3];
    let n = (a.len() / 4) as f64;
    for c in 0..3 {
        let d: Vec<f64> = a.chunks_exact(4).zip(before.chunks_exact(4)).map(|(p, q)| p[c] as f64 - q[c] as f64).collect();
        let mean = d.iter().sum::<f64>() / n;
        s[c] = (d.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n).sqrt();
    }
    s
}

fn is_match(e: &Effect) -> bool {
    matches!(e, Effect::MatchGrain { .. })
}

#[test]
fn b334_match_grain() {
    let mut t = Table::new(
        "match_grain",
        "# D-454: Match Grain\n\nB-334, after After Effects' Match Grain: the grain of the whole \
         noise source layer is measured in red, green and blue by Immerkaer's fast noise estimate \
         (1996), the sum of a 3 by 3 Laplacian difference over every window the layer covers, and \
         D-443's Add Grain then lays grain whose spread in each channel is that measure times the \
         channel's intensity and the intensity. Add Grain's settings shape it. No noise source \
         leaves the layer as it is. The rule, ranges and starting values are ours, as Adobe \
         publishes none. Every expected pixel is \
         `Fixtures/match_grain/expected_match_grain.json`, written by \
         `tools/match_grain_reference.py` before this code existed and printed in document 25 as \
         FX-MATCHGRAIN-001 to 023. Tolerance 2e-5.\n",
    );

    t.heading("FX-MATCHGRAIN-001 to 023 (document 25)");
    t.fixtures("expected_match_grain.json");

    t.heading("The file");
    let files: Vec<String> = (1..=23).map(|n| format!("fx_matchgrain_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_matchgrain_016.json");
    t.row(
        "fx_matchgrain_016.json is saved with its noise source, its words and numbers as written, the intensity's keys kept, and no picture",
        &saved.to_string(),
        saved["layer"] == "steady" && saved["blending_mode"] == "add" && saved["intensity"]["keyframes"][1]["value"] == json!(4) && saved.get("map").is_none(),
    );
    for (file, want) in [
        ("fx_matchgrain_019.json", "Match Grain's intensity runs from 0 to 10, and this is 11."),
        ("fx_matchgrain_023.json", "Match Grain's noise source layer is the name of a layer of this composition, and this is 5."),
    ] {
        let why = holder_effect(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == want);
    }
    for (file, want) in [("fx_matchgrain_021.json", "\"screen\""), ("fx_matchgrain_022.json", "\"yes\"")] {
        let why = holder_effect(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want) && why.starts_with("Match Grain"));
    }
    let mut no_layer: J = serde_json::from_str(STARTING).unwrap();
    t.shape_refused("fx_matchgrain_002.json", "a Match Grain with no `layer`", &no_layer.to_string());
    no_layer["layer"] = J::from("steady");
    no_layer.as_object_mut().unwrap().remove("size");
    t.shape_refused("fx_matchgrain_002.json", "a Match Grain with no `size`", &no_layer.to_string());

    t.heading("Commands");
    holder_commands(
        &mut t,
        "fx_matchgrain_002.json",
        vec![
            ("intensity 10.5", set_holder(matched(J::from("steady"), 10.5, "film"))),
            ("blending mode \"screen\"", set_holder(matched(J::from("steady"), 1.0, "screen"))),
            ("the noise source a number", set_holder(matched(J::from(4), 1.0, "film"))),
            ("intensity keyed to 12", keys_holder("intensity", &[(0, &[0.0]), (4, &[12.0])])),
        ],
        vec![
            ("intensity 3 by Add, the noise source holes", set_holder(matched(J::from("holes"), 3.0, "add"))),
            ("intensity keyed from 0 to 10", keys_holder("intensity", &[(0, &[0.0]), (4, &[10.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_matchgrain_002.json", 0),
        ("fx_matchgrain_003.json", 3),
        ("fx_matchgrain_009.json", 0),
        ("fx_matchgrain_013.json", 0),
        ("fx_matchgrain_018.json", 2),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // FX-MATCHGRAIN-001 names no source, 014's source is flat so nothing is measured, 015's
    // source is missing, and 019 to 023 are refused, so the card is not asked.
    let mut none: Vec<u32> = vec![1, 14, 15];
    none.extend(19..=23);
    card_fixtures(&mut t, &mut gpu, "fx_matchgrain", 23, &none, is_match);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("layer2's grain, as added", json!({"layer": "layer-2"})),
            ("layer2's grain, intensity 3 by Add, monochromatic", json!({"layer": "layer-2", "intensity": 3, "blending_mode": "add", "monochromatic": "on"})),
            ("layer2's grain, size 2, softness 0.5, overlay, intensity keyed 0.5 to 5", json!({"layer": "layer-2", "size": 2, "softness": 0.5, "blending_mode": "overlay", "intensity": keyed(&[(0, json!(0.5)), (239, json!(5))])})),
        ],
        is_match,
    );

    t.heading("Pictures: the street, in `verification/D-454 pictures/`");
    let dir = repo("verification/D-454 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let light = json!([fx("core.add_grain", "p", &json!({"intensity": 1.5, "random_seed": 3}))]);
    let heavy = json!([fx("core.add_grain", "p", &json!({"intensity": 5, "random_seed": 3}))]);
    let (before, said) = picture(&dir, json!([]), light.clone());
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let one = |p: J| json!([fx("core.match_grain", "fx-0-0", &p)]);
    let (none_named, said) = picture(&dir, one(json!({})), light.clone());
    write("2_no_source.png", &none_named);
    t.row(
        "2_no_source.png, as added (no noise source): the street untouched; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&none_named, &before).1),
        said.is_empty() && distance(&none_named, &before).1 == 0,
    );
    let (lit, said) = picture(&dir, one(json!({"layer": "plate"})), light.clone());
    write("3_light_plate.png", &lit);
    let (hard, said_heavy) = picture(&dir, one(json!({"layer": "plate"})), heavy.clone());
    write("4_heavy_plate.png", &hard);
    let (s_light, s_heavy) = (spread(&lit, &before), spread(&hard, &before));
    t.row(
        "3_light_plate.png, matching a plate with light grain: fine grain over the street; draws cleanly",
        &format!("{said:?}, spread of the grain in red, green, blue {:.2?} levels", s_light),
        said.is_empty() && s_light.iter().all(|&s| s > 0.0),
    );
    t.row(
        "4_heavy_plate.png, matching a plate with heavy grain: heavier grain than 3, in every channel; draws cleanly",
        &format!("{said_heavy:?}, spread {:.2?} levels", s_heavy),
        said_heavy.is_empty() && (0..3).all(|c| s_heavy[c] > s_light[c]),
    );
    let (ghost, said) = picture(&dir, one(json!({"layer": "ghost"})), light);
    t.row(
        "a noise source that is no layer: the street untouched, and the warning says which layer",
        &format!("{said:?}, {} pixels changed", distance(&ghost, &before).1),
        distance(&ghost, &before).1 == 0 && said.iter().any(|s| s.starts_with(DiagnosticId::EffectLayerMissing.as_str()) && s.contains("ghost")),
    );

    t.finish("D-454_match_grain_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B334_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-334: a measurement, run deliberately with --release --ignored"]
fn b334_match_grain_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B334_CPU").is_ok();
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
        ("Noise, then Match Grain as added (no noise source)", Some(json!({}))),
        ("Noise, then Match Grain reading layer2", Some(json!({"layer": "layer-2"}))),
        ("Noise, then Match Grain reading layer2, size 3, softness 0.5, monochromatic", Some(json!({"layer": "layer-2", "size": 3, "softness": 0.5, "monochromatic": "on"}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx("core.noise", &format!("{id}n"), &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some(p) = &e {
                v.push(fx("core.match_grain", &format!("{id}m"), p));
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
    let out = std::env::var("B334_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-334_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
