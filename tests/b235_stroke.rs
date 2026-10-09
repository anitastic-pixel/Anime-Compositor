//! B-235: D-356, Path Stroke, after After Effects' Stroke, the first effect drawn along a path
//! (P0-22 in `docs/effects/EFFECTS.md`).
//!
//! Every expected pixel is `Fixtures/stroke/expected_stroke.json`, written by
//! `tools/stroke_reference.py` before this code existed.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, set, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

/// `base` with one setting changed.
fn with(base: &Effect, change: impl Fn(&mut Effect)) -> Effect {
    let mut e = base.clone();
    change(&mut e);
    e
}

/// The picture's circle: centre, radius.
const CIRCLE: ((f64, f64), f64) = ((240.0, 135.0), 100.0);

/// A circle as four curved points, the way a mask tool draws one.
fn circle(((cx, cy), r): ((f64, f64), f64)) -> J {
    let k = 0.552_284_749_8 * r;
    json!({"points": [
        {"point": [cx, cy - r], "in": [-k, 0], "out": [k, 0]},
        {"point": [cx + r, cy], "in": [0, -k], "out": [0, k]},
        {"point": [cx, cy + r], "in": [k, 0], "out": [-k, 0]},
        {"point": [cx - r, cy], "in": [0, k], "out": [0, -k]},
    ]})
}

/// A setting keyed linearly from `a` at frame 0 to `b` at frame 24.
fn keyed(a: f64, b: f64) -> J {
    json!({"base": a, "keyframes": [{"frame": 0, "value": a, "interp": "linear"}, {"frame": 24, "value": b, "interp": "linear"}]})
}

/// The town with a circle mask of mode None carrying `parameters`' Path Stroke, at `frame`.
fn picture(dir: &Path, parameters: &J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let (w, h) = TOWN;
    let t = |v: J| json!({"base": v, "keyframes": []});
    let masks = json!([{"name": "Mask 1", "enabled": true, "inverted": false, "mode": "none", "opacity": 1.0,
        "feather_px": 0.0, "expansion_px": 0.0, "path": {"base": circle(CIRCLE), "keyframes": []}}]);
    let project = json!({
        "schema_version": 0, "project_id": "proj-b235-picture",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-town", "kind": "still", "name": "town", "path": "town.png",
            "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
            "id": MAIN, "name": "Main", "width": w, "height": h, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 25,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 25},
            "layer_order": ["town"],
            "layers": [{
                "id": "town", "kind": "raster", "name": "town", "asset_id": "asset-town", "enabled": true,
                "locked": false, "in_frame": 0, "out_frame": 25, "source_offset_frames": 0,
                "transform": {
                    "anchor": t(json!([w as f64 / 2.0, h as f64 / 2.0])), "position": t(json!([w as f64 / 2.0, h as f64 / 2.0])),
                    "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                },
                "exposure_spans": [],
                "masks": masks,
                "matte": null, "blend_mode": "normal",
                "effects": [{"instance_id": "fx-0-0", "type_id": "core.stroke", "enabled": true, "parameters": parameters}]
            }]
        }]
    });
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
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

/// The Path Strokes the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c, render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::Stroke { .. })))
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

