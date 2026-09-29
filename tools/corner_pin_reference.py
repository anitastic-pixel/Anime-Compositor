"""Corner pin, worked a second way.

D-198 adds `core.corner_pin`, After Effects' Corner Pin by a rule of our own: the drawing's four
corners are pinned to four new places and the drawing is stretched between them in perspective,
the way a poster, a screen or a sign is put onto a wall that leans away. Each corner is a point in
per cent of the drawing's own width and height; at (0, 0), (100, 0), (0, 100) and (100, 100) the
drawing is as it was. The layer grows so that a corner pulled outside it is not cut off. It is
modelled on After Effects' Corner Pin and not claimed to match it; nothing is ported. Document 21
is the rule in words; this file is the reference for the numbers document 25 pins against it.

The rule. W and H are the drawing's own size and o its place in the input buffer, which an
earlier effect may have grown. The corners in the drawing's own space are A = upper left, B =
upper right, C = lower right and D = lower left, each (p_x / 100 W, p_y / 100 H). With all four at
their starting places the output is the input and nothing grows. Otherwise, unless the cross
products z_i = (P_i - P_i-1) x (P_i+1 - P_i) round the ring A, B, C, D are all above 0 or all
below 0, the four corners make no drawable shape (crossed, bent in, or three in a line) and the
output is the input's size and clear. Otherwise the drawing's square u, v in 0..1 is carried onto
the four corners by Heckbert's square-to-quad map M = [[a, b, c], [d, e, f], [g, h, 1]] (worked in
`square_to_quad` below), (x, y) = ((a u + b v + c) / (g u + h v + 1), (d u + e v + f) / (g u + h
v + 1)). The layer grows by gx on the left and on the right, gx = max(0, ceil(the most any
corner's x lies past the input buffer's left or right edge)), and gy likewise down. The output at
a pixel with centre P, in the drawing's own space, is worked from (U, V, T) = adj(M) (P_x, P_y, 1):
clear unless T det(M) > 0, which keeps only the side of the horizon the drawing lies on, and
otherwise document 21's bilinear sample of the whole input, grown pixels and all, at (U / T W,
V / T H), transparent outside it.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: Wave Warp's stripes, imported from `tools/wave_warp_reference.py`. The drawing
goes into `Fixtures/corner_pin/media`, the projects into `Fixtures/corner_pin`, and the expected
frames into `Fixtures/corner_pin/expected_corner_pin.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/corner_pin_reference.py
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
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
from wave_warp_reference import DRAWINGS  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "corner_pin"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGE = (-400, 500)
NAMES = ("upper_left", "upper_right", "lower_left", "lower_right")
START = {"upper_left": (0, 0), "upper_right": (100, 0), "lower_left": (0, 100),
         "lower_right": (100, 100)}
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def points(pins, dw, dh):
    """A, B, C, D: upper left, upper right, lower right, lower left, in the drawing's space."""
    ul, ur, ll, lr = (pins[k] for k in NAMES)
    return [(p[0] / 100 * dw, p[1] / 100 * dh) for p in (ul, ur, lr, ll)]


def convex(ring):
    z = []
    for i in range(4):
        (px, py), (qx, qy), (rx, ry) = ring[i - 1], ring[i], ring[(i + 1) % 4]
        z.append((qx - px) * (ry - qy) - (qy - py) * (rx - qx))
    return all(v > 0 for v in z) or all(v < 0 for v in z)


def square_to_quad(A, B, C, D):
    """Heckbert's map of the unit square onto A (0, 0), B (1, 0), C (1, 1), D (0, 1)."""
    sx, sy = A[0] - B[0] + C[0] - D[0], A[1] - B[1] + C[1] - D[1]
    dx1, dx2, dy1, dy2 = B[0] - C[0], D[0] - C[0], B[1] - C[1], D[1] - C[1]
    den = dx1 * dy2 - dx2 * dy1
    g = (sx * dy2 - dx2 * sy) / den
    h = (dx1 * sy - sx * dy1) / den
    return [[B[0] - A[0] + g * B[0], D[0] - A[0] + h * D[0], A[0]],
            [B[1] - A[1] + g * B[1], D[1] - A[1] + h * D[1], A[1]],
            [g, h, 1.0]]


