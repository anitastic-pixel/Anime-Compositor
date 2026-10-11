"""CC Ball Action, worked a second way.

`core.cc_ball_action`, modelled on CycoreFX's CC Ball Action (Simulation), which transforms "the
source layer into an array of balls" that can be rotated and twisted "around a specified axis"
and scattered "in all directions". The CycoreFX HD manual names the controls: Scatter ("the
further the balls scatter away, in all directions, from their original positions"), Rotation
Axis (X, Y, Z, XY, XZ, YZ, XYZ, X15Z "15 times around the Z axis for every one rotation around
the X axis", XY15Z), Rotation, Twist Property (X Axis, Y Axis, Center-X, Center-Y, Radius "the
twisting effect diminishes toward the periphery", Random, Red, Green, Blue "the least intense
areas twist the most", Brightness "the darkest portion twist the most", Diamond, Rectangle,
Fast Top "around the X and Y axis, with faster twisting around the Y axis"), Twist Angle, Grid
Spacing ("a higher value creates less and larger cells"), Ball Size ("relative to the Grid
Spacing") and Instability State ("rotate scattered balls around their individual array
positions. Scatter must be non zero"). "Only completely opaque pixels in the source layer
transform into balls." It supports After Effects' comp camera; ours does not (P0-12): it draws
its own fake 3D with its own eye and light, as CC Cylinder (D-530) does. CycoreFX publishes no
formula, no lighting and no settings when added, so the rule and those settings are ours
(document 21, D-551).

The settings. `scatter`, 0 to 1000 pixels, 0 when added; `rotation_axis`, one of AXES, "x" when
added; `rotation`, -36000 to 36000 degrees, 0 when added; `twist_property`, one of TWISTS,
"x_axis" when added; `twist_angle`, -36000 to 36000 degrees, 0 when added; `grid_spacing`, 2 to
200 pixels, 8 when added; `ball_size`, 0 to 400 per cent of the grid spacing, 100 when added;
`instability_state`, -36000 to 36000 degrees, 0 when added. The six numbers are keyable; the two
words are exact. A draft scales Scatter and Grid Spacing (a spacing below 1 is worked as 1).

The rule, in the drawing's own W by H space, centre (cx, cy) = (W / 2, H / 2), x right, y down
and z away from the eye:

- The grid. With s = grid_spacing, cols = floor(W / s) and rows = floor(H / s) cells, centred:
  the cell (i, j) has its centre at (ox + (i + 1/2) s, oy + (j + 1/2) s), ox = (W - cols s) / 2,
  oy = (H - rows s) / 2. A cell holds a ball only when the drawing's pixel under its centre
  (floor of each) covers completely, alpha exactly 1; the ball is that pixel's colour. Its
  radius is r = s ball_size / 200.
- Each ball's numbers in -1..1, `unit(seed 551, i, j, frame 0, channel)` of tools/noise_reference.py
  (the build's `grade::unit`): channels 0 to 2 its scatter direction u, 3 to 5 its instability
  axis a, 6 its random twist.
- Scatter. With Instability State not 0, u is turned about a (normalised; left alone when a is
  0) by that angle, Rodrigues' way. The offset is scatter u.
- Twist. With xn = (x - cx) / (W / 2), yn = (y - cy) / (H / 2) from the cell centre (x, y) and
  rho = sqrt(xn^2 + yn^2) / sqrt(2), the factor f is: x_axis xn; y_axis yn; center_x |xn|;
  center_y |yn|; radius 1 - rho; random (u6 + 1) / 2; red, green, blue 1 - the ball's linear
  channel held in 0..1; brightness 1 - its linear luminance (0.2126, 0.7152, 0.0722) held in
  0..1; diamond 1 - (|xn| + |yn|) / 2; rectangle 1 - max(|xn|, |yn|); fast_top 1 - rho.
- Turning. With Rx, Ry, Rz the right-handed turns about each axis and theta = rotation +
  twist_angle f (degrees), the axis's turn is x Rx(theta), y Ry(theta), z Rz(theta), xy
  Ry Rx, xz Rz Rx, yz Rz Ry, xyz Rz Ry Rx, x15z Rz(15 theta) Rx(theta), xy15z Rz(15 theta)
  Ry(theta) Rx(theta). For fast_top, theta = rotation and the turn is the axis's times
  Ry(2 phi) Rx(phi), phi = twist_angle f. The ball's place is p = M (x - cx + scatter u_x,
  y - cy + scatter u_y, scatter u_z).
- The eye. As CC Cylinder's (D-530): a 50 mm lens on a 36 mm frame, Z = W 50 / 36 in front of
  the drawing's centre, looking along z. A ball at depth d = Z + p_z is drawn when d > Z / 100,
  centred at (cx + p_x Z / d, cy + p_y Z / d) with radius R = r Z / d.
- The order. The balls drawn are put farthest first (by d, larger first; equal depths by cell,
  row by row), and numbered 1 up in that order.
- Each pixel, at its centre (px, py): a ball covers it by c = clamp(R + 1/2 - e, 0, 1)
  min(2 R, 1), e its distance from the ball's centre; covered when c > 0. Of the balls covering
  it, the highest numbered is the first and the next highest the second; the pixel is
  c1 (S1, 1) + (1 - c1) c2 (S2, 1), premultiplied, or transparent when none covers it.
- The light, fixed: towards it L = (-1, -1, -2) / sqrt(6), from the upper left and in front;
  the half-way vector Hv = (L + (0, 0, -1)) normalised. On a ball, with (dx, dy) = ((px - X) / R,
  (py - Y) / R) and q = dx^2 + dy^2, the normal is (dx, dy, -sqrt(1 - q)) inside it and
  (dx, dy, 0) / sqrt(q) on the antialiased rim. S = C (0.25 + 0.75 max(0, n . L)) +
  0.5 max(0, n . Hv)^32, C the ball's colour, linear.

The layer never grows: balls past the layer's edge are cut off.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values; trigonometry is the platform's.

The drawing is Broadcast Colors' (tools/broadcast_safe_reference.py), 16 by 10, with a
transparent first column and a half-covered last one. The projects go into
`Fixtures/cc_ball_action`, the expected frames into
`Fixtures/cc_ball_action/expected_cc_ball_action.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/cc_ball_action_reference.py
"""

