//! T-08: exporting a frame range of a composition to a PNG sequence.
//!
//! Writes `verification/T-08_export_table.md` and the frames under `verification/T-08_frames/`.
//!
//! # What this proves that nothing before it could
//!
//! B-08a turned a project file into pixels in memory. This turns those pixels into files a
//! person can open, and it is the first thing in the build that produces output meant to leave
//! the application. Everything it checks is a rule from a planning document rather than a
//! preference:
//!
//! - **Document 07: an export range is inclusive at both ends.** 0 through 239 is 240 files, and
//!   12 through 12 is one file. The off-by-one this guards against is the classic one, and it
//!   would show up as a shot one frame short.
//! - **Document 07: a missing drawing blocks a final export by default.** The reference shot has
//!   a hole in layer 3 on purpose, so a default export of a range containing it must refuse and
//!   say which frames, having written nothing.
//! - **Document 28: a write failure reports the completed frames and the failing path, and a
//!   cancellation preserves the completed-frame list and claims no success.**
//! - **Document 21 line 31: export converts the linear working RGB to the declared output
//!   encoding and writes straight alpha unless asked otherwise. A display transform is never
//!   baked in.**
//!
//! # Where the expected values come from
//!
//! `Fixtures/reference_shot/generate_cels.py`, the fixture's own record of how the cels were
//! drawn, and arithmetic done here by hand. The two facts it supplies:
//!
//! ```text
//! layer 3, drawing i: a bar of (255, 220, 0, 255), x in [160i, 160i+159], y in [180, 419]
//! layer 3 has no drawing 7: that file is a deliberate defect of the shot
//! ```
//!
//! The exported layer runs on twos, so composition frame `f` shows drawing `f / 2 % 12`. Frame
//! 12 shows drawing 6, whose bar covers x in [960, 1119]; every colour row samples (1000, 300),
//! which is forty pixels inside it on every side.
//!
//! The layer is exported at 50% opacity, which is what makes the alpha rules visible. In the
//! working space that pixel is premultiplied `(0.5, ., 0, 0.5)`. Written as **straight** alpha
//! it is unpremultiplied first, so red returns to 1.0 and encodes to 255, and alpha encodes to
//! `floor(0.5 * 255 + 0.5) = 128`. Written as **premultiplied** it is not, so red stays at
//! linear 0.5, and the sRGB transfer function of 0.5 is 0.735357, which encodes to
//! `floor(0.735357 * 255 + 0.5) = 188`. At sixteen bits the same straight pixel is 65535 and
//! `floor(0.5 * 65535 + 0.5) = 32768`. Those four numbers are the whole of document 21 line 31
//! in a form that can be read off a file.
//!
//! Green is not checked here, for the reason B-02's table gives: the transfer function is
//! checked to six places there, and 220 is not a round number in it. Red, blue and alpha are
//! exact.
//!
//! # What is deliberately not here
//!
//! **No video file.** R-09 asks for an image sequence and that is what this is. A video needs an
//! encoder, an encoder is a dependency and a licence, and both are the owner's decision, so the
//! question is registered rather than answered.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::export::{export_sequence, ExportReport, ExportRequest, MissingSource};
use anime_compositor::media::import_sequence;
use anime_compositor::model::{Asset, AssetKind, Composition, Id, Layer, Project, Prop, Value};
use anime_compositor::time::{ExposureMap, ExposureSpan, FrameRate};
use anime_compositor::{OutputAlpha, OutputDepth};

const COMP: &str = "comp-export";
const SMALL: &str = "comp-export-small";
const NEGATIVE: &str = "comp-export-negative";

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

/// A scratch folder for output that is not part of the artifact, emptied first so a run never
/// reads a previous run's files.
fn scratch(name: &str) -> PathBuf {
    let dir = repo("target/t08").join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap_or_else(|e| panic!("make {}: {e}", dir.display()));
    dir
}

