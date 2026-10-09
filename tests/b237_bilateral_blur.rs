//! B-237: D-358, Bilateral Blur, after After Effects' Bilateral Blur: smoothing that keeps strong
//! edges ("Blur & Sharpen | Bilateral Blur" in `docs/effects/EFFECTS.md`).
//!
//! Writes `verification/D-358_bilateral_blur_table.md`.
//!
//! Every expected pixel is `Fixtures/bilateral_blur/expected_bilateral_blur.json`, written by
//! `tools/bilateral_blur_reference.py` before this code existed and printed in document 25 as
//! FX-BILAT-001 to 017. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws a face with grainy skin, specks and pinholes, B-138's, and smooths it into
//! `verification/D-358 pictures/`, three times enlarged, over a grey check where it is clear.

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

fn bilateral(radius: f64, threshold: f64, colorize: &str) -> Effect {
    Effect::BilateralBlur { radius, threshold, colorize: colorize.to_string() }
}

const PLATE: (usize, usize) = (120, 80);
const MIDDLE: (f64, f64) = (60.0, 42.0);
const SKIN: [u8; 3] = [246, 214, 190];
const LINE: [u8; 3] = [30, 26, 36];

/// What the face's pixel is.
#[derive(Clone, Copy, PartialEq)]
enum Part {
    Clear,
    Line,
    Hair,
    Skin,
    Speck,
}

fn from_middle((x, y): (usize, usize)) -> f64 {
    (x as f64 + 0.5 - MIDDLE.0).hypot(y as f64 + 0.5 - MIDDLE.1)
}

/// B-138's drawn face: skin with grain of ten levels either way, a line three pixels wide round
/// it, two eyes, a one-pixel strand of hair, and sixty specks of white, of line or of nothing.
fn face() -> (Vec<u8>, Vec<Part>) {
    let mut seed = 138u32;
    let mut rnd = |n: u32| {
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        (seed >> 8) % n
    };
    let (w, h) = PLATE;
    let mut parts = Vec::new();
    let mut bytes = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let d = from_middle((x, y));
            let eye = |ex: f64| (x as f64 + 0.5 - ex).hypot(y as f64 + 0.5 - 38.0) < 4.0;
            let (part, c) = if d > 34.0 {
                (Part::Clear, [0, 0, 0, 0])
            } else if d > 31.0 || eye(48.0) || eye(72.0) {
                (Part::Line, [LINE[0], LINE[1], LINE[2], 255])
            } else if y == 14 + (x.saturating_sub(40)) / 3 && (40..82).contains(&x) {
                (Part::Hair, [LINE[0], LINE[1], LINE[2], 255])
            } else {
                let g = rnd(21) as i32 - 10;
                let c = |v: u8| (v as i32 + g).clamp(0, 255) as u8;
                (Part::Skin, [c(SKIN[0]), c(SKIN[1]), c(SKIN[2]), 255])
            };
            parts.push(part);
            bytes.extend(c);
        }
    }
    let mut placed = 0;
    while placed < 60 {
        let i = (rnd(h as u32) as usize) * w + rnd(w as u32) as usize;
        if parts[i] == Part::Skin && from_middle((i % w, i / w)) < 27.0 {
            parts[i] = Part::Speck;
            let c = [[255, 255, 255, 255], [LINE[0], LINE[1], LINE[2], 255], [0, 0, 0, 0]][rnd(3) as usize];
            bytes[4 * i..4 * i + 4].copy_from_slice(&c);
            placed += 1;
        }
    }
    (bytes, parts)
}

