//! Rational time and explicit exposure spans (B-04, requirement R-02).
//!
//! Document 15 calls R-02 "the central requirement of the product", and the reason is in document
//! 20's line about conversions: "When importing a sequence, file number is not assumed to equal
//! composition frame." Everything here exists to keep three numbering systems apart — composition
//! frames, layer-local frames, and drawing numbers — that every naive implementation collapses
//! into one.
//!
//! Time is integer frames plus a rational frame rate. Seconds are derived and never stored, per
//! document 20: "Floating-point seconds are presentation data, not project identity." There is no
//! `f64` anywhere in this module's frame arithmetic.

use std::fmt;
use std::path::PathBuf;

use crate::diagnostics::{Diagnostic, DiagnosticId, Severity};
use crate::media::SequenceAsset;

#[derive(Debug, PartialEq, Eq)]
pub enum TimeError {
    /// A frame rate with a zero term is not a rate.
    DegenerateFrameRate { numerator: u32, denominator: u32 },
    /// `end_frame_exclusive <= start_frame`. A span that covers no frame cannot be evaluated.
    EmptySpan {
        start_frame: i32,
        end_frame_exclusive: i32,
    },
    /// Document 20 requires "the unique ExposureSpan" covering a frame. Two spans covering one
    /// frame, or spans out of order, make that phrase meaningless, so both are rejected at
    /// construction rather than resolved by a first-match rule nobody wrote down.
    SpansNotDisjoint { previous_end: i32, next_start: i32 },
}

impl fmt::Display for TimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeError::DegenerateFrameRate {
                numerator,
                denominator,
            } => {
                write!(f, "frame rate {numerator}/{denominator} has a zero term")
            }
            TimeError::EmptySpan {
                start_frame,
                end_frame_exclusive,
            } => write!(
                f,
                "exposure span [{start_frame}, {end_frame_exclusive}) covers no frame"
            ),
            TimeError::SpansNotDisjoint {
                previous_end,
                next_start,
            } => write!(
                f,
                "exposure spans overlap or are out of order: a span ends at {previous_end} \
                 and the next starts at {next_start}"
            ),
        }
    }
}

/// A reduced rational frame rate in frames per second.
///
/// Document 20: "For 24000/1001 and similar rates, do not store rounded decimal rates such as
/// 23.976 as authority." The decimal only ever appears in [`FrameRate::label`], which nothing
/// reads back.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FrameRate {
    numerator: u32,
    denominator: u32,
}

impl FrameRate {
    pub fn new(numerator: u32, denominator: u32) -> Result<Self, TimeError> {
        if numerator == 0 || denominator == 0 {
            return Err(TimeError::DegenerateFrameRate {
                numerator,
                denominator,
            });
        }
        let g = gcd(numerator as u64, denominator as u64) as u32;
        Ok(FrameRate {
            numerator: numerator / g,
            denominator: denominator / g,
        })
    }

    pub fn numerator(&self) -> u32 {
        self.numerator
    }

    pub fn denominator(&self) -> u32 {
        self.denominator
    }

    /// Exact seconds at a composition frame, reduced: `frame * denominator / numerator`.
    ///
    /// Returned as a rational because that is what it is. Frame 1 at 24000/1001 is 1001/24000
    /// seconds, and no `f64` holds that exactly.
    pub fn seconds_at(&self, frame: i32) -> (i64, i64) {
        reduce(
            frame as i64 * self.denominator as i64,
            self.numerator as i64,
        )
    }

    /// The frame nearest a time in seconds, given as an exact rational.
    ///
    /// Document 20: round half away from zero. Computed in `i128` so a legitimate time cannot
    /// overflow its way into the wrong frame, and with no floating point at all, because this
    /// conversion decides frame identity.
    pub fn frame_at_seconds(&self, numerator: i64, denominator: i64) -> Option<i32> {
        if denominator == 0 {
            return None;
        }
        // frame = seconds * rate = (sn * rn) / (sd * rd), rounded half away from zero.
        let (mut n, mut d) = (
            numerator as i128 * self.numerator as i128,
            denominator as i128 * self.denominator as i128,
        );
        if d < 0 {
            n = -n;
            d = -d;
        }
        let rounded = if n >= 0 {
            (2 * n + d) / (2 * d)
        } else {
            -((-2 * n + d) / (2 * d))
        };
        i32::try_from(rounded).ok()
    }

