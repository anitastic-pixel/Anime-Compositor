//! B-403: D-523, Depth of Field, after After Effects' Depth of Field (3D Channel): a render with
//! a depth pass in its EXR file blurred more the further each pixel lies from the focal plane,
//! by D-359's Lens Blur with a blur map.
//!
//! Every expected pixel is `Fixtures/depth_of_field/expected_depth_of_field.json`, written by
//! `tools/depth_of_field_reference.py` before this code existed. The card is held to the processor
//! within 1 level of 255 (ADR-006, D-100) on every fixture and on the sample render.

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
        "schema_version": 0, "project_id": "proj-b403",
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

fn dof(focal: f64, radius: f64, thickness: f64, bias: f64) -> J {
    fx(
        "fx-1",
        "core.depth_of_field",
        json!({"focal_plane": focal, "maximum_radius": radius, "focal_plane_thickness": thickness, "focal_bias": bias}),
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

fn said(log: FrameLog) -> String {
    let mut ids: Vec<&str> = log.finish().iter().map(|d| d.id.as_str()).collect();
    ids.sort();
    ids.dedup();
    ids.join(", ")
}

#[test]
fn b403_depth_of_field() {
    let mut t = Table::new(
        "depth_of_field",
        "# B-403: Depth of Field\n\nD-523: Depth of Field, after After Effects' Depth of Field \
         (3D Channel): a render with a depth pass in its EXR file blurred more the further each \
         pixel lies from the focal plane, by D-359's Lens Blur with a blur map. Every expected \
         pixel is `Fixtures/depth_of_field/expected_depth_of_field.json`, written by \
         `tools/depth_of_field_reference.py` before this code existed and printed in document 25 \
         as FX-DOF-001 to 027. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-DOF-001 to 027 (document 25)");
    t.fixtures_numbered("expected_depth_of_field.json", 1..=27);

    t.heading("The file");
    let files: Vec<String> = (1..=27).map(|n| format!("fx_dof_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    for file in ["fx_dof_002.json", "fx_dof_009.json"] {
        let params = t.saved_parameters(file);
        t.row(
            &format!("{file}: the depth read for a frame is never saved"),
            &params.to_string(),
            params.get("channels").is_none(),
        );
    }
    let why = effect_of(&t.load("fx_dof_023.json").document).why_invalid();
    let said23 = "Depth of Field's maximum radius runs from 0 to 200, and this is 201.";
    t.row("fx_dof_023.json is refused in a sentence naming it", &why, why == said23);

    t.heading("Commands");
    let mut document = t.load("fx_dof_002.json").document;
    let base = effect_of(&document);
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = base.clone();
        f(&mut e);
        e
    };
    let refused = document.apply(set(with(&|e| if let Effect::DepthOfField { focal_bias, .. } = e { *focal_bias = 101.0 }))).err();
    let untouched = effect_of(&document) == base;
    t.row(
        "Focal Bias 101 is refused with a sentence, and nothing changes",
        &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
        refused.is_some() && untouched,
    );
    t.taken(
        &mut document,
        "fx_dof_002.json",
        vec![
            ("Focal Plane 9, Thickness 2,", set(with(&|e| if let Effect::DepthOfField { focal_plane, focal_plane_thickness, .. } = e {
                *focal_plane = 9.0;
                *focal_plane_thickness = 2.0;
            }))),
            ("Maximum Radius 12, Bias 80,", set(with(&|e| if let Effect::DepthOfField { maximum_radius, focal_bias, .. } = e {
                *maximum_radius = 12.0;
                *focal_bias = 80.0;
            }))),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_dof_001.json", 0), ("fx_dof_002.json", 0), ("fx_dof_013.json", 0), ("fx_dof_019.json", 0)]);

    let fixtures = effect_table::repo("Fixtures/depth_of_field");
    let samples = effect_table::repo("Fixtures/depth_channel");
    let draw = |p: &Project| {
        let mut log = FrameLog::new(8);
        let b = render_frame(p, &Id::new(MAIN), 0, &samples, 64, &mut log).expect("the sample draws");
        (b, said(log))
    };

    t.heading("The card against the processor, within 1 level of 255 (ADR-006, D-100)");
    let mut gpu = Gpu::new().expect("a usable card");
    let mut shots: Vec<(String, Project, bool)> = Vec::new();
    for n in 1..=22 {
        let file = format!("fx_dof_{n:03}.json");
        shots.push((file.clone(), t.load(&file).document.project().clone(), false));
    }
    let settings: [(&str, J); 6] = [
        ("focused on the red ball (6), radius 8", json!([dof(6.0, 8.0, 0.0, 50.0)])),
        ("focused on the blue ball (9), radius 8", json!([dof(9.0, 8.0, 0.0, 50.0)])),
        ("focused on the yellow ball (14), thickness 2, radius 12", json!([dof(14.0, 12.0, 2.0, 50.0)])),
        ("focused on the blue ball, bias 100, radius 20", json!([dof(9.0, 20.0, 0.0, 100.0)])),
        ("focused on the blue ball, bias 10, radius 30", json!([dof(9.0, 30.0, 0.0, 10.0)])),
        ("as added (focal plane 0, radius 10)", json!([dof(0.0, 10.0, 0.0, 50.0)])),
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

    t.heading("Pictures: the sample render, in `verification/D-523 pictures/`");
    let dir = effect_table::repo("verification/D-523 pictures");
    fs::create_dir_all(&dir).unwrap();
    let shots = [
        ("before", "the sample with no effect: a red ball near, a blue one further, a yellow one far, on a checked floor", json!([])),
        ("focus_red", "Focal Plane 6, radius 8: the red ball sharp, the blue and yellow balls and the far floor soft", json!([dof(6.0, 8.0, 0.0, 50.0)])),
        ("focus_blue", "Focal Plane 9, radius 8: the blue ball sharp, the red ball and the yellow ball soft", json!([dof(9.0, 8.0, 0.0, 50.0)])),
        ("focus_yellow", "Focal Plane 14, Thickness 2, radius 12: the yellow ball sharp, the red ball very soft", json!([dof(14.0, 12.0, 2.0, 50.0)])),
        ("focus_blue_bias_100", "Focal Plane 9, Bias 100, radius 20: only a thin band of the floor by the blue ball sharp", json!([dof(9.0, 20.0, 0.0, 100.0)])),
        ("focus_blue_thick", "Focal Plane 9, Thickness 8, radius 8: everything from depth 5 to 13 sharp, the yellow ball a little soft", json!([dof(9.0, 8.0, 8.0, 50.0)])),
    ];
    for (file, what, effects) in shots {
        let (b, s) = draw(&sample(effects));
        png_out::write_rgba(&dir.join(format!("{file}.png")), 320, 180, OutputDepth::Eight, &[], &b.to_srgb8_straight()).unwrap();
        t.row(&format!("{file}.png, {what}"), &format!("warnings {s:?}"), s.is_empty());
    }

    t.finish("D-523_depth_of_field_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-229's way: a 1920 by 1080 EXR still with a picture and a depth (written
/// here, into the system's temporary folder), one layer, the effect then a Noise that changes
/// every frame so nothing is kept, every eighth frame of 240 asked for as the viewer asks, whole,
/// with Draw on: GPU. The first loop starts with empty caches and its 30 frames' median is
/// "first"; then seven loops are timed, the median of their 210 frames is "again".
#[test]
#[ignore = "B-403: a measurement, run deliberately with --release --ignored"]
fn b403_depth_of_field_timing() {
    use exr::prelude::{AnyChannel, AnyChannels, FlatSamples, Image, WritableImage};
    use std::fmt::Write as _;

    let (w, h) = (1920usize, 1080usize);
    let dir = std::env::temp_dir().join("b403_timing");
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
    let shots: [(&str, Option<J>); 4] = [
        ("Noise alone", None),
        ("Depth of Field, radius 10, then Noise", Some(dof(8.0, 10.0, 0.0, 50.0))),
        ("Depth of Field, radius 30, then Noise", Some(dof(8.0, 30.0, 0.0, 50.0))),
        ("Depth of Field, radius 60, then Noise", Some(dof(8.0, 60.0, 0.0, 50.0))),
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
    let out = std::env::var("B403_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-403_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
