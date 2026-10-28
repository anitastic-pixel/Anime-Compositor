"""Minimax, worked a second way.

`core.minimax`, after After Effects' Minimax (Channel). Adobe's help page ("Channel effects",
helpx.adobe.com/after-effects/using/channel-effects.html, read through the Wayback Machine on
2026-10-27) says: "The Minimax effect assigns each channel of a pixel the minimum or maximum value
for that channel found within a specified radius ... a white solid area surrounded by black
shrinks one pixel on each side using Minimum and a radius of 1." Operation: "Minimum ...
Maximum ... Minimum Then Maximum performs the Minimum operation and then the Maximum operation.
Maximum Then Minimum performs the Maximum operation and then the Minimum operation." Direction:
"The axis along which to scan for values. Horizontal & Vertical scans all directions." Adobe's
feature request "Minimax kernel shape option" (community.adobe.com, 2025) says the neighbourhood
is a square. The controls and their order are After Effects': Operation, Radius, Channel,
Direction, Don't Shrink Edges (w12b video 1 at 6:25 uses it as F's Max).

The settings. `operation`, "maximum" (when added), "minimum", "min_then_max" or "max_then_min";
`radius`, 0 to 500 pixels, 0 when added, keyable, used in whole pixels (rounded half up, so 1.5
is 2); `channel`, "color" (when added: red, green and blue), "alpha_and_color", "alpha", "red",
"green" or "blue"; `direction`, "both" (when added: Horizontal & Vertical, the square),
"horizontal" or "vertical"; `dont_shrink_edges`, "off" (when added) or "on". Every word exact.

The rule (ours where Adobe is silent, document 21). The layer's straight colour (linear light,
colour over covering; 0 where nothing shows) and its covering. Each chosen channel of each pixel
becomes the largest (maximum) or smallest (minimum) value of that channel over the window: the
(2r + 1) square about the pixel, or the row of 2r + 1 (horizontal), or the column (vertical).
- The covering is taken over every pixel of the window inside the layer, a clear one counting 0.
  Past the layer's edge a pixel counts as clear (covering 0), so Minimum eats in from the edge;
  with Don't Shrink Edges on, the pixels past the edge are left out instead.
- A colour channel is taken over the pixels of the window that show (covering above 0); a pixel
  whose window holds none keeps its own.
- Min then Max is the minimum over the window, then the maximum of that over the window again
  (and the other way); each pass reads the one before.
The colour unchosen keeps its own; the result is the straight colour times the new covering. The
layer keeps its size. Because the sRGB curve only rises, the largest or smallest straight value is
the same pixel's in linear light as in encoded 8-bit, so this matches After Effects' 8-bit
choice exactly, where the covering is the same.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, by the plain definition (every window searched in full), where
the build slides a running extreme along rows and columns.

The drawing (16 by 10): a block of colours, red falling and green rising, in columns 0 to 4 on
every row, so it touches the left, top and bottom edges; a blue column at half covering in column
6, rows 3 to 6; a white box in columns 8 to 12, rows 2 to 7, with one black pixel at (10, 4); and
one yellow pixel alone at (14, 1). Everything else is clear.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/minimax_reference.py
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "minimax"
WORDS = ("operation", "channel", "direction", "dont_shrink_edges")
CHANNELS = {"color": (0, 1, 2), "alpha_and_color": (0, 1, 2, 3), "alpha": (3,),
            "red": (0,), "green": (1,), "blue": (2,)}


def pixel(x, y):
    if x <= 4:
        return (255 - 22 * y, 30 + 45 * x, 40, 255)
    if x == 6 and 3 <= y <= 6:
        return (40, 60, 230, 128)
    if 8 <= x <= 12 and 2 <= y <= 7:
        return (0, 0, 0, 255) if (x, y) == (10, 4) else (255, 255, 255, 255)
    if (x, y) == (14, 1):
        return (255, 230, 0, 255)
    return S.NONE


B.DRAWINGS["colours"] = [[pixel(x, y) for x in range(W)] for y in range(H)]


# --- the rule -------------------------------------------------------------------------------

def straight(px):
    return [[v / w[3] for v in w[:3]] + [w[3]] if w[3] > 0 else [0.0, 0.0, 0.0, 0.0]
            for w in px]


def window(x, y, r, direction):
    xs = range(x - r, x + r + 1) if direction != "vertical" else range(x, x + 1)
    ys = range(y - r, y + r + 1) if direction != "horizontal" else range(y, y + 1)
    return [(u, v) for v in ys for u in xs]


def one_pass(st, r, pick, chans, direction, keep_edges):
    out = []
    for y in range(H):
        for x in range(W):
            own = st[y * W + x]
            new = list(own)
            taps = window(x, y, r, direction)
            inside = [st[v * W + u] for u, v in taps if 0 <= u < W and 0 <= v < H]
            outside = len(inside) < len(taps)
            for ch in chans:
                if ch == 3:
                    vals = [p[3] for p in inside] + ([0.0] if outside and not keep_edges else [])
                    new[3] = pick(vals)
                else:
                    vals = [p[ch] for p in inside if p[3] > 0]
                    if vals:
                        new[ch] = pick(vals)
            out.append(new)
    return out


def minimax(px, k):
    st = straight(px)
    r = int(k["radius"] + 0.5)
    if r == 0:
        return px
    chans = CHANNELS[k["channel"]]
    picks = {"maximum": [max], "minimum": [min], "min_then_max": [min, max],
             "max_then_min": [max, min]}[k["operation"]]
    for pick in picks:
        st = one_pass(st, r, pick, chans, k["direction"], k["dont_shrink_edges"] == "on")
    return [[v * p[3] for v in p[:3]] + [p[3]] if p[3] > 0 else [0.0, 0.0, 0.0, 0.0]
            for p in st] if 3 in chans else \
        [[v * w[3] for v in p[:3]] + [w[3]] for p, w in zip(st, px)]


def working():
    return [R.working(p) for p in B.pixels()]


# --- the cases ------------------------------------------------------------------------------

def case(operation="maximum", radius=0, channel="color", direction="both",
         dont_shrink_edges="off", shift=0):
    return {"operation": operation, "radius": radius, "channel": channel,
            "direction": direction, "dont_shrink_edges": dont_shrink_edges, "shift": shift}


def settings(c, frame_no):
    k = {w: c[w] for w in WORDS}
    k["radius"] = min(500, max(0, value_at(c["radius"], frame_no)))
    return k


def render(c, frame_no):
    return R.frame(minimax(working(), settings(c, frame_no)), c["shift"])


def plain(c):
    return R.frame(working(), c["shift"])


def file_json(fx, c):
    params = {w: c[w] for w in WORDS}
    params["radius"] = setting_json(c["radius"])
    return B.project_json(fx, "core.minimax", params, c["shift"])


CASES = {
    "FX-MINIMAX-001": ("The settings as they start: Maximum, radius 0, Color, Horizontal & "
                       "Vertical: nothing changes.", case(), [0]),
    "FX-MINIMAX-002": ("Maximum, radius 1, Color: each colour channel becomes the largest of "
                       "the 3 by 3 square's showing pixels; the covering is untouched, so the "
                       "white box's black pixel turns white and the shapes keep their outline.",
                       case(radius=1), [0]),
    "FX-MINIMAX-003": ("Minimum, radius 1, Color: the smallest of each channel; the white box's "
                       "black pixel spreads to a 3 by 3 black square.",
                       case("minimum", 1), [0]),
    "FX-MINIMAX-004": ("Maximum, radius 2, Alpha and Color: every shape grows two pixels each "
                       "way in a square, the new pixels taking the largest showing colour (F's "
                       "Max, w12b video 1 at 6:25).", case(radius=2, channel="alpha_and_color"),
                       [0]),
    "FX-MINIMAX-005": ("Minimum, radius 1, Alpha: every shape's covering shrinks a pixel each "
                       "side, the lone yellow pixel and the half-covered column go, and the block "
                       "on the left loses its pixels along the layer's edges too, since past the "
                       "edge counts as clear.", case("minimum", 1, "alpha"), [0]),
    "FX-MINIMAX-006": ("FX-MINIMAX-005 with Don't Shrink Edges on: the same, but the block keeps "
                       "its pixels along the layer's left, top and bottom edges.",
                       case("minimum", 1, "alpha", dont_shrink_edges="on"), [0]),
    "FX-MINIMAX-007": ("Minimum Then Maximum, radius 1, Alpha and Color: the lone yellow pixel "
                       "and the half-covered column are taken away and the big shapes come back "
                       "to their size.", case("min_then_max", 1, "alpha_and_color"), [0]),
    "FX-MINIMAX-008": ("Maximum Then Minimum, radius 1, Alpha and Color: the white box's black "
                       "pixel is filled in and the shapes come back to their size.",
                       case("max_then_min", 1, "alpha_and_color"), [0]),
    "FX-MINIMAX-009": ("Maximum, radius 2, Red, Just Horizontal: only red changes, the largest "
                       "of five along the row.", case(radius=2, channel="red",
                                                      direction="horizontal"), [0]),
    "FX-MINIMAX-010": ("Minimum, radius 1, Green, Just Vertical: only green changes, the "
                       "smallest of three down the column.",
                       case("minimum", 1, "green", "vertical"), [0]),
    "FX-MINIMAX-011": ("Maximum, radius 1, Blue: only blue changes, so the blue column's blue "
                       "spills into its showing neighbours.", case(radius=1, channel="blue"),
                       [0]),
    "FX-MINIMAX-012": ("Radius keyed from 0 at frame 0 to 4 at frame 4, linear, Maximum, Alpha "
                       "and Color: the shapes grow a pixel a frame.",
                       case(radius=keyed((0, 0), (4, 4)), channel="alpha_and_color"),
                       [0, 2, 4]),
    "FX-MINIMAX-013": ("FX-MINIMAX-005 with the layer moved three pixels right: the same, moved; "
                       "the layer's own edges still count.", case("minimum", 1, "alpha", shift=3),
                       [0]),
    "FX-MINIMAX-014": ("Radius 1.5 is used as 2 (whole pixels, half up), radius 2.4 as 2: both "
                       "frames equal FX-MINIMAX-004's.",
                       case(radius=keyed((0, 1.5), (4, 2.4)), channel="alpha_and_color"),
                       [0, 4]),
}

INVALID = {
    "FX-MINIMAX-015": ("Radius 501, above 500.", case(radius=501)),
    "FX-MINIMAX-016": ("Radius -1, below 0.", case(radius=-1)),
    "FX-MINIMAX-017": ("Operation \"max\": not one of the four words.", case("max")),
    "FX-MINIMAX-018": ("Channel \"rgb\": not one of the six words.", case(channel="rgb")),
    "FX-MINIMAX-019": ("Direction \"Horizontal\": the word is exact, so a capital is not it.",
                       case(direction="Horizontal")),
    "FX-MINIMAX-020": ("Don't Shrink Edges \"yes\": not on or off.",
                       case(dont_shrink_edges="yes")),
}


def main():
    expected = B.write_cases(OUT, "minimax", CASES, INVALID, render, plain, file_json)
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    px = working()
    at = lambda x, y: y * W + x  # noqa: E731
    gone = lambda p: all(v == 0 for v in p)  # noqa: E731
    assert c["FX-MINIMAX-001"]["0"] == px
    two = c["FX-MINIMAX-002"]["0"]
    assert all(p[3] == q[3] for p, q in zip(two, px))
    assert two[at(10, 4)] == px[at(9, 4)]
    three = c["FX-MINIMAX-003"]["0"]
    assert all(three[at(x, y)][:3] == [0, 0, 0] for x in (9, 10, 11) for y in (3, 4, 5))
    four = c["FX-MINIMAX-004"]["0"]
    assert four[at(7, 0)][3] == 1 and four[at(6, 0)][3] == 1 and four[at(15, 9)][3] == 0
    assert four[at(15, 3)][3] == 1 and four[at(15, 4)][3] == 0
    five, six = c["FX-MINIMAX-005"]["0"], c["FX-MINIMAX-006"]["0"]
    assert gone(five[at(14, 1)]) and all(gone(five[at(6, y)]) for y in range(H))
    assert gone(five[at(0, 5)]) and gone(five[at(2, 0)]) and five[at(2, 5)][3] == 1
    assert six[at(0, 5)][3] == 1 and six[at(2, 0)][3] == 1 and gone(six[at(4, 5)])
    seven = c["FX-MINIMAX-007"]["0"]
    assert gone(seven[at(14, 1)]) and gone(seven[at(6, 4)]) and seven[at(12, 7)][3] == 1
    eight = c["FX-MINIMAX-008"]["0"]
    assert eight[at(10, 4)] == px[at(9, 4)] and gone(eight[at(13, 4)])
    nine = c["FX-MINIMAX-009"]["0"]
    assert all(p[1:] == q[1:] for p, q in zip(nine, px))
    ten = c["FX-MINIMAX-010"]["0"]
    assert all(p[0] == q[0] and p[2:] == q[2:] for p, q in zip(ten, px))
    eleven = c["FX-MINIMAX-011"]["0"]
    assert all(p[:2] == q[:2] and p[3] == q[3] for p, q in zip(eleven, px))
    twelve = c["FX-MINIMAX-012"]
    assert twelve["0"] == px
    assert twelve["2"] == render(case(radius=2, channel="alpha_and_color"), 0) == four
    moved = c["FX-MINIMAX-013"]["0"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == five[at(0, y):at(W - 3, y)]
    assert c["FX-MINIMAX-014"]["0"] == four and c["FX-MINIMAX-014"]["4"] == four
    print("checked")


if __name__ == "__main__":
    main()
