//! B-425: D-545, pick #14 (EFFECTS.md row 261), Lens Dirt: a dirt picture from another layer,
//! lit where the light is bright, as a setting group on Soft Physical Glow and on Lens Flare, with
//! Deep Glow 2's controls (Opacity, Inherit Glow Color, Matte Exposure, Matte Gamma, Repeat
//! Edges), as PLUGINS.md's ranked list settles.
//!
//! Every expected pixel is `Fixtures/lens_dirt/expected_lens_dirt.json`, written by
//! `tools/lens_dirt_reference.py` before this code existed. D-353's, D-542's, D-543's, D-544's
//! and D-426's fixtures are run again, unchanged, since the two effects share the code. The dirt
//! is drawn on the card too, and checked against the processor within 1 level of 255 (ADR-006,
//! D-100).

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, median, said, set, Table, MAIN, TOWN};
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

fn dirty(e: &Effect) -> bool {
    matches!(e, Effect::SoftGlow { .. } | Effect::LensFlare { .. })
}

/// A Soft Physical Glow (radius 40) with `changes` laid over it.
fn glow(changes: J) -> J {
    let mut p = json!({"falloff": "physical", "threshold_mode": "chroma", "threshold": 50, "threshold_smooth": 30,
        "saturation_bias": 0, "radius": 40, "exposure": 1, "aspect_ratio": 1, "aspect_angle": 0, "operation": "screen",
        "source_opacity": 100, "unmult": "on"});
    for (k, v) in changes.as_object().unwrap() {
        p[k] = v.clone();
    }
    p
}

/// A Lens Flare as added with `changes` laid over it.
fn flare(changes: J) -> J {
    let mut p = json!({"flare_center": [30, 30], "flare_brightness": 100, "lens_type": "zoom", "blend_with_original": 0});
    for (k, v) in changes.as_object().unwrap() {
        p[k] = v.clone();
    }
    p
}

/// The reference shot with `type_id` of `parameters` on its first three layers, the fourth
/// layer as their dirt when they name it.
fn reference(type_id: &str, parameters: &J) -> Project {
    let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        layers[i]["effects"] = json!([{"instance_id": format!("b425-{id}"), "type_id": type_id, "enabled": true, "parameters": parameters}]);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c, render::OnCard::Fx(f) if dirty(&f.instance.effect)))
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

/// A dirt picture for the pictures: `w` by `h` soft grey smudges and specks of our own, placed by
/// a small fixed sequence, on clear.
fn smudges(w: usize, h: usize) -> Vec<u8> {
    let mut out = vec![0u8; w * h * 4];
    let mut seed = 12345u32;
    let mut next = || {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        (seed >> 8) as f64 / (1u32 << 24) as f64
    };
    let spots: Vec<(f64, f64, f64, f64)> = (0..40).map(|_| (next() * w as f64, next() * h as f64, 2.0 + next() * 14.0, 0.3 + next() * 0.7)).collect();
    for y in 0..h {
        for x in 0..w {
            let mut a: f64 = 0.0;
            for &(cx, cy, r, s) in &spots {
                let d = ((x as f64 + 0.5 - cx).powi(2) + (y as f64 + 0.5 - cy).powi(2)).sqrt() / r;
                a = a.max(s * (1.0 - d).clamp(0.0, 1.0));
            }
            let i = (y * w + x) * 4;
            out[i..i + 4].copy_from_slice(&[230, 225, 210, (a * 255.0).round() as u8]);
        }
    }
    out
}

/// B-233's framed town: the town (480 by 270) in a composition 160 pixels wider and taller, with
/// the dirt picture (`dirt.png`) as a layer turned off.
fn framed(effects: J) -> Project {
    let (w, h) = TOWN;
    let t = |v: J| json!({"base": v, "keyframes": []});
    let layer = |id: &str, asset: &str, enabled: bool, effects: J| json!({"id": id, "kind": "raster", "name": id, "asset_id": asset, "enabled": enabled,
        "locked": false, "in_frame": 0, "out_frame": 8, "source_offset_frames": 0,
        "transform": {"anchor": t(json!([0, 0])), "position": t(json!([80, 80])), "scale": t(json!([100, 100])),
                      "rotation": t(json!(0)), "opacity": t(json!(1))},
        "exposure_spans": [], "mask": null, "matte": null, "blend_mode": "normal", "effects": effects});
    let project = json!({
        "schema_version": 0, "project_id": "proj-b425",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-town", "kind": "still", "name": "town", "path": "town.png",
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}},
                   {"id": "asset-dirt", "kind": "still", "name": "dirt", "path": "dirt.png",
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
            "id": MAIN, "name": "Main", "width": w + 160, "height": h + 160, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 8,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 8},
            "layer_order": ["art", "dirt"],
            "layers": [layer("art", "asset-town", true, effects), layer("dirt", "asset-dirt", false, json!([]))]
        }]
    });
    persist::load_str(&project.to_string()).unwrap_or_else(|d| panic!("the framed town opens: {}", d.message)).document.project().clone()
}

