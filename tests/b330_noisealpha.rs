//! B-330: D-450 Noise Alpha, after After Effects' Noise Alpha ("Noise & Grain" in
//! `docs/effects/EFFECTS.md`): noise laid on the layer's covering (its alpha).
//!
//! Writes `verification/D-450_noisealpha_table.md`.
//!
//! Every expected pixel is `Fixtures/noisealpha/expected_noisealpha.json`, written by
//! `tools/noisealpha_reference.py` before this code existed and printed in document 25 as
//! FX-NOISEALPHA-001 to 031. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-450 pictures/`.

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

/// A Noise Alpha: [amount, random seed, noise phase, cycle], [noise, original alpha, overflow,
/// cycle noise].
fn alpha(n: [f64; 4], w: [&str; 4]) -> Effect {
    let [amount, random_seed, noise_phase, cycle] = n;
    let [noise, original_alpha, overflow, cycle_noise] = w.map(str::to_string);
    Effect::NoiseAlpha { noise, amount, original_alpha, overflow, random_seed, noise_phase, cycle_noise, cycle }
}

const ADDED: [f64; 4] = [20.0, 0.0, 0.0, 1.0];
const WORDS: [&str; 4] = ["uniform_random", "clamp", "clip", "off"];

fn numbers(change: &[(usize, f64)]) -> [f64; 4] {
    let mut n = ADDED;
    for &(i, v) in change {
        n[i] = v;
    }
    n
}

