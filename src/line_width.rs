//! D-94's line width: document 21's rule, on a layer's own pixels.
//!
//! This program's own method; nothing is ported. `tools/line_width_reference.py` is the same
//! rule worked a second way, and `tests/b38_line_width.rs` holds this to its numbers.

use crate::selective_blur::{chosen, targets};
use crate::WorkingBuffer;
use rayon::prelude::*;

/// Every offset within `width`, nearest first, then the one above, then the one to the left:
/// the order in which equals are decided.
fn disc(width: f64) -> Vec<(i64, i64)> {
    let r = width.abs().floor() as i64;
    let mut d: Vec<(i64, i64)> = (-r..=r)
        .flat_map(|dy| (-r..=r).map(move |dx| (dx, dy)))
        .filter(|&(dx, dy)| ((dx * dx + dy * dy) as f64) <= width * width)
        .collect();
    d.sort_by_key(|&(dx, dy)| (dx * dx + dy * dy, dy, dx));
    d
}

/// Thicken or thin `source` by `width` pixels, in place, and return how far it grew on each
/// side. The settings are already valid; width 0, or colours with none chosen, changes nothing.
pub(crate) fn line_width(
    source: &mut WorkingBuffer,
    width: f64,
    shape: bool,
    colors: &[String],
    tolerance: f64,
) -> usize {
    let targets = targets(colors);
    let grow = if width > 0.0 {
        width.ceil() as usize
    } else {
        0
    };
    if width == 0.0 {
        return 0;
    }
    let disc = disc(width);
    let (w0, h0) = (source.width() as i64, source.height() as i64);
    let src = source.data();
    let at = |x: i64, y: i64| -> [f32; 4] {
        if (0..w0).contains(&x) && (0..h0).contains(&y) {
            let i = ((y * w0 + x) * 4) as usize;
            [src[i], src[i + 1], src[i + 2], src[i + 3]]
        } else {
            [0.0; 4]
        }
    };
    // D-88's choice, once a pixel; outside the layer is transparent and so never chosen.
    let picked: Vec<bool> = if shape {
        Vec::new()
    } else {
        src.par_chunks_exact(4)
            .map(|px| chosen(px, &targets, tolerance))
            .collect()
    };
    let is_picked = |x: i64, y: i64| {
        (0..w0).contains(&x) && (0..h0).contains(&y) && picked[(y * w0 + x) as usize]
    };
    let g = grow as i64;
    let w = (w0 + 2 * g) as usize;
    let mut out = WorkingBuffer::transparent(w, (h0 + 2 * g) as usize);
    // ponytail: a whole disc a pixel, ~1250 looks at width 20; the search stops at the first
    // answer it can, and a distance transform is the upgrade if P-17 finds it matters.
    out.data_mut()
        .par_chunks_exact_mut(w * 4)
        .enumerate()
        .for_each(|(y, row)| {
            let ly = y as i64 - g;
            for (x, px) in row.chunks_exact_mut(4).enumerate() {
                let lx = x as i64 - g;
                let own = at(lx, ly);
                let value = if shape && width > 0.0 {
                    let mut best = own;
                    for &(dx, dy) in &disc {
                        if best[3] >= 1.0 {
                            break;
                        }
                        let p = at(lx + dx, ly + dy);
                        if p[3] > best[3] {
                            best = p;
                        }
                    }
                    best
                } else if shape {
                    let mut least = own[3];
                    for &(dx, dy) in &disc {
                        if least <= 0.0 {
                            break;
                        }
                        least = least.min(at(lx + dx, ly + dy)[3]);
                    }
                    if own[3] > 0.0 {
                        let k = least / own[3];
                        [own[0] * k, own[1] * k, own[2] * k, least]
                    } else {
                        own
                    }
                } else {
                    let wanted = width > 0.0;
                    if targets.is_empty() || is_picked(lx, ly) == wanted {
                        own
                    } else {
                        disc.iter()
                            .find(|&&(dx, dy)| is_picked(lx + dx, ly + dy) == wanted)
                            .map_or(own, |&(dx, dy)| at(lx + dx, ly + dy))
                    }
                };
                px.copy_from_slice(&value);
            }
        });
    *source = out;
    grow
}
