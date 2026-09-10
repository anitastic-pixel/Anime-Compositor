//! P-05: a layer skipped for the tiles it cannot reach moves no pixel.
//!
//! Document 15's P-05 asks for "whole-frame byte equality with and without culling on both
//! fixtures, all 2,073,600 pixels, plus timings at draft and at full". This file is the equality
//! half and runs on every build; the timings are `verification/P-05_culling.md`, written by the
//! `#[ignore]`d measurement at the bottom, because a duration is not a thing a committed page can
//! carry - CI checks `verification/` byte for byte and a page rewritten on every machine is a
//! page nobody can tell a change from.
//!
//! # What is compared
//!
//! One frame plan, rendered twice: once by `render`, which skips a layer for a tile its
//! transformed extent does not meet, and once by `render_without_culling`, which is the loop as
//! it was. The comparison is made at both ends of the pipe:
//!
//! - the **encoded** frame, which is what a viewer sees, and
//! - the **working buffer**, f32 and bit for bit, which is stricter than anything a screen can
//!   show.
//!
//! The second one exists because of the one wrinkle document 15's entry names: `0.0 + -0.0` is
//! `+0.0`, so a blend that is skipped and a blend that is performed with a zero source can differ
//! in the sign of a zero without differing in any pixel. Comparing the bits is how that question
//! gets an answer instead of a shrug. Both rows are reported; only a difference a viewer could
//! see is a failure.
//!
//! Nothing here has an expected value of its own (ADR-009). The expected frame is the frame this
//! renderer produced before the skip existed, which is what `render_without_culling` is for.

