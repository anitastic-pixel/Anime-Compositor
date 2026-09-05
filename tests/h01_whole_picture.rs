//! H-01: the whole picture, checked against an independent compositor rather than by sampling.
//!
//! Writes `verification/H-01_whole_picture_table.md`, `verification/H-01_independent_frame.png`
//! and `verification/H-01_renderer_frame.png`.
//!
//! # The gap this closes
//!
//! `verification/HARDENING_mutation_report.md` says it plainly: every table in this build reads
//! **named pixels and named values**. A break that moved every layer one pixel across, dimmed
//! the whole frame by a hair, or swapped red and blue would pass all 137 deliberate defects and
//! every row of every table, because no check looks at the image as a whole. The reference shot
//! is compared by eye, and an eye does not see one part in a thousand.
//!
//! So this compares the image as a whole, and it does it against a second compositor written
//! here, from document 21, on purpose:
//!
//! - no tiles and no threads: one loop over 2,073,600 pixels, bottom layer first;
//! - no code shared with the renderer — the PNGs are decoded straight from the `png` crate, the
//!   transfer function is written out longhand, and the composite is the formula as document 21
//!   line 57 states it;
//! - no expected value read off a run of the renderer (ADR-009). The oracle is the
//!   specification, expressed twice.
//!
//! Two implementations of the same document can only agree by both being right or by being
//! wrong in exactly the same way, and the second is a great deal less likely than a shift, a
//! dim or a channel swap slipping past a table of sampled pixels.
//!
//! # Where the expected values come from
//!
//! - Document 21 line 57, the authoritative normal composite for premultiplied source over
//!   destination: `Co = Cs + Cd*(1-As)`, `Ao = As + Ad*(1-As)`.
//! - Document 21 line 29: PNG alpha is straight, sRGB RGB is converted to linear light before
//!   premultiplication.
//! - D-17, the sRGB transfer function this build uses: `c/12.92` for `c <= 0.04045`, and
//!   `((c+0.055)/1.055)^2.4` above it.
//! - Document 21 line 37, the layer order: decode, then transform, then opacity, then
//!   composite. Every layer of the reference shot sits at the identity transform, so step 4
//!   moves nothing and no resampling rule is being relied on here. That is deliberate: this
//!   test is about the picture as a whole, and bilinear sampling has its own table in B-05a.
//! - Document 22's cadences and `Fixtures/reference_shot/exposure_sheet.json` for which drawing
//!   each layer shows: layer 1 held, layer 2 on ones over 24 drawings, layer 3 on twos over 12,
//!   layer 4 from the sheet. The arithmetic is done here rather than asked of `src/time.rs`.
//!
//! # What "identical" means here
//!
//! Every channel of every pixel, compared exactly. Not a tolerance, not a sample, not a
//! checksum: the two buffers are walked together and any difference at all is counted and the
//! largest one is reported. Both compositors work in float32 and apply the same operations in
//! the same order, so exact agreement is the honest expectation; a tolerance here would be a
//! place for a real fault to hide.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::media::import_sequence;
use anime_compositor::model::{Asset, AssetKind, Composition, Id, Layer, Project};
use anime_compositor::time::{ExposureMap, ExposureSpan, FrameRate};
use anime_compositor::WorkingBuffer;

const COMP: &str = "comp-reference-shot";
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

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
// The independent compositor
// ---------------------------------------------------------------------------------------

/// D-17's transfer function, written out rather than called, because a shared helper would
/// make the two compositors agree about colour by construction.
fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// One cel, decoded to linear premultiplied RGBA exactly as document 21 line 29 describes:
/// straight alpha, sRGB RGB converted to linear light, then premultiplied.
///
/// Returns `None` when the file does not exist, which is what layer 3's deliberate hole looks
/// like from here.
fn decode_linear(path: &Path) -> Option<Vec<[f32; 4]>> {
    let file = fs::File::open(path).ok()?;
    let mut reader = png::Decoder::new(std::io::BufReader::new(file))
        .read_info()
        .unwrap_or_else(|e| panic!("{} is not a readable PNG: {e}", path.display()));
    let mut bytes = vec![0u8; reader.output_buffer_size().expect("a sized buffer")];
    let info = reader
        .next_frame(&mut bytes)
        .unwrap_or_else(|e| panic!("{} does not decode: {e}", path.display()));
    assert_eq!(
        (info.width as usize, info.height as usize),
        (WIDTH, HEIGHT),
        "{} is not the composition's size",
        path.display()
    );
    assert_eq!(
        info.color_type,
        png::ColorType::Rgba,
        "{} is not RGBA",
        path.display()
    );
    Some(
        bytes
            .chunks_exact(4)
            .map(|p| {
                let a = p[3] as f32 / 255.0;
                [
                    srgb_to_linear(p[0] as f32 / 255.0) * a,
                    srgb_to_linear(p[1] as f32 / 255.0) * a,
                    srgb_to_linear(p[2] as f32 / 255.0) * a,
                    a,
                ]
            })
            .collect(),
    )
}

