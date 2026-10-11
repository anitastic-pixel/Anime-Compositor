"""Linear Color Key, worked a second way.

`core.linear_color_key`, modelled on After Effects' Linear Color Key (Keying): it compares each
pixel with a key colour and makes the pixels that match transparent, those that match less less
transparent, and those that do not match opaque. Adobe describes the effect ("Linear keys create
a range of transparency across an image. A linear key compares each pixel in the image to the
key color you specify ... uses RGB, hue, or chroma information"; "Matching Tolerance ... A value
of 0 makes the entire image opaque; a value of 100 makes the entire image transparent"; "Matching
Softness: Softens the tolerance by tapering the tolerance value ... values under 20% produce the
best results") and names its views (Final Output, Source Only, Matte Only), but publishes no
formula and no defaults that we found. The rule and the settings when added are ours (document
21, D-504); where they differ from Adobe's own maths is a logged gap. Key Operation's Keep Colors
(a second instance that brings back colours a first one keyed out) is not here: it needs the
colour under pixels already fully keyed, which this pipeline's premultiplied frames do not keep;
it is a logged gap. The eyedroppers and thumbnails are After Effects' window, not settings.

The settings. `view`, "final" (when added), "source" or "matte"; `key_color`, a "#rrggbb"
colour, "#0000ff" when added; `match_colors`, "rgb" (when added, Adobe's advice), "hue" or
"chroma"; every word exact. `matching_tolerance` and `matching_softness`, 0 to 100, 10 and 10
when added, both keyable.

The rule. At a pixel with covering a > 0, e = (eR, eG, eB) its straight colour through the sRGB
curve held inside 0 to 1, and k the key colour's 8-bit values over 255. The distance d, 0 to 100:

- "rgb": 100 times the largest of |e_i - k_i|.
- "hue": 100 times the angle between their HSV hues over 180 (the shorter way round), and 100
  when either is a grey (no hue).
- "chroma": the chromaticities c = e / (eR + eG + eB) (a black's is (1/3, 1/3, 1/3)), 100 times
  the largest of |c_i - ck_i|; a colour and its darker or brighter self are the same here.

The share kept is m = (d - tolerance) / softness held inside 0 to 1; with softness 0 it is 0
where d <= tolerance and 1 elsewhere. So tolerance 100 keys everything, and tolerance 0 keys
only the key colour itself (Adobe: "the entire image opaque"; a logged difference). The views:
"final", every channel of the pixel, colour and covering, times m, so the colour is kept and
only the covering falls; "source", the layer exactly as it is; "matte", m as an encoded grey,
back to linear at the pixel's covering, which is kept. A pixel with a = 0 is left as it is.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works on its single-precision buffers. The rule
is continuous in the colour except where softness is 0 and at a grey in "hue"; the drawing's
greys are exact, and every case with softness 0 is checked that no pixel lies near its step.

The drawing is Color Difference Key's (tools/color_difference_key_reference.py): black, white,
grey, a green screen, a blue screen, skin, pure blue, cyan, pure green, yellow, red, magenta,
a violet, a pale blue, and the blue screen at half covering, each row darker by 8%. The
projects go into `Fixtures/linear_color_key`, the expected frames into
`Fixtures/linear_color_key/expected_linear_color_key.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/linear_color_key_reference.py
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import color_difference_key_reference as D  # noqa: E402  (draws the shared picture)

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "linear_color_key"
NUMBERS = ("matching_tolerance", "matching_softness")
BLUE_SCREEN = "#1e46c8"   # (30, 70, 200), the drawing's blue screen
GREEN_SCREEN = "#28b43c"  # (40, 180, 60), the drawing's green screen


# --- the rule -------------------------------------------------------------------------------

def chromaticity(e):
    s = sum(e)
    return [1 / 3] * 3 if s == 0 else [v / s for v in e]


def distance(e, k, match):
    if match == "rgb":
        return 100 * max(abs(u - v) for u, v in zip(e, k))
    if match == "hue":
        h, hk = D.hue(e), D.hue(k)
        if h is None or hk is None:
            return 100.0
        a = abs(h - hk) % 360
        return 100 * min(a, 360 - a) / 180
    return 100 * max(abs(u - v) for u, v in zip(chromaticity(e), chromaticity(k)))


def kept(d, k):
    if k["softness"] == 0:
        return 0.0 if d <= k["tolerance"] else 1.0
    return min(1.0, max(0.0, (d - k["tolerance"]) / k["softness"]))


def key(px, k):
    if k["view"] == "source":
        return [R.working(p) for p in px]
    out = []
    for p in px:
        w = R.working(p)
        if p[3] == 0:
            out.append(w)
            continue
        m = kept(distance(B.encoded(p), k["key"], k["match"]), k)
        if k["view"] == "final":
            out.append([v * m for v in w])
        else:
            a = p[3] / 255
            g = S.srgb_to_linear(m) * a
            out.append([g, g, g, a])
    return out


# --- the cases ------------------------------------------------------------------------------

def case(view="final", key_color="#0000ff", match_colors="rgb", shift=0, **numbers):
    c = {"view": view, "key_color": key_color, "match_colors": match_colors, "shift": shift,
         "matching_tolerance": 10, "matching_softness": 10}
    for n, v in numbers.items():
        assert n in NUMBERS, n
        c[n] = v
    return c


def settings(c, frame_no):
    v = {n: min(100, max(0, value_at(c[n], frame_no))) for n in NUMBERS}
    return {"view": c["view"], "key": D.hex_colour(c["key_color"]), "match": c["match_colors"],
            "tolerance": v["matching_tolerance"], "softness": v["matching_softness"]}


def render(c, frame_no):
    return R.frame(key(B.pixels(), settings(c, frame_no)), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    params = {"view": c["view"], "key_color": c["key_color"], "match_colors": c["match_colors"]}
    params.update({n: setting_json(c[n]) for n in NUMBERS})
    return B.project_json(fx, "core.linear_color_key", params, c["shift"])


CASES = {
    "FX-LCKEY-001": ("The settings as they start: final, key colour pure blue, RGB, tolerance "
                     "10, softness 10. Pure blue's top two rows (within 10% of it) lose all their "
                     "covering, its third row keeps a share, its darker rows and every other "
                     "colour keep all of theirs.", case(), [0]),
    "FX-LCKEY-002": ("RGB, key colour the blue screen (30, 70, 200), tolerance 10, softness 10: "
                     "the blue screen's top two rows and the half-covered one's fall, its third "
                     "row keeps a quarter, its darker rows all of it.", case(key_color=BLUE_SCREEN),
                     [0]),
    "FX-LCKEY-003": ("The same with softness 0: inside 10% gone, outside kept, nothing between.",
                     case(key_color=BLUE_SCREEN, matching_softness=0), [0]),
    "FX-LCKEY-004": ("Hue, key colour the green screen (hue about 129 degrees), tolerance 20, "
                     "softness 10: every row of the green screen and of pure green (hue 120) "
                     "falls, lightness not counted; cyan (hue 180) keeps a share; the greys, "
                     "which have no hue, and the far hues keep all their covering.",
                     case(key_color=GREEN_SCREEN, match_colors="hue", matching_tolerance=20),
                     [0]),
    "FX-LCKEY-005": ("Chroma, key colour the green screen, tolerance 10, softness 10: every row "
                     "of the green screen falls, its darker self has the same chromaticity; pure "
                     "green, far in chromaticity, keeps all its covering.",
                     case(key_color=GREEN_SCREEN, match_colors="chroma"), [0]),
    "FX-LCKEY-006": ("Tolerance 100: every pixel is within it, so the whole layer is keyed out.",
                     case(matching_tolerance=100), [0]),
    "FX-LCKEY-007": ("Tolerance 0, softness 20: only pure blue's top row, the key colour itself, "
                     "is gone; its next two rows, within 20% of it, keep a share d / 20; every "
                     "other pixel keeps all its covering.",
                     case(matching_tolerance=0, matching_softness=20), [0]),
    "FX-LCKEY-008": ("Hue with a grey key colour (#808080), tolerance 50: a grey has no hue, so "
                     "nothing matches and the layer is as it was.",
                     case(key_color="#808080", match_colors="hue", matching_tolerance=50), [0]),
    "FX-LCKEY-009": ("View matte, FX-LCKEY-004's settings: the share kept as grey, black where "
                     "the greens were keyed, white where nothing was, covering kept.",
                     case("matte", GREEN_SCREEN, "hue", matching_tolerance=20), [0]),
    "FX-LCKEY-010": ("View source, tolerance 100: the layer exactly as it is.",
                     case("source", matching_tolerance=100), [0]),
    "FX-LCKEY-011": ("Tolerance keyed from 0 at frame 0 to 40 at frame 4, linear, RGB on the "
                     "blue screen.", case(key_color=BLUE_SCREEN,
                                          matching_tolerance=keyed((0, 0), (4, 40))), [0, 2, 4]),
    "FX-LCKEY-012": ("Softness keyed from 0 at frame 0 to 60 at frame 4, linear, hue on the "
                     "green screen, tolerance 20.",
                     case(key_color=GREEN_SCREEN, match_colors="hue", matching_tolerance=20,
                          matching_softness=keyed((0, 0), (4, 60))), [2, 4]),
    "FX-LCKEY-013": ("FX-LCKEY-002 moved three pixels right: the same, moved.",
                     case(key_color=BLUE_SCREEN, shift=3), [0, 3]),
}

INVALID = {
    "FX-LCKEY-014": ("Matching tolerance 101, above 100.", case(matching_tolerance=101)),
    "FX-LCKEY-015": ("Matching softness -1, below 0.", case(matching_softness=-1)),
    "FX-LCKEY-016": ("View \"Final\": the word is exact, so a capital is not it.", case("Final")),
    "FX-LCKEY-017": ("Match colors \"lab\": not one of RGB, hue and chroma.",
                     case(match_colors="lab")),
    "FX-LCKEY-018": ("Key colour \"blue\", not a #rrggbb colour.", case(key_color="blue")),
}


def main():
    step_clear()
    expected = B.write_cases(OUT, "linear_color_key", CASES, INVALID, render, plain, file_json)
    check(expected)


def step_clear():
    """With softness 0 a pixel is in or out; none may lie within a hair of the tolerance."""
    for fx, (_, c, frames) in CASES.items():
        for f in frames:
            k = settings(c, f)
            if k["softness"] != 0:
                continue
            for p in B.pixels():
                if p[3]:
                    d = distance(B.encoded(p), k["key"], k["match"])
                    assert abs(d - k["tolerance"]) > 0.05, (fx, p, d)


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
    gone = lambda f, x, y: near(f[at(x, y)], [0, 0, 0, 0])  # noqa: E731
    others = lambda f, skip: all(full(f, x, y) for x in range(1, 16) if x not in skip  # noqa
                                 for y in range(H))

    one = c["FX-LCKEY-001"]["0"]
    assert gone(one, 7, 0) and gone(one, 7, 1) and 0.01 < cover(one, 7, 2) < 0.99
    assert all(full(one, 7, y) for y in range(4, H)) and others(one, {7})
    two = c["FX-LCKEY-002"]["0"]
    assert gone(two, 5, 0) and gone(two, 5, 1) and gone(two, 15, 0) and gone(two, 15, 1)
    assert abs(cover(two, 5, 2) - 0.25) < 0.05 and full(two, 5, 4)
    three = c["FX-LCKEY-003"]["0"]
    for x in range(1, 16):
        for y in range(H):
            assert gone(three, x, y) or full(three, x, y), (x, y)
    assert gone(three, 5, 1) and full(three, 5, 2)
    four = c["FX-LCKEY-004"]["0"]
    assert all(gone(four, 4, y) and gone(four, 9, y) for y in range(H))
    assert 0.01 < cover(four, 8, 0) < 0.99
    assert all(full(four, x, y) for x in (1, 2, 3, 7, 11, 12) for y in range(H))
    five = c["FX-LCKEY-005"]["0"]
    assert all(gone(five, 4, y) for y in range(H)) and full(five, 9, 0)
    assert all(near(p, [0, 0, 0, 0]) for p in c["FX-LCKEY-006"]["0"])
    seven = c["FX-LCKEY-007"]["0"]
    assert gone(seven, 7, 0) and 0.01 < cover(seven, 7, 1) < 0.99
    assert like(c["FX-LCKEY-008"]["0"], drawn)
    nine = c["FX-LCKEY-009"]["0"]
    for i, (p, q) in enumerate(zip(nine, four)):
        a = px[i][3] / 255
        assert abs(p[3] - a) < 1e-12 and p[0] == p[1] == p[2], i
        if a:
            assert abs(p[0] - S.srgb_to_linear(q[3] / a) * a) < 1e-12, i
    assert like(c["FX-LCKEY-010"]["0"], drawn)
    eleven = c["FX-LCKEY-011"]
    assert like(eleven["2"], render(case(key_color=BLUE_SCREEN, matching_tolerance=20), 0))
    assert gone(eleven["4"], 5, 4) and gone(eleven["0"], 5, 0)
    twelve = c["FX-LCKEY-012"]
    assert like(twelve["2"], render(case(key_color=GREEN_SCREEN, match_colors="hue",
                                         matching_tolerance=20, matching_softness=30), 0))
    # The colour is kept wherever any covering is left.
    for fx, frames in c.items():
        for f in frames.values():
            for p in f:
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
    for i, p in enumerate(two):
        q = drawn[i]
        if p[3] > 1e-6:
            assert near([u / p[3] for u in p[:3]], [u / q[3] for u in q[:3]], 1e-7)
    moved = c["FX-LCKEY-013"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == two[at(0, y):at(W - 3, y)]
    print("checked")


if __name__ == "__main__":
    main()
