"""Radial Shadow, worked a second way.

D-211 adds `core.radial_shadow`: a shadow of the drawing cast from a point of light onto a
surface behind it, so it spreads out from the light, bigger the farther the surface, where a
Drop Shadow only slides. It is After Effects' Radial Shadow in purpose and names, and this
program's own rule. Nothing is ported. Document 21 is the rule in words; this file is the
reference for the numbers document 25 pins against it.

The rule. `light`, per cent of the drawing's own width and height as Bulge's centre is, is the
light's place L. `distance`, 0 to 1000, is how far the surface lies behind the drawing, and the
shadow is the drawing scaled up about L by k = 1 + distance / 100: a pixel's centre P of the
shadow reads document 21's bilinear sample of the drawing at L + (P - L) / k. The layer grows
across by gx = min(ceil(distance / 100 * max(Lx - left, right - Lx)), its width) and down by
the same with the height, so the scaled drawing is kept but never more than the layer's own
size on each side; then the shadow, all four channels, is blurred by document 21's Gaussian at
sigma softness / 3, which grows it by that blur's reach r on every side. With S the blurred
sample, a = S.a * opacity / 100; with `render` "regular" the shadow is (C * a, a), C the
colour's linear value; with "glass_edge" it takes the drawing's own colours, as light through
stained glass: ((1 - f) * C * S.a + f * S.rgb) * opacity / 100 with f = color_influence / 100,
covering a. The output is the drawing I over the shadow, I + shadow * (1 - I.a), or with
`shadow_only` "on" the shadow alone. At distance 0 the shadow lies exactly under the drawing.
A draft scales the softness, a distance in pixels; the light, a share of the drawing, and the
distance, a ratio, are left as they are.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless the
case says: Drop Shadow's card (`tools/drop_shadow_reference.py`). The drawing goes into
`Fixtures/radial_shadow/media`, the projects into `Fixtures/radial_shadow`, and the expected
frames into `Fixtures/radial_shadow/expected_radial_shadow.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/radial_shadow_reference.py
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
import drop_shadow_reference as D  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "radial_shadow"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"opacity": (0, 100), "light": (-1000, 1000), "distance": (0, 1000),
          "softness": (0, 500), "color_influence": (0, 100)}
WORDS = ("render", "shadow_only")
NAMES = ("color", "opacity", "light", "distance", "softness", "render", "color_influence",
         "shadow_only")
EMPTY = [0.0] * 4


# --- the rule -------------------------------------------------------------------------------

def growth(layer, light, distance):
    """How far the layer grows across and down before the blur: the drawing scaled about the
    light, kept, but never more than the layer's own width or height on each side."""
    lx, ly = light[0] / 100 * W, light[1] / 100 * H
    left, top = layer["left"], layer["top"]
    right, bottom = left + layer["w"], top + layer["h"]
    gx = min(math.ceil(distance / 100 * max(lx - left, right - lx)), layer["w"])
    gy = min(math.ceil(distance / 100 * max(ly - top, bottom - ly)), layer["h"])
    return gx, gy


def radial_shadow(layer, color, n, c):
    lx, ly = n["light"][0] / 100 * W, n["light"][1] / 100 * H
    k = 1 + n["distance"] / 100
    gx, gy = growth(layer, n["light"], n["distance"])
    left, top = layer["left"] - gx, layer["top"] - gy
    w, h = layer["w"] + 2 * gx, layer["h"] + 2 * gy
    cast = []
    for y in range(top, top + h):
        for x in range(left, left + w):
            px, py = x + 0.5, y + 0.5
            cast.append(bilinear(layer, lx + (px - lx) / k, ly + (py - ly) / k))
    shade = gaussian({"px": cast, "left": left, "top": top, "w": w, "h": h},
                     n["softness"] / 3, "transparent")
    col = [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]
    op, f = n["opacity"] / 100, n["color_influence"] / 100
    out = []
    for j in range(shade["h"]):
        for i in range(shade["w"]):
            s = shade["px"][j * shade["w"] + i]
            a = s[3] * op
            if c["render"] == "glass_edge":
                rgb = [((1 - f) * col[m] * s[3] + f * s[m]) * op for m in range(3)]
            else:
                rgb = [col[m] * a for m in range(3)]
            shadow = rgb + [a]
            d = D.pixel(layer, shade["left"] + i, shade["top"] + j)
            out.append(shadow if c["shadow_only"] == "on" else
                       [d[m] + shadow[m] * (1 - d[3]) for m in range(4)])
    return dict(shade, px=out)


# --- the cases ------------------------------------------------------------------------------

BLUE_HEX = "#2040a0"


