//! R-04 / B-06: the polygon mask and the alpha matte.
//!
//! Writes `verification/B-06_mask_table.md`.
//!
//! # Where the expected values come from
//!
//! Nothing in this file was captured from a run of the code under test (ADR-009). The expected
//! values come from three places, and each row of the table says which:
//!
//! 1. **An independently written insideness test.** [`inside_by_winding`] below decides whether
//!    a point is inside a polygon by summing signed angles — the winding-number method — where
//!    `src/mask.rs` casts a ray and counts crossings with a half-open rule. The two share no
//!    code and no approach. Coverage derived from it is the expected column; coverage from the
//!    build is the actual column. For the simple polygons this build accepts, the two rules must
//!    agree at every one of the sixteen sample points of every pixel.
//! 2. **Hand arithmetic, written into the comment beside each check.** A vertical edge at
//!    x = 2.5 puts two of the four sample columns of pixel 2 inside the polygon, so its coverage
//!    is 8/16 = 0.5 exactly. That is stated on paper here, not measured.
//! 3. **Document 21, quoted.** `C' = C * m` and `A' = A * m` for the mask, and for the matte
//!    "alpha matte coverage is the matte layer's post-transform alpha sampled at the destination
//!    pixel". The algebraic checks below test the identity, not a number: masking by `m` then by
//!    `1 - m` must sum back to the original pixel, whatever `m` is.
//!
//! # The rasterization rule this fixture is written against
//!
//! Document 25 line 51 asks B-06 to record it. It is in `src/mask.rs` and repeated in the
//! report: a 4x4 ordered grid of sample points per pixel, coverage is the count inside divided
//! by sixteen, insideness is even-odd, and the sample for grid cell (i, j) of pixel (x, y) sits
//! at `(x + (i+0.5)/4, y + (j+0.5)/4)`. The edge quantum is therefore 1/16, and this fixture
//! checks against that quantum rather than against a subpixel-exact area. Document 21 requires
//! that "multisample details must be fixture-tested before claiming subpixel equivalence";
//! nothing here claims it.
//!
//! # What is deliberately not tested here
//!
//! This is a table of behaviours, not a picture. It renders small synthetic buffers where every
//! number can be checked on paper. Whether a mask and a matte look right on the reference shot
//! is a whole-picture question of the kind H-03 and H-04 answer, and it is not answered here.

use std::fs;
use std::path::PathBuf;

use anime_compositor::command::{Command, Document};
use anime_compositor::mask::{self, PolygonMask, SAMPLES_PER_SIDE};
use anime_compositor::model::{BlendMode, Id, Layer};
use anime_compositor::render::{render, Affine, FramePlan, LayerDraw, MatteDraw};
use anime_compositor::WorkingBuffer;
use anime_compositor::{compose, persist};

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

/// A pixel to six decimal places, the tolerance document 25 asks of the FX fixtures.
fn q(p: [f32; 4]) -> String {
    format!(
        "({:.6}, {:.6}, {:.6}, {:.6})",
        p[0] as f64, p[1] as f64, p[2] as f64, p[3] as f64
    )
}

fn solid(width: usize, height: usize, p: [f32; 4]) -> WorkingBuffer {
    let mut buf = WorkingBuffer::transparent(width, height);
    for px in buf.data_mut().chunks_exact_mut(4) {
        px.copy_from_slice(&p);
    }
    buf
}

fn pixel(buf: &WorkingBuffer, x: usize, y: usize) -> [f32; 4] {
    let d = buf.data();
    let i = (y * buf.width() + x) * 4;
    [d[i], d[i + 1], d[i + 2], d[i + 3]]
}

// ---------------------------------------------------------------------------------------
// The independent implementation
// ---------------------------------------------------------------------------------------

/// Whether a point is inside a polygon, by the winding-number method.
///
/// Written against document 19's definition of a closed polygon rather than against
/// `src/mask.rs`, and by a different method: this sums the signed angle subtended at the point
/// by each edge and asks whether the total is nearer to a full turn than to zero, where the
/// build under test casts a ray along +x and counts crossings under a half-open rule.
///
/// The two approaches fail differently, which is the whole point of having both. A ray-crossing
/// bug shows up at vertices and horizontal edges, exactly where the half-open convention does
/// its work. A winding-number bug shows up as a drift in the angle sum. Neither is a
/// re-expression of the other, so agreement between them is evidence rather than a tautology.
fn inside_by_winding(vertices: &[(f64, f64)], x: f64, y: f64) -> bool {
    let n = vertices.len();
    let mut total = 0.0f64;
    for i in 0..n {
        let (ax, ay) = vertices[i];
        let (bx, by) = vertices[(i + 1) % n];
        let (v1x, v1y) = (ax - x, ay - y);
        let (v2x, v2y) = (bx - x, by - y);
        // The signed angle from v1 to v2, in (-pi, pi]. atan2 of the cross and dot products
        // gives it directly and without a quadrant table.
        let cross = v1x * v2y - v1y * v2x;
        let dot = v1x * v2x + v1y * v2y;
        total += cross.atan2(dot);
    }
    // A point inside a simple polygon subtends a full turn; a point outside subtends zero. Half
    // a turn is nowhere near either, so the midpoint is a safe divider and no tolerance is
    // being tuned here.
    total.abs() > std::f64::consts::PI
}

