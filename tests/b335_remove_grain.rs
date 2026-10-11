//! B-335: D-455 Remove Grain, after After Effects' Remove Grain ("Noise & Grain" in
//! `docs/effects/EFFECTS.md`): the layer's grain measured per channel by Immerkaer's fast noise
//! estimate, then smoothed by passes of a bilateral filter whose colour spread is twice the
//! measured noise times Noise Reduction, then sharpened by D-317's Sharpen as the Unsharp Mask.
//!
//! Writes `verification/D-455_remove_grain_table.md`.
//!
//! Every expected pixel is `Fixtures/remove_grain/expected_remove_grain.json`, written by
//! `tools/remove_grain_reference.py` before this code existed and printed in document 25 as
//! FX-REMOVEGRAIN-001 to 021. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-455 pictures/`.

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

/// A Remove Grain with no Unsharp Mask.
fn removing(noise_reduction: f64, passes: f64, mode: &str) -> Effect {
    Effect::RemoveGrain {
        noise_reduction,
        passes,
        mode: mode.into(),
        unsharp_amount: 0.0,
        unsharp_radius: 1.0,
        unsharp_threshold: 0.0,
        noise: None,
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

/// Remove Grain's settings as it is added.
const STARTING: &str = r#"{"noise_reduction": 1, "passes": 1, "mode": "multichannel", "unsharp_amount": 0,
    "unsharp_radius": 1, "unsharp_threshold": 0}"#;

/// An effect of `type_id` with the settings `p`; for Remove Grain every setting `p` leaves out is
/// the one a new one takes, since the file holds them all.
fn fx(type_id: &str, id: &str, p: &J) -> J {
    let mut all = if type_id == "core.remove_grain" { serde_json::from_str(STARTING).unwrap() } else { json!({}) };
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": all})
}

/// A Noise changing every frame, the grain Remove Grain is shown.
fn grain(id: &str) -> J {
    fx("core.noise", id, &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))
}

