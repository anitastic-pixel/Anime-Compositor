"""Drop shadow, worked a second way.

D-115 adds `core.drop_shadow`. It lays a shadow of the drawing's own shape behind it, moved
`distance` pixels in `direction` (degrees clockwise from up, 135 down and to the right), softened
by `softness`, in `color` at `opacity` per cent. The shadow is the drawing's covering, blurred
by document 21's Gaussian at sigma softness / 3, then read back by document 21's bilinear sample
`distance` pixels against the direction, so a shadow moved a whole number of pixels straight
across or down is the covering moved exactly, and one moved slantwise or by a part of a pixel is
spread over the pixels it falls between. It goes behind the drawing: where the drawing covers
fully, the shadow does not show; where it covers part of a pixel, the shadow shows through the
rest; where the drawing is empty, the shadow is all there is. The layer grows by the distance
rounded up and the blur's reach, so a shadow falling past the drawing's edge is kept. It is this
program's own method, modelled on After Effects' Drop Shadow; nothing is ported. Document 21 is
the rule in words; this file is the reference for the numbers document 25 pins against it.

The rule. With s = softness / 3 and r = ceil(3 s), A is the Gaussian blur of the drawing's
covering at s, on the drawing grown by r (A is the covering itself at softness 0). With u the
direction's unit step, (sin, -cos) of it, exactly (0, -1), (1, 0), (0, 1) or (-1, 0) at a whole
quarter turn after reducing mod 360, the shadow's covering at the pixel centre P is
a_s = bilinear(A, P - distance * u) * opacity / 100 and its colour (C * a_s, a_s), C the
colour's linear value. The output is I + shadow * (1 - I.a), I the drawing's pixel, transparent
outside it. The layer grows by ceil(distance) + r on every side, at opacity 0 too.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: a small card drawn below, with room round it for the shadow to show in the
frame. The drawing goes into `Fixtures/drop_shadow/media`, the projects into
`Fixtures/drop_shadow`, and the expected frames into
`Fixtures/drop_shadow/expected_drop_shadow.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/drop_shadow_reference.py
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
from radial_blur_reference import bilinear  # noqa: E402
from edges_reference import gaussian  # noqa: E402
from directional_blur_reference import QUARTERS  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "drop_shadow"
TOLERANCE = 2e-5  # document 25's default for a filter
OPACITY, DIRECTION, DISTANCE, SOFTNESS = (0, 100), (-3600, 3600), (0, 1000), (0, 500)
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def unit(direction):
    """Degrees clockwise from up as a step in pixel space, y down; exact at quarter turns."""
    d = direction % 360
    u = QUARTERS.get(d)
    if u is None:
        a = math.radians(d)
        u = (math.sin(a), -math.cos(a))
    return u


def pixel(layer, x, y):
    """The layer's pixel at (x, y) of layer space, transparent outside it."""
    lx, ly = x - layer["left"], y - layer["top"]
    if 0 <= lx < layer["w"] and 0 <= ly < layer["h"]:
        return layer["px"][ly * layer["w"] + lx]
    return EMPTY


def drop_shadow(layer, color, opacity, direction, distance, softness):
    """The layer with its shadow behind it, grown by ceil(distance) + the blur's reach."""
    s = softness / 3
    r = math.ceil(3 * s)
    covering = dict(layer, px=[[0.0, 0.0, 0.0, p[3]] for p in layer["px"]])
    A = gaussian(covering, s, "transparent")
    g = math.ceil(distance) + r
    ux, uy = unit(direction)
    c = [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]
    k = opacity / 100
    left, top = layer["left"] - g, layer["top"] - g
    w, h = layer["w"] + 2 * g, layer["h"] + 2 * g
    out = []
    for y in range(top, top + h):
        for x in range(left, left + w):
            a = bilinear(A, x + 0.5 - distance * ux, y + 0.5 - distance * uy)[3] * k
            i = pixel(layer, x, y)
            shadow = [c[0] * a, c[1] * a, c[2] * a, a]
            out.append([i[j] + shadow[j] * (1 - i[3]) for j in range(4)])
    return {"px": out, "left": left, "top": top, "w": w, "h": h}


