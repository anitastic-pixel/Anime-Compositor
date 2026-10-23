"""Curl Noise, worked a second way.

D-446 adds `core.curl_noise`, after After Effects' Curl Noise (After Effects 26.3 beta, "Curl
Noise: swirling, animated 2D noise", Adobe's community announcement): a swirling noise whose
flow lines follow the curl of a smooth noise field (Bridson's "curl noise", 2007). Its settings
are After Effects' Source (Internal, This Layer or Other Layer; D-583 builds the two layer
sources, with Source Layer, Softness, Source Mapping and Mirror at Edge), Speed and Direction,
the Transform's Size and Offset, Evolution, Turbulence Speed, Swirl, Density, Smoothness,
Vertical Bias, Sample Count, Sample Radius, Flow Softness, Edge Definition, Flow Falloff, View,
Contrast, Brightness, Clip HDR Results and Channel. Adobe publishes no formula; the numbers
below are this program's own rule, built on P0-19's value noise (`grade::value`, D-127, D-128),
and nothing is ported.

D-749 replaces D-446's look (and D-680's walk) with one fitted to After Effects' own pictures in
the tutorials the audit's Wave 1 read (the audit's rows 8, 13 and 14): a smooth value noise
carried along the curl's flow lines, so the texture is large, soft marbling that grows with
Size and Scale and whose swirls grow with Sample Radius; Flow Softness the After Effects way round (higher is crisper);
and Speed and Turbulence Speed added up frame by frame, so a keyed speed moves the noise on
from where it is instead of jumping. D-750 adds Scale Width, Scale Height and Uniform Scaling.

The rule, at a point p of the drawing's own pixels (its top-left corner (0, 0), however far an
effect above grew the buffer):

1. Travel: the drift and the depth add up the frames before this one. At frame F the drift is
   the sum over the frames f from 0 to F - 1 of speed(f) / 10 pixels towards direction(f) (0 up,
   90 right), times 24 / the composition's frames a second, and the depth's travel the same sum
   of turbulence_speed(f) / 10 (a frame before 0 counts back). With Speed not keyed the drift is
   speed F / 10 at 24 frames a second, as before D-749.
2. The point: with c the drawing's middle, theta = rotation, mx = my = 100 / scale with Uniform
   Scaling on, else mx = 100 / scale_width and my = 100 / scale_height, d = p - (offset + drift)
   - c, the noise is read at q = c + (mx (d_x cos + d_y sin), my (-d_x sin + d_y cos)). With
   Perspective Offset on, octave o of the value noise (step 4) is read at q + (1 - 0.5^o) T(offset
   + drift), T the same turn and scale without c.
3. The flow: the noise sizes are s = 2 size 2^(-density / 50) pixels for the flow and 1.6 s for
   the value. With b = vertical_bias / 100, the flow is stretched by a_x = 1 + max(0, 1 - 2 b)
   across and a_y = 1 + max(0, 2 b - 1) down. The potential psi is P0-19's smooth value noise,
   one octave, two lattices: lattice A on channel 0 at (X, Y) = (q_x / (s a_x), q_y / (s a_y), z),
   z = (evolution + the depth's travel) / 360; lattice B on channel 4 at (X cos 0.7 - Y sin 0.7
   + 0.37, X sin 0.7 + Y cos 0.7 + 0.61, z); their sum over sqrt 2. Its slope (psi_X, psi_Y) is
   worked exactly over all eight corners of each cell, lattice B's turned back by -0.7. The flow
   is w = sigma (psi_Y / a_y, -psi_X / a_x), sigma = swirl / 180; it moves the point s w pixels
   per unit of trace time. It is the curl of a stretched noise, so it never gathers or spreads.
4. The value V: four octaves of the same two-lattice noise (channels 8 o and 8 o + 4), octave o
   at (q_x / (1.6 s g_x) 2^o, q_y / (1.6 s g_y) 2^o, z 2^o), g_x = 1 + 0.6 max(0, 1 - 2 b),
   g_y = 1 + 0.6 max(0, 2 b - 1), weighted 1, r, r^2, r^3 with r = 0.4 (1 - smoothness / 100),
   over the sum of the weights.
5. The trace: from q, the flow is followed backwards for tau = 0.045 sample_radius units of
   trace time, in n = floor(sample_count) equal parts; each part in k = max(1, ceil(tau / n /
   0.1)) steps of h = tau / n / k, each step the midpoint rule (w1 at the point, w2 at the point
   less h / 2 s w1, the point less h s w2). V is read at q (V0) and at the end of each part:
   n + 1 values, their mean M and the last E. With tau 0 or Swirl 0 the point does not move.
6. The grey: with crisp = (flow_softness / 100)^0.37 and the gain C = 2 2^((edge_definition -
   50) / 50), out = crisp E + (1 - crisp) M, and Final Render's grey is
   0.5 + 0.5 C out (1 - flow_falloff / 100 (0.5 - 0.5 V0)). Input Noise: 0.5 + 0.5 C V0, so
   Sample Radius 0 is Input Noise. Curl Generation: red 0.5 + 0.5 the flow across, green
   0.5 + 0.5 the flow down, the flow w at q made one long and brought back to the picture's
   directions (divided by mx and my, then turned by theta), nought where it is nought; blue
   0.5 + 0.5 C V0.
   The four numbers, the direction's two, C V0 and the grey, are kept in single precision, as
   the card keeps them.
7. Each channel o becomes 0.5 + (o - 0.5) contrast / 100 + brightness / 100, held at 0, and at 1
   too unless Clip HDR Results is off in a composition that works in Float.
8. Channel RGB: the colour, through the sRGB curve, replaces the pixel's at its covering; Red,
   Green or Blue: only that channel is replaced; Alpha: the covering is multiplied by the three's
   mean, held in 0 and 1, and the colour kept.

A pixel that does not show is left.

D-583, Source This Layer or Other Layer: steps 3 and 4's noise comes from a picture instead.

S1. The picture: This Layer, the layer's pixels as they reach the effect (the whole buffer, so
    what an effect above grew is in it); Other Layer, D-189's map of the layer `source_layer`
    names, fitted to the drawing by Source Mapping, `fill` stretched over it (D-189's stretch) or
    `center` at its own size in the middle (D-189's center). Other Layer naming no layer is
    refused; a name not in the composition draws the frame without the effect, with D-189's
    EFFECT_LAYER_MISSING.
S2. Its lightness plane: each pixel's luminance (`grade::phase_of`: Rec. 709 weights on the
    straight linear colour, through the sRGB curve) times its covering, so an empty pixel is 0,
    kept in single precision. With Softness above 0 the plane is blurred by D-109's Gaussian with
    the edges held, sigma softness / 10 pixels, its sums in single precision.
S3. The point is q = p - (offset + drift), unturned and unscaled; the plane L is read at q on the
    picture (This Layer: q plus the buffer corner's distance from the drawing's), pixel centres
    at halves, between pixels as document 21's bilinear does; outside the plane 0, or with Mirror
    at Edge on, the pixel mirrored back in (... 1 0 | 0 1 ... w-1 | w-1 ...).
S4. V = 2 L - 1 wherever the trace reads it, and the flow is w = sigma (s g_y / 2, -s g_x / 2),
    g_x L one pixel right less L one pixel left and g_y the same down (nought where (g_x, g_y)
    is under 1e-5 long), made 1 / 9 long where it is longer, s from Size and Density, so the
    point moves at most s / 9 pixels per unit of trace time (Sample Radius pixels in all at Size
    100, Density 0). The trace's parts (step 5) are each in k = max(1, ceil(tau / n / 0.1),
    ceil(tau / n s / 9)) steps, so no step moves the point more than a pixel and a flow line
    keeps to the picture's own edges. Rotation, Scale, Perspective Offset, Smoothness, Vertical
    Bias, Evolution and Turbulence Speed do nothing with a picture. Steps 5 to 8 are otherwise
    unchanged.

Settings: `source` `internal` (when added), `this_layer` or `other_layer`; `source_layer`
D-189's layer word, "" (when added); `softness` 0 to 100, 0; `source_mapping` `fill` (when
added) or `center`; `mirror_at_edge` `off` (when added) or `on`. A file without the four reads
them as when added, and they are written only when they are not (D-304). `speed` 0
to 100, 10 when added; `direction` -100000 to 100000 degrees, 0; `size` 1 to 1000 pixels, 100;
`offset` a point, -100000 to 100000, [0, 0]; `evolution` -100000 to 100000 degrees, 0;
`turbulence_speed` 0 to 200, 20; `swirl` -360 to 360, 45; `density` -100 to 100, 0; `smoothness`
0 to 100, 50; `vertical_bias` 0 to 100, 50; `sample_count` 3 to 24, 12, its whole part;
`sample_radius` 0 to 200 pixels, 30; `flow_softness` 0 to 100, 20; `edge_definition` 0 to 100,
50; `flow_falloff` 0 to 100, 0; `view` `final_render` (when added), `input_noise` or
`curl_generation`; `contrast` 0 to 1000, 100; `brightness` -1000 to 1000, 0; `clip_hdr_results`
`on` (when added) or `off`; `channel` `rgb` (when added), `red`, `green`, `blue` or `alpha`.
D-646 and D-750's Transform (`tools/curl_transform_reference.py`): `rotation`, `scale`,
`perspective_offset`, `uniform_scaling`, `scale_width` and `scale_height`, written only when not
as added. Every number can be keyed.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, keeping the single-precision steps the rule names.

Every case is a composition 16 by 10 holding Noise's card, the same size, unmoved unless the case
says; D-583's Other Layer cases add a switched-off layer "ramp", 8 by 5 greys, under it. The
expected frames are in `Fixtures/curl_noise/expected_curl_noise.json`.

D-583 (D-566): FX-CURL-033 recorded This Layer refused while it was not built and is now a
working This Layer case; FX-CURL-034 stays refused, now for naming no layer, its frames unchanged.
033 takes the small cells and lines the other D-583 cases use (the coordinator's ruling,
2026-10-12): flow lines crossing the card's sharp edges magnify the last bit of a lightness value.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/curl_noise_reference.py
"""