/// The identifiers a report carries, in order, as one string.
fn ids(report: &ExportReport) -> String {
    report
        .diagnostics
        .iter()
        .map(|d| d.id.as_str().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

fn file_names(dir: &Path) -> String {
    let mut names: Vec<String> = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
        .map(|e| e.expect("entry").file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names.join(", ")
}

// ---------------------------------------------------------------------------------------
// Reading a PNG back
// ---------------------------------------------------------------------------------------

struct Decoded {
    /// Set when the file could not be read at all. Every accessor returns it instead of a
    /// number, so a build that fails to write a file fails a named row rather than crashing the
    /// table before it is printed.
    note: Option<String>,
    width: usize,
    height: usize,
    depth: u8,
    samples: Vec<u8>,
    tags: Vec<(String, String)>,
}

impl Decoded {
    /// One pixel's red, blue and alpha, at whatever depth the file declares.
    fn rba(&self, x: usize, y: usize) -> String {
        if let Some(note) = &self.note {
            return note.clone();
        }
        let per = if self.depth == 16 { 2 } else { 1 };
        if (y * self.width + x + 1) * 4 * per > self.samples.len() {
            return format!("the file has no pixel at ({x}, {y})");
        }
        let at = |c: usize| -> u32 {
            let i = ((y * self.width + x) * 4 + c) * per;
            if per == 2 {
                u16::from_be_bytes([self.samples[i], self.samples[i + 1]]) as u32
            } else {
                self.samples[i] as u32
            }
        };
        format!("R {}, B {}, A {}", at(0), at(2), at(3))
    }

    fn tag(&self, key: &str) -> String {
        if let Some(note) = &self.note {
            return note.clone();
        }
        self.tags
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone())
            .unwrap_or_else(|| "there is no such tag".to_string())
    }
}

fn decode(path: &Path) -> Decoded {
    let missing = |what: String| Decoded {
        note: Some(what),
        width: 0,
        height: 0,
        depth: 0,
        samples: Vec::new(),
        tags: Vec::new(),
    };
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(e) => {
            return missing(format!(
                "{} was not written: {e}",
                path.file_name().unwrap_or_default().to_string_lossy()
            ))
        }
    };
    let mut reader = match png::Decoder::new(std::io::BufReader::new(file)).read_info() {
        Ok(reader) => reader,
        Err(e) => return missing(format!("{} is not a readable PNG: {e}", path.display())),
    };
    let mut samples = vec![0u8; reader.output_buffer_size().expect("a sized buffer")];
    let info = match reader.next_frame(&mut samples) {
        Ok(info) => info,
        Err(e) => return missing(format!("{} does not decode: {e}", path.display())),
    };
    samples.truncate(info.buffer_size());
    let meta = reader.info();
    let tags = meta
        .utf8_text
        .iter()
        .map(|chunk| {
            (
                chunk.keyword.clone(),
                chunk.get_text().unwrap_or_default().to_string(),
            )
        })
        .collect();
    Decoded {
        note: None,
        width: info.width as usize,
        height: info.height as usize,
        depth: match info.bit_depth {
            png::BitDepth::Sixteen => 16,
            _ => 8,
        },
        samples,
        tags,
    }
}

// ---------------------------------------------------------------------------------------
// The project
// ---------------------------------------------------------------------------------------

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

fn on_twos() -> Vec<ExposureSpan> {
    let exposures: Vec<(u32, u32)> = (0..120).map(|k| (k % 12, 2)).collect();
    ExposureMap::from_lengths(&exposures)
        .expect("twelve drawings on twos is a valid sheet")
        .spans()
        .to_vec()
}

fn held(frames: u32) -> Vec<ExposureSpan> {
    ExposureMap::from_lengths(&[(0, frames)])
        .expect("one drawing held is a valid sheet")
        .spans()
        .to_vec()
}

/// Three compositions of the same layer 3 cels:
///
/// - `comp-export` at the shot's own size, the layer on twos, at 50% opacity. This is what the
///   colour rows and the artifact frames come from, and it carries layer 3's deliberate hole.
/// - `comp-export-small` at 320x180 with the drawing held, so the cancellation rows can export a
///   hundred frames without spending a minute on pixels that prove nothing.
/// - `comp-export-negative`, fixture FX-TIME-004's shape: starts at -12 and runs 24 frames.
///
/// `layer-hidden` exists only so the matte rows have another layer to point at. It is switched
/// off, so nothing it holds reaches the picture.
fn build_project() -> Project {
    let mut project = Project::new(id("proj-t08-export"));
    let fps = FrameRate::new(24, 1).expect("24 fps");
    project.compositions.push(Composition::new(
        id(COMP),
        "export probe",
        1920,
        1080,
        fps,
        0,
        240,
    ));
    project.compositions.push(Composition::new(
        id(SMALL),
        "export probe, small",
        320,
        180,
        fps,
        0,
        240,
    ));
    project.compositions.push(Composition::new(
        id(NEGATIVE),
        "export probe, starting before zero",
        320,
        180,
        fps,
        -12,
        24,
    ));

    let mut doc = Document::new(project);
    doc.apply_all(vec![
        Command::AddAsset {
            asset: asset_for(3),
        },
        Command::AddAsset {
            asset: asset_for(1),
        },
    ])
    .expect("both assets are valid");

    let mut hidden = Layer::new(id("layer-hidden"), "hidden", id("asset-layer1"), 0, 240);
    hidden.enabled = false;
    hidden.exposure_spans = held(240);
    let mut main = Layer::new(id("layer-cels"), "layer3", id("asset-layer3"), 0, 240);
    main.exposure_spans = on_twos();
    let mut small = Layer::new(id("layer-small"), "layer3", id("asset-layer3"), 0, 240);
    small.exposure_spans = held(240);
    let mut negative = Layer::new(id("layer-negative"), "layer3", id("asset-layer3"), -12, 12);
    negative.exposure_spans = held(24);

    doc.apply_all(vec![
        Command::AddLayer {
            composition: id(COMP),
            layer: Box::new(hidden),
            index: 0,
        },
        Command::AddLayer {
            composition: id(COMP),
            layer: Box::new(main),
            index: 1,
        },
        Command::AddLayer {
            composition: id(SMALL),
            layer: Box::new(small),
            index: 0,
        },
        Command::AddLayer {
            composition: id(NEGATIVE),
            layer: Box::new(negative),
            index: 0,
        },
        // Document 21 step 6 at half strength, so straight and premultiplied alpha differ in
        // the file and the difference can be read off it.
        Command::SetPropertyBase {
            composition: id(COMP),
            layer_id: id("layer-cels"),
            prop: Prop::Opacity,
            value: Value::Scalar(0.5),
        },
    ])
    .expect("the opening project is valid");
    doc.project().clone()
}

/// The request every row starts from: one composition, one folder, straight eight-bit colour,
/// and document 07's default for a missing drawing.
fn request(composition: &str, first: i32, last: i32, dir: &Path, naming: &str) -> ExportRequest {
    ExportRequest {
        composition: id(composition),
        first_frame: first,
        last_frame: last,
        output_dir: dir.to_path_buf(),
        naming: naming.to_string(),
        depth: OutputDepth::Eight,
        alpha: OutputAlpha::Straight,
        tile_size: DEFAULT_TILE_SIZE,
        missing: MissingSource::Block,
    }
}

fn run(project: &Project, request: &ExportRequest) -> ExportReport {
    export_sequence(project, &root(), request, &AtomicBool::new(false))
}

// ---------------------------------------------------------------------------------------
// The test
// ---------------------------------------------------------------------------------------

#[test]
fn t08_exports_a_frame_range_to_a_png_sequence() {
    let mut report = Report::default();
    let project = build_project();

    // ---- document 07: the range is inclusive at both ends ---------------------------------
    // Asked for with a naming pattern that has no frame number, so the count is reported and
    // nothing is planned or written: the arithmetic is what this row is about.
    let dir = scratch("naming");
    let no_number = run(&project, &request(COMP, 0, 239, &dir, "everyframe.png"));
    report.check(
        "frames 0 to 239 is 240 files, because document 07 includes both ends",
        240,
        no_number.frames_requested,
    );
    report.check(
        "a naming pattern with no frame number is refused before anything is written",
        "COMMAND_INVALID_VALUE, 0 files",
        format!(
            "{}, {} files",
            ids(&no_number),
            fs::read_dir(&dir).unwrap().count()
        ),
    );
    report.check(
        "and the refusal says why, in the owner's words",
        "The output naming everyframe.png contains no frame number, so every frame would be \
         written to one file.",
        no_number
            .diagnostics
            .first()
            .map(|d| d.message.clone())
            .unwrap_or_default(),
    );

    let dir = scratch("reversed");
    let reversed = run(&project, &request(COMP, 20, 12, &dir, "shot_%04d.png"));
    report.check(
        "a range that ends before it starts is refused, not silently swapped",
        "Failed, COMMAND_INVALID_VALUE, 0 files",
        format!(
            "{:?}, {}, {} files",
            reversed.status,
            ids(&reversed),
            fs::read_dir(&dir).unwrap().count()
        ),
    );
    report.check(
        "a refused export never reports success",
        false,
        reversed.succeeded(),
    );

    // ---- document 07: a missing drawing blocks the export ---------------------------------
    // Frames 0 to 47 of this composition show drawings 0 to 11 twice over. Drawing 7 is the
    // hole in the fixture, and the layer is on twos, so the frames without a drawing are
    // 14, 15 and then 38, 39.
    let dir = scratch("blocked");
    let blocked = run(&project, &request(COMP, 0, 47, &dir, "shot_%04d.png"));
    report.check(
        "an export whose range contains a missing drawing is blocked, and writes nothing",
        "Blocked, 0 files",
        format!(
            "{:?}, {} files",
            blocked.status,
            fs::read_dir(&dir).unwrap().count()
        ),
    );
    report.check(
        "a blocked export does not report success",
        false,
        blocked.succeeded(),
    );
    report.check(
        "it counts the frames it could not draw against the frames asked for",
        "4 of the 48 frames asked for have a drawing that is missing, so nothing was exported.",
        blocked
            .diagnostics
            .first()
            .map(|d| d.message.clone())
            .unwrap_or_default(),
    );
    report.check(
        "and names them, as ranges rather than as a list of forty-eight",
        "Frames 14 to 15, 38 to 39.",
        blocked
            .diagnostics
            .first()
            .map(|d| d.detail.clone())
            .unwrap_or_default(),
    );
    report.check(
        "under an identifier that says what happened",
        "EXPORT_BLOCKED_MISSING_MEDIA",
        blocked
            .diagnostics
            .first()
            .map(|d| d.id.as_str().to_string())
            .unwrap_or_default(),
    );

    // ---- the artifact: frames 12 to 17, written with the hole left in ----------------------
    // The same range, exported with the missing drawing chosen as transparent instead. This is
    // the override document 07 requires to be a recorded choice, and it still warns.
    let frames_dir = repo("verification/T-08_frames");
    let _ = fs::remove_dir_all(&frames_dir);
    fs::create_dir_all(&frames_dir).expect("make the artifact folder");
    let mut req = request(COMP, 12, 17, &frames_dir, "shot_%04d.png");
    req.missing = MissingSource::RenderTransparent;
    let written = run(&project, &req);
    report.check(
        "the same six frames export when the missing drawing is chosen to be transparent",
        "Completed, 6 files",
        format!("{:?}, {} files", written.status, written.written.len()),
    );
    report.check(
        "and that is the only status that reports success",
        true,
        written.succeeded(),
    );
    report.check(
        "the files are named from the frame number, padded to the pattern's width",
        "shot_0012.png, shot_0013.png, shot_0014.png, shot_0015.png, shot_0016.png, \
         shot_0017.png",
        file_names(&frames_dir),
    );
    report.check(
        "the two frames with no drawing are warned about, once each",
        "MEDIA_SEQUENCE_GAP, MEDIA_SEQUENCE_GAP",
        ids(&written),
    );
    // The layer runs on twos, so frames 14 and 15 are the two that ask for drawing 7, and
    // drawing 7 is the hole in the fixture. Each warning has to say which frame and which
    // drawing, or it cannot be acted on.
    report.check(
        "and each warning names its own frame and the drawing that is missing",
        "14 asks for 7, 15 asks for 7",
        written
            .diagnostics
            .iter()
            .map(|d| {
                let named = [14, 15]
                    .iter()
                    .find(|f| d.message.contains(&format!("Frame {f} exposes drawing 7")));
                match named {
                    Some(f) => format!("{f} asks for 7"),
                    None => format!("a warning that names no frame: {}", d.message),
                }
            })
            .collect::<Vec<_>>()
            .join(", "),
    );
    report.check(
        "a missing drawing is not a bypassed feature, so fidelity is not marked incomplete",
        false,
        written.fidelity_incomplete,
    );

    let f12 = decode(&frames_dir.join("shot_0012.png"));
    let f14 = decode(&frames_dir.join("shot_0014.png"));
    report.check(
        "an exported frame is the composition's own size",
        "1920x1080",
        format!("{}x{}", f12.width, f12.height),
    );
    report.check(
        "frame 12 shows drawing 6, whose bar covers x 960 to 1119",
        "R 255, B 0, A 128",
        f12.rba(1000, 300),
    );
    report.check(
        "and nothing is there two bar-widths to the left, where drawing 4's bar would be",
        "R 0, B 0, A 0",
        f12.rba(700, 300),
    );
    report.check(
        "frame 14 has no drawing, and nothing was substituted for the one that is missing",
        "R 0, B 0, A 0",
        f14.rba(1000, 300),
    );
    report.check(
        "an exported file says what wrote it",
        "anime_compositor export (R-09)",
        f12.tag("Software"),
    );
    report.check(
        "and what its numbers mean, so a person opening it later is not guessing",
        "sRGB IEC 61966-2-1, 8 bits per channel / Straight / 12",
        format!(
            "{} / {} / {}",
            f12.tag("ColorSpace"),
            f12.tag("AlphaMode"),
            f12.tag("Frame")
        ),
    );

    // ---- document 21 line 31: the declared encoding, and alpha as asked -------------------
    let dir = scratch("premultiplied");
    let mut req = request(COMP, 12, 12, &dir, "shot_%04d.png");
    req.alpha = OutputAlpha::Premultiplied;
    run(&project, &req);
    report.check(
        "asking for one frame, 12 to 12, writes one file",
        "shot_0012.png",
        file_names(&dir),
    );
    report.check(
        "written premultiplied, the same pixel keeps alpha folded into the colour: linear 0.5 \
         through the sRGB curve is 188, not 255",
        "R 188, B 0, A 128",
        decode(&dir.join("shot_0012.png")).rba(1000, 300),
    );

    let dir = scratch("sixteen");
    let mut req = request(COMP, 12, 12, &dir, "shot_%04d.png");
    req.depth = OutputDepth::Sixteen;
    run(&project, &req);
    let deep = decode(&dir.join("shot_0012.png"));
    report.check(
        "asked for sixteen bits, the file declares sixteen bits",
        16,
        deep.depth,
    );
    report.check(
        "and the same straight pixel is the same colour at the deeper precision",
        "R 65535, B 0, A 32768",
        deep.rba(1000, 300),
    );
    report.check(
        "the eight-bit file declares eight, so the two are not the same file with a label",
        8,
        f12.depth,
    );

    // ---- the file is the render, not a second opinion about it -----------------------------
    let mut log = FrameLog::new(3);
    let rendered = render_frame(
        &project,
        &id(COMP),
        12,
        &root(),
        DEFAULT_TILE_SIZE,
        &mut log,
    )
    .expect("frame 12 renders");
    report.check(
        "the samples in the exported file are exactly what the renderer produced, byte for byte",
        "identical",
        if f12.samples == rendered.encode(OutputDepth::Eight, OutputAlpha::Straight) {
            "identical"
        } else {
            "the file and the render disagree"
        },
    );
    let dir = scratch("again");
    run(&project, &request(COMP, 12, 12, &dir, "shot_%04d.png"));
    report.check(
        "exporting the same frame twice produces the same file, byte for byte",
        "identical",
        if fs::read(dir.join("shot_0012.png")).unwrap()
            == fs::read(frames_dir.join("shot_0012.png")).unwrap()
        {
            "identical"
        } else {
            "the two exports differ"
        },
    );

    // ---- fixture FX-TIME-004: a composition that starts before zero ------------------------
    let dir = scratch("negative");
    let negative = run(&project, &request(NEGATIVE, -12, 11, &dir, "neg_%04d.png"));
    report.check(
        "a composition starting at -12 and lasting 24 frames exports 24 files (FX-TIME-004)",
        "Completed, 24 files",
        format!("{:?}, {} files", negative.status, negative.written.len()),
    );
    report.check(
        "a negative frame number keeps its sign in front of the padded digits (D-29)",
        "neg_-0012.png",
        negative
            .written
            .first()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default(),
    );
    report.check(
        "and the last file is the last frame of the range, included",
        "neg_0011.png",
        negative
            .written
            .last()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default(),
    );

    // ---- document 28: a write failure names the path and the completed count ---------------
    // Frame 14's file name is taken by a folder, which is a write failure this test can cause
    // on any machine and any file system.
    let dir = scratch("writefail");
    fs::create_dir(dir.join("shot_0014.png")).expect("occupy the file name");
    let mut req = request(COMP, 12, 17, &dir, "shot_%04d.png");
    req.missing = MissingSource::RenderTransparent;
    let failed = run(&project, &req);
    report.check(
        "an export that cannot write a file stops, and keeps the frames it finished",
        "Failed, 2 written",
        format!("{:?}, {} written", failed.status, failed.written.len()),
    );
    report.check("it never reports success", false, failed.succeeded());
    report.check(
        "it names the frame and the path that failed",
        "EXPORT_WRITE_FAILED: Frame 14 could not be written.",
        failed
            .diagnostics
            .iter()
            .find(|d| d.id.as_str() == "EXPORT_WRITE_FAILED")
            .map(|d| {
                format!(
                    "{}: {}",
                    d.id.as_str(),
                    d.message
                        .split(" to ")
                        .next()
                        .unwrap_or_default()
                        .to_string()
                        + "."
                )
            })
            .unwrap_or_else(|| "no write failure was reported".to_string()),
    );
    report.check(
        "and says how many frames had been written when it happened",
        true,
        failed
            .diagnostics
            .iter()
            .any(|d| d.detail.contains("2 of 6 frames had been written")),
    );
    report.check(
        "the two frames it did write are still on disk and still readable",
        "shot_0012.png 1920x1080, shot_0013.png 1920x1080",
        failed
            .written
            .iter()
            .map(|p| {
                let d = decode(p);
                format!(
                    "{} {}x{}",
                    p.file_name().unwrap().to_string_lossy(),
                    d.width,
                    d.height
                )
            })
            .collect::<Vec<_>>()
            .join(", "),
    );

    // ---- document 03 and 28: cancellation ---------------------------------------------------
    let dir = scratch("cancel-before");
    let cancelled_at_once = export_sequence(
        &project,
        &root(),
        &request(SMALL, 0, 99, &dir, "small_%04d.png"),
        &AtomicBool::new(true),
    );
    report.check(
        "an export cancelled before it starts writes nothing and claims nothing",
        "Cancelled, 0 files, EXPORT_CANCELLED",
        format!(
            "{:?}, {} files, {}",
            cancelled_at_once.status,
            fs::read_dir(&dir).unwrap().count(),
            ids(&cancelled_at_once)
        ),
    );
    report.check(
        "a cancelled export is not a successful one",
        false,
        cancelled_at_once.succeeded(),
    );

    // Cancelled while running: the flag is set by another thread once two files exist, which is
    // the shape a stop button has.
    let dir = scratch("cancel-during");
    let cancel = AtomicBool::new(false);
    let midway = std::thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..30_000 {
                if fs::read_dir(&dir).map(|d| d.count()).unwrap_or(0) >= 2 {
                    break;
                }
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            cancel.store(true, Ordering::SeqCst);
        });
        export_sequence(
            &project,
            &root(),
            &request(SMALL, 0, 99, &dir, "small_%04d.png"),
            &cancel,
        )
    });
    report.check(
        "an export cancelled while it is running stops and reports it",
        "Cancelled, EXPORT_CANCELLED",
        format!("{:?}, {}", midway.status, ids(&midway)),
    );
    report.check(
        "it stopped short of the hundred frames asked for",
        true,
        midway.written.len() < 100 && !midway.written.is_empty(),
    );
    report.check(
        "and every frame it had finished is a whole file, because the check is between frames",
        "all 320x180",
        {
            let whole = midway
                .written
                .iter()
                .filter(|p| {
                    let d = decode(p);
                    d.width == 320 && d.height == 180
                })
                .count();
            if whole == midway.written.len() {
                "all 320x180".to_string()
            } else {
                format!("{whole} of {} are whole files", midway.written.len())
            }
        },
    );
    report.check(
        "the frames it wrote are still on disk after the stop",
        midway.written.len(),
        fs::read_dir(&dir).unwrap().count(),
    );

    // ---- document 28: output produced with a feature bypassed says so ----------------------
    let mut doc = Document::new(project.clone());
    doc.apply(Command::SetMatte {
        composition: id(COMP),
        layer_id: id("layer-cels"),
        matte: Some(id("layer-hidden")),
    })
    .expect("a matte naming a layer that exists is a valid command");
    let with_matte = doc.project().clone();
    let dir = scratch("matte");
    let bypassed = run(&with_matte, &request(COMP, 12, 12, &dir, "shot_%04d.png"));
    report.check(
        "a frame drawn without a parked feature still exports",
        "Completed, 1 written",
        format!("{:?}, {} written", bypassed.status, bypassed.written.len()),
    );
    report.check(
        "but the report says the fidelity is incomplete",
        true,
        bypassed.fidelity_incomplete,
    );
    report.check(
        "and so does the file itself, where it cannot be separated from the picture",
        "incomplete: a layer carrying a parked feature was drawn without it",
        decode(&dir.join("shot_0012.png")).tag("Fidelity"),
    );
    report.check(
        "a frame with nothing bypassed carries no such tag",
        "there is no such tag",
        f12.tag("Fidelity"),
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
        "# T-08 — exporting a frame range to a PNG sequence\n\n\
         **{passed} of {} checks passed.**\n\n\
         Produced by `tests/t08_export.rs`. Covers R-09: a declared inclusive frame range, \
         written as PNGs with a chosen bit depth, naming and alpha policy, with failure \
         reported and cancellation supported between frames.\n\n\
         ## What this is\n\n\
         B-08a made a picture in memory. This writes the picture to files, and it is the first \
         output this build produces that is meant to leave the application. The six frames in \
         `verification/T-08_frames/` were exported by the code these checks describe.\n\n\
         ## What to look at\n\n\
         - **`T-08_frames/shot_0014.png`** — read this one first, because it should look empty \
         and that is correct. Frame 14 asks for drawing 7 of layer 3, which your fixture \
         deliberately does not contain. The file was written, it is the right size, and it has \
         nothing in it. Nothing was substituted.\n\
         - **`T-08_frames/shot_0012.png`** and **`shot_0016.png`** — the bar moves right by one \
         bar-width between them, because the layer runs on twos and these are two drawings \
         apart. The bar is half-transparent: the layer is exported at 50% opacity on purpose, \
         because that is what makes the alpha rules in document 21 visible in a file.\n\
         - **`T-08_export_table.md`**, this table.\n\n\
         ## The rule that will matter most to you\n\n\
         By default, **an export whose range contains a frame with no drawing is refused before \
         a single file is written**, and it tells you exactly which frames are missing. That is \
         document 07's rule and this build obeys it. Writing those frames anyway, with the \
         layer left out, is possible but has to be asked for, and it still warns.\n\n\
         Document 28 says the opposite for the same situation. That conflict is registered as \
         **D-28** and is yours to settle; nothing here depends on which way it goes except one \
         default.\n\n\
         ## What is not here\n\n\
         **There is no video file.** R-09 asks for an image sequence and that is what was \
         built. A video needs an encoder, and an encoder is a dependency and a licence \
         decision, which is yours rather than the code's. That question is registered rather \
         than answered.\n\n\
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
    fs::write(repo("verification/T-08_export_table.md"), out).expect("write report");
}
