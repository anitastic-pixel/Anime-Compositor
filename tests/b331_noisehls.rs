//! B-331: D-451 Noise HLS, after After Effects' Noise HLS ("Noise & Grain" in
//! `docs/effects/EFFECTS.md`): noise laid on each pixel's hue, lightness and saturation.
//!
//! Writes `verification/D-451_noisehls_table.md`.
//!
//! Every expected pixel is `Fixtures/noisehls/expected_noisehls.json`, written by
//! `tools/noisehls_reference.py` before this code existed and printed in document 25 as
//! FX-NOISEHLS-001 to 026. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-451 pictures/`.

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

/// A Noise HLS: its noise and [hue, lightness, saturation, grain size, noise phase].
fn hls(noise: &str, n: [f64; 5]) -> Effect {
    let [hue, lightness, saturation, grain_size, noise_phase] = n;
    Effect::NoiseHls { noise: noise.to_string(), hue, lightness, saturation, grain_size, noise_phase }
}

const ADDED: [f64; 5] = [0.0, 10.0, 0.0, 1.0, 0.0];

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
        "core.noise_hls" => json!({"noise": "uniform", "hue": 0, "lightness": 10, "saturation": 0, "grain_size": 1, "noise_phase": 0}),
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
        let project = reference(|id| json!([fx(type_id, &format!("b331-{id}"), p)]));
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

// --- Noise HLS ------------------------------------------------------------------------------

