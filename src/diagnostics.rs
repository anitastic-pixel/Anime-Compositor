//! Diagnostics, per document 28.
//!
//! Document 28's contract: "Diagnostics have stable machine IDs, severity, concise user
//! message, technical detail and optional remediation action. Internal exceptions or library
//! error strings are not user-facing identifiers."
//!
//! Only the IDs actually raised by implemented code appear here. The catalog in document 28
//! is larger; adding an unused variant would claim behaviour that does not exist.

use std::fmt;

/// Document 28: "WARNING permits the current operation with explicit degradation; ERROR
/// rejects the requested operation but keeps the app usable; FATAL means the current
/// project/process cannot continue safely."
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Fatal,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Severity::Info => "INFO",
            Severity::Warning => "WARNING",
            Severity::Error => "ERROR",
            Severity::Fatal => "FATAL",
        })
    }
}

/// One line per identifier: its name in the code, its spelling, and whether document 28 lists
/// it. [`DiagnosticId::as_str`], [`DiagnosticId::in_catalog`] and [`DiagnosticId::ALL`] are all
/// made from these lines, so an identifier cannot be added without saying both.
macro_rules! diagnostic_ids {
    ($($(#[$doc:meta])* $id:ident = ($spelling:literal, $listed:literal),)*) => {
        /// Stable machine identifiers.
        ///
        /// Eight of these are **proposed additions** to document 28 rather than entries from it, and
        /// are registered as D-19, D-21 and D-24 in document 14. They are marked below. Test T-01 requires
        /// mismatched-dimension handling and document 28 has no identifier for it, so the choice was
        /// between inventing one openly and reusing an unrelated identifier, which would have been a
        /// silent reinterpretation of the catalog.
        #[derive(Clone, Copy, PartialEq, Eq, Debug)]
        pub enum DiagnosticId {
            $($(#[$doc])* $id,)*
        }

        impl DiagnosticId {
            /// Every identifier, in the order below.
            pub const ALL: &[DiagnosticId] = &[$(DiagnosticId::$id),*];

            pub fn as_str(self) -> &'static str {
                match self {
                    $(DiagnosticId::$id => $spelling,)*
                }
            }

            /// False for the identifiers D-19 and D-21 propose but document 28 does not yet list.
            ///
            /// Reports can therefore separate "the catalog said to say this" from "an agent decided
            /// to say this", without a reader having to know document 28 by heart.
            pub fn in_catalog(self) -> bool {
                match self {
                    $(DiagnosticId::$id => $listed,)*
                }
            }
        }
    };
}

diagnostic_ids! {
    /// Document 28: project requires a newer unsupported schema; do not reinterpret.
    ProjectSchemaNewer = ("PROJECT_SCHEMA_NEWER", true),
    /// Document 28: project violates the schema or an invariant; refuse model construction.
    ProjectSchemaInvalid = ("PROJECT_SCHEMA_INVALID", true),
    /// Document 28: a save could not complete; the previous valid save is kept.
    ProjectSaveFailed = ("PROJECT_SAVE_FAILED", true),
    /// Document 28: an autosave or recovery candidate exists; show the timestamp and path.
    ProjectRecoveryAvailable = ("PROJECT_RECOVERY_AVAILABLE", true),
    /// Document 28: referenced media unavailable; the reference is preserved.
    MediaMissing = ("MEDIA_MISSING", true),
    /// Document 28: effect type not installed or implemented; the record is preserved and
    /// bypassed.
    EffectUnsupported = ("EFFECT_UNSUPPORTED", true),
    /// Document 28: an effect parameter violates its contract. Raised when a stored value is
    /// outside what document 21 defines - a negative blur sigma, a tint amount past 1 - which
    /// can only reach the model through a hand-edited file, because the command that sets a
    /// parameter refuses it first. The effect is bypassed, never clamped: clamping would accept
    /// the number and render a different one.
    EffectParameterInvalid = ("EFFECT_PARAMETER_INVALID", true),
    /// **Proposed (D-24).** A structurally valid record for a project feature this build does
    /// not implement, other than an effect. The record is preserved and takes no part in
    /// rendering.
    ///
    /// Masks were the only case until B-06 drew them on 2026-09-06. A mask this build refuses
    /// now says so through `MaskInvalidOutline`, which D-43 added to document 28 because a
    /// crossed outline is a specific fault with a specific fix rather than an unimplemented
    /// feature. D-24 registered it, and export treats it as grounds for marking a file's fidelity
    /// incomplete. D-527: Timecode on a layer with a file says it, as information, since the
    /// file's own starting timecode is not read.
    ProjectFeatureUnsupported = ("PROJECT_FEATURE_UNSUPPORTED", false),
    /// Document 28: requested drawing number absent; do not substitute adjacent frame.
    MediaSequenceGap = ("MEDIA_SEQUENCE_GAP", true),
    /// Document 28: decoder not supported; preserve asset record, report format.
    MediaUnsupportedFormat = ("MEDIA_UNSUPPORTED_FORMAT", true),
    /// Document 28: supported decoder failed on file.
    MediaDecodeFailed = ("MEDIA_DECODE_FAILED", true),
    /// Document 28, added by D-62: an EXR file was drawn, but not exactly as stored. One reason
    /// per thing done, such as channels left unused or samples that were not numbers.
    MediaExrAdjusted = ("MEDIA_EXR_ADJUSTED", true),
    /// Document 28, added by D-71: a WAV file's data ends before its header says. What is
    /// there plays.
    MediaAudioCutShort = ("MEDIA_AUDIO_CUT_SHORT", true),
    /// Document 28, added by D-71: a sound file is not what its name claims. The layer is
    /// kept and silent.
    MediaAudioUnreadable = ("MEDIA_AUDIO_UNREADABLE", true),
    /// Document 28, added by D-639: a sound the core does not read (Opus). The window still
    /// plays it; exports and sound effects leave it out.
    MediaAudioPreviewOnly = ("MEDIA_AUDIO_PREVIEW_ONLY", true),
    /// **Proposed (D-19).** Files in one sequence disagree on pixel dimensions.
    MediaSequenceDimensionMismatch = ("MEDIA_SEQUENCE_DIMENSION_MISMATCH", false),
    /// **Proposed (D-19).** Two files in one selection claim the same frame number.
    MediaSequenceDuplicateNumber = ("MEDIA_SEQUENCE_DUPLICATE_NUMBER", false),
    /// **Proposed (D-19).** A selected file carries no frame number at all.
    MediaSequenceUnnumbered = ("MEDIA_SEQUENCE_UNNUMBERED", false),
    /// **Proposed (D-19).** A file joins the sequence under a name the pattern does not
    /// generate. Informational: the file is used, and its literal name is recorded.
    MediaSequenceNameVariant = ("MEDIA_SEQUENCE_NAME_VARIANT", false),
    /// Document 28: matte ID unresolved. Raised as ERROR by a command that would create the
    /// unresolved reference; the catalog's WARNING applies to one found while loading.
    MatteReferenceMissing = ("MATTE_REFERENCE_MISSING", true),
    /// Document 28: matte dependency cycle detected; reject the command.
    MatteCycle = ("MATTE_CYCLE", true),
    /// Document 28, added by D-57: a parent ID no layer in the composition matches. WARNING
    /// when a loaded project holds one, in which case the reference is kept and the layer draws
    /// as if it had no parent; ERROR from a command that would create one.
    ParentReferenceMissing = ("PARENT_REFERENCE_MISSING", true),
    /// Document 28, added by D-57: a parent chain that loops back on itself. Refused the way
    /// `MatteCycle` is, by both the command and the loader.
    ParentCycle = ("PARENT_CYCLE", true),
    /// Document 28, added by D-67: a composition layer naming a composition the project does
    /// not have. WARNING: the reference is kept and the layer draws nothing.
    CompositionReferenceMissing = ("COMPOSITION_REFERENCE_MISSING", true),
    /// Document 28, added by D-67: a composition that would show itself, directly or through
    /// others. Refused the way `MatteCycle` is, by both the command and the loader.
    CompositionCycle = ("COMPOSITION_CYCLE", true),
    /// Document 28, added by D-58: a layer level with the camera or behind it, which has no
    /// size because `world_depth - camera_depth` is zero or negative. WARNING: the layer is not
    /// drawn for that frame, its record is untouched, and nothing is clamped into a working
    /// value. One pixel in front of the camera is not this case and is drawn enormous, which is
    /// correct.
    CameraPlaneBehind = ("CAMERA_PLANE_BEHIND", true),
    /// Document 28, added by D-43: a mask outline this build refuses to draw -- fewer than
    /// three corners, or an outline that crosses itself. ERROR when a command would create
    /// one, WARNING when a project already on disk holds one, in which case the record is
    /// preserved, the layer draws unmasked, and the output says its fidelity is incomplete.
    MaskInvalidOutline = ("MASK_INVALID_OUTLINE", true),
    /// Document 28, added by D-78: a shape with fewer than two points, which has nothing to fill
    /// and nothing to stroke between. ERROR when a command would create one, WARNING when a
    /// project already on disk holds one, in which case the record is preserved and that shape
    /// draws nothing.
    ///
    /// A shape that crosses itself is deliberately *not* this: unlike a mask, where nobody can
    /// say which side to keep, the even-odd rule says exactly what a crossed shape fills, so it
    /// is drawn.
    ShapeInvalidOutline = ("SHAPE_INVALID_OUTLINE", true),
    /// Document 28, added by D-263: a text layer names a font this machine does not have, or a
    /// file that is not a font. WARNING per frame; the words are kept and draw nothing, and no
    /// other font is put in its place.
    TextFontMissing = ("TEXT_FONT_MISSING", true),
    /// **Proposed (D-21).** A command named a composition, layer or keyframe that is not there.
    CommandTargetMissing = ("COMMAND_TARGET_MISSING", false),
    /// **Proposed (D-21).** A command carried a value the model cannot hold: the wrong value
    /// kind for the property, a non-finite number, or an index outside the layer stack.
    CommandInvalidValue = ("COMMAND_INVALID_VALUE", false),
    /// **Proposed (D-21).** A command would have changed a locked layer.
    CommandLayerLocked = ("COMMAND_LAYER_LOCKED", false),
    /// Document 28: an output file could not be written; the completed frames and the failing
    /// path are reported and the job is not called successful.
    ExportWriteFailed = ("EXPORT_WRITE_FAILED", true),
    /// Document 28: the export was cancelled; the completed-frame list is preserved and no
    /// success is claimed.
    ExportCancelled = ("EXPORT_CANCELLED", true),
    /// **Proposed (D-28).** An export was refused before writing anything because a frame in
    /// the requested range has no source drawing. Document 07 requires a blocked final export
    /// as the default missing-frame behaviour and document 28 names no identifier for it.
    ExportBlockedMissingMedia = ("EXPORT_BLOCKED_MISSING_MEDIA", false),
    /// Document 28, added by D-641: an export's sound went past full level, and those samples
    /// were written at full level. WARNING; the file is written.
    ExportSoundClipped = ("EXPORT_SOUND_CLIPPED", true),
    /// Document 28, added by D-59: an expression that cannot be read, or that names a word, a
    /// member or a function the language does not have. ERROR, though the frame is still drawn with
    /// the property at its keyed value; an export that meets one is refused.
    ExpressionSyntax = ("EXPRESSION_SYNTAX", true),
    /// Document 28, added by D-59: an expression that reads but gives the wrong kind of value,
    /// or a number that is not finite.
    ExpressionType = ("EXPRESSION_TYPE", true),
    /// Document 28, added by D-59: an expression naming a layer ID the composition does not hold.
    ExpressionReferenceMissing = ("EXPRESSION_REFERENCE_MISSING", true),
    /// Document 28, added by D-59: expressions that read each other in a loop.
    ExpressionCycle = ("EXPRESSION_CYCLE", true),
    /// Document 28, added by D-59: an expression that takes more than its steps or its depth.
    ExpressionTimeout = ("EXPRESSION_TIMEOUT", true),
    /// Document 28, added by D-61: collecting was refused because the destination folder holds something, or its `.partial` sibling exists.
    PackageDestinationNotEmpty = ("PACKAGE_DESTINATION_NOT_EMPTY", true),
    /// Document 28, added by D-61: a package could not be written; the partial folder was removed.
    PackageWriteFailed = ("PACKAGE_WRITE_FAILED", true),
    /// Document 28, added by D-61: a package's manifest is absent or cannot be read.
    PackageManifestInvalid = ("PACKAGE_MANIFEST_INVALID", true),
    /// Document 28, added by D-61: a packaged file's size or fingerprint differs from its manifest.
    PackageFileChanged = ("PACKAGE_FILE_CHANGED", true),
    /// Document 28, added by D-61: a file the package lists but did not copy, because its asset may not be passed on.
    PackageMediaExcluded = ("PACKAGE_MEDIA_EXCLUDED", true),
    /// Document 28, added by D-61: a file is present where the manifest recorded none, so nothing can vouch for it.
    PackageFileUnverified = ("PACKAGE_FILE_UNVERIFIED", true),
    /// Document 28, added by D-84: the chosen folder holds no `.xdts` file, or more than one.
    TimesheetNotFound = ("TIMESHEET_NOT_FOUND", true),
    /// Document 28, added by D-84: the timesheet is not XDTS, or has no timetable or length.
    TimesheetUnreadable = ("TIMESHEET_UNREADABLE", true),
    /// Document 28, added by D-84: no drawing column of the timesheet can become a layer.
    TimesheetNoCells = ("TIMESHEET_NO_CELLS", true),
    /// Document 28, added by D-84: the timesheet's version is not 5; it is read anyway.
    TimesheetVersion = ("TIMESHEET_VERSION", true),
    /// Document 28, added by D-84: a timesheet's timetables after the first are not read.
    TimesheetTableNotRead = ("TIMESHEET_TABLE_NOT_READ", true),
    /// Document 28, added by D-84: a dialogue, camerawork or unknown field is not read.
    TimesheetFieldNotRead = ("TIMESHEET_FIELD_NOT_READ", true),
    /// Document 28, added by D-84: a drawing column with no name makes no layer.
    TimesheetColumnUnnamed = ("TIMESHEET_COLUMN_UNNAMED", true),
    /// Document 28, added by D-84: an entry outside the sheet, or a second on one frame, is left out.
    TimesheetEntryIgnored = ("TIMESHEET_ENTRY_IGNORED", true),
    /// Document 28, added by D-84: an entry before frame 0 stands on frame 0.
    TimesheetEntryCarriedIn = ("TIMESHEET_ENTRY_CARRIED_IN", true),
    /// Document 28, added by D-84: a cell that is not a whole number or a symbol reads as blank.
    TimesheetCellUnreadable = ("TIMESHEET_CELL_UNREADABLE", true),
    /// Document 28, added by D-84: a tick mark, which changes nothing shown.
    TimesheetMark = ("TIMESHEET_MARK", true),
    /// Document 28, added by D-84: a drawing column that never shows a drawing makes no layer.
    TimesheetColumnEmpty = ("TIMESHEET_COLUMN_EMPTY", true),
    /// Document 28, added by D-84: a column with no folder and no loose files makes no layer.
    TimesheetColumnNoDrawings = ("TIMESHEET_COLUMN_NO_DRAWINGS", true),
    /// Document 28, added by D-84: the sheet calls for a drawing its column does not have.
    TimesheetDrawingMissing = ("TIMESHEET_DRAWING_MISSING", true),
    /// Document 28, added by D-84: a column has drawings the sheet never shows.
    TimesheetDrawingUnused = ("TIMESHEET_DRAWING_UNUSED", true),
    /// Document 28, added by D-84: a folder or drawing beside the sheet that no column used.
    TimesheetNotUsed = ("TIMESHEET_NOT_USED", true),
    /// **Accepted (D-101)**, which keeps it out of document 28's core table (it is listed after
    /// it, under the identifiers accepted by decision). The viewer asked the GPU for a frame (B-44) and the CPU drew it
    /// instead, with the reason: an adjustment layer, a picture larger than the card allows, or
    /// the card failing. INFO when planned, WARNING when the card failed. Nothing is refused, so
    /// this is not document 28's GPU_BACKEND_FAILED, which is an ERROR.
    GpuPreviewOnCpu = ("GPU_PREVIEW_ON_CPU", false),
    /// **Accepted (D-893)**, outside document 28's core table. A viewer frame was stopped between
    /// layers because newer input made it useless ([`crate::stop`]). INFO. Nothing of it is shown
    /// or kept; the newer frame is drawn instead.
    PreviewFrameStopped = ("PREVIEW_FRAME_STOPPED", false),
    /// Document 28, added by D-180: a preset file that is not the preset file format, or is a
    /// newer version of it. Nothing is imported and nothing is changed.
    PresetFileInvalid = ("PRESET_FILE_INVALID", true),
    /// Document 28, added by D-189: an effect's layer setting naming a layer that is not in the
    /// holder's composition, deleted or of another composition. WARNING: the setting is kept
    /// as written and the effect is skipped, every frame.
    EffectLayerMissing = ("EFFECT_LAYER_MISSING", true),
    /// Document 28, added by D-189 and built with D-191: effects' layer settings that lead
    /// back round to a layer. ERROR: refused on command and on load, as MATTE_CYCLE is.
    EffectLayerCycle = ("EFFECT_LAYER_CYCLE", true),
    /// Document 28, added by D-348: an effect that reads a pass stored beside the colour in an
    /// EXR file - the depth or the normals - on a layer whose footage at that frame has none, or
    /// is not an EXR, or that has no footage. WARNING: the settings are kept and the effect is
    /// skipped, every frame.
    EffectChannelMissing = ("EFFECT_CHANNEL_MISSING", true),
    /// Document 28, added by D-407: an effect that would grow the layer past what this build can
    /// hold - a Detail-preserving Upscale wider or taller than 30000 pixels, or whose memory
    /// cannot be had. WARNING: the settings are kept and the effect is skipped, every frame.
    EffectLayerTooLarge = ("EFFECT_LAYER_TOO_LARGE", true),
    /// Document 28, added by D-356: an effect that draws along the layer's masks (Path Stroke)
    /// on a layer with no such mask at that frame - none numbered so, or none enabled with two
    /// points or more. WARNING: the settings are kept and the layer is left as it is, every frame.
    EffectPathMissing = ("EFFECT_PATH_MISSING", true),
    /// Document 28, added by D-420: an effect that listens to a sound layer (Audio Spectrum)
    /// whose Audio Layer is a layer that holds no sound. WARNING: the settings are kept and the
    /// layer is left as it is, every frame.
    EffectSoundMissing = ("EFFECT_SOUND_MISSING", true),
    /// Document 28, added by D-350: a text animator on a layer that is not a text layer, which has
    /// no letters to move. WARNING every frame; the animator is kept and changes nothing.
    TextAnimatorNoText = ("TEXT_ANIMATOR_NO_TEXT", true),
    /// Document 28, added by D-351: a Stretch effect with temporal smoothing on an adjustment
    /// layer, where the frames beneath at other times are not to hand. WARNING every frame; each
    /// frame is stretched by itself and the setting is kept.
    TemporalSmoothingSkipped = ("TEMPORAL_SMOOTHING_SKIPPED", true),
}

impl fmt::Display for DiagnosticId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One diagnostic, in the shape document 28 requires.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Diagnostic {
    pub id: DiagnosticId,
    pub severity: Severity,
    /// Concise, addressed to the person using the application.
    pub message: String,
    /// Technical detail: which file, which numbers, what was found.
    pub detail: String,
    /// Document 28: "Every actionable message states what failed and the next safe action."
    pub remediation: Option<String>,
}

impl Diagnostic {
    pub fn new(
        id: DiagnosticId,
        severity: Severity,
        message: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Diagnostic {
            id,
            severity,
            message: message.into(),
            detail: detail.into(),
            remediation: None,
        }
    }

    pub fn with_remediation(mut self, remediation: impl Into<String>) -> Self {
        self.remediation = Some(remediation.into());
        self
    }

    /// One diagnostic as the window says it: what happened, then what to do about it.
    pub fn sentence(&self) -> String {
        match &self.remediation {
            Some(next) => format!("{} {}", self.message, next),
            None => self.message.clone(),
        }
    }

    /// W-25: a diagnostic as the notes strip carries it: the sentence, then after a tab its
    /// document 28 code and detail, which the page shows only when the error details are opened.
    pub fn note(&self) -> String {
        format!(
            "{}\t{} {}",
            self.sentence(),
            self.id.as_str(),
            self.detail.replace(['\n', '\t'], " ")
        )
    }
}

/// The multi-line form a user would see in a diagnostics panel.
///
/// This is rendered into the B-03 verification artifact so the owner reviews the actual words
/// rather than a description of them.
impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} [{}]", self.severity, self.id)?;
        writeln!(f, "{}", self.message)?;
        write!(f, "{}", self.detail)?;
        if let Some(r) = &self.remediation {
            write!(f, "\n{r}")?;
        }
        Ok(())
    }
}

