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

/// D-211: Radial Shadow's light in the buffer, how much the cast is scaled up about it, and how
/// far the cast grows across and down before its blur. B-151: the card casts the same way.
pub(crate) fn radial_cast(
    light: [f64; 2],
    distance: f64,
    (w, h): (usize, usize),
    origin: (usize, usize),
) -> (f64, f64, f64, usize, usize) {
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    let (dw, dh) = (w as f64 - 2.0 * ox, h as f64 - 2.0 * oy);
    let (lx, ly) = (light[0] / 100.0 * dw + ox, light[1] / 100.0 * dh + oy);
    let k = 1.0 + distance / 100.0;
    let gx = ((distance / 100.0 * lx.max(w as f64 - lx)).ceil() as usize).min(w);
    let gy = ((distance / 100.0 * ly.max(h as f64 - ly)).ceil() as usize).min(h);
    (lx, ly, k, gx, gy)
}

/// D-211: the drawing whose corner is at `origin` cast from the light at `light`, per cent of the
/// drawing's own size, onto a wall `distance` behind, so scaled up about the light by
/// 1 + distance / 100, then blurred by `softness` / 3 and laid behind the drawing: in `color`,
/// or with `glass` the drawing's own colours mixed with it by `influence`. With `shadow_only`
/// the drawing is left out. The layer grows by the scaled drawing, never more than its own size
/// on each side, and the blur's reach; the growth across and down is returned.
#[allow(clippy::too_many_arguments)]
pub(crate) fn radial_shadow(
    source: &mut WorkingBuffer,
    color: [f64; 3],
    opacity: f64,
    light: [f64; 2],
    distance: f64,
    softness: f64,
    glass: bool,
    influence: f64,
    shadow_only: bool,
    origin: (usize, usize),
) -> (usize, usize) {
    let (w, h) = (source.width(), source.height());
    let (lx, ly, k, gx, gy) = radial_cast(light, distance, (w, h), origin);
    let cw = w + 2 * gx;
    let mut cast = WorkingBuffer::transparent(cw, h + 2 * gy);
    let drawing = &*source;
    cast.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let x = (i % cw) as f64 - gx as f64 + 0.5;
            let y = (i / cw) as f64 - gy as f64 + 0.5;
            px.copy_from_slice(&sample_bilinear(drawing, lx + (x - lx) / k, ly + (y - ly) / k));
        });
    let r = blur(&mut cast, softness / 3.0);
    let (ex, ey) = (gx + r, gy + r);
    let (op, f, c) = (opacity / 100.0, influence / 100.0, color.map(crate::grade::to_linear));
    let ow = cast.width();
    let mut out = WorkingBuffer::transparent(ow, cast.height());
    let shade = &cast;
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let s = &shade.data()[i * 4..i * 4 + 4];
            let a = s[3] as f64 * op;
            let mut shadow = [0.0; 4];
            for j in 0..3 {
                shadow[j] = if glass {
                    ((1.0 - f) * c[j] * s[3] as f64 + f * s[j] as f64) * op
                } else {
                    c[j] * a
                };
            }
            shadow[3] = a;
            let d = if shadow_only {
                [0.0; 4]
            } else {
                at(drawing, (i % ow) as isize - ex as isize, (i / ow) as isize - ey as isize)
            };
            for j in 0..4 {
                px[j] = (d[j] as f64 + shadow[j] * (1.0 - d[3] as f64)) as f32;
            }
        });
    *source = out;
    (ex, ey)
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
    let (src, sums, empty) = lens_rows(source, iris);
    let src = &src[..];
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

/// D-116 and D-121's steps before the gather: the pixels, each lit by the highlight when the
/// gain is above 0; each row's running totals, `sums[y][x]` the sum of its first x pixels; and
/// which rows have nothing on them (P-21: such a row adds only +0 to sums that started at +0).
#[allow(clippy::type_complexity)]
fn lens_rows<'a>(source: &'a WorkingBuffer, iris: &Iris) -> (std::borrow::Cow<'a, [f32]>, Vec<Vec<[f64; 4]>>, Vec<bool>) {
    let w = source.width();
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
        std::borrow::Cow::Owned(copy)
    } else {
        std::borrow::Cow::Borrowed(source.data())
    };
    let sums: Vec<Vec<[f64; 4]>> = src
        .par_chunks(w.max(1) * 4)
        .map(|row| {
            let mut s = vec![[0.0; 4]; w + 1];
            for (x, px) in row.chunks_exact(4).enumerate() {
                for i in 0..4 {
                    s[x + 1][i] = s[x][i] + px[i] as f64;
                }
            }
            s
        })
        .collect();
    let empty: Vec<bool> = src.par_chunks(w.max(1) * 4).map(|row| row.iter().all(|&v| v == 0.0)).collect();
    (src, sums, empty)
}

/// D-359: how Lens Blur reads its blur map: by its covering rather than its luminance, the
/// value kept sharp (`focal_distance` / 255), and whether the map is turned over first.
pub(crate) struct LensMap {
    pub alpha: bool,
    pub focus: f64,
    pub invert: bool,
}

/// D-359: the blur map's levels, level j the iris at radius `radius` (j / J) for j = 0 to J,
/// J = max(1, ceil(radius)); each level's runs as [`iris_runs`] gives them.
pub(crate) fn lens_levels(radius: f64, iris: &Iris) -> Vec<Vec<[isize; 3]>> {
    let big = (radius.ceil() as usize).max(1);
    (0..=big)
        .map(|j| iris_runs(radius * (j as f64 / big as f64), iris.blades, iris.roundness, iris.rotation, iris.aspect))
        .collect()
}

/// D-359: the blur map's reading at a pixel, s = d J of `tools/lens_blur_map_reference.py`'s
/// steps 2 to 4, `p` the map's premultiplied pixel and `big` J.
pub(crate) fn lens_level(p: &[f32], read: &LensMap, big: usize) -> f64 {
    let v = if read.alpha {
        p[3] as f64
    } else {
        let luma = 0.2126 * p[0] as f64 + 0.7152 * p[1] as f64 + 0.0722 * p[2] as f64;
        crate::grade::to_srgb(luma.clamp(0.0, 1.0))
    };
    let v = if read.invert { 1.0 - v } else { v };
    (v - read.focus).abs() * big as f64
}

/// D-359: Lens Blur with a blur map, `map` the named layer's picture fitted to the drawing, whose
/// corner is at `origin` in `source`. Each output pixel reads the map where it lies, held inside
/// the map's rectangle; how far that value is from the focus, d, says how far out of focus the
/// pixel is: its output is the mean over the iris at level floor(d J) mixed toward the next
/// level by the rest, each mean taken from the rows' running totals in the iris's order, so the
/// last level is [`lens_blur`]'s pixel to the bit. The layer grows as [`lens_blur`]'s does,
/// returned. The settings are already valid.
pub(crate) fn lens_blur_map(
    source: &mut WorkingBuffer,
    radius: f64,
    repeat: bool,
    iris: &Iris,
    map: &WorkingBuffer,
    origin: (usize, usize),
    read: &LensMap,
) -> usize {
    let g = if repeat {
        0
    } else {
        lens_reach(radius, iris.aspect).ceil() as isize
    };
    let levels = lens_levels(radius, iris);
    let big = levels.len() - 1;
    let counts: Vec<f64> = levels
        .iter()
        .map(|runs| runs.iter().map(|&[_, lo, hi]| hi - lo + 1).sum::<isize>() as f64)
        .collect();
    let (w, h) = (source.width() as isize, source.height() as isize);
    let (mw, mh) = (map.width() as isize, map.height() as isize);
    let (src, sums, empty) = lens_rows(source, iris);
    let src = &src[..];
    // One level's mean at the drawing's pixel (x, y), each run summed as `lens_blur` sums it.
    let mean = |runs: &[[isize; 3]], n: f64, x: isize, y: isize| {
        let mut acc = [0.0f64; 4];
        for &[dy, lo, hi] in runs {
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
            let (a, b) = (x - hi, x - lo);
            let (l, r) = (a.max(0), b.min(w - 1));
            for i in 0..4 {
                if l <= r {
                    acc[i] += s[r as usize + 1][i] - s[l as usize][i];
                }
                if repeat {
                    let left = (b.min(-1) - a + 1).max(0) as f64;
                    let right = (b - a.max(w) + 1).max(0) as f64;
                    acc[i] += left * src[row + i] as f64 + right * src[row + (w as usize - 1) * 4 + i] as f64;
                }
            }
        }
        acc.map(|v| v / n)
    };
    let ow = (w + 2 * g) as usize;
    let mut out = WorkingBuffer::transparent(ow, (h + 2 * g) as usize);
    out.data_mut().par_chunks_mut(ow * 4).enumerate().for_each(|(oy, line)| {
        let y = oy as isize - g;
        for (ox, px) in line.chunks_exact_mut(4).enumerate() {
            let x = ox as isize - g;
            let s = if mw > 0 && mh > 0 {
                let mx = (x - origin.0 as isize).clamp(0, mw - 1);
                let my = (y - origin.1 as isize).clamp(0, mh - 1);
                lens_level(&map.data()[(my * mw + mx) as usize * 4..][..4], read, big)
            } else {
                lens_level(&[0.0; 4], read, big)
            };
            let j = (s.floor() as usize).min(big);
            let t = s - j as f64;
            let mut acc = mean(&levels[j], counts[j], x, y);
            if t > 0.0 {
                let next = mean(&levels[j + 1], counts[j + 1], x, y);
                for i in 0..4 {
                    acc[i] += t * (next[i] - acc[i]);
                }
            }
            if iris.gain > 0.0 {
                for i in 0..3 {
                    acc[i] = acc[i].min(acc[3]);
                }
            }
            for i in 0..4 {
                px[i] = acc[i] as f32;
            }
        }
    });
    *source = out;
    g as usize
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
    let (cx, cy) = crate::effects::radial_center(center, (source.width(), source.height()), origin);
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
    crate::blurs::radial_blur(&mut rays, false, length, center, false, None);
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
    (displacement, pin): (&str, bool),
) -> usize {
    if amount == 0.0 {
        return 0;
    }
    let g = if repeat { 0 } else { amount.ceil() as usize };
    let (iw, ih) = (source.width() as f64, source.height() as f64);
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
            // D-306: a pinned push fades over one `size` from each edge of the buffer it was handed.
            let k = if pin {
                let fade = |d: f64| {
                    let t = (d / size).clamp(0.0, 1.0);
                    t * t * (3.0 - 2.0 * t)
                };
                let (u, v) = (x - g as f64, y - g as f64);
                fade(u) * fade(iw - u) * fade(v) * fade(ih - v)
            } else {
                1.0
            };
            let mut sx = x;
            let mut sy = y;
            if displacement != "vertical" {
                sx += k * amount * crate::grade::fractal(base, 0, p, octaves);
            }
            if displacement != "horizontal" {
                sy += k * amount * crate::grade::fractal(base, 1, p, octaves);
            }
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

/// D-213: each shown pixel moved toward `light`, linear, by `intensity` times its slope when the
/// slope is above 0, or toward black by it when below, its covering unchanged.
fn bevel_shade(source: &mut WorkingBuffer, slope: impl Fn(usize, usize) -> f64 + Sync, light: [f64; 3], intensity: f64) {
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
            let s = slope(i % w, i / w);
            for c in 0..3 {
                let p = px[c] as f64;
                px[c] = if s > 0.0 {
                    p + (light[c] * a - p) * intensity * s
                } else {
                    p * (1.0 + intensity * s)
                } as f32;
            }
        });
}

/// D-213: Bevel Alpha. The slope from the covering blurred at sigma `thickness / 2`, by central
/// differences, 0 beyond the blur. The settings are already valid.
pub(crate) fn bevel_alpha(source: &mut WorkingBuffer, thickness: f64, angle: f64, light: [f64; 3], intensity: f64) {
    if thickness <= 0.0 || intensity <= 0.0 {
        return;
    }
    let mut cover = source.clone();
    cover.data_mut().par_chunks_exact_mut(4).for_each(|px| px[..3].fill(0.0));
    let r = blur(&mut cover, thickness / 2.0) as isize;
    let (bw, bh) = (cover.width() as isize, cover.height() as isize);
    let data = cover.data();
    let height = |x: isize, y: isize| {
        let (bx, by) = (x + r, y + r);
        if bx < 0 || by < 0 || bx >= bw || by >= bh {
            0.0
        } else {
            data[((by * bw + bx) * 4 + 3) as usize] as f64
        }
    };
    let (ux, uy) = crate::blurs::along(angle);
    bevel_shade(
        source,
        |x, y| {
            let (x, y) = (x as isize, y as isize);
            let gx = (height(x + 1, y) - height(x - 1, y)) / 2.0;
            let gy = (height(x, y + 1) - height(x, y - 1)) / 2.0;
            (-1.25 * thickness * (gx * ux + gy * uy)).clamp(-1.0, 1.0)
        },
        light,
        intensity,
    );
}