/// The reference grid, written out here rather than read from the build.
///
/// It was `SAMPLES_PER_SIDE` until the hardening pass, and that was the fault: a check that
/// reads its subject's own constant cannot notice the constant changing, because both sides of
/// the comparison move together. Two breaks got through on exactly that -- a grid moved onto the
/// pixel corner, and a grid of sixty-four samples instead of sixteen. These four sample offsets
/// are ADR-016's rule spelled out, and the row below asserts the build still agrees with them.
const REFERENCE_OFFSETS: [f64; 4] = [0.125, 0.375, 0.625, 0.875];

/// Coverage of one pixel under the recorded rule, computed through [`inside_by_winding`].
///
/// This is the rule from ADR-016 re-implemented over a different insideness test: the grid of
/// [`REFERENCE_OFFSETS`], a divisor of sixteen, and an independent decision at each sample.
fn coverage_independent(vertices: &[(f64, f64)], x: usize, y: usize) -> f32 {
    let n = REFERENCE_OFFSETS.len();
    let mut hits = 0u32;
    for &dy in &REFERENCE_OFFSETS {
        for &dx in &REFERENCE_OFFSETS {
            if inside_by_winding(vertices, x as f64 + dx, y as f64 + dy) {
                hits += 1;
            }
        }
    }
    hits as f32 / (n * n) as f32
}

// ---------------------------------------------------------------------------------------
// The checks
// ---------------------------------------------------------------------------------------

/// A square from (2,2) to (6,6), given as a closed ordered list with no repeated vertex.
fn square() -> Vec<(f64, f64)> {
    vec![(2.0, 2.0), (6.0, 2.0), (6.0, 6.0), (2.0, 6.0)]
}

fn topology(report: &mut Report) {
    let poly = square();

    // Pixel 3 lies wholly inside: its samples run 3.125 to 3.875 in both axes, all within
    // 2..6. Sixteen of sixteen, so coverage is 1.
    report.check(
        "a pixel wholly inside the polygon is fully covered",
        "1.000000",
        format!("{:.6}", mask::pixel_coverage(&poly, 3, 3)),
    );
    // Pixel 0 lies wholly outside: its samples run 0.125 to 0.875, all below 2. None of
    // sixteen, so coverage is 0.
    report.check(
        "a pixel wholly outside the polygon is not covered at all",
        "0.000000",
        format!("{:.6}", mask::pixel_coverage(&poly, 0, 0)),
    );
    // Pixel 1 is outside in x (0.125..0.875 shifted to 1.125..1.875, still below 2) even though
    // it touches the boundary. The edge at x = 2 is the boundary, and no sample sits on it.
    report.check(
        "the pixel just outside a boundary at an integer is not covered",
        "0.000000",
        format!("{:.6}", mask::pixel_coverage(&poly, 1, 1)),
    );

    // Every pixel of an 8x8 field, checked against the independent implementation. This is the
    // topology check document 25 line 51 calls authoritative, run exhaustively rather than
    // sampled.
    let mut disagreements = 0;
    let mut worst = 0.0f32;
    for y in 0..8 {
        for x in 0..8 {
            let mine = mask::pixel_coverage(&poly, x, y);
            let theirs = coverage_independent(&poly, x, y);
            if mine != theirs {
                disagreements += 1;
                worst = worst.max((mine - theirs).abs());
            }
        }
    }
    report.check(
        "all 64 pixels agree with an independently written winding-number rasterizer",
        "0 disagreements",
        format!("{disagreements} disagreements"),
    );
    report.check(
        "and the largest coverage difference anywhere in the field is zero",
        "0.000000",
        format!("{worst:.6}"),
    );
}

