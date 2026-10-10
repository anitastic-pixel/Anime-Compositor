//! B-304: D-425 Threads, our name for CycoreFX's CC Threads ("Generate" in
//! `docs/effects/EFFECTS.md`): the layer woven into a cloth of threads in its own colours.
//!
//! Writes `verification/D-425_threads_table.md`.
//!
//! Every expected pixel is `Fixtures/threads/expected_threads.json`, written by
//! `tools/threads_reference.py` before this code existed and printed in document 25 as
//! FX-THREADS-001 to 028. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-425 pictures/`.

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
        "core.threads" => json!({"width": 50, "height": 50, "overlaps": 1, "direction": 0, "center": [50, 50], "coverage": 90, "shadowing": 50, "texture": 0}),
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
        let project = reference(|id| json!([fx(type_id, &format!("b304-{id}"), p)]));
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

// --- Threads ---------------------------------------------------------------------------------

fn is_threads(e: &Effect) -> bool {
    matches!(e, Effect::Threads { .. })
}

/// A Threads as a new one starts, changed by `change`.
fn threads(change: impl FnOnce(&mut Effect)) -> Effect {
    let mut e = Effect::Threads {
        width: 50.0,
        height: 50.0,
        overlaps: 1.0,
        direction: 0.0,
        center: [50.0, 50.0],
        coverage: 90.0,
        shadowing: 50.0,
        texture: 0.0,
    };
    change(&mut e);
    e
}

/// Set the `holder` layer's Threads; its fixtures' drawing is `holder`, not `art`.
fn set_holder(effect: Effect) -> Command {
    Command::SetEffectParameters { composition: Id::new(MAIN), layer_id: Id::new("holder"), instance_id: Id::new("fx-1"), effect }
}

