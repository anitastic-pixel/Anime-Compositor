//! D-670: the Simulation effects that sit beside the particle engine but are not its particle
//! system, each drawn by the effect itself (document 21). Every rule is ours, written after what
//! each control is for; each effect's `tools/<name>_reference.py` is its reference, and this file
//! follows it step for step.
//!
//! No state is carried from frame to frame (document 20): every frame is worked from the layer's
//! in point, so a frame drawn alone equals the same frame played from the start.

use crate::effects::{Effect, BUBBLES_SETTINGS, DRIZZLE_SETTINGS, HAIR_SETTINGS, MERCURY_SETTINGS, POLYGON_SHARDS_SETTINGS, FOAM_SETTINGS, SCATTERIZE_SETTINGS};
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

/// D-672, D-674: the births by each frame from the in point to this one, C(0) .. C(u): the
/// trapezoid sums of `rate(m)`, births a second at the m-th frame since the in point.
// ponytail: O(frames since the in point) a frame; keep the sums between frames if long shots
// grow slow.
fn birth_counts(u: usize, fps: f64, rate: impl Fn(usize) -> f64) -> Vec<f64> {
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
    let c = birth_counts(*u as usize, *fps, |m| drizzle_held(history.map_or(values, |h| &h[m.min(h.len() - 1)]))[0]);
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

/// D-674: Mercury's settings held inside their ranges.
fn mercury_held(v: &[f64; 24]) -> [f64; 24] {
    std::array::from_fn(|k| v[k].clamp(MERCURY_SETTINGS[k].1, MERCURY_SETTINGS[k].2))
}

/// D-674: Mercury's share of a drop's size at `s`, from its birth to its death, by the influence
/// map: swelling in over the first quarter, shrinking out over the last.
fn mercury_share(word: &str, s: f64) -> f64 {
    match word {
        "in_out" => (4.0 * s).min(4.0 * (1.0 - s)).min(1.0),
        "in" => (4.0 * s).min(1.0),
        "out" => (4.0 * (1.0 - s)).min(1.0),
        _ => 1.0,
    }
}

/// D-674: Mercury's live drops at this frame, in order of birth, as sprite numbers the particle
/// engine's binning reads: the head left at zero, then `STRIDE` numbers a drop, `[3, x, y, rho, r, x_0,
/// y_0, u_2, 0, 0]`, its place, how far it reaches, its radius, its birthplace and the share its
/// birthplace's alpha must pass for it to live, in the drawing's own pixels. `w` and `h` the
/// drawing's size. The birth and path settings are each drop's birth frame's, the longevity the in
/// point's, the influence this frame's. Whether a drop lives, and its colour, are left to the
/// drawing, which reads the layer at its birthplace.
pub(crate) fn mercury_list(e: &Effect, w: f64, h: f64) -> Vec<f64> {
    let Effect::Mercury { values, words, clock: [u, fps], history, .. } = e else { unreachable!("Mercury of another effect") };
    let history = history.as_deref().map(|h| h.as_slice());
    let at_frame = |m: usize| mercury_held(history.map_or(values, |h| &h[m.min(h.len() - 1)]));
    let life = at_frame(0)[1];
    let big_a = mercury_held(values)[11] / 100.0;
    let mut k = vec![0.0; HEAD];
    if *u < 0.0 || *fps <= 0.0 || life == 0.0 {
        return k;
    }
    let c = birth_counts(*u as usize, *fps, |m| at_frame(m)[0]);
    let (lo, hi) = (count_at(&c, u - life * fps), c[c.len() - 1]);
    let base = crate::grade::mix(0);
    let tau = 2.0 * std::f64::consts::PI;
    let mut m = 0;
    for j in lo.floor() as i64 + 1..=hi.floor() as i64 {
        let jf = j as f64;
        while c[m + 1] < jf {
            m += 1;
        }
        let born = m as f64 + (jf - c[m]) / (c[m + 1] - c[m]);
        let a = (u - born) / fps;
        let s = a / life;
        let v = at_frame(m);
        let at = |n| (crate::grade::unit(base, j, 0, 0, n) + 1.0) / 2.0;
        let (px, py) = (v[2] / 100.0 * w, v[3] / 100.0 * h);
        let (rx, ry) = (v[4] / 100.0 * h, v[5] / 100.0 * h);
        let x0 = px + rx * at(0).sqrt() * (tau * at(1)).cos();
        let y0 = py + ry * at(0).sqrt() * (tau * at(1)).sin();
        let sp = v[7] / 100.0 * h * (1.0 - v[8] / 100.0 * at(3));
        let kr = v[10] / 100.0;
        let f1 = if kr == 0.0 { a } else { -(-kr * a).exp_m1() / kr };
        let f2 = if kr == 0.0 { a * a / 2.0 } else { (a - f1) / kr };
        let g = v[9] / 100.0 * h * f2;
        let (ox, oy) = (x0 - px, y0 - py);
        let (x, y) = match words[0].as_str() {
            "fixed" => (x0, y0),
            "vortex" => {
                let rad = ox.hypot(oy) + sp / 2.0 * f1;
                let ang = oy.atan2(ox) + tau * sp / h * f1;
                (px + rad * ang.cos(), py + rad * ang.sin() + g)
            }
            anim => {
                let (dx, dy) = match anim {
                    "explosive" => ((tau * at(4)).cos(), (tau * at(4)).sin()),
                    "direction" => crate::blurs::along(v[6] + (at(4) - 0.5) * 90.0 * v[8] / 100.0),
                    _ => {
                        let n = ox.hypot(oy);
                        if n > 0.0 { (-oy / n, ox / n) } else { crate::blurs::along(v[6]) }
                    }
                };
                (x0 + sp * dx * f1, y0 + sp * dy * f1 + g)
            }
        };
        let r = h / 20.0 * (v[12] / 100.0 + (v[13] - v[12]) / 100.0 * s) * mercury_share(&words[1], s);
        if r <= 0.0 {
            continue;
        }
        k.extend([3.0, x, y, r * (1.0 + big_a), r, x0, y0, at(2), 0.0, 0.0, 0.0]);
    }
    k
}

/// D-674: the numbers every pixel of Mercury shares, the card's as the processor's: the blob
/// influence as a share and the field's surface, the light's colour in linear light, its
/// intensity, ambient, diffuse, specular, metal and roughness, 1 for a point light, then the
/// point light's place in the drawing's own pixels and its height, or the distant light's
/// direction times 100 and its height. `w` and `h` the drawing's size.
pub(crate) fn mercury_light(e: &Effect, w: f64, h: f64) -> [f64; 15] {
    let Effect::Mercury { values, words, light_color, .. } = e else { unreachable!("Mercury of another effect") };
    let v = mercury_held(values);
    let lc = crate::effects::encoded(light_color).map(crate::grade::to_linear);
    let (ux, uy) = crate::blurs::along(v[18]);
    let point = words[2] == "point";
    let (lx, ly) = if point { (v[16] / 100.0 * w, v[17] / 100.0 * h) } else { (100.0 * ux, 100.0 * uy) };
    let a = v[11] / 100.0;
    let edge = 1.0 - 1.0 / ((1.0 + a) * (1.0 + a));
    [a, edge * edge, lc[0], lc[1], lc[2], v[14] / 100.0, v[19] / 100.0, v[20] / 100.0, v[21] / 100.0, v[23] / 100.0, v[22], point as u8 as f64, lx, ly, v[15]]
}

/// D-674: Mercury, after CycoreFX's CC Mr. Mercury (document 21): the layer turned into drops of
/// liquid metal born at a producer, thrown and falling, running together as metaballs and lit
/// by Drizzle's light. The layer is replaced by its drops. `(ox, oy)` the drawing's corner in the
/// buffer: the drops and the light are in the drawing's own space, every pixel of the buffer can
/// be covered, and the birthplaces read the layer clamped to the buffer. Never grows. `e`'s clock
/// and history already filled as compose fills them.
pub(crate) fn mercury(source: &mut WorkingBuffer, e: &Effect, (ox, oy): (usize, usize)) {
    let (bw, bh) = (source.width(), source.height());
    let (w0, h0) = (bw.saturating_sub(2 * ox), bh.saturating_sub(2 * oy));
    if w0 == 0 || h0 == 0 {
        return;
    }
    let k = mercury_list(e, w0 as f64, h0 as f64);
    let [_, edge, lr, lg, lb, inten, amb, dif, spc, metal, rough, point, lx, ly, lz] = mercury_light(e, w0 as f64, h0 as f64);
    let lc = [lr, lg, lb];
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
    // Each drop's straight colour where it was born, when it lives there.
    let cols: Vec<Option<[f64; 3]>> = k[HEAD..]
        .par_chunks_exact(STRIDE)
        .map(|s| {
            let c = read(s[5], s[6]);
            (s[7] < c[3]).then(|| [c[0] / c[3], c[1] / c[3], c[2] / c[3]])
        })
        .collect();
    let tw = bw.div_ceil(TILE);
    source.data_mut().par_chunks_mut(bw * 4 * TILE).enumerate().for_each(|(ty, band)| {
        for (row, line) in band.chunks_exact_mut(bw * 4).enumerate() {
            let y = (ty * TILE + row) as f64 - oy as f64 + 0.5;
            for (bx, px) in line.chunks_exact_mut(4).enumerate() {
                let t = ty * tw + bx / TILE;
                let x = bx as f64 - ox as f64 + 0.5;
                let (mut f, mut fx, mut fy, mut wt, mut cs) = (0.0, 0.0, 0.0, 0.0, [0.0; 3]);
                for &i in &list[starts[t]..starts[t + 1]] {
                    let Some(col) = cols[i] else { continue };
                    let s = &k[HEAD + STRIDE * i..HEAD + STRIDE * i + STRIDE];
                    let (dx, dy, rho) = (x - s[1], y - s[2], s[3]);
                    let q2 = (dx * dx + dy * dy) / (rho * rho);
                    if q2 >= 1.0 {
                        continue;
                    }
                    let one = 1.0 - q2;
                    let ff = one * one;
                    f += ff;
                    fx += -4.0 * one * dx / (rho * rho);
                    fy += -4.0 * one * dy / (rho * rho);
                    wt += ff * rho;
                    for ch in 0..3 {
                        cs[ch] += ff * col[ch];
                    }
                }
                let gl = fx.hypot(fy);
                let cov = if f <= 0.0 {
                    0.0
                } else if gl > 0.0 {
                    ((f - edge) / gl + 0.5).clamp(0.0, 1.0)
                } else if f > edge {
                    1.0
                } else {
                    0.0
                };
                if cov == 0.0 {
                    px.fill(0.0);
                    continue;
                }
                let col = cs.map(|c| c / f);
                let cc = wt / f;
                let nv = [-cc * fx, -cc * fy, 2.0 * (f - edge).max(0.0).sqrt()];
                let nn = if nv.iter().any(|&c| c != 0.0) { unit(nv) } else { [0.0, 0.0, 1.0] };
                let l = if point == 1.0 { unit([lx - x, ly - y, lz]) } else { distant };
                let nl = nn[0] * l[0] + nn[1] * l[1] + nn[2] * l[2];
                let rz = 2.0 * nl * nn[2] - l[2];
                let shine = if nl > 0.0 && rz > 0.0 { rz.powf(1.0 / rough) } else { 0.0 };
                for ch in 0..3 {
                    let lit = col[ch] * (amb + dif * inten * lc[ch] * nl.max(0.0)) + spc * inten * (lc[ch] + (col[ch] - lc[ch]) * metal) * shine;
                    px[ch] = (lit * cov) as f32;
                }
                px[3] = cov as f32;
            }
        }
    });
}

/// D-675: Polygon Shards' settings held inside their ranges.
fn shards_held(v: &[f64; 9]) -> [f64; 9] {
    std::array::from_fn(|k| v[k].clamp(POLYGON_SHARDS_SETTINGS[k].1, POLYGON_SHARDS_SETTINGS[k].2))
}

/// D-675: Polygon Shards' shards at this frame, in the order they are drawn, or `None` before the
/// start time, when the layer is drawn as it is. Three lists: the sprite numbers the particle
/// engine's binning reads (the head, k[2] and k[3] the drawing's size, then `STRIDE` numbers a shard,
/// `[3, x, y, reach, part, r, g, b, a, 0]`: its centre as seen and how far its corners reach from
/// it, 0 the upper right triangle, 1 the lower left, 2 a square, and its own premultiplied colour,
/// clear when textured); sixteen numbers a shard, its place P, its turn M by rows, its centre c
/// and its cell's corner; and the seven every pixel shares, the eye (a_x, a_y, A_l), the drawing's
/// size, the grid spacing and 1 when textured. `drawing` the buffer's pixels, `bw` its width,
/// `(ox, oy)` the drawing's corner in it, `w` and `h` the drawing's size. Every setting is this
/// frame's.
// ponytail: a cell or two a shard, so a grid spacing of 2 on a large layer is millions of shards;
// draw only the shards that move if small spacings grow slow.
pub(crate) fn shards_list(e: &Effect, drawing: &[f32], bw: usize, (ox, oy): (usize, usize), w: usize, h: usize) -> Option<(Vec<f64>, Vec<f64>, [f64; 7])> {
    let Effect::PolygonShards { values, words, clock: [u, fps], view } = e else { unreachable!("Polygon Shards of another effect") };
    let v = shards_held(values);
    let t = u / fps - v[8];
    if *fps <= 0.0 || !(t > 0.0) {
        return None;
    }
    let (wf, hf, s) = (w as f64, h as f64, v[7]);
    let (ax, ay, al) = if view[0] != 0.0 { (view[1], view[2], view[3]) } else { (wf / 2.0, hf / 2.0, wf / 2.0 * (1.0 / 19.8f64.to_radians().tan())) };
    let (cols, rows) = ((wf / s).ceil() as usize, (hf / s).ceil() as usize);
    let tri = words[0] == "polygon" || words[0] == "textured_polygon";
    let textured = words[0].starts_with("textured");
    let (fx, fy) = (v[3] / 100.0 * wf, v[4] / 100.0 * hf);
    let (dr, sr) = (v[5] / 100.0, v[6] / 100.0);
    let base = crate::grade::mix(0);
    let pi = std::f64::consts::PI;
    let pixel = |x: usize, y: usize| -> [f64; 4] {
        let i = ((y + oy) * bw + x + ox) * 4;
        [0, 1, 2, 3].map(|c| drawing[i + c] as f64)
    };
    let mut shards: Vec<(f64, usize, [f64; STRIDE], [f64; 16])> = Vec::new();
    for n in 0..cols * rows {
        let (x0, y0) = (s * (n % cols) as f64, s * (n / cols) as f64);
        for part in if tri { 0..2 } else { 2..3 } {
            let kk = if tri { 2 * n + part } else { n };
            let (cx, cy) = [(x0 + 2.0 * s / 3.0, y0 + s / 3.0), (x0 + s / 3.0, y0 + 2.0 * s / 3.0), (x0 + s / 2.0, y0 + s / 2.0)][part];
            // A triangle's last corner repeated, so the three kinds are four corners each.
            let corners = [
                [(x0, y0), (x0 + s, y0), (x0 + s, y0 + s), (x0 + s, y0 + s)],
                [(x0, y0), (x0 + s, y0 + s), (x0, y0 + s), (x0, y0 + s)],
                [(x0, y0), (x0 + s, y0), (x0 + s, y0 + s), (x0, y0 + s)],
            ][part];
            let uq: [f64; 7] = std::array::from_fn(|q| (crate::grade::unit(base, kk as i64, 0, 0, q as u64) + 1.0) / 2.0);
            let (dx, dy) = (cx - fx, cy - fy);
            let dist = dx.hypot(dy);
            let theta = if dist > 0.0 { dy.atan2(dx) } else { 2.0 * pi * uq[0] } + (uq[1] - 0.5) * pi * dr;
            let phi = (45.0 + (uq[2] - 0.5) * 90.0 * dr).to_radians();
            let way = [phi.cos() * theta.cos(), phi.cos() * theta.sin(), -phi.sin()];
            let sp = v[0] / 100.0 * (hf / 4.0) * (hf / (hf + dist)) * (1.0 - sr * uq[3]);
            let g = v[1] * hf;
            let p = [cx + sp * way[0] * t, cy + sp * way[1] * t + g * t * t / 2.0, sp * way[2] * t];
            let z = 2.0 * uq[4] - 1.0;
            let psi = 2.0 * pi * uq[5];
            let rr = (1.0 - z * z).max(0.0).sqrt();
            // Rodrigues' rotation as a matrix, about the unit axis (x, y, z).
            let (x, y) = (rr * psi.cos(), rr * psi.sin());
            let (si, co) = (v[2] * (0.5 + uq[6]) * t).to_radians().sin_cos();
            let c1 = 1.0 - co;
            let m = [
                [co + x * x * c1, x * y * c1 - z * si, x * z * c1 + y * si],
                [y * x * c1 + z * si, co + y * y * c1, y * z * c1 - x * si],
                [z * x * c1 - y * si, z * y * c1 + x * si, co + z * z * c1],
            ];
            if corners.iter().any(|&(px, py)| al + (p[2] + m[2][0] * (px - cx) + m[2][1] * (py - cy)) <= 1.0) {
                continue;
            }
            let see = |a: f64, b: f64| {
                let q = [0, 1, 2].map(|r| p[r] + m[r][0] * a + m[r][1] * b);
                let f = al / (al + q[2]);
                (ax + (q[0] - ax) * f, ay + (q[1] - ay) * f)
            };
            let (sx, sy) = see(0.0, 0.0);
            // Two pixels past the farthest corner: the edge is a pixel soft, more past a corner.
            let reach = corners
                .iter()
                .map(|&(px, py)| {
                    let (qx, qy) = see(px - cx, py - cy);
                    (qx - sx).hypot(qy - sy)
                })
                .fold(0.0, f64::max)
                + 2.0;
            let colour = if textured {
                [0.0; 4]
            } else {
                let (mut acc, mut count) = ([0.0; 4], 0usize);
                for yy in y0 as usize..h.min((y0 + s) as usize + 1) {
                    for xx in x0 as usize..w.min((x0 + s) as usize + 1) {
                        let (a, b) = (xx as f64 + 0.5 - x0, yy as f64 + 0.5 - y0);
                        if !(0.0..=s).contains(&a) || !(0.0..=s).contains(&b) || (part == 0 && !(b < a)) || (part == 1 && !(b >= a)) {
                            continue;
                        }
                        let c = pixel(xx, yy);
                        for q in 0..4 {
                            acc[q] += c[q];
                        }
                        count += 1;
                    }
                }
                if count > 0 { acc.map(|a| a / count as f64) } else { [0.0; 4] }
            };
            let mut block = [0.0; 16];
            block[..3].copy_from_slice(&p);
            block[3..12].copy_from_slice(&m.concat());
            block[12..].copy_from_slice(&[cx, cy, x0, y0]);
            shards.push((p[2], kk, [3.0, sx, sy, reach, part as f64, colour[0], colour[1], colour[2], colour[3], 0.0, 0.0], block));
        }
    }
    if words[1] == "on" {
        // Far to near, then by number; partial_cmp so that 0 and -0 are one place.
        shards.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal).then(a.1.cmp(&b.1)));
    }
    let mut k = vec![0.0; HEAD];
    k[2] = wf;
    k[3] = hf;
    let mut blocks = Vec::with_capacity(16 * shards.len());
    for (_, _, sprite, block) in &shards {
        k.extend(sprite);
        blocks.extend(block);
    }
    Some((k, blocks, [ax, ay, al, wf, hf, s, textured as u8 as f64]))
}