/// Collapses repeated frame-level warnings, per document 28.
///
/// Document 28's logging rule: "Repeated frame-level warnings should be rate-limited while
/// retaining counts/ranges." A 240-frame composition whose layer exposes one missing drawing
/// every twelfth frame raises the same warning twenty times; a person reading that log learns
/// nothing from occurrences four through twenty that occurrence one did not already tell them,
/// and the twenty copies bury every other diagnostic in the run.
///
/// The policy this implements is **not** in document 28, which gives no number and no shape.
/// It is registered as D-25 in document 14 and is PROVISIONAL:
///
/// - the first [`FrameLog::new`] `limit` occurrences of a warning are logged in full, unchanged;
/// - every later occurrence is suppressed;
/// - one summary record per suppressed group is appended at the end, carrying the total count
///   and the frame ranges, which is the part document 28 does require.
///
/// Grouping is by identifier **and** a caller-supplied subject, never by the message text: a
/// frame-level message names its own frame, so no two are equal and grouping on them would
/// suppress nothing. The subject is what makes two warnings "the same warning" — for a sequence
/// gap, the sequence and the drawing number.
pub struct FrameLog {
    limit: usize,
    groups: Vec<FrameGroup>,
    logged: Vec<Diagnostic>,
    /// D-231: every record whole and in order, kept only by [`FrameLog::journal`].
    journal: Option<Vec<(i32, String, Diagnostic)>>,
    /// Audit W8, W9: the effects the processor ran on a frame the card was asked for, by name,
    /// each once. Not a diagnostic: the viewer says it beside the frame ("partly on the
    /// processor"), as it says which processor drew it.
    on_processor: Vec<String>,
}