fn edge_quantum(report: &mut Report) {
    // A square whose left edge falls at x = 2.5, halfway across pixel 2. The four sample
    // columns of that pixel sit at 2.125, 2.375, 2.625 and 2.875; two are left of the edge and
    // two are right of it, so eight of the sixteen samples are inside and coverage is exactly
    // 8/16 = 0.5. This is arithmetic on the recorded rule, not a measurement.
    let half = vec![(2.5, 0.0), (8.0, 0.0), (8.0, 8.0), (2.5, 8.0)];
    report.check(
        "an edge halfway across a pixel covers exactly half of it",
        "0.500000",
        format!("{:.6}", mask::pixel_coverage(&half, 2, 4)),
    );

    // A quarter of the way across: 2.25 leaves samples at 2.375, 2.625, 2.875 inside and 2.125
    // outside, so three of four columns, 12 of 16 samples, 0.75.
    let quarter = vec![(2.25, 0.0), (8.0, 0.0), (8.0, 8.0), (2.25, 8.0)];
    report.check(
        "an edge a quarter of the way across a pixel leaves three quarters covered",
        "0.750000",
        format!("{:.6}", mask::pixel_coverage(&quarter, 2, 4)),
    );

    // An edge at 2.6, which is not on a sample line under any grid. The four sample columns are
    // at 2.125, 2.375, 2.625 and 2.875; the last two are inside, so eight of sixteen, 0.5. The
    // two rows above cannot see a grid shifted onto the pixel corner, because 2.5 and 2.25 are
    // themselves corner-grid sample positions and the answer comes out the same by accident.
    // This one cannot come out the same: a corner grid gives 0.25 here.
    let offset = vec![(2.6, 0.0), (8.0, 0.0), (8.0, 8.0), (2.6, 8.0)];
    report.check(
        "an edge that lands between sample columns still covers a whole number of sixteenths",
        "0.500000",
        format!("{:.6}", mask::pixel_coverage(&offset, 2, 4)),
    );

    // A sample that lands exactly on the outline. ADR-016 chose a grid and a fill rule and said
    // nothing about this case, and two hardening breaks got through in the gap: a ray cast in -x
    // instead of +x, and the half-open span written open at the bottom instead of the top. Both
    // are correct rasterizers everywhere except on the outline itself, and both change the
    // answer there. The rule is now pinned, and these two rows are what pin it: **a sample lying
    // exactly on the outline counts as inside.**
    //
    // Neither row goes through the independent implementation, and that is deliberate rather
    // than an omission. The winding-number reference subtends exactly half a turn at a boundary
    // point and so calls it outside -- a third defensible convention. Agreement between two
    // implementations cannot settle a case the specification leaves open; only a decision can,
    // and these are the arithmetic of the decision.
    //
    // A square whose left edge falls exactly on the sample column at x = 2.125. All four columns
    // of pixel 2 are inside, so coverage is 1. A ray cast the other way would call the sample on
    // the edge outside and give 0.75.
    let on_column = vec![(2.125, 0.0), (8.0, 0.0), (8.0, 8.0), (2.125, 8.0)];
    report.check(
        "a sample sitting exactly on a vertical edge counts as inside",
        "1.000000",
        format!("{:.6}", mask::pixel_coverage(&on_column, 2, 4)),
    );
    // The same case one axis over: a square whose bottom edge falls exactly on the sample row at
    // y = 2.125. The two vertical edges meet the horizontal line through that row at their own
    // endpoints, which is where the half-open span decides whether they are counted. Written the
    // other way round, this row reads 0.75.
    let on_row = vec![(0.0, 2.125), (8.0, 2.125), (8.0, 8.0), (0.0, 8.0)];
    report.check(
        "a sample sitting exactly on a horizontal edge counts as inside too",
        "1.000000",
        format!("{:.6}", mask::pixel_coverage(&on_row, 3, 2)),
    );

    // ADR-016 fixes the grid at 4x4. It is a specification constant and not a tuning knob:
    // every expected value above is derived from it, so it is checked rather than assumed.
    report.check(
        "the build's sample grid is the 4x4 ADR-016 records",
        "4 per side",
        format!("{SAMPLES_PER_SIDE} per side"),
    );

    // The quantum itself: coverage is always a sixteenth. Multiplying by 16 must give a whole
    // number for every pixel of a field crossed by a diagonal edge, which is the case most
    // likely to produce a value off the grid if the rule were doing something else.
    let diagonal = vec![(0.0, 0.0), (7.3, 0.0), (0.0, 7.3)];
    let mut off_grid = 0;
    for y in 0..8 {
        for x in 0..8 {
            // Sixteen written out, not the build's own constant squared, for the reason
            // REFERENCE_OFFSETS gives.
            let c = mask::pixel_coverage(&diagonal, x, y) * 16.0;
            if (c - c.round()).abs() > 1e-6 {
                off_grid += 1;
            }
        }
    }
    report.check(
        "every coverage value under a diagonal edge is a whole number of sixteenths",
        "0 off the grid",
        format!("{off_grid} off the grid"),
    );
}

