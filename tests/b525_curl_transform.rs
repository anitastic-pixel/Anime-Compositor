//! B-525: D-646, Curl Noise's Transform, after After Effects' Curl Noise Rotation, Scale and
//! Perspective Offset: the made noise turned and scaled round the drawing's middle, and each finer
//! octave slid half as far.
//!
//! Writes `verification/D-646_curl_transform_table.md` and draws pictures into
//! `verification/D-646 pictures/`.
//!
//! Every expected pixel is `Fixtures/curl_noise/expected_curl_transform.json`, written by
//! `tools/curl_transform_reference.py` before this code existed and printed in document 25 as
//! FX-CURL-061 to 077. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// The Curl Noises the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::CurlNoise { .. })))
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

/// A Curl Noise with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"source": "internal", "speed": 10, "direction": 0, "size": 100, "offset": [0, 0], "evolution": 0,
        "turbulence_speed": 20, "swirl": 45, "density": 0, "smoothness": 50, "vertical_bias": 50, "sample_count": 12, "sample_radius": 30,
        "flow_softness": 20, "edge_definition": 50, "flow_falloff": 0, "view": "final_render", "contrast": 100, "brightness": 0,
        "clip_hdr_results": "on", "channel": "rgb"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.curl_noise", "enabled": true, "parameters": all})
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
fn b525_curl_transform() {
    let mut t = Table::new(
        "curl_noise",
        "# D-646: Curl Noise, Rotation, Scale and Perspective Offset\n\nB-525: `core.curl_noise` takes After Effects' \
         Transform controls. With c the drawing's middle, m = 100 / Scale and theta = Rotation, octave o of the made noise \
         is read at c + m R(-theta) (p - w_o (Offset + drift frame) - c), w_o = 0.5^o with Perspective Offset on, else 1; \
         its slope is brought back by m R(theta) before Swirl and Vertical Bias, and the seed is read at octave 0's point. \
         With Source This Layer or Other Layer the picture is read unturned, so Rotation and Scale move only the seed. \
         Rotation 0, Scale 100 and Perspective Offset off draw every file before D-646 as before. The rule is this \
         program's own, after Fractal Noise's same controls; Adobe publishes none. Every expected pixel is \
         `Fixtures/curl_noise/expected_curl_transform.json`, written by `tools/curl_transform_reference.py` before this \
         code existed and printed in document 25 as FX-CURL-061 to 077. Tolerance 2e-5.\n",
    );

    t.heading("FX-CURL-061 to 077 (document 25)");
    t.fixtures_numbered("expected_curl_transform.json", 61..=77);

    t.heading("The file");
    let all: Vec<String> = (61..=77).map(|n| format!("fx_curl_{n:03}.json")).collect();
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let old = t.saved_parameters("fx_curl_001.json");
    t.row(
        "fx_curl_001.json, from before the Transform, opens as added and saves without the three new settings",
        &old.to_string(),
        ["rotation", "scale", "perspective_offset"].iter().all(|k| old.get(k).is_none()),
    );
    let saved = t.saved_parameters("fx_curl_067.json");
    t.row(
        "fx_curl_067.json is saved with Rotation 45, Scale 150 and Perspective Offset on",
        &saved.to_string(),
        saved["rotation"] == json!(45) && saved["scale"] == json!(150) && saved["perspective_offset"] == "on",
    );
    let saved = t.saved_parameters("fx_curl_070.json");
    t.row("fx_curl_070.json is saved with Rotation's keys kept", &saved["rotation"].to_string(), saved["rotation"]["keyframes"][1]["value"] == json!(90));
    for (file, want) in [
        ("fx_curl_074.json", "Curl Noise's scale runs from 1 to 10000, and this is 0.5."),
        ("fx_curl_075.json", "Curl Noise's rotation runs from -100000 to 100000, and this is 200000."),
        ("fx_curl_076.json", "Curl Noise's perspective offset is \"off\" or \"on\", and this is \"yes\"."),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == want);
    }

    t.heading("Commands");
    let mut document = t.load("fx_curl_072.json").document;
    let with = |f: fn(&mut Effect)| changed(&t, "fx_curl_072.json", f);
    let refused = vec![
        ("scale 0.5", set(with(|e| if let Effect::CurlNoise { scale, .. } = e { *scale = 0.5 }))),
        ("rotation 200000", set(with(|e| if let Effect::CurlNoise { rotation, .. } = e { *rotation = 200000.0 }))),
        ("perspective offset \"yes\"", set(with(|e| if let Effect::CurlNoise { perspective_offset, .. } = e { *perspective_offset = "yes".into() }))),
    ];
    let taken = vec![(
        "rotation -45, scale 300, perspective offset on",
        set(with(|e| {
            if let Effect::CurlNoise { rotation, scale, perspective_offset, .. } = e {
                *rotation = -45.0;
                *scale = 300.0;
                *perspective_offset = "on".into();
            }
        })),
    )];
    t.refused(&mut document, refused);
    t.taken(&mut document, "fx_curl_072.json", taken);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_curl_062.json", 0), ("fx_curl_067.json", 2), ("fx_curl_069.json", 0), ("fx_curl_071.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    for n in 61..=77u32 {
        let file = format!("fx_curl_{n:03}.json");
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
        // The refused ones (074 to 077) leave nothing for the card.
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == (n >= 74),
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
    let as_added = cpu(&reference(|id| json!([fx(&format!("b525-{id}"), &json!({}))])));
    for (what, p) in [
        ("Rotation 30", json!({"rotation": 30})),
        ("Rotation 45, Scale 250, Perspective Offset on, Speed 40", json!({"rotation": 45, "scale": 250, "perspective_offset": "on", "speed": 40})),
        ("Rotation keyed 0 to 720, Curl Generation", json!({"rotation": keyed(&[(0, json!(0)), (239, json!(720))]), "view": "curl_generation"})),
        ("This Layer, Rotation 90, Scale 50", json!({"source": "this_layer", "rotation": 90, "scale": 50})),
    ] {
        let project = reference(|id| json!([fx(&format!("b525-{id}"), &p)]));
        let drawn = cpu(&project);
        let (changed, moved) = (distance(&drawn, &plain).1, distance(&drawn, &as_added).1);
        t.row(
            &format!("the reference shot, Curl Noise {what}: the processor's frame 100 differs from the shot without it and from Curl Noise as added, so the comparisons below test the Transform"),
            &format!("{changed} pixels changed, {moved} differ from as added"),
            changed > 0 && moved > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(&mut gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Curl Noise {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }

    t.heading("Pictures: the street, in `verification/D-646 pictures/`");
    let dir = repo("verification/D-646 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let street = |p: J| picture(&dir, json!([fx("fx-0-0", &p)]), 0);
    let (plain, said) = street(json!({"size": 40, "view": "input_noise"}));
    write("1_noise.png", &plain);
    t.row("1_noise.png, Size 40, View Input Noise: the noise as made, its blobs upright; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (turned, said) = street(json!({"size": 40, "view": "input_noise", "rotation": 90}));
    write("2_noise_rotation_90.png", &turned);
    t.row(
        "2_noise_rotation_90.png, the same with Rotation 90: the same blobs turned a quarter clockwise round the middle; draws cleanly",
        &format!("{said:?}, differs from 1_noise.png {}", turned != plain),
        said.is_empty() && turned != plain,
    );
    let (bigger, said) = street(json!({"size": 40, "view": "input_noise", "scale": 300}));
    write("3_noise_scale_300.png", &bigger);
    t.row(
        "3_noise_scale_300.png, the same with Scale 300: the blobs three times as big, grown from the middle; draws cleanly",
        &format!("{said:?}, differs from 1_noise.png {}", bigger != plain),
        said.is_empty() && bigger != plain,
    );
    let (lines, said) = street(json!({"size": 40, "rotation": 45}));
    let (upright, _) = street(json!({"size": 40}));
    write("4_flow_upright.png", &upright);
    write("5_flow_rotation_45.png", &lines);
    t.row(
        "4_flow_upright.png and 5_flow_rotation_45.png, the flow lines at Rotation 0 and 45: the second's swirls are the first's turned an eighth; draws cleanly",
        &format!("{said:?}, they differ {}", lines != upright),
        said.is_empty() && lines != upright,
    );

    t.finish("D-646_curl_transform_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-461's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, then Curl Noise as added and with the
/// Transform moved, every eighth frame asked for as the viewer asks, whole. With `B525_CPU`
/// set, the processor draws instead.
#[test]
#[ignore = "B-525: a measurement, run deliberately with --release --ignored"]
fn b525_curl_transform_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B525_CPU").is_ok();
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
        ("Noise, then Curl Noise as added", Some(json!({}))),
        ("the same with Rotation 45, Scale 150, Perspective Offset on", Some(json!({"rotation": 45, "scale": 150, "perspective_offset": "on"}))),
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
    let out = std::env::var("B525_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-525_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
