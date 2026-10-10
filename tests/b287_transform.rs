//! B-287: D-408, After Effects' Transform effect under `core.transform`: the drawing moved,
//! scaled, skewed, turned and faded inside its own layer, with its own motion blur.
//!
//! Writes `verification/D-408_transform_table.md` and draws pictures into
//! `verification/D-408 pictures/`.
//!
//! Every expected pixel is `Fixtures/transform/expected_transform.json`, written by
//! `tools/transform_reference.py` before this code existed and printed in document 25 as
//! FX-XFORM-001 to 032. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// The Transforms the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::Transform { .. })))
        .count()
        // An adjustment layer's run is made from its stack when the card draws it.
        + plan.layers
            .iter()
            .filter_map(|l| l.adjust.as_ref())
            .filter_map(|stack| compose::adjust_run(stack, (plan.width, plan.height)))
            .flatten()
            .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::Transform { .. })))
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

/// A Transform with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"anchor_point": [50, 50], "position": [50, 50], "uniform_scale": "on", "scale_height": 100,
        "scale_width": 100, "skew": 0, "skew_axis": 0, "rotation": 0, "opacity": 100,
        "use_composition_shutter_angle": "on", "shutter_angle": 0, "sampling": "bilinear"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.transform", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` leave nothing to the card: untouched, refused, or through the shutter
