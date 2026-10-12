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
