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

/// D-409: Chromatic Aberration's new form. Each channel is sampled at its own shift, `scales`
/// (red, green, blue, as fractions) of `amount` pixels: radial, outward from `center` and
/// growing as the distance times (distance / half diagonal)^(2 falloff / 100); offset, along
/// [`crate::blurs::along`]`(angle)`. With fringe blur each channel is the mean of
/// [`crate::effects::lens_aberration_counts`] samples evenly along its shift, `fringe_blur`
/// (a fraction) of its length, summed in order and divided once. The covering is the largest of
/// the three channels'. The layer does not grow. The settings are already valid.
#[allow(clippy::too_many_arguments)]
pub(crate) fn lens_chromatic_aberration(
    source: &mut WorkingBuffer,
    offset: bool,
    amount: f64,
    center: [f64; 2],
    angle: f64,
    falloff: f64,
    scales: [f64; 3],
    fringe_blur: f64,
    origin: (usize, usize),
) {
    if amount == 0.0 || scales == [0.0; 3] {
        return;
    }
    let (cx, cy) = crate::effects::radial_center(center, (source.width(), source.height()), origin);
    let w0 = (source.width() - 2 * origin.0) as f64;
    let h0 = (source.height() - 2 * origin.1) as f64;
    let rc = (w0 * w0 + h0 * h0).sqrt() / 2.0;
    let k = amount / rc;
    let e = 2.0 * falloff / 100.0;
    let (ux, uy) = crate::blurs::along(angle);
    let counts = crate::effects::lens_aberration_counts(amount, scales, fringe_blur);
    let drawing = source.clone();
    let w = source.width();
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
            let (dx, dy) = (x - cx, y - cy);
            let g = if e == 0.0 {
                1.0
            } else {
                let rho = (dx * dx + dy * dy).sqrt() / rc;
                if rho > 0.0 { rho.powf(e) } else { 0.0 }
            };
            let mut chans = [[0.0f32; 4]; 3];
            for c in 0..3 {
                let (s, n) = (scales[c], counts[c]);
                let mut total = [0.0f32; 4];
                for j in 0..n {
                    let t = if n > 1 { j as f64 / (n - 1) as f64 - 0.5 } else { 0.0 };
                    let q = if offset {
                        let m = amount * (t * fringe_blur * s.abs() - s);
                        sample_bilinear(&drawing, x + ux * m, y + uy * m)
                    } else {
                        let f = 1.0 - s * k * g + t * fringe_blur * s.abs() * k * g;
                        sample_bilinear(&drawing, cx + dx * f, cy + dy * f)
                    };
                    for v in 0..4 {
                        total[v] += q[v];
                    }
                }
                for v in 0..4 {
                    chans[c][v] = total[v] / n as f32;
                }
            }
            px[0] = chans[0][0];
            px[1] = chans[1][1];
            px[2] = chans[2][2];
            px[3] = chans[0][3].max(chans[1][3]).max(chans[2][3]);
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
    add_rays(source, &rays, intensity, color.map(crate::grade::to_linear), false, 0);
}

/// D-422: Light Burst. The whole layer is the light, zoomed about `center` by `sweep` with
/// amount `length`, and added on as Light Rays' are, `k` times; with `tint`, an encoded colour,
/// the rays are that colour at their covering, otherwise the layer's own colours.
pub(crate) fn light_burst(
    source: &mut WorkingBuffer,
    center: (f64, f64),
    k: f64,
    length: f64,
    sweep: crate::blurs::Sweep,
    tint: Option<[f64; 3]>,
) {
    if k == 0.0 {
        return;
    }
    let mut rays = source.clone();
    crate::blurs::radial_blur(&mut rays, false, length, center, false, Some(sweep));
    let c = tint.map_or([1.0; 3], |c| c.map(crate::grade::to_linear));
    add_rays(source, &rays, k, c, tint.is_some(), 0);
}

/// D-423: Light Rays with CC Light Rays' controls. The light is the layer times each pixel's
/// share of a source `radius` pixels about `center` (round, or with `square`, the sine and cosine
/// of its turn, a square), zoomed out from the centre at amount 100 and spun `turn` degrees, then
/// laid on by `mode` (as [`crate::effects::ray_mode`]) `k` times; with `tint`, an encoded
/// colour, the rays are that colour at their covering. The settings are already valid.
#[allow(clippy::too_many_arguments)]
pub(crate) fn cc_light_rays(
    source: &mut WorkingBuffer,
    center: (f64, f64),
    k: f64,
    radius: f64,
    square: Option<(f64, f64)>,
    turn: f64,
    tint: Option<[f64; 3]>,
    mode: u8,
) {
    if k == 0.0 {
        return;
    }
    let mut rays = source.clone();
    let w = rays.width();
    rays.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let m = light_share(((i % w) as f64 + 0.5 - center.0, (i / w) as f64 + 0.5 - center.1), radius, square);
        for v in px.iter_mut() {
            *v = (*v as f64 * m) as f32;
        }
    });
    crate::blurs::radial_blur(&mut rays, false, 100.0, center, false, None);
    crate::blurs::radial_blur(&mut rays, true, turn, center, false, None);
    let c = tint.map_or([1.0; 3], |c| c.map(crate::grade::to_linear));
    add_rays(source, &rays, k, c, tint.is_some(), mode);
}

/// D-423: how much of a pixel `d` from the centre the light source takes, a pixel-wide soft
/// edge at `radius`: round, or with `square` (sine, cosine), max(|u|, |v|) of `d` turned back.
fn light_share(d: (f64, f64), radius: f64, square: Option<(f64, f64)>) -> f64 {
    let (dx, dy) = d;
    let distance = match square {
        Some((s, c)) => (dx * c + dy * s).abs().max((-dx * s + dy * c).abs()),
        None => (dx * dx + dy * dy).sqrt(),
    };
    (radius + 0.5 - distance).clamp(0.0, 1.0)
}

/// D-124's last step, shared with D-422 and D-423: with `mode` 0, O + `intensity` C R in
/// colour, the covering min(1, O.a + `intensity` R.a); with `from_alpha`, R's covering stands
/// for its colour. D-423's other modes lay Q = `intensity` C R, covering a = min(1, `intensity`
/// R.a), on O: 1 over it, Q + O (1 - a); 2 screened, O + Q - O Q; 3 the larger.
fn add_rays(source: &mut WorkingBuffer, rays: &WorkingBuffer, intensity: f64, c: [f64; 3], from_alpha: bool, mode: u8) {
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .zip(rays.data().par_chunks_exact(4))
        .for_each(|(px, r)| {
            let (o, a) = (px[3] as f64, (intensity * r[3] as f64).min(1.0));
            for k in 0..3 {
                let v = if from_alpha { r[3] } else { r[k] };
                let (p, q) = (px[k] as f64, intensity * c[k] * v as f64);
                px[k] = match mode {
                    1 => q + p * (1.0 - a),
                    2 => p + q - p * q,
                    3 => p.max(q),
                    _ => p + q,
                } as f32;
            }
            px[3] = match mode {
                1 => a + o * (1.0 - a),
                2 => o + a - o * a,
                3 => o.max(a),
                _ => (o + intensity * r[3] as f64).min(1.0),
            } as f32;
        });
}

/// D-127: each pixel read from a place pushed up to `amount` pixels by two channels of D-128's
/// fractal field, `size` pixels a cell, `octaves` deep at depth `z`, fixed to the drawing's own
/// space (its corner at `origin` in `source`), slid by `drift` (D-411). With transparent edges the layer first grows by
/// the amount rounded up, returned; with `repeat` a place past the edge reads the edge. The
/// settings are already valid; amount 0 changes nothing.
#[allow(clippy::too_many_arguments)]
pub(crate) fn turbulent_displace(
    source: &mut WorkingBuffer,
    amount: f64,
    size: f64,
    octaves: usize,
    seed: u64,
    z: f64,
    repeat: bool,
    origin: (usize, usize),
    (displacement, pin): (&str, bool),
    drift: (f64, f64),
) -> usize {
    if amount == 0.0 {
        return 0;
    }
    let g = if repeat { 0 } else { amount.ceil() as usize };
    let (iw, ih) = (source.width() as f64, source.height() as f64);
    let (w, h) = (source.width() + 2 * g, source.height() + 2 * g);
    let (ox, oy) = ((origin.0 + g) as f64, (origin.1 + g) as f64);
    let base = crate::grade::mix(seed);
    let mut out = WorkingBuffer::transparent(w, h);
    let drawing = &*source;
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
            let p = ((x - ox - drift.0) / size, (y - oy - drift.1) / size, z);
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

/// Each run of `n` columns, or of rows, that falls in one block `size` wide laid from `o`, as
/// D-144's Mosaic and D-449's Block Load cut them.
pub(crate) fn block_runs(n: usize, o: usize, size: f64) -> Vec<std::ops::Range<usize>> {
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
}

/// D-144: every block `size` pixels wide, laid from the drawing's corner `(ox, oy)` in `source`,
/// painted the mean of the buffer's premultiplied pixels in it, empty ones counted as empty. A
/// block past the right or bottom edge is cut short. Size 1 or less changes nothing.
pub(crate) fn mosaic(source: &mut WorkingBuffer, size: f64, (ox, oy): (usize, usize)) {
    if size <= 1.0 {
        return;
    }
    let (w, h) = (source.width(), source.height());
    let (columns, rows) = (block_runs(w, ox, size), block_runs(h, oy, size));
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

/// D-449: Block Load's pass under way at `completion` (0 to 100) with `scans` (its whole part):
/// the power of two its blocks are wide, whether the picture before it is empty (Start Cleared's
/// first pass), and how far through it is. None when every pass is done, the layer as it is.
pub(crate) fn block_load_pass(completion: f64, scans: f64, cleared: bool) -> Option<(i32, bool, f64)> {
    let n = scans.floor() as i32;
    let first = if cleared { 0 } else { 1 };
    let t = n + 1 - first;
    let q = completion / 100.0 * t as f64;
    let j = q.floor();
    if j >= t as f64 {
        return None;
    }
    let k = first + j as i32;
    Some((n - k, k == 0, q - j))
}

/// D-449: the first buffer row the pass under way has not reached: of the rows of blocks `size`
/// wide laid from `oy`, the first floor(f m) are done.
pub(crate) fn block_load_edge(h: usize, oy: usize, size: f64, f: f64) -> usize {
    let rows = block_runs(h, oy, size);
    let done = (f * rows.len() as f64).floor() as usize;
    rows.get(done).map_or(h, |r| r.start)
}

/// D-449: after After Effects' CC Block Load. The pass under way paints blocks 2^e `pixel` wide
/// (D-144's, or blended between their centres when `bilinear`) down to its edge; below it the
/// pass before, blocks twice as wide, or nothing before Start Cleared's first pass. The layer
/// never grows.
#[allow(clippy::too_many_arguments)]
pub(crate) fn block_load(source: &mut WorkingBuffer, completion: f64, scans: f64, cleared: bool, bilinear: bool, pixel: f64, origin: (usize, usize)) {
    let Some((e, empty, f)) = block_load_pass(completion, scans, cleared) else {
        return;
    };
    let size = 2f64.powi(e) * pixel;
    let (w, h) = (source.width(), source.height());
    let edge = block_load_edge(h, origin.1, size, f);
    let before = (edge < h && !empty).then(|| {
        let mut b = source.clone();
        block_picture(&mut b, size * 2.0, bilinear, origin);
        b
    });
    block_picture(source, size, bilinear, origin);
    if edge < h {
        let below = &mut source.data_mut()[edge * w * 4..];
        match &before {
            Some(b) => below.copy_from_slice(&b.data()[edge * w * 4..]),
            None => below.fill(0.0),
        }
    }
}

/// D-461: Hex Tile's numbers for a buffer `w` by `h` whose drawing's corner is at `origin`
/// (document 21): the centre and the honeycomb's origin in the buffer, the turn's cosine and
/// sine, the three factors to the tile's axial place, the two from it to the tile's middle, the
/// smearing as a share, the render (0 tile, 1 fold aligned, 2 fold seamless), sqrt(3) / 2, the
/// cosine and sine of each of the six triangles' turns, and a 0 for the card's rounded products.
pub(crate) fn hex_tile_setup(
    render: &str,
    radius: f64,
    center: [f64; 2],
    lock: bool,
    rotate: f64,
    smearing: f64,
    size: (usize, usize),
    origin: (usize, usize),
) -> Vec<f64> {
    let (cx, cy) = crate::effects::radial_center(center, size, origin);
    let (ox, oy) = if lock { crate::effects::radial_center([50.0, 50.0], size, origin) } else { (cx, cy) };
    let a = rotate.to_radians();
    let r3 = 3.0f64.sqrt();
    let mode = match render {
        "fold_aligned" => 1.0,
        "fold_seamless" => 2.0,
        _ => 0.0,
    };
    let mut k = vec![
        cx,
        cy,
        ox,
        oy,
        a.cos(),
        a.sin(),
        2.0 / (3.0 * radius),
        -1.0 / (3.0 * radius),
        r3 / (3.0 * radius),
        1.5 * radius,
        r3 * radius,
        smearing / 100.0,
        mode,
        r3 / 2.0,
    ];
    for i in 0..6 {
        let t = (60.0 * i as f64).to_radians();
        k.extend([t.cos(), t.sin()]);
    }
    k.push(0.0);
    k
}

/// D-461: where Hex Tile's pixel whose centre is (x, y), in the buffer, reads, `k` from
/// `hex_tile_setup`: its tile is the nearest middle of a flat-topped honeycomb, its place in the
/// tile is kept (tile) or folded into the tile's first triangle (the folds, every odd one
/// mirrored when seamless), and it reads that place round the centre, moved out towards its own
/// tile by the smearing. The card's `warp` mode 100 works the same products in the same order.
pub(crate) fn hex_tile_at(k: &[f64], x: f64, y: f64) -> (f64, f64) {
    let (dx, dy) = (x - k[2], y - k[3]);
    let (c, s) = (k[4], k[5]);
    let u = c * dx + s * dy;
    let v = c * dy - s * dx;
    let qf = u * k[6];
    let rf = u * k[7] + v * k[8];
    let t = [qf, -qf - rf, rf];
    let mut r = t.map(|e| (e + 0.5).floor());
    let g = [(r[0] - t[0]).abs(), (r[1] - t[1]).abs(), (r[2] - t[2]).abs()];
    if g[0] > g[1] && g[0] > g[2] {
        r[0] = -r[1] - r[2];
    } else if g[1] > g[2] {
        r[1] = -r[0] - r[2];
    } else {
        r[2] = -r[0] - r[1];
    }
    let (mu, mv) = (k[9] * r[0], k[10] * (r[2] + r[0] / 2.0));
    let (mut lu, mut lv) = (u - mu, v - mv);
    if k[12] != 0.0 {
        let h = k[13];
        let e1 = lv / 2.0 - h * lu;
        let e2 = -lv / 2.0 - h * lu;
        let n = if lv >= 0.0 {
            if e1 < 0.0 {
                0
            } else if e2 < 0.0 {
                1
            } else {
                2
            }
        } else if e1 > 0.0 {
            3
        } else if e2 > 0.0 {
            4
        } else {
            5
        };
        let (ck, sk) = (k[14 + 2 * n], k[15 + 2 * n]);
        (lu, lv) = (ck * lu + sk * lv, ck * lv - sk * lu);
        if k[12] == 2.0 && n % 2 == 1 {
            (lu, lv) = (lu / 2.0 + h * lv, h * lu - lv / 2.0);
        }
    }
    let m = k[11];
    (k[0] + c * lu - s * lv + m * (c * mu - s * mv), k[1] + s * lu + c * lv + m * (s * mu + c * mv))
}

/// D-461: after After Effects' CC HexTile. Each pixel reads `hex_tile_at`'s place, folded back
/// into the buffer by mirroring at its edges and held inside its pixel centres, so the honeycomb
/// never goes clear. The buffer keeps its size.
pub(crate) fn hex_tile(source: &mut WorkingBuffer, k: &[f64]) {
    let (w, h) = (source.width(), source.height());
    if w == 0 || h == 0 {
        return;
    }
    let fold = |u: f64, n: f64| {
        let t = 2.0 * n;
        let m = u - t * (u / t).floor();
        (if m > n { t - m } else { m }).clamp(0.5, n - 0.5)
    };
    let mut out = WorkingBuffer::transparent(w, h);
    let drawing = &*source;
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (sx, sy) = hex_tile_at(k, (i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
            px.copy_from_slice(&sample_bilinear(drawing, fold(sx, w as f64), fold(sy, h as f64)));
        });
    *source = out;
}

/// D-460: Burn Film's ragged edge, scorched edge and char line, as shares of the diagonal.
const BURN_ROUGH: f64 = 0.12;
const BURN_SCORCH: f64 = 0.06;
const BURN_CHAR: f64 = 0.02;

/// D-460: Burn Film's numbers for a picture `w` by `h`: the seed's mix, then the burn front, the
/// diagonal, and the seven burn points' x, y and delay, the centre first (document 21).
pub(crate) fn burn_film_setup((w, h): (usize, usize), burn: f64, center: [f64; 2], seed: f64) -> (u64, Vec<f64>) {
    let base = crate::grade::mix(seed.floor() as u64);
    let (w, h) = (w as f64, h as f64);
    let d = w.hypot(h);
    let (cx, cy) = (center[0] / 100.0 * w, center[1] / 100.0 * h);
    let far = [(0.0, 0.0), (w, 0.0), (0.0, h), (w, h)].iter().map(|&(x, y)| (x - cx).hypot(y - cy)).fold(0.0, f64::max);
    let s = burn / 100.0 * (far / d + BURN_ROUGH + BURN_SCORCH + BURN_CHAR) - BURN_SCORCH;
    let mut k = vec![s, d, cx, cy, 0.0];
    for i in 1..=6 {
        let u = |ch| crate::grade::unit(base, i, 0, -1, ch);
        k.extend([(u(0) + 1.0) / 2.0 * w, (u(1) + 1.0) / 2.0 * h, 0.25 + 0.125 * (u(2) + 1.0)]);
    }
    (base, k)
}

/// D-460: after After Effects' CC Burn Film. Each pixel burns at its distance from the nearest
/// burn point (over the diagonal, plus that point's delay) plus a ragged edge of D-127's fractal
/// noise; behind the front it is scorched to black, then a black char line fading out, then
/// gone. The layer never grows.
pub(crate) fn burn_film(source: &mut WorkingBuffer, burn: f64, center: [f64; 2], seed: f64) {
    let (w, h) = (source.width(), source.height());
    if w == 0 || h == 0 {
        return;
    }
    let (base, k) = burn_film_setup((w, h), burn, center, seed);
    let (s, d) = (k[0], k[1]);
    source.data_mut().par_chunks_mut(w * 4).enumerate().for_each(|(y, line)| {
        let py = y as f64 + 0.5;
        for (x, px) in line.chunks_mut(4).enumerate() {
            let qx = x as f64 + 0.5;
            let near = k[2..].chunks(3).map(|q| (qx - q[0]).hypot(py - q[1]) / d + q[2]).fold(f64::INFINITY, f64::min);
            let n = crate::grade::fractal(base, 0, (qx / (d / 10.0), py / (d / 10.0), 0.0), 6);
            let g = near + BURN_ROUGH * (n + 1.0) / 2.0 - s;
            if g >= BURN_SCORCH {
                continue;
            }
            if g >= 0.0 {
                let t = g / BURN_SCORCH;
                for c in px[..3].iter_mut() {
                    *c = (*c as f64 * t) as f32;
                }
            } else if g > -BURN_CHAR {
                px[3] = (px[3] as f64 * (1.0 + g / BURN_CHAR)) as f32;
                px[..3].fill(0.0);
            } else {
                px.fill(0.0);
            }
        }
    });
}

/// Blocks `size` wide as D-144's Mosaic lays them, flat, or when `bilinear` each pixel the blend,
/// across then down, of the means of the two blocks whose centres are either side of it (the edge
/// block's alone past the first or last centre).
fn block_picture(source: &mut WorkingBuffer, size: f64, bilinear: bool, (ox, oy): (usize, usize)) {
    if !bilinear || size <= 1.0 {
        return mosaic(source, size, (ox, oy));
    }
    let (w, h) = (source.width(), source.height());
    let (columns, rows) = (block_runs(w, ox, size), block_runs(h, oy, size));
    let data = source.data();
    let means: Vec<[f32; 4]> = rows
        .par_iter()
        .flat_map_iter(|ys| {
            columns.iter().map(move |xs| {
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
                sum.map(|v| (v / n) as f32)
            })
        })
        .collect();
    // For each pixel along, the two blocks either side of its centre and how far between.
    let pairs = |runs: &[std::ops::Range<usize>]| {
        let centre = |c: usize| (runs[c].start + runs[c].end) as f64 / 2.0;
        let mut out = Vec::new();
        for (c, r) in runs.iter().enumerate() {
            for p in r.clone() {
                let p = p as f64 + 0.5;
                let (a, b) = if p < centre(c) { (c.wrapping_sub(1), c) } else { (c, c + 1) };
                out.push(if a == usize::MAX {
                    (0, 0, 0.0)
                } else if b >= runs.len() {
                    (runs.len() - 1, runs.len() - 1, 0.0)
                } else {
                    (a, b, (p - centre(a)) / (centre(b) - centre(a)))
                });
            }
        }
        out
    };
    let (across, down) = (pairs(&columns), pairs(&rows));
    let nc = columns.len();
    let mut out = vec![0.0f32; w * h * 4];
    out.par_chunks_mut(w * 4).enumerate().for_each(|(y, line)| {
        let (ya, yb, ty) = down[y];
        for (x, px) in line.chunks_mut(4).enumerate() {
            let (xa, xb, tx) = across[x];
            for c in 0..4 {
                let m = |r: usize, k: usize| means[r * nc + k][c] as f64;
                let top = (1.0 - tx) * m(ya, xa) + tx * m(ya, xb);
                let bottom = (1.0 - tx) * m(yb, xa) + tx * m(yb, xb);
                px[c] = ((1.0 - ty) * top + ty * bottom) as f32;
            }
        }
    });
    source.data_mut().copy_from_slice(&out);
}

/// The picture luma of a premultiplied pixel, its colour over black, encoded.
pub(crate) fn picture_luma(p: [f32; 4]) -> f64 {
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

/// D-561: CC Glass's distant or point light: the point in the buffer's pixels, if any, the
/// light's height and the ambient, diffuse, specular, roughness and metal shares out of 1.
pub(crate) struct GlassLight {
    pub(crate) point: Option<(f64, f64)>,
    pub(crate) height: f64,
    pub(crate) shares: [f64; 5],
}

/// D-317: After Effects' CC Glass, reduced. The bump is the phase (D-316) `property` reads from
/// the drawing, or from `map` lying on it with its corner at the given origin (clear outside it),
/// times the covering unless the property is alpha, blurred at sigma `softness / 2`. Its slope `n`
/// is the bump's central differences, 0 beyond the blur, times `height / 100 * 1.25 *
/// max(softness, 1)`. Each pixel takes the drawing read bilinearly `displacement` pixels along `n`,
/// then is lit as Bevel Alpha lights it, its slope toward the light `-(n . u)` held to -1..1. So
/// at height 100 and no displacement it is Bevel Alpha with an edge thickness of the softness.
/// D-561: with `full`, the bent pixel is lit instead as Plastic's light lights it (N = (-nx, -ny,
/// 1) made one long), without the dust or an ambient colour. The settings are already valid.
pub(crate) fn glass(
    source: &mut WorkingBuffer,
    map: Option<(&WorkingBuffer, (usize, usize))>,
    property: &str,
    (softness, height, displacement): (f64, f64, f64),
    (angle, light, intensity): (f64, [f64; 3], f64),
    full: Option<GlassLight>,
) {
    if full.is_none() && (height == 0.0 || (displacement == 0.0 && intensity <= 0.0)) {
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
    if let Some(GlassLight { point, height: light_height, shares: [ambient, diffuse, specular, roughness, metal] }) = full {
        let unit = |v: [f64; 3]| {
            let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            if l > 0.0 { v.map(|c| c / l) } else { [0.0; 3] }
        };
        let (ux, uy) = crate::blurs::along(angle);
        let distant = unit([100.0 * ux, 100.0 * uy, light_height]);
        source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
            let a = px[3] as f64;
            if a <= 0.0 {
                px.fill(0.0);
                return;
            }
            let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
            let color = [0, 1, 2].map(|c| px[c] as f64 / a);
            let (nx, ny) = slope[i];
            let n = unit([-nx, -ny, 1.0]);
            let l = match point {
                Some((lx, ly)) => unit([lx - x, ly - y, light_height]),
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
        });
    } else if intensity > 0.0 {
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
    let (height, r) = flow_height(source, map, property, softness);
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

/// D-336/D-462/D-463: Vector Blur's, Mr. Smoothie's and Plastic's height, in the alpha of a buffer: the
/// `property` phase of the drawing, or of `map` lying on it with its corner at the given origin
/// (clear outside it), times the covering unless the property is alpha, blurred at sigma
/// `softness / 2` with nothing outside. The buffer is grown by the reach it returns.
fn flow_height(source: &WorkingBuffer, map: Option<(&WorkingBuffer, (usize, usize))>, property: &str, softness: f64) -> (WorkingBuffer, isize) {
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
    (height, r)
}

/// D-462: Mr. Smoothie's numbers for a buffer `size` whose drawing's corner is at `origin`: the
/// phase over 360, the colour loop's place in [`crate::effects::MR_SMOOTHIE_LOOPS`], Sample A's x
/// and y and Sample B's in the buffer, and a 0 for the card's rounded products.
pub(crate) fn mr_smoothie_setup(phase: f64, color_loop: &str, a: [f64; 2], b: [f64; 2], size: (usize, usize), origin: (usize, usize)) -> [f64; 7] {
    let (ax, ay) = crate::effects::radial_center(a, size, origin);
    let (bx, by) = crate::effects::radial_center(b, size, origin);
    let lp = crate::effects::MR_SMOOTHIE_LOOPS.iter().position(|l| *l == color_loop).unwrap_or(2);
    [phase / 360.0, lp as f64, ax, ay, bx, by, 0.0]
}

/// D-462: after After Effects' CC Mr. Smoothie (document 21). The flow is `flow_height` at
/// `smoothness`, held to 0 to 1 and moved on by the phase, wrapped only outside 0 to 1; the
/// colour loop makes it a place s along the line from Sample A to Sample B (AB s = t, BA 1 - t,
/// ABA 1 - |2t - 1|, BAB |2t - 1|), and each pixel takes the straight colour of the bilinear
/// sample there, held inside the pixel centres, times its own covering. `k` from
/// `mr_smoothie_setup`; the card's `smoothie` pass works the same products in the same order.
pub(crate) fn mr_smoothie(source: &mut WorkingBuffer, map: Option<(&WorkingBuffer, (usize, usize))>, property: &str, smoothness: f64, k: &[f64; 7]) {
    let (w, h) = (source.width(), source.height());
    if w == 0 || h == 0 {
        return;
    }
    let (height, r) = flow_height(source, map, property, smoothness);
    let (bw, data) = (height.width() as isize, height.data());
    let (fw, fh) = (w as f64, h as f64);
    let still = source.clone();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let a = px[3] as f64;
        if a <= 0.0 {
            px.copy_from_slice(&[0.0; 4]);
            return;
        }
        let (x, y) = ((i % w) as isize + r, (i / w) as isize + r);
        let v = (data[((y * bw + x) * 4 + 3) as usize] as f64).clamp(0.0, 1.0);
        let mut t = v + k[0];
        if t < 0.0 || t > 1.0 {
            t -= t.floor();
        }
        let s = match k[1] as u8 {
            0 => t,
            1 => 1.0 - t,
            2 => 1.0 - (2.0 * t - 1.0).abs(),
            _ => (2.0 * t - 1.0).abs(),
        };
        let q = sample_bilinear(&still, (k[2] + s * (k[4] - k[2])).clamp(0.5, fw - 0.5), (k[3] + s * (k[5] - k[3])).clamp(0.5, fh - 0.5));
        let qa = q[3] as f64;
        for c in 0..3 {
            px[c] = if qa > 0.0 { (q[c] as f64 / qa * a) as f32 } else { 0.0 };
        }
    });
}

/// D-463: Plastic, after After Effects' CC Plastic (document 21). The bump is `flow_height` at
/// `softness`, cut to b = (v - lo) / (hi - lo) held to 0..1 (a step at lo when lo = hi); its
/// surface, normal (-k gx, -k gy, 1) with k = `height` / 100 1.25 max(softness, 1) and g the cut
/// bump's central differences, is lit as Blobbylize's, with the ambient share in the ambient
/// light's colour and a dust term, Lommel and Seeliger's N.L / (N.L + Nz), beside the diffuse.
/// The layer keeps its own straight colour under the light and its own covering. The settings are
/// already valid, the shares out of 1 and the colours linear.
pub(crate) fn plastic(
    source: &mut WorkingBuffer,
    map: Option<(&WorkingBuffer, (usize, usize))>,
    property: &str,
    (softness, height, lo, hi): (f64, f64, f64, f64),
    (point, direction, light_height, light, intensity): (Option<(f64, f64)>, f64, f64, [f64; 3], f64),
    ambient_light: [f64; 3],
    [ambient, diffuse, dust, specular, roughness, metal]: [f64; 6],
) {
    let w = source.width();
    if w == 0 || source.height() == 0 {
        return;
    }
    let (bump, r) = flow_height(source, map, property, softness);
    let (bw, bh, data) = (bump.width() as isize, bump.height() as isize, bump.data());
    let cut = |v: f64| {
        if hi == lo {
            if v >= lo { 1.0 } else { 0.0 }
        } else {
            ((v - lo) / (hi - lo)).clamp(0.0, 1.0)
        }
    };
    let at = |x: isize, y: isize| {
        let (bx, by) = (x + r, y + r);
        cut(if bx < 0 || by < 0 || bx >= bw || by >= bh { 0.0 } else { data[((by * bw + bx) * 4 + 3) as usize] as f64 })
    };
    let k = height / 100.0 * 1.25 * softness.max(1.0);
    let unit = |v: [f64; 3]| {
        let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        if l > 0.0 { v.map(|c| c / l) } else { [0.0; 3] }
    };
    let (ux, uy) = crate::blurs::along(direction);
    let distant = unit([100.0 * ux, 100.0 * uy, light_height]);
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let a = px[3] as f64;
        if a <= 0.0 {
            px.fill(0.0);
            return;
        }
        let (x, y) = ((i % w) as isize, (i / w) as isize);
        let color = [0, 1, 2].map(|c| px[c] as f64 / a);
        let (gx, gy) = ((at(x + 1, y) - at(x - 1, y)) / 2.0, (at(x, y + 1) - at(x, y - 1)) / 2.0);
        let n = unit([-k * gx, -k * gy, 1.0]);
        let l = match point {
            Some((lx, ly)) => unit([lx - (x as f64 + 0.5), ly - (y as f64 + 0.5), light_height]),
            None => distant,
        };
        let nl = n[0] * l[0] + n[1] * l[1] + n[2] * l[2];
        let dusty = if nl > 0.0 { nl / (nl + n[2]) } else { 0.0 };
        let rz = 2.0 * nl * n[2] - l[2];
        let shine = if nl > 0.0 && rz > 0.0 { rz.powf(1.0 / roughness) } else { 0.0 };
        for c in 0..3 {
            let lit = color[c] * (ambient * ambient_light[c] + diffuse * intensity * light[c] * nl.max(0.0) + dust * intensity * light[c] * dusty)
                + specular * intensity * (light[c] + (color[c] - light[c]) * metal) * shine;
            px[c] = (lit * a) as f32;
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

/// D-424: Glue Gun's blobs at the frame `clock[0]` (`clock[1]` frames a second), oldest first.
/// `trail` is where the brush was, in pixels of the buffer, now and at each frame before, as
/// many as the time span holds. Blob k, born at frame k / `density`, lies where the brush was
/// then, the trail joined by straight lines; with `wobble` (width, height, turns a second) it
/// swings by a sine a step out of its neighbours'. No trail or no density, no blobs.
pub(crate) fn glue_blobs(trail: &[(f64, f64)], [f, fps]: [f64; 2], d: f64, wobble: Option<(f64, f64, f64)>) -> Vec<(f64, f64)> {
    use std::f64::consts::PI;
    if d == 0.0 || trail.is_empty() {
        return Vec::new();
    }
    let n = trail.len() - 1;
    let frac = |v: f64| v - v.floor();
    let (lo, hi) = (((f - n as f64) * d).ceil() as i64, (f * d).floor() as i64);
    (lo..=hi)
        .map(|k| {
            let kf = k as f64;
            let u = (f * d - kf) / d;
            let j = u.floor();
            let t = u - j;
            let j = j as usize;
            let (mut x, mut y) = if j >= n {
                trail[n]
            } else {
                (trail[j].0 + (trail[j + 1].0 - trail[j].0) * t, trail[j].1 + (trail[j + 1].1 - trail[j].1) * t)
            };
            if let Some((ww, wh, s)) = wobble {
                let now = f / fps;
                x += ww * (2.0 * PI * (s * now + frac(kf * 0.6180339887498949))).sin();
                y += wh * (2.0 * PI * (s * now + frac(kf * 0.7548776662466927))).sin();
            }
            (x, y)
        })
        .collect()
}

/// D-424: the blobs that reach each band of 16 rows, `reach` pixels round each, in order.
pub(crate) fn glue_bands(blobs: &[(f64, f64)], reach: f64, h: usize) -> Vec<Vec<u32>> {
    let mut bands = vec![Vec::new(); h.div_ceil(16)];
    for (i, &(_, cy)) in blobs.iter().enumerate() {
        let (lo, hi) = ((cy - reach - 0.5).floor(), (cy + reach - 0.5).ceil());
        if !(hi >= 0.0 && lo <= (h - 1) as f64) {
            continue;
        }
        for band in &mut bands[lo.max(0.0) as usize / 16..=(hi as usize).min(h - 1) / 16] {
            band.push(i as u32);
        }
    }
    bands
}

/// D-424: Glue Gun, this program's own reading of CycoreFX's CC Glue Gun. Each blob pulls
/// (1 - d^2 / R^2)^2 within R = `r` (1 + `strength`) of it; the stroke is where the pull F
/// passes a lone blob's at `r`, T0, with a pixel of soft edge from s = (F - T0) / |G|, G its
/// slope. The surface is a tube of radius `r` round the edge, and the paint shows the drawing
/// `reflection` 2 `r` along the surface's slope, straight (black where it is clear), lit as
/// Blobbylize's is, and laid over the drawing. The settings are already valid, the shares out of
/// 1 and `light` linear.
pub(crate) fn glue_gun(
    source: &mut WorkingBuffer,
    blobs: &[(f64, f64)],
    (r, strength, reflection): (f64, f64, f64),
    (point, direction, height, light, intensity): (Option<(f64, f64)>, f64, f64, [f64; 3], f64),
    [ambient, diffuse, specular, roughness, metal]: [f64; 5],
) {
    if blobs.is_empty() || r <= 0.0 {
        return;
    }
    let (w, h) = (source.width(), source.height());
    let big = r * (1.0 + strength);
    let t0 = (1.0 - (r / big) * (r / big)) * (1.0 - (r / big) * (r / big));
    let look = reflection * 2.0 * r;
    let bands = glue_bands(blobs, big, h);
    let unit = |v: [f64; 3]| {
        let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        if l > 0.0 { v.map(|c| c / l) } else { [0.0; 3] }
    };
    let (ux, uy) = crate::blurs::along(direction);
    let distant = unit([100.0 * ux, 100.0 * uy, height]);
    let still = source.clone();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
        let (mut f, mut gx, mut gy) = (0.0, 0.0, 0.0);
        for &b in &bands[i / w / 16] {
            let (cx, cy) = blobs[b as usize];
            let (dx, dy) = (x - cx, y - cy);
            let q = (dx * dx + dy * dy) / (big * big);
            if q < 1.0 {
                f += (1.0 - q) * (1.0 - q);
                gx += -4.0 * (1.0 - q) * dx / (big * big);
                gy += -4.0 * (1.0 - q) * dy / (big * big);
            }
        }
        let g = (gx * gx + gy * gy).sqrt();
        let (a, n) = if g > 0.0 {
            let s = (f - t0) / g;
            let m = (1.0 - s / r).clamp(0.0, 1.0);
            ((s + 0.5).clamp(0.0, 1.0), [-m * gx / g, -m * gy / g, (1.0 - m * m).sqrt()])
        } else {
            (if f > t0 { 1.0 } else { 0.0 }, [0.0, 0.0, 1.0])
        };
        if a == 0.0 {
            return;
        }
        let seen = sample_bilinear(&still, x - look * n[0], y - look * n[1]).map(f64::from);
        let color = if seen[3] > 0.0 { [seen[0] / seen[3], seen[1] / seen[3], seen[2] / seen[3]] } else { [0.0; 3] };
        let l = match point {
            Some((lx, ly)) => unit([lx - x, ly - y, height]),
            None => distant,
        };
        let nl = n[0] * l[0] + n[1] * l[1] + n[2] * l[2];
        let rz = 2.0 * nl * n[2] - l[2];
        let shine = if nl > 0.0 && rz > 0.0 { rz.powf(1.0 / roughness) } else { 0.0 };
        let under = [px[0] as f64, px[1] as f64, px[2] as f64, px[3] as f64];
        for c in 0..3 {
            let lit = color[c] * (ambient + diffuse * intensity * light[c] * nl.max(0.0))
                + specular * intensity * (light[c] + (color[c] - light[c]) * metal) * shine;
            px[c] = (lit * a + (1.0 - a) * under[c]) as f32;
        }
        px[3] = (a + (1.0 - a) * under[3]) as f32;
    });
}

/// D-425: Threads. The buffer woven into a cloth about `center`, in its pixels, turned by
/// `turn` (the direction's sine and cosine): warp threads `size.0` apart across, weft threads
/// `size.1` apart down, each `coverage` of its spacing wide, the warp over where (i + j) mod 2n
/// < n, with `n` the overlaps taken whole; the thread beneath shadowed by `shadowing`, each
/// shaded round by `texture`. The settings are already valid.
pub(crate) fn threads(
    source: &mut WorkingBuffer,
    center: (f64, f64),
    (s, c): (f64, f64),
    size: (f64, f64),
    n: f64,
    [coverage, shadowing, texture]: [f64; 3],
) {
    let w = source.width();
    let half = (coverage * size.0 / 2.0, coverage * size.1 / 2.0);
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let dx = (i % w) as f64 + 0.5 - center.0;
        let dy = (i / w) as f64 + 0.5 - center.1;
        let (u, v) = (dx * c + dy * s, -dx * s + dy * c);
        let (ti, tj) = ((u / size.0).floor(), (v / size.1).floor());
        let warp = thread(u - (ti + 0.5) * size.0, half.0, texture);
        let weft = thread(v - (tj + 0.5) * size.1, half.1, texture);
        let (up, down) = if (ti + tj).rem_euclid(2.0 * n) < n { (warp, weft) } else { (weft, warp) };
        let g = if up.3 > 0.0 { (1.0 - (up.2.abs() - up.3) / up.3).clamp(0.0, 1.0) } else { 0.0 };
        let k = up.0 * up.1 + down.0 * down.1 * (1.0 - shadowing * (1.0 + g) / 2.0) * (1.0 - up.0);
        let a = up.0 + down.0 * (1.0 - up.0);
        for v in &mut px[..3] {
            *v = (*v as f64 * k) as f32;
        }
        px[3] = (px[3] as f64 * a) as f32;
    });
}

/// D-425: a thread's share of the pixel `e` from its middle, its shade by `texture`, `e` and
/// its half width.
fn thread(e: f64, half: f64, texture: f64) -> (f64, f64, f64, f64) {
    let share = ((e + 0.5).min(half) - (e - 0.5).max(-half)).max(0.0);
    let shade = if half > 0.0 { 1.0 - texture * (e / half).powi(2).min(1.0) } else { 1.0 };
    (share, shade, e, half)
}

/// D-426: each lens's parts, (kind, t, r, w, n, red, green, blue): kind 0 a glow, 1 a ring, 2 a
/// disc (a ghost), 3 a star of n rays; t its place on the line from the flare (0) through the
/// layer's middle (1); r its size and w its edge (a star's: its rays' sharpness), as shares of the
/// drawing's diagonal; its colour in linear light. The same numbers are in
/// tools/lens_flare_reference.py.
pub(crate) const FLARE_LENSES: [(&str, &[[f64; 8]]); 3] = [
    (
        "zoom",
        &[
            [0.0, 0.0, 0.012, 0.0, 0.0, 2.0, 2.0, 2.0],
            [0.0, 0.0, 0.06, 0.0, 0.0, 1.0, 0.75, 0.45],
            [1.0, 0.0, 0.2, 0.012, 0.0, 0.12, 0.18, 0.3],
            [3.0, 0.0, 0.1, 40.0, 6.0, 0.5, 0.45, 0.4],
            [2.0, 0.45, 0.02, 0.006, 0.0, 0.1, 0.18, 0.35],
            [2.0, 0.7, 0.045, 0.01, 0.0, 0.12, 0.3, 0.12],
            [2.0, 1.2, 0.03, 0.008, 0.0, 0.35, 0.2, 0.08],
            [2.0, 1.5, 0.08, 0.02, 0.0, 0.08, 0.08, 0.25],
            [2.0, 1.8, 0.015, 0.005, 0.0, 0.4, 0.1, 0.1],
            [1.0, 2.0, 0.1, 0.008, 0.0, 0.1, 0.25, 0.15],
        ],
    ),
    (
        "35mm",
        &[
            [0.0, 0.0, 0.01, 0.0, 0.0, 2.0, 2.0, 2.0],
            [0.0, 0.0, 0.04, 0.0, 0.0, 1.0, 0.85, 0.65],
            [3.0, 0.0, 0.06, 60.0, 8.0, 0.4, 0.4, 0.4],
            [2.0, 0.5, 0.025, 0.006, 0.0, 0.1, 0.22, 0.4],
            [2.0, 1.3, 0.05, 0.012, 0.0, 0.2, 0.12, 0.35],
            [2.0, 1.7, 0.02, 0.005, 0.0, 0.4, 0.25, 0.1],
        ],
    ),
    (
        "105mm",
        &[
            [0.0, 0.0, 0.015, 0.0, 0.0, 2.5, 2.5, 2.5],
            [0.0, 0.0, 0.07, 0.0, 0.0, 1.2, 0.85, 0.5],
            [1.0, 0.0, 0.11, 0.02, 0.0, 0.18, 0.1, 0.04],
            [3.0, 0.0, 0.14, 30.0, 12.0, 0.6, 0.55, 0.45],
            [2.0, 0.6, 0.03, 0.008, 0.0, 0.08, 0.18, 0.4],
            [2.0, 1.4, 0.06, 0.015, 0.0, 0.35, 0.15, 0.06],
        ],
    ),
];

/// D-426: the lens's parts placed for this drawing, (kind, x, y, R, w, n, red, green, blue), in
/// pixels, the colour times `brightness`: the flare at `flare`, the middle at `middle`, the
/// diagonal `size`. Handed as they are to the card.
pub(crate) fn flare_parts(lens: &str, flare: (f64, f64), middle: (f64, f64), size: f64, brightness: f64) -> Vec<[f64; 9]> {
    let parts = FLARE_LENSES.iter().find(|(l, _)| *l == lens).map_or(&[][..], |(_, p)| p);
    parts
        .iter()
        .map(|&[kind, t, r, w, n, red, green, blue]| {
            [
                kind,
                flare.0 + t * (middle.0 - flare.0),
                flare.1 + t * (middle.1 - flare.1),
                r * size,
                if kind == 3.0 { w } else { w * size },
                n,
                red * brightness,
                green * brightness,
                blue * brightness,
            ]
        })
        .collect()
}

/// D-426: Lens Flare. The light of the placed `parts` added to each pixel, its covering raised by
/// the light's brightest channel, the result kept by `keep` (1 - Blend With Original) against the
/// pixel as it was. D-545: with `dirt`, the light first has the dirt matted by it on top.
pub(crate) fn lens_flare(source: &mut WorkingBuffer, parts: &[[f64; 9]], keep: f64, dirt: Option<&crate::soft_glow::Dirt>) {
    let (w, h) = (source.width(), source.height());
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
        let mut light = [0.0f64; 3];
        for &[kind, cx, cy, big, edge, n, red, green, blue] in parts {
            let (dx, dy) = (x - cx, y - cy);
            let d = dx.hypot(dy);
            let v = match kind as u8 {
                0 => (-(d / big).powi(2)).exp(),
                1 => (-((d - big) / edge).powi(2)).exp(),
                2 => ((big - d) / edge + 0.5).clamp(0.0, 1.0),
                _ => {
                    let a = if d > 0.0 { dy.atan2(dx) } else { 0.0 };
                    (-d / big).exp() * ((1.0 + (n * a).cos()) / 2.0).powf(edge)
                }
            };
            light[0] += red * v;
            light[1] += green * v;
            light[2] += blue * v;
        }
        if let Some(d) = dirt {
            let (x, y) = ((i % w) as isize, (i / w) as isize);
            light = d.lit(light.map(|v| v as f32), d.at(x, y, w, h)).map(f64::from);
        }
        let a = px[3] as f64;
        let lifted = a + (1.0 - a) * light[0].max(light[1]).max(light[2]).min(1.0);
        for c in 0..3 {
            px[c] = (px[c] as f64 + keep * light[c]) as f32;
        }
        px[3] = (a + keep * (lifted - a)) as f32;
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


/// D-407: After Effects' largest layer side. An Upscale that would grow the layer past it is not
/// drawn, and says so (document 28's EFFECT_LAYER_TOO_LARGE).
pub(crate) const LARGEST_SIDE: usize = 30000;

/// D-407: the first tap and the six Lanczos-3 weights of output place `x` of `big`, read from a
/// row of `n` at scale `s`: u = n / 2 + (x + 1/2 - big / 2) / s - 1/2, the taps floor(u) - 2 to
/// floor(u) + 3, each weight sinc(t) sinc(t / 3) (exactly 0 at every other whole t), divided by
/// the six's sum.
pub(crate) fn lanczos_taps(n: usize, big: usize, s: f64, x: usize) -> (i64, [f64; 6]) {
    let u = n as f64 / 2.0 + (x as f64 + 0.5 - big as f64 / 2.0) / s - 0.5;
    let f = u.floor();
    let mut w = [0.0; 6];
    for (k, v) in w.iter_mut().enumerate() {
        let t = u - (f - 2.0 + k as f64);
        *v = if t == 0.0 {
            1.0
        } else if t.abs() >= 3.0 || t == t.floor() {
            0.0
        } else {
            let a = std::f64::consts::PI * t;
            3.0 * a.sin() * (a / 3.0).sin() / (a * a)
        };
    }
    let sum: f64 = w.iter().sum();
    w.iter_mut().for_each(|v| *v /= sum);
    (f as i64 - 2, w)
}

/// D-407: how far an Upscale grows the layer on each side, across and down: ceil(n (scale - 100)
/// / 200), worked on the per cent so a whole per cent gives a whole answer exactly.
pub(crate) fn upscale_growth((w, h): (usize, usize), scale: f64) -> (usize, usize) {
    let g = |n: usize| (n as f64 * (scale - 100.0) / 200.0).ceil().max(0.0) as usize;
    (g(w), g(h))
}

/// D-407: whether the grown layer would be wider or taller than [`LARGEST_SIDE`].
pub(crate) fn upscale_too_large((w, h): (usize, usize), scale: f64) -> bool {
    let (gx, gy) = upscale_growth((w, h), scale);
    w + 2 * gx > LARGEST_SIDE || h + 2 * gy > LARGEST_SIDE
}

/// D-407: Detail-preserving Upscale, document 21's rule. Reduce Noise softens the picture at
/// sigma `reduce_noise` / 50 (clear outside, kept its size); the layer grows by
/// [`upscale_growth`] about its middle and each pixel is the Lanczos-3 sum of
/// [`lanczos_taps`], across then down, a tap outside the picture clear; Detail adds
/// `detail` / 50 times the enlargement less its blur at sigma `scale` / 200; the covering is held
/// to 0..1 and each colour to 0..covering. Returns the growth on each side, or `Err` when the
/// grown layer is larger than this build can hold (too wide or tall, or its memory cannot be
/// had). The settings are already valid.
pub(crate) fn detail_upscale(source: &mut WorkingBuffer, scale: f64, reduce_noise: f64, detail: f64) -> Result<(usize, usize), ()> {
    if scale == 100.0 && reduce_noise <= 0.0 && detail <= 0.0 {
        return Ok((0, 0));
    }
    let (w, h) = (source.width(), source.height());
    if upscale_too_large((w, h), scale) {
        return Err(());
    }
    let (gx, gy) = upscale_growth((w, h), scale);
    let (bw, bh) = (w + 2 * gx, h + 2 * gy);
    let s = scale / 100.0;
    let r = if detail > 0.0 { kernel_radius(s / 2.0) } else { 0 };
    // ponytail: a probe of the buffers below, given back before they are made; one that fails
    // after it still stops the program as any allocation does.
    let floats = 4 * (h * bw + 2 * bw * bh + (bw + 2 * r) * (bh + 2 * r));
    if Vec::<f32>::new().try_reserve_exact(floats).is_err() {
        return Err(());
    }
    let soft = (reduce_noise > 0.0).then(|| {
        let mut b = source.clone();
        let r0 = blur(&mut b, reduce_noise / 50.0);
        (b, r0)
    });
    let (src, r0) = match &soft {
        Some((b, r0)) => (b, *r0),
        None => (&*source, 0),
    };
    let (sw, sd) = (src.width(), src.data());
    let taps = |n: usize, big: usize| -> Vec<(i64, [f32; 6])> {
        (0..big).map(|x| {
            let (first, k) = lanczos_taps(n, big, s, x);
            (first, k.map(|v| v as f32))
        })
        .collect()
    };
    let (cols, rows) = (taps(w, bw), taps(h, bh));
    let mut mid = vec![0.0f32; h * bw * 4];
    mid.par_chunks_exact_mut(bw * 4).enumerate().for_each(|(y, row)| {
        for (px, (first, k)) in row.chunks_exact_mut(4).zip(&cols) {
            for (i, kv) in k.iter().enumerate() {
                let xx = first + i as i64;
                if xx < 0 || xx >= w as i64 {
                    continue;
                }
                let o = ((y + r0) * sw + xx as usize + r0) * 4;
                for c in 0..4 {
                    px[c] += kv * sd[o + c];
                }
            }
        }
    });
    let mut out = WorkingBuffer::transparent(bw, bh);
    out.data_mut().par_chunks_exact_mut(bw * 4).enumerate().for_each(|(y, row)| {
        let (first, k) = rows[y];
        for (j, kv) in k.iter().enumerate() {
            let yy = first + j as i64;
            if yy < 0 || yy >= h as i64 {
                continue;
            }
            for (o, v) in row.iter_mut().zip(&mid[yy as usize * bw * 4..][..bw * 4]) {
                *o += kv * v;
            }
        }
    });
    if detail > 0.0 {
        let a = (detail / 50.0) as f32;
        let mut b = out.clone();
        let r = blur(&mut b, s / 2.0);
        let (gw, gd) = (b.width(), b.data());
        out.data_mut().par_chunks_exact_mut(bw * 4).enumerate().for_each(|(y, row)| {
            for (o, v) in row.iter_mut().zip(&gd[((y + r) * gw + r) * 4..][..bw * 4]) {
                *o += a * (*o - v);
            }
        });
    }
    out.data_mut().par_chunks_exact_mut(4).for_each(|px| {
        let al = px[3].max(0.0).min(1.0);
        px[3] = al;
        for c in &mut px[..3] {
            *c = c.max(0.0).min(al);
        }
    });
    *source = out;
    Ok((gx, gy))
}

/// D-406: Spherize. Each pixel within `radius` of `center`, a point in the buffer, rho of the
/// way out, read at center + v (2 asin(rho) / pi) / rho, a half sphere seen from in front; the
/// rest left as they are. The settings are already valid.
pub(crate) fn spherize(source: &mut WorkingBuffer, radius: f64, center: (f64, f64)) {
    if radius <= 0.0 {
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
            let rho = d / radius;
            let k = if d == 0.0 { 0.0 } else { 2.0 * rho.asin() / std::f64::consts::PI / rho };
            px.copy_from_slice(&sample_bilinear(&drawing, center.0 + k * vx, center.1 + k * vy));
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

/// D-464: how far Repeat Tile grows a `w` by `h` buffer across and down, on each side: the
/// larger of the two expansions each way (right and left, down and up), each rounded up to
/// whole pixels; nothing for an empty buffer. The card's plan asks it too.
pub(crate) fn repeat_tile_growth(expand: [f64; 4], (w, h): (usize, usize)) -> (usize, usize) {
    if w == 0 || h == 0 {
        return (0, 0);
    }
    let e = expand.map(|e| e.ceil().max(0.0) as usize);
    (e[0].max(e[1]), e[2].max(e[3]))
}

/// D-464: Repeat Tile's numbers (document 21) from its expansions (right, left, down, up), its
/// Tiling and its Blend Borders: the Tiling's place in `REPEAT_TILE_TILINGS`, the left, right,
/// top and bottom expansions in whole pixels, and the blend as a share, 0 under Unfold, None and
/// Slide, which ignore it. The card's warp mode 101 reads the same six.
pub(crate) fn repeat_tile_setup(expand: [f64; 4], tiling: &str, blend: f64) -> [f64; 6] {
    let t = crate::effects::REPEAT_TILE_TILINGS.iter().position(|m| *m == tiling).unwrap_or(3);
    let b = if ["unfold", "none", "slide"].contains(&tiling) { 0.0 } else { blend / 100.0 };
    let e = expand.map(f64::ceil);
    [t as f64, e[1], e[0], e[3], e[2], b]
}

/// D-464: the way, 0 to 7, Repeat Tile lays tile (i, j) under Tiling `mode`: 1 flips across, 2
/// flips down and 4 swaps across and down, so 3 is a half turn, 5 a quarter anticlockwise and 6
/// a quarter clockwise.
fn repeat_way(mode: usize, i: i64, j: i64) -> u32 {
    let quarter = |q: i64| [0, 6, 3, 5][q.rem_euclid(4) as usize];
    let p = (i + j).rem_euclid(2) as u32;
    match mode {
        1 => p,
        2 => 2 * p,
        4 => 3 * p,
        5 => 4 * p,
        6 => 6 * p,
        7 => 5 * p,
        3 => (i.rem_euclid(2) + 2 * j.rem_euclid(2)) as u32,
        8 => match (i.rem_euclid(2), j.rem_euclid(2)) {
            (0, 0) => 0,
            (1, 0) => 6,
            (1, 1) => 3,
            _ => 5,
        },
        9 => {
            let mut t = (i as u32).wrapping_mul(0x9E37_79B1) ^ (j as u32).wrapping_mul(0x85EB_CA77);
            t ^= t >> 15;
            t = t.wrapping_mul(0x2C1B_3C6D);
            t ^= t >> 12;
            t % 8
        }
        11 => quarter(i + j),
        12 => quarter(-(i + j)),
        13 => quarter(i - j),
        _ => 0,
    }
}

/// D-464: the input pixel Repeat Tile's output place (x, y), counted from the input's corner,
/// reads, or None where the output is clear; `k` from `repeat_tile_setup`. Every choice is made
/// in whole numbers, so the card's warp mode 101 makes the same ones.
pub(crate) fn repeat_tile_at(k: &[f64; 6], x: i64, y: i64, (w, h): (i64, i64)) -> Option<(usize, usize)> {
    let mode = k[0] as usize;
    if x < -(k[1] as i64) || x >= w + k[2] as i64 || y < -(k[3] as i64) || y >= h + k[4] as i64 {
        return None;
    }
    if mode == 14 {
        return Some((x.clamp(0, w - 1) as usize, y.clamp(0, h - 1) as usize));
    }
    if mode == 10 {
        return ((0..w).contains(&x) && (0..h).contains(&y)).then_some((x as usize, y as usize));
    }
    let j = y.div_euclid(h);
    let x = if mode == 15 && j.rem_euclid(2) == 1 { x - w / 2 } else { x };
    let i = x.div_euclid(w);
    let (lx, ly) = (x - i * w, y - j * h);
    let o = repeat_way(mode, i, j);
    let q = |t: i64, n: i64, m: i64| (2 * t + 1) * m / (2 * n);
    let (mut u, mut v) = if o & 4 != 0 { (q(ly, h, w), q(lx, w, h)) } else { (lx, ly) };
    if o & 1 != 0 {
        u = w - 1 - u;
    }
    if o & 2 != 0 {
        v = h - 1 - v;
    }
    Some((u as usize, v as usize))
}

/// D-464: after After Effects' CC RepeTile. The buffer grown by `repeat_tile_growth` on each
/// side, the room within the expansions filled with copies of the buffer laid as
/// `repeat_tile_at` says, the rest clear; with a blend, each pixel mixed towards the tile's
/// edges with the one half a tile on, so copies meet without a seam. Returns the growth on the
/// left and on the top. The settings are already valid.
pub(crate) fn repeat_tile(source: &mut WorkingBuffer, k: &[f64; 6]) -> (usize, usize) {
    let (w, h) = (source.width(), source.height());
    if w == 0 || h == 0 {
        return (0, 0);
    }
    let (gx, gy) = (k[1].max(k[2]) as usize, k[3].max(k[4]) as usize);
    if (gx, gy) == (0, 0) && k[5] == 0.0 {
        return (0, 0);
    }
    let weight = |t: usize, n: usize| {
        if k[5] == 0.0 {
            return 0.0;
        }
        let e = (t as f64 + 0.5).min(n as f64 - t as f64 - 0.5);
        (1.0 - e / (k[5] * n as f64 / 2.0)).clamp(0.0, 1.0) as f32
    };
    let ow = w + 2 * gx;
    let mut out = WorkingBuffer::transparent(ow, h + 2 * gy);
    let d = source.data();
    let at = |u: usize, v: usize| {
        let s = (v * w + u) * 4;
        [d[s], d[s + 1], d[s + 2], d[s + 3]]
    };
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let place = ((i % ow) as i64 - gx as i64, (i / ow) as i64 - gy as i64);
            let Some((u, v)) = repeat_tile_at(k, place.0, place.1, (w as i64, h as i64)) else {
                return;
            };
            let (wx, wy) = (weight(u, w), weight(v, h));
            if wx == 0.0 && wy == 0.0 {
                px.copy_from_slice(&at(u, v));
                return;
            }
            let (u2, v2) = ((u + w / 2) % w, (v + h / 2) % h);
            let (a, b, c, e) = (at(u, v), at(u2, v), at(u, v2), at(u2, v2));
            for n in 0..4 {
                px[n] = (1.0 - wy) * ((1.0 - wx) * a[n] + wx * b[n]) + wy * ((1.0 - wx) * c[n] + wx * e[n]);
            }
        });
    *source = out;
    (gx, gy)
}

/// D-465: Scatter's numbers (document 21): its reach in whole pixels, floor(amount + 1/2), and
/// its Grain's place in `SCATTER_GRAINS`, 0 both ways, 1 across only, 2 down only. The card's
/// warp mode 102 reads the same two.
pub(crate) fn scatter_setup(amount: f64, grain: &str) -> [u32; 2] {
    let g = crate::effects::SCATTER_GRAINS.iter().position(|m| *m == grain).unwrap_or(0);
    [(amount + 0.5).floor().max(0.0) as u32, g as u32]
}

/// D-465: Scatter's move, across and down, for output place (x, y) at frame `f`, reach `k[0]`
/// and Grain `k[1]`: each a whole number from -reach to reach, from a 32-bit hash of the place,
/// the frame and the direction. The card's warp mode 102 works it the same way.
pub(crate) fn scatter_move(k: [u32; 2], x: u32, y: u32, f: i32) -> (i64, i64) {
    let hash = |d: u32| {
        let mut t = x.wrapping_mul(0x9E37_79B1) ^ y.wrapping_mul(0x85EB_CA77) ^ (f as u32).wrapping_mul(0xC2B2_AE3D) ^ d.wrapping_mul(0x27D4_EB2F);
        t ^= t >> 15;
        t = t.wrapping_mul(0x2C1B_3C6D);
        t ^= t >> 12;
        t = t.wrapping_mul(0x297A_2D39);
        t ^ (t >> 15)
    };
    let span = 2 * k[0] + 1;
    let dx = if k[1] == 2 { 0 } else { (hash(0) % span) as i64 - k[0] as i64 };
    let dy = if k[1] == 1 { 0 } else { (hash(1) % span) as i64 - k[0] as i64 };
    (dx, dy)
}

/// D-465: after After Effects' Scatter. Each pixel the buffer's pixel `scatter_move` away, or
/// clear where that lies outside it; nothing is mixed and nothing grows. `k` from
/// `scatter_setup`; a reach of 0 leaves the buffer as it is.
pub(crate) fn scatter(source: &mut WorkingBuffer, k: [u32; 2], f: i32) {
    let (w, h) = (source.width(), source.height());
    if k[0] == 0 || w == 0 || h == 0 {
        return;
    }
    let mut out = WorkingBuffer::transparent(w, h);
    let d = source.data();
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = (i % w, i / w);
            let (dx, dy) = scatter_move(k, x as u32, y as u32, f);
            let (u, v) = (x as i64 + dx, y as i64 + dy);
            if (0..w as i64).contains(&u) && (0..h as i64).contains(&v) {
                let s = (v as usize * w + u as usize) * 4;
                px.copy_from_slice(&d[s..s + 4]);
            }
        });
    *source = out;
}

