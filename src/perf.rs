//! P-01: per-stage timers, so that "where does the frame go" is a table and not an opinion.
//!
//! `verification/T-06_declared_fixture.md` measures 264.17 ms a frame against the 41.667 ms a
//! 24 fps clock allows, and `Markdown/32_Performance_Architecture_Investigation.md` section 1.2
//! could not account for about half of that. Neither research document knows where the time
//! goes because nobody had measured it. This module is the measurement, and it is deliberately
//! the smallest thing that can produce one: an atomic nanosecond counter and a call count per
//! named stage, a switch that is off, and nothing else.
//!
//! Three properties are structural rather than remembered.
//!
//! **Off by default.** [`enable`] is called by the harness that writes P-01's artifact and by
//! nothing else. While it is off, [`time`] is one relaxed atomic load and a call, so no shipped
//! render pays for the timers it is not using. That is ADR-012's rule for trace mode applied to
//! its stopwatch: a diagnostic is never on in a build the owner is looking at unless asked for.
//!
//! **The stages are disjoint.** No stage below is inside another, so a table of them can be
//! summed and subtracted from a frame time to leave a residual. A nested pair would double-count
//! and the residual would go negative, which is a bug that reads as a result. `Decode` is
//! therefore split at its real seams — the read, the byte-to-float pass, and the transfer
//! function with the premultiply — rather than wrapped whole.
//!
//! **It measures, it does not judge.** There is no budget here, no threshold, no assertion. The
//! numbers go into `verification/P-01_frame_trace.md` and the owner reads them.
//!
//! `TileLoop` is wall-clock across the rayon fan-out rather than the sum of the threads' own
//! time, because the frame's cost is when the last tile finishes and not how many core-seconds
//! were spent. Every other stage runs on the calling thread.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Instant;

/// A stage of one preview frame that this build spends measurable time in.
///
/// The order is the order they run in, which is the order the artifact prints them, so that a
/// reader walks the frame rather than a sorted list.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stage {
    /// The wait for the viewer mutex in the window's `serve`, before any work begins. Zero in a
    /// headless harness, which has no second thread to wait for; P-04 is where it is measured.
    LockWait,
    /// The parallel decode of every cel this frame is about to ask for, wall-clock from
    /// fan-out to join (P-03(b)). The three stages below are what it does; while a cel is being
    /// decoded inside this fan-out they record nothing, so that this stage and they stay
    /// disjoint and the table can still be summed. A cel decoded outside it — by an export, by
    /// a caller with no cache, or by a request this could not see coming — reports in them as
    /// before and nothing in this one.
    Prewarm,
    /// Opening a cel's file and reading the compressed bytes back out as 8-bit RGBA.
    FileRead,
    /// Those bytes divided by 255 into f32, which is `ImageBuffer::from_srgb8_straight`.
    Dequantise,
    /// The sRGB transfer function and the premultiply, which is `ImageBuffer::into_working`.
    ToLinear,
    /// A cache hit: the scan for the key, and the copy of the buffer handed back.
    CacheHit,
    /// A cache miss admitting the decoded cel: the second copy, and any eviction it forces.
    CacheStore,
    /// The polygon mask rasterised into the layer's own pixels.
    Mask,
    /// The copy `Arc::make_mut` takes so the effect stack has a buffer of its own to write in.
    /// Separate from the effects themselves because it is paid once a layer however long the
    /// stack is, and P-11 is ranking the effects against each other.
    EffectCopy,
    /// The exposure effect, whole-layer, per ADR-017.
    EffectExposure,
    /// The tint effect, whole-layer, per ADR-017.
    EffectTint,
    /// The gaussian blur, whole-layer, per ADR-017. Split out from the other two by P-11, whose
    /// entry says the first thing it does is find out whether the blur really is the expensive
    /// one rather than assuming it from what the three effects do.
    EffectBlur,
    /// The lookup and admission of an evaluated effect result (P-11).
    EffectCache,
    /// The tiled sample-and-blend fan-out, wall-clock from fan-out to join.
    TileLoop,
    /// Copying the finished tiles into the frame buffer.
    FrameAssembly,
    /// `WorkingBuffer::to_srgb8_straight`: unpremultiply, encode, quantise, for the page.
    Encode,
}

