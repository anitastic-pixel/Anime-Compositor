//! The cel colour effects, each a pixel at a time on a layer's own pixels: D-91's line recolour
//! and D-93's select colour.
//!
//! This program's own methods; nothing is ported. `tools/recolor_reference.py` and
//! `tools/select_color_reference.py` are the same rules worked a second way, and
//! `tests/b35_line_recolor.rs` and `tests/b37_select_color.rs` hold these to their numbers.

use crate::color::srgb_to_linear;
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
