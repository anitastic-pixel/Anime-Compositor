//! D-353 (EFFECTS.md P0-21): the shared soft-glow engine, and pick #1, Soft Physical Glow, the
//! "physical" falloff of Glow. Our own reading of `docs/effects/PLUGINS.md` section 2.3 and
//! section 5: a light picked by a soft threshold, spread by several Gaussian sizes a doubling
//! apart added in linear light (an exponential falloff whose count follows Radius), times
//! Exposure, then laid back with the untouched layer. Document 21 has the rule in words;
//! `tools/soft_glow_reference.py` works it a second way. No plugin code or presets are used.
//!
//! The pieces are kept apart so the picks that share the engine (#6, #10, #11, #13, #14) can call
//! them: [`plan`] (the levels, their cells and kernels, and how far the layer grows, for the CPU,
//! the card and the layer's bounds alike), [`light`], [`spread`] and [`finish`].

use crate::WorkingBuffer;
use rayon::prelude::*;

/// One level of the spread: a Gaussian of size sigma worked on cells of `d` by `d` pixels as
/// two one-way passes. The first runs along x (or along y when `first_y`) with `first`; the
/// second along the other axis with `second`, reading `mu` cells aside per step between two
/// cells. `reach` is how many cells it spreads past its own, across and down.
pub(crate) struct Level {
    pub weight: f32,
    pub d: usize,
    pub first: Vec<f32>,
    pub first_y: bool,
    pub second: Vec<f32>,
    pub mu: f64,
    pub reach: (usize, usize),
}

/// The levels, the light's own share, and how far the layer grows on each side.
pub(crate) struct Plan {
    pub own: f32,
    pub levels: Vec<Level>,
    pub grow: usize,
}

/// The settings, already inside their ranges.
pub(crate) struct Settings {
    pub luminance: bool,
    pub threshold: f64,
    pub smooth: f64,
    pub bias: f64,
    pub radius: f64,
    pub exposure: f64,
    pub aspect: f64,
    pub angle: f64,
    pub screen: bool,
    pub opacity: f64,
    pub unmult: bool,
}

/// The largest power of two `d` with 8 d <= sigma, at least 1: a level is always blurred at 4 to
/// 8 cells, so the step as it moves to coarser cells stays small (document 21).
fn cell(sigma: f64) -> usize {
    let mut d = 1;
    while 8.0 * d as f64 <= sigma {
        d *= 2;
    }
    d
}

/// Document 21's normalised taps for a variance (one tap at 0).
fn taps(var: f64) -> Vec<f32> {
    if var > 0.0 { crate::effects::reach_weights(var.sqrt(), false) } else { vec![1.0] }
}

/// The levels for a Radius, Aspect Ratio and Angle. sigma_k = Radius / 3 / 2^k while the level
/// counts at all; each fades in as clamp(2 sigma - 1, 0, 1), the weights sum to one with the
/// light's own share.
pub(crate) fn plan(radius: f64, aspect: f64, angle: f64) -> Plan {
    let s = radius / 3.0;
    let mut found = Vec::new();
    let mut k = 0;
    loop {
        let sigma = s / 2f64.powi(k);
        let f = (2.0 * sigma - 1.0).clamp(0.0, 1.0);
        if f == 0.0 || f.is_nan() {
            break;
        }
        found.push((sigma, f));
        k += 1;
    }
    let total: f64 = found.iter().map(|(_, f)| f).sum();
    let (co, si) = (angle.to_radians().cos(), angle.to_radians().sin());
    let mut grow = 0;
    let levels = found
        .into_iter()
        .map(|(sigma, f)| {
            let d = cell(sigma);
            let (su, sv) = (sigma * aspect / d as f64, sigma * (2.0 - aspect) / d as f64);
            let a = su * su * co * co + sv * sv * si * si;
            let b = (su * su - sv * sv) * co * si;
            let c = su * su * si * si + sv * sv * co * co;
            let first_y = c < a;
            let (rest, second, mu) = if first_y { (c - b * b / a, a, b / a) } else { (a - b * b / c, c, b / c) };
            let (first, second) = (taps(rest.max(0.0)), taps(second));
            let (r1, r2) = (first.len() / 2, second.len() / 2);
            let aside = r1 + (mu.abs() * r2 as f64).ceil() as usize + usize::from(mu != 0.0);
            let reach = if first_y { (r2, aside) } else { (aside, r2) };
            grow = grow.max(d * (reach.0.max(reach.1) + 2));
            Level { weight: (f / total.max(1.0)) as f32, d, first, first_y, second, mu, reach }
        })
        .collect();
    Plan { own: (1.0 - total).max(0.0) as f32, levels, grow }
}