import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from noise_reference import u as U  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "cc_ball_action"
NUMBERS = ("scatter", "rotation", "twist_angle", "grid_spacing", "ball_size", "instability_state")
WORDS = ("rotation_axis", "twist_property")
AXES = ("x", "y", "z", "xy", "xz", "yz", "xyz", "x15z", "xy15z")
TWISTS = ("x_axis", "y_axis", "center_x", "center_y", "radius", "random", "red", "green", "blue",
          "brightness", "diamond", "rectangle", "fast_top")
RANGES = {"scatter": (0, 1000), "rotation": (-36000, 36000), "twist_angle": (-36000, 36000),
          "grid_spacing": (2, 200), "ball_size": (0, 400), "instability_state": (-36000, 36000)}
SEED = 551
L = [v / math.sqrt(6) for v in (-1.0, -1.0, -2.0)]
_h = [L[0], L[1], L[2] - 1.0]
HV = [v / math.sqrt(sum(c * c for c in _h)) for v in _h]


# --- the rule -------------------------------------------------------------------------------

def rx(a):
    c, s = math.cos(a), math.sin(a)
    return [[1, 0, 0], [0, c, -s], [0, s, c]]


def ry(a):
    c, s = math.cos(a), math.sin(a)
    return [[c, 0, s], [0, 1, 0], [-s, 0, c]]


def rz(a):
    c, s = math.cos(a), math.sin(a)
    return [[c, -s, 0], [s, c, 0], [0, 0, 1]]


