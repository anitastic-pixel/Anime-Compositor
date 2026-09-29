//! D-184's HSV key: document 21's rule, on a layer's own pixels.
//!
//! OpenToonz's HSV Key, ported from `toonz/sources/stdfx/hsvkeyfx.cpp` with the colour
//! conversion `OLDRGB2HSV` from `toonz/sources/toonzlib/hsvutil.cpp`, at commit
//! 6571328019e3a6a99c13f408ee6f4755c9cddf73, under its licence:
//!
//! ```text
//! Copyright (c) 2016 - 2026, DWANGO Co., Ltd.
//! Copyright (c) 2016 - 2026, the respective contributors.
//! All rights reserved.
//! ```
//!
//! BSD 3-Clause "New" or "Revised" License. The full text, with its conditions and its
//! disclaimer, is kept in `docs/third_party/OpenToonz-LICENSE.txt` and ships with the build.
//!
//! D-184's changes from OpenToonz are marked where they are made: the pixel's straight colour,
//! a hue window that wraps round the circle, and saturation and value in percent. The fourth,
//! that it is added as a green key, is in the app's defaults. `tools/hsv_key_reference.py` is
//! the same rule worked a second way, and `tests/b120_hsv_key.rs` holds this to its numbers.

use crate::color::{linear_to_srgb, quantise_u8};
use crate::WorkingBuffer;
use rayon::prelude::*;

/// The six windows, as the effect holds them: hue and its range in degrees, the rest in percent.
pub(crate) struct Windows {
    pub hue: f64,
    pub hue_range: f64,
    pub saturation: f64,
    pub saturation_range: f64,
    pub value: f64,
    pub value_range: f64,
}

/// OpenToonz's `OLDRGB2HSV` on 8-bit red, green and blue over 255: hue in degrees, saturation
/// and value 0 to 1. A grey has saturation 0 and hue 0.
fn hsv(q: [u8; 3]) -> (f64, f64, f64) {
    let [r, g, b] = q.map(|v| f64::from(v) / 255.0);
    let (hi, lo) = (r.max(g).max(b), r.min(g).min(b));
    let s = if hi != 0.0 { (hi - lo) / hi } else { 0.0 };
    if s == 0.0 {
        return (0.0, s, hi);
    }
    let d = hi - lo;
    let h = if r == hi {
        (g - b) / d
    } else if g == hi {
        2.0 + (b - r) / d
    } else {
        4.0 + (r - g) / d
    } * 60.0;
    (if h < 0.0 { h + 360.0 } else { h }, s, hi)
}

/// D-184: each pixel that shows is made transparent when it is inside all three windows, or,
/// with `invert`, when it is not. The settings are already valid.
pub(crate) fn hsv_key(source: &mut WorkingBuffer, w: &Windows, invert: bool) {
    source.data_mut().par_chunks_exact_mut(4).for_each(|px| {
        let a = px[3];
        if a <= 0.0 {
            return;
        }
        // D-184's change: the straight colour, as colour key reads it; OpenToonz reads it
        // premultiplied.
        let q = [0, 1, 2].map(|i| quantise_u8(linear_to_srgb(px[i] / a)));
        let (h, s, v) = hsv(q);
        // D-184's change: the short way round the circle; OpenToonz cuts the window at 0 and 360.
        let d = (h - w.hue).abs();
        let inside = w.hue_range - d.min(360.0 - d) >= 0.0
            && w.saturation_range - (100.0 * s - w.saturation).abs() >= 0.0
            && w.value_range - (100.0 * v - w.value).abs() >= 0.0;
        if inside != invert {
            px.fill(0.0);
        }
    });
}
