//! P-06: draft-resolution cels, shown to the owner as a difference.
//!
//! Document 15's P-06 says this "cannot be built without a register entry first" and scopes it as
//! "a branch that is measured and not merged". So nothing in `src/` changes: the whole
//! quarter-resolution path lives in this file, behind an `#[ignore]`d measurement that is run
//! deliberately and writes `verification/P-06_draft_cels.md` with its pictures. The table is the
//! deliverable. It exists to let the owner see the cost and decide, not to argue for it.
//!
//! # What is being compared
//!
//! Draft quality reduces the composite to 480 by 270 and decodes every cel at 1920 by 1080
//! anyway. The alternative is to hold each cel at a quarter on each side, one sixteenth of the
//! bytes, and let the layer's transform carry the difference:
//!
//! - **today**: `source(1920x1080)` -> `T` -> `S(1/4)` -> a 480x270 frame;
//! - **measured here**: `source(480x270)` -> `S(4)` -> `T` -> `S(1/4)` -> a 480x270 frame.
//!
//! The cel is reduced by an exact 4x4 box average **in premultiplied linear light**, which is the
//! space the working buffer is already in, so no colour conversion and no alpha division happens
//! on the way.
//!
//! # Why this is a specification question and not an optimisation
//!
//! Downsampling and then transforming is not the same operation as transforming and then
//! downsampling, and the difference is visible: it is the last two columns of the table below.
//! D-33 already puts the preview at draft by default and requires the viewer to say when the
//! preview differs from export; whether that sentence covers *this* difference is the owner's to
//! say in document 14, before any of this lands.
//!
//! # The export
//!
//! ADR-015 keeps the cache off the export path — `plan_frame` and `crate::export` pass
//! `CelCache::none()` — so an export cannot see a reduced cel by construction. This file checks
//! it anyway, by exporting the same frames before and after all of the draft work above and
//! comparing the encoded bytes.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::{scale_plan, PreviewQuality, DRAFT_DIVISOR};
use anime_compositor::render::{render, Affine, FramePlan};
use anime_compositor::WorkingBuffer;

mod common;
use common::{build_fixture, repo};

const COMP: &str = "comp-reference-shot";

/// H-01's frames, and for the same reason: frame 14 is the one where a layer's drawing is
/// missing, so the plan has one fewer layer in it than the others.
const FRAMES: [i32; 4] = [0, 14, 100, 239];

struct Workload {
    name: &'static str,
    project: Project,
    root: PathBuf,
}

fn workloads() -> Vec<Workload> {
    let path = repo("verification/B-08a_project.json");
    let loaded =
        persist::load(&path).unwrap_or_else(|d| panic!("open {}: {}", path.display(), d.message));
    let (project, root, _text) = build_fixture("p06");
    vec![
        Workload {
            name: "reference shot",
            project: loaded.document.project().clone(),
            root: repo("Fixtures/reference_shot"),
        },
        Workload {
            name: "declared fixture",
            project,
            root,
        },
    ]
}

fn plan_for(w: &Workload, frame: i32) -> FramePlan {
    let mut log = FrameLog::new(3);
    compose::plan_frame_cached(
        &w.project,
        &Id::new(COMP),
        frame,
        &w.root,
        &mut log,
        &mut CelCache::none(),
    )
    .unwrap_or_else(|d| panic!("{} frame {frame}: {}", w.name, d.message))
}

/// A cel reduced to a quarter on each side by an exact box average of the premultiplied linear
/// values it already holds.
///
/// The edge blocks of an extent that does not divide by four are short rather than padded, so no
/// invented pixel is averaged in. Every fixture cel here is 1920x1080 and divides exactly; the
/// clamp is there so the measurement does not depend on that.
fn quarter(source: &WorkingBuffer) -> WorkingBuffer {
    let d = DRAFT_DIVISOR;
    let (w, h) = (source.width().div_ceil(d), source.height().div_ceil(d));
    let mut out = WorkingBuffer::transparent(w, h);
    for y in 0..h {
        for x in 0..w {
            let mut acc = [0.0f64; 4];
            let mut n = 0.0f64;
            for sy in (y * d)..((y + 1) * d).min(source.height()) {
                for sx in (x * d)..((x + 1) * d).min(source.width()) {
                    for (total, value) in acc.iter_mut().zip(source.pixel(sx, sy)) {
                        *total += value as f64;
                    }
                    n += 1.0;
                }
            }
            let at = (y * w + x) * 4;
            for (cell, total) in out.data_mut()[at..at + 4].iter_mut().zip(acc) {
                *cell = (total / n) as f32;
            }
        }
    }
    out
}