def mul(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def apply(m, v):
    return [m[i][0] * v[0] + m[i][1] * v[1] + m[i][2] * v[2] for i in range(3)]


def turn(axis, theta):
    t = math.radians(theta)
    return {
        "x": lambda: rx(t), "y": lambda: ry(t), "z": lambda: rz(t),
        "xy": lambda: mul(ry(t), rx(t)), "xz": lambda: mul(rz(t), rx(t)),
        "yz": lambda: mul(rz(t), ry(t)), "xyz": lambda: mul(rz(t), mul(ry(t), rx(t))),
        "x15z": lambda: mul(rz(15 * t), rx(t)),
        "xy15z": lambda: mul(rz(15 * t), mul(ry(t), rx(t))),
    }[axis]()


def rodrigues(v, a, theta):
    n = math.sqrt(sum(c * c for c in a))
    if n == 0:
        return v
    k = [c / n for c in a]
    t = math.radians(theta)
    c, s = math.cos(t), math.sin(t)
    cr = [k[1] * v[2] - k[2] * v[1], k[2] * v[0] - k[0] * v[2], k[0] * v[1] - k[1] * v[0]]
    d = sum(k[i] * v[i] for i in range(3))
    return [v[i] * c + cr[i] * s + k[i] * d * (1 - c) for i in range(3)]


def factor(prop, xn, yn, col, rand):
    rho = math.sqrt(xn * xn + yn * yn) / math.sqrt(2)
    held = lambda v: min(1.0, max(0.0, v))  # noqa: E731
    return {
        "x_axis": xn, "y_axis": yn, "center_x": abs(xn), "center_y": abs(yn),
        "radius": 1 - rho, "random": (rand + 1) / 2,
        "red": 1 - held(col[0]), "green": 1 - held(col[1]), "blue": 1 - held(col[2]),
        "brightness": 1 - held(0.2126 * col[0] + 0.7152 * col[1] + 0.0722 * col[2]),
        "diamond": 1 - (abs(xn) + abs(yn)) / 2, "rectangle": 1 - max(abs(xn), abs(yn)),
        "fast_top": 1 - rho,
    }[prop]


def placed(px, k, w0=W, h0=H):
    """The balls drawn, farthest first, as (depth, cell, [X, Y, R, colour])."""
    s = max(k["grid_spacing"], 1.0)
    cols, rows = math.floor(w0 / s), math.floor(h0 / s)
    ox, oy = (w0 - cols * s) / 2, (h0 - rows * s) / 2
    cx, cy = w0 / 2, h0 / 2
    r = s * k["ball_size"] / 200
    z = w0 * 50 / 36
    out = []
    for j in range(rows):
        for i in range(cols):
            x, y = ox + (i + 0.5) * s, oy + (j + 0.5) * s
            p = px[math.floor(y) * w0 + math.floor(x)]
            if p[3] != 1.0:
                continue
            col = p[:3]
            un = [U(SEED, i, j, 0, ch) for ch in range(7)]
            u = un[0:3]
            if k["instability_state"] != 0:
                u = rodrigues(u, un[3:6], k["instability_state"])
            xn, yn = (x - cx) / (w0 / 2), (y - cy) / (h0 / 2)
            f = factor(k["twist_property"], xn, yn, col, un[6])
            if k["twist_property"] == "fast_top":
                phi = math.radians(k["twist_angle"] * f)
                m = mul(turn(k["rotation_axis"], k["rotation"]), mul(ry(2 * phi), rx(phi)))
            else:
                m = turn(k["rotation_axis"], k["rotation"] + k["twist_angle"] * f)
            sc = k["scatter"]
            q = apply(m, [x - cx + sc * u[0], y - cy + sc * u[1], sc * u[2]])
            d = z + q[2]
            if d <= z / 100:
                continue
            out.append((d, j * cols + i, [cx + q[0] * z / d, cy + q[1] * z / d, r * z / d, col]))
    out.sort(key=lambda b: (-b[0], b[1]))
    return out


def balls(px, k):
    """The balls drawn, farthest first, as [X, Y, R, colour]."""
    return [b[2] for b in placed(px, k)]


def cover(b, x, y):
    e = math.sqrt((x - b[0]) ** 2 + (y - b[1]) ** 2)
    return min(1.0, max(0.0, b[2] + 0.5 - e)) * min(2 * b[2], 1.0)


def shade(b, x, y):
    dx, dy = (x - b[0]) / b[2], (y - b[1]) / b[2]
    q = dx * dx + dy * dy
    if q >= 1:
        n = [dx / math.sqrt(q), dy / math.sqrt(q), 0.0]
    else:
        n = [dx, dy, -math.sqrt(1 - q)]
    dif = max(0.0, sum(n[i] * L[i] for i in range(3)))
    sp = max(0.0, sum(n[i] * HV[i] for i in range(3))) ** 32
    return [c * (0.25 + 0.75 * dif) + 0.5 * sp for c in b[3]]


def ball_action(px, k):
    bs = balls(px, k)
    out = []
    for n in range(W * H):
        x, y = n % W + 0.5, n // W + 0.5
        hit = [(m, c) for m, c in ((m, cover(b, x, y)) for m, b in enumerate(bs)) if c > 0]
        if not hit:
            out.append([0.0] * 4)
            continue
        m1, c1 = hit[-1]
        s1 = shade(bs[m1], x, y)
        p = [c1 * v for v in s1] + [c1]
        if len(hit) > 1:
            m2, c2 = hit[-2]
            s2 = shade(bs[m2], x, y)
            p = [p[i] + (1 - c1) * c2 * s2[i] for i in range(3)] + [c1 + (1 - c1) * c2]
        out.append(p)
    return out


def working():
    return [R.working(p) for p in B.pixels()]


# --- the cases ------------------------------------------------------------------------------

def case(rotation_axis="x", twist_property="x_axis", shift=0, **numbers):
    c = {"rotation_axis": rotation_axis, "twist_property": twist_property, "shift": shift,
         "scatter": 0, "rotation": 0, "twist_angle": 0, "grid_spacing": 8, "ball_size": 100,
         "instability_state": 0}
    for n, v in numbers.items():
        assert n in NUMBERS, n
        c[n] = v
    return c


def settings(c, frame_no):
    k = {n: value_at(c[n], frame_no) for n in NUMBERS}
    k.update(rotation_axis=c["rotation_axis"], twist_property=c["twist_property"])
    return k


def render(c, frame_no):
    return R.frame(ball_action(working(), settings(c, frame_no)), c["shift"])


def plain(c):
    return R.frame(working(), c["shift"])


def file_json(fx, c):
    params = {w: c[w] for w in WORDS}
    params.update({n: setting_json(c[n]) for n in NUMBERS})
    return B.project_json(fx, "core.cc_ball_action", params, c["shift"])


def tw(prop, **n):
    return case("y", prop, grid_spacing=4, twist_angle=80, **n)


CASES = {
    "FX-BALL-001": ("The settings as they start: grid spacing 8, so two cells across and one "
                    "down, each a ball 8 pixels wide of the colour under its centre (yellow on "
                    "the left, the warm shadow on the right), lit from the upper left, with "
                    "transparent round them.", case(), [0]),
    "FX-BALL-002": ("Grid spacing 4: four balls across and two down, touching.",
                    case(grid_spacing=4), [0]),
    "FX-BALL-003": ("Grid spacing 3: five across and three down, half a pixel in from each edge.",
                    case(grid_spacing=3), [0]),
    "FX-BALL-004": ("Grid spacing 5: three across and two down; the left cell's centre falls on "
                    "a covered column.", case(grid_spacing=5), [0]),
    "FX-BALL-005": ("Grid spacing 2, the least: a ball on every other column and row, seven "
                    "across and five down, the half-covered last column making none.",
                    case(grid_spacing=2), [0]),
    "FX-BALL-006": ("Ball size 50: the balls half as wide as their cells, with gaps between.",
                    case(grid_spacing=4, ball_size=50), [0]),
    "FX-BALL-007": ("Ball size 160: the balls overlap, each later one in its row and each lower "
                    "row in front at the same depth.", case(grid_spacing=4, ball_size=160), [0]),
    "FX-BALL-008": ("Ball size 0: no balls, the layer transparent.",
                    case(grid_spacing=4, ball_size=0), [0]),
    "FX-BALL-009": ("Rotation 40 about X: the grid tips back at the top, its upper row farther "
                    "and smaller, its lower row nearer and larger.",
                    case(grid_spacing=4, rotation=40), [0]),
    "FX-BALL-010": ("Rotation 60 about Y: the grid swings, its left side nearer and larger, its "
                    "right farther, the columns closing up.",
                    case("y", grid_spacing=4, rotation=60), [0]),
    "FX-BALL-011": ("Rotation 30 about Z: the grid turns flat in the picture.",
                    case("z", grid_spacing=4, rotation=30), [0]),
    "FX-BALL-012": ("Rotation 35 about XY.", case("xy", grid_spacing=4, rotation=35), [0]),
    "FX-BALL-013": ("Rotation 35 about XZ.", case("xz", grid_spacing=4, rotation=35), [0]),
    "FX-BALL-014": ("Rotation 35 about YZ.", case("yz", grid_spacing=4, rotation=35), [0]),
    "FX-BALL-015": ("Rotation 25 about XYZ.", case("xyz", grid_spacing=4, rotation=25), [0]),
    "FX-BALL-016": ("Rotation 8 about X15Z: 8 degrees about X and 120 about Z.",
                    case("x15z", grid_spacing=4, rotation=8), [0]),
    "FX-BALL-017": ("Rotation 6 about XY15Z: 6 degrees about X and Y and 90 about Z.",
                    case("xy15z", grid_spacing=4, rotation=6), [0]),
    "FX-BALL-018": ("Rotation 90 about Y: the grid edge on, every ball in one column, the "
                    "nearest in front.", case("y", grid_spacing=4, rotation=90), [0]),
    "FX-BALL-019": ("Twist X Axis, 80 degrees about Y: each column turned by its place across, "
                    "the left ones one way and the right ones the other.",
                    tw("x_axis"), [0]),
    "FX-BALL-020": ("Twist Y Axis, 80 degrees about X: each row turned by its place down.",
                    case("x", "y_axis", grid_spacing=4, twist_angle=80), [0]),
    "FX-BALL-021": ("Twist Center-X: the columns farther from the middle turned more.",
                    tw("center_x"), [0]),
    "FX-BALL-022": ("Twist Center-Y, about X: the rows farther from the middle turned more.",
                    case("x", "center_y", grid_spacing=4, twist_angle=80), [0]),
    "FX-BALL-023": ("Twist Radius: the balls near the middle turned most.", tw("radius"), [0]),
    "FX-BALL-024": ("Twist Random: each ball turned by its own fixed amount.", tw("random"), [0]),
    "FX-BALL-025": ("Twist Red: the balls with least red turned most.", tw("red"), [0]),
    "FX-BALL-026": ("Twist Green.", tw("green"), [0]),
    "FX-BALL-027": ("Twist Blue.", tw("blue"), [0]),
    "FX-BALL-028": ("Twist Brightness: the darkest balls turned most.", tw("brightness"), [0]),
    "FX-BALL-029": ("Twist Diamond.", tw("diamond"), [0]),
    "FX-BALL-030": ("Twist Rectangle.", tw("rectangle"), [0]),
    "FX-BALL-031": ("Twist Fast Top: each ball's place turned about X and twice as fast about "
                    "Y, most near the middle.", tw("fast_top"), [0]),
    "FX-BALL-032": ("Scatter 3: each ball moved up to 3 pixels its own way, nearer ones larger.",
                    case(grid_spacing=4, scatter=3), [0]),
    "FX-BALL-033": ("Scatter 3 with Instability State 120: each ball's scatter turned 120 "
                    "degrees about its own axis.",
                    case(grid_spacing=4, scatter=3, instability_state=120), [0]),
    "FX-BALL-034": ("Instability State 120 with Scatter 0: nothing scattered, so the same as "
                    "FX-BALL-002.", case(grid_spacing=4, instability_state=120), [0]),
    "FX-BALL-035": ("Rotation keyed from 0 at frame 0 to 80 at frame 4, linear, about X.",
                    case(grid_spacing=4, rotation=keyed((0, 0), (4, 80))), [0, 2, 4]),
    "FX-BALL-036": ("FX-BALL-002 moved three pixels right: the same, moved.",
                    case(grid_spacing=4, shift=3), [0, 3]),
    "FX-BALL-037": ("Rotation 360 about X: once round, the same as FX-BALL-002.",
                    case(grid_spacing=4, rotation=360), [0]),
}

INVALID = {
    "FX-BALL-038": ("Scatter -1, below 0.", case(scatter=-1)),
    "FX-BALL-039": ("Scatter 1001, above 1000.", case(scatter=1001)),
    "FX-BALL-040": ("Grid spacing 1.5, below 2.", case(grid_spacing=1.5)),
    "FX-BALL-041": ("Ball size 401, above 400.", case(ball_size=401)),
    "FX-BALL-042": ("Rotation 36001, above 36000.", case(rotation=36001)),
    "FX-BALL-043": ("Rotation axis \"w\": not one of the nine.", case("w")),
    "FX-BALL-044": ("Rotation axis \"X\": the word is exact, so a capital is not it.", case("X")),
    "FX-BALL-045": ("Twist property \"spiral\": not one of the thirteen.", case("x", "spiral")),
}


def main():
    step_clear()
    expected = B.write_cases(OUT, "cc_ball_action", CASES, INVALID, render, plain, file_json)
    check(expected)


def step_clear():
    """No ball near the eye's cut-off, and no two balls that both cover a pixel at nearly the
    same depth unless exactly the same (then the cell order settles it on both sides)."""
    px = working()
    for fx, (_, c, frames) in CASES.items():
        for f in frames:
            k = settings(c, f)
            z = W * 50 / 36
            pl = placed(px, k)
            for d, _, _ in pl:
                assert abs(d - z / 100) > 1e-3, fx
            for a in range(len(pl)):
                for b in range(a + 1, len(pl)):
                    (da, _, p), (db, _, q) = pl[a], pl[b]
                    near = math.hypot(p[0] - q[0], p[1] - q[1]) < p[2] + q[2] + 1
                    assert not (near and 0 < abs(da - db) < 1e-6), (fx, a, b, da - db)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    px = working()
    near = lambda p, q, e=1e-12: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    same = lambda a, b, e=1e-9: all(near(p, q, e) for p, q in zip(a, b))  # noqa: E731
    one = c["FX-BALL-001"]["0"]
    assert len(balls(px, settings(case(), 0))) == 2
    assert one[0] == [0.0] * 4 and one[5 * W + 4][3] == 1.0
    assert len(balls(px, settings(case(grid_spacing=4), 0))) == 8
    assert len(balls(px, settings(case(grid_spacing=3), 0))) == 15
    assert len(balls(px, settings(case(grid_spacing=2), 0))) == 7 * 5
    assert len(balls(px, settings(case(grid_spacing=5), 0))) == 6
    two = c["FX-BALL-002"]["0"]
    # a ball's middle is its colour lit: brighter on its upper left than its lower right
    b = balls(px, settings(case(grid_spacing=4), 0))[0]
    assert sum(shade(b, b[0] - 1, b[1] - 1)) > sum(shade(b, b[0] + 1, b[1] + 1))
    assert all(p == [0.0] * 4 for p in c["FX-BALL-008"]["0"])
    assert sum(p[3] for p in c["FX-BALL-006"]["0"]) < sum(p[3] for p in two)
    assert sum(p[3] for p in c["FX-BALL-007"]["0"]) > sum(p[3] for p in two)
    nine = balls(px, settings(case(grid_spacing=4, rotation=40), 0))
    assert max(b[2] for b in nine) > 2 > min(b[2] for b in nine)
    for fx in ("FX-BALL-%03d" % n for n in range(9, 34)):
        assert not same(c[fx]["0"], two), fx
    assert same(c["FX-BALL-034"]["0"], two)
    assert same(c["FX-BALL-037"]["0"], two, 1e-6)
    assert c["FX-BALL-035"]["0"] == two
    assert c["FX-BALL-035"]["2"] == render(case(grid_spacing=4, rotation=40), 0)
    moved = c["FX-BALL-036"]["0"]
    for y in range(H):
        assert moved[y * W + 3:(y + 1) * W] == two[y * W:y * W + W - 3]
    twists = [c["FX-BALL-%03d" % n]["0"] for n in range(19, 32)]
    for a in range(len(twists)):
        for b in range(a + 1, len(twists)):
            assert twists[a] != twists[b], (a, b)
    print("checked")


if __name__ == "__main__":
    main()
