//! D-670: the Simulation effects that sit beside the particle engine but are not its particle
//! system, each drawn by the effect itself (document 21). Every rule is ours, written after what
//! each control is for; each effect's `tools/<name>_reference.py` is its reference, and this file
//! follows it step for step.
//!
//! No state is carried from frame to frame (document 20): every frame is worked from the layer's
//! in point, so a frame drawn alone equals the same frame played from the start.

use crate::effects::{Effect, BUBBLES_SETTINGS};
use crate::particles::{tile_lists, HEAD, TILE};
use crate::WorkingBuffer;
use rayon::prelude::*;

/// D-671: the running sums of speed (index 1) and wobble frequency (index 3) from the in point,
/// in seconds, as trapezoid sums over the settings at each frame from the in point to this one;
/// without a history (nothing keyed) the settings are steady, so each is the setting times the
/// time.
fn bubble_sums(now: &[f64; 6], [u, fps]: [f64; 2], history: Option<&[[f64; 6]]>) -> (f64, f64) {
    if fps <= 0.0 || u <= 0.0 {
        return (0.0, 0.0);
    }
    let held = |v: &[f64; 6], k: usize| v[k].clamp(BUBBLES_SETTINGS[k].1, BUBBLES_SETTINGS[k].2);
    match history {
        Some(h) => {
            let n = (u as usize).min(h.len().saturating_sub(1));
            let sum = |k| (0..n).map(|j| (held(&h[j], k) + held(&h[j + 1], k)) / 2.0).sum::<f64>() / fps;
            (sum(1), sum(3))
        }
        None => (held(now, 1) * u / fps, held(now, 3) * u / fps),
    }
}

