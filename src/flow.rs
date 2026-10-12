//! D-650: this program's own motion estimate between two pictures (P0-13, D-600: "Blur +
//! retiming only"), read by Pixel Motion Blur and Timewarp, and Pixel Motion Blur's gather.
//!
//! Dense pyramidal Lucas-Kanade with a ridge term: a fixed count of steps, no search and no
//! choice made on the data, so the same two pictures always give the same field. The rule is
//! pinned step by step in `tools/flow_reference.py`; this is that rule in double precision.
//! Pixels are indexed from 0; "held" means an index outside the picture is moved to its edge.

use rayon::prelude::*;

use crate::WorkingBuffer;

/// The estimate's settings. Pixel Motion Blur sets Vector Detail and keeps the rest as added.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Tuning {
    pub vector_detail: f64,
    pub global_smoothness: f64,
    pub local_smoothness: f64,
    pub iterations: u32,
    pub block_size: f64,
    pub weights: [f64; 3],
    pub correct_luminance: bool,
}

impl Tuning {
    /// D-650's settings when added: Vector Detail 20, both smoothnesses 0.5, four iterations,
    /// Block Size 6, weights 0.3, 0.6 and 0.1, no luminance correction.
    pub(crate) fn added(vector_detail: f64) -> Self {
        Tuning {
            vector_detail,
            global_smoothness: 0.5,
            local_smoothness: 0.5,
            iterations: 4,
            block_size: 6.0,
            weights: [0.3, 0.6, 0.1],
            correct_luminance: false,
        }
    }
}

/// A field of vectors in pixels, across and down, `w` by `h`.
pub(crate) struct Field {
    pub w: usize,
    pub h: usize,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}

fn held(i: isize, n: usize) -> usize {
    i.clamp(0, n as isize - 1) as usize
}

/// One level of one picture: the weighted colour and the alpha.
struct Planes {
    w: usize,
    h: usize,
    c: [Vec<f64>; 2],
}

fn grey(p: &WorkingBuffer, weights: [f64; 3]) -> Planes {
    let [wr, wg, wb] = weights;
    let px = p.data().chunks_exact(4);
    Planes {
        w: p.width(),
        h: p.height(),
        c: [
            px.clone().map(|q| wr * q[0] as f64 + wg * q[1] as f64 + wb * q[2] as f64).collect(),
            px.map(|q| q[3] as f64).collect(),
        ],
    }
}

fn halve(plane: &[f64], w: usize, h: usize) -> Vec<f64> {
    let (w2, h2) = (w.div_ceil(2), h.div_ceil(2));
    (0..w2 * h2)
        .into_par_iter()
        .map(|i| {
            let (x, y) = ((i % w2) as isize, (i / w2) as isize);
            let mut s = 0.0;
            for dy in 0..2 {
                for dx in 0..2 {
                    s += plane[held(2 * y + dy, h) * w + held(2 * x + dx, w)];
                }
            }
            s / 4.0
        })
        .collect()
}

/// The sum over the (2r + 1)^2 pixels around each, held, across first, then down.
fn boxed(plane: &[f64], w: usize, h: usize, r: isize) -> Vec<f64> {
    let mut across = vec![0.0; w * h];
    across.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        for (x, out) in row.iter_mut().enumerate() {
            let mut s = 0.0;
            for i in -r..=r {
                s += plane[y * w + held(x as isize + i, w)];
            }
            *out = s;
        }
    });
    let mut out = vec![0.0; w * h];
    out.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        for (x, o) in row.iter_mut().enumerate() {
            let mut s = 0.0;
            for j in -r..=r {
                s += across[held(y as isize + j, h) * w + x];
            }
            *o = s;
        }
    });
    out
}

fn bilinear_held(plane: &[f64], w: usize, h: usize, x: f64, y: f64) -> f64 {
    let x = x.clamp(0.0, w as f64 - 1.0);
    let y = y.clamp(0.0, h as f64 - 1.0);
    let (x0, y0) = (x.floor(), y.floor());
    let (ux, uy) = (x - x0, y - y0);
    let (x0, y0) = (x0 as usize, y0 as usize);
    let (x1, y1) = ((x0 + 1).min(w - 1), (y0 + 1).min(h - 1));
    let top = plane[y0 * w + x0] * (1.0 - ux) + plane[y0 * w + x1] * ux;
    let bottom = plane[y1 * w + x0] * (1.0 - ux) + plane[y1 * w + x1] * ux;
    top * (1.0 - uy) + bottom * uy
}