/// D-675: Polygon Shards, after CycoreFX's CC Pixel Polly (document 21): the layer cut along a
/// grid into triangles or squares thrown away from a force centre in 3D, falling and spinning,
/// seen through the composition camera or the effect's own eye. The layer is replaced by its
/// shards from the start time, and drawn as it is before. `(ox, oy)` the drawing's corner in the
/// buffer: the shards are in the drawing's own space, every pixel of the buffer can be covered,
/// and a textured shard reads the drawing clamped to the drawing, as Bubbles does. Never grows.
/// `e`'s clock and view already filled as compose fills them.
pub(crate) fn polygon_shards(source: &mut WorkingBuffer, e: &Effect, (ox, oy): (usize, usize)) {
    let (bw, bh) = (source.width(), source.height());
    let (w0, h0) = (bw.saturating_sub(2 * ox), bh.saturating_sub(2 * oy));
    if w0 == 0 || h0 == 0 {
        return;
    }
    let Some((k, blocks, [ax, ay, al, wf, hf, s, textured])) = shards_list(e, source.data(), bw, (ox, oy), w0, h0) else { return };
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
    let r2 = std::f64::consts::SQRT_2;
    let tw = bw.div_ceil(TILE);
    source.data_mut().par_chunks_mut(bw * 4 * TILE).enumerate().for_each(|(ty, band)| {
        for (row, line) in band.chunks_exact_mut(bw * 4).enumerate() {
            let y = (ty * TILE + row) as f64 - oy as f64 + 0.5;
            for (bx, px) in line.chunks_exact_mut(4).enumerate() {
                let t = ty * tw + bx / TILE;
                let x = bx as f64 - ox as f64 + 0.5;
                let (rx, ry) = (x - ax, y - ay);
                let rl = (rx * rx + ry * ry + al * al).sqrt();
                let mut o = [0.0f64; 4];
                for &n in &list[starts[t]..starts[t + 1]] {
                    let sp = &k[HEAD + STRIDE * n..HEAD + STRIDE * n + STRIDE];
                    let b = &blocks[16 * n..16 * n + 16];
                    let (p, m) = (&b[..3], &b[3..12]);
                    let den = m[2] * rx + m[5] * ry + m[8] * al;
                    if !(den.abs() > 1e-9 * rl) {
                        continue;
                    }
                    let lam = (m[2] * (p[0] - ax) + m[5] * (p[1] - ay) + m[8] * (p[2] + al)) / den;
                    if !(lam > 0.0) {
                        continue;
                    }
                    let q = [ax + lam * rx, ay + lam * ry, -al + lam * al];
                    let d = [q[0] - p[0], q[1] - p[1], q[2] - p[2]];
                    let (x0, y0) = (b[12] + (m[0] * d[0] + m[3] * d[1] + m[6] * d[2]), b[13] + (m[1] * d[0] + m[4] * d[1] + m[7] * d[2]));
                    let (a, bb) = (x0 - b[14], y0 - b[15]);
                    let inside = match sp[4] as u8 {
                        0 => bb.min(s - a).min((a - bb) / r2),
                        1 => a.min(s - bb).min((bb - a) / r2),
                        _ => a.min(bb).min(s - a).min(s - bb),
                    }
                    .min(x0)
                    .min(y0)
                    .min(wf - x0)
                    .min(hf - y0);
                    let cov = (inside * al / (al + q[2]) + 0.5).clamp(0.0, 1.0);
                    if cov == 0.0 {
                        continue;
                    }
                    let col = if textured == 1.0 { read(x0, y0) } else { [sp[5], sp[6], sp[7], sp[8]] };
                    o = [0, 1, 2, 3].map(|c| col[c] * cov + o[c] * (1.0 - col[3] * cov));
                }
                for (v, o) in px.iter_mut().zip(o) {
                    *v = o as f32;
                }
            }
        }
    });
}

