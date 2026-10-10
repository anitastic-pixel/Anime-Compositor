//! B-332: D-452 Noise HLS Auto, after After Effects' Noise HLS Auto ("Noise & Grain" in
//! `docs/effects/EFFECTS.md`): D-451's Noise HLS that moves by itself, frame by frame.
//!
//! Writes `verification/D-452_noisehlsauto_table.md`.
//!
//! Every expected pixel is `Fixtures/noisehlsauto/expected_noisehlsauto.json`, written by
//! `tools/noisehlsauto_reference.py` before this code existed and printed in document 25 as
//! FX-NOISEHLSAUTO-001 to 024. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-452 pictures/`.

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

/// A Noise HLS Auto: its noise and [hue, lightness, saturation, grain size, animation speed].
fn hls(noise: &str, n: [f64; 5]) -> Effect {
    let [hue, lightness, saturation, grain_size, animation_speed] = n;
    Effect::NoiseHlsAuto { noise: noise.to_string(), hue, lightness, saturation, grain_size, animation_speed, frame: 0 }
}

const ADDED: [f64; 5] = [0.0, 10.0, 0.0, 1.0, 1.0];

fn numbers(change: &[(usize, f64)]) -> [f64; 5] {
    let mut n = ADDED;
    for &(i, v) in change {
        n[i] = v;
    }
    n
}

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

/// The effects `pick` takes that the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality, pick: fn(&Effect) -> bool) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if pick(&f.instance.effect)))
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

/// An effect of `type_id` with the settings `p`; for the ones here, every setting `p` leaves
/// out is the one a new one takes, since the file holds them all.
fn fx(type_id: &str, id: &str, p: &J) -> J {
    let mut all = match type_id {
        "core.noise_hls_auto" => json!({"noise": "uniform", "hue": 0, "lightness": 10, "saturation": 0, "grain_size": 1, "animation_speed": 1}),
        _ => json!({}),
    };
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": all})
}