/// The top level: the largest whole number up to 6 with min(w, h) / 2^L at least 8.
pub(crate) fn top_level(w: usize, h: usize) -> usize {
    let mut level = 0;
    while level < 6 && w.min(h) as f64 / f64::powi(2.0, level as i32 + 1) >= 8.0 {
        level += 1;
    }
    level
}

/// The finest level estimated for a Vector Detail: 100 every pixel, 20 one in four.
pub(crate) fn finest(vector_detail: f64, top: usize) -> usize {
    if vector_detail <= 0.0 {
        return top;
    }
    ((100.0 / vector_detail).log2() + 0.5).floor().clamp(0.0, top as f64) as usize
}

/// `field` read at (x + 0.5)/f - 0.5 for each pixel of a `w2` by `h2` picture, times f.
fn enlarge(field: &Field, w2: usize, h2: usize, f: f64) -> Field {
    let (x, y) = (0..w2 * h2)
        .into_par_iter()
        .map(|i| {
            let sx = ((i % w2) as f64 + 0.5) / f - 0.5;
            let sy = ((i / w2) as f64 + 0.5) / f - 0.5;
            (f * bilinear_held(&field.x, field.w, field.h, sx, sy), f * bilinear_held(&field.y, field.w, field.h, sx, sy))
        })
        .unzip();
    Field { w: w2, h: h2, x, y }
}

/// The field from `a` to `b`, the same size: for each pixel p, u(p) with a(p) close to
/// b(p + u(p)).
pub(crate) fn estimate(a: &WorkingBuffer, b: &WorkingBuffer, s: &Tuning) -> Field {
    let (w, h) = (a.width(), a.height());
    if w == 0 || h == 0 {
        return Field { w, h, x: Vec::new(), y: Vec::new() };
    }
    let ca = grey(a, s.weights);
    let mut cb = grey(b, s.weights);
    if s.correct_luminance {
        let (ma, mb) = (ca.c[0].iter().sum::<f64>() / (w * h) as f64, cb.c[0].iter().sum::<f64>() / (w * h) as f64);
        if mb > 0.0 {
            cb.c[0].iter_mut().for_each(|v| *v *= ma / mb);
        }
    }
    let top = top_level(w, h);
    let lf = finest(s.vector_detail, top);
    let mut levels = vec![(ca, cb)];
    for _ in 0..top {
        let (pa, pb) = levels.last().unwrap();
        let (lw, lh) = (pa.w, pa.h);
        let next = |p: &Planes| Planes { w: lw.div_ceil(2), h: lh.div_ceil(2), c: [halve(&p.c[0], lw, lh), halve(&p.c[1], lw, lh)] };
        let pair = (next(pa), next(pb));
        levels.push(pair);
    }
    let r = (s.block_size / 2.0).floor().max(1.0) as isize;
    let area = ((2 * r + 1) * (2 * r + 1)) as f64;
    let ridge = area * (0.0001 + 0.01 * s.global_smoothness);
    let mut field: Option<Field> = None;
    for level in (lf..=top).rev() {
        let (pa, pb) = &levels[level];
        let (lw, lh) = (pa.w, pa.h);
        let n = lw * lh;
        let Field { mut x, mut y, .. } = match &field {
            None => Field { w: lw, h: lh, x: vec![0.0; n], y: vec![0.0; n] },
            Some(f) => enlarge(f, lw, lh, 2.0),
        };
        let grad = |c: &[f64], dx: isize, dy: isize| -> Vec<f64> {
            (0..n)
                .into_par_iter()
                .map(|i| {
                    let (px, py) = ((i % lw) as isize, (i / lw) as isize);
                    (c[held(py + dy, lh) * lw + held(px + dx, lw)] - c[held(py - dy, lh) * lw + held(px - dx, lw)]) / 2.0
                })
                .collect()
        };
        let gx = [grad(&pa.c[0], 1, 0), grad(&pa.c[1], 1, 0)];
        let gy = [grad(&pa.c[0], 0, 1), grad(&pa.c[1], 0, 1)];
        let gxx = boxed(&(0..n).map(|i| gx[0][i] * gx[0][i] + gx[1][i] * gx[1][i]).collect::<Vec<_>>(), lw, lh, r);
        let gxy = boxed(&(0..n).map(|i| gx[0][i] * gy[0][i] + gx[1][i] * gy[1][i]).collect::<Vec<_>>(), lw, lh, r);
        let gyy = boxed(&(0..n).map(|i| gy[0][i] * gy[0][i] + gy[1][i] * gy[1][i]).collect::<Vec<_>>(), lw, lh, r);
        for _ in 0..s.iterations {
            let (ex, ey): (Vec<f64>, Vec<f64>) = (0..n)
                .into_par_iter()
                .map(|i| {
                    let (px, py) = ((i % lw) as f64 + x[i], (i / lw) as f64 + y[i]);
                    let e0 = bilinear_held(&pb.c[0], lw, lh, px, py) - pa.c[0][i];
                    let e1 = bilinear_held(&pb.c[1], lw, lh, px, py) - pa.c[1][i];
                    (gx[0][i] * e0 + gx[1][i] * e1, gy[0][i] * e0 + gy[1][i] * e1)
                })
                .unzip();
            let (bx, by) = (boxed(&ex, lw, lh, r), boxed(&ey, lw, lh, r));
            x.par_iter_mut().zip(y.par_iter_mut()).enumerate().for_each(|(i, (ux, uy))| {
                let (a, d, c) = (gxx[i] + ridge, gyy[i] + ridge, gxy[i]);
                let det = a * d - c * c;
                *ux -= (d * bx[i] - c * by[i]) / det;
                *uy -= (a * by[i] - c * bx[i]) / det;
            });
        }
        let smooth = s.local_smoothness;
        let (mx, my) = (boxed(&x, lw, lh, r), boxed(&y, lw, lh, r));
        x.iter_mut().zip(&mx).for_each(|(u, m)| *u += smooth * (m / area - *u));
        y.iter_mut().zip(&my).for_each(|(u, m)| *u += smooth * (m / area - *u));
        field = Some(Field { w: lw, h: lh, x, y });
    }
    let field = field.unwrap();
    if lf == 0 {
        field
    } else {
        enlarge(&field, w, h, f64::powi(2.0, lf as i32))
    }
}