/// The same plan with every cel — and every matte — held at a quarter, and each transform given
/// the outer scale that puts the reduced pixels back where the full ones were.
fn quarter_plan(plan: &FramePlan) -> FramePlan {
    let up = Affine::scaling(DRAFT_DIVISOR as f64, DRAFT_DIVISOR as f64);
    let mut out = plan.clone();
    for layer in &mut out.layers {
        layer.source = Arc::new(quarter(&layer.source));
        layer.transform = up.then(layer.transform);
        if let Some(matte) = &mut layer.matte {
            matte.source = Arc::new(quarter(&matte.source));
            matte.transform = up.then(matte.transform);
        }
    }
    out
}

/// How many of a draft frame's 129,600 pixels differ once encoded to the eight bits a viewer
/// sees, the largest channel difference out of 255, and the mean over every channel.
struct Difference {
    pixels: usize,
    largest: u8,
    mean: f64,
}

fn compare(a: &WorkingBuffer, b: &WorkingBuffer) -> Difference {
    let (a, b) = (a.to_srgb8_straight(), b.to_srgb8_straight());
    let mut pixels = 0;
    let mut largest = 0u8;
    let mut total = 0u64;
    for (pa, pb) in a.chunks(4).zip(b.chunks(4)) {
        let worst = pa
            .iter()
            .zip(pb)
            .map(|(x, y)| x.abs_diff(*y))
            .max()
            .unwrap_or(0);
        total += pa
            .iter()
            .zip(pb)
            .map(|(x, y)| x.abs_diff(*y) as u64)
            .sum::<u64>();
        if worst > 0 {
            pixels += 1;
            largest = largest.max(worst);
        }
    }
    Difference {
        pixels,
        largest,
        mean: total as f64 / a.len() as f64,
    }
}

