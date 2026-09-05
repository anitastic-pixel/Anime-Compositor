//! T-08: write a frame range of a composition to a PNG sequence.
//!
//! R-09: "render a declared inclusive frame range to a PNG sequence with chosen bit depth,
//! naming and alpha policy; report failure and support cancellation between frames." This
//! module is that, and only that. **It writes image sequences and nothing else.** A video file
//! needs an encoder, an encoder is a dependency and a licence, and both belong to the owner of
//! the project rather than to the code that would use them.
//!
//! The rules it has to obey come from four documents and they are worth stating together:
//!
//! - Document 07: an export range is **inclusive at both ends** and converts explicitly to an
//!   internal interval. Exporting 0 through 239 writes 240 files.
//! - Document 07: the default behaviour when a frame has no source drawing is a **blocked**
//!   final export. Any other behaviour is an explicit choice and must appear in the warnings.
//! - Document 21 line 31, and document 08: export converts the linear working RGB to the
//!   declared output encoding and writes straight alpha. **A display transform is never baked
//!   in**, and the viewer's path is not on the way to a file.
//! - Document 28: a write failure reports the completed frames and the failing path;
//!   cancellation preserves the completed-frame list and claims no success; and output produced
//!   while a parked feature was bypassed must **report that fidelity is incomplete**.
//!
//! The pixels are the renderer's, unchanged. [`export_sequence`] calls the same
//! [`crate::compose::render_frame`] the viewer will, and the bytes it writes are exactly
//! [`crate::WorkingBuffer::encode`]'s output, so an exported frame cannot drift from the frame
//! the build says it rendered.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::compose;
use crate::diagnostics::{Diagnostic, DiagnosticId, FrameLog, Severity};
use crate::model::{Id, Project};
use crate::png_out;
use crate::{OutputAlpha, OutputDepth};

/// What to do about a frame whose layer has no drawing to show.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MissingSource {
    /// Document 07's default: refuse the job before writing anything, and say which frames.
    Block,
    /// Document 28's render fallback: the layer contributes nothing and the frame is written
    /// anyway. Never silent - every affected frame is in the report, and the files carry a
    /// `Fidelity` tag saying so.
    RenderTransparent,
}

/// How a job ended. There is no variant that means "finished, with problems hidden".
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ExportStatus {
    /// Every frame in the range was written.
    Completed,
    /// Refused before writing anything, because a frame in the range has no source drawing and
    /// the policy is [`MissingSource::Block`].
    Blocked,
    /// Stopped between frames because the caller asked. The frames already written are kept.
    Cancelled,
    /// A file could not be written. The frames already written are kept and named.
    Failed,
}

/// One export job.
pub struct ExportRequest {
    pub composition: Id,
    /// Inclusive, per document 07. `first_frame == last_frame` exports one frame.
    pub first_frame: i32,
    /// Inclusive.
    pub last_frame: i32,
    pub output_dir: PathBuf,
    /// A file name containing one `%0Nd`, as sequence patterns are spelled everywhere else in
    /// this build: `shot_%04d.png` writes `shot_0000.png` and so on.
    pub naming: String,
    pub depth: OutputDepth,
    pub alpha: OutputAlpha,
    pub tile_size: usize,
    pub missing: MissingSource,
}

/// What a job did. Everything a person needs to know without opening the folder.
#[derive(Clone, Debug)]
pub struct ExportReport {
    pub status: ExportStatus,
    /// How many files the request asked for: `last - first + 1`.
    pub frames_requested: usize,
    /// The files written, in the order they were written.
    pub written: Vec<PathBuf>,
    /// Document 28: output produced while a parked feature was bypassed must say so.
    pub fidelity_incomplete: bool,
    pub diagnostics: Vec<Diagnostic>,
}

impl ExportReport {
    /// The one question the caller actually asks. False for every status but
    /// [`ExportStatus::Completed`], and false while any file is missing, so a job cannot be
    /// called successful by a caller that forgot to look at the count.
    pub fn succeeded(&self) -> bool {
        self.status == ExportStatus::Completed && self.written.len() == self.frames_requested
    }
}

