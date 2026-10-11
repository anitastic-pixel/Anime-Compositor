"""Advanced Spill Suppressor, worked a second way.

`core.advanced_spill_suppressor`, modelled on After Effects' Advanced Spill Suppressor (Keying):
it takes the green or blue screen's colour back out of a keyed subject's edges, hair and skin.
Adobe says what it is for and names its controls (the Standard method "is simpler,
automatically detects the dominant key color, and requires less user input"; the Ultra method
"is based on the spill suppression in the Ultra Key effect in Premiere"), but publishes no
formula, so the rule below is ours (document 21, D-457), in the same way as every effect whose
maths Adobe keeps to itself. Nothing is ported.

The settings. `method`, "standard" or "ultra", "standard" when added; the word is exact.
`suppression`, 0 to 100 per cent, how much of the spill is taken out, 100 when added. Then the
Ultra settings, read only when the method is "ultra": `key_color`, `#rrggbb`, the screen,
`#00ff00` when added; `tolerance`, 0 to 100, how far from the key's hue a colour may be and
still count as spill, 50; `desaturate`, 0 to 100, how grey a spilled pixel turns as its spill is
taken out, 25; `spill_range`, 0 to 100, how low the screen channel is cut, 50; `spill_color_
correction`, 0 to 100, how much of what is taken out is given back to the other two channels,
50; `luma_correction`, 0 to 100, how much of the lost brightness is put back, 50. Every number
is keyable; the key colour and method are not.

The rule. At a pixel with covering a > 0, e = (eR, eG, eB) its straight colour through the sRGB
curve held inside 0 to 1. S is the suppression over 100.

Standard: for green and for blue in turn, the excess is how far that channel stands above the
larger of the other two, max(0, e_s - max(e_p, e_q)); at most one of the two can have one. The
channel with the excess drops by S times it. Nothing else changes. So a colour where green or
blue leads goes down to the next channel's level (a green-tinted skin to its own red), and
yellow, cyan, the greys and every colour where red leads stay as they are. Because each pixel is
judged alone, a strongly green or blue object loses its lead too; Ultra with a key colour keeps
such objects when their hue is far from the screen's.

Ultra: the screen channel s is the key colour's largest channel (on a tie green before blue
before red), p and q the other two. The key's hue h_k and the pixel's hue h are HSV hues in
degrees; a grey has none. The hue weight w is 0 when either has none or the tolerance is 0;
otherwise, with d the distance round the wheel (0 to 180) and the window 1.8 times the tolerance
in degrees, w = max(0, 1 - d / window). With hi and lo the larger and smaller of e_p and e_q,
the limit is L = hi - R (hi - lo), R the spill range over 100: at 0 the screen channel is cut to
the larger other channel, at 50 to their mean, at 100 to the smaller. The spill taken out is
x = S w max(0, e_s - L). Then, in order:

- e_s drops by x, and e_p and e_q each rise by C x / 2 (C the spill colour correction over 100);
- desaturate: with f = x / e_s (the share of the screen channel taken out; 0 when e_s is 0) and
  Y the luma 0.2126 R + 0.7152 G + 0.0722 B of the colour so far, the colour moves toward Y by
  D f (D the desaturate over 100);
- luma correction: each channel rises by U (Y0 - Y1), U the luma correction over 100, Y0 the
  luma of e and Y1 the luma so far.

The result held inside 0 to 1, back to linear at the pixel's covering (document 21's shared
colour rule); the covering is never changed. A pixel with a = 0 is left as it is. With the
suppression 0, or Ultra with the tolerance 0, the layer is left exactly as it is.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works on its single-precision buffers. The rule
is continuous in the colour except where a hue is undefined (a grey, which has no excess
anyway), so rounding can't flip a pixel.

The drawing is this file's own, laid out as Broadcast Safe's (tools/broadcast_safe_reference.py,
each column one colour, each row darker by 8%): black, white, grey, a green screen, a blue
screen, a green-spilled skin, a blue-spilled skin, a green-spilled dark edge, yellow, cyan, pure
green, pure blue, a blue shirt, a yellow-green leaf, and the green-spilled skin at half covering.
The projects go into `Fixtures/advanced_spill_suppressor`, the expected frames into
`Fixtures/advanced_spill_suppressor/expected_advanced_spill_suppressor.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/advanced_spill_suppressor_reference.py
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "advanced_spill_suppressor"
NUMBERS = ("suppression", "tolerance", "desaturate", "spill_range", "spill_color_correction",
           "luma_correction")

COLOURS = [
    None,               # 0: empty
    (0, 0, 0),          # 1: black
    (255, 255, 255),    # 2: white
    (128, 128, 128),    # 3: grey
    (40, 180, 60),      # 4: a green screen
    (30, 70, 200),      # 5: a blue screen
    (200, 214, 160),    # 6: skin with green spill
    (214, 190, 228),    # 7: skin with blue spill
    (90, 122, 70),      # 8: a dark edge with green spill
    (255, 255, 0),      # 9: yellow
    (0, 255, 255),      # 10: cyan
    (0, 255, 0),        # 11: green
    (0, 0, 255),        # 12: blue
    (40, 60, 160),      # 13: a blue shirt
    (150, 200, 40),     # 14: a yellow-green leaf
    (200, 214, 160),    # 15: skin with green spill at half covering
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

def luma(e):
    return 0.2126 * e[0] + 0.7152 * e[1] + 0.0722 * e[2]


def hue(e):
    """HSV hue in degrees, None for a grey."""
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


def hex_colour(text):
    return [int(text[i:i + 2], 16) / 255 for i in (1, 3, 5)]


def screen(key):
    """The key's largest channel, green before blue before red on a tie, and the other two."""
    s = max((1, 2, 0), key=lambda i: (key[i], -(1, 2, 0).index(i)))
    return s, [i for i in range(3) if i != s]


