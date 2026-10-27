//! B-783: D-903, Quality Fast on Refine Hard Matte and Refine Soft Matte (wave 11, pick 9).
//!
//! Every expected pixel of Fast is `Fixtures/matte_refine/expected_matte_refine_fast.json`,
//! written by `tools/matte_refine_fast_reference.py` before this code existed. Exact, the
//! default, is B-232's own table, unchanged.

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
    comp.layer(&Id::new("art")).unwrap().effects[0]
        .effect
        .clone()
}

fn quality_of(e: &Effect) -> String {
    match e {
        Effect::RefineMatte { quality, .. } => quality.clone(),
        _ => "not a Refine effect".into(),
    }
}

fn is_refine(e: &Effect) -> bool {
    matches!(e, Effect::RefineMatte { .. })
}

fn soft(edge_radius: f64, feather: f64, decontamination_radius: f64, quality: &str) -> J {
    let mut p = json!({"edge_radius": edge_radius, "view_edge_region": "off", "feather": feather,
        "contrast": 0, "shift_edge": 0, "decontaminate": "on", "decontamination_amount": 100,
        "decontamination_radius": decontamination_radius, "view_decontamination_map": "off"});
    if quality != "exact" {
        p["quality"] = J::from(quality);
    }
    p
}

fn hard(feather: f64, decontamination_radius: f64, quality: &str) -> J {
    let mut p = json!({"feather": feather, "contrast": 50, "shift_edge": 0, "decontaminate": "off",
        "decontamination_amount": 100, "decontamination_radius": decontamination_radius,
        "view_decontamination_map": "off"});
    if quality != "exact" {
        p["quality"] = J::from(quality);
    }
    p
}

