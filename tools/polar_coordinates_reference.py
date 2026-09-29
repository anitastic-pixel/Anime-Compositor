"""Polar coordinates, worked a second way.

D-201 adds `core.polar_coordinates`, After Effects' Polar Coordinates by a rule of our own: the
drawing is bent round its middle, its rows into rings and its columns into spokes (Rect to
Polar), or unrolled back again, its rings into rows and its spokes into columns (Polar to Rect).
Interpolation, 0 to 100, goes part of the way. It is modelled on After Effects' Polar
Coordinates and not claimed to match it; nothing is ported. Document 21 is the rule in words;
this file is the reference for the numbers document 25 pins against it.

The rule. W and H are the drawing's own size and o its place in the input buffer, which an
earlier effect may have grown. The output is the input's size; nothing grows. At an output
pixel whose centre is P, in the drawing's own space, the conversion reads the drawing at S:

  Rect to Polar: n = ((P_x - W/2) / (W/2), (P_y - H/2) / (H/2)), r = |n|, phi = atan2(n_x, -n_y),
  plus a whole turn if below 0; S = (phi / (2 pi) W, r H). The drawing's top row goes to the
  middle and its foot to the ellipse that touches the drawing's sides; its left edge runs
  straight up from the middle and its columns go round clockwise until its right edge comes
  back to the left.

  Polar to Rect: u = P_x / W, v = P_y / H, a = 2 pi u; S = (W/2 (1 + v sin a), H/2 (1 - v cos a)),
  the very inverse.

With k = interpolation / 100 the output is document 21's bilinear sample of the input at
Q = P + k (S - P), plus o, transparent outside the input. With k = 0 the output is the input
exactly. In Rect to Polar the drawing's left and right edges meet straight up from the middle,
so there a sample's columns are taken round the drawing's own width: a tap in column c of the
drawing's space reads column c mod W of it; rows are not taken round.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: Wave Warp's stripes, imported from `tools/wave_warp_reference.py`. The drawing
goes into `Fixtures/polar_coordinates/media`, the projects into `Fixtures/polar_coordinates`, and
the expected frames into `Fixtures/polar_coordinates/expected_polar_coordinates.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/polar_coordinates_reference.py
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
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "polar_coordinates"
TOLERANCE = 2e-5  # document 25's default for a filter
CONVERSIONS = ("rect_to_polar", "polar_to_rect")
EMPTY = [0.0] * 4
TURN = 2 * math.pi


# --- the rule -------------------------------------------------------------------------------

def source_point(conversion, dw, dh, x, y):
    """S: where the conversion reads the drawing for the output point (x, y)."""
    if conversion == "rect_to_polar":
        nx, ny = (x - dw / 2) / (dw / 2), (y - dh / 2) / (dh / 2)
        phi = math.atan2(nx, -ny)
        if phi < 0:
            phi += TURN
        return phi / TURN * dw, math.hypot(nx, ny) * dh
    a = TURN * (x / dw)
    v = y / dh
    return dw / 2 * (1 + v * math.sin(a)), dh / 2 * (1 - v * math.cos(a))


def wrapped(layer, x, y, dw):
    """Document 21's bilinear sample, each tap's column taken round the drawing's width."""
    fx, fy = x - 0.5, y - 0.5
    x0, y0 = math.floor(fx), math.floor(fy)
    ux, uy = fx - x0, fy - y0
    out = [0.0] * 4
    for dy, wy in ((0, 1 - uy), (1, uy)):
        for dx, wx in ((0, 1 - ux), (1, ux)):
            sx, sy = (x0 + dx) % dw - layer["left"], y0 + dy - layer["top"]
            if 0 <= sx < layer["w"] and 0 <= sy < layer["h"] and wx * wy:
                p = layer["px"][sy * layer["w"] + sx]
                for i in range(4):
                    out[i] += p[i] * wx * wy
    return out


def polar(layer, interpolation, conversion, dw, dh, wrap=True):
    """The converted layer, the same rectangle as the input. `wrap=False` reads the columns
    straight, for the check that taking them round matters."""
    k = interpolation / 100
    if k == 0:
        return layer
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            x, y = layer["left"] + i + 0.5, layer["top"] + j + 0.5
            sx, sy = source_point(conversion, dw, dh, x, y)
            qx, qy = x + k * (sx - x), y + k * (sy - y)
            px.append(wrapped(layer, qx, qy, dw) if conversion == "rect_to_polar" and wrap
                      else bilinear(layer, qx, qy))
    return dict(layer, px=px)


# --- the drawing ----------------------------------------------------------------------------

