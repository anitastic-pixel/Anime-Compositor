"""Vegas, worked a second way.

D-444 adds `core.vegas`, after After Effects' Vegas (Generate): dashes run along a path, each
fading from its start to its end, and turn round it as Rotation moves. Adobe names the controls
(Stroke, Segments, Length, Segment Distribution, Rotation, Random Phase, Random Seed, Blend Mode,
Color, Width, Hardness, Start Opacity, Mid-point Opacity, Mid-point Position, End Opacity) and
publishes no formula. The rule below is this program's own, drawn with P0-22's machinery (the
paths Path Stroke finds); nothing is ported. The owner's decision of 2026-10-10, "Masks only for
now": the paths are the layer's masks or a shape layer's paths. After Effects' default stroke,
Image Contours (dashes along the edges of the picture), is not built: a file asking for it is
read, the effect kept as written and left out of every frame with a sentence saying so. Masks are
always closed (document 19).

The paths. `stroke` "masks": the layer's masks at the frame, each as document 19 flattens it,
closed; "shapes": the shape layer's own paths, each as D-78 flattens it, open or closed as the
shape says. `mask` (1 the first, its floor taken) picks one, or `all_masks` "on" takes every one
in the layer's order. A mask or shape is a path when it is on and has two points or more. With
none, nothing is drawn, the layer is left as it is, and EFFECT_PATH_MISSING is said every frame.
Exactly Path Stroke's choice (document 21, D-356 and D-357).

The dashes. Each path, numbered c = 0, 1 ... in the order chosen, is measured along its length L
from its first point (an open one without the piece from its last point back to its first). With
N = floor(`segments`), l = `length` L / N each dash's length, and the offset
o = (`rotation` / 360 + phase) L, where phase is 0 with `random_phase` "off" and otherwise
(mix(mix(floor(`random_seed`)) xor c) >> 11) / 2^53, D-119's SplitMix64 finaliser `mix`: dash
i = 0 .. N - 1 begins at a = o + i L / N with `segment_distribution` "even" (spread round the
path) or a = o + i l with "bunched" (end to end from the offset, like the cars of a train), and
runs to a + l. Places along a path are taken modulo L, so a dash runs on past the path's end from
its start: a' = a mod L in [0, L), and the dash covers the path's pieces between a' and a' + l,
going round once more where it passes L. Each straight piece of the path, from `at` along to
`at + n` (n > 0), meets the dash on [max(a', at + k L), min(a' + l, at + n + k L)], k 0 or 1,
when the first is less than the second: a straight bit of the dash, from point P to point Q, its
ends at s_P and s_Q along the dash, s = (u - a') / l, 0 at the dash's start and 1 at its end.
`length` 0 or `width` 0 draws nothing.

The brush. At a pixel's centre x, for each straight bit, the nearest point of it, t = clamp(((x -
P) . (Q - P)) / |Q - P|^2, 0, 1), its distance d = |x - P - t (Q - P)| and its place
s = s_P + t (s_Q - s_P). With r = `width` / 2, w = max(r (1 - `hardness`), 1) and
v = clamp((r + 0.5 - d) / w, 0, 1), the bit gives v^2 (3 - 2 v) times the opacity along the dash
at s: from S = `start_opacity` at 0 to M at m = `mid_point_position`, then to E = `end_opacity`
at 1, in straight lines, M = clamp(S + m (E - S) + `mid_point_opacity`, 0, 1) (Mid-point
Opacity 0 a straight fade from start to end); at m 0 the first line is not used and at m 1 the
second is M. The covering c is the largest any bit gives: dashes do not build up.

Blend Mode, with the layer O (linear, premultiplied) and the colour C (linear): "over" the stroke
over the layer, O.rgb (1 - c) + C c and O.a (1 - c) + c; "under" behind it, O.rgb + (1 - O.a) C c
and O.a + (1 - O.a) c; "transparent" the stroke alone, C c and c; "stencil" the layer only where
the stroke is, O c, the colour unused. The layer does not grow. A draft scales Width as a
distance; the paths are scaled as they always are.

`stroke` "masks", "shapes" or "image_contours" (refused), "masks" when added; `mask` 1 to 1000, 1;
`all_masks` "off"/"on", "off"; `segments` 1 to 1000, 32; `length` 0 to 1, 1;
`segment_distribution` "bunched"/"even", "bunched"; `rotation` -360000 to 360000 degrees, 0;
`random_phase` "off"/"on", "off"; `random_seed` 0 to 100000, 1; `blend_mode` "transparent",
"over", "under" or "stencil", "over"; `color` #rrggbb, white; `width` 0 to 200 pixels, 2;
`hardness` 0 to 1, 0; `start_opacity` 0 to 1, 1; `mid_point_opacity` -1 to 1, 0;
`mid_point_position` 0 to 1, 0.5; `end_opacity` 0 to 1, 0. Every number keyable. The values when
added are chosen here after After Effects' (32 segments, Length 1, Bunched, Width 2, Start
Opacity 1, End Opacity 0); Adobe publishes no ranges.

**This file never runs the build's code path.** It works in double precision from the drawing's
8-bit values, where the build writes single precision into its buffers.

Every case is a composition 16 pixels by 10 holding Lightning Bolt's night, its left half a night
sky and its right half empty, with masks of mode None, as Path Stroke's cases are (FX-STROKE-001):
`tools/stroke_reference.py` builds the layer, the masks and the shape layers. The drawing goes
into `Fixtures/vegas/media`, the projects into `Fixtures/vegas`, and the expected frames into
`Fixtures/vegas/expected_vegas.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/vegas_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import recolor_reference as R  # noqa: E402
import smooth_reference as S  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from lightning_bolt_reference import DRAWINGS  # noqa: E402
from mask_reference import keyed as moving_path  # noqa: E402
from audio_waveform_reference import mix  # noqa: E402
import stroke_reference as ST  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "vegas"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H = ST.W, ST.H
RANGES = {"mask": (1, 1000), "segments": (1, 1000), "length": (0, 1),
          "rotation": (-360000, 360000), "random_seed": (0, 100000), "width": (0, 200),
          "hardness": (0, 1), "start_opacity": (0, 1), "mid_point_opacity": (-1, 1),
          "mid_point_position": (0, 1), "end_opacity": (0, 1)}
WORDS = ("stroke", "all_masks", "segment_distribution", "random_phase", "blend_mode", "color")
NAMES = ("stroke", "mask", "all_masks", "segments", "length", "segment_distribution", "rotation",
         "random_phase", "random_seed", "blend_mode", "color", "width", "hardness",
         "start_opacity", "mid_point_opacity", "mid_point_position", "end_opacity")
ADDED = {"stroke": "masks", "mask": 1, "all_masks": "off", "segments": 32, "length": 1,
         "segment_distribution": "bunched", "rotation": 0, "random_phase": "off",
         "random_seed": 1, "blend_mode": "over", "color": "#ffffff", "width": 2, "hardness": 0,
         "start_opacity": 1, "mid_point_opacity": 0, "mid_point_position": 0.5,
         "end_opacity": 0}
RED = "#ff3020"
BOX, SMALL, RING, LOW, STAR, ROOF = ST.BOX, ST.SMALL, ST.RING, ST.LOW, ST.STAR, ST.ROOF
none, sh = ST.none, ST.sh


# --- the rule -------------------------------------------------------------------------------

def phase(seed, c):
    return (mix(mix(int(math.floor(seed))) ^ c) >> 11) / 2 ** 53


def bits(paths, n):
    """Every straight bit of every dash: (P, Q, s_P, s_Q)."""
    out = []
    count = math.floor(n["segments"])
    for c, (outline, closed) in enumerate(paths):
        pieces, L = ST.measured(outline, closed)
        if L <= 0:
            continue
        l = n["length"] * L / count
        if l <= 0:
            continue
        o = (n["rotation"] / 360 + (phase(n["random_seed"], c) if n["random_phase"] else 0)) * L
        for i in range(count):
            a = o + i * (L / count if n["even"] else l)
            a = a % L
            for pa, pb, at, size in pieces:
                if size <= 0:
                    continue
                for k in (0, 1):
                    lo = at + k * L
                    u0, u1 = max(a, lo), min(a + l, lo + size)
                    if u0 < u1:
                        pt = lambda u: (pa[0] + (u - lo) / size * (pb[0] - pa[0]),  # noqa: E731
                                        pa[1] + (u - lo) / size * (pb[1] - pa[1]))
                        out.append((pt(u0), pt(u1), (u0 - a) / l, (u1 - a) / l))
    return out


def along(n, s):
    S0, E, m = n["start_opacity"], n["end_opacity"], n["mid_point_position"]
    M = min(1.0, max(0.0, S0 + m * (E - S0) + n["mid_point_opacity"]))
    if s < m:
        return S0 + (M - S0) * s / m
    if m < 1:
        return M + (E - M) * (s - m) / (1 - m)
    return M


def covering(dashes, n, X, Y):
    if n["width"] == 0:
        return 0.0
    r = n["width"] / 2
    w = max(r * (1 - n["hardness"]), 1.0)
    best = 0.0
    for p, q, sp, sq in dashes:
        dx, dy = q[0] - p[0], q[1] - p[1]
        l2 = dx * dx + dy * dy
        t = 0.0 if l2 == 0 else min(1.0, max(0.0, ((X - p[0]) * dx + (Y - p[1]) * dy) / l2))
        d = math.hypot(X - p[0] - t * dx, Y - p[1] - t * dy)
        v = min(1.0, max(0.0, (r + 0.5 - d) / w))
        best = max(best, v * v * (3 - 2 * v) * along(n, sp + t * (sq - sp)))
    return best


def vegas(layer, paths, n, color, mode):
    C = [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]
    dashes = bits(paths, n)
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            p = layer["px"][j * layer["w"] + i]
            c = covering(dashes, n, layer["left"] + i + 0.5, layer["top"] + j + 0.5)
            if mode == "over":
                px.append([p[ch] * (1 - c) + C[ch] * c for ch in range(3)] + [p[3] * (1 - c) + c])
            elif mode == "under":
                px.append([p[ch] + (1 - p[3]) * C[ch] * c for ch in range(3)]
                          + [p[3] + (1 - p[3]) * c])
            elif mode == "transparent":
                px.append([C[ch] * c for ch in range(3)] + [c])
            else:
                px.append([v * c for v in p])
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(masks=None, shapes=None, shift=0, tile=False, **settings):
    v = {**ADDED, **settings}
    base = ST.case(masks=masks, mask=v["mask"], all_masks=v["all_masks"], shift=shift, tile=tile,
                   source="shapes" if v["stroke"] == "shapes" else "masks", shapes=shapes)
    return {**base, **v}


def shaped(*items, masks=(), **more):
    return case(masks=list(masks), shapes=list(items), stroke="shapes", **more)


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def render(c, frame_no):
    paths = ST.paths_of(c, frame_no)
    if paths is None:
        return plain(c)
    n = {k: held(c, k, frame_no) for k in RANGES}
    n["even"] = c["segment_distribution"] == "even"
    n["random_phase"] = c["random_phase"] == "on"
    return frame(vegas(ST.layer_of(c), paths, n, c["color"], c["blend_mode"]), c["shift"])


def plain(c):
    return frame(ST.layer_of(c), c["shift"])


TWO = [none(BOX), none(SMALL)]
FOUR = {"segments": 4}
DASHES = {"segments": 4, "length": 0.5}

CASES = {
    "FX-VEGAS-001": ("Mask 1, the box from (2, 2) to (13, 7), 32 pixels round, of mode None; "
                     "Vegas as added but Segments 4: four dashes end to end, each 8 pixels "
                     "long, white and whole at its start fading to nothing at its end, over the "
                     "night sky and the empty half.", case(**FOUR), [0]),
    "FX-VEGAS-002": ("As added, 32 segments: 32 dashes of 1 pixel each, a fine shimmer all "
                     "round the box.", case(), [0]),
    "FX-VEGAS-003": ("Segments 4, Length 0.5, Even: four 4-pixel dashes spread round the box, "
                     "8 pixels apart.", case(**DASHES, segment_distribution="even"), [0]),
    "FX-VEGAS-004": ("Segments 4, Length 0.5, Bunched: four 4-pixel dashes end to end round "
                     "the first half of the box; the second half bare.", case(**DASHES), [0]),
    "FX-VEGAS-005": ("FX-VEGAS-003 with Rotation 45: every dash an eighth of the way on, 4 "
                     "pixels, into the gaps.", case(**DASHES, segment_distribution="even",
                                                    rotation=45), [0]),
    "FX-VEGAS-006": ("FX-VEGAS-004 with Rotation -90: the dashes a quarter of the way back, "
                     "running on past the box's first corner from its end.",
                     case(**DASHES, rotation=-90), [0]),
    "FX-VEGAS-007": ("FX-VEGAS-004 with Rotation keyed from 0 at frame 0 to 360 at frame 4, "
                     "linear: the train goes once round; frames 0, 1, 2 and 4, frame 4 "
                     "FX-VEGAS-004's.", case(**DASHES, rotation=keyed((0, 0), (4, 360))),
                     [0, 1, 2, 4]),
    "FX-VEGAS-008": ("Two masks, the box and a small box from (5, 4) to (9, 6), All Masks on, "
                     "Segments 2, Length 0.5, Random Phase off: each box's dashes start at its "
                     "own first point.", case(masks=TWO, all_masks="on", segments=2, length=0.5),
                     [0]),
    "FX-VEGAS-009": ("FX-VEGAS-008 with Random Phase on, Random Seed 1: each box's dashes "
                     "start somewhere of their own.",
                     case(masks=TWO, all_masks="on", segments=2, length=0.5,
                          random_phase="on"), [0]),
    "FX-VEGAS-010": ("FX-VEGAS-009 with Random Seed 2: other starts.",
                     case(masks=TWO, all_masks="on", segments=2, length=0.5,
                          random_phase="on", random_seed=2), [0]),
    "FX-VEGAS-011": ("FX-VEGAS-008 with Random Seed 7 and Random Phase off: the seed is "
                     "unused, FX-VEGAS-008's frame.",
                     case(masks=TWO, all_masks="on", segments=2, length=0.5, random_seed=7),
                     [0]),
    "FX-VEGAS-012": ("FX-VEGAS-001 with Blend Mode Transparent: the dashes alone, the night "
                     "gone.", case(**FOUR, blend_mode="transparent"), [0]),
    "FX-VEGAS-013": ("FX-VEGAS-001 with Blend Mode Under: the dashes behind the layer, seen "
                     "only on the empty right half.", case(**FOUR, blend_mode="under"), [0]),
    "FX-VEGAS-014": ("FX-VEGAS-001 with Blend Mode Stencil: the night only under the dashes, "
                     "the colour unused.", case(**FOUR, blend_mode="stencil"), [0]),
    "FX-VEGAS-015": ("Segments 4, a red #ff3020, Width 4, Hardness 0.5.",
                     case(**FOUR, color=RED, width=4, hardness=0.5), [0]),
    "FX-VEGAS-016": ("Segments 4, Width 3, Hardness 1: hard edges, smoothed over one pixel.",
                     case(**FOUR, width=3, hardness=1), [0]),
    "FX-VEGAS-017": ("Segments 4, Length 0.5, Even, Start Opacity 1 and End Opacity 1: whole "
                     "dashes, no fade.", case(**DASHES, segment_distribution="even",
                                              end_opacity=1), [0]),
    "FX-VEGAS-018": ("Segments 4, Start Opacity 0.2, End Opacity 1, Mid-point Opacity 0.5 at "
                     "Mid-point Position 0.25: faint, quickly brighter, then whole at the end.",
                     case(**FOUR, start_opacity=0.2, end_opacity=1, mid_point_opacity=0.5,
                          mid_point_position=0.25), [0]),
    "FX-VEGAS-019": ("Segments 4, Start and End Opacity 1, Mid-point Opacity -1: each dash "
                     "dips to nothing at its middle.",
                     case(**FOUR, end_opacity=1, mid_point_opacity=-1), [0]),
    "FX-VEGAS-020": ("Segments 4, Mid-point Position 0, Mid-point Opacity -0.4: the first "
                     "line unused, each dash starting at 0.6 and fading to nothing.",
                     case(**FOUR, mid_point_position=0, mid_point_opacity=-0.4), [0]),
    "FX-VEGAS-021": ("Segments 4, Mid-point Position 1, Mid-point Opacity 0.5: the first line "
                     "the whole dash, from 1 down to M, 0.5, at its end; End Opacity unused.",
                     case(**FOUR, mid_point_position=1, mid_point_opacity=0.5), [0]),
    "FX-VEGAS-022": ("Length 0: nothing is drawn; the drawing, untouched.", case(length=0),
                     [0]),
    "FX-VEGAS-023": ("Width 0: nothing is drawn; the drawing, untouched.", case(width=0), [0]),
    "FX-VEGAS-024": ("Width 0, Stencil: nothing at all.", case(width=0, blend_mode="stencil"),
                     [0]),
    "FX-VEGAS-025": ("Segments 1, Length 0.75: one long dash three quarters of the way round, "
                     "fading from the box's top-left corner.", case(segments=1, length=0.75),
                     [0]),
    "FX-VEGAS-026": ("Segments 4.7: its floor, 4, FX-VEGAS-001's frame.", case(segments=4.7),
                     [0]),
    "FX-VEGAS-027": ("A curved mask, a circle 8 across about (8, 5), Segments 6, Length 0.6, "
                     "Even: six dashes follow the curve.",
                     case(masks=[none(RING)], segments=6, length=0.6,
                          segment_distribution="even"), [0]),
    "FX-VEGAS-028": ("Two masks, Path mask 2: the small box alone, Segments 4.",
                     case(masks=TWO, mask=2, **FOUR), [0]),
    "FX-VEGAS-029": ("The mask's path keyed from the box at frame 0 to the box two rows lower "
                     "at frame 4: the dashes follow it, frames 0, 2 and 4.",
                     case(masks=[{**moving_path((0, BOX), (4, LOW)), "mode": "none"}], **FOUR),
                     [0, 2, 4]),
    "FX-VEGAS-030": ("FX-VEGAS-001 moved three pixels right: the dashes move with the drawing.",
                     case(**FOUR, shift=3), [0]),
    "FX-VEGAS-031": ("After a Motion Tile that grows the layer: the mask is the drawing's own, "
                     "so the frame is FX-VEGAS-001's.", case(**FOUR, tile=True), [0]),
    "FX-VEGAS-032": ("The box's mask of mode Add: the drawing is cut to the box first, then "
                     "the dashes drawn, their outer half over nothing.",
                     case(masks=[{"points": BOX, "mode": "add"}], **FOUR), [0]),
    "FX-VEGAS-033": ("Width 3 and Segments keyed from 1 at frame 0 to 8 at frame 4: more, "
                     "shorter dashes; frames 0, 2 and 4.",
                     case(width=3, segments=keyed((0, 1), (4, 8))), [0, 2, 4]),
}

SHAPE_CASES = {
    "FX-VEGAS-034": ("A shape layer holding the box as a shape with no fill and no stroke, "
                     "Stroke Shapes, Segments 4: the dashes over nothing, FX-VEGAS-012's frame.",
                     shaped(sh(BOX), **FOUR), [0]),
    "FX-VEGAS-035": ("An open shape, a roof of two legs from (2, 7) up to (8, 2) and down to "
                     "(14, 7), Segments 3, Length 0.5, Rotation 300: the open path has no "
                     "bottom, and a dash that passes its end runs on from its start.",
                     shaped(sh(ROOF, closed=False), segments=3, length=0.5, rotation=300), [0]),
    "FX-VEGAS-036": ("A star of five points as a shape, Segments 5, Length 0.5, Even, Width 1: "
                     "dashes on its arms.", shaped(sh(STAR), segments=5, length=0.5, width=1,
                                                   segment_distribution="even"), [0]),
    "FX-VEGAS-037": ("A shape layer with the box as a shape and the small box as a mask of mode "
                     "None, Stroke Masks, Segments 4: the small box, the mask, alone.",
                     {**shaped(sh(BOX), masks=[none(SMALL)], **FOUR), "stroke": "masks",
                      "source": "masks"}, [0]),
}

MISSING = {
    "FX-VEGAS-038": ("No masks at all.", case(masks=[])),
    "FX-VEGAS-039": ("Path mask 3, of two.", case(masks=TWO, mask=3)),
    "FX-VEGAS-040": ("All Masks on, every mask switched off.",
                     case(masks=[none(BOX, enabled=False)], all_masks="on")),
    "FX-VEGAS-041": ("Stroke Shapes on the night drawing, which is not a shape layer.",
                     case(stroke="shapes")),
}

INVALID = {
    "FX-VEGAS-042": ("Stroke \"image_contours\", After Effects' Image Contours, which is not "
                     "built: refused with a sentence, never drawn along the masks instead.",
                     case(stroke="image_contours")),
    "FX-VEGAS-043": ("Stroke \"edges\", which is not \"masks\", \"shapes\" or "
                     "\"image_contours\".", case(stroke="edges")),
    "FX-VEGAS-044": ("Segments 0, below 1.", case(segments=0)),
    "FX-VEGAS-045": ("Segments 1001, above 1000.", case(segments=1001)),
    "FX-VEGAS-046": ("Length 1.5, above 1.", case(length=1.5)),
    "FX-VEGAS-047": ("Width 201, above 200.", case(width=201)),
    "FX-VEGAS-048": ("Hardness 1.5, above 1.", case(hardness=1.5)),
    "FX-VEGAS-049": ("Start Opacity -0.1, below 0.", case(start_opacity=-0.1)),
    "FX-VEGAS-050": ("Mid-point Opacity 1.5, above 1.", case(mid_point_opacity=1.5)),
    "FX-VEGAS-051": ("Mid-point Position -0.1, below 0.", case(mid_point_position=-0.1)),
    "FX-VEGAS-052": ("End Opacity 2, above 1.", case(end_opacity=2)),
    "FX-VEGAS-053": ("Blend Mode \"add\", which is not \"transparent\", \"over\", \"under\" or "
                     "\"stencil\".", case(blend_mode="add")),
    "FX-VEGAS-054": ("Segment Distribution \"random\", which is not \"bunched\" or \"even\".",
                     case(segment_distribution="random")),
    "FX-VEGAS-055": ("Random Phase \"yes\", which is not \"off\" or \"on\".",
                     case(random_phase="yes")),
    "FX-VEGAS-056": ("A colour \"#12345\", not six hex digits.", case(color="#12345")),
    "FX-VEGAS-057": ("All Masks \"maybe\", which is not \"off\" or \"on\".",
                     case(all_masks="maybe")),
    "FX-VEGAS-058": ("Random Seed -1, below 0.", case(random_seed=-1)),
    "FX-VEGAS-059": ("Path mask 0, below 1.", case(mask=0)),
    "FX-VEGAS-060": ("Rotation keyed to 360001 at frame 4, above 360000.",
                     case(rotation=keyed((0, 0), (4, 360001)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = ST.project_json(fx, c)
    layer = p["compositions"][0]["layers"][0]
    layer["effects"][-1] = {"instance_id": layer["effects"][-1]["instance_id"],
                            "type_id": "core.vegas", "enabled": True,
                            "parameters": {k: (c[k] if k in WORDS else setting_json(c[k]))
                                           for k in NAMES}}
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    (OUT / "media" / "night.png").write_bytes(S.png(DRAWINGS["night"]))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in {**CASES, **SHAPE_CASES}.items():
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
    (OUT / "expected_vegas.json").write_text(json.dumps(expected, indent=1) + "\n",
                                             encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda a, b, e=1e-9: all(near(p, q, e) for p, q in zip(a, b))  # noqa: E731
    art = plain(case())
    white, clear = [1.0] * 4, [0.0] * 4
    for fx, frames in c.items():
        for px in frames.values():
            assert all(0 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3])
                       for p in px), fx

    one = c["FX-VEGAS-001"]["0"]
    # Dash 1 runs along the top from (2, 2) to (10, 2): whole at its start, faint at its end
    # (pixel 9 is lit again by dash 2's round start).
    assert near(one[at(2, 1)], white, 0.07) and one[at(8, 1)][0] < one[at(5, 1)][0]
    assert one[at(8, 4)] == art[at(8, 4)] and one[at(15, 4)] == art[at(15, 4)]
    assert c["FX-VEGAS-002"]["0"] != one
    even, bunched = c["FX-VEGAS-003"]["0"], c["FX-VEGAS-004"]["0"]
    # Even: a dash on each side; Bunched: the bottom (16 to 27 along) and left are bare.
    assert even[at(11, 7)] != art[at(11, 7)] and bunched[at(11, 7)] == art[at(11, 7)]
    assert bunched[at(4, 1)] != art[at(4, 1)] and bunched[at(1, 4)] == art[at(1, 4)]
    assert c["FX-VEGAS-005"]["0"] != even
    assert c["FX-VEGAS-006"]["0"][at(1, 4)] != art[at(1, 4)]
    turn = c["FX-VEGAS-007"]
    assert same(turn["0"], bunched) and same(turn["4"], bunched, 1e-6) and turn["2"] != bunched
    assert c["FX-VEGAS-009"]["0"] != c["FX-VEGAS-008"]["0"] != c["FX-VEGAS-010"]["0"]
    assert c["FX-VEGAS-011"]["0"] == c["FX-VEGAS-008"]["0"]
    alone, under, stencil = (c[f"FX-VEGAS-0{n}"]["0"] for n in (12, 13, 14))
    assert alone[at(8, 4)] == clear and near(alone[at(2, 1)], white, 0.07)
    assert under[at(5, 1)] == art[at(5, 1)] and under[at(13, 4)] != art[at(13, 4)]
    assert stencil[at(8, 4)] == clear and stencil[at(15, 4)] == clear
    red = c["FX-VEGAS-015"]["0"]
    assert red[at(3, 2)][0] > red[at(3, 2)][2]
    hard = c["FX-VEGAS-016"]["0"]
    assert hard != one
    whole = c["FX-VEGAS-017"]["0"]
    assert near(whole[at(3, 1)], white) and near(whole[at(4, 1)], white)
    for fx in ("FX-VEGAS-018", "FX-VEGAS-019", "FX-VEGAS-020", "FX-VEGAS-021"):
        assert c[fx]["0"] != one, fx
    dip = c["FX-VEGAS-019"]["0"]
    assert dip[at(6, 1)][0] < dip[at(3, 1)][0]
    assert c["FX-VEGAS-022"]["0"] == art and c["FX-VEGAS-023"]["0"] == art
    assert all(p == clear for p in c["FX-VEGAS-024"]["0"])
    long = c["FX-VEGAS-025"]["0"]
    assert long[at(1, 4)] == art[at(1, 4)] and long[at(13, 4)] != art[at(13, 4)]
    assert c["FX-VEGAS-026"]["0"] == one
    assert c["FX-VEGAS-027"]["0"][at(8, 5)] == art[at(8, 5)]
    small = c["FX-VEGAS-028"]["0"]
    assert small[at(8, 1)] == art[at(8, 1)] and small[at(5, 3)] != art[at(5, 3)]
    moving = c["FX-VEGAS-029"]
    assert moving["0"] == one and moving["4"] != one
    moved = c["FX-VEGAS-030"]["0"]
    assert all(moved[at(x, y)] == one[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert c["FX-VEGAS-031"]["0"] == one
    cut = c["FX-VEGAS-032"]["0"]
    assert cut[at(0, 4)] != art[at(0, 4)] and cut[at(8, 4)] == art[at(8, 4)]
    more = c["FX-VEGAS-033"]
    assert more["0"] != more["2"] != more["4"]
    assert c["FX-VEGAS-034"]["0"] == alone
    roof = c["FX-VEGAS-035"]["0"]
    assert roof[at(8, 7)] == clear
    assert c["FX-VEGAS-036"]["0"][at(8, 5)] == clear
    assert c["FX-VEGAS-037"]["0"] == render(case(masks=[none(SMALL)], blend_mode="transparent",
                                                 **FOUR), 0)
    for fx in list(MISSING) + list(INVALID):
        assert c[fx]["0"] == c[fx]["4"] == plain(case())
    print("checked")


if __name__ == "__main__":
    main()
