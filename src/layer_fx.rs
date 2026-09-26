//! D-115's drop shadow and D-116's lens blur: document 21's rules, on a layer's own pixels.
//!
//! This program's own methods, modelled on After Effects' Drop Shadow and Camera Lens Blur;
//! nothing is ported. `tools/drop_shadow_reference.py` and `tools/lens_blur_reference.py` are
//! the same rules worked a second way, and `tests/b58_drop_shadow.rs` and
//! `tests/b59_lens_blur.rs` hold these to their numbers.

use crate::effects::{blur, kernel_radius};
use crate::render::sample_bilinear;
use crate::WorkingBuffer;
use rayon::prelude::*;

/// The pixel of `b` at `(x, y)`, transparent outside it.
fn at(b: &WorkingBuffer, x: isize, y: isize) -> [f32; 4] {
    if x < 0 || y < 0 || x as usize >= b.width() || y as usize >= b.height() {
        return [0.0; 4];
    }
    b.pixel(x as usize, y as usize)
}

/// D-115: the drawing's own shape, blurred at sigma `softness` / 3, moved `distance` pixels in
/// `direction`, in `color` (encoded 0 to 1) at `opacity` per cent, laid behind the drawing. The
/// layer grows by `ceil(distance)` and the blur's reach on every side, returned. The settings
/// are already valid.
pub(crate) fn drop_shadow(
    source: &mut WorkingBuffer,
    color: [f64; 3],
    opacity: f64,
    direction: f64,
    distance: f64,
    softness: f64,
) -> usize {
    let s = softness / 3.0;
    let r = kernel_radius(s) as isize;
    let g = distance.ceil() as usize + r as usize;
    let mut covering = source.clone();
    for px in covering.data_mut().chunks_exact_mut(4) {
        px[..3].fill(0.0);
    }
    blur(&mut covering, s);
    let (ux, uy) = crate::blurs::along(direction);
    let (k, c) = (opacity / 100.0, color.map(crate::grade::to_linear));
    let ow = source.width() + 2 * g;
    let mut out = WorkingBuffer::transparent(ow, source.height() + 2 * g);
    let drawing = &*source;
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = ((i % ow) as isize - g as isize, (i / ow) as isize - g as isize);
            let a = sample_bilinear(
                &covering,
                (x + r) as f64 + 0.5 - distance * ux,
                (y + r) as f64 + 0.5 - distance * uy,
            )[3] as f64
                * k;
            let d = at(drawing, x, y);
            let rest = a * (1.0 - d[3] as f64);
            for j in 0..3 {
                px[j] = (d[j] as f64 + c[j] * rest) as f32;
            }
            px[3] = (d[3] as f64 + rest) as f32;
        });
    *source = out;
    g
}

/// D-116: each pixel the plain mean of every pixel whose centre is within `radius` of its own,
/// a flat disc. Transparent edges read nothing outside the layer, which grows by
/// `ceil(radius)` on every side, returned; `repeat` holds each column and row inside it, and it
/// does not grow. Each row of the disc is one run, summed from the row's running totals. The
/// settings are already valid.
pub(crate) fn lens_blur(source: &mut WorkingBuffer, radius: f64, repeat: bool) -> usize {
    let g = if repeat { 0 } else { radius.ceil() as usize };
    let rr = radius * radius;
    let r = radius.floor() as isize;
    // Each row of the disc, its offset and half-width: the widest whole dx with dx^2 + dy^2 <= rr.
    let runs: Vec<(isize, isize)> = (-r..=r)
        .map(|dy| {
            let mut hw = (rr - (dy * dy) as f64).max(0.0).sqrt() as isize;
            while ((hw + 1) * (hw + 1) + dy * dy) as f64 <= rr {
                hw += 1;
            }
            while hw > 0 && (hw * hw + dy * dy) as f64 > rr {
                hw -= 1;
            }
            (dy, hw)
        })
        .collect();
    let n = runs.iter().map(|&(_, hw)| 2 * hw + 1).sum::<isize>() as f64;
    let (w, h) = (source.width() as isize, source.height() as isize);
    let src = source.data();
    // Each row's running totals, `sums[y][x]` the sum of its first x pixels.
    let sums: Vec<Vec<[f64; 4]>> = src
        .par_chunks(w as usize * 4)
        .map(|row| {
            let mut s = vec![[0.0; 4]; w as usize + 1];
            for (x, px) in row.chunks_exact(4).enumerate() {
                for i in 0..4 {
                    s[x + 1][i] = s[x][i] + px[i] as f64;
                }
            }
            s
        })
        .collect();
    let ow = w as usize + 2 * g;
    let mut out = WorkingBuffer::transparent(ow, h as usize + 2 * g);
    out.data_mut()
        .par_chunks_mut(ow * 4)
        .enumerate()
        .for_each(|(oy, line)| {
            let y = oy as isize - g as isize;
            for (ox, px) in line.chunks_exact_mut(4).enumerate() {
                let x = ox as isize - g as isize;
                let mut acc = [0.0f64; 4];
                for &(dy, hw) in &runs {
                    let mut sy = y + dy;
                    if repeat {
                        sy = sy.clamp(0, h - 1);
                    } else if sy < 0 || sy >= h {
                        continue;
                    }
                    let (s, row) = (&sums[sy as usize], sy as usize * w as usize * 4);
                    let (a, b) = (x - hw, x + hw);
                    let (lo, hi) = (a.max(0), b.min(w - 1));
                    for i in 0..4 {
                        if lo <= hi {
                            acc[i] += s[hi as usize + 1][i] - s[lo as usize][i];
                        }
                        if repeat {
                            let left = (b.min(-1) - a + 1).max(0) as f64;
                            let right = (b - a.max(w) + 1).max(0) as f64;
                            acc[i] += left * src[row + i] as f64
                                + right * src[row + (w as usize - 1) * 4 + i] as f64;
                        }
                    }
                }
                for i in 0..4 {
                    px[i] = (acc[i] / n) as f32;
                }
            }
        });
    *source = out;
    g
}
