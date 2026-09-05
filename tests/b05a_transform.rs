//! B-05a: the tiled transform renderer. Fixtures FX-XF-001 through FX-XF-004 of document 25,
//! which are the render half of test T-03, plus ADR-011's requirement that a tiled render and
//! a whole-frame render of the same request be byte-identical.
//!
//! Every expected value below is written as a literal derived from document 21's stated rules,
//! never captured from a run of the renderer. Where a number is not obvious the row says where
//! it comes from. FX-XF-003's four quarters are `0.5 * 0.5` from "bilinear weights are computed
//! from source pixel-center coordinates"; FX-XF-004's placement comes from applying
//! `T(position) * R(rotation) * S(scale) * T(-anchor)` to one pixel centre by hand.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::media::import_sequence;
use anime_compositor::model::{BlendMode, Id};
use anime_compositor::render::{render, sample_bilinear, Affine, FramePlan, LayerDraw};
use anime_compositor::time::{resolve, ExposureMap, LayerTiming, SourceAt};
use anime_compositor::WorkingBuffer;

// -- reporting ------------------------------------------------------------------------------

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
    notes: Vec<String>,
}

impl Report {
    fn check(&mut self, check: &str, expected: impl ToString, actual: impl ToString) {
        self.rows.push(Row {
            check: check.to_string(),
            expected: expected.to_string(),
            actual: actual.to_string(),
        });
    }

    fn note(&mut self, text: impl Into<String>) {
        self.notes.push(text.into());
    }

    fn failures(&self) -> Vec<&Row> {
        self.rows.iter().filter(|r| !r.pass()).collect()
    }
}

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

// -- small helpers over the working buffer --------------------------------------------------

/// A source buffer built straight in the working space, so no colour conversion sits between
/// the values written here and the values the sampler reads. These fixtures are about
/// geometry; B-02's fixtures already cover the colour pipeline.
fn working(width: usize, height: usize, pixels: &[(usize, usize, [f32; 4])]) -> WorkingBuffer {
    let mut buffer = WorkingBuffer::transparent(width, height);
    let data = buffer.data_mut();
    for &(x, y, px) in pixels {
        let i = (y * width + x) * 4;
        data[i..i + 4].copy_from_slice(&px);
    }
    buffer
}

/// A single opaque pixel of a distinctive colour, in an otherwise transparent image.
/// Document 25 calls FX-XF-002 a "1x1 impulse".
const IMPULSE: [f32; 4] = [0.25, 0.5, 0.75, 1.0];

fn impulse_source(size: usize, at: (usize, usize)) -> WorkingBuffer {
    working(size, size, &[(at.0, at.1, IMPULSE)])
}

fn plan(width: usize, height: usize, layers: Vec<LayerDraw>) -> FramePlan {
    FramePlan {
        width,
        height,
        layers,
    }
}

fn one_layer(source: WorkingBuffer, transform: Affine) -> Vec<LayerDraw> {
    vec![LayerDraw {
        id: Id::new("fixture"),
        source,
        transform,
        opacity: 1.0,
        blend: BlendMode::Normal,
    }]
}

/// Every pixel that is not exactly transparent black, as `(x, y)` in reading order.
fn nonzero(frame: &WorkingBuffer) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for y in 0..frame.height() {
        for x in 0..frame.width() {
            if frame.pixel(x, y) != [0.0; 4] {
                out.push((x, y));
            }
        }
    }
    out
}

fn px(frame: &WorkingBuffer, x: usize, y: usize) -> String {
    let p = frame.pixel(x, y);
    format!("[{}, {}, {}, {}]", p[0], p[1], p[2], p[3])
}

fn rounded(frame: &WorkingBuffer, x: usize, y: usize) -> String {
    let p = frame.pixel(x, y);
    format!("[{:.6}, {:.6}, {:.6}, {:.6}]", p[0], p[1], p[2], p[3])
}

fn scaled(px: [f32; 4], k: f32) -> String {
    format!(
        "[{}, {}, {}, {}]",
        px[0] * k,
        px[1] * k,
        px[2] * k,
        px[3] * k
    )
}

fn total_alpha(frame: &WorkingBuffer) -> f64 {
    frame.data().chunks_exact(4).map(|p| p[3] as f64).sum()
}

