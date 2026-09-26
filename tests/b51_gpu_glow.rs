//! B-51: Glow on the graphics card, and the check that it glows as the CPU does.
//!
//! What is compared is what the page receives, eight-bit straight sRGB: the CPU's frame through
//! `preview_frame_cached`, and the card's through `preview_frame_srgb8`, whose plan leaves a
//! layer's last Glow to the card. The CPU stays the authority (ADR-006, D-100).
//!
//! Each row also says whether the Glow was in fact left to the card, since a row where it was
//! not compares the CPU with itself. A Glow with nothing that glows, or at intensity 0, is not
//! left (it changes nothing), nor is one whose settings are invalid (the CPU reports and skips
//! it); on those rows any difference is the card's layering, which D-100 holds to 1 level.
//!
//! Writes `verification/B-51_gpu_glow_table.md` and, for the worst frame, three pictures in
//! `verification/B-51 pictures/`. `b51_gpu_glow_timing`, run deliberately in release, writes
//! `verification/B-51_gpu_glow_timing_table.md`.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::png_out;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::render;
use anime_compositor::OutputDepth;
use serde_json::json;

mod common;
use common::repo;

/// D-108's proposed tolerance, in levels of 255.
const LIMIT: u8 = 1;

struct Shot {
    name: String,
    project: Project,
    root: PathBuf,
    comp: Id,
    frames: Vec<i32>,
}

/// The reference shot with Glows added: a wide glow of the bright parts, added, on the first
/// layer; a glow of the second's one colour, tinted and screened; and a glow with no blur
/// at all on the third.
fn glowing_reference() -> Shot {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: serde_json::Value = serde_json::from_str(&text).expect("the reference shot is JSON");
    let glow = |id: &str, parameters: serde_json::Value| {
        json!({"instance_id": id, "type_id": "core.glow", "enabled": true, "parameters": parameters})
    };
    let layers = &mut j["compositions"][0]["layers"];
    layers[0]["effects"] = json!([glow("b51-a", json!({"based_on": "bright", "threshold": 60, "colors": [], "tolerance": 0, "radius": 30, "intensity": 1, "operation": "add", "tint": ""}))]);
    layers[1]["effects"] = json!([glow(
        "b51-b",
        json!({"based_on": "colors", "threshold": 60, "colors": ["#e6285a"], "tolerance": 10, "radius": 12, "intensity": 1.5, "operation": "screen", "tint": "#ff8020"})
    )]);
    layers[2]["effects"] = json!([glow("b51-c", json!({"based_on": "bright", "threshold": 50, "colors": [], "tolerance": 0, "radius": 0, "intensity": 0.5, "operation": "add", "tint": ""}))]);
    let loaded = persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the glowing reference shot: {}", d.message));
    Shot {
        name: "the reference shot with three Glows".into(),
        project: loaded.document.project().clone(),
        root: repo("Fixtures/reference_shot"),
        comp: Id::new("comp-reference-shot"),
        frames: vec![0, 100, 239],
    }
}

