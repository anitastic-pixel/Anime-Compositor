//! B-526: D-647, Turbulent Noise's Overflow, Transform and Sub Settings, after After Effects'
//! Turbulent Noise: Rotation, Uniform Scaling, Perspective Offset, Sub Influence, Sub Scaling,
//! Sub Rotation, Sub Offset and Center Subscale, and five more blending modes.
//!
//! Writes `verification/D-647_turbulent_more_table.md` and draws pictures into
//! `verification/D-647 pictures/`.
//!
//! Every expected pixel is `Fixtures/turbulent_noise/expected_turbulent_more.json`, written by
//! `tools/turbulent_more_reference.py` before this code existed and printed in document 25 as
//! FX-TURBNOISE-033 to 060; FX-TURBNOISE-001 to 032 and FX-TURBTYPE-001 to 025 rerun unchanged.
//! Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{repo, set, town, Table, MAIN, TOWN};
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

/// The Turbulent Noises the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::TurbulentNoise { .. })))
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

/// A Turbulent Noise with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"fractal_type": "basic", "noise_type": "smooth", "invert": "off", "contrast": 100, "brightness": 0,
        "size": 100, "scale_width": 100, "scale_height": 100, "offset": [0, 0], "complexity": 6, "evolution": 0, "seed": 0,
        "opacity": 100, "blend": "normal"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.turbulent_noise", "enabled": true, "parameters": all})
}

/// A setting keyed linearly through `points`, each a frame and a value.
fn keyed(points: &[(i32, J)]) -> J {
    let frames: Vec<J> = points.iter().map(|(f, v)| json!({"frame": f, "value": v, "interp": "linear"})).collect();
    json!({"base": points[0].1, "keyframes": frames})
}

/// The `art` layer's effect in `file`, changed by `f`.
fn changed(t: &Table, file: &str, f: impl FnOnce(&mut Effect)) -> Effect {
    let d = t.load(file).document;
    let mut e = d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    f(&mut e);
    e
}

