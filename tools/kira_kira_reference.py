"""Kira-kira, worked a second way.

D-186 adds `core.kira_kira`: the kira-kira (キラキラ) of anime finishing, small sparkling stars
set on a drawing's highlights, an eye's glint, a blade's edge, a jewel, each twinkling in and out
at its own time. It differs from Cross Glare (D-161), which streaks every bright pixel the same
way on every frame: Kira-kira sets one star on each patch of highlight, at the patch's middle,
leaves some patches bare, and makes each star grow and fade on its own beat. It is this program's
own rule; nothing is ported. Document 21 is the rule in words; this file is the reference for the
numbers document 25 pins against it.

The rule. With `size`, `density` or `opacity` 0 the output is the input and nothing grows.
Otherwise the layer grows by ceil(size) transparent pixels on every side. A highlight is a pixel
that shows at all whose smallest channel, in 8-bit steps, is at least `threshold` percent of 255:
near white, as an anime highlight is painted. A bright colour, a gold or a pale skin, is no
highlight, where Glow's and Cross Glare's bright test (D-89) takes the largest channel and would
sparkle a whole face. The drawing's own space, a
pixel's centre (X, Y) = (x + 0.5, y + 0.5) with its top-left pixel (0, 0), is cut into square
cells `spacing` wide, cell (i, j) = (floor(X / spacing), floor(Y / spacing)). A cell with at least
one highlight holds a star when (U(seed, i, j, 0, 0) + 1) / 2 < density / 100, U Noise's hash
(D-119, `tools/noise_reference.py`); the star's centre c is the mean of its highlights' centres,
its phase phi = (U(seed, i, j, 0, 1) + 1) / 2 and its scale beta = 0.6 + 0.2 (U(seed, i, j, 0, 2)
+ 1). At composition frame f its twinkle is g = 0.5 + 0.5 cos(2 pi (f / period + phi)), its life
tau = 1 - twinkle / 100 (1 - g) and its arm R = size beta tau; a star with R = 0 gives nothing.
Its light at a point P, d = P - c, is tau times the largest of: for each arm axis v with length L,
(1 - a / L)^2 (1 - b / h) where a = |d . v| < L and b = |d x v| < h, h = 0.5 + R / 32, else 0;
and the core, (1 - |d| / (R / 4))^2 where |d| < R / 4, else 0. The axes are u(angle) and
u(angle + 90), length R; with `shape` "star" also u(angle + 45) and u(angle + 135), length R / 2;
u(t) = (sin t, -cos t), exact at whole multiples of 90. G(P) is the largest light of any star at
P. With C the colour's linear value, O the grown input and s = opacity / 100 G at the pixel's
centre, the output is O.rgb + s C, not clamped, and O.a + s (1 - O.a). A draft scales spacing and
size as distances.

A star's being there is a yes-or-no choice from a number: `check` asserts that no cell any case
reaches has (U + 1) / 2 within 1e-5 of density / 100.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless the
case says, drawn below. The drawing goes into `Fixtures/kira_kira/media`, the projects into
`Fixtures/kira_kira`, and the expected frames into `Fixtures/kira_kira/expected_kira_kira.json`. A
layer that grows is shown with its grown pixels in place.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/kira_kira_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
from noise_reference import u as U  # noqa: E402
from rain_reference import unit  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import glow_reference as G  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "kira_kira"
TOLERANCE = 2e-5  # document 25's default for a filter
CLIFF = 1e-5      # the spec's margin: no reached cell this close to the density's cliff
RANGES = {"threshold": (0, 100), "spacing": (2, 1000), "density": (0, 100), "size": (0, 1000),
          "angle": (-3600, 3600), "twinkle": (0, 100), "period": (1, 1000),
          "seed": (0, 100000), "opacity": (0, 100)}
FLOORED = ("seed",)
NUMBERS = ("threshold", "spacing", "density", "size", "angle", "twinkle", "period", "seed",
           "opacity")
NAMES = NUMBERS + ("shape", "color")
SHAPES = ("cross", "star")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def growth(size, density, opacity):
    return 0 if size == 0 or density == 0 or opacity == 0 else math.ceil(size)


def highlights(pixels, threshold, w=W):
    return [(i % w + 0.5, i // w + 0.5) for i, p in enumerate(pixels)
            if p[3] > 0 and min(p[:3]) * 100 >= threshold * 255]


def cells(points, spacing):
    """Each cell holding a highlight, with its highlights' centres, in the order first met."""
    out = {}
    for X, Y in points:
        out.setdefault((math.floor(X / spacing), math.floor(Y / spacing)), []).append((X, Y))
    return out


