"""Mosaic, worked a second way.

D-144 adds `core.mosaic`. It breaks a picture into square blocks `size` pixels wide, 10 as it
starts, and paints each block one flat colour, the average of what was in it: the pixelated
look of a censored face or an old game. The blocks are laid from the drawing's own top left
corner, so they move with the drawing; a block that runs past the layer's right or bottom edge
is cut short and averages only the pixels it has. It is this program's own method, modelled on
After Effects' Mosaic; nothing is ported. Document 21 is the rule in words; this file is the
reference for the numbers document 25 pins against it.

The rule. On the input as it reaches the effect, w by h pixels, the pixels an earlier effect
grew included, with o the drawing's own corner in that buffer (o = 0 unless an earlier effect
grew the layer): pixel (x, y) is in block (floor((x - o_x) / size), floor((y - o_y) / size)).
Every pixel of a block becomes the mean of all the buffer's pixels in that block, all four
premultiplied channels alike, empty pixels counted as empty, so a block the drawing only partly
covers comes out partly covered. `size` is 1 to 1000, keyable and held inside its range; size 1
makes one-pixel blocks and changes nothing, and so does a size below 1 after a draft's
scaling. The layer does not grow; a draft scales `size`.

Which block a pixel falls in is a choice made from a number, so this file's `check` asserts that
no pixel of any case sits within 1e-5 of a block's edge unless it sits exactly on it with a size
that single precision holds exactly, where the build's division is exact too.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: a strip of skin, shadow, line and white with a soft edge and a red dot among
empty pixels, drawn below. The drawing goes into `Fixtures/mosaic/media`, the projects into
`Fixtures/mosaic`, and the expected frames into `Fixtures/mosaic/expected_mosaic.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/mosaic_reference.py
"""

import json
import math
import struct
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import directional_blur_reference as D  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "mosaic"
TOLERANCE = 2e-5  # document 25's default for a filter
CLIFF = 1e-5      # the spec's margin: no pixel this close to a block's edge but exactly on it
SIZE = (1, 1000)
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def blocks(n, origin, size):
    """The block index of each of n buffer columns (or rows), the drawing's corner at `origin`."""
    if size < 1:
        return list(range(n))
    return [math.floor((i - origin) / size) for i in range(n)]


def mosaic(layer, size):
    """The layer's premultiplied pixels, each block its mean; its rectangle kept. The layer's
    `left` and `top` are the growth of earlier effects, so the drawing's corner is at buffer
    pixel (-left, -top)."""
    w, h, src = layer["w"], layer["h"], layer["px"]
    bx, by = blocks(w, -layer["left"], size), blocks(h, -layer["top"], size)
    sums = {}
    for y in range(h):
        for x in range(w):
            s = sums.setdefault((bx[x], by[y]), [0.0, 0.0, 0.0, 0.0, 0])
            for k in range(4):
                s[k] += src[y * w + x][k]
            s[4] += 1
    return [[v / s[4] for v in s[:4]] for s in (sums[(bx[x], by[y])]
                                               for y in range(h) for x in range(w))]


# --- the drawing ----------------------------------------------------------------------------

LINE, SKIN, TRACE, NONE = R.LINE, R.SKIN, R.TRACE, S.NONE  # #1e1a24, #f6d6be, #c82828
SHADE = (220, 160, 140, 255)     # the skin's shadow, #dca08c
WHITE = (255, 255, 255, 255)
SOFT = SKIN[:3] + (128,)         # the skin at half covering, a soft edge


def strip(x, y):
    """Columns 4 to 11: skin in rows 0 to 3, shadow in rows 4 and 5, the line along row 6; white
    in rows 7 to 9 from column 4 to the right edge; the skin at half covering down column 3,
    rows 0 to 6, a soft edge; one red dot at (13, 5); everything else empty."""
    if (x, y) == (13, 5):
        return TRACE
    if y >= 7:
        return WHITE if x >= 4 else NONE
    if 4 <= x <= 11:
        return SKIN if y <= 3 else SHADE if y <= 5 else LINE
    if x == 3:
        return SOFT
    return NONE


