//! Transfer functions between sRGB encoding and linear light.
//!
//! **Specification gap, flagged rather than assumed.** Document 21 requires that "RGB tagged
//! sRGB is converted to linear light before premultiplication" and that PNG output
//! "converts the linear working RGB to the declared output encoding", but it never states
//! which transfer function that is, nor the rounding rule for the 8-bit quantisation step.
//! This module implements the IEC 61966-2-1 sRGB transfer function and round-half-away-from-
//! zero quantisation, and says so here so the choice is inspectable. If document 21 later
//! declares something different, this file is wrong and the fixtures move with the document.

/// sRGB electro-optical transfer function: encoded value to linear light.
///
/// `c_lin = c/12.92` for `c <= 0.04045`, otherwise `((c+0.055)/1.055)^2.4`.
pub fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// Inverse: linear light to sRGB encoded value.
///
/// `c = 12.92 * c_lin` for `c_lin <= 0.0031308`, otherwise `1.055 * c_lin^(1/2.4) - 0.055`.
pub fn linear_to_srgb(c: f32) -> f32 {
    if c <= 0.0031308 {
        12.92 * c
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

/// Quantise an sRGB-encoded channel to 8 bits.
///
/// Clamps to 0..1 first: document 21 says intermediate math may exceed 0..1 and that
/// "final integer output conversion clamps only at the declared encoding step". This is
/// that step. Rounding is to nearest, ties away from zero.
pub fn quantise_u8(c: f32) -> u8 {
    let c = if c.is_nan() { 0.0 } else { c.clamp(0.0, 1.0) };
    (c * 255.0 + 0.5).floor() as u8
}

/// Dequantise an 8-bit channel back to a normalised sRGB-encoded value.
pub fn dequantise_u8(v: u8) -> f32 {
    v as f32 / 255.0
}
