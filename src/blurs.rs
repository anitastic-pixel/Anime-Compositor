//! D-92's directional blur, read by D-98's lines, and D-95's radial blur: document 21's rules,
//! on a layer's own pixels.
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

/// D-98's weights along a line, by column `j` from the line's own: `a - b * |j|` for
/// `|j| <= inner`, and `ends` beyond, so a running sum does the inner part whatever its width.
pub(crate) struct Weights {
    pub inner: usize,
    pub a: f64,
    pub b: f64,
    pub ends: Vec<(isize, f64)>,
}

/// D-98's lines over a `width` by `height` layer grown by `grow`: whether x and y are exchanged
/// (mostly down), the slope, the grown size in the exchanged picture, and the first and last
/// line. B-47's card draws the same lines from these.
pub(crate) struct LineFrame {
    pub down: bool,
    pub s: f64,
    pub ow: usize,
    pub oh: usize,
    pub k0: isize,
    pub k1: isize,
}

pub(crate) fn line_frame(u: (f64, f64), width: usize, height: usize, grow: usize) -> LineFrame {
    let down = u.0.abs() < u.1.abs();
    let (u, w, h) = if down { ((u.1, u.0), height, width) } else { (u, width, height) };
    let s = u.1 / u.0;
    let g = grow as isize;
    let (ow, oh) = (w + 2 * grow, h + 2 * grow);
    // Line k runs through y = k + 0.5 + x s in layer pixels; the pixel (x, y) sits between
    // lines floor(y - x s) and the one after, and the corners give the first and last needed.
    let line_of = |x: isize, y: isize| (y as f64 - x as f64 * s).floor() as isize;
    let corners = [
        (-g, -g),
        (ow as isize - 1 - g, -g),
        (-g, oh as isize - 1 - g),
        (ow as isize - 1 - g, oh as isize - 1 - g),
    ];
    let k0 = corners.iter().map(|&(x, y)| line_of(x, y)).min().unwrap();
    let k1 = corners.iter().map(|&(x, y)| line_of(x, y)).max().unwrap() + 1;
    LineFrame { down, s, ow, oh, k0, k1 }
}

/// D-98: `source` read along lines of step `u`, one pixel apart, each point of a line the
/// weighted sum of the line's samples at the column centres, and each pixel of the layer grown
/// by `grow` the straight mix of the two lines round its centre. Mostly down is mostly across
/// with x and y exchanged, read in place: turning the picture on its side and back cost more
/// than the blur (B-42).
pub(crate) fn by_lines(
    source: &WorkingBuffer,
    u: (f64, f64),
    wt: &Weights,
    grow: usize,
) -> WorkingBuffer {
    // Below, x and y are across and down in the exchanged picture when `down`.
    let LineFrame { down, s, ow, oh, k0, k1 } = line_frame(u, source.width(), source.height(), grow);
    let g = grow as isize;
    let reach = wt
        .ends
        .iter()
        .map(|e| e.0.unsigned_abs())
        .max()
        .unwrap_or(0)
        .max(wt.inner);
    let span = ow + 2 * reach;
    // Whether a sample can be anything but zero: by each row's span across, or by the
    // drawing's box down, which is enough to skip an empty cel's lines.
    let spans = spans(source);
    let r0 = spans.iter().position(Option::is_some).unwrap_or(1) as isize;
    let r1 = spans.iter().rposition(Option::is_some).unwrap_or(0) as isize;
    let c0 = spans.iter().flatten().map(|s| s.0).min().unwrap_or(1) as isize;
    let c1 = spans.iter().flatten().map(|s| s.1).max().unwrap_or(0) as isize;
    let shows = |x: isize, y: f64| {
        let r = (y - 0.5).floor() as isize;
        if down {
            r0 <= x && x <= r1 && c0 <= r + 1 && r <= c1
        } else {
            [r, r + 1].into_iter().any(|r| {
                r >= 0
                    && (r as usize) < spans.len()
                    && matches!(spans[r as usize], Some((a, b)) if a as isize <= x && x <= b as isize)
            })
        }
    };

    // Every line's value at every column, lines in parallel. A running total of the samples,
    // and of the samples times their column for a tent, gives each point with a few subtractions.
    // ponytail: every line held at once, ~100 MB for a 1080p layer at 45 degrees; bands of
    // lines are the upgrade if memory matters.
    let mut lines = vec![[0.0f32; 4]; (k1 - k0 + 1) as usize * ow];
    // Each rayon job zeroes its own totals, so a job is at least 16 lines, or that zeroing
    // outweighs the blur (P-17's harness, B-42).
    let fresh = || {
        (
            vec![[0.0f32; 4]; span],
            vec![[0.0f64; 4]; span + 1],
            vec![[0.0f64; 4]; span + 1],
            vec![0u32; span + 1],
        )
    };
    // Which lines hold anything, so the mix below can leave an empty cel's pixels alone.
    let used: Vec<bool> = lines
        .par_chunks_exact_mut(ow)
        .enumerate()
        .with_min_len(16)
        .map_init(fresh, |(v, p, q, c), (i, line)| {
            let k = k0 + i as isize;
            for t in 0..span {
                let x = t as isize - reach as isize - g;
                let y = k as f64 + 0.5 + x as f64 * s;
                v[t] = if !shows(x, y) {
                    [0.0; 4]
                } else if down {
                    sample_bilinear(source, y, x as f64 + 0.5)
                } else {
                    sample_bilinear(source, x as f64 + 0.5, y)
                };
                c[t + 1] = c[t] + v[t].iter().any(|&z| z != 0.0) as u32;
                for ch in 0..4 {
                    p[t + 1][ch] = p[t][ch] + v[t][ch] as f64;
                    q[t + 1][ch] = q[t][ch] + t as f64 * v[t][ch] as f64;
                }
            }
            if c[span] == 0 {
                return false;
            }
            let n = wt.inner;
            for (x, out) in line.iter_mut().enumerate() {
                let t = x + reach;
                // Nothing in reach is exactly nothing, whatever the running totals have rounded to.
                if c[t + reach + 1] == c[t - reach] {
                    continue;
                }
                let tf = t as f64;
                for ch in 0..4 {
                    let mut r = wt.a * (p[t + n + 1][ch] - p[t - n][ch]);
                    if wt.b != 0.0 {
                        let right = (q[t + n + 1][ch] - q[t + 1][ch])
                            - tf * (p[t + n + 1][ch] - p[t + 1][ch]);
                        let left = tf * (p[t][ch] - p[t - n][ch]) - (q[t][ch] - q[t - n][ch]);
                        r -= wt.b * (right + left);
                    }
                    for &(j, w) in &wt.ends {
                        r += w * v[(t as isize + j) as usize][ch] as f64;
                    }
                    out[ch] = r as f32;
                }
            }
            true
        })
        .collect();

    let (width, height) = if down { (oh, ow) } else { (ow, oh) };
    let mut out = WorkingBuffer::transparent(width, height);
    out.data_mut()
        .par_chunks_exact_mut(width * 4)
        .enumerate()
        .for_each(|(ro, row)| {
            for (co, px) in row.chunks_exact_mut(4).enumerate() {
                let (xo, yo) = if down { (ro, co) } else { (co, ro) };
                let d = (yo as isize - g) as f64 - (xo as isize - g) as f64 * s;
                let k = d.floor();
                let f = (d - k) as f32;
                let at = (k as isize - k0) as usize;
                if !used[at] && !used[at + 1] {
                    continue;
                }
                let i = at * ow + xo;
                let (lo, hi) = (lines[i], lines[i + ow]);
                for ch in 0..4 {
                    px[ch] = (1.0 - f) * lo[ch] + f * hi[ch];
                }
            }
        });
    out
}