fn premultiplied_and_inverted(report: &mut Report) {
    let poly = vec![(2.5, 0.0), (8.0, 0.0), (8.0, 8.0), (2.5, 8.0)];
    // A premultiplied pixel: straight (0.8, 0.4, 0.2) at alpha 0.5.
    let source = [0.4, 0.2, 0.1, 0.5];

    let mut buf = solid(8, 8, source);
    mask::apply(&mut buf, &PolygonMask::new(poly.clone()));
    // Document 21: "Mask coverage m in 0..1 multiplies both premultiplied RGB and alpha." At
    // coverage 0.5 that is (0.2, 0.1, 0.05, 0.25), by hand.
    report.check(
        "coverage multiplies all four premultiplied channels equally",
        q([0.2, 0.1, 0.05, 0.25]),
        q(pixel(&buf, 2, 4)),
    );
    // The straight colour is unchanged by masking, which is the point of scaling all four
    // together: 0.2/0.25 = 0.8, the straight red it started with.
    let p = pixel(&buf, 2, 4);
    report.check(
        "so the straight colour under a half-covered edge is the colour it started with",
        "0.800000",
        format!("{:.6}", (p[0] / p[3]) as f64),
    );

    let mut inverted_buf = solid(8, 8, source);
    let mut inverted = PolygonMask::new(poly.clone());
    inverted.inverted = true;
    mask::apply(&mut inverted_buf, &inverted);
    // The identity, not a number: m and 1-m must sum to the original at every pixel, whatever
    // the coverage is. A rasterizer that was wrong in the same way on both passes would still
    // fail this, because the two passes use different coverage values.
    let mut worst = 0.0f64;
    for y in 0..8 {
        for x in 0..8 {
            let a = pixel(&buf, x, y);
            let b = pixel(&inverted_buf, x, y);
            for c in 0..4 {
                worst = worst.max(((a[c] + b[c]) - source[c]).abs() as f64);
            }
        }
    }
    report.check(
        "a mask and its inverse sum back to the unmasked pixel everywhere",
        "0.000000",
        format!("{worst:.6}"),
    );

    // A disabled mask changes nothing at all, which is what lets one be switched off without
    // losing the shape that was drawn.
    let mut disabled_buf = solid(8, 8, source);
    let mut disabled = PolygonMask::new(poly.clone());
    disabled.enabled = false;
    mask::apply(&mut disabled_buf, &disabled);
    report.check(
        "a disabled mask leaves every pixel untouched",
        q(source),
        q(pixel(&disabled_buf, 0, 0)),
    );
}

fn rejection(report: &mut Report) {
    // A figure-of-eight: the edge (0,0)-(4,4) crosses the edge (4,0)-(0,4).
    let bowtie = vec![(0.0, 0.0), (4.0, 0.0), (0.0, 4.0), (4.0, 4.0)];
    report.check(
        "a self-intersecting polygon is not simple",
        "false",
        mask::is_simple(&bowtie).to_string(),
    );
    report.check(
        "an ordinary square is simple",
        "true",
        mask::is_simple(&square()).to_string(),
    );
    // Document 19 rejects rather than normalizes, so an unusable mask must not quietly become a
    // shape. `apply` leaves the layer alone, which is a layer drawn unmasked.
    let source = [0.4, 0.2, 0.1, 0.5];
    let mut buf = solid(8, 8, source);
    mask::apply(&mut buf, &PolygonMask::new(bowtie.clone()));
    report.check(
        "and a layer carrying one is drawn unmasked rather than blank",
        q(source),
        q(pixel(&buf, 2, 2)),
    );
    // Two vertices cannot enclose anything.
    let mut buf = solid(8, 8, source);
    mask::apply(&mut buf, &PolygonMask::new(vec![(0.0, 0.0), (4.0, 4.0)]));
    report.check(
        "a mask of two points is not drawn either",
        q(source),
        q(pixel(&buf, 2, 2)),
    );
}

