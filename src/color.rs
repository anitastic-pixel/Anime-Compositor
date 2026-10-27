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

// ---------------------------------------------------------------------------------------------
// Named colour spaces (ADR-023, D-906). Nothing in the build calls these yet: colour management
// is off, and today's pipeline above is unchanged. `tests/b785_colour_conversions.rs` checks them
// against OpenColorIO's numbers in `Fixtures/colour/expected_conversions.json`.
// ---------------------------------------------------------------------------------------------

/// A set of RGB primaries and a white, as CIE 1931 xy chromaticities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gamut {
    /// ITU-R BT.709 / sRGB primaries, D65 white: today's working space, unnamed until now.
    Rec709,
    /// ACES AP1 (ACEScg, ACEScct, ACEScc), the ACES white (about D60).
    Ap1,
    /// ACES AP0 (ACES2065-1), the ACES white.
    Ap0,
    /// ITU-R BT.2020 primaries, D65 white.
    Rec2020,
    /// DCI-P3 primaries, D65 white (Display P3's primaries).
    P3D65,
}

type M3 = [[f64; 3]; 3];

const D65: [f64; 2] = [0.3127, 0.3290];
const ACES_WHITE: [f64; 2] = [0.32168, 0.33767];

impl Gamut {
    pub const ALL: [Gamut; 5] = [
        Gamut::Rec709,
        Gamut::Ap1,
        Gamut::Ap0,
        Gamut::Rec2020,
        Gamut::P3D65,
    ];

    /// Red, green, blue and white xy, from ITU-R BT.709, SMPTE ST 2065-1, the ACEScg
    /// specification (S-2014-004), ITU-R BT.2020 and SMPTE EG 432-1.
    pub fn chromaticities(self) -> [[f64; 2]; 4] {
        match self {
            Gamut::Rec709 => [[0.64, 0.33], [0.30, 0.60], [0.15, 0.06], D65],
            Gamut::Ap1 => [[0.713, 0.293], [0.165, 0.830], [0.128, 0.044], ACES_WHITE],
            Gamut::Ap0 => [[0.7347, 0.2653], [0.0, 1.0], [0.0001, -0.0770], ACES_WHITE],
            Gamut::Rec2020 => [[0.708, 0.292], [0.170, 0.797], [0.131, 0.046], D65],
            Gamut::P3D65 => [[0.680, 0.320], [0.265, 0.690], [0.150, 0.060], D65],
        }
    }

    /// RGB to CIE XYZ (Y of white = 1), the usual normalised primary matrix.
    pub fn to_xyz(self) -> M3 {
        let [r, g, b, w] = self.chromaticities();
        let p = [xyz(r), xyz(g), xyz(b)];
        let p = [
            [p[0][0], p[1][0], p[2][0]],
            [p[0][1], p[1][1], p[2][1]],
            [p[0][2], p[1][2], p[2][2]],
        ];
        let s = apply(&invert(&p), xyz(w));
        [0, 1, 2].map(|row| [0, 1, 2].map(|col| p[row][col] * s[col]))
    }
}

fn xyz([x, y]: [f64; 2]) -> [f64; 3] {
    [x / y, 1.0, (1.0 - x - y) / y]
}

fn mul(a: &M3, b: &M3) -> M3 {
    [0, 1, 2].map(|r| [0, 1, 2].map(|c| a[r][0] * b[0][c] + a[r][1] * b[1][c] + a[r][2] * b[2][c]))
}

fn apply(m: &M3, v: [f64; 3]) -> [f64; 3] {
    [0, 1, 2].map(|r| m[r][0] * v[0] + m[r][1] * v[1] + m[r][2] * v[2])
}

fn invert(m: &M3) -> M3 {
    let [[a, b, c], [d, e, f], [g, h, i]] = *m;
    let (ca, cb, cc) = (e * i - f * h, f * g - d * i, d * h - e * g);
    let det = a * ca + b * cb + c * cc;
    [
        [ca / det, (c * h - b * i) / det, (b * f - c * e) / det],
        [cb / det, (a * i - c * g) / det, (c * d - a * f) / det],
        [cc / det, (b * g - a * h) / det, (a * e - b * d) / det],
    ]
}

/// Bradford white adaptation from one white to another. Bradford, not CAT02, because that is
/// what OpenColorIO's ACES configs were made with: their Rec.709-to-ACES2065-1 matrix equals the
/// Bradford one to 1e-15 and differs from CAT02's by up to 5.2e-3 in an entry (D-906).
fn bradford(from: [f64; 2], to: [f64; 2]) -> M3 {
    const M: M3 = [
        [0.8951, 0.2664, -0.1614],
        [-0.7502, 1.7135, 0.0367],
        [0.0389, -0.0685, 1.0296],
    ];
    let (s, d) = (apply(&M, xyz(from)), apply(&M, xyz(to)));
    let scale = [[d[0] / s[0], 0.0, 0.0], [0.0, d[1] / s[1], 0.0], [0.0, 0.0, d[2] / s[2]]];
    mul(&invert(&M), &mul(&scale, &M))
}

/// The 3x3 matrix taking linear RGB in `from` to linear RGB in `to`, in double precision; a
/// Bradford adaptation between the two whites when they differ, none when they are the same.
pub fn gamut_matrix(from: Gamut, to: Gamut) -> M3 {
    let (wf, wt) = (from.chromaticities()[3], to.chromaticities()[3]);
    let src = if wf == wt {
        from.to_xyz()
    } else {
        mul(&bradford(wf, wt), &from.to_xyz())
    };
    mul(&invert(&to.to_xyz()), &src)
}

