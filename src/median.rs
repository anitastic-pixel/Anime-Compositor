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
// ponytail: every pixel sorts its disc afresh, about 300 taps at radius 10; a sliding histogram
// would be the upgrade if a larger radius is ever wanted.
pub(crate) fn median(source: &mut WorkingBuffer, radius: f64, operate_on_alpha: bool) {
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
                let a = if operate_on_alpha { middle(&mut v[3]) } else { straight[y * w + x][3] };
                if a <= 0.0 || v[0].is_empty() {
                    continue;
                }
                for i in 0..3 {
                    px[i] = middle(&mut v[i]) * a;
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
