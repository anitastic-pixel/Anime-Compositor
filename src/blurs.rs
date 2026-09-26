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
pub(crate) fn along(direction: f64) -> (f64, f64) {
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

/// Each row's first and last column holding anything but zero, or `None` for an empty row.
pub(crate) fn spans(b: &WorkingBuffer) -> Vec<Option<(usize, usize)>> {
    b.data()
        .par_chunks_exact(b.width() * 4)
        .map(|row| {
            let full = |px: &[f32]| px.iter().any(|&v| v != 0.0);
            let lo = row.chunks_exact(4).position(full)?;
            Some((lo, row.chunks_exact(4).rposition(full)?))
        })
        .collect()
}

/// The columns of a row `grow` pixels wider on each side whose bilinear sample at
/// `(x - grow + 0.5 + dx, y)` can touch anything in `spans`, a pixel wide of the truth either
/// side. Every other sample in the row is exactly nothing, so leaving it out moves no bit.
pub(crate) fn reached(
    spans: &[Option<(usize, usize)>],
    y: f64,
    dx: f64,
    grow: usize,
    width: usize,
) -> std::ops::Range<usize> {
    let y0 = (y - 0.5).floor();
    let (mut lo, mut hi) = (usize::MAX, 0);
    for r in [y0, y0 + 1.0] {
        if r >= 0.0 && (r as usize) < spans.len() {
            if let Some((a, b)) = spans[r as usize] {
                (lo, hi) = (lo.min(a), hi.max(b));
            }
        }
    }
    if lo > hi {
        return 0..0;
    }
    let start = (lo as f64 - 2.0 + grow as f64 - dx).floor().max(0.0) as usize;
    let end = (hi as f64 + 3.0 + grow as f64 - dx)
        .ceil()
        .clamp(0.0, width as f64) as usize;
    start.min(end)..end
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
    let spans = spans(src);
    // P-17: a step at a time along the whole row, reading the source in order, and only where
    // the drawing is; each pixel still adds the same samples in the same order, so no bit moves.
    // ponytail: still n bilinear samples a pixel where the drawing is; a running sum along the
    // line is the upgrade if a longer streak matters.
    out.data_mut()
        .par_chunks_exact_mut(w * 4)
        .enumerate()
        .for_each(|(y, row)| {
            let cy = y as f64 - grow as f64 + 0.5;
            let mut sum = vec![[0.0f32; 4]; w];
            for &(dx, dy) in &steps {
                for x in reached(&spans, cy + dy, dx, grow, w) {
                    let cx = x as f64 - grow as f64 + 0.5;
                    let s = sample_bilinear(src, cx + dx, cy + dy);
                    for i in 0..4 {
                        sum[x][i] += s[i];
                    }
                }
            }
            for (px, sum) in row.chunks_exact_mut(4).zip(&sum) {
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
    // P-17: each sample count's turns, or scales for a zoom, worked once by the same sums a
    // pixel would do, rather than a sine and cosine for every sample of every pixel.
    let turns: Vec<Vec<(f64, f64)>> = (0..=MOST)
        .map(|n| {
            (0..if n < 2 { 0 } else { n })
                .map(|k| {
                    if spin {
                        let t = (-amount / 2.0 + k as f64 * amount / (n - 1) as f64).to_radians();
                        t.sin_cos()
                    } else {
                        (
                            1.0 - amount / 200.0 + k as f64 * (amount / 100.0) / (n - 1) as f64,
                            0.0,
                        )
                    }
                })
                .collect()
        })
        .collect();
    // P-17: the box round everything drawn, in pixel edges and a pixel wider each side. A path
    // that stays outside it takes nothing but empty samples, whose average is the empty pixel
    // the output already holds, so it is not walked.
    let spans = spans(src);
    let rows: Vec<usize> = (0..h).filter(|&y| spans[y].is_some()).collect();
    let (x0, x1) = spans
        .iter()
        .flatten()
        .fold((usize::MAX, 0), |(a, b), &(lo, hi)| (a.min(lo), b.max(hi)));
    let bbox = rows.first().map(|&y0| {
        let y1 = *rows.last().unwrap();
        (
            x0 as f64 - 1.0,
            y0 as f64 - 1.0,
            x1 as f64 + 2.0,
            y1 as f64 + 2.0,
        )
    });
    // How near to and far from the centre the box comes.
    let (near, far) = bbox.map_or((0.0, 0.0), |(l, t, r, b)| {
        let ex = (l - cx).max(cx - r).max(0.0);
        let ey = (t - cy).max(cy - b).max(0.0);
        let fx = (l - cx).abs().max((r - cx).abs());
        let fy = (t - cy).abs().max((b - cy).abs());
        ((ex * ex + ey * ey).sqrt(), (fx * fx + fy * fy).sqrt())
    });
    // ponytail: still up to 256 bilinear samples a pixel whose path meets the drawing; a
    // polar resampling is the upgrade if a big spin on a full-frame plate matters.
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
                let missed = match bbox {
                    None => true,
                    // A spin's samples all lie r from the centre.
                    Some(_) if spin => r + 1.0 < near || r - 1.0 > far,
                    // A zoom's lie on the line between its two ends.
                    Some((l, t, rr, b)) => {
                        let (s0, s1) = (turns[n][0].0, turns[n][n - 1].0);
                        let (ax, bx) = (cx + s0 * dx, cx + s1 * dx);
                        let (ay, by) = (cy + s0 * dy, cy + s1 * dy);
                        ax.max(bx) + 1.0 < l
                            || ax.min(bx) - 1.0 > rr
                            || ay.max(by) + 1.0 < t
                            || ay.min(by) - 1.0 > b
                    }
                };
                if missed {
                    continue;
                }
                // Summed in double precision: 256 single-precision additions would drift near
                // the fixtures' tolerance.
                let mut sum = [0.0f64; 4];
                for &(a, b) in &turns[n] {
                    let (sx, sy) = if spin {
                        let (sin, cos) = (a, b);
                        (cx + dx * cos - dy * sin, cy + dx * sin + dy * cos)
                    } else {
                        (cx + a * dx, cy + a * dy)
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
