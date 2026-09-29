//! D-203's Median and Smart Blur: document 21's rules, on a layer's own pixels.
//!
//! This program's own methods, modelled on After Effects' Median and Smart Blur; nothing is
//! ported. `tools/median_smart_blur_reference.py` is the same rules worked a second way, and
//! `tests/b138_median_smart_blur.rs` holds these to its numbers.

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
