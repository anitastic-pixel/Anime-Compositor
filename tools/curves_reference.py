"""Curves, worked a second way.

D-111 adds `core.curves`. It regrades a drawing's colours through four curves, one for all
three channels (`master`) and one each for red, green and blue, as a colourist bends a tone
curve. Each curve is 2 to 16 points `[in, out]` on the 0 to 255 scale of a colour as a drawing
program holds it; between its points it is the natural cubic spline through them, a straight
line for two points, and outside its end points it stays flat at the end point's value. A pixel
that shows is taken to its straight colour through the sRGB curve; each channel goes first
through its own curve, then through the master, the result held inside 0 to 255; the colour
comes back to linear at the pixel's own covering. A pixel that does not show is left as it is.
With all four curves the default, the straight line from `[0, 0]` to `[255, 255]`, it changes
nothing. It is this program's own method, modelled on After Effects' Curves; nothing is
ported. Document 21 is the rule in words; this file is the reference for the numbers document
25 pins against it.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: a colour chart, drawn below, which Levels' fixtures share. The drawing goes into
`Fixtures/curves/media`, the projects into `Fixtures/curves`, and the expected frames into
`Fixtures/curves/expected_curves.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/curves_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "curves"
TOLERANCE = 2e-5  # document 25's default for a filter
DEFAULT = [[0, 0], [255, 255]]
NAMES = ("master", "red", "green", "blue")


# --- the rule -------------------------------------------------------------------------------

def spline(points):
    """The curve through the points as a function of x: flat outside its end points, the
    natural cubic spline between them (second derivative 0 at both ends)."""
    xs = [float(p[0]) for p in points]
    ys = [float(p[1]) for p in points]
    n = len(xs)
    h = [xs[i + 1] - xs[i] for i in range(n - 1)]
    m = [0.0] * n
    if n > 2:
        # The tridiagonal system for M_1..M_{n-2}, solved by elimination down and back up.
        sub = [h[i - 1] for i in range(1, n - 1)]
        dia = [2 * (h[i - 1] + h[i]) for i in range(1, n - 1)]
        sup = [h[i] for i in range(1, n - 1)]
        rhs = [6 * ((ys[i + 1] - ys[i]) / h[i] - (ys[i] - ys[i - 1]) / h[i - 1])
               for i in range(1, n - 1)]
        for k in range(1, n - 2):
            f = sub[k] / dia[k - 1]
            dia[k] -= f * sup[k - 1]
            rhs[k] -= f * rhs[k - 1]
        sol = [0.0] * (n - 2)
        for k in reversed(range(n - 2)):
            sol[k] = (rhs[k] - (sup[k] * sol[k + 1] if k + 1 < n - 2 else 0.0)) / dia[k]
        m[1:n - 1] = sol

    def at(x):
        if x <= xs[0]:
            return ys[0]
        if x >= xs[-1]:
            return ys[-1]
        i = max(k for k in range(n - 1) if xs[k] <= x)
        t, hi = x - xs[i], h[i]
        return (ys[i] + t * ((ys[i + 1] - ys[i]) / hi - hi * (2 * m[i] + m[i + 1]) / 6)
                + t * t * m[i] / 2 + t ** 3 * (m[i + 1] - m[i]) / (6 * hi))
    return at


def clamp(v, lo, hi):
    return min(hi, max(lo, v))


def grade(pixels, fns):
    """The shared colour rule of the batch: a pixel that shows is taken to its encoded straight
    colour, each channel's 0..255 value x goes through fns[c](x), the result held inside 0..255
    comes back to linear at the pixel's own covering. A pixel with no covering is left as it
    is. `pixels` are 8-bit straight RGBA; the result is document 21's working values."""
    out = []
    for p in pixels:
        w = R.working(p)
        if p[3] > 0:
            a = w[3]
            e = [S.linear_to_srgb(clamp(w[i] / a, 0, 1)) for i in range(3)]
            w = [srgb_to_linear(clamp(fns[i](e[i] * 255) / 255, 0, 1)) * a
                 for i in range(3)] + [a]
        out.append(w)
    return out


def curves(pixels, master, red, green, blue):
    if all(c == DEFAULT for c in (master, red, green, blue)):
        return [R.working(p) for p in pixels]  # the build exits early; the identity agrees
    m = spline(master)
    return grade(pixels, [lambda x, c=spline(c): clamp(m(clamp(c(x), 0, 255)), 0, 255)
                          for c in (red, green, blue)])


# --- the drawing ----------------------------------------------------------------------------