DRAWINGS = {"strip": [[strip(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(size=10, shift=0, before=None):
    """`before` is a directional blur (direction, length) ahead of the mosaic."""
    return {"drawing": "strip", "size": size, "shift": shift, "before": before}


def held(c, frame_no):
    return min(SIZE[1], max(SIZE[0], value_at(c["size"], frame_no)))


def layer_of(c):
    """The drawing as the mosaic receives it: grown by a directional blur ahead of it."""
    drawn = [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row]
    layer = {"px": drawn, "left": 0, "top": 0, "w": W, "h": H}
    if c["before"]:
        direction, length = c["before"]
        g = math.ceil(length / 2)
        layer = {"px": [D.blurred(drawn, direction, length, x, y)
                        for y in range(-g, H + g) for x in range(-g, W + g)],
                 "left": -g, "top": -g, "w": W + 2 * g, "h": H + 2 * g}
    return layer


def placed(layer, px, shift):
    """The layer's pixels `px` in the frame, moved `shift` pixels right."""
    out = []
    for y in range(H):
        for x in range(W):
            lx, ly = x - shift - layer["left"], y - layer["top"]
            inside = 0 <= lx < layer["w"] and 0 <= ly < layer["h"]
            out.append(px[ly * layer["w"] + lx] if inside else list(EMPTY))
    return out


def render(c, frame_no):
    layer = layer_of(c)
    return placed(layer, mosaic(layer, held(c, frame_no)), c["shift"])


def plain(c):
    """The drawing untouched, moved and grown as the case moves and grows it."""
    layer = layer_of(c)
    return placed(layer, layer["px"], c["shift"])


CASES = {
    "FX-MOSAIC-001": ("The settings as they start, size 10: two blocks across, columns 0 to 9 "
                      "and a block cut short at the right edge, columns 10 to 15, each the full "
                      "height and one flat colour, the mean of its pixels; both are partly "
                      "empty, so both come out partly covered, the left 63.5 of its 100 "
                      "pixels' worth, the right 33 of its 60, and the frame keeps each "
                      "channel's total.",
                      case(), [0]),
    "FX-MOSAIC-002": ("Size 1, the least: one-pixel blocks, and the drawing untouched.",
                      case(size=1), [0]),
    "FX-MOSAIC-003": ("Size 2: blocks of two by two, eight across and five down; the soft edge "
                      "down column 3 is averaged with the empty column 2 beside it, to a "
                      "quarter covering.",
                      case(size=2), [0]),
    "FX-MOSAIC-004": ("Size 4: blocks of four by four, the bottom row of blocks cut short to "
                      "rows 8 and 9; a block all of skin, columns 4 to 7 and rows 0 to 3, "
                      "stays skin; the empty top right block stays empty; the top left block, "
                      "empty but for four soft pixels, is skin at a quarter of their half "
                      "covering.",
                      case(size=4), [0]),
    "FX-MOSAIC-005": ("Size 3: 16 and 10 are each one more than a multiple of 3, so column 15 "
                      "and row 9 are blocks one pixel wide, and the corner pixel (15, 9), a "
                      "block of its own, is exactly the drawing's white.",
                      case(size=3), [0]),
    "FX-MOSAIC-006": ("Size 2.5, part of a pixel: the blocks are three and two pixels wide by "
                      "turns, columns 0 to 2, 3 and 4, 5 to 7, 8 and 9, 10 to 12, 13 and 14, "
                      "and 15 alone; rows 0 to 2, 3 and 4, 5 to 7, 8 and 9.",
                      case(size=2.5), [0]),
    "FX-MOSAIC-007": ("Size 16, the drawing's width: one block, the whole frame, every pixel "
                      "the drawing's mean colour at its mean covering.",
                      case(size=16), [0]),
    "FX-MOSAIC-008": ("Size 1000, the most: the same single block as size 16, FX-MOSAIC-007.",
                      case(size=1000), [0]),
    "FX-MOSAIC-009": ("Size 8: two blocks across, columns 0 to 7 and 8 to 15, and two down, "
                      "rows 0 to 7 and a short row of blocks, rows 8 and 9, which are the "
                      "white strip's and its empty start, so the bottom left block is white at "
                      "half covering and the bottom right white.",
                      case(size=8), [0]),
    "FX-MOSAIC-010": ("Size keyed from 2 at frame 0 to 6 at frame 4, linear: frame 0 is "
                      "FX-MOSAIC-003, frame 2 is size 4, FX-MOSAIC-004, and frame 4 size 6.",
                      case(size=keyed((0, 2), (4, 6))), [0, 2, 4]),
    "FX-MOSAIC-011": ("Size keyed from 1 at frame 0 to 1000 at frame 4, eased past its end: "
                      "at frame 2 it would pass 1000, is held at 1000, and is FX-MOSAIC-008, "
                      "as frame 4 is; frame 0 is the drawing untouched.",
                      case(size=keyed((0, 1, OVERSHOOT), (4, 1000))), [0, 2, 4]),
    "FX-MOSAIC-012": ("Size 4, the layer moved three pixels right: FX-MOSAIC-004 moved; the "
                      "blocks are laid from the drawing's own corner, so they move with it, "
                      "and the three columns left of it stay empty.",
                      case(size=4, shift=3), [0, 3]),
    "FX-MOSAIC-013": ("A directional blur, direction 90 and length 4, then size 4: the blur "
                      "grew the layer two pixels on every side, and the blocks are still laid "
                      "from the drawing's own corner, not the grown layer's, so the blocks in "
                      "the frame are FX-MOSAIC-004's, averaged from the blurred pixels, and "
                      "the bottom row of blocks, rows 8 to 11, takes in the two grown rows "
                      "below the frame, which the sideways blur left empty, so it comes out "
                      "half as covered as rows 8 and 9 are.",
                      case(size=4, before=(90, 4)), [0]),
    "FX-MOSAIC-014": ("Size 10.5: blocks columns 0 to 10 and 11 to 15, rows 0 to 9 one block: "
                      "the part of a pixel moves the block's edge past column 10.",
                      case(size=10.5), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-MOSAIC-015": ("Size 0, below 1.", case(size=0)),
    "FX-MOSAIC-016": ("Size 0.5, below 1 by half a pixel.", case(size=0.5)),
    "FX-MOSAIC-017": ("Size -10, below 1.", case(size=-10)),
    "FX-MOSAIC-018": ("Size 1001, above 1000.", case(size=1001)),
    "FX-MOSAIC-019": ("Size keyed to 1500 at frame 4.", case(size=keyed((0, 10), (4, 1500)))),
    "FX-MOSAIC-020": ("Size keyed to 0 at frame 4.", case(size=keyed((0, 10), (4, 0)))),
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
    effects = []
    if c["before"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.directional_blur",
                        "enabled": True,
                        "parameters": {"direction": c["before"][0], "length": c["before"][1]}})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.mosaic",
                    "enabled": True, "parameters": {"size": setting_json(c["size"])}})
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

    (OUT / "expected_mosaic.json").write_text(json.dumps(expected, indent=1) + "\n",
                                              encoding="utf-8")
    check(expected)


def single(v):
    """v as single precision holds it."""
    return struct.unpack("f", struct.pack("f", v))[0]


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    one = lambda fx: c[fx]["0"]  # noqa: E731
    close = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    total = lambda f: [sum(p[k] for p in f) for k in range(4)]  # noqa: E731

    def mean(xs, ys, src=drawn):
        ps = [src[at(x, y)] for x in xs for y in ys]
        return [sum(p[k] for p in ps) / len(ps) for k in range(4)]

    def flat(px, xs, ys, want):
        return all(close(px[at(x, y)], want) for x in xs for y in ys)

    # The rule's own pieces: blocks start at the drawing's corner, cut short at the far edge,
    # and a size below 1 is one-pixel blocks.
    assert blocks(16, 0, 10) == [0] * 10 + [1] * 6
    assert blocks(16, 0, 2.5) == [0, 0, 0, 1, 1, 2, 2, 2, 3, 3, 4, 4, 4, 5, 5, 6]
    assert blocks(8, 2, 4) == [-1, -1, 0, 0, 0, 0, 1, 1]
    assert blocks(5, 0, 0.5) == blocks(5, 0, 1) == [0, 1, 2, 3, 4]
    layer = layer_of(case())
    assert mosaic(layer, 1) == drawn == mosaic(layer, 0.4)

    # The block edges: no pixel any case places within 1e-5 of a block's edge, unless exactly on
    # it with a size single precision holds exactly.
    margin = 1.0
    for fx, (says, cs, frames) in CASES.items():
        for f in frames:
            size, ly = held(cs, f), layer_of(cs)
            assert single(size) == size, (fx, f, size)
            for n, o in ((ly["w"], -ly["left"]), (ly["h"], -ly["top"])):
                for i in range(n):
                    q = (i - o) / size
                    d = abs(q - round(q))
                    assert d == 0 or d > CLIFF, (fx, f, i, q)
                    if d:
                        margin = min(margin, d)
    print(f"closest to a block's edge, off it: {margin:.3g}")

    # Every case: premultiplied values in range; with nothing moved or grown, nothing is lost or
    # gained, each channel's total over the frame is the drawing's.
    for fx, frames in c.items():
        for px in frames.values():
            for i, p in enumerate(px):
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (fx, i)
        cs = (CASES.get(fx) or INVALID.get(fx))[1]
        if not cs["shift"] and not cs["before"]:
            for px in frames.values():
                assert close(total(px), total(drawn), 1e-9), fx
        if fx in INVALID:
            assert all(px == drawn for px in frames.values())

    L, Rt = range(10), range(10, 16)
    one1 = one("FX-MOSAIC-001")
    assert flat(one1, L, range(H), mean(L, range(H)))
    assert flat(one1, Rt, range(H), mean(Rt, range(H)))
    soft_a = 128 / 255
    assert abs(one1[0][3] * 100 - (7 * soft_a + 6 * 7 + 6 * 3)) < 1e-12  # 63.5 pixels' worth
    assert abs(one1[0][3] * 100 - 63.5) < 0.02 and abs(one1[15][3] * 60 - 33) < 1e-12
    assert one("FX-MOSAIC-002") == drawn
    three = one("FX-MOSAIC-003")
    for bx in range(0, W, 2):
        for by in range(0, H, 2):
            assert flat(three, (bx, bx + 1), (by, by + 1), mean((bx, bx + 1), (by, by + 1)))
    assert abs(three[at(2, 0)][3] - soft_a / 2) < 1e-15
    four = one("FX-MOSAIC-004")
    for bx in range(0, W, 4):
        for by in range(0, H, 4):
            xs, ys = range(bx, bx + 4), range(by, min(H, by + 4))
            assert flat(four, xs, ys, mean(xs, ys))
    skin = R.working(SKIN)
    assert flat(four, range(4, 8), range(4), skin)
    assert all(four[at(x, y)] == EMPTY for x in range(12, 16) for y in range(4))
    assert close(four[0], [v / 4 for v in R.working(SOFT)])
    assert abs(four[0][3] - soft_a / 4) < 1e-15
    five = one("FX-MOSAIC-005")
    assert five[at(15, 9)] == drawn[at(15, 9)] == R.working(WHITE)
    assert flat(five, (15,), range(3), mean((15,), range(3)))
    assert flat(five, range(12, 15), (9,), mean(range(12, 15), (9,)))
    six = one("FX-MOSAIC-006")
    groups = [(0, 1, 2), (3, 4), (5, 6, 7), (8, 9), (10, 11, 12), (13, 14), (15,)]
    for xs in groups:
        for ys in [(0, 1, 2), (3, 4), (5, 6, 7), (8, 9)]:
            assert flat(six, xs, ys, mean(xs, ys))
    everything = mean(range(W), range(H))
    assert flat(one("FX-MOSAIC-007"), range(W), range(H), everything)
    assert one("FX-MOSAIC-008") == one("FX-MOSAIC-007")
    nine = one("FX-MOSAIC-009")
    white = R.working(WHITE)
    assert flat(nine, range(8), (8, 9), [v / 2 for v in white])
    assert flat(nine, range(8, 16), (8, 9), white)
    assert flat(nine, range(8), range(8), mean(range(8), range(8)))
    ten = c["FX-MOSAIC-010"]
    assert ten["0"] == three and ten["2"] == four and ten["4"] == render(case(size=6), 0)
    raw = value_at(CASES["FX-MOSAIC-011"][1]["size"], 2)
    assert ease(OVERSHOOT, 0.5) > 1 and raw > 1000
    eleven = c["FX-MOSAIC-011"]
    assert eleven["0"] == drawn and eleven["2"] == eleven["4"] == one("FX-MOSAIC-008")
    # Moved: the blocks move with the drawing.
    twelve = c["FX-MOSAIC-012"]
    assert twelve["0"] == twelve["3"]
    assert all(twelve["0"][at(x, y)] == four[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(twelve["0"][at(x, y)] == EMPTY for x in range(3) for y in range(H))
    # After a directional blur: blocks from the drawing's corner, the grown border its own
    # blocks two pixels wide.
    grown = case(size=4, before=(90, 4))
    gl = layer_of(grown)
    thirteen, blurred = one("FX-MOSAIC-013"), plain(grown)
    assert (gl["left"], gl["top"], gl["w"], gl["h"]) == (-2, -2, 20, 14)
    gat = lambda x, y: (y + 2) * 20 + x + 2  # noqa: E731
    assert all(gl["px"][gat(x, y)] == EMPTY for x in range(-2, 18) for y in (-2, -1, 10, 11))
    for bx in range(0, W, 4):
        for by in range(0, H, 4):
            xs, ys = range(bx, bx + 4), range(by, by + 4)
            ps = [gl["px"][gat(x, y)] for x in xs for y in ys]
            want = [sum(p[k] for p in ps) / 16 for k in range(4)]
            assert flat(thirteen, xs, range(by, min(H, by + 4)), want)
            if by == 8:  # the two empty grown rows halve the bottom row of blocks
                assert close(want, [v / 2 for v in mean(xs, (8, 9), blurred)])
    whole = mosaic(gl, 4)
    left = [gl["px"][gat(x, y)] for x in (-2, -1) for y in range(4)]
    assert close(whole[gat(-2, 0)], [sum(p[k] for p in left) / 8 for k in range(4)])
    assert whole[gat(-2, 0)] == whole[gat(-1, 3)] and whole[gat(-1, 0)] != whole[gat(0, 0)]
    assert thirteen != placed(gl, mosaic(dict(gl, left=0, top=0), 4), 0)  # not the grown corner
    fourteen = one("FX-MOSAIC-014")
    assert blocks(16, 0, 10.5) == [0] * 11 + [1] * 5
    assert flat(fourteen, range(11), range(H), mean(range(11), range(H)))
    assert flat(fourteen, range(11, 16), range(H), mean(range(11, 16), range(H)))
    assert fourteen != one1
    print("checked")


if __name__ == "__main__":
    main()
