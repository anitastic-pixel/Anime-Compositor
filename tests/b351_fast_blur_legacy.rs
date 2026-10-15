//! B-351: D-471, After Effects' Fast Blur (Legacy) (Obsolete) under `core.fast_blur_legacy`, a
//! second name over Fast Box Blur's engine (D-327), as D-383 and D-394: Blurriness, Blur
//! Dimensions and Repeat Edge Pixels, drawn as Fast Box Blur at radius 0.37 x Blurriness and 3
//! iterations.
//!
//! Writes `verification/D-471_fast_blur_legacy_table.md` and draws pictures into
//! `verification/D-471 pictures/`.
//!
//! Every expected pixel is `Fixtures/fast_blur_legacy/expected_fast_blur_legacy.json`, written by
//! `tools/fast_blur_legacy_reference.py` before this code existed and printed in document 25 as
//! FX-FASTLEG-001 to 013. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, changed, distance, keys, median, on_card, reference, repo, set, town, Table, MAIN, TOWN,
};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

fn legacy(blurriness: f64, edges: &str, dimensions: &str) -> Effect {
    Effect::FastBlurLegacy { blurriness, edges: edges.into(), dimensions: dimensions.into() }
}

/// A Fast Blur (Legacy) with the settings `p` over Blurriness 10, transparent edges, both ways.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"blurriness": 10, "edges": "transparent", "dimensions": "both"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.fast_blur_legacy", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left off the card; FX-FASTLEG-008 is a Float composition, which
/// the card refuses whole (D-319), so the processor draws it and the card's frame says so.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=13 {
        let file = format!("fx_fastleg_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::FastBlurLegacy { .. }));
            }
        }
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && if n == 8 { refused } else { !refused && agree && (card == 0) == none.contains(&n) },
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor, 3 on the card each frame.
fn card_reference(t: &mut Table, gpu: &mut Gpu, cases: &[(&str, J)]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p) in cases {
        let project = reference(|id| json!([fx(&format!("b351-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Fast Blur (Legacy) {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::FastBlurLegacy { .. }));
                t.row(
                    &format!("the reference shot, Fast Blur (Legacy) {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

/// `effects` on the still `asset` (in `dir`, `size` pixels), drawn on the processor at frame 0,
/// straight 8-bit, with what it warned of.
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

#[test]
fn b351_fast_blur_legacy() {
    let mut t = Table::new(
        "fast_blur_legacy",
        "# D-471: Fast Blur (Legacy)\n\nB-351: `core.fast_blur_legacy` takes After Effects' Fast Blur (Legacy) \
         controls (Blurriness, Blur Dimensions, Repeat Edge Pixels) and draws them with Fast Box Blur's engine at \
         radius 0.37 times Blurriness and 3 iterations (Maschwitz's match; D-383 and D-394, both names, one engine). \
         Every expected pixel is `Fixtures/fast_blur_legacy/expected_fast_blur_legacy.json`, written by \
         `tools/fast_blur_legacy_reference.py` before this code existed and printed in document 25 as \
         FX-FASTLEG-001 to 013. Tolerance 2e-5.\n",
    );

    t.heading("FX-FASTLEG-001 to 013 (document 25)");
    t.fixtures_numbered("expected_fast_blur_legacy.json", 1..=13);

    t.heading("One engine");
    let text = fs::read_to_string(t.root.join("fx_fastleg_001.json")).unwrap();
    let mut j: J = serde_json::from_str(&text).unwrap();
    j["compositions"][0]["layers"][0]["effects"][0] = json!({"instance_id": "fx-0-0", "type_id": "core.fast_box_blur", "enabled": true,
        "parameters": {"radius": 3.7, "iterations": 3}});
    let boxed = persist::load_str(&j.to_string()).expect("the Fast Box Blur project reads").document;
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_fast_blur_legacy.json")).unwrap()).unwrap();
    let d = effect_table::largest_difference(&t.render(&boxed, 0, 64), &expected["cases"]["FX-FASTLEG-001"]["frames"]["0"]);
    t.row(
        "FX-FASTLEG-001's drawing with Fast Box Blur, radius 3.7 and 3 iterations, in its place draws FX-FASTLEG-001's frame (Blurriness 10)",
        &format!("largest difference {d:.1e}"),
        d <= 2e-5,
    );

    t.heading("The file");
    let files: Vec<String> = (1..=8).map(|n| format!("fx_fastleg_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_fastleg_004.json");
    t.row(
        "fx_fastleg_004.json is saved with its blurriness and dimensions, and no radius or iterations of its own",
        &saved.to_string(),
        saved["blurriness"] == 10.0 && saved["dimensions"] == "horizontal" && saved.get("radius").is_none() && saved.get("iterations").is_none(),
    );
    for (file, want) in [
        ("fx_fastleg_009.json", "blurriness"),
        ("fx_fastleg_010.json", "blurriness"),
        ("fx_fastleg_011.json", "dimensions"),
        ("fx_fastleg_012.json", "edges"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming Fast Blur (Legacy) and its {want}"), &why, why.contains("Fast Blur (Legacy)") && why.contains(want));
    }
    let full = r##"{"blurriness": 10, "edges": "transparent", "dimensions": "both"}"##;
    t.shape_refused("fx_fastleg_001.json", "a Fast Blur (Legacy) whose blurriness is a word", &full.replace("10", "\"soft\""));
    t.shape_refused("fx_fastleg_001.json", "a Fast Blur (Legacy) whose dimensions is a number", &full.replace("\"both\"", "2"));

    t.heading("Commands");
    let mut document = t.load("fx_fastleg_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("blurriness 1001", set(legacy(1001.0, "transparent", "both"))),
            ("blurriness -1", set(legacy(-1.0, "transparent", "both"))),
            ("Blur Dimensions \"Both\", written with a capital", set(legacy(10.0, "transparent", "Both"))),
            ("edges \"on\"", set(legacy(10.0, "on", "both"))),
            ("blurriness keyed to 1200", keys("blurriness", &[(0, &[10.0]), (4, &[1200.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_fastleg_001.json",
        vec![
            ("blurriness 20, edges repeated, horizontal", set(legacy(20.0, "repeat", "horizontal"))),
            ("blurriness keyed from 0 to 1000", keys("blurriness", &[(0, &[0.0]), (4, &[1000.0])])),
        ],
    );

    t.heading("A half-size draft");
    let mut draft = legacy(20.0, "transparent", "both");
    draft.scale_distances(|v| v / 2.0);
    t.row("A half-size draft halves the blurriness", &format!("{draft:?}"), draft == legacy(10.0, "transparent", "both"));

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_fastleg_001.json", 0), ("fx_fastleg_003.json", 0), ("fx_fastleg_005.json", 0), ("fx_fastleg_007.json", 2)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Blurriness 0 (006) leaves the effect out; refused ones (009 to 013) leave nothing for the card.
    let none: Vec<u32> = [6].into_iter().chain(9..=13).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("Blurriness 10", json!({})),
            ("Blurriness 60, Repeat Edge Pixels, horizontal", json!({"blurriness": 60, "edges": "repeat", "dimensions": "horizontal"})),
        ],
    );

    t.heading("Pictures: in `verification/D-471 pictures/`");
    let dir = repo("verification/D-471 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let street = |e: J| picture(&dir, "town.png", TOWN, json!([e]));
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());

    let (legacy20, said) = street(fx("fx-0-0", &json!({"blurriness": 20})));
    write("2_blurriness_20.png", &legacy20);
    let (boxed, said_box) = street(json!({"instance_id": "fx-0-0", "type_id": "core.fast_box_blur", "enabled": true,
        "parameters": {"radius": 7.4, "iterations": 3}}));
    write("3_fast_box_radius_7_4.png", &boxed);
    t.row(
        "2_blurriness_20.png, Fast Blur (Legacy) Blurriness 20, and 3_fast_box_radius_7_4.png, Fast Box Blur radius 7.4 with 3 iterations: the same soft street, byte for byte; both draw cleanly",
        &format!("{said:?} {said_box:?}, changed {}, the same {}", legacy20 != before, legacy20 == boxed),
        said.is_empty() && said_box.is_empty() && legacy20 != before && legacy20 == boxed,
    );

    let (across, said) = street(fx("fx-0-0", &json!({"blurriness": 40, "dimensions": "horizontal"})));
    write("4_horizontal_40.png", &across);
    // A row's sums of differences against the street, across and down.
    let (w, h) = TOWN;
    let step = |p: &[u8], dx: usize, dy: usize| {
        let mut s = 0u64;
        for y in 0..h - dy {
            for x in 0..w - dx {
                let (a, b) = ((y * w + x) * 4, ((y + dy) * w + x + dx) * 4);
                s += p[a].abs_diff(p[b]) as u64;
            }
        }
        s
    };
    t.row(
        "4_horizontal_40.png, Blurriness 40, Blur Dimensions horizontal: smeared sideways only, so neighbours across differ far less than in the street while neighbours down differ about as much; draws cleanly",
        &format!("{said:?}, across {} against {}, down {} against {}", step(&across, 1, 0), step(&before, 1, 0), step(&across, 0, 1), step(&before, 0, 1)),
        said.is_empty() && step(&across, 1, 0) * 3 < step(&before, 1, 0) && step(&across, 0, 1) * 2 > step(&before, 0, 1),
    );

    t.finish("D-471_fast_blur_legacy_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B351_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-351: a measurement, run deliberately with --release --ignored"]
fn b351_fast_blur_legacy_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B351_CPU").is_ok();
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
        ("Noise, then Fast Blur (Legacy) Blurriness 10", Some(json!({}))),
        ("the same at Blurriness 100", Some(json!({"blurriness": 100}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}b"), p));
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
    let out = std::env::var("B351_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-351_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