/// Which drawing each layer shows at a composition frame, from document 22's cadences and the
/// exposure sheet. Layer 1 is held; layer 2 runs on ones over 24 drawings; layer 3 on twos over
/// 12; layer 4's irregular timing is read from the sheet and accumulated here.
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

/// The file holding one layer's drawing, found by reading the folder rather than by asking the
/// importer, so the oracle does not depend on the naming rules it is checking beside.
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

/// The whole frame, composited from scratch: four layers, bottom first, one pixel at a time.
///
/// This is document 21 lines 37 and 57 and nothing else. No tiles, no threads, no shared code.
fn independent_frame(frame: i32) -> Vec<[f32; 4]> {
    let mut out = vec![[0.0f32; 4]; WIDTH * HEIGHT];
    for layer in 1..=4u32 {
        let drawing = drawing_at(layer, frame);
        let Some(path) = cel_path(layer, drawing) else {
            // Layer 3's missing drawing 7. Document 07: nothing is substituted for it.
            continue;
        };
        let Some(source) = decode_linear(&path) else {
            continue;
        };
        for (dst, src) in out.iter_mut().zip(&source) {
            // Co = Cs + Cd*(1-As), Ao = As + Ad*(1-As). Opacity is 1.0 for every layer of the
            // reference shot, so step 6 of the layer order multiplies by one and is left out.
            let inv = 1.0 - src[3];
            for c in 0..4 {
                dst[c] = src[c] + dst[c] * inv;
            }
        }
    }
    out
}

// ---------------------------------------------------------------------------------------
// The project the renderer is given
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

fn build_project() -> Project {
    let mut project = Project::new(id("proj-h01-whole-picture"));
    project.compositions.push(Composition::new(
        id(COMP),
        "reference shot",
        WIDTH as u32,
        HEIGHT as u32,
        FrameRate::new(24, 1).expect("24 fps"),
        0,
        240,
    ));
    let mut doc = Document::new(project);
    for n in 1..=4u32 {
        let asset = asset_for(n);
        let mut layer = Layer::new(
            id(&format!("layer-{n}")),
            format!("layer{n}"),
            asset.id.clone(),
            0,
            240,
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
    doc.project().clone()
}

// ---------------------------------------------------------------------------------------
// Comparing two whole pictures
// ---------------------------------------------------------------------------------------

/// How many of the 2,073,600 pixels differ at all, and by how much at worst.
struct Difference {
    pixels: usize,
    largest: f32,
}

fn compare(rendered: &WorkingBuffer, oracle: &[[f32; 4]]) -> Difference {
    let mut pixels = 0usize;
    let mut largest = 0.0f32;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let a = rendered.pixel(x, y);
            let b = oracle[y * WIDTH + x];
            let mut worst = 0.0f32;
            for c in 0..4 {
                worst = worst.max((a[c] - b[c]).abs());
            }
            if worst > 0.0 {
                pixels += 1;
                largest = largest.max(worst);
            }
        }
    }
    Difference { pixels, largest }
}

