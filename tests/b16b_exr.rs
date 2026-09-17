//! B-16b: EXR in and out (D-62, T-14 for EXR).
//!
//! Every expected value comes from `Fixtures/exr/expected_exr.json`, which
//! `tools/exr_reference.py` wrote with OpenEXR's own library. This test reads every fixture file
//! with the build and compares each pixel exactly, each reason and each refusal (FX-EXR-001 to
//! 007), imports the sequence with its gap (FX-EXR-008), and exports the four pictures of
//! FX-EXR-009 and 010 into `verification/B-16b export/`, where
//! `python tools/exr_reference.py check "verification/B-16b export"` checks them with OpenEXR.
//!
//! Writes `verification/B-16b_exr_table.md`.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

use serde_json::Value as J;

use anime_compositor::command::{Command, Document};
use anime_compositor::diagnostics::{DiagnosticId, Severity};
use anime_compositor::export::{export_sequence, ExportRequest, MissingSource, OutputFormat};
use anime_compositor::exr_io::{self, ExrSamples};
use anime_compositor::media;
use anime_compositor::model::{Asset, Composition, Id, Interpretation, Layer, Project};
use anime_compositor::time::FrameRate;
use anime_compositor::{AlphaMode, ColorSpace, OutputAlpha, OutputDepth};

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

struct Table {
    out: String,
    checks: usize,
    passed: usize,
}

impl Table {
    fn row(&mut self, what: &str, built: &str, ok: bool) {
        self.checks += 1;
        self.passed += ok as usize;
        let verdict = if ok { "yes" } else { "**NO**" };
        self.out
            .push_str(&format!("| {what} | {built} | {verdict} |\n"));
    }

    fn heading(&mut self, text: &str) {
        self.out.push_str(&format!(
            "\n## {text}\n\n| Check | The build's answer | Matches |\n| --- | --- | --- |\n"
        ));
    }
}

/// How many samples differ from the expected pixels. Exact: no tolerance (D-62).
fn differing(data: &[f32], expected: &J) -> usize {
    let expected: Vec<f32> = expected
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|p| {
            p.as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_f64().unwrap() as f32)
        })
        .collect();
    if expected.len() != data.len() {
        return expected.len().max(data.len());
    }
    data.iter().zip(&expected).filter(|(a, b)| a != b).count()
}

fn reasons(adjusted: &[(&str, String)]) -> String {
    if adjusted.is_empty() {
        return "none".into();
    }
    adjusted
        .iter()
        .map(|(r, d)| format!("`{r}` {d}"))
        .collect::<Vec<_>>()
        .join("; ")
}

/// A composition of the fixture's size holding one EXR still at identity, exported at frame 0.
fn export_one(case: &J, out: &Path) -> (Vec<PathBuf>, bool) {
    let rate = &case["frame_rate"];
    let rate = FrameRate::new(
        rate[0].as_u64().unwrap() as u32,
        rate[1].as_u64().unwrap() as u32,
    )
    .unwrap();
    let mut project = Project::new(Id::new("proj-b16b"));
    project.compositions.push(Composition::new(
        Id::new("comp"),
        "exr export",
        case["width"].as_u64().unwrap() as u32,
        case["height"].as_u64().unwrap() as u32,
        rate,
        0,
        1,
    ));
    let mut doc = Document::new(project);
    let source = case["source"].as_str().unwrap();
    doc.apply(Command::AddAsset {
        asset: Asset::still(Id::new("asset-exr"), source, source),
    })
    .unwrap();
    doc.apply(Command::AddLayer {
        composition: Id::new("comp"),
        layer: Box::new(Layer::new(
            Id::new("layer-exr"),
            "exr",
            Id::new("asset-exr"),
            0,
            1,
        )),
        index: 0,
    })
    .unwrap();
    let request = ExportRequest {
        composition: Id::new("comp"),
        first_frame: 0,
        last_frame: 0,
        output_dir: out.to_path_buf(),
        naming: format!("{}_%04d.exr", case["name"].as_str().unwrap()),
        depth: OutputDepth::Eight,
        alpha: OutputAlpha::Straight,
        tile_size: 64,
        missing: MissingSource::Block,
        format: OutputFormat::Exr(match case["depth"].as_str().unwrap() {
            "half" => ExrSamples::Half,
            _ => ExrSamples::Float,
        }),
    };
    let report = export_sequence(
        doc.project(),
        &repo("Fixtures/exr"),
        &request,
        &AtomicBool::new(false),
    );
    (report.written.clone(), report.succeeded())
}

