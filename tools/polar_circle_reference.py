"""D-320's round Polar Coordinates, worked a second way.

The owner's D-308 approval (2026-10-04, from P-26's tutorial 1) gives `core.polar_coordinates` a
`shape`: "ellipse", D-201's rule, which bends the drawing into the ellipse that touches its
sides; or "circle", After Effects' look, a circle round the drawing's middle whose radius is half
its shorter side. A file without `shape` is "ellipse", so FX-POLAR-001 to 015 stand as written in
`Fixtures/polar_coordinates/expected_polar_coordinates.json`. The cases below pin "circle" with
`tools/polar_coordinates_reference.py`'s drawing, sample and frame; its own cases are not written
again.

The rule. With W and H the drawing's own size, r = min(W, H) / 2 for a circle and (W/2, H/2) for
an ellipse, as (rx, ry):

  Rect to Polar: n = ((P_x - W/2) / rx, (P_y - H/2) / ry), and then as D-201: phi = atan2(n_x,
  -n_y), plus a turn if below 0; S = (phi / (2 pi) W, |n| H).

  Polar to Rect: a = 2 pi P_x / W, v = P_y / H; S = (W/2 + rx v sin a, H/2 - ry v cos a), the
  very inverse.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/polar_circle_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import polar_coordinates_reference as P  # noqa: E402

W, H = P.W, P.H


def source_point(conversion, shape, dw, dh, x, y):
    """S for the output point (x, y), with the shape's half-axes."""
    rx, ry = (min(dw, dh) / 2,) * 2 if shape == "circle" else (dw / 2, dh / 2)
    if conversion == "rect_to_polar":
        nx, ny = (x - dw / 2) / rx, (y - dh / 2) / ry
        phi = math.atan2(nx, -ny)
        if phi < 0:
            phi += P.TURN
        return phi / P.TURN * dw, math.hypot(nx, ny) * dh
    a = P.TURN * (x / dw)
    v = y / dh
    return dw / 2 + rx * v * math.sin(a), dh / 2 - ry * v * math.cos(a)


def polar(layer, interpolation, conversion, shape):
    k = interpolation / 100
    if k == 0:
        return layer
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            x, y = layer["left"] + i + 0.5, layer["top"] + j + 0.5
            sx, sy = source_point(conversion, shape, W, H, x, y)
            qx, qy = x + k * (sx - x), y + k * (sy - y)
            px.append(P.wrapped(layer, qx, qy, W) if conversion == "rect_to_polar"
                      else P.bilinear(layer, qx, qy))
    return dict(layer, px=px)


def case(interpolation=100, conversion="rect_to_polar", shape="circle", shift=0, then=None):
    """`then` is a second Polar Coordinates, (interpolation, conversion, shape)."""
    c = P.case(interpolation, conversion, shift)
    c.update(shape=shape, then=then)
    return c


def render(c, frame_no=0):
    layer = polar(P.layer_of(c), P.held(c["interpolation"]), c["conversion"], c["shape"])
    if c["then"]:
        layer = polar(layer, *c["then"])
    return P.frame(layer, c["shift"])


CASES = {
    "FX-POLAR-016": ("Shape circle, Rect to Polar at 100, on the 16 by 10 drawing: the drawing "
                     "bent into a circle of radius 5 round the frame's middle. Its blue band is "
                     "a round ring halfway out, its foot lies round the circle, which touches "
                     "the top and bottom, and the frame's left and right ends, beyond the "
                     "circle, are clear.", case(), [0]),
    "FX-POLAR-017": ("Shape circle, Polar to Rect at 100: the drawing unrolled from the circle; "
                     "each row reads a ring of the same radius across and down.",
                     case(conversion="polar_to_rect"), [0]),
    "FX-POLAR-018": ("Shape circle, Rect to Polar at 50: half way to FX-POLAR-016.",
                     case(50), [0]),
    "FX-POLAR-019": ("Shape circle, Rect to Polar at 100, then a second, shape circle, Polar to "
                     "Rect at 100: the drawing back again, roughly, as FX-POLAR-011 is for the "
                     "ellipse.", case(then=(100, "polar_to_rect", "circle")), [0]),
    "FX-POLAR-020": ("Shape circle, Rect to Polar at 100, the layer moved three pixels right: "
                     "FX-POLAR-016 moved with it.", case(shift=3), [0]),
    "FX-POLAR-021": ("Shape ellipse, written: FX-POLAR-001 exactly.", case(shape="ellipse"), [0]),
}

