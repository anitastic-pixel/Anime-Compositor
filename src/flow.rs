//! D-650: this program's own motion estimate between two pictures (P0-13, D-600: "Blur +
//! retiming only"), read by Pixel Motion Blur and Timewarp, and Pixel Motion Blur's gather.
//!
//! Dense pyramidal Lucas-Kanade with a ridge term: a fixed count of steps, no search and no
//! choice made on the data, so the same two pictures always give the same field. The rule is
//! pinned step by step in `tools/flow_reference.py`; this is that rule, worked in single
//! precision (D-804): the found motion stays within 1e-4 px of the rule's double precision.
//! Pixels are indexed from 0; "held" means an index outside the picture is moved to its edge.

use rayon::prelude::*;

use crate::WorkingBuffer;

mod card;

/// D-878: whether the finder has a card on this machine whose found motion is within
/// [`CARD_OFF`] px of the processor's. It is checked once, on a test pair; a card that is not
/// is not used, and says so.
pub(crate) fn card_available() -> bool {
    static AGREES: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *AGREES.get_or_init(|| card::available() && agrees())
}

/// D-878: how many fields the card has found, so that a frame the card then refuses is planned
/// again on the processor (`preview::preview_frame_srgb8`).
pub(crate) fn card_uses() -> u64 {
    CARD_USES.load(std::sync::atomic::Ordering::Relaxed)
}

static CARD_USES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// D-878: the card finder switched off, for B-758's before-and-after checks.
pub(crate) static CARD_SWITCHED_OFF: std::sync::atomic::AtomicBool =
    std::sync::atomic::AtomicBool::new(false);

/// D-878: whether a tuning that asks for the card is found there, for a cache that keeps what
/// the finder found.
pub(crate) fn card_on(s: &Tuning) -> bool {
    s.card && !CARD_SWITCHED_OFF.load(std::sync::atomic::Ordering::Relaxed) && card_available()
}

/// The most the card's found motion may be off the processor's on the test pair: a tenth of
/// D-804's 1e-4 px. This machine's card was 4.8e-7 px off on it, and 0 on the reference shot.
const CARD_OFF: f64 = 1e-5;

/// The card's and the processor's found motion's largest difference on a test pair, at Vector
/// Detail 20 and 100, within [`CARD_OFF`].
fn agrees() -> bool {
    let (w, h) = (200, 120);
    let picture = |shift: f32| {
        let mut p = WorkingBuffer::transparent(w, h);
        for (i, q) in p.data_mut().chunks_exact_mut(4).enumerate() {
            let (x, y) = ((i % w) as f32 + shift, (i / w) as f32 - shift / 2.0);
            let v = 0.5 + 0.25 * (x * 0.13).sin() * (y * 0.07 + x * 0.02).cos();
            let alpha = 0.9 + 0.1 * (y * 0.05).sin();
            q.copy_from_slice(&[v, 0.8 * v, 1.0 - v, alpha]);
        }
        p
    };
    let (a, b) = (picture(0.0), picture(1.7));
    let off = [20.0, 100.0].iter().try_fold(0.0f64, |most, &vd| {
        let s = Tuning {
            correct_luminance: true,
            ..Tuning::added(vd)
        };
        let p = prepare(&a, &b, &s);
        let smooth = s.local_smoothness as f32;
        let found = card::estimate(
            &p.ca,
            &p.cb,
            p.top,
            p.lf,
            p.r,
            p.ridge,
            p.area,
            s.iterations,
            smooth,
        )?;
        let cpu = estimate(&a, &b, &s);
        let pairs = found.x.iter().zip(&cpu.x).chain(found.y.iter().zip(&cpu.y));
        Some(pairs.fold(most, |m, (u, v)| m.max((u - v).abs())))
    });
    match off {
        Some(d) if d <= CARD_OFF => true,
        Some(d) => {
            eprintln!("D-878: this card's found motion is {d:e} px off the processor's; the processor finds it");
            false
        }
        None => false,
    }
}

/// The planes and the levels [`estimate`] works on, for the processor and the card alike.
struct Prepared {
    ca: Planes,
    cb: Planes,
    top: usize,
    lf: usize,
    r: isize,
    area: f32,
    ridge: f32,
}

