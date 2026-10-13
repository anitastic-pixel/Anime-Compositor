"""Mesh Warp's vertex handles, worked a second way.

D-701 closes D-603's first gap: After Effects gives each of Mesh Warp's vertices Bezier handles,
and D-603's mesh had none. It adds one setting to `core.mesh_warp`, `vertex_handles`. After
Effects says what the handles do and publishes no formula; the rule below is this program's own,
and nothing is ported. Everything in `tools/mesh_warp_reference.py` (D-603) stands; with no
handles moved, every frame is D-603's.

The rule. W, H, R, C, q, the vertices V(i, j) and their carrying on past the edges are D-603's.

1. `vertex_handles` is a flat list of numbers, eight for each of the (R + 1) (C + 1) vertices, row
   by row from the top left as `distortion_mesh` is: the right handle's x and y, the left's, the
   lower's and the upper's, each how far that handle is moved from where the smooth mesh has it,
   per cent of the drawing (x / 100 W, y / 100 H). An empty list moves none.
2. Where the smooth mesh has a handle. D-603's Catmull-Rom surface is, on each cell, the bicubic
   Bezier patch whose control points are G = M P M^T, P the 4 by 4 vertices V(c - 1 .. c + 2,
   r - 1 .. r + 2) and M the rows (0, 1, 0, 0), (-1/6, 1, 1/6, 0), (0, 1/6, 1, -1/6),
   (0, 0, 1, 0). So the right handle of V(i, j) is at V(i, j) + (V(i + 1, j) - V(i - 1, j)) / 6,
   the left at V(i, j) - (V(i + 1, j) - V(i - 1, j)) / 6, the lower and upper the same down.
3. Each cell (c, r) is that patch with its control points moved: G[a][b], a across and b down,
   each 0 to 3. The edge points move with the handle they are: G[1][0] by V(c, r)'s right handle,
   G[2][0] by V(c + 1, r)'s left, G[1][3] by V(c, r + 1)'s right, G[2][3] by V(c + 1, r + 1)'s
   left, G[0][1] by V(c, r)'s lower, G[0][2] by V(c, r + 1)'s upper, G[3][1] by V(c + 1, r)'s
   lower, G[3][2] by V(c + 1, r + 1)'s upper. Each inner point moves by both handles of the
   corner it is next to: G[1][1] by V(c, r)'s right and lower, G[2][1] by V(c + 1, r)'s left and
   lower, G[1][2] by V(c, r + 1)'s right and upper, G[2][2] by V(c + 1, r + 1)'s left and upper.
   A handle on the mesh's outside (a left handle on the first column, say) moves nothing.
4. The surface at (s, t) on the cell is sum B_a(s) B_b(t) G[a][b], B the cubic Bernstein
   polynomials, and it is cut and drawn as D-603 cuts and draws it.
5. With the mesh at rest and no handle moved, the drawing, untouched; a moved handle bends the
   drawing even when no vertex has moved.

`vertex_handles` empty, or 8 (R + 1) (C + 1) numbers each -400 to 400; keyable, a keyed list the
same length at each key. A list of any other length, or a number past its range, is refused with
EFFECT_PARAMETER_INVALID. Everything is a share of the drawing, so a draft needs no change.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is D-603's composition, 16 pixels by 10 holding Bulge's striped drawing. The drawing
goes into `Fixtures/mesh_handles/media`, the projects into `Fixtures/mesh_handles`, and the
expected frames into `Fixtures/mesh_handles/expected_mesh_handles.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/mesh_handles_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import bulge_reference as B  # noqa: E402
import mesh_reference as M  # noqa: E402
import mesh_warp_reference as MW  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402

W, H = MW.W, MW.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "mesh_handles"
TOLERANCE = MW.TOLERANCE
NAMES = MW.NAMES + ("vertex_handles",)
RANGES = dict(MW.RANGES, vertex_handles=(-400, 400))
TO_BEZIER = ((0, 1, 0, 0), (-1 / 6, 1, 1 / 6, 0), (0, 1 / 6, 1, -1 / 6), (0, 0, 1, 0))


# --- the rule -------------------------------------------------------------------------------

def bern(s):
    return ((1 - s) ** 3, 3 * s * (1 - s) ** 2, 3 * s * s * (1 - s), s ** 3)


def handle(h, cols, i, j, which):
    """Vertex (i, j)'s handle `which` (0 right, 1 left, 2 lower, 3 upper), moved by, in pixels."""
    if not h:
        return (0.0, 0.0)
    n = 8 * (j * (cols + 1) + i) + 2 * which
    return (h[n] / 100 * W, h[n + 1] / 100 * H)


