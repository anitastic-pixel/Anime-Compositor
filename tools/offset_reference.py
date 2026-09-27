"""Offset, worked a second way.

D-131 adds `core.offset`. It slides a layer's picture across itself with wrap-around: what
leaves one edge comes back in at the opposite edge, so a picture that tiles can be slid round
for ever, as a looping background slide is. `shift` is two numbers, x then y, in pixels, each
-100000 to 100000, starting at 0, 0; positive x slides right and positive y down. It is a
distance, so a draft preview scales it. It is this program's own method, modelled on After
Effects' Offset (its Shift Center To); nothing is ported. Document 21 is the rule in words; this
file is the reference for the numbers document 25 pins against it.

The rule. On the input as it reaches the effect, w by h pixels, the pixels an earlier effect
grew included, the output at the pixel whose centre is P is document 21's bilinear sample at
Q = P - shift, wrapped: with X = Q.x - 0.5, i = floor(X) and f = X - i, it reads the columns
i mod w and (i + 1) mod w, the mod always 0 to w - 1, with the weights 1 - f and f, and the rows
likewise. All four channels are premultiplied and mixed alike. A whole-pixel shift moves every
pixel exactly; a shift of a whole width or height, or any number of them, changes nothing; and
the layer does not grow, so nothing is drawn outside it and nothing wraps in from outside it.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: a tile, drawn below, touching the drawing's edges so the wrap shows. The drawing
goes into `Fixtures/offset/media`, the projects into `Fixtures/offset`, and the expected frames
into `Fixtures/offset/expected_offset.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/offset_reference.py
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

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "offset"
TOLERANCE = 2e-5  # document 25's default for a filter
SHIFT = (-100000, 100000)
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def offset(layer, shift):
    """The layer's pixels slid by `shift` with wrap-around; the same size and place."""
    w, h, src = layer["w"], layer["h"], layer["px"]
    out = []
    for y in range(h):
        for x in range(w):
            X = (x + 0.5 - shift[0]) - 0.5
            Y = (y + 0.5 - shift[1]) - 0.5
            i, j = math.floor(X), math.floor(Y)
            fx, fy = X - i, Y - j
            p = [0.0] * 4
            for row, wy in ((j, 1 - fy), (j + 1, fy)):
                for col, wx in ((i, 1 - fx), (i + 1, fx)):
                    if wx * wy:
                        s = src[(row % h) * w + col % w]
                        for k in range(4):
                            p[k] += s[k] * wx * wy
            out.append(p)
    return dict(layer, px=out)


# --- the drawing ----------------------------------------------------------------------------

WHITE = (255, 255, 255, 255)
LINE, SKIN, TRACE, NONE = R.LINE, R.SKIN, R.TRACE, S.NONE  # #1e1a24, #f6d6be, #c82828
SOFT = SKIN[:3] + (128,)              # the skin at half covering, a soft edge


def tile(x, y):
    """A white block in columns 0 to 4 and rows 1 to 4, against the drawing's left edge, closed
    by a line down column 5; a skin block in columns 9 to 13 and rows 5 to 9, against its bottom
    edge, with a half-covering edge down column 14; and one red dot in the top right corner,
    (15, 0). Everything else is empty."""
    if (x, y) == (15, 0):
        return TRACE
    if 1 <= y <= 4 and x <= 4:
        return WHITE
    if 1 <= y <= 4 and x == 5:
        return LINE
    if 5 <= y and 9 <= x <= 13:
        return SKIN
    if 5 <= y and x == 14:
        return SOFT
    return NONE


DRAWINGS = {"tile": [[tile(x, y) for x in range(W)] for y in range(H)]}


