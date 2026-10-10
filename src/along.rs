//! D-356 (`docs/effects/EFFECTS.md` P0-22): effects that draw along a path.
//!
//! A path here is an outline as `mask::flatten` or `shape::flatten` gives it, with whether it is
//! closed (D-357: a shape's path may be open), in the coordinates it is drawn in. [`runs`] measures the paths along their length, trims them to a start and an end, and
//! lays dabs on them a step apart; [`distance`] finds the nearest dab of one run. Path Stroke
//! (`core.stroke`) is the first effect drawn with them; Vegas, Scribble, Write-on, Fill, Audio
//! Waveform and Energy Stroke are to be.

use rayon::prelude::*;

use crate::WorkingBuffer;

/// The dabs on one straight piece of a path: the first dab's centre, the last's, and the number
/// of steps between them, 0 for one dab. A negative count is a brush laid all along, from the
/// first point to the last.
pub(crate) type Run = [f64; 5];

/// The runs of a brush laid along `paths`, outlines each with whether it is closed, between `start` and `end` per cent
/// of each one's length (taken smaller first), a dab every `step` pixels from each path's first
/// point, so the dabs stay put as the start moves; `step` 0 lays the brush all along. With
/// `sequential`, the paths are one length, end to end in order, and `start` and `end` are per
/// cent of it. Document 21's Path Stroke paragraph is the rule.
pub(crate) fn runs(paths: &[(Vec<(f64, f64)>, bool)], start: f64, end: f64, step: f64, sequential: bool) -> Vec<Run> {
    let (s, e) = (start.min(end) / 100.0, start.max(end) / 100.0);
    let pieces: Vec<Vec<((f64, f64), (f64, f64))>> = paths.iter().map(|(p, closed)| crate::shape::path_segments(p, *closed)).collect();
    let length = |(a, b): &((f64, f64), (f64, f64))| (b.0 - a.0).hypot(b.1 - a.1);
    let lengths: Vec<f64> = pieces.iter().map(|p| p.iter().map(length).sum()).collect();
    let total: f64 = lengths.iter().sum();
    let (mut before, mut out) = (0.0, Vec::new());
    for (pieces, &l) in pieces.iter().zip(&lengths) {
        let (lo, hi) = if sequential { ((s * total - before).max(0.0), (e * total - before).min(l)) } else { (s * l, e * l) };
        before += l;
        if lo >= hi {
            continue;
        }
        let mut at = 0.0;
        for piece in pieces {
            let n = length(piece);
            let (u0, u1) = (lo.max(at), hi.min(at + n));
            let (a, b) = *piece;
            let point = |u: f64| {
                let f = (u - at) / n;
                (a.0 + f * (b.0 - a.0), a.1 + f * (b.1 - a.1))
            };
            if n > 0.0 && u0 <= u1 {
                if step == 0.0 {
                    if u0 < u1 {
                        let (p, q) = (point(u0), point(u1));
                        out.push([p.0, p.1, q.0, q.1, -1.0]);
                    }
                } else {
                    let (k0, k1) = ((u0 / step - 1e-12).ceil(), (u1 / step + 1e-12).floor());
                    if k0 <= k1 {
                        let (p, q) = (point(k0 * step), point(k1 * step));
                        out.push([p.0, p.1, q.0, q.1, k1 - k0]);
                    }
                }
            }
            at += n;
        }
    }
    out
}

/// Path Stroke's runs: `paths` trimmed to `start` and `end` with a dab every `spacing` per cent
/// of the brush `size` (0 for all along), measured where the paths are and moved by `origin`,
/// the drawing's corner in a buffer grown by an effect above. The processor and the card share it.
pub(crate) fn stroke_runs(paths: &[(Vec<(f64, f64)>, bool)], (ox, oy): (usize, usize), [start, end, spacing, size]: [f64; 4], sequential: bool) -> Vec<Run> {
    let mut out = runs(paths, start, end, spacing / 100.0 * size, sequential);
    for run in &mut out {
        run[0] += ox as f64;
        run[1] += oy as f64;
        run[2] += ox as f64;
        run[3] += oy as f64;
    }
    out
}

/// The distance from (`x`, `y`) to the nearest dab of `run`: the one nearest along it, which on
/// a straight piece is the nearest of all. Ties are the same distance, whichever is taken.
pub(crate) fn distance(run: &Run, x: f64, y: f64) -> f64 {
    let [px, py, qx, qy, n] = *run;
    let (dx, dy) = (qx - px, qy - py);
    let l2 = dx * dx + dy * dy;
    let mut t = if l2 == 0.0 { 0.0 } else { (((x - px) * dx + (y - py) * dy) / l2).clamp(0.0, 1.0) };
    if n > 0.0 {
        t = (t * n + 0.5).floor() / n;
    }
    (x - px - t * dx).hypot(y - py - t * dy)
}