#[test]
fn b425_lens_dirt() {
    let mut t = Table::new(
        "soft_glow",
        "# B-425: Lens Dirt (pick #14)\n\n\
         D-545 (EFFECTS.md row 261): Soft Physical Glow and Lens Flare each gain a Dirt Layer: \
         the picture of another layer (a dirty lens, smudges, specks), centred on the layer and \
         mirror-repeated past its edges unless Repeat Edges is off, shown only where the glow or \
         the flare is bright, as dirt on a real lens lights up only in strong light. The matte is \
         the light's brightness (Inherit Glow Color 0) or the light channel by channel (100), \
         brightened by Matte Exposure and shaped by Matte Gamma; the dirt goes over the light at \
         Opacity. Deep Glow 2's controls. Both effects draw as before without a dirt layer, so \
         D-353's 45 fixtures, D-542's 22, D-543's 20, D-544's 18 and D-426's 19 are run again \
         here unchanged. Every expected pixel is `Fixtures/lens_dirt/expected_lens_dirt.json`, \
         written by `tools/lens_dirt_reference.py` before this code existed (FX-DIRT-001 to \
         029), compared sample by sample against the catalogue's tolerance of 2e-5. The dirt is \
         drawn on the card too: the card's picture against the processor's, within 1 level of \
         255 (ADR-006, D-100).\n",
    );

    t.heading("D-353's FX-SGLOW-001 to 045, unchanged: Soft Physical Glow draws as before");
    t.fixtures("expected_soft_glow.json");
    t.round_trips(&["fx_sglow_001.json", "fx_sglow_020.json"]);
    for (root, heading, expected) in [
        ("glow_tint", "D-542's FX-GTINT-001 to 022, unchanged", "expected_glow_tint.json"),
        ("spectral_glow", "D-543's FX-SPGLOW-001 to 020, unchanged", "expected_spectral_glow.json"),
        ("halation", "D-544's FX-HALO-001 to 018, unchanged", "expected_halation.json"),
        ("lens_flare", "D-426's FX-FLARE-001 to 019, unchanged: Lens Flare draws as before", "expected_lens_flare.json"),
    ] {
        t.root = effect_table::repo(&format!("Fixtures/{root}"));
        t.heading(heading);
        t.fixtures(expected);
    }
    t.round_trips(&["fx_flare_001.json", "fx_flare_010.json"]);

    t.root = effect_table::repo("Fixtures/lens_dirt");
    t.heading("FX-DIRT-001 to 029");
    t.fixtures("expected_lens_dirt.json");

    t.heading("The file");
    let files: Vec<String> = (1..=29).map(|n| format!("fx_dirt_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let p1 = t.saved_parameters("fx_dirt_001.json");
    t.shape_refused("fx_dirt_001.json", "a dirt opacity written as a word", &{
        let mut o = p1.clone();
        o["dirt_opacity"] = json!("lots");
        o.to_string()
    });
    t.shape_refused("fx_dirt_001.json", "a dirt repeat written as a number", &{
        let mut o = p1.clone();
        o["dirt_repeat"] = json!(1);
        o.to_string()
    });
    for (file, said) in [
        ("fx_dirt_023.json", "Soft Physical Glow's dirt opacity runs from 0 to 100, and this is 101."),
        ("fx_dirt_026.json", "Soft Physical Glow's dirt gamma runs from 0.1 to 10, and this is 0.05."),
        ("fx_dirt_027.json", "Soft Physical Glow's dirt repeat is \"on\" or \"off\", and this is \"yes\"."),
    ] {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming it"), &why, why == said);
    }

    t.heading("Commands");
    let mut document = t.load("fx_dirt_001.json").document;
    let base = effect_of(&document);
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = base.clone();
        f(&mut e);
        e
    };
    t.refused(
        &mut document,
        vec![
            ("dirt opacity 101", set(with(&|e| if let Effect::SoftGlow { dirt, .. } = e { dirt.opacity = 101.0 }))),
            ("dirt gamma 0", set(with(&|e| if let Effect::SoftGlow { dirt, .. } = e { dirt.gamma = 0.0 }))),
            ("dirt repeat \"yes\"", set(with(&|e| if let Effect::SoftGlow { dirt, .. } = e { dirt.repeat = "yes".into() }))),
        ],
    );
    t.taken(
        &mut document,
        "fx_dirt_001.json",
        vec![
            ("dirt layer specks, inherit 100, exposure 1, repeat off", set(with(&|e| if let Effect::SoftGlow { dirt, .. } = e {
                dirt.layer = json!("specks");
                dirt.inherit = 100.0;
                dirt.exposure = 1.0;
                dirt.repeat = "off".into();
            }))),
            ("dirt opacity keyed 0 to 100 over frames 0 to 4", keys("dirt_opacity", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );
    // D-68: a setting keyed is saved with its keys, even at its start value.
    let loaded = t.load("fx_dirt_001.json");
    let mut keyed = loaded.document;
    keyed.apply(keys("dirt_gamma", &[(0, &[1.0]), (4, &[2.0])])).expect("dirt gamma keyed");
    let back = persist::load_str(&persist::to_json(keyed.project(), &loaded.preserved)).expect("opened again").document;
    // D-68: the keys and the constants; a keyed setting's unused base is not compared.
    let fx = |d: &Document| d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].clone();
    let (a, b) = (fx(&back), fx(&keyed));
    let kept = a.effect == b.effect && a.keys("dirt_gamma").len() == 2 && a.keys("dirt_gamma") == b.keys("dirt_gamma");
    t.row("dirt gamma keyed 1 to 2, saved and opened again: the keys are kept", if kept { "kept" } else { "lost" }, kept);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_dirt_001.json", 0), ("fx_dirt_002.json", 0), ("fx_dirt_012.json", 0), ("fx_dirt_013.json", 0), ("fx_dirt_017.json", 0), ("fx_dirt_021.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    let fixture_root = effect_table::repo("Fixtures/lens_dirt");
    for n in 1..=22 {
        let file = format!("fx_dirt_{n:03}.json");
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
        // FX-DIRT-016's dirt layer is missing, so the effect is left out on both.
        let wanted = if n == 16 { 0 } else { 10 };
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && card == wanted,
        );
    }
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = effect_table::repo("Fixtures/reference_shot");
    for (what, type_id, p) in [
        ("Soft Physical Glow, the fourth layer as its dirt, every dirt setting as added", "core.glow", glow(json!({"dirt_layer": "layer-4"}))),
        ("Soft Physical Glow, the fourth layer as its dirt, inherit 60, exposure 2, gamma 2.2, repeat off", "core.glow",
            glow(json!({"dirt_layer": "layer-4", "dirt_inherit": 60, "dirt_exposure": 2, "dirt_gamma": 2.2, "dirt_repeat": "off"}))),
        ("Lens Flare, the fourth layer as its dirt, exposure 2", "core.lens_flare", flare(json!({"dirt_layer": "layer-4", "dirt_exposure": 2}))),
    ] {
        let project = reference(type_id, &p);
        let mut none = p.clone();
        none["dirt_opacity"] = json!(0);
        let plain = reference(type_id, &none);
        let cpu = |project: &Project| {
            let mut log = FrameLog::new(3);
            preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
                .expect("the reference shot draws")
                .to_srgb8_straight()
        };
        let changed = distance(&cpu(&project), &cpu(&plain)).1;
        t.row(
            &format!("the reference shot, {what}: the processor's frame 100 differs from the same at dirt opacity 0, so the comparisons below test the dirt"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(&mut gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, {what}, on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }

    t.heading("Pictures: the town, set in a larger frame, with smudges of our own as the dirt, in `verification/D-545 pictures/`");
    let dir = effect_table::repo("verification/D-545 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    png_out::write_rgba(&dir.join("town.png"), w, h, OutputDepth::Eight, &[], &effect_table::town()).unwrap();
    let (dw, dh) = (160, 100);
    png_out::write_rgba(&dir.join("dirt.png"), dw, dh, OutputDepth::Eight, &[], &smudges(dw, dh)).unwrap();
    let (fw, fh) = (w + 160, h + 160);
    let draw = |effects: J| {
        let mut log = FrameLog::new(8);
        let b = render_frame(&framed(effects), &Id::new(MAIN), 0, &dir, 64, &mut log).expect("the town draws");
        (b.to_srgb8_straight(), log.finish().iter().map(|d| d.id.as_str().to_string()).collect::<Vec<_>>())
    };
    let fx = |type_id: &str, p: J| json!([{"instance_id": "fx-0-0", "type_id": type_id, "enabled": true, "parameters": p}]);
    let pictures = [
        ("glow_plain.png", "Soft Physical Glow (threshold 50, radius 40), no dirt: for comparison", fx("core.glow", glow(json!({})))),
        ("glow_dirt.png", "the same with the smudges as its dirt, every dirt setting as added: smudges lit round the town's lights", fx("core.glow", glow(json!({"dirt_layer": "dirt"})))),
        ("glow_dirt_strong.png", "Matte Exposure 3, Matte Gamma 2: the smudges show further into the glow", fx("core.glow", glow(json!({"dirt_layer": "dirt", "dirt_exposure": 3, "dirt_gamma": 2})))),
        ("glow_dirt_colour.png", "Inherit Glow Color 100: the smudges in the glow's own colours", fx("core.glow", glow(json!({"dirt_layer": "dirt", "dirt_exposure": 3, "dirt_inherit": 100})))),
        ("glow_dirt_once.png", "Repeat Edges off, Matte Exposure 3: the smudge picture once, in the middle", fx("core.glow", glow(json!({"dirt_layer": "dirt", "dirt_exposure": 3, "dirt_repeat": "off"})))),
        ("flare_plain.png", "Lens Flare as added, no dirt: for comparison", fx("core.lens_flare", flare(json!({})))),
        ("flare_dirt.png", "the same with the smudges as its dirt, Matte Exposure 2: smudges lit round the flare", fx("core.lens_flare", flare(json!({"dirt_layer": "dirt", "dirt_exposure": 2})))),
    ];
    let mut drawn = Vec::new();
    for (file, what, effects) in pictures {
        let (bytes, said) = draw(effects);
        png_out::write_rgba(&dir.join(file), fw, fh, OutputDepth::Eight, &[], &bytes).unwrap();
        drawn.push((file, what, bytes, said));
    }
    let plain_of = |file: &str| if file.starts_with("glow") { 0 } else { 5 };
    for (file, what, bytes, said) in &drawn {
        let (_, changed) = distance(bytes, &drawn[plain_of(file)].2);
        let plain = file.ends_with("plain.png");
        t.row(
            &format!("{file}, {what}"),
            &format!("{changed} pixels differ from the same without dirt; warnings {said:?}"),
            said.is_empty() && (plain || changed > 0),
        );
    }
    fs::remove_file(dir.join("town.png")).unwrap();

    t.finish("D-545_lens_dirt_table.md");
}

/// The frame times, B-233's way: the reference shot with a changing Noise on its first three
/// layers, then the effect, every eighth frame asked for as the viewer asks, with Draw on: GPU.
#[test]
#[ignore = "B-425: a measurement, run deliberately with --release --ignored"]
fn b425_lens_dirt_timing() {
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
    let shots: [(&str, Option<(&str, J)>); 5] = [
        ("Noise alone", None),
        ("Noise, then Soft Physical Glow (radius 40)", Some(("core.glow", glow(json!({}))))),
        ("Noise, then Soft Physical Glow with the fourth layer as its dirt", Some(("core.glow", glow(json!({"dirt_layer": "layer-4"}))))),
        ("Noise, then Lens Flare", Some(("core.lens_flare", flare(json!({}))))),
        ("Noise, then Lens Flare with the fourth layer as its dirt", Some(("core.lens_flare", flare(json!({"dirt_layer": "layer-4"}))))),
    ];
    for (name, e) in shots {
        let stack = |id: &str| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true, "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some((type_id, p)) = &e {
                v.push(json!({"instance_id": format!("{id}d"), "type_id": type_id, "enabled": true, "parameters": p}));
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
    let out = std::env::var("B425_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-425_lens_dirt_timing_table.md"));
    fs::write(out, s).expect("write the timing table");
}
