"""Directional blur, worked a second way.

D-92 adds `core.directional_blur`. It smears a layer along one direction, as a camera moving
during the exposure would: each pixel becomes the plain average of samples taken along a line
through it. `direction` is in degrees, clockwise from up, so 0 streaks up and down and 90 left
and right; `length` is the whole streak in pixels, half on each side. It is this program's own
method, modelled on After Effects' Directional Blur; nothing is ported. Document 21 is the rule
in words; this file is the reference for the numbers document 25 pins against it.

The rule. With u = (sin direction, -cos direction), exactly (0, -1), (1, 0), (0, 1) or (-1, 0)
when the direction is a whole multiple of 90 degrees, n = ceil(length) + 1 and, when n > 1,
t_k = -length / 2 + k * length / (n - 1) for k = 0 to n - 1 (t_0 = 0 when n = 1), the output at
a pixel whose centre is c is (1 / n) times the sum over k of document 21's bilinear sample of
the layer at c + t_k * u, transparent outside the layer. Premultiplied red, green, blue and
alpha are averaged alike. The layer grows by ceil(length / 2) on every side, so a streak that
leaves the drawing's edge is kept.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says. The drawing goes into `Fixtures/directional_blur/media`, the projects into
`Fixtures/directional_blur`, and the expected frames into
`Fixtures/directional_blur/expected_directional_blur.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/directional_blur_reference.py

**D-98, proposed and not accepted**, is a rule a running sum can work, run with `--d98`: it
writes `expected_directional_blur_d98.json` beside the D-92 file and leaves that file alone.
When the direction is mostly across (|u_x| >= |u_y|), the picture is read along lines of slope
s = u_y / u_x, each line sampled at every column centre by the bilinear sample. With
d = length / ceil(length) and H = |u_x| * (length + d) / 2, the columns j from the line's own
are weighted w_j = the integral over t from -H to H of tent(j - t), tent(v) = max(0, 1 - |v|),
and divided by 2H, their sum: the line, drawn straight between its column samples, averaged
over 2H columns. A pixel is the straight mix of the two lines just above and below its centre
in its column, the lines being spaced one pixel apart down each column. Mostly down, the same
with across and down exchanged. Length 0 is the drawing untouched.
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402

W, H = 16, 10
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "directional_blur"
TOLERANCE = 2e-5  # document 25's default for a filter
LENGTH = (0, 500)
DIRECTION = (-3600, 3600)


# --- the rule -------------------------------------------------------------------------------

def working(p):
    """Document 21's PNG reading: sRGB to linear, then premultiplied."""
    a = p[3] / 255
    return [srgb_to_linear(p[c] / 255) * a for c in range(3)] + [a]


def bilinear(layer, x, y):
    """Document 21's sample from pixel centres, transparent outside the layer."""
    fx, fy = x - 0.5, y - 0.5
    x0, y0 = math.floor(fx), math.floor(fy)
    ux, uy = fx - x0, fy - y0
    out = [0.0] * 4
    for dy, wy in ((0, 1 - uy), (1, uy)):
        for dx, wx in ((0, 1 - ux), (1, ux)):
            sx, sy = x0 + dx, y0 + dy
            if 0 <= sx < W and 0 <= sy < H and wx * wy:
                p = layer[sy * W + sx]
                for i in range(4):
                    out[i] += p[i] * wx * wy
    return out


# At a whole quarter turn u is exact, so a streak straight across or straight down takes nothing
# at all from the rows or columns beside it, where sine and cosine in floating point would give
# a trace.
QUARTERS = {0: (0.0, -1.0), 90: (1.0, 0.0), 180: (0.0, 1.0), 270: (-1.0, 0.0)}


def blurred(layer, direction, length, x, y):
    """The output at the pixel (x, y) of layer space, which may lie in the grown border."""
    n = math.ceil(length) + 1
    ts = [0.0] if n == 1 else [-length / 2 + k * length / (n - 1) for k in range(n)]
    u = QUARTERS.get(direction % 360)
    if u is None:
        a = math.radians(direction)
        u = (math.sin(a), -math.cos(a))
    total = [0.0] * 4
    for t in ts:
        s = bilinear(layer, x + 0.5 + t * u[0], y + 0.5 + t * u[1])
        for i in range(4):
            total[i] += s[i]
    return [v / n for v in total]


# --- D-98's rule, proposed ------------------------------------------------------------------

def tent_integral(a, b):
    """The integral of tent(v) = max(0, 1 - |v|) from a to b."""
    def up_to(v):
        v = min(1.0, max(-1.0, v))
        return (1 + v) ** 2 / 2 if v < 0 else 1 - (1 - v) ** 2 / 2
    return up_to(b) - up_to(a)


