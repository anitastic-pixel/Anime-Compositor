//! D-86's line smoothing: document 21's rule, on a layer's own pixels.
//!
//! Reshetov's morphological antialiasing, ported from OpenToonz,
//! `toonz/sources/common/trop/tantialias.cpp` at commit
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
//! D-86's three changes from OpenToonz are marked where they are made: a difference exactly at
//! the threshold counts as the same colour, colours are compared and mixed in their encoded
//! values, and the corner guard. `tools/smooth_reference.py` is the same rule worked a second
//! way, and `tests/b30_line_smooth.rs` holds this to its numbers.

use crate::color::{linear_to_srgb, srgb_to_linear};
use crate::WorkingBuffer;

/// D-86: the end of a run this long whose crossing edge is this long is a corner, kept sharp.
const CORNER: isize = 4;

/// Smooth `source` in place. Softness 0 changes nothing.
///
/// ponytail: one thread, two passes over the whole cel; split the row pass by pairs of rows if
/// P-11's table ever ranks it.
pub(crate) fn line_smooth(source: &mut WorkingBuffer, softness: f64, threshold: f64) {
    let (w, h) = (source.width(), source.height());
    if softness <= 0.0 || w == 0 || h == 0 {
        return;
    }
    let p: Vec<[f32; 4]> = source.data().chunks_exact(4).map(encoded).collect();
    let mut s = Smooth {
        o: p.clone(),
        touched: vec![false; p.len()],
        p,
        threshold: (threshold / 255.0) as f32,
        slope: 50.0 / softness,
    };
    let (w, h) = (w as isize, h as isize);
    for y in 0..h - 1 {
        s.line(y, w, h, y * w, (y + 1) * w, 1, w, true);
    }
    for x in 0..w - 1 {
        s.line(x, h, w, x, x + 1, w, 1, false);
    }
    // Only a mixed pixel comes back through the curve; every other keeps its value exactly.
    for (i, px) in source.data_mut().chunks_exact_mut(4).enumerate() {
        if s.touched[i] {
            px.copy_from_slice(&working(s.o[i]));
        }
    }
}

/// D-86: the colour as a drawing program stores it, straight through the sRGB curve and
/// premultiplied again.
fn encoded(px: &[f32]) -> [f32; 4] {
    let a = px[3];
    if a <= 0.0 {
        return [0.0; 4];
    }
    [
        linear_to_srgb(px[0] / a) * a,
        linear_to_srgb(px[1] / a) * a,
        linear_to_srgb(px[2] / a) * a,
        a,
    ]
}

fn working(e: [f32; 4]) -> [f32; 4] {
    let a = e[3];
    if a <= 0.0 {
        return [0.0; 4];
    }
    [
        srgb_to_linear(e[0] / a) * a,
        srgb_to_linear(e[1] / a) * a,
        srgb_to_linear(e[2] / a) * a,
        a,
    ]
}

struct Smooth {
    /// The encoded source, read by both passes.
    p: Vec<[f32; 4]>,
    /// The encoded output both passes mix into.
    o: Vec<[f32; 4]>,
    touched: Vec<bool>,
    threshold: f32,
    slope: f64,
}

impl Smooth {
    /// D-86: `<=` where OpenToonz has `<`.
    fn eq(&self, a: isize, b: isize) -> bool {
        let (a, b) = (self.p[a as usize], self.p[b as usize]);
        (0..4).all(|i| (a[i] - b[i]).abs() <= self.threshold)
    }

    fn mix(&mut self, out: isize, other: isize, area: f64) {
        let (out, b, area) = (out as usize, self.p[other as usize], area as f32);
        self.touched[out] = true;
        for i in 0..4 {
            self.o[out][i] = self.o[out][i] * (1.0 - area) + b[i] * area;
        }
    }

    /// OpenToonz's checkNeighbourHood: when both diagonals could be joined, join the minority.
    #[allow(clippy::too_many_arguments)]
    fn neighbourhood(&self, x: isize, y: isize, pix: isize, lx: isize, ly: isize, dx: isize, dy: isize) -> bool {
        let (mut c1, mut c2) = (0, 0);
        let mut count = |a: isize, b: isize| {
            c1 += self.eq(pix - dx, a) as i32 + self.eq(pix - dx, b) as i32;
            c2 += self.eq(pix, a) as i32 + self.eq(pix, b) as i32;
        };
        if y > 1 {
            count(pix - 2 * dy, pix - 2 * dy - dx);
        }
        if y < ly - 1 {
            count(pix + dy, pix + dy - dx);
        }
        if x > 1 {
            count(pix - 2 * dx, pix - 2 * dx - dy);
        }
        if x < lx - 1 {
            count(pix + dx, pix + dx - dy);
        }
        c1 > c2
    }

