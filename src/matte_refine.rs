//! D-352 (EFFECTS.md P0-20): the matte refinement kit. An edge-aware matte filter, the guided
//! filter of He, Sun and Tang ("Guided Image Filtering", ECCV 2010 and TPAMI 2013, Algorithm 2
//! with a colour guide), kept here once for the keying effects to come, and three effects that
//! use it or its box sums: Matte Choker, Refine Hard Matte and Refine Soft Matte, after After
//! Effects' effects of those names. Document 21 has the rules in words;
//! `tools/matte_refine_reference.py` works them a second way. Nothing is ported: Adobe does not
//! publish its methods.
//!
//! Everything is worked in double precision from the layer's single-precision pixels, on the
//! whole layer, and written back once.

use crate::WorkingBuffer;
use rayon::prelude::*;

/// The guided filter's epsilon, for a guide in sRGB values 0 to 1.
pub(crate) const EPS: f64 = 1e-4;

/// The sum of each of the `ch` channels of `f` over the square of side 2r + 1 about each pixel,
/// cut to the w by h picture: running totals down the columns, then along the rows.
pub(crate) fn box_sum(f: &[f64], ch: usize, w: usize, h: usize, r: usize) -> Vec<f64> {
    let line = w * ch;
    // Down the columns: run[y] is the sum of rows 0 to y - 1.
    let mut run = vec![0.0f64; (h + 1) * line];
    for y in 0..h {
        let (done, next) = run.split_at_mut((y + 1) * line);
        let (prev, src) = (&done[y * line..], &f[y * line..(y + 1) * line]);
        for ((n, p), s) in next[..line].iter_mut().zip(prev).zip(src) {
            *n = p + s;
        }
    }
    let mut out = vec![0.0f64; h * line];
    out.par_chunks_mut(line).enumerate().for_each(|(y, row)| {
        let (hi, lo) = ((y + r).min(h - 1) + 1, y.saturating_sub(r));
        let col: Vec<f64> = (0..line).map(|i| run[hi * line + i] - run[lo * line + i]).collect();
        // Along the row: acc[x] is the sum of columns 0 to x - 1.
        let mut acc = vec![0.0f64; (w + 1) * ch];
        for x in 0..w {
            for j in 0..ch {
                acc[(x + 1) * ch + j] = acc[x * ch + j] + col[x * ch + j];
            }
        }
        for x in 0..w {
            let (hi, lo) = ((x + r).min(w - 1) + 1, x.saturating_sub(r));
            for j in 0..ch {
                row[x * ch + j] = acc[hi * ch + j] - acc[lo * ch + j];
            }
        }
    });
    out
}

/// The number of pixels in the cut square about (x, y).
fn count(x: usize, y: usize, w: usize, h: usize, r: usize) -> f64 {
    (((x + r).min(w - 1) - x.saturating_sub(r) + 1) * ((y + r).min(h - 1) - y.saturating_sub(r) + 1)) as f64
}

/// `m` x = `v` for a 3 by 3 `m`, by Cramer's rule.
fn solve3(m: [[f64; 3]; 3], v: [f64; 3]) -> [f64; 3] {
    let det = |m: [[f64; 3]; 3]| {
        m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1]) - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
            + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
    };
    let d = det(m);
    let mut x = [0.0; 3];
    for (i, xi) in x.iter_mut().enumerate() {
        let mut c = m;
        for (row, vi) in c.iter_mut().zip(v) {
            row[i] = vi;
        }
        *xi = det(c) / d;
    }
    x
}

