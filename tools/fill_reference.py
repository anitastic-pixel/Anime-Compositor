"""Fill, worked a second way.

D-418 adds `core.fill`, After Effects' Fill (Generate): "The Fill effect fills specified masks with
a specified color" (Adobe's help page on the Generate effects). Its settings are After Effects':
Fill Mask (none, or one of the layer's masks), All Masks, Color, Invert, Horizontal Feather,
Vertical Feather and Opacity. Adobe gives no formula; the numbers below are this program's own
rule and nothing is ported.

The rule. The paths are chosen exactly as Path Stroke chooses them (D-356): the layer's masks at
the frame, each as document 19 flattens it, in the drawing's own space (the top-left corner
(0, 0), however far an effect above grew the buffer). With All Masks on, every mask that is
switched on and has two points or more, whatever its own mode, inversion, feather, expansion or
opacity; with it off, the mask numbered `mask` (1 the first, its floor taken), when it is such a
mask. `mask` 0 is After Effects' None: no mask, and the whole layer is filled. Asked for a mask
and finding none, nothing is filled, the layer is left as it is, and EFFECT_PATH_MISSING is said
every frame.

1. Each path's covering of a pixel is ADR-016's: a 4 by 4 grid of samples, each inside by the
   even-odd rule, the count over sixteen, worked out wherever it is asked for, inside the layer or
   outside it. Several paths are joined as a mask's Add joins them: m = 1 - (1 - m1)(1 - m2)...
   With no mask (`mask` 0, All Masks off) m is 1 everywhere.
2. Feathers: document 21's Gaussian, separable, across with sigma Horizontal Feather / 2 and down
   with sigma Vertical Feather / 2, each cut at ceil(3 sigma) and normalised, on the covering as it
   lies on the plane (a path running off the layer does not fade at the layer's edge). A feather
   0 leaves that direction alone. With no mask there is nothing to feather.
3. Invert makes the covering 1 - m.
4. k = m * Opacity / 100, and the layer's pixel P, premultiplied, becomes
   P (1 - k) + C P.a k, its alpha kept: the colour C (linear light) is laid on what the layer
   already covers, never where it is empty.

`mask` 0 to 1000, keyable, 0 when added; `all_masks` `off` (when added) or `on`; `color` `#rrggbb`,
red when added, read in small letters; `invert` `off` (when added) or `on`; `horizontal_feather`
and `vertical_feather` 0 to 1000 pixels, keyable, 0 when added; `opacity` 0 to 100, keyable, 100
when added. The feathers are distances: a draft halves them; the masks are scaled as they always
are. The layer never grows.

**This file never runs the build's code path.** It samples every pixel's sixteen points against
the outline and blurs tap by tap in double precision on lists, straight from the drawing's 8-bit
values.

Every case is a composition 16 by 10 holding Gradient's cel, the same size, unmoved unless the
case says, with masks of mode None (which leave the drawing as it is) unless the case says. The
expected frames are in `Fixtures/fill/expected_fill.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/fill_reference.py
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
from mask_reference import flatten, points_at, points, mask_json  # noqa: E402
from mask_reference import keyed as moving_path  # noqa: E402

W, H = K.W, K.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "fill"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"mask": (0, 1000), "horizontal_feather": (0, 1000), "vertical_feather": (0, 1000),
          "opacity": (0, 100)}
WORDS = ("all_masks", "color", "invert")
NAMES = ("mask", "all_masks", "color", "invert", "horizontal_feather", "vertical_feather",
         "opacity")
RED = "#ff0000"
BLUE = "#3080ff"
N = 4  # ADR-016's samples a side

BOX = points([2, 2], [13, 2], [13, 7], [2, 7])
SMALL = points([5, 4], [9, 4], [9, 6], [5, 6])           # inside BOX
LEFT = points([1, 1], [7, 1], [7, 6], [1, 6])
RIGHT = points([5, 3], [12, 3], [12, 9], [5, 9])          # overlaps LEFT in (5..7, 3..6)
SLOPE = points([1, 1], [14, 1], [1, 9])                   # a sloped edge, partial coverings
C4 = 0.5522847498307936
RING = points([8, 1], [12, 5], [8, 9], [4, 5], handles=[  # a circle 8 across, curved
    ([-4 * C4, 0], [4 * C4, 0]), ([0, -4 * C4], [0, 4 * C4]),
    ([4 * C4, 0], [-4 * C4, 0]), ([0, 4 * C4], [0, -4 * C4])])
LOW = points([2, 4], [13, 4], [13, 9], [2, 9])            # BOX two rows lower
WIDE = points([-30, -30], [12, -30], [12, 40], [-30, 40])   # far past the left edge
LINE = points([2, 2], [13, 7])                            # two points: encloses nothing


# --- the rule -------------------------------------------------------------------------------

def inside(outline, x, y):
    """ADR-016's even-odd rule: a ray along +x, half-open in y."""
    hit = False
    for i, (x0, y0) in enumerate(outline):
        x1, y1 = outline[(i + 1) % len(outline)]
        if (y0 <= y) != (y1 <= y):
            if x < x0 + (y - y0) / (y1 - y0) * (x1 - x0):
                hit = not hit
    return hit


