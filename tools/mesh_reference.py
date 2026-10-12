"""Mesh and brush warping's shared part (D-601), worked a second way.

P0-18's six effects (Bezier Warp, Mesh Warp, Warp, Liquify, Reshape, Smear) bend a drawing in
one of two ways, and this file holds both, for their own reference files to import. This
program's own rule; nothing is ported.

**A forward mesh.** The drawing (its own size W by H) is cut into `cols` by `rows` equal cells.
The effect says where each of the (cols + 1) (rows + 1) grid corners lands, in the drawing's
space: point (i, j), row by row, is where the drawing's (i / cols W, j / rows H) goes. Each cell
is two triangles, (i, j) (i + 1, j) (i + 1, j + 1) and then (i, j) (i + 1, j + 1) (i, j + 1),
and the triangles are numbered in that order, cell by cell, row by row. An output pixel centre
X lies in a triangle A B C when, with d = (B - A) x (C - A) not 0,
l1 = ((X - A) x (C - A)) / d and l2 = ((B - A) x (X - A)) / d are each at least 0 and l1 + l2
is at most 1 (worked exactly so, in this order). Where X lies in several triangles (a fold),
the last numbered wins; in none, the pixel is clear. In the winning triangle, X came from the
grid place A + l1 (B - A) + l2 (C - A), so from the drawing's (u / cols W, v / rows H), and the
pixel is document 21's bilinear sample there. A mesh whose points are the grid's own places
draws the drawing back, to within the sample's rounding.

**A backward field.** A grid of `gx` by `gy` cells over the drawing holds at each corner
(i, j), at the drawing's (i / gx W, j / gy H), a step (dx, dy). The step at any place is the
bilinear blend of the four corners round it, the place held inside the grid first. The output
at X is the drawing's bilinear sample at X + s D(X - o), `s` a share (Liquify's Distortion
Percentage) and `o` an offset (its Distortion Mesh Offset).

Everything works in double precision on lists.

    python tools/mesh_reference.py      (checks itself)
"""

import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from radial_blur_reference import bilinear  # noqa: E402

EMPTY = [0.0] * 4


def grid(cols, rows, w, h):
    """The grid's own places: the mesh that changes nothing."""
    return [(i / cols * w, j / rows * h) for j in range(rows + 1) for i in range(cols + 1)]


def locate(points, cols, rows, x, y):
    """Where the output point (x, y) came from in grid units (u, v), or None."""
    n = cols + 1
    found = None
    for j in range(rows):
        for i in range(cols):
            a = points[j * n + i]
            b = points[j * n + i + 1]
            c = points[(j + 1) * n + i + 1]
            e = points[(j + 1) * n + i]
            for t, (p, q, r) in enumerate(((a, b, c), (a, c, e))):
                hit = barycentric(p, q, r, x, y)
                if hit is None:
                    continue
                l1, l2 = hit
                found = (i + l1 + l2, j + l2) if t == 0 else (i + l1, j + l1 + l2)
    return found


def barycentric(a, b, c, x, y):
    """(l1, l2) when (x, y) lies in a b c, else None; exactly as the build works it."""
    bx, by = b[0] - a[0], b[1] - a[1]
    cx, cy = c[0] - a[0], c[1] - a[1]
    d = bx * cy - by * cx
    if d == 0:
        return None
    px, py = x - a[0], y - a[1]
    l1 = (px * cy - py * cx) / d
    l2 = (bx * py - by * px) / d
    if l1 >= 0 and l2 >= 0 and l1 + l2 <= 1:
        return l1, l2
    return None


def draw_mesh(layer, points, cols, rows, w, h):
    """The forward mesh over `layer` (its drawing w by h at its own left and top): same size."""
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            x, y = i + 0.5, j + 0.5  # the drawing's space; the layer starts at its own corner
            uv = locate(points, cols, rows, x, y)
            if uv is None:
                px.append(EMPTY)
            else:
                px.append(bilinear(layer, uv[0] / cols * w + layer["left"],
                                   uv[1] / rows * h + layer["top"]))
    return dict(layer, px=px)


def field_at(field, gx, gy, w, h, x, y):
    """The step at the drawing's (x, y): the four corners round it blended, held in the grid."""
    fx = min(max(x / w * gx, 0.0), float(gx))
    fy = min(max(y / h * gy, 0.0), float(gy))
    i, j = min(int(math.floor(fx)), gx - 1), min(int(math.floor(fy)), gy - 1)
    u, v = fx - i, fy - j
    n = gx + 1
    p, q = field[j * n + i], field[j * n + i + 1]
    r, s = field[(j + 1) * n + i], field[(j + 1) * n + i + 1]
    return tuple((1 - v) * ((1 - u) * p[k] + u * q[k]) + v * ((1 - u) * r[k] + u * s[k])
                 for k in range(2))


def draw_field(layer, field, gx, gy, w, h, share=1.0, offset=(0.0, 0.0)):
    """The backward field over `layer`: same size."""
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            x, y = i + 0.5, j + 0.5
            dx, dy = field_at(field, gx, gy, w, h, x - offset[0], y - offset[1])
            px.append(bilinear(layer, x + share * dx + layer["left"],
                               y + share * dy + layer["top"]))
    return dict(layer, px=px)


def check():
    import bulge_reference as B
    layer = B.drawn_layer("stripes")
    w, h = layer["w"], layer["h"]
    near = lambda p, q: all(abs(a - b) < 1e-9 for a, b in zip(p, q))  # noqa: E731

    # The grid's own places draw the drawing back.
    for cols, rows in ((1, 1), (3, 2), (8, 5)):
        out = draw_mesh(layer, grid(cols, rows, w, h), cols, rows, w, h)
        assert all(near(p, q) for p, q in zip(out["px"], layer["px"])), (cols, rows)
    # Every place moved two pixels right: the drawing two pixels right, its first two columns clear.
    moved = [(x + 2, y) for x, y in grid(4, 4, w, h)]
    out = draw_mesh(layer, moved, 4, 4, w, h)
    for j in range(h):
        for i in range(w):
            want = EMPTY if i < 2 else layer["px"][j * w + i - 2]
            assert near(out["px"][j * w + i], want), (i, j)
    # A fold: the right half laid back over the left; the later triangles win.
    folded = [(min(x, w - x), y) for x, y in grid(2, 1, w, h)]
    out = draw_mesh(layer, folded, 2, 1, w, h)
    for j in range(h):
        for i in range(w // 2):
            assert near(out["px"][j * w + i], layer["px"][j * w + (w - 1 - i)]), (i, j)
        assert out["px"][j * w + w - 1] == EMPTY
    # A field of zero steps changes nothing; a field of (1, 0) everywhere reads one pixel right.
    zero = [(0.0, 0.0)] * (5 * 4)
    out = draw_field(layer, zero, 4, 3, w, h)
    assert all(near(p, q) for p, q in zip(out["px"], layer["px"]))
    one = [(1.0, 0.0)] * (5 * 4)
    out = draw_field(layer, one, 4, 3, w, h)
    for j in range(h):
        for i in range(w):
            want = EMPTY if i == w - 1 else layer["px"][j * w + i + 1]
            assert near(out["px"][j * w + i], want), (i, j)
    # Half the share is half the step; an offset moves where the field is read.
    ramp = [(i / 4 * 2.0, 0.0) for j in range(4) for i in range(5)]
    assert near(field_at(ramp, 4, 3, w, h, w / 2, 3.0), (1.0, 0.0))
    assert near(field_at(ramp, 4, 3, w, h, -5.0, 3.0), (0.0, 0.0))
    print("checked")


if __name__ == "__main__":
    check()