/// D-317: After Effects' CC Glass, reduced. The bump is the phase (D-316) `property` reads from
/// the drawing, or from `map` lying on it with its corner at the given origin (clear outside it),
/// times the covering unless the property is alpha, blurred at sigma `softness / 2`. Its slope `n`
/// is the bump's central differences, 0 beyond the blur, times `height / 100 * 1.25 *
/// max(softness, 1)`. Each pixel takes the drawing read bilinearly `displacement` pixels along `n`,
/// then is lit as Bevel Alpha lights it, its slope toward the light `-(n . u)` held to -1..1. So
/// at height 100 and no displacement it is Bevel Alpha with an edge thickness of the softness.
/// The settings are already valid.
pub(crate) fn glass(
    source: &mut WorkingBuffer,
    map: Option<(&WorkingBuffer, (usize, usize))>,
    property: &str,
    (softness, height, displacement): (f64, f64, f64),
    (angle, light, intensity): (f64, [f64; 3], f64),
) {
    if height == 0.0 || (displacement == 0.0 && intensity <= 0.0) {
        return;
    }
    let w = source.width();
    let mut bump = WorkingBuffer::transparent(w, source.height());
    let from = source.data();
    bump.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let q = match map {
            Some((m, (ox, oy))) => {
                let (mx, my) = ((i % w).wrapping_sub(ox), (i / w).wrapping_sub(oy));
                if mx < m.width() && my < m.height() { m.pixel(mx, my) } else { [0.0; 4] }
            }
            None => [from[i * 4], from[i * 4 + 1], from[i * 4 + 2], from[i * 4 + 3]],
        };
        px[3] = (crate::grade::phase_of(property, &q) * if property == "alpha" { 1.0 } else { q[3] as f64 }) as f32;
    });
    let r = if softness > 0.0 { blur(&mut bump, softness / 2.0) as isize } else { 0 };
    let (bw, bh) = (bump.width() as isize, bump.height() as isize);
    let data = bump.data();
    let at = |x: isize, y: isize| {
        let (bx, by) = (x + r, y + r);
        if bx < 0 || by < 0 || bx >= bw || by >= bh {
            0.0
        } else {
            data[((by * bw + bx) * 4 + 3) as usize] as f64
        }
    };
    let k = height / 100.0 * 1.25 * softness.max(1.0);
    let slope: Vec<(f64, f64)> = (0..from.len() / 4)
        .into_par_iter()
        .map(|i| {
            let (x, y) = ((i % w) as isize, (i / w) as isize);
            ((at(x + 1, y) - at(x - 1, y)) / 2.0 * k, (at(x, y + 1) - at(x, y - 1)) / 2.0 * k)
        })
        .collect();
    if displacement != 0.0 {
        let still = source.clone();
        source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
            let (nx, ny) = slope[i];
            let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
            px.copy_from_slice(&sample_bilinear(&still, x + displacement * nx, y + displacement * ny));
        });
    }
    if intensity > 0.0 {
        let (ux, uy) = crate::blurs::along(angle);
        bevel_shade(source, |x, y| {
            let (nx, ny) = slope[y * w + x];
            (-(nx * ux + ny * uy)).clamp(-1.0, 1.0)
        }, light, intensity);
    }
}

/// D-336: CC Vector Blur, this program's own reading of CycoreFX's manual. The height is the
/// `property` phase of the drawing, or of `map` lying on it with its corner at the given origin
/// (clear outside it), times the covering unless the property is alpha, blurred at sigma
/// `softness / 2`. Natural, constant and perpendicular smear along its slope turned `angle`
/// degrees (90 more for perpendicular), where the slope is at least 1/10000; natural's length is
/// `amount` shortened on gentle slopes by `ridge`. The directional types smear `amount` along
/// `angle + 360 ridge height` degrees from up. Each pixel is the weighted mean of bilinear samples
/// at `k / ceil(amount)` of its vector, `k` from `-ceil(amount)` (from 0 for direction_fading) to
/// `ceil(amount)`, weighing `1 - |t|` for natural, perpendicular and direction_fading, else 1.
/// The settings are already valid.
pub(crate) fn vector_blur(
    source: &mut WorkingBuffer,
    map: Option<(&WorkingBuffer, (usize, usize))>,
    (kind, property): (&str, &str),
    [amount, angle, ridge, softness]: [f64; 4],
) {
    const FLAT: f64 = 1e-4;
    if amount == 0.0 {
        return;
    }
    let w = source.width();
    let mut height = WorkingBuffer::transparent(w, source.height());
    let from = source.data();
    height.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let q = match map {
            Some((m, (ox, oy))) => {
                let (mx, my) = ((i % w).wrapping_sub(ox), (i / w).wrapping_sub(oy));
                if mx < m.width() && my < m.height() { m.pixel(mx, my) } else { [0.0; 4] }
            }
            None => [from[i * 4], from[i * 4 + 1], from[i * 4 + 2], from[i * 4 + 3]],
        };
        px[3] = (crate::grade::phase_of(property, &q) * if property == "alpha" { 1.0 } else { q[3] as f64 }) as f32;
    });
    let r = if softness > 0.0 { blur(&mut height, softness / 2.0) as isize } else { 0 };
    let (bw, bh) = (height.width() as isize, height.height() as isize);
    let data = height.data();
    let at = |x: isize, y: isize| {
        let (bx, by) = (x + r, y + r);
        if bx < 0 || by < 0 || bx >= bw || by >= bh {
            0.0
        } else {
            data[((by * bw + bx) * 4 + 3) as usize] as f64
        }
    };
    let n = amount.ceil() as i64;
    let fading = matches!(kind, "natural" | "perpendicular" | "direction_fading");
    let still = source.clone();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as isize, (i / w) as isize);
        let ((ux, uy), size) = if kind.starts_with("direction") {
            (crate::blurs::along(angle + 360.0 * ridge * at(x, y)), amount)
        } else {
            let (gx, gy) = ((at(x + 1, y) - at(x - 1, y)) / 2.0, (at(x, y + 1) - at(x, y - 1)) / 2.0);
            let g = (gx * gx + gy * gy).sqrt();
            if g < FLAT {
                return;
            }
            let turn = (angle + if kind == "perpendicular" { 90.0 } else { 0.0 }).to_radians();
            let (c, s, ux, uy) = (turn.cos(), turn.sin(), gx / g, gy / g);
            let size = if kind == "constant" || ridge == 0.0 {
                amount
            } else {
                amount * 100.0 * g / ((100.0 * g).powi(2) + ridge * ridge).sqrt()
            };
            ((ux * c - uy * s, ux * s + uy * c), size)
        };
        let (mut acc, mut total) = ([0.0f64; 4], 0.0);
        for k in if kind == "direction_fading" { 0 } else { -n }..=n {
            let t = k as f64 / n as f64;
            let weight = if fading { 1.0 - t.abs() } else { 1.0 };
            if weight == 0.0 {
                continue;
            }
            let s = sample_bilinear(&still, x as f64 + 0.5 + t * size * ux, y as f64 + 0.5 + t * size * uy);
            for c in 0..4 {
                acc[c] += weight * s[c] as f64;
            }
            total += weight;
        }
        for c in 0..4 {
            px[c] = (acc[c] / total) as f32;
        }
    });
}

/// D-377: Bend It, this program's own reading of CycoreFX's CC Bend It. The bar from `start` to
/// `end` (points of the buffer, length L, direction t, n = t turned a quarter clockwise) is bent
/// into an arc turning `bend` degrees over its length, round C = S + R n with R = L / theta. Each
/// pixel lays over one another, the farthest along the bar on top, the bilinear samples of every
/// place the bend puts on it: on the arc, phi = atan2(-q.t, q.n) + j pi for q = P - C, v = R +
/// (-1)^j |q| and u = phi L / theta, kept for `lo` <= u <= L. `prestart` 0 draws nothing before
/// Start; 1 (static) the drawing unbent where (P - S).t < 0; 2 (bend) carries the arc back to -L
/// and on straight past it; 3 (mirror) bends the bar's own stretch back to -L, read at (-u, v).
/// `extended` carries the drawing on straight past End. Bend 0 is the straight bar. The settings
/// are already valid.
pub(crate) fn bend_it(source: &mut WorkingBuffer, bend: f64, start: (f64, f64), end: (f64, f64), prestart: u8, extended: bool) {
    let length = (end.0 - start.0).hypot(end.1 - start.1);
    if length == 0.0 {
        return;
    }
    let (tx, ty) = ((end.0 - start.0) / length, (end.1 - start.1) / length);
    let (nx, ny) = (-ty, tx);
    let from = |u: f64, v: f64| (start.0 + u * tx + v * nx, start.1 + u * ty + v * ny);
    let theta = bend.to_radians();
    let r = length / theta;
    let (cx, cy) = (start.0 + r * nx, start.1 + r * ny);
    // The frame turned `phi` along the arc: its point on the bar's line, its t and n.
    let frame = |phi: f64| {
        let (s, c) = phi.sin_cos();
        let (t, n) = ((tx * c + nx * s, ty * c + ny * s), (nx * c - tx * s, ny * c - ty * s));
        ((cx - r * n.0, cy - r * n.1), t, n)
    };
    let lo = if prestart >= 2 { -length } else { 0.0 };
    let w = source.width();
    let still = source.clone();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
        let mut acc = [0.0f64; 4];
        let mut lay = |(sx, sy): (f64, f64)| {
            let s = sample_bilinear(&still, sx, sy);
            let k = 1.0 - acc[3];
            for c in 0..4 {
                acc[c] += k * s[c] as f64;
            }
        };
        let (a, b) = ((x - start.0) * tx + (y - start.1) * ty, (x - start.0) * nx + (y - start.1) * ny);
        if bend == 0.0 {
            if (0.0..=length).contains(&a) || (a > length && extended) || (a < 0.0 && (prestart == 1 || prestart == 2)) {
                lay((x, y));
            } else if (-length..0.0).contains(&a) && prestart == 3 {
                lay(from(-a, b));
            }
        } else {
            let tail = |phi: f64| {
                let ((ox, oy), t, n) = frame(phi);
                ((x - ox) * t.0 + (y - oy) * t.1, (x - ox) * n.0 + (y - oy) * n.1)
            };
            if extended {
                let (a2, b2) = tail(theta);
                if a2 > 0.0 {
                    lay(from(length + a2, b2));
                }
            }
            let (qx, qy) = (x - cx, y - cy);
            let first = (-(qx * tx + qy * ty)).atan2(qx * nx + qy * ny);
            let size = qx.hypot(qy);
            let k = theta / length;
            let (lo_phi, hi_phi) = if k > 0.0 { (k * lo, k * length) } else { (k * length, k * lo) };
            let pi = std::f64::consts::PI;
            let (j0, j1) = (((lo_phi - first) / pi).floor() as i64 - 1, ((hi_phi - first) / pi).ceil() as i64 + 1);
            // Farthest along first: u = phi / k rises with j when k is above 0.
            let mut j = if k > 0.0 { j1 } else { j0 };
            while (j0..=j1).contains(&j) {
                let phi = first + j as f64 * pi;
                let u = phi / k;
                let v = r + if j.rem_euclid(2) == 0 { size } else { -size };
                if lo <= u && u <= length {
                    lay(if u < 0.0 && prestart == 3 { from(-u, v) } else { from(u, v) });
                }
                j += if k > 0.0 { -1 } else { 1 };
            }
            if prestart == 1 && a < 0.0 {
                lay((x, y));
            } else if prestart == 2 {
                let (a2, b2) = tail(-theta);
                if a2 < 0.0 {
                    lay(from(-length + a2, b2));
                }
            }
        }
        for c in 0..4 {
            px[c] = acc[c] as f32;
        }
    });
}

/// D-378: Bender, this program's own reading of CycoreFX's CC Bender. Each pixel at `s` along
/// the axis from `base` (0) to `top` (1), points of the buffer, reads the bilinear sample of the
/// drawing `d(s)` back across the axis, `d` by `style` from `amount` pixels: bend A s^2 from the
/// Base, straight on past the Top; marilyn A sin^2(pi s) and sharp A (1 - |2s - 1|) between the
/// points; boxer A (3s^2 - 2s^3), A past the Top. The settings are already valid.
pub(crate) fn bender(source: &mut WorkingBuffer, amount: f64, style: &str, base: (f64, f64), top: (f64, f64)) {
    let length = (top.0 - base.0).hypot(top.1 - base.1);
    if length == 0.0 || amount == 0.0 {
        return;
    }
    let (ux, uy) = ((top.0 - base.0) / length, (top.1 - base.1) / length);
    let (nx, ny) = (-uy, ux);
    let w = source.width();
    let still = source.clone();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
        let d = bender_push(style, amount, ((x - base.0) * ux + (y - base.1) * uy) / length);
        if d != 0.0 {
            px.copy_from_slice(&sample_bilinear(&still, x - d * nx, y - d * ny));
        }
    });
}

