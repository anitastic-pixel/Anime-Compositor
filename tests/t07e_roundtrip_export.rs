//! T-07, export half: a project saved, reopened from its file, and exported to the same files.
//!
//! Writes `verification/T-07e_roundtrip_table.md`, `verification/T-07e_project.json` and
//! `verification/T-07e_reopened.json`.
//!
//! # What this proves that nothing before it could
//!
//! B-09 proved a project file survives being opened and saved: the text comes back byte for
//! byte. T-08 proved a project in memory exports to PNGs that obey documents 07, 21 and 28.
//! Neither of them joined the two, and the join is where a persistence fault would actually
//! hurt: a field that is dropped, rounded or reordered on the way through the file does not
//! corrupt the text — B-09 would catch that — it corrupts the *picture*, days later, in an
//! export nobody compares against anything.
//!
//! So this exports the same range twice: once from the project as it was built in memory, and
//! once from the project read back out of the file on disk. Every file has to match its
//! counterpart byte for byte. Anything the file loses that the renderer reads — an exposure
//! sheet, an opacity, a layer's order, an asset's interpretation — makes two different pictures
//! and fails a named row.
//!
//! Document 11 line 80 is what this closes: "a project saved, reopened and exported to the same
//! files". Q-01, which line 24 lists alongside it, is a release-candidate gate rather than a
//! check — "no known reproducible project corruption in the release candidate" — and stays open
//! until there is a release candidate to say it about.
//!
//! # Where the expected values come from
//!
//! `Fixtures/reference_shot/generate_cels.py` and arithmetic done here, exactly as in T-08:
//!
//! ```text
//! layer 3, drawing i: a bar of (255, 220, 0, 255), x in [160i, 160i+159], y in [180, 419]
//! layer 3 has no drawing 7: that file is a deliberate defect of the shot
//! ```
//!
//! The layer runs on twos, so composition frame `f` shows drawing `f / 2 % 12`. Frame 12 shows
//! drawing 6, whose bar covers x in [960, 1119], and (1000, 300) is forty pixels inside it.
//! Exported at 50% opacity with straight alpha at eight bits, that pixel is `R 255, B 0, A 128`:
//! the working pixel is premultiplied `(0.5, ., 0, 0.5)`, unpremultiplying returns red to 1.0
//! which encodes to 255, and `floor(0.5 * 255 + 0.5) = 128`. Frames 14 and 15 ask for drawing 7,
//! which the fixture does not contain, so with the transparent override chosen they are written
//! and empty: `R 0, B 0, A 0`.
//!
//! No expected value here was read off a run of the code under test (ADR-009).
//!
//! # What is deliberately not here
//!
//! No new export behaviour. Everything about ranges, naming, cancellation, write failure and
//! alpha policy is T-08's table and is not repeated. This is one question only: does the file
//! give the renderer back what it was given?

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::export::{export_sequence, ExportReport, ExportRequest, MissingSource};
use anime_compositor::media::import_sequence;
use anime_compositor::model::{Asset, AssetKind, Composition, Id, Layer, Project, Prop, Value};
use anime_compositor::persist::{self, Preserved};
use anime_compositor::time::{ExposureMap, ExposureSpan, FrameRate};
use anime_compositor::{OutputAlpha, OutputDepth};

const COMP: &str = "comp-export";

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

/// A scratch folder, emptied first so a run never reads a previous run's files.
fn scratch(name: &str) -> PathBuf {
    let dir = repo("target/t07e").join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap_or_else(|e| panic!("make {}: {e}", dir.display()));
    dir
}

/// "identical", or a sentence saying how the two files differ. Never a panic: a build that
/// writes no file has to fail a named row rather than crash the table before it is printed.
fn same_bytes(a: &Path, b: &Path) -> String {
    match (fs::read(a), fs::read(b)) {
        (Ok(x), Ok(y)) if x == y => "identical".to_string(),
        (Ok(x), Ok(y)) => format!(
            "the two files differ: {} bytes against {}",
            x.len(),
            y.len()
        ),
        (Err(e), _) => format!("{} could not be read: {e}", a.display()),
        (_, Err(e)) => format!("{} could not be read: {e}", b.display()),
    }
}

// ---------------------------------------------------------------------------------------
// Reading a PNG back
// ---------------------------------------------------------------------------------------

struct Decoded {
    note: Option<String>,
    width: usize,
    samples: Vec<u8>,
}

impl Decoded {
    /// One pixel's red, blue and alpha at eight bits, or a sentence saying why there is none.
    fn rba(&self, x: usize, y: usize) -> String {
        if let Some(note) = &self.note {
            return note.clone();
        }
        if (y * self.width + x + 1) * 4 > self.samples.len() {
            return format!("the file has no pixel at ({x}, {y})");
        }
        let at = |c: usize| self.samples[(y * self.width + x) * 4 + c];
        format!("R {}, B {}, A {}", at(0), at(2), at(3))
    }
}

