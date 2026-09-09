//! P-03(f): the tile size a draft frame is cut into.
//!
//! Document 15's P-03, item (f): "a draft tile size chosen for a 480 by 270 frame, which today
//! cuts into twelve tiles for twenty-four threads, while `DEFAULT_TILE_SIZE` stays what it is for
//! export". `compose::DEFAULT_TILE_SIZE` is 128 pixels and it was measured — by
//! `verification/B-05a_scaling_table.md`, on a 1920x1080 frame. A draft preview is a quarter of
//! that in each direction, so the same constant cuts it into four columns of three, and twelve
//! pieces of work cannot fill twenty-four threads however fast each piece is.
//!
//! Document 21: "Tile size is a tunable measured on the reference machine, not a constant chosen
//! in advance." This file is that measurement for the draft extent. It renders the same draft
//! plans at several tile sizes, times only `render::render` — the decode is not what this item
//! touches and would drown what it does — and **compares every render against the 128px one byte
//! for byte**, so a size is only reported if it changed nothing but the clock. That is the same
//! guarantee `verification/B-07_effects_table.md` already makes at six tile sizes and ADR-011
//! makes in general; this file re-makes it for the sizes it is proposing.
//!
//! It writes `verification/P-03f_draft_tile_size.md` and is `#[ignore]`d, like every other
//! measurement in this repository: a timing run inside the normal suite is a flaky test.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use anime_compositor::cache::CelCache;
use anime_compositor::compose;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::render::{self, FramePlan};

mod common;
use common::{build_fixture, repo};

const COMP: &str = "comp-reference-shot";
/// The sizes swept. 128 is the incumbent and the byte-for-byte reference; the rest divide a
/// 480x270 draft frame into progressively more pieces, down to a size where the per-tile overhead
/// is expected to win the argument back.
const SIZES: [usize; 7] = [128, 96, 64, 48, 32, 24, 16];
/// Frames per size. Each one is rendered from a plan built once, so this is repetitions of the
/// renderer and nothing else.
const SAMPLES: usize = 20;
/// Whole sweeps of the size list. Two, because one sweep cannot tell a tile size from a noisy
/// minute.
const PASSES: usize = 2;
/// Coprime with the 240 frames of the work area, as in `tests/p01_frame_trace.rs`.
const SCATTER: i32 = 97;
/// Room for every drawing the sample touches, so that no plan waits on a decode.
const WARM_BUDGET_BYTES: usize = 6 * 1024 * 1024 * 1024;
/// 24 fps as a frame budget in milliseconds.
const FRAME_BUDGET_MS: f64 = 1000.0 / 24.0;

/// One line of the table: a size, how many tiles it cuts the draft frame into, and its median
/// render time in each sweep.
struct Row {
    workload: &'static str,
    layers: usize,
    size: usize,
    tiles: usize,
    passes: [f64; PASSES],
}

struct Workload {
    name: &'static str,
    project: Project,
    root: PathBuf,
    layers: usize,
}

fn workloads() -> Vec<Workload> {
    let path = repo("verification/B-08a_project.json");
    let loaded =
        persist::load(&path).unwrap_or_else(|d| panic!("open {}: {}", path.display(), d.message));
    let (project, root, _text) = build_fixture("p03f");
    vec![
        Workload {
            name: "the reference shot",
            project: loaded.document.project().clone(),
            root: repo("Fixtures/reference_shot"),
            layers: 4,
        },
        Workload {
            name: "the declared ten-layer fixture",
            project,
            root,
            layers: 10,
        },
    ]
}

/// FNV-1a, 64 bit, over the encoded frame. The same hash and the same reason as
/// `tests/p03_byte_equality.rs`: the question is "did these bytes change", not "can this be
/// forged".
fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

fn ns_ms(ns: u64) -> f64 {
    ns as f64 / 1_000_000.0
}

fn p50(mut xs: Vec<f64>) -> f64 {
    xs.sort_by(|a, b| a.partial_cmp(b).expect("a timing was NaN"));
    xs[xs.len() / 2]
}

/// The draft plans for the sampled frames, built once with a cache big enough to hold them all.
fn draft_plans(workload: &Workload) -> Vec<FramePlan> {
    let comp = Id::new(COMP);
    let mut cache = CelCache::with_budget(WARM_BUDGET_BYTES);
    (0..SAMPLES as i32)
        .map(|i| (i * SCATTER) % 240)
        .map(|frame| {
            let mut log = FrameLog::new(3);
            let plan = compose::plan_frame_cached(
                &workload.project,
                &comp,
                frame,
                &workload.root,
                &mut log,
                &mut cache,
            )
            .unwrap_or_else(|d| panic!("{} frame {frame}: {}", workload.name, d.message));
            preview::scale_plan(plan, PreviewQuality::Draft)
        })
        .collect()
}

