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
        let at = (0..=rows)
            .flat_map(|j| {
                (0..=cols).map(move |i| [i as f64 / cols as f64 * dw, j as f64 / rows as f64 * dh])
            })
            .collect();
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
    pub fn tiles(
        &self,
        (w, h): (usize, usize),
        (ox, oy): (usize, usize),
    ) -> (usize, Vec<u32>, Vec<u32>) {
        let (tx, ty) = (w.div_ceil(TILE), h.div_ceil(TILE));
        let mut each: Vec<Vec<u32>> = vec![Vec::new(); tx * ty];
        let tile = |v: f64, o: usize, n: usize| {
            (((v + o as f64 - 0.5) / TILE as f64).floor().max(0.0) as usize).min(n - 1)
        };
        for t in 0..self.cols * self.rows * 2 {
            let p = self.triangle(t);
            if p.iter().flatten().any(|v| !v.is_finite()) {
                continue;
            }
            let (x0, x1) = (
                p[0][0].min(p[1][0]).min(p[2][0]),
                p[0][0].max(p[1][0]).max(p[2][0]),
            );
            let (y0, y1) = (
                p[0][1].min(p[1][1]).min(p[2][1]),
                p[0][1].max(p[1][1]).max(p[2][1]),
            );
            // Wholly past the buffer's last pixel centre or before its first holds none.
            if x1 + ox as f64 - 0.5 < 0.0
                || y1 + oy as f64 - 0.5 < 0.0
                || x0 + ox as f64 - 0.5 > (w - 1) as f64
                || y0 + oy as f64 - 0.5 > (h - 1) as f64
            {
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
            Some(if second {
                (i + l1, j + l1 + l2)
            } else {
                (i + l1 + l2, j + l2)
            })
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
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (bx, by) = (i % w, i / w);
            let t = by / TILE * tx + bx / TILE;
            let mine = &list[offsets[t] as usize..offsets[t + 1] as usize];
            if let Some((u, v)) = mesh.locate(mine, [bx as f64 + 0.5 - ox, by as f64 + 0.5 - oy]) {
                px.copy_from_slice(&sample_bilinear(
                    drawing,
                    u / cols * dw + ox,
                    v / rows * dh + oy,
                ));
            }
        });
    *source = out;
}

/// The card's numbers for `mesh` (warp mode 150): a 0, the columns and rows, the drawing's size
/// and corner, the tiles across, then where the points, the offsets and the list start, padded
/// to 16; then those three.
pub(crate) fn mesh_numbers(
    mesh: &Mesh,
    (w, h): (usize, usize),
    origin: (usize, usize),
) -> Vec<f64> {
    let (tx, offsets, list) = mesh.tiles((w, h), origin);
    let (dw, dh) = ((w - 2 * origin.0) as f64, (h - 2 * origin.1) as f64);
    let points = 16;
    let offs = points + 2 * mesh.at.len();
    let start = offs + offsets.len();
    let mut k = vec![
        0.0,
        mesh.cols as f64,
        mesh.rows as f64,
        dw,
        dh,
        origin.0 as f64,
        origin.1 as f64,
        tx as f64,
        points as f64,
        offs as f64,
        start as f64,
    ];
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
        Field {
            gx,
            gy,
            step: vec![[0.0; 2]; (gx + 1) * (gy + 1)],
        }
    }

    /// The step at the drawing's (x, y) of a drawing `dw` by `dh`: the four corners round it
    /// blended, the place held inside the grid.
    pub fn at(&self, (dw, dh): (f64, f64), x: f64, y: f64) -> [f64; 2] {
        let (gx, gy) = (self.gx as f64, self.gy as f64);
        let fx = (x / dw * gx).clamp(0.0, gx);
        let fy = (y / dh * gy).clamp(0.0, gy);
        let (i, j) = (
            (fx.floor() as usize).min(self.gx - 1),
            (fy.floor() as usize).min(self.gy - 1),
        );
        let (u, v) = (fx - i as f64, fy - j as f64);
        let n = self.gx + 1;
        let (p, q, r, s) = (
            self.step[j * n + i],
            self.step[j * n + i + 1],
            self.step[(j + 1) * n + i],
            self.step[(j + 1) * n + i + 1],
        );
        [0, 1]
            .map(|k| (1.0 - v) * ((1.0 - u) * p[k] + u * q[k]) + v * ((1.0 - u) * r[k] + u * s[k]))
    }
}

/// D-601: the backward `field` over the drawing in `source` (its corner at `origin`), `share` of
/// each step, read `offset` back; the same size.
pub(crate) fn draw_field(
    source: &mut WorkingBuffer,
    field: &Field,
    share: f64,
    offset: [f64; 2],
    origin: (usize, usize),
) {
    let (w, h) = (source.width(), source.height());
    let (ox, oy) = (origin.0 as f64, origin.1 as f64);
    let size = (w as f64 - 2.0 * ox, h as f64 - 2.0 * oy);
    let drawing = &*source;
    let mut out = WorkingBuffer::transparent(w, h);
    out.data_mut()
        .par_chunks_exact_mut(4)
        .enumerate()
        .for_each(|(i, px)| {
            let (x, y) = ((i % w) as f64 + 0.5 - ox, (i / w) as f64 + 0.5 - oy);
            let [dx, dy] = field.at(size, x - offset[0], y - offset[1]);
            px.copy_from_slice(&sample_bilinear(
                drawing,
                x + share * dx + ox,
                y + share * dy + oy,
            ));
        });
    *source = out;
}