fn decode(path: &Path) -> Decoded {
    let missing = |what: String| Decoded {
        note: Some(what),
        width: 0,
        samples: Vec::new(),
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
    Decoded {
        note: None,
        width: info.width as usize,
        samples,
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

/// Deliberately the composition T-08 exported its artifact frames from: layer 3's cels on twos
/// at 50% opacity, with a switched-off layer beneath them. Building the same thing here is what
/// lets the round trip be checked against files that are already committed and already looked
/// at, rather than against a second copy of the same pictures.
fn build_project() -> Project {
    let mut project = Project::new(id("proj-t07e-roundtrip"));
    project.compositions.push(Composition::new(
        id(COMP),
        "export probe",
        1920,
        1080,
        FrameRate::new(24, 1).expect("24 fps"),
        0,
        240,
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

/// Frames 12 to 17, eight-bit straight alpha, with the missing drawing written as transparent:
/// the same request T-08 used for the frames it committed.
fn request(dir: &Path) -> ExportRequest {
    ExportRequest {
        composition: id(COMP),
        first_frame: 12,
        last_frame: 17,
        output_dir: dir.to_path_buf(),
        naming: "shot_%04d.png".to_string(),
        depth: OutputDepth::Eight,
        alpha: OutputAlpha::Straight,
        tile_size: DEFAULT_TILE_SIZE,
        missing: MissingSource::RenderTransparent,
    }
}

fn run(project: &Project, request: &ExportRequest) -> ExportReport {
    export_sequence(project, &root(), request, &AtomicBool::new(false))
}

// ---------------------------------------------------------------------------------------
// The test
// ---------------------------------------------------------------------------------------

#[test]
fn t07e_a_reopened_project_exports_the_same_files() {
    let mut report = Report::default();

    // ---- save, reopen from the file on disk, save again ------------------------------------
    let built = build_project();
    let saved = repo("verification/T-07e_project.json");
    let mut doc = Document::new(built.clone());
    persist::save(&saved, &mut doc, &Preserved::none()).expect("the project saves");

    let text = fs::read_to_string(&saved).expect("the saved project can be read back");
    let loaded = persist::load_str(&text).expect("the project this build wrote, it can open");
    let reopened = loaded.document.project().clone();

    let resaved = repo("verification/T-07e_reopened.json");
    let mut redoc = Document::new(reopened.clone());
    persist::save(&resaved, &mut redoc, &loaded.preserved).expect("the reopened project saves");

    report.check(
        "opening the saved project reports nothing wrong",
        "",
        loaded
            .warnings
            .iter()
            .map(|d| d.id.as_str().to_string())
            .collect::<Vec<_>>()
            .join(", "),
    );
    report.check(
        "saving the reopened project reproduces the file it came from, byte for byte",
        "identical",
        same_bytes(&saved, &resaved),
    );

    // ---- what the renderer will read, checked before the pictures are made ------------------
    let comp = reopened
        .composition(&id(COMP))
        .expect("the composition survived the round trip");
    report.check(
        "the reopened composition has the same size and length",
        "1920x1080, 240 frames from 0",
        format!(
            "{}x{}, {} frames from {}",
            comp.width, comp.height, comp.duration_frames, comp.start_frame
        ),
    );
    report.check(
        "both layers came back, in the order they composite in",
        "layer-hidden, layer-cels",
        comp.layers_in_order()
            .map(|l| l.id.as_str().to_string())
            .collect::<Vec<_>>()
            .join(", "),
    );
    report.check(
        "the switched-off layer is still in the file and still switched off",
        "false",
        comp.layers_in_order()
            .find(|l| l.id.as_str() == "layer-hidden")
            .map(|l| l.enabled.to_string())
            .unwrap_or_else(|| "there is no such layer".to_string()),
    );
    report.check(
        "the exposure sheet came back whole: 120 spans, two frames each",
        "120 spans, first drawing 0 over frames 0 to 1",
        comp.layers_in_order()
            .find(|l| l.id.as_str() == "layer-cels")
            .map(|l| {
                // A build that loses the sheet entirely has to fail this row with a sentence,
                // not crash the table before it is printed.
                let Some(s) = l.exposure_spans.first() else {
                    return "the layer came back with no exposure sheet at all".to_string();
                };
                format!(
                    "{} spans, first drawing {} over frames {} to {}",
                    l.exposure_spans.len(),
                    s.drawing_number,
                    s.start_frame,
                    s.end_frame_exclusive - 1
                )
            })
            .unwrap_or_else(|| "there is no such layer".to_string()),
    );
    report.check(
        "layer 3's asset still has no drawing 7, because that file is a deliberate defect",
        "0, 1, 2, 3, 4, 5, 6, 8, 9, 10, 11",
        reopened
            .assets
            .iter()
            .find(|a| a.id.as_str() == "asset-layer3")
            .expect("layer 3's asset")
            .frames
            .keys()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(", "),
    );

    // ---- the same range exported from each, and compared file by file ----------------------
    let before = scratch("in_memory");
    let after = scratch("reopened");
    let a = run(&built, &request(&before));
    let b = run(&reopened, &request(&after));

    report.check(
        "the project in memory exports six frames",
        "Completed, 6 files",
        format!("{:?}, {} files", a.status, a.written.len()),
    );
    report.check(
        "and the project read back out of the file exports six frames too",
        "Completed, 6 files",
        format!("{:?}, {} files", b.status, b.written.len()),
    );
    report.check(
        "both report the same warnings: the two frames whose drawing is missing",
        "MEDIA_SEQUENCE_GAP, MEDIA_SEQUENCE_GAP",
        b.diagnostics
            .iter()
            .map(|d| d.id.as_str().to_string())
            .collect::<Vec<_>>()
            .join(", "),
    );
    for frame in 12..=17 {
        let name = format!("shot_{frame:04}.png");
        report.check(
            &format!(
                "{name} is the same file whether it was exported before or after the round trip"
            ),
            "identical",
            same_bytes(&before.join(&name), &after.join(&name)),
        );
    }

    // ---- and the same as the frames already committed and already looked at ----------------
    report.check(
        "the frames exported from the file are the frames T-08 committed as its artifact",
        "identical",
        same_bytes(
            &repo("verification/T-08_frames/shot_0012.png"),
            &after.join("shot_0012.png"),
        ),
    );

    // ---- the pixels themselves, against values derived by hand ------------------------------
    report.check(
        "frame 12 exported from the file shows drawing 6's bar at half opacity",
        "R 255, B 0, A 128",
        decode(&after.join("shot_0012.png")).rba(1000, 300),
    );
    report.check(
        "frame 14, which asks for the drawing the fixture does not have, is empty there",
        "R 0, B 0, A 0",
        decode(&after.join("shot_0014.png")).rba(1000, 300),
    );
    report.check(
        "and frame 16, two drawings later, has moved the bar two bar-widths right",
        "R 0, B 0, A 0",
        decode(&after.join("shot_0016.png")).rba(1000, 300),
    );
    report.check(
        "which is where drawing 8's bar now is",
        "R 255, B 0, A 128",
        decode(&after.join("shot_0016.png")).rba(1320, 300),
    );

    // ---- document 07's default still applies to a project that came from a file -------------
    let blocked_dir = scratch("blocked");
    let mut blocking = request(&blocked_dir);
    blocking.missing = MissingSource::Block;
    let blocked = run(&reopened, &blocking);
    report.check(
        "by default the same range is refused, and nothing is written",
        "Blocked, 0 files",
        format!("{:?}, {} files", blocked.status, blocked.written.len()),
    );
    report.check(
        "naming the frames whose drawing is missing",
        "EXPORT_BLOCKED_MISSING_MEDIA: Frames 14 to 15.",
        blocked
            .diagnostics
            .first()
            .map(|d| format!("{}: {}", d.id.as_str(), d.detail))
            .unwrap_or_else(|| "there was no diagnostic".to_string()),
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
        "# T-07, export half — a project saved, reopened and exported to the same files\n\n\
         **{passed} of {} checks passed.**\n\n\
         Produced by `tests/t07e_roundtrip_export.rs`. Closes the half of T-07 that document 11 \
         line 80 said was still owed.\n\n\
         ## What this is\n\n\
         Saving a project and getting the same text back is one thing, and it is already \
         checked: `B-09_persistence_table.md`, 92 checks. Getting the same **picture** back is \
         a different thing, and it is the one that would hurt. A field that the file quietly \
         rounds, drops or reorders does not damage the text — it damages an export, weeks \
         later, in a shot nobody thought to compare against anything.\n\n\
         So the same six frames were exported twice: once from the project as it was built, and \
         once from the project read back out of a file on disk. Every pair had to match byte \
         for byte, and one of them had to match a file that was exported yesterday and is \
         already committed.\n\n\
         ## What to look at\n\n\
         - **`T-07e_project.json`** and **`T-07e_reopened.json`** — the project as it was \
         saved, and the same project after being reopened and saved again. Open both in any \
         diff tool. They are the same file. If your tool reports a single differing line, \
         something in this build is losing data.\n\
         - **`T-08_frames/shot_0012.png`** — one row here says that picture is exactly what \
         comes out of a project that has been through the file format. It was not exported \
         again for this table; it was compared against.\n\n\
         ## What is still owed on T-07\n\n\
         **Q-01** — \"no known reproducible project corruption in the release candidate\" — is \
         a statement about a release candidate, not a check that can be run. It stays open \
         until there is one.\n\n\
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
    fs::write(repo("verification/T-07e_roundtrip_table.md"), out).expect("write report");
}