# --- the drawing ----------------------------------------------------------------------------

LINE, SOFT, SKIN, NONE = R.LINE, R.SOFT, R.SKIN, R.NONE


def card(x, y):
    """A box of line in columns 5 to 9 and rows 3 to 6, filled with skin, and the line at half
    covering at (10, 4), its antialiased edge. The rest is empty."""
    if (x, y) == (10, 4):
        return SOFT
    if not (5 <= x <= 9 and 3 <= y <= 6):
        return NONE
    if x in (5, 9) or y in (3, 6):
        return LINE
    return SKIN


DRAWINGS = {"card": [[card(x, y) for x in range(W)] for y in range(H)]}


def drawn_layer(name):
    return {"px": [R.working(p) for row in DRAWINGS[name] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


def frame(layer, shift):
    """The composition's W by H frame: the layer's pixels, grown ones in place, moved right."""
    return [pixel(layer, x - shift, y) for y in range(H) for x in range(W)]


# --- the cases ------------------------------------------------------------------------------

BLUE_HEX = "#2040a0"


def case(color="#000000", opacity=50, direction=135, distance=5, softness=0, shift=0):
    return {"drawing": "card", "color": color, "opacity": opacity, "direction": direction,
            "distance": distance, "softness": softness, "shift": shift}


def held(v, r):
    return min(r[1], max(r[0], v))


def render(c, frame_no):
    at = lambda k, r: held(value_at(c[k], frame_no), r)  # noqa: E731
    return frame(drop_shadow(drawn_layer(c["drawing"]), c["color"], at("opacity", OPACITY),
                             at("direction", DIRECTION), at("distance", DISTANCE),
                             at("softness", SOFTNESS)), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return frame(drawn_layer(c["drawing"]), c["shift"])


CASES = {
    "FX-SHADOW-001": ("The defaults: black at 50 per cent, direction 135, distance 5, softness "
                      "0. The card's shadow falls down and to the right, 3.54 pixels each way, "
                      "spread by the bilinear sample over the pixels it falls between; where "
                      "it lies wholly under the card's shape it is black at half covering. The "
                      "card itself, fully covered, is unchanged, and the shadow's lower part "
                      "falls below the frame.",
                      case(), [0]),
    "FX-SHADOW-002": ("Direction 0, distance 2: the shadow sits exactly two pixels above the "
                      "card, black at half covering in rows 1 and 2; the soft pixel's shadow, "
                      "at (10, 2), is half its covering of 128/255.",
                      case(direction=0, distance=2), [0]),
    "FX-SHADOW-003": ("Direction 90, distance 2: exactly two pixels right. The soft pixel, "
                      "with the card's shadow behind it, takes the shadow through its "
                      "uncovered share: its covering goes from 0.502 to 0.751, its colour "
                      "unchanged, as the shadow is black.",
                      case(direction=90, distance=2), [0]),
    "FX-SHADOW-004": ("Direction 180, distance 2: exactly two pixels below, in rows 7 and 8, "
                      "and the soft pixel's, at (10, 6), is half its covering.",
                      case(direction=180, distance=2), [0]),
    "FX-SHADOW-005": ("Direction 270, distance 2: exactly two pixels left, in columns 3 and "
                      "4.",
                      case(direction=270, distance=2), [0]),
    "FX-SHADOW-006": ("Direction -270 is a quarter turn clockwise from up, the same as 90: "
                      "FX-SHADOW-003 exactly.",
                      case(direction=-270, distance=2), [0]),
    "FX-SHADOW-007": ("Distance 0, softness 0: the shadow sits exactly behind the card, so it "
                      "shows only through the soft pixel, whose covering goes from 0.502 to "
                      "0.627; nothing else changes and the layer does not grow.",
                      case(distance=0), [0]),
    "FX-SHADOW-008": ("Distance 0, softness 3: the shadow, blurred at sigma 1, spreads three "
                      "pixels out from the card on every side, darkest next to it and fading "
                      "outward; column 1 and beyond stay empty.",
                      case(distance=0, softness=3), [0]),
    "FX-SHADOW-009": ("Softness 6 at the default direction and distance: the shadow is "
                      "FX-SHADOW-001's, blurred at sigma 2, so its darkest pixel is lighter "
                      "than half covering and it reaches further.",
                      case(softness=6), [0]),
    "FX-SHADOW-010": ("Opacity 0: no shadow; the frame is the drawing, untouched, though the "
                      "layer still grows.",
                      case(opacity=0), [0]),
    "FX-SHADOW-011": ("Colour #2040a0, opacity 100, distance 2: a blue shadow at full "
                      "covering where it lies wholly under the card's shape, its straight "
                      "colour exactly #2040a0.",
                      case(color=BLUE_HEX, opacity=100, distance=2), [0]),
    "FX-SHADOW-012": ("FX-SHADOW-011 with the colour written in capitals: the same.",
                      case(color=BLUE_HEX.upper(), opacity=100, distance=2), [0]),
    "FX-SHADOW-013": ("Direction 90, distance keyed from 0 at frame 0 to 4 at frame 4, linear: "
                      "frame 0 is FX-SHADOW-007, frame 2 is FX-SHADOW-003, frame 4 is four "
                      "pixels right.",
                      case(direction=90, distance=keyed((0, 0), (4, 4))), [0, 2, 4]),
    "FX-SHADOW-014": ("Distance 2, direction keyed from 0 at frame 0 to 180 at frame 4, "
                      "linear: frame 0 is FX-SHADOW-002, frame 2, at 90, is FX-SHADOW-003, and "
                      "frame 4 is FX-SHADOW-004.",
                      case(distance=2, direction=keyed((0, 0), (4, 180))), [0, 2, 4]),
    "FX-SHADOW-015": ("Opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end "
                      "(132.5 at frame 2): frame 0 has no shadow; frame 2 is held at 100 and is "
                      "frame 4.",
                      case(opacity=keyed((0, 0, OVERSHOOT), (4, 100))), [0, 2, 4]),
    "FX-SHADOW-016": ("FX-SHADOW-001 moved three pixels right: the same, moved, and the "
                      "shadow's right end, which would fall in columns 16 and 17, is cut off by "
                      "the frame's edge.",
                      case(shift=3), [0]),
    "FX-SHADOW-017": ("Direction 0, distance 5: the shadow of the card's two lower rows shows "
                      "in rows 0 and 1; the rest falls above the frame and is cut off, and row "
                      "2 stays empty.",
                      case(direction=0), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-SHADOW-018": ("Opacity 101, above 100.", case(opacity=101)),
    "FX-SHADOW-019": ("Distance -1, below 0.", case(distance=-1)),
    "FX-SHADOW-020": ("Softness 501, above 500.", case(softness=501)),
    "FX-SHADOW-021": ("Direction 3601, above 3600.", case(direction=3601)),
    "FX-SHADOW-022": ("A colour written \"black\".", case(color="black")),
    "FX-SHADOW-023": ("Opacity keyed to 150 at frame 4.",
                      case(opacity=keyed((0, 0), (4, 150)))),
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
        "instance_id": "fx-0-0", "type_id": "core.drop_shadow", "enabled": True,
        "parameters": {k: setting_json(c[k])
                       for k in ("color", "opacity", "direction", "distance", "softness")}}]
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

    (OUT / "expected_drop_shadow.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for p, q in zip(a, b))  # noqa: E731
    alpha = lambda x, y: drawn[at(x, y)][3] if 0 <= x < W and 0 <= y < H else 0.0  # noqa: E731
    soft = (10, 4)
    a = SOFT[3] / 255

    # The rule's own pieces.
    assert unit(90) == (1.0, 0.0) and unit(-270) == (1.0, 0.0) and unit(720) == (0.0, -1.0)
    assert near(unit(135), (math.sqrt(0.5), math.sqrt(0.5)))
    one = drop_shadow(drawn_layer("card"), "#000000", 50, 135, 5, 0)
    assert (one["left"], one["w"]) == (-5, W + 10)
    grown = drop_shadow(drawn_layer("card"), "#000000", 0, 90, 2.5, 6)
    assert (grown["left"], grown["top"], grown["h"]) == (-9, -9, H + 18)
    assert all(p == EMPTY for p in grown["px"][:grown["w"]])  # opacity 0: only empty growth

    for name, frames in c.items():
        for f in frames.values():
            for i, p in enumerate(f):
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (name, p)
                if name != "FX-SHADOW-016" and drawn[i][3] == 1:
                    assert p == drawn[i], name  # the card, fully covered, never changes

    def moved(f, dx, dy, k=0.5):
        """Straight across or down: every empty pixel is the covering moved, times k, black."""
        for y in range(H):
            for x in range(W):
                if drawn[at(x, y)][3] == 0:
                    want = alpha(x - dx, y - dy) * k
                    assert f[at(x, y)] == [0.0, 0.0, 0.0, want], (x, y)
        return True

    first = c["FX-SHADOW-001"]["0"]
    assert near(first[at(12, 8)], [0, 0, 0, 0.5]) and near(first[at(10, 7)], [0, 0, 0, 0.5])
    assert 0 < first[at(8, 7)][3] < 0.5  # a part-pixel at the shadow's edge
    assert first[at(12, 2)] == EMPTY
    assert moved(c["FX-SHADOW-002"]["0"], 0, -2) and c["FX-SHADOW-002"]["0"][at(10, 2)][3] == 0.5 * a
    assert moved(c["FX-SHADOW-003"]["0"], 2, 0) and moved(c["FX-SHADOW-004"]["0"], 0, 2)
    assert moved(c["FX-SHADOW-005"]["0"], -2, 0)
    three = c["FX-SHADOW-003"]["0"]
    assert three[at(*soft)] == drawn[at(*soft)][:3] + [a + (1 - a) * 0.5]
    assert c["FX-SHADOW-006"]["0"] == three
    seven = c["FX-SHADOW-007"]["0"]
    assert [i for i in range(W * H) if seven[i] != drawn[i]] == [at(*soft)]
    assert seven[at(*soft)][3] == a + (1 - a) * a * 0.5 and seven[at(*soft)][:3] == drawn[at(*soft)][:3]
    eight = c["FX-SHADOW-008"]["0"]
    assert eight[at(4, 4)][3] > eight[at(3, 4)][3] > eight[at(2, 4)][3] > 0
    assert eight[at(1, 4)] == EMPTY and eight[at(4, 0)][3] > 0 and eight[at(7, 0)][3] > 0
    nine = c["FX-SHADOW-009"]["0"]
    assert max(p[3] for i, p in enumerate(nine) if drawn[i][3] == 0) < 0.5
    assert nine[at(15, 9)][3] > 0 == first[at(15, 9)][3]
    assert c["FX-SHADOW-010"]["0"] == drawn
    eleven = c["FX-SHADOW-011"]["0"]
    blue = [srgb_to_linear(v / 255) for v in R.hex_color(BLUE_HEX)]
    assert near(eleven[at(10, 7)], blue + [1.0]) and near(eleven[at(8, 7)], blue + [1.0])
    assert c["FX-SHADOW-012"]["0"] == eleven
    thirteen = c["FX-SHADOW-013"]
    assert thirteen["0"] == seven and thirteen["2"] == three and moved(thirteen["4"], 4, 0)
    fourteen = c["FX-SHADOW-014"]
    assert fourteen["0"] == c["FX-SHADOW-002"]["0"] and fourteen["2"] == three
    assert fourteen["4"] == c["FX-SHADOW-004"]["0"]
    fifteen = c["FX-SHADOW-015"]
    assert fifteen["0"] == drawn and fifteen["2"] == fifteen["4"]
    assert near(fifteen["4"][at(12, 8)], [0, 0, 0, 1])
    sixteen = c["FX-SHADOW-016"]["0"]
    assert all(sixteen[at(x, y)] == EMPTY for x in range(3) for y in range(H))
    assert all(sixteen[at(x, y)] == first[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert first[at(13, 8)][3] > 0 and first[at(14, 8)][3] > 0  # the columns that go past 15
    seventeen = c["FX-SHADOW-017"]["0"]
    assert seventeen[at(7, 0)] == [0, 0, 0, 0.5] == seventeen[at(7, 1)]
    assert seventeen[at(7, 2)] == EMPTY and moved(seventeen, 0, -5)
    print("checked")


if __name__ == "__main__":
    main()
