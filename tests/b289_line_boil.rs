//! B-289: D-410, Line Boil (PLUGINS.md pick #4): `core.turbulent_displace` takes a new seed
//! every N frames (`new_seed_every`), so a warp holds for N frames and jumps to another, as
//! hand-drawn lines redrawn on twos or threes do. A file without the setting keeps one seed.
//!
//! Writes `verification/D-410_line_boil_table.md` and draws pictures into
//! `verification/D-410 pictures/`.
//!
//! Every expected pixel is `Fixtures/line_boil/expected_line_boil.json`, written by
//! `tools/line_boil_reference.py` before this code existed and printed in document 25 as
//! FX-BOIL-001 to 013. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, repo, same_json, saved, set, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
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

fn is_turb(c: &render::OnCard) -> bool {
    matches!(c, render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::TurbulentDisplace { .. }))
}

/// The Turbulent Displaces the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers.iter().flat_map(|l| &l.on_card).filter(|c| is_turb(c.unmixed())).count()
        + plan.layers
            .iter()
            .filter_map(|l| l.adjust.as_ref())
            .filter_map(|stack| compose::adjust_run(stack, (plan.width, plan.height)))
            .flatten()
            .filter(|c| is_turb(c.unmixed()))
            .count()
}

/// The processor and the card, each drawing the frame the page receives.
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

/// A Turbulent Displace as added from the panel, with `p` laid over it.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"amount": 30, "size": 40, "complexity": 2, "evolution": 0, "speed": 0, "seed": 0,
        "edges": "transparent", "units": "after_effects", "new_seed_every": 2});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.turbulent_displace", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=13 {
        let file = format!("fx_boil_{n:03}.json");
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