/// `effects` on the street (in `dir`) at `frame`, drawn by the processor, straight 8-bit, with
/// what it warned of.
fn picture(dir: &Path, effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    comp["duration_frames"] = J::from(48);
    comp["work_area"]["end_frame_exclusive"] = J::from(48);
    let layer = &mut comp["layers"][0];
    layer["out_frame"] = J::from(48);
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

#[test]
fn b526_turbulent_more() {
    let mut t = Table::new(
        "turbulent_noise",
        "# D-647: Turbulent Noise, Overflow, Transform and Sub Settings\n\nB-526: `core.turbulent_noise` takes \
         After Effects' Overflow (Allow HDR Results, Clip, Soft Clamp, Wrap Back), Rotation, Uniform Scaling, \
         Perspective Offset, Sub Settings (Sub Influence, Sub Scaling, Sub Rotation, Sub Offset, Center Subscale) \
         and five more blending modes (Soft Light, Hard Light, Darken, Lighten, Difference). With c the drawing's \
         middle, octave o of the clouds is read at c + R(-t_o) (p - w_o Offset - o Sub Offset - c), t_o = Rotation \
         + o Sub Rotation, w_o = 0.5^o with Perspective Offset on, else 1; each octave Sub Influence per cent of \
         the last's strength (cloudy 0.35 scaled by Sub Influence / 50) and 100 / Sub Scaling times as fine, round \
         c with Center Subscale on. Soft Clamp is 0.5 + 0.5 tanh(2 (v - 0.5)); Wrap Back folds at black and white. \
         Every setting as added (Allow HDR Results, 0, off, off, 50, 50, 0, (0, 0), off) draws every file before \
         D-647 as before; After Effects' own Sub Influence 70 and Sub Scaling 56 are not used for that reason. \
         The rule is this program's own; Adobe publishes none. Overlay and the hue, saturation, color, luminosity, \
         none and stencil modes are not built here (D-730 builds the last six; D-731, by the owner's decision of \
         2026-10-13, builds Overlay and Hue, FX-TURBNOISE-032 and 060 regenerated). \
         Every expected pixel is `Fixtures/turbulent_noise/expected_turbulent_more.json`, written by \
         `tools/turbulent_more_reference.py` before this code existed and printed in document 25 as \
         FX-TURBNOISE-033 to 060. Tolerance 2e-5.\n",
    );

    t.heading("FX-TURBNOISE-033 to 060 (document 25)");
    t.fixtures_numbered("expected_turbulent_more.json", 33..=60);

    t.heading("Turbulent Noise's earlier fixtures, unchanged");
    t.fixtures_numbered("expected_turbulent_noise.json", 1..=32);
    let root = t.root.clone();
    t.root = repo("Fixtures/turbulent_noise_types");
    t.fixtures_numbered("expected_turbulent_noise_types.json", 1..=25);
    t.root = root;

    t.heading("The file");
    let all: Vec<String> = (33..=60).map(|n| format!("fx_turbnoise_{n:03}.json")).collect();
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let old = t.saved_parameters("fx_turbnoise_001.json");
    let new_keys = ["overflow", "rotation", "uniform_scaling", "perspective_offset", "sub_influence", "sub_scaling", "sub_rotation", "sub_offset", "center_subscale"];
    t.row(
        "fx_turbnoise_001.json, from before D-647, opens as added and saves none of the nine new settings",
        &old.to_string(),
        new_keys.iter().all(|k| old.get(k).is_none()),
    );
    let saved = t.saved_parameters("fx_turbnoise_055.json");
    t.row(
        "fx_turbnoise_055.json, every new setting written as added, keeps all nine when saved",
        &saved.to_string(),
        new_keys.iter().all(|k| saved.get(k).is_some()),
    );
    let saved = t.saved_parameters("fx_turbnoise_047.json");
    t.row(
        "fx_turbnoise_047.json is saved with Rotation 30, Perspective Offset on, Sub Influence 70, Sub Scaling 60, Sub Rotation 20, Sub Offset (1, -2), Center Subscale on",
        &saved.to_string(),
        saved["rotation"] == json!(30)
            && saved["perspective_offset"] == "on"
            && saved["sub_influence"] == json!(70)
            && saved["sub_scaling"] == json!(60)
            && saved["sub_rotation"] == json!(20)
            && saved["sub_offset"] == json!([1, -2])
            && saved["center_subscale"] == "on",
    );
    let saved = t.saved_parameters("fx_turbnoise_037.json");
    t.row("fx_turbnoise_037.json is saved with Rotation's keys kept", &saved["rotation"].to_string(), saved["rotation"]["keyframes"][1]["value"] == json!(90));
    for (file, words) in [
        ("fx_turbnoise_056.json", ["overflow", "\"hdr\""]),
        ("fx_turbnoise_057.json", ["sub_scaling", "5"]),
        ("fx_turbnoise_058.json", ["sub_influence", "101"]),
        ("fx_turbnoise_059.json", ["uniform scaling", "\"yes\""]),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(
            &format!("{file} is refused in a sentence naming Turbulent Noise, {} and {}", words[0], words[1]),
            &why,
            why.contains("Turbulent Noise") && words.iter().all(|w| why.replace('_', " ").contains(&w.replace('_', " "))),
        );
    }

    t.heading("Commands");
    let mut document = t.load("fx_turbnoise_002.json").document;
    let with = |f: fn(&mut Effect)| changed(&t, "fx_turbnoise_002.json", f);
    let refused = vec![
        ("overflow \"hdr\"", set(with(|e| if let Effect::TurbulentNoise { more, .. } = e { more.overflow = "hdr".into() }))),
        ("sub scaling 5", set(with(|e| if let Effect::TurbulentNoise { more, .. } = e { more.sub_scaling = 5.0 }))),
    ];
    let taken = vec![(
        "rotation -45, sub rotation 30, center subscale on, wrap back, difference",
        set(with(|e| {
            if let Effect::TurbulentNoise { more, blend, .. } = e {
                more.rotation = -45.0;
                more.sub_rotation = 30.0;
                more.center_subscale = "on".into();
                more.overflow = "wrap_back".into();
                *blend = "difference".into();
            }
        })),
    )];
    t.refused(&mut document, refused);
    t.taken(&mut document, "fx_turbnoise_002.json", taken);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_turbnoise_036.json", 0), ("fx_turbnoise_037.json", 2), ("fx_turbnoise_046.json", 0), ("fx_turbnoise_047.json", 0), ("fx_turbnoise_054.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    for n in 33..=60u32 {
        let file = format!("fx_turbnoise_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(&mut gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality);
            }
        }
        // The refused ones (056 to 059) leave nothing for the card; 060 is drawn since D-731 (Hue).
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == (56..=59).contains(&n),
        );
    }
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    let as_added = cpu(&reference(|id| json!([fx(&format!("b526-{id}"), &json!({"size": 60}))])));
    for (what, p) in [
        ("Rotation 30, Offset (40, -25) with Perspective Offset", json!({"size": 60, "rotation": 30, "offset": [40, -25], "perspective_offset": "on"})),
        (
            "Sub Influence 80, Sub Scaling 65, Sub Rotation keyed 0 to 90, Sub Offset (7, 3), Center Subscale on, contrast 200, Soft Clamp",
            json!({"size": 60, "sub_influence": 80, "sub_scaling": 65, "sub_rotation": keyed(&[(0, json!(0)), (239, json!(90))]), "sub_offset": [7, 3],
                "center_subscale": "on", "contrast": 200, "overflow": "soft_clamp"}),
        ),
        ("contrast 250, Wrap Back, Hard Light at opacity 70", json!({"size": 60, "contrast": 250, "overflow": "wrap_back", "blend": "hard_light", "opacity": 70})),
        ("Difference", json!({"size": 60, "blend": "difference"})),
    ] {
        let project = reference(|id| json!([fx(&format!("b526-{id}"), &p)]));
        let drawn = cpu(&project);
        let (changed, moved) = (distance(&drawn, &plain).1, distance(&drawn, &as_added).1);
        t.row(
            &format!("the reference shot, Turbulent Noise {what}: the processor's frame 100 differs from the shot without it and from Turbulent Noise as added, so the comparisons below test D-647"),
            &format!("{changed} pixels changed, {moved} differ from as added"),
            changed > 0 && moved > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(&mut gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Turbulent Noise {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }

    t.heading("Pictures: the street, in `verification/D-647 pictures/`");
    let dir = repo("verification/D-647 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let street = |p: J| picture(&dir, json!([fx("fx-0-0", &p)]), 0);
    let (base, said) = street(json!({"size": 40}));
    write("1_as_added.png", &base);
    t.row("1_as_added.png, Size 40, every setting as added: the clouds as before D-647; draws cleanly", &format!("{said:?}"), said.is_empty());
    let mut seen = vec![base];
    for (name, what, p) in [
        ("2_rotation_45.png", "Rotation 45: the same clouds turned an eighth round the middle", json!({"size": 40, "rotation": 45})),
        ("3_sub_settings.png", "Sub Influence 80, Sub Scaling 70, Sub Rotation 45: busier, softer fine detail, each layer turned further", json!({"size": 40, "sub_influence": 80, "sub_scaling": 70, "sub_rotation": 45})),
        ("4_center_subscale.png", "Center Subscale on, Sub Scaling 70: the fine detail grows from the middle", json!({"size": 40, "sub_scaling": 70, "center_subscale": "on"})),
        ("5_wrap_back.png", "contrast 300, Wrap Back: greys past white fold back to dark, banded", json!({"size": 40, "contrast": 300, "overflow": "wrap_back"})),
        ("6_soft_clamp.png", "contrast 300, Soft Clamp: the same greys eased, no flat white or black patches", json!({"size": 40, "contrast": 300, "overflow": "soft_clamp"})),
        ("7_difference.png", "blending mode Difference: the street's colours flipped where the clouds are light", json!({"size": 40, "blend": "difference"})),
        ("8_hard_light.png", "blending mode Hard Light: the street darkened and lightened hard by the clouds", json!({"size": 40, "blend": "hard_light"})),
    ] {
        let (img, said) = street(p);
        write(name, &img);
        let new = !seen.contains(&img);
        seen.push(img);
        t.row(&format!("{name}, {what}; unlike every picture before; draws cleanly"), &format!("{said:?}, its own: {new}"), said.is_empty() && new);
    }

    t.finish("D-647_turbulent_more_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-461's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, then Turbulent Noise as added and with
/// D-647's settings moved, every eighth frame asked for as the viewer asks, whole. With
/// `B526_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-526: a measurement, run deliberately with --release --ignored"]
fn b526_turbulent_more_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B526_CPU").is_ok();
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
        ("Noise, then Turbulent Noise as added", Some(json!({}))),
        (
            "the same with Rotation 30, Perspective Offset, Sub Settings moved, Center Subscale, Soft Clamp, Hard Light",
            Some(json!({"rotation": 30, "offset": [40, -25], "perspective_offset": "on", "sub_influence": 70, "sub_scaling": 60,
                "sub_rotation": 20, "sub_offset": [3, 2], "center_subscale": "on", "overflow": "soft_clamp", "blend": "hard_light"})),
        ),
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
    let out = std::env::var("B526_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-526_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
