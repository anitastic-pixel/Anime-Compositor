//! B-349: D-469, CycoreFX's CC Threshold RGB under `core.threshold_rgb`: a threshold and an Invert
//! for each of red, green and blue and Blend With Original, a second name over Channel
//! Threshold's rule (D-468, as D-383 and D-394).
//!
//! Writes `verification/D-469_threshold_rgb_table.md` and draws pictures into
//! `verification/D-469 pictures/`.
//!
//! Every expected pixel is `Fixtures/threshold_rgb/expected_threshold_rgb.json`, written by
//! `tools/threshold_rgb_reference.py` before this code existed and printed in document 25 as
//! FX-THRESHRGB-001 to 014. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// The Threshold RGBs the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::ThresholdRgb { .. })))
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

/// A Threshold RGB with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"red_threshold": 128, "green_threshold": 128, "blue_threshold": 128, "invert_red": "off", "invert_green": "off", "invert_blue": "off", "blend_with_original": 0});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.threshold_rgb", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning or stay on the processor.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=14 {
        let file = format!("fx_threshrgb_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b349-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Threshold RGB {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Threshold RGB {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the still `asset` (in `dir`, `size` pixels), drawn on the processor, straight
/// 8-bit, with what it warned of.
fn picture(dir: &Path, asset: &str, size: (usize, usize), effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from(asset);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(size.0);
    comp["height"] = J::from(size.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([size.0 as f64 / 2.0, size.1 as f64 / 2.0]);
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

/// Covered pixels of `before` and `after` side by side.
fn pairs<'a>(before: &'a [u8], after: &'a [u8]) -> impl Iterator<Item = (&'a [u8], &'a [u8])> {
    before.chunks_exact(4).zip(after.chunks_exact(4)).filter(|(b, _)| b[3] > 0)
}

#[test]
fn b349_threshold_rgb() {
    let mut t = Table::new(
        "threshold_rgb",
        "# D-469: Threshold RGB\n\nB-349: `core.threshold_rgb` takes CycoreFX's CC Threshold RGB \
         controls, a threshold and an Invert for each of red, green and blue and Blend With \
         Original, under our own name: a second name over Channel Threshold's rule (D-468, as \
         D-383 and D-394) with a level and an Invert per channel. Every expected pixel is \
         `Fixtures/threshold_rgb/expected_threshold_rgb.json`, written by \
         `tools/threshold_rgb_reference.py` before this code existed and printed in document 25 as \
         FX-THRESHRGB-001 to 014; Channel Threshold's FX-CHTHRESH-001 to 023 and Threshold's \
         FX-THRESH-001 to 018 rerun unchanged (019 is in dispute, D-164). Tolerance 2e-5.\n",
    );

    t.heading("FX-THRESHRGB-001 to 014 (document 25)");
    t.fixtures_numbered("expected_threshold_rgb.json", 1..=14);

    t.heading("Channel Threshold and Threshold, the rule under the second name: their fixtures, unchanged");
    let root = t.root.clone();
    t.root = repo("Fixtures/channel_threshold");
    t.fixtures_numbered("expected_channel_threshold.json", 1..=23);
    t.root = repo("Fixtures/threshold");
    t.fixtures_numbered("expected_threshold.json", 1..=18);
    t.root = root;

    t.heading("The file");
    let files: Vec<String> = (1..=14).map(|n| format!("fx_threshrgb_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_threshrgb_002.json");
    t.row(
        "fx_threshrgb_002.json is saved with its seven settings",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 7
            && saved["red_threshold"] == 230.0
            && saved["green_threshold"] == 128.0
            && saved["blue_threshold"] == 50.0
            && saved["invert_red"] == "off"
            && saved["invert_green"] == "off"
            && saved["invert_blue"] == "off"
            && saved["blend_with_original"] == 0.0,
    );
    for (file, want) in [
        ("fx_threshrgb_010.json", "red_threshold"),
        ("fx_threshrgb_011.json", "green_threshold"),
        ("fx_threshrgb_013.json", "invert red"),
        ("fx_threshrgb_014.json", "blend_with_original"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        let says = why.contains(want) || why.contains(&want.replace('_', " "));
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, says);
    }
    t.shape_refused("fx_threshrgb_001.json", "a Threshold RGB whose red threshold is a word", r#"{"red_threshold": "half", "green_threshold": 128, "blue_threshold": 128, "invert_red": "off", "invert_green": "off", "invert_blue": "off", "blend_with_original": 0}"#);
    t.shape_refused("fx_threshrgb_001.json", "a Threshold RGB without its blue threshold", r#"{"red_threshold": 128, "green_threshold": 128, "invert_red": "off", "invert_green": "off", "invert_blue": "off", "blend_with_original": 0}"#);
    t.shape_refused("fx_threshrgb_001.json", "a Threshold RGB whose invert green is a number", r#"{"red_threshold": 128, "green_threshold": 128, "blue_threshold": 128, "invert_red": "off", "invert_green": 1, "invert_blue": "off", "blend_with_original": 0}"#);

    t.heading("Commands");
    let mut document = t.load("fx_threshrgb_001.json").document;
    let over = changed(&t, "fx_threshrgb_001.json", |e| if let Effect::ThresholdRgb { red_threshold, .. } = e { *red_threshold = 256.0 });
    let bad = changed(&t, "fx_threshrgb_001.json", |e| if let Effect::ThresholdRgb { invert_blue, .. } = e { *invert_blue = "yes".into() });
    t.refused(
        &mut document,
        vec![
            ("red threshold 256", set(over)),
            ("invert blue yes", set(bad)),
            ("green threshold keyed to 300", keys("green_threshold", &[(0, &[128.0]), (4, &[300.0])])),
        ],
    );
    let green = changed(&t, "fx_threshrgb_001.json", |e| {
        if let Effect::ThresholdRgb { invert_green, blue_threshold, .. } = e {
            *invert_green = "on".into();
            *blue_threshold = 40.0;
        }
    });
    t.taken(
        &mut document,
        "fx_threshrgb_001.json",
        vec![
            ("green turned over, blue at 40", set(green)),
            ("red keyed from 0 to 255", keys("red_threshold", &[(0, &[0.0]), (4, &[255.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_threshrgb_001.json", 0),
        ("fx_threshrgb_002.json", 0),
        ("fx_threshrgb_003.json", 0),
        ("fx_threshrgb_005.json", 0),
        ("fx_threshrgb_007.json", 2),
        ("fx_threshrgb_009.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Refused (010 to 014) leave nothing for the card.
    let none: Vec<u32> = (10..=14).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({}), 3),
            ("red 200, green 90 turned over, blue 40, blend 30", json!({"red_threshold": 200, "green_threshold": 90, "blue_threshold": 40, "invert_green": "on", "blend_with_original": 30}), 3),
            ("all three turned over", json!({"invert_red": "on", "invert_green": "on", "invert_blue": "on"}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-469 pictures/`");
    let dir = repo("verification/D-469 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]));
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (same, said) = street(json!({"blend_with_original": 100}));
    t.row(
        "blend with original 100 changes nothing: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );

    let solid = |b: &[u8]| b[3] == 255;
    let full = |v: u8| v == 0 || v == 255;
    let (plain, said) = street(json!({}));
    write("2_as_added.png", TOWN, &plain);
    let eight = pairs(&before, &plain).filter(|(b, _)| solid(b)).all(|(_, a)| a[..3].iter().all(|v| full(*v)));
    t.row(
        "2_as_added.png, as added (every threshold 128): every channel of every solid pixel full or nothing, so one of eight pure colours; draws cleanly",
        &format!("{said:?}, every channel full or nothing: {eight}"),
        said.is_empty() && eight && plain != before,
    );

    let (low_red, said) = street(json!({"red_threshold": 20}));
    write("3_red_20.png", TOWN, &low_red);
    let redder = pairs(&plain, &low_red).filter(|(b, _)| solid(b)).all(|(p, q)| q[0] >= p[0] && q[1] == p[1] && q[2] == p[2]);
    let some = pairs(&plain, &low_red).any(|(p, q)| q[0] > p[0]);
    t.row(
        "3_red_20.png, red at 20, the rest at 128: red full in more of the street than in 2, green and blue as in 2; draws cleanly",
        &format!("{said:?}, red only grows and the rest is kept: {redder}, some pixels gain red: {some}"),
        said.is_empty() && redder && some,
    );

    let (green, said) = street(json!({"invert_green": "on"}));
    write("4_invert_green.png", TOWN, &green);
    let flipped = pairs(&plain, &green).filter(|(b, _)| solid(b)).all(|(p, q)| q[1] == 255 - p[1] && q[0] == p[0] && q[2] == p[2]);
    t.row(
        "4_invert_green.png, Invert Green: 2 with its green turned over and its red and blue kept; draws cleanly",
        &format!("{said:?}, green turned over, the rest kept: {flipped}"),
        said.is_empty() && flipped,
    );

    let (half, said) = street(json!({"blend_with_original": 50}));
    write("5_blend_50.png", TOWN, &half);
    t.row(
        "5_blend_50.png, as added at blend 50: nearer the street than 2 is; draws cleanly",
        &format!("{said:?}, largest difference from the street {} against 2's {}", distance(&half, &before).0, distance(&plain, &before).0),
        said.is_empty() && distance(&half, &before).0 < distance(&plain, &before).0,
    );

    // The second name: one level and one Invert for all three draws Channel Threshold's RGB
    // street byte for byte.
    for (level, invert) in [(60, "off"), (128, "on"), (210, "off")] {
        let ch = json!({"instance_id": "fx-0-0", "type_id": "core.channel_threshold", "enabled": true, "parameters": {
            "threshold": level, "channel": "rgb", "invert": invert, "blend_with_original": 20}});
        let (a, said0) = street(json!({"red_threshold": level, "green_threshold": level, "blue_threshold": level,
            "invert_red": invert, "invert_green": invert, "invert_blue": invert, "blend_with_original": 20}));
        let (b, said1) = picture(&dir, "town.png", TOWN, json!([ch]));
        t.row(
            &format!("Threshold RGB with every level {level} and Invert {invert} at blend 20 against Channel Threshold's RGB the same: the same street byte for byte; both draw cleanly"),
            &format!("{said0:?} {said1:?}, {} pixels differ", distance(&a, &b).1),
            said0.is_empty() && said1.is_empty() && a == b && a != before,
        );
    }

    t.finish("D-469_threshold_rgb_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B349_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-349: a measurement, run deliberately with --release --ignored"]
fn b349_threshold_rgb_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B349_CPU").is_ok();
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
        ("Noise, then Threshold RGB as added (every level 128)", Some(json!({}))),
        ("Noise, then Threshold RGB, red 200, green 90 turned over, blue 40, blend 30", Some(json!({"red_threshold": 200, "green_threshold": 90, "blue_threshold": 40, "invert_green": "on", "blend_with_original": 30}))),
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
    let out = std::env::var("B349_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-349_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
