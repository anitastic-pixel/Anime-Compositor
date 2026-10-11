"""Push Iris, worked a second way.

D-515 adds Push Iris (`core.push_iris`), after the purpose of CycoreFX's CC Radial ScaleWipe
(Transition, EFFECTS.md row 277), under a name of our own. The CycoreFX HD 1.8.1 manual says
only that it is a "radial wipe where edge folds", and names the controls: Completion, Center
("the point of origin for the wipe") and Reverse Transition. It publishes no formula and no
settings when added, so the rule and those settings are ours (document 21, D-515).

The settings. `completion`, 0 to 100, 0 when added; `center`, x then y, -1000 to 1000 per cent
of the drawing's own width and height, [50, 50] when added; both keyable. `reverse`, "off" (when
added) or "on", exact.

The rule, in the drawing's own W by H space. The centre C = (center_x W / 100, center_y H / 100),
D the largest distance of the drawing's four corners from C, c = completion / 100 and R = c D.
At a point P, r = |P - C|. Forward: a hole of radius R opens at C; where r < R the point is
clear, and elsewhere it is document 21's bilinear read of the input at C + (P - C)(r - R) / r,
the picture pushed out from the centre, so the middle of the picture is stretched round the
hole's edge (the edge magnifies it) and the outside is pushed off the drawing. Reversed: the
picture is pulled in instead; where r >= (1 - c) D the point is clear, and elsewhere it is the
read at C + (P - C)(r + R) / r (at r = 0, at C + (R, 0)), so the outside closes in like an iris
and the middle of the picture is folded into the centre. At completion 0 the output is the
input either way; at 100 every pixel is clear.

The layer never grows. Nothing is a distance, so a draft changes nothing.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values. Every case is checked that no pixel centre lies on the hole's
or the iris's edge.

The drawing is Broadcast Colors' (tools/broadcast_safe_reference.py), 16 by 10, with transparent
columns and a half-covered one. The projects go into `Fixtures/push_iris`, the expected frames
into `Fixtures/push_iris/expected_push_iris.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/push_iris_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "push_iris"
NUMBERS = ("completion", "center")
WORDS = ("reverse",)
CLEAR = [0.0, 0.0, 0.0, 0.0]


# --- the rule -------------------------------------------------------------------------------

def numbers(k):
    """The centre, the corner's distance, the hole's radius and the edge kept within."""
    cx, cy = k["center"][0] / 100 * W, k["center"][1] / 100 * H
    far = max(math.hypot(x - cx, y - cy) for x in (0, W) for y in (0, H))
    big = k["completion"] / 100 * far
    return cx, cy, far, big


def edge(k):
    cx, cy, far, big = numbers(k)
    return far - big if k["reverse"] == "on" else big


def push_iris(px, k):
    cx, cy, far, big = numbers(k)
    src = L.pic(W, H, px)
    out = []
    for n in range(W * H):
        x, y = n % W + 0.5, n // W + 0.5
        dx, dy = x - cx, y - cy
        r = math.sqrt(dx * dx + dy * dy)
        if k["reverse"] == "on":
            if r >= far - big:
                out.append(list(CLEAR))
            elif r == 0:
                out.append(L.bilinear(src, cx + big, cy))
            else:
                f = (r + big) / r
                out.append(L.bilinear(src, cx + dx * f, cy + dy * f))
        elif r < big:
            out.append(list(CLEAR))
        else:
            f = (r - big) / r
            out.append(L.bilinear(src, cx + dx * f, cy + dy * f))
    return out


def working():
    return [R.working(p) for p in B.pixels()]


# --- the cases ------------------------------------------------------------------------------

def case(reverse="off", shift=0, **numbers_):
    c = {"reverse": reverse, "shift": shift, "completion": 0, "center": [50, 50]}
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
    return R.frame(push_iris(working(), k), c["shift"])


def plain(c):
    return R.frame(working(), c["shift"])


def file_json(fx, c):
    params = {w: c[w] for w in WORDS}
    params.update({n: setting_json(c[n]) for n in NUMBERS})
    return B.project_json(fx, "core.push_iris", params, c["shift"])


CASES = {
    "FX-PUSH-001": ("The settings as they start: completion 0, so nothing changes.", case(), [0]),
    "FX-PUSH-002": ("Completion 100: every pixel clear.", case(completion=100), [0]),
    "FX-PUSH-003": ("Completion 30: a hole in the middle, the picture pushed out round it, its "
                    "middle stretched round the hole's edge.", case(completion=30), [0]),
    "FX-PUSH-004": ("Completion 60: a bigger hole, more of the picture pushed off.",
                    case(completion=60), [0]),
    "FX-PUSH-005": ("Reversed, completion 30: the outside closed in, the picture pulled toward "
                    "the middle.", case("on", completion=30), [0]),
    "FX-PUSH-006": ("Center [25, 30], completion 30: the hole moved up and to the left.",
                    case(completion=30, center=[25, 30]), [0]),
    "FX-PUSH-007": ("Completion 7.3: a small hole, every pixel read between pixels.",
                    case(completion=7.3), [0]),
    "FX-PUSH-008": ("Center [120, 50], off the drawing to the right, completion 40: the hole's "
                    "edge sweeps in from the right.", case(completion=40, center=[120, 50]), [0]),
    "FX-PUSH-009": ("Completion keyed from 0 at frame 0 to 100 at frame 4, linear: frame 2 is "
                    "completion 50, frame 4 all clear.",
                    case(completion=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-PUSH-010": ("FX-PUSH-003 moved three pixels right: the same, moved.",
                    case(completion=30, shift=3), [0, 3]),
    "FX-PUSH-011": ("Reversed, completion 65, Center [60, 40].",
                    case("on", completion=65, center=[60, 40]), [0]),
}

INVALID = {
    "FX-PUSH-012": ("Completion 101, above 100.", case(completion=101)),
    "FX-PUSH-013": ("Completion -1, below 0.", case(completion=-1)),
    "FX-PUSH-014": ("Center x 1001, above 1000.", case(center=[1001, 50])),
    "FX-PUSH-015": ("Reverse \"yes\": it is \"off\" or \"on\".", case("yes")),
    "FX-PUSH-016": ("Reverse \"On\": the word is exact, so a capital is not it.", case("On")),
    "FX-PUSH-017": ("Completion keyed to 150 at frame 4: a key outside the range.",
                    case(completion=keyed((0, 0), (4, 150)))),
}


def main():
    clear()
    expected = B.write_cases(OUT, "push_iris", CASES, INVALID, render, plain, file_json)
    check(expected)


def clear():
    """No pixel centre on the hole's or the iris's edge, nor on the centre."""
    for fx, (_, c, frames) in CASES.items():
        for f in frames:
            k = settings(c, f)
            if k["completion"] == 0:
                continue
            cx, cy, _, _ = numbers(k)
            e = edge(k)
            for y in range(H):
                for x in range(W):
                    r = math.hypot(x + 0.5 - cx, y + 0.5 - cy)
                    assert abs(r - e) > 1e-4 and r > 1e-4, (fx, f, x, y)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    px = working()
    clear_px = [list(CLEAR)] * (W * H)
    assert c["FX-PUSH-001"]["0"] == R.frame(px, 0)
    assert c["FX-PUSH-002"]["0"] == R.frame(clear_px, 0)
    assert c["FX-PUSH-009"]["4"] == R.frame(clear_px, 0)
    assert len({repr(c[f]["0"]) for f in ("FX-PUSH-003", "FX-PUSH-004", "FX-PUSH-005",
                                          "FX-PUSH-006", "FX-PUSH-007", "FX-PUSH-008",
                                          "FX-PUSH-011")}) == 7
    # Forward: the middle clear, a corner still covered.
    three = push_iris(px, settings(CASES["FX-PUSH-003"][1], 0))
    assert three[4 * W + 7] == CLEAR and three[4 * W + 8] == CLEAR
    assert three[0 * W + 15] != CLEAR
    # Reversed: the corners clear, the middle covered.
    five = push_iris(px, settings(CASES["FX-PUSH-005"][1], 0))
    assert five[0] == CLEAR and five[9 * W + 15] == CLEAR and five[4 * W + 8] != CLEAR
    assert c["FX-PUSH-009"]["2"] == render(case(completion=50), 0)
    moved, still = c["FX-PUSH-010"]["0"], c["FX-PUSH-003"]["0"]
    for y in range(H):
        assert moved[y * W + 3:(y + 1) * W] == still[y * W:y * W + W - 3]
    print("checked")


if __name__ == "__main__":
    main()