/// The card's numbers for `field` (warp mode 151): a 0, the cells across and down, the
/// drawing's size and corner, the share, the offset, 0, 0; then the steps.
pub(crate) fn field_numbers(
    field: &Field,
    share: f64,
    offset: [f64; 2],
    (w, h): (usize, usize),
    origin: (usize, usize),
) -> Vec<f64> {
    let (dw, dh) = ((w - 2 * origin.0) as f64, (h - 2 * origin.1) as f64);
    let mut k = vec![
        0.0,
        field.gx as f64,
        field.gy as f64,
        dw,
        dh,
        origin.0 as f64,
        origin.1 as f64,
        share,
        offset[0],
        offset[1],
        0.0,
        0.0,
    ];
    k.extend(field.step.iter().flatten());
    k
}

/// D-602: Bezier Warp's mesh, worked as `tools/bezier_warp_reference.py` works it: the Coons patch
/// of the outline's four cubic edges over the drawing's 4 q by 4 q grid, q `quality` rounded.
/// `points` are per cent of the drawing `dw` by `dh`, in [`crate::effects::BEZIER_POINTS`]'
/// order; `None` when every one is where it starts, the drawing as it was.
pub(crate) fn bezier_mesh(
    points: &[[f64; 2]; 12],
    quality: f64,
    (dw, dh): (f64, f64),
) -> Option<Mesh> {
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
                    - ((1.0 - u) * (1.0 - v) * tl[k]
                        + u * (1.0 - v) * tr[k]
                        + (1.0 - u) * v * bl[k]
                        + u * v * br[k])
            })
        })
        .collect();
    Some(Mesh {
        cols: n,
        rows: n,
        at,
    })
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
/// mesh at rest has it (per cent, i / C 100 across and j / R 100 down). D-701: and no handle `h`
/// moved.
pub(crate) fn warp_rests(rows: f64, columns: f64, v: &[f64], h: &[f64]) -> bool {
    let (r, c) = warp_size(rows, columns);
    h.iter().all(|x| *x == 0.0)
        && (v.is_empty()
            || (v.len() == warp_count(rows, columns)
                && (0..=r).all(|j| {
                    (0..=c).all(|i| {
                        let n = 2 * (j * (c + 1) + i);
                        v[n] == i as f64 / c as f64 * 100.0
                            && v[n + 1] == j as f64 / r as f64 * 100.0
                    })
                })))
}

/// D-603: Mesh Warp's mesh, worked as `tools/mesh_warp_reference.py` works it: the Catmull-Rom
/// surface through the vertices `v` (per cent of the drawing `dw` by `dh`), each cell cut into
/// q by q, q `quality` rounded. `None` when the mesh rests, the drawing as it was.
///
/// D-701, as `tools/mesh_handles_reference.py` works it: `h`, empty or eight numbers a vertex
/// (right, left, lower and upper handle, x then y, per cent), moves the control points of each
/// cell's Bezier patch; the surface is linear in them, so each moved handle adds its offset times
/// the two Bernstein weights of the point it moves. An empty `v` is the mesh at rest.
pub(crate) fn warp_mesh(
    rows: f64,
    columns: f64,
    quality: f64,
    v: &[f64],
    h: &[f64],
    (dw, dh): (f64, f64),
) -> Option<Mesh> {
    if warp_rests(rows, columns, v, h) {
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
        if v.is_empty() {
            return [
                i as f64 / c as f64 * 100.0 / 100.0 * d.0,
                j as f64 / r as f64 * 100.0 / 100.0 * d.1,
            ];
        }
        let n = 2 * (j * (c + 1) + i) as usize;
        [v[n] / 100.0 * d.0, v[n + 1] / 100.0 * d.1]
    }
    // D-701: control point (a across, b down) of cell (col, row), moved by vertex (col + di,
    // row + dj)'s handle `which` (0 right, 1 left, 2 lower, 3 upper).
    const MOVES: [(usize, usize, usize, usize, usize); 16] = [
        (1, 0, 0, 0, 0),
        (2, 0, 1, 0, 1),
        (1, 3, 0, 1, 0),
        (2, 3, 1, 1, 1),
        (0, 1, 0, 0, 2),
        (0, 2, 0, 1, 3),
        (3, 1, 1, 0, 2),
        (3, 2, 1, 1, 3),
        (1, 1, 0, 0, 0),
        (1, 1, 0, 0, 2),
        (2, 1, 1, 0, 1),
        (2, 1, 1, 0, 2),
        (1, 2, 0, 1, 0),
        (1, 2, 0, 1, 3),
        (2, 2, 1, 1, 1),
        (2, 2, 1, 1, 3),
    ];
    let bern = |s: f64| {
        [
            (1.0 - s).powi(3),
            3.0 * s * (1.0 - s).powi(2),
            3.0 * s * s * (1.0 - s),
            s.powi(3),
        ]
    };
    let cr = |p0: f64, p1: f64, p2: f64, p3: f64, s: f64| {
        0.5 * (2.0 * p1
            + (p2 - p0) * s
            + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * s * s
            + (3.0 * p1 - p0 - 3.0 * p2 + p3) * s * s * s)
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
                    let p: Vec<[f64; 2]> = (col as isize - 1..col as isize + 3)
                        .map(|ii| vertex(v, ri, ci, (dw, dh), ii, jj))
                        .collect();
                    [0, 1].map(|k| cr(p[0][k], p[1][k], p[2][k], p[3][k], s))
                })
                .collect();
            let mut p =
                [0, 1].map(|k| cr(across[0][k], across[1][k], across[2][k], across[3][k], t));
            if !h.is_empty() {
                let (bs, bt) = (bern(s), bern(t));
                for (ca, cb, di, dj, which) in MOVES {
                    let n = 8 * ((row + dj) * (c + 1) + col + di) + 2 * which;
                    p[0] += bs[ca] * bt[cb] * h[n] / 100.0 * dw;
                    p[1] += bs[ca] * bt[cb] * h[n + 1] / 100.0 * dh;
                }
            }
            at.push(p);
        }
    }
    Some(Mesh {
        cols: c * q,
        rows: r * q,
        at,
    })
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
pub(crate) fn warp_preset_mesh(
    style: &str,
    axis: &str,
    bend: f64,
    horizontal: f64,
    vertical: f64,
    (dw, dh): (f64, f64),
) -> Option<Mesh> {
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
    Some(Mesh {
        cols: g,
        rows: g,
        at,
    })
}