/// D-650: Pixel Motion Blur's step 6. `o` is the present frame, `back` and `ahead` the fields
/// to the frames before and after; `share` the share of a frame the shutter is open and
/// `samples` the moments. Each pixel is the mean of `o` read bilinearly (document 21,
/// transparent outside) at the pixel less |s_k| times the vector of its half of the shutter.
pub(crate) fn gather(o: &WorkingBuffer, back: &Field, ahead: &Field, share: f64, samples: u32) -> WorkingBuffer {
    let (w, h) = (o.width(), o.height());
    let mut out = WorkingBuffer::transparent(w, h);
    let times: Vec<f64> = (0..samples).map(|k| share * ((k as f64 + 0.5) / samples as f64 - 0.5)).collect();
    out.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
        let mut acc = [0.0f32; 4];
        for &s in &times {
            let f = if s < 0.0 { back } else { ahead };
            let q = crate::render::sample_bilinear(o, x - s.abs() * f.x[i], y - s.abs() * f.y[i]);
            for c in 0..4 {
                acc[c] += q[c];
            }
        }
        for c in 0..4 {
            px[c] = acc[c] / samples as f32;
        }
    });
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square(cx: f64, cy: f64) -> WorkingBuffer {
        let mut p = WorkingBuffer::transparent(32, 32);
        for (i, px) in p.data_mut().chunks_exact_mut(4).enumerate() {
            let (x, y) = ((i % 32) as f64 + 0.5, (i / 32) as f64 + 0.5);
            let d = (x - cx).abs().max((y - cy).abs());
            let c = ((7.0 - d) / 3.0).clamp(0.0, 1.0) as f32;
            px.copy_from_slice(&[0.8 * c, 0.4 * c, 0.1 * c, c]);
        }
        p
    }

    /// tools/flow_reference.py's own check: a soft square moving 2 right and 1 down is found
    /// moving so at its middle (1.962, 0.950 there), and a still one not at all.
    #[test]
    fn a_moving_square_is_found_moving() {
        let u = estimate(&square(14.0, 15.0), &square(16.0, 16.0), &Tuning::added(100.0));
        let i = 15 * 32 + 14;
        assert!((u.x[i] - 1.962).abs() < 1e-3 && (u.y[i] - 0.950).abs() < 1e-3, "{} {}", u.x[i], u.y[i]);
        let still = estimate(&square(14.0, 15.0), &square(14.0, 15.0), &Tuning::added(20.0));
        assert!(still.x.iter().chain(&still.y).all(|v| *v == 0.0));
    }
}
