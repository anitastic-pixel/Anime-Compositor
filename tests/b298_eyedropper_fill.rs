//! B-298: D-419 Eyedropper Fill, after After Effects' Eyedropper Fill ("Generate" in
//! `docs/effects/EFFECTS.md`): the layer filled with one colour sampled from an area of itself.
//!
//! Writes `verification/D-419_eyedropper_fill_table.md`.
//!
//! Every expected pixel is `Fixtures/eyedropper_fill/expected_eyedropper_fill.json`, written by
//! `tools/eyedropper_fill_reference.py` before this code existed and printed in document 25 as
//! FX-EYEFILL-001 to 032. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-419 pictures/`.

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

fn eye(sample_point: [f64; 2], [sample_radius, blend_with_original]: [f64; 2], [average, maintain]: [&str; 2]) -> Effect {
    Effect::EyedropperFill {
        sample_point,
        sample_radius,
        average_pixel_colors: average.to_string(),
        maintain_original_alpha: maintain.to_string(),
        blend_with_original,
    }
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
        "core.eyedropper_fill" => json!({"sample_point": [50, 50], "sample_radius": 0, "average_pixel_colors": "skip_empty", "maintain_original_alpha": "off", "blend_with_original": 0}),
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
        let project = reference(|id| json!([fx(type_id, &format!("b298-{id}"), p)]));
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
            &format!("{said:?}, {} pixels changed of {}", distance(&bytes, &before).1, TOWN.0 * TOWN.1),
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

// --- Eyedropper Fill ------------------------------------------------------------------------

fn is_eye(e: &Effect) -> bool {
    matches!(e, Effect::EyedropperFill { .. })
}

/// The pixels the picture shows, as one colour: true when every pixel showing is within a level
/// of the first one, so the street is one flat colour.
fn flat(a: &[u8]) -> bool {
    let mut showing = a.chunks_exact(4).filter(|p| p[3] > 0);
    let first = showing.next().unwrap().to_vec();
    showing.all(|p| p.iter().zip(&first).all(|(x, y)| x.abs_diff(*y) <= 1))
}

/// Nearly every pixel changed (the ones already that colour stay), to one flat colour.
fn one_colour(a: &[u8], b: &[u8]) -> bool {
    distance(a, b).1 as f64 / (a.len() / 4) as f64 > 0.9 && flat(a)
}

/// Nearly every pixel changed, alpha kept, not one flat colour (the street shows through).
fn mixed_back(a: &[u8], b: &[u8]) -> bool {
    distance(a, b).1 as f64 / (a.len() / 4) as f64 > 0.9 && !flat(a) && a.chunks_exact(4).zip(b.chunks_exact(4)).all(|(p, q)| p[3] == q[3])
}

#[test]
fn b298_eyedropper_fill() {
    let mut t = Table::new(
        "eyedropper_fill",
        "# D-419: Eyedropper Fill\n\nB-298, after After Effects' Eyedropper Fill: an area about the \
         Sample Point (per cent of the drawing) of every pixel whose centre lies within Sample Radius, \
         row by row (or the one pixel holding the point when none does), pixels past the layer counted \
         as clear; its colour the straight colours' average over the pixels showing (Skip Empty) or \
         over all (All), the premultiplied average (All Premultiplied), or that with the area's \
         covering (Including Alpha); the layer filled with it, times each pixel's alpha with Maintain \
         Original Alpha, then mixed back by Blend With Original. The layer never grows. Every expected \
         pixel is `Fixtures/eyedropper_fill/expected_eyedropper_fill.json`, written by \
         `tools/eyedropper_fill_reference.py` before this code existed and printed in document 25 as \
         FX-EYEFILL-001 to 032. Tolerance 2e-5.\n",
    );

    t.heading("FX-EYEFILL-001 to 032 (document 25)");
    t.fixtures_numbered("expected_eyedropper_fill.json", 1..=32);

    t.heading("The file");
    let all = files("fx_eyefill", 32);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_eyefill_022.json");
    t.row(
        "fx_eyefill_022.json is saved with its words and numbers as written",
        &saved.to_string(),
        saved["sample_point"] == json!([12.5, 50]) && saved["sample_radius"] == 2 && saved["average_pixel_colors"] == "including_alpha"
            && saved["maintain_original_alpha"] == "on" && saved["blend_with_original"] == 50,
    );
    why(
        &mut t,
        &[
            ("fx_eyefill_024.json", "Eyedropper Fill's sample point runs from -1000 to 1000, and this is 1001."),
            ("fx_eyefill_025.json", "Eyedropper Fill's sample point runs from -1000 to 1000, and this is -1001."),
            ("fx_eyefill_026.json", "Eyedropper Fill's sample radius runs from 0 to 10000, and this is -1."),
            ("fx_eyefill_027.json", "Eyedropper Fill's sample radius runs from 0 to 10000, and this is 10001."),
            ("fx_eyefill_028.json", "Eyedropper Fill's blend with original runs from 0 to 100, and this is -1."),
            ("fx_eyefill_029.json", "Eyedropper Fill's blend with original runs from 0 to 100, and this is 101."),
            ("fx_eyefill_030.json", "Eyedropper Fill's average pixel colors is one of skip_empty, all, all_premultiplied, including_alpha, and this is \"sum\"."),
            ("fx_eyefill_031.json", "Eyedropper Fill's maintain original alpha is \"off\" or \"on\", and this is \"yes\"."),
        ],
    );
    t.shape_refused("fx_eyefill_001.json", "an Eyedropper Fill with no `blend_with_original`", r#"{"sample_point": [50, 50], "sample_radius": 0, "average_pixel_colors": "skip_empty", "maintain_original_alpha": "off"}"#);
    t.shape_refused("fx_eyefill_001.json", "an Eyedropper Fill whose sample point is one number", r#"{"sample_point": 50, "sample_radius": 0, "average_pixel_colors": "skip_empty", "maintain_original_alpha": "off", "blend_with_original": 0}"#);
    t.shape_refused("fx_eyefill_001.json", "an Eyedropper Fill whose maintain original alpha is true", r#"{"sample_point": [50, 50], "sample_radius": 0, "average_pixel_colors": "skip_empty", "maintain_original_alpha": true, "blend_with_original": 0}"#);

    t.heading("Commands");
    let mut document = t.load("fx_eyefill_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("sample point 1001, 50", set(eye([1001.0, 50.0], [0.0, 0.0], ["skip_empty", "off"]))),
            ("sample radius -1", set(eye([50.0, 50.0], [-1.0, 0.0], ["skip_empty", "off"]))),
            ("blend with original 101", set(eye([50.0, 50.0], [0.0, 101.0], ["skip_empty", "off"]))),
            ("average pixel colors \"sum\"", set(eye([50.0, 50.0], [0.0, 0.0], ["sum", "off"]))),
            ("maintain original alpha \"yes\"", set(eye([50.0, 50.0], [0.0, 0.0], ["skip_empty", "yes"]))),
            ("blend with original keyed to 101", keys("blend_with_original", &[(0, &[0.0]), (4, &[101.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_eyefill_001.json",
        vec![
            ("sample point 12.5, 50, radius 2, Including Alpha, alpha kept, blend 30", set(eye([12.5, 50.0], [2.0, 30.0], ["including_alpha", "on"]))),
            ("sample point keyed from 50, 50 to 12.5, 50", keys("sample_point", &[(0, &[50.0, 50.0]), (4, &[12.5, 50.0])])),
            ("sample radius keyed from 0 to 4", keys("sample_radius", &[(0, &[0.0]), (4, &[4.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_eyefill_002.json", 0), ("fx_eyefill_005.json", 0), ("fx_eyefill_010.json", 0), ("fx_eyefill_011.json", 0), ("fx_eyefill_017.json", 2), ("fx_eyefill_022.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // FX-EYEFILL-009 (blend 100) changes nothing and 024 to 032 are refused, so the card is not asked.
    let none: Vec<u32> = [9].into_iter().chain(24..=32).collect();
    card_fixtures(&mut t, &mut gpu, "fx_eyefill", 32, &none, is_eye);
    card_reference(
        &mut t,
        &mut gpu,
        "Eyedropper Fill",
        "core.eyedropper_fill",
        &[
            ("as added (the middle pixel)", json!({})),
            ("point 30, 40, radius 50, All", json!({"sample_point": [30, 40], "sample_radius": 50, "average_pixel_colors": "all"})),
            ("radius 200, Including Alpha, alpha kept", json!({"sample_radius": 200, "average_pixel_colors": "including_alpha", "maintain_original_alpha": "on"})),
            ("radius 1000, blend 50", json!({"sample_radius": 1000, "blend_with_original": 50})),
            ("point 75, 25, radius 10, All Premultiplied, alpha kept", json!({"sample_point": [75, 25], "sample_radius": 10, "average_pixel_colors": "all_premultiplied", "maintain_original_alpha": "on"})),
        ],
        is_eye,
    );

    street(
        &mut t,
        "D-419",
        "core.eyedropper_fill",
        &[
            ("as_added", json!({}), "as added: the whole street the colour of its middle pixel", one_colour),
            ("radius_60", json!({"sample_radius": 60}), "radius 60: the whole street the average colour about its middle", one_colour),
            ("upper_left_kept", json!({"sample_point": [20, 20], "sample_radius": 30, "maintain_original_alpha": "on", "blend_with_original": 40}), "point 20, 20, radius 30, alpha kept, blend 40: the upper left's colour over the street, the street showing through", mixed_back),
        ],
    );

    t.finish("D-419_eyedropper_fill_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B298_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-298: a measurement, run deliberately with --release --ignored"]
fn b298_eyedropper_fill_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B298_CPU").is_ok();
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
        ("Noise, then Eyedropper Fill as added (one pixel)", Some(json!({}))),
        ("Noise, then Eyedropper Fill, radius 200, All", Some(json!({"sample_radius": 200, "average_pixel_colors": "all"}))),
        ("Noise, then Eyedropper Fill, radius 1000, Including Alpha, alpha kept", Some(json!({"sample_radius": 1000, "average_pixel_colors": "including_alpha", "maintain_original_alpha": "on"}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx("core.noise", &format!("{id}n"), &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some(p) = &e {
                v.push(fx("core.eyedropper_fill", &format!("{id}c"), p));
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
    let out = std::env::var("B298_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-298_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