/// The reference shot (1920 by 1080) with `stack` on its first three layers, each with three
/// circle masks of mode None to draw along.
fn reference(stack: impl Fn(&str) -> J) -> Project {
    let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        let masks: Vec<J> = [((960.0, 540.0), 400.0), ((560.0, 400.0), 220.0), ((1400.0, 620.0), 300.0)]
            .iter()
            .enumerate()
            .map(|(m, &((x, y), r))| json!({"name": format!("Mask {}", m + 1), "enabled": true, "inverted": false, "mode": "none", "opacity": 1.0,
                "feather_px": 0.0, "expansion_px": 0.0, "path": {"base": circle(((x + 40.0 * i as f64, y), r)), "keyframes": []}}))
            .collect();
        layers[i]["masks"] = J::Array(masks);
        layers[i]["effects"] = stack(id);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

/// Path Stroke's parameters: the effect's defaults as added with `changes` laid over them.
fn stroke_of(changes: J) -> J {
    let mut p = json!({"mask": 1, "all_masks": "off", "stroke_sequentially": "off", "color": "#ffffff", "brush_size": 2,
        "brush_hardness": 75, "opacity": 100, "start": 0, "end": 100, "spacing": 15, "paint_style": "on_original"});
    for (k, v) in changes.as_object().unwrap() {
        p[k] = v.clone();
    }
    p
}

/// Frames side by side, a white gap between.
fn strip(frames: &[Vec<u8>]) -> (usize, Vec<u8>) {
    let (w, h) = TOWN;
    let gap = 8;
    let wide = frames.len() * (w + gap) - gap;
    let mut out = vec![255u8; wide * h * 4];
    for (n, f) in frames.iter().enumerate() {
        for y in 0..h {
            let at = (y * wide + n * (w + gap)) * 4;
            out[at..at + w * 4].copy_from_slice(&f[y * w * 4..(y + 1) * w * 4]);
        }
    }
    (wide, out)
}

#[test]
fn b235_stroke() {
    let mut t = Table::new(
        "stroke",
        "# B-235: Path Stroke\n\nD-356, P0-22: effects that draw along paths, proved with Path \
         Stroke, after After Effects' Stroke (Generate). Every expected pixel is \
         `Fixtures/stroke/expected_stroke.json`, written by `tools/stroke_reference.py` before \
         this code existed and printed in document 25 as FX-STROKE-001 to 046. The build's frame \
         is compared sample by sample; the answer is the largest difference over all of them, \
         against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-STROKE-001 to 046 (document 25)");
    t.fixtures("expected_stroke.json");

    t.heading("The file");
    let files: Vec<String> = (1..=46).map(|n| format!("fx_stroke_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let effect_of = |d: &anime_compositor::command::Document| {
        d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone()
    };
    let saved = t.saved_parameters("fx_stroke_001.json");
    t.row("the paths compose finds are not saved", &saved.to_string(), saved.get("paths").is_none());
    for (file, said) in [
        ("fx_stroke_042.json", "Path Stroke's colour is written #rrggbb, and this is \"#12345\"."),
        ("fx_stroke_043.json", "Path Stroke's paint style is \"on_original\", \"on_transparent\" or \"reveal\", and this is \"paint\"."),
        ("fx_stroke_044.json", "Path Stroke's all masks is \"off\" or \"on\", and this is \"yes\"."),
        ("fx_stroke_045.json", "Path Stroke's stroke sequentially is \"off\" or \"on\", and this is \"maybe\"."),
    ] {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == said);
    }
    t.shape_refused("fx_stroke_001.json", "a brush size written as a word", r##"{"mask": 1, "all_masks": "off", "stroke_sequentially": "off", "color": "#ffffff", "brush_size": "3", "brush_hardness": 75, "opacity": 100, "start": 0, "end": 100, "spacing": 15, "paint_style": "on_original"}"##);
    t.shape_refused("fx_stroke_001.json", "no paint style", r##"{"mask": 1, "all_masks": "off", "stroke_sequentially": "off", "color": "#ffffff", "brush_size": 3, "brush_hardness": 75, "opacity": 100, "start": 0, "end": 100, "spacing": 15}"##);

    t.heading("Commands");
    let mut document = t.load("fx_stroke_001.json").document;
    let base = effect_of(&document);
    t.refused(
        &mut document,
        vec![
            ("brush size 201", set(with(&base, |e| if let Effect::Stroke { brush_size, .. } = e { *brush_size = 201.0 }))),
            ("paint style \"Reveal\"", set(with(&base, |e| if let Effect::Stroke { paint_style, .. } = e { *paint_style = "Reveal".into() }))),
            ("End keyed to 120", keys("end", &[(0, &[0.0]), (4, &[120.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_stroke_001.json",
        vec![
            ("End 60, On Transparent,", set(with(&base, |e| if let Effect::Stroke { end, paint_style, .. } = e { *end = 60.0; *paint_style = "on_transparent".into() }))),
            ("End keyed from 0 to 100,", keys("end", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_stroke_001.json", 0), ("fx_stroke_006.json", 0), ("fx_stroke_020.json", 2), ("fx_stroke_022.json", 0), ("fx_stroke_027.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    let fixture_root = effect_table::repo("Fixtures/stroke");
    for n in 1..=34 {
        let file = format!("fx_stroke_{n:03}.json");
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
        // FX-STROKE-031 to 034 have no path: the layer is left as it is, and the card not asked.
        let expect = if n >= 31 { 0 } else { 10 };
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames (expected {expect}); the same warnings: {agree}"),
            largest <= 1 && !refused && agree && card == expect,
        );
    }
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = effect_table::repo("Fixtures/reference_shot");
    for (what, p) in [
        ("as added (mask 1, white, Brush Size 2)", stroke_of(json!({}))),
        ("All Masks, Stroke Sequentially, End 70, orange, Brush Size 24, Hardness 30, Spacing 0", stroke_of(json!({"all_masks": "on", "stroke_sequentially": "on", "end": 70, "color": "#ff8800", "brush_size": 24, "brush_hardness": 30, "spacing": 0}))),
        ("mask 2, Start 20, End 90, Brush Size 40, Spacing 100, On Transparent", stroke_of(json!({"mask": 2, "start": 20, "end": 90, "brush_size": 40, "spacing": 100, "paint_style": "on_transparent"}))),
        ("All Masks, Brush Size 60, Hardness 0, Reveal Original Image", stroke_of(json!({"all_masks": "on", "brush_size": 60, "brush_hardness": 0, "paint_style": "reveal"}))),
    ] {
        let project = reference(|id| json!([{"instance_id": format!("b235-{id}"), "type_id": "core.stroke", "enabled": true, "parameters": p.clone()}]));
        let cpu = |project: &Project| {
            let mut log = FrameLog::new(3);
            preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
                .expect("the reference shot draws")
                .to_srgb8_straight()
        };
        let changed = distance(&cpu(&project), &cpu(&reference(|_| json!([])))).1;
        t.row(
            &format!("the reference shot, Path Stroke {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(&mut gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Path Stroke {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }

    t.heading("Pictures: a street with a circle mask, in `verification/D-356 pictures/`");
    let dir = effect_table::repo("verification/D-356 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    let street = town();
    png_out::write_rgba(&dir.join("town.png"), w, h, OutputDepth::Eight, &[], &street).unwrap();
    let ((cx, cy), r) = CIRCLE;
    // Pixels changed, and how many of them are further from the circle than the brush reaches.
    let changed = |b: &[u8], reach: f64| {
        let all: Vec<usize> = (0..w * h).filter(|&i| b[i * 4..][..4] != street[i * 4..][..4]).collect();
        let off = all.iter().filter(|&&i| (((i % w) as f64 + 0.5 - cx).hypot((i / w) as f64 + 0.5 - cy) - r).abs() > reach).count();
        (all.len(), off)
    };
    let stroke = |start: J, end: J, size: f64, hardness: f64, spacing: f64, style: &str| {
        json!({"mask": 1, "all_masks": "off", "stroke_sequentially": "off", "color": "#ff8800", "brush_size": size,
            "brush_hardness": hardness, "opacity": 100, "start": start, "end": end, "spacing": spacing, "paint_style": style})
    };
    for (name, what, p) in [
        ("write_on", "End keyed from 0 at frame 0 to 100 at frame 24: the line draws itself round the circle clockwise from the top", stroke(json!(0), keyed(0.0, 100.0), 8.0, 60.0, 15.0, "on_original")),
        ("travel", "Start keyed from 0 to 75 and End from 25 to 100: a quarter of the circle chases round it", stroke(keyed(0.0, 75.0), keyed(25.0, 100.0), 8.0, 60.0, 15.0, "on_original")),
    ] {
        let mut frames = Vec::new();
        let mut counts = Vec::new();
        let mut ok = true;
        for frame in [0, 6, 12, 18, 24] {
            let (bytes, said) = picture(&dir, &p, frame);
            let (all, off) = changed(&bytes, 5.0);
            ok &= said.is_empty() && off == 0;
            counts.push(all);
            png_out::write_rgba(&dir.join(format!("{name}_f{frame:02}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
            frames.push(bytes);
        }
        let (wide, bytes) = strip(&frames);
        png_out::write_rgba(&dir.join(format!("{name}_strip.png")), wide, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let shape = if name == "write_on" { counts[0] == 0 && counts.windows(2).all(|c| c[0] < c[1]) } else { counts.iter().all(|&c| c > 0) };
        t.row(
            &format!("{name}_strip.png (frames 0, 6, 12, 18 and 24, each also alone as {name}_fNN.png), {what}; nothing changes off the circle"),
            &format!("pixels changed per frame {counts:?}"),
            ok && shape,
        );
    }
    for (name, what, p, reach) in [
        ("beads", "Spacing 100, Hardness 0, Brush Size 16: soft round dabs a brush apart, a string of beads", stroke(json!(0), json!(100), 16.0, 0.0, 100.0, "on_original"), 9.0),
        ("reveal", "Reveal Original Image, Brush Size 30: the street shows only along the circle", stroke(json!(0), json!(100), 30.0, 50.0, 15.0, "reveal"), 16.0),
    ] {
        let (bytes, said) = picture(&dir, &p, 0);
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let (all, off) = changed(&bytes, reach);
        let pass = said.is_empty() && all > 0 && (name == "reveal" || off == 0);
        t.row(&format!("{name}.png, {what}"), &format!("{said:?}, {all} pixels changed, {off} off the circle"), pass);
    }

    t.finish("D-356_stroke_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-233's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole, with Draw on: GPU. The first loop starts
/// with empty caches and its 30 frames' median is "first"; then seven loops are timed, the
/// median of their 210 frames is "again". With `B235_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-235: a measurement, run deliberately with --release --ignored"]
fn b235_stroke_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B235_CPU").is_ok();
    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n- Drawn by: {}\n\n\
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
        ("Noise, then Path Stroke as added (mask 1, Brush Size 2)", Some(stroke_of(json!({})))),
        ("Noise, then Path Stroke, All Masks, Brush Size 24, Spacing 0", Some(stroke_of(json!({"all_masks": "on", "brush_size": 24, "spacing": 0})))),
        ("Noise, then Path Stroke, All Masks, Brush Size 60, Hardness 0, Spacing 100", Some(stroke_of(json!({"all_masks": "on", "brush_size": 60, "brush_hardness": 0, "spacing": 100})))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true, "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(json!({"instance_id": format!("{id}s"), "type_id": "core.stroke", "enabled": true, "parameters": p.clone()}));
            }
            J::Array(v)
        });
        let (comp, root) = (Id::new("comp-reference-shot"), effect_table::repo("Fixtures/reference_shot"));
        let mut cache = CelCache::viewer();
        gpu.forget();
        let (mut first, mut times) = (Vec::new(), Vec::new());
        for pass in 0..8 {
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
    let out = std::env::var("B235_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-235_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
