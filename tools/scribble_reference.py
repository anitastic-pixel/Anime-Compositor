"""Scribble, worked a second way.

D-442 adds `core.scribble`, after After Effects' Scribble (Generate): it scribbles over a closed
mask with lines that zigzag across it at an angle, and wiggles them over time. Adobe's After
Effects 6.0 tutorial "Using the Scribble effect with Auto-trace" (adobe.com/jp/tips/aft6scribble)
says it is "similar to Illustrator's Scribble Fill", that "To create fun, loopy lines, increase
the Curviness value in the Stroke Options controls", that the Wiggle controls animate it, and
"Here you choose the angle, color, and style of Scribble". Storyblocks' tutorial "Create a
Scribble Effect in After Effects" lists the controls: Scribble (Single Mask, All Masks, All Masks
Using Modes), Mask, Fill Type (Inside, Centered Edge, Inside Edge, Outside Edge, Left Edge, Right
Edge), Edge Options, Color, Opacity, Angle, Stroke Width, Curviness, Curviness Variation, Spacing,
Spacing Variation, Path Overlap ("how far lines spill past the mask edge (positive) or pull inside
it (negative)"), Path Overlap Variation, Start, End, Fill Paths Sequentially, Wiggle Type,
Wiggles/Second, Random Seed and Composite (On Original Image, On Transparent, Reveal Original
Image). Neither gives a formula or the values when added; the rule below is this program's own and
nothing is ported.

The rule. The masks are chosen as Path Stroke and Fill choose them (D-356, D-418): a mask is
usable when it is switched on and has two points or more, flattened as document 19 flattens it,
in the drawing's own space (moved by the corner of a buffer an effect above grew). Single Mask
takes the mask numbered `mask` (its floor); All Masks and All Masks Using Modes take every usable
mask. Asked for a mask and finding none, nothing is drawn, the layer is left as it is, and
EFFECT_PATH_MISSING is said every frame. A mask's feather, expansion and opacity are not used.

1. The lines run along d = (cos a, -sin a), a = `angle` in degrees (0 runs left to right, 90 runs
   up the screen). Each point is read as u = x cos a - y sin a along the lines and v = x sin a +
   y cos a across them, and turned back by x = u cos a + v sin a, y = -u sin a + v cos a.
2. A mask's own region on the line at v is a set of intervals of u:
   - Inside: the stretches inside it by the even-odd rule, an edge from (u0, v0) to (u1, v1)
     crossing where (v0 <= v) differs from (v1 <= v), at u0 + (v - v0) / (v1 - v0) (u1 - u0).
   - Centered Edge: the band within w / 2 of its outline, w = `edge_width`: the union over every
     edge of the round-ended band about it (its two end discs and the rectangle between).
   - Inside Edge: the band within w, inside it; Outside Edge: the band within w, outside it.
   - Right Edge and Left Edge: the band within w on that side of the outline as it is drawn. A
     mask drawn clockwise on the screen (twice its area, sum of x_i y_(i+1) - x_(i+1) y_i, 0 or
     more) has its inside on the right, so Right Edge is Inside Edge for it and Outside Edge
     otherwise, and Left Edge the other way about.
3. Groups. Single Mask is one group of its mask; All Masks makes each mask its own group; All
   Masks Using Modes is one group of every mask whose mode is not None, its regions joined first
   to last as D-77 joins coverage: the first in Subtract or Intersect starts from the whole
   buffer, any other from nothing; Add is the union, Subtract takes away, Intersect keeps the
   common part, Difference keeps either but not both; an inverted mask's region is the whole
   buffer less its own. In the other two the masks' modes and inversion are not used.
4. Lines. Over a group, lo and hi are the least and greatest v of its outlines, moved out by w for
   the edge types, and out to the buffer's own corners when the group can start from the whole
   buffer (Using Modes, with an inverted mask or a first mask in Subtract or Intersect). Line 0 is
   at v = lo + g_0 / 2 and line i at v_(i-1) + g_i while v <= hi, at most 100000 lines, with
   g_i = max(`spacing` + `spacing_variation` (2 r - 1), 0.5) for line i's random r.
5. Overlap. Each interval of a line, in the order of u, is lengthened at each end by `path_overlap`
   + `path_overlap_variation` (2 r - 1), its own r for each end (a negative value shortens it); an
   interval left with no length is dropped.
6. The pen. Line i is drawn towards increasing u when i is even and back when it is odd. Each
   interval is drawn straight. The first interval of line i joins the last of line i - 1 when
   that line has any, by a turn: a cubic from P0, where the last ended, to P3, where the next
   begins, with P1 = P0 + h t and P2 = P3 + h t, t line i - 1's direction along u, and h = c / 100
   2/3 |P3 - P0| for c = max(`curviness` + `curviness_variation` (2 r - 1), 0), r line i - 1's;
   the turn is the cubic's points at s = 1/8, 2/8 ... 1, each worked out as (1-s)^3 P0 +
   3 (1-s)^2 s P1 + 3 (1-s) s^2 P2 + s^3 P3. Otherwise the pen lifts and a new line begins. So a
   group is drawn as one or more open lines.
7. Random numbers. r = H / 2^32 for a 32-bit hash H of (seed, slot, kind, group, line, end), with
   seed the floor of `random_seed`, kind 1 for spacing, 2 for overlap (end 2k at the start of
   interval k and 2k + 1 at its end) and 3 for curviness, and group the group's place (0 for one
   group): H = m(seed), then H = m(H xor x) for each of slot, kind, group, line and end in turn,
   every number taken modulo 2^32, where m(x) is x ^= x >> 16, x *= 0x7feb352d, x ^= x >> 15,
   x *= 0x846ca68b, x ^= x >> 16, modulo 2^32. Wiggle: T is the layer's key time from its in
   point in seconds and f = T `wiggles_per_second`. Static, or 0 wiggles a second, is slot 0 at
   every time. Jumpy is slot floor(f + 1e-9). Smooth is slots s = floor(f) and s + 1 blended by
   e = q^2 (3 - 2q), q = f - s: r_s (1 - e) + r_(s+1) e.
8. Start and End. Per group, the group's open lines are one length, end to end in order, and
   trimmed to between `start` and `end` per cent of it (taken smaller first), as Path Stroke
   trims with Stroke Sequentially (a lift has no length). With `fill_paths_sequentially` on and
   All Masks, every group's lines are one length in the order of the masks instead.
9. The stroke: D-356's round brush, `stroke_width` across, hardness 100, `opacity`, drawn all
   along the trimmed lines in `color` (linear), the nearest piece at each pixel's centre winning,
   laid by `composite` as Path Stroke's paint style lays it. The layer never grows. A draft halves
   the edge width, stroke width, spacing, overlap and their variations as distances.

Settings, in Adobe's order: `scribble` single_mask (when added), all_masks or
all_masks_using_modes; `mask` 1 to 1000, 1; `fill_type` inside (when added), centered_edge,
inside_edge, outside_edge, left_edge or right_edge; `edge_width` 0 to 1000 pixels, 10; `color`
`#rrggbb`, `#ffffff`; `opacity` 0 to 100, 100; `angle` -3600 to 3600 degrees, 45; `stroke_width`
0 to 1000 pixels, 5; `curviness` 0 to 100, 5; `curviness_variation` 0 to 100, 1; `spacing` 1 to
1000 pixels, 5; `spacing_variation` 0 to 1000 pixels, 0.5; `path_overlap` -1000 to 1000 pixels, 0;
`path_overlap_variation` 0 to 1000 pixels, 2; `start` and `end` 0 to 100, 0 and 100;
`fill_paths_sequentially` off or on (when added); `wiggle_type` static, jumpy or smooth (when
added); `wiggles_per_second` 0 to 100, 5; `random_seed` 0 to 100000, 1; `composite` on_original
(when added), on_transparent or reveal. The numbers are keyable. The values when added are chosen
here. After Effects' End Cap, Join and Miter Limit are not offered: the brush is round.

**This file never runs the build's code path.** It works every line, interval, turn and pixel in
double precision on lists, straight from the drawing's 8-bit values.

Every case is a composition 16 pixels by 10 at 24 frames a second, 12 frames long, holding
Gradient's cel the same size, unmoved unless the case says, with masks of mode None (which leave
the drawing as it is) unless the case says; the cases with other modes draw On Transparent, where
the layer under the stroke is not seen. The expected frames are in
`Fixtures/scribble/expected_scribble.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/scribble_reference.py
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
from gradient_reference import DRAWINGS  # noqa: E402
from mask_reference import flatten, points_at, points, mask_json  # noqa: E402

W, H = K.W, K.H
FPS = 24
FRAMES = 12
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "scribble"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"mask": (1, 1000), "edge_width": (0, 1000), "opacity": (0, 100),
          "angle": (-3600, 3600), "stroke_width": (0, 1000), "curviness": (0, 100),
          "curviness_variation": (0, 100), "spacing": (1, 1000), "spacing_variation": (0, 1000),
          "path_overlap": (-1000, 1000), "path_overlap_variation": (0, 1000), "start": (0, 100),
          "end": (0, 100), "wiggles_per_second": (0, 100), "random_seed": (0, 100000)}
NAMES = ("scribble", "mask", "fill_type", "edge_width", "color", "opacity", "angle",
         "stroke_width", "curviness", "curviness_variation", "spacing", "spacing_variation",
         "path_overlap", "path_overlap_variation", "start", "end", "fill_paths_sequentially",
         "wiggle_type", "wiggles_per_second", "random_seed", "composite")
WORDS = ("scribble", "fill_type", "color", "fill_paths_sequentially", "wiggle_type", "composite")
RED = "#ff3020"
EMPTY = [0.0] * 4
LINES = 100000
M32 = 0xFFFFFFFF


# --- the random numbers ---------------------------------------------------------------------

def mix(x):
    x &= M32
    x ^= x >> 16
    x = (x * 0x7FEB352D) & M32
    x ^= x >> 15
    x = (x * 0x846CA68B) & M32
    x ^= x >> 16
    return x


def rnd(seed, slot, kind, g, i, j):
    h = mix(seed)
    for v in (slot, kind, g, i, j):
        h = mix(h ^ (v & M32))
    return h / 4294967296.0


def wiggled(n, kind, g, i, j):
    seed = math.floor(n["random_seed"])
    wps = n["wiggles_per_second"]
    if n["wiggle_type"] == "static" or wps == 0:
        return rnd(seed, 0, kind, g, i, j)
    f = n["time"] * wps
    if n["wiggle_type"] == "jumpy":
        return rnd(seed, math.floor(f + 1e-9), kind, g, i, j)
    s = math.floor(f)
    q = f - s
    e = q * q * (3 - 2 * q)
    return rnd(seed, s, kind, g, i, j) * (1 - e) + rnd(seed, s + 1, kind, g, i, j) * e


def varied(n, base, spread, kind, g, i, j):
    return n[base] + n[spread] * (2 * wiggled(n, kind, g, i, j) - 1)


# --- the intervals --------------------------------------------------------------------------

def crossings(poly, v):
    """The even-odd inside of an outline (u, v) on the line at v, as intervals of u."""
    xs = []
    for k, (u0, v0) in enumerate(poly):
        u1, v1 = poly[(k + 1) % len(poly)]
        if (v0 <= v) != (v1 <= v):
            xs.append(u0 + (v - v0) / (v1 - v0) * (u1 - u0))
    xs.sort()
    return [(xs[k], xs[k + 1]) for k in range(0, len(xs) - 1, 2) if xs[k + 1] > xs[k]]


def capsule(a, b, r, v):
    """Where the line at v crosses the points within r of the piece a-b: (lo, hi) or None."""
    found = []
    for pu, pv in (a, b):
        dv = v - pv
        if dv * dv <= r * r:
            s = math.sqrt(r * r - dv * dv)
            found += [pu - s, pu + s]
    du, dw = b[0] - a[0], b[1] - a[1]
    length = math.hypot(du, dw)
    if length > 0:
        nu, nv = -dw / length * r, du / length * r
        quad = [(a[0] + nu, a[1] + nv), (b[0] + nu, b[1] + nv), (b[0] - nu, b[1] - nv),
                (a[0] - nu, a[1] - nv)]
        for k, (q0u, q0v) in enumerate(quad):
            q1u, q1v = quad[(k + 1) % 4]
            if (q0v - v) * (q1v - v) <= 0 and q0v != q1v:
                found.append(q0u + (v - q0v) / (q1v - q0v) * (q1u - q0u))
    return (min(found), max(found)) if found else None


def band(poly, r, v):
    pieces = []
    for k, a in enumerate(poly):
        c = capsule(a, poly[(k + 1) % len(poly)], r, v)
        if c is not None and c[1] > c[0]:
            pieces.append(c)
    pieces.sort()
    out = []
    for lo, hi in pieces:
        if out and lo <= out[-1][1]:
            out[-1] = (out[-1][0], max(out[-1][1], hi))
        else:
            out.append((lo, hi))
    return out


def combine(a, b, op):
    """The intervals where op(in a, in b) holds."""
    xs = sorted(set(x for iv in a + b for x in iv))
    out = []
    for x0, x1 in zip(xs, xs[1:]):
        m = (x0 + x1) / 2
        if op(any(lo < m < hi for lo, hi in a), any(lo < m < hi for lo, hi in b)):
            if out and out[-1][1] == x0:
                out[-1] = (out[-1][0], x1)
            else:
                out.append((x0, x1))
    return out


AND = lambda p, q: p and q  # noqa: E731
BUT = lambda p, q: p and not q  # noqa: E731
OPS = {"add": lambda p, q: p or q, "subtract": BUT, "intersect": AND,
       "difference": lambda p, q: p != q}


def clockwise(poly):
    return sum(poly[k][0] * poly[(k + 1) % len(poly)][1] - poly[(k + 1) % len(poly)][0]
               * poly[k][1] for k in range(len(poly))) >= 0


def region(poly, kind, w, v):
    inside = crossings(poly, v)
    if kind == "inside":
        return inside
    if kind == "centered_edge":
        return band(poly, w / 2, v)
    if kind in ("left_edge", "right_edge"):
        kind = "inside_edge" if (kind == "right_edge") == clockwise(poly) else "outside_edge"
    return combine(band(poly, w, v), inside, AND if kind == "inside_edge" else BUT)


# --- the lines ------------------------------------------------------------------------------

def group_lines(n, g, group, box, using_modes):
    """A group's open lines (lists of (x, y)), `group` its masks as (outline in u, v, mode,
    inverted)."""
    c, s = math.cos(math.radians(n["angle"])), math.sin(math.radians(n["angle"]))
    edge = n["fill_type"] != "inside"
    w = n["edge_width"]
    vs = [v for poly, _, _ in group for _, v in poly]
    if not vs:
        return []
    lo, hi = min(vs) - (w if edge else 0), max(vs) + (w if edge else 0)
    full = using_modes and (any(inv for _, _, inv in group)
                            or group[0][1] in ("subtract", "intersect"))
    if full:
        lo, hi = min(lo, min(v for _, v in box)), max(hi, max(v for _, v in box))

    def line(v):
        if not using_modes:
            return region(group[0][0], n["fill_type"], w, v)
        acc = None
        for poly, mode, inv in group:
            r = region(poly, n["fill_type"], w, v)
            if inv:
                r = combine(crossings(box, v), r, BUT)
            if acc is None:
                acc = crossings(box, v) if mode in ("subtract", "intersect") else []
            acc = combine(acc, r, OPS[mode])
        return acc

    out, cur, last = [], None, None
    i = 0
    v = lo + max(varied(n, "spacing", "spacing_variation", 1, g, 0, 0), 0.5) / 2
    while v <= hi and i < LINES:
        ivs = []
        for k, (a, b) in enumerate(line(v)):
            a -= varied(n, "path_overlap", "path_overlap_variation", 2, g, i, 2 * k)
            b += varied(n, "path_overlap", "path_overlap_variation", 2, g, i, 2 * k + 1)
            if b > a:
                ivs.append((a, b))
        forward = i % 2 == 0
        if not forward:
            ivs = [(b, a) for a, b in reversed(ivs)]
        for k, (a, b) in enumerate(ivs):
            if k == 0 and cur is not None and last == i - 1:
                p0, p3 = cur[-1], (a, v)
                t = 1.0 if (i - 1) % 2 == 0 else -1.0
                cv = max(varied(n, "curviness", "curviness_variation", 3, g, i - 1, 0), 0)
                h = cv / 100 * (2 / 3) * math.hypot(p3[0] - p0[0], p3[1] - p0[1])
                p1, p2 = (p0[0] + h * t, p0[1]), (p3[0] + h * t, p3[1])
                for m in range(1, 9):
                    q = m / 8
                    mq = 1 - q
                    b0, b1, b2, b3 = mq * mq * mq, 3 * mq * mq * q, 3 * mq * q * q, q * q * q
                    cur.append((b0 * p0[0] + b1 * p1[0] + b2 * p2[0] + b3 * p3[0],
                                b0 * p0[1] + b1 * p1[1] + b2 * p2[1] + b3 * p3[1]))
                cur.append((b, v))
            else:
                if cur is not None:
                    out.append(cur)
                cur = [(a, v), (b, v)]
        if ivs:
            last = i
        i += 1
        v += max(varied(n, "spacing", "spacing_variation", 1, g, i, 0), 0.5)
    if cur is not None:
        out.append(cur)
    return [[(u * c + vv * s, -u * s + vv * c) for u, vv in p] for p in out]


def scribble_lines(n, masks, size, origin):
    """Every group's open lines in the buffer's pixels, each group a list. `masks` are (outline,
    mode, inverted) as compose chose them."""
    c, s = math.cos(math.radians(n["angle"])), math.sin(math.radians(n["angle"]))
    uv = lambda x, y: (x * c - y * s, x * s + y * c)  # noqa: E731
    bw, bh = size
    box = [uv(0, 0), uv(bw, 0), uv(bw, bh), uv(0, bh)]
    chosen = [([uv(x + origin[0], y + origin[1]) for x, y in o], mode, inv)
              for o, mode, inv in masks]
    if n["scribble"] == "all_masks":
        groups = [[m] for m in chosen]
    elif n["scribble"] == "all_masks_using_modes":
        groups = [[m for m in chosen if m[1] != "none"]]
    else:
        groups = [chosen]
    using = n["scribble"] == "all_masks_using_modes"
    return [group_lines(n, g, grp, box, using) for g, grp in enumerate(groups)]


# --- the trim and the brush -----------------------------------------------------------------

def trimmed(paths, start, end):
    """Path Stroke's runs laid all along, the open `paths` one length end to end."""
    s, e = min(start, end) / 100, max(start, end) / 100
    pieces = [[(p[k], p[k + 1]) for k in range(len(p) - 1)] for p in paths]
    length = lambda ab: math.hypot(ab[1][0] - ab[0][0], ab[1][1] - ab[0][1])  # noqa: E731
    lengths = [sum(length(ab) for ab in pc) for pc in pieces]
    total = sum(lengths)
    before, out = 0.0, []
    for pc, ln in zip(pieces, lengths):
        lo, hi = max(s * total - before, 0.0), min(e * total - before, ln)
        before += ln
        if lo >= hi:
            continue
        at = 0.0
        for a, b in pc:
            m = length((a, b))
            u0, u1 = max(lo, at), min(hi, at + m)
            if m > 0 and u0 < u1:
                f0, f1 = (u0 - at) / m, (u1 - at) / m
                out.append(((a[0] + f0 * (b[0] - a[0]), a[1] + f0 * (b[1] - a[1])),
                            (a[0] + f1 * (b[0] - a[0]), a[1] + f1 * (b[1] - a[1]))))
            at += m
    return out


