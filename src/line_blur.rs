//! D-183's line blur: document 21's rule, on a layer's own pixels.
//!
//! This program's own rule, inspired by OpenToonz's Line Blur and not a port.
//! `tools/line_blur_reference.py` is the same rule worked a second way, and
//! `tests/b119_line_blur.rs` holds this to its numbers. The direction and the sample positions
//! are worked in double precision, so a tap lands where the reference's does.

use crate::WorkingBuffer;
use rayon::prelude::*;

/// How far the tensor reaches past anything drawn: a slope's one pixel and the smoothing's two.
pub(crate) const GROW: usize = 3;
const LUMA: [f64; 3] = [0.2126, 0.7152, 0.0722];
const SMOOTH: [f64; 5] = [1.0, 4.0, 6.0, 4.0, 1.0];
/// A side of the blur stops at the first tap covered less than this: the line has ended.
const STOP: f64 = 1.0 / 256.0;

fn cover(p: [f64; 4]) -> f64 {
    p[3]
}

fn ink(p: [f64; 4]) -> f64 {
    (p[3] - LUMA[0] * p[0] - LUMA[1] * p[1] - LUMA[2] * p[2]).clamp(0.0, 1.0)
}

/// The weight of each tap along the line, 1 to `length` rounded up pixels out. B-123's card takes
/// the same.
pub(crate) fn weights(length: f64) -> Vec<f64> {
    let sigma = length / 2.0;
    (1..=length.ceil() as i64).map(|k| (-((k * k) as f64) / (2.0 * sigma * sigma)).exp()).collect()
}

/// Soften `source`'s lines along their own length, in place, and return how far it grew on
/// each side. The settings are already valid; length 0 or strength 0 changes nothing.
pub(crate) fn line_blur(
    source: &mut WorkingBuffer,
    length: f64,
    strength: f64,
    lines_only: bool,
) -> usize {
    if length == 0.0 {
        return 0;
    }
    let g = GROW as i64;
    let (w0, h0) = (source.width() as i64, source.height() as i64);
    let (w, h) = (w0 + 2 * g, h0 + 2 * g);
    let src = source.data();
    let at = |x: i64, y: i64| -> [f64; 4] {
        if (0..w0).contains(&x) && (0..h0).contains(&y) {
            let i = ((y * w0 + x) * 4) as usize;
            [src[i] as f64, src[i + 1] as f64, src[i + 2] as f64, src[i + 3] as f64]
        } else {
            [0.0; 4]
        }
    };
    // The unsmoothed tensor, covering and ink together, two pixels past the grown bounds on
    // every side for the smoothing to read; (rx, ry) is the layer's (rx - g - 2, ry - g - 2).
    let rw = w + 4;
    let raw: Vec<[f64; 3]> = (0..(h + 4) * rw)
        .into_par_iter()
        .map(|i| {
            let (x, y) = (i % rw - g - 2, i / rw - g - 2);
            let (l, r, u, d) = (at(x - 1, y), at(x + 1, y), at(x, y - 1), at(x, y + 1));
            let mut t = [0.0; 3];
            for f in [cover as fn([f64; 4]) -> f64, ink] {
                let (gx, gy) = ((f(r) - f(l)) / 2.0, (f(d) - f(u)) / 2.0);
                t[0] += gx * gx;
                t[1] += gx * gy;
                t[2] += gy * gy;
            }
            t
        })
        .collect();
    let weights = weights(length);
    let sample = |px: f64, py: f64| -> [f64; 4] {
        let (x0, y0) = (px.floor(), py.floor());
        let (fx, fy) = (px - x0, py - y0);
        let (x0, y0) = (x0 as i64, y0 as i64);
        let (p00, p10, p01, p11) = (at(x0, y0), at(x0 + 1, y0), at(x0, y0 + 1), at(x0 + 1, y0 + 1));
        std::array::from_fn(|i| {
            (p00[i] * (1.0 - fx) + p10[i] * fx) * (1.0 - fy) + (p01[i] * (1.0 - fx) + p11[i] * fx) * fy
        })
    };
    let mut out = WorkingBuffer::transparent(w as usize, h as usize);
    // ponytail: every tap is four bilinear reads, ~400 at length 50; a pixel with no line near
    // it stops at the tensor, and draft halves the length.
    out.data_mut()
        .par_chunks_exact_mut(w as usize * 4)
        .enumerate()
        .for_each(|(y, row)| {
            let y = y as i64;
            for (x, px) in row.chunks_exact_mut(4).enumerate() {
                let x = x as i64;
                let (lx, ly) = (x - g, y - g);
                let own = at(lx, ly);
                let value = (|| {
                    if strength == 0.0 {
                        return own;
                    }
                    let (mut a, mut b, mut c) = (0.0, 0.0, 0.0);
                    for (j, wy) in SMOOTH.iter().enumerate() {
                        for (i, wx) in SMOOTH.iter().enumerate() {
                            let k = wx * wy / 256.0;
                            let t = raw[((y + j as i64) * rw + x + i as i64) as usize];
                            a += k * t[0];
                            b += k * t[1];
                            c += k * t[2];
                        }
                    }
                    let r = ((a - c) * (a - c) + 4.0 * b * b).sqrt();
                    if r == 0.0 {
                        return own;
                    }
                    let (tx, ty) = if a >= c {
                        (b, -(a - c + r) / 2.0)
                    } else {
                        (-(c - a + r) / 2.0, b)
                    };
                    let norm = (tx * tx + ty * ty).sqrt();
                    let (tx, ty) = (tx / norm, ty / norm);
                    let (mut acc, mut total) = (own, 1.0);
                    for side in [1.0, -1.0] {
                        for (k, wk) in weights.iter().enumerate() {
                            let step = side * (k + 1) as f64;
                            let s = sample(lx as f64 + step * tx, ly as f64 + step * ty);
                            if s[3] < STOP {
                                break;
                            }
                            for (v, q) in acc.iter_mut().zip(s) {
                                *v += wk * q;
                            }
                            total += wk;
                        }
                    }
                    let amount =
                        strength / 100.0 * r / (a + c) * if lines_only { ink(own) } else { 1.0 };
                    std::array::from_fn(|i| own[i] + (acc[i] / total - own[i]) * amount)
                })();
                for (p, v) in px.iter_mut().zip(value) {
                    *p = v as f32;
                }
            }
        });
    *source = out;
    GROW
}