    /// The conventional decimal label, for display only. Never stored, never read back.
    pub fn label(&self) -> String {
        if self.denominator == 1 {
            self.numerator.to_string()
        } else {
            format!("{:.3}", self.numerator as f64 / self.denominator as f64)
        }
    }
}

impl fmt::Display for FrameRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a.max(1)
    } else {
        gcd(b, a % b)
    }
}

fn reduce(n: i64, d: i64) -> (i64, i64) {
    let g = gcd(n.unsigned_abs(), d.unsigned_abs()) as i64;
    let (n, d) = (n / g, d / g);
    if d < 0 {
        (-n, -d)
    } else {
        (n, d)
    }
}

/// A composition's frame interval and rate.
///
/// Document 20: the valid interval is half-open, `[start_frame, start_frame + duration_frames)`.
/// `start_frame` is signed and genuinely may be negative; fixture FX-TIME-004 starts at -12.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Composition {
    pub start_frame: i32,
    pub duration_frames: u32,
    pub frame_rate: FrameRate,
}

impl Composition {
    /// The last exportable frame: `start_frame + duration_frames - 1`.
    pub fn last_frame(&self) -> i32 {
        self.start_frame + self.duration_frames as i32 - 1
    }

    pub fn contains(&self, frame: i32) -> bool {
        frame >= self.start_frame && frame <= self.last_frame()
    }

    pub fn frames(&self) -> impl Iterator<Item = i32> + use<> {
        self.start_frame..=self.last_frame()
    }

    /// Convert a range the user typed, which is inclusive at both ends, to the internal interval.
    ///
    /// Document 20: "UI inclusive ranges convert to this internal half-open convention
    /// immediately." Doing it at the boundary is what stops an off-by-one from reaching the
    /// exporter, where it becomes 239 files instead of 240.
    pub fn from_inclusive_ui_range(
        first: i32,
        last: i32,
        frame_rate: FrameRate,
    ) -> Option<Composition> {
        if last < first {
            return None;
        }
        Some(Composition {
            start_frame: first,
            duration_frames: (last - first + 1) as u32,
            frame_rate,
        })
    }
}

/// One held drawing: `[start_frame, end_frame_exclusive)` in layer-local frames.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ExposureSpan {
    pub start_frame: i32,
    pub end_frame_exclusive: i32,
    pub drawing_number: u32,
}

impl ExposureSpan {
    pub fn len(&self) -> u32 {
        (self.end_frame_exclusive - self.start_frame).max(0) as u32
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// An ordered, disjoint set of exposure spans.
///
/// Drawing numbers are under no ordering constraint whatsoever. The reference shot's layer 4 runs
/// 12, 13, 14, 11, 16, 17 across consecutive exposures — a re-exposure of an earlier drawing,
/// which real cel work does constantly. Only the *frames* are ordered.
#[derive(Clone, Debug, Default)]
pub struct ExposureMap {
    spans: Vec<ExposureSpan>,
}

impl ExposureMap {
    pub fn new(spans: Vec<ExposureSpan>) -> Result<Self, TimeError> {
        let mut previous_end: Option<i32> = None;
        for span in &spans {
            if span.end_frame_exclusive <= span.start_frame {
                return Err(TimeError::EmptySpan {
                    start_frame: span.start_frame,
                    end_frame_exclusive: span.end_frame_exclusive,
                });
            }
            if let Some(end) = previous_end {
                if span.start_frame < end {
                    return Err(TimeError::SpansNotDisjoint {
                        previous_end: end,
                        next_start: span.start_frame,
                    });
                }
            }
            previous_end = Some(span.end_frame_exclusive);
        }
        Ok(ExposureMap { spans })
    }

    /// Every drawing held for `count` frames in turn, starting at local frame 0.
    ///
    /// The shape almost every cel layer actually has: "on 2s" is this with `count` 2.
    pub fn on_twos_style(drawings: &[u32], count: u32) -> Result<Self, TimeError> {
        Self::from_lengths(&drawings.iter().map(|&d| (d, count)).collect::<Vec<_>>())
    }

    /// Consecutive spans from `(drawing, length)` pairs, starting at local frame 0.
    pub fn from_lengths(exposures: &[(u32, u32)]) -> Result<Self, TimeError> {
        let mut frame = 0;
        let mut spans = Vec::with_capacity(exposures.len());
        for &(drawing_number, length) in exposures {
            spans.push(ExposureSpan {
                start_frame: frame,
                end_frame_exclusive: frame + length as i32,
                drawing_number,
            });
            frame += length as i32;
        }
        ExposureMap::new(spans)
    }

    pub fn spans(&self) -> &[ExposureSpan] {
        &self.spans
    }

    /// The drawing exposed at a layer-local frame, or `None` if no span covers it.
    ///
    /// Document 20: "If no span covers the frame, the layer source is transparent for that
    /// frame." That is a different outcome from a covered frame whose drawing is missing from
    /// disk, which is a diagnostic. See [`resolve`].
    pub fn drawing_at(&self, local_frame: i32) -> Option<u32> {
        let i = self
            .spans
            .partition_point(|s| s.end_frame_exclusive <= local_frame);
        self.spans
            .get(i)
            .filter(|s| s.start_frame <= local_frame)
            .map(|s| s.drawing_number)
    }

    /// Total frames covered, which is not the last end frame when there are holes between spans.
    pub fn exposed_frame_count(&self) -> u32 {
        self.spans.iter().map(ExposureSpan::len).sum()
    }
}

/// Where a layer sits in composition time.
///
/// Document 20: "Moving a layer changes `in_frame/out_frame`; trimming and changing source offset
/// are distinct commands." They are separate fields here for that reason, not merged into one
/// offset that would make the two commands indistinguishable after the fact.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct LayerTiming {
    pub in_frame: i32,
    pub out_frame: i32,
    pub source_offset_frames: i32,
}

impl LayerTiming {
    /// `local_frame = composition_frame - in_frame + source_offset_frames`, or `None` outside the
    /// layer's active half-open interval `[in_frame, out_frame)`.
    ///
    /// Document 20: "Frames outside the active interval produce transparent output and do not
    /// request media." The `None` is what stops the request, so callers must not fall back to
    /// clamping into range.
    pub fn local_frame(&self, composition_frame: i32) -> Option<i32> {
        if composition_frame < self.in_frame || composition_frame >= self.out_frame {
            return None;
        }
        Some(composition_frame - self.in_frame + self.source_offset_frames)
    }
}

/// What a layer's source is at one composition frame.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SourceAt {
    /// The layer is inactive, or no span covers this frame. Renders transparent, reads no file.
    Transparent,
    Drawing {
        number: u32,
        path: PathBuf,
    },
}