/// `plate` as the composition's one layer, with `effects`, drawn, straight 8-bit; and the same
/// enlarged three times over a grey check where it is clear.
fn picture(dir: &Path, plate: &str, effects: J) -> (Vec<u8>, Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/bilateral_blur/fx_bilat_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from(plate);
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
    let bytes = frame.to_srgb8_straight();
    let big: Vec<u8> = (0..PLATE.1 * 3)
        .flat_map(|y| (0..PLATE.0 * 3).map(move |x| (x, y)))
        .flat_map(|(x, y)| {
            let i = (y / 3 * PLATE.0 + x / 3) * 4;
            let check = if (x / 12 + y / 12) % 2 == 0 { 96.0 } else { 128.0 };
            let a = bytes[i + 3] as f64 / 255.0;
            let over = |c: u8| (c as f64 * a + check * (1.0 - a)).round() as u8;
            [over(bytes[i]), over(bytes[i + 1]), over(bytes[i + 2]), 255]
        })
        .collect();
    (bytes, big, said)
}

/// Bilateral Blur as a layer's `effects`.
fn one(radius: f64, threshold: f64, colorize: &str) -> J {
    json!([{ "instance_id": "fx-0-0", "type_id": "core.bilateral_blur", "enabled": true,
        "parameters": {"radius": radius, "threshold": threshold, "colorize": colorize} }])
}

