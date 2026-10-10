"""Grid, worked a second way.

D-417 adds `core.grid`, After Effects' Grid (Generate): it "creates a customizable grid", which
"you can render ... in a solid color or as a mask in the alpha channel of the source layer"
(Adobe's help page on the Generate effects, through its search snippets; the page refused
automated reading). Its settings are After Effects': Anchor, Size From (Corner Point, Width
Slider, Width & Height Sliders, as Checkerboard's), Corner, Width, Height, Border ("the
thickness of the grid lines"; "a value of 0 makes the grid disappear"), Feather Width and
Height, Invert Grid ("inverts the transparent and opaque areas of the grid"), Color, Opacity and
Blending Mode. Adobe gives no formula; the numbers below are this program's own rule and nothing
is ported.

The rule, at a pixel of the layer's buffer whose centre is X = (x, y) in the drawing's own
pixels (the drawing's top-left corner (0, 0), however far an effect above grew the buffer):

1. A is the anchor and K the corner, each in per cent of the drawing's own width and height.
   The cell is w by h pixels exactly as Checkerboard's (D-413): Corner Point, w = |K.x - A.x|
   and h = |K.y - A.y|, each held at least 1; Width Slider, w = h = width; Width & Height
   Sliders, w = width and h = height. The lines run along the cells' edges, x = A.x + k w and
   y = A.y + k h, so the anchor is on a crossing.
2. Along x: u = (x - A.x) / w, t = u - floor(u), the distances to the two nearest upright lines
   d1 = w t and d2 = w (1 - t). A line B = border pixels thick is seen through a box r_x =
   max(feather_width, 1) pixels wide: line(d) = clamp((min(d + r_x / 2, B / 2) -
   max(d - r_x / 2, -B / 2)) / r_x, 0, 1), the share of the box inside the line, so with no
   feather a pixel is covered by exactly how much of it the line crosses, a feather ramps each
   side of the line over r_x pixels, and B = 0 draws nothing. g_x = min(line(d1) + line(d2), 1).
   g_y the same down, with h and feather_height.
3. The covering is c = g_x + g_y - g_x g_y (on an upright line or a level one); Invert Grid
   makes it 1 - c.
4. The pattern is laid exactly as Checkerboard's: S = (C c o, c o), premultiplied, C the colour
   in linear light and o = opacity / 100, then `blending_mode` `none` (the pattern alone, in
   place of the layer), `normal`, `add`, `multiply`, `screen`, `overlay`, `soft_light` or
   `stencil_alpha` (the layer kept only under the grid: the grid as a mask in its alpha), by
   `tools/checkerboard_reference.py`'s `lay`.

`anchor` and `corner` -1000 to 1000 per cent, keyable, (50, 50) and (60, 60) when added;
`size_from` `corner_point` (when added), `width_slider` or `width_and_height_sliders`; `width`
and `height` 1 to 10000 pixels, keyable, 64 when added; `border` 0 to 10000 pixels, keyable, 2
when added; `feather_width` and `feather_height` 0 to 10000 pixels, keyable, 0 when added;
`invert` `off` (when added) or `on`; `color` `#rrggbb`, white when added, read in small letters;
`opacity` 0 to 100, keyable, 100 when added; `blending_mode` as above, `none` when added. Width,
height, border and the feathers are distances: a draft halves them. The layer never grows.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 by 10 holding Gradient's cel, the same size, unmoved unless the
case says. The expected frames are in `Fixtures/grid/expected_grid.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/grid_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import checkerboard_reference as K  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
from gradient_reference import DRAWINGS  # noqa: E402

W, H = K.W, K.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "grid"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"anchor": (-1000, 1000), "corner": (-1000, 1000), "width": (1, 10000),
          "height": (1, 10000), "border": (0, 10000), "feather_width": (0, 10000),
          "feather_height": (0, 10000), "opacity": (0, 100)}
WORDS = ("size_from", "invert", "color", "blending_mode")
NAMES = tuple(RANGES) + WORDS
VIOLET = K.VIOLET


# --- the rule -------------------------------------------------------------------------------

def line(d, border, r):
    return min(max((min(d + r / 2, border / 2) - max(d - r / 2, -border / 2)) / r, 0.0), 1.0)


def axis(x, a, w, border, feather):
    u = (x - a) / w
    t = u - math.floor(u)
    r = max(feather, 1)
    return min(line(w * t, border, r) + line(w * (1 - t), border, r), 1.0)


def grid(n, size_from, invert):
    ax, ay = n["anchor"][0] / 100 * W, n["anchor"][1] / 100 * H
    if size_from == "corner_point":
        kx, ky = n["corner"][0] / 100 * W, n["corner"][1] / 100 * H
        w, h = max(abs(kx - ax), 1), max(abs(ky - ay), 1)
    elif size_from == "width_slider":
        w = h = n["width"]
    else:
        w, h = n["width"], n["height"]

    def cover(x, y):
        gx = axis(x, ax, w, n["border"], n["feather_width"])
        gy = axis(y, ay, h, n["border"], n["feather_height"])
        c = gx + gy - gx * gy
        return 1 - c if invert == "on" else c
    return cover


# --- the cases ------------------------------------------------------------------------------

def case(anchor=(50, 50), size_from="corner_point", corner=(60, 60), width=64, height=64,
         border=2, feather_width=0, feather_height=0, invert="off", color="#ffffff",
         opacity=100, blending_mode="none", shift=0, tile=False):
    return {"drawing": "cel", "anchor": anchor, "size_from": size_from, "corner": corner,
            "width": width, "height": height, "border": border,
            "feather_width": feather_width, "feather_height": feather_height, "invert": invert,
            "color": color, "opacity": opacity, "blending_mode": blending_mode, "shift": shift,
            "tile": tile}


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
    out = K.generate(layer_of(c), grid(n, c["size_from"], c["invert"]), c["color"],
                     n["opacity"], c["blending_mode"])
    return frame(out, c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


# Width 4 with the anchor at (8.5, 5.5) pixels: lines through pixel centres.
MID = (53.125, 55)
WS = "width_slider"

CASES = {
    "FX-GRID-001": ("The settings as they start: anchor in the middle, Corner Point at (60, 60) "
                    "per cent, cells 1.6 by 1 pixels in this small drawing, border 2, white, "
                    "blending mode none: the lines are thicker than the cells, so the whole "
                    "layer is the grid's white and the cel is gone.", case(), [0]),
    "FX-GRID-002": ("Width Slider 4, border 1, the anchor at (8, 5): the lines run along pixel "
                    "edges, so they are two half-covered pixels wide, and three quarters where "
                    "they cross.", case(size_from=WS, width=4, border=1), [0]),
    "FX-GRID-003": ("Width Slider 4, border 1, the anchor at (8.5, 5.5): the lines run through "
                    "pixel centres, so columns 0, 4, 8, 12 and rows 1, 5, 9 are white, the rest "
                    "clear.", case(anchor=MID, size_from=WS, width=4, border=1), [0]),
    "FX-GRID-004": ("Width & Height Sliders 4 by 2, border 1, the anchor at (8, 5).",
                    case(size_from="width_and_height_sliders", width=4, height=2, border=1),
                    [0]),
    "FX-GRID-005": ("Corner Point with the corner at (75, 70) per cent, (12, 7) pixels: cells 4 "
                    "by 2, FX-GRID-004's.", case(corner=(75, 70), border=1), [0]),
    "FX-GRID-006": ("Border 0: no grid at all, the layer clear.",
                    case(anchor=MID, size_from=WS, width=4, border=0), [0]),
    "FX-GRID-007": ("Width 4, border 2, feather width 2: the upright lines ramp over 2 pixels "
                    "each side, the level ones stay sharp.",
                    case(anchor=MID, size_from=WS, width=4, border=2, feather_width=2), [0]),
    "FX-GRID-008": ("Width 5, border 3, both feathers 4: soft lines both ways.",
                    case(anchor=MID, size_from=WS, width=5, border=3, feather_width=4,
                         feather_height=4), [0]),
    "FX-GRID-009": ("FX-GRID-003 with Invert Grid: the cells white, the lines clear.",
                    case(anchor=MID, size_from=WS, width=4, border=1, invert="on"), [0]),
    "FX-GRID-010": ("FX-GRID-003 in orange #ff8000 at opacity 50.",
                    case(anchor=MID, size_from=WS, width=4, border=1, color="#ff8000",
                         opacity=50), [0]),
    "FX-GRID-011": ("FX-GRID-003, normal: white lines over the cel.",
                    case(anchor=MID, size_from=WS, width=4, border=1, blending_mode="normal"),
                    [0]),
    "FX-GRID-012": ("FX-GRID-003 in violet #6450a0, multiply: the cel darkened and tinted on "
                    "the lines, as it was elsewhere.",
                    case(anchor=MID, size_from=WS, width=4, border=1, color=VIOLET,
                         blending_mode="multiply"), [0]),
    "FX-GRID-013": ("The same, screen.",
                    case(anchor=MID, size_from=WS, width=4, border=1, color=VIOLET,
                         blending_mode="screen"), [0]),
    "FX-GRID-014": ("The same, add.",
                    case(anchor=MID, size_from=WS, width=4, border=1, color=VIOLET,
                         blending_mode="add"), [0]),
    "FX-GRID-015": ("The same, overlay.",
                    case(anchor=MID, size_from=WS, width=4, border=1, color=VIOLET,
                         blending_mode="overlay"), [0]),
    "FX-GRID-016": ("The same, soft light.",
                    case(anchor=MID, size_from=WS, width=4, border=1, color=VIOLET,
                         blending_mode="soft_light"), [0]),
    "FX-GRID-017": ("FX-GRID-003, stencil alpha: the grid as a mask, the cel kept only on the "
                    "lines.",
                    case(anchor=MID, size_from=WS, width=4, border=1,
                         blending_mode="stencil_alpha"), [0]),
    "FX-GRID-018": ("FX-GRID-003, normal at opacity 0: the cel exactly as it was.",
                    case(anchor=MID, size_from=WS, width=4, border=1, opacity=0,
                         blending_mode="normal"), [0]),
    "FX-GRID-019": ("Width 4, border 1, the anchor keyed from (53.125, 55) at frame 0 to "
                    "(78.125, 55) at frame 4, linear: the lines slide right 1 pixel a frame; "
                    "frame 4, moved one cell, is frame 0.",
                    case(anchor=keyed((0, MID), (4, (78.125, 55))), size_from=WS, width=4,
                         border=1), [0, 2, 4]),
    "FX-GRID-020": ("Width 4, the border keyed from 0 at frame 0 to 4 at frame 4, linear: frame "
                    "2 is border 2.",
                    case(anchor=MID, size_from=WS, width=4, border=keyed((0, 0), (4, 4))),
                    [0, 2, 4]),
    "FX-GRID-021": ("FX-GRID-011 moved three pixels right: the grid moves with the layer.",
                    case(anchor=MID, size_from=WS, width=4, border=1, blending_mode="normal",
                         shift=3), [0]),
    "FX-GRID-022": ("After a Motion Tile that grows the layer: the anchor is the drawing's own, "
                    "so the frame is FX-GRID-003's.",
                    case(anchor=MID, size_from=WS, width=4, border=1, tile=True), [0]),
    "FX-GRID-023": ("FX-GRID-010 with its colour in capitals, #FF8000: the same.",
                    case(anchor=MID, size_from=WS, width=4, border=1, color="#FF8000",
                         opacity=50), [0]),
    "FX-GRID-024": ("Width 2, border 3: the lines wider than the cells, every pixel white.",
                    case(size_from=WS, width=2, border=3), [0]),
    "FX-GRID-025": ("Opacity keyed from 100 at frame 0 to 0 at frame 4 past its end by an "
                    "ease, FX-GRID-003, none: held at 0, frame 4 is clear everywhere.",
                    case(anchor=MID, size_from=WS, width=4, border=1,
                         opacity=keyed((0, 100, OVERSHOOT), (4, 0))), [0, 4]),
    "FX-GRID-026": ("Invert Grid with stencil alpha: the cel kept only in the cells.",
                    case(anchor=MID, size_from=WS, width=4, border=1, invert="on",
                         blending_mode="stencil_alpha"), [0]),
    "FX-GRID-027": ("Width 3, border 0.5, the anchor at (-100, -100) per cent, outside the "
                    "drawing: thin lines still across the whole layer, each pixel covered by "
                    "the share of it a line crosses.",
                    case(anchor=(-100, -100), size_from=WS, width=3, border=0.5), [0]),
}

INVALID = {
    "FX-GRID-028": ("Width 0, below 1.", case(width=0)),
    "FX-GRID-029": ("Height 10001, above 10000.", case(height=10001)),
    "FX-GRID-030": ("Border -1, below 0.", case(border=-1)),
    "FX-GRID-031": ("Feather height 10001, above 10000.", case(feather_height=10001)),
    "FX-GRID-032": ("Opacity 101, above 100.", case(opacity=101)),
    "FX-GRID-033": ("Anchor 1001 per cent across, above 1000.", case(anchor=(1001, 50))),
    "FX-GRID-034": ("Size From \"corner\", not one of its three words.", case(size_from="corner")),
    "FX-GRID-035": ("Blending mode \"darken\", which this program does not have.",
                    case(blending_mode="darken")),
    "FX-GRID-036": ("Colour \"#12345\", not six hex digits.", case(color="#12345")),
    "FX-GRID-037": ("Invert \"yes\", not off or on.", case(invert="yes")),
    "FX-GRID-038": ("Border keyed to 10001 at frame 4, above 10000.",
                    case(border=keyed((0, 2), (4, 10001)))),
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
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.grid",
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
    (OUT / "expected_grid.json").write_text(json.dumps(expected, indent=1) + "\n",
                                            encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    white, clear = [1.0, 1.0, 1.0, 1.0], [0.0, 0.0, 0.0, 0.0]
    art = plain(case())
    assert all(near(p, white) for p in c["FX-GRID-001"]["0"])
    two = c["FX-GRID-002"]["0"]
    for y in range(H):
        for x in range(W):
            gx = 0.5 if x % 4 in (0, 3) else 0.0
            gy = 0.5 if (y - 1) % 4 in (0, 3) else 0.0
            a = gx + gy - gx * gy
            assert near(two[at(x, y)], [a] * 4), (x, y, two[at(x, y)])
    three = c["FX-GRID-003"]["0"]
    on = lambda x, y: x % 4 == 0 or y % 4 == 1  # noqa: E731
    for y in range(H):
        for x in range(W):
            assert near(three[at(x, y)], white if on(x, y) else clear), (x, y)
    assert c["FX-GRID-005"]["0"] == c["FX-GRID-004"]["0"]
    four = c["FX-GRID-004"]["0"]
    assert near(four[at(0, 0)], [0.75] * 4) and near(four[at(1, 1)], [0.5] * 4)
    assert all(p[3] >= 0.5 for p in four)
    assert all(near(p, clear) for p in c["FX-GRID-006"]["0"])
    # Feather width 2 on a line 2 thick through x 8.5: centres 7.5 and 9.5 half, 6.5 none.
    seven = c["FX-GRID-007"]["0"]
    assert near(seven[at(8, 3)], white) and near(seven[at(7, 3)], [0.5] * 4)
    assert near(seven[at(9, 3)], [0.5] * 4) and near(seven[at(6, 3)], clear)
    assert near(seven[at(6, 5)], white) and near(seven[at(6, 6)], [0.5] * 4)
    eight = c["FX-GRID-008"]["0"]
    assert any(0 < p[3] < 1 for p in eight)
    nine = c["FX-GRID-009"]["0"]
    assert all(near(nine[i], clear if three[i][3] == 1 else white) for i in range(W * H))
    orange = [srgb_to_linear(v / 255) * 0.5 for v in (255, 128, 0)] + [0.5]
    ten = c["FX-GRID-010"]["0"]
    assert all(near(ten[i], orange if three[i][3] == 1 else clear) for i in range(W * H))
    assert c["FX-GRID-023"]["0"] == ten
    eleven = c["FX-GRID-011"]["0"]
    assert all(near(eleven[i], white if three[i][3] == 1 else art[i]) for i in range(W * H))
    mult, scr, add = (c[f"FX-GRID-01{k}"]["0"] for k in (2, 3, 4))
    for i in range(W * H):
        if three[i][3] == 0:
            assert near(mult[i], art[i]) and near(scr[i], art[i]) and near(add[i], art[i])
        elif art[i][3] == 1:
            assert all(mult[i][k] <= art[i][k] + 1e-12 for k in range(3))
            assert all(scr[i][k] >= art[i][k] - 1e-12 for k in range(3))
    for fx in ("FX-GRID-015", "FX-GRID-016"):
        assert c[fx]["0"] != art and all(near(c[fx]["0"][i], art[i]) for i in range(W * H)
                                         if three[i][3] == 0), fx
    seventeen = c["FX-GRID-017"]["0"]
    assert all(near(seventeen[i], art[i] if three[i][3] == 1 else clear) for i in range(W * H))
    assert c["FX-GRID-018"]["0"] == art
    nineteen = c["FX-GRID-019"]
    assert nineteen["0"] == three and nineteen["4"] == three and nineteen["2"] != three
    twenty = c["FX-GRID-020"]
    assert all(near(p, clear) for p in twenty["0"])
    assert twenty["2"] == render(case(anchor=MID, size_from=WS, width=4, border=2), 0)
    moved = c["FX-GRID-021"]["0"]
    assert all(moved[at(x, y)] == eleven[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert c["FX-GRID-022"]["0"] == three
    assert all(near(p, white) for p in c["FX-GRID-024"]["0"])
    frames = c["FX-GRID-025"]
    assert frames["0"] == three and all(p == clear for p in frames["4"])
    twentysix = c["FX-GRID-026"]["0"]
    assert all(near(twentysix[i], clear if three[i][3] == 1 else art[i]) for i in range(W * H))
    thin = c["FX-GRID-027"]["0"]
    assert all(p[3] < 1 for p in thin) and any(p[3] > 0 for p in thin)
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == art
    print("checked")


if __name__ == "__main__":
    main()
