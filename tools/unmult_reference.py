"""Unmult, worked a second way.

`core.unmult`, modelled on After Effects' Unmult (Keying, new in After Effects 26.0, January
2026): it turns a black or white background into transparency, so fire, smoke or a flare shot on
black composites cleanly. Adobe describes the effect ("removes solid black or white backgrounds
... to restore the original transparency") and names its controls: Background Color (Black or
White); Black Level or White Level, one threshold whose name follows the background ("Any color
values below the Black Level will become transparent"; "above the White Level"); Softness ("a
smoother gradient between transparent and opaque areas"); Remove Color Matting ("Removes the
background color from semi-transparent areas, then restores colors to what they likely were
before blending with the background"); and Clip HDR Results ("Limits color values in HDR footage
... with negative values or values greater than 1 during the Remove Color Matting process"). It
publishes no formula and no defaults that we found. The rule and the settings when added are
ours (document 21, D-505); where they differ from Adobe's own maths is a logged gap.

The settings. `background_color`, "black" (when added) or "white"; `black_level`, 0 to 100, 0
when added, used with black; `white_level`, 0 to 100, 100 when added, used with white (After
Effects shows one of the two, under one name; we keep both, so switching the background never
loses the other's value); `softness`, 0 to 100, 0 when added; the three numbers keyable.
`remove_color_matting`, "on" (when added) or "off"; `clip_hdr_results`, "off" (when added) or
"on"; every word exact.

The rule, on the working pixel: linear light, premultiplied, colour c = (cR, cG, cB) and covering
a. A pixel with a = 0 is left as it is. How far the pixel is from the background, m, held inside
0 to a:

- black: m = the largest of cR, cG, cB (the pixel over black, its brightest channel); the level
  L = black_level / 100;
- white: m = a - the smallest of cR, cG, cB (how far the pixel over white falls below white in
  its darkest channel); L = 1 - white_level / 100.

The share w: with softness s = softness / 100 above 0, w = (m - L) / s held inside 0 to 1; with
softness 0, w is 1 where m > L and 0 elsewhere. The new covering is m w. Then:

- Remove Color Matting on: the colour the pixel had before it was laid over the background,
  u = c / m (black) or (c - a + m) / m (white), so the pixel over its background looks exactly
  as before wherever w is 1. With Clip HDR Results on, u is held inside 0 to 1.
- off: the pixel's own straight colour c / a is kept, so the edges keep the background's tint.

The output is u times the new covering, with the new covering; a new covering of 0 is a fully
transparent pixel. As added (black, level 0, softness 0) the new covering is the brightest
channel and the colour over black is untouched: Soft Physical Glow's Unmult (D-353), which this
is. Worked in linear light, as the whole pipeline is, so a mid grey's covering is its linear
value (about 0.22 for 128), not its encoded 0.5: a logged difference from an 8-bit After Effects
project, which works on encoded values.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works on its single-precision buffers. The rule
is continuous except where softness is 0 at m = L; every such case is checked that no pixel lies
near its step.

The drawing is Color Difference Key's (tools/color_difference_key_reference.py): black, white,
grey, a green screen, a blue screen, skin, pure blue, cyan, pure green, yellow, red, magenta,
a violet, a pale blue, and the blue screen at half covering, each row darker by 8%. FX-UNMULT-010
and 011 put an Exposure of 2 stops (document 21: linear premultiplied colour times 4) before the
Unmult, so the pixels run past 1. The projects go into `Fixtures/unmult`, the expected frames
into `Fixtures/unmult/expected_unmult.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/unmult_reference.py
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import color_difference_key_reference as D  # noqa: E402,F401  (draws the shared picture)

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "unmult"
NUMBERS = ("black_level", "white_level", "softness")
WORDS = ("background_color", "remove_color_matting", "clip_hdr_results")


# --- the rule -------------------------------------------------------------------------------

def far(w, k):
    """m, how far the working pixel w is from the background, and the level L."""
    c, a = w[:3], w[3]
    if k["background"] == "black":
        return min(a, max(0.0, max(c))), k["black"] / 100
    return min(a, max(0.0, a - min(c))), 1 - k["white"] / 100


def unmult(px, k):
    out = []
    for w in px:
        if w[3] == 0:
            out.append(w)
            continue
        c, a = w[:3], w[3]
        m, level = far(w, k)
        s = k["softness"] / 100
        share = (1.0 if m > level else 0.0) if s == 0 else min(1.0, max(0.0, (m - level) / s))
        cover = m * share
        if cover == 0:
            out.append([0.0, 0.0, 0.0, 0.0])
            continue
        if k["matting"] == "on":
            u = [v / m for v in c] if k["background"] == "black" else [(v - a + m) / m for v in c]
            if k["clip"] == "on":
                u = [min(1.0, max(0.0, v)) for v in u]
        else:
            u = [v / a for v in c]
        out.append([v * cover for v in u] + [cover])
    return out


def working(c):
    """The layer as the Unmult receives it: the drawing, through the Exposure if there is one."""
    g = 2.0 ** c["stops"]
    return [[v * g for v in w[:3]] + [w[3]] for w in (R.working(p) for p in B.pixels())]


# --- the cases ------------------------------------------------------------------------------

def case(background_color="black", remove_color_matting="on", clip_hdr_results="off", shift=0,
         stops=0, **numbers):
    c = {"background_color": background_color, "remove_color_matting": remove_color_matting,
         "clip_hdr_results": clip_hdr_results, "shift": shift, "stops": stops,
         "black_level": 0, "white_level": 100, "softness": 0}
    for n, v in numbers.items():
        assert n in NUMBERS, n
        c[n] = v
    return c


def settings(c, frame_no):
    v = {n: min(100, max(0, value_at(c[n], frame_no))) for n in NUMBERS}
    return {"background": c["background_color"], "black": v["black_level"],
            "white": v["white_level"], "softness": v["softness"],
            "matting": c["remove_color_matting"], "clip": c["clip_hdr_results"]}


def render(c, frame_no):
    return R.frame(unmult(working(c), settings(c, frame_no)), c["shift"])


def plain(c):
    return R.frame(working(c), c["shift"])


def file_json(fx, c):
    params = {w: c[w] for w in WORDS}
    params.update({n: setting_json(c[n]) for n in NUMBERS})
    p = B.project_json(fx, "core.unmult", params, c["shift"])
    if c["stops"]:
        p["compositions"][0]["layers"][0]["effects"].insert(0, {
            "instance_id": "fx-0-e", "type_id": "core.exposure", "enabled": True,
            "parameters": {"stops": c["stops"]}})
    return p


CASES = {
    "FX-UNMULT-001": ("The settings as they start: background black, black level 0, softness 0, "
                      "remove color matting on, clip HDR results off. Every pixel's covering "
                      "becomes its brightest channel in linear light, so black is gone, white "
                      "stays whole and a mid grey keeps about a fifth; the colour over black is "
                      "exactly as it was.", case(), [0]),
    "FX-UNMULT-002": ("Black level 10, softness 0: every pixel whose brightest channel is 10% or "
                      "less is gone too, the rest as FX-UNMULT-001.", case(black_level=10), [0]),
    "FX-UNMULT-003": ("Black level 10, softness 20: those pixels fade in over the next 20% "
                      "instead of starting at once.", case(black_level=10, softness=20), [0]),
    "FX-UNMULT-004": ("Remove color matting off: the same coverings as FX-UNMULT-001, but each "
                      "pixel keeps its own colour, so the part-covered ones look darker over "
                      "black than before.", case(remove_color_matting="off"), [0]),
    "FX-UNMULT-005": ("Background white, white level 100: white is gone, black stays whole, a "
                      "colour's covering is how far its darkest channel is below white; the "
                      "colour over white is exactly as it was.", case("white"), [0]),
    "FX-UNMULT-006": ("Background white, white level 80, softness 20: the pixels within 20% of "
                      "white in their darkest channel go, the next 20% fade in.",
                      case("white", white_level=80, softness=20), [0]),
    "FX-UNMULT-007": ("Background white, remove color matting off: FX-UNMULT-005's coverings "
                      "with each pixel's own colour.", case("white", "off"), [0]),
    "FX-UNMULT-008": ("Softness 50 at black level 0: every covering is the brightest channel "
                      "times the brightest channel over half, so the darker pixels thin out "
                      "faster.", case(softness=50), [0]),
    "FX-UNMULT-009": ("Black level 100, softness 10: no pixel is brighter than it, so the whole "
                      "layer is gone.", case(black_level=100, softness=10), [0]),
    "FX-UNMULT-010": ("An Exposure of 2 stops first, so the brighter pixels run past 1, then "
                      "the settings as they start: a pixel brighter than 1 stays whole with its "
                      "colour past 1 kept.", case(stops=2), [0]),
    "FX-UNMULT-011": ("The same with clip HDR results on: the colours past 1 are held at 1, so "
                      "white comes back to white, the brightest colours lose their brightness.",
                      case(clip_hdr_results="on", stops=2), [0]),
    "FX-UNMULT-012": ("Black level keyed from 0 at frame 0 to 40 at frame 4, linear, softness "
                      "10.", case(black_level=keyed((0, 0), (4, 40)), softness=10), [0, 2, 4]),
    "FX-UNMULT-013": ("Softness keyed from 0 at frame 0 to 40 at frame 4, linear, background "
                      "white, white level 90.",
                      case("white", white_level=90, softness=keyed((0, 0), (4, 40))), [2, 4]),
    "FX-UNMULT-014": ("FX-UNMULT-003 moved three pixels right: the same, moved.",
                      case(black_level=10, softness=20, shift=3), [0, 3]),
}

INVALID = {
    "FX-UNMULT-015": ("Black level 101, above 100.", case(black_level=101)),
    "FX-UNMULT-016": ("Softness -1, below 0.", case(softness=-1)),
    "FX-UNMULT-017": ("Background color \"grey\": not black or white.", case("grey")),
    "FX-UNMULT-018": ("Remove color matting \"yes\": not on or off.", case(remove_color_matting="yes")),
    "FX-UNMULT-019": ("Clip HDR results \"On\": the word is exact, so a capital is not it.",
                      case(clip_hdr_results="On")),
}


def main():
    step_clear()
    expected = B.write_cases(OUT, "unmult", CASES, INVALID, render, plain, file_json)
    check(expected)


def step_clear():
    """With softness 0 a pixel is in or out; none may lie within a hair of the level, but at
    level 0 (m = 0 is a gone pixel either way)."""
    for fx, (_, c, frames) in CASES.items():
        for f in frames:
            k = settings(c, f)
            if k["softness"] != 0:
                continue
            for w in working(c):
                if w[3]:
                    m, level = far(w, k)
                    if level > 0:
                        assert abs(m - level) > 1e-3, (fx, w, m)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    px = [R.working(p) for p in B.pixels()]
    near = lambda p, q, e=1e-12: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731
    gone = lambda p: near(p, [0, 0, 0, 0])  # noqa: E731

    one = c["FX-UNMULT-001"]["0"]
    for p, q in zip(one, px):
        if q[3]:
            assert near(p[:3], q[:3]) and abs(p[3] - min(q[3], max(q[:3]))) < 1e-12, q
    assert all(gone(one[at(1, y)]) for y in range(H)) and one[at(2, 0)][3] == 1.0
    assert abs(one[at(3, 0)][3] - S.srgb_to_linear(128 / 255)) < 1e-12
    two = c["FX-UNMULT-002"]["0"]
    for p, q in zip(two, one):
        assert near(p, q) or (gone(p) and 0 < q[3] <= 0.1)
    assert sum(gone(p) for p in two) > sum(gone(p) for p in one) + 5
    three = c["FX-UNMULT-003"]["0"]
    assert any(0 < p[3] < q[3] - 1e-6 for p, q in zip(three, one))
    four = c["FX-UNMULT-004"]["0"]
    for p, q, o in zip(four, px, one):
        assert abs(p[3] - o[3]) < 1e-12
        if q[3] and p[3]:
            assert near([v / p[3] for v in p[:3]], [v / q[3] for v in q[:3]], 1e-9)
    assert four[at(3, 0)][0] < one[at(3, 0)][0]
    five = c["FX-UNMULT-005"]["0"]
    for p, q in zip(five, px):
        if q[3]:
            # The pixel over white, before and after.
            assert near([v + 1 - p[3] for v in p[:3]], [v + 1 - q[3] for v in q[:3]], 1e-9), q
    assert gone(five[at(2, 0)]) and all(five[at(1, y)][3] == 1.0 for y in range(H))
    six = c["FX-UNMULT-006"]["0"]
    assert sum(gone(p) for p in six) > sum(gone(p) for p in five)
    assert any(0 < p[3] < q[3] - 1e-6 for p, q in zip(six, five))
    seven = c["FX-UNMULT-007"]["0"]
    for p, q, o in zip(seven, px, five):
        assert abs(p[3] - o[3]) < 1e-12
    eight = c["FX-UNMULT-008"]["0"]
    for p, o in zip(eight, one):
        assert abs(p[3] - o[3] * min(1, o[3] / 0.5)) < 1e-12
    assert all(gone(p) for p in c["FX-UNMULT-009"]["0"])
    hot = working(case(stops=2))
    ten, eleven = c["FX-UNMULT-010"]["0"], c["FX-UNMULT-011"]["0"]
    assert near(ten[at(2, 0)], [4, 4, 4, 1]) and near(eleven[at(2, 0)], [1, 1, 1, 1])
    # A pixel at least as bright as its covering is left exactly as the Exposure made it.
    assert all(near(p, q) for p, q in zip(ten, hot) if q[3] and max(q[:3]) >= q[3])
    assert any(max(p[:3]) > 1 for p in ten) and all(max(p[:3]) <= p[3] + 1e-12 for p in eleven)
    twelve = c["FX-UNMULT-012"]
    assert twelve["2"] == render(case(black_level=20, softness=10), 0)
    thirteen = c["FX-UNMULT-013"]
    assert thirteen["2"] == render(case("white", white_level=90, softness=20), 0)
    moved = c["FX-UNMULT-014"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == three[at(0, y):at(W - 3, y)]
    # Never more covered than the layer was.
    for fx, frames in c.items():
        if fx in ("FX-UNMULT-010", "FX-UNMULT-011"):
            continue
        for f in frames.values():
            for p, q in zip(f, R.frame(px, 3 if fx == "FX-UNMULT-014" else 0)):
                assert p[3] <= q[3] + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
    print("checked")


if __name__ == "__main__":
    main()