/// D-909: `rgb` through a [`gamut_matrix`] in double precision, rounded once to single precision.
pub fn apply_f32(m: &M3, rgb: [f32; 3]) -> [f32; 3] {
    apply(m, rgb.map(f64::from)).map(|c| c as f32)
}

/// D-910: every pixel of a premultiplied RGBA picture through [`gamut_matrix`]`(from, to)`, alpha
/// untouched (a matrix commutes with premultiplying). The 25 matrices are worked out once.
pub fn convert_rgba(data: &mut [f32], from: Gamut, to: Gamut) {
    use rayon::prelude::*;
    static ALL: std::sync::OnceLock<[[M3; 5]; 5]> = std::sync::OnceLock::new();
    let m = &ALL.get_or_init(|| Gamut::ALL.map(|f| Gamut::ALL.map(|t| gamut_matrix(f, t))))
        [from as usize][to as usize];
    data.par_chunks_mut(4).for_each(|px| {
        let c = apply_f32(m, [px[0], px[1], px[2]]);
        px[..3].copy_from_slice(&c);
    });
}

/// A transfer curve: how a stored number relates to linear light.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Curve {
    Linear,
    /// IEC 61966-2-1, exactly today's [`srgb_to_linear`] and [`linear_to_srgb`] (D-17).
    Srgb,
    /// ITU-R BT.1886 with black at 0: a pure power of 2.4. Below 0 a number passes unchanged,
    /// as OpenColorIO's ExponentTransform (pass_thru) does.
    Bt1886,
    /// A pure power of 2.2, below 0 unchanged as [`Curve::Bt1886`].
    Gamma22,
    /// ACEScct (S-2016-001), without the half-float cap at 65504, as OpenColorIO.
    AcesCct,
    /// ACEScc (S-2014-003), without the half-float cap at 65504.
    AcesCc,
}

impl Curve {
    /// Stored number to linear light.
    pub fn to_linear(self, c: f32) -> f32 {
        let x = c as f64;
        match self {
            Curve::Linear => c,
            Curve::Srgb => srgb_to_linear(c),
            Curve::Bt1886 if c >= 0.0 => x.powf(2.4) as f32,
            Curve::Gamma22 if c >= 0.0 => x.powf(2.2) as f32,
            Curve::Bt1886 | Curve::Gamma22 => c,
            Curve::AcesCct if x <= 0.155251141552511 => {
                ((x - 0.0729055341958355) / 10.5402377416545) as f32
            }
            Curve::AcesCc if x <= (9.72 - 15.0) / 17.52 => {
                ((2f64.powf(x * 17.52 - 9.72) - 2f64.powi(-16)) * 2.0) as f32
            }
            Curve::AcesCct | Curve::AcesCc => 2f64.powf(x * 17.52 - 9.72) as f32,
        }
    }

    /// Linear light to stored number.
    pub fn from_linear(self, c: f32) -> f32 {
        let x = c as f64;
        match self {
            Curve::Linear => c,
            Curve::Srgb => linear_to_srgb(c),
            Curve::Bt1886 if c >= 0.0 => x.powf(1.0 / 2.4) as f32,
            Curve::Gamma22 if c >= 0.0 => x.powf(1.0 / 2.2) as f32,
            Curve::Bt1886 | Curve::Gamma22 => c,
            Curve::AcesCct if x <= 0.0078125 => (10.5402377416545 * x + 0.0729055341958355) as f32,
            Curve::AcesCc if x <= 0.0 => ((-16.0 + 9.72) / 17.52) as f32,
            Curve::AcesCc if x < 2f64.powi(-15) => {
                (((2f64.powi(-16) + x * 0.5).log2() + 9.72) / 17.52) as f32
            }
            Curve::AcesCct | Curve::AcesCc => ((x.log2() + 9.72) / 17.52) as f32,
        }
    }
}

/// A named colour space: primaries and white, and a curve.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Space {
    pub gamut: Gamut,
    pub curve: Curve,
}

pub const LINEAR_REC709: Space = Space { gamut: Gamut::Rec709, curve: Curve::Linear };
pub const SRGB_REC709: Space = Space { gamut: Gamut::Rec709, curve: Curve::Srgb };
pub const BT1886_REC709: Space = Space { gamut: Gamut::Rec709, curve: Curve::Bt1886 };
pub const GAMMA22_REC709: Space = Space { gamut: Gamut::Rec709, curve: Curve::Gamma22 };
pub const ACESCG: Space = Space { gamut: Gamut::Ap1, curve: Curve::Linear };
pub const ACESCCT: Space = Space { gamut: Gamut::Ap1, curve: Curve::AcesCct };
pub const ACESCC: Space = Space { gamut: Gamut::Ap1, curve: Curve::AcesCc };
pub const ACES2065_1: Space = Space { gamut: Gamut::Ap0, curve: Curve::Linear };
pub const LINEAR_REC2020: Space = Space { gamut: Gamut::Rec2020, curve: Curve::Linear };
pub const LINEAR_P3_D65: Space = Space { gamut: Gamut::P3D65, curve: Curve::Linear };

/// One colour from `from` to `to`: each channel to linear light, the matrix in double precision
/// (skipped when the primaries are the same), then each channel to `to`'s curve.
// ponytail: the matrix is worked out on every call; a caller converting pictures keeps
// `gamut_matrix` once (C3 does, at decode).
pub fn convert(rgb: [f32; 3], from: Space, to: Space) -> [f32; 3] {
    let lin = rgb.map(|c| from.curve.to_linear(c));
    let lin = if from.gamut == to.gamut {
        lin
    } else {
        apply(&gamut_matrix(from.gamut, to.gamut), lin.map(f64::from)).map(|c| c as f32)
    };
    lin.map(|c| to.curve.from_linear(c))
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
