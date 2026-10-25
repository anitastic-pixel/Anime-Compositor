//! W8 (wave 8, lane d): the time whole shots take, beyond one effect on one layer: many layers,
//! effect chains, nesting, 4K, export, the viewer's caches while editing, opening and saving.
//! A measurement, run deliberately with
//! `cargo test --release --test w8_arch_audit -- --ignored --nocapture`, the shots named in the
//! file `W8_CASES` (one `name|project|media root|frames` a line), written to `W8_OUT`.
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::export::{self, ExportChoices, ExportRequest, MissingSource, OutputFormat};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist::{self, Preserved};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{OutputAlpha, OutputDepth};

fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}

struct Case {
    name: String,
    project: Project,
    comp: Id,
    root: PathBuf,
    frames: Vec<i32>,
}

fn cases() -> Vec<Case> {
    let list = std::fs::read_to_string(std::env::var("W8_CASES").expect("W8_CASES")).unwrap();
    list.lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .map(|l| {
            let p: Vec<&str> = l.split('|').collect();
            let text = std::fs::read_to_string(p[1]).unwrap();
            let project = persist::load_str(&text)
                .unwrap_or_else(|d| panic!("{}: {}", p[1], d.message))
                .document
                .project()
                .clone();
            let comp = project.compositions[0].id.clone();
            Case {
                name: p[0].to_string(),
                project,
                comp,
                root: PathBuf::from(p[2]),
                frames: p[3].split(',').map(|f| f.parse().unwrap()).collect(),
            }
        })
        .collect()
}

fn cpu_frame(c: &Case, f: i32, q: PreviewQuality, cache: &mut CelCache) -> f64 {
    let mut log = FrameLog::new(3);
    let t = Instant::now();
    let b = preview::preview_frame_cached(
        &c.project, &c.comp, f, &c.root, q, DEFAULT_TILE_SIZE, &mut log, cache,
    )
    .expect("CPU frame");
    drop(b.to_srgb8_straight());
    ms(t)
}

fn card_frame(c: &Case, f: i32, q: PreviewQuality, cache: &mut CelCache, gpu: &mut Gpu) -> (f64, bool) {
    let mut log = FrameLog::new(3);
    let t = Instant::now();
    drop(
        preview::preview_frame_srgb8(
            &c.project, &c.comp, f, &c.root, q, DEFAULT_TILE_SIZE, &mut log, cache, gpu,
        )
        .expect("card frame"),
    );
    let took = ms(t);
    let fell = log
        .finish()
        .iter()
        .any(|d| format!("{:?}", d.id).contains("Gpu"));
    (took, fell)
}

/// Effects in the plan made for the card: (left to the card, all of them).
fn on_card(c: &Case, f: i32) -> (usize, usize) {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(
        &c.project,
        &c.comp,
        f,
        &c.root,
        PreviewQuality::Full,
        &mut log,
        &mut CelCache::viewer(),
    )
    .expect("plan");
    let card: usize = plan.layers.iter().map(|l| l.on_card.len()).sum();
    let all: usize = c
        .project
        .composition(&c.comp)
        .unwrap()
        .layers_in_order()
        .map(|l| l.effects.iter().filter(|e| e.enabled).count())
        .sum();
    (card, all)
}

fn export_ms(c: &Case, missing: MissingSource, frames: i32, dir: &Path) -> f64 {
    let _ = std::fs::remove_dir_all(dir);
    std::fs::create_dir_all(dir).unwrap();
    let request = ExportRequest {
        composition: c.comp.clone(),
        first_frame: 0,
        last_frame: frames - 1,
        output_dir: dir.to_path_buf(),
        naming: "f_%04d.png".into(),
        depth: OutputDepth::Eight,
        alpha: OutputAlpha::Straight,
        tile_size: DEFAULT_TILE_SIZE,
        missing,
        format: OutputFormat::Png,
        choices: ExportChoices::default(),
    };
    let t = Instant::now();
    let r = export::export_sequence(&c.project, &c.root, &request, &AtomicBool::new(false));
    let took = ms(t);
    assert!(r.succeeded(), "{}: {:?}", c.name, r.diagnostics);
    took
}

