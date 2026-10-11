"""Glow Wipe, worked a second way.

D-513 adds Glow Wipe (`core.glow_wipe`), after the purpose of CycoreFX's CC Light Wipe
(Transition, EFFECTS.md row 275), under a name of our own. The CycoreFX HD 1.8.1 manual says the
effect "uses a light source to wipe away the source layer", the light "either a solid color or
... sampled from the source layer as it is being wiped", and names the controls: Completion,
Center ("the point of origin for the wipe"), Intensity ("the power (or strength) of the light"),
Shape (Doors, "splits the source layer in two parts with a wall of light wiping each part away";
Radial, "a circle of light"; Rectangular, "a rectangle of light"), Direction ("the direction, at
which light rays strikes the source layer", not used by the round shape), Color from Source (off
when added), Color, and Reverse Transition. It publishes no formula and no settings when added
beyond Color from Source off, so the rule and those settings are ours (document 21, D-513).

The settings. `completion`, 0 to 100, 0 when added; `center`, x then y, -1000 to 1000 per cent
of the drawing's own width and height, [50, 50] when added; `intensity`, 0 to 1000 per cent, 100
when added; `direction`, -3600 to 3600 degrees, 0 when added (positive turns clockwise on the
screen); all four keyable. `shape`, "doors", "radial" (when added) or "rectangular";
`color_from_source`, "off" (when added) or "on"; `color`, `#rrggbb`, "#ffffff" when added;
`reverse`, "off" (when added) or "on"; all exact.

The rule, in the drawing's own W by H space. The centre C = (center_x W / 100, center_y H / 100),
d = (cos a, sin a) and n = (-sin a, cos a), a the direction. At a point P, u = (P - C) . d and
v = (P - C) . n, and the shape's distance: doors |u| (two walls moving apart along d); radial
sqrt(u^2 + v^2) (the direction unused); rectangular max(|u|, |v|). D is the largest distance of
the drawing's four corners, the band b = D / 10. With c = completion / 100, the front
f = c (D + 2 b) - b, and s = the distance, or D - the distance reversed (the wipe from the outside
in). The layer is kept where s >= f and gone where s < f, a hard edge. The light's share
l = max(0, 1 - |s - f| / b), g = l intensity / 100: with Color from Source off, the light
L = K g, K the colour's linear value, its covering h = min(g, 1); on, L = g times the pixel's own
premultiplied colour and h = min(g a, 1), so the light is the layer's own colour and only where
the layer was. On the kept (or cleared) pixel q: q + L in colour and q_a + (1 - q_a) h in
covering. At completion 0 the output is the input; at 100 every pixel is clear and unlit.

The layer never grows. Nothing is a distance, so a draft changes nothing.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values. Every case is checked that no pixel centre lies on the edge.

The drawing is Broadcast Colors' (tools/broadcast_safe_reference.py), 16 by 10, with transparent
columns and a half-covered one. The projects go into `Fixtures/glow_wipe`, the expected frames
into `Fixtures/glow_wipe/expected_glow_wipe.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/glow_wipe_reference.py
"""

import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "glow_wipe"
NUMBERS = ("completion", "center", "intensity", "direction")
WORDS = ("shape", "color_from_source", "color", "reverse")
CLEAR = [0.0, 0.0, 0.0, 0.0]


# --- the rule -------------------------------------------------------------------------------

def distance(k, px, py):
    cx, cy = k["center"][0] / 100 * W, k["center"][1] / 100 * H
    r = math.radians(k["direction"])
    cs, sn = math.cos(r), math.sin(r)
    dx, dy = px - cx, py - cy
    u, v = dx * cs + dy * sn, -dx * sn + dy * cs
    if k["shape"] == "doors":
        return abs(u)
    if k["shape"] == "radial":
        return math.sqrt(u * u + v * v)
    return max(abs(u), abs(v))


def numbers(k):
    far = max(distance(k, x, y) for x in (0, W) for y in (0, H))
    b = far / 10
    return far, b, k["completion"] / 100 * (far + 2 * b) - b


