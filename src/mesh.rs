//! D-601: mesh and brush warping's shared part (P0-18), this program's own rule, worked a second
//! way in `tools/mesh_reference.py`. Two ways to bend a drawing:
//!
//! - A forward mesh: the drawing cut into `cols` by `rows` equal cells, each grid corner moved to
//!   where the effect puts it, each cell two triangles; an output pixel takes the drawing from
//!   where it came from in the last numbered triangle holding it, and is clear in none.
//! - A backward field: a grid of steps over the drawing; the output at X is the drawing at X plus
//!   a share of the step there, the step read at X less an offset.
//!
//! The card works both from the same numbers (`gpu.rs`, warp modes 150 and 151), from the tile
//! lists below, so it finds the same triangle.

use crate::render::sample_bilinear;
use crate::WorkingBuffer;
use rayon::prelude::*;

/// The side of a tile the triangles are sorted into, the card's workgroup.
pub(crate) const TILE: usize = 16;

/// Where each grid corner lands, in the drawing's space, row by row: (cols + 1) (rows + 1) points.
pub(crate) struct Mesh {
    pub cols: usize,
    pub rows: usize,
    pub at: Vec<[f64; 2]>,
}

impl Mesh {
    /// The grid's own places over a drawing `dw` by `dh`: the mesh that changes nothing.
    pub fn grid(cols: usize, rows: usize, (dw, dh): (f64, f64)) -> Mesh {
        let at = (0..=rows).flat_map(|j| (0..=cols).map(move |i| [i as f64 / cols as f64 * dw, j as f64 / rows as f64 * dh])).collect();
        Mesh { cols, rows, at }
    }

    /// Triangle `t`'s corners (cell `t / 2`, row by row; first half, then second).
    fn triangle(&self, t: usize) -> [[f64; 2]; 3] {
        let (c, second) = (t / 2, t % 2 == 1);
        let (i, j, n) = (c % self.cols, c / self.cols, self.cols + 1);
        let a = self.at[j * n + i];
        let far = self.at[(j + 1) * n + i + 1];
        if second {
            [a, far, self.at[(j + 1) * n + i]]
        } else {
            [a, self.at[j * n + i + 1], far]
        }
    }

    /// Each tile's triangles, in order: the offsets (one past the last tile too) and the list,
    /// for a buffer `w` by `h` with the drawing's corner at `origin`.
    pub fn tiles(&self, (w, h): (usize, usize), (ox, oy): (usize, usize)) -> (usize, Vec<u32>, Vec<u32>) {
        let (tx, ty) = (w.div_ceil(TILE), h.div_ceil(TILE));
        let mut each: Vec<Vec<u32>> = vec![Vec::new(); tx * ty];
        let tile = |v: f64, o: usize, n: usize| (((v + o as f64 - 0.5) / TILE as f64).floor().max(0.0) as usize).min(n - 1);
        for t in 0..self.cols * self.rows * 2 {
            let p = self.triangle(t);
            if p.iter().flatten().any(|v| !v.is_finite()) {
                continue;
            }
            let (x0, x1) = (p[0][0].min(p[1][0]).min(p[2][0]), p[0][0].max(p[1][0]).max(p[2][0]));
            let (y0, y1) = (p[0][1].min(p[1][1]).min(p[2][1]), p[0][1].max(p[1][1]).max(p[2][1]));
            // Wholly past the buffer's last pixel centre or before its first holds none.
            if x1 + ox as f64 - 0.5 < 0.0 || y1 + oy as f64 - 0.5 < 0.0 || x0 + ox as f64 - 0.5 > (w - 1) as f64 || y0 + oy as f64 - 0.5 > (h - 1) as f64 {
                continue;
            }
            for b in tile(y0, oy, ty)..=tile(y1, oy, ty) {
                for a in tile(x0, ox, tx)..=tile(x1, ox, tx) {
                    each[b * tx + a].push(t as u32);
                }
            }
        }
        let mut offsets = vec![0u32];
        let mut list = Vec::new();
        for l in each {
            list.extend(l);
            offsets.push(list.len() as u32);
        }
        (tx, offsets, list)
    }