/// D-605: where Liquify's brush lands along `stroke` (x then y, per cent of the drawing `dw` by
/// `dh`) at reach `reach`, and how far each dab moves, as `tools/liquify_reference.py` lays them.
fn liquify_dabs(stroke: &[f64], reach: f64, (dw, dh): (f64, f64)) -> Vec<([f64; 2], [f64; 2])> {
    let pts: Vec<[f64; 2]> = stroke
        .chunks_exact(2)
        .map(|p| [p[0] / 100.0 * dw, p[1] / 100.0 * dh])
        .collect();
    let pieces: Vec<([f64; 2], [f64; 2], f64)> = pts
        .windows(2)
        .map(|w| (w[0], w[1], (w[1][0] - w[0][0]).hypot(w[1][1] - w[0][1])))
        .collect();
    let length = pieces.iter().fold(0.0, |s, p| s + p.2);
    if length == 0.0 {
        return vec![(pts[0], [0.0, 0.0])];
    }
    let n = (length / (reach / 4.0).max(0.5)).ceil().max(1.0) as usize;
    let mut out = Vec::with_capacity(n);
    for k in 1..=n {
        let mut left = k as f64 * length / n as f64;
        for (m, &(a, b, l)) in pieces.iter().enumerate() {
            if l == 0.0 {
                continue;
            }
            if left <= l || m == pieces.len() - 1 {
                let u = (left / l).min(1.0);
                let c = [a[0] + (b[0] - a[0]) * u, a[1] + (b[1] - a[1]) * u];
                let d = [
                    (b[0] - a[0]) / l * length / n as f64,
                    (b[1] - a[1]) / l * length / n as f64,
                ];
                out.push((c, d));
                break;
            }
            left -= l;
        }
    }
    out
}

/// D-605: the step one dab of D-605's six tools gives the corner `p`, `f` the brush's weight
/// there, `c` the dab and `d` how far it moves.
fn liquify_step(tool: &str, f: f64, p: [f64; 2], c: [f64; 2], d: [f64; 2]) -> [f64; 2] {
    let (qx, qy) = (p[0] - c[0], p[1] - c[1]);
    match tool {
        "warp" => [-f * d[0], -f * d[1]],
        "twirl_clockwise" | "twirl_counterclockwise" => {
            let a = 0.2 * f * if tool == "twirl_clockwise" { -1.0 } else { 1.0 };
            [
                qx * a.cos() - qy * a.sin() - qx,
                qx * a.sin() + qy * a.cos() - qy,
            ]
        }
        "pucker" => [0.2 * f * qx, 0.2 * f * qy],
        "bloat" => [-0.2 * f * qx, -0.2 * f * qy],
        _ => [-f * d[1], f * d[0]], // shift_pixels
    }
}

