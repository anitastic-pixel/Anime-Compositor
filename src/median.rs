//! D-203's Median and Smart Blur, and D-358's Bilateral Blur: document 21's rules, on a layer's
//! own pixels.
//!
//! This program's own methods, modelled on After Effects' Median, Smart Blur and Bilateral Blur;
//! nothing is ported. `tools/median_smart_blur_reference.py` and `tools/bilateral_blur_reference.py`
//! are the same rules worked a second way, and `tests/b138_median_smart_blur.rs` and
//! `tests/b237_bilateral_blur.rs` hold these to their numbers.

use crate::color::{linear_to_srgb, quantise_u8};
use crate::WorkingBuffer;
use rayon::prelude::*;

/// The offsets within `radius` of a pixel, its own among them.
fn disc(radius: f64) -> Vec<(isize, isize)> {
    let r = radius.floor() as isize;
    let mut taps = Vec::new();
    for dy in -r..=r {
        for dx in -r..=r {
            if ((dx * dx + dy * dy) as f64) <= radius * radius {
                taps.push((dx, dy));
            }
        }
    }
    taps
}

/// The middle of `v`, or the mean of the middle two when there are an even number; `v` is
/// reordered and never empty.
fn middle(v: &mut [f32]) -> f32 {
    let n = v.len();
    let (lower, m, _) = v.select_nth_unstable_by(n / 2, f32::total_cmp);
    let m = *m;
    if n % 2 == 1 {
        m
    } else {
        (lower.iter().copied().fold(f32::MIN, f32::max) + m) / 2.0
    }
}

/// D-203: each pixel's straight colour the median of the taps within `radius` that show, one
/// channel at a time, at its own covering, or with `operate_on_alpha` at the median of every
/// tap's covering. The buffer keeps its size; the settings are already valid.
///
/// D-453's Dust & Scratches with `threshold` 0 or more: a channel, or with `operate_on_alpha` the
/// covering, whose 8-bit value (D-88's, colours through the sRGB curve) is within `threshold` of
/// the median's keeps its own. Median is a `threshold` of -1, nothing kept.
// ponytail: every pixel sorts its disc afresh, about 300 taps at radius 10; a sliding histogram
// would be the upgrade if a larger radius is ever wanted.
pub(crate) fn median(source: &mut WorkingBuffer, radius: f64, threshold: f64, operate_on_alpha: bool) {
    let taps = disc(radius);
    if taps.len() == 1 {
        return;
    }
    let (w, h) = (source.width(), source.height());
    let straight: Vec<[f32; 4]> = source
        .data()
        .par_chunks_exact(4)
        .map(|p| if p[3] > 0.0 { [p[0] / p[3], p[1] / p[3], p[2] / p[3], p[3]] } else { [0.0; 4] })
        .collect();
    let mut out = WorkingBuffer::transparent(w, h);
    out.data_mut()
        .par_chunks_exact_mut(4 * w)
        .enumerate()
        .for_each(|(y, row)| {
            let mut v: [Vec<f32>; 4] = Default::default();
            for (x, px) in row.chunks_exact_mut(4).enumerate() {
                // A disc all of one colour and covering, as flat paint is, is its own median:
                // the same numbers the sort would give, for far less.
                let own = straight[y * w + x];
                let flat = taps.iter().all(|&(dx, dy)| {
                    let (tx, ty) = (x as isize + dx, y as isize + dy);
                    tx >= 0 && ty >= 0 && tx < w as isize && ty < h as isize && straight[ty as usize * w + tx as usize] == own
                });
                if flat {
                    if own[3] > 0.0 {
                        (0..3).for_each(|i| px[i] = own[i] * own[3]);
                        px[3] = own[3];
                    }
                    continue;
                }
                v.iter_mut().for_each(Vec::clear);
                for &(dx, dy) in &taps {
                    let (tx, ty) = (x as isize + dx, y as isize + dy);
                    let t = if tx < 0 || ty < 0 || tx >= w as isize || ty >= h as isize {
                        [0.0; 4]
                    } else {
                        straight[ty as usize * w + tx as usize]
                    };
                    v[3].push(t[3]);
                    if t[3] > 0.0 {
                        (0..3).for_each(|i| v[i].push(t[i]));
                    }
                }
                let near = |p: u8, q: u8| (p as f64 - q as f64).abs() <= threshold;
                let mut a = if operate_on_alpha { middle(&mut v[3]) } else { own[3] };
                if operate_on_alpha && near(quantise_u8(a), quantise_u8(own[3])) {
                    a = own[3];
                }
                if a <= 0.0 || v[0].is_empty() {
                    continue;
                }
                for i in 0..3 {
                    let m = middle(&mut v[i]);
                    let level = |c: f32| quantise_u8(linear_to_srgb(c));
                    let c = if own[3] > 0.0 && threshold >= 0.0 && near(level(own[i]), level(m)) { own[i] } else { m };
                    px[i] = c * a;
                }
                px[3] = a;
            }
        });
    *source = out;
}