    /// Where the drawing's point `x` came from, in grid units, searching `list` from its end.
    fn locate(&self, list: &[u32], x: [f64; 2]) -> Option<(f64, f64)> {
        list.iter().rev().find_map(|&t| {
            let t = t as usize;
            let [a, b, c] = self.triangle(t);
            let (l1, l2) = barycentric(a, b, c, x)?;
            let (c, second) = (t / 2, t % 2 == 1);
            let (i, j) = ((c % self.cols) as f64, (c / self.cols) as f64);
            Some(if second { (i + l1, j + l1 + l2) } else { (i + l1 + l2, j + l2) })
        })
    }
}

/// (l1, l2) when `x` lies in the triangle `a b c`, exactly as `tools/mesh_reference.py` and the
/// card work it, else `None`.
fn barycentric(a: [f64; 2], b: [f64; 2], c: [f64; 2], x: [f64; 2]) -> Option<(f64, f64)> {
    let (bx, by) = (b[0] - a[0], b[1] - a[1]);
    let (cx, cy) = (c[0] - a[0], c[1] - a[1]);
    let d = bx * cy - by * cx;
    if d == 0.0 {
        return None;
    }
    let (px, py) = (x[0] - a[0], x[1] - a[1]);
    let l1 = (px * cy - py * cx) / d;
    let l2 = (bx * py - by * px) / d;
    (l1 >= 0.0 && l2 >= 0.0 && l1 + l2 <= 1.0).then_some((l1, l2))
}

/// D-601: the forward `mesh` over the drawing in `source` (its corner at `origin`), the same size.
pub(crate) fn draw(source: &mut WorkingBuffer, mesh: &Mesh, origin: (usize, usize)) {
    let (w, h) = (source.width(), source.height());
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    let (dw, dh) = (w as f64 - 2.0 * ox, h as f64 - 2.0 * oy);
    let (tx, offsets, list) = mesh.tiles((w, h), origin);
    let (cols, rows) = (mesh.cols as f64, mesh.rows as f64);
    let drawing = &*source;
    let mut out = WorkingBuffer::transparent(w, h);
    out.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (bx, by) = (i % w, i / w);
        let t = by / TILE * tx + bx / TILE;
        let mine = &list[offsets[t] as usize..offsets[t + 1] as usize];
        if let Some((u, v)) = mesh.locate(mine, [bx as f64 + 0.5 - ox, by as f64 + 0.5 - oy]) {
            px.copy_from_slice(&sample_bilinear(drawing, u / cols * dw + ox, v / rows * dh + oy));
        }
    });
    *source = out;
}

/// The card's numbers for `mesh` (warp mode 150): a 0, the columns and rows, the drawing's size
/// and corner, the tiles across, then where the points, the offsets and the list start, padded
/// to 16; then those three.
pub(crate) fn mesh_numbers(mesh: &Mesh, (w, h): (usize, usize), origin: (usize, usize)) -> Vec<f64> {
    let (tx, offsets, list) = mesh.tiles((w, h), origin);
    let (dw, dh) = ((w - 2 * origin.0) as f64, (h - 2 * origin.1) as f64);
    let points = 16;
    let offs = points + 2 * mesh.at.len();
    let start = offs + offsets.len();
    let mut k = vec![0.0, mesh.cols as f64, mesh.rows as f64, dw, dh, origin.0 as f64, origin.1 as f64, tx as f64, points as f64, offs as f64, start as f64];
    k.resize(points, 0.0);
    k.extend(mesh.at.iter().flatten());
    k.extend(offsets.iter().map(|&o| o as f64));
    k.extend(list.iter().map(|&t| t as f64));
    k
}

/// A backward field: `gx` by `gy` cells over the drawing, each corner's step, row by row.
pub(crate) struct Field {
    pub gx: usize,
    pub gy: usize,
    pub step: Vec<[f64; 2]>,
}

impl Field {
    /// No step anywhere.
    pub fn still(gx: usize, gy: usize) -> Field {
        Field { gx, gy, step: vec![[0.0; 2]; (gx + 1) * (gy + 1)] }
    }

