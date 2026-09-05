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

/// Stable machine identifiers.
///
/// Eight of these are **proposed additions** to document 28 rather than entries from it, and
/// are registered as D-19, D-21 and D-24 in document 14. They are marked below. Test T-01 requires
/// mismatched-dimension handling and document 28 has no identifier for it, so the choice was
/// between inventing one openly and reusing an unrelated identifier, which would have been a
/// silent reinterpretation of the catalog.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DiagnosticId {
    /// Document 28: project requires a newer unsupported schema; do not reinterpret.
    ProjectSchemaNewer,
    /// Document 28: project violates the schema or an invariant; refuse model construction.
    ProjectSchemaInvalid,
    /// Document 28: a save could not complete; the previous valid save is kept.
    ProjectSaveFailed,
    /// Document 28: an autosave or recovery candidate exists; show the timestamp and path.
    ProjectRecoveryAvailable,
    /// Document 28: referenced media unavailable; the reference is preserved.
    MediaMissing,
    /// Document 28: effect type not installed or implemented; the record is preserved and
    /// bypassed.
    EffectUnsupported,
    /// **Proposed (D-24).** A structurally valid record for a project feature this build does
    /// not implement, other than an effect. Masks are the only case today: document 23 parks
    /// them with R-04, the schema carries them, and document 28 has an identifier for an
    /// unsupported effect but none for an unsupported feature. The record is preserved and
    /// takes no part in rendering.
    ProjectFeatureUnsupported,
    /// Document 28: requested drawing number absent; do not substitute adjacent frame.
    MediaSequenceGap,
    /// Document 28: decoder not supported; preserve asset record, report format.
    MediaUnsupportedFormat,
    /// Document 28: supported decoder failed on file.
    MediaDecodeFailed,
    /// **Proposed (D-19).** Files in one sequence disagree on pixel dimensions.
    MediaSequenceDimensionMismatch,
    /// **Proposed (D-19).** Two files in one selection claim the same frame number.
    MediaSequenceDuplicateNumber,
    /// **Proposed (D-19).** A selected file carries no frame number at all.
    MediaSequenceUnnumbered,
    /// **Proposed (D-19).** A file joins the sequence under a name the pattern does not
    /// generate. Informational: the file is used, and its literal name is recorded.
    MediaSequenceNameVariant,
    /// Document 28: matte ID unresolved. Raised as ERROR by a command that would create the
    /// unresolved reference; the catalog's WARNING applies to one found while loading.
    MatteReferenceMissing,
    /// Document 28: matte dependency cycle detected; reject the command.
    MatteCycle,
    /// **Proposed (D-21).** A command named a composition, layer or keyframe that is not there.
    CommandTargetMissing,
    /// **Proposed (D-21).** A command carried a value the model cannot hold: the wrong value
    /// kind for the property, a non-finite number, or an index outside the layer stack.
    CommandInvalidValue,
    /// **Proposed (D-21).** A command would have changed a locked layer.
    CommandLayerLocked,
}

impl DiagnosticId {
    pub fn as_str(self) -> &'static str {
        match self {
            DiagnosticId::ProjectSchemaNewer => "PROJECT_SCHEMA_NEWER",
            DiagnosticId::ProjectSchemaInvalid => "PROJECT_SCHEMA_INVALID",
            DiagnosticId::ProjectSaveFailed => "PROJECT_SAVE_FAILED",
            DiagnosticId::ProjectRecoveryAvailable => "PROJECT_RECOVERY_AVAILABLE",
            DiagnosticId::MediaMissing => "MEDIA_MISSING",
            DiagnosticId::EffectUnsupported => "EFFECT_UNSUPPORTED",
            DiagnosticId::ProjectFeatureUnsupported => "PROJECT_FEATURE_UNSUPPORTED",
            DiagnosticId::MediaSequenceGap => "MEDIA_SEQUENCE_GAP",
            DiagnosticId::MediaUnsupportedFormat => "MEDIA_UNSUPPORTED_FORMAT",
            DiagnosticId::MediaDecodeFailed => "MEDIA_DECODE_FAILED",
            DiagnosticId::MediaSequenceDimensionMismatch => "MEDIA_SEQUENCE_DIMENSION_MISMATCH",
            DiagnosticId::MediaSequenceDuplicateNumber => "MEDIA_SEQUENCE_DUPLICATE_NUMBER",
            DiagnosticId::MediaSequenceUnnumbered => "MEDIA_SEQUENCE_UNNUMBERED",
            DiagnosticId::MediaSequenceNameVariant => "MEDIA_SEQUENCE_NAME_VARIANT",
            DiagnosticId::MatteReferenceMissing => "MATTE_REFERENCE_MISSING",
            DiagnosticId::MatteCycle => "MATTE_CYCLE",
            DiagnosticId::CommandTargetMissing => "COMMAND_TARGET_MISSING",
            DiagnosticId::CommandInvalidValue => "COMMAND_INVALID_VALUE",
            DiagnosticId::CommandLayerLocked => "COMMAND_LAYER_LOCKED",
        }
    }

    /// False for the identifiers D-19 and D-21 propose but document 28 does not yet list.
    ///
    /// Reports can therefore separate "the catalog said to say this" from "an agent decided
    /// to say this", without a reader having to know document 28 by heart.
    pub fn in_catalog(self) -> bool {
        matches!(
            self,
            DiagnosticId::ProjectSchemaNewer
                | DiagnosticId::ProjectSchemaInvalid
                | DiagnosticId::ProjectSaveFailed
                | DiagnosticId::ProjectRecoveryAvailable
                | DiagnosticId::MediaMissing
                | DiagnosticId::EffectUnsupported
                | DiagnosticId::MediaSequenceGap
                | DiagnosticId::MediaUnsupportedFormat
                | DiagnosticId::MediaDecodeFailed
                | DiagnosticId::MatteReferenceMissing
                | DiagnosticId::MatteCycle
        )
    }
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
