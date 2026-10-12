"""Liquify, worked a second way.

D-605 adds `core.liquify`, after After Effects' Liquify (Distort): a brush dragged over the
layer pushes, turns, pinches or swells the drawing under it. After Effects names its tools and
publishes no formula; the rule below is this program's own, and nothing is ported.

The settings. `tool`, the brush: `warp` (pushes the drawing along the stroke), `twirl_clockwise`
and `twirl_counterclockwise` (turns it about each dab), `pucker` (draws it in toward each dab),
`bloat` (swells it out from each dab) or `shift_pixels` (moves it square to the stroke, to the
left of the way it goes); `warp` when added. `brush_size`, the brush's width in pixels, 1 to
1000, 64; `brush_pressure`, 1 to 100, 50. `stroke`, the brush's path: none, or x then y for each
point, per cent of the drawing, -100 to 200; none when added. `distortion_percentage`, how much
of the distortion is drawn, 0 to 200, 100; `distortion_mesh_offset`, where the distortion sits
against the drawing, x then y in pixels, -10000 to 10000, (0, 0). Every number is keyable, the
stroke as one list.

The rule. The stroke's points are taken to pixels (x = X / 100 W, y = Y / 100 H). The brush
lands in dabs along the path: L is the path's whole length and s = max(R / 4, 1/2) the spacing,
R = `brush_size` / 2 the brush's reach; n = max(1, ceil(L / s)) dabs, dab k (1 to n) at k L / n
along the path, each moving d, L / n along the piece of path it is on (a path of one point, or
of no length, is one dab at its first point with d = (0, 0)). Each dab at c reaches the point p
with f = P (1 - t^2)^2 for t = |p - c| / R below 1, else 0, P = `brush_pressure` / 100, and adds
a step there, the drawing read that far back from p:

- `warp`: -f d: the drawing carried along the stroke.
- `twirl_clockwise`: the turn of (p - c) by -a about c, less (p - c), a = 0.2 f radians: the
  drawing turned clockwise on screen (y runs down). `twirl_counterclockwise` turns by +a.
- `pucker`: 0.2 f (p - c): read from farther out, so drawn in. `bloat`: -0.2 f (p - c).
- `shift_pixels`: -f (d_y, -d_x): carried to the left of the way the stroke goes.

The steps are added up over every dab at each corner of a grid of gx by gy cells over the
drawing, gx = min(256, ceil(W / 2)), gy = min(256, ceil(H / 2)), and the frame drawn as
`tools/mesh_reference.py` (D-601) draws a backward field: the output at X is the drawing at X
plus `distortion_percentage` / 100 of the step at X less the offset, bilinear, clear outside the
drawing. With no stroke, or `distortion_percentage` 0, the drawing is untouched.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 holding Bulge's striped drawing, the same size,
unmoved unless the case says. The drawing goes into `Fixtures/liquify/media`, the projects into
`Fixtures/liquify`, and the expected frames into `Fixtures/liquify/expected_liquify.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/liquify_reference.py
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
import bulge_reference as B  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
import mesh_reference as M  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "liquify"
TOLERANCE = 2e-5  # document 25's default for a filter
TOOLS = ("warp", "twirl_clockwise", "twirl_counterclockwise", "pucker", "bloat", "shift_pixels")
NAMES = ("tool", "brush_size", "brush_pressure", "stroke", "distortion_percentage",
         "distortion_mesh_offset")
RANGES = {"brush_size": (1, 1000), "brush_pressure": (1, 100), "stroke": (-100, 200),
          "distortion_percentage": (0, 200), "distortion_mesh_offset": (-10000, 10000)}


# --- the rule -------------------------------------------------------------------------------

def dabs(points, reach):
    """Where each dab lands and how far it moves, in pixels."""
    pts = [(points[i] / 100 * W, points[i + 1] / 100 * H) for i in range(0, len(points), 2)]
    pieces = [(a, b, math.hypot(b[0] - a[0], b[1] - a[1])) for a, b in zip(pts, pts[1:])]
    length = sum(p[2] for p in pieces)
    if length == 0:
        return [(pts[0], (0.0, 0.0))]
    n = max(1, math.ceil(length / max(reach / 4, 0.5)))
    out = []
    for k in range(1, n + 1):
        left = k * length / n
        for a, b, l in pieces:
            if l == 0:
                continue
            if left <= l or (a, b, l) == pieces[-1]:
                u = min(left / l, 1.0)
                c = (a[0] + (b[0] - a[0]) * u, a[1] + (b[1] - a[1]) * u)
                d = ((b[0] - a[0]) / l * length / n, (b[1] - a[1]) / l * length / n)
                out.append((c, d))
                break
            left -= l
    return out


def step(tool, f, p, c, d):
    """One dab's step at p."""
    qx, qy = p[0] - c[0], p[1] - c[1]
    if tool == "warp":
        return -f * d[0], -f * d[1]
    if tool in ("twirl_clockwise", "twirl_counterclockwise"):
        a = 0.2 * f * (-1 if tool == "twirl_clockwise" else 1)
        return (qx * math.cos(a) - qy * math.sin(a) - qx, qx * math.sin(a) + qy * math.cos(a) - qy)
    if tool == "pucker":
        return 0.2 * f * qx, 0.2 * f * qy
    if tool == "bloat":
        return -0.2 * f * qx, -0.2 * f * qy
    return -f * d[1], f * d[0]  # shift_pixels


