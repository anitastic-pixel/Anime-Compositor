"""Iris Wipe's After Effects shape controls, worked a second way.

D-1045 gives `core.iris_wipe` (D-158) After Effects' Iris Wipe controls: `points` (Iris Points),
`outer_radius` (Outer Radius), `use_inner` (Use Inner Radius), `inner_radius` (Inner Radius) and
`rotation` (Rotation). Adobe publishes no formula; the shape is read off the tutorial
rsGGCshMDr8 (9:12 to 9:40: Iris Points 6 gives a triangle standing on its base, 8 a diamond, and
Outer Radius growing from 0 opens a hole from the centre). Document 21 is the rule in words; this
file is the reference for the numbers document 25 pins against it.

The rule. n = floor(points). Below 6 the shape is D-158's circle. From 6 to 32 it is a star of n
points about the centre c: point j (0 to n - 1) at the angle rotation + 360 j / n degrees,
clockwise from straight up, at 1 for even j and q for odd j, where q = cos(360 / n degrees) (the
middle of the straight side between two outer points, so the shape is a polygon of n / 2
corners) unless `use_inner` is on and `outer_radius` is above 0, when q = inner_radius /
outer_radius. A point P's measure g is how far out P is in units of that shape: for P - c = (dx,
dy), j the sector P's angle falls in, V_j, V_j+1 its two points, g = cross(P - c, V_j+1 - V_j)
/ cross(V_j, V_j+1), so the shape's edge is g = 1 and the circle's g is the plain distance; g is
infinite off the centre when q is 0 (an empty star).

Outer Radius 0: D-158's rule with d replaced by g: R is the largest g at the four corners of the
drawing's own box. Above 0: Completion is not read; r = outer_radius, and the wipe is After
Effects' hole: invert off, k = clamp((g - r) / f + 0.5, 0, 1), with f = 0 k = 1 when g >= r and
0 otherwise; invert on, the inside kept, k = clamp((r - g) / f + 0.5, 0, 1), with f = 0 k = 1
when g <= r. The output is p k, all four channels.

**This file never runs the build's code path.** It works in double precision on lists.

    python tools/iris_shape_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import iris_wipe_reference as I  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = I.W, I.H
OUT = I.OUT
RANGES = dict(I.RANGES, points=(0, 32), outer_radius=(0, 100000), inner_radius=(0, 100000),
              rotation=(-100000, 100000))
NAMES = I.NAMES + ("points", "outer_radius", "use_inner", "inner_radius", "rotation")
CLIFF = 1e-5


def star(n, q, rotation):
    out = []
    for j in range(n):
        a = math.radians(rotation + 360 * j / n)
        rho = 1.0 if j % 2 == 0 else q
        out.append((rho * math.sin(a), -rho * math.cos(a)))
    return out


def measure(dx, dy, n, q, rotation):
    """g for the step (dx, dy) from the centre."""
    if n < 6:
        return math.hypot(dx, dy)
    if dx == 0 and dy == 0:
        return 0.0
    if q == 0:
        return math.inf
    v = star(n, q, rotation)
    t = (math.atan2(dx, -dy) - math.radians(rotation)) % (2 * math.pi)
    j = min(n - 1, int(t / (2 * math.pi / n)))
    a, b = v[j], v[(j + 1) % n]
    cross = lambda p, s: p[0] * s[1] - p[1] * s[0]  # noqa: E731
    return cross((dx, dy), (b[0] - a[0], b[1] - a[1])) / cross(a, b)


def shape_of(s):
    n = int(math.floor(s["points"]))
    q = math.cos(2 * math.pi / n) if n >= 6 else 1.0
    if n >= 6 and s["use_inner"] == "on" and s["outer_radius"] > 0:
        q = s["inner_radius"] / s["outer_radius"]
    return n, q


def keep(s, px, py, w=W, h=H):
    n, q = shape_of(s)
    cx, cy = s["center"][0] / 100 * w, s["center"][1] / 100 * h
    g = measure(px - cx, py - cy, n, q, s["rotation"])
    f = s["feather"]
    if s["outer_radius"] > 0:
        r = s["outer_radius"]
        inside = g - r if s["invert"] == "off" else r - g
    else:
        if s["completion"] == 0:
            return 1.0
        if s["completion"] == 100:
            return 0.0
        big = max(measure(a - cx, b - cy, n, q, s["rotation"]) for a in (0, w) for b in (0, h))
        cc = s["completion"] / 100
        r = ((1 - cc) if s["invert"] == "off" else cc) * (big + f) - f / 2
        inside = r - g if s["invert"] == "off" else g - r
    if f > 0:
        return min(1.0, max(0.0, inside / f + 0.5))
    return 1.0 if inside >= 0 else 0.0


def case(points=0, outer_radius=0, use_inner="off", inner_radius=0, rotation=0, **kw):
    c = I.case(**kw)
    c.update(points=points, outer_radius=outer_radius, use_inner=use_inner,
             inner_radius=inner_radius, rotation=rotation)
    return c


def settings(c, frame_no):
    held = {"invert": c["invert"], "use_inner": c["use_inner"]}
    for k, (lo, hi) in RANGES.items():
        v = value_at(c[k], frame_no)
        held[k] = ([min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple))
                   else min(hi, max(lo, v)))
    return held


def render(c, frame_no):
    s = settings(c, frame_no)
    pixels = I.drawn_pixels(c["drawing"])
    out = [[v * keep(s, i % W + 0.5, i // W + 0.5) for v in p] for i, p in enumerate(pixels)]
    return R.frame(out, c["shift"])


HALF = {"completion": 50}
CASES = {
    "FX-IRIS-028": ("Iris Points 6, completion 50: a triangle standing on its base, its top "
                    "point straight up from the middle, closing on (8, 5); inside it is kept "
                    "and outside it gone.",
                    case(points=6, **HALF), [0]),
    "FX-IRIS-029": ("Iris Points 8, completion 50: a diamond, four corners up, right, down and "
                    "left, closing on the middle.",
                    case(points=8, **HALF), [0]),
    "FX-IRIS-030": ("Iris Points 8, Outer Radius 4.3: After Effects' hole, a diamond reaching "
                    "4.3 pixels up, down, left and right from the middle, transparent; every "
                    "pixel outside it is kept exactly. Completion is not read.",
                    case(points=8, outer_radius=4.3), [0]),
    "FX-IRIS-031": ("Iris Points 10, Outer Radius 4.6, Use Inner Radius on, Inner Radius 2.1: a "
                    "five-pointed star hole, its top point straight up.",
                    case(points=10, outer_radius=4.6, use_inner="on", inner_radius=2.1), [0]),
    "FX-IRIS-032": ("FX-IRIS-030 turned 45 degrees: the diamond becomes a square standing "
                    "level, its corners 4.3 from the middle.",
                    case(points=8, outer_radius=4.3, rotation=45), [0]),
    "FX-IRIS-033": ("FX-IRIS-030 with feather 2: the hole's edge soft over 2 units of the "
                    "shape's own size, each pixel faded as a whole.",
                    case(points=8, outer_radius=4.3, feather=2), [0]),
    "FX-IRIS-034": ("FX-IRIS-030 with invert on: only the diamond is kept, everything outside "
                    "it gone, so the two add up to the drawing.",
                    case(points=8, outer_radius=4.3, invert="on"), [0]),
    "FX-IRIS-035": ("Iris Points 0, the circle, Outer Radius 4.3: a round hole of radius 4.3.",
                    case(outer_radius=4.3), [0]),
    "FX-IRIS-036": ("Iris Points 8, Outer Radius keyed from 0 at frame 0 to 8.6 at frame 4, "
                    "linear, as the tutorial keys it: frame 0 is the drawing (Outer Radius 0 "
                    "reads Completion, 0), frame 2 is FX-IRIS-030, and frame 4 a hole twice as "
                    "big.",
                    case(points=8, outer_radius=keyed((0, 0), (4, 8.6))), [0, 2, 4]),
    "FX-IRIS-037": ("Iris Points 4, below 6: the circle, FX-IRIS-003 exactly.",
                    case(points=4, **HALF), [0]),
    "FX-IRIS-038": ("Iris Points 6, completion 50, Use Inner Radius on with Inner Radius 1: "
                    "with Outer Radius 0 the inner radius is not read, so FX-IRIS-028 exactly.",
                    case(points=6, use_inner="on", inner_radius=1, **HALF), [0]),
    "FX-IRIS-039": ("FX-IRIS-029 moved three pixels right: the same, moved, and the three "
                    "columns left of the drawing stay empty.",
                    case(points=8, shift=3, **HALF), [0]),
}

INVALID = {
    "FX-IRIS-040": ("Iris Points 33, above 32.", case(points=33, **HALF)),
    "FX-IRIS-041": ("Iris Points -1, below 0.", case(points=-1, **HALF)),
    "FX-IRIS-042": ("Outer Radius -1, below 0.", case(points=8, outer_radius=-1)),
    "FX-IRIS-043": ("Inner Radius 100001, above 100000.",
                    case(points=8, outer_radius=4, use_inner="on", inner_radius=100001)),
    "FX-IRIS-044": ("Use Inner Radius \"yes\", not \"off\" or \"on\".",
                    case(points=8, outer_radius=4, use_inner="yes")),
    "FX-IRIS-045": ("Rotation 100001, past the most.", case(points=8, rotation=100001, **HALF)),
}


def project_json(fx, c):
    p = I.project_json(fx, c)
    p["compositions"][0]["layers"][0]["effects"][0]["parameters"] = {
        k: setting_json(c[k]) for k in NAMES}
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    expected = {"tolerance": I.TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = I.plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = I.plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_iris_shape.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def check(expected):
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = I.plain(I.case())
    at = lambda x, y: y * W + x  # noqa: E731
    old = json.loads((OUT / "expected_iris_wipe.json").read_text(encoding="utf-8"))["cases"]
    # The measure: the star's own points are at 1, the side's middle at q.
    assert abs(measure(0, -1, 6, 0.5, 0) - 1) < 1e-12  # the top point
    assert abs(measure(0, 0.5, 6, 0.5, 0) - 1) < 1e-12  # the base's middle, straight down
    assert abs(measure(1, 0, 8, math.cos(math.pi / 4), 0) - 1) < 1e-12  # the right corner
    assert abs(measure(0.5, -0.5, 8, math.cos(math.pi / 4), 0) - 1) < 1e-12  # a side
    assert abs(measure(3, 4, 0, 1, 0) - 5) < 1e-12
    # Below 6, and the inner radius under Completion, change nothing.
    assert c["FX-IRIS-037"]["0"] == old["FX-IRIS-003"]["frames"]["0"]
    assert c["FX-IRIS-038"]["0"] == c["FX-IRIS-028"]["0"]
    assert c["FX-IRIS-028"]["0"] != old["FX-IRIS-003"]["frames"]["0"]
    # The triangle stands on its base: just above the middle is kept further out sideways
    # below than above.
    tri = c["FX-IRIS-028"]["0"]
    assert tri[at(8, 1)] != I.EMPTY or tri[at(8, 2)] != I.EMPTY  # up the point
    # The hole: the middle gone, the corners kept; inverted it adds back to the drawing.
    hole, kept = c["FX-IRIS-030"]["0"], c["FX-IRIS-034"]["0"]
    assert hole[at(8, 5)] == I.EMPTY and hole[at(1, 2)] == drawn[at(1, 2)]
    assert all(abs(a + b - d) < 1e-12 for p, q_, e in zip(hole, kept, drawn)
               for a, b, d in zip(p, q_, e))
    # Turned 45, the diamond is a level square: (5, 2), 2.5 and 2.5 from the middle, is
    # outside the diamond (5 > 4.3) and inside the square (2.5 * sqrt 2 < 4.3).
    assert hole[at(5, 2)] == drawn[at(5, 2)] and c["FX-IRIS-032"]["0"][at(5, 2)] == I.EMPTY
    k36 = c["FX-IRIS-036"]
    assert k36["0"] == drawn and k36["2"] == hole and k36["4"] not in (drawn, hole)
    assert c["FX-IRIS-031"]["0"] not in (hole, drawn)
    assert c["FX-IRIS-033"]["0"] != hole
    nine = c["FX-IRIS-039"]["0"]
    eight = c["FX-IRIS-029"]["0"]
    assert all(nine[at(x, y)] == eight[at(x - 3, y)] for x in range(3, W) for y in range(H))
    # The cliff: no hard edge passes within 1e-5 of a pixel centre.
    for fx, (_, cs, frames) in CASES.items():
        for f in frames:
            s = settings(cs, f)
            if s["feather"] != 0:
                continue
            n, q = shape_of(s)
            cx, cy = s["center"][0] / 100 * W, s["center"][1] / 100 * H
            if s["outer_radius"] > 0:
                r = s["outer_radius"]
            elif 0 < s["completion"] < 100:
                big = max(measure(a - cx, b - cy, n, q, s["rotation"])
                          for a in (0, W) for b in (0, H))
                cc = s["completion"] / 100
                r = ((1 - cc) if s["invert"] == "off" else cc) * big
            else:
                continue
            for y in range(H):
                for x in range(W):
                    g = measure(x + 0.5 - cx, y + 0.5 - cy, n, q, s["rotation"])
                    assert abs(g - r) >= CLIFF, (fx, f, x, y, g, r)
    print("checked")


if __name__ == "__main__":
    main()