/// (D-188 draws a blurred layer on the processor, as it does every effect on one).
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=32 {
        let file = format!("fx_xform_{n:03}.json");
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
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J, usize)]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p, want) in settings {
        let project = reference(|id| json!([fx(&format!("b287-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Transform {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Transform {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the street, frame `frame`, drawn on the processor, straight 8-bit, with what it
/// warned of; with `blur`, the layer's switch on and the composition's blur at 180 degrees.
fn picture(dir: &Path, effects: J, blur: bool, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    if blur {
        comp["motion_blur"] = json!({"enabled": true, "shutter_angle": 180, "shutter_phase": -90, "samples": 16});
    }
    let layer = &mut comp["layers"][0];
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    if blur {
        layer["motion_blur"] = J::from(true);
    }
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

#[test]
fn b287_transform() {
    let mut t = Table::new(
        "transform",
        "# D-408: Transform\n\nB-287: `core.transform`, After Effects' Transform effect: the \
         drawing moved, scaled, skewed, turned and faded inside its own layer (the layer never \
         grows), bilinear or bicubic, with its own motion blur when the layer's switch is on and \
         the composition's blur enabled. Every expected pixel is \
         `Fixtures/transform/expected_transform.json`, written by `tools/transform_reference.py` \
         before this code existed and printed in document 25 as FX-XFORM-001 to 032. Tolerance \
         2e-5.\n",
    );

    t.heading("FX-XFORM-001 to 032 (document 25)");
    t.fixtures_numbered("expected_transform.json", 1..=32);

    t.heading("The file");
    let files: Vec<String> = (1..=25).map(|n| format!("fx_xform_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_xform_019.json");
    t.row(
        "fx_xform_019.json is saved with its twelve settings, its own shutter as written",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 12 && saved["use_composition_shutter_angle"] == "off" && saved["shutter_angle"] == 360.0,
    );
    for (file, want) in [
        ("fx_xform_026.json", "skew"),
        ("fx_xform_027.json", "opacity"),
        ("fx_xform_029.json", "shutter_angle"),
        ("fx_xform_030.json", "uniform scale"),
        ("fx_xform_031.json", "sampling"),
        ("fx_xform_032.json", "shutter angle"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        let says = why.contains(want) || why.contains(&want.replace('_', " "));
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, says);
    }
    t.shape_refused("fx_xform_001.json", "a Transform whose rotation is a word", r##"{"anchor_point": [50, 50], "position": [50, 50], "uniform_scale": "on", "scale_height": 100, "scale_width": 100, "skew": 0, "skew_axis": 0, "rotation": "some", "opacity": 100, "use_composition_shutter_angle": "on", "shutter_angle": 0, "sampling": "bilinear"}"##);
    t.shape_refused("fx_xform_001.json", "a Transform without its position", r##"{"anchor_point": [50, 50], "uniform_scale": "on", "scale_height": 100, "scale_width": 100, "skew": 0, "skew_axis": 0, "rotation": 0, "opacity": 100, "use_composition_shutter_angle": "on", "shutter_angle": 0, "sampling": "bilinear"}"##);
    t.shape_refused("fx_xform_001.json", "a Transform whose anchor point is one number", r##"{"anchor_point": 50, "position": [50, 50], "uniform_scale": "on", "scale_height": 100, "scale_width": 100, "skew": 0, "skew_axis": 0, "rotation": 0, "opacity": 100, "use_composition_shutter_angle": "on", "shutter_angle": 0, "sampling": "bilinear"}"##);

    t.heading("Commands");
    let mut document = t.load("fx_xform_001.json").document;
    let skew = changed(&t, "fx_xform_001.json", |e| if let Effect::Transform { skew, .. } = e { *skew = 86.0 });
    let fade = changed(&t, "fx_xform_001.json", |e| if let Effect::Transform { opacity, .. } = e { *opacity = 101.0 });
    let near = changed(&t, "fx_xform_001.json", |e| if let Effect::Transform { sampling, .. } = e { *sampling = "nearest".into() });
    t.refused(
        &mut document,
        vec![
            ("skew 86", set(skew)),
            ("opacity 101", set(fade)),
            ("sampling nearest", set(near)),
            ("rotation keyed to 4000", keys("rotation", &[(0, &[0.0]), (4, &[4000.0])])),
        ],
    );
    let turn = changed(&t, "fx_xform_001.json", |e| {
        if let Effect::Transform { rotation, sampling, .. } = e {
            *rotation = 30.0;
            *sampling = "bicubic".into();
        }
    });
    t.taken(
        &mut document,
        "fx_xform_001.json",
        vec![
            ("rotation 30, bicubic", set(turn)),
            ("position keyed from (50, 50) to (100, 50)", keys("position", &[(0, &[50.0, 50.0]), (4, &[100.0, 50.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_xform_003.json", 0),
        ("fx_xform_006.json", 0),
        ("fx_xform_008.json", 0),
        ("fx_xform_009.json", 0),
        ("fx_xform_011.json", 0),
        ("fx_xform_016.json", 2),
        ("fx_xform_022.json", 2),
        ("fx_xform_024.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // As added (001, 025) leaves nothing to draw; through the shutter (016, 019 to 022) the
    // layer is drawn on the processor, as D-188 draws every blurred layer; refused (026 to 032).
    let none: Vec<u32> = [1, 16, 19, 20, 21, 22, 25].into_iter().chain(26..=32).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("turned 15 degrees", json!({"rotation": 15}), 3),
            ("skewed 20 along 30, scaled 80 by 120, bicubic", json!({"skew": 20, "skew_axis": 30, "uniform_scale": "off", "scale_height": 80, "scale_width": 120, "sampling": "bicubic"}), 3),
            ("moved to (60, 40) round (45, 55) at opacity 70", json!({"anchor_point": [45, 55], "position": [60, 40], "opacity": 70}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-408 pictures/`");
    let dir = repo("verification/D-408 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let street = |p: J| picture(&dir, json!([fx("fx-0-0", &p)]), false, 0);
    let clear = |p: &[u8]| p.chunks_exact(4).filter(|q| q[3] == 0).count();
    let (before, said) = picture(&dir, json!([]), false, 0);
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (same, said) = street(json!({}));
    t.row(
        "as added changes nothing: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );

    let (turned, said) = street(json!({"rotation": 20, "uniform_scale": "on", "scale_height": 80}));
    write("2_turned_small.png", &turned);
    t.row(
        "2_turned_small.png, rotation 20 at scale 80: the street smaller and tilted clockwise inside the frame, clear corners round it, the frame no bigger; draws cleanly",
        &format!("{said:?}, {} clear pixels against {}", clear(&turned), clear(&before)),
        said.is_empty() && clear(&turned) > clear(&before),
    );

    let (skewed, said) = street(json!({"skew": 25, "sampling": "bicubic"}));
    write("3_skew_25_bicubic.png", &skewed);
    t.row(
        "3_skew_25_bicubic.png, skew 25, bicubic: the houses lean, the top of the street slid right and the bottom left; draws cleanly",
        &format!("{said:?}, {} clear pixels", clear(&skewed)),
        said.is_empty() && clear(&skewed) > 0,
    );

    let (faded, said) = street(json!({"opacity": 50}));
    write("4_opacity_50.png", &faded);
    let alpha = |p: &[u8]| p.chunks_exact(4).map(|q| q[3] as u32).max().unwrap();
    t.row(
        "4_opacity_50.png, opacity 50: the street see-through, no pixel more than half covered; draws cleanly",
        &format!("{said:?}, the most covered pixel {} of 255", alpha(&faded)),
        said.is_empty() && alpha(&faded) == 128,
    );

    // Position keyed across the street over frames 0 to 4, drawn at frame 2.
    let slide = json!([{"instance_id": "fx-0-0", "type_id": "core.transform", "enabled": true, "parameters": {
        "anchor_point": [50, 50],
        "position": {"base": [50, 50], "keyframes": [{"frame": 0, "value": [50, 50], "interp": "linear"}, {"frame": 4, "value": [70, 50], "interp": "linear"}]},
        "uniform_scale": "on", "scale_height": 100, "scale_width": 100, "skew": 0, "skew_axis": 0, "rotation": 0, "opacity": 100,
        "use_composition_shutter_angle": "on", "shutter_angle": 0, "sampling": "bilinear"}}]);
    let (sharp, said_sharp) = picture(&dir, slide.clone(), false, 2);
    write("5_sliding_sharp.png", &sharp);
    let (blurred, said) = picture(&dir, slide, true, 2);
    write("6_sliding_blurred.png", &blurred);
    // Sideways contrast: the summed difference between each pixel and the one to its right.
    let edges = |p: &[u8]| p.chunks_exact(4).collect::<Vec<_>>().windows(2).map(|w| w[0][0].abs_diff(w[1][0]) as u64).sum::<u64>();
    t.row(
        "5_sliding_sharp.png and 6_sliding_blurred.png, the street sliding right through frame 2, without and with motion blur: the blurred one smeared sideways, its edges softer; both draw cleanly",
        &format!("{said_sharp:?} {said:?}, sideways contrast {} against {}", edges(&blurred), edges(&sharp)),
        said_sharp.is_empty() && said.is_empty() && edges(&blurred) < edges(&sharp),
    );

    t.finish("D-408_transform_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B287_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-287: a measurement, run deliberately with --release --ignored"]
fn b287_transform_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B287_CPU").is_ok();
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
        ("Noise, then Transform turned 15 degrees (bilinear)", Some(json!({"rotation": 15}))),
        ("Noise, then Transform skewed 20, scaled 80 by 120, bicubic", Some(json!({"skew": 20, "uniform_scale": "off", "scale_height": 80, "scale_width": 120, "sampling": "bicubic"}))),
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
    let out = std::env::var("B287_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-287_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
