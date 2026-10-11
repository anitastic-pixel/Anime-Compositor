"""Stripe Sweep, worked a second way.

D-514 adds Stripe Sweep (`core.stripe_sweep`), after the purpose of CycoreFX's CC Line Sweep
(Transition, EFFECTS.md row 276), under a name of our own. The CycoreFX HD 1.8.1 manual says the
effect "will wipe the source layer line by line where thickness, overlap and angle can be
controlled", and names the controls: Completion, Direction ("the angle in which the source layer
is wiped", in degrees, "can not be animated"), Thickness ("the line thickness", "can not be
animated"), Slant ("the overlap of lines during the transition": at 0 "new lines are not
produced until the previous line has finished wiping", at 99 "all lines start at the same
time"), and Flip Direction (lines that moved left to right move right to left). It publishes no
formula and no settings when added, so the rule and those settings are ours (document 21,
D-514).

The settings. `completion`, 0 to 100, 0 when added; `direction`, -3600 to 3600 degrees, 0 when
added (positive turns clockwise on the screen); `thickness`, 1 to 1000 pixels, 20 when added;
`slant`, 0 to 99, 50 when added; all four keyable (CycoreFX lets neither Direction nor
Thickness be keyed; keying them here breaks nothing, so we allow it). `flip`, "off" (when added)
or "on", exact.

The rule, in the drawing's own W by H space. The middle C = (W / 2, H / 2), d = (cos a, sin a)
the way the lines sweep and n = (-sin a, cos a) across them, a the direction. At a point P,
u = (P - C) . d and v = (P - C) . n; U and V are the largest |u| and |v| of the drawing's four
corners. The lines are bands across n, T thick: line i = floor((v + V) / T), the last one
N - 1 = ceil(2 V / T) - 1 (a point on the far edge is in it). Along its line the point is at
t = (u + U) / (2 U), or 1 - t flipped. With o = slant / 99, every line takes the share
s = 1 / (1 + (N - 1)(1 - o)) of the transition and line i starts at i s (1 - o), so at slant 0
each line starts as the one before ends, at 99 all start at once, and the last one ends at
completion 100 whatever the slant. With c = completion / 100, line i has gone the share
p = clamp((c - i s (1 - o)) / s, 0, 1) of its length, and the point is gone where t < p and kept
where t >= p, a hard edge. At completion 0 the output is the input; at 100 every pixel is clear.

The layer never grows. The thickness is a distance, so a draft scales it as it scales the
drawing; the lines are the same lines.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values. Every case is checked that no pixel centre lies on an edge
between lines or on a line's moving edge.

The drawing is Broadcast Colors' (tools/broadcast_safe_reference.py), 16 by 10, with transparent
columns and a half-covered one. The projects go into `Fixtures/stripe_sweep`, the expected frames
into `Fixtures/stripe_sweep/expected_stripe_sweep.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/stripe_sweep_reference.py
"""

import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "stripe_sweep"
NUMBERS = ("completion", "direction", "thickness", "slant")
WORDS = ("flip",)
CLEAR = [0.0, 0.0, 0.0, 0.0]


# --- the rule -------------------------------------------------------------------------------

def frame_of(k):
    r = math.radians(k["direction"])
    cs, sn = math.cos(r), math.sin(r)
    corners = [(x - W / 2, y - H / 2) for x in (0, W) for y in (0, H)]
    U = max(abs(x * cs + y * sn) for x, y in corners)
    V = max(abs(-x * sn + y * cs) for x, y in corners)
    n = max(1, math.ceil(2 * V / k["thickness"]))
    o = k["slant"] / 99
    s = 1 / (1 + (n - 1) * (1 - o))
    return cs, sn, U, V, n, o, s


def place(k, px, py):
    """The point's line, its place t along it, and the line's share gone."""
    cs, sn, U, V, n, o, s = frame_of(k)
    dx, dy = px - W / 2, py - H / 2
    u, v = dx * cs + dy * sn, -dx * sn + dy * cs
    i = min(math.floor((v + V) / k["thickness"]), n - 1)
    t = (u + U) / (2 * U)
    if k["flip"] == "on":
        t = 1 - t
    p = min(max((k["completion"] / 100 - i * s * (1 - o)) / s, 0.0), 1.0)
    return (v + V) / k["thickness"], t, p