/// D-378: Bender's push at `s` along the axis, `a` its amount.
pub(crate) fn bender_push(style: &str, a: f64, s: f64) -> f64 {
    match style {
        "bend" if s < 0.0 => 0.0,
        "bend" if s <= 1.0 => a * s * s,
        "bend" => a * (2.0 * s - 1.0),
        "boxer" if s < 0.0 => 0.0,
        "boxer" if s <= 1.0 => a * (3.0 * s * s - 2.0 * s * s * s),
        "boxer" => a,
        _ if !(0.0..=1.0).contains(&s) => 0.0,
        "marilyn" => a * (std::f64::consts::PI * s).sin().powi(2),
        _ => a * (1.0 - (2.0 * s - 1.0).abs()),
    }
}

/// D-385: Flow Motion, this program's own reading of CycoreFX's CC Flo Motion. Each knot, a
/// point of the buffer with its strength `a`, scales the picture about itself by
/// m = 1 + a g (a >= 0) or 1 / (1 - a g), g = sigma^2 / (sigma^2 + r^2); a point P reads the
/// drawing at P + sum (P - K)(m - 1), mirrored round the buffer when `tile`, transparent outside
/// it otherwise, and a pixel averages `n` by `n` such points across it. Both strengths 0 is the
/// drawing. The settings are already valid.
pub(crate) fn flow_motion(source: &mut WorkingBuffer, knots: [((f64, f64), f64); 2], sigma: f64, tile: bool, n: usize) {
    if knots.iter().all(|k| k.1 == 0.0) {
        return;
    }
    let w = source.width();
    let s2 = sigma * sigma;
    let still = source.clone();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x0, y0) = ((i % w) as f64, (i / w) as f64);
        let mut sum = [0.0f64; 4];
        for j in 0..n {
            for k in 0..n {
                let (x, y) = (x0 + (k as f64 + 0.5) / n as f64, y0 + (j as f64 + 0.5) / n as f64);
                let (mut sx, mut sy) = (x, y);
                for &((kx, ky), a) in &knots {
                    if a != 0.0 {
                        let (dx, dy) = (x - kx, y - ky);
                        let m = flow_scale(a, s2 / (s2 + dx * dx + dy * dy));
                        sx += dx * (m - 1.0);
                        sy += dy * (m - 1.0);
                    }
                }
                let p = if tile { sample_mirrored(&still, sx, sy) } else { sample_bilinear(&still, sx, sy) };
                for c in 0..4 {
                    sum[c] += p[c] as f64;
                }
            }
        }
        let nn = (n * n) as f64;
        for c in 0..4 {
            px[c] = (sum[c] / nn) as f32;
        }
    });
}

/// D-385: a knot's scale, `a` its strength and `g` its weight at the point.
pub(crate) fn flow_scale(a: f64, g: f64) -> f64 {
    if a >= 0.0 { 1.0 + a * g } else { 1.0 / (1.0 - a * g) }
}

/// D-385: `sample_bilinear` of the buffer repeated round itself, every other copy turned over:
/// column i reads column j = i mod 2w, or 2w - 1 - j when j >= w, and the same for rows.
fn sample_mirrored(src: &WorkingBuffer, x: f64, y: f64) -> [f32; 4] {
    let (w, h) = (src.width() as i64, src.height() as i64);
    let fold = |i: i64, n: i64| {
        let j = i.rem_euclid(2 * n);
        (if j >= n { 2 * n - 1 - j } else { j }) as usize
    };
    let (fx, fy) = (x - 0.5, y - 0.5);
    let (x0, y0) = (fx.floor(), fy.floor());
    let (ux, uy) = (fx - x0, fy - y0);
    let (x0, y0) = (x0 as i64, y0 as i64);
    let data = src.data();
    let mut out = [0.0f32; 4];
    for (dy, wy) in [(0, 1.0 - uy), (1, uy)] {
        for (dx, wx) in [(0, 1.0 - ux), (1, ux)] {
            let wt = wx * wy;
            if wt != 0.0 {
                let s = (fold(y0 + dy, h) * w as usize + fold(x0 + dx, w)) * 4;
                for c in 0..4 {
                    out[c] += data[s + c] * wt as f32;
                }
            }
        }
    }
    out
}

/// D-386: Griddler, this program's own reading of CycoreFX's CC Griddler. Tiles `tile` pixels
/// square from the drawing's corner at `origin`; a pixel's offset from its tile's centre, turned
/// back by `turn` radians (clockwise on the screen) and divided by `scale` across and down, is
/// where it reads the drawing from that centre, transparent outside the tile's own square when
/// `cut`. A scale of 0 draws nothing. The settings are already valid.
pub(crate) fn griddler(source: &mut WorkingBuffer, scale: (f64, f64), tile: f64, turn: f64, cut: bool, origin: (usize, usize)) {
    if scale == (1.0, 1.0) && turn == 0.0 {
        return;
    }
    let w = source.width();
    if scale.0 == 0.0 || scale.1 == 0.0 {
        source.data_mut().fill(0.0);
        return;
    }
    let (sin, cos) = turn.sin_cos();
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    let still = source.clone();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as f64 + 0.5 - ox, (i / w) as f64 + 0.5 - oy);
        let (cx, cy) = (((x / tile).floor() + 0.5) * tile, ((y / tile).floor() + 0.5) * tile);
        let (qx, qy) = (x - cx, y - cy);
        let ux = (qx * cos + qy * sin) / scale.0;
        let uy = (qy * cos - qx * sin) / scale.1;
        if cut && (ux.abs() > tile / 2.0 || uy.abs() > tile / 2.0) {
            px.fill(0.0);
        } else {
            px.copy_from_slice(&sample_bilinear(&still, ox + cx + ux, oy + cy + uy));
        }
    });
}

/// D-387: Fisheye, this program's own reading of CycoreFX's CC Lens. A pixel `r` from `center`,
/// rho = min(r / radius, 1) of the way out, reads the drawing at center + (P - center) f / rho,
/// f = rho + c (2 asin(rho) / pi - rho) for `c` >= 0, rho - c (sin(pi rho / 2) - rho) below,
/// times the lens's cover min(1, max(0, radius - r + 0.5)). A radius of 0 draws nothing. The
/// settings are already valid.
pub(crate) fn fisheye(source: &mut WorkingBuffer, center: (f64, f64), radius: f64, c: f64) {
    let w = source.width();
    if radius <= 0.0 {
        source.data_mut().fill(0.0);
        return;
    }
    let still = source.clone();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (dx, dy) = ((i % w) as f64 + 0.5 - center.0, (i / w) as f64 + 0.5 - center.1);
        let r = dx.hypot(dy);
        let cover = (radius - r + 0.5).clamp(0.0, 1.0);
        if cover == 0.0 {
            px.fill(0.0);
            return;
        }
        let k = if r == 0.0 {
            0.0
        } else {
            let rho = (r / radius).min(1.0);
            fisheye_curve(c, rho) / rho
        };
        let p = sample_bilinear(&still, center.0 + dx * k, center.1 + dy * k);
        for ch in 0..4 {
            px[ch] = (p[ch] as f64 * cover) as f32;
        }
    });
}

/// D-387: how far out a point rho of the way out reads, `c` the convergence over 100.
pub(crate) fn fisheye_curve(c: f64, rho: f64) -> f64 {
    use std::f64::consts::PI;
    if c >= 0.0 { rho + c * (2.0 * rho.asin() / PI - rho) } else { rho - c * ((PI * rho / 2.0).sin() - rho) }
}

/// D-379: Blobbylize, this program's own reading of CycoreFX's CC Blobbylize. The blob's height
/// is the `property` phase of the drawing, or of `map` lying on it with its corner at the given
/// origin (clear outside it), times the covering unless alpha, blurred at sigma `softness / 2`;
/// its covering is the height less `cut`, stretched back to 0..1. The colour is the drawing over
/// its own blur at the same sigma, straight. Its surface, normal (-k gx, -k gy, 1) with k = 1.25
/// max(softness, 1) and g the height's central differences, is lit by Phong's rule from a point
/// light at `point` (a point of the buffer, `height` above) or from `direction` degrees at
/// `height` over 100: ambient + diffuse I Lc (N.L) on the colour, specular I (Lc toward the
/// colour by metal) (R.z)^(1 / roughness) added. The settings are already valid, the shares out of
/// 1 and `light` linear.
pub(crate) fn blobbylize(
    source: &mut WorkingBuffer,
    map: Option<(&WorkingBuffer, (usize, usize))>,
    property: &str,
    (softness, cut): (f64, f64),
    (point, direction, height, light, intensity): (Option<(f64, f64)>, f64, f64, [f64; 3], f64),
    [ambient, diffuse, specular, roughness, metal]: [f64; 5],
) {
    let w = source.width();
    let mut bump = WorkingBuffer::transparent(w, source.height());
    let from = source.data();
    bump.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let q = match map {
            Some((m, (ox, oy))) => {
                let (mx, my) = ((i % w).wrapping_sub(ox), (i / w).wrapping_sub(oy));
                if mx < m.width() && my < m.height() { m.pixel(mx, my) } else { [0.0; 4] }
            }
            None => [from[i * 4], from[i * 4 + 1], from[i * 4 + 2], from[i * 4 + 3]],
        };
        px[3] = (crate::grade::phase_of(property, &q) * if property == "alpha" { 1.0 } else { q[3] as f64 }) as f32;
    });
    let mut spread = source.clone();
    let r = if softness > 0.0 {
        blur(&mut spread, softness / 2.0);
        blur(&mut bump, softness / 2.0) as isize
    } else {
        0
    };
    let (bw, bh) = (bump.width() as isize, bump.height() as isize);
    let (data, spread) = (bump.data(), spread.data());
    let at = |x: isize, y: isize| {
        let (bx, by) = (x + r, y + r);
        if bx < 0 || by < 0 || bx >= bw || by >= bh {
            0.0
        } else {
            data[((by * bw + bx) * 4 + 3) as usize] as f64
        }
    };
    let k = 1.25 * softness.max(1.0);
    let unit = |v: [f64; 3]| {
        let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        if l > 0.0 { v.map(|c| c / l) } else { [0.0; 3] }
    };
    let (ux, uy) = crate::blurs::along(direction);
    let distant = unit([100.0 * ux, 100.0 * uy, height]);
    let still = source.clone();
    let from = still.data();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as isize, (i / w) as isize);
        let a = if cut >= 1.0 { 0.0 } else { ((at(x, y) - cut) / (1.0 - cut)).clamp(0.0, 1.0) };
        if a == 0.0 {
            px.fill(0.0);
            return;
        }
        let o = &from[i * 4..i * 4 + 4];
        let b = ((y + r) * bw + x + r) as usize * 4;
        let kk = [0, 1, 2, 3].map(|c| o[c] as f64 + (1.0 - o[3] as f64) * spread[b + c] as f64);
        let color = if kk[3] > 0.0 { [kk[0] / kk[3], kk[1] / kk[3], kk[2] / kk[3]] } else { [0.0; 3] };
        let (gx, gy) = ((at(x + 1, y) - at(x - 1, y)) / 2.0, (at(x, y + 1) - at(x, y - 1)) / 2.0);
        let n = unit([-k * gx, -k * gy, 1.0]);
        let l = match point {
            Some((lx, ly)) => unit([lx - (x as f64 + 0.5), ly - (y as f64 + 0.5), height]),
            None => distant,
        };
        let nl = n[0] * l[0] + n[1] * l[1] + n[2] * l[2];
        let rz = 2.0 * nl * n[2] - l[2];
        let shine = if nl > 0.0 && rz > 0.0 { rz.powf(1.0 / roughness) } else { 0.0 };
        for c in 0..3 {
            let lit = color[c] * (ambient + diffuse * intensity * light[c] * nl.max(0.0))
                + specular * intensity * (light[c] + (color[c] - light[c]) * metal) * shine;
            px[c] = (lit * a) as f32;
        }
        px[3] = a as f32;
    });
}

