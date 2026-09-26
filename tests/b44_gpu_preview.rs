//! B-44: the GPU draws the viewer's picture, and this is the check that it draws the CPU's.
//!
//! Written before the GPU code, as document 15 asks, so the rule was fixed first: at every pixel
//! of every frame checked, no channel of the GPU's eight-bit picture is more than 1 level of 255
//! from the CPU's (D-100's tolerance, from P-07's measurement). The CPU stays the authority;
//! this only says how far the viewer's GPU picture may be from it.
//!
//! What is compared is what the page receives: straight sRGB, eight bits, RGBA. The CPU side is
//! exactly the window's CPU path, `preview_frame_cached` and `to_srgb8_straight`.
//!
//! On a machine with no usable card the table says the check was not run and passes nothing.
//!
//! Writes `verification/B-44_gpu_preview_table.md` and, for the worst frame, three pictures in
//! `verification/B-44 pictures/`. `b44_gpu_timing`, run deliberately in release, writes
//! `verification/B-44b_gpu_timing_table.md`.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{BlendMode, Id, Project};
use anime_compositor::persist;
use anime_compositor::png_out;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::render::{self, Affine, FramePlan};
use anime_compositor::OutputDepth;

mod common;
use common::{build_fixture, repo};

const LIMIT: u8 = 1;

struct Shot {
    name: &'static str,
    project: Project,
    root: PathBuf,
    comp: Id,
}

fn shots() -> Vec<Shot> {
    let reference = persist::load(&repo("verification/B-08a_project.json"))
        .unwrap_or_else(|d| panic!("the reference shot: {}", d.message));
    let (declared, declared_root, _) = build_fixture("b44");
    vec![
        Shot {
            name: "the reference shot",
            project: reference.document.project().clone(),
            root: repo("Fixtures/reference_shot"),
            comp: Id::new("comp-reference-shot"),
        },
        Shot {
            name: "the ten-layer fixture",
            project: declared,
            root: declared_root,
            // `build_fixture` keeps the reference shot's composition id.
            comp: Id::new("comp-reference-shot"),
        },
    ]
}

/// Largest channel difference and the number of pixels with any difference.
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

/// The CPU's picture of a plan, as the page would receive it.
fn cpu(plan: &FramePlan) -> Vec<u8> {
    render::render(plan, DEFAULT_TILE_SIZE).to_srgb8_straight()
}

struct Table {
    rows: String,
    checks: usize,
    passed: usize,
    /// The worst comparison so far: its distance, its name, and CPU picture, GPU picture, size.
    worst: Option<((u8, usize), String, Vec<u8>, Vec<u8>, usize, usize)>,
}

impl Table {
    fn row(&mut self, case: &str, expected: &str, actual: String, pass: bool) {
        self.checks += 1;
        self.passed += pass as usize;
        let _ = writeln!(
            self.rows,
            "| {case} | {expected} | {actual} | {} |",
            if pass { "PASS" } else { "FAIL" }
        );
    }

    fn compare(&mut self, case: &str, cpu: Vec<u8>, gpu: Vec<u8>, size: (usize, usize)) {
        let d = distance(&cpu, &gpu);
        self.row(
            case,
            "at most 1 level",
            format!("largest difference {} of 255, pixels differing: {}", d.0, d.1),
            d.0 <= LIMIT,
        );
        if self.worst.as_ref().is_none_or(|w| d > w.0) {
            self.worst = Some((d, case.to_string(), cpu, gpu, size.0, size.1));
        }
    }
}

/// A frame through the window's own path, on the CPU and on the GPU.
fn both(gpu: &mut Gpu, shot: &Shot, frame: i32, quality: PreviewQuality) -> (Vec<u8>, Vec<u8>, (usize, usize), FrameLog) {
    let mut cache = CelCache::viewer();
    let mut log = FrameLog::new(3);
    let on_cpu = preview::preview_frame_cached(
        &shot.project, &shot.comp, frame, &shot.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache,
    )
    .unwrap_or_else(|d| panic!("{} frame {frame} on the CPU: {}", shot.name, d.message));
    let (w, h) = (on_cpu.width(), on_cpu.height());
    let on_cpu = on_cpu.to_srgb8_straight();
    let mut log = FrameLog::new(3);
    let (on_gpu, gw, gh) = preview::preview_frame_srgb8(
        &shot.project, &shot.comp, frame, &shot.root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, gpu,
    )
    .unwrap_or_else(|d| panic!("{} frame {frame} on the GPU: {}", shot.name, d.message));
    assert_eq!((w, h), (gw, gh));
    (on_cpu, on_gpu, (w, h), log)
}