def field(s_):
    gx, gy = min(256, math.ceil(W / 2)), min(256, math.ceil(H / 2))
    reach = s_["brush_size"] / 2
    pressure = s_["brush_pressure"] / 100
    steps = []
    landed = dabs(s_["stroke"], reach)
    for j in range(gy + 1):
        for i in range(gx + 1):
            p = (i * W / gx, j * H / gy)
            sx = sy = 0.0
            for c, d in landed:
                t = math.hypot(p[0] - c[0], p[1] - c[1]) / reach
                if t < 1:
                    f = pressure * (1 - t * t) ** 2
                    a, b = step(s_["tool"], f, p, c, d)
                    sx += a
                    sy += b
            steps.append((sx, sy))
    return steps, gx, gy


def liquify(layer, s_):
    if not s_["stroke"] or s_["distortion_percentage"] == 0:
        return layer
    steps, gx, gy = field(s_)
    return M.draw_field(layer, steps, gx, gy, W, H, s_["distortion_percentage"] / 100,
                        tuple(s_["distortion_mesh_offset"]))


# --- the cases ------------------------------------------------------------------------------

ACROSS = [25, 50, 75, 50]


def case(shift=0, tool="warp", brush_size=8, brush_pressure=50, stroke=ACROSS,
         distortion_percentage=100, distortion_mesh_offset=(0, 0)):
    return {"drawing": "stripes", "shift": shift, "tool": tool, "brush_size": brush_size,
            "brush_pressure": brush_pressure, "stroke": list(stroke) if isinstance(stroke, (list, tuple)) else stroke,
            "distortion_percentage": distortion_percentage,
            "distortion_mesh_offset": list(distortion_mesh_offset)}


def settings(c, frame_no):
    held = {"tool": c["tool"]}
    for k, (lo, hi) in RANGES.items():
        v = value_at(c[k], frame_no)
        held[k] = ([min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple))
                   else min(hi, max(lo, v)))
    return held


def render(c, frame_no):
    return frame(liquify(B.drawn_layer(c["drawing"]), settings(c, frame_no)), c["shift"])


def plain(c):
    return frame(B.drawn_layer(c["drawing"]), c["shift"])


