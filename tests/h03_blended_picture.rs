//! H-03: the whole picture again, with the layers set to multiply, screen and add.
//!
//! Writes `verification/H-03_blended_table.md`, `verification/H-03_renderer_frame.png` and
//! `verification/H-03_independent_frame.png`.
//!
//! # What H-01 and H-02 left, and this picks up
//!
//! H-01 composites the reference shot twice and compares every pixel, and H-02 does it again with
//! the layers moved and scaled. Every layer in both of them is set to **normal**, which is the one
//! blend mode that never recovers a straight colour: document 21 states normal separately as
//! `Co = Cs + Cd*(1-As)` and the renderer routes it to a different function entirely. So the
//! branch that multiply, screen and add all go through - the unpremultiply, the `As*Ad*B` term,
//! and the `Ao = As + Ad - As*Ad` alpha - has never been run on a real frame by anything in this
//! repository.
//!
//! What it has been run on is `tests/b05c_blend.rs`, whose eighteen rows are single pixels of
//! made-up colour worked out on paper. That table says so itself: *"Nothing assembles a render
//! from a saved project yet, so no test here can show a mode travelling from a file to the
//! screen."* Assembly exists now. Document 21 line 77 asks for exactly this before the modes are
//! trusted: *"Independent fixtures in 25 must verify each mode before the GPU implementation is
//! accepted."*
//!
//! A fault reachable here and nowhere else: an unpremultiply that clamps, a blend applied to
//! premultiplied colour instead of straight, the `As*Ad` weight dropped, or the alpha of a
//! blended composite computed as though it were normal-over. On a made-up pixel with alpha 1
//! several of those are invisible, because `As*Ad*B` and `As*B` are the same number when both
//! alphas are 1. The reference shot's cels are full of antialiased edges where they are not.
//!
//! # The modes chosen, and why each one
//!
//! | layer | mode | opacity | why that one |
//! |---|---|---|---|
//! | 1 | normal | 1.0 | the bottom of the stack composites onto nothing, so its mode is not the thing under test; leaving it normal keeps the background identical to H-01's |
//! | 2 | multiply | 1.0 | darkens; the mode whose result is furthest from the source, so a mode that was stored and then ignored shows up immediately |
//! | 3 | screen | 0.5 | lightens, at half opacity, so document 21's step 6 (opacity) has to happen before step 7 (blend). Applying opacity after the blend gives a different picture everywhere the layers overlap |
//! | 4 | add | 1.0 | the clamped mode, `min(1, cs + cd)`; the clamp is on the straight colours before weighting, which is a place an implementation can plausibly put it somewhere else |
//!
//! Transforms are left at the identity on purpose. H-02 owns resampling; mixing the two would
//! mean a disagreement could come from either and neither could be blamed.
//!
//! # Where the expected values come from
//!
//! - Document 21 lines 65-77, written out below as arithmetic: `multiply: B = cs*cd`,
//!   `screen: B = cs + cd - cs*cd`, `add: B = min(1, cs + cd)`, then
//!   `Co = (1-As)*Cd + (1-Ad)*Cs + As*Ad*B(cs,cd)` and `Ao = As + Ad - As*Ad`, with straight
//!   colours recovered where alpha is nonzero and **zero where it is not**.
//! - Document 21 line 57 for normal, which the bottom layer uses.
//! - Document 21 lines 42-43: opacity is step 6 and the blend is step 7, in that order.
//! - Document 21 line 29 for the decode, and D-17 for the sRGB curve, as in H-01.
//!
//! The renderer reaches the same equations through `BlendMode`, a dispatch and a shortcut that
//! skips the unpremultiply for normal. This does none of that: it branches on a mode name in the
//! test and does the division every time.
//!
//! # The tolerance, and why there is one
//!
//! H-01 demands exact equality, because normal-over is two multiplies and an add and both
//! compositors do them in the same order in `f32`. This one cannot: recovering a straight colour
//! is a division, and this compositor accumulates in `f64` where the renderer works in `f32`.
//! The bound is `1e-6`, argued the same way H-02 argues its own: an `f32` mantissa is 24 bits,
//! the values here are at most about 1, so one rounding step is about `6e-8`, and the handful of
//! steps in one blended pixel cannot accumulate more than a few of those.
//!
//! The division deserves a sentence of its own, because it is where a bound like this could
//! plausibly be too tight. At a nearly-transparent edge pixel, `cd = Cd/Ad` magnifies a rounding
//! error in `Cd` by `1/Ad`, which for an alpha of one 255th is a factor of 255. But the term that
//! error lands in is `As*Ad*B`, which multiplies it straight back down by `Ad`. The magnification
//! cancels, which is why a small alpha does not need a larger allowance. The largest difference
//! actually found is reported in the table either way, so the allowance can be judged rather than
//! taken on trust.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::media::import_sequence;
use anime_compositor::model::{
    Asset, AssetKind, BlendMode, Composition, Id, Layer, Project, Prop, Value,
};
use anime_compositor::time::{ExposureMap, ExposureSpan, FrameRate};
use anime_compositor::WorkingBuffer;