/// D-671: Bubbles' bubbles at this frame, in order, as sprite numbers the particle engine's
/// binning reads: the head left at zero, then ten numbers a bubble, `[3, x, y, radius, 0 ..]`,
/// in the drawing's own pixels. `w` and `h` the drawing's size.
pub(crate) fn bubble_list(e: &Effect, w: f64, h: f64) -> Vec<f64> {
    let Effect::Bubbles { values, clock, history, .. } = e else { unreachable!("Bubbles of another effect") };
    let v: [f64; 6] = std::array::from_fn(|k| values[k].clamp(BUBBLES_SETTINGS[k].1, BUBBLES_SETTINGS[k].2));
    let (s, f) = bubble_sums(&v, *clock, history.as_deref().map(|h| h.as_slice()));
    let base = crate::grade::mix(v[5].floor() as u64);
    let mut k = vec![0.0; HEAD];
    for i in 0..v[0].floor() as i64 {
        let u = |n| (crate::grade::unit(base, i, 0, 0, n) + 1.0) / 2.0;
        let rho = v[4] * (0.5 + u(2)) * h / 40.0;
        if rho == 0.0 {
            continue;
        }
        let r = h + 2.0 * rho;
        let q = u(1) + (0.5 + u(3)) * s * (h / 4.0) / r;
        let y = h + rho - (q - q.floor()) * r;
        let phi = u(4) + (0.75 + u(5) / 2.0) * f;
        let x = u(0) * (w + 2.0 * rho) - rho + v[2] / 10.0 * rho * (2.0 * std::f64::consts::PI * phi).sin();
        k.extend([3.0, x, y, rho, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }
    k
}

/// D-671: one bubble's colour at the own pixel centre (x, y), with its cover, or None where it
/// does not reach. `read` the bilinear read of the drawing at an own point; `sign` -1 for
/// Inverse, 1 for World; `shading` the word's place in [`crate::effects::BUBBLES_WORDS`].
fn bubble_at(s: &[f64], x: f64, y: f64, sign: f64, shading: usize, read: &impl Fn(f64, f64) -> [f64; 4]) -> Option<([f64; 4], f64)> {
    let (bx, by, rho) = (s[1], s[2], s[3]);
    let (dx, dy) = (x - bx, y - by);
    let d = dx.hypot(dy);
    let cover = (rho - d + 0.5).clamp(0.0, 1.0);
    if cover == 0.0 {
        return None;
    }
    let (ex, ey) = if d > rho { (dx / d, dy / d) } else { (dx / rho, dy / rho) };
    let t2 = (ex * ex + ey * ey).min(1.0);
    let k = 2.0 * rho / (1.0 + (1.0 - t2).sqrt());
    let c = read(bx + sign * k * ex, by + sign * k * ey);
    let b = match shading {
        1 => c.map(|v| v * (1.0 - t2) + t2),
        2 => [c[0] * (1.0 - t2), c[1] * (1.0 - t2), c[2] * (1.0 - t2), c[3] * (1.0 - t2) + t2],
        3 => c.map(|v| v * t2),
        4 => c.map(|v| v * (1.0 - t2)),
        _ => c,
    };
    Some((b, cover))
}

/// D-671: Bubbles, after CycoreFX's CC Bubbles (document 21): the layer becomes bubbles alone,
/// each the drawing read through itself, laid in order onto clear. `(ox, oy)` the drawing's
/// corner in the buffer; the bubbles stay in the drawing's own space and read only the drawing.
/// Never grows. `e`'s clock and history already filled as compose fills them.
// ponytail: processor tiles hold every bubble that reaches them, so many large bubbles make a
// pixel try them all; that is the rule (each lays over the last), so no cheaper cut exists.
pub(crate) fn bubbles(source: &mut WorkingBuffer, e: &Effect, (ox, oy): (usize, usize)) {
    let Effect::Bubbles { words, .. } = e else { unreachable!("Bubbles of another effect") };
    let (bw, bh) = (source.width(), source.height());
    let (w0, h0) = (bw.saturating_sub(2 * ox), bh.saturating_sub(2 * oy));
    if w0 == 0 || h0 == 0 {
        return;
    }
    let k = bubble_list(e, w0 as f64, h0 as f64);
    let sign = if words[0] == "world" { 1.0 } else { -1.0 };
    let shading = crate::effects::BUBBLES_WORDS[1].1.iter().position(|w| *w == words[1]).unwrap_or(3);
    let (starts, list) = tile_lists(&k, (ox, oy), bw, bh);
    let drawing = source.data().to_vec();
    let read = |x: f64, y: f64| -> [f64; 4] {
        let (fx, fy) = ((x - 0.5).clamp(0.0, (w0 - 1) as f64), (y - 0.5).clamp(0.0, (h0 - 1) as f64));
        let (x0, y0) = (fx.floor() as usize, fy.floor() as usize);
        let (x1, y1) = ((x0 + 1).min(w0 - 1), (y0 + 1).min(h0 - 1));
        let (tx, ty) = (fx - x0 as f64, fy - y0 as f64);
        let at = |x: usize, y: usize, c: usize| drawing[((y + oy) * bw + x + ox) * 4 + c] as f64;
        [0, 1, 2, 3].map(|c| (at(x0, y0, c) * (1.0 - tx) + at(x1, y0, c) * tx) * (1.0 - ty) + (at(x0, y1, c) * (1.0 - tx) + at(x1, y1, c) * tx) * ty)
    };
    let tw = bw.div_ceil(TILE);
    source.data_mut().par_chunks_mut(bw * 4 * TILE).enumerate().for_each(|(ty, band)| {
        for (row, line) in band.chunks_exact_mut(bw * 4).enumerate() {
            let y = (ty * TILE + row) as f64 - oy as f64 + 0.5;
            for (bx, px) in line.chunks_exact_mut(4).enumerate() {
                let t = ty * tw + bx / TILE;
                let x = bx as f64 - ox as f64 + 0.5;
                let mut o = [0.0f64; 4];
                for &n in &list[starts[t]..starts[t + 1]] {
                    if let Some((b, cover)) = bubble_at(&k[HEAD + 10 * n..HEAD + 10 * n + 10], x, y, sign, shading, &read) {
                        o = [0, 1, 2, 3].map(|c| b[c] * cover + o[c] * (1.0 - b[3] * cover));
                    }
                }
                for (v, o) in px.iter_mut().zip(o) {
                    *v = o as f32;
                }
            }
        }
    });
}