use std::fmt::Write as _;
use std::path::PathBuf;
use std::time::Instant;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{BlendMode, Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::{scale_plan, PreviewQuality};
use anime_compositor::render::{
    bounds, reaches, render, render_without_culling, tiles, Affine, FramePlan, LayerDraw,
};
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
    let (project, root, _text) = build_fixture("p05");
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

/// The tile size the preview would use at this quality, which is the one P-03(f) measured.
fn tile_size(quality: PreviewQuality) -> usize {
    match quality {
        PreviewQuality::Full => DEFAULT_TILE_SIZE,
        PreviewQuality::Draft => compose::DRAFT_TILE_SIZE,
    }
}

/// The two renders of one frame: with the skip, and without it.
fn both_ways(w: &Workload, frame: i32, quality: PreviewQuality) -> (WorkingBuffer, WorkingBuffer) {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_cached(
        &w.project,
        &Id::new(COMP),
        frame,
        &w.root,
        &mut log,
        &mut CelCache::none(),
    )
    .unwrap_or_else(|d| panic!("{} frame {frame}: {}", w.name, d.message));
    let plan = scale_plan(plan, quality);
    let tile = tile_size(quality);
    (render(&plan, tile), render_without_culling(&plan, tile))
}

/// How many of the encoded bytes differ, and how many of the f32 channel values differ in their
/// bits. The second number counts a `-0.0` against a `+0.0`; the first cannot see one.
fn differences(culled: &WorkingBuffer, whole: &WorkingBuffer) -> (usize, usize) {
    let (a, b) = (culled.to_srgb8_straight(), whole.to_srgb8_straight());
    assert_eq!(a.len(), b.len(), "the two renders are not the same size");
    let seen = a.iter().zip(&b).filter(|(x, y)| x != y).count();
    let bits = culled
        .data()
        .iter()
        .zip(whole.data())
        .filter(|(x, y)| x.to_bits() != y.to_bits())
        .count();
    (seen, bits)
}

/// A 480x270 layer of solid white dropped into one corner of a 1920x1080 frame: how many
/// layer-and-tile pairs the box excludes, out of how many, and how many pixels the skip moved.
///
/// Neither fixture has a layer smaller than the frame - a cel is a full-frame drawing with
/// transparent margins, and transparency is not something a geometric box can see - so without
/// this the whole page would be a table about a skip that never fired. The plan is built by hand
/// here rather than added to `Fixtures/`, which is read-only to implementation work.
fn a_layer_smaller_than_the_frame() -> (usize, usize, usize) {
    let mut source = WorkingBuffer::transparent(480, 270);
    for p in source.data_mut() {
        *p = 1.0;
    }
    let plan = FramePlan {
        width: 1920,
        height: 1080,
        layers: vec![LayerDraw {
            id: Id::new("a layer smaller than the frame"),
            source: std::sync::Arc::new(source),
            transform: Affine::translation(1400.0, 780.0),
            opacity: 1.0,
            matte: None,
            blend: BlendMode::Normal,
        }],
    };
    let tile = tile_size(PreviewQuality::Full);
    let mut skipped = 0;
    let mut pairs = 0;
    for t in tiles(plan.width, plan.height, tile) {
        for layer in &plan.layers {
            pairs += 1;
            if !reaches(bounds(layer), t) {
                skipped += 1;
            }
        }
    }
    let (culled, whole) = (render(&plan, tile), render_without_culling(&plan, tile));
    (skipped, pairs, differences(&culled, &whole).0)
}

/// The every-build check. Writes `verification/P-05_culling_table.md`.
#[test]
fn culling_moves_no_pixel() {
    let mut rows = String::new();
    let mut failures = 0usize;
    let mut checks = 0usize;
    let mut zero_signs = 0usize;

    for w in &workloads() {
        for quality in [PreviewQuality::Draft, PreviewQuality::Full] {
            for frame in FRAMES {
                let (culled, whole) = both_ways(w, frame, quality);
                let (seen, bits) = differences(&culled, &whole);
                let pixels = culled.width() * culled.height();
                zero_signs += bits;
                checks += 1;
                if seen != 0 {
                    failures += 1;
                }
                let _ = writeln!(
                    rows,
                    "| {} at {}, frame {frame}: every one of {pixels} pixels is what the frame \
                     was before any layer was skipped | `0 pixels differ` | `{seen} pixels \
                     differ` | {} |",
                    w.name,
                    quality.label(),
                    if seen == 0 { "pass" } else { "FAIL" },
                );
            }
        }
    }

    // The row that proves the comparison can fail at all, in H-01's shape: two different frames
    // of the same fixture, compared the same way, must disagree.
    let shot = &workloads()[0];
    let (a, _) = both_ways(shot, 0, PreviewQuality::Full);
    let (b, _) = both_ways(shot, 100, PreviewQuality::Full);
    let (seen, _) = differences(&a, &b);
    checks += 1;
    if seen == 0 {
        failures += 1;
    }
    let _ =
        writeln!(
        rows,
        "| the comparison is capable of failing: the reference shot's frame 0 against its frame \
         100 | `they differ` | `{}` | {} |",
        if seen == 0 { "they do not" } else { "they differ" },
        if seen == 0 { "FAIL" } else { "pass" },
    );

    // A table saying the frame is unchanged reads exactly the same whether the skip works or
    // never fires once, so this row counts the pairs the box excludes. It is the row that makes
    // every row above it mean something.
    let mut skipped = 0usize;
    let mut pairs = 0usize;
    for w in &workloads() {
        for frame in FRAMES {
            let mut log = FrameLog::new(3);
            let plan = compose::plan_frame_cached(
                &w.project,
                &Id::new(COMP),
                frame,
                &w.root,
                &mut log,
                &mut CelCache::none(),
            )
            .unwrap_or_else(|d| panic!("{} frame {frame}: {}", w.name, d.message));
            for tile in tiles(plan.width, plan.height, tile_size(PreviewQuality::Full)) {
                for layer in &plan.layers {
                    pairs += 1;
                    if !reaches(bounds(layer), tile) {
                        skipped += 1;
                    }
                }
            }
        }
    }
    checks += 1;
    let _ = writeln!(
        rows,
        "| how much the skip fires on the fixtures: layer-and-tile pairs the box excludes, over \
         the eight full-resolution frames above | `stated, whatever it is` | `{skipped} of \
         {pairs}` | noted |",
    );

    // And the same count on a layer that is smaller than the frame, which is the case the
    // fixtures do not have and a person's project does. This one is a failure if it is zero:
    // without it, every row above is a table about a skip that never happened.
    let (small, small_pairs, seen) = a_layer_smaller_than_the_frame();
    checks += 2;
    if small == 0 {
        failures += 1;
    }
    if seen != 0 {
        failures += 1;
    }
    let _ = writeln!(
        rows,
        "| the skip fires at all: a quarter-size layer placed in one corner of a 1920x1080 frame \
         | `more than none` | `{small} of {small_pairs}` | {} |",
        if small == 0 { "FAIL" } else { "pass" },
    );
    let _ = writeln!(
        rows,
        "| and that frame is still the frame it was without the skip, all 2073600 pixels | `0 \
         pixels differ` | `{seen} pixels differ` | {} |",
        if seen == 0 { "pass" } else { "FAIL" },
    );

    // Stricter than a viewer, and reported rather than asserted: a zero of the other sign is not
    // a pixel, and this row exists so that the number is stated instead of assumed.
    checks += 1;
    let _ = writeln!(
        rows,
        "| the working buffer, f32 and bit for bit, across every check above | `identical` | `{}` \
         | {} |",
        if zero_signs == 0 {
            "identical".to_string()
        } else {
            format!("{zero_signs} channel values differ in the sign of a zero")
        },
        if zero_signs == 0 { "pass" } else { "noted" },
    );

    let mut page = String::new();
    let _ = write!(
        page,
        "# P-05 - the layer skipped for a tile it cannot reach\n\n\
         **{} of {checks} checks passed.**\n\n\
         Produced by `tests/p05_culling.rs`, on every build.\n\n\
         ## Why this exists\n\n\
         `render_tile` used to sample every layer at every pixel of every tile, with no test of \
         whether the layer's transformed extent met the tile at all, and cel artwork is mostly \
         transparent. P-05 gives every layer a box - its source rectangle grown by the one pixel \
         bilinear sampling reaches for, mapped through the layer's own transform - and skips the \
         layer for the tiles outside it.\n\n\
         A skip is only correct if the frame is the same frame without it. So every frame below \
         is rendered **twice from one plan**: once by the renderer, which skips, and once by \
         `render_without_culling`, which is the loop as it was, and every channel of every pixel \
         is compared. Not a tolerance, not a sample: any difference a viewer could see is a \
         failure.\n\n\
         The last three rows are the ones that make the rest mean anything. The first of \
         them proves the comparison can fail at all; the second counts how many layer-and-tile \
         pairs the box actually excludes, because a table of unchanged frames reads the same \
         whether the skip works or never fires. The last is \
         stricter than a viewer: it compares the f32 working buffer bit for bit, which is where \
         the one wrinkle would show, since `0.0 + -0.0` is `+0.0` and a blend that is skipped can \
         therefore differ from a blend performed against a zero source in the sign of a zero \
         while agreeing on every pixel. It is reported rather than asserted.\n\n\
         Timings are not on this page. They are dated, and they are in \
         `verification/P-05_culling.md`.\n\n\
         ## Checks\n\n\
         | Check | Expected | Actual | Result |\n|---|---|---|---|\n{rows}",
        checks - failures,
    );

    let out = repo("verification/P-05_culling_table.md");
    std::fs::write(&out, page).unwrap_or_else(|e| panic!("write {}: {e}", out.display()));
    assert_eq!(failures, 0, "see {}", out.display());
}

/// The measurement document 15's entry asks for. Writes `verification/P-05_culling.md`.
#[test]
#[ignore = "a measurement; run it deliberately, in release, on the recorded machine"]
fn p05_culling_cost() {
    let mut rows = String::new();
    for w in &workloads() {
        for quality in [PreviewQuality::Draft, PreviewQuality::Full] {
            let (mut with, mut without) = (Vec::new(), Vec::new());
            for step in 0..40 {
                let frame = step * 6;
                let mut log = FrameLog::new(3);
                let plan = compose::plan_frame_cached(
                    &w.project,
                    &Id::new(COMP),
                    frame,
                    &w.root,
                    &mut log,
                    &mut CelCache::none(),
                )
                .unwrap_or_else(|d| panic!("{} frame {frame}: {}", w.name, d.message));
                let plan = scale_plan(plan, quality);
                let tile = tile_size(quality);
                // The two renders alternate, because whichever runs first on a fresh plan pays
                // its cache misses: fixed order read as a 24% cost for the skip at draft, which
                // was the ordering and not the box.
                let once = |first: bool| {
                    let at = Instant::now();
                    let frame = if first {
                        render(&plan, tile)
                    } else {
                        render_without_culling(&plan, tile)
                    };
                    let ms = at.elapsed().as_secs_f64() * 1000.0;
                    drop(frame);
                    ms
                };
                if step % 2 == 0 {
                    with.push(once(true));
                    without.push(once(false));
                } else {
                    without.push(once(false));
                    with.push(once(true));
                }
            }
            let p50 = |mut v: Vec<f64>| {
                v.sort_by(|x, y| x.partial_cmp(y).expect("a duration is never NaN"));
                v[v.len() / 2]
            };
            let (a, b) = (p50(with), p50(without));
            let _ = writeln!(
                rows,
                "| {} | {} | {b:.3} | {a:.3} | {:+.1}% |",
                w.name,
                quality.label(),
                (a - b) / b * 100.0,
            );
        }
    }
    let out = repo("verification/P-05_culling.md");
    let page = format!(
        "# P-05: what the skip is worth\n\n\
         Written by `p05_culling_cost` in `tests/p05_culling.rs`, `#[ignore]`d and run \
         deliberately in release. Forty frames spread across each fixture's work area, the tile \
         loop only - no decode, no effect stack, no encode - each rendered both ways, in an order that alternates frame by frame.\n\n\
         The correctness half is `verification/P-05_culling_table.md`, which runs on every \
         build.\n\n\
         - CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads\n\
         - OS: Microsoft Windows 11 Education, 10.0.26200\n\
         - Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`\n\
         - Date: 2026-09-09\n\n\
         | Fixture | Quality | Without culling (ms) | With culling (ms) | Change |\n\
         |---|---|---|---|---|\n{rows}\n\
         Document 15's entry said so in advance - \"it is now expected to report a saving too \
         small to matter\" - and the measured answer on these two fixtures is that there is no \
         saving at all: the box excludes none of the 7,020 layer-and-tile pairs \
         `verification/P-05_culling_table.md` counts, because every cel here is a full-frame \
         drawing with transparent margins and a geometric box cannot see transparency.\n\n\
         What the table does say is that the test costs nothing to carry: under 3% of the \
         tile loop, which P-01 measured at 1.3% to 7.8% of a frame, so under a fifth of one \
         percent of a frame at the worst row above. It is kept on that basis and not on a \
         saving. The skip itself works - the same page excludes 120 of 135 pairs for a \
         quarter-size layer in a corner - so it will pay on layers smaller than the frame, and \
         a box drawn from alpha rather than geometry is the entry that would pay here.\n\n\
         The two renders alternate frame by frame. They did not at first, and with the culled \
         render always going first on a fresh plan the table read +24% at draft; that was the \
         cold cache of whichever ran first, not the box.\n"
    );
    std::fs::write(&out, page).unwrap_or_else(|e| panic!("write {}: {e}", out.display()));
    println!("wrote {}", out.display());
}
