//! B-286: D-407 Detail-preserving Upscale, after After Effects' Detail-preserving Upscale
//! ("Distort" in `docs/effects/EFFECTS.md`): the layer enlarged up to ten times with its edges
//! kept sharp.
//!
//! Writes `verification/D-407_detail_upscale_table.md`.
//!
//! Every expected pixel is `Fixtures/detail_upscale/expected_detail_upscale.json`, written by
//! `tools/detail_upscale_reference.py` before this code existed and printed in document 25 as
//! FX-UPSCALE-001 to 020. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-407 pictures/`.

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

fn upscale(scale: f64, reduce_noise: f64, detail: f64) -> Effect {
    Effect::DetailUpscale { scale, reduce_noise, detail }
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
        "core.detail_upscale" => json!({"scale": 100, "reduce_noise": 0, "detail": 20}),
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
        let project = reference(|id| json!([fx(type_id, &format!("b286-{id}"), p)]));
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

// --- Detail-preserving Upscale --------------------------------------------------------------

fn is_upscale(e: &Effect) -> bool {
    matches!(e, Effect::DetailUpscale { .. })
}

/// FX-UPSCALE-001's file with three Motion Tiles of ten times the width put before its Upscale,
/// so the Upscale is handed a layer 16000 pixels wide, and the Upscale's scale set to `scale`
/// (or left out for `None`).
fn wide(t: &Table, scale: Option<f64>) -> Project {
    let mut j: J = serde_json::from_str(&fs::read_to_string(t.root.join("fx_upscale_001.json")).unwrap()).unwrap();
    let tile = |n: usize| {
        json!({"instance_id": format!("tile-{n}"), "type_id": "core.motion_tile", "enabled": true, "parameters": {
            "output_width": 1000, "output_height": 100, "mirror": "off", "tile_center": [50, 50], "tile_width": 100, "tile_height": 100}})
    };
    let mut effects = vec![tile(1), tile(2), tile(3)];
    if let Some(scale) = scale {
        effects.push(fx("core.detail_upscale", "fx-0-0", &json!({"scale": scale})));
    }
    j["compositions"][0]["layers"][0]["effects"] = J::Array(effects);
    persist::load_str(&j.to_string()).expect("the wide project reads").document.project().clone()
}

/// The processor's frame 0 at Full, straight 8-bit, with what it warned of.
fn cpu_frame(t: &Table, project: &Project) -> (Vec<u8>, String) {
    let mut log = FrameLog::new(3);
    let frame = preview::preview_frame_cached(project, &Id::new(MAIN), 0, &t.root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
        .expect("the frame draws");
    (frame.to_srgb8_straight(), said(log))
}

#[test]
fn b286_detail_upscale() {
    let mut t = Table::new(
        "detail_upscale",
        "# D-407: Detail-preserving Upscale\n\nB-286, after After Effects' Detail-preserving Upscale: \
         the layer softened by Reduce Noise (a Gaussian of sigma Reduce Noise / 50), grown by \
         (Scale - 100) / 200 of its size on each side, every pixel read through a Lanczos-3 filter \
         across then down, then sharpened by Detail / 50 of the difference from itself blurred at \
         sigma Scale / 200, and held within what can be shown. Every expected pixel is \
         `Fixtures/detail_upscale/expected_detail_upscale.json`, written by \
         `tools/detail_upscale_reference.py` before this code existed and printed in document 25 \
         as FX-UPSCALE-001 to 020. Tolerance 2e-5. The owner chose on 2026-10-10 the scale's range \
         (\"1000%, like AE\") and Fit to Comp Width and Height as one-time buttons; a layer grown \
         past 30000 pixels is not drawn and EFFECT_LAYER_TOO_LARGE says so.\n",
    );

    t.heading("FX-UPSCALE-001 to 020 (document 25)");
    t.fixtures("expected_detail_upscale.json");

    t.heading("The file");
    let all = files("fx_upscale", 20);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_upscale_008.json");
    t.row(
        "fx_upscale_008.json is saved with its scale, Reduce Noise and Detail",
        &saved.to_string(),
        saved["scale"] == 200 && saved["reduce_noise"] == 50 && saved["detail"] == 50,
    );
    why(
        &mut t,
        &[
            ("fx_upscale_015.json", "Detail-preserving Upscale's scale runs from 100 to 1000, and this is 99."),
            ("fx_upscale_016.json", "Detail-preserving Upscale's scale runs from 100 to 1000, and this is 1001."),
            ("fx_upscale_017.json", "Detail-preserving Upscale's reduce noise runs from 0 to 100, and this is -1."),
            ("fx_upscale_018.json", "Detail-preserving Upscale's reduce noise runs from 0 to 100, and this is 101."),
            ("fx_upscale_019.json", "Detail-preserving Upscale's detail runs from 0 to 100, and this is -1."),
            ("fx_upscale_020.json", "Detail-preserving Upscale's detail runs from 0 to 100, and this is 101."),
        ],
    );
    t.shape_refused("fx_upscale_001.json", "an Upscale with no `detail`", r#"{"scale": 200, "reduce_noise": 0}"#);
    t.shape_refused("fx_upscale_001.json", "an Upscale with its scale in words", r#"{"scale": "double", "reduce_noise": 0, "detail": 20}"#);
    t.shape_refused("fx_upscale_001.json", "an Upscale with two numbers for Detail", r#"{"scale": 200, "reduce_noise": 0, "detail": [20, 30]}"#);

    t.heading("Commands");
    let mut document = t.load("fx_upscale_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("scale 99", set(upscale(99.0, 0.0, 20.0))),
            ("Reduce Noise 101", set(upscale(200.0, 101.0, 20.0))),
            ("Detail keyed to 101", keys("detail", &[(0, &[4.0]), (4, &[101.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_upscale_001.json",
        vec![
            ("scale 250, Reduce Noise 30, Detail 40", set(upscale(250.0, 30.0, 40.0))),
            ("scale keyed from 100 to 400", keys("scale", &[(0, &[100.0]), (4, &[400.0])])),
            ("Detail keyed from 0 to 100", keys("detail", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_upscale_001.json", 0), ("fx_upscale_004.json", 0), ("fx_upscale_006.json", 0), ("fx_upscale_008.json", 0), ("fx_upscale_009.json", 0), ("fx_upscale_011.json", 2), ("fx_upscale_013.json", 0)]);

    t.heading("Too large for this build: EFFECT_LAYER_TOO_LARGE (document 28)");
    let mut gpu = Gpu::new().expect("a usable card");
    let (plain, plain_said) = cpu_frame(&t, &wide(&t, None));
    let refused = wide(&t, Some(200.0));
    let (big, big_said) = cpu_frame(&t, &refused);
    t.row(
        "three Motion Tiles make the 16-pixel layer 16000 wide; an Upscale of 200 would make it 32000, past 30000, so the processor leaves the Upscale out and says so",
        &format!("warnings [{big_said}]; {} pixels differ from the frame without the Upscale", distance(&big, &plain).1),
        big_said.contains(DiagnosticId::EffectLayerTooLarge.as_str()) && !plain_said.contains(DiagnosticId::EffectLayerTooLarge.as_str()) && distance(&big, &plain).1 == 0,
    );
    let ((d, _), _, _, gpu_said) = both(&mut gpu, &refused, &Id::new(MAIN), &t.root, 0, PreviewQuality::Full);
    t.row(
        "the same frame asked of the card: the Upscale left out the same way and the same warning",
        &format!("largest difference {d} of 255 from the processor; warnings [{gpu_said}]"),
        d <= 1 && gpu_said.contains(DiagnosticId::EffectLayerTooLarge.as_str()),
    );
    let mut log = FrameLog::new(3);
    let _ = compose::plan_frame_for_card(&refused, &Id::new(MAIN), 0, &t.root, PreviewQuality::Full, &mut log, &mut CelCache::viewer());
    let card_said = said(log);
    t.row(
        "the card's own plan refuses it before drawing, with the same warning",
        &format!("[{card_said}]; {} on the card", on_card(&refused, &Id::new(MAIN), &t.root, 0, PreviewQuality::Full, is_upscale)),
        card_said.contains(DiagnosticId::EffectLayerTooLarge.as_str()) && on_card(&refused, &Id::new(MAIN), &t.root, 0, PreviewQuality::Full, is_upscale) == 0,
    );
    let (edge, edge_said) = cpu_frame(&t, &wide(&t, Some(187.5)));
    t.row(
        "an Upscale of 187.5 makes it exactly 30000 wide, which is drawn",
        &format!("warnings [{edge_said}]; {} pixels differ from the frame without the Upscale", distance(&edge, &plain).1),
        !edge_said.contains(DiagnosticId::EffectLayerTooLarge.as_str()) && distance(&edge, &plain).1 > 0,
    );

    t.heading("On the card against the processor: within 1 level of 255");
    card_fixtures(&mut t, &mut gpu, "fx_upscale", 20, &[2, 15, 16, 17, 18, 19, 20], is_upscale);
    card_reference(
        &mut t,
        &mut gpu,
        "Detail-preserving Upscale",
        "core.detail_upscale",
        &[
            ("as it starts (scale 100, Detail 20)", json!({})),
            ("scale 200, Detail 0", json!({"scale": 200, "detail": 0})),
            ("scale 150, Reduce Noise 50, Detail 50", json!({"scale": 150, "reduce_noise": 50, "detail": 50})),
            ("scale 125, Detail 100", json!({"scale": 125, "detail": 100})),
        ],
        is_upscale,
    );

    street(
        &mut t,
        "D-407",
        "core.detail_upscale",
        &[
            ("as_added", json!({}), "as it starts: the same size, edges a little sharpened", changed),
            ("scale_200", json!({"scale": 200}), "scale 200: twice the size about the middle, cut by the frame's edges", changed),
            ("scale_1000", json!({"scale": 1000}), "scale 1000, the most: ten times the size, the middle of the street filling the frame", changed),
            ("soft", json!({"scale": 150, "reduce_noise": 100, "detail": 0}), "scale 150, Reduce Noise 100, Detail 0: enlarged and softened", changed),
            ("sharp", json!({"scale": 150, "detail": 100}), "scale 150, Detail 100: enlarged and sharpened hard, light halos at the edges", changed),
        ],
    );

    t.finish("D-407_detail_upscale_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B286_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-286: a measurement, run deliberately with --release --ignored"]
fn b286_detail_upscale_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B286_CPU").is_ok();
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
        ("Noise, then Upscale as added (scale 100, Detail 20)", Some(("core.detail_upscale", json!({})))),
        ("Noise, then Upscale, scale 200", Some(("core.detail_upscale", json!({"scale": 200})))),
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
    let out = std::env::var("B286_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-286_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