/// A setting keyed linearly through `points`, each a frame and a value.
fn keyed(points: &[(i32, J)]) -> J {
    let frames: Vec<J> = points.iter().map(|(f, v)| json!({"frame": f, "value": v, "interp": "linear"})).collect();
    json!({"base": points[0].1, "keyframes": frames})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out, so the card is not asked.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, stem: &str, count: u32, none: &[u32], pick: fn(&Effect) -> bool) {
    let comp = Id::new(MAIN);
    for n in 1..=count {
        let file = format!("{stem}_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, pick);
            }
        }
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor.
fn card_reference(t: &mut Table, gpu: &mut Gpu, name: &str, type_id: &str, settings: &[(&str, J)], pick: fn(&Effect) -> bool) {
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
        let project = reference(|id| json!([fx(type_id, &format!("b332-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, {name} {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, pick);
                t.row(
                    &format!("the reference shot, {name} {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

/// `effects` on the street at `frame` of a two-second shot, drawn, straight 8-bit, with what it
/// warned of.
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

type Shot<'a> = (&'a str, J, i32, &'a str, fn(&[u8], &[u8]) -> bool);

/// The street drawn plain and with each of `shots` of `type_id` at its frame, written as numbered
/// pictures into `verification/{d} pictures/`; a row each that it draws cleanly and changes what
/// it says it changes.
fn street(t: &mut Table, d: &str, type_id: &str, shots: &[Shot]) {
    t.heading(&format!("Pictures: the street, in `verification/{d} pictures/`"));
    let dir = repo(&format!("verification/{d} pictures"));
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (before, said) = picture(&dir, json!([]), 0);
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    for (i, (name, p, frame, what, check)) in shots.iter().enumerate() {
        let file = format!("{}_{name}.png", i + 2);
        let (bytes, said) = picture(&dir, json!([fx(type_id, "fx-0-0", p)]), *frame);
        write(&file, &bytes);
        t.row(
            &format!("{file}, frame {frame}, {what}; draws cleanly"),
            &format!("{said:?}, {} pixels changed of {}, the largest by {} of 255", distance(&bytes, &before).1, TOWN.0 * TOWN.1, distance(&bytes, &before).0),
            said.is_empty() && check(&bytes, &before),
        );
    }
}

fn why(t: &mut Table, cases: &[(&str, &str)]) {
    for (file, want) in cases {
        let doc = t.load(file).document;
        let why = doc.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == *want);
    }
}

fn files(stem: &str, count: u32) -> Vec<String> {
    (1..=count).map(|n| format!("{stem}_{n:03}.json")).collect()
}

// --- Noise HLS Auto -------------------------------------------------------------------------

fn is_noise_hls_auto(e: &Effect) -> bool {
    matches!(e, Effect::NoiseHlsAuto { .. })
}

/// The street flecked in colour: more than half the pixels changed, the covering kept, and no
/// pixel moved by more than `most` of 255 in any channel.
fn flecked(a: &[u8], b: &[u8], most: u8) -> bool {
    let (largest, changed) = distance(a, b);
    let covering_kept = a.chunks_exact(4).zip(b.chunks_exact(4)).all(|(p, q)| p[3].abs_diff(q[3]) <= 1);
    changed > a.len() / 4 / 2 && covering_kept && largest <= most
}

/// Each pixel's lightness, (largest + smallest) / 2 of 255, as D-113 reads it.
fn lightness(p: &[u8]) -> i32 {
    (p[0].max(p[1]).max(p[2]) as i32 + p[0].min(p[1]).min(p[2]) as i32) / 2
}

#[test]
fn b332_noisehlsauto() {
    let mut t = Table::new(
        "noisehlsauto",
        "# D-452: Noise HLS Auto\n\nB-332, after After Effects' Noise HLS Auto: D-451's Noise HLS \
         (three noises laid on each shown pixel's hue, lightness and saturation through D-113's \
         HSL; Uniform, Squared, Grain) whose noise moves by itself: its depth is the frame times \
         Noise Animation Speed, so at speed 1 every frame is a whole new noise and at 0 it holds \
         still, as D-443's Add Grain reads its speed. The covering is kept. Every expected pixel \
         is `Fixtures/noisehlsauto/expected_noisehlsauto.json`, written by \
         `tools/noisehlsauto_reference.py` before this code existed and printed in document 25 as \
         FX-NOISEHLSAUTO-001 to 024. Tolerance 2e-5.\n",
    );

    t.heading("FX-NOISEHLSAUTO-001 to 024 (document 25)");
    t.fixtures("expected_noisehlsauto.json");

    t.heading("The file");
    let all = files("fx_noisehlsauto", 24);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_noisehlsauto_013.json");
    t.row(
        "fx_noisehlsauto_013.json is saved with its words and numbers as written, and the speed's keys kept",
        &saved.to_string(),
        saved["noise"] == "uniform" && saved["grain_size"] == 1 && saved["hue"] == 0 && saved["lightness"] == 30 && saved["saturation"] == 0
            && saved["animation_speed"]["keyframes"][1]["value"] == json!(2),
    );
    let saved = t.saved_parameters("fx_noisehlsauto_015.json");
    t.row(
        "fx_noisehlsauto_015.json is saved with its words and numbers as written",
        &saved.to_string(),
        saved["noise"] == "grain" && saved["grain_size"] == 3 && saved["hue"] == 20 && saved["lightness"] == 25 && saved["saturation"] == 60
            && saved["animation_speed"] == 0.75,
    );
    why(
        &mut t,
        &[
            ("fx_noisehlsauto_016.json", "Noise HLS Auto's hue runs from 0 to 100, and this is 101."),
            ("fx_noisehlsauto_017.json", "Noise HLS Auto's lightness runs from 0 to 100, and this is -1."),
            ("fx_noisehlsauto_018.json", "Noise HLS Auto's saturation runs from 0 to 100, and this is 150."),
            ("fx_noisehlsauto_019.json", "Noise HLS Auto's grain size runs from 0.5 to 100, and this is 0.25."),
            ("fx_noisehlsauto_020.json", "Noise HLS Auto's animation speed runs from 0 to 10, and this is 11."),
            ("fx_noisehlsauto_021.json", "Noise HLS Auto's animation speed runs from 0 to 10, and this is -1."),
            ("fx_noisehlsauto_022.json", "Noise HLS Auto's noise is one of uniform, squared, grain, and this is \"Squared\"."),
            ("fx_noisehlsauto_023.json", "Noise HLS Auto's noise is one of uniform, squared, grain, and this is \"film\"."),
        ],
    );
    let p = r#""hue": 0, "lightness": 10, "saturation": 0, "grain_size": 1, "animation_speed": 1"#;
    t.shape_refused("fx_noisehlsauto_001.json", "a Noise HLS Auto with no `noise`", &format!("{{{p}}}"));
    t.shape_refused("fx_noisehlsauto_001.json", "a Noise HLS Auto whose noise is a number", &format!("{{{p}, \"noise\": 2}}"));
    t.shape_refused("fx_noisehlsauto_001.json", "a Noise HLS Auto with no `animation_speed`", "{\"noise\": \"uniform\", \"hue\": 0, \"lightness\": 10, \"saturation\": 0, \"grain_size\": 1}");

    t.heading("Commands");
    let mut document = t.load("fx_noisehlsauto_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("hue 101", set(hls("uniform", numbers(&[(0, 101.0)])))),
            ("saturation -5", set(hls("uniform", numbers(&[(2, -5.0)])))),
            ("grain size 0.4", set(hls("grain", numbers(&[(3, 0.4)])))),
            ("animation speed 10.5", set(hls("uniform", numbers(&[(4, 10.5)])))),
            ("noise \"Grain\"", set(hls("Grain", ADDED))),
            ("lightness keyed to 120", keys("lightness", &[(0, &[10.0]), (4, &[120.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_noisehlsauto_001.json",
        vec![
            ("Grain, hue 35, lightness 20, saturation 55, grain size 2.5, speed 0.4", set(hls("grain", [35.0, 20.0, 55.0, 2.5, 0.4]))),
            ("lightness keyed from 0 to 40", keys("lightness", &[(0, &[0.0]), (4, &[40.0])])),
            ("animation speed keyed from 0 to 2", keys("animation_speed", &[(0, &[0.0]), (4, &[2.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_noisehlsauto_001.json", 1),
        ("fx_noisehlsauto_003.json", 1),
        ("fx_noisehlsauto_007.json", 1),
        ("fx_noisehlsauto_010.json", 2),
        ("fx_noisehlsauto_011.json", 2),
        ("fx_noisehlsauto_013.json", 4),
        ("fx_noisehlsauto_015.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // FX-NOISEHLSAUTO-009 is all three amounts 0, which changes nothing, and 016 to 024 are
    // refused, so the card is not asked.
    let mut none: Vec<u32> = vec![9];
    none.extend(16..=24);
    card_fixtures(&mut t, &mut gpu, "fx_noisehlsauto", 24, &none, is_noise_hls_auto);
    card_reference(
        &mut t,
        &mut gpu,
        "Noise HLS Auto",
        "core.noise_hls_auto",
        &[
            ("as added (Uniform, lightness 10, speed 1)", json!({})),
            ("Squared, hue 60, saturation 40, lightness 0, speed 0.3", json!({"noise": "squared", "hue": 60, "saturation": 40, "lightness": 0, "animation_speed": 0.3})),
            ("Grain, grain size 4, hue 25, lightness 30, saturation 50, speed 0.1", json!({"noise": "grain", "grain_size": 4, "hue": 25, "lightness": 30, "saturation": 50, "animation_speed": 0.1})),
            (
                // Not from 0: with every amount 0 the effect changes nothing and is rightly not sent.
                "Grain, grain size 1.5, lightness keyed 5 to 60 and speed keyed 0 to 4",
                json!({"noise": "grain", "grain_size": 1.5, "lightness": keyed(&[(0, json!(5)), (239, json!(60))]),
                    "animation_speed": keyed(&[(0, json!(0)), (239, json!(4))])}),
            ),
        ],
        is_noise_hls_auto,
    );

    street(
        &mut t,
        "D-452",
        "core.noise_hls_auto",
        &[
            ("as_added", json!({}), 0, "as added: the street flecked lighter and darker pixel by pixel, a lightness tenth moving a channel by no more than a fifth", |a, b| flecked(a, b, 52)),
            ("as_added_frame_1", json!({}), 1, "as added, frame 1: flecked the same way, a pattern of its own", |a, b| flecked(a, b, 52)),
            (
                "hue_60_frame_5",
                json!({"hue": 60, "lightness": 0}),
                5,
                "Hue 60, Lightness 0, frame 5: the colours flecked round the wheel, each pixel keeping its lightness",
                |a, b| flecked(a, b, 255) && a.chunks_exact(4).zip(b.chunks_exact(4)).all(|(p, q)| lightness(p).abs_diff(lightness(q)) <= 2),
            ),
            (
                "grain_4_saturation_60_frame_24",
                json!({"noise": "grain", "grain_size": 4, "saturation": 60, "lightness": 20, "animation_speed": 0.25}),
                24,
                "Grain, grain size 4, Saturation 60, Lightness 20, speed 0.25, frame 24: soft blotches of stronger and weaker colour",
                |a, b| flecked(a, b, 255),
            ),
        ],
    );
    let dir = repo("verification/D-452 pictures");
    let (zero, _) = picture(&dir, json!([fx("core.noise_hls_auto", "fx-0-0", &json!({}))]), 0);
    let (one, _) = picture(&dir, json!([fx("core.noise_hls_auto", "fx-0-0", &json!({}))]), 1);
    let moved = distance(&zero, &one).1;
    t.row(
        "2_as_added.png and 3_as_added_frame_1.png differ: it moves by itself, with nothing keyed",
        &format!("{moved} pixels differ of {}", TOWN.0 * TOWN.1),
        moved > TOWN.0 * TOWN.1 / 2,
    );
    let still = |frame| picture(&dir, json!([fx("core.noise_hls_auto", "fx-0-0", &json!({"animation_speed": 0}))]), frame).0;
    let held = distance(&still(0), &still(24)).1;
    t.row("with speed 0 the street at frame 24 is the street at frame 0: the noise holds still", &format!("{held} pixels differ"), held == 0);

    t.finish("D-452_noisehlsauto_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B332_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-332: a measurement, run deliberately with --release --ignored"]
fn b332_noisehlsauto_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B332_CPU").is_ok();
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
        ("Noise, then Noise HLS Auto Auto as added (Uniform, lightness 10, speed 1)", Some(json!({}))),
        ("Noise, then Noise HLS Auto Squared, hue 40, lightness 20, saturation 40", Some(json!({"noise": "squared", "hue": 40, "lightness": 20, "saturation": 40}))),
        ("Noise, then Noise HLS Auto Grain, grain size 2.5, hue 40, lightness 20, saturation 40", Some(json!({"noise": "grain", "grain_size": 2.5, "hue": 40, "lightness": 20, "saturation": 40}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx("core.noise", &format!("{id}n"), &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some(p) = &e {
                v.push(fx("core.noise_hls_auto", &format!("{id}h"), p));
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
    let out = std::env::var("B332_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-332_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
