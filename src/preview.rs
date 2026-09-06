//! B-08: the preview path — resolution selection (D-33) and the playback clock (D-32).
//!
//! B-08a joined a project file to a rendered frame. This is the half that sits between that
//! frame and a person looking at it, and it is deliberately the half with no window in it: a
//! resolution choice, a scale applied to a frame plan, and a clock that says which frame should
//! be on screen at a given instant. All three can be checked by a fixture. The window, the
//! transport and the screenshots are the rest of B-08 and use what is here.
//!
//! Two decisions the owner made on 2026-09-05 are implemented here rather than described:
//!
//! - **D-33.** The preview starts at draft resolution. SP-05 measured 145 fps at draft against
//!   24.9 at full, so the default decides whether scrubbing is comfortable or marginal. R-06a
//!   requires "a visible indication when preview quality differs from final export", which is
//!   what makes a draft default safe rather than a lie: [`PreviewQuality::differs_from_export`]
//!   is what a viewer shows, and it is a property of the quality rather than a flag someone has
//!   to remember to set.
//! - **D-32.** Playback holds real time and drops the frames it cannot render in time. The clock
//!   is wall-clock driven, not frame driven: [`Playback::at`] answers "which frame belongs on
//!   screen at this instant", and the frames between that answer and the last one shown were
//!   skipped. [`Playback::skipped`] counts them, because a silently dropped frame is a fidelity
//!   fallback and document 28 forbids those.

use std::path::Path;
use std::time::Duration;

use crate::cache::CelCache;
use crate::compose;
use crate::diagnostics::{Diagnostic, FrameLog};
use crate::model::{Id, Project};
use crate::render::{self, Affine, FramePlan};
use crate::time::FrameRate;
use crate::WorkingBuffer;

/// Draft is a quarter of the composition on each axis.
///
/// Measured, not chosen: SP-05 transported 480×270 against the reference shot's 1920×1080 and
/// recorded 145 fps against 24.9 at full (`spikes/B-01_G0_spike_report.md`, the draft rows).
/// This constant is that measurement's shape. Changing it changes speed and preview sharpness
/// and nothing else, because no exported pixel passes through this module.
///
/// What the divisor does **not** change is the cost of decoding a drawing, which happens at the
/// drawing's own size before anything scales it. `verification/B-08_preview_latency.md` measures
/// both halves on the production path and finds decoding to be about three quarters of a draft
/// frame, so a smaller divisor here would not make preview much faster - it would only make it
/// blurrier. That measurement fired the bounded cache's revisit trigger; see D-37.
pub const DRAFT_DIVISOR: usize = 4;

/// Which resolution the preview is rendering at. D-33: the default is [`Draft`](Self::Draft).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum PreviewQuality {
    /// A quarter of the composition on each axis. Fast enough to scrub, and not final pixels.
    #[default]
    Draft,
    /// The composition's own extent. Identical to what an export of the same frame produces.
    Full,
}

impl PreviewQuality {
    /// What a viewer puts on screen. Document 05 requires the viewer to identify draft
    /// resolution; R-06a requires an indication whenever preview differs from export.
    pub fn label(self) -> &'static str {
        match self {
            PreviewQuality::Draft => "Draft",
            PreviewQuality::Full => "Full",
        }
    }

    /// Whether what is on screen differs from what an export would write.
    ///
    /// This is the whole of R-06a's "visible indication" condition, and it is derived from the
    /// quality rather than tracked alongside it, so the indicator cannot fall out of step with
    /// the thing it indicates.
    pub fn differs_from_export(self) -> bool {
        self != PreviewQuality::Full
    }

    /// The divisor applied to the composition extent. Full is 1, and so changes nothing.
    pub fn divisor(self) -> usize {
        match self {
            PreviewQuality::Draft => DRAFT_DIVISOR,
            PreviewQuality::Full => 1,
        }
    }

    /// The extent a frame of `width` by `height` is previewed at.
    ///
    /// Rounded up, so a composition whose width is not a multiple of the divisor keeps its last
    /// column rather than losing it. At [`Full`](Self::Full) this returns the input unchanged.
    pub fn extent(self, width: usize, height: usize) -> (usize, usize) {
        let d = self.divisor();
        (width.div_ceil(d), height.div_ceil(d))
    }
}

