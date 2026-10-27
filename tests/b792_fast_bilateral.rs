//! B-792: D-913, Quality Fast on Bilateral Blur and Cartoon, the bilateral grid (wave 11, pick 9).
//!
//! Every expected pixel of Fast is `Fixtures/bilateral_blur/expected_bilateral_blur_fast.json`
//! and `Fixtures/cartoon/expected_cartoon_fast.json`, written by
//! `tools/bilateral_grid_reference.py` before this code existed. Exact, the default, is B-237's
//! and B-328's tables, unchanged.

mod effect_table;

use std::fs;

use effect_table::{median, set, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

fn effect_of(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    let comp = d.project().composition(&Id::new(MAIN)).unwrap();
    comp.layer(&Id::new("art")).unwrap().effects.last().unwrap().effect.clone()
}

fn quality_of(e: &Effect) -> String {
    match e {
        Effect::BilateralBlur { quality, .. } | Effect::Cartoon { quality, .. } => quality.clone(),
        _ => "not Bilateral Blur or Cartoon".into(),
    }
}

fn is_mine(e: &Effect) -> bool {
    matches!(e, Effect::BilateralBlur { .. } | Effect::Cartoon { .. })
}

fn bilateral(radius: f64, threshold: f64, quality: &str) -> J {
    let mut p = json!({"radius": radius, "threshold": threshold, "colorize": "on"});
    if quality != "exact" {
        p["quality"] = J::from(quality);
    }
    p
}

fn cartoon(detail_radius: f64, detail_threshold: f64, shading_steps: f64, quality: &str) -> J {
    let mut p = json!({"render": "fill_and_edges", "detail_radius": detail_radius,
        "detail_threshold": detail_threshold, "shading_steps": shading_steps,
        "shading_smoothness": 40, "edge_threshold": 1.6, "edge_width": 1.5, "edge_softness": 60,
        "edge_opacity": 100, "edge_enhancement": 0, "edge_black_level": 0, "edge_contrast": 0.5});
    if quality != "exact" {
        p["quality"] = J::from(quality);
    }
    p
}

/// The reference shot (1920 by 1080) with B-222's Color Key, an animated Noise and then the
/// effect on layer `a` only, as W7 and B-783 measure one layer.
fn shot(type_id: &str, parameters: &J) -> Project {
    effect_table::reference(|id| {
        if id != "a" {
            return json!([]);
        }
        let mut v = json!([
            {"instance_id": "b792-k", "type_id": "core.color_key", "enabled": true,
             "parameters": {"colors": ["#1e1a24", "#f0d0b0"], "tolerance": 30, "softness": 10, "match": "rgb"}},
            {"instance_id": "b792-n", "type_id": "core.noise", "enabled": true,
             "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}}
        ]);
        if !type_id.is_empty() {
            v.as_array_mut()
                .unwrap()
                .push(json!({"instance_id": "b792-r", "type_id": type_id,
                "enabled": true, "parameters": parameters}));
        }
        v
    })
}

fn cpu_frame(project: &Project, frame: i32) -> Vec<u8> {
    let mut log = FrameLog::new(3);
    preview::preview_frame_cached(
        project,
        &Id::new("comp-reference-shot"),
        frame,
        &effect_table::repo("Fixtures/reference_shot"),
        PreviewQuality::Full,
        DEFAULT_TILE_SIZE,
        &mut log,
        &mut CelCache::viewer(),
    )
    .expect("the reference shot draws")
    .to_srgb8_straight()
}

/// The 480 by 270 of a 1920 by 1080 picture around pixel `at` (kept inside the picture).
fn around(p: &[u8], at: usize) -> Vec<u8> {
    let x0 = (at % 1920).saturating_sub(240).min(1920 - 480);
    let y0 = (at / 1920).saturating_sub(135).min(1080 - 270);
    let mut out = Vec::with_capacity(480 * 270 * 4);
    for y in y0..y0 + 270 {
        out.extend_from_slice(&p[(y * 1920 + x0) * 4..(y * 1920 + x0 + 480) * 4]);
    }
    out
}

/// Each pixel's largest channel difference (opacity included) of two 8-bit pictures.
fn per_pixel(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.chunks_exact(4)
        .zip(b.chunks_exact(4))
        .map(|(p, q)| (0..4).map(|i| p[i].abs_diff(q[i])).max().unwrap())
        .collect()
}

#[test]
fn b792_fast_bilateral() {
    let mut t = Table::new(
        "bilateral_blur",
        "# B-792: Quality Fast on Bilateral Blur and Cartoon\n\n\
         D-913 (wave 11, pick 9): a setting Quality, Exact (the default, as always) or Fast, \
         which works the bilateral filter on a bilateral grid (Paris and Durand 2006; Chen, \
         Paris and Durand 2007): each pixel added into a coarse grid of place and level, the \
         grid blurred, each pixel read back at its own place and level. Every expected pixel of \
         Fast is `expected_bilateral_blur_fast.json` and `expected_cartoon_fast.json`, written by \
         `tools/bilateral_grid_reference.py` before this code existed and printed in document 25 \
         as FX-BILAT-018 to 023 and FX-CARTOON-033 and 034, compared sample by sample against \
         the tolerance of 2e-5. Exact is B-237's and B-328's tables, unchanged.\n",
    );

    t.heading("FX-BILAT-018 to 023 (document 25)");
    t.fixtures("expected_bilateral_blur_fast.json");

    t.heading("The file");
    let files: Vec<String> = (18..=23).map(|n| format!("fx_bilat_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    t.round_trips(&["fx_bilat_001.json", "fx_bilat_004.json"]);
    let e = effect_of(&t, "fx_bilat_001.json");
    let params = t.saved_parameters("fx_bilat_001.json");
    t.row(
        "fx_bilat_001.json, an older file with no quality: opens as Exact and saves no quality",
        &format!("quality {:?}; saved {params}", quality_of(&e)),
        quality_of(&e) == "exact" && params.get("quality").is_none(),
    );
    let params = t.saved_parameters("fx_bilat_018.json");
    t.row(
        "fx_bilat_018.json: Fast is saved as \"fast\"",
        &params.to_string(),
        params.get("quality") == Some(&J::from("fast")),
    );
    let why = effect_of(&t, "fx_bilat_023.json").why_invalid();
    t.row(
        "fx_bilat_023.json is refused in a sentence naming it",
        &why,
        why == "Bilateral Blur's quality is \"exact\" or \"fast\", and this is \"quick\".",
    );

    t.heading("Commands");
    let mut document = t.load("fx_bilat_001.json").document;
    let with = |q: &str| {
        let mut e = effect_of(&t, "fx_bilat_001.json");
        if let Effect::BilateralBlur { quality, .. } = &mut e {
            *quality = q.into();
        }
        e
    };
    let (quick, fast) = (with("quick"), with("fast"));
    t.refused(&mut document, vec![("Bilateral Blur's quality \"quick\"", set(quick))]);
    t.taken(
        &mut document,
        "fx_bilat_001.json",
        vec![("Bilateral Blur's quality Fast", set(fast))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_bilat_018.json", 0), ("fx_bilat_022.json", 0)]);

    t.heading("The card and the processor: Fast is not on the card; the frame the viewer shows is the processor's, within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    t.card_fixtures(
        &mut gpu,
        "expected_bilateral_blur_fast.json",
        &[18, 19, 20, 21, 22, 23],
        is_mine,
    );

    t.root = effect_table::repo("Fixtures/cartoon");
    t.heading("FX-CARTOON-033 and 034 (document 25)");
    t.fixtures("expected_cartoon_fast.json");
    t.round_trips(&["fx_cartoon_033.json", "fx_cartoon_034.json", "fx_cartoon_001.json"]);
    let e = effect_of(&t, "fx_cartoon_001.json");
    let params = t.saved_parameters("fx_cartoon_001.json");
    t.row(
        "fx_cartoon_001.json, an older file with no quality: opens as Exact and saves no quality",
        &format!("quality {:?}; saved {params}", quality_of(&e)),
        quality_of(&e) == "exact" && params.get("quality").is_none(),
    );
    let why = effect_of(&t, "fx_cartoon_034.json").why_invalid();
    t.row(
        "fx_cartoon_034.json is refused in a sentence naming it",
        &why,
        why == "Cartoon's quality is \"exact\" or \"fast\", and this is \"rough\".",
    );
    t.tiles(&[("fx_cartoon_033.json", 0)]);
    t.card_fixtures(&mut gpu, "expected_cartoon_fast.json", &[33, 34], is_mine);

    let plain = persist::load(&effect_table::repo("verification/B-08a_project.json"))
        .expect("the reference shot")
        .document
        .project()
        .clone();
    t.card_reference(
        &mut gpu,
        &plain,
        &[
            (
                "Bilateral Blur at radius 40, Fast, on layer a",
                shot("core.bilateral_blur", &bilateral(40.0, 20.0, "fast")),
                0,
            ),
            (
                "Cartoon as added, Fast, on layer a",
                shot("core.cartoon", &cartoon(8.0, 10.0, 8.0, "fast")),
                0,
            ),
        ],
        is_mine,
    );

    t.heading("Fast against Exact, the reference shot at 1920 by 1080, frame 100 at Full (an export's frame)");
    let pictures = effect_table::repo("verification/B-792 pictures");
    fs::create_dir_all(&pictures).unwrap();
    let keyed_only = cpu_frame(&shot("", &J::Null), 100);
    let cases: [(&str, &str, J, J, &str); 4] = [
        (
            "Bilateral Blur as added (radius 5, threshold 20)",
            "core.bilateral_blur",
            bilateral(5.0, 20.0, "exact"),
            bilateral(5.0, 20.0, "fast"),
            "bilateral_as_added",
        ),
        (
            "Bilateral Blur at radius 40",
            "core.bilateral_blur",
            bilateral(40.0, 20.0, "exact"),
            bilateral(40.0, 20.0, "fast"),
            "bilateral_radius_40",
        ),
        (
            "Cartoon as added (detail radius 8, threshold 10)",
            "core.cartoon",
            cartoon(8.0, 10.0, 8.0, "exact"),
            cartoon(8.0, 10.0, 8.0, "fast"),
            "cartoon_as_added",
        ),
        (
            "Cartoon heavy (detail radius 40, threshold 20, 16 steps)",
            "core.cartoon",
            cartoon(40.0, 20.0, 16.0, "exact"),
            cartoon(40.0, 20.0, 16.0, "fast"),
            "cartoon_heavy",
        ),
    ];
    for (what, type_id, exact, fast, picture) in cases {
        let (e, f) = (
            cpu_frame(&shot(type_id, &exact), 100),
            cpu_frame(&shot(type_id, &fast), 100),
        );
        let changed = effect_table::distance(&e, &keyed_only).1;
        let d = per_pixel(&e, &f);
        let largest = *d.iter().max().unwrap();
        let sum: u64 = e.iter().zip(&f).map(|(x, y)| x.abs_diff(*y) as u64).sum();
        let mean = sum as f64 / e.len() as f64;
        let within = d.iter().filter(|&&v| v <= 1).count() as f64 / d.len() as f64;
        t.row(
            &format!("{what}: Exact changes the keyed shot, and Fast against Exact"),
            &format!(
                "Exact changes {changed} pixels; Fast differs from Exact by {mean:.4} levels of 255 on average, at most {largest}; {:.2} per cent of pixels within 1 level",
                within * 100.0
            ),
            changed > 0 && largest > 0,
        );
        // Cut around the pixel where Fast and Exact differ most.
        let at = (0..d.len()).max_by_key(|&i| d[i]).unwrap();
        for (name, bytes) in [("exact", &e), ("fast", &f)] {
            png_out::write_rgba(
                &pictures.join(format!("{picture}_{name}.png")),
                480,
                270,
                OutputDepth::Eight,
                &[],
                &around(bytes, at),
            )
            .unwrap();
        }
        // The difference, eight times as bright, grey on black.
        let diff: Vec<u8> = around(&e, at)
            .chunks_exact(4)
            .zip(around(&f, at).chunks_exact(4))
            .flat_map(|(p, q)| {
                let v = (0..4).map(|i| p[i].abs_diff(q[i])).max().unwrap();
                let v = (v as u32 * 8).min(255) as u8;
                [v, v, v, 255]
            })
            .collect();
        png_out::write_rgba(
            &pictures.join(format!("{picture}_difference_x8.png")),
            480,
            270,
            OutputDepth::Eight,
            &[],
            &diff,
        )
        .unwrap();
    }

    t.finish("B-792_fast_bilateral_table.md");
}

/// Exact and Fast in alternating rounds: the reference shot, Color Key, Noise and the effect on
/// layer a (1920 by 1080), frames 0 to 210 by 30 on the processor at Full, an export's frame.
/// One frame before each run is not counted. `B792_ROUNDS` rounds (5 by default), written to
/// `B792_OUT`.
#[test]
#[ignore = "B-792: a measurement, run deliberately with --release --ignored"]
fn b792_fast_bilateral_timing() {
    use std::fmt::Write as _;
    let rounds: usize = std::env::var("B792_ROUNDS")
        .ok()
        .and_then(|r| r.parse().ok())
        .unwrap_or(5);
    let comp = Id::new("comp-reference-shot");
    let root = effect_table::repo("Fixtures/reference_shot");
    let time = |project: &Project| {
        let mut out = Vec::new();
        for (i, frame) in std::iter::once(239).chain((0..8).map(|i| i * 30)).enumerate() {
            let mut log = FrameLog::new(3);
            let at = std::time::Instant::now();
            drop(
                preview::preview_frame_cached(
                    project,
                    &comp,
                    frame,
                    &root,
                    PreviewQuality::Full,
                    DEFAULT_TILE_SIZE,
                    &mut log,
                    &mut CelCache::viewer(),
                )
                .expect("CPU frame")
                .to_srgb8_straight(),
            );
            if i > 0 {
                out.push(at.elapsed().as_secs_f64() * 1000.0);
            }
        }
        median(out)
    };
    let shots: [(&str, &str, J, J); 4] = [
        (
            "Bilateral Blur as added (radius 5, threshold 20)",
            "core.bilateral_blur",
            bilateral(5.0, 20.0, "exact"),
            bilateral(5.0, 20.0, "fast"),
        ),
        (
            "Bilateral Blur, radius 40",
            "core.bilateral_blur",
            bilateral(40.0, 20.0, "exact"),
            bilateral(40.0, 20.0, "fast"),
        ),
        (
            "Cartoon as added (detail radius 8, threshold 10)",
            "core.cartoon",
            cartoon(8.0, 10.0, 8.0, "exact"),
            cartoon(8.0, 10.0, 8.0, "fast"),
        ),
        (
            "Cartoon heavy (detail radius 40, threshold 20, 16 steps)",
            "core.cartoon",
            cartoon(40.0, 20.0, 16.0, "exact"),
            cartoon(40.0, 20.0, 16.0, "fast"),
        ),
    ];
    let mut s = format!(
        "- Processor: {}, {} threads\n- System: {}\n- Build: {}\n- Shot: the reference shot, \
         1920 by 1080, Color Key, Noise (animated) and then the effect on layer a, frames 0 to \
         210 by 30 on the processor at Full, median ms of the whole frame; {rounds} rounds, Exact \
         then Fast in each\n\n| Shot | Round | Exact | Fast |\n|---|---:|---:|---:|\n",
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let projects: Vec<(&str, Project, Project)> = shots
        .iter()
        .map(|(name, type_id, e, f)| (*name, shot(type_id, e), shot(type_id, f)))
        .collect();
    let mut all: Vec<(Vec<f64>, Vec<f64>)> = vec![(Vec::new(), Vec::new()); projects.len()];
    for round in 1..=rounds {
        for (i, (name, exact, fast)) in projects.iter().enumerate() {
            let (e, f) = (time(exact), time(fast));
            all[i].0.push(e);
            all[i].1.push(f);
            let row = format!("| {name} | {round} | {e:.1} | {f:.1} |\n");
            eprint!("{row}");
            s.push_str(&row);
        }
    }
    s.push_str("\n| Shot | Exact, median of rounds | Fast, median of rounds | Fast / Exact |\n|---|---:|---:|---:|\n");
    for ((name, ..), (e, f)) in projects.iter().zip(all) {
        let (e, f) = (median(e), median(f));
        let _ = writeln!(s, "| {name} | {e:.1} | {f:.1} | {:.2} |", f / e);
    }
    let out = std::env::var("B792_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| effect_table::repo("verification/B-792_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
