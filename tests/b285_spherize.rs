//! B-285: D-406 Spherize, after After Effects' Spherize ("Distort" in `docs/effects/EFFECTS.md`):
//! a round part of the layer wrapped onto a sphere, its middle swelling toward the viewer.
//!
//! Writes `verification/D-406_spherize_table.md`.
//!
//! Every expected pixel is `Fixtures/spherize/expected_spherize.json`, written by
//! `tools/spherize_reference.py` before this code existed and printed in document 25 as
//! FX-SPHERIZE-001 to 015. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-406 pictures/`.

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

fn spherize(radius: f64, center: [f64; 2]) -> Effect {
    Effect::Spherize { radius, center }
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
        "core.spherize" => json!({"radius": 100, "center": [50, 50]}),
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
        let project = reference(|id| json!([fx(type_id, &format!("b285-{id}"), p)]));
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

// --- Spherize -------------------------------------------------------------------------------

fn is_spherize(e: &Effect) -> bool {
    matches!(e, Effect::Spherize { .. })
}

#[test]
fn b285_spherize() {
    let mut t = Table::new(
        "spherize",
        "# D-406: Spherize\n\nB-285, after After Effects' Spherize: each pixel within Radius of the \
         Center of Sphere, rho of the way out, reads the layer at the centre plus its offset times \
         2 asin(rho) / (pi rho), by document 21's bilinear sample, a half sphere seen from in front; \
         the pixels outside the sphere are left as they are. Every expected pixel is \
         `Fixtures/spherize/expected_spherize.json`, written by `tools/spherize_reference.py` before \
         this code existed and printed in document 25 as FX-SPHERIZE-001 to 015. Tolerance 2e-5.\n",
    );

    t.heading("FX-SPHERIZE-001 to 015 (document 25)");
    t.fixtures("expected_spherize.json");

    t.heading("The file");
    let all = files("fx_spherize", 15);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_spherize_004.json");
    t.row(
        "fx_spherize_004.json is saved with its radius and centre",
        &saved.to_string(),
        saved["radius"] == 5 && saved["center"] == json!([25, 50]),
    );
    why(
        &mut t,
        &[
            ("fx_spherize_012.json", "Spherize's radius runs from 0 to 2500, and this is -1."),
            ("fx_spherize_013.json", "Spherize's radius runs from 0 to 2500, and this is 2501."),
            ("fx_spherize_014.json", "Spherize's center runs from -1000 to 1000, and this is 1001."),
            ("fx_spherize_015.json", "Spherize's center runs from -1000 to 1000, and this is -1001."),
        ],
    );
    t.shape_refused("fx_spherize_001.json", "a Spherize with no `center`", r#"{"radius": 100}"#);
    t.shape_refused("fx_spherize_001.json", "a Spherize with a radius in words", r#"{"radius": "big", "center": [50, 50]}"#);
    t.shape_refused("fx_spherize_001.json", "a Spherize with one number for its centre", r#"{"radius": 100, "center": [50]}"#);

    t.heading("Commands");
    let mut document = t.load("fx_spherize_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("radius -1", set(spherize(-1.0, [50.0, 50.0]))),
            ("centre 50, 1001", set(spherize(5.0, [50.0, 1001.0]))),
            ("radius keyed to 2501", keys("radius", &[(0, &[4.0]), (4, &[2501.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_spherize_001.json",
        vec![
            ("radius 6 round 30, 40", set(spherize(6.0, [30.0, 40.0]))),
            ("radius keyed from 0 to 8", keys("radius", &[(0, &[0.0]), (4, &[8.0])])),
            ("centre keyed from 25, 50 to 75, 50", keys("center", &[(0, &[25.0, 50.0]), (4, &[75.0, 50.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_spherize_001.json", 0), ("fx_spherize_004.json", 0), ("fx_spherize_005.json", 0), ("fx_spherize_006.json", 0), ("fx_spherize_009.json", 2), ("fx_spherize_010.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_spherize", 15, &[2, 12, 13, 14, 15], is_spherize);
    card_reference(
        &mut t,
        &mut gpu,
        "Spherize",
        "core.spherize",
        &[
            ("as it starts (radius 100 in the middle)", json!({})),
            ("radius 500 round 30, 40", json!({"radius": 500, "center": [30, 40]})),
            ("radius 2500, the whole layer inside", json!({"radius": 2500})),
            ("radius 400 round -10, 50, half off the edge", json!({"radius": 400, "center": [-10, 50]})),
        ],
        is_spherize,
    );

    street(
        &mut t,
        "D-406",
        "core.spherize",
        &[
            ("as_added", json!({}), "as it starts: a circle of radius 100 in the middle swelling toward you", changed),
            ("radius_130", json!({"radius": 130}), "radius 130: the sphere nearly the street's height", changed),
            ("off_centre", json!({"radius": 80, "center": [25, 40]}), "radius 80 round 25, 40, over the left houses", changed),
            ("whole", json!({"radius": 2500}), "radius 2500: the whole street inside the sphere, enlarged about pi / 2 times from the middle", changed),
            ("edge", json!({"radius": 120, "center": [0, 50]}), "radius 120 round 0, 50: half a sphere on the left edge", changed),
        ],
    );

    t.finish("D-406_spherize_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B285_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-285: a measurement, run deliberately with --release --ignored"]
fn b285_spherize_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B285_CPU").is_ok();
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
    let shots: [(&str, Option<(&str, J)>); 3] = [
        ("Noise alone", None),
        ("Noise, then Spherize as added (radius 100)", Some(("core.spherize", json!({})))),
        ("Noise, then Spherize, radius 2500", Some(("core.spherize", json!({"radius": 2500})))),
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
    let out = std::env::var("B285_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-285_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