/// Render `request`'s inclusive frame range and write each frame as a PNG.
///
/// `cancel` is read once before each frame, which is what document 03 means by "support
/// cancellation between frames": a frame is never half-written, and a cancelled job's files are
/// the frames that finished.
pub fn export_sequence(
    project: &Project,
    root: &Path,
    request: &ExportRequest,
    cancel: &AtomicBool,
) -> ExportReport {
    let mut report = ExportReport {
        status: ExportStatus::Completed,
        frames_requested: 0,
        written: Vec::new(),
        fidelity_incomplete: false,
        diagnostics: Vec::new(),
    };

    // Document 07: the inclusive range converts explicitly to a count. It is converted once,
    // here, and every later loop counts frames rather than re-deriving the arithmetic.
    if request.last_frame < request.first_frame {
        report.status = ExportStatus::Failed;
        report.diagnostics.push(invalid(format!(
            "The export range ends before it starts: {} to {}.",
            request.first_frame, request.last_frame
        )));
        return report;
    }
    let frames: Vec<i32> = (request.first_frame..=request.last_frame).collect();
    report.frames_requested = frames.len();

    if !request.naming.contains("%0") || !request.naming.contains('d') {
        report.status = ExportStatus::Failed;
        report.diagnostics.push(invalid(format!(
            "The output naming {} contains no frame number, so every frame would be written to \
             one file.",
            request.naming
        )));
        return report;
    }

    // Document 07's default: a missing drawing blocks a final export, and nothing is written
    // before that is known. This plans every frame first, which costs the decode twice over a
    // long range.
    // ponytail: re-planning is the cost of blocking before the first write. A job snapshot that
    // kept the plans would remove it, and belongs with the cache in document 27, which is PARKED.
    if request.missing == MissingSource::Block {
        let mut scan = FrameLog::new(3);
        let mut unresolved: Vec<i32> = Vec::new();
        for &frame in &frames {
            match compose::plan_frame(project, &request.composition, frame, root, &mut scan) {
                Ok(_) => {}
                Err(d) => {
                    report.status = ExportStatus::Failed;
                    report.diagnostics.push(d);
                    return report;
                }
            }
            if scan
                .ids_at(frame)
                .iter()
                .any(|id| is_unresolved_source(*id))
            {
                unresolved.push(frame);
            }
        }
        if !unresolved.is_empty() {
            report.status = ExportStatus::Blocked;
            report.diagnostics.push(
                Diagnostic::new(
                    DiagnosticId::ExportBlockedMissingMedia,
                    Severity::Error,
                    format!(
                        "{} of the {} frames asked for have a drawing that is missing, so nothing \
                         was exported.",
                        unresolved.len(),
                        frames.len()
                    ),
                    format!("Frames {}.", ranges(&unresolved)),
                )
                .with_remediation(
                    "Relink or restore the missing drawings, or export again having chosen to \
                     write the affected frames with that layer left out.",
                ),
            );
            report.diagnostics.extend(scan.finish());
            return report;
        }
        report.diagnostics.extend(scan.finish());
    }

    let mut log = FrameLog::new(3);
    for &frame in &frames {
        // Between frames, not during one: a cancelled job leaves whole files behind.
        if cancel.load(Ordering::SeqCst) {
            report.status = ExportStatus::Cancelled;
            report.diagnostics.push(Diagnostic::new(
                DiagnosticId::ExportCancelled,
                Severity::Info,
                format!(
                    "Export stopped at your request after {} of {} frames.",
                    report.written.len(),
                    frames.len()
                ),
                format!(
                    "Frame {frame} had not been written when the request arrived. The frames \
                     already written are complete files and were left in place."
                ),
            ));
            report.diagnostics.extend(log.finish());
            return report;
        }

        let buffer = match compose::render_frame(
            project,
            &request.composition,
            frame,
            root,
            request.tile_size,
            &mut log,
        ) {
            Ok(buffer) => buffer,
            Err(d) => {
                report.status = ExportStatus::Failed;
                report.diagnostics.push(d);
                report.diagnostics.extend(log.finish());
                return report;
            }
        };
        let bypassed = log
            .ids_at(frame)
            .contains(&DiagnosticId::ProjectFeatureUnsupported);
        report.fidelity_incomplete |= bypassed;

        let path = request.output_dir.join(expand(&request.naming, frame));
        let mut tags = vec![
            ("Software", "anime_compositor export (R-09)".to_string()),
            (
                "ColorSpace",
                match request.depth {
                    OutputDepth::Eight => "sRGB IEC 61966-2-1, 8 bits per channel".to_string(),
                    OutputDepth::Sixteen => "sRGB IEC 61966-2-1, 16 bits per channel".to_string(),
                },
            ),
            (
                "AlphaMode",
                match request.alpha {
                    OutputAlpha::Straight => "Straight".to_string(),
                    OutputAlpha::Premultiplied => "Premultiplied".to_string(),
                },
            ),
            (
                "WorkingSpace",
                "converted from linear light, premultiplied, float32".to_string(),
            ),
            ("Frame", frame.to_string()),
        ];
        if bypassed {
            // Document 28: "exported output must report that fidelity is incomplete".
            tags.push((
                "Fidelity",
                "incomplete: a layer carrying a parked feature was drawn without it".to_string(),
            ));
        }
        let samples = buffer.encode(request.depth, request.alpha);
        if let Err(e) = png_out::write_rgba(
            &path,
            buffer.width(),
            buffer.height(),
            request.depth,
            &tags,
            &samples,
        ) {
            report.status = ExportStatus::Failed;
            report.diagnostics.push(
                Diagnostic::new(
                    DiagnosticId::ExportWriteFailed,
                    Severity::Error,
                    format!("Frame {frame} could not be written to {}.", path.display()),
                    format!(
                        "{e}. {} of {} frames had been written when this happened, and they were \
                         left in place.",
                        report.written.len(),
                        frames.len()
                    ),
                )
                .with_remediation(
                    "Check that the folder exists, is writable and has room, then export the \
                     frames that are missing.",
                ),
            );
            report.diagnostics.extend(log.finish());
            return report;
        }
        report.written.push(path);
    }

    report.diagnostics.extend(log.finish());
    report
}