struct FrameGroup {
    id: DiagnosticId,
    subject: String,
    /// The first occurrence, kept whole so the summary can reuse its severity and remediation
    /// rather than inventing new words for a condition already described.
    first: Diagnostic,
    frames: Vec<i32>,
}

impl FrameLog {
    /// `limit` is how many occurrences of one warning are logged in full before suppression.
    pub fn new(limit: usize) -> Self {
        FrameLog {
            limit,
            groups: Vec::new(),
            logged: Vec::new(),
            journal: None,
            on_processor: Vec::new(),
        }
    }

    /// Audit W8, W9: `name`, an effect the processor ran on a frame planned for the card.
    pub fn ran_on_processor(&mut self, name: &str) {
        if !self.on_processor.iter().any(|n| n == name) {
            self.on_processor.push(name.to_string());
        }
    }

    /// The effects [`ran_on_processor`](Self::ran_on_processor) was told of, in order.
    pub fn on_processor(&self) -> &[String] {
        &self.on_processor
    }

    /// D-231: a log for one frame rendered on its own, beside others. It keeps every record
    /// whole, so [`replay_into`](Self::replay_into) can hand them to the job's log afterwards
    /// exactly as if the frame had been rendered into that log.
    pub fn journal() -> Self {
        FrameLog {
            journal: Some(Vec::new()),
            ..FrameLog::new(usize::MAX)
        }
    }

