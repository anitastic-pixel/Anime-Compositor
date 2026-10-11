"""Teeth Wipe, worked a second way.

D-511 adds Teeth Wipe (`core.teeth_wipe`), after the purpose of CycoreFX's CC Jaws (Transition,
EFFECTS.md row 274), under a name of our own. The CycoreFX HD 1.8.1 manual says the effect
"splits the source layer into two parts, with each part having a tooth-shaped border along the
splitting edge", and names the controls: Completion, Center ("the point of origin for the wipe
... also ... the pivot point for the angle (rotation) set in the Direction control"), Direction
("the angle (rotation) of the break/cut between the two parts"), Height and Width ("the height
and width of the teeth") and Shape (Spikes, triangular teeth; Robo Jaw, trapezoidal teeth; Block,
rectangular teeth). It publishes no formula and no settings when added, so the rule and those
settings are ours (document 21, D-511), and so are the words for the shapes.

The settings. `completion`, 0 to 100, 0 when added; `center`, x then y, -1000 to 1000 per cent
of the drawing's own width and height, [50, 50] when added; `direction`, -3600 to 3600 degrees,
0 when added (the cut runs left to right; positive turns it clockwise on the screen);
`height` and `width`, the teeth's, in pixels, `height` 0 to 1000 and `width` 1 to 1000, 40 each
when added; all five keyable. `shape`, "spikes" (when added), "trapezoids" or "blocks", exact.

The rule, in the drawing's own W by H space. The centre C = (center_x W / 100, center_y H / 100),
the cut's direction d = (cos a, sin a) and its normal n = (-sin a, cos a), a the direction. At a
point P, along v = (P - C) . d and across w = (P - C) . n. The teeth: with the tooth width T,
t = |2 frac(v / T) - 1|, 1 at a tooth's middle and 0 half way between; the shape's profile
s(t): spikes t; trapezoids clamp(2 t - 1/2, 0, 1); blocks 1 when t > 1/2, else 0. The cut
g(v) = height (s(t) - 1/2). The part before the cut is where w < g(v), the part after where
w >= g(v).

The two parts move apart across the cut: at c = completion / 100 each moves o = c R, R =
height / 2 + the largest |w| of the drawing's four corners, so at 100 both have left the drawing.
The output at the pixel centre P (x + 1/2, y + 1/2), with w the across of P and g = g(v) of P:
where w + o < g, document 21's bilinear read of the input at P + o n (the first part, moved back
by -o n); else where w - o >= g, the read at P - o n (the second part, moved by +o n); else
transparent. At completion 0 the output is the input.

The layer never grows. A draft divides `height` and `width` (distances).

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values. Every case is checked that no pixel centre lies on the moved
cut, nor on a block's step.

The drawing is Broadcast Colors' (tools/broadcast_safe_reference.py), 16 by 10, with transparent
columns and a half-covered one. The projects go into `Fixtures/teeth_wipe`, the expected frames
into `Fixtures/teeth_wipe/expected_teeth_wipe.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/teeth_wipe_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "teeth_wipe"
NUMBERS = ("completion", "center", "direction", "height", "width")
WORDS = ("shape",)
CLEAR = [0.0, 0.0, 0.0, 0.0]


# --- the rule -------------------------------------------------------------------------------

def frame_of(k):
    cx, cy = k["center"][0] / 100 * W, k["center"][1] / 100 * H
    r = math.radians(k["direction"])
    return cx, cy, math.cos(r), math.sin(r)


def along_across(k, px, py):
    cx, cy, cs, sn = frame_of(k)
    dx, dy = px - cx, py - cy
    return dx * cs + dy * sn, -dx * sn + dy * cs


def phase(k, v):
    f = v / k["width"]
    return f - math.floor(f)


def profile(shape, t):
    if shape == "spikes":
        return t
    if shape == "trapezoids":
        return min(max(2 * t - 0.5, 0.0), 1.0)
    return 1.0 if t > 0.5 else 0.0


def cut(k, v):
    t = abs(2 * phase(k, v) - 1)
    return k["height"] * (profile(k["shape"], t) - 0.5)


def opened(k):
    far = max(abs(along_across(k, x, y)[1]) for x in (0, W) for y in (0, H))
    return k["completion"] / 100 * (k["height"] / 2 + far)


def teeth_wipe(px, k):
    o = opened(k)
    _, _, cs, sn = frame_of(k)
    nx, ny = -sn, cs
    src = L.pic(W, H, px)
    out = []
    for n in range(W * H):
        x, y = n % W + 0.5, n // W + 0.5
        v, w = along_across(k, x, y)
        g = cut(k, v)
        if w + o < g:
            out.append(L.bilinear(src, x + o * nx, y + o * ny))
        elif w - o >= g:
            out.append(L.bilinear(src, x - o * nx, y - o * ny))
        else:
            out.append(list(CLEAR))
    return out


def working():
    return [R.working(p) for p in B.pixels()]


# --- the cases ------------------------------------------------------------------------------

def case(shape="spikes", shift=0, **numbers):
    c = {"shape": shape, "shift": shift, "completion": 0, "center": [50, 50], "direction": 0,
         "height": 40, "width": 40}
    for n, v in numbers.items():
        assert n in NUMBERS, n
        c[n] = v
    return c


def settings(c, frame_no):
    k = {n: value_at(c[n], frame_no) for n in NUMBERS}
    k["shape"] = c["shape"]
    return k


def render(c, frame_no):
    k = settings(c, frame_no)
    if k["completion"] == 0:
        return R.frame(working(), c["shift"])
    return R.frame(teeth_wipe(working(), k), c["shift"])


def plain(c):
    return R.frame(working(), c["shift"])


def file_json(fx, c):
    params = {w: c[w] for w in WORDS}
    params.update({n: setting_json(c[n]) for n in NUMBERS})
    return B.project_json(fx, "core.teeth_wipe", params, c["shift"])


T = dict(height=3, width=4)
CASES = {
    "FX-TEETH-001": ("The settings as they start: completion 0, so nothing changes.", case(), [0]),
    "FX-TEETH-002": ("Completion 100: both parts gone from the drawing, every pixel clear.",
                     case(completion=100, **T), [0]),
    "FX-TEETH-003": ("Completion 20, spikes 3 high and 4 wide: the drawing parted across its "
                     "middle along a zigzag, the top half moved up, the bottom half down.",
                     case(completion=20, **T), [0]),
    "FX-TEETH-004": ("Trapezoids 3 high and 7 wide, completion 20: flat-topped teeth.",
                     case("trapezoids", completion=20, height=3, width=7), [0]),
    "FX-TEETH-005": ("FX-TEETH-003 with blocks: square teeth.",
                     case("blocks", completion=20, **T), [0]),
    "FX-TEETH-006": ("FX-TEETH-003 with direction 90: the cut runs top to bottom, the halves "
                     "move left and right.", case(completion=20, direction=90, **T), [0]),
    "FX-TEETH-007": ("FX-TEETH-003 with direction 30: the cut turned about the centre.",
                     case(completion=20, direction=30, **T), [0]),
    "FX-TEETH-008": ("Center [25, 30], completion 20: the cut moved up and to the left, the "
                     "teeth counted from the new centre.",
                     case(completion=20, center=[25, 30], **T), [0]),
    "FX-TEETH-009": ("Height 0, completion 20: a straight cut, no teeth.",
                     case(completion=20, height=0, width=4), [0]),
    "FX-TEETH-010": ("Completion 7.3: a move of part of a pixel, each part read between pixels.",
                     case(completion=7.3, **T), [0]),
    "FX-TEETH-011": ("Spikes 5 high and 2.5 wide, completion 15: tall narrow teeth.",
                     case(completion=15, height=5, width=2.5), [0]),
    "FX-TEETH-012": ("Completion keyed from 0 at frame 0 to 97 at frame 4, linear: frame 2 is "
                     "completion 48.5; at frame 4 every pixel is in the gap.", case(completion=keyed((0, 0), (4, 97)), **T), [0, 2, 4]),
    "FX-TEETH-013": ("Direction keyed from 0 at frame 0 to 60 at frame 4, linear, completion 20: "
                     "frame 2 is direction 30.",
                     case(completion=20, direction=keyed((0, 0), (4, 60)), **T), [0, 2, 4]),
    "FX-TEETH-014": ("Height keyed from 0 at frame 0 to 6 at frame 4, linear, completion 20, "
                     "blocks.", case("blocks", completion=20, height=keyed((0, 0), (4, 6)),
                                     width=4), [0, 4]),
    "FX-TEETH-015": ("FX-TEETH-003 moved three pixels right: the same, moved.",
                     case(completion=20, shift=3, **T), [0, 3]),
    "FX-TEETH-016": ("Direction -135, trapezoids 2 high and 3 wide, completion 35, Center "
                     "[60, 40].", case("trapezoids", completion=35, direction=-135,
                                       center=[60, 40], height=2, width=3), [0]),
}

INVALID = {
    "FX-TEETH-017": ("Completion 101, above 100.", case(completion=101)),
    "FX-TEETH-018": ("Completion -1, below 0.", case(completion=-1)),
    "FX-TEETH-019": ("Center y -1001, below -1000.", case(center=[50, -1001])),
    "FX-TEETH-020": ("Direction -3601, below -3600.", case(direction=-3601)),
    "FX-TEETH-021": ("Height -1, below 0.", case(height=-1)),
    "FX-TEETH-022": ("Height 1001, above 1000.", case(height=1001)),
    "FX-TEETH-023": ("Width 0.5, below 1.", case(width=0.5)),
    "FX-TEETH-024": ("Width 1001, above 1000.", case(width=1001)),
    "FX-TEETH-025": ("Shape \"Spikes\": the word is exact, so a capital is not it.",
                     case("Spikes")),
    "FX-TEETH-026": ("Shape \"robo_jaw\": not a shape of Teeth Wipe.", case("robo_jaw")),
    "FX-TEETH-027": ("Width keyed to 2000 at frame 4: a key outside the range.",
                     case(width=keyed((0, 40), (4, 2000)))),
}


def main():
    clear()
    expected = B.write_cases(OUT, "teeth_wipe", CASES, INVALID, render, plain, file_json)
    check(expected)


def clear():
    """No pixel centre on the moved cut, nor on a block's step."""
    for fx, (_, c, frames) in CASES.items():
        for f in frames:
            k = settings(c, f)
            if k["completion"] == 0:
                continue
            o = opened(k)
            for y in range(H):
                for x in range(W):
                    v, w = along_across(k, x + 0.5, y + 0.5)
                    g = cut(k, v)
                    assert abs(w + o - g) > 1e-4 and abs(w - o - g) > 1e-4, (fx, f, x, y)
                    if k["shape"] == "blocks" and k["height"] > 0:
                        q = 4 * phase(k, v)
                        assert abs(q - round(q)) > 1e-4, (fx, f, x, y)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    px = working()
    clear_px = [list(CLEAR)] * (W * H)
    assert c["FX-TEETH-001"]["0"] == R.frame(px, 0)
    assert c["FX-TEETH-002"]["0"] == R.frame(clear_px, 0)
    assert len({repr(c[f]["0"]) for f in ("FX-TEETH-003", "FX-TEETH-004", "FX-TEETH-005",
                                          "FX-TEETH-006", "FX-TEETH-007", "FX-TEETH-008",
                                          "FX-TEETH-009")}) == 7
    three = c["FX-TEETH-003"]["0"]
    # Some covered pixels gone in the gap; some read between pixels (neither as before nor clear).
    assert any(p == CLEAR and q[3] > 0 for p, q in zip(three, px))
    assert any(p != CLEAR and p != q for p, q in zip(three, px))
    assert c["FX-TEETH-012"]["2"] == render(case(completion=48.5, **T), 0)
    assert c["FX-TEETH-012"]["4"] == R.frame(clear_px, 0)
    assert c["FX-TEETH-013"]["2"] == render(case(completion=20, direction=30, **T), 0)
    moved, still = c["FX-TEETH-015"]["0"], c["FX-TEETH-003"]["0"]
    for y in range(H):
        assert moved[y * W + 3:(y + 1) * W] == still[y * W:y * W + W - 3]
    print("checked")


if __name__ == "__main__":
    main()
