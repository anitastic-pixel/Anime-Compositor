//! B-404: D-524, Fog 3D, after After Effects' Fog 3D (3D Channel): a render with a depth pass in
//! its EXR file laid over with fog that thickens with distance, thinned where a grey gradient
//! layer is dark.
//!
//! Every expected pixel is `Fixtures/fog_3d/expected_fog_3d.json`, written by
//! `tools/fog_3d_reference.py` before this code existed. The card is held to the processor within
//! 1 level of 255 (ADR-006, D-100) on every fixture and on the sample render.

mod effect_table;

use std::fs;

use effect_table::{median, said, Table, MAIN};
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

/// A one-layer composition `w` by `h` of the still `path`, carrying `effects`.
fn still(path: &str, (w, h): (usize, usize), frames: u32, effects: J) -> Project {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let project = json!({
        "schema_version": 0, "project_id": "proj-b404",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-scene", "kind": "still", "name": "scene", "path": path,
                    "interpretation": {"color_space": "linear-srgb", "alpha": "premultiplied"}}],
        "compositions": [{
            "id": MAIN, "name": "Main", "width": w, "height": h, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": frames,
            "work_area": {"start_frame": 0, "end_frame_exclusive": frames},
            "layer_order": ["scene"],
            "layers": [{"id": "scene", "kind": "raster", "name": "scene", "asset_id": "asset-scene", "enabled": true,
                "locked": false, "in_frame": 0, "out_frame": frames, "source_offset_frames": 0,
                "transform": {"anchor": t(json!([0, 0])), "position": t(json!([0, 0])), "scale": t(json!([100, 100])),
                              "rotation": t(json!(0)), "opacity": t(json!(1))},
                "exposure_spans": [], "mask": null, "matte": null, "blend_mode": "normal", "effects": effects}]
        }]
    });
    persist::load_str(&project.to_string()).expect("the project reads").document.project().clone()
}

/// The playtest's sample, the depth fixtures' `sample/spheres.exr` (320 by 180): balls at
/// depths about 6, 9 and 14, the sky at 1,000; drawn from `Fixtures/depth_channel`.
fn sample(effects: J) -> Project {
    still("sample/spheres.exr", (320, 180), 5, effects)
}

fn fx(id: &str, type_id: &str, p: J) -> J {
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": p})
}

