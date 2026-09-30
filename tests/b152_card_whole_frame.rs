//! B-152: a motion-blurred, frame-mixed or dissolved layer no longer sends the whole frame to the
//! CPU (GPU plan G1, D-218). The CPU still builds that one layer's picture, as it does today, and
//! the card lays it like any drawing; the rest of the frame stays on the card. The check: every
//! frame of every motion blur and frame blending fixture, and the reference shot with motion blur
//! and with frame mix and dissolve, at Full and Draft, drawn by the CPU and by the card, within
//! the card's 1 level (D-100), the card drawing the frame itself.
//!
//! Writes `verification/B-152_card_whole_frame_table.md` and, for the worst frame, three pictures
//! in `verification/B-152 pictures/`.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::png_out;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::OutputDepth;
use serde_json::json;

mod common;
use common::repo;

/// D-100's tolerance, in levels of 255.
const LIMIT: u8 = 1;

struct Shot {
    group: &'static str,
    name: String,
    project: Project,
    root: PathBuf,
    comp: Id,
    frames: Vec<i32>,
    reference: bool,
}

/// Every fixture in the two folders, each at every frame it has.
fn fixtures() -> Vec<Shot> {
    let mut shots = Vec::new();
    for (group, folder) in [("Motion blur", "motion_blur"), ("Frame blending", "frame_blending")] {
        let mut files: Vec<String> = fs::read_dir(repo(&format!("Fixtures/{folder}")))
            .expect("read a fixture folder")
            .map(|e| e.expect("a fixture file").file_name().to_string_lossy().into_owned())
            .filter(|n| n.starts_with("fx_") && n.ends_with(".json"))
            .collect();
        files.sort();
        for file in files {
            // A fixture that must not open has no frame to compare.
            let Ok(loaded) = persist::load(&repo(&format!("Fixtures/{folder}/{file}"))) else { continue };
            let project = loaded.document.project().clone();
            let comp = &project.compositions[0];
            let (id, frames) = (comp.id.clone(), (comp.start_frame..comp.start_frame + comp.duration_frames as i32).collect());
            let name = file.trim_end_matches(".json").to_string();
            shots.push(Shot { group, name, project, root: repo(&format!("Fixtures/{folder}")), comp: id, frames, reference: false });
        }
    }
    shots
}

/// The reference shot with motion blur on every layer, three of them moved by keys, or with frame
/// blending on, frame mix on a stretched second layer and a dissolve on the fourth.
fn reference(blur: bool) -> Shot {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: serde_json::Value = serde_json::from_str(&text).expect("the reference shot is JSON");
    if blur {
        j["compositions"][0]["motion_blur"] = json!({"enabled": true, "shutter_angle": 180, "shutter_phase": -90, "samples": 8});
    }
    let layers = j["compositions"][0]["layers"].as_array_mut().expect("layers");
    if blur {
        for l in layers.iter_mut() {
            l["motion_blur"] = true.into();
        }
        let keys = |a: serde_json::Value, b: serde_json::Value| {
            json!({"base": a, "keyframes": [{"frame": 0, "value": a, "interp": "linear"}, {"frame": 239, "value": b, "interp": "linear"}]})
        };
        layers[1]["transform"]["scale"] = keys(json!([100, 100]), json!([130, 130]));
        layers[2]["transform"]["position"] = keys(json!([0, 0]), json!([600, 120]));
        layers[3]["transform"]["rotation"] = keys(json!(0), json!(40));
    } else {
        // The second layer changes drawing every frame, the fourth holds some for three or five.
        layers[1]["time_stretch"] = 150.into();
        layers[1]["frame_blend"] = "frame_mix".into();
        layers[3]["drawing_dissolve"] = 2.into();
    }
    if !blur {
        j["compositions"][0]["frame_blending"] = true.into();
    }
    let group = if blur { "Motion blur" } else { "Frame blending" };
    let name = if blur { "the reference shot with motion blur" } else { "the reference shot with frame mix and dissolve" };
    let loaded = persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("{name}: {}", d.message));
    Shot {
        group,
        name: name.into(),
        project: loaded.document.project().clone(),
        root: repo("Fixtures/reference_shot"),
        comp: Id::new("comp-reference-shot"),
        frames: vec![0, 50, 100, 150, 239],
        reference: true,
    }
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

