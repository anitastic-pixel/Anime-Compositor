//! P-15: what one mask costs to rasterise, on a layer the size of the owner's.
//!
//! Writes `verification/P-15_mask_cost.md`. `#[ignore]`d in normal runs, like every other
//! measurement in this project, and run with `--release --ignored`.
//!
//! # Why this exists
//!
//! On 2026-09-22 the owner reported that dragging a mask point still lagged heavily after B-24g
//! bounded the request queue. The window's own stopwatch (P-15's `x-ms` header) answered
//! **17750.1 ms for one frame** at Draft resolution, against the 2.502 ms
//! `verification/P-01_frame_trace.md` measures for a warm draft frame of the reference shot. The
//! difference is the mask: P-01's fixtures carry none, so its `layer mask` row reads 0.000 ms in
//! all twelve tables and the cost had never been measured anywhere.
//!
//! This file measures it, against the layer sizes and outline sizes a person actually draws.
//!
//! # What is measured, and what is not
//!
//! `mask::coverage` alone: the field, and nothing that uses it. Not a frame, not a drag, not the
//! window. The frame around it is `verification/P-01_frame_trace.md`'s job and the window's is
//! the owner's hand. A ratio here is a ratio of this function only.
//!
//! The "sample by sample" column is the rule the build followed until 2026-09-22, and it is
//! written out here rather than kept in the build for measuring: [`sample_by_sample_ms`] asks
//! each of a pixel's sixteen samples which side of the whole outline it is on, and how far from
//! it, through the same public `mask::point_inside` and `mask::distance_to_path` the old path
//! used. Keeping a second rasteriser in `src/mask.rs` so that a benchmark could call it would be
//! a worse trade than writing the loop here, where nothing draws with it.

use anime_compositor::mask::{self, Mask, MaskPoint, SAMPLES_PER_SIDE};
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

/// A rough circle of `corners` bezier points, the shape a person draws round a character.
///
/// Handles are a third of the way to the neighbour, which is the usual circle approximation, so
/// the outline flattens into the hundreds of edges a curved mask really has rather than the
/// handful a polygon of corners would.
fn drawn_mask(corners: usize, radius: f64, at: (f64, f64)) -> Mask {
    let points = (0..corners)
        .map(|i| {
            let turn = std::f64::consts::TAU * i as f64 / corners as f64;
            let (cos, sin) = (turn.cos(), turn.sin());
            let reach = radius * std::f64::consts::TAU / corners as f64 / 3.0;
            MaskPoint {
                point: (at.0 + radius * cos, at.1 + radius * sin),
                in_handle: (reach * sin, -reach * cos),
                out_handle: (-reach * sin, reach * cos),
            }
        })
        .collect();
    Mask {
        points,
        ..Mask::default()
    }
}

/// The median of `runs` rasterisations of `mask`, in milliseconds.
fn median_ms(mask: &Mask, width: usize, height: usize, runs: usize) -> f64 {
    let mut times: Vec<f64> = Vec::with_capacity(runs);
    for _ in 0..runs {
        let began = Instant::now();
        let field = mask::coverage(std::slice::from_ref(mask), width, height);
        times.push(began.elapsed().as_secs_f64() * 1000.0);
        // So that no optimiser is tempted to decide the field was never wanted.
        assert!(field.is_some(), "the mask must be drawable");
    }
    times.sort_by(|a, b| a.partial_cmp(b).expect("a duration is never NaN"));
    times[times.len() / 2]
}

/// One rasterisation by the rule the build followed until P-15, in milliseconds.
///
/// Sixteen samples a pixel, each one walking the whole outline for its side and, when the mask is
/// expanded, the whole outline again for its distance. No feather: this is the field only, which
/// is the part P-15 changed, and it is compared against the same field.
///
/// One run, because at 1920x1080 it is measured in seconds and a median of five would have this
/// file take a quarter of an hour to say the same thing.
fn sample_by_sample_ms(mask: &Mask, width: usize, height: usize) -> f64 {
    let outline = mask.outline();
    let expansion = mask.expansion_px;
    let n = SAMPLES_PER_SIDE;
    let began = Instant::now();
    let mut total = 0u64;
    for y in 0..height {
        for x in 0..width {
            for j in 0..n {
                for i in 0..n {
                    let sx = x as f64 + (i as f64 + 0.5) / n as f64;
                    let sy = y as f64 + (j as f64 + 0.5) / n as f64;
                    let inside = mask::point_inside(&outline, sx, sy);
                    let hit = if expansion == 0.0 {
                        inside
                    } else {
                        let away = mask::distance_to_path(&outline, true, sx, sy);
                        away * if inside { 1.0 } else { -1.0 } + expansion >= 0.0
                    };
                    total += hit as u64;
                }
            }
        }
    }
    let ms = began.elapsed().as_secs_f64() * 1000.0;
    assert!(total > 0, "the mask must cover something");
    ms
}

/// The same measurement with rayon held to one thread, so the threading can be told from the rule.
fn on_one_thread(mask: &Mask, width: usize, height: usize, runs: usize) -> f64 {
    rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .expect("a pool of one")
        .install(|| median_ms(mask, width, height, runs))
}