def runs_of(n, groups):
    if n["fill_paths_sequentially"] == "on" and n["scribble"] == "all_masks":
        return trimmed([p for grp in groups for p in grp], n["start"], n["end"])
    return [r for grp in groups for r in trimmed(grp, n["start"], n["end"])]


def nearest(runs, x, y):
    d = math.inf
    for (px, py), (qx, qy) in runs:
        dx, dy = qx - px, qy - py
        l2 = dx * dx + dy * dy
        t = 0.0 if l2 == 0 else min(1.0, max(0.0, ((x - px) * dx + (y - py) * dy) / l2))
        d = min(d, math.hypot(x - px - t * dx, y - py - t * dy))
    return d


def stroke(layer, runs, n, color, style):
    C = [srgb_to_linear(v / 255) for v in R.hex_color(color.lower())]
    r = n["stroke_width"] / 2
    if n["stroke_width"] == 0 or n["opacity"] == 0:
        runs = []
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            p = layer["px"][j * layer["w"] + i]
            d = nearest(runs, i + 0.5, j + 0.5)
            t = min(1.0, max(0.0, (r + 0.5 - d) / 1.0))
            k = n["opacity"] / 100 * t * t * (3 - 2 * t)
            if style == "on_original":
                px.append([p[ch] * (1 - k) + C[ch] * k for ch in range(3)] + [p[3] * (1 - k) + k])
            elif style == "on_transparent":
                px.append([C[ch] * k for ch in range(3)] + [k])
            else:
                px.append([v * k for v in p])
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def none(pts, **more):
    return {"points": pts, "mode": "none", **more}