/// Stage 1: the light that glows from one premultiplied pixel. Threshold Mode chroma tests each
/// channel, luminance one Rec. 709 brightness; Saturation Bias moves the tested value towards the
/// saturation or away; at or over the threshold it glows fully, inside Smooth in proportion.
pub(crate) fn light(px: &[f32], s: &Settings) -> [f32; 3] {
    let rgb = [px[0], px[1], px[2]];
    let t = (s.threshold / 100.0) as f32;
    if t == 0.0 {
        return rgb;
    }
    let (hi, lo) = (rgb[0].max(rgb[1]).max(rgb[2]), rgb[0].min(rgb[1]).min(rgb[2]));
    let sat = if hi > 0.0 { (hi - lo) / hi } else { 0.0 };
    let tested = if s.luminance {
        [0.2126 * rgb[0] + 0.7152 * rgb[1] + 0.0722 * rgb[2]; 3]
    } else {
        rgb
    };
    let b = (s.bias / 100.0) as f32;
    let m = (s.smooth / 100.0) as f32;
    let mut out = rgb;
    for (o, v) in out.iter_mut().zip(tested) {
        let v = if b > 0.0 {
            (1.0 - b) * v + b * sat
        } else if b < 0.0 {
            (1.0 + b) * v - b * (1.0 - sat)
        } else {
            v
        };
        let w = if m == 0.0 {
            if v >= t { 1.0 } else { 0.0 }
        } else {
            ((v - t * (1.0 - m)) / (t * m)).clamp(0.0, 1.0)
        };
        *o *= w;
    }
    out
}

/// A pass's steps, for the CPU and the card alike: each tap's offset along the pass, the cell
/// aside it starts from, and its weight there and one cell further.
pub(crate) fn steps(taps: &[f32], mu: f64) -> Vec<(isize, isize, f32, f32)> {
    let r = (taps.len() / 2) as isize;
    taps.iter()
        .enumerate()
        .map(|(i, &w)| {
            let t = i as isize - r;
            let off = mu * t as f64;
            let lo = off.floor();
            let fr = off - lo;
            (t, lo as isize, (w as f64 * (1.0 - fr)) as f32, (w as f64 * fr) as f32)
        })
        .collect()
}

/// One pass over a padded plane of `pw` columns (three channels a cell): each cell the sum of
/// `taps` along x (or y when `down`), reading `mu` cells aside per step, between two cells.
fn pass(src: &[f32], pw: usize, taps: &[f32], down: bool, mu: f64) -> Vec<f32> {
    let ph = src.len() / (3 * pw);
    let steps = steps(taps, mu);
    let at = |x: isize, y: isize| -> [f32; 3] {
        if x < 0 || y < 0 || x >= pw as isize || y >= ph as isize {
            return [0.0; 3];
        }
        let i = (y as usize * pw + x as usize) * 3;
        [src[i], src[i + 1], src[i + 2]]
    };
    let mut out = vec![0.0f32; src.len()];
    out.par_chunks_exact_mut(pw * 3).enumerate().for_each(|(j, row)| {
        for (i, o) in row.chunks_exact_mut(3).enumerate() {
            let (i, j) = (i as isize, j as isize);
            let mut acc = [0.0f32; 3];
            for &(t, lo, w0, w1) in &steps {
                let (p, q) = if down { (at(i + lo, j + t), at(i + lo + 1, j + t)) } else { (at(i + t, j + lo), at(i + t, j + lo + 1)) };
                for c in 0..3 {
                    acc[c] += w0 * p[c];
                    if w1 != 0.0 {
                        acc[c] += w1 * q[c];
                    }
                }
            }
            o.copy_from_slice(&acc);
        }
    });
    out
}

/// Stage 2 for one level: the light (`lit`, w by h, three channels) averaged into cells laid
/// from the layer's top-left corner, then spread. The result is a plane of
/// (ceil(w / d) + 2 reach.0) by (ceil(h / d) + 2 reach.1) cells, the layer's first cell at
/// (reach.0, reach.1).
pub(crate) fn spread(lit: &[f32], w: usize, h: usize, lv: &Level) -> Vec<f32> {
    let d = lv.d;
    let (nw, nh) = (w.div_ceil(d), h.div_ceil(d));
    let (rx, ry) = lv.reach;
    let pw = nw + 2 * rx;
    let mut plane = vec![0.0f32; pw * (nh + 2 * ry) * 3];
    let area = (d * d) as f64;
    plane[ry * pw * 3..(ry + nh) * pw * 3].par_chunks_exact_mut(pw * 3).enumerate().for_each(|(j, row)| {
        for i in 0..nw {
            let mut sum = [0.0f64; 3];
            for y in j * d..((j + 1) * d).min(h) {
                for x in i * d..((i + 1) * d).min(w) {
                    for c in 0..3 {
                        sum[c] += lit[(y * w + x) * 3 + c] as f64;
                    }
                }
            }
            for c in 0..3 {
                row[(rx + i) * 3 + c] = (sum[c] / area) as f32;
            }
        }
    });
    let first = pass(&plane, pw, &lv.first, lv.first_y, 0.0);
    pass(&first, pw, &lv.second, !lv.first_y, lv.mu)
}