impl Stage {
    pub const ALL: [Stage; 16] = [
        Stage::LockWait,
        Stage::Prewarm,
        Stage::FileRead,
        Stage::Dequantise,
        Stage::ToLinear,
        Stage::CacheHit,
        Stage::CacheStore,
        Stage::Mask,
        Stage::EffectCopy,
        Stage::EffectExposure,
        Stage::EffectTint,
        Stage::EffectBlur,
        Stage::EffectCache,
        Stage::TileLoop,
        Stage::FrameAssembly,
        Stage::Encode,
    ];

    /// The name the artifact prints. Written for the owner, not for a log parser.
    pub fn label(self) -> &'static str {
        match self {
            Stage::LockWait => "wait for the viewer lock",
            Stage::Prewarm => "decode this frame's cels in parallel",
            Stage::FileRead => "open and read the cel file",
            Stage::Dequantise => "bytes to float",
            Stage::ToLinear => "transfer function and premultiply",
            Stage::CacheHit => "cache lookup and its copy",
            Stage::CacheStore => "cache admit and its copy",
            Stage::Mask => "layer mask",
            Stage::EffectCopy => "effect stack: the copy it writes into",
            Stage::EffectExposure => "effect: exposure",
            Stage::EffectTint => "effect: tint",
            Stage::EffectBlur => "effect: gaussian blur",
            Stage::EffectCache => "effect result cache: lookup and admit",
            Stage::TileLoop => "tile loop: sample and blend",
            Stage::FrameAssembly => "assemble the frame from the tiles",
            Stage::Encode => "encode for the page",
        }
    }

    fn index(self) -> usize {
        match self {
            Stage::LockWait => 0,
            Stage::Prewarm => 1,
            Stage::FileRead => 2,
            Stage::Dequantise => 3,
            Stage::ToLinear => 4,
            Stage::CacheHit => 5,
            Stage::CacheStore => 6,
            Stage::Mask => 7,
            Stage::EffectCopy => 8,
            Stage::EffectExposure => 9,
            Stage::EffectTint => 10,
            Stage::EffectBlur => 11,
            Stage::EffectCache => 12,
            Stage::TileLoop => 13,
            Stage::FrameAssembly => 14,
            Stage::Encode => 15,
        }
    }
}

const N: usize = Stage::ALL.len();

static ON: AtomicBool = AtomicBool::new(false);
#[allow(clippy::declare_interior_mutable_const)]
const ZERO: AtomicU64 = AtomicU64::new(0);
static NANOS: [AtomicU64; N] = [ZERO; N];
static CALLS: [AtomicU64; N] = [ZERO; N];