DOT = [50, 50]
CASES = {
    "FX-LIQUIFY-001": ("The settings as they start: Warp, brush 64, pressure 50, and no stroke: "
                       "the drawing, untouched.",
                       case(brush_size=64, stroke=[]), [0]),
    "FX-LIQUIFY-002": ("Warp, brush 8, pressure 50, stroked across the middle from a quarter to "
                       "three quarters: the stripes under the stroke pushed right.", case(), [0]),
    "FX-LIQUIFY-003": ("Twirl Clockwise, brush 10, pressure 100, one dab in the middle: the "
                       "stripes there turned clockwise.",
                       case(tool="twirl_clockwise", brush_size=10, brush_pressure=100,
                            stroke=DOT), [0]),
    "FX-LIQUIFY-004": ("Twirl Counterclockwise, as FX-LIQUIFY-003: turned the other way.",
                       case(tool="twirl_counterclockwise", brush_size=10, brush_pressure=100,
                            stroke=DOT), [0]),
    "FX-LIQUIFY-005": ("Pucker, brush 10, pressure 100, one dab in the middle: the drawing "
                       "drawn in toward it.",
                       case(tool="pucker", brush_size=10, brush_pressure=100, stroke=DOT), [0]),
    "FX-LIQUIFY-006": ("Bloat, as FX-LIQUIFY-005: swollen out from it.",
                       case(tool="bloat", brush_size=10, brush_pressure=100, stroke=DOT), [0]),
    "FX-LIQUIFY-007": ("Shift Pixels, stroked as FX-LIQUIFY-002: the stripes moved square to "
                       "the stroke, up the picture, to its left.", case(tool="shift_pixels"), [0]),
    "FX-LIQUIFY-008": ("FX-LIQUIFY-002 at Distortion Percentage 200: pushed twice as far.",
                       case(distortion_percentage=200), [0]),
    "FX-LIQUIFY-009": ("FX-LIQUIFY-002 with the Distortion Mesh Offset 3 pixels right: the "
                       "push lands three pixels on.", case(distortion_mesh_offset=(3, 0)), [0]),
    "FX-LIQUIFY-010": ("FX-LIQUIFY-002 with the layer moved three pixels right: the pushed "
                       "drawing moves with it.", case(3), [0]),
    "FX-LIQUIFY-011": ("Distortion Percentage keyed from 0 at frame 0 to 100 at frame 4, "
                       "linear: frame 0 the drawing, frame 4 FX-LIQUIFY-002, frame 2 between.",
                       case(distortion_percentage=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-LIQUIFY-012": ("The stroke keyed from across the middle at frame 0 to rising to the "
                       "top right at frame 4: the push turns upward.",
                       case(stroke=keyed((0, ACROSS), (4, [25, 50, 75, 20]))), [0, 2, 4]),
    "FX-LIQUIFY-013": ("Warp, brush 6, pressure 80, stroked down and back up in a V: the push "
                       "follows each leg.",
                       case(brush_size=6, brush_pressure=80, stroke=[10, 20, 50, 80, 90, 20]),
                       [0]),
    "FX-LIQUIFY-014": ("Brush Pressure keyed from 20 at frame 0 to 100 at frame 4, eased past "
                       "its end: frame 2 would pass 100 and is held there, the same as frame 4.",
                       case(brush_pressure=keyed((0, 20, OVERSHOOT), (4, 100))), [0, 2, 4]),
    "FX-LIQUIFY-015": ("Warp with a brush of 1000, wider than the drawing: the whole drawing "
                       "pushed right almost evenly, four pixels, clear down the left.",
                       case(brush_size=1000), [0]),
}

INVALID = {
    "FX-LIQUIFY-016": ("Tool \"freeze\", not one of the six.", case(tool="freeze")),
    "FX-LIQUIFY-017": ("Brush Size 0, below 1.", case(brush_size=0)),
    "FX-LIQUIFY-018": ("Brush Pressure 101, above 100.", case(brush_pressure=101)),
    "FX-LIQUIFY-019": ("A stroke of three numbers, half a point over.", case(stroke=[25, 50, 75])),
    "FX-LIQUIFY-020": ("Distortion Percentage 201, above 200.", case(distortion_percentage=201)),
    "FX-LIQUIFY-021": ("The stroke keyed to end at 300 per cent across at frame 4.",
                       case(stroke=keyed((0, ACROSS), (4, [25, 50, 300, 50])))),
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
        "instance_id": "fx-0-0", "type_id": "core.liquify", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_bytes((json.dumps(project_json(fx, c), indent=2) + "\n").encode("utf-8"))
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in B.DRAWINGS.items():
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

    (OUT / "expected_liquify.json").write_bytes(
        (json.dumps(expected, indent=1) + "\n").encode("utf-8"))
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    layer = B.drawn_layer("stripes")
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda a, b, e=1e-9: all(near(p, q, e) for p, q in zip(a, b))  # noqa: E731
    at = lambda px, x, y: px[y * W + x]  # noqa: E731

    # The dabs: a straight stroke of 8 pixels at reach 4 lands 8 dabs a pixel apart, each moving
    # one pixel right; one point is one still dab.
    d = dabs(ACROSS, 4)
    assert len(d) == 8 and near(d[-1][0], (12, 5)) and all(near(m, (1, 0)) for _, m in d)
    assert dabs(DOT, 5) == [((8.0, 5.0), (0.0, 0.0))]
    v = dabs([10, 20, 50, 80, 90, 20], 3)
    assert near(v[-1][0], (14.4, 2)), v[-1]
    # The twirls turn opposite ways; pucker and bloat step opposite ways.
    assert near(step("twirl_clockwise", 1, (1, 0), (0, 0), None),
                (math.cos(0.2) - 1, -math.sin(0.2)))
    assert near(step("pucker", 1, (1, 2), (0, 0), None),
                [-u for u in step("bloat", 1, (1, 2), (0, 0), None)])

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values())

    assert c["FX-LIQUIFY-001"]["0"] == drawn
    looks = [c[f"FX-LIQUIFY-{n:03}"]["0"] for n in range(2, 8)]
    for i, a in enumerate(looks):
        assert not same(a, drawn), i
        for b in looks[i + 1:]:
            assert not same(a, b), i
    # The push leaves the corners, far from the stroke, alone.
    push = c["FX-LIQUIFY-002"]["0"]
    for x, y in ((0, 0), (15, 0), (0, 9), (15, 9)):
        assert near(at(push, x, y), at(drawn, x, y)), (x, y)
    assert not same(c["FX-LIQUIFY-008"]["0"], push) and not same(c["FX-LIQUIFY-009"]["0"], push)
    assert same(c["FX-LIQUIFY-010"]["0"], frame(liquify(layer, settings(case(), 0)), 3))
    keyed_on = c["FX-LIQUIFY-011"]
    assert keyed_on["0"] == drawn and same(keyed_on["4"], push) and not same(keyed_on["2"], push)
    turned = c["FX-LIQUIFY-012"]
    assert same(turned["0"], push) and not same(turned["4"], push)
    held = c["FX-LIQUIFY-014"]
    assert value_at(keyed((0, 20, OVERSHOOT), (4, 100)), 2) > 100 and held["2"] == held["4"]
    # The wide brush: the left columns read from left of the drawing, so clear.
    wide = c["FX-LIQUIFY-015"]["0"]
    assert all(at(wide, 0, y)[3] < 1e-9 for y in range(H)) and not same(wide, push)
    print("checked")


if __name__ == "__main__":
    main()
