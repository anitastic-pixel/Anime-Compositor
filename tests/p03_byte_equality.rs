//! P-03's proof that an optimisation moved no pixel: every frame of both fixtures, at both
//! qualities, reduced to one number a difference of one byte would change.
//!
//! Writes `target/p03_manifest.txt` under `--release --ignored`:
//!
//! ```text
//! cargo test --release --test p03_byte_equality -- --ignored
//! ```
//!
//! # Why this exists rather than a run of `b10_full_shot`
//!
//! Document 15's P-03 asks each item in it to prove that "all 240 frames of both fixtures
//! exported before and after the change" are byte-identical, and says an item that moves one
//! byte is reverted rather than given a tolerance. `tests/b10_full_shot.rs` compares two exports
//! of one shot **within a single run**, which is determinism; it cannot compare a run before a
//! change against a run after it, because nothing of pass 1 survives the rebuild. The 480 frames
//! it writes are half a gigabyte, so keeping them across a rebuild is not the answer either.
//!
//! What survives a rebuild cheaply is a manifest: one line a frame, and the line changes if any
//! byte of that frame changes. Run it on the commit before the change, keep the file, run it on
//! the change, and `diff` the two. The frames themselves are still the evidence for B-10 and for
//! the H-series; this file's evidence is that the bytes did not move.
//!
//! # What is hashed
//!
//! The same buffer `app/src/main.rs` hands the page and `tests/p01_frame_trace.rs` times to:
//! `preview_frame_cached` followed by `WorkingBuffer::to_srgb8_straight`. That is deliberately
//! the *encoded* bytes rather than the f32 working buffer, because those are the pixels a viewer
//! sees, and because a change that perturbs a working value below the last sRGB step is one this
//! project would still rather know about — it will show up here the moment it crosses a step on
//! any one of 1920 x 1080 pixels across 960 renders.
//!
//! Both qualities are covered because Draft and Full take different paths through the sampler,
//! and an item like P-03(f) touches only one of them.
//!
//! The manifest is `CelCache::none()` throughout: a cel cache changes what is recomputed, never
//! what is computed, and `tests/b08b_cache.rs` is where that claim is tested. Leaving it out keeps
//! the manifest's answer independent of the budget.
//!
//! Every frame is then rendered a **second** time, through one cache with the viewer's own D-40
//! budget, and the two encodes are compared byte for byte (P-03(b)). That is `b08b`'s rule, which
//! covers the reference shot at several budgets, extended to the declared ten-layer fixture, and
//! P-03(b) is why it is needed here: it made a budgeted render decode its cels on several threads
//! ahead of the layer loop instead of one at a time inside it, so "with a budget" and "without
//! one" are no longer the same schedule of the same work. The comparison is inside one run and
//! therefore cannot answer a question about a code change — that is what the manifest is for —
//! but the question it does answer is the whole of ADR-015's worry.
//!
//! # What is asserted
//!
//! Nothing about the hashes — this file has no expected values and invents none, which is
//! ADR-009's line. It asserts only that every frame rendered and that the encode produced a
//! buffer of the right shape, so a manifest of 960 lines is 960 frames and not 960 failures.
//! The comparison is the `diff`, and it is a human's.

use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};

mod common;
use common::{build_fixture, repo};

const COMP: &str = "comp-reference-shot";
const FRAMES: i32 = 240;

struct Workload {
    name: &'static str,
    project: Project,
    root: PathBuf,
}

fn workloads() -> Vec<Workload> {
    let path = repo("verification/B-08a_project.json");
    let loaded =
        persist::load(&path).unwrap_or_else(|d| panic!("open {}: {}", path.display(), d.message));
    let (project, root, _text) = build_fixture("p03");
    vec![
        Workload {
            name: "reference_shot",
            project: loaded.document.project().clone(),
            root: repo("Fixtures/reference_shot"),
        },
        Workload {
            name: "declared_fixture",
            project,
            root,
        },
    ]
}

/// FNV-1a, 64 bit, over the encoded frame.
///
/// ponytail: a non-cryptographic hash, because the question is "did these bytes change between
/// two builds of this repository", not "can someone forge a frame". If P-03 ever has to defend a
/// hash against something other than an accident, this is where a real digest goes — and that
/// means a new dependency, `docs/DEPENDENCIES.md` and the licence archive, which is why it is not
/// here for a question that does not need it.
fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

#[test]
#[ignore = "P-03: 960 renders, run deliberately with --release --ignored"]
fn p03_byte_equality() {
    let comp = Id::new(COMP);
    let mut manifest = String::new();
    for workload in &workloads() {
        for quality in [PreviewQuality::Draft, PreviewQuality::Full] {
            let mut cache = CelCache::none();
            // One cache for the whole pass, so that the frames a fan-out decodes ahead, the ones
            // it finds already held and the ones it evicts all happen the way they do in a
            // viewer, rather than being reset between frames.
            let mut budgeted = CelCache::viewer();
            for frame in 0..FRAMES {
                let mut log = FrameLog::new(3);
                let buffer = preview::preview_frame_cached(
                    &workload.project,
                    &comp,
                    frame,
                    &workload.root,
                    quality,
                    DEFAULT_TILE_SIZE,
                    &mut log,
                    &mut cache,
                )
                .unwrap_or_else(|d| panic!("{} frame {frame}: {}", workload.name, d.message));
                let pixels = buffer.to_srgb8_straight();
                assert_eq!(
                    pixels.len(),
                    buffer.width() * buffer.height() * 4,
                    "{} frame {frame}: the encode produced a buffer of the wrong shape",
                    workload.name
                );
                let mut budgeted_log = FrameLog::new(3);
                let with_budget = preview::preview_frame_cached(
                    &workload.project,
                    &comp,
                    frame,
                    &workload.root,
                    quality,
                    DEFAULT_TILE_SIZE,
                    &mut budgeted_log,
                    &mut budgeted,
                )
                .unwrap_or_else(|d| {
                    panic!(
                        "{} frame {frame}, with a budget: {}",
                        workload.name, d.message
                    )
                })
                .to_srgb8_straight();
                assert!(
                    with_budget == pixels,
                    "{} {} frame {frame}: rendering with a {} cache produced different bytes \
                     from rendering with none. Document 27 requires the two to be equivalent and \
                     ADR-015 keeps the cache off the export path precisely so that this can \
                     never reach a picture; it has still gone wrong.",
                    workload.name,
                    quality.label(),
                    anime_compositor::cache::budget_label(
                        anime_compositor::cache::DEFAULT_BUDGET_BYTES
                    ),
                );

                manifest.push_str(&format!(
                    "{} {} {:04} {} {}x{} {:016x}\n",
                    workload.name,
                    quality.label(),
                    frame,
                    pixels.len(),
                    buffer.width(),
                    buffer.height(),
                    fnv1a(&pixels),
                ));
            }
        }
    }

    let out = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("p03_manifest.txt");
    fs::create_dir_all(out.parent().unwrap()).expect("create target/");
    fs::write(&out, manifest.as_bytes()).unwrap_or_else(|e| panic!("write {}: {e}", out.display()));
    println!("wrote {}", out.display());
}