/// Steps 4 and 5 of document 20's evaluation order: derive the layer-local frame, then resolve
/// the drawing.
///
/// The gap is the whole point. Document 20: "Sequence gaps are not collapsed. If drawing 1002 is
/// referenced but absent, evaluation returns a missing-source diagnostic for 1002 rather than
/// substituting 1001 or 1003." An exposed-but-absent drawing is an `Err`, not `Transparent`,
/// because the two mean different things to the person watching: one is an empty frame they
/// authored, the other is a file that should be there and is not.
pub fn resolve(
    timing: &LayerTiming,
    exposures: &ExposureMap,
    asset: &SequenceAsset,
    composition_frame: i32,
) -> Result<SourceAt, Diagnostic> {
    resolve_in(
        timing,
        exposures,
        asset.frames(),
        asset.pattern(),
        composition_frame,
    )
}

/// [`resolve`] against a frame list that is not a [`SequenceAsset`].
///
/// B-08a assembles a frame from a saved [`crate::model::Project`], whose asset record holds the
/// same frame list as strings relative to the project. That is the second caller, so the lookup
/// moved here rather than being written twice: two spellings of document 20's gap rule would be
/// two places for it to drift.
pub fn resolve_in(
    timing: &LayerTiming,
    exposures: &ExposureMap,
    frames: &std::collections::BTreeMap<u32, PathBuf>,
    pattern: &str,
    composition_frame: i32,
) -> Result<SourceAt, Diagnostic> {
    let Some(local) = timing.local_frame(composition_frame) else {
        return Ok(SourceAt::Transparent);
    };
    let Some(number) = exposures.drawing_at(local) else {
        return Ok(SourceAt::Transparent);
    };
    match frames.get(&number) {
        Some(path) => Ok(SourceAt::Drawing {
            number,
            path: path.clone(),
        }),
        None => Err(Diagnostic::new(
            DiagnosticId::MediaSequenceGap,
            Severity::Warning,
            format!(
                "Frame {composition_frame} exposes drawing {number} of {pattern}, which is missing."
            ),
            format!(
                "Layer-local frame {local} maps to drawing {number}. \
                 No file in the sequence carries that number, so the frame renders transparent. \
                 No neighbouring drawing is substituted."
            ),
        )
        .with_remediation(
            "Add the missing file and relink the sequence, or change the exposure to a drawing \
             that exists.",
        )),
    }
}
