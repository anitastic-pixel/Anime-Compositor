//! D-115's drop shadow, D-116's lens blur, D-117's rim light, D-118's outline and D-120's
//! chromatic aberration, and D-123's distance gradation, D-124's light rays, D-127's turbulent
//! displace and D-131's offset: document 21's rules, on a layer's own pixels.
//!
//! This program's own methods, modelled on After Effects' Drop Shadow, Camera Lens Blur,
//! Stroke and Optics Compensation and on rim lighting as compositors build it from a shifted
//! matte; nothing is ported. Each
//! `tools/<name>_reference.py` is the same rule worked a second way, and each
//! `tests/b5x_<name>.rs` holds it to its numbers.

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

/// Each row of the whole steps within `radius` of a pixel, its offset and half-width: the widest
/// whole dx with dx^2 + dy^2 <= radius^2.
pub(crate) fn disc_runs(radius: f64) -> Vec<(isize, isize)> {
    let rr = radius * radius;
    let r = radius.floor() as isize;
    (-r..=r)
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
        .collect()
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
    covering.data_mut().par_chunks_exact_mut(4).for_each(|px| {
        px[..3].fill(0.0);
    });
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

/// D-121: the iris's blades by its word, 0 for the circle; `None` for a word that is not one.
pub(crate) fn blades(iris: &str) -> Option<usize> {
    let names = [
        "circle", "", "", "triangle", "square", "pentagon", "hexagon", "heptagon", "octagon",
        "nonagon", "decagon",
    ];
    names.iter().position(|n| !n.is_empty() && *n == iris)
}

/// D-121: the furthest the iris reaches across or down, `radius` stretched by the aspect.
pub(crate) fn lens_reach(radius: f64, aspect: f64) -> f64 {
    let k = aspect.sqrt();
    radius * k.max(1.0 / k)
}

/// D-121: how far the step (dx, dy) is out on the iris: turned back by `rotation` (degrees
/// clockwise from up), squeezed back by the aspect, then the plain distance for a circle, or
/// for `n` blades the furthest it goes toward a side over the middle's distance to one, the
/// shape standing on a side with its corners at the radius, mixed toward the circle by
/// `roundness`. `tools/lens_blur_reference.py`'s `measure`, operation for operation.
fn iris_measure(dx: f64, dy: f64, n: usize, roundness: f64, rotation: f64, aspect: f64) -> f64 {
    let t = rotation.to_radians();
    let (s, c) = (t.sin(), t.cos());
    let (x, y) = (dx * c + dy * s, -dx * s + dy * c);
    let k = aspect.sqrt();
    let (u, v) = (x / k, y * k);
    let round = (u * u + v * v).sqrt();
    if n == 0 {
        return round;
    }
    let poly = (0..n)
        .map(|j| {
            let a = 2.0 * std::f64::consts::PI * j as f64 / n as f64;
            -u * a.sin() + v * a.cos()
        })
        .fold(f64::NEG_INFINITY, f64::max)
        / (std::f64::consts::PI / n as f64).cos();
    let w = roundness / 100.0;
    (1.0 - w) * poly + w * round
}

/// D-121: each row of the iris's whole steps, its offset and its first and last step across. A
/// step exactly on the edge counts, with a billionth of a pixel of slack. The iris is convex,
/// so each row is one run.
pub(crate) fn iris_runs(radius: f64, n: usize, roundness: f64, rotation: f64, aspect: f64) -> Vec<[isize; 3]> {
    const SLACK: f64 = 1e-9;
    if radius < 1.0 {
        return vec![[0, 0, 0]];
    }
    let r = lens_reach(radius + SLACK, aspect).floor() as isize;
    (-r..=r)
        .filter_map(|dy| {
            let mut on = (-r..=r).filter(|&dx| {
                iris_measure(dx as f64, dy as f64, n, roundness, rotation, aspect) <= radius + SLACK
            });
            let lo = on.next()?;
            Some([dy, lo, on.last().unwrap_or(lo)])
        })
        .collect()
}

/// D-116 and D-121's settings besides the radius and the edges.
pub(crate) struct Iris {
    pub blades: usize,
    pub roundness: f64,
    pub rotation: f64,
    pub aspect: f64,
    pub gain: f64,
    pub threshold: f64,
}

/// D-116 and D-121: each pixel spreads evenly over the iris, so each pixel is the plain mean of
/// the pixels the iris turned half round covers about it. Before it, a pixel whose brightest
/// straight channel is at or above the threshold has its colour multiplied by 1 + gain, and
/// after it a colour lit past its covering is held at it. Transparent edges read nothing
/// outside the layer, which grows by the iris's reach rounded up on every side, returned;
/// `repeat` holds each column and row inside it, and it does not grow. Each row of the iris is
/// one run, summed from the row's running totals. The settings are already valid.
pub(crate) fn lens_blur(source: &mut WorkingBuffer, radius: f64, repeat: bool, iris: &Iris) -> usize {
    let g = if repeat {
        0
    } else {
        lens_reach(radius, iris.aspect).ceil() as usize
    };
    let runs = iris_runs(radius, iris.blades, iris.roundness, iris.rotation, iris.aspect);
    let n = runs.iter().map(|&[_, lo, hi]| hi - lo + 1).sum::<isize>() as f64;
    let (w, h) = (source.width() as isize, source.height() as isize);
    let lit: Vec<f32>;
    let src = if iris.gain > 0.0 {
        let (m, at) = ((1.0 + iris.gain) as f32, (iris.threshold / 100.0) as f32);
        let mut copy = source.data().to_vec();
        copy.par_chunks_exact_mut(4).for_each(|p| {
            let bright = p[3] > 0.0 && p[0].max(p[1]).max(p[2]) / p[3] >= at;
            let k = if bright { m } else { 1.0 };
            for c in &mut p[..3] {
                *c *= k;
            }
        });
        lit = copy;
        &lit[..]
    } else {
        source.data()
    };
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
    // P-21: a row with nothing on it adds only +0 to sums that started at +0.
    let empty: Vec<bool> = src.par_chunks(w as usize * 4).map(|row| row.iter().all(|&v| v == 0.0)).collect();
    let ow = w as usize + 2 * g;
    let mut out = WorkingBuffer::transparent(ow, h as usize + 2 * g);
    out.data_mut()
        .par_chunks_mut(ow * 4)
        .enumerate()
        .for_each(|(oy, line)| {
            let y = oy as isize - g as isize;
            // P-21: an iris row at a time along the whole output row, each pixel still taking
            // the runs in order, so the bits are the pixel-at-a-time loop's.
            let mut sum = vec![[0.0f64; 4]; ow];
            for &[dy, lo, hi] in &runs {
                let mut sy = y - dy;
                if repeat {
                    sy = sy.clamp(0, h - 1);
                } else if sy < 0 || sy >= h {
                    continue;
                }
                if empty[sy as usize] {
                    continue;
                }
                let (s, row) = (&sums[sy as usize], sy as usize * w as usize * 4);
                if !repeat {
                    // Only the pixels whose run meets the row, where it is never empty.
                    let (first, last) = ((lo + g as isize).max(0), (w - 1 + hi + g as isize).min(ow as isize - 1));
                    for ox in first..=last {
                        let x = ox - g as isize;
                        let (l, r) = ((x - hi).max(0) as usize, (x - lo).min(w - 1) as usize);
                        let acc = &mut sum[ox as usize];
                        for i in 0..4 {
                            acc[i] += s[r + 1][i] - s[l][i];
                        }
                    }
                    continue;
                }
                for (ox, acc) in sum.iter_mut().enumerate() {
                    let x = ox as isize - g as isize;
                    let (a, b) = (x - hi, x - lo);
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
            }
            for (px, acc) in line.chunks_exact_mut(4).zip(&mut sum) {
                if iris.gain > 0.0 {
                    for i in 0..3 {
                        acc[i] = acc[i].min(acc[3]);
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

/// D-117: the edge of the drawing that faces the light lit. A pixel that shows is lit as far as
/// the point `width` pixels from it toward the light, `direction` degrees clockwise from up,
/// lies off the drawing's covering blurred at sigma `softness` / 3; it takes `color` (encoded 0
/// to 1) by `blend` at `intensity` per cent of that, its covering kept. The layer does not
/// grow. The settings are already valid.
pub(crate) fn rim_light(
    source: &mut WorkingBuffer,
    color: [f64; 3],
    direction: f64,
    width: f64,
    softness: f64,
    intensity: f64,
    blend: &str,
) {
    if intensity == 0.0 {
        return;
    }
    let s = softness / 3.0;
    let r = kernel_radius(s) as f64;
    let mut covering = source.clone();
    covering.data_mut().par_chunks_exact_mut(4).for_each(|px| {
        px[..3].fill(0.0);
    });
    blur(&mut covering, s);
    let (ux, uy) = crate::blurs::along(direction);
    let (k, c) = (intensity / 100.0, color.map(crate::grade::to_linear));
    let mix = crate::grade::mixer(blend);
    let w = source.width();
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let a = px[3] as f64;
            if a <= 0.0 {
                return;
            }
            let (x, y) = ((i % w) as f64 + r + 0.5, (i / w) as f64 + r + 0.5);
            let off = sample_bilinear(&covering, x + width * ux, y + width * uy)[3] as f64;
            let rim = (1.0 - off).clamp(0.0, 1.0) * k;
            for j in 0..3 {
                let b = px[j] as f64 / a;
                px[j] = ((b + rim * (mix(b, c[j]) - b)) * a) as f32;
            }
        });
}

/// D-118: a band of `color` (encoded 0 to 1) round the drawing's shape, laid behind it at
/// `opacity` per cent. The band at a pixel is the greatest covering within `width` of it, on
/// whole steps, blurred at sigma `softness` / 3. The layer grows by `ceil(width)` and the blur's
/// reach on every side, returned; width 0 changes nothing. The settings are already valid.
pub(crate) fn outline(
    source: &mut WorkingBuffer,
    color: [f64; 3],
    width: f64,
    softness: f64,
    opacity: f64,
) -> usize {
    if width == 0.0 {
        return 0;
    }
    let n = width.ceil() as usize;
    let (w, h) = (source.width(), source.height());
    let (bw, bh) = (w + 2 * n, h + 2 * n);
    let runs = disc_runs(width);
    // `row[sy][bx]`, the greatest covering in row sy within k of column bx - n, widened one step
    // of k at a time; each disc row of half-width k takes its maximum from it.
    let alpha = |x: isize, y: usize| at(source, x, y as isize)[3];
    let mut row = vec![0.0f32; h * bw];
    row.par_chunks_mut(bw).enumerate().for_each(|(sy, line)| {
        for (bx, v) in line.iter_mut().enumerate() {
            *v = alpha(bx as isize - n as isize, sy);
        }
    });
    let mut band = vec![0.0f32; bw * bh];
    let top = runs.iter().map(|&(_, hw)| hw).max().unwrap_or(0);
    for k in 0..=top {
        if k > 0 {
            row.par_chunks_mut(bw).enumerate().for_each(|(sy, line)| {
                for (bx, v) in line.iter_mut().enumerate() {
                    let x = bx as isize - n as isize;
                    *v = v.max(alpha(x - k, sy)).max(alpha(x + k, sy));
                }
            });
        }
        let rows = &row;
        band.par_chunks_mut(bw).enumerate().for_each(|(by, line)| {
            for &(dy, _) in runs.iter().filter(|&&(_, hw)| hw == k) {
                let sy = by as isize - n as isize + dy;
                if sy < 0 || sy >= h as isize {
                    continue;
                }
                let from = &rows[sy as usize * bw..][..bw];
                for (v, &u) in line.iter_mut().zip(from) {
                    *v = v.max(u);
                }
            }
        });
    }
    let mut ring = WorkingBuffer::transparent(bw, bh);
    ring.data_mut().par_chunks_exact_mut(4).zip(&band).for_each(|(px, &a)| {
        px[3] = a;
    });
    let g = n + blur(&mut ring, softness / 3.0);
    let (k, c) = (opacity / 100.0, color.map(crate::grade::to_linear));
    let ow = ring.width();
    let drawing = &*source;
    ring.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = ((i % ow) as isize - g as isize, (i / ow) as isize - g as isize);
            let d = at(drawing, x, y);
            let rest = px[3] as f64 * k * (1.0 - d[3] as f64);
            for j in 0..3 {
                px[j] = (d[j] as f64 + c[j] * rest) as f32;
            }
            px[3] = (d[3] as f64 + rest) as f32;
        });
    *source = ring;
    g
}

/// D-120: red drawn a little larger and blue a little smaller about `center`, per cent of the
/// drawing's own size (its corner at `origin` in `source` after the effects above grew it), so
/// that each moves `amount` pixels at the drawing's corner. Green is each pixel's own and the
/// covering the largest of the three. The layer does not grow; amount 0 changes nothing. The
/// settings are already valid.
pub(crate) fn chromatic_aberration(
    source: &mut WorkingBuffer,
    amount: f64,
    center: [f64; 2],
    origin: (usize, usize),
) {
    if amount == 0.0 {
        return;
    }
    let (cx, cy) = crate::effects::radial_center(center, source, origin);
    let w0 = (source.width() - 2 * origin.0) as f64;
    let h0 = (source.height() - 2 * origin.1) as f64;
    let k = amount / ((w0 * w0 + h0 * h0).sqrt() / 2.0);
    let drawing = source.clone();
    let w = source.width();
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (dx, dy) = ((i % w) as f64 + 0.5 - cx, (i / w) as f64 + 0.5 - cy);
            let red = sample_bilinear(&drawing, cx + dx * (1.0 - k), cy + dy * (1.0 - k));
            let blue = sample_bilinear(&drawing, cx + dx * (1.0 + k), cy + dy * (1.0 + k));
            px[0] = red[0];
            px[2] = blue[2];
            px[3] = px[3].max(red[3]).max(blue[3]);
        });
}

/// D-123: each place's squared distance along a line to the nearest place where `f` is 0, the
/// lower envelope of the parabolas rooted there (Felzenszwalb and Huttenlocher's pass). Every
/// other place holds a number far past any distance.
fn squared_distances(f: &[f64]) -> Vec<f64> {
    let n = f.len();
    let (mut v, mut z) = (vec![0usize; n], vec![0.0; n + 1]);
    let mut k = 0;
    z[0] = f64::NEG_INFINITY;
    z[1] = f64::INFINITY;
    let root = |q: usize| f[q] + (q * q) as f64;
    for q in 1..n {
        loop {
            let s = (root(q) - root(v[k])) / (2 * (q - v[k])) as f64;
            if s <= z[k] {
                k -= 1;
                continue;
            }
            k += 1;
            v[k] = q;
            z[k] = s;
            z[k + 1] = f64::INFINITY;
            break;
        }
    }
    k = 0;
    (0..n)
        .map(|q| {
            while z[k + 1] < q as f64 {
                k += 1;
            }
            let d = q as f64 - v[k] as f64;
            d * d + f[v[k]]
        })
        .collect()
}

/// D-123: `color` (encoded 0 to 1) shading in from the edge of the covering, all of `opacity`
/// per cent at the edge and none `width` pixels in, or the other way round with `invert`, mixed
/// by its blend. The edge is every pixel less than half covered and every place past the
/// buffer; distances are between pixel centres. The settings are already valid; width 0 changes
/// nothing.
pub(crate) fn distance_gradation(
    source: &mut WorkingBuffer,
    color: [f64; 3],
    width: f64,
    opacity: f64,
    invert: bool,
    blend: &str,
) {
    if width == 0.0 || opacity == 0.0 {
        return;
    }
    // The buffer inside a ring of edge pixels, one wide: no place past the buffer is nearer a
    // pixel than the ring is.
    let (w, h) = (source.width(), source.height());
    let (pw, ph) = (w + 2, h + 2);
    let far = 1e20;
    let drawing = &*source;
    let columns: Vec<Vec<f64>> = (0..pw)
        .into_par_iter()
        .map(|x| {
            let f: Vec<f64> = (0..ph)
                .map(|y| {
                    let inside = x >= 1 && y >= 1 && x <= w && y <= h;
                    if inside && drawing.pixel(x - 1, y - 1)[3] >= 0.5 { far } else { 0.0 }
                })
                .collect();
            squared_distances(&f)
        })
        .collect();
    let (g, mix) = (color.map(crate::grade::to_linear), crate::grade::mixer(blend));
    source
        .data_mut()
        .par_chunks_mut(w * 4)
        .enumerate()
        .for_each(|(y, row)| {
            let d = squared_distances(&(0..pw).map(|x| columns[x][y + 1]).collect::<Vec<_>>());
            for (x, px) in row.chunks_exact_mut(4).enumerate() {
                let a = px[3] as f64;
                if a <= 0.0 {
                    continue;
                }
                let t = (1.0 - (d[x + 1].sqrt() - 0.5) / width).clamp(0.0, 1.0);
                let o = if invert { 1.0 - t } else { t } * opacity / 100.0;
                for c in 0..3 {
                    let b = px[c] as f64 / a;
                    px[c] = ((b + o * (mix(b, g[c]) - b)) * a) as f32;
                }
            }
        });
}

/// D-124: the pixels D-89's bright test lights, zoomed out from `center` (in the buffer's
/// pixels) by Radial Blur's zoom at `length`, in `color` (encoded 0 to 1) at `intensity`,
/// added over the drawing. The rays stop at the buffer's edge. The settings are already valid;
/// intensity 0 changes nothing.
pub(crate) fn light_rays(
    source: &mut WorkingBuffer,
    center: (f64, f64),
    length: f64,
    threshold: f64,
    intensity: f64,
    color: [f64; 3],
) {
    if intensity == 0.0 {
        return;
    }
    let mut rays = source.clone();
    rays.data_mut().par_chunks_exact_mut(4).for_each(|px| {
        if !crate::bloom::bright(px, threshold) {
            px.fill(0.0);
        }
    });
    crate::blurs::radial_blur(&mut rays, false, length, center, false);
    let c = color.map(crate::grade::to_linear);
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .zip(rays.data().par_chunks_exact(4))
        .for_each(|(px, r)| {
            for k in 0..3 {
                px[k] = (px[k] as f64 + intensity * c[k] * r[k] as f64) as f32;
            }
            px[3] = (px[3] as f64 + intensity * r[3] as f64).min(1.0) as f32;
        });
}

/// D-127: each pixel read from a place pushed up to `amount` pixels by two channels of D-128's
/// fractal field, `size` pixels a cell, `octaves` deep at depth `z`, fixed to the drawing's own
/// space (its corner at `origin` in `source`). With transparent edges the layer first grows by
/// the amount rounded up, returned; with `repeat` a place past the edge reads the edge. The
/// settings are already valid; amount 0 changes nothing.
#[allow(clippy::too_many_arguments)]
pub(crate) fn turbulent_displace(
    source: &mut WorkingBuffer,
    amount: f64,
    size: f64,
    octaves: usize,
    seed: f64,
    z: f64,
    repeat: bool,
    origin: (usize, usize),
) -> usize {
    if amount == 0.0 {
        return 0;
    }
    let g = if repeat { 0 } else { amount.ceil() as usize };
    let (w, h) = (source.width() + 2 * g, source.height() + 2 * g);
    let (ox, oy) = ((origin.0 + g) as f64, (origin.1 + g) as f64);
    let base = crate::grade::mix(seed.floor() as u64);
    let mut out = WorkingBuffer::transparent(w, h);
    let drawing = &*source;
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
            let p = ((x - ox) / size, (y - oy) / size, z);
            let mut sx = x + amount * crate::grade::fractal(base, 0, p, octaves);
            let mut sy = y + amount * crate::grade::fractal(base, 1, p, octaves);
            if repeat {
                sx = sx.clamp(0.5, w as f64 - 0.5);
                sy = sy.clamp(0.5, h as f64 - 0.5);
            }
            px.copy_from_slice(&sample_bilinear(drawing, sx - g as f64, sy - g as f64));
        });
    *source = out;
    g
}

