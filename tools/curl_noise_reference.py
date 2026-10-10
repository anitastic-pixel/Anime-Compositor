"""Curl Noise, worked a second way.

D-446 adds `core.curl_noise`, after After Effects' Curl Noise (After Effects 26.3 beta, "Curl
Noise: swirling, animated 2D noise", Adobe's community announcement): a swirling noise whose
flow lines follow the curl of a smooth noise field (Bridson's "curl noise", 2007). Its settings
are After Effects' Source (Internal only: This Layer and Other Layer are refused in a sentence,
not built), Speed and Direction, the Transform's Size and Offset, Evolution, Turbulence Speed,
Swirl, Density, Smoothness, Vertical Bias, Sample Count, Sample Radius, Flow Softness, Edge
Definition, Flow Falloff, View, Contrast, Brightness, Clip HDR Results and Channel. Adobe
publishes no formula; the numbers below are this program's own rule, built on P0-19's value
noise (`grade::value`, D-127, D-128), and nothing is ported.

The rule, at a point p of the drawing's own pixels (its top-left corner (0, 0), however far an
effect above grew the buffer):

1. The field's size is s = size 2^(-density / 50); it drifts speed / 10 pixels a frame towards
   Direction (0 up, 90 right), so p' = p - offset - drift frame. Its depth is
   z = (evolution + turbulence_speed frame / 10) / 360.
2. The noise N: four octaves of P0-19's smooth value noise for seed 0, octave o at
   (p' / s 2^o, z 2^o) on channel 8 o, weighted 1, r, r^2, r^3 with r = 0.7 - 0.4 smoothness / 100,
   over the sum of the weights; and its slope, worked exactly from the smoothing curve's own
   slope 30 t^2 (1 - t)^2, over all eight corners of each cell.
3. The flow: the slope turned by -90 degrees + swirl N degrees (at Swirl 0 exactly the curl,
   which never gathers or spreads), then its across part times min(1, 2 (1 - b)) and its down part
   times min(1, 2 b), b = vertical_bias / 100, and made one long (left nought where it is
   nought).
4. The seed: P0-19's value noise on channel 7 at (p' / (s / 50), z), blocks and smooth mixed by
   edge_definition / 100 (1 all blocks).
5. These four numbers, the flow's two, N and the seed, are worked for every pixel of the buffer
   and M = ceil(sample_radius) + 2 round it, kept in single precision, as the card keeps them.
6. Final Render: from the pixel's middle the flow is followed both ways, n = floor(sample_count)
   steps of sample_radius / n each, reading the four numbers between pixels as
   `render::sample_bilinear` does, until the flow is nought; the seed is averaged with weight
   1 at the middle and 1 - i / (n + 1) at step i, and scaled by the weights' sum over the root of
   their squares' sum, L. With f = flow_softness / 100, the lines are
   N + (1 - f) min(1, sample_radius / 8) (L - N), then times 1 - flow_falloff / 100 (0.5 - 0.5 N),
   and the grey is 0.5 + 0.5 times that.
   Input Noise: the grey is 0.5 + 0.5 N. Curl Generation: red 0.5 + 0.5 the flow across, green
   0.5 + 0.5 the flow down, blue 0.5 + 0.5 N.
7. Each channel o becomes 0.5 + (o - 0.5) contrast / 100 + brightness / 100, held at 0, and at 1
   too unless Clip HDR Results is off in a composition that works in Float.
8. Channel RGB: the colour, through the sRGB curve, replaces the pixel's at its covering; Red,
   Green or Blue: only that channel is replaced; Alpha: the covering is multiplied by the three's
   mean, held in 0 and 1, and the colour kept.

A pixel that does not show is left.

Settings: `source` `internal` (when added; `this_layer` and `other_layer` refused); `speed` 0
to 100, 10 when added; `direction` -100000 to 100000 degrees, 0; `size` 1 to 1000 pixels, 100;
`offset` a point, -100000 to 100000, [0, 0]; `evolution` -100000 to 100000 degrees, 0;
`turbulence_speed` 0 to 200, 20; `swirl` -360 to 360, 45; `density` -100 to 100, 0; `smoothness`
0 to 100, 50; `vertical_bias` 0 to 100, 50; `sample_count` 3 to 24, 12, its whole part;
`sample_radius` 0 to 200 pixels, 30; `flow_softness` 0 to 100, 20; `edge_definition` 0 to 100,
50; `flow_falloff` 0 to 100, 0; `view` `final_render` (when added), `input_noise` or
`curl_generation`; `contrast` 0 to 1000, 100; `brightness` -1000 to 1000, 0; `clip_hdr_results`
`on` (when added) or `off`; `channel` `rgb` (when added), `red`, `green`, `blue` or `alpha`.
Every number can be keyed.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, keeping the single-precision steps the rule names.

Every case is a composition 16 by 10 holding Noise's card, the same size, unmoved unless the case
says. The expected frames are in `Fixtures/curl_noise/expected_curl_noise.json`.

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
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
from noise_reference import DRAWINGS, u  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "curl_noise"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"speed": (0, 100), "direction": (-100000, 100000), "size": (1, 1000),
          "offset": (-100000, 100000), "evolution": (-100000, 100000),
          "turbulence_speed": (0, 200), "swirl": (-360, 360), "density": (-100, 100),
          "smoothness": (0, 100), "vertical_bias": (0, 100), "sample_count": (3, 24),
          "sample_radius": (0, 200), "flow_softness": (0, 100), "edge_definition": (0, 100),
          "flow_falloff": (0, 100), "contrast": (0, 1000), "brightness": (-1000, 1000)}
WORDS = ("source", "view", "clip_hdr_results", "channel")
NAMES = ("source", "speed", "direction", "size", "offset", "evolution", "turbulence_speed",
         "swirl", "density", "smoothness", "vertical_bias", "sample_count", "sample_radius",
         "flow_softness", "edge_definition", "flow_falloff", "view", "contrast", "brightness",
         "clip_hdr_results", "channel")
CHANNELS = ("rgb", "red", "green", "blue", "alpha")


def f32(v):
    return struct.unpack("f", struct.pack("f", v))[0]


# --- the rule -------------------------------------------------------------------------------

def fade(t):
    return t * t * t * (t * (6 * t - 15) + 10)


def slope(t):
    return 30 * t * t * (1 - t) * (1 - t)


def value(ch, x, y, z, block):
    """grade::value for seed 0, a corner that weighs nothing skipped."""
    i, j, k = math.floor(x), math.floor(y), math.floor(z)
    s = [0.0, 0.0, fade(z - k)] if block else [fade(x - i), fade(y - j), fade(z - k)]
    v = 0.0
    for corner in range(8):
        d = [corner & 1, (corner >> 1) & 1, corner >> 2]
        w = 1.0
        for a in range(3):
            w *= s[a] if d[a] == 1 else 1 - s[a]
        if w != 0:
            v += w * u(0, i + d[0], j + d[1], k + d[2], ch)
    return v


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


def setup(n, frame_no):
    s = n["size"] * 2 ** (-n["density"] / 50)
    t = math.radians(n["direction"])
    drift = (math.sin(t) * n["speed"] / 10, -math.cos(t) * n["speed"] / 10)
    z = (n["evolution"] + n["turbulence_speed"] * frame_no / 10) / 360
    r = 0.7 - 0.4 * n["smoothness"] / 100
    b = n["vertical_bias"] / 100
    return {"s": s, "shift": (n["offset"][0] + drift[0] * frame_no, n["offset"][1] + drift[1] * frame_no),
            "z": z, "amps": [1, r, r * r, r * r * r], "bias": (min(1.0, 2 * (1 - b)), min(1.0, 2 * b)),
            "edge": n["edge_definition"] / 100}


def field_at(k, swirl, px, py):
    """The four numbers at the drawing's point (px, py), kept in single precision."""
    x, y = px - k["shift"][0], py - k["shift"][1]
    s = k["s"]
    nv = gx = gy = total = 0.0
    fine = 1.0
    for o in range(4):
        v, a, b = value_slope(8 * o, x / s * fine, y / s * fine, k["z"] * fine)
        amp = k["amps"][o]
        nv += amp * v
        gx += amp * fine * a
        gy += amp * fine * b
        total += amp
        fine *= 2
    nv, gx, gy = nv / total, gx / total / s, gy / total / s
    t = math.radians(-90 + swirl * nv)
    c, sn = math.cos(t), math.sin(t)
    vx = (c * gx - sn * gy) * k["bias"][0]
    vy = (sn * gx + c * gy) * k["bias"][1]
    length = math.hypot(vx, vy)
    d = (vx / length, vy / length) if length > 0 else (0.0, 0.0)
    cell, e = s / 50, k["edge"]
    sx, sy = x / cell, y / cell
    if e == 1:
        seed = value(7, sx, sy, k["z"], True)
    elif e == 0:
        seed = value(7, sx, sy, k["z"], False)
    else:
        seed = e * value(7, sx, sy, k["z"], True) + (1 - e) * value(7, sx, sy, k["z"], False)
    return [f32(d[0]), f32(d[1]), f32(nv), f32(seed)]


