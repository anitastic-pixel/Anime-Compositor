//! D-89's glow: document 21's rule, on a layer's own pixels.
//!
//! This program's own method, modelled on After Effects' Glow and built from document 21's
//! Gaussian blur and D-88's colour test; nothing is ported. `tools/glow_reference.py` is the same
//! rule worked a second way, and `tests/b33_glow.rs` holds this to its numbers.

use crate::color::{linear_to_srgb, quantise_u8, srgb_to_linear};
use crate::selective_blur::{matches, parse_hex, targets};
use crate::WorkingBuffer;
use rayon::prelude::*;

/// Lay the glow of `source` on top of it, in place, and return how far it grew on each side.
/// Every number is already inside its range and every colour already parses; intensity 0, or no
/// pixel that glows, changes nothing.
#[allow(clippy::too_many_arguments)]
pub(crate) fn glow(
    source: &mut WorkingBuffer,
    based_on: &str,
    threshold: f64,
    colors: &[String],
    tolerance: f64,
    radius: f64,
    intensity: f64,
    operation: &str,
    tint: &str,
) -> usize {
    let (w, h) = (source.width(), source.height());
    if intensity == 0.0 || w == 0 || h == 0 {
        return 0;
    }
    let targets = targets(colors);
    let tint = parse_hex(tint).map(|t| t.map(|v| srgb_to_linear(v as f32 / 255.0)));

    // (1) What glows and (2) the light it gives: the pixel itself, or the tint at its covering.
    // A pixel that does not show never glows. The test is on its 8-bit encoded colour, as a
    // drawing program stores it: bright parts by its brightest channel, chosen colours by D-88.
    let mut light = WorkingBuffer::transparent(w, h);
    light
        .data_mut()
        .par_chunks_exact_mut(4)
        .zip(source.data().par_chunks_exact(4))
        .for_each(|(g, px)| {
            let a = px[3];
            if a <= 0.0 {
                return;
            }
            let q = [0, 1, 2].map(|i| quantise_u8(linear_to_srgb(px[i] / a)));
            let glows = if based_on == "bright" {
                100.0 * *q.iter().max().unwrap() as f64 >= 255.0 * threshold
            } else {
                matches(q, &targets, tolerance)
            };
            if glows {
                match tint {
                    Some(t) => g.copy_from_slice(&[t[0] * a, t[1] * a, t[2] * a, a]),
                    None => g.copy_from_slice(px),
                }
            }
        });
    if light.data().chunks_exact(4).all(|g| g[3] == 0.0) {
        return 0;
    }

    // (3) Spread: document 21's blur at sigma radius / 3, growing the light's bounds.
    let r = crate::effects::blur(&mut light, radius / 3.0);

    // (4) Strength and (5) on top of the picture, which is empty outside its own bounds.
    let k = intensity as f32;
    let add = operation == "add";
    let lw = light.width();
    let o = source.data();
    light
        .data_mut()
        .par_chunks_exact_mut(lw * 4)
        .enumerate()
        .for_each(|(y, row)| {
            for (x, g) in row.chunks_exact_mut(4).enumerate() {
                let (sx, sy) = (x as isize - r as isize, y as isize - r as isize);
                let p = if (0..w as isize).contains(&sx) && (0..h as isize).contains(&sy) {
                    let i = (sy as usize * w + sx as usize) * 4;
                    [o[i], o[i + 1], o[i + 2], o[i + 3]]
                } else {
                    [0.0; 4]
                };
                if add {
                    for c in 0..3 {
                        g[c] = p[c] + g[c] * k;
                    }
                    g[3] = (p[3] + g[3] * k).min(1.0);
                } else {
                    for c in 0..4 {
                        let v = (g[c] * k).clamp(0.0, 1.0);
                        g[c] = p[c] + v - p[c] * v;
                    }
                }
            }
        });
    *source = light;
    r
}
