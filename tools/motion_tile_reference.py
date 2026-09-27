"""Motion tile, worked a second way.

D-154 adds `core.motion_tile`. It repeats a layer's picture around itself, tile against tile, so
a small patch of pattern fills a larger area: a repeating background, a wallpaper, a floor, or a
picture that must still cover the frame after it is moved or shaken. `output_width` and
`output_height`, each 100 to 1000 per cent, starting at 100, say how wide and tall the tiled
layer is against the picture; `mirror`, the word "off" or "on", starting at "off", turns every
other tile over, left to right across and top to bottom down, so the tiles meet edge to
matching edge with no seam. It is this program's own method, modelled on After Effects' Motion
Tile in spirit and not claimed to match it; nothing is ported. Document 21 is the rule in
words; this file is the reference for the numbers document 25 pins against it.

The rule. On the input as it reaches the effect, w by h pixels, the pixels an earlier effect
grew included, gx = ceil(w (output_width / 100 - 1) / 2) and gy likewise from h and
output_height; the output is w + 2 gx by h + 2 gy pixels, its origin moved by (gx, gy), so the
picture stays where it was, in the middle. Output pixel (x, y): i = x - gx, k = floor(i / w),
j = i - k w, and with mirror "on" and k odd, j = w - 1 - j; the row likewise; the output is the
input pixel at column j and that row, exactly, all four channels, with no mixing. Both 100: the
output is the input. The percentages are not distances: a draft does not scale them.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing 6 pixels by 4, drawn below, its
top left corner at (5, 3) so the composition's centre is the drawing's; moved unless the case
says, never. The tiles land around it, so a layer that grows is shown with its grown pixels in
place. The drawing goes into `Fixtures/motion_tile/media`, the projects into
`Fixtures/motion_tile`, and the expected frames into
`Fixtures/motion_tile/expected_motion_tile.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/motion_tile_reference.py
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
from drop_shadow_reference import drop_shadow, frame  # noqa: E402

W, H = R.W, R.H          # the composition, 16 by 10
DW, DH = 6, 4            # the drawing
LEFT, TOP = (W - DW) // 2, (H - DH) // 2  # where the drawing's top left corner lands, (5, 3)
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "motion_tile"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGE = (100, 1000)
NAMES = ("output_width", "output_height", "mirror")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def growth(size, percent):
    """The pixels added on each side: ceil(size (percent / 100 - 1) / 2)."""
    return math.ceil(size * (percent / 100 - 1) / 2)


def source(i, size, mirror):
    """The input column (or row) that output column i, counted from the input's own first
    column, repeats."""
    k = math.floor(i / size)
    j = i - k * size
    return size - 1 - j if mirror == "on" and k % 2 else j


def motion_tile(layer, output_width, output_height, mirror):
    """The layer tiled around itself, grown by (gx, gy) on each side, its origin moved by them."""
    w, h, src = layer["w"], layer["h"], layer["px"]
    gx, gy = growth(w, output_width), growth(h, output_height)
    out = [src[source(y - gy, h, mirror) * w + source(x - gx, w, mirror)]
           for y in range(h + 2 * gy) for x in range(w + 2 * gx)]
    return {"px": out, "left": layer["left"] - gx, "top": layer["top"] - gy,
            "w": w + 2 * gx, "h": h + 2 * gy}


# --- the drawing ----------------------------------------------------------------------------

LINE, SKIN, TRACE, NONE = R.LINE, R.SKIN, R.TRACE, S.NONE  # #1e1a24, #f6d6be, #c82828
SOFT_SKIN = SKIN[:3] + (128,)        # the skin at half covering, a soft edge
BAND = (58, 111, 216, 255)           # #3a6fd8, the ball's blue band


def patch(x, y):
    """Six by four, turned neither way alike, so a tile turned over shows: skin in columns 0 to
    3 of rows 0 to 2, a red dot at (1, 1), the skin at half covering at (0, 2), a line down
    column 4 in rows 0 to 2, a blue band along row 3 from column 2 and up column 5 into row 2;
    (5, 0), (5, 1), (0, 3) and (1, 3) are empty."""
    if (x, y) == (1, 1):
        return TRACE
    if (x, y) == (0, 2):
        return SOFT_SKIN
    if (x, y) in ((5, 0), (5, 1), (0, 3), (1, 3)):
        return NONE
    if y == 3 or x == 5:
        return BAND
    if x == 4:
        return LINE
    return SKIN


DRAWINGS = {"patch": [[patch(x, y) for x in range(DW)] for y in range(DH)]}


def drawn_layer(name):
    return {"px": [R.working(p) for row in DRAWINGS[name] for p in row],
            "left": LEFT, "top": TOP, "w": DW, "h": DH}


# --- the cases ------------------------------------------------------------------------------

# A drop shadow ahead of the tile, black, opacity 100, direction 90, distance 2, softness 0: the
# shadow two pixels right, and the layer grown two pixels on every side, to 10 by 8.
SHADOW = {"color": "#000000", "opacity": 100, "direction": 90, "distance": 2, "softness": 0}


def case(output_width=100, output_height=100, mirror="off", shift=0, shadow=False):
    return {"drawing": "patch", "output_width": output_width, "output_height": output_height,
            "mirror": mirror, "shift": shift, "shadow": shadow}


def held(v):
    return min(RANGE[1], max(RANGE[0], v))


def layer_of(c):
    layer = drawn_layer(c["drawing"])
    if c["shadow"]:
        layer = drop_shadow(layer, SHADOW["color"], SHADOW["opacity"], SHADOW["direction"],
                            SHADOW["distance"], SHADOW["softness"])
    return layer


def percents(c, frame_no):
    return held(value_at(c["output_width"], frame_no)), held(value_at(c["output_height"], frame_no))


def render(c, frame_no):
    return frame(motion_tile(layer_of(c), *percents(c, frame_no), c["mirror"]), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return frame(drawn_layer(c["drawing"]), c["shift"])


CASES = {
    "FX-TILE-001": ("The settings as they start, output width 100, output height 100, mirror "
                    "off: the drawing, untouched, and nothing grows.",
                    case(), [0, 4]),
    "FX-TILE-002": ("Mirror on at 100, 100: there is only the one tile, the drawing itself, "
                    "so it is untouched.",
                    case(mirror="on"), [0]),
    "FX-TILE-003": ("Output width 200: the layer grows by 3 on the left and the right, 6 (1 - "
                    "1) / 2 rounded up, and is 12 wide: columns 2 to 4 repeat the drawing's "
                    "columns 3 to 5, the right part of the tile to its left, and columns 11 to "
                    "13 its columns 0 to 2, so the red dot is repeated at (12, 4); rows 3 to 6 "
                    "only, nothing above or below.",
                    case(output_width=200), [0]),
    "FX-TILE-004": ("Output height 200: the layer grows by 2 above and below and is 8 tall: "
                    "rows 1 and 2 repeat the drawing's rows 2 and 3, and rows 7 and 8 its rows "
                    "0 and 1, so the red dot is repeated at (6, 8); columns 5 to 10 only.",
                    case(output_height=200), [0]),
    "FX-TILE-005": ("Output width and height 200: a layer 12 by 8, columns 2 to 13 and rows 1 "
                    "to 8, every pixel of it a pixel of the drawing, the tiles repeated across, "
                    "down and at the corners; the rest of the frame stays empty.",
                    case(output_width=200, output_height=200), [0]),
    "FX-TILE-006": ("FX-TILE-005 with mirror on: the tiles left and right of the drawing are "
                    "turned over left to right and those above and below top to bottom, so "
                    "each edge meets its own reflection: column 4 is the drawing's column 0 "
                    "and column 11 its column 5, row 2 is its row 0 and row 7 its row 3, and "
                    "the red dot is repeated at (3, 4), (6, 1) and (3, 1).",
                    case(output_width=200, output_height=200, mirror="on"), [0]),
    "FX-TILE-007": ("Output width and height 300: the layer grows by 6 and by 4 and is 18 by "
                    "12, more than the frame, so the whole frame is tiled: every pixel is the "
                    "drawing's pixel at column (x - 5) mod 6 and row (y - 3) mod 4.",
                    case(output_width=300, output_height=300), [0]),
    "FX-TILE-008": ("FX-TILE-007 with mirror on: the whole frame tiled with every other tile "
                    "turned over, a pattern that repeats every 12 columns and 8 rows with no "
                    "seam.",
                    case(output_width=300, output_height=300, mirror="on"), [0]),
    "FX-TILE-009": ("Output width and height 1000, the most: the layer grows by 27 and by 18 "
                    "and is 60 by 40, far larger than the frame, and the tiles keep their "
                    "places against the drawing, so the frame is FX-TILE-007's exactly.",
                    case(output_width=1000, output_height=1000), [0]),
    "FX-TILE-010": ("Output width and height 150: the growth is rounded up to whole pixels, "
                    "1.5 to 2 on the left and the right and exactly 1 above and below, so the "
                    "layer is 10 by 6, columns 3 to 12 and rows 2 to 7, not exactly 150 per "
                    "cent.",
                    case(output_width=150, output_height=150), [0]),
    "FX-TILE-011": ("Output width 110 and height 125: the least growth, 0.3 and 0.5 rounded "
                    "up to 1, one pixel on every side, each the pixel from the opposite edge "
                    "of the drawing: column 4 is its column 5 and row 2 its row 3.",
                    case(output_width=110, output_height=125), [0]),
    "FX-TILE-012": ("FX-TILE-011 with mirror on: each added pixel is the drawing's own edge "
                    "pixel beside it, as a held edge is: column 4 is its column 0, column 11 "
                    "its column 5, row 2 its row 0 and row 7 its row 3.",
                    case(output_width=110, output_height=125, mirror="on"), [0]),
    "FX-TILE-013": ("Output width keyed from 100 at frame 0 to 300 at frame 4, linear: frame 0 "
                    "is the drawing, frame 2 is width 200, FX-TILE-003, and frame 4 is width "
                    "300, a band of tiles across the whole frame in rows 3 to 6.",
                    case(output_width=keyed((0, 100), (4, 300))), [0, 2, 4]),
    "FX-TILE-014": ("Output height keyed from 100 at frame 0 to 1000 at frame 4, eased past "
                    "its end (1292.5 at frame 2): frame 2 is held at 1000, so frames 2 and 4 are "
                    "the same, a column of tiles down the whole frame in columns 5 to 10.",
                    case(output_height=keyed((0, 100, OVERSHOOT), (4, 1000))), [0, 2, 4]),
    "FX-TILE-015": ("FX-TILE-005 with the layer moved three pixels right: the tiles move with "
                    "it, columns 5 to 15 are FX-TILE-005's columns 2 to 12, its column 13 is "
                    "past the frame's edge, and columns 0 to 4 stay empty.",
                    case(output_width=200, output_height=200, shift=3), [0, 3]),
    "FX-TILE-016": ("A drop shadow two pixels right, then output width and height 150: the "
                    "shadow grew the layer by 2 on every side to 10 by 8, and the tiles are "
                    "that grown layer's, shadow and all: it grows by 3 across (2.5 rounded up) "
                    "and 2 down to 16 by 12, filling the frame.",
                    case(output_width=150, output_height=150, shadow=True), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-TILE-017": ("Output width 99, below 100.", case(output_width=99)),
    "FX-TILE-018": ("Output width 1001, above 1000.", case(output_width=1001)),
    "FX-TILE-019": ("Output height 99.5, below 100.", case(output_height=99.5)),
    "FX-TILE-020": ("Output height 1000.5, above 1000.", case(output_height=1000.5)),
    "FX-TILE-021": ("Output width keyed to 1200 at frame 4.",
                    case(output_width=keyed((0, 200), (4, 1200)))),
    "FX-TILE-022": ("Mirror \"yes\", which is not a choice.", case(mirror="yes")),
    "FX-TILE-023": ("Mirror \"On\": the word is exact, so a capital is not the choice.",
                    case(mirror="On")),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    t = comp["layers"][0]["transform"]
    t["anchor"] = prop([DW / 2, DH / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    effects = []
    if c["shadow"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.drop_shadow", "enabled": True,
                        "parameters": dict(SHADOW)})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.motion_tile",
                    "enabled": True, "parameters": {k: setting_json(c[k]) for k in NAMES}})
    comp["layers"][0]["effects"] = effects
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

    (OUT / "expected_motion_tile.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    layer = drawn_layer("patch")
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    one = lambda fx: c[fx]["0"]  # noqa: E731
    d = lambda x, y: layer["px"][y * DW + x]  # noqa: E731  the drawing's own pixel

    def tiled(gx, gy, mirror, shift=0):
        """The frame worked a second way: a mirrored run repeats every 2w, the tile then its
        reflection, so column i reads m = i mod 2w, or 2w - 1 - m past w."""
        def col(i, n):
            if mirror == "off":
                return i % n
            m = i % (2 * n)
            return m if m < n else 2 * n - 1 - m
        out = []
        for y in range(H):
            for x in range(W):
                i, r = x - shift - LEFT, y - TOP
                inside = -gx <= i < DW + gx and -gy <= r < DH + gy
                out.append(d(col(i, DW), col(r, DH)) if inside else EMPTY)
        return out

    # The rule's own pieces: the growth, and the floor that runs below 0.
    assert [growth(6, p) for p in (100, 110, 125, 150, 200, 300, 1000)] == [0, 1, 1, 2, 3, 6, 27]
    assert [growth(4, p) for p in (100, 125, 150, 200, 300, 1000)] == [0, 1, 1, 2, 4, 18]
    assert [source(i, 6, "off") for i in (-7, -6, -1, 0, 5, 6, 13)] == [5, 0, 5, 0, 5, 0, 1]
    assert [source(i, 6, "on") for i in (-7, -6, -1, 0, 5, 6, 13)] == [5, 5, 0, 0, 5, 5, 1]
    assert motion_tile(layer, 100, 100, "on") == layer
    # The ceil is a cliff: every growth it decides is a whole number worked exactly (the
    # percentage a whole multiple of 25, exact in binary at any precision) or at least 1e-5
    # from one.
    for fx, (_, cs, frames) in CASES.items():
        size = layer_of(cs)
        for f in frames:
            for n, p in zip((size["w"], size["h"]), percents(cs, f)):
                arg = n * (p / 100 - 1) / 2
                assert (arg == round(arg) and p % 25 == 0) or \
                    abs(arg - round(arg)) > 1e-5, (fx, f, arg)

    for fx, frames in c.items():
        for px in frames.values():
            for i, p in enumerate(px):
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (fx, i)
    # No mixing: every pixel of every case without a shadow is one of the drawing's, or empty.
    pixels = layer["px"] + [EMPTY]
    for fx in ("FX-TILE-%03d" % n for n in range(1, 16)):
        for px in c[fx].values():
            assert all(p in pixels for p in px), fx

    assert one("FX-TILE-001") == drawn == c["FX-TILE-001"]["4"] == one("FX-TILE-002")
    for fx, (gx, gy, mirror) in (("003", (3, 0, "off")), ("004", (0, 2, "off")),
                                 ("005", (3, 2, "off")), ("006", (3, 2, "on")),
                                 ("007", (6, 4, "off")), ("008", (6, 4, "on")),
                                 ("009", (27, 18, "off")), ("010", (2, 1, "off")),
                                 ("011", (1, 1, "off")), ("012", (1, 1, "on"))):
        assert one("FX-TILE-" + fx) == tiled(gx, gy, mirror), fx
    red = R.working(TRACE)
    three, four, five, six = (one(f"FX-TILE-00{n}") for n in (3, 4, 5, 6))
    assert all(three[at(2 + k, y)] == d(3 + k, y - 3) for k in range(3) for y in range(3, 7))
    assert all(three[at(11 + k, y)] == d(k, y - 3) for k in range(3) for y in range(3, 7))
    assert three[at(12, 4)] == red and all(three[at(x, y)] == EMPTY for x in range(W)
                                           for y in (0, 1, 2, 7, 8, 9))
    assert all(four[at(x, 1)] == d(x - 5, 2) and four[at(x, 8)] == d(x - 5, 1)
               for x in range(5, 11))
    assert four[at(6, 8)] == red and all(four[at(x, y)] == EMPTY for x in (0, 4, 11, 15)
                                         for y in range(H))
    assert sum(p != EMPTY for p in five) == 12 * 8 - 4 * 4  # four empties in every tile
    assert all(five[at(x, y)] == EMPTY for x in range(W) for y in range(H)
               if not (2 <= x <= 13 and 1 <= y <= 8))
    for y in range(3, 7):
        assert six[at(4, y)] == d(0, y - 3) and six[at(11, y)] == d(5, y - 3)
    for x in range(5, 11):
        assert six[at(x, 2)] == d(x - 5, 0) and six[at(x, 7)] == d(x - 5, 3)
    assert six[at(3, 4)] == six[at(6, 1)] == six[at(3, 1)] == red != five[at(3, 4)]
    seven, eight = one("FX-TILE-007"), one("FX-TILE-008")
    assert all(seven[at(x, y)] == d((x - 5) % 6, (y - 3) % 4) for x in range(W) for y in range(H))
    assert eight != seven and all(eight[at(x, y)] == eight[at(x + 12, y)]
                                  for x in range(W - 12) for y in range(H))
    assert all(eight[at(x, y)] == eight[at(x, y + 8)] for x in range(W) for y in range(H - 8))
    assert one("FX-TILE-009") == seven
    ten = one("FX-TILE-010")
    assert all((ten[at(x, y)] != EMPTY) == (d((x - 5) % 6, (y - 3) % 4) != EMPTY)
               for x in range(3, 13) for y in range(2, 8))
    assert all(ten[at(x, y)] == EMPTY for x in range(W) for y in range(H)
               if not (3 <= x <= 12 and 2 <= y <= 7))
    eleven, twelve = one("FX-TILE-011"), one("FX-TILE-012")
    for y in range(3, 7):
        assert eleven[at(4, y)] == d(5, y - 3) and twelve[at(4, y)] == d(0, y - 3)
        assert eleven[at(11, y)] == d(0, y - 3) and twelve[at(11, y)] == d(5, y - 3)
    for x in range(5, 11):
        assert eleven[at(x, 2)] == d(x - 5, 3) and twelve[at(x, 2)] == d(x - 5, 0)
        assert eleven[at(x, 7)] == d(x - 5, 0) and twelve[at(x, 7)] == d(x - 5, 3)
    assert twelve[at(4, 5)] == R.working(SOFT_SKIN)  # the soft edge held outward
    thirteen, fourteen = c["FX-TILE-013"], c["FX-TILE-014"]
    assert thirteen["0"] == drawn and thirteen["2"] == three
    assert thirteen["4"] == render(case(output_width=300), 0) == tiled(6, 0, "off")
    assert all(thirteen["4"][at(x, y)] != EMPTY or d((x - 5) % 6, y - 3) == EMPTY
               for x in range(W) for y in range(3, 7))
    raw = value_at(CASES["FX-TILE-014"][1]["output_height"], 2)
    assert raw > 1000 and fourteen["0"] == drawn
    assert fourteen["2"] == fourteen["4"] == render(case(output_height=1000), 0) \
        == tiled(0, 18, "off")
    fifteen = c["FX-TILE-015"]
    assert fifteen["0"] == fifteen["3"] == tiled(3, 2, "off", shift=3)
    assert all(fifteen["0"][at(x, y)] == five[at(x - 3, y)] for x in range(3, W)
               for y in range(H))
    assert all(fifteen["0"][at(x, y)] == EMPTY for x in range(5) for y in range(H))
    # After a drop shadow: the tiles are the grown 10 by 8 layer's, shadow and all.
    shadowed = layer_of(case(shadow=True))
    assert (shadowed["left"], shadowed["top"], shadowed["w"], shadowed["h"]) == (3, 1, 10, 8)
    sixteen = one("FX-TILE-016")
    s = lambda x, y: shadowed["px"][y * 10 + x]  # noqa: E731
    assert all(sixteen[at(x, y)] == s((x - 3) % 10, (y - 1) % 8)
               for x in range(W) for y in range(H))
    assert sixteen[at(12, 5)][3] == 1 and sixteen[at(12, 5)][:3] == [0.0] * 3  # the shadow
    assert sixteen[at(2, 5)] == sixteen[at(12, 5)]  # and its tile to the left
    print("checked")


if __name__ == "__main__":
    main()