LINE = R.LINE                          # #1e1a24
SOFT_LINE = LINE[:3] + (128,)          # the line at half covering, its antialiased end
SKIN = R.SKIN                          # #f6d6be
SOFT_SKIN = SKIN[:3] + (128,)
SHADE = (220, 160, 140, 255)           # the skin's shadow, #dca08c
RED = (200, 40, 40, 255)               # #c82828
BLUE = (60, 90, 200, 255)              # #3c5ac8
GREEN = (40, 170, 90, 255)             # #28aa5a
WHITE, BLACK = (255, 255, 255, 255), (0, 0, 0, 255)
NONE = S.NONE
RAMP = [0, 16, 32, 48, 64, 80, 96, 112, 128, 160, 192, 224, 240, 255]  # row 3, columns 1 to 14
COLORS = [RED, BLUE, GREEN, SKIN, SHADE, WHITE, BLACK]                # row 4, columns 1 to 7
SOFTS = [32, 64, 96, 128, 160, 192, 224]                              # row 7's coverings


def chart(x, y):
    """Column 0 and rows 0 and 9 are empty. Rows 1 and 6 are line, ending in a half-covering
    line pixel at column 15. Rows 2 and 5 are skin (columns 2 to 7) and its shadow (8 to 13)
    between line pixels, ending in half-covering skin. Row 3 is a grey ramp from 0 to 255 in
    columns 1 to 14. Row 4 is red, blue, green, skin, shadow, white and black in columns 1 to 7
    and the same at half covering in 8 to 14. Row 7 is skin at coverings 32 to 224 in columns 1
    to 7 and red at the same coverings in 8 to 14."""
    if x == 0 or y in (0, 8, 9):
        return NONE
    if y in (1, 6):
        return SOFT_LINE if x == 15 else LINE
    if y in (2, 5):
        if x == 15:
            return SOFT_SKIN
        return LINE if x in (1, 14) else SKIN if x <= 7 else SHADE
    if x == 15:
        return NONE
    if y == 3:
        v = RAMP[x - 1]
        return (v, v, v, 255)
    if y == 4:
        c = COLORS[(x - 1) % 7]
        return c if x <= 7 else c[:3] + (128,)
    return (SKIN if x <= 7 else RED)[:3] + (SOFTS[(x - 1) % 7],)