import json
import math
import struct
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import effect_layer_reference as L  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
from noise_reference import DRAWINGS, u  # noqa: E402

W, H = R.W, R.H
FPS = 24  # every case's composition
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "curl_noise"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"speed": (0, 100), "direction": (-100000, 100000), "size": (1, 1000),
          "offset": (-100000, 100000), "evolution": (-100000, 100000),
          "turbulence_speed": (0, 200), "swirl": (-360, 360), "density": (-100, 100),
          "smoothness": (0, 100), "vertical_bias": (0, 100), "sample_count": (3, 24),
          "sample_radius": (0, 200), "flow_softness": (0, 100), "edge_definition": (0, 100),
          "flow_falloff": (0, 100), "contrast": (0, 1000), "brightness": (-1000, 1000),
          "softness": (0, 100), "rotation": (-100000, 100000), "scale": (1, 10000),
          "scale_width": (1, 10000), "scale_height": (1, 10000)}
WORDS = ("source", "view", "clip_hdr_results", "channel")
# D-583: written only when not as added (D-304), so the files before it are unchanged.
SOURCE_KEYS = {"source_layer": "", "softness": 0, "source_mapping": "fill",
               "mirror_at_edge": "off"}
# D-646 and D-750's Transform, the same.
TRANSFORM_KEYS = {"rotation": 0, "scale": 100, "perspective_offset": "off",
                  "uniform_scaling": "on", "scale_width": 100, "scale_height": 100}
NAMES = ("source", "speed", "direction", "size", "offset", "evolution", "turbulence_speed",
         "swirl", "density", "smoothness", "vertical_bias", "sample_count", "sample_radius",
         "flow_softness", "edge_definition", "flow_falloff", "view", "contrast", "brightness",
         "clip_hdr_results", "channel")
CHANNELS = ("rgb", "red", "green", "blue", "alpha")
TURN = (math.cos(0.7), math.sin(0.7))


def f32(v):
    return struct.unpack("f", struct.pack("f", v))[0]


# --- the rule -------------------------------------------------------------------------------

def fade(t):
    return t * t * t * (t * (6 * t - 15) + 10)


def slope(t):
    return 30 * t * t * (1 - t) * (1 - t)


