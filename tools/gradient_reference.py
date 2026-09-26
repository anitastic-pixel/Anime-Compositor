"""Gradient, worked a second way.

D-114 adds `core.gradient`: the paraffin (パラ) of anime finishing, a colour gradient laid over
a cel. Between two points, `start` and `end`, each per cent of the drawing's own width and
height as Radial Blur's centre is, it lays a colour that runs from `start_color` to
`end_color`, at a strength that runs from `start_opacity` to `end_opacity`, and mixes it into
each pixel that shows by `blend`: "normal" lays the colour on, "multiply" darkens by it, "screen"
lightens by it and "add" adds it. Pixels that do not show stay as they are, and every pixel keeps
its own covering, so the gradient never spills past the cel. It is this program's own method,
modelled on After Effects' Gradient Ramp laid over a layer; nothing is ported. Document 21 is
the rule in words; this file is the reference for the numbers document 25 pins against it.

The rule. At a pixel with covering a > 0, P its centre (x + 0.5, y + 0.5) in layer space and S
and E the two points in layer pixels: with `shape` "linear", t = clamp(((P - S) . (E - S)) /
|E - S|^2, 0, 1); with "radial", t = clamp(|P - S| / |E - S|, 0, 1); when S == E, t = 1. The
colour is mixed in encoded values, G_enc = start + t (end - start), each channel its 8-bit value
/ 255, and G = srgb_to_linear(G_enc). The strength is o = (start_opacity + t (end_opacity -
start_opacity)) / 100. With b the pixel's straight linear colour, f = G (normal), b * G
(multiply), 1 - (1 - b)(1 - G) (screen) or b + G (add), and out = ((b + o (f - b)) * a, a).

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: a cel, drawn below. The drawing goes into `Fixtures/gradient/media`, the projects
into `Fixtures/gradient`, and the expected frames into `Fixtures/gradient/expected_gradient.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/gradient_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "gradient"
TOLERANCE = 2e-5  # document 25's default for a filter
OPACITY = (0, 100)
POINT = (-1000, 1000)
NAMES = ("shape", "start", "end", "start_color", "end_color", "start_opacity", "end_opacity",
         "blend")


# --- the rule -------------------------------------------------------------------------------

BLENDS = {
    "normal": lambda b, g: g,
    "multiply": lambda b, g: b * g,
    "screen": lambda b, g: 1 - (1 - b) * (1 - g),
    "add": lambda b, g: b + g,
}


def along(shape, px, py, s, e):
    """How far along the gradient the point (px, py) is, 0 at the start and 1 at the end."""
    ex, ey = e[0] - s[0], e[1] - s[1]
    dx, dy = px - s[0], py - s[1]
    ll = ex * ex + ey * ey
    if ll == 0:
        return 1.0
    if shape == "linear":
        t = (dx * ex + dy * ey) / ll
    else:
        t = math.sqrt(dx * dx + dy * dy) / math.sqrt(ll)
    return min(1.0, max(0.0, t))


def gradient(pixels, shape, start, end, start_color, end_color, start_opacity, end_opacity,
             blend, w=W, h=H):
    """`pixels` are 8-bit straight RGBA, rows top to bottom, a drawing w by h."""
    s = (start[0] / 100 * w, start[1] / 100 * h)
    e = (end[0] / 100 * w, end[1] / 100 * h)
    c0 = [v / 255 for v in R.hex_color(start_color.lower())]
    c1 = [v / 255 for v in R.hex_color(end_color.lower())]
    mix = BLENDS[blend]
    out = []
    for i, p in enumerate(pixels):
        if p[3] == 0:
            out.append(R.working(p))
            continue
        t = along(shape, i % w + 0.5, i // w + 0.5, s, e)
        g = [srgb_to_linear(u + t * (v - u)) for u, v in zip(c0, c1)]
        o = (start_opacity + t * (end_opacity - start_opacity)) / 100
        a = p[3] / 255
        b = [srgb_to_linear(p[c] / 255) for c in range(3)]
        out.append([(b[c] + o * (mix(b[c], g[c]) - b[c])) * a for c in range(3)] + [a])
    return out


# --- the drawing ----------------------------------------------------------------------------

LINE = R.LINE                    # #1e1a24
SOFT = R.SOFT                    # the line at half covering, its antialiased edge
SKIN = R.SKIN                    # #f6d6be
SHADE = (220, 160, 140, 255)     # the skin's shadow, #dca08c
NONE = S.NONE


def cel(x, y):
    """A box of line in columns 2 to 14 and rows 1 to 8, filled with skin, its right part,
    columns 10 to 13, in shadow, with a half-covering edge down its left side in column 1.
    Column 0, column 15 and rows 0 and 9 are empty."""
    if not (1 <= x <= 14 and 1 <= y <= 8):
        return NONE
    if x == 1:
        return SOFT
    if x in (2, 14) or y in (1, 8):
        return LINE
    if x >= 10:
        return SHADE
    return SKIN


DRAWINGS = {"cel": [[cel(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(shape="linear", start=(50, 0), end=(50, 100), start_color="#ffffff",
         end_color="#6450a0", start_opacity=0, end_opacity=50, blend="multiply", shift=0):
    return {"drawing": "cel", "shape": shape, "start": start, "end": end,
            "start_color": start_color, "end_color": end_color,
            "start_opacity": start_opacity, "end_opacity": end_opacity, "blend": blend,
            "shift": shift}


def clamp(v, r):
    return min(r[1], max(r[0], v))


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    point = lambda k: [clamp(v, POINT) for v in value_at(c[k], frame_no)]  # noqa: E731
    opacity = lambda k: clamp(value_at(c[k], frame_no), OPACITY)  # noqa: E731
    return R.frame(gradient(pixels, c["shape"], point("start"), point("end"), c["start_color"],
                            c["end_color"], opacity("start_opacity"), opacity("end_opacity"),
                            c["blend"]), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(start_opacity=0, end_opacity=0, shift=c["shift"]), 0)


FULL = {"start_opacity": 100, "end_opacity": 100, "blend": "normal"}

CASES = {
    "FX-GRAD-001": ("The settings as they start: linear from the top middle to the bottom "
                    "middle, white at 0 per cent to #6450a0 at 50, multiply. Row 1, the line "
                    "along the top, is barely touched, and each row down is darker and more "
                    "violet, the bottom most; "
                    "every pixel in a row is changed alike, and the empty pixels stay empty.",
                    case(), [0]),
    "FX-GRAD-002": ("Blend normal, both opacities 100: the cel is painted over with the "
                    "gradient itself, white fading down to #6450a0, the line and the shadow "
                    "gone under it, each pixel at its own covering.",
                    case(**FULL), [0]),
    "FX-GRAD-003": ("Blend screen, white to #6450a0, both opacities 100: every pixel that "
                    "shows is lightened, and none darkened.",
                    case(start_opacity=100, end_opacity=100, blend="screen"), [0]),
    "FX-GRAD-004": ("Blend add, #000000 to #6450a0, 0 to 50 per cent: the violet is added, "
                    "nothing at the top and most at the bottom; no channel passes the pixel's "
                    "covering.",
                    case(start_color="#000000", blend="add"), [0]),
    "FX-GRAD-005": ("Radial, from the middle, 50, 50, to the middle of the right edge, 100, 50, "
                    "blend normal at 100: the colour runs in rings about the middle, and "
                    "everything eight pixels or further from it is #6450a0.",
                    case(shape="radial", start=(50, 50), end=(100, 50), **FULL), [0]),
    "FX-GRAD-006": ("Start and end the same point, 50, 50: t is 1 everywhere, so, blend normal "
                    "at 100, every pixel that shows is #6450a0 at its own covering.",
                    case(start=(50, 50), end=(50, 50), **FULL), [0]),
    "FX-GRAD-007": ("Linear, left to right, 0, 50 to 100, 50, blend normal at 100: the colour "
                    "runs across the columns, every pixel in a column alike.",
                    case(start=(0, 50), end=(100, 50), **FULL), [0]),
    "FX-GRAD-008": ("Both points outside the drawing, 50, -100 and 50, 200, blend normal at "
                    "100: the drawing sees only the middle third of the gradient, t from "
                    "11.5/30 in row 1, the line along its top, to 18.5/30 in row 8.",
                    case(start=(50, -100), end=(50, 200), **FULL), [0]),
    "FX-GRAD-009": ("Both points past the bottom, 50, 150 and 50, 200, blend normal at 100: "
                    "every pixel is before the start, t is 0, and the cel is all white.",
                    case(start=(50, 150), end=(50, 200), **FULL), [0]),
    "FX-GRAD-010": ("The start point keyed from 50, 0 at frame 0 to 50, 100 at frame 4, the end "
                    "50, 100, blend normal at 100, linear: frame 0 is FX-GRAD-002; frame 2 "
                    "starts at the middle row, so rows 0 to 4 are white; frame 4 has start and "
                    "end the same, and is FX-GRAD-006.",
                    case(start=keyed((0, (50, 0)), (4, (50, 100))), **FULL), [0, 2, 4]),
    "FX-GRAD-011": ("Start opacity keyed from 0 at frame 0 to 100 at frame 4, end opacity from "
                    "50 to 0: frame 0 is FX-GRAD-001, frame 2 is 50 to 25, frame 4 is 100 to 0, "
                    "the top now most touched and the bottom row barely.",
                    case(start_opacity=keyed((0, 0), (4, 100)),
                         end_opacity=keyed((0, 50), (4, 0))), [0, 2, 4]),
    "FX-GRAD-012": ("Both opacities 0: the drawing, untouched.",
                    case(start_opacity=0, end_opacity=0), [0]),
    "FX-GRAD-013": ("White to white, blend normal at 100: every pixel that shows turns white at "
                    "its own covering, the half-covering edge white at half covering, and the "
                    "empty pixels stay empty.",
                    case(end_color="#ffffff", **FULL), [0]),
    "FX-GRAD-014": ("FX-GRAD-001 with its colours written in capitals: the same.",
                    case(start_color="#FFFFFF", end_color="#6450A0"), [0]),
    "FX-GRAD-015": ("FX-GRAD-001 moved three pixels right: the same, moved; the gradient moves "
                    "with the drawing.",
                    case(shift=3), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-GRAD-016": ("Start opacity 101, above 100.", case(start_opacity=101)),
    "FX-GRAD-017": ("End opacity -1, below 0.", case(end_opacity=-1)),
    "FX-GRAD-018": ("End opacity keyed to 150 at frame 4.",
                    case(end_opacity=keyed((0, 50), (4, 150)))),
    "FX-GRAD-019": ("Start point 1001, 0, past ten widths.", case(start=(1001, 0))),
    "FX-GRAD-020": ("Shape \"conic\", which is not a shape.", case(shape="conic")),
    "FX-GRAD-021": ("Blend \"overlay\", which is not a blend.", case(blend="overlay")),
    "FX-GRAD-022": ("A start colour written \"#12345\", one digit short.",
                    case(start_color="#12345")),
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
        "instance_id": "fx-0-0", "type_id": "core.gradient", "enabled": True,
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

    (OUT / "expected_gradient.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda u, v: all(abs(p - q) < 1e-12 for p, q in zip(u, v))  # noqa: E731
    shows = [i for i in range(W * H) if drawn[i][3] > 0]
    violet = [srgb_to_linear(v / 255) for v in (100, 80, 160)]
    white = [1.0, 1.0, 1.0]
    painted = lambda f, i, rgb: near(f[i], [v * drawn[i][3] for v in rgb] + [drawn[i][3]])  # noqa: E731,E501

    # The rule's own pieces.
    assert along("linear", 8.5, 0.5, (8, 0), (8, 10)) == 0.05
    assert along("linear", 3, 3, (1, 1), (1, 1)) == 1 == along("radial", 3, 3, (1, 1), (1, 1))
    assert along("radial", 12.5, 5.5, (8, 5), (16, 5)) == math.sqrt(4.5 ** 2 + 0.25) / 8
    assert along("linear", 0, -5, (0, 0), (0, 10)) == 0 and along("linear", 0, 50, (0, 0),
                                                                  (0, 10)) == 1
    # A pixel that does not show, and the covering, are kept by every case.
    for fx, frames in c.items():
        shift = 3 if fx == "FX-GRAD-015" else 0
        base = plain(case(shift=shift))
        for px in frames.values():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
                assert all(-1e-12 <= v <= px[i][3] + 1e-12 for v in px[i][:3]), (fx, i)

    one = c["FX-GRAD-001"]["0"]
    # Darker down the rows, alike along each row: the skin in column 5.
    reds = [one[at(5, y)][0] for y in range(2, 8)]
    assert all(u > v for u, v in zip(reds, reds[1:])) and reds[0] < drawn[at(5, 2)][0]
    for y in range(H):
        row = [one[at(x, y)][0] / drawn[at(x, y)][0] for x in range(3, 10)
               if drawn[at(x, y)][3] > 0]
        assert all(abs(v - row[0]) < 1e-12 for v in row)
    two = c["FX-GRAD-002"]["0"]
    for i in shows:
        t = (i // W + 0.5) / H
        g = [srgb_to_linear(u + t * (v - u)) for u, v in zip([1, 1, 1], [100 / 255, 80 / 255,
                                                                         160 / 255])]
        assert painted(two, i, g), i
    three = c["FX-GRAD-003"]["0"]
    assert all(three[i][0] >= drawn[i][0] and three[i][2] >= drawn[i][2] for i in shows)
    four = c["FX-GRAD-004"]["0"]
    assert all(four[i][2] > drawn[i][2] for i in shows)
    five = c["FX-GRAD-005"]["0"]
    for i in shows:
        if math.hypot(i % W + 0.5 - 8, i // W + 0.5 - 5) >= 8:
            assert painted(five, i, violet), i
    assert five[at(8, 5)][0] > five[at(5, 5)][0]
    six = c["FX-GRAD-006"]["0"]
    assert all(painted(six, i, violet) for i in shows)
    seven = c["FX-GRAD-007"]["0"]
    for x in range(W):
        col = [seven[at(x, y)][1] / seven[at(x, y)][3] for y in range(H) if drawn[at(x, y)][3]]
        assert all(abs(v - col[0]) < 1e-12 for v in col)
    eight = c["FX-GRAD-008"]["0"]
    assert painted(eight, at(5, 1), [srgb_to_linear(1 + 11.5 / 30 * (v / 255 - 1))
                                     for v in (100, 80, 160)])
    assert painted(eight, at(5, 8), [srgb_to_linear(1 + 18.5 / 30 * (v / 255 - 1))
                                     for v in (100, 80, 160)])
    assert eight[at(5, 1)] != two[at(5, 1)] and eight[at(5, 8)] != two[at(5, 8)]
    assert all(painted(c["FX-GRAD-009"]["0"], i, white) for i in shows)
    ten = c["FX-GRAD-010"]
    assert ten["0"] == two and ten["4"] == six
    assert all(painted(ten["2"], i, white) for i in shows if i // W <= 4)
    assert not painted(ten["2"], at(5, 5), white)
    eleven = c["FX-GRAD-011"]
    assert eleven["0"] == one
    assert near(eleven["2"][at(5, 4)], render(case(start_opacity=50, end_opacity=25), 0)[at(5, 4)])
    assert eleven["4"][at(5, 1)] != drawn[at(5, 1)]
    assert abs(eleven["4"][at(5, 8)][0] - drawn[at(5, 8)][0]) < abs(one[at(5, 8)][0]
                                                                   - drawn[at(5, 8)][0])
    assert c["FX-GRAD-012"]["0"] == drawn
    thirteen = c["FX-GRAD-013"]["0"]
    assert all(painted(thirteen, i, white) for i in shows)
    assert near(thirteen[at(1, 4)], [128 / 255] * 4)
    assert c["FX-GRAD-014"]["0"] == one
    moved = c["FX-GRAD-015"]["0"]
    assert moved == c["FX-GRAD-015"]["3"]
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == one[at(0, y):at(W - 3, y)]
    print("checked")


if __name__ == "__main__":
    main()
