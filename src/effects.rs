//! Step 3 of document 21: the ordered effect stack, evaluated in layer space.
//!
//! Three effects are in G1, and document 21 states each one as arithmetic rather than as a
//! description: exposure multiplies premultiplied RGB by `2^e` and leaves alpha alone; tint
//! recovers straight RGB, mixes toward a colour, and premultiplies back; Gaussian blur filters
//! premultiplied RGB and alpha together with separable normalised weights and a radius of
//! `ceil(3*sigma)`. Each is written here in that form, and `tests/b07_effects.rs` checks it
//! against weights generated a second way.
//!
//! Effects run after the mask (step 2) and before the transform (step 4), on the layer's own
//! pixels. That ordering is what makes the blur radius a number in source pixels: a layer scaled
//! to half size blurs by the same source-pixel sigma and the result is scaled with everything
//! else, which is what document 09 means by "specify radius units in source pixels".
//!
//! **Bounds.** Document 21 line 89 requires every effect to declare its input bounds expansion.
//! Exposure and tint declare zero -- they read one pixel and write it. Blur declares its kernel
//! radius, and this module honours that literally: the buffer grows by the radius on all four
//! sides, so light that leaves the edge of the cel is kept rather than cropped. The caller is
//! handed the offset and shifts the layer transform by it, which leaves every unblurred pixel
//! exactly where it was.
//!
//! **Tiling.** ADR-011 warns that a neighbourhood operation cannot be tiled without a declared
//! margin. Nothing here is tiled: the stack runs once over the whole layer buffer while the
//! frame plan is being built, and the renderer's tiles are cut from the composition afterwards,
//! by which time the blur is already baked into the layer's pixels. So there is no margin to
//! get wrong, and no ROI optimisation to prove equal to full-frame math. What there is to prove
//! is that the frame still does not depend on tile size once a blur is in it, and the fixture
//! renders the same blurred frame at six tile sizes to say so.

use rayon::prelude::*;

use crate::model::Id;
use crate::WorkingBuffer;

/// One entry in a layer's ordered effect stack.
///
/// Document 19: "EffectInstance stores stable instance ID, effect type ID, enabled flag and a
/// typed parameter map."
#[derive(Clone, PartialEq, Debug)]
pub struct EffectInstance {
    pub instance_id: Id,
    pub enabled: bool,
    pub effect: Effect,
}

impl EffectInstance {
    pub fn new(instance_id: Id, effect: Effect) -> Self {
        EffectInstance {
            instance_id,
            enabled: true,
            effect,
        }
    }

    pub fn type_id(&self) -> &str {
        self.effect.type_id()
    }
}

/// The typed parameter map of document 19, made a type per effect rather than a map.
///
/// A map would have to be validated at every read; document 19 requires that "effect parameter
/// types match the registered effect schema", and the cheapest way to guarantee that is to make
/// the wrong shape unrepresentable once the record has been read. Validation therefore happens
/// exactly once, where the file is parsed.
///
/// `Unsupported` is the exception and is the point of the enum being open at all. Document 19:
/// "Unknown effect records must survive project load/save where feasible but render as
/// unsupported with an explicit warning; they may not be silently discarded."
#[derive(Clone, PartialEq, Debug)]
pub enum Effect {
    /// Document 21: "parameter is stops `e`; linear premultiplied RGB is multiplied by `2^e`;
    /// alpha is unchanged."
    Exposure { stops: f64 },
    /// Document 21: "parameter `sigma_px >= 0` ... kernel radius `ceil(3*sigma_px)`."
    GaussianBlur { sigma_px: f64 },
    /// Document 21: "parameter color is linear RGB and amount `t` in 0..1."
    Tint { color: [f64; 3], amount: f64 },
    /// An effect this build does not have. Preserved, never drawn, always reported.
    Unsupported { type_id: String },
}

/// The identifiers used in the project file. Namespaced the way the fixture's
/// `vendor.future.effect` is, so a built-in and a third-party effect can never collide.
pub const EXPOSURE: &str = "core.exposure";
pub const GAUSSIAN_BLUR: &str = "core.gaussian_blur";
pub const TINT: &str = "core.tint";