/// D-466: whether Strobe Light is on at `time` seconds and frame `count` from the layer's in
/// point (document 21): always when the period is at most the duration, in the first `duration`
/// seconds of each period, or when the frame's random draw from the seed is below the
/// probability.
pub(crate) fn strobe_on(duration: f64, period: f64, probability: f64, seed: f64, time: f64, count: i64) -> bool {
    if period <= duration || time - period * (time / period).floor() < duration {
        return true;
    }
    let mut h = (count as u32).wrapping_mul(0x9E37_79B1) ^ (seed.floor() as u32).wrapping_mul(0x85EB_CA77);
    h ^= h >> 15;
    h = h.wrapping_mul(0x2C1B_3C6D);
    h ^= h >> 12;
    h = h.wrapping_mul(0x297A_2D39);
    h ^= h >> 15;
    (h as f64) / 4_294_967_296.0 < probability / 100.0
}

/// D-466: Strobe Light's numbers: the strobe colour in the working space, Blend With Original
/// over 100, the operator's place in `STROBE_LIGHT_OPERATORS`, and 1 for Makes Layer
/// Transparent. The card's warp mode 103 reads the same six.
pub(crate) fn strobe_setup(colour: [f64; 3], blend: f64, strobe: &str, op: &str) -> [f64; 6] {
    let o = crate::effects::STROBE_LIGHT_OPERATORS.iter().position(|m| *m == op).unwrap_or(0);
    [colour[0], colour[1], colour[2], blend / 100.0, o as f64, (strobe == "transparent") as u8 as f64]
}

/// D-466: Strobe Light's operator `op` (its place in `STROBE_LIGHT_OPERATORS`) on the straight
/// colour `c` and the strobe colour `s`, held in 0 to 1. And, Or and Xor work on whole levels of
/// 255; Darken and Lighten take the whole colour of lower or higher brightness, `c` on a tie.
pub(crate) fn strobe_operate(op: usize, c: [f64; 3], s: [f64; 3]) -> [f64; 3] {
    let luma = |v: [f64; 3]| 0.2126 * v[0] + 0.7152 * v[1] + 0.0722 * v[2];
    let level = |v: f64| (255.0 * v.clamp(0.0, 1.0) + 0.5).floor() as u32;
    let bits = |f: fn(u32, u32) -> u32| -> [f64; 3] { std::array::from_fn(|i| f(level(c[i]), level(s[i])) as f64 / 255.0) };
    let each = |f: fn(f64, f64) -> f64| -> [f64; 3] { std::array::from_fn(|i| f(c[i], s[i])) };
    let r = match op {
        0 => s,
        1 => each(|a, b| a + b),
        2 => each(|a, b| a - b),
        3 => each(|a, b| a * b),
        4 => each(|a, b| (a - b).abs()),
        5 => bits(|a, b| a & b),
        6 => bits(|a, b| a | b),
        7 => bits(|a, b| a ^ b),
        8 => each(f64::min),
        9 => each(f64::max),
        10 => if luma(s) < luma(c) { s } else { c },
        _ => if luma(s) > luma(c) { s } else { c },
    };
    r.map(|v| v.clamp(0.0, 1.0))
}

/// D-466: after After Effects' Strobe Light, while it is on. `k` from `strobe_setup`. Makes
/// Layer Transparent scales every channel by the blend; otherwise each covered pixel's straight
/// colour is worked by the operator, mixed back with the blend and laid over its own alpha,
/// which is kept.
pub(crate) fn strobe_light(source: &mut WorkingBuffer, k: [f64; 6]) {
    let (s, b, op, clear) = ([k[0], k[1], k[2]], k[3], k[4] as usize, k[5] != 0.0);
    source.data_mut().par_chunks_exact_mut(4).for_each(|px| {
        if clear {
            px.iter_mut().for_each(|v| *v = (*v as f64 * b) as f32);
            return;
        }
        let a = px[3] as f64;
        if a <= 0.0 {
            return;
        }
        let c = [px[0] as f64 / a, px[1] as f64 / a, px[2] as f64 / a];
        let r = strobe_operate(op, c, s);
        for i in 0..3 {
            px[i] = (((1.0 - b) * r[i] + b * c[i]) * a) as f32;
        }
    });
}

/// D-467: after After Effects' Texturize (document 21): Emboss's colour relief, read from the
/// luminance of the texture `map` (the drawing's size, its corner at the given place) one pixel
/// ahead and behind along `direction`, each sample held inside the pixel centres, times
/// `contrast` over 2. The colours, covering and size are kept. The settings are already valid.
pub(crate) fn texturize(source: &mut WorkingBuffer, map: (&WorkingBuffer, (usize, usize)), direction: f64, contrast: f64) {
    let (w, h) = (source.width(), source.height());
    if w == 0 || h == 0 {
        return;
    }
    let (height, _) = flow_height(source, Some(map), "luminance", 0.0);
    let (ux, uy) = crate::blurs::along(direction);
    let (fw, fh) = (w as f64, h as f64);
    let hold = |x: f64, y: f64| (x.max(0.5).min(fw - 0.5), y.max(0.5).min(fh - 0.5));
    let k = contrast / 2.0;
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let a = px[3] as f64;
        if a <= 0.0 {
            return;
        }
        let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
        let (ax, ay) = hold(x + ux, y + uy);
        let (bx, by) = hold(x - ux, y - uy);
        let v = (sample_bilinear(&height, ax, ay)[3] as f64 - sample_bilinear(&height, bx, by)[3] as f64) * k;
        for c in 0..3 {
            let e = crate::grade::to_srgb((px[c] as f64 / a).clamp(0.0, 1.0)) + v;
            px[c] = (crate::grade::to_linear(e.clamp(0.0, 1.0)) * a) as f32;
        }
    });
}

/// D-470: Contour Paint's numbers (document 21): the tensor's blur, (0.5 + 4.5 `cleanliness` /
/// 100) `px`; the turn `offset` degrees as its cosine and sine, exact at whole quarter turns; and
/// with `quantize` q above 0 the directions (cos kq, sin kq), k = 0 to ceil(180 / q) - 1.
pub(crate) fn contour_setup(cleanliness: f64, offset: f64, quantize: f64, px: f64) -> (f64, (f64, f64), Vec<(f64, f64)>) {
    let sigma = (0.5 + 4.5 * cleanliness / 100.0) * px;
    let q = offset.rem_euclid(360.0);
    let turn = if q == 0.0 {
        (1.0, 0.0)
    } else if q == 90.0 {
        (0.0, 1.0)
    } else if q == 180.0 {
        (-1.0, 0.0)
    } else if q == 270.0 {
        (0.0, -1.0)
    } else {
        (offset.to_radians().cos(), offset.to_radians().sin())
    };
    // ponytail: one direction per step, so a tiny Quantize (0.01 is 18000 directions) is slow
    // on the processor; work the nearest by its angle if that is ever used.
    let ways = if quantize > 0.0 {
        (0..(180.0 / quantize).ceil() as usize).map(|k| ((k as f64 * quantize).to_radians().cos(), (k as f64 * quantize).to_radians().sin())).collect()
    } else {
        Vec::new()
    };
    (sigma, turn, ways)
}

/// D-470: the smear's step weights, exp(-k² / 2 s²) for k = 1 to ceil(2 s), s = (0.5 + 7.5
/// `stylization` / 100) `px`.
pub(crate) fn contour_weights(stylization: f64, px: f64) -> Vec<f64> {
    let s = (0.5 + 7.5 * stylization / 100.0) * px;
    (1..=(2.0 * s).ceil() as usize).map(|k| (-((k * k) as f64) / (2.0 * s * s)).exp()).collect()
}

/// D-470: the flow and its strength from the blurred tensor (a, b, c): Line Blur's tangent, made
/// one long (none where its length is 0), turned by `turn`, then snapped to the first of `ways`
/// nearest it; the strength min(1, 4 sqrt(r)).
pub(crate) fn contour_tangent(a: f64, b: f64, c: f64, turn: (f64, f64), ways: &[(f64, f64)]) -> [f64; 3] {
    let r = ((a - c) * (a - c) + 4.0 * b * b).sqrt();
    let (tx, ty) = if a >= c { (b, -(a - c + r) / 2.0) } else { (-(c - a + r) / 2.0, b) };
    let n = (tx * tx + ty * ty).sqrt();
    let e = (4.0 * r.sqrt()).min(1.0);
    if n == 0.0 {
        return [0.0, 0.0, e];
    }
    let (tx, ty) = (tx / n, ty / n);
    let (mut tx, mut ty) = (tx * turn.0 - ty * turn.1, tx * turn.1 + ty * turn.0);
    let mut best = -1.0;
    let (ux, uy) = (tx, ty);
    for &(wx, wy) in ways {
        let d = (ux * wx + uy * wy).abs();
        if d > best {
            best = d;
            (tx, ty) = (wx, wy);
        }
    }
    [tx, ty, e]
}

/// D-470: Contour Paint's flow (document 21) at each pixel of `source`, or of `map` lying on it
/// with its corner at the given origin: CC Vector Blur's luminance, Sobel's slopes over 8 with
/// each neighbour held inside, the tensor (gx², gx gy, gy²) blurred by D-109's held Gaussian of
/// `sigma`, then [`contour_tangent`].
pub(crate) fn contour_flow(source: &WorkingBuffer, map: Option<(&WorkingBuffer, (usize, usize))>, sigma: f64, turn: (f64, f64), ways: &[(f64, f64)]) -> Vec<[f64; 3]> {
    let (w, h) = (source.width(), source.height());
    let (lum, _) = flow_height(source, map, "luminance", 0.0);
    let l = lum.data();
    let at = |x: isize, y: isize| l[(y.clamp(0, h as isize - 1) as usize * w + x.clamp(0, w as isize - 1) as usize) * 4 + 3] as f64;
    let mut tensor = WorkingBuffer::transparent(w, h);
    tensor.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as isize, (i / w) as isize);
        let gx = (at(x + 1, y - 1) + 2.0 * at(x + 1, y) + at(x + 1, y + 1) - at(x - 1, y - 1) - 2.0 * at(x - 1, y) - at(x - 1, y + 1)) / 8.0;
        let gy = (at(x - 1, y + 1) + 2.0 * at(x, y + 1) + at(x + 1, y + 1) - at(x - 1, y - 1) - 2.0 * at(x, y - 1) - at(x + 1, y - 1)) / 8.0;
        px[0] = (gx * gx) as f32;
        px[1] = (gx * gy) as f32;
        px[2] = (gy * gy) as f32;
    });
    crate::effects::held_blur(&mut tensor, sigma);
    tensor.data().par_chunks_exact(4).map(|p| contour_tangent(p[0] as f64, p[1] as f64, p[2] as f64, turn, ways)).collect()
}

/// D-470: the flow at (x, y) for a walk facing `d`: the bilinear mix of the flow at the four
/// pixels round it, each held inside and turned to face `d` first.
fn contour_way(field: &[[f64; 3]], (w, h): (usize, usize), (x, y): (f64, f64), d: (f64, f64)) -> (f64, f64) {
    let (fx, fy) = (x - 0.5, y - 0.5);
    let (x0, y0) = (fx.floor(), fy.floor());
    let (ux, uy) = (fx - x0, fy - y0);
    let mut out = (0.0, 0.0);
    for (dy, wy) in [(0, 1.0 - uy), (1, uy)] {
        for (dx, wx) in [(0, 1.0 - ux), (1, ux)] {
            let qx = (x0 as isize + dx).clamp(0, w as isize - 1) as usize;
            let qy = (y0 as isize + dy).clamp(0, h as isize - 1) as usize;
            let [tx, ty, _] = field[qy * w + qx];
            let sgn = if tx * d.0 + ty * d.1 < 0.0 { -1.0 } else { 1.0 };
            out.0 += sgn * tx * wx * wy;
            out.1 += sgn * ty * wx * wy;
        }
    }
    out
}