BOX = points([2, 2], [13, 2], [13, 7], [2, 7])             # drawn clockwise on the screen
BACK = points([2, 7], [13, 7], [13, 2], [2, 2])            # the same box drawn the other way
SMALL = points([5, 4], [9, 4], [9, 6], [5, 6])
LEFT = points([1, 1], [8, 1], [8, 6], [1, 6])
RIGHT = points([6, 4], [14, 4], [14, 9], [6, 9])           # overlaps LEFT in (6..8, 4..6)
VEE = points([1, 1], [8, 8], [15, 1], [8, 5])              # a concave arrow head
C4 = 0.5522847498307936
RING = points([8, 1], [12, 5], [8, 9], [4, 5], handles=[
    ([-4 * C4, 0], [4 * C4, 0]), ([0, -4 * C4], [0, 4 * C4]),
    ([4 * C4, 0], [-4 * C4, 0]), ([0, 4 * C4], [0, -4 * C4])])
LINE = points([2, 2], [13, 7])                             # two points: encloses nothing


def case(masks=None, shift=0, **more):
    c = {"drawing": "cel", "masks": [none(BOX)] if masks is None else masks, "shift": shift,
         "scribble": "single_mask", "mask": 1, "fill_type": "inside", "edge_width": 2,
         "color": RED, "opacity": 100, "angle": 0, "stroke_width": 1, "curviness": 0,
         "curviness_variation": 0, "spacing": 2.5, "spacing_variation": 0, "path_overlap": 0,
         "path_overlap_variation": 0, "start": 0, "end": 100, "fill_paths_sequentially": "on",
         "wiggle_type": "static", "wiggles_per_second": 6, "random_seed": 1,
         "composite": "on_original"}
    c.update(more)
    return c