DRAWINGS = {"chart": [[chart(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(master=DEFAULT, red=DEFAULT, green=DEFAULT, blue=DEFAULT, shift=0):
    return {"drawing": "chart", "master": master, "red": red, "green": green, "blue": blue,
            "shift": shift}


def render(c, frame_no=0):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    return R.frame(curves(pixels, *(c[k] for k in NAMES)), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(shift=c["shift"]), 0)


LIFT = [[0, 0], [128, 180], [255, 255]]
S_RED = [[0, 0], [64, 40], [192, 215], [255, 255]]
ENDS = [[64, 32], [192, 224]]
HUMP = [[0, 0], [64, 255], [128, 255], [255, 0]]
INVERT = [[0, 255], [255, 0]]
HALF = [[0, 0], [255, 128]]
DOUBLE = [[0, 0], [128, 255]]
ZIGZAG = [[17 * i, 0 if i % 2 == 0 else 255] for i in range(16)]  # 0, 17, ..., 255
STRAIGHT = [[0, 0], [128, 128], [255, 255]]

CASES = {
    "FX-CURVES-001": ("All four curves the default, [[0, 0], [255, 255]]: the drawing, "
                      "untouched.",
                      case(), [0]),
    "FX-CURVES-002": ("Master through [[0, 0], [128, 180], [255, 255]]: every colour is "
                      "lightened, the middle greys most; black and white stay, and so do the "
                      "empty pixels.",
                      case(master=LIFT), [0]),
    "FX-CURVES-003": ("Red alone through an S, [[0, 0], [64, 40], [192, 215], [255, 255]]: "
                      "only the red channel moves, darker below the middle and lighter above; "
                      "green and blue stay exactly as drawn.",
                      case(red=S_RED), [0]),
    "FX-CURVES-004": ("Master through [[64, 32], [192, 224]]: flat outside its end points, so "
                      "the ramp's greys at or below 64 all become 32 and those at or above 192 "
                      "all become 224; between, the straight line.",
                      case(master=ENDS), [0]),
    "FX-CURVES-005": ("Master through [[0, 0], [64, 255], [128, 255], [255, 0]]: the spline "
                      "rises above 255 between 64 and 128 and is held at 255 there.",
                      case(master=HUMP), [0]),
    "FX-CURVES-006": ("Master through 16 points, the most allowed, alternating 0 and 255 every "
                      "17: the ramp's greys land wherever the zigzag puts them; its 16 swings "
                      "past 255 and its 240 below 0, each held.",
                      case(master=ZIGZAG), [0]),
    "FX-CURVES-007": ("Red through [[0, 0], [255, 128]], halving it, and master through "
                      "[[0, 0], [128, 255]], doubling: the channel's own curve runs first, so "
                      "red comes back as drawn while green and blue double, held at 255.",
                      case(master=DOUBLE, red=HALF), [0]),
    "FX-CURVES-008": ("Master through [[0, 255], [255, 0]], inverting: every colour that shows "
                      "is inverted at its own covering, the soft pixels still soft; the empty "
                      "pixels stay empty, not white.",
                      case(master=INVERT), [0]),
    "FX-CURVES-009": ("Green alone through [[0, 0], [255, 128]]: only green is halved.",
                      case(green=HALF), [0]),
    "FX-CURVES-010": ("Blue alone through [[0, 64], [255, 255]]: only blue is lifted, black's "
                      "blue to 64.",
                      case(blue=[[0, 64], [255, 255]]), [0]),
    "FX-CURVES-011": ("All four at once: master FX-CURVES-002's lift, red the S, green halved, "
                      "blue inverted.",
                      case(master=LIFT, red=S_RED, green=HALF, blue=INVERT), [0]),
    "FX-CURVES-012": ("Master through three points on the straight line, [[0, 0], [128, 128], "
                      "[255, 255]]: not the default as written, so it is worked, but the spline "
                      "through them is the line, so the drawing.",
                      case(master=STRAIGHT), [0]),
    "FX-CURVES-013": ("Master through [[0, 0], [127.5, 200.25], [255, 255]], points written "
                      "with fractions: a lift a little stronger than FX-CURVES-002's.",
                      case(master=[[0, 0], [127.5, 200.25], [255, 255]]), [0]),
    "FX-CURVES-014": ("FX-CURVES-011 moved three pixels right: the same, moved.",
                      case(master=LIFT, red=S_RED, green=HALF, blue=INVERT, shift=3), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-CURVES-015": ("Master of one point, [[128, 128]], fewer than two.",
                      case(master=[[128, 128]])),
    "FX-CURVES-016": ("Red of 17 points, more than sixteen.",
                      case(red=[[15 * i, 15 * i] for i in range(17)])),
    "FX-CURVES-017": ("Green with an out of 256, above 255.",
                      case(green=[[0, 0], [255, 256]])),
    "FX-CURVES-018": ("Blue with an in of -1, below 0.",
                      case(blue=[[-1, 0], [255, 255]])),
    "FX-CURVES-019": ("Master with two points at in 128, so in does not strictly increase.",
                      case(master=[[0, 0], [128, 100], [128, 160], [255, 255]])),
    "FX-CURVES-020": ("Master with a point of three numbers, [128, 128, 128].",
                      case(master=[[0, 0], [128, 128, 128], [255, 255]])),
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
        "instance_id": "fx-0-0", "type_id": "core.curves", "enabled": True,
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

    (OUT / "expected_curves.json").write_text(json.dumps(expected, indent=1) + "\n",
                                              encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    art = DRAWINGS["chart"]
    at = lambda x, y: y * W + x  # noqa: E731
    close = lambda p, q: all(abs(p[i] - q[i]) < 1e-12 for i in range(4))  # noqa: E731

    def enc(f, x, y):
        """The frame's pixel as 0..255 straight encoded colour, to compare with the curves."""
        p = f[at(x, y)]
        return [S.linear_to_srgb(p[i] / p[3]) * 255 for i in range(3)]

    def want(f, x, y, fns):
        """The pixel is the drawing's own with each channel's 8-bit value through fns[c], at
        its own covering."""
        p, q = art[y][x], f[at(x, y)]
        assert abs(q[3] - p[3] / 255) < 1e-15, (x, y)
        got = enc(f, x, y)
        assert all(abs(got[i] - fns[i](p[i])) < 1e-9 for i in range(3)), (x, y, got)

    showing = [(x, y) for y in range(H) for x in range(W) if art[y][x][3] > 0]
    empty = [(x, y) for y in range(H) for x in range(W) if art[y][x][3] == 0]
    ramp = lambda f: [enc(f, x, 3)[0] for x in range(1, 15)]  # noqa: E731

    # The rule's own pieces.
    for pts in (DEFAULT, STRAIGHT):
        f = spline(pts)
        assert all(abs(f(x) - x) < 1e-12 for x in range(256))
    f = spline(S_RED)
    assert all(abs(f(px) - py) < 1e-12 for px, py in S_RED)
    assert spline(ENDS)(10) == 32 and spline(ENDS)(250) == 224 and spline(ENDS)(128) == 128
    hump = spline(HUMP)
    assert max(hump(x) for x in range(64, 129)) > 280  # the spline overshoots, well past 255
    zig = spline(ZIGZAG)
    assert all(zig(17 * i) == (0 if i % 2 == 0 else 255) for i in range(16))
    # A natural spline's second derivative is 0 at its ends: the first segment is straight
    # at x_0, so the slope on each side of a small step agrees.
    lift = spline(LIFT)
    d1, d2 = lift(1) - lift(0), lift(2) - lift(1)
    assert abs(d1 - d2) < 1e-4

    assert c["FX-CURVES-001"] == drawn
    for fx in ("FX-CURVES-%03d" % i for i in range(2, 14)):
        for x, y in empty:
            assert c[fx][at(x, y)] == [0.0] * 4, (fx, x, y)
    two = c["FX-CURVES-002"]
    ident = lambda v: v  # noqa: E731
    lift_fn = lambda v: spline(LIFT)(v)  # noqa: E731
    for x, y in showing:
        want(two, x, y, [lift_fn] * 3)
    assert abs(ramp(two)[8] - 180) < 1e-9 and abs(ramp(two)[0]) < 1e-9
    assert abs(ramp(two)[13] - 255) < 1e-9
    assert all(r > v for r, v in zip(ramp(two)[1:13], RAMP[1:13]))
    three = c["FX-CURVES-003"]
    for x, y in showing:
        want(three, x, y, [spline(S_RED), ident, ident])
    assert enc(three, 5, 3)[0] < 64 < 192 < enc(three, 11, 3)[0]
    four = ramp(c["FX-CURVES-004"])
    assert all(abs(r - (32 if v <= 64 else 224 if v >= 192 else 32 + (v - 64) * 1.5)) < 1e-9
               for r, v in zip(four, RAMP))
    five = c["FX-CURVES-005"]
    for x, y in showing:
        want(five, x, y, [lambda v: clamp(hump(v), 0, 255)] * 3)
    assert all(abs(r - 255) < 1e-9 for r, v in zip(ramp(five), RAMP) if 64 <= v <= 128)
    six = c["FX-CURVES-006"]
    for x, y in showing:
        want(six, x, y, [lambda v: clamp(zig(v), 0, 255)] * 3)
    assert zig(16) > 255 and zig(240) < 0
    assert abs(enc(six, 2, 3)[0] - 255) < 1e-9 and abs(enc(six, 13, 3)[0]) < 1e-9
    seven = c["FX-CURVES-007"]
    for x, y in showing:
        want(seven, x, y, [ident, lambda v: min(255, v * 255 / 128),
                           lambda v: min(255, v * 255 / 128)])
    eight = c["FX-CURVES-008"]
    for x, y in showing:
        want(eight, x, y, [lambda v: 255 - v] * 3)
    assert abs(eight[at(1, 7)][3] - 32 / 255) < 1e-15  # the softest skin keeps its covering
    for fx, fns in (("FX-CURVES-009", [ident, lambda v: v * 128 / 255, ident]),
                    ("FX-CURVES-010", [ident, ident, lambda v: 64 + v * 191 / 255])):
        for x, y in showing:
            want(c[fx], x, y, fns)
    eleven = c["FX-CURVES-011"]
    for x, y in showing:
        want(eleven, x, y, [lambda v: lift_fn(spline(S_RED)(v)), lambda v: lift_fn(v * 128 / 255),
                            lambda v: lift_fn(255 - v)])
    assert all(close(p, q) for p, q in zip(c["FX-CURVES-012"], drawn))
    assert all(r >= q - 1e-9 for r, q in zip(ramp(c["FX-CURVES-013"]), ramp(two)))
    assert ramp(c["FX-CURVES-013"])[8] > ramp(two)[8]
    moved = expected["cases"]["FX-CURVES-014"]["frames"]
    assert moved["0"] == moved["3"]
    for y in range(H):
        assert moved["0"][at(3, y):at(0, y + 1)] == eleven[at(0, y):at(W - 3, y)]
    print("checked")


if __name__ == "__main__":
    main()
