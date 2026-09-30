//! B-156b: a motion-blurred layer's moments summed on the card (GPU plan G5, D-226). Until now
//! the CPU averaged a motion-blurred layer's moments into a new picture every frame, and a frame
//! with one and no effect for the card went to the CPU whole (B-153b). The check: every frame of
//! every motion blur fixture, and the reference shot with motion blur on every layer, at Full and
//! Draft, drawn by the CPU and by the card. **The card draws the frame itself, within its 1 level
//! (D-100), with the same warnings**; only a frame whose matte is motion-blurred may still go to
//! the CPU, and then it must be the CPU's picture exactly with the card's one message. Every
//! frame of the reference shot with motion blur must be drawn on the card.
//!
//! Writes `verification/B-156b_gpu_motion_blur_table.md` and, for the worst frame, three pictures
//! in `verification/B-156b pictures/`.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
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
    name: String,
    project: Project,
    root: PathBuf,
    comp: Id,
    frames: Vec<i32>,
}

/// Every motion blur fixture that opens, each at every frame it has, then the reference shot with
/// motion blur on every layer and three of them moved by keys, as B-152 has it.
fn shots() -> Vec<Shot> {
    let mut files: Vec<String> = fs::read_dir(repo("Fixtures/motion_blur"))
        .expect("read the fixture folder")
        .map(|e| e.expect("a fixture file").file_name().to_string_lossy().into_owned())
        .filter(|n| n.starts_with("fx_") && n.ends_with(".json"))
        .collect();
    files.sort();
    let mut shots = Vec::new();
    for file in files {
        let Ok(loaded) = persist::load(&repo(&format!("Fixtures/motion_blur/{file}"))) else { continue };
        let project = loaded.document.project().clone();
        let comp = &project.compositions[0];
        let (id, frames) = (comp.id.clone(), (comp.start_frame..comp.start_frame + comp.duration_frames as i32).collect());
        shots.push(Shot { name: file.trim_end_matches(".json").into(), project, root: repo("Fixtures/motion_blur"), comp: id, frames });
    }
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: serde_json::Value = serde_json::from_str(&text).expect("the reference shot is JSON");
    j["compositions"][0]["motion_blur"] = json!({"enabled": true, "shutter_angle": 180, "shutter_phase": -90, "samples": 8});
    let layers = j["compositions"][0]["layers"].as_array_mut().expect("layers");
    for l in layers.iter_mut() {
        l["motion_blur"] = true.into();
    }
    let keys = |a: serde_json::Value, b: serde_json::Value| {
        json!({"base": a, "keyframes": [{"frame": 0, "value": a, "interp": "linear"}, {"frame": 239, "value": b, "interp": "linear"}]})
    };
    layers[1]["transform"]["scale"] = keys(json!([100, 100]), json!([130, 130]));
    layers[2]["transform"]["position"] = keys(json!([0, 0]), json!([600, 120]));
    layers[3]["transform"]["rotation"] = keys(json!(0), json!(40));
    shots.push(Shot {
        name: REFERENCE.into(),
        project: persist::load_str(&j.to_string()).expect("the reference shot with motion blur").document.project().clone(),
        root: repo("Fixtures/reference_shot"),
        comp: Id::new("comp-reference-shot"),
        frames: vec![0, 50, 100, 150, 239],
    });
    shots
}

const REFERENCE: &str = "the reference shot with motion blur";

