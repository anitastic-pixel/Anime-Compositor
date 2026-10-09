//! B-240: D-361, Spin & Zoom Blur, after CycoreFX's CC Radial Blur: six ways of blurring along
//! circles or lines through a centre ("Blur & Sharpen | CC Radial Blur" in
//! `docs/effects/EFFECTS.md`).
//!
//! Writes `verification/D-361_spin_zoom_blur_table.md`.
//!
//! Every expected pixel is `Fixtures/spin_zoom_blur/expected_spin_zoom_blur.json`, written by
//! `tools/spin_zoom_blur_reference.py` before this code existed and printed in document 25 as
//! FX-SPINZOOM-001 to 022. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws a car wheel and spins and zooms it into `verification/D-361 pictures/`.

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

fn spin_zoom(kind: &str, amount: f64, quality: f64, center: [f64; 2]) -> Effect {
    Effect::SpinZoomBlur { kind: kind.into(), amount, quality, center }
}

const PLATE: (usize, usize) = (192, 108);
const HUB: (f64, f64) = (96.0, 54.0);

/// A car wheel on a pale road: a dark tyre from 36 to 46 pixels out, a silver rim inside it with
/// five dark spokes 4 pixels wide, and a dark hub 6 pixels across.
fn wheel() -> Vec<u8> {
    let (w, h) = PLATE;
    (0..w * h)
        .flat_map(|i| {
            let (x, y) = ((i % w) as f64 + 0.5 - HUB.0, (i / w) as f64 + 0.5 - HUB.1);
            let r = x.hypot(y);
            let spoke = (0..5).any(|k| {
                let a = k as f64 * std::f64::consts::TAU / 5.0;
                let (s, c) = a.sin_cos();
                (x * c + y * s) > 0.0 && (x * s - y * c).abs() < 2.0
            });
            if r > 46.0 {
                [200, 196, 186, 255]
            } else if r > 36.0 {
                [28, 28, 32, 255]
            } else if r < 6.0 || spoke {
                [50, 52, 60, 255]
            } else {
                [210, 214, 224, 255]
            }
        })
        .collect()
}

