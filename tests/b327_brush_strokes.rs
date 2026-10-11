//! B-327: D-447, After Effects' Brush Strokes under `core.brush_strokes`: Stroke Angle, Brush
//! Size, Stroke Length, Stroke Density, Stroke Randomness, Paint Surface and Blend With Original,
//! with our Random Seed and Animate (new strokes each frame, on as After Effects does it).
//!
//! Writes `verification/D-447_brush_strokes_table.md` and draws pictures into
//! `verification/D-447 pictures/`.
//!
//! Every expected pixel is `Fixtures/brush_strokes/expected_brush_strokes.json`, written by
//! `tools/brush_strokes_reference.py` before this code existed and printed in document 25 as
//! FX-BRUSH-001 to 029. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, repo, set, town, Table, MAIN, TOWN};
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

/// The Brush Strokes the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::BrushStrokes { .. })))
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

/// A Brush Strokes with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"stroke_angle": 135, "brush_size": 2, "stroke_length": 8, "stroke_density": 1, "stroke_randomness": 1,
        "paint_surface": "original", "blend_with_original": 0, "random_seed": 0, "animate": "on"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.brush_strokes", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=29 {
        let file = format!("fx_brush_{n:03}.json");
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

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor; `want` of 3 on the card each frame.
fn card_reference(t: &mut Table, gpu: &mut Gpu, cases: &[(&str, J, usize)]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p, want) in cases {
        let project = reference(|id| json!([fx(&format!("b327-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Brush Strokes {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Brush Strokes {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the still `asset` (in `dir`, `size` pixels), drawn on the processor at `frame`,
/// straight 8-bit, with what it warned of.
fn picture(dir: &Path, asset: &str, size: (usize, usize), effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from(asset);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(size.0);
    comp["height"] = J::from(size.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([size.0 as f64 / 2.0, size.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// The `art` layer's effect in `file`, changed by `f`.
fn changed(t: &Table, file: &str, f: impl FnOnce(&mut Effect)) -> Effect {
    let d = t.load(file).document;
    let mut e = d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    f(&mut e);
    e
}

/// How many neighbouring pixel pairs differ by more than `step` levels in red: across, then down.
fn edges(p: &[u8], (w, h): (usize, usize), step: u8) -> (usize, usize) {
    let r = |x: usize, y: usize| p[(y * w + x) * 4];
    let (mut across, mut down) = (0, 0);
    for y in 0..h - 1 {
        for x in 0..w - 1 {
            across += (r(x, y).abs_diff(r(x + 1, y)) > step) as usize;
            down += (r(x, y).abs_diff(r(x, y + 1)) > step) as usize;
        }
    }
    (across, down)
}

#[test]
fn b327_brush_strokes() {
    let mut t = Table::new(
        "brush_strokes",
        "# D-447: Brush Strokes\n\nB-327: `core.brush_strokes` takes After Effects' Brush Strokes \
         controls (Stroke Angle, Brush Size, Stroke Length, Stroke Density, Stroke Randomness, Paint \
         Surface, Blend With Original) with our Random Seed and Animate. Every expected pixel is \
         `Fixtures/brush_strokes/expected_brush_strokes.json`, written by \
         `tools/brush_strokes_reference.py` before this code existed and printed in document 25 as \
         FX-BRUSH-001 to 029. Tolerance 2e-5.\n",
    );

    t.heading("FX-BRUSH-001 to 029 (document 25)");
    t.fixtures_numbered("expected_brush_strokes.json", 1..=29);

    t.heading("The file");
    let files: Vec<String> = (1..=29).map(|n| format!("fx_brush_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_brush_020.json");
    t.row(
        "fx_brush_020.json is saved with its nine settings and no frame",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 9 && saved["stroke_angle"] == 200.0 && saved["paint_surface"] == "black" && saved.get("frame").is_none(),
    );
    for (file, want) in [
        ("fx_brush_021.json", "brush size"),
        ("fx_brush_022.json", "stroke length"),
        ("fx_brush_023.json", "stroke density"),
        ("fx_brush_024.json", "stroke randomness"),
        ("fx_brush_025.json", "blend with original"),
        ("fx_brush_026.json", "random seed"),
        ("fx_brush_027.json", "paint surface"),
        ("fx_brush_028.json", "animate"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming Brush Strokes and its {want}"), &why, why.contains("Brush Strokes") && why.contains(want));
    }
    let full = r#"{"stroke_angle": 135, "brush_size": 2, "stroke_length": 8, "stroke_density": 1, "stroke_randomness": 1, "paint_surface": "original", "blend_with_original": 0, "random_seed": 0, "animate": "on"}"#;
    t.shape_refused("fx_brush_001.json", "a Brush Strokes whose brush size is a word", &full.replace("\"brush_size\": 2", "\"brush_size\": \"fat\""));
    t.shape_refused("fx_brush_001.json", "a Brush Strokes without its stroke length", &full.replace("\"stroke_length\": 8, ", ""));
    t.shape_refused("fx_brush_001.json", "a Brush Strokes whose paint surface is a number", &full.replace("\"original\"", "2"));

    t.heading("Commands");
    let mut document = t.load("fx_brush_001.json").document;
    let thin = changed(&t, "fx_brush_001.json", |e| if let Effect::BrushStrokes { brush_size, .. } = e { *brush_size = 0.4 });
    let bad = changed(&t, "fx_brush_001.json", |e| if let Effect::BrushStrokes { paint_surface, .. } = e { *paint_surface = "White".into() });
    t.refused(
        &mut document,
        vec![
            ("brush size 0.4", set(thin)),
            ("paint surface White", set(bad)),
            ("stroke length keyed to 150", keys("stroke_length", &[(0, &[8.0]), (4, &[150.0])])),
        ],
    );
    let fat = changed(&t, "fx_brush_001.json", |e| {
        if let Effect::BrushStrokes { brush_size, paint_surface, .. } = e {
            *brush_size = 4.0;
            *paint_surface = "white".into();
        }
    });
    t.taken(
        &mut document,
        "fx_brush_001.json",
        vec![
            ("brush size 4 on white", set(fat)),
            ("stroke angle keyed from 0 to 360", keys("stroke_angle", &[(0, &[0.0]), (4, &[360.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_brush_001.json", 1),
        ("fx_brush_010.json", 0),
        ("fx_brush_017.json", 2),
        ("fx_brush_019.json", 0),
        ("fx_brush_020.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Refused (021 to 029) leave nothing for the card.
    let none: Vec<u32> = (21..=29).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added (new strokes each frame)", json!({}), 3),
            ("on black, brush 4, length 20, held", json!({"paint_surface": "black", "brush_size": 4, "stroke_length": 20, "animate": "off"}), 3),
            ("angle 90, randomness 2, density 3, blend 30", json!({"stroke_angle": 90, "stroke_randomness": 2, "stroke_density": 3, "blend_with_original": 30}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-447 pictures/`");
    let dir = repo("verification/D-447 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let street = |p: J, frame: i32| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]), frame);
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]), 0);
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());

    let (added, said) = street(json!({}), 0);
    write("2_as_added.png", TOWN, &added);
    let (next, said1) = street(json!({}), 1);
    write("2b_as_added_frame_1.png", TOWN, &next);
    t.row(
        "2_as_added.png and 2b_as_added_frame_1.png, as added: the street repainted in short strokes leaning down and to the right; frame 1 has new strokes, so the two differ; both draw cleanly",
        &format!("{said:?} {said1:?}, pixels differing from the street {}, between the frames {}", distance(&added, &before).1, distance(&added, &next).1),
        said.is_empty() && said1.is_empty() && added != before && added != next,
    );

    let (held, said) = street(json!({"animate": "off"}), 0);
    let (held3, said1) = street(json!({"animate": "off"}), 3);
    write("3_held.png", TOWN, &held);
    t.row(
        "3_held.png, New Strokes Each Frame off: frame 3 the same as frame 0 byte for byte; both draw cleanly",
        &format!("{said:?} {said1:?}, {} pixels differ", distance(&held, &held3).1),
        said.is_empty() && said1.is_empty() && held == held3 && held != before,
    );

    let (level, said) = street(json!({"stroke_angle": 90, "stroke_randomness": 0, "stroke_length": 30, "animate": "off"}), 0);
    write("4_level_strokes.png", TOWN, &level);
    let (e0, e1) = (edges(&before, TOWN, 8), edges(&level, TOWN, 8));
    t.row(
        "4_level_strokes.png, angle 90, randomness 0, length 30: long level strokes running right, so fewer edges across than the street has, and fewer across than down; draws cleanly",
        &format!("{said:?}, edges across/down: street {}/{}, strokes {}/{}", e0.0, e0.1, e1.0, e1.1),
        said.is_empty() && e1.0 < e0.0 && e1.0 < e1.1,
    );

    let (sparse, said) = street(json!({"stroke_density": 0.3, "paint_surface": "black", "animate": "off"}), 0);
    write("5_sparse_on_black.png", TOWN, &sparse);
    let black = |p: &[u8]| p.chunks_exact(4).filter(|q| q[0] == 0 && q[1] == 0 && q[2] == 0 && q[3] == 255).count();
    t.row(
        "5_sparse_on_black.png, density 0.3 on black: scattered strokes with black between them, more pure black pixels than the street; draws cleanly",
        &format!("{said:?}, pure black pixels: street {}, strokes {}", black(&before), black(&sparse)),
        said.is_empty() && black(&sparse) > black(&before) + 1000,
    );

    let (fat, said) = street(json!({"brush_size": 8, "stroke_length": 24, "animate": "off"}), 0);
    write("6_brush_8.png", TOWN, &fat);
    let (ef, eh) = (edges(&fat, TOWN, 8), edges(&held, TOWN, 8));
    t.row(
        "6_brush_8.png, brush size 8, length 24: big blocky dabs, fewer edges than 3_held.png's size 2; draws cleanly",
        &format!("{said:?}, edges across+down: size 2 {}, size 8 {}", eh.0 + eh.1, ef.0 + ef.1),
        said.is_empty() && ef.0 + ef.1 < eh.0 + eh.1,
    );

    let (half, said) = street(json!({"blend_with_original": 50, "animate": "off"}), 0);
    write("7_blend_50.png", TOWN, &half);
    let between = before.chunks_exact(4).zip(held.chunks_exact(4)).zip(half.chunks_exact(4)).all(|((b, s), m)| (0..3).all(|c| m[c] >= b[c].min(s[c]).saturating_sub(1) && m[c] <= b[c].max(s[c]).saturating_add(1)));
    let (whole, said1) = street(json!({"blend_with_original": 100}), 0);
    t.row(
        "7_blend_50.png, Blend With Original 50: every pixel between the street and 3_held.png; at 100 the street itself byte for byte; both draw cleanly",
        &format!("{said:?} {said1:?}, between: {between}, 100 the street: {}", whole == before),
        said.is_empty() && said1.is_empty() && between && half != held && whole == before,
    );

    t.finish("D-447_brush_strokes_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B327_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-327: a measurement, run deliberately with --release --ignored"]
fn b327_brush_strokes_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B327_CPU").is_ok();
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
        ("Noise, then Brush Strokes as added", Some(json!({}))),
        ("Noise, then Brush Strokes brush 6, length 30, density 4, randomness 2 (many strokes a pixel)", Some(json!({"brush_size": 6, "stroke_length": 30, "stroke_density": 4, "stroke_randomness": 2}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}c"), p));
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
    let out = std::env::var("B327_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-327_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