    /// Record, in `into`, everything this journal recorded, in the order it was recorded.
    pub fn replay_into(self, into: &mut FrameLog) {
        for (frame, subject, diagnostic) in
            self.journal.expect("replay_into needs FrameLog::journal")
        {
            into.record(frame, subject, diagnostic);
        }
    }

    /// D-812: a second journal of the same records, to replay into a second log.
    pub fn copy(&self) -> FrameLog {
        FrameLog {
            journal: self.journal.clone(),
            ..FrameLog::new(usize::MAX)
        }
    }

    /// Whether nothing has been recorded.
    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    /// Offer one frame's diagnostic to the log.
    ///
    /// Nothing is dropped here: a suppressed occurrence still contributes its frame to the
    /// group's count and ranges.
    pub fn record(&mut self, frame: i32, subject: impl Into<String>, diagnostic: Diagnostic) {
        let subject = subject.into();
        if let Some(journal) = &mut self.journal {
            journal.push((frame, subject.clone(), diagnostic.clone()));
        }
        let index = match self
            .groups
            .iter()
            .position(|g| g.id == diagnostic.id && g.subject == subject)
        {
            Some(i) => i,
            None => {
                self.groups.push(FrameGroup {
                    id: diagnostic.id,
                    subject,
                    first: diagnostic.clone(),
                    frames: Vec::new(),
                });
                self.groups.len() - 1
            }
        };
        let group = &mut self.groups[index];
        if group.frames.len() < self.limit {
            self.logged.push(diagnostic);
        }
        group.frames.push(frame);
    }

