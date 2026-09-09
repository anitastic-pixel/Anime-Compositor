//! B-08b: the bounded cache of decoded cels (R-06b), specified by document 27.
//!
//! This is the one module in the crate that exists for speed rather than for correctness, and
//! document 27 states the rule that follows from that: "Caching may improve interaction but must
//! never define correctness. A cold render and a fully warm render for the same immutable request
//! must produce equivalent pixels and diagnostics." So the shape here is deliberately small.
//!
//! **What it holds.** One decoded cel, after document 21's step 1: decoded, re-tagged with the
//! asset's interpretation, and converted into the working space. Not a finished frame. D-37
//! measured the reason: 75.15 ms of an 81.69 ms draft preview frame is reading and decoding the
//! four cels it needs, and 6.53 ms is rendering them into a picture. A cache of frames would
//! chase the 6.53.
//!
//! **What it is keyed on.** Document 27: "File path alone is not sufficient media identity. At
//! minimum include observed size/mtime for interactive invalidation." So the key is the path, the
//! file's length, its modification time, and the interpretation the buffer was tagged with — the
//! last because the conversion into the working space is inside what is being remembered, so two
//! assets reading the same file differently must not share an entry. A file whose metadata cannot
//! be read is decoded and not stored, because an entry that cannot notice the file changing is
//! worse than no entry.
//!
//! **How it is bounded.** In bytes actually held, not in entries. A cel is 1920x1080 f32 RGBA,
//! about 33 MB, which is four times what the PNG decoder writes for the same cel and is the
//! figure `verification/D-37_decode_cost.md` now quotes, having originally priced a held cel at
//! the decoder's 8-bit output. Eviction is least-recently-used and changes performance only, which is document 27's
//! requirement and is checked as a byte comparison rather than asserted.
//!
//! **Where it is not.** Export never sees one. [`crate::compose::plan_frame`] and
//! [`crate::export`] use [`CelCache::none`], whose budget is zero and which therefore cannot store
//! anything; only the preview path is handed a real one. A full-resolution preview and an export
//! of the same frame differ in 0 of 8,294,400 samples, and a cache is the obvious way to lose
//! that. ADR-015 makes keeping it out of the export path part of the decision rather than an
//! implementation choice.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::SystemTime;

use rayon::prelude::*;

use crate::compose::retag;
use crate::diagnostics::Diagnostic;
use crate::media;
use crate::model::Interpretation;
use crate::WorkingBuffer;

/// The budget the viewer uses when nobody says otherwise.
///
/// Document 27: "Define a configurable cache budget after measuring the reference machine." It was
/// measured, and `verification/B-08b_cache_budget.md` is that measurement: forty-eight consecutive
/// frames of the reference shot at four budgets, on the reference machine, in a release build.
///
/// The first number came out of that table rather than out of a guess about it: 128 MB took a
/// draft preview frame of the reference shot from 100.29 ms to 42.55 ms, and 512 MB, four times
/// the memory, took it to 42.16 ms. On a four-layer shot the gap is smaller than the run-to-run
/// noise, because playback is sequential - a cel is asked for again within a few frames of first
/// use or not for a long time, so what has to fit is the reuse distance and not the shot.
///
/// **That table measured the wrong shot, and `verification/T-06_declared_fixture.md` measured the
/// right one.** Document 08 line 41 declares a ten-layer fixture. One cel of it costs 33,177,600
/// bytes to hold and one frame needs ten of them, which is 316.4 MiB, so at 128 MB the cache
/// could not hold a single frame of the shot the project's own performance target is written
/// against: showing a frame evicted the cels that made it, every loop decoded 2,340 cels against
/// 480 from memory, and asking for the frame just shown cost 371.71 ms at the median. At a budget
/// with room for one frame the same request cost 190.15 ms. That is the whole of what this number
/// decides, and it is worth about half the cost of a repeated frame on the declared fixture.
///
/// One gibibyte is 32 cels of that size: a frame of the heaviest fixture this project declares,
/// plus two more frames' worth of neighbourhood to scrub inside. It is a ceiling and not an
/// allocation - the cache holds what it has been given, so a four-layer shot still holds four
/// layers' worth and this costs it nothing. Reaching the ceiling on the reference machine's 32 GB
/// is about 1.3 GB of working set.
///
/// **What it does not buy is the target.** 190.15 ms is not document 08 line 41's 100 ms, and no
/// budget reaches it: with all ten cels in memory the remaining cost is compositing, not decoding.
/// Raising this closes half of D-40 and leaves the other half open for the owner.
pub const DEFAULT_BUDGET_BYTES: usize = 1024 * 1024 * 1024;

