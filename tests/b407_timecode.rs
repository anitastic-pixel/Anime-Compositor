//! B-407: D-527, Timecode, after After Effects' Timecode (Text): the layer's or the
//! composition's time, or a count of its own, burnt onto the picture on a box.
//!
//! Every expected frame is a project in `Fixtures/timecode/same/`, written by
//! `tools/timecode_reference.py` before this code existed: the box as a solid and the characters
//! as D-263 text layers. Each case's frame is compared sample by sample with that project's frame.
//! The card is held to the processor within 1 level of 255 (ADR-006, D-100).

mod effect_table;

use std::fs;

use effect_table::{Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{self, plan_frame, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog, Severity};
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
    layer.effects.iter().find(|i| i.instance_id.as_str() == "fx-1").unwrap().effect.clone()
}

/// Where the plates are written: the system's temporary folder.
fn plates() -> std::path::PathBuf {
    std::env::temp_dir().join("b407_plates")
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
        "schema_version": 0, "project_id": "proj-b407",
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

/// Timecode as added with `changes` laid over its settings.
fn timecode(changes: J) -> J {
    let mut p = json!({"display_format": "smpte", "time_source": "layer_source", "time_units": 30,
        "drop_frame": "off", "starting_frame": 0, "text_position": [50, 85], "text_size": 36,
        "text_color": "#ffffff", "show_box": "on", "box_color": "#000000", "opacity": 100,
        "composite_on_original": "on"});
    for (k, v) in changes.as_object().unwrap() {
        p[k] = v.clone();
    }
    json!({"instance_id": "fx-1", "type_id": "core.timecode", "enabled": true, "parameters": p})
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

#[test]
fn b407_timecode() {
    let mut t = Table::new(
        "timecode",
        "# B-407: Timecode\n\nD-527: Timecode, after After Effects' Timecode (Text): the layer's or \
         the composition's time, or a count of its own, burnt onto the picture as SMPTE timecode, a \
         frame number or feet and frames of film, on a box. Every expected frame is a project in \
         `Fixtures/timecode/same/`, written by `tools/timecode_reference.py` before this code \
         existed and printed in document 25 as FX-TC-001 to 046: the box as a solid and the \
         characters as text layers. The build's frame is compared sample by sample with that \
         project's frame; the answer is the largest difference over all of them, against the \
         catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-TC-001 to 046 (document 25): each frame against the words it must show");
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_timecode.json")).unwrap()).unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();
    let mut files = Vec::new();
    let mut shots: Vec<(String, Project, i32)> = Vec::new();
    for (name, case) in expected["cases"].as_object().unwrap() {
        let says = case["says"].as_str().unwrap();
        let file = case["project"].as_str().unwrap();
        files.push(file.to_string());
        let loaded = t.load(file);
        let mut first = None;
        for (frame, f) in case["frames"].as_object().unwrap() {
            let frame: i32 = frame.parse().unwrap();
            first.get_or_insert(frame);
            let same = t.load(f["same"].as_str().unwrap()).document;
            let built = t.render(&loaded.document, frame, 64);
            let want = t.render(&same, frame, 64);
            let d = built.data().iter().zip(want.data()).map(|(a, b)| (a - b).abs() as f64).fold(0.0, f64::max);
            let shows = f["shows"].as_str().map_or("nothing".to_string(), |s| format!("\"{s}\""));
            t.row(
                &format!("{name} frame {frame}: {says} Shows {shows}"),
                &format!("largest difference {d:.1e}"),
                d <= tolerance && built.data().len() == want.data().len(),
            );
            shots.push((format!("{file} frame {frame}"), loaded.document.project().clone(), frame));
        }
        let on_open: Vec<&str> = loaded.warnings.iter().map(|d| d.id.as_str()).collect();
        let mut log = FrameLog::new(8);
        let _ = plan_frame(loaded.document.project(), &Id::new(MAIN), first.unwrap(), &t.root, &mut log);
        let at_frame: Vec<&str> = log.finish().iter().map(|d| d.id.as_str()).collect();
        let want: Vec<&str> = case.get("warning").and_then(J::as_str).into_iter().collect();
        t.row(
            &format!("{name}: what opening it warns of, and what its first frame warns of"),
            &format!("{on_open:?} and {at_frame:?}"),
            on_open == want && at_frame == want,
        );
    }

    t.heading("A layer with a file: its own starting timecode is not read, and that is said");
    for (what, changes, want) in [
        ("layer source", json!({}), true),
        ("composition source", json!({"time_source": "composition"}), false),
        ("custom", json!({"time_source": "custom"}), false),
    ] {
        let project = plate((480, 160), 60, json!([timecode(changes)]));
        let mut log = FrameLog::new(8);
        let _ = plan_frame(&project, &Id::new(MAIN), 5, &plates(), &mut log);
        let said = log.finish();
        let info = said.iter().find(|d| d.id == DiagnosticId::ProjectFeatureUnsupported);
        t.row(
            &format!("a still with Timecode, {what}: {}", if want { "said, as information" } else { "nothing said" }),
            &info.map_or("nothing".to_string(), |d| format!("{} {}", d.severity, d.message)),
            info.is_some() == want && said.len() == want as usize && info.map_or(true, |d| d.severity == Severity::Info),
        );
    }

    t.heading("The file");
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    for file in ["fx_tc_001.json", "fx_tc_011.json"] {
        let params = t.saved_parameters(file);
        t.row(
            &format!("{file}: the count worked each frame is never saved"),
            &params.to_string(),
            params.get("count").is_none() && params.as_object().map_or(0, |m| m.len()) == 12,
        );
    }
    let refusals = [
        ("fx_tc_040.json", "Timecode's display format is one of smpte, frames, feet_frames_35mm, feet_frames_16mm, and this is \"seconds\"."),
        ("fx_tc_041.json", "Timecode's time source is one of layer_source, composition, custom, and this is \"clock\"."),
        ("fx_tc_042.json", "Timecode's time units runs from 1 to 999, and this is 0."),
        ("fx_tc_043.json", "Timecode's drop frame counts only at time units that are a multiple of 30, and this is 25."),
        ("fx_tc_044.json", "Timecode's text colour is written #rrggbb, and this is \"white\"."),
        ("fx_tc_045.json", "Timecode's opacity runs from 0 to 100, and this is 150."),
        ("fx_tc_046.json", "Timecode's starting frame runs from -10000000 to 10000000, and this is 20000000."),
    ];
    for (file, sentence) in refusals {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming it"), &why, why == sentence);
    }

    t.heading("Commands");
    let mut document = t.load("fx_tc_001.json").document;
    let base = effect_of(&document);
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = base.clone();
        f(&mut e);
        e
    };
    let refused = document.apply(set(with(&|e| if let Effect::Timecode { text_size, .. } = e { *text_size = 0.0 }))).err();
    let untouched = effect_of(&document) == base;
    t.row(
        "Text size 0 is refused with a sentence, and nothing changes",
        &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
        refused.is_some() && untouched,
    );
    t.taken(
        &mut document,
        "fx_tc_001.json",
        vec![
            ("Display Frames, Composition source,", set(with(&|e| if let Effect::Timecode { display_format, time_source, .. } = e {
                *display_format = "frames".to_string();
                *time_source = "composition".to_string();
            }))),
            ("Custom, drop frame at 60 time units, a blue box at half opacity,", set(with(&|e| if let Effect::Timecode { time_source, drop_frame, time_units, box_color, opacity, .. } = e {
                *time_source = "custom".to_string();
                *drop_frame = "on".to_string();
                *time_units = 60.0;
                *box_color = "#3050a0".to_string();
                *opacity = 50.0;
            }))),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_tc_001.json", 37), ("fx_tc_006.json", 1800), ("fx_tc_021.json", 37), ("fx_tc_024.json", 37), ("fx_tc_027.json", 10)]);

    t.heading("The card against the processor, within 1 level of 255 (ADR-006, D-100)");
    let mut gpu = Gpu::new().expect("a usable card");
    let fixtures = t.root.clone();
    let plate_dir = plates();
    let mut card: Vec<(String, Project, i32, bool)> = shots.into_iter().map(|(n, p, f)| (n, p, f, false)).collect();
    let samples: [(&str, J); 4] = [
        ("as added", json!({})),
        ("frames, a blue box at half opacity, composite off", json!({"display_format": "frames", "time_source": "composition", "box_color": "#3050a0", "opacity": 50, "composite_on_original": "off"})),
        ("custom drop frame at 30, size 60, near the top", json!({"time_source": "custom", "drop_frame": "on", "starting_frame": 107892, "text_size": 60, "text_position": [50, 40]})),
        ("feet and frames of 16mm, no box, yellow", json!({"display_format": "feet_frames_16mm", "time_source": "composition", "show_box": "off", "text_color": "#ffd040"})),
    ];
    for (name, changes) in samples {
        card.push((format!("a 480 by 160 picture, {name}"), plate((480, 160), 60, json!([timecode(changes)])), 9, true));
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

    t.heading("Pictures: in `verification/D-527 pictures/`");
    let dir = effect_table::repo("verification/D-527 pictures");
    fs::create_dir_all(&dir).unwrap();
    let pictures = [
        ("fx_tc_001.json", 37, "as_added", "as added: 00:00:01:13 in white on a black box, low in the middle of the dark blue plate"),
        ("fx_tc_006.json", 1800, "drop_frame", "29.97 drop frame at the new minute: 00:01:00;02, a semicolon before the frames"),
        ("fx_tc_011.json", 0, "custom_hour", "custom, drop frame, starting frame 107,892: 01:00:00;00"),
        ("fx_tc_013.json", 24, "negative", "custom from -50 at 25 time units: -00:00:01:00"),
        ("fx_tc_016.json", 37, "frames", "the frame number, 00037"),
        ("fx_tc_018.json", 37, "feet_35mm", "feet and frames of 35mm: 0002+05"),
        ("fx_tc_020.json", 37, "no_box", "no box: the white numbers straight on the plate"),
        ("fx_tc_021.json", 37, "blue_box", "yellow numbers on a blue box at half opacity, the plate showing through the box"),
        ("fx_tc_022.json", 37, "composite_off", "composite off: the black box and white numbers on transparent (shown on black: only the numbers show)"),
        ("fx_tc_024.json", 37, "corner", "smaller, near the top left corner, the box cut off by the picture's edges"),
        ("fx_tc_025.json", 37, "size_80", "size 80: big numbers on a big box"),
    ];
    for (file, frame, png, what) in pictures {
        let document = t.load(file).document;
        let mut log = FrameLog::new(8);
        let b = render_frame(document.project(), &Id::new(MAIN), frame, &fixtures, 64, &mut log).expect("the frame draws");
        let s = said(log);
        png_out::write_rgba(&dir.join(format!("{png}.png")), 480, 160, OutputDepth::Eight, &[], &b.to_srgb8_straight()).unwrap();
        t.row(&format!("{png}.png, {what}"), &format!("warnings {s:?}"), s.is_empty());
    }

    t.finish("D-527_timecode_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-229's way: a 1920 by 1080 still, the effect then a Noise that changes every
/// frame so nothing is kept, every eighth frame of 240 asked for as the viewer asks, whole, with
/// Draw on: GPU. The first loop starts with empty caches and its 30 frames' median is "first";
/// then seven loops are timed, the median of their 210 frames is "again".
#[test]
#[ignore = "B-407: a measurement, run deliberately with --release --ignored"]
fn b407_timecode_timing() {
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
    let noise = json!({"instance_id": "fx-2", "type_id": "core.noise", "enabled": true,
                       "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}});
    let shots: [(&str, Option<J>); 3] = [
        ("Noise alone", None),
        ("Timecode, composition source at size 200, then Noise", Some(timecode(json!({"time_source": "composition", "text_size": 200})))),
        ("Timecode, custom drop frame on a half-opacity box, composite off, then Noise", Some(timecode(json!({"time_source": "custom",
            "drop_frame": "on", "text_size": 120, "opacity": 50, "composite_on_original": "off"})))),
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
                drop(preview::preview_frame_srgb8(&project, &Id::new(MAIN), frame, &dir, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).expect("GPU frame"));
                let ms = t.elapsed().as_secs_f64() * 1000.0;
                assert!(log.finish().is_empty(), "{name}: the frame draws without a warning");
                if pass > 0 { times.push(ms) } else { first.push(ms) }
            }
        }
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B407_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-407_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