fn is_noise_hls(e: &Effect) -> bool {
    matches!(e, Effect::NoiseHls { .. })
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
fn b331_noisehls() {
    let mut t = Table::new(
        "noisehls",
        "# D-451: Noise HLS\n\nB-331, after After Effects' Noise HLS: three noises from -1 to 1, one \
         each for hue, lightness and saturation, laid on each shown pixel through D-113's HSL: the \
         hue turns up to half the wheel at Hue 100, the lightness and saturation move up to 1 at \
         100, held in 0 to 1. Uniform is one number a pixel, Squared pushes it towards its ends, \
         Grain is smooth in cells of Grain Size pixels. Noise Phase moves through the noise, a \
         new noise a turn. The covering is kept. Every expected pixel is \
         `Fixtures/noisehls/expected_noisehls.json`, written by `tools/noisehls_reference.py` \
         before this code existed and printed in document 25 as FX-NOISEHLS-001 to 026. \
         Tolerance 2e-5.\n",
    );

    t.heading("FX-NOISEHLS-001 to 026 (document 25)");
    t.fixtures("expected_noisehls.json");

    t.heading("The file");
    let all = files("fx_noisehls", 26);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_noisehls_018.json");
    t.row(
        "fx_noisehls_018.json is saved with its words and numbers as written, and Noise Phase's keys kept",
        &saved.to_string(),
        saved["noise"] == "grain" && saved["grain_size"] == 3 && saved["hue"] == 20 && saved["lightness"] == 25 && saved["saturation"] == 60
            && saved["noise_phase"]["keyframes"][1]["value"] == json!(500),
    );
    why(
        &mut t,
        &[
            ("fx_noisehls_019.json", "Noise HLS's hue runs from 0 to 100, and this is 101."),
            ("fx_noisehls_020.json", "Noise HLS's lightness runs from 0 to 100, and this is -1."),
            ("fx_noisehls_021.json", "Noise HLS's saturation runs from 0 to 100, and this is 150."),
            ("fx_noisehls_022.json", "Noise HLS's grain size runs from 0.5 to 100, and this is 0.25."),
            ("fx_noisehls_023.json", "Noise HLS's noise phase runs from -100000 to 100000, and this is 200000."),
            ("fx_noisehls_024.json", "Noise HLS's noise is one of uniform, squared, grain, and this is \"Uniform\"."),
            ("fx_noisehls_025.json", "Noise HLS's noise is one of uniform, squared, grain, and this is \"grainy\"."),
        ],
    );
    let p = r#""hue": 0, "lightness": 10, "saturation": 0, "grain_size": 1, "noise_phase": 0"#;
    t.shape_refused("fx_noisehls_001.json", "a Noise HLS with no `noise`", &format!("{{{p}}}"));
    t.shape_refused("fx_noisehls_001.json", "a Noise HLS whose noise is a number", &format!("{{{p}, \"noise\": 2}}"));
    t.shape_refused("fx_noisehls_001.json", "a Noise HLS whose hue is a word", &format!("{{\"noise\": \"uniform\", \"lightness\": 10, \"saturation\": 0, \"grain_size\": 1, \"noise_phase\": 0, \"hue\": \"lots\"}}"));

    t.heading("Commands");
    let mut document = t.load("fx_noisehls_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("hue 101", set(hls("uniform", numbers(&[(0, 101.0)])))),
            ("saturation -5", set(hls("uniform", numbers(&[(2, -5.0)])))),
            ("grain size 0.4", set(hls("grain", numbers(&[(3, 0.4)])))),
            ("noise phase 100001", set(hls("uniform", numbers(&[(4, 100001.0)])))),
            ("noise \"Grain\"", set(hls("Grain", ADDED))),
            ("lightness keyed to 120", keys("lightness", &[(0, &[10.0]), (4, &[120.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_noisehls_001.json",
        vec![
            ("Grain, hue 35, lightness 20, saturation 55, grain size 2.5, phase 240", set(hls("grain", [35.0, 20.0, 55.0, 2.5, 240.0]))),
            ("lightness keyed from 0 to 40", keys("lightness", &[(0, &[0.0]), (4, &[40.0])])),
            ("noise phase keyed from 0 to 720", keys("noise_phase", &[(0, &[0.0]), (4, &[720.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_noisehls_001.json", 0),
        ("fx_noisehls_005.json", 0),
        ("fx_noisehls_008.json", 0),
        ("fx_noisehls_011.json", 1),
        ("fx_noisehls_014.json", 0),
        ("fx_noisehls_015.json", 0),
        ("fx_noisehls_018.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // FX-NOISEHLS-013 is all three amounts 0, which changes nothing, and 019 to 026 are refused,
    // so the card is not asked.
    let mut none: Vec<u32> = vec![13];
    none.extend(19..=26);
    card_fixtures(&mut t, &mut gpu, "fx_noisehls", 26, &none, is_noise_hls);
    card_reference(
        &mut t,
        &mut gpu,
        "Noise HLS",
        "core.noise_hls",
        &[
            ("as added (Uniform, lightness 10)", json!({})),
            ("Squared, hue 60, saturation 40, lightness 0", json!({"noise": "squared", "hue": 60, "saturation": 40, "lightness": 0})),
            ("Grain, grain size 4, hue 25, lightness 30, saturation 50, phase 200", json!({"noise": "grain", "grain_size": 4, "hue": 25, "lightness": 30, "saturation": 50, "noise_phase": 200})),
            (
                // Not from 0: with every amount 0 the effect changes nothing and is rightly not sent.
                "Grain, grain size 1.5, lightness keyed 5 to 60 and phase keyed 0 to 3600",
                json!({"noise": "grain", "grain_size": 1.5, "lightness": keyed(&[(0, json!(5)), (239, json!(60))]),
                    "noise_phase": keyed(&[(0, json!(0)), (239, json!(3600))])}),
            ),
        ],
        is_noise_hls,
    );

    street(
        &mut t,
        "D-451",
        "core.noise_hls",
        &[
            ("as_added", json!({}), 0, "as added: the street flecked lighter and darker pixel by pixel, a lightness tenth moving a channel by no more than a fifth", |a, b| flecked(a, b, 52)),
            (
                "hue_60",
                json!({"hue": 60, "lightness": 0}),
                0,
                "Hue 60, Lightness 0: the colours flecked round the wheel, each pixel keeping its lightness",
                |a, b| flecked(a, b, 255) && a.chunks_exact(4).zip(b.chunks_exact(4)).all(|(p, q)| lightness(p).abs_diff(lightness(q)) <= 2),
            ),
            (
                "grain_4_saturation_60",
                json!({"noise": "grain", "grain_size": 4, "saturation": 60, "lightness": 20}),
                0,
                "Grain, grain size 4, Saturation 60, Lightness 20: soft blotches of stronger and weaker colour",
                |a, b| flecked(a, b, 255),
            ),
            (
                "phase_frame_24",
                json!({"noise": "squared", "lightness": 25, "noise_phase": keyed(&[(0, json!(0)), (47, json!(470))])}),
                24,
                "Squared, Lightness 25, phase keyed 0 to 470 over the shot, at frame 24",
                |a, b| flecked(a, b, 255),
            ),
        ],
    );

    t.finish("D-451_noisehls_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B331_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-331: a measurement, run deliberately with --release --ignored"]
fn b331_noisehls_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B331_CPU").is_ok();
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
        ("Noise, then Noise HLS as added (Uniform, lightness 10)", Some(json!({}))),
        ("Noise, then Noise HLS Squared, hue 40, lightness 20, saturation 40", Some(json!({"noise": "squared", "hue": 40, "lightness": 20, "saturation": 40}))),
        ("Noise, then Noise HLS Grain, grain size 2.5, hue 40, lightness 20, saturation 40", Some(json!({"noise": "grain", "grain_size": 2.5, "hue": 40, "lightness": 20, "saturation": 40}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx("core.noise", &format!("{id}n"), &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some(p) = &e {
                v.push(fx("core.noise_hls", &format!("{id}h"), p));
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
    let out = std::env::var("B331_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-331_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
