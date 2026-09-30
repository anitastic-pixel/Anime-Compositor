//! B-173: the card keeps the picture of each run of effects it has drawn on a drawing, not only the
//! last (D-246). The quiet re-measure of 2026-09-30 found the viewer at Draft slower since B-151 and
//! B-155 on shots whose effects move: the CPU's cache keeps each frame's effect result, so a second
//! play costs it only the composite, while the card drew a moving run anew every frame. A frame
//! asked for again, after another in between, must now run no pass on the card and give the very
//! picture the card drew the first time.
//!
//! Every effect the card draws that moves by itself with the frame, and three the card draws with a
//! keyed setting, are put on the reference shot's first three layers as B-151's check puts them
//! (the second layer's after a Drop Shadow), and B-155's moving Noise first before a run of four.
//! A Median that does not move is the control. Every run stays the card's, at Draft as at Full.
//!
//! What is compared is what the page receives, eight-bit straight sRGB: the CPU's frame through
//! `preview_frame_cached`, and the card's through `preview_frame_srgb8`. The CPU stays the
//! authority (ADR-006, D-100); the rule stays 1 level of 255 (D-165).
//!
//! Writes `verification/B-173_draft_moving_table.md`.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};
use serde_json::{json, Value};

mod common;
use common::repo;

/// D-165's tolerance, in levels of 255.
const LIMIT: u8 = 1;

struct Shot {
    name: String,
    project: Project,
    root: PathBuf,
    comp: Id,
    /// Whether an effect on each of the first three layers is set differently from one frame
    /// to the next.
    moving: [bool; 3],
}

fn fx(id: &str, type_id: &str, parameters: &Value) -> Value {
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": parameters})
}

/// A setting keyed from `a` at the first frame to `b` at the last.
fn keyed(a: f64, b: f64) -> Value {
    json!({"base": a, "keyframes": [{"frame": 0, "value": a, "interp": "linear"}, {"frame": 239, "value": b, "interp": "linear"}]})
}

fn shot(name: String, stacks: [Vec<Value>; 3], moving: [bool; 3]) -> Shot {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: Value = serde_json::from_str(&text).expect("the reference shot is JSON");
    for (i, stack) in stacks.into_iter().enumerate() {
        j["compositions"][0]["layers"][i]["effects"] = Value::Array(stack);
    }
    let loaded = persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("{name}: {}", d.message));
    Shot { name, project: loaded.document.project().clone(), root: repo("Fixtures/reference_shot"), comp: Id::new("comp-reference-shot"), moving }
}