/// Key a setting of the `holder` layer's Threads, linearly.
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
fn b304_threads() {
    let mut t = Table::new(
        "threads",
        "# D-425: Threads\n\nB-304, after CycoreFX's CC Threads: the layer woven into a cloth of \
         threads in its own colours. Warp threads Width apart and weft threads Height apart, laid \
         about Center and turned by Direction, each Coverage of its spacing wide; each passes over \
         and under Overlaps of the others (taken whole); the thread beneath is darkened by \
         Shadowing beside the one above, and Texture shades each thread round. Where neither \
         thread lies the layer is clear. The manual gives no formula, ranges or defaults, so those \
         are ours. Every expected pixel is `Fixtures/threads/expected_threads.json`, written by \
         `tools/threads_reference.py` before this code existed and printed in document 25 as \
         FX-THREADS-001 to 028. Tolerance 2e-5.\n",
    );

    t.heading("FX-THREADS-001 to 028 (document 25)");
    t.fixtures("expected_threads.json");

    t.heading("How far it reaches");
    let got = threads(|_| {}).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing: the cloth is woven on the layer as it is", &got.to_string(), got == 0);
    let mut draft = threads(|e| {
        if let Effect::Threads { width, height, .. } = e {
            *width = 8.0;
            *height = 6.0;
        }
    });
    draft.scale_distances(|d| d * 0.5);
    let want = threads(|e| {
        if let Effect::Threads { width, height, .. } = e {
            *width = 4.0;
            *height = 3.0;
        }
    });
    t.row(
        "a half-size draft preview halves the thread spacing; the centre is a share of the drawing",
        &format!("{draft:?}"),
        draft == want,
    );

    t.heading("The file");
    let all = files("fx_threads", 28);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_threads_019.json");
    t.row(
        "fx_threads_019.json is saved with all 8 settings",
        &saved.to_string(),
        saved["width"] == 5
            && saved["height"] == 3
            && saved["overlaps"] == 2
            && saved["direction"] == -20
            && saved["center"] == json!([40, 60])
            && saved["coverage"] == 85
            && saved["shadowing"] == 70
            && saved["texture"] == 60
            && saved.as_object().unwrap().len() == 8,
    );
    let keyed = t.saved_parameters("fx_threads_015.json");
    t.row(
        "fx_threads_015.json is saved with the direction's two keys",
        &keyed["direction"].to_string(),
        keyed["direction"]["keyframes"].as_array().is_some_and(|k| k.len() == 2),
    );
    why_holder(
        &mut t,
        &[
            ("fx_threads_020.json", "Threads's width runs from 1 to 1000, and this is 0.5."),
            ("fx_threads_021.json", "Threads's height runs from 1 to 1000, and this is 1001."),
            ("fx_threads_022.json", "Threads's overlaps runs from 1 to 10, and this is 0."),
            ("fx_threads_023.json", "Threads's overlaps runs from 1 to 10, and this is 11."),
            ("fx_threads_024.json", "Threads's direction runs from -3600 to 3600, and this is 3601."),
            ("fx_threads_025.json", "Threads's center runs from -1000 to 1000, and this is -1001."),
            ("fx_threads_026.json", "Threads's coverage runs from 0 to 100, and this is 101."),
            ("fx_threads_027.json", "Threads's shadowing runs from 0 to 100, and this is -1."),
            ("fx_threads_028.json", "Threads's texture runs from 0 to 100, and this is 101."),
        ],
    );
    let rest = r#""height": 50, "overlaps": 1, "direction": 0, "center": [50, 50], "coverage": 90, "shadowing": 50, "texture": 0"#;
    t.shape_refused("fx_threads_001.json", "a Threads with no `width`", &format!("{{{rest}}}"));
    t.shape_refused("fx_threads_001.json", "a Threads whose width is a word", &format!("{{\"width\": \"50\", {rest}}}"));
    t.shape_refused("fx_threads_001.json", "a Threads whose center is one number", &format!("{{\"width\": 50, {}}}", rest.replace("[50, 50]", "50")));
    t.shape_refused("fx_threads_001.json", "a Threads with no `texture`", &format!("{{\"width\": 50, {}}}", rest.replace(", \"texture\": 0", "")));

    t.heading("Commands");
    let mut document = t.load("fx_threads_001.json").document;
    refused_holder(
        &mut t,
        &mut document,
        vec![
            ("width 0.5", set_holder(threads(|e| if let Effect::Threads { width, .. } = e { *width = 0.5 }))),
            ("height 1000.5", set_holder(threads(|e| if let Effect::Threads { height, .. } = e { *height = 1000.5 }))),
            ("overlaps 10.5", set_holder(threads(|e| if let Effect::Threads { overlaps, .. } = e { *overlaps = 10.5 }))),
            ("coverage -0.5", set_holder(threads(|e| if let Effect::Threads { coverage, .. } = e { *coverage = -0.5 }))),
            ("center 1001, 50", set_holder(threads(|e| if let Effect::Threads { center, .. } = e { *center = [1001.0, 50.0] }))),
            ("direction keyed to 3601", keys_holder("direction", &[(0, &[0.0]), (4, &[3601.0])])),
            ("texture keyed to 150", keys_holder("texture", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_threads_001.json",
        vec![
            (
                "the tops: width and height 1000, overlaps 10, direction 3600, coverage, shadowing and texture 100",
                set_holder(threads(|e| {
                    if let Effect::Threads { width, height, overlaps, direction, coverage, shadowing, texture, .. } = e {
                        *width = 1000.0;
                        *height = 1000.0;
                        *overlaps = 10.0;
                        *direction = 3600.0;
                        *coverage = 100.0;
                        *shadowing = 100.0;
                        *texture = 100.0;
                    }
                })),
            ),
            (
                "the bottoms: width and height 1, overlaps 1, direction -3600, coverage, shadowing and texture 0",
                set_holder(threads(|e| {
                    if let Effect::Threads { width, height, overlaps, direction, coverage, shadowing, texture, .. } = e {
                        *width = 1.0;
                        *height = 1.0;
                        *overlaps = 1.0;
                        *direction = -3600.0;
                        *coverage = 0.0;
                        *shadowing = 0.0;
                        *texture = 0.0;
                    }
                })),
            ),
            ("overlaps 2.5, read whole", set_holder(threads(|e| if let Effect::Threads { overlaps, .. } = e { *overlaps = 2.5 }))),
            ("direction keyed from 0 to 90", keys_holder("direction", &[(0, &[0.0]), (4, &[90.0])])),
            ("center keyed from 0, 0 to 100, 100", keys_holder("center", &[(0, &[0.0, 0.0]), (4, &[100.0, 100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_threads_001.json", 0),
        ("fx_threads_002.json", 0),
        ("fx_threads_008.json", 0),
        ("fx_threads_011.json", 0),
        ("fx_threads_014.json", 0),
        ("fx_threads_016.json", 2),
        ("fx_threads_019.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_threads", 28, &[20, 21, 22, 23, 24, 25, 26, 27, 28], is_threads);
    let turning = json!({"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"}, {"frame": 239, "value": 90, "interp": "linear"}]});
    card_reference(
        &mut t,
        &mut gpu,
        "Threads",
        "core.threads",
        &[
            ("as it starts (50 apart, coverage 90, shadowing 50)", json!({})),
            ("12 by 8, overlaps 3, turned 30 degrees, shadowing 100, texture 60", json!({"width": 12, "height": 8, "overlaps": 3, "direction": 30, "shadowing": 100, "texture": 60})),
            ("the direction keyed 0 to 90, width 20, coverage 60, centre 30, 70", json!({"direction": turning, "width": 20, "height": 20, "coverage": 60, "center": [30, 70]})),
        ],
        is_threads,
    );

    t.heading("Pictures: the street at frame 0, in `verification/D-425 pictures/`");
    let dir = repo("verification/D-425 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (before, said) = street(&dir, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let shots: [(&str, J, &str); 5] = [
        ("as_added", json!({}), "as it starts: threads 50 pixels apart crossing over the street, thin clear gaps between them"),
        ("fine_weave", json!({"width": 8, "height": 8, "shadowing": 80}), "8 pixels apart, shadowing 80: a fine plain weave, the threads beneath darker"),
        ("twill_turned", json!({"width": 10, "height": 10, "overlaps": 3, "direction": 45, "shadowing": 80, "texture": 50}), "10 apart, overlaps 3, turned 45 degrees, texture 50: a diagonal twill of rounded threads"),
        ("open_cloth", json!({"width": 16, "height": 16, "coverage": 50, "texture": 100}), "16 apart, coverage 50, texture 100: an open basket of round threads with big clear holes"),
        ("ribbons", json!({"width": 40, "height": 12, "coverage": 80, "shadowing": 100}), "width 40, height 12, coverage 80: broad ribbons across narrow ones"),
    ];
    for (i, (name, p, what)) in shots.iter().enumerate() {
        let file = format!("{}_{name}.png", i + 2);
        let (bytes, said) = street(&dir, json!([fx("core.threads", "fx-0-0", p)]));
        write(&file, &bytes);
        t.row(
            &format!("{file}, {what}; draws cleanly"),
            &format!("{said:?}, {} pixels changed", distance(&bytes, &before).1),
            said.is_empty() && changed(&bytes, &before),
        );
    }

    t.finish("D-425_threads_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B304_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-304: a measurement, run deliberately with --release --ignored"]
fn b304_threads_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B304_CPU").is_ok();
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
        ("Noise, then Threads as added (50 apart, coverage 90, shadowing 50)", Some(("core.threads", json!({})))),
        ("Noise, then Threads, 12 by 8, overlaps 3, turned 30, shadowing 100, texture 60", Some(("core.threads", json!({"width": 12, "height": 8, "overlaps": 3, "direction": 30, "shadowing": 100, "texture": 60})))),
        ("Noise, then Threads, 4 by 4, coverage 50, texture 100", Some(("core.threads", json!({"width": 4, "height": 4, "coverage": 50, "texture": 100})))),
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
    let out = std::env::var("B304_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-304_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