/// D-203: each pixel that shows the mean of the taps within `radius` that show and whose 8-bit
/// straight colour and covering (D-88's) are each within `threshold` of its own. The buffer
/// keeps its size; the settings are already valid.
pub(crate) fn smart_blur(source: &mut WorkingBuffer, radius: f64, threshold: f64) {
    let taps = disc(radius);
    if taps.len() == 1 {
        return;
    }
    let (w, h) = (source.width(), source.height());
    let levels: Vec<Option<[u8; 4]>> = source
        .data()
        .par_chunks_exact(4)
        .map(|p| {
            let a = p[3];
            (a > 0.0).then(|| {
                let q = |c: f32| quantise_u8(linear_to_srgb(c / a));
                [q(p[0]), q(p[1]), q(p[2]), quantise_u8(a)]
            })
        })
        .collect();
    let src = source.data();
    let mut out = WorkingBuffer::transparent(w, h);
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let Some(own) = levels[i] else { return };
            let (x, y) = ((i % w) as isize, (i / w) as isize);
            let (mut sum, mut n) = ([0.0f64; 4], 0u32);
            for &(dx, dy) in &taps {
                let (tx, ty) = (x + dx, y + dy);
                if tx < 0 || ty < 0 || tx >= w as isize || ty >= h as isize {
                    continue;
                }
                let j = ty as usize * w + tx as usize;
                let Some(t) = levels[j] else { continue };
                if (0..4).all(|k| (t[k] as f64 - own[k] as f64).abs() <= threshold) {
                    (0..4).for_each(|k| sum[k] += src[4 * j + k] as f64);
                    n += 1;
                }
            }
            (0..4).for_each(|k| px[k] = (sum[k] / n as f64) as f32);
        });
    *source = out;
}

