"""Mirror, worked a second way.

D-153 adds `core.mirror`, a reflection: a straight line is drawn through the drawing, the picture
on one side of it is kept exactly, and the other side shows the kept side's reflection, as if a
looking-glass stood on the line. `center` is a point the line runs through, per cent of the
drawing; `angle` turns the line: at 0 it runs straight up and down, the right side is kept and
the left side shows the right side's reflection; at 90 the line lies across, the bottom is kept
and the top shows it; at 180 the left is kept, at 270 the top. It is this program's own method,
modelled on After Effects' Mirror in spirit and not claimed to match it; nothing is ported.
Document 21 is the rule in words; this file is the reference for the numbers document 25 pins
against it.

The rule. The line's normal is n = u(angle + 90), u(t) = (sin t, -cos t), exactly (1, 0),
(0, 1), (-1, 0) or (0, -1) when angle is a whole multiple of 90. With c the centre point,
o + (cx / 100 * w, cy / 100 * h), and P a pixel's centre, delta = (P - c) . n: at delta >= 0
the input pixel is kept exactly; else the output is document 21's bilinear sample of the input
at P - 2 delta n, its reflection across the line, transparent outside the input. The layer does
not grow. The two sides meet without a seam: on the line itself the reflection is the point
itself, so the choice is not a cliff.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says, drawn below. The drawing goes into `Fixtures/mirror/media`, the projects into
`Fixtures/mirror`, and the expected frames into `Fixtures/mirror/expected_mirror.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/mirror_reference.py
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

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "mirror"
TOLERANCE = 2e-5  # document 25's default for a filter
CENTER, ANGLE = (-1000, 1000), (-3600, 3600)
NAMES = ("center", "angle")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def u(theta):
    """The direction theta degrees clockwise from straight up, exact at whole quarter turns."""
    if theta % 90 == 0:
        return ((0.0, -1.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 0.0))[int(theta // 90) % 4]
    r = math.radians(theta)
    return math.sin(r), -math.cos(r)


def side(layer, center, angle, x, y):
    """delta, the reflection's normal and the pixel's centre, for pixel (x, y) of the layer."""
    cx, cy = center[0] / 100 * layer["w"], center[1] / 100 * layer["h"]
    nx, ny = u(angle + 90)
    px, py = x + 0.5, y + 0.5
    return (px - cx) * nx + (py - cy) * ny, (nx, ny), (px, py)


def mirrored(layer, center, angle, x, y):
    """The output at pixel (x, y) of the layer's own space."""
    d, (nx, ny), (px, py) = side(layer, center, angle, x, y)
    if d >= 0:
        return layer["px"][y * layer["w"] + x]
    return bilinear(layer, px - 2 * d * nx, py - 2 * d * ny)


def mirror(layer, center, angle):
    return [mirrored(layer, center, angle, x, y)
            for y in range(layer["h"]) for x in range(layer["w"])]


# --- the drawing ----------------------------------------------------------------------------

LINE, SKIN, TRACE, NONE = R.LINE, R.SKIN, R.TRACE, S.NONE
BAND = (58, 111, 216, 255)            # #3a6fd8, the ball's blue band
SOFT_SKIN = SKIN[:3] + (128,)         # the skin at half covering, a soft tip
SOFT_LINE = LINE[:3] + (128,)         # the line at half covering, a soft edge


def arrow(x, y):
    """An arrow pointing right: a blue shaft in rows 4 and 5, columns 2 to 10; a red head in
    columns 11 to 13 (rows 2 to 7, 3 to 6, 4 and 5), tipped by the skin at half covering in
    column 14, rows 4 and 5. Above it on the left a dark block, the line, in rows 1 and 2,
    columns 1 to 3, with a half-covering edge in column 4; below it a skin block in rows 7 and
    8, columns 5 to 9, across the middle. Everything else is empty."""
    if 1 <= y <= 2 and 1 <= x <= 3:
        return LINE
    if 1 <= y <= 2 and x == 4:
        return SOFT_LINE
    if 4 <= y <= 5 and 2 <= x <= 10:
        return BAND
    if (x == 11 and 2 <= y <= 7) or (x == 12 and 3 <= y <= 6) or (x == 13 and 4 <= y <= 5):
        return TRACE
    if x == 14 and 4 <= y <= 5:
        return SOFT_SKIN
    if 7 <= y <= 8 and 5 <= x <= 9:
        return SKIN
    return NONE


DRAWINGS = {"arrow": [[arrow(x, y) for x in range(W)] for y in range(H)]}


