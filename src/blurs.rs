//! D-92's directional blur and D-95's radial blur: document 21's rules, on a layer's own
//! pixels.
//!
//! This program's own methods, modelled on After Effects' Directional Blur and Radial Blur;
//! nothing is ported. `tools/directional_blur_reference.py` and `tools/radial_blur_reference.py`
//! are the same rules worked a second way, and `tests/b36_directional_blur.rs` and
//! `tests/b39_radial_blur.rs` hold these to their numbers.

use crate::render::sample_bilinear;
use crate::WorkingBuffer;
use rayon::prelude::*;

/// One pixel's step along `direction`, in degrees clockwise from up. Exact at a whole quarter
/// turn, so a streak straight across or straight down takes nothing from the lines beside it.
fn along(direction: f64) -> (f64, f64) {
    let q = direction.rem_euclid(360.0);
    if q == 0.0 {
        (0.0, -1.0)
    } else if q == 90.0 {
        (1.0, 0.0)
    } else if q == 180.0 {
        (0.0, 1.0)
    } else if q == 270.0 {
        (-1.0, 0.0)
    } else {
        let a = direction.to_radians();
        (a.sin(), -a.cos())
    }
}

/// Average each pixel of `source` along a line `length` pixels long through it, in place, and
/// return how far it grew on each side. Both settings are already inside their ranges; length 0
/// changes nothing.
pub(crate) fn directional_blur(source: &mut WorkingBuffer, direction: f64, length: f64) -> usize {
    let n = length.ceil() as usize + 1;
    if n == 1 {
        return 0;
    }
    let grow = (length / 2.0).ceil() as usize;
    let u = along(direction);
    let steps: Vec<(f64, f64)> = (0..n)
        .map(|k| {
            let t = -length / 2.0 + k as f64 * length / (n - 1) as f64;
            (t * u.0, t * u.1)
        })
        .collect();
    let (w, h) = (source.width() + 2 * grow, source.height() + 2 * grow);
    let mut out = WorkingBuffer::transparent(w, h);
    let src = &*source;
    // ponytail: n bilinear samples a pixel, so a 1080p layer at length 500 is ~1e9 samples;
    // a running sum along the line is the upgrade if P-17 finds it matters.
    out.data_mut()
        .par_chunks_exact_mut(w * 4)
        .enumerate()
        .for_each(|(y, row)| {
            let cy = y as f64 - grow as f64 + 0.5;
            for (x, px) in row.chunks_exact_mut(4).enumerate() {
                let cx = x as f64 - grow as f64 + 0.5;
                let mut sum = [0.0f32; 4];
                for &(dx, dy) in &steps {
                    let s = sample_bilinear(src, cx + dx, cy + dy);
                    for i in 0..4 {
                        sum[i] += s[i];
                    }
                }
                for i in 0..4 {
                    px[i] = sum[i] / n as f32;
                }
            }
        });
    *source = out;
    grow
}

/// D-95's most samples a pixel.
const MOST: usize = 256;

/// Average each pixel of `source` round `center`, in the buffer's pixels, in place: along the
/// arc about it for a spin, along the line from it for a zoom. The settings are already inside
/// their ranges; amount 0 changes nothing. The buffer keeps its size.
pub(crate) fn radial_blur(source: &mut WorkingBuffer, spin: bool, amount: f64, center: (f64, f64)) {
    if amount == 0.0 {
        return;
    }
    let (w, h) = (source.width(), source.height());
    let (cx, cy) = center;
    let mut out = WorkingBuffer::transparent(w, h);
    let src = &*source;
    // ponytail: up to 256 bilinear samples a pixel, with a sine and cosine each for a spin; a
    // table of turns per sample count is the upgrade if P-17 finds it matters.
    out.data_mut()
        .par_chunks_exact_mut(w * 4)
        .enumerate()
        .for_each(|(y, row)| {
            let dy = y as f64 + 0.5 - cy;
            for (x, px) in row.chunks_exact_mut(4).enumerate() {
                let dx = x as f64 + 0.5 - cx;
                let r = (dx * dx + dy * dy).sqrt();
                let path = if spin {
                    r * amount * std::f64::consts::PI / 180.0
                } else {
                    r * amount / 100.0
                };
                let n = (path.ceil() as usize + 1).min(MOST);
                if n == 1 {
                    let i = (y * w + x) * 4;
                    px.copy_from_slice(&src.data()[i..i + 4]);
                    continue;
                }
                // Summed in double precision: 256 single-precision additions would drift near
                // the fixtures' tolerance.
                let mut sum = [0.0f64; 4];
                for k in 0..n {
                    let (sx, sy) = if spin {
                        let t = (-amount / 2.0 + k as f64 * amount / (n - 1) as f64).to_radians();
                        let (sin, cos) = t.sin_cos();
                        (cx + dx * cos - dy * sin, cy + dx * sin + dy * cos)
                    } else {
                        let s = 1.0 - amount / 200.0 + k as f64 * (amount / 100.0) / (n - 1) as f64;
                        (cx + s * dx, cy + s * dy)
                    };
                    let s = sample_bilinear(src, sx, sy);
                    for i in 0..4 {
                        sum[i] += s[i] as f64;
                    }
                }
                for i in 0..4 {
                    px[i] = (sum[i] / n as f64) as f32;
                }
            }
        });
    *source = out;
}