/// D-470: one pass of Contour Paint's smear (document 21): each pixel with a flow walks from its
/// centre both ways, a pixel a step, held inside the pixel centres, sampling the picture at the
/// `weights`, turning to the flow where it stands, ending where that has no length; the pass is
/// O + e (S - O), S the weighted mean with the pixel at 1. With `keep` the covering is kept.
pub(crate) fn contour_smear(source: &mut WorkingBuffer, field: &[[f64; 3]], weights: &[f64], keep: bool) {
    let (w, h) = (source.width(), source.height());
    let (fw, fh) = (w as f64, h as f64);
    let still = source.clone();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let [tx, ty, e] = field[i];
        if tx == 0.0 && ty == 0.0 {
            return;
        }
        let p: [f64; 4] = std::array::from_fn(|c| px[c] as f64);
        let (mut acc, mut total) = (p, 1.0);
        for side in [1.0, -1.0] {
            let mut pt = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
            let mut d = (side * tx, side * ty);
            for &wt in weights {
                pt = ((pt.0 + d.0).clamp(0.5, fw - 0.5), (pt.1 + d.1).clamp(0.5, fh - 0.5));
                let s = sample_bilinear(&still, pt.0, pt.1);
                for c in 0..4 {
                    acc[c] += s[c] as f64 * wt;
                }
                total += wt;
                let m = contour_way(field, (w, h), pt, d);
                let ln = (m.0 * m.0 + m.1 * m.1).sqrt();
                if ln <= 1e-6 {
                    break;
                }
                d = (m.0 / ln, m.1 / ln);
            }
        }
        let res: [f64; 4] = std::array::from_fn(|c| p[c] + e * (acc[c] / total - p[c]));
        if keep {
            for c in 0..3 {
                px[c] = if res[3] > 0.0 { (res[c] / res[3] * p[3]) as f32 } else { 0.0 };
            }
        } else {
            for c in 0..4 {
                px[c] = res[c] as f32;
            }
        }
    });
}

/// D-470: the encoded straight colour of a covered pixel.
fn contour_encoded(p: &[f32], a: f64) -> [f64; 3] {
    std::array::from_fn(|c| crate::grade::to_srgb((p[c] as f64 / a).clamp(0.0, 1.0)))
}

/// D-470: the palette colour nearest `v`, the first on a tie.
fn contour_nearest(palette: &[[f64; 3]], v: &[f64; 3]) -> usize {
    let mut best = (f64::INFINITY, 0);
    for (j, q) in palette.iter().enumerate() {
        let d = (q[0] - v[0]).powi(2) + (q[1] - v[1]).powi(2) + (q[2] - v[2]).powi(2);
        if d < best.0 {
            best = (d, j);
        }
    }
    best.1
}

/// D-470: Contour Paint's palette from `source` (document 21): the encoded straight colours
/// sampled every g pixels, g = max(1, floor(sqrt(w h / 4096))), cut by Heckbert's median cut to
/// `levels` boxes and each box's mean, then with `k_means` moved 8 times to the mean of the
/// samples nearest each. Empty where nothing is covered.
pub(crate) fn contour_palette(source: &WorkingBuffer, levels: usize, k_means: bool) -> Vec<[f64; 3]> {
    let (w, h) = (source.width(), source.height());
    let g = (((w * h) as f64 / 4096.0).sqrt().floor() as usize).max(1);
    let mut samples = Vec::new();
    for y in (0..h).step_by(g) {
        for x in (0..w).step_by(g) {
            let p = source.pixel(x, y);
            if p[3] > 0.0 {
                samples.push(contour_encoded(&p, p[3] as f64));
            }
        }
    }
    if samples.is_empty() {
        return Vec::new();
    }
    let spans = |b: &[[f64; 3]]| -> [f64; 3] {
        std::array::from_fn(|c| b.iter().map(|v| v[c]).fold(f64::NEG_INFINITY, f64::max) - b.iter().map(|v| v[c]).fold(f64::INFINITY, f64::min))
    };
    let widest = |s: [f64; 3]| (0..3).fold(0, |m, c| if s[c] > s[m] { c } else { m });
    let mut boxes = vec![samples.clone()];
    while boxes.len() < levels {
        let all: Vec<[f64; 3]> = boxes.iter().map(|b| spans(b)).collect();
        let i = (0..boxes.len()).fold(0, |m, j| if all[j][widest(all[j])] > all[m][widest(all[m])] { j } else { m });
        let ch = widest(all[i]);
        if all[i][ch] <= 0.0 {
            break;
        }
        let mut b = boxes.remove(i);
        b.sort_by(|p, q| p[ch].partial_cmp(&q[ch]).expect("colours are numbers"));
        let upper = b.split_off(b.len() / 2);
        boxes.insert(i, b);
        boxes.insert(i + 1, upper);
    }
    let mean = |b: &[[f64; 3]]| -> [f64; 3] { std::array::from_fn(|c| b.iter().map(|v| v[c]).sum::<f64>() / b.len() as f64) };
    let mut palette: Vec<[f64; 3]> = boxes.iter().map(|b| mean(b)).collect();
    if k_means {
        for _ in 0..8 {
            let mut sums = vec![([0.0; 3], 0usize); palette.len()];
            for v in &samples {
                let j = contour_nearest(&palette, v);
                for c in 0..3 {
                    sums[j].0[c] += v[c];
                }
                sums[j].1 += 1;
            }
            for (q, (s, n)) in palette.iter_mut().zip(sums) {
                if n > 0 {
                    *q = s.map(|v| v / n as f64);
                }
            }
        }
    }
    palette
}

/// D-584: Contour Paint's custom colours as its palette, each `#rrggbb` its encoded colour.
pub(crate) fn contour_custom(colors: &[String]) -> Vec<[f64; 3]> {
    colors.iter().filter_map(|c| crate::selective_blur::parse_hex(c)).map(|b| b.map(|v| v as f64 / 255.0)).collect()
}

/// D-584: an encoded colour as unscaled CIE L*a*b*, D-400's, for CIE76's colour difference.
pub(crate) fn contour_lab(v: [f64; 3]) -> [f64; 3] {
    let [l, p, q] = crate::grade::shadow_highlight_lab(v.map(crate::grade::to_linear));
    [100.0 * l, 128.0 * p, 128.0 * q]
}

/// D-470: Contour Paint's fills (document 21): each covered pixel whose flow strength times 100
/// lies from `low` to `high` takes the palette colour nearest its encoded straight colour, at its
/// covering; elsewhere it is kept, or cleared with `only`. D-584: `perceptual`, nearest in CIE
/// L*a*b* ([`contour_lab`]) instead.
pub(crate) fn contour_fill(source: &mut WorkingBuffer, field: &[[f64; 3]], palette: &[[f64; 3]], (low, high): (f64, f64), only: bool, perceptual: bool) {
    let linear: Vec<[f64; 3]> = palette.iter().map(|q| q.map(crate::grade::to_linear)).collect();
    let key = |v: [f64; 3]| if perceptual { contour_lab(v) } else { v };
    let keys: Vec<[f64; 3]> = palette.iter().map(|&q| key(q)).collect();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let a = px[3] as f64;
        let v = 100.0 * field[i][2];
        if a > 0.0 && low <= v && v <= high {
            let q = linear[contour_nearest(&keys, &key(contour_encoded(px, a)))];
            for c in 0..3 {
                px[c] = (q[c] * a) as f32;
            }
        } else if only {
            px.copy_from_slice(&[0.0; 4]);
        }
    });
}

/// D-470: Contour Paint's Edge Map view (document 21): each pixel opaque grey, its flow strength
/// encoded, and in cells 12 `px` square a cyan line along the flow at the pixel nearest the
/// cell's middle, 0.4 of the cell each way, laid on at that pixel's strength times
/// clamp(1 - d / px, 0, 1), d the pixel centre's distance from the line.
/// D-585: with `heads`, each line also has a head at its end along the flow: two strokes a fifth
/// of the cell long, back along the line turned 30 degrees each way.
pub(crate) fn contour_edges(source: &mut WorkingBuffer, field: &[[f64; 3]], px: f64, heads: bool) {
    let (w, h) = (source.width(), source.height());
    let cell = 12.0 * px;
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, out)| {
        let g = crate::grade::to_linear(field[i][2]);
        let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
        let (cx, cy) = (((x / cell).floor() + 0.5) * cell, ((y / cell).floor() + 0.5) * cell);
        let (qx, qy) = ((cx.floor() as usize).min(w - 1), (cy.floor() as usize).min(h - 1));
        let [tx, ty, e] = field[qy * w + qx];
        let mut c = [g, g, g];
        if tx != 0.0 || ty != 0.0 {
            let half = 0.4 * cell;
            let (vx, vy) = (x - cx, y - cy);
            let along = (vx * tx + vy * ty).clamp(-half, half);
            let mut d = ((vx - along * tx).powi(2) + (vy - along * ty).powi(2)).sqrt();
            if heads {
                let (wx, wy) = (vx - half * tx, vy - half * ty);
                for sg in [1.0, -1.0] {
                    let (bx, by) = (-tx * C30 + ty * sg * 0.5, -tx * sg * 0.5 - ty * C30);
                    let a = (wx * bx + wy * by).clamp(0.0, 0.2 * cell);
                    d = d.min(((wx - a * bx).powi(2) + (wy - a * by).powi(2)).sqrt());
                }
            }
            let cov = e * (1.0 - d / px).clamp(0.0, 1.0);
            c = [0.0, 1.0, 1.0].map(|k| g + cov * (k - g));
        }
        out.copy_from_slice(&[c[0] as f32, c[1] as f32, c[2] as f32, 1.0]);
    });
}

/// D-585: cos 30 degrees, as tools/contour_paint_reference.py writes it.
const C30: f64 = 0.8660254037844386;

/// D-404: Tiles, Motion Tile's sized tile held at the buffer's size, `scale` per cent across
/// and down round `center`, then `blend` per cent of the original mixed back.
pub(crate) fn tiles(source: &mut WorkingBuffer, scale: f64, center: [f64; 2], blend: f64) {
    if blend >= 100.0 {
        return;
    }
    let original = (blend > 0.0).then(|| source.clone());
    motion_tile(source, (100.0, 100.0), false, (center, scale, scale));
    if let Some(original) = original {
        let b = (blend / 100.0) as f32;
        source.data_mut().par_iter_mut().zip(original.data().par_iter()).for_each(|(t, o)| *t = *t * (1.0 - b) + *o * b);
    }
}

/// D-405: Magnify's centre in a buffer `dims` whose drawing's corner is at `origin`, its radius
/// and feather (times the magnification when linked), and how far it grows the buffer on every
/// side: when Resize Layer is on and nothing is linked, enough to hold the square round the area.
pub(crate) fn magnify_lens(
    center: [f64; 2],
    magnification: f64,
    link: &str,
    [size, feather]: [f64; 2],
    resize: bool,
    dims: (usize, usize),
    origin: (usize, usize),
) -> ((f64, f64), f64, f64, usize) {
    let m = magnification / 100.0;
    let r = if link == "none" { size } else { size * m };
    let f = if link == "size_feather" { feather * m } else { feather };
    let (cx, cy) = crate::effects::radial_center(center, dims, origin);
    let (w, h) = (dims.0 as f64, dims.1 as f64);
    let g = if resize && link == "none" { 0f64.max(r - cx).max(cx + r - w).max(r - cy).max(cy + r - h).ceil() as usize } else { 0 };
    ((cx, cy), r, f, g)
}

/// D-405: Magnify. `words`, the shape, link, scaling and blending mode, already valid; `numbers`,
/// the magnification, size, feather and opacity. The area round the centre is read at
/// `c + v / m`, weighed by its covering and the opacity, and laid over the buffer by the
/// blending mode (document 21's layer blend), or alone for "none". Returns the growth on every
/// side; a clear area over the buffer grows nothing, as it changes nothing.
pub(crate) fn magnify(
    source: &mut WorkingBuffer,
    [shape, link, scaling, blend]: [&str; 4],
    [magnification, size, feather, opacity]: [f64; 4],
    center: [f64; 2],
    resize: bool,
    origin: (usize, usize),
) -> usize {
    if opacity == 0.0 && blend != "none" {
        return 0;
    }
    let (w, h) = (source.width(), source.height());
    let ((cx, cy), r, f, g) = magnify_lens(center, magnification, link, [size, feather], resize, (w, h), origin);
    let m = magnification / 100.0;
    let jit = 1.0 - 1.0 / m;
    let base = crate::grade::mix(0);
    let mode = crate::model::BlendMode::from_str(blend);
    let gw = w + 2 * g;
    let drawing = std::mem::replace(source, WorkingBuffer::transparent(gw, h + 2 * g));
    let read = |i: i64, j: i64| {
        if (0..w as i64).contains(&i) && (0..h as i64).contains(&j) {
            drawing.pixel(i as usize, j as usize)
        } else {
            [0.0; 4]
        }
    };
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(n, px)| {
        let (i, j) = ((n % gw) as i64 - g as i64, (n / gw) as i64 - g as i64);
        let (vx, vy) = (i as f64 + 0.5 - cx, j as f64 + 0.5 - cy);
        let (d, inside) = if shape == "square" {
            let d = vx.abs().max(vy.abs());
            (d, d <= r)
        } else {
            let s = vx * vx + vy * vy;
            (s.sqrt(), s <= r * r)
        };
        let k = if f == 0.0 { f64::from(u8::from(inside)) } else { ((r - d) / f).clamp(0.0, 1.0) };
        let mut a = [0.0f32; 4];
        if k > 0.0 {
            let (mut qx, mut qy) = (cx + vx / m, cy + vy / m);
            let p = if scaling == "soft" {
                sample_bilinear(&drawing, qx, qy)
            } else {
                if scaling == "scatter" {
                    // Noise's hash at the pixel's place in the drawing, so it stays with it.
                    let (hx, hy) = (i - origin.0 as i64, j - origin.1 as i64);
                    qx += jit * crate::grade::unit(base, hx, hy, 0, 0);
                    qy += jit * crate::grade::unit(base, hx, hy, 0, 1);
                }
                read(qx.floor() as i64, qy.floor() as i64)
            };
            let weight = (k * (opacity / 100.0)) as f32;
            a = p.map(|v| v * weight);
        }
        px.copy_from_slice(&match mode {
            Some(mode) => crate::composite::blend_pixel(mode, a, read(i, j)),
            None => a,
        });
    });
    g
}

/// D-413: a generator's pattern laid on the layer. At each pixel `cover` gives the pattern's
/// covering at its centre, in the buffer's pixels; times `opacity`, a share, it is the pattern
/// in `color` (linear, premultiplied), laid on the pixel by `blend` (document 21's layer blend,
/// as a layer's blend mode lays a layer) or, for "none", replacing it.
pub(crate) fn lay_pattern(source: &mut WorkingBuffer, cover: impl Fn(f64, f64) -> f64 + Sync, color: [f64; 3], opacity: f64, blend: &str) {
    let w = source.width();
    let mode = crate::model::BlendMode::from_str(blend);
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(n, px)| {
        let k = cover((n % w) as f64 + 0.5, (n / w) as f64 + 0.5) * opacity;
        let s = [color[0] * k, color[1] * k, color[2] * k, k].map(|v| v as f32);
        let out = match mode {
            Some(mode) => crate::composite::blend_pixel(mode, s, [px[0], px[1], px[2], px[3]]),
            None => s,
        };
        px.copy_from_slice(&out);
    });
}

/// D-413: Checkerboard's anchor in a buffer `dims` whose drawing's corner is at `origin`, and
/// its cell's width and height: from the anchor to the corner, each at least 1 pixel, for
/// "corner_point"; `width` both ways for "width_slider"; else `width` by `height`.
pub(crate) fn checker_cells(
    anchor: [f64; 2],
    size_from: &str,
    corner: [f64; 2],
    [width, height]: [f64; 2],
    dims: (usize, usize),
    origin: (usize, usize),
) -> ((f64, f64), (f64, f64)) {
    let a = crate::effects::radial_center(anchor, dims, origin);
    let cell = match size_from {
        "corner_point" => {
            let k = crate::effects::radial_center(corner, dims, origin);
            ((k.0 - a.0).abs().max(1.0), (k.1 - a.1).abs().max(1.0))
        }
        "width_slider" => (width, width),
        _ => (width, height),
    };
    (a, cell)
}

/// D-413: Checkerboard's covering at `(x, y)`, in the buffer's pixels, cells `(w, h)` from the
/// anchor `a`. On each axis the distance to the nearer cell edge over `ramps`, half of
/// max(feather, 1), held to 1 and signed by the cell; the covering is (1 + fx fy) / 2, 1 in the
/// cell that starts at the anchor and every second one from it, a half on an edge.
pub(crate) fn checker_cover((x, y): (f64, f64), (ax, ay): (f64, f64), (w, h): (f64, f64), [rx, ry]: [f64; 2]) -> f64 {
    let axis = |x: f64, a: f64, w: f64, half: f64| {
        let u = (x - a) / w;
        let i = u.floor();
        let d = w * (u - i).min(i + 1.0 - u);
        let s = if i.rem_euclid(2.0) == 0.0 { 1.0 } else { -1.0 };
        s * (d / half).min(1.0)
    };
    (1.0 + axis(x, ax, w, rx) * axis(y, ay, h, ry)) / 2.0
}

/// D-419: Eyedropper Fill's area about the point `(px, py)` (the buffer's pixels) of radius `r`:
/// row by row, the pixels whose centres lie within `r`, the columns
/// ceil(px - 0.5 - hw) to floor(px - 0.5 + hw), hw = sqrt(max(0, r r - dy dy)),
/// dy = y + 0.5 - py; or, when no centre does, the one pixel holding the point. Returns the area's
/// pixel count, wherever they lie (those past the buffer count as clear), and its rows inside the
/// buffer as [row, first column, last column].
pub(crate) fn eyedropper_rows((px, py): (f64, f64), r: f64, (w, h): (usize, usize)) -> (f64, Vec<[usize; 3]>) {
    let mut rows = Vec::new();
    let keep = |rows: &mut Vec<[usize; 3]>, y: f64, x0: f64, x1: f64| {
        let (a, b) = (x0.max(0.0), x1.min(w as f64 - 1.0));
        if y >= 0.0 && y < h as f64 && a <= b {
            rows.push([y as usize, a as usize, b as usize]);
        }
    };
    let mut n = 0.0;
    let (mut y, last) = ((py - 0.5 - r).ceil(), (py - 0.5 + r).floor());
    while y <= last {
        let dy = y + 0.5 - py;
        let hw = (r * r - dy * dy).max(0.0).sqrt();
        let (x0, x1) = ((px - 0.5 - hw).ceil(), (px - 0.5 + hw).floor());
        if x1 >= x0 {
            n += x1 - x0 + 1.0;
            keep(&mut rows, y, x0, x1);
        }
        y += 1.0;
    }
    if n == 0.0 {
        n = 1.0;
        keep(&mut rows, py.floor(), px.floor(), px.floor());
    }
    (n, rows)
}

/// D-419: one row of Eyedropper Fill's area summed in double precision: the premultiplied
/// pixels, then the straight colours of those that show and their count.
fn eyedropper_row(source: &WorkingBuffer, [y, x0, x1]: [usize; 3]) -> [f64; 8] {
    let w = source.width();
    let mut s = [0.0; 8];
    for q in source.data()[(y * w + x0) * 4..(y * w + x1 + 1) * 4].chunks_exact(4) {
        let a = q[3] as f64;
        for c in 0..4 {
            s[c] += q[c] as f64;
        }
        if a > 0.0 {
            for c in 0..3 {
                s[4 + c] += q[c] as f64 / a;
            }
            s[7] += 1.0;
        }
    }
    s
}

/// D-419: the colour Eyedropper Fill samples, premultiplied (c A, A), from the rows'
/// sums added in order: Skip Empty the straight colours over those that show, All over all n,
/// All Premultiplied the premultiplied over n (each with A 1), Including Alpha A = P.a / n and
/// c = P.rgb / P.a; black where nothing is there to divide by.
pub(crate) fn eyedropper_colour(source: &WorkingBuffer, rows: &[[usize; 3]], n: f64, average: &str) -> [f64; 4] {
    let each: Vec<[f64; 8]> = rows.par_iter().map(|&r| eyedropper_row(source, r)).collect();
    let mut s = [0.0; 8];
    for r in &each {
        for (a, b) in s.iter_mut().zip(r) {
            *a += b;
        }
    }
    let over = |v: f64, d: f64| if d > 0.0 { v / d } else { 0.0 };
    let (c, a) = match average {
        "skip_empty" => ([4, 5, 6].map(|i| over(s[i], s[7])), 1.0),
        "all" => ([4, 5, 6].map(|i| s[i] / n), 1.0),
        "all_premultiplied" => ([0, 1, 2].map(|i| s[i] / n), 1.0),
        _ => ([0, 1, 2].map(|i| over(s[i], s[3])), s[3] / n),
    };
    [c[0] * a, c[1] * a, c[2] * a, a]
}

/// D-419: every pixel P becomes E (1 - b) + P b, E the sampled colour `e`, times the pixel's
/// own alpha when `maintain` is on.
pub(crate) fn eyedropper_fill(source: &mut WorkingBuffer, e: [f64; 4], maintain: bool, b: f64) {
    source.data_mut().par_chunks_exact_mut(4).for_each(|p| {
        let m = if maintain { p[3] as f64 } else { 1.0 };
        for (c, v) in p.iter_mut().enumerate() {
            *v = (e[c] * m * (1.0 - b) + *v as f64 * b) as f32;
        }
    });
}

/// D-440: Paint Bucket's settings for finding its area: the point in the buffer's pixels, the
/// selector, the tolerance as a share, Invert Fill, the stroke and its three distances.
pub(crate) struct Bucket<'a> {
    pub point: (f64, f64),
    pub selector: &'a str,
    pub tolerance: f64,
    pub invert: bool,
    pub stroke: &'a str,
    pub radius: f64,
    pub width: f64,
    pub softness: f64,
}

/// D-440: which pixels match the pixel holding the point (step 2 of D-440: each pixel's alpha a
/// and straight colour encoded e, held to 0..1, 0 where a is 0, against the point pixel's own),
/// with the point pixel's index; None when the point holds no pixel.
pub(crate) fn bucket_matches(source: &WorkingBuffer, b: &Bucket) -> Option<(Vec<bool>, usize)> {
    let (w, h) = (source.width(), source.height());
    let (i, j) = (b.point.0.floor(), b.point.1.floor());
    if i < 0.0 || j < 0.0 || i >= w as f64 || j >= h as f64 {
        return None;
    }
    let seed = j as usize * w + i as usize;
    let values = |p: &[f32]| {
        let a = p[3] as f64;
        if a <= 0.0 {
            ([0.0; 3], 0.0)
        } else {
            ([0, 1, 2].map(|c| crate::grade::to_srgb((p[c] as f64 / a).clamp(0.0, 1.0))), a)
        }
    };
    let (e0, a0) = values(&source.data()[seed * 4..seed * 4 + 4]);
    let t = b.tolerance;
    let ok = source
        .data()
        .par_chunks_exact(4)
        .map(|p| {
            let (e, a) = values(p);
            let d = match b.selector {
                "color_and_alpha" => (0..3).map(|c| (e[c] * a - e0[c] * a0).abs()).fold((a - a0).abs(), f64::max),
                "straight_color" => (0..3).map(|c| (e[c] - e0[c]).abs()).fold(0.0, f64::max),
                "transparency" => a,
                "opacity" => 1.0 - a,
                _ => (a - a0).abs(),
            };
            d <= t
        })
        .collect();
    Some((ok, seed))
}

/// D-440: each pixel's squared distance to the nearest pixel that is `want` in `got`, every place
/// past the buffer counted as outside the area (as `want` false), by [`squared_distances`] down
/// then across a buffer inside a ring one pixel wide.
fn bucket_distances(got: &[bool], (w, h): (usize, usize), want: bool) -> Vec<f64> {
    let (pw, ph) = (w + 2, h + 2);
    let far = 1e20;
    let columns: Vec<Vec<f64>> = (0..pw)
        .into_par_iter()
        .map(|x| {
            let f: Vec<f64> = (0..ph)
                .map(|y| {
                    let inside = x >= 1 && y >= 1 && x <= w && y <= h;
                    let is = if inside { got[(y - 1) * w + x - 1] } else { false };
                    if is == want { 0.0 } else { far }
                })
                .collect();
            squared_distances(&f)
        })
        .collect();
    (0..h)
        .into_par_iter()
        .flat_map_iter(|y| {
            let d = squared_distances(&(0..pw).map(|x| columns[x][y + 1]).collect::<Vec<_>>());
            d[1..=w].to_vec()
        })
        .collect()
}

/// D-440: Paint Bucket's area after its stroke, 1 in it and 0 out, before the blur: the matching
/// pixels joined to the point pixel up, down, left and right (every match for alpha_channel),
/// turned over by Invert Fill, then spread, choked or stroked by distances between pixel
/// centres. The flood fill and the distances are on the processor on either path.
pub(crate) fn bucket_area(source: &WorkingBuffer, b: &Bucket) -> Vec<f32> {
    let (w, h) = (source.width(), source.height());
    let mut got = vec![false; w * h];
    if let Some((ok, seed)) = bucket_matches(source, b) {
        if b.selector == "alpha_channel" {
            got = ok;
        } else if ok[seed] {
            got[seed] = true;
            let mut todo = std::collections::VecDeque::from([seed]);
            while let Some(k) = todo.pop_front() {
                let (x, y) = (k % w, k / w);
                let near = [(x > 0).then(|| k - 1), (x + 1 < w).then(|| k + 1), (y > 0).then(|| k - w), (y + 1 < h).then(|| k + w)];
                for n in near.into_iter().flatten() {
                    if ok[n] && !got[n] {
                        got[n] = true;
                        todo.push_back(n);
                    }
                }
            }
        }
    }
    if b.invert {
        got.iter_mut().for_each(|g| *g = !*g);
    }
    let keep: Vec<bool> = match b.stroke {
        "spread" => {
            let d = bucket_distances(&got, (w, h), true);
            got.iter().zip(&d).map(|(&g, d)| g || d.sqrt() <= b.radius).collect()
        }
        "choke" => {
            let d = bucket_distances(&got, (w, h), false);
            got.iter().zip(&d).map(|(&g, d)| g && d.sqrt() > b.radius).collect()
        }
        "stroke" => {
            let d = bucket_distances(&got, (w, h), false);
            got.iter().zip(&d).map(|(&g, d)| g && d.sqrt() <= b.width).collect()
        }
        _ => got,
    };
    keep.into_iter().map(|g| if g { 1.0 } else { 0.0 }).collect()
}

/// D-440: the blur's taps: for feather document 21's Gaussian, sigma softness / 2; else a box
/// three pixels wide.
pub(crate) fn bucket_taps(b: &Bucket) -> Vec<f32> {
    if b.stroke == "feather" { crate::effects::gaussian_weights(b.softness / 2.0) } else { vec![1.0 / 3.0; 3] }
}

/// D-440: Paint Bucket. With `view` every pixel opaque, white where it matches and black
/// elsewhere; else the area blurred across then down, its edges held, times `opacity` (and the
/// pixel's own alpha for straight_color) laid in `color` (linear) by `blend` as
/// [`lay_pattern`] lays a pattern, fill_only replacing the pixel.
pub(crate) fn paint_bucket(source: &mut WorkingBuffer, b: &Bucket, view: bool, color: [f64; 3], opacity: f64, blend: &str) {
    let (w, h) = (source.width(), source.height());
    if view {
        let ok = bucket_matches(source, b).map_or_else(|| vec![false; w * h], |(ok, _)| ok);
        source.data_mut().par_chunks_exact_mut(4).zip(ok).for_each(|(p, g)| {
            p.copy_from_slice(&if g { [1.0; 4] } else { [0.0, 0.0, 0.0, 1.0] });
        });
        return;
    }
    let mut field = WorkingBuffer::transparent(w, h);
    for (p, v) in field.data_mut().chunks_exact_mut(4).zip(bucket_area(source, b)) {
        p[0] = v;
    }
    let taps = bucket_taps(b);
    crate::effects::held_blur_axes(&mut field, &taps, (true, false));
    crate::effects::held_blur_axes(&mut field, &taps, (false, true));
    let straight = b.selector == "straight_color";
    let cover: Vec<f64> = field
        .data()
        .chunks_exact(4)
        .zip(source.data().chunks_exact(4))
        .map(|(m, p)| m[0] as f64 * if straight { p[3] as f64 } else { 1.0 })
        .collect();
    let blend = if blend == "fill_only" { "none" } else { blend };
    lay_pattern(source, |x, y| cover[y as usize * w + x as usize], color, opacity, blend);
}

/// D-418: Fill's feathers as the taps across and down (document 21's Gaussian, sigma feather / 2)
/// and how far the covering must reach past the buffer for both: the larger radius.
pub(crate) fn fill_taps(feathers: [f64; 2]) -> ([Vec<f32>; 2], usize) {
    let taps = feathers.map(|f| crate::effects::gaussian_weights(f / 2.0));
    let r = taps[0].len().max(taps[1].len()) / 2;
    (taps, r)
}

/// D-418: Fill's covering before its feathers, of the buffer `(w, h)` grown by `r` on every
/// side, row by row: each path, moved into the buffer by `origin`, covered as ADR-016 covers a
/// mask (`mask::scanline_field`), the paths joined as a mask's Add joins them,
/// 1 - (1 - m1)(1 - m2)...
pub(crate) fn fill_cover(paths: &[Vec<(f64, f64)>], (ox, oy): (usize, usize), (w, h): (usize, usize), r: usize) -> Vec<f32> {
    let (fw, fh) = (w + 2 * r, h + 2 * r);
    let mut rest = vec![1.0f32; fw * fh];
    for path in paths {
        let moved: Vec<(f64, f64)> = path.iter().map(|&(x, y)| (x + ox as f64, y + oy as f64)).collect();
        for (v, m) in rest.iter_mut().zip(crate::mask::scanline_field(&moved, fw, fh, r)) {
            *v *= 1.0 - m;
        }
    }
    rest.iter_mut().for_each(|v| *v = 1.0 - *v);
    rest
}

/// D-418: Fill. The covering m is 1 with no paths (the whole layer), else [`fill_cover`]
/// feathered across and down, each with its edges held: the covering reaches `r` past the
/// buffer, so every pixel reads it as it lies on the plane. Invert makes it 1 - m; with
/// k = m * opacity each premultiplied pixel P becomes P (1 - k) + C P.a k, its alpha kept.
pub(crate) fn fill(source: &mut WorkingBuffer, paths: &[Vec<(f64, f64)>], origin: (usize, usize), feathers: [f64; 2], color: [f64; 3], invert: bool, opacity: f64) {
    let (w, h) = (source.width(), source.height());
    let ([tx, ty], r) = fill_taps(feathers);
    let field = (!paths.is_empty()).then(|| {
        let cover = fill_cover(paths, origin, (w, h), r);
        let mut grown = WorkingBuffer::transparent(w + 2 * r, h + 2 * r);
        for (p, v) in grown.data_mut().chunks_exact_mut(4).zip(&cover) {
            p[0] = *v;
        }
        crate::effects::held_blur_axes(&mut grown, &tx, (true, false));
        crate::effects::held_blur_axes(&mut grown, &ty, (false, true));
        grown
    });
    let fw = w + 2 * r;
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, p)| {
        let m = field.as_ref().map_or(1.0, |f| f.data()[((i / w + r) * fw + i % w + r) * 4] as f64);
        let k = if invert { 1.0 - m } else { m } * opacity;
        let a = p[3] as f64;
        for (c, v) in p[..3].iter_mut().enumerate() {
            *v = (*v as f64 * (1.0 - k) + color[c] * a * k) as f32;
        }
    });
}

/// D-502: the Chebyshev distance from every pixel to the nearest one of `region` equal to
/// `want`, by the two passes of the 3 by 3 chamfer whose steps all cost 1 (exact for that
/// distance); `None` when there is none.
fn chessboard(region: &[u8], want: u8, (w, h): (usize, usize)) -> Option<Vec<u32>> {
    if !region.contains(&want) {
        return None;
    }
    let far = u32::MAX / 2;
    let mut d: Vec<u32> = region.iter().map(|&r| if r == want { 0 } else { far }).collect();
    for y in 0..h {
        for x in 0..w {
            let mut v = d[y * w + x];
            if x > 0 {
                v = v.min(d[y * w + x - 1] + 1);
            }
            if y > 0 {
                for nx in x.saturating_sub(1)..(x + 2).min(w) {
                    v = v.min(d[(y - 1) * w + nx] + 1);
                }
            }
            d[y * w + x] = v;
        }
    }
    for y in (0..h).rev() {
        for x in (0..w).rev() {
            let mut v = d[y * w + x];
            if x + 1 < w {
                v = v.min(d[y * w + x + 1] + 1);
            }
            if y + 1 < h {
                for nx in x.saturating_sub(1)..(x + 2).min(w) {
                    v = v.min(d[(y + 1) * w + nx] + 1);
                }
            }
            d[y * w + x] = v;
        }
    }
    Some(d)
}

