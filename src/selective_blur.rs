//! D-87's selective colour blur: document 21's rule, on a layer's own pixels.
//!
//! Ported from F's Plugins SelectiveColorBlur, `SelectiveColorBlur/SelectiveColorBlurSub.cpp` in
//! bryful/F-s-PluginsProjects at commit db6dad3959522cfcd593dfe12a9fa4ad022d31e3, under its
//! licence:
//!
//! ```text
//! MIT License
//! Copyright (c) 2019 bryful
//! ```
//!
//! The full text, with its permission notice and its disclaimer, is kept in
//! `docs/third_party/F-s-PluginsProjects-LICENSE.txt` and ships with the build.
//!
//! D-87's two changes from the original are marked where they are made: every pixel keeps its
//! own alpha, and nothing is rounded to 8 bits between passes. `tools/selblur_reference.py` is
//! the same rule worked a second way, and `tests/b31_selective_blur.rs` holds this to its numbers.

use crate::color::{linear_to_srgb, quantise_u8, srgb_to_linear};
use crate::WorkingBuffer;
use rayon::prelude::*;

/// A colour written `#rrggbb`, in either case, as its three 8-bit values.
pub fn parse_hex(s: &str) -> Option<[u8; 3]> {
    let hex = s.strip_prefix('#')?;
    if hex.len() != 6 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let byte = |i: usize| u8::from_str_radix(&hex[i..i + 2], 16).ok();
    Some([byte(0)?, byte(2)?, byte(4)?])
}

/// Blur the chosen colours of `source` into each other, in place. `blur` is already inside
/// 0..200 and every colour already parses; blur 0, or no pixel chosen, changes nothing.
pub(crate) fn selective_color_blur(source: &mut WorkingBuffer, blur: f64, colors: &[String]) {
    let r = (blur + 0.5).floor() as usize;
    let targets: Vec<[u8; 3]> = colors.iter().filter_map(|c| parse_hex(c)).collect();
    let (w, h) = (source.width(), source.height());
    if r == 0 || targets.is_empty() || w == 0 || h == 0 {
        return;
    }
    // Each pixel's straight colour through the sRGB curve, and whether it is chosen: it shows,
    // and its 8-bit colour is one of the chosen ones exactly. D-87: a pixel that does not show
    // is never chosen, whatever colour its zeros would make.
    let (enc, chosen): (Vec<[f32; 3]>, Vec<bool>) = source
        .data()
        .par_chunks_exact(4)
        .map(|px| {
            let a = px[3];
            if a <= 0.0 {
                return ([0.0; 3], false);
            }
            let c = [0, 1, 2].map(|i| linear_to_srgb(px[i] / a));
            (c, targets.contains(&c.map(quantise_u8)))
        })
        .unzip();
    if !chosen.contains(&true) {
        return;
    }
    // The weights are always the full blur's, whatever a pass reaches.
    let zone = r as f64 / 3.0;
    let tbl: Vec<f32> = (0..=r)
        .map(|k| (-((k * k) as f64) / (2.0 * zone * zone)).exp() as f32)
        .collect();
    let mut out = enc.clone();
    // The original's five passes, each reading what the one before wrote.
    for (reach, across) in [(r, true), (r, false), (r / 4, true), (r / 16, false), (r / 64, true)] {
        if reach > 0 {
            out = pass(&out, &chosen, w, h, &tbl, reach, across);
        }
    }
    // Back to working values, alpha unchanged (D-87; the original makes it opaque). A pixel
    // whose colour came out as it went in keeps its value exactly.
    source.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        if chosen[i] && out[i] != enc[i] {
            let a = px[3];
            for c in 0..3 {
                px[c] = srgb_to_linear(out[i][c]) * a;
            }
        }
    });
}

/// One pass along the rows (`across`) or the columns. Each chosen pixel becomes the weighted
/// average of itself and the chosen pixels on either side, out to `reach`; each side stops at
/// the first pixel that is not chosen, or at the edge, and the weights left are made to sum to
/// one. D-87: nothing is rounded, as the original's floating-point path does.
fn pass(
    enc: &[[f32; 3]],
    chosen: &[bool],
    w: usize,
    h: usize,
    tbl: &[f32],
    reach: usize,
    across: bool,
) -> Vec<[f32; 3]> {
    let (count, len) = if across { (h, w) } else { (w, h) };
    let at = move |line: usize, j: usize| if across { line * w + j } else { j * w + line };
    let lines: Vec<Vec<[f32; 3]>> = (0..count)
        .into_par_iter()
        .map(|line| {
            (0..len)
                .map(|j| {
                    let own = enc[at(line, j)];
                    if !chosen[at(line, j)] {
                        return own;
                    }
                    // Summed as differences from the pixel's own colour, which is the same
                    // average, so that a flat patch comes out exactly as it went in.
                    let (mut got, mut total) = ([0.0f32; 3], tbl[0]);
                    for step in [-1isize, 1] {
                        for (k, &wk) in tbl.iter().enumerate().take(reach + 1).skip(1) {
                            let m = j as isize + step * k as isize;
                            if m < 0 || m >= len as isize || !chosen[at(line, m as usize)] {
                                break;
                            }
                            let other = enc[at(line, m as usize)];
                            for c in 0..3 {
                                got[c] += wk * (other[c] - own[c]);
                            }
                            total += wk;
                        }
                    }
                    [0, 1, 2].map(|c| own[c] + got[c] / total)
                })
                .collect()
        })
        .collect();
    let mut out = vec![[0.0f32; 3]; w * h];
    for (line, values) in lines.into_iter().enumerate() {
        for (j, v) in values.into_iter().enumerate() {
            out[at(line, j)] = v;
        }
    }
    out
}
