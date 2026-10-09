"""Kernel, worked a second way.

`core.kernel`, modelled on CycoreFX's CC Kernel (its manual): every pixel's colour is rebuilt
from itself and its eight neighbours, each weighed by a number the user types into a three by
three grid, then the sum is divided by a divider. One grid blurs, another sharpens, another
draws edges or embosses. Nothing is ported; this is this program's own reading. Document 21 is
the rule in words; this file is the reference for the numbers document 25 pins against it.

The rule. Settings: `line_1`, `line_2` and `line_3`, three numbers each, -1000 to 1000, the
grid's top, middle and bottom rows, "0, 0, 0", "0, 1, 0" and "0, 0, 0" when added (the middle
pixel alone, which changes nothing); `divider`, 0.01 to 1000, 1 when added; `absolute_values`,
"off" or "on", "off" when added; the word is exact. CC Kernel's Blend w. Original is the Mix
every effect has (D-202): 100 less it.

- A pixel's value is its straight colour through the sRGB curve held inside 0 to 1, channel by
  channel, or 0 for a pixel that does not show (covering 0). Past the layer's border the border
  pixel repeats, as Find Edges' neighbours do.
- At a pixel (x, y) with covering a > 0, each channel's sum is
  S = sum over j, i in 0..2 of line_(j+1)[i] x value(x + i - 1, y + j - 1): line 1 weighs the
  row above, line 3 the row below, and each line's first number the pixel to the left. Then
  u = S / divider; with absolute_values "on" u becomes |u|; and u is held inside 0 to 1, so a
  negative result is black, as CC Kernel's is at 8 and 16 bits per channel. The result comes
  back to linear at the pixel's own covering (document 21's shared colour rule). The covering
  is kept; a pixel that does not show stays as it is; the layer does not grow.
- When the grid divided by the divider is the middle pixel alone (the middle number over the
  divider exactly 1, every other number 0), the layer is left exactly as it is.

**This file never runs the build's code path.** It works in double precision on lists,
straight from the drawing's 8-bit values, where the build works on its single-precision
buffers.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py): columns of colour,
darker row by row, an empty column 0 and a half-covered column 15. The projects go into
`Fixtures/kernel`, the expected frames into `Fixtures/kernel/expected_kernel.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/kernel_reference.py
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "kernel"
IDENTITY = ([0, 0, 0], [0, 1, 0], [0, 0, 0])


# --- the rule -------------------------------------------------------------------------------

def value(p):
    return B.encoded(p) if p[3] > 0 else [0.0, 0.0, 0.0]


def kernel(px, lines, divider, absolute):
    grid = [[v / divider for v in line] for line in lines]
    if grid == [[0, 0, 0], [0, 1, 0], [0, 0, 0]]:
        return [R.working(p) for p in px]
    vals = [value(p) for p in px]
    at = lambda x, y: vals[min(max(y, 0), H - 1) * W + min(max(x, 0), W - 1)]  # noqa: E731
    out = []
    for y in range(H):
        for x in range(W):
            p = px[y * W + x]
            if p[3] == 0:
                out.append(R.working(p))
                continue
            u = [sum(lines[j][i] * at(x + i - 1, y + j - 1)[c] for j in range(3) for i in range(3))
                 / divider for c in range(3)]
            if absolute == "on":
                u = [abs(v) for v in u]
            out.append(B.back(u, p))
    return out


# --- the cases ------------------------------------------------------------------------------

def case(lines=IDENTITY, divider=1, absolute="off", shift=0):
    return {"lines": [list(v) for v in lines], "divider": divider, "absolute": absolute,
            "shift": shift}


def render(c, frame_no):
    divider = min(1000, max(0.01, value_at(c["divider"], frame_no)))
    return R.frame(kernel(B.pixels(), c["lines"], divider, c["absolute"]), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    params = {f"line_{n + 1}": v for n, v in enumerate(c["lines"])}
    params["divider"] = setting_json(c["divider"])
    params["absolute_values"] = c["absolute"]
    return B.project_json(fx, "core.kernel", params, c["shift"])


BOX = ([1, 1, 1], [1, 1, 1], [1, 1, 1])
SHARPEN = ([0, -1, 0], [-1, 5, -1], [0, -1, 0])
EDGES = ([-1, -1, -1], [-1, 8, -1], [-1, -1, -1])
EMBOSS = ([-2, -1, 0], [-1, 1, 1], [0, 1, 2])
LEFT = ([0, 0, 0], [1, 0, 0], [0, 0, 0])
ABOVE = ([0, 1, 0], [0, 0, 0], [0, 0, 0])

CASES = {
    "FX-KERNEL-001": ("The grid as it starts, the middle pixel alone, divider 1: the drawing, "
                      "untouched.", case(), [0]),
    "FX-KERNEL-002": ("All nine 1, divider 9: each pixel the average of itself and its eight "
                      "neighbours, a soft blur; the columns' edges soften and the empty column "
                      "darkens its neighbour.", case(BOX, 9), [0]),
    "FX-KERNEL-003": ("0 -1 0 / -1 5 -1 / 0 -1 0, divider 1: a sharpen; flat areas stay, each "
                      "column's edge gets a light and a dark rim.", case(SHARPEN), [0]),
    "FX-KERNEL-004": ("-1 all round, 8 in the middle, divider 1, absolute values off: edges "
                      "only; flat areas turn black, and where a pixel is darker than its "
                      "neighbours the negative result is black too.", case(EDGES), [0]),
    "FX-KERNEL-005": ("The same edge grid, absolute values on: the negative results turn "
                      "positive, so both sides of every edge light up.",
                      case(EDGES, absolute="on"), [0]),
    "FX-KERNEL-006": ("-2 -1 0 / -1 1 1 / 0 1 2, divider 1: an emboss, lit from the lower "
                      "right; it shows the grid is read with line 1 above and each line's first "
                      "number on the left.", case(EMBOSS), [0]),
    "FX-KERNEL-007": ("Only line 2's first number, 1: every pixel takes the colour of the pixel "
                      "to its left, so the picture moves one pixel right; the first shown column "
                      "takes the empty column's 0, black.", case(LEFT), [0]),
    "FX-KERNEL-008": ("Only line 1's middle number, 1: every pixel takes the colour of the pixel "
                      "above, so the picture moves one pixel down; the top row repeats.",
                      case(ABOVE), [0]),
    "FX-KERNEL-009": ("The middle pixel alone, divider 2: every colour's encoded value halved.",
                      case(divider=2), [0]),
    "FX-KERNEL-010": ("The middle pixel alone, divider 0.5: every colour's encoded value "
                      "doubled, held at white.", case(divider=0.5), [0]),
    "FX-KERNEL-011": ("Middle number 4, divider keyed from 4 at frame 0 to 1 at frame 4, "
                      "linear: frame 0 untouched, frame 2 every value times 1.6, frame 4 times "
                      "4, held at white.",
                      case(([0, 0, 0], [0, 4, 0], [0, 0, 0]), keyed((0, 4), (4, 1))), [0, 2, 4]),
    "FX-KERNEL-012": ("FX-KERNEL-003 moved three pixels right: the same, moved.",
                      case(SHARPEN, shift=3), [0, 3]),
}

INVALID = {
    "FX-KERNEL-013": ("Line 1's first number 1001, above 1000.",
                      case(([1001, 0, 0], [0, 1, 0], [0, 0, 0]))),
    "FX-KERNEL-014": ("Line 2 with two numbers, not three.",
                      case(([0, 0, 0], [0, 1], [0, 0, 0]))),
    "FX-KERNEL-015": ("Divider 0, below 0.01.", case(divider=0)),
    "FX-KERNEL-016": ("Divider 1001, above 1000.", case(divider=1001)),
    "FX-KERNEL-017": ("Absolute values \"yes\", which is not a choice.", case(absolute="yes")),
    "FX-KERNEL-018": ("Absolute values \"ON\": the word is exact, so capitals are not it.",
                      case(absolute="ON")),
}


def main():
    expected = B.write_cases(OUT, "kernel", CASES, INVALID, render, plain, file_json)
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    px = B.pixels()
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731
    shows = [i for i in range(W * H) if px[i][3] > 0]
    enc = lambda p: [S.linear_to_srgb(u / p[3]) for u in p[:3]]  # noqa: E731

    assert c["FX-KERNEL-001"]["0"] == drawn
    # The blur averages: worked by hand at (4, 4).
    s = [sum(B.encoded(px[at(x, y)])[k] for x in (3, 4, 5) for y in (3, 4, 5)) / 9
         for k in range(3)]
    assert near(enc(c["FX-KERNEL-002"]["0"][at(4, 4)]), s)
    # The edge grid on its own sums to 0, so a pixel equal to the mean of its neighbours is 0.
    # Off: never above the absolute version, and equal to it wherever positive.
    four, five = c["FX-KERNEL-004"]["0"], c["FX-KERNEL-005"]["0"]
    for i in shows:
        e4, e5 = enc(four[i]), enc(five[i])
        assert all(a <= b + 1e-9 and (a < 1e-12 or abs(a - b) < 1e-9) for a, b in zip(e4, e5))
    assert any(enc(four[i])[k] < 1e-12 < enc(five[i])[k] for i in shows for k in range(3))
    # Line 2's first number moves the picture right; line 1's middle moves it down.
    seven, eight = c["FX-KERNEL-007"]["0"], c["FX-KERNEL-008"]["0"]
    for y in range(H):
        for x in range(1, W):
            if px[at(x, y)][3] == 0:
                continue
            assert near(enc(seven[at(x, y)]), B.encoded(px[at(x - 1, y)]) if x > 1 else [0] * 3)
            assert near(enc(eight[at(x, y)]), B.encoded(px[at(x, max(y - 1, 0))]))
    # The divider scales: halved, doubled and held at white, keyed.
    for i in shows:
        assert near(enc(c["FX-KERNEL-009"]["0"][i]), [v / 2 for v in B.encoded(px[i])])
        assert near(enc(c["FX-KERNEL-010"]["0"][i]), [min(1, 2 * v) for v in B.encoded(px[i])])
        assert near(enc(c["FX-KERNEL-011"]["2"][i]), [min(1, 1.6 * v) for v in B.encoded(px[i])])
        assert near(enc(c["FX-KERNEL-011"]["4"][i]), [min(1, 4 * v) for v in B.encoded(px[i])])
    assert like(c["FX-KERNEL-011"]["0"], drawn)
    # The emboss is not its own mirror: turning the grid round changes the picture.
    turned = render(case(tuple(line[::-1] for line in EMBOSS[::-1])), 0)
    assert not like(turned, c["FX-KERNEL-006"]["0"])
    three, moved = c["FX-KERNEL-003"]["0"], c["FX-KERNEL-012"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == three[at(0, y):at(W - 3, y)]
    for fx, frames in c.items():
        for f in frames.values():
            for i, p in enumerate(f):
                assert p[3] == drawn[i][3] or fx == "FX-KERNEL-012"
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
    print("checked")


if __name__ == "__main__":
    main()