/// The effects the card draws that move by themselves with the frame, with the settings B-155's
/// check gives them, then three the card draws with one setting keyed across the shot.
fn moving_effects() -> Vec<(&'static str, &'static str, Value)> {
    vec![
        ("Noise", "core.noise", json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"})),
        ("Exposure Flicker", "core.exposure_flicker", json!({"amount": 0.5, "hold": 3, "seed": 4})),
        ("Turbulent Displace", "core.turbulent_displace", json!({"amount": 12, "size": 50, "complexity": 3, "evolution": 30, "speed": 10, "seed": 5, "edges": "transparent"})),
        ("Fractal Noise", "core.fractal_noise", json!({"size": 80, "complexity": 5, "contrast": 130, "brightness": 5, "evolution": 0, "speed": 15, "seed": 3, "dark_color": "#102030", "light_color": "#ffe8c0", "opacity": 60, "blend": "screen"})),
        ("Ripple", "core.ripple", json!({"center": [50, 50], "amplitude": 5, "wavelength": 30, "speed": 20, "phase": 0, "fade": 0})),
        ("Wave Warp", "core.wave_warp", json!({"shape": "sine", "height": 10, "width": 40, "direction": 90, "speed": 1, "phase": 0, "edges": "transparent"})),
        ("Speed Lines", "core.speed_lines", json!({"center": [50, 50], "color": "#000000", "count": 120, "thickness": 1.5, "inner": 150, "inner_jitter": 40, "angle_jitter": 50, "seed": 0, "hold": 2, "opacity": 100})),
        ("Camera Shake", "core.camera_shake", json!({"amount": 10, "rotation": 0, "hold": 1, "seed": 0})),
        ("Rain", "core.rain", json!({"color": "#c8d8ff", "density": 30, "spacing": 24, "length": 20, "width": 1, "direction": 170, "speed": 30, "seed": 0, "opacity": 60})),
        ("Snowfall", "core.snowfall", json!({"color": "#ffffff", "density": 60, "spacing": 40, "size": 5, "depth": 40, "speed": 3, "wind": 0.5, "wiggle": 3, "period": 48, "seed": 5, "opacity": 90})),
        ("Roughen Edges", "core.roughen_edges", json!({"edge_type": "roughen_color", "edge_color": "#8a3c14", "border": 6, "size": 8, "complexity": 3, "evolution": 30, "speed": 10, "seed": 3})),
        ("Kira-kira", "core.kira_kira", json!({"threshold": 60, "spacing": 24, "density": 80, "size": 12, "angle": 15, "twinkle": 50, "period": 24, "seed": 7, "opacity": 90, "shape": "star", "color": "#fff0c0"})),
        ("Gaussian Blur, radius keyed 2 to 40", "core.gaussian_blur", json!({"sigma_px": keyed(2.0, 40.0)})),
        ("Bloom, intensity keyed 0.5 to 3", "core.bloom", json!({"threshold": 60, "radius": 10, "intensity": keyed(0.5, 3.0), "streaks": "star", "length": 60, "angle": 15})),
        ("Directional Blur, length keyed 5 to 80", "core.directional_blur", json!({"direction": 45, "length": keyed(5.0, 80.0)})),
    ]
}

fn shots() -> Vec<Shot> {
    let shadow = fx("b173-s", "core.drop_shadow", &json!({"color": "#000000", "opacity": 60, "direction": 200, "distance": 6, "softness": 4}));
    let one = |name: &str, type_id: &str, p: &Value, moving: bool| {
        shot(format!("{name} on three layers"), [vec![fx("b173-a", type_id, p)], vec![shadow.clone(), fx("b173-b", type_id, p)], vec![fx("b173-c", type_id, p)]], [moving; 3])
    };
    let mut shots: Vec<Shot> = moving_effects().iter().map(|(name, type_id, p)| one(name, type_id, p, true)).collect();
    // B-155's timing shot: the first two runs beginning with a moving Noise.
    let get = |id: &str, type_id: &str, p: Value| fx(id, type_id, &p);
    let noise = || get("b173-n", "core.noise", json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}));
    let levels = get("b173-l", "core.levels", json!({"input_black": 20, "input_white": 230, "gamma": 1.6, "output_black": 10, "output_white": 245}));
    let blur = get("b173-g", "core.gaussian_blur", json!({"sigma_px": 4}));
    let hue = get("b173-h", "core.hue_saturation", json!({"hue": 40, "saturation": 30, "lightness": -10}));
    let vignette = get("b173-v", "core.vignette", json!({"amount": 60, "color": "#201030", "size": 90, "roundness": 50, "softness": 60, "center": [45, 55]}));
    let drop = get("b173-d", "core.drop_shadow", json!({"color": "#102040", "opacity": 70, "direction": 135, "distance": 12, "softness": 9}));
    let curves = get("b173-u", "core.curves", json!({"master": [[0, 0], [128, 150], [255, 255]], "red": [[0, 0], [64, 40], [192, 215], [255, 255]], "green": [[0, 0], [255, 255]], "blue": [[0, 20], [255, 235]]}));
    let balance = get("b173-b", "core.color_balance", json!({"shadows": [-20, 10, 30], "midtones": [15, -5, -10], "highlights": [10, 5, -25]}));
    let outline = get("b173-o", "core.outline", json!({"color": "#ffffff", "width": 3.5, "softness": 2, "opacity": 90}));
    let directional = get("b173-r", "core.directional_blur", json!({"direction": 45, "length": 30}));
    let offset = get("b173-f", "core.offset", json!({"shift": [37.5, -21.25]}));
    shots.push(shot(
        "B-155's moving Noise first, before Levels, a Gaussian Blur, Hue/Saturation and a Vignette".into(),
        [vec![noise(), levels, blur, hue, vignette], vec![noise(), drop, curves, balance], vec![outline, directional, offset]],
        [true, true, false],
    ));
    // The control: a Median, which does not move, stays on the card at Draft.
    shots.push(one("Median, which does not move,", "core.median", &json!({"radius": 4, "operate_on_alpha": "on"}), false));
    shots
}

fn distance(a: &[u8], b: &[u8]) -> (u8, usize) {
    assert_eq!(a.len(), b.len(), "the two pictures are different sizes");
    let mut largest = 0;
    let mut pixels = 0;
    for (p, q) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
        let d = p.iter().zip(q).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0);
        largest = largest.max(d);
        pixels += (d > 0) as usize;
    }
    (largest, pixels)
}