/// D-700: Turbulence's value noise at (x, y) for channel `k`, as
/// `tools/liquify_brushes_reference.py` has it: -1 to 1.
fn liquify_noise(x: f64, y: f64, k: i64) -> f64 {
    let hash = |i: i64, j: i64| {
        let mut h = (73856093u32.wrapping_mul(i as u32))
            ^ (19349663u32.wrapping_mul(j as u32))
            ^ (83492791u32.wrapping_mul(k as u32));
        h ^= h >> 13;
        h = h.wrapping_mul(1540483477);
        h ^= h >> 15;
        2.0 * (h as f64 / 4294967296.0) - 1.0
    };
    let (i, j) = (x.floor(), y.floor());
    let (u, v) = (x - i, y - j);
    let (su, sv) = (u * u * (3.0 - 2.0 * u), v * v * (3.0 - 2.0 * v));
    let (i, j) = (i as i64, j as i64);
    let top = hash(i, j) + su * (hash(i + 1, j) - hash(i, j));
    let bottom = hash(i, j + 1) + su * (hash(i + 1, j + 1) - hash(i, j + 1));
    top + sv * (bottom - top)
}

/// D-700: what is wrong with one of Liquify's earlier strokes, if anything: its tool's number,
/// brush size, brush pressure, turbulent jitter, clone offset x and y and mode's number, then
/// one point or more, each held to its setting's range.
pub(crate) fn liquify_stroke_fault(e: &[f64]) -> Option<String> {
    if e.len() < 9 || e.len() % 2 == 0 {
        return Some(format!("is its tool, brush size, brush pressure, turbulent jitter, clone offset x and y and reconstruction mode, then x then y for one point or more, and this has {} numbers", e.len()));
    }
    let whole = |v: f64, hi: f64| v.fract() == 0.0 && (0.0..=hi).contains(&v);
    let limits = [
        ("brush size", 1, 1.0, 1000.0),
        ("brush pressure", 2, 1.0, 100.0),
        ("turbulent jitter", 3, 1.0, 100.0),
        ("clone offset x", 4, -10000.0, 10000.0),
        ("clone offset y", 5, -10000.0, 10000.0),
    ];
    if !whole(e[0], 11.0) {
        Some(format!(
            "has tool number {}, and the tools are numbered 0 to 11",
            e[0]
        ))
    } else if !whole(e[6], 3.0) {
        Some(format!(
            "has reconstruction mode number {}, and the modes are numbered 0 to 3",
            e[6]
        ))
    } else if let Some((name, i, lo, hi)) = limits
        .into_iter()
        .find(|&(_, i, lo, hi)| !(lo..=hi).contains(&e[i]))
    {
        Some(format!("has {name} {}, and it is {lo} to {hi}", e[i]))
    } else {
        e[7..]
            .iter()
            .find(|v| !(-100.0..=200.0).contains(*v))
            .map(|v| format!("has a point at {v} per cent, and they are -100 to 200"))
    }
}

