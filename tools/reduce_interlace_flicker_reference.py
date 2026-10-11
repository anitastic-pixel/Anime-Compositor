"""Reduce Interlace Flicker, worked a second way.

D-549 adds After Effects' Reduce Interlace Flicker (Stylize, an Obsolete effect) as
`core.reduce_interlace_flicker`, a second name over Gaussian Blur's engine (D-303's one-way
blur), as Levels (Individual Controls) is over Levels (D-383) and Split 2 over Split (D-394).
After Effects' one control, Softness, blurs the picture a little up and down so a thin
horizontal line no longer sits on one field of interlaced video; users describe it as "just a
slight amount of vertical gaussian blur", 0.5 to 1 being typical (Creative COW). Adobe does not
publish its kernel or what Softness measures, so the rule is ours (document 21):

`softness` is 0 to 500 (Gaussian Blur's own range), 1 when added, keyable. It is drawn as
Gaussian Blur with sigma `softness` pixels in document 21's units (reach ceil(3 sigma), the
weights normalised after that cut), Blur Dimensions vertical, edges repeat: the rows past the
top and bottom read the edge row, so the edges do not fade and the layer does not grow. The blur
works on the premultiplied linear pixels, as Gaussian Blur's does. Softness 0 changes nothing.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, summing the kernel down each column at each pixel.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py): pure colours, greys,
a skin tone, orange and three warm tones, each column darker down the rows, an empty column and
a yellow one at half covering. The projects go into `Fixtures/reduce_interlace_flicker`, the
expected frames into `Fixtures/reduce_interlace_flicker/expected_reduce_interlace_flicker.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/reduce_interlace_flicker_reference.py
"""

import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "reduce_interlace_flicker"
SOFTNESS = (0, 500)


# --- the rule -------------------------------------------------------------------------------

def taps(sigma):
    """Document 21's normalised kernel, index 0 being r pixels above the middle."""
    r = math.ceil(3 * sigma) if sigma > 0 else 0
    if r == 0:
        return [1.0]
    one = [math.exp(-(d * d) / (2 * sigma * sigma)) for d in range(-r, r + 1)]
    return [v / sum(one) for v in one]


def flicker(px, softness):
    """`px` are premultiplied linear working pixels, W by H; each column blurred down only,
    the top and bottom rows held."""
    k = taps(softness)
    r = len(k) // 2
    out = []
    for y in range(H):
        for x in range(W):
            acc = [0.0] * 4
            for j, t in enumerate(k):
                p = px[min(H - 1, max(0, y + j - r)) * W + x]
                for c in range(4):
                    acc[c] += t * p[c]
            out.append(acc)
    return out


# --- the cases ------------------------------------------------------------------------------

def case(softness=1, shift=0):
    return {"softness": softness, "shift": shift}


def render(c, frame_no):
    s = min(SOFTNESS[1], max(SOFTNESS[0], value_at(c["softness"], frame_no)))
    return R.frame(flicker([R.working(p) for p in B.pixels()], s), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    return B.project_json(fx, "core.reduce_interlace_flicker",
                          {"softness": setting_json(c["softness"])}, c["shift"])


CASES = {
    "FX-RIF-001": ("The setting as it starts: Softness 1, a Gaussian of sigma 1 down each "
                   "column reaching 3 rows. Each row takes a little of the rows above and below "
                   "it, so the darkening down each column is smoothed; the colours of "
                   "neighbouring columns never mix; the top and bottom rows read themselves past "
                   "the edge, so they do not fade; every covering is kept.", case(), [0]),
    "FX-RIF-002": ("Softness 0: the drawing exactly as it is.", case(softness=0), [0]),
    "FX-RIF-003": ("Softness 2.5: reaching 8 rows, every column far smoother than "
                   "FX-RIF-001.", case(softness=2.5), [0]),
    "FX-RIF-004": ("Softness 0.3: reaching one row, a slight softening.", case(softness=0.3),
                   [0]),
    "FX-RIF-005": ("Softness keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the "
                   "drawing, frame 2 is FX-RIF-001.",
                   case(softness=keyed((0, 0), (4, 2))), [0, 2, 4]),
    "FX-RIF-006": ("FX-RIF-001 moved three pixels right: the same, moved.", case(shift=3),
                   [0, 3]),
}

INVALID = {
    "FX-RIF-007": ("Softness 501, above 500.", case(softness=501)),
    "FX-RIF-008": ("Softness -1, below 0.", case(softness=-1)),
    "FX-RIF-009": ("Softness keyed to 600 at frame 4.", case(softness=keyed((0, 1), (4, 600)))),
}


def main():
    expected = B.write_cases(OUT, "reduce_interlace_flicker", CASES, INVALID, render, plain,
                             file_json)
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    px = [R.working(p) for p in B.pixels()]
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731

    assert taps(1) == taps(1.0) and len(taps(1)) == 7 and len(taps(2.5)) == 17
    assert len(taps(0.3)) == 3 and abs(sum(taps(2.5)) - 1) < 1e-12
    one = c["FX-RIF-001"]["0"]
    for x in range(W):
        col = [px[at(x, y)] for y in range(H)]
        for y in range(H):
            # Only the column's own pixels, and its covering kept.
            want = [sum(t * col[min(H - 1, max(0, y + j - 3))][ch] for j, t in enumerate(taps(1)))
                    for ch in range(4)]
            assert near(one[at(x, y)], want) and abs(one[at(x, y)][3] - col[y][3]) < 1e-12, (x, y)
    # A column of one colour all the way down would be untouched; this drawing darkens, so
    # every row but the black and empty columns' changes, and row 0 of white gets darker.
    assert one[at(2, 0)][0] < px[at(2, 0)][0] and near(one[at(1, 5)], px[at(1, 5)])
    assert one[at(0, 5)] == [0.0] * 4
    assert c["FX-RIF-002"]["0"] == drawn
    three = c["FX-RIF-003"]["0"]
    assert abs(three[at(2, 0)][0] - px[at(2, 0)][0]) > abs(one[at(2, 0)][0] - px[at(2, 0)][0])
    four = c["FX-RIF-004"]["0"]
    assert abs(four[at(2, 4)][0] - px[at(2, 4)][0]) < abs(one[at(2, 4)][0] - px[at(2, 4)][0])
    five = c["FX-RIF-005"]
    assert five["0"] == drawn and like(five["2"], one)
    moved = c["FX-RIF-006"]
    assert moved["0"] == moved["3"]
    for y in range(H):
        assert moved["0"][at(3, y):at(0, y + 1)] == one[at(0, y):at(W - 3, y)]
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-RIF-006" else 0))
        for fr in frames.values():
            for i, p in enumerate(fr):
                assert abs(p[3] - base[i][3]) < 1e-12, (fx, i)
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), (fx, i)
    print("checked")


if __name__ == "__main__":
    main()
