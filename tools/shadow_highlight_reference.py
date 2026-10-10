"""Shadow/Highlight, worked a second way.

`core.shadow_highlight`, modelled on After Effects' Shadow/Highlight: the shadows brightened and
the highlights darkened, each pixel judged a shadow or a highlight by the brightness round it,
not by its own alone. Adobe does not publish its method. The rule below is darktable's
"shadows and highlights" module (src/iop/shadhi.c, its Gaussian softening), the rule GIMP's
Shadows-Highlights filter also takes: a blur of the lightness, inverted, laid on the pixel's own
lightness by an overlay, once for each whole step of the amount squared, with the colour's
chroma scaled to follow. Nothing is ported (D-400). Document 21 is the rule in words; this file
is the reference for the numbers document 25 pins against it.

The settings, each keyable, with the numbers After Effects' panel shows when it is added:
`shadow_amount` 0 to 100 (50), `highlight_amount` 0 to 100 (0), `shadow_tonal_width` and
`highlight_tonal_width` 0 to 100 (50), `shadow_radius` and `highlight_radius` 0 to 500 pixels
(30), `color_correction` 0 to 100 (20). Blend With Original is the mix every effect has (100
less it). After Effects' Auto Amounts, Temporal Smoothing, Scene Detect, Midtone Contrast and
Black and White Clip are not settings here (gaps, document 14).

The rule. At a pixel with covering a > 0, b its straight linear colour. Its CIE L*a*b* on the
sRGB primaries' own white (X, Y, Z = M b, with M the sRGB matrix of IEC 61966-2-1 and the white
M (1, 1, 1)), scaled as darktable scales it: l = L* / 100, p = a* / 128, q = b* / 128. A pixel
with a = 0 has l = 0 for the blur and is left as it is.

The blurs. l over the layer blurred by Gaussian Blur's kernel (document 21, sigma the radius in
pixels, normalised after truncation at three sigma), the edge pixels repeated: ls at the shadow
radius, lh at the highlight radius; a radius 0 is the pixel's own l. Inverted: ts = 1 - ls,
th = 1 - lh.

The amounts: s = 2 shadow / 100, h = 2 highlight / 100; the compressions cs = min(1 - shadow
width / 100, 0.99), ch = likewise; the colour share k = color correction / 100.

Highlights first, while h^2 > 0 (n = h^2, taking min(n, 1) and then 1 less each time), with
x = clamp(1 - th / (1 - ch), 0, 1):

    o = min(n, 1) x;   lb = (th - 1/2) sign(1 - l) + 1/2
    l' = l (1 - o) + (l > 1/2 ? 1 - (1 - 2 (l - 1/2)) (1 - lb) : 2 l lb) o
    f = l' r(l) (1 - k) + (1 - l') r(1 - l) k;   p' = p (1 - o + f o), q' likewise

then the shadows, while s^2 > 0, with x = clamp(ts / (1 - cs) - cs / (1 - cs), 0, 1), lb from
ts, and f = l' r(l) k + (1 - l') r(1 - l) (1 - k). r(v) is 1 / v with v's sign, |v| held at
1e-6 or more; sign(0) is 1. Back from l, p, q to linear (the same matrix's inverse), each
channel held inside 0 to 1, at the pixel's covering; the covering is never changed. With both
amounts 0 the layer is left exactly as it is.

What follows from it: black and white stay as they are (the overlay of 0 is 0, of 1 is 1); a
pixel is lifted only where the blur round it is darker than its tonal width lets through.

Deviations from darktable (recorded in document 14): two blurs and two tonal widths, one each
for the shadows and the highlights, as After Effects has them, where darktable has one radius
and one compress; darktable's white point adjustment is left at 0 (After Effects has none);
L*a*b* on sRGB's own white rather than darktable's D50 working profile; the result held inside
0 to 1. Gaussian softening, not darktable's default bilateral.

**This file never runs the build's code path.** It works in double precision on lists,
straight from the drawing's 8-bit values, where the build blurs in single precision.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py): pure colours, greys,
a skin tone, orange and three warm tones, each column darker down the rows. The projects go into
`Fixtures/shadow_highlight`, the expected frames into
`Fixtures/shadow_highlight/expected_shadow_highlight.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/shadow_highlight_reference.py
"""

import math
import sys
from fractions import Fraction
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "shadow_highlight"
NAMES = ("shadow_amount", "highlight_amount", "shadow_tonal_width", "shadow_radius",
         "highlight_tonal_width", "highlight_radius", "color_correction")
START = dict(shadow_amount=50, highlight_amount=0, shadow_tonal_width=50, shadow_radius=30,
             highlight_tonal_width=50, highlight_radius=30, color_correction=20)