def line_mean(layer, u, weights, x, y):
    """D-98's reading along lines: the pixel (x, y)'s mix of the two lines round its centre,
    each line's value the weighted sum over its column samples, weights [(j, w_j)] summing
    to 1. Mostly down, the same with across and down exchanged."""
    across = abs(u[0]) >= abs(u[1])
    if not across:
        u, x, y = (u[1], u[0]), y, x
    s = u[1] / u[0]
    beta = y + 0.5 - x * s  # the line through the centre is at beta + X * s in column X
    lo = math.floor(beta - 0.5) + 0.5
    f = beta - lo
    out = [0.0] * 4
    for b, wb in ((lo, 1 - f), (lo + 1, f)):
        if not wb:
            continue
        for j, wj in weights:
            px, py = x + j + 0.5, b + (x + j) * s
            v = bilinear(layer, px, py) if across else bilinear(layer, py, px)
            for i in range(4):
                out[i] += v[i] * wj * wb
    return out


def blurred_d98(layer, direction, length, x, y):
    """D-98's output at the pixel (x, y) of layer space."""
    if length == 0:
        return bilinear(layer, x + 0.5, y + 0.5)
    u = QUARTERS.get(direction % 360)
    if u is None:
        a = math.radians(direction)
        u = (math.sin(a), -math.cos(a))
    d = length / math.ceil(length)
    h = max(abs(u[0]), abs(u[1])) * (length + d) / 2
    reach = math.ceil(h + 1)
    weights = [(j, tent_integral(j - h, j + h) / (2 * h)) for j in range(-reach, reach + 1)]
    return line_mean(layer, u, [(j, w) for j, w in weights if w], x, y)


RULE = {"blurred": blurred}


# --- the drawing ----------------------------------------------------------------------------

LINE = (30, 26, 36, 255)        # the face drawing's line, #1e1a24
SKIN = (246, 214, 190, 255)     # #f6d6be
SOFT = (200, 40, 40, 128)       # a red at half covering, #c82828
NONE = S.NONE


def bars(x, y):
    """A line down the drawing's left edge, column 0, rows 2 to 7; a block of skin in columns 5
    to 9 and rows 3 to 6; and one red pixel at half covering at (12, 4)."""
    if x == 0 and 2 <= y <= 7:
        return LINE
    if 5 <= x <= 9 and 3 <= y <= 6:
        return SKIN
    if (x, y) == (12, 4):
        return SOFT
    return NONE