    /// The step at the drawing's (x, y) of a drawing `dw` by `dh`: the four corners round it
    /// blended, the place held inside the grid.
    pub fn at(&self, (dw, dh): (f64, f64), x: f64, y: f64) -> [f64; 2] {
        let (gx, gy) = (self.gx as f64, self.gy as f64);
        let fx = (x / dw * gx).clamp(0.0, gx);
        let fy = (y / dh * gy).clamp(0.0, gy);
        let (i, j) = ((fx.floor() as usize).min(self.gx - 1), (fy.floor() as usize).min(self.gy - 1));
        let (u, v) = (fx - i as f64, fy - j as f64);
        let n = self.gx + 1;
        let (p, q, r, s) = (self.step[j * n + i], self.step[j * n + i + 1], self.step[(j + 1) * n + i], self.step[(j + 1) * n + i + 1]);
        [0, 1].map(|k| (1.0 - v) * ((1.0 - u) * p[k] + u * q[k]) + v * ((1.0 - u) * r[k] + u * s[k]))
    }
}

/// D-601: the backward `field` over the drawing in `source` (its corner at `origin`), `share` of
/// each step, read `offset` back; the same size.
pub(crate) fn draw_field(source: &mut WorkingBuffer, field: &Field, share: f64, offset: [f64; 2], origin: (usize, usize)) {
    let (w, h) = (source.width(), source.height());
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    let size = (w as f64 - 2.0 * ox, h as f64 - 2.0 * oy);
    let drawing = &*source;
    let mut out = WorkingBuffer::transparent(w, h);
    out.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, px)| {
        let (x, y) = ((i % w) as f64 + 0.5 - ox, (i / w) as f64 + 0.5 - oy);
        let [dx, dy] = field.at(size, x - offset[0], y - offset[1]);
        px.copy_from_slice(&sample_bilinear(drawing, x + share * dx + ox, y + share * dy + oy));
    });
    *source = out;
}

/// The card's numbers for `field` (warp mode 151): a 0, the cells across and down, the
/// drawing's size and corner, the share, the offset, 0, 0; then the steps.
pub(crate) fn field_numbers(field: &Field, share: f64, offset: [f64; 2], (w, h): (usize, usize), origin: (usize, usize)) -> Vec<f64> {
    let (dw, dh) = ((w - 2 * origin.0) as f64, (h - 2 * origin.1) as f64);
    let mut k = vec![0.0, field.gx as f64, field.gy as f64, dw, dh, origin.0 as f64, origin.1 as f64, share, offset[0], offset[1], 0.0, 0.0];
    k.extend(field.step.iter().flatten());
    k
}

/// D-602: Bezier Warp's mesh, worked as `tools/bezier_warp_reference.py` works it: the Coons patch
/// of the outline's four cubic edges over the drawing's 4 q by 4 q grid, q `quality` rounded.
/// `points` are per cent of the drawing `dw` by `dh`, in [`crate::effects::BEZIER_POINTS`]'
/// order; `None` when every one is where it starts, the drawing as it was.
pub(crate) fn bezier_mesh(points: &[[f64; 2]; 12], quality: f64, (dw, dh): (f64, f64)) -> Option<Mesh> {
    if *points == crate::effects::BEZIER_START {
        return None;
    }
    let p = points.map(|q| [q[0] / 100.0 * dw, q[1] / 100.0 * dh]);
    let cubic = |a: [f64; 2], b: [f64; 2], c: [f64; 2], d: [f64; 2], t: f64| {
        let s = 1.0 - t;
        let (wa, wb, wc, wd) = (s * s * s, 3.0 * s * s * t, 3.0 * s * t * t, t * t * t);
        [0, 1].map(|k| wa * a[k] + wb * b[k] + wc * c[k] + wd * d[k])
    };
    let n = 4 * (quality + 0.5).floor() as usize;
    let (tl, tr, br, bl) = (p[0], p[3], p[6], p[9]);
    let at = (0..=n)
        .flat_map(|j| (0..=n).map(move |i| (i as f64 / n as f64, j as f64 / n as f64)))
        .map(|(u, v)| {
            let top = cubic(tl, p[1], p[2], tr, u);
            let right = cubic(tr, p[4], p[5], br, v);
            let bottom = cubic(bl, p[8], p[7], br, u);
            let left = cubic(tl, p[11], p[10], bl, v);
            [0, 1].map(|k| {
                (1.0 - v) * top[k] + v * bottom[k] + (1.0 - u) * left[k] + u * right[k]
                    - ((1.0 - u) * (1.0 - v) * tl[k] + u * (1.0 - v) * tr[k] + (1.0 - u) * v * bl[k] + u * v * br[k])
            })
        })
        .collect();
    Some(Mesh { cols: n, rows: n, at })
}