/// How many effects the card's plan leaves to the card on each of the first three layers.
fn left(shot: &Shot, frame: i32, quality: PreviewQuality) -> Vec<usize> {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(&shot.project, &shot.comp, frame, &shot.root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    let kinds = ["Radial(Radial", "Bloom(Bloom", "Directional(Directional", "Gaussian(Gaussian", "Glow(Glow", "Fx(Fx"];
    plan.layers
        .iter()
        .take(3)
        .map(|l| {
            let said = format!("{:?}", l.on_card);
            kinds.iter().map(|k| said.matches(k).count()).sum()
        })
        .collect()
}

#[test]
fn b173_draft_moving() {
    let out = repo("verification/B-173_draft_moving_table.md");
    let mut gpu = match Gpu::new() {
        Ok(gpu) => gpu,
        Err(why) => {
            fs::write(&out, format!("# B-173: moving effects at Draft\n\n**NOT RUN.** No usable card: {why}\n\nNo check in this table was run, so none of them passes.\n"))
                .expect("write the B-173 table");
            return;
        }
    };
    let (mut rows, mut checks, mut passed) = (String::new(), 0, 0);
    let (mut again, mut again_checks, mut again_passed) = (String::new(), 0, 0);
    let mut worst: (u8, usize, String) = (0, 0, "none".into());
    for shot in &shots() {
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            let mut cache = CelCache::viewer();
            gpu.forget();
            let mut first = Vec::new();
            for frame in [0, 100] {
                let full = left(shot, frame, PreviewQuality::Full);
                let said = |log: FrameLog| {
                    let mut ids: Vec<&str> = log.finish().iter().map(|d| d.id.as_str()).collect();
                    ids.sort();
                    ids.dedup();
                    ids.join(", ")
                };
                let mut log = FrameLog::new(3);
                let c = preview::preview_frame_cached(&shot.project, &shot.comp, frame, &shot.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache)
                    .unwrap_or_else(|d| panic!("{} frame {frame} on the CPU: {}", shot.name, d.message))
                    .to_srgb8_straight();
                let said_cpu = said(log);
                let mut log = FrameLog::new(3);
                let (g, ..) = preview::preview_frame_srgb8(&shot.project, &shot.comp, frame, &shot.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu)
                    .unwrap_or_else(|d| panic!("{} frame {frame} on the GPU: {}", shot.name, d.message));
                let said_gpu = said(log);
                let on_cpu = said_gpu.contains(DiagnosticId::GpuPreviewOnCpu.as_str());
                let d = distance(&c, &g);
                if frame == 0 {
                    first = g;
                }
                let n = if quality == PreviewQuality::Full { full.clone() } else { left(shot, frame, quality) };
                // Every layer's run stays the card's, at Draft as at Full.
                let expected = full.clone();
                let close = !on_cpu && said_cpu == said_gpu && d.0 <= LIMIT;
                let placed = n == expected && full.first().is_some_and(|&k| k > 0);
                let pass = close && placed;
                checks += 1;
                passed += pass as usize;
                let case = format!("{} frame {frame}, {}", shot.name, quality.label());
                if (d.0, d.1) > (worst.0, worst.1) {
                    worst = (d.0, d.1, case.clone());
                }
                let list = |v: &[usize]| v.iter().map(|k| k.to_string()).collect::<Vec<_>>().join(" / ");
                let _ = writeln!(
                    rows,
                    "| {case} | {} | {} | {} | {} | {} | {} |",
                    list(&n),
                    list(&expected),
                    d.0,
                    d.1,
                    match (said_gpu.is_empty(), said_cpu == said_gpu) {
                        (true, true) => "none".to_string(),
                        (false, true) => format!("{said_gpu}, on both"),
                        (_, false) => format!("CPU: {said_cpu}; GPU: {said_gpu}"),
                    },
                    match (on_cpu, close, placed) {
                        (true, ..) => "FAIL: the CPU drew the whole frame",
                        (false, true, true) => "PASS",
                        (false, false, _) => "FAIL: the pictures differ",
                        (false, true, false) => "FAIL: not where it should be drawn",
                    }
                );
            }
            // Frame 0 again, after frame 100.
            let mut log = FrameLog::new(3);
            let before = gpu.dispatched();
            let (g, ..) = preview::preview_frame_srgb8(&shot.project, &shot.comp, 0, &shot.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu)
                .unwrap_or_else(|d| panic!("{} frame 0 again on the GPU: {}", shot.name, d.message));
            let passes = gpu.dispatched() - before;
            // One pass lays each layer; no more may run.
            let mut log = FrameLog::new(3);
            let laid = compose::plan_frame_for_card(&shot.project, &shot.comp, 0, &shot.root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame").layers.len() as u64;
            let passes = passes - passes.min(laid);
            let d = distance(&first, &g);
            let pass = passes == 0 && d == (0, 0);
            again_checks += 1;
            again_passed += pass as usize;
            let moving = if shot.moving.contains(&true) { "moves" } else { "still" };
            let result = match (passes == 0, d == (0, 0)) {
                (true, true) => "PASS",
                (false, _) => "FAIL: drawn again",
                (true, false) => "FAIL: the pictures differ",
            };
            let _ = writeln!(again, "| {} frame 0 again, {} | {moving} | {passes} | {} | {} | {result} |", shot.name, quality.label(), d.0, d.1);
        }
    }
    let s = format!(
        "# B-173: the card keeps the picture of each run it has drawn\n\n\
         Written by `tests/b173_draft_moving.rs`. The card: {}.\n\n\
         Each effect the card draws that moves by itself from frame to frame, and three the card \
         draws with one setting keyed across the shot, is put on the reference shot's first three \
         layers (the second layer's after a Drop Shadow, as B-151's check does), and B-155's moving \
         Noise first, before a run of four. A Median, which does not move, is the control.\n\n\
         **The rule (D-246):** frame 0, asked for again after frame 100, runs **no pass** on the \
         card beyond the one that lays each layer, and gives **the very picture** (every byte) the card drew for it the first time. Every \
         run stays the card's, at Draft as at Full. Each frame drawn also compares the eight-bit \
         picture the page receives, drawn by the CPU and by the GPU: **no channel of any pixel \
         more than {LIMIT} level of 255 apart**, the same warnings on both, and the card drawing \
         the frame itself.\n\n\
         **{again_passed} of {again_checks} frames asked for again pass; {passed} of {checks} \
         frames drawn pass.**\n\n\
         The largest difference is in \"{}\": {} of 255, pixels differing: {}.\n\n\
         ## Asked for again\n\n\
         | Case | Effects | Passes the card ran for effects | Largest difference from its first picture (of 255) | Pixels differing | Result |\n\
         |---|---|---:|---:|---:|---|\n{again}\n\
         ## Every frame drawn\n\n\
         Effects left to the card on the first three layers, and what they must be.\n\n\
         | Case | Left to the card | Must be | Largest difference (of 255) | Pixels differing | Warnings | Result |\n|---|---:|---:|---:|---:|---|---|\n{rows}",
        gpu.about(),
        worst.2,
        worst.0,
        worst.1,
    );
    fs::write(&out, s).expect("write the B-173 table");
    assert_eq!((passed, again_passed), (checks, again_checks), "B-173: {passed} of {checks} frames drawn, {again_passed} of {again_checks} asked for again pass");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times at Draft of every shot above, on the CPU and on the card, as B-151's timing takes
/// them: every eighth frame asked for as the viewer asks; one loop fills the caches ("first"), then
/// seven are timed ("again"); the median of their frames, in ms. Run on the build before B-173 and
/// on B-173's, turn about.
#[test]
#[ignore]
fn b173_draft_moving_timing() {
    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n\
         | Shot | Quality | CPU first | GPU first | CPU again | GPU again |\n|---|---|---:|---:|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    for shot in shots() {
        let quality = PreviewQuality::Draft;
        let (mut first_row, mut again_row) = (String::new(), String::new());
        for on_card in [false, true] {
            let mut cache = CelCache::viewer();
            gpu.forget();
            let (mut first, mut times) = (Vec::new(), Vec::new());
            for pass in 0..8 {
                for frame in (0..240).step_by(8) {
                    let mut log = FrameLog::new(3);
                    let t = std::time::Instant::now();
                    if on_card {
                        drop(preview::preview_frame_srgb8(&shot.project, &shot.comp, frame, &shot.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).expect("GPU frame"));
                    } else {
                        drop(preview::preview_frame_cached(&shot.project, &shot.comp, frame, &shot.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache).expect("CPU frame").to_srgb8_straight());
                    }
                    let ms = t.elapsed().as_secs_f64() * 1000.0;
                    if pass > 0 { times.push(ms) } else { first.push(ms) }
                }
            }
            let _ = write!(first_row, " {:.1} |", median(first));
            let _ = write!(again_row, " {:.1} |", median(times));
        }
        let _ = writeln!(s, "| {} | {} |{first_row}{again_row}", shot.name, quality.label());
    }
    fs::write(repo("verification/B-173_timing_raw.md"), s).expect("write the timing table");
}