/// D-605, D-700: Liquify's backward field over the drawing `dw` by `dh`, as
/// `tools/liquify_brushes_reference.py` works it: each stroke in turn, `strokes` and then
/// `stroke`, on a grid of cells two pixels across (at most 256 each way), held back where the
/// freeze area mask or an earlier Freeze Area holds it. `None` when it draws nothing: no stroke,
/// none of it drawn, or a freeze area mask named and not found.
pub(crate) fn liquify(effect: &crate::effects::Effect, (dw, dh): (f64, f64)) -> Option<Field> {
    let crate::effects::Effect::Liquify {
        tool,
        brush_size,
        brush_pressure,
        stroke,
        distortion_percentage,
        turbulent_jitter,
        clone_offset,
        reconstruction_mode,
        strokes,
        freeze_path,
        ..
    } = effect
    else {
        return None;
    };
    if (stroke.is_empty() && strokes.is_empty())
        || *distortion_percentage == 0.0
        || freeze_path.as_ref().is_some_and(|p| p.is_empty())
    {
        return None;
    }
    use crate::effects::{LIQUIFY_MODES, LIQUIFY_TOOLS};
    // Every stroke as (tool, brush size, pressure, jitter, clone offset, mode, points), oldest first.
    let mut all: Vec<(&str, f64, f64, f64, [f64; 2], &str, &[f64])> = strokes
        .iter()
        .map(|e| {
            (
                LIQUIFY_TOOLS[e[0] as usize],
                e[1],
                e[2],
                e[3],
                [e[4], e[5]],
                LIQUIFY_MODES[e[6] as usize],
                &e[7..],
            )
        })
        .collect();
    if !stroke.is_empty() {
        all.push((
            tool.as_str(),
            *brush_size,
            *brush_pressure,
            *turbulent_jitter,
            *clone_offset,
            reconstruction_mode.as_str(),
            stroke.as_slice(),
        ));
    }
    let (gx, gy) = (
        (dw / 2.0).ceil().min(256.0) as usize,
        (dh / 2.0).ceil().min(256.0) as usize,
    );
    let corner = |n: usize| {
        [
            (n % (gx + 1)) as f64 * dw / gx as f64,
            (n / (gx + 1)) as f64 * dh / gy as f64,
        ]
    };
    let count = (gx + 1) * (gy + 1);
    let inside: Vec<bool> = (0..count)
        .map(|n| {
            freeze_path
                .as_ref()
                .is_some_and(|m| crate::mask::point_inside(m, corner(n)[0], corner(n)[1]))
        })
        .collect();
    let mut s = Field::still(gx, gy);
    let mut z = vec![0.0; count];
    for (tool, size, pressure, jitter, clone, mode, pts) in all {
        let (reach, pressure) = (size / 2.0, pressure / 100.0);
        let landed = liquify_dabs(pts, reach, (dw, dh));
        let frozen: Vec<f64> = (0..count)
            .map(|n| if inside[n] { 1.0 } else { z[n] })
            .collect();
        let fall = |p: [f64; 2], c: [f64; 2]| {
            let t = (p[0] - c[0]).hypot(p[1] - c[1]) / reach;
            if t < 1.0 {
                pressure * (1.0 - t * t).powi(2)
            } else {
                0.0
            }
        };
        match tool {
            "freeze_area" | "thaw_area" => {
                for &(c, _) in &landed {
                    for (n, z) in z.iter_mut().enumerate() {
                        let f = fall(corner(n), c);
                        *z = if tool == "freeze_area" {
                            *z + f * (1.0 - *z)
                        } else {
                            *z - f * *z
                        };
                    }
                }
            }
            "reflection" | "clone" | "reconstruction" => {
                // Reconstruction's target, from the steps as the stroke began.
                let a = [pts[0] / 100.0 * dw, pts[1] / 100.0 * dh];
                let at = |x: f64, y: f64| s.at((dw, dh), x, y);
                let (hx, hy) = (dw / gx as f64, dh / gy as f64);
                let big_a = at(a[0], a[1]);
                let (r, l, d, u) = (
                    at(a[0] + hx, a[1]),
                    at(a[0] - hx, a[1]),
                    at(a[0], a[1] + hy),
                    at(a[0], a[1] - hy),
                );
                let gxv = [0, 1].map(|k| (r[k] - l[k]) / (2.0 * hx));
                let gyv = [0, 1].map(|k| (d[k] - u[k]) / (2.0 * hy));
                let lin = if mode == "amplitwist" {
                    let (al, be) = ((1.0 + gxv[0] + 1.0 + gyv[1]) / 2.0, (gxv[1] - gyv[0]) / 2.0);
                    [[al - 1.0, -be], [be, al - 1.0]]
                } else {
                    [[gxv[0], gyv[0]], [gxv[1], gyv[1]]]
                };
                let target = |p: [f64; 2]| match mode {
                    "revert" => [0.0, 0.0],
                    "displace" => big_a,
                    _ => [0, 1]
                        .map(|k| big_a[k] + lin[k][0] * (p[0] - a[0]) + lin[k][1] * (p[1] - a[1])),
                };
                let targets: Vec<[f64; 2]> = if tool == "reconstruction" {
                    (0..count).map(|n| target(corner(n))).collect()
                } else {
                    Vec::new()
                };
                for &(c, d) in &landed {
                    let before = Field {
                        gx,
                        gy,
                        step: s.step.clone(),
                    };
                    let ln = d[0].hypot(d[1]);
                    s.step.par_iter_mut().enumerate().for_each(|(n, sn)| {
                        let p = corner(n);
                        let f = fall(p, c);
                        if f == 0.0 {
                            return;
                        }
                        let t = match tool {
                            "reflection" => {
                                if ln == 0.0 {
                                    return;
                                }
                                let u = [d[0] / ln, d[1] / ln];
                                let q = [p[0] - c[0], p[1] - c[1]];
                                if u[0] * q[1] - u[1] * q[0] <= 0.0 {
                                    return;
                                }
                                let dot = q[0] * u[0] + q[1] * u[1];
                                [0, 1].map(|k| c[k] + 2.0 * dot * u[k] - q[k] - p[k])
                            }
                            "clone" => before.at((dw, dh), p[0] + clone[0], p[1] + clone[1]),
                            _ => targets[n],
                        };
                        let g = f * (1.0 - frozen[n]);
                        *sn = [0, 1].map(|k| sn[k] + g * (t[k] - sn[k]));
                    });
                }
            }
            _ => {
                // D-605's six and Turbulence: every dab's step added, less the frozen share.
                let jitter = jitter / 100.0;
                s.step.par_iter_mut().enumerate().for_each(|(n, sn)| {
                    let p = corner(n);
                    for &(c, d) in &landed {
                        let f = fall(p, c);
                        if f == 0.0 {
                            continue;
                        }
                        let st = if tool == "turbulence" {
                            let a = 0.1 * f * jitter * reach;
                            [
                                a * liquify_noise(p[0] / reach, p[1] / reach, 0),
                                a * liquify_noise(p[0] / reach, p[1] / reach, 1),
                            ]
                        } else {
                            liquify_step(tool, f, p, c, d)
                        };
                        sn[0] += (1.0 - frozen[n]) * st[0];
                        sn[1] += (1.0 - frozen[n]) * st[1];
                    }
                });
            }
        }
    }
    Some(s)
}

/// D-606: how many points each of Reshape's outlines is resampled to.
const RESHAPE_POINTS: usize = 64;