/// D-213: Bevel Edges. A pixel nearer than `thickness` times the buffer's smaller side to the
/// buffer's nearest side, the first of left, top, right and bottom among equals, is on that
/// side's face. The settings are already valid.
pub(crate) fn bevel_edges(source: &mut WorkingBuffer, thickness: f64, angle: f64, light: [f64; 3], intensity: f64) {
    let (w, h) = (source.width() as f64, source.height() as f64);
    let t = thickness * w.min(h);
    if t <= 0.0 || intensity <= 0.0 {
        return;
    }
    let (ux, uy) = crate::blurs::along(angle);
    bevel_shade(
        source,
        |x, y| {
            let (cx, cy) = (x as f64 + 0.5, y as f64 + 0.5);
            let mut near = (cx, -ux);
            for side in [(cy, -uy), (w - cx, ux), (h - cy, uy)] {
                if side.0 < near.0 {
                    near = side;
                }
            }
            if near.0 < t {
                near.1
            } else {
                0.0
            }
        },
        light,
        intensity,
    );
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

/// D-368: each pixel that shows rebuilt from itself and its eight neighbours' encoded colours
/// (0 for one that does not show; the border repeats past the edge), weighed by `grid`, row 0
/// above and each row's first number on the left, over `divider`, made positive when
/// `absolute`, held inside 0 to 1. The middle pixel alone, over a divider of the same, leaves
/// the layer as it is. The settings are already valid.
pub(crate) fn kernel(source: &mut WorkingBuffer, grid: [[f64; 3]; 3], divider: f64, absolute: bool) {
    if grid.map(|row| row.map(|v| v / divider)) == [[0.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0]] {
        return;
    }
    let (w, h) = (source.width() as i64, source.height() as i64);
    let values: Vec<[f64; 3]> = source
        .data()
        .par_chunks_exact(4)
        .map(|p| {
            let a = p[3] as f64;
            if a <= 0.0 {
                return [0.0; 3];
            }
            [0, 1, 2].map(|c| crate::grade::to_srgb((p[c] as f64 / a).clamp(0.0, 1.0)))
        })
        .collect();
    let at = |x: i64, y: i64| values[(y.clamp(0, h - 1) * w + x.clamp(0, w - 1)) as usize];
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
            let mut sum = [0.0; 3];
            for (j, row) in grid.iter().enumerate() {
                for (k, weight) in row.iter().enumerate() {
                    let v = at(x + k as i64 - 1, y + j as i64 - 1);
                    for c in 0..3 {
                        sum[c] += weight * v[c];
                    }
                }
            }
            for c in 0..3 {
                let u = sum[c] / divider;
                let u = if absolute { u.abs() } else { u };
                px[c] = (crate::grade::to_linear(u.clamp(0.0, 1.0)) * a) as f32;
            }
        });
}


/// D-147: each pixel that shows pushed `amount` percent further from its colour blurred at
/// sigma `radius`, both as written, the blur divided by its own covering. D-317: a channel
/// nearer its blur than `threshold` levels of 255 is left alone. The settings are already valid.
pub(crate) fn sharpen(source: &mut WorkingBuffer, amount: f64, radius: f64, threshold: f64) {
    if amount <= 0.0 || radius <= 0.0 {
        return;
    }
    let mut blurred = source.clone();
    let r = crate::effects::blur(&mut blurred, radius);
    let (k, t) = (amount / 100.0, threshold / 255.0);
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
                    if (e - eb).abs() < t {
                        continue;
                    }
                    let e = e + k * (e - eb);
                    px[c] = (crate::grade::to_linear(e.clamp(0.0, 1.0)) * a) as f32;
                }
            }
        });
}


/// D-148: each pixel that shows moved `amount` percent of the way toward its colour laid with
/// `blend` under the picture blurred at a third of `radius`, the blur divided by its own
/// covering, in linear light and not clamped. D-364: then `second_amount` percent of the way
/// toward that blur laid over the result in `second_blend` (soft light or overlay), on encoded
/// colours held to 0 to 1, as the layer modes do (D-301). The settings are already valid.
pub(crate) fn diffusion(
    source: &mut WorkingBuffer,
    radius: f64,
    amount: f64,
    blend: &str,
    (second_amount, second_blend): (f64, &str),
) {
    if radius <= 0.0 || (amount <= 0.0 && second_amount <= 0.0) {
        return;
    }
    let mut blurred = source.clone();
    let r = crate::effects::blur(&mut blurred, radius / 3.0);
    let k = amount / 100.0;
    let k2 = second_amount / 100.0;
    let mixer = crate::grade::mixer(second_blend);
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
                    let r = b + k * (f - b);
                    let r = if k2 > 0.0 {
                        let s = mixer(
                            crate::grade::to_srgb(r.clamp(0.0, 1.0)),
                            crate::grade::to_srgb(g.clamp(0.0, 1.0)),
                        );
                        r + k2 * (crate::grade::to_linear(s) - r)
                    } else {
                        r
                    };
                    px[c] = (r * a) as f32;
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
pub(crate) fn bulge(source: &mut WorkingBuffer, [radius, vertical, taper]: [f64; 3], height: f64, center: (f64, f64)) {
    if height == 0.0 || radius <= 0.0 {
        return;
    }
    // D-310: 0 follows the radius, a circle as before.
    let tall = if vertical > 0.0 { vertical } else { radius };
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
            let t = if tall == radius {
                if d >= radius {
                    return;
                }
                1.0 - d / radius
            } else {
                let n = (vx / radius).hypot(vy / tall);
                if n >= 1.0 {
                    return;
                }
                1.0 - n
            };
            // D-310: the swell fades by smoothstep over `taper` pixels in from the edge, the
            // edge's distance along this pixel's ray from the centre, d (1 - n) / n.
            let h = if taper > 0.0 && t < 1.0 {
                let s = (d * t / (1.0 - t) / taper).min(1.0);
                height * s * s * (3.0 - 2.0 * s)
            } else {
                height
            };
            let m = (1.0 - h * t * t / 2.0).max(0.0);
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
/// D-304: `tile`, the tiles' centre in per cent of the buffer and their width and height in per
/// cent; at (50, 50), 100 and 100 it is D-154's tile, the buffer itself.
pub(crate) fn motion_tile(source: &mut WorkingBuffer, size: (f64, f64), mirror: bool, tile: ([f64; 2], f64, f64)) -> (usize, usize) {
    let (w, h) = (source.width(), source.height());
    let (gx, gy) = tile_growth(size, (w, h));
    if tile != (PLAIN_TILE, 100.0, 100.0) {
        sized_tile(source, (gx, gy), mirror, tile);
        return (gx, gy);
    }
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

/// D-304: where a Motion Tile's tiles sit when nothing moved them, the buffer's middle.
pub(crate) const PLAIN_TILE: [f64; 2] = [50.0, 50.0];

/// D-304: [`motion_tile`] with tiles that are the buffer shrunk or grown to `tile`'s width and
/// height and set round its centre. Each output pixel averages `mx` by `my` evenly spaced
/// points across it, enough that a shrunk tile skips none of the buffer's pixels; each point
/// is the bilinear sample of the buffer where the tile it lands in reads, held inside the
/// buffer's pixel centres, as D-109's held sample is.
fn sized_tile(source: &mut WorkingBuffer, (gx, gy): (usize, usize), mirror: bool, (center, tw, th): ([f64; 2], f64, f64)) {
    let (w, h) = (source.width(), source.height());
    if w == 0 || h == 0 {
        return;
    }
    let (sx, sy) = (tw / 100.0, th / 100.0);
    // Where place `p` of the row or column, from the buffer's corner, reads the buffer: tiles
    // `n s` long, one set round `c`, every other one turned over when `mirror`.
    let read = |p: f64, n: usize, c: f64, s: f64| {
        let (n, t) = (n as f64, n as f64 * s);
        let u = p - (c / 100.0 * n - t / 2.0);
        let k = (u / t).floor();
        let f = u - k * t;
        let f = if mirror && k.rem_euclid(2.0) != 0.0 { t - f } else { f };
        (f / s).clamp(0.5, n - 0.5)
    };
    // ponytail: at most 16 points a side, so a tile below 6 per cent may shimmer; an area
    // average if one does.
    let points = |s: f64| (1.0 / s).ceil().clamp(1.0, 16.0) as usize;
    let (mx, my) = (points(sx), points(sy));
    let ow = w + 2 * gx;
    let mut out = WorkingBuffer::transparent(ow, h + 2 * gy);
    let drawing = &*source;
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = ((i % ow) as f64 - gx as f64, (i / ow) as f64 - gy as f64);
            let mut sum = [0.0f32; 4];
            for b in 0..my {
                let ry = read(y + (b as f64 + 0.5) / my as f64, h, center[1], sy);
                for a in 0..mx {
                    let rx = read(x + (a as f64 + 0.5) / mx as f64, w, center[0], sx);
                    let p = sample_bilinear(drawing, rx, ry);
                    for c in 0..4 {
                        sum[c] += p[c];
                    }
                }
            }
            let n = (mx * my) as f32;
            px.copy_from_slice(&sum.map(|v| v / n));
        });
    *source = out;
}

/// D-198: Corner Pin's map from the buffer back into the drawing, the map's determinant and how
/// far the buffer grows across and down; `None` for crossed, bent-in or in-line corners.
/// B-151: the card maps the same way.
pub(crate) fn corner_map(
    [ul, ur, ll, lr]: [[f64; 2]; 4],
    (w, h): (usize, usize),
    origin: (usize, usize),
) -> Option<([[f64; 3]; 3], f64, (usize, usize))> {
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    let (dw, dh) = (w as f64 - 2.0 * ox, h as f64 - 2.0 * oy);
    // A, B, C, D round the ring, in the drawing's space.
    let ring = [ul, ur, lr, ll].map(|p| (p[0] / 100.0 * dw, p[1] / 100.0 * dh));
    if !convex(&ring) {
        return None;
    }
    let (adj, det) = adjugate(square_to_quad(ring));
    Some((adj, det, ring_growth(&ring, (dw, dh), (ox, oy))))
}

/// D-198: four points A, B, C, D round a ring, in the drawing's space.
pub(crate) type Ring = [(f64, f64); 4];

/// D-198: whether `ring` turns the same way at every corner: not crossed, bent in or in line.
fn convex(ring: &Ring) -> bool {
    let z = |i: usize| {
        let ((px, py), (qx, qy), (rx, ry)) = (ring[(i + 3) % 4], ring[i], ring[(i + 1) % 4]);
        (qx - px) * (ry - qy) - (qy - py) * (rx - qx)
    };
    let z = [z(0), z(1), z(2), z(3)];
    z.iter().all(|v| *v > 0.0) || z.iter().all(|v| *v < 0.0)
}

/// D-198: Heckbert's map of the unit square onto A (0, 0), B (1, 0), C (1, 1), D (0, 1), by rows.
pub(crate) fn square_to_quad([a, b, c, d]: Ring) -> [[f64; 3]; 3] {
    let (sx, sy) = (a.0 - b.0 + c.0 - d.0, a.1 - b.1 + c.1 - d.1);
    let (dx1, dx2, dy1, dy2) = (b.0 - c.0, d.0 - c.0, b.1 - c.1, d.1 - c.1);
    let den = dx1 * dy2 - dx2 * dy1;
    let (g, hh) = ((sx * dy2 - dx2 * sy) / den, (dx1 * sy - sx * dy1) / den);
    [
        [b.0 - a.0 + g * b.0, d.0 - a.0 + hh * d.0, a.0],
        [b.1 - a.1 + g * b.1, d.1 - a.1 + hh * d.1, a.1],
        [g, hh, 1.0],
    ]
}

/// D-198: `m`'s adjugate, the map back, and its determinant.
pub(crate) fn adjugate([[m0, m1, m2], [m3, m4, m5], [m6, m7, m8]]: [[f64; 3]; 3]) -> ([[f64; 3]; 3], f64) {
    let adj = [
        [m4 * m8 - m5 * m7, m2 * m7 - m1 * m8, m1 * m5 - m2 * m4],
        [m5 * m6 - m3 * m8, m0 * m8 - m2 * m6, m2 * m3 - m0 * m5],
        [m3 * m7 - m4 * m6, m1 * m6 - m0 * m7, m0 * m4 - m1 * m3],
    ];
    let det = m0 * (m4 * m8 - m5 * m7) - m1 * (m3 * m8 - m5 * m6) + m2 * (m3 * m7 - m4 * m6);
    (adj, det)
}

/// D-198: how far any of `ring` lies past the sides of a buffer holding a drawing `dw` by `dh`
/// with its corner at (`ox`, `oy`), across and down, rounded up.
fn ring_growth(ring: &Ring, (dw, dh): (f64, f64), (ox, oy): (f64, f64)) -> (usize, usize) {
    let past = |lo: f64, hi: f64, v: [f64; 4]| {
        let (min, max) = v.iter().fold((f64::MAX, f64::MIN), |(n, x), v| (n.min(*v), x.max(*v)));
        (lo - min).max(max - hi).ceil().max(0.0) as usize
    };
    (past(-ox, dw + ox, ring.map(|p| p.0)), past(-oy, dh + oy, ring.map(|p| p.1)))
}

/// D-389: `m` at the square's (u, v): the point, and the third row, at most 0 at or past the
/// horizon. A third row of 0 gives no point.
fn homogeneous(m: &[[f64; 3]; 3], u: f64, v: f64) -> (f64, f64, f64) {
    let [x, y, t] = m.map(|r| r[0] * u + r[1] * v + r[2]);
    if t != 0.0 {
        (x / t, y / t, t)
    } else {
        (f64::INFINITY, f64::INFINITY, t)
    }
}

/// D-389: the bilinear map of the unit square onto `ring`.
fn bilinear_map([a, b, c, d]: &Ring, u: f64, v: f64) -> (f64, f64) {
    let at = |i: usize| {
        let p = |q: &(f64, f64)| if i == 0 { q.0 } else { q.1 };
        (1.0 - u) * (1.0 - v) * p(a) + u * (1.0 - v) * p(b) + u * v * p(c) + (1.0 - u) * v * p(d)
    };
    (at(0), at(1))
}

/// D-389: the square's (u, v) the bilinear map onto `ring` carries to `x`, the root nearer the
/// square's middle, or `None`.
fn unbilinear([a, b, c, d]: &Ring, x: (f64, f64)) -> Option<(f64, f64)> {
    let cross = |p: (f64, f64), q: (f64, f64)| p.0 * q.1 - p.1 * q.0;
    let e = (b.0 - a.0, b.1 - a.1);
    let f = (d.0 - a.0, d.1 - a.1);
    let g = (a.0 - b.0 + c.0 - d.0, a.1 - b.1 + c.1 - d.1);
    let h = (x.0 - a.0, x.1 - a.1);
    let k2 = cross(g, f);
    let k1 = cross(e, f) + cross(h, g);
    let k0 = cross(h, e);
    let disc = k1 * k1 - 4.0 * k2 * k0;
    if disc < 0.0 {
        return None;
    }
    let root = disc.sqrt();
    let q = -0.5 * (k1 + if k1 < 0.0 { -root } else { root });
    let mut best: Option<(f64, f64, f64)> = None;
    for v in [if k2 != 0.0 { q / k2 } else { f64::INFINITY }, if q != 0.0 { k0 / q } else { f64::INFINITY }] {
        if !v.is_finite() {
            continue;
        }
        let (ex, ey) = (e.0 + v * g.0, e.1 + v * g.1);
        let den = ex * ex + ey * ey;
        if den == 0.0 {
            continue;
        }
        let u = ((h.0 - v * f.0) * ex + (h.1 - v * f.1) * ey) / den;
        let far = (u - 0.5).abs().max((v - 0.5).abs());
        if best.is_none_or(|b| far < b.0) {
            best = Some((far, u, v));
        }
    }
    best.map(|b| (b.1, b.2))
}

/// D-389: Power Pin's target, the expanded square's corners carried by `p` of Corner Pin's map
/// and 1 - `p` of the bilinear one, in the drawing's space (`dw` by `dh`), round the ring top
/// left, top right, bottom right, bottom left; `None` for corners that are not a ring, or a
/// corner carried to or past the horizon. `pins` top left, top right, bottom left, bottom right,
/// per cent; `expansion` top, left, right, bottom, per cent.
pub(crate) fn power_pin_ring([tl, tr, bl, br]: [[f64; 2]; 4], p: f64, [top, left, right, bottom]: [f64; 4], (dw, dh): (f64, f64)) -> Option<Ring> {
    let ring = [tl, tr, br, bl].map(|q| (q[0] / 100.0 * dw, q[1] / 100.0 * dh));
    if !convex(&ring) {
        return None;
    }
    let m = square_to_quad(ring);
    let (l, t, r, b) = (left / 100.0, top / 100.0, right / 100.0, bottom / 100.0);
    let mut q = ring;
    for (k, (u, v)) in [(-l, -t), (1.0 + r, -t), (1.0 + r, 1.0 + b), (-l, 1.0 + b)].into_iter().enumerate() {
        // Both maps carry the square's own corner to its pin, exactly.
        if (u == 0.0 || u == 1.0) && (v == 0.0 || v == 1.0) {
            continue;
        }
        let (x, y, w) = homogeneous(&m, u, v);
        if p > 0.0 && w <= 0.0 {
            return None;
        }
        let (bx, by) = bilinear_map(&ring, u, v);
        q[k] = if p > 0.0 { (p * x + (1.0 - p) * bx, p * y + (1.0 - p) * by) } else { (bx, by) };
    }
    convex(&q).then_some(q)
}

/// D-389: Power Pin's target ring, its square-to-quad map, that map's adjugate and determinant,
/// and how far the buffer grows across and down with Unstretch off; `None` when there is no ring.
#[allow(clippy::type_complexity)]
pub(crate) fn power_pin_map(
    pins: [[f64; 2]; 4],
    p: f64,
    expansion: [f64; 4],
    (w, h): (usize, usize),
    origin: (usize, usize),
) -> Option<(Ring, [[f64; 3]; 3], [[f64; 3]; 3], f64, (usize, usize))> {
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    let (dw, dh) = (w as f64 - 2.0 * ox, h as f64 - 2.0 * oy);
    let q = power_pin_ring(pins, p, expansion, (dw, dh))?;
    let mq = square_to_quad(q);
    let (adj, det) = adjugate(mq);
    Some((q, mq, adj, det, ring_growth(&q, (dw, dh), (ox, oy))))
}

/// D-389: Power Pin, this program's own reading of CycoreFX's CC Power Pin: Corner Pin with
/// `perspective` (0 to 1) of its map and the rest bilinear, the pinned square expanded by
/// `expansion` (top, left, right, bottom, per cent) past the pins, and with `unstretch` the
/// target's shape stretched out to fill the drawing instead. The drawing's corner is at `origin`;
/// returns how far it grew on the left and on the top, the same as on the right and the bottom.
/// No ring leaves the buffer clear. The settings are already valid.
pub(crate) fn power_pin(
    source: &mut WorkingBuffer,
    pins: [[f64; 2]; 4],
    perspective: f64,
    unstretch: bool,
    expansion: [f64; 4],
    origin: (usize, usize),
) -> (usize, usize) {
    if pins == [[0.0, 0.0], [100.0, 0.0], [0.0, 100.0], [100.0, 100.0]] && expansion == [0.0; 4] {
        return (0, 0);
    }
    let (w, h) = (source.width(), source.height());
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    let (dw, dh) = (w as f64 - 2.0 * ox, h as f64 - 2.0 * oy);
    let p = perspective;
    let Some((q, mq, adj, det, (gx, gy))) = power_pin_map(pins, p, expansion, (w, h), origin) else {
        *source = WorkingBuffer::transparent(w, h);
        return (0, 0);
    };
    let (wf, hf) = (w as f64, h as f64);
    let drawing = &*source;
    // A point in the drawing's space; far off, or not a number, reads nothing.
    let read = |x: f64, y: f64| {
        let (sx, sy) = (x + ox, y + oy);
        (sx > -1.0 && sx < wf + 1.0 && sy > -1.0 && sy < hf + 1.0).then(|| sample_bilinear(drawing, sx, sy))
    };
    if unstretch {
        let mut out = WorkingBuffer::transparent(w, h);
        out.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
            let (u, v) = (((i % w) as f64 + 0.5 - ox) / dw, ((i / w) as f64 + 0.5 - oy) / dh);
            let (mut x, mut y) = bilinear_map(&q, u, v);
            if p > 0.0 {
                let (hx, hy, t) = homogeneous(&mq, u, v);
                if t <= 0.0 {
                    return;
                }
                (x, y) = (p * hx + (1.0 - p) * x, p * hy + (1.0 - p) * y);
            }
            if let Some(s) = read(x, y) {
                px.copy_from_slice(&s);
            }
        });
        *source = out;
        return (0, 0);
    }
    let ow = w + 2 * gx;
    let (left, top) = ((origin.0 + gx) as f64, (origin.1 + gy) as f64);
    let mut out = WorkingBuffer::transparent(ow, h + 2 * gy);
    out.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let x = ((i % ow) as f64 + 0.5 - left, (i / ow) as f64 + 0.5 - top);
        let (mut u, mut v) = (0.0, 0.0);
        if p > 0.0 {
            let [uu, vv, t] = adj.map(|r| r[0] * x.0 + r[1] * x.1 + r[2]);
            if t * det <= 0.0 {
                return;
            }
            (u, v) = (p * (uu / t), p * (vv / t));
        }
        if p < 1.0 {
            let Some((bu, bv)) = unbilinear(&q, x) else {
                return;
            };
            (u, v) = (u + (1.0 - p) * bu, v + (1.0 - p) * bv);
        }
        if let Some(s) = read(u * dw, v * dh) {
            px.copy_from_slice(&s);
        }
    });
    *source = out;
    (gx, gy)
}

