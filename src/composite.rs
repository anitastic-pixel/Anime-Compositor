//! Normal-over compositing and the straight/premultiplied conversions it depends on.
//!
//! Document 21, "Normal composite", is the authority for this file:
//!
//! ```text
//! Co = Cs + Cd*(1-As)
//! Ao = As + Ad*(1-As)
//! ```
//!
//! Both operands are premultiplied linear-light RGBA. Nothing here clamps: document 21 says
//! intermediate math may exceed 0..1 and that clamping happens only at the encoding step.
//!
//! The same document's "Blend modes" section adds multiply, screen and add, which do not
//! reduce to the equation above and get their own function.

use crate::model::BlendMode;
use crate::WorkingBuffer;

/// Premultiplied normal-over for one pixel. Source over destination.
pub fn over_pixel(src: [f32; 4], dst: [f32; 4]) -> [f32; 4] {
    let inv = 1.0 - src[3];
    [
        src[0] + dst[0] * inv,
        src[1] + dst[1] * inv,
        src[2] + dst[2] * inv,
        src[3] + dst[3] * inv,
    ]
}

/// Multiply straight RGB by alpha. Alpha is unchanged.
pub fn premultiply(straight: [f32; 4]) -> [f32; 4] {
    let a = straight[3];
    [straight[0] * a, straight[1] * a, straight[2] * a, a]
}

/// Recover straight RGB from a premultiplied pixel.
///
/// Document 21 line 9: "Whenever straight color must be recovered from a premultiplied pixel
/// with alpha zero, define straight RGB as zero to avoid NaN/Inf propagation."
pub fn unpremultiply(premul: [f32; 4]) -> [f32; 4] {
    let a = premul[3];
    if a == 0.0 {
        [0.0, 0.0, 0.0, a]
    } else {
        [premul[0] / a, premul[1] / a, premul[2] / a, a]
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum CompositeError {
    ExtentMismatch {
        src: (usize, usize),
        dst: (usize, usize),
    },
}

impl std::fmt::Display for CompositeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompositeError::ExtentMismatch { src, dst } => write!(
                f,
                "source extent {}x{} does not match destination {}x{}",
                src.0, src.1, dst.0, dst.1
            ),
        }
    }
}

/// Composite `src` over `dst` in place.
///
/// Per-pixel and order-independent, so this is tile-safe without qualification
/// (document 21, "Spatial support and margins").
pub fn over(src: &WorkingBuffer, dst: &mut WorkingBuffer) -> Result<(), CompositeError> {
    if src.width() != dst.width() || src.height() != dst.height() {
        return Err(CompositeError::ExtentMismatch {
            src: (src.width(), src.height()),
            dst: (dst.width(), dst.height()),
        });
    }
    for (s, d) in src
        .data()
        .chunks_exact(4)
        .zip(dst.data_mut().chunks_exact_mut(4))
    {
        let out = over_pixel([s[0], s[1], s[2], s[3]], [d[0], d[1], d[2], d[3]]);
        d.copy_from_slice(&out);
    }
    Ok(())
}

/// D-828: [`blend_pixel_in`] for a layer with After Effects' Preserve Underlying Transparency (the
/// T switch): the layer shows only where the layers beneath have alpha, and their alpha is kept.
/// The general equation without its `(1-Ad)*Cs` term, Porter and Duff's "atop":
///
/// ```text
/// Co = (1-As)*Cd + As*Ad*B(cs,cd)      (Normal: Cs*Ad + Cd*(1-As))
/// Ao = Ad
/// ```
///
/// A clear source changes nothing, so a tile the layer does not reach is still skipped. A stencil
/// draws nothing of its own, so the switch changes nothing on one.
pub fn blend_pixel_kept(
    mode: BlendMode,
    src: [f32; 4],
    dst: [f32; 4],
    float: bool,
    display: bool,
) -> [f32; 4] {
    let mut out = blend_pixel_in(mode, src, dst, float, display);
    if matches!(mode, BlendMode::StencilAlpha | BlendMode::StencilLuma) {
        return out;
    }
    let clear = 1.0 - dst[3];
    for c in 0..3 {
        out[c] -= clear * src[c];
    }
    out[3] = dst[3];
    out
}

/// B-800: how near a mode's hard edge counts as on it (a value of exactly 0 or 1, or Hard Mix's sum
/// of exactly 1), so a few last bits of rounding never flip a pixel from black to white and the CPU
/// and the card agree; After Effects' 8 bpc whole numbers land on the edge exactly.
const EDGE: f64 = 1e-5;

/// D-921: Color Dodge, on encoded `b` beneath and `s` the layer.
fn dodge(b: f64, s: f64) -> f64 {
    if b <= EDGE {
        0.0
    } else if s >= 1.0 - EDGE {
        1.0
    } else {
        (b / (1.0 - s)).min(1.0)
    }
}

