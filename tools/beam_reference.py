"""Beam, worked a second way.

D-207 adds `core.beam`: a straight beam of light, a laser or an energy shot, drawn along the line
from one point of the layer to another, its colour running from an inside colour along its middle
to an outside colour at its edges, a stretch of it moving from the start to the end as Time goes
from 0 to 100. It is After Effects' Beam in purpose, and this program's own rule. Nothing is
ported. Document 21 is the rule in words; this file is the reference for the numbers document 25
pins against it.

The rule. `start` and `end` are points in per cent of the drawing's own size, S and E in its own
space, the top-left corner (0, 0). With l = length / 100 and a = time / 100 (1 - l), the beam is
the stretch from A = S + a (E - S) to B = S + (a + l) (E - S): at time 0 it begins at the start,
at time 100 it ends at the end. At a point X, t = clamp((X - A) . (B - A) / |B - A|^2, 0, 1) (0
when A = B), the nearest point on the stretch is N = A + t (B - A), its place on the whole line u
= a + t l, d = |X - N|, and the half-thickness there r = (start_thickness + u (end_thickness -
start_thickness)) / 2. With w = max(1, 2 r softness / 100), the covering is c = clamp((min(d + w
/ 2, r) - max(d - w / 2, -r)) / w, 0, 1): the share of a box w wide that a line 2 r thick covers,
so with softness 0 a hard line smoothed over one pixel, and with softness 100 a line whose middle
alone is solid, fading to nothing at twice its thickness. The colour is L = (1 - q) inside + q
outside, both linear, with q = clamp(d / r, 0, 1) (1 when r = 0). With composite "on" the beam
goes over the layer O: O.rgb (1 - c) + L c and O.a (1 - c) + c. With composite "off" the layer is
replaced by the beam alone: L c and c. Everything is worked at the output pixel's centre. Nothing
grows: the beam is drawn inside the layer. A draft scales the two thicknesses as distances.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build writes single precision into its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless the
case says: Lightning Bolt's night, its left half a night sky and its right half empty. The drawing
goes into `Fixtures/beam/media`, the projects into `Fixtures/beam`, and the expected frames into
`Fixtures/beam/expected_beam.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/beam_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
from lightning_bolt_reference import DRAWINGS  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "beam"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"start": (-1000, 1000), "end": (-1000, 1000), "length": (0, 100), "time": (0, 100),
          "start_thickness": (0, 500), "end_thickness": (0, 500), "softness": (0, 100)}
WORDS = ("inside_color", "outside_color", "composite")
NAMES = tuple(RANGES) + WORDS
BLUE = "#3c8cff"


# --- the rule -------------------------------------------------------------------------------

def light(n, X, Y):
    """The covering c and the mix q at the drawing-space point (X, Y)."""
    Sx, Sy = n["start"][0] / 100 * W, n["start"][1] / 100 * H
    Ex, Ey = n["end"][0] / 100 * W, n["end"][1] / 100 * H
    l = n["length"] / 100
    a = n["time"] / 100 * (1 - l)
    Ax, Ay = Sx + a * (Ex - Sx), Sy + a * (Ey - Sy)
    dx, dy = l * (Ex - Sx), l * (Ey - Sy)
    L2 = dx * dx + dy * dy
    t = 0.0 if L2 == 0 else min(1.0, max(0.0, ((X - Ax) * dx + (Y - Ay) * dy) / L2))
    d = math.hypot(X - Ax - t * dx, Y - Ay - t * dy)
    u = a + t * l
    r = (n["start_thickness"] + u * (n["end_thickness"] - n["start_thickness"])) / 2
    w = max(1.0, 2 * r * n["softness"] / 100)
    c = min(1.0, max(0.0, (min(d + w / 2, r) - max(d - w / 2, -r)) / w))
    q = 1.0 if r == 0 else min(1.0, d / r)
    return c, q


def beam(layer, n, inside, outside, composite):
    I = [srgb_to_linear(v / 255) for v in R.hex_color(inside.lower())]
    O = [srgb_to_linear(v / 255) for v in R.hex_color(outside.lower())]
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            p = layer["px"][j * layer["w"] + i]
            c, q = light(n, layer["left"] + i + 0.5, layer["top"] + j + 0.5)
            L = [(1 - q) * I[ch] + q * O[ch] for ch in range(3)]
            keep = 1 - c if composite == "on" else 0.0
            px.append([p[ch] * keep + L[ch] * c for ch in range(3)] + [p[3] * keep + c])
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(start=(25, 45), end=(75, 45), length=100, time=0, start_thickness=4,
         end_thickness=4, softness=0, inside_color="#ffffff", outside_color=BLUE, composite="on",
         shift=0, tile=False):
    return {"drawing": "night", "start": start, "end": end, "length": length, "time": time,
            "start_thickness": start_thickness, "end_thickness": end_thickness,
            "softness": softness, "inside_color": inside_color, "outside_color": outside_color,
            "composite": composite, "shift": shift, "tile": tile}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = value_at(c[k], frame_no)
    if isinstance(v, (list, tuple)):
        return [min(hi, max(lo, e)) for e in v]
    return min(hi, max(lo, v))


def layer_of(c):
    layer = {"px": [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
             "left": 0, "top": 0, "w": W, "h": H}
    return motion_tile(layer, 300, 300, "on") if c["tile"] else layer


def render(c, frame_no):
    n = {k: held(c, k, frame_no) for k in RANGES}
    return frame(beam(layer_of(c), n, c["inside_color"], c["outside_color"], c["composite"]),
                 c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


CASES = {
    "FX-BEAM-001": ("From (25, 45) to (75, 45) per cent, from 4 to 12 pixels across the middle "
                    "of row 4, length 100, time 0, 4 pixels thick at both ends, softness 0, white "
                    "inside, blue #3c8cff outside, composite on: a hard beam over the night "
                    "sky and the empty half, white along row 4 turning blue at its edges, rows 3 "
                    "to 5 wholly covered and rows 2 and 6 half, its ends round.",
                    case(), [0]),
    "FX-BEAM-002": ("Length 25, time 0: the first quarter of the line, from 4 to 6 pixels across.",
                    case(length=25), [0]),
    "FX-BEAM-003": ("Length 25, time 50: the quarter in the middle of the line.",
                    case(length=25, time=50), [0]),
    "FX-BEAM-004": ("Length 25, time 100: the last quarter, from 10 to 12 pixels across.",
                    case(length=25, time=100), [0]),
    "FX-BEAM-005": ("Length 0: a round dot 4 pixels across at the start.", case(length=0), [0]),
    "FX-BEAM-006": ("2 pixels thick at the start and 8 at the end: the beam widens along the "
                    "line.", case(start_thickness=2, end_thickness=8), [0]),
    "FX-BEAM-007": ("Softness 50: the edges fade over two pixels.", case(softness=50), [0]),
    "FX-BEAM-008": ("Softness 100: solid only along its middle, fading out to twice its "
                    "thickness.", case(softness=100), [0]),
    "FX-BEAM-009": ("Composite off: the beam alone, the night sky gone.", case(composite="off"),
                    [0]),
    "FX-BEAM-010": ("Inside #ffe080, a pale yellow, outside #ff3020, a red.",
                    case(inside_color="#ffe080", outside_color="#FF3020"), [0]),
    "FX-BEAM-011": ("Both thicknesses 0: nothing is drawn; the drawing, untouched.",
                    case(start_thickness=0, end_thickness=0), [0]),
    "FX-BEAM-012": ("Both thicknesses 0 with composite off: nothing at all.",
                    case(start_thickness=0, end_thickness=0, composite="off"), [0]),
    "FX-BEAM-013": ("From the top-left corner to the bottom-right, slanted.",
                    case(start=(0, 0), end=(100, 100)), [0]),
    "FX-BEAM-014": ("The start and the end the same point, (50, 50): a round dot there.",
                    case(start=(50, 50), end=(50, 50)), [0]),
    "FX-BEAM-015": ("Length 25, time keyed from 0 at frame 0 to 100 at frame 4, linear: the "
                    "shot travels, frame 0 FX-BEAM-002, frame 2 FX-BEAM-003 and frame 4 "
                    "FX-BEAM-004.", case(length=25, time=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-BEAM-016": ("Start thickness keyed from 4 at frame 0 to 500 at frame 4, eased past its "
                    "end: frame 2 would pass 500, is held at 500, and is start thickness 500.",
                    case(start_thickness=keyed((0, 4, OVERSHOOT), (4, 500))), [0, 2]),
    "FX-BEAM-017": ("FX-BEAM-001 moved three pixels right: the beam moves with the drawing.",
                    case(shift=3), [0]),
    "FX-BEAM-018": ("After a Motion Tile that grows the layer: the points are the drawing's own, "
                    "so the frame is FX-BEAM-001's.", case(tile=True), [0]),
    "FX-BEAM-019": ("Starting past the drawing's left edge, from (-50, 45): the beam comes in "
                    "from outside it.", case(start=(-50, 45)), [0]),
}

INVALID = {
    "FX-BEAM-020": ("Length 101, above 100.", case(length=101)),
    "FX-BEAM-021": ("Time -1, below 0.", case(time=-1)),
    "FX-BEAM-022": ("Start thickness 501, above 500.", case(start_thickness=501)),
    "FX-BEAM-023": ("End thickness -1, below 0.", case(end_thickness=-1)),
    "FX-BEAM-024": ("Softness 101, above 100.", case(softness=101)),
    "FX-BEAM-025": ("A start 1001 per cent across, above 1000.", case(start=(1001, 45))),
    "FX-BEAM-026": ("An inside colour \"#12345\", not six hex digits.",
                    case(inside_color="#12345")),
    "FX-BEAM-027": ("An outside colour \"blue\", a word.", case(outside_color="blue")),
    "FX-BEAM-028": ("Composite \"yes\", which is not \"on\" or \"off\".", case(composite="yes")),
    "FX-BEAM-029": ("Time keyed to 101 at frame 4, above 100.",
                    case(time=keyed((0, 0), (4, 101)))),
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
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.beam",
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
    (OUT / "expected_beam.json").write_text(json.dumps(expected, indent=1) + "\n",
                                            encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    art = plain(case())
    white = [1.0, 1.0, 1.0, 1.0]
    blue = [srgb_to_linear(v / 255) for v in R.hex_color(BLUE)] + [1.0]

    # Every covering in 0..1 and every colour inside its covering.
    for fx, frames in c.items():
        for px in frames.values():
            assert all(0 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3])
                       for p in px), fx

    one = c["FX-BEAM-001"]["0"]
    # White on its line, wholly covered one row either side, half on the rows past, nothing
    # further; the ends round; nothing past them.
    for x in range(4, 12):
        assert near(one[at(x, 4)], white), x
        assert one[at(x, 3)][3] == one[at(x, 5)][3] == 1.0
        assert abs(one[at(x, 2)][3] - 0.5 - 0.5 * art[at(x, 2)][3]) < 1e-12
        assert one[at(x, 1)] == art[at(x, 1)] and one[at(x, 7)] == art[at(x, 7)]
    assert one[at(1, 4)] == art[at(1, 4)] and one[at(15, 4)] == art[at(15, 4)]
    assert one[at(13, 2)][3] < one[at(10, 2)][3]
    # Blue towards the edge: row 2's colour is the blue, over what was there.
    b = one[at(10, 2)]
    assert near(b, [0.5 * blue[k] + 0.5 * art[at(10, 2)][k] for k in range(3)] + [0.5])
    two, three, four = (c[f"FX-BEAM-00{k}"]["0"] for k in (2, 3, 4))
    assert near(two[at(4, 4)], white) and two[at(9, 4)] == art[at(9, 4)]
    assert near(three[at(7, 4)], white) and three[at(4, 4)] == art[at(4, 4)]
    assert three[at(12, 4)] == art[at(12, 4)]
    assert near(four[at(11, 4)], white) and four[at(7, 4)] == art[at(7, 4)]
    dot = c["FX-BEAM-005"]["0"]
    assert dot[at(4, 4)] != art[at(4, 4)] and dot[at(7, 4)] == art[at(7, 4)]
    wide = c["FX-BEAM-006"]["0"]
    changed = lambda x: sum(wide[at(x, y)] != art[at(x, y)] for y in range(H))  # noqa: E731
    assert changed(11) > changed(5)
    for k, fx in ((7, "FX-BEAM-007"), (8, "FX-BEAM-008")):
        soft = c[fx]["0"]
        assert soft != one and near(soft[at(8, 4)], white)
    assert c["FX-BEAM-008"]["0"][at(8, 3)][3] < 1 and c["FX-BEAM-008"]["0"][at(8, 1)][3] > 0
    off = c["FX-BEAM-009"]["0"]
    assert all(off[at(x, y)] == [0.0] * 4 for x in range(W) for y in (0, 1, 8, 9))
    assert all(off[at(x, y)] == one[at(x, y)] for x in range(4, 12) for y in (3, 4, 5))
    assert c["FX-BEAM-010"]["0"] != one
    assert c["FX-BEAM-011"]["0"] == art
    assert all(p == [0.0] * 4 for p in c["FX-BEAM-012"]["0"])
    slant = c["FX-BEAM-013"]["0"]
    assert near(slant[at(5, 3)], white, 0.2) and slant[at(12, 1)] == art[at(12, 1)]
    point = c["FX-BEAM-014"]["0"]
    assert abs(point[at(8, 5)][3] - 1) < 1e-9 and point[at(8, 0)] == art[at(8, 0)]
    fifteen = c["FX-BEAM-015"]
    assert fifteen["0"] == two and fifteen["2"] == three and fifteen["4"] == four
    sixteen = c["FX-BEAM-016"]
    assert ease(OVERSHOOT, 0.5) > 1 and sixteen["0"] == one
    assert sixteen["2"] == render(case(start_thickness=500), 2)
    moved = c["FX-BEAM-017"]["0"]
    assert all(moved[at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert c["FX-BEAM-018"]["0"] == one
    past = c["FX-BEAM-019"]["0"]
    assert near(past[at(0, 4)], white) and past != one
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == art
    print("checked")


if __name__ == "__main__":
    main()
