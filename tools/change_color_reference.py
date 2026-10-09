"""Change Color, worked a second way.

`core.change_color`, modelled on After Effects' Change Color: one range of colours, picked by a
centre colour and a tolerance, is turned round the colour wheel, made lighter or darker, and
made stronger or greyer, leaving the rest of the picture alone. It is a separate effect from
Change to Color (D-197), which turns one colour into another; this one moves a range by amounts.
Nothing is ported; Adobe does not publish its method, so the rule below is ours. Document 21 is
the rule in words; this file is the reference for the numbers document 25 pins against it.

The rule. Settings: `view`, "corrected" or "mask", "corrected" when added; `hue_transform`,
-3600 to 3600 degrees, 0; `lightness_transform` and `saturation_transform`, -100 to 100, 0;
`color_to_change`, `#rrggbb`, "#ff0000"; `tolerance` and `softness`, 0 to 100, 15 and 0;
`match_colors`, "rgb", "hue" or "chroma", "hue"; `invert_mask`, "off" or "on", "off". Words
are exact.

- At a pixel with covering a > 0, e its straight colour through the sRGB curve held inside 0
  to 1, and c the colour to change encoded (8-bit over 255). Its distance d from c:
  "rgb", the straight-line distance between e and c divided by the square root of 3, so black
  to white is 1; "hue", the angle between their hues (HLS, as Change to Color's) over 180, so
  opposite hues are 1, and 1 when either is a grey (no hue); "chroma", the straight-line
  distance between their two colour-difference values, Cb = (B - Y) / 1.8556 and
  Cr = (R - Y) / 1.5748 with Y = 0.2126 R + 0.7152 G + 0.0722 B (the HD video weights), so a
  colour and a lighter or darker copy of it are near only as far as their colour signals are.
- The mask k, with t = tolerance / 100 and s = softness / 100: 1 when d <= t; 0 when s is 0 or
  d >= t + s; otherwise 1 - (d - t) / s, fading over s beyond the tolerance. With invert_mask
  "on", k becomes 1 - k.
- View "mask": the pixel becomes the grey k (encoded), at its own covering.
- View "corrected": with (h, l, s) the pixel's HLS (h 0 for a grey), h' = h + hue_transform;
  with p = lightness_transform / 100, l' = l + (1 - l) p when p >= 0, l (1 + p) when below; s'
  the same from s with saturation_transform. The colour of (h', l', s'), rgb, and the output
  e + k (rgb - e), held inside 0 to 1, back to linear at the pixel's covering (document 21's
  shared colour rule). A pixel with k = 0 is left exactly as it is; when all three transforms
  are 0 the layer is left exactly as it is.

**This file never runs the build's code path.** It works in double precision on lists,
straight from the drawing's 8-bit values, with Python's own colorsys for HLS, where the build
works on its single-precision buffers. Where softness is 0 it asserts that no pixel's distance
lies within a millionth of the tolerance, so rounding can't put a pixel on the other side.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py): pure colours, greys,
a skin tone, orange and three warm tones. The projects go into `Fixtures/change_color`, the
expected frames into `Fixtures/change_color/expected_change_color.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/change_color_reference.py
"""

import colorsys
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "change_color"
NUMBERS = {"hue_transform": (-3600, 3600), "lightness_transform": (-100, 100),
           "saturation_transform": (-100, 100), "tolerance": (0, 100), "softness": (0, 100)}


# --- the rule -------------------------------------------------------------------------------

def hex_colour(h):
    return [int(h[i:i + 2], 16) / 255 for i in (1, 3, 5)]


def grey(e):
    return e[0] == e[1] == e[2]


def cbcr(e):
    y = 0.2126 * e[0] + 0.7152 * e[1] + 0.0722 * e[2]
    return (e[2] - y) / 1.8556, (e[0] - y) / 1.5748