/// `effects` on the wheel, drawn, straight 8-bit.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J =
        serde_json::from_str(&fs::read_to_string(effect_table::repo("Fixtures/spin_zoom_blur/fx_spinzoom_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("wheel.png");
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
    json!([{ "instance_id": "fx-0-0", "type_id": "core.spin_zoom_blur", "enabled": true, "parameters": p }])
}

/// The Spin & Zoom Blurs the card's plan leaves to the card, over every layer.
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
fn b240_spin_zoom_blur() {
    let mut t = Table::new(
        "spin_zoom_blur",
        "# D-361: Spin & Zoom Blur\n\nB-240, after CycoreFX's CC Radial Blur: each pixel averaged \
         along a circle round a centre or a line through it, six ways: Straight, Fading and \
         Centered Zoom, Rotate, Scratch and Rotate Fading, with a Quality setting for how many \
         points it reads. Every expected pixel is \
         `Fixtures/spin_zoom_blur/expected_spin_zoom_blur.json`, written by \
         `tools/spin_zoom_blur_reference.py` before this code existed and printed in document 25 \
         as FX-SPINZOOM-001 to 022. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-SPINZOOM-001 to 022 (document 25)");
    t.fixtures("expected_spin_zoom_blur.json");

    t.heading("How far it reaches");
    let got = spin_zoom("rotate", 360.0, 100.0, [-1000.0, -1000.0]).bounds_expansion();
    t.row("it never grows the layer: it declares no growth, as Radial Blur", &format!("{got:?}"), got == 0);
    let mut draft = spin_zoom("centered_zoom", 30.0, 80.0, [40.0, 60.0]);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview keeps every setting: the amount is degrees or a share of the \
         distance, the centre a share of the size, and the quality points a pixel, so the paths \
         halve with the picture",
        &format!("{draft:?}"),
        draft == spin_zoom("centered_zoom", 30.0, 80.0, [40.0, 60.0]),
    );

    t.heading("The file");
    let files: Vec<String> = (1..=22).map(|n| format!("fx_spinzoom_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_spinzoom_001.json");
    t.row(
        "fx_spinzoom_001.json, which leaves out the quality, is saved without it: 50 is the default",
        &saved.to_string(),
        saved.get("quality").is_none() && saved["type"] == "straight_zoom",
    );
    let effect_of = |d: &anime_compositor::command::Document| {
        d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone()
    };
    let why = effect_of(&t.load("fx_spinzoom_020.json").document).why_invalid();
    t.row(
        "fx_spinzoom_020.json is refused in a sentence",
        &why,
        why == "Spin & Zoom Blur's type is \"straight_zoom\", \"fading_zoom\", \"centered_zoom\", \"rotate\", \"scratch\" or \"rotate_fading\", and this is \"spin\".",
    );
    t.shape_refused("fx_spinzoom_001.json", "a Spin & Zoom Blur with no `type`", r#"{"amount": 30, "center": [50, 50]}"#);
    t.shape_refused("fx_spinzoom_001.json", "a Spin & Zoom Blur whose centre is one number", r#"{"type": "rotate", "amount": 30, "center": 50}"#);

    t.heading("Commands");
    let mut document = t.load("fx_spinzoom_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("amount 360.5", set(spin_zoom("rotate", 360.5, 50.0, [50.0, 50.0]))),
            ("quality 0.5", set(spin_zoom("rotate", 30.0, 0.5, [50.0, 50.0]))),
            ("type \"Scratch\", written with a capital", set(spin_zoom("Scratch", 30.0, 50.0, [50.0, 50.0]))),
            ("centre 1000.5, 50", set(spin_zoom("rotate", 30.0, 50.0, [1000.5, 50.0]))),
            ("amount keyed to 400", keys("amount", &[(0, &[0.0]), (4, &[400.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_spinzoom_001.json",
        vec![
            ("type rotate, amount 45, quality 80, centre 40, 60,", set(spin_zoom("rotate", 45.0, 80.0, [40.0, 60.0]))),
            ("centre keyed from 0, 50 to 100, 50", keys("center", &[(0, &[0.0, 50.0]), (4, &[100.0, 50.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_spinzoom_001.json", 0), ("fx_spinzoom_003.json", 0), ("fx_spinzoom_007.json", 0), ("fx_spinzoom_011.json", 2), ("fx_spinzoom_014.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    let fixture_root = effect_table::repo("Fixtures/spin_zoom_blur");
    for n in 1..=22 {
        let file = format!("fx_spinzoom_{n:03}.json");
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
        // FX-SPINZOOM-016 to 022 are left out with a warning.
        let none = n >= 16;
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none,
        );
    }
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = effect_table::repo("Fixtures/reference_shot");
    for (what, p) in [
        ("as added (Straight Zoom 10, quality 50, about the middle)", json!({"type": "straight_zoom", "amount": 10, "center": [50, 50]})),
        ("Rotate 30, quality 80, about 30, 40", json!({"type": "rotate", "amount": 30, "quality": 80, "center": [30, 40]})),
        ("Fading Zoom -40, quality 20", json!({"type": "fading_zoom", "amount": -40, "quality": 20, "center": [50, 50]})),
    ] {
        let project = reference(|id| json!([{"instance_id": format!("b240-{id}"), "type_id": "core.spin_zoom_blur", "enabled": true, "parameters": p.clone()}]));
        let cpu = |project: &Project| {
            let mut log = FrameLog::new(3);
            preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
                .expect("the reference shot draws")
                .to_srgb8_straight()
        };
        let changed = distance(&cpu(&project), &cpu(&reference(|_| json!([])))).1;
        t.row(
            &format!("the reference shot, Spin & Zoom Blur {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(&mut gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Spin & Zoom Blur {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }

    t.heading("Pictures: a car wheel spun and zoomed, in `verification/D-361 pictures/`");
    let dir = effect_table::repo("verification/D-361 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], bytes).unwrap();
    write("wheel.png", &wheel());
    // The red of the pixel `r` out from the hub at `degrees`, measured clockwise from the right;
    // a spoke points right, the next 72 degrees round.
    let at = |bytes: &[u8], r: f64, degrees: f64| {
        let (s, c) = degrees.to_radians().sin_cos();
        let (x, y) = ((HUB.0 + r * c) as usize, (HUB.1 + r * s) as usize);
        bytes[(y * w + x) * 4]
    };
    // Pixels inside the tyre that moved by more than 10 levels.
    let smeared = |bytes: &[u8], before: &[u8]| {
        (0..w * h)
            .filter(|&i| ((i % w) as f64 + 0.5 - HUB.0).hypot((i / w) as f64 + 0.5 - HUB.1) < 34.0)
            .filter(|&i| bytes[4 * i].abs_diff(before[4 * i]) > 10)
            .count()
    };

    let (before, said) = picture(&dir, J::Array(vec![]));
    write("before.png", &before);
    t.row("before.png, the wheel with no effect; draws cleanly", &format!("{said:?}, {} pixels smeared", smeared(&before, &before)), said.is_empty());

    let (spun, said) = picture(&dir, one(json!({"type": "rotate", "amount": 20, "center": [50, 50]})));
    write("rotate_20.png", &spun);
    t.row(
        "rotate_20.png, Rotate 20 about the middle, a wheel turning: the spokes smeared round into \
         a blur, the hub, where every path is nothing, and the tyre, the same all the way round, \
         as they were; draws cleanly",
        &format!(
            "{said:?}, {} pixels inside the tyre smeared; hub {} was {}, tyre {} was {}",
            smeared(&spun, &before),
            at(&spun, 0.0, 0.0),
            at(&before, 0.0, 0.0),
            at(&spun, 41.0, 36.0),
            at(&before, 41.0, 36.0)
        ),
        said.is_empty()
            && smeared(&spun, &before) > 400
            && at(&spun, 0.0, 0.0).abs_diff(at(&before, 0.0, 0.0)) <= 1
            && at(&spun, 41.0, 36.0).abs_diff(at(&before, 41.0, 36.0)) <= 2,
    );

    let (zoomed, said) = picture(&dir, one(json!({"type": "straight_zoom", "amount": 30, "center": [50, 50]})));
    write("straight_zoom_30.png", &zoomed);
    let (edge, edge_was) = (at(&zoomed, 37.0, 36.0), at(&before, 37.0, 36.0));
    t.row(
        "straight_zoom_30.png, Straight Zoom 30, rushing toward the wheel: the tyre's inner edge \
         smeared inward, while a spoke, lying along the streaks, stays as it was 20 pixels out; \
         draws cleanly",
        &format!("{said:?}, the tyre's inner edge {edge} was {edge_was}; spoke {} was {}", at(&zoomed, 20.0, 0.0), at(&before, 20.0, 0.0)),
        said.is_empty() && edge.abs_diff(edge_was) > 20 && at(&zoomed, 20.0, 0.0).abs_diff(at(&before, 20.0, 0.0)) <= 2,
    );

    let (fading, said) = picture(&dir, one(json!({"type": "fading_zoom", "amount": 30, "center": [50, 50]})));
    write("fading_zoom_30.png", &fading);
    let faded = at(&fading, 37.0, 36.0);
    t.row(
        "fading_zoom_30.png, Fading Zoom 30: the same streaks fading as they run, so the tyre's \
         inner edge moves less than with Straight Zoom; draws cleanly",
        &format!("{said:?}, the tyre's inner edge {faded}, Straight Zoom's {edge}, before {edge_was}"),
        said.is_empty() && faded.abs_diff(edge_was) < edge.abs_diff(edge_was) && faded != edge_was,
    );

    let (scratch, said) = picture(&dir, one(json!({"type": "scratch", "amount": 20, "center": [50, 50]})));
    write("scratch_20.png", &scratch);
    t.row(
        "scratch_20.png, Scratch 20: the turn spread both ways from each pixel, as Radial Blur's \
         spin; the spokes smear as with Rotate, but stay centred where they were; draws cleanly",
        &format!("{said:?}, {} pixels inside the tyre smeared", smeared(&scratch, &before)),
        said.is_empty() && smeared(&scratch, &before) > 400,
    );

    t.finish("D-361_spin_zoom_blur_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-237's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B240_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-240: a measurement, run deliberately with --release --ignored"]
fn b240_spin_zoom_blur_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B240_CPU").is_ok();
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
        ("Noise, then Spin & Zoom Blur as added (Straight Zoom 10, quality 50)", Some(json!({"type": "straight_zoom", "amount": 10, "center": [50, 50]}))),
        ("Noise, then Spin & Zoom Blur, Rotate 30, quality 50", Some(json!({"type": "rotate", "amount": 30, "center": [50, 50]}))),
        ("Noise, then Spin & Zoom Blur, Centered Zoom 60, quality 100", Some(json!({"type": "centered_zoom", "amount": 60, "quality": 100, "center": [50, 50]}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true, "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(json!({"instance_id": format!("{id}b"), "type_id": "core.spin_zoom_blur", "enabled": true, "parameters": p.clone()}));
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
    let out = std::env::var("B240_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-240_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