/// Whether the card's plan has a motion-blurred or mixed layer; and an adjustment layer, which
/// still has the CPU draw the whole frame (B-44).
fn marked(shot: &Shot, frame: i32, quality: PreviewQuality) -> (bool, bool) {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(&shot.project, &shot.comp, frame, &shot.root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    (plan.layers.iter().any(|l| l.motion_blur || l.mixed), plan.layers.iter().any(|l| l.adjust.is_some()))
}

#[test]
fn b152_card_whole_frame() {
    let out = repo("verification/B-152_card_whole_frame_table.md");
    let mut gpu = match Gpu::new() {
        Ok(gpu) => gpu,
        Err(why) => {
            fs::write(&out, format!("# B-152: the whole frame on the card\n\n**NOT RUN.** No usable card: {why}\n\nNo check in this table was run, so none of them passes.\n"))
                .expect("write the B-152 table");
            return;
        }
    };
    let (mut rows, mut checks, mut passed) = (String::new(), 0, 0);
    // Each group's checks, passes, marked frames and largest difference.
    let mut groups: Vec<(&str, usize, usize, usize, u8)> = vec![("Motion blur", 0, 0, 0, 0), ("Frame blending", 0, 0, 0, 0)];
    let mut worst: Option<((u8, usize), String, Vec<u8>, Vec<u8>, usize, usize)> = None;
    let mut shots = fixtures();
    shots.extend([reference(true), reference(false)]);
    for shot in &shots {
        let mut marks = 0;
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for &frame in &shot.frames {
                let mut cache = CelCache::viewer();
                let mut log = FrameLog::new(3);
                let c = preview::preview_frame_cached(&shot.project, &shot.comp, frame, &shot.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache)
                    .unwrap_or_else(|d| panic!("{} frame {frame} on the CPU: {}", shot.name, d.message));
                let (w, h) = (c.width(), c.height());
                let said = |log: FrameLog| {
                    let mut ids: Vec<&str> = log.finish().iter().map(|d| d.id.as_str()).collect();
                    ids.sort();
                    ids.dedup();
                    ids.join(", ")
                };
                let said_cpu = said(log);
                let c = c.to_srgb8_straight();
                let mut log = FrameLog::new(3);
                let (g, ..) = preview::preview_frame_srgb8(&shot.project, &shot.comp, frame, &shot.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu)
                    .unwrap_or_else(|d| panic!("{} frame {frame} on the GPU: {}", shot.name, d.message));
                let said_gpu = said(log);
                let on_cpu = said_gpu.contains(DiagnosticId::GpuPreviewOnCpu.as_str());
                let (mark, adjusted) = marked(shot, frame, quality);
                marks += mark as usize;
                let d = distance(&c, &g);
                // A frame with an adjustment layer is still the CPU's by B-44's rule, and must be
                // the CPU's picture exactly, with the CPU's warnings and the card's one message.
                let pass = if adjusted {
                    let mut ids: Vec<&str> = said_cpu.split(", ").filter(|s| !s.is_empty()).chain([DiagnosticId::GpuPreviewOnCpu.as_str()]).collect();
                    ids.sort();
                    on_cpu && d.0 == 0 && said_gpu == ids.join(", ")
                } else {
                    !on_cpu && said_cpu == said_gpu && d.0 <= LIMIT
                };
                checks += 1;
                passed += pass as usize;
                let g_row = groups.iter_mut().find(|g| g.0 == shot.group).expect("a group");
                g_row.1 += 1;
                g_row.2 += pass as usize;
                g_row.3 += mark as usize;
                g_row.4 = g_row.4.max(d.0);
                let case = format!("{} frame {frame}, {}", shot.name, quality.label());
                let _ = writeln!(
                    rows,
                    "| {case} | {} | {} | {} | {} | {} |",
                    if mark { "yes" } else { "no" },
                    d.0,
                    d.1,
                    match (said_gpu.is_empty(), said_cpu == said_gpu) {
                        (true, true) => "none".to_string(),
                        (false, true) => format!("{said_gpu}, on both"),
                        (_, false) => format!("CPU: {said_cpu}; GPU: {said_gpu}"),
                    },
                    match (adjusted, on_cpu, pass) {
                        (true, _, true) => "PASS: an adjustment layer, so the CPU drew it (B-44)",
                        (true, _, false) => "FAIL",
                        (false, true, _) => "FAIL: the CPU drew it",
                        (false, false, true) => "PASS",
                        (false, false, false) => "FAIL",
                    }
                );
                if worst.as_ref().is_none_or(|w| d > w.0) {
                    worst = Some((d, case, c, g, w, h));
                }
            }
        }
        // A reference shot that never blurs or mixes would pass by comparing plain frames.
        if shot.reference {
            checks += 1;
            passed += (marks > 0) as usize;
            let _ = writeln!(
                rows,
                "| {}: frames with a blurred or mixed layer | {marks} | — | — | — | {} |",
                shot.name,
                if marks > 0 { "PASS" } else { "FAIL: nothing was blurred or mixed" }
            );
        }
    }

    let pictures = repo("verification/B-152 pictures");
    fs::create_dir_all(&pictures).expect("make the pictures folder");
    let ((largest, count), worst_case, c, g, w, h) = worst.expect("something was compared");
    let mut diff = vec![0u8; c.len()];
    for p in (0..w * h).filter(|&p| c[p * 4..p * 4 + 4] != g[p * 4..p * 4 + 4]) {
        let (x, y) = ((p % w) as isize, (p / w) as isize);
        for yy in (y - 3).max(0)..(y + 4).min(h as isize) {
            for xx in (x - 3).max(0)..(x + 4).min(w as isize) {
                let i = (yy as usize * w + xx as usize) * 4;
                diff[i..i + 3].fill(255);
            }
        }
    }
    for px in diff.chunks_exact_mut(4) {
        px[3] = 255;
    }
    for (name, bytes) in [("cpu.png", &c), ("gpu.png", &g), ("difference.png", &diff)] {
        png_out::write_rgba(&pictures.join(name), w, h, OutputDepth::Eight, &[], bytes).expect("write a picture");
    }

    let mut summary = String::new();
    for (group, n, p, m, most) in &groups {
        let _ = writeln!(summary, "| {group} | {n} | {m} | {most} | {p} of {n} |");
    }
    let s = format!(
        "# B-152: the whole frame on the card, blurred and mixed layers included\n\n\
         Written by `tests/b152_card_whole_frame.rs`. The card: {}.\n\n\
         Each row compares the eight-bit picture the page receives, drawn by the CPU and by the \
         GPU. A motion-blurred, frame-mixed or dissolved layer is built by the CPU and laid by the \
         card with the rest of the frame (D-218). **The rule: the card draws the frame itself, no \
         channel of any pixel more than {LIMIT} level of 255 apart** (D-100), with the same \
         warnings on both. A frame with an adjustment layer is still drawn by the CPU (B-44): \
         that one must be the CPU's picture exactly, the card's message `GPU_PREVIEW_ON_CPU` its \
         only extra warning. Each reference shot must have at least one frame with a blurred or \
         mixed layer.\n\n\
         **{passed} of {checks} checks pass.**\n\n\
         The worst comparison is \"{worst_case}\": largest difference {largest} of 255, pixels differing: {count}. \
         Its pictures are in `verification/B-152 pictures/`: `cpu.png`, `gpu.png`, and \
         `difference.png`, black where the two agree and a white 7 by 7 square around every pixel \
         where they do not.\n\n\
         ## Each group\n\n\
         | Group | Frames compared | Frames with a blurred or mixed layer | Largest difference (of 255) | Pass |\n|---|---:|---:|---:|---|\n{summary}\n\
         ## Every frame\n\n\
         | Case | Blurred or mixed | Largest difference (of 255) | Pixels differing | Warnings | Result |\n|---|---|---:|---:|---|---|\n{rows}",
        gpu.about(),
    );
    fs::write(&out, s).expect("write the B-152 table");
    assert_eq!(passed, checks, "B-152: {passed} of {checks} checks pass");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times on the card: the plain reference shot, then with motion blur, then with frame
/// mix and dissolve, every eighth frame asked for as the viewer asks. One loop fills the caches,
/// then seven are timed; the median of their 210 frames, in ms. The same test run on the build
/// before B-152 gives the "before" column, when the CPU drew such frames whole.
#[test]
#[ignore = "B-152: a measurement, run deliberately with --release --ignored"]
fn b152_card_whole_frame_timing() {
    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n| Shot | Quality | GPU |\n|---|---|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let mut plain = reference(true);
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    plain.project = persist::load_str(&text).expect("the reference shot").document.project().clone();
    plain.name = "the reference shot".into();
    // Motion blur beside a card effect that changes every frame, on the first layer, its own
    // motion blur switch off: before B-152 the CPU drew that effect too.
    let mut rough = reference(true);
    let blurred = &rough.project;
    let mut j: serde_json::Value = serde_json::from_str(&persist::to_json(blurred, &Default::default())).expect("the shot as JSON");
    j["compositions"][0]["layers"][0]["effects"] = json!([{"instance_id": "b152-r", "type_id": "core.roughen_edges", "enabled": true,
        "parameters": {"edge_type": "roughen_color", "edge_color": "#8a3c14", "border": 6, "size": 8, "complexity": 3, "evolution": 30, "speed": 10, "seed": 3}}]);
    j["compositions"][0]["layers"][0]["motion_blur"] = false.into();
    rough.project = persist::load_str(&j.to_string()).expect("the shot with Roughen Edges").document.project().clone();
    rough.name = "the reference shot with motion blur and Roughen Edges".into();
    for shot in [plain, reference(true), reference(false), rough] {
        for quality in [PreviewQuality::Draft, PreviewQuality::Full] {
            let mut cache = CelCache::viewer();
            gpu.forget();
            let mut times = Vec::new();
            for pass in 0..8 {
                for frame in (0..240).step_by(8) {
                    let mut log = FrameLog::new(3);
                    let t = std::time::Instant::now();
                    drop(preview::preview_frame_srgb8(&shot.project, &shot.comp, frame, &shot.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).expect("GPU frame"));
                    if pass > 0 {
                        times.push(t.elapsed().as_secs_f64() * 1000.0);
                    }
                }
            }
            let _ = writeln!(s, "| {} | {} | {:.1} |", shot.name, quality.label(), median(times));
        }
    }
    fs::write(repo("verification/B-152_timing_raw.md"), s).expect("write the timing table");
}