/// D-603: Mesh Warp's rows and columns as drawn, `rows` and `columns` rounded and held to 1 to 31.
pub(crate) fn warp_size(rows: f64, columns: f64) -> (usize, usize) {
    let n = |v: f64| (v + 0.5).floor().clamp(1.0, 31.0) as usize;
    (n(rows), n(columns))
}

/// D-603: how many numbers a whole Mesh Warp mesh of that size holds, x then y for each vertex.
pub(crate) fn warp_count(rows: f64, columns: f64) -> usize {
    let (r, c) = warp_size(rows, columns);
    2 * (r + 1) * (c + 1)
}

/// D-603: whether Mesh Warp's `v` leaves the drawing as it was: empty, or every vertex where the
/// mesh at rest has it (per cent, i / C 100 across and j / R 100 down).
pub(crate) fn warp_rests(rows: f64, columns: f64, v: &[f64]) -> bool {
    let (r, c) = warp_size(rows, columns);
    v.is_empty()
        || (v.len() == warp_count(rows, columns)
            && (0..=r).all(|j| {
                (0..=c).all(|i| {
                    let n = 2 * (j * (c + 1) + i);
                    v[n] == i as f64 / c as f64 * 100.0 && v[n + 1] == j as f64 / r as f64 * 100.0
                })
            }))
}

/// D-603: Mesh Warp's mesh, worked as `tools/mesh_warp_reference.py` works it: the Catmull-Rom
/// surface through the vertices `v` (per cent of the drawing `dw` by `dh`), each cell cut into
/// q by q, q `quality` rounded. `None` when the mesh rests, the drawing as it was.
pub(crate) fn warp_mesh(rows: f64, columns: f64, quality: f64, v: &[f64], (dw, dh): (f64, f64)) -> Option<Mesh> {
    if warp_rests(rows, columns, v) {
        return None;
    }
    let (r, c) = warp_size(rows, columns);
    // Vertex (i, j) in pixels, carried on in a straight line past the edges, down first.
    fn vertex(v: &[f64], r: isize, c: isize, d: (f64, f64), i: isize, j: isize) -> [f64; 2] {
        let line = |a: [f64; 2], b: [f64; 2]| [2.0 * a[0] - b[0], 2.0 * a[1] - b[1]];
        if j < 0 {
            return line(vertex(v, r, c, d, i, 0), vertex(v, r, c, d, i, 1));
        }
        if j > r {
            return line(vertex(v, r, c, d, i, r), vertex(v, r, c, d, i, r - 1));
        }
        if i < 0 {
            return line(vertex(v, r, c, d, 0, j), vertex(v, r, c, d, 1, j));
        }
        if i > c {
            return line(vertex(v, r, c, d, c, j), vertex(v, r, c, d, c - 1, j));
        }
        let n = 2 * (j * (c + 1) + i) as usize;
        [v[n] / 100.0 * d.0, v[n + 1] / 100.0 * d.1]
    }
    let cr = |p0: f64, p1: f64, p2: f64, p3: f64, s: f64| {
        0.5 * (2.0 * p1 + (p2 - p0) * s + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * s * s + (3.0 * p1 - p0 - 3.0 * p2 + p3) * s * s * s)
    };
    let (ri, ci) = (r as isize, c as isize);
    let q = (quality + 0.5).floor() as usize;
    let mut at = Vec::with_capacity((r * q + 1) * (c * q + 1));
    for b in 0..=r * q {
        let row = (b / q).min(r - 1);
        let t = (b - row * q) as f64 / q as f64;
        for a in 0..=c * q {
            let col = (a / q).min(c - 1);
            let s = (a - col * q) as f64 / q as f64;
            let across: Vec<[f64; 2]> = (row as isize - 1..row as isize + 3)
                .map(|jj| {
                    let p: Vec<[f64; 2]> = (col as isize - 1..col as isize + 3).map(|ii| vertex(v, ri, ci, (dw, dh), ii, jj)).collect();
                    [0, 1].map(|k| cr(p[0][k], p[1][k], p[2][k], p[3][k], s))
                })
                .collect();
            at.push([0, 1].map(|k| cr(across[0][k], across[1][k], across[2][k], across[3][k], t)));
        }
    }
    Some(Mesh { cols: c * q, rows: r * q, at })
}