/// D-358's Bilateral Blur: each pixel that shows the weighted mean of the taps within `radius`
/// that show, each weighed by a bell on its distance (spread `radius / 2`), its covering, and a
/// bell on how far its value is from the pixel's own in sRGB levels (spread `threshold`). With
/// `colorize` red, green and blue are worked one at a time; without it the luminance alone, and
/// the pixel turns grey. Every pixel keeps its covering; the settings are already valid.
// ponytail: every pixel weighs its whole disc, about 7,850 taps at Radius 50; a grid or
// separable approximation would be the upgrade, at the price of the rule.
pub(crate) fn bilateral_blur(source: &mut WorkingBuffer, radius: f64, threshold: f64, colorize: bool) {
    let taps = disc(radius);
    let still = taps.len() == 1 || threshold <= 0.0;
    if colorize && still {
        return;
    }
    let (w, h) = (source.width(), source.height());
    let n = if colorize { 3 } else { 1 };
    let level = |v: f64| 255.0 * if v <= 0.0031308 { 12.92 * v } else { 1.055 * v.powf(1.0 / 2.4) - 0.055 };
    // Each pixel's worked values in straight linear light, and their levels; None where it does
    // not show.
    let vals: Vec<Option<([f64; 3], [f64; 3])>> = source
        .data()
        .par_chunks_exact(4)
        .map(|p| {
            let a = p[3] as f64;
            (a > 0.0).then(|| {
                let c = [p[0] as f64 / a, p[1] as f64 / a, p[2] as f64 / a];
                let v = if colorize { c } else { [0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2]; 3] };
                (v, v.map(level))
            })
        })
        .collect();
    let s = radius / 2.0;
    let near: Vec<f64> = if still {
        Vec::new()
    } else {
        taps.iter().map(|&(dx, dy)| (-((dx * dx + dy * dy) as f64) / (2.0 * s * s)).exp()).collect()
    };
    let spread = 1.0 / (2.0 * threshold * threshold);
    let src = source.data();
    let mut out = WorkingBuffer::transparent(w, h);
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let Some((own, own_level)) = vals[i] else { return };
            let a = src[4 * i + 3];
            let mut v = own;
            if !still {
                let (x, y) = ((i % w) as isize, (i / w) as isize);
                let (mut sum, mut weight) = ([0.0f64; 3], [0.0f64; 3]);
                for (&(dx, dy), &g) in taps.iter().zip(&near) {
                    let (tx, ty) = (x + dx, y + dy);
                    if tx < 0 || ty < 0 || tx >= w as isize || ty >= h as isize {
                        continue;
                    }
                    let j = ty as usize * w + tx as usize;
                    let Some((t, t_level)) = vals[j] else { continue };
                    let ga = g * src[4 * j + 3] as f64;
                    for k in 0..n {
                        let d = t_level[k] - own_level[k];
                        let wt = ga * (-d * d * spread).exp();
                        sum[k] += wt * t[k];
                        weight[k] += wt;
                    }
                }
                (0..n).for_each(|k| v[k] = sum[k] / weight[k]);
                if !colorize {
                    v = [v[0]; 3];
                }
            }
            (0..3).for_each(|k| px[k] = (v[k] * a as f64) as f32);
            px[3] = a;
        });
    *source = out;
}

/// D-448: Cartoon's numbers, as the processor and the card share them: Edge Threshold, the edge's
/// ramp (10.01 - 10 Edge Contrast), Edge Width, the line's half softness (0.5 + Softness / 100 x
/// Width), Edge Opacity out of 1, the shading steps less one, Shading Smoothness out of 1, Edge
/// Enhancement out of 1, Edge Black Level, the render's number in [`crate::effects::CARTOON_RENDERS`]
/// and how far a line can reach.
pub(crate) fn cartoon_k(e: &crate::effects::Effect) -> Option<[f64; 11]> {
    let crate::effects::Effect::Cartoon {
        render,
        shading_steps,
        shading_smoothness,
        edge_threshold,
        edge_width,
        edge_softness,
        edge_opacity,
        edge_enhancement,
        edge_black_level,
        edge_contrast,
        ..
    } = e
    else {
        return None;
    };
    let soft = 0.5 + edge_softness / 100.0 * edge_width;
    Some([
        *edge_threshold,
        10.01 - 10.0 * edge_contrast,
        *edge_width,
        soft,
        edge_opacity / 100.0,
        shading_steps.floor() - 1.0,
        shading_smoothness / 100.0,
        edge_enhancement / 100.0,
        *edge_black_level,
        crate::effects::CARTOON_RENDERS.iter().position(|v| v == render).unwrap_or(2) as f64,
        (edge_width + 0.5 + soft).floor() + 1.0,
    ])
}

/// D-448: the fill of a smoothed pixel, encoded; None where it does not show. `n` is the shading
/// steps less one, `s` the smoothness out of 1.
fn cartoon_fill(p: &[f32], n: f64, s: f64) -> Option<[f64; 3]> {
    let a = p[3] as f64;
    if a <= 0.0 {
        return None;
    }
    let c = [0, 1, 2].map(|i| p[i] as f64 / a);
    let l = crate::grade::to_srgb((0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2]).clamp(0.0, 1.0));
    let q = l * n;
    let k = q.floor();
    let t = q - k;
    let ramp = if s == 0.0 { (t >= 0.5) as u8 as f64 } else { ((t - 0.5) / s + 0.5).clamp(0.0, 1.0) };
    let shade = (k + ramp) / n;
    Some(c.map(|v| (crate::grade::to_srgb(v.clamp(0.0, 1.0)) + shade - l).clamp(0.0, 1.0)))
}

