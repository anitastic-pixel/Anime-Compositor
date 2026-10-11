//! B-422: D-542, pick #6 (EFFECTS.md row 258), Glow core-to-edge colour: Soft Physical Glow's
//! Tint, Inner Colour, Outer Colour and Tint Amount, on the shared soft-glow engine (D-353).
//!
//! Every expected pixel is `Fixtures/glow_tint/expected_glow_tint.json`, written by
//! `tools/glow_tint_reference.py` before this code existed. D-353's own fixtures are run again,
//! unchanged, so a glow with no tint draws as it always did. The tint is drawn on the card too,
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
        layers[i]["effects"] = json!([{"instance_id": format!("b422-{id}"), "type_id": "core.glow", "enabled": true, "parameters": parameters}]);
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
        "schema_version": 0, "project_id": "proj-b422",
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
fn b422_glow_tint() {
    let mut t = Table::new(
        "soft_glow",
        "# B-422: Soft Physical Glow's tint (pick #6, Glow core-to-edge colour)\n\n\
         D-542 (EFFECTS.md row 258): four new settings of Soft Physical Glow, after Deep Glow's \
         Tint Mode, Color Inner, Color Outer and Tint Strength (PLUGINS.md 2.3). Tint One Colour \
         colours the whole glow; Core to Edge colours it the inner colour close to the light and \
         the outer colour out in the tail, level by level of the spread; either way a grey light \
         glows the colour as a coloured one does. A glow with Tint Off, or a file from before, \
         draws exactly as D-353 drew it: its 45 fixtures are run again here unchanged. Every \
         expected pixel is `Fixtures/glow_tint/expected_glow_tint.json`, written by \
         `tools/glow_tint_reference.py` before this code existed (FX-GTINT-001 to 022), compared \
         sample by sample against the catalogue's tolerance of 2e-5. The tint is drawn on the card \
         too: the card's picture against the processor's, within 1 level of 255 (ADR-006, D-100).\n",
    );

    t.heading("D-353's FX-SGLOW-001 to 045, unchanged: no tint draws as before");
    t.fixtures("expected_soft_glow.json");
    let files: Vec<String> = (1..=45).map(|n| format!("fx_sglow_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let params = t.saved_parameters("fx_sglow_001.json");
    t.row(
        "fx_sglow_001.json, a glow from before the tint, is saved with none of the tint's settings",
        &params.to_string(),
        ["tint", "inner_color", "outer_color", "tint_amount"].iter().all(|k| params.get(*k).is_none()),
    );

    t.root = effect_table::repo("Fixtures/glow_tint");
    t.heading("FX-GTINT-001 to 022");
    t.fixtures("expected_glow_tint.json");

    t.heading("The file");
    let files: Vec<String> = (1..=22).map(|n| format!("fx_gtint_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    t.shape_refused("fx_gtint_002.json", "a tint amount written as a word", &physical(json!({"tint": "core_to_edge", "tint_amount": "full"})).to_string());
    t.shape_refused("fx_gtint_002.json", "a tint written as a number", &physical(json!({"tint": 2})).to_string());
    for (file, said) in [
        ("fx_gtint_017.json", "Soft Physical Glow's tint is \"off\", \"one_color\" or \"core_to_edge\", and this is \"duocolor\"."),
        ("fx_gtint_018.json", "Soft Physical Glow's inner colour is written #rrggbb, and this is \"#12345\"."),
        ("fx_gtint_019.json", "Soft Physical Glow's outer colour is written #rrggbb, and this is \"blue\"."),
        ("fx_gtint_020.json", "Soft Physical Glow's tint amount runs from 0 to 100, and this is 101."),
        ("fx_gtint_021.json", "Soft Physical Glow's tint amount runs from 0 to 100, and this is -1."),
    ] {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming it"), &why, why == said);
    }

    t.heading("Commands");
    let mut document = t.load("fx_gtint_002.json").document;
    let base = effect_of(&document);
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = base.clone();
        f(&mut e);
        e
    };
    t.refused(
        &mut document,
        vec![
            ("tint \"duocolor\"", set(with(&|e| if let Effect::SoftGlow { tint, .. } = e { *tint = "duocolor".into() }))),
            ("inner colour \"#12\"", set(with(&|e| if let Effect::SoftGlow { inner_color, .. } = e { *inner_color = "#12".into() }))),
            ("tint amount 101", set(with(&|e| if let Effect::SoftGlow { tint_amount, .. } = e { *tint_amount = 101.0 }))),
        ],
    );
    t.taken(
        &mut document,
        "fx_gtint_002.json",
        vec![
            ("one colour, orange, tint amount 60", set(with(&|e| if let Effect::SoftGlow { tint, inner_color, tint_amount, .. } = e {
                *tint = "one_color".into();
                *inner_color = "#ff8020".into();
                *tint_amount = 60.0;
            }))),
            ("tint amount keyed 0 to 100 over frames 0 to 4", keys("tint_amount", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_gtint_002.json", 0), ("fx_gtint_003.json", 0), ("fx_gtint_014.json", 0), ("fx_gtint_016.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    let fixture_root = effect_table::repo("Fixtures/glow_tint");
    for n in 1..=16 {
        let file = format!("fx_gtint_{n:03}.json");
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
        // FX-GTINT-008 has a threshold with no smooth: a hard step, left to the processor.
        let expect = if n == 8 { 0 } else { 10 };
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames (expected {expect}); the same warnings: {agree}"),
            largest <= 1 && !refused && agree && card == expect,
        );
    }
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = effect_table::repo("Fixtures/reference_shot");
    for (what, p) in [
        ("Core to Edge as added (radius 500, white to blue)", physical(json!({"tint": "core_to_edge"}))),
        ("One Colour orange at 70 %, radius 60, threshold 40 with smooth 50", physical(json!({"tint": "one_color", "inner_color": "#ff8020", "tint_amount": 70, "radius": 60, "threshold": 40, "threshold_smooth": 50}))),
        ("Core to Edge orange to purple, radius 150, aspect 1.6 at 25 degrees, add, unmult off", physical(json!({"tint": "core_to_edge", "inner_color": "#ff8020", "outer_color": "#8020ff", "radius": 150, "aspect_ratio": 1.6, "aspect_angle": 25, "operation": "add", "unmult": "off"}))),
    ] {
        let project = reference(&p);
        let untinted = reference(&physical(json!({}))
            .as_object()
            .map(|o| {
                let mut o = o.clone();
                for (k, v) in p.as_object().unwrap() {
                    if !["tint", "inner_color", "outer_color", "tint_amount"].contains(&k.as_str()) {
                        o.insert(k.clone(), v.clone());
                    }
                }
                J::Object(o)
            })
            .unwrap());
        let cpu = |project: &Project| {
            let mut log = FrameLog::new(3);
            preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
                .expect("the reference shot draws")
                .to_srgb8_straight()
        };
        let changed = distance(&cpu(&project), &cpu(&untinted)).1;
        t.row(
            &format!("the reference shot, Soft Physical Glow {what}: the processor's frame 100 differs from the same glow untinted, so the comparisons below test the tint"),
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

    t.heading("Pictures: the town, set in a larger frame, in `verification/D-542 pictures/`");
    let dir = effect_table::repo("verification/D-542 pictures");
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
    // against its red says which colour the tail is.
    let tail = |b: &[u8]| {
        let i = ((fh / 2) * fw + 70) * 4;
        [b[i], b[i + 1], b[i + 2], b[i + 3]]
    };
    let lit = json!({"threshold": 60, "threshold_smooth": 50, "radius": 60});
    let with_lit = |extra: J| {
        let mut p = physical(lit.clone());
        for (k, v) in extra.as_object().unwrap() {
            p[k] = v.clone();
        }
        p
    };
    for (file, what, p, check) in [
        ("untinted.png", "Soft Physical Glow, threshold 60 with smooth 50, radius 60, no tint, for comparison: the glow takes the town's own warm colours", with_lit(json!({})), 0),
        ("one_colour_orange.png", "the same, Tint One Colour orange: the whole glow orange, the sky's pale light included", with_lit(json!({"tint": "one_color", "inner_color": "#ff8020"})), 1),
        ("core_to_edge.png", "the same, Tint Core to Edge as added: white close to the bright parts, blue in the tail", with_lit(json!({"tint": "core_to_edge"})), 2),
        ("core_to_edge_wide.png", "Tint Core to Edge, orange core to purple edge, radius 200: a long purple tail round an orange core", with_lit(json!({"tint": "core_to_edge", "inner_color": "#ff8020", "outer_color": "#8020ff", "radius": 200})), 3),
    ] {
        let (bytes, said) = draw(fx(p));
        png_out::write_rgba(&dir.join(file), fw, fh, OutputDepth::Eight, &[], &bytes).unwrap();
        let px = tail(&bytes);
        let ok = said.is_empty()
            && px[3] > 0
            && match check {
                1 => px[0] > px[2],
                2 => px[2] > px[0],
                3 => px[2] > px[1],
                _ => true,
            };
        t.row(&format!("{file}, {what}"), &format!("the tail 10 pixels out, red {} green {} blue {} covering {}; warnings {said:?}", px[0], px[1], px[2], px[3]), ok);
    }
    fs::remove_file(dir.join("town.png")).unwrap();

    t.finish("D-542_glow_tint_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-233's way: the reference shot with a changing Noise on its first three
/// layers, then the glow, every eighth frame asked for as the viewer asks, with Draw on: GPU.
#[test]
#[ignore = "B-422: a measurement, run deliberately with --release --ignored"]
fn b422_glow_tint_timing() {
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
        ("Noise, then Soft Physical Glow, radius 60, threshold 40 smooth 50, no tint", Some(physical(json!({"radius": 60, "threshold": 40, "threshold_smooth": 50})))),
        ("Noise, then the same, Tint Core to Edge", Some(physical(json!({"radius": 60, "threshold": 40, "threshold_smooth": 50, "tint": "core_to_edge"})))),
        ("Noise, then Soft Physical Glow as added, radius 500, Tint Core to Edge", Some(physical(json!({"tint": "core_to_edge"})))),
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
    let out = std::env::var("B422_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-422_glow_tint_timing_table.md"));
    fs::write(out, s).expect("write the timing table");
}