/// A whole-frame render: one tile covering the entire output. Document 21's tile contract
/// speaks of "a hypothetical whole-frame render", and this is it, expressed as the degenerate
/// tiling rather than as a second code path that could drift from the real one.
fn whole_frame(plan: &FramePlan) -> WorkingBuffer {
    render(plan, plan.width.max(plan.height))
}

// -- FX-XF-001 through FX-XF-004 ------------------------------------------------------------

#[test]
fn b05a_transform_fixtures() {
    let mut report = Report::default();

    // FX-XF-001: "identity preserves pixels and bounds."
    //
    // The source carries a different value in every channel of every pixel, so a transposed
    // matrix, an off-by-one in the sampler or a swapped row stride all show up as a
    // difference rather than cancelling out on a symmetric image.
    let mut asymmetric = Vec::new();
    for y in 0..4 {
        for x in 0..4 {
            let k = (y * 4 + x) as f32;
            asymmetric.push((x, y, [k / 16.0, k / 32.0, k / 64.0, (k + 1.0) / 16.0]));
        }
    }
    let source = working(4, 4, &asymmetric);
    let identity = render(&plan(4, 4, one_layer(source.clone(), Affine::IDENTITY)), 4);
    report.check(
        "FX-XF-001: identity output extent equals the composition extent",
        "4x4",
        format!("{}x{}", identity.width(), identity.height()),
    );
    report.check(
        "FX-XF-001: identity reproduces every source pixel exactly, bit for bit",
        "identical",
        if identity.data() == source.data() {
            "identical".to_string()
        } else {
            let i = identity
                .data()
                .iter()
                .zip(source.data())
                .position(|(a, b)| a != b)
                .expect("lengths differ");
            format!("first difference at float {i}")
        },
    );

    // FX-XF-002: "integer translation moves a 1x1 impulse exactly one pixel."
    //
    // A whole-pixel shift lands the destination centre exactly on a source centre, so the
    // bilinear weights are exactly 1 and 0. Any blur here is a bug in the sampler, not a
    // property of bilinear filtering.
    let translated = render(
        &plan(
            5,
            5,
            one_layer(impulse_source(5, (2, 2)), Affine::translation(1.0, 0.0)),
        ),
        5,
    );
    report.check(
        "FX-XF-002: the impulse lands whole on the pixel one to the right",
        scaled(IMPULSE, 1.0),
        px(&translated, 3, 2),
    );
    report.check(
        "FX-XF-002: exactly one pixel is touched, so nothing was smeared",
        "[(3, 2)]",
        format!("{:?}", nonzero(&translated)),
    );
    report.check(
        "FX-XF-002: the pixel the impulse left is exactly transparent black",
        "[0, 0, 0, 0]",
        px(&translated, 2, 2),
    );

    // FX-XF-003: "half-pixel translation verifies bilinear weights."
    //
    // Shifting by half a pixel on both axes puts each destination centre exactly between four
    // source centres, so every weight is 0.5 * 0.5 = 0.25. The four values below are that
    // product times the impulse, written out rather than measured.
    let half = render(
        &plan(
            5,
            5,
            one_layer(impulse_source(5, (2, 2)), Affine::translation(0.5, 0.5)),
        ),
        5,
    );
    let quarter = scaled(IMPULSE, 0.25);
    for (x, y) in [(2, 2), (3, 2), (2, 3), (3, 3)] {
        report.check(
            &format!("FX-XF-003: pixel ({x}, {y}) carries one quarter of the impulse"),
            &quarter,
            px(&half, x, y),
        );
    }
    report.check(
        "FX-XF-003: exactly four pixels are touched",
        "[(2, 2), (3, 2), (2, 3), (3, 3)]",
        format!("{:?}", nonzero(&half)),
    );
    report.check(
        "FX-XF-003: the four quarters sum to the whole impulse, so no energy was invented",
        1.0,
        total_alpha(&half),
    );
    report.check(
        "FX-XF-003: the weights are exactly 0.25, not merely close",
        "0.25",
        (half.pixel(2, 2)[3]).to_string(),
    );

    // The same half-pixel shift with the impulse in the top-left corner. Three of the four
    // taps fall outside the source, and document 21 says a sample outside the source extent
    // is transparent black. Weights are not renormalised, so three quarters of the impulse
    // is genuinely lost rather than being redistributed onto the surviving tap.
    let corner = render(
        &plan(
            5,
            5,
            one_layer(impulse_source(5, (0, 0)), Affine::translation(-0.5, -0.5)),
        ),
        5,
    );
    report.check(
        "outside the source extent is transparent black, so edge weights are not renormalised",
        0.25,
        total_alpha(&corner),
    );
    report.check(
        "the surviving quarter is the top-left pixel",
        "[(0, 0)]",
        format!("{:?}", nonzero(&corner)),
    );

    // The same rule seen on a solid edge rather than an impulse: a fully opaque square shifted
    // half a pixel must fade at its border, not clamp to the edge pixel and stay opaque.
    let solid = working(
        2,
        2,
        &[
            (0, 0, [1.0, 1.0, 1.0, 1.0]),
            (1, 0, [1.0, 1.0, 1.0, 1.0]),
            (0, 1, [1.0, 1.0, 1.0, 1.0]),
            (1, 1, [1.0, 1.0, 1.0, 1.0]),
        ],
    );
    let edge = render(
        &plan(4, 4, one_layer(solid, Affine::translation(0.5, 0.5))),
        4,
    );
    report.check(
        "an opaque edge shifted half a pixel fades to 0.25 at its corner, it does not clamp",
        0.25,
        edge.pixel(0, 0)[3],
    );
    report.check("and to 0.5 along its side", 0.5, edge.pixel(1, 0)[3]);
    report.check("the interior stays fully opaque", 1.0, edge.pixel(1, 1)[3]);

    // FX-XF-004: "rotates around a nonzero anchor using the matrix order in 21."
    //
    // Anchor and position are both (2.5, 2.5), the centre of pixel (2, 2) in a 5x5 image, so
    // the layer rotates in place. Working through `T(position) * R(90) * S(1) * T(-anchor)`
    // by hand for the impulse at pixel (1, 2), whose centre is (1.5, 2.5):
    //   T(-anchor) -> (-1, 0); R(90) with cos=0, sin=1 -> (0, -1); T(position) -> (2.5, 1.5),
    // which is the centre of pixel (2, 1). Left of centre becomes above centre, which is
    // clockwise, as document 21 requires of a positive rotation in screen coordinates.
    let rotated = render(
        &plan(
            5,
            5,
            one_layer(
                impulse_source(5, (1, 2)),
                Affine::from_transform((2.5, 2.5), (2.5, 2.5), (1.0, 1.0), 90.0),
            ),
        ),
        5,
    );
    report.check(
        "FX-XF-004: a 90 degree rotation about the anchor sends left-of-centre to above-centre",
        format!(
            "[{:.6}, {:.6}, {:.6}, {:.6}]",
            IMPULSE[0], IMPULSE[1], IMPULSE[2], IMPULSE[3]
        ),
        rounded(&rotated, 2, 1),
    );
    report.check(
        "FX-XF-004: the pixel the impulse left is empty to six decimal places",
        "[0.000000, 0.000000, 0.000000, 0.000000]",
        rounded(&rotated, 1, 2),
    );
    report.check(
        "FX-XF-004: total alpha is preserved, so the rotation neither lost nor invented cover",
        "1.000000",
        format!("{:.6}", total_alpha(&rotated)),
    );
    report.note(
        "FX-XF-004 is checked to six decimal places rather than exactly. `cos(90 degrees)` in \
         f64 is 6.1e-17 and not 0, so a right-angle rotation leaks about 1e-16 of the impulse \
         into its neighbours. Document 21 asks for \"exact/near-exact float comparisons \
         appropriate to the operation\"; that is what near-exact means for a trigonometric \
         one. Every other fixture on this page is compared exactly.",
    );

    // A 360 degree rotation is the identity geometrically but not in floating point, and it is
    // worth knowing which. This is not one of document 25's fixtures; it is here because the
    // row above claims a tolerance and this shows the size of the thing being tolerated.
    let full_turn = render(
        &plan(
            5,
            5,
            one_layer(
                impulse_source(5, (2, 2)),
                Affine::from_transform((2.5, 2.5), (2.5, 2.5), (1.0, 1.0), 360.0),
            ),
        ),
        5,
    );
    report.check(
        "a 360 degree rotation returns the impulse to six decimal places",
        "[0.250000, 0.500000, 0.750000, 1.000000]",
        rounded(&full_turn, 2, 2),
    );

    // Scale, as a unit factor. D-22 records why 1.0 and not 100.
    //
    // A fully opaque 4x4 source scaled by two covers the whole 8x8 output geometrically, but
    // only its 6x6 interior is fully opaque: along the border the outer bilinear tap falls
    // outside the source and contributes transparent black, exactly as it does in the corner
    // fixture above. Destination centre (i+0.5) maps to source (i+0.5)/2, which has both taps
    // inside the source only for i in 1..=6, which is six values on each axis.
    let opaque_4x4: Vec<(usize, usize, [f32; 4])> = (0..4)
        .flat_map(|y| (0..4).map(move |x| (x, y, [1.0, 1.0, 1.0, 1.0])))
        .collect();
    let fully_opaque =
        |frame: &WorkingBuffer| frame.data().chunks_exact(4).filter(|p| p[3] == 1.0).count();
    let unscaled = render(
        &plan(
            8,
            8,
            one_layer(
                working(4, 4, &opaque_4x4),
                Affine::from_transform((0.0, 0.0), (0.0, 0.0), (1.0, 1.0), 0.0),
            ),
        ),
        8,
    );
    report.check(
        "scale 1.0 is identity: the opaque 4x4 source stays 4x4 of fully opaque output",
        16,
        fully_opaque(&unscaled),
    );
    let doubled = render(
        &plan(
            8,
            8,
            one_layer(
                working(4, 4, &opaque_4x4),
                Affine::from_transform((0.0, 0.0), (0.0, 0.0), (2.0, 2.0), 0.0),
            ),
        ),
        8,
    );
    report.check(
        "scale 2.0 doubles it: 8x8 of coverage with a fully opaque 6x6 interior",
        36,
        fully_opaque(&doubled),
    );
    report.check(
        "a scale of zero renders nothing rather than dividing by zero",
        0.0,
        total_alpha(&render(
            &plan(
                4,
                4,
                one_layer(
                    working(4, 4, &[(0, 0, [1.0, 1.0, 1.0, 1.0])]),
                    Affine::from_transform((0.0, 0.0), (0.0, 0.0), (0.0, 1.0), 0.0),
                ),
            ),
            4,
        )),
    );

    // Opacity, document 21's step 6. Applied to the premultiplied sample, so RGB and alpha
    // scale together and the composite stays premultiplied.
    let faded = render(
        &plan(
            5,
            5,
            vec![LayerDraw {
                id: Id::new("fixture"),
                source: impulse_source(5, (2, 2)),
                transform: Affine::IDENTITY,
                opacity: 0.5,
                blend: BlendMode::Normal,
            }],
        ),
        5,
    );
    report.check(
        "layer opacity 0.5 halves the premultiplied sample, RGB and alpha together",
        scaled(IMPULSE, 0.5),
        px(&faded, 2, 2),
    );

    // The sampler on its own, away from the renderer, at a coordinate whose weights are not
    // all equal: 0.25 across and 0.75 down gives 0.75 * 0.25 for the top-right tap.
    let two_by_two = working(
        2,
        2,
        &[
            (0, 0, [0.0, 0.0, 0.0, 0.0]),
            (1, 0, [1.0, 1.0, 1.0, 1.0]),
            (0, 1, [0.0, 0.0, 0.0, 0.0]),
            (1, 1, [0.0, 0.0, 0.0, 0.0]),
        ],
    );
    // Sampling at (0.9, 0.6) sits 0.4 of the way from source column 0 to column 1 and 0.1 of
    // the way from row 0 to row 1, so the only nonzero tap, pixel (1, 0), weighs 0.4 * 0.9.
    // The two weights differ, so an implementation that swapped the axes would report 0.06.
    report.check(
        "bilinear weight of the one nonzero tap at (0.9, 0.6) is 0.4 * 0.9",
        "0.360000",
        format!("{:.6}", sample_bilinear(&two_by_two, 0.9, 0.6)[3]),
    );
    report.check(
        "and swapping the sample coordinates gives a different answer, 0.1 * 0.6",
        "0.060000",
        format!("{:.6}", sample_bilinear(&two_by_two, 0.6, 0.9)[3]),
    );

    write_fixture_artifact(&report);
    assert_report(&report);
}

