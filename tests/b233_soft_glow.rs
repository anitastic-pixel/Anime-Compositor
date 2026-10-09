//! B-233: D-353, the shared soft-glow engine (EFFECTS.md P0-21) and pick #1, Soft Physical Glow,
//! Glow's "physical" falloff.
//!
//! Every expected pixel is `Fixtures/soft_glow/expected_soft_glow.json`, written by
//! `tools/soft_glow_reference.py` before this code existed. The glow is drawn on the card too,
//! and checked against the processor within 1 level of 255 (ADR-006, D-100).

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, set, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::Document;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

fn effect_of(d: &Document) -> Effect {
    let comp = d.project().composition(&Id::new(MAIN)).unwrap();
    comp.layer(&Id::new("art")).unwrap().effects[0].effect.clone()
}

fn distance(a: &[u8], b: &[u8]) -> (u8, usize) {
    assert_eq!(a.len(), b.len(), "the two pictures are different sizes");
    let (mut largest, mut pixels) = (0, 0);
    for (p, q) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
        // A pixel clear in both has no colour to compare: an unmult glow's faint edge, a hair
        // over nothing, divides its colour by a covering that rounds to 0, a different colour
        // each way and invisible either way.
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

/// A Soft Physical Glow's parameters: the effect's defaults with `changes` laid over them.
fn physical(changes: J) -> J {
    let mut p = json!({"falloff": "physical", "threshold_mode": "chroma", "threshold": 0, "threshold_smooth": 0,
        "saturation_bias": 0, "radius": 500, "exposure": 1, "aspect_ratio": 1, "aspect_angle": 0,
        "operation": "screen", "source_opacity": 100, "unmult": "on"});
    for (k, v) in changes.as_object().unwrap() {
        p[k] = v.clone();
    }
    p
}

/// The reference shot (1920 by 1080) with a Soft Physical Glow of `parameters` on its first three
/// layers, as B-223 has it.
fn reference(parameters: &J) -> Project {
    let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        layers[i]["effects"] = json!([{"instance_id": format!("b233-{id}"), "type_id": "core.glow", "enabled": true, "parameters": parameters}]);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

/// The Soft Physical Glows the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c, render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::SoftGlow { .. })))
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

/// The town (480 by 270) in a composition 160 pixels wider and taller, set in its middle, with
/// `effects`, so a glow can be seen spreading past the layer's own edge.
fn framed(dir: &Path, effects: J) -> Project {
    let (w, h) = TOWN;
    let t = |v: J| json!({"base": v, "keyframes": []});
    let project = json!({
        "schema_version": 0, "project_id": "proj-b233",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-town", "kind": "still", "name": "town", "path": "town.png",
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
            "id": MAIN, "name": "Main", "width": w + 160, "height": h + 160, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 8,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 8},
            "layer_order": ["art"],
            "layers": [{"id": "art", "kind": "raster", "name": "art", "asset_id": "asset-town", "enabled": true,
                "locked": false, "in_frame": 0, "out_frame": 8, "source_offset_frames": 0,
                "transform": {"anchor": t(json!([0, 0])), "position": t(json!([80, 80])), "scale": t(json!([100, 100])),
                              "rotation": t(json!(0)), "opacity": t(json!(1))},
                "exposure_spans": [], "mask": null, "matte": null, "blend_mode": "normal", "effects": effects}]
        }]
    });
    let _ = dir;
    persist::load_str(&project.to_string()).unwrap_or_else(|d| panic!("the framed town opens: {}", d.message)).document.project().clone()
}

