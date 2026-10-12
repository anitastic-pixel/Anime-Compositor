"""Contour Paint, worked a second way.

D-470 adds `core.contour_paint`, after After Effects' Contour Paint (Stylize): the layer's
colours smeared along its own contours, so it looks painted in long strokes that follow the
shapes, with flat colour fills laid over if wanted. What is public (Adobe's beta announcement
"New in Beta: Contour Paint effect", August 2026, After Effects Beta 27.0 build 6 and later, and
Adobe's help page of Stylize effects) is: View is Final Result or Edge Map, and the Edge Map
shows the edges with cyan arrows whose direction and opacity show the flow's direction and
strength; Stylization is how exaggerated the smears are and how much they follow the larger
contours instead of the smaller details; Cleanliness is how much detail the smearing removes;
Smear Iterations runs the smear again, up to 100 times; Apply to Alpha smears the covering too;
Draw Color Fills is None, Over Smeared Image or Only; Reduction Method is K-Means, Median or Match
to Nearest Custom Color; Posterization Level is how many colours; Limit Fills by Edge Strength
has a Low and a High Strength; Offset Flow Angles turns the flow; Quantize Flow Angles snaps it
to steps, 0 for none; Freehand Flow Adjustments paints changes to the flow; External Edge Map
lets another layer's edges steer the smear. No version it ships in, no values, no ranges and no
formula are published. The rule below is this program's own, a coherence-enhancing smear along
the flow of the structure tensor (Kyprianidis and Kang's "Image and Video Abstraction by
Coherence-Enhancing Filtering", 2011, after Cabral and Leedom's line integral convolution,
1993), with D-183 Line Blur's tensor and tangent; nothing is ported.

The rule, at the holder's frame, with its input O, linear and premultiplied, w by h, and px the
size of a pixel (1 at Full, D-259's fraction at Draft):

1. The flow is read from a picture F: the layer's own drawing as it is at each pass, or, with
   `edge_map_layer` naming a layer (D-189's layer setting, centred on the drawing and lying on
   it), that layer once for all passes. A layer not in the composition is said, and the layer's
   own drawing is used.
2. l is at each pixel F's luminance as CC Vector Blur reads it: the encoded luminance of its
   straight colour, times its covering. Its slopes gx, gy are Sobel's over 8, each neighbour held
   inside the picture. The tensor (gx², gx gy, gy²) is blurred by D-109's held Gaussian of sigma
   (0.5 + 4.5 `cleanliness` / 100) px.
3. With the blurred tensor (a, b, c), r = sqrt((a - c)² + 4 b²) and the flow t is Line Blur's:
   (b, -(a - c + r) / 2) when a >= c, else (-(c - a + r) / 2, b), made one long, or none where
   its length is 0. Its strength is e = min(1, 4 sqrt(r)).
4. The flow is turned `offset_flow_angles` degrees clockwise, (x cos - y sin, x sin + y cos),
   exact at whole quarter turns, then, with `quantize_flow_angles` q > 0, snapped to the first of
   the directions (cos kq, sin kq), k = 0 to ceil(180 / q) - 1, whose |dot| with it is largest.
5. One pass of the smear: sigma s = (0.5 + 7.5 `stylization` / 100) px, n = ceil(2 s) steps. A
   pixel with a flow walks from its centre both ways, along t and along -t: each step one pixel
   along the way it faces, the point then held inside the pixel centres, the picture sampled there
   (document 21's bilinear sample of the premultiplied pixels) at weight exp(-k² / 2 s²) for step
   k, and the way it faces then becomes the bilinear mix of the flow at the four pixels round the
   point, each turned to face the same way first (negated where its dot with the way is below 0),
   made one long; the walk ends early where that mix's length is 1e-6 or less. S is the pixel and
   its samples, weighted (the pixel at weight 1), and the pass's result is O' = O + e (S - O).
   Without `apply_to_alpha` the covering is kept: each result pixel is its straight colour times
   the covering it had before the pass.
6. `smear_iterations` (floored) passes, each from the last one's result.
7. With `draw_color_fills` not `none`: the palette is made from O, sampled every g pixels across
   and down from the corner, g = max(1, floor(sqrt(w h / 4096))), at every sample with a covering,
   each the encoded straight colour. `median` is Heckbert's median cut to `posterization_level`
   (floored) boxes: while there are fewer, the box whose largest channel range is widest (the
   first on a tie, none when every range is 0) is sorted, stably, on that channel (the lowest on a
   tie) and cut in two at half its length rounded down, the upper half put after it; the palette is
   each box's mean. `k_means` starts from that palette and moves each colour to the mean of the
   samples nearest it, 8 times, a colour with none staying. Each pixel of the smeared picture with
   a covering takes the palette colour nearest its encoded straight colour (the first on a tie), at
   its covering, where the flow strength of the smeared picture (steps 1 to 3 read from it, or the
   edge-map layer) times 100 lies from `low_strength` to `high_strength`. `over_smeared_image`
   keeps the smeared pixel elsewhere; `only` leaves elsewhere clear.
   D-584: `match_to_nearest_custom_color` (After Effects' Match to Nearest Custom Color) takes
   the palette from `colors` instead, each `#rrggbb` its encoded colour, and nearest is the
   smallest CIE76 colour difference: the straight Euclidean distance in CIE L*a*b* (D-400's: the
   sRGB matrix of IEC 61966-2-1 and its own white, L* a* b* unscaled) between the custom colour
   taken to linear and the pixel's encoded straight colour taken back to linear, the first on a
   tie. Posterization Level is not used. With no colours nothing is filled.
8. View `edge_map` instead shows steps 1 to 4 read once from O (or the edge-map layer): each pixel
   opaque grey, its encoded value e, and over it in cells 12 px square from the corner a cyan
   (0, 1, 1) line through the cell's middle along the flow at the pixel nearest the middle (held
   in the picture), 0.4 of the cell long each way, laid on at that pixel's e times
   clamp(1 - d / px, 0, 1), d the distance from the pixel's centre to the line.

Nothing happens with View `final_result`, Smear Iterations 0 and Draw Color Fills `none`.

`view` `final_result` or `edge_map`, `final_result`; `stylization` 0 to 100, 50; `cleanliness`
0 to 100, 50; `smear_iterations` 0 to 100, 3; `apply_to_alpha` `off` or `on`, `off`;
`draw_color_fills` `none`, `over_smeared_image` or `only`, `none`; `reduction_method` `k_means`,
`median` or `match_to_nearest_custom_color` (D-584), `k_means`; `posterization_level` 2 to 32, 8;
`low_strength` and `high_strength` 0 to 100, 0 and 100; `offset_flow_angles` -3600 to 3600, 0;
`quantize_flow_angles` 0 to 90, 0; `edge_map_layer` D-189's word, ""; `colors` (D-584) D-87's
list of up to eight `#rrggbb`, none, a file without it having none, not keyable. Every number
keyable. Every value, range and constant here is chosen by this program; Freehand Flow
Adjustments, the arrows' heads, keys on the custom colour set and the External Edge Map's
placement are not built (gaps logged).

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawings' 8-bit values, where the build works in single precision.

Every case is a project of one composition 16 by 10, five frames, in `Fixtures/contour_paint/`:
the drawing `holder` at the top with the effect, and under it the map layers, every one switched
off. The expected pixels are in `Fixtures/contour_paint/expected_contour_paint.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/contour_paint_reference.py
"""