def distance(e, c, match):
    if match == "rgb":
        return math.dist(e, c) / math.sqrt(3)
    if match == "chroma":
        return math.dist(cbcr(e), cbcr(c))
    if grey(e) or grey(c):
        return 1.0
    d = abs(colorsys.rgb_to_hls(*e)[0] - colorsys.rgb_to_hls(*c)[0]) * 360
    return min(d, 360 - d) / 180


def push(v, p):
    return v + (1 - v) * p if p >= 0 else v * (1 + p)


def change(px, s):
    if s["view"] == "corrected" and (s["hue_transform"], s["lightness_transform"],
                                     s["saturation_transform"]) == (0, 0, 0):
        return [R.working(p) for p in px]
    c = hex_colour(s["color_to_change"])
    t, w = s["tolerance"] / 100, s["softness"] / 100
    out = []
    for p in px:
        if p[3] == 0:
            out.append(R.working(p))
            continue
        e = B.encoded(p)
        d = distance(e, c, s["match_colors"])
        if w == 0:
            assert abs(d - t) > 1e-6, "a pixel's distance lies on the tolerance"
        k = 1.0 if d <= t else 0.0 if w == 0 or d >= t + w else 1 - (d - t) / w
        if s["invert_mask"] == "on":
            k = 1 - k
        if s["view"] == "mask":
            out.append(B.back([k] * 3, p))
            continue
        if k == 0:
            out.append(R.working(p))
            continue
        h, l, sat = colorsys.rgb_to_hls(*e)
        h = (h * 360 + s["hue_transform"]) % 360 / 360
        l = push(l, s["lightness_transform"] / 100)
        sat = push(sat, s["saturation_transform"] / 100)
        rgb = colorsys.hls_to_rgb(h, l, sat)
        out.append(B.back([v + k * (u - v) for v, u in zip(e, rgb)], p))
    return out


# --- the cases ------------------------------------------------------------------------------

def case(view="corrected", hue=0, light=0, sat=0, colour="#ff0000", tolerance=15, softness=0,
         match="hue", invert="off", shift=0):
    return {"view": view, "hue_transform": hue, "lightness_transform": light,
            "saturation_transform": sat, "color_to_change": colour, "tolerance": tolerance,
            "softness": softness, "match_colors": match, "invert_mask": invert, "shift": shift}