/// D-921: Color Burn, on encoded `b` beneath and `s` the layer.
fn burn(b: f64, s: f64) -> f64 {
    if b >= 1.0 - EDGE {
        1.0
    } else if s <= EDGE {
        0.0
    } else {
        1.0 - ((1.0 - b) / s).min(1.0)
    }
}

/// D-921: the fourteen modes' per-channel rules (document 21), the colour beneath first.
fn more_mixer(mode: BlendMode) -> Option<fn(f64, f64) -> f64> {
    let f: fn(f64, f64) -> f64 = match mode {
        BlendMode::ColorDodge | BlendMode::ClassicColorDodge => dodge,
        BlendMode::ColorBurn => burn,
        BlendMode::ClassicColorBurn => |b, s| if s <= EDGE { b } else { burn(b, s) },
        BlendMode::LinearBurn => |b, s| (b + s - 1.0).max(0.0),
        BlendMode::LinearLight => |b, s| (b + 2.0 * s - 1.0).clamp(0.0, 1.0),
        BlendMode::VividLight => |b, s| {
            if s <= 0.5 {
                burn(b, 2.0 * s)
            } else {
                dodge(b, 2.0 * s - 1.0)
            }
        },
        BlendMode::PinLight => |b, s| {
            if s <= 0.5 {
                b.min(2.0 * s)
            } else {
                b.max(2.0 * s - 1.0)
            }
        },
        BlendMode::HardMix => |b, s| if b + s >= 1.0 - EDGE { 1.0 } else { 0.0 },
        BlendMode::Exclusion => |b, s| b + s - 2.0 * b * s,
        BlendMode::Subtract => |b, s| (b - s).max(0.0),
        BlendMode::Divide => |b, s| {
            if s <= EDGE {
                if b > EDGE { 1.0 } else { 0.0 }
            } else {
                (b / s).min(1.0)
            }
        },
        _ => return None,
    };
    Some(f)
}

/// D-921: Darker Color and Lighter Color, the whole colour with the lower or higher W3C Lum, the
/// colour beneath on a tie.
fn more_whole(mode: BlendMode) -> Option<fn([f64; 3], [f64; 3]) -> [f64; 3]> {
    fn lum(c: [f64; 3]) -> f64 {
        0.3 * c[0] + 0.59 * c[1] + 0.11 * c[2]
    }
    let f: fn([f64; 3], [f64; 3]) -> [f64; 3] = match mode {
        BlendMode::DarkerColor => |b, s| if lum(s) < lum(b) { s } else { b },
        BlendMode::LighterColor => |b, s| if lum(s) > lum(b) { s } else { b },
        _ => return None,
    };
    Some(f)
}

/// One pixel through document 21's "Blend modes", source over destination.
///
/// The document's general equation, with `B` the mode's component-wise blend of the *straight*
/// colours:
///
/// ```text
/// Co = (1-As)*Cd + (1-Ad)*Cs + As*Ad*B(cs,cd)
/// Ao = As + Ad - As*Ad
/// ```
///
/// Normal routes to [`over_pixel`] instead. Document 21 states it separately, as
/// `Co = Cs + Cd*(1-As)`, and that is the authoritative form; substituting `B = cs` into the
/// equation above reduces to exactly it, so this is a shortcut past an unpremultiply and a
/// re-multiply rather than a second definition. `tests/b05c_blend.rs` checks the two agree.
///
/// `add` clamps its blend function to 1, which document 21 calls "the bounded G1 display
/// blend". That clamp is part of the mode, not the encoding clamp this module otherwise
/// refuses to do: it is applied to the straight colours before weighting, so a result can
/// still leave 0..1 through the `(1-Ad)*Cs` term the way normal-over can.
pub fn blend_pixel(mode: BlendMode, src: [f32; 4], dst: [f32; 4]) -> [f32; 4] {
    blend_pixel_at(mode, src, dst, false)
}

/// D-319: [`blend_pixel`] in a composition's working depth. In Float, Add is `cs + cd`, not held
/// to 1, and Screen is Nuke's rule, `cs + cd - cs*cd` while either is at most 1, else the larger.
/// Every other mode is the same in both depths.
pub fn blend_pixel_at(mode: BlendMode, src: [f32; 4], dst: [f32; 4], float: bool) -> [f32; 4] {
    blend_pixel_in(mode, src, dst, float, false)
}