RANGE = dict(shadow_amount=(0, 100), highlight_amount=(0, 100), shadow_tonal_width=(0, 100),
             shadow_radius=(0, 500), highlight_tonal_width=(0, 100), highlight_radius=(0, 500),
             color_correction=(0, 100))


# --- L*a*b* ---------------------------------------------------------------------------------

M = [[Fraction("0.4124"), Fraction("0.3576"), Fraction("0.1805")],
     [Fraction("0.2126"), Fraction("0.7152"), Fraction("0.0722")],
     [Fraction("0.0193"), Fraction("0.1192"), Fraction("0.9505")]]


def inverse(m):
    """A 3 by 3 inverse in fractions, by the adjugate."""
    (a, b, c), (d, e, f), (g, h, i) = m
    det = a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
    adj = [[e * i - f * h, c * h - b * i, b * f - c * e],
           [f * g - d * i, a * i - c * g, c * d - a * f],
           [d * h - e * g, b * g - a * h, a * e - b * d]]
    return [[v / det for v in row] for row in adj]


MI = [[float(v) for v in row] for row in inverse(M)]
WHITE = [float(sum(row)) for row in M]
MF = [[float(v) for v in row] for row in M]
EPS = (6 / 29) ** 3


def mul(m, v):
    return [sum(m[i][j] * v[j] for j in range(3)) for i in range(3)]


def f(t):
    return t ** (1 / 3) if t > EPS else t / (3 * (6 / 29) ** 2) + 4 / 29


def finv(u):
    return u ** 3 if u > 6 / 29 else 3 * (6 / 29) ** 2 * (u - 4 / 29)


def lab(b):
    """Straight linear colour to darktable's scaled l, p, q."""
    x, y, z = (v / w for v, w in zip(mul(MF, b), WHITE))
    fx, fy, fz = f(x), f(y), f(z)
    return [(116 * fy - 16) / 100, 500 * (fx - fy) / 128, 200 * (fy - fz) / 128]


def unlab(l, p, q):
    fy = (100 * l + 16) / 116
    fx, fz = fy + 128 * p / 500, fy - 128 * q / 200
    return mul(MI, [w * finv(u) for w, u in zip(WHITE, (fx, fy, fz))])


# --- the blur -------------------------------------------------------------------------------

def taps(sigma):
    """Gaussian Blur's weights (document 21), index 0 the farthest before the centre."""
    if sigma <= 0:
        return [1.0]
    r = math.ceil(3 * sigma)
    w = [math.exp(-((i - r) ** 2) / (2 * sigma * sigma)) for i in range(2 * r + 1)]
    s = sum(w)
    return [v / s for v in w]


def held_blur(plane, sigma):
    """`plane` (W by H) blurred across and then down, the edge pixels repeated."""
    t = taps(sigma)
    r = len(t) // 2
    across = [[sum(t[k] * plane[y][min(W - 1, max(0, x + k - r))] for k in range(len(t)))
               for x in range(W)] for y in range(H)]
    return [[sum(t[k] * across[min(H - 1, max(0, y + k - r))][x] for k in range(len(t)))
             for x in range(W)] for y in range(H)]


# --- the rule -------------------------------------------------------------------------------

def sign(v):
    return -1.0 if v < 0 else 1.0


def recip(v):
    return math.copysign(1 / abs(v) if abs(v) > 1e-6 else 1e6, v)


def overlay(l, p, q, t, amount, x_of, shadows, k):
    """darktable's loop: n = amount^2 in steps of at most 1, each laid by the overlay."""
    n = amount * amount
    x = x_of(t)
    while n > 0:
        la = l
        lb = (t - 0.5) * sign(1 - la) + 0.5
        o = min(n, 1.0) * x
        n -= 1
        l = la * (1 - o) + (1 - (1 - 2 * (la - 0.5)) * (1 - lb) if la > 0.5
                            else 2 * la * lb) * o
        if shadows:
            fac = l * recip(la) * k + (1 - l) * recip(1 - la) * (1 - k)
        else:
            fac = l * recip(la) * (1 - k) + (1 - l) * recip(1 - la) * k
        p = p * (1 - o) + p * fac * o
        q = q * (1 - o) + q * fac * o
    return l, p, q