/// The reference shot with each setting on its first three layers, on the card against the
/// processor, all three on the card each frame.
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J)]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project, frame: i32| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, frame, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    for (what, p) in settings {
        let project = reference(|id| json!([fx(&format!("b289-{id}"), p)]));
        let one = reference(|id| json!([fx(&format!("b289-{id}"), &{
            let mut q = p.clone();
            q["new_seed_every"] = J::from(0);
            q
        })]));
        // Frame 102 is past a jump for both settings below: 2 and 3 each step between 100 and 102.
        let jumped = distance(&cpu(&project, 102), &cpu(&one, 102)).1;
        t.row(
            &format!("the reference shot, Turbulent Displace {what}: the processor's frame 102 differs from the same warp with one seed, so the comparisons below test the new seed"),
            &format!("{jumped} pixels changed"),
            jumped > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 1, 100, 102, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Turbulent Displace {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

/// `effects` on the street at `frame`, drawn on the processor, straight 8-bit, with what it
/// warned of.
fn picture(dir: &Path, effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
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
    let out = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (out.to_srgb8_straight(), said)
}

/// The `art` layer's effect in `file`, changed by `f`.
fn changed(t: &Table, file: &str, f: impl FnOnce(&mut Effect)) -> Effect {
    let d = t.load(file).document;
    let mut e = d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    f(&mut e);
    e
}

/// `json` opened and drawn at `frame` on the processor, from the folder `root`.
fn draw(json: &J, root: &Path, frame: i32) -> Vec<f32> {
    let loaded = persist::load_str(&json.to_string()).expect("the file reads");
    let mut log = FrameLog::new(3);
    render_frame(loaded.document.project(), &Id::new(MAIN), frame, root, 64, &mut log).expect("the frame draws").data().to_vec()
}

#[test]
fn b289_line_boil() {
    let mut t = Table::new(
        "line_boil",
        "# D-410: Line Boil\n\nB-289: PLUGINS.md's pick #4, merged into `core.turbulent_displace` \
         as New Seed Every (`new_seed_every`, 0 to 100 frames, keyable, its whole part counted). \
         With a whole part N of 1 or more the seed for frame f is the seed plus floor(f / N), \
         P0-23's held step, so the warp holds for N frames and jumps to another. 0, what a file \
         without it means, keeps one seed. Every expected pixel is \
         `Fixtures/line_boil/expected_line_boil.json`, written by `tools/line_boil_reference.py` \
         before this code existed and printed in document 25 as FX-BOIL-001 to 013. Tolerance \
         2e-5.\n",
    );

    t.heading("FX-BOIL-001 to 013 (document 25)");
    t.fixtures_numbered("expected_line_boil.json", 1..=13);

    t.heading("Older projects: one seed, as before");
    for (folder, prefix, count) in [("turbulent_displace", "fx_turb", 26), ("turbulent_ae", "fx_turb_ae", 12)] {
        let root = repo(&format!("Fixtures/{folder}"));
        for n in 1..=count {
            let file = format!("{prefix}_{n:03}.json");
            let text = fs::read_to_string(root.join(&file)).unwrap();
            let Ok(loaded) = persist::load_str(&text) else { continue };
            let old: J = serde_json::from_str(&text).unwrap();
            let kept = same_json(&saved(&loaded), &old);
            // Its twin with New Seed Every 0 written draws the same frames, bit for bit.
            let mut twin = old.clone();
            let effects = &mut twin["compositions"][0]["layers"][0]["effects"];
            let is_turb = effects[0]["type_id"] == "core.turbulent_displace";
            if is_turb {
                effects[0]["parameters"]["new_seed_every"] = J::from(0);
            }
            let same = is_turb && [0, 4].iter().all(|&f| draw(&old, &root, f) == draw(&twin, &root, f));
            t.row(
                &format!("{folder}/{file} (no new_seed_every) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with New Seed Every 0"),
                &format!("saved {}; {}", if kept { "the same" } else { "differently" }, if same { "bit-identical" } else { "differ" }),
                kept && same,
            );
        }
    }

    t.heading("The file");
    let files: Vec<String> = (1..=13).map(|n| format!("fx_boil_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved_p = t.saved_parameters("fx_boil_004.json");
    t.row(
        "fx_boil_004.json is saved with New Seed Every 3",
        &saved_p.to_string(),
        saved_p["new_seed_every"] == 3.0,
    );
    for (file, what) in [("fx_boil_011.json", "-1"), ("fx_boil_012.json", "101")] {
        let why = changed(&t, file, |_| {}).why_invalid();
        let says = why.contains("new_seed_every") || why.contains("new seed every");
        t.row(&format!("{file} (New Seed Every {what}) is refused in a sentence naming new_seed_every"), &why, says);
    }
    t.shape_refused(
        "fx_boil_001.json",
        "a Turbulent Displace whose New Seed Every is a word",
        r##"{"amount": 30, "size": 8, "complexity": 2, "evolution": 0, "speed": 0, "seed": 0, "edges": "transparent", "units": "after_effects", "new_seed_every": "twos"}"##,
    );

    t.heading("How far it reaches");
    let mut draft = changed(&t, "fx_boil_001.json", |_| {});
    draft.scale_distances(|d| d * 0.5);
    let kept = matches!(draft, Effect::TurbulentDisplace { new_seed_every, .. } if new_seed_every == 2.0);
    t.row("a half-size draft preview leaves New Seed Every, a count of frames, as it was", &format!("{draft:?}"), kept);

    t.heading("Commands");
    let mut document = t.load("fx_boil_001.json").document;
    let high = changed(&t, "fx_boil_001.json", |e| if let Effect::TurbulentDisplace { new_seed_every, .. } = e { *new_seed_every = 101.0 });
    let low = changed(&t, "fx_boil_001.json", |e| if let Effect::TurbulentDisplace { new_seed_every, .. } = e { *new_seed_every = -1.0 });
    t.refused(
        &mut document,
        vec![
            ("new seed every 101", set(high)),
            ("new seed every -1", set(low)),
            ("new seed every keyed to 150", keys("new_seed_every", &[(0, &[2.0]), (4, &[150.0])])),
        ],
    );
    let threes = changed(&t, "fx_boil_001.json", |e| if let Effect::TurbulentDisplace { new_seed_every, .. } = e { *new_seed_every = 3.0 });
    t.taken(
        &mut document,
        "fx_boil_001.json",
        vec![
            ("new seed every 3", set(threes)),
            ("new seed every keyed from 1 to 4", keys("new_seed_every", &[(0, &[1.0]), (4, &[4.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_boil_001.json", 0),
        ("fx_boil_001.json", 2),
        ("fx_boil_004.json", 3),
        ("fx_boil_006.json", 3),
        ("fx_boil_007.json", 2),
        ("fx_boil_009.json", 2),
        ("fx_boil_010.json", 2),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, &[11, 12, 13]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("amount 30, size 40, new seed every 2", json!({})),
            ("amount 60, size 20, speed 20, seed 7, new seed every 3", json!({"amount": 60, "size": 20, "speed": 20, "seed": 7, "new_seed_every": 3})),
        ],
    );

    t.heading("Pictures: in `verification/D-410 pictures/`");
    let dir = repo("verification/D-410 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (before, said) = picture(&dir, json!([]), 0);
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());

    // A light boil: amount 6, size 30, speed 0, a new seed every 2 frames.
    let boil = json!([fx("fx-0-0", &json!({"amount": 6, "size": 30}))]);
    let frames: Vec<(Vec<u8>, Vec<String>)> = (0..5).map(|f| picture(&dir, boil.clone(), f)).collect();
    for (f, (p, _)) in frames.iter().enumerate() {
        write(&format!("{}_boil_frame_{f}.png", f + 2), p);
    }
    let clean = frames.iter().all(|(_, s)| s.is_empty());
    let d = |a: usize, b: usize| distance(&frames[a].0, &frames[b].0).1;
    t.row(
        "2_boil_frame_0.png to 6_boil_frame_4.png, amount 6, size 30, a new seed every 2: frames 0 and 1 the same, frames 2 and 3 the same and a different wobble, frame 4 another; each warped from the street; all draw cleanly",
        &format!("pixels differing: 0 to 1 {}, 1 to 2 {}, 2 to 3 {}, 3 to 4 {}; frame 0 against the street {}", d(0, 1), d(1, 2), d(2, 3), d(3, 4), distance(&frames[0].0, &before).1),
        clean && d(0, 1) == 0 && d(2, 3) == 0 && d(1, 2) > 0 && d(3, 4) > 0 && distance(&frames[0].0, &before).1 > 0,
    );
    let one = json!([fx("fx-0-0", &json!({"amount": 6, "size": 30, "new_seed_every": 0}))]);
    let (held, said) = picture(&dir, one, 4);
    write("7_one_seed_frame_4.png", &held);
    t.row(
        "7_one_seed_frame_4.png, the same with New Seed Every 0, frame 4: the warp of frame 0, never jumping; draws cleanly",
        &format!("{said:?}; {} pixels differ from 2_boil_frame_0.png", distance(&held, &frames[0].0).1),
        said.is_empty() && distance(&held, &frames[0].0).1 == 0,
    );

    t.finish("D-410_line_boil_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, then the effect, every eighth frame
/// asked for as the viewer asks, whole. With `B289_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-289: a measurement, run deliberately with --release --ignored"]
fn b289_line_boil_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B289_CPU").is_ok();
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
        ("Noise, then Turbulent Displace amount 30, size 40, one seed", Some(json!({"new_seed_every": 0}))),
        ("Noise, then Turbulent Displace amount 30, size 40, a new seed every 2", Some(json!({}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}t"), p));
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
    let out = std::env::var("B289_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-289_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