def value_slope(ch, x, y, z):
    """The smooth value noise for seed 0 and its slope across and down, over all eight corners."""
    i, j, k = math.floor(x), math.floor(y), math.floor(z)
    tx, ty, tz = x - i, y - j, z - k
    sx, sy, sz = fade(tx), fade(ty), fade(tz)
    dx, dy = slope(tx), slope(ty)
    v = gx = gy = 0.0
    for corner in range(8):
        a, b, c = corner & 1, (corner >> 1) & 1, corner >> 2
        wx = sx if a else 1 - sx
        wy = sy if b else 1 - sy
        wz = sz if c else 1 - sz
        n = u(0, i + a, j + b, k + c, ch)
        v += wx * wy * wz * n
        gx += (dx if a else -dx) * wy * wz * n
        gy += wx * (dy if b else -dy) * wz * n
    return v, gx, gy


def two(ch, x, y, z):
    """Steps 3 and 4's two lattices: the value and its slope across and down."""
    c, s = TURN
    a = value_slope(ch, x, y, z)
    b = value_slope(ch + 4, x * c - y * s + 0.37, x * s + y * c + 0.61, z)
    q = 1 / math.sqrt(2)
    return ((a[0] + b[0]) * q, (a[1] + b[1] * c + b[2] * s) * q, (a[2] - b[1] * s + b[2] * c) * q)


def travel(c, frame_no):
    """Step 1: the drift and the depth's travel at the frame, added up over the frames before."""
    dx = dy = dz = 0.0
    way = 1 if frame_no >= 0 else -1
    for f in (range(frame_no) if way == 1 else range(frame_no, 0)):
        sp, t = held(c, "speed", f), math.radians(held(c, "direction", f))
        dx += way * math.sin(t) * sp / 10 * 24 / FPS
        dy += way * -math.cos(t) * sp / 10 * 24 / FPS
        dz += way * held(c, "turbulence_speed", f) / 10 * 24 / FPS
    return dx, dy, dz


def setup(n, moved):
    s = 2 * n["size"] * 2 ** (-n["density"] / 50)
    b = n["vertical_bias"] / 100
    shift = (n["offset"][0] + moved[0], n["offset"][1] + moved[1])
    if n["uniform_scaling"] == "on":
        mx = my = 100 / n["scale"]
    else:
        mx, my = 100 / n["scale_width"], 100 / n["scale_height"]
    t = math.radians(n["rotation"])
    turn = None if n["rotation"] == 0 and mx == 1 and my == 1 else (math.cos(t), math.sin(t), mx, my)
    k = {"s": s, "shift": shift, "z": (n["evolution"] + moved[2]) / 360, "turn": turn,
         "a": (1 + max(0.0, 1 - 2 * b), 1 + max(0.0, 2 * b - 1)),
         "g": (1 + 0.6 * max(0.0, 1 - 2 * b), 1 + 0.6 * max(0.0, 2 * b - 1)),
         "r": 0.4 * (1 - n["smoothness"] / 100), "sigma": n["swirl"] / 180,
         "tau": 0.045 * n["sample_radius"], "steps": math.floor(n["sample_count"]),
         "crisp": (n["flow_softness"] / 100) ** 0.37,
         "gain": 2 * 2 ** ((n["edge_definition"] - 50) / 50), "ff": n["flow_falloff"] / 100}
    sx, sy = (lin(k, *shift) if n["perspective_offset"] == "on" else (0.0, 0.0))
    k["slide"] = [((1 - 0.5 ** o) * sx, (1 - 0.5 ** o) * sy) for o in range(4)]
    return k


def lin(k, x, y):
    """T: the turn and scale without the middle."""
    if k["turn"] is None:
        return x, y
    c, s, mx, my = k["turn"]
    return (x * c + y * s) * mx, (-x * s + y * c) * my


def point(k, px, py):
    """Step 2's q."""
    cx, cy = W / 2, H / 2
    dx, dy = lin(k, px - k["shift"][0] - cx, py - k["shift"][1] - cy)
    return cx + dx, cy + dy


def plane_at(src, x, y):
    """D-583's step S3: the lightness plane read at (x, y), pixel centres at halves, 0 outside
    or mirrored back in."""
    p, w, h, mirror = src["plane"], src["w"], src["h"], src["mirror"]

    def get(i, j):
        if mirror:
            i, j = i % (2 * w), j % (2 * h)
            i, j = (2 * w - 1 - i if i >= w else i), (2 * h - 1 - j if j >= h else j)
        elif not (0 <= i < w and 0 <= j < h):
            return 0.0
        return p[j * w + i]

    fx, fy = x - 0.5, y - 0.5
    i0, j0 = math.floor(fx), math.floor(fy)
    ux, uy = fx - i0, fy - j0
    top = (1 - ux) * get(i0, j0) + ux * get(i0 + 1, j0)
    bottom = (1 - ux) * get(i0, j0 + 1) + ux * get(i0 + 1, j0 + 1)
    return (1 - uy) * top + uy * bottom


def capped(wx, wy, most):
    ln = math.hypot(wx, wy)
    return (wx * most / ln, wy * most / ln) if ln > most else (wx, wy)


def flow(k, qx, qy, src):
    """Step 3's w (S4's with a picture)."""
    if src is None:
        ax, ay = k["a"]
        _, gx, gy = two(0, qx / (k["s"] * ax), qy / (k["s"] * ay), k["z"])
        return k["sigma"] * gy / ay, -k["sigma"] * gx / ax
    bx, by = qx + src["origin"][0], qy + src["origin"][1]
    gx = plane_at(src, bx + 1, by) - plane_at(src, bx - 1, by)
    gy = plane_at(src, bx, by + 1) - plane_at(src, bx, by - 1)
    if math.hypot(gx, gy) < 1e-5:
        return 0.0, 0.0
    return capped(k["sigma"] * k["s"] * gy / 2, -k["sigma"] * k["s"] * gx / 2, 1 / 9)


def value(k, qx, qy, src):
    """Step 4's V (S4's with a picture)."""
    if src is not None:
        return 2 * plane_at(src, qx + src["origin"][0], qy + src["origin"][1]) - 1
    gx, gy = k["g"]
    v = total = 0.0
    amp, fine = 1.0, 1.0
    for o in range(4):
        x = (qx + k["slide"][o][0]) / (1.6 * k["s"] * gx) * fine
        y = (qy + k["slide"][o][1]) / (1.6 * k["s"] * gy) * fine
        v += amp * two(8 * o, x, y, k["z"] * fine)[0]
        total += amp
        amp *= k["r"]
        fine *= 2
    return v / total