/// Scale a frame plan to a preview extent, leaving [`PreviewQuality::Full`] untouched.
///
/// At `Full` the plan is returned exactly as it came rather than scaled by one. Composing an
/// identity scale would in fact change nothing today - multiplying a coefficient by one is exact,
/// and a mutation pass confirmed no pixel moves either way - so this early return is not what
/// makes a full-resolution preview byte-identical to an export. It is what keeps it so: the
/// moment the draft path gains a half-pixel correction, a filter choice or a rounding step, a
/// full-resolution preview that went through that path would quietly stop matching the export it
/// is meant to be checked against. The cheapest guarantee is not to enter the path at all.
///
/// At `Draft` each layer's source-to-composition transform gains an outer uniform scale, so a
/// preview pixel centre maps back to the composition coordinate it covers — at a quarter scale,
/// preview pixel 0's centre at 0.5 maps to composition coordinate 2.0, which bilinear sampling
/// reads as the region those four source pixels share. No half-pixel correction is needed and
/// none is applied; adding one would shift the preview against the export by an eighth of a
/// composition pixel.
pub fn scale_plan(plan: FramePlan, quality: PreviewQuality) -> FramePlan {
    if quality == PreviewQuality::Full {
        return plan;
    }
    let s = 1.0 / quality.divisor() as f64;
    let (width, height) = quality.extent(plan.width, plan.height);
    FramePlan {
        width,
        height,
        layers: plan
            .layers
            .into_iter()
            .map(|mut layer| {
                layer.transform = layer.transform.then(Affine::scaling(s, s));
                layer
            })
            .collect(),
    }
}

/// One frame for display: document 20's evaluation order, then the preview scale, then a render.
///
/// The result is in the working space, like [`compose::render_frame`]. Step 9, the display
/// transform, still belongs to whoever asked for the frame.
///
/// At [`PreviewQuality::Full`] this composes exactly what an export of the same frame composes,
/// through the same plan and the same renderer, which is what makes B-08's exit condition — a
/// preview matching export — a byte comparison rather than a tolerance.
pub fn preview_frame(
    project: &Project,
    composition_id: &Id,
    frame: i32,
    root: &Path,
    quality: PreviewQuality,
    tile_size: usize,
    log: &mut FrameLog,
) -> Result<WorkingBuffer, Diagnostic> {
    preview_frame_cached(
        project,
        composition_id,
        frame,
        root,
        quality,
        tile_size,
        log,
        &mut CelCache::none(),
    )
}

/// [`preview_frame`], remembering the decoded cels between frames (B-08b, R-06b, D-37).
///
/// This is the only caller in the crate that is handed a cache with a budget. `verification/
/// B-08b_cache_table.md` checks that it changes nothing but the clock: every frame of the
/// reference shot, rendered warm, is byte-identical to the same frame rendered cold.
#[allow(clippy::too_many_arguments)]
pub fn preview_frame_cached(
    project: &Project,
    composition_id: &Id,
    frame: i32,
    root: &Path,
    quality: PreviewQuality,
    tile_size: usize,
    log: &mut FrameLog,
    cache: &mut CelCache,
) -> Result<WorkingBuffer, Diagnostic> {
    let plan = compose::plan_frame_cached(project, composition_id, frame, root, log, cache)?;
    Ok(render::render(&scale_plan(plan, quality), tile_size))
}

/// A work-area playback clock that holds real time and drops what it cannot deliver (D-32).
///
/// The work area is a closed range of composition frames and playback loops within it, which is
/// what R-06a means by "work-area playback" and what document 08 measures over "ten repeated
/// work-area loops".
///
/// Nothing here sleeps, waits or reads a clock. [`at`](Self::at) takes the elapsed time since
/// playback began and answers which frame belongs on screen at that instant; the caller supplies
/// the instant, whether that comes from a real timer or from a fixture. That is what makes a
/// decision about dropped frames checkable by a table rather than only by watching it.
#[derive(Clone, Debug)]
pub struct Playback {
    first: i32,
    last: i32,
    rate: FrameRate,
    shown: u32,
    skipped: u32,
    /// Frames advanced since playback began, counted along the loop rather than modulo it, so
    /// the skip count is right across a loop boundary. `None` until the first [`at`](Self::at).
    position: Option<i64>,
}

/// What [`Playback::at`] answers: the frame to show now, and what was passed over to reach it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Shown {
    /// The composition frame that belongs on screen at the instant asked about.
    pub frame: i32,
    /// How many frames were passed over since the previous answer, never displayed.
    ///
    /// Zero on the first call and whenever playback kept up. One or more means the machine did
    /// not render fast enough and D-32 chose the clock over the drawings.
    pub skipped: u32,
}