/// D-131: the whole buffer slid by `shift` pixels, what leaves one edge coming back in at the
/// other, read between pixels as bilinear sampling reads them. A whole-pixel shift moves the
/// pixels exactly. The settings are already valid.
pub(crate) fn offset(source: &mut WorkingBuffer, shift: [f64; 2]) {
    let (w, h) = (source.width(), source.height());
    if shift == [0.0, 0.0] || w == 0 || h == 0 {
        return;
    }
    let (qx, qy) = (-shift[0], -shift[1]);
    let (i0, j0) = (qx.floor(), qy.floor());
    let (fx, fy) = (qx - i0, qy - j0);
    let (i0, j0) = (i0 as i64, j0 as i64);
    let drawing = source.clone();
    let p = |x: i64, y: i64| {
        drawing.pixel(x.rem_euclid(w as i64) as usize, y.rem_euclid(h as i64) as usize)
    };
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = ((i % w) as i64 + i0, (i / w) as i64 + j0);
            let [a, b, c, d] = [p(x, y), p(x + 1, y), p(x, y + 1), p(x + 1, y + 1)];
            for k in 0..4 {
                let top = (1.0 - fx) * a[k] as f64 + fx * b[k] as f64;
                let bottom = (1.0 - fx) * c[k] as f64 + fx * d[k] as f64;
                px[k] = ((1.0 - fy) * top + fy * bottom) as f32;
            }
        });
}

/// D-144: every block `size` pixels wide, laid from the drawing's corner `(ox, oy)` in `source`,
/// painted the mean of the buffer's premultiplied pixels in it, empty ones counted as empty. A
/// block past the right or bottom edge is cut short. Size 1 or less changes nothing.
pub(crate) fn mosaic(source: &mut WorkingBuffer, size: f64, (ox, oy): (usize, usize)) {
    if size <= 1.0 {
        return;
    }
    let (w, h) = (source.width(), source.height());
    // Each run of columns, or of rows, that falls in one block.
    let runs = |n: usize, o: usize| {
        let block = |p: usize| ((p as f64 - o as f64) / size).floor();
        let mut runs = Vec::new();
        let mut start = 0;
        for p in 1..=n {
            if p == n || block(p) != block(start) {
                runs.push(start..p);
                start = p;
            }
        }
        runs
    };
    let (columns, rows) = (runs(w, ox), runs(h, oy));
    // P-22: each row of blocks on a thread of its own, its rows cut from the buffer in order.
    let mut rest = source.data_mut();
    let mut bands = Vec::new();
    for ys in &rows {
        let (band, tail) = rest.split_at_mut(ys.len() * w * 4);
        bands.push(band);
        rest = tail;
    }
    bands.into_par_iter().for_each(|data| {
        let n_rows = data.len() / (w * 4);
        for xs in &columns {
            let mut sum = [0.0f64; 4];
            for y in 0..n_rows {
                for x in xs.clone() {
                    let i = 4 * (y * w + x);
                    for c in 0..4 {
                        sum[c] += data[i + c] as f64;
                    }
                }
            }
            let n = (n_rows * xs.len()) as f64;
            let mean = sum.map(|v| (v / n) as f32);
            for y in 0..n_rows {
                for x in xs.clone() {
                    let i = 4 * (y * w + x);
                    data[i..i + 4].copy_from_slice(&mean);
                }
            }
        }
    });
}

/// The picture luma of a premultiplied pixel, its colour over black, encoded.
fn picture_luma(p: [f32; 4]) -> f64 {
    let y = 0.2126 * p[0] as f64 + 0.7152 * p[1] as f64 + 0.0722 * p[2] as f64;
    crate::grade::to_srgb(y.clamp(0.0, 1.0))
}

/// D-145: each pixel that shows lit by the difference in picture luma `relief` pixels ahead of
/// it along `direction` and as far behind, each read between pixels with the point held inside
/// the layer; grey, or with `color` laid over its own colours. The settings are already valid.
pub(crate) fn emboss(source: &mut WorkingBuffer, direction: f64, relief: f64, contrast: f64, color: bool) {
    let (w, h) = (source.width(), source.height());
    let (ux, uy) = crate::blurs::along(direction);
    let (dx, dy) = (relief * ux, relief * uy);
    let (fw, fh) = (w as f64, h as f64);
    let hold = |x: f64, y: f64| (x.max(0.5).min(fw - 0.5), y.max(0.5).min(fh - 0.5));
    let k = contrast / 100.0;
    let drawing = source.clone();
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let a = px[3] as f64;
            if a <= 0.0 {
                return;
            }
            let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
            let (ax, ay) = hold(x + dx, y + dy);
            let (bx, by) = hold(x - dx, y - dy);
            let ahead = picture_luma(sample_bilinear(&drawing, ax, ay));
            let behind = picture_luma(sample_bilinear(&drawing, bx, by));
            let v = 0.5 + (ahead - behind) * k;
            for c in 0..3 {
                let e = if color {
                    crate::grade::to_srgb((px[c] as f64 / a).clamp(0.0, 1.0)) + v - 0.5
                } else {
                    v
                };
                px[c] = (crate::grade::to_linear(e.clamp(0.0, 1.0)) * a) as f32;
            }
        });
}


/// D-146: each pixel that shows moved `amount` of the way toward the strength of the change in
/// picture luma around it, dark lines on white, or light on black when `invert`. The settings
/// are already valid.
pub(crate) fn find_edges(source: &mut WorkingBuffer, invert: bool, amount: f64) {
    if amount <= 0.0 {
        return;
    }
    let (w, h) = (source.width() as i64, source.height() as i64);
    let lumas: Vec<f64> = source.data().par_chunks_exact(4).map(|p| picture_luma([p[0], p[1], p[2], p[3]])).collect();
    let at = |x: i64, y: i64| lumas[(y.clamp(0, h - 1) * w + x.clamp(0, w - 1)) as usize];
    let t = amount / 100.0;
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let a = px[3] as f64;
            if a <= 0.0 {
                return;
            }
            let (x, y) = (i as i64 % w, i as i64 / w);
            let gx = at(x + 1, y - 1) + 2.0 * at(x + 1, y) + at(x + 1, y + 1)
                - at(x - 1, y - 1)
                - 2.0 * at(x - 1, y)
                - at(x - 1, y + 1);
            let gy = at(x - 1, y + 1) + 2.0 * at(x, y + 1) + at(x + 1, y + 1)
                - at(x - 1, y - 1)
                - 2.0 * at(x, y - 1)
                - at(x + 1, y - 1);
            let m = ((gx * gx + gy * gy).sqrt() / 2.0).min(1.0);
            let v = if invert { m } else { 1.0 - m };
            for c in 0..3 {
                let e = crate::grade::to_srgb((px[c] as f64 / a).clamp(0.0, 1.0));
                let e = e + t * (v - e);
                px[c] = (crate::grade::to_linear(e.clamp(0.0, 1.0)) * a) as f32;
            }
        });
}