/// The guided filter of the matte `p` by the three-channel `guide`, at radius `r`, epsilon EPS.
pub(crate) fn guided_filter(guide: &[f64], p: &[f64], w: usize, h: usize, r: usize) -> Vec<f64> {
    // I, p, the six of I I^T and I p, boxed together.
    let mut f = vec![0.0f64; w * h * 13];
    f.par_chunks_mut(13).enumerate().for_each(|(i, o)| {
        let (g, q) = (&guide[i * 3..i * 3 + 3], p[i]);
        o[..3].copy_from_slice(g);
        o[3] = q;
        o[4..10].copy_from_slice(&[g[0] * g[0], g[0] * g[1], g[0] * g[2], g[1] * g[1], g[1] * g[2], g[2] * g[2]]);
        o[10..].copy_from_slice(&[g[0] * q, g[1] * q, g[2] * q]);
    });
    let s = box_sum(&f, 13, w, h, r);
    let mut ab = vec![0.0f64; w * h * 4];
    ab.par_chunks_mut(4).enumerate().for_each(|(i, o)| {
        let n = count(i % w, i / w, w, h, r);
        let m: Vec<f64> = s[i * 13..i * 13 + 13].iter().map(|v| v / n).collect();
        let (mi, mp) = ([m[0], m[1], m[2]], m[3]);
        let c = |k: usize, a: usize, b: usize| m[k] - mi[a] * mi[b];
        let sig = [
            [c(4, 0, 0) + EPS, c(5, 0, 1), c(6, 0, 2)],
            [c(5, 1, 0), c(7, 1, 1) + EPS, c(8, 1, 2)],
            [c(6, 2, 0), c(8, 2, 1), c(9, 2, 2) + EPS],
        ];
        let cov = [m[10] - mi[0] * mp, m[11] - mi[1] * mp, m[12] - mi[2] * mp];
        let a = solve3(sig, cov);
        o[..3].copy_from_slice(&a);
        o[3] = mp - (a[0] * mi[0] + a[1] * mi[1] + a[2] * mi[2]);
    });
    let s = box_sum(&ab, 4, w, h, r);
    (0..w * h)
        .into_par_iter()
        .map(|i| {
            let n = count(i % w, i / w, w, h, r);
            let g = &guide[i * 3..i * 3 + 3];
            (s[i * 4] * g[0] + s[i * 4 + 1] * g[1] + s[i * 4 + 2] * g[2] + s[i * 4 + 3]) / n
        })
        .collect()
}

/// 0 to 1 across `w` about `c`, each end held inside 0 to 1; a step at `c` when that leaves no
/// width.
pub(crate) fn ramp(v: f64, c: f64, w: f64) -> f64 {
    let (lo, hi) = ((c - w / 2.0).clamp(0.0, 1.0), (c + w / 2.0).clamp(0.0, 1.0));
    if hi > lo {
        ((v - lo) / (hi - lo)).clamp(0.0, 1.0)
    } else {
        (v > c) as u8 as f64
    }
}

fn srgb(c: f64) -> f64 {
    crate::grade::to_srgb(c.clamp(0.0, 1.0))
}

/// The straight colour of a premultiplied pixel, black where it has no covering.
fn straight(p: &[f64]) -> [f64; 3] {
    if p[3] > 0.0 {
        [p[0] / p[3], p[1] / p[3], p[2] / p[3]]
    } else {
        [0.0; 3]
    }
}

fn read(source: &WorkingBuffer) -> Vec<f64> {
    source.data().iter().map(|&v| v as f64).collect()
}

fn write(source: &mut WorkingBuffer, px: &[f64]) {
    for (d, s) in source.data_mut().iter_mut().zip(px) {
        *d = *s as f32;
    }
}

/// One Matte Choker stage: geometric softness `g`, choke `k`, gray level softness `s`.
fn choker_stage(px: &[f64], w: usize, h: usize, g: f64, k: f64, s: f64) -> Vec<f64> {
    let runs = crate::layer_fx::disc_runs(g);
    let size: isize = runs.iter().map(|&(_, hw)| 2 * hw + 1).sum();
    let mut sums = vec![0.0f64; h * (w + 1) * 4];
    sums.par_chunks_mut((w + 1) * 4).enumerate().for_each(|(y, line)| {
        for x in 0..w {
            for j in 0..4 {
                line[(x + 1) * 4 + j] = line[x * 4 + j] + px[(y * w + x) * 4 + j];
            }
        }
    });
    let mut out = vec![0.0f64; w * h * 4];
    out.par_chunks_mut(4).enumerate().for_each(|(i, o)| {
        let (x, y) = ((i % w) as isize, (i / w) as isize);
        let mut t = [0.0f64; 4];
        for &(dy, hw) in &runs {
            let sy = y + dy;
            if sy < 0 || sy >= h as isize {
                continue;
            }
            let (lo, hi) = ((x - hw).max(0) as usize, ((x + hw).min(w as isize - 1) + 1) as usize);
            if hi <= lo {
                continue;
            }
            let line = &sums[sy as usize * (w + 1) * 4..];
            for j in 0..4 {
                t[j] += line[hi * 4 + j] - line[lo * 4 + j];
            }
        }
        let a = ramp(t[3] / size as f64, 0.5 + k / 255.0, s / 100.0);
        let own = &px[i * 4..i * 4 + 4];
        let c = if own[3] > 0.0 { straight(own) } else { straight(&t) };
        o.copy_from_slice(&[c[0] * a, c[1] * a, c[2] * a, a]);
    });
    out
}