/// Whether a layer of the shot is cut by a matte whose own motion blur switch is on, the one case
/// left to the CPU.
fn blurred_matte(shot: &Shot) -> bool {
    let comp = &shot.project.compositions[0];
    comp.motion_blur.enabled
        && comp.layers_in_order().filter_map(|l| l.matte.as_ref()).any(|m| comp.layer(&m.layer_id).is_some_and(|l| l.motion_blur))
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

#[test]
fn b156b_gpu_motion_blur() {
    let out = repo("verification/B-156b_gpu_motion_blur_table.md");
    let mut gpu = match Gpu::new() {
        Ok(gpu) => gpu,
        Err(why) => {
            fs::write(&out, format!("# B-156b: motion blur summed on the card\n\n**NOT RUN.** No usable card: {why}\n\nNo check in this table was run, so none of them passes.\n"))
                .expect("write the B-156b table");
            return;
        }
    };
    let (mut rows, mut checks, mut passed) = (String::new(), 0, 0);
    let mut worst: Option<((u8, usize), String, Vec<u8>, Vec<u8>, usize, usize)> = None;
    for shot in &shots() {
        let matte = blurred_matte(shot);
        let mut on_card = 0;
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
                on_card += !on_cpu as usize;
                let d = distance(&c, &g);
                let pass = if on_cpu {
                    let mut ids: Vec<&str> = said_cpu.split(", ").filter(|s| !s.is_empty()).chain([DiagnosticId::GpuPreviewOnCpu.as_str()]).collect();
                    ids.sort();
                    matte && d.0 == 0 && said_gpu == ids.join(", ")
                } else {
                    said_cpu == said_gpu && d.0 <= LIMIT
                };
                checks += 1;
                passed += pass as usize;
                let case = format!("{} frame {frame}, {}", shot.name, quality.label());
                let _ = writeln!(
                    rows,
                    "| {case} | {} | {} | {} | {} | {} |",
                    if on_cpu { "CPU" } else { "GPU" },
                    d.0,
                    d.1,
                    match (said_gpu.is_empty(), said_cpu == said_gpu) {
                        (true, true) => "none".to_string(),
                        (false, true) => format!("{said_gpu}, on both"),
                        (_, false) => format!("CPU: {said_cpu}; GPU: {said_gpu}"),
                    },
                    match (on_cpu, pass) {
                        (true, true) => "PASS: a motion-blurred matte, so the CPU drew it",
                        (true, false) => "FAIL: the CPU drew it",
                        (false, true) => "PASS",
                        (false, false) => "FAIL",
                    }
                );
                if worst.as_ref().is_none_or(|w| d > w.0) {
                    worst = Some((d, case, c, g, w, h));
                }
            }
        }
        if shot.name == REFERENCE {
            let all = 2 * shot.frames.len();
            checks += 1;
            passed += (on_card == all) as usize;
            let _ = writeln!(
                rows,
                "| {REFERENCE}: frames drawn on the card | {on_card} of {all} | — | — | — | {} |",
                if on_card == all { "PASS" } else { "FAIL: not every frame" }
            );
        }
    }

    let pictures = repo("verification/B-156b pictures");
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

    let s = format!(
        "# B-156b: motion blur summed on the card\n\n\
         Written by `tests/b156b_gpu_motion_blur.rs`. The card: {}.\n\n\
         Each row compares the eight-bit picture the page receives, drawn by the CPU and by the \
         GPU, for every frame of every motion blur fixture and for the reference shot with motion \
         blur on every layer. **The rule: the card draws the frame itself, adding up each \
         motion-blurred layer's moments there, no channel of any pixel more than {LIMIT} level of \
         255 apart** (D-100), with the same warnings on both. Only a frame whose matte is \
         motion-blurred may still be drawn by the CPU, and then it must be the CPU's picture \
         exactly, the card's message `GPU_PREVIEW_ON_CPU` its only extra warning. Every frame of \
         the reference shot with motion blur must be drawn on the card.\n\n\
         **{passed} of {checks} checks pass.**\n\n\
         The worst comparison is \"{worst_case}\": largest difference {largest} of 255, pixels differing: {count}. \
         Its pictures are in `verification/B-156b pictures/`: `cpu.png`, `gpu.png`, and \
         `difference.png`, black where the two agree and a white 7 by 7 square around every pixel \
         where they do not.\n\n\
         | Case | Drawn on | Largest difference (of 255) | Pixels differing | Warnings | Result |\n|---|---|---:|---:|---|---|\n{rows}",
        gpu.about(),
    );
    fs::write(&out, s).expect("write the B-156b table");
    assert_eq!(passed, checks, "B-156b: {passed} of {checks} checks pass");
}
