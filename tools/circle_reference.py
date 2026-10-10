"""Circle, worked a second way.

D-414 adds `core.circle`, After Effects' Circle (Generate): "The Circle effect creates a
customizable solid disk or ring" (Adobe's After Effects user manual, page 514 of the copy on
manualsdir.com). Its settings are After Effects': Center, Radius, Edge (None: "a solid disk";
Edge Radius: "the difference between the Edge Radius property and the Radius property is the
thickness of the ring"; Thickness: "the Thickness property sets the thickness of the ring";
Thickness * Radius: "the product of the Thickness property and the Radius property is the
thickness of the ring"; Thickness & Feather * Radius: the same, and "the product of the Feather
property and the Radius property is the feather of the ring"), the Edge Radius / Thickness value,
Feather Outer Edge and Feather Inner Edge ("the thickness of the feather"), Invert Circle ("inverts
the matte"), Color, Opacity and Blending Mode ("used to combine the shape and the original
layer"). The manual gives no formula; the numbers below are this program's own rule, nothing is
ported.

The rule, at a pixel of the layer's buffer whose centre is X = (x, y) in the drawing's own
pixels (the drawing's top-left corner (0, 0), however far an effect above grew the buffer):

1. C is the centre, in per cent of the drawing's own width and height; R the radius; T the edge
   value; Fo and Fi the outer and inner feathers. The ring runs from Ri out to Ro:
   - none: Ro = R, Ri = 0 (a disk; the inner feather is not used);
   - edge_radius: Ro = max(R, T), Ri = min(R, T);
   - thickness: Ro = R, Ri = max(R - T, 0), the ring inside the radius;
   - thickness_radius: as thickness with the thickness T R / 100 (T read as per cent of the
     radius, since a product of two distances is not a distance);
   - thickness_feather_radius: as thickness_radius, with the feathers Fo R / 100 and Fi R / 100.
2. d = |X - C|. The outer covering is clamp((Ro - d) / max(Fo, 1) + 1/2, 0, 1): a straight ramp
   max(Fo, 1) pixels wide centred on the edge, 1 pixel when there is no feather, a pixel's
   covering of a straight edge. When Ri > 0 it is multiplied by the inner covering
   clamp((d - Ri) / max(Fi, 1) + 1/2, 0, 1). Invert Circle on: the covering is 1 minus that.
3. and 4. As Checkerboard's (`tools/checkerboard_reference.py`, D-413): the pattern
   S = (colour c o, c o) laid on the layer by `blending_mode`, the same eight words.

`center` -1000 to 1000 per cent, keyable, (50, 50) when added; `radius` 0 to 10000 pixels,
keyable, 75; `edge` one of the five words, `none` when added; `edge_thickness` 0 to 10000,
keyable, 10; `feather_outer` and `feather_inner` 0 to 10000, keyable, 0; `invert` `off` (when
added) or `on`; `color` `#rrggbb`, white, read in small letters; `opacity` 0 to 100, keyable, 100;
`blending_mode` `none` when added. The radius is a distance and a draft halves it; the edge value
and the feathers are halved too, except where they are per cent of the radius (edge
thickness_radius and thickness_feather_radius for the edge value, thickness_feather_radius for
the feathers). The layer never grows.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 by 10 holding Gradient's cel, the same size, unmoved unless the
case says. The expected frames are in `Fixtures/circle/expected_circle.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/circle_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import checkerboard_reference as K  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from gradient_reference import DRAWINGS  # noqa: E402

W, H = K.W, K.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "circle"
TOLERANCE = 2e-5  # document 25's default for a filter
EDGES = ("none", "edge_radius", "thickness", "thickness_radius", "thickness_feather_radius")
RANGES = {"center": (-1000, 1000), "radius": (0, 10000), "edge_thickness": (0, 10000),
          "feather_outer": (0, 10000), "feather_inner": (0, 10000), "opacity": (0, 100)}
WORDS = ("edge", "invert", "color", "blending_mode")
NAMES = tuple(RANGES) + WORDS
VIOLET = K.VIOLET


# --- the rule -------------------------------------------------------------------------------

def rings(n, edge):
    r, t, fo, fi = n["radius"], n["edge_thickness"], n["feather_outer"], n["feather_inner"]
    if edge == "none":
        return r, 0.0, fo, fi
    if edge == "edge_radius":
        return max(r, t), min(r, t), fo, fi
    if edge in ("thickness_radius", "thickness_feather_radius"):
        t = t * r / 100
    if edge == "thickness_feather_radius":
        fo, fi = fo * r / 100, fi * r / 100
    return r, max(r - t, 0.0), fo, fi


def circle(n, edge, invert):
    cx, cy = n["center"][0] / 100 * W, n["center"][1] / 100 * H
    ro, ri, fo, fi = rings(n, edge)

    def cover(x, y):
        d = math.hypot(x - cx, y - cy)
        c = min(max((ro - d) / max(fo, 1) + 0.5, 0.0), 1.0)
        if ri > 0:
            c *= min(max((d - ri) / max(fi, 1) + 0.5, 0.0), 1.0)
        return 1 - c if invert == "on" else c
    return cover


# --- the cases ------------------------------------------------------------------------------

def case(center=(50, 50), radius=75, edge="none", edge_thickness=10, feather_outer=0,
         feather_inner=0, invert="off", color="#ffffff", opacity=100, blending_mode="none",
         shift=0, tile=False):
    return {"drawing": "cel", "center": center, "radius": radius, "edge": edge,
            "edge_thickness": edge_thickness, "feather_outer": feather_outer,
            "feather_inner": feather_inner, "invert": invert, "color": color,
            "opacity": opacity, "blending_mode": blending_mode, "shift": shift, "tile": tile}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = value_at(c[k], frame_no)
    if isinstance(v, (list, tuple)):
        return [min(hi, max(lo, e)) for e in v]
    return min(hi, max(lo, v))


def render(c, frame_no):
    n = {k: held(c, k, frame_no) for k in RANGES}
    out = K.generate(K.layer_of(c), circle(n, c["edge"], c["invert"]), c["color"], n["opacity"],
                     c["blending_mode"])
    return frame(out, c["shift"])


def plain(c):
    return frame(K.layer_of(c), c["shift"])


CASES = {
    "FX-CIRCLE-001": ("The settings as they start: centre in the middle, radius 75, edge none, "
                      "white, opacity 100, blending mode none: the disk covers the whole small "
                      "frame, white everywhere, the cel gone.", case(), [0]),
    "FX-CIRCLE-002": ("Radius 4: a white disk 8 across at (8, 5), the pixels on its rim partly "
                      "covered, the rest clear.", case(radius=4), [0]),
    "FX-CIRCLE-003": ("Radius 4, outer feather 4: the rim ramps over 4 pixels, from 2 inside to "
                      "2 outside.", case(radius=4, feather_outer=4), [0]),
    "FX-CIRCLE-004": ("Edge Radius, radius 4, edge radius 2: a ring from 2 to 4.",
                      case(radius=4, edge="edge_radius", edge_thickness=2), [0]),
    "FX-CIRCLE-005": ("Edge Radius, radius 2, edge radius 4: the same ring, the larger of the two "
                      "outside.", case(radius=2, edge="edge_radius", edge_thickness=4), [0]),
    "FX-CIRCLE-006": ("Thickness 2, radius 4: the ring inside the radius, from 2 to 4, "
                      "FX-CIRCLE-004's.", case(radius=4, edge="thickness", edge_thickness=2), [0]),
    "FX-CIRCLE-007": ("Thickness * Radius, thickness 50, radius 4: thickness 50 per cent of the "
                      "radius, 2, FX-CIRCLE-006's.",
                      case(radius=4, edge="thickness_radius", edge_thickness=50), [0]),
    "FX-CIRCLE-008": ("Thickness & Feather * Radius, thickness 50, outer feather 50, inner "
                      "feather 25, radius 4: thickness 2, feathers 2 and 1, as Thickness with "
                      "those.", case(radius=4, edge="thickness_feather_radius", edge_thickness=50,
                                    feather_outer=50, feather_inner=25), [0]),
    "FX-CIRCLE-009": ("Thickness 2, radius 4, inner feather 2: the inner edge soft, the outer "
                      "sharp.", case(radius=4, edge="thickness", edge_thickness=2,
                                     feather_inner=2), [0]),
    "FX-CIRCLE-010": ("Thickness 10, radius 4: thicker than the radius, a whole disk, "
                      "FX-CIRCLE-002's.", case(radius=4, edge="thickness"), [0]),
    "FX-CIRCLE-011": ("Radius 4, Invert Circle on: white everywhere but the disk, clear in it.",
                      case(radius=4, invert="on"), [0]),
    "FX-CIRCLE-012": ("Radius 4, orange #ff8000 at opacity 50: an orange disk at half covering.",
                      case(radius=4, color="#ff8000", opacity=50), [0]),
    "FX-CIRCLE-013": ("Radius 4, normal: a white disk over the cel, the cel round it.",
                      case(radius=4, blending_mode="normal"), [0]),
    "FX-CIRCLE-014": ("Radius 4, violet #6450a0, multiply: the cel darkened in the disk.",
                      case(radius=4, color=VIOLET, blending_mode="multiply"), [0]),
    "FX-CIRCLE-015": ("Radius 4, violet, screen: the cel lightened in the disk.",
                      case(radius=4, color=VIOLET, blending_mode="screen"), [0]),
    "FX-CIRCLE-016": ("Radius 4, violet, add.", case(radius=4, color=VIOLET, blending_mode="add"),
                      [0]),
    "FX-CIRCLE-017": ("Radius 4, violet, overlay.",
                      case(radius=4, color=VIOLET, blending_mode="overlay"), [0]),
    "FX-CIRCLE-018": ("Radius 4, violet, soft light.",
                      case(radius=4, color=VIOLET, blending_mode="soft_light"), [0]),
    "FX-CIRCLE-019": ("Radius 4, stencil alpha: the cel seen only through the disk.",
                      case(radius=4, blending_mode="stencil_alpha"), [0]),
    "FX-CIRCLE-020": ("Radius 4, normal at opacity 0: the cel exactly as it was.",
                      case(radius=4, opacity=0, blending_mode="normal"), [0]),
    "FX-CIRCLE-021": ("Radius keyed from 2 at frame 0 to 6 at frame 4, linear: the disk grows; "
                      "frame 2 is radius 4.", case(radius=keyed((0, 2), (4, 6))), [0, 2, 4]),
    "FX-CIRCLE-022": ("Radius 3, the centre keyed from (50, 50) at frame 0 to (25, 50) at frame "
                      "4, linear: the disk slides left, 1 pixel a frame.",
                      case(radius=3, center=keyed((0, (50, 50)), (4, (25, 50)))), [0, 2, 4]),
    "FX-CIRCLE-023": ("FX-CIRCLE-013 moved three pixels right: the disk moves with the layer.",
                      case(radius=4, blending_mode="normal", shift=3), [0]),
    "FX-CIRCLE-024": ("After a Motion Tile that grows the layer: the centre is the drawing's own, "
                      "so the frame is FX-CIRCLE-002's.", case(radius=4, tile=True), [0]),
    "FX-CIRCLE-025": ("FX-CIRCLE-012 with its colour in capitals, #FF8000: the same.",
                      case(radius=4, color="#FF8000", opacity=50), [0]),
    "FX-CIRCLE-026": ("Radius 10, the centre at (-25, 50) per cent, outside the drawing: the left "
                      "edge of the frame covered, the right clear.",
                      case(radius=10, center=(-25, 50)), [0]),
    "FX-CIRCLE-027": ("Edge none with an inner feather of 3: the inner feather is not used, "
                      "FX-CIRCLE-002's.", case(radius=4, feather_inner=3), [0]),
    "FX-CIRCLE-028": ("Opacity keyed from 100 at frame 0 to 0 at frame 4 past its end by an ease, "
                      "radius 4: held at 0, frame 4 clear everywhere.",
                      case(radius=4, opacity=keyed((0, 100, OVERSHOOT), (4, 0))), [0, 4]),
    "FX-CIRCLE-029": ("Radius 0, edge none: nothing covered, clear everywhere.", case(radius=0),
                      [0]),
}

INVALID = {
    "FX-CIRCLE-030": ("Radius -1, below 0.", case(radius=-1)),
    "FX-CIRCLE-031": ("Radius 10001, above 10000.", case(radius=10001)),
    "FX-CIRCLE-032": ("Edge \"ring\", not one of its five words.", case(edge="ring")),
    "FX-CIRCLE-033": ("Edge thickness -1, below 0.", case(edge_thickness=-1)),
    "FX-CIRCLE-034": ("Outer feather 10001, above 10000.", case(feather_outer=10001)),
    "FX-CIRCLE-035": ("Opacity 101, above 100.", case(opacity=101)),
    "FX-CIRCLE-036": ("Centre 1001 per cent across, above 1000.", case(center=(1001, 50))),
    "FX-CIRCLE-037": ("Invert \"yes\", not off or on.", case(invert="yes")),
    "FX-CIRCLE-038": ("Blending mode \"darken\", which this program does not have.",
                      case(blending_mode="darken")),
    "FX-CIRCLE-039": ("Colour \"#12345\", not six hex digits.", case(color="#12345")),
    "FX-CIRCLE-040": ("Radius keyed to 10001 at frame 4, above 10000.",
                      case(radius=keyed((0, 75), (4, 10001)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = K.project_json(fx, dict(K.case(), shift=c["shift"], tile=c["tile"]))
    effects = p["compositions"][0]["layers"][0]["effects"]
    effects[-1] = {"instance_id": effects[-1]["instance_id"], "type_id": "core.circle",
                   "enabled": True,
                   "parameters": {k: (c[k] if k in WORDS else setting_json(c[k]))
                                  for k in NAMES}}
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
    (OUT / "expected_circle.json").write_text(json.dumps(expected, indent=1) + "\n",
                                              encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    white, clear = [1.0, 1.0, 1.0, 1.0], [0.0, 0.0, 0.0, 0.0]
    art = plain(case())
    shows = [i for i in range(W * H) if art[i][3] > 0]
    assert all(near(p, white) for p in c["FX-CIRCLE-001"]["0"])
    two = c["FX-CIRCLE-002"]["0"]
    for y in range(H):
        for x in range(W):
            d = math.hypot(x + 0.5 - 8, y + 0.5 - 5)
            if d <= 3.5:
                assert near(two[at(x, y)], white), (x, y)
            elif d >= 4.5:
                assert near(two[at(x, y)], clear), (x, y)
            else:
                assert near(two[at(x, y)], [4.5 - d] * 4), (x, y)
    three = c["FX-CIRCLE-003"]["0"]
    for i, (x, y) in enumerate((i % W, i // W) for i in range(W * H)):
        d = math.hypot(x + 0.5 - 8, y + 0.5 - 5)
        assert abs(three[i][3] - min(max((4 - d) / 4 + 0.5, 0), 1)) < 1e-12
    four = c["FX-CIRCLE-004"]["0"]
    assert c["FX-CIRCLE-005"]["0"] == four == c["FX-CIRCLE-006"]["0"] == c["FX-CIRCLE-007"]["0"]
    assert near(four[at(8, 5)], clear) and near(four[at(10, 5)], white)  # d 0.71 and 2.55
    assert c["FX-CIRCLE-008"]["0"] == render(case(radius=4, edge="thickness", edge_thickness=2,
                                                  feather_outer=2, feather_inner=1), 0)
    nine = c["FX-CIRCLE-009"]["0"]
    assert abs(nine[at(9, 6)][3] - (math.hypot(1.5, 1.5) - 2 + 1) / 2) < 1e-12
    assert abs(nine[at(10, 5)][3] - (math.hypot(2.5, 0.5) - 2 + 1) / 2) < 1e-12
    assert near(nine[at(8, 5)], clear) and near(nine[at(13, 5)], clear)
    assert near(nine[at(11, 5)], [4.5 - math.hypot(3.5, 0.5)] * 4)  # the outer edge sharp
    assert c["FX-CIRCLE-010"]["0"] == two == c["FX-CIRCLE-027"]["0"]
    eleven = c["FX-CIRCLE-011"]["0"]
    assert all(near(eleven[i], [1 - two[i][3]] * 4) for i in range(W * H))
    orange = [srgb_to_linear(v / 255) * 0.5 for v in (255, 128, 0)]
    twelve = c["FX-CIRCLE-012"]["0"]
    assert all(near(twelve[i], [v * two[i][3] for v in orange] + [0.5 * two[i][3]])
               for i in range(W * H))
    assert c["FX-CIRCLE-025"]["0"] == twelve
    thirteen = c["FX-CIRCLE-013"]["0"]
    assert all(near(thirteen[i], white if two[i][3] == 1 else art[i])
               for i in range(W * H) if two[i][3] in (0, 1))
    mult, scr = c["FX-CIRCLE-014"]["0"], c["FX-CIRCLE-015"]["0"]
    for i in shows:
        if two[i][3] == 1 and art[i][3] == 1:
            assert all(mult[i][k] <= art[i][k] + 1e-12 for k in range(3))
            assert all(scr[i][k] >= art[i][k] - 1e-12 for k in range(3))
    for fx in ("FX-CIRCLE-014", "FX-CIRCLE-015", "FX-CIRCLE-016", "FX-CIRCLE-017",
               "FX-CIRCLE-018"):
        assert c[fx]["0"] != art and all(near(c[fx]["0"][i], art[i]) for i in range(W * H)
                                         if two[i][3] == 0), fx
    nineteen = c["FX-CIRCLE-019"]["0"]
    assert all(near(nineteen[i], [v * two[i][3] for v in art[i]]) for i in range(W * H))
    assert c["FX-CIRCLE-020"]["0"] == art
    grow = c["FX-CIRCLE-021"]
    assert grow["2"] == two and grow["0"] == render(case(radius=2), 0)
    assert grow["4"] == render(case(radius=6), 0)
    slide = c["FX-CIRCLE-022"]
    assert all(slide["4"][at(x, y)] == slide["0"][at(x + 4, y)] for x in range(W - 4)
               for y in range(H))
    moved = c["FX-CIRCLE-023"]["0"]
    assert all(moved[at(x, y)] == thirteen[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert c["FX-CIRCLE-024"]["0"] == two
    far = c["FX-CIRCLE-026"]["0"]
    assert near(far[at(0, 5)], white) and near(far[at(W - 1, 5)], clear)
    ease = c["FX-CIRCLE-028"]
    assert ease["0"] == two and all(p == clear for p in ease["4"])
    assert all(p == clear for p in c["FX-CIRCLE-029"]["0"])
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == art
    print("checked")


if __name__ == "__main__":
    main()