/// D-147: each pixel that shows pushed `amount` percent further from its colour blurred at
/// sigma `radius`, both as written, the blur divided by its own covering. The settings are
/// already valid.
pub(crate) fn sharpen(source: &mut WorkingBuffer, amount: f64, radius: f64) {
    if amount <= 0.0 || radius <= 0.0 {
        return;
    }
    let mut blurred = source.clone();
    let r = crate::effects::blur(&mut blurred, radius);
    let k = amount / 100.0;
    let w = source.width();
    source
        .data_mut()
        .par_chunks_mut(w * 4)
        .enumerate()
        .for_each(|(y, row)| {
            for (x, px) in row.chunks_exact_mut(4).enumerate() {
                let a = px[3] as f64;
                if a <= 0.0 {
                    continue;
                }
                let b = blurred.pixel(x + r, y + r);
                let ba = b[3] as f64;
                for c in 0..3 {
                    let e = crate::grade::to_srgb((px[c] as f64 / a).clamp(0.0, 1.0));
                    let eb = if ba > 0.0 { crate::grade::to_srgb((b[c] as f64 / ba).clamp(0.0, 1.0)) } else { e };
                    let e = e + k * (e - eb);
                    px[c] = (crate::grade::to_linear(e.clamp(0.0, 1.0)) * a) as f32;
                }
            }
        });
}


/// D-148: each pixel that shows moved `amount` percent of the way toward its colour laid with
/// `blend` under the picture blurred at a third of `radius`, the blur divided by its own
/// covering, in linear light and not clamped. The settings are already valid.
pub(crate) fn diffusion(source: &mut WorkingBuffer, radius: f64, amount: f64, blend: &str) {
    if amount <= 0.0 || radius <= 0.0 {
        return;
    }
    let mut blurred = source.clone();
    let r = crate::effects::blur(&mut blurred, radius / 3.0);
    let k = amount / 100.0;
    let w = source.width();
    source
        .data_mut()
        .par_chunks_mut(w * 4)
        .enumerate()
        .for_each(|(y, row)| {
            for (x, px) in row.chunks_exact_mut(4).enumerate() {
                let a = px[3] as f64;
                if a <= 0.0 {
                    continue;
                }
                let g = blurred.pixel(x + r, y + r);
                let ga = g[3] as f64;
                for c in 0..3 {
                    let b = px[c] as f64 / a;
                    let g = if ga > 0.0 { g[c] as f64 / ga } else { b };
                    let f = match blend {
                        "screen" => 1.0 - (1.0 - b) * (1.0 - g),
                        "lighten" => b.max(g),
                        _ => g,
                    };
                    px[c] = ((b + k * (f - b)) * a) as f32;
                }
            }
        });
}


/// D-149: each pixel read from a place pushed `height` pixels across a wave `width` pixels
/// long running along `direction`, at `phase` degrees, fixed to the drawing's own space (its
/// corner at `origin` in `source`). With transparent edges the layer first grows by the height
/// rounded up, returned; with `repeat` a place past the edge reads the edge. The settings are
/// already valid; height 0 changes nothing.
pub(crate) fn wave_warp(
    source: &mut WorkingBuffer,
    triangle: bool,
    (height, width): (f64, f64),
    direction: f64,
    phase: f64,
    repeat: bool,
    origin: (usize, usize),
) -> usize {
    if height == 0.0 {
        return 0;
    }
    let g = if repeat { 0 } else { height.ceil() as usize };
    let (w, h) = (source.width() + 2 * g, source.height() + 2 * g);
    let (ox, oy) = ((origin.0 + g) as f64, (origin.1 + g) as f64);
    let (tx, ty) = crate::blurs::along(direction);
    let (nx, ny) = (-ty, tx);
    let phi = phase.to_radians();
    let tau = std::f64::consts::TAU;
    let mut out = WorkingBuffer::transparent(w, h);
    let drawing = &*source;
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
            let s = tx * (x - ox) + ty * (y - oy);
            let a = tau * s / width + phi;
            let v = if triangle { a.sin().asin() * 2.0 / std::f64::consts::PI } else { a.sin() };
            let mut sx = x - height * v * nx;
            let mut sy = y - height * v * ny;
            if repeat {
                sx = sx.clamp(0.5, w as f64 - 0.5);
                sy = sy.clamp(0.5, h as f64 - 0.5);
            }
            px.copy_from_slice(&sample_bilinear(drawing, sx - g as f64, sy - g as f64));
        });
    *source = out;
    g
}


/// D-150: each pixel read from a place pushed toward or away from `center`, a point in the
/// buffer, by `amplitude` pixels across rings `wavelength` apart at `phase` degrees, dying away
/// to nothing `fade` pixels out unless fade is 0. The settings are already valid.
pub(crate) fn ripple(source: &mut WorkingBuffer, center: (f64, f64), amplitude: f64, wavelength: f64, phase: f64, fade: f64) {
    if amplitude == 0.0 {
        return;
    }
    let w = source.width();
    let phi = phase.to_radians();
    let drawing = source.clone();
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
            let (vx, vy) = (x - center.0, y - center.1);
            let d = vx.hypot(vy);
            if d == 0.0 {
                return;
            }
            let f = if fade == 0.0 { 1.0 } else { (1.0 - d / fade).max(0.0) };
            let k = (std::f64::consts::TAU * d / wavelength - phi).sin() * f;
            if k == 0.0 {
                return;
            }
            let s = amplitude * k / d;
            px.copy_from_slice(&sample_bilinear(&drawing, x + s * vx, y + s * vy));
        });
}


/// D-151: each pixel within `radius` of `center`, a point in the buffer, read from a place
/// turned back round it by `angle` degrees times the square of how near the centre it is, so a
/// positive angle twists the picture clockwise, most at the middle. The settings are already
/// valid.
pub(crate) fn twirl(source: &mut WorkingBuffer, angle: f64, radius: f64, center: (f64, f64)) {
    if angle == 0.0 || radius <= 0.0 {
        return;
    }
    let w = source.width();
    let turn = angle.to_radians();
    let drawing = source.clone();
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
            let (vx, vy) = (x - center.0, y - center.1);
            let d = vx.hypot(vy);
            if d >= radius {
                return;
            }
            let t = 1.0 - d / radius;
            let (sin, cos) = (turn * t * t).sin_cos();
            let (sx, sy) = (vx * cos + vy * sin, vy * cos - vx * sin);
            px.copy_from_slice(&sample_bilinear(&drawing, center.0 + sx, center.1 + sy));
        });
}


/// D-152: each pixel within `radius` of `center`, a point in the buffer, read from a place
/// drawn toward the centre when `height` is above 0, a swell, or pushed away from it when
/// below, a pinch, most at the middle. The settings are already valid.
pub(crate) fn bulge(source: &mut WorkingBuffer, radius: f64, height: f64, center: (f64, f64)) {
    if height == 0.0 || radius <= 0.0 {
        return;
    }
    let w = source.width();
    let drawing = source.clone();
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
            let (vx, vy) = (x - center.0, y - center.1);
            let d = vx.hypot(vy);
            if d >= radius {
                return;
            }
            let t = 1.0 - d / radius;
            let m = (1.0 - height * t * t / 2.0).max(0.0);
            px.copy_from_slice(&sample_bilinear(&drawing, center.0 + m * vx, center.1 + m * vy));
        });
}


/// D-153: a straight line through `center`, a point in the buffer, turned by `angle` degrees
/// clockwise from straight up and down. Pixels on its kept side stay exactly; the rest take the
/// bilinear sample at their reflection across the line. The settings are already valid.
pub(crate) fn mirror(source: &mut WorkingBuffer, angle: f64, center: (f64, f64)) {
    let (nx, ny) = mirror_normal(angle);
    let w = source.width();
    let drawing = source.clone();
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
            let d = (x - center.0) * nx + (y - center.1) * ny;
            if d >= 0.0 {
                return;
            }
            px.copy_from_slice(&sample_bilinear(&drawing, x - 2.0 * d * nx, y - 2.0 * d * ny));
        });
}


/// D-153's normal, u(angle + 90) with u(t) = (sin t, -cos t), exact at whole quarter turns.
/// B-107's card takes the same.
pub(crate) fn mirror_normal(angle: f64) -> (f64, f64) {
    let theta = angle + 90.0;
    if theta.rem_euclid(90.0) == 0.0 {
        [(0.0, -1.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 0.0)][((theta / 90.0).floor() as i64).rem_euclid(4) as usize]
    } else {
        let (sin, cos) = theta.to_radians().sin_cos();
        (sin, -cos)
    }
}


/// D-154: how far a Motion Tile of `size` per cent grows a `w` by `h` drawing across and down,
/// on each side; nothing for an empty drawing. B-115: the card's plan asks it too.
pub(crate) fn tile_growth(size: (f64, f64), (w, h): (usize, usize)) -> (usize, usize) {
    if w == 0 || h == 0 {
        return (0, 0);
    }
    let grow = |n: usize, percent: f64| (n as f64 * (percent / 100.0 - 1.0) / 2.0).ceil() as usize;
    (grow(w, size.0), grow(h, size.1))
}

/// D-154: the buffer repeated round itself, tile against tile, to `size` per cent of its width
/// and height, every other tile turned over when `mirror`. Returns how far it grew on the left
/// and on the top, the same as on the right and the bottom. The settings are already valid.
pub(crate) fn motion_tile(source: &mut WorkingBuffer, size: (f64, f64), mirror: bool) -> (usize, usize) {
    let (w, h) = (source.width(), source.height());
    let (gx, gy) = tile_growth(size, (w, h));
    if (gx, gy) == (0, 0) {
        return (0, 0);
    }
    // The drawing's pixel for position `i` of the row or column, `g` in from the new edge.
    let tile = |i: usize, g: usize, n: usize| {
        let i = i as isize - g as isize;
        let (k, j) = (i.div_euclid(n as isize), i.rem_euclid(n as isize) as usize);
        if mirror && k % 2 != 0 { n - 1 - j } else { j }
    };
    let ow = w + 2 * gx;
    let mut out = WorkingBuffer::transparent(ow, h + 2 * gy);
    let drawing = source.data();
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let s = (tile(i / ow, gy, h) * w + tile(i % ow, gx, w)) * 4;
            px.copy_from_slice(&drawing[s..s + 4]);
        });
    *source = out;
    (gx, gy)
}

/// D-198: the drawing, whose corner is at `origin` in the buffer, stretched in perspective so
/// its upper left, upper right, lower left and lower right corners land on `pins`, each in per
/// cent of the drawing's width and height. Returns how far it grew on the left and on the top,
/// the same as on the right and the bottom. Crossed, bent-in or in-line corners leave the
/// buffer clear. The settings are already valid.
pub(crate) fn corner_pin(source: &mut WorkingBuffer, pins: [[f64; 2]; 4], origin: (usize, usize)) -> (usize, usize) {
    if pins == [[0.0, 0.0], [100.0, 0.0], [0.0, 100.0], [100.0, 100.0]] {
        return (0, 0);
    }
    let (w, h) = (source.width(), source.height());
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    let (dw, dh) = (w as f64 - 2.0 * ox, h as f64 - 2.0 * oy);
    let [ul, ur, ll, lr] = pins;
    // A, B, C, D round the ring, in the drawing's space.
    let ring = [ul, ur, lr, ll].map(|p| (p[0] / 100.0 * dw, p[1] / 100.0 * dh));
    let z = |i: usize| {
        let ((px, py), (qx, qy), (rx, ry)) = (ring[(i + 3) % 4], ring[i], ring[(i + 1) % 4]);
        (qx - px) * (ry - qy) - (qy - py) * (rx - qx)
    };
    let z = [z(0), z(1), z(2), z(3)];
    if !(z.iter().all(|v| *v > 0.0) || z.iter().all(|v| *v < 0.0)) {
        *source = WorkingBuffer::transparent(w, h);
        return (0, 0);
    }
    // Heckbert's map of the unit square onto A (0, 0), B (1, 0), C (1, 1), D (0, 1).
    let [a, b, c, d] = ring;
    let (sx, sy) = (a.0 - b.0 + c.0 - d.0, a.1 - b.1 + c.1 - d.1);
    let (dx1, dx2, dy1, dy2) = (b.0 - c.0, d.0 - c.0, b.1 - c.1, d.1 - c.1);
    let den = dx1 * dy2 - dx2 * dy1;
    let (g, hh) = ((sx * dy2 - dx2 * sy) / den, (dx1 * sy - sx * dy1) / den);
    let [[m0, m1, m2], [m3, m4, m5], [m6, m7, m8]] = [
        [b.0 - a.0 + g * b.0, d.0 - a.0 + hh * d.0, a.0],
        [b.1 - a.1 + g * b.1, d.1 - a.1 + hh * d.1, a.1],
        [g, hh, 1.0],
    ];
    let adj = [
        [m4 * m8 - m5 * m7, m2 * m7 - m1 * m8, m1 * m5 - m2 * m4],
        [m5 * m6 - m3 * m8, m0 * m8 - m2 * m6, m2 * m3 - m0 * m5],
        [m3 * m7 - m4 * m6, m1 * m6 - m0 * m7, m0 * m4 - m1 * m3],
    ];
    let det = m0 * (m4 * m8 - m5 * m7) - m1 * (m3 * m8 - m5 * m6) + m2 * (m3 * m7 - m4 * m6);
    // How far any corner lies past the buffer's sides, rounded up.
    let past = |lo: f64, hi: f64, v: [f64; 4]| {
        let (min, max) = v.iter().fold((f64::MAX, f64::MIN), |(n, x), v| (n.min(*v), x.max(*v)));
        (lo - min).max(max - hi).ceil().max(0.0) as usize
    };
    let gx = past(-ox, dw + ox, ring.map(|p| p.0));
    let gy = past(-oy, dh + oy, ring.map(|p| p.1));
    let ow = w + 2 * gx;
    let (left, top) = ((origin.0 + gx) as f64, (origin.1 + gy) as f64);
    let (wf, hf) = (w as f64, h as f64);
    let mut out = WorkingBuffer::transparent(ow, h + 2 * gy);
    let drawing = &*source;
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = ((i % ow) as f64 + 0.5 - left, (i / ow) as f64 + 0.5 - top);
            let [u, v, t] = adj.map(|r| r[0] * x + r[1] * y + r[2]);
            // Beyond the horizon the map draws the picture again, turned over; nothing is there.
            if !(t * det > 0.0) {
                return;
            }
            let (sx, sy) = (u / t * dw + ox, v / t * dh + oy);
            if !(sx > -1.0 && sx < wf + 1.0 && sy > -1.0 && sy < hf + 1.0) {
                return;
            }
            px.copy_from_slice(&sample_bilinear(drawing, sx, sy));
        });
    *source = out;
    (gx, gy)
}