/// A setting keyed linearly through `points`, each a frame and a value.
fn keyed(points: &[(i32, J)]) -> J {
    let frames: Vec<J> = points.iter().map(|(f, v)| json!({"frame": f, "value": v, "interp": "linear"})).collect();
    json!({"base": points[0].1, "keyframes": frames})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` change nothing, are left to the processor, or are refused, so the
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

/// The reference shot with a Noise and then each setting on its first three layers, frames 0,
/// 100 and 239 at Full and Draft, on the card against the processor.
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J)], pick: fn(&Effect) -> bool) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|id| json!([grain(&format!("{id}n"))])));
    for (what, p) in settings {
        let project = reference(|id| json!([grain(&format!("{id}n")), fx("core.remove_grain", &format!("b335-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot with Noise, Remove Grain {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, pick);
                t.row(
                    &format!("the reference shot, Noise then Remove Grain {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

/// `effects` on the street at frame 0. Drawn, straight 8-bit, with what it warned of.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<String>) {
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

/// The average difference from the clean street over every channel, in 8-bit levels.
fn error(a: &[u8], clean: &[u8]) -> f64 {
    let (mut sum, mut n) = (0.0, 0.0);
    for (p, q) in a.chunks_exact(4).zip(clean.chunks_exact(4)) {
        for c in 0..3 {
            sum += (p[c] as f64 - q[c] as f64).abs();
            n += 1.0;
        }
    }
    sum / n
}

/// The average difference across the edges of the street (a step of 40 levels or more between
/// neighbours in the clean street), in 8-bit levels: how hard the edges stay.
fn edges(a: &[u8], clean: &[u8]) -> f64 {
    let w = TOWN.0;
    let (mut sum, mut n) = (0.0, 0.0);
    for i in 0..clean.len() / 4 {
        if (i + 1) % w == 0 {
            continue;
        }
        let step = |b: &[u8]| (0..3).map(|c| (b[4 * i + c] as f64 - b[4 * i + 4 + c] as f64).abs()).sum::<f64>();
        if step(clean) >= 40.0 {
            sum += step(a);
            n += 1.0;
        }
    }
    sum / n
}

/// The average difference between side-by-side pixels where the clean street has the same colour
/// on both, in 8-bit levels: how rough the plain areas are.
fn flat(a: &[u8], clean: &[u8]) -> f64 {
    let (mut sum, mut n) = (0.0, 0.0);
    for i in 0..clean.len() / 4 {
        if (i + 1) % TOWN.0 == 0 || clean[4 * i..4 * i + 3] != clean[4 * i + 4..4 * i + 7] {
            continue;
        }
        sum += (0..3).map(|c| (a[4 * i + c] as f64 - a[4 * i + 4 + c] as f64).abs()).sum::<f64>() / 3.0;
        n += 1.0;
    }
    sum / n
}

fn is_remove(e: &Effect) -> bool {
    matches!(e, Effect::RemoveGrain { .. })
}

#[test]
fn b335_remove_grain() {
    let mut t = Table::new(
        "remove_grain",
        "# D-455: Remove Grain\n\nB-335, after After Effects' Remove Grain: the grain of the layer \
         as it reaches the effect is measured in red, green and blue by Immerkaer's fast noise \
         estimate (1996), as Match Grain (D-454) measures it. Each pass k (1 to Passes) is a \
         bilateral filter (Tomasi and Manduchi, 1998) over a disc of radius 2k, spatial spread k, \
         on the encoded colour, whose colour spread is twice the measured noise times Noise \
         Reduction (the 2-sigma rule of Zhang and Gunturk, 2008); Multichannel weighs all three \
         channels together, Single Channel each alone. The Unsharp Mask is D-317's Sharpen. \
         Noise Reduction 0, or no grain measured, leaves the layer as it is. The rule, ranges \
         and starting values are ours, as Adobe publishes none. Every expected pixel is \
         `Fixtures/remove_grain/expected_remove_grain.json`, written by \
         `tools/remove_grain_reference.py` before this code existed and printed in document 25 as \
         FX-REMOVEGRAIN-001 to 021. Tolerance 2e-5.\n",
    );

    t.heading("FX-REMOVEGRAIN-001 to 021 (document 25)");
    t.fixtures("expected_remove_grain.json");

    t.heading("The file");
    let files: Vec<String> = (1..=21).map(|n| format!("fx_removegrain_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_removegrain_013.json");
    t.row(
        "fx_removegrain_013.json is saved with its words and numbers as written, Noise Reduction's keys kept, and no measured noise",
        &saved.to_string(),
        saved["mode"] == "multichannel" && saved["noise_reduction"]["keyframes"][1]["value"] == json!(3) && saved.get("noise").is_none(),
    );
    for (file, want) in [
        ("fx_removegrain_017.json", "3.5"),
        ("fx_removegrain_018.json", "0"),
        ("fx_removegrain_019.json", "\"both\""),
        ("fx_removegrain_020.json", "600"),
    ] {
        let why = holder_effect(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want) && why.starts_with("Remove Grain"));
    }
    let mut no_passes: J = serde_json::from_str(STARTING).unwrap();
    no_passes.as_object_mut().unwrap().remove("passes");
    t.shape_refused("fx_removegrain_001.json", "a Remove Grain with no `passes`", &no_passes.to_string());

    t.heading("Commands");
    holder_commands(
        &mut t,
        "fx_removegrain_001.json",
        vec![
            ("noise reduction 3.5", set_holder(removing(3.5, 1.0, "multichannel"))),
            ("mode \"both\"", set_holder(removing(1.0, 1.0, "both"))),
            ("passes keyed to 5", keys_holder("passes", &[(0, &[1.0]), (4, &[5.0])])),
        ],
        vec![
            ("noise reduction 2, two passes, single channel", set_holder(removing(2.0, 2.0, "single_channel"))),
            ("passes keyed from 1 to 4", keys_holder("passes", &[(0, &[1.0]), (4, &[4.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_removegrain_001.json", 0),
        ("fx_removegrain_006.json", 0),
        ("fx_removegrain_007.json", 0),
        ("fx_removegrain_010.json", 0),
        ("fx_removegrain_011.json", 0),
        ("fx_removegrain_013.json", 2),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // FX-REMOVEGRAIN-002 (Noise Reduction 0) and 012 (no grain measured) change nothing, 010's
    // Unsharp Mask has a threshold, which the card does not draw (D-317), and 017 to 021 are
    // refused, so the card is not asked. 013's frame 0 is Noise Reduction 0.
    let mut none: Vec<u32> = vec![2, 10, 12];
    none.extend(17..=21);
    card_fixtures(&mut t, &mut gpu, "fx_removegrain", 21, &none, is_remove);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({})),
            ("Noise Reduction 2, three passes, single channel", json!({"noise_reduction": 2, "passes": 3, "mode": "single_channel"})),
            ("Noise Reduction keyed 0.5 to 3, Unsharp Mask 150 at radius 2", json!({"noise_reduction": keyed(&[(0, json!(0.5)), (239, json!(3))]), "unsharp_amount": 150, "unsharp_radius": 2})),
        ],
        is_remove,
    );

    t.heading("Pictures: the street, in `verification/D-455 pictures/`");
    let dir = repo("verification/D-455 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (clean, said) = picture(&dir, json!([]));
    write("1_clean.png", &clean);
    t.row("1_clean.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let stack = |p: J| json!([grain("n"), fx("core.remove_grain", "fx-0-0", &p)]);
    let (grainy, said) = picture(&dir, json!([grain("n")]));
    write("2_grainy.png", &grainy);
    let (e_grainy, edge_clean) = (error(&grainy, &clean), edges(&clean, &clean));
    t.row(
        "2_grainy.png, the street with Noise: grain everywhere; draws cleanly",
        &format!("{said:?}, average {e_grainy:.2} levels from the clean street"),
        said.is_empty() && e_grainy > 1.0,
    );
    let (zero, said) = picture(&dir, stack(json!({"noise_reduction": 0})));
    t.row(
        "Noise Reduction 0: the grainy street untouched; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&zero, &grainy).1),
        said.is_empty() && distance(&zero, &grainy).1 == 0,
    );
    let (added, said) = picture(&dir, stack(json!({})));
    write("3_as_added.png", &added);
    let (e_added, edge_added) = (error(&added, &clean), edges(&added, &clean));
    t.row(
        "3_as_added.png, Remove Grain as added: closer to the clean street than 2, and the edges keep at least nine tenths of their step; draws cleanly",
        &format!("{said:?}, average {e_added:.2} levels from the clean street; edges {edge_added:.1} against {edge_clean:.1} clean"),
        said.is_empty() && e_added < e_grainy && edge_added >= 0.9 * edge_clean,
    );
    let (strong, said) = picture(&dir, stack(json!({"noise_reduction": 3, "passes": 3})));
    write("4_strong.png", &strong);
    let (f_added, f_strong, edge_strong) = (flat(&added, &clean), flat(&strong, &clean), edges(&strong, &clean));
    t.row(
        "4_strong.png, Noise Reduction 3, three passes: plain areas smoother than in 3, at the cost of softer edges (the colour spread is now wider than most edges); draws cleanly",
        &format!("{said:?}, roughness of plain areas {f_strong:.2} levels against {f_added:.2} in 3; edges {edge_strong:.1} against {edge_clean:.1} clean"),
        said.is_empty() && f_strong < f_added,
    );
    let (sharp, said) = picture(&dir, stack(json!({"noise_reduction": 3, "passes": 3, "unsharp_amount": 150, "unsharp_radius": 1.5})));
    write("5_strong_sharpened.png", &sharp);
    let edge_sharp = edges(&sharp, &clean);
    t.row(
        "5_strong_sharpened.png, 4 then Unsharp Mask 150 at radius 1.5: harder edges than 4; draws cleanly",
        &format!("{said:?}, edges {edge_sharp:.1} against {edge_strong:.1} in 4"),
        said.is_empty() && edge_sharp > edge_strong,
    );

    t.finish("D-455_remove_grain_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B335_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-335: a measurement, run deliberately with --release --ignored"]
fn b335_remove_grain_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B335_CPU").is_ok();
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
        ("Noise, then Remove Grain as added", Some(json!({}))),
        ("Noise, then Remove Grain Noise Reduction 2, three passes, single channel", Some(json!({"noise_reduction": 2, "passes": 3, "mode": "single_channel"}))),
        ("Noise, then Remove Grain as added with Unsharp Mask 150", Some(json!({"unsharp_amount": 150}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![grain(&format!("{id}n"))];
            if let Some(p) = &e {
                v.push(fx("core.remove_grain", &format!("{id}r"), p));
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
    let out = std::env::var("B335_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-335_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