import json
import sys
from math import ceil, cos, exp, floor, radians, sin, sqrt
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from motion_blur_reference import png  # noqa: E402
from vector_blur_reference import phase  # noqa: E402
from blobbylize_reference import DRAWINGS, decoded, kernel  # noqa: E402
from adjust_reference import srgb_to_linear  # noqa: E402
from shadow_highlight_reference import lab  # noqa: E402
import smooth_reference as S  # noqa: E402
import effect_layer_reference as L  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "contour_paint"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 5
MAPS = ("dots", "ramp", "spot")


# --- the rule -------------------------------------------------------------------------------

def held(w, h, x, y):
    return min(max(x, 0), w - 1), min(max(y, 0), h - 1)


def flow(f, s, px=1.0):
    """Steps 2 to 4 on F: each pixel's flow (tx, ty) and strength e."""
    w, h = f["w"], f["h"]
    lum = [phase("luminance", p) for p in f["px"]]
    l = lambda x, y: lum[(lambda q: q[1] * w + q[0])(held(w, h, x, y))]  # noqa: E731
    t3 = []
    for y in range(h):
        for x in range(w):
            gx = (l(x + 1, y - 1) + 2 * l(x + 1, y) + l(x + 1, y + 1)
                  - l(x - 1, y - 1) - 2 * l(x - 1, y) - l(x - 1, y + 1)) / 8
            gy = (l(x - 1, y + 1) + 2 * l(x, y + 1) + l(x + 1, y + 1)
                  - l(x - 1, y - 1) - 2 * l(x, y - 1) - l(x + 1, y - 1)) / 8
            t3.append((gx * gx, gx * gy, gy * gy))
    r, k = kernel((0.5 + 4.5 * s["cleanliness"] / 100) * px)
    for axis in (0, 1):
        out = []
        for y in range(h):
            for x in range(w):
                acc = [0.0, 0.0, 0.0]
                for i, wt in enumerate(k):
                    qx, qy = held(w, h, x + (i - r if axis == 0 else 0), y + (i - r if axis == 1 else 0))
                    for c in range(3):
                        acc[c] += t3[qy * w + qx][c] * wt
                out.append(tuple(acc))
        t3 = out
    turn = s["offset_flow_angles"]
    cs = {0: (1.0, 0.0), 90: (0.0, 1.0), 180: (-1.0, 0.0), 270: (0.0, -1.0)}.get(
        turn % 360, (cos(radians(turn)), sin(radians(turn))))
    q = s["quantize_flow_angles"]
    ways = [(cos(radians(k_ * q)), sin(radians(k_ * q))) for k_ in range(ceil(180 / q))] if q > 0 else []
    field = []
    for a, b, c in t3:
        rr = sqrt((a - c) ** 2 + 4 * b * b)
        t = (b, -(a - c + rr) / 2) if a >= c else (-(c - a + rr) / 2, b)
        n = sqrt(t[0] ** 2 + t[1] ** 2)
        e = min(1.0, 4 * sqrt(rr))
        if n == 0:
            field.append((0.0, 0.0, e))
            continue
        t = (t[0] / n, t[1] / n)
        t = (t[0] * cs[0] - t[1] * cs[1], t[0] * cs[1] + t[1] * cs[0])
        if ways:
            best = max(range(len(ways)), key=lambda i: (abs(t[0] * ways[i][0] + t[1] * ways[i][1]), -i))
            t = ways[best]
        field.append((t[0], t[1], e))
    return field


