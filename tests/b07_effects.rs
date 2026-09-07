//! R-05 / B-07: the ordered layer effect stack — exposure, tint and Gaussian blur.
//!
//! Writes `verification/B-07_effects_table.md`.
//!
//! # Where the expected values come from
//!
//! Document 21, "Effects", states all three as arithmetic:
//!
//! ```text
//! Exposure: parameter is stops e; linear premultiplied RGB is multiplied by 2^e;
//!           alpha is unchanged.
//! Tint:     color is linear RGB, amount t in 0..1. Recover straight source RGB where
//!           alpha > 0, compute mix(source_rgb, tint_rgb, t), then premultiply by original
//!           alpha. Alpha is unchanged.
//! Blur:     sigma_px >= 0. Separable normalized Gaussian weights, kernel radius
//!           ceil(3*sigma_px). Sigma zero is identity. Samples outside the image are
//!           transparent black. Blur operates on premultiplied RGB and alpha together.
//!           Bounds expand by the kernel radius.
//! ```
//!
//! Four rows are document 25's FX-E-001, FX-E-002, FX-T-001 and FX-T-002, copied from the
//! catalogue. Every other expected value was worked out from the three definitions above and the
//! arithmetic is written into the comment beside it. Nothing here was captured from a run of the
//! code under test (ADR-009).
//!
//! # The blur, and what "independently generated" means here
//!
//! Document 25: "Gaussian blur uses a synthetic single-pixel impulse in a transparent image.
//! Expected weights are independently generated from the normalized sigma/kernel definition in
//! 21. Test symmetry, normalization, alpha behavior and expanded bounds."
//!
//! [`reference_weights`] below is a second statement of that definition, written from the
//! document rather than from `src/effects.rs`. A second statement of the same formula is worth
//! something but not much on its own, so the kernel is also pinned by three properties that do
//! not depend on either implementation and together determine it completely: the weights sum to
//! one, they are symmetric about the centre, and each weight's ratio to the centre weight is
//! `exp(-d^2 / 2*sigma^2)`. A kernel that satisfies all three is the kernel document 21 asks
//! for, whatever code produced it.
//!
//! The impulse then carries that into two dimensions: a single opaque pixel in a transparent
//! image, blurred, must come out as the outer product `w[i] * w[j]`. That is the whole claim of
//! the word "separable", and it is checked pixel by pixel rather than asserted.
//!
//! # The tiling question
//!
//! ADR-011 warns that a neighbourhood operation cannot be tiled without a declared margin, and
//! document 21 line 121 says the blur is the first operation to exercise margin handling. In
//! this build the answer turns out to be that nothing is tiled at effect time: the stack runs
//! once over the whole layer buffer while the frame plan is built, and the renderer cuts its
//! tiles from the composition afterwards, by which time the blur is already in the layer's
//! pixels. There is therefore no margin to get wrong — but that is a claim about the code, so
//! the last section renders the same blurred frame at six tile sizes and requires six identical
//! frames.
//!
//! # What is deliberately not covered here
//!
//! The unknown-effect record's survival through load and save is `verification/B-09_persistence_table.md`,
//! which already opens `Fixtures/projects/unknown_effect_project.json`, keeps its
//! `opaque_unknown_data` and warns. This file checks the other half of that requirement: that an
//! unknown effect is not *drawn*, and says so per frame.