thread_local! {
    /// Set only by [`untimed`]. `Cell<bool>` rather than an atomic because it is per thread by
    /// definition: one worker suppressing its own timers must not suppress another's.
    static SUPPRESSED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Start recording. Nothing in `src/` or `app/` calls this; the P-01 harness does.
pub fn enable() {
    ON.store(true, Ordering::Relaxed);
}

/// Stop recording. Counters keep whatever they hold; [`reset`] is what clears them.
pub fn disable() {
    ON.store(false, Ordering::Relaxed);
}

pub fn is_enabled() -> bool {
    ON.load(Ordering::Relaxed)
}

/// Zero every counter. Called between measured frames, so a row is one frame and not a run.
pub fn reset() {
    for i in 0..N {
        NANOS[i].store(0, Ordering::Relaxed);
        CALLS[i].store(0, Ordering::Relaxed);
    }
}

/// Run `f`, and if recording is on, add how long it took to `stage`.
///
/// The `Instant::now()` pair costs tens of nanoseconds and every stage it wraps is measured in
/// microseconds or milliseconds, so the timer is inside the noise of the thing it times. When
/// recording is off it is not taken at all.
pub fn time<T>(stage: Stage, f: impl FnOnce() -> T) -> T {
    if !is_enabled() || SUPPRESSED.with(|s| s.get()) {
        return f();
    }
    let at = Instant::now();
    let out = f();
    record(stage, at.elapsed().as_nanos() as u64);
    out
}

/// Run `f` with every [`time`] call on this thread recording nothing (P-03(b)).
///
/// There is exactly one caller: the parallel decode in [`crate::cache::CelCache::prewarm`], which
/// wraps each cel it decodes. Without this, the read, the byte-to-float pass and the transfer
/// function would each add their *core* time to a stage while the frame paid only the wall-clock
/// of the widest thread, the stages would stop being disjoint, and the residual this module
/// promises would go negative — "a bug that reads as a result", in the words at the top of this
/// file. [`Stage::Prewarm`] is what those cels report as instead, once, around the whole fan-out.
///
/// It is a thread-local and set inside the fan-out rather than around it, because a rayon closure
/// runs on whichever worker steals it and the calling thread steals work too.
pub fn untimed<T>(f: impl FnOnce() -> T) -> T {
    if !is_enabled() {
        return f();
    }
    SUPPRESSED.with(|s| s.set(true));
    let out = f();
    SUPPRESSED.with(|s| s.set(false));
    out
}

/// Add an already-measured duration to a stage, for a caller that cannot wrap a closure around
/// it — a lock acquisition whose guard has to outlive the timing.
pub fn record(stage: Stage, nanos: u64) {
    if !is_enabled() || SUPPRESSED.with(|s| s.get()) {
        return;
    }
    NANOS[stage.index()].fetch_add(nanos, Ordering::Relaxed);
    CALLS[stage.index()].fetch_add(1, Ordering::Relaxed);
}

/// `(nanoseconds, calls)` for one stage since the last [`reset`].
pub fn read(stage: Stage) -> (u64, u64) {
    (
        NANOS[stage.index()].load(Ordering::Relaxed),
        CALLS[stage.index()].load(Ordering::Relaxed),
    )
}

/// Every stage in run order, with its nanoseconds and call count.
pub fn snapshot() -> Vec<(Stage, u64, u64)> {
    Stage::ALL
        .iter()
        .map(|&s| {
            let (nanos, calls) = read(s);
            (s, nanos, calls)
        })
        .collect()
}

/// The sum of every stage, in nanoseconds. The frame time minus this is P-01's residual.
pub fn total_nanos() -> u64 {
    Stage::ALL.iter().map(|&s| read(s).0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Off means nothing is recorded, on means it is, and reset means it is not any more. The
    /// three properties the artifact depends on, in the order a reader would doubt them.
    #[test]
    fn records_only_while_enabled() {
        // Serialised by holding the whole check in one test: the counters are process-global.
        reset();
        disable();
        time(Stage::Mask, || std::hint::black_box(1));
        assert_eq!(read(Stage::Mask), (0, 0), "recorded while switched off");

        enable();
        // Long enough to be longer than the clock's own step. `black_box(1)` was not: Windows
        // ticks its performance counter in the hundreds of nanoseconds and an integer that is
        // handed straight back takes a few, so the assertion below failed on about one run in a
        // dozen -- a false alarm about the timer rather than a fault in it.
        time(Stage::Mask, || {
            let at = std::time::Instant::now();
            while at.elapsed().as_nanos() == 0 {
                std::hint::spin_loop();
            }
        });
        disable();
        let (nanos, calls) = read(Stage::Mask);
        assert_eq!(calls, 1, "one timed call counted {calls} times");
        assert!(nanos > 0, "a timed call took no measurable time");

        reset();
        assert_eq!(read(Stage::Mask), (0, 0), "reset left {nanos} ns behind");
    }
}