def patch(v, h, rows, cols, c, r):
    """The cell's 4 by 4 control points G[a][b], a across and b down, handles moved."""
    p = [[MW.vertex(v, rows, cols, c - 1 + a, r - 1 + b) for b in range(4)] for a in range(4)]
    g = [[tuple(sum(TO_BEZIER[a][x] * TO_BEZIER[b][y] * p[x][y][k] for x in range(4) for y in range(4))
                for k in range(2)) for b in range(4)] for a in range(4)]
    moves = {  # control point (a, b): the handles, as (i, j, which), that move it
        (1, 0): [(c, r, 0)], (2, 0): [(c + 1, r, 1)], (1, 3): [(c, r + 1, 0)], (2, 3): [(c + 1, r + 1, 1)],
        (0, 1): [(c, r, 2)], (0, 2): [(c, r + 1, 3)], (3, 1): [(c + 1, r, 2)], (3, 2): [(c + 1, r + 1, 3)],
        (1, 1): [(c, r, 0), (c, r, 2)], (2, 1): [(c + 1, r, 1), (c + 1, r, 2)],
        (1, 2): [(c, r + 1, 0), (c, r + 1, 3)], (2, 2): [(c + 1, r + 1, 1), (c + 1, r + 1, 3)],
    }
    for (a, b), hs in moves.items():
        for i, j, which in hs:
            d = handle(h, cols, i, j, which)
            g[a][b] = (g[a][b][0] + d[0], g[a][b][1] + d[1])
    return g


def surface(g, s, t):
    bs, bt = bern(s), bern(t)
    return tuple(sum(bs[a] * bt[b] * g[a][b][k] for a in range(4) for b in range(4)) for k in range(2))


def mesh_warp(layer, s_):
    rows, cols = math.floor(s_["rows"] + 0.5), math.floor(s_["columns"] + 0.5)
    v = list(s_["distortion_mesh"]) or MW.rest(rows, cols)
    h = list(s_["vertex_handles"])
    if v == MW.rest(rows, cols) and not any(h):
        return layer
    q = math.floor(s_["quality"] + 0.5)
    cells = {}
    points = []
    for b in range(rows * q + 1):
        r = min(b // q, rows - 1)
        for a in range(cols * q + 1):
            c = min(a // q, cols - 1)
            if (c, r) not in cells:
                cells[(c, r)] = patch(v, h, rows, cols, c, r)
            points.append(surface(cells[(c, r)], (a - c * q) / q, (b - r * q) / q))
    return M.draw_mesh(layer, points, cols * q, rows * q, W, H)


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, rows=2, columns=2, quality=8, distortion_mesh=(), vertex_handles=(), **more):
    c = MW.case(shift, rows, columns, quality, distortion_mesh, **more)
    c["vertex_handles"] = vertex_handles
    return c


def handles(rows, cols, moves):
    """No handle moved but those in `moves`: (i, j, which) to (x, y) per cent."""
    h = [0.0] * (8 * (rows + 1) * (cols + 1))
    for (i, j, which), (x, y) in moves.items():
        n = 8 * (j * (cols + 1) + i) + 2 * which
        h[n], h[n + 1] = x, y
    return h


def settings(c, frame_no):
    held = {}
    for k in RANGES:
        v = value_at(c[k], frame_no)
        lo, hi = RANGES[k]
        held[k] = ([min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple))
                   else min(hi, max(lo, v)))
    return held


def render(c, frame_no):
    return frame(mesh_warp(B.drawn_layer(c["drawing"]), settings(c, frame_no)), c["shift"])


def plain(c):
    return frame(B.drawn_layer(c["drawing"]), c["shift"])


LIFT = handles(2, 2, {(1, 1, 0): (0, -20)})  # the middle vertex's right handle lifted
SAG = handles(1, 1, {(0, 0, 0): (0, 30), (1, 0, 1): (0, 30)})  # the top edge's two handles down
TURN = handles(2, 2, {(1, 1, 2): (-15, 0), (1, 1, 3): (15, 0)})  # the middle's lower and upper turned