#[test]
#[ignore = "W8: a measurement, run deliberately with --release --ignored"]
fn w8_arch_timing() {
    let out = PathBuf::from(std::env::var("W8_OUT").expect("W8_OUT"));
    let tmp = out.with_extension("export");
    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- Build: {}\n\n| Shot | Effects on card | \
         CPU Draft | CPU Full | Card Draft | Card Full | Card fell back | Export frame (render_frame) \
         | Export 8 frames, Block | Export 8 frames, RenderTransparent |\n|---|---|---:|---:|---:|---:|---|---:|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_default(),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    for c in cases() {
        let mut cache = CelCache::with_budget(8 << 30);
        let (card, all) = on_card(&c, c.frames[0]);
        let mut row = format!("| {} | {card} of {all} |", c.name);
        // One frame first, not counted, so cels are read and the card's passes made.
        for q in [PreviewQuality::Draft, PreviewQuality::Full] {
            cpu_frame(&c, c.frames[0] + 1, q, &mut cache);
            let t: Vec<f64> = c.frames.iter().map(|&f| cpu_frame(&c, f, q, &mut cache)).collect();
            row.push_str(&format!(" {:.1} |", med(t)));
        }
        let mut fell = false;
        for q in [PreviewQuality::Draft, PreviewQuality::Full] {
            gpu.forget();
            card_frame(&c, c.frames[0] + 1, q, &mut cache, &mut gpu);
            let t: Vec<f64> = c
                .frames
                .iter()
                .map(|&f| {
                    let (t, f) = card_frame(&c, f, q, &mut cache, &mut gpu);
                    fell |= f;
                    t
                })
                .collect();
            row.push_str(&format!(" {:.1} |", med(t)));
        }
        row.push_str(if fell { " yes |" } else { " no |" });
        let t: Vec<f64> = c
            .frames
            .iter()
            .map(|&f| {
                let mut log = FrameLog::new(3);
                let t = Instant::now();
                drop(
                    compose::render_frame(&c.project, &c.comp, f, &c.root, DEFAULT_TILE_SIZE, &mut log)
                        .unwrap(),
                );
                ms(t)
            })
            .collect();
        row.push_str(&format!(" {:.1} |", med(t)));
        if std::env::var("W8_NO_EXPORT").is_err() {
            let n = (c.project.composition(&c.comp).unwrap().duration_frames as i32).min(8);
            for m in [MissingSource::Block, MissingSource::RenderTransparent] {
                row.push_str(&format!(" {:.0} ({n}) |", export_ms(&c, m, n, &tmp)));
            }
        } else {
            row.push_str(" - | - |");
        }
        row.push('\n');
        eprint!("{row}");
        s.push_str(&row);
        std::fs::write(&out, &s).unwrap();
    }
    let _ = std::fs::remove_dir_all(&tmp);
}

/// Opening and saving: `persist::load_str` and `persist::to_json` of each case's file, 5 times.
#[test]
#[ignore = "W8: a measurement, run deliberately with --release --ignored"]
fn w8_open_save() {
    let list = std::fs::read_to_string(std::env::var("W8_CASES").expect("W8_CASES")).unwrap();
    for l in list.lines().filter(|l| !l.trim().is_empty() && !l.starts_with('#')) {
        let p: Vec<&str> = l.split('|').collect();
        let text = std::fs::read_to_string(p[1]).unwrap();
        let (mut open, mut save) = (Vec::new(), Vec::new());
        for _ in 0..5 {
            let t = Instant::now();
            let loaded = persist::load_str(&text).unwrap();
            open.push(ms(t));
            let t = Instant::now();
            drop(persist::to_json(loaded.document.project(), &Preserved::none()));
            save.push(ms(t));
        }
        eprintln!(
            "| {} | {} KB | open {:.1} ms | save text {:.1} ms |",
            p[0],
            text.len() / 1024,
            med(open),
            med(save)
        );
    }
}

/// Where one frame goes, by the perf stages: `W8_CASES` shots, frame list's first, at Full on
/// the processor (preview with a viewer cache) and as an export draws it.
#[test]
#[ignore = "W8: a measurement, run deliberately with --release --ignored"]
fn w8_stages() {
    anime_compositor::perf::enable();
    for c in cases() {
        let f = c.frames[0];
        let mut cache = CelCache::with_budget(8 << 30);
        cpu_frame(&c, f + 1, PreviewQuality::Full, &mut cache);
        for how in ["viewer CPU", "export"] {
            anime_compositor::perf::reset();
            let t = Instant::now();
            if how == "export" {
                let mut log = FrameLog::new(3);
                drop(compose::render_frame(&c.project, &c.comp, f, &c.root, DEFAULT_TILE_SIZE, &mut log));
            } else {
                cpu_frame(&c, f, PreviewQuality::Full, &mut cache);
            }
            let mut line = format!("{} {how} {:.1} ms:", c.name, ms(t));
            for (s, nanos, calls) in anime_compositor::perf::snapshot() {
                if nanos > 2_000_000 {
                    line.push_str(&format!(" {}={:.1}x{calls}", s.label(), nanos as f64 / 1e6));
                }
            }
            eprintln!("{line}");
        }
    }
}

