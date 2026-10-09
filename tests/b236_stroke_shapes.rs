//! B-236: D-357, Path Stroke along a shape layer's paths, open or closed (P0-22's second part
//! in `docs/effects/EFFECTS.md`).
//!
//! Every expected pixel is `Fixtures/stroke/expected_stroke_shapes.json`, written by
//! `tools/stroke_reference.py` before this code existed.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{set, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

/// A ten-cornered star about (`cx`, `cy`), its points `outer` out and its dips `inner` in,
/// clockwise from the top point.
fn star(cx: f64, cy: f64, outer: f64, inner: f64) -> Vec<(f64, f64)> {
    (0..10)
        .map(|k| {
            let r = if k % 2 == 0 { outer } else { inner };
            let a = (-90.0 + 36.0 * k as f64).to_radians();
            (cx + r * a.cos(), cy + r * a.sin())
        })
        .collect()
}

/// The picture's open path: a zigzag of four straight legs.
const ZIGZAG: [(f64, f64); 5] = [(60.0, 200.0), (160.0, 80.0), (260.0, 200.0), (360.0, 80.0), (420.0, 140.0)];

/// A shape of straight segments, no fill and no stroke of its own.
fn shape_of(points: &[(f64, f64)], closed: bool) -> J {
    let points: Vec<J> = points.iter().map(|&(x, y)| json!({"point": [x, y], "in": [0, 0], "out": [0, 0]})).collect();
    json!({"name": "Shape 1", "enabled": true, "closed": closed, "path": {"base": {"points": points}, "keyframes": []}, "fill": null, "stroke": null})
}

/// The distance from (`x`, `y`) to a path of straight segments.
fn path_distance(points: &[(f64, f64)], closed: bool, x: f64, y: f64) -> f64 {
    let n = points.len();
    (0..if closed { n } else { n - 1 })
        .map(|i| {
            let (a, b) = (points[i], points[(i + 1) % n]);
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let t = (((x - a.0) * dx + (y - a.1) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
            (x - a.0 - t * dx).hypot(y - a.1 - t * dy)
        })
        .fold(f64::INFINITY, f64::min)
}

/// A setting keyed linearly from `a` at frame 0 to `b` at frame 24.
fn keyed(a: f64, b: f64) -> J {
    json!({"base": a, "keyframes": [{"frame": 0, "value": a, "interp": "linear"}, {"frame": 24, "value": b, "interp": "linear"}]})
}

/// Path Stroke from shapes: the effect's defaults as added with `changes` laid over them.
fn stroke_of(changes: J) -> J {
    let mut p = json!({"mask": 1, "all_masks": "off", "stroke_sequentially": "off", "color": "#ffffff", "brush_size": 2,
        "brush_hardness": 75, "opacity": 100, "start": 0, "end": 100, "spacing": 15, "paint_style": "on_original", "source": "shapes"});
    for (k, v) in changes.as_object().unwrap() {
        p[k] = v.clone();
    }
    p
}

/// The town under a shape layer holding `shapes` and carrying `parameters`' Path Stroke.
fn picture_project(shapes: J, parameters: &J) -> Project {
    let (w, h) = TOWN;
    let t = |v: J| json!({"base": v, "keyframes": []});
    let transform = json!({
        "anchor": t(json!([w as f64 / 2.0, h as f64 / 2.0])), "position": t(json!([w as f64 / 2.0, h as f64 / 2.0])),
        "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
    });
    let project = json!({
        "schema_version": 0, "project_id": "proj-b236-picture",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-town", "kind": "still", "name": "town", "path": "town.png",
            "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
            "id": MAIN, "name": "Main", "width": w, "height": h, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 25,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 25},
            "layer_order": ["town", "art"],
            "layers": [{
                "id": "town", "kind": "raster", "name": "town", "asset_id": "asset-town", "enabled": true,
                "locked": false, "in_frame": 0, "out_frame": 25, "source_offset_frames": 0,
                "transform": transform, "exposure_spans": [], "masks": [],
                "matte": null, "blend_mode": "normal", "effects": []
            }, {
                "id": "art", "kind": "shape", "name": "art", "enabled": true, "locked": false,
                "in_frame": 0, "out_frame": 25, "shapes": shapes, "transform": transform, "masks": [],
                "matte": null, "blend_mode": "normal",
                "effects": [{"instance_id": "fx-0-0", "type_id": "core.stroke", "enabled": true, "parameters": parameters}]
            }]
        }]
    });
    persist::load_str(&project.to_string()).expect("the picture's project reads").document.project().clone()
}