    /// OpenToonz's filterLine: the pixels under the slope take the other line's colour by the
    /// area of them the slope leaves on its side.
    #[allow(clippy::too_many_arguments)]
    fn filter(&mut self, mut in_l: isize, mut in_u: isize, mut out: isize, ll: isize, step: isize, slope: f64, lower: bool) {
        let mut h0 = 0.5;
        let base = h0 / slope;
        let end = (base.floor() as isize).min(ll);
        for _ in 0..end {
            let h1 = h0 - slope;
            self.mix(out, if lower { in_u } else { in_l }, 0.5 * (h0 + h1));
            in_l += step;
            in_u += step;
            out += step;
            h0 = h1;
        }
        if end < ll {
            self.mix(out, if lower { in_u } else { in_l }, 0.5 * (base - end as f64) * h0);
        }
    }

    /// OpenToonz's processLine for one pair of rows (or, turned, of columns), with D-86's corner
    /// guard at each end of a run.
    #[allow(clippy::too_many_arguments)]
    fn line(&mut self, r: isize, lx: isize, ly: isize, l_row: isize, u_row: isize, dx: isize, dy: isize, do1: bool) {
        let r = r + 1;
        let off = u_row - l_row;
        let l_end = l_row + lx * dx;
        let slope = self.slope;

        // How far the edge across the end of a run goes, from this pair outward, while both
        // sides of it keep their colours.
        let crossing = |s: &Self, al: isize, bl: isize, au: isize, bu: isize| -> isize {
            let mut n = 0;
            for (a, b, step, mut room) in [(au, bu, dy, ly - 1 - r), (al, bl, -dy, r - 1)] {
                if s.eq(a, b) {
                    continue;
                }
                n += 1;
                let (mut qa, mut qb) = (a + step, b + step);
                while room > 0 && s.eq(qa, a) && s.eq(qb, b) {
                    n += 1;
                    room -= 1;
                    qa += step;
                    qb += step;
                }
            }
            n
        };
        let corner = |s: &Self, len: isize, al: isize, bl: isize, au: isize, bu: isize| {
            len >= CORNER && crossing(s, al, bl, au, bu) >= CORNER
        };
        let check_length = |s: &Self, len: isize, l1: isize, u1: isize, l2: isize, u2: isize, unite_u: bool| {
            len > 1
                || do1
                    && (unite_u && r > 1 && !(s.eq(l1, l1 - dy) && s.eq(l2, l2 - dy))
                        || r < ly - 1 && !(s.eq(u1, u1 + dy) && s.eq(u2, u2 + dy)))
        };
        let right = |s: &mut Self, ll: isize, lr: isize, whole: bool| {
            let ur = lr + off;
            let len = (lr - ll) / dx;
            let (l1, u1) = (lr - dx, ur - dx);
            let x = (l1 - l_row) / dx;
            if corner(s, len, l1, lr, u1, ur) {
                return;
            }
            let mut unite_u = s.eq(u1, lr);
            let unite_l = s.eq(l1, ur);
            if unite_u || unite_l {
                if unite_u && unite_l {
                    unite_u = !s.neighbourhood(x + 1, r, ur, lx, ly, dx, dy);
                }
                if check_length(s, len, l1, u1, lr, ur, unite_u) {
                    let out = if unite_u { l1 } else { u1 };
                    let k = if whole { 2.0 } else { 1.0 };
                    s.filter(l1, u1, out, len, -dx, slope / (len as f64 * k), unite_u);
                }
            }
        };
        let left = |s: &mut Self, ll: isize, lr: isize, whole: bool| {
            let ul = ll + off;
            let len = (lr - ll) / dx;
            let (l0, u0) = (ll - dx, ul - dx);
            let x = (ll - l_row) / dx;
            if corner(s, len, l0, ll, u0, ul) {
                return;
            }
            let mut unite_u = s.eq(ul, l0);
            let unite_l = s.eq(ll, u0);
            if unite_u || unite_l {
                if unite_u && unite_l {
                    unite_u = s.neighbourhood(x, r, ul, lx, ly, dx, dy);
                }
                if check_length(s, len, l0, u0, ll, ul, unite_u) {
                    let out = if unite_u { ll } else { ul };
                    let k = if whole { 2.0 } else { 1.0 };
                    s.filter(ll, ul, out, len, dx, slope / (len as f64 * k), unite_u);
                }
            }
        };
        let same = |s: &Self, i: isize| s.eq(i, i + off);
        // Where the run starting at `i` ends: both lines keep their colours up to there.
        let run_from = |s: &Self, i: isize| {
            let mut j = i + dx;
            while j != l_end && s.eq(i, j) && s.eq(i + off, j + off) {
                j += dx;
            }
            j
        };

        let mut ll = l_row;
        let mut lr = l_end;
        if !same(self, ll) {
            lr = run_from(self, ll);
            if lr != l_end {
                right(self, ll, lr, true);
            }
            ll = lr;
        }
        while ll != l_end && same(self, ll) {
            ll += dx;
        }
        while ll != l_end {
            lr = run_from(self, ll);
            if lr == l_end {
                break;
            }
            left(self, ll, lr, false);
            right(self, ll, lr, false);
            ll = lr;
            while ll != l_end && same(self, ll) {
                ll += dx;
            }
        }
        if ll != l_end {
            left(self, ll, lr, true);
        }
    }
}
