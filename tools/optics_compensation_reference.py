"""Optics Compensation, worked a second way.

D-210 adds `core.optics_compensation`: the curve of a wide lens, put into a picture as a fisheye,
the middle kept and the edges squeezed round it, or, with `reverse` on, taken out, the edges
stretched back so lines bowed by a fisheye run straight. It is After Effects' Optics
Compensation in purpose, and this program's own rule. Nothing is ported. Document 21 is the rule
in words; this file is the reference for the numbers document 25 pins against it.

The rule. `field_of_view`, 0 to 180 degrees, is the angle the lens sees across the span that
`orientation` names: "horizontal" the drawing's width, "vertical" its height, "diagonal" its
diagonal; R is half that span in the drawing's own pixels, and theta half the angle in radians.
`center`, per cent of the drawing's own width and height as Bulge's is, is the lens's middle c.
For a pixel's centre P at d = |P - c| from it, a = d / R * theta. With `reverse` "off" (a
fisheye put in), a pixel with a >= pi / 2 is empty, and any other reads from s = R tan(a) / theta
out from c along the same line; with "on" (a fisheye taken out), s = R atan(a) / theta. The
output is document 21's bilinear sample of the input at c + (s / d)(P - c), transparent outside
it; a pixel at c itself reads itself. So the middle keeps its size either way; "off" reads from
farther out, more so the farther out it is, squeezing the edges in and emptying what reads past
the drawing, and "on" reads from nearer in, stretching the edges out; "on" undoes "off" at the
same settings exactly, but for the blur of sampling twice. Field of view 0 leaves the layer as it
is. The layer does not grow, and a draft changes nothing: R is the drawing's own size, which a
draft already scales, and the angle and the centre are not distances. After Effects' Optimal
Pixels and Resize are not taken: the layer keeps its size and what is squeezed off stays off.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless the
case says: Bulge's stripes (`tools/bulge_reference.py`). The drawing goes into
`Fixtures/optics_compensation/media`, the projects into `Fixtures/optics_compensation`, and the
expected frames into `Fixtures/optics_compensation/expected_optics_compensation.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/optics_compensation_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
import bulge_reference as B  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "optics_compensation"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"field_of_view": (0, 180), "center": (-1000, 1000)}
ORIENTATIONS = ("horizontal", "vertical", "diagonal")
WORDS = ("reverse", "orientation")
NAMES = ("field_of_view", "reverse", "orientation", "center")
DRAWINGS = {"stripes": B.DRAWINGS["stripes"]}
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def half_span(orientation):
    return {"horizontal": W / 2, "vertical": H / 2, "diagonal": math.hypot(W, H) / 2}[orientation]


def source(fov, reverse, orientation, center, px, py):
    """Where the pixel whose centre is (px, py), in the drawing's own space, reads from, or None
    where it is empty."""
    theta = math.radians(fov) / 2
    cx, cy = center[0] / 100 * W, center[1] / 100 * H
    dx, dy = px - cx, py - cy
    d = math.hypot(dx, dy)
    if d == 0:
        return px, py
    r = half_span(orientation)
    a = d / r * theta
    if reverse == "off":
        if a >= math.pi / 2:
            return None
        s = r * math.tan(a) / theta
    else:
        s = r * math.atan(a) / theta
    m = s / d
    return cx + m * dx, cy + m * dy


def optics(layer, n, c):
    if n["field_of_view"] == 0:
        return layer
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            q = source(n["field_of_view"], c["reverse"], c["orientation"], n["center"],
                       layer["left"] + i + 0.5, layer["top"] + j + 0.5)
            px.append(EMPTY if q is None else bilinear(layer, *q))
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(field_of_view=90, reverse="off", orientation="horizontal", center=(50, 50), shift=0,
         tile=False):
    return {"drawing": "stripes", "field_of_view": field_of_view, "reverse": reverse,
            "orientation": orientation, "center": center, "shift": shift, "tile": tile}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = value_at(c[k], frame_no)
    return [min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple)) else min(hi, max(lo, v))


def layer_of(c):
    layer = {"px": [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
             "left": 0, "top": 0, "w": W, "h": H}
    return motion_tile(layer, 300, 300, "on") if c["tile"] else layer


def render(c, frame_no):
    n = {k: held(c, k, frame_no) for k in RANGES}
    return frame(optics(layer_of(c), n, c), c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


CASES = {
    "FX-OPTICS-001": ("Field of view 90, horizontal, reverse off, the centre in the middle: a "
                      "fisheye. The middle keeps its size and the stripes bow outward round it, "
                      "squeezed more the farther out they are; the left and right edges read "
                      "from past the drawing, so they and the corners are emptied.",
                      case(), [0]),
    "FX-OPTICS-002": ("Field of view 0, as it starts: the drawing, untouched.",
                      case(field_of_view=0), [0]),
    "FX-OPTICS-003": ("Reverse on: the fisheye taken out, every pixel reading from nearer the "
                      "middle, the stripes stretched outward, the empty right-hand column and top "
                      "and bottom rows filled from inside.", case(reverse="on"), [0]),
    "FX-OPTICS-004": ("Vertical: the 90 degrees span the height, ten pixels, not the width, so "
                      "the squeeze is stronger and more of the drawing is emptied than in "
                      "FX-OPTICS-001.", case(orientation="vertical"), [0]),
    "FX-OPTICS-005": ("Diagonal: the 90 degrees span the diagonal, so the squeeze is gentler and "
                      "less is emptied than in FX-OPTICS-001.", case(orientation="diagonal"), [0]),
    "FX-OPTICS-006": ("Field of view 180, the most, reverse off: only the round part within "
                      "eight pixels of the middle shows, squeezed hard toward its rim; every "
                      "pixel farther out is empty.", case(field_of_view=180), [0]),
    "FX-OPTICS-007": ("Field of view 180, reverse on: the whole picture reads from within eight "
                      "pixels of the middle, stretched hard toward the edges.",
                      case(field_of_view=180, reverse="on"), [0]),
    "FX-OPTICS-008": ("Centre 25, 50: the lens's middle at (4, 5), the left of the drawing kept "
                      "and the right squeezed away.", case(center=(25, 50)), [0]),
    "FX-OPTICS-009": ("Centre 53.125, 55: the middle is the centre of pixel (8, 5) itself, so "
                      "that pixel reads itself and is kept exactly.",
                      case(center=(53.125, 55)), [0]),
    "FX-OPTICS-010": ("Field of view keyed from 0 at frame 0 to 120 at frame 4, linear: frame 0 "
                      "is the drawing, frame 2 is field of view 60 and frame 4 is 120.",
                      case(field_of_view=keyed((0, 0), (4, 120))), [0, 2, 4]),
    "FX-OPTICS-011": ("Centre keyed from 50, 50 at frame 0 to 25, 50 at frame 4, linear: frame "
                      "0 is FX-OPTICS-001, frame 2 is centre 37.5, 50, and frame 4 is "
                      "FX-OPTICS-008.", case(center=keyed((0, (50, 50)), (4, (25, 50)))),
                      [0, 2, 4]),
    "FX-OPTICS-012": ("Field of view eased from 90 at frame 0 to 180 at frame 4 on a curve that "
                      "overshoots: at frame 2 it would pass 180, is held at 180, and is "
                      "FX-OPTICS-006.", case(field_of_view=keyed((0, 90, OVERSHOOT), (4, 180))),
                      [0, 2]),
    "FX-OPTICS-013": ("FX-OPTICS-001 moved three pixels right: the lens moves with the drawing, "
                      "and nothing grows.", case(shift=3), [0]),
    "FX-OPTICS-014": ("After a Motion Tile that grows the layer: the lens's middle and its span "
                      "are still the drawing's own, so the stripes bow as in FX-OPTICS-001, but "
                      "the edges now read the tiles around instead of emptiness.",
                      case(tile=True), [0]),
}

INVALID = {
    "FX-OPTICS-015": ("Field of view -1, below 0.", case(field_of_view=-1)),
    "FX-OPTICS-016": ("Field of view 181, above 180.", case(field_of_view=181)),
    "FX-OPTICS-017": ("Orientation \"sideways\", which is not \"horizontal\", \"vertical\" or "
                      "\"diagonal\".", case(orientation="sideways")),
    "FX-OPTICS-018": ("Reverse \"yes\", which is not \"on\" or \"off\".", case(reverse="yes")),
    "FX-OPTICS-019": ("Centre 50, 1001, past ten heights.", case(center=(50, 1001))),
    "FX-OPTICS-020": ("Field of view keyed to 200 at frame 4, above 180.",
                      case(field_of_view=keyed((0, 90), (4, 200)))),
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
                                       "mirror": "on"}})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.optics_compensation",
                    "enabled": True,
                    "parameters": {k: (c[k] if k in WORDS else setting_json(c[k]))
                                   for k in NAMES}})
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
    (OUT / "expected_optics_compensation.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                           encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    drawn = plain(case())
    everywhere = [(x, y) for y in range(H) for x in range(W)]
    empties = lambda px: sum(p[3] == 0 for p in px)  # noqa: E731

    # The rule's own pieces: "on" undoes "off" at the same settings, the middle reads itself,
    # and a pixel past a quarter turn is empty.
    for fov in (30, 90, 179):
        for o in ORIENTATIONS:
            for x, y in everywhere:
                q = source(fov, "on", o, (50, 50), x + 0.5, y + 0.5)
                back = source(fov, "off", o, (50, 50), *q)
                assert back is not None and math.dist(back, (x + 0.5, y + 0.5)) < 1e-9
    assert source(90, "off", "horizontal", (53.125, 55), 8.5, 5.5) == (8.5, 5.5)
    assert source(180, "off", "horizontal", (50, 50), 16, 5) is None
    assert half_span("diagonal") == math.hypot(16, 10) / 2

    # Every case keeps every channel within its covering; the invalid ones are the drawing.
    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)

    one = c["FX-OPTICS-001"]["0"]
    assert sum(one[i] != drawn[i] for i in range(W * H)) > 100
    assert all(one[at(0, y)][3] == 0 for y in range(H))  # the left edge emptied
    assert one[at(0, 0)] == EMPTY and one[at(15, 9)] == EMPTY
    # The middle keeps its size: the pixel next to it reads within a tenth of a pixel of itself.
    q = source(90, "off", "horizontal", (50, 50), 8.5, 5.5)
    assert math.dist(q, (8.5, 5.5)) < 0.1 * math.dist((8.5, 5.5), (8, 5)) + 0.01
    assert c["FX-OPTICS-002"]["0"] == drawn
    three = c["FX-OPTICS-003"]["0"]
    assert all(drawn[at(15, y)][3] == 0 < three[at(15, y)][3] for y in range(2, 8))
    assert drawn[at(8, 0)][3] == 0 < three[at(8, 0)][3]
    assert empties(c["FX-OPTICS-004"]["0"]) > empties(one) > empties(c["FX-OPTICS-005"]["0"])
    six = c["FX-OPTICS-006"]["0"]
    for x, y in everywhere:
        if math.dist((x + 0.5, y + 0.5), (8, 5)) >= 8:
            assert six[at(x, y)] == EMPTY
    assert six != one and c["FX-OPTICS-007"]["0"] not in (three, drawn)
    eight = c["FX-OPTICS-008"]["0"]
    assert eight != one and all(eight[at(x, 0)] == drawn[at(x, 0)] == EMPTY for x in range(W))
    nine = c["FX-OPTICS-009"]["0"]
    assert nine[at(8, 5)] == drawn[at(8, 5)] and nine != drawn
    ten = c["FX-OPTICS-010"]
    assert ten["0"] == drawn and ten["2"] == render(case(field_of_view=60), 0)
    assert ten["4"] == render(case(field_of_view=120), 0) not in (ten["2"], one)
    eleven = c["FX-OPTICS-011"]
    assert eleven["0"] == one and eleven["4"] == eight
    assert eleven["2"] == render(case(center=(37.5, 50)), 0) not in (one, eight)
    twelve = c["FX-OPTICS-012"]
    assert ease(OVERSHOOT, 0.5) * 90 + 90 > 180
    assert twelve["0"] == one and twelve["2"] == six
    moved = c["FX-OPTICS-013"]["0"]
    assert all(moved[at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(moved[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    tiled = c["FX-OPTICS-014"]["0"]
    assert tiled != one and one[at(0, 5)] == EMPTY and tiled[at(0, 5)][3] > 0
    assert tiled[at(8, 5)] == one[at(8, 5)]  # near the middle, where no tile is read
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == drawn
    print("checked")


if __name__ == "__main__":
    main()