/// D-201: the drawing, whose corner is at `origin` in the buffer, bent round its middle, its
/// rows into rings and its columns into spokes (`to_polar`), or unrolled, its rings into rows
/// and its spokes into columns, `interpolation` per cent of the way. The buffer keeps its size.
/// The settings are already valid.
pub(crate) fn polar_coordinates(source: &mut WorkingBuffer, interpolation: f64, to_polar: bool, origin: (usize, usize)) {
    let k = interpolation / 100.0;
    if k == 0.0 {
        return;
    }
    let (w, h) = (source.width(), source.height());
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    let (dw, dh) = (w as f64 - 2.0 * ox, h as f64 - 2.0 * oy);
    let turn = std::f64::consts::TAU;
    let mut out = WorkingBuffer::transparent(w, h);
    let drawing = &*source;
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = ((i % w) as f64 + 0.5 - ox, (i / w) as f64 + 0.5 - oy);
            let (sx, sy) = if to_polar {
                let (nx, ny) = ((x - dw / 2.0) / (dw / 2.0), (y - dh / 2.0) / (dh / 2.0));
                let phi = nx.atan2(-ny);
                let phi = if phi < 0.0 { phi + turn } else { phi };
                (phi / turn * dw, nx.hypot(ny) * dh)
            } else {
                let (a, v) = (turn * x / dw, y / dh);
                (dw / 2.0 * (1.0 + v * a.sin()), dh / 2.0 * (1.0 - v * a.cos()))
            };
            let (qx, qy) = (x + k * (sx - x), y + k * (sy - y));
            px.copy_from_slice(&if to_polar {
                sample_round(drawing, qx, qy + oy, dw as isize, origin.0 as isize)
            } else {
                sample_bilinear(drawing, qx + ox, qy + oy)
            });
        });
    *source = out;
}

/// D-205: one wedge of the drawing, whose corner is at `origin` in the buffer, repeated round
/// `center`, in per cent of the drawing, every other copy mirrored (`mirror`) or every copy
/// turned the same way. A point past the input reads it mirrored back at its edges. The buffer
/// keeps its size. The settings are already valid.
pub(crate) fn kaleidoscope(
    source: &mut WorkingBuffer,
    [segments, rotation, size]: [f64; 3],
    center: [f64; 2],
    mirror: bool,
    origin: (usize, usize),
) {
    let (w, h) = (source.width(), source.height());
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    let (cx, cy) = (center[0] / 100.0 * (w as f64 - 2.0 * ox), center[1] / 100.0 * (h as f64 - 2.0 * oy));
    let (wedge, rot) = (std::f64::consts::TAU / segments.floor(), rotation.to_radians());
    // A buffer coordinate folded into 0..n by mirroring at the edges, then held inside the
    // pixel centres.
    let fold = |u: f64, n: f64| {
        let m = u.rem_euclid(2.0 * n);
        (if m > n { 2.0 * n - m } else { m }).clamp(0.5, n - 0.5)
    };
    let mut out = WorkingBuffer::transparent(w, h);
    let drawing = &*source;
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (dx, dy) = ((i % w) as f64 + 0.5 - ox - cx, (i / w) as f64 + 0.5 - oy - cy);
            let theta = dx.atan2(-dy) - rot;
            let k = (theta / wedge).floor();
            let t = theta - k * wedge;
            let t = if mirror && k.rem_euclid(2.0) == 1.0 { wedge - t } else { t };
            let (s, phi) = (dx.hypot(dy) * 100.0 / size, rot + t);
            let (sx, sy) = (cx + s * phi.sin() + ox, cy - s * phi.cos() + oy);
            px.copy_from_slice(&sample_bilinear(drawing, fold(sx, w as f64), fold(sy, h as f64)));
        });
    *source = out;
}

/// D-206: each pixel keeps only as much covering as the input has a depth `e` away either way
/// across and down, `e` set by D-128's noise on the drawing's own space (corner at `origin`),
/// from nothing to `border`. With `color`, the band that would go at twice the depth takes that
/// linear colour. Nothing grows. The settings are already valid.
pub(crate) fn roughen_edges(
    source: &mut WorkingBuffer,
    color: Option<[f64; 3]>,
    [border, size, octaves, seed, z]: [f64; 5],
    origin: (usize, usize),
) {
    if border == 0.0 {
        return;
    }
    let w = source.width();
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    let base = crate::grade::mix(seed.floor() as u64);
    let input = source.clone();
    let a = |x: f64, y: f64| sample_bilinear(&input, x, y)[3] as f64;
    let least = |x: f64, y: f64, e: f64| a(x, y).min(a(x + e, y)).min(a(x - e, y)).min(a(x, y + e)).min(a(x, y - e));
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let p = px[3] as f64;
            if p == 0.0 {
                return;
            }
            let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
            let f = crate::grade::fractal(base, 0, ((x - ox) / size, (y - oy) / size, z), octaves as usize);
            let e = border * (0.5 + f).clamp(0.0, 1.0);
            let m = least(x, y, e);
            let k = m / p;
            let t = match color {
                Some(_) if m > 0.0 => 1.0 - least(x, y, 2.0 * e).min(m) / m,
                _ => 0.0,
            };
            let c = color.unwrap_or_default();
            for ch in 0..3 {
                px[ch] = ((1.0 - t) * px[ch] as f64 * k + t * c[ch] * m) as f32;
            }
            px[3] = m as f32;
        });
}

/// `sample_bilinear` at `x` across the drawing, `dw` wide from column `ox` of the buffer, and
/// `y` down the buffer, each tap's column taken round the drawing's width, so its left and
/// right edges join.
fn sample_round(src: &WorkingBuffer, x: f64, y: f64, dw: isize, ox: isize) -> [f32; 4] {
    let h = src.height() as isize;
    let (fx, fy) = (x - 0.5, y - 0.5);
    let (x0, y0) = (fx.floor(), fy.floor());
    let (ux, uy) = (fx - x0, fy - y0);
    let (x0, y0) = (x0 as isize, y0 as isize);
    let mut out = [0.0f32; 4];
    for (dy, wy) in [(0isize, 1.0 - uy), (1, uy)] {
        let sy = y0 + dy;
        if wy == 0.0 || sy < 0 || sy >= h {
            continue;
        }
        for (dx, wx) in [(0isize, 1.0 - ux), (1, ux)] {
            if wx == 0.0 {
                continue;
            }
            let weight = (wx * wy) as f32;
            let px = src.pixel(((x0 + dx).rem_euclid(dw) + ox) as usize, sy as usize);
            for i in 0..4 {
                out[i] += px[i] * weight;
            }
        }
    }
    out
}

/// D-155: every pixel behind a straight edge moving along `angle` gone, `completion` per cent of
/// the way across the drawing, whose corner is at `origin` in the buffer, softened over
/// `feather` pixels. The settings are already valid.
pub(crate) fn linear_wipe(
    source: &mut WorkingBuffer,
    completion: f64,
    angle: f64,
    feather: f64,
    origin: (usize, usize),
) {
    if completion == 0.0 {
        return;
    }
    if completion == 100.0 {
        source.data_mut().fill(0.0);
        return;
    }
    let w = source.width();
    let ((ux, uy), edge) = linear_edge(completion, angle, feather, source, origin);
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let x = (i % w) as f64 - ox + 0.5;
            let y = (i / w) as f64 - oy + 0.5;
            let s = ux * x + uy * y;
            let k = if feather > 0.0 {
                ((s - edge) / feather + 0.5).clamp(0.0, 1.0)
            } else if s >= edge {
                1.0
            } else {
                0.0
            };
            for v in px.iter_mut() {
                *v *= k as f32;
            }
        });
}


/// D-155's direction across and where its edge is, for the drawing whose corner is at `origin` in
/// `source`. B-107's card takes the same.
pub(crate) fn linear_edge(completion: f64, angle: f64, feather: f64, source: &WorkingBuffer, origin: (usize, usize)) -> ((f64, f64), f64) {
    let (w0, h0) = ((source.width() - 2 * origin.0) as f64, (source.height() - 2 * origin.1) as f64);
    let (ux, uy) = crate::blurs::along(angle);
    let corners = [0.0, ux * w0, uy * h0, ux * w0 + uy * h0];
    let low = corners.iter().copied().fold(f64::INFINITY, f64::min);
    let high = corners.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    ((ux, uy), low - feather / 2.0 + completion / 100.0 * (high - low + feather))
}


/// D-156: every pixel the hand of a clock has swept past gone, `completion` per cent of a turn
/// from `start_angle` round `center`, a point in the drawing's own pixels, whose corner is at
/// `origin` in the buffer; `wipe` 0 clockwise, 1 counterclockwise, 2 both ways; the sweeping
/// edge softened over `feather` degrees. The settings are already valid.
pub(crate) fn radial_wipe(
    source: &mut WorkingBuffer,
    completion: f64,
    start_angle: f64,
    center: [f64; 2],
    wipe: u8,
    feather: f64,
    origin: (usize, usize),
) {
    if completion == 0.0 {
        return;
    }
    if completion == 100.0 {
        source.data_mut().fill(0.0);
        return;
    }
    let turn = |a: f64| {
        let a = a.rem_euclid(360.0);
        if a >= 360.0 { 0.0 } else { a }
    };
    let w = source.width();
    let (w0, h0) = ((w - 2 * origin.0) as f64, (source.height() - 2 * origin.1) as f64);
    let (cx, cy) = (center[0] / 100.0 * w0, center[1] / 100.0 * h0);
    let edge = completion / 100.0 * (360.0 + feather) - feather / 2.0;
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (vx, vy) = ((i % w) as f64 - ox + 0.5 - cx, (i / w) as f64 - oy + 0.5 - cy);
            let screen = if vx == 0.0 && vy == 0.0 { 0.0 } else { turn(vx.atan2(-vy).to_degrees()) };
            let a = turn(screen - start_angle);
            let swept = match wipe {
                1 => if a > 0.0 { 360.0 - a } else { 0.0 },
                2 => 2.0 * a.min(360.0 - a),
                _ => a,
            };
            let k = if feather > 0.0 {
                ((swept - edge) / feather + 0.5).clamp(0.0, 1.0)
            } else if swept >= edge {
                1.0
            } else {
                0.0
            };
            for v in px.iter_mut() {
                *v *= k as f32;
            }
        });
}


/// D-157: the drawing, whose corner is at `origin` in the buffer, cut into slats `width` pixels
/// across, turned by `angle`, each cleared from one edge `completion` per cent of the way, the
/// moving edge softened over `feather` pixels. The settings are already valid.
pub(crate) fn venetian_blinds(
    source: &mut WorkingBuffer,
    completion: f64,
    angle: f64,
    width: f64,
    feather: f64,
    origin: (usize, usize),
) {
    if completion == 0.0 {
        return;
    }
    if completion == 100.0 {
        source.data_mut().fill(0.0);
        return;
    }
    let w = source.width();
    let (ux, uy) = crate::blurs::along(angle);
    let edge = completion / 100.0 * (width + feather) - feather / 2.0;
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let x = (i % w) as f64 - ox + 0.5;
            let y = (i / w) as f64 - oy + 0.5;
            let s = ux * x + uy * y;
            let t = s - width * (s / width).floor();
            let k = if feather > 0.0 {
                ((t - edge) / feather + 0.5).clamp(0.0, 1.0)
            } else if t >= edge {
                1.0
            } else {
                0.0
            };
            for v in px.iter_mut() {
                *v *= k as f32;
            }
        });
}


/// D-158: every pixel outside a circle closing on `center`, a point in the drawing's own pixels,
/// whose corner is at `origin` in the buffer, gone, `completion` per cent of the way; with
/// `invert`, every pixel inside a hole opening from it; the edge softened over `feather` pixels.
/// The settings are already valid.
pub(crate) fn iris_wipe(
    source: &mut WorkingBuffer,
    completion: f64,
    center: [f64; 2],
    feather: f64,
    invert: bool,
    origin: (usize, usize),
) {
    if completion == 0.0 {
        return;
    }
    if completion == 100.0 {
        source.data_mut().fill(0.0);
        return;
    }
    let w = source.width();
    let ((cx, cy), r) = iris_circle(completion, center, feather, invert, source, origin);
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let d = ((i % w) as f64 - ox + 0.5 - cx).hypot((i / w) as f64 - oy + 0.5 - cy);
            let inside = if invert { d - r } else { r - d };
            let k = if feather > 0.0 {
                (inside / feather + 0.5).clamp(0.0, 1.0)
            } else if inside >= 0.0 {
                1.0
            } else {
                0.0
            };
            for v in px.iter_mut() {
                *v *= k as f32;
            }
        });
}