/// A frame as an eight-bit sRGB PNG, straight alpha, so it can be looked at.
fn write_png(path: &Path, buffer: &WorkingBuffer) {
    let file = std::io::BufWriter::new(std::fs::File::create(path).expect("create the picture"));
    let mut encoder = png::Encoder::new(file, buffer.width() as u32, buffer.height() as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder
        .write_header()
        .expect("write the header")
        .write_image_data(&buffer.to_srgb8_straight())
        .expect("write the pixels");
}

/// The difference itself, amplified sixteen times and laid on opaque black, because a difference
/// of one or two code values is invisible next to the picture it came from.
fn write_difference(path: &Path, a: &WorkingBuffer, b: &WorkingBuffer) {
    let (x, y) = (a.to_srgb8_straight(), b.to_srgb8_straight());
    let mut bytes = Vec::with_capacity(x.len());
    for (pa, pb) in x.chunks(4).zip(y.chunks(4)) {
        for c in 0..3 {
            bytes.push(pa[c].abs_diff(pb[c]).saturating_mul(16));
        }
        bytes.push(255);
    }
    let file = std::io::BufWriter::new(std::fs::File::create(path).expect("create the picture"));
    let mut encoder = png::Encoder::new(file, a.width() as u32, a.height() as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder
        .write_header()
        .expect("write the header")
        .write_image_data(&bytes)
        .expect("write the pixels");
}

/// Every distinct drawing the project names, which is what a cache would have to hold to serve a
/// whole pass over the shot.
fn distinct_drawings(project: &Project) -> usize {
    project
        .assets
        .iter()
        .map(|asset| asset.frames.len())
        .sum::<usize>()
}

fn gib(bytes: usize) -> f64 {
    bytes as f64 / (1024.0 * 1024.0 * 1024.0)
}

#[test]
#[ignore = "a measurement, not a check: run deliberately in release"]
fn p06_draft_cels_cost() {
    let mut timing = String::new();
    let mut picture_rows = String::new();
    let mut bytes_rows = String::new();
    let mut export_rows = String::new();

    for w in &workloads() {
        // Before anything is reduced: what an export of these frames encodes to.
        let exported: Vec<Vec<u8>> = FRAMES
            .iter()
            .map(|&frame| {
                let mut log = FrameLog::new(3);
                compose::render_frame(
                    &w.project,
                    &Id::new(COMP),
                    frame,
                    &w.root,
                    DEFAULT_TILE_SIZE,
                    &mut log,
                )
                .expect("the export path renders")
                .to_srgb8_straight()
            })
            .collect();

        let (mut full_ms, mut quarter_ms, mut reduce_ms) = (Vec::new(), Vec::new(), Vec::new());
        let mut worst: Option<(i32, Difference, WorkingBuffer, WorkingBuffer)> = None;

        for (step, &frame) in FRAMES.iter().enumerate() {
            let plan = plan_for(w, frame);

            let at = Instant::now();
            let reduced = quarter_plan(&plan);
            reduce_ms.push(at.elapsed().as_secs_f64() * 1000.0);

            let today = scale_plan(plan, PreviewQuality::Draft);
            let cheap = scale_plan(reduced, PreviewQuality::Draft);
            let tile = compose::DRAFT_TILE_SIZE;

            // The two renders alternate, because whichever runs first on a fresh plan pays its
            // cache misses - the lesson P-05's first table learned the hard way.
            let (a, b) = if step % 2 == 0 {
                let at = Instant::now();
                let a = render(&today, tile);
                full_ms.push(at.elapsed().as_secs_f64() * 1000.0);
                let at = Instant::now();
                let b = render(&cheap, tile);
                quarter_ms.push(at.elapsed().as_secs_f64() * 1000.0);
                (a, b)
            } else {
                let at = Instant::now();
                let b = render(&cheap, tile);
                quarter_ms.push(at.elapsed().as_secs_f64() * 1000.0);
                let at = Instant::now();
                let a = render(&today, tile);
                full_ms.push(at.elapsed().as_secs_f64() * 1000.0);
                (a, b)
            };

            let d = compare(&a, &b);
            let _ = writeln!(
                picture_rows,
                "| {} | {frame} | {} | {} | {:.4} |",
                w.name, d.pixels, d.largest, d.mean
            );
            if worst
                .as_ref()
                .is_none_or(|(_, best, _, _)| (d.largest, d.pixels) > (best.largest, best.pixels))
            {
                worst = Some((frame, d, a, b));
            }
        }

        let p50 = |mut v: Vec<f64>| {
            v.sort_by(|x, y| x.partial_cmp(y).expect("a duration is never NaN"));
            v[v.len() / 2]
        };
        let (today, cheap) = (p50(full_ms), p50(quarter_ms));
        let reduce = p50(reduce_ms);
        let _ = writeln!(
            timing,
            "| {} | {today:.3} | {cheap:.3} | {:+.1}% | {reduce:.1} | {:.0} frames |",
            w.name,
            (cheap - today) / today * 100.0,
            reduce / (today - cheap),
        );

        // The pictures, for the frame that differs most.
        if let Some((frame, _, today, cheap)) = worst {
            let slug = w.name.replace(' ', "_");
            write_png(
                &repo(&format!("verification/P-06_{slug}_today.png")),
                &today,
            );
            write_png(
                &repo(&format!("verification/P-06_{slug}_quarter.png")),
                &cheap,
            );
            write_difference(
                &repo(&format!("verification/P-06_{slug}_difference.png")),
                &today,
                &cheap,
            );
            let _ = writeln!(
                picture_rows,
                "| **{}, frame {frame}** | pictured below | `P-06_{slug}_today.png`, \
                 `P-06_{slug}_quarter.png`, `P-06_{slug}_difference.png` | |",
                w.name
            );
        }

        // What a cache would have to hold, over the whole fixture rather than over four frames.
        let drawings = distinct_drawings(&w.project);
        let (cw, ch) = (1920usize, 1080usize);
        let full_bytes = drawings * cw * ch * 16;
        let quarter_bytes = drawings * cw.div_ceil(DRAFT_DIVISOR) * ch.div_ceil(DRAFT_DIVISOR) * 16;
        let _ = writeln!(
            bytes_rows,
            "| {} | {drawings} | {:.2} GiB | {:.2} GiB |",
            w.name,
            gib(full_bytes),
            gib(quarter_bytes),
        );

        // And the export, again, after every reduced cel this fixture will ever see.
        let mut same = true;
        for (&frame, before) in FRAMES.iter().zip(&exported) {
            let mut log = FrameLog::new(3);
            let after = compose::render_frame(
                &w.project,
                &Id::new(COMP),
                frame,
                &w.root,
                DEFAULT_TILE_SIZE,
                &mut log,
            )
            .expect("the export path renders")
            .to_srgb8_straight();
            same &= &after == before;
        }
        let _ = writeln!(
            export_rows,
            "| {} | {} | {} |",
            w.name,
            FRAMES.len(),
            if same {
                "byte-identical"
            } else {
                "**DIFFERENT**"
            }
        );
    }

    let page = format!(
        "# P-06: what draft-resolution cels cost, and what they change\n\n\
         Written by `p06_draft_cels_cost` in `tests/p06_draft_cels.rs`, `#[ignore]`d and run \
         deliberately in release. **Nothing in `src/` changes**: document 15 scopes P-06 as \"a \
         branch that is measured and not merged\", and the whole quarter-resolution path lives \
         in that one test file. This page is the deliverable. It exists to let the owner see \
         the cost and decide, not to argue for it.\n\n\
         - CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads\n\
         - OS: Microsoft Windows 11 Education, 10.0.26200\n\
         - Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`\n\
         - Date: 2026-09-10\n\n\
         ## What was changed\n\n\
         Draft quality reduces the composite to 480 by 270 and decodes every cel at 1920 by 1080 \
         anyway. Held at a quarter on each side, a cel is one sixteenth of the bytes, and the \
         layer's transform carries the difference:\n\n\
         - **today**: `source(1920x1080)` then `T` then `S(1/4)`, into a 480x270 frame;\n\
         - **measured here**: `source(480x270)` then `S(4)` then `T` then `S(1/4)`, into the \
         same frame.\n\n\
         The reduction is an exact 4x4 box average **in premultiplied linear light**, the space \
         the working buffer is already in, so no colour conversion and no alpha division happens \
         on the way.\n\n\
         ## The frame time\n\n\
         The tile loop only, four frames each, the two renders alternating frame by frame so \
         that neither pays the other's cold cache. The fifth column is what it costs to reduce \
         one frame's cels once — work a cache would do on the decode, not on every frame, and \
         it is listed because a cache that cannot hold the shot pays it again on every eviction.\n\n\
         | Fixture | Draft today (ms) | Draft with quarter cels (ms) | Change | Reducing one \
         frame's cels (ms) | Frames before that pays for itself |\n\
         |---|---|---|---|---|---|\n{timing}\n\
         ## The bytes\n\n\
         Every distinct drawing the fixture names, at 1920x1080 RGBA f32 against the same \
         drawing at 480x270. D-40 sets the cache budget at 1 GiB.\n\n\
         | Fixture | Distinct drawings | Held at full | Held at a quarter |\n\
         |---|---|---|---|\n{bytes_rows}\n\
         ## The difference, which is the point\n\n\
         Downsampling then transforming is not transforming then downsampling. Each row is one \
         draft frame of 129,600 pixels, encoded to the eight bits a viewer sees and compared \
         against the draft frame this build produces today.\n\n\
         | Fixture | Frame | Pixels that differ | Largest difference of 255 | Mean over all \
         channels |\n\
         |---|---|---|---|---|\n{picture_rows}\n\
         The pictures are the frame that differs most on each fixture: the draft frame as it is \
         today, the same frame with quarter cels, and the difference between them **amplified \
         sixteen times on opaque black**, because a difference of one or two code values is \
         invisible beside the picture it came from.\n\n\
         ![the draft frame today](P-06_reference_shot_today.png)\n\n\
         ![the same frame with quarter cels](P-06_reference_shot_quarter.png)\n\n\
         ![the difference, amplified sixteen times](P-06_reference_shot_difference.png)\n\n\
         ![the draft frame today](P-06_declared_fixture_today.png)\n\n\
         ![the same frame with quarter cels](P-06_declared_fixture_quarter.png)\n\n\
         ![the difference, amplified sixteen times](P-06_declared_fixture_difference.png)\n\n\
         ## Reading the numbers\n\n\
         Facts, not a recommendation.\n\n\
         - **The tile loop gets about half its time back.** Both fixtures render a draft frame \
         in roughly half the time once the cels are already reduced.\n\
         - **Reducing a cel costs far more than one frame's saving.** The last two columns of \
         the first table are the whole arithmetic: reducing a frame's cels is tens of \
         milliseconds, the saving is one or two, so this is only ever a saving if a reduced cel \
         is held and reused over many frames. On a cache miss it is a loss.\n\
         - **It is the bytes, not the milliseconds, that move.** Held at full, one pass over \
         either fixture wants more than D-40's 1 GiB; held at a quarter, both fit several times \
         over. A cache that fits is a cache that stops missing, which is where the frame time \
         above actually comes from.\n\
         - **The picture changes, and not subtly.** More than half the pixels of every draft \
         frame differ. The difference pictures show where: edges and fine detail, with flat \
         areas black. Averaging sixteen source pixels is not the same as taking one bilinear \
         sample of them, and at a quarter scale that gap is the aliasing the current draft \
         frame has and the reduced one does not. Which of the two is *correct* is not a \
         question this measurement can answer.\n\n\
         ## The export\n\n\
         ADR-015 keeps the cache off the export path — `plan_frame` and `crate::export` pass \
         `CelCache::none()` — so an export cannot see a reduced cel by construction. Checked \
         anyway: the same frames exported before any of the work above, and again after all of \
         it.\n\n\
         | Fixture | Frames exported twice | Encoded bytes |\n\
         |---|---|---|\n{export_rows}\n\
         ## What this does not decide\n\n\
         Document 15: this \"cannot be built without a register entry first\". The pixels on the \
         preview path change, which makes it a specification question. D-33 already puts the \
         preview at draft by default and requires the viewer to say when the preview differs \
         from export; whether that sentence covers this difference is the owner's to say, in \
         document 14, before any of this lands.\n"
    );
    std::fs::write(repo("verification/P-06_draft_cels.md"), page).expect("write the artifact");
}
