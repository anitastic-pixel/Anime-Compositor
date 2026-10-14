//! B-601: D-731, Turbulent Noise's Overlay and Hue blending modes, by the owner's specification
//! decision of 2026-10-13.
//!
//! Writes `verification/D-731_turbulent_overlay_hue_table.md` and draws pictures into
//! `verification/D-731 pictures/`.
//!
//! Every expected pixel is FX-TURBNOISE-032 (`Fixtures/turbulent_noise/expected_turbulent_noise.json`)
//! and FX-TURBNOISE-060 (`expected_turbulent_more.json`), regenerated in their own commit before this
//! code. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, changed, distance, median, reference, repo, set, town, Table, MAIN, TOWN,
};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

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
fn b601_turbulent_overlay_hue() {
    let mut t = Table::new(
        "turbulent_noise",
        "# D-731: Turbulent Noise, Overlay and Hue\n\nB-601: `core.turbulent_noise` takes Overlay and Hue, by the owner's \
         specification decision of 2026-10-13, which regenerated FX-TURBNOISE-032 (Overlay) and FX-TURBNOISE-060 (Hue) \
         from refused to drawn. Overlay is the Timeline's Overlay formula (D-301, `grade::mixer`: 2 b g where the layer \
         b is up to one half, else 1 - 2 (1 - b)(1 - g)), taken on the straight linear colour as every other mode of this \
         effect is; the Timeline takes it on the encoded colour. Hue is the W3C Compositing and Blending Level 1 Hue, \
         SetLum(SetSat(noise, Sat(layer)), Lum(layer)), the same basis as D-730's Saturation, Color and Luminosity, not \
         a match to After Effects. The noise is a grey, so Hue gives the layer its own grey at its own lightness, the \
         same picture as Saturation and Color. Each mixes by Opacity as the other modes do. Every expected pixel is \
         `Fixtures/turbulent_noise/expected_turbulent_noise.json` (032) and `expected_turbulent_more.json` (060), \
         regenerated by `tools/turbulent_noise_reference.py` and `tools/turbulent_more_reference.py` in their own commit \
         before this code. Tolerance 2e-5.\n",
    );

    t.heading("FX-TURBNOISE-032 and 060 (document 25, regenerated by D-731)");
    t.fixtures_numbered("expected_turbulent_noise.json", 32..=32);
    t.fixtures_numbered("expected_turbulent_more.json", 60..=60);

    t.heading("The file");
    t.round_trips(&["fx_turbnoise_032.json", "fx_turbnoise_060.json"]);
    for (file, mode) in [("fx_turbnoise_032.json", "overlay"), ("fx_turbnoise_060.json", "hue")] {
        let saved = t.saved_parameters(file);
        t.row(&format!("{file} is saved with blending mode {mode}"), &saved["blend"].to_string(), saved["blend"] == mode);
    }

    t.heading("Commands");
    let mut document = t.load("fx_turbnoise_002.json").document;
    let with = |f: fn(&mut Effect)| changed(&t, "fx_turbnoise_002.json", f);
    let taken = vec![
        ("blending mode overlay", set(with(|e| if let Effect::TurbulentNoise { blend, .. } = e { *blend = "overlay".into() }))),
        ("blending mode hue", set(with(|e| if let Effect::TurbulentNoise { blend, .. } = e { *blend = "hue".into() }))),
    ];
    t.taken(&mut document, "fx_turbnoise_002.json", taken);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_turbnoise_032.json", 0), ("fx_turbnoise_060.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    for n in [32u32, 60] {
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
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && card == 10,
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
    let as_added = cpu(&reference(|id| json!([fx(&format!("b601-{id}"), &json!({"size": 60}))])));
    for (what, p) in [
        ("Overlay", json!({"size": 60, "blend": "overlay"})),
        ("Overlay at opacity 50, contrast 200", json!({"size": 60, "blend": "overlay", "opacity": 50, "contrast": 200})),
        ("Hue", json!({"size": 60, "blend": "hue"})),
        ("Hue at opacity 60", json!({"size": 60, "blend": "hue", "opacity": 60})),
    ] {
        let project = reference(|id| json!([fx(&format!("b601-{id}"), &p)]));
        let drawn = cpu(&project);
        let (changed, moved) = (distance(&drawn, &plain).1, distance(&drawn, &as_added).1);
        t.row(
            &format!("the reference shot, Turbulent Noise {what}: the processor's frame 100 differs from the shot without it and from Turbulent Noise as added, so the comparisons below test D-731"),
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

    t.heading("Pictures: the street, in `verification/D-731 pictures/`");
    let dir = repo("verification/D-731 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let street = |p: J| picture(&dir, json!([fx("fx-0-0", &p)]), 0);
    let (base, said) = street(json!({"size": 40}));
    write("1_normal.png", &base);
    t.row("1_normal.png, Size 40, blending mode Normal: the clouds as before D-731; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (saturation, _) = street(json!({"size": 40, "blend": "saturation"}));
    let mut seen = vec![base];
    for (name, what, p) in [
        ("2_overlay.png", "Overlay: the street's light parts lighter and dark parts darker where the clouds are, its colours kept", json!({"size": 40, "blend": "overlay"})),
        ("3_overlay_contrast.png", "Overlay, contrast 200: the same, stronger", json!({"size": 40, "blend": "overlay", "contrast": 200})),
        ("4_hue_60.png", "Hue at opacity 60: the street's colours partly drained, its light and dark kept", json!({"size": 40, "blend": "hue", "opacity": 60})),
    ] {
        let (img, said) = street(p);
        write(name, &img);
        let new = !seen.contains(&img);
        seen.push(img);
        t.row(&format!("{name}, {what}; unlike every picture before; draws cleanly"), &format!("{said:?}, its own: {new}"), said.is_empty() && new);
    }
    let (img, said) = street(json!({"size": 40, "blend": "hue"}));
    write("5_hue.png", &img);
    t.row(
        "5_hue.png, Hue: the street in its own greys; the noise is grey, so this is the same picture as Saturation (D-730's 2_saturation.png); draws cleanly",
        &format!("{said:?}, the same as Saturation: {}", img == saturation),
        said.is_empty() && img == saturation && !seen.contains(&img),
    );

    t.finish("D-731_turbulent_overlay_hue_table.md");
}

/// The frame times, B-461's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, then Turbulent Noise as added and in
/// Overlay and Hue, every eighth frame asked for as the viewer asks, whole. With `B601_CPU`
/// set, the processor draws instead.
#[test]
#[ignore = "B-601: a measurement, run deliberately with --release --ignored"]
fn b601_turbulent_overlay_hue_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B601_CPU").is_ok();
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
    let shots: [(&str, Option<J>); 4] = [
        ("Noise alone", None),
        ("Noise, then Turbulent Noise as added", Some(json!({}))),
        ("the same in Overlay", Some(json!({"blend": "overlay"}))),
        ("the same in Hue", Some(json!({"blend": "hue"}))),
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
    let out = std::env::var("B601_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-601_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