def render(c, frame_no):
    s = dict(c)
    for k, (lo, hi) in NUMBERS.items():
        s[k] = min(hi, max(lo, value_at(c[k], frame_no)))
    return R.frame(change(B.pixels(), s), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    params = {k: setting_json(c[k]) if k in NUMBERS else c[k] for k in c if k != "shift"}
    return B.project_json(fx, "core.change_color", params, c["shift"])


SOFT = {"hue": 120, "softness": 8}

CASES = {
    "FX-CHCOLOR-001": ("The settings as they start: every transform 0, so the drawing, "
                       "untouched.", case(), [0]),
    "FX-CHCOLOR-002": ("Hue transform 120, matching red by hue, tolerance 15: the red column "
                       "turns green at every darkness, and the skin column, whose hue is within "
                       "15 per cent of red's, turns too; everything else stays.",
                       case(hue=120), [0]),
    "FX-CHCOLOR-003": ("The same with softness 8: the orange and the warm shadow and midtone, "
                       "just past the tolerance, turn part of the way; the warm highlight, "
                       "further, stays.", case(**SOFT), [0]),
    "FX-CHCOLOR-004": ("FX-CHCOLOR-003's settings, view mask: the mask as greys, white where "
                       "the colour is changed fully, black where it is left, grey in between.",
                       case(view="mask", **SOFT), [0]),
    "FX-CHCOLOR-005": ("FX-CHCOLOR-003's settings, invert on: everything but the reds and skin "
                       "turns round the wheel; the greys, having no hue, stay grey.",
                       case(invert="on", **SOFT), [0]),
    "FX-CHCOLOR-006": ("FX-CHCOLOR-004 with invert on: the mask turned round.",
                       case(view="mask", invert="on", **SOFT), [0]),
    "FX-CHCOLOR-007": ("Lightness -50: the reds and skin half way to black.",
                       case(light=-50), [0]),
    "FX-CHCOLOR-008": ("Lightness 50: the reds and skin half way to white.",
                       case(light=50), [0]),
    "FX-CHCOLOR-009": ("Saturation -100: the reds and skin turn grey at their own lightness.",
                       case(sat=-100), [0]),
    "FX-CHCOLOR-010": ("Saturation 60, softness 8: the reds stay pure, the skin and the warm "
                       "tones grow stronger.", case(sat=60, softness=8), [0]),
    "FX-CHCOLOR-011": ("Matching by RGB, colour #f6d6be (the skin), tolerance 20, softness 10, "
                       "hue 180 and lightness -30: the skin and the colours close to it in all "
                       "three channels, the warm highlight and white among them, are changed; "
                       "red, far in RGB, is not.",
                       case(hue=180, light=-30, colour="#f6d6be", tolerance=20, softness=10,
                            match="rgb"), [0]),
    "FX-CHCOLOR-012": ("Matching by chroma, colour #00ff00, tolerance 30, softness 15, hue "
                       "180: the bright greens are changed, the darker greens, whose colour "
                       "signal is weaker, less, and cyan, near in hue but far in chroma, not "
                       "at all.", case(hue=180, colour="#00ff00", tolerance=30, softness=15,
                                       match="chroma"), [0]),
    "FX-CHCOLOR-013": ("Hue transform keyed from 0 at frame 0 to 240 at frame 4, linear, "
                       "softness 8: frame 0 untouched, frame 2 FX-CHCOLOR-003, frame 4 the "
                       "reds turned blue.",
                       case(hue=keyed((0, 0), (4, 240)), softness=8), [0, 2, 4]),
    "FX-CHCOLOR-014": ("FX-CHCOLOR-003 moved three pixels right: the same, moved.",
                       case(shift=3, **SOFT), [0, 3]),
}

INVALID = {
    "FX-CHCOLOR-015": ("Hue transform 3601, above 3600.", case(hue=3601)),
    "FX-CHCOLOR-016": ("Lightness transform 101, above 100.", case(light=101)),
    "FX-CHCOLOR-017": ("Saturation transform -101, below -100.", case(sat=-101)),
    "FX-CHCOLOR-018": ("Tolerance 101, above 100.", case(hue=30, tolerance=101)),
    "FX-CHCOLOR-019": ("Softness -1, below 0.", case(hue=30, softness=-1)),
    "FX-CHCOLOR-020": ("Color to change \"#ff00\", which is not #rrggbb.",
                       case(hue=30, colour="#ff00")),
    "FX-CHCOLOR-021": ("Match colors \"lab\", which is not a choice.", case(hue=30, match="lab")),
    "FX-CHCOLOR-022": ("Match colors \"Hue\": the word is exact, so a capital is not it.",
                       case(hue=30, match="Hue")),
    "FX-CHCOLOR-023": ("View \"matte\", which is not a choice.", case(hue=30, view="matte")),
    "FX-CHCOLOR-024": ("Invert mask \"yes\", which is not a choice.",
                       case(hue=30, invert="yes")),
}


def main():
    expected = B.write_cases(OUT, "change_color", CASES, INVALID, render, plain, file_json)
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    px = B.pixels()
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731
    enc = lambda p: [S.linear_to_srgb(u / p[3]) for u in p[:3]]  # noqa: E731
    col = lambda f, x: [f[at(x, y)] for y in range(H)]  # noqa: E731

    assert c["FX-CHCOLOR-001"]["0"] == drawn
    two, three = c["FX-CHCOLOR-002"]["0"], c["FX-CHCOLOR-003"]["0"]
    red = hex_colour("#ff0000")
    # Hue distances of the warm columns from red, worked by hand from their 8-bit values: skin
    # within 0.15, orange, shadow and midtone within 0.23, the warm highlight beyond.
    hue = lambda x: distance(B.encoded(px[at(x, 0)]), red, "hue")  # noqa: E731
    assert all(0.15 < distance(B.encoded(px[at(x, y)]), red, "hue") < 0.23
               for x in (11, 12, 13) for y in range(H))
    assert all(distance(B.encoded(px[at(14, y)]), red, "hue") > 0.23 for y in range(H))
    assert hue(10) < 0.15 and hue(8) == 0 and hue(3) == 1
    for y in range(H):
        e = enc(two[at(8, y)])  # red turned green: green channel the strongest, as bright
        assert e[1] > 0 and e[0] < 1e-9 and e[2] < 1e-9
    changed = {x for x in range(W) for y in range(H) if two[at(x, y)] != drawn[at(x, y)]}
    assert changed == {8, 10}
    changed = {x for x in range(W) for y in range(H) if three[at(x, y)] != drawn[at(x, y)]}
    assert changed == {8, 10, 11, 12, 13}
    # The mask: white on red, black on the greys and blue, grey on the orange; inverted, turned.
    four, six = c["FX-CHCOLOR-004"]["0"], c["FX-CHCOLOR-006"]["0"]
    assert near(enc(four[at(8, 3)]), [1] * 3) and near(enc(four[at(3, 3)]), [0] * 3)
    k = enc(four[at(11, 0)])[0]
    assert 0 < k < 1 and near(enc(six[at(11, 0)]), [1 - k] * 3)
    assert all(near(enc(four[i]), [1 - enc(six[i])[0]] * 3)
               for i in range(W * H) if px[i][3] > 0)
    # Inverted: the red column is left; the greys, with no hue to turn, stay grey.
    five = c["FX-CHCOLOR-005"]["0"]
    assert col(five, 8) == col(drawn, 8) and all(grey(enc(p)) for p in col(five, 3))
    # Lightness and saturation move toward their ends.
    for y in range(H):
        e0 = B.encoded(px[at(8, y)])
        l0 = colorsys.rgb_to_hls(*e0)[1]
        assert abs(colorsys.rgb_to_hls(*enc(c["FX-CHCOLOR-007"]["0"][at(8, y)]))[1]
                   - l0 / 2) < 1e-9
        assert abs(colorsys.rgb_to_hls(*enc(c["FX-CHCOLOR-008"]["0"][at(8, y)]))[1]
                   - (l0 + (1 - l0) / 2)) < 1e-9
        assert grey([round(v, 9) for v in enc(c["FX-CHCOLOR-009"]["0"][at(8, y)])])
    ten = c["FX-CHCOLOR-010"]["0"]
    assert near(enc(ten[at(8, 0)]), [1, 0, 0])  # pure red is already at full strength
    s0 = colorsys.rgb_to_hls(*B.encoded(px[at(10, 0)]))[2]
    assert colorsys.rgb_to_hls(*enc(ten[at(10, 0)]))[2] > s0
    # RGB matching takes the skin and white, not red; chroma takes bright green, not cyan.
    eleven = c["FX-CHCOLOR-011"]["0"]
    assert eleven[at(10, 0)] != drawn[at(10, 0)] and eleven[at(2, 0)] != drawn[at(2, 0)]
    assert col(eleven, 8) == col(drawn, 8)
    twelve = c["FX-CHCOLOR-012"]["0"]
    assert twelve[at(6, 0)] != drawn[at(6, 0)] and col(twelve, 5) == col(drawn, 5)
    thirteen = c["FX-CHCOLOR-013"]
    assert thirteen["0"] == drawn and like(thirteen["2"], three)
    e = enc(thirteen["4"][at(8, 0)])
    assert near(e, [0, 0, 1])
    moved = c["FX-CHCOLOR-014"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == three[at(0, y):at(W - 3, y)]
    for fx, frames in c.items():
        for f in frames.values():
            for i, p in enumerate(f):
                assert p[3] == drawn[i][3] or fx == "FX-CHCOLOR-014"
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
    print("checked")


if __name__ == "__main__":
    main()