/// D-158's centre, in the drawing's own pixels, and radius, for the drawing whose corner is at
/// `origin` in `source`. B-107's card takes the same.
pub(crate) fn iris_circle(completion: f64, center: [f64; 2], feather: f64, invert: bool, source: &WorkingBuffer, origin: (usize, usize)) -> ((f64, f64), f64) {
    let (w0, h0) = ((source.width() - 2 * origin.0) as f64, (source.height() - 2 * origin.1) as f64);
    let (cx, cy) = (center[0] / 100.0 * w0, center[1] / 100.0 * h0);
    let far = [(0.0, 0.0), (0.0, h0), (w0, 0.0), (w0, h0)]
        .iter()
        .map(|&(a, b)| (cx - a).hypot(cy - b))
        .fold(0.0, f64::max);
    let cc = completion / 100.0;
    ((cx, cy), if invert { cc } else { 1.0 - cc } * (far + feather) - feather / 2.0)
}


/// D-159: the covering shrunk (`choke` > 0) or spread (`choke` < 0) over the whole steps within
/// `|choke|` of each pixel, Outline's disc: shrinking, each covered pixel takes the least
/// covering within reach, outside the layer counting as empty, in its own straight colour;
/// spreading, the layer grows by the reach rounded down and each pixel takes the greatest
/// covering within reach, in its own straight colour where it had covering and the
/// covering-weighted average of the pixels within reach where it had none. Returns the growth.
/// The settings are already valid.
pub(crate) fn simple_choker(source: &mut WorkingBuffer, choke: f64) -> usize {
    if choke == 0.0 {
        return 0;
    }
    let spread = choke < 0.0;
    let g = if spread { (-choke).floor() as usize } else { 0 };
    let (w, h) = (source.width(), source.height());
    let (bw, bh) = (w + 2 * g, h + 2 * g);
    let pick: fn(f32, f32) -> f32 = if spread { f32::max } else { f32::min };
    let runs = disc_runs(choke.abs());
    let alpha = |x: isize, y: usize| at(source, x, y as isize)[3];
    // `row[sy][bx]`, the least or greatest covering in row sy within k of column bx - g, widened
    // one step of k at a time, as Outline's band is; each disc row of half-width k takes from it.
    let mut row = vec![0.0f32; h * bw];
    row.par_chunks_mut(bw).enumerate().for_each(|(sy, line)| {
        for (bx, v) in line.iter_mut().enumerate() {
            *v = alpha(bx as isize - g as isize, sy);
        }
    });
    let mut band = vec![if spread { 0.0 } else { f32::INFINITY }; bw * bh];
    let top = runs.iter().map(|&(_, hw)| hw).max().unwrap_or(0);
    for k in 0..=top {
        if k > 0 {
            row.par_chunks_mut(bw).enumerate().for_each(|(sy, line)| {
                for (bx, v) in line.iter_mut().enumerate() {
                    let x = bx as isize - g as isize;
                    *v = pick(pick(*v, alpha(x - k, sy)), alpha(x + k, sy));
                }
            });
        }
        let rows = &row;
        band.par_chunks_mut(bw).enumerate().for_each(|(by, line)| {
            for &(dy, _) in runs.iter().filter(|&&(_, hw)| hw == k) {
                let sy = by as isize - g as isize + dy;
                if sy < 0 || sy >= h as isize {
                    // A row outside the layer is empty.
                    for v in line.iter_mut() {
                        *v = pick(*v, 0.0);
                    }
                    continue;
                }
                let from = &rows[sy as usize * bw..][..bw];
                for (v, &u) in line.iter_mut().zip(from) {
                    *v = pick(*v, u);
                }
            }
        });
    }
    if !spread {
        source.data_mut().par_chunks_exact_mut(4).zip(&band).for_each(|(px, &a)| {
            if px[3] == 0.0 {
                return;
            }
            let k = a as f64 / px[3] as f64;
            for v in &mut px[..3] {
                *v = (*v as f64 * k) as f32;
            }
            px[3] = a;
        });
        return 0;
    }
    // Each row's running sums of the four channels, so a disc row's sum is one difference.
    let data = source.data();
    let mut sums = vec![0.0f64; h * (w + 1) * 4];
    sums.par_chunks_mut((w + 1) * 4).enumerate().for_each(|(sy, line)| {
        for sx in 0..w {
            for j in 0..4 {
                line[(sx + 1) * 4 + j] = line[sx * 4 + j] + data[(sy * w + sx) * 4 + j] as f64;
            }
        }
    });
    let mut out = WorkingBuffer::transparent(bw, bh);
    let drawing = &*source;
    out.data_mut()
        .par_chunks_exact_mut(4)
        .zip(&band)
        .enumerate()
        .for_each(|(i, (px, &a))| {
            let (x, y) = ((i % bw) as isize - g as isize, (i / bw) as isize - g as isize);
            let p = at(drawing, x, y);
            if p[3] > 0.0 {
                let k = a as f64 / p[3] as f64;
                for j in 0..3 {
                    px[j] = (p[j] as f64 * k) as f32;
                }
                px[3] = a;
                return;
            }
            if a == 0.0 {
                return;
            }
            let mut sum = [0.0f64; 4];
            for &(dy, hw) in &runs {
                let sy = y + dy;
                let (x0, x1) = ((x - hw).max(0), (x + hw + 1).min(w as isize));
                if sy < 0 || sy >= h as isize || x0 >= x1 {
                    continue;
                }
                let line = &sums[sy as usize * (w + 1) * 4..];
                for j in 0..4 {
                    sum[j] += line[x1 as usize * 4 + j] - line[x0 as usize * 4 + j];
                }
            }
            for j in 0..3 {
                px[j] = (sum[j] / sum[3] * a as f64) as f32;
            }
            px[3] = a;
        });
    *source = out;
    g
}


/// D-160: manga focus lines, wedges rushing in toward `center` (in `source`'s pixels), laid on
/// every covered pixel in `color` (linear) by the blend mix, normal. The numbers are `[count,
/// thickness, inner, inner_jitter, angle_jitter, seed, hold, opacity]`, already held; count, seed
/// and hold count by their whole parts. The lines are Noise's hash of the seed, each line's
/// number and `frame` over the hold. The settings are already valid.
pub(crate) fn speed_lines(
    source: &mut WorkingBuffer,
    center: (f64, f64),
    color: [f64; 3],
    [count, thickness, inner, inner_jitter, angle_jitter, seed, hold, opacity]: [f64; 8],
    frame: i32,
) {
    let lines = speed_line_list([count, thickness, inner, inner_jitter, angle_jitter, seed, hold], frame);
    let widest = lines.iter().map(|l| l.1).fold(0.0, f64::max);
    let (n, k, w) = (lines.len(), opacity / 100.0, source.width());
    let lines = &lines;
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            if px[3] == 0.0 {
                return;
            }
            let (vx, vy) = ((i % w) as f64 + 0.5 - center.0, (i / w) as f64 + 0.5 - center.1);
            let d = vx.hypot(vy);
            let alpha = vx.atan2(-vy).to_degrees().rem_euclid(360.0);
            let one = |&(theta, h, r): &(f64, f64, f64)| {
                let turn = (alpha - theta).rem_euclid(360.0);
                ((h - turn.min(360.0 - turn)) * std::f64::consts::PI / 180.0 * d + 0.5).clamp(0.0, 1.0)
                    * (d - r + 0.5).clamp(0.0, 1.0)
            };
            // A line more than `reach` degrees round gives nothing here, so each way's walk from
            // the pixel's own angle stops there.
            let reach = widest + 90.0 / (std::f64::consts::PI * d) + 1e-9;
            let start = lines.partition_point(|l| l.0 < alpha);
            let mut q = 0.0f64;
            for j in 0..n {
                let l = &lines[(start + j) % n];
                if (l.0 - alpha).rem_euclid(360.0) > reach {
                    break;
                }
                q = q.max(one(l));
            }
            for j in 1..=n {
                let l = &lines[(start + n - j) % n];
                if (alpha - l.0).rem_euclid(360.0) > reach {
                    break;
                }
                q = q.max(one(l));
            }
            let (op, a) = (q * k, px[3] as f64);
            for c in 0..3 {
                let v = px[c] as f64;
                px[c] = (v + op * (color[c] * a - v)) as f32;
            }
        });
}


/// D-160's lines at `frame`, each (angle, half-width, inner edge), in angle order, from the first
/// seven of [`speed_lines`]' numbers. B-107's card takes the same.
pub(crate) fn speed_line_list(
    [count, thickness, inner, inner_jitter, angle_jitter, seed, hold]: [f64; 7],
    frame: i32,
) -> Vec<(f64, f64, f64)> {
    let count = count.floor() as i64;
    let m = (frame as i64).div_euclid(hold.floor() as i64);
    let base = crate::grade::mix(seed.floor() as u64);
    let mut lines: Vec<(f64, f64, f64)> = (0..count)
        .map(|k| {
            let u = |ch| crate::grade::unit(base, k, 0, m, ch);
            (
                (360.0 / count as f64 * (k as f64 + 0.5 * angle_jitter / 100.0 * u(0))).rem_euclid(360.0),
                thickness / 2.0 * (1.0 + 0.5 * u(1)),
                inner * (1.0 + inner_jitter / 100.0 * u(2)),
            )
        })
        .collect();
    lines.sort_by(|a, b| a.0.total_cmp(&b.0));
    lines
}


/// D-161: the pixels D-89's bright test lights, streaked `floor(length)` pixels out along
/// `floor(points)` arms spread evenly round from `angle`, each step's light fading as
/// (1 - t / (L + 1))^2 over their sum, tinted by `color` (encoded 0 to 1) at `intensity` and
/// added over the drawing as Light Rays' is. The layer grows by the length on every side,
/// returned; length 0 or intensity 0 changes nothing. The settings are already valid.
pub(crate) fn cross_glare(
    source: &mut WorkingBuffer,
    threshold: f64,
    length: f64,
    points: f64,
    angle: f64,
    intensity: f64,
    color: [f64; 3],
) -> usize {
    let l = length.floor() as usize;
    if l == 0 || intensity == 0.0 {
        return 0;
    }
    let mut light = source.clone();
    light.data_mut().par_chunks_exact_mut(4).for_each(|px| {
        if !crate::bloom::bright(px, threshold) {
            px.fill(0.0);
        }
    });
    let n = points.floor();
    let arms: Vec<(f64, f64)> = (0..n as usize)
        .map(|j| crate::blurs::along(angle + 360.0 * j as f64 / n))
        .collect();
    let fade: Vec<f64> = (1..=l).map(|t| (1.0 - t as f64 / (l + 1) as f64).powi(2)).collect();
    let total: f64 = fade.iter().sum();
    let steps: Vec<(f64, f64)> = fade.iter().enumerate().map(|(t, f)| ((t + 1) as f64, f / total)).collect();
    // P-22: a sample whose four pixels are all dark adds exactly nothing, so it is skipped, and a
    // pixel with no lit pixel within reach skips its whole sum. `lit` counts the lit pixels above
    // and left of each place; `near[(y0 + 1)(sw + 1) + x0 + 1]` marks the places (x0, y0), from
    // -1, whose pixels x0..=x0 + 1, y0..=y0 + 1 hold any light.
    let (sw, sh) = (source.width(), source.height());
    let mut lit = vec![0u32; (sw + 1) * (sh + 1)];
    lit[sw + 1..].par_chunks_mut(sw + 1).enumerate().for_each(|(y, line)| {
        for x in 0..sw {
            line[x + 1] = line[x] + light.pixel(x, y).iter().any(|&v| v != 0.0) as u32;
        }
    });
    for y in 1..=sh {
        let (above, line) = lit[(y - 1) * (sw + 1)..(y + 1) * (sw + 1)].split_at_mut(sw + 1);
        for (v, a) in line.iter_mut().zip(above.iter()) {
            *v += a;
        }
    }
    // The lit pixels in columns x0..x1 and rows y0..y1, each clipped to the layer.
    let count = |x0: isize, x1: isize, y0: isize, y1: isize| {
        let [x0, x1] = [x0, x1].map(|v| v.clamp(0, sw as isize) as usize);
        let [y0, y1] = [y0, y1].map(|v| v.clamp(0, sh as isize) as usize);
        let at = |x: usize, y: usize| lit[y * (sw + 1) + x];
        at(x1, y1) + at(x0, y0) - at(x0, y1) - at(x1, y0)
    };
    let near: Vec<bool> = (0..(sw + 1) * (sh + 1))
        .into_par_iter()
        .map(|i| {
            let (x0, y0) = ((i % (sw + 1)) as isize - 1, (i / (sw + 1)) as isize - 1);
            count(x0, x0 + 2, y0, y0 + 2) > 0
        })
        .collect();
    let reach = l as isize + 1;
    let (w, h) = (sw + 2 * l, sh + 2 * l);
    let c = color.map(crate::grade::to_linear);
    let mut out = WorkingBuffer::transparent(w, h);
    let drawing = &*source;
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = ((i % w) as isize - l as isize, (i / w) as isize - l as isize);
            let (cx, cy) = (x as f64 + 0.5, y as f64 + 0.5);
            let mut g = [0.0f64; 4];
            let any = count(x - reach, x + reach + 1, y - reach, y + reach + 1) > 0;
            for &(vx, vy) in arms.iter().filter(|_| any) {
                for &(t, k) in &steps {
                    let (sx, sy) = (cx - t * vx, cy - t * vy);
                    // sample_bilinear's own corner.
                    let (x0, y0) = ((sx - 0.5).floor(), (sy - 0.5).floor());
                    if x0 < -1.0 || y0 < -1.0 || x0 >= sw as f64 || y0 >= sh as f64 {
                        continue;
                    }
                    if !near[(y0 as usize + 1) * (sw + 1) + x0 as usize + 1] {
                        continue;
                    }
                    let s = sample_bilinear(&light, sx, sy);
                    for ch in 0..4 {
                        g[ch] += k * s[ch] as f64;
                    }
                }
            }
            let o = at(drawing, x, y);
            for ch in 0..3 {
                px[ch] = (o[ch] as f64 + intensity * c[ch] * g[ch]) as f32;
            }
            px[3] = (o[3] as f64 + intensity * g[3]).min(1.0) as f32;
        });
    *source = out;
    l
}


