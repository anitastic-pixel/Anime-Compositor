//! B-04b: rate-limiting repeated frame-level warnings, per document 28.
//!
//! Writes `verification/B-04b_frame_log_table.md`.
//!
//! # Where the expected values come from
//!
//! Document 28's logging section is the whole requirement: "Repeated frame-level warnings
//! should be rate-limited while retaining counts/ranges."
//!
//! The reference-shot rows use a fact this project already established by hand in B-04. The
//! fixture README gives layer 3's cadence as twelve drawings on 2s, and
//! `Fixtures/reference_shot/exposure_sheet.json` records the deliberate defect as layer 3,
//! drawing 7. Drawing 7 of a twelve-drawing cycle held for two frames is therefore exposed on
//! composition frames where `(frame / 2) % 12 == 7`, which over 240 frames is
//!
//! ```text
//! 14, 15, 38, 39, 62, 63, 86, 87, 110, 111, 134, 135, 158, 159, 182, 183, 206, 207, 230, 231
//! ```
//!
//! — twenty frames in ten runs of two. That list is written as a literal in
//! `tests/b04_exposure.rs` as well, where the 240-row table checks it against the sheet. It is
//! the arithmetic above, not a capture of any run (ADR-009).
//!
//! The `frame_ranges` rows are worked out from the function's stated rule — consecutive
//! integers collapse into one run, everything is sorted and de-duplicated first — and each
//! one's working is in the comment beside it.
//!
//! # What is deliberately not tested here
//!
//! Nothing in this build drives a frame loop in production: no application code walks a
//! composition's frames and logs what each one raised. That loop is B-08's. The test drives one
//! itself, exactly as B-04's table does, which is enough to show the collector behaves as
//! document 28 requires but is not the same as showing it is installed. See `HANDOFF.md`.

use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::diagnostics::{frame_ranges, Diagnostic, DiagnosticId, FrameLog, Severity};
use anime_compositor::media::{import_sequence, SequenceAsset};
use anime_compositor::time::{resolve, ExposureMap, LayerTiming};

const FRAMES: i32 = 240;

/// The twenty frames of the reference shot that expose the missing drawing, derived above.
const AFFECTED: [i32; 20] = [
    14, 15, 38, 39, 62, 63, 86, 87, 110, 111, 134, 135, 158, 159, 182, 183, 206, 207, 230, 231,
];

// ---------------------------------------------------------------------------------------
// Reporting
// ---------------------------------------------------------------------------------------

struct Row {
    check: String,
    expected: String,
    actual: String,
}

impl Row {
    fn pass(&self) -> bool {
        self.expected == self.actual
    }
}

#[derive(Default)]
struct Report {
    rows: Vec<Row>,
}

impl Report {
    fn check(&mut self, check: &str, expected: impl ToString, actual: impl ToString) {
        self.rows.push(Row {
            check: check.to_string(),
            expected: expected.to_string(),
            actual: actual.to_string(),
        });
    }
}

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn layer_asset(name: &str) -> SequenceAsset {
    let dir = repo("Fixtures/reference_shot").join(name);
    let mut files: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
        .map(|e| e.unwrap().path())
        .collect();
    files.sort();
    import_sequence(&files)
        .asset
        .unwrap_or_else(|| panic!("no asset for {name}"))
}

/// A warning with no meaning beyond being distinguishable, for the grouping rows.
fn warn(id: DiagnosticId, frame: i32) -> Diagnostic {
    Diagnostic::new(
        id,
        Severity::Warning,
        format!("Frame {frame} is unhappy."),
        format!("Detail for frame {frame}."),
    )
}

/// Count the frames a range string names, by expanding it again.
///
/// This exists for one row: that rate-limiting loses no frame. Expanding the summary's own text
/// is the check a reader can repeat by eye on the table.
fn frames_named(ranges: &str) -> usize {
    ranges
        .split(", ")
        .map(|part| match part.split_once(" to ") {
            Some((a, b)) => match (a.parse::<i32>(), b.parse::<i32>()) {
                (Ok(a), Ok(b)) if b >= a => (b - a + 1) as usize,
                _ => 0,
            },
            None => usize::from(part.parse::<i32>().is_ok()),
        })
        .sum()
}

/// A record's message, or the empty string when the log is shorter than the row expects.
///
/// Every row below that reaches into the log by position goes through this. A build that emits
/// too few records must fail the row that says how many there should be, not take the whole
/// test down with an index panic before the artifact is written.
fn message(records: &[Diagnostic], index: usize) -> String {
    records
        .get(index)
        .map(|d| d.message.clone())
        .unwrap_or_default()
}

