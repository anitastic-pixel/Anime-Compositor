//! B-406: D-526, Numbers, after After Effects' Numbers (Text): a number, timecode, time, date or
//! hexadecimal number drawn as words on the layer.
//!
//! Every expected frame is a project in `Fixtures/numbers/same/`, written by
//! `tools/numbers_reference.py` before this code existed: the words the case must show, laid as
//! D-263 text layers. Each case's frame is compared sample by sample with that project's frame.
//! The card is held to the processor within 1 level of 255 (ADR-006, D-100).

mod effect_table;

use std::fs;

use effect_table::{median, said, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{self, plan_frame, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
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

fn effect_of(d: &Document) -> Effect {
    let comp = d.project().composition(&Id::new(MAIN)).unwrap();
    let layer = comp.layer(&Id::new("plate")).unwrap();
    layer
        .effects
        .iter()
        .find(|i| i.instance_id.as_str() == "fx-1")
        .unwrap()
        .effect
        .clone()
}

/// Where the plates are written: the system's temporary folder.
fn plates() -> std::path::PathBuf {
    std::env::temp_dir().join("b406_plates")
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
            [
                (20 + 80 * x / w + bar) as u8,
                (30 + 40 * y / h) as u8,
                (90 + bar) as u8,
                255,
            ]
        })
        .collect();
    png_out::write_rgba(&dir.join(&file), w, h, OutputDepth::Eight, &[], &pixels).unwrap();
    let t = |v: J| json!({"base": v, "keyframes": []});
    let c = json!([w as f64 / 2.0, h as f64 / 2.0]);
    let project = json!({
        "schema_version": 0, "project_id": "proj-b406",
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
    persist::load_str(&project.to_string())
        .expect("the project reads")
        .document
        .project()
        .clone()
}

/// Numbers as added with `changes` laid over its settings.
fn numbers(changes: J) -> J {
    let mut p = json!({"type": "number", "random_values": "off", "value": 0, "decimal_places": 0,
        "position": [50, 50], "display": "fill_only", "fill_color": "#ffffff", "stroke_color": "#ff3300",
        "stroke_width": 3, "size": 64, "tracking": 0, "proportional_spacing": "on",
        "composite_on_original": "on", "font": "MPLUSRounded1c-Regular.ttf", "alignment": "center"});
    for (k, v) in changes.as_object().unwrap() {
        p[k] = v.clone();
    }
    json!({"instance_id": "fx-1", "type_id": "core.numbers", "enabled": true, "parameters": p})
}

fn distance(a: &[u8], b: &[u8]) -> (u8, usize) {
    assert_eq!(a.len(), b.len(), "the two pictures are different sizes");
    let (mut largest, mut pixels) = (0, 0);
    for (p, q) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
        let d = p
            .iter()
            .zip(q)
            .map(|(x, y)| x.abs_diff(*y))
            .max()
            .unwrap_or(0);
        largest = largest.max(d);
        pixels += (d > 0) as usize;
    }
    (largest, pixels)
}

#[test]
fn b406_numbers() {
    let mut t = Table::new(
        "numbers",
        "# B-406: Numbers\n\nD-526: Numbers, after After Effects' Numbers (Text): a number, \
         timecode, time, date or hexadecimal number drawn as words on the layer. Every expected \
         frame is a project in `Fixtures/numbers/same/`, written by `tools/numbers_reference.py` \
         before this code existed and printed in document 25 as FX-NUM-001 to 045: the words the \
         case must show, as text layers. The build's frame is compared sample by sample with \
         that project's frame; the answer is the largest difference over all of them, against \
         the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-NUM-001 to 045 (document 25): each frame against the words it must show");
    let expected: J =
        serde_json::from_str(&fs::read_to_string(t.root.join("expected_numbers.json")).unwrap())
            .unwrap();
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
            let d = built
                .data()
                .iter()
                .zip(want.data())
                .map(|(a, b)| (a - b).abs() as f64)
                .fold(0.0, f64::max);
            let shows = f["shows"]
                .as_str()
                .map_or("nothing".to_string(), |s| format!("\"{s}\""));
            t.row(
                &format!("{name} frame {frame}: {says} Shows {shows}"),
                &format!("largest difference {d:.1e}"),
                d <= tolerance && built.data().len() == want.data().len(),
            );
            shots.push((
                format!("{file} frame {frame}"),
                loaded.document.project().clone(),
                frame,
            ));
        }
        let on_open: Vec<&str> = loaded.warnings.iter().map(|d| d.id.as_str()).collect();
        let mut log = FrameLog::new(8);
        let _ = plan_frame(
            loaded.document.project(),
            &Id::new(MAIN),
            4,
            &t.root,
            &mut log,
        );
        let at_frame: Vec<&str> = log.finish().iter().map(|d| d.id.as_str()).collect();
        let want: Vec<&str> = case
            .get("warning")
            .and_then(J::as_str)
            .into_iter()
            .collect();
        let mut want_at_frame = want.clone();
        want_at_frame.extend(case.get("frame_warning").and_then(J::as_str));
        t.row(
            &format!("{name}: what opening it warns of, and what frame 4 warns of"),
            &format!("{on_open:?} and {at_frame:?}"),
            on_open == want && at_frame == want_at_frame,
        );
    }

    t.heading("The file");
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    for file in ["fx_num_001.json", "fx_num_024.json"] {
        let params = t.saved_parameters(file);
        t.row(
            &format!(
                "{file}: the clock worked each frame is never saved; the type is saved as type"
            ),
            &params.to_string(),
            params.get("clock").is_none()
                && params.get("kind").is_none()
                && params.get("type").is_some(),
        );
    }
    let refusals = [
        ("fx_num_040.json", "Numbers's type is one of number, number_leading_zeros, timecode_30, timecode_25, timecode_24, time, numerical_date, short_date, long_date, hexadecimal, and this is \"roman\"."),
        ("fx_num_041.json", "Numbers's display is one of fill_only, stroke_only, fill_over_stroke, stroke_over_fill, and this is \"outline\"."),
        ("fx_num_042.json", "Numbers's value runs from -30000 to 30000, and this is 40000."),
        ("fx_num_043.json", "Numbers's decimal places runs from 0 to 10, and this is 11."),
        ("fx_num_045.json", "Numbers's fill colour is written #rrggbb, and this is \"white\"."),
    ];
    for (file, sentence) in refusals {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(
            &format!("{file} is refused in a sentence naming it"),
            &why,
            why == sentence,
        );
    }
    let why = effect_of(&t.load("fx_num_044.json").document).why_invalid();
    t.row(
        "fx_num_044.json is refused in a sentence naming the font",
        &why,
        why.starts_with("Numbers needs ") && why.contains("../fonts/x.ttf"),
    );

    t.heading("Commands");
    let mut document = t.load("fx_num_002.json").document;
    let base = effect_of(&document);
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = base.clone();
        f(&mut e);
        e
    };
    let refused = document
        .apply(set(with(&|e| {
            if let Effect::Numbers { size, .. } = e {
                *size = 0.0
            }
        })))
        .err();
    let untouched = effect_of(&document) == base;
    t.row(
        "Size 0 is refused with a sentence, and nothing changes",
        &refused
            .as_ref()
            .map_or("taken".to_string(), |d| d.message.clone()),
        refused.is_some() && untouched,
    );
    t.taken(
        &mut document,
        "fx_num_002.json",
        vec![
            (
                "Type Long Date, value 365,",
                set(with(&|e| {
                    if let Effect::Numbers { kind, value, .. } = e {
                        *kind = "long_date".to_string();
                        *value = 365.0;
                    }
                })),
            ),
            (
                "Stroke over fill, width 5, monospaced, right-aligned,",
                set(with(&|e| {
                    if let Effect::Numbers {
                        display,
                        stroke_width,
                        proportional_spacing,
                        alignment,
                        ..
                    } = e
                    {
                        *display = "stroke_over_fill".to_string();
                        *stroke_width = 5.0;
                        *proportional_spacing = "off".to_string();
                        *alignment = "right".to_string();
                    }
                })),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_num_001.json", 0),
        ("fx_num_017.json", 0),
        ("fx_num_027.json", 0),
        ("fx_num_030.json", 37),
        ("fx_num_032.json", 0),
    ]);

    t.heading("The card against the processor, within 1 level of 255 (ADR-006, D-100)");
    let mut gpu = Gpu::new().expect("a usable card");
    let fixtures = t.root.clone();
    let plate_dir = plates();
    let mut card: Vec<(String, Project, i32, bool)> = shots
        .into_iter()
        .map(|(n, p, f)| (n, p, f, false))
        .collect();
    let samples: [(&str, J); 4] = [
        ("as added", json!({})),
        (
            "timecode 30, monospaced, fill over stroke",
            json!({"type": "timecode_30", "proportional_spacing": "off", "display": "fill_over_stroke"}),
        ),
        (
            "long date, size 30, stroke over fill, composite off",
            json!({"type": "long_date", "value": 4000, "size": 30, "display": "stroke_over_fill", "stroke_width": 4, "composite_on_original": "off"}),
        ),
        (
            "random values with two places, stroke only",
            json!({"random_values": "on", "value": 500, "decimal_places": 2, "display": "stroke_only", "stroke_width": 5}),
        ),
    ];
    for (name, changes) in samples {
        card.push((
            format!("a 480 by 160 picture, {name}"),
            plate((480, 160), 60, json!([numbers(changes)])),
            9,
            true,
        ));
    }
    let mut worst = 0u8;
    for (name, project, frame, is_sample) in &card {
        let comp = &project.compositions[0];
        let root = if *is_sample { &plate_dir } else { &fixtures };
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            let mut cache = CelCache::viewer();
            let mut log = FrameLog::new(3);
            let c = preview::preview_frame_cached(
                project,
                &comp.id,
                *frame,
                root,
                quality,
                DEFAULT_TILE_SIZE,
                &mut log,
                &mut cache,
            )
            .unwrap_or_else(|d| panic!("{name} on the CPU: {}", d.message));
            let said_cpu = said(log);
            let c = c.to_srgb8_straight();
            let mut log = FrameLog::new(3);
            let (g, ..) = preview::preview_frame_srgb8(
                project,
                &comp.id,
                *frame,
                root,
                quality,
                DEFAULT_TILE_SIZE,
                &mut log,
                &mut cache,
                &mut gpu,
            )
            .unwrap_or_else(|d| panic!("{name} on the GPU: {}", d.message));
            let said_gpu = said(log);
            let mut log = FrameLog::new(3);
            let plan = compose::plan_frame_for_card(
                project,
                &comp.id,
                *frame,
                root,
                quality,
                &mut log,
                &mut CelCache::viewer(),
            )
            .expect("plan the frame");
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
    t.row(
        "the largest difference over every frame above",
        &format!("{worst} level(s)"),
        worst <= 1,
    );

    t.heading("Pictures: in `verification/D-526 pictures/`");
    let dir = effect_table::repo("verification/D-526 pictures");
    fs::create_dir_all(&dir).unwrap();
    let pictures = [
        ("fx_num_001.json", 0, "as_added", "as added: a white 0 in the middle of the dark blue plate"),
        ("fx_num_002.json", 0, "two_places", "1234.57"),
        ("fx_num_008.json", 50, "timecode_30", "the layer's time as 30-frame timecode at frame 50: 00:00:02:02"),
        ("fx_num_017.json", 0, "long_date", "Monday, January 1, 1996, smaller, from the left"),
        ("fx_num_019.json", 0, "hexadecimal", "18000, 1.5 in hexadecimal"),
        ("fx_num_021.json", 7, "random", "a random number up to 100 with two places at frame 7: 90.47"),
        ("fx_num_024.json", 4, "counter_frame_4", "the counter keyed from 0 to 100, at frame 4: 40"),
        ("fx_num_025.json", 0, "stroke_only", "2026 as an orange ring round where the letters would be, the plate showing through them"),
        ("fx_num_026.json", 0, "fill_over_stroke", "2026 in white with a thick orange edge"),
        ("fx_num_027.json", 0, "stroke_over_fill", "2026 in white with a thick orange edge, as fill over stroke (D-264's stroke lies outside the letters)"),
        ("fx_num_029.json", 0, "monospaced", "1.11 with every character, the point too, in a cell as wide as a digit"),
        ("fx_num_030.json", 37, "monospaced_timecode", "00:00:01:13 in 25-frame timecode, monospaced, with an orange edge, from the left"),
        ("fx_num_032.json", 0, "composite_off", "a light blue 88 with a dark blue edge on transparent (shown on black)"),
        ("fx_num_034.json", 0, "missing_font", "a font this computer does not have: the plain plate, and a warning"),
    ];
    for (file, frame, png, what) in pictures {
        let document = t.load(file).document;
        let mut log = FrameLog::new(8);
        let b = render_frame(
            document.project(),
            &Id::new(MAIN),
            frame,
            &fixtures,
            64,
            &mut log,
        )
        .expect("the frame draws");
        let s = said(log);
        png_out::write_rgba(
            &dir.join(format!("{png}.png")),
            480,
            160,
            OutputDepth::Eight,
            &[],
            &b.to_srgb8_straight(),
        )
        .unwrap();
        let want = if file == "fx_num_034.json" {
            "TEXT_FONT_MISSING"
        } else {
            ""
        };
        t.row(
            &format!("{png}.png, {what}"),
            &format!("warnings {s:?}"),
            s == want,
        );
    }

    t.finish("D-526_numbers_table.md");
}

/// The frame times, B-229's way: a 1920 by 1080 solid, the effect then a Noise that changes every
/// frame so nothing is kept, every eighth frame of 240 asked for as the viewer asks, whole, with
/// Draw on: GPU. The first loop starts with empty caches and its 30 frames' median is "first";
/// then seven loops are timed, the median of their 210 frames is "again".
#[test]
#[ignore = "B-406: a measurement, run deliberately with --release --ignored"]
fn b406_numbers_timing() {
    use std::fmt::Write as _;

    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n\
         | Shot | Quality | First | Again |\n|---|---|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        },
    );
    let noise = json!({"instance_id": "fx-2", "type_id": "core.noise", "enabled": true,
                       "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}});
    let shots: [(&str, Option<J>); 3] = [
        ("Noise alone", None),
        (
            "Numbers, timecode 30 at size 200, then Noise",
            Some(numbers(json!({"type": "timecode_30", "size": 200}))),
        ),
        (
            "Numbers, monospaced long date, stroke over fill, then Noise",
            Some(numbers(json!({"type": "long_date", "value": 4000,
            "size": 120, "proportional_spacing": "off", "display": "stroke_over_fill", "stroke_width": 6}))),
        ),
    ];
    let dir = plates();
    for (name, e) in shots {
        let effects: Vec<J> = e.into_iter().chain([noise.clone()]).collect();
        let project = plate((1920, 1080), 240, json!(effects));
        let mut cache = CelCache::viewer();
        gpu.forget();
        let (mut first, mut times) = (Vec::new(), Vec::new());
        for pass in 0..8 {
            for frame in (0..240).step_by(8) {
                let mut log = FrameLog::new(3);
                let t = std::time::Instant::now();
                drop(
                    preview::preview_frame_srgb8(
                        &project,
                        &Id::new(MAIN),
                        frame,
                        &dir,
                        PreviewQuality::Full,
                        DEFAULT_TILE_SIZE,
                        &mut log,
                        &mut cache,
                        &mut gpu,
                    )
                    .expect("GPU frame"),
                );
                let ms = t.elapsed().as_secs_f64() * 1000.0;
                assert!(
                    log.finish().is_empty(),
                    "{name}: the frame draws without a warning"
                );
                if pass > 0 {
                    times.push(ms)
                } else {
                    first.push(ms)
                }
            }
        }
        let _ = writeln!(
            s,
            "| {name} | Full | {:.1} | {:.1} |",
            median(first),
            median(times)
        );
    }
    let out = std::env::var("B406_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| effect_table::repo("verification/B-406_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
