"""Tiles, worked a second way.

D-404 adds `core.tiles`, after CycoreFX's CC Tiler (After Effects' Distort group): the layer's
picture shrunk and repeated, tile against tile, inside the layer's own bounds, then mixed back
with the original. CycoreFX publishes no formula; the rule is this program's own, Motion Tile's
sized tile (D-304) with its output held at the layer's size and its tiles the same per cent
across as down.

The settings. `scale`, 1 to 100 per cent, 50 when added, each tile's size against the picture;
`center`, two numbers, per cent of the picture as it reaches the effect, -1000 to 1000 each,
(50, 50) when added, where one tile sits; `blend`, 0 to 100 per cent, 0 when added, how much of
the original is mixed back over the tiles. All three are keyable. CC Tiler's Scale runs from 0
(a tutorial read for D-404 says 0 per cent still shows the picture, as fine dots); below 1 per
cent is not reproduced and is outside this contract.

The rule. On the picture as it reaches the effect, w by h pixels (an earlier effect's grown
pixels included), s = scale / 100, and across (down likewise, with h and the centre's second
number): the tiles are T = w s long, one of them starting at c / 100 w - T / 2. A place p of the
row, counted from the picture's left edge, reads the picture at

    u = p - (c / 100 w - T / 2),   k = floor(u / T),   f = u - k T,   q = clamp(f / s, 1/2, w - 1/2).

Output pixel (x, y) is the mean of mx by my points, mx = clamp(ceil(1 / s), 1, 16) and my
likewise, the point (a, b) at p = x + (a + 1/2) / mx across and y + (b + 1/2) / my down, each
point document 21's bilinear sample of the picture at (q across, q down). Then, with B = blend /
100, the pixel is the tiles' times (1 - B) plus the original's times B, every channel. Scale 100
with the centre at (50, 50), or blend 100, leaves the picture as it was. The layer never grows.
The settings are per cents, not distances: a draft does not scale them.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, drawn below,
unmoved unless the case says. The drawing goes into `Fixtures/tiles/media`, the projects into
`Fixtures/tiles`, and the expected frames into `Fixtures/tiles/expected_tiles.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/tiles_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402

W, H = R.W, R.H  # the composition and the drawing, 16 by 10
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "tiles"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"scale": (1, 100), "center": (-1000, 1000), "blend": (0, 100)}
NAMES = ("scale", "center", "blend")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def read(p, n, c, s):
    """Where place p of a row (or column) n long reads the picture: tiles n s long, one
    starting at c / 100 n - n s / 2, held inside the picture's pixel centres."""
    t = n * s
    u = p - (c / 100 * n - t / 2)
    k = math.floor(u / t)
    f = u - k * t
    return min(max(f / s, 0.5), n - 0.5)


def points(s):
    return min(max(math.ceil(1 / s), 1), 16)


def tiles(layer, scale, center, blend):
    w, h = layer["w"], layer["h"]
    s = scale / 100
    mx = my = points(s)
    b = blend / 100
    out = []
    for y in range(h):
        for x in range(w):
            acc = [0.0] * 4
            for j in range(my):
                qy = read(y + (j + 0.5) / my, h, center[1], s)
                for i in range(mx):
                    qx = read(x + (i + 0.5) / mx, w, center[0], s)
                    p = bilinear(layer, qx, qy)
                    for ch in range(4):
                        acc[ch] += p[ch]
            o = layer["px"][y * w + x]
            out.append([acc[ch] / (mx * my) * (1 - b) + o[ch] * b for ch in range(4)])
    return {"px": out, "left": layer["left"], "top": layer["top"], "w": w, "h": h}


# --- the drawing ----------------------------------------------------------------------------

LINE, SKIN, TRACE, NONE = R.LINE, R.SKIN, R.TRACE, S.NONE  # #1e1a24, #f6d6be, #c82828
SOFT_SKIN = SKIN[:3] + (128,)        # the skin at half covering, a soft edge
BAND = (58, 111, 216, 255)           # #3a6fd8, the ball's blue band


def card(x, y):
    """Sixteen by ten, alike in no two directions, so a tile slid, shrunk or turned shows: skin,
    a red dot at (3, 2), a line down column 11, a blue band along row 7, the skin at half
    covering down column 0, and (14, 0), (15, 0) and (15, 1) empty."""
    if (x, y) in ((14, 0), (15, 0), (15, 1)):
        return NONE
    if (x, y) == (3, 2):
        return TRACE
    if y == 7:
        return BAND
    if x == 11:
        return LINE
    if x == 0:
        return SOFT_SKIN
    return SKIN


