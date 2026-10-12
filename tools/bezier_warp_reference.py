"""Bezier Warp, worked a second way.

D-602 adds `core.bezier_warp`, after After Effects' Bezier Warp (Distort): the drawing's outline
is four curved edges, each a cubic Bezier from one corner to the next with two tangent handles,
twelve points in all, and the drawing is bent to fill it. After Effects says what the controls
do and publishes no formula; the rule below is this program's own, and nothing is ported.

The rule. W and H are the drawing's own size. Every point is per cent of it, as Corner Pin's
pins are, turned into pixels first (x / 100 W, y / 100 H).

1. The edges, each the cubic Bezier
   P(t) = s^3 P0 + 3 s^2 t P1 + 3 s t^2 P2 + t^3 P3 with s = 1 - t (worked in that order):
   top T(u) from the top left vertex through the top left and top right tangents to the right
   top vertex; right R(v) from the right top vertex through the right top and right bottom
   tangents to the bottom right vertex; bottom Bm(u) from the left bottom vertex through the
   bottom left and bottom right tangents to the bottom right vertex; left L(v) from the top left
   vertex through the left top and left bottom tangents to the left bottom vertex.
2. The surface is the Coons patch of the four edges:
   S(u, v) = (1 - v) T(u) + v Bm(u) + (1 - u) L(v) + u R(v)
             - ((1 - u)(1 - v) TL + u (1 - v) TR + (1 - u) v BL + u v BR).
3. q = floor(`quality` + 0.5) and n = 4 q. The drawing's n by n grid goes to S(i / n, j / n),
   and the mesh is drawn as `tools/mesh_reference.py` (D-601) draws a forward mesh: inside the
   bent outline the drawing, outside it clear; where the outline folds over itself the part drawn
   later (lower and further right in the grid) is on top. Nothing grows: what is bent past the
   drawing's edges is cut off there, as After Effects cuts it.
4. With every point where it starts, the drawing, untouched.

`top_left_vertex` 0, 0; `top_left_tangent` 100/3, 0; `top_right_tangent` 200/3, 0;
`right_top_vertex` 100, 0; `right_top_tangent` 100, 100/3; `right_bottom_tangent` 100, 200/3;
`bottom_right_vertex` 100, 100; `bottom_right_tangent` 200/3, 100; `bottom_left_tangent` 100/3,
100; `left_bottom_vertex` 0, 100; `left_bottom_tangent` 0, 200/3; `left_top_tangent` 0, 100/3:
each -400 to 500 per cent each way, Corner Pin's range. `quality` 1 to 10, 8. All keyable.
Everything is a share of the drawing, so a draft needs no change.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 holding Bulge's striped drawing, the same size,
unmoved unless the case says. The drawing goes into `Fixtures/bezier_warp/media`, the projects
into `Fixtures/bezier_warp`, and the expected frames into
`Fixtures/bezier_warp/expected_bezier_warp.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/bezier_warp_reference.py
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
from radial_blur_reference import bilinear  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
import mesh_reference as M  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "bezier_warp"
TOLERANCE = 2e-5  # document 25's default for a filter
POINTS = ("top_left_vertex", "top_left_tangent", "top_right_tangent", "right_top_vertex",
          "right_top_tangent", "right_bottom_tangent", "bottom_right_vertex",
          "bottom_right_tangent", "bottom_left_tangent", "left_bottom_vertex",
          "left_bottom_tangent", "left_top_tangent")
NAMES = POINTS + ("quality",)
START = {"top_left_vertex": (0, 0), "top_left_tangent": (100 / 3, 0),
         "top_right_tangent": (200 / 3, 0), "right_top_vertex": (100, 0),
         "right_top_tangent": (100, 100 / 3), "right_bottom_tangent": (100, 200 / 3),
         "bottom_right_vertex": (100, 100), "bottom_right_tangent": (200 / 3, 100),
         "bottom_left_tangent": (100 / 3, 100), "left_bottom_vertex": (0, 100),
         "left_bottom_tangent": (0, 200 / 3), "left_top_tangent": (0, 100 / 3)}
RANGES = {**{k: (-400, 500) for k in POINTS}, "quality": (1, 10)}


# --- the rule -------------------------------------------------------------------------------

def cubic(p0, p1, p2, p3, t):
    s = 1 - t
    a, b, c, d = s * s * s, 3 * s * s * t, 3 * s * t * t, t * t * t
    return tuple(a * p0[k] + b * p1[k] + c * p2[k] + d * p3[k] for k in range(2))


def surface(s_, u, v, w=W, h=H):
    p = {k: (s_[k][0] / 100 * w, s_[k][1] / 100 * h) for k in POINTS}
    tl, tr = p["top_left_vertex"], p["right_top_vertex"]
    br, bl = p["bottom_right_vertex"], p["left_bottom_vertex"]
    t = cubic(tl, p["top_left_tangent"], p["top_right_tangent"], tr, u)
    r = cubic(tr, p["right_top_tangent"], p["right_bottom_tangent"], br, v)
    b = cubic(bl, p["bottom_left_tangent"], p["bottom_right_tangent"], br, u)
    l_ = cubic(tl, p["left_top_tangent"], p["left_bottom_tangent"], bl, v)
    return tuple((1 - v) * t[k] + v * b[k] + (1 - u) * l_[k] + u * r[k]
                 - ((1 - u) * (1 - v) * tl[k] + u * (1 - v) * tr[k] + (1 - u) * v * bl[k]
                    + u * v * br[k]) for k in range(2))


def cells(quality):
    return 4 * math.floor(quality + 0.5)


def bezier_warp(layer, s_):
    if all(tuple(s_[k]) == START[k] for k in POINTS):
        return layer
    n = cells(s_["quality"])
    points = [surface(s_, i / n, j / n) for j in range(n + 1) for i in range(n + 1)]
    return M.draw_mesh(layer, points, n, n, W, H)


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, quality=8, **more):
    c = {"drawing": "stripes", "shift": shift, "quality": quality, **START}
    c.update(more)
    return c


def settings(c, frame_no):
    held = {}
    for k in RANGES:
        v = value_at(c[k], frame_no)
        lo, hi = RANGES[k]
        held[k] = (tuple(min(hi, max(lo, u)) for u in v) if isinstance(v, (list, tuple))
                   else min(hi, max(lo, v)))
    return held


def render(c, frame_no):
    return frame(bezier_warp(B.drawn_layer(c["drawing"]), settings(c, frame_no)), c["shift"])


def plain(c):
    return frame(B.drawn_layer(c["drawing"]), c["shift"])


def scaled(lo, hi):
    """Every point moved into the square from lo to hi per cent, the thirds kept."""
    return {k: (lo + (hi - lo) * x / 100, lo + (hi - lo) * y / 100) for k, (x, y) in START.items()}


SAG = dict(top_left_tangent=(100 / 3, 60), top_right_tangent=(200 / 3, 60))
PINCH = dict(right_top_tangent=(50, 100 / 3), right_bottom_tangent=(50, 200 / 3))
S_CURVE = dict(top_left_tangent=(100 / 3, 50), top_right_tangent=(200 / 3, -50),
               bottom_left_tangent=(100 / 3, 150), bottom_right_tangent=(200 / 3, 50))
MIRROR = {k: (100 - v[0], v[1]) for k, v in START.items()}  # every point mirrored in place
CORNER = dict(top_left_vertex=(25, 25))

CASES = {
    "FX-BEZIER-001": ("The settings as they start: the drawing, untouched.", case(), [0]),
    "FX-BEZIER-002": ("The top left vertex pulled in to (25, 25), the tangents left where they "
                      "start: the top and left edges bend in to meet it, and the corner it left "
                      "is clear.", case(**CORNER), [0]),
    "FX-BEZIER-003": ("Both top tangents pulled down to 60 per cent: the top edge sags into the "
                      "drawing in a curve, the band of rows above it clear, deepest in the "
                      "middle.", case(**SAG), [0]),
    "FX-BEZIER-004": ("Both right tangents pulled in to 50 per cent across: the right edge "
                      "pinched in to a curve, the right middle columns clear.",
                      case(**PINCH), [0]),
    "FX-BEZIER-005": ("FX-BEZIER-003 at Quality 1: the same sag cut into 4 by 4 cells, so its "
                      "curve is a coarser chain of straight pieces and the frame differs.",
                      case(quality=1, **SAG), [0]),
    "FX-BEZIER-006": ("An S: the top edge's tangents at 50 and -50 per cent down, the bottom's "
                      "at 150 and 50: both edges wave the same way, the stripes between bent "
                      "into an S.", case(**S_CURVE), [0]),
    "FX-BEZIER-007": ("Every point moved into the square from 25 to 75 per cent, the tangents "
                      "still at the thirds: no curve, the drawing at half its size in the "
                      "middle, as an even squeeze draws it.", case(**scaled(25, 75)), [0]),
    "FX-BEZIER-008": ("Every point mirrored left for right in place: the outline is the same "
                      "square, run the other way, so the drawing is drawn mirrored.",
                      case(**MIRROR), [0]),
    "FX-BEZIER-009": ("The top left vertex keyed from (0, 0) at frame 0 to (25, 25) at frame 4, "
                      "linear: frame 0 the drawing, frame 4 FX-BEZIER-002, frame 2 half way.",
                      case(top_left_vertex=keyed((0, (0, 0)), (4, (25, 25)))), [0, 2, 4]),
    "FX-BEZIER-010": ("FX-BEZIER-002 with the layer moved three pixels right: the bent drawing "
                      "moves with it.", case(3, **CORNER), [0]),
    "FX-BEZIER-011": ("The bottom right vertex dragged past the top left one to (-20, -20): the "
                      "outline folds over itself; where it overlaps, the part later in the grid "
                      "is drawn on top.", case(bottom_right_vertex=(-20, -20)), [0]),
    "FX-BEZIER-012": ("Quality keyed from 1 at frame 0 to 10 at frame 4, eased past its end, "
                      "with the sag: frame 2 would pass 10 and is held there, the same as frame "
                      "4.", case(quality=keyed((0, 1, OVERSHOOT), (4, 10)), **SAG), [0, 2, 4]),
    "FX-BEZIER-013": ("The left top tangent flung out to (300, -200): the left edge loops far "
                      "out past the drawing and back, and what is bent past the edges is cut "
                      "off.", case(left_top_tangent=(300, -200)), [0]),
}

INVALID = {
    "FX-BEZIER-014": ("Quality 11, above 10.", case(quality=11)),
    "FX-BEZIER-015": ("Quality 0, below 1.", case(quality=0)),
    "FX-BEZIER-016": ("The top left vertex at (-401, 0), its x below -400.",
                      case(top_left_vertex=(-401, 0))),
    "FX-BEZIER-017": ("The bottom left tangent keyed to (100, 600) at frame 4.",
                      case(bottom_left_tangent=keyed((0, (100 / 3, 100)), (4, (100, 600))))),
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
    comp["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.bezier_warp", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
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

    (OUT / "expected_bezier_warp.json").write_bytes(
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

    # The thirds make straight edges run evenly, so the surface is the grid itself.
    for u, v in ((0, 0), (0.25, 0.5), (1, 1), (0.7, 0.2)):
        assert near(surface(START, u, v), (u * W, v * H), 1e-12)

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values())

    assert c["FX-BEZIER-001"]["0"] == drawn
    two = c["FX-BEZIER-002"]["0"]
    # (The drawing's own column 15 and rows 0 and 9 are empty.)
    assert clear(two[at(0, 1)]) and clear(two[at(1, 1)]) and clear(two[at(0, 2)])
    assert not clear(two[at(13, 8)]) and not same(two, drawn)
    three = c["FX-BEZIER-003"]["0"]
    assert all(clear(three[at(x, y)]) for x in range(4, 12) for y in (1, 2))
    assert not clear(three[at(0, 2)]) and not clear(three[at(0, 8)])
    assert sum(clear(three[at(8, y)]) for y in range(1, 9)) > sum(clear(three[at(2, y)])
                                                                  for y in range(1, 9))
    four = c["FX-BEZIER-004"]["0"]
    assert all(clear(four[at(x, y)]) for x in range(10, 14) for y in range(3, 7))
    assert not clear(four[at(10, 1)]) and not clear(four[at(8, 5)])
    assert not same(c["FX-BEZIER-005"]["0"], three)
    assert not same(c["FX-BEZIER-006"]["0"], drawn)
    seven = c["FX-BEZIER-007"]["0"]
    assert all(clear(seven[at(x, y)]) for x in range(W) for y in range(H)
               if not (4 <= x <= 11 and 2 <= y <= 7))
    assert all(near(seven[at(x, y)], bilinear(layer, (x + 0.5 - 4) * 2, (y + 0.5 - 2.5) * 2),
                    1e-9) for x in range(4, 12) for y in range(3, 7))
    eight = c["FX-BEZIER-008"]["0"]
    assert all(near(eight[at(x, y)], drawn[at(W - 1 - x, y)], 1e-9)
               for x in range(W) for y in range(H))
    nine = c["FX-BEZIER-009"]
    assert nine["0"] == drawn and same(nine["4"], two) and not same(nine["2"], two)
    assert same(c["FX-BEZIER-010"]["0"], frame(bezier_warp(layer, settings(case(**CORNER), 0)), 3))
    eleven = c["FX-BEZIER-011"]["0"]
    assert not same(eleven, drawn) and any(not clear(p) for p in eleven)
    twelve = c["FX-BEZIER-012"]
    assert value_at(keyed((0, 1, OVERSHOOT), (4, 10)), 2) > 10
    assert twelve["2"] == twelve["4"] and not same(twelve["4"], c["FX-BEZIER-003"]["0"])
    assert same(twelve["0"], c["FX-BEZIER-005"]["0"])
    assert not same(c["FX-BEZIER-013"]["0"], drawn)
    print("checked")


if __name__ == "__main__":
    main()