// -- ADR-011: tiling must not change results -------------------------------------------------

#[test]
fn b05a_tiling_is_invisible() {
    let mut report = Report::default();
    let scene = reference_plan(480, 270);
    report.check(
        "the identical-output plan draws every reference-shot layer",
        4,
        scene.layers.len(),
    );

    let reference = whole_frame(&scene);
    let reference_alpha = total_alpha(&reference);
    report.check(
        "the whole-frame render is not blank, so the comparison below has something to compare",
        true,
        reference_alpha > 0.0,
    );

    // Tile sizes chosen to include ones that divide the extent evenly and ones that do not,
    // and a 1x1 tile, which is the most hostile case the assembly step can be given.
    for tile in [1usize, 7, 16, 64, 100, 256] {
        for threads in [1usize, 2, 4, 12, 24] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .expect("thread pool");
            let out = pool.install(|| render(&scene, tile));
            report.check(
                &format!("tile {tile}px across {threads} threads is byte-identical to whole-frame"),
                "identical",
                if out.data() == reference.data() {
                    "identical".to_string()
                } else {
                    let i = out
                        .data()
                        .iter()
                        .zip(reference.data())
                        .position(|(a, b)| a != b)
                        .expect("lengths differ");
                    format!("first difference at float {i}")
                },
            );
        }
    }

    report.note(format!(
        "The comparison is `==` on the raw float32 buffers, not a tolerance. {} of the {} \
         float values in the frame are nonzero.",
        reference.data().iter().filter(|v| **v != 0.0).count(),
        reference.data().len()
    ));

    write_tiling_artifact(&report);
    assert_report(&report);
}

