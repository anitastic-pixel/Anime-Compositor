//! B-791: D-912, Quality Fast on Key Cleaner (wave 11, pick 9, second step after D-903).
//!
//! Every expected pixel of Fast is `Fixtures/key_cleaner/expected_key_cleaner_fast.json`,
//! written by `tools/key_cleaner_fast_reference.py` before this code existed. Exact, the
//! default, is B-383's own table, unchanged.

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
        Effect::KeyCleaner { quality, .. } => quality.clone(),
        _ => "not Key Cleaner".into(),
    }
}

fn is_key_cleaner(e: &Effect) -> bool {
    matches!(e, Effect::KeyCleaner { .. })
}

fn cleaner(radius: f64, quality: &str) -> J {
    let mut p = json!({"additional_edge_radius": radius, "alpha_contrast": 0, "strength": 100});
    if quality != "exact" {
        p["quality"] = J::from(quality);
    }
    p
}

/// The reference shot (1920 by 1080) with B-222's Color Key (so the matte has edges), an
/// animated Noise and then Key Cleaner on layer `a` only, as B-783 measures Refine.
fn shot(parameters: Option<&J>) -> Project {
    effect_table::reference(|id| {
        if id != "a" {
            return json!([]);
        }
        let mut v = json!([
            {"instance_id": "b791-k", "type_id": "core.color_key", "enabled": true,
             "parameters": {"colors": ["#1e1a24", "#f0d0b0"], "tolerance": 30, "softness": 10, "match": "rgb"}},
            {"instance_id": "b791-n", "type_id": "core.noise", "enabled": true,
             "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}}
        ]);
        if let Some(p) = parameters {
            v.as_array_mut().unwrap().push(json!({"instance_id": "b791-c",
                "type_id": "core.key_cleaner", "enabled": true, "parameters": p}));
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
fn b791_fast_key_cleaner() {
    let mut t = Table::new(
        "key_cleaner",
        "# B-791: Quality Fast on Key Cleaner\n\n\
         D-912 (wave 11, pick 9, second step): Key Cleaner gets D-903's setting Quality, Exact \
         (the default, as always) or Fast, which works its guided filter on blocks of up to 4 \
         by 4 pixels and reads it back smoothly (He and Sun's Fast Guided Filter), exactly as \
         Refine Soft Matte's Fast does. Every expected pixel of Fast is \
         `Fixtures/key_cleaner/expected_key_cleaner_fast.json`, written by \
         `tools/key_cleaner_fast_reference.py` before this code existed and printed in \
         document 25 as FX-KCLEAN-019 to 023, compared sample by sample against the tolerance \
         of 2e-5. Exact is B-383's table, unchanged.\n",
    );

    t.heading("FX-KCLEAN-019 to 023 (document 25)");
    t.fixtures("expected_key_cleaner_fast.json");

    t.heading("The file");
    let files: Vec<String> = (19..=23).map(|n| format!("fx_kclean_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    t.round_trips(&["fx_kclean_001.json", "fx_kclean_002.json"]);
    for file in ["fx_kclean_001.json", "fx_kclean_002.json"] {
        let e = effect_of(&t, file);
        let params = t.saved_parameters(file);
        t.row(
            &format!("{file}, an older file with no quality: opens as Exact and saves no quality"),
            &format!("quality {:?}; saved {params}", quality_of(&e)),
            quality_of(&e) == "exact" && params.get("quality").is_none(),
        );
    }
    let params = t.saved_parameters("fx_kclean_019.json");
    t.row(
        "fx_kclean_019.json: Fast is saved as \"fast\"",
        &params.to_string(),
        params.get("quality") == Some(&J::from("fast")),
    );
    let why = effect_of(&t, "fx_kclean_023.json").why_invalid();
    t.row(
        "fx_kclean_023.json is refused in a sentence naming it",
        &why,
        why == "Key Cleaner's quality is \"exact\" or \"fast\", and this is \"medium\".",
    );

    t.heading("Commands");
    let mut document = t.load("fx_kclean_001.json").document;
    let with = |q: &str| {
        let mut e = effect_of(&t, "fx_kclean_001.json");
        if let Effect::KeyCleaner { quality, .. } = &mut e {
            *quality = q.into();
        }
        e
    };
    let (medium, fast) = (with("medium"), with("fast"));
    t.refused(
        &mut document,
        vec![("Key Cleaner's quality \"medium\"", set(medium))],
    );
    t.taken(
        &mut document,
        "fx_kclean_001.json",
        vec![("Key Cleaner's quality Fast", set(fast))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_kclean_019.json", 0),
        ("fx_kclean_021.json", 0),
        ("fx_kclean_022.json", 0),
    ]);

    t.heading("The card and the processor: neither quality is on the card; the frame the viewer shows is the processor's, within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    t.card_fixtures(
        &mut gpu,
        "expected_key_cleaner_fast.json",
        &[19, 20, 21, 22, 23],
        is_key_cleaner,
    );
    let plain = persist::load(&effect_table::repo("verification/B-08a_project.json"))
        .expect("the reference shot")
        .document
        .project()
        .clone();
    t.card_reference(
        &mut gpu,
        &plain,
        &[(
            "Key Cleaner at radius 40, Fast, on layer a",
            shot(Some(&cleaner(40.0, "fast"))),
            0,
        )],
        is_key_cleaner,
    );

    t.heading("Fast against Exact, the reference shot at 1920 by 1080, frame 100 at Full (an export's frame)");
    let pictures = effect_table::repo("verification/B-791 pictures");
    fs::create_dir_all(&pictures).unwrap();
    let keyed_only = cpu_frame(&shot(None), 100);
    let cases: [(&str, f64, &str); 3] = [
        ("Key Cleaner as added (radius 10: blocks 2 by 2)", 10.0, "as_added"),
        ("Key Cleaner at radius 40 (blocks 4 by 4)", 40.0, "radius_40"),
        ("Key Cleaner at radius 4 (Fast is Exact below radius 8)", 4.0, ""),
    ];
    for (what, radius, picture) in cases {
        let (e, f) = (
            cpu_frame(&shot(Some(&cleaner(radius, "exact"))), 100),
            cpu_frame(&shot(Some(&cleaner(radius, "fast"))), 100),
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

    t.finish("B-791_fast_key_cleaner_table.md");
}

/// Exact and Fast in alternating rounds: the reference shot, Color Key, Noise and Key Cleaner on
/// layer a (1920 by 1080), frames 0 to 210 by 30 asked for on the processor at Full, which is
/// an export's frame and, since the card does not draw Key Cleaner, the viewer's too. One frame
/// before each run is not counted. `B791_ROUNDS` rounds (5 by default), written to `B791_OUT`.
#[test]
#[ignore = "B-791: a measurement, run deliberately with --release --ignored"]
fn b791_fast_key_cleaner_timing() {
    use std::fmt::Write as _;
    let rounds: usize = std::env::var("B791_ROUNDS")
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
    let shots: [(&str, f64); 3] = [
        ("Key Cleaner as added (radius 10)", 10.0),
        ("Key Cleaner, radius 20", 20.0),
        ("Key Cleaner, radius 40", 40.0),
    ];
    let mut s = format!(
        "- Processor: {}, {} threads\n- System: {}\n- Build: {}\n- Shot: the reference shot, \
         1920 by 1080, Color Key, Noise (animated) and then Key Cleaner on layer a, frames 0 to \
         210 by 30 on the processor at Full, median ms of the whole frame; {rounds} rounds, \
         Exact then Fast in each\n\n| Shot | Round | Exact | Fast |\n|---|---:|---:|---:|\n",
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
        .map(|(name, r)| {
            (
                *name,
                shot(Some(&cleaner(*r, "exact"))),
                shot(Some(&cleaner(*r, "fast"))),
            )
        })
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
    let out = std::env::var("B791_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| effect_table::repo("verification/B-791_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