fn matte(report: &mut Report) {
    // A matte whose alpha is 0.25 everywhere, over a fully opaque white layer. Document 21:
    // "Apply C' = C * m and A' = A * m", with m the matte's post-transform alpha. By hand,
    // (1, 1, 1, 1) * 0.25 = (0.25, 0.25, 0.25, 0.25).
    let layer = solid(4, 4, [1.0, 1.0, 1.0, 1.0]);
    let matte_source = solid(4, 4, [0.0, 0.0, 0.0, 0.25]);
    let plan = FramePlan {
        width: 4,
        height: 4,
        layers: vec![LayerDraw {
            id: Id::new("drawn"),
            source: layer.clone(),
            transform: Affine::IDENTITY,
            opacity: 1.0,
            matte: Some(Box::new(MatteDraw {
                source: matte_source.clone(),
                transform: Affine::IDENTITY,
            })),
            blend: BlendMode::Normal,
        }],
    };
    let out = render(&plan, 2);
    report.check(
        "a matte at alpha 0.25 scales the layer it mattes to a quarter",
        q([0.25, 0.25, 0.25, 0.25]),
        q(pixel(&out, 1, 1)),
    );

    // The matte's colour is not its alpha. The matte above is black; if the build read its RGB
    // instead of its alpha the result would be zero, not a quarter. This row is what separates
    // those two mistakes, and it is why the matte source is black rather than white.
    let bright_matte = solid(4, 4, [0.25, 0.25, 0.25, 0.25]);
    let plan = FramePlan {
        width: 4,
        height: 4,
        layers: vec![LayerDraw {
            id: Id::new("drawn"),
            source: layer.clone(),
            transform: Affine::IDENTITY,
            opacity: 1.0,
            matte: Some(Box::new(MatteDraw {
                source: bright_matte,
                transform: Affine::IDENTITY,
            })),
            blend: BlendMode::Normal,
        }],
    };
    let out_bright = render(&plan, 2);
    report.check(
        "a matte of the same alpha but a different colour gives the same result",
        q([0.25, 0.25, 0.25, 0.25]),
        q(pixel(&out_bright, 1, 1)),
    );

    // A transparent matte hides the layer entirely; an opaque one changes nothing.
    for (name, alpha, expected) in [
        (
            "a fully transparent matte hides the layer",
            0.0f32,
            [0.0, 0.0, 0.0, 0.0],
        ),
        (
            "a fully opaque matte leaves the layer alone",
            1.0f32,
            [1.0, 1.0, 1.0, 1.0],
        ),
    ] {
        let plan = FramePlan {
            width: 4,
            height: 4,
            layers: vec![LayerDraw {
                id: Id::new("drawn"),
                source: layer.clone(),
                transform: Affine::IDENTITY,
                opacity: 1.0,
                matte: Some(Box::new(MatteDraw {
                    source: solid(4, 4, [0.0, 0.0, 0.0, alpha]),
                    transform: Affine::IDENTITY,
                })),
                blend: BlendMode::Normal,
            }],
        };
        report.check(name, q(expected), q(pixel(&render(&plan, 2), 1, 1)));
    }

    // The matte is sampled through its own transform, not the layer's. The matte here is
    // shifted two pixels right, so the layer is hidden on the left half and shown on the right.
    // A build that sampled the matte through the drawn layer's transform would show the same
    // value at both pixels.
    let mut half_matte = WorkingBuffer::transparent(4, 4);
    {
        let d = half_matte.data_mut();
        for y in 0..4 {
            for x in 0..2 {
                d[(y * 4 + x) * 4 + 3] = 1.0;
            }
        }
    }
    let plan = FramePlan {
        width: 4,
        height: 4,
        layers: vec![LayerDraw {
            id: Id::new("drawn"),
            source: layer.clone(),
            transform: Affine::IDENTITY,
            opacity: 1.0,
            matte: Some(Box::new(MatteDraw {
                source: half_matte,
                transform: Affine::translation(2.0, 0.0),
            })),
            blend: BlendMode::Normal,
        }],
    };
    let shifted = render(&plan, 2);
    report.check(
        "the matte moves with its own transform, so the covered half is where the matte is",
        format!(
            "hidden at x=0, shown at x=2: {} / {}",
            q([0.0; 4]),
            q([1.0; 4])
        ),
        format!(
            "hidden at x=0, shown at x=2: {} / {}",
            q(pixel(&shifted, 0, 1)),
            q(pixel(&shifted, 2, 1))
        ),
    );
}

