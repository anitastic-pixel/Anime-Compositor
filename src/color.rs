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
///
/// **P-03(c): a table for the 256 values a PNG can produce, and the function for everything
/// else.** `verification/P-01_frame_trace.md` measures this transfer plus the premultiply at
/// 30.7% to 50.7% of a cold preview frame, the largest named stage in every cold row it has,
/// and the `powf` below is why. Every sample that reaches it from a decoded 8-bit PNG is one of
/// exactly 256 values, [`dequantise_u8`] of a byte, so 255 of every 256 calls recompute an
/// answer this build already has.
///
/// The fast path is bit-exact rather than close, and the check is what makes it so. [`TABLE`] is
/// built by calling this function's own slow path on all 256 dequantised values, so it cannot
/// disagree with what it replaces; and a sample takes the table's answer **only after the
/// dequantised value it would be indexed by is compared to it and found equal**. A sample that
/// is not one of the 256 falls through to the `powf` and is unaffected. That is why this is a
/// change no fixture can see, which is what P-03 requires of every item in it.
///
/// The comparison is on the bits and not on the value, because `-0.0 == 0.0` is true and the two
/// do not have the same answer: `-0.0 / 12.92` is `-0.0`. A test below holds that seam open.
pub fn srgb_to_linear(c: f32) -> f32 {
    // Round rather than truncate: v/255.0 times 255.0 lands within a rounding step of v, and
    // truncation would send most of the 256 to the slow path for nothing.
    let index = (c * 255.0 + 0.5) as i32;
    if (0..=255).contains(&index) {
        let index = index as usize;
        if TABLE.dequantised[index] == c.to_bits() {
            return TABLE.linear[index];
        }
    }
    srgb_to_linear_exact(c)
}

/// The transfer function itself, with no table in front of it. This is the definition; the table
/// is built from it and [`srgb_to_linear`] falls back to it.
fn srgb_to_linear_exact(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

/// The 256 values a decoded 8-bit channel can hold, and this transfer function applied to each.
///
/// Both halves are kept because the fast path needs the input side to prove a sample is one of
/// the 256 before it may use the output side.
struct Srgb8Table {
    /// The bit patterns of the 256 dequantised values, so that a negative zero cannot match the
    /// positive zero at index 0.
    dequantised: [u32; 256],
    linear: [f32; 256],
}

static TABLE: std::sync::LazyLock<Srgb8Table> = std::sync::LazyLock::new(|| {
    let mut table = Srgb8Table {
        dequantised: [0; 256],
        linear: [0.0; 256],
    };
    for v in 0..256 {
        let c = dequantise_u8(v as u8);
        table.dequantised[v] = c.to_bits();
        table.linear[v] = srgb_to_linear_exact(c);
    }
    table
});

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

/// Quantise an sRGB-encoded channel to 16 bits.
///
/// The same rule as [`quantise_u8`] against a different maximum: document 21 fixes the clamp at
/// "the declared encoding step" and says nothing about depth, so the two depths differ only in
/// the number they scale by.
pub fn quantise_u16(c: f32) -> u16 {
    let c = if c.is_nan() { 0.0 } else { c.clamp(0.0, 1.0) };
    (c * 65535.0 + 0.5).floor() as u16
}

/// Dequantise an 8-bit channel back to a normalised sRGB-encoded value.
pub fn dequantise_u8(v: u8) -> f32 {
    v as f32 / 255.0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// P-03(c)'s whole claim in one test: for every value a decoded 8-bit channel can hold, the
    /// table and the transfer function agree bit for bit, so no fixture can tell which ran.
    #[test]
    fn the_table_agrees_with_the_function_on_all_256_bytes() {
        for v in 0..=255u8 {
            let c = dequantise_u8(v);
            assert_eq!(
                srgb_to_linear(c).to_bits(),
                srgb_to_linear_exact(c).to_bits(),
                "byte {v} dequantises to {c} and the table disagrees with the function"
            );
        }
    }

    /// A sample that is not one of the 256 must not be given a neighbour's answer. These lie
    /// between dequantised values, on both sides of the 0.04045 knee, and at the ends.
    #[test]
    fn a_value_between_two_bytes_takes_the_slow_path() {
        let between = [
            0.5 / 255.0,
            9.5 / 255.0,
            10.4 / 255.0,
            128.5 / 255.0,
            254.5 / 255.0,
            0.000_001,
            0.999_999,
        ];
        for c in between {
            assert_eq!(
                srgb_to_linear(c).to_bits(),
                srgb_to_linear_exact(c).to_bits(),
                "{c} is not a dequantised byte and took a table entry anyway"
            );
        }
    }

    /// Document 21 lets intermediate values leave 0..1, and NaN reaches this function from a
    /// buffer that has been divided by a zero alpha. Neither may index the table.
    ///
    /// `-0.0` is in this list because it is the one input that compares equal to a table entry
    /// and has a different answer, and the first version of this table got it wrong: it returned
    /// `+0.0` where the function returns `-0.0`. That is one byte in one channel of one pixel,
    /// which is exactly what P-03 says is grounds for reverting an item rather than tolerating
    /// it. The fix is that the fast path compares bit patterns.
    #[test]
    fn values_outside_the_table_and_not_a_number_take_the_slow_path() {
        for c in [-1.0f32, -0.0, 2.0, 1e30, f32::INFINITY, f32::NEG_INFINITY] {
            assert_eq!(
                srgb_to_linear(c).to_bits(),
                srgb_to_linear_exact(c).to_bits(),
                "{c} is outside the table and took an entry anyway"
            );
        }
        assert!(srgb_to_linear(f32::NAN).is_nan(), "NaN came back a number");
    }
}