/// D-448's Cartoon (document 21): the layer smoothed by [`bilateral_blur`]; each pixel's edge from
/// Sobel on the smoothed picture luma, the border repeated; the line the strongest edge round the
/// pixel, faded by its distance; the fill's shading cut into steps and, with Edge Enhancement,
/// pushed from or towards its neighbours; then laid out by the render. Every pixel keeps its
/// covering; the settings are already valid.
pub(crate) fn cartoon(source: &mut WorkingBuffer, e: &crate::effects::Effect) {
    let crate::effects::Effect::Cartoon { detail_radius, detail_threshold, .. } = e else { return };
    let Some([threshold, ramp, width, soft, opacity, n, s, enhance, black, render, reach]) = cartoon_k(e) else { return };
    bilateral_blur(source, *detail_radius, *detail_threshold, true);
    let (w, h) = (source.width() as i64, source.height() as i64);
    let src = source.data();
    let held = |x: i64, y: i64| (y.clamp(0, h - 1) * w + x.clamp(0, w - 1)) as usize;
    let lumas: Vec<f64> = src.par_chunks_exact(4).map(|p| crate::layer_fx::picture_luma([p[0], p[1], p[2], p[3]])).collect();
    let edges: Vec<f64> = (0..w * h)
        .into_par_iter()
        .map(|i| {
            let (x, y) = (i % w, i / w);
            let at = |x: i64, y: i64| lumas[held(x, y)];
            let gx = at(x + 1, y - 1) + 2.0 * at(x + 1, y) + at(x + 1, y + 1) - at(x - 1, y - 1) - 2.0 * at(x - 1, y) - at(x - 1, y + 1);
            let gy = at(x - 1, y + 1) + 2.0 * at(x, y + 1) + at(x + 1, y + 1) - at(x - 1, y - 1) - 2.0 * at(x, y - 1) - at(x + 1, y - 1);
            ((10.0 * (gx * gx + gy * gy).sqrt() / 2.0 - threshold) / ramp).clamp(0.0, 1.0)
        })
        .collect();
    let fills: Vec<Option<[f64; 3]>> = src.par_chunks_exact(4).map(|p| cartoon_fill(p, n, s)).collect();
    let r = reach as i64;
    let mut out = WorkingBuffer::transparent(w as usize, h as usize);
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let Some(mut f) = fills[i] else { return };
            let a = src[4 * i + 3];
            let (x, y) = (i as i64 % w, i as i64 / w);
            if enhance != 0.0 {
                let (mut m, mut count) = ([0.0f64; 3], 0.0);
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if let Some(g) = fills[held(x + dx, y + dy)] {
                            (0..3).for_each(|c| m[c] += g[c]);
                            count += 1.0;
                        }
                    }
                }
                f = std::array::from_fn(|c| (f[c] + enhance * edges[i] * (f[c] - m[c] / count)).clamp(0.0, 1.0));
            }
            let mut ink = 0.0f64;
            for ty in (y - r).max(0)..=(y + r).min(h - 1) {
                for tx in (x - r).max(0)..=(x + r).min(w - 1) {
                    let (dx, dy) = (tx - x, ty - y);
                    let wt = ((width + 0.5 - ((dx * dx + dy * dy) as f64).sqrt()) / (2.0 * soft) + 0.5).clamp(0.0, 1.0);
                    ink = ink.max(edges[(ty * w + tx) as usize] * wt);
                }
            }
            ink *= opacity;
            let c = match render as u8 {
                0 => f,
                1 => [(1.0 - black) * (1.0 - ink) + black * ink; 3],
                _ => f.map(|v| v * (1.0 - ink) + black * ink),
            };
            (0..3).for_each(|k| px[k] = (crate::grade::to_linear(c[k]) * a as f64) as f32);
            px[3] = a;
        });
    *source = out;
}