/// D-606: `outline` resampled to [`RESHAPE_POINTS`] points equally spaced along it, closed,
/// starting at its first point, as `tools/reshape_reference.py` lays them.
pub(crate) fn resample(outline: &[(f64, f64)]) -> Vec<[f64; 2]> {
    let n = outline.len();
    let pieces: Vec<((f64, f64), (f64, f64))> =
        (0..n).map(|i| (outline[i], outline[(i + 1) % n])).collect();
    let lengths: Vec<f64> = pieces
        .iter()
        .map(|(a, b)| (b.0 - a.0).hypot(b.1 - a.1))
        .collect();
    let total = lengths.iter().fold(0.0, |s, l| s + l);
    let (mut i, mut c) = (0, 0.0);
    (0..RESHAPE_POINTS)
        .map(|m| {
            let s = m as f64 * total / RESHAPE_POINTS as f64;
            while i < n - 1 && c + lengths[i] <= s {
                c += lengths[i];
                i += 1;
            }
            let (a, b) = pieces[i];
            let u = if lengths[i] > 0.0 {
                (s - c) / lengths[i]
            } else {
                0.0
            };
            [a.0 + (b.0 - a.0) * u, a.1 + (b.1 - a.1) * u]
        })
        .collect()
}

/// D-606: the shoelace sum, its sign the way `outline` winds.
fn winding(outline: &[(f64, f64)]) -> f64 {
    let n = outline.len();
    (0..n).fold(0.0, |s, i| {
        let ((x0, y0), (x1, y1)) = (outline[i], outline[(i + 1) % n]);
        s + (x0 * y1 - x1 * y0)
    })
}

/// D-606: Reshape's source and destination outlines resampled, the destination first turned to
/// wind the source's way, keeping its first point.
pub(crate) fn reshape_points(
    source: &[(f64, f64)],
    destination: &[(f64, f64)],
) -> (Vec<[f64; 2]>, Vec<[f64; 2]>) {
    if winding(source) * winding(destination) < 0.0 {
        let turned: Vec<(f64, f64)> = std::iter::once(destination[0])
            .chain(destination[1..].iter().rev().copied())
            .collect();
        return (resample(source), resample(&turned));
    }
    (resample(source), resample(destination))
}

/// D-702: the place at the fraction `s` of `outline`'s length, closed, as [`resample`] finds
/// its points.
fn along(outline: &[(f64, f64)], s: f64) -> [f64; 2] {
    let n = outline.len();
    let pieces: Vec<((f64, f64), (f64, f64))> =
        (0..n).map(|i| (outline[i], outline[(i + 1) % n])).collect();
    let lengths: Vec<f64> = pieces
        .iter()
        .map(|(a, b)| (b.0 - a.0).hypot(b.1 - a.1))
        .collect();
    let at = s * lengths.iter().fold(0.0, |s, l| s + l);
    let (mut i, mut c) = (0, 0.0);
    while i < n - 1 && c + lengths[i] <= at {
        c += lengths[i];
        i += 1;
    }
    let (a, b) = pieces[i];
    let u = if lengths[i] > 0.0 {
        (at - c) / lengths[i]
    } else {
        0.0
    };
    [a.0 + (b.0 - a.0) * u, a.1 + (b.1 - a.1) * u]
}

/// D-702: Reshape's points matched by the correspondence points, pairs of per cent places along
/// the source and the destination as drawn; none is D-606's [`reshape_points`]. Between two
/// pairs the source's points, equally spaced from the first pair's, answer places spread evenly
/// along the destination between the pairs' own, as `tools/reshape_points_reference.py` does.
pub(crate) fn reshape_matched(
    source: &[(f64, f64)],
    destination: &[(f64, f64)],
    pairs: &[f64],
) -> (Vec<[f64; 2]>, Vec<[f64; 2]>) {
    if pairs.len() < 2 {
        return reshape_points(source, destination);
    }
    let flip = winding(source) * winding(destination) < 0.0;
    let turned: Vec<(f64, f64)> = if flip {
        std::iter::once(destination[0])
            .chain(destination[1..].iter().rev().copied())
            .collect()
    } else {
        destination.to_vec()
    };
    let frac = |p: f64| (p / 100.0).rem_euclid(1.0);
    let mut ps: Vec<(f64, f64)> = pairs
        .chunks_exact(2)
        .map(|c| {
            (
                frac(c[0]),
                if flip {
                    (1.0 - frac(c[1])).rem_euclid(1.0)
                } else {
                    frac(c[1])
                },
            )
        })
        .collect();
    ps.sort_by(|a, b| a.0.total_cmp(&b.0));
    let a0 = ps[0].0;
    let o: Vec<f64> = ps
        .iter()
        .map(|p| p.0 - a0)
        .chain(std::iter::once(1.0))
        .collect();
    (0..RESHAPE_POINTS)
        .map(|k| {
            let r = k as f64 / RESHAPE_POINTS as f64;
            let i = (0..ps.len()).rev().find(|&j| o[j] <= r).unwrap_or(0);
            let next = ps[(i + 1) % ps.len()].1;
            let f = (r - o[i]) / (o[i + 1] - o[i]);
            let g = if ps.len() == 1 {
                1.0
            } else {
                (next - ps[i].1).rem_euclid(1.0)
            };
            (
                along(source, (a0 + r).rem_euclid(1.0)),
                along(&turned, (ps[i].1 + f * g).rem_euclid(1.0)),
            )
        })
        .unzip()
}

