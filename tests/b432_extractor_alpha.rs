//! B-432: D-552, Pass Extract's alpha channel and UnMult, after fnord's EXtractoR (3D Channel):
//! a fourth channel name picks the alpha, and UnMult divides the three values by it before the
//! black and white points.
//!
//! Every expected pixel is `Fixtures/extractor_alpha/expected_extractor_alpha.json`, written by
//! `tools/extractor_alpha_reference.py` before this code existed. The card is held to the
//! processor within 1 level of 255 (ADR-006, D-100) on every fixture and on the sample render.

mod effect_table;

use std::fs;
use std::path::Path;

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
    layer
        .effects
        .iter()
        .find(|i| i.instance_id.as_str() == "fx-1")
        .unwrap()
        .effect
        .clone()
}

/// A one-layer composition `w` by `h` of the still `path`, carrying `effects`.
fn still(path: &str, (w, h): (usize, usize), frames: u32, effects: J) -> Project {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let project = json!({
        "schema_version": 0, "project_id": "proj-b432",
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
    persist::load_str(&project.to_string())
        .expect("the project reads")
        .document
        .project()
        .clone()
}

/// The playtest's sample, Blender's layout (`Fixtures/depth_channel/sample/spheres_blender.exr`,
/// 320 by 180).
fn sample(effects: J) -> Project {
    still("sample/spheres_blender.exr", (320, 180), 5, effects)
}

fn fx(id: &str, type_id: &str, p: J) -> J {
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": p})
}

fn extract(channel: &str, black: f64, white: f64, unmult: &str) -> J {
    fx(
        "fx-1",
        "core.pass_extract",
        json!({"pass": "named", "black_point": black, "white_point": white,
                                          "invert": "off", "clamp": "on", "channel": channel, "unmult": unmult}),
    )
}

const COMBINED: &str =
    "ViewLayer.Combined.R, ViewLayer.Combined.G, ViewLayer.Combined.B, ViewLayer.Combined.A";
const NORMALS: &str =
    "ViewLayer.Normal.X, ViewLayer.Normal.Y, ViewLayer.Normal.Z, ViewLayer.Combined.A";
const MIST: &str = "ViewLayer.Mist.Z, ViewLayer.Mist.Z, ViewLayer.Mist.Z, ViewLayer.Combined.A";

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
fn b432_extractor_alpha() {
    let mut t = Table::new(
        "extractor_alpha",
        "# D-552: Pass Extract's alpha channel and UnMult\n\nB-432, after fnord's EXtractoR (3D \
         Channel, ProEXR manual 2.6.6): Pass Extract's channel names may be four, the fourth the \
         alpha, and UnMult divides the three values by that alpha before the black and white \
         points; the result is laid over the alpha. One or three names still give alpha 1, and \
         UnMult is left out of the file while off, so every project saved before draws and saves \
         as it did. Every expected pixel is \
         `Fixtures/extractor_alpha/expected_extractor_alpha.json`, written by \
         `tools/extractor_alpha_reference.py` before this code existed and printed in document 25 \
         as FX-EXA-001 to 021. Tolerance 2e-5.\n",
    );

    t.heading("FX-EXA-001 to 021 (document 25)");
    t.fixtures_numbered("expected_extractor_alpha.json", 1..=21);

    t.heading("The file");
    let files: Vec<String> = (1..=21).map(|n| format!("fx_exa_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    for (file, what, want) in [
        (
            "fx_exa_001.json",
            "a project with no UnMult writes none",
            None,
        ),
        (
            "fx_exa_006.json",
            "UnMult written off stays off",
            Some("off"),
        ),
        ("fx_exa_002.json", "UnMult on is saved on", Some("on")),
    ] {
        let params = t.saved_parameters(file);
        t.row(
            &format!("{file}: {what}"),
            &params.to_string(),
            params.get("unmult").and_then(J::as_str) == want,
        );
    }
    for (file, said) in [
        (
            "fx_exa_020.json",
            "Pass Extract's unmult is \"off\" or \"on\", and this is \"yes\".",
        ),
        (
            "fx_exa_021.json",
            "Pass Extract's unmult is \"off\" or \"on\", and this is \"On\".",
        ),
    ] {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(
            &format!("{file} is refused in a sentence naming it"),
            &why,
            why == said,
        );
    }

    t.heading("Commands");
    let mut document = t.load("fx_exa_001.json").document;
    let base = effect_of(&document);
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = base.clone();
        f(&mut e);
        e
    };
    let refused = document
        .apply(set(with(&|e| {
            if let Effect::PassExtract { unmult, .. } = e {
                *unmult = "yes".into()
            }
        })))
        .err();
    let untouched = effect_of(&document) == base;
    t.row(
        "UnMult \"yes\" is refused with a sentence, and nothing changes",
        &refused
            .as_ref()
            .map_or("taken".to_string(), |d| d.message.clone()),
        refused.is_some() && untouched,
    );
    t.taken(
        &mut document,
        "fx_exa_001.json",
        vec![
            (
                "UnMult on,",
                set(with(&|e| {
                    if let Effect::PassExtract { unmult, .. } = e {
                        *unmult = "on".into()
                    }
                })),
            ),
            (
                "the alpha from Mask.Y,",
                set(with(&|e| {
                    if let Effect::PassExtract { channel, .. } = e {
                        *channel = "Reflect.R, Reflect.G, Reflect.B, Mask.Y".into();
                    }
                })),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_exa_002.json", 0),
        ("fx_exa_008.json", 0),
        ("fx_exa_015.json", 0),
    ]);

    let fixtures = effect_table::repo("Fixtures/extractor_alpha");
    let samples = effect_table::repo("Fixtures/depth_channel");

    t.heading("The card against the processor, within 1 level of 255 (ADR-006, D-100)");
    let mut gpu = Gpu::new().expect("a usable card");
    let mut shots: Vec<(String, Project, &Path, bool)> = Vec::new();
    for n in 1..=19 {
        let file = format!("fx_exa_{n:03}.json");
        shots.push((
            file.clone(),
            t.load(&file).document.project().clone(),
            &fixtures,
            false,
        ));
    }
    let settings: [(&str, J); 5] = [
        (
            "Combined with its alpha, UnMult off",
            json!([extract(COMBINED, 0.0, 1.0, "off")]),
        ),
        (
            "Combined with its alpha, UnMult on",
            json!([extract(COMBINED, 0.0, 1.0, "on")]),
        ),
        (
            "the normals over Combined's alpha, black -1, white 1",
            json!([extract(NORMALS, -1.0, 1.0, "off")]),
        ),
        (
            "the mist over Combined's alpha, UnMult on",
            json!([extract(MIST, 0.0, 1.0, "on")]),
        ),
        (
            "Combined, black 0.2, white 0.8, UnMult on, three names",
            json!([extract(
                "ViewLayer.Combined.R, ViewLayer.Combined.G, ViewLayer.Combined.B",
                0.2,
                0.8,
                "on"
            )]),
        ),
    ];
    for (name, effects) in settings {
        shots.push((
            format!("the sample, {name}"),
            sample(effects),
            &samples,
            true,
        ));
    }
    let mut worst = 0u8;
    for (name, project, root, is_sample) in &shots {
        let comp = &project.compositions[0];
        let frames: Vec<i32> =
            (comp.start_frame..comp.start_frame + comp.duration_frames as i32).collect();
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for &frame in frames.iter().filter(|&&f| f == 0 || f == 4) {
                let mut cache = CelCache::viewer();
                let mut log = FrameLog::new(3);
                let c = preview::preview_frame_cached(
                    project,
                    &comp.id,
                    frame,
                    root,
                    quality,
                    DEFAULT_TILE_SIZE,
                    &mut log,
                    &mut cache,
                )
                .unwrap_or_else(|d| panic!("{name} frame {frame} on the CPU: {}", d.message));
                let said_cpu = said(log);
                let c = c.to_srgb8_straight();
                let mut log = FrameLog::new(3);
                let (g, ..) = preview::preview_frame_srgb8(
                    project,
                    &comp.id,
                    frame,
                    root,
                    quality,
                    DEFAULT_TILE_SIZE,
                    &mut log,
                    &mut cache,
                    &mut gpu,
                )
                .unwrap_or_else(|d| panic!("{name} frame {frame} on the GPU: {}", d.message));
                let said_gpu = said(log);
                let mut log = FrameLog::new(3);
                let plan = compose::plan_frame_for_card(
                    project,
                    &comp.id,
                    frame,
                    root,
                    quality,
                    &mut log,
                    &mut CelCache::viewer(),
                )
                .expect("plan the frame");
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
    t.row(
        "the largest difference over every frame above",
        &format!("{worst} level(s)"),
        worst <= 1,
    );

    t.heading("Pictures: the sample render in Blender's layout, in `verification/D-552 pictures/`");
    let draw = |p: &Project| {
        let mut log = FrameLog::new(8);
        let b =
            render_frame(p, &Id::new(MAIN), 0, &samples, 64, &mut log).expect("the sample draws");
        (b.to_srgb8_straight(), said(log))
    };
    let dir = effect_table::repo("verification/D-552 pictures");
    fs::create_dir_all(&dir).unwrap();
    let before = draw(&sample(json!([]))).0;
    let shots = [
        ("before", "the Blender-style file with no effect: three balls on a floor under a clear sky", json!([])),
        ("combined_unmult_off", "Pass Extract, Combined's four channels, UnMult off: the same picture, its sky clear", json!([extract(COMBINED, 0.0, 1.0, "off")])),
        ("combined_unmult_on", "the same with UnMult on: the same picture again (this render's alpha is 0 or 1, so dividing changes nothing)", json!([extract(COMBINED, 0.0, 1.0, "on")])),
        ("normals_over_alpha", "Pass Extract, the normals with Combined's alpha, Black -1, White 1: the balls and floor in normal colours, the sky clear rather than grey", json!([extract(NORMALS, -1.0, 1.0, "off")])),
        ("mist_over_alpha", "Pass Extract, the mist with Combined's alpha: near dark, far light, the sky clear rather than white", json!([extract(MIST, 0.0, 1.0, "off")])),
    ];
    for (file, what, effects) in shots {
        let (b, s) = draw(&sample(effects));
        png_out::write_rgba(
            &dir.join(format!("{file}.png")),
            320,
            180,
            OutputDepth::Eight,
            &[],
            &b,
        )
        .unwrap();
        t.row(
            &format!("{file}.png, {what}"),
            &format!("warnings {s:?}"),
            s.is_empty(),
        );
        if file.starts_with("combined") {
            let d = distance(&b, &before);
            t.row(
                &format!("{file}.png matches before.png within 1 level"),
                &format!("largest {} level(s), {} pixel(s) differ", d.0, d.1),
                d.0 <= 1,
            );
        }
        if file.ends_with("over_alpha") {
            let clear = b
                .chunks_exact(4)
                .zip(before.chunks_exact(4))
                .filter(|(p, q)| q[3] == 0 && p[3] == 0)
                .count();
            let sky = before.chunks_exact(4).filter(|q| q[3] == 0).count();
            t.row(
                &format!("{file}.png is clear wherever before.png is"),
                &format!("{clear} of {sky} sky pixels clear"),
                sky > 0 && clear == sky,
            );
        }
    }

    t.finish("D-552_extractor_alpha_table.md");
}

/// The frame times, B-229's way: a 1920 by 1080 EXR still laid out as Blender writes it, with a
/// soft Combined.A (written here, into the system's temporary folder), one layer, the effect then
/// a Noise that changes every frame so nothing is kept, every eighth frame of 240 asked for as
/// the viewer asks, whole. The first loop starts with empty caches and its 30 frames' median is
/// "first"; then the remaining loops are timed, the median of their frames is "again". With
/// `B432_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-432: a measurement, run deliberately with --release --ignored"]
fn b432_extractor_alpha_timing() {
    use exr::prelude::{AnyChannel, AnyChannels, FlatSamples, Image, WritableImage};
    use std::fmt::Write as _;

    let cpu = std::env::var("B432_CPU").is_ok();
    let passes = if cpu { 3 } else { 8 };
    let (w, h) = (1920usize, 1080usize);
    let dir = std::env::temp_dir().join("b432_timing");
    fs::create_dir_all(&dir).unwrap();
    let plane =
        |f: &dyn Fn(usize, usize) -> f32| (0..w * h).map(|i| f(i % w, i / w)).collect::<Vec<f32>>();
    let half =
        |v: Vec<f32>| FlatSamples::F16(v.into_iter().map(exr::prelude::f16::from_f32).collect());
    let a = |x: usize, _: usize| (x as f32 / w as f32 * 1.5 - 0.25).clamp(0.0, 1.0);
    let channels = AnyChannels::sort(
        vec![
            AnyChannel::new(
                "ViewLayer.Combined.R",
                half(plane(&|x, y| x as f32 / w as f32 * a(x, y))),
            ),
            AnyChannel::new(
                "ViewLayer.Combined.G",
                half(plane(&|x, y| y as f32 / h as f32 * a(x, y))),
            ),
            AnyChannel::new("ViewLayer.Combined.B", half(plane(&|x, y| 0.3 * a(x, y)))),
            AnyChannel::new("ViewLayer.Combined.A", half(plane(&a))),
        ]
        .into(),
    );
    Image::from_channels((w, h), channels)
        .write()
        .to_file(dir.join("soft.exr"))
        .expect("write the timing EXR");

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
    let noise = fx(
        "fx-2",
        "core.noise",
        json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}),
    );
    let shots: [(&str, Option<J>); 4] = [
        ("Noise alone", None),
        (
            "Pass Extract, four names, UnMult off, then Noise",
            Some(extract(COMBINED, 0.0, 1.0, "off")),
        ),
        (
            "Pass Extract, four names, UnMult on, then Noise",
            Some(extract(COMBINED, 0.0, 1.0, "on")),
        ),
        (
            "Pass Extract, three names, then Noise",
            Some(extract(
                "ViewLayer.Combined.R, ViewLayer.Combined.G, ViewLayer.Combined.B",
                0.0,
                1.0,
                "off",
            )),
        ),
    ];
    for (name, e) in shots {
        let effects: Vec<J> = e.into_iter().chain([noise.clone()]).collect();
        let project = still("soft.exr", (w, h), 240, json!(effects));
        let mut cache = CelCache::viewer();
        gpu.forget();
        let (mut first, mut times) = (Vec::new(), Vec::new());
        for pass in 0..passes {
            for frame in (0..240).step_by(8) {
                let mut log = FrameLog::new(3);
                let t = std::time::Instant::now();
                if cpu {
                    drop(
                        preview::preview_frame_cached(
                            &project,
                            &Id::new(MAIN),
                            frame,
                            &dir,
                            PreviewQuality::Full,
                            DEFAULT_TILE_SIZE,
                            &mut log,
                            &mut cache,
                        )
                        .expect("CPU frame"),
                    );
                } else {
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
                }
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
    let out = std::env::var("B432_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| effect_table::repo("verification/B-432_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