// -- the reference shot, rendered -------------------------------------------------------------

/// A frame of the reference shot as a render plan: every layer decoded into the working space,
/// with a transform that spreads the four layers apart so the artifact shows the transform
/// doing something rather than four stacked full-frame images.
fn reference_plan(width: usize, height: usize) -> FramePlan {
    let scale = width as f64 / 1920.0;
    // Layer 1 fills the frame; the other three are shrunk and placed at three corners, and
    // layer 3 is turned, so a wrong matrix order or a swapped sign is visible in the artifact.
    let placements = [
        ((0.0, 0.0), (0.0, 0.0), (1.0, 1.0), 0.0),
        ((960.0, 540.0), (480.0, 270.0), (0.4, 0.4), 0.0),
        ((960.0, 540.0), (1440.0, 270.0), (0.4, 0.4), 12.0),
        ((960.0, 540.0), (960.0, 810.0), (0.4, 0.4), 0.0),
    ];
    let mut layers = Vec::new();
    for (i, (anchor, position, layer_scale, rotation)) in placements.iter().enumerate() {
        let name = format!("layer{}", i + 1);
        let Some(source) = decode_layer(&name, 0) else {
            continue;
        };
        let source = if width == 1920 {
            source
        } else {
            downsample(&source, width, height)
        };
        // The source is scaled by the same factor as the composition, so anchor and position
        // scale with it and the layer's own scale is unchanged.
        layers.push(LayerDraw {
            id: Id::new(&name),
            source,
            transform: Affine::from_transform(
                (anchor.0 * scale, anchor.1 * scale),
                (position.0 * scale, position.1 * scale),
                *layer_scale,
                *rotation,
            ),
            opacity: 1.0,
            blend: BlendMode::Normal,
        });
    }
    plan(width, height, layers)
}