def mixed_way(field, w, h, x, y, d):
    """The flow at (x, y), pixel centres at halves, each corner turned to face d."""
    fx, fy = x - 0.5, y - 0.5
    x0, y0 = floor(fx), floor(fy)
    ux, uy = fx - x0, fy - y0
    out = [0.0, 0.0]
    for dy, wy in ((0, 1 - uy), (1, uy)):
        for dx, wx in ((0, 1 - ux), (1, ux)):
            qx, qy = held(w, h, x0 + dx, y0 + dy)
            tx, ty, _ = field[qy * w + qx]
            sgn = -1.0 if tx * d[0] + ty * d[1] < 0 else 1.0
            out[0] += sgn * tx * wx * wy
            out[1] += sgn * ty * wx * wy
    return out


def smear_pass(o, field, s, px=1.0):
    w, h = o["w"], o["h"]
    sig = (0.5 + 7.5 * s["stylization"] / 100) * px
    n = ceil(2 * sig)
    keep = s["apply_to_alpha"] == "off"
    out = []
    for y in range(h):
        for x in range(w):
            p = o["px"][y * w + x]
            tx, ty, e = field[y * w + x]
            if tx == 0 and ty == 0:
                out.append(list(p))
                continue
            acc, total = list(p), 1.0
            for side in (1.0, -1.0):
                pt, d = (x + 0.5, y + 0.5), (side * tx, side * ty)
                for k_ in range(1, n + 1):
                    pt = (min(max(pt[0] + d[0], 0.5), w - 0.5), min(max(pt[1] + d[1], 0.5), h - 0.5))
                    wt = exp(-k_ * k_ / (2 * sig * sig))
                    smp = L.bilinear(o, *pt)
                    for c in range(4):
                        acc[c] += smp[c] * wt
                    total += wt
                    m = mixed_way(field, w, h, pt[0], pt[1], d)
                    ln = sqrt(m[0] ** 2 + m[1] ** 2)
                    if ln <= 1e-6:
                        break
                    d = (m[0] / ln, m[1] / ln)
            sm = [v / total for v in acc]
            res = [p[c] + e * (sm[c] - p[c]) for c in range(4)]
            if keep:
                a0 = p[3]
                res = [res[c] / res[3] * a0 if res[3] > 0 else 0.0 for c in range(3)] + [a0]
            out.append(res)
    return L.pic(w, h, out)