/// D-678: Foam's settings held inside their ranges.
fn foam_held(v: &[f64; 26]) -> [f64; 26] {
    std::array::from_fn(|k| v[k].clamp(FOAM_SETTINGS[k].1, FOAM_SETTINGS[k].2))
}

/// D-678: the way `deg` degrees points, After Effects' way (0 up, clockwise), worked as the
/// reference works it (not [`crate::blurs::along`], which rounds the four quarters).
fn foam_way(deg: f64) -> (f64, f64) {
    let a = deg.to_radians();
    (a.sin(), -a.cos())
}

/// D-678: the producer's centre and half sizes in the simulation's own space, from settings
/// already held; `on` whether Zoom Producer Point is on.
fn foam_producer(v: &[f64; 26], on: bool, w: f64, h: f64) -> (f64, f64, f64, f64) {
    let (mut px, mut py) = (v[0] / 100.0 * w, v[1] / 100.0 * h);
    let (mut a, mut b) = (v[2] * w / 2.0, v[3] * h / 2.0);
    if !on {
        let z = v[21];
        px = w / 2.0 + (px - w / 2.0) / z;
        py = h / 2.0 + (py - h / 2.0) / z;
        a /= z;
        b /= z;
    }
    (px, py, a, b)
}

/// D-678: a bubble as it moves: its birth time, place, velocity, full radius, life and
/// its seven randoms.
struct Foamed {
    b: f64,
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    r: f64,
    life: f64,
    u: [f64; 7],
}