def drawn_layer(name):
    return {"px": [R.working(p) for row in DRAWINGS[name] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


# --- the cases ------------------------------------------------------------------------------

def case(interpolation=100, conversion="rect_to_polar", shift=0, tile=False, then=None):
    """`then` is a second Polar Coordinates after the first, (interpolation, conversion)."""
    return {"drawing": "stripes", "interpolation": interpolation, "conversion": conversion,
            "shift": shift, "tile": tile, "then": then}


def held(v):
    return min(100, max(0, v))


def layer_of(c):
    layer = drawn_layer(c["drawing"])
    return motion_tile(layer, 300, 300, "off") if c["tile"] else layer


def render(c, frame_no, wrap=True):
    layer = polar(layer_of(c), held(value_at(c["interpolation"], frame_no)), c["conversion"],
                  W, H, wrap)
    if c["then"]:
        layer = polar(layer, c["then"][0], c["then"][1], W, H, wrap)
    return frame(layer, c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it, tiled if the case tiles it."""
    return frame(layer_of(c), c["shift"])


CASES = {
    "FX-POLAR-001": ("The settings as they start, Interpolation 100 and Rect to Polar: the drawing "
                     "bent round its middle. Its top row is squeezed into the middle, its blue "
                     "band, rows 4 and 5, is a ring halfway out, and its foot, the empty row 9, "
                     "lies round the ellipse that touches the frame's sides, so the frame's "
                     "corners, beyond it, are clear. Its upright stripes are spokes from the "
                     "middle, and its empty right-hand column is a clear spoke straight up.",
                     case(), [0]),
    "FX-POLAR-002": ("Interpolation 0: the drawing, untouched.", case(0), [0]),
    "FX-POLAR-003": ("Polar to Rect at 100: the drawing unrolled. Its middle, blue, is spread "
                     "along the frame's top rows; each lower row reads a ring further out, the "
                     "left column straight up from the middle and the columns going round "
                     "clockwise, so the stripes cross the frame as slanting bands.",
                     case(conversion="polar_to_rect"), [0]),
    "FX-POLAR-004": ("Rect to Polar at 50: half way, each pixel read from halfway between "
                     "where it is and where FX-POLAR-001 reads it.",
                     case(50), [0]),
    "FX-POLAR-005": ("Polar to Rect at 50: half way to FX-POLAR-003.",
                     case(50, "polar_to_rect"), [0]),
    "FX-POLAR-006": ("Interpolation keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 "
                     "is the drawing, frame 2 is FX-POLAR-004 and frame 4 is FX-POLAR-001.",
                     case(keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-POLAR-007": ("Interpolation keyed from 0 at frame 0 to 100 at frame 4, eased past its "
                     "end (about 131 at frame 2): frame 0 is the drawing, and frame 2 is held "
                     "at 100, FX-POLAR-001.",
                     case(keyed((0, 0, OVERSHOOT), (4, 100))), [0, 2, 4]),
    "FX-POLAR-008": ("Rect to Polar at 100, the layer moved three pixels right: FX-POLAR-001 "
                     "moved with it.",
                     case(shift=3), [0]),
    "FX-POLAR-009": ("Motion Tile at 300% by 300%, then Rect to Polar at 100, the layer moved "
                     "eight pixels right: inside the ellipse FX-POLAR-001 moved with the layer, "
                     "and beyond it, where Rect to Polar reads below the drawing's foot, the "
                     "tiles beneath the drawing, bent round.",
                     case(shift=8, tile=True), [0]),
    "FX-POLAR-010": ("Motion Tile at 300% by 300%, then Polar to Rect at 100, the layer moved "
                     "eight pixels right: FX-POLAR-003 in the drawing's place, and to its left "
                     "FX-POLAR-003's right half again, the rule going round once more past the "
                     "drawing's side; only the middle of the foot row differs, reading a little "
                     "of the tile below the drawing where FX-POLAR-003 reads nothing.",
                     case(conversion="polar_to_rect", shift=8, tile=True), [0]),
    "FX-POLAR-011": ("Rect to Polar at 100, then a second Polar Coordinates, Polar to Rect at "
                     "100: the drawing back again, roughly: its band and stripes where they "
                     "were, but soft in its top row, which the first squeezed into the middle, "
                     "and all round its edges, which the first bent into the ellipse's rim and "
                     "the wedge its empty column made.",
                     case(then=(100, "polar_to_rect")), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-POLAR-012": ("Interpolation 101, above 100.", case(101)),
    "FX-POLAR-013": ("Interpolation -1, below 0.", case(-1)),
    "FX-POLAR-014": ("Interpolation keyed to 150 at frame 4.", case(keyed((0, 0), (4, 150)))),
    "FX-POLAR-015": ("Conversion \"sideways\", which is not one.", case(conversion="sideways")),
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
    for interpolation, conversion in [(c["interpolation"], c["conversion"])] + (
            [c["then"]] if c["then"] else []):
        effects.append({"instance_id": f"fx-0-{len(effects)}",
                        "type_id": "core.polar_coordinates", "enabled": True,
                        "parameters": {"interpolation": setting_json(interpolation),
                                       "conversion": conversion}})
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

    (OUT / "expected_polar_coordinates.json").write_text(json.dumps(expected, indent=1) + "\n",
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

    # The rule's own pieces: the two conversions undo each other, and S of the middle column
    # straight up is the drawing's left edge.
    for x, y in ((3.5, 2.5), (12.5, 7.5), (8.5, 0.5), (1.5, 9.5), (15.5, 4.5)):
        sx, sy = source_point("rect_to_polar", W, H, x, y)
        bx, by = source_point("polar_to_rect", W, H, sx, sy)
        assert abs(bx - x) < 1e-9 and abs(by - y) < 1e-9, (x, y)
    assert source_point("rect_to_polar", W, H, 8, 1)[0] == 0
    assert abs(source_point("rect_to_polar", W, H, 16, 5)[0] - W / 4) < 1e-12  # right: a quarter
    assert source_point("rect_to_polar", W, H, 16, 5)[1] == H  # the ellipse: the foot
    assert source_point("polar_to_rect", W, H, 0, 0) == (W / 2, H / 2)

    one = c["FX-POLAR-001"]["0"]
    assert c["FX-POLAR-002"]["0"] == drawn
    for x, y in ((0, 0), (15, 0), (0, 9), (15, 9)):
        assert clear(one[at(x, y)]), (x, y)
    for x, y in ((9, 2), (8, 7), (4, 5), (12, 4)):  # halfway out, the band's ring
        assert blue(one[at(x, y)]), (x, y)
    assert not blue(one[at(8, 4)]) and not blue(one[at(8, 5)])  # the middle is the top row
    # Straight up from the middle, the right-hand columns: the soft skin then the empty column.
    assert one[at(7, 0)][3] < 0.9 and one[at(7, 1)][3] < 0.9
    # Taken round, a tap past the right edge reads the left column, skin; read straight, it
    # reads nothing, and a clear line would run straight up from the middle.
    straight = render(case(), 0, wrap=False)
    assert all(straight[at(7, y)] == EMPTY and one[at(7, y)][3] > 0.1 for y in (0, 1, 2))
    three = c["FX-POLAR-003"]["0"]
    assert all(blue(three[at(x, 0)]) for x in range(W))
    assert not blue(three[at(0, 8)]) and three[at(0, 8)][3] > 0.99
    assert c["FX-POLAR-004"]["0"] not in (drawn, one)
    assert c["FX-POLAR-005"]["0"] not in (drawn, three)
    six = c["FX-POLAR-006"]
    assert six["0"] == drawn and six["2"] == c["FX-POLAR-004"]["0"] and six["4"] == one
    seven = c["FX-POLAR-007"]
    assert value_at(keyed((0, 0, OVERSHOOT), (4, 100)), 2) > 100
    assert seven["0"] == drawn and seven["2"] == seven["4"] == one
    eight = c["FX-POLAR-008"]["0"]
    assert all(eight[at(x + 3, y)] == one[at(x, y)] for x in range(W - 3) for y in range(H))
    nine = c["FX-POLAR-009"]["0"]
    inside = [(x, y) for x in range(8) for y in range(H)
              if math.hypot((x + 0.5 - 8) / 8, (y + 0.5 - 5) / 5) < 0.8]
    assert all(near(nine[at(x + 8, y)], one[at(x, y)]) for x, y in inside)
    assert any(not clear(nine[at(x + 8, y)]) and clear(one[at(x, y)])
               for x in range(8) for y in range(H))
    ten = c["FX-POLAR-010"]["0"]
    assert all(near(ten[at(x + 8, y)], three[at(x, y)]) for x in range(8) for y in range(H - 1))
    assert all(near(ten[at(x, y)], three[at(x + 8, y)]) for x in range(8) for y in range(H - 1))
    assert not near(ten[at(15, 9)], three[at(7, 9)]) and ten[at(15, 9)][3] > three[at(7, 9)][3]
    assert all(not clear(ten[at(x, y)]) for x in range(8) for y in range(3))
    eleven = c["FX-POLAR-011"]["0"]
    assert all(blue(eleven[at(x, 4)]) and blue(eleven[at(x, 5)]) for x in range(1, 13))
    assert all(not blue(eleven[at(x, 7)]) for x in range(13))
    assert all(eleven[at(x, 0)][3] < 0.99 for x in range(W))
    assert all(eleven[at(x, y)][3] < 0.9 for x, y in ((0, 9), (8, 9), (15, 5)))
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
        for fx, (_, c, frames) in CASES.items():
            for f in frames:
                print(fx, f)
                print(show(render(c, f)))
        print("no wrap")
        print(show(render(case(), 0, wrap=False)))
    else:
        main()