use std::fs;
use std::path::PathBuf;

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{self, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{apply_stack, Bypassed, Effect, EffectInstance};
use anime_compositor::model::{Asset, Composition, Id, Layer, Project, Prop, Value};
use anime_compositor::persist;
use anime_compositor::render::render;
use anime_compositor::time::FrameRate;
use anime_compositor::WorkingBuffer;

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

/// A pixel to six decimal places, which is document 25's 1e-6 tolerance made visible.
fn q(p: [f32; 4]) -> String {
    format!(
        "({:.6}, {:.6}, {:.6}, {:.6})",
        p[0] as f64, p[1] as f64, p[2] as f64, p[3] as f64
    )
}

fn pixel(buf: &WorkingBuffer, x: usize, y: usize) -> [f32; 4] {
    let i = (y * buf.width() + x) * 4;
    let d = buf.data();
    [d[i], d[i + 1], d[i + 2], d[i + 3]]
}

/// A one-pixel buffer, which is all three of document 25's arithmetic fixtures need.
fn one(p: [f32; 4]) -> WorkingBuffer {
    let mut buf = WorkingBuffer::transparent(1, 1);
    buf.data_mut().copy_from_slice(&p);
    buf
}

/// A flat buffer of one colour.
fn solid(width: usize, height: usize, p: [f32; 4]) -> WorkingBuffer {
    let mut buf = WorkingBuffer::transparent(width, height);
    for px in buf.data_mut().chunks_exact_mut(4) {
        px.copy_from_slice(&p);
    }
    buf
}

/// Run a stack and collect what it bypassed, so a row can name both the pixels and the reasons.
fn run(source: &mut WorkingBuffer, stack: &[EffectInstance]) -> ((usize, usize), Vec<String>) {
    let mut bypassed = Vec::new();
    let offset = apply_stack(source, stack, |instance, why| {
        bypassed.push(format!(
            "{} {}",
            instance.type_id(),
            match why {
                Bypassed::NotImplemented => "not implemented",
                Bypassed::InvalidParameter => "invalid parameter",
            }
        ));
    });
    (offset, bypassed)
}

fn only(effect: Effect) -> Vec<EffectInstance> {
    vec![EffectInstance::new(Id::new("fx"), effect)]
}

// ---------------------------------------------------------------------------------------
// The independent kernel
// ---------------------------------------------------------------------------------------

/// Document 21's kernel, restated here from the document.
///
/// Deliberately not `anime_compositor::effects::gaussian_weights`, and deliberately a different
/// algebraic form of the same exponent — `-(d/sigma)^2 / 2` rather than `-(d*d)/(2*sigma*sigma)`
/// — so a transcription slip in one is not a transcription slip in both. The radius is computed
/// here too rather than borrowed, because `ceil(3*sigma)` is part of what is being checked.
fn reference_weights(sigma: f64) -> Vec<f64> {
    let radius = (3.0 * sigma).ceil() as usize;
    let raw: Vec<f64> = (0..=2 * radius)
        .map(|i| {
            let d = i as f64 - radius as f64;
            (-(d / sigma).powi(2) / 2.0).exp()
        })
        .collect();
    let sum: f64 = raw.iter().sum();
    raw.into_iter().map(|w| w / sum).collect()
}

// ---------------------------------------------------------------------------------------
// Document 25's arithmetic fixtures
// ---------------------------------------------------------------------------------------

fn catalogue_fixtures(report: &mut Report) {
    // FX-E-001: "exposure identity | e=0 | unchanged | 1e-7".
    // 2^0 = 1, and multiplying by one is the identity on every channel.
    let mut buf = one([0.5, 0.25, 0.125, 0.5]);
    run(&mut buf, &only(Effect::Exposure { stops: 0.0 }));
    report.check(
        "FX-E-001 exposure at zero stops leaves the pixel alone",
        q([0.5, 0.25, 0.125, 0.5]),
        q(pixel(&buf, 0, 0)),
    );

    // FX-E-002: "exposure +1 | RGB=.25, e=1 | RGB=.5; alpha unchanged | 1e-6".
    // The catalogue's alpha for this row is .5 (`Fixtures/fixture_manifest.json`). 0.25 * 2^1
    // = 0.5 on each of the three colour channels, and alpha stays at 0.5.
    let mut buf = one([0.25, 0.25, 0.25, 0.5]);
    run(&mut buf, &only(Effect::Exposure { stops: 1.0 }));
    report.check(
        "FX-E-002 exposure of one stop doubles RGB and leaves alpha",
        q([0.5, 0.5, 0.5, 0.5]),
        q(pixel(&buf, 0, 0)),
    );

    // The catalogue does not say whether its RGB is straight or premultiplied, and it does not
    // have to: multiplying by 2^e commutes with multiplying by alpha, so both readings give the
    // same answer. This row is that sentence checked rather than believed -- exposure applied to
    // a straight colour and then premultiplied is the same pixel as exposure applied after.
    let straight = [0.25f32, 0.10, 0.40];
    let alpha = 0.5f32;
    let mut first = one([straight[0], straight[1], straight[2], 1.0]);
    run(&mut first, &only(Effect::Exposure { stops: 1.0 }));
    let after_then_premul = {
        let p = pixel(&first, 0, 0);
        [p[0] * alpha, p[1] * alpha, p[2] * alpha, alpha]
    };
    let mut second = one([
        straight[0] * alpha,
        straight[1] * alpha,
        straight[2] * alpha,
        alpha,
    ]);
    run(&mut second, &only(Effect::Exposure { stops: 1.0 }));
    report.check(
        "exposure gives the same pixel whether the catalogue's RGB is read straight or premultiplied",
        q(after_then_premul),
        q(pixel(&second, 0, 0)),
    );

    // FX-T-001: "tint zero | amount=0 | unchanged | 1e-7".
    // mix(c, tint, 0) = c, so a tint of nothing must not move a channel even by a rounding step.
    let mut buf = one([0.5, 0.25, 0.125, 0.5]);
    run(
        &mut buf,
        &only(Effect::Tint {
            color: [0.2, 0.4, 0.6],
            amount: 0.0,
        }),
    );
    report.check(
        "FX-T-001 a tint of amount zero leaves the pixel alone",
        q([0.5, 0.25, 0.125, 0.5]),
        q(pixel(&buf, 0, 0)),
    );

    // FX-T-002: "tint full | amount=1, alpha=.5 | straight RGB=tint; premultiplied by .5".
    // Straight source (1, 0, 0) at alpha .5 is premultiplied (.5, 0, 0, .5). mix(c, tint, 1) is
    // the tint exactly, so straight RGB becomes (.2, .4, .6) and premultiplied (.1, .2, .3).
    let mut buf = one([0.5, 0.0, 0.0, 0.5]);
    run(
        &mut buf,
        &only(Effect::Tint {
            color: [0.2, 0.4, 0.6],
            amount: 1.0,
        }),
    );
    report.check(
        "FX-T-002 a tint of amount one replaces the straight colour and keeps alpha",
        q([0.1, 0.2, 0.3, 0.5]),
        q(pixel(&buf, 0, 0)),
    );

    // Half a tint, worked out on paper, because FX-T-001 and FX-T-002 are both endpoints and a
    // build that ignored `amount` entirely would pass one of them and fail neither obviously.
    // Straight (1, 0, 0) toward (0.2, 0.4, 0.6) at t=.5 is (0.6, 0.2, 0.3); times alpha .5 is
    // (0.3, 0.1, 0.15).
    let mut buf = one([0.5, 0.0, 0.0, 0.5]);
    run(
        &mut buf,
        &only(Effect::Tint {
            color: [0.2, 0.4, 0.6],
            amount: 0.5,
        }),
    );
    report.check(
        "a tint of amount one half lands exactly half way, in straight colour",
        q([0.3, 0.1, 0.15, 0.5]),
        q(pixel(&buf, 0, 0)),
    );

    // Document 09: "avoid ambiguities around fully transparent pixels". There is no straight
    // colour to recover at alpha zero, so a full tint must leave the pixel transparent black
    // rather than painting the tint colour into a pixel that is not there.
    let mut buf = one([0.0, 0.0, 0.0, 0.0]);
    run(
        &mut buf,
        &only(Effect::Tint {
            color: [0.2, 0.4, 0.6],
            amount: 1.0,
        }),
    );
    report.check(
        "a full tint does not paint colour into a fully transparent pixel",
        q([0.0, 0.0, 0.0, 0.0]),
        q(pixel(&buf, 0, 0)),
    );
}

// ---------------------------------------------------------------------------------------
// The kernel: radius, normalization, symmetry, shape
// ---------------------------------------------------------------------------------------

fn kernel(report: &mut Report) {
    // Document 21: "kernel radius ceil(3*sigma_px)". 3*1 = 3; 3*2 = 6; 3*0.1 = 0.3 -> 1;
    // and "sigma zero is identity", which is a radius of nothing.
    let radii: Vec<String> = [0.0, 0.1, 1.0, 2.0, 1.5]
        .iter()
        .map(|&s| format!("{s}->{}", anime_compositor::effects::kernel_radius(s)))
        .collect();
    report.check(
        "kernel radius is ceil(3*sigma), and sigma zero has none",
        "0->0, 0.1->1, 1->3, 2->6, 1.5->5",
        radii.join(", "),
    );

    for sigma in [0.5, 1.0, 2.5] {
        let w = anime_compositor::effects::gaussian_weights(sigma);
        let reference = reference_weights(sigma);

        report.check(
            &format!("sigma {sigma}: the kernel has 2*radius+1 weights"),
            reference.len(),
            w.len(),
        );

        // Normalization. Document 21 says "normalized"; a kernel that summed to less than one
        // would darken every flat region it touched, by exactly the amount it fell short.
        let sum: f64 = w.iter().map(|&v| v as f64).sum();
        report.check(
            &format!("sigma {sigma}: the weights sum to one"),
            "1.000000",
            format!("{sum:.6}"),
        );

        // Symmetry. A Gaussian is even, so weight k and weight (len-1-k) are the same number. An
        // asymmetric kernel would drift the picture sideways a little on every blur.
        let asymmetric = (0..w.len())
            .filter(|&k| format!("{:.7}", w[k]) != format!("{:.7}", w[w.len() - 1 - k]))
            .count();
        report.check(
            &format!("sigma {sigma}: the weights are symmetric about the centre"),
            0,
            asymmetric,
        );

        // Shape. Sum and symmetry alone would also admit a box kernel, so this is the row that
        // says it is a Gaussian: every weight's ratio to the centre weight is exp(-d^2/2s^2),
        // computed here from the document without reference to either kernel's normalization.
        let radius = (w.len() - 1) / 2;
        let centre = w[radius] as f64;
        let wrong_shape = (0..w.len())
            .filter(|&k| {
                let d = k as f64 - radius as f64;
                let want = (-(d * d) / (2.0 * sigma * sigma)).exp();
                format!("{:.6}", w[k] as f64 / centre) != format!("{want:.6}")
            })
            .count();
        report.check(
            &format!("sigma {sigma}: every weight's ratio to the centre is exp(-d^2/2*sigma^2)"),
            0,
            wrong_shape,
        );

        // And, having pinned it three ways, the weights themselves against the second statement
        // of the formula.
        let differs = (0..w.len())
            .filter(|&k| format!("{:.6}", w[k] as f64) != format!("{:.6}", reference[k]))
            .count();
        report.check(
            &format!("sigma {sigma}: the weights match an independently written kernel"),
            0,
            differs,
        );
    }
}

// ---------------------------------------------------------------------------------------
// Document 25's impulse
// ---------------------------------------------------------------------------------------

fn impulse(report: &mut Report) {
    // "A synthetic single-pixel impulse in a transparent image." One opaque white pixel at the
    // centre of an 11x11 field of nothing, sigma 1, so the kernel radius is 3 and the whole
    // kernel fits inside the expanded bounds with room to spare.
    let sigma = 1.0;
    let radius = 3usize;
    let (w, h) = (11usize, 11usize);
    let (cx, cy) = (5usize, 5usize);
    let mut buf = WorkingBuffer::transparent(w, h);
    let i = (cy * w + cx) * 4;
    buf.data_mut()[i..i + 4].copy_from_slice(&[1.0, 1.0, 1.0, 1.0]);

    let (offset, _) = run(&mut buf, &only(Effect::GaussianBlur { sigma_px: sigma }));

    // "Expanded bounds": document 21's "bounds expand by the kernel radius", on all four sides.
    report.check(
        "the blurred buffer grows by the kernel radius on every side",
        format!(
            "{}x{}, origin moved by (3, 3)",
            w + 2 * radius,
            h + 2 * radius
        ),
        format!(
            "{}x{}, origin moved by ({}, {})",
            buf.width(),
            buf.height(),
            offset.0,
            offset.1
        ),
    );

    // Separability, pixel by pixel: the impulse response must be the outer product of the
    // one-dimensional kernel with itself, from weights this file generated.
    let reference = reference_weights(sigma);
    let (ix, iy) = (cx + radius, cy + radius);
    let mut wrong = 0usize;
    let mut worst = 0.0f64;
    for y in 0..buf.height() {
        for x in 0..buf.width() {
            let dx = x as isize - ix as isize;
            let dy = y as isize - iy as isize;
            let want = if dx.unsigned_abs() <= radius && dy.unsigned_abs() <= radius {
                reference[(dx + radius as isize) as usize]
                    * reference[(dy + radius as isize) as usize]
            } else {
                0.0
            };
            let got = pixel(&buf, x, y);
            for c in got {
                let e = (c as f64 - want).abs();
                if e > worst {
                    worst = e;
                }
                if format!("{:.6}", c as f64) != format!("{want:.6}") {
                    wrong += 1;
                }
            }
        }
    }
    report.check(
        "every pixel of the impulse response is w[i]*w[j] from the independent kernel",
        // 11x11 grown by three on every side is 17x17, and four channels each is 1156.
        "0 of 1156 channels differ",
        format!(
            "{wrong} of {} channels differ",
            buf.width() * buf.height() * 4
        ),
    );
    report.check(
        "and the largest disagreement anywhere in the response",
        "at most 1e-6",
        if worst <= 1e-6 {
            "at most 1e-6".to_string()
        } else {
            format!("{worst:.9}")
        },
    );

    // "Normalization", seen in the picture rather than in the kernel: a blur must not create or
    // destroy light. The impulse carried one unit of alpha in and the expanded bounds are wide
    // enough to hold all of it, so the whole image must still sum to one.
    let total: f64 = buf.data().chunks_exact(4).map(|p| p[3] as f64).sum();
    report.check(
        "the blurred impulse still holds exactly the alpha it started with",
        "1.000000",
        format!("{total:.6}"),
    );

    // "Symmetry", in two dimensions: the response is unchanged by a horizontal or vertical flip
    // about the impulse. An off-by-one in the convolution's centring shows up here and nowhere
    // in the sums.
    let mut asymmetric = 0usize;
    for d in 1..=radius {
        for other in 0..buf.height() {
            if q(pixel(&buf, ix - d, other)) != q(pixel(&buf, ix + d, other)) {
                asymmetric += 1;
            }
        }
        for other in 0..buf.width() {
            if q(pixel(&buf, other, iy - d)) != q(pixel(&buf, other, iy + d)) {
                asymmetric += 1;
            }
        }
    }
    report.check(
        "the response is symmetric about the impulse on both axes",
        0,
        asymmetric,
    );

    // "Alpha behavior". Document 21 blurs "premultiplied RGB and alpha together to avoid
    // dark/bright fringe artifacts". A single half-transparent red impulse is the sharpest test
    // of that: every pixel of the response is one weight times the same premultiplied pixel, so
    // RGB divided by alpha must be the original straight red everywhere the response is nonzero.
    // If RGB and alpha were filtered with different kernels -- or if alpha were left alone --
    // the recovered colour would change across the response, which is what a fringe is.
    let mut fringed = WorkingBuffer::transparent(11, 11);
    let i = (5 * 11 + 5) * 4;
    fringed.data_mut()[i..i + 4].copy_from_slice(&[0.4, 0.0, 0.0, 0.5]);
    run(
        &mut fringed,
        &only(Effect::GaussianBlur { sigma_px: sigma }),
    );
    let mut hues = std::collections::BTreeSet::new();
    for p in fringed.data().chunks_exact(4) {
        if p[3] > 1e-6 {
            hues.insert(format!(
                "({:.5}, {:.5}, {:.5})",
                p[0] / p[3],
                p[1] / p[3],
                p[2] / p[3]
            ));
        }
    }
    report.check(
        "a blurred half-transparent red has the same straight colour everywhere: no fringe",
        "1 colour: (0.80000, 0.00000, 0.00000)",
        format!(
            "{} colour{}: {}",
            hues.len(),
            if hues.len() == 1 { "" } else { "s" },
            hues.iter().take(3).cloned().collect::<Vec<_>>().join(" ")
        ),
    );

    // Sigma zero is the identity document 21 asks for, and identity includes the bounds: a blur
    // of nothing must not quietly pad the buffer, because the caller shifts the transform by
    // whatever it is told and a spurious offset would move the layer.
    let mut untouched = solid(4, 4, [0.5, 0.25, 0.125, 0.5]);
    let (zero_offset, _) = run(
        &mut untouched,
        &only(Effect::GaussianBlur { sigma_px: 0.0 }),
    );
    report.check(
        "a sigma of zero changes neither the pixels nor the bounds",
        format!("4x4, (0, 0), {}", q([0.5, 0.25, 0.125, 0.5])),
        format!(
            "{}x{}, ({}, {}), {}",
            untouched.width(),
            untouched.height(),
            zero_offset.0,
            zero_offset.1,
            q(pixel(&untouched, 2, 2))
        ),
    );

    // An image with nothing in it stays that way. This is the row that catches a normalization
    // written as a division by the realised alpha rather than by the realised weight sum: that
    // arithmetic is 0/0 here, and a NaN in one pixel spreads through every later stage.
    let mut empty = WorkingBuffer::transparent(6, 6);
    run(&mut empty, &only(Effect::GaussianBlur { sigma_px: 2.0 }));
    let bad = empty
        .data()
        .iter()
        .filter(|v| !v.is_finite() || **v != 0.0)
        .count();
    report.check(
        "blurring an empty image leaves it empty, with no NaN anywhere",
        0,
        bad,
    );

    // The interior of a large flat region must survive a blur unchanged. This is normalization
    // again, but where a person would actually see it: a kernel summing to 0.999 would leave a
    // flat cel very slightly darker every time it was blurred.
    let mut flat = solid(41, 41, [0.5, 0.25, 0.125, 0.5]);
    run(&mut flat, &only(Effect::GaussianBlur { sigma_px: 2.0 }));
    report.check(
        "the middle of a large flat region is unchanged by a blur",
        q([0.5, 0.25, 0.125, 0.5]),
        q(pixel(&flat, 26, 26)),
    );

    // What happens at the edge of the cel, which every row above this one is blind to.
    //
    // Document 21: samples outside the source are transparent black, and the weights are not
    // renormalized for them, so an edge fades out. The other defensible reading - repeat the
    // edge pixel, which is what most image libraries do by default - keeps the edge at full
    // strength and invents light that was never drawn. Nothing above can tell the two apart:
    // the impulse rows read a field that is transparent right up to its border, so clamping to
    // the edge clamps to transparent black and gives the same answer.
    //
    // Two rows pin it. The first is conservation: a normalized kernel with nothing outside can
    // only move alpha around, never make more of it, so an 8x8 opaque cel still holds exactly
    // 64 units of alpha after the blur. Repeating the edge manufactures alpha and this sum goes
    // up. The second is the exact corner value. Destination pixel (0, 0) of the expanded buffer
    // sits radius pixels beyond the cel on both axes, so of the whole 2D kernel only the single
    // tap on the cel's own corner lands inside: the answer is w[0] squared, and it is w[0]
    // because the kernel is symmetric. Repeating the edge would put the full weight of both
    // passes on that corner and return 1.
    // Four decimal places on the sum rather than six: it is 64 single-precision additions of
    // products of single-precision weights, and the last bit of that is not a fact about the
    // blur. Repeating the edge does not miss by a rounding error, it misses by whole pixels.
    let mut filled = solid(8, 8, [1.0, 1.0, 1.0, 1.0]);
    run(&mut filled, &only(Effect::GaussianBlur { sigma_px: 1.0 }));
    let total: f64 = filled.data().chunks_exact(4).map(|px| px[3] as f64).sum();
    report.check(
        "blurring an opaque 8x8 cel neither gains nor loses alpha",
        "64.0000",
        format!("{total:.4}"),
    );
    let w0 = reference_weights(1.0)[0];
    report.check(
        "the far corner of the expanded buffer is one kernel tap squared, not a repeated edge \
         pixel",
        format!("{:.6}", w0 * w0),
        format!("{:.6}", pixel(&filled, 0, 0)[3]),
    );
}

// ---------------------------------------------------------------------------------------
// The stack: order, bypass, accumulation
// ---------------------------------------------------------------------------------------

fn stack(report: &mut Report) {
    // Order changes the picture, and a build that stored a stack but applied it in any
    // convenient order would pass every single-effect row above.
    //
    // Start from straight 0.25 at alpha 1, tint white at half:
    //   exposure first:  0.25*2 = 0.5,   then 0.5 + (1-0.5)*0.5     = 0.75
    //   tint first:      0.25 + (1-0.25)*0.5 = 0.625, then *2       = 1.25
    let exposure = EffectInstance::new(Id::new("e"), Effect::Exposure { stops: 1.0 });
    let tint = EffectInstance::new(
        Id::new("t"),
        Effect::Tint {
            color: [1.0, 1.0, 1.0],
            amount: 0.5,
        },
    );
    let mut a = one([0.25, 0.25, 0.25, 1.0]);
    run(&mut a, &[exposure.clone(), tint.clone()]);
    report.check(
        "exposure then tint",
        q([0.75, 0.75, 0.75, 1.0]),
        q(pixel(&a, 0, 0)),
    );
    let mut b = one([0.25, 0.25, 0.25, 1.0]);
    run(&mut b, &[tint.clone(), exposure.clone()]);
    report.check(
        "tint then exposure is a different pixel",
        q([1.25, 1.25, 1.25, 1.0]),
        q(pixel(&b, 0, 0)),
    );

    // Two blurs in one stack each expand the bounds, and the offsets add. Sigma 1 has radius 3,
    // sigma 2 has radius 6, so a 5x5 layer comes out 23x23 with its origin nine pixels in.
    let mut twice = solid(5, 5, [1.0, 1.0, 1.0, 1.0]);
    let (offset, _) = run(
        &mut twice,
        &[
            EffectInstance::new(Id::new("b1"), Effect::GaussianBlur { sigma_px: 1.0 }),
            EffectInstance::new(Id::new("b2"), Effect::GaussianBlur { sigma_px: 2.0 }),
        ],
    );
    report.check(
        "two blurs in a stack expand the bounds twice and the offsets add",
        "23x23, (9, 9)",
        format!(
            "{}x{}, ({}, {})",
            twice.width(),
            twice.height(),
            offset.0,
            offset.1
        ),
    );

    // A bypassed effect is a setting, not a fault: nothing is drawn and nothing is reported.
    let mut disabled = EffectInstance::new(Id::new("e"), Effect::Exposure { stops: 4.0 });
    disabled.enabled = false;
    let mut buf = one([0.25, 0.25, 0.25, 1.0]);
    let (_, reported) = run(&mut buf, &[disabled]);
    report.check(
        "a switched-off effect is not drawn and is not reported as a fault",
        format!("{}, nothing reported", q([0.25, 0.25, 0.25, 1.0])),
        format!(
            "{}, {}",
            q(pixel(&buf, 0, 0)),
            if reported.is_empty() {
                "nothing reported".to_string()
            } else {
                reported.join("; ")
            }
        ),
    );

    // An effect this build does not have is preserved, not drawn, and reported once. Document
    // 19: unknown effects "may not be silently discarded".
    let mut buf = one([0.25, 0.25, 0.25, 1.0]);
    let (_, reported) = run(
        &mut buf,
        &only(Effect::Unsupported {
            type_id: "vendor.future.effect".to_string(),
        }),
    );
    report.check(
        "an unsupported effect is bypassed and says so",
        format!(
            "{}, vendor.future.effect not implemented",
            q([0.25, 0.25, 0.25, 1.0])
        ),
        format!("{}, {}", q(pixel(&buf, 0, 0)), reported.join("; ")),
    );

    // A stored parameter outside document 21's range is bypassed with a different reason, and
    // the rest of the stack still runs. A build that abandoned the whole stack on one bad value
    // would lose the exposure below.
    let mut buf = one([0.25, 0.25, 0.25, 1.0]);
    let (_, reported) = run(
        &mut buf,
        &[
            EffectInstance::new(Id::new("b"), Effect::GaussianBlur { sigma_px: -1.0 }),
            exposure.clone(),
        ],
    );
    report.check(
        "an out-of-range parameter is bypassed by itself, and the rest of the stack still runs",
        format!(
            "{}, core.gaussian_blur invalid parameter",
            q([0.5, 0.5, 0.5, 1.0])
        ),
        format!("{}, {}", q(pixel(&buf, 0, 0)), reported.join("; ")),
    );

    // Document 21 line 89: every effect declares its input bounds expansion. Exposure and tint
    // read one pixel and write it, so they declare nothing; the blur declares its radius.
    report.check(
        "each effect declares its bounds expansion, and only the blur has one",
        "exposure 0, tint 0, blur(2) 6, unsupported 0",
        format!(
            "exposure {}, tint {}, blur(2) {}, unsupported {}",
            Effect::Exposure { stops: 3.0 }.bounds_expansion(),
            Effect::Tint {
                color: [1.0, 0.0, 0.0],
                amount: 1.0
            }
            .bounds_expansion(),
            Effect::GaussianBlur { sigma_px: 2.0 }.bounds_expansion(),
            Effect::Unsupported {
                type_id: "vendor.future.effect".to_string()
            }
            .bounds_expansion(),
        ),
    );
}

// ---------------------------------------------------------------------------------------
// Through the renderer
// ---------------------------------------------------------------------------------------

fn through_the_renderer(report: &mut Report) {
    let root = std::env::temp_dir();
    let comp = Id::new("comp");
    let layer_id = Id::new("a");

    let plan_of = |doc: &Document| {
        let mut log = FrameLog::new(64);
        let plan = compose::plan_frame(doc.project(), &comp, 0, &root, &mut log)
            .expect("frame 0 is inside the composition");
        (plan, log)
    };

    let plain = seeded_document();
    let (plain_plan, _) = plan_of(&plain);
    let plain_frame = render(&plain_plan, DEFAULT_TILE_SIZE);

    let mut blurred = seeded_document();
    blurred
        .apply(Command::AddEffect {
            composition: comp.clone(),
            layer_id: layer_id.clone(),
            effect: EffectInstance::new(Id::new("fx-blur"), Effect::GaussianBlur { sigma_px: 1.0 }),
            index: None,
        })
        .expect("a sigma of one is a valid blur");
    let (blur_plan, _) = plan_of(&blurred);
    let blur_frame = render(&blur_plan, DEFAULT_TILE_SIZE);

    // The layer's own buffer grew by the radius. If that growth reached the frame as a shifted
    // picture the whole composition would slide three pixels up and left, which is why the
    // renderer is handed a transform pre-shifted by the offset.
    report.check(
        "a blurred layer arrives at the renderer six pixels larger on each axis",
        format!("{}x{}", 4 + 6, 4 + 6),
        format!(
            "{}x{}",
            blur_plan.layers[0].source.width(),
            blur_plan.layers[0].source.height()
        ),
    );

    // The picture did not move. The alpha centroid of the frame is a number that changes if the
    // offset compensation is wrong by even one pixel, and does not change under a symmetric
    // blur: the layer sits well inside the composition, so no part of the glow is clipped.
    report.check(
        "the blur does not move the picture: the alpha centroid is where it was",
        centroid(&plain_frame),
        centroid(&blur_frame),
    );

    // ...and it is genuinely blurred, or the row above would pass on a build that ignored the
    // effect entirely.
    report.check(
        "and the frame really is blurred, not merely unmoved",
        "different",
        if plain_frame.data() == blur_frame.data() {
            "identical"
        } else {
            "different"
        },
    );

    // ADR-011 and document 21 line 119: a neighbourhood operation cannot be tiled without a
    // declared margin. The stack runs before the plan exists, so the tiles never see a blur in
    // progress -- and this is that claim checked, not assumed. Four tile sizes, including two
    // that do not divide the 32-pixel frame, must give four identical frames.
    let sizes = [1usize, 3, 7, 16, 32, 64];
    let differing = sizes
        .iter()
        .filter(|&&s| render(&blur_plan, s).data() != blur_frame.data())
        .count();
    report.check(
        "the blurred frame is the same at every tile size, including sizes that do not divide it",
        "0 of 6 tile sizes differ",
        format!("{differing} of {} tile sizes differ", sizes.len()),
    );

    // An unsupported effect on a real layer reaches the frame log, which is what puts the
    // incomplete-fidelity mark on an export of it (`verification/T-08_export_table.md`).
    let mut unknown = seeded_document();
    unknown
        .apply(Command::AddEffect {
            composition: comp.clone(),
            layer_id: layer_id.clone(),
            effect: EffectInstance::new(
                Id::new("fx-unknown"),
                Effect::Unsupported {
                    type_id: "vendor.future.effect".to_string(),
                },
            ),
            index: None,
        })
        .expect("an unknown effect can be held on a layer");
    let (_, log) = plan_of(&unknown);
    report.check(
        "a layer carrying an effect this build does not have says so once per frame",
        "1 EFFECT_UNSUPPORTED",
        format!(
            "{} {}",
            log.ids_at(0).len(),
            log.ids_at(0)
                .first()
                .map(|id| id.as_str().to_string())
                .unwrap_or_else(|| "nothing".to_string())
        ),
    );
}

/// The alpha-weighted centre of a frame, to two decimal places.
fn centroid(frame: &WorkingBuffer) -> String {
    let (mut sx, mut sy, mut sa) = (0.0f64, 0.0f64, 0.0f64);
    for y in 0..frame.height() {
        for x in 0..frame.width() {
            let a = pixel(frame, x, y)[3] as f64;
            sx += x as f64 * a;
            sy += y as f64 * a;
            sa += a;
        }
    }
    if sa == 0.0 {
        return "no alpha at all".to_string();
    }
    format!("({:.2}, {:.2})", sx / sa, sy / sa)
}

// ---------------------------------------------------------------------------------------
// Commands and the file
// ---------------------------------------------------------------------------------------

fn command_rules(report: &mut Report) {
    let mut doc = seeded_document();
    let comp = Id::new("comp");
    let layer_id = Id::new("a");

    let add = |instance: EffectInstance, index: Option<usize>| Command::AddEffect {
        composition: comp.clone(),
        layer_id: layer_id.clone(),
        effect: instance,
        index,
    };
    let outcome = |r: Result<(), anime_compositor::diagnostics::Diagnostic>| match r {
        Ok(()) => "accepted".to_string(),
        Err(d) => format!("{}: {}", d.id.as_str(), d.message),
    };
    let stack_of = |doc: &Document| {
        layer_of(doc, &comp, &layer_id)
            .effects
            .iter()
            .map(|e| e.instance_id.as_str().to_string())
            .collect::<Vec<_>>()
            .join(", ")
    };

    doc.apply(add(
        EffectInstance::new(Id::new("one"), Effect::Exposure { stops: 1.0 }),
        None,
    ))
    .expect("a first effect");
    doc.apply(add(
        EffectInstance::new(Id::new("two"), Effect::GaussianBlur { sigma_px: 1.0 }),
        None,
    ))
    .expect("a second effect");
    report.check("effects are added in order", "one, two", stack_of(&doc));

    // Order is a picture-changing property, so it must be settable, not merely appended to.
    doc.apply(add(
        EffectInstance::new(
            Id::new("first"),
            Effect::Tint {
                color: [0.0, 0.0, 0.0],
                amount: 0.5,
            },
        ),
        Some(0),
    ))
    .expect("an effect can be inserted at the front");
    report.check(
        "an effect can be put at a chosen place in the stack",
        "first, one, two",
        stack_of(&doc),
    );

    doc.undo().expect("undo the insert");
    report.check(
        "undo puts the stack back exactly as it was",
        "one, two",
        stack_of(&doc),
    );

    // Document 07 requires stable instance IDs. Two effects sharing one would make every later
    // edit ambiguous, so the second is refused rather than renamed.
    report.check(
        "a second effect with an instance ID already in use is refused",
        "COMMAND_INVALID_VALUE: This layer already has an effect called one.",
        outcome(
            doc.apply(add(
                EffectInstance::new(Id::new("one"), Effect::Exposure { stops: 2.0 }),
                None,
            ))
            .map(|_| ()),
        ),
    );

    // Document 21 gives a range for each parameter. Out of range is refused, not clamped: a
    // build that clamped would accept the number the person typed and draw a different one.
    report.check(
        "a negative sigma is refused rather than clamped",
        "EFFECT_PARAMETER_INVALID: A Gaussian blur needs a sigma of zero or more, and this is -1.",
        outcome(
            doc.apply(add(
                EffectInstance::new(Id::new("bad"), Effect::GaussianBlur { sigma_px: -1.0 }),
                None,
            ))
            .map(|_| ()),
        ),
    );
    report.check(
        "a tint amount above one is refused for the same reason",
        "EFFECT_PARAMETER_INVALID: A tint amount runs from 0 to 1, and this is 1.5.",
        outcome(
            doc.apply(add(
                EffectInstance::new(
                    Id::new("bad"),
                    Effect::Tint {
                        color: [1.0, 0.0, 0.0],
                        amount: 1.5,
                    },
                ),
                None,
            ))
            .map(|_| ()),
        ),
    );
    report.check(
        "and a refused command leaves the stack alone",
        "one, two",
        stack_of(&doc),
    );

    // Bypass is a stored setting, so it survives in the record rather than removing it.
    doc.apply(Command::SetEffectEnabled {
        composition: comp.clone(),
        layer_id: layer_id.clone(),
        instance_id: Id::new("two"),
        enabled: false,
    })
    .expect("an effect can be switched off");
    report.check(
        "switching an effect off keeps it in the stack",
        "one, two; two is off",
        format!(
            "{}; two is {}",
            stack_of(&doc),
            if layer_of(&doc, &comp, &layer_id).effects[1].enabled {
                "on"
            } else {
                "off"
            }
        ),
    );

    // Changing parameters is one command for the whole set: a tint has a colour and an amount,
    // and committing them one at a time would put half-changed states in the history.
    doc.apply(Command::SetEffectParameters {
        composition: comp.clone(),
        layer_id: layer_id.clone(),
        instance_id: Id::new("one"),
        effect: Effect::Exposure { stops: 3.0 },
    })
    .expect("new settings of the same kind");
    report.check(
        "an effect's settings can be changed in place",
        "3 stops",
        match &layer_of(&doc, &comp, &layer_id).effects[0].effect {
            Effect::Exposure { stops } => format!("{stops} stops"),
            other => other.type_id().to_string(),
        },
    );

    // But not into a different kind of effect: the instance ID would then point at something
    // else, and every reference to it would quietly mean something new.
    report.check(
        "settings for a different kind of effect are refused",
        "COMMAND_INVALID_VALUE: Effect one is a core.exposure, and these are settings for a core.tint.",
        outcome(
            doc.apply(Command::SetEffectParameters {
                composition: comp.clone(),
                layer_id: layer_id.clone(),
                instance_id: Id::new("one"),
                effect: Effect::Tint {
                    color: [1.0, 0.0, 0.0],
                    amount: 0.5,
                },
            })
            .map(|_| ()),
        ),
    );

    report.check(
        "a command naming an effect that is not there is refused",
        "COMMAND_TARGET_MISSING: There is no effect called nope on this layer.",
        outcome(
            doc.apply(Command::RemoveEffect {
                composition: comp.clone(),
                layer_id: layer_id.clone(),
                instance_id: Id::new("nope"),
            })
            .map(|_| ()),
        ),
    );

    doc.apply(Command::RemoveEffect {
        composition: comp.clone(),
        layer_id: layer_id.clone(),
        instance_id: Id::new("one"),
    })
    .expect("removing an effect that is there");
    report.check("an effect can be removed", "two", stack_of(&doc));
    doc.undo().expect("undo the removal");
    report.check(
        "and undo puts it back where it was, not on the end",
        "one, two",
        stack_of(&doc),
    );

    // The whole stack has to survive the file, or none of the above would last past a save.
    let text = persist::to_json(doc.project(), &persist::Preserved::none());
    let reopened = persist::load_str(&text).expect("what this build wrote, it opens");
    let back = reopened
        .document
        .project()
        .composition(&comp)
        .expect("composition")
        .layer(&layer_id)
        .expect("layer a")
        .effects
        .clone();
    report.check(
        "the stack is saved and comes back in order, with its settings and its bypass flag",
        "one core.exposure 3 stops on, two core.gaussian_blur sigma 1 off",
        back.iter()
            .map(|e| {
                let params = match &e.effect {
                    Effect::Exposure { stops } => format!("{stops} stops"),
                    Effect::GaussianBlur { sigma_px } => format!("sigma {sigma_px}"),
                    Effect::Tint { amount, .. } => format!("amount {amount}"),
                    Effect::Unsupported { .. } => "unknown".to_string(),
                };
                format!(
                    "{} {} {} {}",
                    e.instance_id.as_str(),
                    e.type_id(),
                    params,
                    if e.enabled { "on" } else { "off" }
                )
            })
            .collect::<Vec<_>>()
            .join(", "),
    );
}

// ---------------------------------------------------------------------------------------
// Scaffolding
// ---------------------------------------------------------------------------------------

fn layer_of<'a>(doc: &'a Document, comp: &Id, layer: &Id) -> &'a Layer {
    doc.project()
        .composition(comp)
        .expect("composition")
        .layer(layer)
        .expect("layer")
}

