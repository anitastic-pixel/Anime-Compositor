//! B-600: D-730, Turbulent Noise's Saturation, Color, Luminosity, None, Stencil Alpha and Stencil
//! Luma blending modes.
//!
//! Writes `verification/D-730_turbulent_blends_table.md` and draws pictures into
//! `verification/D-730 pictures/`.
//!
//! Every expected pixel is `Fixtures/turbulent_noise/expected_turbulent_blends.json`, written by
//! `tools/turbulent_blends_reference.py` before this code existed and printed in document 25 as
//! FX-TURBNOISE-061 to 074; FX-TURBNOISE-001 to 060 and FX-TURBTYPE-001 to 025 rerun unchanged.
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
fn b600_turbulent_blends() {
    let mut t = Table::new(
        "turbulent_noise",
        "# D-730: Turbulent Noise, six more blending modes\n\nB-600: `core.turbulent_noise` takes Saturation, Color, \
         Luminosity, None, Stencil Alpha and Stencil Luma. Saturation, Color and Luminosity follow the W3C Compositing \
         and Blending Level 1 non-separable rules (Lum = 0.3 R + 0.59 G + 0.11 B, SetLum with ClipColor, SetSat): this \
         is our basis, not a match to After Effects, whose own rule is unpublished. Turbulent Noise's colour is a grey, \
         so Saturation and Color both give the layer the noise's grey at the layer's own lightness, and Luminosity \
         gives the layer's colour the noise's lightness; each mixes by Opacity as the other modes do. Adobe's help says \
         Fractal Noise's modes are the Timeline's and None draws the noise only, so None replaces the layer with the \
         noise at Opacity (D-413), and the stencils keep the layer where the noise's alpha (Stencil Alpha) or lightness \
         (Stencil Luma) is, as the Timeline's stencils do (D-301). Silhouette Alpha and Silhouette Luma are refused: \
         the program has no Timeline rule for them to follow. Hue (FX-TURBNOISE-060) and Overlay (FX-TURBNOISE-032) \
         were refused here; D-731, by the owner's decision of 2026-10-13, regenerated both and builds them. Every expected pixel is \
         `Fixtures/turbulent_noise/expected_turbulent_blends.json`, written by `tools/turbulent_blends_reference.py` \
         before this code existed and printed in document 25 as FX-TURBNOISE-061 to 074. Tolerance 2e-5.\n",
    );

    t.heading("FX-TURBNOISE-061 to 074 (document 25)");
    t.fixtures_numbered("expected_turbulent_blends.json", 61..=74);

    t.heading("Turbulent Noise's earlier fixtures, unchanged");
    t.fixtures_numbered("expected_turbulent_noise.json", 1..=32);
    t.fixtures_numbered("expected_turbulent_more.json", 33..=60);
    let root = t.root.clone();
    t.root = repo("Fixtures/turbulent_noise_types");
    t.fixtures_numbered("expected_turbulent_noise_types.json", 1..=25);
    t.root = root;

    t.heading("The file");
    let all: Vec<String> = (61..=74).map(|n| format!("fx_turbnoise_{n:03}.json")).collect();
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    for (file, mode) in [
        ("fx_turbnoise_061.json", "saturation"),
        ("fx_turbnoise_062.json", "color"),
        ("fx_turbnoise_063.json", "luminosity"),
        ("fx_turbnoise_065.json", "none"),
        ("fx_turbnoise_067.json", "stencil_alpha"),
        ("fx_turbnoise_069.json", "stencil_luma"),
    ] {
        let saved = t.saved_parameters(file);
        t.row(&format!("{file} is saved with blending mode {mode}"), &saved["blend"].to_string(), saved["blend"] == mode);
    }
    for (file, words) in [
        ("fx_turbnoise_073.json", ["blending mode", "\"silhouette_alpha\""]),
        ("fx_turbnoise_074.json", ["blending mode", "\"silhouette_luma\""]),
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
        ("blending mode \"silhouette_alpha\"", set(with(|e| if let Effect::TurbulentNoise { blend, .. } = e { *blend = "silhouette_alpha".into() }))),
        ("blending mode \"silhouette_luma\"", set(with(|e| if let Effect::TurbulentNoise { blend, .. } = e { *blend = "silhouette_luma".into() }))),
    ];
    let taken = vec![
        ("blending mode luminosity", set(with(|e| if let Effect::TurbulentNoise { blend, .. } = e { *blend = "luminosity".into() }))),
        ("blending mode stencil luma", set(with(|e| if let Effect::TurbulentNoise { blend, .. } = e { *blend = "stencil_luma".into() }))),
    ];
    t.refused(&mut document, refused);
    t.taken(&mut document, "fx_turbnoise_002.json", taken);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_turbnoise_061.json", 0), ("fx_turbnoise_063.json", 0), ("fx_turbnoise_066.json", 0), ("fx_turbnoise_070.json", 0), ("fx_turbnoise_072.json", 2)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    for n in 61..=74u32 {
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
        // The refused ones (073 and 074) leave nothing for the card.
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == (n >= 73),
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
    let as_added = cpu(&reference(|id| json!([fx(&format!("b600-{id}"), &json!({"size": 60}))])));
    for (what, p) in [
        ("Saturation", json!({"size": 60, "blend": "saturation"})),
        ("Luminosity at opacity 70", json!({"size": 60, "blend": "luminosity", "opacity": 70})),
        ("None at opacity 60", json!({"size": 60, "blend": "none", "opacity": 60})),
        ("Stencil Luma, contrast 200", json!({"size": 60, "blend": "stencil_luma", "contrast": 200})),
    ] {
        let project = reference(|id| json!([fx(&format!("b600-{id}"), &p)]));
        let drawn = cpu(&project);
        let (changed, moved) = (distance(&drawn, &plain).1, distance(&drawn, &as_added).1);
        t.row(
            &format!("the reference shot, Turbulent Noise {what}: the processor's frame 100 differs from the shot without it and from Turbulent Noise as added, so the comparisons below test D-730"),
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

    t.heading("Pictures: the street, in `verification/D-730 pictures/`");
    let dir = repo("verification/D-730 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let street = |p: J| picture(&dir, json!([fx("fx-0-0", &p)]), 0);
    let (base, said) = street(json!({"size": 40}));
    write("1_normal.png", &base);
    t.row("1_normal.png, Size 40, blending mode Normal: the clouds as before D-730; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (img, said) = street(json!({"size": 40, "blend": "none"}));
    write("4_none.png", &img);
    t.row(
        "4_none.png, None: the clouds alone, the street gone; the street is solid everywhere, so this is the same picture as 1_normal.png (Normal at full opacity also covers it); draws cleanly",
        &format!("{said:?}, the same as 1_normal.png: {}", img == base),
        said.is_empty() && img == base,
    );
    let mut seen = vec![base];
    for (name, what, p) in [
        ("2_saturation.png", "Saturation: the street's colours drained to grey, its light and dark kept", json!({"size": 40, "blend": "saturation"})),
        ("3_luminosity.png", "Luminosity: the street's colours kept, lit and darkened by the clouds", json!({"size": 40, "blend": "luminosity"})),
        ("5_none_60.png", "None at opacity 60: the clouds alone, see-through, the street gone", json!({"size": 40, "blend": "none", "opacity": 60})),
        ("6_stencil_luma.png", "Stencil Luma: the street shows only where the clouds are light, clear where they are dark", json!({"size": 40, "blend": "stencil_luma"})),
        ("7_stencil_luma_50.png", "Stencil Luma at opacity 50: the street fainter, holes where the clouds are dark", json!({"size": 40, "blend": "stencil_luma", "opacity": 50})),
    ] {
        let (img, said) = street(p);
        write(name, &img);
        let new = !seen.contains(&img);
        seen.push(img);
        t.row(&format!("{name}, {what}; unlike every picture before; draws cleanly"), &format!("{said:?}, its own: {new}"), said.is_empty() && new);
    }

    t.finish("D-730_turbulent_blends_table.md");
}


fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-461's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, then Turbulent Noise as added and with
/// D-730's modes, every eighth frame asked for as the viewer asks, whole. With
/// `B600_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-600: a measurement, run deliberately with --release --ignored"]
fn b600_turbulent_blends_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B600_CPU").is_ok();
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
    let shots: [(&str, Option<J>); 5] = [
        ("Noise alone", None),
        ("Noise, then Turbulent Noise as added", Some(json!({}))),
        ("the same in Luminosity", Some(json!({"blend": "luminosity"}))),
        ("the same in None at opacity 60", Some(json!({"blend": "none", "opacity": 60}))),
        ("the same in Stencil Luma", Some(json!({"blend": "stencil_luma"}))),
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
    let out = std::env::var("B600_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-600_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
