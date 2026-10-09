"""Slant, worked a second way.

D-391 adds `core.slant`, after CycoreFX's CC Slant: the picture leant over sideways like a
parallelogram, its floor line held still, optionally squashed or stretched in height and filled
with one colour, the way a cast shadow is made. CycoreFX's notes say what each control does in a
sentence and publish no formula; the rule below is this program's own reading of them, and
nothing is ported.

The rule. The floor line is the level fy = `floor`'s second number per cent of the drawing's
height (only its height counts: the line is level). theta is `slant` in degrees. The height
scale is s = `height` / 100, times cos theta when `stretching` is off, so that with stretching
off a leant edge keeps the length it had standing; with stretching on, the picture keeps its
height and its edges grow. A pixel's centre (x', y') lies v = y' - fy below the floor line, and
reads the drawing at

    (x' + v tan theta, fy + v / s),

document 21's bilinear sample, transparent outside the drawing. A positive slant leans the part
above the floor to the right and the part below it to the left. Height 0 draws nothing. With
`set_color` on, the colour `color` replaces the picture's, kept under its coverage: red, green
and blue are the colour's, linear, times the sample's alpha. The layer does not grow, so what
leans past its edge is cut. There are no distances in pixels, so a draft needs no change.

`slant` -80 to 80 degrees, 0 when added; `stretching` `off` or `on`, off; `height` 0 to 1000 per
cent, 100; `floor` -1000 to 1000 per cent each way, 50, 100 (the bottom edge, so only the top
moves); `set_color` `off` or `on`, off; `color` a colour, #000000. The numbers are keyable. The
values when added are chosen here.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 holding Bulge's striped drawing, the same size,
unmoved unless the case says. The drawing goes into `Fixtures/slant/media`, the projects into
`Fixtures/slant`, and the expected frames into `Fixtures/slant/expected_slant.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/slant_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import prop, srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import bulge_reference as B  # noqa: E402
from radial_blur_reference import bilinear  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "slant"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"slant": (-80, 80), "height": (0, 1000), "floor": (-1000, 1000)}
NAMES = ("slant", "stretching", "height", "floor", "set_color", "color")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def hex_linear(h):
    return [srgb_to_linear(v / 255) for v in R.hex_color(h.lower())]


def seen(layer, s_, x, y):
    if not (0 <= x < layer["w"] and 0 <= y < layer["h"]):
        return EMPTY
    theta = math.radians(s_["slant"])
    s = s_["height"] / 100 * (1 if s_["stretching"] == "on" else math.cos(theta))
    if s <= 0:
        return EMPTY
    fy = s_["floor"][1] / 100 * H
    v = y + 0.5 - fy
    p = bilinear(layer, x + 0.5 + v * math.tan(theta), fy + v / s)
    if s_["set_color"] == "on":
        return [c * p[3] for c in hex_linear(s_["color"])] + [p[3]]
    return p


# --- the cases ------------------------------------------------------------------------------

def case(slant=0, stretching="off", height=100, floor=(50, 100), set_color="off",
         color="#000000", shift=0):
    return {"drawing": "stripes", "slant": slant, "stretching": stretching, "height": height,
            "floor": floor, "set_color": set_color, "color": color, "shift": shift}


def settings(c, frame_no):
    held = {}
    for k in NAMES:
        v = value_at(c[k], frame_no)
        if k in RANGES:
            lo, hi = RANGES[k]
            v = ([min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple))
                 else min(hi, max(lo, v)))
        held[k] = v
    return held


def render(c, frame_no):
    layer = B.drawn_layer(c["drawing"])
    s_ = settings(c, frame_no)
    return [seen(layer, s_, x - c["shift"], y) for y in range(H) for x in range(W)]


def plain(c):
    layer = B.drawn_layer(c["drawing"])
    return [list(layer["px"][y * W + x - c["shift"]]) if 0 <= x - c["shift"] < W else EMPTY
            for y in range(H) for x in range(W)]


ON = {"stretching": "on"}

CASES = {
    "FX-SLANT-001": ("The settings as they start: slant 0, height 100: the drawing, "
                     "untouched.", case(), [0]),
    "FX-SLANT-002": ("Slant 30, stretching off, the floor at the bottom: the top leans right "
                     "and the picture sinks to cos 30 of its height, its leant edges as long "
                     "as they stood.", case(slant=30), [0]),
    "FX-SLANT-003": ("Slant 30, stretching on: the top leans right and the height stays.",
                     case(slant=30, **ON), [0]),
    "FX-SLANT-004": ("Slant -30, stretching on: the top leans left.",
                     case(slant=-30, **ON), [0]),
    "FX-SLANT-005": ("Slant 30, stretching on, the floor through the middle: the top leans "
                     "right and the bottom left.", case(slant=30, floor=(50, 50), **ON), [0]),
    "FX-SLANT-006": ("Slant 30, stretching on, the floor at the top: the top row stays and "
                     "the bottom leans left.", case(slant=30, floor=(50, 0), **ON), [0]),
    "FX-SLANT-007": ("Slant 0, height 50, stretching on: squashed to half height onto the "
                     "floor, the top half transparent.", case(height=50, **ON), [0]),
    "FX-SLANT-008": ("Slant 0, height 200, stretching on, the floor through the middle: "
                     "stretched to twice the height about the middle.",
                     case(height=200, floor=(50, 50), **ON), [0]),
    "FX-SLANT-009": ("Height 0: nothing drawn.", case(height=0), [0]),
    "FX-SLANT-010": ("Slant 45, stretching on, height 50, set color on in red: a red shadow "
                     "lying down to the right, as soft at its edges as the drawing.",
                     case(slant=45, height=50, set_color="on", color="#ff0000", **ON), [0]),
    "FX-SLANT-011": ("Slant keyed from 0 at frame 0 to 60 at frame 4, linear, stretching on: "
                     "frame 0 the drawing, frame 2 FX-SLANT-003.",
                     case(slant=keyed((0, 0), (4, 60)), **ON), [0, 2, 4]),
    "FX-SLANT-012": ("Height eased from 0 at frame 0 to 1000 at frame 4 on a curve that "
                     "overshoots, stretching on: at frame 2 it would pass 1000 and is held "
                     "there.", case(height=keyed((0, 0, OVERSHOOT), (4, 1000)), **ON),
                     [0, 2, 4]),
    "FX-SLANT-013": ("FX-SLANT-003 moved three pixels right: the same, moved; nothing grows.",
                     case(slant=30, shift=3, **ON), [0]),
    "FX-SLANT-014": ("Slant 80, stretching off: leant almost flat, cos 80 of its height, "
                     "most of it past the right edge and cut.", case(slant=80), [0]),
    "FX-SLANT-015": ("Floor keyed from 50, 100 at frame 0 to 50, 0 at frame 4, slant 30, "
                     "stretching on: frame 0 FX-SLANT-003, frame 2 FX-SLANT-005, frame 4 "
                     "FX-SLANT-006.", case(slant=30, floor=keyed((0, (50, 100)), (4, (50, 0))),
                                           **ON), [0, 2, 4]),
}

INVALID = {
    "FX-SLANT-016": ("Slant 81, above 80.", case(slant=81)),
    "FX-SLANT-017": ("Slant -81, below -80.", case(slant=-81)),
    "FX-SLANT-018": ("Height 1001, above 1000.", case(height=1001)),
    "FX-SLANT-019": ("Height -1, below 0.", case(height=-1)),
    "FX-SLANT-020": ("Stretching \"yes\", not a word it takes.", case(stretching="yes")),
    "FX-SLANT-021": ("Set Color \"maybe\", not a word it takes.", case(set_color="maybe")),
    "FX-SLANT-022": ("Color \"black\", not a colour.", case(color="black")),
    "FX-SLANT-023": ("Floor at 50, 1001, past ten heights.", case(floor=(50, 1001))),
    "FX-SLANT-024": ("Slant keyed to 90 at frame 4.", case(slant=keyed((0, 0), (4, 90)))),
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
        "instance_id": "fx-0-0", "type_id": "core.slant", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
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

    (OUT / "expected_slant.json").write_text(json.dumps(expected, indent=1) + "\n",
                                             encoding="utf-8")
    check(expected)


def near(a, b):
    return all(abs(u - v) < 1e-9 for p, q in zip(a, b) for u, v in zip(p, q))


def check(expected):
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
        if "warning" in expected["cases"][fx]:
            assert all(px == drawn for px in frames.values())

    assert near(c["FX-SLANT-001"]["0"], drawn)
    two, three = c["FX-SLANT-002"]["0"], c["FX-SLANT-003"]["0"]
    assert two != three and not near(three, drawn)
    # With the floor at the bottom, the bottom row hardly moves: its centre is half a pixel up.
    assert near([three[at(8, 9)]], [bilinear(B.drawn_layer("stripes"), 8.5 - 0.5 * math.tan(
        math.radians(30)), 9.5)])
    assert two[at(8, 1)] == EMPTY and three[at(8, 1)] != EMPTY
    four = c["FX-SLANT-004"]["0"]
    # Leaning left is leaning right seen in a mirror only for a mirrored drawing, so just: not
    # the same, and the top row moved the other way.
    assert four != three
    five, six = c["FX-SLANT-005"]["0"], c["FX-SLANT-006"]["0"]
    assert five != three != six and near(six[0:W], [bilinear(B.drawn_layer("stripes"),
                                                             x + 0.5 + 0.5 * math.tan(
                                                                 math.radians(30)), 0.5)
                                                    for x in range(W)])
    seven = c["FX-SLANT-007"]["0"]
    assert all(p == EMPTY for p in seven[:at(0, 4)]) and seven[at(8, 9)] != EMPTY
    assert c["FX-SLANT-008"]["0"] != drawn
    assert all(p == EMPTY for p in c["FX-SLANT-009"]["0"])
    ten = c["FX-SLANT-010"]["0"]
    assert any(p != EMPTY for p in ten) and all(p[1] == p[2] == 0 for p in ten)
    eleven = c["FX-SLANT-011"]
    assert near(eleven["0"], drawn) and eleven["2"] == three
    twelve = c["FX-SLANT-012"]
    assert all(p == EMPTY for p in twelve["0"])
    assert twelve["2"] == twelve["4"] == render(case(height=1000, **ON), 0)
    thirteen = c["FX-SLANT-013"]["0"]
    assert all(thirteen[at(x, y)] == three[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(thirteen[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    assert c["FX-SLANT-014"]["0"] != two
    fifteen = c["FX-SLANT-015"]
    assert fifteen["0"] == three and fifteen["2"] == five and fifteen["4"] == six
    print("checked")


if __name__ == "__main__":
    main()