def curl_at(k, px, py, src=None):
    """Steps 2 to 6 at the drawing's point (px, py): the direction's two, C V0 and the grey, in
    single precision."""
    if src is None:
        qx, qy = point(k, px, py)
    else:
        qx, qy = px - k["shift"][0], py - k["shift"][1]
    v0 = value(k, qx, qy, src)
    wx, wy = flow(k, qx, qy, src)
    end, total = v0, v0
    if k["tau"] > 0 and k["sigma"] != 0:
        d = k["tau"] / k["steps"]
        parts = max(1, math.ceil(d / 0.1))
        if src is not None:
            parts = max(parts, math.ceil(d * k["s"] / 9))
        h, s = d / parts, k["s"]
        for _ in range(k["steps"]):
            for _ in range(parts):
                ax, ay = flow(k, qx, qy, src)
                bx, by = flow(k, qx - h / 2 * s * ax, qy - h / 2 * s * ay, src)
                qx, qy = qx - h * s * bx, qy - h * s * by
            end = value(k, qx, qy, src)
            total += end
        mean = total / (k["steps"] + 1)
    else:
        mean = v0
    out = k["crisp"] * end + (1 - k["crisp"]) * mean
    grey = 0.5 + 0.5 * k["gain"] * out * (1 - k["ff"] * (0.5 - 0.5 * v0))
    if src is None and k["turn"] is not None:
        c, s, mx, my = k["turn"]
        x, y = wx / mx, wy / my
        wx, wy = x * c - y * s, x * s + y * c
    ln = math.hypot(wx, wy)
    dx, dy = (wx / ln, wy / ln) if ln > 0 else (0.0, 0.0)
    return [f32(dx), f32(dy), f32(k["gain"] * v0), f32(grey)]


def luminance(p):
    """grade::phase_of's luminance of a working pixel, times its covering (step S2)."""
    a = p[3]
    if a <= 0:
        return 0.0
    b = [min(1.0, max(0.0, p[c] / a)) for c in range(3)]
    return f32(S.linear_to_srgb(0.2126 * b[0] + 0.7152 * b[1] + 0.0722 * b[2]) * a)


def held_blur(plane, w, h, sigma):
    """D-109's Gaussian with the edges held, across then down, each pixel's taps added in order
    in single precision, as `effects::held_blur` adds them."""
    r = math.ceil(3 * sigma)
    if r == 0:
        return plane
    taps = [math.exp(-((i - r) ** 2) / (2 * sigma * sigma)) for i in range(2 * r + 1)]
    total = sum(taps)
    taps = [f32(t / total) for t in taps]

    def one(src, across):
        out = []
        for y in range(h):
            for x in range(w):
                o = 0.0
                for i, t in enumerate(taps):
                    sx, sy = (x + i - r, y) if across else (x, y + i - r)
                    v = src[min(h - 1, max(0, sy)) * w + min(w - 1, max(0, sx))]
                    o = f32(o + f32(v * t))
                out.append(o)
        return out

    return one(one(plane, True), False)


def picture(layer, n, w, src_map):
    """D-583's steps S1 and S2: the lightness plane, its size, where the drawing's corner lies on
    it and whether it mirrors; None for Internal."""
    if w["source"] == "internal":
        return None
    if w["source"] == "this_layer":
        pic, origin = layer, (-layer["left"], -layer["top"])
    else:
        pic, origin = src_map, (0, 0)
    plane = [luminance(p) for p in pic["px"]]
    if n["softness"] > 0:
        plane = held_blur(plane, pic["w"], pic["h"], n["softness"] / 10)
    return {"plane": plane, "w": pic["w"], "h": pic["h"], "origin": origin,
            "mirror": w["mirror_at_edge"] == "on"}


def curl_noise(layer, n, w, moved, float_depth, src_map=None):
    k = setup(n, moved)
    src = picture(layer, n, w, src_map)
    hold = not (w["clip_hdr_results"] == "off" and float_depth)
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            p = layer["px"][j * layer["w"] + i]
            a = p[3]
            x, y = layer["left"] + i, layer["top"] + j
            # Only the pixels a frame can show are worked (the tiled case's buffer is 300 by 300);
            # the rest are not in any frame here.
            if a <= 0 or not (-16 <= x < W + 16 and -16 <= y < H + 16):
                px.append(p)
                continue
            dx, dy, cv, grey = curl_at(k, x + 0.5, y + 0.5, src)
            if w["view"] == "input_noise":
                o = [0.5 + 0.5 * cv] * 3
            elif w["view"] == "curl_generation":
                o = [0.5 + 0.5 * dx, 0.5 + 0.5 * dy, 0.5 + 0.5 * cv]
            else:
                o = [grey] * 3
            o = [0.5 + (v - 0.5) * n["contrast"] / 100 + n["brightness"] / 100 for v in o]
            o = [min(1.0, max(0.0, v)) if hold else max(0.0, v) for v in o]
            ch = w["channel"]
            if ch == "rgb":
                px.append([srgb_to_linear(v) * a for v in o] + [a])
            elif ch == "alpha":
                t = a * min(1.0, max(0.0, (o[0] + o[1] + o[2]) / 3))
                px.append([v / a * t for v in p[:3]] + [t])
            else:
                c = CHANNELS.index(ch) - 1
                q = list(p)
                q[c] = srgb_to_linear(o[c]) * a
                px.append(q)
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(source="internal", speed=10, direction=0, size=100, offset=(0, 0), evolution=0,
         turbulence_speed=20, swirl=45, density=0, smoothness=50, vertical_bias=50,
         sample_count=12, sample_radius=30, flow_softness=20, edge_definition=50, flow_falloff=0,
         view="final_render", contrast=100, brightness=0, clip_hdr_results="on", channel="rgb",
         shift=0, tile=False, float_depth=False, source_layer="", softness=0,
         source_mapping="fill", mirror_at_edge="off", rotation=0, scale=100,
         perspective_offset="off", uniform_scaling="on", scale_width=100, scale_height=100):
    c = dict(locals())
    c["drawing"] = "card"
    return c


def small(**k):
    """Noise of 12 pixels (Size 6) and flow lines of radius 4, so the card's 16 by 10 shows them."""
    return case(**{"size": 6, "sample_radius": 4, "sample_count": 6, **k})


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = value_at(c[k], frame_no)
    if isinstance(v, (list, tuple)):
        return [min(hi, max(lo, x)) for x in v]
    return min(hi, max(lo, v))