const COMP: &str = "comp-h03";
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

/// The tolerance argued for in the module documentation.
const BOUND: f64 = 1e-6;

/// What each layer is set to: `(mode, opacity)`, bottom layer first.
const MODES: [(BlendMode, f64); 4] = [
    (BlendMode::Normal, 1.0),
    (BlendMode::Multiply, 1.0),
    (BlendMode::Screen, 0.5),
    (BlendMode::Add, 1.0),
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

/// Document 21 line 65: "first recover straight colors cs and cd where alpha is nonzero", and
/// line 77: "Zero-alpha straight colors are zero".
fn straight(p: [f64; 4]) -> [f64; 3] {
    if p[3] > 0.0 {
        [p[0] / p[3], p[1] / p[3], p[2] / p[3]]
    } else {
        [0.0; 3]
    }
}

/// Document 21 lines 67-69, component-wise on straight colours.
fn blend_function(mode: BlendMode, cs: f64, cd: f64) -> f64 {
    match mode {
        BlendMode::Multiply => cs * cd,
        BlendMode::Screen => cs + cd - cs * cd,
        BlendMode::Add => (cs + cd).min(1.0),
        BlendMode::Normal => cs,
    }
}

/// One premultiplied source over one premultiplied destination, document 21 lines 57 and 73.
///
/// Normal is written as its own two lines rather than routed through the blend equation, because
/// document 21 states it separately and that separate statement is the authoritative one.
fn composite(mode: BlendMode, src: [f64; 4], dst: [f64; 4]) -> [f64; 4] {
    if mode == BlendMode::Normal {
        let inv = 1.0 - src[3];
        return [
            src[0] + dst[0] * inv,
            src[1] + dst[1] * inv,
            src[2] + dst[2] * inv,
            src[3] + dst[3] * inv,
        ];
    }
    let (cs, cd) = (straight(src), straight(dst));
    let (a_s, a_d) = (src[3], dst[3]);
    let mut out = [0.0f64; 4];
    for c in 0..3 {
        let b = blend_function(mode, cs[c], cd[c]);
        out[c] = (1.0 - a_s) * dst[c] + (1.0 - a_d) * src[c] + a_s * a_d * b;
    }
    out[3] = a_s + a_d - a_s * a_d;
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

/// The whole blended frame, one pixel at a time, bottom layer first.
///
/// `layers` says which of the four to include, so the same routine builds both the four-layer
/// picture and the two-layer one that checks what a mode does over nothing.
///
/// The second return is how many times a partly-transparent pixel was composited *onto* another
/// partly-transparent one. Both alphas being strictly between 0 and 1 is the only situation in
/// which the `As*Ad` weight and the `As + Ad - As*Ad` alpha differ from the simpler things they
/// are easily mistaken for, so a stack that never does it cannot check them however many pixels
/// it compares. One row below asserts this count is not zero.
fn independent_frame(
    frame: i32,
    layers: &[u32],
    modes: &[(BlendMode, f64); 4],
) -> (Vec<[f64; 4]>, usize) {
    let mut out = vec![[0.0f64; 4]; WIDTH * HEIGHT];
    let mut overlaps = 0usize;
    for layer in layers {
        let Some(path) = cel_path(*layer, drawing_at(*layer, frame)) else {
            continue; // layer 3's deliberately missing drawing 7
        };
        let Some(source) = decode_linear(&path) else {
            continue;
        };
        let (mode, opacity) = modes[*layer as usize - 1];
        for (dst, src) in out.iter_mut().zip(&source) {
            // Document 21 step 6: opacity scales the premultiplied source, before the blend.
            let faded = [
                src[0] * opacity,
                src[1] * opacity,
                src[2] * opacity,
                src[3] * opacity,
            ];
            let partial = |a: f64| a > 0.0 && a < 1.0;
            if mode != BlendMode::Normal && partial(faded[3]) && partial(dst[3]) {
                overlaps += 1;
            }
            *dst = composite(mode, faded, *dst);
        }
    }
    (out, overlaps)
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

fn build_project(name: &str, layers: &[u32], modes: &[(BlendMode, f64); 4]) -> Project {
    let mut project = Project::new(id(name));
    project.compositions.push(Composition::new(
        id(COMP),
        "reference shot, blended",
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
        let (mode, opacity) = modes[*n as usize - 1];
        // The model carries the mode on the layer itself; there is no command for it, which is
        // why this is set here rather than applied like the opacity below.
        layer.blend_mode = mode;
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
                value: Value::Scalar(opacity),
            },
        ])
        .expect("the blended project is valid");
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

/// How far apart two whole frames are, for the rows that need the modes to have *done* something.
fn largest_gap(a: &WorkingBuffer, b: &WorkingBuffer) -> f64 {
    let mut largest = 0.0f64;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let (p, q) = (a.pixel(x, y), b.pixel(x, y));
            for c in 0..4 {
                largest = largest.max((p[c] as f64 - q[c] as f64).abs());
            }
        }
    }
    largest
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

fn render(project: &Project, frame: i32) -> WorkingBuffer {
    let mut log = FrameLog::new(8);
    render_frame(
        project,
        &id(COMP),
        frame,
        &root(),
        DEFAULT_TILE_SIZE,
        &mut log,
    )
    .unwrap_or_else(|d| panic!("frame {frame} renders: {}", d.message))
}

const ALL_NORMAL: [(BlendMode, f64); 4] = [
    (BlendMode::Normal, 1.0),
    (BlendMode::Normal, 1.0),
    (BlendMode::Normal, 0.5),
    (BlendMode::Normal, 1.0),
];

// ---------------------------------------------------------------------------------------
// The test
// ---------------------------------------------------------------------------------------

#[test]
fn h03_multiply_screen_and_add_match_an_independent_compositor() {
    let mut report = Report::default();
    let all = [1u32, 2, 3, 4];
    let project = build_project("proj-h03-blended", &all, &MODES);

    for frame in [0, 14, 100] {
        let rendered = render(&project, frame);
        let (oracle, _) = independent_frame(frame, &all, &MODES);
        let d = compare(&rendered, &oracle);
        report.check(
            &format!(
                "frame {frame}, layers set to multiply, screen and add: every one of the {} \
                 pixels agrees with a second compositor to within {BOUND}",
                WIDTH * HEIGHT
            ),
            "0 pixels differ by more than the bound",
            format!("{} pixels differ by more than the bound", d.over_bound),
        );
        report.check(
            &format!(
                "frame {frame}: the largest disagreement anywhere was {:e}, float rounding rather \
                 than a fault",
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
            write_png(&repo("verification/H-03_independent_frame.png"), &oracle);
            let mut mine = Vec::with_capacity(WIDTH * HEIGHT);
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let p = rendered.pixel(x, y);
                    mine.push([p[0] as f64, p[1] as f64, p[2] as f64, p[3] as f64]);
                }
            }
            write_png(&repo("verification/H-03_renderer_frame.png"), &mine);
        }
    }

    // The modes have to have done something, or every row above is H-01 again under a new name.
    // The same four layers, same opacities, all set to normal, must give a different picture -
    // and by a wide margin rather than in the last bits.
    let plain = build_project("proj-h03-plain", &all, &ALL_NORMAL);
    let gap = largest_gap(&render(&project, 100), &render(&plain, 100));
    report.check(
        &format!(
            "the modes changed the picture: the same frame with every layer set to normal is \
             {gap:.4} away at its furthest pixel, not a rounding difference"
        ),
        "further apart than 0.01",
        if gap > 0.01 {
            "further apart than 0.01".to_string()
        } else {
            format!("only {gap:e} apart")
        },
    );

    // Document 21's `As*Ad*B` term, read at its edge: over a *transparent* destination `Ad` is
    // zero, so the blended term vanishes and the equation reduces to `Co = Cs`. A screen layer at
    // the bottom of a stack keeps its own colour rather than blowing out against the emptiness
    // behind it. The reference shot's own bottom layer is normal, so every other row in this file
    // blends onto an opaque background where `Ad` is 1 and dropping it changes nothing.
    //
    // Screen and not multiply: over nothing `cd` is zero, and multiply's `B = cs*cd` is then zero
    // too, so a multiply layer looks the same whether the `Ad` weight is there or not. Screen's
    // `B = cs + cd - cs*cd` is `cs`, which the missing weight would add straight into the result.
    // The first draft of this row used multiply and could not fail; see the mutation report.
    let alone = [3u32];
    let screen_alone = build_project("proj-h03-screen-alone", &alone, &MODES);
    let normal_alone = build_project("proj-h03-normal-alone", &alone, &ALL_NORMAL);
    let gap = largest_gap(&render(&screen_alone, 100), &render(&normal_alone, 100));
    report.check(
        "a screen layer over nothing at all is its own colour, exactly as the same layer set to \
         normal would be",
        "identical",
        if gap == 0.0 {
            "identical".to_string()
        } else {
            format!("{gap:e} apart at the furthest pixel")
        },
    );

    // And the other edge of the same equation: a stack with *no* opaque background, so the modes
    // meet soft edges on both sides. `As + Ad - As*Ad` is only distinguishable from the things it
    // is confusable with - `min(1, As+Ad)`, `max(As,Ad)` - when both alphas are strictly between
    // 0 and 1, which cannot happen anywhere in the four-layer stack above once layer 1 has filled
    // the frame. The count of such meetings is reported so this row cannot quietly become vacuous.
    //
    // Frame 110 and not 100, because at frame 100 the drawings do not overlap and the count is
    // zero: layer 4's half-transparent interior has to land on layer 2's antialiased ring, and it
    // only does so on some frames. This was found by asking the count, not by assuming.
    const FLOATING_FRAME: i32 = 110;
    let floating = [2u32, 3, 4];
    let no_background = build_project("proj-h03-no-background", &floating, &MODES);
    let (oracle, overlaps) = independent_frame(FLOATING_FRAME, &floating, &MODES);
    let d = compare(&render(&no_background, FLOATING_FRAME), &oracle);
    report.check(
        &format!(
            "frame {FLOATING_FRAME} with the opaque background layer removed: the modes still \
             agree with the second compositor where soft edges meet soft edges"
        ),
        "0 pixels differ by more than the bound",
        format!("{} pixels differ by more than the bound", d.over_bound),
    );
    report.check(
        &format!(
            "and that stack really does put partly-transparent pixels over partly-transparent \
             ones - it happened {overlaps} times - so the row above is testing something"
        ),
        "more than 1000 such pixels",
        if overlaps > 1000 {
            "more than 1000 such pixels".to_string()
        } else {
            format!("only {overlaps}")
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
        "# H-03 — the whole picture again, with the layers set to multiply, screen and add\n\n\
         **{passed} of {} checks passed.**\n\n\
         Produced by `tests/h03_blended_picture.rs`.\n\n\
         ## Why this exists\n\n\
         H-01 composites your shot twice and compares every pixel; H-02 does it again with the \
         layers moved and scaled. Every layer in both is set to **normal**, which is the one mode \
         that takes a different route through the renderer. The arithmetic that multiply, screen \
         and add all share had never been run on a real frame by anything in this project - only \
         on eighteen single pixels of made-up colour in `B-05c_blend_table.md`, which says so \
         itself.\n\n\
         So the shot is composited twice again, with the second layer set to multiply, the third \
         to screen at half opacity and the fourth to add. Once by the real renderer, once by a \
         second compositor written inside the test from document 21's four equations.\n\n\
         ## What to look at\n\n\
         - **`H-03_renderer_frame.png`** and **`H-03_independent_frame.png`** — frame 100, \
         produced by the two compositors. The picture should look wrong in an obvious, deliberate \
         way: parts of your shot darkened where they overlap, others brightened or blown out. The \
         two files should look the same as each other. `H-01_renderer_frame.png` is the same \
         frame with every layer left on normal, for comparison.\n\n\
         ## The rows that are not pixel comparisons\n\n\
         Three of these rows are here because the pixel comparisons above them, on their own, \
         cannot fail for certain faults. Each was written after breaking the code on purpose and \
         watching the break get through; the mutation report has the detail.\n\n\
         **The modes did something.** The same four layers, same opacities, set back to normal \
         must give a picture far away from this one, not one that differs in the last bits. \
         Without it, a build that stored your choice of mode and then ignored it would pass every \
         other row, because the second compositor would be told the same modes and a mode nobody \
         applies changes nothing on either side.\n\n\
         **A layer over nothing keeps its own colour.** The blended term is weighted by the \
         alphas of *both* the layer and what is under it, so over an empty background it vanishes \
         and a screen layer at the bottom of a stack does not blow out against the emptiness. The \
         reference shot's own bottom layer is normal, so every other row in this table blends \
         onto something opaque, where dropping that weight changes nothing at all.\n\n\
         **Soft edges meeting soft edges.** One more picture, at frame 110, with the opaque \
         background removed so the modes land on half-transparent pixels rather than solid ones. \
         The rule for how two transparencies combine is only distinguishable from the simpler \
         wrong answers when *both* are partly transparent, which never happens once the \
         background has filled the frame. The row after it counts how many times that actually \
         occurred and fails if it is not thousands — at frame 100 it happens **zero** times, \
         which is how the frame was chosen.\n\n\
         ## The tolerance\n\n\
         A difference of **0.000001** per channel is allowed, and the largest difference actually \
         found is reported so the allowance can be judged rather than taken on trust. The reason \
         is arithmetic: these modes divide colour by alpha, and this compositor works to about \
         seventeen digits where the renderer works to about seven. Demanding identical bits would \
         demand the two do the same operations in the same order, which would make them one \
         compositor wearing two hats. A real fault — a blend applied to the wrong kind of colour, \
         a missing weight, opacity applied after the blend instead of before — moves pixels by \
         thousands of times more than the allowance.\n\n\
         ## What is deliberately not here\n\n\
         **Transforms.** Every layer sits where it was drawn, so nothing resamples. H-02 owns \
         moving and scaling; mixing the two would mean a disagreement could have come from \
         either and neither could be blamed for it.\n\n\
         As in H-01 and H-02, both compositors were written from the same document by the same \
         agent: this catches an implementation slip in the renderer, not a misreading of the \
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
    fs::write(repo("verification/H-03_blended_table.md"), out).expect("write report");
}