def standard(e, s_share):
    o = list(e)
    for s in (1, 2):
        p, q = [i for i in range(3) if i != s]
        excess = e[s] - max(e[p], e[q])
        if excess > 0:
            o[s] = e[s] - s_share * excess
    return o


def ultra(e, k):
    s, (p, q) = screen(k["key"])
    hk, h = hue(k["key"]), hue(e)
    if hk is None or h is None or k["tolerance"] == 0:
        return list(e)
    d = abs((h - hk + 180) % 360 - 180)
    w = max(0.0, 1 - d / (1.8 * k["tolerance"]))
    hi, lo = max(e[p], e[q]), min(e[p], e[q])
    limit = hi - k["spill_range"] / 100 * (hi - lo)
    x = k["suppression"] / 100 * w * max(0.0, e[s] - limit)
    o = list(e)
    o[s] -= x
    give = k["spill_color_correction"] / 100 * x / 2
    o[p] += give
    o[q] += give
    f = x / e[s] if e[s] > 0 else 0.0
    y = luma(o)
    t = k["desaturate"] / 100 * f
    o = [v + (y - v) * t for v in o]
    lift = k["luma_correction"] / 100 * (luma(e) - luma(o))
    return [v + lift for v in o]


def spill(px, k):
    if k["suppression"] == 0 or (k["method"] == "ultra" and k["tolerance"] == 0):
        return [R.working(p) for p in px]
    if k["method"] == "standard":
        f = lambda e: standard(e, k["suppression"] / 100)  # noqa: E731
    else:
        f = lambda e: ultra(e, k)  # noqa: E731
    return [R.working(p) if p[3] == 0 else B.back(f(B.encoded(p)), p) for p in px]


# --- the cases ------------------------------------------------------------------------------

def case(method="standard", suppression=100, key_color="#00ff00", tolerance=50, desaturate=25,
         spill_range=50, spill_color_correction=50, luma_correction=50, shift=0):
    return {"method": method, "suppression": suppression, "key_color": key_color,
            "tolerance": tolerance, "desaturate": desaturate, "spill_range": spill_range,
            "spill_color_correction": spill_color_correction, "luma_correction": luma_correction,
            "shift": shift}


def settings(c, frame_no):
    k = {n: min(100, max(0, value_at(c[n], frame_no))) for n in NUMBERS}
    k["method"] = c["method"]
    k["key"] = hex_colour(c["key_color"])
    return k