impl Effect {
    pub fn type_id(&self) -> &str {
        match self {
            Effect::Exposure { .. } => EXPOSURE,
            Effect::GaussianBlur { .. } => GAUSSIAN_BLUR,
            Effect::Tint { .. } => TINT,
            Effect::Unsupported { type_id } => type_id,
        }
    }

    /// Document 21 line 89: "Every effect declares input bounds expansion." In pixels, on every
    /// side.
    ///
    /// An unsupported effect declares zero because it is not run. That is not a claim about what
    /// the effect would expand by if this build had it -- it is the bounds of doing nothing,
    /// which is what actually happens, and the export says fidelity is incomplete so that the
    /// difference is never mistaken for a rendered result.
    pub fn bounds_expansion(&self) -> usize {
        match self {
            Effect::GaussianBlur { sigma_px } => kernel_radius(*sigma_px),
            _ => 0,
        }
    }

    /// Whether the parameters are inside the ranges document 21 states.
    ///
    /// A negative sigma has no kernel and a tint amount outside 0..1 is an extrapolation the
    /// specification does not define, so both are refused at the command boundary rather than
    /// clamped. Clamping would accept a number and silently render a different one.
    pub fn is_valid(&self) -> bool {
        match self {
            Effect::Exposure { stops } => stops.is_finite(),
            Effect::GaussianBlur { sigma_px } => sigma_px.is_finite() && *sigma_px >= 0.0,
            Effect::Tint { color, amount } => {
                color.iter().all(|c| c.is_finite())
                    && amount.is_finite()
                    && (0.0..=1.0).contains(amount)
            }
            Effect::Unsupported { .. } => true,
        }
    }

    /// Why [`is_valid`](Self::is_valid) said no, as a sentence for a person.
    pub fn why_invalid(&self) -> String {
        match self {
            Effect::Exposure { stops } => {
                format!("Exposure needs a finite number of stops, and this is {stops}.")
            }
            Effect::GaussianBlur { sigma_px } => {
                format!("A Gaussian blur needs a sigma of zero or more, and this is {sigma_px}.")
            }
            Effect::Tint { amount, .. } => {
                format!("A tint amount runs from 0 to 1, and this is {amount}.")
            }
            Effect::Unsupported { type_id } => {
                format!("{type_id} has no parameters this build checks.")
            }
        }
    }
}

/// Document 21: "kernel radius `ceil(3*sigma_px)`". Sigma zero gives radius zero, which is the
/// identity the same document asks for.
pub fn kernel_radius(sigma_px: f64) -> usize {
    if sigma_px.is_nan() || sigma_px <= 0.0 {
        return 0;
    }
    (3.0 * sigma_px).ceil() as usize
}

/// The separable normalised weights of document 21, index 0 being the sample `radius` pixels
/// before the centre.
///
/// Normalised after truncation, not before: the untruncated Gaussian integrates to one over the
/// whole line and this kernel is cut at three sigma, so weights taken straight from the
/// exponential would sum slightly under one and darken the image by that much. Dividing by the
/// realised sum is what makes a flat region survive the blur unchanged, which is the property
/// the fixture checks first.
pub fn gaussian_weights(sigma_px: f64) -> Vec<f32> {
    let radius = kernel_radius(sigma_px);
    if radius == 0 {
        return vec![1.0];
    }
    let mut w: Vec<f64> = (0..=2 * radius)
        .map(|i| {
            let d = i as f64 - radius as f64;
            (-(d * d) / (2.0 * sigma_px * sigma_px)).exp()
        })
        .collect();
    let sum: f64 = w.iter().sum();
    for v in &mut w {
        *v /= sum;
    }
    w.into_iter().map(|v| v as f32).collect()
}

