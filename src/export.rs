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

use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;

use rayon::prelude::*;

use crate::compose;
use crate::diagnostics::{Diagnostic, DiagnosticId, FrameLog, Severity};
use crate::exr_io::{self, ExrSamples};
use crate::film_out::Film;
use crate::model::{Id, Project};
use crate::png_out;
use crate::{OutputAlpha, OutputDepth, WorkingBuffer};

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

/// Which kind of file each frame becomes. D-62 added EXR beside PNG.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OutputFormat {
    /// `depth` and `alpha` apply.
    Png,
    /// D-62's file, with half or float samples. `depth` and `alpha` do not apply: an EXR holds
    /// the working buffer as it is, premultiplied.
    Exr(ExrSamples),
    /// D-72: one GIF holding every frame. A preview format: 256 colours a frame, alpha on or
    /// off. `naming` is the file's name as it stands and carries no frame number.
    Gif,
    /// D-72: one animated PNG holding every frame, from the samples a PNG frame would hold.
    /// `depth` and `alpha` apply. `naming` is the file's name as it stands.
    Apng,
    /// D-72: one MP4 holding every frame as H.264, over black, with no sound. Windows only
    /// (D-30). `naming` is the file's name as it stands.
    Mp4,
}

impl OutputFormat {
    /// True when the whole job is one file rather than a file a frame.
    pub fn is_one_file(self) -> bool {
        matches!(self, OutputFormat::Gif | OutputFormat::Apng | OutputFormat::Mp4)
    }
}

/// How a job ended. There is no variant that means "finished, with problems hidden".
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ExportStatus {
    /// Every frame in the range was written.
    Completed,
    /// Refused before writing anything, because a frame in the range has no source drawing and
    /// the policy is [`MissingSource::Block`], or because an expression fails on one (D-59).
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
    pub format: OutputFormat,
    pub choices: ExportChoices,
}

/// What a job did. Everything a person needs to know without opening the folder.
/// D-73's choices. Each belongs to one format and is ignored by the others.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ExportChoices {
    pub mp4_quality: crate::mp4_out::Mp4Quality,
    /// Floyd and Steinberg's dithering in a GIF. Off is what B-21c wrote.
    pub gif_dither: bool,
    /// D-231: how many frames are drawn at once. Nought, the default, lets the export choose
    /// from the machine and the picture's size. Every number writes the same bytes.
    pub frames_at_once: usize,
}

#[derive(Clone, Debug)]
pub struct ExportReport {
    pub status: ExportStatus,
    /// How many files the request asked for: `last - first + 1`.
    pub frames_requested: usize,
    /// How many files that is: one a frame, or one in all for a GIF or an animated PNG (D-72).
    pub files_expected: usize,
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
        self.status == ExportStatus::Completed && self.written.len() == self.files_expected
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
    export_sequence_counting(project, root, request, cancel, &AtomicUsize::new(0))
}

/// [`export_sequence`], adding one to `done` as each frame reaches the disk, so a window can
/// show how far a running export has got.
pub fn export_sequence_counting(
    project: &Project,
    root: &Path,
    request: &ExportRequest,
    cancel: &AtomicBool,
    done: &AtomicUsize,
) -> ExportReport {
    export_sequence_watched(project, root, request, cancel, done, &Watch::default())
}

/// D-252: what a window watching a running export sees of it, and its Pause.
#[derive(Default)]
pub struct Watch {
    /// While set, the export waits before writing its next frame, and goes on when it is cleared.
    /// Cancel still stops a paused export.
    pub pause: AtomicBool,
    /// The last [`WATCHED`] frames written, oldest first, each [`small`] at most 480 across.
    pub written: Mutex<VecDeque<(i32, usize, usize, Vec<u8>)>>,
    /// The frames written so far whose drawing was missing, in order.
    pub missing: Mutex<Vec<i32>>,
}

/// How many written frames a [`Watch`] keeps.
pub const WATCHED: usize = 5;