/// D-502: Inner/Outer Key, the rule ours (document 21, `tools/inner_outer_key_reference.py`).
/// `paths` holds the foreground's outline and, in the two-mask way, the background's, in the
/// drawing's own space; `origin` is where the drawing's corner lies in the buffer. Pixels are
/// kept inside the foreground, gone outside the background (one mask: beyond `radius` of the
/// outline, inside or out), and the rest worked out between the colours of the nearest kept
/// and gone pixels, the edge colour taken back out; then the matte is thinned, feathered and
/// thresholded and mixed back with the picture by `blend` (a share). The layer never grows.
#[allow(clippy::too_many_arguments)]
pub(crate) fn inner_outer_key(source: &mut WorkingBuffer, paths: &[Vec<(f64, f64)>], origin: (usize, usize), radius: f64, thin: f64, feather: f64, threshold: f64, invert: bool, blend: f64) {
    let (w, h) = (source.width(), source.height());
    let n = w * h;
    if n == 0 || paths.is_empty() {
        return;
    }
    let cin = fill_cover(&paths[..1], origin, (w, h), 0);
    let cout = (paths.len() > 1).then(|| fill_cover(&paths[1..2], origin, (w, h), 0));
    // The single mask's band: every pixel whose centre lies within `radius` of a side, row by
    // row in parallel.
    let band = cout.is_none().then(|| {
        let mut near = vec![false; n];
        let p = &paths[0];
        let (ox, oy) = (origin.0 as f64, origin.1 as f64);
        let span = |lo: f64, hi: f64, o: f64, len: usize| {
            let first = (lo - radius - 0.5 + o).floor().max(0.0);
            let last = (hi + radius - 0.5 + o).ceil().min(len as f64 - 1.0);
            (first as usize, last)
        };
        near.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
            for i in 0..p.len() {
                let (a, b) = (p[i], p[(i + 1) % p.len()]);
                let (y0, y1) = span(a.1.min(b.1), a.1.max(b.1), oy, h);
                if y1 < 0.0 || y < y0 || y > y1 as usize {
                    continue;
                }
                let (x0, x1) = span(a.0.min(b.0), a.0.max(b.0), ox, w);
                if x1 < 0.0 {
                    continue;
                }
                let py = y as f64 - oy + 0.5;
                for x in x0..=x1 as usize {
                    if !row[x] && crate::mask::distance_to_segment(a, b, x as f64 - ox + 0.5, py) <= radius {
                        row[x] = true;
                    }
                }
            }
        });
        near
    });
    // 0 unknown, 1 kept, 2 gone.
    let region: Vec<u8> = (0..n)
        .into_par_iter()
        .map(|i| {
            let inside = cin[i] >= 0.5;
            let (k, g) = match (&cout, &band) {
                (Some(c), _) => (inside, !inside && c[i] < 0.5),
                (None, Some(near)) => (inside && !near[i], !inside && !near[i]),
                (None, None) => unreachable!(),
            };
            let (k, g) = if invert { (g, k) } else { (k, g) };
            if k { 1 } else if g { 2 } else { 0 }
        })
        .collect();
    // Summed-area tables of each set's premultiplied colour and alpha, (w + 1) by (h + 1): each
    // row's running sums in parallel, then the rows added down, the same sums in the same order.
    let table = |want: u8| {
        let mut s = vec![[0.0f64; 4]; (w + 1) * (h + 1)];
        let data = source.data();
        s.par_chunks_mut(w + 1).skip(1).enumerate().for_each(|(y, out)| {
            let mut row = [0.0f64; 4];
            for x in 0..w {
                if region[y * w + x] == want {
                    for c in 0..4 {
                        row[c] += data[(y * w + x) * 4 + c] as f64;
                    }
                }
                out[x + 1] = row;
            }
        });
        for y in 1..h {
            let (done, rest) = s.split_at_mut((y + 1) * (w + 1));
            let above = &done[y * (w + 1)..];
            for (o, a) in rest[..w + 1].iter_mut().zip(above) {
                *o = [o[0] + a[0], o[1] + a[1], o[2] + a[2], o[3] + a[3]];
            }
        }
        s
    };
    let ((dk, sk), (dg, sg)) = rayon::join(|| (chessboard(&region, 1, (w, h)), table(1)), || (chessboard(&region, 2, (w, h)), table(2)));
    // E_S: the straight colour of the set's pixels in the square of half-side d about (x, y).
    let mean = |s: &[[f64; 4]], d: u32, x: usize, y: usize| -> [f64; 3] {
        let d = d as usize;
        let (x0, y0, x1, y1) = (x.saturating_sub(d), y.saturating_sub(d), (x + d + 1).min(w), (y + d + 1).min(h));
        let at = |x: usize, y: usize| s[y * (w + 1) + x];
        let (a, b, c, e) = (at(x1, y1), at(x0, y1), at(x1, y0), at(x0, y0));
        let sum: [f64; 4] = [0, 1, 2, 3].map(|k| a[k] - b[k] - c[k] + e[k]);
        if sum[3] > 0.0 {
            [sum[0] / sum[3], sum[1] / sum[3], sum[2] / sum[3]]
        } else {
            [0.0; 3]
        }
    };
    let data = source.data();
    // The extraction and the kept colour of each pixel.
    let worked: Vec<(f64, [f64; 3])> = (0..n)
        .into_par_iter()
        .map(|i| {
            let (x, y) = (i % w, i / w);
            let p = &data[i * 4..i * 4 + 4];
            let a = p[3] as f64;
            let c: [f64; 3] = if a > 0.0 { [p[0] as f64 / a, p[1] as f64 / a, p[2] as f64 / a] } else { [0.0; 3] };
            let f = dk.as_ref().map_or([0.0; 3], |d| mean(&sk, d[i], x, y));
            let b = dg.as_ref().map_or([0.0; 3], |d| mean(&sg, d[i], x, y));
            let al = match (region[i], &dk, &dg) {
                (1, _, _) => 1.0,
                (2, _, _) => 0.0,
                (_, None, _) => 0.0,
                (_, _, None) => 1.0,
                (_, Some(k), Some(g)) => {
                    let v = [f[0] - b[0], f[1] - b[1], f[2] - b[2]];
                    let vv = v[0] * v[0] + v[1] * v[1] + v[2] * v[2];
                    if vv > 1e-12 {
                        (((c[0] - b[0]) * v[0] + (c[1] - b[1]) * v[1] + (c[2] - b[2]) * v[2]) / vv).clamp(0.0, 1.0)
                    } else {
                        g[i] as f64 / (k[i] + g[i]) as f64
                    }
                }
            };
            let fc = match region[i] {
                1 => c,
                0 if al > 0.0 => [0, 1, 2].map(|k| ((c[k] - (1.0 - al) * b[k]) / al).clamp(0.0, 1.0)),
                _ => f,
            };
            (al, fc)
        })
        .collect();
    let mut m: Vec<f32> = worked.iter().map(|&(a, _)| a as f32).collect();
    // Edge Thin: the least (thinning) or greatest (growing) within the disc, inside the buffer.
    if thin != 0.0 {
        let pick: fn(f32, f32) -> f32 = if thin > 0.0 { f32::min } else { f32::max };
        let runs = disc_runs(thin.abs());
        let top = runs.iter().map(|&(_, hw)| hw).max().unwrap_or(0);
        let mut wide = m.clone();
        let mut out = vec![if thin > 0.0 { f32::INFINITY } else { f32::NEG_INFINITY }; n];
        for k in 0..=top {
            if k > 0 {
                // Each row's extreme within k, from its extreme within k - 1.
                let prev = wide.clone();
                wide.par_chunks_mut(w).zip(prev.par_chunks(w)).for_each(|(row, was)| {
                    for x in 0..w {
                        let mut v = was[x];
                        if x > 0 {
                            v = pick(v, was[x - 1]);
                        }
                        if x + 1 < w {
                            v = pick(v, was[x + 1]);
                        }
                        row[x] = v;
                    }
                });
            }
            for &(dy, _) in runs.iter().filter(|&&(_, hw)| hw == k) {
                out.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
                    let sy = y as isize + dy;
                    if sy >= 0 && (sy as usize) < h {
                        for (o, &v) in row.iter_mut().zip(&wide[sy as usize * w..(sy as usize + 1) * w]) {
                            *o = pick(*o, v);
                        }
                    }
                });
            }
        }
        m = out;
    }
    // Edge Feather: document 21's Gaussian, the buffer's edges held.
    if feather > 0.0 {
        let mut b = WorkingBuffer::transparent(w, h);
        for (p, &v) in b.data_mut().chunks_exact_mut(4).zip(&m) {
            p[0] = v;
        }
        crate::effects::held_blur_axes(&mut b, &crate::effects::gaussian_weights(feather / 2.0), (true, true));
        m = b.data().chunks_exact(4).map(|p| p[0]).collect();
    }
    let q = threshold / 100.0;
    source.data_mut().par_chunks_exact_mut(4).zip(worked.par_iter()).zip(m.par_iter()).for_each(|((p, (_, fc)), &v)| {
        let v = v as f64;
        let v = if q < 1.0 { ((v - q) / (1.0 - q)).clamp(0.0, 1.0) } else if v >= 1.0 { 1.0 } else { 0.0 };
        let a = p[3] as f64;
        let out = [a * v * fc[0], a * v * fc[1], a * v * fc[2], a * v];
        for (c, o) in p.iter_mut().zip(out) {
            *c = (o * (1.0 - blend) + *c as f64 * blend) as f32;
        }
    });
}

/// D-417: Grid's covering at `(x, y)`, in the buffer's pixels, cells `(w, h)` from the anchor
/// `a`, lines `border` thick along the cells' edges. On each axis the share of a box `ramps`
/// wide (max(feather, 1)) about the pixel that falls inside the two nearest lines, held to 1;
/// the covering is gx + gy - gx gy, 1 minus that when `invert`.
pub(crate) fn grid_cover((x, y): (f64, f64), (ax, ay): (f64, f64), (w, h): (f64, f64), border: f64, [rx, ry]: [f64; 2], invert: bool) -> f64 {
    let line = |d: f64, r: f64| (((d + r / 2.0).min(border / 2.0) - (d - r / 2.0).max(-border / 2.0)) / r).clamp(0.0, 1.0);
    let axis = |x: f64, a: f64, w: f64, r: f64| {
        let u = (x - a) / w;
        let t = u - u.floor();
        (line(w * t, r) + line(w * (1.0 - t), r)).min(1.0)
    };
    let (gx, gy) = (axis(x, ax, w, rx), axis(y, ay, h, ry));
    let c = gx + gy - gx * gy;
    if invert { 1.0 - c } else { c }
}

/// D-414: Circle's ring, `[outer, inner, outer ramp, inner ramp]`: the radii it runs between
/// and the widths of the ramps on them, max(feather, 1). "none" is a disk; "edge_radius" runs
/// between the radius and the edge radius; "thickness" inward from the radius; the two
/// "*_radius" edges read the thickness (and, for "thickness_feather_radius", the feathers) as
/// per cent of the radius.
pub(crate) fn circle_ring(radius: f64, edge: &str, edge_thickness: f64, [outer, inner]: [f64; 2]) -> [f64; 4] {
    let share = |v: f64| v * radius / 100.0;
    let (t, fo, fi) = match edge {
        "thickness_radius" => (share(edge_thickness), outer, inner),
        "thickness_feather_radius" => (share(edge_thickness), share(outer), share(inner)),
        _ => (edge_thickness, outer, inner),
    };
    let (ro, ri) = match edge {
        "none" => (radius, 0.0),
        "edge_radius" => (radius.max(t), radius.min(t)),
        _ => (radius, (radius - t).max(0.0)),
    };
    [ro, ri, fo.max(1.0), fi.max(1.0)]
}

/// D-414: Circle's covering at `(x, y)`, in the buffer's pixels, centre `c`: a straight ramp
/// on the outer radius, times one on the inner when it is past 0, each centred on its edge;
/// 1 minus that when `invert`.
pub(crate) fn circle_cover((x, y): (f64, f64), c: (f64, f64), [ro, ri, fo, fi]: [f64; 4], invert: bool) -> f64 {
    let d = (x - c.0).hypot(y - c.1);
    let mut k = ((ro - d) / fo + 0.5).clamp(0.0, 1.0);
    if ri > 0.0 {
        k *= ((d - ri) / fi + 0.5).clamp(0.0, 1.0);
    }
    if invert { 1.0 - k } else { k }
}

/// D-207 (Beam) and D-415 (Ellipse): a line `2 r` thick seen `d` from its middle, `softness` per
/// cent of it a ramp at least a pixel wide: the covering, and the share of the outside colour.
pub(crate) fn line_profile(d: f64, r: f64, softness: f64) -> (f64, f64) {
    let sw = (2.0 * r * softness / 100.0).max(1.0);
    let c = (((d + sw / 2.0).min(r) - (d - sw / 2.0).max(-r)) / sw).clamp(0.0, 1.0);
    let q = if r == 0.0 { 1.0 } else { (d / r).min(1.0) };
    (c, q)
}

/// D-415: how far `(u, v)`, from the centre, lies from the outline of the ellipse with half axes
/// `a` and `b`, to first order: |k - 1| k / g, k the ellipse's level and g its gradient (exact
/// for a circle); min(a, b) at the centre, where g is 0.
pub(crate) fn ellipse_distance(u: f64, v: f64, a: f64, b: f64) -> f64 {
    let g = (u * u / a.powi(4) + v * v / b.powi(4)).sqrt();
    if g == 0.0 {
        return a.min(b);
    }
    let k = (u * u / (a * a) + v * v / (b * b)).sqrt();
    (k - 1.0).abs() * k / g
}

/// D-415: After Effects' Ellipse. Its outline, `thickness` pixels across, drawn as Beam's line
/// is (`line_profile`), over the layer or `alone`. `c` the centre in the buffer's pixels, `a`
/// and `b` the half axes.
pub(crate) fn ellipse(source: &mut WorkingBuffer, c: (f64, f64), [a, b, thickness, softness]: [f64; 4], [inside, outside]: [[f64; 3]; 2], alone: bool) {
    let w = source.width();
    let r = thickness / 2.0;
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let d = ellipse_distance((i % w) as f64 + 0.5 - c.0, (i / w) as f64 + 0.5 - c.1, a, b);
            let (k, q) = line_profile(d, r, softness);
            let keep = if alone { 0.0 } else { 1.0 - k };
            for ch in 0..3 {
                px[ch] = (px[ch] as f64 * keep + ((1.0 - q) * inside[ch] + q * outside[ch]) * k) as f32;
            }
            px[3] = (px[3] as f64 * keep + k) as f32;
        });
}

/// D-420 (Audio Spectrum) and D-421 (Audio Waveform): one straight piece of a mark, a dot when
/// its ends meet: its ends x0 y0 x1 y1 in the buffer's pixels, the mark it belongs to, and its
/// inside and outside colours in linear light.
pub(crate) type Piece = [f64; 11];

/// The marks one effect draws, in order: `r` half their thickness, `softness` Beam's (D-207),
/// `how` 0 over the layer, 1 alone, 2 added to it, `blend` Blend Overlapping Colors.
#[derive(Default)]
pub(crate) struct Marks {
    pub pieces: Vec<Piece>,
    pub r: f64,
    pub softness: f64,
    pub how: u8,
    pub blend: bool,
}

impl Marks {
    /// Each piece's box, left right top bottom, out of which it covers nothing: its ends, out by
    /// `r` and half the ramp.
    pub(crate) fn boxes(&self) -> Vec<[f64; 4]> {
        let reach = self.r + (2.0 * self.r * self.softness / 100.0).max(1.0) / 2.0;
        self.pieces
            .iter()
            .map(|p| [p[0].min(p[2]) - reach, p[0].max(p[2]) + reach, p[1].min(p[3]) - reach, p[1].max(p[3]) + reach])
            .collect()
    }

    /// For each band of 16 rows of a buffer `h` tall, the pieces whose boxes reach it, in order.
    pub(crate) fn bands(boxes: &[[f64; 4]], h: usize) -> Vec<Vec<u32>> {
        let mut bands = vec![Vec::new(); h.div_ceil(16)];
        if h == 0 {
            return bands;
        }
        for (j, b) in boxes.iter().enumerate() {
            let rows = (b[2] - 0.5).floor().max(0.0) as usize..=((b[3] - 0.5).ceil().max(0.0) as usize).min(h - 1);
            for band in bands.iter_mut().take(rows.end() / 16 + 1).skip(rows.start() / 16) {
                band.push(j as u32);
            }
        }
        bands
    }
}

/// How far `(x, y)` lies from a piece.
fn piece_distance(p: &Piece, x: f64, y: f64) -> f64 {
    let (dx, dy) = (p[2] - p[0], p[3] - p[1]);
    let l2 = dx * dx + dy * dy;
    let t = if l2 == 0.0 { 0.0 } else { (((x - p[0]) * dx + (y - p[1]) * dy) / l2).clamp(0.0, 1.0) };
    let (ex, ey) = (x - p[0] - t * dx, y - p[1] - t * dy);
    (ex * ex + ey * ey).sqrt()
}

/// D-420: the marks drawn. A mark's distance is the least to its pieces and its colours the
/// nearest piece's (the first of equals); its covering and colour are Beam's (`line_profile`).
/// Blend off, each mark laid over the last; on, the covering 1 - prod(1 - c) and the colour the
/// mean weighted by c. A piece is looked at only within its box, the pixel's band's pieces only:
/// out of it a mark covers nothing, so leaving it out changes nothing.
pub(crate) fn draw_marks(source: &mut WorkingBuffer, m: &Marks) {
    let w = source.width();
    let boxes = m.boxes();
    let bands = Marks::bands(&boxes, source.height());
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
        let (mut acc, mut total, mut weight, mut clear) = ([0.0f64; 4], [0.0f64; 3], 0.0, 1.0);
        let mut lay = |d: f64, p: &Piece| {
            let (c, q) = line_profile(d, m.r, m.softness);
            for ch in 0..3 {
                let l = (1.0 - q) * p[5 + ch] + q * p[8 + ch];
                if m.blend {
                    total[ch] += l * c;
                } else {
                    acc[ch] = l * c + acc[ch] * (1.0 - c);
                }
            }
            if m.blend {
                weight += c;
                clear *= 1.0 - c;
            } else {
                acc[3] = c + acc[3] * (1.0 - c);
            }
        };
        let mut best: Option<(f64, usize)> = None;
        for &j in &bands[i / w / 16] {
            let (j, b) = (j as usize, boxes[j as usize]);
            if x <= b[0] || x >= b[1] || y <= b[2] || y >= b[3] {
                continue;
            }
            if let Some((d, k)) = best {
                if m.pieces[k][4] != m.pieces[j][4] {
                    lay(d, &m.pieces[k]);
                    best = None;
                }
            }
            let d = piece_distance(&m.pieces[j], x, y);
            if best.is_none_or(|(e, _)| d < e) {
                best = Some((d, j));
            }
        }
        if let Some((d, k)) = best {
            lay(d, &m.pieces[k]);
        }
        if m.blend {
            acc[3] = 1.0 - clear;
            for ch in 0..3 {
                acc[ch] = if weight > 0.0 { total[ch] / weight * acc[3] } else { 0.0 };
            }
        }
        let p = [px[0] as f64, px[1] as f64, px[2] as f64, px[3] as f64];
        let out = match m.how {
            1 => acc,
            2 => [p[0] + acc[0], p[1] + acc[1], p[2] + acc[2], (p[3] + acc[3]).min(1.0)],
            _ => std::array::from_fn(|ch| acc[ch] + p[ch] * (1.0 - acc[3])),
        };
        for ch in 0..4 {
            px[ch] = out[ch] as f32;
        }
    });
}

/// One piece from `a` to `b` of mark `group`, coloured `inside` and `outside`.
fn piece(a: (f64, f64), b: (f64, f64), group: usize, [inside, outside]: [[f64; 3]; 2]) -> Piece {
    [a.0, a.1, b.0, b.1, group as f64, inside[0], inside[1], inside[2], outside[0], outside[1], outside[2]]
}

/// D-420 (and D-421): the marks through each band's tips, `feet` each band's foot and facing,
/// `closed` whether the way goes round, the tips `heights` out along the facing (Side A) or
/// back (Side B). Digital: a piece from foot to tip, or tip to tip for both sides; Analog Lines:
/// one line through each side's tips, band b's colours from tip b to tip b + 1 (and from the
/// last to the first when closed); Analog Dots: a dot at each tip, band by band.
pub(crate) fn mark_pieces(feet: &[((f64, f64), (f64, f64))], closed: bool, heights: &[f64], colours: &[[[f64; 3]; 2]], display: &str, side: &str) -> Vec<Piece> {
    let sides: &[f64] = match side {
        "side_a" => &[1.0],
        "side_b" => &[-1.0],
        _ => &[1.0, -1.0],
    };
    let tip = |b: usize, s: f64| {
        let ((x, y), (nx, ny)) = feet[b];
        (x + s * heights[b] * nx, y + s * heights[b] * ny)
    };
    let n = feet.len();
    let mut out = Vec::new();
    match display {
        "digital" => {
            for b in 0..n {
                let (p, q) = if sides.len() == 2 { (tip(b, -1.0), tip(b, 1.0)) } else { (feet[b].0, tip(b, sides[0])) };
                out.push(piece(p, q, b, colours[b]));
            }
        }
        "analog_lines" => {
            for (g, &s) in sides.iter().enumerate() {
                if n == 1 {
                    out.push(piece(tip(0, s), tip(0, s), g, colours[0]));
                    continue;
                }
                for b in 0..n - 1 {
                    out.push(piece(tip(b, s), tip(b + 1, s), g, colours[b]));
                }
                if closed {
                    out.push(piece(tip(n - 1, s), tip(0, s), g, colours[n - 1]));
                }
            }
        }
        _ => {
            for b in 0..n {
                for &s in sides {
                    let g = out.len();
                    out.push(piece(tip(b, s), tip(b, s), g, colours[b]));
                }
            }
        }
    }
    out
}

/// D-420: Audio Spectrum's marks in the pixels of a buffer `size` whose drawing starts at
/// `origin`, from the levels and outline compose filled in; `None` when it filled in none. The
/// bands stand at (b + 1/2) / B of the way: along the line from Start Point to End Point, facing
/// to its left; round Start Point from straight up, clockwise (Use Polar Path); or round the
/// mask Path names. Each band's colours are turned round the hue by Hue Interpolation times its
/// share, counted from the loudest band with Dynamic Hue Phase and folded with Color Symmetry.
pub(crate) fn spectrum_marks(effect: &crate::effects::Effect, size: (usize, usize), (ox, oy): (usize, usize)) -> Option<Marks> {
    let crate::effects::Effect::AudioSpectrum {
        start_point,
        end_point,
        use_polar_path,
        maximum_height,
        thickness,
        softness,
        inside_color,
        outside_color,
        blend_overlapping_colors,
        hue_interpolation,
        dynamic_hue_phase,
        color_symmetry,
        display_options,
        side_options,
        composite,
        levels: Some(levels),
        outline,
        ..
    } = effect
    else {
        return None;
    };
    let bands = levels.len();
    let (feet, closed) = sound_feet(bands, outline.as_deref(), use_polar_path == "on", [*start_point, *end_point], size, (ox, oy));
    let heights: Vec<f64> = levels.iter().map(|a| maximum_height * a.min(1.0)).collect();
    let loudest = if dynamic_hue_phase == "on" {
        levels.iter().enumerate().fold((0, f64::NEG_INFINITY), |(m, top), (b, &a)| if a > top { (b, a) } else { (m, top) }).0
    } else {
        0
    };
    let turned = |hex: &str, degrees: f64| {
        let c = crate::effects::encoded(hex);
        let c = if degrees == 0.0 {
            c
        } else {
            let [h, s, l] = crate::grade::to_hsl(c);
            crate::grade::from_hsl([h + degrees, s, l])
        };
        c.map(crate::grade::to_linear)
    };
    let colours: Vec<[[f64; 3]; 2]> = (0..bands)
        .map(|b| {
            let u = (((b + bands - loudest) % bands) as f64 + 0.5) / bands as f64;
            let u = if color_symmetry == "on" { 1.0 - (2.0 * u - 1.0).abs() } else { u };
            let turn = hue_interpolation * u;
            [turned(inside_color, turn), turned(outside_color, turn)]
        })
        .collect();
    Some(Marks {
        pieces: mark_pieces(&feet, closed, &heights, &colours, display_options, side_options),
        r: thickness / 2.0,
        softness: *softness,
        how: if composite == "on" { 0 } else { 1 },
        blend: blend_overlapping_colors == "on",
    })
}

/// D-420, D-421: where `n` marks stand, at (b + 1/2) / n of the way, and the way they face, and
/// whether the way is closed: round the mask `outline` (the drawing's own pixels); round the
/// first of `points` from straight up, clockwise (`polar`); or along the line from the first of
/// `points` to the second, facing to its left.
fn sound_feet(n: usize, outline: Option<&[(f64, f64)]>, polar: bool, [start, end]: [[f64; 2]; 2], size: (usize, usize), (ox, oy): (usize, usize)) -> (Vec<((f64, f64), (f64, f64))>, bool) {
    let t: Vec<f64> = (0..n).map(|b| (b as f64 + 0.5) / n as f64).collect();
    match outline {
        Some(o) => {
            let o: Vec<(f64, f64)> = o.iter().map(|&(x, y)| (x + ox as f64, y + oy as f64)).collect();
            (t.iter().map(|&u| crate::along::point_along(&o, u)).collect(), true)
        }
        None if polar => {
            let p = crate::effects::radial_center(start, size, (ox, oy));
            let at = |u: f64| -std::f64::consts::PI / 2.0 + 2.0 * std::f64::consts::PI * u;
            (t.iter().map(|&u| (p, (at(u).cos(), at(u).sin()))).collect(), true)
        }
        None => {
            let p = crate::effects::radial_center(start, size, (ox, oy));
            let q = crate::effects::radial_center(end, size, (ox, oy));
            let n = crate::along::facing(q.0 - p.0, q.1 - p.1);
            (t.iter().map(|&u| ((p.0 + u * (q.0 - p.0), p.1 + u * (q.1 - p.1)), n)).collect(), false)
        }
    }
}

/// D-416: Fractal's settings as `fractal` reads them (document 21). A view is its centre and one
/// pixel's size in units, 3 / (h 2^m); the counts are their whole parts.
pub(crate) struct FractalView {
    /// The Julia choices, the inverse ones, the "over Julia" ones.
    pub julia: bool,
    pub inverse: bool,
    pub over_julia: bool,
    /// The Equation's power, 2 to 6.
    pub power: u32,
    /// The Mandelbrot and Julia views: centre across, up, and a pixel's size.
    pub m: [f64; 3],
    pub j: [f64; 3],
    pub m_limit: u32,
    pub j_limit: u32,
    /// 0 Lightness Gradient, 1 Hue Wheel, 2 Black And White, 3 Solid Color.
    pub palette: u8,
    pub hue: f64,
    pub steps: i64,
    pub offset: i64,
    pub transparency: bool,
    pub edge_highlight: bool,
    pub brute: bool,
    pub factor: u32,
    pub overlay: bool,
}

impl FractalView {
    /// The view of `effect`'s settings over a drawing `h0` pixels tall; `None` for any other effect.
    pub(crate) fn of(effect: &crate::effects::Effect, h0: f64) -> Option<Self> {
        let crate::effects::Effect::Fractal {
            set_choice,
            equation,
            mandelbrot_center,
            mandelbrot_magnification,
            mandelbrot_escape_limit,
            julia_center,
            julia_magnification,
            julia_escape_limit,
            overlay,
            transparency,
            palette,
            hue,
            cycle_steps,
            cycle_offset,
            edge_highlight,
            oversample_method,
            oversample_factor,
        } = effect
        else {
            return None;
        };
        let view = |c: [f64; 2], m: f64| [c[0], c[1], 3.0 / (h0 * 2f64.powf(m))];
        Some(FractalView {
            julia: set_choice.starts_with("julia"),
            inverse: set_choice.contains("inverse"),
            over_julia: set_choice.contains("over_julia"),
            power: equation.get(1..).and_then(|p| p.parse().ok()).unwrap_or(2),
            m: view(*mandelbrot_center, *mandelbrot_magnification),
            j: view(*julia_center, *julia_magnification),
            m_limit: mandelbrot_escape_limit.floor() as u32,
            j_limit: julia_escape_limit.floor() as u32,
            palette: crate::effects::FRACTAL_PALETTES.iter().position(|p| p == palette).unwrap_or(0) as u8,
            hue: *hue,
            steps: (cycle_steps.floor() as i64).max(1),
            offset: cycle_offset.floor() as i64,
            transparency: transparency == "on",
            edge_highlight: edge_highlight == "on",
            brute: oversample_method == "brute_force",
            factor: (oversample_factor.floor() as u32).max(1),
            overlay: overlay == "on",
        })
    }

    /// The card's work on `pixels` pixels at most, in steps of z^2 + c: each pixel works its
    /// band and four neighbours' (five escapes), Factor by Factor more where it oversamples
    /// (counted as everywhere) and one more for Overlay, each up to the larger Escape Limit, a
    /// step of z^n costing n - 1 of z^2's. The card made 2.1e10 a second on B-295's
    /// measurement (RTX 4070 Ti SUPER), all in one dispatch, which Windows stops at 2 seconds.
    pub(crate) fn card_work(&self, pixels: usize) -> f64 {
        let samples = 5 + if self.factor > 1 { self.factor * self.factor } else { 0 } + self.overlay as u32;
        pixels as f64 * samples as f64 * self.m_limit.max(self.j_limit) as f64 * (self.power - 1) as f64
    }

    /// The bands a palette has, so its colours can be worked once.
    pub(crate) fn bands(&self) -> i64 {
        match self.palette {
            0 => 8 * self.steps,
            1 => self.steps,
            2 => 2,
            _ => 1,
        }
    }

    /// Step 4's band at the escape count `k`, -1 inside.
    fn band(&self, k: i64) -> i64 {
        match (k < 0, self.palette) {
            (true, _) => -1,
            (_, 3) => 0,
            _ => (k + self.offset) % self.bands(),
        }
    }

    /// Step 5's colour of band `b` (-1 inside), linear and premultiplied.
    pub(crate) fn colour(&self, b: i64) -> [f64; 4] {
        let clear = [0.0; 4];
        let e = match self.palette {
            3 if (b < 0) == self.transparency => return clear,
            3 => crate::grade::from_hls(self.hue.rem_euclid(360.0), 0.5, 1.0),
            _ if b < 0 => return if self.transparency { clear } else { [0.0, 0.0, 0.0, 1.0] },
            0 => {
                let s = self.steps;
                crate::grade::from_hls((self.hue + (45 * (b / s)) as f64).rem_euclid(360.0), (b % s + 1) as f64 / (s + 1) as f64, 1.0)
            }
            1 => crate::grade::from_hls((self.hue + (360 * b) as f64 / self.steps as f64).rem_euclid(360.0), 0.5, 1.0),
            _ => [b as f64; 3],
        };
        let [r, g, bl] = e.map(crate::grade::to_linear);
        [r, g, bl, 1.0]
    }

    /// Step 3's count at the drawing point `(x, y)`, -1 inside; `size` the drawing's.
    fn count(&self, x: f64, y: f64, size: (f64, f64)) -> i64 {
        let (view, limit) = if self.julia { (self.j, self.j_limit) } else { (self.m, self.m_limit) };
        let mut p = fractal_point(x, y, view, size);
        if self.inverse {
            match fractal_invert(p) {
                Some(q) => p = q,
                None => return 1,
            }
        }
        if self.julia {
            return fractal_escape(p, [self.m[0], self.m[1]], self.power, limit);
        }
        let z0 = if self.over_julia { [self.j[0], self.j[1]] } else { [0.0, 0.0] };
        fractal_escape(z0, p, self.power, limit)
    }

    /// Step 8's opposite set at `(x, y)`: whether it is inside.
    fn opposite_inside(&self, x: f64, y: f64, size: (f64, f64)) -> bool {
        if self.julia {
            fractal_escape([0.0, 0.0], fractal_point(x, y, self.m, size), self.power, self.m_limit) < 0
        } else {
            fractal_escape(fractal_point(x, y, self.j, size), [self.m[0], self.m[1]], self.power, self.j_limit) < 0
        }
    }
}

/// D-421: Audio Waveform's marks, standing as Audio Spectrum's do along the line or round the
/// mask (never round a point), from the least and greatest compose filled into `levels`;
/// `None` when it filled in none. Digital: a piece from the least to the greatest; Analog Lines
/// and Dots: the greatest where the top bit of mix(mix(seed) ^ j) is set, else the least, drawn
/// as Audio Spectrum's Side A.
pub(crate) fn waveform_marks(effect: &crate::effects::Effect, size: (usize, usize), origin: (usize, usize)) -> Option<Marks> {
    let crate::effects::Effect::AudioWaveform {
        start_point,
        end_point,
        maximum_height,
        thickness,
        softness,
        random_seed,
        inside_color,
        outside_color,
        display_options,
        composite,
        levels: Some(levels),
        outline,
        ..
    } = effect
    else {
        return None;
    };
    let shown = levels.len() / 2;
    let (feet, closed) = sound_feet(shown, outline.as_deref(), false, [*start_point, *end_point], size, origin);
    let colours = [inside_color, outside_color].map(|c| crate::effects::encoded(c).map(crate::grade::to_linear));
    let pieces = if display_options == "digital" {
        (0..shown)
            .map(|j| {
                let ((x, y), (nx, ny)) = feet[j];
                let at = |v: f64| (x + maximum_height * v * nx, y + maximum_height * v * ny);
                piece(at(levels[2 * j]), at(levels[2 * j + 1]), j, colours)
            })
            .collect()
    } else {
        let seed = crate::grade::mix(random_seed.floor() as u64);
        let heights: Vec<f64> = (0..shown).map(|j| maximum_height * levels[2 * j + (crate::grade::mix(seed ^ j as u64) >> 63) as usize]).collect();
        mark_pieces(&feet, closed, &heights, &vec![colours; shown], display_options, "side_a")
    };
    Some(Marks { pieces, r: thickness / 2.0, softness: *softness, how: if composite == "on" { 2 } else { 1 }, blend: false })
}

/// D-416: the drawing point `(x, y)` in the view `[cx, cy, u]`, the imaginary part up.
fn fractal_point(x: f64, y: f64, [cx, cy, u]: [f64; 3], (w0, h0): (f64, f64)) -> [f64; 2] {
    [cx + (x - w0 / 2.0) * u, cy - (y - h0 / 2.0) * u]
}

/// D-416: 1 / p, `None` at 0.
fn fractal_invert([re, im]: [f64; 2]) -> Option<[f64; 2]> {
    let d = re * re + im * im;
    (d != 0.0).then(|| [re / d, -im / d])
}

/// D-416: the first step from 1 to `limit` after which z^power + c leaves (-2, -2, 2, 2), or -1.
fn fractal_escape([mut zr, mut zi]: [f64; 2], [cr, ci]: [f64; 2], power: u32, limit: u32) -> i64 {
    for k in 1..=limit {
        let (mut qr, mut qi) = (zr, zi);
        for _ in 1..power {
            (qr, qi) = (qr * zr - qi * zi, qr * zi + qi * zr);
        }
        zr = qr + cr;
        zi = qi + ci;
        if zr.abs() > 2.0 || zi.abs() > 2.0 {
            return k as i64;
        }
    }
    -1
}

/// D-416: After Effects' Fractal over the whole buffer, its drawing's corner at `origin`. The
/// layer is replaced and never grows.
pub(crate) fn fractal(source: &mut WorkingBuffer, origin: (usize, usize), v: &FractalView) {
    let (w, h) = (source.width(), source.height());
    let size = ((w - 2 * origin.0) as f64, (h - 2 * origin.1) as f64);
    let (ox, oy) = (origin.0 as i64, origin.1 as i64);
    let palette: Vec<[f64; 4]> = (-1..v.bands()).map(|b| v.colour(b)).collect();
    let colour = |b: i64| palette[(b + 1) as usize];
    let at = |x: f64, y: f64| v.band(v.count(x, y, size));
    // Every pixel centre's band, with a ring a pixel wide outside the buffer for the neighbours.
    let gw = w + 2;
    let bands: Vec<i64> = (0..gw * (h + 2))
        .into_par_iter()
        .map(|i| at(((i % gw) as i64 - 1 - ox) as f64 + 0.5, ((i / gw) as i64 - 1 - oy) as f64 + 0.5))
        .collect();
    let f = v.factor;
    let (cx, cy) = ((size.0 as i64) / 2, (size.1 as i64) / 2);
    let arm = 2.max(size.1 as i64 / 20);
    let cross = |x: i64, y: i64| (y == cy && (x - cx).abs() <= arm) || (x == cx && (y - cy).abs() <= arm);
    source
        .data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let g = (i / w + 1) * gw + i % w + 1;
            let b = bands[g];
            let near = [bands[g - 1], bands[g + 1], bands[g - gw], bands[g + gw]];
            let (x0, y0) = ((i % w) as i64 - ox, (i / w) as i64 - oy);
            let mut p = if f > 1 && (v.brute || near.iter().any(|&n| n != b)) {
                let mut total = [0.0; 4];
                for bb in 0..f {
                    for aa in 0..f {
                        let s = colour(at(x0 as f64 + (aa as f64 + 0.5) / f as f64, y0 as f64 + (bb as f64 + 0.5) / f as f64));
                        for c in 0..4 {
                            total[c] += s[c];
                        }
                    }
                }
                total.map(|t| t / (f * f) as f64)
            } else if f == 1 && v.edge_highlight && (near[0] != b || near[2] != b) {
                [1.0; 4]
            } else {
                colour(b)
            };
            if v.overlay {
                if v.opposite_inside(x0 as f64 + 0.5, y0 as f64 + 0.5, size) {
                    p = p.map(|c| (c + 1.0) / 2.0);
                }
                if cross(x0, y0) {
                    p = [1.0; 4];
                } else if cross(x0 - 1, y0 - 1) {
                    p = [0.0, 0.0, 0.0, 1.0];
                }
            }
            for c in 0..4 {
                px[c] = p[c] as f32;
            }
        });
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
/// was. D-394's Split 2 opens each side by its own amount: `split[0]` the side with d < 0 (the
/// left walking from `a` to `b`), `split[1]` the other; equal amounts are Split's, byte for byte.
/// The settings are already valid.
pub(crate) fn split(source: &mut WorkingBuffer, a: (f64, f64), b: (f64, f64), split: [f64; 2]) {
    let w = source.width();
    let length = (b.0 - a.0).hypot(b.1 - a.1);
    if length == 0.0 || (split[0] <= 0.0 && split[1] <= 0.0) {
        return;
    }
    let (ux, uy) = ((b.0 - a.0) / length, (b.1 - a.1) / length);
    let still = source.clone();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as f64 + 0.5 - a.0, (i / w) as f64 + 0.5 - a.1);
        let along = x * ux + y * uy;
        let d = y * ux - x * uy;
        let t = along / length;
        let wave = if (0.0..=1.0).contains(&t) { (std::f64::consts::PI * t).sin() } else { 0.0 };
        let (g1, g2) = (split[0] / 2.0 * wave, split[1] / 2.0 * wave);
        // For d < 0 this is the mirror of d > 0, each step exact under a change of sign.
        let cover = 1.0 - ((d + 0.5).min(g2) - (d - 0.5).max(-g1)).max(0.0);
        let g = if d >= 0.0 { g2 } else { g1 };
        let m = d.abs();
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

