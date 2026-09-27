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
    let data = source.data_mut();
    // ponytail: one pass on one thread, each pixel read and written once; split the rows of
    // blocks across threads if a profile shows it.
    for ys in &rows {
        for xs in &columns {
            let mut sum = [0.0f64; 4];
            for y in ys.clone() {
                for x in xs.clone() {
                    let i = 4 * (y * w + x);
                    for c in 0..4 {
                        sum[c] += data[i + c] as f64;
                    }
                }
            }
            let n = (ys.len() * xs.len()) as f64;
            let mean = sum.map(|v| (v / n) as f32);
            for y in ys.clone() {
                for x in xs.clone() {
                    let i = 4 * (y * w + x);
                    data[i..i + 4].copy_from_slice(&mean);
                }
            }
        }
    }
}