/// A frame written as an eight-bit sRGB PNG so it can be looked at. Straight alpha, the same
/// encoding the export writes, spelled out here rather than borrowed.
fn write_png(path: &Path, pixels: &[[f32; 4]]) {
    let to_srgb = |c: f32| -> f32 {
        if c <= 0.003_130_8 {
            c * 12.92
        } else {
            1.055 * c.powf(1.0 / 2.4) - 0.055
        }
    };
    let mut bytes = Vec::with_capacity(WIDTH * HEIGHT * 4);
    for p in pixels {
        let a = p[3];
        for c in p.iter().take(3) {
            let straight = if a > 0.0 { c / a } else { 0.0 };
            bytes.push((to_srgb(straight.clamp(0.0, 1.0)) * 255.0 + 0.5).floor() as u8);
        }
        bytes.push((a.clamp(0.0, 1.0) * 255.0 + 0.5).floor() as u8);
    }
    let file = std::io::BufWriter::new(fs::File::create(path).expect("create the artifact"));
    let mut encoder = png::Encoder::new(file, WIDTH as u32, HEIGHT as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder
        .write_header()
        .expect("write the header")
        .write_image_data(&bytes)
        .expect("write the pixels");
}

// ---------------------------------------------------------------------------------------
// The test
// ---------------------------------------------------------------------------------------

#[test]
fn h01_the_whole_picture_matches_an_independent_compositor() {
    let mut report = Report::default();
    let project = build_project();

    // Four frames chosen for what each one contains, not at random: the first frame of the
    // shot; the frame where layer 3's drawing is deliberately missing; one a hundred frames in,
    // where all four cadences are out of phase with each other; and the last frame of all.
    for frame in [0, 14, 100, 239] {
        let mut log = FrameLog::new(8);
        let rendered = render_frame(
            &project,
            &id(COMP),
            frame,
            &root(),
            DEFAULT_TILE_SIZE,
            &mut log,
        )
        .unwrap_or_else(|d| panic!("frame {frame} renders: {}", d.message));
        let oracle = independent_frame(frame);
        let d = compare(&rendered, &oracle);
        report.check(
            &format!(
                "frame {frame}: every one of the {} pixels is what a second compositor, written \
                 from document 21, produces",
                WIDTH * HEIGHT
            ),
            "0 pixels differ",
            format!(
                "{} pixels differ{}",
                d.pixels,
                if d.pixels == 0 {
                    String::new()
                } else {
                    format!(", the largest by {}", d.largest)
                }
            ),
        );

        if frame == 100 {
            write_png(&repo("verification/H-01_independent_frame.png"), &oracle);
            let mut mine = Vec::with_capacity(WIDTH * HEIGHT);
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    mine.push(rendered.pixel(x, y));
                }
            }
            write_png(&repo("verification/H-01_renderer_frame.png"), &mine);
        }
    }

    // The oracle has to be capable of disagreeing, or the rows above prove nothing. Comparing
    // two different frames must fail, and by a lot.
    let mut log = FrameLog::new(8);
    let frame_0 = render_frame(&project, &id(COMP), 0, &root(), DEFAULT_TILE_SIZE, &mut log)
        .expect("frame 0 renders");
    let against_100 = compare(&frame_0, &independent_frame(100));
    report.check(
        "the comparison is capable of failing: frame 0 against frame 100's picture",
        "they differ",
        if against_100.pixels > 0 {
            "they differ"
        } else {
            "the comparison found no difference between two different frames"
        },
    );

    // And the deliberate hole is real: layer 3 has no drawing 7, so frame 14 is composited from
    // three layers where frame 12 has four. Row two above is what says the renderer agrees.
    report.check(
        "layer 3 still has no drawing 7 in the fixture, so frame 14 has one fewer layer in it",
        "there is no file for layer 3 drawing 7",
        match cel_path(3, 7) {
            None => "there is no file for layer 3 drawing 7".to_string(),
            Some(p) => format!("{} exists", p.display()),
        },
    );
    report.check(
        "frame 14 and frame 12 are different pictures in the independent compositor too",
        "they differ",
        if independent_frame(14) != independent_frame(12) {
            "they differ"
        } else {
            "two different frames composited to the same picture"
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
        "# H-01 — the whole picture, not a sample of it\n\n\
         **{passed} of {} checks passed.**\n\n\
         Produced by `tests/h01_whole_picture.rs`.\n\n\
         ## Why this exists\n\n\
         Every other table in this build checks **named pixels**: this corner of that bar, the \
         alpha at that coordinate, this colour to six places. That is precise and it is narrow. \
         A fault that moved every layer one pixel across, dimmed the entire frame by a hair, or \
         swapped red for blue would pass all of them, and pass the hardening report's 137 \
         deliberate defects too, because nothing was looking at the picture as a whole. Your own \
         eye on the reference shot is the only thing that was, and an eye does not see one part \
         in a thousand.\n\n\
         So the reference shot is composited **twice**. Once by the renderer this project is \
         building — tiled, multithreaded, the real one. Once by a second compositor written \
         inside this test, from document 21, in the most obvious way possible: one loop over \
         2,073,600 pixels, bottom layer first, no tiles, no threads, and no code shared with \
         the renderer at all. The cel files are decoded separately, the sRGB curve is written \
         out longhand, and the composite is the formula as document 21 line 57 states it.\n\n\
         Then every channel of every pixel of four frames is compared. Not a tolerance, not a \
         sample: **any difference at all is a failure.**\n\n\
         ## What to look at\n\n\
         - **`H-01_renderer_frame.png`** and **`H-01_independent_frame.png`** — frame 100 of \
         your shot, produced by the two compositors. They should be indistinguishable, and the \
         table says they are identical to the last bit. Flip between them in an image viewer if \
         you want to see it for yourself.\n\n\
         ## What this still does not prove\n\n\
         Both compositors were written from the same document by the same agent, so a \
         misreading of document 21 that infects both would go unnoticed here. What it rules out \
         is the far likelier fault: an implementation slip in the fast, tiled, threaded renderer \
         that the naive one does not share. The frames chosen are frame 0, frame 14 (where layer \
         3's drawing is deliberately missing), frame 100 and frame 239, not every frame of the \
         shot.\n\n\
         The last two rows exist to prove the comparison can fail at all: two different frames \
         are compared and must disagree.\n\n\
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
    fs::write(repo("verification/H-01_whole_picture_table.md"), out).expect("write report");
}
