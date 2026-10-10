//! B-277: D-398, Color Grade's second unit, after Lumetri Color under our own name: RGB curves,
//! hue versus saturation, the three colour wheels and HSL Secondary.
//!
//! Writes `verification/D-398_color_grade_2_table.md` and draws pictures into
//! `verification/D-398 pictures/`.
//!
//! Every expected pixel is `Fixtures/color_grade_2/expected_color_grade_2.json`, written by
//! `tools/color_grade_2_reference.py` before this code existed and printed in document 25 as
//! FX-GRADE2-001 to 040. Nothing here is a snapshot of a run.

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

/// The Color Grades the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::ColorGrade { .. })))
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
    let straight = json!([[0, 0], [255, 255]]);
    let mut all = json!({"look": "", "curve_master": straight, "curve_red": straight, "curve_green": straight, "curve_blue": straight,
        "hue_saturation": [], "key_invert": "off", "key_view": "off"});
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
    for n in 1..=40 {
        let file = format!("fx_grade2_{n:03}.json");
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
/// and Draft, on the card against the processor; 3 on the card each frame.
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J)]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p) in settings {
        let project = reference(|id| json!([fx(&format!("b277-{id}"), p)]));
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
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

/// `effects` on the street, drawn, straight 8-bit, with what it warned of.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
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

/// The covered pixels that are grey, their three channels equal.
fn greys(p: &[u8]) -> usize {
    p.chunks_exact(4).filter(|q| q[3] > 0 && q[0] == q[1] && q[1] == q[2]).count()
}

/// The covered pixels.
fn covered(p: &[u8]) -> usize {
    p.chunks_exact(4).filter(|q| q[3] > 0).count()
}

/// The spread of the covered pixels' brightness: the brightest less the darkest, three channels
/// summed.
fn spread(p: &[u8]) -> u32 {
    let b: Vec<u32> = p.chunks_exact(4).filter(|q| q[3] > 0).map(|q| q[..3].iter().map(|&v| v as u32).sum()).collect();
    let mean = b.iter().sum::<u32>() as f64 / b.len().max(1) as f64;
    (b.iter().map(|&v| (v as f64 - mean).abs()).sum::<f64>() / b.len().max(1) as f64) as u32
}

#[test]
fn b277_color_grade_2() {
    let mut t = Table::new(
        "color_grade_2",
        "# D-398: Color Grade, the second unit\n\nB-277, after Lumetri Color under our own name: \
         RGB curves, hue versus saturation, the three colour wheels and HSL Secondary, after \
         D-397's sections and before the vignette. Every expected pixel is \
         `Fixtures/color_grade_2/expected_color_grade_2.json`, written by \
         `tools/color_grade_2_reference.py` before this code existed and printed in document 25 \
         as FX-GRADE2-001 to 040.\n",
    );

    t.heading("FX-GRADE2-001 to 040 (document 25)");
    t.fixtures_numbered("expected_color_grade_2.json", 1..=40);

    t.heading("The file");
    let files: Vec<String> = (1..=40).map(|n| format!("fx_grade2_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_grade2_022.json");
    t.row(
        "fx_grade2_022.json is saved with its 54 settings: the look, 46 numbers, four curves, the hue curve and two words",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 54 && effect_table::same_json(&saved["curve_master"], &json!([[0.0, 0.0], [64.0, 40.0], [192.0, 215.0], [255.0, 255.0]])) && saved["hue_saturation"].as_array().unwrap().len() == 3,
    );
    let old = t.saved_parameters("fx_grade2_027.json");
    t.row(
        "fx_grade2_027.json, a D-397 file without the new settings, is saved as before: none of them written while each is where it starts (D-121's rule)",
        &old.to_string(),
        old.as_object().unwrap().len() == 23
            && ["curve_red", "hue_saturation", "key_hue_range", "secondary_saturation", "key_view"].iter().all(|k| old.get(*k).is_none()),
    );
    for (file, want) in [
        ("fx_grade2_028.json", "key hue range"),
        ("fx_grade2_029.json", "key softness"),
        ("fx_grade2_030.json", "secondary saturation"),
        ("fx_grade2_031.json", "shadow lightness"),
        ("fx_grade2_032.json", "Color Grade's master curve"),
        ("fx_grade2_033.json", "Color Grade's red curve"),
        ("fx_grade2_034.json", "Color Grade's green curve"),
        ("fx_grade2_035.json", "hue versus saturation"),
        ("fx_grade2_036.json", "hue versus saturation"),
        ("fx_grade2_037.json", "hue versus saturation"),
        ("fx_grade2_038.json", "hue versus saturation"),
        ("fx_grade2_039.json", "key invert"),
        ("fx_grade2_040.json", "key view"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    let mut word = fx("x", &json!({}))["parameters"].clone();
    word["key_invert"] = J::from(1);
    t.shape_refused("fx_grade2_001.json", "a Color Grade whose key invert is a number", &word.to_string());
    let mut flat = fx("x", &json!({}))["parameters"].clone();
    flat["curve_red"] = json!([0, 255]);
    t.shape_refused("fx_grade2_001.json", "a Color Grade whose red curve is numbers, not points", &flat.to_string());

    t.heading("Commands");
    let mut document = t.load("fx_grade2_001.json").document;
    let wide = changed(&t, "fx_grade2_001.json", |e| {
        if let Effect::ColorGrade { values, .. } = e {
            values[COLOR_GRADE_SETTINGS.iter().position(|s| s.0 == "key_hue_range").unwrap()] = 181.0;
        }
    });
    let yes = changed(&t, "fx_grade2_001.json", |e| {
        if let Effect::ColorGrade { key_invert, .. } = e {
            *key_invert = "yes".into();
        }
    });
    let backwards = changed(&t, "fx_grade2_001.json", |e| {
        if let Effect::ColorGrade { hue_saturation, .. } = e {
            *hue_saturation = vec![vec![200.0, 50.0], vec![100.0, 150.0]];
        }
    });
    t.refused(
        &mut document,
        vec![
            ("key hue range 181", set(wide)),
            ("key invert \"yes\"", set(yes)),
            ("hue versus saturation with its hues falling", set(backwards)),
            ("shadow lightness keyed to 101", keys("shadow_lightness", &[(0, &[0.0]), (4, &[101.0])])),
        ],
    );
    let graded = changed(&t, "fx_grade2_001.json", |e| {
        if let Effect::ColorGrade { curves, hue_saturation, key_view, .. } = e {
            curves[1] = vec![vec![0.0, 0.0], vec![128.0, 170.0], vec![255.0, 255.0]];
            *hue_saturation = vec![vec![0.0, 50.0], vec![180.0, 150.0]];
            *key_view = "mask".into();
        }
    });
    t.taken(
        &mut document,
        "fx_grade2_001.json",
        vec![
            ("a red curve, a hue curve and the mask view", set(graded)),
            ("key hue keyed from 0 to 120", keys("key_hue", &[(0, &[0.0]), (4, &[120.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_grade2_002.json", 0),
        ("fx_grade2_006.json", 0),
        ("fx_grade2_009.json", 0),
        ("fx_grade2_013.json", 0),
        ("fx_grade2_017.json", 0),
        ("fx_grade2_022.json", 0),
        ("fx_grade2_024.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // As added (001), every hue point at 100 (007) and refused (028 to 040) leave nothing for
    // the card.
    let none: Vec<u32> = [1, 7].into_iter().chain(28..=40).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            (
                "curves and hue: a master S-curve, the red lifted, the greens stronger",
                json!({"curve_master": [[0, 0], [64, 40], [192, 215], [255, 255]], "curve_red": [[0, 0], [128, 160], [255, 255]],
                    "hue_saturation": [[0, 100], [120, 160], [240, 80]]}),
            ),
            (
                "wheels: teal shadows, warm midtones lifted, cool highlights lowered",
                json!({"shadow_wheel_hue": 195, "shadow_wheel_amount": 60, "midtone_wheel_hue": 30, "midtone_wheel_amount": 40, "midtone_lightness": 15,
                    "highlight_wheel_hue": 220, "highlight_wheel_amount": 40, "highlight_lightness": -20}),
            ),
            (
                "HSL secondary: the warm tones keyed with soft edges, cooled and dulled",
                json!({"key_hue": 25, "key_hue_range": 15, "key_hue_softness": 25, "key_saturation_low": 20, "key_softness": 15,
                    "secondary_saturation": 60, "secondary_wheel_hue": 200, "secondary_wheel_amount": 40, "secondary_contrast": 20}),
            ),
        ],
    );

    t.heading("Pictures: in `verification/D-398 pictures/`");
    let dir = repo("verification/D-398 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let one = |p: J| json!([fx("fx-0-0", &p)]);
    let (before, said) = picture(&dir, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (same, said) = picture(&dir, one(json!({})));
    t.row(
        "Color Grade as added, the new settings written, changes nothing: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );
    let (s, said) = picture(&dir, one(json!({"curve_master": [[0, 0], [64, 30], [192, 225], [255, 255]]})));
    write("2_s_curve.png", &s);
    t.row(
        "2_s_curve.png, a master S-curve: the brightness spread wider; draws cleanly",
        &format!("{said:?}, mean distance from the mean brightness {} before, {} after", spread(&before), spread(&s)),
        said.is_empty() && spread(&s) > spread(&before),
    );
    let (hue, said) = picture(&dir, one(json!({"hue_saturation": [[0, 100], [120, 200], [200, 0], [300, 100]]})));
    write("3_hue_curve.png", &hue);
    t.row(
        "3_hue_curve.png, hue versus saturation: greens stronger, the blue sky grey; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&hue, &before).1),
        said.is_empty() && hue != before,
    );
    let (wheels, said) = picture(&dir, one(json!({"shadow_wheel_hue": 195, "shadow_wheel_amount": 60, "highlight_wheel_hue": 35, "highlight_wheel_amount": 50})));
    write("4_wheels.png", &wheels);
    t.row(
        "4_wheels.png, teal shadow wheel and orange highlight wheel: the street changed; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&wheels, &before).1),
        said.is_empty() && wheels != before,
    );
    let pop = json!({"key_hue": 0, "key_hue_range": 20, "key_hue_softness": 15, "key_invert": "on", "secondary_saturation": 0});
    let (popped, said) = picture(&dir, one(pop.clone()));
    write("5_colour_pop.png", &popped);
    t.row(
        "5_colour_pop.png, the reds keyed and inverted, secondary saturation 0: everything but the reds grey; draws cleanly",
        &format!("{said:?}, grey pixels {} before, {} after, of {}", greys(&before), greys(&popped), covered(&popped)),
        said.is_empty() && greys(&popped) > greys(&before),
    );
    let mut shown = pop;
    shown["key_view"] = J::from("mask");
    let (mask, said) = picture(&dir, one(shown));
    write("6_key_mask.png", &mask);
    t.row(
        "6_key_mask.png, the same key shown as the mask: every pixel grey, white where the correction applies; draws cleanly",
        &format!("{said:?}, grey pixels {} of {}", greys(&mask), covered(&mask)),
        said.is_empty() && greys(&mask) == covered(&mask),
    );

    t.finish("D-398_color_grade_2_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way, as B-276's: the reference shot with a Noise that changes every
/// frame on its first three layers, then the effect, every eighth frame asked for as the viewer
/// asks. With `B277_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-277: a measurement, run deliberately with --release --ignored"]
fn b277_color_grade_2_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B277_CPU").is_ok();
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
        (
            "Noise, then Color Grade, curves, hue curve and wheels",
            Some(json!({"curve_master": [[0, 0], [64, 40], [192, 215], [255, 255]], "hue_saturation": [[0, 100], [120, 160], [240, 80]],
                "shadow_wheel_hue": 195, "shadow_wheel_amount": 60, "highlight_wheel_hue": 35, "highlight_wheel_amount": 40})),
        ),
        (
            "Noise, then Color Grade, an HSL secondary (soft warm key, cooled and dulled)",
            Some(json!({"key_hue": 25, "key_hue_range": 15, "key_hue_softness": 25, "key_saturation_low": 20, "key_softness": 15,
                "secondary_saturation": 60, "secondary_wheel_hue": 200, "secondary_wheel_amount": 40, "secondary_contrast": 20})),
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
    let out = std::env::var("B277_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-277_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
