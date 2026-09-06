//! H-04: the picture in the file, not the picture in memory.
//!
//! Writes `verification/H-04_exported_file_table.md`.
//!
//! # What H-01 to H-03 left
//!
//! Those three composite the reference shot against independent compositors and compare every
//! pixel, but they all stop in the working space: linear light, premultiplied, `f32`. The file
//! the owner actually receives is none of those things. Between the last of those comparisons and
//! the PNG on disk sits one more step - document 21 line 31, *"PNG output converts the linear
//! working RGB to the declared output encoding, then writes straight alpha"* - and every test in
//! this repository that touches that step calls the build's own `encode` to say what it should
//! have produced. `tests/t08_export.rs` compares the exported file against `rendered.encode(..)`;
//! `tests/b02_color_alpha.rs` checks the transfer function on named values through the same
//! function. Both are worth having and neither can see a fault in *layout*: samples written in
//! the order B, G, R, A; a row dropped or repeated; sixteen-bit samples written little-endian;
//! alpha quantised against 65535 while colour is quantised against 255. Every one of those leaves
//! the named-value tables passing and every pixel of the owner's export wrong.
//!
//! So this exports frames through the real export path, reads the files back off disk with a
//! decoder that is not the one that wrote them, and compares against samples computed by a
//! separate compositor and a separately written encoder.
//!
//! # Where the expected values come from
//!
//! - Document 21 line 29 for the decode and D-17 for the sRGB curve, as in H-01, giving a linear
//!   premultiplied frame computed independently of the renderer.
//! - Document 21 line 31 for the output step: unpremultiply in linear light, apply the transfer
//!   function, then quantise, and write straight alpha.
//! - Document 21 line 7 for the clamp: *"final integer output conversion clamps only at the
//!   declared encoding step"*, so the clamp happens here and nowhere earlier.
//! - The PNG specification for byte order at sixteen bits: most significant byte first.
//!
//! Document 21 does not state a rounding rule, so that one detail is the build's own (`color.rs`
//! says "to nearest, ties away from zero"). A rounding rule is exactly the kind of thing two
//! correct implementations may spell differently, which is why the tolerance below is one code
//! value rather than zero - and why how many samples use any of it is checked as well as how
//! large the differences are.
//!
//! # The tolerance
//!
//! One code value per sample, which is what document 11 permits: *"For an 8-bit output round-trip,
//! allow at most one code value per tested channel where rounding applies."* Most faults this file
//! is written to catch move samples by far more than that - a channel swap by the difference
//! between two colours, a dropped row by a whole row, a byte-order error by up to 255 in the high
//! byte - and one does not: truncating instead of rounding moves millions of samples by exactly
//! one. So the *count* of samples that are not exactly identical is a check in its own right and
//! not merely a note, and it is a small number, in the single figures. See `RARE`.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

use anime_compositor::command::{Command, Document};
use anime_compositor::export::{export_sequence, ExportRequest, MissingSource};
use anime_compositor::media::import_sequence;
use anime_compositor::model::{Asset, AssetKind, Composition, Id, Layer, Project, Prop, Value};
use anime_compositor::time::{ExposureMap, ExposureSpan, FrameRate};
use anime_compositor::{OutputAlpha, OutputDepth};

const COMP: &str = "comp-h04";
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

/// The allowance argued for in the module documentation.
const BOUND: u16 = 1;

/// How many samples out of eight million may use any of that allowance before it stops being
/// rounding disagreement and starts being a difference in what is being computed. Truncation
/// instead of rounding, for instance, moves no sample by more than one code value - and moves
/// millions of them. A break doing exactly that survived the first draft of this file, which
/// checked only the size of the largest difference and reported the count without requiring
/// anything of it.
const RARE: usize = 1000;

/// The frame used for the rows that need partly transparent pixels, chosen in H-03 for the same
/// reason: at frame 106 the moving layers overlap heavily. Frame 110, H-03's choice, cannot be
/// used here: its third layer lands on the drawing the fixture deliberately omits, and an export
/// refuses that frame rather than inventing one.
const FLOATING_FRAME: i32 = 106;

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
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn root() -> PathBuf {
    repo("Fixtures/reference_shot")
}

