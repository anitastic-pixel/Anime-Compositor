//! T-01 / R-01 / B-03: import numeric PNG sequences with gaps, Unicode paths and mismatched
//! dimensions; verify grouping and diagnostics.
//!
//! Writes `verification/B-03_import_table.md`, which is what the owner reviews. The table holds
//! the grouping results and the verbatim text of every diagnostic the import produced, because
//! document 15 asks B-03 for "the diagnostic text the user would see for the missing frame" and a
//! description of that text is not that text.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::diagnostics::{Diagnostic, DiagnosticId};
use anime_compositor::media::{import_sequence, ImportResult, SequenceAsset};
use anime_compositor::ImageBuffer;

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

struct Report {
    rows: Vec<Row>,
    diagnostics: Vec<(String, Diagnostic)>,
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

fn layer_dir(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("Fixtures/reference_shot")
        .join(name)
}

/// Everything in a directory, sorted, as if the user had selected all of it in a file dialog.
fn select_all(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
        .map(|e| e.unwrap().path())
        .filter(|p| p.is_file())
        .collect();
    files.sort();
    files
}

/// Minimal RGBA8 PNG. Used only for the synthetic cases below, never for anything in `Fixtures/`.
fn write_png(path: &Path, w: u32, h: u32, depth: png::BitDepth) {
    let file = fs::File::create(path).unwrap();
    let mut encoder = png::Encoder::new(std::io::BufWriter::new(file), w, h);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(depth);
    let bytes_per_channel = if depth == png::BitDepth::Sixteen {
        2
    } else {
        1
    };
    let data = vec![0u8; (w * h * 4) as usize * bytes_per_channel];
    encoder
        .write_header()
        .unwrap()
        .write_image_data(&data)
        .unwrap();
}

/// A scratch directory for the synthetic cases.
///
/// Document 12 makes `Fixtures/` read-only to implementation work. Mismatched dimensions and a
/// 16-bit file are cases T-01 needs that the reference shot deliberately does not contain, so
/// they are generated outside the fixture tree. Adding them to `Fixtures/` would have been an
/// edit to the fixture set dressed up as a test.
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join("anime_compositor_b03").join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn record(report: &mut Report, case: &str, result: &ImportResult) {
    for d in &result.diagnostics {
        report.diagnostics.push((case.to_string(), d.clone()));
    }
}

/// The name of the file that entered the frame map under a drawing number, or why it did not.
///
/// Which file wins when two claim one number is part of R-01's contract, not an internal
/// detail: it decides what the user sees on that frame.
/// A drawing's pixels, with a row recording whether it decoded at all.
///
/// The pixel rows below read specific drawings of specific layers. A build that renumbers or
/// drops one of them should fail a named row and still leave the owner a table to read, rather
/// than aborting on an `expect` and leaving a stack trace.
fn pixels(
    report: &mut Report,
    case: &str,
    asset: &SequenceAsset,
    number: u32,
) -> Option<ImageBuffer> {
    let got = asset.decode(number);
    report.check(
        &format!("{case}: drawing {number} decodes"),
        "decoded",
        match &got {
            Ok(_) => "decoded".to_string(),
            Err(d) => format!("failed: {}", d.id),
        },
    );
    got.ok()
}

fn file_name(asset: &SequenceAsset, number: u32) -> String {
    match asset.frames().get(&number) {
        Some(path) => path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned(),
        None => format!("no file was imported as drawing {number}"),
    }
}

fn asset_of(result: &ImportResult) -> &SequenceAsset {
    result.asset.as_ref().expect("import produced no asset")
}

/// What one layer of the reference shot is expected to import as.
struct Expect {
    layer: &'static str,
    files: usize,
    pattern: &'static str,
    range: (u32, u32),
    /// Comma-separated drawing numbers, or "none".
    missing: &'static str,
    /// Files carrying a clear number under a name the pattern does not generate.
    variants: usize,
}

fn check_layer(report: &mut Report, e: Expect) {
    let Expect {
        layer,
        files,
        pattern,
        range,
        missing,
        variants,
    } = e;
    let selection = select_all(&layer_dir(layer));
    let result = import_sequence(&selection);
    let asset = asset_of(&result);

    report.check(&format!("{layer}: files selected"), files, selection.len());
    report.check(
        &format!("{layer}: frames grouped"),
        files,
        asset.frames().len(),
    );
    report.check(
        &format!("{layer}: inferred pattern"),
        pattern,
        asset.pattern(),
    );
    report.check(
        &format!("{layer}: drawing range"),
        format!("{}-{}", range.0, range.1),
        match asset.range() {
            Some((lo, hi)) => format!("{lo}-{hi}"),
            None => "none".to_string(),
        },
    );
    report.check(
        &format!("{layer}: missing drawings"),
        missing,
        if asset.missing().is_empty() {
            "none".to_string()
        } else {
            asset
                .missing()
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(",")
        },
    );
    report.check(
        &format!("{layer}: names not matching the pattern"),
        variants,
        asset.name_variants().len(),
    );
    report.check(
        &format!("{layer}: gap diagnostic raised"),
        missing != "none",
        result.has(DiagnosticId::MediaSequenceGap),
    );
    record(report, layer, &result);
}

#[test]
fn b03_import_table() {
    let mut report = Report {
        rows: Vec::new(),
        diagnostics: Vec::new(),
    };

    // -- The reference shot ------------------------------------------------------------------
    //
    // Layer 2 is the trap. Drawing 13 exists as `layer2_桜_013.png`, so the pattern
    // `layer2_%03d.png` does not generate it. Layer 2 must report ZERO gaps: the drawing is
    // there, on disk, and the user can see it. Layer 3 must report exactly ONE, at 7.
    for e in [
        Expect {
            layer: "layer1",
            files: 1,
            pattern: "layer1_%03d.png",
            range: (0, 0),
            missing: "none",
            variants: 0,
        },
        Expect {
            layer: "layer2",
            files: 24,
            pattern: "layer2_%03d.png",
            range: (0, 23),
            missing: "none",
            variants: 1,
        },
        Expect {
            layer: "layer3",
            files: 11,
            pattern: "layer3_%03d.png",
            range: (0, 11),
            missing: "7",
            variants: 0,
        },
        Expect {
            layer: "layer4",
            files: 20,
            pattern: "layer4_%03d.png",
            range: (0, 19),
            missing: "none",
            variants: 0,
        },
    ] {
        check_layer(&mut report, e);
    }

    // The Unicode name must survive as itself, not as a normalised or transliterated form.
    let layer2 = import_sequence(&select_all(&layer_dir("layer2")));
    let asset2 = asset_of(&layer2);
    report.check(
        "layer2: drawing 13 maps to its Japanese filename",
        "layer2_桜_013.png",
        file_name(asset2, 13),
    );
    report.check(
        "layer2: drawing 13 decodes",
        "1920x1080",
        match asset2.decode(13) {
            Ok(b) => format!("{}x{}", b.width(), b.height()),
            Err(d) => format!("failed: {}", d.id),
        },
    );

    // -- The missing drawing ------------------------------------------------------------------
    let layer3 = import_sequence(&select_all(&layer_dir("layer3")));
    let asset3 = asset_of(&layer3);
    report.check(
        "layer3: drawing 7 refuses to decode",
        DiagnosticId::MediaSequenceGap.to_string(),
        match asset3.decode(7) {
            Ok(_) => "decoded something".to_string(),
            Err(d) => d.id.to_string(),
        },
    );
    // Document 28: "do not substitute adjacent frame." Drawings 6 and 8 both exist, so an
    // implementation that quietly reached for a neighbour would still return a valid image here.
    report.check(
        "layer3: drawings 6 and 8 exist, so substitution was possible and did not happen",
        "6 and 8 present, 7 absent",
        format!(
            "{} and {} present, 7 {}",
            if asset3.frames().contains_key(&6) {
                "6"
            } else {
                "-"
            },
            if asset3.frames().contains_key(&8) {
                "8"
            } else {
                "-"
            },
            if asset3.frames().contains_key(&7) {
                "present"
            } else {
                "absent"
            }
        ),
    );

    // -- Pixel interpretation of what was imported ---------------------------------------------
    let layer1 = import_sequence(&select_all(&layer_dir("layer1")));
    let bg = pixels(&mut report, "layer1", asset_of(&layer1), 0);
    report.check(
        "layer1: dimensions",
        "1920x1080",
        match &bg {
            Some(b) => format!("{}x{}", b.width(), b.height()),
            None => "no pixels".to_string(),
        },
    );
    report.check(
        "layer1: fully opaque, per the fixture README",
        true,
        matches!(&bg, Some(b) if b.data().chunks_exact(4).all(|p| p[3] == 1.0)),
    );

    let hard = pixels(&mut report, "layer3", asset3, 0);
    report.check(
        "layer3: alpha is binary, per the fixture README",
        true,
        matches!(&hard, Some(b) if b
            .data()
            .chunks_exact(4)
            .all(|p| p[3] == 0.0 || p[3] == 1.0)),
    );

    let layer4 = import_sequence(&select_all(&layer_dir("layer4")));
    let half = pixels(&mut report, "layer4", asset_of(&layer4), 0);
    report.check(
        "layer4: an interior pixel is at exactly code 128, per the fixture README",
        true,
        matches!(&half, Some(b) if b.data().chunks_exact(4).any(|p| p[3] == 128.0 / 255.0)),
    );
    report.check(
        "import tags buffers as sRGB / straight, per document 21",
        "Srgb/Straight",
        match &bg {
            Some(b) => format!("{:?}/{:?}", b.color_space(), b.alpha_mode()),
            None => "no pixels".to_string(),
        },
    );

    // -- Cases the reference shot deliberately does not contain --------------------------------
    let dir = scratch("dimensions");
    write_png(&dir.join("shot_000.png"), 64, 32, png::BitDepth::Eight);
    write_png(&dir.join("shot_001.png"), 64, 32, png::BitDepth::Eight);
    write_png(&dir.join("shot_002.png"), 48, 24, png::BitDepth::Eight);
    let mixed = import_sequence(&select_all(&dir));
    report.check(
        "mismatched dimensions: diagnostic raised",
        true,
        mixed.has(DiagnosticId::MediaSequenceDimensionMismatch),
    );
    report.check(
        "mismatched dimensions: sequence takes the majority size",
        "64x32",
        format!("{}x{}", asset_of(&mixed).width(), asset_of(&mixed).height()),
    );
    report.check(
        "mismatched dimensions: the odd drawing is still imported",
        3,
        asset_of(&mixed).frames().len(),
    );
    record(&mut report, "mismatched dimensions", &mixed);

    let dir = scratch("unsupported");
    write_png(&dir.join("deep_000.png"), 8, 8, png::BitDepth::Sixteen);
    write_png(&dir.join("deep_001.png"), 8, 8, png::BitDepth::Eight);
    let deep = import_sequence(&select_all(&dir));
    report.check(
        "16-bit PNG: reported unsupported rather than truncated to 8-bit",
        true,
        deep.has(DiagnosticId::MediaUnsupportedFormat),
    );
    report.check(
        "16-bit PNG: dropped from the frame map, leaving the 8-bit file",
        1,
        asset_of(&deep).frames().len(),
    );
    record(&mut report, "unsupported bit depth", &deep);

    let dir = scratch("unnumbered");
    write_png(&dir.join("cel_000.png"), 8, 8, png::BitDepth::Eight);
    write_png(&dir.join("cel_001.png"), 8, 8, png::BitDepth::Eight);
    write_png(&dir.join("notes.png"), 8, 8, png::BitDepth::Eight);
    let unnumbered = import_sequence(&select_all(&dir));
    report.check(
        "file with no number: diagnostic raised",
        true,
        unnumbered.has(DiagnosticId::MediaSequenceUnnumbered),
    );
    report.check(
        "file with no number: excluded, the rest import",
        2,
        asset_of(&unnumbered).frames().len(),
    );
    record(&mut report, "no frame number", &unnumbered);

    let dir = scratch("duplicate");
    write_png(&dir.join("cel_007.png"), 8, 8, png::BitDepth::Eight);
    write_png(&dir.join("cel_07.png"), 8, 8, png::BitDepth::Eight);
    let duplicate = import_sequence(&select_all(&dir));
    report.check(
        "two files claiming drawing 7: diagnostic raised",
        true,
        duplicate.has(DiagnosticId::MediaSequenceDuplicateNumber),
    );
    report.check(
        "two files claiming drawing 7: one entry in the frame map",
        1,
        asset_of(&duplicate).frames().len(),
    );
    record(&mut report, "duplicate number", &duplicate);

    // Which of the two survives is not a coin toss. Import sorts the selection before grouping,
    // so the first name in sort order enters the frame map, and cel_007.png sorts before
    // cel_07.png because '0' precedes '7'. A build where the later file won would give a
    // different sequence for the same folder depending on the order the file dialog handed the
    // selection over.
    report.check(
        "two files claiming drawing 7: the first in sorted order is the one imported",
        "cel_007.png",
        file_name(asset_of(&duplicate), 7),
    );
    // The rejected file must not vote on the naming either. One imported file called
    // cel_007.png describes a three-digit pattern; letting cel_07.png vote as well makes it a
    // tie, and a tie breaks toward the smaller shape, so the sequence would be described by a
    // name that nothing in it uses.
    report.check(
        "two files claiming drawing 7: only the imported file describes the pattern",
        "cel_%03d.png",
        asset_of(&duplicate).pattern(),
    );
    // The same folder, selected in the opposite order. R-01 asks for the import of a folder,
    // not of an ordering, so the two results have to agree.
    let mut backwards = select_all(&dir);
    backwards.reverse();
    let backwards = import_sequence(&backwards);
    report.check(
        "the same two files selected in the opposite order import identically",
        "cel_007.png / cel_%03d.png",
        format!(
            "{} / {}",
            file_name(asset_of(&backwards), 7),
            asset_of(&backwards).pattern()
        ),
    );

    // A majority naming with one odd file out. The pattern is the commonest shape, not the
    // first one seen: two files are three-digit and one is two-digit, so the sequence is
    // three-digit and the odd file is reported as a name variant rather than dropped.
    let dir = scratch("pattern");
    write_png(&dir.join("cel_000.png"), 8, 8, png::BitDepth::Eight);
    write_png(&dir.join("cel_001.png"), 8, 8, png::BitDepth::Eight);
    write_png(&dir.join("cel_02.png"), 8, 8, png::BitDepth::Eight);
    let majority = import_sequence(&select_all(&dir));
    report.check(
        "two names of one shape and one of another: the pattern is the commonest",
        "cel_%03d.png",
        asset_of(&majority).pattern(),
    );
    report.check(
        "the odd name is imported anyway and reported as a variant",
        "3 files / 1 variant",
        format!(
            "{} files / {} variant",
            asset_of(&majority).frames().len(),
            asset_of(&majority).name_variants().len()
        ),
    );
    record(&mut report, "majority naming", &majority);

    // A sequence that does not start at zero, with two holes in it. Drawings below the first
    // file are not missing: nobody drew them and nothing exposes them. Only the holes inside
    // 100 to 109 are gaps, and document 28 wants the warning to name them as runs, which means
    // the run 101-103 has to stop at the drawing that exists rather than swallowing 104 and 105.
    let dir = scratch("sparse");
    for n in [100, 104, 105, 109] {
        write_png(
            &dir.join(format!("cel_{n}.png")),
            8,
            8,
            png::BitDepth::Eight,
        );
    }
    let sparse = import_sequence(&select_all(&dir));
    report.check(
        "a sequence starting at 100: drawing range",
        "100-109",
        match asset_of(&sparse).range() {
            Some((lo, hi)) => format!("{lo}-{hi}"),
            None => "none".to_string(),
        },
    );
    report.check(
        "a sequence starting at 100: the drawings below it are not missing",
        "101,102,103,106,107,108",
        asset_of(&sparse)
            .missing()
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(","),
    );
    report.check(
        "the gap warning names runs and keeps the hole between them",
        "6 drawings are missing from cel_%03d.png: 101-103, 106-108.",
        match sparse.get(DiagnosticId::MediaSequenceGap) {
            Some(d) => d.message.clone(),
            None => "no gap warning was raised".to_string(),
        },
    );
    record(&mut report, "sparse numbering", &sparse);

    // -- Severity, per document 28 -------------------------------------------------------------
    // A gap renders transparent and the render proceeds, so it is a WARNING. A file that cannot
    // be imported at all is an ERROR, because the operation the user asked for is refused.
    let layer3 = import_sequence(&select_all(&layer_dir("layer3")));
    report.check(
        "layer3's gap is a warning: the render proceeds with explicit degradation",
        "Warning",
        match layer3.get(DiagnosticId::MediaSequenceGap) {
            Some(d) => format!("{:?}", d.severity),
            None => "no gap warning was raised".to_string(),
        },
    );
    report.check(
        "the gap warning states the next safe action, per document 28",
        true,
        matches!(layer3.get(DiagnosticId::MediaSequenceGap), Some(d) if d.remediation.is_some()),
    );

    let dir = scratch("severity");
    write_png(&dir.join("cel_000.png"), 8, 8, png::BitDepth::Eight);
    write_png(&dir.join("notes.png"), 8, 8, png::BitDepth::Eight);
    let refused = import_sequence(&select_all(&dir));
    report.check(
        "a file that cannot be imported at all is an error, not a warning",
        "Error",
        match refused.get(DiagnosticId::MediaSequenceUnnumbered) {
            Some(d) => format!("{:?}", d.severity),
            None => "no diagnostic was raised".to_string(),
        },
    );

    write_artifact(&report);

    let failed: Vec<&Row> = report.rows.iter().filter(|r| !r.pass()).collect();
    assert!(
        failed.is_empty(),
        "{} of {} checks failed, first: {} expected {:?} got {:?}",
        failed.len(),
        report.rows.len(),
        failed[0].check,
        failed[0].expected,
        failed[0].actual
    );
}

fn write_artifact(report: &Report) {
    let passed = report.rows.iter().filter(|r| r.pass()).count();
    let mut out = String::new();
    out.push_str(&format!(
        "# B-03 import fixture table\n\n\
         Test T-01, requirement R-01. Produced by `tests/b03_import.rs`. \
         **{passed} of {} checks pass.**\n\n\
         Every row compares an expected value written into the test against what the import \
         actually produced. The reference shot's two deliberate defects are the point of the \
         first block: `layer3/layer3_007.png` does not exist, and drawing 13 of layer 2 lives \
         under the Japanese filename `layer2_桜_013.png`.\n\n\
         | Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        report.rows.len()
    ));
    for r in &report.rows {
        out.push_str(&format!(
            "| {} | `{}` | `{}` | {} |\n",
            r.check,
            r.expected,
            r.actual,
            if r.pass() { "PASS" } else { "**FAIL**" }
        ));
    }

    out.push_str(
        "\n## The diagnostics, exactly as a user would read them\n\n\
         Verbatim output, not a description of it. Severities and identifiers follow document 28.\n",
    );
    let mut by_case: BTreeMap<&str, Vec<&Diagnostic>> = BTreeMap::new();
    for (case, d) in &report.diagnostics {
        by_case.entry(case.as_str()).or_default().push(d);
    }
    for (case, diags) in &by_case {
        out.push_str(&format!("\n### {case}\n\n"));
        for d in diags {
            out.push_str(&format!("```\n{d}\n```\n\n"));
        }
    }
    let quiet: Vec<&str> = ["layer1", "layer2", "layer4"]
        .into_iter()
        .filter(|l| !by_case.contains_key(l))
        .collect();
    if !quiet.is_empty() {
        out.push_str(&format!(
            "\n### Silent by design\n\n{} produced no diagnostics at all. \
             For layer 2 that is the result under test: a naive importer reports a false gap at \
             drawing 13 because the pattern does not generate its filename.\n",
            quiet.join(", ")
        ));
    }

    out.push_str(
        "\n## Not run by this test\n\n\
         - Relink after a moved or renamed sequence (R-08, B-09). Import here always finds every \
           file where the selection said it was.\n\
         - Save and reopen of the Unicode filename (T-08, B-09). This test proves it survives \
           import, not that it survives a round trip through the project file.\n\
         - The exposure sheet, holds and out-of-order re-exposure (T-02, B-04). Import produces a \
           drawing-number map; nothing here maps composition frames onto it.\n\
         - Content fingerprints for cache invalidation (document 27, B-09). Deliberately not \
           computed: it would mean a full read of every file for a benefit nothing yet consumes.\n\
         - Formats other than PNG (out of G1 scope, document 04).\n\n\
         ## Four diagnostic identifiers are proposals\n\n\
         `MEDIA_SEQUENCE_GAP`, `MEDIA_UNSUPPORTED_FORMAT` and `MEDIA_DECODE_FAILED` come from \
         document 28. The other four here — dimension mismatch, duplicate number, unnumbered \
         file, name variant — do not appear in it. T-01 requires mismatched-dimension behaviour \
         and document 28 defines no identifier for it, so they are registered as **D-19** in \
         document 14 rather than quietly invented. If the owner names them differently, this \
         table's identifiers change.\n",
    );

    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("verification/B-03_import_table.md");
    fs::write(&path, out).expect("write verification artifact");
}

/// Guards the two properties the whole test rests on, independently of the import code.
///
/// If someone "fixes" the reference shot by adding `layer3_007.png` or renaming the Japanese
/// file, T-01 would start passing for the wrong reason. The fixture README says both defects are
/// required and must not be repaired; this makes that mechanical.
#[test]
fn reference_shot_defects_intact() {
    assert!(
        !layer_dir("layer3").join("layer3_007.png").exists(),
        "layer3_007.png must not exist: the gap is a required fixture defect"
    );
    assert!(
        layer_dir("layer2").join("layer2_桜_013.png").exists(),
        "layer2_桜_013.png must exist under exactly that name"
    );
}
