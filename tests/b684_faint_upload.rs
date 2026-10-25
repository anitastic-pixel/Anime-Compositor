//! B-684: a layer drawing with a pixel too faint for half precision goes to the card in 32-bit
//! floats, so the card draws its colour as the CPU does.
//!
//! Pixel Motion Blur and Timewarp mix neighbour frames and leave pixels whose alpha is about 1e-6:
//! 0 of 255, but not 0. Sent in half precision their colour channels were subnormals of a few bits,
//! and with Exposure after them in a run of colour effects (b222) the straight colour the card wrote
//! for them was up to 10 levels from the CPU's. The three cases b222 failed, CPU against card, at
//! most 1 level (ADR-006, D-100).
//!
//! Writes `verification/B-684_faint_upload_table.md` and `verification/B-684 pictures/`.

use std::fmt::Write as _;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::gpu::Gpu;
use anime_compositor::persist;
use anime_compositor::png_out;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::OutputDepth;
use serde_json::{json, Value};
mod common;
use common::repo;

/// b222's run: Hue/Saturation, the effect, Levels.
fn load(file: &str) -> anime_compositor::model::Project {
    let text = std::fs::read_to_string(repo(file)).unwrap();
    let mut j: Value = serde_json::from_str(&text).unwrap();
    for layer in j["compositions"][0]["layers"].as_array_mut().unwrap() {
        let stack = layer["effects"].as_array_mut().unwrap();
        let mut out = Vec::new();
        for e in stack.drain(..) {
            let exposure = e["type_id"] == "core.exposure";
            if exposure {
                out.push(json!({"instance_id": "hs", "type_id": "core.hue_saturation", "enabled": true, "parameters": {"hue": 40, "saturation": 30, "lightness": -10}}));
            }
            out.push(e);
            if exposure {
                out.push(json!({"instance_id": "lv", "type_id": "core.levels", "enabled": true,
                    "parameters": {"input_black": 20, "input_white": 230, "gamma": 1.6, "output_black": 10, "output_white": 245}}));
            }
        }
        *stack = out;
    }
    persist::load_str(&j.to_string()).unwrap().document.project().clone()
}

#[test]
fn b684_faint_upload() {
    let mut gpu = Gpu::new().expect("a usable card");
    let mut rows = String::new();
    let mut failed = 0;
    let pictures = repo("verification/B-684 pictures");
    std::fs::create_dir_all(&pictures).unwrap();
    for (n, (file, folder, frame, q)) in [
        ("Fixtures/pixel_motion_blur/fx_pmb_013.json", "Fixtures/pixel_motion_blur", 1, PreviewQuality::Full),
        ("Fixtures/pixel_motion_blur/fx_pmb_013.json", "Fixtures/pixel_motion_blur", 2, PreviewQuality::Draft),
        ("Fixtures/timewarp/fx_tw_020.json", "Fixtures/timewarp", 5, PreviewQuality::Draft),
    ]
    .into_iter()
    .enumerate()
    {
        let p = load(file);
        let root = repo(folder);
        let comp = p.compositions[0].id.clone();
        let mut cache = CelCache::viewer();
        let f = preview::preview_frame_cached(&p, &comp, frame, &root, q, DEFAULT_TILE_SIZE, &mut FrameLog::new(3), &mut cache).unwrap();
        let (w, h) = (f.width(), f.height());
        let c = f.to_srgb8_straight();
        let faint = f.data().chunks_exact(4).filter(|p| p[3] > 0.0 && p[3] < 1.0 / 510.0).count();
        let (g, ..) = preview::preview_frame_srgb8(&p, &comp, frame, &root, q, DEFAULT_TILE_SIZE, &mut FrameLog::new(3), &mut cache, &mut gpu).unwrap();
        let (mut most, mut differ) = (0u8, 0usize);
        let mut diff = vec![0u8; c.len()];
        for (i, (a, b)) in c.chunks_exact(4).zip(g.chunks_exact(4)).enumerate() {
            let d = a.iter().zip(b).map(|(x, y)| x.abs_diff(*y)).max().unwrap();
            most = most.max(d);
            differ += (d > 0) as usize;
            let v = (d as usize * 25).min(255) as u8;
            diff[i * 4..i * 4 + 4].copy_from_slice(&[v, v, v, 255]);
        }
        let pass = most <= 1;
        failed += !pass as usize;
        let name = file.rsplit('/').next().unwrap().trim_end_matches(".json");
        let _ = writeln!(rows, "| {name} frame {frame}, {q:?} | {faint} | {differ} | {most} | {} |", if pass { "PASS" } else { "FAIL" });
        if n != 1 {
            for (what, bytes) in [("cpu", &c), ("card", &g), ("difference", &diff)] {
                png_out::write_rgba(&pictures.join(format!("{name}_f{frame}_{what}.png")), w, h, OutputDepth::Eight, &[], bytes).unwrap();
            }
        }
    }
    let s = format!(
        "# B-684: a drawing too faint for half precision goes to the card in 32-bit floats\n\n\
         Written by `tests/b684_faint_upload.rs`. b222's three failed cases, Hue/Saturation, Exposure and \
         Levels after Pixel Motion Blur or Timewarp in one run, drawn by the CPU and by the card. Rule: \
         no channel of any pixel more than 1 level of 255 apart (ADR-006, D-100). Before B-684 these \
         were 10, 10 and 6 levels apart.\n\n\
         | Case | Pixels with alpha above 0 but under half a level | Pixels differing | Largest (levels) | Result |\n\
         |---|---:|---:|---:|---|\n{rows}\n{} of 3 cases pass.\n\n\
         Pictures in `verification/B-684 pictures/`: the CPU's frame, the card's, and the difference \
         (25 times brighter per level; black where they agree).\n",
        3 - failed
    );
    std::fs::write(repo("verification/B-684_faint_upload_table.md"), s).unwrap();
    assert_eq!(failed, 0, "a case differs past 1 level");
}
