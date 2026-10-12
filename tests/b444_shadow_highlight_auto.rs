//! B-444: D-564, Shadow/Highlight's Auto Amounts, Temporal Smoothing and Scene Detect, after
//! After Effects' Shadow/Highlight: with Auto Amounts on, the two amounts come from the picture's
//! own brightness histogram (this program's rule, unverified) instead of the sliders, read over
//! the frames either side by D-351's Temporal Smoothing and Scene Detect.
//!
//! Writes `verification/D-564_shadow_highlight_auto_table.md` and draws pictures into
//! `verification/D-564 pictures/`.
//!
//! Every expected pixel is `Fixtures/shadow_highlight_auto/expected_shadow_highlight_auto.json`,
//! written by `tools/shadow_highlight_auto_reference.py` before this code existed and printed in
//! document 25 as FX-SHAUTO-001 to 014. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// The Shadow/Highlights the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::ShadowHighlight { .. })))
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
    // The card handing the frame over is reported as `refused`; the rest must agree.
    let said_gpu = said_gpu.split(", ").filter(|id| *id != DiagnosticId::GpuPreviewOnCpu.as_str()).collect::<Vec<_>>().join(", ");
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

/// A Shadow/Highlight with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({
        "shadow_amount": 50, "highlight_amount": 0, "shadow_tonal_width": 50, "shadow_radius": 30,
        "highlight_tonal_width": 50, "highlight_radius": 30, "color_correction": 20
    });
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.shadow_highlight", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning or stay on the processor; those in
/// `whole` are adjustment layers, whose frame the card hands to the processor whole (B-156).
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32], whole: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=14 {
        let file = format!("fx_shauto_{n:03}.json");
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
            largest <= 1 && refused == whole.contains(&n) && agree && (card == 0) == none.contains(&n),
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
        let project = reference(|id| json!([fx(&format!("b444-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Shadow/Highlight {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Shadow/Highlight {what} on three layers, frame {frame}, {}", quality.label()),
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

/// The fixtures' drawing is `holder`, its effect `fx-1`.
fn effect_of(d: &Document) -> Effect {
    let comp = d.project().composition(&Id::new(MAIN)).unwrap();
    comp.layer(&Id::new("holder")).unwrap().effects[0].effect.clone()
}

fn set(effect: Effect) -> Command {
    Command::SetEffectParameters { composition: Id::new(MAIN), layer_id: Id::new("holder"), instance_id: Id::new("fx-1"), effect }
}

fn keys(setting: &str, values: &[(i32, f64)]) -> Command {
    Command::SetEffectKeys {
        composition: Id::new(MAIN),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-1"),
        setting: setting.to_string(),
        keys: values.iter().map(|&(frame, v)| EffectKey { frame, value: vec![v], interp: Interp::Linear }).collect(),
    }
}

#[test]
fn b444_shadow_highlight_auto() {
    let mut t = Table::new(
        "shadow_highlight_auto",
        "# D-564: Shadow/Highlight's Auto Amounts, Temporal Smoothing and Scene Detect\n\nB-444, \
         after After Effects' Shadow/Highlight: with Auto Amounts on, the shadow and highlight \
         amounts come from the picture's own brightness histogram (D-351's statistics; the rule \
         is this program's own and unverified) instead of the sliders; Temporal Smoothing and \
         Scene Detect read the frames either side as Stretch Levels does, and do nothing with \
         Auto Amounts off. Every expected pixel is \
         `Fixtures/shadow_highlight_auto/expected_shadow_highlight_auto.json`, written by \
         `tools/shadow_highlight_auto_reference.py` before this code existed and printed in \
         document 25 as FX-SHAUTO-001 to 014. Tolerance 2e-5. FX-SHAUTO-010 warns \
         TEMPORAL_SMOOTHING_SKIPPED on every frame.\n",
    );

    t.heading("FX-SHAUTO-001 to 014 (document 25)");
    t.fixtures_numbered("expected_shadow_highlight_auto.json", 1..=14);

    t.heading("The file");
    let files: Vec<String> = (1..=14).map(|n| format!("fx_shauto_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let old = t.saved_parameters("fx_shauto_001.json");
    t.row(
        "fx_shauto_001.json, written without the three settings, is saved without them (D-304's way: only if moved, keyed or already in the file)",
        &old.to_string(),
        old.as_object().unwrap().len() == 10 && ["auto_amounts", "temporal_smoothing", "scene_detect"].iter().all(|k| old.get(*k).is_none()),
    );
    let saved = t.saved_parameters("fx_shauto_008.json");
    t.row(
        "fx_shauto_008.json is saved with its thirteen settings, and never the statistics a smoothing added up",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 13
            && saved["auto_amounts"] == "on"
            && saved["temporal_smoothing"] == 0.1
            && saved["scene_detect"] == "on"
            && saved.get("stats").is_none(),
    );
    for (file, want) in [
        ("fx_shauto_011.json", "Shadow/Highlight's auto amounts and scene detect are \"off\" or \"on\", and this is \"yes\"."),
        ("fx_shauto_012.json", "Shadow/Highlight's auto amounts and scene detect are \"off\" or \"on\", and this is \"maybe\"."),
        ("fx_shauto_013.json", "temporal smoothing"),
        ("fx_shauto_014.json", "temporal smoothing"),
    ] {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming it"), &why, why.contains(want));
    }
    let rest = r#""shadow_amount": 50, "highlight_amount": 0, "shadow_tonal_width": 50, "shadow_radius": 30, "highlight_tonal_width": 50, "highlight_radius": 30, "color_correction": 20, "midtone_contrast": 0, "black_clip": 0, "white_clip": 0"#;
    t.shape_refused("fx_shauto_002.json", "a Shadow/Highlight whose auto amounts is a number", &format!(r#"{{{rest}, "auto_amounts": 1, "temporal_smoothing": 0, "scene_detect": "off"}}"#));
    t.shape_refused("fx_shauto_002.json", "a Shadow/Highlight whose temporal smoothing is a word", &format!(r#"{{{rest}, "auto_amounts": "on", "temporal_smoothing": "some", "scene_detect": "off"}}"#));

    t.heading("Commands");
    let mut document = t.load("fx_shauto_002.json").document;
    let base = effect_of(&document);
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = base.clone();
        f(&mut e);
        e
    };
    for (what, command) in [
        ("auto amounts \"yes\"", set(with(&|e| if let Effect::ShadowHighlight { auto_amounts, .. } = e { *auto_amounts = "yes".into() }))),
        ("scene detect \"maybe\"", set(with(&|e| if let Effect::ShadowHighlight { scene_detect, .. } = e { *scene_detect = "maybe".into() }))),
        ("temporal smoothing 11", set(with(&|e| if let Effect::ShadowHighlight { temporal_smoothing, .. } = e { *temporal_smoothing = 11.0 }))),
        ("temporal smoothing keyed to -1", keys("temporal_smoothing", &[(0, 0.0), (4, -1.0)])),
    ] {
        let refused = document.apply(command).err();
        let untouched = effect_of(&document) == base;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && untouched,
        );
    }
    t.taken(
        &mut document,
        "fx_shauto_002.json",
        vec![
            ("auto amounts on, temporal smoothing 0.1, scene detect on", set(with(&|e| if let Effect::ShadowHighlight { auto_amounts, temporal_smoothing, scene_detect, .. } = e {
                *auto_amounts = "on".into();
                *temporal_smoothing = 0.1;
                *scene_detect = "on".into();
            }))),
            ("temporal smoothing keyed from 0 to 0.2", keys("temporal_smoothing", &[(0, 0.0), (4, 0.2)])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_shauto_003.json", 0),
        ("fx_shauto_003.json", 4),
        ("fx_shauto_005.json", 4),
        ("fx_shauto_007.json", 3),
        ("fx_shauto_008.json", 4),
        ("fx_shauto_010.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Auto Amounts on keeps the effect on the processor (003 to 005, 007 to 010); refused files
    // (011 to 014) leave nothing for the card.
    let none: Vec<u32> = (3..=5).chain(7..=14).collect();
    // 009 and 010 put it on an adjustment layer, so the card hands those frames over whole.
    card_fixtures(&mut t, &mut gpu, &none, &[9, 10]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added with temporal smoothing 0.1 and scene detect, Auto Amounts off, so on the card", json!({"temporal_smoothing": 0.1, "scene_detect": "on"}), 3),
            ("with Auto Amounts on, on the processor", json!({"auto_amounts": "on"}), 0),
            ("with Auto Amounts on and temporal smoothing 0.1, on the processor", json!({"auto_amounts": "on", "temporal_smoothing": 0.1}), 0),
        ],
    );

    t.heading("Pictures: in `verification/D-564 pictures/`");
    let dir = repo("verification/D-564 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    let luma = |p: &[u8]| 0.2126 * p[0] as f64 + 0.7152 * p[1] as f64 + 0.0722 * p[2] as f64;
    let mean = |v: &[u8]| {
        let l: Vec<f64> = v.chunks_exact(4).filter(|p| p[3] > 0).map(luma).collect();
        l.iter().sum::<f64>() / l.len() as f64
    };
    write("town.png", TOWN, &town());
    // The street at half its brightness, a picture lit from behind.
    let dark: Vec<u8> = town().chunks_exact(4).flat_map(|p| [p[0] / 2, p[1] / 2, p[2] / 2, 255]).collect();
    write("dark_town.png", TOWN, &dark);
    let street = |asset: &str, p: J| picture(&dir, asset, TOWN, json!([fx("fx-0-0", &p)]));
    let (before, s0) = picture(&dir, "town.png", TOWN, json!([]));
    let (added, s1) = street("town.png", json!({}));
    let (auto, s2) = street("town.png", json!({"auto_amounts": "on", "shadow_amount": 90}));
    write("1_before.png", TOWN, &before);
    write("2_as_added.png", TOWN, &added);
    write("3_auto.png", TOWN, &auto);
    t.row(
        "1_before.png, 2_as_added.png and 3_auto.png: the street, Shadow/Highlight as added (the sliders' shadow 50), and with Auto Amounts on (the slider at 90 not used): the auto lift is its own; all draw cleanly",
        &format!("{s0:?} {s1:?} {s2:?}; mean brightness {:.2}, {:.2}, {:.2}", mean(&before), mean(&added), mean(&auto)),
        s0.is_empty() && s1.is_empty() && s2.is_empty() && auto != added && auto != before,
    );
    let (dark_before, s3) = picture(&dir, "dark_town.png", TOWN, json!([]));
    let (dark_auto, s4) = street("dark_town.png", json!({"auto_amounts": "on"}));
    write("4_dark_before.png", TOWN, &dark_before);
    write("5_dark_auto.png", TOWN, &dark_auto);
    let (lift, dark_lift) = (mean(&auto) - mean(&before), mean(&dark_auto) - mean(&dark_before));
    t.row(
        "4_dark_before.png and 5_dark_auto.png: the street at half its brightness, and with Auto Amounts on: the darker picture is lifted more than the street was (whose auto highlight amount pulls its lightest parts down a little); both draw cleanly",
        &format!("{s3:?} {s4:?}; mean brightness raised {dark_lift:.2} on the dark street against {lift:.2} on the street"),
        s3.is_empty() && s4.is_empty() && dark_lift > lift && dark_lift > 0.0,
    );
    fs::remove_file(dir.join("town.png")).unwrap();
    fs::remove_file(dir.join("dark_town.png")).unwrap();

    t.finish("D-564_shadow_highlight_auto_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B444_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-444: a measurement, run deliberately with --release --ignored"]
fn b444_shadow_highlight_auto_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B444_CPU").is_ok();
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
        ("Noise, then Shadow/Highlight as added", Some(json!({}))),
        ("Noise, then Shadow/Highlight with Auto Amounts on (on the processor)", Some(json!({"auto_amounts": "on"}))),
        ("Noise, then Shadow/Highlight with Auto Amounts on and temporal smoothing 0.1 (on the processor)", Some(json!({"auto_amounts": "on", "temporal_smoothing": 0.1}))),
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
    let out = std::env::var("B444_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-444_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