def chosen(seed, i, j, density):
    return (U(seed, i, j, 0, 0) + 1) / 2 < density / 100


def stars(pixels, threshold, spacing, density, size, twinkle, period, seed, frame_no, w=W):
    """Each star as (cx, cy, R, tau)."""
    out = []
    for (i, j), pts in cells(highlights(pixels, threshold, w), spacing).items():
        if not chosen(seed, i, j, density):
            continue
        cx = sum(p[0] for p in pts) / len(pts)
        cy = sum(p[1] for p in pts) / len(pts)
        phi = (U(seed, i, j, 0, 1) + 1) / 2
        beta = 0.6 + 0.2 * (U(seed, i, j, 0, 2) + 1)
        g = 0.5 + 0.5 * math.cos(2 * math.pi * (frame_no / period + phi))
        tau = 1 - twinkle / 100 * (1 - g)
        out.append((cx, cy, size * beta * tau, tau))
    return out


def axes(shape, angle, R):
    out = [(unit(angle), R), (unit(angle + 90), R)]
    if shape == "star":
        out += [(unit(angle + 45), R / 2), (unit(angle + 135), R / 2)]
    return out


def light(star, arms, X, Y):
    cx, cy, R, tau = star
    if R <= 0:
        return 0.0
    dx, dy = X - cx, Y - cy
    h = 0.5 + R / 32
    best = 0.0
    for (vx, vy), L in arms:
        a, b = abs(dx * vx + dy * vy), abs(dx * vy - dy * vx)
        if a < L and b < h:
            best = max(best, (1 - a / L) ** 2 * (1 - b / h))
    r = math.hypot(dx, dy)
    if r < R / 4:
        best = max(best, (1 - r / (R / 4)) ** 2)
    return tau * best


def kira_kira(pixels, threshold, spacing, density, size, angle, twinkle, period, seed, opacity,
              shape, color, frame_no, size_px=(W, H)):
    """A function giving the output at pixel (x, y) of the drawing's own space, a grown pixel
    having negative coordinates or coordinates past the size; empty outside the grown layer.
    Numbers are already held; seed already floored."""
    w, h = size_px
    working = [R.working(p) for p in pixels]
    inside = lambda x, y: 0 <= x < w and 0 <= y < h  # noqa: E731
    grow = growth(size, density, opacity)
    if grow == 0:
        return lambda x, y: working[y * w + x] if inside(x, y) else EMPTY
    found = stars(pixels, threshold, spacing, density, size, twinkle, period, seed, frame_no, w)
    tint = [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]

    def at(x, y):
        if not (-grow <= x < w + grow and -grow <= y < h + grow):
            return EMPTY
        o = working[y * w + x] if inside(x, y) else EMPTY
        g = max([light(st, axes(shape, angle, st[2]), x + 0.5, y + 0.5) for st in found],
                default=0.0)
        s = opacity / 100 * g
        return [o[ch] + s * tint[ch] for ch in range(3)] + [o[3] + s * (1 - o[3])]
    return at


# --- the drawing ----------------------------------------------------------------------------

WHITE = (250, 250, 255, 255)        # a cool white: smallest channel 250, 98 %
GREY = (153, 153, 153, 255)         # smallest channel exactly 60 %
GOLD, DARK = G.BRIGHT, G.DARK       # warm yellow: largest channel 98 %, smallest 47 %; purple
SOFT = WHITE[:3] + (128,)           # the white at half covering, a soft glint
NONE = S.NONE
POINT, PAIR, SOFT_AT = (2, 4), ((10, 2), (11, 2), (10, 3), (11, 3)), (4, 8)
GREY_AT, GOLD_AT = (13, 8), (8, 7)


