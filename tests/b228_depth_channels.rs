//! B-228: D-348, the depth and normals a render saves in an EXR file (EFFECTS.md P0-5), read by
//! Pass Extract (after After Effects' 3D Channel Extract) and Depth Key (after Depth Matte).
//!
//! Every expected pixel is `Fixtures/depth_channel/expected_depth_channel.json`, written by
//! `tools/depth_channel_reference.py` before this code existed. The card is held to the
//! processor within 1 level of 255 (ADR-006, D-100) on every fixture and on the sample render.

mod effect_table;

use std::fs;

use effect_table::{Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// Set the `scene` layer's effect `fx-1`, as the fixtures name it.
fn set(effect: Effect) -> Command {
    Command::SetEffectParameters {
        composition: Id::new(MAIN),
        layer_id: Id::new("scene"),
        instance_id: Id::new("fx-1"),
        effect,
    }
}

fn effect_of(d: &Document) -> Effect {
    let comp = d.project().composition(&Id::new(MAIN)).unwrap();
    let layer = comp.layer(&Id::new("scene")).unwrap();
    layer.effects.iter().find(|i| i.instance_id.as_str() == "fx-1").unwrap().effect.clone()
}

/// The sample render (`Fixtures/depth_channel/sample/spheres.exr`, 320 by 180) as the one layer
/// of a composition its size, carrying `effects`.
fn sample(effects: J) -> Project {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let project = json!({
        "schema_version": 0, "project_id": "proj-b228-sample",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-spheres", "kind": "still", "name": "spheres", "path": "sample/spheres.exr",
                    "interpretation": {"color_space": "linear-srgb", "alpha": "premultiplied"}}],
        "compositions": [{
            "id": MAIN, "name": "Main", "width": 320, "height": 180, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 5,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 5},
            "layer_order": ["scene"],
            "layers": [{"id": "scene", "kind": "raster", "name": "scene", "asset_id": "asset-spheres", "enabled": true,
                "locked": false, "in_frame": 0, "out_frame": 5, "source_offset_frames": 0,
                "transform": {"anchor": t(json!([0, 0])), "position": t(json!([0, 0])), "scale": t(json!([100, 100])),
                              "rotation": t(json!(0)), "opacity": t(json!(1))},
                "exposure_spans": [], "mask": null, "matte": null, "blend_mode": "normal", "effects": effects}]
        }]
    });
    persist::load_str(&project.to_string()).expect("the sample's project reads").document.project().clone()
}

fn fx(id: &str, type_id: &str, p: J) -> J {
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": p})
}

fn extract(pass: &str, black: f64, white: f64, invert: &str, clamp: &str) -> J {
    fx("fx-1", "core.pass_extract", json!({"pass": pass, "black_point": black, "white_point": white, "invert": invert, "clamp": clamp}))
}

fn key(depth: f64, feather: f64, invert: &str) -> J {
    fx("fx-1", "core.depth_key", json!({"depth": depth, "feather": feather, "invert": invert}))
}

fn blur() -> J {
    fx("fx-0", "core.gaussian_blur", json!({"sigma_px": 6}))
}