ADDED = dict(scribble="single_mask", mask=1, fill_type="inside", edge_width=10, color="#ffffff",
             opacity=100, angle=45, stroke_width=5, curviness=5, curviness_variation=1, spacing=5,
             spacing_variation=0.5, path_overlap=0, path_overlap_variation=2, start=0, end=100,
             fill_paths_sequentially="on", wiggle_type="smooth", wiggles_per_second=5,
             random_seed=1, composite="on_original")


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


MISSING_PATH = "missing"


def masks_of(c, frame_no):
    """The chosen (outline, mode, inverted); MISSING_PATH when there is none."""
    usable = lambda m, pts: m.get("enabled", True) and len(pts) >= 2  # noqa: E731
    at = [(m, points_at(m, frame_no)) for m in c["masks"]]
    if c["scribble"] == "single_mask":
        i = math.floor(held(c, "mask", frame_no)) - 1
        chosen = [at[i]] if i < len(at) and usable(*at[i]) else []
    else:
        chosen = [(m, pts) for m, pts in at if usable(m, pts)]
    return [(flatten(pts), m.get("mode", "add"), m.get("inverted", False))
            for m, pts in chosen] or MISSING_PATH


def layer_of(c):
    return {"px": [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
            "left": 0, "top": 0, "w": W, "h": H}


def settings(c, frame_no):
    n = {k: held(c, k, frame_no) for k in RANGES}
    n.update({k: c[k] for k in WORDS})
    n["time"] = frame_no / FPS
    return n


def render(c, frame_no):
    masks = masks_of(c, frame_no)
    if masks == MISSING_PATH:
        return plain(c)
    n = settings(c, frame_no)
    groups = scribble_lines(n, masks, (W, H), (0, 0))
    out = stroke(layer_of(c), runs_of(n, groups), n, c["color"], c["composite"])
    return frame(out, c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


TWO = [none(BOX), none(SMALL)]
PAIR = [none(LEFT), none(RIGHT)]
WOBBLE = dict(spacing_variation=1, path_overlap_variation=1.5, curviness=40,
              curviness_variation=30)


def modes(second, inverted=False, first="add"):
    return [{"points": LEFT, "mode": first}, {"points": RIGHT, "mode": second,
                                               "inverted": inverted}]


T = dict(composite="on_transparent")

CASES = {
    "FX-SCRIBBLE-001": ("The settings as added on the box from (2, 2) to (13, 7): white lines "
                        "5 wide at 45 degrees, 5 apart, a little curved and wiggling, "
                        "covering most of the box.", case(**ADDED), [0, 4]),
    "FX-SCRIBBLE-002": ("Angle 0, Stroke Width 1, Spacing 2.5, no curve or wiggle: two red "
                        "lines across the box at y 3.25 and 5.75, joined at the right by a "
                        "straight turn, a Z laid on its side.", case(), [0]),
    "FX-SCRIBBLE-003": ("FX-SCRIBBLE-002 at Angle 90: the lines run up the box, 2.5 apart from "
                        "its left side.", case(angle=90), [0]),
    "FX-SCRIBBLE-004": ("FX-SCRIBBLE-002 at Angle 30: sloped lines, each cut where it leaves the "
                        "box.", case(angle=30), [0]),
    "FX-SCRIBBLE-005": ("FX-SCRIBBLE-002 at Angle -45.", case(angle=-45), [0]),
    "FX-SCRIBBLE-006": ("FX-SCRIBBLE-002 with Curviness 100: each turn a round loop out past "
                        "the box's side.", case(curviness=100), [0]),
    "FX-SCRIBBLE-007": ("FX-SCRIBBLE-002 with Spacing 1.5: four lines, closer.",
                        case(spacing=1.5), [0]),
    "FX-SCRIBBLE-008": ("FX-SCRIBBLE-002 with Stroke Width 2: thicker lines.",
                        case(stroke_width=2), [0]),
    "FX-SCRIBBLE-009": ("FX-SCRIBBLE-002 with Path Overlap 1.5: each line runs 1.5 past the box "
                        "at both ends.", case(path_overlap=1.5), [0]),
    "FX-SCRIBBLE-010": ("FX-SCRIBBLE-002 with Path Overlap -3: each line pulled 3 inside the "
                        "box at both ends.", case(path_overlap=-3), [0]),
    "FX-SCRIBBLE-011": ("FX-SCRIBBLE-002 with Opacity 50: half covered at most.",
                        case(opacity=50), [0]),
    "FX-SCRIBBLE-012": ("FX-SCRIBBLE-002 On Transparent: the lines alone, the cel gone.",
                        case(**T), [0]),
    "FX-SCRIBBLE-013": ("FX-SCRIBBLE-002 Reveal Original Image: the cel only under the lines.",
                        case(composite="reveal"), [0]),
    "FX-SCRIBBLE-014": ("FX-SCRIBBLE-002 with Stroke Width 0: nothing drawn, the cel as it "
                        "was.", case(stroke_width=0), [0]),
    "FX-SCRIBBLE-015": ("FX-SCRIBBLE-002 with End 50: only the first half of the scribble's "
                        "length.", case(end=50), [0]),
    "FX-SCRIBBLE-016": ("Start 30 and End 80 at Spacing 1.5: the middle half.",
                        case(spacing=1.5, start=30, end=80), [0]),
    "FX-SCRIBBLE-017": ("End keyed from 0 at frame 0 to 100 at frame 8: the scribble draws on, "
                        "nothing at frame 0, the whole at frame 8.",
                        case(spacing=1.5, end=keyed((0, 0), (8, 100))), [0, 4, 8]),
    "FX-SCRIBBLE-018": ("Fill Type Centered Edge, Edge Width 2: lines over the band 1 each side "
                        "of the box's outline, a gap in the middle.",
                        case(fill_type="centered_edge", spacing=1.5), [0]),
    "FX-SCRIBBLE-019": ("Fill Type Inside Edge, Edge Width 2: the band 2 inside the outline.",
                        case(fill_type="inside_edge", spacing=1.5), [0]),
    "FX-SCRIBBLE-020": ("Fill Type Outside Edge, Edge Width 1.5: the band 1.5 outside the "
                        "outline.", case(fill_type="outside_edge", edge_width=1.5, spacing=1.5),
                        [0]),
    "FX-SCRIBBLE-021": ("Right Edge on the box drawn clockwise: FX-SCRIBBLE-019's frame.",
                        case(fill_type="right_edge", spacing=1.5), [0]),
    "FX-SCRIBBLE-022": ("Left Edge on the box drawn clockwise: the outside band, as Outside "
                        "Edge at Edge Width 2.", case(fill_type="left_edge", spacing=1.5), [0]),
    "FX-SCRIBBLE-023": ("Left Edge on the box drawn the other way: the inside band, "
                        "FX-SCRIBBLE-019's frame.",
                        case(masks=[none(BACK)], fill_type="left_edge", spacing=1.5), [0]),
    "FX-SCRIBBLE-024": ("A curved mask, a circle 8 across: the lines cut at its curve.",
                        case(masks=[none(RING)], spacing=2), [0]),
    "FX-SCRIBBLE-025": ("A concave arrow head: on the lines that cross it twice the pen lifts "
                        "over the notch.", case(masks=[none(VEE)], spacing=1.5), [0]),
    "FX-SCRIBBLE-026": ("Two masks, Mask 2: the small box alone.", case(masks=TWO, mask=2), [0]),
    "FX-SCRIBBLE-027": ("Two overlapping boxes, All Masks, Fill Paths Sequentially on, End 50: "
                        "each box scribbled on its own, one length, so only the first box's "
                        "lines and part of the second's.",
                        case(masks=PAIR, scribble="all_masks", end=50), [0]),
    "FX-SCRIBBLE-028": ("FX-SCRIBBLE-027 with Fill Paths Sequentially off: each box trimmed to "
                        "its own half.", case(masks=PAIR, scribble="all_masks", end=50,
                                              fill_paths_sequentially="off"), [0]),
    "FX-SCRIBBLE-029": ("All Masks Using Modes, the second box Add: one region, both boxes "
                        "together, On Transparent.",
                        case(masks=modes("add"), scribble="all_masks_using_modes", **T), [0]),
    "FX-SCRIBBLE-030": ("Using Modes, the second box Subtract: the first box less the second.",
                        case(masks=modes("subtract"), scribble="all_masks_using_modes", **T),
                        [0]),
    "FX-SCRIBBLE-031": ("Using Modes, the second box Intersect: their common part only.",
                        case(masks=modes("intersect"), scribble="all_masks_using_modes",
                             spacing=1, **T), [0]),
    "FX-SCRIBBLE-032": ("Using Modes, the second box Difference: either but not both.",
                        case(masks=modes("difference"), scribble="all_masks_using_modes", **T),
                        [0]),
    "FX-SCRIBBLE-033": ("Using Modes, the second box Add and inverted: the first box and "
                        "everything outside the second.",
                        case(masks=modes("add", True), scribble="all_masks_using_modes", **T),
                        [0]),
    "FX-SCRIBBLE-034": ("Using Modes with a single mask in Subtract: the whole buffer less the "
                        "box.", case(masks=[{"points": BOX, "mode": "subtract"}],
                                     scribble="all_masks_using_modes", **T), [0]),
    "FX-SCRIBBLE-035": ("Using Modes, the second mask of mode None: left out, the first box "
                        "alone.", case(masks=modes("none"), scribble="all_masks_using_modes",
                                       **T), [0]),
    "FX-SCRIBBLE-036": ("Single Mask on an inverted mask: inversion is not used outside Using "
                        "Modes, FX-SCRIBBLE-002's frame.",
                        case(masks=[none(BOX, inverted=True)]), [0]),
    "FX-SCRIBBLE-037": ("Variations on, Wiggle Type Static: uneven spacing, overshoots and "
                        "loops, the same at every frame.", case(**WOBBLE), [0, 4]),
    "FX-SCRIBBLE-038": ("The same, Jumpy at 6 a second: a new scribble every 4 frames, frames "
                        "0 and 2 alike, 4 new.", case(wiggle_type="jumpy", **WOBBLE), [0, 2, 4]),
    "FX-SCRIBBLE-039": ("The same, Smooth: frame 2 half way between frames 0 and 4's "
                        "scribbles.", case(wiggle_type="smooth", **WOBBLE), [0, 2, 4]),
    "FX-SCRIBBLE-040": ("FX-SCRIBBLE-037 with Random Seed 7: a different scribble.",
                        case(random_seed=7, **WOBBLE), [0]),
    "FX-SCRIBBLE-041": ("Smooth at 0 wiggles a second: still, every frame FX-SCRIBBLE-037's.",
                        case(wiggle_type="smooth", wiggles_per_second=0, **WOBBLE), [0, 4]),
    "FX-SCRIBBLE-042": ("FX-SCRIBBLE-002 with the layer moved three pixels right: the same, "
                        "moved.", case(shift=3), [0]),
    "FX-SCRIBBLE-043": ("Mask 1.5: its floor, mask 1, FX-SCRIBBLE-002's frame.", case(mask=1.5),
                        [0]),
    "FX-SCRIBBLE-044": ("A mask of two points encloses nothing: Inside draws nothing; Centered "
                        "Edge, FX-SCRIBBLE-045, scribbles the band about it.",
                        case(masks=[none(LINE)]), [0]),
    "FX-SCRIBBLE-045": ("The two-point mask, Centered Edge, Edge Width 2.",
                        case(masks=[none(LINE)], fill_type="centered_edge", spacing=1), [0]),
    "FX-SCRIBBLE-046": ("Angle keyed from 0 at frame 0 to 90 at frame 4: the lines turn.",
                        case(angle=keyed((0, 0), (4, 90))), [0, 2, 4]),
    "FX-SCRIBBLE-047": ("Opacity keyed from 40 to 100 by an ease that passes its end: held at "
                        "100 at frame 2.", case(opacity=keyed((0, 40, OVERSHOOT), (4, 100))),
                        [0, 2, 4]),
    "FX-SCRIBBLE-048": ("Colour written in capitals, #FF3020: FX-SCRIBBLE-002's frame.",
                        case(color="#FF3020"), [0]),
}

MISSING = {
    "FX-SCRIBBLE-049": ("Mask 1 with no masks at all.", case(masks=[])),
    "FX-SCRIBBLE-050": ("Mask 3, of two.", case(masks=TWO, mask=3)),
    "FX-SCRIBBLE-051": ("Mask 1, switched off.", case(masks=[none(BOX, enabled=False)])),
    "FX-SCRIBBLE-052": ("All Masks, every mask switched off.",
                        case(masks=[none(BOX, enabled=False)], scribble="all_masks")),
    "FX-SCRIBBLE-053": ("All Masks Using Modes with no masks at all.",
                        case(masks=[], scribble="all_masks_using_modes")),
}

INVALID = {
    "FX-SCRIBBLE-054": ("Mask 0, below 1.", case(mask=0)),
    "FX-SCRIBBLE-055": ("Edge Width -1, below 0.", case(edge_width=-1)),
    "FX-SCRIBBLE-056": ("Opacity 101, above 100.", case(opacity=101)),
    "FX-SCRIBBLE-057": ("Angle 3601, above 3600.", case(angle=3601)),
    "FX-SCRIBBLE-058": ("Stroke Width 1001, above 1000.", case(stroke_width=1001)),
    "FX-SCRIBBLE-059": ("Curviness 101, above 100.", case(curviness=101)),
    "FX-SCRIBBLE-060": ("Spacing 0.5, below 1.", case(spacing=0.5)),
    "FX-SCRIBBLE-061": ("Path Overlap -1001, below -1000.", case(path_overlap=-1001)),
    "FX-SCRIBBLE-062": ("End 101, above 100.", case(end=101)),
    "FX-SCRIBBLE-063": ("Wiggles/Second -1, below 0.", case(wiggles_per_second=-1)),
    "FX-SCRIBBLE-064": ("Random Seed 100001, above 100000.", case(random_seed=100001)),
    "FX-SCRIBBLE-065": ("Scribble \"some_masks\", not a word it takes.",
                        case(scribble="some_masks")),
    "FX-SCRIBBLE-066": ("Fill Type \"outline\", not a word it takes.", case(fill_type="outline")),
    "FX-SCRIBBLE-067": ("Fill Paths Sequentially \"yes\", not off or on.",
                        case(fill_paths_sequentially="yes")),
    "FX-SCRIBBLE-068": ("Wiggle Type \"wobbly\", not a word it takes.",
                        case(wiggle_type="wobbly")),
    "FX-SCRIBBLE-069": ("Composite \"glow\", not a word it takes.", case(composite="glow")),
    "FX-SCRIBBLE-070": ("Colour \"#12345\", not #rrggbb.", case(color="#12345")),
    "FX-SCRIBBLE-071": ("Stroke Width keyed to 2000 at frame 4.",
                        case(stroke_width=keyed((0, 1), (4, 2000)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    comp["duration_frames"] = FRAMES
    comp["work_area"] = {"start_frame": 0, "end_frame_exclusive": FRAMES}
    assert comp["frame_rate"] == {"numerator": FPS, "denominator": 1}
    layer = comp["layers"][0]
    layer["out_frame"] = FRAMES
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
    layer["effects"] = [{"instance_id": "fx-0-0", "type_id": "core.scribble", "enabled": True,
                         "parameters": {k: (c[k] if k in WORDS else setting_json(c[k]))
                                        for k in NAMES}}]
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
        if fx in ("FX-SCRIBBLE-044", "FX-SCRIBBLE-045"):
            # D-442's amendment, the owner's decision of 2026-10-10: a mask of two points is
            # warned of on opening and each frame, as every such mask is (document 19).
            expected["cases"][fx]["warning"] = "MASK_INVALID_OUTLINE"
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in MISSING.items():
        says += (" Nothing to scribble: the layer is drawn without the effect, which is kept as "
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
    (OUT / "expected_scribble.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    art = plain(case())
    red = [srgb_to_linear(v / 255) for v in R.hex_color(RED)]

    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(
                    -1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), (fx, p)
        # A refused or missing effect leaves the drawing as it was; a two-point mask's warning
        # (FX-SCRIBBLE-044 and 045) does not stop Centered Edge drawing its band.
        warned = expected["cases"][fx].get("warning") not in (None, "MASK_INVALID_OUTLINE")
        if warned or "frame_warning" in expected["cases"][fx]:
            assert all(px == art for px in frames.values()), fx

    # The geometry of FX-SCRIBBLE-002: two lines at y 3.25 and 5.75 joined at x 13.
    n = settings(case(), 0)
    groups = scribble_lines(n, masks_of(case(), 0), (W, H), (0, 0))
    assert len(groups) == 1 and len(groups[0]) == 1
    line = groups[0][0]
    assert near(line[0], (2, 3.25), 1e-9) and near(line[1], (13, 3.25), 1e-9)
    assert near(line[-1], (2, 5.75), 1e-9) and all(abs(x - 13) < 1e-9 for x, _ in line[1:-1])
    two = c["FX-SCRIBBLE-002"]["0"]
    p = c["FX-SCRIBBLE-012"]["0"][at(7, 3)]  # 0.25 from the line: covered 0.84375
    assert near(p, [v * 0.84375 for v in red] + [0.84375], 1e-9)
    assert two[at(7, 0)] == art[at(7, 0)] and two[at(7, 9)] == art[at(7, 9)]
    assert two[at(0, 4)] == art[at(0, 4)]
    # Curviness 100 loops out past x 13; Curviness 0 does not.
    assert c["FX-SCRIBBLE-006"]["0"][at(14, 4)] != art[at(14, 4)]
    assert two[at(14, 4)] == art[at(14, 4)]
    # Overlap runs past the box; a negative one pulls in.
    assert c["FX-SCRIBBLE-009"]["0"][at(0, 3)] != art[at(0, 3)]
    assert c["FX-SCRIBBLE-010"]["0"][at(3, 3)] == art[at(3, 3)]
    assert c["FX-SCRIBBLE-014"]["0"] == art
    assert all(p[3] == 0 or p[:3] != [0, 0, 0] for p in c["FX-SCRIBBLE-012"]["0"])
    assert c["FX-SCRIBBLE-012"]["0"][at(0, 0)] == EMPTY
    drawn = c["FX-SCRIBBLE-017"]
    assert drawn["0"] == art and drawn["4"] != art and drawn["8"] != drawn["4"]
    assert c["FX-SCRIBBLE-021"]["0"] == c["FX-SCRIBBLE-019"]["0"]
    assert c["FX-SCRIBBLE-023"]["0"] == c["FX-SCRIBBLE-019"]["0"]
    assert c["FX-SCRIBBLE-022"]["0"] == render(case(fill_type="outside_edge", spacing=1.5), 0)
    assert c["FX-SCRIBBLE-019"]["0"] != c["FX-SCRIBBLE-022"]["0"]
    # Centered edge leaves the box's middle alone.
    assert c["FX-SCRIBBLE-018"]["0"][at(7, 4)] == art[at(7, 4)]
    assert c["FX-SCRIBBLE-027"]["0"] != c["FX-SCRIBBLE-028"]["0"]
    assert c["FX-SCRIBBLE-029"]["0"] != c["FX-SCRIBBLE-030"]["0"] != c["FX-SCRIBBLE-031"]["0"]
    assert c["FX-SCRIBBLE-031"]["0"][at(2, 2)] == EMPTY
    assert c["FX-SCRIBBLE-034"]["0"][at(0, 0)] != EMPTY or c["FX-SCRIBBLE-034"]["0"][at(0, 1)] != EMPTY
    assert c["FX-SCRIBBLE-035"]["0"] == render(case(masks=[{"points": LEFT, "mode": "add"}],
                                                    scribble="all_masks_using_modes", **T), 0)
    assert c["FX-SCRIBBLE-036"]["0"] == two
    still = c["FX-SCRIBBLE-037"]
    assert still["0"] == still["4"] != render(case(), 0)
    jumpy = c["FX-SCRIBBLE-038"]
    assert jumpy["0"] == jumpy["2"] != jumpy["4"]
    smooth = c["FX-SCRIBBLE-039"]
    assert smooth["0"] != smooth["2"] != smooth["4"]
    assert c["FX-SCRIBBLE-040"]["0"] != still["0"]
    assert c["FX-SCRIBBLE-041"]["0"] == c["FX-SCRIBBLE-041"]["4"] == still["0"]
    moved = c["FX-SCRIBBLE-042"]["0"]
    assert all(moved[at(x, y)] == two[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert c["FX-SCRIBBLE-043"]["0"] == two and c["FX-SCRIBBLE-048"]["0"] == two
    assert c["FX-SCRIBBLE-044"]["0"] == art and c["FX-SCRIBBLE-045"]["0"] != art
    turn = c["FX-SCRIBBLE-046"]
    assert turn["0"] == two and turn["4"] == c["FX-SCRIBBLE-003"]["0"]
    eased = c["FX-SCRIBBLE-047"]
    assert eased["2"] == eased["4"] == two
    print("checked")


if __name__ == "__main__":
    main()
