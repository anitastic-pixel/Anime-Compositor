//! B-239: D-360, Cross Blur, after CycoreFX's CC Cross Blur: the layer blurred across and, apart,
//! down, the two laid together ("Blur & Sharpen | CC Cross Blur" in `docs/effects/EFFECTS.md`).
//!
//! Writes `verification/D-360_cross_blur_table.md`.
//!
//! Every expected pixel is `Fixtures/cross_blur/expected_cross_blur.json`, written by
//! `tools/cross_blur_reference.py` before this code existed and printed in document 25 as
//! FX-CROSS-001 to 018. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws a night street with lit windows and stars and crosses it into
//! `verification/D-360 pictures/`.

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

fn cross(radius_x: f64, radius_y: f64, mode: &str, edges: &str) -> Effect {
    Effect::CrossBlur { radius_x, radius_y, mode: mode.into(), edges: edges.into() }
}

const PLATE: (usize, usize) = (192, 108);
const STARS: [(usize, usize); 5] = [(30, 14), (70, 24), (120, 10), (160, 30), (96, 40)];

/// A night street: a dark blue sky, five white stars of 3 by 3 pixels, a row of dark houses
/// with warm lit windows of 4 by 4.
fn street() -> Vec<u8> {
    let (w, h) = PLATE;
    (0..w * h)
        .flat_map(|i| {
            let (x, y) = (i % w, i / w);
            let star = STARS.iter().any(|&(sx, sy)| x.abs_diff(sx) <= 1 && y.abs_diff(sy) <= 1);
            let window = y >= 66 && y < 94 && (x % 16) >= 6 && (x % 16) < 10 && (y % 12) >= 2 && (y % 12) < 6;
            if star {
                [255, 255, 255, 255]
            } else if window {
                [255, 214, 140, 255]
            } else if y >= 60 {
                [34, 30, 44, 255]
            } else {
                [16, 24, 60, 255]
            }
        })
        .collect()
}

