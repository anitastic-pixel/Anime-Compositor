//! B-52: Repeat Edge Pixels (D-109) on the graphics card, and the check that the three blurs
//! repeat their edges there as the CPU does; and the pictures a person judges it by.
//!
//! What is compared is what the page receives, eight-bit straight sRGB: the CPU's frame through
//! `preview_frame_cached`, and the card's through `preview_frame_srgb8`, whose plan leaves a
//! layer's last Radial, Directional or Gaussian Blur to the card. The CPU stays the authority
//! (ADR-006, D-100).
//!
//! Writes `verification/B-52_gpu_edges_table.md` and, for the worst frame, three pictures
//! `card_*.png` in `verification/B-52 pictures/`. `b52_pictures` writes the before and after
//! pictures beside them.

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
use anime_compositor::render;
use anime_compositor::OutputDepth;
use serde_json::json;

mod common;
use common::repo;

/// The tolerance each blur already has on the card (D-103, D-106, D-107), in levels of 255.
const LIMIT: u8 = 1;

struct Shot {
    name: String,
    project: Project,
    root: PathBuf,
    comp: Id,
    frames: Vec<i32>,
}

/// The reference shot with `effect` on its first layer, the background plate, which fills the
/// frame, at `edges`.
fn plate(name: &str, effect: serde_json::Value, edges: &str) -> Shot {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: serde_json::Value = serde_json::from_str(&text).expect("the reference shot is JSON");
    let mut effect = effect;
    effect["parameters"]["edges"] = json!(edges);
    j["compositions"][0]["layers"][0]["effects"] = json!([effect]);
    let loaded = persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("{name}: {}", d.message));
    Shot {
        name: format!("the reference shot with {name}, edges {edges}, on the background"),
        project: loaded.document.project().clone(),
        root: repo("Fixtures/reference_shot"),
        comp: Id::new("comp-reference-shot"),
        frames: vec![0, 239],
    }
}

/// The three blurs, as the pictures and the card's check put them on the background.
fn blurs() -> Vec<(&'static str, &'static str, serde_json::Value)> {
    let fx = |type_id: &str, parameters: serde_json::Value| {
        json!({"instance_id": "b52", "type_id": type_id, "enabled": true, "parameters": parameters})
    };
    vec![
        ("gaussian", "a Gaussian Blur of sigma 10", fx("core.gaussian_blur", json!({"sigma_px": 10.0}))),
        ("directional", "a Directional Blur at 45 degrees, 60 long", fx("core.directional_blur", json!({"direction": 45.0, "length": 60.0}))),
        // A spin: since D-110 a zoom about a centre inside the picture samples only inside it,
        // so its edges no longer fade.
        ("radial", "a Radial Blur, spin 30 about the middle", fx("core.radial_blur", json!({"type": "spin", "amount": 30.0, "center": [50.0, 50.0]}))),
    ]
}