/// D-420: the way across `(dx, dy)` to its left on the screen, `(dy, -dx)` at length one: up
/// for a way drawn left to right, and up when it has no length.
pub(crate) fn facing(dx: f64, dy: f64) -> (f64, f64) {
    let length = dx.hypot(dy);
    if length == 0.0 {
        (0.0, -1.0)
    } else {
        (dy / length, -dx / length)
    }
}

/// D-420: the point a share `u` of the way round the closed outline `o` (0 its first point, 1
/// back to it), and the facing of the piece it falls on.
pub(crate) fn point_along(o: &[(f64, f64)], u: f64) -> ((f64, f64), (f64, f64)) {
    if o.is_empty() {
        return ((0.0, 0.0), (0.0, -1.0));
    }
    let piece = |i: usize| (o[i], o[(i + 1) % o.len()]);
    let length = |(a, b): ((f64, f64), (f64, f64))| (b.0 - a.0).hypot(b.1 - a.1);
    let total: f64 = (0..o.len()).map(|i| length(piece(i))).sum();
    if total == 0.0 {
        return (o[0], (0.0, -1.0));
    }
    let (s, mut walked) = (u * total, 0.0);
    for i in 0..o.len() {
        let (a, b) = piece(i);
        let l = length((a, b));
        if l > 0.0 && walked + l >= s {
            let v = (s - walked) / l;
            return ((a.0 + v * (b.0 - a.0), a.1 + v * (b.1 - a.1)), facing(b.0 - a.0, b.1 - a.1));
        }
        walked += l;
    }
    let (a, b) = (0..o.len()).rev().map(piece).find(|(a, b)| a != b).unwrap_or(piece(0));
    (b, facing(b.0 - a.0, b.1 - a.1))
}

/// A run's box, outside which it is further than `reach` from every point: left, right, top,
/// bottom.
pub(crate) fn run_box(run: &Run, reach: f64) -> [f64; 4] {
    [run[0].min(run[2]) - reach, run[0].max(run[2]) + reach, run[1].min(run[3]) - reach, run[1].max(run[3]) + reach]
}

/// D-356: Path Stroke. A round brush `size` pixels across laid on `runs` (in the buffer's
/// coordinates), `hardness` and `opacity` 0 to 100, `color` linear, `style` "on_original",
/// "on_transparent" or "reveal". The covering at a pixel's centre is a smoothstep of the
/// distance to the nearest dab, soft over the brush's soft edge, smoothed over one pixel when
/// hard; dabs do not build up. The buffer never grows.
pub(crate) fn path_stroke(buffer: &mut WorkingBuffer, runs: &[Run], [size, hardness, opacity]: [f64; 3], color: [f64; 3], style: &str) {
    let w = buffer.width();
    let r = size / 2.0;
    let soft = (r * (1.0 - hardness / 100.0)).max(1.0);
    let reach = r + 0.5;
    let runs: Vec<(Run, [f64; 4])> = if size == 0.0 || opacity == 0.0 { Vec::new() } else { runs.iter().map(|run| (*run, run_box(run, reach))).collect() };
    let every = style != "on_original";
    buffer.data_mut().par_chunks_exact_mut(4 * w).enumerate().for_each(|(j, row)| {
        let y = j as f64 + 0.5;
        let mut near = vec![f64::INFINITY; w];
        for (run, [left, right, top, bottom]) in &runs {
            if y <= *top || y >= *bottom {
                continue;
            }
            let x0 = (left - 0.5).ceil().max(0.0) as usize;
            let x1 = ((right - 0.5).floor().max(-1.0) + 1.0).min(w as f64) as usize;
            for (i, d) in near.iter_mut().enumerate().take(x1).skip(x0) {
                *d = d.min(distance(run, i as f64 + 0.5, y));
            }
        }
        for (px, d) in row.chunks_exact_mut(4).zip(&near) {
            if !every && *d >= reach {
                continue;
            }
            let t = ((r + 0.5 - d) / soft).clamp(0.0, 1.0);
            let c = opacity / 100.0 * t * t * (3.0 - 2.0 * t);
            match style {
                "on_original" => {
                    for ch in 0..3 {
                        px[ch] = (px[ch] as f64 * (1.0 - c) + color[ch] * c) as f32;
                    }
                    px[3] = (px[3] as f64 * (1.0 - c) + c) as f32;
                }
                "on_transparent" => {
                    for ch in 0..3 {
                        px[ch] = (color[ch] * c) as f32;
                    }
                    px[3] = c as f32;
                }
                _ => {
                    for v in px.iter_mut() {
                        *v = (*v as f64 * c) as f32;
                    }
                }
            }
        }
    });
}
