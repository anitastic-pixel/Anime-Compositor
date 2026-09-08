//! Document 28's catalogue against the build, in both directions.
//!
//! Document 28 lists twenty-one diagnostic identifiers and requires that "T-07/T-08/T-12/T-14
//! explicitly assert diagnostic IDs for known failure fixtures". Every one of those tests
//! checks the identifiers it happens to meet. Nothing until now checked the catalogue as a
//! whole, which is where the two questions a reader actually has live: is there anything this
//! build says that the catalogue never agreed to, and is there anything the catalogue promises
//! that nothing in this build has ever been seen to say?
//!
//! Both answers are yes, and both are already decided rather than accidental -- D-19, D-21,
//! D-24 and D-28 registered the eight identifiers this build adds, and the six catalogue
//! entries with no code behind them belong to features that do not exist. This test is what
//! stops either list drifting without a decision, and what writes them out where the owner can
//! read them.
//!
//! Writes `verification/B-12b_diagnostic_catalogue.md`.

use anime_compositor::diagnostics::DiagnosticId;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

/// Every identifier this build can print, hand-listed rather than derived, so that adding a
/// variant to `DiagnosticId` and forgetting to answer for it fails here.
///
/// The third column is the artifact whose table shows the identifier's own sentence to a
/// reader; the check is that the artifact exists and names the identifier. An empty artifact
/// means nothing in this build produces the identifier at all, which is a claim in its own
/// right and is checked separately below.
const BUILT: &[(DiagnosticId, &str, &str)] = &[
    (
        DiagnosticId::ProjectSchemaNewer,
        "PROJECT_SCHEMA_NEWER",
        "B-09_persistence_table.md",
    ),
    (
        DiagnosticId::ProjectSchemaInvalid,
        "PROJECT_SCHEMA_INVALID",
        "B-09_persistence_table.md",
    ),
    (
        DiagnosticId::ProjectSaveFailed,
        "PROJECT_SAVE_FAILED",
        "B-09_persistence_table.md",
    ),
    (
        DiagnosticId::ProjectRecoveryAvailable,
        "PROJECT_RECOVERY_AVAILABLE",
        "B-09_persistence_table.md",
    ),
    (
        DiagnosticId::MediaMissing,
        "MEDIA_MISSING",
        "B-08a_compose_table.md",
    ),
    (
        DiagnosticId::EffectUnsupported,
        "EFFECT_UNSUPPORTED",
        "B-07_effects_table.md",
    ),
    (
        DiagnosticId::EffectParameterInvalid,
        "EFFECT_PARAMETER_INVALID",
        "B-07_effects_table.md",
    ),
    (
        DiagnosticId::ProjectFeatureUnsupported,
        "PROJECT_FEATURE_UNSUPPORTED",
        "",
    ),
    (
        DiagnosticId::MediaSequenceGap,
        "MEDIA_SEQUENCE_GAP",
        "B-03_import_table.md",
    ),
    (
        DiagnosticId::MediaUnsupportedFormat,
        "MEDIA_UNSUPPORTED_FORMAT",
        "B-03_import_table.md",
    ),
    (
        DiagnosticId::MediaDecodeFailed,
        "MEDIA_DECODE_FAILED",
        "B-03_import_table.md",
    ),
    (
        DiagnosticId::MediaSequenceDimensionMismatch,
        "MEDIA_SEQUENCE_DIMENSION_MISMATCH",
        "B-03_import_table.md",
    ),
    (
        DiagnosticId::MediaSequenceDuplicateNumber,
        "MEDIA_SEQUENCE_DUPLICATE_NUMBER",
        "B-03_import_table.md",
    ),
    (
        DiagnosticId::MediaSequenceUnnumbered,
        "MEDIA_SEQUENCE_UNNUMBERED",
        "B-03_import_table.md",
    ),
    (
        DiagnosticId::MediaSequenceNameVariant,
        "MEDIA_SEQUENCE_NAME_VARIANT",
        "B-03_import_table.md",
    ),
    (
        DiagnosticId::MatteReferenceMissing,
        "MATTE_REFERENCE_MISSING",
        "B-05_model_table.md",
    ),
    (
        DiagnosticId::MatteCycle,
        "MATTE_CYCLE",
        "B-05_model_table.md",
    ),
    (
        DiagnosticId::MaskInvalidOutline,
        "MASK_INVALID_OUTLINE",
        "B-06_mask_table.md",
    ),
    (
        DiagnosticId::CommandTargetMissing,
        "COMMAND_TARGET_MISSING",
        "B-05_model_table.md",
    ),
    (
        DiagnosticId::CommandInvalidValue,
        "COMMAND_INVALID_VALUE",
        "B-05_model_table.md",
    ),
    (
        DiagnosticId::CommandLayerLocked,
        "COMMAND_LAYER_LOCKED",
        "B-05_model_table.md",
    ),
    (
        DiagnosticId::ExportWriteFailed,
        "EXPORT_WRITE_FAILED",
        "T-08_export_table.md",
    ),
    (
        DiagnosticId::ExportCancelled,
        "EXPORT_CANCELLED",
        "T-08_export_table.md",
    ),
    (
        DiagnosticId::ExportBlockedMissingMedia,
        "EXPORT_BLOCKED_MISSING_MEDIA",
        "T-08_export_table.md",
    ),
];