def shadow_highlight(px, v):
    if v["shadow_amount"] == 0 and v["highlight_amount"] == 0:
        return [R.working(p) for p in px]
    straight = [[srgb_to_linear(c / 255) for c in p[:3]] for p in px]
    labs = [lab(b) if p[3] else [0.0, 0.0, 0.0] for b, p in zip(straight, px)]
    plane = [[labs[y * W + x][0] for x in range(W)] for y in range(H)]
    ls, lh = held_blur(plane, v["shadow_radius"]), held_blur(plane, v["highlight_radius"])
    s, h = 2 * v["shadow_amount"] / 100, 2 * v["highlight_amount"] / 100
    cs = min(1 - v["shadow_tonal_width"] / 100, 0.99)
    ch = min(1 - v["highlight_tonal_width"] / 100, 0.99)
    k = v["color_correction"] / 100
    out = []
    for i, p in enumerate(px):
        if p[3] == 0:
            out.append(R.working(p))
            continue
        y, x = divmod(i, W)
        l, pp, qq = labs[i]
        l, pp, qq = overlay(l, pp, qq, 1 - lh[y][x], h,
                            lambda t: min(1, max(0, 1 - t / (1 - ch))), False, k)
        l, pp, qq = overlay(l, pp, qq, 1 - ls[y][x], s,
                            lambda t: min(1, max(0, t / (1 - cs) - cs / (1 - cs))), True, k)
        a = p[3] / 255
        out.append([min(1.0, max(0.0, c)) * a for c in unlab(l, pp, qq)] + [a])
    return out


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, **settings):
    c = dict(START)
    c.update(settings)
    c["shift"] = shift
    return c