/// D-763: [`blend_pixel_at`], with `display` when both pixels hold display values (a composition
/// in 8 bpc (After Effects), D-330): every mode then blends them as they are, and the modes that
/// encode and decode the colours (D-301, D-748) do neither.
pub fn blend_pixel_in(
    mode: BlendMode,
    src: [f32; 4],
    dst: [f32; 4],
    float: bool,
    display: bool,
) -> [f32; 4] {
    if mode == BlendMode::Normal {
        return over_pixel(src, dst);
    }
    // D-301: a stencil is not drawn; it keeps what is under it by its alpha, or by its picture
    // luma as a Luma matte reads one (its colour over black, encoded).
    if matches!(mode, BlendMode::StencilAlpha | BlendMode::StencilLuma) {
        let k = if mode == BlendMode::StencilAlpha {
            src[3]
        } else {
            let luma = 0.2126 * src[0] as f64 + 0.7152 * src[1] as f64 + 0.0722 * src[2] as f64;
            if display {
                luma.clamp(0.0, 1.0) as f32
            } else {
                crate::grade::to_srgb(luma.clamp(0.0, 1.0)) as f32
            }
        };
        return dst.map(|v| v * k);
    }
    // Document 21: "first recover straight colors cs and cd where alpha is nonzero", and
    // "Zero-alpha straight colors are zero", which is what `unpremultiply` already does.
    let cs = unpremultiply(src);
    let cd = unpremultiply(dst);
    let (a_s, a_d) = (src[3], dst[3]);
    // D-301, D-748: the modes After Effects works on the encoded colours, held to 0..1 as it
    // blends them at 8 bits, and decoded again.
    let e = |v: f32| {
        let v = (v as f64).clamp(0.0, 1.0);
        if display {
            v
        } else {
            crate::grade::to_srgb(v)
        }
    };
    let d = |v: f64| if display { v } else { crate::grade::to_linear(v) };
    // D-748: Hue, Saturation, Color and Luminosity take the whole colour at once, by the W3C's
    // rules (D-730's, Turbulent Noise's), the colour beneath first.
    let whole = crate::grade::noise_whole(mode.as_str())
        .or(more_whole(mode))
        .map(|f| f([0, 1, 2].map(|c| e(cd[c])), [0, 1, 2].map(|c| e(cs[c]))));
    let mut out = [0.0f32; 4];
    // D-921 (our basis): a Classic mode scales the layer's encoded colour by its alpha first and
    // lays the result at full weight wherever there is something beneath.
    if let (BlendMode::ClassicColorDodge | BlendMode::ClassicColorBurn, Some(mix)) =
        (mode, more_mixer(mode))
    {
        for c in 0..3 {
            let b = d(mix(e(cd[c]), e(cs[c]) * a_s as f64)) as f32;
            out[c] = (1.0 - a_d) * src[c] + a_d * b;
        }
        out[3] = a_s + a_d - a_s * a_d;
        return out;
    }
    for c in 0..3 {
        let b = match mode {
            BlendMode::Multiply => cs[c] * cd[c],
            BlendMode::Screen if float && cs[c] > 1.0 && cd[c] > 1.0 => cs[c].max(cd[c]),
            BlendMode::Screen => cs[c] + cd[c] - cs[c] * cd[c],
            BlendMode::Add if float => cs[c] + cd[c],
            BlendMode::Add => (cs[c] + cd[c]).min(1.0),
            // D-301: on the encoded colours, so a 50% grey is Overlay's neutral one; D-748 adds
            // Turbulent Noise's (D-647) Hard Light, Darken, Lighten and Difference the same way.
            // The mixer takes the colour beneath first.
            BlendMode::Overlay
            | BlendMode::SoftLight
            | BlendMode::HardLight
            | BlendMode::Darken
            | BlendMode::Lighten
            | BlendMode::Difference => {
                let mix = crate::grade::noise_mixer(mode.as_str());
                d(mix(e(cd[c]), e(cs[c]))) as f32
            }
            BlendMode::Hue
            | BlendMode::Saturation
            | BlendMode::Color
            | BlendMode::Luminosity
            | BlendMode::DarkerColor
            | BlendMode::LighterColor => d(whole.expect("a whole-colour mode")[c]) as f32,
            BlendMode::ColorBurn
            | BlendMode::LinearBurn
            | BlendMode::ColorDodge
            | BlendMode::LinearLight
            | BlendMode::VividLight
            | BlendMode::PinLight
            | BlendMode::HardMix
            | BlendMode::Exclusion
            | BlendMode::Subtract
            | BlendMode::Divide => {
                let mix = more_mixer(mode).expect("a D-921 mode");
                d(mix(e(cd[c]), e(cs[c]))) as f32
            }
            BlendMode::ClassicColorBurn | BlendMode::ClassicColorDodge => unreachable!("handled above"),
            BlendMode::Normal | BlendMode::StencilAlpha | BlendMode::StencilLuma => {
                unreachable!("handled above")
            }
        };
        out[c] = (1.0 - a_s) * dst[c] + (1.0 - a_d) * src[c] + a_s * a_d * b;
    }
    out[3] = a_s + a_d - a_s * a_d;
    out
}
