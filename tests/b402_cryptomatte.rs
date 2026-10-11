//! B-402: D-522, Cryptomatte, after After Effects' Cryptomatte (3D Channel): objects or
//! materials picked by name from the Cryptomatte a 3D render saved in the layer's EXR file
//! (Psyop's specification 1.2.0), shown as colours, as a matte, or keyed.
//!
//! Every expected pixel is `Fixtures/cryptomatte/expected_cryptomatte.json`, written by
//! `tools/cryptomatte_reference.py` before this code existed. The card is held to the processor
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
use anime_compositor::{exr_io, persist, png_out, OutputDepth};

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
        "schema_version": 0, "project_id": "proj-b402",
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

/// The playtest's sample (`sample/balls.exr`, 320 by 180).
fn sample(effects: J) -> Project {
    still("sample/balls.exr", (320, 180), 5, effects)
}

fn fx(id: &str, type_id: &str, p: J) -> J {
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": p})
}

fn crypto(layer: &str, selection: &str, output: &str) -> J {
    fx("fx-1", "core.cryptomatte", json!({"layer": layer, "selection": selection, "output": output}))
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
fn b402_cryptomatte() {
    let mut t = Table::new(
        "cryptomatte",
        "# B-402: Cryptomatte\n\nD-522: Cryptomatte, after After Effects' Cryptomatte (3D \
         Channel): objects or materials picked by name from the Cryptomatte a 3D render saved in \
         the layer's EXR file (Psyop's specification 1.2.0), shown as colours, as a matte, or \
         keyed. Every expected pixel is `Fixtures/cryptomatte/expected_cryptomatte.json`, written \
         by `tools/cryptomatte_reference.py` before this code existed and printed in document 25 \
         as FX-CRYPTO-001 to 023. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-CRYPTO-001 to 023 (document 25)");
    t.fixtures_numbered("expected_cryptomatte.json", 1..=23);

    t.heading("The name hash");
    let id = exr_io::crypto_id("torus").to_bits();
    t.row("\"torus\" hashes to 4053562365, the specification's own example", &id.to_string(), id == 4_053_562_365);

    t.heading("The file");
    let files: Vec<String> = (1..=23).map(|n| format!("fx_crypto_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    for file in ["fx_crypto_004.json", "fx_crypto_007.json"] {
        let params = t.saved_parameters(file);
        t.row(
            &format!("{file}: the matte read for a frame is never saved, the names kept as typed"),
            &params.to_string(),
            params.get("channels").is_none(),
        );
    }
    let why = effect_of(&t.load("fx_crypto_023.json").document).why_invalid();
    let said23 = "Cryptomatte's output is \"colors\", \"matte\" or \"keyed\", and this is \"preview\".";
    t.row("fx_crypto_023.json is refused in a sentence naming it", &why, why == said23);

    t.heading("Commands");
    let mut document = t.load("fx_crypto_004.json").document;
    let base = effect_of(&document);
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = base.clone();
        f(&mut e);
        e
    };
    let refused = document.apply(set(with(&|e| if let Effect::Cryptomatte { output, .. } = e { *output = "preview".into() }))).err();
    let untouched = effect_of(&document) == base;
    t.row(
        "output \"preview\" is refused with a sentence, and nothing changes",
        &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
        refused.is_some() && untouched,
    );
    t.taken(
        &mut document,
        "fx_crypto_004.json",
        vec![
            ("CryptoMaterial, steel, keyed,", set(with(&|e| if let Effect::Cryptomatte { layer, selection, output, .. } = e {
                *layer = "CryptoMaterial".into();
                *selection = "steel".into();
                *output = "keyed".into();
            }))),
            ("the colours,", set(with(&|e| if let Effect::Cryptomatte { output, .. } = e { *output = "colors".into() }))),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_crypto_002.json", 0), ("fx_crypto_006.json", 0), ("fx_crypto_012.json", 0), ("fx_crypto_019.json", 0)]);

    let root = effect_table::repo("Fixtures/cryptomatte");
    let draw = |p: &Project| {
        let mut log = FrameLog::new(8);
        let b = render_frame(p, &Id::new(MAIN), 0, &root, 64, &mut log).expect("the sample draws");
        (b, said(log))
    };

    t.heading("The card against the processor, within 1 level of 255 (ADR-006, D-100)");
    let mut gpu = Gpu::new().expect("a usable card");
    let mut shots: Vec<(String, Project, bool)> = Vec::new();
    for n in 1..=22 {
        let file = format!("fx_crypto_{n:03}.json");
        shots.push((file.clone(), t.load(&file).document.project().clone(), false));
    }
    let settings: [(&str, J); 7] = [
        ("Colors, the first layer", json!([crypto("", "", "colors")])),
        ("Colors of CryptoObject, the floor picked", json!([crypto("CryptoObject", "floor", "colors")])),
        ("Matte of the yellow ball", json!([crypto("CryptoObject", "yellow ball", "matte")])),
        ("Keyed on the red and blue balls", json!([crypto("CryptoObject", "red ball, blue ball", "keyed")])),
        ("Keyed on the material paint", json!([crypto("CryptoMaterial", "paint", "keyed")])),
        ("Keyed on the sky, after a Gaussian Blur of 6", json!([blur(), crypto("CryptoObject", "sky", "keyed")])),
        ("Matte of gold and tile, the layer named in lower case", json!([crypto("cryptomaterial", "gold, tile", "matte")])),
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

    t.heading("Pictures: the sample render, in `verification/D-522 pictures/`");
    let dir = effect_table::repo("verification/D-522 pictures");
    fs::create_dir_all(&dir).unwrap();
    let shots = [
        ("before", "the sample with no effect: a red, a blue and a yellow ball over a two-tone floor, under a blue sky", json!([])),
        ("colors_objects", "Colors of CryptoObject: each ball, the floor and the sky one flat colour, their edges mixed", json!([crypto("CryptoObject", "", "colors")])),
        ("colors_materials", "Colors of CryptoMaterial: the red and blue balls one colour (they share the material paint)", json!([crypto("CryptoMaterial", "", "colors")])),
        ("colors_floor_picked", "Colors of CryptoObject with floor picked: the floor white, the rest in colours", json!([crypto("CryptoObject", "floor", "colors")])),
        ("matte_yellow_ball", "Matte of yellow ball: that ball white with a soft edge, the rest black", json!([crypto("CryptoObject", "yellow ball", "matte")])),
        ("keyed_red_blue", "Keyed on red ball, blue ball: those two balls alone, everything else clear", json!([crypto("CryptoObject", "red ball, blue ball", "keyed")])),
        ("keyed_material_gold", "Keyed on the material gold: the yellow ball alone", json!([crypto("CryptoMaterial", "gold", "keyed")])),
    ];
    for (file, what, effects) in shots {
        let (b, s) = draw(&sample(effects));
        png_out::write_rgba(&dir.join(format!("{file}.png")), 320, 180, OutputDepth::Eight, &[], &b.to_srgb8_straight()).unwrap();
        t.row(&format!("{file}.png, {what}"), &format!("warnings {s:?}"), s.is_empty());
    }

    t.finish("D-522_cryptomatte_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-229's way: a 1920 by 1080 EXR still with a picture and a CryptoObject
/// layer of two levels (written here, into the system's temporary folder), one layer, the effect
/// then a Noise that changes every frame so nothing is kept, every eighth frame of 240 asked for
/// as the viewer asks, whole, with Draw on: GPU. The first loop starts with empty caches and its
/// 30 frames' median is "first"; then seven loops are timed, the median of their 210 frames is
/// "again".
#[test]
#[ignore = "B-402: a measurement, run deliberately with --release --ignored"]
fn b402_cryptomatte_timing() {
    use exr::prelude::{AnyChannel, AnyChannels, FlatSamples, Image, WritableImage};
    use std::fmt::Write as _;

    let (w, h) = (1920usize, 1080usize);
    let dir = std::env::temp_dir().join("b402_timing");
    fs::create_dir_all(&dir).unwrap();
    let plane = |f: &dyn Fn(usize, usize) -> f32| (0..w * h).map(|i| f(i % w, i / w)).collect::<Vec<f32>>();
    let half = |v: Vec<f32>| FlatSamples::F16(v.into_iter().map(exr::prelude::f16::from_f32).collect());
    let ids: Vec<f32> = (0..6).map(|i| exr_io::crypto_id(&format!("object {i}"))).collect();
    // Each pixel: its block's object at 0.7, the next one at 0.3.
    let block = |x: usize, y: usize| (x / 240 + y / 270) % 6;
    let channels = AnyChannels::sort(
        vec![
            AnyChannel::new("R", half(plane(&|x, _| x as f32 / w as f32))),
            AnyChannel::new("G", half(plane(&|_, y| y as f32 / h as f32))),
            AnyChannel::new("B", half(plane(&|_, _| 0.3))),
            AnyChannel::new("A", half(plane(&|_, _| 1.0))),
            AnyChannel::new("CryptoObject00.r", FlatSamples::F32(plane(&|x, y| ids[block(x, y)]))),
            AnyChannel::new("CryptoObject00.g", FlatSamples::F32(plane(&|_, _| 0.7))),
            AnyChannel::new("CryptoObject00.b", FlatSamples::F32(plane(&|x, y| ids[(block(x, y) + 1) % 6]))),
            AnyChannel::new("CryptoObject00.a", FlatSamples::F32(plane(&|_, _| 0.3))),
            AnyChannel::new("CryptoObject01.r", FlatSamples::F32(plane(&|_, _| 0.0))),
            AnyChannel::new("CryptoObject01.g", FlatSamples::F32(plane(&|_, _| 0.0))),
            AnyChannel::new("CryptoObject01.b", FlatSamples::F32(plane(&|_, _| 0.0))),
            AnyChannel::new("CryptoObject01.a", FlatSamples::F32(plane(&|_, _| 0.0))),
        ]
        .into(),
    );
    Image::from_channels((w, h), channels).write().to_file(dir.join("crypto.exr")).expect("write the timing EXR");

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
        ("Cryptomatte, Colors, then Noise", Some(crypto("CryptoObject", "", "colors"))),
        ("Cryptomatte, Matte of two objects, then Noise", Some(crypto("CryptoObject", "object 2, object 3", "matte"))),
        ("Cryptomatte, Keyed on two objects, then Noise", Some(crypto("CryptoObject", "object 2, object 3", "keyed"))),
    ];
    for (name, e) in shots {
        let effects: Vec<J> = e.into_iter().chain([noise.clone()]).collect();
        let project = still("crypto.exr", (w, h), 240, json!(effects));
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
    let out = std::env::var("B402_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-402_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