#[test]
fn b16b_exr() {
    let root = repo("Fixtures/exr");
    let expected: J =
        serde_json::from_slice(&fs::read(root.join("expected_exr.json")).unwrap()).unwrap();
    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };

    // FX-EXR-001 to 008: every file, read on its own.
    let mut case = String::new();
    for file in expected["files"].as_array().unwrap() {
        let this = file["case"].as_str().unwrap();
        if this != case {
            case = this.to_string();
            t.heading(&format!("{case}: reading"));
        }
        let rel = file["file"].as_str().unwrap();
        let what = format!("`{rel}`: {}", file["what"].as_str().unwrap());
        let read = exr_io::read(&root.join(rel));
        match (file["answer"].as_str().unwrap(), read) {
            ("drawn", Ok(picture)) => {
                let (w, h) = (
                    file["width"].as_u64().unwrap() as usize,
                    file["height"].as_u64().unwrap() as usize,
                );
                let size_ok = (picture.image.width(), picture.image.height()) == (w, h);
                let wrong = differing(picture.image.data(), &file["pixels"]);
                let want: Vec<(String, String)> = file["adjusted"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|a| {
                        (
                            a["reason"].as_str().unwrap().to_string(),
                            a["detail"].as_str().unwrap().to_string(),
                        )
                    })
                    .collect();
                let got: Vec<(String, String)> = picture
                    .adjusted
                    .iter()
                    .map(|(r, d)| (r.to_string(), d.clone()))
                    .collect();
                let tags_ok = picture.image.color_space() == ColorSpace::LinearLight
                    && picture.image.alpha_mode() == AlphaMode::Premultiplied;
                t.row(
                    &format!("{what}: drawn {w} by {h}, every pixel as expected"),
                    &format!(
                        "drawn {} by {}, {wrong} of {} samples differ; reported: {}",
                        picture.image.width(),
                        picture.image.height(),
                        w * h * 4,
                        reasons(&picture.adjusted)
                    ),
                    size_ok && wrong == 0 && want == got && tags_ok,
                );
            }
            ("refused", Err(d)) => {
                let id = file["diagnostic"].as_str().unwrap();
                let reason = file["reason"].as_str().unwrap();
                t.row(
                    &format!("{what}: refused, `{id}`, `{reason}`"),
                    &format!("refused, `{}`: {}", d.id, d.detail),
                    d.id.as_str() == id && d.detail.starts_with(&format!("Reason: {reason}.")),
                );
            }
            (answer, Ok(_)) => t.row(&format!("{what}: {answer}"), "drawn", false),
            (answer, Err(d)) => t.row(
                &format!("{what}: {answer}"),
                &format!("refused, `{}`: {}", d.id, d.detail),
                false,
            ),
        }
    }

    t.heading("MEDIA_EXR_ADJUSTED, as a person reads it");
    let specials = root.join("values/specials.exr");
    let d = exr_io::read(&specials)
        .ok()
        .and_then(|p| p.diagnostic(&specials));
    t.row(
        "`values/specials.exr` gives one WARNING, `MEDIA_EXR_ADJUSTED`, with one line per reason",
        &d.as_ref().map_or("nothing".into(), |d| {
            format!("{} {}: {}", d.severity, d.id, d.detail.replace('\n', "; "))
        }),
        d.as_ref().is_some_and(|d| {
            d.id == DiagnosticId::MediaExrAdjusted
                && d.severity == Severity::Warning
                && d.detail == "non_finite: 4\nalpha_clamped: 2"
        }),
    );
    let plain = root.join("compression/zip_half.exr");
    let none = exr_io::read(&plain).ok().map(|p| p.diagnostic(&plain));
    t.row(
        "`compression/zip_half.exr`, drawn as stored, gives no report",
        &format!("{none:?}"),
        none == Some(None),
    );

    // FX-EXR-008: the sequence, through the same importer as PNG.
    t.heading("FX-EXR-008: importing the sequence");
    let seq = &expected["sequence"];
    let files: Vec<PathBuf> = seq["files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| root.join(f.as_str().unwrap()))
        .collect();
    let imported = media::import_sequence(&files);
    let asset = imported.asset.as_ref().unwrap();
    let pattern = seq["pattern"].as_str().unwrap();
    t.row(
        &format!("The pattern is `{pattern}`"),
        &format!("`{}`", asset.pattern()),
        asset.pattern() == pattern,
    );
    let drawings: Vec<u32> = asset.frames().keys().copied().collect();
    let want: Vec<u32> = seq["drawings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    t.row(
        &format!("Drawings {want:?} are there"),
        &format!("{drawings:?}"),
        drawings == want,
    );
    let missing: Vec<u32> = seq["missing"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    t.row(
        &format!("Drawing {missing:?} is missing, and import says so with `MEDIA_SEQUENCE_GAP`"),
        &format!(
            "{:?}; {}",
            asset.missing(),
            imported
                .diagnostics
                .iter()
                .map(|d| d.id.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ),
        asset.missing() == missing
            && imported.diagnostics.len() == 1
            && imported.has(DiagnosticId::MediaSequenceGap),
    );
    t.row(
        "Asking for drawing 4 gives `MEDIA_SEQUENCE_GAP`, not a neighbouring drawing",
        &match asset.decode(4) {
            Ok(_) => "a picture".to_string(),
            Err(d) => format!("`{}`", d.id),
        },
        asset.decode(4).err().map(|d| d.id) == Some(DiagnosticId::MediaSequenceGap),
    );
    let flat = want
        .iter()
        .zip(seq["files"].as_array().unwrap())
        .all(|(&n, rel)| {
            let entry = expected["files"]
                .as_array()
                .unwrap()
                .iter()
                .find(|f| f["file"] == *rel)
                .unwrap();
            asset
                .decode(n)
                .is_ok_and(|image| differing(image.data(), &entry["pixels"]) == 0)
        });
    t.row(
        "Drawings 1, 2, 3 and 5, asked for by number, give their files' expected pixels",
        if flat { "they do" } else { "they do not" },
        flat,
    );
    let interpretation = Asset::sequence(Id::new("a"), pattern, pattern).interpretation;
    t.row(
        "A new EXR asset is read as linear light, premultiplied (D-62), a PNG one as before",
        &format!(
            "EXR {:?}; PNG {:?}",
            interpretation,
            Asset::still(Id::new("b"), "x.png", "x.png").interpretation
        ),
        interpretation
            == Interpretation {
                color_space: ColorSpace::LinearLight,
                alpha: AlphaMode::Premultiplied,
            }
            && Asset::still(Id::new("b"), "x.png", "x.png").interpretation
                == Interpretation::default(),
    );

    // FX-EXR-009 and 010: export, then read back.
    let out = repo("verification/B-16b export");
    let _ = fs::remove_dir_all(&out);
    fs::create_dir_all(&out).unwrap();
    let again = repo("target/b16b-again");
    let _ = fs::remove_dir_all(&again);
    fs::create_dir_all(&again).unwrap();
    let mut case = String::new();
    for export in expected["export"].as_array().unwrap() {
        let this = export["case"].as_str().unwrap();
        if this != case {
            case = this.to_string();
            t.heading(&format!("{case}: exporting"));
        }
        let name = export["name"].as_str().unwrap();
        let (written, ok) = export_one(export, &out);
        let file = out.join(format!("{name}_0000.exr"));
        t.row(
            &format!("`{name}` exports one frame as `{name}_0000.exr`"),
            &written
                .iter()
                .map(|p| format!("`{}`", p.file_name().unwrap().to_string_lossy()))
                .collect::<Vec<_>>()
                .join(", "),
            ok && written == [file.clone()],
        );
        let back = exr_io::read(&file);
        t.row(
            &format!("`{name}`, read back by the build, holds the expected samples"),
            &match &back {
                Ok(p) => format!(
                    "{} of {} samples differ; reported: {}",
                    differing(p.image.data(), &export["pixels"]),
                    p.image.data().len(),
                    reasons(&p.adjusted)
                ),
                Err(d) => format!("refused, `{}`", d.id),
            },
            back.as_ref().is_ok_and(|p| {
                differing(p.image.data(), &export["pixels"]) == 0 && p.adjusted.is_empty()
            }),
        );
        export_one(export, &again);
        let same = fs::read(&file).ok() == fs::read(again.join(format!("{name}_0000.exr"))).ok();
        t.row(
            &format!("`{name}` written twice is the same file, byte for byte"),
            if same { "the same" } else { "different" },
            same,
        );
    }

    let doc = format!(
        "# B-16b: EXR in and out\n\n\
         **{} of {} checks pass.**\n\n\
         Generated by `tests/b16b_exr.rs`. Covers R-15's first format, test T-14 for EXR and the \
         fixtures FX-EXR-001 to 010 of document 25, under D-62.\n\n\
         ## How to read this\n\n\
         Each row is one fixture file or one thing document 25 says must be true. The expected \
         answers were worked out separately by `tools/exr_reference.py`, which uses OpenEXR's \
         own library, and are in `Fixtures/exr/expected_exr.json`. A drawn file must match \
         every sample exactly and give exactly the reasons listed; a refused one must give the \
         identifier and reason listed. The last column says whether the build's answer \
         matches.\n\n\
         The exported files are in `verification/B-16b export/`. The rows here read them back \
         with the build; the independent check reads them with OpenEXR's library, and its \
         result is in `verification/B-16b_exr_export_check.md`.\n",
        t.passed, t.checks
    );
    fs::write(repo("verification/B-16b_exr_table.md"), doc + &t.out).unwrap();
    assert_eq!(t.passed, t.checks, "see verification/B-16b_exr_table.md");
}