/// D-604: Warp's grid, cells each way.
pub(crate) const WARP_GRID: usize = 64;

/// D-604: where Warp's `style` at bend `b` (Bend / 100) takes (u, v), each -1/2 to 1/2 across the
/// drawing, as `tools/warp_reference.py` works it.
fn warp_styled(style: &str, b: f64, u: f64, v: f64) -> (f64, f64) {
    use std::f64::consts::PI;
    let (e, f) = (1.0 - 4.0 * u * u, 1.0 - 4.0 * v * v);
    match style {
        "arc" => {
            if b == 0.0 {
                return (u, v);
            }
            let t = b * PI / 2.0;
            let big = 1.0 / t;
            let (a, r) = (u * t, big - v);
            (r * a.sin(), big - r * a.cos())
        }
        "arc_lower" => (u, v + b * e * (v + 0.5) / 2.0),
        "arc_upper" => (u, v - b * e * (0.5 - v) / 2.0),
        "arch" => (u, v - b * e / 2.0),
        "bulge" => (u, v * (1.0 + b * e / 2.0)),
        "shell_lower" => (u * (1.0 - b * (v + 0.5) / 2.0), v + b * e * (v + 0.5) / 2.0),
        "shell_upper" => (u * (1.0 - b * (0.5 - v) / 2.0), v - b * e * (0.5 - v) / 2.0),
        "flag" => (u, v + b * (2.0 * PI * u).sin() / 4.0),
        "wave" => (u, v + b * (2.0 * PI * u + PI * (v + 0.5)).sin() / 4.0),
        "fish" => (u, v * (1.0 + b * (2.0 * PI * u).sin() / 2.0)),
        "rise" => (u, v - b * (PI * u).sin() / 2.0),
        "fisheye" => {
            let s = 1.0 + b * (1.0 - 2.0 * (u * u + v * v)) / 2.0;
            (u * s, v * s)
        }
        "inflate" => (u * (1.0 + b * f / 2.0), v * (1.0 + b * e / 2.0)),
        "squeeze" => (u * (1.0 - b * f / 2.0), v * (1.0 + b * e / 2.0)),
        _ => {
            // twist
            let p = b * PI * (1.0 - 2.0 * (u * u + v * v)) / 2.0;
            (u * p.cos() - v * p.sin(), u * p.sin() + v * p.cos())
        }
    }
}