fn prepare(a: &WorkingBuffer, b: &WorkingBuffer, s: &Tuning) -> Prepared {
    let (w, h) = (a.width(), a.height());
    let ca = grey(a, s.weights);
    let mut cb = grey(b, s.weights);
    if s.correct_luminance {
        // The means are summed in double precision, in one pass in order.
        let mean = |c: &[f32]| c.iter().map(|&v| f64::from(v)).sum::<f64>() / (w * h) as f64;
        let (ma, mb) = (mean(&ca.c[0]), mean(&cb.c[0]));
        if mb > 0.0 {
            let k = (ma / mb) as f32;
            cb.c[0].par_iter_mut().for_each(|v| *v *= k);
        }
    }
    let top = top_level(w, h);
    let lf = finest(s.vector_detail, top);
    let r = (s.block_size / 2.0).floor().max(1.0) as isize;
    let area = ((2 * r + 1) * (2 * r + 1)) as f32;
    let ridge = area * (0.0001 + 0.01 * s.global_smoothness as f32);
    Prepared {
        ca,
        cb,
        top,
        lf,
        r,
        area,
        ridge,
    }
}

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
    /// D-878: found on the graphics card, for a frame the card draws; the processor's finder
    /// when the card cannot. Not a setting: compose sets it.
    pub card: bool,
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
            card: false,
        }
    }
}

/// A field of vectors in pixels, across and down, one per pixel of the picture.
pub(crate) struct Field {
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
    c: [Vec<f32>; 2],
}

fn grey(p: &WorkingBuffer, weights: [f64; 3]) -> Planes {
    let [wr, wg, wb] = weights.map(|v| v as f32);
    let px = p.data().par_chunks_exact(4);
    Planes {
        w: p.width(),
        h: p.height(),
        c: [
            px.clone()
                .map(|q| wr * q[0] + wg * q[1] + wb * q[2])
                .collect(),
            px.map(|q| q[3]).collect(),
        ],
    }
}

fn halve(plane: &[f32], w: usize, h: usize) -> Vec<f32> {
    let (w2, h2) = (w.div_ceil(2), h.div_ceil(2));
    (0..w2 * h2)
        .into_par_iter()
        .map(|i| {
            let (x, y) = ((i % w2) as isize, (i / w2) as isize);
            let mut s = 0.0f32;
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
fn boxed(plane: &[f32], w: usize, h: usize, r: isize) -> Vec<f32> {
    let mut across = vec![0.0f32; w * h];
    across.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        for (x, out) in row.iter_mut().enumerate() {
            let mut s = 0.0f32;
            for i in -r..=r {
                s += plane[y * w + held(x as isize + i, w)];
            }
            *out = s;
        }
    });
    let mut out = vec![0.0f32; w * h];
    out.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
        for (x, o) in row.iter_mut().enumerate() {
            let mut s = 0.0f32;
            for j in -r..=r {
                s += across[held(y as isize + j, h) * w + x];
            }
            *o = s;
        }
    });
    out
}

fn bilinear_held(plane: &[f32], w: usize, h: usize, x: f32, y: f32) -> f32 {
    let x = x.clamp(0.0, w as f32 - 1.0);
    let y = y.clamp(0.0, h as f32 - 1.0);
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
    ((100.0 / vector_detail).log2() + 0.5)
        .floor()
        .clamp(0.0, top as f64) as usize
}

/// A field while it is found: `w` by `h`, across and down, in single precision.
struct Grid {
    w: usize,
    h: usize,
    x: Vec<f32>,
    y: Vec<f32>,
}

/// `field` read at (x + 0.5)/f - 0.5 for each pixel of a `w2` by `h2` picture, times f.
fn enlarge(field: &Grid, w2: usize, h2: usize, f: f32) -> Grid {
    let (x, y) = (0..w2 * h2)
        .into_par_iter()
        .map(|i| {
            let sx = ((i % w2) as f32 + 0.5) / f - 0.5;
            let sy = ((i / w2) as f32 + 0.5) / f - 0.5;
            (
                f * bilinear_held(&field.x, field.w, field.h, sx, sy),
                f * bilinear_held(&field.y, field.w, field.h, sx, sy),
            )
        })
        .unzip();
    Grid { w: w2, h: h2, x, y }
}

