"""Color Difference Key, worked a second way.

`core.color_difference_key`, modelled on After Effects' Color Difference Key (Keying): it pulls
a matte from a key colour as two partial mattes, A and B, each with its own levels, combines
them, and takes the combined matte, with levels of its own, as the layer's covering. Adobe
describes the effect ("divides an image into two mattes, partial matte B and partial matte A
... Partial matte B makes the background transparent based on the key color specified, and
partial matte A makes areas transparent that do not contain a second, different color") and
names its controls, but publishes no formula. The rule below follows Stefan Ihringer's
reverse-engineering, "The After Effects Color Difference Key" (bildfehler.de/fx/cdk.html,
2007): hue windows in HLS, the saturation and lightness terms and the screen that combines the
partials are his. Nothing is ported; it is our rule (document 21, D-459), and where it differs
from Adobe's own maths is a logged gap.

The settings. `view`, which picture the layer shows: "final" (when added), "source",
"partial_a_uncorrected", "partial_a", "partial_b_uncorrected", "partial_b",
"matte_uncorrected" or "matte"; the word is exact. `key_color`, `#rrggbb`, the screen,
`#0000ff` when added. Then for partial A and partial B in turn D-112's five levels:
`partial_a_in_black`, `partial_a_in_white`, `partial_a_gamma`, `partial_a_out_black`,
`partial_a_out_white` (and the same for `partial_b_`), 0 to 255 but the gamma 0.1 to 10, when
added 0, 255, 1, 0, 255; then the matte's `matte_in_black`, `matte_in_white` and `matte_gamma`,
0, 255 and 1. Every number is keyable; the view and the key colour are not.

The rule. At a pixel with covering a > 0, e = (eR, eG, eB) its straight colour through the sRGB
curve held inside 0 to 1, and H, L and S its hue in degrees, lightness (max + min) / 2 and
saturation (max - min) / (1 - |2L - 1|), the saturation 0 for a grey, whose hue is then
immaterial. K is the key colour's hue, 0 for a grey key.

- For partial B the centre is K, for partial A it is K - 60 degrees. d is the distance from H
  to the centre round the wheel, 0 to 180, and the hue term is 0 when d is 60 or less, 1 when
  d is 120 or more, and (d - 60) / 60 between.
- Saturation: t = S t + (1 - S) / 2, so a grey is a half.
- Lightness, as a hard light: partial A is 2 t L below L = 1/2 and 1 - 2 (1 - t)(1 - L) from
  there; partial B is 1 - 2 (1 - t) L below and 2 t (1 - L) from there. These are the
  uncorrected partials.
- Each corrected partial is its uncorrected one through its levels: with x = 255 t, v = (x -
  in_black) / (in_white - in_black) held inside 0 to 1 (with in_white = in_black, 1 where x is
  at least in_black, else 0), the partial is (out_black + v^(1/gamma) (out_white - out_black))
  / 255. The same step as Levels (D-112).
- The uncorrected matte is the screen 1 - (1 - A)(1 - B) of the uncorrected partials; the
  corrected matte is the screen of the corrected partials through the matte's levels (its
  output 0 to 255).

What the layer shows: "final", every channel of the pixel (colour and covering) times the
corrected matte, so the colour is kept and only the covering falls; "source", the layer exactly
as it is; any other view, that matte or partial m as grey: m taken as an encoded grey, back to
linear at the pixel's covering, which is kept. A pixel with a = 0 is left as it is.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works on its single-precision buffers. The
rule is continuous in the colour (the hue term's window has no step and a grey's hue is
weighted by its saturation, 0) except where a levels has in_white = in_black; the case that
sets one checks that no pixel lies near its step.

The drawing is this file's own, laid out as Broadcast Safe's (tools/broadcast_safe_reference.py,
each column one colour, each row darker by 8%): black, white, grey, a green screen, a blue
screen, skin, pure blue, cyan, pure green, yellow, red, magenta, a violet, a pale blue, and the
blue screen at half covering. The projects go into `Fixtures/color_difference_key`, the
expected frames into `Fixtures/color_difference_key/expected_color_difference_key.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/color_difference_key_reference.py
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "color_difference_key"
LEVELS = ("in_black", "in_white", "gamma", "out_black", "out_white")
NUMBERS = tuple(f"partial_{s}_{n}" for s in "ab" for n in LEVELS) + (
    "matte_in_black", "matte_in_white", "matte_gamma")
VIEWS = ("final", "source", "partial_a_uncorrected", "partial_a", "partial_b_uncorrected",
         "partial_b", "matte_uncorrected", "matte")

COLOURS = [
    None,               # 0: empty
    (0, 0, 0),          # 1: black
    (255, 255, 255),    # 2: white
    (128, 128, 128),    # 3: grey
    (40, 180, 60),      # 4: a green screen
    (30, 70, 200),      # 5: a blue screen
    (230, 180, 150),    # 6: skin
    (0, 0, 255),        # 7: blue
    (0, 255, 255),      # 8: cyan
    (0, 255, 0),        # 9: green
    (255, 255, 0),      # 10: yellow
    (255, 0, 0),        # 11: red
    (255, 0, 255),      # 12: magenta
    (128, 64, 255),     # 13: a violet
    (150, 170, 230),    # 14: a pale blue
    (30, 70, 200),      # 15: the blue screen at half covering
]


def pixel(x, y):
    c = COLOURS[x]
    if c is None:
        return S.NONE
    f = 1 - 0.08 * y
    return tuple(int(v * f + 0.5) for v in c) + ((128,) if x == 15 else (255,))


# The shared helpers read the drawing from B.DRAWINGS["colours"]; this file's drawing is put
# there, so B.pixels, B.write_cases and B.plain draw it.
B.DRAWINGS["colours"] = [[pixel(x, y) for x in range(W)] for y in range(H)]


# --- the rule -------------------------------------------------------------------------------

def hue(e):
    """HSV (and HLS) hue in degrees, None for a grey."""
    mx, mn = max(e), min(e)
    if mx == mn:
        return None
    d = mx - mn
    r, g, b = e
    if r == mx:
        return (60 * (g - b) / d) % 360
    if g == mx:
        return 60 * ((b - r) / d + 2)
    return 60 * ((r - g) / d + 4)


def hls(e):
    mx, mn = max(e), min(e)
    lt = (mx + mn) / 2
    s = 0.0 if mx == mn else (mx - mn) / (1 - abs(2 * lt - 1))
    return hue(e), lt, s


def hex_colour(text):
    return [int(text[i:i + 2], 16) / 255 for i in (1, 3, 5)]


def window(h, centre):
    d = abs((h - centre) % 360)
    d = min(d, 360 - d)
    return min(1.0, max(0.0, (d - 60) / 60))


def level(s, t):
    ib, iw, g, ob, ow = s
    x = 255 * t
    if iw == ib:
        v = 1.0 if x >= ib else 0.0
    else:
        v = min(1.0, max(0.0, (x - ib) / (iw - ib)))
    return (ob + v ** (1 / g) * (ow - ob)) / 255


def screen(a, b):
    return 1 - (1 - a) * (1 - b)


def mattes(e, k):
    """The uncorrected and corrected partials and mattes of encoded colour e, by name."""
    h, lt, s = hls(e)
    h = 0.0 if h is None else h
    ta = s * window(h, k["hue"] - 60) + (1 - s) / 2
    tb = s * window(h, k["hue"]) + (1 - s) / 2
    if lt < 0.5:
        a, b = 2 * ta * lt, 1 - 2 * (1 - tb) * lt
    else:
        a, b = 1 - 2 * (1 - ta) * (1 - lt), 2 * tb * (1 - lt)
    ca, cb = level(k["a"], a), level(k["b"], b)
    return {"partial_a_uncorrected": a, "partial_a": ca, "partial_b_uncorrected": b,
            "partial_b": cb, "matte_uncorrected": screen(a, b),
            "matte": level(k["matte"], screen(ca, cb))}


def key(px, k):
    if k["view"] == "source":
        return [R.working(p) for p in px]
    out = []
    for p in px:
        if p[3] == 0:
            out.append(R.working(p))
            continue
        m = mattes(B.encoded(p), k)
        if k["view"] == "final":
            out.append([v * m["matte"] for v in R.working(p)])
        else:
            a = p[3] / 255
            g = S.srgb_to_linear(m[k["view"]]) * a
            out.append([g, g, g, a])
    return out


# --- the cases ------------------------------------------------------------------------------

def case(view="final", key_color="#0000ff", shift=0, **numbers):
    c = {"view": view, "key_color": key_color, "shift": shift}
    for n in NUMBERS:
        c[n] = 1 if n.endswith("gamma") else (255 if n.endswith("white") else 0)
    for n, v in numbers.items():
        assert n in NUMBERS, n
        c[n] = v
    return c


def settings(c, frame_no):
    v = {n: min(10 if n.endswith("gamma") else 255, max(0.1 if n.endswith("gamma") else 0,
                                                        value_at(c[n], frame_no)))
         for n in NUMBERS}
    k = {"view": c["view"], "hue": hue(hex_colour(c["key_color"])) or 0.0}
    for s in "ab":
        k[s] = tuple(v[f"partial_{s}_{n}"] for n in LEVELS)
    k["matte"] = (v["matte_in_black"], v["matte_in_white"], v["matte_gamma"], 0, 255)
    return k


def render(c, frame_no):
    return R.frame(key(B.pixels(), settings(c, frame_no)), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    params = {"view": c["view"], "key_color": c["key_color"]}
    params.update({n: setting_json(c[n]) for n in NUMBERS})
    return B.project_json(fx, "core.color_difference_key", params, c["shift"])


A_SET = dict(partial_a_in_black=40, partial_a_in_white=200, partial_a_gamma=1.5,
             partial_a_out_black=10, partial_a_out_white=245)
B_SET = dict(partial_b_in_black=30, partial_b_in_white=180, partial_b_gamma=0.7,
             partial_b_out_black=0, partial_b_out_white=250)
M_SET = dict(matte_in_black=60, matte_in_white=220, matte_gamma=1.3)
ALL = {**A_SET, **B_SET, **M_SET}

CASES = {
    "FX-CDK-001": ("The settings as they start: a blue key, the final view, every levels plain. "
                   "Blue and cyan (partial A's centre, 60 degrees short of blue) lose all their "
                   "covering, the blue screen most of it, the violet half; grey keeps three "
                   "quarters, black and white all of theirs; green, red and yellow, far from "
                   "blue, keep all of it. Colour is kept, only the covering falls.", case(), [0]),
    "FX-CDK-002": ("View source: the layer exactly as it is, whatever the levels.",
                   case("source", **ALL), [0]),
    "FX-CDK-003": ("View partial A uncorrected: partial A as grey, its levels not applied.",
                   case("partial_a_uncorrected", **A_SET), [0]),
    "FX-CDK-004": ("View partial A: partial A through its levels (in 40 to 200, gamma 1.5, "
                   "out 10 to 245).", case("partial_a", **A_SET), [0]),
    "FX-CDK-005": ("View partial B uncorrected: partial B as grey; blue near black, the colours "
                   "far from blue's hue bright.", case("partial_b_uncorrected", **B_SET), [0]),
    "FX-CDK-006": ("View partial B: partial B through its levels (in 30 to 180, gamma 0.7, out "
                   "0 to 250).", case("partial_b", **B_SET), [0]),
    "FX-CDK-007": ("View matte uncorrected: the screen of the two uncorrected partials; every "
                   "levels set and ignored.", case("matte_uncorrected", **ALL), [0]),
    "FX-CDK-008": ("View matte: the corrected partials screened, through the matte's levels "
                   "(in 60 to 220, gamma 1.3).", case("matte", **ALL), [0]),
    "FX-CDK-009": ("Final with every levels set: the covering times the corrected matte of "
                   "FX-CDK-008.", case(**ALL), [0]),
    "FX-CDK-010": ("A green key, #00ff00: green and yellow (partial A's centre) fall, the green "
                   "screen most of the way; blue keeps all its covering.",
                   case(key_color="#00ff00"), [0]),
    "FX-CDK-011": ("A sampled screen green, #28b43c, partial B in 0 to 128: the screen's hue "
                   "window, sharpened.", case(key_color="#28b43c", partial_b_in_white=128),
                   [0]),
    "FX-CDK-012": ("View matte with matte gamma 0.4: the matte darkened toward the key.",
                   case("matte", matte_gamma=0.4), [0]),
    "FX-CDK-013": ("View matte with matte in black and in white both 150: a hard step, white "
                   "where the matte is 150 or more, black below.",
                   case("matte", matte_in_black=150, matte_in_white=150), [0]),
    "FX-CDK-014": ("View partial A with output black 255 and output white 0: partial A turned "
                   "over.", case("partial_a", partial_a_out_black=255, partial_a_out_white=0),
                   [0]),
    "FX-CDK-015": ("A grey key, #808080: a grey's hue is read as 0, so the key is red's: red "
                   "and magenta fall, blue keeps its covering.", case("matte", key_color="#808080"),
                   [0]),
    "FX-CDK-016": ("Final, partial B in white keyed from 255 at frame 0 to 100 at frame 4, "
                   "linear.", case(partial_b_in_white=keyed((0, 255), (4, 100))), [0, 2, 4]),
    "FX-CDK-017": ("View matte, matte gamma keyed from 1 at frame 0 to 3 at frame 4.",
                   case("matte", matte_gamma=keyed((0, 1), (4, 3))), [0, 2, 4]),
    "FX-CDK-018": ("FX-CDK-009 moved three pixels right: the same, moved.",
                   case(shift=3, **ALL), [0, 3]),
}

INVALID = {
    "FX-CDK-019": ("Partial A in black 256, above 255.", case(partial_a_in_black=256)),
    "FX-CDK-020": ("Partial B out white -1, below 0.", case(partial_b_out_white=-1)),
    "FX-CDK-021": ("Partial A gamma 11, above 10.", case(partial_a_gamma=11)),
    "FX-CDK-022": ("Matte gamma 0.05, below 0.1.", case(matte_gamma=0.05)),
    "FX-CDK-023": ("Matte in white 256, above 255.", case(matte_in_white=256)),
    "FX-CDK-024": ("View \"Final\": the word is exact, so a capital is not it.", case("Final")),
    "FX-CDK-025": ("View \"all\": After Effects' four-up view is not one here.", case("all")),
    "FX-CDK-026": ("Key colour \"blue\", not a #rrggbb colour.", case(key_color="blue")),
}


def main():
    step_clear()
    expected = B.write_cases(OUT, "color_difference_key", CASES, INVALID, render, plain,
                             file_json)
    check(expected)


def step_clear():
    """FX-CDK-013's matte has a step at 150: no pixel may lie within a hair of it."""
    k = settings(CASES["FX-CDK-013"][1], 0)
    for p in B.pixels():
        if p[3]:
            m = mattes(B.encoded(p), k)
            x = 255 * screen(m["partial_a"], m["partial_b"])
            assert abs(x - 150) > 1e-2, x


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    px = B.pixels()
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731
    grey = lambda p: S.linear_to_srgb(p[0] / p[3])  # noqa: E731
    cover = lambda f, x, y: f[at(x, y)][3]  # noqa: E731

    one = c["FX-CDK-001"]["0"]
    assert near(one[at(7, 0)], [0, 0, 0, 0])  # pure blue: the key itself, all gone
    for x in (9, 10, 11, 2):  # green, yellow, red, white keep all their covering
        assert abs(cover(one, x, 0) - 1) < 1e-9, x
    assert abs(cover(one, 3, 0) - 0.75) < 1e-4  # a grey near L 1/2: a half and a half screened
    assert near(one[at(8, 0)], [0, 0, 0, 0])  # cyan, partial A's centre: gone too
    assert cover(one, 5, 0) < 0.35 and abs(cover(one, 13, 0) - 0.5) < 0.01
    for x in range(1, 16):  # the colour is kept: the straight colour unchanged where covered
        for y in range(H):
            p, q = one[at(x, y)], drawn[at(x, y)]
            if p[3] > 1e-6:
                assert near([u / p[3] for u in p[:3]], [u / q[3] for u in q[:3]], 1e-7)
    assert c["FX-CDK-002"]["0"] == drawn
    # Partials: the key's pure blue is 0 in B and, 60 degrees from A's centre, 0 in A.
    assert abs(grey(c["FX-CDK-003"]["0"][at(7, 0)])) < 1e-9
    assert abs(grey(c["FX-CDK-005"]["0"][at(7, 0)])) < 1e-9
    assert abs(grey(c["FX-CDK-005"]["0"][at(10, 0)]) - 1) < 1e-9  # yellow, opposite blue
    k = settings(case(**ALL), 0)
    for x, y in ((4, 0), (6, 2), (13, 5), (3, 0)):
        m = mattes(B.encoded(px[at(x, y)]), k)
        assert abs(grey(c["FX-CDK-004"]["0"][at(x, y)]) - m["partial_a"]) < 1e-9
        assert abs(grey(c["FX-CDK-006"]["0"][at(x, y)]) - m["partial_b"]) < 1e-9
        assert abs(grey(c["FX-CDK-008"]["0"][at(x, y)]) - m["matte"]) < 1e-9
        u = screen(m["partial_a_uncorrected"], m["partial_b_uncorrected"])
        assert abs(grey(c["FX-CDK-007"]["0"][at(x, y)]) - u) < 1e-9
        assert abs(cover(c["FX-CDK-009"]["0"], x, y) - m["matte"]) < 1e-9
    # The grey views keep the covering, and the half-covered screen reads as the screen.
    assert abs(c["FX-CDK-008"]["0"][at(15, 0)][3] - 128 / 255) < 1e-12
    assert abs(grey(c["FX-CDK-008"]["0"][at(15, 0)]) - grey(c["FX-CDK-008"]["0"][at(5, 0)])) < 1e-9
    ten = c["FX-CDK-010"]["0"]
    assert near(ten[at(9, 0)], [0, 0, 0, 0]) and abs(cover(ten, 7, 0) - 1) < 1e-9
    assert cover(ten, 4, 0) < cover(one, 4, 0)
    thirteen = c["FX-CDK-013"]["0"]
    assert all(grey(p) in (0.0, 1.0) or abs(grey(p)) < 1e-12 or abs(grey(p) - 1) < 1e-9
               for p in thirteen if p[3] > 0)
    fourteen, four = c["FX-CDK-014"]["0"], c["FX-CDK-003"]["0"]
    assert abs(grey(fourteen[at(6, 1)]) - (1 - grey(four[at(6, 1)]))) < 1e-9
    fifteen = c["FX-CDK-015"]["0"]
    assert abs(grey(fifteen[at(11, 0)])) < 1e-9 and abs(grey(fifteen[at(7, 0)]) - 1) < 1e-9
    sixteen = c["FX-CDK-016"]
    assert like(sixteen["0"], one)
    assert like(sixteen["2"], render(case(partial_b_in_white=177.5), 0))
    assert like(sixteen["4"], render(case(partial_b_in_white=100), 0))
    seventeen = c["FX-CDK-017"]
    assert like(seventeen["2"], render(case("matte", matte_gamma=2), 0))
    nine, moved = c["FX-CDK-009"]["0"], c["FX-CDK-018"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == nine[at(0, y):at(W - 3, y)]
    for fx, frames in c.items():
        for f in frames.values():
            for p in f:
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
                assert -1e-12 <= p[3] <= 1 + 1e-12, fx
    print("checked")


if __name__ == "__main__":
    main()