/// The catalogue entries this build has no variant for, and why not. Hand-written: each one is
/// a claim about scope that a reader can check against the requirement it names.
const NOT_BUILT: &[(&str, &str)] = &[
    (
        "EXPRESSION_CYCLE",
        "R-13, expressions, is G2 work and no expression evaluator exists.",
    ),
    (
        "EXPRESSION_TIMEOUT",
        "R-13, expressions, is G2 work and no expression evaluator exists.",
    ),
    (
        "GPU_BACKEND_FAILED",
        "There is no GPU path; every frame is composited on the processor.",
    ),
    (
        "GPU_OUT_OF_MEMORY",
        "There is no GPU path; every frame is composited on the processor.",
    ),
    (
        "INVALID_PATH",
        "Paths reach this build through Windows file dialogs and are read, not normalised; \
         a path that cannot be read is reported by the identifier for what failed to read it.",
    ),
    (
        "DEPENDENCY_LICENSE_UNRESOLVED",
        "A distribution-time check rather than a running one: tools/archive_licenses.py and \
         docs/DEPENDENCIES.md flag the unresolved entries and CI blocks on them.",
    ),
];

/// The identifiers the catalogue lists, read out of document 28's own table.
fn catalogue() -> BTreeSet<String> {
    let text = std::fs::read_to_string(repo("Markdown/28_Error_Diagnostics_Catalog.md"))
        .expect("read document 28");
    let core = text
        .split("## Core IDs")
        .nth(1)
        .expect("document 28 has a Core IDs section")
        .split("\n## ")
        .next()
        .expect("the section ends");
    core.lines()
        .filter_map(|line| line.strip_prefix("| "))
        .filter_map(|line| line.split(" |").next())
        .map(str::trim)
        // `ID` is the table's own header cell, not an identifier.
        .filter(|id| {
            id.contains('_')
                && id
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c == '_' || c.is_ascii_digit())
        })
        .map(str::to_string)
        .collect()
}

struct Report {
    rows: Vec<(String, String, String)>,
}

impl Report {
    fn check(&mut self, check: &str, expected: impl ToString, actual: impl ToString) {
        self.rows
            .push((check.to_string(), expected.to_string(), actual.to_string()));
    }
}

