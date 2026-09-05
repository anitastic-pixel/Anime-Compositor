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
