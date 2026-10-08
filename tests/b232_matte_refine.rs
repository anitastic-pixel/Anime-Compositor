//! B-232: D-352, the guided filter (EFFECTS.md P0-20) and the three effects built on it:
//! Matte Choker, Refine Hard Matte and Refine Soft Matte, after After Effects' effects of those
//! names.
//!
//! Every expected pixel is `Fixtures/matte_refine/expected_matte_refine.json`, written by
//! `tools/matte_refine_reference.py` before this code existed. Matte Choker is drawn on the card
//! too, and checked against the processor within 1 level of 255 (ADR-006, D-100); the two
//! Refine effects are drawn by the processor.

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

/// Whether the town's pixel (x, y) is a house or the road, not sky, as `town` draws it.
fn solid(x: usize, y: usize) -> bool {
    let (house, lx) = (x / 48, x % 48);
    let top = 110 + (house * 37 % 5) * 16;
    y >= 210 || ((top..210).contains(&y) && lx < 45)
}

/// Within `r` pixels (a square) of the other kind.
fn near_edge(x: usize, y: usize, r: usize) -> bool {
    let (w, h) = TOWN;
    let me = solid(x, y);
    (y.saturating_sub(r)..(y + r + 1).min(h)).any(|yy| (x.saturating_sub(r)..(x + r + 1).min(w)).any(|xx| solid(xx, yy) != me))
}

/// The town keyed off its sky the way a rough key leaves it: the houses and road kept, a band
/// 2 pixels wide inside every edge 60 per cent covered and tinted 40 per cent toward the sky
/// behind (spill), a scatter of specks left in the sky and of holes in the houses. Written into
/// `dir` as `keyed.png`, straight alpha.
fn keyed_town(dir: &Path) {
    let (w, h) = TOWN;
    let sky = |y: usize| {
        let t = y as f64 / h as f64;
        [120.0 + 100.0 * t, 170.0 + 50.0 * t, 230.0 - 30.0 * t]
    };
    let base = town();
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let p = &base[(y * w + x) * 4..][..4];
            let (c, a) = if !solid(x, y) {
                (sky(y), if (x * 7 + y * 13) % 397 == 0 && !near_edge(x, y, 4) { 0.3 } else { 0.0 })
            } else if near_edge(x, y, 2) {
                let s = sky(y);
                ([0, 1, 2].map(|i| 0.6 * p[i] as f64 + 0.4 * s[i]), 0.6)
            } else {
                ([p[0] as f64, p[1] as f64, p[2] as f64], if (x * 11 + y * 5) % 389 == 0 && !near_edge(x, y, 4) { 0.0 } else { 1.0 })
            };
            bytes.extend([c[0].round() as u8, c[1].round() as u8, c[2].round() as u8, (a * 255.0f64).round() as u8]);
        }
    }
    png_out::write_rgba(&dir.join("keyed.png"), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
}

/// One layer of `keyed.png` filling the town-sized composition, carrying `effects`.
fn keyed(effects: J) -> Project {
    let (w, h) = TOWN;
    let t = |v: J| json!({"base": v, "keyframes": []});
    let project = json!({
        "schema_version": 0, "project_id": "proj-b232",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-keyed", "kind": "still", "name": "keyed", "path": "keyed.png",
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
            "id": MAIN, "name": "Main", "width": w, "height": h, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 8,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 8},
            "layer_order": ["art"],
            "layers": [{"id": "art", "kind": "raster", "name": "art", "asset_id": "asset-keyed", "enabled": true,
                "locked": false, "in_frame": 0, "out_frame": 8, "source_offset_frames": 0,
                "transform": {"anchor": t(json!([0, 0])), "position": t(json!([0, 0])), "scale": t(json!([100, 100])),
                              "rotation": t(json!(0)), "opacity": t(json!(1))},
                "exposure_spans": [], "mask": null, "matte": null, "blend_mode": "normal", "effects": effects}]
        }]
    });
    persist::load_str(&project.to_string()).unwrap_or_else(|d| panic!("the keyed town opens: {}", d.message)).document.project().clone()
}