def palette(o, s):
    w, h = o["w"], o["h"]
    g = max(1, floor(sqrt(w * h / 4096)))
    samples = []
    for y in range(0, h, g):
        for x in range(0, w, g):
            p = o["px"][y * w + x]
            if p[3] > 0:
                samples.append([S.linear_to_srgb(min(max(p[c] / p[3], 0.0), 1.0)) for c in range(3)])
    if not samples:
        return []
    want = floor(s["posterization_level"])
    boxes = [samples]
    while len(boxes) < want:
        spans = [max(max(v[c] for v in b) - min(v[c] for v in b) for c in range(3)) for b in boxes]
        i = max(range(len(boxes)), key=lambda j: (spans[j], -j))
        if spans[i] <= 0:
            break
        b = boxes[i]
        ch = max(range(3), key=lambda c: (max(v[c] for v in b) - min(v[c] for v in b), -c))
        b = sorted(b, key=lambda v: v[ch])
        boxes[i:i + 1] = [b[:len(b) // 2], b[len(b) // 2:]]
    pal = [[sum(v[c] for v in b) / len(b) for c in range(3)] for b in boxes]
    if s["reduction_method"] == "k_means":
        for _ in range(8):
            sums = [[0.0, 0.0, 0.0, 0] for _ in pal]
            for v in samples:
                j = nearest(pal, v)
                for c in range(3):
                    sums[j][c] += v[c]
                sums[j][3] += 1
            pal = [[q[c] / q[3] for c in range(3)] if q[3] else old for q, old in zip(sums, pal)]
    return pal


def nearest(pal, v):
    return min(range(len(pal)), key=lambda j: (sum((pal[j][c] - v[c]) ** 2 for c in range(3)), j))


CUSTOM = "match_to_nearest_custom_color"


def cie_lab(v):
    """An encoded colour to unscaled CIE L*a*b* (D-400's matrix and white)."""
    l, p, q = lab([srgb_to_linear(c) for c in v])
    return [100 * l, 128 * p, 128 * q]


def custom(colors):
    """D-584's palette: each `#rrggbb` as its encoded colour."""
    return [[int(h[i:i + 2], 16) / 255 for i in (1, 3, 5)] for h in colors]


def edge_view(field, s, w, h, px=1.0):
    cell = 12 * px
    out = []
    for y in range(h):
        for x in range(w):
            g = srgb_to_linear(field[y * w + x][2])
            cx, cy = (floor((x + 0.5) / cell) + 0.5) * cell, (floor((y + 0.5) / cell) + 0.5) * cell
            qx, qy = held(w, h, floor(cx), floor(cy))
            tx, ty, e = field[qy * w + qx]
            px_ = [g, g, g, 1.0]
            if tx != 0 or ty != 0:
                half = 0.4 * cell
                vx, vy = x + 0.5 - cx, y + 0.5 - cy
                along_ = min(max(vx * tx + vy * ty, -half), half)
                d = sqrt((vx - along_ * tx) ** 2 + (vy - along_ * ty) ** 2)
                cov = e * min(max(1 - d / px, 0.0), 1.0)
                px_ = [g + cov * (c - g) for c in (0.0, 1.0, 1.0)] + [1.0]
            out.append(px_)
    return L.pic(w, h, out)


def contour_paint(o, m, s, px=1.0):
    """O painted along its contours, M the edge-map layer's picture or None."""
    w, h = o["w"], o["h"]
    if s["view"] == "edge_map":
        return edge_view(flow(m if m is not None else o, s, px), s, w, h, px)
    passes = floor(s["smear_iterations"])
    if passes == 0 and s["draw_color_fills"] == "none":
        return o
    fixed = flow(m, s, px) if m is not None else None
    cur = o
    for _ in range(passes):
        cur = smear_pass(cur, fixed if fixed is not None else flow(cur, s, px), s, px)
    if s["draw_color_fills"] == "none":
        return cur
    perceptual = s["reduction_method"] == CUSTOM
    pal = custom(s["colors"]) if perceptual else palette(o, s)
    if not pal:
        return cur
    keys = [cie_lab(q) for q in pal] if perceptual else pal
    last = fixed if fixed is not None else flow(cur, s, px)
    out = []
    for p, (_, _, e) in zip(cur["px"], last):
        a = p[3]
        inside = s["low_strength"] <= 100 * e <= s["high_strength"]
        if a > 0 and inside:
            v = [S.linear_to_srgb(min(max(p[c] / a, 0.0), 1.0)) for c in range(3)]
            j = nearest(keys, cie_lab(v) if perceptual else v)
            out.append([srgb_to_linear(c) * a for c in pal[j]] + [a])
        elif s["draw_color_fills"] == "only":
            out.append([0.0, 0.0, 0.0, 0.0])
        else:
            out.append(list(p))
    return L.pic(w, h, out)


# --- the cases ------------------------------------------------------------------------------

NAMES = ("view", "stylization", "cleanliness", "smear_iterations", "apply_to_alpha",
         "draw_color_fills", "reduction_method", "posterization_level", "low_strength",
         "high_strength", "offset_flow_angles", "quantize_flow_angles", "edge_map_layer")


def case(holder="photo", shift=(0, 0), **kw):
    c = {"view": "final_result", "stylization": 50, "cleanliness": 50, "smear_iterations": 3,
         "apply_to_alpha": "off", "draw_color_fills": "none", "reduction_method": "k_means",
         "posterization_level": 8, "low_strength": 0, "high_strength": 100,
         "offset_flow_angles": 0, "quantize_flow_angles": 0, "edge_map_layer": ""}
    c.update(kw)
    c["holder"], c["shift"] = holder, shift
    return c


def the_map(c):
    return L.fit(decoded(c["edge_map_layer"]), "center", W, H) if c["edge_map_layer"] in MAPS else None


RANGES = {"stylization": (0, 100), "cleanliness": (0, 100), "smear_iterations": (0, 100),
          "posterization_level": (2, 32), "low_strength": (0, 100), "high_strength": (0, 100),
          "offset_flow_angles": (-3600, 3600), "quantize_flow_angles": (0, 90)}


def settings(c, frame):
    """Each number held inside its range after it is worked out (document 20)."""
    s = {k: value_at(c[k], frame) for k in NAMES}
    s["colors"] = c.get("colors", [])
    for k, (lo, hi) in RANGES.items():
        s[k] = min(hi, max(lo, s[k]))
    return s


def render(c, frame):
    out = contour_paint(decoded(c["holder"]), the_map(c), settings(c, frame))
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = decoded(c["holder"])
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


FILLS = dict(draw_color_fills="over_smeared_image", posterization_level=4)
CASES = {
    "FX-CONTOUR-001": ("As added: Stylization 50, Cleanliness 50, 3 smear passes: the photo's "
                       "two-pixel squares smeared along their edges into diagonal strokes.",
                       case(), (0,)),
    "FX-CONTOUR-002": ("Smear Iterations 0, no fills: nothing happens.",
                       case(smear_iterations=0), (0,)),
    "FX-CONTOUR-003": ("Stylization 0: short strokes, nearer the photo than FX-CONTOUR-001.",
                       case(stylization=0), (0,)),
    "FX-CONTOUR-004": ("Stylization 100: long strokes, further from it.",
                       case(stylization=100), (0,)),
    "FX-CONTOUR-005": ("Cleanliness 0: the flow follows each small square.",
                       case(cleanliness=0), (0,)),
    "FX-CONTOUR-006": ("Cleanliness 100: the flow follows only the broad pattern.",
                       case(cleanliness=100), (0,)),
    "FX-CONTOUR-007": ("Smear Iterations 12: smeared again and again, the strokes run together.",
                       case(smear_iterations=12), (0,)),
    "FX-CONTOUR-008": ("The two blocks with clear round them, Apply to Alpha off: the colours "
                       "smear, every covering kept exactly, every clear pixel clear.",
                       case(holder="shapes"), (0,)),
    "FX-CONTOUR-009": ("The same with Apply to Alpha on: the blocks' edges smear out into the "
                       "clear round them.", case(holder="shapes", apply_to_alpha="on"), (0,)),
    "FX-CONTOUR-010": ("Draw Color Fills Over Smeared Image, K-Means, 4 colours: every pixel one "
                       "of four flat colours.", case(**FILLS), (0,)),
    "FX-CONTOUR-011": ("The same with Median: four colours by median cut alone.",
                       case(reduction_method="median", **FILLS), (0,)),
    "FX-CONTOUR-012": ("Fills Only, Low Strength 38: the fills only where the flow is at least "
                       "38 per cent strong, clear elsewhere.",
                       case(draw_color_fills="only", posterization_level=4, low_strength=38), (0,)),
    "FX-CONTOUR-013": ("Fills Over Smeared Image, High Strength 38: the fills where the flow is "
                       "weak, the smear where it is strong.",
                       case(high_strength=38, **FILLS), (0,)),
    "FX-CONTOUR-014": ("View Edge Map: the flow's strength in grey, two cyan lines along the flow "
                       "in the two cells.", case(view="edge_map"), (0,)),
    "FX-CONTOUR-015": ("Offset Flow Angles 90: the strokes run across the edges instead.",
                       case(offset_flow_angles=90), (0,)),
    "FX-CONTOUR-016": ("Quantize Flow Angles 90: every stroke straight across or straight down.",
                       case(quantize_flow_angles=90), (0,)),
    "FX-CONTOUR-017": ("The ramp as the External Edge Map: its edges all run straight down, so "
                       "the photo is smeared straight down.", case(edge_map_layer="ramp"), (0,)),
    "FX-CONTOUR-018": ("Edge Map with the ramp as the External Edge Map: the grey the same "
                       "down each column, and one cyan line straight down the first cell (the "
                       "second cell's middle lies past the drawing's right edge, and its line with "
                       "it).",
                       case(view="edge_map", edge_map_layer="ramp"), (0,)),
    "FX-CONTOUR-019": ("An External Edge Map that is not in the composition, `gone`: the layer's "
                       "own edges, FX-CONTOUR-001, and the warning every frame.",
                       case(edge_map_layer="gone"), (0, 4)),
    "FX-CONTOUR-020": ("FX-CONTOUR-001 on the holder moved 2 right and 1 down: the same, moved.",
                       case(shift=(2, 1)), (0,)),
    "FX-CONTOUR-021": ("Stylization keyed from 0 at frame 0 to 100 at frame 4, linear: frame 2 is "
                       "Stylization 50, FX-CONTOUR-001.",
                       case(stylization=keyed((0, 0), (4, 100))), (0, 2, 4)),
    "FX-CONTOUR-022": ("Posterization Level eased from 3 at frame 0 to 32 at frame 4 on a curve "
                       "that overshoots: at frame 2 it would pass 32, is held at 32.",
                       case(draw_color_fills="over_smeared_image",
                            posterization_level=keyed((0, 3, OVERSHOOT), (4, 32))), (0, 2, 4)),
}

# D-584: Match to Nearest Custom Color.
FOUR = ["#c03030", "#3050c0", "#808080", "#f0f0c0"]
CUSTOM_FILLS = dict(draw_color_fills="over_smeared_image", reduction_method=CUSTOM)
CASES.update({
    "FX-CONTOUR-033": ("Match to Nearest Custom Color with four custom colours, a brick red, a "
                       "blue, a mid grey and a cream, Over Smeared Image: every pixel one of the "
                       "four, each the nearest as the eye sees it, which for some pixels is not the "
                       "nearest by red, green and blue.",
                       case(colors=FOUR, **CUSTOM_FILLS), (0,)),
    "FX-CONTOUR-034": ("The same, Fills Only with Low Strength 38: the four colours only where "
                       "the flow is at least 38 per cent strong, clear elsewhere.",
                       case(colors=FOUR, draw_color_fills="only", reduction_method=CUSTOM,
                            low_strength=38), (0,)),
    "FX-CONTOUR-035": ("Match to Nearest Custom Color with no custom colours: nothing is filled, "
                       "the smear alone, FX-CONTOUR-001.",
                       case(colors=[], **CUSTOM_FILLS), (0,)),
    "FX-CONTOUR-036": ("One custom colour, the blue: every pixel blue.",
                       case(colors=["#3050c0"], **CUSTOM_FILLS), (0,)),
    "FX-CONTOUR-037": ("The two blocks with clear round them, pure red and pure blue as the custom "
                       "colours: the red block red, the blue block blue, every covering kept, "
                       "every clear pixel clear.",
                       case(holder="shapes", colors=["#ff0000", "#0000ff"], **CUSTOM_FILLS), (0,)),
    "FX-CONTOUR-038": ("The four custom colours kept but K-Means chosen: they are not used, "
                       "FX-CONTOUR-010.",
                       case(colors=FOUR, **FILLS), (0,)),
})

INVALID = {
    "FX-CONTOUR-023": ("Stylization 120, above 100.", case(stylization=120)),
    "FX-CONTOUR-024": ("Smear Iterations -1, below 0.", case(smear_iterations=-1)),
    "FX-CONTOUR-025": ("A View written \"arrows\".", case(view="arrows")),
    "FX-CONTOUR-026": ("Draw Color Fills written \"under\".", case(draw_color_fills="under")),
    "FX-CONTOUR-027": ("Reduction Method \"custom\", After Effects' Match to Nearest Custom "
                       "Color, which is not built.", case(reduction_method="custom")),
    "FX-CONTOUR-028": ("Posterization Level 1, below 2.", case(posterization_level=1)),
    "FX-CONTOUR-029": ("Quantize Flow Angles 100, past 90.", case(quantize_flow_angles=100)),
    "FX-CONTOUR-030": ("An External Edge Map written as the number 3, not a word.",
                       case(edge_map_layer=3)),
    "FX-CONTOUR-031": ("Apply to Alpha written \"yes\".", case(apply_to_alpha="yes")),
    "FX-CONTOUR-032": ("Smear Iterations keyed to 150 at frame 4.",
                       case(smear_iterations=keyed((0, 3), (4, 150)))),
    "FX-CONTOUR-039": ("A custom colour written \"#c0303\", five digits.",
                       case(colors=["#c03030", "#c0303"], **CUSTOM_FILLS)),
    "FX-CONTOUR-040": ("Nine custom colours, past eight.",
                       case(colors=FOUR + FOUR + ["#000000"], **CUSTOM_FILLS)),
    "FX-CONTOUR-041": ("A custom colour written \"red\", a word.",
                       case(colors=["red"], **CUSTOM_FILLS)),
}


def effect(c):
    """D-584: `colors` written only by the cases that have it, as a file from before it."""
    parameters = {k: setting_json(c[k]) for k in NAMES}
    if "colors" in c:
        parameters["colors"] = c["colors"]
    return {"instance_id": "fx-1", "type_id": "core.contour_paint", "enabled": True,
            "parameters": parameters}


def project_json(fx, c):
    holder = L.raster("holder", "asset-" + c["holder"], position=c["shift"], out_frame=FRAMES)
    holder["effects"] = [effect(c)]
    maps = [L.raster(n, "asset-" + n, enabled=False, out_frame=FRAMES) for n in MAPS]
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(n, f"media/{n}.png") for n in DRAWINGS],
            "compositions": [L.composition("comp-main", W, H, FRAMES, [holder] + maps)]}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        entry = {"says": says, "project": write(fx, c), "frames": rendered}
        if c["edge_map_layer"] == "gone":
            entry["warning"] = "EFFECT_LAYER_MISSING"
        expected["cases"][fx] = entry
        before = plain(c)
        print(f"{fx}: " + ", ".join(f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} "
                                    "changed" for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_contour_paint.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    photo = plain(case())
    shapes = plain(case(holder="shapes"))
    at = lambda x, y: y * W + x  # noqa: E731
    enc = lambda p: tuple(round(S.linear_to_srgb(v / p[3]) * 255) for v in p[:3]) if p[3] > 0 else None  # noqa: E731
    far = lambda a, b: sum(abs(u - v) for p, q in zip(a, b) for u, v in zip(p, q))  # noqa: E731
    for fx, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= u <= p[3] + 1e-9 for u in p[:3]), (fx, p)
    one = c["FX-CONTOUR-001"]["0"]
    assert one != photo and c["FX-CONTOUR-002"]["0"] == photo
    assert 0 < far(c["FX-CONTOUR-003"]["0"], photo) < far(one, photo) < far(c["FX-CONTOUR-004"]["0"], photo)
    assert c["FX-CONTOUR-005"]["0"] != one != c["FX-CONTOUR-006"]["0"]
    assert far(c["FX-CONTOUR-007"]["0"], photo) > far(one, photo)
    eight, nine = c["FX-CONTOUR-008"]["0"], c["FX-CONTOUR-009"]["0"]
    assert all(abs(p[3] - q[3]) < 1e-12 for p, q in zip(eight, shapes)) and eight != shapes
    assert all(p == [0.0, 0.0, 0.0, 0.0] for p, q in zip(eight, shapes) if q[3] == 0)
    assert any(q[3] == 0 and p[3] > 0 for p, q in zip(nine, shapes))
    for fx in ("010", "011"):
        cols = {enc(p) for p in c["FX-CONTOUR-" + fx]["0"]}
        assert len(cols) <= 4, (fx, cols)
    assert c["FX-CONTOUR-010"]["0"] != c["FX-CONTOUR-011"]["0"]
    twelve, thirteen = c["FX-CONTOUR-012"]["0"], c["FX-CONTOUR-013"]["0"]
    assert any(p[3] == 0 for p in twelve) and any(p[3] > 0 for p in twelve)
    assert len({enc(p) for p in twelve if p[3] > 0}) <= 4
    assert any(p == q for p, q in zip(thirteen, one)) and any(p != q for p, q in zip(thirteen, one))
    edge = c["FX-CONTOUR-014"]["0"]
    assert all(p[3] == 1.0 for p in edge)
    assert any(p[0] < p[1] - 1e-6 for p in edge), "a cyan line"
    assert c["FX-CONTOUR-015"]["0"] != one and c["FX-CONTOUR-016"]["0"] != one
    # The ramp's flow is straight down: each column of the photo keeps its own colours' mix.
    seventeen = c["FX-CONTOUR-017"]["0"]
    for x in range(W):
        col = [photo[at(x, y)] for y in range(H)]
        for y in range(H):
            lo = [min(p[k] for p in col) - 1e-12 for k in range(4)]
            hi = [max(p[k] for p in col) + 1e-12 for k in range(4)]
            assert all(lo[k] <= seventeen[at(x, y)][k] <= hi[k] for k in range(4)), (x, y)
    eighteen = c["FX-CONTOUR-018"]["0"]
    for x in range(W):
        greys = {round(eighteen[at(x, y)][0], 12) for y in range(H) if eighteen[at(x, y)][0] == eighteen[at(x, y)][1]}
        assert len(greys) <= 1, x
    cyan = {x for x in range(W) for y in range(H) if eighteen[at(x, y)][0] < eighteen[at(x, y)][1] - 1e-9}
    assert cyan and all(4 <= x <= 7 for x in cyan), cyan
    assert c["FX-CONTOUR-019"]["0"] == one == c["FX-CONTOUR-019"]["4"]
    twenty = c["FX-CONTOUR-020"]["0"]
    assert all(twenty[at(x, y)] == one[at(x - 2, y - 1)] for x in range(2, W) for y in range(1, H))
    k21 = c["FX-CONTOUR-021"]
    assert k21["2"] == one and k21["0"] == c["FX-CONTOUR-003"]["0"] and k21["4"] == c["FX-CONTOUR-004"]["0"]
    k22 = c["FX-CONTOUR-022"]
    assert k22["2"] == k22["4"] == render(case(draw_color_fills="over_smeared_image", posterization_level=32), 0)
    assert len({enc(p) for p in k22["0"]}) <= 3
    # D-584.
    four = {tuple(round(v * 255) for v in q) for q in custom(FOUR)}
    t33 = c["FX-CONTOUR-033"]["0"]
    assert {enc(p) for p in t33} == four
    s = settings(case(), 0)
    cur = decoded("photo")
    for _ in range(3):
        cur = smear_pass(cur, flow(cur, s), s)
    pal = custom(FOUR)
    rgb = [pal[nearest(pal, [S.linear_to_srgb(min(max(v / p[3], 0.0), 1.0)) for v in p[:3]])]
           for p in cur["px"]]
    assert any(enc(p) != tuple(round(v * 255) for v in q) for p, q in zip(t33, rgb)), "not RGB's"
    t34 = c["FX-CONTOUR-034"]["0"]
    assert any(p[3] == 0 for p in t34) and {enc(p) for p in t34 if p[3] > 0} <= four
    assert c["FX-CONTOUR-035"]["0"] == one
    assert {enc(p) for p in c["FX-CONTOUR-036"]["0"]} == {(48, 80, 192)}
    t37 = c["FX-CONTOUR-037"]["0"]
    assert all(abs(p[3] - q[3]) < 1e-12 for p, q in zip(t37, shapes))
    assert all(enc(t37[at(x, y)]) == (255, 0, 0) for x in range(1, 6) for y in range(2, 8))
    assert all(enc(t37[at(x, y)]) == (0, 0, 255) for x in range(9, 12) for y in range(3, 7))
    assert c["FX-CONTOUR-038"]["0"] == c["FX-CONTOUR-010"]["0"]
    print("checked")


if __name__ == "__main__":
    main()
