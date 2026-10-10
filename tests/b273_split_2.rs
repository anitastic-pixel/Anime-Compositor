//! B-273: D-394 Split 2, after CycoreFX's CC Split 2 ("Distort" in `docs/effects/EFFECTS.md`):
//! Split's engine under a second name, each side of the tear opened by its own amount.
//!
//! Writes `verification/D-394_split_2_table.md`.
//!
//! Every expected pixel is `Fixtures/split_2/expected_split_2.json`, written by
//! `tools/split2_reference.py` before this code existed and printed in document 25 as
//! FX-SPLIT2-001 to 019. Tolerance 2e-5. Nothing here is a snapshot of a run. Split's own
//! pictures (`verification/D-393 pictures/`) are drawn again and must match byte for byte.
//!
//! Also draws the street into `verification/D-394 pictures/`.

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

fn split_2(point_a: [f64; 2], point_b: [f64; 2], split_1: f64, split_2: f64) -> Effect {
    Effect::Split2 { point_a, point_b, split_1, split_2 }
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
        "core.slant" => json!({"slant": 0, "stretching": "off", "height": 100, "floor": [50, 100], "set_color": "off", "color": "#000000"}),
        "core.smear" => json!({"from": [40, 50], "to": [60, 50], "reach": 100, "radius": 70}),
        "core.split" => json!({"point_a": [25, 50], "point_b": [75, 50], "split": 50}),
        "core.split_2" => json!({"point_a": [25, 50], "point_b": [75, 50], "split_1": 50, "split_2": 50}),
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
        let project = reference(|id| json!([fx(type_id, &format!("b273-{id}"), p)]));
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

/// Changed, and some of it left clear.
fn changed_with_gaps(a: &[u8], b: &[u8]) -> bool {
    changed(a, b) && a.chunks_exact(4).any(|p| p[3] == 0)
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

// --- Split 2 ---------------------------------------------------------------------------------

fn is_split_2(e: &Effect) -> bool {
    matches!(e, Effect::Split2 { .. })
}

/// The street drawn with `effects`, written by the same writer as the pictures, as file bytes.
fn picture_file(effects: J, name: &str) -> Vec<u8> {
    let dir = repo("verification/D-393 pictures");
    let (bytes, _) = picture(&dir, effects);
    let out = std::env::temp_dir().join(format!("b273_{}_{name}", std::process::id()));
    png_out::write_rgba(&out, TOWN.0, TOWN.1, OutputDepth::Eight, &[], &bytes).unwrap();
    let file = fs::read(&out).unwrap();
    let _ = fs::remove_file(&out);
    file
}

#[test]
fn b273_split_2() {
    let mut t = Table::new(
        "split_2",
        "# D-394: Split 2\n\nB-273, after CycoreFX's CC Split 2: Split (D-393) under a second name, \
         the side of the tear on your left as you walk from Point A to Point B opened by Split 1 \
         and the side on your right by Split 2. The owner chose \"Second name, one engine \
         (Recommended)\" on 2026-10-09. The formulas are this program's own. Every expected pixel \
         is `Fixtures/split_2/expected_split_2.json`, written by `tools/split2_reference.py` before \
         this code existed and printed in document 25 as FX-SPLIT2-001 to 019. Tolerance 2e-5.\n",
    );

    t.heading("FX-SPLIT2-001 to 019 (document 25)");
    t.fixtures("expected_split_2.json");

    t.heading("Split draws exactly as before");
    for (i, (name, p)) in [
        ("as_added", json!({})),
        ("wide", json!({"split": 150})),
        ("diagonal", json!({"point_a": [10, 90], "point_b": [90, 10], "split": 40})),
    ]
    .iter()
    .enumerate()
    {
        let file = format!("{}_{name}.png", i + 2);
        let committed = fs::read(repo(&format!("verification/D-393 pictures/{file}"))).expect("D-393's picture");
        let now = picture_file(json!([fx("core.split", "fx-0-0", p)]), &file);
        t.row(
            &format!("Split's street picture {file}, drawn again, is the committed D-393 file byte for byte"),
            &format!("{} bytes now, {} committed", now.len(), committed.len()),
            now == committed,
        );
    }
    for (what, p, q) in [
        ("as they start, both 50", json!({}), json!({})),
        ("both 150", json!({"split_1": 150, "split_2": 150}), json!({"split": 150})),
        ("corner to corner, both 40", json!({"point_a": [10, 90], "point_b": [90, 10], "split_1": 40, "split_2": 40}), json!({"point_a": [10, 90], "point_b": [90, 10], "split": 40})),
    ] {
        let dir = repo("verification/D-393 pictures");
        let (two, _) = picture(&dir, json!([fx("core.split_2", "fx-0-0", &p)]));
        let (one, _) = picture(&dir, json!([fx("core.split", "fx-0-0", &q)]));
        t.row(&format!("Split 2 with equal sides, {what}, on the street is Split's picture byte for byte"), &format!("{} pixels differ", distance(&two, &one).1), two == one);
    }

    t.heading("The file");
    let all = files("fx_split2", 19);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_split2_008.json");
    t.row(
        "fx_split2_008.json is saved with its two points and both splits",
        &saved.to_string(),
        saved["point_a"] == json!([10, 10]) && saved["point_b"] == json!([90, 90]) && saved["split_1"] == 3 && saved["split_2"] == 6,
    );
    why(
        &mut t,
        &[
            ("fx_split2_015.json", "Split 2's split 1 runs from 0 to 1000, and this is 1001."),
            ("fx_split2_016.json", "Split 2's split 2 runs from 0 to 1000, and this is -1."),
            ("fx_split2_017.json", "Split 2's point a runs from -1000 to 1000, and this is 1001."),
            ("fx_split2_018.json", "Split 2's point b runs from -1000 to 1000, and this is -1001."),
        ],
    );
    t.shape_refused("fx_split2_001.json", "a Split 2 with no `split_2`", r#"{"point_a": [25, 50], "point_b": [75, 50], "split_1": 50}"#);
    t.shape_refused("fx_split2_001.json", "a Split 2 with Split's one `split` instead", r#"{"point_a": [25, 50], "point_b": [75, 50], "split": 50}"#);

    t.heading("Commands");
    let mut document = t.load("fx_split2_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("split 1 1000.5", set(split_2([25.0, 50.0], [75.0, 50.0], 1000.5, 50.0))),
            ("split 2 -0.5", set(split_2([25.0, 50.0], [75.0, 50.0], 50.0, -0.5))),
            ("split 2 keyed to -10", keys("split_2", &[(0, &[50.0]), (4, &[-10.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_split2_001.json",
        vec![
            ("from 20, 30 to 80, 70, split 1 35, split 2 10", set(split_2([20.0, 30.0], [80.0, 70.0], 35.0, 10.0))),
            ("split 1 keyed from 0 to 200", keys("split_1", &[(0, &[0.0]), (4, &[200.0])])),
            ("point b keyed from 75, 50 to 75, 90", keys("point_b", &[(0, &[75.0, 50.0]), (4, &[75.0, 90.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_split2_001.json", 0), ("fx_split2_003.json", 0), ("fx_split2_007.json", 0), ("fx_split2_008.json", 0), ("fx_split2_011.json", 0), ("fx_split2_014.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_split2", 19, &[5, 6, 15, 16, 17, 18, 19], is_split_2);
    card_reference(
        &mut t,
        &mut gpu,
        "Split 2",
        "core.split_2",
        &[
            ("as it starts (25, 50 to 75, 50, both 50)", json!({})),
            ("down a diagonal, split 1 200, split 2 40", json!({"point_a": [10, 10], "point_b": [90, 90], "split_1": 200, "split_2": 40})),
            ("past both edges, split 1 0, split 2 30", json!({"point_a": [-20, 50], "point_b": [120, 40], "split_1": 0, "split_2": 30})),
        ],
        is_split_2,
    );

    street(
        &mut t,
        "D-394",
        "core.split_2",
        &[
            ("as_added", json!({}), "as it starts: both sides 50, Split's eye-shaped gap", changed_with_gaps),
            ("upper_only", json!({"split_1": 150, "split_2": 0}), "split 1 150, split 2 0: only the upper half pushed up, the lower half untouched", changed_with_gaps),
            ("lower_wide", json!({"split_1": 20, "split_2": 200}), "split 1 20, split 2 200: a little up, a lot down", changed_with_gaps),
            ("diagonal", json!({"point_a": [10, 90], "point_b": [90, 10], "split_1": 0, "split_2": 60}), "corner to corner, split 2 60: only the lower-right side opens", changed_with_gaps),
        ],
    );

    t.finish("D-394_split_2_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B273_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-273: a measurement, run deliberately with --release --ignored"]
fn b273_split_2_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B273_CPU").is_ok();
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
        ("Noise, then Split, a diagonal, split 200", Some(("core.split", json!({"point_a": [10, 10], "point_b": [90, 90], "split": 200})))),
        ("Noise, then Split 2, a diagonal, split 1 200, split 2 40", Some(("core.split_2", json!({"point_a": [10, 10], "point_b": [90, 90], "split_1": 200, "split_2": 40})))),
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
    let out = std::env::var("B273_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-273_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