/// D-458: CC Simple Wire Removal, this program's own reading of CycoreFX's (document 21). Along
/// `a` to `b`, length L, unit step u and n = (-u_y, u_x) across it, a pixel's centre t of the way
/// along and d to the side has the weight w = 1 within h = thickness / 2, falling to 0 over r =
/// thickness (1 - slope / 100) past it, and 0 past E = h + r or for t outside 0 to 1. It becomes
/// c (1 - w) + R w: R clear for "fade"; `other`'s pixel for "frame_offset"; for "displace", the
/// samples at F - E n and F + E n (F its foot on the wire) mixed by (d + E) / 2E, then
/// `numbers[2]` per cent of the sample at F + sign(d) (2E - |d|) n, mixed in; "displace_horizontal"
/// takes each sample along the pixel's own row instead, where the row has that d, and leaves a
/// wire lying flat alone. Samples are held inside the buffer's pixel centres (D-109's repeat).
/// `numbers`: thickness, slope, mirror blend. Thickness 0, `a` on `b`, or "frame_offset" with no
/// `other` leaves the buffer as it was. The settings are already valid.
pub(crate) fn wire_removal(source: &mut WorkingBuffer, a: (f64, f64), b: (f64, f64), style: &str, numbers: [f64; 3], other: Option<&WorkingBuffer>) {
    let [thickness, slope, mirror] = numbers;
    let length = (b.0 - a.0).hypot(b.1 - a.1);
    if length == 0.0 || thickness <= 0.0 || (style == "frame_offset" && other.is_none()) {
        return;
    }
    let (ux, uy) = ((b.0 - a.0) / length, (b.1 - a.1) / length);
    let (nx, ny) = (-uy, ux);
    if style == "displace_horizontal" && nx == 0.0 {
        return;
    }
    let h = thickness / 2.0;
    let r = thickness * (1.0 - slope / 100.0);
    let e = h + r;
    let m = mirror / 100.0;
    let (w, ht) = (source.width(), source.height());
    let still = source.clone();
    let held = |x: f64, y: f64| sample_bilinear(&still, x.clamp(0.5, w as f64 - 0.5), y.clamp(0.5, ht as f64 - 0.5));
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (cx, cy) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
        let (x, y) = (cx - a.0, cy - a.1);
        let along = x * ux + y * uy;
        let d = x * nx + y * ny;
        let t = along / length;
        let q = d.abs();
        if !(0.0..=1.0).contains(&t) || q >= e {
            return;
        }
        let weight = if q <= h { 1.0 } else { 1.0 - (q - h) / r };
        let rr: [f64; 4] = match style {
            "fade" => [0.0; 4],
            "frame_offset" => {
                let o = other.expect("frame_offset with no picture returned above");
                let p = if i % w < o.width() && i / w < o.height() { o.pixel(i % w, i / w) } else { [0.0; 4] };
                p.map(f64::from)
            }
            _ => {
                let at = |k: f64| {
                    if style == "displace" {
                        held(a.0 + along * ux + k * nx, a.1 + along * uy + k * ny)
                    } else {
                        held(cx + (k - d) / nx, cy)
                    }
                };
                let s = if d >= 0.0 { 1.0 } else { -1.0 };
                let (lo, hi, mir) = (at(-e), at(e), at(s * (2.0 * e - q)));
                let f = (d + e) / (2.0 * e);
                std::array::from_fn(|ch| {
                    let inter = lo[ch] as f64 + (hi[ch] as f64 - lo[ch] as f64) * f;
                    inter + (mir[ch] as f64 - inter) * m
                })
            }
        };
        for ch in 0..4 {
            px[ch] = (px[ch] as f64 * (1.0 - weight) + rr[ch] * weight) as f32;
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
            let (c, q) = line_profile(d, r, softness);
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
/// (1 - t / (L + 1))^2 over their sum, tinted by `color` (linear) at `intensity` and added over
/// the drawing as Light Rays' is; D-546: each arm's light first tinted by its own of `tints`
/// (`effects::Glint::tints`, all 1 at its start). The layer grows by the length on every side,
/// returned; length 0 or intensity 0 changes nothing. The settings are already valid.
#[allow(clippy::too_many_arguments)]
pub(crate) fn cross_glare(
    source: &mut WorkingBuffer,
    threshold: f64,
    length: f64,
    points: f64,
    angle: f64,
    intensity: f64,
    color: [f64; 3],
    tints: &[[f64; 4]],
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
    let c = color;
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
            for (&(vx, vy), tint) in arms.iter().zip(tints).filter(|_| any) {
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
                        g[ch] += k * (s[ch] as f64 * tint[ch]);
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


/// D-582: CC Rainfall, rain with depth drawn by the effect itself (document 21). Each drop i
/// takes five numbers by Noise's hash of the seed: its depth in the volume, so its scale F / (F +
/// depth) with F the drawing's width; its tilt within the spread; its own wind; where it is in
/// its fall; and where across it starts. It falls `speed` (and drifts with the wind) pixels a
/// second times its scale, from wholly above the picture to the ground and round again, a
/// streak of half a frame's travel, `size` times its scale wide, at `opacity` times its scale;
/// the drops are laid far to near in `color` (linear) onto clear, and that rain is laid over the
/// layer, or alone, as Composite With Original and the transfer mode say. The numbers are
/// `[drops, size, scene_depth, speed, wind, wind_variation, spread, opacity, ground_level,
/// random_seed]`, already held; drops and the seed count by their whole parts. `clock` is the
/// composition frame and the frames a second. `origin` is the growth of an earlier effect, so
/// the drops stay in the drawing's own space; it grows nothing. The settings are already valid.
// ponytail: processor only, and every row tries every drop; a card pass, or the drops sorted
// into rows first, if many drops on a large picture prove slow.
#[allow(clippy::too_many_arguments)]
pub(crate) fn cc_rainfall(
    source: &mut WorkingBuffer,
    color: [f64; 3],
    [drops, size, scene_depth, speed, wind, wind_variation, spread, opacity, ground_level, random_seed]: [f64; 10],
    lighten: bool,
    with_original: bool,
    [frame, rate]: [f64; 2],
    origin: (usize, usize),
) {
    let (bw, bh) = (source.width(), source.height());
    let (w0, h0) = (bw as f64 - 2.0 * origin.0 as f64, bh as f64 - 2.0 * origin.1 as f64);
    if bw == 0 || w0 <= 0.0 || rate <= 0.0 {
        return;
    }
    let g = ground_level / 100.0 * h0;
    let tau = frame / rate;
    let base = crate::grade::mix(random_seed.floor() as u64);
    // [depth, index, head x, head y, tail x, tail y, width, strength], far first.
    let mut list: Vec<[f64; 8]> = (0..drops.floor() as i64)
        .filter_map(|i| {
            let u = |k| (crate::grade::unit(base, i, 0, 0, k) + 1.0) / 2.0;
            let d = scene_depth * u(0);
            let s = w0 / (w0 + d);
            let theta = (spread * (u(1) - 0.5)).to_radians();
            let own_wind = wind * (1.0 + wind_variation / 100.0 * (2.0 * u(2) - 1.0));
            let (vx, vy) = (s * (own_wind + speed * theta.tan()), s * speed);
            let (lx, ly) = (vx / (2.0 * rate), vy / (2.0 * rate));
            let ws = size * s;
            let top = -(ly + (ws / 2.0 + 0.5).max(1.0));
            let r = g - top;
            if ws == 0.0 || r <= 0.0 {
                return None;
            }
            let q = u(3) + vy * tau / r;
            let hy = top + (q - q.floor()) * r;
            let x0 = w0 * (1.5 * u(4) - 0.25);
            let hx = if vy > 0.0 { x0 + (vx / vy) * ((q - q.floor()) * r - r / 2.0) } else { x0 };
            Some([d, i as f64, hx, hy, hx - lx, hy - ly, ws, opacity / 100.0 * s])
        })
        .collect();
    if list.is_empty() && with_original {
        return;
    }
    list.sort_by(|a, b| b[0].total_cmp(&a[0]).then(a[1].total_cmp(&b[1])));
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    source
        .data_mut()
        .par_chunks_exact_mut(4 * bw)
        .enumerate()
        .for_each(|(row, line)| {
            let y = row as f64 - oy + 0.5;
            let mut rain = vec![[0.0f64; 4]; bw];
            if y <= g {
                for &[_, _, hx, hy, tx, ty, ws, strength] in &list {
                    let reach = (ws / 2.0 + 0.5).max(1.0) + 1.0;
                    if y < hy.min(ty) - reach || y > hy.max(ty) + reach {
                        continue;
                    }
                    let lo = (hx.min(tx) - reach + ox).floor().max(0.0) as usize;
                    let hi = ((hx.max(tx) + reach + ox).ceil().max(0.0) as usize).min(bw);
                    let (dx, dy) = (hx - tx, hy - ty);
                    let ll = dx * dx + dy * dy;
                    for (bx, r) in rain.iter_mut().enumerate().take(hi).skip(lo) {
                        let x = bx as f64 - ox + 0.5;
                        let t = if ll == 0.0 { 0.0 } else { (((x - tx) * dx + (y - ty) * dy) / ll).clamp(0.0, 1.0) };
                        let delta = (x - (tx + t * dx)).hypot(y - (ty + t * dy));
                        let cover = if ws >= 1.0 { (ws / 2.0 + 0.5 - delta).clamp(0.0, 1.0) } else { ws * (1.0 - delta).clamp(0.0, 1.0) };
                        let a = strength * cover;
                        if a > 0.0 {
                            *r = [color[0] * a + r[0] * (1.0 - a), color[1] * a + r[1] * (1.0 - a), color[2] * a + r[2] * (1.0 - a), a + r[3] * (1.0 - a)];
                        }
                    }
                }
            }
            for (px, r) in line.chunks_exact_mut(4).zip(&rain) {
                let mut out = *r;
                if with_original {
                    let b = [px[0] as f64, px[1] as f64, px[2] as f64, px[3] as f64];
                    out = [0, 1, 2, 3].map(|c| r[c] + b[c] * (1.0 - r[3]));
                    if lighten {
                        for c in 0..3 {
                            out[c] = out[c].max(b[c]);
                        }
                    }
                }
                for c in 0..4 {
                    px[c] = out[c] as f32;
                }
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
/// D-412: with `spectrum`, the three colours' shares of the push and the samples' count, each
/// sample read at its own share and each colour the weighted mean of the samples
/// ([`chromatic_samples`]), the covering the largest; the growth then the largest share's.
pub(crate) fn displacement_map(
    source: &mut WorkingBuffer,
    map: &WorkingBuffer,
    origin: (usize, usize),
    words: [&str; 2],
    most: [f64; 2],
    (wrap, expand): (bool, bool),
    spectrum: Option<([f64; 3], usize)>,
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
    let samples = spectrum.map(|(a, n)| chromatic_samples(a, n.max(2)));
    let top = spectrum.map_or(1.0, |(a, _)| a.iter().fold(0.0f64, |t, v| t.max(v.abs())));
    let g = if expand && !wrap { (most[0].abs().max(most[1].abs()) * top).ceil() as usize } else { 0 };
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
            let read = |sx: f64, sy: f64| if wrap { wrapped(&from, (w, h), sx, sy) } else { sample_bilinear(&from, sx, sy) };
            let dx = (2.0 * value(words[0], m) - 1.0) * most[0];
            let dy = (2.0 * value(words[1], m) - 1.0) * most[1];
            let (cx, cy) = (x as f64 + 0.5 - g as f64, y as f64 + 0.5 - g as f64);
            match &samples {
                None => o.copy_from_slice(&read(x as f64 + 0.5 + dx - g as f64, y as f64 + 0.5 + dy - g as f64)),
                Some(samples) => {
                    let (mut sum, mut total, mut cover) = ([0.0f64; 3], [0.0f64; 3], 0.0f32);
                    for (a, wt) in samples {
                        let p = read(cx + a * dx, cy + a * dy);
                        for c in 0..3 {
                            sum[c] += wt[c] * p[c] as f64;
                            total[c] += wt[c];
                        }
                        cover = cover.max(p[3]);
                    }
                    for c in 0..3 {
                        o[c] = (sum[c] / total[c]) as f32;
                    }
                    o[3] = cover;
                }
            }
        }
    });
    *source = out;
    g
}

/// D-412: the spectrum's `n` samples from red (t = 0) through green (1/2) to blue (1), each its
/// share of the push, straight between the three `amounts`, and its weights toward red, green
/// and blue: max(0, 1 - 2t), 1 - |2t - 1| and max(0, 2t - 1). The card reads the same.
pub(crate) fn chromatic_samples(a: [f64; 3], n: usize) -> Vec<(f64, [f64; 3])> {
    (0..n)
        .map(|i| {
            let t = i as f64 / (n - 1) as f64;
            let share = if t <= 0.5 { a[0] + (a[1] - a[0]) * 2.0 * t } else { a[1] + (a[2] - a[1]) * (2.0 * t - 1.0) };
            (share, [(1.0 - 2.0 * t).max(0.0), 1.0 - (2.0 * t - 1.0).abs(), (2.0 * t - 1.0).max(0.0)])
        })
        .collect()
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

/// D-512: Channel Wipe's reading of the premultiplied pixel `p` by `property`: the encoded
/// channels of its colour over black, its covering, its picture luma, or the HSL of the encoded
/// colour (hue over 360).
fn wipe_reading(property: &str, p: [f32; 4]) -> f64 {
    match property {
        "luminance" => picture_luma(p),
        "alpha" => p[3] as f64,
        _ => {
            let e = [0, 1, 2].map(|c| crate::grade::to_srgb((p[c] as f64).clamp(0.0, 1.0)));
            let [h, s, l] = crate::grade::to_hsl(e);
            match property {
                "red" => e[0],
                "green" => e[1],
                "blue" => e[2],
                "hue" => h / 360.0,
                "saturation" => s,
                _ => l,
            }
        }
    }
}

/// D-194: Gradient Wipe. Each pixel is kept by how far the picture luma of the map under it lies
/// above the edge `completion` has reached, over a band `softness` wide; the map lies on the
/// layer's own picture, which sits at `origin` in `source`, and reads black outside it.
/// D-512 (Channel Wipe), the same engine: `map` `None` for the input itself lying on itself;
/// the map read by `reading.0` ([`wipe_reading`]; Gradient Wipe's is `luminance`), and with
/// `reading.1` above 0 the readings blurred at sigma `reading.1 / 2`, the outer pixels repeated.
pub(crate) fn gradient_wipe(
    source: &mut WorkingBuffer,
    map: Option<&WorkingBuffer>,
    origin: (usize, usize),
    completion: f64,
    softness: f64,
    (property, blur): (&str, f64),
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
    let (w, h) = (source.width(), source.height());
    let itself = map.is_none().then(|| source.clone());
    let (map, origin) = match map {
        Some(m) => (m, origin),
        None => (itself.as_ref().expect("cloned when there is no map"), (0, 0)),
    };
    let (mw, mh) = (map.width(), map.height());
    let read = |i: usize| {
        let (x, y) = ((i % w).wrapping_sub(origin.0), (i / w).wrapping_sub(origin.1));
        if x < mw && y < mh { wipe_reading(property, map.pixel(x, y)) } else { 0.0 }
    };
    let field = (blur > 0.0).then(|| {
        let mut f = WorkingBuffer::transparent(w, h);
        f.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| px[3] = read(i) as f32);
        crate::effects::held_blur_axes(&mut f, &crate::effects::gaussian_weights(blur / 2.0), (true, true));
        f
    });
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let v = match &field {
            Some(f) => f.data()[i * 4 + 3] as f64,
            None => read(i),
        };
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

/// D-403: Aerial Haze. Each pixel moved toward `haze`, a linear colour, by `amount` (0 to 1) in
/// premultiplied light, times the picture luma of the matte under it when there is one; the
/// matte lies on the layer's own picture, which sits at its origin in `source`, and reads black
/// outside it. The covering never changes.
pub(crate) fn aerial_haze(source: &mut WorkingBuffer, matte: Option<(&WorkingBuffer, (usize, usize))>, haze: [f64; 3], amount: f64) {
    let w = source.width();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let v = match matte {
            None => 1.0,
            Some((map, origin)) => {
                let (x, y) = ((i % w).wrapping_sub(origin.0), (i / w).wrapping_sub(origin.1));
                if x < map.width() && y < map.height() { picture_luma(map.pixel(x, y)) } else { 0.0 }
            }
        };
        let (k, a) = (amount * v, px[3] as f64);
        for c in 0..3 {
            let q = px[c] as f64;
            px[c] = (q + k * (haze[c] * a - q)) as f32;
        }
    });
}

/// D-524: Fog 3D's shares at each pixel of the drawing, from the depth z compose read and, with
/// a gradient layer named, the picture luma L of its map (both lie on the drawing at the same
/// corner, the map transparent outside itself): t = clamp((z - start) / (end - start), 0, 1), a
/// cut at start where the two are equal; f = t at density 0, else (1 - e^(-d t)) / (1 - e^(-d))
/// with d = density / 10; g = 1 - c + c L (1 without a gradient), c the contribution; the fog
/// on the pixel f g o in the first number and behind it g o (0 with Foggy Background off) in
/// the second, o the opacity. None where the layer is left as it is: no depth read, or a
/// gradient named and not read.
pub(crate) fn fog_shares(e: &crate::effects::Effect) -> Option<WorkingBuffer> {
    let crate::effects::Effect::Fog3d {
        fog_start_depth: start,
        fog_end_depth: end,
        fog_opacity,
        scattering_density,
        foggy_background,
        gradient_layer,
        layer_contribution,
        map,
        channels: Some(depth),
        ..
    } = e
    else {
        return None;
    };
    let gradient = match (gradient_layer.as_str(), map) {
        (Some(""), _) => None,
        (_, Some(m)) => Some(m.0.as_ref()),
        _ => return None,
    };
    let (o, c, d) = (fog_opacity / 100.0, layer_contribution / 100.0, scattering_density / 10.0);
    let behind = foggy_background == "on";
    let (w, h) = (depth.0.width(), depth.0.height());
    let mut out = WorkingBuffer::transparent(w, h);
    out.data_mut().par_chunks_exact_mut(4).zip(depth.0.data().par_chunks_exact(4)).enumerate().for_each(|(i, (px, z))| {
        let z = z[0] as f64;
        let t = if end == start {
            if z >= *start { 1.0 } else { 0.0 }
        } else {
            ((z - start) / (end - start)).clamp(0.0, 1.0)
        };
        let f = if d == 0.0 { t } else { (1.0 - (-d * t).exp()) / (1.0 - (-d).exp()) };
        let g = gradient.map_or(1.0, |m| {
            let (x, y) = (i % w, i / w);
            1.0 - c + c * if x < m.width() && y < m.height() { picture_luma(m.pixel(x, y)) } else { 0.0 }
        });
        px[0] = (f * g * o) as f32;
        px[1] = if behind { (g * o) as f32 } else { 0.0 };
    });
    Some(out)
}

/// D-524: Fog 3D laid on with its shares (F, B) lying at `origin`: each colour moves toward the
/// fog colour `fog` (linear) by F as Aerial Haze moves it, and what does not cover the pixel lets
/// B of the fog behind it show, (rgb + F (fog a - rgb) + (1 - a) B fog, a + (1 - a) B). Outside
/// the shares nothing changes.
pub(crate) fn fog_3d(source: &mut WorkingBuffer, shares: &WorkingBuffer, origin: (usize, usize), fog: [f64; 3]) {
    let w = source.width();
    let s = shares.data();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let Some(j) = pass_at(shares, i, w, origin) else {
            return;
        };
        let (f, b, a) = (s[j] as f64, s[j + 1] as f64, px[3] as f64);
        for c in 0..3 {
            let q = px[c] as f64;
            px[c] = (q + f * (fog[c] * a - q) + (1.0 - a) * b * fog[c]) as f32;
        }
        px[3] = (a + (1.0 - a) * b) as f32;
    });
}

/// D-501: Difference Matte. Each pixel compared with the plate `map` under it (the plate lies
/// on the layer's own picture, at `origin` in `source`, transparent outside it), both blurred
/// first by a Gaussian at `sigma` when above 0: the largest difference of the straight encoded
/// colour and the alpha, in per cent, `d`. The share kept is `clamp((d - tolerance) / softness,
/// 0, 1)`, or at softness 0 nothing where `d <= tolerance` and all elsewhere. `matte` draws
/// the share as a grey under the layer's covering; else every channel is times the share.
pub(crate) fn difference_matte(
    source: &mut WorkingBuffer,
    map: &WorkingBuffer,
    origin: (usize, usize),
    tolerance: f64,
    softness: f64,
    sigma: f64,
    matte: bool,
) {
    let straight = |p: [f32; 4]| -> [f64; 4] {
        let a = p[3] as f64;
        if a <= 0.0 {
            return [0.0; 4];
        }
        let [r, g, b] = [0, 1, 2].map(|c| crate::grade::to_srgb((p[c] as f64 / a).clamp(0.0, 1.0)));
        [r, g, b, a.min(1.0)]
    };
    let blurred = (sigma > 0.0).then(|| {
        let (mut s, mut m) = (source.clone(), map.clone());
        blur(&mut s, sigma);
        blur(&mut m, sigma);
        (s, m)
    });
    let (s, m, r) = match &blurred {
        Some((s, m)) => (s, m, kernel_radius(sigma) as isize),
        None => (&*source, map, 0),
    };
    let w = source.width();
    let (ox, oy) = (origin.0 as isize, origin.1 as isize);
    let shares: Vec<f64> = (0..source.width() * source.height())
        .into_par_iter()
        .map(|i| {
            let (x, y) = ((i % w) as isize, (i / w) as isize);
            let (a, b) = (straight(at(s, x + r, y + r)), straight(at(m, x - ox + r, y - oy + r)));
            let d = 100.0 * (0..4).map(|c| (a[c] - b[c]).abs()).fold(0.0, f64::max);
            if softness > 0.0 {
                ((d - tolerance) / softness).clamp(0.0, 1.0)
            } else if d <= tolerance {
                0.0
            } else {
                1.0
            }
        })
        .collect();
    source.data_mut().par_chunks_exact_mut(4).zip(shares).for_each(|(px, k)| {
        if matte {
            let g = (crate::grade::to_linear(k) * px[3] as f64) as f32;
            px[..3].fill(g);
        } else {
            px.iter_mut().for_each(|v| *v = (*v as f64 * k) as f32);
        }
    });
}

/// D-521: Time Difference. Each pixel against the target `map` under it (the target lies on the
/// layer's own picture, at `origin` in `source`, transparent outside it), both as straight
/// encoded colour and alpha: the difference `d`, all four numbers, times the gain
/// `1 + contrast / 10`, taken whole (`absolute`, so no difference is black) or about mid grey,
/// and held within 0 and 1: `e`. The covering is chosen by `alpha`, an index into
/// [`crate::effects::TDIFF_ALPHAS`]; the colour is `e`'s (white for the last), premultiplied.
pub(crate) fn time_difference(source: &mut WorkingBuffer, map: &WorkingBuffer, origin: (usize, usize), contrast: f64, absolute: bool, alpha: usize) {
    let straight = |p: [f32; 4]| -> [f64; 4] {
        let a = p[3] as f64;
        if a <= 0.0 {
            return [0.0; 4];
        }
        let [r, g, b] = [0, 1, 2].map(|c| crate::grade::to_srgb((p[c] as f64 / a).clamp(0.0, 1.0)));
        [r, g, b, a.min(1.0)]
    };
    let gain = 1.0 + contrast / 10.0;
    let w = source.width();
    let (ox, oy) = (origin.0 as isize, origin.1 as isize);
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as isize, (i / w) as isize);
        let (s, m) = (straight([px[0], px[1], px[2], px[3]]), straight(at(map, x - ox, y - oy)));
        let e: [f64; 4] = std::array::from_fn(|c| {
            let d = s[c] - m[c];
            if absolute { (gain * d.abs()).clamp(0.0, 1.0) } else { (0.5 + gain * d / 2.0).clamp(0.0, 1.0) }
        });
        let (hi, lo) = (e[0].max(e[1]).max(e[2]), e[0].min(e[1]).min(e[2]));
        let a = match alpha {
            0 => s[3],
            1 => m[3],
            2 => (s[3] + m[3]) / 2.0,
            3 => s[3].max(m[3]),
            4 => 1.0,
            5 => (hi + lo) / 2.0,
            6 => hi,
            _ => e[3],
        };
        for c in 0..3 {
            px[c] = (if alpha == 8 { a } else { crate::grade::to_linear(e[c]) * a }) as f32;
        }
        px[3] = a as f32;
    });
}

/// D-506: 3D Glasses for one frame: each view's map, `None` for the layer itself, already
/// swapped; the two distances; the view, an index into [`crate::effects::GLASSES_VIEWS`]; the
/// balance.
pub(crate) struct Glasses<'a> {
    pub maps: [Option<&'a WorkingBuffer>; 2],
    pub convergence: f64,
    pub alignment: f64,
    pub percent: bool,
    pub view: usize,
    pub balance: f64,
}

/// D-506: [`Glasses`] from a 3D Glasses whose settings are valid, or `None` when a view is
/// named but was not read, which leaves the layer as it is.
pub(crate) fn glasses_settings(e: &crate::effects::Effect) -> Option<Glasses<'_>> {
    let crate::effects::Effect::ThreeDGlasses {
        left_view,
        right_view,
        scene_convergence,
        vertical_alignment,
        units,
        swap_left_right,
        three_d_view,
        balance,
        left_map,
        right_map,
    } = e
    else {
        return None;
    };
    fn view<'a>(name: &serde_json::Value, map: &'a Option<crate::layer_map::Map>) -> Option<Option<&'a WorkingBuffer>> {
        match map {
            Some(m) => Some(Some(&*m.0)),
            None => (name.as_str() == Some("")).then_some(None),
        }
    }
    let mut maps = [view(left_view, left_map)?, view(right_view, right_map)?];
    if swap_left_right == "on" {
        maps.swap(0, 1);
    }
    Some(Glasses {
        maps,
        convergence: *scene_convergence,
        alignment: *vertical_alignment,
        percent: units == "percent",
        view: crate::effects::GLASSES_VIEWS.iter().position(|v| v == three_d_view)?,
        balance: *balance,
    })
}

/// D-518: Warp Blend for one frame: the reveal's map, `None` for nothing; the completion; the
/// reactor and the direction as places in their lists; the smoothness, the pull and the span.
pub(crate) struct Warp<'a> {
    pub reveal: Option<&'a WorkingBuffer>,
    pub completion: f64,
    pub reactor: usize,
    pub smoothness: f64,
    pub amount: f64,
    pub direction: usize,
    pub span: f64,
}

/// D-518: [`Warp`] from a Warp Blend whose settings are valid, or `None` when a layer is named but
/// was not read, which leaves the layer as it is.
pub(crate) fn warp_settings(e: &crate::effects::Effect) -> Option<Warp<'_>> {
    let crate::effects::Effect::WarpBlend { completion, layer_to_reveal, reactor, smoothness, warp_amount, warp_direction, blend_span, map } = e
    else {
        return None;
    };
    let reveal = match map {
        Some(m) => Some(&*m.0),
        None if layer_to_reveal.as_str() == Some("") => None,
        None => return None,
    };
    Some(Warp {
        reveal,
        completion: *completion,
        reactor: crate::effects::WARP_BLEND_REACTORS.iter().position(|r| r == reactor)?,
        smoothness: *smoothness,
        amount: *warp_amount,
        direction: crate::effects::WARP_BLEND_DIRECTIONS.iter().position(|d| d == warp_direction)?,
        span: *blend_span,
    })
}

/// D-518: Warp Blend's share of the reveal at completion share `c` with the blend span `span`
/// per cent: clamp((c - (1/2 - s/2)) / s, 0, 1), s the span as a share; with s 0, 1 from the
/// middle on.
pub(crate) fn blend_share(c: f64, span: f64) -> f64 {
    let s = span / 100.0;
    if s == 0.0 {
        return if c >= 0.5 { 1.0 } else { 0.0 };
    }
    ((c - (0.5 - s / 2.0)) / s).clamp(0.0, 1.0)
}

/// D-518: Warp Blend's reactor at a pixel of the layer `a` and of the reveal `b`, by the place
/// `kind` in [`crate::effects::WARP_BLEND_REACTORS`], from the luminance phase (D-316) and the
/// red, green and blue phases, each times its covering.
fn reactor_of(kind: usize, a: &[f32; 4], b: &[f32; 4]) -> f64 {
    let ph = |get: &str, q: &[f32; 4]| crate::grade::phase_of(get, q) * q[3] as f64;
    let (la, lb) = (ph("luminance", a), ph("luminance", b));
    match kind {
        0 => (la + lb) / 2.0,
        1 => ((la - 0.5).abs() - (lb - 0.5).abs()).abs(),
        2 => (la - lb).abs(),
        _ => ["red", "green", "blue"].iter().map(|c| (ph(c, a) - ph(c, b)).abs()).fold(0.0, f64::max),
    }
}

/// D-518: Warp Blend, after CycoreFX's CC WarpoMatic, the rule ours
/// (`tools/warp_blend_reference.py`). The reactor of the layer and the reveal (lying on the
/// drawing at its corner, clear outside it or with none), blurred at sigma `smoothness / 2` with
/// the outer pixels repeated; the pull D its slope by central differences, the neighbours held
/// inside, times `amount` and `1.25 max(smoothness, 1)`, turned a quarter when twisting. With c
/// the completion over 100 and m [`blend_share`], each pixel is (1 - m) A(p + c D) + m B(p -+
/// (1 - c) D), minus for joint and twisting, plus for opposing, both by document 21's bilinear.
/// At completion 0 nothing changes. The layer never grows.
pub(crate) fn warp_blend(source: &mut WorkingBuffer, g: &Warp, origin: (usize, usize)) {
    if g.completion <= 0.0 {
        return;
    }
    let (w, h) = (source.width(), source.height());
    let mut bump = WorkingBuffer::transparent(w, h);
    let from = source.data();
    bump.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let a = [from[i * 4], from[i * 4 + 1], from[i * 4 + 2], from[i * 4 + 3]];
        let b = g.reveal.map_or([0.0; 4], |m| on_drawing(m, i, w, origin));
        px[3] = reactor_of(g.reactor, &a, &b) as f32;
    });
    if g.smoothness > 0.0 {
        crate::effects::held_blur_axes(&mut bump, &crate::effects::gaussian_weights(g.smoothness / 2.0), (true, true));
    }
    let data = bump.data();
    let at = |x: isize, y: isize| {
        let (x, y) = (x.clamp(0, w as isize - 1) as usize, y.clamp(0, h as isize - 1) as usize);
        data[(y * w + x) * 4 + 3] as f64
    };
    let gain = 1.25 * g.smoothness.max(1.0);
    let c = g.completion / 100.0;
    let m = blend_share(c, g.span);
    let t = if g.direction == 1 { 1.0 } else { -1.0 };
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    let still = source.clone();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as isize, (i / w) as isize);
        let mut dx = g.amount * (at(x + 1, y) - at(x - 1, y)) / 2.0 * gain;
        let mut dy = g.amount * (at(x, y + 1) - at(x, y - 1)) / 2.0 * gain;
        if g.direction == 2 {
            (dx, dy) = (-dy, dx);
        }
        let (sx, sy) = (x as f64 + 0.5, y as f64 + 0.5);
        let a = sample_bilinear(&still, sx + c * dx, sy + c * dy);
        let b = match g.reveal {
            Some(r) if m > 0.0 => sample_bilinear(r, sx + t * (1.0 - c) * dx - ox, sy + t * (1.0 - c) * dy - oy),
            _ => [0.0; 4],
        };
        for k in 0..4 {
            px[k] = ((1.0 - m) * a[k] as f64 + m * b[k] as f64) as f32;
        }
    });
}

/// D-517: Twister's back for one frame: `Some(None)` with none named, `Some(Some(map))` when it
/// was read, and `None` when a layer is named but was not read, which leaves the layer as it is.
pub(crate) fn twist_back(e: &crate::effects::Effect) -> Option<Option<&WorkingBuffer>> {
    let crate::effects::Effect::Twister { backside, map, .. } = e else {
        return None;
    };
    match map {
        Some(m) => Some(Some(&*m.0)),
        None => (backside.as_str() == Some("")).then_some(None),
    }
}

/// D-517: Twister's numbers in the drawing's own `w0` by `h0` pixels, shared with the card: the
/// centre (two), the axis's cosine and sine, the largest distance of a corner along the axis,
/// twice the completion as a share, 1 with shading, and 1 with a back.
pub(crate) fn twist_numbers(e: &crate::effects::Effect, back: bool, w0: f64, h0: f64) -> [f64; 8] {
    let crate::effects::Effect::Twister { completion, shading, center, axis, .. } = e else {
        unreachable!("Twister's numbers of another effect")
    };
    let (sn, cs) = axis.to_radians().sin_cos();
    let (cx, cy) = (center[0] / 100.0 * w0, center[1] / 100.0 * h0);
    let far = [(0.0, 0.0), (w0, 0.0), (0.0, h0), (w0, h0)].iter().map(|&(x, y)| ((x - cx) * cs + (y - cy) * sn).abs()).fold(0.0, f64::max);
    [cx, cy, cs, sn, far, 2.0 * (completion / 100.0), (shading == "on") as u8 as f64, back as u8 as f64]
}

/// D-517: at the own point (x, y), cos theta of the strip there and the point it reads, in the
/// drawing's own space (document 21).
fn twist_from(k: &[f64; 8], x: f64, y: f64) -> (f64, f64, f64) {
    let (dx, dy) = (x - k[0], y - k[1]);
    let (u, v) = (dx * k[2] + dy * k[3], -dx * k[3] + dy * k[2]);
    let t = (u + k[4]) / (2.0 * k[4]);
    let c = (std::f64::consts::PI * (k[5] - t).clamp(0.0, 1.0)).cos();
    let b = v / c.abs();
    (c, k[0] + u * k[2] - b * k[3], k[1] + u * k[3] + b * k[2])
}

/// D-517: after CycoreFX's CC Twister, the rule ours (document 21); `back` the map lying on the
/// drawing at its corner, `(ox, oy)` the drawing's corner in the buffer. The front where the
/// strip faces the viewer, the back where it is turned over, clear edge on; shading multiplies
/// the colour by |cos theta|. Never grows; at completion 0 returns at once (compose leaves it off
/// the card then).
pub(crate) fn twister(source: &mut WorkingBuffer, e: &crate::effects::Effect, back: Option<&WorkingBuffer>, (ox, oy): (usize, usize)) {
    if matches!(e, crate::effects::Effect::Twister { completion, .. } if *completion == 0.0) {
        return;
    }
    let (w, h) = (source.width(), source.height());
    let k = twist_numbers(e, back.is_some(), (w - 2 * ox) as f64, (h - 2 * oy) as f64);
    let still = source.clone();
    let (fx, fy) = (ox as f64, oy as f64);
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (c, x, y) = twist_from(&k, (i % w) as f64 - fx + 0.5, (i / w) as f64 - fy + 0.5);
        let mut p = match back {
            _ if c > 0.0 => sample_bilinear(&still, x + fx, y + fy),
            Some(m) if c < 0.0 => sample_bilinear(m, x, y),
            _ => [0.0; 4],
        };
        if k[6] == 1.0 {
            for ch in &mut p[..3] {
                *ch = (*ch as f64 * c.abs()) as f32;
            }
        }
        px.copy_from_slice(&p);
    });
}

/// D-516: Stretch Wipe's numbers in the drawing's own `w0` by `h0` pixels, shared with the card:
/// the centre (two), the direction's cosine and sine, and the scale as a factor.
pub(crate) fn stretch_numbers(e: &crate::effects::Effect, w0: f64, h0: f64) -> [f64; 5] {
    let crate::effects::Effect::StretchWipe { scale, center, direction } = e else {
        unreachable!("Stretch Wipe's numbers of another effect")
    };
    let (sn, cs) = direction.to_radians().sin_cos();
    [center[0] / 100.0 * w0, center[1] / 100.0 * h0, cs, sn, scale / 100.0]
}

/// D-516: where the own point (x, y) is read from past the line through the centre, in the
/// drawing's own space, or None where it is behind the line and stays itself (document 21).
fn stretch_from(k: &[f64; 5], x: f64, y: f64) -> Option<(f64, f64)> {
    let (dx, dy) = (x - k[0], y - k[1]);
    let (u, v) = (dx * k[2] + dy * k[3], -dx * k[3] + dy * k[2]);
    if u <= 0.0 {
        return None;
    }
    let a = u / k[4];
    Some((k[0] + a * k[2] - v * k[3], k[1] + a * k[3] + v * k[2]))
}

/// D-516: after CycoreFX's CC Scale Wipe, the rule ours (document 21); `(ox, oy)` the drawing's
/// corner in the buffer. Never grows; at scale 100 returns at once (compose leaves it off the
/// card then).
pub(crate) fn stretch_wipe(source: &mut WorkingBuffer, e: &crate::effects::Effect, (ox, oy): (usize, usize)) {
    if matches!(e, crate::effects::Effect::StretchWipe { scale, .. } if *scale == 100.0) {
        return;
    }
    let (w, h) = (source.width(), source.height());
    let k = stretch_numbers(e, (w - 2 * ox) as f64, (h - 2 * oy) as f64);
    let still = source.clone();
    let (fx, fy) = (ox as f64, oy as f64);
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        if let Some((x, y)) = stretch_from(&k, (i % w) as f64 - fx + 0.5, (i / w) as f64 - fy + 0.5) {
            px.copy_from_slice(&sample_bilinear(&still, x + fx, y + fy));
        }
    });
}