/// The largest channel difference in levels, and how many pixels differ.
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
fn b228_depth_channels() {
    let mut t = Table::new(
        "depth_channel",
        "# B-228: depth and normals from an EXR file\n\nD-348 (EFFECTS.md P0-5): the depth and \
         normals a render saves in an EXR file, read by Pass Extract (after After Effects' 3D \
         Channel Extract) and Depth Key (after Depth Matte). Every expected pixel is \
         `Fixtures/depth_channel/expected_depth_channel.json`, written by \
         `tools/depth_channel_reference.py` before this code existed and printed in document 25 \
         as FX-DEPTH-001 to 036. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-DEPTH-001 to 036 (document 25)");
    t.fixtures("expected_depth_channel.json");

    t.heading("The file");
    let files: Vec<String> = (1..=36).map(|n| format!("fx_depth_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    for file in ["fx_depth_001.json", "fx_depth_022.json"] {
        let params = t.saved_parameters(file);
        t.row(
            &format!("{file}: the pass read for a frame is never saved"),
            &params.to_string(),
            params.get("channels").is_none(),
        );
    }
    for (file, said) in [
        ("fx_depth_032.json", "Pass Extract's pass is \"depth\" or \"normals\", and this is \"uv\"."),
        ("fx_depth_033.json", "Pass Extract's clamp is \"off\" or \"on\", and this is \"yes\"."),
        ("fx_depth_034.json", "Pass Extract's black point runs from -1000000 to 1000000, and this is 2000000."),
        ("fx_depth_035.json", "Depth Key's feather runs from 0 to 1000000, and this is -1."),
        ("fx_depth_036.json", "Depth Key's invert is \"off\" or \"on\", and this is \"yes\"."),
    ] {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming it"), &why, why == said);
    }

    t.heading("Commands");
    let mut document = t.load("fx_depth_002.json").document;
    let base = effect_of(&document);
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = base.clone();
        f(&mut e);
        e
    };
    for (what, effect) in [
        ("pass \"uv\"", with(&|e| if let Effect::PassExtract { pass, .. } = e { *pass = "uv".into() })),
        ("white point 2,000,000", with(&|e| if let Effect::PassExtract { white_point, .. } = e { *white_point = 2e6 })),
    ] {
        let refused = document.apply(set(effect)).err();
        let untouched = effect_of(&document) == base;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && untouched,
        );
    }
    t.taken(
        &mut document,
        "fx_depth_002.json",
        vec![
            ("the normals,", set(with(&|e| if let Effect::PassExtract { pass, .. } = e { *pass = "normals".into() }))),
            ("black point -1, invert on,", set(with(&|e| if let Effect::PassExtract { black_point, invert, .. } = e {
                *black_point = -1.0;
                *invert = "on".into();
            }))),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_depth_002.json", 0), ("fx_depth_007.json", 0), ("fx_depth_009.json", 0), ("fx_depth_025.json", 0)]);

    t.heading("A blur before Pass Extract grows the drawing; the pass still lies where it was");
    let root = effect_table::repo("Fixtures/depth_channel");
    let draw = |p: &Project| {
        let mut log = FrameLog::new(8);
        let b = render_frame(p, &Id::new(MAIN), 0, &root, 64, &mut log).expect("the sample draws");
        (b, said(log))
    };
    for (what, e) in [("depth", extract("depth", 2.0, 16.0, "off", "on")), ("normals", extract("normals", -1.0, 1.0, "off", "off"))] {
        let (plain, s1) = draw(&sample(json!([e.clone()])));
        let (blurred, s2) = draw(&sample(json!([blur(), e])));
        let same = plain.data() == blurred.data();
        t.row(
            &format!("the sample's {what}, with and without a Gaussian Blur of 6 before it"),
            &format!("{}, warnings {s1:?} and {s2:?}", if same { "byte-identical" } else { "differ" }),
            same && s1.is_empty() && s2.is_empty(),
        );
    }

    t.heading("The card against the processor, within 1 level of 255 (ADR-006, D-100)");
    let mut gpu = Gpu::new().expect("a usable card");
    let mut shots: Vec<(String, Project, bool)> = Vec::new();
    for n in 1..=31 {
        let file = format!("fx_depth_{n:03}.json");
        shots.push((file.clone(), t.load(&file).document.project().clone(), false));
    }
    let settings: [(&str, J); 8] = [
        ("Pass Extract, depth 2 to 16", json!([extract("depth", 2.0, 16.0, "off", "on")])),
        ("Pass Extract, depth 16 to 2, clamp off", json!([extract("depth", 16.0, 2.0, "off", "off")])),
        ("Pass Extract, normals -1 to 1", json!([extract("normals", -1.0, 1.0, "off", "on")])),
        ("Pass Extract, normals inverted, a cut at 0", json!([extract("normals", 0.0, 0.0, "on", "on")])),
        ("Pass Extract, depth after a Gaussian Blur of 6", json!([blur(), extract("depth", 2.0, 16.0, "off", "on")])),
        ("Depth Key at 8, feather 3", json!([key(8.0, 3.0, "off")])),
        ("Depth Key at 10, inverted, no feather", json!([key(10.0, 0.0, "on")])),
        ("Depth Key at 8, feather 3, after a Gaussian Blur of 6", json!([blur(), key(8.0, 3.0, "off")])),
    ];
    for (name, effects) in settings {
        shots.push((format!("the sample, {name}"), sample(effects), true));
    }
    let mut worst = 0u8;
    for (name, project, is_sample) in &shots {
        let comp = &project.compositions[0];
        let frames: Vec<i32> = (comp.start_frame..comp.start_frame + comp.duration_frames as i32).collect();
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for &frame in frames.iter().filter(|&&f| *is_sample || f == 0 || f == 4) {
                let mut cache = CelCache::viewer();
                let mut log = FrameLog::new(3);
                let c = preview::preview_frame_cached(project, &comp.id, frame, &root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache)
                    .unwrap_or_else(|d| panic!("{name} frame {frame} on the CPU: {}", d.message));
                let said_cpu = said(log);
                let c = c.to_srgb8_straight();
                let mut log = FrameLog::new(3);
                let (g, ..) = preview::preview_frame_srgb8(project, &comp.id, frame, &root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu)
                    .unwrap_or_else(|d| panic!("{name} frame {frame} on the GPU: {}", d.message));
                let said_gpu = said(log);
                let mut log = FrameLog::new(3);
                let plan = compose::plan_frame_for_card(project, &comp.id, frame, &root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
                let on_card: usize = plan.layers.iter().map(|l| l.on_card.len()).sum();
                let effects = comp.layer(&Id::new("scene")).map_or(0, |l| l.effects.len());
                let d = distance(&c, &g);
                worst = worst.max(d.0);
                let fallback = said_gpu.contains(DiagnosticId::GpuPreviewOnCpu.as_str());
                t.row(
                    &format!("{name} frame {frame}, {}", quality.label()),
                    &format!("largest {} level(s), {} pixel(s) differ; {on_card} of {effects} effect(s) on the card; warnings {said_gpu:?}", d.0, d.1),
                    d.0 <= 1 && !fallback && said_cpu == said_gpu && (!is_sample || on_card == effects),
                );
            }
        }
    }
    t.row("the largest difference over every frame above", &format!("{worst} level(s)"), worst <= 1);

    t.heading("Pictures: the sample render, in `verification/D-348 pictures/`");
    let dir = effect_table::repo("verification/D-348 pictures");
    fs::create_dir_all(&dir).unwrap();
    let shots = [
        ("before", "the render's colour, as it is drawn without an effect: three balls on a floor, the sky clear (white in a viewer that shows clear as white)", json!([])),
        ("depth", "Pass Extract, depth, Black Point 2, White Point 16: near dark, far light, the sky white", json!([extract("depth", 2.0, 16.0, "off", "on")])),
        ("normals", "Pass Extract, normals, Black -1, White 1: what faces right reddish, up greenish, the camera bluish; the floor all one pale green, the sky (no surface) mid grey", json!([extract("normals", -1.0, 1.0, "off", "on")])),
        ("key_near_out", "Depth Key at 8, Feather 3: the red ball in front and the near floor taken out (clear), softly", json!([key(8.0, 3.0, "off")])),
        ("key_far_out", "Depth Key at 8, Feather 3, Invert: only the red ball and the near floor left", json!([key(8.0, 3.0, "on")])),
    ];
    for (file, what, effects) in shots {
        let (b, s) = draw(&sample(effects));
        png_out::write_rgba(&dir.join(format!("{file}.png")), 320, 180, OutputDepth::Eight, &[], &b.to_srgb8_straight()).unwrap();
        t.row(&format!("{file}.png, {what}"), &format!("warnings {s:?}"), s.is_empty());
    }

    t.finish("D-348_depth_channels_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-227's way: a 1920 by 1080 EXR still with a depth and normals (written here, into the
/// system's temporary folder), one layer, the effect then a Noise that changes every frame so
/// nothing is kept, every eighth frame of 240 asked for as the viewer asks, whole, with Draw on:
/// GPU. The first loop starts with empty caches and its 30 frames' median is "first"; then seven
/// loops are timed, the median of their 210 frames is "again".
#[test]
#[ignore = "B-228: a measurement, run deliberately with --release --ignored"]
fn b228_depth_channels_timing() {
    use exr::prelude::{AnyChannel, AnyChannels, FlatSamples, Image, WritableImage};
    use std::fmt::Write as _;

    let (w, h) = (1920usize, 1080usize);
    let dir = std::env::temp_dir().join("b228_timing");
    fs::create_dir_all(&dir).unwrap();
    let plane = |f: &dyn Fn(usize, usize) -> f32| (0..w * h).map(|i| f(i % w, i / w)).collect::<Vec<f32>>();
    let half = |v: Vec<f32>| FlatSamples::F16(v.into_iter().map(exr::prelude::f16::from_f32).collect());
    let channels = AnyChannels::sort(
        vec![
            AnyChannel::new("R", half(plane(&|x, _| x as f32 / w as f32))),
            AnyChannel::new("G", half(plane(&|_, y| y as f32 / h as f32))),
            AnyChannel::new("B", half(plane(&|_, _| 0.3))),
            AnyChannel::new("A", half(plane(&|_, _| 1.0))),
            AnyChannel::new("Z", FlatSamples::F32(plane(&|x, y| 1.0 + (x + y) as f32 / 100.0))),
            AnyChannel::new("N.x", half(plane(&|x, _| x as f32 / w as f32 - 0.5))),
            AnyChannel::new("N.y", half(plane(&|_, y| 0.5 - y as f32 / h as f32))),
            AnyChannel::new("N.z", half(plane(&|_, _| 0.7))),
        ]
        .into(),
    );
    Image::from_channels((w, h), channels).write().to_file(dir.join("depth.exr")).expect("write the timing EXR");

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
    let noise = fx("fx-2", "core.noise", json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}));
    let shots: [(&str, Option<J>); 4] = [
        ("Noise alone", None),
        ("Pass Extract, depth 0 to 50, then Noise", Some(extract("depth", 0.0, 50.0, "off", "on"))),
        ("Pass Extract, normals, then Noise", Some(extract("normals", -1.0, 1.0, "off", "on"))),
        ("Depth Key at 15, feather 5, then Noise", Some(key(15.0, 5.0, "off"))),
    ];
    for (name, e) in shots {
        let effects: Vec<J> = e.into_iter().chain([noise.clone()]).collect();
        let t = |v: J| json!({"base": v, "keyframes": []});
        let project = json!({
            "schema_version": 0, "project_id": "proj-b228-timing",
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [{"id": "asset-depth", "kind": "still", "name": "depth", "path": "depth.exr",
                        "interpretation": {"color_space": "linear-srgb", "alpha": "premultiplied"}}],
            "compositions": [{
                "id": MAIN, "name": "Main", "width": w, "height": h, "pixel_aspect_ratio": 1,
                "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 240,
                "work_area": {"start_frame": 0, "end_frame_exclusive": 240},
                "layer_order": ["scene"],
                "layers": [{"id": "scene", "kind": "raster", "name": "scene", "asset_id": "asset-depth", "enabled": true,
                    "locked": false, "in_frame": 0, "out_frame": 240, "source_offset_frames": 0,
                    "transform": {"anchor": t(json!([0, 0])), "position": t(json!([0, 0])), "scale": t(json!([100, 100])),
                                  "rotation": t(json!(0)), "opacity": t(json!(1))},
                    "exposure_spans": [], "mask": null, "matte": null, "blend_mode": "normal", "effects": effects}]
            }]
        });
        let project = persist::load_str(&project.to_string()).unwrap_or_else(|d| panic!("{name}: {}", d.message)).document.project().clone();
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
    let out = std::env::var("B228_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-228_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
