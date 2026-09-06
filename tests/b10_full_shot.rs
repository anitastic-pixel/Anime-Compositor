//! B-10's declared artifact: the whole 240-frame reference shot exported, twice, and compared
//! byte for byte.
//!
//! Writes `verification/B-10_full_shot_table.md` and `verification/B-10_contact_sheet.png`.
//! The sequences themselves go to `target/b10_full/pass1` and `target/b10_full/pass2` and are
//! not committed — see "Where the frames are" below.
//!
//! # What this proves that nothing before it could
//!
//! Document 15 names B-10's artifact as "the exported 240-frame sequence, plus a byte comparison
//! of two consecutive exports proving determinism". T-08 exported six frames of a two-layer
//! composition and T-07's export half compared six frames across a save and reopen. Neither is
//! the shot, and neither is determinism: six frames of one composition can agree by luck in a
//! renderer that is order-dependent somewhere, because six frames is six draws of the same
//! short straw.
//!
//! Two things only a whole shot can catch:
//!
//! - **Order-dependent output.** The renderer is tiled across rayon workers, so tiles finish in
//!   whatever order the pool hands them back. A fault that let one tile read another's result,
//!   or that accumulated into shared state, would show up as two exports of the same frame
//!   differing — rarely, and on some frames and not others. 240 frames exported twice is 480
//!   draws rather than twelve.
//! - **A frame the short ranges never reached.** T-08 and T-07e both export frames 12 to 17.
//!   Everything about layer 4's exposure sheet past frame 17, layer 2's second time around its
//!   24 drawings, and every one of the ten places the shot's deliberate hole falls, was
//!   exported by nothing until now.
//!
//! # Where the expected values come from
//!
//! The cadences are `Fixtures/reference_shot/README.md` and `exposure_sheet.json`, as in H-01:
//! layer 1 held on drawing 0, layer 2 `f % 24`, layer 3 `(f / 2) % 12`, layer 4 the sheet's
//! accumulated spans. Only layer 3's cadence is needed for an expected value here.
//!
//! Layer 3 has no drawing 7 — a deliberate defect of the shot — so the frames with a missing
//! drawing are exactly those where `(f / 2) % 12 == 7`. On twos that is a pair of frames every
//! 24, starting at 14:
//!
//! ```text
//! 14, 15,  38, 39,  62, 63,  86, 87,  110, 111,
//! 134, 135,  158, 159,  182, 183,  206, 207,  230, 231
//! ```
//!
//! Twenty frames. Document 14's D-25 fixes the rate limit at three logged in full and one
//! summary per group, so the export raises **four** `MEDIA_SEQUENCE_GAP` diagnostics, and the
//! summary's detail names those twenty frames in `FrameLog`'s range spelling and counts the
//! seventeen it did not log. Both strings are written out below and compared as literals.
//!
//! The file names come from document 07's `%04d`: `shot_0000.png` through `shot_0239.png`.
//!
//! No expected value here was read off a run of the code under test (ADR-009).
//!
//! # Where the frames are
//!
//! One frame of the assembled shot is a two-megabyte PNG, so the sequence is 480 megabytes and
//! committing it would be four times the rest of the repository. The frames are written to
//! `target/b10_full/pass1`, which is not cleaned between runs, so they can be opened, flipped
//! through or dropped into a player straight after `cargo test`. What is committed in their
//! place is `verification/B-10_contact_sheet.png`: all 240 frames at a twelfth scale in a 16 by
//! 15 grid, reading left to right and top to bottom, which is the whole shot on one page. The
//! ten empty gaps in it are the missing drawing and are correct.
//!
//! The contact sheet's thumbnails are averaged in linear light with premultiplied alpha, which
//! is the same space the renderer composites in. It is a viewing aid and no expected value
//! depends on it.
//!
//! # What is deliberately not here
//!
//! No new export behaviour, and no repeat of T-08's table: ranges, naming, cancellation, write
//! failure, bit depth and alpha policy are settled there. The missing-drawing policy is
//! `RenderTransparent` throughout, which is D-28's recorded override rather than the default,
//! because the default refuses the job and a refused job exports no shot to compare. That the
//! default refuses is T-08's row and is not restated here.
//!
//! This is slow — two full-resolution renders and encodes of 240 frames each — so it is
//! `#[ignore]`d, exactly as `b05a_transform`'s timing test is, and run by name.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::DiagnosticId;
use anime_compositor::export::{export_sequence, ExportReport, ExportRequest, MissingSource};
use anime_compositor::media::import_sequence;
use anime_compositor::model::{Asset, AssetKind, Composition, Id, Layer, Project, Prop, Value};
use anime_compositor::time::{ExposureMap, ExposureSpan, FrameRate};
use anime_compositor::{OutputAlpha, OutputDepth};