INVALID = {
    "FX-POLAR-022": ("Shape \"square\", which is not one.", case(shape="square")),
}


def write(fx, c):
    p = P.project_json(fx, dict(c, then=None))
    effects = [e for e in p["compositions"][0]["layers"][0]["effects"]
               if e["type_id"] == "core.polar_coordinates"]
    effects[0]["parameters"]["shape"] = c["shape"]
    if c["then"]:
        effects.append({"instance_id": "fx-0-1", "type_id": "core.polar_coordinates",
                        "enabled": True,
                        "parameters": {"interpolation": c["then"][0], "conversion": c["then"][1],
                                       "shape": c["then"][2]}})
        p["compositions"][0]["layers"][0]["effects"] = effects
    name = f"{fx.lower().replace('-', '_')}.json"
    (P.OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def main():
    expected = {"tolerance": P.TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {str(f): render(c, f) for f in frames}}
        print(f"{fx}: {says[:60]}")
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = P.plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before}, "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (P.OUT / "expected_polar_circle.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                      encoding="utf-8")
    check(expected)


def check(expected):
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    blue = lambda p: p[2] > p[0] + 0.1  # noqa: E731
    # The ellipse here is D-201's rule, to the last bits of a double.
    for x, y in ((3.5, 2.5), (12.5, 7.5), (15.5, 4.5)):
        for conv in P.CONVERSIONS:
            a, b = source_point(conv, "ellipse", W, H, x, y), P.source_point(conv, W, H, x, y)
            assert all(abs(p - q) < 1e-12 for p, q in zip(a, b)), (conv, x, y)
    assert c["FX-POLAR-021"] == P.render(P.case(), 0)
    # The circle's two conversions undo each other.
    for x, y in ((3.5, 2.5), (12.5, 7.5), (8.5, 0.5), (6.5, 6.5)):
        sx, sy = source_point("rect_to_polar", "circle", W, H, x, y)
        bx, by = source_point("polar_to_rect", "circle", W, H, sx, sy)
        assert abs(bx - x) < 1e-9 and abs(by - y) < 1e-9, (x, y)
    one = c["FX-POLAR-016"]
    # Beyond the circle, the frame's ends are clear, where the ellipse has drawing.
    ellipse = P.render(P.case(), 0)
    for x, y in ((0, 5), (1, 4), (15, 5), (14, 4)):
        assert one[at(x, y)] == P.EMPTY and ellipse[at(x, y)] != P.EMPTY, (x, y)
    # The band's ring is round: as far out across as down, 2.5 pixels from the middle.
    for x, y in ((5, 4), (10, 5), (8, 2), (7, 7)):
        assert blue(one[at(x, y)]), (x, y)
    assert c["FX-POLAR-018"] not in (one, P.plain(case()))
    assert c["FX-POLAR-017"] != P.render(P.case(conversion="polar_to_rect"), 0)
    twenty = c["FX-POLAR-020"]
    assert all(twenty[at(x + 3, y)] == one[at(x, y)] for x in range(W - 3) for y in range(H))
    nineteen = c["FX-POLAR-019"]
    assert all(blue(nineteen[at(x, 4)]) for x in range(4, 12))
    print("checked")


if __name__ == "__main__":
    if sys.argv[1:] == ["show"]:
        for fx, (_, c, frames) in CASES.items():
            print(fx)
            print(P.show(render(c)))
        print("ellipse")
        print(P.show(P.render(P.case(), 0)))
    else:
        main()