#[test]
fn every_identifier_is_either_shown_to_somebody_or_recorded_as_unbuilt() {
    let mut report = Report { rows: Vec::new() };
    let listed = catalogue();

    report.check(
        "document 28's catalogue is read from the document, not from a copy of it",
        "21 identifiers",
        format!("{} identifiers", listed.len()),
    );

    // Direction one: everything this build can say.
    for (id, spelling, artifact) in BUILT {
        report.check(
            &format!("{spelling}: the enum spells it the way the catalogue does"),
            spelling,
            id.as_str(),
        );
        report.check(
            &format!("{spelling}: says truthfully whether document 28 lists it"),
            listed.contains(*spelling),
            id.in_catalog(),
        );
        report.check(
            &format!("{spelling}: a table somewhere shows a person this sentence"),
            match artifact.is_empty() {
                true => "nothing produces it, so nothing shows it".to_string(),
                false => format!("named in {artifact}"),
            },
            match artifact.is_empty() {
                true => "nothing produces it, so nothing shows it".to_string(),
                false => {
                    let path = repo("verification").join(artifact);
                    match std::fs::read_to_string(&path) {
                        Err(e) => format!("{artifact}: {e}"),
                        Ok(text) => match text.contains(*spelling) {
                            true => format!("named in {artifact}"),
                            false => format!("{artifact} does not name it"),
                        },
                    }
                }
            },
        );
    }

    // The one identifier that exists and is never raised. D-24 registered it, B-06 took its
    // last use away, and export still treats it as grounds for marking fidelity incomplete --
    // so it is kept deliberately, and this is the check that keeps the claim honest.
    let sources: Vec<PathBuf> = ["src", "app/src"]
        .iter()
        .flat_map(|dir| {
            std::fs::read_dir(repo(dir))
                .expect("read a source directory")
                .filter_map(|e| e.ok().map(|e| e.path()))
        })
        .filter(|p| p.extension().is_some_and(|x| x == "rs"))
        .collect();
    let constructs: Vec<String> = sources
        .iter()
        .filter(|p| !p.ends_with("diagnostics.rs"))
        .filter(|p| {
            std::fs::read_to_string(p)
                .unwrap_or_default()
                .contains("Diagnostic::new(DiagnosticId::ProjectFeatureUnsupported")
        })
        .map(|p| p.display().to_string())
        .collect();
    report.check(
        "PROJECT_FEATURE_UNSUPPORTED is kept but raised by nothing",
        "no source file raises it",
        match constructs.is_empty() {
            true => "no source file raises it".to_string(),
            false => constructs.join(", "),
        },
    );

    // Direction two: everything the catalogue promises.
    let built: BTreeSet<String> = BUILT.iter().map(|(_, s, _)| s.to_string()).collect();
    let unbuilt: BTreeSet<String> = NOT_BUILT.iter().map(|(s, _)| s.to_string()).collect();
    report.check(
        "every catalogue entry is either built or written down as not built",
        "none unaccounted for",
        {
            let missing: Vec<&String> = listed
                .iter()
                .filter(|id| !built.contains(*id) && !unbuilt.contains(*id))
                .collect();
            match missing.is_empty() {
                true => "none unaccounted for".to_string(),
                false => format!("unaccounted for: {missing:?}"),
            }
        },
    );
    report.check(
        "and nothing is written down as not built that the build actually has",
        "none",
        {
            let both: Vec<&String> = unbuilt.iter().filter(|id| built.contains(*id)).collect();
            match both.is_empty() {
                true => "none".to_string(),
                false => format!("both: {both:?}"),
            }
        },
    );
    report.check(
        "and every entry written down as not built is one the catalogue actually lists",
        "none invented",
        {
            let stray: Vec<&String> = unbuilt.iter().filter(|id| !listed.contains(*id)).collect();
            match stray.is_empty() {
                true => "none invented".to_string(),
                false => format!("not in document 28: {stray:?}"),
            }
        },
    );

    write_artifact(&report, &listed, &built);
    let failed: Vec<&(String, String, String)> =
        report.rows.iter().filter(|(_, e, a)| e != a).collect();
    assert!(failed.is_empty(), "catalogue conformance: {failed:#?}");
}