/// Decode the drawing a layer exposes on one composition frame, into the working space.
fn decode_layer(name: &str, frame: i32) -> Option<WorkingBuffer> {
    let dir = repo("Fixtures/reference_shot").join(name);
    let files: Vec<PathBuf> = fs::read_dir(&dir)
        .ok()?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "png"))
        .collect();
    let asset = import_sequence(&files).asset?;
    let first = asset.range()?.0;
    let exposures = ExposureMap::new(vec![anime_compositor::time::ExposureSpan {
        start_frame: 0,
        end_frame_exclusive: 240,
        drawing_number: first,
    }])
    .ok()?;
    let timing = LayerTiming {
        in_frame: 0,
        out_frame: 240,
        source_offset_frames: 0,
    };
    match resolve(&timing, &exposures, &asset, frame).ok()? {
        SourceAt::Drawing { number, .. } => Some(asset.decode(number).ok()?.into_working()),
        SourceAt::Transparent => None,
    }
}

/// Box-average an image down by an integer factor. Only used to keep the identical-output
/// matrix and the debug-profile test run cheap; nothing in `src/` depends on it, and it is
/// not a resampling contract.
fn downsample(src: &WorkingBuffer, width: usize, height: usize) -> WorkingBuffer {
    let fx = src.width() / width;
    let fy = src.height() / height;
    let mut out = WorkingBuffer::transparent(width, height);
    let data = out.data_mut();
    for y in 0..height {
        for x in 0..width {
            let mut acc = [0.0f32; 4];
            for sy in 0..fy {
                for sx in 0..fx {
                    let p = src.pixel(x * fx + sx, y * fy + sy);
                    for c in 0..4 {
                        acc[c] += p[c];
                    }
                }
            }
            let n = (fx * fy) as f32;
            let i = (y * width + x) * 4;
            for c in 0..4 {
                data[i + c] = acc[c] / n;
            }
        }
    }
    out
}