/// Stage 3 at one pixel: the glow `g` (the levels' sum) times Exposure, laid with the untouched
/// pixel `p` at Source Opacity. Screen adds and holds each channel at 1; Unmult on makes the
/// glow's covering its brightest channel, off lays it on solid black.
pub(crate) fn finish(g: [f32; 3], p: [f32; 4], s: &Settings) -> [f32; 4] {
    let (e, o) = (s.exposure as f32, (s.opacity / 100.0) as f32);
    let mut g = g.map(|v| v * e);
    if s.screen {
        g = g.map(|v| v.min(1.0));
    }
    let ga = if s.unmult { g[0].max(g[1]).max(g[2]).min(1.0) } else { 1.0 };
    let mut out = [0.0; 4];
    for c in 0..3 {
        let v = g[c] + o * p[c];
        out[c] = if s.screen { v.min(1.0) } else { v };
    }
    out[3] = (ga + o * p[3]).min(1.0);
    out
}

/// Soft Physical Glow on the CPU: the layer grows by the plan's reach on every side, so no glow
/// is cut off (no silent crop), and the grow is returned.
pub(crate) fn soft_glow(source: &mut WorkingBuffer, s: &Settings) -> usize {
    let (w, h) = (source.width(), source.height());
    if w == 0 || h == 0 {
        return 0;
    }
    let plan = plan(s.radius, s.aspect, s.angle);
    let g = plan.grow;
    let lit: Vec<f32> = source.data().par_chunks_exact(4).flat_map_iter(|px| light(px, s)).collect();
    let planes: Vec<Vec<f32>> = plan.levels.iter().map(|lv| spread(&lit, w, h, lv)).collect();
    let (ow, oh) = (w + 2 * g, h + 2 * g);
    let mut out = WorkingBuffer::transparent(ow, oh);
    let src = source.data();
    out.data_mut().par_chunks_exact_mut(ow * 4).enumerate().for_each(|(row, line)| {
        let y = row as isize - g as isize;
        for (col, o) in line.chunks_exact_mut(4).enumerate() {
            let x = col as isize - g as isize;
            let inside = x >= 0 && y >= 0 && x < w as isize && y < h as isize;
            let i = if inside { y as usize * w + x as usize } else { 0 };
            let mut sum = [0.0f32; 3];
            if inside && plan.own > 0.0 {
                for c in 0..3 {
                    sum[c] += plan.own * lit[i * 3 + c];
                }
            }
            for (lv, plane) in plan.levels.iter().zip(&planes) {
                let d = lv.d as f32;
                let (pw, ph) = (w.div_ceil(lv.d) + 2 * lv.reach.0, h.div_ceil(lv.d) + 2 * lv.reach.1);
                let (u, v) = ((x as f32 + 0.5) / d - 0.5, (y as f32 + 0.5) / d - 0.5);
                let (u0, v0) = (u.floor(), v.floor());
                let (fu, fv) = (u - u0, v - v0);
                for (dj, wj) in [(0, 1.0 - fv), (1, fv)] {
                    let cy = v0 as isize + dj + lv.reach.1 as isize;
                    if wj == 0.0 || cy < 0 || cy >= ph as isize {
                        continue;
                    }
                    for (di, wi) in [(0, 1.0 - fu), (1, fu)] {
                        let cx = u0 as isize + di + lv.reach.0 as isize;
                        if wi == 0.0 || cx < 0 || cx >= pw as isize {
                            continue;
                        }
                        let k = (cy as usize * pw + cx as usize) * 3;
                        let wt = lv.weight * wj * wi;
                        for c in 0..3 {
                            sum[c] += wt * plane[k + c];
                        }
                    }
                }
            }
            let p = if inside { [src[i * 4], src[i * 4 + 1], src[i * 4 + 2], src[i * 4 + 3]] } else { [0.0; 4] };
            o.copy_from_slice(&finish(sum, p, s));
        }
    });
    *source = out;
    g
}
