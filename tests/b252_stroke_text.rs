//! B-252: D-373, Path Stroke along a text layer's letter outlines (P0-22's third part in
//! `docs/effects/EFFECTS.md`).
//!
//! Every expected pixel and outline number is `Fixtures/stroke/expected_stroke_text.json`,
//! written by `tools/stroke_text_reference.py` before this code existed.

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

/// The length of a closed path of straight pieces.
fn closed_length(path: &[(f64, f64)]) -> f64 {
    (0..path.len()).map(|i| {
        let (a, b) = (path[i], path[(i + 1) % path.len()]);
        (b.0 - a.0).hypot(b.1 - a.1)
    }).sum()
}

/// The distance from (`x`, `y`) to the nearest of closed paths of straight pieces.
fn paths_distance(paths: &[Vec<(f64, f64)>], x: f64, y: f64) -> f64 {
    paths
        .iter()
        .flat_map(|p| (0..p.len()).map(move |i| (p[i], p[(i + 1) % p.len()])))
        .map(|(a, b)| {
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let l2 = dx * dx + dy * dy;
            let t = if l2 == 0.0 { 0.0 } else { (((x - a.0) * dx + (y - a.1) * dy) / l2).clamp(0.0, 1.0) };
            (x - a.0 - t * dx).hypot(y - a.1 - t * dy)
        })
        .fold(f64::INFINITY, f64::min)
}

/// Path Stroke from text outlines: the effect's defaults as added with `changes` laid over them.
fn stroke_of(changes: J) -> J {
    let mut p = json!({"mask": 1, "all_masks": "on", "stroke_sequentially": "on", "color": "#ffffff", "brush_size": 2,
        "brush_hardness": 75, "opacity": 100, "start": 0, "end": 100, "spacing": 15, "paint_style": "on_original", "source": "text"});
    for (k, v) in changes.as_object().unwrap() {
        p[k] = v.clone();
    }
    p
}

const WORD: &str = "Write";

