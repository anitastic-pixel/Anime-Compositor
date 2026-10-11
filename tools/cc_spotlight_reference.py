"""CC Spotlight, worked a second way.

`core.cc_spotlight`, modelled on CycoreFX's CC Spotlight (Perspective), which "creates the
illusion of a spotlight shining down on your source layer". The CycoreFX HD manual names the
controls: From and To ("the starting (From) location of the light source and the target (To) on
the source layer, based on X, Y coordinates"), Height ("the distance between the virtual
spotlight and the target on the source layer in Z space"), Cone Angle ("the opening angle"),
Edge Softness ("lower values result in sharper edges"), Color, Intensity ("higher values produce
brighter results"), Render (Light Only, Light Add, Light Add+, Light Shadow, and four Gel
renders that project a Gel Layer) and Gel Layer. It publishes no formula and no settings when
added, so the rule and those settings are ours (document 21, D-507). The four Gel renders and
the Gel Layer are not built yet: a Gel render is refused with a sentence saying so.

The settings. `from` and `to`, x then y, -1000 to 1000 per cent of the drawing's own width and
height (as Radial Shadow's light), [25, 25] and [50, 50] when added; `height`, 0 to 1000 per cent
of the drawing's own width, 50 when added; `cone_angle`, 0 to 180 degrees, the whole opening, 40
when added; `edge_softness`, 0 to 100, 20 when added; `intensity`, 0 to 1000 per cent, 100 when
added; all six keyable. `color`, `#rrggbb`, "#ffffff" when added; `render`, "light_only",
"light_add", "light_add_plus" or "light_shadow" (when added), exact.

The rule, in the drawing's own W by H space, at the pixel's centre P = (x + 1/2, y + 1/2, 0): the
light sits at F = (from_x W / 100, from_y H / 100, -height W / 100) and points at
T = (to_x W / 100, to_y H / 100, 0). With the axis A = T - F and D = P - F, the angle off the
axis is theta = atan2(|A x D|, A . D). With the half opening o = cone_angle / 2 and the inner
angle n = o (1 - edge_softness / 100), the light's share f is 1 where theta <= n, 0 where
theta >= o, and smoothstep's t^2 (3 - 2t), t = (o - theta) / (o - n), between; with softness 0,
f is 1 where theta < o. f is 0 everywhere when A is 0 or the opening is 0. With i = intensity /
100, k = f i and C the colour's linear value, on the premultiplied pixel (c, a):

- light_only: (C k, min(k, 1)), the layer itself gone;
- light_add: (c + C k a, a), light only where the layer is;
- light_add_plus: (c + C k, a + (1 - a) min(k, 1)), light on the transparent parts too;
- light_shadow: (c ((1 - f)(1 - min(i, 1)) + f C), a): outside the cone darkened by the
  intensity, inside tinted by the colour.

The layer never grows: the light falls inside the layer's own picture. A draft changes nothing,
every distance being a share of the drawing.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values. With softness 0 the rule steps at theta = o; every such case is
checked that no pixel centre lies near it.

The drawing is Broadcast Colors' (tools/broadcast_safe_reference.py), 16 by 10, with transparent
columns and a half-covered one. The projects go into `Fixtures/cc_spotlight`, the expected frames
into `Fixtures/cc_spotlight/expected_cc_spotlight.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/cc_spotlight_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "cc_spotlight"
NUMBERS = ("from", "to", "height", "cone_angle", "edge_softness", "intensity")
WORDS = ("color", "render")
RANGES = {"from": (-1000, 1000), "to": (-1000, 1000), "height": (0, 1000),
          "cone_angle": (0, 180), "edge_softness": (0, 100), "intensity": (0, 1000)}


# --- the rule -------------------------------------------------------------------------------

def angle(k, x, y):
    """theta, the angle off the axis at pixel (x, y), or None when there is no axis."""
    f = (k["from"][0] / 100 * W, k["from"][1] / 100 * H, -k["height"] / 100 * W)
    t = (k["to"][0] / 100 * W, k["to"][1] / 100 * H, 0.0)
    a = [t[i] - f[i] for i in range(3)]
    if a == [0.0, 0.0, 0.0]:
        return None
    d = [x + 0.5 - f[0], y + 0.5 - f[1], -f[2]]
    cr = (a[1] * d[2] - a[2] * d[1], a[2] * d[0] - a[0] * d[2], a[0] * d[1] - a[1] * d[0])
    return math.atan2(math.sqrt(sum(v * v for v in cr)), sum(a[i] * d[i] for i in range(3)))


def share(k, x, y):
    th = angle(k, x, y)
    o = math.radians(k["cone_angle"]) / 2
    if th is None or o == 0:
        return 0.0
    n = o * (1 - k["edge_softness"] / 100)
    if k["edge_softness"] == 0:
        return 1.0 if th < o else 0.0
    if th <= n:
        return 1.0
    if th >= o:
        return 0.0
    t = (o - th) / (o - n)
    return t * t * (3 - 2 * t)


def colour(hexa):
    return [S.srgb_to_linear(int(hexa[i:i + 2], 16) / 255) for i in (1, 3, 5)]


def spotlight(px, k):
    C, i = colour(k["color"]), k["intensity"] / 100
    out = []
    for n, w in enumerate(px):
        f = share(k, n % W, n // W)
        q = f * i
        c, a = w[:3], w[3]
        if k["render"] == "light_only":
            out.append([v * q for v in C] + [min(q, 1.0)])
        elif k["render"] == "light_add":
            out.append([c[j] + C[j] * q * a for j in range(3)] + [a])
        elif k["render"] == "light_add_plus":
            out.append([c[j] + C[j] * q for j in range(3)] + [a + (1 - a) * min(q, 1.0)])
        else:
            out.append([c[j] * ((1 - f) * (1 - min(i, 1.0)) + f * C[j]) for j in range(3)] + [a])
    return out


def working():
    return [R.working(p) for p in B.pixels()]


# --- the cases ------------------------------------------------------------------------------

def case(render="light_shadow", color="#ffffff", shift=0, **numbers):
    c = {"render": render, "color": color, "shift": shift, "from": [25, 25], "to": [50, 50],
         "height": 50, "cone_angle": 40, "edge_softness": 20, "intensity": 100}
    for n, v in numbers.items():
        assert n in NUMBERS, n
        c[n] = v
    return c


def settings(c, frame_no):
    k = {n: value_at(c[n], frame_no) for n in NUMBERS}
    k.update(color=c["color"], render=c["render"])
    return k


def render(c, frame_no):
    return R.frame(spotlight(working(), settings(c, frame_no)), c["shift"])


def plain(c):
    return R.frame(working(), c["shift"])


def file_json(fx, c):
    params = {w: c[w] for w in WORDS}
    params.update({n: setting_json(c[n]) for n in NUMBERS})
    return B.project_json(fx, "core.cc_spotlight", params, c["shift"])


CASES = {
    "FX-SPOT-001": ("The settings as they start: a white light at a quarter across and down, half "
                    "the width above the drawing, pointed at its middle, opening 40 degrees, edge "
                    "softness 20, intensity 100, render light shadow. Inside the cone the drawing "
                    "is as it was, outside it is black, between it fades.", case(), [0]),
    "FX-SPOT-002": ("Render light only: the drawing is gone and the light alone is left, white "
                    "and whole inside the cone, fading to transparent at its edge.",
                    case("light_only"), [0]),
    "FX-SPOT-003": ("Render light add: the light added to the drawing where it is covered; the "
                    "transparent columns stay transparent.", case("light_add"), [0]),
    "FX-SPOT-004": ("Render light add plus: the light added everywhere, so inside the cone the "
                    "transparent first column, the light aimed at it, becomes white light.",
                    case("light_add_plus", to=[10, 50], cone_angle=60), [0]),
    "FX-SPOT-005": ("Edge softness 0: the cone's edge is a hard step, every pixel whole or black.",
                    case(edge_softness=0), [0]),
    "FX-SPOT-006": ("Edge softness 100: the light fades from the axis itself to the edge.",
                    case(edge_softness=100), [0]),
    "FX-SPOT-007": ("Cone angle 90: a wider pool of light.", case(cone_angle=90), [0]),
    "FX-SPOT-008": ("Height 10: the light low and to one side, so its pool is a long oval "
                    "reaching away from it.", case(height=10, cone_angle=30), [0]),
    "FX-SPOT-009": ("An orange light, render light add: orange added inside the cone.",
                    case("light_add", "#ff8000"), [0]),
    "FX-SPOT-010": ("Intensity 250, render light add: two and a half times the light, so the "
                    "lit pixels run past 1.", case("light_add", intensity=250), [0]),
    "FX-SPOT-011": ("Intensity 50, render light shadow: outside the cone half as dark.",
                    case(intensity=50), [0]),
    "FX-SPOT-012": ("Intensity 0, render light shadow, white: no shadow and no tint, so the "
                    "drawing is exactly as it was.", case(intensity=0), [0]),
    "FX-SPOT-013": ("From at the middle, straight above the target: a round pool in the middle.",
                    case("light_add", **{"from": [50, 50]}), [0]),
    "FX-SPOT-014": ("Height 0 with From on To: the light has no direction, so nothing is lit and "
                    "light shadow leaves the drawing black.",
                    case(height=0, **{"from": [50, 50]}), [0]),
    "FX-SPOT-015": ("From keyed from [0, 0] at frame 0 to [100, 0] at frame 4, linear, render "
                    "light add: the light swings across the top.",
                    case("light_add", **{"from": keyed((0, (0, 0)), (4, (100, 0)))}), [0, 2, 4]),
    "FX-SPOT-016": ("Cone angle keyed from 10 at frame 0 to 70 at frame 4, linear.",
                    case(cone_angle=keyed((0, 10), (4, 70))), [0, 2, 4]),
    "FX-SPOT-017": ("FX-SPOT-003 moved three pixels right: the same, moved, light and all.",
                    case("light_add", shift=3), [0, 3]),
    "FX-SPOT-018": ("To at [150, 50], past the right edge: only the right side is lit, faintly "
                    "at the rim.", case("light_add", to=[150, 50], cone_angle=60), [0]),
}

INVALID = {
    "FX-SPOT-019": ("From x 1001, above 1000.", case(**{"from": [1001, 25]})),
    "FX-SPOT-020": ("Height -1, below 0.", case(height=-1)),
    "FX-SPOT-021": ("Cone angle 181, above 180.", case(cone_angle=181)),
    "FX-SPOT-022": ("Edge softness 101, above 100.", case(edge_softness=101)),
    "FX-SPOT-023": ("Intensity 1001, above 1000.", case(intensity=1001)),
    "FX-SPOT-024": ("Render \"gel_add\": CC Spotlight's Gel renders are not built yet.",
                    case("gel_add")),
    "FX-SPOT-025": ("Render \"Light_Add\": the word is exact, so a capital is not it.",
                    case("Light_Add")),
    "FX-SPOT-026": ("Color \"#ff80\": not six hex digits.", case(color="#ff80")),
    "FX-SPOT-027": ("Color \"white\": not a #rrggbb colour.", case(color="white")),
}


def main():
    step_clear()
    expected = B.write_cases(OUT, "cc_spotlight", CASES, INVALID, render, plain, file_json)
    check(expected)


def step_clear():
    """With softness 0 no pixel centre may lie within a hair of the cone's edge."""
    for fx, (_, c, frames) in CASES.items():
        for f in frames:
            k = settings(c, f)
            if k["edge_softness"] != 0:
                continue
            o = math.radians(k["cone_angle"]) / 2
            for y in range(H):
                for x in range(W):
                    th = angle(k, x, y)
                    assert th is None or abs(th - o) > 1e-4, (fx, x, y)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    px = working()
    near = lambda p, q, e=1e-12: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    one = c["FX-SPOT-001"]["0"]
    lit = [share(settings(case(), 0), n % W, n // W) for n in range(W * H)]
    assert any(v == 1 for v in lit) and any(v == 0 for v in lit) and any(0 < v < 1 for v in lit)
    for p, q, f in zip(one, px, lit):
        if f == 1:
            assert near(p, q)
        if f == 0:
            assert near(p, [0, 0, 0, q[3]])
    two = c["FX-SPOT-002"]["0"]
    assert all(near(p, [f, f, f, f]) for p, f in zip(two, lit))
    three = c["FX-SPOT-003"]["0"]
    assert all(near(p, q) for p, q in zip(three, px) if q[3] == 0)
    four = c["FX-SPOT-004"]["0"]
    assert any(q[3] == 0 and p[3] > 0.5 for p, q in zip(four, px))
    assert all(near(p, [f, f, f, f]) for p, q, f in zip(four, px, [share(settings(case(to=[10, 50], cone_angle=60), 0), n % W, n // W) for n in range(W * H)]) if q[3] == 0)
    five = c["FX-SPOT-005"]["0"]
    assert all(near(p, q) or near(p, [0, 0, 0, q[3]]) for p, q in zip(five, px))
    assert c["FX-SPOT-007"]["0"] != one and c["FX-SPOT-006"]["0"] != one
    assert any(max(p[:3]) > 1 for p in c["FX-SPOT-010"]["0"])
    eleven = c["FX-SPOT-011"]["0"]
    assert all(near(p, [v / 2 for v in q[:3]] + [q[3]]) for p, q, f in zip(eleven, px, lit) if f == 0)
    assert c["FX-SPOT-012"]["0"] == px
    assert all(near(p, [0, 0, 0, q[3]]) for p, q in zip(c["FX-SPOT-014"]["0"], px))
    fifteen = c["FX-SPOT-015"]
    assert fifteen["2"] == render(case("light_add", **{"from": [50, 0]}), 0)
    assert c["FX-SPOT-016"]["2"] == render(case(cone_angle=40), 0)
    moved = c["FX-SPOT-017"]["0"]
    for y in range(H):
        assert moved[y * W + 3:(y + 1) * W] == three[y * W:y * W + W - 3]
    eighteen = c["FX-SPOT-018"]["0"]
    assert all(near(p, q) for p, q in zip(eighteen[:W // 2], px[:W // 2]))
    print("checked")


if __name__ == "__main__":
    main()
