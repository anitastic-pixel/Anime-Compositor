//! The cel colour effects, each a pixel at a time on a layer's own pixels: D-91's line recolour,
//! D-93's select colour and D-97's colour key.
//!
//! This program's own methods; nothing is ported. `tools/recolor_reference.py`,
//! `tools/select_color_reference.py` and `tools/color_key_reference.py` are the same rules
//! worked a second way, and `tests/b35_line_recolor.rs`, `tests/b37_select_color.rs` and
//! `tests/b41_color_key.rs` hold these to their numbers.

use crate::color::{linear_to_srgb, quantise_u8, srgb_to_linear};
use crate::selective_blur::{chosen, parse_hex, targets};
use crate::WorkingBuffer;
use rayon::prelude::*;

/// D-91: every chosen pixel becomes `new_color` at its own covering. The settings are already
/// valid; no colour chosen changes nothing.
pub(crate) fn line_recolor(
    source: &mut WorkingBuffer,
    colors: &[String],
    tolerance: f64,
    new_color: &str,
) {
    let targets = targets(colors);
    let Some(new) = parse_hex(new_color) else {
        return;
    };
    if targets.is_empty() {
        return;
    }
    let new = new.map(|v| srgb_to_linear(v as f32 / 255.0));
    source.data_mut().par_chunks_exact_mut(4).for_each(|px| {
        if chosen(px, &targets, tolerance) {
            let a = px[3];
            px[..3].copy_from_slice(&[new[0] * a, new[1] * a, new[2] * a]);
        }
    });
}

/// D-93: a pixel is kept when whether it is chosen matches `keep_chosen`, and made transparent
/// when not. The settings are already valid; no colour chosen changes nothing.
pub(crate) fn select_color(
    source: &mut WorkingBuffer,
    colors: &[String],
    tolerance: f64,
    keep_chosen: bool,
) {
    let targets = targets(colors);
    if targets.is_empty() {
        return;
    }
    source.data_mut().par_chunks_exact_mut(4).for_each(|px| {
        if chosen(px, &targets, tolerance) != keep_chosen {
            px.fill(0.0);
        }
    });
}

/// The hue of 8-bit red, green and blue in degrees, 0 to 360, or `None` for a grey. Of equal
/// largest channels, red counts before green and green before blue.
fn hue(q: [u8; 3]) -> Option<f64> {
    let [r, g, b] = q.map(f64::from);
    let (hi, lo) = (r.max(g).max(b), r.min(g).min(b));
    let c = hi - lo;
    if c == 0.0 {
        return None;
    }
    Some(if hi == r {
        60.0 * ((g - b) / c).rem_euclid(6.0)
    } else if hi == g {
        60.0 * ((b - r) / c + 2.0)
    } else {
        60.0 * ((r - g) / c + 4.0)
    })
}

/// D-97: each pixel that shows keeps the share of itself, colour and covering alike, that its
/// distance from the nearest chosen colour gives: none at or within `tolerance`, all of it past
/// `softness` more. The settings are already valid; no colour chosen changes nothing.
pub(crate) fn color_key(
    source: &mut WorkingBuffer,
    colors: &[String],
    tolerance: f64,
    softness: f64,
    by_hue: bool,
) {
    let targets = targets(colors);
    if targets.is_empty() {
        return;
    }
    let hues: Vec<Option<f64>> = targets.iter().map(|t| hue(*t)).collect();
    source.data_mut().par_chunks_exact_mut(4).for_each(|px| {
        let a = px[3];
        if a <= 0.0 {
            return;
        }
        let q = [0, 1, 2].map(|i| quantise_u8(linear_to_srgb(px[i] / a)));
        let d = if by_hue {
            let h = hue(q);
            hues.iter()
                .map(|t| match (h, t) {
                    (Some(h), Some(t)) => {
                        let d = (h - t).abs();
                        d.min(360.0 - d) * 255.0 / 180.0
                    }
                    _ => 255.0,
                })
                .fold(f64::INFINITY, f64::min)
        } else {
            targets
                .iter()
                .map(|t| (0..3).map(|i| q[i].abs_diff(t[i])).max().unwrap_or(0) as f64)
                .fold(f64::INFINITY, f64::min)
        };
        let keep = if d <= tolerance {
            0.0
        } else if softness == 0.0 {
            1.0
        } else {
            ((d - tolerance) / softness).min(1.0)
        };
        if keep < 1.0 {
            px.iter_mut().for_each(|v| *v *= keep as f32);
        }
    });
}
