"""Rim light, worked a second way.

D-117 adds `core.rim_light`. It lights the edge of a drawing that faces a light, as a back
light catches the rim of a figure in anime: a pixel is lit when the point `width` pixels from it
toward the light lies off the drawing. Exactly: B is document 21's Gaussian blur of the layer's
covering (alpha) at sigma softness / 3, on the grid grown by its radius and nothing beyond it;
at a pixel that shows, with P its centre and u the unit vector toward the light, `direction`
degrees clockwise from up, rim = clamp(1 - bilinear(B, P + width * u), 0, 1). With b the
pixel's straight linear colour and C the chosen colour's linear value, the lit colour f is C
("normal"), b + C ("add"), 1 - (1 - b)(1 - C) ("screen") or b * C ("multiply"), and the pixel
becomes (b + intensity / 100 * rim * (f - b)) times its own covering, the covering unchanged. A
pixel that does not show is left exactly as it is, and the layer does not grow. Width 0 lights
only where the covering itself is less than full: the soft edges. It is this program's own
method, modelled on After Effects' rim lighting as compositors build it from a shifted matte;
nothing is ported. Document 21 is the rule in words; this file is the reference for the numbers
document 25 pins against it.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: a block of skin with a line along its foot, a soft edge down its left side and a
red dot beside it, drawn below. The drawing goes into `Fixtures/rim_light/media`, the projects
into `Fixtures/rim_light`, and the expected frames into
`Fixtures/rim_light/expected_rim_light.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/rim_light_reference.py
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
import recolor_reference as RC  # noqa: E402
import radial_blur_reference as RB  # noqa: E402
import directional_blur_reference as D  # noqa: E402
import edges_reference as E  # noqa: E402

W, H = RC.W, RC.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "rim_light"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"direction": (-3600, 3600), "width": (0, 100), "softness": (0, 100),
          "intensity": (0, 100)}
NUMBERS = ("direction", "width", "softness", "intensity")


# --- the rule -------------------------------------------------------------------------------

def toward(direction):
    """The unit vector toward the light, degrees clockwise from up, exact at quarter turns."""
    u = D.QUARTERS.get(direction % 360)
    if u is None:
        a = math.radians(direction)
        u = (math.sin(a), -math.cos(a))
    return u


def blend(b, C, mode):
    if mode == "normal":
        return C
    if mode == "add":
        return b + C
    if mode == "screen":
        return 1 - (1 - b) * (1 - C)
    return b * C


def rim_light(layer, color, direction, width, softness, intensity, mode):
    """The layer lit; its rectangle kept."""
    cover = dict(layer, px=[[0.0, 0.0, 0.0, p[3]] for p in layer["px"]])
    B = E.gaussian(cover, softness / 3, "transparent")
    u = toward(direction)
    C = [srgb_to_linear(v / 255) for v in RC.hex_color(color.lower())]
    k = intensity / 100
    out = []
    for i, p in enumerate(layer["px"]):
        a = p[3]
        if a == 0:
            out.append(list(p))
            continue
        x, y = layer["left"] + i % layer["w"], layer["top"] + i // layer["w"]
        s = RB.bilinear(B, x + 0.5 + width * u[0], y + 0.5 + width * u[1])[3]
        rim = min(1.0, max(0.0, 1 - s))
        b = [v / a for v in p[:3]]
        out.append([(b[c] + k * rim * (blend(b[c], C[c], mode) - b[c])) * a for c in range(3)]
                   + [a])
    return dict(layer, px=out)


# --- the drawing ----------------------------------------------------------------------------

SKIN = RC.SKIN                         # #f6d6be
LINE = RC.LINE                         # #1e1a24
SOFT = (246, 214, 190, 128)            # the skin at half covering, the block's soft left edge
DOT = (200, 40, 40, 255)               # a red dot, #c82828
NONE = S.NONE
ORANGE = "#ffb040"


def figure(x, y):
    """A block of skin in columns 3 to 12 and rows 2 to 7, its foot, row 7, a line; a soft edge
    down column 2 beside it; and a red dot alone at (14, 5). The rest is empty."""
    if (x, y) == (14, 5):
        return DOT
    if not 2 <= y <= 7:
        return NONE
    if x == 2:
        return SOFT
    if 3 <= x <= 12:
        return LINE if y == 7 else SKIN
    return NONE


DRAWINGS = {"figure": [[figure(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(color="#ffffff", direction=45, width=3, softness=1, intensity=100, mode="normal",
         shift=0):
    return {"drawing": "figure", "color": color, "direction": direction, "width": width,
            "softness": softness, "intensity": intensity, "blend": mode, "shift": shift}


def settings(c, frame_no):
    held = {}
    for k in NUMBERS:
        lo, hi = RANGES[k]
        held[k] = min(hi, max(lo, value_at(c[k], frame_no)))
    return held


def render(c, frame_no):
    layer = {"px": [RC.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
             "left": 0, "top": 0, "w": W, "h": H}
    s = settings(c, frame_no)
    lit = rim_light(layer, c["color"], s["direction"], s["width"], s["softness"],
                    s["intensity"], c["blend"])
    return RC.frame(lit["px"], c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(intensity=0, shift=c["shift"]), 0)


CASES = {
    "FX-RIM-001": ("The settings as they start: white light from 45 degrees, the upper right, "
                   "width 3, softness 1, intensity 100, normal. The block's top two rows and right "
                   "two columns turn white, with a fainter third; the red dot, alone, turns "
                   "wholly white, and itself shades the line's end at (12, 7), whose point "
                   "toward the light falls beside the dot; the empty pixels stay empty and the "
                   "block's lower left stays skin.",
                   case(), [0]),
    "FX-RIM-002": ("Direction 0, light from straight above: the block's top three rows turn "
                   "white and the fourth takes a trace of it; the soft edge, whose own covering "
                   "is half, is half lit down its whole length.",
                   case(direction=0), [0]),
    "FX-RIM-003": ("Direction 90, light from the right: the block's right three columns turn "
                   "white, all but (11, 5), whose point three pixels to the right is the red "
                   "dot, which shades it; the soft edge on the left is not lit.",
                   case(direction=90), [0]),
    "FX-RIM-004": ("Direction 180, light from below: the line along the block's foot turns "
                   "white, with the two rows of skin above it, and the third row a trace.",
                   case(direction=180), [0]),
    "FX-RIM-005": ("Direction 270, light from the left: the soft edge turns wholly white at its "
                   "half covering, and the block's left two columns beside it; the third, whose "
                   "point three pixels to the left is the soft edge's centre, is half lit.",
                   case(direction=270), [0]),
    "FX-RIM-006": ("Width 0, softness 0: rim = 1 - covering, so only the soft edge, half "
                   "covered, is lit, by 127/255; every fully covered pixel is untouched.",
                   case(width=0, softness=0), [0]),
    "FX-RIM-007": ("Width 1: a rim about a pixel deep on the top and right.",
                   case(width=1), [0]),
    "FX-RIM-008": ("Width 2.5: between widths 1 and 3, the point sampled falling between "
                   "pixels.",
                   case(width=2.5), [0]),
    "FX-RIM-009": ("Softness 0, direction 0: the rim is hard, the top three rows wholly white "
                   "and the rest untouched, where FX-RIM-002, at softness 1, gave the fourth "
                   "row a trace.",
                   case(direction=0, softness=0), [0]),
    "FX-RIM-010": ("Intensity 0: the drawing, untouched.",
                   case(intensity=0), [0]),
    "FX-RIM-011": ("Intensity 50: every pixel halfway between the drawing and FX-RIM-001.",
                   case(intensity=50), [0]),
    "FX-RIM-012": ("Colour #ffb040, an orange, blend normal: FX-RIM-001's rim in orange.",
                   case(color=ORANGE), [0]),
    "FX-RIM-013": ("Colour #ffb040, blend add: the orange added onto the skin, brighter than "
                   "white on its red, past what a screen reaches.",
                   case(color=ORANGE, mode="add"), [0]),
    "FX-RIM-014": ("Colour #ffb040, blend screen: lighter than the skin, never past white.",
                   case(color=ORANGE, mode="screen"), [0]),
    "FX-RIM-015": ("Colour #ffb040, blend multiply: the rim darkens the skin toward orange, and "
                   "red, fully lit, keeps its own red channel.",
                   case(color=ORANGE, mode="multiply"), [0]),
    "FX-RIM-016": ("FX-RIM-013 with the colour written in capitals, #FFB040: the same.",
                   case(color=ORANGE.upper(), mode="add"), [0]),
    "FX-RIM-017": ("Direction keyed from 0 at frame 0 to 180 at frame 4, linear: frame 0 is "
                   "FX-RIM-002, frame 2, at 90, FX-RIM-003, and frame 4 FX-RIM-004.",
                   case(direction=keyed((0, 0), (4, 180))), [0, 2, 4]),
    "FX-RIM-018": ("Width keyed from 0 at frame 0 to 4 at frame 4, linear: the rim deepens, "
                   "width 2 at frame 2.",
                   case(width=keyed((0, 0), (4, 4))), [0, 2, 4]),
    "FX-RIM-019": ("Intensity eased from 0 at frame 0 to 100 at frame 4 on a curve that "
                   "overshoots: at frame 2 it would pass 100, is held at 100, and is "
                   "FX-RIM-001; frame 0 is the drawing.",
                   case(intensity=keyed((0, 0, OVERSHOOT), (4, 100))), [0, 2]),
    "FX-RIM-020": ("Direction 450, a turn and a quarter: FX-RIM-003 exactly.",
                   case(direction=450), [0]),
    "FX-RIM-021": ("FX-RIM-001 moved three pixels right: the same, moved; the layer does not "
                   "grow.",
                   case(shift=3), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-RIM-022": ("Width 101, above 100.", case(width=101)),
    "FX-RIM-023": ("Intensity -1, below 0.", case(intensity=-1)),
    "FX-RIM-024": ("Softness 101, above 100.", case(softness=101)),
    "FX-RIM-025": ("Direction -3601, below -3600.", case(direction=-3601)),
    "FX-RIM-026": ("Width keyed to 150 at frame 4.", case(width=keyed((0, 3), (4, 150)))),
    "FX-RIM-027": ("Blend \"overlay\", which is not a blend.", case(mode="overlay")),
    "FX-RIM-028": ("Colour \"#fff\", written in three digits, not six.", case(color="#fff")),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    t = comp["layers"][0]["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    comp["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.rim_light", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in ("color",) + NUMBERS + ("blend",)}}]
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

    (OUT / "expected_rim_light.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    close = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    near = lambda a, b, e=1e-12: all(close(p, q, e) for p, q in zip(a, b))  # noqa: E731
    white = lambda a: [a, a, a, a]  # noqa: E731
    lit = lambda f, x, y: close(f[at(x, y)], white(drawn[at(x, y)][3]))  # noqa: E731
    same = lambda f, x, y, e=1e-12: close(f[at(x, y)], drawn[at(x, y)], e)  # noqa: E731
    empty = [(x, y) for y in range(H) for x in range(W) if drawn[at(x, y)][3] == 0]
    block = [(x, y) for y in range(2, 8) for x in range(3, 13)]

    # The rule's own pieces.
    assert toward(0) == (0.0, -1.0) and toward(-90) == (-1.0, 0.0) and toward(450) == (1.0, 0.0)
    assert close(toward(45), (math.sqrt(0.5), -math.sqrt(0.5)), 1e-15)
    assert blend(0.2, 0.5, "screen") == 1 - 0.8 * 0.5 and blend(0.2, 0.5, "add") == 0.7
    assert blend(0.2, 0.5, "multiply") == 0.1 and blend(0.2, 0.5, "normal") == 0.5

    # Every lit frame leaves the empty pixels exactly as they are and keeps every covering.
    for name, frames in c.items():
        for f in frames.values():
            assert all(f[at(x, y)] == drawn[at(x, y)] if name != "FX-RIM-021" else True
                       for x, y in empty), name
            assert name == "FX-RIM-021" or all(f[i][3] == drawn[i][3] for i in range(W * H))

    one = c["FX-RIM-001"]["0"]
    assert lit(one, 12, 2) and lit(one, 14, 5) and same(one, 4, 6, 1e-9) and same(one, 3, 7, 1e-9)
    assert one[at(12, 5)][2] > one[at(8, 5)][2] > drawn[at(8, 5)][2] - 1e-12
    assert same(one, 2, 6, 1e-9)  # the soft edge, lit from the far side, is barely touched
    rise = lambda f, x, y: f[at(x, y)][2] - drawn[at(x, y)][2]  # noqa: E731
    assert rise(one, 12, 7) < rise(one, 11, 7) / 2  # the dot shades the line's end
    assert rise(one, 11, 3) > 0.47 and 0.05 < rise(one, 5, 4) < 0.1  # a fainter third row
    two = c["FX-RIM-002"]["0"]
    for x in range(3, 13):
        assert all(two[at(x, y)][2] > 0.98 for y in (2, 3, 4))
        assert 0 < two[at(x, 5)][2] - drawn[at(x, 5)][2] < 0.05
    for x in range(4, 12):
        assert same(two, x, 6, 1e-9) and same(two, x, 7, 1e-9)
    # Half lit: the rise is about half of what full light would give.
    half = lambda f, x, y: 0.4 < rise(f, x, y) / (drawn[at(x, y)][3] - drawn[at(x, y)][2]) < 0.6  # noqa: E731,E501
    assert all(half(two, 2, y) for y in range(5, 8))
    three = c["FX-RIM-003"]["0"]
    assert all(three[at(x, y)][2] > 0.98 for x in (10, 11, 12) for y in range(2, 8)
               if (x, y) != (11, 5))
    assert rise(three, 11, 5) < 0.05
    assert same(three, 2, 4, 1e-9) and same(three, 5, 4, 1e-9)
    four = c["FX-RIM-004"]["0"]
    assert all(four[at(x, y)][2] > 0.98 for x in range(3, 13) for y in (5, 6, 7))
    assert same(four, 6, 3, 1e-9) and 0 < rise(four, 6, 4) < 0.05
    five = c["FX-RIM-005"]["0"]
    assert all(lit(five, 2, y) for y in range(2, 8))
    assert all(five[at(x, 4)][2] > 0.98 for x in (3, 4)) and half(five, 5, 4)
    six = c["FX-RIM-006"]["0"]
    for y in range(2, 8):
        a = 128 / 255
        want = [(v / a + (127 / 255) * (1 - v / a)) * a for v in drawn[at(2, y)][:3]] + [a]
        assert close(six[at(2, y)], want)
    assert all(six[at(x, y)] == drawn[at(x, y)] for x, y in block + [(14, 5)])
    seven, eight = c["FX-RIM-007"]["0"], c["FX-RIM-008"]["0"]
    # The corner's lower-left neighbour two in: lit a little at width 1, more at 2.5, more at 3.
    assert one[at(10, 4)][2] > eight[at(10, 4)][2] > seven[at(10, 4)][2]
    nine = c["FX-RIM-009"]["0"]
    for x in range(3, 13):
        assert all(lit(nine, x, y) for y in (2, 3, 4))
        assert all(nine[at(x, y)] == drawn[at(x, y)] for y in (5, 6, 7))
    assert c["FX-RIM-010"]["0"] == drawn
    eleven = c["FX-RIM-011"]["0"]
    assert near(eleven, [[(p + q) / 2 for p, q in zip(u, v)] for u, v in zip(drawn, one)])
    orange = [srgb_to_linear(v / 255) for v in RC.hex_color(ORANGE)]
    normal, add, screen, multiply = (c[f"FX-RIM-0{n}"]["0"] for n in (12, 13, 14, 15))
    assert close(normal[at(14, 5)], orange + [1])
    dot = drawn[at(14, 5)]
    assert close(add[at(14, 5)], [dot[i] + orange[i] for i in range(3)] + [1])
    assert close(screen[at(14, 5)], [1 - (1 - dot[i]) * (1 - orange[i]) for i in range(3)] + [1])
    assert close(multiply[at(14, 5)], [dot[i] * orange[i] for i in range(3)] + [1])
    assert multiply[at(14, 5)][0] == dot[0] * 1.0
    assert add[at(12, 2)][0] > 1 and all(v <= 1 + 1e-12 for p in screen for v in p)
    assert multiply[at(12, 2)][1] < drawn[at(12, 2)][1]
    assert c["FX-RIM-016"]["0"] == add
    seventeen = c["FX-RIM-017"]
    assert seventeen["0"] == two and seventeen["2"] == three and seventeen["4"] == four
    eighteen = c["FX-RIM-018"]
    assert eighteen["2"] == render(case(width=2), 0)
    assert near(eighteen["0"], render(case(width=0), 0)) and eighteen["4"] == render(case(width=4), 0)
    assert eighteen["0"] != six  # softness 1 gives the block's border a trace at width 0
    nineteen = c["FX-RIM-019"]
    assert ease(OVERSHOOT, 0.5) > 1 and nineteen["0"] == drawn and nineteen["2"] == one
    assert c["FX-RIM-020"]["0"] == three
    moved = c["FX-RIM-021"]
    assert moved["0"] == moved["3"]
    assert all(moved["0"][at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert all(moved["0"][at(x, y)] == [0.0] * 4 for x in range(3) for y in range(H))
    print("checked")


if __name__ == "__main__":
    main()