def bilinear(get, gw, gh, x, y):
    """render::sample_bilinear on the field, its weights and sums in single precision."""
    fx, fy = x - 0.5, y - 0.5
    x0, y0 = math.floor(fx), math.floor(fy)
    ux, uy = fx - x0, fy - y0
    out = [0.0] * 4
    for dy, wy in ((0, 1 - uy), (1, uy)):
        sy = y0 + dy
        if wy == 0 or sy < 0 or sy >= gh:
            continue
        for dx, wx in ((0, 1 - ux), (1, ux)):
            sx = x0 + dx
            if wx == 0 or sx < 0 or sx >= gw:
                continue
            weight = f32(wx * wy)
            p = get(sx, sy)
            out = [f32(out[c] + f32(p[c] * weight)) for c in range(4)]
    return out


def curl_noise(layer, n, w, frame_no, float_depth):
    k = setup(n, frame_no)
    radius, steps = n["sample_radius"], math.floor(n["sample_count"])
    margin = math.ceil(radius) + 2
    gw, gh = layer["w"] + 2 * margin, layer["h"] + 2 * margin
    memo = {}

    def get(gx, gy):
        if (gx, gy) not in memo:
            memo[(gx, gy)] = field_at(k, n["swirl"], layer["left"] + gx - margin + 0.5,
                                      layer["top"] + gy - margin + 0.5)
        return memo[(gx, gy)]

    fs, ff = n["flow_softness"] / 100, n["flow_falloff"] / 100
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
            dx, dy, nv, seed = get(i + margin, j + margin)
            if w["view"] == "input_noise":
                o = [0.5 + 0.5 * nv] * 3
            elif w["view"] == "curl_generation":
                o = [0.5 + 0.5 * dx, 0.5 + 0.5 * dy, 0.5 + 0.5 * nv]
            else:
                h = radius / steps
                sw, sws, ss = 1.0, 1.0, seed
                for way in (1, -1):
                    qx, qy = i + margin + 0.5, j + margin + 0.5
                    ex, ey = dx, dy
                    for step in range(1, steps + 1):
                        length = math.hypot(ex, ey)
                        if length < 1e-6:
                            break
                        qx += way * h * ex / length
                        qy += way * h * ey / length
                        ex, ey, _, sv = bilinear(get, gw, gh, qx, qy)
                        wt = 1 - step / (steps + 1)
                        ss += wt * sv
                        sw += wt
                        sws += wt * wt
                lines = ss / sw * (sw / math.sqrt(sws))
                mixed = nv + (1 - fs) * min(1.0, radius / 8) * (lines - nv)
                v = 0.5 + 0.5 * (mixed * (1 - ff * (0.5 - 0.5 * nv)))
                o = [v] * 3
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
         shift=0, tile=False, float_depth=False):
    c = dict(locals())
    c["drawing"] = "card"
    return c