impl Foamed {
    fn radius(&self, t: f64, g: f64) -> f64 {
        self.r * ((t - self.b) * g * 10.0).min(1.0)
    }
}

/// D-678: the places in `alive` that pop and those that leave the universe at `t`.
fn foam_gone(alive: &[Foamed], t: f64, v: &[f64; 26], w: f64, h: f64) -> (Vec<usize>, Vec<usize>) {
    let (mut pops, mut leaves) = (Vec::new(), Vec::new());
    for (i, b) in alive.iter().enumerate() {
        let r = b.radius(t, v[9]);
        if t - b.b >= b.life {
            pops.push(i);
        } else if (b.x - w / 2.0).abs() - r > v[22] * w / 2.0 || (b.y - h / 2.0).abs() - r > v[22] * h / 2.0 {
            leaves.push(i);
        }
    }
    (pops, leaves)
}

/// D-678: Foam's bubbles alive at this frame in number order, stepped from the in point
/// (document 20), and the time drawn. `w` and `h` the drawing's size.
// ponytail: O(frames since the in point x bubbles x neighbours) a frame, the neighbours found
// in a grid; keep the bubbles between frames if long shots grow slow.
fn foam_simulate(e: &Effect, w: f64, h: f64) -> (Vec<Foamed>, f64) {
    let Effect::Foam { values, words, clock: [u, fps], history } = e else { unreachable!("Foam of another effect") };
    let history = history.as_deref().map(|h| h.as_slice());
    let at_frame = |i: usize| foam_held(history.map_or(values, |h| &h[i.min(h.len() - 1)]));
    let now = foam_held(values);
    let on = words[1] == "on";
    let q = match words[5].as_str() {
        "high" => 2,
        "intense" => 4,
        _ => 1,
    };
    let (fps, uf) = (*fps, *u);
    let steps = if fps > 0.0 && uf > 0.0 { uf as usize * q } else { 0 };
    let dt = 1.0 / (fps * q as f64);
    let base = crate::grade::mix(now[25].floor() as u64);
    let mut alive: Vec<Foamed> = Vec::new();
    let (mut born, mut total) = (0i64, 0.0);
    for m in 0..steps {
        let v = at_frame(m / q);
        let t = m as f64 / (fps * q as f64);
        let g = v[9];
        let (pops, leaves) = foam_gone(&alive, t, &v, w, h);
        let mut out = vec![false; alive.len()];
        for &i in pops.iter().chain(&leaves) {
            out[i] = true;
        }
        for &j in &pops {
            let (xj, yj, rj) = (alive[j].x, alive[j].y, alive[j].radius(t, g));
            for i in 0..alive.len() {
                if out[i] {
                    continue;
                }
                let bi = &mut alive[i];
                let d2 = 2.0 * (bi.radius(t, g) + rj);
                let (dx, dy) = (bi.x - xj, bi.y - yj);
                let d = (dx * dx + dy * dy).sqrt();
                if d2 > 0.0 && 0.0 < d && d < d2 {
                    let k = v[18] * h / 4.0 * (1.0 - d / d2) / d;
                    bi.vx += k * dx;
                    bi.vy += k * dy;
                }
            }
        }
        let mut n = 0;
        alive.retain(|_| {
            n += 1;
            !out[n - 1]
        });
        total += 30.0 * v[5] * dt;
        let (px, py, a, b) = foam_producer(&v, on, w, h);
        let (co, si) = (v[4].to_radians().cos(), v[4].to_radians().sin());
        while born as f64 + 0.5 <= total {
            let uq: [f64; 7] = std::array::from_fn(|n| (crate::grade::unit(base, born, n as i64, 0, 0) + 1.0) / 2.0);
            let (rho, psi) = (uq[0].sqrt(), 2.0 * std::f64::consts::PI * uq[1]);
            let (lx, ly) = (rho * psi.cos() * a, rho * psi.sin() * b);
            let (sx, sy) = foam_way(v[12]);
            let sp = v[11] * h / 4.0;
            alive.push(Foamed {
                b: t,
                x: px + (lx * co - ly * si),
                y: py + (lx * si + ly * co),
                vx: sp * sx,
                vy: sp * sy,
                r: (v[6] * (h / 40.0) * (1.0 + v[7] * (2.0 * uq[2] - 1.0))).max(0.0),
                life: if v[10] > 0.0 { v[8] / 30.0 * uq[3].powf(1.0 / v[10]) } else { 0.0 },
                u: uq,
            });
            born += 1;
        }
        let (wx, wy) = foam_way(v[14]);
        let wind = v[13] * h / 4.0;
        let tu = v[15] * h / 2.0;
        let l = h / 4.0;
        let kmax = 0.5 / (dt * dt);
        // The neighbours within 1.5 (r_i + r_j) lie in the cells round a bubble's own, each cell
        // a hair wider than that reach can be; each bubble's are summed in number order, as the
        // reference sums every other bubble.
        let radii: Vec<f64> = alive.iter().map(|b| b.radius(t, g)).collect();
        let cell = 3.0 * (1.0 + 1e-9) * radii.iter().fold(0.0f64, |m, &r| m.max(r));
        let key = |b: &Foamed| ((b.x / cell).floor().clamp(-1e15, 1e15) as i64, (b.y / cell).floor().clamp(-1e15, 1e15) as i64);
        let mut grid: Vec<((i64, i64), usize)> = if cell > 0.0 { alive.iter().enumerate().map(|(i, b)| (key(b), i)).collect() } else { Vec::new() };
        grid.sort_unstable();
        let acc: Vec<(f64, f64)> = (0..alive.len())
            .into_par_iter()
            .map(|i| {
                let bi = &alive[i];
                let ri = radii[i];
                let mut ax = wind * wx + tu * crate::grade::value(base, 300, bi.x / l, bi.y / l, t, false, 0);
                let mut ay = wind * wy + tu * crate::grade::value(base, 301, bi.x / l, bi.y / l, t, false, 0);
                let (mut ws, mut vsx, mut vsy) = (0.0, 0.0, 0.0);
                let mut near = Vec::new();
                if cell > 0.0 {
                    let (cx, cy) = key(bi);
                    for gx in cx - 1..=cx + 1 {
                        for gy in cy - 1..=cy + 1 {
                            let lo = grid.partition_point(|e| e.0 < (gx, gy));
                            near.extend(grid[lo..].iter().take_while(|e| e.0 == (gx, gy)).map(|e| e.1));
                        }
                    }
                    near.sort_unstable();
                }
                for &j in &near {
                    if j == i {
                        continue;
                    }
                    let bj = &alive[j];
                    let s = ri + radii[j];
                    let (dx, dy) = (bi.x - bj.x, bi.y - bj.y);
                    let d = (dx * dx + dy * dy).sqrt();
                    if s > 0.0 && 0.0 < d && d < 1.5 * s {
                        let wt = 1.0 - d / (1.5 * s);
                        ws += wt;
                        vsx += wt * bj.vx;
                        vsy += wt * bj.vy;
                        if d < s {
                            let k = (v[17] * 4.0 * h / s).min(kmax) * (s - d) / d;
                            ax += k * dx;
                            ay += k * dy;
                        }
                    }
                }
                if ws > 0.0 {
                    let rate = (v[20] * 10.0 * ws).min(0.5 / dt);
                    ax += rate * (vsx / ws - bi.vx);
                    ay += rate * (vsy / ws - bi.vy);
                }
                (ax, ay)
            })
            .collect();
        let drag = (-10.0 * v[19] * dt).exp();
        for (b, (ax, ay)) in alive.iter_mut().zip(acc) {
            b.vx = (b.vx + ax * dt) * drag;
            b.vy = (b.vy + ay * dt) * drag;
            b.x += b.vx * dt;
            b.y += b.vy * dt;
        }
    }
    let tt = uf / fps;
    let (pops, leaves) = foam_gone(&alive, tt, &now, w, h);
    let mut out = vec![false; alive.len()];
    for &i in pops.iter().chain(&leaves) {
        out[i] = true;
    }
    let mut n = 0;
    alive.retain(|_| {
        n += 1;
        !out[n - 1]
    });
    (alive, tt)
}