/// D-388: Page Turn, this program's own reading of CycoreFX's CC Page Turn. The drawing (its
/// corner at `origin`) rolled round a cylinder of `radius` lying on the line d = X . n - `off`
/// = 0 of the drawing's space, the part past it turned over. The back is `back` (a map lying on
/// the drawing) or the opaque `paper`, `opacity` of the way over the page's own picture and cut
/// to it, shaded by l = L . n, L the light's step, where it curls. `render` 0 the back over the
/// front, 1 the front, 2 the back. The settings are already valid, `paper` linear.
#[allow(clippy::too_many_arguments)]
pub(crate) fn page_turn(
    source: &mut WorkingBuffer,
    (n, off): ((f64, f64), f64),
    radius: f64,
    l: f64,
    back: Option<&WorkingBuffer>,
    paper: [f64; 3],
    opacity: f64,
    render: u8,
    origin: (usize, usize),
) {
    use std::f64::consts::PI;
    let w = source.width();
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    let still = source.clone();
    let r = radius;
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let x = ((i % w) as f64 + 0.5 - ox, (i / w) as f64 + 0.5 - oy);
        let d = x.0 * n.0 + x.1 * n.1 - off;
        let at = |s: f64| (x.0 + (s - d) * n.0, x.1 + (s - d) * n.1);
        let page = |s: f64| {
            let q = at(s);
            sample_bilinear(&still, q.0 + ox, q.1 + oy).map(|v| v as f64)
        };
        let turned = |s: f64, shade: f64| {
            let f = page(s);
            let q = at(s);
            let m = match back {
                Some(b) => sample_bilinear(b, q.0, q.1).map(|v| v as f64),
                None => [paper[0], paper[1], paper[2], 1.0],
            };
            let mut out = [0.0; 4];
            for c in 0..3 {
                out[c] = (f[3] * opacity * m[c] + (1.0 - opacity * m[3]) * f[c]) * shade;
            }
            out[3] = f[3];
            out
        };
        let (top, under) = if d > r || (r == 0.0 && d >= 0.0) {
            ([0.0; 4], [0.0; 4])
        } else if d >= 0.0 {
            let t = (d / r).min(1.0);
            let shade = (l * t + (1.0 - t * t).sqrt()).clamp(0.0, 1.0);
            (turned(r * (PI - t.asin()), shade), page(r * t.asin()))
        } else {
            (turned(PI * r - d, 1.0), page(d))
        };
        let out = match render {
            1 => under,
            2 => top,
            _ => [0, 1, 2, 3].map(|c| top[c] + (1.0 - top[3]) * under[c]),
        };
        for c in 0..4 {
            px[c] = out[c] as f32;
        }
    });
}

/// D-390: Ripple Pulse's push at `u` frames of travel out, the levels' changes joined by
/// straight lines, the newest inside 1/2 and none past n + 1/2.
pub(crate) fn ripple_push(levels: &[f64], u: f64) -> f64 {
    let n = levels.len() - 1;
    let g = |j: usize| if j < n { levels[j] - levels[j + 1] } else { 0.0 };
    if u <= 0.5 {
        return g(0);
    }
    if u >= n as f64 + 0.5 {
        return 0.0;
    }
    let j = (u - 0.5).floor();
    let t = u - 0.5 - j;
    g(j as usize) * (1.0 - t) + g(j as usize + 1) * t
}

/// D-390: Ripple Pulse's height at `u`, the levels joined by straight lines, the oldest past n.
pub(crate) fn ripple_height(levels: &[f64], u: f64) -> f64 {
    let n = levels.len() - 1;
    if u >= n as f64 {
        return levels[n];
    }
    let j = u.floor();
    let t = u - j;
    levels[j as usize] * (1.0 - t) + levels[j as usize + 1] * t
}

/// D-390: Ripple Pulse, this program's own reading of CycoreFX's CC Ripple Pulse: the rings
/// `levels` (the pulse level now, then at each frame before) send out from `center`, a point of
/// the buffer, reaching `big` pixels out after the whole history. Each pixel reads the drawing
/// `amplitude` / 10 times the push away from the centre, or with `bump` the rings' heights,
/// opaque grey. The settings are already valid and `levels` holds at least one.
pub(crate) fn ripple_pulse(source: &mut WorkingBuffer, center: (f64, f64), big: f64, levels: &[f64], amplitude: f64, bump: bool) {
    let w = source.width();
    let n = (levels.len() - 1) as f64;
    let still = source.clone();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
        let (dx, dy) = (x - center.0, y - center.1);
        let r = dx.hypot(dy);
        let u = n * r / big;
        if bump {
            let v = (0.5 + amplitude * (ripple_height(levels, u) - levels[levels.len() - 1]) / 2000.0).clamp(0.0, 1.0) as f32;
            px.copy_from_slice(&[v, v, v, 1.0]);
            return;
        }
        if r == 0.0 {
            return;
        }
        let s = amplitude / 10.0 * ripple_push(levels, u);
        px.copy_from_slice(&sample_bilinear(&still, x - s * dx / r, y - s * dy / r));
    });
}