fn write_artifact(report: &Report, listed: &BTreeSet<String>, built: &BTreeSet<String>) {
    let passed = report.rows.iter().filter(|(_, e, a)| e == a).count();
    let added: Vec<&String> = built.iter().filter(|id| !listed.contains(*id)).collect();

    let mut out = String::from("# Document 28's catalogue against the build\n\n");
    out.push_str(
        "Generated by `tests/b12b_diagnostic_catalogue.rs`. Document 28 is the catalogue of \
         things this program is allowed to say when something is wrong. This page is the two \
         questions a catalogue is for: does the build say anything that is not in it, and does \
         it promise anything the build has never been seen to say.\n\n",
    );

    out.push_str("## What the build can say\n\n");
    out.push_str(
        "One row per identifier this build can print, and the table where a person can read the \
         actual sentence rather than the identifier. Nothing here is a new test of the words: \
         each of those tables already checks its own, and this is the check that none of them \
         has quietly stopped covering one.\n\n",
    );
    out.push_str("| Identifier | In document 28 | Where its words are shown |\n|---|---|---|\n");
    for (_, spelling, artifact) in BUILT {
        out.push_str(&format!(
            "| `{}` | {} | {} |\n",
            spelling,
            match listed.contains(*spelling) {
                true => "yes",
                false => "**no — added by a decision**",
            },
            match artifact.is_empty() {
                true => "**nothing raises it**".to_string(),
                false => format!("`verification/{artifact}`"),
            }
        ));
    }

    out.push_str("\n## What the catalogue promises and the build does not have\n\n");
    out.push_str("| Identifier | Why there is no code for it |\n|---|---|\n");
    for (spelling, why) in NOT_BUILT {
        out.push_str(&format!("| `{spelling}` | {why} |\n"));
    }

    out.push_str("\n## What the build says that the catalogue does not list\n\n");
    out.push_str(&format!(
        "{} identifiers, every one of them registered as a decision in \
         `Markdown/14_Decisions_Risks.md` rather than invented at a keyboard: D-19 for the four \
         import ones, D-21 for the three command ones, D-28 for the blocked export. The reason \
         they exist at all is the rule in document 28 itself — a library error string is not a \
         user-facing identifier — so the alternative to naming a new condition was reusing an \
         identifier that means something else, which is worse for exactly the person a \
         catalogue is written for.\n\n",
        added.len()
    ));
    for id in &added {
        out.push_str(&format!("- `{id}`\n"));
    }

    out.push_str("\n## The checks\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
    for (check, expected, actual) in &report.rows {
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            check,
            expected,
            actual,
            if expected == actual { "pass" } else { "FAIL" }
        ));
    }
    out.push_str(&format!(
        "\n**{} of {} checks pass.**\n",
        passed,
        report.rows.len()
    ));

    out.push_str(
        "\n## What this cannot cover\n\n- **Whether a sentence is any good.** This page checks \
         that each identifier reaches a table where its words are written out; whether those \
         words tell somebody what to do next is a judgement, and document 28's rule against \
         \"marketing-style vague messages\" is not something a test can enforce.\n- **Whether \
         the severity is right.** The catalogue's severity column is not compared against the \
         code. Two identifiers deliberately carry one severity in a command and another in a \
         file — `MASK_INVALID_OUTLINE` by D-43 and `EFFECT_PARAMETER_INVALID` by D-46 — so a \
         single-value comparison would report a defect that is a decision.\n- **Whether every \
         condition has an identifier.** This is a check of two lists against each other. A \
         failure this build meets silently, with no diagnostic at all, is invisible to both \
         lists and to this page.\n",
    );

    std::fs::write(repo("verification/B-12b_diagnostic_catalogue.md"), out)
        .expect("write the artifact");
}