def glints(x, y):
    """A purple patch, columns 1 to 6 and rows 1 to 7, with a white glint at (2, 4) in it; a
    white glint two pixels square, (10, 2) to (11, 3), on the clear; the white at half covering
    at (4, 8), a soft glint; a grey point at (13, 8) and a gold one at (8, 7). Everything else is
    empty."""
    if (x, y) == POINT or (x, y) in PAIR:
        return WHITE
    if (x, y) == SOFT_AT:
        return SOFT
    if (x, y) == GREY_AT:
        return GREY
    if (x, y) == GOLD_AT:
        return GOLD
    if 1 <= x <= 6 and 1 <= y <= 7:
        return DARK
    return NONE


DRAWINGS = {"glints": [[glints(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(threshold=95, spacing=64, density=60, size=40, angle=0, twinkle=100, period=24, seed=0,
         opacity=100, shape="star", color="#ffffff", shift=0):
    return {"drawing": "glints", "threshold": threshold, "spacing": spacing, "density": density,
            "size": size, "angle": angle, "twinkle": twinkle, "period": period, "seed": seed,
            "opacity": opacity, "shape": shape, "color": color, "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = min(hi, max(lo, value_at(c[k], frame_no)))
    return math.floor(v) if k in FLOORED else v


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    out = kira_kira(pixels, *[held(c, k, frame_no) for k in NUMBERS], c["shape"], c["color"],
                    frame_no)
    return [out(x - c["shift"], y) for y in range(H) for x in range(W)]


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(opacity=0, shift=c["shift"]), 0)


STEADY = {"spacing": 8, "size": 6, "density": 100, "twinkle": 0, "shape": "cross"}

CASES = {
    "FX-KIRA-001": ("The settings as they start: threshold 95, spacing 64, density 60, size 40, "
                    "a star, angle 0, twinkle 100, period 24, seed 0, opacity 100, white. The "
                    "frame is one cell, and it is chosen, so its three white glints, the point, "
                    "the square and the soft one, share one star at their middle, (8.5, 4.17), on "
                    "the clear; the grey, 60 %, is no highlight, nor is the gold, bright but "
                    "coloured. At frame 0 the star is early in "
                    "its twinkle and small; four frames later it is larger and brighter.",
                    case(), [0, 4]),
    "FX-KIRA-002": ("Opacity 0: the drawing, untouched, and nothing grows.",
                    case(opacity=0), [0]),
    "FX-KIRA-003": ("Size 0: the drawing, untouched, and nothing grows.",
                    case(size=0), [0]),
    "FX-KIRA-004": ("Density 0: no cell is chosen; the drawing, untouched, and nothing grows.",
                    case(density=0), [0]),
    "FX-KIRA-005": ("Threshold 100: nothing is that bright, so there is no star and the drawing "
                    "is untouched.",
                    case(threshold=100), [0]),
    "FX-KIRA-006": ("Spacing 8, size 6, density 100, twinkle 0, a cross: three steady stars, one "
                    "in each cell with a highlight, a plus of four arms fading to nothing, with a "
                    "small round core: on the white point, (2.5, 4.5), landing on whole pixels; on "
                    "the square's middle, (11, 3), a pixel corner, so its arms are shared between "
                    "the pixels either side; and on the soft glint, (4.5, 8.5), as strong as the "
                    "others. The grey and the gold have none; pixels off the arms stay as they "
                    "were.",
                    case(**STEADY), [0]),
    "FX-KIRA-007": ("FX-KIRA-006 as a star: each also has four short arms on the diagonals, half "
                    "as long, so the pixels diagonally next to the white point light too.",
                    case(**{**STEADY, "shape": "star"}), [0]),
    "FX-KIRA-008": ("FX-KIRA-006 at angle 45: each plus turned into an X; the pixels straight "
                    "beside the white point, lit in FX-KIRA-006, now keep only the core's light.",
                    case(**{**STEADY, "angle": 45}), [0]),
    "FX-KIRA-009": ("Angle 90: a quarter turn of a plus is the same plus, so it is FX-KIRA-006.",
                    case(**{**STEADY, "angle": 90}), [0]),
    "FX-KIRA-010": ("FX-KIRA-006 at threshold 60: the grey, at exactly 60 %, is a highlight "
                    "too, and has its own star at (13.5, 8.5); the gold, its smallest channel "
                    "47 %, is still none.",
                    case(**{**STEADY, "threshold": 60}), [0]),
    "FX-KIRA-011": ("FX-KIRA-006 at spacing 16: the frame is one cell, so the three glints "
                    "share one star at their middle, (8.5, 4.17).",
                    case(**{**STEADY, "spacing": 16}), [0]),
    "FX-KIRA-012": ("FX-KIRA-006 at density 60: the square's cell is not chosen at seed 0, so the "
                    "square has no star; the other two are FX-KIRA-006's exactly.",
                    case(**{**STEADY, "density": 60}), [0]),
    "FX-KIRA-013": ("FX-KIRA-006 at seed 7: the stars stay on the glints, each a different size.",
                    case(**{**STEADY, "seed": 7}), [0]),
    "FX-KIRA-014": ("FX-KIRA-006 twinkling fully, twinkle 100, period 4: each star grows and "
                    "fades on its own beat, frame 1 and frame 2 different from frame 0, and "
                    "frame 4, a whole period on, frame 0 again.",
                    case(**{**STEADY, "twinkle": 100, "period": 4}), [0, 1, 2, 4]),
    "FX-KIRA-015": ("Twinkle 50, period 4: each star breathes between half and full, and never "
                    "goes out.",
                    case(**{**STEADY, "twinkle": 50, "period": 4}), [0, 2]),
    "FX-KIRA-016": ("Colour #ff8000, orange: FX-KIRA-006's stars tinted, their red as before, "
                    "their green a fifth and their blue gone; the covering as FX-KIRA-006's.",
                    case(**{**STEADY, "color": "#ff8000"}), [0]),
    "FX-KIRA-017": ("FX-KIRA-016 with its colour written in capitals, #FF8000: the same.",
                    case(**{**STEADY, "color": "#FF8000"}), [0]),
    "FX-KIRA-018": ("Opacity 50: FX-KIRA-006's light at half strength.",
                    case(**{**STEADY, "opacity": 50}), [0]),
    "FX-KIRA-019": ("Size keyed from 0 at frame 0 to 12 at frame 4, linear: frame 0 is the "
                    "drawing, frame 2 is FX-KIRA-006, frame 4 is size 12.",
                    case(**{**STEADY, "size": keyed((0, 0), (4, 12))}), [0, 2, 4]),
    "FX-KIRA-020": ("Opacity eased from 0 at frame 0 to 100 at frame 4 on a curve that "
                    "overshoots: at frame 2 it would pass 100, is held at 100, and is "
                    "FX-KIRA-006, as frame 4 is.",
                    case(**{**STEADY, "opacity": keyed((0, 0, OVERSHOOT), (4, 100))}), [0, 2, 4]),
    "FX-KIRA-021": ("FX-KIRA-006 moved three pixels right: the stars move with the drawing, and "
                    "the white point's left arm, which runs past the drawing's left edge, shows "
                    "in the grown column just left of it; the two columns further left stay "
                    "empty.",
                    case(**{**STEADY, "shift": 3}), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-KIRA-022": ("Threshold 101, above 100.", case(threshold=101)),
    "FX-KIRA-023": ("Spacing 1, below 2.", case(spacing=1)),
    "FX-KIRA-024": ("Size 1001, above 1000.", case(size=1001)),
    "FX-KIRA-025": ("Period 0, below 1.", case(period=0)),
    "FX-KIRA-026": ("Twinkle keyed to 150 at frame 4.", case(twinkle=keyed((0, 100), (4, 150)))),
    "FX-KIRA-027": ("A shape written \"circle\", not cross or star.", case(shape="circle")),
    "FX-KIRA-028": ("A colour written \"#12345\", one digit short.", case(color="#12345")),
    "FX-KIRA-029": ("Seed 100001, above 100000.", case(seed=100001)),
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
        "instance_id": "fx-0-0", "type_id": "core.kira_kira", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
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

    (OUT / "expected_kira_kira.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    lit = lambda px, x, y: px[at(x, y)][3] > drawn[at(x, y)][3] + 1e-9 or \
        px[at(x, y)][0] > drawn[at(x, y)][0] + 1e-9  # noqa: E731
    same = lambda px, x, y: px[at(x, y)] == drawn[at(x, y)]  # noqa: E731
    pixels = [p for row in DRAWINGS["glints"] for p in row]
    yx, yy = POINT

    # No reached cell sits near the density's cliff.
    for fx, (_, cs, frames) in CASES.items():
        for f in frames:
            d, sp, sd = held(cs, "density", f), held(cs, "spacing", f), held(cs, "seed", f)
            for i, j in cells(highlights(pixels, held(cs, "threshold", f)), sp):
                assert abs((U(sd, i, j, 0, 0) + 1) / 2 - d / 100) > CLIFF, (fx, i, j)
    # The pieces: the highlights, the cells, the axes exact at quarter turns.
    assert highlights(pixels, 95) == [(10.5, 2.5), (11.5, 2.5), (10.5, 3.5), (11.5, 3.5),
                                      (2.5, 4.5), (4.5, 8.5)]
    assert len(highlights(pixels, 60)) == 7 and highlights(pixels, 100) == []
    # The gold is a highlight only at 47 or under, where D-89's test takes it at 95.
    assert (8.5, 7.5) in highlights(pixels, 47) and len(highlights(pixels, 48)) == 7
    assert G.glows(GOLD, {"based_on": "bright"}, 95, 0)
    assert sorted(cells(highlights(pixels, 95), 8)) == [(0, 0), (0, 1), (1, 0)]
    assert list(cells(highlights(pixels, 95), 64)) == [(0, 0)]
    assert [chosen(0, i, j, 60) for i, j in ((0, 0), (1, 0), (0, 1))] == [True, False, True]
    assert axes("cross", 0, 1) == [((0.0, -1.0), 1), ((1.0, 0.0), 1)]
    assert growth(40, 60, 100) == 40 and growth(0.5, 1, 1) == 1 and growth(6, 100, 0) == 0
    # Everywhere: nothing is taken away, the covering stays inside 0 to 1.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-KIRA-021" else 0))
        for px in frames.values():
            for p, q in zip(px, base):
                assert 0 <= p[3] <= 1 and p[3] >= q[3], fx
                assert all(p[ch] >= q[ch] - 1e-15 for ch in range(3)), fx

    # 001: one star at the glints' middle, small at frame 0 and larger at frame 4.
    one = stars(pixels, 95, 64, 60, 40, 100, 24, 0, 0)
    four = stars(pixels, 95, 64, 60, 40, 100, 24, 0, 4)
    assert len(one) == 1 and near([one[0][:2]], [(51 / 6, 25 / 6)])
    assert 0 < one[0][2] < four[0][2] < 40 and one[0][3] < four[0][3]
    lit0 = sum(lit(c["FX-KIRA-001"]["0"], x, y) for x in range(W) for y in range(H))
    lit4 = sum(lit(c["FX-KIRA-001"]["4"], x, y) for x in range(W) for y in range(H))
    assert 0 < lit0 < lit4
    for fx in ("FX-KIRA-002", "FX-KIRA-003", "FX-KIRA-004", "FX-KIRA-005"):
        assert c[fx]["0"] == drawn, fx
    # 006: three stars; the point's arms on whole pixels, exactly; the grey, the gold and the
    # pixels off
    # the arms untouched.
    six = c["FX-KIRA-006"]["0"]
    st = stars(pixels, 95, 8, 100, 6, 0, 24, 0, 0)
    assert [s[:2] for s in st] == [(11.0, 3.0), (2.5, 4.5), (4.5, 8.5)]
    R0 = st[1][2]
    purple, white = R.working(DARK), [1.0, 1.0, 1.0]
    k = (1 - 2 / R0) ** 2
    assert near([six[at(yx + 2, yy)]], [[purple[ch] + k * white[ch] for ch in range(3)] + [1.0]])
    assert near([six[at(yx, yy - 2)][:3]], [[purple[ch] + k for ch in range(3)]])
    assert lit(six, 0, yy) and lit(six, yx, 1) and lit(six, 11, 0) and lit(six, 4, 9)
    assert same(six, *GREY_AT) and same(six, 14, 8) and same(six, *GOLD_AT)
    for x, y in ((yx + 1, yy + 1), (yx - 1, yy - 2), (14, 6), (8, 8), (0, 0), (15, 9)):
        assert same(six, x, y), (x, y)
    assert six[at(10, 2)][0] > drawn[at(10, 2)][0]
    # The square's star, on a pixel corner, lights the pixels either side of each arm alike.
    assert near([six[at(13, 2)]], [six[at(13, 3)]]) and six[at(13, 2)][3] > 0
    # 007: the star adds diagonal arms; 008: the X.
    seven, eight = c["FX-KIRA-007"]["0"], c["FX-KIRA-008"]["0"]
    assert lit(seven, yx + 1, yy + 1) and not lit(six, yx + 1, yy + 1)
    assert all(seven[i][3] >= six[i][3] and seven[i][0] >= six[i][0] for i in range(W * H))
    assert lit(eight, yx + 1, yy + 1) and lit(eight, yx + 2, yy - 2)
    assert not lit(eight, yx + 2, yy) and lit(six, yx + 2, yy)
    assert c["FX-KIRA-009"]["0"] == six
    # 010: the grey's own star, and still none for the gold.
    ten = c["FX-KIRA-010"]["0"]
    assert lit(ten, 14, 8) and lit(ten, 13, 7) and ten[at(yx + 2, yy)] == six[at(yx + 2, yy)]
    # 011: one star at the middle; 012: the square has none, the others as before.
    eleven = stars(pixels, 95, 16, 100, 6, 0, 24, 0, 0)
    assert len(eleven) == 1 and near([eleven[0][:2]], [(51 / 6, 25 / 6)])
    assert c["FX-KIRA-011"]["0"] != six
    twelve = c["FX-KIRA-012"]["0"]
    assert same(twelve, 13, 2) and same(twelve, 11, 0)
    assert all(twelve[at(x, y)] == six[at(x, y)] for x in range(0, 5) for y in range(H))
    # 013: the same places, other sizes.
    thirteen = stars(pixels, 95, 8, 100, 6, 0, 24, 7, 0)
    assert [s[:2] for s in thirteen] == [s[:2] for s in st]
    assert [s[2] for s in thirteen] != [s[2] for s in st] and c["FX-KIRA-013"]["0"] != six
    # 014: twinkling; a whole period on is frame 0 again.
    t = c["FX-KIRA-014"]
    assert t["1"] != t["0"] and t["2"] != t["0"] and t["2"] != t["1"]
    assert near(t["4"], t["0"])
    lives = [s[3] for f in range(4) for s in stars(pixels, 95, 8, 100, 6, 100, 4, 0, f)]
    assert min(lives) < 0.5 < max(lives)
    # 015: never below half.
    half = [s[3] for f in range(4) for s in stars(pixels, 95, 8, 100, 6, 50, 4, 0, f)]
    assert min(half) >= 0.5 and c["FX-KIRA-015"]["0"] != c["FX-KIRA-015"]["2"]
    # 016 and 017: orange; 018: half strength.
    sixteen = c["FX-KIRA-016"]["0"]
    g = srgb_to_linear(128 / 255)
    for i in range(W * H):
        assert sixteen[i][0] == six[i][0] and sixteen[i][2] == drawn[i][2]
        assert sixteen[i][3] == six[i][3]
        assert abs(sixteen[i][1] - drawn[i][1] - g * (six[i][1] - drawn[i][1])) < 1e-12
    assert c["FX-KIRA-017"]["0"] == sixteen
    eighteen = c["FX-KIRA-018"]["0"]
    for i in range(W * H):
        assert abs(eighteen[i][0] - drawn[i][0] - 0.5 * (six[i][0] - drawn[i][0])) < 1e-12
    # The keyed cases meet the plain ones at their frames.
    k = c["FX-KIRA-019"]
    assert k["0"] == drawn and k["2"] == six and k["4"] == render(case(**{**STEADY, "size": 12}), 0)
    k = c["FX-KIRA-020"]
    assert ease(OVERSHOOT, 0.5) > 1
    assert k["0"] == drawn and k["2"] == k["4"] == six
    # 021: FX-KIRA-006 moved, the point's left arm on the grown column.
    moved = c["FX-KIRA-021"]["0"]
    assert all(moved[at(x, y)] == six[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert moved[at(2, yy)][3] > 0 and moved[at(1, yy)] == moved[at(0, yy)] == EMPTY
    assert all(moved[at(x, y)] == EMPTY for x in (0, 1, 2) for y in range(H) if y != yy)
    print("checked")


if __name__ == "__main__":
    main()