def small(**k):
    """Cells of 6 pixels and flow lines 4 long, so the card's 16 by 10 shows them."""
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


def render(c, frame_no):
    n = {k: held(c, k, frame_no) for k in RANGES}
    return frame(curl_noise(layer_of(c), n, {k: c[k] for k in WORDS}, frame_no, c["float_depth"]),
                 c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


STILL = {"turbulence_speed": 0, "speed": 0}

CASES = {
    "FX-CURL-001": ("The settings as they start: size 100, so across the 16 by 10 card the noise "
                    "is one soft cloud, swirling slowly; every shown pixel is a grey, the empty "
                    "pixels stay empty and the soft edge keeps its half covering.", case(), [0, 4]),
    "FX-CURL-002": ("Size 6, sample radius 4, 6 samples: grey flow lines through cells of 6 "
                    "pixels.", small(), [0, 2]),
    "FX-CURL-003": ("View Input Noise: the smooth noise the flow follows, without its lines.",
                    small(view="input_noise"), [0]),
    "FX-CURL-004": ("View Curl Generation: red is the flow across, green the flow down, blue the "
                    "noise.", small(view="curl_generation"), [0]),
    "FX-CURL-005": ("Sample Radius 0: no lines, so the frame is FX-CURL-003's Input Noise.",
                    small(sample_radius=0), [0]),
    "FX-CURL-006": ("Flow Softness 100: the lines softened away, FX-CURL-003's Input Noise again.",
                    small(flow_softness=100), [0]),
    "FX-CURL-007": ("Swirl 0, Curl Generation: the flow is the noise's curl, crossing its slope at "
                    "a right angle.", small(swirl=0, view="curl_generation"), [0]),
    "FX-CURL-008": ("Swirl 180, Curl Generation: the flow turns with the noise, unlike "
                    "FX-CURL-007's.", small(swirl=180, view="curl_generation"), [0]),
    "FX-CURL-009": ("Vertical Bias 100, Curl Generation: the flow only runs up and down, so red is "
                    "the middle grey throughout.", small(vertical_bias=100, view="curl_generation"),
                    [0]),
    "FX-CURL-010": ("Vertical Bias 0, Curl Generation: the flow only runs across, so green is the "
                    "middle grey throughout.", small(vertical_bias=0, view="curl_generation"), [0]),
    "FX-CURL-011": ("Speed 20, Direction 90, Turbulence Speed 0, Input Noise: the noise drifts 2 "
                    "pixels right a frame, so frame 1 is frame 0 moved 2 right.",
                    small(speed=20, direction=90, turbulence_speed=0, view="input_noise"), [0, 1]),
    "FX-CURL-012": ("Offset 3 right, still, Input Noise: the noise moved 3 pixels right.",
                    small(offset=(3, 0), view="input_noise", **STILL), [0]),
    "FX-CURL-013": ("Turbulence Speed 100, Speed 0: the noise changes from frame to frame in "
                    "place.", small(turbulence_speed=100, speed=0), [0, 2]),
    "FX-CURL-014": ("Evolution 90, still: a different noise from FX-CURL-002's.",
                    small(evolution=90, **STILL), [0]),
    "FX-CURL-015": ("Density 50: the field's cells half the size, so it is size 3 at density 0, "
                    "FX-CURL-016.", small(density=50, **STILL), [0]),
    "FX-CURL-016": ("Size 3, still.", small(size=3, **STILL), [0]),
    "FX-CURL-017": ("Smoothness 0, still, Input Noise: the fine octaves stronger.",
                    small(smoothness=0, view="input_noise", **STILL), [0]),
    "FX-CURL-018": ("Smoothness 100, still, Input Noise: the fine octaves weaker.",
                    small(smoothness=100, view="input_noise", **STILL), [0]),
    "FX-CURL-019": ("Edge Definition 100, still: the lines streak blocky seeds.",
                    small(edge_definition=100, **STILL), [0]),
    "FX-CURL-020": ("Edge Definition 0, still: the lines streak smooth seeds.",
                    small(edge_definition=0, **STILL), [0]),
    "FX-CURL-021": ("Flow Falloff 100, still: the lines fade where the noise is low.",
                    small(flow_falloff=100, **STILL), [0]),
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
                    "Input Noise, frame 4 long lines.",
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
}

INVALID = {
    "FX-CURL-033": ("Source This Layer, not built.", case(source="this_layer")),
    "FX-CURL-034": ("Source Other Layer, not built.", case(source="other_layer")),
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
    (OUT / "expected_curl_noise.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
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

    # The slope is the noise's own: a small step's change over the step.
    v0, gx, gy = value_slope(8, 2.3, 7.6, 0.4)
    e = 1e-6
    assert abs((value_slope(8, 2.3 + e, 7.6, 0.4)[0] - v0) / e - gx) < 1e-4
    assert abs((value_slope(8, 2.3, 7.6 + e, 0.4)[0] - v0) / e - gy) < 1e-4
    assert abs(v0 - value(8, 2.3, 7.6, 0.4, False)) < 1e-12

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
    two = c["FX-CURL-002"]["0"]
    assert differ(two, drawn) and differ(two, c["FX-CURL-003"]["0"])
    assert same(c["FX-CURL-005"]["0"], c["FX-CURL-003"]["0"])
    assert same(c["FX-CURL-006"]["0"], c["FX-CURL-003"]["0"])
    four = c["FX-CURL-004"]["0"]
    assert sum(abs(enc(four[i])[0] - enc(four[i])[1]) > 1e-3 for i in shown) > len(shown) / 2
    assert differ(c["FX-CURL-007"]["0"], c["FX-CURL-008"]["0"])
    mid = srgb_to_linear(0.5)
    for i in shown:
        a = drawn[i][3]
        assert abs(c["FX-CURL-009"]["0"][i][0] - mid * a) < 1e-6
        assert abs(c["FX-CURL-010"]["0"][i][1] - mid * a) < 1e-6
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
    assert differ(c["FX-CURL-019"]["0"], c["FX-CURL-020"]["0"])
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
    assert same(k29["0"], still_input)
    assert differ(k29["4"], k29["0"]) and differ(k29["2"], k29["4"])
    moved = c["FX-CURL-030"]["0"]
    assert all(moved[at(x, y)] == two[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert same(c["FX-CURL-031"]["0"], two)
    k32 = c["FX-CURL-032"]
    assert differ(k32["0"], k32["3"])
    for i in shown:
        assert k32["0"][i][0] == drawn[i][0] and k32["0"][i][2] == drawn[i][2]
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