/// D-162: the drawing moved by `amount` times Noise's hash and turned by `rotation` degrees times
/// it, about its centre, one jolt per `hold` frames of `frame`. The layer grows by `amount sqrt(2)`
/// plus the turned corners' swing on every side, returned; amount 0 and rotation 0 change
/// nothing. The settings are already valid.
pub(crate) fn camera_shake(
    source: &mut WorkingBuffer,
    [amount, rotation, hold, seed]: [f64; 4],
    frame: i32,
    origin: (usize, usize),
) -> usize {
    if amount == 0.0 && rotation == 0.0 {
        return 0;
    }
    let (w, h) = (source.width(), source.height());
    let ((cx, cy), g) = shake_reach(amount, rotation, (w, h), origin);
    let (dx, dy, sb, cb) = shake_jolt([amount, rotation, hold, seed], frame);
    let ow = w + 2 * g;
    let mut out = WorkingBuffer::transparent(ow, h + 2 * g);
    let drawing = &*source;
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let vx = (i % ow) as f64 + 0.5 - g as f64 - cx - dx;
            let vy = (i / ow) as f64 + 0.5 - g as f64 - cy - dy;
            px.copy_from_slice(&sample_bilinear(drawing, cx + vx * cb + vy * sb, cy - vx * sb + vy * cb));
        });
    *source = out;
    g
}


/// D-162's jolt at `frame`: the move across and down, and the turn's sine and cosine. B-107's
/// card takes the same.
pub(crate) fn shake_jolt([amount, rotation, hold, seed]: [f64; 4], frame: i32) -> (f64, f64, f64, f64) {
    let m = (frame as i64).div_euclid(hold.floor() as i64);
    let u = |ch| crate::grade::unit(crate::grade::mix(seed.floor() as u64), m, 0, 0, ch);
    let (sb, cb) = (rotation * u(2)).to_radians().sin_cos();
    (amount * u(0), amount * u(1), sb, cb)
}


/// D-162's centre, the drawing's own, and the growth, for a buffer `w` by `h` whose drawing's
/// corner is at `origin`. Compose and B-107's card take the same.
pub(crate) fn shake_reach(amount: f64, rotation: f64, (w, h): (usize, usize), origin: (usize, usize)) -> ((f64, f64), usize) {
    let (cx, cy) = (
        origin.0 as f64 + (w - 2 * origin.0) as f64 / 2.0,
        origin.1 as f64 + (h - 2 * origin.1) as f64 / 2.0,
    );
    let rho = [(0.0, 0.0), (w as f64, 0.0), (0.0, h as f64), (w as f64, h as f64)]
        .iter()
        .map(|&(x, y): &(f64, f64)| (x - cx).hypot(y - cy))
        .fold(0.0, f64::max);
    ((cx, cy), (amount * std::f64::consts::SQRT_2 + 2.0 * rho * (rotation.to_radians() / 2.0).sin()).ceil() as usize)
}


/// D-163: falling rain, a field of `spacing`-pixel cells turned along `direction` and slid
/// `speed` pixels a frame, each cell holding a streak by Noise's hash of the seed; the strongest
/// streak reaching a pixel mixes `color` (linear) in by the blend mix, normal, at the pixel's own
/// covering. The numbers are `[density, spacing, length, width, direction, speed, seed,
/// opacity]`, already held; the seed counts by its whole part. `origin` is the growth of an
/// earlier effect, so the field stays in the drawing's own space. The settings are already
/// valid.
pub(crate) fn rain(
    source: &mut WorkingBuffer,
    color: [f64; 3],
    [density, spacing, length, width, direction, speed, seed, opacity]: [f64; 8],
    frame: i32,
    origin: (usize, usize),
) {
    if density == 0.0 || opacity == 0.0 {
        return;
    }
    let (tx, ty) = crate::blurs::along(direction);
    let (nx, ny) = (-ty, tx);
    let base = crate::grade::mix(seed.floor() as u64);
    let (r, half, fall) = (width / 2.0 + 0.5, length / 2.0, speed * frame as f64);
    let w = source.width();
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let a = px[3] as f64;
            if a == 0.0 {
                return;
            }
            let (x, y) = ((i % w) as f64 - origin.0 as f64 + 0.5, (i / w) as f64 - origin.1 as f64 + 0.5);
            let (fa, fb) = (nx * x + ny * y, tx * x + ty * y - fall);
            let mut q = 0.0f64;
            for ci in ((fa - r) / spacing).floor() as i64..=((fa + r) / spacing).floor() as i64 {
                for cj in ((fb - half - r) / spacing).floor() as i64..=((fb + half + r) / spacing).floor() as i64 {
                    let u = |ch| crate::grade::unit(base, ci, cj, 0, ch);
                    if (u(0) + 1.0) / 2.0 >= density / 100.0 {
                        continue;
                    }
                    let cx = spacing * (ci as f64 + (u(1) + 1.0) / 2.0);
                    let cy = spacing * (cj as f64 + (u(2) + 1.0) / 2.0);
                    let beta = 0.5 + 0.25 * (u(3) + 1.0);
                    let delta = (fa - cx).hypot(((fb - cy).abs() - half).max(0.0));
                    q = q.max((width / 2.0 - delta + 0.5).clamp(0.0, 1.0) * beta);
                }
            }
            let op = q * opacity / 100.0;
            for ch in 0..3 {
                let b = px[ch] as f64 / a;
                px[ch] = ((b + op * (color[ch] - b)) * a) as f32;
            }
        });
}


/// D-186: kira-kira, one star on each chosen cell's near-white highlights, at their middle,
/// twinkling on its own beat; the strongest star's light at a pixel adds `color` (linear) at up
/// to `opacity`, and covers. The numbers are `[threshold, spacing, density, size, angle, twinkle,
/// period, seed, opacity]`, already held; the seed counts by its whole part; `star` adds the short
/// diagonal arms. `origin` is the growth of an earlier effect, so the cells stay in the drawing's
/// own space. The layer grows by the size rounded up, returned, unless size, density or opacity
/// is 0, which changes nothing. The settings are already valid.
/// D-204: soft round flakes in three planes, each a field of cells on the drawing's own space,
/// the far planes smaller, closer and slower by `depth`, falling, drifting and swaying; the
/// brightest flake at each pixel that shows mixes `color` in, as Rain's drops do. It grows
/// nothing, and a pixel no flake reaches is left exactly as it was.
// ponytail: every pixel searches the cells a flake could reach from, so a large size over a
// small spacing is dear; a splat pass per flake would be the upgrade if that is ever wanted.
pub(crate) fn snowfall(
    source: &mut WorkingBuffer,
    color: [f64; 3],
    [density, spacing, size, depth, speed, wind, wiggle, period, seed, opacity]: [f64; 10],
    frame: i32,
    origin: (usize, usize),
) {
    if density == 0.0 || size == 0.0 || opacity == 0.0 {
        return;
    }
    let base = crate::grade::mix(seed.floor() as u64);
    let f = frame as f64;
    // Each plane's cell, largest radius, sway, and how far it has drifted and fallen.
    let planes: Vec<[f64; 5]> = (0..3)
        .map(|l| {
            let z = 1.0 - depth / 100.0 * l as f64 / 3.0;
            [spacing * z, size * z / 2.0, wiggle * z, wind * z * f, speed * z * f]
        })
        .collect();
    let w = source.width();
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let a = px[3] as f64;
            if a == 0.0 {
                return;
            }
            let (x, y) = ((i % w) as f64 - origin.0 as f64 + 0.5, (i / w) as f64 - origin.1 as f64 + 0.5);
            let mut q = 0.0f64;
            for (l, &[s, rmax, wz, drift, fall]) in planes.iter().enumerate() {
                let (fa, fb, reach) = (x - drift, y - fall, rmax + 0.5);
                for ci in ((fa - reach - wz) / s).floor() as i64..=((fa + reach + wz) / s).floor() as i64 {
                    for cj in ((fb - reach) / s).floor() as i64..=((fb + reach) / s).floor() as i64 {
                        let u = |ch| (crate::grade::unit(base, ci, cj, l as i64, ch) + 1.0) / 2.0;
                        if u(0) >= density / 100.0 {
                            continue;
                        }
                        let sway = wz * (std::f64::consts::TAU * (f / period + u(4))).sin();
                        let (cx, cy) = (s * (ci as f64 + u(1)) + sway, s * (cj as f64 + u(2)));
                        let r = rmax * (0.5 + 0.5 * u(3));
                        let d = (fa - cx).hypot(fb - cy);
                        q = q.max((r + 0.5 - d).clamp(0.0, 1.0) * (2.0 * r).min(1.0));
                    }
                }
            }
            if q == 0.0 {
                return;
            }
            let op = q * opacity / 100.0;
            for ch in 0..3 {
                let b = px[ch] as f64 / a;
                px[ch] = ((b + op * (color[ch] - b)) * a) as f32;
            }
        });
}

pub(crate) fn kira_kira(
    source: &mut WorkingBuffer,
    color: [f64; 3],
    [threshold, spacing, density, size, angle, twinkle, period, seed, opacity]: [f64; 9],
    star: bool,
    frame: i32,
    origin: (usize, usize),
) -> usize {
    if size == 0.0 || density == 0.0 || opacity == 0.0 {
        return 0;
    }
    let g = size.ceil() as usize;
    let (sw, sh) = (source.width(), source.height());
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    let stars = kira_stars(source, [threshold, spacing, density, size, twinkle, period, seed], frame, origin);
    let arms = kira_arms(angle, star);
    let light = |&[cx, cy, r, tau]: &[f64; 4], px: f64, py: f64| {
        let (dx, dy, h) = (px - cx, py - cy, 0.5 + r / 32.0);
        let mut best = 0.0f64;
        for &((vx, vy), k) in &arms {
            let (a, b, l) = ((dx * vx + dy * vy).abs(), (dx * vy - dy * vx).abs(), r * k);
            if a < l && b < h {
                best = best.max((1.0 - a / l).powi(2) * (1.0 - b / h));
            }
        }
        let d = dx.hypot(dy);
        if d < r / 4.0 {
            best = best.max((1.0 - d / (r / 4.0)).powi(2));
        }
        tau * best
    };
    // A star lights nothing farther than R + h across or down; `most` is the largest that can be.
    // ponytail: every star near a row is splatted along it, so spacing 2 over a large white area
    // with a large size is slow; a coarser pass per cell block if that is ever wanted.
    let reach = |r: f64| r + 0.5 + r / 32.0;
    let most = reach(size);
    let (w, k) = (sw + 2 * g, opacity / 100.0);
    let (gx, gy) = (g as f64 + ox, g as f64 + oy);
    let mut out = WorkingBuffer::transparent(w, sh + 2 * g);
    let drawing = &*source;
    out.data_mut().par_chunks_exact_mut(4 * w).enumerate().for_each(|(y, line)| {
        let py = y as f64 - gy + 0.5;
        let mut lit = vec![0.0f64; w];
        let first = stars.partition_point(|s| s[1] <= py - most);
        for s in stars[first..].iter().take_while(|s| s[1] < py + most) {
            let e = reach(s[2]);
            if (py - s[1]).abs() >= e {
                continue;
            }
            let x0 = (s[0] + gx - 0.5 - e).floor().max(0.0) as usize;
            let x1 = ((s[0] + gx - 0.5 + e).ceil() + 1.0).clamp(0.0, w as f64) as usize;
            for x in x0..x1 {
                lit[x] = lit[x].max(light(s, x as f64 - gx + 0.5, py));
            }
        }
        for (x, px) in line.chunks_exact_mut(4).enumerate() {
            let o = at(drawing, x as isize - g as isize, y as isize - g as isize);
            let s = k * lit[x];
            for ch in 0..3 {
                px[ch] = (o[ch] as f64 + s * color[ch]) as f32;
            }
            px[3] = (o[3] as f64 + s * (1.0 - o[3] as f64)) as f32;
        }
    });
    *source = out;
    g
}

/// D-186's arms: each one's way and its length as a share of the star's, the long cross and,
/// for a `star`, the short diagonals. B-123's card takes the same.
pub(crate) fn kira_arms(angle: f64, star: bool) -> Vec<((f64, f64), f64)> {
    [(0.0, 1.0), (90.0, 1.0), (45.0, 0.5), (135.0, 0.5)][..if star { 4 } else { 2 }]
        .iter()
        .map(|&(t, k)| (crate::blurs::along(angle + t), k))
        .collect()
}

