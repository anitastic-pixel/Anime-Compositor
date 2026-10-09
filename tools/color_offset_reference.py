"""Color Offset, worked a second way.

`core.color_offset`, modelled on CycoreFX's CC Color Offset (its manual): each colour channel's
value is turned round by its own phase, in degrees, and what runs past the top or the bottom is
brought back by one of three overflows. Nothing is ported; this is this program's own reading.
Document 21 is the rule in words; this file is the reference for the numbers document 25 pins
against it.

The rule. Settings: `red_phase`, `green_phase` and `blue_phase`, -3600 to 3600 degrees, 0 when
added; `overflow`, "wrap", "solarize" or "polarize", "wrap" when added; words are exact. At a
pixel with covering a > 0, e its straight colour through the sRGB curve held inside 0 to 1 (the
space: the encoded colour, as every colour effect's, where CycoreFX works on the stored 0 to 255
values; a deviation from linear light recorded in document 21). Each channel, with f its phase
over 360 (turns), and a channel whose phase is 0 left as it is:

- "wrap": u = e + f; u itself when 0 <= u <= 1, otherwise u less its whole part, so one turn
  runs the value once round the range (black and white are the same point on the circle);
- "solarize": u = e + f, and 1 - |r - 1| with r the remainder of u over 2 (0 to 2): a triangle,
  so the value runs up to white, back down to black, and two turns bring it back;
- "polarize": the value as the height of a point on a half circle, e = (1 - cos t) / 2; the
  point is turned on by half a turn per turn of phase, so the output is (1 - cos(t + pi f)) / 2:
  the same up-and-down as solarize, smoothed into a cosine, and two turns bring it back.

The result, held inside 0 to 1, comes back to linear at the pixel's own covering (document 21's
shared colour rule). With all three phases 0 the layer is left exactly as it is.

**This file never runs the build's code path.** It works in double precision on lists,
straight from the drawing's 8-bit values, where the build works on its single-precision
buffers; polarize takes the arc cosine and cosine directly. It asserts that no wrapped value
lies within a millionth of a whole number, so rounding can't send a pixel round the other way.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py). The projects go into
`Fixtures/color_offset`, the expected frames into `Fixtures/color_offset/expected_color_offset.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/color_offset_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "color_offset"
RANGE = (-3600, 3600)
NAMES = ("red_phase", "green_phase", "blue_phase")


# --- the rule -------------------------------------------------------------------------------

def turn(e, phase, overflow):
    if phase == 0:
        return e
    f = phase / 360
    if overflow == "polarize":
        return (1 - math.cos(math.acos(1 - 2 * e) + math.pi * f)) / 2
    u = e + f
    if overflow == "solarize":
        return 1 - abs(u % 2 - 1)
    assert abs(u - round(u)) > 1e-6, "a wrapped value lies on a whole number"
    return u if 0 <= u <= 1 else u - math.floor(u)


def offset(px, phases, overflow):
    if all(p == 0 for p in phases):
        return [R.working(p) for p in px]
    return [R.working(p) if p[3] == 0 else
            B.back([turn(e, ph, overflow) for e, ph in zip(B.encoded(p), phases)], p)
            for p in px]


# --- the cases ------------------------------------------------------------------------------

def case(red=0, green=0, blue=0, overflow="wrap", shift=0):
    return {"red_phase": red, "green_phase": green, "blue_phase": blue, "overflow": overflow,
            "shift": shift}


def render(c, frame_no):
    phases = [min(RANGE[1], max(RANGE[0], value_at(c[k], frame_no))) for k in NAMES]
    return R.frame(offset(B.pixels(), phases, c["overflow"]), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    params = {k: setting_json(c[k]) for k in NAMES}
    params["overflow"] = c["overflow"]
    return B.project_json(fx, "core.color_offset", params, c["shift"])


CASES = {
    "FX-COFFSET-001": ("All phases 0, the settings as they start: the drawing, untouched.",
                       case(), [0]),
    "FX-COFFSET-002": ("Red phase 90, wrap: red raised a quarter of the range, and what passes "
                       "white comes round from black, so the reddest colours turn dark red.",
                       case(red=90), [0]),
    "FX-COFFSET-003": ("Red 90, green 180, blue 270, wrap: each channel turned its own way, the "
                       "colours scrambled into new ones.", case(red=90, green=180, blue=270),
                       [0]),
    "FX-COFFSET-004": ("All three -90, wrap: every value lowered a quarter, and what passes "
                       "black comes round from white, so the darks turn light.",
                       case(red=-90, green=-90, blue=-90), [0]),
    "FX-COFFSET-005": ("All three 180, solarize: values below half rise by half, values above "
                       "half are folded back down from white.",
                       case(red=180, green=180, blue=180, overflow="solarize"), [0]),
    "FX-COFFSET-006": ("All three 360, solarize: one whole turn folds every value over, the "
                       "picture's negative.",
                       case(red=360, green=360, blue=360, overflow="solarize"), [0]),
    "FX-COFFSET-007": ("All three 720, solarize: two turns bring the picture back.",
                       case(red=720, green=720, blue=720, overflow="solarize"), [0]),
    "FX-COFFSET-008": ("All three 180, polarize: like FX-COFFSET-005 but smooth, with no sharp "
                       "fold at white.",
                       case(red=180, green=180, blue=180, overflow="polarize"), [0]),
    "FX-COFFSET-009": ("All three 360, polarize: one whole turn is the negative, as solarize's.",
                       case(red=360, green=360, blue=360, overflow="polarize"), [0]),
    "FX-COFFSET-010": ("Red 45, green -135, blue 600, polarize: each channel its own way round.",
                       case(red=45, green=-135, blue=600, overflow="polarize"), [0]),
    "FX-COFFSET-011": ("Red phase keyed from 0 at frame 0 to 400 at frame 4, linear, wrap: frame "
                       "0 untouched, frame 2 red 200, frame 4 red 400.",
                       case(red=keyed((0, 0), (4, 400))), [0, 2, 4]),
    "FX-COFFSET-012": ("FX-COFFSET-003 moved three pixels right: the same, moved.",
                       case(red=90, green=180, blue=270, shift=3), [0, 3]),
}

INVALID = {
    "FX-COFFSET-013": ("Red phase 3601, above 3600.", case(red=3601)),
    "FX-COFFSET-014": ("Blue phase -3601, below -3600.", case(blue=-3601)),
    "FX-COFFSET-015": ("Overflow \"mirror\", which is not one.", case(overflow="mirror")),
    "FX-COFFSET-016": ("Overflow \"Wrap\": the word is exact, so a capital is not it.",
                       case(overflow="Wrap")),
}


def main():
    expected = B.write_cases(OUT, "color_offset", CASES, INVALID, render, plain, file_json)
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    px = B.pixels()
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    shows = [i for i in range(W * H) if px[i][3] > 0]
    at = lambda x, y: y * W + x  # noqa: E731

    def each(f, rule):
        """Every showing pixel of frame f is rule(encoded colour), back to linear."""
        return all(near(f[i], B.back(rule(B.encoded(px[i])), px[i])) for i in shows)

    # The rule's own pieces: polarize starts where it is, and both fold to the negative at one
    # turn and come back at two.
    for e in (0.0, 0.2, 0.5, 0.9, 1.0):
        assert abs(turn(e, 1e-12, "polarize") - e) < 1e-6
        for o in ("solarize", "polarize"):
            assert abs(turn(e, 360, o) - (1 - e)) < 1e-12 and abs(turn(e, 720, o) - e) < 1e-12
    assert c["FX-COFFSET-001"]["0"] == drawn
    assert each(c["FX-COFFSET-002"]["0"],
                lambda e: [e[0] + 0.25 - (e[0] > 0.75), e[1], e[2]])
    assert each(c["FX-COFFSET-004"]["0"], lambda e: [v - 0.25 + (v < 0.25) for v in e])
    assert each(c["FX-COFFSET-005"]["0"], lambda e: [v + 0.5 if v <= 0.5 else 1.5 - v for v in e])
    assert each(c["FX-COFFSET-006"]["0"], lambda e: [1 - v for v in e])
    assert each(c["FX-COFFSET-009"]["0"], lambda e: [1 - v for v in e])
    assert like(c["FX-COFFSET-007"]["0"], drawn)
    assert like(c["FX-COFFSET-006"]["0"], c["FX-COFFSET-009"]["0"])
    # Polarize is smooth where solarize folds: at the top of the fold they meet, elsewhere not.
    five, eight = c["FX-COFFSET-005"]["0"], c["FX-COFFSET-008"]["0"]
    assert near(five[at(1, 0)], eight[at(1, 0)]) and not near(five[at(3, 0)], eight[at(3, 0)])
    eleven = c["FX-COFFSET-011"]
    assert eleven["0"] == drawn and like(eleven["2"], render(case(red=200), 0))
    assert like(eleven["4"], render(case(red=400), 0))
    three, moved = c["FX-COFFSET-003"]["0"], c["FX-COFFSET-012"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == three[at(0, y):at(W - 3, y)]
    for fx, frames in c.items():
        for f in frames.values():
            for i, p in enumerate(f):
                assert p[3] == drawn[i][3] or fx == "FX-COFFSET-012"
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
    print("checked")


if __name__ == "__main__":
    main()
