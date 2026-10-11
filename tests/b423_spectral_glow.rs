//! B-423: D-543, pick #10 (EFFECTS.md row 259), Spectral Glow: Soft Physical Glow's Spectral,
//! Spectral Red, Spectral Green and Spectral Blue, on the shared soft-glow engine (D-353).
//!
//! Every expected pixel is `Fixtures/spectral_glow/expected_spectral_glow.json`, written by
//! `tools/spectral_glow_reference.py` before this code existed. D-353's and D-542's fixtures are
//! run again, unchanged, so a glow with Spectral off draws as it did. Spectral is drawn on the
//! card too, and checked against the processor within 1 level of 255 (ADR-006, D-100).

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
        // B-233's reason: a pixel clear in both has no colour to compare.
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

/// A Soft Physical Glow's parameters: the effect as added with `changes` laid over them.
fn physical(changes: J) -> J {
    let mut p = json!({"falloff": "physical", "threshold_mode": "chroma", "threshold": 0, "threshold_smooth": 0,
        "saturation_bias": 0, "radius": 500, "exposure": 1, "aspect_ratio": 1, "aspect_angle": 0,
        "operation": "screen", "source_opacity": 100, "unmult": "on"});
    for (k, v) in changes.as_object().unwrap() {
        p[k] = v.clone();
    }
    p
}

/// The reference shot with a Soft Physical Glow of `parameters` on its first three layers.
fn reference(parameters: &J) -> Project {
    let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        layers[i]["effects"] = json!([{"instance_id": format!("b423-{id}"), "type_id": "core.glow", "enabled": true, "parameters": parameters}]);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c, render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::SoftGlow { .. })))
        .count()
}

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

