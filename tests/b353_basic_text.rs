//! B-353: D-473, Basic Text, after After Effects' Basic Text (Obsolete): words typed into the
//! effect, drawn on the layer as Numbers draws its words (D-526).
//!
//! Every expected frame is a project in `Fixtures/basic_text/same/`, written by
//! `tools/basic_text_reference.py` before this code existed: the words the case must show, laid
//! as D-263 text layers. Each case's frame is compared sample by sample with that project's
//! frame. The card is held to the processor within 1 level of 255 (ADR-006, D-100).

mod effect_table;

use std::fs;

use effect_table::{median, reference, repo, said, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{self, plan_frame, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::{Effect, EffectKey};
use anime_compositor::model::Interp;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

fn set(effect: Effect) -> Command {
    Command::SetEffectParameters {
        composition: Id::new(MAIN),
        layer_id: Id::new("plate"),
        instance_id: Id::new("fx-1"),
        effect,
    }
}

/// Key a setting of the plate's Basic Text, linearly.
fn keys(setting: &str, values: &[(i32, &[f64])]) -> Command {
    Command::SetEffectKeys {
        composition: Id::new(MAIN),
        layer_id: Id::new("plate"),
        instance_id: Id::new("fx-1"),
        setting: setting.to_string(),
        keys: values.iter().map(|&(frame, v)| EffectKey { frame, value: v.to_vec(), interp: Interp::Linear }).collect(),
    }
}

fn effect_of(d: &Document) -> Effect {
    let comp = d.project().composition(&Id::new(MAIN)).unwrap();
    let layer = comp.layer(&Id::new("plate")).unwrap();
    layer.effects.iter().find(|i| i.instance_id.as_str() == "fx-1").unwrap().effect.clone()
}

/// Where the plates are written: the system's temporary folder.
fn plates() -> std::path::PathBuf {
    std::env::temp_dir().join("b353_plates")
}

/// A one-layer composition `w` by `h` at 24 frames a second of a still picture (written to
/// [`plates`]): a dark blue to purple ramp with lighter bars, carrying `effects`. A still, not a
/// solid, since the card is given a layer's effects only when it has a picture of its own.
fn plate((w, h): (usize, usize), frames: u32, effects: J) -> Project {
    let dir = plates();
    fs::create_dir_all(&dir).unwrap();
    let file = format!("plate_{w}x{h}.png");
    let pixels: Vec<u8> = (0..w * h)
        .flat_map(|i| {
            let (x, y) = (i % w, i / w);
            let bar = if (x / 40) % 2 == 0 { 0 } else { 24 };
            [(20 + 80 * x / w + bar) as u8, (30 + 40 * y / h) as u8, (90 + bar) as u8, 255]
        })
        .collect();
    png_out::write_rgba(&dir.join(&file), w, h, OutputDepth::Eight, &[], &pixels).unwrap();
    let t = |v: J| json!({"base": v, "keyframes": []});
    let c = json!([w as f64 / 2.0, h as f64 / 2.0]);
    let project = json!({
        "schema_version": 0, "project_id": "proj-b353",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-plate", "kind": "still", "name": "plate", "path": file,
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
            "id": MAIN, "name": "Main", "width": w, "height": h, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": frames,
            "work_area": {"start_frame": 0, "end_frame_exclusive": frames},
            "layer_order": ["plate"],
            "layers": [{"id": "plate", "kind": "raster", "name": "plate", "asset_id": "asset-plate", "enabled": true,
                "locked": false, "in_frame": 0, "out_frame": frames, "source_offset_frames": 0,
                "transform": {"anchor": t(c.clone()), "position": t(c), "scale": t(json!([100, 100])),
                              "rotation": t(json!(0)), "opacity": t(json!(1))},
                "exposure_spans": [], "mask": null, "matte": null, "blend_mode": "normal", "effects": effects}]
        }]
    });
    persist::load_str(&project.to_string()).expect("the project reads").document.project().clone()
}

