//! D-96's bloom: document 21's rule, on a layer's own pixels.
//!
//! This program's own method, built from D-89's bright test, document 21's Gaussian blur and
//! D-98's lines; nothing is ported. `tools/bloom_reference.py` is the same rule worked a
//! second way, and `tests/b40_bloom.rs` holds this to its numbers.

use crate::blurs::{along, by_lines, Weights};
use crate::color::{linear_to_srgb, quantise_u8};
use crate::effects::{blur, kernel_radius};
use crate::WorkingBuffer;
use rayon::prelude::*;

/// The halo's four blurs, each half as wide as the one before.
pub(crate) const SCALES: [f64; 4] = [1.0, 0.5, 0.25, 0.125];

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

/// One streak's tent along step `u`, `length` long: D-98's weights for [`by_lines`].
pub(crate) fn streak_weights(u: (f64, f64), length: f64) -> Weights {
    let top = u.0.abs().max(u.1.abs()) * length;
    let inner = top.ceil() as usize - 1;
    let n = inner as f64;
    let total = (2.0 * n + 1.0) * top - n * (n + 1.0);
    Weights {
        inner,
        a: top / total,
        b: 1.0 / total,
        ends: Vec::new(),
    }
}

/// D-89's bright test on one premultiplied pixel: its 8-bit encoded colour's largest channel at
/// or over `threshold` percent.
pub(crate) fn bright(px: &[f32], threshold: f64) -> bool {
    let a = px[3];
    a > 0.0 && {
        let q = [0, 1, 2].map(|i| quantise_u8(linear_to_srgb(px[i] / a)));
        100.0 * *q.iter().max().unwrap() as f64 >= 255.0 * threshold
    }
}

/// B-47: the lowest 8-bit level [`bright`] lets through, 256 for none, so the card compares
/// whole numbers as the CPU does.
pub(crate) fn bright_level(threshold: f64) -> u32 {
    (0..256).find(|&q| 100.0 * q as f64 >= 255.0 * threshold).unwrap_or(256)
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
            if bright(px, threshold) {
                g.copy_from_slice(px);
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

    // (3) The streaks, D-98: each line a tent over whole columns (rows, mostly down),
    // max(0, h - |j|) with h = max(|u_x|, |u_y|) * length, the lines averaged into the halo.
    // Length 0 is the light itself, whatever the lines.
    for j in 0..lines {
        let u = along(angle + j as f64 * 180.0 / lines as f64);
        let streak = (length != 0.0).then(|| by_lines(&light, u, &streak_weights(u, length), grow));
        let share = 1.0 / lines as f32;
        halo.data_mut()
            .par_chunks_exact_mut(gw * 4)
            .enumerate()
            .for_each(|(y, hrow)| match &streak {
                Some(t) => {
                    for (d, v) in hrow.iter_mut().zip(&t.data()[y * gw * 4..(y + 1) * gw * 4]) {
                        *d += v * share;
                    }
                }
                None if (grow..grow + h).contains(&y) => {
                    let row = &light.data()[(y - grow) * w * 4..(y - grow + 1) * w * 4];
                    for (d, v) in hrow[grow * 4..(grow + w) * 4].iter_mut().zip(row) {
                        *d += v * share;
                    }
                }
                None => {}
            });
    }

    // (4) Added on top of the picture, which is empty outside its own bounds.
    let k = intensity as f32;
    let o = source.data();
    let halo = &halo;
    let mut out = WorkingBuffer::transparent(gw, h + 2 * grow);
    out.data_mut()
        .par_chunks_exact_mut(gw * 4)
        .enumerate()
        .for_each(|(y, row)| {
            for (x, px) in row.chunks_exact_mut(4).enumerate() {
                let i = (y * gw + x) * 4;
                let g = [0, 1, 2, 3].map(|c| halo.data()[i + c]);
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