/// The picture at `frame` on the processor, and what was said.
fn picture(dir: &Path, project: &Project, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut log = FrameLog::new(3);
    let drawn = render_frame(project, &Id::new(MAIN), frame, dir, 64, &mut log).expect("the picture draws");
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
fn b236_stroke_shapes() {
    let mut t = Table::new(
        "stroke",
        "# B-236: Path Stroke along shape paths\n\nD-357, P0-22's second part: Path Stroke's Path \
         From Shape Paths draws along a shape layer's own shapes, open or closed as each says. \
         Every expected pixel is `Fixtures/stroke/expected_stroke_shapes.json`, written by \
         `tools/stroke_reference.py` before this code existed and printed in document 25 as \
         FX-STROKE-047 to 061. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-STROKE-047 to 061 (document 25)");
    t.fixtures("expected_stroke_shapes.json");

    t.heading("The file");
    let files: Vec<String> = (47..=61).map(|n| format!("fx_stroke_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let effect_of = |d: &anime_compositor::command::Document| {
        d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone()
    };
    let saved = t.saved_parameters("fx_stroke_001.json");
    t.row("Path From Masks, the default, is not written: D-356's files save as they were", &saved.to_string(), saved.get("source").is_none());
    let saved = t.saved_parameters("fx_stroke_049.json");
    t.row("Path From Shape Paths is written \"shapes\"", &saved.to_string(), saved.get("source") == Some(&json!("shapes")));
    let why = effect_of(&t.load("fx_stroke_061.json").document).why_invalid();
    let said = "Path Stroke's source is \"masks\", \"shapes\" or \"text\", and this is \"layer\".";
    t.row("fx_stroke_061.json is refused in a sentence", &why, why == said);

    t.heading("Commands");
    let mut document = t.load("fx_stroke_049.json").document;
    let base = effect_of(&document);
    let with = |change: &dyn Fn(&mut Effect)| {
        let mut e = base.clone();
        change(&mut e);
        e
    };
    t.refused(&mut document, vec![("Path From \"layer\"", set(with(&|e| if let Effect::Stroke { source, .. } = e { *source = "layer".into() })))]);
    t.taken(
        &mut document,
        "fx_stroke_049.json",
        vec![("End 50, Path From Masks,", set(with(&|e| if let Effect::Stroke { end, source, .. } = e { *end = 50.0; *source = "masks".into() })))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_stroke_048.json", 0), ("fx_stroke_049.json", 0), ("fx_stroke_054.json", 2), ("fx_stroke_055.json", 0)]);

    // A shape layer has no drawing, and the card is handed effects on drawings only (B-46's
    // `cel.is_some()` in `compose::resolve_rest`), so every effect on a shape layer, Path Stroke
    // among them, is drawn by the processor in both paths; the card lays the layers. What is
    // checked is the frame the page receives with Draw on: GPU against the processor's.
    t.heading("Draw on: GPU against the processor: within 1 level of 255 (the stroke itself on the processor, as every effect on a shape layer is)");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    let fixture_root = effect_table::repo("Fixtures/stroke");
    for n in 47..=60 {
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
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; the stroke on the card in {card} of 10 frames (expected 0); the card refused a frame: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && card == 0,
        );
    }

    let dir = effect_table::repo("verification/D-357 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    let street = town();
    png_out::write_rgba(&dir.join("town.png"), w, h, OutputDepth::Eight, &[], &street).unwrap();
    let points = star(240.0, 135.0, 115.0, 48.0);
    let star_shape = json!([shape_of(&points, true)]);
    let write_on = stroke_of(json!({"end": keyed(0.0, 100.0), "color": "#ff8800", "brush_size": 8, "brush_hardness": 60}));
    let open = |closed: bool| json!([shape_of(&ZIGZAG, closed)]);
    let zig = stroke_of(json!({"color": "#ff8800", "brush_size": 8, "brush_hardness": 60}));
    for (what, project) in [
        ("the star written on (End keyed 0 to 100)", picture_project(star_shape.clone(), &write_on)),
        ("the open zigzag", picture_project(open(false), &zig)),
        ("the zigzag closed", picture_project(open(true), &zig)),
        ("the star, Brush Size 30, Hardness 0, Spacing 100, Reveal Original Image", picture_project(star_shape.clone(), &stroke_of(json!({"brush_size": 30, "brush_hardness": 0, "spacing": 100, "paint_style": "reveal"})))),
    ] {
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 12, 24] {
                let (d, r, a, b) = both(&mut gpu, &project, &comp, &dir, frame, quality);
                let card = on_card(&project, &comp, &dir, frame, quality);
                t.row(
                    &format!("the town under a shape layer, Path Stroke along {what}, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; the stroke on the card {card} (expected 0); warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 0,
                );
            }
        }
    }

    t.heading("Pictures: the town under a shape layer, in `verification/D-357 pictures/`");
    // Pixels changed, and how many of them are further from the path than the brush reaches.
    let changed = |b: &[u8], path: &[(f64, f64)], closed: bool, reach: f64| {
        let all: Vec<usize> = (0..w * h).filter(|&i| b[i * 4..][..4] != street[i * 4..][..4]).collect();
        let off = all.iter().filter(|&&i| path_distance(path, closed, (i % w) as f64 + 0.5, (i / w) as f64 + 0.5) > reach).count();
        (all.len(), off)
    };
    let project = picture_project(star_shape, &write_on);
    let (mut frames, mut counts, mut ok) = (Vec::new(), Vec::new(), true);
    for frame in [0, 6, 12, 18, 24] {
        let (bytes, said) = picture(&dir, &project, frame);
        let (all, off) = changed(&bytes, &points, true, 5.0);
        ok &= said.is_empty() && off == 0;
        counts.push(all);
        png_out::write_rgba(&dir.join(format!("star_f{frame:02}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        frames.push(bytes);
    }
    let (wide, bytes) = strip(&frames);
    png_out::write_rgba(&dir.join("star_strip.png"), wide, h, OutputDepth::Eight, &[], &bytes).unwrap();
    t.row(
        "star_strip.png (frames 0, 6, 12, 18 and 24, each also alone as star_fNN.png), a five-pointed star drawn as a shape, End keyed from 0 to 100: \
         the orange line draws itself round the star clockwise from the top point; nothing changes off the star",
        &format!("pixels changed per frame {counts:?}"),
        ok && counts[0] == 0 && counts.windows(2).all(|c| c[0] < c[1]),
    );
    let mut drawn = Vec::new();
    for (name, closed) in [("open", false), ("closed", true)] {
        let (bytes, said) = picture(&dir, &picture_project(open(closed), &zig), 0);
        png_out::write_rgba(&dir.join(format!("zigzag_{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let (all, off) = changed(&bytes, &ZIGZAG, closed, 5.0);
        t.row(
            &format!("zigzag_{name}.png, a zigzag of four legs drawn as a shape, {name}: nothing changes off its path"),
            &format!("{said:?}, {all} pixels changed, {off} off the path"),
            said.is_empty() && all > 0 && off == 0,
        );
        drawn.push((all, bytes));
    }
    // The closing leg, from (420, 140) back to (60, 200), passes (100, 193): open, it is not drawn.
    let at = (193 * w + 100) * 4;
    let (open_px, closed_px) = (&drawn[0].1[at..at + 4], &drawn[1].1[at..at + 4]);
    t.row(
        "open, the zigzag's ends are not joined: the pixel at (100, 193) on the closing leg is the street's, and the closed one draws it",
        &format!("open {open_px:?}, closed {closed_px:?}, street {:?}", &street[at..at + 4]),
        open_px == &street[at..at + 4] && closed_px != open_px && drawn[1].0 > drawn[0].0,
    );

    t.finish("D-357_stroke_shapes_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-235's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, and a shape layer on
/// top holding three circles (no fill, no stroke) with Path Stroke from its shapes, every eighth
/// frame asked for as the viewer asks, whole, with Draw on: GPU. The first loop's 30 frames'
/// median is "first"; then seven loops are timed, the median of their 210 frames is "again".
/// With `B236_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-236: a measurement, run deliberately with --release --ignored"]
fn b236_stroke_shapes_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B236_CPU").is_ok();
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
    let circle = |cx: f64, cy: f64, r: f64| {
        let k = 0.552_284_749_8 * r;
        json!({"name": "Circle", "enabled": true, "closed": true, "path": {"base": {"points": [
            {"point": [cx, cy - r], "in": [-k, 0], "out": [k, 0]},
            {"point": [cx + r, cy], "in": [0, -k], "out": [0, k]},
            {"point": [cx, cy + r], "in": [k, 0], "out": [-k, 0]},
            {"point": [cx - r, cy], "in": [0, k], "out": [0, -k]},
        ]}, "keyframes": []}, "fill": null, "stroke": null})
    };
    let shots: [(&str, Option<J>); 4] = [
        ("Noise alone, the shape layer without the effect", None),
        ("Noise, then Path Stroke from shapes as added (shape 1, Brush Size 2)", Some(stroke_of(json!({})))),
        ("Noise, then Path Stroke from shapes, All Masks, Brush Size 24, Spacing 0", Some(stroke_of(json!({"all_masks": "on", "brush_size": 24, "spacing": 0})))),
        ("Noise, then Path Stroke from shapes, All Masks, Brush Size 60, Hardness 0, Spacing 100", Some(stroke_of(json!({"all_masks": "on", "brush_size": 60, "brush_hardness": 0, "spacing": 100})))),
    ];
    let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json")).expect("read the reference shot");
    for (name, e) in shots {
        let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
        let c = &mut j["compositions"][0];
        let layers = c["layers"].as_array_mut().unwrap();
        for (i, layer) in layers.iter_mut().take(3).enumerate() {
            layer["effects"] = json!([{"instance_id": format!("n{i}"), "type_id": "core.noise", "enabled": true, "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}}]);
        }
        let mut shape = layers[0].clone();
        let obj = shape.as_object_mut().unwrap();
        for k in ["asset_id", "source_offset_frames", "exposure_spans"] {
            obj.remove(k);
        }
        obj.insert("id".into(), json!("paths"));
        obj.insert("name".into(), json!("paths"));
        obj.insert("kind".into(), json!("shape"));
        obj.insert("masks".into(), json!([]));
        obj.insert("shapes".into(), json!([circle(960.0, 540.0, 400.0), circle(560.0, 400.0, 220.0), circle(1400.0, 620.0, 300.0)]));
        obj.insert("effects".into(), e.map_or(json!([]), |p| json!([{"instance_id": "s", "type_id": "core.stroke", "enabled": true, "parameters": p}])));
        layers.push(shape);
        c["layer_order"].as_array_mut().unwrap().push(json!("paths"));
        let project = persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone();
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
    let out = std::env::var("B236_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-236_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