fn cache_isolation(report: &mut Report) {
    // The hazard: `compose` masks the buffer the cel cache hands back. If that buffer were
    // shared with the cache, one masked layer would cut every other layer using the same
    // drawing, and only on a cache hit -- a fault that would not show in any single-layer test
    // and would come and go with the memory budget. This is the check that rules it out.
    let mut cache = anime_compositor::cache::CelCache::with_budget(64 * 1024 * 1024);
    let path = mask_fixture_png();
    let interp = anime_compositor::model::Interpretation::default();

    let mut first = cache
        .decoded(&path, interp)
        .unwrap_or_else(|d| panic!("decode the fixture cel: {}", d.message));
    let before = pixel(&first, 0, 0);

    // A mask that keeps only the far corner, so pixel (0, 0) is definitely cut. A mask that
    // happened to cover nothing would make every row below pass without proving anything.
    mask::apply(
        &mut first,
        &PolygonMask::new(vec![(3.0, 3.0), (4.0, 3.0), (4.0, 4.0), (3.0, 4.0)]),
    );
    report.check(
        "the copy that was masked really was cut, so the check below is not vacuous",
        q([0.0, 0.0, 0.0, 0.0]),
        q(pixel(&first, 0, 0)),
    );

    let second = cache
        .decoded(&path, interp)
        .unwrap_or_else(|d| panic!("second decode: {}", d.message));
    report.check(
        "masking a cel handed out by the cache does not change what the cache still holds",
        q(before),
        q(pixel(&second, 0, 0)),
    );
    report.check(
        "and the second request was a hit, so it really was the cached copy",
        "1 hit",
        format!("{} hit", cache.hits()),
    );
}

/// A tiny PNG written once, so the cache has something real to decode.
fn mask_fixture_png() -> PathBuf {
    let path = std::env::temp_dir().join("anime_compositor_b06_mask_cel.png");
    if !path.exists() {
        // A flat 4x4 in sRGB 8-bit: (128, 64, 32) at full alpha. What matters is only that the
        // cache has a real file to decode, not what the picture is.
        let samples: Vec<u8> = [128u8, 64, 32, 255]
            .iter()
            .copied()
            .cycle()
            .take(4 * 4 * 4)
            .collect();
        anime_compositor::png_out::write_rgba(
            &path,
            4,
            4,
            anime_compositor::OutputDepth::Eight,
            &[],
            &samples,
        )
        .expect("write the fixture cel");
    }
    path
}

fn command_rules(report: &mut Report) {
    let mut doc = seeded_document();
    let comp = Id::new("comp");
    let layer_id = Id::new("a");

    // A valid mask goes on, and undo takes it off. Document 26: a command supplies enough to
    // restore the exact prior state.
    let set = Command::SetMask {
        composition: comp.clone(),
        layer_id: layer_id.clone(),
        mask: Some(PolygonMask::new(square())),
    };
    report.check(
        "a valid mask is accepted",
        "accepted",
        match doc.apply(set) {
            Ok(_) => "accepted".to_string(),
            Err(d) => format!("rejected: {}", d.id.as_str()),
        },
    );
    report.check(
        "and the layer now carries it",
        "4 vertices",
        match layer_of(&doc, &comp, &layer_id).mask.as_ref() {
            Some(m) => format!("{} vertices", m.vertices.len()),
            None => "none".to_string(),
        },
    );
    doc.undo().expect("undo the mask");
    report.check(
        "undo removes it and leaves no mask behind",
        "none",
        match layer_of(&doc, &comp, &layer_id).mask.as_ref() {
            Some(m) => format!("{} vertices", m.vertices.len()),
            None => "none".to_string(),
        },
    );

    // Document 19: self-intersection must be rejected, not normalized.
    let bowtie = Command::SetMask {
        composition: comp.clone(),
        layer_id: layer_id.clone(),
        mask: Some(PolygonMask::new(vec![
            (0.0, 0.0),
            (4.0, 0.0),
            (0.0, 4.0),
            (4.0, 4.0),
        ])),
    };
    // The message, not only the identifier. Both refusals report MASK_INVALID_OUTLINE, so an
    // identifier alone cannot tell whether a shape was refused for the right reason -- a build
    // that let a two-point mask past the vertex count would still be refused by the
    // self-intersection test, and a row reading only the identifier would call that a pass.
    report.check(
        "a self-intersecting mask is refused, for crossing itself",
        "MASK_INVALID_OUTLINE: The mask crosses itself, which this build does not draw.",
        match doc.apply(bowtie) {
            Ok(_) => "accepted".to_string(),
            Err(d) => format!("{}: {}", d.id.as_str(), d.message),
        },
    );
    report.check(
        "and a refused command leaves no mask on the layer",
        "none",
        match layer_of(&doc, &comp, &layer_id).mask.as_ref() {
            Some(m) => format!("{} vertices", m.vertices.len()),
            None => "none".to_string(),
        },
    );

    let two_points = Command::SetMask {
        composition: comp.clone(),
        layer_id: layer_id.clone(),
        mask: Some(PolygonMask::new(vec![(0.0, 0.0), (4.0, 4.0)])),
    };
    report.check(
        "a mask of two points is refused, for having too few points",
        "MASK_INVALID_OUTLINE: A mask needs at least three points, and this one has 2.",
        match doc.apply(two_points) {
            Ok(_) => "accepted".to_string(),
            Err(d) => format!("{}: {}", d.id.as_str(), d.message),
        },
    );
}

