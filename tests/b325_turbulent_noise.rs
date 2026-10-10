//! B-325: D-445, After Effects' Turbulent Noise under `core.turbulent_noise`: Fractal Type, Noise
//! Type, Invert, Contrast, Brightness, size and Scale Width and Height, Offset Turbulence,
//! Complexity, Evolution, Random Seed, Opacity and Blending Mode, a second name over Fractal
//! Noise's engine (as D-383 and D-394, and D-402's Tritone).
//!
//! Writes `verification/D-445_turbulent_noise_table.md` and draws pictures into
//! `verification/D-445 pictures/`.
//!
//! Every expected pixel is `Fixtures/turbulent_noise/expected_turbulent_noise.json`, written by
//! `tools/turbulent_noise_reference.py` before this code existed and printed in document 25 as
//! FX-TURBNOISE-001 to 032; Fractal Noise's own FX-FRACTAL-001 to 028 rerun unchanged. Tolerance
//! 2e-5. Nothing here is a snapshot of a run.

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

/// A Turbulent Noise's settings as it is added, with `p` over them.
fn settings(p: &J) -> J {
    let mut all = json!({"fractal_type": "basic", "noise_type": "smooth", "invert": "off", "contrast": 100, "brightness": 0,
        "size": 100, "scale_width": 100, "scale_height": 100, "offset": [0, 0], "complexity": 6, "evolution": 0, "seed": 0,
        "opacity": 100, "blend": "normal"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    all
}

/// A Turbulent Noise with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    json!({"instance_id": id, "type_id": "core.turbulent_noise", "enabled": true, "parameters": settings(p)})
}

/// The Fractal Noise a Turbulent Noise with the settings `p` names: no speed, black to white,
/// never cycling.
fn fractal(id: &str, p: &J) -> J {
    let mut all = settings(p);
    for (k, v) in [("speed", json!(0)), ("dark_color", json!("#000000")), ("light_color", json!("#ffffff")), ("cycle", json!(0))] {
        all[k] = v;
    }
    json!({"instance_id": id, "type_id": "core.fractal_noise", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=32 {
        let file = format!("fx_turbnoise_{n:03}.json");
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
fn card_reference(t: &mut Table, gpu: &mut Gpu, cases: &[(&str, J, usize)]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p, want) in cases {
        let project = reference(|id| json!([fx(&format!("b325-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Turbulent Noise {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Turbulent Noise {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the still `asset` (in `dir`, `size` pixels), drawn on the processor at `frame`,
/// straight 8-bit, with what it warned of.
fn picture(dir: &Path, asset: &str, size: (usize, usize), effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from(asset);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(size.0);
    comp["height"] = J::from(size.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([size.0 as f64 / 2.0, size.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
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

/// How many neighbouring pixel pairs, across and down, differ by more than `step` levels in red.
fn edges(p: &[u8], (w, h): (usize, usize), step: u8) -> usize {
    let r = |x: usize, y: usize| p[(y * w + x) * 4];
    let mut n = 0;
    for y in 0..h - 1 {
        for x in 0..w - 1 {
            n += (r(x, y).abs_diff(r(x + 1, y)) > step) as usize + (r(x, y).abs_diff(r(x, y + 1)) > step) as usize;
        }
    }
    n
}

#[test]
fn b325_turbulent_noise() {
    let mut t = Table::new(
        "turbulent_noise",
        "# D-445: Turbulent Noise\n\nB-325: `core.turbulent_noise` takes After Effects' Turbulent \
         Noise controls (Fractal Type, Noise Type, Invert, Contrast, Brightness, the size with Scale \
         Width and Height, Offset Turbulence, Complexity, Evolution, Random Seed, Opacity, Blending \
         Mode), a second name over Fractal Noise's engine (as D-383 and D-394): Fractal Noise with no \
         speed, black to white, never cycling. Every expected pixel is \
         `Fixtures/turbulent_noise/expected_turbulent_noise.json`, written by \
         `tools/turbulent_noise_reference.py` before this code existed and printed in document 25 as \
         FX-TURBNOISE-001 to 032; Fractal Noise's FX-FRACTAL-001 to 028 rerun unchanged. Tolerance \
         2e-5.\n",
    );

    t.heading("FX-TURBNOISE-001 to 032 (document 25)");
    t.fixtures_numbered("expected_turbulent_noise.json", 1..=32);

    t.heading("Fractal Noise, the engine under the second name: its fixtures, unchanged");
    let root = t.root.clone();
    t.root = repo("Fixtures/fractal_noise");
    // D-318 widened complexity and brightness, superseding 022 and 024, as b71 says.
    t.fixtures_numbered("expected_fractal_noise.json", 1..=21);
    t.fixtures_numbered("expected_fractal_noise.json", 23..=23);
    t.fixtures_numbered("expected_fractal_noise.json", 25..=28);
    t.root = root;

    t.heading("The file");
    let files: Vec<String> = (1..=32).map(|n| format!("fx_turbnoise_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_turbnoise_017.json");
    t.row(
        "fx_turbnoise_017.json is saved with its fourteen settings and no speed, colours or cycle",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 14 && saved["offset"][0] == 2.5 && saved["offset"][1] == -3.0 && saved["size"] == 4.0 && saved.get("speed").is_none(),
    );
    for (file, want) in [
        ("fx_turbnoise_023.json", "size"),
        ("fx_turbnoise_024.json", "complexity"),
        ("fx_turbnoise_025.json", "contrast"),
        ("fx_turbnoise_027.json", "seed"),
        ("fx_turbnoise_028.json", "scale width"),
        ("fx_turbnoise_029.json", "fractal type"),
        ("fx_turbnoise_030.json", "noise type"),
        ("fx_turbnoise_031.json", "invert"),
        ("fx_turbnoise_032.json", "blending mode"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming Turbulent Noise and its {want}"), &why, why.contains("Turbulent Noise") && why.contains(want));
    }
    let full = r#"{"fractal_type": "basic", "noise_type": "smooth", "invert": "off", "contrast": 100, "brightness": 0, "size": 100, "scale_width": 100, "scale_height": 100, "offset": [0, 0], "complexity": 6, "evolution": 0, "seed": 0, "opacity": 100, "blend": "normal"}"#;
    t.shape_refused("fx_turbnoise_001.json", "a Turbulent Noise whose contrast is a word", &full.replace("\"contrast\": 100", "\"contrast\": \"high\""));
    t.shape_refused("fx_turbnoise_001.json", "a Turbulent Noise without its evolution", &full.replace("\"evolution\": 0, ", ""));
    t.shape_refused("fx_turbnoise_001.json", "a Turbulent Noise whose offset is one number", &full.replace("[0, 0]", "0"));

    t.heading("Commands");
    let mut document = t.load("fx_turbnoise_001.json").document;
    let over = changed(&t, "fx_turbnoise_001.json", |e| if let Effect::TurbulentNoise { complexity, .. } = e { *complexity = 21.0 });
    let bad = changed(&t, "fx_turbnoise_001.json", |e| if let Effect::TurbulentNoise { noise_type, .. } = e { *noise_type = "spline".into() });
    t.refused(
        &mut document,
        vec![
            ("complexity 21", set(over)),
            ("noise type spline", set(bad)),
            ("opacity keyed to 150", keys("opacity", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    let veins = changed(&t, "fx_turbnoise_001.json", |e| {
        if let Effect::TurbulentNoise { fractal_type, size, .. } = e {
            *fractal_type = "turbulent".into();
            *size = 4.0;
        }
    });
    t.taken(
        &mut document,
        "fx_turbnoise_001.json",
        vec![
            ("turbulent at size 4", set(veins)),
            ("evolution keyed from 0 to 720", keys("evolution", &[(0, &[0.0]), (4, &[720.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_turbnoise_002.json", 0),
        ("fx_turbnoise_005.json", 0),
        ("fx_turbnoise_013.json", 2),
        ("fx_turbnoise_017.json", 0),
        ("fx_turbnoise_022.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Refused (023 to 032) leave nothing for the card.
    let none: Vec<u32> = (23..=32).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({}), 3),
            ("turbulent, size 40, complexity 20", json!({"fractal_type": "turbulent", "size": 40, "complexity": 20}), 3),
            ("block, inverted, multiply at opacity 60", json!({"noise_type": "block", "invert": "on", "size": 30, "blend": "multiply", "opacity": 60}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-445 pictures/`");
    let dir = repo("verification/D-445 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]), 0);
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]), 0);
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());

    let grey = |p: &[u8]| p.chunks_exact(4).all(|q| q[0] == q[1] && q[1] == q[2]);
    let (added, said) = street(json!({}));
    write("2_as_added.png", TOWN, &added);
    let (later, _) = picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &json!({}))]), 4);
    t.row(
        "2_as_added.png, as added: the street covered by large soft grey clouds, every pixel a grey; frame 4 the same, as Turbulent Noise has no speed of its own; draws cleanly",
        &format!("{said:?}, all grey: {}, frame 4 the same: {}", grey(&added), later == added),
        said.is_empty() && grey(&added) && later == added && added != before,
    );

    let (fine, said) = street(json!({"size": 30}));
    write("3_size_30.png", TOWN, &fine);
    let (veins, said1) = street(json!({"size": 30, "fractal_type": "turbulent"}));
    write("4_turbulent.png", TOWN, &veins);
    let dark = |p: &[u8]| p.chunks_exact(4).filter(|q| q[0] < 40).count();
    t.row(
        "4_turbulent.png, fractal type turbulent at size 30: the clouds creased into dark veins, more very dark pixels than 3_size_30.png's basic clouds; both draw cleanly",
        &format!("{said:?} {said1:?}, pixels darker than 40: basic {}, turbulent {}", dark(&fine), dark(&veins)),
        said.is_empty() && said1.is_empty() && dark(&veins) > dark(&fine),
    );

    let (block, said) = street(json!({"size": 30, "noise_type": "block"}));
    write("5_block.png", TOWN, &block);
    t.row(
        "5_block.png, noise type block at size 30: square steps, more sharp edges between neighbours than 3_size_30.png; draws cleanly",
        &format!("{said:?}, neighbours more than 8 levels apart: smooth {}, block {}", edges(&fine, TOWN, 8), edges(&block, TOWN, 8)),
        said.is_empty() && edges(&block, TOWN, 8) > edges(&fine, TOWN, 8),
    );

    let (inverted, said) = street(json!({"size": 30, "invert": "on"}));
    write("6_inverted.png", TOWN, &inverted);
    let turned = fine.chunks_exact(4).zip(inverted.chunks_exact(4)).filter(|(a, _)| a[3] == 255).all(|(a, b)| (a[0] as i32 + b[0] as i32 - 255).abs() <= 1);
    t.row(
        "6_inverted.png, invert on at size 30: 3_size_30.png's clouds turned over, light where they were dark; draws cleanly",
        &format!("{said:?}, turned over: {turned}"),
        said.is_empty() && turned && inverted != fine,
    );

    let (over, said) = street(json!({"size": 60, "blend": "multiply", "opacity": 70}));
    write("7_multiply_over_the_street.png", TOWN, &over);
    let darker = before.chunks_exact(4).zip(over.chunks_exact(4)).all(|(b, a)| (0..3).all(|c| a[c] <= b[c]));
    t.row(
        "7_multiply_over_the_street.png, multiply at opacity 70, size 60: the street darkened by the clouds, no pixel lighter; draws cleanly",
        &format!("{said:?}, none lighter: {darker}"),
        said.is_empty() && darker && over != before,
    );

    // The second name: Fractal Noise with the same settings, no speed, black to white and never
    // cycling, draws the same street byte for byte.
    for (what, p) in [
        ("as added", json!({})),
        ("turbulent at size 30", json!({"size": 30, "fractal_type": "turbulent"})),
        ("block, inverted, seed 9, offset 20 by -7", json!({"size": 30, "noise_type": "block", "invert": "on", "seed": 9, "offset": [20, -7]})),
        ("scale 250 by 40, evolution 200, screen at 50", json!({"size": 25, "scale_width": 250, "scale_height": 40, "evolution": 200, "blend": "screen", "opacity": 50})),
    ] {
        let (a, said0) = street(p.clone());
        let (b, said1) = picture(&dir, "town.png", TOWN, json!([fractal("fx-0-0", &p)]), 0);
        t.row(
            &format!("Turbulent Noise {what} against Fractal Noise with the same settings, speed 0, black to white, cycle 0: the same street byte for byte; both draw cleanly"),
            &format!("{said0:?} {said1:?}, {} pixels differ", distance(&a, &b).1),
            said0.is_empty() && said1.is_empty() && a == b && a != before,
        );
    }

    t.finish("D-445_turbulent_noise_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B325_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-325: a measurement, run deliberately with --release --ignored"]
fn b325_turbulent_noise_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B325_CPU").is_ok();
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
        ("Noise, then Turbulent Noise as added (complexity 6)", Some(json!({}))),
        ("Noise, then Turbulent Noise turbulent, size 40, complexity 20 (the most)", Some(json!({"fractal_type": "turbulent", "size": 40, "complexity": 20}))),
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
    let out = std::env::var("B325_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-325_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