/// D-515: Push Iris's numbers in the drawing's own `w0` by `h0` pixels, shared with the card:
/// the centre (two), the largest distance of a corner from it, the hole's radius, and 1 reversed.
pub(crate) fn push_numbers(e: &crate::effects::Effect, w0: f64, h0: f64) -> [f64; 5] {
    let crate::effects::Effect::PushIris { completion, center, reverse } = e else {
        unreachable!("Push Iris's numbers of another effect")
    };
    let (cx, cy) = (center[0] / 100.0 * w0, center[1] / 100.0 * h0);
    let far = [(0.0, 0.0), (w0, 0.0), (0.0, h0), (w0, h0)].iter().map(|&(x, y)| (x - cx).hypot(y - cy)).fold(0.0, f64::max);
    [cx, cy, far, completion / 100.0 * far, (reverse == "on") as u8 as f64]
}

/// D-515: where the own point (x, y) is read from, in the drawing's own space, or None where it
/// is clear (document 21).
fn push_from(k: &[f64; 5], x: f64, y: f64) -> Option<(f64, f64)> {
    let (dx, dy) = (x - k[0], y - k[1]);
    let r = (dx * dx + dy * dy).sqrt();
    let f = if k[4] == 1.0 {
        if r >= k[2] - k[3] {
            return None;
        }
        if r == 0.0 {
            return Some((k[0] + k[3], k[1]));
        }
        (r + k[3]) / r
    } else {
        if r < k[3] {
            return None;
        }
        (r - k[3]) / r
    };
    Some((k[0] + dx * f, k[1] + dy * f))
}

/// D-515: after CycoreFX's CC Radial ScaleWipe, the rule ours (document 21); `(ox, oy)` the
/// drawing's corner in the buffer. Never grows; at completion 0 returns at once (compose leaves
/// it off the card then).
pub(crate) fn push_iris(source: &mut WorkingBuffer, e: &crate::effects::Effect, (ox, oy): (usize, usize)) {
    if matches!(e, crate::effects::Effect::PushIris { completion, .. } if *completion == 0.0) {
        return;
    }
    let (w, h) = (source.width(), source.height());
    let k = push_numbers(e, (w - 2 * ox) as f64, (h - 2 * oy) as f64);
    let still = source.clone();
    let (fx, fy) = (ox as f64, oy as f64);
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let p = match push_from(&k, (i % w) as f64 - fx + 0.5, (i / w) as f64 - fy + 0.5) {
            Some((x, y)) => sample_bilinear(&still, x + fx, y + fy),
            None => [0.0; 4],
        };
        px.copy_from_slice(&p);
    });
}

/// D-514: Stripe Sweep's numbers in the drawing's own `w0` by `h0` pixels, shared with the card:
/// the middle (two), the direction's cosine and sine, the largest distance of a corner along the
/// sweep and across it, the thickness, the number of lines, the overlap (the slant as a share of
/// 99), each line's share of the transition, the completion as a share, and 1 flipped.
pub(crate) fn stripe_numbers(e: &crate::effects::Effect, w0: f64, h0: f64) -> [f64; 12] {
    let crate::effects::Effect::StripeSweep { completion, direction, thickness, slant, flip } = e else {
        unreachable!("Stripe Sweep's numbers of another effect")
    };
    let (sn, cs) = direction.to_radians().sin_cos();
    let corners = [(-w0 / 2.0, -h0 / 2.0), (w0 / 2.0, -h0 / 2.0), (-w0 / 2.0, h0 / 2.0), (w0 / 2.0, h0 / 2.0)];
    let along = corners.iter().map(|&(x, y)| (x * cs + y * sn).abs()).fold(0.0, f64::max);
    let across = corners.iter().map(|&(x, y)| (-x * sn + y * cs).abs()).fold(0.0, f64::max);
    let n = (2.0 * across / thickness).ceil().max(1.0);
    let o = slant / 99.0;
    let s = 1.0 / (1.0 + (n - 1.0) * (1.0 - o));
    [w0 / 2.0, h0 / 2.0, cs, sn, along, across, *thickness, n, o, s, completion / 100.0, (flip == "on") as u8 as f64]
}

/// D-514: where the own point (x, y) is along its line, 0 to 1, and how far that line has gone;
/// the point is kept where the first is not below the second (document 21).
fn stripe_place(k: &[f64; 12], x: f64, y: f64) -> (f64, f64) {
    let (dx, dy) = (x - k[0], y - k[1]);
    let (u, v) = (dx * k[2] + dy * k[3], -dx * k[3] + dy * k[2]);
    let i = ((v + k[5]) / k[6]).floor().min(k[7] - 1.0);
    let t = (u + k[4]) / (2.0 * k[4]);
    let t = if k[11] == 1.0 { 1.0 - t } else { t };
    (t, ((k[10] - i * k[9] * (1.0 - k[8])) / k[9]).clamp(0.0, 1.0))
}

/// D-514: after CycoreFX's CC Line Sweep, the rule ours (document 21); `(ox, oy)` the drawing's
/// corner in the buffer. Never grows; at completion 0 returns at once (compose leaves it off the
/// card then).
pub(crate) fn stripe_sweep(source: &mut WorkingBuffer, e: &crate::effects::Effect, (ox, oy): (usize, usize)) {
    if matches!(e, crate::effects::Effect::StripeSweep { completion, .. } if *completion == 0.0) {
        return;
    }
    let (w, h) = (source.width(), source.height());
    let k = stripe_numbers(e, (w - 2 * ox) as f64, (h - 2 * oy) as f64);
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (t, p) = stripe_place(&k, (i % w) as f64 - ox as f64 + 0.5, (i / w) as f64 - oy as f64 + 0.5);
        if t < p {
            px.fill(0.0);
        }
    });
}

/// D-513: Glow Wipe's numbers in the drawing's own `w0` by `h0` pixels, shared with the card:
/// the centre (two), the direction's cosine and sine, the shape (an index into
/// `effects::GLOW_SHAPES`), the farthest corner's distance, the band, the front, the intensity
/// as a share, 1 reversed, 1 with the light's colour from the layer, and the light's linear
/// colour (three).
pub(crate) fn glow_numbers(e: &crate::effects::Effect, w0: f64, h0: f64) -> [f64; 14] {
    let crate::effects::Effect::GlowWipe { completion, center, intensity, direction, shape, color_from_source, color, reverse } = e else {
        unreachable!("Glow Wipe's numbers of another effect")
    };
    let (sn, cs) = direction.to_radians().sin_cos();
    let mode = crate::effects::GLOW_SHAPES.iter().position(|s| s == shape).expect("Glow Wipe's shape was checked");
    let c = crate::effects::encoded(color).map(crate::grade::to_linear);
    let mut k = [center[0] / 100.0 * w0, center[1] / 100.0 * h0, cs, sn, mode as f64, 0.0, 0.0, 0.0, intensity / 100.0,
        (reverse == "on") as u8 as f64, (color_from_source == "on") as u8 as f64, c[0], c[1], c[2]];
    let far = [(0.0, 0.0), (w0, 0.0), (0.0, h0), (w0, h0)].iter().map(|&(x, y)| glow_distance(&k, x, y)).fold(0.0, f64::max);
    let b = far / 10.0;
    k[5] = far;
    k[6] = b;
    k[7] = completion / 100.0 * (far + 2.0 * b) - b;
    k
}

/// D-513: the shape's distance of the own point (x, y) from the centre (document 21).
fn glow_distance(k: &[f64; 14], x: f64, y: f64) -> f64 {
    let (dx, dy) = (x - k[0], y - k[1]);
    let (u, v) = (dx * k[2] + dy * k[3], -dx * k[3] + dy * k[2]);
    match k[4] as usize {
        0 => u.abs(),
        1 => (u * u + v * v).sqrt(),
        _ => u.abs().max(v.abs()),
    }
}

/// D-513: the own point (x, y) of premultiplied `p`: kept or cleared, the band's light added
/// (document 21).
fn glow_pixel(k: &[f64; 14], p: [f64; 4], x: f64, y: f64) -> [f64; 4] {
    let d = glow_distance(k, x, y);
    let s = if k[9] == 1.0 { k[5] - d } else { d };
    let keep = if s >= k[7] { 1.0 } else { 0.0 };
    let g = (1.0 - (s - k[7]).abs() / k[6]).max(0.0) * k[8];
    let (l, cover) = if k[10] == 1.0 {
        ([p[0] * g, p[1] * g, p[2] * g], (g * p[3]).min(1.0))
    } else {
        ([k[11] * g, k[12] * g, k[13] * g], g.min(1.0))
    };
    let qa = p[3] * keep;
    [p[0] * keep + l[0], p[1] * keep + l[1], p[2] * keep + l[2], qa + (1.0 - qa) * cover]
}

/// D-513: after CycoreFX's CC Light Wipe, the rule ours (document 21); `(ox, oy)` the drawing's
/// corner in the buffer. Never grows; at completion 0 returns at once (compose leaves it off the
/// card then).
pub(crate) fn glow_wipe(source: &mut WorkingBuffer, e: &crate::effects::Effect, (ox, oy): (usize, usize)) {
    if matches!(e, crate::effects::Effect::GlowWipe { completion, .. } if *completion == 0.0) {
        return;
    }
    let (w, h) = (source.width(), source.height());
    let k = glow_numbers(e, (w - 2 * ox) as f64, (h - 2 * oy) as f64);
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let p = [px[0] as f64, px[1] as f64, px[2] as f64, px[3] as f64];
        let out = glow_pixel(&k, p, (i % w) as f64 - ox as f64 + 0.5, (i / w) as f64 - oy as f64 + 0.5);
        for (v, o) in px.iter_mut().zip(out) {
            *v = o as f32;
        }
    });
}

/// D-511: Teeth Wipe's numbers in the drawing's own `w0` by `h0` pixels, shared with the card:
/// the centre (two), the direction's cosine and sine, the teeth's height and width, how far each
/// part has moved, and the shape (an index into `effects::TEETH_SHAPES`).
pub(crate) fn teeth_numbers(e: &crate::effects::Effect, w0: f64, h0: f64) -> [f64; 8] {
    let crate::effects::Effect::TeethWipe { completion, center, direction, height, width, shape } = e else {
        unreachable!("Teeth Wipe's numbers of another effect")
    };
    let (sn, cs) = direction.to_radians().sin_cos();
    let (cx, cy) = (center[0] / 100.0 * w0, center[1] / 100.0 * h0);
    let far = [(0.0, 0.0), (w0, 0.0), (0.0, h0), (w0, h0)]
        .iter()
        .map(|&(x, y)| (-(x - cx) * sn + (y - cy) * cs).abs())
        .fold(0.0, f64::max);
    let mode = crate::effects::TEETH_SHAPES.iter().position(|s| s == shape).expect("Teeth Wipe's shape was checked");
    [cx, cy, cs, sn, *height, *width, completion / 100.0 * (height / 2.0 + far), mode as f64]
}

/// D-511: how far across the cut the own point (x, y) is read from: `Some(o)` in the first
/// part, `Some(-o)` in the second, `None` in the gap between them (document 21).
fn teeth_side(k: &[f64; 8], x: f64, y: f64) -> Option<f64> {
    let (dx, dy) = (x - k[0], y - k[1]);
    let v = dx * k[2] + dy * k[3];
    let w = -dx * k[3] + dy * k[2];
    let f = v / k[5];
    let t = (2.0 * (f - f.floor()) - 1.0).abs();
    let s = match k[7] as usize {
        0 => t,
        1 => (2.0 * t - 0.5).clamp(0.0, 1.0),
        _ => (t > 0.5) as u8 as f64,
    };
    let g = k[4] * (s - 0.5);
    if w + k[6] < g {
        Some(k[6])
    } else if w - k[6] >= g {
        Some(-k[6])
    } else {
        None
    }
}

/// D-511: after CycoreFX's CC Jaws, the rule ours (document 21); `(ox, oy)` the drawing's corner
/// in the buffer. Never grows; at completion 0 returns at once (compose leaves it off the card
/// then).
pub(crate) fn teeth_wipe(source: &mut WorkingBuffer, e: &crate::effects::Effect, (ox, oy): (usize, usize)) {
    if matches!(e, crate::effects::Effect::TeethWipe { completion, .. } if *completion == 0.0) {
        return;
    }
    let (w, h) = (source.width(), source.height());
    let k = teeth_numbers(e, (w - 2 * ox) as f64, (h - 2 * oy) as f64);
    let still = source.clone();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
        let p = match teeth_side(&k, x - ox as f64, y - oy as f64) {
            Some(m) => sample_bilinear(&still, x + m * -k[3], y + m * k[2]),
            None => [0.0; 4],
        };
        px.copy_from_slice(&p);
    });
}

/// D-510: how long each tile takes to go, a share of the whole transition (document 21).
const TILE_SPREAD: f64 = 0.25;

/// D-510: Tile Wipe's numbers in the drawing's own `w0` by `h0` pixels, shared with the card:
/// the centre (two), the turn's cosine and sine, the tile size, the furthest tile distance, the
/// completion share, the shape (an index into `effects::TILE_SHAPES`), 1 when reversed, and the
/// border.
pub(crate) fn tile_numbers(e: &crate::effects::Effect, w0: f64, h0: f64) -> [f64; 10] {
    let crate::effects::Effect::TileWipe { completion, center, rotation, border, tiles, shape, reverse } = e else {
        unreachable!("Tile Wipe's numbers of another effect")
    };
    let (sn, cs) = rotation.to_radians().sin_cos();
    let mode = crate::effects::TILE_SHAPES.iter().position(|s| s == shape).expect("Tile Wipe's shape was checked");
    let mut k = [center[0] / 100.0 * w0, center[1] / 100.0 * h0, cs, sn, w0 / tiles, 0.0, completion / 100.0, mode as f64, (reverse == "on") as u8 as f64, *border];
    k[5] = [(0.0, 0.0), (w0, 0.0), (0.0, h0), (w0, h0)].iter().map(|&(x, y)| tile_measure(&k, tile_turn(&k, x, y))).fold(0.0, f64::max);
    k
}

/// D-510: the own point (x, y) less the centre, turned back by the rotation.
fn tile_turn(k: &[f64; 10], x: f64, y: f64) -> (f64, f64) {
    let (dx, dy) = (x - k[0], y - k[1]);
    (dx * k[2] + dy * k[3], -dx * k[3] + dy * k[2])
}

/// D-510: a tile centre's distance from the centre by the shape: doors, radial, rectangle.
fn tile_measure(k: &[f64; 10], t: (f64, f64)) -> f64 {
    match k[7] as usize {
        0 => t.0.abs(),
        1 => (t.0 * t.0 + t.1 * t.1).sqrt(),
        _ => t.0.abs().max(t.1.abs()),
    }
}

/// D-510: the share of the pixel kept at the own point (x, y) (document 21).
fn tile_keep(k: &[f64; 10], x: f64, y: f64) -> f64 {
    let q = tile_turn(k, x, y);
    let s = k[4];
    let t = ((q.0 / s + 0.5).floor() * s, (q.1 / s + 0.5).floor() * s);
    let m = (q.0 - t.0).abs().max((q.1 - t.1).abs());
    let mut u = if k[5] > 0.0 { (tile_measure(k, t) / k[5]).min(1.0) } else { 0.0 };
    if k[8] == 1.0 {
        u = 1.0 - u;
    }
    let p = ((k[6] - u * (1.0 - TILE_SPREAD)) / TILE_SPREAD).clamp(0.0, 1.0);
    let b = k[9];
    let a = (1.0 - p) * (s / 2.0 + b) - b / 2.0;
    if b > 0.0 {
        ((a - m) / b + 0.5).clamp(0.0, 1.0)
    } else if m < a {
        1.0
    } else {
        0.0
    }
}

/// D-510: after CycoreFX's CC Grid Wipe, the rule ours (document 21); `(ox, oy)` the drawing's
/// corner in the buffer. Never grows; at completion 0 returns at once (compose leaves it off the
/// card then), so a pixel on a tile's side is never lost.
pub(crate) fn tile_wipe(source: &mut WorkingBuffer, e: &crate::effects::Effect, (ox, oy): (usize, usize)) {
    if matches!(e, crate::effects::Effect::TileWipe { completion, .. } if *completion == 0.0) {
        return;
    }
    let (w, h) = (source.width(), source.height());
    let k = tile_numbers(e, (w - 2 * ox) as f64, (h - 2 * oy) as f64);
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let f = tile_keep(&k, (i % w) as f64 - ox as f64 + 0.5, (i / w) as f64 - oy as f64 + 0.5) as f32;
        for v in px.iter_mut() {
            *v *= f;
        }
    });
}

/// D-519: CC Spotlight's gel for one frame: `Some(None)` for a Light render or with no gel named,
/// `Some(Some(map))` when it was read, and `None` when a gel is named but was not read, which
/// leaves the layer as it is.
pub(crate) fn spot_gel(e: &crate::effects::Effect) -> Option<Option<&WorkingBuffer>> {
    let crate::effects::Effect::CcSpotlight { render, gel_layer, gel_map, .. } = e else {
        return None;
    };
    if !render.starts_with("gel_") {
        return Some(None);
    }
    match gel_map {
        Some(m) => Some(Some(&*m.0)),
        None => (gel_layer.as_str() == Some("")).then_some(None),
    }
}

/// D-507: CC Spotlight's numbers in the drawing's own `w0` by `h0` pixels, shared with the card:
/// the Light render (an index into the first four of `effects::SPOT_RENDERS`), the light F
/// (three), the axis A = T - F (three), the half opening and the inner angle in radians, the
/// intensity share, the light's linear colour (three), and 1 when there is any light at all (an
/// axis and an opening). D-519: 1 with a gel read, the axis's unit a (three), the gel's across e1
/// (three) and down e2 (three), the tangent of the half opening held at 89 degrees, half the
/// larger side, and the drawing's middle (two).
pub(crate) fn spotlight_numbers(e: &crate::effects::Effect, gel: bool, w0: f64, h0: f64) -> [f64; 28] {
    let crate::effects::Effect::CcSpotlight { from, to, height, cone_angle, edge_softness, color, intensity, render, .. } = e else {
        unreachable!("CC Spotlight's numbers of another effect")
    };
    let f = [from[0] / 100.0 * w0, from[1] / 100.0 * h0, -height / 100.0 * w0];
    let a = [to[0] / 100.0 * w0 - f[0], to[1] / 100.0 * h0 - f[1], -f[2]];
    let o = cone_angle.to_radians() / 2.0;
    let c = crate::effects::encoded(color).map(crate::grade::to_linear);
    let mode = crate::effects::SPOT_RENDERS.iter().position(|r| r == render).expect("CC Spotlight's render was checked");
    let lit = if a == [0.0; 3] || o == 0.0 { 0.0 } else { 1.0 };
    let unit = |v: [f64; 3]| {
        let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        if n > 0.0 { v.map(|c| c / n) } else { v }
    };
    let u = unit(a);
    let mut e1 = [1.0 - u[0] * u[0], -u[0] * u[1], -u[0] * u[2]];
    if (e1[0] * e1[0] + e1[1] * e1[1] + e1[2] * e1[2]).sqrt() < 1e-9 {
        e1 = [-u[1] * u[0], 1.0 - u[1] * u[1], -u[1] * u[2]];
    }
    let e1 = unit(e1);
    let e2 = [u[1] * e1[2] - u[2] * e1[1], u[2] * e1[0] - u[0] * e1[2], u[0] * e1[1] - u[1] * e1[0]];
    [
        (mode % 4) as f64, f[0], f[1], f[2], a[0], a[1], a[2], o, o * (1.0 - edge_softness / 100.0), intensity / 100.0, c[0], c[1], c[2], lit,
        gel as u8 as f64, u[0], u[1], u[2], e1[0], e1[1], e1[2], e2[0], e2[1], e2[2],
        o.min(89f64.to_radians()).tan(), w0.max(h0) / 2.0, w0 / 2.0, h0 / 2.0,
    ]
}

/// D-519: where the gel is read for the own point (x, y): g = (D . e1, D . e2) / ((D . a) tan o'),
/// at the middle plus g times half the larger side (document 21).
fn spot_gel_point(k: &[f64; 28], x: f64, y: f64) -> (f64, f64) {
    let d = [x - k[1], y - k[2], -k[3]];
    let dot = |v: &[f64]| d[0] * v[0] + d[1] * v[1] + d[2] * v[2];
    let s = dot(&k[15..18]) * k[24];
    (k[26] + dot(&k[18..21]) / s * k[25], k[27] + dot(&k[21..24]) / s * k[25])
}

/// D-507: the light's share at the own point (x, y) (document 21).
fn spot_share(k: &[f64; 28], x: f64, y: f64) -> f64 {
    if k[13] == 0.0 {
        return 0.0;
    }
    let (a, d) = ([k[4], k[5], k[6]], [x - k[1], y - k[2], -k[3]]);
    let cr = [a[1] * d[2] - a[2] * d[1], a[2] * d[0] - a[0] * d[2], a[0] * d[1] - a[1] * d[0]];
    let th = (cr[0] * cr[0] + cr[1] * cr[1] + cr[2] * cr[2]).sqrt().atan2(a[0] * d[0] + a[1] * d[1] + a[2] * d[2]);
    let (o, n) = (k[7], k[8]);
    if th <= n {
        1.0
    } else if th >= o {
        0.0
    } else {
        let t = (o - th) / (o - n);
        t * t * (3.0 - 2.0 * t)
    }
}

/// D-507: after CycoreFX's CC Spotlight, the rule ours (document 21); `(ox, oy)` the drawing's
/// corner in the buffer. D-519: `gel` the map lying on the drawing at its corner; where the light
/// falls its colour is multiplied by the gel over white. Never grows.
pub(crate) fn cc_spotlight(source: &mut WorkingBuffer, e: &crate::effects::Effect, gel: Option<&WorkingBuffer>, (ox, oy): (usize, usize)) {
    let (w, h) = (source.width(), source.height());
    let k = spotlight_numbers(e, gel.is_some(), (w - 2 * ox) as f64, (h - 2 * oy) as f64);
    let s = k[9];
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as f64 - ox as f64 + 0.5, (i / w) as f64 - oy as f64 + 0.5);
        let f = spot_share(&k, x, y);
        let q = f * s;
        let mut c = [k[10], k[11], k[12]];
        if let Some(m) = gel.filter(|_| f > 0.0) {
            let (gx, gy) = spot_gel_point(&k, x, y);
            let g = sample_bilinear(m, gx, gy);
            for (j, v) in c.iter_mut().enumerate() {
                *v *= (1.0 - g[3] as f64) + g[j] as f64;
            }
        }
        let p = [px[0] as f64, px[1] as f64, px[2] as f64, px[3] as f64];
        let out = match k[0] as usize {
            0 => [c[0] * q, c[1] * q, c[2] * q, q.min(1.0)],
            1 => [p[0] + c[0] * q * p[3], p[1] + c[1] * q * p[3], p[2] + c[2] * q * p[3], p[3]],
            2 => [p[0] + c[0] * q, p[1] + c[1] * q, p[2] + c[2] * q, p[3] + (1.0 - p[3]) * q.min(1.0)],
            _ => {
                let dark = (1.0 - f) * (1.0 - s.min(1.0));
                [p[0] * (dark + f * c[0]), p[1] * (dark + f * c[1]), p[2] * (dark + f * c[2]), p[3]]
            }
        };
        for (v, o) in px.iter_mut().zip(out) {
            *v = o as f32;
        }
    });
}

/// D-531: CC Environment's numbers for a drawing `w0` by `h0` and an environment `ew` by `eh`,
/// shared with the card: the mapping (an index into `effects::ENV_MAPPINGS`), 1 for four rays a
/// pixel (Filter Environment on a spherical map) else 0, the view's turn M = Ry(pan) Rx(tilt)
/// Rz(roll) by rows (nine), W/2, H/2, the focal length F, `ew` and `eh`.
pub(crate) fn environment_numbers(e: &crate::effects::Effect, w0: f64, h0: f64, (ew, eh): (usize, usize)) -> [f64; 16] {
    let crate::effects::Effect::CcEnvironment { mapping, horizontal_pan, tilt, roll, field_of_view, filter_environment, .. } = e else {
        unreachable!("CC Environment's numbers of another effect")
    };
    type M3 = [[f64; 3]; 3];
    let mul = |a: M3, b: M3| -> M3 { std::array::from_fn(|i| std::array::from_fn(|n| (0..3).map(|j| a[i][j] * b[j][n]).sum())) };
    let (sr, cr) = roll.to_radians().sin_cos();
    let (st, ct) = tilt.to_radians().sin_cos();
    let (sp, cp) = horizontal_pan.to_radians().sin_cos();
    let rz = [[cr, -sr, 0.0], [sr, cr, 0.0], [0.0, 0.0, 1.0]];
    let rx = [[1.0, 0.0, 0.0], [0.0, ct, -st], [0.0, st, ct]];
    let ry = [[cp, 0.0, sp], [0.0, 1.0, 0.0], [-sp, 0.0, cp]];
    let m = mul(ry, mul(rx, rz));
    let mode = crate::effects::ENV_MAPPINGS.iter().position(|r| r == mapping).expect("CC Environment's mapping was checked");
    let four = mode == 0 && filter_environment == "on";
    [
        mode as f64,
        if four { 1.0 } else { 0.0 },
        m[0][0],
        m[0][1],
        m[0][2],
        m[1][0],
        m[1][1],
        m[1][2],
        m[2][0],
        m[2][1],
        m[2][2],
        w0 / 2.0,
        h0 / 2.0,
        (w0 / 2.0) / (field_of_view.to_radians() / 2.0).tan(),
        ew as f64,
        eh as f64,
    ]
}

/// D-531: document 21's bilinear on the environment at (u, v), rows held within it and columns
/// taken round its width with `wrap`, else held; transparent when it is empty.
fn environment_tap(env: &WorkingBuffer, u: f64, v: f64, wrap: bool) -> [f64; 4] {
    let (w, h) = (env.width() as isize, env.height() as isize);
    let mut out = [0.0; 4];
    if w == 0 || h == 0 {
        return out;
    }
    let (fx, fy) = (u - 0.5, v - 0.5);
    let (x0, y0) = (fx.floor(), fy.floor());
    let (ux, uy) = (fx - x0, fy - y0);
    let (x0, y0) = (x0 as isize, y0 as isize);
    for (dy, wy) in [(0isize, 1.0 - uy), (1, uy)] {
        if wy == 0.0 {
            continue;
        }
        let sy = (y0 + dy).clamp(0, h - 1);
        for (dx, wx) in [(0isize, 1.0 - ux), (1, ux)] {
            if wx == 0.0 {
                continue;
            }
            let sx = if wrap { (x0 + dx).rem_euclid(w) } else { (x0 + dx).clamp(0, w - 1) };
            let px = env.pixel(sx as usize, sy as usize);
            for i in 0..4 {
                out[i] += px[i] as f64 * wx * wy;
            }
        }
    }
    out
}

/// D-531: the environment's colour along the ray through the drawing's point (x, y) (document 21).
fn environment_look(env: &WorkingBuffer, k: &[f64; 16], x: f64, y: f64) -> [f64; 4] {
    let c = [x - k[11], y - k[12], k[13]];
    let d: [f64; 3] = std::array::from_fn(|i| k[2 + 3 * i] * c[0] + k[3 + 3 * i] * c[1] + k[4 + 3 * i] * c[2]);
    let n = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    let [dx, dy, dz] = d.map(|q| q / n);
    let (ew, eh) = (k[14], k[15]);
    match k[0] as usize {
        0 => {
            let u = ew * (0.5 + dx.atan2(dz) / std::f64::consts::TAU);
            let v = eh * (0.5 - (-dy).clamp(-1.0, 1.0).asin() / std::f64::consts::PI);
            environment_tap(env, u, v, true)
        }
        1 => {
            let s = (dx * dx + dy * dy).sqrt();
            let q = if s == 0.0 { 0.0 } else { dz.clamp(-1.0, 1.0).acos() / (std::f64::consts::PI * s) };
            environment_tap(env, ew * (1.0 + dx * q) / 2.0, eh * (1.0 + dy * q) / 2.0, false)
        }
        _ => {
            let (ax, ay, az) = (dx.abs(), dy.abs(), dz.abs());
            let (col, row, a, b) = if ax >= ay && ax >= az {
                if dx > 0.0 { (2.0, 1.0, -dz / dx, dy / dx) } else { (0.0, 1.0, dz / ax, dy / ax) }
            } else if ay >= az {
                if dy < 0.0 { (1.0, 0.0, dx / ay, dz / ay) } else { (1.0, 2.0, dx / dy, -dz / dy) }
            } else if dz > 0.0 {
                (1.0, 1.0, dx / dz, dy / dz)
            } else {
                (1.0, 3.0, dx / az, -dy / az)
            };
            let (fw, fh) = (ew / 3.0, eh / 4.0);
            let u = (fw * (col + (a + 1.0) / 2.0)).max(col * fw + 0.5).min((col + 1.0) * fw - 0.5);
            let v = (fh * (row + (b + 1.0) / 2.0)).max(row * fh + 0.5).min((row + 1.0) * fh - 0.5);
            environment_tap(env, u, v, false)
        }
    }
}

/// D-531: after CycoreFX's CC Environment, the rule ours (document 21): the environment read
/// round a view turned by pan, tilt and roll, filling the layer's shape, whose drawing's corner
/// is at `(ox, oy)` in the buffer. Never grows.
pub(crate) fn cc_environment(source: &mut WorkingBuffer, e: &crate::effects::Effect, (ox, oy): (usize, usize)) {
    let crate::effects::Effect::CcEnvironment { map: Some(map), .. } = e else {
        unreachable!("CC Environment drawn with no environment read")
    };
    let env = &*map.0;
    let (w, h) = (source.width(), source.height());
    let k = environment_numbers(e, (w - 2 * ox) as f64, (h - 2 * oy) as f64, (env.width(), env.height()));
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let a = px[3];
        if a == 0.0 {
            px.fill(0.0);
            return;
        }
        let (x, y) = ((i % w) as f64 - ox as f64, (i / w) as f64 - oy as f64);
        let got = if k[1] == 1.0 {
            let mut sum = [0.0; 4];
            for sy in [-0.25, 0.25] {
                for sx in [-0.25, 0.25] {
                    let t = environment_look(env, &k, x + 0.5 + sx, y + 0.5 + sy);
                    for j in 0..4 {
                        sum[j] += t[j];
                    }
                }
            }
            sum.map(|s| s / 4.0)
        } else {
            environment_look(env, &k, x + 0.5, y + 0.5)
        };
        for (p, g) in px.iter_mut().zip(got) {
            *p = (g * a as f64) as f32;
        }
    });
}

/// D-530: CC Cylinder's numbers in the drawing's own `w0` by `h0` pixels, shared with the card:
/// the render (an index into `effects::CYLINDER_RENDERS`), the eye E (three), M = Rz Ry Rx by
/// rows (nine), the eye in the cylinder's own space M^T (E - C) (three), the radius, the light's
/// direction in the cylinder's own space (three), the intensity share, the light's linear colour
/// (three), ambient, diffuse, specular and metal as shares, the highlight's power (0 for none),
/// `w0` and `h0`. D-532: CC Sphere's the same, its offset the position and its centre in the
/// drawing's plane.
pub(crate) fn cylinder_numbers(e: &crate::effects::Effect, w0: f64, h0: f64) -> [f64; 31] {
    use crate::effects::Effect as E;
    #[rustfmt::skip]
    let (radius, position, position_z, rotation_x, rotation_y, rotation_z, render, light_intensity, light_color, light_height, light_direction, ambient, diffuse, specular, roughness, metal) = match e {
        E::CcCylinder { radius, position, position_z, rotation_x, rotation_y, rotation_z, render, light_intensity, light_color, light_height, light_direction, ambient, diffuse, specular, roughness, metal } =>
            (radius, position, position_z, rotation_x, rotation_y, rotation_z, render, light_intensity, light_color, light_height, light_direction, ambient, diffuse, specular, roughness, metal),
        E::CcSphere { radius, offset, rotation_x, rotation_y, rotation_z, render, light_intensity, light_color, light_height, light_direction, ambient, diffuse, specular, roughness, metal } =>
            (radius, offset, &0.0, rotation_x, rotation_y, rotation_z, render, light_intensity, light_color, light_height, light_direction, ambient, diffuse, specular, roughness, metal),
        _ => unreachable!("CC Cylinder's numbers of another effect"),
    };
    type M3 = [[f64; 3]; 3];
    let mul = |a: M3, b: M3| -> M3 { std::array::from_fn(|i| std::array::from_fn(|n| (0..3).map(|j| a[i][j] * b[j][n]).sum())) };
    let (sx, cx) = rotation_x.to_radians().sin_cos();
    let (sy, cy) = rotation_y.to_radians().sin_cos();
    let (sz, cz) = rotation_z.to_radians().sin_cos();
    let rx = [[1.0, 0.0, 0.0], [0.0, cx, -sx], [0.0, sx, cx]];
    let ry = [[cy, 0.0, sy], [0.0, 1.0, 0.0], [-sy, 0.0, cy]];
    let rz = [[cz, -sz, 0.0], [sz, cz, 0.0], [0.0, 0.0, 1.0]];
    let m = mul(rz, mul(ry, rx));
    let own = |v: [f64; 3]| -> [f64; 3] { std::array::from_fn(|i| (0..3).map(|j| m[j][i] * v[j]).sum()) };
    // A 50 mm lens on a 36 mm frame.
    let eye = [w0 / 2.0, h0 / 2.0, -w0 * 50.0 / 36.0];
    let c = [position[0] * w0 / 100.0, position[1] * h0 / 100.0, position_z * w0 / 100.0];
    let o = own([eye[0] - c[0], eye[1] - c[1], eye[2] - c[2]]);
    let (el, g) = ((light_height * 0.9).to_radians(), light_direction.to_radians());
    let l = own([el.cos() * g.sin(), -el.cos() * g.cos(), -el.sin()]);
    let lc = crate::effects::encoded(light_color).map(crate::grade::to_linear);
    let mode = crate::effects::CYLINDER_RENDERS.iter().position(|r| r == render).expect("CC Cylinder's render was checked");
    let power = if *roughness > 0.0 { 2.0 / (roughness / 100.0).powi(2) } else { 0.0 };
    [
        mode as f64,
        eye[0],
        eye[1],
        eye[2],
        m[0][0],
        m[0][1],
        m[0][2],
        m[1][0],
        m[1][1],
        m[1][2],
        m[2][0],
        m[2][1],
        m[2][2],
        o[0],
        o[1],
        o[2],
        radius * w0 / 200.0,
        l[0],
        l[1],
        l[2],
        light_intensity / 100.0,
        lc[0],
        lc[1],
        lc[2],
        ambient / 100.0,
        diffuse / 100.0,
        specular / 100.0,
        metal / 100.0,
        power,
        w0,
        h0,
    ]
}

/// D-530: one hit's lit colour from its premultiplied sample `c`, its normal along `hit` in the
/// cylinder's own space (the hit itself on CC Sphere's, D-532, its axial part 0 on CC Cylinder's)
/// on the ray `v`, the inside's normal turned round (document 21).
fn cylinder_shade(k: &[f64; 31], c: [f64; 4], hit: [f64; 3], v: [f64; 3], inside: bool) -> [f64; 4] {
    let sg = if inside { -1.0 } else { 1.0 };
    let n = hit.map(|q| sg * q / k[16]);
    let lv = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    let hv = [k[17] - v[0] / lv, k[18] - v[1] / lv, k[19] - v[2] / lv];
    let hl = (hv[0] * hv[0] + hv[1] * hv[1] + hv[2] * hv[2]).sqrt();
    let nl = n[0] * k[17] + n[1] * k[18] + n[2] * k[19];
    let s = if k[28] > 0.0 && nl > 0.0 && hl > 0.0 { ((n[0] * hv[0] + n[1] * hv[1] + n[2] * hv[2]) / hl).max(0.0).powf(k[28]) } else { 0.0 };
    let (diff, spec) = (k[25] * k[20] * nl.max(0.0), k[26] * k[20] * s);
    let lit = |j: usize| c[j] * (k[24] + diff * k[21 + j]) + spec * (k[27] * c[j] + (1.0 - k[27]) * k[21 + j] * c[3]);
    [lit(0), lit(1), lit(2), c[3]]
}