fn plan(shot: &Shot, frame: i32, quality: PreviewQuality) -> FramePlan {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_at(
        &shot.project, &shot.comp, frame, &shot.root, quality, &mut log, &mut CelCache::viewer(),
    )
    .unwrap_or_else(|d| panic!("{} frame {frame}: {}", shot.name, d.message));
    preview::scale_plan(plan, quality)
}

/// Why the CPU drew the frame, if it did.
fn fell_back(log: FrameLog) -> Option<String> {
    log.finish()
        .into_iter()
        .find(|d| d.id == DiagnosticId::GpuPreviewOnCpu)
        .map(|d| format!("{} {}", d.message, d.detail))
}

#[test]
fn b44_gpu_preview() {
    let out = repo("verification/B-44_gpu_preview_table.md");
    let mut gpu = match Gpu::new() {
        Ok(gpu) => gpu,
        Err(why) => {
            fs::write(
                &out,
                format!(
                    "# B-44: the GPU's picture against the CPU's\n\n**NOT RUN.** No usable card: \
                     {why}\n\nNo check in this table was run, so none of them passes.\n"
                ),
            )
            .expect("write the B-44 table");
            return;
        }
    };
    let mut t = Table { rows: String::new(), checks: 0, passed: 0, worst: None };
    let shots = shots();
    let (reference, declared) = (&shots[0], &shots[1]);

    // P-07's four H-01 frames, which are the reference shot's.
    for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
        for frame in [0, 14, 100, 239] {
            let (c, g, size, log) = both(&mut gpu, reference, frame, quality);
            let case = format!("H-01 frame {frame}, {}", quality.label());
            if let Some(why) = fell_back(log) {
                panic!("{case} was drawn by the CPU: {why}");
            }
            t.compare(&case, c, g, size);
        }
    }

    // Both fixtures across the shot, every twelfth frame and the last.
    for shot in &shots {
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in (0..240).step_by(12).chain([239]) {
                let (c, g, size, log) = both(&mut gpu, shot, frame, quality);
                let case = format!("{} frame {frame}, {}", shot.name, quality.label());
                if let Some(why) = fell_back(log) {
                panic!("{case} was drawn by the CPU: {why}");
            }
                t.compare(&case, c, g, size);
            }
        }
    }

    // P-07's hard case: the whole stack turned 0.001 degrees about a point 8,388,608 pixels off
    // the canvas, where f32 has one pixel of precision left. The inverse is made in f64 on the
    // CPU, so the card only ever sees numbers near the picture.
    let far = 8_388_608.0;
    let mut hard = plan(reference, 100, PreviewQuality::Full);
    for layer in &mut hard.layers {
        layer.transform = layer
            .transform
            .then(Affine::translation(-far, -far))
            .then(Affine::rotation_degrees(0.001))
            .then(Affine::translation(far, far));
    }
    let size = (hard.width, hard.height);
    let g = gpu.draw(&hard, &CelCache::none()).unwrap_or_else(|d| panic!("the hard case on the GPU: {}", d.message));
    t.compare("P-07's hard case: frame 100 turned 0.001 deg about a point 8,388,608 px off canvas", cpu(&hard), g, size);

    // Document 21's four blend modes and opacity. The fixtures use only Normal at full opacity,
    // so the test changes the plan: every layer in one mode, every other layer at 60%. The
    // ten-layer fixture brings its two mattes and its blurred, tinted drawings with it.
    for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
        for mode in [BlendMode::Normal, BlendMode::Multiply, BlendMode::Screen, BlendMode::Add] {
            let mut p = plan(declared, 100, quality);
            for (i, layer) in p.layers.iter_mut().enumerate() {
                layer.blend = mode;
                if i % 2 == 1 {
                    layer.opacity = 0.6;
                }
            }
            let size = (p.width, p.height);
            let g = gpu.draw(&p, &CelCache::none()).unwrap_or_else(|d| panic!("{mode:?} on the GPU: {}", d.message));
            t.compare(
                &format!("ten-layer frame 100, {}, every layer {}, every other at 60%", quality.label(), mode.as_str()),
                cpu(&p),
                g,
                size,
            );
        }
    }

    // The same frame drawn twice gives the same bytes: nothing is left over between frames.
    let p = plan(declared, 100, PreviewQuality::Full);
    let (a, b) = (gpu.draw(&p, &CelCache::none()).expect("draw once"), gpu.draw(&p, &CelCache::none()).expect("draw again"));
    t.row(
        "ten-layer frame 100, Full, drawn twice on the GPU",
        "the same bytes both times",
        if a == b { "the same bytes".into() } else { format!("{} pixels differ", distance(&a, &b).1) },
        a == b,
    );

    // The card's store under pressure, across the reference shot. A budget of one drawing, so
    // nearly every drawing is sent and dropped again; and one of three, so the store is full and
    // a frame bringing two new drawings evicts an old one between them, which in B-44 could
    // bind a layer to the other's drawing. A drawing the store confused with another would show
    // here as a wrong picture.
    let budget = gpu.budget;
    for (drawings, quality) in [(1, PreviewQuality::Draft), (3, PreviewQuality::Full)] {
        gpu.forget();
        gpu.budget = if drawings == 1 { 1 } else { drawings * 1920 * 1080 * 8 };
        let mut worst = (0u8, 0usize);
        for frame in (0..240).step_by(5) {
            let p = plan(reference, frame, quality);
            let g = gpu.draw(&p, &CelCache::none()).expect("draw with the small budget");
            worst = worst.max(distance(&cpu(&p), &g));
        }
        t.row(
            &format!(
                "the reference shot, every fifth frame at {}, the card keeping {drawings} drawing{} at a time",
                quality.label(),
                if drawings == 1 { "" } else { "s" }
            ),
            "at most 1 level",
            format!("worst frame: largest difference {} of 255, pixels differing: {}", worst.0, worst.1),
            worst.0 <= LIMIT,
        );
    }
    gpu.budget = budget;

    // B-44b: a drawing the CPU's cache has let go of stays on the card by the cache's name for
    // it, so the same frame read again from disk into a new cache sends nothing.
    let named = |cache: &mut CelCache| {
        let mut log = FrameLog::new(3);
        let p = compose::plan_frame_at(&declared.project, &declared.comp, 100, &declared.root, PreviewQuality::Full, &mut log, cache)
            .expect("plan the frame");
        preview::scale_plan(p, PreviewQuality::Full)
    };
    gpu.forget();
    let mut cache = CelCache::viewer();
    let p = named(&mut cache);
    gpu.draw(&p, &cache).expect("draw with a cache");
    let first = gpu.sent();
    drop((p, cache));
    let mut cache = CelCache::viewer();
    let p = named(&mut cache);
    let g = gpu.draw(&p, &cache).expect("draw with a new cache");
    let again = gpu.sent() - first;
    let d = distance(&cpu(&p), &g);
    t.row(
        "ten-layer frame 100, Full, read again into a new CPU cache after the first let go",
        "no drawing sent again, and at most 1 level",
        format!("drawings sent again: {again}; largest difference {} of 255", d.0),
        again == 0 && d.0 <= LIMIT,
    );

    // The fallback: a frame with an adjustment layer is drawn wholly by the CPU, and says so.
    let adjusted = persist::load(&repo("Fixtures/adjust/fx_adj_001.json"))
        .unwrap_or_else(|d| panic!("fx_adj_001: {}", d.message));
    let shot = Shot {
        name: "fx_adj_001",
        project: adjusted.document.project().clone(),
        root: repo("Fixtures/adjust"),
        comp: Id::new("comp-main"),
    };
    for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
        let (c, g, _, log) = both(&mut gpu, &shot, 0, quality);
        let logged = log
            .finish()
            .into_iter()
            .find(|d| d.id == DiagnosticId::GpuPreviewOnCpu)
            .map(|d| format!("{}: {}", d.id.as_str(), d.message));
        let same = c == g;
        t.row(
            &format!("fx_adj_001 frame 0, {}: an adjustment layer, so the CPU draws it", quality.label()),
            "the frame log says so, and the picture is the CPU's byte for byte",
            format!(
                "{}; {}",
                logged.as_deref().unwrap_or("nothing logged"),
                if same { "byte-identical" } else { "the picture differs" }
            ),
            logged.is_some() && same,
        );
    }

    // The worst frame, as three pictures.
    let pictures = repo("verification/B-44 pictures");
    fs::create_dir_all(&pictures).expect("make the pictures folder");
    let ((largest, count), worst_case, c, g, w, h) = t.worst.take().expect("something was compared");
    let mut diff = vec![0u8; c.len()];
    for p in (0..w * h).filter(|&p| c[p * 4..p * 4 + 4] != g[p * 4..p * 4 + 4]) {
        // A differing pixel is drawn as a white 7 by 7 square, so one pixel of 2,073,600 can be
        // found by eye. Everything else is black and opaque.
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
    for (name, bytes) in [("cpu (old).png", &c), ("gpu (new).png", &g), ("difference.png", &diff)] {
        png_out::write_rgba(&pictures.join(name), w, h, OutputDepth::Eight, &[], bytes)
            .expect("write a picture");
    }

    let mut s = format!(
        "# B-44: the GPU's picture against the CPU's\n\n\
         Written by `tests/b44_gpu_preview.rs`. The card: {}.\n\n\
         Each row compares the eight-bit picture the page receives, drawn by the CPU and by the \
         GPU. **The rule: no channel of any pixel more than {LIMIT} level of 255 apart** (D-100). \
         Frames with an adjustment layer are drawn by the CPU (D-66) and must be byte-identical.\n\n\
         **{} of {} checks pass.**\n\n\
         The worst comparison is \"{worst_case}\": largest difference {largest} of 255, pixels differing: {count}. Its \
         pictures are in `verification/B-44 pictures/`: `cpu (old).png`, `gpu (new).png`, and \
         `difference.png`, black where the two agree and a white 7 by 7 square around every pixel \
         where they do not.\n\n\
         | Case | Expected | Actual | Result |\n|---|---|---|---|\n",
        gpu.about(),
        t.passed,
        t.checks
    );
    s.push_str(&t.rows);
    fs::write(&out, s).expect("write the B-44 table");
    assert_eq!(t.passed, t.checks, "B-44: {} of {} checks pass", t.passed, t.checks);
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

#[test]
#[ignore = "B-44: a measurement, run deliberately with --release --ignored"]
fn b44_gpu_timing() {
    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "# B-44b: frame times, CPU and GPU\n\n\
         Written by `tests/b44_gpu_preview.rs` (`cargo test --release --test b44_gpu_preview -- \
         --ignored`).\n\n\
         - Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n\
         Each frame's plan is made first and not timed: reading drawings and running effects are \
         the same work on either path. What is timed is the rest of the frame: on the CPU, \
         `render` and `to_srgb8_straight`; on the GPU, sending any drawing the card does not \
         have yet, drawing, and bringing the eight-bit picture back. **First pass** is the GPU \
         meeting every drawing for the first time; **again** is the same frames with the \
         drawings already on the card, if it kept them. Medians over every frame of the shot, in \
         ms, and how many drawings each pass sent to the card, which may hold {:.1} GB of them.\n\n\
         | Shot | Quality | CPU | GPU, first pass | GPU, again | Drawings sent, first pass | Drawings sent, again |\n\
         |---|---|---|---|---|---|---|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
        gpu.budget as f64 / 1e9,
    );
    for shot in shots() {
        for quality in [PreviewQuality::Draft, PreviewQuality::Full] {
            // One cache for the shot, as the window has, so a drawing met again is the same
            // drawing and the card can keep it. Plans are made one at a time and dropped.
            let mut cache = CelCache::viewer();
            let (mut on_cpu, mut first, mut again) = (Vec::new(), Vec::new(), Vec::new());
            let mut sent = [0; 2];
            gpu.forget();
            for pass in 0..2 {
                let before = gpu.sent();
                for frame in 0..240 {
                    let mut log = FrameLog::new(3);
                    let p = compose::plan_frame_at(&shot.project, &shot.comp, frame, &shot.root, quality, &mut log, &mut cache)
                        .expect("plan the frame");
                    let p = preview::scale_plan(p, quality);
                    if pass == 0 {
                        let t = Instant::now();
                        // The tile size the window's CPU path uses at this quality.
                        let tile = match quality {
                            PreviewQuality::Full => DEFAULT_TILE_SIZE,
                            PreviewQuality::Draft => compose::DRAFT_TILE_SIZE,
                        };
                        drop(render::render(&p, tile).to_srgb8_straight());
                        on_cpu.push(t.elapsed().as_secs_f64() * 1000.0);
                    }
                    let t = Instant::now();
                    drop(gpu.draw(&p, &cache).expect("draw"));
                    [&mut first, &mut again][pass].push(t.elapsed().as_secs_f64() * 1000.0);
                }
                sent[pass] = gpu.sent() - before;
            }
            let _ = writeln!(
                s,
                "| {} | {} | {:.2} | {:.2} | {:.2} | {} | {} |",
                shot.name,
                quality.label(),
                median(on_cpu),
                median(first),
                median(again),
                sent[0],
                sent[1]
            );
        }
    }
    fs::write(repo("verification/B-44b_gpu_timing_table.md"), s).expect("write the timing table");
}