def layer_of(c):
    layer = {"px": [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
             "left": 0, "top": 0, "w": W, "h": H}
    return motion_tile(layer, 300, 300, "on") if c["tile"] else layer


def ramp(x, y):
    """D-583's other layer: 8 by 5, opaque greys brightening right and down."""
    v = 30 + 25 * x + 10 * y
    return (v, v, v, 255)


MAPS = {"ramp": [[ramp(x, y) for x in range(8)] for y in range(5)]}


def the_map(c):
    """D-189's map of the layer Source Layer names, fitted by Source Mapping."""
    if c["source"] != "other_layer" or c["source_layer"] not in MAPS:
        return None
    rows = MAPS[c["source_layer"]]
    pic = L.pic(len(rows[0]), len(rows), [R.working(p) for row in rows for p in row])
    return L.fit(pic, "center" if c["source_mapping"] == "center" else "stretch", W, H)


def render(c, frame_no):
    n = {k: held(c, k, frame_no) for k in RANGES}
    n.update(perspective_offset=c["perspective_offset"], uniform_scaling=c["uniform_scaling"])
    if c["source"] == "other_layer" and c["source_layer"] not in MAPS:
        return plain(c)  # D-189: a layer not in the composition, the frame without the effect
    words = {k: c[k] for k in WORDS + ("mirror_at_edge",)}
    return frame(curl_noise(layer_of(c), n, words, travel(c, frame_no), c["float_depth"],
                            the_map(c)), c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


STILL = {"turbulence_speed": 0, "speed": 0}

CASES = {
    "FX-CURL-001": ("The settings as they start: size 100, so across the 16 by 10 card the noise "
                    "is one soft cloud, swirling slowly; every shown pixel is a grey, the empty "
                    "pixels stay empty and the soft edge keeps its half covering.", case(), [0, 4]),
    "FX-CURL-002": ("Size 6, sample radius 4, 6 samples: the value noise, 19.2 pixels across, "
                    "carried a little along the flow of a noise 12 pixels across.", small(),
                    [0, 2]),
    "FX-CURL-003": ("View Input Noise: the smooth noise the flow carries, before it moves.",
                    small(view="input_noise"), [0]),
    "FX-CURL-004": ("View Curl Generation: red is the flow across, green the flow down, blue the "
                    "noise.", small(view="curl_generation"), [0]),
    "FX-CURL-005": ("Sample Radius 0: no trace, so the frame is FX-CURL-003's Input Noise.",
                    small(sample_radius=0), [0]),
    "FX-CURL-006": ("Flow Softness 100 (D-749: higher is crisper): only the end of each trace "
                    "is shown, not its average, so it differs from FX-CURL-002.",
                    small(flow_softness=100), [0]),
    "FX-CURL-007": ("Swirl 0, Curl Generation (D-749: Swirl is the flow's speed): no flow, so red "
                    "and green are the middle grey.", small(swirl=0, view="curl_generation"), [0]),
    "FX-CURL-008": ("Swirl 180, Curl Generation: the flow is the noise's curl, unlike "
                    "FX-CURL-007's.", small(swirl=180, view="curl_generation"), [0]),
    "FX-CURL-009": ("Vertical Bias 100, Curl Generation: the noise is stretched tall, so the flow "
                    "runs mostly up and down.", small(vertical_bias=100, view="curl_generation"),
                    [0]),
    "FX-CURL-010": ("Vertical Bias 0, Curl Generation: the noise is stretched wide, so the flow "
                    "runs mostly across.", small(vertical_bias=0, view="curl_generation"), [0]),
    "FX-CURL-011": ("Speed 20, Direction 90, Turbulence Speed 0, Input Noise: the noise drifts 2 "
                    "pixels right a frame, so frame 1 is frame 0 moved 2 right.",
                    small(speed=20, direction=90, turbulence_speed=0, view="input_noise"), [0, 1]),
    "FX-CURL-012": ("Offset 3 right, still, Input Noise: the noise moved 3 pixels right.",
                    small(offset=(3, 0), view="input_noise", **STILL), [0]),
    "FX-CURL-013": ("Turbulence Speed 100, Speed 0: the noise changes from frame to frame in "
                    "place.", small(turbulence_speed=100, speed=0), [0, 2]),
    "FX-CURL-014": ("Evolution 90, still: a different noise from FX-CURL-002's.",
                    small(evolution=90, **STILL), [0]),
    "FX-CURL-015": ("Density 50: the noise half the size, so it is size 3 at density 0, "
                    "FX-CURL-016.", small(density=50, **STILL), [0]),
    "FX-CURL-016": ("Size 3, still.", small(size=3, **STILL), [0]),
    "FX-CURL-017": ("Smoothness 0, still, Input Noise: the fine octaves stronger.",
                    small(smoothness=0, view="input_noise", **STILL), [0]),
    "FX-CURL-018": ("Smoothness 100, still, Input Noise: the coarse octave alone.",
                    small(smoothness=100, view="input_noise", **STILL), [0]),
    "FX-CURL-019": ("Edge Definition 100, still (D-749: the gain, 4): the greys four times as far "
                    "from the middle as FX-CURL-020's, held at black and white.",
                    small(edge_definition=100, **STILL), [0]),
    "FX-CURL-020": ("Edge Definition 0, still: the gain 1, soft greys.",
                    small(edge_definition=0, **STILL), [0]),
    "FX-CURL-021": ("Flow Falloff 100, still: the greys fade towards the middle where the noise is "
                    "low.", small(flow_falloff=100, **STILL), [0]),
    "FX-CURL-022": ("Contrast 300, Brightness 10, still: harder greys, held in 0 and 1.",
                    small(contrast=300, brightness=10, **STILL), [0]),
    "FX-CURL-023": ("Contrast 300, Brightness 10, Clip HDR Results off, in a composition that "
                    "does not work in Float: still held at 1, so the frame is FX-CURL-022's.",
                    small(contrast=300, brightness=10, clip_hdr_results="off", **STILL), [0]),
    "FX-CURL-024": ("Contrast 300, Brightness 10, Clip HDR Results off, in a Float composition: "
                    "past white is kept.",
                    small(contrast=300, brightness=10, clip_hdr_results="off", float_depth=True,
                          **STILL), [0]),
    "FX-CURL-025": ("Channel Red, still: only red is replaced; green and blue are the drawing's.",
                    small(channel="red", **STILL), [0]),
    "FX-CURL-026": ("Channel Alpha, still: the covering times the grey, the colour kept.",
                    small(channel="alpha", **STILL), [0]),
    "FX-CURL-027": ("Sample Count 3.9, still: its whole part counts, so this is sample count 3, "
                    "FX-CURL-028.", small(sample_count=3.9, **STILL), [0]),
    "FX-CURL-028": ("Sample Count 3, still.", small(sample_count=3, **STILL), [0]),
    "FX-CURL-029": ("Sample Radius keyed from 0 at frame 0 to 8 at frame 4, still: frame 0 is the "
                    "Input Noise, frame 4 a longer trace.",
                    small(sample_radius=keyed((0, 0), (4, 8)), **STILL), [0, 2, 4]),
    "FX-CURL-030": ("FX-CURL-002 moved three pixels right: the noise is the drawing's own, so it "
                    "moves with it.", small(shift=3), [0]),
    "FX-CURL-031": ("After a Motion Tile that grows the layer: the noise is worked in the "
                    "drawing's own pixels, so the frame is FX-CURL-002's.", small(tile=True), [0]),
    "FX-CURL-032": ("Swirl -200, Speed 35, Direction 200, Evolution 45, Turbulence Speed 60, "
                    "Density -30, Smoothness 20, Vertical Bias 70, 9 samples, radius 6.5, Flow "
                    "Softness 10, Edge Definition 30, Falloff 40, Contrast 150, Brightness -5, "
                    "Channel Green: the controls together.",
                    small(swirl=-200, speed=35, direction=200, evolution=45, turbulence_speed=60,
                          density=-30, smoothness=20, vertical_bias=70, sample_count=9,
                          sample_radius=6.5, flow_softness=10, edge_definition=30, flow_falloff=40,
                          contrast=150, brightness=-5, channel="green"), [0, 3]),
    # D-583 (D-566: FX-CURL-033 was refused while This Layer was not built).
    "FX-CURL-033": ("Source This Layer, small noise and radius, otherwise as added: the card's own "
                    "lightness is the noise, carried along its own edges; Speed 10 slides it 1 "
                    "pixel up a frame.",
                    small(source="this_layer"), [0, 4]),
    "FX-CURL-045": ("Source This Layer, still, Input Noise: each shown pixel's grey is 0.5 + "
                    "(2 L - 1), L its own luminance times its covering (the gain 2 at Edge "
                    "Definition 50), held in 0 and 1.",
                    small(source="this_layer", view="input_noise", **STILL), [0]),
    "FX-CURL-046": ("Source This Layer, still, Curl Generation: inside a flat patch there is no "
                    "flow (red and green the middle grey); at the edges the flow runs along them.",
                    small(source="this_layer", view="curl_generation", **STILL), [0]),
    "FX-CURL-047": ("Source Other Layer, layer \"ramp\" (8 by 5 greys), Source Mapping Fill, "
                    "still, Input Noise: the ramp stretched over the card.",
                    small(source="other_layer", source_layer="ramp", view="input_noise",
                          **STILL), [0]),
    "FX-CURL-048": ("Source Other Layer, \"ramp\", Source Mapping Center, still, Input Noise: the "
                    "ramp at its own size in the middle, black round it.",
                    small(source="other_layer", source_layer="ramp", source_mapping="center",
                          view="input_noise", **STILL), [0]),
    "FX-CURL-049": ("Source Other Layer, \"ramp\", Fill, still: the ramp carried across its own "
                    "slope.", small(source="other_layer", source_layer="ramp", **STILL), [0]),
    "FX-CURL-050": ("Source This Layer, Softness 20, still, Input Noise: FX-CURL-045's greys "
                    "blurred, sigma 2 pixels.",
                    small(source="this_layer", softness=20, view="input_noise", **STILL), [0]),
    "FX-CURL-051": ("Source This Layer, Offset 5 right, Mirror at Edge on, still, Input Noise: "
                    "left of the card's edge the card is read mirrored.",
                    small(source="this_layer", offset=(5, 0), mirror_at_edge="on",
                          view="input_noise", **STILL), [0]),
    "FX-CURL-052": ("Source This Layer, Offset 5 right, Mirror at Edge off, still, Input Noise: "
                    "left of the card's edge is black.",
                    small(source="this_layer", offset=(5, 0), view="input_noise", **STILL), [0]),
    "FX-CURL-053": ("Source Other Layer naming \"gone\", not in the composition: the frames are "
                    "drawn without the effect, with D-189's warning.",
                    case(source="other_layer", source_layer="gone"), [0, 4]),
    "FX-CURL-054": ("FX-CURL-046 moved three pixels right: the picture is the drawing's own, so "
                    "it moves with it.",
                    small(source="this_layer", view="curl_generation", shift=3, **STILL), [0]),
    "FX-CURL-055": ("FX-CURL-046 after a Motion Tile that grows the layer: the picture is the "
                    "layer as it reaches the effect, so at the card's edges the tiles beyond it "
                    "turn the flow; inside, the frame is FX-CURL-046's.",
                    small(source="this_layer", view="curl_generation", tile=True, **STILL), [0]),
    "FX-CURL-056": ("Source This Layer, Softness keyed from 0 at frame 0 to 30 at frame 4, still, "
                    "Input Noise: frame 0 is FX-CURL-045.",
                    small(source="this_layer", softness=keyed((0, 0), (4, 30)), view="input_noise",
                          **STILL), [0, 4]),
    # D-749 (row 14): speeds are added up frame by frame.
    "FX-CURL-078": ("Speed keyed from 0 at frame 0 to 40 at frame 4, Direction 90, Turbulence "
                    "Speed 0, Input Noise: the drift adds up 0, 1, 2 and 3 pixels, so frame 4 is "
                    "frame 0 moved 6 right (not the 16 that speed 40 times 4 frames would jump to).",
                    small(speed=keyed((0, 0), (4, 40)), direction=90, turbulence_speed=0,
                          view="input_noise"), [0, 2, 4]),
}

INVALID = {
    "FX-CURL-034": ("Source Other Layer, with Source Layer naming no layer.",
                    case(source="other_layer")),
    "FX-CURL-035": ("Source \"noise\", not one of its words.", case(source="noise")),
    "FX-CURL-036": ("Size 0.5, below 1.", case(size=0.5)),
    "FX-CURL-037": ("Speed 101, above 100.", case(speed=101)),
    "FX-CURL-038": ("Sample Count 2, below 3.", case(sample_count=2)),
    "FX-CURL-039": ("Sample Radius 201, above 200.", case(sample_radius=201)),
    "FX-CURL-040": ("Swirl 400, above 360.", case(swirl=400)),
    "FX-CURL-041": ("View \"lines\", not one of its words.", case(view="lines")),
    "FX-CURL-042": ("Channel \"Red\", in capitals, kept as written and not the word.",
                    case(channel="Red")),
    "FX-CURL-043": ("Clip HDR Results \"yes\", not on or off.", case(clip_hdr_results="yes")),
    "FX-CURL-044": ("Density keyed to 150 at frame 4, above 100.",
                    case(density=keyed((0, 0), (4, 150)))),
    "FX-CURL-057": ("Softness 101, above 100.", case(source="this_layer", softness=101)),
    "FX-CURL-058": ("Source Mapping \"stretch\", not fill or center.",
                    case(source="other_layer", source_layer="ramp", source_mapping="stretch")),
    "FX-CURL-059": ("Mirror at Edge \"yes\", not on or off.",
                    case(source="this_layer", mirror_at_edge="yes")),
    "FX-CURL-060": ("Source Layer 3, a number and not a layer's word.",
                    case(source="other_layer", source_layer=3)),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    if c["float_depth"]:
        comp["float_depth"] = True
    t = comp["layers"][0]["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    effects = []
    if c["tile"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.motion_tile", "enabled": True,
                        "parameters": {"output_width": 300, "output_height": 300,
                                       "mirror": "on"}})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.curl_noise",
                    "enabled": True,
                    "parameters": {k: (c[k] if k in WORDS else setting_json(c[k]))
                                   for k in NAMES}})
    for k, v in {**SOURCE_KEYS, **TRANSFORM_KEYS}.items():
        if c[k] != v:
            effects[-1]["parameters"][k] = c[k] if isinstance(v, str) else setting_json(c[k])
    comp["layers"][0]["effects"] = effects
    if c["source_layer"] in MAPS:
        p["assets"].append(L.still(c["source_layer"], f"media/{c['source_layer']}.png"))
        comp["layers"].append(L.raster(c["source_layer"], "asset-" + c["source_layer"],
                                       enabled=False, out_frame=S.FRAMES))
        comp["layer_order"].append(c["source_layer"])
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    with open(OUT / name, "w", encoding="utf-8", newline="") as f:
        f.write(json.dumps(project_json(fx, c), indent=2) + "\n")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in {**DRAWINGS, **MAPS}.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        if c["source"] == "other_layer" and c["source_layer"] not in MAPS:
            expected["cases"][fx]["warning"] = "EFFECT_LAYER_MISSING"
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
    with open(OUT / "expected_curl_noise.json", "w", encoding="utf-8", newline="") as f:
        f.write(json.dumps(expected, indent=1) + "\n")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    drawn = plain(case())
    shown = [i for i in range(W * H) if drawn[i][3] > 0]
    enc = lambda p: [S.linear_to_srgb(v / p[3]) for v in p[:3]]  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda f, g, e=1e-9: all(near(f[i], g[i], e) for i in range(W * H))  # noqa: E731
    differ = lambda f, g: sum(not near(f[i], g[i]) for i in shown) > len(shown) * 0.5  # noqa: E731
    soft = at(15, 4)

    # The slope is the noise's own: a small step's change over the step, for both lattices.
    e = 1e-6
    for fn in (lambda x, y: value_slope(8, x, y, 0.4), lambda x, y: two(8, x, y, 0.4)):
        v0, gx, gy = fn(2.3, 7.6)
        assert abs((fn(2.3 + e, 7.6)[0] - v0) / e - gx) < 1e-4
        assert abs((fn(2.3, 7.6 + e)[0] - v0) / e - gy) < 1e-4
    # The flow is the curl of the stretched noise: it never gathers or spreads (its divergence,
    # worked from small steps, is nought), at any bias.
    for b in (0, 30, 50, 100):
        n = {k: held(small(vertical_bias=b, swirl=180, **STILL), k, 0) for k in RANGES}
        n.update(perspective_offset="off", uniform_scaling="on")
        k = setup(n, (0, 0, 0))
        for x, y in ((3.2, 4.1), (11.7, 2.2)):
            div = ((flow(k, x + e, y, None)[0] - flow(k, x - e, y, None)[0])
                   + (flow(k, x, y + e, None)[1] - flow(k, x, y - e, None)[1])) / (2 * e)
            assert abs(div) < 1e-4, (b, div)

    for f in c["FX-CURL-001"].values():
        for i in range(W * H):
            if drawn[i][3] == 0:
                assert f[i] == [0.0] * 4
            else:
                assert f[i][3] == drawn[i][3]
                g = enc(f[i])
                assert abs(g[0] - g[1]) < 1e-9 and abs(g[1] - g[2]) < 1e-9
        assert f[soft][3] == 128 / 255
    assert not same(c["FX-CURL-001"]["0"], c["FX-CURL-001"]["4"])
    two_ = c["FX-CURL-002"]["0"]
    assert differ(two_, drawn) and differ(two_, c["FX-CURL-003"]["0"])
    assert same(c["FX-CURL-005"]["0"], c["FX-CURL-003"]["0"], 1e-6)  # single precision
    assert differ(c["FX-CURL-006"]["0"], two_) and differ(c["FX-CURL-006"]["0"], c["FX-CURL-003"]["0"])
    four = c["FX-CURL-004"]["0"]
    assert sum(abs(enc(four[i])[0] - enc(four[i])[1]) > 1e-3 for i in shown) > len(shown) / 2
    for i in shown:
        assert near(enc(c["FX-CURL-007"]["0"][i])[:2], [0.5, 0.5], 1e-6)
    assert differ(c["FX-CURL-007"]["0"], c["FX-CURL-008"]["0"])
    across = lambda f, ch: sum(abs(enc(f[i])[ch] - 0.5) for i in shown)  # noqa: E731
    k9, k10 = c["FX-CURL-009"]["0"], c["FX-CURL-010"]["0"]
    assert across(k9, 0) < across(k9, 1) and across(k10, 1) < across(k10, 0)
    k11 = c["FX-CURL-011"]
    for y in range(H):
        for x in range(3, W):
            i, j = at(x, y), at(x - 2, y)
            if drawn[i][3] > 0 and drawn[j][3] > 0:
                assert abs(enc(k11["1"][i])[0] - enc(k11["0"][j])[0]) < 1e-5
    still_input = render(small(view="input_noise", **STILL), 0)
    for y in range(H):
        for x in range(4, W):
            i, j = at(x, y), at(x - 3, y)
            if drawn[i][3] > 0 and drawn[j][3] > 0:
                assert abs(enc(c["FX-CURL-012"]["0"][i])[0] - enc(still_input[j])[0]) < 1e-5
    assert differ(c["FX-CURL-013"]["0"], c["FX-CURL-013"]["2"])
    assert differ(c["FX-CURL-014"]["0"], render(small(**STILL), 0))
    assert same(c["FX-CURL-015"]["0"], c["FX-CURL-016"]["0"])
    assert differ(c["FX-CURL-017"]["0"], c["FX-CURL-018"]["0"])
    k19, k20 = c["FX-CURL-019"]["0"], c["FX-CURL-020"]["0"]
    for i in shown:
        g19, g20 = enc(k19[i])[0] - 0.5, enc(k20[i])[0] - 0.5
        assert abs(g19 - max(-0.5, min(0.5, 4 * g20))) < 1e-5, (g19, g20)
    assert differ(c["FX-CURL-021"]["0"], render(small(**STILL), 0))
    k22, k24 = c["FX-CURL-022"]["0"], c["FX-CURL-024"]["0"]
    assert same(c["FX-CURL-023"]["0"], k22)
    assert any(k22[i][0] == drawn[i][3] for i in shown)  # some held at white
    assert any(k24[i][0] > drawn[i][3] + 1e-6 for i in shown)  # some past white
    red = c["FX-CURL-025"]["0"]
    for i in shown:
        assert red[i][1:] == drawn[i][1:]
    assert differ(red, drawn)
    alpha = c["FX-CURL-026"]["0"]
    for i in shown:
        if alpha[i][3] > 0:
            assert near(enc(alpha[i]), enc(drawn[i]), 1e-9)
        assert alpha[i][3] <= drawn[i][3] + 1e-12
    assert same(c["FX-CURL-027"]["0"], c["FX-CURL-028"]["0"])
    k29 = c["FX-CURL-029"]
    assert same(k29["0"], still_input, 1e-6)
    assert differ(k29["4"], k29["0"]) and differ(k29["2"], k29["4"])
    moved = c["FX-CURL-030"]["0"]
    assert all(moved[at(x, y)] == two_[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert same(c["FX-CURL-031"]["0"], two_)
    k32 = c["FX-CURL-032"]
    assert differ(k32["0"], k32["3"])
    for i in shown:
        assert k32["0"][i][0] == drawn[i][0] and k32["0"][i][2] == drawn[i][2]
    # D-583.
    assert differ(c["FX-CURL-033"]["0"], c["FX-CURL-001"]["0"])
    assert differ(c["FX-CURL-033"]["0"], c["FX-CURL-033"]["4"])
    k45 = c["FX-CURL-045"]["0"]
    for i in shown:
        assert abs(enc(k45[i])[0] - min(1.0, max(0.0, 2 * luminance(drawn[i]) - 0.5))) < 1e-6
    k46, mid_enc = c["FX-CURL-046"]["0"], 0.5
    inner = at(7, 5)  # skin all round
    assert near(enc(k46[inner])[:2], [mid_enc, mid_enc], 1e-6)
    assert any(abs(enc(k46[i])[0] - 0.5) > 0.1 for i in shown)
    k47, k48 = c["FX-CURL-047"]["0"], c["FX-CURL-048"]["0"]
    for y in range(H - 1):
        row = [enc(k47[at(x, y)])[0] for x in range(1, W - 1)]
        assert all(a <= b for a, b in zip(row, row[1:])) and row[0] < row[-1], y
    for y in range(H - 1):
        for x in range(1, 4):
            assert k48[at(x, y)][:3] == [0.0, 0.0, 0.0]
    assert differ(c["FX-CURL-049"]["0"], k47)
    k50 = c["FX-CURL-050"]["0"]
    rough = lambda f: sum(abs(enc(f[at(x + 1, y)])[0] - enc(f[at(x, y)])[0])  # noqa: E731
                          for x in range(1, W - 2) for y in range(H - 1))
    assert rough(k50) < rough(k45) and differ(k50, k45)
    k51, k52 = c["FX-CURL-051"]["0"], c["FX-CURL-052"]["0"]
    for y in range(H - 1):
        for x in range(1, 5):
            assert k52[at(x, y)][:3] == [0.0, 0.0, 0.0]
    assert any(k51[at(x, y)][0] > 0 for x in range(1, 4) for y in range(H - 1))
    for y in range(H - 1):
        for x in range(6, W):
            assert k51[at(x, y)] == k52[at(x, y)]
    assert c["FX-CURL-053"]["0"] == c["FX-CURL-053"]["4"] == drawn
    k54 = c["FX-CURL-054"]["0"]
    assert all(k54[at(x, y)] == k46[at(x - 3, y)] for x in range(3, W) for y in range(H))
    k55 = c["FX-CURL-055"]["0"]
    assert k55[inner] == k46[inner] and k55 != k46
    k56 = c["FX-CURL-056"]
    assert k56["0"] == k45 and differ(k56["4"], k45)
    # D-749, row 14: frame 4 is frame 0 moved 6 right, the drift added up.
    k78 = c["FX-CURL-078"]
    assert k78["0"] == still_input
    for y in range(H):
        for x in range(7, W):
            i, j = at(x, y), at(x - 6, y)
            if drawn[i][3] > 0 and drawn[j][3] > 0:
                assert abs(enc(k78["4"][i])[0] - enc(k78["0"][j])[0]) < 1e-5
    assert travel(small(speed=10), 7) == (0.0, -7.0, 14.0)  # not keyed: as before D-749
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == drawn
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(v >= 0 for v in p[:3]), (name, p)
                if name != "FX-CURL-024":
                    assert all(v <= p[3] + 1e-12 for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