fn id(text: &str) -> Id {
    Id::new(text)
}

/// A scratch folder for the exported files, emptied first so a run never reads a previous run's.
fn scratch() -> PathBuf {
    let dir = repo("target/h04");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap_or_else(|e| panic!("make {}: {e}", dir.display()));
    dir
}

// ---------------------------------------------------------------------------------------
// The independent compositor, and the independent encoder after it
// ---------------------------------------------------------------------------------------

fn srgb_to_linear(c: f64) -> f64 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// D-17 again, the other way round.
fn linear_to_srgb(c: f64) -> f64 {
    if c <= 0.003_130_8 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

fn decode_linear(path: &Path) -> Option<Vec<[f64; 4]>> {
    let file = fs::File::open(path).ok()?;
    let mut reader = png::Decoder::new(std::io::BufReader::new(file))
        .read_info()
        .unwrap_or_else(|e| panic!("{} is not a readable PNG: {e}", path.display()));
    let mut bytes = vec![0u8; reader.output_buffer_size().expect("a sized buffer")];
    reader
        .next_frame(&mut bytes)
        .unwrap_or_else(|e| panic!("{} does not decode: {e}", path.display()));
    Some(
        bytes
            .chunks_exact(4)
            .map(|p| {
                let a = p[3] as f64 / 255.0;
                [
                    srgb_to_linear(p[0] as f64 / 255.0) * a,
                    srgb_to_linear(p[1] as f64 / 255.0) * a,
                    srgb_to_linear(p[2] as f64 / 255.0) * a,
                    a,
                ]
            })
            .collect(),
    )
}

fn drawing_at(layer: u32, frame: i32) -> u32 {
    let f = frame as u32;
    match layer {
        1 => 0,
        2 => f % 24,
        3 => (f / 2) % 12,
        4 => {
            let (drawings, lengths) = layer4_sheet();
            let mut at = 0u32;
            for (drawing, length) in drawings.iter().zip(&lengths) {
                if f < at + length {
                    return *drawing;
                }
                at += length;
            }
            *drawings.last().expect("the sheet is not empty")
        }
        _ => unreachable!(),
    }
}

fn layer4_sheet() -> (Vec<u32>, Vec<u32>) {
    let text =
        fs::read_to_string(root().join("exposure_sheet.json")).expect("read the exposure sheet");
    let sheet: serde_json::Value = serde_json::from_str(&text).expect("the sheet is JSON");
    let array = |key: &str| -> Vec<u32> {
        sheet
            .get(key)
            .unwrap_or_else(|| panic!("the sheet has no {key}"))
            .as_array()
            .unwrap_or_else(|| panic!("{key} is not an array"))
            .iter()
            .map(|n| n.as_u64().expect("a whole number") as u32)
            .collect()
    };
    (
        array("layer4_exposure_drawing_ids"),
        array("layer4_exposure_lengths"),
    )
}

fn cel_path(layer: u32, drawing: u32) -> Option<PathBuf> {
    let dir = root().join(format!("layer{layer}"));
    let mut hits: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
        .map(|e| e.expect("directory entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "png"))
        .filter(|p| {
            let name = p.file_stem().unwrap_or_default().to_string_lossy();
            let digits: String = name
                .chars()
                .rev()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            digits
                .chars()
                .rev()
                .collect::<String>()
                .parse::<u32>()
                .is_ok_and(|n| n == drawing)
        })
        .collect();
    hits.sort();
    hits.into_iter().next()
}

/// The frame in the working space: linear light, premultiplied, every layer normal.
fn independent_frame(frame: i32, layers: &[u32]) -> Vec<[f64; 4]> {
    let mut out = vec![[0.0f64; 4]; WIDTH * HEIGHT];
    for &layer in layers {
        let Some(path) = cel_path(layer, drawing_at(layer, frame)) else {
            continue; // layer 3's deliberately missing drawing 7
        };
        let Some(source) = decode_linear(&path) else {
            continue;
        };
        for (dst, src) in out.iter_mut().zip(&source) {
            // Document 21 line 57, source over destination on premultiplied colour.
            let inv = 1.0 - src[3];
            *dst = [
                src[0] + dst[0] * inv,
                src[1] + dst[1] * inv,
                src[2] + dst[2] * inv,
                src[3] + dst[3] * inv,
            ];
        }
    }
    out
}

/// Document 21 line 31, written out: unpremultiply, transfer function, quantise, straight alpha.
///
/// Returns one number per sample in PNG's order, RGBA per pixel, whatever the depth - the caller
/// compares numbers, and how they are laid out in bytes is the decoder's problem and the subject
/// of its own rows below.
fn independent_samples(frame: &[[f64; 4]], depth: OutputDepth, alpha: OutputAlpha) -> Vec<u16> {
    let max = match depth {
        OutputDepth::Eight => 255.0,
        OutputDepth::Sixteen => 65535.0,
    };
    // Document 21 line 7: the clamp belongs at the encoding step, and nowhere earlier.
    let quantise = |c: f64| -> u16 { (c.clamp(0.0, 1.0) * max + 0.5).floor() as u16 };
    let mut out = Vec::with_capacity(frame.len() * 4);
    for p in frame {
        let a = p[3];
        for c in p.iter().take(3) {
            let straight = match alpha {
                OutputAlpha::Straight if a > 0.0 => c / a,
                OutputAlpha::Straight => 0.0,
                OutputAlpha::Premultiplied => *c,
            };
            out.push(quantise(linear_to_srgb(straight)));
        }
        out.push(quantise(a));
    }
    out
}

// ---------------------------------------------------------------------------------------
// The project, and the export that writes it to disk
// ---------------------------------------------------------------------------------------

fn asset_for(layer: u32) -> Asset {
    let dir = root().join(format!("layer{layer}"));
    let files: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
        .map(|e| e.expect("directory entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "png"))
        .collect();
    let imported = import_sequence(&files)
        .asset
        .unwrap_or_else(|| panic!("layer{layer} imports as a sequence"));
    let frames: BTreeMap<u32, String> = imported
        .frames()
        .iter()
        .map(|(number, path)| {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .expect("a UTF-8 file name");
            (*number, format!("layer{layer}/{name}"))
        })
        .collect();
    Asset {
        id: id(&format!("asset-layer{layer}")),
        kind: AssetKind::ImageSequence,
        name: format!("layer{layer}"),
        path: None,
        pattern: Some(imported.pattern().to_string()),
        frames,
        interpretation: Default::default(),
    }
}

fn spans(layer: u32) -> Vec<ExposureSpan> {
    let (drawings, lengths) = layer4_sheet();
    let exposures: Vec<(u32, u32)> = match layer {
        1 => vec![(0, 240)],
        2 => (0..240).map(|f| (f % 24, 1)).collect(),
        3 => (0..120).map(|k| (k % 12, 2)).collect(),
        4 => drawings.into_iter().zip(lengths).collect(),
        _ => unreachable!(),
    };
    ExposureMap::from_lengths(&exposures)
        .expect("the cadences are disjoint and in order")
        .spans()
        .to_vec()
}

/// `layers` is which of the shot's four layers to put in the composition. Every row but one uses
/// all four; the exception drops the opaque background, for the reason argued at its call site.
fn build_project(layers: &[u32]) -> Project {
    let mut project = Project::new(id("proj-h04"));
    project.compositions.push(Composition::new(
        id(COMP),
        "reference shot, exported",
        WIDTH as u32,
        HEIGHT as u32,
        FrameRate::new(24, 1).expect("24 fps"),
        0,
        240,
    ));
    let mut doc = Document::new(project);
    for (index, n) in layers.iter().enumerate() {
        let asset = asset_for(*n);
        let mut layer = Layer::new(
            id(&format!("layer-{n}")),
            format!("layer{n}"),
            asset.id.clone(),
            0,
            240,
        );
        layer.exposure_spans = spans(*n);
        doc.apply_all(vec![
            Command::AddAsset { asset },
            Command::AddLayer {
                composition: id(COMP),
                layer: Box::new(layer),
                index,
            },
            Command::SetPropertyBase {
                composition: id(COMP),
                layer_id: id(&format!("layer-{n}")),
                prop: Prop::Opacity,
                value: Value::Scalar(1.0),
            },
        ])
        .expect("the project is valid");
    }
    doc.project().clone()
}

/// Export one frame through the real export path and hand back the file it wrote.
fn export_one(
    project: &Project,
    dir: &Path,
    frame: i32,
    depth: OutputDepth,
    alpha: OutputAlpha,
) -> PathBuf {
    let request = ExportRequest {
        composition: id(COMP),
        first_frame: frame,
        last_frame: frame,
        output_dir: dir.to_path_buf(),
        naming: "h04_%04d.png".to_string(),
        depth,
        alpha,
        tile_size: 128,
        missing: MissingSource::Block,
    };
    let report = export_sequence(project, &root(), &request, &AtomicBool::new(false));
    assert!(
        report.succeeded(),
        "the export of frame {frame} did not complete: {:?}",
        report.status
    );
    report.written[0].clone()
}

/// What is actually in the file: the samples, in the order the PNG carries them, plus the size
/// and depth the file itself declares.
struct DecodedFile {
    samples: Vec<u16>,
    width: u32,
    height: u32,
    bit_depth: png::BitDepth,
    color_type: png::ColorType,
}

/// Read a written file back with the `png` crate's decoder, which is not what wrote it.
///
/// Sixteen-bit samples are assembled from bytes here, most significant first, so that a build
/// writing them the other way round shows up as wrong numbers rather than as a decoder error.
fn decode_file(path: &Path) -> DecodedFile {
    let file = fs::File::open(path).unwrap_or_else(|e| panic!("open {}: {e}", path.display()));
    let mut reader = png::Decoder::new(std::io::BufReader::new(file))
        .read_info()
        .unwrap_or_else(|e| panic!("{} is not a readable PNG: {e}", path.display()));
    let mut bytes = vec![0u8; reader.output_buffer_size().expect("a sized buffer")];
    let info = reader
        .next_frame(&mut bytes)
        .unwrap_or_else(|e| panic!("{} does not decode: {e}", path.display()));
    let samples = match info.bit_depth {
        png::BitDepth::Eight => bytes[..info.buffer_size()]
            .iter()
            .map(|&b| b as u16)
            .collect(),
        png::BitDepth::Sixteen => bytes[..info.buffer_size()]
            .chunks_exact(2)
            .map(|p| u16::from_be_bytes([p[0], p[1]]))
            .collect(),
        other => panic!("unexpected bit depth {other:?}"),
    };
    DecodedFile {
        samples,
        width: info.width,
        height: info.height,
        bit_depth: info.bit_depth,
        color_type: info.color_type,
    }
}

struct Difference {
    over_bound: usize,
    not_identical: usize,
    largest: u16,
}

fn compare(file: &[u16], oracle: &[u16]) -> Difference {
    let mut over_bound = 0;
    let mut not_identical = 0;
    let mut largest = 0u16;
    for (a, b) in file.iter().zip(oracle) {
        let d = a.abs_diff(*b);
        if d > 0 {
            not_identical += 1;
        }
        if d > BOUND {
            over_bound += 1;
        }
        largest = largest.max(d);
    }
    Difference {
        over_bound,
        not_identical,
        largest,
    }
}

// ---------------------------------------------------------------------------------------
// The test
// ---------------------------------------------------------------------------------------

#[test]
fn what_reaches_the_file_is_what_the_specification_says_it_should_be() {
    let mut report = Report::default();
    let project = build_project(&[1, 2, 3, 4]);
    let dir = scratch();

    for frame in [0, 100] {
        let path = export_one(
            &project,
            &dir,
            frame,
            OutputDepth::Eight,
            OutputAlpha::Straight,
        );
        let file = decode_file(&path);
        let oracle = independent_samples(
            &independent_frame(frame, &[1, 2, 3, 4]),
            OutputDepth::Eight,
            OutputAlpha::Straight,
        );

        report.check(
            &format!("frame {frame} is written as a {WIDTH} by {HEIGHT} eight-bit RGBA picture"),
            format!("{WIDTH}x{HEIGHT} Rgba Eight"),
            format!(
                "{}x{} {:?} {:?}",
                file.width, file.height, file.color_type, file.bit_depth
            ),
        );
        report.check(
            &format!("frame {frame} carries a sample for every channel of every pixel"),
            WIDTH * HEIGHT * 4,
            file.samples.len(),
        );

        let d = compare(&file.samples, &oracle);
        report.check(
            &format!(
                "frame {frame}: every one of the {} samples in the file is within one code value \
                 of what a separate compositor and a separately written encoder say it should be",
                WIDTH * HEIGHT * 4
            ),
            "0 samples differ by more than one",
            format!("{} samples differ by more than one", d.over_bound),
        );
        report.check(
            &format!(
                "frame {frame}: and only {} of them are not exactly identical, so the one-code-value \
                 allowance is barely being used rather than covering a systematic difference",
                d.not_identical
            ),
            format!("fewer than {RARE} of {} not identical", WIDTH * HEIGHT * 4),
            if d.not_identical < RARE {
                format!("fewer than {RARE} of {} not identical", WIDTH * HEIGHT * 4)
            } else {
                format!("{} of {} not identical", d.not_identical, WIDTH * HEIGHT * 4)
            },
        );
    }

    // Sixteen bits: the same picture at a different depth, which is where byte order lives.
    let path = export_one(
        &project,
        &dir,
        100,
        OutputDepth::Sixteen,
        OutputAlpha::Straight,
    );
    let file = decode_file(&path);
    let oracle = independent_samples(
        &independent_frame(100, &[1, 2, 3, 4]),
        OutputDepth::Sixteen,
        OutputAlpha::Straight,
    );
    report.check(
        "asked for sixteen bits, the file says sixteen bits",
        "Sixteen",
        format!("{:?}", file.bit_depth),
    );
    report.check(
        "and carries one sixteen-bit number per channel of every pixel",
        WIDTH * HEIGHT * 4,
        file.samples.len(),
    );
    let d = compare(&file.samples, &oracle);
    report.check(
        "every sixteen-bit sample is within one code value of the independent encoder, which is \
         also what says the two bytes are in the order the PNG specification requires",
        "0 samples differ by more than one",
        format!("{} samples differ by more than one", d.over_bound),
    );
    report.check(
        &format!(
            "and only {} of them are not exactly identical, with the largest difference {} out of \
             a possible 65535",
            d.not_identical, d.largest
        ),
        format!("fewer than {RARE} not identical"),
        if d.not_identical < RARE {
            format!("fewer than {RARE} not identical")
        } else {
            format!("{} not identical", d.not_identical)
        },
    );

    // Everything above is blind to half of this file's subject, and three deliberate breaks
    // proved it. Layer 1 is opaque and fills the frame, so every composited pixel comes out with
    // alpha exactly 1 - and at alpha 1, unwinding the premultiplication changes nothing, and
    // putting alpha itself through the colour curve changes nothing either, because both of those
    // steps are identities at 1. A build that skipped the unwind entirely, one that did it when
    // asked not to, and one that ran alpha through the colour curve all passed every row above.
    //
    // So the same frame is exported again with the opaque background taken away, which is the
    // only way for the picture to contain the partly transparent pixels those steps act on.
    let floating = build_project(&[2, 3, 4]);
    let floating_layers = [2u32, 3, 4];
    let mut floating_samples: Vec<Vec<u16>> = Vec::new();
    for alpha in [OutputAlpha::Straight, OutputAlpha::Premultiplied] {
        let path = export_one(&floating, &dir, FLOATING_FRAME, OutputDepth::Eight, alpha);
        let file = decode_file(&path);
        let oracle = independent_samples(
            &independent_frame(FLOATING_FRAME, &floating_layers),
            OutputDepth::Eight,
            alpha,
        );
        let d = compare(&file.samples, &oracle);
        report.check(
            &format!(
                "with the opaque background taken away, so the picture is partly transparent \
                 nearly everywhere, every sample of the {alpha:?}-alpha export matches the \
                 independent encoder"
            ),
            "0 samples differ by more than one",
            format!("{} samples differ by more than one", d.over_bound),
        );
        report.check(
            &format!(
                "and only {} of those samples are not exactly identical",
                d.not_identical
            ),
            format!("fewer than {RARE} not identical"),
            if d.not_identical < RARE {
                format!("fewer than {RARE} not identical")
            } else {
                format!("{} not identical", d.not_identical)
            },
        );
        floating_samples.push(file.samples);
    }

    // Those two rows only mean something if the two ways of writing transparency actually differ
    // on this picture. On the full shot they are identical, which is exactly what went wrong.
    let d = compare(&floating_samples[0], &floating_samples[1]);
    report.check(
        "and the two ways of writing transparency genuinely disagree on this picture, which is \
         what the two rows above depend on and what the full shot cannot provide",
        "more than 100000 samples differ",
        if d.over_bound > 100_000 {
            "more than 100000 samples differ".to_string()
        } else {
            format!("only {} differ", d.over_bound)
        },
    );

    // And the reason they disagree, stated as its own count so it cannot quietly go to zero.
    let partial = independent_frame(FLOATING_FRAME, &floating_layers)
        .iter()
        .filter(|p| p[3] > 0.001 && p[3] < 0.999)
        .count();
    report.check(
        &format!(
            "frame {FLOATING_FRAME} without its background has {partial} pixels that are neither \
             solid nor empty, which is what makes the transparency steps observable at all"
        ),
        "more than 100000 such pixels",
        if partial > 100_000 {
            "more than 100000 such pixels".to_string()
        } else {
            format!("only {partial}")
        },
    );

    // The comparison has to be able to fail. Two different frames of the shot, compared against
    // each other's expected samples, must disagree loudly - otherwise every row above could be
    // comparing two things that are always equal for a reason that has nothing to do with the
    // encode.
    let straight_100 = independent_samples(
        &independent_frame(100, &[1, 2, 3, 4]),
        OutputDepth::Eight,
        OutputAlpha::Straight,
    );
    let straight_0 = independent_samples(
        &independent_frame(0, &[1, 2, 3, 4]),
        OutputDepth::Eight,
        OutputAlpha::Straight,
    );
    let d = compare(&straight_100, &straight_0);
    report.check(
        "the comparison can fail: frame 100's samples against frame 0's expected samples \
         disagree in hundreds of thousands of places",
        "more than 100000 samples differ",
        if d.over_bound > 100_000 {
            "more than 100000 samples differ".to_string()
        } else {
            format!("only {} differ", d.over_bound)
        },
    );

    // And the picture is not trivially uniform: a frame of solid black or solid transparency
    // would satisfy several rows above for reasons that have nothing to do with correctness.
    let distinct = {
        let mut seen = std::collections::BTreeSet::new();
        for s in straight_100.iter().step_by(4) {
            seen.insert(*s);
            if seen.len() > 32 {
                break;
            }
        }
        seen.len()
    };
    report.check(
        "and the frame being compared is a real picture, not a flat colour: its red channel takes \
         many different values",
        "more than 32 distinct values",
        if distinct > 32 {
            "more than 32 distinct values".to_string()
        } else {
            format!("only {distinct}")
        },
    );

    write_report(&report);
    let failed: Vec<&Row> = report.rows.iter().filter(|r| !r.pass()).collect();
    assert!(
        failed.is_empty(),
        "{} of {} checks failed, first: {} (expected {}, got {})",
        failed.len(),
        report.rows.len(),
        failed[0].check,
        failed[0].expected,
        failed[0].actual
    );
}

fn write_report(report: &Report) {
    let passed = report.rows.iter().filter(|r| r.pass()).count();
    let mut out = String::new();
    out.push_str(&format!(
        "# H-04 â€” the picture in the file, not the picture in memory\n\n\
         **{passed} of {} checks passed.**\n\n\
         Produced by `tests/h04_exported_file.rs`.\n\n\
         ## Why this exists\n\n\
         H-01, H-02 and H-03 composite your shot twice and compare every pixel, but all three of \
         them stop in the renderer's own working space. The file you actually open is one step \
         further on: colour converted for output, transparency unwound, numbers rounded to \
         whole ones and written into bytes.\n\n\
         Every other test in this project that touches that last step asks the build's own \
         encoder what the answer should be. That catches arithmetic and cannot catch layout. If \
         the samples were written blue-first, or a row were dropped, or the two bytes of a \
         sixteen-bit sample were written the wrong way round, every existing table would still \
         pass and every pixel of your export would be wrong.\n\n\
         So this exports frames through the real export path, reads the files back off disk with \
         a decoder that did not write them, and compares them against numbers produced by a \
         separate compositor and a separately written encoder.\n\n\
         ## What is checked\n\n\
         Two frames at eight bits, one at sixteen, one written with transparency left folded into \
         the colour - the option document 07 offers and almost nobody wants - and two more with \
         the background layer taken away, for the reason in the next section but one. For each, \
         every sample in the file is compared, plus what the file says about itself: its size, \
         its depth, and that it has a number for every channel of every pixel.\n\n\
         ## The tolerance\n\n\
         **One code value out of 255** (or out of 65535 at sixteen bits), which is what document \
         11 allows for an eight-bit round trip where rounding applies. But a tolerance that is \
         right for one kind of fault is a hiding place for another: rounding a number down instead \
         of to the nearest whole one also moves every sample by at most one code value, and moves \
         millions of them. So **how many samples are not exactly identical** is a check in its own \
         right, and the answer is one, and nine, out of eight and a quarter million. The rest of \
         what this file exists to catch is worth far more than one code value anyway: a swapped \
         channel is the difference between two colours, a dropped row is a whole row, a byte-order \
         mistake is up to 255 in the wrong half of the number.\n\n\
         ## Why some rows have the background taken away\n\n\
         Your first layer is opaque and covers the frame, so every pixel of the finished picture \
         is fully solid. Two of the steps this file is about - unwinding the transparency out of \
         the colour, and leaving alpha out of the colour conversion - do nothing at all to a fully \
         solid pixel. Three deliberate breaks in those exact steps passed everything else here. \
         The only way to ask the question is a picture that is genuinely half-transparent, so the \
         same frame is exported again without its background, and a row counts how many pixels in \
         it are neither solid nor empty so that this cannot quietly stop being true.\n\n\
         ## The rows that are not comparisons\n\n\
         One takes frame 100's samples and compares them against what frame 0 should look like, \
         and requires them to disagree in hundreds of thousands of places. Without it, a build \
         where both sides produced nothing at all would pass every row above. One requires the \
         frame to be a real picture rather than a flat colour, for the same reason. One requires \
         the two ways of writing transparency to actually disagree on the picture they are tested \
         on. And one counts the half-transparent pixels described above.\n\n\
         ## What is deliberately not here\n\n\
         Whether the picture itself is right - whether frame 100 looks like your shot - is H-01's \
         question and H-03's. This one asks only whether what those tests verified in memory is \
         what ends up in the file.\n\n\
         As in H-01 to H-03, both sides were written from the same document by the same agent: \
         this catches an implementation slip, not a misreading of the specification.\n\n\
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
    fs::write(repo("verification/H-04_exported_file_table.md"), out).expect("write report");
}