/// D-352: Matte Choker. `stages` are each geometric softness, choke and gray level softness;
/// stage 1 then stage 2, the pair repeated floor(`iterations`) times. The settings are valid.
pub(crate) fn matte_choker(source: &mut WorkingBuffer, stages: [[f64; 3]; 2], iterations: f64) {
    let (w, h) = (source.width(), source.height());
    if w == 0 || h == 0 {
        return;
    }
    let mut px = read(source);
    for _ in 0..iterations.floor() as usize {
        for [g, k, s] in stages {
            px = choker_stage(&px, w, h, g, k, s);
        }
    }
    write(source, &px);
}

/// Refine Hard Matte's and Refine Soft Matte's settings, valid. `edge_radius` is Soft's only
/// (`soft` false, it is not read).
pub(crate) struct Refine<'a> {
    pub soft: bool,
    pub edge_radius: f64,
    pub view_edge_region: &'a str,
    pub feather: f64,
    pub contrast: f64,
    pub shift_edge: f64,
    pub decontaminate: &'a str,
    pub decontamination_amount: f64,
    pub decontamination_radius: f64,
    pub view_decontamination_map: &'a str,
}

/// The edge region at `re`: a covered pixel and a pixel not fully covered both in the square.
fn edge_region(a0: &[f64], w: usize, h: usize, re: usize) -> Vec<bool> {
    let f: Vec<f64> = a0.iter().flat_map(|&a| [(a > 0.0) as u8 as f64, (a < 1.0) as u8 as f64]).collect();
    box_sum(&f, 2, w, h, re).chunks(2).map(|c| c[0] > 0.0 && c[1] > 0.0).collect()
}

