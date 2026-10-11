"""Color Range, worked a second way.

`core.color_range`, modelled on After Effects' Color Range (Keying): it keys out the colours
inside a box chosen in Lab, YUV or RGB, with a fuzziness that softens the box's edge. Adobe
describes the effect ("creates transparency by keying out a specified range of colors in the
Lab, YUV, or RGB color space ... Fuzziness: softens the edges ... L, Y, R sliders control the
first component of the specified color space; the a, U, G sliders control the second component;
and the b, V, B sliders control the third component") and names its controls, but publishes no
formula and no defaults that we found. The rule and the settings when added are ours (document
21, D-500); where they differ from Adobe's own maths is a logged gap. Its eyedroppers and matte
thumbnail are After Effects' window, not settings, and are not here.

The settings. `color_space`, "lab" (when added), "yuv" or "rgb"; the word is exact.
`fuzziness`, 0 to 255, 0 when added. Then the box, `min_1`, `max_1`, `min_2`, `max_2`, `min_3`,
`max_3`, the low and high end of each of the space's three components, 0 to 255, all 0 when
added, which in Lab is a box no colour falls in, so the effect keys nothing until set. A min
above its max is allowed and makes a box nothing is inside. Every number is keyable.

The rule. At a pixel with covering a > 0, e = (eR, eG, eB) its straight colour through the sRGB
curve held inside 0 to 1. Its three components v, each on a 0 to 255 scale:

- "rgb": 255 eR, 255 eG, 255 eB.
- "yuv": the full-range BT.601 YCbCr of JPEG, Y = 255 (0.299 eR + 0.587 eG + 0.114 eB),
  U = 128 + 255 (-0.168736 eR - 0.331264 eG + 0.5 eB), V = 128 + 255 (0.5 eR - 0.418688 eG -
  0.081312 eB).
- "lab": CIE L*a*b* of the linear colour (e back through the sRGB curve), D65 white, by Shadow/
  Highlight's conversion (tools/shadow_highlight_reference.py, D-400), as L = 255 L* / 100,
  a = 128 + a*, b = 128 + b*; not held inside 0 to 255.

d is how far v lies outside the box: for each component max(min - v, v - max, 0), and d the
largest of the three. The pixel's share kept is min(1, d / fuzziness), and with fuzziness 0 it
is 0 inside the box (d = 0) and 1 outside. Every channel of the pixel, colour and covering, is
multiplied by it, so the colour is kept and only the covering falls. A pixel with a = 0 is left
as it is.

**This file never runs the build's code path.** It works in double precision on lists,
straight from the drawing's 8-bit values, where the build works on its single-precision
buffers. The rule is continuous in the colour except at the box's edge with fuzziness 0; every
case with fuzziness 0 is checked that no pixel lies within a hair of an edge it could cross.

The drawing is Color Difference Key's (tools/color_difference_key_reference.py): black, white,
grey, a green screen, a blue screen, skin, pure blue, cyan, pure green, yellow, red, magenta,
a violet, a pale blue, and the blue screen at half covering, each row darker by 8%. The
projects go into `Fixtures/color_range`, the expected frames into
`Fixtures/color_range/expected_color_range.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/color_range_reference.py
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import color_difference_key_reference as D  # noqa: E402,F401  (draws the shared picture)
import shadow_highlight_reference as SH  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "color_range"
BOX = ("min_1", "max_1", "min_2", "max_2", "min_3", "max_3")
NUMBERS = ("fuzziness",) + BOX
SPACES = ("lab", "yuv", "rgb")


# --- the rule -------------------------------------------------------------------------------

def components(e, space):
    r, g, b = e
    if space == "rgb":
        return [255 * r, 255 * g, 255 * b]
    if space == "yuv":
        return [255 * (0.299 * r + 0.587 * g + 0.114 * b),
                128 + 255 * (-0.168736 * r - 0.331264 * g + 0.5 * b),
                128 + 255 * (0.5 * r - 0.418688 * g - 0.081312 * b)]
    l, p, q = SH.lab([S.srgb_to_linear(v) for v in e])
    return [255 * l, 128 + 128 * p, 128 + 128 * q]


def outside(v, k):
    return max(max(lo - x, x - hi, 0.0) for x, (lo, hi) in zip(v, k["box"]))


def kept(e, k):
    d = outside(components(e, k["space"]), k)
    if k["fuzziness"] == 0:
        return 0.0 if d == 0 else 1.0
    return min(1.0, d / k["fuzziness"])


def key(px, k):
    out = []
    for p in px:
        w = R.working(p)
        if p[3] == 0:
            out.append(w)
            continue
        m = kept(B.encoded(p), k)
        out.append([v * m for v in w])
    return out


# --- the cases ------------------------------------------------------------------------------

def case(color_space="lab", shift=0, **numbers):
    c = {"color_space": color_space, "shift": shift}
    for n in NUMBERS:
        c[n] = 0
    for n, v in numbers.items():
        assert n in NUMBERS, n
        c[n] = v
    return c


def settings(c, frame_no):
    v = {n: min(255, max(0, value_at(c[n], frame_no))) for n in NUMBERS}
    return {"space": c["color_space"], "fuzziness": v["fuzziness"],
            "box": [(v[f"min_{i}"], v[f"max_{i}"]) for i in (1, 2, 3)]}


def render(c, frame_no):
    return R.frame(key(B.pixels(), settings(c, frame_no)), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    params = {"color_space": c["color_space"]}
    params.update({n: setting_json(c[n]) for n in NUMBERS})
    return B.project_json(fx, "core.color_range", params, c["shift"])


def box(lo1, hi1, lo2, hi2, lo3, hi3):
    return dict(min_1=lo1, max_1=hi1, min_2=lo2, max_2=hi2, min_3=lo3, max_3=hi3)


# The green screen (40, 180, 60) is about L 165, a 68, b 177 here (L* 65, a* -60, b* 49).
GREEN_LAB = box(120, 200, 40, 100, 150, 220)
# The blue screen (30, 70, 200) is about Y 71, U 199, V 103.
BLUE_YUV = box(20, 120, 170, 230, 80, 125)
# Green in RGB: little red, much green, little blue.
GREEN_RGB = box(0, 90, 140, 255, 0, 110)

CASES = {
    "FX-CRANGE-001": ("The settings as they start: Lab, every end of the box 0, fuzziness 0. "
                      "No colour has a* or b* as low as -128, so nothing is keyed and the layer "
                      "is as it was.", case(), [0]),
    "FX-CRANGE-002": ("Lab, a box about the green screen (L 120 to 200, a 40 to 100, b 150 to "
                      "220), fuzziness 0: the green screen's top four rows and pure green's "
                      "middle rows, inside the same box, lose all their covering; everything "
                      "else keeps all of it.", case(**GREEN_LAB), [0]),
    "FX-CRANGE-003": ("The same box with fuzziness 30: the screen still all gone; colours just "
                      "outside the box keep a share of their covering, d / 30.",
                      case(fuzziness=30, **GREEN_LAB), [0]),
    "FX-CRANGE-004": ("YUV, a box about the blue screen (Y 20 to 120, U 170 to 230, V 80 to "
                      "125), fuzziness 20: the blue screen falls, the half-covered one too; "
                      "pure blue (U about 255) keeps all its covering and its darker rows fall; "
                      "the violet keeps a share.", case("yuv", fuzziness=20,
                                                                   **BLUE_YUV), [0]),
    "FX-CRANGE-005": ("RGB, a box about green (R 0 to 90, G 140 to 255, B 0 to 110), "
                      "fuzziness 40: pure green and the green screen fall, cyan and yellow, "
                      "far outside in blue or red, keep all their covering.",
                      case("rgb", fuzziness=40, **GREEN_RGB), [0]),
    "FX-CRANGE-006": ("RGB, the whole cube (every min 0, every max 255), fuzziness 1: every "
                      "pixel is inside, so the whole layer is keyed out.",
                      case("rgb", fuzziness=1, **box(0, 255, 0, 255, 0, 255)), [0]),
    "FX-CRANGE-007": ("Lab with the whole range of L and b but a* only from -8 to 40 (a 120 to "
                      "168), fuzziness 2: the greys, black and white (a* 0), skin, the blue "
                      "screen and the pale blue fall; the greens (a* below -8) and pure red, "
                      "magenta and pure blue (above 40) keep their covering.",
                      case(fuzziness=2, **box(0, 255, 120, 168, 0, 255)), [0]),
    "FX-CRANGE-008": ("RGB, a minimum above its maximum (red from 200 down to 100): no colour "
                      "is inside, and with fuzziness 60 a pixel keeps the share of its distance "
                      "past the nearer end, so mid reds lose some covering.",
                      case("rgb", fuzziness=60, **box(200, 100, 0, 255, 0, 255)), [0]),
    "FX-CRANGE-009": ("Fuzziness 255 about the green screen in Lab: the softest edge; colours "
                      "near green lose much of their covering, the far ones little.",
                      case(fuzziness=255, **GREEN_LAB), [0]),
    "FX-CRANGE-010": ("Fuzziness keyed from 0 at frame 0 to 80 at frame 4, linear, about the "
                      "green screen in Lab.", case(fuzziness=keyed((0, 0), (4, 80)), **GREEN_LAB),
                      [0, 2, 4]),
    "FX-CRANGE-011": ("RGB, the top of green keyed from 255 at frame 0 to 150 at frame 4, the "
                      "rest of FX-CRANGE-005: the box shrinks and fewer greens fall.",
                      case("rgb", fuzziness=40, **{**GREEN_RGB, "max_2": keyed((0, 255),
                                                                              (4, 150))}),
                      [0, 2, 4]),
    "FX-CRANGE-012": ("FX-CRANGE-003 moved three pixels right: the same, moved.",
                      case(shift=3, fuzziness=30, **GREEN_LAB), [0, 3]),
}

INVALID = {
    "FX-CRANGE-013": ("Fuzziness 256, above 255.", case(fuzziness=256)),
    "FX-CRANGE-014": ("Fuzziness -1, below 0.", case(fuzziness=-1)),
    "FX-CRANGE-015": ("Min (L, Y, R) 256, above 255.", case(min_1=256)),
    "FX-CRANGE-016": ("Max (b, V, B) -1, below 0.", case(max_3=-1)),
    "FX-CRANGE-017": ("Colour space \"Lab\": the word is exact, so a capital is not it.",
                      case("Lab")),
    "FX-CRANGE-018": ("Colour space \"hsl\": not one of Lab, YUV and RGB.", case("hsl")),
}


def main():
    step_clear()
    expected = B.write_cases(OUT, "color_range", CASES, INVALID, render, plain, file_json)
    check(expected)


def step_clear():
    """With fuzziness 0 a pixel is inside or out; none may lie within a hair of an edge."""
    for fx, (_, c, frames) in CASES.items():
        for f in frames:
            k = settings(c, f)
            if k["fuzziness"] != 0:
                continue
            for p in B.pixels():
                if not p[3]:
                    continue
                v = components(B.encoded(p), k["space"])
                d = outside(v, k)
                if d == 0:
                    assert all(min(x - lo, hi - x) > 1e-2 for x, (lo, hi) in zip(v, k["box"])), (
                        fx, p, v)
                else:
                    assert d > 1e-2, (fx, p, v)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    px = B.pixels()
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731
    cover = lambda f, x, y: f[at(x, y)][3]  # noqa: E731
    full = lambda f, x, y: abs(cover(f, x, y) - px[at(x, y)][3] / 255) < 1e-9  # noqa: E731

    assert like(c["FX-CRANGE-001"]["0"], drawn)
    two = c["FX-CRANGE-002"]["0"]
    for y in range(4):
        assert near(two[at(4, y)], [0, 0, 0, 0]), y  # the green screen, its darker rows
        assert near(two[at(9, y + 2)], [0, 0, 0, 0]), y  # pure green's middle rows
    for x in (1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15):
        assert full(two, x, 0), x
    three = c["FX-CRANGE-003"]["0"]
    assert near(three[at(4, 0)], [0, 0, 0, 0])
    assert any(0.01 < cover(three, x, y) < 0.99 for x in range(1, 16) for y in range(H))
    four = c["FX-CRANGE-004"]["0"]
    assert near(four[at(5, 0)], [0, 0, 0, 0]) and near(four[at(15, 0)], [0, 0, 0, 0])
    assert 0.01 < cover(four, 13, 0) < 0.99 and full(four, 4, 0) and full(four, 6, 0)
    assert full(four, 7, 0) and near(four[at(7, 3)], [0, 0, 0, 0])
    five = c["FX-CRANGE-005"]["0"]
    assert near(five[at(9, 0)], [0, 0, 0, 0]) and near(five[at(4, 0)], [0, 0, 0, 0])
    assert full(five, 8, 0) and full(five, 10, 0)
    assert all(near(p, [0, 0, 0, 0]) for p in c["FX-CRANGE-006"]["0"])
    seven = c["FX-CRANGE-007"]["0"]
    for x in (1, 2, 3, 5, 6, 14):
        assert near(seven[at(x, 0)], [0, 0, 0, 0]), x
    for x in (4, 7, 9, 11, 12):
        assert full(seven, x, 0), x
    eight = c["FX-CRANGE-008"]["0"]
    assert full(eight, 1, 0) and full(eight, 11, 0)  # red 0 and 255: 100 or more outside
    assert all(cover(eight, x, y) > 1e-9 for x in range(1, 16) for y in range(H))
    assert any(cover(eight, x, y) < px[at(x, y)][3] / 255 - 0.01
               for x in range(1, 16) for y in range(H))
    nine = c["FX-CRANGE-009"]["0"]
    for x in range(1, 16):  # softer than FX-CRANGE-003 everywhere
        for y in range(H):
            assert cover(nine, x, y) <= cover(three, x, y) + 1e-12
    ten = c["FX-CRANGE-010"]
    assert like(ten["0"], two)
    assert like(ten["2"], render(case(fuzziness=40, **GREEN_LAB), 0))
    assert like(ten["4"], render(case(fuzziness=80, **GREEN_LAB), 0))
    eleven = c["FX-CRANGE-011"]
    assert like(eleven["0"], five)
    assert like(eleven["2"], render(case("rgb", fuzziness=40, **{**GREEN_RGB, "max_2": 202.5}),
                                    0))
    assert full(eleven["4"], 9, 0) and 0.5 < cover(eleven["4"], 4, 0) < 0.99  # G above 150
    # The colour is kept wherever any covering is left.
    for fx, frames in c.items():
        for f in frames.values():
            for i, p in enumerate(f):
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
                assert -1e-12 <= p[3] <= 1 + 1e-12, fx
    for i, p in enumerate(three):
        q = drawn[i]
        if p[3] > 1e-6:
            assert near([u / p[3] for u in p[:3]], [u / q[3] for u in q[:3]], 1e-7)
    moved = c["FX-CRANGE-012"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == three[at(0, y):at(W - 3, y)]
    print("checked")


if __name__ == "__main__":
    main()
