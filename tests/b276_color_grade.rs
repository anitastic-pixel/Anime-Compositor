//! B-276: D-397, Color Grade, after Lumetri Color under our own name, its first unit: Basic
//! Correction (white balance, exposure, the tone sliders, saturation), Creative (a look file,
//! its intensity, faded film, vibrance, saturation, shadow and highlight tints) and Vignette.
//!
//! Writes `verification/D-397_color_grade_table.md` and draws pictures into
//! `verification/D-397 pictures/`.
//!
//! Every expected pixel is `Fixtures/color_grade/expected_color_grade.json`, written by
//! `tools/color_grade_reference.py` before this code existed and printed in document 25 as
//! FX-GRADE-001 to 034. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, repo, set, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::{Effect, COLOR_GRADE_SETTINGS};
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

fn is_cg(e: &Effect) -> bool {
    matches!(e, Effect::ColorGrade { .. })
}

/// The Color Grades the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if is_cg(&f.instance.effect)))
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

/// A Color Grade with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"look": ""});
    for (name, _, _, start) in COLOR_GRADE_SETTINGS {
        all[name] = json!(start);
    }
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.color_grade", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` leave nothing for the card.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=34 {
        let file = format!("fx_grade_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b276-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Color Grade {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Color Grade {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the street, drawn, straight 8-bit, with what it warned of. The project has the
/// fixtures' look file as `asset-look`.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    project["assets"].as_array_mut().unwrap().push(json!({"id": "asset-look", "kind": "lut", "name": "look", "path": "look.cube"}));
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
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

/// Color Grade's setting `name` in `e`, set to `v`.
fn with(e: &mut Effect, name: &str, v: f64) {
    if let Effect::ColorGrade { values, .. } = e {
        values[COLOR_GRADE_SETTINGS.iter().position(|s| s.0 == name).unwrap()] = v;
    }
}

/// The mean of red less blue over the covered pixels, in levels.
fn warmth(p: &[u8]) -> f64 {
    let px: Vec<&[u8]> = p.chunks_exact(4).filter(|q| q[3] > 0).collect();
    px.iter().map(|q| q[0] as f64 - q[2] as f64).sum::<f64>() / px.len().max(1) as f64
}

/// The darkest channel over the covered pixels.
fn darkest(p: &[u8]) -> u8 {
    p.chunks_exact(4).filter(|q| q[3] > 0).flat_map(|q| q[..3].to_vec()).min().unwrap_or(0)
}

/// The brightness, the three channels summed, of the pixel at (x, y).
fn at(p: &[u8], x: usize, y: usize) -> u32 {
    let i = 4 * (y * TOWN.0 as usize + x);
    p[i..i + 3].iter().map(|&v| v as u32).sum()
}

#[test]
fn b276_color_grade() {
    let mut t = Table::new(
        "color_grade",
        "# D-397: Color Grade\n\nB-276, after Lumetri Color under our own name, its first unit: \
         Basic Correction (white balance, exposure, contrast, highlights, shadows, whites, blacks, \
         saturation), Creative (a look file at its intensity, faded film, vibrance, saturation, \
         shadow and highlight tints) and Vignette, each step in that order. Every expected pixel \
         is `Fixtures/color_grade/expected_color_grade.json`, written by \
         `tools/color_grade_reference.py` before this code existed and printed in document 25 as \
         FX-GRADE-001 to 034.\n",
    );

    t.heading("FX-GRADE-001 to 034 (document 25)");
    t.fixtures_numbered("expected_color_grade.json", 1..=34);

    t.heading("The file");
    let files: Vec<String> = (1..=34).map(|n| format!("fx_grade_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_grade_024.json");
    t.row(
        "fx_grade_024.json is saved with its 23 settings, the look and 22 numbers",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 23 && saved["look"] == "asset-look" && saved["look_intensity"] == 80.0,
    );
    for (file, want) in [
        ("fx_grade_028.json", "temperature"),
        ("fx_grade_029.json", "exposure"),
        ("fx_grade_030.json", "saturation"),
        ("fx_grade_031.json", "look intensity"),
        ("fx_grade_032.json", "shadow tint hue"),
        ("fx_grade_033.json", "vignette amount"),
        ("fx_grade_034.json", "vignette roundness"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    let mut word = fx("x", &json!({}))["parameters"].clone();
    word["exposure"] = J::from("1");
    t.shape_refused("fx_grade_001.json", "a Color Grade whose exposure is a word", &word.to_string());
    let mut lookless = fx("x", &json!({}))["parameters"].clone();
    lookless.as_object_mut().unwrap().remove("look");
    t.shape_refused("fx_grade_001.json", "a Color Grade without its look", &lookless.to_string());

    t.heading("Commands");
    let mut document = t.load("fx_grade_001.json").document;
    let hot = changed(&t, "fx_grade_001.json", |e| with(e, "temperature", 101.0));
    let elsewhere = changed(&t, "fx_grade_001.json", |e| {
        if let Effect::ColorGrade { look, .. } = e {
            *look = "asset-colours".into();
        }
    });
    t.refused(
        &mut document,
        vec![
            ("temperature 101", set(hot)),
            ("look \"asset-colours\", a picture and not a lookup file", set(elsewhere)),
            ("exposure keyed to 6", keys("exposure", &[(0, &[0.0]), (4, &[6.0])])),
        ],
    );
    let graded = changed(&t, "fx_grade_001.json", |e| {
        with(e, "contrast", 40.0);
        with(e, "faded_film", 30.0);
        if let Effect::ColorGrade { look, .. } = e {
            *look = "asset-look".into();
        }
    });
    t.taken(
        &mut document,
        "fx_grade_001.json",
        vec![("contrast 40, faded film 30, the look file", set(graded)), ("exposure keyed from -2 to 2", keys("exposure", &[(0, &[-2.0]), (4, &[2.0])]))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_grade_001.json", 0), ("fx_grade_006.json", 0), ("fx_grade_013.json", 0), ("fx_grade_023.json", 0), ("fx_grade_024.json", 0), ("fx_grade_026.json", 3)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // As added (001), only the look at intensity 0 (016) and refused (028 to 034) leave nothing
    // for the card.
    let none: Vec<u32> = [1, 16].into_iter().chain(28..=34).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("basic: temperature 30, exposure 0.5, contrast 40, shadows 30, saturation 120", json!({"temperature": 30, "exposure": 0.5, "contrast": 40, "shadows": 30, "saturation": 120}), 3),
            (
                "creative: faded film 40, vibrance 50, teal shadows and orange highlights",
                json!({"faded_film": 40, "vibrance": 50, "shadow_tint_hue": 195, "shadow_tint_amount": 60, "highlight_tint_hue": 35, "highlight_tint_amount": 50}),
                3,
            ),
            ("highlights -40, whites 30 and a vignette of -3", json!({"highlights": -40, "whites": 30, "vignette_amount": -3, "vignette_roundness": 50}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-397 pictures/`");
    let dir = repo("verification/D-397 pictures");
    fs::create_dir_all(&dir).unwrap();
    fs::copy(repo("Fixtures/color_grade/luts/look.cube"), dir.join("look.cube")).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let one = |p: J| json!([fx("fx-0-0", &p)]);
    let (before, said) = picture(&dir, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (same, said) = picture(&dir, one(json!({})));
    t.row(
        "Color Grade as added changes nothing: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );
    let (warm, said) = picture(&dir, one(json!({"temperature": 60})));
    write("2_warm.png", &warm);
    let (w0, w1) = (warmth(&before), warmth(&warm));
    t.row(
        "2_warm.png, temperature 60: the street warmer; draws cleanly",
        &format!("{said:?}, red less blue {w0:.1} before, {w1:.1} after"),
        said.is_empty() && w1 > w0,
    );
    let (teal, said) = picture(&dir, one(json!({"contrast": 30, "shadow_tint_hue": 195, "shadow_tint_amount": 60, "highlight_tint_hue": 35, "highlight_tint_amount": 50})));
    write("3_teal_orange.png", &teal);
    t.row(
        "3_teal_orange.png, contrast 30, teal shadows and orange highlights: the street changed; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&teal, &before).1),
        said.is_empty() && teal != before,
    );
    let (look, said) = picture(&dir, one(json!({"look": "asset-look"})));
    write("4_look.png", &look);
    t.row(
        "4_look.png, the fixtures' own look file at intensity 100: the street changed; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&look, &before).1),
        said.is_empty() && look != before,
    );
    let (faded, said) = picture(&dir, one(json!({"faded_film": 70})));
    write("5_faded_film.png", &faded);
    t.row(
        "5_faded_film.png, faded film 70: the darkest level lifted; draws cleanly",
        &format!("{said:?}, darkest level {} before, {} after", darkest(&before), darkest(&faded)),
        said.is_empty() && darkest(&faded) > darkest(&before),
    );
    let (dark, said) = picture(&dir, one(json!({"vignette_amount": -3})));
    write("6_vignette.png", &dark);
    let (cx, cy) = (TOWN.0 as usize / 2, TOWN.1 as usize / 2);
    t.row(
        "6_vignette.png, vignette -3: the corners darker, the middle as it was; draws cleanly",
        &format!("{said:?}, corner {} before, {} after; middle {} before, {} after", at(&before, 0, 0), at(&dark, 0, 0), at(&before, cx, cy), at(&dark, cx, cy)),
        said.is_empty() && at(&dark, 0, 0) < at(&before, 0, 0) && at(&dark, cx, cy) == at(&before, cx, cy),
    );

    t.finish("D-397_color_grade_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B276_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-276: a measurement, run deliberately with --release --ignored"]
fn b276_color_grade_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B276_CPU").is_ok();
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
        ("Noise, then Color Grade, basic (temperature 30, exposure 0.5, contrast 40, saturation 120)", Some(json!({"temperature": 30, "exposure": 0.5, "contrast": 40, "saturation": 120}))),
        (
            "Noise, then Color Grade, every step but the look (basic, faded film, vibrance, tints, vignette)",
            Some(json!({"temperature": 30, "exposure": 0.5, "contrast": 40, "saturation": 120, "faded_film": 30, "vibrance": 40,
                "shadow_tint_hue": 195, "shadow_tint_amount": 50, "highlight_tint_hue": 35, "highlight_tint_amount": 40, "vignette_amount": -2})),
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
    let out = std::env::var("B276_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-276_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