/// D-530: after CycoreFX's CC Cylinder, the rule ours (document 21): the drawing, whose corner is
/// at `(ox, oy)` in the buffer, wrapped round a lit open cylinder seen by a fixed eye; one ray a
/// pixel. Never grows.
pub(crate) fn cc_cylinder(source: &mut WorkingBuffer, e: &crate::effects::Effect, (ox, oy): (usize, usize)) {
    let (w, h) = (source.width(), source.height());
    let (dw, dh) = (w - 2 * ox, h - 2 * oy);
    let k = cylinder_numbers(e, dw as f64, dh as f64);
    let mut out = WorkingBuffer::transparent(w, h);
    let drawing = &*source;
    let (o, r, mode) = ([k[13], k[14], k[15]], k[16], k[0] as usize);
    out.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let d = [(i % w) as f64 - ox as f64 + 0.5 - k[1], (i / w) as f64 - oy as f64 + 0.5 - k[2], -k[3]];
        let v: [f64; 3] = std::array::from_fn(|n| k[4 + n] * d[0] + k[7 + n] * d[1] + k[10 + n] * d[2]);
        let a = v[0] * v[0] + v[2] * v[2];
        let b = o[0] * v[0] + o[2] * v[2];
        let disc = b * b - a * (o[0] * o[0] + o[2] * o[2] - r * r);
        if r == 0.0 || a == 0.0 || disc < 0.0 {
            return;
        }
        let sq = disc.sqrt();
        let mut got = [0.0f64; 4];
        // The inside first, the outside laid over it.
        for (t, inside) in [((-b + sq) / a, true), ((-b - sq) / a, false)] {
            let hy = o[1] + t * v[1];
            if (inside && mode == 1) || (!inside && mode == 2) || !(t > 0.0 && hy.abs() <= k[30] / 2.0) {
                continue;
            }
            let hit = [o[0] + t * v[0], hy, o[2] + t * v[2]];
            let phi = hit[0].atan2(-hit[2]);
            let s = sample_round(drawing, k[29] / 2.0 + phi / std::f64::consts::TAU * k[29], hy + k[30] / 2.0 + oy as f64, dw as isize, ox as isize);
            let lit = cylinder_shade(&k, s.map(f64::from), [hit[0], 0.0, hit[2]], v, inside);
            got = std::array::from_fn(|j| lit[j] + (1.0 - lit[3]) * got[j]);
        }
        for (p, g) in px.iter_mut().zip(got) {
            *p = g as f32;
        }
    });
    *source = out;
}

/// D-532: after CycoreFX's CC Sphere, the rule ours (document 21): the drawing, whose corner is
/// at `(ox, oy)` in the buffer, wrapped round a lit sphere as a map of the world, seen by CC
/// Cylinder's fixed eye and lit by its light; one ray a pixel. Never grows.
pub(crate) fn cc_sphere(source: &mut WorkingBuffer, e: &crate::effects::Effect, (ox, oy): (usize, usize)) {
    let (w, h) = (source.width(), source.height());
    let (dw, dh) = (w - 2 * ox, h - 2 * oy);
    let k = cylinder_numbers(e, dw as f64, dh as f64);
    let mut out = WorkingBuffer::transparent(w, h);
    let drawing = &*source;
    let (o, r, mode) = ([k[13], k[14], k[15]], k[16], k[0] as usize);
    let dot = |p: [f64; 3], q: [f64; 3]| p[0] * q[0] + p[1] * q[1] + p[2] * q[2];
    out.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let d = [(i % w) as f64 - ox as f64 + 0.5 - k[1], (i / w) as f64 - oy as f64 + 0.5 - k[2], -k[3]];
        let v: [f64; 3] = std::array::from_fn(|n| k[4 + n] * d[0] + k[7 + n] * d[1] + k[10 + n] * d[2]);
        let (a, b) = (dot(v, v), dot(o, v));
        let disc = b * b - a * (dot(o, o) - r * r);
        if r == 0.0 || disc < 0.0 {
            return;
        }
        let sq = disc.sqrt();
        let mut got = [0.0f64; 4];
        // The inside first, the outside laid over it.
        for (t, inside) in [((-b + sq) / a, true), ((-b - sq) / a, false)] {
            if (inside && mode == 1) || (!inside && mode == 2) || !(t > 0.0) {
                continue;
            }
            let hit: [f64; 3] = std::array::from_fn(|n| o[n] + t * v[n]);
            let phi = hit[0].atan2(-hit[2]);
            let lat = hit[1].atan2((hit[0] * hit[0] + hit[2] * hit[2]).sqrt());
            // The taps' rows held to the drawing: the same as the row held half a pixel in.
            let y = (k[30] / 2.0 + lat / std::f64::consts::PI * k[30]).clamp(0.5, k[30] - 0.5);
            let s = sample_round(drawing, k[29] / 2.0 + phi / std::f64::consts::TAU * k[29], y + oy as f64, dw as isize, ox as isize);
            let lit = cylinder_shade(&k, s.map(f64::from), hit, v, inside);
            got = std::array::from_fn(|j| lit[j] + (1.0 - lit[3]) * got[j]);
        }
        for (p, g) in px.iter_mut().zip(got) {
            *p = g as f32;
        }
    });
    *source = out;
}

type M3 = [[f64; 3]; 3];

fn m3_mul(a: &M3, b: &M3) -> M3 {
    std::array::from_fn(|i| std::array::from_fn(|j| a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j]))
}

fn m3_apply(m: &M3, v: [f64; 3]) -> [f64; 3] {
    std::array::from_fn(|i| m[i][0] * v[0] + m[i][1] * v[1] + m[i][2] * v[2])
}

/// D-551: the right-handed turn by `t` radians about axis 0 (x), 1 (y) or 2 (z).
fn m3_turn(axis: usize, t: f64) -> M3 {
    let (c, s) = (t.cos(), t.sin());
    match axis {
        0 => [[1.0, 0.0, 0.0], [0.0, c, -s], [0.0, s, c]],
        1 => [[c, 0.0, s], [0.0, 1.0, 0.0], [-s, 0.0, c]],
        _ => [[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]],
    }
}

/// D-551: CC Ball Action's turn for its rotation axis by `theta` degrees (document 21).
fn ball_turn(axis: &str, theta: f64) -> M3 {
    let t = theta.to_radians();
    let (x, y, z) = (m3_turn(0, t), m3_turn(1, t), m3_turn(2, t));
    match axis {
        "x" => x,
        "y" => y,
        "z" => z,
        "xy" => m3_mul(&y, &x),
        "xz" => m3_mul(&z, &x),
        "yz" => m3_mul(&z, &y),
        "xyz" => m3_mul(&z, &m3_mul(&y, &x)),
        "x15z" => m3_mul(&m3_turn(2, 15.0 * t), &x),
        _ => m3_mul(&m3_turn(2, 15.0 * t), &m3_mul(&y, &x)),
    }
}

/// D-551: `v` turned `theta` degrees about `a`, Rodrigues' way; left alone when `a` is 0.
fn rodrigues(v: [f64; 3], a: [f64; 3], theta: f64) -> [f64; 3] {
    let n = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt();
    if n == 0.0 {
        return v;
    }
    let k = a.map(|c| c / n);
    let t = theta.to_radians();
    let (c, s) = (t.cos(), t.sin());
    let cr = [k[1] * v[2] - k[2] * v[1], k[2] * v[0] - k[0] * v[2], k[0] * v[1] - k[1] * v[0]];
    let d = k[0] * v[0] + k[1] * v[1] + k[2] * v[2];
    std::array::from_fn(|i| v[i] * c + cr[i] * s + k[i] * d * (1.0 - c))
}

/// D-551: how many numbers lead [`ball_numbers`]: the light L and the half-way vector Hv.
pub(crate) const BALL_HEAD: usize = 6;

/// D-551: CC Ball Action's balls, shared with the card: L and Hv (three each), then for each ball
/// drawn, farthest first, its centre X and Y and radius R in the drawing's own space and its
/// colour, linear (six). `source` is the layer, its drawing's corner at `(ox, oy)` (document 21).
pub(crate) fn ball_numbers(source: &WorkingBuffer, e: &crate::effects::Effect, (ox, oy): (usize, usize)) -> Vec<f64> {
    let crate::effects::Effect::CcBallAction {
        scatter,
        rotation_axis,
        rotation,
        twist_property,
        twist_angle,
        grid_spacing,
        ball_size,
        instability_state,
    } = e
    else {
        unreachable!("CC Ball Action's balls of another effect")
    };
    let w = source.width();
    let (w0, h0) = ((w - 2 * ox) as f64, (source.height() - 2 * oy) as f64);
    let s = grid_spacing.max(1.0);
    let (cols, rows) = ((w0 / s).floor() as usize, (h0 / s).floor() as usize);
    let (gx, gy) = ((w0 - cols as f64 * s) / 2.0, (h0 - rows as f64 * s) / 2.0);
    let (cx, cy) = (w0 / 2.0, h0 / 2.0);
    let r = s * ball_size / 200.0;
    let z = w0 * 50.0 / 36.0;
    let base = crate::grade::mix(551);
    let data = source.data();
    let mut balls: Vec<(f64, usize, [f64; 6])> = (0..rows * cols)
        .into_par_iter()
        .filter_map(|cell| {
            let (i, j) = (cell % cols, cell / cols);
            let (x, y) = (gx + (i as f64 + 0.5) * s, gy + (j as f64 + 0.5) * s);
            let at = ((y.floor() as usize + oy) * w + x.floor() as usize + ox) * 4;
            if data[at + 3] != 1.0 {
                return None;
            }
            let col = [data[at] as f64, data[at + 1] as f64, data[at + 2] as f64];
            let un: [f64; 7] = std::array::from_fn(|ch| crate::grade::unit(base, i as i64, j as i64, 0, ch as u64));
            let mut u = [un[0], un[1], un[2]];
            if *instability_state != 0.0 {
                u = rodrigues(u, [un[3], un[4], un[5]], *instability_state);
            }
            let (xn, yn) = ((x - cx) / (w0 / 2.0), (y - cy) / (h0 / 2.0));
            let rho = (xn * xn + yn * yn).sqrt() / std::f64::consts::SQRT_2;
            let held = |v: f64| v.clamp(0.0, 1.0);
            let f = match twist_property.as_str() {
                "x_axis" => xn,
                "y_axis" => yn,
                "center_x" => xn.abs(),
                "center_y" => yn.abs(),
                "random" => (un[6] + 1.0) / 2.0,
                "red" => 1.0 - held(col[0]),
                "green" => 1.0 - held(col[1]),
                "blue" => 1.0 - held(col[2]),
                "brightness" => 1.0 - held(0.2126 * col[0] + 0.7152 * col[1] + 0.0722 * col[2]),
                "diamond" => 1.0 - (xn.abs() + yn.abs()) / 2.0,
                "rectangle" => 1.0 - xn.abs().max(yn.abs()),
                // radius and fast_top
                _ => 1.0 - rho,
            };
            let m = if twist_property == "fast_top" {
                let phi = (twist_angle * f).to_radians();
                m3_mul(&ball_turn(rotation_axis, *rotation), &m3_mul(&m3_turn(1, 2.0 * phi), &m3_turn(0, phi)))
            } else {
                ball_turn(rotation_axis, rotation + twist_angle * f)
            };
            let q = m3_apply(&m, [x - cx + scatter * u[0], y - cy + scatter * u[1], scatter * u[2]]);
            let d = z + q[2];
            (d > z / 100.0).then(|| (d, cell, [cx + q[0] * z / d, cy + q[1] * z / d, r * z / d, col[0], col[1], col[2]]))
        })
        .collect();
    balls.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
    let l = [-1.0, -1.0, -2.0].map(|v: f64| v / 6f64.sqrt());
    let hv = [l[0], l[1], l[2] - 1.0];
    let n = (hv[0] * hv[0] + hv[1] * hv[1] + hv[2] * hv[2]).sqrt();
    let mut k = vec![l[0], l[1], l[2], hv[0] / n, hv[1] / n, hv[2] / n];
    k.extend(balls.iter().flat_map(|b| b.2));
    k
}

/// D-551: how much the ball `b` (X, Y, R) covers the own point (x, y).
fn ball_cover(b: &[f64], x: f64, y: f64) -> f64 {
    let e = ((x - b[0]) * (x - b[0]) + (y - b[1]) * (y - b[1])).sqrt();
    (b[2] + 0.5 - e).clamp(0.0, 1.0) * (2.0 * b[2]).min(1.0)
}

/// D-551: the ball `b` lit at the own point (x, y), by the light and half-way vector in `head`.
fn ball_shade(head: &[f64], b: &[f64], x: f64, y: f64) -> [f64; 3] {
    let (dx, dy) = ((x - b[0]) / b[2], (y - b[1]) / b[2]);
    let q = dx * dx + dy * dy;
    let n = if q >= 1.0 { [dx / q.sqrt(), dy / q.sqrt(), 0.0] } else { [dx, dy, -(1.0 - q).sqrt()] };
    let dif = (n[0] * head[0] + n[1] * head[1] + n[2] * head[2]).max(0.0);
    let mut sp = (n[0] * head[3] + n[1] * head[4] + n[2] * head[5]).max(0.0);
    for _ in 0..5 {
        sp *= sp;
    }
    [0, 1, 2].map(|c| b[3 + c] * (0.25 + 0.75 * dif) + 0.5 * sp)
}

/// D-551: along one side of a buffer `size` long, the pixels a ball centred at `c` with radius
/// `r` may cover, the drawing's corner at `o`, as the card's `balls` pass bounds them.
fn ball_span(c: f64, r: f64, o: usize, size: usize) -> std::ops::Range<usize> {
    let lo = (c + o as f64 - r - 1.5).floor().clamp(0.0, size as f64) as usize;
    let hi = (c + o as f64 + r + 1.5).ceil().clamp(0.0, size as f64) as usize;
    lo..hi.max(lo)
}

/// D-551: the pixel at the own point (x, y) from its first and second balls, numbered 1 up (0
/// for none), as the card's `ballshade` pass has it.
pub(crate) fn ball_pixel(k: &[f64], first: u32, second: u32, x: f64, y: f64) -> [f64; 4] {
    if first == 0 {
        return [0.0; 4];
    }
    let ball = |key: u32| &k[BALL_HEAD + 6 * (key as usize - 1)..][..6];
    let (b1, c1) = (ball(first), ball_cover(ball(first), x, y));
    let s1 = ball_shade(k, b1, x, y);
    let mut p = [c1 * s1[0], c1 * s1[1], c1 * s1[2], c1];
    if second != 0 {
        let b2 = ball(second);
        let (c2, s2) = (ball_cover(b2, x, y), ball_shade(k, b2, x, y));
        let t = (1.0 - c1) * c2;
        p = [p[0] + t * s2[0], p[1] + t * s2[1], p[2] + t * s2[2], c1 + t];
    }
    p
}

/// D-551: after CycoreFX's CC Ball Action, the rule ours (document 21): the drawing as a grid of
/// lit balls, turned, twisted and scattered in a fake 3D with its own eye and light (P0-12: no
/// comp camera). `(ox, oy)` the drawing's corner in the buffer. Never grows.
pub(crate) fn cc_ball_action(source: &mut WorkingBuffer, e: &crate::effects::Effect, (ox, oy): (usize, usize)) {
    let (w, h) = (source.width(), source.height());
    let k = ball_numbers(source, e, (ox, oy));
    // The first and second balls at each pixel, numbered 1 up; laid farthest first, so each
    // ball covering a pixel is nearer than the one it moves to second.
    // ponytail: one thread lays every ball, the sum of their boxes' areas; lay rows of the
    // buffer in parallel if large balls at a fine spacing come to be slow.
    let (mut first, mut second) = (vec![0u32; w * h], vec![0u32; w * h]);
    for (n, b) in k[BALL_HEAD..].chunks_exact(6).enumerate() {
        for by in ball_span(b[1], b[2], oy, h) {
            let y = by as f64 - oy as f64 + 0.5;
            for bx in ball_span(b[0], b[2], ox, w) {
                if ball_cover(b, bx as f64 - ox as f64 + 0.5, y) > 0.0 {
                    let p = by * w + bx;
                    second[p] = first[p];
                    first[p] = n as u32 + 1;
                }
            }
        }
    }
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let out = ball_pixel(&k, first[i], second[i], (i % w) as f64 - ox as f64 + 0.5, (i / w) as f64 - oy as f64 + 0.5);
        for (v, o) in px.iter_mut().zip(out) {
            *v = o as f32;
        }
    });
}

/// D-580: Card Wipe's numbers for a drawing `w0` by `h0`, the CPU's and the card's (document
/// 21): the columns C and rows R, a card's width and height, its drawn half width and half
/// height, `w0`, `h0`, the focal length in pixels F, the eye in the cards' space (three), the
/// camera's turn M by rows (nine), the light's kind (0 distant, 1 point), its place in the cards'
/// space (three), its intensity, its linear colour (three), ambient, diffuse, specular, the
/// highlight's sharpness, 1 where a back picture was read, the card scale, the half thickness
/// of the slab every card lies in, then for each card, rows then columns, its axis (0 x, 1 y)
/// and the sine and cosine of its turn.
pub(crate) fn card_wipe_numbers(e: &crate::effects::Effect, w0: f64, h0: f64) -> Vec<f64> {
    #[rustfmt::skip]
    let crate::effects::Effect::CardWipe { transition_completion, transition_width, back_map, rows_columns, rows, columns, card_scale, flip_axis, flip_direction, flip_order, gradient_map, timing_randomness, random_seed, x_rotation, y_rotation, z_rotation, xy_position, z_position, focal_length, light_type, light_intensity, light_color, light_position, light_depth, ambient_light, diffuse_reflection, specular_reflection, highlight_sharpness, .. } = e
    else {
        unreachable!("Card Wipe's numbers of another effect")
    };
    let r = (rows + 0.5).floor();
    let c = if rows_columns == "columns_follow_rows" { r } else { (columns + 0.5).floor() };
    let (cw, ch) = (w0 / c, h0 / r);
    let (hx, hy) = (card_scale * cw / 2.0, card_scale * ch / 2.0);
    type M3 = [[f64; 3]; 3];
    let mul = |a: M3, b: M3| -> M3 { std::array::from_fn(|i| std::array::from_fn(|n| (0..3).map(|j| a[i][j] * b[j][n]).sum())) };
    let (sx, cx) = x_rotation.to_radians().sin_cos();
    let (sy, cy) = y_rotation.to_radians().sin_cos();
    let (sz, cz) = z_rotation.to_radians().sin_cos();
    let rx = [[1.0, 0.0, 0.0], [0.0, cx, -sx], [0.0, sx, cx]];
    let ry = [[cy, 0.0, sy], [0.0, 1.0, 0.0], [-sy, 0.0, cy]];
    let rz = [[cz, -sz, 0.0], [sz, cz, 0.0], [0.0, 0.0, 1.0]];
    let m = mul(rz, mul(ry, rx));
    let own = |v: [f64; 3]| -> [f64; 3] { std::array::from_fn(|i| (0..3).map(|j| m[j][i] * v[j]).sum()) };
    let f = w0 * focal_length / 36.0;
    let o = own([xy_position[0] * w0 / 100.0 - w0 / 2.0, xy_position[1] * h0 / 100.0 - h0 / 2.0, -z_position * f]);
    let pl = own([light_position[0] * w0 / 100.0 - w0 / 2.0, light_position[1] * h0 / 100.0 - h0 / 2.0, -light_depth * w0]);
    let lc = crate::effects::encoded(light_color).map(crate::grade::to_linear);
    let mut k = vec![c, r, cw, ch, hx, hy, w0, h0, f, o[0], o[1], o[2]];
    k.extend(m.iter().flatten());
    k.extend([if light_type == "point" { 1.0 } else { 0.0 }, pl[0], pl[1], pl[2], *light_intensity, lc[0], lc[1], lc[2]]);
    k.extend([*ambient_light, *diffuse_reflection, *specular_reflection, *highlight_sharpness]);
    k.extend([if back_map.is_some() { 1.0 } else { 0.0 }, *card_scale, hx.max(hy)]);
    let seed = crate::grade::mix(random_seed.floor() as u64);
    let g = |a: usize, b: usize, ch: u64| crate::grade::unit(seed, a as i64, b as i64, 0, ch);
    let (tr, done, wide) = (*timing_randomness, transition_completion / 100.0, transition_width / 100.0);
    let grad = gradient_map.as_ref().map(|m| &*m.0);
    let (cols, rows) = (c as usize, r as usize);
    k.reserve(3 * cols * rows);
    for b in 0..rows {
        for a in 0..cols {
            let (u, v) = ((a as f64 + 0.5) / c, (b as f64 + 0.5) / r);
            let order = match flip_order.as_str() {
                "right_to_left" => 1.0 - u,
                "top_to_bottom" => v,
                "bottom_to_top" => 1.0 - v,
                "top_left_to_bottom_right" => (u + v) / 2.0,
                "bottom_left_to_top_right" => (u + 1.0 - v) / 2.0,
                "top_right_to_bottom_left" => (1.0 - u + v) / 2.0,
                "bottom_right_to_top_left" => (2.0 - u - v) / 2.0,
                // The gradient's pixel under the card's centre; none read, every card at 0.
                "gradient" => grad.map_or(0.0, |p| {
                    let px = ((a as f64 + 0.5) * cw).floor().min(w0 - 1.0) as usize;
                    let py = ((b as f64 + 0.5) * ch).floor().min(h0 - 1.0) as usize;
                    if px < p.width() && py < p.height() { picture_luma(p.pixel(px, py)) } else { 0.0 }
                }),
                _ => u,
            };
            let o2 = (1.0 - tr) * order + tr * (g(a, b, 0) + 1.0) / 2.0;
            let p = if wide > 0.0 {
                ((done * (1.0 + wide) - o2) / wide).clamp(0.0, 1.0)
            } else if done > 0.0 && done >= o2 {
                1.0
            } else {
                0.0
            };
            let axis = match flip_axis.as_str() {
                "x" => 0.0,
                "y" => 1.0,
                _ => if g(a, b, 1) < 0.0 { 0.0 } else { 1.0 },
            };
            let sign = match flip_direction.as_str() {
                "positive" => 1.0,
                "negative" => -1.0,
                _ => if g(a, b, 2) < 0.0 { -1.0 } else { 1.0 },
            };
            let (s, cs) = (sign * p * std::f64::consts::PI).sin_cos();
            k.extend([axis, s, cs]);
        }
    }
    k
}

/// D-580: the columns or rows `0..n` a card of which may hold a point between `lo` and `hi` along
/// the ray, cards `size` apart with their centres from `size/2 - half_page`, each reaching `reach`
/// either side: one more each way than the reach asks, held to the deck, as a range.
fn card_span(lo: f64, hi: f64, size: f64, half_page: f64, reach: f64, n: f64) -> (usize, usize) {
    let first = ((lo + half_page - reach) / size - 0.5).floor() - 1.0;
    let last = ((hi + half_page + reach) / size - 0.5).ceil() + 1.0;
    (first.clamp(0.0, n) as usize, (last + 1.0).clamp(0.0, n) as usize)
}

/// D-580: After Effects' Card Wipe, the rule ours (document 21): the drawing, whose corner is at
/// `(ox, oy)` in the buffer, cut into a grid of cards that flip over in turn about their own
/// axes, seen by the effect's own camera and lit by its own light; each card's back shows its
/// share of the back picture, or is clear. One ray a pixel, the nearest card it meets winning.
/// Only the cards near the ray are tried: every card lies within its half size of the drawing's
/// plane, so a ray meets one only where it crosses that slab. Never grows.
pub(crate) fn card_wipe(source: &mut WorkingBuffer, e: &crate::effects::Effect, (ox, oy): (usize, usize)) {
    let crate::effects::Effect::CardWipe { back_map, .. } = e else {
        unreachable!("Card Wipe of another effect")
    };
    let (w, h) = (source.width(), source.height());
    let k = card_wipe_numbers(e, (w - 2 * ox) as f64, (h - 2 * oy) as f64);
    let back = back_map.as_ref().map(|m| &*m.0);
    let mut out = WorkingBuffer::transparent(w, h);
    let drawing = &*source;
    let (cols, rows, cw, ch, hx, hy, w0, h0) = (k[0], k[1], k[2], k[3], k[4], k[5], k[6], k[7]);
    let o = [k[9], k[10], k[11]];
    let (sc, zmax) = (k[34], k[35]);
    let dot = |p: [f64; 3], q: [f64; 3]| p[0] * q[0] + p[1] * q[1] + p[2] * q[2];
    let unit = |p: [f64; 3]| {
        let n = dot(p, p).sqrt();
        if n == 0.0 { [0.0; 3] } else { p.map(|q| q / n) }
    };
    out.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        if sc == 0.0 {
            return;
        }
        let d = [(i % w) as f64 - ox as f64 + 0.5 - w0 / 2.0, (i / w) as f64 - oy as f64 + 0.5 - h0 / 2.0, k[8]];
        let v: [f64; 3] = std::array::from_fn(|n| k[12 + n] * d[0] + k[15 + n] * d[1] + k[18 + n] * d[2]);
        // The slab, a little thicker than the cards reach, and the cards over its crossing.
        let zm = zmax * 1.001 + 1e-9;
        let ((a0, a1), (b0, b1)) = if v[2] != 0.0 {
            let (ta, tb) = ((-zm - o[2]) / v[2], (zm - o[2]) / v[2]);
            let (t0, t1) = (ta.min(tb).max(0.0), ta.max(tb));
            if t1 < t0 {
                return;
            }
            let (xa, xb) = (o[0] + t0 * v[0], o[0] + t1 * v[0]);
            let (ya, yb) = (o[1] + t0 * v[1], o[1] + t1 * v[1]);
            (card_span(xa.min(xb), xa.max(xb), cw, w0 / 2.0, hx, cols), card_span(ya.min(yb), ya.max(yb), ch, h0 / 2.0, hy, rows))
        } else if o[2].abs() > zm {
            return;
        } else {
            ((0, cols as usize), (0, rows as usize))
        };
        // The nearest hit, the first card in rows then columns where two tie.
        let mut best: Option<(f64, usize, [f64; 2], bool, [f64; 2])> = None;
        for b in b0..b1 {
            for a in a0..a1 {
                let at = 36 + 3 * (b * cols as usize + a);
                let (axis, s, c) = (k[at], k[at + 1], k[at + 2]);
                let (cx, cy) = ((a as f64 + 0.5) * cw, (b as f64 + 0.5) * ch);
                let q = [o[0] - (cx - w0 / 2.0), o[1] - (cy - h0 / 2.0), o[2]];
                let inv = |p: [f64; 3]| {
                    if axis == 0.0 {
                        [p[0], c * p[1] + s * p[2], -s * p[1] + c * p[2]]
                    } else {
                        [c * p[0] - s * p[2], p[1], s * p[0] + c * p[2]]
                    }
                };
                let (o2, d2) = (inv(q), inv(v));
                if d2[2] == 0.0 {
                    continue;
                }
                let t = -o2[2] / d2[2];
                if t <= 0.0 || best.is_some_and(|(bt, ..)| t >= bt) {
                    continue;
                }
                let (lx, ly) = (o2[0] + t * d2[0], o2[1] + t * d2[1]);
                if lx.abs() <= hx && ly.abs() <= hy {
                    best = Some((t, at, [lx, ly], d2[2] > 0.0, [cx, cy]));
                }
            }
        }
        let Some((t, at, [lx, ly], front, [cx, cy])) = best else {
            return;
        };
        let (axis, s, c) = (k[at], k[at + 1], k[at + 2]);
        let p = if front {
            sample_bilinear(drawing, cx + lx / sc + ox as f64, cy + ly / sc + oy as f64)
        } else {
            match back {
                None => [0.0; 4],
                Some(b) if axis == 1.0 => sample_bilinear(b, cx - lx / sc, cy + ly / sc),
                Some(b) => sample_bilinear(b, cx + lx / sc, cy - ly / sc),
            }
        }
        .map(f64::from);
        let pl = [k[22], k[23], k[24]];
        let hit: [f64; 3] = std::array::from_fn(|n| o[n] + t * v[n]);
        let lv = unit(if k[21] == 0.0 { pl } else { std::array::from_fn(|n| pl[n] - hit[n]) });
        let n = if axis == 0.0 { [0.0, s, -c] } else { [-s, 0.0, -c] };
        let n = if front { n } else { n.map(|q| -q) };
        let vv = unit(v.map(|q| -q));
        let hv = [lv[0] + vv[0], lv[1] + vv[1], lv[2] + vv[2]];
        let nl = dot(n, lv);
        let hn = dot(hv, hv).sqrt();
        let sp = if nl > 0.0 && hn > 0.0 { (dot(n, hv) / hn).max(0.0).powf(k[32]) } else { 0.0 };
        let (i_, a_, d_, s_) = (k[25], k[29], k[30], k[31]);
        for j in 0..3 {
            px[j] = (p[j] * (a_ + d_ * i_ * k[26 + j] * nl.max(0.0)) + s_ * i_ * sp * k[26 + j] * p[3]) as f32;
        }
        px[3] = p[3] as f32;
    });
    *source = out;
}

/// D-506: 3D Glasses. Each view is read in the drawing's own space, a map at (u, v) or the layer
/// itself at (u + ox, v + oy), transparent outside, by document 21's bilinear. The anaglyphs and
/// the interlace read the left view half the convergence c to the right and half the alignment
/// v up, the right view the other way: l = L(x + 1/2 - c/2, y + 1/2 + v/2), r = R(x + 1/2 + c/2,
/// y + 1/2 - v/2). Side by side squeezes each view into a half (the mean of each two columns),
/// over under likewise down; interlace takes even rows from the left. The balanced views first
/// draw the two toward their mean by the balance; then red green is (Y(r), Y(l), 0), red blue
/// (Y(r), Y(l), Y(l)) and balanced colored red blue (r.r, l.g, l.b), the alpha the larger, the
/// colour held at 0 or above and the alpha within 0 to 1. The layer never grows.
pub(crate) fn three_d_glasses(source: &mut WorkingBuffer, g: &Glasses, (ox, oy): (usize, usize)) {
    let (w, h) = (source.width(), source.height());
    let (w0, h0) = ((w - 2 * ox) as f64, (h - 2 * oy) as f64);
    let (c, v) = if g.percent {
        (g.convergence / 100.0 * w0, g.alignment / 100.0 * h0)
    } else {
        (g.convergence, g.alignment)
    };
    let src = source.clone();
    let read = |side: usize, sx: f64, sy: f64| -> [f64; 4] {
        let p = match g.maps[side] {
            Some(m) => sample_bilinear(m, sx, sy),
            None => sample_bilinear(&src, sx + ox as f64, sy + oy as f64),
        };
        p.map(|v| v as f64)
    };
    let luma = |p: [f64; 4]| 0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2];
    let view = crate::effects::GLASSES_VIEWS[g.view];
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as f64 - ox as f64, (i / w) as f64 - oy as f64);
        let out = match view {
            "side_by_side" if 2.0 * x < w0 => read(0, 2.0 * x + 1.0, y + 0.5),
            "side_by_side" => read(1, 2.0 * x + 1.0 - w0, y + 0.5),
            "over_under" if 2.0 * y < h0 => read(0, x + 0.5, 2.0 * y + 1.0),
            "over_under" => read(1, x + 0.5, 2.0 * y + 1.0 - h0),
            _ => {
                let mut l = read(0, x + 0.5 - c / 2.0, y + 0.5 + v / 2.0);
                let mut r = read(1, x + 0.5 + c / 2.0, y + 0.5 - v / 2.0);
                if view == "interlace" {
                    if (y as i64).rem_euclid(2) == 0 { l } else { r }
                } else {
                    if view.starts_with("balanced") {
                        for k in 0..4 {
                            let m = (l[k] + r[k]) / 2.0;
                            l[k] = m + g.balance * (l[k] - m);
                            r[k] = m + g.balance * (r[k] - m);
                        }
                    }
                    let a = l[3].max(r[3]).clamp(0.0, 1.0);
                    let rgb = match view {
                        "red_green" | "balanced_red_green" => [luma(r), luma(l), 0.0],
                        "balanced_colored_red_blue" => [r[0], l[1], l[2]],
                        _ => [luma(r), luma(l), luma(l)],
                    };
                    [rgb[0].max(0.0), rgb[1].max(0.0), rgb[2].max(0.0), a]
                }
            }
        };
        for k in 0..4 {
            px[k] = out[k] as f32;
        }
    });
}

/// D-509: Glass Melt for one frame: the gradient's map, `None` for the layer itself; the reveal's
/// map, `None` for nothing; the completion, the softness and the displacement.
pub(crate) struct Melt<'a> {
    pub gradient: Option<&'a WorkingBuffer>,
    pub reveal: Option<&'a WorkingBuffer>,
    pub completion: f64,
    pub softness: f64,
    pub amount: f64,
}

/// D-509: Glass Melt's wipe band, in the bump's 0 to 1.
pub(crate) const MELT_BAND: f64 = 0.1;

/// D-509: [`Melt`] from a Glass Melt whose settings are valid, or `None` when a layer is named but
/// was not read, which leaves the layer as it is.
pub(crate) fn melt_settings(e: &crate::effects::Effect) -> Option<Melt<'_>> {
    let crate::effects::Effect::GlassMelt {
        completion,
        layer_to_reveal,
        gradient_layer,
        softness,
        displacement_amount,
        reveal_map,
        gradient_map,
    } = e
    else {
        return None;
    };
    fn read<'a>(name: &serde_json::Value, map: &'a Option<crate::layer_map::Map>) -> Option<Option<&'a WorkingBuffer>> {
        match map {
            Some(m) => Some(Some(&*m.0)),
            None => (name.as_str() == Some("")).then_some(None),
        }
    }
    Some(Melt {
        gradient: read(gradient_layer, gradient_map)?,
        reveal: read(layer_to_reveal, reveal_map)?,
        completion: *completion,
        softness: *softness,
        amount: *displacement_amount,
    })
}

/// D-509: the pixel of `map` lying on the drawing with its corner at (ox, oy) under the buffer's
/// pixel `i` of a buffer `w` wide, transparent outside it.
fn on_drawing(map: &WorkingBuffer, i: usize, w: usize, (ox, oy): (usize, usize)) -> [f32; 4] {
    let (mx, my) = ((i % w).wrapping_sub(ox), (i / w).wrapping_sub(oy));
    if mx < map.width() && my < map.height() { map.pixel(mx, my) } else { [0.0; 4] }
}

