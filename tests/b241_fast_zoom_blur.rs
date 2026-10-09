//! B-241: D-362, Fast Zoom Blur, after CycoreFX's CC Radial Fast Blur: quick fading streaks out
//! from a centre, plain or keeping only the lightest or the darkest ("Blur & Sharpen | CC Radial
//! Fast Blur" in `docs/effects/EFFECTS.md`).
//!
//! Writes `verification/D-362_fast_zoom_blur_table.md`.
//!
//! Every expected pixel is `Fixtures/fast_zoom_blur/expected_fast_zoom_blur.json`, written by
//! `tools/fast_zoom_blur_reference.py` before this code existed and printed in document 25 as
//! FX-FASTZOOM-001 to 014. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws a title and streaks light out of it, the tutorial's passing light, into
//! `verification/D-362 pictures/`.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, set, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

fn fast_zoom(amount: f64, center: [f64; 2], zoom: &str) -> Effect {
    Effect::FastZoomBlur { amount, center, zoom: zoom.into() }
}

const PLATE: (usize, usize) = (192, 108);

/// A title on a dark blue card: five white blocks 20 by 24 pixels, standing for letters, each
/// with a dark slit down its middle.
fn title() -> Vec<u8> {
    let (w, h) = PLATE;
    (0..w * h)
        .flat_map(|i| {
            let (x, y) = (i % w, i / w);
            let letter = (42..66).contains(&y) && (36..156).contains(&x) && (x - 36) % 24 < 20 && (x - 36) % 24 / 2 != 4;
            if letter {
                [250, 250, 250, 255]
            } else {
                [20, 30, 70, 255]
            }
        })
        .collect()
}