def adjugate(M):
    (a, b, c), (d, e, f), (g, h, i) = M
    return [[e * i - f * h, c * h - b * i, b * f - c * e],
            [f * g - d * i, a * i - c * g, c * d - a * f],
            [d * h - e * g, b * g - a * h, a * e - b * d]]


def det(M):
    (a, b, c), (d, e, f), (g, h, i) = M
    return a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)


def growth(layer, ring):
    """(gx, gy): how far any corner lies past the input buffer's sides, rounded up."""
    xs, ys = [p[0] for p in ring], [p[1] for p in ring]
    x0, y0 = layer["left"], layer["top"]
    x1, y1 = x0 + layer["w"], y0 + layer["h"]
    return (max(0, math.ceil(max(x0 - min(xs), max(xs) - x1))),
            max(0, math.ceil(max(y0 - min(ys), max(ys) - y1))))


def pinned_at(layer, M, adj, dm, dw, dh, X, Y, sign=True):
    """The output at the point (X, Y) of the drawing's space. `sign=False` drops the horizon
    test, for the check that it matters."""
    U, V, T = (r[0] * X + r[1] * Y + r[2] for r in adj)
    if (T * dm <= 0) if sign else T == 0:
        return EMPTY
    sx, sy = U / T * dw, V / T * dh
    if not (math.isfinite(sx) and math.isfinite(sy)):
        return EMPTY
    return bilinear(layer, sx, sy)


def corner_pin(layer, pins, dw, dh, sign=True, cut=True):
    """The pinned layer: its pixels and the rectangle of the drawing's space they cover.
    `cut=False` makes the output far larger than its growth, for the check that nothing is cut."""
    if all(tuple(pins[k]) == START[k] for k in NAMES):
        return layer
    ring = points(pins, dw, dh)
    if not convex(ring):
        return dict(layer, px=[EMPTY] * (layer["w"] * layer["h"]))
    M = square_to_quad(*ring)
    adj, dm = adjugate(M), det(M)
    gx, gy = growth(layer, ring) if cut else (40, 40)
    left, top = layer["left"] - gx, layer["top"] - gy
    w, h = layer["w"] + 2 * gx, layer["h"] + 2 * gy
    px = [pinned_at(layer, M, adj, dm, dw, dh, left + i + 0.5, top + j + 0.5, sign)
          for j in range(h) for i in range(w)]
    return {"px": px, "left": left, "top": top, "w": w, "h": h}


# --- the drawing ----------------------------------------------------------------------------

