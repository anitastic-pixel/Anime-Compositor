//! D-96's bloom: document 21's rule, on a layer's own pixels.
//!
//! This program's own method, built from D-89's bright test, document 21's Gaussian blur and
//! D-92's line samples; nothing is ported. `tools/bloom_reference.py` is the same rule worked a
//! second way, and `tests/b40_bloom.rs` holds this to its numbers.

use crate::color::{linear_to_srgb, quantise_u8};
use crate::effects::{blur, kernel_radius};
use crate::render::sample_bilinear;
use crate::WorkingBuffer;
use rayon::prelude::*;

/// The halo's four blurs, each half as wide as the one before.
const SCALES: [f64; 4] = [1.0, 0.5, 0.25, 0.125];

/// How many streak lines the word asks for: none, a cross of two or a star of four.
pub(crate) fn lines(streaks: &str) -> usize {
    match streaks {
        "cross" => 2,
        "star" => 4,
        _ => 0,
    }
}

/// How far a bloom reaches past the drawing on each side: the widest blur's reach, or the
/// streaks' length if that is more.
pub(crate) fn reach(radius: f64, lines: usize, length: f64) -> usize {
    let streak = if lines > 0 { length.ceil() as usize } else { 0 };
    kernel_radius(radius / 3.0).max(streak)
}

/// Lay the bloom of `source` on top of it, in place, and return how far it grew on each side.
/// Every setting is already inside its range; intensity 0, or no pixel bright enough, changes
/// nothing.
pub(crate) fn bloom(
    source: &mut WorkingBuffer,
    threshold: f64,
    radius: f64,
    intensity: f64,
    lines: usize,
    length: f64,
    angle: f64,
) -> usize {
    let (w, h) = (source.width(), source.height());
    if intensity == 0.0 || w == 0 || h == 0 {
        return 0;
    }

    // (1) The light: D-89's bright test on the 8-bit encoded colour, the pixel as it is.
    let mut light = WorkingBuffer::transparent(w, h);
    light
        .data_mut()
        .par_chunks_exact_mut(4)
        .zip(source.data().par_chunks_exact(4))
        .for_each(|(g, px)| {
            let a = px[3];
            if a > 0.0 {
                let q = [0, 1, 2].map(|i| quantise_u8(linear_to_srgb(px[i] / a)));
                if 100.0 * *q.iter().max().unwrap() as f64 >= 255.0 * threshold {
                    g.copy_from_slice(px);
                }
            }
        });
    if light.data().chunks_exact(4).all(|g| g[3] == 0.0) {
        return 0;
    }
    let grow = reach(radius, lines, length);
    let gw = w + 2 * grow;

    // (2) The halo: the average of the four blurs, each placed by its own reach.
    let mut halo = WorkingBuffer::transparent(gw, h + 2 * grow);
    for s in SCALES {
        let mut b = light.clone();
        let r = blur(&mut b, radius / 3.0 * s);
        let off = grow - r;
        let bw = b.width() * 4;
        // P-17: row by row in parallel; each pixel still adds the four in the same order.
        halo.data_mut()
            .par_chunks_exact_mut(gw * 4)
            .skip(off)
            .zip(b.data().par_chunks_exact(bw))
            .for_each(|(hrow, row)| {
                for (d, v) in hrow[off * 4..off * 4 + bw].iter_mut().zip(row) {
                    *d += v * 0.25;
                }
            });
    }

    // (3) The streaks: each line's tent of samples, m - |k - m| for k = 0 to 2m, and every
    // line's offsets in one list, so a pixel sums them all and divides once.
    let m = if lines > 0 { length.ceil() as usize } else { 0 };
    let steps: Vec<(f64, f64, f64)> = (0..lines)
        .flat_map(|j| {
            let u = crate::blurs::along(angle + j as f64 * 180.0 / lines as f64);
            (0..=2 * m).map(move |k| {
                if m == 0 {
                    return (0.0, 0.0, 1.0);
                }
                let t = (k as f64 - m as f64) * length / m as f64;
                (t * u.0, t * u.1, (m - k.abs_diff(m)) as f64)
            })
        })
        .collect();
    let total = (m * m).max(1) as f64 * lines as f64;

    // (4) Added on top of the picture, which is empty outside its own bounds.
    let k = intensity as f32;
    let o = source.data();
    let (halo, light) = (&halo, &light);
    let spans = if steps.is_empty() {
        Vec::new()
    } else {
        crate::blurs::spans(light)
    };
    let mut out = WorkingBuffer::transparent(gw, h + 2 * grow);
    // P-17: the streaks a step at a time along the whole row, and only where light is; each
    // pixel still adds the same samples in the same order, so no bit moves. A tent's two end
    // samples weigh nothing and add exactly nothing, so they are left out too.
    // ponytail: still each line's 2m + 1 samples a pixel where light reaches, ~0.5 s for a
    // fully lit 1080p star at length 60; a running sum along each line is the upgrade.
    out.data_mut()
        .par_chunks_exact_mut(gw * 4)
        .enumerate()
        .for_each(|(y, row)| {
            let cy = y as f64 - grow as f64 + 0.5;
            let mut sums = vec![[0.0f64; 4]; if steps.is_empty() { 0 } else { gw }];
            for &(dx, dy, wt) in &steps {
                if wt == 0.0 {
                    continue;
                }
                for x in crate::blurs::reached(&spans, cy + dy, dx, grow, gw) {
                    let cx = x as f64 - grow as f64 + 0.5;
                    let s = sample_bilinear(light, cx + dx, cy + dy);
                    for c in 0..4 {
                        sums[x][c] += s[c] as f64 * wt;
                    }
                }
            }
            for (x, px) in row.chunks_exact_mut(4).enumerate() {
                let i = (y * gw + x) * 4;
                let mut g = [0, 1, 2, 3].map(|c| halo.data()[i + c]);
                if let Some(sum) = sums.get(x) {
                    for c in 0..4 {
                        g[c] += (sum[c] / total) as f32;
                    }
                }
                let (sx, sy) = (x as isize - grow as isize, y as isize - grow as isize);
                let p = if (0..w as isize).contains(&sx) && (0..h as isize).contains(&sy) {
                    let j = (sy as usize * w + sx as usize) * 4;
                    [o[j], o[j + 1], o[j + 2], o[j + 3]]
                } else {
                    [0.0; 4]
                };
                for c in 0..3 {
                    px[c] = p[c] + g[c] * k;
                }
                px[3] = (p[3] + g[3] * k).min(1.0);
            }
        });
    *source = out;
    grow
}