/// D-391: Slant, this program's own reading of CycoreFX's CC Slant. A pixel's centre v below the
/// level line `floor` (in the buffer) reads the buffer at (x + v `tan`, floor + v / `s`), `tan`
/// the slant's tangent and `s` the height scale; `s` 0 draws nothing. With `color`, linear, the
/// colour replaces the picture's under its coverage. The settings are already valid.
pub(crate) fn slant(source: &mut WorkingBuffer, floor: f64, tan: f64, s: f64, color: Option<[f64; 3]>) {
    let w = source.width();
    if s <= 0.0 {
        source.data_mut().fill(0.0);
        return;
    }
    let still = source.clone();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
        let v = y - floor;
        let p = sample_bilinear(&still, x + v * tan, floor + v / s);
        match color {
            Some(c) => px.copy_from_slice(&[(c[0] * p[3] as f64) as f32, (c[1] * p[3] as f64) as f32, (c[2] * p[3] as f64) as f32, p[3]]),
            None => px.copy_from_slice(&p),
        }
    });
}

/// D-392: Smear, this program's own reading of CycoreFX's CC Smear. A pixel's centre P, s =
/// clamp((P - from) . v / |v|^2, 0, 1) of the way along the drag `v` and rho from its nearest
/// point on it, reads the buffer at P - w s v, w = (1 - (rho / radius)^2)^2 inside the radius
/// and 0 past it. Radius 0 or no drag leaves the buffer as it was. The settings are already
/// valid.
pub(crate) fn smear(source: &mut WorkingBuffer, from: (f64, f64), v: (f64, f64), radius: f64) {
    let w = source.width();
    let v2 = v.0 * v.0 + v.1 * v.1;
    if radius <= 0.0 || v2 == 0.0 {
        return;
    }
    let still = source.clone();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
        let s = (((x - from.0) * v.0 + (y - from.1) * v.1) / v2).clamp(0.0, 1.0);
        let rho = (x - from.0 - s * v.0).hypot(y - from.1 - s * v.1);
        if rho >= radius {
            return;
        }
        let k = (1.0 - (rho / radius).powi(2)).powi(2) * s;
        px.copy_from_slice(&sample_bilinear(&still, x - k * v.0, y - k * v.1));
    });
}