#[test]
fn b233_soft_glow() {
    let mut t = Table::new(
        "soft_glow",
        "# B-233: the shared soft-glow engine and Soft Physical Glow\n\n\
         D-353 (EFFECTS.md P0-21 and pick #1): Glow's new \"physical\" falloff. A soft threshold \
         picks the light (Threshold Mode, Smooth, Saturation Bias), several blur sizes a doubling \
         apart are added in linear light so the glow has a bright core and a long, soft tail, \
         Exposure brightens it, and the untouched layer goes back on top. A Glow without the \
         falloff, or with \"classic\", is the Glow it always was. Every expected pixel is \
         `Fixtures/soft_glow/expected_soft_glow.json`, written by `tools/soft_glow_reference.py` \
         before this code existed and printed in document 25 as FX-SGLOW-001 to 045. The build's \
         frame is compared sample by sample; the answer is the largest difference over all of \
         them, against the catalogue's tolerance of 2e-5. The glow is drawn on the card too: the \
         card's picture against the processor's, within 1 level of 255 (ADR-006, D-100).\n",
    );

    t.heading("FX-SGLOW-001 to 045 (document 25)");
    t.fixtures("expected_soft_glow.json");

    t.heading("The file");
    let files: Vec<String> = (1..=45).map(|n| format!("fx_sglow_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let params = t.saved_parameters("fx_sglow_001.json");
    t.row(
        "fx_sglow_001.json: a Soft Physical Glow writes its falloff, \"physical\", and none of the classic Glow's settings",
        &params.to_string(),
        params["falloff"] == "physical" && params.get("based_on").is_none() && params.get("intensity").is_none(),
    );
    t.shape_refused("fx_sglow_001.json", "Soft Physical Glow with no radius", &physical(json!({})).as_object().map(|o| {
        let mut o = o.clone();
        o.remove("radius");
        J::Object(o).to_string()
    }).unwrap());
    t.shape_refused("fx_sglow_001.json", "an exposure written as a word", &physical(json!({"exposure": "lots"})).to_string());
    t.shape_refused("fx_sglow_001.json", "unmult written as a number", &physical(json!({"unmult": 1})).to_string());
    for (file, said) in [
        ("fx_sglow_033.json", "Glow's falloff is \"classic\" or \"physical\", and this is \"gaussian\"."),
        ("fx_sglow_034.json", "Soft Physical Glow's radius runs from 0 to 2000, and this is 2001."),
        ("fx_sglow_035.json", "Soft Physical Glow's radius runs from 0 to 2000, and this is -1."),
        ("fx_sglow_037.json", "Soft Physical Glow's saturation bias runs from -100 to 100, and this is 101."),
        ("fx_sglow_038.json", "Soft Physical Glow's threshold smooth runs from 0 to 100, and this is 101."),
        ("fx_sglow_039.json", "Soft Physical Glow's threshold runs from 0 to 100, and this is -1."),
        ("fx_sglow_040.json", "Soft Physical Glow's aspect ratio runs from 0 to 2, and this is 2.1."),
        ("fx_sglow_041.json", "Soft Physical Glow's exposure runs from 0 to 100, and this is -1."),
        ("fx_sglow_042.json", "Soft Physical Glow's source opacity runs from 0 to 100, and this is 101."),
        ("fx_sglow_043.json", "Soft Physical Glow's threshold mode is \"chroma\" or \"luminance\", and this is \"rgb\"."),
        ("fx_sglow_044.json", "Soft Physical Glow's blend mode is \"add\" or \"screen\", and this is \"multiply\"."),
        ("fx_sglow_045.json", "Soft Physical Glow's unmult is \"off\" or \"on\", and this is \"yes\"."),
    ] {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming it"), &why, why == said);
    }

    t.heading("Commands");
    let mut document = t.load("fx_sglow_001.json").document;
    let base = effect_of(&document);
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = base.clone();
        f(&mut e);
        e
    };
    t.refused(
        &mut document,
        vec![
            ("radius 2001", set(with(&|e| if let Effect::SoftGlow { radius, .. } = e { *radius = 2001.0 }))),
            ("threshold mode \"rgb\"", set(with(&|e| if let Effect::SoftGlow { threshold_mode, .. } = e { *threshold_mode = "rgb".into() }))),
        ],
    );
    t.taken(
        &mut document,
        "fx_sglow_001.json",
        vec![
            ("threshold 30 in luminance, exposure 2", set(with(&|e| if let Effect::SoftGlow { threshold, threshold_mode, exposure, .. } = e {
                *threshold = 30.0;
                *threshold_mode = "luminance".into();
                *exposure = 2.0;
            }))),
            ("radius keyed 0 to 100 over frames 0 to 4", keys("radius", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_sglow_001.json", 0),
        ("fx_sglow_006.json", 0),
        ("fx_sglow_015.json", 0),
        ("fx_sglow_022.json", 0),
        ("fx_sglow_027.json", 0),
        ("fx_sglow_032.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    let fixture_root = effect_table::repo("Fixtures/soft_glow");
    for n in 1..=32 {
        let file = format!("fx_sglow_{n:03}.json");
        let project = persist::load(&fixture_root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(&mut gpu, &project, &comp, &fixture_root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &fixture_root, frame, quality);
            }
        }
        // FX-SGLOW-007, 008, 010 to 012 and 024 have a threshold with no smooth: a hard step,
        // left to the processor (D-122's reason).
        let hard = [7, 8, 10, 11, 12, 24, 30].contains(&n);
        let expect = if hard { 0 } else { 10 };
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames (expected {expect}); the same warnings: {agree}"),
            largest <= 1 && !refused && agree && card == expect,
        );
    }
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = effect_table::repo("Fixtures/reference_shot");
    for (what, p) in [
        ("as added (radius 500)", physical(json!({}))),
        ("radius 60, threshold 40 with smooth 50, luminance, saturation bias 30, exposure 1.5", physical(json!({"radius": 60, "threshold": 40, "threshold_smooth": 50, "threshold_mode": "luminance", "saturation_bias": 30, "exposure": 1.5}))),
        ("radius 150, aspect 1.6 at 25 degrees, add, source opacity 60, unmult off", physical(json!({"radius": 150, "aspect_ratio": 1.6, "aspect_angle": 25, "operation": "add", "source_opacity": 60, "unmult": "off"}))),
    ] {
        let project = reference(&p);
        let cpu = |project: &Project| {
            let mut log = FrameLog::new(3);
            preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
                .expect("the reference shot draws")
                .to_srgb8_straight()
        };
        let plain = persist::load(&effect_table::repo("verification/B-08a_project.json")).expect("the reference shot").document.project().clone();
        let changed = distance(&cpu(&project), &cpu(&plain)).1;
        t.row(
            &format!("the reference shot, Soft Physical Glow {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(&mut gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Soft Physical Glow {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
    let hard = reference(&physical(json!({"radius": 60, "threshold": 50})));
    let card = on_card(&hard, &ref_comp, &ref_root, 100, PreviewQuality::Full);
    t.row(
        "a threshold with smooth 0 is a hard step, so it is left to the processor (D-122's reason)",
        &format!("{card} of 3 on the card"),
        card == 0,
    );

    t.heading("Pictures: the town, set in a larger frame, in `verification/D-353 pictures/`");
    let dir = effect_table::repo("verification/D-353 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    png_out::write_rgba(&dir.join("town.png"), w, h, OutputDepth::Eight, &[], &town()).unwrap();
    let (fw, fh) = (w + 160, h + 160);
    let draw = |effects: J| {
        let mut log = FrameLog::new(8);
        let b = render_frame(&framed(&dir, effects.clone()), &Id::new(MAIN), 0, &dir, 64, &mut log).expect("the town draws");
        (b.to_srgb8_straight(), log.finish().iter().map(|d| d.id.as_str().to_string()).collect::<Vec<_>>())
    };
    // How far the glow carries past the layer: the brightest pixel 40 pixels out from the
    // town's left edge (the town starts at column 80), and in the frame's far corner.
    let reach = |b: &[u8]| {
        let col = |x: usize| (0..fh).map(|y| b[(y * fw + x) * 4 + 3]).max().unwrap_or(0);
        (col(40), b[3])
    };
    let fx = |p: J| json!([{"instance_id": "fx-0-0", "type_id": "core.glow", "enabled": true, "parameters": p}]);
    let (before, said) = draw(json!([]));
    png_out::write_rgba(&dir.join("before.png"), fw, fh, OutputDepth::Eight, &[], &before).unwrap();
    t.row("before.png, the town with no glow: nothing outside the town", &format!("covering 40 pixels out {}, warnings {said:?}", reach(&before).0), said.is_empty() && reach(&before).0 == 0);
    let classic = json!({"based_on": "bright", "threshold": 60, "colors": [], "tolerance": 0, "radius": 60, "intensity": 1, "operation": "screen", "tint": "", "units": "after_effects"});
    for (file, what, effects, check) in [
        ("classic_glow.png", "the classic Glow, Bright parts above 60 %, radius 60, for comparison: one blur, a short even halo", fx(classic.clone()), 0usize),
        ("soft_glow_threshold_60.png", "Soft Physical Glow, threshold 60 with smooth 50, radius 60: a bright core on the light parts and a long soft tail, reaching past the town", fx(physical(json!({"threshold": 60, "threshold_smooth": 50, "radius": 60}))), 1),
        ("soft_glow_as_added.png", "Soft Physical Glow as added (radius 500, threshold 0): the whole town glows, a wide haze filling the frame to its corners", fx(physical(json!({}))), 2),
        ("soft_glow_stretched.png", "Soft Physical Glow, threshold 60 with smooth 50, radius 120, aspect 1.8: the glow stretched sideways, an anamorphic streak", fx(physical(json!({"threshold": 60, "threshold_smooth": 50, "radius": 120, "aspect_ratio": 1.8}))), 1),
    ] {
        let (bytes, said) = draw(effects);
        png_out::write_rgba(&dir.join(file), fw, fh, OutputDepth::Eight, &[], &bytes).unwrap();
        let (out40, corner) = reach(&bytes);
        let ok = said.is_empty()
            && match check {
                0 => true,
                1 => out40 > 0,
                _ => out40 > 0 && corner > 0,
            };
        t.row(&format!("{file}, {what}"), &format!("covering 40 pixels out {out40}, in the far corner {corner}; warnings {said:?}"), ok);
    }
    fs::remove_file(dir.join("town.png")).unwrap();

    t.finish("D-353_soft_glow_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-232's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole, with Draw on: GPU. The first loop starts
/// with empty caches and its 30 frames' median is "first"; then seven loops are timed, the
/// median of their 210 frames is "again".
#[test]
#[ignore = "B-233: a measurement, run deliberately with --release --ignored"]
fn b233_soft_glow_timing() {
    use std::fmt::Write as _;

    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n\
         | Shot | Quality | First | Again |\n|---|---|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let fx = |id: &str, p: J| json!({"instance_id": id, "type_id": "core.glow", "enabled": true, "parameters": p});
    let shots: [(&str, Option<J>); 5] = [
        ("Noise alone", None),
        ("Noise, then Soft Physical Glow as added, radius 500", Some(physical(json!({})))),
        ("Noise, then Soft Physical Glow, radius 60, threshold 40 smooth 50", Some(physical(json!({"radius": 60, "threshold": 40, "threshold_smooth": 50})))),
        ("Noise, then Soft Physical Glow, radius 2000", Some(physical(json!({"radius": 2000})))),
        ("Noise, then Soft Physical Glow, radius 60, threshold 40, no smooth (processor)", Some(physical(json!({"radius": 60, "threshold": 40})))),
    ];
    for (name, e) in shots {
        let stack = |id: &str| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true, "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}g"), p.clone()));
            }
            v
        };
        let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json")).expect("read the reference shot");
        let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
        let layers = &mut j["compositions"][0]["layers"];
        for (i, id) in ["a", "b", "c"].iter().enumerate() {
            layers[i]["effects"] = J::Array(stack(id));
        }
        let loaded = persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("{name}: {}", d.message));
        let (project, comp, root) = (loaded.document.project().clone(), Id::new("comp-reference-shot"), effect_table::repo("Fixtures/reference_shot"));
        let mut cache = CelCache::viewer();
        gpu.forget();
        let (mut first, mut times) = (Vec::new(), Vec::new());
        for pass in 0..8 {
            for frame in (0..240).step_by(8) {
                let mut log = FrameLog::new(3);
                let t = std::time::Instant::now();
                drop(preview::preview_frame_srgb8(&project, &comp, frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).expect("GPU frame"));
                let ms = t.elapsed().as_secs_f64() * 1000.0;
                if pass > 0 { times.push(ms) } else { first.push(ms) }
            }
        }
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B233_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-233_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