fn distance(a: &[u8], b: &[u8]) -> (u8, usize) {
    assert_eq!(a.len(), b.len(), "the two pictures are different sizes");
    let (mut largest, mut pixels) = (0, 0);
    for (p, q) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
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

/// The Bilateral Blurs the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c, render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::BilateralBlur { .. })))
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
fn b237_bilateral_blur() {
    let mut t = Table::new(
        "bilateral_blur",
        "# D-358: Bilateral Blur\n\nB-237, after After Effects' Bilateral Blur: each pixel mixes \
         with the pixels round it, weighed by how near they are and how alike, so grain and flat \
         colour are smoothed and strong edges kept. Every expected pixel is \
         `Fixtures/bilateral_blur/expected_bilateral_blur.json`, written by \
         `tools/bilateral_blur_reference.py` before this code existed and printed in document 25 \
         as FX-BILAT-001 to 017. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-BILAT-001 to 017 (document 25)");
    t.fixtures("expected_bilateral_blur.json");

    t.heading("How far it reaches");
    let got = bilateral(50.0, 255.0, "on").bounds_expansion();
    t.row("it never grows the layer: it declares no growth", &format!("{got:?}"), got == 0);
    let mut draft = bilateral(5.0, 20.0, "off");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the radius, 5 to 2.5, and keeps the threshold, which is \
         colour, not distance, and the word",
        &format!("{draft:?}"),
        draft == bilateral(2.5, 20.0, "off"),
    );

    t.heading("The file");
    let files: Vec<String> = (1..=17).map(|n| format!("fx_bilat_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let effect_of = |d: &anime_compositor::command::Document| {
        d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone()
    };
    let why = effect_of(&t.load("fx_bilat_017.json").document).why_invalid();
    t.row("fx_bilat_017.json is refused in a sentence", &why, why == "Bilateral Blur's colorize is \"off\" or \"on\", and this is \"sometimes\".");
    t.shape_refused("fx_bilat_001.json", "a Bilateral Blur with no `colorize`", r#"{"radius": 5, "threshold": 20}"#);
    t.shape_refused("fx_bilat_001.json", "a Bilateral Blur with no `threshold`", r#"{"radius": 5, "colorize": "on"}"#);
    t.shape_refused("fx_bilat_001.json", "a Bilateral Blur whose colorize is a number", r#"{"radius": 5, "threshold": 20, "colorize": 1}"#);

    t.heading("Commands");
    let mut document = t.load("fx_bilat_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("radius 50.5", set(bilateral(50.5, 20.0, "on"))),
            ("threshold 255.5", set(bilateral(5.0, 255.5, "on"))),
            ("colorize \"On\", written with a capital", set(bilateral(5.0, 20.0, "On"))),
            ("radius keyed to 60", keys("radius", &[(0, &[0.0]), (4, &[60.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_bilat_001.json",
        vec![
            ("radius 50, threshold 255, colorize off,", set(bilateral(50.0, 255.0, "off"))),
            ("threshold keyed from 0 to 40", keys("threshold", &[(0, &[0.0]), (4, &[40.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_bilat_001.json", 0), ("fx_bilat_004.json", 0), ("fx_bilat_008.json", 2), ("fx_bilat_011.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    let fixture_root = effect_table::repo("Fixtures/bilateral_blur");
    for n in 1..=17 {
        let file = format!("fx_bilat_{n:03}.json");
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
        // FX-BILAT-002 and 003 leave the layer as it is, so the card is not asked; 012 to 017 are
        // left out with a warning.
        let none = n == 2 || n == 3 || n >= 12;
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none,
        );
    }
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = effect_table::repo("Fixtures/reference_shot");
    for (what, p) in [
        ("as added (Radius 5, Threshold 20, Colorize on)", json!({"radius": 5, "threshold": 20, "colorize": "on"})),
        ("Radius 8, Threshold 30, Colorize off", json!({"radius": 8, "threshold": 30, "colorize": "off"})),
        ("Radius 16, Threshold 60, Colorize on", json!({"radius": 16, "threshold": 60, "colorize": "on"})),
    ] {
        let project = reference(|id| json!([{"instance_id": format!("b237-{id}"), "type_id": "core.bilateral_blur", "enabled": true, "parameters": p.clone()}]));
        let cpu = |project: &Project| {
            let mut log = FrameLog::new(3);
            preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
                .expect("the reference shot draws")
                .to_srgb8_straight()
        };
        let changed = distance(&cpu(&project), &cpu(&reference(|_| json!([])))).1;
        t.row(
            &format!("the reference shot, Bilateral Blur {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(&mut gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Bilateral Blur {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }

    t.heading("Pictures: a scanned face smoothed, in `verification/D-358 pictures/`, three times enlarged");
    let dir = effect_table::repo("verification/D-358 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let write = |name: &str, bytes: &[u8], scale: usize| {
        png_out::write_rgba(&dir.join(name), w * scale, h * scale, OutputDepth::Eight, &[], bytes).unwrap()
    };
    let (plate, parts) = face();
    write("face.png", &plate, 1);
    let px = |bytes: &[u8], i: usize| [bytes[4 * i], bytes[4 * i + 1], bytes[4 * i + 2], bytes[4 * i + 3]];
    // How much the skin changes from one pixel to the next, across grain only.
    let grain = |bytes: &[u8]| {
        let pairs: Vec<usize> = (0..w * h - 1).filter(|&i| parts[i] == Part::Skin && parts[i + 1] == Part::Skin).collect();
        let d: u32 = pairs.iter().map(|&i| (0..3).map(|k| bytes[4 * i + k].abs_diff(bytes[4 * i + 4 + k]) as u32).sum::<u32>()).sum();
        d as f64 / pairs.len() as f64
    };
    let lines: Vec<usize> = (0..w * h).filter(|&i| matches!(parts[i], Part::Line | Part::Hair)).collect();
    // The line's and the hair's largest change in any channel.
    let line_moved = |bytes: &[u8], before: &[u8]| {
        lines.iter().map(|&i| (0..4).map(|k| bytes[4 * i + k].abs_diff(before[4 * i + k])).max().unwrap()).max().unwrap()
    };
    let covering_kept = |bytes: &[u8], before: &[u8]| (0..w * h).all(|i| bytes[4 * i + 3] == before[4 * i + 3]);

    let (before, big, said) = picture(&dir, "face.png", J::Array(vec![]));
    write("before.png", &big, 3);
    t.row(
        "before.png, the face with no effect: grainy skin, specks, pinholes; draws cleanly",
        &format!("{said:?}, grain {:.1}", grain(&before)),
        said.is_empty() && grain(&before) > 10.0,
    );

    let (smooth, big, said) = picture(&dir, "face.png", one(5.0, 20.0, "on"));
    write("bilateral_default.png", &big, 3);
    t.row(
        "bilateral_default.png, Bilateral Blur as it starts, Radius 5, Threshold 20, Colorize on: \
         the grain less than half what it was, the line, the eyes and the one-pixel hair kept to \
         within a level, as the skin is far too unlike them to be mixed in, and every pixel's \
         covering as it was, pinholes still clear; draws cleanly",
        &format!("{said:?}, grain {:.1} from {:.1}, line moved at most {} levels", grain(&smooth), grain(&before), line_moved(&smooth, &before)),
        said.is_empty() && grain(&smooth) * 2.0 < grain(&before) && line_moved(&smooth, &before) <= 1 && covering_kept(&smooth, &before),
    );

    let (strong, big, said) = picture(&dir, "face.png", one(12.0, 30.0, "on"));
    write("bilateral_12_30.png", &big, 3);
    t.row(
        "bilateral_12_30.png, Radius 12, Threshold 30: smoother skin than at the defaults, while \
         the line, 150 to 220 levels from the skin, still stays within 2 levels; draws cleanly",
        &format!("{said:?}, grain {:.1}, line moved at most {} levels", grain(&strong), line_moved(&strong, &before)),
        said.is_empty() && grain(&strong) < grain(&smooth) && line_moved(&strong, &before) <= 2 && covering_kept(&strong, &before),
    );

    let (soft, big, said) = picture(&dir, "face.png", one(12.0, 60.0, "on"));
    write("bilateral_12_60.png", &big, 3);
    let hair: Vec<usize> = (0..w * h).filter(|&i| parts[i] == Part::Hair && from_middle((i % w, i / w)) < 28.0).collect();
    let hair_moved = hair.iter().map(|&i| (0..3).map(|k| soft[4 * i + k].abs_diff(before[4 * i + k])).max().unwrap()).max().unwrap();
    t.row(
        "bilateral_12_60.png, Radius 12, Threshold 60: so high a threshold lets a little of the \
         skin into the line, and the one-pixel hair, outnumbered by the skin round it, is lightened \
         by more than 20 levels: the edge-keeping gives way as Threshold rises; draws cleanly",
        &format!("{said:?}, grain {:.1}, hair lightened by up to {hair_moved} levels", grain(&soft)),
        said.is_empty() && hair_moved > 20 && covering_kept(&soft, &before),
    );

    let (grey, big, said) = picture(&dir, "face.png", one(5.0, 20.0, "off"));
    write("bilateral_colorize_off.png", &big, 3);
    let greyed = (0..w * h).filter(|&i| px(&grey, i)[3] > 0).all(|i| {
        let p = px(&grey, i);
        p[0] == p[1] && p[1] == p[2]
    });
    t.row(
        "bilateral_colorize_off.png, Colorize off: the same smoothing worked on brightness alone, \
         and every pixel that shows grey, red, green and blue equal; covering as it was; draws \
         cleanly",
        &format!("{said:?}, every pixel grey: {greyed}"),
        said.is_empty() && greyed && covering_kept(&grey, &before),
    );

    t.finish("D-358_bilateral_blur_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-235's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole, with Draw on: GPU. The first loop starts
/// with empty caches and its 30 frames' median is "first"; then the loops after it are timed and
/// the median of their frames is "again": seven loops on the card, two on the processor, which is
/// far slower. With `B237_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-237: a measurement, run deliberately with --release --ignored"]
fn b237_bilateral_blur_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B237_CPU").is_ok();
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
        ("Noise, then Bilateral Blur as added (Radius 5, Threshold 20, Colorize on)", Some(json!({"radius": 5, "threshold": 20, "colorize": "on"}))),
        ("Noise, then Bilateral Blur, Radius 5, Threshold 20, Colorize off", Some(json!({"radius": 5, "threshold": 20, "colorize": "off"}))),
        ("Noise, then Bilateral Blur, Radius 10, Threshold 40, Colorize on", Some(json!({"radius": 10, "threshold": 40, "colorize": "on"}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true, "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(json!({"instance_id": format!("{id}b"), "type_id": "core.bilateral_blur", "enabled": true, "parameters": p.clone()}));
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
    let out = std::env::var("B237_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-237_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