/// D-678: Foam's bubbles at this frame as sprite numbers the particle engine's binning reads, in
/// the order they are laid down: the head left at zero, then `STRIDE` numbers a bubble, `[3, x, y,
/// reach, R, cos theta, sin theta, cos 2 alpha, sin 2 alpha, 0]` on screen in the drawing's own
/// pixels, and the twelve numbers every pixel shares: the view (1 rendered), the blend (1 over),
/// the texture's place in [`crate::effects::FOAM_WORDS`], the wobble, the reflection's strength
/// and convergence, the producer's centre and half sizes on screen and its turn's cosine and
/// sine. `w` and `h` the drawing's size.
pub(crate) fn foam_list(e: &Effect, w: f64, h: f64) -> (Vec<f64>, [f64; 12]) {
    let Effect::Foam { values, words, .. } = e else { unreachable!("Foam of another effect") };
    let (alive, tt) = foam_simulate(e, w, h);
    let v = foam_held(values);
    let z = v[21];
    let (cx, cy) = (w / 2.0, h / 2.0);
    let o = v[16];
    let mut k = vec![0.0; HEAD];
    for b in &alive {
        let r = z * b.radius(tt, v[9]);
        if !(r > 0.0) {
            continue;
        }
        let age = tt - b.b;
        let th = match words[4].as_str() {
            "physical" => ((2.0 * b.u[5] - 1.0) * 180.0 * age).to_radians(),
            "velocity" if b.vx != 0.0 || b.vy != 0.0 => b.vx.atan2(-b.vy),
            _ => 0.0,
        };
        let al = th + 2.0 * std::f64::consts::PI * (b.u[4] + (0.5 + b.u[6]) * age);
        k.extend([3.0, cx + z * (b.x - cx), cy + z * (b.y - cy), r * (1.0 + o), r, th.cos(), th.sin(), (2.0 * al).cos(), (2.0 * al).sin(), 0.0, 0.0]);
    }
    let rendered = words[0] == "rendered";
    if rendered && words[2] == "solid_old_on_top" {
        let n = (k.len() - HEAD) / STRIDE;
        let sprites = k.split_off(HEAD);
        for i in (0..n).rev() {
            k.extend_from_slice(&sprites[STRIDE * i..STRIDE * i + STRIDE]);
        }
    }
    let (px, py, a, b) = foam_producer(&v, words[1] == "on", w, h);
    let po = v[4].to_radians();
    let texture = crate::effects::FOAM_WORDS[3].1.iter().position(|t| *t == words[3]).unwrap_or(0);
    let shared = [
        rendered as u8 as f64,
        (words[2] != "transparent") as u8 as f64,
        texture as f64,
        o,
        v[23],
        v[24],
        cx + z * (px - cx),
        cy + z * (py - cy),
        (z * a).max(0.5),
        (z * b).max(0.5),
        po.cos(),
        po.sin(),
    ];
    (k, shared)
}