/// D-509: Glass Melt, after CycoreFX's CC Glass Wipe, the rule ours
/// (`tools/glass_melt_reference.py`). The bump h is the luminance phase (D-316) of the gradient,
/// the map lying on the drawing at its corner (clear outside it) or the input itself, times its
/// covering, blurred at sigma `softness / 2` with the outer pixels repeated. Its slope n by
/// central differences, the neighbours held inside, times `1.25 max(softness, 1)`; the bend
/// D = `amount` n. With E = -B/2 + c(1 + B), c the completion over 100 and B [`MELT_BAND`],
/// k = clamp((h - E) / B + 1/2, 0, 1), and each pixel is k O(p + (1 - k) D) + (1 - k) R(p - k D),
/// both by document 21's bilinear, R the reveal lying on the drawing (clear with none). At
/// completion 0 nothing changes. The layer never grows.
pub(crate) fn glass_melt(source: &mut WorkingBuffer, g: &Melt, origin: (usize, usize)) {
    if g.completion <= 0.0 {
        return;
    }
    let (w, h) = (source.width(), source.height());
    let mut bump = WorkingBuffer::transparent(w, h);
    let from = source.data();
    bump.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let q = match g.gradient {
            Some(m) => on_drawing(m, i, w, origin),
            None => [from[i * 4], from[i * 4 + 1], from[i * 4 + 2], from[i * 4 + 3]],
        };
        px[3] = (crate::grade::phase_of("luminance", &q) * q[3] as f64) as f32;
    });
    if g.softness > 0.0 {
        crate::effects::held_blur_axes(&mut bump, &crate::effects::gaussian_weights(g.softness / 2.0), (true, true));
    }
    let data = bump.data();
    let at = |x: isize, y: isize| {
        let (x, y) = (x.clamp(0, w as isize - 1) as usize, y.clamp(0, h as isize - 1) as usize);
        data[(y * w + x) * 4 + 3] as f64
    };
    let gain = 1.25 * g.softness.max(1.0);
    let edge = -MELT_BAND / 2.0 + g.completion / 100.0 * (1.0 + MELT_BAND);
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    let still = source.clone();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as isize, (i / w) as isize);
        let dx = g.amount * (at(x + 1, y) - at(x - 1, y)) / 2.0 * gain;
        let dy = g.amount * (at(x, y + 1) - at(x, y - 1)) / 2.0 * gain;
        let k = ((at(x, y) - edge) / MELT_BAND + 0.5).clamp(0.0, 1.0);
        let (sx, sy) = (x as f64 + 0.5, y as f64 + 0.5);
        let o = sample_bilinear(&still, sx + (1.0 - k) * dx, sy + (1.0 - k) * dy);
        let r = match g.reveal {
            Some(m) if k < 1.0 => sample_bilinear(m, sx - k * dx - ox, sy - k * dy - oy),
            _ => [0.0; 4],
        };
        for c in 0..4 {
            px[c] = (k * o[c] as f64 + (1.0 - k) * r[c] as f64) as f32;
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
/// `clamp`: an opaque picture over the drawing, transparent outside it. D-552: the pass's alpha
/// `a` (1 unless a named pass picks one), held to 0..1, clear where 0; each value first divided
/// by `a` with `unmult`, and the result laid over `a`, premultiplied.
#[allow(clippy::too_many_arguments)]
pub(crate) fn pass_extract(
    source: &mut WorkingBuffer,
    pass: &WorkingBuffer,
    origin: (usize, usize),
    black: f64,
    white: f64,
    invert: bool,
    clamp: bool,
    unmult: bool,
) {
    let w = source.width();
    let p = pass.data();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let Some(j) = pass_at(pass, i, w, origin) else {
            px.fill(0.0);
            return;
        };
        let a = (p[j + 3] as f64).clamp(0.0, 1.0);
        if a <= 0.0 {
            px.fill(0.0);
            return;
        }
        for c in 0..3 {
            let v = p[j + c] as f64;
            let v = if unmult { v / a } else { v };
            let o = if white != black {
                (v - black) / (white - black)
            } else if v >= white {
                1.0
            } else {
                0.0
            };
            let o = if invert { 1.0 - o } else { o };
            px[c] = (if clamp { o.clamp(0.0, 1.0) } else { o } * a) as f32;
        }
        px[3] = a as f32;
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
    keep_by(source, &matte, origin);
}

/// D-526: what Numbers shows now, rules 1 and 2 of `tools/numbers_reference.py`: with random
/// values the value is r x (the value, or 30,000 when it is 0), r D-119's number for the layer's
/// frame; a number with its places (a minus dropped when every digit is 0), padded to five whole
/// digits with leading zeros; a non-drop timecode of the layer's time plus the value in seconds
/// (or the random value alone); a 24-hour time of the value in hours; a date the value's days
/// after 1 January 1995; a hexadecimal number of the value x 65,536.
pub(crate) fn numbers_shown(e: &crate::effects::Effect) -> String {
    let crate::effects::Effect::Numbers { kind, random_values, value, decimal_places, clock: [t, fps], .. } = e else {
        return String::new();
    };
    let random = random_values == "on";
    let mut v = *value;
    if random {
        let frame = (t * fps).round() as i64;
        let r = (crate::grade::unit(crate::grade::mix(0x4E55_4D42), 0, 0, frame, 0) + 1.0) / 2.0;
        v = r * if v != 0.0 { v } else { 30000.0 };
    }
    let d = decimal_places.floor().max(0.0) as usize;
    let unsigned = |s: String| match s.strip_prefix('-') {
        Some(rest) if !rest.bytes().any(|b| (b'1'..=b'9').contains(&b)) => rest.to_string(),
        _ => s,
    };
    match kind.as_str() {
        "number" => unsigned(format!("{v:.d$}")),
        "number_leading_zeros" => {
            let s = format!("{:.d$}", v.abs());
            let (whole, rest) = s.split_at(s.find('.').unwrap_or(s.len()));
            let s = format!("{whole:0>5}{rest}");
            if v < 0.0 { unsigned(format!("-{s}")) } else { s }
        }
        "timecode_30" | "timecode_25" | "timecode_24" => {
            let rate: i64 = kind[9..].parse().unwrap_or(30);
            let when = if random { v } else { t + v };
            let f = (when.abs() * rate as f64 + 1e-6).floor() as i64;
            let s = format!("{:02}:{:02}:{:02}:{:02}", f / (rate * 3600), f / (rate * 60) % 60, f / rate % 60, f % rate);
            if when < 0.0 && f > 0 { format!("-{s}") } else { s }
        }
        "time" => {
            let m = ((v * 60.0 + 1e-6).floor() as i64).rem_euclid(1440);
            format!("{:02}:{:02}", m / 60, m % 60)
        }
        "numerical_date" | "short_date" | "long_date" => {
            const MONTHS: [&str; 12] =
                ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
            const DAYS: [&str; 7] = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
            // Days from 1 January 1970, which was a Thursday; 1 January 1995 is day 9131.
            let z = (v + 1e-6).floor() as i64 + 9131;
            let (y, m, day) = civil_from_days(z);
            let month = MONTHS[m as usize - 1];
            match kind.as_str() {
                "numerical_date" => format!("{m}/{day}/{y}"),
                "short_date" => format!("{} {day}, {y}", &month[..3]),
                _ => format!("{}, {month} {day}, {y}", DAYS[(z + 4).rem_euclid(7) as usize]),
            }
        }
        _ => {
            let h = (v.abs() * 65536.0 + 1e-6).floor() as i64;
            format!("{}{h:X}", if v < 0.0 && h > 0 { "-" } else { "" })
        }
    }
}

/// The proleptic Gregorian date `z` days after 1 January 1970: year, month 1 to 12, day 1 to 31
/// (Howard Hinnant's civil-from-days).
fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    (yoe + era * 400 + i64::from(m <= 2), m, day)
}

/// D-526: whether Numbers can draw in `font` on this computer.
pub(crate) fn numbers_font_ok(font: &str) -> bool {
    crate::text::advance(font, 1.0, '0').is_some()
}

/// D-526: Numbers' words, rules 3 to 5 of `tools/numbers_reference.py`, as a picture `w` by `h`
/// with the drawing at `origin`: the fill picture F (D-263's text, or with proportional spacing
/// off each character its own text centred in a cell as wide as the font's widest digit), and
/// the stroke picture S (the same in the stroke colour with D-264's stroke round it), laid as
/// the display says: F; F over S; S x (1 - F's alpha); or that over F. `None` when the font
/// cannot be drawn.
pub(crate) fn numbers_picture(e: &crate::effects::Effect, (w, h): (usize, usize), origin: (usize, usize)) -> Option<WorkingBuffer> {
    numbers_words(e, &numbers_shown(e), (w, h), origin)
}

/// D-473: Basic Text's picture `w` by `h` with the drawing at `origin`: [`numbers_words`] of its
/// text with proportional spacing on, its other settings Numbers' own; an empty text draws
/// nothing. `None` when the font cannot be drawn.
pub(crate) fn basic_text_picture(e: &crate::effects::Effect, (w, h): (usize, usize), origin: (usize, usize)) -> Option<WorkingBuffer> {
    let crate::effects::Effect::BasicText {
        text, position, display, fill_color, stroke_color, stroke_width, size, tracking, composite_on_original, font, alignment,
    } = e
    else {
        return None;
    };
    if text.is_empty() {
        return Some(WorkingBuffer::transparent(w, h));
    }
    let numbers = crate::effects::Effect::Numbers {
        kind: "number".into(),
        random_values: "off".into(),
        value: 0.0,
        decimal_places: 0.0,
        position: *position,
        display: display.clone(),
        fill_color: fill_color.clone(),
        stroke_color: stroke_color.clone(),
        stroke_width: *stroke_width,
        size: *size,
        tracking: *tracking,
        proportional_spacing: "on".into(),
        composite_on_original: composite_on_original.clone(),
        font: font.clone(),
        alignment: alignment.clone(),
        clock: [0.0; 2],
    };
    numbers_words(&numbers, text, (w, h), origin)
}

/// D-526: [`numbers_picture`] of the words `s`; D-527's Timecode draws its count through it.
/// D-529: the last few pictures are kept by the words and how they are drawn, not by the frame,
/// so words that stay the same from frame to frame are drawn once.
// ponytail: four 1080p pictures, about 130 MB, as `text::drawn` keeps; key by layer if a project
// shows more Numbers or Timecode words than that redrawing on every frame.
fn numbers_words(e: &crate::effects::Effect, s: &str, (w, h): (usize, usize), origin: (usize, usize)) -> Option<WorkingBuffer> {
    let crate::effects::Effect::Numbers {
        position, display, fill_color, stroke_color, stroke_width, size, tracking, proportional_spacing, font, alignment, ..
    } = e
    else {
        return None;
    };
    // Every f64 written as Rust's shortest exact form, so two keys are equal only when every
    // setting is.
    let key = format!(
        "{position:?}|{display}|{fill_color}|{stroke_color}|{stroke_width:?}|{size:?}|{tracking:?}|{proportional_spacing}|{font}|{alignment}|{w}x{h}|{origin:?}|{s}"
    );
    static KEPT: std::sync::Mutex<Vec<(String, WorkingBuffer)>> = std::sync::Mutex::new(Vec::new());
    let hit = |kept: &mut Vec<(String, WorkingBuffer)>| {
        let i = kept.iter().position(|(k, _)| *k == key)?;
        let found = kept.remove(i);
        let picture = found.1.clone();
        kept.push(found);
        Some(picture)
    };
    if let Some(p) = hit(&mut KEPT.lock().unwrap_or_else(|e| e.into_inner())) {
        return Some(p);
    }
    let picture = numbers_drawn(e, s, (w, h), origin)?;
    let mut kept = KEPT.lock().unwrap_or_else(|e| e.into_inner());
    if hit(&mut kept).is_none() {
        if kept.len() == 4 {
            kept.remove(0);
        }
        kept.push((key, picture.clone()));
    }
    Some(picture)
}

/// [`numbers_words`] drawn afresh.
fn numbers_drawn(e: &crate::effects::Effect, s: &str, (w, h): (usize, usize), origin: (usize, usize)) -> Option<WorkingBuffer> {
    let crate::effects::Effect::Numbers {
        position, display, fill_color, stroke_color, stroke_width, size, tracking, proportional_spacing, font, alignment, ..
    } = e
    else {
        return None;
    };
    let (px, py) = crate::effects::radial_center(*position, (w, h), origin);
    let align = crate::text::Align::parse(alignment)?;
    let linear = |c: &str| crate::effects::encoded(c).map(crate::grade::to_linear);
    let base = crate::text::Text { font: font.clone(), size: *size, ..Default::default() };
    let texts = |color: [f64; 3], stroke: Option<crate::text::Stroke>| -> Option<Vec<crate::text::Text>> {
        if proportional_spacing == "on" {
            return Some(vec![crate::text::Text { text: s.to_string(), color, at: [px, py], align, tracking: *tracking, stroke, ..base.clone() }]);
        }
        let cell = ('0'..='9').map(|c| crate::text::advance(font, *size, c)).try_fold(0.0f64, |m, a| a.map(|a| m.max(a)))?;
        let a = *tracking / 1000.0 * *size;
        let n = s.chars().count() as f64;
        let line = n * cell + (n - 1.0) * a;
        let x0 = match align {
            crate::text::Align::Center => px - line / 2.0,
            crate::text::Align::Right => px - line,
            _ => px,
        };
        s.chars()
            .enumerate()
            .map(|(i, ch)| {
                let x = x0 + i as f64 * (cell + a) + (cell - crate::text::advance(font, *size, ch)?) / 2.0;
                Some(crate::text::Text { text: ch.to_string(), color, at: [x, py], stroke: stroke.clone(), ..base.clone() })
            })
            .collect()
    };
    let draw = |texts: Vec<crate::text::Text>| -> Option<WorkingBuffer> {
        let mut out = WorkingBuffer::transparent(w, h);
        for t in &texts {
            // D-529: each character drawn and laid over only in the rows it reaches; over every
            // other row it is transparent, which leaves the pixel under it as it is.
            let (part, top) = crate::text::draw_rows(t, w, h)?;
            out.data_mut()[top * w * 4..(top + part.height()) * w * 4]
                .par_chunks_exact_mut(4)
                .zip(part.data().par_chunks_exact(4))
                .for_each(|(d, s)| d.copy_from_slice(&crate::composite::over_pixel([s[0], s[1], s[2], s[3]], [d[0], d[1], d[2], d[3]])));
        }
        Some(out)
    };
    let fill = draw(texts(linear(fill_color), None)?)?;
    if *stroke_width <= 0.0 || display == "fill_only" {
        return Some(if display == "stroke_only" { WorkingBuffer::transparent(w, h) } else { fill });
    }
    let stroke_color = linear(stroke_color);
    let mut stroke = draw(texts(stroke_color, Some(crate::text::Stroke { color: stroke_color, width: *stroke_width }))?)?;
    if display == "fill_over_stroke" {
        crate::composite::over(&fill, &mut stroke).ok()?;
        return Some(stroke);
    }
    // The ring outside the letters: the stroke where the fill does not cover.
    stroke.data_mut().par_chunks_exact_mut(4).zip(fill.data().par_chunks_exact(4)).for_each(|(px, f)| {
        let k = 1.0 - f[3] as f64;
        for v in px.iter_mut() {
            *v = (*v as f64 * k) as f32;
        }
    });
    if display == "stroke_only" {
        return Some(stroke);
    }
    let mut out = fill;
    crate::composite::over(&stroke, &mut out).ok()?;
    Some(out)
}

/// D-527: what Timecode shows, rule 3 of `tools/timecode_reference.py`, from the count compose
/// worked: a minus when it is below 0, then on its size: five digits or more of frames; feet
/// (16 or 40 frames a foot) with four digits, a plus and the frames over with two; or
/// hh:mm:ss:ff, with drop frame R / 15 frame numbers left out at each minute but every tenth and
/// a semicolon before the frames.
pub(crate) fn timecode_shown(e: &crate::effects::Effect) -> String {
    let crate::effects::Effect::Timecode { display_format, count: [f, rate, drop], .. } = e else {
        return String::new();
    };
    let sign = if *f < 0.0 { "-" } else { "" };
    let mut f = f.abs() as i64;
    let rate = (*rate as i64).max(1);
    match display_format.as_str() {
        "frames" => format!("{sign}{f:05}"),
        "feet_frames_35mm" | "feet_frames_16mm" => {
            let per = if display_format == "feet_frames_35mm" { 16 } else { 40 };
            format!("{sign}{:04}+{:02}", f / per, f % per)
        }
        _ => {
            let mut sep = ':';
            if *drop == 1.0 {
                let d = rate / 15;
                let per_ten = 600 * rate - 9 * d;
                let (tens, m) = (f / per_ten, f % per_ten);
                f += 9 * d * tens + if m > d { d * ((m - d) / (60 * rate - d)) } else { 0 };
                sep = ';';
            }
            format!("{sign}{:02}:{:02}:{:02}{sep}{:02}", f / (rate * 3600), f / (rate * 60) % 60, f / rate % 60, f % rate)
        }
    }
}

/// D-527: Timecode's picture `w` by `h` with the drawing at `origin`, rules 4 and 5 of
/// `tools/timecode_reference.py`: [`timecode_shown`] in the bundled font, each character centred
/// in a cell as wide as its widest digit (Numbers' words with proportional spacing off, centred
/// on the position), over the box with show box on: the line's cells and size / 5 round them,
/// up to the top of the font's "0", each edge at the nearest whole pixel, in the box colour at
/// the opacity. `None` when the bundled font cannot be read.
pub(crate) fn timecode_picture(e: &crate::effects::Effect, (w, h): (usize, usize), origin: (usize, usize)) -> Option<WorkingBuffer> {
    let crate::effects::Effect::Timecode { text_position, text_size, text_color, show_box, box_color, opacity, .. } = e else {
        return None;
    };
    let font = crate::text::Text::BUNDLED_FONT;
    let s = timecode_shown(e);
    let numbers = crate::effects::Effect::Numbers {
        kind: "number".into(),
        random_values: "off".into(),
        value: 0.0,
        decimal_places: 0.0,
        position: *text_position,
        display: "fill_only".into(),
        fill_color: text_color.clone(),
        stroke_color: text_color.clone(),
        stroke_width: 0.0,
        size: *text_size,
        tracking: 0.0,
        proportional_spacing: "off".into(),
        composite_on_original: "on".into(),
        font: font.into(),
        alignment: "center".into(),
        clock: [0.0; 2],
    };
    let words = numbers_words(&numbers, &s, (w, h), origin)?;
    if show_box != "on" {
        return Some(words);
    }
    let (px, py) = crate::effects::radial_center(*text_position, (w, h), origin);
    let cell = ('0'..='9').map(|c| crate::text::advance(font, *text_size, c)).try_fold(0.0f64, |m, a| a.map(|a| m.max(a)))?;
    let line = s.chars().count() as f64 * cell;
    let x0 = px - line / 2.0;
    let (p, k) = (*text_size / 5.0, crate::text::top(font, *text_size, '0')?);
    let edge = |v: f64, most: usize| ((v + 0.5).floor().max(0.0) as usize).min(most);
    let (l, r, t, b) = (edge(x0 - p, w), edge(x0 + line + p, w), edge(py - k - p, h), edge(py + p, h));
    let a = *opacity / 100.0;
    let c = crate::effects::encoded(box_color).map(crate::grade::to_linear);
    let fill = [c[0] * a, c[1] * a, c[2] * a, a].map(|v| v as f32);
    let mut out = WorkingBuffer::transparent(w, h);
    if l < r {
        for y in t..b {
            out.data_mut()[(y * w + l) * 4..(y * w + r) * 4].chunks_exact_mut(4).for_each(|q| q.copy_from_slice(&fill));
        }
    }
    crate::composite::over(&words, &mut out).ok()?;
    Some(out)
}

/// D-525: IDentifier's picture at the ids' size from each id v. "colors": the nearest whole id
/// k = floor(v + 0.5) at hue (k x 0.618033988749895) mod 1, HSV (hue, 0.7, 0.9) taken to linear;
/// "luma_matte": white where |v - id| < 0.5, else black; "raw": the grey clamp(v / 255, 0, 1)
/// taken to linear, so the 8-bit picture reads the id. All opaque. "alpha_matte": the matte
/// (1 or 0) in the first number, for [`keep_by`].
pub(crate) fn identifier(pass: &WorkingBuffer, id: f64, output: &str) -> WorkingBuffer {
    let mut out = WorkingBuffer::transparent(pass.width(), pass.height());
    out.data_mut().par_chunks_exact_mut(4).zip(pass.data().par_chunks_exact(4)).for_each(|(px, p)| {
        let v = p[0] as f64;
        let rgb = match output {
            "colors" => {
                let h = ((v + 0.5).floor() * 0.618033988749895).rem_euclid(1.0);
                hsv(h, 0.7, 0.9).map(crate::grade::to_linear)
            }
            "raw" => [crate::grade::to_linear((v / 255.0).clamp(0.0, 1.0)); 3],
            _ => [if (v - id).abs() < 0.5 { 1.0 } else { 0.0 }; 3],
        };
        for c in 0..3 {
            px[c] = rgb[c] as f32;
        }
        px[3] = 1.0;
    });
    out
}

/// HSV to RGB as Python's `colorsys.hsv_to_rgb` works it, `h` in 0..1.
fn hsv(h: f64, s: f64, v: f64) -> [f64; 3] {
    let i = (h * 6.0) as i64;
    let f = h * 6.0 - i as f64;
    let (p, q, t) = (v * (1.0 - s), v * (1.0 - s * f), v * (1.0 - s * (1.0 - f)));
    match i % 6 {
        0 => [v, t, p],
        1 => [q, v, p],
        2 => [p, v, t],
        3 => [p, q, v],
        4 => [t, p, v],
        _ => [v, p, q],
    }
}

/// D-349, D-522: the drawing times the matte's first number, the matte lying at `origin`;
/// transparent outside it.
pub(crate) fn keep_by(source: &mut WorkingBuffer, matte: &WorkingBuffer, origin: (usize, usize)) {
    let w = source.width();
    let m = matte.data();
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let k = pass_at(matte, i, w, origin).map_or(0.0, |j| m[j] as f64);
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

/// D-408: where a Transform moment `m` ([`crate::effects::transform_moment`]'s numbers) reads
/// each output pixel from, in the buffer of `size` whose drawing has its corner at `origin`: the
/// inverse of T(P) R(rotation) K(skew, axis) S(across, down) T(-A), as `[a, b, c, d, e, f]` with
/// the source at (a x + b y + c, d x + e y + f), or `None` when it cannot be undone (a scale of
/// 0). R turns clockwise on the screen; K slides along the skew axis by the skew's tangent.
pub(crate) fn transform_map(m: &[f64; 10], size: (usize, usize), origin: (usize, usize)) -> Option<[f64; 6]> {
    let (ax, ay) = crate::effects::radial_center([m[0], m[1]], size, origin);
    let (px, py) = crate::effects::radial_center([m[2], m[3]], size, origin);
    let mul = |a: [f64; 4], b: [f64; 4]| [a[0] * b[0] + a[1] * b[2], a[0] * b[1] + a[1] * b[3], a[2] * b[0] + a[3] * b[2], a[2] * b[1] + a[3] * b[3]];
    let turn = |deg: f64| {
        let (s, c) = deg.to_radians().sin_cos();
        [c, -s, s, c]
    };
    let shear = if m[6] == 0.0 { [1.0, 0.0, 0.0, 1.0] } else { mul(mul(turn(m[7]), [1.0, -m[6].to_radians().tan(), 0.0, 1.0]), turn(-m[7])) };
    let l = mul(mul(turn(m[8]), shear), [m[5] / 100.0, 0.0, 0.0, m[4] / 100.0]);
    let det = l[0] * l[3] - l[1] * l[2];
    if det == 0.0 || !det.is_finite() {
        return None;
    }
    let [a, b, d, e] = [l[3] / det, -l[1] / det, -l[2] / det, l[0] / det];
    Some([a, b, ax - (a * px + b * py), d, e, ay - (d * px + e * py)])
}

/// D-408: Keys' cubic with a = -0.5 (Catmull-Rom).
fn keys_cubic(d: f64) -> f64 {
    let d = d.abs();
    if d <= 1.0 {
        (1.5 * d - 2.5) * d * d + 1.0
    } else if d < 2.0 {
        ((-0.5 * d + 2.5) * d - 4.0) * d + 2.0
    } else {
        0.0
    }
}

/// D-408: the 4 by 4 cubic sample from pixel centres, transparent outside, as
/// `sample_bilinear`; then each number held at 0 or above and the covering at 1 or below.
pub(crate) fn sample_bicubic(src: &WorkingBuffer, x: f64, y: f64) -> [f32; 4] {
    let (w, h) = (src.width() as isize, src.height() as isize);
    let (fx, fy) = (x - 0.5, y - 0.5);
    if !(fx > -2.0 && fy > -2.0 && fx < w as f64 + 1.0 && fy < h as f64 + 1.0) {
        return [0.0; 4];
    }
    let (x0, y0) = (fx.floor(), fy.floor());
    let mut out = [0.0f32; 4];
    for j in -1..3isize {
        let wy = keys_cubic(fy - (y0 + j as f64));
        let sy = y0 as isize + j;
        if wy == 0.0 || sy < 0 || sy >= h {
            continue;
        }
        for i in -1..3isize {
            let wx = keys_cubic(fx - (x0 + i as f64));
            let sx = x0 as isize + i;
            if wx == 0.0 || sx < 0 || sx >= w {
                continue;
            }
            let weight = (wx * wy) as f32;
            let px = src.pixel(sx as usize, sy as usize);
            for c in 0..4 {
                out[c] += px[c] * weight;
            }
        }
    }
    [out[0].max(0.0), out[1].max(0.0), out[2].max(0.0), out[3].clamp(0.0, 1.0)]
}

/// D-408: the layer drawn through each moment's map (`None` draws nothing) times its opacity
/// (0 to 1), the moments summed in order and divided by their number once. The buffer keeps its
/// size. One moment through the unchanged map at full opacity leaves it as it is.
pub(crate) fn transform(source: &mut WorkingBuffer, moments: &[(Option<[f64; 6]>, f64)], bicubic: bool) {
    if let [(Some([1.0, 0.0, 0.0, 0.0, 1.0, 0.0]), 1.0)] = moments {
        return;
    }
    let w = source.width();
    let n = moments.len() as f32;
    let mut out = WorkingBuffer::transparent(w, source.height());
    let drawing = &*source;
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
            let mut sum = [0.0f32; 4];
            for (map, opacity) in moments {
                let Some([a, b, c, d, e, f]) = *map else { continue };
                let (sx, sy) = (a * x + b * y + c, d * x + e * y + f);
                let s = if bicubic { sample_bicubic(drawing, sx, sy) } else { sample_bilinear(drawing, sx, sy) };
                for k in 0..4 {
                    sum[k] += s[k] * *opacity as f32;
                }
            }
            if moments.len() > 1 {
                sum = sum.map(|v| v / n);
            }
            px.copy_from_slice(&sum);
        });
    *source = out;
}

/// D-630: Wave World, `tools/wave_world_reference.py`'s rule step for step, in double
/// precision: the water run on its grid from the layer's in point (after the pre-roll) to this
/// frame through the settings history compose filled, then drawn as a grey height map over the
/// drawing in its place. The layer's own picture is not used. The settings are already valid.
pub(crate) fn wave_world(source: &mut WorkingBuffer, e: &crate::effects::Effect, (ox, oy): (usize, usize)) {
    let crate::effects::Effect::WaveWorld {
        render_dry_areas,
        grid_res_downsamples,
        reflect_edges,
        ground_map,
        producer_1_type,
        producer_2_type,
        history,
        fps,
        grid_scale,
        ..
    } = e
    else {
        unreachable!("Wave World of another effect")
    };
    // The places of `Effect::wave_numbers`; a producer's eight from 11 and 19.
    const BRIGHT: usize = 0;
    const CONTRAST: usize = 1;
    const GAMMA: usize = 2;
    const CLEAR: usize = 3;
    const GRID: usize = 4;
    const SPEED: usize = 5;
    const DAMP: usize = 6;
    const PRE: usize = 7;
    const STEEP: usize = 8;
    const HEIGHT: usize = 9;
    const STRENGTH: usize = 10;
    const PROD: [usize; 2] = [11, 19];
    let (w, h) = (source.width(), source.height());
    let (lw, lh) = ((w - 2 * ox) as f64, (h - 2 * oy) as f64);
    let now = e.wave_numbers();
    let samples: Vec<&[f64]> = if history.is_empty() { vec![&now] } else { history.iter().map(Vec::as_slice).collect() };
    let (k0, kn) = (samples[0], samples[samples.len() - 1]);
    let r = *fps;
    let tau = 2.0 * std::f64::consts::PI;

    // Step 2: the grid.
    let rnd = |v: f64| (v + 0.5).floor();
    let mut n = rnd(k0[GRID]).max(1.0);
    if grid_res_downsamples == "on" {
        n = rnd(n * grid_scale).max(1.0);
    }
    let m = rnd(n * lh / lw).max(1.0);
    let (n, m) = (n as usize, m as usize);
    let (dx, dy) = (1.0 / n as f64, lh / (m as f64 * lw));

    // Step 3: the ground's luma per cell, rows then columns.
    let g: Vec<f64> = (0..n * m)
        .map(|c| match ground_map {
            Some(map) => picture_luma(sample_bilinear(&map.0, ((c % n) as f64 + 0.5) * dx * lw, ((c / n) as f64 + 0.5) * dy * lw)),
            None => 0.0,
        })
        .collect();
    let steps_for = |ks: &[&[f64]]| {
        let cmax = ks.iter().map(|k| k[SPEED] * (2.0 * k[HEIGHT]).sqrt()).fold(f64::MIN, f64::max);
        ((1.6 * cmax / (r * dx.min(dy))).ceil() as usize).max(4)
    };
    // Step 6: a producer's weight per cell and its drive.
    let types = [producer_1_type, producer_2_type];
    let weights = |k: &[f64], p: usize| {
        let o = PROD[p];
        let (ht, wd) = (k[o + 2], k[o + 3]);
        let mut out = vec![0.0; n * m];
        if wd == 0.0 || ht == 0.0 {
            return out;
        }
        let (cx, cy) = (k[o] * lw / 100.0 / lw, k[o + 1] * lh / 100.0 / lw);
        let th = k[o + 4].to_radians();
        let (co, si) = (th.cos(), th.sin());
        for (c, wc) in out.iter_mut().enumerate() {
            let (ox, oy) = (((c % n) as f64 + 0.5) * dx - cx, ((c / n) as f64 + 0.5) * dy - cy);
            let (a, b) = (ox * co + oy * si, -ox * si + oy * co);
            if types[p] == "ring" {
                let rr = (2.0 * a / wd).powi(2) + (2.0 * b / ht).powi(2);
                if rr < 1.0 {
                    *wc = (1.0 - rr).powi(2);
                }
            } else if a.abs() <= ht / 2.0 && (2.0 * b).abs() < wd {
                *wc = (1.0 - (2.0 * b / wd).powi(2)).powi(2);
            }
        }
        out
    };
    let drive = |k: &[f64], p: usize, t: f64| {
        let ph = k[PROD[p] + 7].to_radians();
        k[PROD[p] + 5] * ((t + ph).sin() - ph.sin())
    };

    // Steps 4 and 5: the water, still at the start.
    let (mut hh, mut uu) = (vec![0.0; n * m], vec![0.0; n * m]);
    let mut lap = vec![0.0; n * m];
    let reflect = reflect_edges == "all";
    // The last weights worked, by the producer's place, size and angle.
    let mut cached: [(Vec<f64>, Vec<f64>); 2] = Default::default();
    let mut one = |ka: &[f64], kb: &[f64], ta: [f64; 2], tb: [f64; 2], dt: f64| {
        let d: Vec<f64> = g.iter().map(|gi| ka[HEIGHT] - ka[STEEP] * gi).collect();
        let c2: Vec<f64> = d.iter().map(|di| 2.0 * di * ka[SPEED].powi(2)).collect();
        let (h0, u0) = (&hh, &uu);
        let row_lap = |(j, row): (usize, &mut [f64])| {
            let at = |i: isize, jj: isize, c: usize, s: f64| {
                if (0..n as isize).contains(&i) && (0..m as isize).contains(&jj) {
                    let q = jj as usize * n + i as usize;
                    return if d[q] > 0.0 { h0[q] } else { h0[c] };
                }
                if reflect || c2[c] == 0.0 {
                    h0[c]
                } else {
                    h0[c] - s * u0[c] / c2[c].sqrt()
                }
            };
            for (i, l) in row.iter_mut().enumerate() {
                let c = j * n + i;
                if d[c] <= 0.0 {
                    continue;
                }
                let (ii, jj) = (i as isize, j as isize);
                *l = (at(ii - 1, jj, c, dx) + at(ii + 1, jj, c, dx) - 2.0 * h0[c]) / (dx * dx)
                    + (at(ii, jj - 1, c, dy) + at(ii, jj + 1, c, dy) - 2.0 * h0[c]) / (dy * dy);
            }
        };
        // ponytail: a step's rows go to the threads only on a grid of 65536 cells or more;
        // below that, waiting for the threads every step cost more than the sums (B-510's
        // timing). Tune the threshold if a quiet machine says otherwise.
        if n * m >= 1 << 16 {
            lap.par_chunks_mut(n).enumerate().for_each(row_lap);
        } else {
            lap.chunks_mut(n).enumerate().for_each(row_lap);
        }
        let damp = (-10.0 * ka[DAMP] * dt).exp();
        for c in 0..n * m {
            if d[c] > 0.0 {
                uu[c] += dt * c2[c] * lap[c];
                uu[c] *= damp;
                hh[c] += dt * uu[c];
            }
        }
        for p in 0..2 {
            let dp = drive(kb, p, tb[p]) - drive(ka, p, ta[p]);
            if dp != 0.0 {
                let key = &kb[PROD[p]..PROD[p] + 5];
                if cached[p].0 != key {
                    cached[p] = (key.to_vec(), weights(kb, p));
                }
                for c in 0..n * m {
                    if d[c] > 0.0 {
                        hh[c] += cached[p].1[c] * dp;
                    }
                }
            }
        }
        if kb[STEEP] != ka[STEEP] {
            for c in 0..n * m {
                if d[c] > 0.0 {
                    hh[c] += kb[STRENGTH] * (kb[STEEP] - ka[STEEP]) * g[c];
                }
            }
        }
        for c in 0..n * m {
            if d[c] <= 0.0 {
                hh[c] = 0.0;
                uu[c] = 0.0;
            }
        }
    };
    // The pre-roll, on sample 0, its time before the in point.
    let pre_frames = (k0[PRE] * r).ceil() as usize;
    if pre_frames > 0 {
        let q0 = steps_for(&[k0]);
        let total = pre_frames * q0;
        let big_t = |p: usize, s: f64| tau * k0[PROD[p] + 6] * s;
        for s in 0..total {
            let sa = (s as f64 - total as f64) / (r * q0 as f64);
            let sb = (s as f64 + 1.0 - total as f64) / (r * q0 as f64);
            one(k0, k0, [big_t(0, sa), big_t(1, sa)], [big_t(0, sb), big_t(1, sb)], 1.0 / (r * q0 as f64));
        }
    }
    // The frame intervals; the frequency's running sum at each sample.
    let count = samples.len() - 1;
    let mut tsum = [vec![0.0], vec![0.0]];
    for (p, sum) in tsum.iter_mut().enumerate() {
        let f = PROD[p] + 6;
        for j in 0..count {
            let next = sum[j] + tau / r * (samples[j][f] + samples[j + 1][f]) / 2.0;
            sum.push(next);
        }
    }
    let big_t = |p: usize, j: usize, f: f64| {
        if f == 0.0 {
            return tsum[p][j];
        }
        let (fa, fb) = (samples[j][PROD[p] + 6], samples[j + 1][PROD[p] + 6]);
        tsum[p][j] + tau / r * (fa * f + (fb - fa) * f * f / 2.0)
    };
    let lerp = |a: &[f64], b: &[f64], f: f64| -> Vec<f64> { a.iter().zip(b).map(|(x, y)| x + (y - x) * f).collect() };
    for j in 0..count {
        let (sa, sb) = (samples[j], samples[j + 1]);
        let q = steps_for(&[sa, sb]);
        for i in 0..q {
            let (fa, fb) = (i as f64 / q as f64, (i + 1) as f64 / q as f64);
            let ka = lerp(sa, sb, fa);
            let last = i + 1 == q;
            let kb = if last { sb.to_vec() } else { lerp(sa, sb, fb) };
            let ta = [big_t(0, j, fa), big_t(1, j, fa)];
            let tb = if last { [tsum[0][j + 1], tsum[1][j + 1]] } else { [big_t(0, j, fb), big_t(1, j, fb)] };
            one(&ka, &kb, ta, tb, 1.0 / (r * q as f64));
        }
    }

    // Step 7: the map, this frame's settings, over the drawing in its place.
    let d: Vec<f64> = g.iter().map(|gi| kn[HEIGHT] - kn[STEEP] * gi).collect();
    let bilerp = |v: &[f64], gx: f64, gy: f64| {
        let (gx, gy) = (gx.clamp(0.0, (n - 1) as f64), gy.clamp(0.0, (m - 1) as f64));
        let (i0, j0) = (gx.floor() as usize, gy.floor() as usize);
        let (i1, j1) = ((i0 + 1).min(n - 1), (j0 + 1).min(m - 1));
        let (fx, fy) = (gx - i0 as f64, gy - j0 as f64);
        (v[j0 * n + i0] * (1.0 - fx) + v[j0 * n + i1] * fx) * (1.0 - fy) + (v[j1 * n + i0] * (1.0 - fx) + v[j1 * n + i1] * fx) * fy
    };
    let transparent = render_dry_areas == "transparent";
    let mut out = WorkingBuffer::transparent(w, h);
    out.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as isize - ox as isize, (i / w) as isize - oy as isize);
        if x < 0 || y < 0 || x >= lw as isize || y >= lh as isize {
            return;
        }
        let gx = (x as f64 + 0.5) * n as f64 / lw - 0.5;
        let gy = (y as f64 + 0.5) * m as f64 / lh - 0.5;
        let dd = bilerp(&d, gx, gy);
        let mut hv = bilerp(&hh, gx, gy);
        if dd <= 0.0 {
            if transparent {
                return;
            }
            hv = 0.0;
        }
        let v = (kn[BRIGHT] + kn[CONTRAST] * hv).clamp(0.0, 1.0);
        let grey = v.powf(1.0 / kn[GAMMA]);
        let a = if kn[HEIGHT] > 0.0 {
            (1.0 - kn[CLEAR] * (1.0 - dd.max(0.0) / kn[HEIGHT])).clamp(0.0, 1.0)
        } else {
            (1.0 - kn[CLEAR]).clamp(0.0, 1.0)
        };
        let lin = crate::grade::to_linear(grey) * a;
        px.copy_from_slice(&[lin as f32, lin as f32, lin as f32, a as f32]);
    });
    *source = out;
}