/// FX-EDGES-001 to 009, each at every frame it has.
fn fixtures() -> Vec<Shot> {
    (1..=9)
        .map(|i| {
            let name = format!("fx_edges_{i:03}");
            let loaded = persist::load(&repo(&format!("Fixtures/edges/{name}.json"))).unwrap_or_else(|d| panic!("{name}: {}", d.message));
            let project = loaded.document.project().clone();
            let comp = &project.compositions[0];
            let (id, frames) = (comp.id.clone(), (comp.start_frame..comp.start_frame + comp.duration_frames as i32).collect());
            Shot { name, project, root: repo("Fixtures/edges"), comp: id, frames }
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

/// How many layers of the card's plan have a Radial, Directional or Gaussian Blur left for the card.
fn left(shot: &Shot, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    compose::plan_frame_for_card(&shot.project, &shot.comp, frame, &shot.root, quality, &mut log, &mut CelCache::viewer())
        .expect("plan the frame")
        .layers
        .iter()
        .filter(|l| {
            use anime_compositor::render::OnCard;
            l.on_card.iter().any(|c| matches!(c, OnCard::Radial(_) | OnCard::Directional(_) | OnCard::Gaussian(_)))
        })
        .count()
}

#[test]
fn b52_gpu_edges() {
    let out = repo("verification/B-52_gpu_edges_table.md");
    let mut gpu = match Gpu::new() {
        Ok(gpu) => gpu,
        Err(why) => {
            fs::write(&out, format!("# B-52: Repeat Edge Pixels on the GPU\n\n**NOT RUN.** No usable card: {why}\n\nNo check in this table was run, so none of them passes.\n"))
                .expect("write the B-52 table");
            return;
        }
    };
    let (mut rows, mut checks, mut passed) = (String::new(), 0, 0);
    let mut worst: Option<((u8, usize), String, Vec<u8>, Vec<u8>, usize, usize)> = None;
    let mut shots = fixtures();
    shots.extend(blurs().into_iter().map(|(_, name, effect)| plate(name, effect, "repeat")));
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
    for reference in &shots[shots.len() - 3..] {
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
    }

    let pictures = repo("verification/B-52 pictures");
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
    for (name, bytes) in [("card_cpu.png", &c), ("card_gpu.png", &g), ("card_difference.png", &diff)] {
        png_out::write_rgba(&pictures.join(name), w, h, OutputDepth::Eight, &[], bytes).expect("write a picture");
    }

    let s = format!(
        "# B-52: Repeat Edge Pixels on the GPU against the CPU\n\n\
         Written by `tests/b52_gpu_edges.rs`. The card: {}.\n\n\
         Each row compares the eight-bit picture the page receives, drawn by the CPU and by the \
         GPU, with the layer's last blur, repeating its edges (D-109), done on the card. **The \
         rule: no channel of any pixel more than {LIMIT} level of 255 apart**, the tolerance each \
         of the three blurs already has there (D-103, D-106, D-107). FX-EDGES-009's first blur \
         runs on the CPU, before the spin the card does. On every row both paths must give the \
         same warnings.\n\n\
         **{passed} of {checks} checks pass.**\n\n\
         The worst comparison is \"{worst_case}\": largest difference {largest} of 255, pixels differing: {count}. \
         Its pictures are in `verification/B-52 pictures/`: `card_cpu.png`, `card_gpu.png`, and \
         `card_difference.png`, black where the two agree and a white 7 by 7 square around every \
         pixel where they do not.\n\n\
         | Case | Blurs left to the card | Largest difference (of 255) | Pixels differing | Warnings | Result |\n|---|---:|---:|---:|---|---|\n{rows}",
        gpu.about(),
    );
    fs::write(&out, s).expect("write the B-52 table");
    assert_eq!(passed, checks, "B-52: {passed} of {checks} checks pass");
}

/// The before and after pictures: frame 0 of the reference shot with each blur on the background,
/// its edges transparent and then repeated, drawn by the CPU as an export would be, and how many
/// of each picture's pixels are anything less than solid.
#[test]
fn b52_pictures() {
    let pictures = repo("verification/B-52 pictures");
    fs::create_dir_all(&pictures).expect("make the pictures folder");
    let mut rows = String::new();
    let mut ok = true;
    for (file, name, effect) in blurs() {
        let mut see_through = Vec::new();
        for edges in ["transparent", "repeat"] {
            let shot = plate(name, effect.clone(), edges);
            let mut log = FrameLog::new(3);
            let frame = preview::preview_frame_cached(&shot.project, &shot.comp, 0, &shot.root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
                .unwrap_or_else(|d| panic!("{}: {}", shot.name, d.message));
            let (w, h) = (frame.width(), frame.height());
            let bytes = frame.to_srgb8_straight();
            see_through.push(bytes.chunks_exact(4).filter(|p| p[3] < 255).count());
            // Over a grey checkerboard, so what is see-through shows in any viewer.
            let shown: Vec<u8> = bytes
                .chunks_exact(4)
                .enumerate()
                .flat_map(|(i, p)| {
                    let (x, y) = (i % w, i / w);
                    let back = if (x / 32 + y / 32) % 2 == 0 { 96.0 } else { 160.0 };
                    let a = p[3] as f64 / 255.0;
                    let mix = |c: u8| (c as f64 * a + back * (1.0 - a)).round() as u8;
                    [mix(p[0]), mix(p[1]), mix(p[2]), 255]
                })
                .collect();
            png_out::write_rgba(&pictures.join(format!("{file}_{edges}.png")), w, h, OutputDepth::Eight, &[], &shown).expect("write a picture");
        }
        let pass = see_through[0] > 0 && see_through[1] == 0;
        ok &= pass;
        let _ = writeln!(
            rows,
            "| {name} | `{file}_transparent.png` | {} | `{file}_repeat.png` | {} | {} |",
            see_through[0],
            see_through[1],
            if pass { "PASS" } else { "FAIL" }
        );
    }
    let s = format!(
        "# B-52: the before and after pictures\n\n\
         Written by `tests/b52_gpu_edges.rs` (`b52_pictures`). Frame 0 of the reference shot, whose \
         background fills the frame, with one blur on the background: its edges left transparent, as \
         before D-109, and then repeated. Drawn by the CPU, as an export is, and shown over a grey checkerboard, which shows through \
         wherever the picture is see-through. The pictures are in \
         `verification/B-52 pictures/`. **The rule: with the edges transparent some pixels are see-through, \
         and with them repeated none is.**\n\n\
         | Blur | Transparent | Pixels see-through | Repeat Edge Pixels | Pixels see-through | Result |\n|---|---|---:|---|---:|---|\n{rows}"
    );
    fs::write(repo("verification/B-52_pictures_table.md"), s).expect("write the pictures table");
    assert!(ok, "B-52: a picture with repeated edges has see-through pixels, or one without has none");
}