/// D-352: Refine Hard Matte and Refine Soft Matte, document 21's rule.
pub(crate) fn refine_matte(source: &mut WorkingBuffer, s: &Refine) {
    let (w, h) = (source.width(), source.height());
    if w == 0 || h == 0 {
        return;
    }
    let n = w * h;
    let px = read(source);
    let a0: Vec<f64> = px.chunks(4).map(|p| p[3]).collect();
    let known: Vec<bool> = a0.iter().map(|&a| a > 0.0).collect();
    let re = if s.soft { s.edge_radius.floor() as usize } else { 0 };
    let rf = s.feather.floor() as usize;
    let big = 2 * (re + rf);
    let mut c: Vec<[f64; 3]> = px.chunks(4).map(straight).collect();
    let mut filled = vec![false; n];
    if big > 0 {
        let total = box_sum(&px, 4, w, h, big);
        for i in 0..n {
            if !known[i] {
                filled[i] = total[i * 4 + 3] > 0.0;
                c[i] = straight(&total[i * 4..i * 4 + 4]);
            }
        }
    }
    let mut a = a0.clone();
    let region = if s.soft { Some(edge_region(&a0, w, h, re)) } else { None };
    if let (Some(region), true) = (&region, re > 0) {
        let guide: Vec<f64> = px.chunks(4).flat_map(|p| straight(p).map(srgb)).collect();
        let q = guided_filter(&guide, &a0, w, h, re);
        for i in 0..n {
            if region[i] {
                a[i] = q[i].clamp(0.0, 1.0);
            }
        }
    }
    if rf > 0 {
        let guide: Vec<f64> = c.iter().flat_map(|v| v.map(srgb)).collect();
        a = guided_filter(&guide, &a, w, h, rf).into_iter().map(|v| v.clamp(0.0, 1.0)).collect();
    }
    let (mid, width) = (0.5 - s.shift_edge / 200.0, 1.0 - s.contrast / 100.0);
    for i in 0..n {
        a[i] = if known[i] || filled[i] { ramp(a[i], mid, width) } else { 0.0 };
    }
    let amount = s.decontamination_amount / 100.0;
    let on = s.decontaminate == "on";
    if on {
        let rd = big + 1 + s.decontamination_radius.floor() as usize;
        // Weighted by covering and by its want, over the pixels that had covering.
        let mut f = vec![0.0f64; n * 8];
        for i in 0..n {
            if known[i] {
                let (wf, wb) = (a[i], 1.0 - a[i]);
                f[i * 8..i * 8 + 8].copy_from_slice(&[
                    wf * c[i][0], wf * c[i][1], wf * c[i][2], wf, wb * c[i][0], wb * c[i][1], wb * c[i][2], wb,
                ]);
            }
        }
        let t = box_sum(&f, 8, w, h, rd);
        c.par_iter_mut().enumerate().for_each(|(i, ci)| {
            let t = &t[i * 8..i * 8 + 8];
            let fh = if t[3] > 0.0 { [t[0] / t[3], t[1] / t[3], t[2] / t[3]] } else { *ci };
            let bh = if t[7] > 0.0 { [t[4] / t[7], t[5] / t[7], t[6] / t[7]] } else { [0.0; 3] };
            let ai = a[i];
            for j in 0..3 {
                let fj = (fh[j] + ai * (ci[j] - ai * fh[j] - (1.0 - ai) * bh[j])).clamp(0.0, 1.0);
                ci[j] += amount * (fj - ci[j]);
            }
        });
    }
    let out: Vec<f64> = if let (Some(region), "on") = (&region, s.view_edge_region) {
        region.iter().flat_map(|&r| [r as u8 as f64, r as u8 as f64, r as u8 as f64, 1.0]).collect()
    } else if s.view_decontamination_map == "on" {
        let k = if on { amount } else { 0.0 };
        a.iter().flat_map(|&a| {
            let v = k * 4.0 * a * (1.0 - a);
            [v, v, v, 1.0]
        })
        .collect()
    } else {
        c.iter().zip(&a).flat_map(|(c, &a)| [c[0] * a, c[1] * a, c[2] * a, a]).collect()
    };
    write(source, &out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guided_filter_keeps_a_constant_and_boxes_under_a_flat_guide() {
        let (w, h) = (7, 5);
        let guide: Vec<f64> = (0..w * h).flat_map(|i| [i as f64 / 40.0, 0.3, (i % 3) as f64 / 3.0]).collect();
        let q = guided_filter(&guide, &vec![0.7; w * h], w, h, 2);
        assert!(q.iter().all(|v| (v - 0.7).abs() < 1e-12));
        // A flat guide: the box mean of the box mean.
        let p: Vec<f64> = (0..w * h).map(|i| ((i * 7) % 5) as f64 / 4.0).collect();
        let q = guided_filter(&vec![0.4; w * h * 3], &p, w, h, 1);
        let mean = |f: &[f64]| -> Vec<f64> {
            box_sum(f, 1, w, h, 1).iter().enumerate().map(|(i, v)| v / count(i % w, i / w, w, h, 1)).collect()
        };
        let twice = mean(&mean(&p));
        assert!(q.iter().zip(&twice).all(|(a, b)| (a - b).abs() < 1e-12));
    }

    #[test]
    fn ramp_ends_and_step() {
        assert_eq!(ramp(0.3, 0.5, 1.0), 0.3);
        assert_eq!(ramp(0.6, 0.5, 0.0), 1.0);
        assert_eq!(ramp(0.5, 0.5, 0.0), 0.0);
        assert_eq!(ramp(0.9, 0.5, 0.2), 1.0);
    }
}