CASES = {
    "FX-MESHH-001": ("D-603's FX-MESH-003 (the middle vertex pushed to 75 per cent) with every "
                     "handle written and none moved: the same frame as FX-MESH-003.",
                     case(distortion_mesh=MW.PUSH, vertex_handles=handles(2, 2, {})), [0]),
    "FX-MESHH-002": ("The mesh at rest written out, every handle written and none moved: the "
                     "drawing, untouched.",
                     case(distortion_mesh=MW.rest(2, 2), vertex_handles=handles(2, 2, {})), [0]),
    "FX-MESHH-003": ("A 2 by 2 mesh at rest (no list), the middle vertex's right handle lifted 20 "
                     "per cent: no vertex moved, but the stripes right of the middle bend up.",
                     case(vertex_handles=LIFT), [0]),
    "FX-MESHH-004": ("FX-MESHH-003 with the mesh at rest written out: the same frame.",
                     case(distortion_mesh=MW.rest(2, 2), vertex_handles=LIFT), [0]),
    "FX-MESHH-005": ("A 1 by 1 mesh, the top edge's two handles pulled 30 per cent down: the top "
                     "edge sags in a curve, clear above it, the corners where they were.",
                     case(rows=1, columns=1, vertex_handles=SAG), [0]),
    "FX-MESHH-006": ("FX-MESH-003's push with the middle vertex's lower handle swung left and "
                     "upper right: the bow becomes an S, leaning the other way above and below.",
                     case(distortion_mesh=MW.PUSH, vertex_handles=TURN), [0]),
    "FX-MESHH-007": ("FX-MESHH-005's handles keyed from none moved at frame 0 to moved at frame "
                     "4, linear: frame 0 the drawing, frame 4 FX-MESHH-005, frame 2 between.",
                     case(rows=1, columns=1, vertex_handles=keyed((0, handles(1, 1, {})), (4, SAG))),
                     [0, 2, 4]),
    "FX-MESHH-008": ("FX-MESHH-006 with the layer moved three pixels right: the bent drawing moves "
                     "with it.", case(3, distortion_mesh=MW.PUSH, vertex_handles=TURN), [0]),
    "FX-MESHH-009": ("FX-MESHH-005 at Quality 1: the cell is drawn as one piece straight between "
                     "its corners, so the sag is lost and the drawing untouched.",
                     case(rows=1, columns=1, quality=1, vertex_handles=SAG), [0]),
    "FX-MESHH-010": ("A handle on the mesh's outside, the top left vertex's left handle, pulled "
                     "far: it moves nothing, the drawing untouched.",
                     case(vertex_handles=handles(2, 2, {(0, 0, 1): (-40, 30)})), [0]),
}

INVALID = {
    "FX-MESHH-011": ("A 2 by 2 mesh's handles written with 70 numbers, not 72.",
                     case(vertex_handles=LIFT[:70])),
    "FX-MESHH-012": ("A handle moved 401 per cent, past 400.",
                     case(vertex_handles=handles(2, 2, {(1, 1, 0): (401, 0)}))),
    "FX-MESHH-013": ("A handle keyed out to -500 per cent at frame 4.",
                     case(vertex_handles=keyed((0, handles(2, 2, {})), (4, handles(2, 2, {(1, 1, 0): (-500, 0)}))))),
}

# --- the project files ----------------------------------------------------------------------


def project_json(fx, c):
    p = MW.project_json(fx, c)
    params = p["compositions"][0]["layers"][0]["effects"][0]["parameters"]
    params["vertex_handles"] = setting_json(c["vertex_handles"])
    if isinstance(params["vertex_handles"], tuple):
        params["vertex_handles"] = list(params["vertex_handles"])
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_bytes((json.dumps(project_json(fx, c), indent=2) + "\n").encode("utf-8"))
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in B.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(MW.S.png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_mesh_handles.json").write_bytes(
        (json.dumps(expected, indent=1) + "\n").encode("utf-8"))
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda a, b, e=1e-9: all(near(p, q, e) for p, q in zip(a, b))  # noqa: E731
    clear = lambda p: all(u == 0 for u in p)  # noqa: E731

    # With no handle moved the patch is D-603's surface, inside and on the edges.
    v = MW.moved(3, 2, {(1, 1): (60, 30), (2, 2): (90, 70)})
    for c_, r_, s, t in ((0, 0, 0, 0), (1, 2, 0.5, 0.25), (1, 2, 1, 1), (0, 1, 0.3, 0.9)):
        assert near(surface(patch(v, [], 3, 2, c_, r_), s, t), MW.surface(v, 3, 2, c_, r_, s, t), 1e-9)

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values())

    three = MW.render(MW.case(rows=2, columns=2, distortion_mesh=MW.PUSH), 0)
    assert same(c["FX-MESHH-001"]["0"], three, 1e-9)
    assert c["FX-MESHH-002"]["0"] == drawn
    lift = c["FX-MESHH-003"]["0"]
    assert not same(lift, drawn) and same(c["FX-MESHH-004"]["0"], lift, 1e-12)
    sag = c["FX-MESHH-005"]["0"]
    assert clear(sag[at(8, 0)]) and clear(sag[at(8, 1)]) and not clear(sag[at(8, 3)])
    assert not clear(sag[at(0, 3)]) and not clear(sag[at(8, 9)])
    turn = c["FX-MESHH-006"]["0"]
    assert not same(turn, three) and not same(turn, drawn)
    seven = c["FX-MESHH-007"]
    assert seven["0"] == drawn and same(seven["4"], sag) and not same(seven["2"], sag)
    assert not same(seven["2"], drawn)
    assert same(c["FX-MESHH-008"]["0"], frame(mesh_warp(B.drawn_layer("stripes"), settings(
        case(distortion_mesh=MW.PUSH, vertex_handles=TURN), 0)), 3))
    assert c["FX-MESHH-009"]["0"] == drawn
    assert same(c["FX-MESHH-010"]["0"], drawn, 1e-12)
    print("checked")


if __name__ == "__main__":
    main()