/// D-393: Split, this program's own reading of CycoreFX's CC Split. Along `a` to `b`, length L,
/// a pixel's centre t of the way along and d to the side has the gap's half width g = `split` /
/// 2 sin(pi t) (0 past the points); with D = g + L / 2, one with g <= |d| < D reads the buffer
/// (|d| - g) D / (D - g) out from the line on its own side, one inside the gap the line itself,
/// times the share of the pixel outside the gap. Split 0 or `a` on `b` leaves the buffer as it
/// was. The settings are already valid.
pub(crate) fn split(source: &mut WorkingBuffer, a: (f64, f64), b: (f64, f64), split: f64) {
    let w = source.width();
    let length = (b.0 - a.0).hypot(b.1 - a.1);
    if length == 0.0 || split <= 0.0 {
        return;
    }
    let (ux, uy) = ((b.0 - a.0) / length, (b.1 - a.1) / length);
    let still = source.clone();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as f64 + 0.5 - a.0, (i / w) as f64 + 0.5 - a.1);
        let along = x * ux + y * uy;
        let d = y * ux - x * uy;
        let t = along / length;
        let g = if (0.0..=1.0).contains(&t) { split / 2.0 * (std::f64::consts::PI * t).sin() } else { 0.0 };
        let m = d.abs();
        let cover = 1.0 - ((m + 0.5).min(g) - (m - 0.5).max(-g)).max(0.0);
        let far = g + length / 2.0;
        let p = if m >= far {
            sample_bilinear(&still, x + a.0, y + a.1)
        } else {
            let k = if m >= g { d.signum() * (m - g) * far / (far - g) } else { 0.0 };
            sample_bilinear(&still, a.0 + along * ux - k * uy, a.1 + along * uy + k * ux)
        };
        for ch in 0..4 {
            px[ch] = (p[ch] as f64 * cover) as f32;
        }
    });
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
    let Some((adj, det, (gx, gy))) = corner_map(pins, (w, h), origin) else {
        *source = WorkingBuffer::transparent(w, h);
        return (0, 0);
    };
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
/// The settings are already valid. D-320: the rings are ellipses that touch the drawing's sides,
/// or with `circle` circles, the outermost half the shorter side across.
pub(crate) fn polar_coordinates(source: &mut WorkingBuffer, interpolation: f64, to_polar: bool, circle: bool, origin: (usize, usize)) {
    let k = interpolation / 100.0;
    if k == 0.0 {
        return;
    }
    let (w, h) = (source.width(), source.height());
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    let (dw, dh) = (w as f64 - 2.0 * ox, h as f64 - 2.0 * oy);
    let turn = std::f64::consts::TAU;
    // The half-axes, and each as a share of the drawing's half, exactly 1 for the ellipse so
    // D-201's sums are unchanged.
    let (rx, ry) = if circle { (dw.min(dh) / 2.0, dw.min(dh) / 2.0) } else { (dw / 2.0, dh / 2.0) };
    let (fx, fy) = (rx / (dw / 2.0), ry / (dh / 2.0));
    let mut out = WorkingBuffer::transparent(w, h);
    let drawing = &*source;
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = ((i % w) as f64 + 0.5 - ox, (i / w) as f64 + 0.5 - oy);
            let (sx, sy) = if to_polar {
                let (nx, ny) = ((x - dw / 2.0) / rx, (y - dh / 2.0) / ry);
                let phi = nx.atan2(-ny);
                let phi = if phi < 0.0 { phi + turn } else { phi };
                (phi / turn * dw, nx.hypot(ny) * dh)
            } else {
                let (a, v) = (turn * x / dw, y / dh);
                (dw / 2.0 * (1.0 + v * a.sin() * fx), dh / 2.0 * (1.0 - v * a.cos() * fy))
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

/// D-210: a lens of `fov` degrees across the drawing's own width, height or diagonal (`span`),
/// its middle at `center`, in per cent of the drawing whose corner is at `origin` in the buffer.
/// With `reverse` off each pixel reads from farther out along its line from the middle, by the
/// tangent, and is empty past a quarter turn; on, from nearer in, by the arctangent. The buffer
/// keeps its size. The settings are already valid.
pub(crate) fn optics_compensation(
    source: &mut WorkingBuffer,
    fov: f64,
    reverse: bool,
    span: &str,
    center: [f64; 2],
    origin: (usize, usize),
) {
    if fov == 0.0 {
        return;
    }
    let (w, h) = (source.width(), source.height());
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    let (dw, dh) = (w as f64 - 2.0 * ox, h as f64 - 2.0 * oy);
    let (cx, cy) = (center[0] / 100.0 * dw + ox, center[1] / 100.0 * dh + oy);
    let r = match span {
        "vertical" => dh / 2.0,
        "diagonal" => dw.hypot(dh) / 2.0,
        _ => dw / 2.0,
    };
    let theta = fov.to_radians() / 2.0;
    let mut out = WorkingBuffer::transparent(w, h);
    let drawing = &*source;
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
            let (dx, dy) = (x - cx, y - cy);
            let d = dx.hypot(dy);
            let m = if d == 0.0 {
                0.0
            } else {
                let a = d / r * theta;
                if reverse {
                    r * a.atan() / theta / d
                } else if a >= std::f64::consts::FRAC_PI_2 {
                    return;
                } else {
                    r * a.tan() / theta / d
                }
            };
            px.copy_from_slice(&sample_bilinear(drawing, cx + m * dx, cy + m * dy));
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

/// D-207: a straight beam along the stretch of the line from `s` to `e` (buffer pixels) that
/// `length` and `time` pick, its thickness running from `t0` to `t1` along the whole line, its
/// edge softened over one pixel up to the whole thickness, its colour linear `inside` on the line
/// to `outside` at the edge; over the layer, or alone. Nothing grows. The settings are already
/// valid.
pub(crate) fn beam(
    source: &mut WorkingBuffer,
    [s, e]: [(f64, f64); 2],
    [length, time, t0, t1, softness]: [f64; 5],
    [inside, outside]: [[f64; 3]; 2],
    alone: bool,
) {
    let w = source.width();
    let l = length / 100.0;
    let a = time / 100.0 * (1.0 - l);
    let (ax, ay) = (s.0 + a * (e.0 - s.0), s.1 + a * (e.1 - s.1));
    let (dx, dy) = (l * (e.0 - s.0), l * (e.1 - s.1));
    let l2 = dx * dx + dy * dy;
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = ((i % w) as f64 + 0.5 - ax, (i / w) as f64 + 0.5 - ay);
            let t = if l2 == 0.0 { 0.0 } else { ((x * dx + y * dy) / l2).clamp(0.0, 1.0) };
            let d = (x - t * dx).hypot(y - t * dy);
            let r = (t0 + (a + t * l) * (t1 - t0)) / 2.0;
            let sw = (2.0 * r * softness / 100.0).max(1.0);
            let c = (((d + sw / 2.0).min(r) - (d - sw / 2.0).max(-r)) / sw).clamp(0.0, 1.0);
            let q = if r == 0.0 { 1.0 } else { (d / r).min(1.0) };
            let keep = if alone { 0.0 } else { 1.0 - c };
            for ch in 0..3 {
                px[ch] = (px[ch] as f64 * keep + ((1.0 - q) * inside[ch] + q * outside[ch]) * c) as f32;
            }
            px[3] = (px[3] as f64 * keep + c) as f32;
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
    let ((ux, uy), edge) = linear_edge(completion, angle, feather, (source.width(), source.height()), origin);
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
/// a buffer `w` by `h`. B-107's card takes the same.
pub(crate) fn linear_edge(completion: f64, angle: f64, feather: f64, (w, h): (usize, usize), origin: (usize, usize)) -> ((f64, f64), f64) {
    let (w0, h0) = ((w - 2 * origin.0) as f64, (h - 2 * origin.1) as f64);
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


/// D-214: the drawing, whose corner is at `origin` in the buffer, cut into blocks `block_width`
/// by `block_height` from that corner, past its edges too, each gone once `completion` per cent
/// passes its number from D-119's hash; each pixel keeps the share of the square `feather` across
/// round its centre that lies in kept blocks. The settings are already valid.
pub(crate) fn block_dissolve(
    source: &mut WorkingBuffer,
    completion: f64,
    block_width: f64,
    block_height: f64,
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
    let (w, h) = (source.width(), source.height());
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    let (bw, bh, half) = (block_width, block_height, feather / 2.0);
    let kept = dissolve_kept(completion);
    let (j0, ny, down) = if feather > 0.0 {
        dissolve_down((w, h), completion, (bw, bh), feather, origin)
    } else {
        (0, 0, Vec::new())
    };
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = (i % w, i / w);
            let (cx, cy) = (x as f64 - ox + 0.5, y as f64 - oy + 0.5);
            let k = if feather > 0.0 {
                let col = &down[x * (ny + 1)..(x + 1) * (ny + 1)];
                ((upto(col, cy + half, j0, bh) - upto(col, cy - half, j0, bh)) / (feather * feather)).clamp(0.0, 1.0)
            } else {
                kept((cx / bw).floor() as i64, (cy / bh).floor() as i64)
            };
            for v in px.iter_mut() {
                *v *= k as f32;
            }
        });
}

/// D-214: whether Block Dissolve at `completion` per cent keeps block (i, j), 1 or 0. B-226: the
/// card is handed these, worked out here.
pub(crate) fn dissolve_kept(completion: f64) -> impl Fn(i64, i64) -> f64 + Sync {
    let level = completion / 100.0;
    let base = crate::grade::mix(0);
    move |i, j| {
        if (crate::grade::unit(base, i, j, 0, 0) + 1.0) / 2.0 >= level {
            1.0
        } else {
            0.0
        }
    }
}

/// The length from the first block's start to `t` that is kept, along a line of blocks `size`
/// long whose running sums of kept are `sums`.
fn upto(sums: &[f64], t: f64, first: i64, size: f64) -> f64 {
    let s = t / size - first as f64;
    let a = (s.floor().max(0.0) as usize).min(sums.len() - 2);
    (sums[a] + (s - a as f64) * (sums[a + 1] - sums[a])) * size
}

/// D-214 with a feather: the square's kept area is a box across each row of blocks the squares
/// reach, then a box down the running sums of those. The first row of blocks, how many rows,
/// and each column's running sums, `rows + 1` of them a column. B-226: the card is handed these.
/// ponytail: the tables are the blocks' rows by the buffer's width, large only for tiny blocks
/// under a huge feather.
pub(crate) fn dissolve_down(
    (w, h): (usize, usize),
    completion: f64,
    (bw, bh): (f64, f64),
    feather: f64,
    origin: (usize, usize),
) -> (i64, usize, Vec<f64>) {
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    let half = feather / 2.0;
    let kept = dissolve_kept(completion);
    let first = |o: f64, size: f64| ((0.5 - o - half) / size).floor() as i64;
    let (i0, j0) = (first(ox, bw), first(oy, bh));
    let nx = (((w as f64 - ox - 0.5 + half) / bw).floor() as i64 - i0 + 1) as usize;
    let ny = (((h as f64 - oy - 0.5 + half) / bh).floor() as i64 - j0 + 1) as usize;
    let mut across = vec![0.0; ny * w];
    across.par_chunks_mut(w).enumerate().for_each(|(r, row)| {
        let j = j0 + r as i64;
        let mut sums = vec![0.0; nx + 1];
        for a in 0..nx {
            sums[a + 1] = sums[a] + kept(i0 + a as i64, j);
        }
        for (x, v) in row.iter_mut().enumerate() {
            let cx = x as f64 - ox + 0.5;
            *v = upto(&sums, cx + half, i0, bw) - upto(&sums, cx - half, i0, bw);
        }
    });
    let mut down = vec![0.0; w * (ny + 1)];
    down.par_chunks_mut(ny + 1).enumerate().for_each(|(x, col)| {
        for r in 0..ny {
            col[r + 1] = col[r] + across[r * w + x];
        }
    });
    (j0, ny, down)
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
    let ((cx, cy), r) = iris_circle(completion, center, feather, invert, (source.width(), source.height()), origin);
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
/// `origin` in a buffer `w` by `h`. B-107's card takes the same.
pub(crate) fn iris_circle(completion: f64, center: [f64; 2], feather: f64, invert: bool, (w, h): (usize, usize), origin: (usize, usize)) -> ((f64, f64), f64) {
    let (w0, h0) = ((w - 2 * origin.0) as f64, (h - 2 * origin.1) as f64);
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


/// D-204: Snowfall's three planes, nearest first: each one's cell, largest radius, sway, and how
/// far it has drifted and fallen by `frame`. B-151: the card falls the same way.
pub(crate) fn snow_planes([spacing, size, depth, speed, wind, wiggle]: [f64; 6], frame: i32) -> [[f64; 5]; 3] {
    let f = frame as f64;
    [0, 1, 2].map(|l| {
        let z = 1.0 - depth / 100.0 * l as f64 / 3.0;
        [spacing * z, size * z / 2.0, wiggle * z, wind * z * f, speed * z * f]
    })
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
    let planes = snow_planes([spacing, size, depth, speed, wind, wiggle], frame);
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
/// both cover. Nothing grows. The settings are already valid. D-324: `kind` and `[turbulence,
/// decay, conductivity]`, Advanced Lightning's; `ends[2]` the drawing's bottom edge below
/// `ends[0]`, for Vertical; and `blocks`, one per pixel of `source` (none when empty), where
/// Alpha Obstacle stops the bolt. D-329: `outside`, whether past the layer's edge blocks too (a
/// negative obstacle), and `around`, whether each main bolt goes round what blocks. D-334:
/// `soft`, whether the core fades from the middle to its edge. D-338: `forks`, "long" when the
/// main bolts' first forks run on down to the end, as Advanced Lightning's do at a low decay;
/// D-339: "full", the same but each as wide as the main bolt where it leaves it.
pub(crate) fn lightning_bolt(
    source: &mut WorkingBuffer,
    ends: [(f64, f64); 3],
    [jagged, detail, branches, width, glow, opacity, hold, seed]: [f64; 8],
    (kind, [turbulence, decay, conductivity], forks): (&str, [f64; 3], &str),
    (blocks, outside, around): (&[bool], bool, bool),
    ([core, halo], soft): ([[f64; 3]; 2], bool),
    frame: i32,
) {
    if opacity == 0.0 || (width == 0.0 && glow == 0.0) {
        return;
    }
    let size = (source.width(), source.height());
    let segs = bolt_list(size, ends, [jagged, detail, branches, hold, seed], (kind, [turbulence, decay, conductivity], forks), (blocks, outside, around), frame);
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
                let (c, g) = bolt_light(s, width, glow, soft, x as f64 + 0.5, py);
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

/// D-190's segments for a frame, each `[px, py, qx, qy, wP, wQ]` in a buffer `size`: what
/// `lightning_bolt` lights, which the card (B-225) is handed as it is.
pub(crate) fn bolt_list(
    size: (usize, usize),
    ends: [(f64, f64); 3],
    [jagged, detail, branches, hold, seed]: [f64; 5],
    (kind, [turbulence, decay, conductivity], forks): (&str, [f64; 3], &str),
    (blocks, outside, around): (&[bool], bool, bool),
    frame: i32,
) -> Vec<[f64; 6]> {
    let m = (frame as i64).div_euclid(hold.floor() as i64);
    // D-324's Conductivity State: the numbers of the whole state below and the one above, mixed
    // by the smoothed part between; state 0 is D-190's seed alone.
    let (n, base) = (conductivity.floor(), crate::grade::mix(seed.floor() as u64));
    let t = conductivity - n;
    let s = t * t * (3.0 - 2.0 * t);
    let below = if n == 0.0 { base } else { crate::grade::mix(base ^ n as u64) };
    let above = crate::grade::mix(base ^ (n as u64 + 1));
    let r = |key: i64, b: i64, j: u64| {
        let v = crate::grade::unit(below, key, b, m, j);
        if s == 0.0 { v } else { v + s * (crate::grade::unit(above, key, b, m, j) - v) }
    };
    let blocked = |x: f64, y: f64| {
        let (i, j) = (x.floor(), y.floor());
        if i >= 0.0 && i < size.0 as f64 && j >= 0.0 && j < size.1 as f64 { blocks[j as usize * size.0 + i as usize] } else { outside }
    };
    let route = (around && !blocks.is_empty()).then_some((&blocked as &dyn Fn(f64, f64) -> bool, size));
    let mut segs = bolt_segments((kind, forks), ends, [jagged, branches, turbulence, decay], detail.floor() as u32, &r, route);
    if !blocks.is_empty() {
        segs = bolt_stop(segs, &blocked, route.is_some());
    }
    segs.into_iter().map(|(p, q, wp, wq, ..)| [p.0, p.1, q.0, q.1, wp, wq]).collect()
}

/// D-190's segment `[px, py, qx, qy, wP, wQ]`: its core and glow at the point (x, y). The core is
/// the share of the pixel's width across the line that the core covers; D-334's soft core is
/// 1 - distance / half its width there instead, taken the same way.
fn bolt_light(s: &[f64; 6], width: f64, glow: f64, soft: bool, x: f64, y: f64) -> (f64, f64) {
    let (dx, dy) = (s[2] - s[0], s[3] - s[1]);
    let l2 = dx * dx + dy * dy;
    let t = if l2 == 0.0 { 0.0 } else { (((x - s[0]) * dx + (y - s[1]) * dy) / l2).clamp(0.0, 1.0) };
    let d = (x - s[0] - t * dx).hypot(y - s[1] - t * dy);
    let w = s[4] + t * (s[5] - s[4]);
    let half = w * width / 2.0;
    let core = if soft {
        // The fall from 0 to u, odd in u: u - u^2 / (2 half) up to the edge, half / 2 past it.
        let rise = |u: f64| {
            let a = u.abs().min(half);
            u.signum() * (a - a * a / (2.0 * half))
        };
        if half > 0.0 { rise(d + 0.5) - rise(d - 0.5) } else { 0.0 }
    } else {
        ((d + 0.5).min(half) - (d - 0.5).max(-half)).clamp(0.0, 1.0)
    };
    let r = w * glow;
    (core, if d < r { w * (1.0 - d / r).powi(2) } else { 0.0 })
}

/// A segment of D-190's bolt: (P, Q, wP, wQ, depth, bolt, key, the bolt it forks from). A key and
/// a bolt name a segment for good, so a fork or a finer halving never moves what is already drawn.
type BoltSeg = ((f64, f64), (f64, f64), f64, f64, u32, i64, i64, i64);

/// D-190's bolt as segments: each start bolt halved `detail` times, each middle pushed aside by
/// `r(key, bolt, j)`, with forks three deep. Worked in double precision. D-324: `kind` picks the
/// start bolts from `ends` (start, end, and the bottom edge below the start) and how a fork
/// turns and starts; turbulence pushes harder each halving and forks more; decay thins each
/// start bolt to 1 - decay / 100 at its end. D-329: with `around` (what blocks, and the layer's
/// size) each start bolt is first routed round what blocks, and a main bolt's middle is pushed
/// only where both halves stay clear. D-338: with `long`, a fork leaving a main bolt in one of
/// the first three halvings leaves along that main bolt's own way, from its start to its end,
/// turned 10 to 30 degrees, and goes on until it has covered 1 - decay / 100 of what is left of
/// that way, thinning by the same share; Alpha Obstacle then stops it where the ground starts.
/// D-339: with `full`, the same, but starting at the main bolt's own weight.
fn bolt_segments(
    (kind, forks): (&str, &str),
    [o, d, bottom]: [(f64, f64); 3],
    [jagged, branches, turbulence, decay]: [f64; 4],
    detail: u32,
    r: &dyn Fn(i64, i64, u64) -> f64,
    around: Option<(&dyn Fn(f64, f64) -> bool, (usize, usize))>,
) -> Vec<BoltSeg> {
    let kd = 1.0 - decay / 100.0;
    let one = |p, q, b| (p, q, 1.0, kd, 0u32, b, 1i64, b);
    let turn = |v: (f64, f64), deg: f64| {
        let (c, s) = (deg.to_radians().cos(), deg.to_radians().sin());
        (v.0 * c - v.1 * s, v.0 * s + v.1 * c)
    };
    let mut segs = match kind {
        "bouncy" => vec![one(o, d, 0), one(d, o, -1), one(o, d, -2)],
        "omni" => (0..6i64)
            .map(|i| {
                let t = turn((d.0 - o.0, d.1 - o.1), 60.0 * i as f64);
                one(o, (o.0 + t.0, o.1 + t.1), -i)
            })
            .collect(),
        "anywhere" => {
            let a = (180.0 * (r(1, -1, 0) + 1.0)).to_radians();
            let l = (d.0 - o.0).hypot(d.1 - o.1) * (r(1, -1, 1) + 1.0) / 2.0;
            vec![one(o, (o.0 + l * a.cos(), o.1 + l * a.sin()), 0)]
        }
        "vertical" => vec![one(o, bottom, 0)],
        "two_way" => {
            let mid = ((o.0 + d.0) / 2.0, (o.1 + d.1) / 2.0);
            vec![one(o, mid, 0), one(d, mid, -1)]
        }
        _ => vec![one(o, d, 0)],
    };
    if let Some((blocked, size)) = around {
        segs = segs.into_iter().flat_map(|root| bolt_pieces(root, blocked, size)).collect();
    }
    // D-338: each main bolt's own way, by its name.
    let ways: Vec<(i64, (f64, f64), (f64, f64))> = segs.iter().map(|s| (s.5, s.0, s.1)).collect();
    for i in 0..detail {
        let mut out = Vec::with_capacity(segs.len() * 3);
        for (p, q, wp, wq, depth, b, key, parent) in segs {
            let (dx, dy) = (q.0 - p.0, q.1 - p.1);
            let l = dx.hypot(dy);
            if l == 0.0 {
                out.push((p, q, wp, wq, depth, b, key, parent));
                continue;
            }
            let a = r(key, b, 0) * jagged / 100.0 * l / 2.0 * (1.0 + turbulence / 100.0 * i as f64 / 2.0);
            let mut mid = ((p.0 + q.0) / 2.0 - dy / l * a, (p.1 + q.1) / 2.0 + dx / l * a);
            if let Some((blocked, _)) = around.filter(|_| depth == 0) {
                if !(bolt_clear(p, mid, blocked) && bolt_clear(mid, q, blocked)) {
                    mid = ((p.0 + q.0) / 2.0, (p.1 + q.1) / 2.0);
                }
            }
            let wm = (wp + wq) / 2.0;
            out.push((p, mid, wp, wm, depth, b, 2 * key, parent));
            out.push((mid, q, wm, wq, depth, b, 2 * key + 1, parent));
            if depth < 3 && (r(key, b, 1) + 1.0) / 2.0 < branches / 100.0 * (1.0 + turbulence / 100.0) {
                let mut way = (dx / l, dy / l);
                if kind == "strike" {
                    let (tx, ty) = (d.0 - mid.0, d.1 - mid.1);
                    let tl = tx.hypot(ty);
                    if tl > 0.0 {
                        way = (tx / tl, ty / tl);
                    }
                }
                let side = if r(key, b, 3) < 0.0 { -1.0 } else { 1.0 };
                let w0 = if kind == "breaking" { wm } else { wm / 2.0 };
                // D-338: the main bolt's own way, and how much of it is left past the fork.
                let run = ways.iter().find(|w| w.0 == b).filter(|_| forks != "short" && depth == 0 && i < 3).and_then(|&(_, o, e)| {
                    let (ux, uy) = (e.0 - o.0, e.1 - o.1);
                    let ul = ux.hypot(uy);
                    let u = (ux / ul, uy / ul);
                    let left = (e.0 - mid.0) * u.0 + (e.1 - mid.1) * u.1;
                    (ul > 0.0 && left > 0.0).then_some((u, left))
                });
                if let Some((u, left)) = run {
                    let v = turn(u, (10.0 + 10.0 * (r(key, b, 2) + 1.0)) * side);
                    let lb = kd * left / (v.0 * u.0 + v.1 * u.1);
                    let ws = if forks == "full" { wm } else { w0 };
                    out.push((mid, (mid.0 + v.0 * lb, mid.1 + v.1 * lb), ws, ws * kd, depth + 1, 1024 * b + key, 1, b));
                } else {
                    let v = turn(way, (15.0 + 15.0 * (r(key, b, 2) + 1.0)) * side);
                    let lb = l * (0.3 + 0.15 * (r(key, b, 4) + 1.0));
                    out.push((mid, (mid.0 + v.0 * lb, mid.1 + v.1 * lb), w0, 0.0, depth + 1, 1024 * b + key, 1, b));
                }
            }
        }
        segs = out;
    }
    segs
}

/// D-324's walk from `p` to `q`: `n`, and the point `j / n` of the way, `j` = 0 to `n`, a pixel
/// or less apart.
fn bolt_walk(p: (f64, f64), q: (f64, f64)) -> (usize, impl Fn(usize) -> (f64, f64)) {
    let n = ((q.0 - p.0).hypot(q.1 - p.1).ceil() as usize).max(1);
    (n, move |j: usize| (p.0 + (q.0 - p.0) * (j as f64 / n as f64), p.1 + (q.1 - p.1) * (j as f64 / n as f64)))
}

/// D-329: no point of D-324's walk from `p` to `q` blocks.
fn bolt_clear(p: (f64, f64), q: (f64, f64), blocked: &dyn Fn(f64, f64) -> bool) -> bool {
    let (n, at) = bolt_walk(p, q);
    (0..=n).all(|j| {
        let x = at(j);
        !blocked(x.0, x.1)
    })
}

/// D-329: a start bolt routed round what blocks, as pieces, each a bolt of its own; none when its
/// start blocks. The way is a breadth-first search over the layer's pixels from the start's to
/// the end's (or to the nearest the search reaches), pulled straight wherever the line is clear.
// ponytail: one search over the whole layer per start bolt, and the straightening tests every
// later point, O(cells^2) on a long way; fine at a few bolts, bound the search if it shows.
fn bolt_pieces(root: BoltSeg, blocked: &dyn Fn(f64, f64) -> bool, (w, h): (usize, usize)) -> Vec<BoltSeg> {
    let (o, d, _, kd, _, b, _, _) = root;
    let cell = |p: (f64, f64)| (p.0.floor().clamp(0.0, w as f64 - 1.0) as i64, p.1.floor().clamp(0.0, h as f64 - 1.0) as i64);
    let free = |(i, j): (i64, i64)| i >= 0 && i < w as i64 && j >= 0 && j < h as i64 && !blocked(i as f64 + 0.5, j as f64 + 0.5);
    let at = |(i, j): (i64, i64)| j as usize * w + i as usize;
    let (s, t) = (cell(o), cell(d));
    if !free(s) {
        return Vec::new();
    }
    let far = |c: (i64, i64)| (c.0 - t.0).pow(2) + (c.1 - t.1).pow(2);
    let mut parent = vec![usize::MAX; w * h];
    parent[at(s)] = at(s);
    let (mut queue, mut best) = (std::collections::VecDeque::from([s]), s);
    'search: while s != t {
        let Some((ci, cj)) = queue.pop_front() else { break };
        for (di, dj) in [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)] {
            let n = (ci + di, cj + dj);
            if !free(n) || parent[at(n)] != usize::MAX || (di != 0 && dj != 0 && !(free((ci + di, cj)) && free((ci, cj + dj)))) {
                continue;
            }
            parent[at(n)] = at((ci, cj));
            queue.push_back(n);
            if far(n) < far(best) {
                best = n;
            }
            if n == t {
                break 'search;
            }
        }
    }
    let mut way = vec![best];
    let mut c = at(best);
    while parent[c] != c {
        c = parent[c];
        way.push(((c % w) as i64, (c / w) as i64));
    }
    way.reverse();
    let middle = |(i, j): (i64, i64)| (i as f64 + 0.5, j as f64 + 0.5);
    let mut pts = vec![o];
    pts.extend(way.iter().skip(1).take(way.len().saturating_sub(2)).map(|&c| middle(c)));
    pts.push(if best == t { d } else { middle(best) });
    let mut kept = vec![pts[0]];
    let mut i = 0;
    while i < pts.len() - 1 {
        let mut j = i + 1;
        while j + 1 < pts.len() && bolt_clear(pts[i], pts[j + 1], blocked) {
            j += 1;
        }
        kept.push(pts[j]);
        i = j;
    }
    let lengths: Vec<f64> = kept.windows(2).map(|e| (e[1].0 - e[0].0).hypot(e[1].1 - e[0].1)).collect();
    let total: f64 = lengths.iter().sum();
    let mut along = 0.0;
    let mut out = Vec::with_capacity(lengths.len());
    for (k, e) in kept.windows(2).enumerate() {
        let ws = if k == 0 { 1.0 } else { 1.0 + (kd - 1.0) * along / total };
        along += lengths[k];
        let we = if k == lengths.len() - 1 { kd } else { 1.0 + (kd - 1.0) * along / total };
        let bp = if k == 0 { b } else { b - 8 * k as i64 };
        out.push((e[0], e[1], ws, we, 0, bp, 1, bp));
    }
    out
}

/// D-324's Alpha Obstacle: each bolt, then each fork, walked from its start at points a pixel or
/// less apart, ends at the first point `blocked`; the rest of it goes, and so does every fork
/// whose start is no longer on what is left of its parent. D-329: with `keep_main` the main bolts
/// (depth 0), already routed round, are left whole.
fn bolt_stop(segs: Vec<BoltSeg>, blocked: impl Fn(f64, f64) -> bool, keep_main: bool) -> Vec<BoltSeg> {
    use std::collections::{HashMap, HashSet};
    let mut kept: HashMap<i64, Vec<(f64, f64)>> = HashMap::new();
    let (mut done, mut started) = (HashSet::new(), HashSet::new());
    let mut out = Vec::new();
    for depth in 0..4 {
        for &(p, q, wp, wq, dd, b, key, parent) in &segs {
            if dd != depth || done.contains(&b) {
                continue;
            }
            if started.insert(b) && dd > 0 && !kept.get(&parent).is_some_and(|k| k.contains(&p)) {
                done.insert(b);
                continue;
            }
            let (n, at) = bolt_walk(p, q);
            let hit = (0..=n).find(|&j| {
                let x = at(j);
                !(keep_main && dd == 0) && blocked(x.0, x.1)
            });
            let mine = kept.entry(b).or_default();
            match hit {
                None => {
                    out.push((p, q, wp, wq, dd, b, key, parent));
                    mine.extend([p, q]);
                }
                Some(j) => {
                    done.insert(b);
                    if j > 0 {
                        let x = at(j);
                        out.push((p, x, wp, wp + j as f64 / n as f64 * (wq - wp), dd, b, key, parent));
                        mine.extend([p, x]);
                    }
                }
            }
        }
    }
    out
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
/// Outside the picture is clear, or with `wrap`, the picture's far side. D-315: with `expand`
/// and not `wrap`, the buffer first grows clear by the larger maximum rounded up, returned, and a
/// place past the map's edge reads the map's nearest edge pixel, so a push carries on outward.
pub(crate) fn displacement_map(
    source: &mut WorkingBuffer,
    map: &WorkingBuffer,
    origin: (usize, usize),
    words: [&str; 2],
    most: [f64; 2],
    (wrap, expand): (bool, bool),
) -> usize {
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
    let g = if expand && !wrap { most[0].abs().max(most[1].abs()).ceil() as usize } else { 0 };
    let from = std::mem::replace(source, WorkingBuffer::transparent(0, 0));
    let (w, h) = (from.width(), from.height());
    let (mw, mh) = (map.width(), map.height());
    let mut out = WorkingBuffer::transparent(w + 2 * g, h + 2 * g);
    let (ox, oy) = ((origin.0 + g) as i64, (origin.1 + g) as i64);
    out.data_mut().par_chunks_mut((w + 2 * g) * 4).enumerate().for_each(|(y, row)| {
        for (x, o) in row.chunks_mut(4).enumerate() {
            let (mx, my) = (x as i64 - ox, y as i64 - oy);
            let m = if mx >= 0 && my >= 0 && (mx as usize) < mw && (my as usize) < mh {
                map.pixel(mx as usize, my as usize)
            } else if g > 0 && mw > 0 && mh > 0 {
                map.pixel(mx.clamp(0, mw as i64 - 1) as usize, my.clamp(0, mh as i64 - 1) as usize)
            } else {
                [0.0; 4]
            };
            let sx = x as f64 + 0.5 + (2.0 * value(words[0], m) - 1.0) * most[0] - g as f64;
            let sy = y as f64 + 0.5 + (2.0 * value(words[1], m) - 1.0) * most[1] - g as f64;
            let p = if wrap { wrapped(&from, (w, h), sx, sy) } else { sample_bilinear(&from, sx, sy) };
            o.copy_from_slice(&p);
        }
    });
    *source = out;
    g
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

/// D-348: the pass's pixel under the drawing's pixel `i` of a buffer `w` wide with the drawing's
/// top-left at `origin`, as an index into `pass`'s data, or `None` outside the drawing.
fn pass_at(pass: &WorkingBuffer, i: usize, w: usize, origin: (usize, usize)) -> Option<usize> {
    let (x, y) = ((i % w).wrapping_sub(origin.0), (i / w).wrapping_sub(origin.1));
    (x < pass.width() && y < pass.height()).then(|| (y * pass.width() + x) * 4)
}

/// D-348: Pass Extract. Each of the pass's three values `v` as `(v - black) / (white - black)`,
/// or a cut at `white` where the two are equal, turned over by `invert` and then held to 0..1 by
/// `clamp`: an opaque picture over the drawing, transparent outside it.
pub(crate) fn pass_extract(
    source: &mut WorkingBuffer,
    pass: &WorkingBuffer,
    origin: (usize, usize),
    black: f64,
    white: f64,
    invert: bool,
    clamp: bool,
) {
    let w = source.width();
    let p = pass.data();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let Some(j) = pass_at(pass, i, w, origin) else {
            px.fill(0.0);
            return;
        };
        for c in 0..3 {
            let v = p[j + c] as f64;
            let o = if white != black {
                (v - black) / (white - black)
            } else if v >= white {
                1.0
            } else {
                0.0
            };
            let o = if invert { 1.0 - o } else { o };
            px[c] = if clamp { o.clamp(0.0, 1.0) } else { o } as f32;
        }
        px[3] = 1.0;
    });
}

/// D-348: Depth Key. The drawing kept where its depth is at least `depth`, with a soft edge
/// `feather` wide centred on it, or the other way with `invert`; transparent outside it.
pub(crate) fn depth_key(
    source: &mut WorkingBuffer,
    pass: &WorkingBuffer,
    origin: (usize, usize),
    depth: f64,
    feather: f64,
    invert: bool,
) {
    let w = source.width();
    let p = pass.data();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let Some(j) = pass_at(pass, i, w, origin) else {
            px.fill(0.0);
            return;
        };
        let z = p[j] as f64;
        let k = if feather > 0.0 {
            ((z - depth) / feather + 0.5).clamp(0.0, 1.0)
        } else if z >= depth {
            1.0
        } else {
            0.0
        };
        let k = if invert { 1.0 - k } else { k };
        for v in px.iter_mut() {
            *v = (*v as f64 * k) as f32;
        }
    });
}

/// D-349: ID Key. The drawing kept where its id is within a half of `id`, or the other way with
/// `invert`, the matte blurred `feather` pixels with its edges held; transparent outside it.
pub(crate) fn id_key(
    source: &mut WorkingBuffer,
    pass: &WorkingBuffer,
    origin: (usize, usize),
    id: f64,
    feather: f64,
    invert: bool,
) {
    let mut matte = WorkingBuffer::transparent(pass.width(), pass.height());
    matte.data_mut().par_chunks_exact_mut(4).zip(pass.data().par_chunks_exact(4)).for_each(|(m, p)| {
        let hit = (p[0] as f64 - id).abs() < 0.5;
        m.fill(if hit != invert { 1.0 } else { 0.0 });
    });
    if feather > 0.0 {
        crate::effects::held_blur_axes(&mut matte, &crate::effects::gaussian_weights(feather), (true, true));
    }
    let w = source.width();
    let m = matte.data();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let k = pass_at(pass, i, w, origin).map_or(0.0, |j| m[j] as f64);
        for v in px.iter_mut() {
            *v = (*v as f64 * k) as f32;
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
    let waves = radio_wave_list(s);
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

/// D-200's waves alive at the frame, oldest first: (centre x, centre y, corner radius, first
/// corner, half width, strength). The card (D-345) is handed them as they are.
pub(crate) fn radio_wave_list(s: &RadioWaves) -> Vec<(f64, f64, f64, f64, f64, f64)> {
    let fade = |t: f64, time: f64| if time > 0.0 { (t / time).min(1.0) } else { 1.0 };
    let (ux, uy) = crate::blurs::along(s.direction);
    let f = s.frame as f64;
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
    waves
}