/// D-678: one rendered bubble's colour, premultiplied, at the offset (dx, dy) from its centre
/// on screen `(sx, sy)`, `rw` its radius that way and `e` the offset's length, by the texture's
/// place `tex`, with the reflection; `read` the bilinear read of the drawing at an own point.
fn foam_colour(s: &[f64], dx: f64, dy: f64, e: f64, rw: f64, sh: &[f64; 12], w: f64, h: f64, read: &impl Fn(f64, f64) -> [f64; 4]) -> [f64; 4] {
    let (cth, sth) = (s[5], s[6]);
    let qq = e / rw;
    let cut = qq.max(1.0);
    let ux = (dx * cth + dy * sth) / rw / cut;
    let uy = (-dx * sth + dy * cth) / rw / cut;
    let qq = qq.min(1.0);
    let mut col = if sh[2] == 4.0 {
        read((ux + 1.0) * w / 2.0, (uy + 1.0) * h / 2.0)
    } else {
        let hl = (1.0 - ((ux + 0.35).powf(2.0) + (uy + 0.35).powf(2.0)) / 0.0625).max(0.0).powf(2.0);
        let (a, s) = match sh[2] as u8 {
            0 => (0.12 + 0.68 * qq.powf(4.0), [0.85 + 0.15 * qq * qq, 0.92, 1.0 - 0.15 * qq * qq]),
            1 => (0.45 + 0.45 * qq * qq, [0.95, 0.95, 0.93]),
            2 => {
                let lit = 0.5 + 0.5 * (-0.4 * ux - 0.4 * uy + 0.824 * (1.0 - qq * qq).max(0.0).sqrt()).max(0.0);
                (0.95, [1.0 * lit, 0.45 * lit, 0.7 * lit])
            }
            _ => (0.3 + 0.45 * qq.powf(3.0), [0.72, 0.68, 0.58]),
        };
        let a = a + (1.0 - a) * 0.85 * hl;
        let s = s.map(|x| x + (1.0 - x) * hl);
        [s[0] * a, s[1] * a, s[2] * a, a]
    };
    let (fr, cv) = (sh[4], sh[5]);
    if fr > 0.0 {
        let ex = (1.0 - cv) * (s[1] + 2.0 * dx / cut) + cv * ((ux + 1.0) * w / 2.0);
        let ey = (1.0 - cv) * (s[2] + 2.0 * dy / cut) + cv * ((uy + 1.0) * h / 2.0);
        let ee = read(ex, ey);
        for q in 0..3 {
            col[q] = col[q] * (1.0 - fr) + ee[q] * col[3] * fr;
        }
    }
    col
}