/// The owner-facing picture: one composited frame of the reference shot, first with every
/// layer left where it was drawn, then with three of the four moved, shrunk and one turned.
/// The pair is the check: the first image says the stack composites, the second says the
/// transform does what its numbers say, and the difference between them is the whole of R-03.
#[test]
fn b05a_renders_a_frame_of_the_reference_shot() {
    let mut flat = reference_plan(1920, 1080);
    assert_eq!(flat.layers.len(), 4, "all four reference layers decoded");
    for layer in &mut flat.layers {
        layer.transform = Affine::IDENTITY;
    }
    let untransformed = render(&flat, 128);
    assert!(
        total_alpha(&untransformed) > 0.0,
        "the composited frame is not blank"
    );
    write_png(
        &repo("verification/B-05a_reference_frame_untransformed.png"),
        &untransformed,
    );

    let scene = reference_plan(1920, 1080);
    let frame = render(&scene, 128);
    assert!(
        frame.data() != untransformed.data(),
        "the transformed render differs from the untransformed one"
    );
    write_png(&repo("verification/B-05a_reference_frame.png"), &frame);
}

/// The scaling table ADR-011 asks for. Ignored by default because a debug build measures the
/// compiler's inlining decisions rather than the renderer, and because thirty full-resolution
/// renders do not belong in every `cargo test`. Run it deliberately:
///
/// ```text
/// cargo test --release --test b05a_transform -- --ignored --nocapture
/// ```
#[test]
#[ignore = "timing; run under --release with --ignored"]
fn b05a_scaling_table() {
    let scene = reference_plan(1920, 1080);
    let mut rows = Vec::new();
    let mut baseline = None;
    for tile in [32usize, 64, 128, 256] {
        for threads in [1usize, 2, 4, 8, 12, 24] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .expect("thread pool");
            // One untimed render first, so the measured one is not paying for the first touch
            // of freshly allocated pages.
            pool.install(|| render(&scene, tile));
            let start = std::time::Instant::now();
            let frame = pool.install(|| render(&scene, tile));
            let elapsed = start.elapsed().as_secs_f64() * 1000.0;
            match &baseline {
                None => baseline = Some(frame.data().to_vec()),
                Some(b) => assert!(
                    frame.data() == b.as_slice(),
                    "tile {tile} on {threads} threads did not match the first render"
                ),
            }
            rows.push((tile, threads, elapsed));
        }
    }
    write_scaling_artifact(&rows);
}

