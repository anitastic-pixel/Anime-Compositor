"""Tile Wipe, worked a second way.

D-510 adds Tile Wipe (`core.tile_wipe`), after the purpose of CycoreFX's CC Grid Wipe
(Transition, EFFECTS.md row 272), under a name of our own. The CycoreFX HD 1.8.1 manual says the
effect "cuts the source layer into squares and wipes them off the screen", starting "from a
user-definable center point" and moving "evenly out towards the edge of the source layer", and
names the controls: Completion, Center, Rotation ("the entire grid rotates, not individual
squares"), Border ("the size of the transition border"), Tiles ("the total number of tiles in
the grid"), Shape (Doors, "splits the source layer into two parts"; Radial; Rectangle) and
Reverse Transition. It publishes no formula and no settings when added, so the rule and those
settings are ours (document 21, D-510).

The settings. `completion`, 0 to 100, 0 when added; `center`, x then y, -1000 to 1000 per cent
of the drawing's own width and height, [50, 50] when added; `rotation`, -3600 to 3600 degrees, 0
when added; `border`, 0 to 100 pixels, 0 when added; `tiles`, 1 to 200, the tiles across the
drawing's width, 10 when added; all five keyable. `shape`, "doors", "radial" (when added) or
"rectangle", exact; `reverse`, "off" (when added) or "on", exact.

The rule, in the drawing's own W by H space. The centre C = (center_x W / 100, center_y H / 100)
and the tile size s = W / tiles. At the pixel's centre P = (x + 1/2, y + 1/2), q is P - C turned
by -rotation (the grid turned by rotation about C). The tile holding q is i = floor(q / s + 1/2)
on each axis, its centre t = i s, and the pixel's place in it l = q - t, m = max(|l_x|, |l_y|).
The tile's distance d from the centre: radial |t|, rectangle max(|t_x|, |t_y|), doors |t_x|; the
furthest dmax the same measure of the drawing's four corners turned the same way. The tile's turn
u = min(d / dmax, 1) (0 when dmax is 0), 1 - u with reverse on. With c = completion / 100 and the
spread L = 1/4, the tile's progress p = clamp((c - u (1 - L)) / L, 0, 1): the tiles nearest the
centre go first, and each takes a quarter of the transition to go. The tile shrinks about its
centre: with border b, its half size a = (1 - p)(s / 2 + b) - b / 2, and the pixel keeps
k = clamp((a - m) / b + 1/2, 0, 1) of itself; with border 0, k = 1 where m < a, else 0. Every
channel of the premultiplied pixel is multiplied by k. At completion 0 every k is 1; at 100,
0.

The layer never grows. A draft divides `border` (a distance); the tile size is a share of the
drawing's width.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values. Every case is checked that no pixel centre lies on a tile's
side, and with border 0 that none lies near a shrinking tile's edge.

The drawing is Broadcast Colors' (tools/broadcast_safe_reference.py), 16 by 10, with transparent
columns and a half-covered one. The projects go into `Fixtures/tile_wipe`, the expected frames
into `Fixtures/tile_wipe/expected_tile_wipe.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/tile_wipe_reference.py
"""

import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "tile_wipe"
NUMBERS = ("completion", "center", "rotation", "border", "tiles")
WORDS = ("shape", "reverse")
SPREAD = 0.25


# --- the rule -------------------------------------------------------------------------------

def turned(k, px, py):
    """P - C turned by -rotation."""
    cx, cy = k["center"][0] / 100 * W, k["center"][1] / 100 * H
    r = math.radians(k["rotation"])
    dx, dy = px - cx, py - cy
    return (dx * math.cos(r) + dy * math.sin(r), -dx * math.sin(r) + dy * math.cos(r))


def measure(shape, t):
    if shape == "radial":
        return math.hypot(t[0], t[1])
    if shape == "rectangle":
        return max(abs(t[0]), abs(t[1]))
    return abs(t[0])


def place(k, x, y):
    """(q / s + 1/2, the tile centre t, m) at pixel (x, y)."""
    s = W / k["tiles"]
    q = turned(k, x + 0.5, y + 0.5)
    f = [v / s + 0.5 for v in q]
    t = [math.floor(v) * s for v in f]
    m = max(abs(q[0] - t[0]), abs(q[1] - t[1]))
    return f, t, m, s