def drawn_layer(name):
    return {"px": [R.working(p) for row in DRAWINGS[name] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


# --- the cases ------------------------------------------------------------------------------

# A drop shadow ahead of the offset, black, opacity 100, direction 90, distance 2, softness 0:
# the shadow two pixels right, and the layer grown two pixels on every side, to 20 by 14.
SHADOW = {"color": "#000000", "opacity": 100, "direction": 90, "distance": 2, "softness": 0}


def case(shift=(0, 0), move=0, shadow=False):
    return {"drawing": "tile", "shift": shift, "move": move, "shadow": shadow}


def held(v, r):
    return min(r[1], max(r[0], v))


def layer_of(c):
    layer = drawn_layer(c["drawing"])
    if c["shadow"]:
        layer = drop_shadow(layer, SHADOW["color"], SHADOW["opacity"], SHADOW["direction"],
                            SHADOW["distance"], SHADOW["softness"])
    return layer


def render(c, frame_no):
    shift = [held(v, SHIFT) for v in value_at(c["shift"], frame_no)]
    return frame(offset(layer_of(c), shift), c["move"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return frame(drawn_layer(c["drawing"]), c["move"])


CASES = {
    "FX-OFFSET-001": ("The settings as they start, shift 0, 0: the drawing, untouched.",
                      case(), [0]),
    "FX-OFFSET-002": ("Shift 3, 0: every pixel moves exactly three pixels right, and the three "
                      "columns that leave the right edge come back in at the left, so the red "
                      "dot is at (2, 0) and the soft edge down column 1.",
                      case(shift=(3, 0)), [0]),
    "FX-OFFSET-003": ("Shift 0, 2: every pixel moves exactly two pixels down, and the skin "
                      "block's two lowest rows come back in at the top, in rows 0 and 1; the "
                      "red dot is at (15, 2).",
                      case(shift=(0, 2)), [0]),
    "FX-OFFSET-004": ("Shift -3, -2, left and up: every pixel moves exactly; the white block's "
                      "left three columns come back in at the right, in columns 13 to 15, and "
                      "its top row at the bottom, in row 9; the red dot is at (12, 8).",
                      case(shift=(-3, -2)), [0]),
    "FX-OFFSET-005": ("Shift 8, 5, half the drawing each way: the four quarters change places "
                      "corner to corner, so the red dot, from the top right corner, is at "
                      "(7, 5) in the middle; the white block, from the left edge, is in "
                      "columns 8 to 12 and rows 6 to 9; and the skin block, from the bottom "
                      "right, is in columns 1 to 5 and rows 0 to 4.",
                      case(shift=(8, 5)), [0]),
    "FX-OFFSET-006": ("Shift 16, 10, one whole width and height: every pixel comes back to its "
                      "own place, and the drawing is untouched.",
                      case(shift=(16, 10)), [0]),
    "FX-OFFSET-007": ("Shift 35, -23, more than twice round each way: exactly the same as "
                      "shift 3, -3, as 35 is two widths and 3, and -23 is three heights "
                      "up and 7 down again.",
                      case(shift=(35, -23)), [0]),
    "FX-OFFSET-008": ("Shift 100000, -100000, the most each way: 100000 is a whole number of "
                      "widths (6250) and of heights (10000), so the drawing is untouched.",
                      case(shift=(100000, -100000)), [0]),
    "FX-OFFSET-009": ("Shift -99999, 99999: thousands of times round, and exactly the same as "
                      "shift 1, -1.",
                      case(shift=(-99999, 99999)), [0]),
    "FX-OFFSET-010": ("Shift 0.5, 0, half a pixel right: every pixel is the even mix of itself "
                      "and the one to its left, column 0 mixing with column 15 round the "
                      "wrap, so the red dot shows half-covering at (0, 0) and (15, 0), and the "
                      "soft edge spreads to column 15 at about a quarter covering.",
                      case(shift=(0.5, 0)), [0]),
    "FX-OFFSET-011": ("Shift -2.25, 1.75, a part of a pixel both ways: every pixel is a mix of "
                      "four, with weights 9/16, 3/16, 3/16 and 1/16, wrapped; nothing is lost "
                      "and nothing gained, each channel's total over the frame is the "
                      "drawing's.",
                      case(shift=(-2.25, 1.75)), [0]),
    "FX-OFFSET-012": ("Shift -31.5, 20.25, part-pixel shifts nearly twice round: exactly the "
                      "same as shift 0.5, 0.25.",
                      case(shift=(-31.5, 20.25)), [0]),
    "FX-OFFSET-013": ("Shift keyed from 0, 0 at frame 0 to 8, 4 at frame 4, linear: frame 0 "
                      "untouched, frame 2 is shift 4, 2, and frame 4 is shift 8, 4, each moved "
                      "exactly.",
                      case(shift=keyed((0, (0, 0)), (4, (8, 4)))), [0, 2, 4]),
    "FX-OFFSET-014": ("Shift keyed from 0, 0 at frame 0 to 3, 1 at frame 4, linear: frame 2 "
                      "falls between whole pixels, at 1.5, 0.5, and is the plain case of that "
                      "shift, softened by the mix: the red dot is spread over four pixels, a "
                      "quarter covering at (1, 0).",
                      case(shift=keyed((0, (0, 0)), (4, (3, 1)))), [0, 2, 4]),
    "FX-OFFSET-015": ("Shift keyed from 0, 0 at frame 0 to 100000, 0 at frame 4, eased past "
                      "its end (132500 at frame 2): frame 2 is held at 100000, a whole number "
                      "of widths, so it is the drawing untouched, as frame 4 is; unheld it "
                      "would have slid four pixels.",
                      case(shift=keyed((0, (0, 0), OVERSHOOT), (4, (100000, 0)))), [0, 2, 4]),
    "FX-OFFSET-016": ("Shift 3, 0, the layer moved three pixels right: FX-OFFSET-002 moved; "
                      "the wrap stays inside the layer's own pixels, so the three columns left "
                      "of it stay empty.",
                      case(shift=(3, 0), move=3), [0, 3]),
    "FX-OFFSET-017": ("A drop shadow two pixels right, then shift 16, 0: the shadow grew the "
                      "layer two pixels on every side, to 20 by 14, and the wrap goes round "
                      "that, so 16 is not a whole width: it is shift -4 on the grown layer, "
                      "and the shadow and the drawing slide four pixels left.",
                      case(shift=(16, 0), shadow=True), [0]),
    "FX-OFFSET-018": ("A drop shadow two pixels right, then shift 20, 14, the grown layer's "
                      "whole width and height: the drawing with its shadow, as the drop shadow "
                      "alone draws it.",
                      case(shift=(20, 14), shadow=True), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-OFFSET-019": ("Shift 100001, 0: x above 100000.", case(shift=(100001, 0))),
    "FX-OFFSET-020": ("Shift 0, -100001: y below -100000.", case(shift=(0, -100001))),
    "FX-OFFSET-021": ("Shift 100000.5, 0: x past 100000 by half a pixel.",
                      case(shift=(100000.5, 0))),
    "FX-OFFSET-022": ("Shift keyed to 150000, 0 at frame 4.",
                      case(shift=keyed((0, (0, 0)), (4, (150000, 0))))),
    "FX-OFFSET-023": ("Shift keyed to 0, -100001 at frame 4.",
                      case(shift=keyed((0, (0, 0)), (4, (0, -100001))))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["move"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    t = comp["layers"][0]["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["move"], H / 2])
    effects = []
    if c["shadow"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.drop_shadow", "enabled": True,
                        "parameters": dict(SHADOW)})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.offset",
                    "enabled": True, "parameters": {"shift": setting_json(c["shift"])}})
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

    (OUT / "expected_offset.json").write_text(json.dumps(expected, indent=1) + "\n",
                                              encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    one = lambda fx: c[fx]["0"]  # noqa: E731

    def rolled(sx, sy):
        """A whole-pixel shift written out by hand: each pixel from (x - sx, y - sy), wrapped."""
        return [drawn[at((x - sx) % W, (y - sy) % H)] for y in range(H) for x in range(W)]

    def total(f):
        return [sum(p[k] for p in f) for k in range(4)]

    # The rule's own pieces: the mod is always 0 to w - 1, and whole shifts take no mixing.
    assert (-1) % W == W - 1 and (-17) % W == W - 1 and 35 % W == 3 and (-23) % H == 7
    layer = drawn_layer("tile")
    assert offset(layer, (0, 0))["px"] == drawn
    assert offset(layer, (-W, 3 * H))["px"] == drawn

    for fx, frames in c.items():
        for px in frames.values():
            for i, p in enumerate(px):
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (fx, i)
    # Nothing lost or gained: with the layer the frame's size, each channel's total is kept.
    for fx in ("FX-OFFSET-%03d" % n for n in range(1, 16)):
        for px in c[fx].values():
            assert all(abs(a - b) < 1e-9 for a, b in zip(total(px), total(drawn))), fx

    assert one("FX-OFFSET-001") == drawn
    # Whole-pixel shifts move every pixel exactly, wrapping.
    for fx, (sx, sy) in (("002", (3, 0)), ("003", (0, 2)), ("004", (-3, -2)), ("005", (8, 5)),
                         ("006", (0, 0)), ("007", (3, -3)), ("008", (0, 0)), ("009", (1, -1))):
        assert one("FX-OFFSET-" + fx) == rolled(sx, sy), fx
    red = R.working(TRACE)
    soft = R.working(SOFT)
    two, three, four, five = (one(f"FX-OFFSET-00{n}") for n in (2, 3, 4, 5))
    assert two[at(2, 0)] == red and all(two[at(1, y)] == soft for y in range(5, H))
    assert two[at(15, 0)] == EMPTY
    skin = R.working(SKIN)
    assert three[at(10, 0)] == skin == three[at(10, 1)] and three[at(15, 2)] == red
    white = R.working(WHITE)
    assert all(four[at(x, 9)] == white for x in (13, 14, 15, 0, 1)) and four[at(12, 8)] == red
    assert five[at(7, 5)] == red
    assert all(five[at(x, y)] == white for x, y in ((8, 6), (12, 6), (8, 9), (12, 9)))
    assert all(five[at(x, y)] == skin for x, y in ((1, 0), (5, 0), (1, 4), (5, 4)))
    assert (100000 % W, 100000 % H, (-99999) % W, 99999 % H) == (0, 0, 1, 9)
    # Half a pixel right: the even mix of each pixel and its left neighbour, wrapped.
    ten = one("FX-OFFSET-010")
    half = [[0.5 * a + 0.5 * b for a, b in zip(drawn[at((x - 1) % W, y)], drawn[at(x, y)])]
            for y in range(H) for x in range(W)]
    assert ten == half
    assert ten[at(0, 0)] == [v * 0.5 for v in red] == ten[at(15, 0)]
    assert ten[at(15, 5)] == [v * 0.5 for v in soft] and abs(ten[at(15, 5)][3] - 128 / 510) < 1e-15
    # -2.25, 1.75: from columns x + 2 and x + 3, rows y - 2 and y - 1, weights 3/4 and 1/4.
    eleven = one("FX-OFFSET-011")
    for y in range(H):
        for x in range(W):
            want = [0.0] * 4
            for dx, wx in ((2, 0.75), (3, 0.25)):
                for dy, wy in ((-2, 0.75), (-1, 0.25)):
                    s = drawn[at((x + dx) % W, (y + dy) % H)]
                    want = [a + b * wx * wy for a, b in zip(want, s)]
            assert all(abs(a - b) < 1e-15 for a, b in zip(eleven[at(x, y)], want)), (x, y)
    assert all(abs(a - 0.75 * b) < 1e-15 for a, b in zip(eleven[at(12, 0)], soft))
    assert one("FX-OFFSET-012") == render(case(shift=(0.5, 0.25)), 0)
    thirteen, fourteen, fifteen = c["FX-OFFSET-013"], c["FX-OFFSET-014"], c["FX-OFFSET-015"]
    assert thirteen["0"] == drawn and thirteen["2"] == rolled(4, 2) and thirteen["4"] == rolled(8, 4)
    assert fourteen["0"] == drawn and fourteen["4"] == rolled(3, 1)
    assert fourteen["2"] == render(case(shift=(1.5, 0.5)), 0)
    partial = lambda f: sum(0 < p[3] < 1 for p in f)  # noqa: E731
    assert partial(fourteen["2"]) > partial(drawn) and fourteen["2"][at(1, 0)][3] == 0.25
    raw = value_at(CASES["FX-OFFSET-015"][1]["shift"], 2)[0]
    assert raw > 100000 and round(raw) % W == 4
    assert fifteen["0"] == drawn == fifteen["2"] == fifteen["4"]
    assert frame(offset(layer, (raw, 0)), 0) != drawn
    # Moved: the same, moved, and nothing wraps into the three columns left of the layer.
    sixteen = c["FX-OFFSET-016"]
    assert sixteen["0"] == sixteen["3"]
    assert all(sixteen["0"][at(x, y)] == two[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(sixteen["0"][at(x, y)] == EMPTY for x in range(3) for y in range(H))
    # After a drop shadow: the wrap is round the grown 20 by 14 layer, not the drawing.
    shadowed = layer_of(case(shadow=True))
    assert (shadowed["left"], shadowed["top"], shadowed["w"], shadowed["h"]) == (-2, -2, 20, 14)
    seventeen, eighteen = one("FX-OFFSET-017"), one("FX-OFFSET-018")
    assert seventeen == frame(offset(shadowed, (-4, 0)), 0) != rolled(0, 0)
    assert all(seventeen[at(x, y)] == frame(shadowed, 0)[at(x + 4, y)]
               for x in range(W - 4) for y in range(H))
    assert eighteen == frame(shadowed, 0) != drawn
    assert eighteen[at(15, 6)][3] == 1 and drawn[at(15, 6)] == EMPTY  # the shadow shows
    print("checked")


if __name__ == "__main__":
    main()