fn write_scaling_artifact(rows: &[(usize, usize, f64)]) {
    let single: std::collections::HashMap<usize, f64> = rows
        .iter()
        .filter(|(_, t, _)| *t == 1)
        .map(|(tile, _, ms)| (*tile, *ms))
        .collect();
    let mut s = String::from("# B-05a tiled render, scaling by thread count\n\n");
    s.push_str(
        "ADR-011: \"identical output to single-threaded evaluation, plus measured scaling on \
         the reference machine\". The identical-output half is `B-05a_tiling_proof.md`; this \
         is the timing half. Produced by `tests/b05a_transform.rs`, which is `#[ignore]`d in \
         normal runs.\n\n",
    );
    s.push_str(
        "## Machine, build and configuration\n\n\
         - CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads\n\
         - OS: Microsoft Windows 11 Education, 10.0.26200\n\
         - Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`\n\
         - Workload: one 1920x1080 frame, four reference-shot layers, each transformed and \
         composited, bilinear sampling throughout\n\
         - Each figure is one render, preceded by an untimed warm render on the same pool\n\n",
    );
    let _ = writeln!(
        s,
        "Debug assertions in this build: {}. A run with `true` there is a debug build, and its \
         numbers say more about the compiler than about the renderer.\n",
        cfg!(debug_assertions)
    );
    s.push_str(
        "## Measurements\n\n\
         | Tile | Threads | Milliseconds | Speed-up over 1 thread |\n|---|---|---|---|\n",
    );
    for (tile, threads, ms) in rows {
        let speedup = single
            .get(tile)
            .map(|base| format!("{:.2}x", base / ms))
            .unwrap_or_else(|| "-".to_string());
        let _ = writeln!(s, "| {tile}px | {threads} | {ms:.1} | {speedup} |");
    }
    s.push_str(
        "\n## How to read this\n\nDocument 21: \"Tile size is a tunable measured on the \
         reference machine, not a constant chosen in advance.\" That is what the tile column \
         is for. Too small and the render spends its time in scheduling; too large and threads \
         sit idle at the end of the frame while the last few tiles finish. The best row is a \
         measurement, not a number chosen ahead of time, and no default tile size is hard-coded \
         anywhere in `src/`.\n\n\
         Speed-up is measured against the same tile size on one thread, so it describes the \
         scaling of the parallel decomposition rather than comparing tile sizes with each \
         other. Perfect scaling is not expected: frame assembly is serial, the machine has 12 \
         physical cores behind its 24 hardware threads, and a workload that reads four \
         full-resolution layers per frame is partly bound by memory bandwidth.\n\n\
         Every render in this table was compared against the first one and was byte-identical, \
         so nothing here trades correctness for speed.\n",
    );
    fs::write(repo("verification/B-05a_scaling_table.md"), s).expect("write artifact");
}

fn write_png(path: &Path, frame: &WorkingBuffer) {
    // B-05b moved the encoder into the crate for the trace facility; this is its second caller.
    anime_compositor::trace::write_png(path, frame, &[]).expect("write png");
}

// -- artifacts --------------------------------------------------------------------------------

fn assert_report(report: &Report) {
    let failures = report.failures();
    assert!(
        failures.is_empty(),
        "{} of {} checks failed, first: {} expected {:?} got {:?}",
        failures.len(),
        report.rows.len(),
        failures[0].check,
        failures[0].expected,
        failures[0].actual
    );
}

