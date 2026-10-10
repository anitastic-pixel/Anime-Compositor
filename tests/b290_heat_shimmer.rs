//! B-290: D-411, Heat Shimmer (PLUGINS.md pick #17): `core.turbulent_displace` takes a drift,
//! a direction and a speed in pixels a frame, so the ripple travels, as heat rising through the
//! air does. A file without the settings does not drift.
//!
//! Writes `verification/D-411_heat_shimmer_table.md` and draws pictures into
//! `verification/D-411 pictures/`.
//!
//! Every expected pixel is `Fixtures/heat_shimmer/expected_heat_shimmer.json`, written by
//! `tools/heat_shimmer_reference.py` before this code existed and printed in document 25 as
//! FX-SHIMMER-001 to 016. Tolerance 2e-5. Nothing here is a snapshot of a run.

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
        "edges": "transparent", "units": "after_effects", "drift_direction": 0, "drift_speed": 4});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.turbulent_displace", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=16 {
        let file = format!("fx_shimmer_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b290-{id}"), p)]));
        let one = reference(|id| json!([fx(&format!("b290-{id}"), &{
            let mut q = p.clone();
            q["drift_speed"] = J::from(0);
            q
        })]));
        let drifted = distance(&cpu(&project, 102), &cpu(&one, 102)).1;
        t.row(
            &format!("the reference shot, Turbulent Displace {what}: the processor's frame 102 differs from the same warp without the drift, so the comparisons below test the drift"),
            &format!("{drifted} pixels changed"),
            drifted > 0,
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
fn b290_heat_shimmer() {
    let mut t = Table::new(
        "heat_shimmer",
        "# D-411: Heat Shimmer\n\nB-290: PLUGINS.md's pick #17, merged into `core.turbulent_displace` \
         as a drift (\"if anything\", PLUGINS.md says): Drift Direction (`drift_direction`, -3600 to \
         3600 degrees, 0 up, 90 right) and Drift Speed (`drift_speed`, 0 to 1000 pixels a frame), both \
         keyable. On frame f the field's point is D-127's moved back by v f (sin a, -cos a), so the \
         ripple slides that way. Speed 0, what a file without them means, is D-127's and D-328's rule. \
         Every expected pixel is `Fixtures/heat_shimmer/expected_heat_shimmer.json`, written by \
         `tools/heat_shimmer_reference.py` before this code existed and printed in document 25 as \
         FX-SHIMMER-001 to 016. Tolerance 2e-5.\n",
    );

    t.heading("FX-SHIMMER-001 to 016 (document 25)");
    t.fixtures_numbered("expected_heat_shimmer.json", 1..=16);

    t.heading("Older projects: no drift, as before");
    for (folder, prefix, count) in [("turbulent_displace", "fx_turb", 26), ("turbulent_ae", "fx_turb_ae", 12), ("line_boil", "fx_boil", 13)] {
        let root = repo(&format!("Fixtures/{folder}"));
        for n in 1..=count {
            let file = format!("{prefix}_{n:03}.json");
            let text = fs::read_to_string(root.join(&file)).unwrap();
            let Ok(loaded) = persist::load_str(&text) else { continue };
            let old: J = serde_json::from_str(&text).unwrap();
            let kept = same_json(&saved(&loaded), &old);
            // Its twin with Drift Direction 90 and Drift Speed 0 written draws the same frames.
            let mut twin = old.clone();
            let effects = &mut twin["compositions"][0]["layers"][0]["effects"];
            let is_turb = effects[0]["type_id"] == "core.turbulent_displace";
            if is_turb {
                effects[0]["parameters"]["drift_direction"] = J::from(90);
                effects[0]["parameters"]["drift_speed"] = J::from(0);
            }
            let same = is_turb && [0, 4].iter().all(|&f| draw(&old, &root, f) == draw(&twin, &root, f));
            t.row(
                &format!("{folder}/{file} (no drift) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Drift Direction 90 and Drift Speed 0"),
                &format!("saved {}; {}", if kept { "the same" } else { "differently" }, if same { "bit-identical" } else { "differ" }),
                kept && same,
            );
        }
    }

    t.heading("The file");
    let files: Vec<String> = (1..=16).map(|n| format!("fx_shimmer_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved_p = t.saved_parameters("fx_shimmer_003.json");
    t.row(
        "fx_shimmer_003.json is saved with Drift Direction 90 and Drift Speed 2",
        &saved_p.to_string(),
        saved_p["drift_direction"] == 90.0 && saved_p["drift_speed"] == 2.0,
    );
    for (file, what, want) in [
        ("fx_shimmer_013.json", "Drift Speed -1", "drift speed"),
        ("fx_shimmer_014.json", "Drift Speed 1001", "drift speed"),
        ("fx_shimmer_015.json", "Drift Direction 3601", "drift direction"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        let says = why.contains(want) || why.contains(&want.replace(' ', "_"));
        t.row(&format!("{file} ({what}) is refused in a sentence naming {want}"), &why, says);
    }
    t.shape_refused(
        "fx_shimmer_001.json",
        "a Turbulent Displace whose Drift Speed is a word",
        r##"{"amount": 30, "size": 8, "complexity": 2, "evolution": 0, "speed": 0, "seed": 0, "edges": "transparent", "units": "after_effects", "drift_direction": 0, "drift_speed": "fast"}"##,
    );

    t.heading("How far it reaches");
    let mut draft = changed(&t, "fx_shimmer_003.json", |_| {});
    draft.scale_distances(|d| d * 0.5);
    let halved = matches!(draft, Effect::TurbulentDisplace { drift_speed, drift_direction, .. } if drift_speed == 1.0 && drift_direction == 90.0);
    t.row("a half-size draft preview halves Drift Speed, a distance a frame, and leaves its direction", &format!("{draft:?}"), halved);
    let base = changed(&t, "fx_shimmer_003.json", |_| {});
    t.row(
        "the drift leaves the drawing's bounds as they were",
        &format!("{} against {} without the drift", base.bounds_expansion(), changed(&t, "fx_shimmer_002.json", |_| {}).bounds_expansion()),
        base.bounds_expansion() == changed(&t, "fx_shimmer_002.json", |_| {}).bounds_expansion(),
    );

    t.heading("Commands");
    let mut document = t.load("fx_shimmer_001.json").document;
    let fast = changed(&t, "fx_shimmer_001.json", |e| if let Effect::TurbulentDisplace { drift_speed, .. } = e { *drift_speed = 1001.0 });
    let turned = changed(&t, "fx_shimmer_001.json", |e| if let Effect::TurbulentDisplace { drift_direction, .. } = e { *drift_direction = 3601.0 });
    t.refused(
        &mut document,
        vec![
            ("drift speed 1001", set(fast)),
            ("drift direction 3601", set(turned)),
            ("drift speed keyed to 2000", keys("drift_speed", &[(0, &[1.0]), (4, &[2000.0])])),
        ],
    );
    let right = changed(&t, "fx_shimmer_001.json", |e| {
        if let Effect::TurbulentDisplace { drift_direction, drift_speed, .. } = e {
            *drift_direction = 90.0;
            *drift_speed = 2.0;
        }
    });
    t.taken(
        &mut document,
        "fx_shimmer_001.json",
        vec![
            ("drift right at 2 pixels a frame", set(right)),
            ("drift speed keyed from 0 to 4", keys("drift_speed", &[(0, &[0.0]), (4, &[4.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_shimmer_001.json", 3),
        ("fx_shimmer_004.json", 1),
        ("fx_shimmer_005.json", 2),
        ("fx_shimmer_006.json", 3),
        ("fx_shimmer_007.json", 3),
        ("fx_shimmer_010.json", 2),
        ("fx_shimmer_011.json", 2),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, &[13, 14, 15, 16]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("amount 30, size 40, drifting up at 4 pixels a frame", json!({})),
            ("amount 60, size 20, speed 20, seed 7, drifting at 30 degrees at 2.5 pixels a frame, a new seed every 3", json!({"amount": 60, "size": 20, "speed": 20, "seed": 7, "drift_direction": 30, "drift_speed": 2.5, "new_seed_every": 3})),
        ],
    );

    t.heading("Pictures: in `verification/D-411 pictures/`");
    let dir = repo("verification/D-411 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (before, said) = picture(&dir, json!([]), 0);
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());

    // A heat shimmer: amount 8, size 20, the ripple rising 6 pixels a frame (the street runs frames 0 to 4), held still otherwise.
    let rise = |speed: f64| json!([fx("fx-0-0", &json!({"amount": 8, "size": 20, "drift_speed": speed}))]);
    let frames: Vec<(Vec<u8>, Vec<String>)> = [0, 2, 4].iter().map(|&f| picture(&dir, rise(6.0), f)).collect();
    for ((p, _), (n, f)) in frames.iter().zip([(2, 0), (3, 2), (4, 4)]) {
        write(&format!("{n}_rising_frame_{f}.png"), p);
    }
    let still: Vec<Vec<u8>> = [0, 4].iter().map(|&f| picture(&dir, rise(0.0), f).0).collect();
    let clean = frames.iter().all(|(_, s)| s.is_empty());
    t.row(
        "2_rising_frame_0.png, 3_rising_frame_2.png and 4_rising_frame_4.png, amount 8, size 20, drifting up at 6 pixels a frame: frame 0 the warp without drift, frames 2 and 4 the ripple moved up 12 and 24 pixels, each different; without the drift frame 4 is frame 0; all draw cleanly",
        &format!("pixels differing: frame 0 from no drift {}, 0 to 2 {}, 2 to 4 {}; without drift 0 to 4 {}", distance(&frames[0].0, &still[0]).1, distance(&frames[0].0, &frames[1].0).1, distance(&frames[1].0, &frames[2].0).1, distance(&still[0], &still[1]).1),
        clean && distance(&frames[0].0, &still[0]).1 == 0 && distance(&frames[0].0, &frames[1].0).1 > 0 && distance(&frames[1].0, &frames[2].0).1 > 0 && distance(&still[0], &still[1]).1 == 0,
    );
    let shimmer = json!([fx("fx-0-0", &json!({"amount": 8, "size": 20, "speed": 20, "drift_speed": 6}))]);
    let (p, said) = picture(&dir, shimmer, 4);
    write("5_shimmer_speed_20_frame_4.png", &p);
    t.row(
        "5_shimmer_speed_20_frame_4.png, the same with speed 20, frame 4: the rising ripple also changes as it goes; draws cleanly",
        &format!("{said:?}; {} pixels differ from 4_rising_frame_4.png", distance(&p, &frames[2].0).1),
        said.is_empty() && distance(&p, &frames[2].0).1 > 0,
    );

    t.finish("D-411_heat_shimmer_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, then the effect, every eighth frame
/// asked for as the viewer asks, whole. With `B290_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-290: a measurement, run deliberately with --release --ignored"]
fn b290_heat_shimmer_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B290_CPU").is_ok();
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
        ("Noise, then Turbulent Displace amount 30, size 40, no drift", Some(json!({"drift_speed": 0}))),
        ("Noise, then Turbulent Displace amount 30, size 40, drifting up at 4 pixels a frame", Some(json!({}))),
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
    let out = std::env::var("B290_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-290_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