def sampled(outline, px, py):
    """The pixel whose top-left corner is (px, py), covered by the share of its sixteen samples
    that fall inside."""
    n = sum(inside(outline, px + (i + 0.5) / N, py + (j + 0.5) / N)
            for j in range(N) for i in range(N))
    return n / (N * N)


def joined(outlines, px, py):
    rest = 1.0
    for o in outlines:
        rest *= 1 - sampled(o, px, py)
    return 1 - rest


def weights(feather):
    sigma = feather / 2
    if sigma <= 0:
        return [1.0], 0
    r = math.ceil(3 * sigma)
    w = [math.exp(-(d * d) / (2 * sigma * sigma)) for d in range(-r, r + 1)]
    s = sum(w)
    return [v / s for v in w], r


def covering(outlines, hf, vf):
    """m at each drawing pixel (px, py), feathered, before Invert; None means 1 everywhere."""
    if outlines is None:
        return None
    wx, rx = weights(hf)
    wy, ry = weights(vf)
    memo = {}

    def plain(px, py):
        if (px, py) not in memo:
            memo[(px, py)] = joined(outlines, px, py)
        return memo[(px, py)]

    def m(px, py):
        return sum(wy[j] * sum(wx[i] * plain(px + i - rx, py + j - ry) for i in range(len(wx)))
                   for j in range(len(wy)))
    return m


def fill(layer, outlines, n, color, invert):
    C = [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]
    m = covering(outlines, n["horizontal_feather"], n["vertical_feather"])
    o = n["opacity"] / 100
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            p = layer["px"][j * layer["w"] + i]
            c = 1.0 if m is None else m(layer["left"] + i, layer["top"] + j)
            if invert == "on":
                c = 1 - c
            k = c * o
            px.append([p[ch] * (1 - k) + C[ch] * p[3] * k for ch in range(3)] + [p[3]])
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(masks=None, mask=1, all_masks="off", color=RED, invert="off", horizontal_feather=0,
         vertical_feather=0, opacity=100, shift=0, tile=False):
    return {"drawing": "cel", "masks": [none(BOX)] if masks is None else masks, "mask": mask,
            "all_masks": all_masks, "color": color, "invert": invert,
            "horizontal_feather": horizontal_feather, "vertical_feather": vertical_feather,
            "opacity": opacity, "shift": shift, "tile": tile}


def none(pts, **more):
    return {"points": pts, "mode": "none", **more}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


MISSING_PATH = "missing"


def paths_of(c, frame_no):
    """The chosen outlines; None for the whole layer; MISSING_PATH when a mask was asked for and
    there is none."""
    usable = lambda m, pts: m.get("enabled", True) and len(pts) >= 2  # noqa: E731
    at = [(m, points_at(m, frame_no)) for m in c["masks"]]
    if c["all_masks"] == "on":
        chosen = [(m, pts) for m, pts in at if usable(m, pts)]
    else:
        i = math.floor(held(c, "mask", frame_no)) - 1
        if i < 0:
            return None
        chosen = [at[i]] if i < len(at) and usable(*at[i]) else []
    return [flatten(pts) for _, pts in chosen] or MISSING_PATH