fn words<'a>(change: &[(usize, &'a str)]) -> [&'a str; 4] {
    let mut w = WORDS;
    for &(i, v) in change {
        w[i] = v;
    }
    w
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
        "core.noise_alpha" => json!({"noise": "uniform_random", "amount": 20, "original_alpha": "clamp", "overflow": "clip",
            "random_seed": 0, "noise_phase": 0, "cycle_noise": "off", "cycle": 1}),
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
        let project = reference(|id| json!([fx(type_id, &format!("b330-{id}"), p)]));
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

// --- Noise Alpha ----------------------------------------------------------------------------

fn is_noise_alpha(e: &Effect) -> bool {
    matches!(e, Effect::NoiseAlpha { .. })
}

/// The street made see-through in speckles: more than a third of the pixels changed (on a fully
/// covered street the noise that would add covering is held at full, so about half), only in
/// their covering. `most` is the least covering any pixel keeps, of 255.
fn speckled(a: &[u8], b: &[u8], most: u8) -> bool {
    let (_, changed) = distance(a, b);
    let colour_kept = a.chunks_exact(4).zip(b.chunks_exact(4)).all(|(p, q)| p[3] < 8 || (0..3).all(|c| p[c].abs_diff(q[c]) <= 1));
    let lowest = a.chunks_exact(4).map(|p| p[3]).min().unwrap_or(255);
    changed > a.len() / 4 / 3 && colour_kept && lowest >= most
}

#[test]
fn b330_noisealpha() {
    let mut t = Table::new(
        "noisealpha",
        "# D-450: Noise Alpha\n\nB-330, after After Effects' Noise Alpha: a noise of one number a pixel, \
         from -1 to 1, Squared pushing it towards its ends, times Amount, laid on the layer's \
         covering (alpha) where Original Alpha says (Clamp: only fully covered pixels; Add: \
         everywhere; Scale: in proportion to the covering; Edges: only the partly covered ones), \
         a covering pushed out of 0 to 1 held, reflected or wrapped by Overflow. The Random kinds \
         take Random Seed and stay still; the Animation kinds move through the noise a whole field \
         a turn of Noise Phase, coming back every Cycle turns when Cycle Noise is on. The colour \
         is kept; a pixel that had no covering gains black. Every expected pixel is \
         `Fixtures/noisealpha/expected_noisealpha.json`, written by `tools/noisealpha_reference.py` \
         before this code existed and printed in document 25 as FX-NOISEALPHA-001 to 031. \
         Tolerance 2e-5.\n",
    );

    t.heading("FX-NOISEALPHA-001 to 031 (document 25)");
    t.fixtures("expected_noisealpha.json");

    t.heading("The file");
    let all = files("fx_noisealpha", 31);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_noisealpha_022.json");
    t.row(
        "fx_noisealpha_022.json is saved with its words and numbers as written, and Noise Phase's keys kept",
        &saved.to_string(),
        saved["noise"] == "squared_animation" && saved["amount"] == 60 && saved["original_alpha"] == "scale" && saved["overflow"] == "wrap_back"
            && saved["cycle_noise"] == "on" && saved["cycle"] == 3 && saved["noise_phase"]["keyframes"][1]["value"] == json!(400),
    );
    why(
        &mut t,
        &[
            ("fx_noisealpha_023.json", "Noise Alpha's amount runs from 0 to 100, and this is 101."),
            ("fx_noisealpha_024.json", "Noise Alpha's random seed runs from 0 to 100000, and this is 100001."),
            ("fx_noisealpha_025.json", "Noise Alpha's noise phase runs from -100000 to 100000, and this is 200000."),
            ("fx_noisealpha_026.json", "Noise Alpha's cycle runs from 1 to 1000, and this is 0.5."),
            (
                "fx_noisealpha_027.json",
                "Noise Alpha's noise is one of uniform_random, squared_random, uniform_animation, squared_animation, and this is \"uniform\".",
            ),
            ("fx_noisealpha_028.json", "Noise Alpha's original alpha is one of clamp, add, scale, edges, and this is \"Add\"."),
            ("fx_noisealpha_029.json", "Noise Alpha's overflow is one of clip, wrap_back, wrap, and this is \"wrapback\"."),
            ("fx_noisealpha_030.json", "Noise Alpha's cycle noise is \"off\" or \"on\", and this is \"yes\"."),
        ],
    );
    let p = r#""amount": 20, "original_alpha": "clamp", "overflow": "clip", "random_seed": 0, "noise_phase": 0, "cycle": 1"#;
    t.shape_refused("fx_noisealpha_001.json", "a Noise Alpha with no `noise`", &format!("{{{p}, \"cycle_noise\": \"off\"}}"));
    t.shape_refused("fx_noisealpha_001.json", "a Noise Alpha whose cycle noise is false", &format!("{{{p}, \"noise\": \"uniform_random\", \"cycle_noise\": false}}"));
    t.shape_refused("fx_noisealpha_001.json", "a Noise Alpha whose amount is a word", &format!("{{{p}, \"noise\": \"uniform_random\", \"cycle_noise\": \"off\", \"amount\": \"lots\"}}"));

    t.heading("Commands");
    let mut document = t.load("fx_noisealpha_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("amount 101", set(alpha(numbers(&[(0, 101.0)]), WORDS))),
            ("random seed -1", set(alpha(numbers(&[(1, -1.0)]), WORDS))),
            ("cycle 1001", set(alpha(numbers(&[(3, 1001.0)]), WORDS))),
            ("noise \"squared\"", set(alpha(ADDED, words(&[(0, "squared")])))),
            ("original alpha \"Edges\"", set(alpha(ADDED, words(&[(1, "Edges")])))),
            ("overflow \"wrap back\"", set(alpha(ADDED, words(&[(2, "wrap back")])))),
            ("cycle noise \"On\"", set(alpha(ADDED, words(&[(3, "On")])))),
            ("amount keyed to 150", keys("amount", &[(0, &[20.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_noisealpha_001.json",
        vec![
            (
                "amount 45, seed 9, phase 120, cycle 3, Squared Animation, Edges, Wrap Back, cycle noise on",
                set(alpha([45.0, 9.0, 120.0, 3.0], ["squared_animation", "edges", "wrap_back", "on"])),
            ),
            ("amount keyed from 0 to 40", keys("amount", &[(0, &[0.0]), (4, &[40.0])])),
            ("noise phase keyed from 0 to 720", keys("noise_phase", &[(0, &[0.0]), (4, &[720.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_noisealpha_001.json", 0),
        ("fx_noisealpha_002.json", 0),
        ("fx_noisealpha_006.json", 0),
        ("fx_noisealpha_011.json", 2),
        ("fx_noisealpha_019.json", 0),
        ("fx_noisealpha_020.json", 0),
        ("fx_noisealpha_022.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // FX-NOISEALPHA-018 is Amount 0, which changes nothing, and 023 to 031 are refused, so the
    // card is not asked.
    let mut none: Vec<u32> = vec![18];
    none.extend(23..=31);
    card_fixtures(&mut t, &mut gpu, "fx_noisealpha", 31, &none, is_noise_alpha);
    card_reference(
        &mut t,
        &mut gpu,
        "Noise Alpha",
        "core.noise_alpha",
        &[
            ("as added (Uniform Random, amount 20, Clamp, Clip)", json!({})),
            (
                "Squared Random, amount 60, Add, Wrap Back, seed 42",
                json!({"noise": "squared_random", "amount": 60, "original_alpha": "add", "overflow": "wrap_back", "random_seed": 42}),
            ),
            ("Uniform Animation, amount 80, Scale, Wrap, phase 200", json!({"noise": "uniform_animation", "amount": 80, "original_alpha": "scale", "overflow": "wrap", "noise_phase": 200})),
            (
                // Not from 0: at Amount 0 the effect changes nothing and is rightly not sent.
                "Squared Animation, Edges, amount keyed 5 to 100 and phase keyed 0 to 3600, cycle noise on, cycle 4",
                json!({"noise": "squared_animation", "original_alpha": "edges", "amount": keyed(&[(0, json!(5)), (239, json!(100))]),
                    "noise_phase": keyed(&[(0, json!(0)), (239, json!(3600))]), "cycle_noise": "on", "cycle": 4}),
            ),
        ],
        is_noise_alpha,
    );

    street(
        &mut t,
        "D-450",
        "core.noise_alpha",
        &[
            ("as_added", json!({}), 0, "as added: the street speckled see-through, keeping at least four fifths of its covering", |a, b| speckled(a, b, 203)),
            (
                "squared_60",
                json!({"noise": "squared_random", "amount": 60}),
                0,
                "Squared Random, amount 60: deeper, more contrasting holes, keeping at least two fifths of its covering",
                |a, b| speckled(a, b, 101),
            ),
            (
                "wrap_100",
                json!({"amount": 100, "overflow": "wrap"}),
                0,
                "amount 100, Wrap: a covering pushed past full comes round from nothing, so some pixels all but vanish",
                |a, b| speckled(a, b, 0) && a.chunks_exact(4).any(|p| p[3] < 26),
            ),
            (
                "animation_frame_24",
                json!({"noise": "uniform_animation", "amount": 50, "noise_phase": keyed(&[(0, json!(0)), (47, json!(470))])}),
                24,
                "Uniform Animation, amount 50, phase keyed 0 to 470 over the shot, at frame 24",
                |a, b| speckled(a, b, 127),
            ),
        ],
    );

    t.finish("D-450_noisealpha_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B330_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-330: a measurement, run deliberately with --release --ignored"]
fn b330_noisealpha_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B330_CPU").is_ok();
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
        ("Noise, then Noise Alpha as added (Uniform Random, Clamp)", Some(json!({}))),
        ("Noise, then Noise Alpha Squared Random, Add, Wrap Back", Some(json!({"noise": "squared_random", "original_alpha": "add", "overflow": "wrap_back"}))),
        ("Noise, then Noise Alpha Uniform Animation, phase 200, Scale", Some(json!({"noise": "uniform_animation", "noise_phase": 200, "original_alpha": "scale"}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx("core.noise", &format!("{id}n"), &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some(p) = &e {
                v.push(fx("core.noise_alpha", &format!("{id}a"), p));
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
    let out = std::env::var("B330_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-330_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
