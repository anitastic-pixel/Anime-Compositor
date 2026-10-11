"""Stretch Wipe, worked a second way.

D-516 adds Stretch Wipe (`core.stretch_wipe`), after the purpose of CycoreFX's CC Scale Wipe
(Transition, EFFECTS.md row 278), under a name of our own. The CycoreFX HD 1.8.1 manual says the
effect "stretches a layer off the screen (or onto it) by moving the scale center", and names the
controls: Scale ("the scale factor"), Center ("the point of origin for the wipe") and Direction
("the direction (angle) in which the source layer is scaled"). There is no Completion: the wipe
is made by moving the Center. It publishes no formula and no settings when added, so the rule
and those settings are ours (document 21, D-516).

The settings. `scale`, 100 to 100000 per cent, 100 when added (nothing stretched); `center`, x
then y, -1000 to 1000 per cent of the drawing's own width and height, [50, 50] when added;
`direction`, -3600 to 3600 degrees, 0 when added (the stretch toward the right; positive turns
clockwise on the screen); all three keyable.

The rule, in the drawing's own W by H space. The centre C = (center_x W / 100, center_y H / 100),
d = (cos a, sin a) and n = (-sin a, cos a), a the direction, S = scale / 100. At a point P,
u = (P - C) . d and v = (P - C) . n. Where u <= 0 (behind the line through C across d) the point
is the input's own. Where u > 0 it is document 21's bilinear read of the input at
C + (u / S) d + v n: everything past the line stretched S times along d, so at a large scale the
line's own pixels are drawn out in streaks, and moving the Center back across the layer
stretches it off the drawing. The two sides meet without a seam, since at u = 0 the read is the
point itself.

The layer never grows. Nothing is a distance, so a draft changes nothing.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

The drawing is Broadcast Colors' (tools/broadcast_safe_reference.py), 16 by 10, with transparent
columns and a half-covered one. The projects go into `Fixtures/stretch_wipe`, the expected frames
into `Fixtures/stretch_wipe/expected_stretch_wipe.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/stretch_wipe_reference.py
"""

import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import effect_layer_reference as L  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "stretch_wipe"
NUMBERS = ("scale", "center", "direction")
CLEAR = [0.0, 0.0, 0.0, 0.0]


# --- the rule -------------------------------------------------------------------------------

def stretch_wipe(px, k):
    cx, cy = k["center"][0] / 100 * W, k["center"][1] / 100 * H
    r = math.radians(k["direction"])
    cs, sn = math.cos(r), math.sin(r)
    s = k["scale"] / 100
    src = L.pic(W, H, px)
    out = []
    for n in range(W * H):
        x, y = n % W + 0.5, n // W + 0.5
        dx, dy = x - cx, y - cy
        u, v = dx * cs + dy * sn, -dx * sn + dy * cs
        if u <= 0:
            out.append(list(px[n]))
        else:
            a = u / s
            out.append(L.bilinear(src, cx + a * cs - v * sn, cy + a * sn + v * cs))
    return out


def working():
    return [R.working(p) for p in B.pixels()]


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, **numbers_):
    c = {"shift": shift, "scale": 100, "center": [50, 50], "direction": 0}
    for n, v in numbers_.items():
        assert n in NUMBERS, n
        c[n] = v
    return c


def settings(c, frame_no):
    return {n: value_at(c[n], frame_no) for n in NUMBERS}


def render(c, frame_no):
    k = settings(c, frame_no)
    if k["scale"] == 100:
        return R.frame(working(), c["shift"])
    return R.frame(stretch_wipe(working(), k), c["shift"])


def plain(c):
    return R.frame(working(), c["shift"])


def file_json(fx, c):
    params = {n: setting_json(c[n]) for n in NUMBERS}
    return B.project_json(fx, "core.stretch_wipe", params, c["shift"])


