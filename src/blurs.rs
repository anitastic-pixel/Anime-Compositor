//! D-92's directional blur: document 21's rule, on a layer's own pixels.
//!
//! This program's own method, modelled on After Effects' Directional Blur; nothing is ported.
//! `tools/directional_blur_reference.py` is the same rule worked a second way, and
//! `tests/b36_directional_blur.rs` holds this to its numbers.

use crate::render::sample_bilinear;
use crate::WorkingBuffer;
use rayon::prelude::*;

/// One pixel's step along `direction`, in degrees clockwise from up. Exact at a whole quarter
/// turn, so a streak straight across or straight down takes nothing from the lines beside it.
fn along(direction: f64) -> (f64, f64) {
    let q = direction.rem_euclid(360.0);
    if q == 0.0 {
        (0.0, -1.0)
    } else if q == 90.0 {
        (1.0, 0.0)
    } else if q == 180.0 {
        (0.0, 1.0)
    } else if q == 270.0 {
        (-1.0, 0.0)
    } else {
        let a = direction.to_radians();
        (a.sin(), -a.cos())
    }
}

/// Average each pixel of `source` along a line `length` pixels long through it, in place, and
/// return how far it grew on each side. Both settings are already inside their ranges; length 0
/// changes nothing.
pub(crate) fn directional_blur(source: &mut WorkingBuffer, direction: f64, length: f64) -> usize {
    let n = length.ceil() as usize + 1;
    if n == 1 {
        return 0;
    }
    let grow = (length / 2.0).ceil() as usize;
    let u = along(direction);
    let steps: Vec<(f64, f64)> = (0..n)
        .map(|k| {
            let t = -length / 2.0 + k as f64 * length / (n - 1) as f64;
            (t * u.0, t * u.1)
        })
        .collect();
    let (w, h) = (source.width() + 2 * grow, source.height() + 2 * grow);
    let mut out = WorkingBuffer::transparent(w, h);
    let src = &*source;
    // ponytail: n bilinear samples a pixel, so a 1080p layer at length 500 is ~1e9 samples;
    // a running sum along the line is the upgrade if P-17 finds it matters.
    out.data_mut()
        .par_chunks_exact_mut(w * 4)
        .enumerate()
        .for_each(|(y, row)| {
            let cy = y as f64 - grow as f64 + 0.5;
            for (x, px) in row.chunks_exact_mut(4).enumerate() {
                let cx = x as f64 - grow as f64 + 0.5;
                let mut sum = [0.0f32; 4];
                for &(dx, dy) in &steps {
                    let s = sample_bilinear(src, cx + dx, cy + dy);
                    for i in 0..4 {
                        sum[i] += s[i];
                    }
                }
                for i in 0..4 {
                    px[i] = sum[i] / n as f32;
                }
            }
        });
    *source = out;
    grow
}