#[test]
#[ignore = "P-03(f): a measurement, run deliberately with --release --ignored"]
fn p03f_draft_tile_size() {
    let mut rows: Vec<Row> = Vec::new();

    for workload in &workloads() {
        let plans = draft_plans(workload);
        let (width, height) = (plans[0].width, plans[0].height);
        let mut reference: Vec<u64> = Vec::new();
        let mut passes: Vec<[f64; PASSES]> = vec![[0.0; PASSES]; SIZES.len()];

        // Two whole sweeps rather than two batches per size, so that a size is never measured
        // twice in a row: whatever the machine was doing during one sweep, the other sweep saw a
        // different half of it. P-03(e) was decided the wrong way round by a single sweep once
        // already, and this table is smaller than that one was.
        for pass in 0..PASSES {
            for (index_of_size, &size) in SIZES.iter().enumerate() {
                // One untimed render first, as B-05a does, so that the figure is the renderer and
                // not the first touch of a freshly allocated frame.
                let _ = render::render(&plans[0], size);

                let mut times = Vec::with_capacity(plans.len());
                for (index, plan) in plans.iter().enumerate() {
                    let at = Instant::now();
                    let frame = render::render(plan, size);
                    times.push(ns_ms(at.elapsed().as_nanos() as u64));

                    let digest = fnv1a(&frame.to_srgb8_straight());
                    if pass == 0 && size == SIZES[0] {
                        reference.push(digest);
                    } else {
                        assert_eq!(
                            digest, reference[index],
                            "{}: rendering draft frame {index} in {size}px tiles produced                              different bytes from rendering it in {}px tiles. ADR-011 requires a                              tiled render to equal a whole-frame one, so no tile size may change                              a picture, and this one did.",
                            workload.name, SIZES[0]
                        );
                    }
                }
                passes[index_of_size][pass] = p50(times);
            }
        }

        for (index_of_size, &size) in SIZES.iter().enumerate() {
            rows.push(Row {
                workload: workload.name,
                layers: workload.layers,
                size,
                tiles: render::tiles(width, height, size).len(),
                passes: passes[index_of_size],
            });
        }
    }

    write_artifact(&rows);
}

fn write_artifact(rows: &[Row]) {
    let mut s = String::from("# P-03(f) the draft tile size\n\n");
    s.push_str(
        "Document 15's P-03, item (f). `compose::DEFAULT_TILE_SIZE` is 128 pixels because \
         `verification/B-05a_scaling_table.md` measured it on a 1920x1080 frame. A draft preview \
         is 480x270 - a quarter in each direction - and the same 128 pixels cut that into four \
         columns of three: **twelve pieces of work for twenty-four hardware threads**, so half \
         the machine has nothing to do however fast each piece is.\n\nThis is the same \
         measurement B-05a made, made again at the extent the viewer actually renders at. \
         Produced by `tests/p03f_draft_tile_size.rs`, which is `#[ignore]`d in normal runs.\n\n",
    );
    s.push_str("## Machine, build and configuration\n\n");
    s.push_str(
        "- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads\n\
         - OS: Microsoft Windows 11 Education, 10.0.26200\n\
         - Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`\n\
         - Workload: the draft plan of twenty frames scattered across each fixture's 240-frame \
         work area, 480x270\n\
         - Only `render::render` is timed. The decode, the transfer function and the encode are \
         outside the span on purpose: this item moves none of them, and at a first playthrough \
         they are large enough to hide it entirely\n\
         - Each figure is the median of twenty renders, preceded by one untimed render at the \
         same size\n\n",
    );
    let _ = writeln!(
        s,
        "Debug assertions in this build: {}. A run with `true` there is a debug build, and its \
         numbers say more about the compiler than about the renderer.\n",
        cfg!(debug_assertions)
    );

    s.push_str("## Measurements\n\n");
    s.push_str(
        "| Workload | Layers | Tile | Tiles | Sweep 1 p50 (ms) | Sweep 2 p50 (ms) |          Against 128px |
",
    );
    s.push_str(
        "|---|---|---|---|---|---|---|
",
    );
    let mut baseline = [0.0_f64; PASSES];
    for row in rows {
        if row.size == SIZES[0] {
            baseline = row.passes;
        }
        let against = if row.size == SIZES[0] {
            String::from("-")
        } else {
            row.passes
                .iter()
                .zip(baseline.iter())
                .map(|(ms, base)| format!("{:+.1}%", 100.0 * (ms - base) / base))
                .collect::<Vec<String>>()
                .join(", ")
        };
        let _ = writeln!(
            s,
            "| {} | {} | {}px | {} | {:.3} | {:.3} | {against} |",
            row.workload, row.layers, row.size, row.tiles, row.passes[0], row.passes[1]
        );
    }

    s.push_str("\n## What to check by eye\n\n");
    s.push_str(
        "Two things, and the second one matters more than the first.\n\n**The tile column and \
         the tiles column.** Twelve tiles is the row this item exists to replace, and every \
         smaller size gives the machine more pieces than it has threads. The point where the \
         time stops falling is where per-tile overhead starts costing more than the extra \
         parallelism buys, and that point is what the constant is set to - not the smallest size \
         in the table.\n\n**Every row rendered the same picture.** The test compares each \
         render's encoded bytes against the 128px render of the same frame and fails if one byte \
         differs, so the whole table is forty renders of two pictures. That is ADR-011's \
         requirement and the same guarantee `verification/B-07_effects_table.md` makes at six \
         tile sizes; a tile size is a schedule, never a picture.\n\n",
    );
    let _ = writeln!(
        s,
        "For scale, the 24 fps budget document 08 sets is {FRAME_BUDGET_MS:.3} ms for the whole \
         frame, of which this table is one stage.\n"
    );
    s.push_str(
        "\n## What this changes\n\n\
         `compose::DRAFT_TILE_SIZE` is the row this table picks, and `preview::preview_frame` \
         uses it when the quality is `Draft`. `DEFAULT_TILE_SIZE` is untouched, which is what \
         document 15 requires: **export still renders in 128px tiles**, measured on the extent \
         it renders at, and ADR-015's separation of preview from export is not disturbed by a \
         preview-only tuning.\n",
    );

    let path = repo("verification/P-03f_draft_tile_size.md");
    fs::write(&path, s).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
    println!("wrote {}", path.display());
}