/// The way a budget is written in the pages that quote one, so a page and the constant it is
/// quoting cannot drift apart. Every artifact that names the default calls this rather than
/// spelling the number, which is how "128 MB" ended up in four committed pages.
pub fn budget_label(bytes: usize) -> String {
    let mib = bytes / (1024 * 1024);
    if mib >= 1024 && mib.is_multiple_of(1024) {
        format!("{} GiB", mib / 1024)
    } else {
        format!("{mib} MiB")
    }
}

/// What makes two requests for a decoded cel the same request.
#[derive(PartialEq, Eq, Debug)]
struct Key {
    path: PathBuf,
    len: u64,
    modified: SystemTime,
    interpretation: Interpretation,
}

impl Key {
    /// `None` when the file's metadata cannot be read, which makes the request uncacheable rather
    /// than an error: the decode that follows will report the problem properly if there is one.
    fn of(path: &Path, interpretation: Interpretation) -> Option<Key> {
        let meta = std::fs::metadata(path).ok()?;
        Some(Key {
            path: path.to_path_buf(),
            len: meta.len(),
            modified: meta.modified().ok()?,
            interpretation,
        })
    }
}

/// A bounded, least-recently-used cache of decoded cels in the working space.
///
/// Constructed either [`with_budget`](Self::with_budget) or as [`none`](Self::none), which is a
/// cache that can never hold anything and is what every path outside the preview uses.
pub struct CelCache {
    budget: usize,
    held: usize,
    /// Least recently used first. ponytail: a linear scan, because this holds tens of entries and
    /// a frame asks it four questions; a map plus an intrusive list if a real project ever makes
    /// the scan measurable.
    entries: Vec<(Key, Arc<WorkingBuffer>)>,
    /// Cels [`prewarm`](Self::prewarm) decoded ahead of the layer loop and that no request has
    /// collected yet (P-03(b)). Not part of `held`: a pending cel has not been admitted, and
    /// [`decoded`](Self::decoded) admits it through [`store`](Self::store) like any other miss,
    /// in the order the layer loop asks for it, so the budget, the eviction order and the hit
    /// and miss counts are exactly what they were when every decode was serial.
    pending: Vec<(Key, Arc<WorkingBuffer>)>,
    hits: u64,
    misses: u64,
    evicted: u64,
}

impl CelCache {
    /// A cache that may hold up to `budget` bytes of decoded cels.
    pub fn with_budget(budget: usize) -> CelCache {
        CelCache {
            budget,
            held: 0,
            entries: Vec::new(),
            pending: Vec::new(),
            hits: 0,
            misses: 0,
            evicted: 0,
        }
    }

    /// A cache that holds nothing, ever. Export and every non-preview caller use this.
    ///
    /// It is a real cache with a zero budget rather than an `Option<&mut CelCache>` at every call
    /// site, so there is exactly one code path through [`decoded`](Self::decoded) and the "no
    /// cache" case is exercised by every test that renders a frame.
    pub fn none() -> CelCache {
        CelCache::with_budget(0)
    }

    /// The cel at `path`, tagged as `interpretation` says and converted into the working space.
    ///
    /// Decoded on a miss, remembered if it fits, returned from memory on a hit. The result is the
    /// same buffer either way: this function has one path that produces pixels and one that hands
    /// back pixels it already produced.
    ///
    /// **It is shared, not copied** (P-03(a)). A held cel is 33,177,600 bytes and
    /// `verification/P-01_frame_trace.md` measured copying it at up to 55.6% of a warm frame, for
    /// a copy almost no caller needed: a layer with no mask and no effects only ever reads its
    /// source. A caller that does need to write on it says so with [`Arc::make_mut`], which copies
    /// exactly then and never otherwise — `src/compose.rs` is the one caller that does, and
    /// `tests/b06_mask.rs`'s cache isolation check is what proves the copy really happens rather
    /// than trusting this paragraph.
    pub fn decoded(
        &mut self,
        path: &Path,
        interpretation: Interpretation,
    ) -> Result<Arc<WorkingBuffer>, Diagnostic> {
        let key = Key::of(path, interpretation);

        if let Some(key) = &key {
            let hit = crate::perf::time(crate::perf::Stage::CacheHit, || {
                let at = self.entries.iter().position(|(k, _)| k == key)?;
                let entry = self.entries.remove(at);
                let buffer = Arc::clone(&entry.1);
                self.entries.push(entry);
                Some(buffer)
            });
            if let Some(buffer) = hit {
                self.hits += 1;
                return Ok(buffer);
            }
        }

        self.misses += 1;
        // A cel `prewarm` decoded for this frame is still a miss: it was decoded, just earlier
        // and on another thread. Counting it as a hit would report a cache that answered a
        // request it never held, and `verification/B-08b_cache_table.md` is a table of exactly
        // those counts.
        let ready = key
            .as_ref()
            .and_then(|key| self.pending.iter().position(|(k, _)| k == key))
            .map(|at| self.pending.remove(at).1);
        let buffer = match ready {
            Some(buffer) => buffer,
            None => Arc::new(retag(media::decode_png(path)?, interpretation).into_working()),
        };
        if let Some(key) = key {
            crate::perf::time(crate::perf::Stage::CacheStore, || {
                self.store(key, Arc::clone(&buffer))
            });
        }
        Ok(buffer)
    }