def place(k, far, x, y):
    d = distance(k, x, y)
    return far - d if k["reverse"] == "on" else d


def colour(hexa):
    return [S.srgb_to_linear(int(hexa[i:i + 2], 16) / 255) for i in (1, 3, 5)]


def glow_wipe(px, k):
    far, b, f = numbers(k)
    K, i = colour(k["color"]), k["intensity"] / 100
    out = []
    for n, p in enumerate(px):
        s = place(k, far, n % W + 0.5, n // W + 0.5)
        q = list(p) if s >= f else list(CLEAR)
        g = max(0.0, 1 - abs(s - f) / b) * i
        if k["color_from_source"] == "on":
            light, h = [c * g for c in p[:3]], min(g * p[3], 1.0)
        else:
            light, h = [c * g for c in K], min(g, 1.0)
        out.append([q[j] + light[j] for j in range(3)] + [q[3] + (1 - q[3]) * h])
    return out


def working():
    return [R.working(p) for p in B.pixels()]


# --- the cases ------------------------------------------------------------------------------

def case(shape="radial", color_from_source="off", color="#ffffff", reverse="off", shift=0,
         **numbers_):
    c = {"shape": shape, "color_from_source": color_from_source, "color": color,
         "reverse": reverse, "shift": shift, "completion": 0, "center": [50, 50],
         "intensity": 100, "direction": 0}
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
    return R.frame(glow_wipe(working(), k), c["shift"])


def plain(c):
    return R.frame(working(), c["shift"])


def file_json(fx, c):
    params = {w: c[w] for w in WORDS}
    params.update({n: setting_json(c[n]) for n in NUMBERS})
    return B.project_json(fx, "core.glow_wipe", params, c["shift"])


CASES = {
    "FX-GLOW-001": ("The settings as they start: completion 0, so nothing changes.", case(), [0]),
    "FX-GLOW-002": ("Completion 100: every pixel gone, and no light left.",
                    case(completion=100), [0]),
    "FX-GLOW-003": ("Radial, completion 40: a hole from the centre, a white glow around its edge "
                    "on the layer and inside the hole.", case(completion=40), [0]),
    "FX-GLOW-004": ("Doors, completion 40: two walls of light moving left and right from the "
                    "centre.", case("doors", completion=40), [0]),
    "FX-GLOW-005": ("Rectangular, completion 40: a growing square of light.",
                    case("rectangular", completion=40), [0]),
    "FX-GLOW-006": ("Rectangular, direction 30, completion 40: the square turned.",
                    case("rectangular", completion=40, direction=30), [0]),
    "FX-GLOW-007": ("Doors, direction 90, completion 40: the walls move up and down.",
                    case("doors", completion=40, direction=90), [0]),
    "FX-GLOW-008": ("Radial, completion 40, Color from Source on: the glow is the layer's own "
                    "colour, and none over the transparent columns.",
                    case(color_from_source="on", completion=40), [0]),
    "FX-GLOW-009": ("Radial, completion 40, colour #ff8000, intensity 250: a strong orange glow.",
                    case(color="#ff8000", completion=40, intensity=250), [0]),
    "FX-GLOW-010": ("Radial, completion 40, intensity 0: the hole alone, no light.",
                    case(completion=40, intensity=0), [0]),
    "FX-GLOW-011": ("Radial, completion 40, reversed: the wipe from the outside in, the middle "
                    "kept.", case(reverse="on", completion=40), [0]),
    "FX-GLOW-012": ("Radial, Center [25, 30], completion 30: the hole moved up and to the left.",
                    case(completion=30, center=[25, 30]), [0]),
    "FX-GLOW-013": ("Completion keyed from 0 at frame 0 to 100 at frame 4, linear: frame 2 is "
                    "completion 50, frame 4 all clear.",
                    case(completion=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-GLOW-014": ("Intensity keyed from 0 at frame 0 to 400 at frame 4, linear, completion 40: "
                    "frame 2 is intensity 200.",
                    case(completion=40, intensity=keyed((0, 0), (4, 400))), [0, 2, 4]),
    "FX-GLOW-015": ("FX-GLOW-003 moved three pixels right: the same, moved.",
                    case(completion=40, shift=3), [0, 3]),
    "FX-GLOW-016": ("Doors, direction -135, reversed, Color from Source on, Center [60, 40], "
                    "completion 55.", case("doors", "on", reverse="on", completion=55,
                                           direction=-135, center=[60, 40]), [0]),
}

INVALID = {
    "FX-GLOW-017": ("Completion 101, above 100.", case(completion=101)),
    "FX-GLOW-018": ("Completion -1, below 0.", case(completion=-1)),
    "FX-GLOW-019": ("Center y -1001, below -1000.", case(center=[50, -1001])),
    "FX-GLOW-020": ("Intensity -1, below 0.", case(intensity=-1)),
    "FX-GLOW-021": ("Intensity 1001, above 1000.", case(intensity=1001)),
    "FX-GLOW-022": ("Direction 3601, above 3600.", case(direction=3601)),
    "FX-GLOW-023": ("Shape \"Radial\": the word is exact, so a capital is not it.",
                    case("Radial")),
    "FX-GLOW-024": ("Shape \"round\": not a shape of Glow Wipe.", case("round")),
    "FX-GLOW-025": ("Color from Source \"yes\": it is \"off\" or \"on\".", case(color_from_source="yes")),
    "FX-GLOW-026": ("Color \"#ff80\": not six hex digits.", case(color="#ff80")),
    "FX-GLOW-027": ("Reverse \"true\": it is \"off\" or \"on\".", case(reverse="true")),
    "FX-GLOW-028": ("Intensity keyed to 2000 at frame 4: a key outside the range.",
                    case(intensity=keyed((0, 100), (4, 2000)))),
}


def main():
    clear()
    expected = B.write_cases(OUT, "glow_wipe", CASES, INVALID, render, plain, file_json)
    check(expected)


def clear():
    """No pixel centre on the edge."""
    for fx, (_, c, frames) in CASES.items():
        for f in frames:
            k = settings(c, f)
            if k["completion"] == 0:
                continue
            far, _, front = numbers(k)
            for y in range(H):
                for x in range(W):
                    assert abs(place(k, far, x + 0.5, y + 0.5) - front) > 1e-4, (fx, f, x, y)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    px = working()
    clear_px = [list(CLEAR)] * (W * H)
    assert c["FX-GLOW-001"]["0"] == R.frame(px, 0)
    assert c["FX-GLOW-002"]["0"] == R.frame(clear_px, 0)
    assert c["FX-GLOW-013"]["4"] == R.frame(clear_px, 0)
    assert len({repr(c[f]["0"]) for f in ("FX-GLOW-003", "FX-GLOW-004", "FX-GLOW-005",
                                          "FX-GLOW-006", "FX-GLOW-007", "FX-GLOW-008",
                                          "FX-GLOW-009", "FX-GLOW-010", "FX-GLOW-011",
                                          "FX-GLOW-012")}) == 10
    three, ten = c["FX-GLOW-003"]["0"], c["FX-GLOW-010"]["0"]
    # Some pixels gone; some lit inside the hole (clear before the light); some lit on the layer.
    assert any(p == CLEAR and q[3] > 0 for p, q in zip(ten, px))
    assert any(t == CLEAR and p[3] > 0 for p, t in zip(three, ten))
    assert any(t != CLEAR and p[0] > t[0] for p, t in zip(three, ten))
    # From source: no light where the layer was transparent.
    eight = c["FX-GLOW-008"]["0"]
    assert all(e == CLEAR for e, q in zip(eight, px) if q[3] == 0)
    assert c["FX-GLOW-013"]["2"] == render(case(completion=50), 0)
    assert c["FX-GLOW-014"]["2"] == render(case(completion=40, intensity=200), 0)
    moved, still = c["FX-GLOW-015"]["0"], c["FX-GLOW-003"]["0"]
    for y in range(H):
        assert moved[y * W + 3:(y + 1) * W] == still[y * W:y * W + W - 3]
    print("checked")


if __name__ == "__main__":
    main()