/// Basic Text as added with `changes` laid over its settings.
fn basic_text(id: &str, changes: J) -> J {
    let mut p = json!({"text": "Text", "position": [50, 50], "display": "fill_only", "fill_color": "#ffffff",
        "stroke_color": "#ff3300", "stroke_width": 3, "size": 64, "tracking": 0,
        "composite_on_original": "on", "font": "MPLUSRounded1c-Regular.ttf", "alignment": "center"});
    for (k, v) in changes.as_object().unwrap() {
        p[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.basic_text", "enabled": true, "parameters": p})
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

#[test]
fn b353_basic_text() {
    let mut t = Table::new(
        "basic_text",
        "# D-473: Basic Text\n\nB-353: `core.basic_text`, after After Effects' Basic Text (Obsolete): words \
         typed into the effect, drawn on the layer as Numbers draws its words (D-526), proportional spacing \
         on. Every expected frame is a project in `Fixtures/basic_text/same/`, written by \
         `tools/basic_text_reference.py` before this code existed and printed in document 25 as FX-BTEXT-001 \
         to 025: the words the case must show, as text layers. The build's frame is compared sample by sample \
         with that project's frame; the answer is the largest difference over all of them, against the \
         catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-BTEXT-001 to 025 (document 25): each frame against the words it must show");
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_basic_text.json")).unwrap()).unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();
    let mut files = Vec::new();
    let mut shots: Vec<(String, Project, i32)> = Vec::new();
    for (name, case) in expected["cases"].as_object().unwrap() {
        let says = case["says"].as_str().unwrap();
        let file = case["project"].as_str().unwrap();
        files.push(file.to_string());
        let loaded = t.load(file);
        for (frame, f) in case["frames"].as_object().unwrap() {
            let frame: i32 = frame.parse().unwrap();
            let same = t.load(f["same"].as_str().unwrap()).document;
            let built = t.render(&loaded.document, frame, 64);
            let want = t.render(&same, frame, 64);
            let d = built.data().iter().zip(want.data()).map(|(a, b)| (a - b).abs() as f64).fold(0.0, f64::max);
            let shows = f["shows"].as_str().map_or("nothing".to_string(), |s| format!("{s:?}"));
            t.row(
                &format!("{name} frame {frame}: {says} Shows {shows}"),
                &format!("largest difference {d:.1e}"),
                d <= tolerance && built.data().len() == want.data().len(),
            );
            shots.push((format!("{file} frame {frame}"), loaded.document.project().clone(), frame));
        }
        let on_open: Vec<&str> = loaded.warnings.iter().map(|d| d.id.as_str()).collect();
        let mut log = FrameLog::new(8);
        let _ = plan_frame(loaded.document.project(), &Id::new(MAIN), 4, &t.root, &mut log);
        let at_frame: Vec<&str> = log.finish().iter().map(|d| d.id.as_str()).collect();
        let want: Vec<&str> = case.get("warning").and_then(J::as_str).into_iter().collect();
        let mut want_at_frame = want.clone();
        want_at_frame.extend(case.get("frame_warning").and_then(J::as_str));
        t.row(
            &format!("{name}: what opening it warns of, and what frame 4 warns of"),
            &format!("{on_open:?} and {at_frame:?}"),
            on_open == want && at_frame == want_at_frame,
        );
    }

    t.heading("The file");
    let valid: Vec<&str> = files.iter().map(String::as_str).filter(|f| f < &"fx_btext_020.json").collect();
    t.round_trips(&valid);
    let saved = t.saved_parameters("fx_btext_009.json");
    t.row(
        "fx_btext_009.json is saved with its two lines as typed and no Numbers settings of its own",
        &saved.to_string(),
        saved["text"] == "Basic\nText" && saved.get("type").is_none() && saved.get("proportional_spacing").is_none() && saved.get("clock").is_none(),
    );
    let refusals = [
        ("fx_btext_020.json", "Basic Text's display is one of fill_only, stroke_only, fill_over_stroke, stroke_over_fill, and this is \"outline\"."),
        ("fx_btext_021.json", "Basic Text's text is at most 1000 characters, and this is 1001."),
        ("fx_btext_023.json", "Basic Text's fill colour is written #rrggbb, and this is \"white\"."),
        ("fx_btext_024.json", "Basic Text's alignment is one of left, center, right, and this is \"justify\"."),
    ];
    for (file, sentence) in refusals {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming it"), &why, why == sentence);
    }
    let why = effect_of(&t.load("fx_btext_022.json").document).why_invalid();
    t.row("fx_btext_022.json is refused in a sentence naming the font", &why, why.starts_with("Basic Text needs ") && why.contains("../fonts/x.ttf"));

    t.heading("Commands");
    let mut document = t.load("fx_btext_002.json").document;
    let base = effect_of(&document);
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = base.clone();
        f(&mut e);
        e
    };
    for (what, command) in [
            ("size 0", set(with(&|e| if let Effect::BasicText { size, .. } = e { *size = 0.0 }))),
            ("a text of 1001 characters", set(with(&|e| if let Effect::BasicText { text, .. } = e { *text = "x".repeat(1001) }))),
            ("display \"Fill Only\", written as shown", set(with(&|e| if let Effect::BasicText { display, .. } = e { *display = "Fill Only".into() }))),
            ("stroke width keyed to 600", keys("stroke_width", &[(0, &[3.0]), (4, &[600.0])])),
    ] {
        let held = format!("{:?}", effect_of(&document));
        let refused = document.apply(command).err();
        let untouched = format!("{:?}", effect_of(&document)) == held;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && untouched,
        );
    }
    t.taken(
        &mut document,
        "fx_btext_002.json",
        vec![
            ("The words changed to Goodbye,", set(with(&|e| if let Effect::BasicText { text, .. } = e { *text = "Goodbye".into() }))),
            ("Stroke over fill, width 5, right-aligned,", set(with(&|e| if let Effect::BasicText { display, stroke_width, alignment, .. } = e {
                *display = "stroke_over_fill".to_string();
                *stroke_width = 5.0;
                *alignment = "right".to_string();
            }))),
            ("size keyed from 20 to 200", keys("size", &[(0, &[20.0]), (4, &[200.0])])),
        ],
    );

    t.heading("A half-size draft");
    let mut draft = with(&|_| {});
    draft.scale_distances(|v| v / 2.0);
    let halved = matches!(&draft, Effect::BasicText { size, stroke_width, position, tracking, .. }
        if *size == 24.0 && *stroke_width == 1.5 && *position == [5.0, 50.0] && *tracking == 0.0);
    t.row("A half-size draft halves the size and the stroke, and keeps the position (a per cent) and tracking (a share of the size)", &format!("{draft:?}"), halved);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_btext_001.json", 0), ("fx_btext_006.json", 0), ("fx_btext_009.json", 0), ("fx_btext_013.json", 4)]);

    t.heading("The card against the processor, within 1 level of 255 (ADR-006, D-100)");
    let mut gpu = Gpu::new().expect("a usable card");
    let fixtures = t.root.clone();
    let plate_dir = plates();
    let mut card: Vec<(String, Project, i32, bool)> = shots.into_iter().map(|(n, p, f)| (n, p, f, false)).collect();
    let samples: [(&str, J); 4] = [
        ("as added", json!({})),
        ("Hello, world, stroke over fill, size 40, from the left", json!({"text": "Hello, world", "display": "stroke_over_fill", "size": 40, "alignment": "left", "position": [5, 60]})),
        ("two lines, fill over stroke, composite off", json!({"text": "Basic\nText", "display": "fill_over_stroke", "stroke_width": 6, "size": 40, "position": [50, 40], "composite_on_original": "off"})),
        ("kana, stroke only, tracking 100", json!({"text": "こんにちは", "display": "stroke_only", "stroke_width": 4, "tracking": 100})),
    ];
    for (name, changes) in samples {
        card.push((format!("a 480 by 160 picture, {name}"), plate((480, 160), 60, json!([basic_text("fx-1", changes)])), 9, true));
    }
    let mut worst = 0u8;
    for (name, project, frame, is_sample) in &card {
        let comp = &project.compositions[0];
        let root = if *is_sample { &plate_dir } else { &fixtures };
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            let mut cache = CelCache::viewer();
            let mut log = FrameLog::new(3);
            let c = preview::preview_frame_cached(project, &comp.id, *frame, root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache)
                .unwrap_or_else(|d| panic!("{name} on the CPU: {}", d.message));
            let said_cpu = said(log);
            let c = c.to_srgb8_straight();
            let mut log = FrameLog::new(3);
            let (g, ..) = preview::preview_frame_srgb8(project, &comp.id, *frame, root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu)
                .unwrap_or_else(|d| panic!("{name} on the GPU: {}", d.message));
            let said_gpu = said(log);
            let mut log = FrameLog::new(3);
            let plan = compose::plan_frame_for_card(project, &comp.id, *frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
            let on_card: usize = plan.layers.iter().map(|l| l.on_card.len()).sum();
            let effects = comp.layer(&Id::new("plate")).map_or(0, |l| l.effects.len());
            let d = distance(&c, &g);
            worst = worst.max(d.0);
            let fallback = said_gpu.contains(DiagnosticId::GpuPreviewOnCpu.as_str());
            t.row(
                &format!("{name}, {}", quality.label()),
                &format!("largest {} level(s), {} pixel(s) differ; {on_card} of {effects} effect(s) on the card; warnings {said_gpu:?}", d.0, d.1),
                d.0 <= 1 && !fallback && said_cpu == said_gpu && (!is_sample || on_card == effects),
            );
        }
    }
    t.row("the largest difference over every frame above", &format!("{worst} level(s)"), worst <= 1);

    t.heading("Pictures: in `verification/D-473 pictures/`");
    let dir = repo("verification/D-473 pictures");
    fs::create_dir_all(&dir).unwrap();
    let pictures = [
        ("fx_btext_001.json", 0, "1_as_added", "as added: the word Text in white in the middle of the plate"),
        ("fx_btext_002.json", 0, "2_hello_world", "Hello, world, smaller, from the left edge"),
        ("fx_btext_003.json", 0, "3_spaced_right", "Spaced, its letters set wide, ending at the right"),
        ("fx_btext_004.json", 0, "4_stroke_only", "Ring as an orange outline, the plate showing through the letters"),
        ("fx_btext_006.json", 0, "5_stroke_over_fill", "Edge in white with a thick orange edge"),
        ("fx_btext_008.json", 0, "6_composite_off", "Alone in light blue with a dark blue edge on transparent (shown on black)"),
        ("fx_btext_009.json", 0, "7_two_lines", "Basic above Text, two centred lines"),
        ("fx_btext_010.json", 0, "8_kana", "konnichiwa in kana"),
        ("fx_btext_012.json", 5, "9_grow_frame_5", "Grow at size 70, halfway through its size keys"),
        ("fx_btext_014.json", 0, "10_missing_font", "a font this computer does not have: the plain plate, and a warning"),
    ];
    for (file, frame, png, what) in pictures {
        let document = t.load(file).document;
        let mut log = FrameLog::new(8);
        let b = render_frame(document.project(), &Id::new(MAIN), frame, &fixtures, 64, &mut log).expect("the frame draws");
        let s = said(log);
        png_out::write_rgba(&dir.join(format!("{png}.png")), 480, 160, OutputDepth::Eight, &[], &b.to_srgb8_straight()).unwrap();
        let want = if file == "fx_btext_014.json" { "TEXT_FONT_MISSING" } else { "" };
        t.row(&format!("{png}.png, {what}"), &format!("warnings {s:?}"), s == want);
    }

    t.finish("D-473_basic_text_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B353_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-353: a measurement, run deliberately with --release --ignored"]
fn b353_basic_text_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B353_CPU").is_ok();
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
        ("Noise, then Basic Text at size 200", Some(json!({"text": "Basic Text", "size": 200}))),
        ("the same in two lines, stroke over fill, width 6", Some(json!({"text": "Basic\nText", "size": 200, "display": "stroke_over_fill", "stroke_width": 6}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(basic_text(&format!("{id}b"), p.clone()));
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
    let out = std::env::var("B353_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-353_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