fn id_of(records: &[Diagnostic], index: usize) -> String {
    records
        .get(index)
        .map(|d| d.id.to_string())
        .unwrap_or_default()
}

#[test]
fn b04b_frame_log_table() {
    let mut report = Report::default();

    // -- The reference shot, walked frame by frame -------------------------------------------
    //
    // Layer 3 on 2s, twelve drawings, one of which is not on disk. This is the condition
    // document 28's rule exists for: the same warning, once per affected frame, twenty times in
    // one 240-frame shot.
    let asset3 = layer_asset("layer3");
    let drawings: Vec<u32> = (0..20).flat_map(|_| 0..12).collect();
    let map3 = ExposureMap::on_twos_style(&drawings, 2).expect("layer 3 is twelve drawings on 2s");
    let timing = LayerTiming {
        in_frame: 0,
        out_frame: FRAMES,
        source_offset_frames: 0,
    };

    let mut unlimited: Vec<i32> = Vec::new();
    let mut log = FrameLog::new(3);
    let mut first_raised: Option<Diagnostic> = None;
    for frame in 0..FRAMES {
        if let Err(d) = resolve(&timing, &map3, &asset3, frame) {
            let number = map3
                .drawing_at(frame)
                .expect("an exposed frame has a drawing");
            unlimited.push(frame);
            if first_raised.is_none() {
                first_raised = Some(d.clone());
            }
            log.record(frame, format!("layer3 drawing {number}"), d);
        }
    }
    let records = log.finish();

    report.check(
        "reference shot: frames that raise the warning at all",
        AFFECTED
            .iter()
            .map(i32::to_string)
            .collect::<Vec<_>>()
            .join(","),
        unlimited
            .iter()
            .map(i32::to_string)
            .collect::<Vec<_>>()
            .join(","),
    );
    report.check(
        "reference shot: how many times it is raised",
        20,
        unlimited.len(),
    );
    // Limit 3, so three logged in full plus one summary.
    report.check(
        "reference shot: records in the rate-limited log",
        4,
        records.len(),
    );
    report.check(
        "reference shot: the three logged in full are the first three occurrences",
        "Frame 14 exposes drawing 7 / Frame 15 exposes drawing 7 / Frame 38 exposes drawing 7",
        records
            .iter()
            .take(3)
            .map(|d| {
                d.message
                    .split(" of ")
                    .next()
                    .unwrap_or_default()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join(" / "),
    );
    report.check(
        "reference shot: the first logged record is the diagnostic itself, unaltered",
        true,
        records.first() == first_raised.as_ref(),
    );

    let empty = Diagnostic::new(
        DiagnosticId::MediaSequenceGap,
        Severity::Info,
        "no summary was appended",
        "no summary was appended",
    );
    // The last record is the summary when there is one. When there is not, the rows below read
    // against this placeholder and fail by name rather than by panic.
    let summary = match records.len() > 3 {
        true => records.last().unwrap(),
        false => &empty,
    };
    report.check(
        "reference shot: the summary states the total",
        "layer3 drawing 7: 20 frames affected. The first 3 are logged in full above.",
        &summary.message,
    );
    report.check(
        "reference shot: the summary states the ranges and how many were suppressed",
        "Frames 14 to 15, 38 to 39, 62 to 63, 86 to 87, 110 to 111, 134 to 135, 158 to 159, \
         182 to 183, 206 to 207, 230 to 231. 17 further identical warnings were not logged \
         individually.",
        &summary.detail,
    );
    report.check(
        "reference shot: the ranges name every affected frame and no others",
        20,
        frames_named(
            summary
                .detail
                .trim_start_matches("Frames ")
                .split_once(". ")
                .map(|(ranges, _)| ranges)
                .unwrap_or_default(),
        ),
    );
    report.check(
        "reference shot: the summary keeps the identifier it summarises",
        "MEDIA_SEQUENCE_GAP",
        summary.id.to_string(),
    );
    report.check(
        "reference shot: the summary keeps the severity",
        "WARNING",
        summary.severity.to_string(),
    );
    report.check(
        "reference shot: the summary keeps the remediation of the first occurrence",
        "Add the missing file and relink the sequence, or change the exposure to a drawing that \
         exists.",
        summary.remediation.clone().unwrap_or_default(),
    );

    // -- The limit itself ---------------------------------------------------------------------
    //
    // Three rows around the boundary. A group at the limit has nothing suppressed and must get
    // no summary; one past it must get one that says exactly one was suppressed.
    let mut at_limit = FrameLog::new(3);
    for frame in 0..3 {
        at_limit.record(frame, "same", warn(DiagnosticId::MediaMissing, frame));
    }
    report.check(
        "a group at the limit is logged in full and summarised not at all",
        3,
        at_limit.finish().len(),
    );

    let mut past_limit = FrameLog::new(3);
    for frame in 0..4 {
        past_limit.record(frame, "same", warn(DiagnosticId::MediaMissing, frame));
    }
    let past = past_limit.finish();
    report.check(
        "one occurrence past the limit produces one summary",
        4,
        past.len(),
    );
    report.check(
        "and the summary says one was suppressed, over frames 0 to 3",
        "Frames 0 to 3. 1 further identical warnings were not logged individually.",
        past.get(3).map(|d| d.detail.clone()).unwrap_or_default(),
    );

    let mut none_shown = FrameLog::new(0);
    for frame in 0..5 {
        none_shown.record(frame, "same", warn(DiagnosticId::MediaMissing, frame));
    }
    let none = none_shown.finish();
    report.check(
        "a limit of zero logs nothing in full and keeps the count and ranges anyway",
        "1 / Frames 0 to 4. 5 further identical warnings were not logged individually.",
        format!(
            "{} / {}",
            none.len(),
            none.first().map(|d| d.detail.clone()).unwrap_or_default()
        ),
    );

    // -- Grouping -----------------------------------------------------------------------------
    //
    // Two conditions in one shot must not be collapsed into one. The key is the identifier and
    // the subject together: the message cannot be the key, because a frame-level message names
    // its own frame and so no two are ever equal.
    let mut two_subjects = FrameLog::new(1);
    for frame in 0..3 {
        two_subjects.record(
            frame,
            "layer3 drawing 7",
            warn(DiagnosticId::MediaMissing, frame),
        );
        two_subjects.record(
            frame,
            "layer4 drawing 2",
            warn(DiagnosticId::MediaMissing, frame),
        );
    }
    let subjects = two_subjects.finish();
    report.check(
        "two subjects under one identifier stay apart: two logged, two summaries",
        4,
        subjects.len(),
    );
    report.check(
        "and each summary counts only its own subject's frames",
        "layer3 drawing 7: 3 frames affected. The first 1 are logged in full above. / \
         layer4 drawing 2: 3 frames affected. The first 1 are logged in full above.",
        format!("{} / {}", message(&subjects, 2), message(&subjects, 3)),
    );

    let mut two_ids = FrameLog::new(1);
    for frame in 0..3 {
        two_ids.record(frame, "same", warn(DiagnosticId::MediaMissing, frame));
        two_ids.record(frame, "same", warn(DiagnosticId::MediaDecodeFailed, frame));
    }
    let ids = two_ids.finish();
    report.check(
        "two identifiers under one subject stay apart: two logged, two summaries",
        4,
        ids.len(),
    );
    report.check(
        "and the summaries carry the two identifiers, not one twice",
        "MEDIA_MISSING / MEDIA_DECODE_FAILED",
        format!("{} / {}", id_of(&ids, 2), id_of(&ids, 3)),
    );

    // Interleaved recording: the log is in the order the frames were walked, and the summaries
    // follow in the order the groups first appeared. A reader scrolling the log sees the frames
    // in sequence, not sorted into buckets.
    report.check(
        "records stay in the order they were recorded, summaries after them",
        "MEDIA_MISSING f0 / MEDIA_DECODE_FAILED f0 / MEDIA_MISSING sum / MEDIA_DECODE_FAILED sum",
        ids.iter()
            .enumerate()
            .map(|(i, d)| {
                if i < 2 {
                    format!("{} f0", d.id)
                } else {
                    format!("{} sum", d.id)
                }
            })
            .collect::<Vec<_>>()
            .join(" / "),
    );

    let mut mixed = FrameLog::new(1);
    for frame in 0..4 {
        mixed.record(frame, "noisy", warn(DiagnosticId::MediaMissing, frame));
    }
    mixed.record(9, "quiet", warn(DiagnosticId::MediaMissing, 9));
    let mixed = mixed.finish();
    report.check(
        "a quiet group beside a noisy one is not summarised",
        "3 / noisy: 4 frames affected. The first 1 are logged in full above.",
        format!("{} / {}", mixed.len(), message(&mixed, 2)),
    );

    // -- Ranges -------------------------------------------------------------------------------
    //
    // Each expected string below is the function's stated rule applied by hand.
    report.check(
        "ranges: two runs and a singleton",
        "14 to 15, 38 to 39, 41",
        frame_ranges(&[14, 15, 38, 39, 41]),
    );
    report.check("ranges: one frame", "7", frame_ranges(&[7]));
    report.check(
        "ranges: one unbroken run",
        "0 to 4",
        frame_ranges(&[0, 1, 2, 3, 4]),
    );
    // Sorted and de-duplicated first: 9,3,4,3,5 becomes 3,4,5,9.
    report.check(
        "ranges: out of order and repeated input still reads as one honest set",
        "3 to 5, 9",
        frame_ranges(&[9, 3, 4, 3, 5]),
    );
    // The reason the separator is a word: a dash here would print "-2-1".
    report.check(
        "ranges: a run that crosses zero is still legible",
        "-2 to 1",
        frame_ranges(&[-2, -1, 0, 1]),
    );
    report.check(
        "ranges: a gap of one frame is a gap, not a run",
        "1, 3",
        frame_ranges(&[1, 3]),
    );
    report.check("ranges: nothing", "", frame_ranges(&[]));

    // -- Write the artifact -------------------------------------------------------------------
    write_report(&report);
    let failed: Vec<&Row> = report.rows.iter().filter(|r| !r.pass()).collect();
    assert!(
        failed.is_empty(),
        "{} checks failed\n{}",
        failed.len(),
        failed
            .iter()
            .map(|r| format!("{}: expected {} got {}", r.check, r.expected, r.actual))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn write_report(report: &Report) {
    let passed = report.rows.iter().filter(|r| r.pass()).count();
    let mut out = String::new();
    out.push_str(&format!(
        "# B-04b — rate-limiting repeated frame-level warnings\n\n\
         **{passed} of {} checks passed.**\n\n\
         Generated by `tests/b04b_frame_log.rs`. Covers the logging rule of document 28: \
         \"Repeated frame-level warnings should be rate-limited while retaining counts/ranges.\"\n\n\
         ## What to look at\n\n\
         The reference shot has one drawing deliberately missing from layer 3. That layer holds \
         each drawing for two frames, so over the shot's 240 frames the missing one is asked \
         for twenty times, and every one of those raises the same warning. Twenty copies of one \
         message is how a real problem gets buried.\n\n\
         The first rows walk the shot frame by frame and show what the log looks like after \
         rate-limiting: **the first three occurrences in full, then one summary saying twenty \
         frames were affected and naming which ones** — 14 to 15, 38 to 39, and so on through \
         230 to 231. Nothing is thrown away. The summary carries the same identifier, the same \
         severity and the same suggested fix as the warning it stands for, so it is still \
         actionable on its own.\n\n\
         Two rows are worth reading beyond that:\n\n\
         - **The ranges name every affected frame and no others.** Twenty frames go in, twenty \
         come back out of the summary's own text. A rate limiter that quietly lost the \
         suppressed frames would be worse than no rate limiter, because the log would then \
         under-report a real defect.\n\
         - **Two different problems in one shot stay apart.** The grouping rows record two \
         subjects, and two identifiers, into one log and require four records out, not two. If \
         everything collapsed into one summary, a missing drawing and a failed decode would \
         become one indistinguishable line.\n\n\
         ## What this does not cover\n\n\
         Nothing in this build walks a composition's frames in production — that loop is B-08's \
         work, and until it exists this collector is a facility with no installed caller. The \
         test drives the walk itself, over the real reference shot, which shows the behaviour \
         is right but not that it is switched on.\n\n\
         The number three, and the choice to log the first few in full rather than only a \
         summary, are not in document 28. They are registered as **D-25** in \
         `Markdown/14_Decisions_Risks.md` and are marked PROVISIONAL, awaiting a ruling.\n\n\
         ## Checks\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        report.rows.len(),
    ));
    for r in &report.rows {
        out.push_str(&format!(
            "| {} | `{}` | `{}` | {} |\n",
            r.check,
            r.expected,
            r.actual,
            if r.pass() { "pass" } else { "**FAIL**" }
        ));
    }
    fs::write(repo("verification/B-04b_frame_log_table.md"), out).expect("write report");
}