def layer_of(c):
    layer = {"px": [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
             "left": 0, "top": 0, "w": W, "h": H}
    # A mask of mode Add cuts the drawing first (document 21 step 2). The cases' Add masks have
    # whole-number corners on a rectangle, so each pixel is wholly in or out.
    for m in c["masks"]:
        if m.get("mode") == "add":
            xs = [p["point"][0] for p in m["points"]]
            ys = [p["point"][1] for p in m["points"]]
            layer["px"] = [p if min(xs) <= i % W < max(xs) and min(ys) <= i // W < max(ys)
                           else [0.0] * 4 for i, p in enumerate(layer["px"])]
    return motion_tile(layer, 300, 300, "on") if c["tile"] else layer


def render(c, frame_no):
    outlines = paths_of(c, frame_no)
    if outlines == MISSING_PATH:
        return plain(c)
    n = {k: held(c, k, frame_no) for k in RANGES}
    return frame(fill(layer_of(c), outlines, n, c["color"], c["invert"]), c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


TWO = [none(BOX), none(SMALL)]
PAIR = [none(LEFT), none(RIGHT)]

CASES = {
    "FX-FILL-001": ("The settings as they start: Fill Mask 0 (None), red, opacity 100: the whole "
                    "cel red, its line, skin and shadow alike, the soft left column half red "
                    "and half see-through as it was, the empty border still empty.",
                    case(mask=0), [0]),
    "FX-FILL-002": ("Fill Mask 1, a box from (2, 2) to (13, 7) of mode None: red inside the box, "
                    "the cel as it was outside it.", case(), [0]),
    "FX-FILL-003": ("FX-FILL-002 with Invert: red outside the box, the cel inside it.",
                    case(invert="on"), [0]),
    "FX-FILL-004": ("FX-FILL-002 at opacity 50: half way to red inside the box.",
                    case(opacity=50), [0]),
    "FX-FILL-005": ("Horizontal Feather 4: the box's left and right edges soften over about two "
                    "pixels each side, its top and bottom stay sharp.",
                    case(horizontal_feather=4), [0]),
    "FX-FILL-006": ("Vertical Feather 4: the top and bottom soften, the sides stay sharp.",
                    case(vertical_feather=4), [0]),
    "FX-FILL-007": ("Horizontal Feather 3 and Vertical Feather 6: soft both ways, more down than "
                    "across.", case(horizontal_feather=3, vertical_feather=6), [0]),
    "FX-FILL-008": ("Both feathers 4 with Invert: the soft band turned over with the rest.",
                    case(horizontal_feather=4, vertical_feather=4, invert="on"), [0]),
    "FX-FILL-009": ("A curved mask, a circle 8 across about (8, 5): its edge pixels part red, as "
                    "much as of each pixel's sixteen samples falls inside.",
                    case(masks=[none(RING)]), [0]),
    "FX-FILL-010": ("A triangle with a sloped edge from (14, 1) to (1, 9): the pixels it crosses "
                    "part red.", case(masks=[none(SLOPE)]), [0]),
    "FX-FILL-011": ("Two masks, the box and a small box inside it, Fill Mask 2: the small box "
                    "alone.", case(masks=TWO, mask=2), [0]),
    "FX-FILL-012": ("Two overlapping masks, (1, 1) to (7, 6) and (5, 3) to (12, 9), All Masks "
                    "on: both filled, the overlap once.", case(masks=PAIR, all_masks="on"), [0]),
    "FX-FILL-013": ("The same two, All Masks on, both feathers 3: soft edges, the overlap still "
                    "no redder than full.",
                    case(masks=PAIR, all_masks="on", horizontal_feather=3, vertical_feather=3),
                    [0]),
    "FX-FILL-014": ("Two masks, the first switched off, All Masks on: the second alone, "
                    "FX-FILL-012's right box only.",
                    case(masks=[none(LEFT, enabled=False), none(RIGHT)], all_masks="on"), [0]),
    "FX-FILL-015": ("The box's mask of mode Add: the cel is cut to the box first, then filled, "
                    "so the box is red and the rest empty.",
                    case(masks=[{"points": BOX, "mode": "add"}]), [0]),
    "FX-FILL-016": ("Colour #3080ff, a blue.", case(color=BLUE), [0]),
    "FX-FILL-017": ("The same blue written in capitals, #3080FF: the same.",
                    case(color="#3080FF"), [0]),
    "FX-FILL-018": ("The mask's path keyed from the box at frame 0 to the box two rows lower at "
                    "frame 4: the fill follows it, frames 0, 2 and 4.",
                    case(masks=[{**moving_path((0, BOX), (4, LOW)), "mode": "none"}]),
                    [0, 2, 4]),
    "FX-FILL-019": ("Opacity keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 the cel, "
                    "frame 2 half way, frame 4 FX-FILL-002's.",
                    case(opacity=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-FILL-020": ("Horizontal Feather keyed from 0 at frame 0 to 8 at frame 4: sharp, then "
                    "softer.", case(horizontal_feather=keyed((0, 0), (4, 8))), [0, 2, 4]),
    "FX-FILL-021": ("FX-FILL-002 moved three pixels right: the fill moves with the layer.",
                    case(shift=3), [0]),
    "FX-FILL-022": ("After a Motion Tile that grows the layer: the mask is the drawing's own, so "
                    "the frame is FX-FILL-002's.", case(tile=True), [0]),
    "FX-FILL-023": ("Fill Mask 1.5: its floor, mask 1, FX-FILL-002's frame.", case(mask=1.5),
                    [0]),
    "FX-FILL-024": ("Fill Mask 0 with Invert: nothing is filled, the cel as it was.",
                    case(mask=0, invert="on"), [0]),
    "FX-FILL-025": ("Opacity 0: the cel as it was.", case(opacity=0), [0]),
    "FX-FILL-026": ("A mask of two points encloses nothing: the cel as it was. Inverted, "
                    "FX-FILL-027, everything is filled.", case(masks=[none(LINE)]), [0]),
    "FX-FILL-027": ("The two-point mask, inverted: the whole cel red, FX-FILL-001's frame.",
                    case(masks=[none(LINE)], invert="on"), [0]),
    "FX-FILL-028": ("A mask from far left of the layer to x 12, Horizontal Feather 6: the covering "
                    "is worked out past the layer's edge too, so the left edge stays fully red "
                    "and only the edge at x 12 softens.",
                    case(masks=[none(WIDE)], horizontal_feather=6), [0]),
    "FX-FILL-029": ("Opacity keyed from 40 at frame 0 to 100 at frame 4 by an ease that passes "
                    "its end: held at 100 at frame 2, where it would pass it.",
                    case(opacity=keyed((0, 40, OVERSHOOT), (4, 100))), [0, 2, 4]),
    "FX-FILL-030": ("Fill Mask 0 with All Masks on and two masks: All Masks wins, FX-FILL-012's "
                    "frame.", case(masks=PAIR, mask=0, all_masks="on"), [0]),
}

MISSING = {
    "FX-FILL-031": ("Fill Mask 1 with no masks at all.", case(masks=[])),
    "FX-FILL-032": ("Fill Mask 3, of two.", case(masks=TWO, mask=3)),
    "FX-FILL-033": ("Fill Mask 1, switched off.", case(masks=[none(BOX, enabled=False)])),
    "FX-FILL-034": ("All Masks on, every mask switched off.",
                    case(masks=[none(BOX, enabled=False)], all_masks="on")),
    "FX-FILL-035": ("All Masks on with no masks at all.", case(masks=[], all_masks="on")),
}

INVALID = {
    "FX-FILL-036": ("Fill Mask -1, below 0.", case(mask=-1)),
    "FX-FILL-037": ("Fill Mask 1001, above 1000.", case(mask=1001)),
    "FX-FILL-038": ("Horizontal Feather -1, below 0.", case(horizontal_feather=-1)),
    "FX-FILL-039": ("Vertical Feather 1001, above 1000.", case(vertical_feather=1001)),
    "FX-FILL-040": ("Opacity 101, above 100.", case(opacity=101)),
    "FX-FILL-041": ("Colour \"#12345\", not six hex digits.", case(color="#12345")),
    "FX-FILL-042": ("Invert \"yes\", not off or on.", case(invert="yes")),
    "FX-FILL-043": ("All Masks \"maybe\", not off or on.", case(all_masks="maybe")),
    "FX-FILL-044": ("Opacity keyed to 101 at frame 4, above 100.",
                    case(opacity=keyed((0, 100), (4, 101)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    layer = comp["layers"][0]
    t = layer["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    if c["masks"]:
        # In the order the build writes a layer, `masks` where `mask` was.
        layer = {k: v for k, v in layer.items() if k != "mask"}
        at = list(layer).index("matte")
        items = list(layer.items())
        items.insert(at, ("masks", [mask_json(m, i + 1) for i, m in enumerate(c["masks"])]))
        layer = dict(items)
        comp["layers"][0] = layer
    effects = []
    if c["tile"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.motion_tile", "enabled": True,
                        "parameters": {"output_width": 300, "output_height": 300,
                                       "mirror": "on"}})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.fill",
                    "enabled": True,
                    "parameters": {k: (c[k] if k in WORDS else setting_json(c[k]))
                                   for k in NAMES}})
    layer["effects"] = effects
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
    for fx, (says, c) in MISSING.items():
        says += (" Nothing to fill: the layer is drawn without the effect, which is kept as "
                 "written, and EFFECT_PATH_MISSING is said every frame.")
        before = plain(c)
        assert render(c, 0) == before
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "frame_warning": "EFFECT_PATH_MISSING"}
        print(f"{fx}: no path")
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_fill.json").write_text(json.dumps(expected, indent=1) + "\n",
                                            encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    art = plain(case())
    red = [1.0, 0.0, 0.0]
    filled = lambda p, k=1.0: [p[0] * (1 - k) + red[0] * p[3] * k,  # noqa: E731
                               p[1] * (1 - k), p[2] * (1 - k), p[3]]
    in_box = lambda x, y: 2 <= x < 13 and 2 <= y < 7  # noqa: E731

    # Alpha is always kept and every colour stays inside its covering.
    for fx, frames in c.items():
        for px in frames.values():
            if not fx.startswith("FX-FILL-015"):
                assert all(p[3] == a[3] for p, a in zip(px, art)) or fx in ("FX-FILL-021",
                                                                            "FX-FILL-031"), fx
            assert all(all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]) for p in px), fx
    one = c["FX-FILL-001"]["0"]
    assert all(near(one[i], filled(art[i])) for i in range(W * H))
    assert one[at(0, 0)] == [0.0] * 4 and near(one[at(1, 4)], [art[at(1, 4)][3], 0, 0,
                                                               art[at(1, 4)][3]])
    two = c["FX-FILL-002"]["0"]
    for y in range(H):
        for x in range(W):
            want = filled(art[at(x, y)]) if in_box(x, y) else art[at(x, y)]
            assert near(two[at(x, y)], want), (x, y)
    three = c["FX-FILL-003"]["0"]
    for y in range(H):
        for x in range(W):
            want = art[at(x, y)] if in_box(x, y) else filled(art[at(x, y)])
            assert near(three[at(x, y)], want), (x, y)
    four = c["FX-FILL-004"]["0"]
    assert near(four[at(5, 4)], filled(art[at(5, 4)], 0.5))
    five, six = c["FX-FILL-005"]["0"], c["FX-FILL-006"]["0"]
    # Feather 4 across: the box's left edge at x 2 softens, its top edge at y 2 does not.
    assert 0 < five[at(1, 4)][1] < art[at(1, 4)][1] or art[at(1, 4)][1] == 0
    assert five[at(2, 4)] != two[at(2, 4)] and five[at(7, 1)] == art[at(7, 1)]
    assert six[at(7, 1)] != art[at(7, 1)] and near(six[at(2, 4)], two[at(2, 4)], 1e-9) is False \
        or six[at(2, 4)] == two[at(2, 4)]
    assert six[at(7, 4)] != five[at(7, 1)]
    assert c["FX-FILL-007"]["0"] not in (five, six, two)
    eight = c["FX-FILL-008"]["0"]
    assert eight[at(7, 4)] != three[at(7, 4)] or eight[at(7, 4)] == art[at(7, 4)]
    ring = c["FX-FILL-009"]["0"]
    assert near(ring[at(8, 5)], filled(art[at(8, 5)])) and ring[at(1, 5)] == art[at(1, 5)]
    assert any(0 < (art[i][1] - ring[i][1]) < art[i][1] * 0.99 for i in range(W * H))
    slope = c["FX-FILL-010"]["0"]
    assert any(0 < (art[i][1] - slope[i][1]) < art[i][1] * 0.99 for i in range(W * H))
    small = c["FX-FILL-011"]["0"]
    assert small[at(3, 3)] == art[at(3, 3)] and near(small[at(6, 5)], filled(art[at(6, 5)]))
    pair = c["FX-FILL-012"]["0"]
    assert near(pair[at(6, 4)], filled(art[at(6, 4)])) and near(pair[at(2, 2)],
                                                                filled(art[at(2, 2)]))
    assert near(pair[at(10, 7)], filled(art[at(10, 7)])) and pair[at(10, 1)] == art[at(10, 1)]
    soft = c["FX-FILL-013"]["0"]
    assert all(p[0] <= p[3] + 1e-12 for p in soft) and soft != pair
    right = c["FX-FILL-014"]["0"]
    assert right[at(2, 2)] == art[at(2, 2)] and near(right[at(10, 7)], filled(art[at(10, 7)]))
    cut = c["FX-FILL-015"]["0"]
    assert all(near(cut[at(x, y)], filled(art[at(x, y)]) if in_box(x, y) else [0.0] * 4)
               for x in range(W) for y in range(H))
    assert c["FX-FILL-017"]["0"] == c["FX-FILL-016"]["0"] != two
    moving = c["FX-FILL-018"]
    assert moving["0"] == two and moving["4"] != two and moving["2"] not in (two, moving["4"])
    rising = c["FX-FILL-019"]
    assert rising["0"] == art and rising["4"] == two
    assert near(rising["2"][at(5, 4)], filled(art[at(5, 4)], 0.5))
    feather = c["FX-FILL-020"]
    assert feather["0"] == two and feather["2"] != two and feather["4"] != feather["2"]
    moved = c["FX-FILL-021"]["0"]
    assert all(moved[at(x, y)] == two[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert c["FX-FILL-022"]["0"] == two
    assert c["FX-FILL-023"]["0"] == two
    assert c["FX-FILL-024"]["0"] == art and c["FX-FILL-025"]["0"] == art
    assert c["FX-FILL-026"]["0"] == art and c["FX-FILL-027"]["0"] == one
    wide = c["FX-FILL-028"]["0"]
    assert near(wide[at(1, 4)], filled(art[at(1, 4)]), 1e-9)
    assert near(wide[at(2, 4)], filled(art[at(2, 4)]), 1e-9)
    assert wide[at(11, 4)] != filled(art[at(11, 4)]) and wide[at(13, 4)] != art[at(13, 4)]
    eased = c["FX-FILL-029"]
    assert eased["0"] != two and eased["2"] == two and eased["4"] == two
    assert c["FX-FILL-030"]["0"] == pair
    for fx in list(MISSING) + list(INVALID):
        assert c[fx]["0"] == c[fx]["4"] == art
    print("checked")


if __name__ == "__main__":
    main()