/// The reference shot (1920 by 1080) with B-222's Color Key (so the matte has edges), an
/// animated Noise and then the effect on layer `a` only, so nothing is kept from one frame to the next, as W7 measures one layer.
fn shot(type_id: &str, parameters: &J) -> Project {
    effect_table::reference(|id| {
        if id != "a" {
            return json!([]);
        }
        let mut v = json!([
            {"instance_id": "b783-k", "type_id": "core.color_key", "enabled": true,
             "parameters": {"colors": ["#1e1a24", "#f0d0b0"], "tolerance": 30, "softness": 10, "match": "rgb"}},
            {"instance_id": "b783-n", "type_id": "core.noise", "enabled": true,
             "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}}
        ]);
        if !type_id.is_empty() {
            v.as_array_mut()
                .unwrap()
                .push(json!({"instance_id": "b783-r", "type_id": type_id,
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

/// The largest difference of 255, the pixels differing, and the mean difference over all
/// samples, of two 8-bit pictures.
fn apart(a: &[u8], b: &[u8]) -> (u8, usize, f64) {
    let (largest, pixels) = effect_table::distance(a, b);
    let sum: u64 = a.iter().zip(b).map(|(x, y)| x.abs_diff(*y) as u64).sum();
    (largest, pixels, sum as f64 / a.len() as f64)
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

#[test]
fn b783_fast_refine() {
    let mut t = Table::new(
        "matte_refine",
        "# B-783: Quality Fast on Refine Hard Matte and Refine Soft Matte\n\n\
         D-903 (wave 11, pick 9): a setting Quality, Exact (the default, as always) or Fast, \
         which works each guided filter on blocks of up to 4 by 4 pixels and reads it back \
         smoothly (He and Sun's Fast Guided Filter). Every expected pixel of Fast is \
         `Fixtures/matte_refine/expected_matte_refine_fast.json`, written by \
         `tools/matte_refine_fast_reference.py` before this code existed and printed in \
         document 25 as FX-MREF-036 to 041, compared sample by sample against the tolerance of \
         2e-5. Exact is B-232's table, unchanged.\n",
    );

    t.heading("FX-MREF-036 to 041 (document 25)");
    t.fixtures("expected_matte_refine_fast.json");

    t.heading("The file");
    let files: Vec<String> = (36..=41).map(|n| format!("fx_mref_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    t.round_trips(&["fx_mref_013.json", "fx_mref_023.json"]);
    for file in ["fx_mref_013.json", "fx_mref_023.json"] {
        let e = effect_of(&t, file);
        let params = t.saved_parameters(file);
        t.row(
            &format!("{file}, an older file with no quality: opens as Exact and saves no quality"),
            &format!("quality {:?}; saved {params}", quality_of(&e)),
            quality_of(&e) == "exact" && params.get("quality").is_none(),
        );
    }
    let params = t.saved_parameters("fx_mref_036.json");
    t.row(
        "fx_mref_036.json: Fast is saved as \"fast\"",
        &params.to_string(),
        params.get("quality") == Some(&J::from("fast")),
    );
    let why = effect_of(&t, "fx_mref_041.json").why_invalid();
    t.row(
        "fx_mref_041.json is refused in a sentence naming it",
        &why,
        why == "Refine Soft Matte's quality is \"exact\" or \"fast\", and this is \"medium\".",
    );

    t.heading("Commands");
    let mut document = t.load("fx_mref_023.json").document;
    let with = |q: &str| {
        let mut e = effect_of(&t, "fx_mref_023.json");
        if let Effect::RefineMatte { quality, .. } = &mut e {
            *quality = q.into();
        }
        e
    };
    let (medium, fast) = (with("medium"), with("fast"));
    t.refused(
        &mut document,
        vec![("Refine Soft Matte's quality \"medium\"", set(medium))],
    );
    t.taken(
        &mut document,
        "fx_mref_023.json",
        vec![("Refine Soft Matte's quality Fast", set(fast))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_mref_036.json", 0),
        ("fx_mref_039.json", 0),
        ("fx_mref_040.json", 0),
    ]);

    t.heading("The card and the processor: neither quality is on the card; the frame the viewer shows is the processor's, within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    t.card_fixtures(
        &mut gpu,
        "expected_matte_refine_fast.json",
        &[36, 37, 38, 39, 40, 41],
        is_refine,
    );
    let plain = persist::load(&effect_table::repo("verification/B-08a_project.json"))
        .expect("the reference shot")
        .document
        .project()
        .clone();
    let heavy_fast = shot("core.refine_soft_matte", &soft(40.0, 40.0, 40.0, "fast"));
    t.card_reference(
        &mut gpu,
        &plain,
        &[(
            "Refine Soft Matte at radius 40 (edge, feather, decontamination), Fast, on layer a",
            heavy_fast.clone(),
            0,
        )],
        is_refine,
    );

    t.heading("Fast against Exact, the reference shot at 1920 by 1080, frame 100 at Full (an export's frame)");
    let pictures = effect_table::repo("verification/B-783 pictures");
    fs::create_dir_all(&pictures).unwrap();
    let keyed_only = cpu_frame(&shot("", &J::Null), 100);
    let cases: [(&str, &str, J, J, &str); 4] = [
        (
            "Refine Soft Matte as added (edge radius 10: blocks 2 by 2)",
            "core.refine_soft_matte",
            soft(10.0, 0.0, 0.0, "exact"),
            soft(10.0, 0.0, 0.0, "fast"),
            "soft_as_added",
        ),
        (
            "Refine Soft Matte at radius 40 (blocks 4 by 4)",
            "core.refine_soft_matte",
            soft(40.0, 40.0, 40.0, "exact"),
            soft(40.0, 40.0, 40.0, "fast"),
            "soft_radius_40",
        ),
        (
            "Refine Hard Matte at feather 40 (blocks 4 by 4)",
            "core.refine_hard_matte",
            hard(40.0, 40.0, "exact"),
            hard(40.0, 40.0, "fast"),
            "hard_feather_40",
        ),
        (
            "Refine Hard Matte as added (feather 2: Fast is Exact below radius 8)",
            "core.refine_hard_matte",
            hard(2.0, 0.0, "exact"),
            hard(2.0, 0.0, "fast"),
            "",
        ),
    ];
    for (what, type_id, exact, fast, picture) in cases {
        let (e, f) = (
            cpu_frame(&shot(type_id, &exact), 100),
            cpu_frame(&shot(type_id, &fast), 100),
        );
        let changed = effect_table::distance(&e, &keyed_only).1;
        let (largest, pixels, mean) = apart(&e, &f);
        let below_8 = picture.is_empty();
        t.row(
            &format!("{what}: Exact changes the keyed shot, and Fast against Exact"),
            &format!(
                "Exact changes {changed} pixels; Fast differs from Exact by at most {largest} of 255, in {pixels} of 2,073,600 pixels, {mean:.4} levels on average"
            ),
            changed > 0 && if below_8 { largest == 0 } else { largest > 0 },
        );
        if !below_8 {
            // Cut around the pixel where Fast and Exact differ most.
            let at = (0..1920 * 1080)
                .max_by_key(|i| (0..4).map(|c| e[i * 4 + c].abs_diff(f[i * 4 + c])).max())
                .unwrap();
            let middle = |p: &[u8]| around(p, at);
            for (name, bytes) in [("exact", &e), ("fast", &f)] {
                png_out::write_rgba(
                    &pictures.join(format!("{picture}_{name}.png")),
                    480,
                    270,
                    OutputDepth::Eight,
                    &[],
                    &middle(bytes),
                )
                .unwrap();
            }
            // The largest difference of the four channels (opacity included), eight times as
            // bright, in grey on black.
            let diff: Vec<u8> = middle(&e)
                .chunks_exact(4)
                .zip(middle(&f).chunks_exact(4))
                .flat_map(|(p, q)| {
                    let d = (0..4).map(|i| p[i].abs_diff(q[i])).max().unwrap();
                    let d = (d as u32 * 8).min(255) as u8;
                    [d, d, d, 255]
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
    }

    t.finish("B-783_fast_refine_table.md");
}

/// Exact and Fast in alternating rounds: the reference shot, Color Key, Noise and the effect on layer a
/// (1920 by 1080), frames 0 to 210 by 30 asked for on the processor at Full, which is an
/// export's frame and, since the card does not draw these effects, the viewer's too. One frame
/// before each run is not counted. `B783_ROUNDS` rounds (5 by default), written to `B783_OUT`.
#[test]
#[ignore = "B-783: a measurement, run deliberately with --release --ignored"]
fn b783_fast_refine_timing() {
    use std::fmt::Write as _;
    let rounds: usize = std::env::var("B783_ROUNDS")
        .ok()
        .and_then(|r| r.parse().ok())
        .unwrap_or(5);
    let comp = Id::new("comp-reference-shot");
    let root = effect_table::repo("Fixtures/reference_shot");
    let time = |project: &Project| {
        let mut out = Vec::new();
        for (i, frame) in std::iter::once(239)
            .chain((0..8).map(|i| i * 30))
            .enumerate()
        {
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
            "Refine Soft Matte as added (edge radius 10)",
            "core.refine_soft_matte",
            soft(10.0, 0.0, 0.0, "exact"),
            soft(10.0, 0.0, 0.0, "fast"),
        ),
        (
            "Refine Soft Matte heavy (edge radius, feather, decontamination radius 40)",
            "core.refine_soft_matte",
            soft(40.0, 40.0, 40.0, "exact"),
            soft(40.0, 40.0, 40.0, "fast"),
        ),
        (
            "Refine Hard Matte heavy (feather, decontamination radius 40)",
            "core.refine_hard_matte",
            hard(40.0, 40.0, "exact"),
            hard(40.0, 40.0, "fast"),
        ),
        (
            "Refine Hard Matte, feather 16",
            "core.refine_hard_matte",
            hard(16.0, 0.0, "exact"),
            hard(16.0, 0.0, "fast"),
        ),
    ];
    let mut s = format!(
        "- Processor: {}, {} threads\n- System: {}\n- Build: {}\n- Shot: the reference shot, \
         1920 by 1080, Color Key, Noise (animated) and then the effect on layer a, frames 0 to 210 by 30 on the \
         processor at Full, median ms of the whole frame; {rounds} rounds, Exact then Fast in \
         each\n\n| Shot | Round | Exact | Fast |\n|---|---:|---:|---:|\n",
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        },
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
    let out = std::env::var("B783_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| effect_table::repo("verification/B-783_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