/// D-702: Reshape's Interpolation. `linear` or `smooth`, at the key time `u` strictly between
/// two of Percent's keys: the Percents to work the field at (each key's, held inside 0 to 100)
/// and their weights, a straight line between the two keys, or the uniform Catmull-Rom through
/// the key before, the two and the key after (an end key standing in for one not there); the
/// keys' eases are not read. Empty, for Percent at `u` alone (D-606), anywhere else.
pub(crate) fn reshape_between(
    interpolation: &str,
    keys: &[crate::model::Keyframe],
    u: f64,
) -> Vec<(f64, f64)> {
    if !matches!(interpolation, "linear" | "smooth") {
        return Vec::new();
    }
    let Some(j) = keys
        .windows(2)
        .position(|w| (w[0].frame as f64) < u && u < w[1].frame as f64)
    else {
        return Vec::new();
    };
    let val = |i: isize| {
        keys[i.clamp(0, keys.len() as isize - 1) as usize]
            .value
            .as_scalar()
            .unwrap_or(0.0)
            .clamp(0.0, 100.0)
    };
    let j = j as isize;
    let s = (u - keys[j as usize].frame as f64)
        / (keys[j as usize + 1].frame - keys[j as usize].frame) as f64;
    if interpolation == "linear" {
        return vec![(val(j), 1.0 - s), (val(j + 1), s)];
    }
    let (s2, s3) = (s * s, s * s * s);
    vec![
        (val(j - 1), (-s3 + 2.0 * s2 - s) / 2.0),
        (val(j), (3.0 * s3 - 5.0 * s2 + 2.0) / 2.0),
        (val(j + 1), (-3.0 * s3 + 4.0 * s2 + s) / 2.0),
        (val(j + 2), (s3 - s2) / 2.0),
    ]
}

/// D-606: Reshape's field from compose's outlines (source, destination, then the boundary if
/// one is named), `percent` of the way, at `elasticity`. D-702: matched by `pairs`, and, when
/// `between` holds Percents and weights, those fields blended; `None` when every Percent is 0.
pub(crate) fn reshape(
    paths: &[Vec<(f64, f64)>],
    percent: f64,
    between: &[(f64, f64)],
    pairs: &[f64],
    elasticity: &str,
    size: (f64, f64),
) -> Option<Field> {
    let parts = if between.is_empty() {
        vec![(percent, 1.0)]
    } else {
        between.to_vec()
    };
    if parts.iter().all(|(p, _)| *p == 0.0) {
        return None;
    }
    let (src, dst) = reshape_matched(&paths[0], &paths[1], pairs);
    let e = RESHAPE_ELASTICITY
        .iter()
        .find(|(w, _)| *w == elasticity)
        .map_or(2.5, |(_, e)| *e);
    let mut total: Option<Field> = None;
    for (p, w) in parts {
        let f = reshape_field(
            &src,
            &dst,
            paths.get(2).map(Vec::as_slice),
            p / 100.0,
            e,
            size,
        );
        match &mut total {
            None => {
                total = Some(Field {
                    step: f.step.iter().map(|v| [w * v[0], w * v[1]]).collect(),
                    ..f
                })
            }
            Some(t) => t
                .step
                .iter_mut()
                .zip(&f.step)
                .for_each(|(a, v)| *a = [a[0] + w * v[0], a[1] + w * v[1]]),
        }
    }
    total
}

/// D-606: the power each elasticity gives the inverse-distance blend, stiff to liquid.
pub(crate) const RESHAPE_ELASTICITY: [(&str, f64); 7] = [
    ("stiff", 1.0),
    ("less_stiff", 1.5),
    ("below_normal", 2.0),
    ("normal", 2.5),
    ("above_normal", 3.0),
    ("super_fluid", 4.0),
    ("liquid", 5.0),
];

/// D-606: Reshape's backward field over the drawing `dw` by `dh`: each source point `src[k]`
/// pulled `t` of the way to `dst[k]` reads the drawing back at the source point; the boundary's
/// points are held; the steps blended by inverse distance to the power `e` at each corner of a
/// grid of cells two pixels across (at most 256 each way), 0 outside the boundary.
pub(crate) fn reshape_field(
    src: &[[f64; 2]],
    dst: &[[f64; 2]],
    boundary: Option<&[(f64, f64)]>,
    t: f64,
    e: f64,
    (dw, dh): (f64, f64),
) -> Field {
    let (gx, gy) = (
        (dw / 2.0).ceil().min(256.0) as usize,
        (dh / 2.0).ceil().min(256.0) as usize,
    );
    let rect = [(0.0, 0.0), (dw, 0.0), (dw, dh), (0.0, dh)];
    let mut anchors: Vec<([f64; 2], [f64; 2])> = src
        .iter()
        .zip(dst)
        .map(|(s, d)| {
            let q = [s[0] + t * (d[0] - s[0]), s[1] + t * (d[1] - s[1])];
            (q, [s[0] - q[0], s[1] - q[1]])
        })
        .collect();
    anchors.extend(
        resample(boundary.unwrap_or(&rect))
            .into_iter()
            .map(|b| (b, [0.0, 0.0])),
    );
    let step = (0..(gx + 1) * (gy + 1))
        .into_par_iter()
        .map(|n| {
            let p = [
                (n % (gx + 1)) as f64 * dw / gx as f64,
                (n / (gx + 1)) as f64 * dh / gy as f64,
            ];
            if boundary.is_some_and(|b| !crate::mask::point_inside(b, p[0], p[1])) {
                return [0.0, 0.0];
            }
            let (mut nx, mut ny, mut den) = (0.0, 0.0, 0.0);
            for (q, v) in &anchors {
                let (dx, dy) = (p[0] - q[0], p[1] - q[1]);
                let d2 = dx * dx + dy * dy;
                if d2 == 0.0 {
                    return *v;
                }
                let w = d2.powf(-e / 2.0);
                nx += w * v[0];
                ny += w * v[1];
                den += w;
            }
            [nx / den, ny / den]
        })
        .collect();
    Field { gx, gy, step }
}

