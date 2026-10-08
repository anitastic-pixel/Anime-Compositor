//! B-229: D-349, the object and material ids and any named channel a render saves in an EXR file
//! (EFFECTS.md P0-5, part 2), read by ID Key (after After Effects' ID Matte) and Pass Extract,
//! and Blender's multilayer files taken as a picture.
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
        "schema_version": 0, "project_id": "proj-b229",
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

/// The playtest's sample, Blender's layout (`sample/spheres_blender.exr`, 320 by 180).
fn sample(effects: J) -> Project {
    still("sample/spheres_blender.exr", (320, 180), 5, effects)
}

fn fx(id: &str, type_id: &str, p: J) -> J {
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": p})
}

fn key(aux: &str, id: f64, feather: f64, invert: &str) -> J {
    fx("fx-1", "core.id_key", json!({"aux_channel": aux, "id": id, "feather": feather, "invert": invert}))
}

fn extract(pass: &str, black: f64, white: f64, channel: &str) -> J {
    fx("fx-1", "core.pass_extract", json!({"pass": pass, "black_point": black, "white_point": white,
                                          "invert": "off", "clamp": "on", "channel": channel}))
}

fn blur() -> J {
    fx("fx-0", "core.gaussian_blur", json!({"sigma_px": 6}))
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
fn b229_id_key() {
    let mut t = Table::new(
        "depth_channel",
        "# B-229: object and material ids, named channels and Blender's files\n\nD-349 \
         (EFFECTS.md P0-5, part 2): the object and material ids and any named channel a render \
         saves in an EXR file, read by ID Key (after After Effects' ID Matte) and Pass Extract, \
         and Blender's multilayer files taken as a picture. Every expected pixel is \
         `Fixtures/depth_channel/expected_depth_channel.json`, written by \
         `tools/depth_channel_reference.py` before this code existed and printed in document 25 \
         as FX-DEPTH-037 to 064. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-DEPTH-037 to 064 (document 25)");
    t.fixtures_numbered("expected_depth_channel.json", 37..=64);

    t.heading("The file");
    let files: Vec<String> = (37..=64).map(|n| format!("fx_depth_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    for file in ["fx_depth_038.json", "fx_depth_052.json"] {
        let params = t.saved_parameters(file);
        t.row(
            &format!("{file}: the ids or channel read for a frame are never saved"),
            &params.to_string(),
            params.get("channels").is_none(),
        );
    }
    let params = t.saved_parameters("fx_depth_050.json");
    t.row(
        "fx_depth_050.json: a Pass Extract with no channel names writes none",
        &params.to_string(),
        params.get("channel").is_none(),
    );
    for (file, said) in [
        ("fx_depth_061.json", "ID Key's channel is \"object_id\" or \"material_id\", and this is \"uv\"."),
        ("fx_depth_062.json", "ID Key's id runs from 0 to 1000000, and this is -1."),
        ("fx_depth_063.json", "ID Key's feather runs from 0 to 100, and this is 101."),
        ("fx_depth_064.json", "ID Key's invert is \"off\" or \"on\", and this is \"yes\"."),
    ] {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming it"), &why, why == said);
    }

    t.heading("Commands");
    let mut document = t.load("fx_depth_038.json").document;
    let base = effect_of(&document);
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = base.clone();
        f(&mut e);
        e
    };
    for (what, effect) in [
        ("channel \"uv\"", with(&|e| if let Effect::IdKey { aux_channel, .. } = e { *aux_channel = "uv".into() })),
        ("feather 101", with(&|e| if let Effect::IdKey { feather, .. } = e { *feather = 101.0 })),
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
        "fx_depth_038.json",
        vec![
            ("the material ids,", set(with(&|e| if let Effect::IdKey { aux_channel, .. } = e { *aux_channel = "material_id".into() }))),
            ("id 2, feather 1.5, invert on,", set(with(&|e| if let Effect::IdKey { id, feather, invert, .. } = e {
                *id = 2.0;
                *feather = 1.5;
                *invert = "on".into();
            }))),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_depth_038.json", 0), ("fx_depth_041.json", 0), ("fx_depth_049.json", 0), ("fx_depth_053.json", 0)]);

    let root = effect_table::repo("Fixtures/depth_channel");
    let draw = |p: &Project| {
        let mut log = FrameLog::new(8);
        let b = render_frame(p, &Id::new(MAIN), 0, &root, 64, &mut log).expect("the sample draws");
        (b, said(log))
    };

    t.heading("The card against the processor, within 1 level of 255 (ADR-006, D-100)");
    let mut gpu = Gpu::new().expect("a usable card");
    let mut shots: Vec<(String, Project, bool)> = Vec::new();
    for n in 37..=60 {
        let file = format!("fx_depth_{n:03}.json");
        shots.push((file.clone(), t.load(&file).document.project().clone(), false));
    }
    let settings: [(&str, J); 9] = [
        ("ID Key, object 1", json!([key("object_id", 1.0, 0.0, "off")])),
        ("ID Key, object 4 (the floor), feather 3", json!([key("object_id", 4.0, 3.0, "off")])),
        ("ID Key, object 2, inverted, feather 5", json!([key("object_id", 2.0, 5.0, "on")])),
        ("ID Key, material 3, feather 1.5", json!([key("material_id", 3.0, 1.5, "off")])),
        ("ID Key, object 3, feather 3, after a Gaussian Blur of 6", json!([blur(), key("object_id", 3.0, 3.0, "off")])),
        ("Pass Extract, object ids 0 to 4", json!([extract("object_id", 0.0, 4.0, "")])),
        ("Pass Extract, material ids 0 to 4", json!([extract("material_id", 0.0, 4.0, "")])),
        ("Pass Extract, ViewLayer.Mist.Z", json!([extract("named", 0.0, 1.0, "ViewLayer.Mist.Z")])),
        ("Pass Extract, depth", json!([extract("depth", 2.0, 16.0, "")])),
    ];
    for (name, effects) in settings {
        shots.push((format!("the sample, {name}"), sample(effects), true));
    }
    let mut worst = 0u8;
    for (name, project, is_sample) in &shots {
        let comp = &project.compositions[0];
        let frames: Vec<i32> = (comp.start_frame..comp.start_frame + comp.duration_frames as i32).collect();
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for &frame in frames.iter().filter(|&&f| f == 0 || f == 4) {
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

    t.heading("Pictures: the sample render in Blender's layout, in `verification/D-349 pictures/`");
    let dir = effect_table::repo("verification/D-349 pictures");
    fs::create_dir_all(&dir).unwrap();
    let shots = [
        ("before", "the Blender-style file with no effect: the same three balls on a floor as D-348's before.png, its colour taken from ViewLayer.Combined", json!([])),
        ("object_ids", "Pass Extract, Object ID, Black 0, White 4: the sky black, the three balls three greys, the floor white", json!([extract("object_id", 0.0, 4.0, "")])),
        ("material_ids", "Pass Extract, Material ID, Black 0, White 4: the floor's squares two greys, the red and blue balls a very light grey, the yellow ball white", json!([extract("material_id", 0.0, 4.0, "")])),
        ("mist", "Pass Extract, Named Channel ViewLayer.Mist.Z: near dark, far light, the sky white", json!([extract("named", 0.0, 1.0, "ViewLayer.Mist.Z")])),
        ("key_object_2", "ID Key, Object ID 2: one ball alone, everything else clear", json!([key("object_id", 2.0, 0.0, "off")])),
        ("key_object_2_inverted_soft", "ID Key, Object ID 2, Feather 3, Invert: that ball cut out of the picture, its edge soft", json!([key("object_id", 2.0, 3.0, "on")])),
        ("key_material_3", "ID Key, Material ID 3: the red and blue balls together (they share a material), the rest clear", json!([key("material_id", 3.0, 0.0, "off")])),
    ];
    for (file, what, effects) in shots {
        let (b, s) = draw(&sample(effects));
        png_out::write_rgba(&dir.join(format!("{file}.png")), 320, 180, OutputDepth::Eight, &[], &b.to_srgb8_straight()).unwrap();
        t.row(&format!("{file}.png, {what}"), &format!("warnings {s:?}"), s.is_empty());
    }

    t.finish("D-349_id_key_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-228's way: a 1920 by 1080 EXR still laid out as Blender writes it, with
/// object ids (written here, into the system's temporary folder), one layer, the effect then a
/// Noise that changes every frame so nothing is kept, every eighth frame of 240 asked for as the
/// viewer asks, whole, with Draw on: GPU. The first loop starts with empty caches and its 30
/// frames' median is "first"; then seven loops are timed, the median of their 210 frames is
/// "again".
#[test]
#[ignore = "B-229: a measurement, run deliberately with --release --ignored"]
fn b229_id_key_timing() {
    use exr::prelude::{AnyChannel, AnyChannels, FlatSamples, Image, WritableImage};
    use std::fmt::Write as _;

    let (w, h) = (1920usize, 1080usize);
    let dir = std::env::temp_dir().join("b229_timing");
    fs::create_dir_all(&dir).unwrap();
    let plane = |f: &dyn Fn(usize, usize) -> f32| (0..w * h).map(|i| f(i % w, i / w)).collect::<Vec<f32>>();
    let half = |v: Vec<f32>| FlatSamples::F16(v.into_iter().map(exr::prelude::f16::from_f32).collect());
    let channels = AnyChannels::sort(
        vec![
            AnyChannel::new("ViewLayer.Combined.R", half(plane(&|x, _| x as f32 / w as f32))),
            AnyChannel::new("ViewLayer.Combined.G", half(plane(&|_, y| y as f32 / h as f32))),
            AnyChannel::new("ViewLayer.Combined.B", half(plane(&|_, _| 0.3))),
            AnyChannel::new("ViewLayer.Combined.A", half(plane(&|_, _| 1.0))),
            AnyChannel::new("ViewLayer.IndexOB.X", FlatSamples::F32(plane(&|x, y| ((x / 240 + y / 270) % 6) as f32))),
        ]
        .into(),
    );
    Image::from_channels((w, h), channels).write().to_file(dir.join("ids.exr")).expect("write the timing EXR");

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
        ("ID Key, object 3, no feather, then Noise", Some(key("object_id", 3.0, 0.0, "off"))),
        ("ID Key, object 3, feather 3, then Noise", Some(key("object_id", 3.0, 3.0, "off"))),
        ("Pass Extract, object ids 0 to 5, then Noise", Some(extract("object_id", 0.0, 5.0, ""))),
    ];
    for (name, e) in shots {
        let effects: Vec<J> = e.into_iter().chain([noise.clone()]).collect();
        let project = still("ids.exr", (w, h), 240, json!(effects));
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
    let out = std::env::var("B229_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-229_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