    /// D-67: take in what a composition layer's inner frame logged, as having happened at
    /// `frame` of the composition that was asked for, under `layer`'s name.
    pub fn absorb(&mut self, inner: FrameLog, frame: i32, layer: &str) {
        for group in inner.groups {
            for _ in &group.frames {
                self.record(
                    frame,
                    format!("{layer}/{}", group.subject),
                    group.first.clone(),
                );
            }
        }
    }

    /// D-196: take in what resolving a layer at a held frame logged, as having happened at
    /// `frame`, the frame that was asked for.
    pub fn retime(&mut self, inner: FrameLog, frame: i32) {
        for name in &inner.on_processor {
            self.ran_on_processor(name);
        }
        for group in inner.groups {
            for _ in &group.frames {
                self.record(frame, group.subject.clone(), group.first.clone());
            }
        }
    }

    /// Every identifier recorded against `frame`, **including suppressed occurrences**.
    ///
    /// The export writer asks this to decide whether a frame had a drawing to show. Reading the
    /// logged records instead would answer correctly for the first three frames of a gap and
    /// wrongly for the rest, which is the failure rate limiting is most likely to cause.
    pub fn ids_at(&self, frame: i32) -> Vec<DiagnosticId> {
        self.groups
            .iter()
            .filter(|g| g.frames.contains(&frame))
            .map(|g| g.id)
            .collect()
    }