/// The integral of tent(v) = max(0, 1 - |v|) from a to b.
fn tent_integral(a: f64, b: f64) -> f64 {
    let up_to = |v: f64| {
        let v = v.clamp(-1.0, 1.0);
        if v < 0.0 {
            (1.0 + v).powi(2) / 2.0
        } else {
            1.0 - (1.0 - v).powi(2) / 2.0
        }
    };
    up_to(b) - up_to(a)
}

/// Average each pixel of `source` along a line `length` pixels long through it, in place, and
/// return how far it grew on each side. Both settings are already inside their ranges; length 0
/// changes nothing. D-98: each line, drawn straight between its column samples, is averaged
/// over `2H = |u_x| * (length + length / ceil(length))` columns, mostly across.
pub(crate) fn directional_blur(source: &mut WorkingBuffer, direction: f64, length: f64) -> usize {
    if length == 0.0 {
        return 0;
    }
    let grow = (length / 2.0).ceil() as usize;
    let u = along(direction);
    let half = u.0.abs().max(u.1.abs()) * (length + length / length.ceil()) / 2.0;
    let reach = (half + 1.0).ceil() as isize;
    let full = 1.0 / (2.0 * half);
    let weight = |j: isize| tent_integral(j as f64 - half, j as f64 + half) / (2.0 * half);
    // The columns weighing exactly 1 / 2H are the running sum; a blur too short for even its
    // own column to is all ends.
    let inner = (0..=reach).take_while(|&j| weight(j) == full).last();
    let wt = Weights {
        inner: inner.unwrap_or(0) as usize,
        a: if inner.is_some() { full } else { 0.0 },
        b: 0.0,
        ends: (-reach..=reach)
            .filter(|&j| inner.map_or(true, |n| j.abs() > n) && weight(j) != 0.0)
            .map(|j| (j, weight(j)))
            .collect(),
    };
    *source = by_lines(source, u, &wt, grow);
    grow
}

/// D-95's most samples a pixel.
pub(crate) const MOST: usize = 256;

/// P-17: each sample count's turns as `(sin, cos)`, or scales as `(scale, 0)` for a zoom, worked
/// once by the same sums a pixel would do, rather than a sine and cosine for every sample of
/// every pixel. Entry `n` holds `n` of them from 2 up; 0 and 1 are empty. B-46 sends the same
/// numbers to the graphics card.
pub(crate) fn radial_turns(spin: bool, amount: f64) -> Vec<Vec<(f64, f64)>> {
    (0..=MOST)
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
        .collect()
}

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
    let turns = radial_turns(spin, amount);
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