def case(color="#000000", opacity=50, light=(50, 0), distance=10, softness=0, render="regular",
         color_influence=100, shadow_only="off", shift=0):
    return {"drawing": "card", "color": color, "opacity": opacity, "light": light,
            "distance": distance, "softness": softness, "render": render,
            "color_influence": color_influence, "shadow_only": shadow_only, "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = value_at(c[k], frame_no)
    return [min(hi, max(lo, u)) for u in v] if isinstance(v, (list, tuple)) else min(hi, max(lo, v))


def render(c, frame_no):
    n = {k: held(c, k, frame_no) for k in RANGES}
    color = value_at(c["color"], frame_no)
    return D.frame(radial_shadow(D.drawn_layer(c["drawing"]), color, n, c), c["shift"])


def plain(c):
    return D.frame(D.drawn_layer(c["drawing"]), c["shift"])


CORNER = dict(light=(0, 0), distance=40)

CASES = {
    "FX-RSHADOW-001": ("As it starts: black at 50 per cent, the light at the top of the middle "
                       "(8, 0), distance 10, so the shadow is the card made a tenth bigger "
                       "about the light, peeping out below the card and a little to each side; "
                       "the card itself, fully covered, is unchanged.", case(), [0]),
    "FX-RSHADOW-002": ("Distance 0: the shadow lies exactly under the card, so it shows only "
                       "through the half-covered pixel (10, 4), and the light's place does not "
                       "matter.", case(distance=0), [0]),
    "FX-RSHADOW-003": ("The light in the middle (8, 5), distance 50: the card half as big again "
                       "about the middle, a dark ring round it on every side.",
                       case(light=(50, 50), distance=50), [0]),
    "FX-RSHADOW-004": ("The light at the top-left corner, distance 40: the shadow thrown down "
                       "and to the right, away from the light, and grown by 1.4.",
                       case(**CORNER), [0]),
    "FX-RSHADOW-005": ("FX-RSHADOW-004 in blue #2040a0 at opacity 100: where the shadow lies "
                       "wholly within the card's scaled shape it is exactly that blue, fully "
                       "covered.", case(color=BLUE_HEX, opacity=100, **CORNER), [0]),
    "FX-RSHADOW-006": ("FX-RSHADOW-004 with softness 3: the shadow blurred at sigma 1, fading "
                       "out further, its darkest lighter.", case(softness=3, **CORNER), [0]),
    "FX-RSHADOW-007": ("Render Glass Edge, colour influence 100, opacity 100: the shadow takes "
                       "the card's own colours, skin inside and line round it, as light "
                       "through stained glass, instead of black.",
                       case(render="glass_edge", opacity=100, **CORNER), [0]),
    "FX-RSHADOW-008": ("Glass Edge at colour influence 50: halfway between the card's colours "
                       "and black.",
                       case(render="glass_edge", opacity=100, color_influence=50, **CORNER), [0]),
    "FX-RSHADOW-009": ("Shadow Only on: the card is gone and only its shadow is left, under "
                       "where the card was too.", case(shadow_only="on", **CORNER), [0]),
    "FX-RSHADOW-010": ("Opacity 0: no shadow; the frame is the drawing, untouched, though the "
                       "layer still grows.", case(opacity=0, **CORNER), [0]),
    "FX-RSHADOW-011": ("Distance 40, the light keyed from the top-left corner at frame 0 to the "
                       "top-right at frame 4, linear: frame 0 is FX-RSHADOW-004, frame 2 has "
                       "the light at the top of the middle, and at frame 4 the shadow is thrown "
                       "down and to the left.",
                       case(distance=40, light=keyed((0, (0, 0)), (4, (100, 0)))), [0, 2, 4]),
    "FX-RSHADOW-012": ("The light at the top-left, distance keyed from 0 at frame 0 to 40 at "
                       "frame 4, linear: frame 0 is FX-RSHADOW-002, frame 4 is FX-RSHADOW-004, "
                       "and frame 2 lies between.",
                       case(light=(0, 0), distance=keyed((0, 0), (4, 40))), [0, 2, 4]),
    "FX-RSHADOW-013": ("Opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end: "
                       "frame 2 is held at 100 and is frame 4.",
                       case(opacity=keyed((0, 0, OVERSHOOT), (4, 100)), **CORNER), [0, 2, 4]),
    "FX-RSHADOW-014": ("FX-RSHADOW-004 moved three pixels right: the shadow moves with the "
                       "drawing.", case(shift=3, **CORNER), [0]),
    "FX-RSHADOW-015": ("The light far off to the right (1000, 50), distance 1000: the shadow is "
                       "thrown far to the left, past the most the layer grows, its own width, "
                       "so none of it is kept, and the frame is the drawing.",
                       case(light=(1000, 50), distance=1000), [0]),
}

INVALID = {
    "FX-RSHADOW-016": ("Opacity 101, above 100.", case(opacity=101)),
    "FX-RSHADOW-017": ("Distance -1, below 0.", case(distance=-1)),
    "FX-RSHADOW-018": ("Distance 1001, above 1000.", case(distance=1001)),
    "FX-RSHADOW-019": ("Softness 501, above 500.", case(softness=501)),
    "FX-RSHADOW-020": ("A colour written \"black\".", case(color="black")),
    "FX-RSHADOW-021": ("Render \"glassy\", which is not \"regular\" or \"glass_edge\".",
                       case(render="glassy")),
    "FX-RSHADOW-022": ("Shadow only \"yes\", which is not \"on\" or \"off\".",
                       case(shadow_only="yes")),
    "FX-RSHADOW-023": ("Colour influence 101, above 100.", case(color_influence=101)),
    "FX-RSHADOW-024": ("Light 50, 1001, past ten heights.", case(light=(50, 1001))),
    "FX-RSHADOW-025": ("Distance keyed to 1200 at frame 4, above 1000.",
                       case(distance=keyed((0, 10), (4, 1200)))),
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
        "instance_id": "fx-0-0", "type_id": "core.radial_shadow", "enabled": True,
        "parameters": {k: (c[k] if k in WORDS else setting_json(c[k])) for k in NAMES}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in D.DRAWINGS.items():
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
    (OUT / "expected_radial_shadow.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for p, q in zip(a, b))  # noqa: E731
    drawn = plain(case())
    soft = at(10, 4)
    a = drawn[soft][3]
    layer = D.drawn_layer("card")

    # The growth: kept, but never past the layer's own size.
    assert growth(layer, (50, 0), 10) == (1, 1)
    assert growth(layer, (0, 0), 40) == (7, 4)
    assert growth(layer, (1000, 50), 1000) == (W, H)
    assert growth(layer, (50, 50), 0) == (0, 0)
    grown = radial_shadow(layer, "#000000", dict(opacity=50, light=(0, 0), distance=40,
                                                softness=3, color_influence=100),
                          case())
    assert (grown["left"], grown["top"], grown["w"], grown["h"]) == (-10, -7, W + 20, H + 14)

    for fx, frames in c.items():
        for px in frames.values():
            for i, p in enumerate(px):
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= u <= p[3] + 1e-12 for u in p[:3]), (fx, p)
                if fx not in ("FX-RSHADOW-009", "FX-RSHADOW-014") and drawn[i][3] == 1:
                    assert p == drawn[i], fx  # the card, fully covered, never changes

    one = c["FX-RSHADOW-001"]["0"]
    assert one[at(7, 7)][3] > 0 == drawn[at(7, 7)][3]  # peeping out below
    assert one[at(7, 2)] == EMPTY  # nothing above, toward the light
    two = c["FX-RSHADOW-002"]["0"]
    assert [i for i in range(W * H) if two[i] != drawn[i]] == [soft]
    assert abs(two[soft][3] - (a + (1 - a) * a * 0.5)) < 1e-12
    assert c["FX-RSHADOW-012"]["0"] == two
    three = c["FX-RSHADOW-003"]["0"]
    assert all(three[p][3] > 0 for p in (at(4, 5), at(11, 5), at(7, 2), at(7, 7)))
    four = c["FX-RSHADOW-004"]["0"]
    assert four[at(12, 8)][3] > 0 and four[at(4, 2)] == EMPTY
    blue = [srgb_to_linear(v / 255) for v in R.hex_color(BLUE_HEX)]
    assert near(c["FX-RSHADOW-005"]["0"][at(12, 8)], blue + [1.0])
    six = c["FX-RSHADOW-006"]["0"]
    assert six != four and six[at(15, 9)][3] > 0
    seven = c["FX-RSHADOW-007"]["0"]
    # Glass Edge at 100: the shadow's colour is the card's own, read at the scaled point.
    q = bilinear(layer, 12.5 / 1.4, 8.5 / 1.4)
    assert near(seven[at(12, 8)], q, 1e-12) and seven[at(12, 8)] != four[at(12, 8)]
    eight = c["FX-RSHADOW-008"]["0"]
    assert near(eight[at(12, 8)], [q[m] / 2 for m in range(3)] + [q[3]])
    nine = c["FX-RSHADOW-009"]["0"]
    assert nine[at(7, 4)] != drawn[at(7, 4)] and nine[at(12, 8)] == four[at(12, 8)]
    assert c["FX-RSHADOW-010"]["0"] == drawn
    eleven = c["FX-RSHADOW-011"]
    assert eleven["0"] == four and eleven["2"] == render(case(distance=40), 0)
    assert eleven["4"][at(3, 8)][3] > 0 == four[at(3, 8)][3]
    twelve = c["FX-RSHADOW-012"]
    assert twelve["4"] == four and twelve["2"] not in (two, four)
    thirteen = c["FX-RSHADOW-013"]
    assert thirteen["0"] == drawn and thirteen["2"] == thirteen["4"]
    moved = c["FX-RSHADOW-014"]["0"]
    assert all(moved[at(x, y)] == four[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert c["FX-RSHADOW-015"]["0"] == drawn
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == drawn
    print("checked")


if __name__ == "__main__":
    main()