const COMP: &str = "comp-full-shot";
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const FRAMES: i32 = 240;

/// The contact sheet: a twelfth of full size, sixteen across.
const THUMB: usize = 12;
const COLUMNS: usize = 16;

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

// ---------------------------------------------------------------------------------------
// The project: the reference shot as drawn, all four layers at their own cadence
// ---------------------------------------------------------------------------------------

/// The `layer4_exposure_drawing_ids` and `layer4_exposure_lengths` arrays of the shot's sheet.
fn layer4_sheet() -> (Vec<u32>, Vec<u32>) {
    let path = root().join("exposure_sheet.json");
    let text = fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let sheet: serde_json::Value = serde_json::from_str(&text).expect("the sheet is JSON");
    let array = |key: &str| -> Vec<u32> {
        sheet[key]
            .as_array()
            .unwrap_or_else(|| panic!("{key} is an array"))
            .iter()
            .map(|v| v.as_u64().expect("a whole number") as u32)
            .collect()
    };
    (
        array("layer4_exposure_drawing_ids"),
        array("layer4_exposure_lengths"),
    )
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

/// The asset record an import of one layer's folder produces, with paths relative to the
/// project, which is how document 07 says they are stored.
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

fn build_project() -> Project {
    let mut project = Project::new(id("proj-b10-full-shot"));
    project.compositions.push(Composition::new(
        id(COMP),
        "reference shot",
        WIDTH as u32,
        HEIGHT as u32,
        FrameRate::new(24, 1).expect("24 fps"),
        0,
        FRAMES as u32,
    ));
    let mut doc = Document::new(project);
    for n in 1..=4u32 {
        let asset = asset_for(n);
        let mut layer = Layer::new(
            id(&format!("layer-{n}")),
            format!("layer{n}"),
            asset.id.clone(),
            0,
            FRAMES,
        );
        layer.exposure_spans = spans(n);
        doc.apply_all(vec![
            Command::AddAsset { asset },
            Command::AddLayer {
                composition: id(COMP),
                layer: Box::new(layer),
                index: (n - 1) as usize,
            },
        ])
        .expect("the opening project is valid");
    }
    // Document 22's shot is opaque behind and painted in front; nothing here changes what was
    // drawn. The one property set is layer 4's opacity, which the shot's own sheet calls for.
    doc.apply_all(vec![Command::SetPropertyBase {
        composition: id(COMP),
        layer_id: id("layer-4"),
        prop: Prop::Opacity,
        value: Value::Scalar(1.0),
    }])
    .expect("an opacity of one is valid");
    doc.project().clone()
}

// ---------------------------------------------------------------------------------------
// Exporting
// ---------------------------------------------------------------------------------------

/// A scratch folder under `target/`, emptied first so a run never compares a previous run's
/// files. `pass1` survives the run on purpose: it is the sequence to look at.
fn pass_dir(name: &str) -> PathBuf {
    let dir = repo("target/b10_full").join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap_or_else(|e| panic!("make {}: {e}", dir.display()));
    dir
}

fn request(dir: &Path) -> ExportRequest {
    ExportRequest {
        composition: id(COMP),
        first_frame: 0,
        last_frame: FRAMES - 1,
        output_dir: dir.to_path_buf(),
        naming: "shot_%04d.png".to_string(),
        depth: OutputDepth::Eight,
        alpha: OutputAlpha::Straight,
        tile_size: DEFAULT_TILE_SIZE,
        // D-28's recorded override. The default blocks, which is T-08's row.
        missing: MissingSource::RenderTransparent,
    }
}

fn run(project: &Project, dir: &Path) -> ExportReport {
    export_sequence(project, &root(), &request(dir), &AtomicBool::new(false))
}

/// The frames where layer 3 asks for its missing drawing 7, derived from the cadence rather
/// than from anything the export said.
fn gap_frames() -> Vec<i32> {
    (0..FRAMES).filter(|f| (f / 2) % 12 == 7).collect()
}

// ---------------------------------------------------------------------------------------
// Reading the frames back
// ---------------------------------------------------------------------------------------

/// Eight-bit straight-alpha RGBA samples, or a sentence saying why there are none. Never a
/// panic: a build that writes no file has to fail a named row rather than crash the table.
fn decode(path: &Path) -> Result<Vec<u8>, String> {
    let file =
        fs::File::open(path).map_err(|e| format!("{} was not written: {e}", path.display()))?;
    let mut reader = png::Decoder::new(std::io::BufReader::new(file))
        .read_info()
        .map_err(|e| format!("{} is not a readable PNG: {e}", path.display()))?;
    let mut samples = vec![0u8; reader.output_buffer_size().expect("a sized buffer")];
    let info = reader
        .next_frame(&mut samples)
        .map_err(|e| format!("{} does not decode: {e}", path.display()))?;
    if info.width as usize != WIDTH || info.height as usize != HEIGHT {
        return Err(format!(
            "{} is {} by {}",
            path.display(),
            info.width,
            info.height
        ));
    }
    samples.truncate(info.buffer_size());
    Ok(samples)
}

/// "identical", or a sentence saying how the two files differ.
fn same_bytes(a: &Path, b: &Path) -> Result<(), String> {
    match (fs::read(a), fs::read(b)) {
        (Ok(x), Ok(y)) if x == y => Ok(()),
        (Ok(x), Ok(y)) => Err(format!(
            "{} and {} differ: {} bytes against {}",
            a.display(),
            b.display(),
            x.len(),
            y.len()
        )),
        (Err(e), _) => Err(format!("{} could not be read: {e}", a.display())),
        (_, Err(e)) => Err(format!("{} could not be read: {e}", b.display())),
    }
}

// ---------------------------------------------------------------------------------------
// The contact sheet
// ---------------------------------------------------------------------------------------

fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn linear_to_srgb(c: f32) -> f32 {
    if c <= 0.003_130_8 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

/// One frame reduced by `THUMB` in each direction, averaged in linear light with premultiplied
/// alpha, written into `sheet` at grid cell `cell`.
fn thumbnail_into(sheet: &mut [u8], sheet_width: usize, cell: usize, samples: &[u8]) {
    let (tw, th) = (WIDTH / THUMB, HEIGHT / THUMB);
    let (ox, oy) = ((cell % COLUMNS) * tw, (cell / COLUMNS) * th);
    let count = (THUMB * THUMB) as f32;
    for ty in 0..th {
        for tx in 0..tw {
            let mut acc = [0.0f32; 4];
            for sy in 0..THUMB {
                for sx in 0..THUMB {
                    let at = (((ty * THUMB + sy) * WIDTH) + tx * THUMB + sx) * 4;
                    let a = samples[at + 3] as f32 / 255.0;
                    for c in 0..3 {
                        acc[c] += srgb_to_linear(samples[at + c] as f32 / 255.0) * a;
                    }
                    acc[3] += a;
                }
            }
            for v in acc.iter_mut() {
                *v /= count;
            }
            let out = ((oy + ty) * sheet_width + ox + tx) * 4;
            let quantise = |v: f32| (v.clamp(0.0, 1.0) * 255.0 + 0.5).floor() as u8;
            // Back to straight alpha, the way export unpremultiplies.
            for c in 0..3 {
                let straight = if acc[3] > 0.0 { acc[c] / acc[3] } else { 0.0 };
                sheet[out + c] = quantise(linear_to_srgb(straight));
            }
            sheet[out + 3] = quantise(acc[3]);
        }
    }
}

fn write_png(path: &Path, width: usize, height: usize, samples: &[u8]) {
    let file = fs::File::create(path).unwrap_or_else(|e| panic!("create {}: {e}", path.display()));
    let mut encoder = png::Encoder::new(std::io::BufWriter::new(file), width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().expect("a PNG header");
    writer.write_image_data(samples).expect("the sheet writes");
}

// ---------------------------------------------------------------------------------------
// The test
// ---------------------------------------------------------------------------------------

#[test]
#[ignore = "renders and encodes 480 full-resolution frames; run by name"]
fn b10_the_whole_shot_exports_the_same_twice() {
    let mut report = Report::default();
    let project = build_project();

    let dir1 = pass_dir("pass1");
    let dir2 = pass_dir("pass2");
    let first = run(&project, &dir1);
    let second = run(&project, &dir2);

    // ---- the job ---------------------------------------------------------------------------
    report.check(
        "the whole shot is asked for and the whole shot is written",
        "240 frames requested, 240 written, completed",
        format!(
            "{} frames requested, {} written, {}",
            first.frames_requested,
            first.written.len(),
            if first.succeeded() {
                "completed"
            } else {
                "did not complete"
            }
        ),
    );
    report.check(
        "the second export asks for and writes the same number of frames",
        "240 frames requested, 240 written, completed",
        format!(
            "{} frames requested, {} written, {}",
            second.frames_requested,
            second.written.len(),
            if second.succeeded() {
                "completed"
            } else {
                "did not complete"
            }
        ),
    );

    let name_of = |report: &ExportReport, i: usize| {
        report
            .written
            .get(i)
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "no such file".to_string())
    };
    report.check(
        "the first and last file are named for their own frame, four digits wide",
        "shot_0000.png and shot_0239.png",
        format!(
            "{} and {}",
            name_of(&first, 0),
            name_of(&first, first.written.len().saturating_sub(1))
        ),
    );

    // ---- determinism, the declared exit ------------------------------------------------------
    let mut differ: Vec<String> = Vec::new();
    for frame in 0..FRAMES {
        let name = format!("shot_{frame:04}.png");
        if let Err(why) = same_bytes(&dir1.join(&name), &dir2.join(&name)) {
            differ.push(why);
        }
    }
    report.check(
        "every one of the 240 frames is byte for byte what a second export of the same project \
         produced",
        "all 240 pairs identical",
        if differ.is_empty() {
            "all 240 pairs identical".to_string()
        } else {
            format!("{} pairs differ; the first is {}", differ.len(), differ[0])
        },
    );

    // A comparison that cannot fail proves nothing. Pairing each frame with its neighbour must
    // find differences, or the 240 rows above are comparing a file with itself.
    let mut neighbours_same = 0usize;
    for frame in 0..FRAMES - 1 {
        let a = dir1.join(format!("shot_{frame:04}.png"));
        let b = dir2.join(format!("shot_{:04}.png", frame + 1));
        if same_bytes(&a, &b).is_ok() {
            neighbours_same += 1;
        }
    }
    // Layer 2 changes drawing on every single frame, so no two consecutive frames of this shot
    // are the same picture and none of the 239 shifted pairs may match.
    report.check(
        "the byte comparison can fail: each frame paired with the next frame's file instead",
        "0 of the 239 neighbouring pairs match",
        format!("{neighbours_same} of the 239 neighbouring pairs match"),
    );

    // ---- the shot's deliberate hole ----------------------------------------------------------
    let gaps = gap_frames();
    report.check(
        "the frames whose drawing the shot deliberately does not contain are the twenty the \
         cadence predicts",
        "14, 15, 38, 39, 62, 63, 86, 87, 110, 111, 134, 135, 158, 159, 182, 183, 206, 207, 230, 231",
        gaps.iter()
            .map(|f| f.to_string())
            .collect::<Vec<_>>()
            .join(", "),
    );

    let gap_diagnostics: Vec<_> = first
        .diagnostics
        .iter()
        .filter(|d| d.id == DiagnosticId::MediaSequenceGap)
        .collect();
    report.check(
        "twenty affected frames are reported as three in full and one summary, per D-25",
        "4 gap diagnostics",
        format!("{} gap diagnostics", gap_diagnostics.len()),
    );
    report.check(
        "the summary names every affected frame and counts the ones it did not log",
        "Frames 14 to 15, 38 to 39, 62 to 63, 86 to 87, 110 to 111, 134 to 135, 158 to 159, \
         182 to 183, 206 to 207, 230 to 231. 17 further identical warnings were not logged \
         individually.",
        gap_diagnostics
            .last()
            .map(|d| d.detail.clone())
            .unwrap_or_else(|| "there is no summary".to_string()),
    );
    // The fidelity flag is document 28's signal for a *parked feature* bypassed on a layer that
    // was still drawn — a mask, today. A drawing that is missing is a different thing and is
    // reported as the warnings above: the layer is left out, not silently approximated. The
    // reference shot carries no mask, so the flag stays down and that is correct.
    report.check(
        "no parked feature was bypassed, so the fidelity flag stays down; the missing drawings are reported as the warnings above instead",
        "fidelity incomplete: false",
        format!("fidelity incomplete: {}", first.fidelity_incomplete),
    );
    let named_in_report = first
        .diagnostics
        .iter()
        .any(|d| d.detail.contains("230 to 231"));
    report.check(
        "a reader of the export report is told about the last affected frames, not just the first ones logged",
        "the report names frames 230 to 231: true",
        format!("the report names frames 230 to 231: {named_in_report}"),
    );

    // Layer 3's cels are drawn with `fill=(255, 220, 0, 255)` and no antialiasing
    // (`Fixtures/reference_shot/generate_cels.py` line 69), and no other layer of the shot
    // paints that colour: layer 2 is blurred scenery, layer 4 is green at half alpha, layer 1
    // is the background. So on a frame where layer 3 was left out, that exact colour cannot
    // appear anywhere — which is the missing drawing visible in the picture rather than only in
    // the log. Frames 14 and 15 are the first two of the twenty.
    //
    // The whole frame is scanned rather than one pixel. A single sample is not enough: a build
    // that substituted the nearest drawing for the missing one would still paint a bar, just in
    // the wrong place, and every drawing of layer 3 puts its bar somewhere else. That defect was
    // made on purpose (Y3) and passed a one-pixel version of this row.
    let layer3_pixels = |frame: i32| -> String {
        match decode(&dir1.join(format!("shot_{frame:04}.png"))) {
            Ok(s) => {
                let hits = s
                    .chunks_exact(4)
                    .filter(|p| *p == [255, 220, 0, 255])
                    .count();
                if hits == 0 {
                    "no layer 3 paint".to_string()
                } else {
                    "layer 3 paint present".to_string()
                }
            }
            Err(why) => why,
        }
    };
    report.check(
        "a frame missing its drawing has none of layer 3's paint anywhere in it",
        "frame 14: no layer 3 paint, frame 15: no layer 3 paint",
        format!(
            "frame 14: {}, frame 15: {}",
            layer3_pixels(14),
            layer3_pixels(15)
        ),
    );
    // The positive control, and the reason the row above cannot pass by accident: the frames on
    // either side of the gap are the same shot with layer 3 present, and they must contain that
    // colour. Only the presence or absence is asserted, never a count, because a count would be
    // read off a run rather than derived.
    report.check(
        "the frames either side of the gap do contain layer 3's paint",
        "frame 13: layer 3 paint present, frame 16: layer 3 paint present",
        format!(
            "frame 13: {}, frame 16: {}",
            layer3_pixels(13),
            layer3_pixels(16)
        ),
    );

    // ---- the contact sheet -------------------------------------------------------------------
    let (tw, th) = (WIDTH / THUMB, HEIGHT / THUMB);
    let rows = (FRAMES as usize).div_ceil(COLUMNS);
    let (sheet_w, sheet_h) = (tw * COLUMNS, th * rows);
    let mut sheet = vec![0u8; sheet_w * sheet_h * 4];
    let mut unreadable: Vec<String> = Vec::new();
    for frame in 0..FRAMES {
        match decode(&dir1.join(format!("shot_{frame:04}.png"))) {
            Ok(samples) => thumbnail_into(&mut sheet, sheet_w, frame as usize, &samples),
            Err(why) => unreadable.push(why),
        }
    }
    report.check(
        "every exported frame reads back as a 1920 by 1080 PNG",
        "240 of 240 readable",
        format!(
            "{} of 240 readable{}",
            FRAMES as usize - unreadable.len(),
            if unreadable.is_empty() {
                String::new()
            } else {
                format!("; the first fault is {}", unreadable[0])
            }
        ),
    );
    write_png(
        &repo("verification/B-10_contact_sheet.png"),
        sheet_w,
        sheet_h,
        &sheet,
    );
    report.check(
        "the contact sheet holds all 240 frames in a 16 by 15 grid",
        "2560 by 1350, 240 cells",
        format!("{sheet_w} by {sheet_h}, {} cells", COLUMNS * rows),
    );

    write_report(&report, &dir1);
    let failed: Vec<&Row> = report.rows.iter().filter(|r| !r.pass()).collect();
    assert!(
        failed.is_empty(),
        "{} of {} checks failed, first: {} (expected {}, got {}); see verification/B-10_full_shot_table.md",
        failed.len(),
        report.rows.len(),
        failed[0].check,
        failed[0].expected,
        failed[0].actual
    );
}

fn write_report(report: &Report, frames: &Path) {
    let passed = report.rows.iter().filter(|r| r.pass()).count();
    let mut out = String::new();
    out.push_str("# B-10 full shot: 240 frames exported twice\n\n");
    out.push_str(
        "Document 15 asks B-10 for \"the exported 240-frame sequence, plus a byte comparison of \
         two consecutive exports proving determinism\". This is that comparison. Produced by \
         `tests/b10_full_shot.rs`, which is `#[ignore]`d in normal runs and run by name.\n\n",
    );
    out.push_str(&format!(
        "The frames themselves are not committed: one frame of the assembled shot is about two \
         megabytes, so the sequence is 480 megabytes across the two passes. They are written to \
         `{}` and left there, so they can be opened or flipped through after a run. What is \
         committed in their place is `B-10_contact_sheet.png`, which is all 240 frames at a \
         twelfth scale in a 16 by 15 grid, reading left to right and top to bottom.\n\n",
        frames
            .strip_prefix(env!("CARGO_MANIFEST_DIR"))
            .unwrap_or(frames)
            .display()
            .to_string()
            .replace('\\', "/")
    ));
    out.push_str(
        "**Ten pairs of cells in the contact sheet are missing their yellow bar. That is \
         correct.** Layer 3 has no drawing 7 — a deliberate defect of the reference shot per \
         `Fixtures/reference_shot/README.md` — and the twenty frames that ask for it are \
         written with that layer left out and warned about, which is D-28's recorded override. \
         Nothing was substituted for the missing drawing.\n\n",
    );
    out.push_str("## Checks\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
    for row in &report.rows {
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            row.check,
            row.expected,
            row.actual,
            if row.pass() { "pass" } else { "**FAIL**" }
        ));
    }
    out.push_str(&format!(
        "\n**{passed} of {} checks pass.**\n",
        report.rows.len()
    ));
    let path = repo("verification/B-10_full_shot_table.md");
    fs::write(&path, out).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
}