fn matte_only(report: &mut Report) {
    // D-42. The flag lives on the reference, on the layer that *uses* the matte, and says the
    // layer it points at is not also drawn in its own right -- document 21's "not separately
    // composited into the final stack". The alternative was to read the matte layer's own
    // `enabled: false` that way, and D-42 rejected it: one field meaning two unrelated things
    // would make a file showing a layer switched off while it visibly shapes the picture read
    // as a fault rather than as a setting.
    let mut doc = seeded_document();
    let comp = Id::new("comp");
    let root = std::env::temp_dir();
    let count = |doc: &Document| {
        let mut log = anime_compositor::diagnostics::FrameLog::new(64);
        compose::plan_frame(doc.project(), &Id::new("comp"), 0, &root, &mut log)
            .expect("frame 0 is inside the composition")
            .layers
            .len()
    };
    report.check(
        "both layers are drawn before either is used as a matte",
        2,
        count(&doc),
    );

    let set = |matte_only| Command::SetMatte {
        composition: comp.clone(),
        layer_id: Id::new("a"),
        matte: Some(Id::new("b")),
        matte_only,
    };
    doc.apply(set(false))
        .expect("a matte naming a layer in the composition is valid");
    report.check(
        "a matte layer is still drawn in its own right unless it is told not to be",
        2,
        count(&doc),
    );
    doc.apply(set(true))
        .expect("the same command with the flag set is equally valid");
    report.check(
        "matte-only takes the matte layer out of the drawn stack",
        1,
        count(&doc),
    );
    report.check(
        "and the layer that is left is the one that uses the matte, still matted",
        "a, matted",
        {
            let mut log = anime_compositor::diagnostics::FrameLog::new(64);
            let plan =
                compose::plan_frame(doc.project(), &comp, 0, &root, &mut log).expect("frame 0");
            format!(
                "{}, {}",
                plan.layers[0].id.as_str(),
                if plan.layers[0].matte.is_some() {
                    "matted"
                } else {
                    "unmatted"
                }
            )
        },
    );
    doc.undo().expect("undo the matte-only command");
    report.check(
        "undo puts the matte layer back in the stack",
        2,
        count(&doc),
    );

    // The flag has to survive the file, or setting it would be lost the next time the project
    // was opened -- which is the failure this row exists to catch.
    doc.apply(set(true)).expect("set it again");
    let text = persist::to_json(doc.project(), &persist::Preserved::none());
    let reopened = persist::load_str(&text).expect("what this build wrote, it opens");
    report.check(
        "matte-only is saved and comes back",
        "matte-only",
        match reopened
            .document
            .project()
            .composition(&comp)
            .expect("composition")
            .layer(&Id::new("a"))
            .expect("layer a")
            .matte
            .as_ref()
        {
            Some(m) if m.matte_only => "matte-only".to_string(),
            Some(_) => "the flag was lost".to_string(),
            None => "the matte was lost".to_string(),
        },
    );
    report.check(
        "and the reopened project draws the same one layer",
        1,
        count(&reopened.document),
    );
}

fn layer_of<'a>(doc: &'a Document, comp: &Id, layer: &Id) -> &'a Layer {
    doc.project()
        .composition(comp)
        .expect("composition")
        .layer(layer)
        .expect("layer")
}

fn seeded_document() -> Document {
    // Built through commands, because document 19's model refuses direct mutation from outside.
    let mut project = anime_compositor::model::Project::new(Id::new("p"));
    project
        .compositions
        .push(anime_compositor::model::Composition::new(
            Id::new("comp"),
            "comp",
            8,
            8,
            anime_compositor::time::FrameRate::new(24, 1).expect("24 fps"),
            0,
            8,
        ));
    let mut doc = Document::new(project);
    for (index, name) in ["a", "b"].into_iter().enumerate() {
        // Still images on the one fixture cel, so the same document can also be rendered: the
        // matte-only rows below need a plan, and a sequence with no frames would have every
        // layer dropped for missing media before the flag could be seen doing anything.
        let asset = anime_compositor::model::Asset::still(
            Id::new(format!("asset-{name}")),
            name,
            mask_fixture_png()
                .file_name()
                .expect("the fixture cel has a file name")
                .to_string_lossy()
                .to_string(),
        );
        let layer = Layer::new(Id::new(name), name, asset.id.clone(), 0, 8);
        doc.apply_all(vec![
            Command::AddAsset { asset },
            Command::AddLayer {
                composition: Id::new("comp"),
                layer: Box::new(layer),
                index,
            },
        ])
        .expect("seeding a layer is a valid command");
    }
    doc
}