    /// The log: every occurrence that was not suppressed, in the order it was recorded, then one
    /// summary for each group that had occurrences suppressed, in the order the groups first
    /// appeared.
    pub fn finish(self) -> Vec<Diagnostic> {
        let FrameLog {
            limit,
            groups,
            mut logged,
            ..
        } = self;
        for group in groups {
            let total = group.frames.len();
            if total <= limit {
                continue;
            }
            let mut summary = Diagnostic::new(
                group.id,
                group.first.severity,
                format!(
                    "{}: {total} frames affected. The first {limit} are logged in full above.",
                    group.subject
                ),
                format!(
                    "Frames {}. {} further identical warnings were not logged individually.",
                    frame_ranges(&group.frames),
                    total - limit
                ),
            );
            summary.remediation = group.first.remediation.clone();
            logged.push(summary);
        }
        logged
    }
}

/// Frame numbers as runs: `[14, 15, 38, 39, 41]` becomes `14 to 15, 38 to 39, 41`.
///
/// The separator is the word rather than a dash because layer-local frames are signed: a run
/// from -2 to 1 written with a dash reads `-2-1`, which is not a range and not a number.
///
/// Document 28 asks for ranges, not a list, and the difference matters at this scale: twenty
/// frames of a 240-frame shot read as ten places something is wrong, not twenty numbers.
/// Input is sorted and de-duplicated first, so a caller that visits frames out of order or
/// twice still gets one honest set of ranges.
pub fn frame_ranges(frames: &[i32]) -> String {
    let mut sorted: Vec<i32> = frames.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    let mut out = String::new();
    let mut i = 0;
    while i < sorted.len() {
        let start = sorted[i];
        let mut end = start;
        while i + 1 < sorted.len() && sorted[i + 1] == end + 1 {
            i += 1;
            end = sorted[i];
        }
        if !out.is_empty() {
            out.push_str(", ");
        }
        if start == end {
            out.push_str(&start.to_string());
        } else {
            out.push_str(&format!("{start} to {end}"));
        }
        i += 1;
    }
    out
}
