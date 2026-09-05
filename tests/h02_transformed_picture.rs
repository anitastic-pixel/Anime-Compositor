//! H-02: the whole picture again, but with the layers moved, scaled and faded.
//!
//! Writes `verification/H-02_transformed_table.md`, `verification/H-02_renderer_frame.png` and
//! `verification/H-02_independent_frame.png`.
//!
//! # What H-01 left, and this picks up
//!
//! H-01 compares the whole frame against a second compositor, but every layer of the reference
//! shot sits at the identity transform, so nothing there resamples: the renderer's transform
//! and sampling code is exercised only by B-05a's table, which reads **named pixels** of small
//! synthetic buffers. That leaves the same hole one level down. A fault in the inverse
//! transform, in the bilinear weights, or in where opacity is applied in the layer order would
//! change a real frame everywhere at once and be caught by nothing that looks at a real frame.
//!
//! So the reference shot is composited again, twice, with its layers transformed:
//!
//! | layer | what is done to it | why that one |
//! |---|---|---|
//! | 1 | scaled to 50% | it is the only cel whose paint reaches its own edge, so most of the frame samples *past* that edge - which is where document 21's "outside is transparent black" either holds or does not |
//! | 2 | moved by exactly `(+320, -180)` pixels | whole-pixel translation, no resampling |
//! | 3 | anchor at the centre, scaled to 200%, opacity 50% | resampling on quarter-pixel weights, an anchor that is not the origin, and step 6 of the layer order |
//! | 4 | scaled to 50% | resampling on half-pixel weights |
//!
//! Layer 1's scale and layer 3's anchor are both there because the first mutation pass without
//! them had two survivors: with every anchor at the origin, `T(-anchor)` is the identity and the
//! transform chain can be composed in the wrong order unnoticed, and with every sampled layer
//! transparent at its border, clamping to the edge pixel looks exactly like transparent black.
//! The fixture was changed, per `DAY_RUN.md`, never the assertion.
//!
//! **Rotation is deliberately not here.** `cos(90°)` in floating point is 6.1e-17 rather than
//! zero, so a rotated layer's sample points miss the pixel centres by a hair and two honest
//! implementations of the same formula disagree in the last bits for reasons that have nothing
//! to do with correctness. Rotation stays with B-05a's table, where the expected values are
//! named pixels worked out by hand. Choosing translations and scales that are exact in binary
//! is what makes a whole-frame comparison a fair check rather than a float-noise detector.
//!
//! # Where the expected values come from
//!
//! - Document 21 line 17: `p_comp = T(position) * R(rotation) * S(scale) * T(-anchor) * p_layer`,
//!   and "renderer sampling uses the inverse transform from destination pixel center to source
//!   space". With rotation left out and the anchor at the origin, that inverse is
//!   `p_layer = (p_comp - position) / scale + anchor`, which is written out directly below rather than
//!   built as a matrix and inverted. The renderer builds and inverts the full chain; this does
//!   not. That is the point.
//! - Document 21 line 23: bilinear in premultiplied linear RGBA, weights from source
//!   **pixel-centre** coordinates, and samples outside the source extent are transparent black
//!   — not the edge pixel, and not a renormalised three-neighbour average.
//! - Document 21 line 37: opacity is step 6, after the transform and before the composite.
//! - Document 21 line 57 for the composite, and D-17 for the sRGB curve, as in H-01.
//! - D-22: scale is stored as a unit factor, so 200% is `2.0`.
//!
//! # The tolerance, and why there is one
//!
//! H-01 demands exact equality because nothing there resamples. Here both compositors add four
//! weighted neighbours per pixel, and this one accumulates in `f64` where the renderer works in
//! `f32`. Float addition is not associative, so insisting on identical bits would be insisting
//! the two implementations perform the same operations in the same order — the opposite of an
//! independent check. The bound is therefore `1e-6`: a `f32` mantissa is 24 bits, values here
//! are at most about 1, so one rounding step is about `6e-8` and a four-term weighted sum plus
//! the composite cannot accumulate more than a few of those. A real fault in a transform, a
//! weight or the layer order moves a pixel by orders of magnitude more than that; the largest
//! difference actually seen is reported in the table so the number can be judged rather than
//! trusted.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::media::import_sequence;
use anime_compositor::model::{Asset, AssetKind, Composition, Id, Layer, Project, Prop, Value};
use anime_compositor::time::{ExposureMap, ExposureSpan, FrameRate};
use anime_compositor::WorkingBuffer;