def half(k, x, y):
    """The shrinking tile's half size a and the pixel's m."""
    _, t, m, s = place(k, x, y)
    dmax = max(measure(k["shape"], turned(k, cx, cy)) for cx in (0, W) for cy in (0, H))
    u = min(measure(k["shape"], t) / dmax, 1.0) if dmax > 0 else 0.0
    if k["reverse"] == "on":
        u = 1 - u
    c = k["completion"] / 100
    p = min(max((c - u * (1 - SPREAD)) / SPREAD, 0.0), 1.0)
    b = k["border"]
    return (1 - p) * (s / 2 + b) - b / 2, m


def keep(k, x, y):
    a, m = half(k, x, y)
    b = k["border"]
    if b == 0:
        return 1.0 if m < a else 0.0
    return min(max((a - m) / b + 0.5, 0.0), 1.0)


def tile_wipe(px, k):
    out = []
    for n, w in enumerate(px):
        f = keep(k, n % W, n // W)
        out.append([v * f for v in w])
    return out


def working():
    return [R.working(p) for p in B.pixels()]


# --- the cases ------------------------------------------------------------------------------

def case(shape="radial", reverse="off", shift=0, **numbers):
    c = {"shape": shape, "reverse": reverse, "shift": shift, "completion": 0,
         "center": [50, 50], "rotation": 0, "border": 0, "tiles": 10}
    for n, v in numbers.items():
        assert n in NUMBERS, n
        c[n] = v
    return c


def settings(c, frame_no):
    k = {n: value_at(c[n], frame_no) for n in NUMBERS}
    k.update(shape=c["shape"], reverse=c["reverse"])
    return k


def render(c, frame_no):
    return R.frame(tile_wipe(working(), settings(c, frame_no)), c["shift"])


def plain(c):
    return R.frame(working(), c["shift"])


def file_json(fx, c):
    params = {w: c[w] for w in WORDS}
    params.update({n: setting_json(c[n]) for n in NUMBERS})
    return B.project_json(fx, "core.tile_wipe", params, c["shift"])


CASES = {
    "FX-TILE-001": ("The settings as they start: completion 0, so nothing changes.", case(), [0]),
    "FX-TILE-002": ("Completion 100: every tile gone, every pixel clear.",
                    case(completion=100), [0]),
    "FX-TILE-003": ("Completion 50, tiles 4 (tiles 4 pixels wide), radial: the tiles round the "
                    "middle gone, the next ring shrinking, the corners whole.",
                    case(completion=50, tiles=4), [0]),
    "FX-TILE-004": ("FX-TILE-003 with shape rectangle: the tiles go in square rings.",
                    case("rectangle", completion=50, tiles=4), [0]),
    "FX-TILE-005": ("FX-TILE-003 with shape doors: the tiles go in columns from the middle out, "
                    "splitting the layer in two.", case("doors", completion=50, tiles=4), [0]),
    "FX-TILE-006": ("FX-TILE-003 reversed: the corners go first, the middle stays.",
                    case(reverse="on", completion=50, tiles=4), [0]),
    "FX-TILE-007": ("FX-TILE-003 with border 2: each shrinking tile's edge soft over 2 pixels.",
                    case(completion=50, tiles=4, border=2), [0]),
    "FX-TILE-008": ("FX-TILE-003 with rotation 30: the whole grid turned, not each tile.",
                    case(completion=50, tiles=4, rotation=30), [0]),
    "FX-TILE-009": ("Center [0, 0], completion 50, tiles 4: the wipe starts from the top-left "
                    "corner.", case(completion=50, tiles=4, center=[0, 0]), [0]),
    "FX-TILE-010": ("Tiles 1, completion 10: one tile the drawing's width, four tenths of the "
                    "way gone, shrunk about the middle.", case(completion=10, tiles=1), [0]),
    "FX-TILE-011": ("Tiles 2.5, completion 60, rectangle: tile 6.4 pixels wide, so the tiles "
                    "need not fit the drawing.", case("rectangle", completion=60, tiles=2.5), [0]),
    "FX-TILE-012": ("Completion keyed from 0 at frame 0 to 100 at frame 4, linear, tiles 4, "
                    "border 1: frame 2 is completion 50.",
                    case(completion=keyed((0, 0), (4, 100)), tiles=4, border=1), [0, 2, 4]),
    "FX-TILE-013": ("Rotation keyed from 0 at frame 0 to 40 at frame 4, linear, completion 50, "
                    "tiles 4: frame 2 is rotation 20.",
                    case(completion=50, tiles=4, rotation=keyed((0, 0), (4, 40))), [0, 2, 4]),
    "FX-TILE-014": ("Center keyed from [25, 50] at frame 0 to [75, 50] at frame 4, linear, "
                    "completion 40, tiles 4, rectangle.",
                    case("rectangle", completion=40, tiles=4,
                         center=keyed((0, (25, 50)), (4, (75, 50)))), [0, 4]),
    "FX-TILE-015": ("FX-TILE-007 moved three pixels right: the same, moved.",
                    case(completion=50, tiles=4, border=2, shift=3), [0, 3]),
    "FX-TILE-016": ("Doors reversed, completion 70, tiles 8, border 0.5: the outer columns gone, "
                    "a soft middle left.",
                    case("doors", "on", completion=70, tiles=8, border=0.5), [0]),
}

INVALID = {
    "FX-TILE-017": ("Completion 101, above 100.", case(completion=101)),
    "FX-TILE-018": ("Completion -1, below 0.", case(completion=-1)),
    "FX-TILE-019": ("Center x 1001, above 1000.", case(center=[1001, 50])),
    "FX-TILE-020": ("Rotation 3601, above 3600.", case(rotation=3601)),
    "FX-TILE-021": ("Border -1, below 0.", case(border=-1)),
    "FX-TILE-022": ("Border 101, above 100.", case(border=101)),
    "FX-TILE-023": ("Tiles 0.5, below 1.", case(tiles=0.5)),
    "FX-TILE-024": ("Tiles 201, above 200.", case(tiles=201)),
    "FX-TILE-025": ("Shape \"Radial\": the word is exact, so a capital is not it.",
                    case("Radial")),
    "FX-TILE-026": ("Shape \"star\": not a shape of Tile Wipe.", case("star")),
    "FX-TILE-027": ("Reverse \"yes\": \"on\" or \"off\".", case(reverse="yes")),
    "FX-TILE-028": ("Completion keyed to 200 at frame 4: a key outside the range.",
                    case(completion=keyed((0, 0), (4, 200)))),
}


def main():
    clear()
    expected = B.write_cases(OUT, "tile_wipe", CASES, INVALID, render, plain, file_json)
    check(expected)


def clear():
    """No pixel centre on a tile's side; with border 0, none near a shrinking tile's edge."""
    for fx, (_, c, frames) in CASES.items():
        for f in frames:
            k = settings(c, f)
            for y in range(H):
                for x in range(W):
                    g, _, _, _ = place(k, x, y)
                    assert all(abs(v - round(v)) > 1e-4 for v in g), (fx, f, x, y)
                    if k["border"] == 0:
                        a, m = half(k, x, y)
                        assert abs(a - m) > 1e-4, (fx, f, x, y)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    px = working()
    clear_px = [[0.0, 0.0, 0.0, 0.0]] * (W * H)
    assert c["FX-TILE-001"]["0"] == R.frame(px, 0)
    assert c["FX-TILE-002"]["0"] == R.frame(clear_px, 0)
    three = settings(case(completion=50, tiles=4), 0)
    ks = [keep(three, n % W, n // W) for n in range(W * H)]
    assert ks[5 * W + 8] == 0 and ks[0] == 1 and any(0 < half(three, n % W, n // W)[0] < 2 for n in range(W * H))
    assert len({c[f]["0"].__repr__() for f in ("FX-TILE-003", "FX-TILE-004", "FX-TILE-005", "FX-TILE-006", "FX-TILE-008")}) == 5
    six = [keep(settings(case(reverse="on", completion=50, tiles=4), 0), n % W, n // W) for n in range(W * H)]
    assert six[5 * W + 8] == 1 and six[0] == 0
    seven = [keep(settings(case(completion=50, tiles=4, border=2), 0), n % W, n // W) for n in range(W * H)]
    assert any(0 < v < 1 for v in seven)
    assert c["FX-TILE-012"]["2"] == render(case(completion=50, tiles=4, border=1), 0)
    assert c["FX-TILE-012"]["4"] == R.frame(clear_px, 0)
    assert c["FX-TILE-013"]["2"] == render(case(completion=50, tiles=4, rotation=20), 0)
    moved, still = c["FX-TILE-015"]["0"], c["FX-TILE-007"]["0"]
    for y in range(H):
        assert moved[y * W + 3:(y + 1) * W] == still[y * W:y * W + W - 3]
    print("checked")


if __name__ == "__main__":
    main()