/// `effects` on the title, drawn, straight 8-bit.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J =
        serde_json::from_str(&fs::read_to_string(effect_table::repo("Fixtures/fast_zoom_blur/fx_fastzoom_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("title.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(PLATE.0);
    comp["height"] = J::from(PLATE.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([PLATE.0 as f64 / 2.0, PLATE.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

fn one(p: J) -> J {
    json!([{ "instance_id": "fx-0-0", "type_id": "core.fast_zoom_blur", "enabled": true, "parameters": p }])
}

/// The Fast Zoom Blurs the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Radial(r) if r.sweep.is_some()))
        .count()
}

fn distance(a: &[u8], b: &[u8]) -> (u8, usize) {
    assert_eq!(a.len(), b.len(), "the two pictures are different sizes");
    let (mut largest, mut pixels) = (0, 0);
    for (p, q) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
        // A pixel clear in both has no colour to compare, as B-233's: a far sample landing a
        // hair inside the drawing on one and a hair outside on the other leaves a covering that
        // rounds to 0 either way, a different colour each way and invisible either way.
        if p[3] == 0 && q[3] == 0 {
            continue;
        }
        let d =p.iter().zip(q).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0);
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
    let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        layers[i]["effects"] = stack(id);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}


#[test]
fn b241_fast_zoom_blur() {
    let mut t = Table::new(
        "fast_zoom_blur",
        "# D-362: Fast Zoom Blur\n\nB-241, after CycoreFX's CC Radial Fast Blur: each pixel reads \
         back along the line toward a centre, the nearer points counting more, so streaks fade as \
         they run out; Brightest keeps only the lightest of them and Darkest only the darkest. \
         Every expected pixel is `Fixtures/fast_zoom_blur/expected_fast_zoom_blur.json`, written \
         by `tools/fast_zoom_blur_reference.py` before this code existed and printed in document \
         25 as FX-FASTZOOM-001 to 014. The build's frame is compared sample by sample; the answer \
         is the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-FASTZOOM-001 to 014 (document 25)");
    t.fixtures("expected_fast_zoom_blur.json");

    t.heading("How far it reaches");
    let got = fast_zoom(100.0, [-1000.0, -1000.0], "brightest").bounds_expansion();
    t.row("it never grows the layer: it declares no growth, as Radial Blur", &format!("{got:?}"), got == 0);
    let mut draft = fast_zoom(70.0, [30.0, 40.0], "darkest");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview keeps every setting: the amount is a share of the distance and \
         the centre a share of the size, so the streaks halve with the picture",
        &format!("{draft:?}"),
        draft == fast_zoom(70.0, [30.0, 40.0], "darkest"),
    );

    t.heading("The file");
    let files: Vec<String> = (1..=14).map(|n| format!("fx_fastzoom_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_fastzoom_001.json");
    t.row(
        "fx_fastzoom_001.json, which leaves out the zoom, is saved without it: standard is the default",
        &saved.to_string(),
        saved.get("zoom").is_none() && saved["amount"] == 50,
    );
    let effect_of = |d: &anime_compositor::command::Document| {
        d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone()
    };
    let why = effect_of(&t.load("fx_fastzoom_012.json").document).why_invalid();
    t.row(
        "fx_fastzoom_012.json is refused in a sentence",
        &why,
        why == "Fast Zoom Blur's zoom is \"standard\", \"brightest\" or \"darkest\", and this is \"bright\".",
    );
    t.shape_refused("fx_fastzoom_001.json", "a Fast Zoom Blur with no `amount`", r#"{"center": [50, 50]}"#);
    t.shape_refused("fx_fastzoom_001.json", "a Fast Zoom Blur whose zoom is a number", r#"{"amount": 50, "center": [50, 50], "zoom": 1}"#);

    t.heading("Commands");
    let mut document = t.load("fx_fastzoom_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("amount 100.5", set(fast_zoom(100.5, [50.0, 50.0], "standard"))),
            ("amount -0.5", set(fast_zoom(-0.5, [50.0, 50.0], "standard"))),
            ("zoom \"Brightest\", written with a capital", set(fast_zoom(50.0, [50.0, 50.0], "Brightest"))),
            ("centre -1000.5, 50", set(fast_zoom(50.0, [-1000.5, 50.0], "standard"))),
            ("amount keyed to 120", keys("amount", &[(0, &[0.0]), (4, &[120.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_fastzoom_001.json",
        vec![
            ("amount 70, centre 30, 40, brightest,", set(fast_zoom(70.0, [30.0, 40.0], "brightest"))),
            ("centre keyed from 0, 50 to 100, 50", keys("center", &[(0, &[0.0, 50.0]), (4, &[100.0, 50.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_fastzoom_001.json", 0), ("fx_fastzoom_002.json", 0), ("fx_fastzoom_003.json", 0), ("fx_fastzoom_008.json", 2), ("fx_fastzoom_009.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    let fixture_root = effect_table::repo("Fixtures/fast_zoom_blur");
    for n in 1..=14 {
        let file = format!("fx_fastzoom_{n:03}.json");
        let project = persist::load(&fixture_root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(&mut gpu, &project, &comp, &fixture_root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &fixture_root, frame, quality);
            }
        }
        // FX-FASTZOOM-010 to 014 are left out with a warning.
        let none = n >= 10;
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none,
        );
    }
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = effect_table::repo("Fixtures/reference_shot");
    for (what, p) in [
        ("as added (amount 50, standard, about the middle)", json!({"amount": 50, "center": [50, 50]})),
        ("amount 80, brightest, about 20, 30", json!({"amount": 80, "center": [20, 30], "zoom": "brightest"})),
        ("amount 30, darkest, about 70, 60", json!({"amount": 30, "center": [70, 60], "zoom": "darkest"})),
    ] {
        let project = reference(|id| json!([{"instance_id": format!("b241-{id}"), "type_id": "core.fast_zoom_blur", "enabled": true, "parameters": p.clone()}]));
        let cpu = |project: &Project| {
            let mut log = FrameLog::new(3);
            preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
                .expect("the reference shot draws")
                .to_srgb8_straight()
        };
        let changed = distance(&cpu(&project), &cpu(&reference(|_| json!([])))).1;
        t.row(
            &format!("the reference shot, Fast Zoom Blur {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(&mut gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Fast Zoom Blur {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }

    t.heading("Pictures: light streaking out of a title, in `verification/D-362 pictures/`");
    let dir = effect_table::repo("verification/D-362 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], bytes).unwrap();
    write("title.png", &title());
    // How many pixels are lighter, and how many darker, than before by more than a level, in
    // any of red, green and blue.
    let moved = |bytes: &[u8], before: &[u8]| {
        let (mut lighter, mut darker) = (0, 0);
        for (p, q) in bytes.chunks_exact(4).zip(before.chunks_exact(4)) {
            lighter += (0..3).any(|k| p[k] > q[k].saturating_add(1)) as usize;
            darker += (0..3).any(|k| p[k].saturating_add(1) < q[k]) as usize;
        }
        (lighter, darker)
    };

    let (before, said) = picture(&dir, J::Array(vec![]));
    write("before.png", &before);
    t.row("before.png, the title with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());

    let (plain, said) = picture(&dir, one(json!({"amount": 50, "center": [50, 50]})));
    write("standard_50.png", &plain);
    let (l, d) = moved(&plain, &before);
    t.row(
        "standard_50.png, Fast Zoom Blur as it starts, amount 50, standard: the letters streak \
         outward, fading, and the card's blue streaks in over them, so pixels both lighten and \
         darken; draws cleanly",
        &format!("{said:?}, {l} pixels lighter, {d} darker"),
        said.is_empty() && l > 300 && d > 300,
    );

    let (bright, said) = picture(&dir, one(json!({"amount": 40, "center": [30, 40], "zoom": "brightest"})));
    write("brightest_40.png", &bright);
    let (l, d) = moved(&bright, &before);
    t.row(
        "brightest_40.png, amount 40, brightest, about a light up and left of the title: light \
         rays thrown down and right from every letter over the card, and no pixel darker; draws \
         cleanly",
        &format!("{said:?}, {l} pixels lighter, {d} darker"),
        said.is_empty() && l > 300 && d == 0,
    );

    let (dark, said) = picture(&dir, one(json!({"amount": 40, "center": [30, 40], "zoom": "darkest"})));
    write("darkest_40.png", &dark);
    let (l, d) = moved(&dark, &before);
    t.row(
        "darkest_40.png, amount 40, darkest: the card's blue eats into the letters' edges and \
         slits, away from the centre, and no pixel lighter; draws cleanly",
        &format!("{said:?}, {l} pixels lighter, {d} darker"),
        said.is_empty() && d > 300 && l == 0,
    );

    t.finish("D-362_fast_zoom_blur_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-237's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B241_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-241: a measurement, run deliberately with --release --ignored"]
fn b241_fast_zoom_blur_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B241_CPU").is_ok();
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
        ("Noise, then Fast Zoom Blur as added (amount 50, standard)", Some(json!({"amount": 50, "center": [50, 50]}))),
        ("Noise, then Fast Zoom Blur, amount 50, brightest", Some(json!({"amount": 50, "center": [50, 50], "zoom": "brightest"}))),
        ("Noise, then Fast Zoom Blur, amount 100, darkest, about 20, 30", Some(json!({"amount": 100, "center": [20, 30], "zoom": "darkest"}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true, "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(json!({"instance_id": format!("{id}b"), "type_id": "core.fast_zoom_blur", "enabled": true, "parameters": p.clone()}));
            }
            J::Array(v)
        });
        let (comp, root) = (Id::new("comp-reference-shot"), effect_table::repo("Fixtures/reference_shot"));
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
    let out = std::env::var("B241_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-241_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