def drawn_layer(name):
    return {"px": [R.working(p) for row in DRAWINGS[name] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, tile=False, **pins):
    return {"drawing": "stripes", "shift": shift, "tile": tile, **START, **pins}


def held(c, k, frame_no):
    return tuple(min(RANGE[1], max(RANGE[0], v)) for v in value_at(c[k], frame_no))


def layer_of(c):
    layer = drawn_layer(c["drawing"])
    return motion_tile(layer, 300, 300, "off") if c["tile"] else layer


def render(c, frame_no, sign=True, cut=True):
    pins = {k: held(c, k, frame_no) for k in NAMES}
    return frame(corner_pin(layer_of(c), pins, W, H, sign, cut), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return frame(drawn_layer(c["drawing"]), c["shift"])


FLIP = dict(upper_left=(100, 0), upper_right=(0, 0), lower_left=(100, 100), lower_right=(0, 100))
TURN = dict(upper_left=(100, 100), upper_right=(0, 100), lower_left=(100, 0), lower_right=(0, 0))
SHRINK = dict(upper_left=(25, 25), upper_right=(75, 25), lower_left=(25, 75), lower_right=(75, 75))
LEFTWARD = dict(upper_left=(-25, 0), upper_right=(75, 0), lower_left=(-25, 100),
                lower_right=(75, 100))
FAR_LEFT = dict(upper_left=(0, 45), lower_left=(0, 55))

CASES = {
    "FX-PIN-001": ("The settings as they start, the corners at (0, 0), (100, 0), (0, 100) and "
                   "(100, 100): the drawing, untouched, and nothing grows.",
                   case(), [0]),
    "FX-PIN-002": ("Shrunk to the middle, the corners at (25, 25), (75, 25), (25, 75) and (75, "
                   "75): the whole drawing at half its size in columns 4 to 11 and rows 2 to 6, "
                   "its stripes one pixel wide; everything round it is clear. Nothing grows.",
                   case(**SHRINK), [0]),
    "FX-PIN-003": ("A keystone, Upper Left at (25, 0) and Upper Right at (75, 0): the top is half "
                   "as wide as the foot, as if the drawing leaned back, and in perspective its "
                   "far upper half is drawn smaller than its near lower half, so the blue band, "
                   "halfway down the drawing, lands in rows 2 and 3 instead of 4 and 5.",
                   case(upper_left=(25, 0), upper_right=(75, 0)), [0]),
    "FX-PIN-004": ("Turned over left to right, Upper Left at (100, 0), Upper Right at (0, 0), "
                   "Lower Left at (100, 100) and Lower Right at (0, 100): the drawing mirrored, "
                   "column 15 of the frame being column 0 of the drawing.",
                   case(**FLIP), [0]),
    "FX-PIN-005": ("A half turn, Upper Left at (100, 100), Upper Right at (0, 100), Lower Left "
                   "at (100, 0) and Lower Right at (0, 0): the drawing upside down and mirrored.",
                   case(**TURN), [0]),
    "FX-PIN-006": ("Upper Left pulled out to (-25, 0), the layer moved three pixels right: the "
                   "top left corner stretches four pixels past the drawing's left edge, the "
                   "layer grows four pixels each side, and the three columns left of the drawing "
                   "show the stretched corner in rows 0 to 8.",
                   case(3, upper_left=(-25, 0)), [0]),
    "FX-PIN-007": ("Lower Right pulled in to (60, 60): the drawing's lower right corner folds "
                   "in towards its middle, the frame's lower right is clear, and the stripes "
                   "and band bend towards it in perspective.",
                   case(lower_right=(60, 60)), [0]),
    "FX-PIN-008": ("Upper Right at (100, 100) and Lower Right at (100, 0), crossed like a bow "
                   "tie: no drawable shape, so the frame is clear.",
                   case(upper_right=(100, 100), lower_right=(100, 0)), [0]),
    "FX-PIN-009": ("Lower Right at (30, 30), inside the other three, a shape bent in: clear.",
                   case(lower_right=(30, 30)), [0]),
    "FX-PIN-010": ("Upper Right at (50, 0) and Lower Right at (100, 0), three corners in a "
                   "line: clear.",
                   case(upper_right=(50, 0), lower_right=(100, 0)), [0]),
    "FX-PIN-011": ("Upper Right keyed from (100, 0) at frame 0 to (100, 50) at frame 4, "
                   "linear: frame 0 is the drawing, frame 2 is Upper Right at (100, 25) and "
                   "frame 4 at (100, 50), the drawing's right side ever shorter and further "
                   "away.",
                   case(upper_right=keyed((0, (100, 0)), (4, (100, 50)))), [0, 2, 4]),
    "FX-PIN-012": ("Upper Left keyed from (0, 0) at frame 0 to (-400, 0) at frame 4, eased "
                   "past its end (about (-530, 0) at frame 2): frame 0 is the drawing, and "
                   "frame 2 is held at (-400, 0), the same as frame 4.",
                   case(upper_left=keyed((0, (0, 0), OVERSHOOT), (4, (-400, 0)))), [0, 2, 4]),
    "FX-PIN-013": ("All four corners 25 to the left, the layer moved four pixels right: the "
                   "drawing is moved four pixels left inside the layer and back again, so the "
                   "frame is the drawing as it was, its first four columns kept in the layer's "
                   "growth.",
                   case(4, **LEFTWARD), [0]),
    "FX-PIN-014": ("Motion Tile at 300% by 300%, then shrunk to the middle as FX-PIN-002: the "
                   "tiles come along, so the frame is filled with half-size tiles, 8 pixels "
                   "wide and 5 high, repeating across and down.",
                   case(tile=True, **SHRINK), [0]),
    "FX-PIN-015": ("Motion Tile at 300% by 300%, then Upper Left at (0, 45) and Lower Left at "
                   "(0, 55), the drawing's left side squeezed small and far away, the layer "
                   "moved eight pixels right: the tiles run away to a horizon 1.78 pixels left "
                   "of the drawing, and the columns left of it are clear, where a map without "
                   "the horizon test would draw the tiles again turned over.",
                   case(8, True, **FAR_LEFT), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-PIN-016": ("Upper Left at (-401, 0), its x below -400.", case(upper_left=(-401, 0))),
    "FX-PIN-017": ("Lower Right at (100, 501), its y above 500.",
                   case(lower_right=(100, 501))),
    "FX-PIN-018": ("Upper Right keyed to (600, 0) at frame 4.",
                   case(upper_right=keyed((0, (100, 0)), (4, (600, 0))))),
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
    if c["tile"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.motion_tile", "enabled": True,
                        "parameters": {"output_width": 300, "output_height": 300,
                                       "mirror": "off"}})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.corner_pin",
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

    (OUT / "expected_corner_pin.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    layer = drawn_layer("stripes")
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    blue = lambda p: p[2] > p[0] + 0.1  # noqa: E731  the band, not skin or line
    clear = lambda p: p == EMPTY  # noqa: E731

    # The rule's own pieces.
    for pins in (SHRINK, FLIP, TURN, FAR_LEFT, dict(upper_left=(25, 0), upper_right=(75, 0)),
                 dict(lower_right=(60, 60)), dict(upper_left=(-400, 0))):
        ring = points({**START, **pins}, W, H)
        assert convex(ring)
        M = square_to_quad(*ring)
        for (u, v), p in zip(((0, 0), (1, 0), (1, 1), (0, 1)), ring):
            x, y, t = (r[0] * u + r[1] * v + r[2] for r in M)
            assert t > 0 and abs(x / t - p[0]) < 1e-9 and abs(y / t - p[1]) < 1e-9
        adj, dm = adjugate(M), det(M)
        for i in range(3):
            for j in range(3):
                v = sum(adj[i][k] * M[k][j] for k in range(3))
                assert abs(v - (dm if i == j else 0)) < 1e-9 * max(1, abs(dm))
    for pins in (dict(upper_right=(100, 100), lower_right=(100, 0)), dict(lower_right=(30, 30)),
                 dict(upper_right=(50, 0), lower_right=(100, 0))):
        assert not convex(points({**START, **pins}, W, H))
    grow = lambda pins: growth(layer, points({**START, **pins}, W, H))  # noqa: E731
    assert grow({}) == grow(SHRINK) == grow(FAR_LEFT) == (0, 0)
    assert grow(dict(upper_left=(-25, 0))) == grow(LEFTWARD) == (4, 0)
    assert grow(dict(upper_left=(-400, 0))) == (64, 0)
    assert grow(dict(lower_right=(100, 500))) == (0, 40)
    tiled = motion_tile(layer, 300, 300, "off")
    assert growth(tiled, points({**START, **SHRINK}, W, H)) == (0, 0)

    # Nothing is cut: every case's pixels are the same with the layer grown far more.
    for fx, (_, cs, frames) in CASES.items():
        for f in frames:
            assert c[fx][str(f)] == render(cs, f, cut=False), fx

    assert c["FX-PIN-001"]["0"] == drawn
    two = c["FX-PIN-002"]["0"]
    for x in range(W):
        for y in range(H):
            assert clear(two[at(x, y)]) != (4 <= x <= 11 and 2 <= y <= 6), (x, y)
    assert two[at(4, 3)][3] > 0.99 and blue(two[at(6, 4)]) and blue(two[at(6, 5)])
    three = c["FX-PIN-003"]["0"]
    assert blue(three[at(8, 2)]) and blue(three[at(8, 3)])
    assert not any(blue(three[at(8, y)]) for y in (4, 5, 6))
    assert all(clear(three[at(x, 0)]) for x in (0, 1, 2, 3, 12, 13, 14, 15))
    assert not clear(three[at(1, 6)]) and not clear(three[at(13, 6)])
    four = c["FX-PIN-004"]["0"]
    five = c["FX-PIN-005"]["0"]
    for x in range(W):
        for y in range(H):
            assert near(four[at(x, y)], drawn[at(W - 1 - x, y)])
            assert near(five[at(x, y)], drawn[at(W - 1 - x, H - 1 - y)])
    six = c["FX-PIN-006"]["0"]
    assert all(any(six[at(x, y)][3] > 0.2 for x in range(3)) for y in range(9))
    assert render(case(3), 0)[at(0, 0)] == EMPTY  # the drawing unpinned leaves them clear
    seven = c["FX-PIN-007"]["0"]
    assert all(clear(seven[at(x, y)]) for x in range(12, W) for y in range(4, H))
    assert seven != drawn and near(seven[at(1, 1)], drawn[at(1, 1)], 0.2)
    for fx in ("FX-PIN-008", "FX-PIN-009", "FX-PIN-010"):
        assert all(clear(p) for p in c[fx]["0"]), fx
    eleven = c["FX-PIN-011"]
    assert eleven["0"] == drawn
    assert eleven["2"] == render(case(upper_right=(100, 25)), 0) != drawn
    assert eleven["4"] == render(case(upper_right=(100, 50)), 0) != eleven["2"]
    twelve = c["FX-PIN-012"]
    assert value_at(keyed((0, (0, 0), OVERSHOOT), (4, (-400, 0))), 2)[0] < -400
    assert twelve["0"] == drawn and twelve["2"] == twelve["4"] != drawn
    assert twelve["2"] == render(case(upper_left=(-400, 0)), 0)
    thirteen = c["FX-PIN-013"]["0"]
    assert all(near(p, q) for p, q in zip(thirteen, drawn))
    fourteen = c["FX-PIN-014"]["0"]
    assert all(not clear(fourteen[at(x, y)]) for x in range(W) for y in range(H))
    for x in range(W):
        for y in range(H):
            if x + 8 < W:
                assert near(fourteen[at(x, y)], fourteen[at(x + 8, y)])
            if y + 5 < H:
                assert near(fourteen[at(x, y)], fourteen[at(x, y + 5)])
    assert all(near(fourteen[at(x, y)], two[at(x, y)]) for x in range(5, 11) for y in range(3, 6))
    fifteen = c["FX-PIN-015"]["0"]
    M = square_to_quad(*points({**START, **FAR_LEFT}, W, H))
    horizon = M[0][0] / M[2][0]  # where x = (a u + c) / (g u + 1) runs to, u far away
    assert M[2][1] == 0 and abs(horizon + 1.78) < 5e-3
    assert all(clear(fifteen[at(x, y)]) for x in range(7) for y in range(H))
    assert any(not clear(fifteen[at(x, y)]) for x in range(7, W) for y in range(H))
    ghost = render(case(8, True, **FAR_LEFT), 0, sign=False)
    assert any(ghost[at(x, y)][3] > 0.2 for x in range(6) for y in range(H))
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (name, p)
    print("checked")


def show(px):
    """A map for working the cases out: . clear, b the band, s skin, l line, + part covered."""
    rows = []
    for y in range(H):
        row = ""
        for x in range(W):
            p = px[y * W + x]
            row += ("." if p[3] < 1e-9 else "+" if p[3] < 0.99 else
                    "b" if p[2] > p[0] + 0.1 else "s" if p[0] > 0.3 else "l")
        rows.append(row)
    return "\n".join(rows)


if __name__ == "__main__":
    if sys.argv[1:] == ["show"]:
        for name, pins, shift, tile in [
                ("drawn", {}, 0, False), ("shrink", SHRINK, 0, False),
                ("keystone", dict(upper_left=(25, 0), upper_right=(75, 0)), 0, False),
                ("flip", FLIP, 0, False), ("turn", TURN, 0, False),
                ("ul-25 shift3", dict(upper_left=(-25, 0)), 3, False),
                ("lr60", dict(lower_right=(60, 60)), 0, False),
                ("leftward shift4", LEFTWARD, 4, False),
                ("tile shrink", SHRINK, 0, True),
                ("tile farleft shift8", FAR_LEFT, 8, True)]:
            c = case(shift, tile, **pins)
            print(name)
            print(show(render(c, 0)))
            if tile and name.endswith("shift8"):
                print("no sign")
                print(show(render(c, 0, sign=False)))
    else:
        main()