fn fx(type_id: &str, p: J) -> J {
    json!({"instance_id": "fx-0-0", "type_id": type_id, "enabled": true, "parameters": p})
}

fn choker() -> J {
    fx("core.matte_choker", json!({"geometric_softness_1": 4, "choke_1": 75, "gray_level_softness_1": 10,
        "geometric_softness_2": 0, "choke_2": 0, "gray_level_softness_2": 100, "iterations": 1}))
}

fn hard(decontaminate: &str, view_map: &str) -> J {
    fx("core.refine_hard_matte", json!({"feather": 2, "contrast": 50, "shift_edge": 0, "decontaminate": decontaminate,
        "decontamination_amount": 100, "decontamination_radius": 0, "view_decontamination_map": view_map}))
}

fn soft(edge_radius: f64, view_edge: &str) -> J {
    fx("core.refine_soft_matte", json!({"edge_radius": edge_radius, "view_edge_region": view_edge, "feather": 0, "contrast": 0, "shift_edge": 0,
        "decontaminate": "on", "decontamination_amount": 100, "decontamination_radius": 0, "view_decontamination_map": "off"}))
}

fn distance(a: &[u8], b: &[u8]) -> (u8, usize) {
    assert_eq!(a.len(), b.len(), "the two pictures are different sizes");
    let (mut largest, mut pixels) = (0, 0);
    for (p, q) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
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

/// The reference shot (1920 by 1080) with a Matte Choker of `parameters` on its first three
/// layers, as B-223 has it.
fn reference(parameters: &J) -> Project {
    let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        layers[i]["effects"] = json!([{"instance_id": format!("b232-{id}"), "type_id": "core.matte_choker", "enabled": true, "parameters": parameters}]);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

/// The Matte Choker effects the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c, render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::MatteChoker { .. })))
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

#[test]
fn b232_matte_refine() {
    let mut t = Table::new(
        "matte_refine",
        "# B-232: the guided filter, and Matte Choker, Refine Hard Matte and Refine Soft Matte\n\n\
         D-352 (EFFECTS.md P0-20): an edge-aware (guided) filter, and three effects that clean a \
         keyed layer's edge, after After Effects' Matte Choker, Refine Hard Matte and Refine Soft \
         Matte. Every expected pixel is `Fixtures/matte_refine/expected_matte_refine.json`, \
         written by `tools/matte_refine_reference.py` before this code existed and printed in \
         document 25 as FX-MREF-001 to 035. The build's frame is compared sample by sample; the \
         answer is the largest difference over all of them, against the catalogue's tolerance of \
         2e-5. Matte Choker is drawn on the card too: the card's picture against the \
         processor's, within 1 level of 255 (ADR-006, D-100).\n",
    );

    t.heading("FX-MREF-001 to 035 (document 25)");
    t.fixtures("expected_matte_refine.json");

    t.heading("The file");
    let files: Vec<String> = (1..=35).map(|n| format!("fx_mref_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let params = t.saved_parameters("fx_mref_013.json");
    t.row(
        "fx_mref_013.json: Refine Hard Matte writes no edge radius and no view edge region, which are Refine Soft Matte's",
        &params.to_string(),
        params.get("edge_radius").is_none() && params.get("view_edge_region").is_none(),
    );
    let params = t.saved_parameters("fx_mref_023.json");
    t.row(
        "fx_mref_023.json: Refine Soft Matte writes its edge radius and view edge region",
        &params.to_string(),
        params.get("edge_radius").is_some() && params.get("view_edge_region").is_some(),
    );
    t.shape_refused("fx_mref_001.json", "Matte Choker with no iterations", r#"{"geometric_softness_1": 4, "choke_1": 75, "gray_level_softness_1": 10, "geometric_softness_2": 0, "choke_2": 0, "gray_level_softness_2": 100}"#);
    t.shape_refused("fx_mref_001.json", "a choke written as a word", r#"{"geometric_softness_1": 4, "choke_1": "lots", "gray_level_softness_1": 10, "geometric_softness_2": 0, "choke_2": 0, "gray_level_softness_2": 100, "iterations": 1}"#);
    t.shape_refused("fx_mref_013.json", "Refine Hard Matte's decontaminate written as a number", r#"{"feather": 2, "contrast": 50, "shift_edge": 0, "decontaminate": 1, "decontamination_amount": 100, "decontamination_radius": 0, "view_decontamination_map": "off"}"#);
    t.shape_refused("fx_mref_023.json", "Refine Soft Matte with no edge radius", r#"{"view_edge_region": "off", "feather": 0, "contrast": 0, "shift_edge": 0, "decontaminate": "on", "decontamination_amount": 100, "decontamination_radius": 0, "view_decontamination_map": "off"}"#);
    for (file, said) in [
        ("fx_mref_009.json", "Matte Choker's choke 1 runs from -127 to 127, and this is 128."),
        ("fx_mref_010.json", "Matte Choker's gray level softness 2 runs from 0 to 100, and this is 101."),
        ("fx_mref_011.json", "Matte Choker's iterations runs from 1 to 10, and this is 0."),
        ("fx_mref_012.json", "Matte Choker's geometric softness 1 runs from 0 to 100, and this is -1."),
        ("fx_mref_028.json", "Refine Hard Matte's feather runs from 0 to 100, and this is 101."),
        ("fx_mref_029.json", "Refine Hard Matte's contrast runs from 0 to 100, and this is -1."),
        ("fx_mref_030.json", "Refine Hard Matte's decontaminate is \"off\" or \"on\", and this is \"yes\"."),
        ("fx_mref_031.json", "Refine Hard Matte's shift edge runs from -100 to 100, and this is 101."),
        ("fx_mref_032.json", "Refine Hard Matte's decontamination radius runs from 0 to 100, and this is -1."),
        ("fx_mref_033.json", "Refine Soft Matte's edge radius runs from 0 to 100, and this is 101."),
        ("fx_mref_034.json", "Refine Soft Matte's view edge region is \"off\" or \"on\", and this is \"maybe\"."),
        ("fx_mref_035.json", "Refine Soft Matte's decontamination amount runs from 0 to 100, and this is 101."),
    ] {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming it"), &why, why == said);
    }

    t.heading("Commands");
    let mut document = t.load("fx_mref_001.json").document;
    let base = effect_of(&document);
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = base.clone();
        f(&mut e);
        e
    };
    t.refused(
        &mut document,
        vec![
            ("choke 1 at 128", set(with(&|e| if let Effect::MatteChoker { choke_1, .. } = e { *choke_1 = 128.0 }))),
            ("iterations 11", set(with(&|e| if let Effect::MatteChoker { iterations, .. } = e { *iterations = 11.0 }))),
        ],
    );
    t.taken(
        &mut document,
        "fx_mref_001.json",
        vec![
            ("choke 1 at -40 and two iterations", set(with(&|e| if let Effect::MatteChoker { choke_1, iterations, .. } = e {
                *choke_1 = -40.0;
                *iterations = 2.0;
            }))),
            ("geometric softness 1 keyed 0 to 6 over frames 0 to 4", keys("geometric_softness_1", &[(0, &[0.0]), (4, &[6.0])])),
        ],
    );
    let mut document = t.load("fx_mref_013.json").document;
    let base = effect_of(&document);
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = base.clone();
        f(&mut e);
        e
    };
    t.refused(
        &mut document,
        vec![("Refine Hard Matte's decontaminate \"yes\"", set(with(&|e| if let Effect::RefineMatte { decontaminate, .. } = e { *decontaminate = "yes".into() })))],
    );
    t.taken(
        &mut document,
        "fx_mref_013.json",
        vec![("Refine Hard Matte with decontaminate on, amount 60", set(with(&|e| if let Effect::RefineMatte { decontaminate, decontamination_amount, .. } = e {
            *decontaminate = "on".into();
            *decontamination_amount = 60.0;
        })))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_mref_001.json", 0),
        ("fx_mref_006.json", 0),
        ("fx_mref_013.json", 0),
        ("fx_mref_018.json", 0),
        ("fx_mref_023.json", 0),
        ("fx_mref_027.json", 0),
    ]);

    t.heading("Matte Choker on the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    let fixture_root = effect_table::repo("Fixtures/matte_refine");
    for n in 1..=12 {
        let file = format!("fx_mref_{n:03}.json");
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
            largest <= 1 && !refused && agree,
        );
    }
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = effect_table::repo("Fixtures/reference_shot");
    for (what, p) in [
        ("as added", json!({"geometric_softness_1": 4, "choke_1": 75, "gray_level_softness_1": 10, "geometric_softness_2": 0, "choke_2": 0, "gray_level_softness_2": 100, "iterations": 1})),
        ("spread then choke, two iterations", json!({"geometric_softness_1": 3, "choke_1": -60, "gray_level_softness_1": 20, "geometric_softness_2": 3, "choke_2": 60, "gray_level_softness_2": 20, "iterations": 2})),
        ("a wide soft disc", json!({"geometric_softness_1": 12.5, "choke_1": 0, "gray_level_softness_1": 100, "geometric_softness_2": 1, "choke_2": 20, "gray_level_softness_2": 40, "iterations": 1})),
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
            &format!("the reference shot, Matte Choker {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(&mut gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Matte Choker {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
    let threshold = reference(&json!({"geometric_softness_1": 2, "choke_1": 10, "gray_level_softness_1": 0, "geometric_softness_2": 0, "choke_2": 0, "gray_level_softness_2": 100, "iterations": 1}));
    let card = on_card(&threshold, &ref_comp, &ref_root, 100, PreviewQuality::Full);
    t.row(
        "a Matte Choker stage with gray level softness 0 is a hard step, so it is left to the processor (D-122's reason)",
        &format!("{card} of 3 on the card"),
        card == 0,
    );

    t.heading("Pictures: the town keyed off its sky, in `verification/D-352 pictures/`");
    let dir = effect_table::repo("verification/D-352 pictures");
    fs::create_dir_all(&dir).unwrap();
    keyed_town(&dir);
    let (w, h) = TOWN;
    let draw = |effects: J| {
        let mut log = FrameLog::new(8);
        let b = render_frame(&keyed(effects), &Id::new(MAIN), 0, &dir, 64, &mut log).expect("the keyed town draws");
        (b.to_srgb8_straight(), log.finish().iter().map(|d| d.id.as_str().to_string()).collect::<Vec<_>>())
    };
    let base = town();
    // Over the 2-pixel band inside every edge, how far the colour is from the house's own, and
    // how covered it is; the specks left in the sky; the holes left in the houses, away from the
    // picture's border, which a choke takes in as it does any edge.
    let measure = |b: &[u8]| {
        let (mut spill, mut band, mut specks, mut holes) = (0.0, 0usize, 0usize, 0usize);
        for y in 0..h {
            for x in 0..w {
                let (p, q) = (&b[(y * w + x) * 4..][..4], &base[(y * w + x) * 4..][..4]);
                if solid(x, y) && near_edge(x, y, 2) && p[3] > 0 {
                    spill += (0..3).map(|i| (p[i] as f64 - q[i] as f64).abs()).sum::<f64>() / 3.0;
                    band += 1;
                } else if !solid(x, y) && !near_edge(x, y, 4) && p[3] > 0 {
                    specks += 1;
                } else if solid(x, y) && !near_edge(x, y, 4) && (8..w - 8).contains(&x) && y < h - 8 && p[3] < 255 {
                    holes += 1;
                }
            }
        }
        (spill / band.max(1) as f64, specks, holes)
    };
    let (before, said) = draw(json!([]));
    png_out::write_rgba(&dir.join("before.png"), w, h, OutputDepth::Eight, &[], &before).unwrap();
    let (spill0, specks0, holes0) = measure(&before);
    t.row(
        "before.png, the rough key: a soft, sky-tinted band round every house, specks in the sky, holes in the walls",
        &format!("the band {spill0:.1} levels from the houses' own colour; {specks0} speck pixels; {holes0} hole pixels; warnings {said:?}"),
        said.is_empty() && specks0 > 0 && holes0 > 0,
    );
    let (bytes, said) = draw(json!([choker()]));
    png_out::write_rgba(&dir.join("matte_choker.png"), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
    let (_, specks, holes) = measure(&bytes);
    t.row(
        "matte_choker.png, Matte Choker as added: the specks gone, the holes filled, the edge taken in and its corners rounded",
        &format!("{specks} speck pixels, {holes} hole pixels; warnings {said:?}"),
        said.is_empty() && specks == 0 && holes == 0,
    );
    for (file, what, e) in [
        ("refine_hard_matte.png", "Refine Hard Matte, decontaminate off: the edge softened along the houses' outlines and steepened", hard("off", "off")),
        ("refine_hard_matte_decontaminated.png", "Refine Hard Matte, decontaminate on: the sky's tint taken out of the edge, which is the houses' colour again", hard("on", "off")),
        ("refine_soft_matte.png", "Refine Soft Matte as added: the covering within 10 pixels of an edge fitted to the colours, and decontaminated. Here every pixel is within 10 of an edge, a speck or a hole, and the holes the key left keep no colour (black), so the fit takes dark for clear and the dark windows come out part see-through: a gap of D-352, not a fault of this check", soft(10.0, "off")),
    ] {
        let (bytes, said) = draw(json!([e]));
        png_out::write_rgba(&dir.join(file), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let (spill, specks, holes) = measure(&bytes);
        let decontaminated = file != "refine_hard_matte.png";
        t.row(
            &format!("{file}, {what}"),
            &format!("the band {spill:.1} levels from the houses' own colour (before {spill0:.1}); {specks} speck pixels; {holes} hole pixels; warnings {said:?}"),
            said.is_empty() && (!decontaminated || spill < spill0),
        );
    }
    for (file, what, e) in [
        ("refine_soft_edge_region.png", "Refine Soft Matte, edge radius 4, View Edge Region: white within 4 pixels of an edge (the houses' outlines, and round each speck and hole), black elsewhere", soft(4.0, "on")),
        ("decontamination_map.png", "Refine Hard Matte, View Decontamination Map: grey where the colour is decontaminated, brightest where half covered", hard("on", "on")),
    ] {
        let (bytes, said) = draw(json!([e]));
        png_out::write_rgba(&dir.join(file), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let opaque = bytes.chunks_exact(4).all(|p| p[3] == 255);
        let lit = bytes.chunks_exact(4).filter(|p| p[0] > 0).count();
        let dark = bytes.chunks_exact(4).filter(|p| p[0] == 0).count();
        t.row(
            &format!("{file}, {what}"),
            &format!("opaque {opaque}; {lit} pixels lit, {dark} black; warnings {said:?}"),
            said.is_empty() && opaque && lit > 0 && dark > 0,
        );
    }
    fs::remove_file(dir.join("keyed.png")).unwrap();

    t.finish("D-352_matte_refine_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-231's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole, with Draw on: GPU. The first loop starts
/// with empty caches and its 30 frames' median is "first"; then seven loops are timed, the
/// median of their 210 frames is "again". Matte Choker is drawn on the card; the two Refine
/// effects by the processor.
#[test]
#[ignore = "B-232: a measurement, run deliberately with --release --ignored"]
fn b232_matte_refine_timing() {
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
    let fx = |id: &str, type_id: &str, p: J| json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": p});
    let p = |e: J| (e["type_id"].as_str().unwrap().to_string(), e["parameters"].clone());
    let shots: [(&str, Option<(String, J)>); 5] = [
        ("Noise alone", None),
        ("Noise, then Matte Choker as added (card)", Some(p(choker()))),
        ("Noise, then Refine Hard Matte as added (processor)", Some(p(hard("off", "off")))),
        ("Noise, then Refine Hard Matte, decontaminate on (processor)", Some(p(hard("on", "off")))),
        ("Noise, then Refine Soft Matte as added (processor)", Some(p(soft(10.0, "off")))),
    ];
    for (name, e) in shots {
        let stack = |id: &str| {
            let mut v = vec![fx(&format!("{id}n"), "core.noise", json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some((type_id, p)) = &e {
                v.push(fx(&format!("{id}m"), type_id, p.clone()));
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
    let out = std::env::var("B232_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-232_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