/// D-604: Warp's mesh over the drawing `dw` by `dh`: the `WARP_GRID` grid's points moved by the
/// style (u and v changing places about it with `axis` "vertical") and then the distortions.
/// `None` with Bend and both Distortions 0, the drawing as it was.
pub(crate) fn warp_preset_mesh(style: &str, axis: &str, bend: f64, horizontal: f64, vertical: f64, (dw, dh): (f64, f64)) -> Option<Mesh> {
    if bend == 0.0 && horizontal == 0.0 && vertical == 0.0 {
        return None;
    }
    let (b, h, d) = (bend / 100.0, horizontal / 100.0, vertical / 100.0);
    let g = WARP_GRID;
    let at = (0..=g)
        .flat_map(|c| (0..=g).map(move |a| (a as f64 / g as f64 - 0.5, c as f64 / g as f64 - 0.5)))
        .map(|(u, v)| {
            let (p, q) = if axis == "vertical" {
                let (q, p) = warp_styled(style, b, v, u);
                (p, q)
            } else {
                warp_styled(style, b, u, v)
            };
            let (p, q) = (p * (1.0 + d * q), q * (1.0 + h * p));
            [(p + 0.5) * dw, (q + 0.5) * dh]
        })
        .collect();
    Some(Mesh { cols: g, rows: g, at })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A drawing `w` by `h` whose every pixel is its own.
    fn drawing(w: usize, h: usize) -> WorkingBuffer {
        let mut b = WorkingBuffer::transparent(w, h);
        for (i, p) in b.data_mut().chunks_exact_mut(4).enumerate() {
            let (x, y) = ((i % w) as f32, (i / w) as f32);
            p.copy_from_slice(&[x / w as f32, y / h as f32, 0.5, 1.0]);
        }
        b
    }

    fn close(a: &WorkingBuffer, b: &WorkingBuffer) -> bool {
        a.data().iter().zip(b.data()).all(|(p, q)| (p - q).abs() < 1e-6)
    }

    #[test]
    fn the_grid_changes_nothing_and_a_move_moves() {
        let (w, h) = (40, 23);
        let plain = drawing(w, h);
        for (c, r) in [(1, 1), (3, 2), (17, 9)] {
            let mut b = plain.clone();
            draw(&mut b, &Mesh::grid(c, r, (w as f64, h as f64)), (0, 0));
            assert!(close(&b, &plain), "{c} by {r}");
        }
        let mut moved = Mesh::grid(5, 4, (w as f64, h as f64));
        moved.at.iter_mut().for_each(|p| p[0] += 2.0);
        let mut b = plain.clone();
        draw(&mut b, &moved, (0, 0));
        for y in 0..h {
            for x in 0..w {
                let got = &b.data()[(y * w + x) * 4..][..4];
                let want: &[f32] = if x < 2 { &[0.0; 4] } else { &plain.data()[(y * w + x - 2) * 4..][..4] };
                assert!(got.iter().zip(want).all(|(p, q)| (p - q).abs() < 1e-6), "{x} {y}");
            }
        }
    }

    #[test]
    fn in_a_fold_the_later_triangle_wins() {
        let (w, h) = (40, 20);
        let plain = drawing(w, h);
        let mut fold = Mesh::grid(2, 1, (w as f64, h as f64));
        fold.at.iter_mut().for_each(|p| p[0] = p[0].min(w as f64 - p[0]));
        let mut b = plain.clone();
        draw(&mut b, &fold, (0, 0));
        for y in 0..h {
            for x in 0..w / 2 {
                assert_eq!(&b.data()[(y * w + x) * 4..][..4], &plain.data()[(y * w + w - 1 - x) * 4..][..4]);
            }
            assert_eq!(&b.data()[(y * w + w - 1) * 4..][..4], &[0.0; 4]);
        }
    }

    #[test]
    fn the_tiles_hold_every_triangle_a_pixel_lies_in() {
        // A mesh flung about: the answer from the tiles is the answer from every triangle.
        let (w, h) = (70, 45);
        let mut m = Mesh::grid(6, 5, (w as f64, h as f64));
        for (n, p) in m.at.iter_mut().enumerate() {
            p[0] += ((n * 37 % 23) as f64 - 11.0) * 2.5;
            p[1] += ((n * 53 % 19) as f64 - 9.0) * 2.5;
        }
        let (tx, offsets, list) = m.tiles((w, h), (3, 2));
        let all: Vec<u32> = (0..(m.cols * m.rows * 2) as u32).collect();
        for y in 0..h {
            for x in 0..w {
                let t = y / TILE * tx + x / TILE;
                let p = [x as f64 + 0.5 - 3.0, y as f64 + 0.5 - 2.0];
                assert_eq!(m.locate(&list[offsets[t] as usize..offsets[t + 1] as usize], p), m.locate(&all, p), "{x} {y}");
            }
        }
    }

    #[test]
    fn a_still_field_changes_nothing_and_a_step_reads_aside() {
        let (w, h) = (30, 17);
        let plain = drawing(w, h);
        let mut b = plain.clone();
        draw_field(&mut b, &Field::still(4, 3), 1.0, [0.0; 2], (0, 0));
        assert!(close(&b, &plain));
        let mut one = Field::still(4, 3);
        one.step.iter_mut().for_each(|s| s[0] = 1.0);
        let mut b = plain.clone();
        draw_field(&mut b, &one, 1.0, [0.0; 2], (0, 0));
        for y in 0..h {
            for x in 0..w - 1 {
                assert_eq!(&b.data()[(y * w + x) * 4..][..4], &plain.data()[(y * w + x + 1) * 4..][..4]);
            }
        }
        let ramp = Field { gx: 4, gy: 3, step: (0..4).flat_map(|_| (0..5).map(|i| [i as f64 / 4.0 * 2.0, 0.0])).collect() };
        assert_eq!(ramp.at((w as f64, h as f64), w as f64 / 2.0, 3.0), [1.0, 0.0]);
        assert_eq!(ramp.at((w as f64, h as f64), -5.0, 3.0), [0.0, 0.0]);
    }
}