/// A picture each side a whole number of times smaller, so it is at most `most` pixels across.
/// Every block of pixels is averaged in linear light, premultiplied, then made 8-bit sRGB with
/// straight alpha as frames are. D-260's thumbnails and D-252's watched frames are made with it.
pub fn small(full: &WorkingBuffer, most: usize) -> (usize, usize, Vec<u8>) {
    let by = full.width().max(full.height()).div_ceil(most).max(1);
    let (w, h) = ((full.width() / by).max(1), (full.height() / by).max(1));
    let mut small = WorkingBuffer::transparent(w, h);
    let (source, stride) = (full.data(), full.width());
    for (i, out) in small.data_mut().chunks_mut(4).enumerate() {
        let (x, y) = (i % w, i / w);
        for c in 0..4 {
            let mut sum = 0.0f32;
            for dy in 0..by.min(full.height()) {
                for dx in 0..by.min(full.width()) {
                    sum += source[((y * by + dy) * stride + x * by + dx) * 4 + c];
                }
            }
            out[c] = sum / (by.min(full.height()) * by.min(full.width())) as f32;
        }
    }
    (w, h, small.to_srgb8_straight())
}

/// [`export_sequence_counting`], telling `watch` about each frame it writes and waiting while
/// `watch.pause` is set. Pausing changes when frames are written, never what is written.
pub fn export_sequence_watched(
    project: &Project,
    root: &Path,
    request: &ExportRequest,
    cancel: &AtomicBool,
    done: &AtomicUsize,
    watch: &Watch,
) -> ExportReport {
    let mut report = ExportReport {
        status: ExportStatus::Completed,
        frames_requested: 0,
        files_expected: 0,
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
    let one_file = request.format.is_one_file();
    report.files_expected = if one_file { 1 } else { frames.len() };
    let film_path = request.output_dir.join(&request.naming);
    let mut film: Option<Film> = None;

    if request.format == OutputFormat::Apng {
        let refusal = project
            .composition(&request.composition)
            .and_then(|c| Film::apng_refusal(c.frame_rate));
        if let Some(why) = refusal {
            report.status = ExportStatus::Failed;
            report.diagnostics.push(invalid(why));
            return report;
        }
    }

    if !one_file && (!request.naming.contains("%0") || !request.naming.contains('d')) {
        report.status = ExportStatus::Failed;
        report.diagnostics.push(invalid(format!(
            "The output naming {} contains no frame number, so every frame would be written to \
             one file.",
            request.naming
        )));
        return report;
    }

    // D-59: an expression that fails on any frame of the range refuses the export, whatever the
    // missing-drawing policy, and before anything is written.
    if let Some(comp) = project.composition(&request.composition) {
        let mut refused = false;
        for (target, prop) in crate::expr::live_properties(comp) {
            let failing: Vec<(i32, crate::expr::ExprError)> = frames
                .iter()
                .filter_map(|&f| {
                    crate::expr::evaluate(comp, &target, prop, f)
                        .err()
                        .map(|e| (f, e))
                })
                .collect();
            let Some((first, e)) = failing.first() else {
                continue;
            };
            refused = true;
            let owner = crate::expr::owner_name(comp, &target);
            let at: Vec<i32> = failing.iter().map(|(f, _)| *f).collect();
            report.diagnostics.push(
                Diagnostic::new(
                    e.id,
                    Severity::Error,
                    format!(
                        "The expression on {owner}'s {prop} does not work at frame {first}, so \
                         nothing was exported."
                    ),
                    format!("{}. It fails on frames {}.", e.message, ranges(&at)),
                )
                .with_remediation("Correct the expression, or switch it off, and export again."),
            );
        }
        // D-291: and so does one on an effect's setting.
        for layer in comp.layers_in_order() {
            for i in layer.effects.iter().filter(|i| {
                i.tracks.values().any(|t| t[0].live_expression().is_some())
            }) {
                let mut failing: Vec<(i32, String, crate::expr::ExprError)> = Vec::new();
                for &f in &frames {
                    let (_, failed) =
                        crate::expr::effect_at(comp, &layer.id, i, f, layer.key_time(f as f64));
                    failing.extend(failed.into_iter().map(|(name, e)| (f, name, e)));
                }
                let Some((first, name, e)) = failing.first() else {
                    continue;
                };
                refused = true;
                let at: Vec<i32> = failing.iter().map(|(f, ..)| *f).collect();
                report.diagnostics.push(
                    Diagnostic::new(
                        e.id,
                        Severity::Error,
                        format!(
                            "The expression on {}'s {} {name} does not work at frame {first}, so \
                             nothing was exported.",
                            layer.name,
                            i.effect.name()
                        ),
                        format!("{}. It fails on frames {}.", e.message, ranges(&at)),
                    )
                    .with_remediation("Correct the expression, or switch it off, and export again."),
                );
            }
        }
        if refused {
            report.status = ExportStatus::Blocked;
            return report;
        }
    }

    // Document 07's default: a missing drawing blocks a final export, and nothing is written
    // before that is known. This plans every frame first, which costs the decode twice over a
    // long range.
    // ponytail: re-planning is the cost of blocking before the first write. A job snapshot that
    // kept the plans would remove it, and belongs with the cache in document 27, which is PARKED.
    if request.missing == MissingSource::Block {
        let mut scan = FrameLog::new(3);
        let mut unresolved: Vec<i32> = Vec::new();
        // D-231: planned side by side, then read in order as if planned one after another.
        for chunk in frames.chunks(frames_at_once(project, request)) {
            let planned: Vec<(FrameLog, Result<(), Diagnostic>)> = chunk
                .par_iter()
                .map(|&frame| {
                    let mut journal = FrameLog::journal();
                    let planned =
                        compose::plan_frame(project, &request.composition, frame, root, &mut journal);
                    (journal, planned.map(|_| ()))
                })
                .collect();
            for (&frame, (journal, planned)) in chunk.iter().zip(planned) {
                journal.replay_into(&mut scan);
                if let Err(d) = planned {
                    report.status = ExportStatus::Failed;
                    report.diagnostics.push(d);
                    return report;
                }
                if scan
                    .ids_at(frame)
                    .iter()
                    .any(|id| is_unresolved_source(*id))
                {
                    unresolved.push(frame);
                }
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
    // D-231: a chunk of frames is drawn, and each frame's file encoded, side by side. Then they
    // are written one at a time in order, each frame's log handed to the job's log as if the
    // frames had been drawn one after another, so the files, the report and the order of writing
    // are what drawing one frame at a time gave.
    for chunk in frames.chunks(frames_at_once(project, request)) {
        let mut drawn = chunk
            .par_iter()
            .map(|&frame| draw(project, root, request, frame, cancel))
            .collect::<Vec<_>>()
            .into_iter();
        for &frame in chunk {
            // D-252: Pause waits here, between frames as Cancel does, so the frames already drawn
            // in this chunk are held, not written, until it is pressed again.
            while watch.pause.load(Ordering::SeqCst) && !cancel.load(Ordering::SeqCst) {
                std::thread::sleep(std::time::Duration::from_millis(20));
            }
            // Between frames, not during one: a cancelled job leaves whole files behind.
            if cancel.load(Ordering::SeqCst) {
                report.status = ExportStatus::Cancelled;
                report.diagnostics.push(Diagnostic::new(
                    DiagnosticId::ExportCancelled,
                    Severity::Info,
                    if one_file {
                        "Export stopped at your request. An animated file is whole or it is nothing, \
                         so no file was left."
                            .to_string()
                    } else {
                        format!(
                            "Export stopped at your request after {} of {} frames.",
                            report.written.len(),
                            frames.len()
                        )
                    },
                    if one_file {
                        format!(
                            "Frame {frame} had not been written when the request arrived. An animated \
                             file is whole or it is nothing, so {} was removed.",
                            film_path.display()
                        )
                    } else {
                        format!(
                            "Frame {frame} had not been written when the request arrived. The frames \
                             already written are complete files and were left in place."
                        )
                    },
                ));
                if film.take().is_some() {
                    let _ = std::fs::remove_file(&film_path);
                }
                report.diagnostics.extend(log.finish());
                return report;
            }

            let (journal, drawn) = drawn
                .next()
                .flatten()
                .expect("a frame is left undrawn only once cancelled, which the check above stops at");
            journal.replay_into(&mut log);
            let Drawn { buffer, bypassed: drew_bypassed, encoded, small } = match drawn {
                Ok(drawn) => drawn,
                Err(d) => {
                    report.status = ExportStatus::Failed;
                    report.diagnostics.push(d);
                    report.diagnostics.extend(log.finish());
                    return report;
                }
            };
            let bypassed = bypassed(&log.ids_at(frame));
            report.fidelity_incomplete |= bypassed;

            let path = if one_file {
                film_path.clone()
            } else {
                request.output_dir.join(expand(&request.naming, frame))
            };
            let tags = tags(request, frame, bypassed);
            if one_file && film.is_none() {
                let rate = project
                    .composition(&request.composition)
                    .map(|c| c.frame_rate)
                    .ok_or_else(|| "the composition is not in the project".to_string());
                let (w, h) = (buffer.width(), buffer.height());
                let opened = rate.and_then(|rate| match request.format {
                    OutputFormat::Gif => match Film::gif_refusal(w, h) {
                        Some(why) => Err(why),
                        None => Film::gif(&path, w, h, rate, request.choices.gif_dither),
                    },
                    OutputFormat::Mp4 => match crate::mp4_out::refusal(w, h) {
                        Some(why) => Err(why),
                        None => crate::mp4_out::Mp4::create(&path, w, h, rate, request.choices.mp4_quality)
                            .map(Film::Mp4),
                    },
                    // The header is written once, so it carries no `Frame` tag, and its `Fidelity`
                    // tag speaks for the first frame only; the report speaks for them all.
                    _ => {
                        let header: Vec<(&str, String)> =
                            tags.iter().filter(|(k, _)| *k != "Frame").cloned().collect();
                        Film::apng(&path, w, h, request.depth, rate, frames.len() as u32, &header)
                    }
                });
                match opened {
                    Ok(opened) => film = Some(opened),
                    Err(why) => {
                        report.status = ExportStatus::Failed;
                        report.diagnostics.push(invalid(why));
                        return report;
                    }
                }
            }
            let written = match encoded {
                Encoded::Film { srgb8, samples } => film
                    .as_mut()
                    .expect("the film was opened above")
                    .push(buffer.width(), buffer.height(), &srgb8, &samples),
                // A PNG was encoded with the fidelity mark its own frame's log called for. The job's
                // log is the one that decides, and should an earlier frame have logged against this
                // one, the file is encoded again here with the job's answer.
                Encoded::Png(bytes) => {
                    let bytes = if drew_bypassed == bypassed {
                        bytes
                    } else {
                        png_encode(request, &buffer, &tags)
                    };
                    bytes.and_then(|b| std::fs::write(&path, b).map_err(|e| e.to_string()))
                }
                // D-62 writes no attribute beyond its list, so the tags above, the fidelity mark
                // included, are not in an EXR; the report's `fidelity_incomplete` still says it.
                Encoded::Exr(bytes) => bytes.and_then(|b| exr_io::write_bytes(&path, &b)),
            };
            if let Err(e) = written {
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
                if film.take().is_some() {
                    let _ = std::fs::remove_file(&film_path);
                }
                report.diagnostics.extend(log.finish());
                return report;
            }
            if !one_file {
                report.written.push(path);
            }
            if log.ids_at(frame).iter().any(|id| is_unresolved_source(*id)) {
                watch.missing.lock().expect("the watch lock was poisoned").push(frame);
            }
            {
                let mut written = watch.written.lock().expect("the watch lock was poisoned");
                if written.len() == WATCHED {
                    written.pop_front();
                }
                written.push_back((frame, small.0, small.1, small.2));
            }
            done.fetch_add(1, Ordering::SeqCst);
        }
    }
    if let Some(film) = film {
        match film.finish() {
            Ok(()) => report.written.push(film_path),
            Err(e) => {
                report.status = ExportStatus::Failed;
                report.diagnostics.push(invalid(format!(
                    "{} could not be finished: {e}.",
                    film_path.display()
                )));
                let _ = std::fs::remove_file(&film_path);
            }
        }
    }

    report.diagnostics.extend(log.finish());
    report
}

/// D-231: how many frames are drawn at once. As many as the machine has threads, and no more
/// than keeps the frames in flight near 2 GiB, counting four float pictures a frame for what a
/// render holds while it works: 15 at 1920x1080, 3 at 3840x2160.
fn frames_at_once(project: &Project, request: &ExportRequest) -> usize {
    if request.choices.frames_at_once > 0 {
        return request.choices.frames_at_once;
    }
    let pixels = project
        .composition(&request.composition)
        .map_or(1, |c| c.width as usize * c.height as usize)
        .max(1);
    // ponytail: a fixed 2 GiB. Take it from Preferences' memory setting if a heavy project
    // runs a machine short.
    ((2usize << 30) / (pixels * 16 * 4)).clamp(1, rayon::current_num_threads())
}

/// One frame drawn ahead of its turn to be written.
struct Drawn {
    buffer: WorkingBuffer,
    /// Whether this frame's own log marked it as missing something, which is what its PNG was
    /// encoded with.
    bypassed: bool,
    encoded: Encoded,
    /// D-252: the frame [`small`], for a window watching.
    small: (usize, usize, Vec<u8>),
}

/// The file a frame becomes, made beside the other frames' so that writing it is only writing.
enum Encoded {
    Png(Result<Vec<u8>, String>),
    Exr(Result<Vec<u8>, String>),
    /// The two encodings `Film::push` takes.
    Film { srgb8: Vec<u8>, samples: Vec<u8> },
}

/// Draw `frame` into a log of its own and encode its file. `None` when the job was cancelled
/// before it started, which the writing loop sees before it reaches this frame.
#[allow(clippy::type_complexity)]
/// D-311: lay 8-bit straight RGBA over a background colour, linear RGB, in encoded values, the
/// way the MP4 was always laid over black; every alpha becomes 255.
pub fn lay_over(srgb8: &mut [u8], background: [f64; 3]) {
    let bg = background.map(|v| crate::color::linear_to_srgb(v as f32) * 255.0);
    for px in srgb8.chunks_exact_mut(4) {
        let a = px[3] as f32 / 255.0;
        for i in 0..3 {
            px[i] = (px[i] as f32 * a + bg[i] * (1.0 - a)).round() as u8;
        }
        px[3] = 255;
    }
}

fn draw(
    project: &Project,
    root: &Path,
    request: &ExportRequest,
    frame: i32,
    cancel: &AtomicBool,
) -> Option<(FrameLog, Result<Drawn, Diagnostic>)> {
    if cancel.load(Ordering::SeqCst) {
        return None;
    }
    let mut journal = FrameLog::journal();
    let drawn = compose::render_frame(
        project,
        &request.composition,
        frame,
        root,
        request.tile_size,
        &mut journal,
    )
    .map(|buffer| {
        let bypassed = bypassed(&journal.ids_at(frame));
        let encoded = match request.format {
            OutputFormat::Png => Encoded::Png(png_encode(request, &buffer, &tags(request, frame, bypassed))),
            OutputFormat::Exr(samples) => Encoded::Exr(
                project
                    .composition(&request.composition)
                    .map(|c| c.frame_rate)
                    .ok_or_else(|| "the composition is not in the project".to_string())
                    .and_then(|rate| exr_io::encode(&buffer, samples, rate)),
            ),
            OutputFormat::Gif | OutputFormat::Apng | OutputFormat::Mp4 => {
                let mut srgb8 = buffer.encode(OutputDepth::Eight, OutputAlpha::Straight);
                // D-311: an MP4 has no alpha, so it is laid over the background colour, in the
                // same encoded values it was always laid over black in, as the viewer does.
                let background = project.composition(&request.composition).and_then(|c| c.background_color);
                if let (OutputFormat::Mp4, Some(bg)) = (request.format, background) {
                    lay_over(&mut srgb8, bg);
                }
                Encoded::Film { srgb8, samples: buffer.encode(request.depth, request.alpha) }
            }
        };
        let small = small(&buffer, 480);
        Drawn { buffer, bypassed, encoded, small }
    });
    Some((journal, drawn))
}

/// Document 28: "exported output must report that fidelity is incomplete" when a feature was
/// bypassed. Four identifiers mean that -- the generic one D-24 registered, the mask outline
/// D-43 added, and the two effect faults B-07 brought in -- and any one of them is enough to
/// mark the file.
fn bypassed(ids: &[DiagnosticId]) -> bool {
    ids.contains(&DiagnosticId::ProjectFeatureUnsupported)
        || ids.contains(&DiagnosticId::MaskInvalidOutline)
        || ids.contains(&DiagnosticId::EffectUnsupported)
        || ids.contains(&DiagnosticId::EffectParameterInvalid)
        || ids.contains(&DiagnosticId::EffectLayerTooLarge)
        || ids.contains(&DiagnosticId::TextFontMissing)
}

/// The text tags an exported PNG carries.
fn tags(request: &ExportRequest, frame: i32, bypassed: bool) -> Vec<(&'static str, String)> {
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
            // The wording said "a parked feature" until B-07. Nothing that raises this is
            // parked any more -- a crossed mask outline and an effect this build does not
            // have are both things it refuses to guess at -- so the tag says what is
            // actually true of the file.
            "incomplete: a layer carried something this build could not draw".to_string(),
        ));
    }
    tags
}

fn png_encode(request: &ExportRequest, buffer: &WorkingBuffer, tags: &[(&str, String)]) -> Result<Vec<u8>, String> {
    png_out::encode_rgba(
        buffer.width(),
        buffer.height(),
        request.depth,
        tags,
        &buffer.encode(request.depth, request.alpha),
    )
    .map_err(|e| e.to_string())
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
pub fn ranges(frames: &[i32]) -> String {
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
