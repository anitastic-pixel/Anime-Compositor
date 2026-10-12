"""Mesh Warp, worked a second way.

D-603 adds `core.mesh_warp`, after After Effects' Mesh Warp (Distort): a grid of `rows` by
`columns` cells laid over the drawing, whose vertices are dragged, and the drawing bent smoothly
with them. After Effects says what the controls do and publishes no formula; the rule below is
this program's own, and nothing is ported. After Effects also gives each vertex Bezier handles;
this effect has none, so its cells' edges bend only as the vertices round them pull (a gap,
written in document 21).

The rule. W and H are the drawing's own size; R = floor(`rows` + 0.5) and C =
floor(`columns` + 0.5).

1. `distortion_mesh` is a flat list of numbers, x then y for each of the (R + 1) (C + 1)
   vertices, row by row from the top left, each per cent of the drawing (x / 100 W,
   y / 100 H). An empty list is the mesh at rest: vertex (i, j) at (i / C W, j / R H).
2. Past the mesh's edges each vertex row and column is carried on in a straight line:
   V(i, -1) = 2 V(i, 0) - V(i, 1) and V(i, R + 1) = 2 V(i, R) - V(i, R - 1), and then the same
   across for i = -1 and C + 1 on those rows (worked down first, then across).
3. The surface through the vertices is the Catmull-Rom spline both ways: on the cell (c, r) at
   (s, t) in 0 to 1, each of the four vertex rows r - 1 to r + 2 is first worked across at s,
   CR(P0, P1, P2, P3, s) = 0.5 (2 P1 + (P2 - P0) s + (2 P0 - 5 P1 + 4 P2 - P3) s^2
   + (3 P1 - P0 - 3 P2 + P3) s^3) (in that order), from vertices c - 1 to c + 2, and the four
   results then down at t. The mesh at rest is the drawing's own grid.
4. q = floor(`quality` + 0.5). Each cell is cut into q by q, so the drawing's C q by R q grid
   point (a, b) is the surface on cell (min(a div q, C - 1), min(b div q, R - 1)) at its
   remainder over q (1 on the last), and the mesh is drawn as `tools/mesh_reference.py` (D-601)
   draws a forward mesh: outside the bent drawing clear, where it folds the part later in the
   grid on top. Nothing grows: what is bent past the edges is cut off, as After Effects cuts it.
5. With the mesh at rest, or every vertex where it rests, the drawing, untouched.

`rows` and `columns` 1 to 31, 7; `quality` 1 to 10, 8; `distortion_mesh` empty, or 2 (R + 1)
(C + 1) numbers each -400 to 500. A list of any other length is refused with
EFFECT_PARAMETER_INVALID. All keyable; a keyed mesh is a list of the same length at each key.
Everything is a share of the drawing, so a draft needs no change.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 holding Bulge's striped drawing, the same size,
unmoved unless the case says. The drawing goes into `Fixtures/mesh_warp/media`, the projects
into `Fixtures/mesh_warp`, and the expected frames into
`Fixtures/mesh_warp/expected_mesh_warp.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/mesh_warp_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import bulge_reference as B  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
import mesh_reference as M  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "mesh_warp"
TOLERANCE = 2e-5  # document 25's default for a filter
NAMES = ("rows", "columns", "quality", "distortion_mesh")
RANGES = {"rows": (1, 31), "columns": (1, 31), "quality": (1, 10), "distortion_mesh": (-400, 500)}


# --- the rule -------------------------------------------------------------------------------

def rest(rows, cols):
    """The mesh at rest, per cent, flat."""
    return [v for j in range(rows + 1) for i in range(cols + 1) for v in (i / cols * 100, j / rows * 100)]


def cr(p0, p1, p2, p3, s):
    return 0.5 * (2 * p1 + (p2 - p0) * s + (2 * p0 - 5 * p1 + 4 * p2 - p3) * s * s
                  + (3 * p1 - p0 - 3 * p2 + p3) * s * s * s)


def vertex(v, rows, cols, i, j):
    """V(i, j), in pixels, carried on in a straight line past the edges, down first."""
    if j < 0:
        return tuple(2 * a - b for a, b in zip(vertex(v, rows, cols, i, 0), vertex(v, rows, cols, i, 1)))
    if j > rows:
        return tuple(2 * a - b for a, b in zip(vertex(v, rows, cols, i, rows),
                                                vertex(v, rows, cols, i, rows - 1)))
    if i < 0:
        return tuple(2 * a - b for a, b in zip(vertex(v, rows, cols, 0, j), vertex(v, rows, cols, 1, j)))
    if i > cols:
        return tuple(2 * a - b for a, b in zip(vertex(v, rows, cols, cols, j),
                                                vertex(v, rows, cols, cols - 1, j)))
    n = 2 * (j * (cols + 1) + i)
    return (v[n] / 100 * W, v[n + 1] / 100 * H)


def surface(v, rows, cols, c, r, s, t):
    across = []
    for jj in range(r - 1, r + 3):
        p = [vertex(v, rows, cols, ii, jj) for ii in range(c - 1, c + 3)]
        across.append(tuple(cr(p[0][k], p[1][k], p[2][k], p[3][k], s) for k in range(2)))
    return tuple(cr(across[0][k], across[1][k], across[2][k], across[3][k], t) for k in range(2))


def mesh_warp(layer, s_):
    rows, cols = math.floor(s_["rows"] + 0.5), math.floor(s_["columns"] + 0.5)
    v = list(s_["distortion_mesh"])
    if not v or v == rest(rows, cols):
        return layer
    q = math.floor(s_["quality"] + 0.5)
    points = []
    for b in range(rows * q + 1):
        r = min(b // q, rows - 1)
        for a in range(cols * q + 1):
            c = min(a // q, cols - 1)
            points.append(surface(v, rows, cols, c, r, (a - c * q) / q, (b - r * q) / q))
    return M.draw_mesh(layer, points, cols * q, rows * q, W, H)


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, rows=7, columns=7, quality=8, distortion_mesh=(), **more):
    c = {"drawing": "stripes", "shift": shift, "rows": rows, "columns": columns, "quality": quality,
         "distortion_mesh": distortion_mesh}
    c.update(more)
    return c


def moved(rows, cols, moves):
    """The mesh at rest with the vertices in `moves` ((i, j) to (x, y) per cent) moved."""
    v = rest(rows, cols)
    for (i, j), (x, y) in moves.items():
        n = 2 * (j * (cols + 1) + i)
        v[n], v[n + 1] = x, y
    return v


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


PUSH = moved(2, 2, {(1, 1): (75, 50)})  # the middle vertex pushed a quarter right
WAVE = moved(3, 3, {(1, 0): (100 / 3, 20), (2, 0): (200 / 3, -15), (1, 3): (100 / 3, 115),
                    (2, 3): (200 / 3, 80)})
TWIST = moved(4, 2, {(1, 1): (65, 25), (1, 3): (35, 75), (1, 2): (50, 50)})

CASES = {
    "FX-MESH-001": ("The settings as they start, the mesh at rest (an empty list): the drawing, "
                    "untouched.", case(), [0]),
    "FX-MESH-002": ("The 7 by 7 mesh written out where it rests: the drawing, untouched.",
                    case(distortion_mesh=rest(7, 7)), [0]),
    "FX-MESH-003": ("A 2 by 2 mesh, its middle vertex pushed from 50 to 75 per cent across: the "
                    "stripes bow right round it, smoothly, and the edges stay.",
                    case(rows=2, columns=2, distortion_mesh=PUSH), [0]),
    "FX-MESH-004": ("FX-MESH-003 at Quality 1: each cell left whole, so the bow is a few "
                    "straight pieces and the frame differs.",
                    case(rows=2, columns=2, quality=1, distortion_mesh=PUSH), [0]),
    "FX-MESH-005": ("A 3 by 3 mesh, the top edge's inner vertices raised and lowered and the "
                    "bottom's too: the top and bottom edges wave, clear where they dip in.",
                    case(rows=3, columns=3, distortion_mesh=WAVE), [0]),
    "FX-MESH-006": ("A 1 by 1 mesh, its top left vertex pulled in to (20, 20): with only the "
                    "corners the bend is straight-edged, the corner it left clear.",
                    case(rows=1, columns=1, distortion_mesh=moved(1, 1, {(0, 0): (20, 20)})), [0]),
    "FX-MESH-007": ("FX-MESH-003's middle vertex keyed from where it rests at frame 0 to 75 per "
                    "cent at frame 4, linear: frame 0 the drawing, frame 4 FX-MESH-003, frame 2 "
                    "between.", case(rows=2, columns=2,
                                     distortion_mesh=keyed((0, rest(2, 2)), (4, PUSH))), [0, 2, 4]),
    "FX-MESH-008": ("FX-MESH-003 with the layer moved three pixels right: the bent drawing "
                    "moves with it.", case(3, rows=2, columns=2, distortion_mesh=PUSH), [0]),
    "FX-MESH-009": ("The middle vertex dragged out to 150 per cent, past the right edge: the "
                    "mesh folds; where it overlaps, the part later in the grid is on top.",
                    case(rows=2, columns=2, distortion_mesh=moved(2, 2, {(1, 1): (150, 50)})), [0]),
    "FX-MESH-010": ("Four rows by two columns, the inner vertices twisted: the stripes bend one "
                    "way above the middle and the other way below.",
                    case(rows=4, columns=2, distortion_mesh=TWIST), [0]),
    "FX-MESH-011": ("Quality keyed from 1 at frame 0 to 10 at frame 4, eased past its end, with "
                    "FX-MESH-003's push: frame 2 would pass 10 and is held there, the same as "
                    "frame 4.", case(rows=2, columns=2, quality=keyed((0, 1, OVERSHOOT), (4, 10)),
                                     distortion_mesh=PUSH), [0, 2, 4]),
    "FX-MESH-012": ("Rows 2.4 and Columns 1.6, which round to 2 and 2: the same as FX-MESH-003.",
                    case(rows=2.4, columns=1.6, distortion_mesh=PUSH), [0]),
}

INVALID = {
    "FX-MESH-013": ("Rows 32, above 31.", case(rows=32)),
    "FX-MESH-014": ("Columns 0, below 1.", case(columns=0)),
    "FX-MESH-015": ("A 2 by 2 mesh written with 16 numbers, not 18.",
                    case(rows=2, columns=2, distortion_mesh=PUSH[:16])),
    "FX-MESH-016": ("A vertex at 501 per cent across, past 500.",
                    case(rows=2, columns=2, distortion_mesh=moved(2, 2, {(1, 1): (501, 50)}))),
    "FX-MESH-017": ("Quality 11, above 10.", case(quality=11)),
    "FX-MESH-018": ("The middle vertex keyed out to 600 per cent at frame 4.",
                    case(rows=2, columns=2, distortion_mesh=keyed(
                        (0, rest(2, 2)), (4, moved(2, 2, {(1, 1): (600, 50)}))))),
}

# --- the project files ----------------------------------------------------------------------


def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    t = comp["layers"][0]["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    params = {k: setting_json(c[k]) for k in NAMES}
    if isinstance(params["distortion_mesh"], tuple):
        params["distortion_mesh"] = list(params["distortion_mesh"])
    comp["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.mesh_warp", "enabled": True,
        "parameters": params}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_bytes((json.dumps(project_json(fx, c), indent=2) + "\n").encode("utf-8"))
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in B.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))

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

    (OUT / "expected_mesh_warp.json").write_bytes(
        (json.dumps(expected, indent=1) + "\n").encode("utf-8"))
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    layer = B.drawn_layer("stripes")
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda a, b, e=1e-9: all(near(p, q, e) for p, q in zip(a, b))  # noqa: E731
    clear = lambda p: p == M.EMPTY  # noqa: E731

    # At rest the surface is the grid itself, inside and on the edges.
    v = rest(3, 2)
    for c_, r_, s, t in ((0, 0, 0, 0), (1, 2, 0.5, 0.25), (1, 2, 1, 1), (0, 1, 0.3, 0.9)):
        assert near(surface(v, 3, 2, c_, r_, s, t), ((c_ + s) / 2 * W, (r_ + t) / 3 * H), 1e-9)

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values())

    assert c["FX-MESH-001"]["0"] == drawn and c["FX-MESH-002"]["0"] == drawn
    three = c["FX-MESH-003"]["0"]
    assert not same(three, drawn)
    # The top and bottom edges stay where they were: their rows barely move.
    assert all(near(three[at(x, y)], drawn[at(x, y)], 2e-2) for x in range(W) for y in (0, H - 1))
    # Above the band, round the middle, the drawing comes from further left: the stripes move.
    assert any(not near(three[at(x, 3)], drawn[at(x, 3)], 1e-3) for x in range(6, 12))
    assert not same(c["FX-MESH-004"]["0"], three)
    five = c["FX-MESH-005"]["0"]
    assert any(clear(five[at(x, y)]) for x in range(5, 11) for y in (1, 2))
    six = c["FX-MESH-006"]["0"]
    assert clear(six[at(0, 1)]) and clear(six[at(1, 1)]) and not clear(six[at(13, 8)])
    seven = c["FX-MESH-007"]
    assert seven["0"] == drawn and same(seven["4"], three) and not same(seven["2"], three)
    assert not same(seven["2"], drawn)
    assert same(c["FX-MESH-008"]["0"], frame(mesh_warp(layer, settings(case(rows=2, columns=2,
                                                                         distortion_mesh=PUSH), 0)), 3))
    nine = c["FX-MESH-009"]["0"]
    assert not same(nine, drawn) and not same(nine, three)
    ten = c["FX-MESH-010"]["0"]
    assert not same(ten, drawn)
    eleven = c["FX-MESH-011"]
    assert value_at(keyed((0, 1, OVERSHOOT), (4, 10)), 2) > 10
    assert eleven["2"] == eleven["4"] and same(eleven["0"], c["FX-MESH-004"]["0"])
    assert same(c["FX-MESH-012"]["0"], three)
    print("checked")


if __name__ == "__main__":
    main()