/// D-678: Foam, after After Effects' Foam (document 21): the layer becomes its bubbles alone,
/// laid onto clear, as rings (Draft, with the producer in red) or textured bubbles (Rendered).
/// `(ox, oy)` the drawing's corner in the buffer; the bubbles stay in the drawing's own space and
/// read only the drawing. Never grows. `e`'s clock and history already filled as compose fills
/// them.
pub(crate) fn foam(source: &mut WorkingBuffer, e: &Effect, (ox, oy): (usize, usize)) {
    let (bw, bh) = (source.width(), source.height());
    let (w0, h0) = (bw.saturating_sub(2 * ox), bh.saturating_sub(2 * oy));
    if w0 == 0 || h0 == 0 {
        return;
    }
    let (w, h) = (w0 as f64, h0 as f64);
    let (k, sh) = foam_list(e, w, h);
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
    let over = |o: [f64; 4], c: [f64; 4], cov: f64| -> [f64; 4] { std::array::from_fn(|q| c[q] * cov + o[q] * (1.0 - c[3] * cov)) };
    let tw = bw.div_ceil(TILE);
    source.data_mut().par_chunks_mut(bw * 4 * TILE).enumerate().for_each(|(ty, band)| {
        for (row, line) in band.chunks_exact_mut(bw * 4).enumerate() {
            let y = (ty * TILE + row) as f64 - oy as f64 + 0.5;
            for (bx, px) in line.chunks_exact_mut(4).enumerate() {
                let t = ty * tw + bx / TILE;
                let x = bx as f64 - ox as f64 + 0.5;
                let mut acc = [0.0f64; 4];
                if sh[0] == 0.0 {
                    let (lx, ly) = ((x - sh[6]) * sh[10] + (y - sh[7]) * sh[11], -(x - sh[6]) * sh[11] + (y - sh[7]) * sh[10]);
                    let kk = ((lx / sh[8]).powf(2.0) + (ly / sh[9]).powf(2.0)).sqrt();
                    let g2 = ((lx / (sh[8] * sh[8])).powf(2.0) + (ly / (sh[9] * sh[9])).powf(2.0)).sqrt();
                    if g2 > 0.0 {
                        acc = over(acc, [1.0, 0.15, 0.1, 1.0], (1.0 - ((kk * kk - kk) / g2).abs()).clamp(0.0, 1.0));
                    }
                }
                for &n in &list[starts[t]..starts[t + 1]] {
                    let s = &k[HEAD + STRIDE * n..HEAD + STRIDE * n + STRIDE];
                    let (dx, dy) = (x - s[1], y - s[2]);
                    let e2 = dx * dx + dy * dy;
                    let e = e2.sqrt();
                    let rw = if e2 > 0.0 { s[4] * (1.0 + sh[3] * (s[7] * (dx * dx - dy * dy) + s[8] * 2.0 * dx * dy) / e2) } else { s[4] };
                    if sh[0] == 0.0 {
                        let cov = (1.0 - (e - rw).abs()).clamp(0.0, 1.0);
                        if cov > 0.0 {
                            acc = over(acc, [0.2, 0.5, 1.0, 1.0], cov);
                        }
                        continue;
                    }
                    let cov = (rw - e + 0.5).clamp(0.0, 1.0);
                    // A wobble of 1 can squash a bubble to nothing one way; nothing is drawn there.
                    if cov == 0.0 || !(rw > 0.0) {
                        continue;
                    }
                    let col = foam_colour(s, dx, dy, e, rw, &sh, w, h, &read);
                    acc = if sh[1] == 0.0 { std::array::from_fn(|q| acc[q] + col[q] * cov - acc[q] * col[q] * cov) } else { over(acc, col, cov) };
                }
                for (v, o) in px.iter_mut().zip(acc) {
                    *v = o as f32;
                }
            }
        }
    });
}

/// D-676: Scatterize's dots on a drawing `w` by `h`, one for every pixel, farthest first: each
/// `(x, y, n)`, where its pixel n = j w + i ends after the twist and the scatter, in the drawing's
/// own pixels. Which dots show is left to the drawing (a clear pixel's dot changes nothing in any
/// mode), so the card's list needs only the settings.
fn scatter_dots(e: &Effect, w: usize, h: usize) -> Vec<(f64, f64, usize)> {
    let Effect::Scatterize { values, .. } = e else { unreachable!("Scatterize of another effect") };
    let v: [f64; 3] = std::array::from_fn(|k| values[k].clamp(SCATTERIZE_SETTINGS[k].1, SCATTERIZE_SETTINGS[k].2));
    let (wf, hf) = (w as f64, h as f64);
    let big = v[0] * hf / 1080.0;
    let base = crate::grade::mix(0);
    // ponytail: every pixel a dot, sorted, about 64 MB and a sort at 1920 by 1080; the twist alone
    // sets the order, so an untwisted layer could skip the sort if it shows in the timings.
    let mut d: Vec<(f64, usize, f64, f64)> = (0..w * h)
        .into_par_iter()
        .map(|n| {
            let (i, j) = (n % w, n / w);
            let (x, y) = (i as f64 + 0.5, j as f64 + 0.5);
            let a = (v[2] + (v[1] - v[2]) * x / wf).to_radians();
            let yt = hf / 2.0 + (y - hf / 2.0) * a.cos();
            let z = (y - hf / 2.0) * a.sin();
            let u = |k| (crate::grade::unit(base, i as i64, j as i64, 0, k) + 1.0) / 2.0;
            let (r, t) = (big * u(0), 2.0 * std::f64::consts::PI * u(1));
            (z, n, x + r * t.cos(), yt + r * t.sin())
        })
        .collect();
    d.par_sort_unstable_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
    d.into_iter().map(|(_, n, x, y)| (x, y, n)).collect()
}