/// B-233's framed town: the town (480 by 270) in a composition 160 pixels wider and taller.
fn framed(effects: J) -> Project {
    let (w, h) = TOWN;
    let t = |v: J| json!({"base": v, "keyframes": []});
    let project = json!({
        "schema_version": 0, "project_id": "proj-b423",
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
    persist::load_str(&project.to_string()).unwrap_or_else(|d| panic!("the framed town opens: {}", d.message)).document.project().clone()
}

#[test]
fn b423_spectral_glow() {
    let mut t = Table::new(
        "soft_glow",
        "# B-423: Soft Physical Glow's Spectral (pick #10, Spectral Glow)\n\n\
         D-543 (EFFECTS.md row 259): four new settings of Soft Physical Glow, after Deep Glow's \
         Glow Aberration (PLUGINS.md 2.3): with Spectral on, red, green and blue each glow at \
         their own share of Radius (80, 100 and 120 per cent as added), so the glow's tail splits \
         into colour like light through a lens. A glow with Spectral off, or a file from before, \
         draws exactly as before: D-353's 45 fixtures and D-542's 22 are run again here unchanged. \
         Every expected pixel is `Fixtures/spectral_glow/expected_spectral_glow.json`, written by \
         `tools/spectral_glow_reference.py` before this code existed (FX-SPGLOW-001 to 020), \
         compared sample by sample against the catalogue's tolerance of 2e-5. Spectral is drawn on \
         the card too: the card's picture against the processor's, within 1 level of 255 (ADR-006, \
         D-100).\n",
    );
    let spectral_keys = ["spectral", "spectral_red", "spectral_green", "spectral_blue"];

    t.heading("D-353's FX-SGLOW-001 to 045, unchanged: Spectral off draws as before");
    t.fixtures("expected_soft_glow.json");
    let files: Vec<String> = (1..=45).map(|n| format!("fx_sglow_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let params = t.saved_parameters("fx_sglow_001.json");
    t.row(
        "fx_sglow_001.json, a glow from before Spectral, is saved with none of Spectral's settings",
        &params.to_string(),
        spectral_keys.iter().all(|k| params.get(*k).is_none()),
    );

    t.root = effect_table::repo("Fixtures/glow_tint");
    t.heading("D-542's FX-GTINT-001 to 022, unchanged: a tint with Spectral off draws as before");
    t.fixtures("expected_glow_tint.json");

    t.root = effect_table::repo("Fixtures/spectral_glow");
    t.heading("FX-SPGLOW-001 to 020");
    t.fixtures("expected_spectral_glow.json");

    t.heading("The file");
    let files: Vec<String> = (1..=20).map(|n| format!("fx_spglow_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    t.shape_refused("fx_spglow_001.json", "a spectral red written as a word", &physical(json!({"spectral": "on", "spectral_red": "wide"})).to_string());
    t.shape_refused("fx_spglow_001.json", "a spectral written as a number", &physical(json!({"spectral": 1})).to_string());
    for (file, said) in [
        ("fx_spglow_017.json", "Soft Physical Glow's spectral is \"off\" or \"on\", and this is \"yes\"."),
        ("fx_spglow_018.json", "Soft Physical Glow's spectral red runs from 0 to 200, and this is 201."),
        ("fx_spglow_019.json", "Soft Physical Glow's spectral blue runs from 0 to 200, and this is -1."),
    ] {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming it"), &why, why == said);
    }

    t.heading("Commands");
    let mut document = t.load("fx_spglow_002.json").document;
    let base = effect_of(&document);
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = base.clone();
        f(&mut e);
        e
    };
    t.refused(
        &mut document,
        vec![
            ("spectral \"yes\"", set(with(&|e| if let Effect::SoftGlow { spectral, .. } = e { *spectral = "yes".into() }))),
            ("spectral red 201", set(with(&|e| if let Effect::SoftGlow { spectral_red, .. } = e { *spectral_red = 201.0 }))),
            ("spectral blue -1", set(with(&|e| if let Effect::SoftGlow { spectral_blue, .. } = e { *spectral_blue = -1.0 }))),
        ],
    );
    t.taken(
        &mut document,
        "fx_spglow_002.json",
        vec![
            ("red 50, blue 200", set(with(&|e| if let Effect::SoftGlow { spectral_red, spectral_blue, .. } = e {
                *spectral_red = 50.0;
                *spectral_blue = 200.0;
            }))),
            ("spectral blue keyed 100 to 200 over frames 0 to 4", keys("spectral_blue", &[(0, &[100.0]), (4, &[200.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_spglow_001.json", 0), ("fx_spglow_004.json", 0), ("fx_spglow_008.json", 0), ("fx_spglow_013.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    let fixture_root = effect_table::repo("Fixtures/spectral_glow");
    for n in 1..=16 {
        let file = format!("fx_spglow_{n:03}.json");
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
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && card == 10,
        );
    }
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = effect_table::repo("Fixtures/reference_shot");
    for (what, p) in [
        ("Spectral on as added (radius 500; 80, 100, 120)", physical(json!({"spectral": "on"}))),
        ("Spectral red 50, blue 200, radius 60, threshold 40 with smooth 50", physical(json!({"spectral": "on", "spectral_red": 50, "spectral_blue": 200, "radius": 60, "threshold": 40, "threshold_smooth": 50}))),
        ("Spectral on with Tint Core to Edge, radius 150, aspect 1.6 at 25 degrees, add, unmult off", physical(json!({"spectral": "on", "tint": "core_to_edge", "radius": 150, "aspect_ratio": 1.6, "aspect_angle": 25, "operation": "add", "unmult": "off"}))),
    ] {
        let project = reference(&p);
        let mut off = p.clone();
        off["spectral"] = json!("off");
        let plain = reference(&off);
        let cpu = |project: &Project| {
            let mut log = FrameLog::new(3);
            preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
                .expect("the reference shot draws")
                .to_srgb8_straight()
        };
        let changed = distance(&cpu(&project), &cpu(&plain)).1;
        t.row(
            &format!("the reference shot, Soft Physical Glow {what}: the processor's frame 100 differs from the same glow with Spectral off, so the comparisons below test Spectral"),
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

    t.heading("Pictures: the town, set in a larger frame, in `verification/D-543 pictures/`");
    let dir = effect_table::repo("verification/D-543 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    png_out::write_rgba(&dir.join("town.png"), w, h, OutputDepth::Eight, &[], &town()).unwrap();
    let (fw, fh) = (w + 160, h + 160);
    let draw = |effects: J| {
        let mut log = FrameLog::new(8);
        let b = render_frame(&framed(effects), &Id::new(MAIN), 0, &dir, 64, &mut log).expect("the town draws");
        (b.to_srgb8_straight(), log.finish().iter().map(|d| d.id.as_str().to_string()).collect::<Vec<_>>())
    };
    let fx = |p: J| json!([{"instance_id": "fx-0-0", "type_id": "core.glow", "enabled": true, "parameters": p}]);
    // The glow 10 pixels out from the town's left edge, in the frame's middle row: its blue
    // against its red says how far each colour reaches.
    let tail = |b: &[u8]| {
        let i = ((fh / 2) * fw + 70) * 4;
        [b[i], b[i + 1], b[i + 2], b[i + 3]]
    };
    let lit = |extra: J| {
        let mut p = physical(json!({"threshold": 60, "threshold_smooth": 50, "radius": 60}));
        for (k, v) in extra.as_object().unwrap() {
            p[k] = v.clone();
        }
        p
    };
    let blueness = |px: [u8; 4]| px[2] as f64 / (px[0] as f64 + 1.0);
    let mut off = 0.0;
    for (file, what, p) in [
        ("spectral_off.png", "Soft Physical Glow, threshold 60 with smooth 50, radius 60, Spectral off, for comparison", lit(json!({}))),
        ("spectral_on.png", "the same, Spectral on as added (red 80, green 100, blue 120): the tail bluer than with Spectral off", lit(json!({"spectral": "on"}))),
        ("spectral_strong.png", "the same, red 50 and blue 200: a strong fringe, blue far out, red held close", lit(json!({"spectral": "on", "spectral_red": 50, "spectral_blue": 200}))),
        ("spectral_reversed.png", "the same, red 200 and blue 50: the other way round, the tail redder than with Spectral off", lit(json!({"spectral": "on", "spectral_red": 200, "spectral_blue": 50}))),
    ] {
        let (bytes, said) = draw(fx(p));
        png_out::write_rgba(&dir.join(file), fw, fh, OutputDepth::Eight, &[], &bytes).unwrap();
        let px = tail(&bytes);
        let b = blueness(px);
        let ok = said.is_empty()
            && px[3] > 0
            && match file {
                "spectral_off.png" => {
                    off = b;
                    true
                }
                "spectral_reversed.png" => b < off,
                _ => b > off,
            };
        t.row(&format!("{file}, {what}"), &format!("the tail 10 pixels out, red {} green {} blue {} covering {}; warnings {said:?}", px[0], px[1], px[2], px[3]), ok);
    }
    fs::remove_file(dir.join("town.png")).unwrap();

    t.finish("D-543_spectral_glow_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-233's way: the reference shot with a changing Noise on its first three
/// layers, then the glow, every eighth frame asked for as the viewer asks, with Draw on: GPU.
#[test]
#[ignore = "B-423: a measurement, run deliberately with --release --ignored"]
fn b423_spectral_glow_timing() {
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
    let shots: [(&str, Option<J>); 4] = [
        ("Noise alone", None),
        ("Noise, then Soft Physical Glow, radius 60, threshold 40 smooth 50, Spectral off", Some(physical(json!({"radius": 60, "threshold": 40, "threshold_smooth": 50})))),
        ("Noise, then the same, Spectral on (80, 100, 120)", Some(physical(json!({"radius": 60, "threshold": 40, "threshold_smooth": 50, "spectral": "on"})))),
        ("Noise, then Soft Physical Glow as added, radius 500, Spectral on (80, 100, 120)", Some(physical(json!({"spectral": "on"})))),
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
    let out = std::env::var("B423_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-423_spectral_glow_timing_table.md"));
    fs::write(out, s).expect("write the timing table");
}
