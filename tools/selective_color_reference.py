"""Selective Color, worked a second way.

`core.selective_color`, modelled on After Effects' Selective Color (the same as Photoshop's
Selective Color adjustment): the cyan, magenta, yellow and black in each of nine colour families
(reds, yellows, greens, cyans, blues, magentas, whites, neutrals, blacks) turned up or down, the
change landing only on the pixels of that family, in proportion to how much they belong to it.
Adobe does not publish its method. The rule below is Clément Bœsch's reverse-engineering of
Photoshop's, measured against Photoshop and published with FFmpeg's `selectivecolor` filter
(libavfilter/vf_selectivecolor.c); nothing is ported (D-396). Document 21 is the rule in words;
this file is the reference for the numbers document 25 pins against it.

The settings. `method`, "relative" or "absolute", "relative" when added (Photoshop's own
start); the word is exact. `reds`, `yellows`, `greens`, `cyans`, `blues`, `magentas`, `whites`,
`neutrals` and `blacks`, each four numbers, the cyan, magenta, yellow and black change in per
cent, -100 to 100, all 0 when added, keyable. (After Effects' Colors menu only chooses which
family's four sliders are shown, so it is not a setting here.)

The rule. At a pixel with covering a > 0, e = (eR, eG, eB) its straight colour through the sRGB
curve held inside 0 to 1, and mx, md, mn its largest, middle and smallest channel. How much the
pixel belongs to each family, its weight w:

- reds, w = mx - md when eR is the largest; greens when eG is; blues when eB is;
- cyans, w = md - mn when eR is the smallest; magentas when eG is; yellows when eB is;
- whites, w = 2 mn - 1; neutrals, w = 1 - (|mx - 0.5| + |mn - 0.5|); blacks, w = 1 - 2 mx.

Only a family with w > 0 counts. For each counted family with amounts c, m, y, k over 100,
each channel takes its own: red the cyan, green the magenta, blue the yellow (s below), and

    t = (-1 - s) k - s,   times (1 - e) for "relative",   held inside -e to 1 - e,

the channel's e its own; the channel moves by t w, summed over the families. The result held
inside 0 to 1, back to linear at the pixel's covering (document 21's shared colour rule); the
covering is never changed. A pixel with a = 0 is left as it is; with every amount 0 the layer is
left exactly as it is. Adobe's own example agrees: a pixel of 50% magenta given +10% is 55%
relative, 60% absolute; and relative cannot change a channel already full, so pure white stays
white under any whites change.

Bœsch's measurements of Photoshop, on a pixel (180, 100, 50) through the reds (weight 80/255):
absolute cyan +60 moves red by -48; cyan, magenta, yellow +100 give (124, 69, 34), -100 give
(204, 149, 114); cyan 40, magenta -60, yellow 10 with black -40 give red 193, with black 20 red
126, with black -80 blue 112, with black -10 blue 51. `check()` asserts the rule gives each
within 1 level.

Deviations from FFmpeg's code (recorded in document 14): FFmpeg rounds each family's change to
whole levels and works on 8-bit integers, so a whitest channel of exactly 128 is not a white
there (weight 1/255); this file and the build work in fractions, without that rounding. FFmpeg's
default method is absolute; Photoshop's, and ours, is relative.

**This file never runs the build's code path.** It works in double precision on lists,
straight from the drawing's 8-bit values, where the build works on its single-precision buffers.
The rule has no steps: every weight is continuous in the colour, so rounding can't flip a pixel.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py): pure colours, greys,
a skin tone, orange and three warm tones. The projects go into `Fixtures/selective_color`, the
expected frames into `Fixtures/selective_color/expected_selective_color.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/selective_color_reference.py
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "selective_color"
NAMES = ("reds", "yellows", "greens", "cyans", "blues", "magentas", "whites", "neutrals",
         "blacks")
ZERO = (0, 0, 0, 0)


# --- the rule -------------------------------------------------------------------------------

def weights(e):
    """Each family's weight, in NAMES' order."""
    mx, md, mn = max(e), sorted(e)[1], min(e)
    top, low = mx - md, md - mn
    return [top if e[0] == mx else 0, low if e[2] == mn else 0,
            top if e[1] == mx else 0, low if e[0] == mn else 0,
            top if e[2] == mx else 0, low if e[1] == mn else 0,
            2 * mn - 1, 1 - (abs(mx - 0.5) + abs(mn - 0.5)), 1 - 2 * mx]


def selective(e, amounts, relative):
    """One encoded colour through the rule; amounts maps each name to four per cents."""
    move = [0.0, 0.0, 0.0]
    for name, w in zip(NAMES, weights(e)):
        if w <= 0:
            continue
        c, m, y, k = (v / 100 for v in amounts[name])
        for i, s in enumerate((c, m, y)):
            t = (-1 - s) * k - s
            if relative:
                t *= 1 - e[i]
            move[i] += min(1 - e[i], max(-e[i], t)) * w
    return [v + d for v, d in zip(e, move)]


def selective_color(px, amounts, relative):
    if all(v == 0 for a in amounts.values() for v in a):
        return [R.working(p) for p in px]
    return [R.working(p) if p[3] == 0 else B.back(selective(B.encoded(p), amounts, relative), p)
            for p in px]


# --- the cases ------------------------------------------------------------------------------

def case(method="relative", shift=0, **amounts):
    c = {"method": method, "shift": shift}
    c.update({n: amounts.get(n, ZERO) for n in NAMES})
    return c


def render(c, frame_no):
    amounts = {n: [min(100, max(-100, v)) for v in value_at(c[n], frame_no)] for n in NAMES}
    return R.frame(selective_color(B.pixels(), amounts, c["method"] == "relative"), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    params = {"method": c["method"]}
    params.update({n: setting_json(c[n]) for n in NAMES})
    return B.project_json(fx, "core.selective_color", params, c["shift"])


ALL = dict(reds=(-20, 10, 30, 5), yellows=(15, -10, -40, 0), greens=(30, -20, 10, -10),
           cyans=(-30, 20, 0, 15), blues=(20, 30, -20, 0), magentas=(-10, -30, 20, 10),
           whites=(0, 0, 10, -10), neutrals=(5, -5, -15, 5), blacks=(0, 10, 0, 20))

CASES = {
    "FX-SELC-001": ("The settings as they start: relative, every amount 0: the drawing exactly "
                    "as it is.", case(), [0]),
    "FX-SELC-002": ("Reds, cyan +100, absolute: the reds lose their red by as much as they are "
                    "red, pure red goes black, orange olive, skin greyer; colours "
                    "with no red family (greys, green, cyan, blue) are untouched.",
                    case("absolute", reds=(100, 0, 0, 0)), [0]),
    "FX-SELC-003": ("The same, relative: a channel's change is scaled by how much room it has "
                    "below full, so pure red and orange, whose red is full, are untouched and "
                    "the darker rows change most.", case(reds=(100, 0, 0, 0)), [0]),
    "FX-SELC-004": ("Yellows, yellow -100, absolute: the yellows' blue raised by as much as "
                    "they are yellow: pure yellow becomes white, orange pink, the warm tones "
                    "greyer.", case("absolute", yellows=(0, 0, -100, 0)), [0]),
    "FX-SELC-005": ("Greens, magenta +100, absolute: the greens' green taken down, pure green "
                    "to black; yellow and cyan, where green ties for largest, untouched.",
                    case("absolute", greens=(0, 100, 0, 0)), [0]),
    "FX-SELC-006": ("Cyans, cyan -100, relative: the cyans' red raised, pure cyan becomes "
                    "white.", case(cyans=(-100, 0, 0, 0)), [0]),
    "FX-SELC-007": ("Blues, yellow +100, absolute: the blues' blue taken down, pure blue to "
                    "black; cyan and magenta, where blue ties for largest, untouched.",
                    case("absolute", blues=(0, 0, 100, 0)), [0]),
    "FX-SELC-008": ("Magentas, magenta -50 and black +50, absolute: the black darkens red and "
                    "blue by half; on green the black and the magenta cut partly cancel, so "
                    "pure magenta's green rises a quarter.",
                    case("absolute", magentas=(0, -50, 0, 50)), [0]),
    "FX-SELC-009": ("Whites, black +100, absolute: the light pixels darkened by how light "
                    "they are, white to black, the light greys and the skin and warm highlight "
                    "darker; nothing at or below half grey changes.",
                    case("absolute", whites=(0, 0, 0, 100)), [0]),
    "FX-SELC-010": ("The same, relative: a full channel can't be changed, so pure white stays "
                    "white (Adobe's note); the light greys still darken.",
                    case(whites=(0, 0, 0, 100)), [0]),
    "FX-SELC-011": ("Neutrals, cyan -30, yellow +40, relative: the midtones warmed, more red "
                    "and less blue, most at half grey; pure colours, black and white untouched.",
                    case(neutrals=(-30, 0, 40, 0)), [0]),
    "FX-SELC-012": ("Blacks, black -50, relative: the dark pixels lifted by how dark they are, "
                    "black to half grey's level; nothing at or above half grey changes.",
                    case(blacks=(0, 0, 0, -50)), [0]),
    "FX-SELC-013": ("Reds, cyan 40, magenta -60, yellow 10, black 20, absolute, Bœsch's "
                    "Photoshop settings: the black darkening every channel as the colour "
                    "changes shift them.", case("absolute", reds=(40, -60, 10, 20)), [0]),
    "FX-SELC-014": ("Reds, every amount +100, absolute: each change held at the channel's own "
                    "floor, the reds go to black by as much as they are red.",
                    case("absolute", reds=(100, 100, 100, 100)), [0]),
    "FX-SELC-015": ("All nine families at once, relative: each pixel moved by every family it "
                    "belongs to, added together.", case(**ALL), [0]),
    "FX-SELC-016": ("All nine, absolute: the same amounts, unscaled, so larger changes.",
                    case("absolute", **ALL), [0]),
    "FX-SELC-017": ("Reds keyed from 0, 0, 0, 0 at frame 0 to cyan +100 at frame 4, linear, "
                    "absolute: frame 0 untouched, frame 2 cyan +50, frame 4 as FX-SELC-002.",
                    case("absolute", reds=keyed((0, ZERO), (4, (100, 0, 0, 0)))), [0, 2, 4]),
    "FX-SELC-018": ("FX-SELC-015 moved three pixels right: the same, moved.",
                    case(shift=3, **ALL), [0, 3]),
}

INVALID = {
    "FX-SELC-019": ("Reds' cyan 101, above 100.", case(reds=(101, 0, 0, 0))),
    "FX-SELC-020": ("Blacks' black -101, below -100.", case(blacks=(0, 0, 0, -101))),
    "FX-SELC-021": ("Greens three numbers, not four.", case(greens=(0, 10, 0))),
    "FX-SELC-022": ("Method \"Relative\": the word is exact, so a capital is not it.",
                    case("Relative")),
    "FX-SELC-023": ("Method \"percentage\", which is not one.", case("percentage")),
}


def main():
    expected = B.write_cases(OUT, "selective_color", CASES, INVALID, render, plain, file_json)
    check(expected)


def photoshop():
    """Bœsch's Photoshop measurements on (180, 100, 50), each within 1 level."""
    px = [180, 100, 50]
    e = [v / 255 for v in px]

    def got(**reds):
        a = {n: ZERO for n in NAMES}
        a.update(reds)
        return [min(255, max(0, v * 255)) for v in selective(e, a, False)]

    def near(out, want):
        assert all(abs(u - v) <= 1 for u, v in zip(out, want)), (out, want)

    assert abs(weights(e)[0] - 80 / 255) < 1e-15
    near(got(reds=(60, 0, 0, 0))[:1], [180 - 48])
    near(got(reds=(0, -60, 0, 0))[1:2], [100 + 48])
    near(got(reds=(0, 0, -70, 0))[2:], [50 + 56])
    near(got(reds=(100, 100, 100, 0)), [124, 69, 34])
    near(got(reds=(-100, -100, -100, 0)), [204, 149, 114])
    near(got(reds=(40, -60, 10, -40))[:1], [193])
    near(got(reds=(40, -60, 10, 20))[:1], [126])
    near(got(reds=(40, -60, 10, -80))[2:], [112])
    near(got(reds=(40, -60, 10, -10))[2:], [51])
    # Adobe's example: 50% magenta (green 0.5) given +10% magenta, 55% relative, 60% absolute,
    # at full weight.
    a = {n: ZERO for n in NAMES}
    a["magentas"] = (0, 10, 0, 0)
    g = [1.0, 0.5, 1.0]  # green the smallest, md - mn = 0.5; scale to weight 1 below
    assert abs((selective(g, a, True)[1] - 0.5) / 0.5 + 0.05) < 1e-12
    assert abs((selective(g, a, False)[1] - 0.5) / 0.5 + 0.10) < 1e-12


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    photoshop()
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    px = B.pixels()
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731
    enc = lambda p: [S.linear_to_srgb(u / p[3]) for u in p[:3]]  # noqa: E731

    assert c["FX-SELC-001"]["0"] == drawn
    two, three = c["FX-SELC-002"]["0"], c["FX-SELC-003"]["0"]
    assert near(enc(two[at(8, 0)]), [0, 0, 0])  # red goes black
    for x in (1, 2, 3, 5, 6, 9):  # no red family
        for y in range(H):
            assert near(two[at(x, y)], drawn[at(x, y)])
    assert near(three[at(8, 0)], drawn[at(8, 0)]) and near(three[at(11, 0)], drawn[at(11, 0)])
    assert not near(three[at(8, 5)], drawn[at(8, 5)])
    assert near(enc(c["FX-SELC-004"]["0"][at(4, 0)]), [1, 1, 1])  # yellow to white
    assert near(enc(c["FX-SELC-005"]["0"][at(6, 0)]), [0, 0, 0])
    assert near(enc(c["FX-SELC-006"]["0"][at(5, 0)]), [1, 1, 1])  # cyan to white
    assert near(enc(c["FX-SELC-007"]["0"][at(9, 0)]), [0, 0, 0])
    eight = enc(c["FX-SELC-008"]["0"][at(7, 0)])
    assert near(eight, [0.5, 0.25, 0.5])  # magenta: red and blue -0.5, green +0.25
    nine, ten = c["FX-SELC-009"]["0"], c["FX-SELC-010"]["0"]
    assert near(enc(nine[at(2, 0)]), [0, 0, 0]) and near(ten[at(2, 0)], drawn[at(2, 0)])
    assert enc(ten[at(2, 2)])[0] < B.encoded(px[at(2, 2)])[0]
    for i in range(W * H):
        if px[i][3] and min(px[i][:3]) <= 127:
            assert near(nine[i], drawn[i]) and near(ten[i], drawn[i])
    eleven = c["FX-SELC-011"]["0"]
    for x in (1, 2, 4, 5, 6, 7, 8, 9):
        assert near(eleven[at(x, 0)], drawn[at(x, 0)])
    g = enc(eleven[at(3, 0)])
    assert g[0] > g[1] > g[2]
    twelve = c["FX-SELC-012"]["0"]
    assert near(enc(twelve[at(1, 0)]), [0.5, 0.5, 0.5])
    for i in range(W * H):
        if px[i][3] and max(px[i][:3]) >= 128:
            assert near(twelve[i], drawn[i])
    seventeen = c["FX-SELC-017"]
    assert seventeen["0"] == drawn and like(seventeen["4"], two)
    assert like(seventeen["2"], render(case("absolute", reds=(50, 0, 0, 0)), 0))
    moved = c["FX-SELC-018"]["0"]
    fifteen = c["FX-SELC-015"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == fifteen[at(0, y):at(W - 3, y)]
    assert not like(fifteen, c["FX-SELC-016"]["0"])
    # The soft yellow is graded as the yellow, at half covering.
    assert near([v * 255 / 128 for v in fifteen[at(15, 0)][:3]], fifteen[at(4, 0)][:3])
    for fx, frames in c.items():
        for f in frames.values():
            for i, p in enumerate(f):
                assert p[3] == drawn[i][3] or fx == "FX-SELC-018"
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
    print("checked")


if __name__ == "__main__":
    main()