/// D-607: Smear's source outline resampled as Reshape's is, and its destination: each point moved
/// about the points' mean (summed in order), scaled by `scale` per cent, turned `rotation` degrees
/// clockwise on the screen and moved by `offset` pixels, as `tools/mask_smear_reference.py` moves it.
pub(crate) fn smear_points(
    source: &[(f64, f64)],
    offset: [f64; 2],
    rotation: f64,
    scale: f64,
) -> (Vec<[f64; 2]>, Vec<[f64; 2]>) {
    let src = resample(source);
    let (mut cx, mut cy) = (0.0, 0.0);
    for p in &src {
        cx += p[0];
        cy += p[1];
    }
    let (cx, cy) = (cx / src.len() as f64, cy / src.len() as f64);
    let (s, a) = (scale / 100.0, rotation.to_radians());
    let (cos, sin) = (a.cos(), a.sin());
    let dst = src
        .iter()
        .map(|p| {
            let (dx, dy) = (s * (p[0] - cx), s * (p[1] - cy));
            [
                cx + offset[0] + (dx * cos - dy * sin),
                cy + offset[1] + (dx * sin + dy * cos),
            ]
        })
        .collect();
    (src, dst)
}

/// D-607: Smear's field from compose's outlines (the source, then the boundary if one is named),
/// drawn as Reshape's is.
pub(crate) fn smear(
    paths: &[Vec<(f64, f64)>],
    offset: [f64; 2],
    rotation: f64,
    scale: f64,
    percent: f64,
    elasticity: &str,
    size: (f64, f64),
) -> Field {
    let (src, dst) = smear_points(&paths[0], offset, rotation, scale);
    let e = RESHAPE_ELASTICITY
        .iter()
        .find(|(w, _)| *w == elasticity)
        .map_or(2.5, |(_, e)| *e);
    reshape_field(
        &src,
        &dst,
        paths.get(1).map(Vec::as_slice),
        percent / 100.0,
        e,
        size,
    )
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
        a.data()
            .iter()
            .zip(b.data())
            .all(|(p, q)| (p - q).abs() < 1e-6)
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
                let want: &[f32] = if x < 2 {
                    &[0.0; 4]
                } else {
                    &plain.data()[(y * w + x - 2) * 4..][..4]
                };
                assert!(
                    got.iter().zip(want).all(|(p, q)| (p - q).abs() < 1e-6),
                    "{x} {y}"
                );
            }
        }
    }

    #[test]
    fn in_a_fold_the_later_triangle_wins() {
        let (w, h) = (40, 20);
        let plain = drawing(w, h);
        let mut fold = Mesh::grid(2, 1, (w as f64, h as f64));
        fold.at
            .iter_mut()
            .for_each(|p| p[0] = p[0].min(w as f64 - p[0]));
        let mut b = plain.clone();
        draw(&mut b, &fold, (0, 0));
        for y in 0..h {
            for x in 0..w / 2 {
                assert_eq!(
                    &b.data()[(y * w + x) * 4..][..4],
                    &plain.data()[(y * w + w - 1 - x) * 4..][..4]
                );
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
                assert_eq!(
                    m.locate(&list[offsets[t] as usize..offsets[t + 1] as usize], p),
                    m.locate(&all, p),
                    "{x} {y}"
                );
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
                assert_eq!(
                    &b.data()[(y * w + x) * 4..][..4],
                    &plain.data()[(y * w + x + 1) * 4..][..4]
                );
            }
        }
        let ramp = Field {
            gx: 4,
            gy: 3,
            step: (0..4)
                .flat_map(|_| (0..5).map(|i| [i as f64 / 4.0 * 2.0, 0.0]))
                .collect(),
        };
        assert_eq!(
            ramp.at((w as f64, h as f64), w as f64 / 2.0, 3.0),
            [1.0, 0.0]
        );
        assert_eq!(ramp.at((w as f64, h as f64), -5.0, 3.0), [0.0, 0.0]);
    }
}
