//! B-424: D-544, pick #13 (EFFECTS.md row 260), Halation: `core.halation`, a small effect of its
//! own on the shared soft-glow engine (D-353), as PLUGINS.md's ranked list settles.
//!
//! Every expected pixel is `Fixtures/halation/expected_halation.json`, written by
//! `tools/halation_reference.py` before this code existed. D-353's, D-542's and D-543's fixtures
//! are run again, unchanged, since Halation shares their code. Halation is drawn on the card too,
//! and checked against the processor within 1 level of 255 (ADR-006, D-100).

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, median, said, set, time_passes, town, Table, MAIN, TOWN};
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

/// Halation's parameters: the effect as added with `changes` laid over them.
fn halation(changes: J) -> J {
    let mut p = json!({"threshold": 60, "threshold_smooth": 50, "radius": 25, "amount": 100, "color": "#ff4020"});
    for (k, v) in changes.as_object().unwrap() {
        p[k] = v.clone();
    }
    p
}

/// The reference shot with a Halation of `parameters` on its first three layers.
fn reference(parameters: &J) -> Project {
    let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        layers[i]["effects"] = json!([{"instance_id": format!("b424-{id}"), "type_id": "core.halation", "enabled": true, "parameters": parameters}]);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c, render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::Halation { .. })))
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
        "schema_version": 0, "project_id": "proj-b424",
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
fn b424_halation() {
    let mut t = Table::new(
        "soft_glow",
        "# B-424: Halation (pick #13)\n\n\
         D-544 (EFFECTS.md row 260): a new effect, Halation (`core.halation`), on Soft Physical \
         Glow's engine: the light above Threshold spreads by Radius, and only where it spreads \
         onto something darker than itself does it add a halo, in Halation's colour (red-orange \
         as added) at Amount, as film's halation hugs bright edges and leaves the highlights \
         alone. Soft Physical Glow shares the code, so D-353's 45 fixtures, D-542's 22 and \
         D-543's 20 are run again here unchanged. Every expected pixel is \
         `Fixtures/halation/expected_halation.json`, written by `tools/halation_reference.py` \
         before this code existed (FX-HALO-001 to 018), compared sample by sample against the \
         catalogue's tolerance of 2e-5. Halation is drawn on the card too: the card's picture \
         against the processor's, within 1 level of 255 (ADR-006, D-100).\n",
    );

    t.heading("D-353's FX-SGLOW-001 to 045, unchanged: Soft Physical Glow draws as before");
    t.fixtures("expected_soft_glow.json");
    t.root = effect_table::repo("Fixtures/glow_tint");
    t.heading("D-542's FX-GTINT-001 to 022, unchanged");
    t.fixtures("expected_glow_tint.json");
    t.root = effect_table::repo("Fixtures/spectral_glow");
    t.heading("D-543's FX-SPGLOW-001 to 020, unchanged");
    t.fixtures("expected_spectral_glow.json");

    t.root = effect_table::repo("Fixtures/halation");
    t.heading("FX-HALO-001 to 018");
    t.fixtures("expected_halation.json");

    t.heading("The file");
    let files: Vec<String> = (1..=18).map(|n| format!("fx_halo_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    t.shape_refused("fx_halo_001.json", "a radius written as a word", &halation(json!({"radius": "wide"})).to_string());
    t.shape_refused("fx_halo_001.json", "a colour written as a number", &halation(json!({"color": 5})).to_string());
    t.shape_refused("fx_halo_001.json", "no amount", &json!({"threshold": 60, "threshold_smooth": 50, "radius": 25, "color": "#ff4020"}).to_string());
    for (file, said) in [
        ("fx_halo_015.json", "Halation's threshold runs from 0 to 100, and this is 101."),
        ("fx_halo_016.json", "Halation's radius runs from 0 to 2000, and this is -1."),
        ("fx_halo_017.json", "Halation's colour is written #rrggbb, and this is \"orange\"."),
    ] {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming it"), &why, why == said);
    }

    t.heading("Commands");
    let mut document = t.load("fx_halo_001.json").document;
    let base = effect_of(&document);
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = base.clone();
        f(&mut e);
        e
    };
    t.refused(
        &mut document,
        vec![
            ("threshold 101", set(with(&|e| if let Effect::Halation { threshold, .. } = e { *threshold = 101.0 }))),
            ("amount 401", set(with(&|e| if let Effect::Halation { amount, .. } = e { *amount = 401.0 }))),
            ("colour \"orange\"", set(with(&|e| if let Effect::Halation { color, .. } = e { *color = "orange".into() }))),
        ],
    );
    t.taken(
        &mut document,
        "fx_halo_001.json",
        vec![
            ("radius 60, amount 300, colour #ffb040", set(with(&|e| if let Effect::Halation { radius, amount, color, .. } = e {
                *radius = 60.0;
                *amount = 300.0;
                *color = "#ffb040".into();
            }))),
            ("amount keyed 0 to 200 over frames 0 to 4", keys("amount", &[(0, &[0.0]), (4, &[200.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_halo_001.json", 0), ("fx_halo_004.json", 0), ("fx_halo_010.json", 0), ("fx_halo_011.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    let fixture_root = effect_table::repo("Fixtures/halation");
    for n in 1..=14 {
        let file = format!("fx_halo_{n:03}.json");
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
        // FX-HALO-012's threshold is a step, which stays on the processor (D-122's reason).
        let wanted = if n == 12 { 0 } else { 10 };
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && card == wanted,
        );
    }
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = effect_table::repo("Fixtures/reference_shot");
    for (what, p) in [
        ("as added (threshold 60, smooth 50, radius 25, amount 100, red-orange)", halation(json!({}))),
        ("threshold 30, smooth 20, radius 150, amount 250, colour #ffb040", halation(json!({"threshold": 30, "threshold_smooth": 20, "radius": 150, "amount": 250, "color": "#ffb040"}))),
    ] {
        let project = reference(&p);
        let mut none = p.clone();
        none["amount"] = json!(0);
        let plain = reference(&none);
        let cpu = |project: &Project| {
            let mut log = FrameLog::new(3);
            preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
                .expect("the reference shot draws")
                .to_srgb8_straight()
        };
        let changed = distance(&cpu(&project), &cpu(&plain)).1;
        t.row(
            &format!("the reference shot, Halation {what}: the processor's frame 100 differs from the same at amount 0, so the comparisons below test the halo"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(&mut gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Halation {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }

    t.heading("Pictures: the town, set in a larger frame, in `verification/D-544 pictures/`");
    let dir = effect_table::repo("verification/D-544 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    png_out::write_rgba(&dir.join("town.png"), w, h, OutputDepth::Eight, &[], &town()).unwrap();
    let (fw, fh) = (w + 160, h + 160);
    let draw = |effects: J| {
        let mut log = FrameLog::new(8);
        let b = render_frame(&framed(effects), &Id::new(MAIN), 0, &dir, 64, &mut log).expect("the town draws");
        (b.to_srgb8_straight(), log.finish().iter().map(|d| d.id.as_str().to_string()).collect::<Vec<_>>())
    };
    let fx = |p: J| json!([{"instance_id": "fx-0-0", "type_id": "core.halation", "enabled": true, "parameters": p}]);
    // The halo 10 pixels out from the town's left edge, in the frame's middle row.
    let tail = |b: &[u8]| {
        let i = ((fh / 2) * fw + 70) * 4;
        [b[i], b[i + 1], b[i + 2], b[i + 3]]
    };
    for (file, what, p) in [
        ("halation_added.png", "Halation as added (threshold 60, smooth 50, radius 25, amount 100, red-orange): a red-orange halo outside the town's bright edges", halation(json!({}))),
        ("halation_wide.png", "radius 60, amount 300: wider and stronger", halation(json!({"radius": 60, "amount": 300}))),
        ("halation_white.png", "the same, colour white: the halo without colour, for comparison", halation(json!({"radius": 60, "amount": 300, "color": "#ffffff"}))),
        ("halation_low.png", "threshold 20, radius 60, amount 200: more of the town makes it", halation(json!({"threshold": 20, "radius": 60, "amount": 200}))),
    ] {
        let (bytes, said) = draw(fx(p));
        png_out::write_rgba(&dir.join(file), fw, fh, OutputDepth::Eight, &[], &bytes).unwrap();
        let px = tail(&bytes);
        let ok = said.is_empty()
            && px[3] > 0
            && match file {
                "halation_white.png" => px[0].abs_diff(px[1]) <= 1 && px[1].abs_diff(px[2]) <= 1,
                _ => px[0] > px[1] && px[1] > px[2],
            };
        t.row(&format!("{file}, {what}"), &format!("the halo 10 pixels out, red {} green {} blue {} covering {}; warnings {said:?}", px[0], px[1], px[2], px[3]), ok);
    }
    fs::remove_file(dir.join("town.png")).unwrap();

    t.finish("D-544_halation_table.md");
}

/// The frame times, B-233's way: the reference shot with a changing Noise on its first three
/// layers, then Halation, every eighth frame asked for as the viewer asks, with Draw on: GPU.
#[test]
#[ignore = "B-424: a measurement, run deliberately with --release --ignored"]
fn b424_halation_timing() {
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
    let fx = |id: &str, p: J| json!({"instance_id": id, "type_id": "core.halation", "enabled": true, "parameters": p});
    let shots: [(&str, Option<J>); 3] = [
        ("Noise alone", None),
        ("Noise, then Halation as added (radius 25)", Some(halation(json!({})))),
        ("Noise, then Halation, radius 200, amount 200", Some(halation(json!({"radius": 200, "amount": 200})))),
    ];
    for (name, e) in shots {
        let stack = |id: &str| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true, "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}h"), p.clone()));
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
        let (first, times) = time_passes(&mut gpu, false, 8, &project, &comp, &root, PreviewQuality::Full);
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B424_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-424_halation_timing_table.md"));
    fs::write(out, s).expect("write the timing table");
}
