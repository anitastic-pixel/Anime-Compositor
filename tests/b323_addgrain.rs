//! B-323: D-443 Add Grain, after After Effects' Add Grain ("Noise & Grain" in
//! `docs/effects/EFFECTS.md`): film grain laid over the layer.
//!
//! Writes `verification/D-443_addgrain_table.md`.
//!
//! Every expected pixel is `Fixtures/addgrain/expected_addgrain.json`, written by
//! `tools/addgrain_reference.py` before this code existed and printed in document 25 as
//! FX-ADDGRAIN-001 to 041. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-443 pictures/`.

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

/// An Add Grain: [intensity, size, softness, aspect ratio, red, green, blue, saturation,
/// shadows, midtones, highlights, midpoint, animation speed, random seed], [monochromatic,
/// blending mode, animate smoothly].
fn grain(n: [f64; 14], w: [&str; 3]) -> Effect {
    let [intensity, size, softness, aspect_ratio, red_intensity, green_intensity, blue_intensity, saturation, shadows, midtones, highlights, midpoint, animation_speed, random_seed] = n;
    let [monochromatic, blending_mode, animate_smoothly] = w.map(str::to_string);
    Effect::AddGrain {
        intensity,
        size,
        softness,
        aspect_ratio,
        red_intensity,
        green_intensity,
        blue_intensity,
        monochromatic,
        saturation,
        blending_mode,
        shadows,
        midtones,
        highlights,
        midpoint,
        animation_speed,
        animate_smoothly,
        random_seed,
        frame: 0,
    }
}

const ADDED: [f64; 14] = [1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5, 1.0, 0.0];
const WORDS: [&str; 3] = ["off", "film", "on"];

fn numbers(change: &[(usize, f64)]) -> [f64; 14] {
    let mut n = ADDED;
    for &(i, v) in change {
        n[i] = v;
    }
    n
}