    /// Decode the cels this frame is about to ask for, all at once, across the thread pool
    /// (P-03(b)).
    ///
    /// `verification/P-01_frame_trace.md` measured the read, the byte-to-float pass and the
    /// transfer function at three quarters of a cold draft frame, and a frame's cels are
    /// independent files: nothing about decoding one depends on another. What forbade doing them
    /// together was this `&mut CelCache`, threaded through `plan_frame_cached`'s layer loop, which
    /// makes the loop the only place a decode can happen. This moves the decode ahead of the loop
    /// and leaves the loop's shape alone.
    ///
    /// Three things it deliberately does not do.
    ///
    /// **It does not decide anything.** Every cel it decodes is one the loop was going to ask
    /// for; a key it fails on, or never had metadata for, is simply absent from `pending` and
    /// decodes serially in [`decoded`](Self::decoded) a moment later, where the diagnostic is
    /// raised and logged against the right layer exactly as before. That is why P-03(b)'s
    /// requirement to collect a `FrameLog` per layer and merge it in composition order does not
    /// appear here: no diagnostic is raised on a worker thread, so there is no order to restore.
    ///
    /// **It does not admit anything.** Nothing here touches `held`, the eviction order, or the
    /// hit and miss counts.
    ///
    /// **It does nothing at all without a budget.** [`none`](Self::none) is what export and every
    /// non-preview caller hold, and ADR-015 keeps the cache off that path; decoding in advance
    /// for a cache that cannot store would be work thrown away twice over.
    ///
    /// ponytail: the whole `wanted` set is decoded in one fan-out, so the transient peak is one
    /// frame's cels held at once. That is the frame's own working set, which is what the budget
    /// is chosen to hold; chunk it if a composition ever has more layers than the budget has
    /// room for.
    pub fn prewarm(&mut self, wanted: &[(PathBuf, Interpretation)]) {
        if self.budget == 0 || wanted.is_empty() {
            return;
        }
        let todo: Vec<Key> = wanted
            .iter()
            .filter_map(|(path, interpretation)| Key::of(path, *interpretation))
            .filter(|key| !self.entries.iter().any(|(k, _)| k == key))
            .fold(Vec::new(), |mut todo, key| {
                // A matte and the layer that uses it name the same file: decode it once.
                if !todo.contains(&key) {
                    todo.push(key);
                }
                todo
            });
        if todo.len() < 2 {
            // One cel is not a fan-out, and the serial path already times its three stages
            // properly. Nothing is lost by leaving it to the loop.
            return;
        }
        let decoded: Vec<(Key, Arc<WorkingBuffer>)> =
            crate::perf::time(crate::perf::Stage::Prewarm, || {
                todo.into_par_iter()
                    .filter_map(|key| {
                        // `untimed` because these three stages are running on however many
                        // threads rayon gave them, and adding their core time to the same
                        // counters the frame's wall-clock is measured against would make the
                        // stage table sum to more than the frame.
                        let buffer = crate::perf::untimed(|| {
                            let decoded = media::decode_png(&key.path).ok()?;
                            Some(retag(decoded, key.interpretation).into_working())
                        })?;
                        Some((key, Arc::new(buffer)))
                    })
                    .collect()
            });
        self.pending = decoded;
    }

    fn store(&mut self, key: Key, buffer: Arc<WorkingBuffer>) {
        let bytes = bytes_of(&buffer);
        // A cel larger than the whole budget is not stored at all. Evicting everything to hold
        // one thing that will be evicted by the next request is worse than not holding it.
        if bytes > self.budget {
            return;
        }
        self.entries.push((key, buffer));
        self.held += bytes;
        while self.held > self.budget {
            let (_, evicted) = self.entries.remove(0);
            self.held -= bytes_of(&evicted);
            self.evicted += 1;
        }
    }

    /// How many requests were answered from memory.
    pub fn hits(&self) -> u64 {
        self.hits
    }

    /// How many requests had to decode.
    pub fn misses(&self) -> u64 {
        self.misses
    }

    /// How many held cels were dropped to stay inside the budget.
    pub fn evictions(&self) -> u64 {
        self.evicted
    }

    /// How many bytes of decoded cels are held right now. Never more than the budget.
    pub fn held_bytes(&self) -> usize {
        self.held
    }

    /// How many cels are held right now.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The budget this cache was built with.
    pub fn budget(&self) -> usize {
        self.budget
    }
}

/// What one decoded cel costs to hold: its samples, four bytes each.
fn bytes_of(buffer: &WorkingBuffer) -> usize {
    std::mem::size_of_val(buffer.as_image().data())
}