def render(c, frame_no):
    v = {n: min(RANGE[n][1], max(RANGE[n][0], value_at(c[n], frame_no))) for n in NAMES}
    return R.frame(shadow_highlight(B.pixels(), v), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    return B.project_json(fx, "core.shadow_highlight", {n: setting_json(c[n]) for n in NAMES},
                          c["shift"])


CASES = {
    "FX-SHHI-001": ("The settings as they start (shadow 50, highlight 0, widths 50, radii 30, "
                    "colour correction 20): the dark rows lifted, most where the blur round "
                    "them is darkest; black and white stay as they are.", case(), [0]),
    "FX-SHHI-002": ("Both amounts 0: the drawing exactly as it is.",
                    case(shadow_amount=0), [0]),
    "FX-SHHI-003": ("Shadow 100: the overlay laid four times (2 squared), lifted further than "
                    "FX-SHHI-001.", case(shadow_amount=100), [0]),
    "FX-SHHI-004": ("Highlight 50 alone at radius 2, shadow 0: the light pixels darkened "
                    "where the blur round them is light; white and black stay.",
                    case(shadow_amount=0, highlight_amount=50, highlight_radius=2), [0]),
    "FX-SHHI-005": ("Highlight 100 alone at radius 2: darkened further.",
                    case(shadow_amount=0, highlight_amount=100, highlight_radius=2), [0]),
    "FX-SHHI-006": ("Shadow 60 and highlight 40 together: the highlights first, then the "
                    "shadows.", case(shadow_amount=60, highlight_amount=40), [0]),
    "FX-SHHI-007": ("Shadow tonal width 30: only pixels whose surroundings are darker "
                    "change, less than FX-SHHI-001.", case(shadow_tonal_width=30), [0]),
    "FX-SHHI-008": ("Shadow tonal width 100: every pixel's surroundings count as shadow, "
                    "more than FX-SHHI-001.", case(shadow_tonal_width=100), [0]),
    "FX-SHHI-009": ("Shadow radius 1: each pixel judged by its nearest neighbours, so the "
                    "lift follows the columns' own brightness.", case(shadow_radius=1), [0]),
    "FX-SHHI-010": ("Shadow radius 0: each pixel judged by its own lightness alone.",
                    case(shadow_radius=0), [0]),
    "FX-SHHI-011": ("Shadow 70 at radius 2 and highlight 60 at radius 6: two blurs, one for "
                    "each.", case(shadow_amount=70, shadow_radius=2, highlight_amount=60,
                                  highlight_radius=6), [0]),
    "FX-SHHI-012": ("Colour correction 0: the lifted shadows lose colour (their chroma scaled "
                    "by how far they are from white).", case(color_correction=0), [0]),
    "FX-SHHI-013": ("Colour correction 100: the lifted shadows keep their colour's strength, "
                    "chroma scaled with the lightness.", case(color_correction=100), [0]),
    "FX-SHHI-014": ("Highlight tonal width 100, highlight 80, radius 3, shadow 0: the "
                    "highlights' widest reach.", case(shadow_amount=0, highlight_amount=80,
                                                      highlight_tonal_width=100,
                                                      highlight_radius=3), [0]),
    "FX-SHHI-015": ("Shadow amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame "
                    "0 untouched, frame 2 shadow 50 (FX-SHHI-001), frame 4 as FX-SHHI-003.",
                    case(shadow_amount=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-SHHI-016": ("FX-SHHI-011 moved three pixels right: the same, moved.",
                    case(shift=3, shadow_amount=70, shadow_radius=2, highlight_amount=60,
                         highlight_radius=6), [0, 3]),
}

INVALID = {
    "FX-SHHI-017": ("Shadow amount 101, above 100.", case(shadow_amount=101)),
    "FX-SHHI-018": ("Highlight amount -1, below 0.", case(highlight_amount=-1)),
    "FX-SHHI-019": ("Shadow tonal width 101, above 100.", case(shadow_tonal_width=101)),
    "FX-SHHI-020": ("Highlight radius 501, above 500.", case(highlight_radius=501)),
    "FX-SHHI-021": ("Colour correction -1, below 0.", case(color_correction=-1)),
}


def main():
    expected = B.write_cases(OUT, "shadow_highlight", CASES, INVALID, render, plain, file_json)
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    # The matrix's inverse and the round trip.
    for b in ([0.2, 0.5, 0.9], [1, 1, 1], [0.001, 0.002, 0.0], [1, 0, 0]):
        assert all(abs(u - v) < 1e-12 for u, v in zip(unlab(*lab(b)), b)), b
    assert all(abs(v) < 1e-12 for v in lab([0.3, 0.3, 0.3])[1:])  # greys have no chroma
    assert abs(lab([1, 1, 1])[0] - 1) < 1e-12 and lab([0, 0, 0])[0] == 0
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    px = B.pixels()
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731
    luma = lambda p: 0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2]  # noqa: E731
    covered = [i for i in range(W * H) if px[i][3]]

    one = c["FX-SHHI-001"]["0"]
    assert c["FX-SHHI-002"]["0"] == drawn
    for fx in ("FX-SHHI-001", "FX-SHHI-003", "FX-SHHI-004", "FX-SHHI-005", "FX-SHHI-006"):
        assert near(c[fx]["0"][at(1, 0)], drawn[at(1, 0)])  # black stays black
        assert near(c[fx]["0"][at(2, 0)], drawn[at(2, 0)])  # white stays white
    # Shadows only lift; the darkest grey rows lift most against FX-SHHI-001's neighbours.
    for i in covered:
        assert luma(one[i]) >= luma(drawn[i]) - 1e-12
        assert luma(c["FX-SHHI-003"]["0"][i]) >= luma(one[i]) - 1e-12
        assert luma(c["FX-SHHI-004"]["0"][i]) <= luma(drawn[i]) + 1e-12
        assert luma(c["FX-SHHI-005"]["0"][i]) <= luma(c["FX-SHHI-004"]["0"][i]) + 1e-12
        assert luma(c["FX-SHHI-007"]["0"][i]) <= luma(one[i]) + 1e-12
        assert luma(c["FX-SHHI-008"]["0"][i]) >= luma(one[i]) - 1e-12
    assert luma(one[at(3, 9)]) > luma(drawn[at(3, 9)]) + 1e-3
    assert luma(c["FX-SHHI-004"]["0"][at(3, 0)]) < luma(drawn[at(3, 0)]) - 1e-3
    # Radius 0: two pixels of one colour and row, whatever their neighbours, change alike; the
    # yellow at half covering is graded as the yellow.
    ten = c["FX-SHHI-010"]["0"]
    assert near([v / ten[at(15, 4)][3] for v in ten[at(15, 4)][:3]],
                [v / ten[at(4, 4)][3] for v in ten[at(4, 4)][:3]])
    assert not like(c["FX-SHHI-009"]["0"], one) and not like(ten, one)
    # Colour correction: the dark warm shadow's chroma grows with it.
    sat = lambda p: max(p[:3]) - min(p[:3])  # noqa: E731
    assert sat(c["FX-SHHI-012"]["0"][at(12, 5)]) < sat(one[at(12, 5)]) < sat(
        c["FX-SHHI-013"]["0"][at(12, 5)])
    fifteen = c["FX-SHHI-015"]
    assert fifteen["0"] == drawn and like(fifteen["2"], one) and like(fifteen["4"],
                                                                      c["FX-SHHI-003"]["0"])
    moved, eleven = c["FX-SHHI-016"]["0"], c["FX-SHHI-011"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == eleven[at(0, y):at(W - 3, y)]
    for fx, frames in c.items():
        for fr in frames.values():
            for i, p in enumerate(fr):
                assert p[3] == drawn[i][3] or fx == "FX-SHHI-016"
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
    print("checked")


if __name__ == "__main__":
    main()