/// Run one layer's stack over its pixels, in order.
///
/// Returns how far the buffer's origin moved, in pixels: `(0, 0)` unless a blur expanded it.
/// The caller must shift the layer transform by that offset, or every pixel moves.
///
/// `report` is called once for each instance that was skipped and why, which is how a bypassed
/// effect reaches the frame log and, through it, the incomplete-fidelity mark on an export.
pub fn apply_stack(
    source: &mut WorkingBuffer,
    stack: &[EffectInstance],
    mut report: impl FnMut(usize, &EffectInstance, Bypassed),
) -> (usize, usize) {
    let (mut ox, mut oy) = (0usize, 0usize);
    // The position is reported alongside the instance because P-11's effect cache replays a
    // bypass on a hit, and a position is the one thing about an instance that survives being
    // written down and read back next frame.
    for (at, instance) in stack.iter().enumerate() {
        if !instance.enabled {
            // Deliberately silent. A bypassed effect is a setting a person chose, not a fault,
            // and document 28's incomplete-fidelity mark is for what this build could not do.
            continue;
        }
        match &instance.effect {
            Effect::Unsupported { .. } => {
                report(at, instance, Bypassed::NotImplemented);
                continue;
            }
            e if !e.is_valid() => {
                report(at, instance, Bypassed::InvalidParameter);
                continue;
            }
            // One stage per kind of effect rather than one for the stack (P-11). The three are
            // disjoint and none of them nests, so `src/perf.rs`'s promise that the table can be
            // summed still holds; what they buy is the ranking P-11's entry says it needs before
            // it decides which effect is worth caching.
            Effect::Exposure { stops } => {
                crate::perf::time(crate::perf::Stage::EffectExposure, || {
                    exposure(source, *stops)
                })
            }
            Effect::Tint { color, amount } => {
                crate::perf::time(crate::perf::Stage::EffectTint, || {
                    tint(source, *color, *amount)
                })
            }
            Effect::GaussianBlur { sigma_px } => {
                let r =
                    crate::perf::time(crate::perf::Stage::EffectBlur, || blur(source, *sigma_px));
                ox += r;
                oy += r;
            }
        }
    }
    (ox, oy)
}

/// Why an effect in the stack did not run.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bypassed {
    /// This build does not have the effect. Document 28's `EFFECT_UNSUPPORTED`.
    NotImplemented,
    /// The build has the effect, but the stored parameters are outside what document 21 defines.
    /// Document 28's `EFFECT_PARAMETER_INVALID`.
    InvalidParameter,
}

/// Document 21: "linear premultiplied RGB is multiplied by `2^e`; alpha is unchanged."
///
/// Three channels, not four, and that is the whole difference between this and the mask's
/// coverage multiply. Exposure changes how much light a pixel carries and not how much of the
/// pixel there is, so the premultiplied product moves and the coverage does not.
fn exposure(source: &mut WorkingBuffer, stops: f64) {
    let gain = (2.0f64).powf(stops) as f32;
    if gain == 1.0 {
        return;
    }
    for px in source.data_mut().chunks_exact_mut(4) {
        for c in &mut px[..3] {
            *c *= gain;
        }
    }
}

/// Document 21: "Recover straight source RGB where alpha > 0, compute `mix(source_rgb,
/// tint_rgb, t)`, then premultiply by original alpha. Alpha is unchanged."
///
/// Written in exactly that order. Mixing the premultiplied values directly would be a different
/// operation: a half-transparent red would mix toward half the tint colour rather than toward
/// the tint colour, so the same paint would read as a different hue wherever the artwork is
/// soft. Where alpha is zero there is no straight colour to recover, and zero times anything is
/// zero, so those pixels are left alone -- which is document 09's "avoid ambiguities around
/// fully transparent pixels".
fn tint(source: &mut WorkingBuffer, color: [f64; 3], amount: f64) {
    if amount == 0.0 {
        return;
    }
    let t = amount as f32;
    let tint = [color[0] as f32, color[1] as f32, color[2] as f32];
    for px in source.data_mut().chunks_exact_mut(4) {
        let a = px[3];
        if a <= 0.0 {
            continue;
        }
        for i in 0..3 {
            let straight = px[i] / a;
            px[i] = (straight + (tint[i] - straight) * t) * a;
        }
    }
}

