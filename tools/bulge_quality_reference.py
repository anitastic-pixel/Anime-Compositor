"""Bulge's Antialiasing and Pin All Edges, worked a second way.

D-935 adds two of After Effects' Bulge controls to `core.bulge` (D-152, D-310):
`antialiasing`, `low` or `high`, and `pin_all_edges`, `off` or `on`. Adobe's page says only that
Antialiasing sets how smooth the bulge is, at Best quality only, and that Pin All Edges keeps the
layer's edges from moving; it gives no formula, so both rules are ours. Document 21 is the rule in
words; this file is the reference for the numbers document 25 pins against it.

The rule. Q(P) is D-152's read point for a point P, None where P is out of reach (d >= radius,
or height or radius 0). With `pin_all_edges` `on`, the drawing w by h, e = max(0, min(P.x, w -
P.x, P.y, h - P.y) - 1/2), so 0 at the centres of the border pixels, B = min(w, h) / 4,
s = min(1, e / B) and k = s^2 (3 - 2 s), worked at the pixel's centre; the read point becomes
P + k (Q(P) - P), and is None where k is 0. `off` is k = 1. `low` (and a file without the word): the pixel is kept exactly
where its centre's read point is None, else document 21's bilinear sample there, D-152 exactly.
`high`: the four points P + (+-1/4, +-1/4) are each worked the same way, with the pixel's k, one whose read point is
None giving the pixel itself, and the pixel is the mean of the four. A pixel whose four points
are all out of reach is kept exactly.

**This file never runs the build's code path.** It works in double precision on lists.

    python tools/bulge_quality_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import bulge_reference as B  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402

W, H = B.W, B.H
OUT = B.OUT
SUBS = ((-0.25, -0.25), (0.25, -0.25), (-0.25, 0.25), (0.25, 0.25))


def pin(px, py, w=W, h=H):
    e = max(0.0, min(px, w - px, py, h - py) - 0.5)
    s = min(1.0, e / (min(w, h) / 4))
    return s * s * (3 - 2 * s)


def read_point(s, px, py, k):
    m = B.scale(s["center"], s["radius"], s["height"], px, py)
    if m is None or k == 0:
        return None
    cx, cy = B.centre(s["center"])
    qx, qy = cx + m * (px - cx), cy + m * (py - cy)
    return px + k * (qx - px), py + k * (qy - py)


def pixel(layer, s, x, y, aa, pinned):
    if not (0 <= x < layer["w"] and 0 <= y < layer["h"]):
        return B.EMPTY
    own = list(layer["px"][y * layer["w"] + x])
    k = pin(x + 0.5, y + 0.5) if pinned else 1.0
    if aa == "low":
        q = read_point(s, x + 0.5, y + 0.5, k)
        return own if q is None else bilinear(layer, *q)
    reads = [read_point(s, x + 0.5 + dx, y + 0.5 + dy, k) for dx, dy in SUBS]
    if all(q is None for q in reads):
        return own
    got = [own if q is None else bilinear(layer, *q) for q in reads]
    return [sum(g[i] for g in got) / 4 for i in range(4)]


def case(aa="low", pinned="off", **kw):
    c = B.case(**kw)
    c["antialiasing"], c["pin_all_edges"] = aa, pinned
    return c


def render(c, frame_no=0):
    layer = B.drawn_layer(c["drawing"])
    s = B.settings(c, frame_no)
    return [pixel(layer, s, x - c["shift"], y, c["antialiasing"], c["pin_all_edges"] == "on")
            for y in range(H) for x in range(W)]


SMALL = B.SMALL
CASES = {
    "FX-BULGE-024": ("Antialiasing High on FX-BULGE-004 (radius 6, height 1): each pixel the mean "
                     "of four reads a quarter pixel either way, so the swollen stripes' edges "
                     "come out softer; every pixel whose four points are all past the circle, "
                     "the drawing's two ends among them, is kept exactly.",
                     case("high", **SMALL)),
    "FX-BULGE-025": ("Antialiasing High on FX-BULGE-007 (radius 6, height -4), the hard pinch "
                     "where one read per pixel jumps furthest: the four reads smooth it.",
                     case("high", height=-4, **SMALL)),
    "FX-BULGE-026": ("Antialiasing written Low: FX-BULGE-004 exactly.",
                     case("low", **SMALL)),
    "FX-BULGE-027": ("Antialiasing High at height 0: the drawing, untouched.",
                     case("high", height=0)),
    "FX-BULGE-028": ("Pin All Edges on FX-BULGE-001 (radius 50, height 1, every pixel in "
                     "reach): the border pixels, rows 0 and 9 and columns 0 and 15, are kept "
                     "exactly, and the pull fades in over the 2.5 pixels inside them, so the "
                     "empty top and bottom rows and the right-hand column stay empty.",
                     case(pinned="on")),
    "FX-BULGE-029": ("Pin All Edges on FX-BULGE-004 (radius 6): the circle reaches the top and "
                     "bottom rows, which are kept; rows 1 and 8 swell less; columns 0, 1, 14 "
                     "and 15, out of reach, are kept as before.",
                     case(pinned="on", **SMALL)),
    "FX-BULGE-030": ("Pin All Edges and Antialiasing High together on FX-BULGE-001: the border "
                     "kept, the inside swollen and smoothed.",
                     case("high", "on")),
    "FX-BULGE-031": ("FX-BULGE-028 moved three pixels right: the same, moved; the pinned edges "
                     "are the drawing's own, and the three columns left of it stay empty.",
                     case(pinned="on", shift=3)),
    "FX-BULGE-032": ("Pin All Edges written off: FX-BULGE-001 exactly.",
                     case(pinned="off")),
}

INVALID = {
    "FX-BULGE-033": ("Antialiasing written \"medium\".", case("medium")),
    "FX-BULGE-034": ("Pin All Edges written \"yes\".", case(pinned="yes")),
}


def project_json(fx, c):
    p = B.project_json(fx, c)
    params = p["compositions"][0]["layers"][0]["effects"][0]["parameters"]
    params["antialiasing"] = c["antialiasing"]
    params["pin_all_edges"] = c["pin_all_edges"]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    expected = {"tolerance": B.TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c) in CASES.items():
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": render(c)}}
        print(fx)
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = B.plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(fx, "invalid")
    (OUT / "expected_bulge_quality.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def check(expected):
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    drawn = B.plain(B.case())
    old = {n: B.render(B.case(**kw), 0) for n, kw in
           (("001", {}), ("004", SMALL), ("007", dict(height=-4, **SMALL)))}
    # Low, and Pin off, are D-152 exactly.
    assert c["FX-BULGE-026"] == old["004"] and c["FX-BULGE-032"] == old["001"]
    assert c["FX-BULGE-027"] == drawn
    assert pin(0.5, 5) == 0 and pin(8, 5) == 1 and 0 < pin(2, 5) < 1
    # High differs from Low inside, and keeps the ends exactly.
    for fx, low in (("FX-BULGE-024", "004"), ("FX-BULGE-025", "007")):
        assert c[fx] != old[low]
        assert all(c[fx][at(x, y)] == drawn[at(x, y)] for x in (0, 15) for y in range(H))
    # Pinned: the border exact, the middle as before, the rows between in between.
    for fx in ("FX-BULGE-028", "FX-BULGE-030"):
        px = c[fx]
        assert all(px[at(x, y)] == drawn[at(x, y)] for x in range(W) for y in (0, H - 1))
        assert all(px[at(x, y)] == drawn[at(x, y)] for x in (0, W - 1) for y in range(H))
    assert c["FX-BULGE-028"][at(8, 4)] == old["001"][at(8, 4)]
    assert c["FX-BULGE-028"][at(8, 1)] not in (old["001"][at(8, 1)], drawn[at(8, 1)])
    assert c["FX-BULGE-029"] != old["004"]
    assert all(c["FX-BULGE-029"][at(8, y)] == drawn[at(8, y)] for y in (0, H - 1))
    assert old["004"][at(8, 0)] != drawn[at(8, 0)]  # unpinned, the top row moved
    assert c["FX-BULGE-030"] not in (c["FX-BULGE-028"], old["001"])
    shifted = c["FX-BULGE-031"]
    assert all(shifted[at(x, y)] == c["FX-BULGE-028"][at(x - 3, y)]
               for x in range(3, W) for y in range(H))
    for fx, frames in expected["cases"].items():
        for f in frames["frames"].values():
            for p in f:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
    print("checked")


if __name__ == "__main__":
    main()