CASES = {
    "FX-STRETCH-001": ("The settings as they start: scale 100, so nothing changes.", case(), [0]),
    "FX-STRETCH-002": ("Scale 200: the right half stretched twice as wide, from the middle.",
                       case(scale=200), [0]),
    "FX-STRETCH-003": ("Scale 400, direction 90: the bottom half stretched four times down.",
                       case(scale=400, direction=90), [0]),
    "FX-STRETCH-004": ("Scale 100000: the right half a streak of the middle column.",
                       case(scale=100000), [0]),
    "FX-STRETCH-005": ("Scale 300, direction 30: the stretch turned.",
                       case(scale=300, direction=30), [0]),
    "FX-STRETCH-006": ("Center [25, 30], scale 250: stretched from a line further left.",
                       case(scale=250, center=[25, 30]), [0]),
    "FX-STRETCH-007": ("Center [-20, 50], scale 150: the line off the drawing to the left, the "
                       "whole drawing stretched, its left edge pulled in from outside it.",
                       case(scale=150, center=[-20, 50]), [0]),
    "FX-STRETCH-008": ("Direction 180, scale 300: the left half stretched to the left.",
                       case(scale=300, direction=180), [0]),
    "FX-STRETCH-009": ("Scale keyed from 100 at frame 0 to 500 at frame 4, linear: frame 2 is "
                       "scale 300.", case(scale=keyed((0, 100), (4, 500))), [0, 2, 4]),
    "FX-STRETCH-010": ("Direction keyed from 0 at frame 0 to 90 at frame 4, linear, scale 300: "
                       "frame 2 is direction 45.",
                       case(scale=300, direction=keyed((0, 0), (4, 90))), [0, 2, 4]),
    "FX-STRETCH-011": ("FX-STRETCH-002 moved three pixels right: the same, moved.",
                       case(scale=200, shift=3), [0, 3]),
    "FX-STRETCH-012": ("Direction -135, Center [60, 40], scale 175.",
                       case(scale=175, direction=-135, center=[60, 40]), [0]),
}

INVALID = {
    "FX-STRETCH-013": ("Scale 99, below 100.", case(scale=99)),
    "FX-STRETCH-014": ("Scale 100001, above 100000.", case(scale=100001)),
    "FX-STRETCH-015": ("Center y -1001, below -1000.", case(center=[50, -1001])),
    "FX-STRETCH-016": ("Direction 3601, above 3600.", case(direction=3601)),
    "FX-STRETCH-017": ("Scale keyed to 200000 at frame 4: a key outside the range.",
                       case(scale=keyed((0, 100), (4, 200000)))),
}


def main():
    expected = B.write_cases(OUT, "stretch_wipe", CASES, INVALID, render, plain, file_json)
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    px = working()
    assert c["FX-STRETCH-001"]["0"] == R.frame(px, 0)
    assert len({repr(c[f]["0"]) for f in ("FX-STRETCH-002", "FX-STRETCH-003", "FX-STRETCH-004",
                                          "FX-STRETCH-005", "FX-STRETCH-006", "FX-STRETCH-007",
                                          "FX-STRETCH-008", "FX-STRETCH-012")}) == 8
    # The left half untouched at scale 200, the right half changed.
    two = stretch_wipe(px, settings(CASES["FX-STRETCH-002"][1], 0))
    assert all(two[y * W + x] == px[y * W + x] for y in range(H) for x in range(8))
    assert any(two[y * W + x] != px[y * W + x] for y in range(H) for x in range(8, W))
    # At 100000 every row's right half is within a hair of the middle line's colour, half way
    # between columns 7 and 8.
    four = stretch_wipe(px, settings(CASES["FX-STRETCH-004"][1], 0))
    for y in range(H):
        mid = [(a + b) / 2 for a, b in zip(px[y * W + 7], px[y * W + 8])]
        for x in range(8, W):
            assert all(abs(a - b) < 1e-2 for a, b in zip(four[y * W + x], mid)), (x, y)
    assert c["FX-STRETCH-009"]["2"] == render(case(scale=300), 0)
    assert c["FX-STRETCH-010"]["2"] == render(case(scale=300, direction=45), 0)
    moved, still = c["FX-STRETCH-011"]["0"], c["FX-STRETCH-002"]["0"]
    for y in range(H):
        assert moved[y * W + 3:(y + 1) * W] == still[y * W:y * W + W - 3]
    print("checked")


if __name__ == "__main__":
    main()