/// True for a diagnostic that means "this frame has no drawing to show", which is what
/// document 07's blocked export is about. A parked feature is deliberately not one of these:
/// document 28 requires it to be reported, not to stop the job.
fn is_unresolved_source(id: DiagnosticId) -> bool {
    matches!(
        id,
        DiagnosticId::MediaSequenceGap
            | DiagnosticId::MediaMissing
            | DiagnosticId::MediaDecodeFailed
            | DiagnosticId::MediaUnsupportedFormat
    )
}

/// Substitute a frame number into a `%0Nd` naming pattern.
///
/// A negative composition frame keeps its sign in front of the padded digits, so a composition
/// starting at -12 exports `shot_-0012.png` through `shot_0011.png` and the file names still
/// sort into the order the frames play in for the frames that share a sign. D-29.
fn expand(pattern: &str, frame: i32) -> String {
    let Some(start) = pattern.find("%0") else {
        return pattern.to_string();
    };
    let Some(end) = pattern[start..].find('d').map(|i| start + i) else {
        return pattern.to_string();
    };
    let width: usize = pattern[start + 2..end].parse().unwrap_or(0);
    let number = if frame < 0 {
        format!("-{:0width$}", frame.unsigned_abs(), width = width)
    } else {
        format!("{frame:0width$}")
    };
    format!("{}{}{}", &pattern[..start], number, &pattern[end + 1..])
}

/// "14 to 15, 20" - the same shape [`FrameLog`] uses, so a reader sees one spelling of a frame
/// range in the whole build.
fn ranges(frames: &[i32]) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < frames.len() {
        let start = frames[i];
        let mut end = start;
        while i + 1 < frames.len() && frames[i + 1] == end + 1 {
            i += 1;
            end = frames[i];
        }
        out.push(if start == end {
            start.to_string()
        } else {
            format!("{start} to {end}")
        });
        i += 1;
    }
    out.join(", ")
}

fn invalid(message: String) -> Diagnostic {
    Diagnostic::new(
        DiagnosticId::CommandInvalidValue,
        Severity::Error,
        message,
        "Nothing was written. The export was refused before it started.".to_string(),
    )
    .with_remediation("Correct the request and export again.")
}