/// `effects` on the street, drawn, straight 8-bit.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J =
        serde_json::from_str(&fs::read_to_string(effect_table::repo("Fixtures/cross_blur/fx_cross_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("street.png");
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
    json!([{ "instance_id": "fx-0-0", "type_id": "core.cross_blur", "enabled": true, "parameters": p }])
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

/// The Cross Blurs the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::CrossBlur { .. })))
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
    let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        layers[i]["effects"] = stack(id);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

#[test]
fn b239_cross_blur() {
    let mut t = Table::new(
        "cross_blur",
        "# D-360: Cross Blur\n\nB-239, after CycoreFX's CC Cross Blur: the layer blurred across by \
         one box of Fast Box Blur and, apart, down by another, the two laid together by a transfer \
         mode, so a bright point becomes a soft cross. Every expected pixel is \
         `Fixtures/cross_blur/expected_cross_blur.json`, written by `tools/cross_blur_reference.py` \
         before this code existed and printed in document 25 as FX-CROSS-001 to 018. The build's \
         frame is compared sample by sample; the answer is the largest difference over all of \
         them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-CROSS-001 to 018 (document 25)");
    t.fixtures("expected_cross_blur.json");

    t.heading("How far it reaches");
    for (e, want) in [
        (cross(4.0, 4.0, "blend", "transparent"), 4),
        (cross(2.5, 1.0, "blend", "transparent"), 3),
        (cross(0.0, 9.0, "add", "transparent"), 9),
        (cross(40.0, 4.0, "blend", "repeat"), 0),
    ] {
        let got = e.bounds_expansion();
        t.row(&format!("{e:?} grows the layer by the larger reach, none with edges repeat: {want}"), &format!("{got}"), got == want);
    }
    let mut draft = cross(20.0, 6.0, "add", "repeat");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves both radii, 20 and 6 to 10 and 3, and keeps the words",
        &format!("{draft:?}"),
        draft == cross(10.0, 3.0, "add", "repeat"),
    );

    t.heading("The file");
    let files: Vec<String> = (1..=18).map(|n| format!("fx_cross_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_cross_001.json");
    t.row(
        "fx_cross_001.json, which leaves out the mode and the edges, is saved without them: blend and transparent are the defaults",
        &saved.to_string(),
        saved.get("mode").is_none() && saved.get("edges").is_none(),
    );
    let effect_of = |d: &anime_compositor::command::Document| {
        d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone()
    };
    let why = effect_of(&t.load("fx_cross_016.json").document).why_invalid();
    t.row(
        "fx_cross_016.json is refused in a sentence",
        &why,
        why == "Cross Blur's mode is \"blend\", \"add\", \"screen\", \"multiply\", \"lighten\" or \"darken\", and this is \"overlay\".",
    );
    t.shape_refused("fx_cross_001.json", "a Cross Blur with no `radius_y`", r#"{"radius_x": 4}"#);
    t.shape_refused("fx_cross_001.json", "a Cross Blur whose mode is a number", r#"{"radius_x": 4, "radius_y": 4, "mode": 1}"#);

    t.heading("Commands");
    let mut document = t.load("fx_cross_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("radius X 500.5", set(cross(500.5, 4.0, "blend", "transparent"))),
            ("radius Y -0.5", set(cross(4.0, -0.5, "blend", "transparent"))),
            ("mode \"Screen\", written with a capital", set(cross(4.0, 4.0, "Screen", "transparent"))),
            ("edges \"clamp\"", set(cross(4.0, 4.0, "blend", "clamp"))),
            ("radius X keyed to 600", keys("radius_x", &[(0, &[0.0]), (4, &[600.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_cross_001.json",
        vec![
            ("radius X 20, radius Y 6, mode add, edges repeat,", set(cross(20.0, 6.0, "add", "repeat"))),
            ("radius Y keyed from 0 to 8", keys("radius_y", &[(0, &[0.0]), (4, &[8.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_cross_001.json", 0), ("fx_cross_003.json", 0), ("fx_cross_008.json", 0), ("fx_cross_011.json", 2), ("fx_cross_012.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    let fixture_root = effect_table::repo("Fixtures/cross_blur");
    for n in 1..=18 {
        let file = format!("fx_cross_{n:03}.json");
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
        // FX-CROSS-010 leaves the layer as it is, so the card is not asked; FX-CROSS-011's frame 0
        // at Draft and Full blurs down only, still a cross; 014 to 018 are left out with a warning.
        let none = n == 10 || n >= 14;
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none,
        );
    }
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = effect_table::repo("Fixtures/reference_shot");
    for (what, p) in [
        ("as added (Radius X 10, Radius Y 10, blend)", json!({"radius_x": 10, "radius_y": 10})),
        ("Radius X 40, Radius Y 4, add", json!({"radius_x": 40, "radius_y": 4, "mode": "add"})),
        ("Radius X 6, Radius Y 30, darken, edges repeat", json!({"radius_x": 6, "radius_y": 30, "mode": "darken", "edges": "repeat"})),
    ] {
        let project = reference(|id| json!([{"instance_id": format!("b239-{id}"), "type_id": "core.cross_blur", "enabled": true, "parameters": p.clone()}]));
        let cpu = |project: &Project| {
            let mut log = FrameLog::new(3);
            preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
                .expect("the reference shot draws")
                .to_srgb8_straight()
        };
        let changed = distance(&cpu(&project), &cpu(&reference(|_| json!([])))).1;
        t.row(
            &format!("the reference shot, Cross Blur {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(&mut gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Cross Blur {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }

    t.heading("Pictures: a night street crossed, in `verification/D-360 pictures/`");
    let dir = effect_table::repo("verification/D-360 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], bytes).unwrap();
    write("street.png", &street());
    let at = |bytes: &[u8], x: usize, y: usize| bytes[(y * w + x) * 4];
    let (sx, sy) = STARS[4];

    let (before, said) = picture(&dir, J::Array(vec![]));
    write("before.png", &before);
    t.row("before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());

    let (soft, said) = picture(&dir, one(json!({"radius_x": 10, "radius_y": 10})));
    write("cross_as_added.png", &soft);
    let (arm, corner, sky) = (at(&soft, sx + 6, sy), at(&soft, sx + 6, sy + 6), at(&before, sx + 6, sy + 6));
    t.row(
        "cross_as_added.png, Cross Blur as it starts, Radius X 10, Radius Y 10, blend: each star a \
         soft plus, 6 pixels right of the middle star lit above the sky, 6 right and 6 down the \
         sky as it was; draws cleanly",
        &format!("{said:?}, red 6 right {arm}, 6 right and 6 down {corner}, the sky {sky}"),
        said.is_empty() && arm > sky + 10 && corner.abs_diff(sky) <= 1,
    );

    let (streak, said) = picture(&dir, one(json!({"radius_x": 40, "radius_y": 2, "mode": "add"})));
    write("cross_streak_add.png", &streak);
    let (far, up) = (at(&streak, sx + 30, sy), at(&streak, sx + 30, sy - 6));
    t.row(
        "cross_streak_add.png, Radius X 40, Radius Y 2, add: a long flare across each star and \
         window, 30 pixels right of the middle star lit well above the pixel 6 higher, where only \
         the sky is added to itself and so is a little lighter than it was; draws cleanly",
        &format!("{said:?}, red 30 right {far}, 30 right and 6 up {up}, the sky before {}", at(&before, sx + 30, sy - 6)),
        said.is_empty() && far > up + 20 && up > at(&before, sx + 30, sy - 6),
    );

    let (dark, said) = picture(&dir, one(json!({"radius_x": 8, "radius_y": 8, "mode": "darken"})));
    write("cross_darken.png", &dark);
    let (arm, middle) = (at(&dark, sx + 6, sy), at(&dark, sx, sy));
    t.row(
        "cross_darken.png, Radius 8 both ways, darken: only where both blurs reach do the lights \
         keep any strength, so each star shrinks to a faint dot with no arms, 6 pixels right of \
         the middle star the sky as it was; draws cleanly",
        &format!("{said:?}, red at the star {middle}, 6 right {arm}, the sky {}", at(&before, sx + 6, sy)),
        said.is_empty() && arm.abs_diff(at(&before, sx + 6, sy)) <= 1 && middle > arm,
    );

    t.finish("D-360_cross_blur_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-237's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B239_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-239: a measurement, run deliberately with --release --ignored"]
fn b239_cross_blur_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B239_CPU").is_ok();
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
        ("Noise, then Cross Blur as added (Radius X 10, Radius Y 10, blend)", Some(json!({"radius_x": 10, "radius_y": 10}))),
        ("Noise, then Cross Blur, Radius X 40, Radius Y 4, add", Some(json!({"radius_x": 40, "radius_y": 4, "mode": "add"}))),
        ("Noise, then Cross Blur, Radius X 100, Radius Y 100, screen", Some(json!({"radius_x": 100, "radius_y": 100, "mode": "screen"}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true, "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(json!({"instance_id": format!("{id}b"), "type_id": "core.cross_blur", "enabled": true, "parameters": p.clone()}));
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
    let out = std::env::var("B239_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-239_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
