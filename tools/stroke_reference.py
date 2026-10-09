"""Path Stroke, worked a second way.

D-356 adds `core.stroke`: a brush drawn along a layer's mask, a write-on where Start and End
trim it, after After Effects' Stroke (Generate). It is the first effect on P0-22's hook, effects
that draw along a path. It is After Effects' Stroke in purpose, and this program's own rule.
Nothing is ported. Document 21 is the rule in words; this file is the reference for the numbers
document 25 pins against it.

The rule. The paths are the layer's masks at the frame, each as document 19 flattens it (always
closed: the last point joins the first), in the drawing's own space, the top-left corner (0, 0).
With All Masks off the path is the mask numbered `mask` (1 the first, its floor taken); with it
on, every mask in the layer's order. A mask is a path when it is switched on and has two points
or more, whatever its mode, inversion, feather or expansion; with none, nothing is drawn, the
layer is left as it is, and EFFECT_PATH_MISSING is said every frame.

Each path is measured along its length, L, from its first point. Start and End are per cent,
taken smaller first: s = min, e = max. With Stroke Sequentially off, each path draws from s L to
e L of its own length; with it on (and All Masks on), the paths are one length T, end to end in
order, and a path whose own length begins G along it draws from max(s T - G, 0) to min(e T - G,
L). A path draws nothing when what it draws from is not less than what it draws to.

The brush is laid as dabs, round, each Brush Size across, at every k step along each path, k =
0, 1, 2 ..., where step = Spacing / 100 Brush Size, those from the first place it draws to the
last, both ends included: so the dabs stay where they are as Start moves. With Spacing 0 the
brush is laid all along, from the first place to the last. At a pixel, d is the distance from
its centre to the nearest dab (with Spacing 0, to the nearest point of what is drawn). With r =
Brush Size / 2, w = max(r (1 - Brush Hardness / 100), 1) and t = clamp((r + 0.5 - d) / w, 0,
1), the covering is c = Opacity / 100 t^2 (3 - 2 t): soft over w, smoothed over one pixel even
when hard. Dabs do not build up: the nearest one is the one that counts. Brush Size 0 draws
nothing.

Paint Style. On Original Image: the colour C, linear, over the layer O: O.rgb (1 - c) + C c and
O.a (1 - c) + c. On Transparent: the stroke alone, C c and c. Reveal Original Image: the layer
only where the stroke is, O c, the colour unused. Nothing grows: the stroke is drawn inside the
layer. A draft scales Brush Size as a distance; the masks are scaled as they always are.

**This file never runs the build's code path.** It lists every dab and works in double precision
on lists, straight from the drawing's 8-bit values, where the build finds the nearest dab on
each straight piece of the path and writes single precision into its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless the
case says: Lightning Bolt's night, its left half a night sky and its right half empty, with masks
of mode None (which leave the drawing as it is) unless the case says. The drawing goes into
`Fixtures/stroke/media`, the projects into `Fixtures/stroke`, and the expected frames into
`Fixtures/stroke/expected_stroke.json`.

D-357 (P0-22, second part) adds a shape layer's paths as a source: `source` "shapes" takes the
paths from the layer's own shapes instead of its masks, each as it is at the frame and flattened
as D-78 flattens it, open or closed as the shape says; an open path is measured without the piece
from its last point back to its first, so it is drawn open. With `source` "shapes", Path and All
Masks pick shapes as they pick masks; a shape is a path when it is on and has two points or more,
whatever its fill and stroke. On a layer that is not a shape layer, or with no such shape,
nothing is drawn and EFFECT_PATH_MISSING is said every frame. `source` "masks", the default, is
D-356 unchanged, and a file that does not say is "masks". Those cases are FX-STROKE-047 to 061, a
shape layer the composition's size (16 by 10), into `Fixtures/stroke/expected_stroke_shapes.json`
so FX-STROKE-001 to 046 stay as written.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/stroke_reference.py
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
from mask_reference import flatten, points_at, points, mask_json  # noqa: E402
from mask_reference import keyed as moving_path  # noqa: E402
import shape_reference as SH  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "stroke"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"mask": (1, 1000), "brush_size": (0, 200), "brush_hardness": (0, 100),
          "opacity": (0, 100), "start": (0, 100), "end": (0, 100), "spacing": (0, 100)}
WORDS = ("color", "paint_style", "all_masks", "stroke_sequentially")
NAMES = ("mask", "all_masks", "stroke_sequentially", "color", "brush_size", "brush_hardness",
         "opacity", "start", "end", "spacing", "paint_style")
RED = "#ff3020"

BOX = points([2, 2], [13, 2], [13, 7], [2, 7])        # 32 long: 11 across, 5 down, 11, 5
SMALL = points([5, 4], [9, 4], [9, 6], [5, 6])        # 12 long, inside BOX
K = 0.5522847498307936
RING = points([8, 1], [12, 5], [8, 9], [4, 5], handles=[  # a circle 8 across, curved
    ([-4 * K, 0], [4 * K, 0]), ([0, -4 * K], [0, 4 * K]),
    ([4 * K, 0], [-4 * K, 0]), ([0, 4 * K], [0, -4 * K])])
LOW = points([2, 4], [13, 4], [13, 8], [2, 8])         # BOX two rows lower


# --- the rule -------------------------------------------------------------------------------

def measured(outline, closed=True):
    """The outline's straight pieces, each with where along the path it begins. D-357: an open
    path (a shape's) has no piece from its last point back to its first."""
    pieces, at = [], 0.0
    for i, a in enumerate(outline if closed else outline[:-1]):
        b = outline[(i + 1) % len(outline)]
        n = math.hypot(b[0] - a[0], b[1] - a[1])
        pieces.append((a, b, at, n))
        at += n
    return pieces, at


def place(pieces, u):
    """The point u along the path."""
    for a, b, at, n in pieces:
        if n > 0 and u <= at + n:
            f = min(1.0, max(0.0, (u - at) / n))
            return (a[0] + f * (b[0] - a[0]), a[1] + f * (b[1] - a[1]))
    a, b, at, n = [p for p in pieces if p[3] > 0][-1]
    return b


def windows(paths, start, end, sequential):
    """Each path's pieces with the stretch of it drawn, (from, to), or None."""
    s, e = min(start, end) / 100, max(start, end) / 100
    out, g = [], 0.0
    measures = [measured(p, closed) for p, closed in paths]
    total = sum(L for _, L in measures)
    for pieces, L in measures:
        if sequential:
            lo, hi = max(s * total - g, 0.0), min(e * total - g, L)
        else:
            lo, hi = s * L, e * L
        g += L
        out.append((pieces, (lo, hi) if lo < hi else None))
    return out


def seg_distance(a, b, x, y):
    dx, dy = b[0] - a[0], b[1] - a[1]
    l2 = dx * dx + dy * dy
    t = 0.0 if l2 == 0 else min(1.0, max(0.0, ((x - a[0]) * dx + (y - a[1]) * dy) / l2))
    return math.hypot(x - a[0] - t * dx, y - a[1] - t * dy)


def drawn(paths, n):
    """Every dab's centre, or with Spacing 0 every straight bit of what is drawn."""
    step = n["spacing"] / 100 * n["brush_size"]
    dabs, bits = [], []
    for pieces, window in windows(paths, n["start"], n["end"], n["sequential"]):
        if window is None:
            continue
        lo, hi = window
        if step == 0:
            # The drawn stretch as straight bits: from lo to each corner inside it, to hi.
            stops = [lo] + [at for _, _, at, _ in pieces if lo < at < hi] + [hi]
            bits += [(place(pieces, u), place(pieces, v)) for u, v in zip(stops, stops[1:])]
        else:
            k = math.ceil(lo / step - 1e-12)
            while k * step <= hi + 1e-12:
                if k * step >= lo - 1e-12:
                    dabs.append(place(pieces, k * step))
                k += 1
    return dabs, bits


def covering(dabs, bits, n, X, Y):
    if n["brush_size"] == 0 or (not dabs and not bits):
        return 0.0
    d = min([math.hypot(X - p[0], Y - p[1]) for p in dabs]
            + [seg_distance(a, b, X, Y) for a, b in bits])
    r = n["brush_size"] / 2
    w = max(r * (1 - n["brush_hardness"] / 100), 1.0)
    t = min(1.0, max(0.0, (r + 0.5 - d) / w))
    return n["opacity"] / 100 * t * t * (3 - 2 * t)


def stroke(layer, paths, n, color, style):
    C = [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]
    dabs, bits = drawn(paths, n)
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            p = layer["px"][j * layer["w"] + i]
            c = covering(dabs, bits, n, layer["left"] + i + 0.5, layer["top"] + j + 0.5)
            if style == "on_original":
                px.append([p[ch] * (1 - c) + C[ch] * c for ch in range(3)] + [p[3] * (1 - c) + c])
            elif style == "on_transparent":
                px.append([C[ch] * c for ch in range(3)] + [c])
            else:
                px.append([v * c for v in p])
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(masks=None, mask=1, all_masks="off", stroke_sequentially="off", color="#ffffff",
         brush_size=3, brush_hardness=75, opacity=100, start=0, end=100, spacing=15,
         paint_style="on_original", shift=0, tile=False, source="masks", shapes=None):
    return {"drawing": "night", "masks": [{"points": BOX, "mode": "none"}] if masks is None
            else masks, "mask": mask, "all_masks": all_masks,
            "stroke_sequentially": stroke_sequentially, "color": color, "brush_size": brush_size,
            "brush_hardness": brush_hardness, "opacity": opacity, "start": start, "end": end,
            "spacing": spacing, "paint_style": paint_style, "shift": shift, "tile": tile,
            "source": source, "shapes": shapes}


def shaped(*items, masks=(), **more):
    """D-357: a shape layer holding `items`, Path Stroke drawing along its shapes."""
    return case(masks=list(masks), source="shapes", shapes=list(items), **more)


def sh(pts, closed=True, **more):
    return {"points": pts, "closed": closed, **more}


def none(pts, **more):
    return {"points": pts, "mode": "none", **more}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def paths_of(c, frame_no):
    """The masks that are paths, as the case's settings choose them; None when there are none."""
    usable = lambda m, pts: m.get("enabled", True) and len(pts) >= 2  # noqa: E731
    shapes = c["source"] == "shapes"
    items = (c["shapes"] or []) if shapes else c["masks"]
    at = [(m, points_at(m, frame_no)) for m in items]
    if c["all_masks"] == "on":
        chosen = [(m, pts) for m, pts in at if usable(m, pts)]
    else:
        i = math.floor(held(c, "mask", frame_no)) - 1
        chosen = [at[i]] if i < len(at) and usable(*at[i]) else []
    if shapes:
        # D-357: a shape's path as D-78 flattens it, open or closed as the shape says.
        return [(SH.flatten(pts, m["closed"]), m["closed"]) for m, pts in chosen] or None
    return [(flatten(pts), True) for _, pts in chosen] or None


def layer_of(c):
    if c["shapes"] is not None:
        # D-357: a shape layer, transparent black with its shapes' fills laid in. The cases'
        # fills are rectangles with whole-number corners, so each pixel is wholly in or out.
        px = [[0.0] * 4 for _ in range(W * H)]
        for s in c["shapes"]:
            if s.get("fill") and s.get("enabled", True):
                xs = [q["point"][0] for q in s["points"]]
                ys = [q["point"][1] for q in s["points"]]
                col, a = s["fill"]["color"], s["fill"]["opacity"]
                px = [[v * a for v in col] + [a] if min(xs) <= i % W < max(xs)
                      and min(ys) <= i // W < max(ys) else q for i, q in enumerate(px)]
        return {"px": px, "left": 0, "top": 0, "w": W, "h": H}
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
    paths = paths_of(c, frame_no)
    if paths is None:
        return plain(c)
    n = {k: held(c, k, frame_no) for k in RANGES}
    n["sequential"] = c["all_masks"] == "on" and c["stroke_sequentially"] == "on"
    return frame(stroke(layer_of(c), paths, n, c["color"], c["paint_style"]), c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


TWO = [none(BOX), none(SMALL)]

CASES = {
    "FX-STROKE-001": ("Mask 1, a box from (2, 2) to (13, 7) of mode None, stroked with the "
                      "defaults but Brush Size 3: a white line 3 pixels wide all round the box, "
                      "over the night sky and the empty half, Hardness 75, Spacing 15, Start 0, "
                      "End 100, On Original Image.", case(), [0]),
    "FX-STROKE-002": ("End 50: from the top-left corner along the top and down the right side, "
                      "half the box's 32 pixels.", case(end=50), [0]),
    "FX-STROKE-003": ("Start 25, End 75: from 8 pixels along to 24.", case(start=25, end=75),
                      [0]),
    "FX-STROKE-004": ("Start 75, End 25: taken smaller first, FX-STROKE-003's frame.",
                      case(start=75, end=25), [0]),
    "FX-STROKE-005": ("Start 40, End 40: nothing is drawn; the drawing, untouched.",
                      case(start=40, end=40), [0]),
    "FX-STROKE-006": ("Spacing 100, Hardness 0: round dabs a brush apart, each soft, a string "
                      "of beads.", case(spacing=100, brush_hardness=0), [0]),
    "FX-STROKE-007": ("Spacing 0: the brush laid all along, a smooth line.", case(spacing=0),
                      [0]),
    "FX-STROKE-008": ("Hardness 0: soft from its middle out.", case(brush_hardness=0), [0]),
    "FX-STROKE-009": ("Hardness 100: hard, smoothed over one pixel.", case(brush_hardness=100),
                      [0]),
    "FX-STROKE-010": ("Brush Size 6.", case(brush_size=6), [0]),
    "FX-STROKE-011": ("Opacity 50: half covered at most.", case(opacity=50), [0]),
    "FX-STROKE-012": ("Colour #ff3020, a red.", case(color=RED), [0]),
    "FX-STROKE-013": ("On Transparent: the stroke alone, the night sky gone.",
                      case(paint_style="on_transparent"), [0]),
    "FX-STROKE-014": ("Reveal Original Image: the night sky only under the stroke, the colour "
                      "unused.", case(paint_style="reveal"), [0]),
    "FX-STROKE-015": ("Brush Size 0: nothing is drawn; the drawing, untouched.",
                      case(brush_size=0), [0]),
    "FX-STROKE-016": ("Brush Size 0, On Transparent: nothing at all.",
                      case(brush_size=0, paint_style="on_transparent"), [0]),
    "FX-STROKE-017": ("Two masks, the box and a small box from (5, 4) to (9, 6), Path mask 2: "
                      "the small box alone.", case(masks=TWO, mask=2), [0]),
    "FX-STROKE-018": ("Two masks, All Masks on: both.", case(masks=TWO, all_masks="on"), [0]),
    "FX-STROKE-019": ("Two masks, All Masks on, End 50: each mask's own first half.",
                      case(masks=TWO, all_masks="on", end=50), [0]),
    "FX-STROKE-020": ("Two masks, All Masks and Stroke Sequentially on, End keyed from 0 at "
                      "frame 0 to 100 at frame 4, linear: the box draws on, then the small box, "
                      "as one 44-pixel path; frame 0 nothing, frame 2 22 pixels of the box, "
                      "frame 3 the box and 1 pixel of the small box, frame 4 both.",
                      case(masks=TWO, all_masks="on", stroke_sequentially="on",
                           end=keyed((0, 0), (4, 100))), [0, 1, 2, 3, 4]),
    "FX-STROKE-021": ("Stroke Sequentially on with All Masks off: the one mask, FX-STROKE-002's "
                      "frame.", case(stroke_sequentially="on", end=50), [0]),
    "FX-STROKE-022": ("A curved mask, a circle 8 across about (8, 5): the stroke follows the "
                      "curve.", case(masks=[none(RING)]), [0]),
    "FX-STROKE-023": ("Start keyed from 0 to 50 and End from 50 to 100, frames 0 to 4: half the "
                      "box travels round it, frames 0, 2 and 4.",
                      case(start=keyed((0, 0), (4, 50)), end=keyed((0, 50), (4, 100))),
                      [0, 2, 4]),
    "FX-STROKE-024": ("The mask's path keyed from the box at frame 0 to the box two rows lower "
                      "at frame 4: the stroke follows it, frames 0, 2 and 4.",
                      case(masks=[{**moving_path((0, BOX), (4, LOW)), "mode": "none"}]),
                      [0, 2, 4]),
    "FX-STROKE-025": ("End keyed from 4 at frame 0 to 100 at frame 4, eased past its end: "
                      "frame 2 would pass 100, is held at 100, and is End 100.",
                      case(end=keyed((0, 4, OVERSHOOT), (4, 100))), [0, 2]),
    "FX-STROKE-026": ("FX-STROKE-001 moved three pixels right: the stroke moves with the "
                      "drawing.", case(shift=3), [0]),
    "FX-STROKE-027": ("After a Motion Tile that grows the layer: the mask is the drawing's own, "
                      "so the frame is FX-STROKE-001's.", case(tile=True), [0]),
    "FX-STROKE-028": ("The box's mask of mode Add: the drawing is cut to the box first, then "
                      "stroked, the stroke's outer half over nothing.",
                      case(masks=[{"points": BOX, "mode": "add"}]), [0]),
    "FX-STROKE-029": ("Two masks, the first switched off, All Masks on: the small box alone.",
                      case(masks=[none(BOX, enabled=False), none(SMALL)], all_masks="on"), [0]),
    "FX-STROKE-030": ("Path mask 1.5: its floor, mask 1, FX-STROKE-001's frame.",
                      case(mask=1.5), [0]),
}

MISSING = {
    "FX-STROKE-031": ("No masks at all.", case(masks=[])),
    "FX-STROKE-032": ("Path mask 3, of two.", case(masks=TWO, mask=3)),
    "FX-STROKE-033": ("Path mask 1, switched off.", case(masks=[none(BOX, enabled=False)])),
    "FX-STROKE-034": ("All Masks on, every mask switched off.",
                      case(masks=[none(BOX, enabled=False)], all_masks="on")),
}

INVALID = {
    "FX-STROKE-035": ("Brush Size 201, above 200.", case(brush_size=201)),
    "FX-STROKE-036": ("Brush Hardness -1, below 0.", case(brush_hardness=-1)),
    "FX-STROKE-037": ("Opacity 101, above 100.", case(opacity=101)),
    "FX-STROKE-038": ("Start -1, below 0.", case(start=-1)),
    "FX-STROKE-039": ("End 101, above 100.", case(end=101)),
    "FX-STROKE-040": ("Spacing 101, above 100.", case(spacing=101)),
    "FX-STROKE-041": ("Path mask 0, below 1.", case(mask=0)),
    "FX-STROKE-042": ("A colour \"#12345\", not six hex digits.", case(color="#12345")),
    "FX-STROKE-043": ("Paint Style \"paint\", which is not \"on_original\", \"on_transparent\" "
                      "or \"reveal\".", case(paint_style="paint")),
    "FX-STROKE-044": ("All Masks \"yes\", which is not \"on\" or \"off\".",
                      case(all_masks="yes")),
    "FX-STROKE-045": ("Stroke Sequentially \"maybe\", which is not \"on\" or \"off\".",
                      case(stroke_sequentially="maybe")),
    "FX-STROKE-046": ("End keyed to 101 at frame 4, above 100.",
                      case(end=keyed((0, 0), (4, 101)))),
}


# D-357: shape-layer paths. A ten-pointed star about (8, 5), its points 4.5 out and 2 in.
STAR = points(*[[8 + (4.5 if k % 2 == 0 else 2) * math.cos(math.radians(-90 + 36 * k)),
                 5 + (4.5 if k % 2 == 0 else 2) * math.sin(math.radians(-90 + 36 * k))]
                for k in range(10)])
ROOF = points([2, 7], [8, 2], [14, 7])                 # two legs, 7.81 long each
BLUE = {"color": [0.2, 0.5, 0.8], "opacity": 1.0}

SHAPE_CASES = {
    "FX-STROKE-047": ("A shape layer holding one shape, the box, with no fill and no stroke "
                      "(it draws nothing), Path From Shapes: the white line all round the box "
                      "over nothing, FX-STROKE-013's frame.", shaped(sh(BOX)), [0]),
    "FX-STROKE-048": ("A star of five points (ten corners) as a shape, Brush Size 1, End 60: "
                      "the line follows its points and dips, clockwise from the top point, "
                      "three fifths of the way round; the left arms are not drawn yet.",
                      shaped(sh(STAR), end=60, brush_size=1), [0]),
    "FX-STROKE-049": ("An open shape, a roof of two legs from (2, 7) up to (8, 2) and down to "
                      "(14, 7): drawn open, nothing along the bottom.",
                      shaped(sh(ROOF, closed=False)), [0]),
    "FX-STROKE-050": ("The same roof closed: the bottom is drawn too.", shaped(sh(ROOF)), [0]),
    "FX-STROKE-051": ("The open roof, End 50: the left leg alone, half of its open length.",
                      shaped(sh(ROOF, closed=False), end=50), [0]),
    "FX-STROKE-052": ("A curved shape, the circle 8 across about (8, 5): FX-STROKE-022's curve.",
                      shaped(sh(RING)), [0]),
    "FX-STROKE-053": ("Two shapes, the box and the small box, Path 2: the small box alone.",
                      shaped(sh(BOX), sh(SMALL), mask=2), [0]),
    "FX-STROKE-054": ("Two shapes, All Masks and Stroke Sequentially on, End keyed from 0 at "
                      "frame 0 to 100 at frame 4: the box draws on, then the small box, as "
                      "FX-STROKE-020 does with masks; frames 0, 2 and 4.",
                      shaped(sh(BOX), sh(SMALL), all_masks="on", stroke_sequentially="on",
                             end=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-STROKE-055": ("The box as a shape filled blue: the stroke is drawn over the fill.",
                      shaped(sh(BOX, fill=BLUE)), [0]),
    "FX-STROKE-056": ("A shape layer with the box as a shape and the small box as a mask of "
                      "mode None, Path From Shapes: the box alone, FX-STROKE-047's frame.",
                      shaped(sh(BOX), masks=[none(SMALL)]), [0]),
    "FX-STROKE-057": ("The same layer, Path From Masks: the small box, its mask, alone.",
                      {**shaped(sh(BOX), masks=[none(SMALL)]), "source": "masks"}, [0]),
}

SHAPE_MISSING = {
    "FX-STROKE-058": ("Path From Shapes on the night drawing, which is not a shape layer and has "
                      "no shapes (its box mask is not a shape).", case(source="shapes")),
    "FX-STROKE-059": ("A shape layer whose one shape is switched off.",
                      shaped(sh(BOX, enabled=False))),
    "FX-STROKE-060": ("A shape layer of one shape, Path 2.", shaped(sh(BOX), mask=2)),
}

SHAPE_INVALID = {
    "FX-STROKE-061": ("Path From \"layer\", which is not \"masks\" or \"shapes\".",
                      case(source="layer")),
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
    parameters = {k: (c[k] if k in WORDS else setting_json(c[k])) for k in NAMES}
    if c["source"] != "masks":
        # D-357: written only when it is not the default, so D-356's files are as they were.
        parameters["source"] = c["source"]
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.stroke",
                    "enabled": True, "parameters": parameters})
    layer["effects"] = effects
    if c["shapes"] is not None:
        # D-357: a shape layer has no asset, exposures or source offset, and its space is the
        # composition's; its shapes are written as D-78 writes them.
        p["assets"] = []
        layer = {"id": layer["id"], "kind": "shape", "name": layer["name"], "enabled": True,
                 "locked": False, "in_frame": layer["in_frame"], "out_frame": layer["out_frame"],
                 "shapes": [SH.shape_json(q, n + 1) for n, q in enumerate(c["shapes"])],
                 "transform": layer["transform"],
                 "masks": [mask_json(m, i + 1) for i, m in enumerate(c["masks"])],
                 "matte": None, "blend_mode": "normal", "effects": effects}
        comp["layers"][0] = layer
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
        says += (" Nothing to draw along: the layer is drawn without the effect, which is kept "
                 "as written, and EFFECT_PATH_MISSING is said every frame.")
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
    (OUT / "expected_stroke.json").write_text(json.dumps(expected, indent=1) + "\n",
                                              encoding="utf-8")
    check(expected)
    shapes = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in SHAPE_CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        shapes["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in SHAPE_MISSING.items():
        says += (" Nothing to draw along: the layer is drawn without the effect, which is kept "
                 "as written, and EFFECT_PATH_MISSING is said every frame.")
        before = plain(c)
        assert render(c, 0) == before
        shapes["cases"][fx] = {"says": says, "project": write(fx, c),
                               "frames": {"0": before, "4": before},
                               "frame_warning": "EFFECT_PATH_MISSING"}
        print(f"{fx}: no path")
    for fx, (says, c) in SHAPE_INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        shapes["cases"][fx] = {"says": says, "project": write(fx, c),
                               "frames": {"0": plain(c), "4": plain(c)},
                               "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_stroke_shapes.json").write_text(json.dumps(shapes, indent=1) + "\n",
                                                     encoding="utf-8")
    check_shapes(shapes, expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    art = plain(case())
    white = [1.0, 1.0, 1.0, 1.0]

    # Every covering in 0..1 and every colour inside its covering.
    for fx, frames in c.items():
        for px in frames.values():
            assert all(0 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3])
                       for p in px), fx

    one = c["FX-STROKE-001"]["0"]
    # White on the box's edges (the pixels whose centres are half a pixel from them), the inside
    # of the box and the far outside untouched.
    for x in range(3, 12):
        assert near(one[at(x, 1)], white) and near(one[at(x, 2)], white), x
        assert near(one[at(x, 6)], white) and near(one[at(x, 7)], white), x
        assert one[at(x, 9)] == art[at(x, 9)]
        assert x in (3, 11) or one[at(x, 4)] == art[at(x, 4)]
    for y in range(3, 6):
        assert near(one[at(1, y)], white) and near(one[at(13, y)], white), y
    assert one[at(15, 4)] == art[at(15, 4)] and one[at(7, 0)] != art[at(7, 0)]
    half = c["FX-STROKE-002"]["0"]
    assert near(half[at(8, 1)], white) and near(half[at(13, 4)], white)
    assert half[at(1, 4)] == art[at(1, 4)] and half[at(8, 7)] == art[at(8, 7)]
    mid = c["FX-STROKE-003"]["0"]
    assert mid == c["FX-STROKE-004"]["0"]
    assert near(mid[at(13, 4)], white) and mid[at(4, 1)] == art[at(4, 1)]
    assert c["FX-STROKE-005"]["0"] == art
    beads = c["FX-STROKE-006"]["0"]
    assert beads != one
    assert c["FX-STROKE-007"]["0"] != one
    soft, hard = c["FX-STROKE-008"]["0"], c["FX-STROKE-009"]["0"]
    assert soft[at(8, 0)][3] < one[at(8, 0)][3] <= hard[at(8, 0)][3] + 1e-12
    wide = c["FX-STROKE-010"]["0"]
    assert sum(p != q for p, q in zip(wide, art)) > sum(p != q for p, q in zip(one, art))
    faint = c["FX-STROKE-011"]["0"]
    a = art[at(10, 2)]
    assert near(faint[at(10, 2)], [0.5 * v + 0.5 for v in a])
    red = c["FX-STROKE-012"]["0"]
    assert near(red[at(8, 2)], [srgb_to_linear(v / 255) for v in R.hex_color(RED)] + [1.0])
    alone = c["FX-STROKE-013"]["0"]
    assert alone[at(8, 4)] == [0.0] * 4 and near(alone[at(8, 2)], white)
    reveal = c["FX-STROKE-014"]["0"]
    assert reveal[at(8, 4)] == [0.0] * 4 and near(reveal[at(3, 2)], art[at(3, 2)])
    assert c["FX-STROKE-015"]["0"] == art
    assert all(p == [0.0] * 4 for p in c["FX-STROKE-016"]["0"])
    small = c["FX-STROKE-017"]["0"]
    assert small[at(8, 1)] == art[at(8, 1)] and near(small[at(7, 4)], white)
    both = c["FX-STROKE-018"]["0"]
    assert near(both[at(8, 1)], white) and near(both[at(7, 4)], white)
    seq = c["FX-STROKE-020"]
    assert seq["0"] == art and seq["4"] == both
    assert seq["2"][at(7, 4)] == art[at(7, 4)] and near(seq["2"][at(13, 4)], white)
    assert seq["3"][at(9, 4)] == art[at(9, 4)] and seq["3"][at(5, 4)] != art[at(5, 4)]
    assert c["FX-STROKE-021"]["0"] == half
    ring = c["FX-STROKE-022"]["0"]
    assert near(ring[at(8, 0)], white, 0.05) and ring[at(8, 5)] == art[at(8, 5)]
    trav = c["FX-STROKE-023"]
    assert trav["0"] == half and trav["2"] == mid
    moved_path = c["FX-STROKE-024"]
    assert moved_path["0"] == one and moved_path["4"] != one
    over = c["FX-STROKE-025"]
    assert ease(OVERSHOOT, 0.5) > 1 and over["2"] == one
    moved = c["FX-STROKE-026"]["0"]
    assert all(moved[at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert c["FX-STROKE-027"]["0"] == one
    cut = c["FX-STROKE-028"]["0"]
    assert cut[at(8, 4)] == art[at(8, 4)] and cut[at(1, 4)][3] == one[at(1, 4)][3]
    assert cut[at(0, 4)] != art[at(0, 4)]
    assert c["FX-STROKE-029"]["0"] == small
    assert c["FX-STROKE-030"]["0"] == one
    for fx in list(MISSING) + list(INVALID):
        assert c[fx]["0"] == c[fx]["4"] == plain(case())
    print("checked")


def check_shapes(shapes, expected):
    """D-357's claims, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in shapes["cases"].items()}
    old = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    clear = [0.0] * 4
    alone = lambda **k: render(case(paint_style="on_transparent", **k), 0)  # noqa: E731
    for fx, frames in c.items():
        for px in frames.values():
            assert all(0 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3])
                       for p in px), fx
    box = c["FX-STROKE-047"]["0"]
    # Over nothing, On Original Image is the stroke alone: the mask's On Transparent frame.
    assert box == old["FX-STROKE-013"]["0"]
    star = c["FX-STROKE-048"]["0"]
    assert star[at(8, 1)][3] > 0.9 and star[at(11, 3)][3] > 0.9 and star[at(5, 8)][3] > 0.9
    assert all(star[at(x, y)] == clear for x in range(4) for y in range(H))
    assert star[at(8, 5)] == clear
    open_roof, closed_roof = c["FX-STROKE-049"]["0"], c["FX-STROKE-050"]["0"]
    assert open_roof[at(8, 7)] == clear and closed_roof[at(8, 7)][3] > 0.9
    assert open_roof[at(5, 4)] == closed_roof[at(5, 4)] and open_roof[at(5, 4)][3] > 0.5
    left = c["FX-STROKE-051"]["0"]
    assert left[at(4, 5)][3] > 0.5 and left[at(12, 5)] == clear
    # The shape's curve is the mask's: D-78 flattens closed paths as D-77 does.
    assert c["FX-STROKE-052"]["0"] == alone(masks=[none(RING)])
    small = c["FX-STROKE-053"]["0"]
    assert small[at(8, 1)] == clear and small[at(7, 4)][3] > 0.9
    seq = c["FX-STROKE-054"]
    for f in (0, 2, 4):
        assert seq[str(f)] == render(case(masks=TWO, all_masks="on", stroke_sequentially="on",
                                          end=keyed((0, 0), (4, 100)),
                                          paint_style="on_transparent"), f)
    filled = c["FX-STROKE-055"]["0"]
    assert filled[at(7, 4)] == [0.2, 0.5, 0.8, 1.0] and filled[at(8, 2)] == [1.0] * 4
    assert c["FX-STROKE-056"]["0"] == box
    assert c["FX-STROKE-057"]["0"] == alone(masks=[none(SMALL)])
    for fx in list(SHAPE_MISSING) + list(SHAPE_INVALID):
        assert c[fx]["0"] == c[fx]["4"]
    print("checked the shapes")


if __name__ == "__main__":
    main()