fn words<'a>(change: &[(usize, &'a str)]) -> [&'a str; 3] {
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
        "core.add_grain" => json!({"intensity": 1, "size": 1, "softness": 0, "aspect_ratio": 1, "red_intensity": 1, "green_intensity": 1,
            "blue_intensity": 1, "monochromatic": "off", "saturation": 1, "blending_mode": "film", "shadows": 1, "midtones": 1, "highlights": 1,
            "midpoint": 0.5, "animation_speed": 1, "animate_smoothly": "on", "random_seed": 0}),
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
        let project = reference(|id| json!([fx(type_id, &format!("b323-{id}"), p)]));
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

// --- Add Grain ------------------------------------------------------------------------------

fn is_grain(e: &Effect) -> bool {
    matches!(e, Effect::AddGrain { .. })
}

/// Grain: most of the street changed, none by more than a third of the range.
fn grainy(a: &[u8], b: &[u8]) -> bool {
    let (largest, changed) = distance(a, b);
    changed > a.len() / 4 / 2 && largest < 85
}

/// Grey grain: as `grainy`, and every pixel's three channels moved by nearly the same amount.
fn grey(a: &[u8], b: &[u8]) -> bool {
    let same = a.chunks_exact(4).zip(b.chunks_exact(4)).all(|(p, q)| {
        let d: Vec<i32> = (0..3).map(|c| p[c] as i32 - q[c] as i32).collect();
        // Through the sRGB curve the same push is not the same number of levels, so the three
        // channels only move the same way.
        d.iter().all(|x| x.signum() == d[0].signum() || *x == 0 || d[0] == 0)
    });
    grainy(a, b) && same
}

#[test]
fn b323_addgrain() {
    let mut t = Table::new(
        "addgrain",
        "# D-443: Add Grain\n\nB-323, after After Effects' Add Grain: a value noise of Size pixels (Aspect \
         Ratio wider), blocky at Softness 0 and smooth at 1, one number for all three channels when \
         Monochromatic, else three pulled towards their mean by Saturation, scaled by each channel's \
         intensity and weighted by the pixel's brightness through Shadows, Midtones and Highlights \
         about Midpoint, laid on by Film, Add or Overlay at a tenth of Intensity a channel, through \
         the sRGB curve. The grain belongs to the drawing, moves with it, and changes Animation Speed \
         times a frame, gliding when Animate Smoothly is on. Every expected pixel is \
         `Fixtures/addgrain/expected_addgrain.json`, written by `tools/addgrain_reference.py` before \
         this code existed and printed in document 25 as FX-ADDGRAIN-001 to 041. Tolerance 2e-5.\n",
    );

    t.heading("FX-ADDGRAIN-001 to 041 (document 25)");
    t.fixtures("expected_addgrain.json");

    t.heading("The file");
    let all = files("fx_addgrain", 41);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_addgrain_028.json");
    t.row(
        "fx_addgrain_028.json is saved with its words and numbers as written, and no frame",
        &saved.to_string(),
        saved["monochromatic"] == "on" && saved["blending_mode"] == "film" && saved["size"] == 3 && saved["softness"] == 0.3
            && saved["aspect_ratio"] == 0.5 && saved["random_seed"] == 11 && saved.get("frame").is_none(),
    );
    let saved = t.saved_parameters("fx_addgrain_022.json");
    t.row("fx_addgrain_022.json is saved with Intensity's keys kept", &saved["intensity"].to_string(), saved["intensity"]["keyframes"][1]["value"] == json!(4));
    why(
        &mut t,
        &[
            ("fx_addgrain_029.json", "Add Grain's intensity runs from 0 to 10, and this is 11."),
            ("fx_addgrain_030.json", "Add Grain's size runs from 0.1 to 100, and this is 0.05."),
            ("fx_addgrain_031.json", "Add Grain's softness runs from 0 to 1, and this is 1.5."),
            ("fx_addgrain_032.json", "Add Grain's aspect ratio runs from 0.25 to 4, and this is 5."),
            ("fx_addgrain_033.json", "Add Grain's green intensity runs from 0 to 10, and this is -1."),
            ("fx_addgrain_034.json", "Add Grain's saturation runs from 0 to 1, and this is 2."),
            ("fx_addgrain_035.json", "Add Grain's midpoint runs from 0.01 to 0.99, and this is 1."),
            ("fx_addgrain_036.json", "Add Grain's animation speed runs from 0 to 10, and this is 11."),
            ("fx_addgrain_037.json", "Add Grain's random seed runs from 0 to 100000, and this is 100001."),
            ("fx_addgrain_038.json", "Add Grain's blending mode is one of film, add, overlay, and this is \"screen\"."),
            ("fx_addgrain_039.json", "Add Grain's monochromatic is \"off\" or \"on\", and this is \"yes\"."),
            ("fx_addgrain_040.json", "Add Grain's animate smoothly is \"on\" or \"off\", and this is \"On\"."),
        ],
    );
    let p = r#""intensity": 1, "size": 1, "softness": 0, "aspect_ratio": 1, "red_intensity": 1, "green_intensity": 1, "blue_intensity": 1, "saturation": 1, "shadows": 1, "midtones": 1, "highlights": 1, "midpoint": 0.5, "animation_speed": 1, "random_seed": 0, "monochromatic": "off""#;
    t.shape_refused("fx_addgrain_001.json", "an Add Grain with no `blending_mode`", &format!("{{{p}, \"animate_smoothly\": \"on\"}}"));
    t.shape_refused("fx_addgrain_001.json", "an Add Grain whose animate smoothly is true", &format!("{{{p}, \"blending_mode\": \"film\", \"animate_smoothly\": true}}"));
    t.shape_refused("fx_addgrain_001.json", "an Add Grain whose size is a word", &format!("{{{p}, \"blending_mode\": \"film\", \"animate_smoothly\": \"on\", \"size\": \"big\"}}"));

    t.heading("Commands");
    let mut document = t.load("fx_addgrain_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("intensity 11", set(grain(numbers(&[(0, 11.0)]), WORDS))),
            ("size 0.05", set(grain(numbers(&[(1, 0.05)]), WORDS))),
            ("midpoint 1", set(grain(numbers(&[(11, 1.0)]), WORDS))),
            ("monochromatic \"yes\"", set(grain(ADDED, words(&[(0, "yes")])))),
            ("blending mode \"screen\"", set(grain(ADDED, words(&[(1, "screen")])))),
            ("animate smoothly \"On\"", set(grain(ADDED, words(&[(2, "On")])))),
            ("shadows keyed to 20", keys("shadows", &[(0, &[1.0]), (4, &[20.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_addgrain_001.json",
        vec![
            (
                "intensity 2.5, size 3, softness 0.4, aspect 1.5, channels 2, 0.5 and 3, saturation 0.25, shadows 2, midtones 0.5, highlights 4, midpoint 0.3, speed 2, seed 9, monochromatic, Overlay, not smooth",
                set(grain([2.5, 3.0, 0.4, 1.5, 2.0, 0.5, 3.0, 0.25, 2.0, 0.5, 4.0, 0.3, 2.0, 9.0], ["on", "overlay", "off"])),
            ),
            ("intensity keyed from 0 to 4", keys("intensity", &[(0, &[0.0]), (4, &[4.0])])),
            ("size keyed from 1 to 5", keys("size", &[(0, &[1.0]), (4, &[5.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_addgrain_001.json", 4),
        ("fx_addgrain_009.json", 0),
        ("fx_addgrain_011.json", 0),
        ("fx_addgrain_024.json", 2),
        ("fx_addgrain_025.json", 2),
        ("fx_addgrain_028.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // FX-ADDGRAIN-017 is Intensity 0, which changes nothing, and 029 to 041 are refused, so the
    // card is not asked.
    let mut none: Vec<u32> = vec![17];
    none.extend(29..=41);
    card_fixtures(&mut t, &mut gpu, "fx_addgrain", 41, &none, is_grain);
    card_reference(
        &mut t,
        &mut gpu,
        "Add Grain",
        "core.add_grain",
        &[
            ("as added (intensity 1, size 1, Film, in colour)", json!({})),
            (
                "size 6, softness 1, aspect 2, monochromatic, Overlay, intensity 3",
                json!({"size": 6, "softness": 1, "aspect_ratio": 2, "monochromatic": "on", "blending_mode": "overlay", "intensity": 3}),
            ),
            (
                "size 2.5, softness 0.4, channels 2, 0.5 and 1.5, saturation 0.3, Add, shadows 3, highlights 0, midpoint 0.3, speed 0.5 not smooth, seed 42",
                json!({"size": 2.5, "softness": 0.4, "red_intensity": 2, "green_intensity": 0.5, "blue_intensity": 1.5, "saturation": 0.3,
                    "blending_mode": "add", "shadows": 3, "highlights": 0, "midpoint": 0.3, "animation_speed": 0.5, "animate_smoothly": "off", "random_seed": 42}),
            ),
            (
                // Not from 0: at Intensity 0 the effect changes nothing and is rightly not sent.
                "intensity keyed 0.5 to 5 and size keyed 1 to 20, speed 0.25",
                json!({"intensity": keyed(&[(0, json!(0.5)), (239, json!(5))]), "size": keyed(&[(0, json!(1)), (239, json!(20))]), "animation_speed": 0.25}),
            ),
        ],
        is_grain,
    );

    street(
        &mut t,
        "D-443",
        "core.add_grain",
        &[
            ("as_added", json!({}), 0, "as added: a fine colour grain over the whole street", grainy),
            ("coarse_mono", json!({"size": 3, "softness": 0.5, "monochromatic": "on", "intensity": 2}), 0, "size 3, softness 0.5, monochromatic, intensity 2: a coarser grey grain", grey),
            (
                "shadows_overlay",
                json!({"blending_mode": "overlay", "intensity": 3, "shadows": 4, "midtones": 0.5, "highlights": 0, "size": 1.5}),
                12,
                "Overlay, intensity 3, shadows 4, midtones 0.5, highlights 0: heavy grain in the dark parts, none in the sky's brightest",
                grainy,
            ),
            ("frame_24", json!({}), 24, "as added at frame 24: a different grain from frame 0's", grainy),
        ],
    );

    t.finish("D-443_addgrain_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B323_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-323: a measurement, run deliberately with --release --ignored"]
fn b323_addgrain_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B323_CPU").is_ok();
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
        ("Noise, then Add Grain as added (size 1, blocky, in colour)", Some(json!({}))),
        ("Noise, then Add Grain size 4, softness 1 (smooth)", Some(json!({"size": 4, "softness": 1}))),
        ("Noise, then Add Grain size 3, softness 0.5 (both), Overlay", Some(json!({"size": 3, "softness": 0.5, "blending_mode": "overlay"}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx("core.noise", &format!("{id}n"), &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some(p) = &e {
                v.push(fx("core.add_grain", &format!("{id}g"), p));
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
    let out = std::env::var("B323_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-323_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