/// D-676: the first and last pixel, across or down a drawing `size` long, whose centre can be
/// within 1 of `p`, or None when none of the drawing's are.
fn scatter_reach(p: f64, size: usize) -> Option<(usize, usize)> {
    let (a, b) = ((p - 1.5).floor().max(0.0), ((p + 0.5).floor() + 1.0).min(size as f64 - 1.0));
    (a <= b).then_some((a as usize, b as usize))
}

/// D-676: one dot `s` (premultiplied colour times its cover) laid onto `d` by the transfer mode,
/// its place in [`crate::effects::SCATTERIZE_WORDS`].
fn scatter_lay(s: [f64; 4], d: [f64; 4], mode: usize) -> [f64; 4] {
    match mode {
        1 => [0, 1, 2, 3].map(|c| s[c] + d[c] - s[c] * d[c]),
        2 => [s[0] + d[0], s[1] + d[1], s[2] + d[2], s[3] + d[3] * (1.0 - s[3])],
        3 => [s[0] + d[0], s[1] + d[1], s[2] + d[2], (s[3] + d[3]).min(1.0)],
        _ => [0, 1, 2, 3].map(|c| s[c] + d[c] * (1.0 - s[3])),
    }
}

fn scatter_mode(e: &Effect) -> usize {
    let Effect::Scatterize { words, .. } = e else { unreachable!("Scatterize of another effect") };
    crate::effects::SCATTERIZE_WORDS[0].1.iter().position(|w| *w == words[0]).unwrap_or(0)
}

/// D-676: the card's list for Scatterize on a drawing `w0` by `h0` at `(ox, oy)` in a buffer
/// `bw` by `bh`: k[1] the transfer mode, k[2] and k[3] the drawing's size, k[6] the count; the
/// dots from 7, three each, `[x, y, n]`, farthest first; then each 16 by 16 tile's start and the
/// lists, every dot in each tile it can cover, in order.
pub(crate) fn scatterize_list(e: &Effect, (w0, h0): (usize, usize), (ox, oy): (usize, usize), (bw, bh): (usize, usize)) -> Vec<f64> {
    let dots = scatter_dots(e, w0, h0);
    let (tw, th) = (bw.div_ceil(TILE), bh.div_ceil(TILE));
    let tiles = |&(x, y, _): &(f64, f64, usize)| {
        let ((x0, x1), (y0, y1)) = (scatter_reach(x, w0)?, scatter_reach(y, h0)?);
        Some(((x0 + ox) / TILE..=(x1 + ox) / TILE, (y0 + oy) / TILE..=(y1 + oy) / TILE))
    };
    let mut starts = vec![0usize; tw * th + 1];
    for (cols, rows) in dots.iter().filter_map(tiles) {
        for t in rows.flat_map(|ty| cols.clone().map(move |tx| ty * tw + tx)) {
            starts[t + 1] += 1;
        }
    }
    for t in 0..tw * th {
        starts[t + 1] += starts[t];
    }
    let mut list = vec![0.0; starts[tw * th]];
    let mut fill = starts.clone();
    for (n, d) in dots.iter().enumerate() {
        if let Some((cols, rows)) = tiles(d) {
            for t in rows.flat_map(|ty| cols.clone().map(move |tx| ty * tw + tx)) {
                list[fill[t]] = n as f64;
                fill[t] += 1;
            }
        }
    }
    let mut k = vec![0.0; HEAD];
    k[1] = scatter_mode(e) as f64;
    k[2] = w0 as f64;
    k[3] = h0 as f64;
    k[6] = dots.len() as f64;
    k.reserve(3 * dots.len() + starts.len() + list.len());
    k.extend(dots.iter().flat_map(|&(x, y, n)| [x, y, n as f64]));
    k.extend(starts.iter().map(|&s| s as f64));
    k.extend(list);
    k
}

/// D-676: Scatterize, after CycoreFX's CC Scatterize (document 21): every pixel of the drawing
/// that shows a dot, twisted about the middle row and thrown away from where it was, laid farthest
/// first onto clear by the transfer mode. `(ox, oy)` the drawing's corner in the buffer; the dots
/// are cut at the drawing's edges and the rest of the buffer is clear. Never grows.
// ponytail: the processor lays the dots one after another on one thread, a few tens of
// milliseconds at 1920 by 1080; the card's tile lists are the parallel way if it matters.
pub(crate) fn scatterize(source: &mut WorkingBuffer, e: &Effect, (ox, oy): (usize, usize)) {
    let (bw, bh) = (source.width(), source.height());
    let (w0, h0) = (bw.saturating_sub(2 * ox), bh.saturating_sub(2 * oy));
    if w0 == 0 || h0 == 0 {
        return;
    }
    let mode = scatter_mode(e);
    let drawing = source.data();
    let mut out = vec![[0.0f64; 4]; bw * bh];
    for (x, y, n) in scatter_dots(e, w0, h0) {
        let at = ((n / w0 + oy) * bw + n % w0 + ox) * 4;
        let col: [f64; 4] = std::array::from_fn(|c| drawing[at + c] as f64);
        if col[3] <= 0.0 {
            continue;
        }
        let (Some((x0, x1)), Some((y0, y1))) = (scatter_reach(x, w0), scatter_reach(y, h0)) else { continue };
        for py in y0..=y1 {
            for px in x0..=x1 {
                let c = (1.0 - (px as f64 + 0.5 - x).hypot(py as f64 + 0.5 - y)).clamp(0.0, 1.0);
                if c > 0.0 {
                    let o = &mut out[(py + oy) * bw + px + ox];
                    *o = scatter_lay(col.map(|v| v * c), *o, mode);
                }
            }
        }
    }
    for (px, o) in source.data_mut().chunks_exact_mut(4).zip(out) {
        for (v, o) in px.iter_mut().zip(o) {
            *v = o as f32;
        }
    }
}