/// Document 21's Gaussian blur, separable, on premultiplied RGB and alpha together.
///
/// Returns the radius the buffer grew by on each side. The growth is document 21's "Bounds
/// expand by the kernel radius", and it is not a nicety: without it a character blurred near the
/// edge of its own cel would have the glow cut off in a straight line, which is a visible fault
/// that no amount of correct arithmetic inside the old extent would fix.
///
/// Two one-dimensional passes rather than one two-dimensional kernel, which is what "separable"
/// means and is why a sigma of 10 costs 61 multiplies per pixel per axis instead of 3,721.
/// Samples outside the source are transparent black, exactly as the bilinear sampler treats
/// them, so a pixel near the edge is a weighted sum in which the missing neighbours contribute
/// nothing -- and because the weights are not renormalised for them, an edge fades out rather
/// than staying artificially bright.
fn blur(source: &mut WorkingBuffer, sigma_px: f64) -> usize {
    let radius = kernel_radius(sigma_px);
    if radius == 0 {
        return 0;
    }
    let weights = gaussian_weights(sigma_px);
    let (w, h) = (source.width(), source.height());

    // Horizontal, into a buffer wider by the radius on each side.
    let wide_w = w + 2 * radius;
    let mut wide = WorkingBuffer::transparent(wide_w, h);
    convolve(
        source.data(),
        w,
        wide.data_mut(),
        wide_w,
        radius,
        &weights,
        Axis::X,
    );

    // Vertical, into a buffer taller by the radius on each side.
    let tall_h = h + 2 * radius;
    let mut tall = WorkingBuffer::transparent(wide_w, tall_h);
    convolve(
        wide.data(),
        wide_w,
        tall.data_mut(),
        wide_w,
        radius,
        &weights,
        Axis::Y,
    );

    *source = tall;
    radius
}

enum Axis {
    X,
    Y,
}

/// One separable pass. `dst` is `src` grown by `radius` on both ends of `axis`.
///
/// **Across the thread pool, one destination row at a time (P-13).** A row of the destination
/// reads the source and writes nothing but itself, so the rows are independent whichever axis is
/// being filtered, and `par_chunks_mut` hands each one out without any of them being able to see
/// another. Nothing about the arithmetic moves: a destination pixel is still the same taps
/// accumulated in the same order into the same `acc`, so the result is the same bits on one
/// thread or on sixteen, which is what `verification/P-13_parallel_blur.md` compares rather than
/// asserts. `verification/P-01_frame_trace.md` is why the blur is the loop that got this: on the
/// declared ten-layer fixture with everything warm, the effect stack is 65.2% of a draft frame
/// and the tile loop beside it, already spread across this same pool, is 2.1%.
fn convolve(
    src: &[f32],
    src_w: usize,
    dst: &mut [f32],
    dst_w: usize,
    radius: usize,
    weights: &[f32],
    axis: Axis,
) {
    let src_h = src.len() / (src_w * 4);
    dst.par_chunks_mut(dst_w * 4)
        .enumerate()
        .for_each(|(y, out)| {
            for x in 0..dst_w {
                let mut acc = [0.0f32; 4];
                for (k, &weight) in weights.iter().enumerate() {
                    // `k - radius` is the offset from the centre; the centre of destination pixel
                    // (x, y) sits at source pixel (x - radius) or (y - radius) on the blurred axis.
                    let offset = k as isize - radius as isize;
                    let (sx, sy) = match axis {
                        Axis::X => (x as isize - radius as isize + offset, y as isize),
                        Axis::Y => (x as isize, y as isize - radius as isize + offset),
                    };
                    if sx < 0 || sy < 0 || sx >= src_w as isize || sy >= src_h as isize {
                        continue;
                    }
                    let i = (sy as usize * src_w + sx as usize) * 4;
                    for c in 0..4 {
                        acc[c] += src[i + c] * weight;
                    }
                }
                let o = x * 4;
                out[o..o + 4].copy_from_slice(&acc);
            }
        });
}