def drawn_layer(name):
    return {"px": [R.working(p) for row in DRAWINGS[name] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


# --- the cases ------------------------------------------------------------------------------

def case(center=(50, 50), angle=0, shift=0):
    return {"drawing": "arrow", "center": center, "angle": angle, "shift": shift}


def clamp(v, r):
    return min(r[1], max(r[0], v))


def render(c, frame_no):
    center = [clamp(v, CENTER) for v in value_at(c["center"], frame_no)]
    angle = clamp(value_at(c["angle"], frame_no), ANGLE)
    return R.frame(mirror(drawn_layer(c["drawing"]), center, angle), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return R.frame(drawn_layer("arrow")["px"], c["shift"])


CASES = {
    "FX-MIRROR-001": ("The settings as they start: centre 50, 50, the point (8, 5), angle 0. "
                      "The line runs straight down between columns 7 and 8; the right half is "
                      "kept exactly and the left half shows its reflection, column 7 taking "
                      "column 8, 6 taking 9, down to 0 taking 15: the arrow gets a second head "
                      "pointing left, with the soft tip in column 1, the skin block spreads to "
                      "columns 6 to 9, and the dark block is gone.",
                      case(), [0]),
    "FX-MIRROR-002": ("Angle 90: the line lies across between rows 4 and 5; the bottom half is "
                      "kept and the top half shows its reflection, row 4 taking row 5 up to row "
                      "0 taking row 9. The dark block is gone and the skin block shows in rows "
                      "1 and 2 as well as 7 and 8; the arrow, even top to bottom, is unchanged.",
                      case(angle=90), [0]),
    "FX-MIRROR-003": ("Angle 180: the left half is kept and the right half shows its "
                      "reflection, column 8 taking 7 up to 15 taking 0: the head and tip are "
                      "gone, the shaft runs on to column 13, the dark block shows again in "
                      "columns 12 to 14 with its soft edge in column 11, and the skin block "
                      "spans columns 5 to 10.",
                      case(angle=180), [0]),
    "FX-MIRROR-004": ("Angle 270: the top half is kept and the bottom half shows its "
                      "reflection, row 5 taking row 4 down to row 9 taking row 0: the skin "
                      "block is gone and the dark block shows in rows 7 and 8 too.",
                      case(angle=270), [0]),
    "FX-MIRROR-005": ("Angle 360, a whole turn: FX-MIRROR-001.",
                      case(angle=360), [0]),
    "FX-MIRROR-006": ("Angle -90, a quarter turn the other way: FX-MIRROR-004.",
                      case(angle=-90), [0]),
    "FX-MIRROR-007": ("Angle 45: the line runs corner-wise through the point (8, 5), down to "
                      "the left, and the part below and right of it is kept. Each pixel above "
                      "and left of it takes the pixel mirrored across it, row and column "
                      "swapped about it: pixel (x, y) takes (12 - y, 12 - x), and one whose "
                      "mirror falls outside the drawing is empty.",
                      case(angle=45), [0]),
    "FX-MIRROR-008": ("Angle 30: the line leans a third of a quarter turn; the kept side is "
                      "exact and the reflected side falls between pixels, so its colours are "
                      "blends of neighbours, softening the reflected edges.",
                      case(angle=30), [0]),
    "FX-MIRROR-009": ("Centre 53.125, 50, the point (8.5, 5): the line runs down the middle "
                      "of column 8, which is kept, with column 7 taking column 9 up to column 0 "
                      "taking column 16, past the drawing, so column 0 is empty.",
                      case(center=(53.125, 50)), [0]),
    "FX-MIRROR-010": ("Centre 51.5625, 50, the point (8.25, 5): the line runs a quarter of "
                      "the way into column 8. The right is kept, and each pixel on the left "
                      "reads its mirror half-way between two pixels, so it is an even blend of "
                      "the two: column 7 half column 8 and half column 9, and column 5, where "
                      "the shaft meets the head, half blue and half red.",
                      case(center=(51.5625, 50)), [0]),
    "FX-MIRROR-011": ("Centre 25, 50, the point (4, 5): columns 4 to 15 are kept and columns "
                      "0 to 3 show columns 7 to 4: the dark block gives way to its own soft edge "
                      "in column 3, the shaft reaches the left edge, and the skin block shows "
                      "again in columns 0 to 2.",
                      case(center=(25, 50)), [0]),
    "FX-MIRROR-012": ("Centre 0, 50: the line is the drawing's left edge and the whole drawing "
                      "is on the kept side: the drawing, untouched.",
                      case(center=(0, 50)), [0]),
    "FX-MIRROR-013": ("Centre 100, 50: the line is the drawing's right edge and the whole "
                      "drawing is on the reflected side, whose mirror lies past the edge: every "
                      "pixel is empty.",
                      case(center=(100, 50)), [0]),
    "FX-MIRROR-014": ("Centre 50, 0 at angle 0: the line still runs straight down through "
                      "column 8's left edge, so the centre's height changes nothing: "
                      "FX-MIRROR-001.",
                      case(center=(50, 0)), [0]),
    "FX-MIRROR-015": ("Centre keyed from 50, 50 at frame 0 to 0, 50 at frame 4, linear: frame "
                      "0 is FX-MIRROR-001, frame 2 is centre 25, 50, FX-MIRROR-011, and frame 4 "
                      "the drawing, untouched, FX-MIRROR-012.",
                      case(center=keyed((0, (50, 50)), (4, (0, 50)))), [0, 2, 4]),
    "FX-MIRROR-016": ("Angle keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 is "
                      "FX-MIRROR-001, frame 2 is angle 90, FX-MIRROR-002, frame 4 is "
                      "FX-MIRROR-003.",
                      case(angle=keyed((0, 0), (4, 180))), [0, 2, 4]),
    "FX-MIRROR-017": ("Angle eased from 0 at frame 0 to 3600 at frame 4 on a curve that "
                      "overshoots: at frame 2 it would be 4770, a quarter turn past a whole "
                      "number of turns, is held at 3600 and is FX-MIRROR-001, as frames 0 and 4 "
                      "are.",
                      case(angle=keyed((0, 0, OVERSHOOT), (4, 3600))), [0, 2, 4]),
    "FX-MIRROR-018": ("FX-MIRROR-001 moved three pixels right: the centre is per cent of the "
                      "drawing, so the line moves with it and the picture is FX-MIRROR-001 "
                      "moved; nothing grows, so the three columns left of the drawing stay "
                      "empty.",
                      case(shift=3), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-MIRROR-019": ("Centre 1001, 50, past ten widths.", case(center=(1001, 50))),
    "FX-MIRROR-020": ("Centre 50, -1001, past ten heights.", case(center=(50, -1001))),
    "FX-MIRROR-021": ("Angle 3601, above 3600.", case(angle=3601)),
    "FX-MIRROR-022": ("Angle -3601, below -3600.", case(angle=-3601)),
    "FX-MIRROR-023": ("Angle keyed to 4000 at frame 4.", case(angle=keyed((0, 0), (4, 4000)))),
    "FX-MIRROR-024": ("Centre keyed to 50, 1500 at frame 4.",
                      case(center=keyed((0, (50, 50)), (4, (50, 1500))))),
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
        "instance_id": "fx-0-0", "type_id": "core.mirror", "enabled": True,
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

    (OUT / "expected_mirror.json").write_text(json.dumps(expected, indent=1) + "\n",
                                              encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u_, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u_, v))
    layer = drawn_layer("arrow")
    pix = lambda x, y: drawn[at(x, y)] if 0 <= x < W and 0 <= y < H else EMPTY  # noqa: E731

    def mapped(f):
        """The drawing with every pixel (x, y) taking the pixel f(x, y), or kept on None."""
        return [pix(*f(x, y)) if f(x, y) else drawn[at(x, y)] for y in range(H) for x in range(W)]

    # The rule's pieces: u is exact at quarter turns and the usual direction between; on the
    # line the reflection is the point itself, so the two sides meet without a seam.
    assert [u(t) for t in (0, 90, 180, 270, 360, -90, 3690)] == [
        (0.0, -1.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0), (-1.0, 0.0), (1.0, 0.0)]
    assert abs(u(30)[0] - 0.5) < 1e-15 and abs(u(30)[1] + math.sqrt(3) / 2) < 1e-15
    assert side(layer, (50, 50), 0, 8, 3)[0] == 0.5 and side(layer, (50, 50), 0, 7, 3)[0] == -0.5
    assert side(layer, (50, 50), 90, 3, 5)[0] == 0.5 and side(layer, (50, 50), 90, 3, 4)[0] == -0.5
    on = [(x, y) for x in range(W) for y in range(H) if side(layer, (53.125, 50), 0, x, y)[0] == 0]
    assert on == [(8, y) for y in range(H)]
    assert near([bilinear(layer, 8.5, 3.5)], [drawn[at(8, 3)]])
    # The drawing: soft edges, empty pixels and both halves different.
    assert drawn[at(4, 1)][3] == drawn[at(14, 4)][3] == 128 / 255
    assert drawn[at(0, 0)] == EMPTY and drawn[at(2, 1)] != EMPTY and drawn[at(13, 1)] == EMPTY

    # 001: right kept exactly, left the right's reflection.
    one = c["FX-MIRROR-001"]["0"]
    assert one == mapped(lambda x, y: None if x >= 8 else (15 - x, y))
    assert one[at(1, 4)] == drawn[at(14, 4)] and one[at(1, 4)][3] == 128 / 255   # soft tip
    assert one[at(4, 3)] == drawn[at(11, 3)] != drawn[at(4, 3)]                  # second head
    assert all(one[at(x, 1)] == EMPTY == one[at(x - 1, 2)] for x in range(2, 5))  # block gone
    assert all(one[at(x, y)] == drawn[at(8, 7)] for x in range(6, 10) for y in (7, 8))
    two = c["FX-MIRROR-002"]["0"]
    assert two == mapped(lambda x, y: None if y >= 5 else (x, 9 - y))
    assert all(two[at(x, y)] == drawn[at(x, y)] for x in range(W) for y in range(3, 7))
    assert all(two[at(x, y)] == drawn[at(x, y)] for x in range(10, W) for y in range(H))
    assert all(two[at(x, y)] == drawn[at(5, 7)] for x in range(5, 10) for y in (1, 2))
    assert all(two[at(x, y)] == EMPTY for x in range(1, 5) for y in (1, 2))
    three = c["FX-MIRROR-003"]["0"]
    assert three == mapped(lambda x, y: None if x <= 7 else (15 - x, y))
    assert all(three[at(x, y)][3] == 0 for x in range(14, 16) for y in (4, 5))
    assert all(three[at(x, y)] == drawn[at(2, 4)] for x in range(2, 14) for y in (4, 5))
    assert three[at(11, 1)] == drawn[at(4, 1)] and three[at(13, 2)] == drawn[at(2, 2)]
    four = c["FX-MIRROR-004"]["0"]
    assert four == mapped(lambda x, y: None if y <= 4 else (x, 9 - y))
    assert all(four[at(x, y)] == drawn[at(x, 9 - y)] for x in range(1, 5) for y in (7, 8))
    assert all(four[at(x, y)][3] == 0 for x in range(5, 11) for y in (7, 8))
    assert c["FX-MIRROR-005"]["0"] == one and c["FX-MIRROR-006"]["0"] == four
    seven = c["FX-MIRROR-007"]["0"]
    assert near(seven, mapped(lambda x, y: None if x + y >= 12 else (12 - y, 12 - x)))
    assert seven != drawn
    # 008: the kept side exact; the reflected side has blends (a colour not in the drawing).
    eight = c["FX-MIRROR-008"]["0"]
    kept = reflected = blends = 0
    for y in range(H):
        for x in range(W):
            if side(layer, (50, 50), 30, x, y)[0] >= 0:
                assert eight[at(x, y)] == drawn[at(x, y)]
                kept += 1
            else:
                reflected += 1
                blends += eight[at(x, y)] not in drawn
    assert kept > 40 and reflected > 40 and blends > 10
    assert c["FX-MIRROR-009"]["0"] == mapped(lambda x, y: None if x >= 8 else (16 - x, y))
    ten = c["FX-MIRROR-010"]["0"]
    for y in range(H):
        for x in range(W):
            want = drawn[at(x, y)] if x >= 8 else [
                (pix(16 - x, y)[i] + pix(15 - x, y)[i]) / 2 for i in range(4)]
            assert near([ten[at(x, y)]], [want]), (x, y)
    assert ten[at(5, 4)] not in drawn  # half blue, half red
    eleven = c["FX-MIRROR-011"]["0"]
    assert eleven == mapped(lambda x, y: None if x >= 4 else (7 - x, y))
    assert eleven[at(3, 1)] == drawn[at(4, 1)] and eleven[at(0, 4)] == drawn[at(7, 4)]
    assert all(eleven[at(x, y)] == drawn[at(5, 7)] for x in range(3) for y in (7, 8))
    assert c["FX-MIRROR-012"]["0"] == drawn
    assert all(p == EMPTY for p in c["FX-MIRROR-013"]["0"])
    assert c["FX-MIRROR-014"]["0"] == one
    fifteen = c["FX-MIRROR-015"]
    assert fifteen["0"] == one and fifteen["2"] == eleven and fifteen["4"] == drawn
    sixteen = c["FX-MIRROR-016"]
    assert sixteen["0"] == one and sixteen["2"] == two and sixteen["4"] == three
    k = keyed((0, 0, OVERSHOOT), (4, 3600))
    assert abs(value_at(k, 2) - 4770) < 1e-9
    assert all(v == one for v in c["FX-MIRROR-017"].values())
    unheld = mirror(layer, (50, 50), value_at(k, 2))  # were it not held, a quarter turn more
    assert near(unheld, two, 1e-9) and not near(unheld, one, 1e-3)
    moved = c["FX-MIRROR-018"]["0"]
    assert moved == [one[at(x - 3, y)] if x >= 3 else EMPTY for y in range(H) for x in range(W)]
    # Everywhere: the covering inside 0 to 1, and no colour past its covering.
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