/// Fog 3D with no gradient: the colour, start, end, opacity, density and Foggy Background.
fn fog(color: &str, start: f64, end: f64, opacity: f64, density: f64, background: &str) -> J {
    fx(
        "fx-1",
        "core.fog_3d",
        json!({"fog_color": color, "fog_start_depth": start, "fog_end_depth": end, "fog_opacity": opacity,
               "scattering_density": density, "foggy_background": background, "gradient_layer": "",
               "layer_contribution": 100}),
    )
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
fn b404_fog_3d() {
    let mut t = Table::new(
        "fog_3d",
        "# B-404: Fog 3D\n\nD-524: Fog 3D, after After Effects' Fog 3D (3D Channel): a render \
         with a depth pass in its EXR file laid over with fog that thickens with distance, thinned \
         where a grey gradient layer is dark. Every expected pixel is \
         `Fixtures/fog_3d/expected_fog_3d.json`, written by `tools/fog_3d_reference.py` before \
         this code existed and printed in document 25 as FX-FOG-001 to 030. The build's frame is \
         compared sample by sample; the answer is the largest difference over all of them, \
         against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-FOG-001 to 030 (document 25)");
    t.fixtures_numbered("expected_fog_3d.json", 1..=30);

    t.heading("The file");
    let files: Vec<String> = (1..=30).map(|n| format!("fx_fog_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    for file in ["fx_fog_001.json", "fx_fog_010.json"] {
        let params = t.saved_parameters(file);
        t.row(
            &format!("{file}: the depth and gradient read for a frame are never saved"),
            &params.to_string(),
            params.get("channels").is_none() && params.get("map").is_none(),
        );
    }
    let why = effect_of(&t.load("fx_fog_025.json").document).why_invalid();
    let said25 = "Fog 3D's fog opacity runs from 0 to 100, and this is 101.";
    t.row("fx_fog_025.json is refused in a sentence naming it", &why, why == said25);
    let why = effect_of(&t.load("fx_fog_027.json").document).why_invalid();
    let said27 = "Fog 3D's foggy background is \"on\" or \"off\", and this is \"yes\".";
    t.row("fx_fog_027.json is refused in a sentence naming it", &why, why == said27);
    let why = effect_of(&t.load("fx_fog_028.json").document).why_invalid();
    let said28 = "Fog 3D's Fog Color is written #rrggbb, and this is \"white\".";
    t.row("fx_fog_028.json is refused in a sentence naming it", &why, why == said28);

    t.heading("Commands");
    let mut document = t.load("fx_fog_002.json").document;
    let base = effect_of(&document);
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = base.clone();
        f(&mut e);
        e
    };
    let refused = document.apply(set(with(&|e| if let Effect::Fog3d { layer_contribution, .. } = e { *layer_contribution = 101.0 }))).err();
    let untouched = effect_of(&document) == base;
    t.row(
        "Layer Contribution 101 is refused with a sentence, and nothing changes",
        &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
        refused.is_some() && untouched,
    );
    t.taken(
        &mut document,
        "fx_fog_002.json",
        vec![
            ("Start 4, End 12,", set(with(&|e| if let Effect::Fog3d { fog_start_depth, fog_end_depth, .. } = e {
                *fog_start_depth = 4.0;
                *fog_end_depth = 12.0;
            }))),
            ("Colour #4060a0, Foggy Background off, the ramp at 50,", set(with(&|e| if let Effect::Fog3d { fog_color, foggy_background, gradient_layer, layer_contribution, .. } = e {
                *fog_color = "#4060a0".to_string();
                *foggy_background = "off".to_string();
                *gradient_layer = json!("grad");
                *layer_contribution = 50.0;
            }))),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_fog_001.json", 0), ("fx_fog_010.json", 0), ("fx_fog_014.json", 2), ("fx_fog_022.json", 0)]);

    let fixtures = effect_table::repo("Fixtures/fog_3d");
    let samples = effect_table::repo("Fixtures/depth_channel");
    let draw = |p: &Project| {
        let mut log = FrameLog::new(8);
        let b = render_frame(p, &Id::new(MAIN), 0, &samples, 64, &mut log).expect("the sample draws");
        (b, said(log))
    };

    t.heading("The card against the processor, within 1 level of 255 (ADR-006, D-100)");
    let mut gpu = Gpu::new().expect("a usable card");
    let mut shots: Vec<(String, Project, bool)> = Vec::new();
    for n in 1..=24 {
        let file = format!("fx_fog_{n:03}.json");
        shots.push((file.clone(), t.load(&file).document.project().clone(), false));
    }
    let settings: [(&str, J); 5] = [
        ("as added (white, 0 to 10, density 50)", json!([fog("#ffffff", 0.0, 10.0, 100.0, 50.0, "on")])),
        ("blue-grey from 5 to 15, density 0", json!([fog("#8090a8", 5.0, 15.0, 100.0, 0.0, "on")])),
        ("white from 4 to 30, density 100", json!([fog("#ffffff", 4.0, 30.0, 100.0, 100.0, "on")])),
        ("warm from 6 to 14, opacity 60", json!([fog("#d8b890", 6.0, 14.0, 60.0, 30.0, "on")])),
        ("from 14 to 6, the other way round, background off", json!([fog("#ffffff", 14.0, 6.0, 100.0, 0.0, "off")])),
    ];
    for (name, effects) in settings {
        shots.push((format!("the sample, {name}"), sample(effects), true));
    }
    let mut worst = 0u8;
    for (name, project, is_sample) in &shots {
        let comp = &project.compositions[0];
        let root = if *is_sample { &samples } else { &fixtures };
        let frames: Vec<i32> = (comp.start_frame..comp.start_frame + comp.duration_frames as i32).collect();
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for &frame in frames.iter().filter(|&&f| f == 0 || f == 4) {
                let mut cache = CelCache::viewer();
                let mut log = FrameLog::new(3);
                let c = preview::preview_frame_cached(project, &comp.id, frame, root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache)
                    .unwrap_or_else(|d| panic!("{name} frame {frame} on the CPU: {}", d.message));
                let said_cpu = said(log);
                let c = c.to_srgb8_straight();
                let mut log = FrameLog::new(3);
                let (g, ..) = preview::preview_frame_srgb8(project, &comp.id, frame, root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu)
                    .unwrap_or_else(|d| panic!("{name} frame {frame} on the GPU: {}", d.message));
                let said_gpu = said(log);
                let mut log = FrameLog::new(3);
                let plan = compose::plan_frame_for_card(project, &comp.id, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
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

    t.heading("Pictures: the sample render, in `verification/D-524 pictures/`");
    let dir = effect_table::repo("verification/D-524 pictures");
    fs::create_dir_all(&dir).unwrap();
    let shots = [
        ("before", "the sample with no effect: a red ball near, a blue one further, a yellow one far, on a checked floor", json!([])),
        ("as_added", "as added, white fog from depth 0 to 10 at density 50: thick almost at once, only the near floor's checks clear, the red ball a faint pink, the blue ball barely seen, the rest white", json!([fog("#ffffff", 0.0, 10.0, 100.0, 50.0, "on")])),
        ("blue_grey_5_to_15", "blue-grey fog from 5 to 15, even: the red ball almost clear, the yellow ball deep in fog, the horizon blue-grey", json!([fog("#8090a8", 5.0, 15.0, 100.0, 0.0, "on")])),
        ("dense_4_to_30", "white fog from 4 to 30 at density 100: thick soon after depth 4, every ball faded", json!([fog("#ffffff", 4.0, 30.0, 100.0, 100.0, "on")])),
        ("warm_opacity_60", "warm fog from 6 to 14 at opacity 60: the far part tinted, never fully hidden", json!([fog("#d8b890", 6.0, 14.0, 60.0, 30.0, "on")])),
        ("reversed", "white fog from 14 to 6, the other way round: the near floor and the red ball white, the blue ball pale, the yellow ball, the far floor and the sky clear", json!([fog("#ffffff", 14.0, 6.0, 100.0, 0.0, "off")])),
    ];
    for (file, what, effects) in shots {
        let (b, s) = draw(&sample(effects));
        png_out::write_rgba(&dir.join(format!("{file}.png")), 320, 180, OutputDepth::Eight, &[], &b.to_srgb8_straight()).unwrap();
        t.row(&format!("{file}.png, {what}"), &format!("warnings {s:?}"), s.is_empty());
    }

    t.finish("D-524_fog_3d_table.md");
}

/// The frame times, B-229's way: a 1920 by 1080 EXR still with a picture and a depth (written
/// here, into the system's temporary folder), one layer, the effect then a Noise that changes
/// every frame so nothing is kept, every eighth frame of 240 asked for as the viewer asks, whole,
/// with Draw on: GPU. The first loop starts with empty caches and its 30 frames' median is
/// "first"; then seven loops are timed, the median of their 210 frames is "again".
#[test]
#[ignore = "B-404: a measurement, run deliberately with --release --ignored"]
fn b404_fog_3d_timing() {
    use exr::prelude::{AnyChannel, AnyChannels, FlatSamples, Image, WritableImage};
    use std::fmt::Write as _;

    let (w, h) = (1920usize, 1080usize);
    let dir = std::env::temp_dir().join("b404_timing");
    fs::create_dir_all(&dir).unwrap();
    let plane = |f: &dyn Fn(usize, usize) -> f32| (0..w * h).map(|i| f(i % w, i / w)).collect::<Vec<f32>>();
    let half = |v: Vec<f32>| FlatSamples::F16(v.into_iter().map(exr::prelude::f16::from_f32).collect());
    let channels = AnyChannels::sort(
        vec![
            AnyChannel::new("R", half(plane(&|x, y| ((x / 60 + y / 60) % 2) as f32))),
            AnyChannel::new("G", half(plane(&|_, y| y as f32 / h as f32))),
            AnyChannel::new("B", half(plane(&|_, _| 0.3))),
            AnyChannel::new("A", half(plane(&|_, _| 1.0))),
            // Depth from 2 at the left to 20 at the right.
            AnyChannel::new("Z", FlatSamples::F32(plane(&|x, _| 2.0 + 18.0 * x as f32 / w as f32))),
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
    let shots: [(&str, Option<J>); 3] = [
        ("Noise alone", None),
        ("Fog 3D as added, then Noise", Some(fog("#ffffff", 0.0, 10.0, 100.0, 50.0, "on"))),
        ("Fog 3D from 4 to 20, density 0, then Noise", Some(fog("#8090a8", 4.0, 20.0, 100.0, 0.0, "on"))),
    ];
    for (name, e) in shots {
        let effects: Vec<J> = e.into_iter().chain([noise.clone()]).collect();
        let project = still("depth.exr", (w, h), 240, json!(effects));
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
    let out = std::env::var("B404_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-404_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