fn table(report: &Report) -> String {
    let mut s = String::from("| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
    for row in &report.rows {
        let _ = writeln!(
            s,
            "| {} | `{}` | `{}` | {} |",
            row.check,
            row.expected,
            row.actual,
            if row.pass() { "PASS" } else { "**FAIL**" }
        );
    }
    s
}

fn write_fixture_artifact(report: &Report) {
    let mut s = format!(
        "# B-05a transform fixtures\n\nTest T-03 (render half), requirement R-03, fixtures \
         FX-XF-001 through FX-XF-004 of document 25. Produced by `tests/b05a_transform.rs`. \
         **{} of {} checks pass.**\n\n",
        report.rows.len() - report.failures().len(),
        report.rows.len()
    );
    s.push_str(
        "## What to check by eye\n\nEvery expected value in the table is a literal taken from \
         document 21's rules, worked out by hand before the renderer ran. The comments in the \
         test say where each one comes from. Nothing here was captured from a run of the code \
         it is testing.\n\nThe four fixtures document 25 names:\n\n\
         - **FX-XF-001**, identity preserves pixels and bounds. A 4x4 image where no two pixels \
         and no two channels share a value goes in, and the same floats come out.\n\
         - **FX-XF-002**, integer translation moves a 1x1 impulse exactly one pixel. One pixel \
         is touched, not two: a whole-pixel shift must not blur.\n\
         - **FX-XF-003**, half-pixel translation verifies bilinear weights. The impulse becomes \
         four pixels of exactly one quarter each, because 0.5 across times 0.5 down is 0.25.\n\
         - **FX-XF-004**, rotation about a nonzero anchor. A pixel to the left of the anchor \
         ends up above it, which is clockwise, which is what document 21 calls positive.\n\n\
         Beside them are rows for the edge rule, opacity and scale, because those are the parts \
         of the same code path that the four fixtures do not exercise.\n\n",
    );
    s.push_str("## Checks\n\n");
    s.push_str(&table(report));
    if !report.notes.is_empty() {
        s.push_str("\n## Notes\n\n");
        for note in &report.notes {
            let _ = writeln!(s, "- {note}\n");
        }
    }
    s.push_str(
        "\n## Not run by this test\n\n\
         - Masks, effects and alpha mattes, which are steps 2, 3 and 5 of document 21's layer \
         render order. Masks are parked to G1-rest with R-04 under D-12; effects and mattes are \
         B-06. A layer here is decoded, transformed, faded and composited, and nothing else.\n\
         - The multiply, screen and add blend modes, which are now implemented and are \
         verified in `verification/B-05c_blend_table.md` against document 25's FX-B fixtures. \
         Every layer in this table is `normal`.\n\
         - Tile margins for neighbourhood operations. Every operation in G1-core is per-pixel, \
         so no margin is needed yet. Document 21 says the first one that needs it is the blur \
         in R-05, which is parked.\n\
         - Sub-pixel-accurate polygon rasterisation, which document 21 requires be \
         fixture-tested before subpixel equivalence is claimed. No polygon exists yet.\n",
    );
    fs::write(repo("verification/B-05a_transform_table.md"), s).expect("write artifact");
}

fn write_tiling_artifact(report: &Report) {
    let mut s = format!(
        "# B-05a tiled render, identical-output proof\n\nADR-011 and document 21's tile \
         contract. Produced by `tests/b05a_transform.rs`. **{} of {} checks pass.**\n\n",
        report.rows.len() - report.failures().len(),
        report.rows.len()
    );
    s.push_str(
        "## What to check by eye\n\nDocument 21: \"a tiled render and a hypothetical whole-frame \
         render of the same request must be byte-identical, and B-05a proves this rather than \
         assuming it\". Each row below renders the same four-layer reference-shot frame at one \
         tile size on one number of worker threads, and compares the whole float32 buffer \
         against a single whole-frame render. Every row must say `identical`. A row that named \
         a float index instead would mean the picture changes depending on how the machine \
         happened to divide the work, which is the failure this task exists to rule out.\n\n\
         Thirty combinations are covered: six tile sizes, including one that is a single pixel \
         and two that do not divide the frame evenly, across five thread counts from one to \
         twenty-four.\n\n",
    );
    s.push_str("## Checks\n\n");
    s.push_str(&table(report));
    if !report.notes.is_empty() {
        s.push_str("\n## Notes\n\n");
        for note in &report.notes {
            let _ = writeln!(s, "- {note}\n");
        }
    }
    s.push_str(
        "\n## Why this holds\n\nA tile owns its own accumulator and reads only immutable source \
         buffers, so no float addition ever changes its order with thread count, and the \
         assembly step writes each tile to a position fixed before any thread started. \
         Determinism here is a property of the structure rather than something the test coaxes \
         out of it. The test exists because ADR-011 asks for proof, not because the structure \
         is in doubt.\n\n\
         This is not the same claim as SP-04's render determinism across runs, which is about \
         two separate invocations, or B-10's, which is about two exported sequences. Those \
         remain their own tests.\n",
    );
    fs::write(repo("verification/B-05a_tiling_proof.md"), s).expect("write artifact");
}