/// A 4x4 cel, written once, so a plan has something real to decode.
fn cel_png() -> PathBuf {
    let path = std::env::temp_dir().join("anime_compositor_b07_effects_cel.png");
    if !path.exists() {
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

/// One 4x4 layer at (14, 14) in a 32x32 composition.
///
/// Placed well inside the frame on purpose: the centroid row below requires that none of a
/// sigma-1 glow is clipped by the edge of the composition, and three pixels of kernel plus four
/// of layer leave eleven to spare on every side.
fn seeded_document() -> Document {
    let mut project = Project::new(Id::new("p"));
    project.compositions.push(Composition::new(
        Id::new("comp"),
        "comp",
        32,
        32,
        FrameRate::new(24, 1).expect("24 fps"),
        0,
        8,
    ));
    let mut doc = Document::new(project);
    let asset = Asset::still(
        Id::new("asset-a"),
        "a",
        cel_png()
            .file_name()
            .expect("the fixture cel has a file name")
            .to_string_lossy()
            .to_string(),
    );
    let layer = Layer::new(Id::new("a"), "a", asset.id.clone(), 0, 8);
    doc.apply_all(vec![
        Command::AddAsset { asset },
        Command::AddLayer {
            composition: Id::new("comp"),
            layer: Box::new(layer),
            index: 0,
        },
        Command::SetPropertyBase {
            composition: Id::new("comp"),
            layer_id: Id::new("a"),
            prop: Prop::Position,
            value: Value::Vec2(14.0, 14.0),
        },
    ])
    .expect("seeding a layer is a valid command");
    doc
}

// ---------------------------------------------------------------------------------------
// The test
// ---------------------------------------------------------------------------------------

#[test]
fn b07_effects() {
    let mut report = Report::default();
    catalogue_fixtures(&mut report);
    kernel(&mut report);
    impulse(&mut report);
    stack(&mut report);
    through_the_renderer(&mut report);
    command_rules(&mut report);

    write_report(&report);
    let failed = report.rows.iter().filter(|r| !r.pass()).count();
    assert_eq!(
        failed,
        0,
        "B-07 effects: {failed} of {} checks failed\n{}",
        report.rows.len(),
        report
            .rows
            .iter()
            .filter(|r| !r.pass())
            .map(|r| format!(
                "  {}\n    expected {}\n    actual   {}",
                r.check, r.expected, r.actual
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn write_report(report: &Report) {
    let passed = report.rows.iter().filter(|r| r.pass()).count();
    let mut out = String::new();
    out.push_str(&format!(
        "# B-07 - the layer effect stack\n\n\
         **{passed} of {} checks passed.**\n\n\
         Generated by `tests/b07_effects.rs`. Covers requirement R-05, document 21's \"Effects\" \
         section, and document 25's fixtures FX-E-001, FX-E-002, FX-T-001, FX-T-002 and the \
         single-pixel impulse.\n\n\
         ## What to look at\n\n\
         An effect changes a layer's own pixels before the layer is moved, faded or composited. \
         Three of them are in this build: exposure makes a layer brighter or darker by stops, \
         tint pulls its colour toward another colour, and Gaussian blur softens it. Every number \
         below comes from the arithmetic document 21 states, worked out before the code was run \
         and written into the test beside each row.\n\n\
         Six rows are worth reading on their own:\n\n\
         - **The blur is checked against a kernel written twice.** The weights this build uses \
         are compared against weights generated separately in the test from document 21's \
         definition, and then pinned three further ways that do not depend on either piece of \
         code: they sum to one, they are symmetric, and each one's ratio to the middle weight is \
         the Gaussian curve. A box blur would pass the first two and fail the third.\n\
         - **A single bright pixel, blurred, is checked pixel by pixel.** Document 25 asks for \
         exactly this test. All 1156 numbers in the result have to match the independent kernel's \
         outer product, which is what the word \"separable\" claims and which nothing short of \
         checking every pixel would establish.\n\
         - **Order matters, and the table shows it.** The same two effects in the two possible \
         orders give two different pixels. A build that stored a stack and applied it in whatever \
         order was convenient would pass every other row in this file.\n\
         - **A blur makes the layer bigger, and the picture must not move.** Document 21 requires \
         the layer's bounds to expand by the kernel radius so a glow near the edge of a cel is \
         not cut off in a straight line. The row that matters is the one after: the centre of the \
         frame's alpha is in exactly the same place before and after, so the extra margin did not \
         shift anything.\n\
         - **The edge of a cel fades rather than smearing.** When a blur reaches past the edge of the drawing, what it finds there is nothing, not another copy of the edge pixel. Most image libraries do the opposite by default, and the difference is a hard bright rim around every blurred cel. Two rows say so: an opaque square still holds exactly as much of itself after the blur as before, which repeating the edge would inflate, and the far corner of the enlarged layer is the one faint tap the arithmetic allows rather than a solid pixel.\n\
         - **The blurred frame is identical at six tile sizes.** The renderer divides a frame \
         into tiles and works on them in parallel. A blur reads its neighbours, so a blur done \
         inside a tile would show seams at the tile boundaries. In this build the effects run \
         over the whole layer before the tiles are cut, and that row is six renders of the \
         same frame at six tile sizes, all required to come out identical.\n\n\
         ## What this does not cover\n\n\
         There is no user interface for effects yet: the commands exist and are checked here, \
         but nothing puts them on screen. An unknown effect's survival through save and reopen \
         is checked in `verification/B-09_persistence_table.md` against \
         `Fixtures/projects/unknown_effect_project.json`; what this file adds is that such an \
         effect is not drawn and says so, once per frame, which is what marks an export of it \
         incomplete.\n\n\
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
    fs::write(repo("verification/B-07_effects_table.md"), out).expect("write report");
}