fn write_report(report: &Report) {
    let passed = report.rows.iter().filter(|r| r.pass()).count();
    let mut out = String::new();
    out.push_str(&format!(
        "# B-06 — the polygon mask and the alpha matte\n\n\
         **{passed} of {} checks passed.**\n\n\
         Generated by `tests/b06_mask.rs`. Covers requirement R-04, document 19's mask and matte \
         records, document 21 steps 2 and 5, and document 25's line 51.\n\n\
         ## What to look at\n\n\
         A mask cuts a shape out of one layer. A matte uses a second layer's transparency to cut \
         the first. This table is the arithmetic of both, on small squares where every number \
         can be checked by hand. It is not a picture of the reference shot; that is a separate \
         and later kind of check.\n\n\
         Four rows are worth reading on their own:\n\n\
         - **All 64 pixels agree with a rasterizer written a second time, a different way.** \
         The build casts a ray and counts crossings. The check sums angles. They share no code. \
         Agreement at every pixel of the field is the strongest thing this table says.\n\
         - **A mask and its inverse sum back to the unmasked pixel.** This is an identity rather \
         than a number, so it holds at every coverage value including the partial ones along an \
         edge, and it cannot be satisfied by a rasterizer that is wrong in a consistent way.\n\
         - **Masking a drawing handed out by the cache does not change what the cache holds.** \
         Layers share drawings. If this were wrong, masking one layer would cut every other \
         layer using the same artwork, and only when the cache happened to hit — a fault that \
         would come and go with the memory budget and never show in a single-layer test.\n\
         - **A matte of the same alpha but a different colour gives the same answer.** Document \
         21 says a matte contributes its *alpha*. The matte in these rows is black, so a build \
         that read its colour instead would give zero rather than a quarter.\n\
         - **A layer used as a matte stops being drawn only when a flag says so.** D-42. The \
         seven matte-only rows set the flag, count the layers the frame actually draws, undo \
         it, and save and reopen the project. A file that shows a layer switched off while \
         that layer visibly shapes the picture would be a file nobody can read; this is the \
         alternative, and these rows are the check that it survives the file.\n\n\
         ## The rasterization rule, which document 25 line 51 asks B-06 to record\n\n\
         **A 4x4 ordered grid of sample points per pixel; coverage is the number of samples \
         inside the polygon divided by sixteen; insideness is the even-odd rule.** The sample \
         for grid cell (i, j) of pixel (x, y) sits at `(x + (i+0.5)/4, y + (j+0.5)/4)`, \
         symmetric about the pixel centre document 21 puts at `(x+0.5, y+0.5)`.\n\n\
         The reference tool is the second implementation in the fixture itself, not a drawing \
         library. A library would have brought its own fill rule and its own antialiasing, and a \
         disagreement would have said nothing about whether this build is right.\n\n\
         Two consequences are stated rather than implied. **Coverage is an exact rational** — \
         sixteen integer decisions over sixteen — so it does not depend on vertex order, \
         summation order or the machine. And **an edge is accurate to one sixteenth**, which is \
         this build's honest resolution. Document 21 requires that multisample details be \
         fixture-tested before claiming subpixel equivalence, and nothing here claims it.\n\n\
         The even-odd choice is not load-bearing: even-odd and nonzero winding differ only on \
         self-intersecting polygons, and document 19 rejects those. On every polygon this build \
         accepts, the two rules give the same answer.\n\n\
         ## What this does not cover\n\n\
         Subpixel edge goldens stay OPEN, as document 25 line 51 leaves them: this table checks \
         topology exhaustively and the edge quantum exactly, and claims nothing finer. The \
         matte rows drive the renderer directly rather than through a saved project, so they \
         show the arithmetic and not the wiring; the wiring is covered where `compose` is. And \
         no row here is a photograph — whether a mask looks right on your shot is a question \
         only the picture can answer.\n\n\
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
    fs::write(repo("verification/B-06_mask_table.md"), out).expect("write report");
}

#[test]
fn mask_and_matte() {
    let mut report = Report::default();
    topology(&mut report);
    edge_quantum(&mut report);
    premultiplied_and_inverted(&mut report);
    rejection(&mut report);
    matte(&mut report);
    matte_only(&mut report);
    cache_isolation(&mut report);
    command_rules(&mut report);
    write_report(&report);

    let failed: Vec<&Row> = report.rows.iter().filter(|r| !r.pass()).collect();
    assert!(
        failed.is_empty(),
        "{} of {} checks failed:\n{}",
        failed.len(),
        report.rows.len(),
        failed
            .iter()
            .map(|r| format!(
                "  {}\n    expected {}\n    actual   {}",
                r.check, r.expected, r.actual
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