/// D-186's stars as `[cx, cy, R, tau]` in the drawing's own space, by their centre's row. The
/// numbers are `[threshold, spacing, density, size, twinkle, period, seed]`. B-123's card takes
/// the same.
pub(crate) fn kira_stars(
    source: &WorkingBuffer,
    [threshold, spacing, density, size, twinkle, period, seed]: [f64; 7],
    frame: i32,
    origin: (usize, usize),
) -> Vec<[f64; 4]> {
    let sw = source.width();
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    // Each cell holding a highlight: the sums of its highlights' centres, and their count. The
    // centres are whole numbers and halves, so the sums are exact in any order.
    type Cells = std::collections::HashMap<(i64, i64), (f64, f64, f64)>;
    let cells = source
        .data()
        .par_chunks_exact(4)
        .enumerate()
        .filter(|(_, px)| crate::bloom::white(px, threshold))
        .fold(Cells::new, |mut m, (i, _)| {
            let (x, y) = ((i % sw) as f64 - ox + 0.5, (i / sw) as f64 - oy + 0.5);
            let e = m.entry(((x / spacing).floor() as i64, (y / spacing).floor() as i64)).or_default();
            *e = (e.0 + x, e.1 + y, e.2 + 1.0);
            m
        })
        .reduce(Cells::new, |mut a, b| {
            for (k, (x, y, n)) in b {
                let e = a.entry(k).or_default();
                *e = (e.0 + x, e.1 + y, e.2 + n);
            }
            a
        });
    // Each star as [cx, cy, R, tau], by its centre's row.
    let base = crate::grade::mix(seed.floor() as u64);
    let mut stars: Vec<[f64; 4]> = cells
        .into_iter()
        .filter_map(|((i, j), (x, y, n))| {
            let u = |ch| crate::grade::unit(base, i, j, 0, ch);
            if (u(0) + 1.0) / 2.0 >= density / 100.0 {
                return None;
            }
            let (phi, beta) = ((u(1) + 1.0) / 2.0, 0.6 + 0.2 * (u(2) + 1.0));
            let beat = 0.5 + 0.5 * (2.0 * std::f64::consts::PI * (frame as f64 / period + phi)).cos();
            let tau = 1.0 - twinkle / 100.0 * (1.0 - beat);
            let r = size * beta * tau;
            (r > 0.0).then_some([x / n, y / n, r, tau])
        })
        .collect();
    stars.sort_by(|a, b| a[1].total_cmp(&b[1]));
    stars
}

/// D-190: lightning bolt, from `ends[0]` to `ends[1]` in `source`'s own pixels. The numbers are
/// `[jagged, detail, branches, width, glow, opacity, hold, seed]`, already held; detail, hold and
/// seed count by their whole parts. Each pixel takes the largest core and glow of any segment;
/// the core adds `colours[0]` (linear) and the glow `colours[1]` round it, at up to `opacity`, and
/// both cover. Nothing grows. The settings are already valid.
pub(crate) fn lightning_bolt(
    source: &mut WorkingBuffer,
    ends: [(f64, f64); 2],
    [jagged, detail, branches, width, glow, opacity, hold, seed]: [f64; 8],
    [core, halo]: [[f64; 3]; 2],
    frame: i32,
) {
    if opacity == 0.0 || (width == 0.0 && glow == 0.0) {
        return;
    }
    let m = (frame as i64).div_euclid(hold.floor() as i64);
    let base = crate::grade::mix(seed.floor() as u64);
    let segs = bolt_segments(ends, [jagged, branches], detail.floor() as u32, base, m);
    // Each segment's box: nothing past its reach from the line is lit.
    let boxes: Vec<[f64; 4]> = segs
        .iter()
        .map(|s| {
            let w = s[4].max(s[5]);
            let e = (w * width / 2.0 + 0.5).max(w * glow);
            [s[0].min(s[2]) - e, s[0].max(s[2]) + e, s[1].min(s[3]) - e, s[1].max(s[3]) + e]
        })
        .collect();
    let (w, k) = (source.width(), opacity / 100.0);
    // ponytail: every segment is tested against every row, and a wide glow lights a wide box per
    // segment, so detail 8 with branches 100 and glow 500 is slow; bin segments by row if needed.
    source.data_mut().par_chunks_exact_mut(4 * w).enumerate().for_each(|(y, line)| {
        let py = y as f64 + 0.5;
        let mut lit = vec![(0.0f64, 0.0f64); w];
        for (s, b) in segs.iter().zip(&boxes) {
            if py <= b[2] || py >= b[3] {
                continue;
            }
            let x0 = (b[0] - 0.5).floor().max(0.0) as usize;
            let x1 = ((b[1] - 0.5).ceil() + 1.0).clamp(0.0, w as f64) as usize;
            for (x, l) in lit.iter_mut().enumerate().take(x1).skip(x0) {
                let (c, g) = bolt_light(s, width, glow, x as f64 + 0.5, py);
                *l = (l.0.max(c), l.1.max(g));
            }
        }
        for (px, &(c, g)) in line.chunks_exact_mut(4).zip(&lit) {
            if c == 0.0 && g == 0.0 {
                continue;
            }
            for ch in 0..3 {
                px[ch] = (px[ch] as f64 + k * (c * core[ch] + (1.0 - c) * g * halo[ch])) as f32;
            }
            px[3] = (px[3] as f64 + k * (c + (1.0 - c) * g) * (1.0 - px[3] as f64)) as f32;
        }
    });
}

/// D-190's segment `[px, py, qx, qy, wP, wQ]`: its core and glow at the point (x, y).
fn bolt_light(s: &[f64; 6], width: f64, glow: f64, x: f64, y: f64) -> (f64, f64) {
    let (dx, dy) = (s[2] - s[0], s[3] - s[1]);
    let l2 = dx * dx + dy * dy;
    let t = if l2 == 0.0 { 0.0 } else { (((x - s[0]) * dx + (y - s[1]) * dy) / l2).clamp(0.0, 1.0) };
    let d = (x - s[0] - t * dx).hypot(y - s[1] - t * dy);
    let w = s[4] + t * (s[5] - s[4]);
    let half = w * width / 2.0;
    let core = ((d + 0.5).min(half) - (d - 0.5).max(-half)).clamp(0.0, 1.0);
    let r = w * glow;
    (core, if d < r { w * (1.0 - d / r).powi(2) } else { 0.0 })
}

/// D-190's bolt as segments `[px, py, qx, qy, wP, wQ]`: the line from `ends[0]` to `ends[1]`
/// halved `detail` times, each middle pushed aside by Noise's hash of the seed's `base`, the
/// segment's key and bolt, and `m`, with forks three deep. Worked in double precision.
pub(crate) fn bolt_segments(
    ends: [(f64, f64); 2],
    [jagged, branches]: [f64; 2],
    detail: u32,
    base: u64,
    m: i64,
) -> Vec<[f64; 6]> {
    // (P, Q, wP, wQ, depth, bolt, key): a key and a bolt name a segment for good, so a fork or a
    // finer halving never moves what is already drawn.
    let mut segs = vec![(ends[0], ends[1], 1.0, 1.0, 0u32, 0i64, 1i64)];
    for _ in 0..detail {
        let mut out = Vec::with_capacity(segs.len() * 3);
        for (p, q, wp, wq, depth, b, key) in segs {
            let (dx, dy) = (q.0 - p.0, q.1 - p.1);
            let l = dx.hypot(dy);
            if l == 0.0 {
                out.push((p, q, wp, wq, depth, b, key));
                continue;
            }
            let r = |j| crate::grade::unit(base, key, b, m, j);
            let a = r(0) * jagged / 100.0 * l / 2.0;
            let mid = ((p.0 + q.0) / 2.0 - dy / l * a, (p.1 + q.1) / 2.0 + dx / l * a);
            let wm = (wp + wq) / 2.0;
            out.push((p, mid, wp, wm, depth, b, 2 * key));
            out.push((mid, q, wm, wq, depth, b, 2 * key + 1));
            if depth < 3 && (r(1) + 1.0) / 2.0 < branches / 100.0 {
                let turn = ((15.0 + 15.0 * (r(2) + 1.0)) * if r(3) < 0.0 { -1.0 } else { 1.0 }).to_radians();
                let (vx, vy) = (dx / l, dy / l);
                let v = (vx * turn.cos() - vy * turn.sin(), vx * turn.sin() + vy * turn.cos());
                let lb = l * (0.3 + 0.15 * (r(4) + 1.0));
                out.push((mid, (mid.0 + v.0 * lb, mid.1 + v.1 * lb), wm / 2.0, 0.0, depth + 1, 1024 * b + key, 1));
            }
        }
        segs = out;
    }
    segs.into_iter().map(|(p, q, wp, wq, ..)| [p.0, p.1, q.0, q.1, wp, wq]).collect()
}

/// D-191: Compound Blur. Each pixel is blurred by `max_blur / 3` times the brightness of the map
/// under it, the map lying on the layer's own picture, which sits at `origin` in `source`; with
/// `invert`, by one less that brightness. Six levels, 0 and a sixteenth, an eighth, a quarter, a
/// half and all of the largest, are each one Gaussian of the whole picture, and a pixel between
/// two is their mix. `repeat` holds the edge pixels, as Gaussian Blur's edges do.
pub(crate) fn compound_blur(
    source: &mut WorkingBuffer,
    map: &WorkingBuffer,
    origin: (usize, usize),
    max_blur: f64,
    invert: bool,
    repeat: bool,
) {
    let big = max_blur / 3.0;
    let levels = [0.0, big / 16.0, big / 8.0, big / 4.0, big / 2.0, big];
    let (w, h) = (source.width(), source.height());
    let (mw, mh) = (map.width(), map.height());
    let sigmas: Vec<f64> = (0..w * h)
        .into_par_iter()
        .map(|i| {
            let (x, y) = ((i % w).wrapping_sub(origin.0), (i / w).wrapping_sub(origin.1));
            let v = if x < mw && y < mh {
                let p = &map.data()[(y * mw + x) * 4..][..3];
                let luma = 0.2126 * p[0] as f64 + 0.7152 * p[1] as f64 + 0.0722 * p[2] as f64;
                crate::grade::to_srgb(luma.clamp(0.0, 1.0))
            } else {
                0.0
            };
            (if invert { 1.0 - v } else { v }) * big
        })
        .collect();
    let segment = |s: f64| (0..5).rev().find(|&k| levels[k] <= s).unwrap_or(0);
    let top = sigmas.iter().copied().fold(0.0, f64::max);
    if top <= 0.0 {
        return;
    }
    let blurred = |sigma: f64| {
        let mut b = source.clone();
        if repeat {
            crate::effects::held_blur(&mut b, sigma);
        } else {
            let r = blur(&mut b, sigma);
            if r > 0 {
                b = crate::layer_map::cut(&b, (r, r), (w, h));
            }
        }
        b
    };
    let mut out = source.clone();
    let mut low = source.clone();
    for k in 0..=segment(top) {
        let (lo, hi) = (levels[k], levels[k + 1]);
        let high = blurred(hi);
        out.data_mut().par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
            for (x, o) in row.chunks_mut(4).enumerate() {
                let i = y * w + x;
                let s = sigmas[i];
                if segment(s) == k {
                    let t = (s - lo) / (hi - lo);
                    for c in 0..4 {
                        let (a, b) = (low.data()[i * 4 + c] as f64, high.data()[i * 4 + c] as f64);
                        o[c] = (a + t * (b - a)) as f32;
                    }
                }
            }
        });
        low = high;
    }
    *source = out;
}

/// D-193: Displacement Map. Each pixel takes the picture's colour from a place moved across and
/// down by the map under it, the map lying on the layer's own picture, which sits at `origin` in
/// `source`. `words` name the map's channels for across and down, and `most` how far each moves
/// where that channel is white; black moves as far the other way, and mid grey not at all.
/// Outside the picture is clear, or with `wrap`, the picture's far side.
pub(crate) fn displacement_map(
    source: &mut WorkingBuffer,
    map: &WorkingBuffer,
    origin: (usize, usize),
    words: [&str; 2],
    most: [f64; 2],
    wrap: bool,
) {
    let value = |word: &str, m: [f32; 4]| {
        let a = m[3] as f64;
        match word {
            "full" => return 1.0,
            "off" => return 0.5,
            "alpha" => return a,
            _ if a <= 0.0 => return 0.5,
            _ => {}
        }
        let straight = [0, 1, 2].map(|c| (m[c] as f64 / a).clamp(0.0, 1.0));
        let c = straight.map(crate::grade::to_srgb);
        let k = match word {
            "red" => c[0],
            "green" => c[1],
            "blue" => c[2],
            "luminance" => crate::grade::to_srgb(0.2126 * straight[0] + 0.7152 * straight[1] + 0.0722 * straight[2]),
            "hue" => crate::grade::to_hsl(c)[0] / 360.0,
            "saturation" => crate::grade::to_hsl(c)[1],
            _ => crate::grade::to_hsl(c)[2],
        };
        0.5 + a * (k - 0.5)
    };
    let from = source.clone();
    let (w, h) = (from.width(), from.height());
    let (mw, mh) = (map.width(), map.height());
    source.data_mut().par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
        for (x, o) in row.chunks_mut(4).enumerate() {
            let (mx, my) = (x.wrapping_sub(origin.0), y.wrapping_sub(origin.1));
            let m = if mx < mw && my < mh { map.pixel(mx, my) } else { [0.0; 4] };
            let sx = x as f64 + 0.5 + (2.0 * value(words[0], m) - 1.0) * most[0];
            let sy = y as f64 + 0.5 + (2.0 * value(words[1], m) - 1.0) * most[1];
            let p = if wrap { wrapped(&from, (w, h), sx, sy) } else { sample_bilinear(&from, sx, sy) };
            o.copy_from_slice(&p);
        }
    });
}