/// The field from `a` to `b`, the same size: for each pixel p, u(p) with a(p) close to
/// b(p + u(p)).
pub(crate) fn estimate(a: &WorkingBuffer, b: &WorkingBuffer, s: &Tuning) -> Field {
    let (w, h) = (a.width(), a.height());
    if w == 0 || h == 0 {
        return Field {
            x: Vec::new(),
            y: Vec::new(),
        };
    }
    let Prepared {
        ca,
        cb,
        top,
        lf,
        r,
        area,
        ridge,
    } = prepare(a, b, s);
    if card_on(s) {
        let smooth = s.local_smoothness as f32;
        if let Some(f) = card::estimate(&ca, &cb, top, lf, r, ridge, area, s.iterations, smooth) {
            CARD_USES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return f;
        }
    }
    let mut levels = vec![(ca, cb)];
    for _ in 0..top {
        let (pa, pb) = levels.last().unwrap();
        let (lw, lh) = (pa.w, pa.h);
        let next = |p: &Planes| Planes {
            w: lw.div_ceil(2),
            h: lh.div_ceil(2),
            c: [halve(&p.c[0], lw, lh), halve(&p.c[1], lw, lh)],
        };
        let pair = (next(pa), next(pb));
        levels.push(pair);
    }
    let mut field: Option<Grid> = None;
    for level in (lf..=top).rev() {
        let (pa, pb) = &levels[level];
        let (lw, lh) = (pa.w, pa.h);
        let n = lw * lh;
        let Grid { mut x, mut y, .. } = match &field {
            None => Grid {
                w: lw,
                h: lh,
                x: vec![0.0; n],
                y: vec![0.0; n],
            },
            Some(f) => enlarge(f, lw, lh, 2.0),
        };
        let grad = |c: &[f32], dx: isize, dy: isize| -> Vec<f32> {
            (0..n)
                .into_par_iter()
                .map(|i| {
                    let (px, py) = ((i % lw) as isize, (i / lw) as isize);
                    (c[held(py + dy, lh) * lw + held(px + dx, lw)]
                        - c[held(py - dy, lh) * lw + held(px - dx, lw)])
                        / 2.0
                })
                .collect()
        };
        let gx = [grad(&pa.c[0], 1, 0), grad(&pa.c[1], 1, 0)];
        let gy = [grad(&pa.c[0], 0, 1), grad(&pa.c[1], 0, 1)];
        let gxx = boxed(
            &(0..n)
                .into_par_iter()
                .map(|i| gx[0][i] * gx[0][i] + gx[1][i] * gx[1][i])
                .collect::<Vec<_>>(),
            lw,
            lh,
            r,
        );
        let gxy = boxed(
            &(0..n)
                .into_par_iter()
                .map(|i| gx[0][i] * gy[0][i] + gx[1][i] * gy[1][i])
                .collect::<Vec<_>>(),
            lw,
            lh,
            r,
        );
        let gyy = boxed(
            &(0..n)
                .into_par_iter()
                .map(|i| gy[0][i] * gy[0][i] + gy[1][i] * gy[1][i])
                .collect::<Vec<_>>(),
            lw,
            lh,
            r,
        );
        for _ in 0..s.iterations {
            let (ex, ey): (Vec<f32>, Vec<f32>) = (0..n)
                .into_par_iter()
                .map(|i| {
                    let (px, py) = ((i % lw) as f32 + x[i], (i / lw) as f32 + y[i]);
                    let e0 = bilinear_held(&pb.c[0], lw, lh, px, py) - pa.c[0][i];
                    let e1 = bilinear_held(&pb.c[1], lw, lh, px, py) - pa.c[1][i];
                    (gx[0][i] * e0 + gx[1][i] * e1, gy[0][i] * e0 + gy[1][i] * e1)
                })
                .unzip();
            let (bx, by) = (boxed(&ex, lw, lh, r), boxed(&ey, lw, lh, r));
            x.par_iter_mut()
                .zip(y.par_iter_mut())
                .enumerate()
                .for_each(|(i, (ux, uy))| {
                    let (a, d, c) = (gxx[i] + ridge, gyy[i] + ridge, gxy[i]);
                    let det = a * d - c * c;
                    *ux -= (d * bx[i] - c * by[i]) / det;
                    *uy -= (a * by[i] - c * bx[i]) / det;
                });
        }
        let smooth = s.local_smoothness as f32;
        let (mx, my) = (boxed(&x, lw, lh, r), boxed(&y, lw, lh, r));
        x.par_iter_mut()
            .zip(&mx)
            .for_each(|(u, m)| *u += smooth * (m / area - *u));
        y.par_iter_mut()
            .zip(&my)
            .for_each(|(u, m)| *u += smooth * (m / area - *u));
        field = Some(Grid { w: lw, h: lh, x, y });
    }
    let field = field.unwrap();
    let field = if lf == 0 {
        field
    } else {
        enlarge(&field, w, h, f32::powi(2.0, lf as i32))
    };
    Field {
        x: field.x.into_iter().map(f64::from).collect(),
        y: field.y.into_iter().map(f64::from).collect(),
    }
}

