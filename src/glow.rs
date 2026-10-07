//! D-89's glow: document 21's rule, on a layer's own pixels.
//!
//! This program's own method, modelled on After Effects' Glow and built from document 21's
//! Gaussian blur and D-88's colour test; nothing is ported. `tools/glow_reference.py` is the same
//! rule worked a second way, and `tests/b33_glow.rs` holds this to its numbers.

use crate::color::srgb_to_linear;
use crate::render::Glow;
use crate::selective_blur::parse_hex;
use crate::WorkingBuffer;
use rayon::prelude::*;

/// B-51: a Glow's settings read once, for the CPU here and for the card (`render::OnCard`).
/// Every number is already inside its range and every colour already parses.
#[allow(clippy::too_many_arguments)]
pub(crate) fn settings(
    based_on: &str,
    threshold: f64,
    colors: &[String],
    tolerance: f64,
    radius: f64,
    intensity: f64,
    operation: &str,
    tint: &str,
    units: &str,
) -> Glow {
    let mut targets = [[0; 3]; 8];
    let chosen = crate::selective_blur::targets(colors);
    targets[..chosen.len()].copy_from_slice(&chosen);
    Glow {
        bright: based_on == "bright",
        threshold,
        targets,
        count: chosen.len(),
        tolerance,
        radius,
        intensity,
        screen: operation != "add",
        tint: parse_hex(tint),
        after_effects: units == "after_effects",
        display: false,
    }
}

/// (1) Whether a premultiplied pixel glows. A pixel that does not show never glows. The test is
/// on its 8-bit encoded colour, as a drawing program stores it: bright parts by its brightest
/// channel, as Bloom's, and chosen colours by D-88.
pub(crate) fn glows(px: &[f32], g: &Glow) -> bool {
    if g.bright {
        return crate::bloom::bright(px, g.threshold);
    }
    crate::selective_blur::chosen(px, &g.targets[..g.count], g.tolerance)
}

/// Lay the glow of `source` on top of it, in place, and return how far it grew on each side.
/// Intensity 0, or no pixel that glows, changes nothing.
pub(crate) fn glow(source: &mut WorkingBuffer, g: &Glow) -> usize {
    let (w, h) = (source.width(), source.height());
    if g.intensity == 0.0 || w == 0 || h == 0 {
        return 0;
    }
    // D-337: in display values the tint is used as written, and what glows is still tested on
    // the pixel's own linear value.
    let tint = g.tint.map(|t| t.map(|v| if g.display { v as f32 / 255.0 } else { srgb_to_linear(v as f32 / 255.0) }));
    let lights = |px: &[f32]| {
        if g.display && px[3] > 0.0 {
            let a = px[3];
            glows(&[srgb_to_linear(px[0] / a) * a, srgb_to_linear(px[1] / a) * a, srgb_to_linear(px[2] / a) * a, a], g)
        } else {
            glows(px, g)
        }
    };

    // (2) The light a glowing pixel gives: the pixel itself, or the tint at its covering.
    let mut light = WorkingBuffer::transparent(w, h);
    light
        .data_mut()
        .par_chunks_exact_mut(4)
        .zip(source.data().par_chunks_exact(4))
        .for_each(|(l, px)| {
            if lights(px) {
                let a = px[3];
                match tint {
                    Some(t) => l.copy_from_slice(&[t[0] * a, t[1] * a, t[2] * a, a]),
                    None => l.copy_from_slice(px),
                }
            }
        });
    if light.data().chunks_exact(4).all(|l| l[3] == 0.0) {
        return 0;
    }

    // (3) Spread: document 21's blur at sigma radius / 3, growing the light's bounds; D-322's
    // After Effects units blur as Gaussian Blur at Blurriness radius.
    let (sigma, long) = crate::effects::glow_reach(g.radius, if g.after_effects { "after_effects" } else { "classic" });
    let r = crate::effects::blur_axes(&mut light, &crate::effects::reach_weights(sigma, long), (true, true));

    // (4) Strength and (5) on top of the picture, which is empty outside its own bounds.
    // D-331: Creative COW's GI*(GT/100) + GI*16*(1-GT/100) is the colour read straight: the
    // colour at the intensity, the covering divided by t + 16 (1 - t). D-337: in display values,
    // where Creative COW measured it, the colour is multiplied by all of it, the covering untouched.
    let (k, ka) = if g.after_effects {
        let t = g.threshold / 100.0;
        let c = t + 16.0 * (1.0 - t);
        if g.display {
            ((g.intensity * c) as f32, 1.0)
        } else {
            (g.intensity as f32, (1.0 / c) as f32)
        }
    } else {
        (g.intensity as f32, g.intensity as f32)
    };
    let add = !g.screen;
    let lw = light.width();
    let o = source.data();
    light
        .data_mut()
        .par_chunks_exact_mut(lw * 4)
        .enumerate()
        .for_each(|(y, row)| {
            for (x, l) in row.chunks_exact_mut(4).enumerate() {
                let (sx, sy) = (x as isize - r as isize, y as isize - r as isize);
                let p = if (0..w as isize).contains(&sx) && (0..h as isize).contains(&sy) {
                    let i = (sy as usize * w + sx as usize) * 4;
                    [o[i], o[i + 1], o[i + 2], o[i + 3]]
                } else {
                    [0.0; 4]
                };
                if add {
                    for c in 0..3 {
                        l[c] = p[c] + l[c] * k;
                    }
                    l[3] = (p[3] + l[3] * ka).min(1.0);
                } else {
                    for c in 0..4 {
                        let v = (l[c] * if c == 3 { ka } else { k }).clamp(0.0, 1.0);
                        l[c] = p[c] + v - p[c] * v;
                    }
                }
            }
        });
    *source = light;
    r
}