/// FX-GLOW-001 to 033, each at every frame it has: every way a Glow is set, including the ones
/// the CPU reports as invalid.
fn fixtures() -> Vec<Shot> {
    (1..=33)
        .map(|n| {
            let name = format!("fx_glow_{n:03}");
            let loaded = persist::load(&repo(&format!("Fixtures/glow/{name}.json"))).unwrap_or_else(|d| panic!("{name}: {}", d.message));
            let project = loaded.document.project().clone();
            let comp = &project.compositions[0];
            let (id, frames) = (comp.id.clone(), (comp.start_frame..comp.start_frame + comp.duration_frames as i32).collect());
            Shot { name, project, root: repo("Fixtures/glow"), comp: id, frames }
        })
        .collect()
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

/// How many layers of the card's plan have a Glow left for the card.
fn left(shot: &Shot, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    compose::plan_frame_for_card(&shot.project, &shot.comp, frame, &shot.root, quality, &mut log, &mut CelCache::viewer())
        .expect("plan the frame")
        .layers
        .iter()
        .filter(|l| matches!(l.on_card, Some(anime_compositor::render::OnCard::Glow(_))))
        .count()
}

#[test]
fn b51_gpu_glow() {
    let out = repo("verification/B-51_gpu_glow_table.md");
    let mut gpu = match Gpu::new() {
        Ok(gpu) => gpu,
        Err(why) => {
            fs::write(&out, format!("# B-51: Glow on the GPU\n\n**NOT RUN.** No usable card: {why}\n\nNo check in this table was run, so none of them passes.\n"))
                .expect("write the B-51 table");
            return;
        }
    };
    let (mut rows, mut checks, mut passed) = (String::new(), 0, 0);
    let mut worst: Option<((u8, usize), String, Vec<u8>, Vec<u8>, usize, usize)> = None;
    let mut shots = fixtures();
    shots.push(glowing_reference());
    for shot in &shots {
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
                let n = left(shot, frame, quality);
                let d = distance(&c, &g);
                let pass = !on_cpu && said_cpu == said_gpu && d.0 <= LIMIT;
                checks += 1;
                passed += pass as usize;
                let case = format!("{} frame {frame}, {}", shot.name, quality.label());
                let _ = writeln!(
                    rows,
                    "| {case} | {n} | {} | {} | {} | {} |",
                    d.0,
                    d.1,
                    match (said_gpu.is_empty(), said_cpu == said_gpu) {
                        (true, true) => "none".to_string(),
                        (false, true) => format!("{said_gpu}, on both"),
                        (_, false) => format!("CPU: {said_cpu}; GPU: {said_gpu}"),
                    },
                    match (on_cpu, pass) {
                        (true, _) => "FAIL: the CPU drew it",
                        (false, true) => "PASS",
                        (false, false) => "FAIL",
                    }
                );
                if worst.as_ref().is_none_or(|w| d > w.0) {
                    worst = Some((d, case, c, g, w, h));
                }
            }
        }
    }

    // The CPU drawing a plan made for the card, as it does when the card refuses a frame, draws
    // the same frame as a plan made for the CPU, byte for byte.
    let reference = shots.last().expect("the reference shot is last");
    for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
        let mut log = FrameLog::new(3);
        let tile = match quality {
            PreviewQuality::Full => DEFAULT_TILE_SIZE,
            PreviewQuality::Draft => compose::DRAFT_TILE_SIZE,
        };
        let plan = |card: bool, log: &mut FrameLog| {
            let make = if card { compose::plan_frame_for_card } else { compose::plan_frame_at };
            let p = make(&reference.project, &reference.comp, 100, &reference.root, quality, log, &mut CelCache::viewer()).expect("plan");
            preview::scale_plan(p, quality)
        };
        let a = render::render(&plan(false, &mut log), tile);
        let b = render::render(&plan(true, &mut log), tile);
        let same = a.data() == b.data();
        checks += 1;
        passed += same as usize;
        let _ = writeln!(
            rows,
            "| {} frame 100, {}: the plan made for the card, drawn by the CPU | — | — | {} | — | {} |",
            reference.name,
            quality.label(),
            if same { "byte-identical".to_string() } else { "the frame differs".to_string() },
            if same { "PASS" } else { "FAIL" }
        );
    }

    let pictures = repo("verification/B-51 pictures");
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
        "# B-51: Glow on the GPU against the CPU\n\n\
         Written by `tests/b51_gpu_glow.rs`. The card: {}.\n\n\
         Each row compares the eight-bit picture the page receives, drawn by the CPU and by the \
         GPU, with the layer's last Glow done on the card. **The rule: no channel of any pixel \
         more than {LIMIT} level of 255 apart** (D-108, proposed). A Glow at intensity 0 or with \
         nothing that glows changes nothing, and one with invalid settings is reported and \
         skipped, so none of these is left to the card; on those rows any difference is the \
         card's layering, held to the same 1 level by D-100. On every row both paths must give the same warnings.\n\n\
         **{passed} of {checks} checks pass.**\n\n\
         The worst comparison is \"{worst_case}\": largest difference {largest} of 255, pixels differing: {count}. \
         Its pictures are in `verification/B-51 pictures/`: `cpu.png`, `gpu.png`, and \
         `difference.png`, black where the two agree and a white 7 by 7 square around every pixel \
         where they do not.\n\n\
         | Case | Glows left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |\n|---|---:|---:|---:|---|---|\n{rows}",
        gpu.about(),
    );
    fs::write(&out, s).expect("write the B-51 table");
    assert_eq!(passed, checks, "B-51: {passed} of {checks} checks pass");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

#[test]
#[ignore = "B-51: a measurement, run deliberately with --release --ignored"]
fn b51_gpu_glow_timing() {
    let mut gpu = Gpu::new().expect("a usable card");
    let shot = glowing_reference();
    let mut s = format!(
        "# B-51: frame times with three Glows, CPU and GPU\n\n\
         Written by `tests/b51_gpu_glow.rs` (`cargo test --release --test b51_gpu_glow -- \
         --ignored`).\n\n\
         - Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n\
         The shot is the reference shot with the three Glows of the B-51 table. Every \
         frame of it is asked for as the viewer asks, whole: planning, the effects, drawing, and \
         the eight-bit picture. On the CPU the three Glows run inside planning; on the GPU the \
         card runs them. Each path starts with empty caches and plays the shot twice: the first \
         loop fills the caches, the second is what playing it again costs. Medians over all 240 \
         frames, in ms.\n\n\
         | Quality | CPU, first loop | CPU, again | GPU, first loop | GPU, again |\n|---|---:|---:|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    for quality in [PreviewQuality::Draft, PreviewQuality::Full] {
        let tile = DEFAULT_TILE_SIZE;
        let mut row = format!("| {} |", quality.label());
        for on_card in [false, true] {
            let mut cache = CelCache::viewer();
            gpu.forget();
            for _ in 0..2 {
                let mut times = Vec::new();
                for frame in 0..240 {
                    let mut log = FrameLog::new(3);
                    let t = Instant::now();
                    if on_card {
                        drop(preview::preview_frame_srgb8(&shot.project, &shot.comp, frame, &shot.root, quality, tile, &mut log, &mut cache, &mut gpu).expect("GPU frame"));
                    } else {
                        drop(preview::preview_frame_cached(&shot.project, &shot.comp, frame, &shot.root, quality, tile, &mut log, &mut cache).expect("CPU frame").to_srgb8_straight());
                    }
                    times.push(t.elapsed().as_secs_f64() * 1000.0);
                }
                let _ = write!(row, " {:.1} |", median(times));
            }
        }
        let _ = writeln!(s, "{row}");
    }
    fs::write(repo("verification/B-51_gpu_glow_timing_table.md"), s).expect("write the timing table");
}