/// D-650: Pixel Motion Blur's step 6. `o` is the present frame, `back` and `ahead` the fields
/// to the frames before and after; `share` the share of a frame the shutter is open and
/// `samples` the moments. Each pixel is the mean of `o` read bilinearly (document 21,
/// transparent outside) at the pixel less |s_k| times the vector of its half of the shutter.
pub(crate) fn gather(
    o: &WorkingBuffer,
    back: &Field,
    ahead: &Field,
    share: f64,
    samples: u32,
) -> WorkingBuffer {
    let (w, h) = (o.width(), o.height());
    let mut out = WorkingBuffer::transparent(w, h);
    let times: Vec<f64> = (0..samples)
        .map(|k| share * ((k as f64 + 0.5) / samples as f64 - 0.5))
        .collect();
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
            let mut acc = [0.0f32; 4];
            for &s in &times {
                let f = if s < 0.0 { back } else { ahead };
                let q =
                    crate::render::sample_bilinear(o, x - s.abs() * f.x[i], y - s.abs() * f.y[i]);
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

/// D-652: Timewarp's picture `g` of the way from `pa` to `pb` (g may lie outside 0 to 1), with
/// `f01` the field from `pa` to `pb` and `f10` back. Each frame is read along its share of the
/// two fields, bilinearly by document 21 or, with `extreme`, by Transform's cubic; the two reads
/// are mixed by g held inside 0 and 1, or with `one_image` the nearer frame's read alone; and the
/// result is pulled toward the plain mix of the two frames by `threshold` times how far the two
/// reads differ (`weights` on the colour, plus the covering), at most all the way.
#[allow(clippy::too_many_arguments)]
pub(crate) fn tween(
    pa: &WorkingBuffer,
    pb: &WorkingBuffer,
    f01: &Field,
    f10: &Field,
    g: f64,
    extreme: bool,
    one_image: bool,
    threshold: f64,
    weights: [f64; 3],
) -> Vec<[f64; 4]> {
    let w = pa.width();
    let h = g.clamp(0.0, 1.0);
    let read: fn(&WorkingBuffer, f64, f64) -> [f32; 4] = if extreme {
        crate::layer_fx::sample_bicubic
    } else {
        crate::render::sample_bilinear
    };
    (0..w * pa.height())
        .into_par_iter()
        .map(|i| {
            let (x, y) = ((i % w) as f64 + 0.5, (i / w) as f64 + 0.5);
            let (ax, ay) = (
                -(1.0 - g) * g * f01.x[i] + g * g * f10.x[i],
                -(1.0 - g) * g * f01.y[i] + g * g * f10.y[i],
            );
            let (bx, by) = (
                (1.0 - g) * (1.0 - g) * f01.x[i] - g * (1.0 - g) * f10.x[i],
                (1.0 - g) * (1.0 - g) * f01.y[i] - g * (1.0 - g) * f10.y[i],
            );
            let a = read(pa, x + ax, y + ay).map(f64::from);
            let b = read(pb, x + bx, y + by).map(f64::from);
            let (qa, qb) = (&pa.data()[i * 4..i * 4 + 4], &pb.data()[i * 4..i * 4 + 4]);
            let lum = |p: &[f64; 4]| weights[0] * p[0] + weights[1] * p[1] + weights[2] * p[2];
            let m = (threshold * ((lum(&a) - lum(&b)).abs() + (a[3] - b[3]).abs())).min(1.0);
            std::array::from_fn(|c| {
                let px = if !one_image {
                    (1.0 - h) * a[c] + h * b[c]
                } else if g < 0.5 {
                    a[c]
                } else {
                    b[c]
                };
                let fm = (1.0 - h) * qa[c] as f64 + h * qb[c] as f64;
                (1.0 - m) * px + m * fm
            })
        })
        .collect()
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
        let u = estimate(
            &square(14.0, 15.0),
            &square(16.0, 16.0),
            &Tuning::added(100.0),
        );
        let i = 15 * 32 + 14;
        assert!(
            (u.x[i] - 1.962).abs() < 1e-3 && (u.y[i] - 0.950).abs() < 1e-3,
            "{} {}",
            u.x[i],
            u.y[i]
        );
        let still = estimate(
            &square(14.0, 15.0),
            &square(14.0, 15.0),
            &Tuning::added(20.0),
        );
        assert!(still.x.iter().chain(&still.y).all(|v| *v == 0.0));
    }
}