DRAWINGS = {"card": [[card(x, y) for x in range(W)] for y in range(H)]}


def drawn_layer(name):
    return {"px": [R.working(p) for row in DRAWINGS[name] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


# --- the cases ------------------------------------------------------------------------------

def case(scale=50, center=(50, 50), blend=0, shift=0):
    return {"drawing": "card", "scale": scale, "center": center, "blend": blend, "shift": shift}


def settings(c, frame_no):
    held = {}
    for k in NAMES:
        lo, hi = RANGES[k]
        v = value_at(c[k], frame_no)
        held[k] = ([min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple))
                   else min(hi, max(lo, v)))
    return held


def placed(layer, shift):
    return [list(layer["px"][y * W + x - shift]) if 0 <= x - shift < W else EMPTY
            for y in range(H) for x in range(W)]


def render(c, frame_no):
    s_ = settings(c, frame_no)
    return placed(tiles(drawn_layer(c["drawing"]), s_["scale"], s_["center"], s_["blend"]),
                  c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return placed(drawn_layer(c["drawing"]), c["shift"])


CASES = {
    "FX-TILES-001": ("The settings as they start, scale 50, centre 50, 50, blend 0: tiles 8 by "
                     "5 set round the middle, one starting at column 4 and row 2.5, so the "
                     "frame holds two tiles across and two down, cut at the edges, each pixel "
                     "the mean of 2 by 2 points.",
                     case(), [0, 4]),
    "FX-TILES-002": ("Scale 100 with the centre at 50, 50: the one tile is the picture itself, "
                     "untouched.", case(scale=100), [0]),
    "FX-TILES-003": ("Scale 100 with the centre at 25, 50: the picture slid four columns left, "
                     "the four columns that fall off the left coming round on the right, "
                     "pixel for pixel: column x is the drawing's column (x + 4) mod 16.",
                     case(scale=100, center=(25, 50)), [0]),
    "FX-TILES-004": ("Scale 25: tiles 4 by 2.5, one starting at column 6 and row 3.75, each "
                     "pixel the mean of 4 by 4 points; across, the frame repeats every 4 "
                     "columns.", case(scale=25), [0]),
    "FX-TILES-005": ("Scale 75: tiles 12 by 7.5, more than half the picture, so the tiles' "
                     "edges show near the frame's sides, each pixel the mean of 2 by 2 "
                     "points.", case(scale=75), [0]),
    "FX-TILES-006": ("Scale 33.3: 1 / 0.333 is just over 3, so 4 by 4 points a pixel; tiles "
                     "5.328 by 3.33.", case(scale=33.3), [0]),
    "FX-TILES-007": ("Scale 1, the least: tiles 0.16 by 0.1 of a pixel, each pixel the mean of "
                     "16 by 16 points, the most there are, so the picture becomes a fine "
                     "even mixture.", case(scale=1), [0]),
    "FX-TILES-008": ("Scale 50 with the centre at 25, 50: the tiles start at column 0 instead "
                     "of 4, so the frame is FX-TILES-001's slid four columns left, wrapping.",
                     case(center=(25, 50)), [0]),
    "FX-TILES-009": ("Blend 50: halfway between FX-TILES-001's tiles and the drawing, every "
                     "channel.", case(blend=50), [0]),
    "FX-TILES-010": ("Blend 100: the drawing, untouched.", case(blend=100), [0]),
    "FX-TILES-011": ("Scale keyed from 100 at frame 0 to 50 at frame 4, linear: frame 0 the "
                     "drawing, frame 2 scale 75 (FX-TILES-005), frame 4 FX-TILES-001.",
                     case(scale=keyed((0, 100), (4, 50))), [0, 2, 4]),
    "FX-TILES-012": ("Blend keyed from 100 at frame 0 to 0 at frame 4: frame 0 the drawing, "
                     "frame 2 FX-TILES-009, frame 4 FX-TILES-001.",
                     case(blend=keyed((0, 100), (4, 0))), [0, 2, 4]),
    "FX-TILES-013": ("Centre keyed from 50, 50 at frame 0 to 25, 50 at frame 4 at scale 100: "
                     "the picture slides left and wraps, two columns at frame 2.",
                     case(scale=100, center=keyed((0, (50, 50)), (4, (25, 50)))), [0, 2, 4]),
    "FX-TILES-014": ("Scale eased from 50 at frame 0 to 1 at frame 4 on a curve that "
                     "overshoots: at frame 2 it would pass below 1 and is held there, so frames "
                     "2 and 4 are FX-TILES-007.",
                     case(scale=keyed((0, 50, OVERSHOOT), (4, 1))), [0, 2, 4]),
    "FX-TILES-015": ("FX-TILES-001 with the layer moved three pixels right: the same tiles, "
                     "moved; nothing grows, so the three columns past the right edge are cut "
                     "and columns 0 to 2 stay empty.", case(shift=3), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-TILES-016": ("Scale 0.5, below 1.", case(scale=0.5)),
    "FX-TILES-017": ("Scale 100.5, above 100.", case(scale=100.5)),
    "FX-TILES-018": ("Blend -1, below 0.", case(blend=-1)),
    "FX-TILES-019": ("Blend 101, above 100.", case(blend=101)),
    "FX-TILES-020": ("Centre at 1001, 50, past ten widths.", case(center=(1001, 50))),
    "FX-TILES-021": ("Scale keyed to 0 at frame 4.", case(scale=keyed((0, 50), (4, 0)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    t = comp["layers"][0]["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    comp["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.tiles", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_tiles.json").write_text(json.dumps(expected, indent=1) + "\n",
                                             encoding="utf-8")
    check(expected)


def near(a, b, tol=1e-12):
    return all(abs(u - v) < tol for p, q in zip(a, b) for u, v in zip(p, q))


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    layer = drawn_layer("card")
    d = lambda x, y: layer["px"][y * W + x]  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731
    one = lambda fx: c[fx]["0"]  # noqa: E731

    # The rule's pieces: the points, and the read at the plain tile is the place itself.
    assert [points(s / 100) for s in (100, 75, 50, 33.3, 25, 10, 1)] == [1, 2, 2, 4, 4, 10, 16]
    assert all(read(p + 0.5, 16, 50, 1.0) == p + 0.5 for p in range(16))
    assert read(0.25, 16, 50, 0.5) == read(8.25, 16, 50, 0.5)  # one tile on, the same place
    # No ceil and no floor sits on a cliff: every 1 / s is a whole number worked exactly or at
    # least 1e-5 from one.
    for s in (100, 75, 50, 33.3, 25, 1):
        v = 1 / (s / 100)
        assert v == round(v) or abs(v - round(v)) > 1e-5, s

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values())

    first = one("FX-TILES-001")
    assert first == c["FX-TILES-001"]["4"] and not near(first, drawn, 1e-3)
    # Tiles 8 wide starting at column 4: the frame repeats every 8 columns.
    assert all(first[at(x, y)] == first[at(x + 8, y)] for x in range(W - 8) for y in range(H))
    assert one("FX-TILES-002") == drawn == one("FX-TILES-010")
    three = one("FX-TILES-003")
    assert all(three[at(x, y)] == d((x + 4) % W, y) for x in range(W) for y in range(H))
    four = one("FX-TILES-004")
    assert all(near([four[at(x, y)]], [four[at(x + 4, y)]]) for x in range(W - 4)
               for y in range(H))
    assert not near(one("FX-TILES-005"), first, 1e-3)
    assert not near(one("FX-TILES-006"), four, 1e-3)
    seven = one("FX-TILES-007")
    assert max(abs(seven[at(x, y)][3] - seven[at(5, 5)][3]) for x in range(W)
               for y in range(H)) < 0.2  # a fine even mixture, nearly flat
    eight = one("FX-TILES-008")
    assert all(near([eight[at(x, y)]], [first[at((x + 4) % W, y)]]) for x in range(W)
               for y in range(H))
    nine = one("FX-TILES-009")
    assert near(nine, [[(a + b) / 2 for a, b in zip(p, q)] for p, q in zip(first, drawn)], 1e-12)
    eleven = c["FX-TILES-011"]
    assert eleven["0"] == drawn and eleven["2"] == one("FX-TILES-005") and eleven["4"] == first
    twelve = c["FX-TILES-012"]
    assert twelve["0"] == drawn and near(twelve["2"], nine) and twelve["4"] == first
    thirteen = c["FX-TILES-013"]
    assert thirteen["0"] == drawn and thirteen["4"] == three
    assert all(thirteen["2"][at(x, y)] == d((x + 2) % W, y) for x in range(W) for y in range(H))
    raw = value_at(CASES["FX-TILES-014"][1]["scale"], 2)
    fourteen = c["FX-TILES-014"]
    assert raw < 1 and fourteen["0"] == first and fourteen["2"] == fourteen["4"] == seven
    fifteen = one("FX-TILES-015")
    assert all(fifteen[at(x, y)] == first[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(fifteen[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    print("checked")


if __name__ == "__main__":
    main()