def render(c, frame_no):
    return R.frame(spill(B.pixels(), settings(c, frame_no)), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    params = {"method": c["method"], "key_color": c["key_color"]}
    params.update({n: setting_json(c[n]) for n in NUMBERS})
    return B.project_json(fx, "core.advanced_spill_suppressor", params, c["shift"])


U = "ultra"
CASES = {
    "FX-SPILL-001": ("The settings as they start: Standard at 100%. The green screen, the "
                     "green-spilled skin and edge, pure green, the leaf, the blue screen, the "
                     "blue-spilled skin, pure blue and the shirt lose their leading green or "
                     "blue down to the next channel; black, white, grey, yellow and cyan are "
                     "untouched.", case(), [0]),
    "FX-SPILL-002": ("Standard, suppression 0: the drawing exactly as it is.",
                     case(suppression=0), [0]),
    "FX-SPILL-003": ("Standard, suppression 50: half of each lead taken out.",
                     case(suppression=50), [0]),
    "FX-SPILL-004": ("Ultra as it starts: green key, tolerance 50, desaturate 25, spill range "
                     "50, correction 50, luma 50. The greens cut toward the mean of red and "
                     "blue, half the cut given back to red and blue, a little greyer, half the "
                     "lost brightness put back; yellow and cyan, 60 degrees off, a third "
                     "counted; the blues, further than 90 degrees, untouched.",
                     case(U), [0]),
    "FX-SPILL-005": ("Ultra, blue key: the blue screen, blue skin, blue and the shirt "
                     "corrected; the greens untouched.", case(U, key_color="#0000ff"), [0]),
    "FX-SPILL-006": ("Ultra, tolerance 0: the drawing exactly as it is.",
                     case(U, tolerance=0), [0]),
    "FX-SPILL-007": ("Ultra, tolerance 100: the hue window reaches round to the opposite hue, "
                     "so yellow-greens and blue-greens count fully nearer the key.",
                     case(U, tolerance=100), [0]),
    "FX-SPILL-008": ("Ultra, spill range 0, nothing given back, no desaturate, no luma: green "
                     "cut to the larger other channel times the hue weight.",
                     case(U, spill_range=0, spill_color_correction=0, desaturate=0,
                          luma_correction=0), [0]),
    "FX-SPILL-009": ("Ultra, spill range 100, otherwise as FX-SPILL-008: green cut to the "
                     "smaller other channel, a harder cut.",
                     case(U, spill_range=100, spill_color_correction=0, desaturate=0,
                          luma_correction=0), [0]),
    "FX-SPILL-010": ("Ultra, spill colour correction 100, no desaturate, no luma: all that is "
                     "taken from green given back to red and blue in halves (pure green ends a "
                     "half magenta).",
                     case(U, spill_color_correction=100, desaturate=0, luma_correction=0), [0]),
    "FX-SPILL-011": ("Ultra, desaturate 100, no correction, no luma: a spilled pixel greyed by "
                     "the share of its green taken out; pure green, whose green all goes, "
                     "ends black.",
                     case(U, desaturate=100, spill_color_correction=0, luma_correction=0), [0]),
    "FX-SPILL-012": ("Ultra, luma correction 100, no correction, no desaturate: every changed "
                     "pixel keeps its brightness (until a channel reaches full).",
                     case(U, luma_correction=100, spill_color_correction=0, desaturate=0), [0]),
    "FX-SPILL-013": ("Ultra with a sampled screen green, #3cbe50: the window centred on that "
                     "hue instead of pure green's.", case(U, key_color="#3cbe50"), [0]),
    "FX-SPILL-014": ("Ultra with a red key: red is the screen channel, so warm colours near "
                     "red's hue (yellow, the skins, the leaf) lose some red; the greens, cyan "
                     "and blues are not touched.",
                     case(U, key_color="#ff0000"), [0]),
    "FX-SPILL-015": ("Ultra with a grey key: a grey has no hue, so nothing is spill; the "
                     "drawing as it is.", case(U, key_color="#808080"), [0]),
    "FX-SPILL-016": ("Standard, suppression keyed from 0 at frame 0 to 100 at frame 4, linear: "
                     "frame 0 untouched, frame 2 as 50, frame 4 as FX-SPILL-001.",
                     case(suppression=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-SPILL-017": ("Ultra, tolerance keyed from 20 at frame 0 to 80 at frame 4.",
                     case(U, tolerance=keyed((0, 20), (4, 80))), [0, 2, 4]),
    "FX-SPILL-018": ("FX-SPILL-004 moved three pixels right: the same, moved.",
                     case(U, shift=3), [0, 3]),
}

INVALID = {
    "FX-SPILL-019": ("Suppression 101, above 100.", case(suppression=101)),
    "FX-SPILL-020": ("Suppression -1, below 0.", case(suppression=-1)),
    "FX-SPILL-021": ("Tolerance 101, above 100.", case(U, tolerance=101)),
    "FX-SPILL-022": ("Desaturate -1, below 0.", case(U, desaturate=-1)),
    "FX-SPILL-023": ("Spill range 101, above 100.", case(U, spill_range=101)),
    "FX-SPILL-024": ("Spill colour correction 101, above 100.",
                     case(U, spill_color_correction=101)),
    "FX-SPILL-025": ("Luma correction -1, below 0.", case(U, luma_correction=-1)),
    "FX-SPILL-026": ("Method \"Standard\": the word is exact, so a capital is not it.",
                     case("Standard")),
    "FX-SPILL-027": ("Method \"auto\", which is not one.", case("auto")),
    "FX-SPILL-028": ("Key colour \"green\", not a #rrggbb colour.", case(U, key_color="green")),
}


def main():
    expected = B.write_cases(OUT, "advanced_spill_suppressor", CASES, INVALID, render, plain,
                             file_json)
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
    same_cols = lambda f, cols: all(near(f[at(x, y)], drawn[at(x, y)])  # noqa: E731
                                    for x in cols for y in range(H))

    one = c["FX-SPILL-001"]["0"]
    assert same_cols(one, (1, 2, 3, 9, 10))
    assert near(enc(one[at(6, 0)]), [200 / 255, 200 / 255, 160 / 255])  # skin green to red
    assert near(enc(one[at(7, 0)]), [214 / 255, 190 / 255, 214 / 255])  # skin blue to red
    assert near(enc(one[at(11, 0)]), [0, 0, 0])  # pure green to black
    assert near(enc(one[at(4, 0)]), [40 / 255, 60 / 255, 60 / 255])
    for fx in ("FX-SPILL-002", "FX-SPILL-006", "FX-SPILL-015"):
        assert c[fx]["0"] == drawn, fx
    half = enc(c["FX-SPILL-003"]["0"][at(6, 0)])
    assert near(half, [200 / 255, 207 / 255, 160 / 255])
    four = c["FX-SPILL-004"]["0"]
    assert same_cols(four, (1, 2, 3, 5, 7, 12, 13))  # blues far from green's hue
    assert not same_cols(four, (9,)) and not same_cols(four, (10,))  # 60 degrees: a third
    assert not same_cols(four, (4,)) and not same_cols(four, (6,))
    five = c["FX-SPILL-005"]["0"]
    assert same_cols(five, (4, 6, 8, 11, 14)) and not same_cols(five, (5,))
    eight, nine = enc(c["FX-SPILL-008"]["0"][at(11, 0)]), enc(c["FX-SPILL-009"]["0"][at(11, 0)])
    assert near(eight, [0, 0, 0]) and near(nine, [0, 0, 0])  # pure green, weight 1, cut to 0
    e6 = B.encoded(px[at(6, 0)])
    w6 = 1 - abs(hue(e6) - 120) / 90
    assert near(enc(c["FX-SPILL-008"]["0"][at(6, 0)]), [e6[0], e6[1] - w6 * (e6[1] - e6[0]), e6[2]])
    assert near(enc(c["FX-SPILL-009"]["0"][at(6, 0)]), [e6[0], e6[1] - w6 * (e6[1] - e6[2]), e6[2]])
    ten = enc(c["FX-SPILL-010"]["0"][at(11, 0)])
    assert near(ten, [0.5, 0, 0.5])  # pure green: 1 taken, 0.5 to each of red and blue
    eleven = enc(c["FX-SPILL-011"]["0"][at(11, 0)])
    assert near(eleven, [0, 0, 0])  # all its green gone, nothing else: grey already
    twelve = c["FX-SPILL-012"]["0"]
    for x in (4, 6, 8):
        e0 = B.encoded(px[at(x, 3)])
        assert abs(luma(enc(twelve[at(x, 3)])) - luma(e0)) < 1e-9
    fourteen = c["FX-SPILL-014"]["0"]
    assert same_cols(fourteen, (4, 5, 8, 10, 11, 12, 13))  # hues 97 degrees or more from red
    assert not same_cols(fourteen, (9,))  # yellow, 60 degrees from red, where red leads
    sixteen = c["FX-SPILL-016"]
    assert sixteen["0"] == drawn and like(sixteen["4"], one)
    assert like(sixteen["2"], c["FX-SPILL-003"]["0"])
    seventeen = c["FX-SPILL-017"]
    assert like(seventeen["2"], render(case(U, tolerance=50), 0))
    moved = c["FX-SPILL-018"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == four[at(0, y):at(W - 3, y)]
    # The half-covered skin is graded as the skin, at half covering.
    assert near([v * 255 / 128 for v in one[at(15, 0)][:3]], one[at(6, 0)][:3])
    for fx, frames in c.items():
        for f in frames.values():
            for i, p in enumerate(f):
                assert p[3] == drawn[i][3] or fx == "FX-SPILL-018"
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
    print("checked")


if __name__ == "__main__":
    main()
