//! D-48: the export path already runs across threads, and the bytes do not depend on how many.
//!
//! ADR-015 bound 3 says the export path "gains nothing and must not", and two units in a row -
//! P-03(b) and P-10 - have read that sentence, been unable to tell whether it forbids a *cache*
//! or forbids *threads*, and left the question open rather than guess. D-48 asked the owner to
//! settle the wording, and on 2026-09-12 it was settled: bound 3 now names the cache -"the export
//! path neither reads nor writes the cel cache or the effect cache" - and a thread count is not a
//! cache. This file is the evidence that decision rested on, and it stays, because what made the
//! reading safe to discard is a check rather than an argument: it is a check rather than a
//! measurement, so it runs on every build.
//!
//! # What it shows
//!
//! ADR-011 has rendered every frame tiled across the rayon pool since B-05a, and
//! `crate::export` renders through `compose::render_frame` like everything else. So threads have
//! been running on the export path since long before there was a cache to keep off it, and bound
//! 3 has never been read as forbidding them. What this file does is say so in an artifact: the
//! same export frames rendered in a pool of **one** thread and in a pool of **twenty-four**,
//! digested, and the two digests compared.
//!
//! The thread counts are written down rather than taken from the machine, because
//! `verification/` is compared byte for byte on every build and a table that said "24" here and
//! "2" on a smaller runner would fail that gate for a reason that has nothing to do with the
//! renderer. A pool may hold more threads than the machine has cores; that is fine, and it is
//! what makes the high row the same row everywhere.
//!
//! # What it does not show
//!
//! Nothing about caching. Both columns hold `CelCache::none()`, which is what export holds, so
//! this file has no opinion on bound 3's actual subject and does not touch it. It removes one
//! reading of the sentence - that threads are what bound 3 forbids - by showing that reading is
//! already false in the shipped build.

use std::fmt::Write as _;
use std::path::PathBuf;

use anime_compositor::compose::{self, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;

mod common;
use common::{build_fixture, repo};

const COMP: &str = "comp-reference-shot";

/// H-01's frames, and frame 14 for the same reason as everywhere else: it is the one whose
/// drawing is missing, so its plan has one fewer layer than the others.
const FRAMES: [i32; 4] = [0, 14, 100, 239];

/// Written down rather than read from the machine, so the artifact is the same on every runner.
const POOLS: [usize; 2] = [1, 24];

struct Workload {
    name: &'static str,
    project: Project,
    root: PathBuf,
}

fn workloads() -> Vec<Workload> {
    let path = repo("verification/B-08a_project.json");
    let loaded =
        persist::load(&path).unwrap_or_else(|d| panic!("open {}: {}", path.display(), d.message));
    let (project, root, _text) = build_fixture("d48");
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

/// FNV-1a, 64 bit, over the encoded frame - the same hash and the same reason as
/// `tests/p03_byte_equality.rs`: the question is whether these bytes changed, not whether someone
/// could forge them.
fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// One export frame, rendered in a pool of exactly `threads`.
///
/// `compose::render_frame` is the function `crate::export` calls, so this is the export path and
/// not an imitation of it.
fn export_frame(w: &Workload, frame: i32, threads: usize) -> Vec<u8> {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap_or_else(|e| panic!("build a pool of {threads}: {e}"));
    pool.install(|| {
        let mut log = FrameLog::new(3);
        compose::render_frame(
            &w.project,
            &Id::new(COMP),
            frame,
            &w.root,
            DEFAULT_TILE_SIZE,
            &mut log,
        )
        .unwrap_or_else(|d| panic!("{} frame {frame}: {}", w.name, d.message))
        .to_srgb8_straight()
    })
}

#[test]
fn d48_export_is_the_same_frame_however_many_threads_render_it() {
    let mut rows = String::new();
    let mut checks = 0;

    for w in &workloads() {
        for &frame in &FRAMES {
            let low = export_frame(w, frame, POOLS[0]);
            let high = export_frame(w, frame, POOLS[1]);

            assert_eq!(
                low.len(),
                high.len(),
                "{} frame {frame}: the two pools produced frames of different sizes",
                w.name
            );
            assert!(
                low == high,
                "{} frame {frame}: rendering the export path in a pool of {} thread(s) produced \
                 different bytes from a pool of {} thread(s). ADR-011 carves the frame into one disjoint \
                 set of row slices per tile before any thread starts, so no two tiles can ever \
                 be handed the same pixel; if that is still true, the difference is somewhere \
                 else and this is the test that found it.",
                w.name,
                POOLS[0],
                POOLS[1],
            );
            checks += 1;

            let _ = writeln!(
                rows,
                "| {} | {frame} | {} | `{:016x}` | `{:016x}` | pass |",
                w.name,
                low.len(),
                fnv1a(&low),
                fnv1a(&high),
            );
        }
    }

    let page = format!(
        "# D-48: the export frame does not depend on how many threads rendered it\n\n\
         Written by `d48_export_is_the_same_frame_however_many_threads_render_it` in \
         `tests/d48_export_threads.rs`, which runs on every build. **{checks} of {checks} \
         checks pass.**\n\n\
         ADR-015 bound 3 once said the export path \"gains nothing and must not\", and two units \
         in a row could not tell from that sentence whether it forbids a *cache* or forbids \
         *threads*. **D-48 settled it on 2026-09-12**: the bound now reads that the export path \
         neither reads nor writes the cel cache or the effect cache, because a thread count is \
         not a cache. This page is the check that made that safe to say, and it keeps running so \
         that it stays true.\n\n\
         Each row is one frame of the export path - `compose::render_frame`, the function \
         `crate::export` calls - rendered twice: once in a rayon pool of **{low} thread** and \
         once in a pool of **{high}**. Both hold `CelCache::none()`, which is what export holds, \
         so nothing here is cached and this page has no opinion on bound 3's actual subject.\n\n\
         The two thread counts are written into the test rather than read from the machine, so \
         that this artifact is the same on every runner. A pool may hold more threads than the \
         machine has cores.\n\n\
         | Fixture | Frame | Bytes | Digest at {low} thread | Digest at {high} threads | Result |\n\
         |---|---|---|---|---|---|\n{rows}\n\
         ## What this settles, and what it does not\n\n\
         It settles one reading of bound 3, and that reading is now retired. Threads have run on \
         the export path since B-05a, because ADR-011 renders every frame tiled across the rayon \
         pool and export renders through the same function the viewer does. Whatever bound 3 \
         forbids, it has never been read as forbidding that, and the frames above are identical \
         either way.\n\n\
         What it does **not** do is give export a parallel decode. The parallel decode this build \
         has is `CelCache::prewarm`, which returns early at a zero budget, and export holds a \
         zero budget. Building one would be a unit of its own with byte-equality evidence of its \
         own. The amended bound removes the reason not to; it does not do the work.\n\n\
         It settles nothing about caching, which is what bound 3 is for. A full-resolution \
         preview matching an export in 0 of 8,294,400 samples is a property this build has, and \
         a cache reachable from export is the obvious way to lose it. That property is guarded by \
         `tests/p03_byte_equality.rs` and is not touched here.\n",
        low = POOLS[0],
        high = POOLS[1],
    );
    std::fs::write(repo("verification/D-48_export_threads.md"), page).expect("write the artifact");
}
