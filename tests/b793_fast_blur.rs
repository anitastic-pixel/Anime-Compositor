//! B-793: D-914, Quality Fast on the big blurs inside Glow, Bloom, Deband and Halation (wave
//! 11, pick 9, step 4).
//!
//! Every expected pixel of Fast is in `Fixtures/{glow,bloom,deband,halation}/expected_*_fast.json`,
//! written by `tools/fast_blur_reference.py` before this code existed. Exact, the default, is
//! each effect's own table, unchanged.

mod effect_table;

use std::fs;

use effect_table::{median, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::Command;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// The first effect of the main composition, on whichever layer holds it.
fn effect_of(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    let comp = d.project().composition(&Id::new(MAIN)).unwrap();
    let e = comp
        .layers_in_order()
        .find_map(|l| l.effects.first())
        .unwrap()
        .effect
        .clone();
    e
}

fn quality_mut(e: &mut Effect) -> Option<&mut String> {
    match e {
        Effect::Glow { look, .. } => Some(&mut look.quality),
        Effect::Bloom { quality, .. }
        | Effect::Deband { quality, .. }
        | Effect::Halation { quality, .. } => Some(quality),
        _ => None,
    }
}

fn quality_of(e: &Effect) -> String {
    quality_mut(&mut e.clone()).map_or("not one of the four".into(), |q| q.clone())
}

fn is_glow(e: &Effect) -> bool {
    matches!(e, Effect::Glow { .. })
}
fn is_bloom(e: &Effect) -> bool {
    matches!(e, Effect::Bloom { .. })
}
fn is_deband(e: &Effect) -> bool {
    matches!(e, Effect::Deband { .. })
}
fn is_halation(e: &Effect) -> bool {
    matches!(e, Effect::Halation { .. })
}

fn glow(radius: f64, quality: &str) -> J {
    let mut p = json!({"based_on": "bright", "threshold": 60, "colors": [], "tolerance": 0,
        "radius": radius, "intensity": 1, "operation": "screen", "tint": ""});
    if quality != "exact" {
        p["quality"] = J::from(quality);
    }
    p
}

fn bloom(radius: f64, quality: &str) -> J {
    let mut p = json!({"threshold": 80, "radius": radius, "intensity": 1, "streaks": "star",
        "length": 60, "angle": 0});
    if quality != "exact" {
        p["quality"] = J::from(quality);
    }
    p
}

fn deband(blur: f64, quality: &str) -> J {
    let mut p = json!({"view": "normal", "edge_threshold": 4, "edge_softness": 2,
        "blur_amount": blur, "edge_blur_reduction": 100, "diffusion_amount": 1,
        "edge_diffusion_reduction": 100, "diffusion_reseed_interval": 1, "noise_amount": 1,
        "edge_noise_reduction": 100, "noise_reseed_interval": 1});
    if quality != "exact" {
        p["quality"] = J::from(quality);
    }
    p
}

fn halation(radius: f64, quality: &str) -> J {
    let mut p = json!({"threshold": 60, "threshold_smooth": 50, "radius": radius, "amount": 100,
        "color": "#ff4020"});
    if quality != "exact" {
        p["quality"] = J::from(quality);
    }
    p
}

/// The reference shot (1920 by 1080) with an animated Noise and then the effect on layer `a`
/// only, so nothing is kept from one frame to the next, as W7 measures one layer. Deband has
/// no Noise before it: noise is what it keeps, so on a noisy layer it changes next to nothing.
/// `enabled` false gives the shot without the effect.
fn shot_with(type_id: &str, parameters: &J, enabled: bool) -> Project {
    effect_table::reference(|id| {
        if id != "a" {
            return json!([]);
        }
        let mut v = json!([]);
        if type_id != "core.deband" {
            v.as_array_mut().unwrap().push(json!(
            {"instance_id": "b793-n", "type_id": "core.noise", "enabled": true,
             "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}}));
        }
        v.as_array_mut()
            .unwrap()
            .push(json!({"instance_id": "b793-e", "type_id": type_id,
            "enabled": enabled, "parameters": parameters}));
        v
    })
}

fn shot(type_id: &str, parameters: &J) -> Project {
    shot_with(type_id, parameters, true)
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

/// One effect's part of the table: its fixtures, its files, its commands, its tiles and the card.
#[allow(clippy::too_many_arguments)]
fn part(
    t: &mut Table,
    gpu: &mut Gpu,
    name: &str,
    folder: &str,
    expected: &str,
    files: &[&str],
    older: &str,
    medium_file: &str,
    medium_says: &str,
    numbers_off_card: &[u32],
    mine: fn(&Effect) -> bool,
) {
    t.root = effect_table::repo(&format!("Fixtures/{folder}"));
    t.heading(&format!("{name}: the fixtures (document 25)"));
    t.fixtures(expected);

    t.heading(&format!("{name}: the file"));
    t.round_trips(files);
    t.round_trips(&[older]);
    let e = effect_of(t, older);
    let params = t.saved_parameters(older);
    t.row(
        &format!("{older}, an older file with no quality: opens as Exact and saves no quality"),
        &format!("quality {:?}; saved {params}", quality_of(&e)),
        quality_of(&e) == "exact" && params.get("quality").is_none(),
    );
    let params = t.saved_parameters(files[1]);
    t.row(
        &format!("{}: Fast is saved as \"fast\"", files[1]),
        &params.to_string(),
        params.get("quality") == Some(&J::from("fast")),
    );
    let why = effect_of(t, medium_file).why_invalid();
    t.row(
        &format!("{medium_file} is refused in a sentence naming it"),
        &why,
        why == medium_says,
    );

    t.heading(&format!("{name}: commands"));
    let mut document = t.load(older).document;
    // Deband's files hold it on a layer `holder`, the others on `art`.
    let layer = if folder == "deband" { "holder" } else { "art" };
    let with = |q: &str| {
        let mut e = effect_of(t, older);
        *quality_mut(&mut e).unwrap() = q.into();
        Command::SetEffectParameters {
            composition: Id::new(MAIN),
            layer_id: Id::new(layer),
            instance_id: Id::new("fx-0-0"),
            effect: e,
        }
    };
    let (medium, fast) = (with("medium"), with("fast"));
    let held = format!("{:?}", document.project());
    let why = document.apply(medium).err();
    let untouched = format!("{:?}", document.project()) == held;
    t.row(
        &format!("{name}'s quality \"medium\" is refused with a sentence, and nothing changes"),
        &why.as_ref().map_or("taken".into(), |d| d.message.clone()),
        why.is_some() && untouched,
    );
    let ok = document.apply(fast).is_ok();
    t.row(
        &format!("{name}'s quality Fast is taken"),
        if ok { "taken" } else { "refused" },
        ok,
    );
    let before = t.render(&t.load(older).document, 0, 64);
    document.undo();
    let same = t.render(&document, 0, 64).data() == before.data();
    t.row(
        "undo once: frame 0 is the frame it was",
        if same { "byte-identical" } else { "differ" },
        same,
    );

    t.heading(&format!("{name}: the frame does not depend on how it is cut up"));
    let tiles: Vec<(&str, i32)> = files[1..3].iter().map(|f| (*f, 0)).collect();
    t.tiles(&tiles);

    t.heading(&format!("{name}: the card and the processor, within 1 level of 255"));
    t.card_fixtures(gpu, expected, numbers_off_card, mine);
}

#[test]
fn b793_fast_blur() {
    let mut t = Table::new(
        "glow",
        "# B-793: Quality Fast on the big blurs inside Glow, Bloom, Deband and Halation\n\n\
         D-914 (wave 11, pick 9, step 4): a setting Quality, Exact (the default, as always) or \
         Fast. Fast blurs a picture reduced to cells of 2, 4 or 8 pixels and reads it back \
         smoothly (as Nuke's Blur quality), only where the blur is wide enough that a cell is at \
         most an eighth of it; below sigma 8 Fast is Exact. Glow and Bloom blur the reduced \
         picture; Deband reads a tap every cell through its edges; Halation's levels are one \
         cell coarser. Every expected pixel of Fast is `Fixtures/*/expected_*_fast.json`, \
         written by `tools/fast_blur_reference.py` before this code existed and printed in \
         document 25 as FX-GLOW-034 to 037, FX-BLOOM-029 to 032, FX-DEBAND-025 to 028 and \
         FX-HALO-019 to 022, compared sample by sample against the tolerance of 2e-5. Exact is \
         each effect's own table, unchanged. The card has no reduced blur for Glow, Bloom and \
         Deband, so at Fast they are drawn on the processor and the card draws what follows \
         them; Halation's Fast is on the card.\n",
    );
    let mut gpu = Gpu::new().expect("a usable card");
    part(
        &mut t,
        &mut gpu,
        "Glow",
        "glow",
        "expected_glow_fast.json",
        &["fx_glow_034.json", "fx_glow_035.json", "fx_glow_036.json", "fx_glow_037.json"],
        "fx_glow_001.json",
        "fx_glow_037.json",
        "Glow's quality is \"exact\", \"fast\", and this is \"medium\".",
        &[34, 35, 36, 37],
        is_glow,
    );
    part(
        &mut t,
        &mut gpu,
        "Bloom",
        "bloom",
        "expected_bloom_fast.json",
        &["fx_bloom_029.json", "fx_bloom_030.json", "fx_bloom_031.json", "fx_bloom_032.json"],
        "fx_bloom_010.json",
        "fx_bloom_032.json",
        "Bloom's quality is \"exact\" or \"fast\", and this is \"medium\".",
        &[29, 30, 31, 32],
        is_bloom,
    );
    part(
        &mut t,
        &mut gpu,
        "Deband",
        "deband",
        "expected_deband_fast.json",
        &["fx_deband_025.json", "fx_deband_026.json", "fx_deband_027.json", "fx_deband_028.json"],
        "fx_deband_001.json",
        "fx_deband_028.json",
        "Deband's quality is \"exact\" or \"fast\", and this is \"medium\".",
        &[25, 26, 27, 28],
        is_deband,
    );
    part(
        &mut t,
        &mut gpu,
        "Halation",
        "halation",
        "expected_halation_fast.json",
        &["fx_halo_019.json", "fx_halo_020.json", "fx_halo_021.json", "fx_halo_022.json"],
        "fx_halo_001.json",
        "fx_halo_022.json",
        "Halation's quality is \"exact\" or \"fast\", and this is \"medium\".",
        &[22],
        is_halation,
    );

    t.heading("Halation at Fast on the card, the reference shot");
    let plain = persist::load(&effect_table::repo("verification/B-08a_project.json"))
        .expect("the reference shot")
        .document
        .project()
        .clone();
    t.card_reference(
        &mut gpu,
        &plain,
        &[(
            "Halation at radius 100, Fast, on layer a",
            shot("core.halation", &halation(100.0, "fast")),
            1,
        )],
        is_halation,
    );

    t.heading("Fast against Exact, the reference shot at 1920 by 1080, frame 100 at Full (an export's frame)");
    let pictures = effect_table::repo("verification/B-793 pictures");
    fs::create_dir_all(&pictures).unwrap();
    let cases: [(&str, &str, J, J, &str); 6] = [
        (
            "Glow at radius 60 (sigma 20: cells 4 by 4)",
            "core.glow",
            glow(60.0, "exact"),
            glow(60.0, "fast"),
            "glow_radius_60",
        ),
        (
            "Bloom at radius 100 with a star (cells 8, 4, 2 and 1)",
            "core.bloom",
            bloom(100.0, "exact"),
            bloom(100.0, "fast"),
            "bloom_radius_100",
        ),
        (
            "Deband at blur 60 (sigma 20: a tap every 4 pixels)",
            "core.deband",
            deband(60.0, "exact"),
            deband(60.0, "fast"),
            "deband_blur_60",
        ),
        (
            "Halation at radius 100 (each level one cell coarser)",
            "core.halation",
            halation(100.0, "exact"),
            halation(100.0, "fast"),
            "halation_radius_100",
        ),
        (
            "Glow at radius 21 (sigma 7: Fast is Exact below sigma 8)",
            "core.glow",
            glow(21.0, "exact"),
            glow(21.0, "fast"),
            "",
        ),
        (
            "Deband at blur 16 (sigma 5.33: Fast is Exact below sigma 8)",
            "core.deband",
            deband(16.0, "exact"),
            deband(16.0, "fast"),
            "",
        ),
    ];
    for (what, type_id, exact, fast, picture) in cases {
        let (e, f) = (
            cpu_frame(&shot(type_id, &exact), 100),
            cpu_frame(&shot(type_id, &fast), 100),
        );
        let without = cpu_frame(&shot_with(type_id, &exact, false), 100);
        let changed = effect_table::distance(&e, &without).1;
        let (largest, pixels, mean) = apart(&e, &f);
        let below_8 = picture.is_empty();
        t.row(
            &format!("{what}: Exact changes the shot, and Fast against Exact (close: under a tenth of a level on average)"),
            &format!(
                "Exact changes {changed} pixels; Fast differs from Exact by at most {largest} of 255, in {pixels} of 2,073,600 pixels, {mean:.4} levels on average"
            ),
            changed > 10_000 && if below_8 { largest == 0 } else { largest > 0 && mean < 0.1 },
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

    t.finish("B-793_fast_blur_table.md");
}

/// Exact and Fast in alternating rounds: the reference shot, Noise and the effect on layer a
/// (1920 by 1080), frames 0 to 210 by 30 asked for on the processor at Full, an export's frame.
/// One frame before each run is not counted. `B793_ROUNDS` rounds (5 by default), written to
/// `B793_OUT`.
#[test]
#[ignore = "B-793: a measurement, run deliberately with --release --ignored"]
fn b793_fast_blur_timing() {
    use std::fmt::Write as _;
    let rounds: usize = std::env::var("B793_ROUNDS")
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
    let shots: [(&str, &str, J, J); 8] = [
        ("Glow, radius 30 (sigma 10)", "core.glow", glow(30.0, "exact"), glow(30.0, "fast")),
        ("Glow, radius 60 (sigma 20)", "core.glow", glow(60.0, "exact"), glow(60.0, "fast")),
        ("Bloom, radius 30, star", "core.bloom", bloom(30.0, "exact"), bloom(30.0, "fast")),
        ("Bloom, radius 100, star", "core.bloom", bloom(100.0, "exact"), bloom(100.0, "fast")),
        ("Deband, blur 30 (sigma 10)", "core.deband", deband(30.0, "exact"), deband(30.0, "fast")),
        ("Deband, blur 60 (sigma 20)", "core.deband", deband(60.0, "exact"), deband(60.0, "fast")),
        ("Halation as added (radius 25)", "core.halation", halation(25.0, "exact"), halation(25.0, "fast")),
        ("Halation, radius 100", "core.halation", halation(100.0, "exact"), halation(100.0, "fast")),
    ];
    let mut s = format!(
        "- Processor: {}, {} threads\n- System: {}\n- Build: {}\n- Shot: the reference shot, \
         1920 by 1080, Noise (animated) and then the effect on layer a, frames 0 to 210 by 30 on the \
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
    let out = std::env::var("B793_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| effect_table::repo("verification/B-793_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