DRAWINGS = {"bars": [[bars(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(direction=0, length=4, shift=0, name="bars"):
    return {"drawing": name, "direction": direction, "length": length, "shift": shift}


def clamp(v, r):
    return min(r[1], max(r[0], v))


def render(c, frame_no):
    layer = [working(p) for row in DRAWINGS[c["drawing"]] for p in row]
    direction = clamp(value_at(c["direction"], frame_no), DIRECTION)
    length = clamp(value_at(c["length"], frame_no), LENGTH)
    return [RULE["blurred"](layer, direction, length, x - c["shift"], y)
            for y in range(H) for x in range(W)]


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(length=0, shift=c["shift"], name=c["drawing"]), 0)


CASES = {
    "FX-DIRBLUR-001": ("Direction 0, length 4: every edge streaks up and down, two pixels each "
                       "way, and left and right stay sharp.",
                       case(), [0]),
    "FX-DIRBLUR-002": ("Direction 90, length 4: the same streak left and right, and up and down "
                       "stay sharp.",
                       case(direction=90), [0]),
    "FX-DIRBLUR-003": ("Direction 270, the opposite way: the streak runs both ways, so this is "
                       "FX-DIRBLUR-002.",
                       case(direction=270), [0]),
    "FX-DIRBLUR-004": ("Direction 45, length 6: a diagonal streak, up and right and down and "
                       "left, between pixels.",
                       case(direction=45, length=6), [0]),
    "FX-DIRBLUR-005": ("Length 0: the drawing, untouched.",
                       case(length=0), [0]),
    "FX-DIRBLUR-006": ("Length 1, direction 90: two samples half a pixel either side, so the "
                       "block's left edge column is three quarters covered and the column "
                       "outside it one quarter.",
                       case(direction=90, length=1), [0]),
    "FX-DIRBLUR-007": ("Length 2.5, direction 90: four samples, spaced evenly across 2.5 "
                       "pixels.",
                       case(direction=90, length=2.5), [0]),
    "FX-DIRBLUR-008": ("Direction 3600, ten turns: this is FX-DIRBLUR-001.",
                       case(direction=3600), [0]),
    "FX-DIRBLUR-009": ("Direction 90, length 6, moved three pixels right: the line on the "
                       "drawing's left edge streaks three pixels past it into the grown border.",
                       case(direction=90, length=6, shift=3), [0, 3]),
    "FX-DIRBLUR-010": ("Length keyed from 0 at frame 0 to 8 at frame 4, direction 90, linear: "
                       "frame 0 untouched, frame 2 at length 4 is FX-DIRBLUR-002, frame 4 at 8.",
                       case(direction=90, length=keyed((0, 0), (4, 8))), [0, 2, 4]),
    "FX-DIRBLUR-011": ("Direction keyed from 0 at frame 0 to 90 at frame 4, length 4: frame 0 is "
                       "FX-DIRBLUR-001, frame 2 streaks at 45 degrees, frame 4 is FX-DIRBLUR-002.",
                       case(direction=keyed((0, 0), (4, 90))), [0, 2, 4]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-DIRBLUR-012": ("Length 501, above 500.", case(length=501)),
    "FX-DIRBLUR-013": ("Length -1, below 0.", case(length=-1)),
    "FX-DIRBLUR-014": ("Direction 3601, past ten turns.", case(direction=3601)),
    "FX-DIRBLUR-015": ("Length keyed to 600 at frame 4.", case(length=keyed((0, 0), (4, 600)))),
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
        "instance_id": "fx-0-0", "type_id": "core.directional_blur", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in ("direction", "length")}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    d98 = "--d98" in sys.argv
    if d98:
        RULE["blurred"] = blurred_d98
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

    name = "expected_directional_blur_d98.json" if d98 else "expected_directional_blur.json"
    (OUT / name).write_text(json.dumps(expected, indent=1) + "\n", encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b: all(abs(p - q) < 1e-12 for u, v in zip(a, b)  # noqa: E731
                            for p, q in zip(u, v))
    one, two = c["FX-DIRBLUR-001"]["0"], c["FX-DIRBLUR-002"]["0"]

    d98 = RULE["blurred"] is blurred_d98
    # Vertical: row 4 of the block takes rows 2 to 6, four of them skin, so four fifths covered;
    # its top edge spreads to row 1, but a row-4 pixel beside the block stays empty. D-98 weighs
    # rows 2 to 6 at 1, 1, 1, 1 and 7/8 over 5, and row 7 at 1/8, so rows 3 to 6 give 3.875 / 5.
    assert abs(one[at(7, 4)][3] - (3.875 / 5 if d98 else 0.8)) < 1e-12 and one[at(7, 1)][3] > 0
    assert one[at(4, 4)] == [0.0] * 4 and one[at(10, 4)] == [0.0] * 4
    # Horizontal: the reverse.
    assert two[at(7, 1)] == [0.0] * 4 and two[at(4, 4)][3] > 0 and two[at(3, 4)][3] > 0
    assert near(c["FX-DIRBLUR-003"]["0"], two)
    assert near(c["FX-DIRBLUR-008"]["0"], one)
    assert c["FX-DIRBLUR-005"]["0"] == drawn
    # Length 1 at 90: samples half a pixel either side, each between two pixels, so the block's
    # left edge column is three quarters covered and the one outside it a quarter.
    six = c["FX-DIRBLUR-006"]["0"]
    assert abs(six[at(5, 4)][3] - 0.75) < 1e-12 and abs(six[at(4, 4)][3] - 0.25) < 1e-12
    # Four samples across 2.5 pixels: two columns left of the block, only the last sample, at
    # 4.75, reaches it, a quarter into its first column, so the covering is a quarter of a quarter.
    # D-98: H = (2.5 + 2.5 / 3) / 2 = 5 / 3, and the column two along weighs the tent from 1/3 to
    # 1, 2/9, over 2H = 10 / 3, so 1/15.
    seven = c["FX-DIRBLUR-007"]["0"]
    assert abs(seven[at(3, 4)][3] - (1 / 15 if d98 else 1 / 16)) < 1e-12
    # The streak leaves the drawing: comp columns 0 to 2 are layer columns -3 to -1.
    nine = c["FX-DIRBLUR-009"]
    assert nine["0"] == nine["3"] and all(nine["0"][at(x, 4)][3] > 0 for x in range(3))
    ten, eleven = c["FX-DIRBLUR-010"], c["FX-DIRBLUR-011"]
    assert ten["0"] == drawn and near(ten["2"], two) and ten["4"] != two
    assert near(eleven["0"], one) and near(eleven["4"], two)
    assert near(eleven["2"], render(case(direction=45), 0))
    # Averaging keeps the covering: the block's five pixels of row 3, spread over columns 3 to 11.
    # D-98's ends reach a column further, 2 to 12, where the line in column 0 also reaches, with
    # 7/8 and 1/8 over 5 of its own covering.
    if d98:
        assert abs(sum(two[at(x, 3)][3] for x in range(2, 13)) - 5 - 1 / 5) < 1e-12
    else:
        assert abs(sum(two[at(x, 3)][3] for x in range(3, 12)) - 5) < 1e-12
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (name, p)


if __name__ == "__main__":
    main()