/// The town under a text layer of `WORD` carrying `parameters`' Path Stroke, or none.
fn picture_project(parameters: Option<&J>) -> Project {
    let (w, h) = TOWN;
    let t = |v: J| json!({"base": v, "keyframes": []});
    let transform = json!({
        "anchor": t(json!([w as f64 / 2.0, h as f64 / 2.0])), "position": t(json!([w as f64 / 2.0, h as f64 / 2.0])),
        "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
    });
    let project = json!({
        "schema_version": 0, "project_id": "proj-b252-picture",
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
                "id": "art", "kind": "text", "name": "art", "enabled": true, "locked": false,
                "in_frame": 0, "out_frame": 25, "transform": transform, "masks": [],
                "matte": null, "blend_mode": "normal",
                "effects": parameters.map_or(json!([]), |p| json!([{"instance_id": "fx-0-0", "type_id": "core.stroke", "enabled": true, "parameters": p}])),
                "source_text": {"text": WORD, "font": "MPLUSRounded1c-Regular.ttf", "size": 150, "color": [1, 1, 1], "at": [40, 190], "align": "left"}
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
fn b252_stroke_text() {
    let mut t = Table::new(
        "stroke",
        "# B-252: Path Stroke along text outlines\n\nD-373, P0-22's third part: Path Stroke's Path \
         From Text Outlines draws along a text layer's letters, every outline closed, glyph by \
         glyph in reading order, text animators included. Every expected pixel and outline number \
         is `Fixtures/stroke/expected_stroke_text.json`, written by `tools/stroke_text_reference.py` \
         before this code existed and printed in document 25 as FX-STROKE-062 to 069. The build's \
         frame is compared sample by sample; the answer is the largest difference over all of them, \
         against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-STROKE-062 to 069 (document 25)");
    t.fixtures("expected_stroke_text.json");
    // FX-STROKE-069's layer is drawn as it is, letters filled: the same frame as with the effect off.
    let mut off = t.load("fx_stroke_069.json").document;
    let mut base = off.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].clone();
    let with_effect = t.render(&off, 0, 64);
    base.enabled = false;
    let _ = off.apply(anime_compositor::command::Command::SetEffectEnabled {
        composition: Id::new(MAIN),
        layer_id: Id::new("art"),
        instance_id: base.instance_id.clone(),
        enabled: false,
    });
    let without = t.render(&off, 0, 64);
    t.row(
        "FX-STROKE-069 frame 0 (Path 9 of four outlines) is the frame with the effect switched off",
        if with_effect.data() == without.data() { "byte-identical" } else { "differ" },
        with_effect.data() == without.data() && without.data().iter().any(|v| *v > 0.0),
    );

    t.heading("The outlines: \"Yes\" in M PLUS Rounded 1c, size 24, against the reference");
    let expected: J = serde_json::from_str(&fs::read_to_string(effect_table::repo("Fixtures/stroke/expected_stroke_text.json")).unwrap()).unwrap();
    let o = &expected["outlines"];
    let tolerance = o["length_tolerance"].as_f64().unwrap();
    let yes = t.load("fx_stroke_062.json").document;
    let words = yes.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().text.clone().expect("a text layer");
    let outlines = anime_compositor::text::outlines(&words, &[]).expect("the bundled font");
    let count = o["count"].as_u64().unwrap() as usize;
    t.row(&format!("{count} outlines (Y; e's eye, then its edge, as the font stores them; s)"), &outlines.len().to_string(), outlines.len() == count);
    let points: Vec<usize> = outlines.iter().map(Vec::len).collect();
    let want_points: Vec<usize> = o["points"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as usize).collect();
    t.row("each outline cut into as many points as the reference's", &format!("{points:?} (reference {want_points:?})"), points == want_points);
    let lengths: Vec<f64> = outlines.iter().map(|p| closed_length(p)).collect();
    let want: Vec<f64> = o["lengths"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
    let worst = lengths.iter().zip(&want).map(|(a, b)| (a - b).abs()).fold(0.0, f64::max);
    t.row(
        "each outline's length, closed, in pixels",
        &format!("{:?}, largest difference {worst:.1e}", lengths.iter().map(|l| format!("{l:.4}")).collect::<Vec<_>>()),
        lengths.len() == want.len() && worst <= tolerance,
    );
    let total: f64 = lengths.iter().sum();
    let want_total = o["total"].as_f64().unwrap();
    t.row("the total length", &format!("{total:.6} (reference {want_total:.6})"), (total - want_total).abs() <= tolerance);
    let starts: Vec<(f64, f64)> = o["starts"].as_array().unwrap().iter().map(|p| (p[0].as_f64().unwrap(), p[1].as_f64().unwrap())).collect();
    let worst = outlines.iter().zip(&starts).map(|(p, s)| (p[0].0 - s.0).abs().max((p[0].1 - s.1).abs())).fold(0.0, f64::max);
    t.row("each outline begins where the reference's does (D-372's slide included)", &format!("largest difference {worst:.1e}"), worst <= tolerance);
    // Stroke Sequentially, End 50: each outline's stretch, worked from the build's lengths.
    let mut g = 0.0;
    let mut half = Vec::new();
    for l in &lengths {
        let (lo, hi) = ((0.0f64 - g).max(0.0), (0.5 * total - g).min(*l));
        half.push((lo < hi).then_some((lo, hi)));
        g += l;
    }
    let want_half: Vec<Option<(f64, f64)>> = o["sequential_end_50"].as_array().unwrap().iter().map(|w| w.as_array().map(|w| (w[0].as_f64().unwrap(), w[1].as_f64().unwrap()))).collect();
    let agree = half.len() == want_half.len()
        && half.iter().zip(&want_half).all(|(a, b)| match (a, b) {
            (Some(a), Some(b)) => (a.0 - b.0).abs() <= tolerance && (a.1 - b.1).abs() <= tolerance,
            (None, None) => true,
            _ => false,
        });
    t.row("Stroke Sequentially, End 50: what each outline draws (from, to), the frame is FX-STROKE-064 frame 2", &format!("{half:.3?}"), agree);

    t.heading("The file");
    let files: Vec<String> = (62..=69).map(|n| format!("fx_stroke_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_stroke_062.json");
    t.row("Path From Text Outlines is written \"text\"", &saved.to_string(), saved.get("source") == Some(&json!("text")));
    let saved = t.saved_parameters("fx_stroke_064.json");
    t.row("End keyed from 0 to 100 is saved with its keys", &saved["end"].to_string(), saved["end"]["keyframes"].as_array().is_some_and(|k| k.len() == 2));

    t.heading("Commands");
    let mut document = t.load("fx_stroke_062.json").document;
    let base = document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    let with = |change: &dyn Fn(&mut Effect)| {
        let mut e = base.clone();
        change(&mut e);
        e
    };
    t.taken(
        &mut document,
        "fx_stroke_062.json",
        vec![
            ("Path 3, All Masks off,", set(with(&|e| if let Effect::Stroke { mask, all_masks, .. } = e { *mask = 3.0; *all_masks = "off".into() }))),
            ("End keyed from 0 at frame 0 to 100 at frame 4", keys("end", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_stroke_062.json", 0), ("fx_stroke_064.json", 2), ("fx_stroke_066.json", 0)]);

    // A text layer's first step is its picture with no cel (`(picture, None)` in
    // `compose::resolve_held`), and the card is handed effects on cels only (B-46's
    // `cel.is_some()` in `compose::resolve_rest`), so Path Stroke on a text layer is drawn by
    // the processor in both paths; the card lays the layers.
    t.heading("Draw on: GPU against the processor: within 1 level of 255 (the stroke itself on the processor, as every effect on a text layer is)");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    let fixture_root = effect_table::repo("Fixtures/stroke");
    for n in 62..=69 {
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

    let dir = effect_table::repo("verification/D-373 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    let street = town();
    png_out::write_rgba(&dir.join("town.png"), w, h, OutputDepth::Eight, &[], &street).unwrap();
    let write_on = stroke_of(json!({"end": {"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"}, {"frame": 24, "value": 100, "interp": "linear"}]},
        "color": "#ff8800", "brush_size": 6, "brush_hardness": 60}));
    let reveal = stroke_of(json!({"brush_size": 24, "brush_hardness": 0, "spacing": 0, "paint_style": "reveal", "stroke_sequentially": "off"}));
    for (what, project) in [("written on (End keyed 0 to 100)", picture_project(Some(&write_on))), ("Brush Size 24, Hardness 0, Spacing 0, Reveal Original Image", picture_project(Some(&reveal)))] {
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 12, 24] {
                let (d, r, a, b) = both(&mut gpu, &project, &comp, &dir, frame, quality);
                let card = on_card(&project, &comp, &dir, frame, quality);
                t.row(
                    &format!("the town under \"{WORD}\", Path Stroke along its outlines {what}, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; the stroke on the card {card} (expected 0); warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 0,
                );
            }
        }
    }

    t.heading("Pictures: the town under a text layer, in `verification/D-373 pictures/`");
    let project = picture_project(Some(&write_on));
    let words = project.composition(&comp).unwrap().layer(&Id::new("art")).unwrap().text.clone().unwrap();
    let paths = anime_compositor::text::outlines(&words, &[]).unwrap();
    // The text layer's own letters, filled white, as the frame with no stroke shows them.
    let (letters, _) = picture(&dir, &picture_project(None), 0);
    let (mut frames, mut counts, mut ok) = (Vec::new(), Vec::new(), true);
    for frame in [0, 12, 24] {
        let (bytes, said) = picture(&dir, &project, frame);
        let changed: Vec<usize> = (0..w * h).filter(|&i| bytes[i * 4..][..4] != letters[i * 4..][..4]).collect();
        let off = changed.iter().filter(|&&i| paths_distance(&paths, (i % w) as f64 + 0.5, (i / w) as f64 + 0.5) > 4.5).count();
        ok &= said.is_empty() && off == 0;
        counts.push((changed.len(), off));
        png_out::write_rgba(&dir.join(format!("write_end_{:03}.png", frame * 100 / 24)), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        frames.push(bytes);
    }
    let (wide, bytes) = strip(&frames);
    png_out::write_rgba(&dir.join("write_strip.png"), wide, h, OutputDepth::Eight, &[], &bytes).unwrap();
    t.row(
        "write_strip.png (End 0, 50 and 100, each also alone as write_end_NNN.png): \"Write\" in white over the town, an orange line \
         drawing itself round the letters' edges one after another, W first; nothing changes further than the brush from an outline",
        &format!("pixels changed and of them off the outlines, per frame {counts:?}"),
        ok && counts[0].0 == 0 && counts[1].0 > 0 && counts[1].0 < counts[2].0,
    );
    let (bytes, said) = picture(&dir, &picture_project(Some(&reveal)), 0);
    png_out::write_rgba(&dir.join("write_reveal.png"), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
    t.row("write_reveal.png: Reveal Original Image, Brush Size 24: only a soft band along the letters' edges shows, the white letters' insides and the town under them seen only there", &format!("{said:?}"), said.is_empty());

    t.finish("D-373_stroke_text_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-236's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, and a text layer on
/// top, "Path Stroke" at size 200, with Path Stroke from its outlines, every eighth frame asked
/// for as the viewer asks, whole, with Draw on: GPU. The first loop's 30 frames' median is
/// "first"; then seven loops are timed, the median of their 210 frames is "again". With
/// `B252_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-252: a measurement, run deliberately with --release --ignored"]
fn b252_stroke_text_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B252_CPU").is_ok();
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
        ("Noise alone, the text layer without the effect", None),
        ("Noise, then Path Stroke from text outlines, outline 1 alone, Brush Size 2", Some(stroke_of(json!({"all_masks": "off", "stroke_sequentially": "off"})))),
        ("Noise, then Path Stroke from text outlines, All Masks, Stroke Sequentially, End 50, Brush Size 6", Some(stroke_of(json!({"end": 50, "brush_size": 6})))),
        ("Noise, then Path Stroke from text outlines, All Masks, Brush Size 24, Spacing 0", Some(stroke_of(json!({"stroke_sequentially": "off", "brush_size": 24, "spacing": 0})))),
    ];
    let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json")).expect("read the reference shot");
    for (name, e) in shots {
        let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
        let c = &mut j["compositions"][0];
        let layers = c["layers"].as_array_mut().unwrap();
        for (i, layer) in layers.iter_mut().take(3).enumerate() {
            layer["effects"] = json!([{"instance_id": format!("n{i}"), "type_id": "core.noise", "enabled": true, "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}}]);
        }
        let mut words = layers[0].clone();
        let obj = words.as_object_mut().unwrap();
        for k in ["asset_id", "source_offset_frames", "exposure_spans"] {
            obj.remove(k);
        }
        obj.insert("id".into(), json!("words"));
        obj.insert("name".into(), json!("words"));
        obj.insert("kind".into(), json!("text"));
        obj.insert("masks".into(), json!([]));
        obj.insert("source_text".into(), json!({"text": "Path Stroke", "font": "MPLUSRounded1c-Regular.ttf", "size": 200, "color": [1, 1, 1], "at": [360, 620], "align": "left"}));
        obj.insert("effects".into(), e.map_or(json!([]), |p| json!([{"instance_id": "s", "type_id": "core.stroke", "enabled": true, "parameters": p}])));
        layers.push(words);
        c["layer_order"].as_array_mut().unwrap().push(json!("words"));
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
    let out = std::env::var("B252_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-252_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