/// Document 21's bilinear read, the four pixels' places taken round the picture's edges.
fn wrapped(src: &WorkingBuffer, (w, h): (usize, usize), x: f64, y: f64) -> [f32; 4] {
    let (fx, fy) = (x - 0.5, y - 0.5);
    let (x0, y0) = (fx.floor(), fy.floor());
    let (ux, uy) = (fx - x0, fy - y0);
    let mut out = [0.0f32; 4];
    for (dy, wy) in [(0.0, 1.0 - uy), (1.0, uy)] {
        for (dx, wx) in [(0.0, 1.0 - ux), (1.0, ux)] {
            let px = src.pixel((x0 + dx).rem_euclid(w as f64) as usize, (y0 + dy).rem_euclid(h as f64) as usize);
            for i in 0..4 {
                out[i] += px[i] * (wx * wy) as f32;
            }
        }
    }
    out
}

/// D-194: Gradient Wipe. Each pixel is kept by how far the picture luma of the map under it lies
/// above the edge `completion` has reached, over a band `softness` wide; the map lies on the
/// layer's own picture, which sits at `origin` in `source`, and reads black outside it.
pub(crate) fn gradient_wipe(
    source: &mut WorkingBuffer,
    map: &WorkingBuffer,
    origin: (usize, usize),
    completion: f64,
    softness: f64,
    invert: bool,
) {
    if completion == 0.0 {
        return;
    }
    if completion == 100.0 {
        source.data_mut().fill(0.0);
        return;
    }
    let (c, s) = (completion / 100.0, softness / 100.0);
    let edge = -s / 2.0 + c * (1.0 + s);
    let w = source.width();
    let (mw, mh) = (map.width(), map.height());
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w).wrapping_sub(origin.0), (i / w).wrapping_sub(origin.1));
        let v = if x < mw && y < mh { picture_luma(map.pixel(x, y)) } else { 0.0 };
        let v = if invert { 1.0 - v } else { v };
        let k = if s > 0.0 {
            ((v - edge) / s + 0.5).clamp(0.0, 1.0)
        } else if v >= edge {
            1.0
        } else {
            0.0
        };
        for p in px.iter_mut() {
            *p = (*p as f64 * k) as f32;
        }
    });
}

/// D-195: Echo's operators, as the file writes them.
pub(crate) const ECHO_OPERATORS: [&str; 7] =
    ["add", "maximum", "minimum", "screen", "composite_in_back", "composite_in_front", "blend"];

/// D-195: Echo's copies put together one at a time, so no more than one is held: each `P_k` in
/// turn with its weight `I D^k`, then [`EchoFold::finish`].
pub(crate) struct EchoFold {
    operator: String,
    out: WorkingBuffer,
    count: usize,
}

impl EchoFold {
    pub(crate) fn new(operator: &str, (w, h): (usize, usize)) -> Self {
        let mut out = WorkingBuffer::transparent(w, h);
        // Screen keeps what each copy leaves, which starts whole.
        if operator == "screen" {
            out.data_mut().fill(1.0);
        }
        EchoFold { operator: operator.to_string(), out, count: 0 }
    }

    /// `p` is `P_k`, `None` where the layer shows nothing; it is the size the fold was made at.
    pub(crate) fn add(&mut self, p: Option<&WorkingBuffer>, weight: f64) {
        let first = self.count == 0;
        self.count += 1;
        // Nothing added, laid over or under changes nothing; only Maximum and Minimum see it.
        if p.is_none() && !matches!(self.operator.as_str(), "maximum" | "minimum") {
            return;
        }
        let fold: fn(&mut [f32], [f32; 4], bool) = match self.operator.as_str() {
            "maximum" => |r, q, first| (0..4).for_each(|c| r[c] = if first { q[c] } else { r[c].max(q[c]) }),
            "minimum" => |r, q, first| (0..4).for_each(|c| r[c] = if first { q[c] } else { r[c].min(q[c]) }),
            "screen" => |r, q, _| (0..4).for_each(|c| r[c] *= 1.0 - q[c].clamp(0.0, 1.0)),
            // What is there so far over the new copy, or the new copy over it.
            "composite_in_back" => |r, q, _| {
                let a = r[3];
                (0..4).for_each(|c| r[c] += (1.0 - a) * q[c])
            },
            "composite_in_front" => |r, q, _| (0..4).for_each(|c| r[c] = q[c] + (1.0 - q[3]) * r[c]),
            _ => |r, q, _| (0..4).for_each(|c| r[c] += q[c]),
        };
        let w = weight as f32;
        let src = p.map(|p| p.data());
        self.out.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, r)| {
            let q = src.map_or([0.0; 4], |s| std::array::from_fn(|c| s[i * 4 + c] * w));
            fold(r, q, first);
        });
    }

    pub(crate) fn finish(mut self) -> WorkingBuffer {
        let n = self.count.max(1) as f32;
        let last: Option<fn(&mut f32, f32)> = match self.operator.as_str() {
            "add" => Some(|v, _| *v = v.min(1.0)),
            "screen" => Some(|v, _| *v = 1.0 - *v),
            "blend" => Some(|v, n| *v /= n),
            _ => None,
        };
        if let Some(last) = last {
            self.out.data_mut().par_iter_mut().for_each(|v| last(v, n));
        }
        self.out
    }
}

/// D-195: `picture` laid on `source` with its corner at `origin`, and nothing elsewhere.
pub(crate) fn lay(source: &mut WorkingBuffer, picture: &WorkingBuffer, (ox, oy): (usize, usize)) {
    let w = source.width();
    let across = picture.width().min(w.saturating_sub(ox));
    let rows = picture.height().min(source.height().saturating_sub(oy));
    let data = source.data_mut();
    data.fill(0.0);
    for y in 0..rows {
        let to = ((oy + y) * w + ox) * 4;
        data[to..to + across * 4].copy_from_slice(&picture.data()[y * picture.width() * 4..][..across * 4]);
    }
}

/// D-199's settings, read once for a frame: the colour encoded 0 to 1, the intensities 0 to 100.
pub(crate) struct LightSweep<'a> {
    pub center: [f64; 2],
    pub direction: f64,
    pub shape: &'a str,
    pub width: f64,
    pub sweep: f64,
    pub edge: f64,
    pub thickness: f64,
    pub color: [f64; 3],
    pub reception: &'a str,
}

/// D-199: a band of light laid across the drawing, whose corner is at `origin` in the buffer,
/// along the line through `center` at `direction`, fading to its sides by `shape`; the drawing's
/// outer edges, `thickness` whole pixels deep, take more, and only what is drawn is lit. The
/// layer does not grow. The settings are already valid.
pub(crate) fn light_sweep(source: &mut WorkingBuffer, s: &LightSweep, origin: (usize, usize)) {
    let cutout = s.reception == "cutout";
    let r = s.width / 2.0;
    if !cutout && (r == 0.0 || s.sweep == 0.0 && s.edge == 0.0) {
        return;
    }
    let w = source.width();
    let (dw, dh) = ((w - 2 * origin.0) as f64, (source.height() - 2 * origin.1) as f64);
    let (cx, cy) = (origin.0 as f64 + s.center[0] / 100.0 * dw, origin.1 as f64 + s.center[1] / 100.0 * dh);
    let (nx, ny) = crate::blurs::along(s.direction + 90.0);
    let least = if s.edge > 0.0 && r > 0.0 {
        least_covering(source, s.thickness.floor() as usize)
    } else {
        Vec::new()
    };
    let light = s.color.map(crate::grade::to_linear);
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let a = px[3] as f64;
            if a <= 0.0 {
                return;
            }
            let d = (nx * ((i % w) as f64 + 0.5 - cx) + ny * ((i / w) as f64 + 0.5 - cy)).abs();
            let t = d / r;
            let p = match s.shape {
                _ if r == 0.0 => 0.0,
                "linear" => (1.0 - t).max(0.0),
                "smooth" if t < 1.0 => 1.0 - t * t * (3.0 - 2.0 * t),
                "smooth" => 0.0,
                _ => (r + 0.5 - d).clamp(0.0, 1.0),
            };
            let edge = if p > 0.0 && !least.is_empty() { s.edge / 100.0 * (a - least[i] as f64) / a } else { 0.0 };
            let l = p * (s.sweep / 100.0 + edge);
            if l == 0.0 && !cutout {
                return;
            }
            let m = l.min(1.0);
            match s.reception {
                "add" => (0..3).for_each(|j| px[j] = (px[j] as f64 + l * a * light[j]) as f32),
                "composite" => (0..3).for_each(|j| px[j] = (px[j] as f64 + m * (a * light[j] - px[j] as f64)) as f32),
                _ => {
                    (0..3).for_each(|j| px[j] = (m * a * light[j]) as f32);
                    px[3] = (m * a) as f32;
                }
            }
        });
}

/// The least covering in the square `2k + 1` on a side round each pixel, outside the buffer
/// counting as uncovered, so a square that reaches past it is 0: along the rows, then down.
// ponytail: each pass is 2k + 1 reads a pixel, fine to the 50-pixel cap; van Herk / Gil-Werman
// makes it three whatever k if thick edges on big frames get slow.
fn least_covering(source: &WorkingBuffer, k: usize) -> Vec<f32> {
    let (w, h) = (source.width(), source.height());
    let alpha: Vec<f32> = source.data().chunks_exact(4).map(|p| p[3]).collect();
    let mut rows = vec![0.0f32; w * h];
    rows.par_chunks_mut(w).enumerate().for_each(|(y, line)| {
        let src = &alpha[y * w..(y + 1) * w];
        for x in k..w.saturating_sub(k) {
            line[x] = src[x - k..=x + k].iter().copied().fold(f32::INFINITY, f32::min);
        }
    });
    let mut out = vec![0.0f32; w * h];
    out.par_chunks_mut(w).enumerate().for_each(|(y, line)| {
        if y < k || y + k >= h {
            return;
        }
        line.copy_from_slice(&rows[(y - k) * w..(y - k + 1) * w]);
        for v in y - k + 1..=y + k {
            for (o, r) in line.iter_mut().zip(&rows[v * w..(v + 1) * w]) {
                *o = o.min(*r);
            }
        }
    });
    out
}

/// D-200's settings, held for a frame: the producer already in the buffer's pixels and the
/// colour linear.
pub(crate) struct RadioWaves<'a> {
    pub producer: (f64, f64),
    pub sides: f64,
    pub interval: f64,
    pub expansion: f64,
    pub orientation: f64,
    pub direction: f64,
    pub velocity: f64,
    pub spin: f64,
    pub lifespan: f64,
    pub opacity: f64,
    pub fade_in: f64,
    pub fade_out: f64,
    pub widths: [f64; 2],
    pub profile: &'a str,
    pub color: [f64; 3],
    pub frame: i32,
}

/// D-200: the waves alive at the frame, each a regular polygon sent from the producer every
/// `interval` frames, painted over the layer by `T O + (1 - T) (K, 1)`. The layer does not
/// grow. The settings are already valid.
pub(crate) fn radio_waves(source: &mut WorkingBuffer, s: &RadioWaves) {
    let fade = |t: f64, time: f64| if time > 0.0 { (t / time).min(1.0) } else { 1.0 };
    let (ux, uy) = crate::blurs::along(s.direction);
    let f = s.frame as f64;
    // (centre x, centre y, corner radius, first corner, half width, strength), oldest first.
    let mut waves = Vec::new();
    let mut k = ((f - s.lifespan) / s.interval).floor().max(0.0);
    while k * s.interval <= f {
        let t = f - k * s.interval;
        k += 1.0;
        if t >= s.lifespan {
            continue;
        }
        let w = s.widths[0] + (s.widths[1] - s.widths[0]) * t / s.lifespan;
        let g = s.opacity / 100.0 * fade(t, s.fade_in) * fade(s.lifespan - t, s.fade_out);
        if w == 0.0 || g == 0.0 {
            continue;
        }
        let d = s.velocity * t;
        waves.push((
            s.producer.0 + d * ux,
            s.producer.1 + d * uy,
            s.expansion * t,
            s.orientation + s.spin * t,
            w / 2.0,
            g,
        ));
    }
    if waves.is_empty() {
        return;
    }
    let n = s.sides.floor();
    let a = 360.0 / n;
    let apothem = (std::f64::consts::PI / n).cos();
    let w = source.width();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
        let mut t = 1.0;
        for &(cx, cy, r, theta, h, g) in &waves {
            let (qx, qy) = (x - cx, y - cy);
            let q = qx.hypot(qy);
            // A profile reaches h + 0.5 from the outline at most; the outline runs between the
            // edges' middles, r cos(180 / n) out, and the corners, r out.
            if q < r * apothem - h - 0.5 || q * apothem > r * apothem + h + 0.5 {
                continue;
            }
            let phi = qx.atan2(-qy).to_degrees();
            let delta = phi - theta - a * (((phi - theta) / a).floor() + 0.5);
            let d = (q * delta.to_radians().cos() - r * apothem).abs();
            let p = match s.profile {
                "square" => ((d + 0.5).min(h) - (d - 0.5).max(-h)).clamp(0.0, 1.0),
                "triangle" => (1.0 - d / h).max(0.0),
                _ if d < h => (90.0 * d / h).to_radians().cos(),
                _ => 0.0,
            };
            t *= 1.0 - g * p;
        }
        if t == 1.0 {
            return;
        }
        for j in 0..3 {
            px[j] = (t * px[j] as f64 + (1.0 - t) * s.color[j]) as f32;
        }
        px[3] = (t * px[3] as f64 + 1.0 - t) as f32;
    });
}
