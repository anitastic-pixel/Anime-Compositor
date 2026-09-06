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
//! about 33 MB, which is four times the figure `verification/D-37_decode_cost.md` quotes for a cel
//! on disk. Eviction is least-recently-used and changes performance only, which is document 27's
//! requirement and is checked as a byte comparison rather than asserted.
//!
//! **Where it is not.** Export never sees one. [`crate::compose::plan_frame`] and
//! [`crate::export`] use [`CelCache::none`], whose budget is zero and which therefore cannot store
//! anything; only the preview path is handed a real one. A full-resolution preview and an export
//! of the same frame differ in 0 of 8,294,400 samples, and a cache is the obvious way to lose
//! that. ADR-015 makes keeping it out of the export path part of the decision rather than an
//! implementation choice.

use std::path::{Path, PathBuf};
use std::time::SystemTime;

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
/// The number came out of that table rather than out of a guess about it. 128 MB took a draft
/// preview frame from 100.29 ms to 42.55 ms; 512 MB, four times the memory, took it to 42.16 ms.
/// The gap is smaller than the run-to-run noise, and the reason is that playback is sequential: a
/// cel is asked for again within a few frames of first use or not for a long time, so what has to
/// fit is the reuse distance and not the shot. Holding the whole shot would cost about 1.9 GB and
/// this table says it would buy nothing.
pub const DEFAULT_BUDGET_BYTES: usize = 128 * 1024 * 1024;

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
    entries: Vec<(Key, WorkingBuffer)>,
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
    /// same buffer either way: this function has one path that produces pixels and one that copies
    /// pixels it already produced.
    pub fn decoded(
        &mut self,
        path: &Path,
        interpretation: Interpretation,
    ) -> Result<WorkingBuffer, Diagnostic> {
        let key = Key::of(path, interpretation);

        if let Some(key) = &key {
            if let Some(at) = self.entries.iter().position(|(k, _)| k == key) {
                let entry = self.entries.remove(at);
                let buffer = entry.1.clone();
                self.entries.push(entry);
                self.hits += 1;
                return Ok(buffer);
            }
        }

        self.misses += 1;
        let buffer = retag(media::decode_png(path)?, interpretation).into_working();
        if let Some(key) = key {
            self.store(key, buffer.clone());
        }
        Ok(buffer)
    }

    fn store(&mut self, key: Key, buffer: WorkingBuffer) {
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