impl Playback {
    /// A work area of `[first, last]` inclusive, played at `rate`.
    ///
    /// `last` below `first` is an empty work area and is treated as the single frame `first`,
    /// because a viewer with nothing to show is not a state a caller can do anything useful
    /// with, and one frame is what a work area of one frame means.
    pub fn new(first: i32, last: i32, rate: FrameRate) -> Playback {
        Playback {
            first,
            last: last.max(first),
            rate,
            shown: 0,
            skipped: 0,
            position: None,
        }
    }

    /// The frame at rest, before playback begins and after it stops without having run.
    ///
    /// Assumed rather than decided: the work area's first frame. Registered as D-35 rather than
    /// left implicit, because "which frame is shown at rest" is a viewer question nobody has
    /// answered and this is the answer this build behaves as though it had.
    pub fn at_rest(&self) -> i32 {
        self.first
    }

    /// How many frames the work area contains.
    pub fn length(&self) -> i64 {
        self.last as i64 - self.first as i64 + 1
    }

    /// Which frame belongs on screen `elapsed` after playback began, and what was skipped.
    ///
    /// D-32: the frame is derived from the clock, so a caller that renders slowly gets a later
    /// frame rather than a stretched one. Every frame between the previous answer and this one
    /// was passed over without being drawn and is counted.
    ///
    /// `elapsed` must not go backwards between calls; if it does, the position is held and
    /// nothing is counted as skipped, because time running backwards is the caller's fault and
    /// inventing a skip count from it would put a wrong number in front of the owner.
    pub fn at(&mut self, elapsed: Duration) -> Shown {
        let position = frames_elapsed(elapsed, self.rate);
        let advanced = match self.position {
            None => 0,
            Some(previous) => (position - previous).max(0),
        };
        let position = match self.position {
            Some(previous) if position < previous => previous,
            _ => position,
        };
        self.position = Some(position);
        self.shown += 1;
        // One frame of advance is the frame after the one just shown, which is not a skip. Every
        // further frame of advance is a frame that came and went without being drawn. Zero
        // advance - the first answer, or two answers inside one frame time - skips nothing, and
        // the clamp is what says so rather than counting backwards.
        let skipped = (advanced - 1).clamp(0, u32::MAX as i64) as u32;
        self.skipped += skipped;
        Shown {
            frame: self.first + (position.rem_euclid(self.length())) as i32,
            skipped,
        }
    }

    /// How many frames have been put on screen since playback began.
    pub fn frames_shown(&self) -> u32 {
        self.shown
    }

    /// How many frames were dropped since playback began.
    ///
    /// D-32 requires this to be reported rather than hidden: playback that quietly showed two
    /// thirds of the drawings would be a silent fidelity fallback, which document 28 forbids.
    pub fn skipped(&self) -> u32 {
        self.skipped
    }

    /// The sentence a viewer puts in front of the owner when playback stops.
    ///
    /// Document 28 wants a problem and its next action, not a number to decode. A run that kept
    /// up says so; a run that did not says what it cost and what to do about it.
    pub fn report(&self) -> String {
        if self.skipped == 0 {
            format!(
                "Played {} frames in real time. No frames were dropped.",
                self.shown
            )
        } else {
            format!(
                "Played {} frames in real time and dropped {} to keep the timing true. \
                 Step through the frames to see every drawing, or switch the preview to draft \
                 resolution.",
                self.shown, self.skipped
            )
        }
    }
}

/// Whole frames elapsed at `rate` after `elapsed`, rounded **down**.
///
/// This is not [`FrameRate::frame_at_seconds`] and must not be. That conversion rounds half away
/// from zero, because document 20 makes it the map from a stored time to the frame it names.
/// This one answers a different question: which frame is on screen now. Frame `k` occupies the
/// half-open interval `[k/rate, (k+1)/rate)`, so a third of the way into frame 0 is still frame
/// 0, and rounding would show frame 1 half a frame early for the whole of playback.
///
/// Computed in `i128` from the duration's own nanoseconds, with no floating point, so a rate of
/// 24000/1001 is exact rather than nearly right.
fn frames_elapsed(elapsed: Duration, rate: FrameRate) -> i64 {
    let ns = elapsed.as_nanos() as i128;
    let frames = (ns * rate.numerator() as i128) / (rate.denominator() as i128 * 1_000_000_000i128);
    i64::try_from(frames).unwrap_or(i64::MAX)
}
