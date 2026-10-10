//! B-284: D-405 Magnify, after After Effects' Magnify ("Distort" in `docs/effects/EFFECTS.md`):
//! a round or square part of the layer enlarged about its centre and laid back over the layer by
//! a blending mode, the layer grown to hold it when Resize Layer is on.
//!
//! Writes `verification/D-405_magnify_table.md`.
//!
//! Every expected pixel is `Fixtures/magnify/expected_magnify.json`, written by
//! `tools/magnify_reference.py` before this code existed and printed in document 25 as
//! FX-MAGNIFY-001 to 034. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-405 pictures/`.

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

fn magnify(magnification: f64, size: f64, feather: f64, opacity: f64, words: [&str; 5]) -> Effect {
    let [shape, link, scaling, blending_mode, resize_layer] = words.map(str::to_string);
    Effect::Magnify { shape, center: [50.0, 50.0], magnification, link, size, feather, opacity, scaling, blending_mode, resize_layer }
}

const PLAIN: [&str; 5] = ["circle", "none", "standard", "normal", "off"];

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
        "core.magnify" => json!({"shape": "circle", "center": [50, 50], "magnification": 200, "link": "none", "size": 100, "feather": 0, "opacity": 100, "scaling": "standard", "blending_mode": "normal", "resize_layer": "off"}),
        _ => json!({}),
    };
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` change nothing or are left out with a warning, so the card is not asked.
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
        let project = reference(|id| json!([fx(type_id, &format!("b284-{id}"), p)]));
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

/// The street drawn plain and with each of `shots` of `type_id`, written as numbered pictures
/// into `verification/{d} pictures/`; a row each that it draws cleanly and changes what it says
/// it changes.
fn street(t: &mut Table, d: &str, type_id: &str, shots: &[(&str, J, &str, fn(&[u8], &[u8]) -> bool)]) {
    t.heading(&format!("Pictures: the street, in `verification/{d} pictures/`"));
    let dir = repo(&format!("verification/{d} pictures"));
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (before, said) = picture(&dir, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    for (i, (name, p, what, check)) in shots.iter().enumerate() {
        let file = format!("{}_{name}.png", i + 2);
        let (bytes, said) = picture(&dir, json!([fx(type_id, "fx-0-0", p)]));
        write(&file, &bytes);
        t.row(
            &format!("{file}, {what}; draws cleanly"),
            &format!("{said:?}, {} pixels changed", distance(&bytes, &before).1),
            said.is_empty() && check(&bytes, &before),
        );
    }
}

fn changed(a: &[u8], b: &[u8]) -> bool {
    distance(a, b).1 > 0
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

// --- Magnify --------------------------------------------------------------------------------

fn is_magnify(e: &Effect) -> bool {
    matches!(e, Effect::Magnify { .. })
}

/// Changed, and some of it left clear.
fn gaps(a: &[u8], b: &[u8]) -> bool {
    changed(a, b) && a.chunks_exact(4).any(|p| p[3] == 0)
}

#[test]
fn b284_magnify() {
    let mut t = Table::new(
        "magnify",
        "# D-405: Magnify\n\nB-284, after After Effects' Magnify: a circle or square of radius Size \
         round Center, the layer read in it at the centre plus the offset over the magnification \
         (standard the pixel holding the place, soft document 21's bilinear sample, scatter the \
         place nudged by Noise's hash), faded over Feather inside its edge and by Opacity, then laid \
         over the layer by the blending mode (document 21's layer blend), or alone for None. Resize \
         Layer grows the layer to hold the area unless Size is linked. Every expected pixel is \
         `Fixtures/magnify/expected_magnify.json`, written by `tools/magnify_reference.py` before \
         this code existed and printed in document 25 as FX-MAGNIFY-001 to 034. Tolerance 2e-5.\n",
    );

    t.heading("FX-MAGNIFY-001 to 034 (document 25)");
    t.fixtures("expected_magnify.json");

    t.heading("The file");
    let all = files("fx_magnify", 34);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_magnify_017.json");
    t.row(
        "fx_magnify_017.json is saved with its words and numbers",
        &saved.to_string(),
        saved["resize_layer"] == "on" && saved["center"] == json!([90, 50]) && saved["size"] == 4 && saved["shape"] == "circle"
            && saved["blending_mode"] == "normal" && saved["magnification"] == 200,
    );
    why(
        &mut t,
        &[
            ("fx_magnify_024.json", "Magnify's magnification runs from 100 to 1000, and this is 99."),
            ("fx_magnify_025.json", "Magnify's magnification runs from 100 to 1000, and this is 1001."),
            ("fx_magnify_026.json", "Magnify's size runs from 0 to 1000, and this is -1."),
            ("fx_magnify_027.json", "Magnify's feather runs from 0 to 1000, and this is 1001."),
            ("fx_magnify_028.json", "Magnify's opacity runs from 0 to 100, and this is 101."),
            ("fx_magnify_029.json", "Magnify's center runs from -1000 to 1000, and this is 1001."),
            ("fx_magnify_030.json", "Magnify's shape is \"circle\" or \"square\", and this is \"oval\"."),
            ("fx_magnify_031.json", "Magnify's link is \"none\", \"size\" or \"size_feather\", and this is \"feather\"."),
            ("fx_magnify_032.json", "Magnify's scaling is \"standard\", \"soft\" or \"scatter\", and this is \"bicubic\"."),
            (
                "fx_magnify_033.json",
                "Magnify's blending mode is \"none\", \"normal\", \"add\", \"multiply\", \"screen\", \"overlay\" or \"soft_light\", and this is \"difference\".",
            ),
            ("fx_magnify_034.json", "Magnify's resize layer is \"off\" or \"on\", and this is \"yes\"."),
        ],
    );
    t.shape_refused("fx_magnify_001.json", "a Magnify with no `scaling`", r#"{"shape": "circle", "center": [50, 50], "magnification": 200, "link": "none", "size": 100, "feather": 0, "opacity": 100, "blending_mode": "normal", "resize_layer": "off"}"#);
    t.shape_refused("fx_magnify_001.json", "a Magnify with a magnification in words", r#"{"shape": "circle", "center": [50, 50], "magnification": "double", "link": "none", "size": 100, "feather": 0, "opacity": 100, "scaling": "standard", "blending_mode": "normal", "resize_layer": "off"}"#);

    t.heading("Commands");
    let mut document = t.load("fx_magnify_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("magnification 99", set(magnify(99.0, 100.0, 0.0, 100.0, PLAIN))),
            ("shape \"oval\"", set(magnify(200.0, 100.0, 0.0, 100.0, ["oval", "none", "standard", "normal", "off"]))),
            ("blending mode \"difference\"", set(magnify(200.0, 100.0, 0.0, 100.0, ["circle", "none", "standard", "difference", "off"]))),
            ("size keyed to -1", keys("size", &[(0, &[4.0]), (4, &[-1.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_magnify_001.json",
        vec![
            ("a square, size 3, feather 1, opacity 60, soft, Screen, resized", set(magnify(300.0, 3.0, 1.0, 60.0, ["square", "none", "soft", "screen", "on"]))),
            ("magnification keyed from 100 to 400", keys("magnification", &[(0, &[100.0]), (4, &[400.0])])),
            ("centre keyed from 25, 50 to 75, 50", keys("center", &[(0, &[25.0, 50.0]), (4, &[75.0, 50.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_magnify_001.json", 0), ("fx_magnify_005.json", 0), ("fx_magnify_009.json", 0), ("fx_magnify_010.json", 0), ("fx_magnify_017.json", 0), ("fx_magnify_020.json", 2)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_magnify", 34, &[24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34], is_magnify);
    card_reference(
        &mut t,
        &mut gpu,
        "Magnify",
        "core.magnify",
        &[
            ("as it starts (a circle of radius 100, magnification 200, Normal)", json!({})),
            ("a square of radius 200, feather 30, magnification 300, soft, Overlay", json!({"shape": "square", "size": 200, "feather": 30, "magnification": 300, "scaling": "soft", "blending_mode": "overlay"})),
            ("radius 300 round 30, 40, scatter, Soft Light, opacity 70, Resize Layer on", json!({"size": 300, "center": [30, 40], "scaling": "scatter", "blending_mode": "soft_light", "opacity": 70, "resize_layer": "on"})),
            ("None, size and feather linked (100 and 10 at 250 per cent)", json!({"blending_mode": "none", "link": "size_feather", "size": 100, "feather": 10, "magnification": 250})),
            ("Add, radius 250", json!({"blending_mode": "add", "size": 250})),
            ("Multiply, radius 250, soft", json!({"blending_mode": "multiply", "size": 250, "scaling": "soft"})),
            ("Screen, radius 250, scatter at 600 per cent", json!({"blending_mode": "screen", "size": 250, "scaling": "scatter", "magnification": 600})),
        ],
        is_magnify,
    );

    street(
        &mut t,
        "D-405",
        "core.magnify",
        &[
            ("as_added", json!({}), "as it starts: a circle of radius 100 in the middle, the street in it twice as big, in 2 by 2 blocks", changed),
            ("square_300", json!({"shape": "square", "size": 60, "magnification": 300}), "a square 120 across, three times as big", changed),
            ("soft_feather", json!({"size": 80, "feather": 30, "scaling": "soft"}), "soft scaling, feather 30: smooth, the edge fading into the street", changed),
            ("scatter", json!({"size": 80, "magnification": 400, "scaling": "scatter"}), "scatter at 400 per cent: the blocks' edges broken up", changed),
            ("none", json!({"size": 80, "blending_mode": "none"}), "blending mode None: the lens alone, clear round it", gaps),
            ("screen", json!({"size": 80, "blending_mode": "screen"}), "Screen: the enlarged street lightening the street under it", changed),
        ],
    );

    t.finish("D-405_magnify_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B284_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-284: a measurement, run deliberately with --release --ignored"]
fn b284_magnify_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B284_CPU").is_ok();
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
    let shots: [(&str, Option<(&str, J)>); 4] = [
        ("Noise alone", None),
        ("Noise, then Magnify as added (radius 100)", Some(("core.magnify", json!({})))),
        ("Noise, then Magnify, radius 500, soft, feather 50", Some(("core.magnify", json!({"size": 500, "scaling": "soft", "feather": 50})))),
        ("Noise, then Magnify, radius 500, scatter, Overlay", Some(("core.magnify", json!({"size": 500, "scaling": "scatter", "blending_mode": "overlay"})))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx("core.noise", &format!("{id}n"), &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some((type_id, p)) = &e {
                v.push(fx(type_id, &format!("{id}c"), p));
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
    let out = std::env::var("B284_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-284_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