#[test]
#[ignore = "a measurement; run with --release --ignored"]
fn mask_cost() {
    // Layer sizes: a 1080p cel, a 720p one, and the quarter-size one a draft preview does not
    // actually get (a mask runs in layer space, which is why Draft did not help the owner).
    let sizes = [(1920usize, 1080usize), (1280, 720), (480, 270)];
    // Eight corners, each one a bezier segment that flattens to at least MIN_PIECES edges, so the
    // outline is in the hundreds where a hand-drawn one is.
    let corners = 8;
    let mut out = String::new();
    out.push_str("# P-15: what a mask costs to rasterise\n\n");
    out.push_str(
        "Produced by `tests/p15_mask_cost.rs`, which is `#[ignore]`d in normal runs. \
         **This measures `mask::coverage` and nothing around it.**\n\n\
         The owner's window measured 17750.1 ms inside a single Draft frame while a mask point \
         was being dragged, against the 2.502 ms `verification/P-01_frame_trace.md` measures for \
         a warm Draft frame with no mask on it. This table is where that time was.\n\n",
    );
    out.push_str("## Machine, build and configuration\n\n");
    out.push_str(&format!(
        "- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads\n\
         - OS: Microsoft Windows 11 Education, 10.0.26200\n\
         - Toolchain: cargo release profile, `opt-level = 3`\n\
         - Debug assertions in this build: {}\n\
         - One mask, mode Add, opacity 1. Outline of {corners} bezier corners.\n\
         - Threads available to rayon: {}\n\n",
        cfg!(debug_assertions),
        rayon::current_num_threads(),
    ));
    out.push_str(
        "## The field, sample by sample and by scanline\n\n\
         No feather in this table: the field is the part P-15 changed. \"Sample by sample\" is the \
         rule the build followed until 2026-09-22, one run; \"now\" is the median of five.\n\n\
         | Layer | Mask | Outline edges | Sample by sample, ms | Now, ms | Times faster |\n\
         |---|---|---|---|---|---|\n",
    );
    let kinds: [(&str, f64); 2] = [("plain", 0.0), ("expanded 4 px", 4.0)];
    let mut worst_ratio = f64::INFINITY;
    for (width, height) in sizes {
        for (name, expansion) in kinds {
            let radius = (width.min(height) as f64) * 0.4;
            let centre = (width as f64 / 2.0, height as f64 / 2.0);
            let mask = Mask {
                expansion_px: expansion,
                ..drawn_mask(corners, radius, centre)
            };
            let edges = mask.outline().len();
            let slow_ms = sample_by_sample_ms(&mask, width, height);
            let fast_ms = median_ms(&mask, width, height, 5);
            worst_ratio = worst_ratio.min(slow_ms / fast_ms);
            let _ = writeln!(
                out,
                "| {width}x{height} | {name} | {edges} | {slow_ms:.1} | {fast_ms:.2} | {:.0}x |",
                slow_ms / fast_ms
            );
        }
    }
    let _ = write!(
        out,
        "\nThe smallest gain in the table is {worst_ratio:.0} times.\n\n",
    );

    out.push_str(
        "## One thread and all of them\n\n\
         The same call at 1920x1080, run inside a rayon pool of one thread and inside the default \
         pool, median of five each. This separates what the new rule saved from what the threads \
         saved, and it is the only honest way to say how much the threads are worth on a machine \
         that has twenty-four of them and a layer that has two million pixels.\n\n\
         | Mask | One thread, ms | All threads, ms | Times faster |\n\
         |---|---|---|---|\n",
    );
    let feathered: [(&str, f64, f64); 3] = [
        ("plain", 0.0, 0.0),
        ("expanded 4 px", 4.0, 0.0),
        ("feathered 20 px", 0.0, 20.0),
    ];
    for (name, expansion, feather) in feathered {
        let radius = 1080.0 * 0.4;
        let mask = Mask {
            expansion_px: expansion,
            feather_px: feather,
            ..drawn_mask(corners, radius, (960.0, 540.0))
        };
        let one = on_one_thread(&mask, 1920, 1080, 5);
        let many = median_ms(&mask, 1920, 1080, 5);
        let _ = writeln!(out, "| {name} | {one:.2} | {many:.2} | {:.1}x |", one / many);
    }

    out.push_str(
        "\n## How to read this\n\n\
         **The layer size is the layer's, not the preview's.** A mask is rasterised in layer \
         space, before the transform and before the composite, so a Draft preview of a 1080p cel \
         still rasterises the mask at 1920x1080. That is why switching to Draft did nothing for \
         the owner's drag, and it is a property of where document 21 puts the mask rather than a \
         fault.\n\n\
         **A drag pays this once per update per masked layer.** The figure to compare against is \
         the 41.667 ms a 24 fps clock allows, and for a drag, the tens of milliseconds a hand \
         notices.\n\n\
         **The same picture, either column.** `verification/B-06_mask_table.md` is where that is \
         checked, not here: two of its rows compare the plain field against the sample-by-sample \
         field pixel for pixel, and two more compare the expanded field against an independently \
         written one. This file only says how long each took.\n",
    );
    fs::write(repo("verification/P-15_mask_cost.md"), out).expect("write the measurement");
}