def stripe_sweep(px, k):
    out = []
    for n, p in enumerate(px):
        _, t, gone = place(k, n % W + 0.5, n // W + 0.5)
        out.append(list(p) if t >= gone else list(CLEAR))
    return out


def working():
    return [R.working(p) for p in B.pixels()]


# --- the cases ------------------------------------------------------------------------------

def case(flip="off", shift=0, **numbers_):
    c = {"flip": flip, "shift": shift, "completion": 0, "direction": 0, "thickness": 20,
         "slant": 50}
    for n, v in numbers_.items():
        assert n in NUMBERS, n
        c[n] = v
    return c


def settings(c, frame_no):
    k = {n: value_at(c[n], frame_no) for n in NUMBERS}
    k.update({w: c[w] for w in WORDS})
    return k


def render(c, frame_no):
    k = settings(c, frame_no)
    if k["completion"] == 0:
        return R.frame(working(), c["shift"])
    return R.frame(stripe_sweep(working(), k), c["shift"])


def plain(c):
    return R.frame(working(), c["shift"])


def file_json(fx, c):
    params = {w: c[w] for w in WORDS}
    params.update({n: setting_json(c[n]) for n in NUMBERS})
    return B.project_json(fx, "core.stripe_sweep", params, c["shift"])


CASES = {
    "FX-STRIPE-001": ("The settings as they start: completion 0, so nothing changes.", case(), [0]),
    "FX-STRIPE-002": ("Completion 100: every pixel gone.", case(completion=100), [0]),
    "FX-STRIPE-003": ("Thickness 3, completion 40: four lines across the drawing, each wiped from "
                      "the left, the top one furthest along.",
                      case(completion=40, thickness=3), [0]),
    "FX-STRIPE-004": ("Thickness 3, slant 0, completion 40: the top line done, the second part "
                      "way, the others not started.",
                      case(completion=40, thickness=3, slant=0), [0]),
    "FX-STRIPE-005": ("Thickness 3, slant 99, completion 40: all four lines together, a straight "
                      "edge.", case(completion=40, thickness=3, slant=99), [0]),
    "FX-STRIPE-006": ("FX-STRIPE-003 flipped: the lines wiped from the right.",
                      case("on", completion=40, thickness=3), [0]),
    "FX-STRIPE-007": ("Direction 90, thickness 4, completion 40: the lines are columns, wiped "
                      "downward.", case(completion=40, thickness=4, direction=90), [0]),
    "FX-STRIPE-008": ("Direction 30, thickness 3, completion 50: the lines turned.",
                      case(completion=50, thickness=3, direction=30), [0]),
    "FX-STRIPE-009": ("Thickness 1, completion 50: ten one-pixel lines, a staircase edge.",
                      case(completion=50, thickness=1), [0]),
    "FX-STRIPE-010": ("Thickness 20 (as added), completion 60: one line, a straight wipe.",
                      case(completion=60), [0]),
    "FX-STRIPE-011": ("Completion keyed from 0 at frame 0 to 100 at frame 4, linear, thickness 3: "
                      "frame 2 is completion 50, frame 4 all clear.",
                      case(completion=keyed((0, 0), (4, 100)), thickness=3), [0, 2, 4]),
    "FX-STRIPE-012": ("Thickness keyed from 2 at frame 0 to 6 at frame 4, linear, completion 50: "
                      "frame 2 is thickness 4.",
                      case(completion=50, thickness=keyed((0, 2), (4, 6))), [0, 2, 4]),
    "FX-STRIPE-013": ("FX-STRIPE-003 moved three pixels right: the same, moved.",
                      case(completion=40, thickness=3, shift=3), [0, 3]),
    "FX-STRIPE-014": ("Direction -135, flipped, slant 20, thickness 2, completion 65.",
                      case("on", completion=65, direction=-135, slant=20, thickness=2), [0]),
}

INVALID = {
    "FX-STRIPE-015": ("Completion 101, above 100.", case(completion=101)),
    "FX-STRIPE-016": ("Completion -1, below 0.", case(completion=-1)),
    "FX-STRIPE-017": ("Direction 3601, above 3600.", case(direction=3601)),
    "FX-STRIPE-018": ("Thickness 0, below 1.", case(thickness=0)),
    "FX-STRIPE-019": ("Thickness 1001, above 1000.", case(thickness=1001)),
    "FX-STRIPE-020": ("Slant -1, below 0.", case(slant=-1)),
    "FX-STRIPE-021": ("Slant 100, above 99.", case(slant=100)),
    "FX-STRIPE-022": ("Flip \"yes\": it is \"off\" or \"on\".", case("yes")),
    "FX-STRIPE-023": ("Flip \"On\": the word is exact, so a capital is not it.", case("On")),
    "FX-STRIPE-024": ("Slant keyed to 120 at frame 4: a key outside the range.",
                      case(slant=keyed((0, 50), (4, 120)))),
}


def main():
    clear()
    expected = B.write_cases(OUT, "stripe_sweep", CASES, INVALID, render, plain, file_json)
    check(expected)


def clear():
    """No pixel centre on an edge between lines or on a line's moving edge."""
    for fx, (_, c, frames) in CASES.items():
        for f in frames:
            k = settings(c, f)
            if k["completion"] == 0:
                continue
            for y in range(H):
                for x in range(W):
                    band, t, p = place(k, x + 0.5, y + 0.5)
                    assert abs(band - round(band)) > 1e-4, (fx, f, x, y, "between lines")
                    assert abs(t - p) > 1e-4, (fx, f, x, y, "on the edge")


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    px = working()
    clear_px = [list(CLEAR)] * (W * H)
    assert c["FX-STRIPE-001"]["0"] == R.frame(px, 0)
    assert c["FX-STRIPE-002"]["0"] == R.frame(clear_px, 0)
    assert c["FX-STRIPE-011"]["4"] == R.frame(clear_px, 0)
    assert len({repr(c[f]["0"]) for f in ("FX-STRIPE-003", "FX-STRIPE-004", "FX-STRIPE-005",
                                          "FX-STRIPE-006", "FX-STRIPE-007", "FX-STRIPE-008",
                                          "FX-STRIPE-009", "FX-STRIPE-010")}) == 8

    def row_gone(fx):
        k = settings(CASES[fx][1], 0)
        out = stripe_sweep(px, k)
        return [sum(out[y * W + x] == CLEAR and px[y * W + x] != CLEAR for x in range(W))
                for y in range(H)]

    # Slant 0: the top line (rows 0 to 2) wiped across, the bottom line (row 9) untouched.
    k4 = settings(CASES["FX-STRIPE-004"][1], 0)
    out4 = stripe_sweep(px, k4)
    assert all(out4[y * W + x] == CLEAR for y in range(3) for x in range(W))
    assert all(out4[9 * W + x] == px[9 * W + x] for x in range(W))
    # Slant 99: every row gone to the same column.
    out5 = stripe_sweep(px, settings(CASES["FX-STRIPE-005"][1], 0))
    edges = {next(x for x in range(W) if out5[y * W + x] != CLEAR) for y in range(H)}
    assert len(edges) == 1, edges
    # Slant 50: each line further along than the one below it.
    r3 = row_gone("FX-STRIPE-003")
    assert r3[0] > r3[3] > r3[6] > r3[9] - 1
    # Flipped: the gone pixels mirrored left to right.
    out3 = stripe_sweep(px, settings(CASES["FX-STRIPE-003"][1], 0))
    out6 = stripe_sweep(px, settings(CASES["FX-STRIPE-006"][1], 0))
    for y in range(H):
        for x in range(W):
            assert (out3[y * W + x] == CLEAR) == (out6[y * W + W - 1 - x] == CLEAR) or \
                px[y * W + x] == CLEAR or px[y * W + W - 1 - x] == CLEAR
    assert c["FX-STRIPE-011"]["2"] == render(case(completion=50, thickness=3), 0)
    assert c["FX-STRIPE-012"]["2"] == render(case(completion=50, thickness=4), 0)
    moved, still = c["FX-STRIPE-013"]["0"], c["FX-STRIPE-003"]["0"]
    for y in range(H):
        assert moved[y * W + 3:(y + 1) * W] == still[y * W:y * W + W - 3]
    print("checked")


if __name__ == "__main__":
    main()