const COMP: &str = "comp-h02";
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

/// The tolerance argued for in the module documentation.
const BOUND: f64 = 1e-6;

/// What is done to each layer: `(anchor, position, scale, opacity)`.
///
/// Every number here is exact in binary, so the sample points land on quarter and half pixels
/// and both compositors compute the same weights without rounding.
#[allow(clippy::type_complexity)]
const MOVES: [((f64, f64), (f64, f64), (f64, f64), f64); 4] = [
    ((0.0, 0.0), (0.0, 0.0), (0.5, 0.5), 1.0),
    ((0.0, 0.0), (320.0, -180.0), (1.0, 1.0), 1.0),
    ((960.0, 540.0), (0.0, 0.0), (2.0, 2.0), 0.5),
    ((0.0, 0.0), (0.0, 0.0), (0.5, 0.5), 1.0),
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

/// D-17's transfer function, written out rather than called.
fn srgb_to_linear(c: f64) -> f64 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// One cel as linear premultiplied RGBA, per document 21 line 29.
fn decode_linear(path: &Path) -> Option<Vec<[f64; 4]>> {
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

/// Bilinear at continuous source coordinates, document 21 line 23.
///
/// The neighbours are the four source **pixel centres** around the point, which is why the
/// index is `floor(x - 0.5)`; a neighbour off the edge contributes transparent black rather
/// than being clamped to the border pixel or dropped from the weighting.
fn sample(source: &[[f64; 4]], x: f64, y: f64) -> [f64; 4] {
    let (fx, fy) = (x - 0.5, y - 0.5);
    let (x0, y0) = (fx.floor(), fy.floor());
    let (ux, uy) = (fx - x0, fy - y0);
    let mut out = [0.0f64; 4];
    for (dy, wy) in [(0i64, 1.0 - uy), (1, uy)] {
        let sy = y0 as i64 + dy;
        for (dx, wx) in [(0i64, 1.0 - ux), (1, ux)] {
            let sx = x0 as i64 + dx;
            if sx < 0 || sy < 0 || sx >= WIDTH as i64 || sy >= HEIGHT as i64 {
                continue; // transparent black contributes nothing but still holds its weight
            }
            let px = source[sy as usize * WIDTH + sx as usize];
            let w = wx * wy;
            for c in 0..4 {
                out[c] += px[c] * w;
            }
        }
    }
    out
}

/// Which drawing each layer shows, from document 22's cadences and the exposure sheet.
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

/// The whole transformed frame, one pixel at a time, bottom layer first.
fn independent_frame(frame: i32) -> Vec<[f64; 4]> {
    let mut out = vec![[0.0f64; 4]; WIDTH * HEIGHT];
    for layer in 1..=4u32 {
        let Some(path) = cel_path(layer, drawing_at(layer, frame)) else {
            continue; // layer 3's deliberately missing drawing 7
        };
        let Some(source) = decode_linear(&path) else {
            continue;
        };
        let ((ax, ay), (px, py), (sx, sy), opacity) = MOVES[layer as usize - 1];
        for j in 0..HEIGHT {
            for i in 0..WIDTH {
                // Document 21: the destination sample point is the pixel centre, and the
                // inverse of T(position)*S(scale)*T(-anchor) is this, written as arithmetic
                // rather than as a matrix the renderer would recognise.
                let src = sample(
                    &source,
                    (i as f64 + 0.5 - px) / sx + ax,
                    (j as f64 + 0.5 - py) / sy + ay,
                );
                let dst = &mut out[j * WIDTH + i];
                // Step 6 opacity, then Co = Cs + Cd*(1-As), Ao = As + Ad*(1-As).
                let inv = 1.0 - src[3] * opacity;
                for c in 0..4 {
                    dst[c] = src[c] * opacity + dst[c] * inv;
                }
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
    let mut project = Project::new(id("proj-h02-transformed"));
    project.compositions.push(Composition::new(
        id(COMP),
        "reference shot, transformed",
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
        let ((ax, ay), (px, py), (sx, sy), opacity) = MOVES[n as usize - 1];
        doc.apply_all(vec![
            Command::AddAsset { asset },
            Command::AddLayer {
                composition: id(COMP),
                layer: Box::new(layer),
                index: (n - 1) as usize,
            },
            Command::SetPropertyBase {
                composition: id(COMP),
                layer_id: id(&format!("layer-{n}")),
                prop: Prop::Anchor,
                value: Value::Vec2(ax, ay),
            },
            Command::SetPropertyBase {
                composition: id(COMP),
                layer_id: id(&format!("layer-{n}")),
                prop: Prop::Position,
                value: Value::Vec2(px, py),
            },
            Command::SetPropertyBase {
                composition: id(COMP),
                layer_id: id(&format!("layer-{n}")),
                prop: Prop::Scale,
                value: Value::Vec2(sx, sy),
            },
            Command::SetPropertyBase {
                composition: id(COMP),
                layer_id: id(&format!("layer-{n}")),
                prop: Prop::Opacity,
                value: Value::Scalar(opacity),
            },
        ])
        .expect("the transformed project is valid");
    }
    doc.project().clone()
}

// ---------------------------------------------------------------------------------------
// Comparing two whole pictures
// ---------------------------------------------------------------------------------------

struct Difference {
    over_bound: usize,
    largest: f64,
}

fn compare(rendered: &WorkingBuffer, oracle: &[[f64; 4]]) -> Difference {
    let mut over_bound = 0usize;
    let mut largest = 0.0f64;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let a = rendered.pixel(x, y);
            let b = oracle[y * WIDTH + x];
            let mut worst = 0.0f64;
            for c in 0..4 {
                worst = worst.max((a[c] as f64 - b[c]).abs());
            }
            if worst > BOUND {
                over_bound += 1;
            }
            largest = largest.max(worst);
        }
    }
    Difference {
        over_bound,
        largest,
    }
}

/// A frame as an eight-bit sRGB PNG so it can be looked at.
fn write_png(path: &Path, pixels: &[[f64; 4]]) {
    let to_srgb = |c: f64| -> f64 {
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
fn h02_moved_and_scaled_layers_match_an_independent_compositor() {
    let mut report = Report::default();
    let project = build_project();

    for frame in [0, 14, 100] {
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
                "frame {frame}, layers moved and scaled: every one of the {} pixels agrees with \
                 a second compositor to within {BOUND}",
                WIDTH * HEIGHT
            ),
            "0 pixels differ by more than the bound",
            format!("{} pixels differ by more than the bound", d.over_bound),
        );
        report.check(
            &format!(
                "frame {frame}: the largest disagreement anywhere was {:e}, float rounding rather than a fault",
                d.largest
            ),
            format!("no more than {BOUND}"),
            if d.largest <= BOUND {
                format!("no more than {BOUND}")
            } else {
                format!("{:e}", d.largest)
            },
        );

        if frame == 100 {
            write_png(&repo("verification/H-02_independent_frame.png"), &oracle);
            let mut mine = Vec::with_capacity(WIDTH * HEIGHT);
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let p = rendered.pixel(x, y);
                    mine.push([p[0] as f64, p[1] as f64, p[2] as f64, p[3] as f64]);
                }
            }
            write_png(&repo("verification/H-02_renderer_frame.png"), &mine);
        }
    }

    // The transforms have to have done something, or the rows above are H-01 again under a new
    // name. The transformed frame must differ from the untransformed one by a great deal.
    let mut plain = Project::new(id("proj-h02-plain"));
    plain.compositions.push(Composition::new(
        id(COMP),
        "reference shot",
        WIDTH as u32,
        HEIGHT as u32,
        FrameRate::new(24, 1).expect("24 fps"),
        0,
        240,
    ));
    let mut doc = Document::new(plain);
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
        .expect("the untransformed project is valid");
    }
    let mut log = FrameLog::new(8);
    let untransformed = render_frame(
        doc.project(),
        &id(COMP),
        100,
        &root(),
        DEFAULT_TILE_SIZE,
        &mut log,
    )
    .expect("the untransformed frame renders");
    let against_plain = compare(&untransformed, &independent_frame(100));
    report.check(
        "the transforms changed the picture: the same frame without them does not match",
        "they differ",
        if against_plain.over_bound > 0 {
            "they differ"
        } else {
            "moving and scaling every layer changed nothing"
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
        "# H-02 — the whole picture again, with the layers moved and scaled\n\n\
         **{passed} of {} checks passed.**\n\n\
         Produced by `tests/h02_transformed_picture.rs`.\n\n\
         ## Why this exists\n\n\
         H-01 composites your shot twice and compares every pixel, which is the strongest check \
         in this build — but every layer of the shot sits exactly where it was drawn. Nothing \
         there is moved, scaled or faded, so nothing there resamples, and the code that decides \
         *where a moved layer's pixels land* is checked only by a table of named pixels on small \
         made-up images.\n\n\
         So the same shot is composited twice again, with the layers pushed around: layer 2 \
         moved 320 pixels right and 180 up, layer 3 blown up to twice its size at half opacity, \
         layer 4 shrunk to half. Once by the real renderer, once by a second compositor written \
         inside the test from document 21, which works out where each pixel comes from with \
         ordinary arithmetic instead of the renderer's matrices.\n\n\
         ## What to look at\n\n\
         - **`H-02_renderer_frame.png`** and **`H-02_independent_frame.png`** — frame 100, \
         produced by the two compositors. The picture should look wrong in an obvious, \
         deliberate way: pieces of your shot shifted and resized. The two files should look the \
         same as each other. Compare them against `H-01_renderer_frame.png`, which is the same \
         frame untouched.\n\n\
         ## The tolerance\n\n\
         H-01 demands the two compositors agree *exactly*. This one allows a difference of \
         **0.000001** per channel, and reports the largest difference it actually found so the \
         allowance can be judged rather than taken on trust. The reason is arithmetic, not \
         laxity: blending four neighbouring pixels means adding four numbers, and adding the \
         same four numbers in a different order gives answers that differ in the last bit or \
         two. Demanding identical bits would demand the two compositors do the arithmetic in \
         the same order, which would make them the same compositor. A real fault — a wrong \
         weight, an inverted transform, opacity applied at the wrong step — moves pixels by \
         thousands of times more than the allowance.\n\n\
         ## What is deliberately not here\n\n\
         **Rotation.** `cos(90°)` in floating point is not zero but 0.000000000000000061, so a \
         rotated layer's samples land a hair off the pixel centres and two correct \
         implementations disagree in the last bits for reasons that have nothing to do with \
         either being wrong. Rotation is checked in `B-05a_transform_table.md`, pixel by named \
         pixel, where that is not a problem. The moves and scales chosen here are exact in \
         binary on purpose, which is what makes comparing whole frames fair.\n\n\
         As in H-01, both compositors were written from the same document by the same agent: \
         this catches an implementation slip in the renderer, not a misreading of the \
         specification.\n\n\
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
    fs::write(repo("verification/H-02_transformed_table.md"), out).expect("write report");
}
