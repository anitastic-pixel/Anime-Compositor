//! D-670: the Simulation effects that sit beside the particle engine but are not its particle
//! system, each drawn by the effect itself (document 21). Every rule is ours, written after what
//! each control is for; each effect's `tools/<name>_reference.py` is its reference, and this file
//! follows it step for step.
//!
//! No state is carried from frame to frame (document 20): every frame is worked from the layer's
//! in point, so a frame drawn alone equals the same frame played from the start.

use crate::effects::{Effect, BUBBLES_SETTINGS, DRIZZLE_SETTINGS, HAIR_SETTINGS};
use crate::particles::{tile_lists, HEAD, STRIDE, TILE};
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
/// binning reads: the head left at zero, then `STRIDE` numbers a bubble, `[3, x, y, radius, 0 ..]`,
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
        k.extend([3.0, x, y, rho, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
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
                    if let Some((b, cover)) = bubble_at(&k[HEAD + STRIDE * n..HEAD + STRIDE * n + STRIDE], x, y, sign, shading, &read) {
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

/// D-672: Drizzle's settings held inside their ranges.
fn drizzle_held(v: &[f64; 16]) -> [f64; 16] {
    std::array::from_fn(|k| v[k].clamp(DRIZZLE_SETTINGS[k].1, DRIZZLE_SETTINGS[k].2))
}

/// D-672: the drops born by each frame from the in point to this one, C(0) .. C(u): the trapezoid
/// sums of the drip rate (drops a second), the rate at each frame from `history` when any setting
/// is keyed, else the steady rate.
// ponytail: O(frames since the in point) a frame; keep the sums between frames if long shots
// grow slow.
fn drip_counts(now: &[f64; 16], u: usize, fps: f64, history: Option<&[[f64; 16]]>) -> Vec<f64> {
    let rate = |m: usize| drizzle_held(history.map_or(now, |h| &h[m.min(h.len() - 1)]))[0];
    let mut c = Vec::with_capacity(u + 1);
    c.push(0.0);
    for m in 0..u {
        c.push(c[m] + (rate(m) + rate(m + 1)) / (2.0 * fps));
    }
    c
}

/// D-672: C at `t` frames since the in point, joined by straight lines between whole frames.
fn count_at(c: &[f64], t: f64) -> f64 {
    if t <= 0.0 {
        return 0.0;
    }
    let m = t.floor() as usize;
    if m >= c.len() - 1 {
        return c[c.len() - 1];
    }
    c[m] + (c[m + 1] - c[m]) * (t - m as f64)
}

/// D-672: Drizzle's live drops at this frame, in order of birth, as sprite numbers the particle
/// engine's binning reads: the head left at zero, then `STRIDE` numbers a drop, `[3, x, y, R, A, 0
/// ..]`, its place, its ripple's radius and its height, in the drawing's own pixels. `w` and `h`
/// the drawing's size. The longevity is the in point's (CycoreFX's "cannot be animated").
pub(crate) fn drizzle_list(e: &Effect, w: f64, h: f64) -> Vec<f64> {
    let Effect::Drizzle { values, clock: [u, fps], history, .. } = e else { unreachable!("Drizzle of another effect") };
    let history = history.as_deref().map(|h| h.as_slice());
    let v = drizzle_held(values);
    let life = drizzle_held(history.map_or(values, |h| &h[0]))[1];
    let mut k = vec![0.0; HEAD];
    if *u < 0.0 || *fps <= 0.0 || life == 0.0 {
        return k;
    }
    let c = drip_counts(values, *u as usize, *fps, history);
    let (lo, hi) = (count_at(&c, u - life * fps), c[c.len() - 1]);
    let base = crate::grade::mix(0);
    let mut m = 0;
    for j in lo.floor() as i64 + 1..=hi.floor() as i64 {
        let jf = j as f64;
        while c[m + 1] < jf {
            m += 1;
        }
        let born = m as f64 + (jf - c[m]) / (c[m + 1] - c[m]);
        let s = (u - born) / fps / life;
        let r = s * v[5] / 100.0 * h / 4.0;
        if r <= 0.0 {
            continue;
        }
        let a = v[4] / 100.0 * (h / 50.0) * 4.0 * s * (1.0 - s);
        let at = |n| (crate::grade::unit(base, j, 0, 0, n) + 1.0) / 2.0;
        k.extend([3.0, at(0) * w, at(1) * h, r, a, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }
    k
}

/// D-672: the numbers every pixel of Drizzle shares, the card's as the processor's: the rings a
/// ripple, the displacement's scale, the light's colour in linear light, its intensity, ambient,
/// diffuse, specular, metal and roughness, 1 for a point light, then the point light's place in
/// the drawing's own pixels and its height, or the distant light's direction times 100 and its
/// height. `w` and `h` the drawing's size.
pub(crate) fn drizzle_light(e: &Effect, w: f64, h: f64) -> [f64; 15] {
    let Effect::Drizzle { values, words, light_color, .. } = e else { unreachable!("Drizzle of another effect") };
    let v = drizzle_held(values);
    let lc = crate::effects::encoded(light_color).map(crate::grade::to_linear);
    let (ux, uy) = crate::blurs::along(v[10]);
    let point = words[0] == "point";
    let (lx, ly) = if point { (v[8] / 100.0 * w, v[9] / 100.0 * h) } else { (100.0 * ux, 100.0 * uy) };
    [
        1.0 + v[2] / 360.0,
        v[3] / 100.0 * (h / 50.0),
        lc[0],
        lc[1],
        lc[2],
        v[6] / 100.0,
        v[11] / 100.0,
        v[12] / 100.0,
        v[13] / 100.0,
        v[15] / 100.0,
        v[14],
        point as u8 as f64,
        lx,
        ly,
        v[7],
    ]
}

/// D-672: Drizzle, after CycoreFX's CC Drizzle (document 21): raindrops on a pond, each a ring
/// of ripples born at its drop, spreading and dying away; the layer read through the water's
/// slope and lit by Blobbylize's rule. `(ox, oy)` the drawing's corner in the buffer: the drops
/// and the light are in the drawing's own space, every pixel of the buffer is water, and the read
/// is clamped to the buffer. Never grows. `e`'s clock and history already filled as compose
/// fills them.
pub(crate) fn drizzle(source: &mut WorkingBuffer, e: &Effect, (ox, oy): (usize, usize)) {
    let (bw, bh) = (source.width(), source.height());
    let (w0, h0) = (bw.saturating_sub(2 * ox), bh.saturating_sub(2 * oy));
    if w0 == 0 || h0 == 0 {
        return;
    }
    let k = drizzle_list(e, w0 as f64, h0 as f64);
    let [n, kd, lr, lg, lb, inten, amb, dif, spc, metal, rough, point, lx, ly, lz] = drizzle_light(e, w0 as f64, h0 as f64);
    let lc = [lr, lg, lb];
    let tau = 2.0 * std::f64::consts::PI * n;
    let unit = |v: [f64; 3]| {
        let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
        if l > 0.0 { v.map(|c| c / l) } else { [0.0; 3] }
    };
    let distant = unit([lx, ly, lz]);
    let (starts, list) = tile_lists(&k, (ox, oy), bw, bh);
    let still = source.data().to_vec();
    let read = |x: f64, y: f64| -> [f64; 4] {
        let (fx, fy) = ((x + ox as f64 - 0.5).clamp(0.0, (bw - 1) as f64), (y + oy as f64 - 0.5).clamp(0.0, (bh - 1) as f64));
        let (x0, y0) = (fx.floor() as usize, fy.floor() as usize);
        let (x1, y1) = ((x0 + 1).min(bw - 1), (y0 + 1).min(bh - 1));
        let (tx, ty) = (fx - x0 as f64, fy - y0 as f64);
        let at = |x: usize, y: usize, c: usize| still[(y * bw + x) * 4 + c] as f64;
        [0, 1, 2, 3].map(|c| (at(x0, y0, c) * (1.0 - tx) + at(x1, y0, c) * tx) * (1.0 - ty) + (at(x0, y1, c) * (1.0 - tx) + at(x1, y1, c) * tx) * ty)
    };
    let tw = bw.div_ceil(TILE);
    source.data_mut().par_chunks_mut(bw * 4 * TILE).enumerate().for_each(|(ty, band)| {
        for (row, line) in band.chunks_exact_mut(bw * 4).enumerate() {
            let y = (ty * TILE + row) as f64 - oy as f64 + 0.5;
            for (bx, px) in line.chunks_exact_mut(4).enumerate() {
                let t = ty * tw + bx / TILE;
                let x = bx as f64 - ox as f64 + 0.5;
                let (mut zx, mut zy) = (0.0, 0.0);
                for &i in &list[starts[t]..starts[t + 1]] {
                    let s = &k[HEAD + STRIDE * i..HEAD + STRIDE * i + STRIDE];
                    let (dx, dy, r) = (x - s[1], y - s[2], s[3]);
                    let d2 = dx * dx + dy * dy;
                    if d2 >= r * r {
                        continue;
                    }
                    let q = d2.sqrt() / r;
                    let one = 1.0 - q * q;
                    let ph = tau * q;
                    let sq = if q == 0.0 { tau * tau } else { tau * ph.sin() / q };
                    let g = s[4] * (-4.0 * one * ph.cos() - one * one * sq);
                    zx += g * dx / (r * r);
                    zy += g * dy / (r * r);
                }
                let c = read(x - kd * zx, y - kd * zy);
                let al = c[3];
                let col = if al > 0.0 { [c[0] / al, c[1] / al, c[2] / al] } else { [0.0; 3] };
                let nn = unit([-zx, -zy, 1.0]);
                let l = if point == 1.0 { unit([lx - x, ly - y, lz]) } else { distant };
                let nl = nn[0] * l[0] + nn[1] * l[1] + nn[2] * l[2];
                let rz = 2.0 * nl * nn[2] - l[2];
                let shine = if nl > 0.0 && rz > 0.0 { rz.powf(1.0 / rough) } else { 0.0 };
                for ch in 0..3 {
                    let lit = col[ch] * (amb + dif * inten * lc[ch] * nl.max(0.0)) + spc * inten * (lc[ch] + (col[ch] - lc[ch]) * metal) * shine;
                    px[ch] = (lit * al) as f32;
                }
                px[3] = al as f32;
            }
        }
    });
}

/// D-673: Hair's settings held inside their ranges.
fn hair_held(v: &[f64; 13]) -> [f64; 13] {
    std::array::from_fn(|k| v[k].clamp(HAIR_SETTINGS[k].1, HAIR_SETTINGS[k].2))
}

/// D-673: Hair's strands, every one the density asks for, in order, as sprite numbers the
/// particle engine's binning reads: the head left at zero, then `STRIDE` numbers a strand, `[3, x, y,
/// reach, d_x, d_y, d_z, L_j, b_j, u_2]`, its root, how far from the root it can cover, the way it
/// leaves the root, its length and bend, and the share its root's alpha must pass for it to grow,
/// in the drawing's own pixels. `w` and `h` the drawing's size. Whether a strand grows is left to
/// the drawing, which reads the layer at its root.
pub(crate) fn hair_list(e: &Effect, w: f64, h: f64) -> Vec<f64> {
    let Effect::Hair { values, words, .. } = e else { unreachable!("Hair of another effect") };
    let v = hair_held(values);
    let mut k = vec![0.0; HEAD];
    let big = v[0] / 100.0 * h / 10.0;
    if big == 0.0 {
        return k;
    }
    let n = (v[3] / 100.0 * 4000.0 * w / h).floor() as i64;
    let t = v[1] * h / 1080.0;
    let base = crate::grade::mix(0);
    k.reserve(STRIDE * n.max(0) as usize);
    for j in 0..n {
        let u: [f64; 6] = std::array::from_fn(|c| (crate::grade::unit(base, j, 0, 0, c as u64) + 1.0) / 2.0);
        let (p, th) = (2.0 * std::f64::consts::PI * u[3], std::f64::consts::PI / 3.0 * u[4]);
        let d = [th.sin() * p.cos(), th.sin() * p.sin(), th.cos()];
        let len = big * (0.6 + 0.4 * u[5]);
        let b = v[2] / 100.0 * if words[0] == "on" { len / big } else { 1.0 };
        let reach = len * (d[0].hypot(d[1]) + b.abs()) + t / 2.0 + 1.0;
        k.extend([3.0, u[0] * w, u[1] * h, reach, d[0], d[1], d[2], len, b, u[2], 0.0]);
    }
    k
}

/// D-673: the numbers every pixel of Hair shares, the card's as the processor's: the hair's
/// colour in linear light, the inheritance and opacity as shares, the thickness in pixels, the
/// light's intensity, the light's direction (towards it, unit length), ambient, diffuse,
/// specular, roughness and metal. `h` the drawing's height.
pub(crate) fn hair_light(e: &Effect, h: f64) -> [f64; 15] {
    let Effect::Hair { values, hair_color, .. } = e else { unreachable!("Hair of another effect") };
    let v = hair_held(values);
    let hc = crate::effects::encoded(hair_color).map(crate::grade::to_linear);
    let (ux, uy) = crate::blurs::along(v[7]);
    let l = (ux * ux + uy * uy + 1.0).sqrt();
    [hc[0], hc[1], hc[2], v[4] / 100.0, v[5] / 100.0, v[1] * h / 1080.0, v[6] / 100.0, ux / l, uy / l, 1.0 / l, v[8] / 100.0, v[9] / 100.0, v[10] / 100.0, v[11], v[12] / 100.0]
}

/// D-673: Hair, after CycoreFX's CC Hair (document 21): strands grown where the layer is opaque,
/// 3D curves seen straight on, bent by their weight, coloured from their own colour and the
/// picture at the root, lit as hair is, and laid over the layer one after another. `(ox, oy)`
/// the drawing's corner in the buffer: the strands are in the drawing's own space and every
/// pixel of the buffer can be covered; the roots read the layer clamped to the buffer. Never
/// grows.
pub(crate) fn hair(source: &mut WorkingBuffer, e: &Effect, (ox, oy): (usize, usize)) {
    let (bw, bh) = (source.width(), source.height());
    let (w0, h0) = (bw.saturating_sub(2 * ox), bh.saturating_sub(2 * oy));
    if w0 == 0 || h0 == 0 {
        return;
    }
    let k = hair_list(e, w0 as f64, h0 as f64);
    if k.len() == HEAD {
        return;
    }
    let [hr, hg, hb, inh, op, t, inten, lx, ly, lz, amb, dif, spc, rough, metal] = hair_light(e, h0 as f64);
    let hc = [hr, hg, hb];
    let (starts, list) = tile_lists(&k, (ox, oy), bw, bh);
    let still = source.data().to_vec();
    let read = |x: f64, y: f64| -> [f64; 4] {
        let (fx, fy) = ((x + ox as f64 - 0.5).clamp(0.0, (bw - 1) as f64), (y + oy as f64 - 0.5).clamp(0.0, (bh - 1) as f64));
        let (x0, y0) = (fx.floor() as usize, fy.floor() as usize);
        let (x1, y1) = ((x0 + 1).min(bw - 1), (y0 + 1).min(bh - 1));
        let (tx, ty) = (fx - x0 as f64, fy - y0 as f64);
        let at = |x: usize, y: usize, c: usize| still[(y * bw + x) * 4 + c] as f64;
        [0, 1, 2, 3].map(|c| (at(x0, y0, c) * (1.0 - tx) + at(x1, y0, c) * tx) * (1.0 - ty) + (at(x0, y1, c) * (1.0 - tx) + at(x1, y1, c) * tx) * ty)
    };
    // Each strand worked once: its colour, when it grows (the picture's straight colour at its
    // root mixed in), and its nine points.
    let strands: Vec<Option<([f64; 3], [[f64; 2]; 9])>> = k[HEAD..]
        .par_chunks_exact(STRIDE)
        .map(|s| {
            let c = read(s[1], s[2]);
            (s[9] < c[3]).then(|| {
                let col = std::array::from_fn(|ch| hc[ch] * (1.0 - inh) + c[ch] / c[3] * inh);
                let pts = std::array::from_fn(|i| {
                    let q = i as f64 / 8.0;
                    [s[1] + s[7] * s[4] * q, s[2] + s[7] * (s[5] * q + s[8] * q * q)]
                });
                (col, pts)
            })
        })
        .collect();
    let tw = bw.div_ceil(TILE);
    source.data_mut().par_chunks_mut(bw * 4 * TILE).enumerate().for_each(|(ty, band)| {
        for (row, line) in band.chunks_exact_mut(bw * 4).enumerate() {
            let y = (ty * TILE + row) as f64 - oy as f64 + 0.5;
            for (bx, px) in line.chunks_exact_mut(4).enumerate() {
                let tt = ty * tw + bx / TILE;
                let x = bx as f64 - ox as f64 + 0.5;
                let mut o = [px[0] as f64, px[1] as f64, px[2] as f64, px[3] as f64];
                for &i in &list[starts[tt]..starts[tt + 1]] {
                    let Some((col, p)) = &strands[i] else { continue };
                    let (mut best, mut at) = (0.0, 0.0);
                    for n in 0..8 {
                        let (ax, ay, ex, ey) = (p[n][0], p[n][1], p[n + 1][0] - p[n][0], p[n + 1][1] - p[n][1]);
                        let ee = ex * ex + ey * ey;
                        let s = if ee == 0.0 { 0.0 } else { (((x - ax) * ex + (y - ay) * ey) / ee).clamp(0.0, 1.0) };
                        let e = (x - (ax + s * ex)).hypot(y - (ay + s * ey));
                        let along = (n as f64 + s) / 8.0;
                        let cov = (t / 2.0 * (1.0 - 0.75 * along) - e + 0.5).clamp(0.0, 1.0);
                        if cov > best {
                            (best, at) = (cov, along);
                        }
                    }
                    if best == 0.0 {
                        continue;
                    }
                    let sp = &k[HEAD + STRIDE * i..HEAD + STRIDE * i + STRIDE];
                    let dv = [sp[4], sp[5] + 2.0 * sp[8] * at, sp[6]];
                    let dl = (dv[0] * dv[0] + dv[1] * dv[1] + dv[2] * dv[2]).sqrt();
                    let dv = dv.map(|c| c / dl);
                    let a = dv[0] * lx + dv[1] * ly + dv[2] * lz;
                    let kd = (1.0 - a * a).max(0.0).sqrt();
                    let br = kd * (1.0 - dv[2] * dv[2]).max(0.0).sqrt() - a * dv[2];
                    let ks = if br > 0.0 { br.powf(1.0 / rough) } else { 0.0 };
                    let al = best * op;
                    for ch in 0..3 {
                        let lit = col[ch] * (amb + dif * inten * kd) + spc * inten * (1.0 + (col[ch] - 1.0) * metal) * ks;
                        o[ch] = lit * al + o[ch] * (1.0 - al);
                    }
                    o[3] = al + o[3] * (1.0 - al);
                }
                for c in 0..4 {
                    px[c] = o[c] as f32;
                }
            }
        }
    });
}
