"""Wave World, worked a second way.

D-630 adds Wave World (`core.wave_world`), after After Effects' Wave World (Simulation): water
waves on a grid, drawn as a grey height map. Adobe's CS5 help names the controls and says what
several do (the pre-roll runs on the first frame's settings; the ground layer is read only at its
first frame, its brightness the floor's height, waves running faster in deep water; Wave Strength
0 makes no waves from the ground's movement; Phase 0 rises first, 180 dips first). It publishes
no formula, ranges or settings when added, so those are ours (document 21), and nothing is
ported. The view stays the height map: Wireframe Preview and its three controls come with D-635.

No state is carried from frame to frame (D-610's rule): each frame runs the water from its start
again, so a frame drawn alone equals the same frame drawn after the ones before it.

The settings. `brightness` 0 to 1, 0.5 when added; `contrast` 0 to 10, 0.5; `gamma` 0.1 to 10,
1; `render_dry_areas` "level" (when added) or "transparent"; `transparency` 0 to 1, 0;
`grid_resolution` 1 to 600, 60, rounded to the nearest whole number (halves away from zero);
`grid_res_downsamples` "off" (when added) or "on"; `wave_speed` 0 to 1 layer widths a second,
0.25; `damping` 0 to 1, 0.05; `reflect_edges` "none" (when added) or "all"; `pre_roll` 0 to 10
seconds, 0; `ground_layer` D-189's layer word, "" when added, fitted as `stretch` fits it;
`steepness` 0 to 1, 0.25; `height` 0 to 1, 0.5; `wave_strength` 0 to 10, 0. Two producers, k = 1
and 2: `producer_k_type` "ring" (when added) or "line"; `producer_k_position` x then y, -1000 to
1000 per cent of the layer's width and height, [50, 50] and [75, 25]; `producer_k_height` (the
line's length for a line) and `producer_k_width` 0 to 10 layer widths, 0.1; `producer_k_angle`
-3600 to 3600 degrees, 0; `producer_k_amplitude` 0 to 10, 0.5 and 0 (the second is off when
added); `producer_k_frequency` 0 to 30 a second, 1; `producer_k_phase` -3600 to 3600 degrees, 0.
Every number is keyable and held to its range; the words are exact.

The rule, on a layer W by H pixels at r frames a second, drawn at composition frame n, its in
point n0; lengths are in layer widths, x right and y down from the top left.

1. The settings history. Sample j is every number at composition frame n0 + j, j = 0 .. n - n0.
   At a time f of the way from sample j to j + 1 a setting is read in a straight line between
   them. Grid Resolution, Grid Res Downsamples, Reflect Edges and Pre-roll are read from sample
   0 only, and the ground picture at frame n0 only.
2. The grid. N = grid resolution (with downsamples on, times the draft's scale, rounded, at
   least 1) columns and M = max(1, round(N H / W)) rows; cell (i, j) has its centre at
   X = (i + 1/2) dx, Y = (j + 1/2) dy, dx = 1 / N, dy = H / (M W).
3. The ground. g = document 21's luma (clamped to 0..1) of the ground picture read bilinearly at
   the cell centre's pixel (X W, Y W); with no ground picture (none named, a layer not in the
   composition, with D-189's warning, or a layer not begun at n0) g = 0. At a time, the floor
   e = steepness g and the depth d = height - e; a cell with d <= 0 is dry, else wet with speed
   squared c^2 = 2 d wave_speed^2.
4. The steps. The water's height h and speed u are 0 in every cell at the start. The pre-roll
   is K = ceil(pre_roll r) frame lengths, each of q0 steps, all on sample 0. Then frame interval
   j (from sample j to j + 1, j = 0 .. n - n0 - 1) takes q_j steps of dt = 1 / (r q_j), step i
   running from f = i / q_j to (i + 1) / q_j. q = max(4, ceil(1.6 cmax / (r min(dx, dy)))),
   with cmax the largest wave_speed sqrt(2 height) of the samples the interval reads.
5. One step, from settings at its start (a) and its end (b): with the wet cells and c^2 at a,
   for each wet cell, its four neighbours' heights, where a dry neighbour, or the layer's edge
   with reflect edges "all", gives the cell's own h, and the layer's edge with "none" gives
   h - s u / c (s = dx across, dy down: the first-order absorbing edge),
       u += dt c^2 ((hl + hr - 2 h) / dx^2 + (hu + hd - 2 h) / dy^2)   (every wet cell at once)
       u *= exp(-10 damping dt);   h += dt u
   then each producer adds w_b (P(b) - P(a)) and the ground adds wave_strength_b (e_b - e_a).
   A dry cell's h and u are 0.
6. A producer's drive is P = amplitude (sin(T + phase) - sin(phase)), with T = 2 pi frequency s
   for a time s before the in point, and from it the frequency's running sum: T at sample j + f
   is T_j + (2 pi / r)(F_j f + (F_(j+1) - F_j) f^2 / 2), T_0 = 0. Its weight at a cell is read
   about the centre (px W / 100, py H / 100) / W, the cell's offset turned back by the angle,
   (a', b') = (ox cos + oy sin, -ox sin + oy cos): for "ring", rr = (2 a' / width)^2 +
   (2 b' / height)^2 and w = (1 - rr)^2 where rr < 1; for "line", w = (1 - (2 b' / width)^2)^2
   where |a'| <= height / 2 and |2 b'| < width; else 0, and 0 when width or height is 0.
7. The map, at buffer pixel (x, y) of the layer: gx = (x + 1/2) N / W - 1/2 and
   gy = (y + 1/2) M / H - 1/2 held inside the grid, h and d read bilinearly from the cells (d
   from the current frame's height and steepness). Where d <= 0 the pixel is dry: "level" draws
   it as h = 0, "transparent" leaves it clear. Else, and for "level", v = clamp(brightness +
   contrast h, 0, 1), grey = v^(1 / gamma), a = clamp(1 - transparency (1 - max(d, 0) /
   height), 0, 1) (a = 1 - transparency where height is 0), and the pixel is
   (srgb_to_linear(grey) a, ..., a). The layer's own picture is not used; the layer never grows.

A draft with downsamples off draws the same water on fewer pixels, every length being a share of
the layer.

**This file never runs the build's code path.** It works in double precision on lists. The rule
steps where a cell or a pixel is just dry and where a line producer ends; every case is checked
that no cell or pixel lies near one.

Every case is a project of one composition 32 by 18, 49 frames at 24 a second, in
`Fixtures/wave_world/`: the map layers, switched off, and on top of them the layer `holder`,
showing the drawing `shot`, with the effect. The expected pixels are in
`Fixtures/wave_world/expected_wave_world.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/wave_world_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import compound_blur_reference as K  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "wave_world"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES, FPS = 32, 18, 49, 24
CLEAR = [0.0, 0.0, 0.0, 0.0]


def shot_drawing():
    """The holder's own picture, which the effect does not use: blue and orange halves, the
    bottom right 6 by 4 clear."""
    return [[(0, 0, 0, 0) if x >= 26 and y >= 14 else
             ((40, 80, 200, 255) if x < 16 else (230, 140, 30, 255))
             for x in range(W)] for y in range(H)]


def logo_drawing():
    """The ground: dark grey, a white disc of radius 4.5 about (21, 9), a grey bar below left."""
    def px(x, y):
        if (x + 0.5 - 21) ** 2 + (y + 0.5 - 9) ** 2 <= 4.5 ** 2:
            return (255, 255, 255, 255)
        if 4 <= x < 12 and 12 <= y < 15:
            return (150, 150, 150, 255)
        return (40, 40, 40, 255)
    return [[px(x, y) for x in range(W)] for y in range(H)]


DRAWINGS = {"shot": shot_drawing(), "logo": logo_drawing()}
MAPS = {"logo": "logo", "late": "logo"}


def decoded(name):
    rows = DRAWINGS[name]
    return L.pic(len(rows[0]), len(rows),
                 [[srgb_to_linear(r / 255) * a / 255, srgb_to_linear(g / 255) * a / 255,
                   srgb_to_linear(b / 255) * a / 255, a / 255] for row in rows for r, g, b, a in row])


def map_picture(name, frame):
    """A map layer's picture fitted to the layer, or None for none named or not there."""
    if name == "" or name not in MAPS:
        return None
    p = L.EMPTY if name == "late" and frame < 4 else decoded(MAPS[name])
    return L.fit(p, "stretch", W, H)


# --- the rule -------------------------------------------------------------------------------

def rnd(v):
    return math.floor(v + 0.5)


def lerp(a, b, f):
    if isinstance(a, list):
        return [x + (y - x) * f for x, y in zip(a, b)]
    return a + (b - a) * f


def grid_shape(k0, w=W, h=H):
    n = max(1, rnd(k0["grid_resolution"]))
    m = max(1, rnd(n * h / w))
    return n, m, 1 / n, h / (m * w)


def ground_luma(ground, n, m, dx, dy, w=W):
    """Step 3's g per cell, rows then columns."""
    if ground is None:
        return [0.0] * (n * m)
    return [K.luma(L.bilinear(ground, (i + 0.5) * dx * w, (j + 0.5) * dy * w))
            for j in range(m) for i in range(n)]


def steps_for(samples, dx, dy):
    cmax = max(k["wave_speed"] * math.sqrt(2 * k["height"]) for k in samples)
    return max(4, math.ceil(1.6 * cmax / (FPS * min(dx, dy))))


def weights(k, p, n, m, dx, dy, w=W, h=H):
    """Step 6's weight of producer p per cell."""
    pre = f"producer_{p}_"
    wd, ht = k[pre + "width"], k[pre + "height"]
    out = [0.0] * (n * m)
    if wd == 0 or ht == 0:
        return out
    pos = k[pre + "position"]
    cx, cy = pos[0] * w / 100 / w, pos[1] * h / 100 / w
    th = math.radians(k[pre + "angle"])
    co, si = math.cos(th), math.sin(th)
    for j in range(m):
        for i in range(n):
            ox, oy = (i + 0.5) * dx - cx, (j + 0.5) * dy - cy
            a, b = ox * co + oy * si, -ox * si + oy * co
            if k[pre + "type"] == "ring":
                rr = (2 * a / wd) ** 2 + (2 * b / ht) ** 2
                if rr < 1:
                    out[j * n + i] = (1 - rr) ** 2
            elif abs(a) <= ht / 2 and abs(2 * b) < wd:
                out[j * n + i] = (1 - (2 * b / wd) ** 2) ** 2
    return out


def line_ends(k, p, n, m, dx, dy, w=W, h=H):
    """The least distance of a cell centre with weight from a line producer's ends."""
    pre = f"producer_{p}_"
    if k[pre + "type"] != "line" or k[pre + "width"] == 0 or k[pre + "height"] == 0:
        return math.inf
    pos = k[pre + "position"]
    cx, cy = pos[0] * w / 100 / w, pos[1] * h / 100 / w
    th = math.radians(k[pre + "angle"])
    co, si = math.cos(th), math.sin(th)
    best = math.inf
    for j in range(m):
        for i in range(n):
            ox, oy = (i + 0.5) * dx - cx, (j + 0.5) * dy - cy
            a, b = ox * co + oy * si, -ox * si + oy * co
            if abs(2 * b) < k[pre + "width"]:
                best = min(best, abs(abs(a) - k[pre + "height"] / 2))
    return best


def drive(k, p, big_t):
    pre = f"producer_{p}_"
    ph = math.radians(k[pre + "phase"])
    return k[pre + "amplitude"] * (math.sin(big_t + ph) - math.sin(ph))


def depth(k, g):
    return [k["height"] - k["steepness"] * gi for gi in g]


def step(h, u, d, c2, dt, n, m, dx, dy, reflect):
    """Step 5's wave part, in place. `d` the depths and `c2` the speeds squared at the start."""
    wet = [di > 0 for di in d]

    def at(i, j, i0, j0, s):
        if 0 <= i < n and 0 <= j < m:
            return h[j * n + i] if wet[j * n + i] else h[j0 * n + i0]
        c = j0 * n + i0
        if reflect == "all" or c2[c] == 0:
            return h[c]
        return h[c] - s * u[c] / math.sqrt(c2[c])

    lap = [0.0] * (n * m)
    for j in range(m):
        for i in range(n):
            c = j * n + i
            if not wet[c]:
                continue
            lap[c] = ((at(i - 1, j, i, j, dx) + at(i + 1, j, i, j, dx) - 2 * h[c]) / (dx * dx)
                      + (at(i, j - 1, i, j, dy) + at(i, j + 1, i, j, dy) - 2 * h[c]) / (dy * dy))
    for c in range(n * m):
        if wet[c]:
            u[c] += dt * c2[c] * lap[c]
    return wet


def simulate(c, frame, in_frame=None, w=W, h=H):
    """Steps 1 to 5: the grid, the depths at the frame, and the water's height."""
    n0 = c["in_frame"] if in_frame is None else in_frame
    count = frame - n0
    samples = [settings(c, n0 + j) for j in range(count + 1)]
    k0 = samples[0]
    n, m, dx, dy = grid_shape(k0, w, h)
    g = ground_luma(map_picture(c["ground_layer"], n0), n, m, dx, dy, w)
    hh, uu = [0.0] * (n * m), [0.0] * (n * m)
    reflect = c["reflect_edges"]
    # The frequency's running sum at each sample, per producer.
    tsum = {p: [0.0] for p in (1, 2)}
    for p in (1, 2):
        f = f"producer_{p}_frequency"
        for j in range(count):
            tsum[p].append(tsum[p][-1] + 2 * math.pi / FPS
                           * (samples[j][f] + samples[j + 1][f]) / 2)

    def big_t(p, j, f):
        if j is None:  # before the in point, `f` the time in seconds
            return 2 * math.pi * k0[f"producer_{p}_frequency"] * f
        if f == 0:
            return tsum[p][j]
        fa, fb = samples[j][f"producer_{p}_frequency"], samples[j + 1][f"producer_{p}_frequency"]
        return tsum[p][j] + 2 * math.pi / FPS * (fa * f + (fb - fa) * f * f / 2)

    def one(ka, kb, ta, tb, dt):
        d = depth(ka, g)
        c2 = [2 * di * ka["wave_speed"] ** 2 for di in d]
        wet = step(hh, uu, d, c2, dt, n, m, dx, dy, reflect)
        damp = math.exp(-10 * ka["damping"] * dt)
        for i in range(n * m):
            if wet[i]:
                uu[i] *= damp
                hh[i] += dt * uu[i]
        for p in (1, 2):
            dp = drive(kb, p, tb[p]) - drive(ka, p, ta[p])
            if dp != 0:
                wp = weights(kb, p, n, m, dx, dy, w, h)
                for i in range(n * m):
                    if wet[i]:
                        hh[i] += wp[i] * dp
        if kb["steepness"] != ka["steepness"]:
            for i in range(n * m):
                if wet[i]:
                    hh[i] += kb["wave_strength"] * (kb["steepness"] - ka["steepness"]) * g[i]
        for i in range(n * m):
            if not wet[i]:
                hh[i] = uu[i] = 0.0

    # The pre-roll, on sample 0.
    pre_frames = math.ceil(k0["pre_roll"] * FPS)
    q0 = steps_for([k0], dx, dy)
    total = pre_frames * q0
    for s in range(total):
        sa, sb = (s - total) / (FPS * q0), (s + 1 - total) / (FPS * q0)
        ta = {p: big_t(p, None, sa) for p in (1, 2)}
        tb = {p: big_t(p, None, sb) for p in (1, 2)}
        one(k0, k0, ta, tb, 1 / (FPS * q0))
    # The frame intervals.
    for j in range(count):
        q = steps_for(samples[j:j + 2], dx, dy)
        for i in range(q):
            fa, fb = i / q, (i + 1) / q
            ka = {x: lerp(samples[j][x], samples[j + 1][x], fa) for x in NUMBERS}
            kb = {x: lerp(samples[j][x], samples[j + 1][x], fb) for x in NUMBERS}
            for kk in (ka, kb):
                kk.update({x: c[x] for x in WORDS})
            if fb == 1:
                kb = samples[j + 1]
            one(ka, kb, {p: big_t(p, j, fa) for p in (1, 2)},
                {p: big_t(p, j + 1, 0) if fb == 1 else big_t(p, j, fb) for p in (1, 2)},
                1 / (FPS * q))
    kn = samples[-1]
    return n, m, g, depth(kn, g), hh, kn


def bilerp(vals, n, m, gx, gy):
    gx, gy = min(max(gx, 0.0), n - 1), min(max(gy, 0.0), m - 1)
    i0, j0 = math.floor(gx), math.floor(gy)
    i1, j1 = min(i0 + 1, n - 1), min(j0 + 1, m - 1)
    fx, fy = gx - i0, gy - j0
    return ((vals[j0 * n + i0] * (1 - fx) + vals[j0 * n + i1] * fx) * (1 - fy)
            + (vals[j1 * n + i0] * (1 - fx) + vals[j1 * n + i1] * fx) * fy)


def pixel(k, d, hv):
    if d <= 0:
        if k["render_dry_areas"] == "transparent":
            return list(CLEAR)
        hv = 0.0
    v = min(max(k["brightness"] + k["contrast"] * hv, 0.0), 1.0)
    grey = v ** (1 / k["gamma"])
    if k["height"] > 0:
        a = min(max(1 - k["transparency"] * (1 - max(d, 0.0) / k["height"]), 0.0), 1.0)
    else:
        a = min(max(1 - k["transparency"], 0.0), 1.0)
    lin = srgb_to_linear(grey)
    return [lin * a, lin * a, lin * a, a]


def wave_world(c, frame):
    n, m, _, d, hh, k = simulate(c, frame)
    out = []
    for y in range(H):
        for x in range(W):
            gx, gy = (x + 0.5) * n / W - 0.5, (y + 0.5) * m / H - 0.5
            out.append(pixel(k, bilerp(d, n, m, gx, gy), bilerp(hh, n, m, gx, gy)))
    return out


# --- the cases ------------------------------------------------------------------------------

NUMBERS = ("brightness", "contrast", "gamma", "transparency", "grid_resolution", "wave_speed",
           "damping", "pre_roll", "steepness", "height", "wave_strength") + tuple(
    f"producer_{p}_{n}" for p in (1, 2) for n in ("position", "height", "width", "angle",
                                                  "amplitude", "frequency", "phase"))
WORDS = ("render_dry_areas", "grid_res_downsamples", "reflect_edges", "ground_layer",
         "producer_1_type", "producer_2_type")
DEFAULTS = {
    "brightness": 0.5, "contrast": 0.5, "gamma": 1, "render_dry_areas": "level",
    "transparency": 0, "grid_resolution": 60, "grid_res_downsamples": "off", "wave_speed": 0.25,
    "damping": 0.05, "reflect_edges": "none", "pre_roll": 0, "ground_layer": "",
    "steepness": 0.25, "height": 0.5, "wave_strength": 0,
    "producer_1_type": "ring", "producer_1_position": [50, 50], "producer_1_height": 0.1,
    "producer_1_width": 0.1, "producer_1_angle": 0, "producer_1_amplitude": 0.5,
    "producer_1_frequency": 1, "producer_1_phase": 0,
    "producer_2_type": "ring", "producer_2_position": [75, 25], "producer_2_height": 0.1,
    "producer_2_width": 0.1, "producer_2_angle": 0, "producer_2_amplitude": 0,
    "producer_2_frequency": 1, "producer_2_phase": 0,
}


def case(shift=(0, 0), in_frame=0, **kw):
    c = dict(DEFAULTS, shift=shift, in_frame=in_frame)
    for n, v in kw.items():
        assert n in DEFAULTS, n
        c[n] = v
    return c


def small(**kw):
    """A grid of 16 by 9 (cells 2 pixels square) and a ring a quarter of the layer wide."""
    return case(**dict(dict(grid_resolution=16, producer_1_width=0.25, producer_1_height=0.25),
                       **kw))


def settings(c, frame):
    k = {n: value_at(c[n], frame) for n in NUMBERS}
    k.update({w: c[w] for w in WORDS})
    return k


def render(c, frame):
    if frame < c["in_frame"]:
        return [list(CLEAR) for _ in range(W * H)]
    out = L.pic(W, H, wave_world(c, frame))
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = decoded("shot")
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


GROUND = dict(ground_layer="logo", producer_1_position=[20, 30])
ISLAND = dict(GROUND, steepness=0.6)

CASES = {
    "FX-WW-001": ("As added: frame 0 is flat water, grey 0.5 everywhere and opaque; then a small "
                  "ring of ripples spreads from the middle.", case(), (0, 12, 24)),
    "FX-WW-002": ("Grid 16 and a ring a quarter wide: the water rises in the middle, then rings "
                  "spread out.", small(), (3, 12, 24)),
    "FX-WW-003": ("Phase 180: the same ripples upside down, dipping first.",
                  small(producer_1_phase=180), (3, 12)),
    "FX-WW-004": ("Frequency 2.5: closer ripples.", small(producer_1_frequency=2.5), (24,)),
    "FX-WW-005": ("A turned ring, 0.4 by 0.15 at 30 degrees: an oval source.",
                  small(producer_1_width=0.4, producer_1_height=0.15, producer_1_angle=30), (18,)),
    "FX-WW-006": ("A line 0.55 long and 0.2 wide at 20 degrees: waves leave both sides of it.",
                  small(producer_1_type="line", producer_1_height=0.55, producer_1_width=0.2,
                        producer_1_angle=20), (6, 18)),
    "FX-WW-007": ("Producer 2 switched on (amplitude 0.6, phase 90): two sources whose waves "
                  "cross.", small(producer_2_amplitude=0.6, producer_2_phase=90,
                                  producer_2_width=0.2, producer_2_height=0.2), (20,)),
    "FX-WW-008": ("Amplitude keyed 0 at frame 0 to 1 at frame 12: the source grows.",
                  small(producer_1_amplitude=keyed((0, 0), (12, 1))), (6, 12, 18)),
    "FX-WW-009": ("Frequency keyed 0.5 at frame 0 to 3 at frame 18: the source speeds up "
                  "smoothly, its phase the running sum of the frequency.",
                  small(producer_1_frequency=keyed((0, 0.5), (18, 3))), (9, 18, 24)),
    "FX-WW-010": ("Damping 0.5: the rings fade as they spread.", small(damping=0.5), (24,)),
    "FX-WW-011": ("Wave speed 0.6: the rings spread faster (more steps a frame).",
                  small(wave_speed=0.6), (12,)),
    "FX-WW-012": ("A source near the right edge, edges absorbing (none): the waves leave.",
                  small(producer_1_position=[85, 50], producer_1_frequency=2), (40,)),
    "FX-WW-013": ("FX-WW-012 with reflect edges all: the waves come back off the edges.",
                  small(producer_1_position=[85, 50], producer_1_frequency=2,
                        reflect_edges="all"), (40,)),
    "FX-WW-014": ("Frame 40 of FX-WW-002, drawn alone: the same whether or not the frames "
                  "before it were drawn.", small(), (40,)),
    "FX-WW-015": ("Pre-roll 0.5 seconds: frame 0 already has ripples.", small(pre_roll=0.5),
                  (0, 6)),
    "FX-WW-016": ("Pre-roll 0.5 with amplitude keyed 0.8 at frame 0 to 0 at frame 6: the "
                  "pre-roll runs on frame 0's amplitude, then the source stops.",
                  small(pre_roll=0.5, producer_1_amplitude=keyed((0, 0.8), (6, 0))), (0, 12)),
    "FX-WW-017": ("The holder's in point at frame 3: flat there, and frame 15 is FX-WW-002's "
                  "frame 12 (time counts from the in point).", small(in_frame=3), (3, 15)),
    "FX-WW-018": ("The logo as ground, steepness 0.6: the white disc is dry land, drawn level; "
                  "the waves go round it.", small(**ISLAND), (30,)),
    "FX-WW-019": ("FX-WW-018 drawing dry areas transparent: the disc is clear.",
                  small(render_dry_areas="transparent", **ISLAND), (30,)),
    "FX-WW-020": ("The logo as ground, steepness 0.3: all water, slower over the shallow disc.",
                  small(steepness=0.3, **GROUND), (30,)),
    "FX-WW-021": ("FX-WW-020 with transparency 0.8: clearer where shallow.",
                  small(steepness=0.3, transparency=0.8, **GROUND), (30,)),
    "FX-WW-022": ("Steepness keyed 0 to 0.3 over frames 0 to 12, wave strength 1, no source: "
                  "the rising ground alone makes waves.",
                  small(ground_layer="logo", producer_1_amplitude=0, wave_strength=1,
                        steepness=keyed((0, 0), (12, 0.3))), (6, 18)),
    "FX-WW-023": ("FX-WW-022 with wave strength 0: no waves, flat grey.",
                  small(ground_layer="logo", producer_1_amplitude=0, wave_strength=0,
                        steepness=keyed((0, 0), (12, 0.3))), (6, 18)),
    "FX-WW-024": ("A ground layer whose in point is frame 4, `late`: read at the holder's in "
                  "point (frame 0) it is empty, so the ground is flat, as FX-WW-002.",
                  small(ground_layer="late", steepness=0.6), (12, 24)),
    "FX-WW-025": ("A ground layer not in the composition, `gone`: flat ground, as FX-WW-002, "
                  "and the warning.", small(ground_layer="gone", steepness=0.6), (0, 12)),
    "FX-WW-026": ("Brightness 0.3, contrast 2, gamma 2.2.",
                  small(brightness=0.3, contrast=2, gamma=2.2), (12,)),
    "FX-WW-027": ("Contrast 8: the peaks and troughs clipped to white and black.",
                  small(contrast=8), (12,)),
    "FX-WW-028": ("Gamma 0.5: darker greys.", small(gamma=0.5), (12,)),
    "FX-WW-029": ("Height 0: every cell dry, drawn level, the brightness everywhere.",
                  small(height=0), (12,)),
    "FX-WW-030": ("Height 0 drawing dry areas transparent: clear.",
                  small(height=0, render_dry_areas="transparent"), (12,)),
    "FX-WW-031": ("FX-WW-002 on the holder moved 3 right and 1 down: the same, moved.",
                  small(shift=(3, 1)), (12,)),
    "FX-WW-032": ("Grid 7 (7 by 4 cells): coarse, smooth water.", small(grid_resolution=7),
                  (12,)),
    "FX-WW-033": ("Grid 1: one cell, the whole layer rising and falling as one.",
                  small(grid_resolution=1, producer_1_width=3, producer_1_height=3), (3, 6)),
    "FX-WW-034": ("Brightness keyed 0.2 to 0.8 over frames 0 to 12: the map follows the frame "
                  "drawn, not the history.", small(brightness=keyed((0, 0.2), (12, 0.8))),
                  (0, 12)),
}

INVALID = {
    "FX-WW-035": ("Grid resolution 0, below 1.", small(grid_resolution=0)),
    "FX-WW-036": ("Grid resolution 601, above 600.", small(grid_resolution=601)),
    "FX-WW-037": ("Wave speed -0.1, below 0.", small(wave_speed=-0.1)),
    "FX-WW-038": ("Damping 1.5, above 1.", small(damping=1.5)),
    "FX-WW-039": ("Pre-roll -1, below 0.", small(pre_roll=-1)),
    "FX-WW-040": ("Gamma 0, below 0.1.", small(gamma=0)),
    "FX-WW-041": ("Transparency 2, above 1.", small(transparency=2)),
    "FX-WW-042": ("Reflect edges \"some\".", small(reflect_edges="some")),
    "FX-WW-043": ("Render dry areas as \"hidden\".", small(render_dry_areas="hidden")),
    "FX-WW-044": ("Grid res downsamples \"maybe\".", small(grid_res_downsamples="maybe")),
    "FX-WW-045": ("Producer 1 type \"dot\".", small(producer_1_type="dot")),
    "FX-WW-046": ("A ground layer written as the number 3, not a word.", small(ground_layer=3)),
    "FX-WW-047": ("Steepness keyed to 1.5 at frame 4.",
                  small(steepness=keyed((0, 0), (4, 1.5)))),
    "FX-WW-048": ("Producer 2 position with one number.", small(producer_2_position=[10])),
}


def effect(fid, c):
    params = {w: c[w] for w in WORDS}
    params.update({n: setting_json(c[n]) for n in NUMBERS})
    return {"instance_id": fid, "type_id": "core.wave_world", "enabled": True,
            "parameters": params}


def map_layers():
    return [L.raster("logo", "asset-logo", enabled=False, out_frame=FRAMES),
            L.raster("late", "asset-logo", enabled=False, in_frame=4, out_frame=FRAMES)]


def project_json(fx, c):
    holder = L.raster("holder", "asset-shot", position=c["shift"], in_frame=c["in_frame"],
                      out_frame=FRAMES)
    holder["effects"] = [effect("fx-1", c)]
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(n, f"media/{n}.png") for n in DRAWINGS],
            "compositions": [L.composition("comp-main", W, H, FRAMES, map_layers() + [holder])]}


def write(name, p):
    (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        step_clear(fx, c, frames)
        rendered = {str(f): render(c, f) for f in frames}
        entry = {"says": says, "project": write(f"{fx.lower().replace('-', '_')}.json",
                                                project_json(fx, c)), "frames": rendered}
        if c["ground_layer"] == "gone":
            entry["warning"] = "EFFECT_LAYER_MISSING"
        expected["cases"][fx] = entry
        flat = rendered[str(frames[0])][0]
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(1 for p in px if max(abs(a - b) for a, b in zip(p, flat)) > 1e-4)}"
            " off the first pixel" for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(
            f"{fx.lower().replace('-', '_')}.json", project_json(fx, c)),
            "frames": {"0": before, "4": before}, "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_wave_world.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                  encoding="utf-8")
    check(expected)


def step_clear(fx, c, frames):
    """No cell is near dry at any step's settings, no pixel near dry in the map, and no cell
    centre with weight near a line producer's end."""
    for f in frames:
        if f < c["in_frame"]:
            continue
        n0 = c["in_frame"]
        k0 = settings(c, n0)
        n, m, dx, dy = grid_shape(k0)
        g = ground_luma(map_picture(c["ground_layer"], n0), n, m, dx, dy)
        for j in range(f - n0 + 1):
            for fr in (0, 0.25, 0.5, 0.75):
                ka, kb = settings(c, n0 + j), settings(c, n0 + j + 1)
                k = {x: lerp(ka[x], kb[x], fr) for x in NUMBERS}
                k.update({x: c[x] for x in WORDS})
                if k["height"] > 0:
                    assert all(abs(di) > 1e-6 for di in depth(k, g)), (fx, f, j)
                for p in (1, 2):
                    assert line_ends(k, p, n, m, dx, dy) > 1e-6, (fx, f, j, p)
        kn = settings(c, f)
        d = depth(kn, g)
        if kn["height"] > 0:
            for y in range(H):
                for x in range(W):
                    dd = bilerp(d, n, m, (x + 0.5) * n / W - 0.5, (y + 0.5) * m / H - 0.5)
                    assert abs(dd) > 1e-6, (fx, f, x, y)


def near(a, b, eps=1e-12):
    return all(abs(x - y) <= eps for p, q in zip(a, b) for x, y in zip(p, q))


def to_srgb(v):
    return 12.92 * v if v <= 0.0031308 else 1.055 * v ** (1 / 2.4) - 0.055


def hand_step():
    """One step worked by hand on a 6 by 5 grid (a layer 30 by 25): a bump of 1 in cell (2, 2)
    and a dry cell at (3, 2) beside it, still water elsewhere."""
    n, m, dx, dy = grid_shape({"grid_resolution": 6}, 30, 25)
    assert (n, m) == (6, 5) and dx == 1 / 6 and abs(dy - 25 / (5 * 30)) < 1e-15
    h, u = [0.0] * 30, [0.0] * 30
    h[2 * 6 + 2] = 1.0
    d = [0.5] * 30
    d[2 * 6 + 3] = -0.1
    c2 = [2 * di * 0.25 ** 2 for di in d]
    dt = 0.01
    step(h, u, d, c2, dt, n, m, dx, dy, "none")
    # The bump: its left, up and down neighbours are 0; its right neighbour is dry, so mirrors 1.
    assert abs(u[14] - dt * 0.0625 * ((0 + 1 - 2) / dx ** 2 + (0 + 0 - 2) / dy ** 2)) < 1e-12
    # Its left neighbour feels the bump once across.
    assert abs(u[13] - dt * 0.0625 * (1 / dx ** 2)) < 1e-12
    # The dry cell does not move; a far cell does not move.
    assert u[15] == 0.0 and u[0] == 0.0
    # An edge cell with speed: the absorbing edge feeds back its own speed.
    h2, u2 = [0.0] * 30, [0.0] * 30
    u2[0] = 0.2
    step(h2, u2, [0.5] * 30, [0.0625] * 30, dt, n, m, dx, dy, "none")
    want = 0.2 + dt * 0.0625 * ((-dx * 0.2 / 0.25) / dx ** 2 + (-dy * 0.2 / 0.25) / dy ** 2)
    assert abs(u2[0] - want) < 1e-12
    h3, u3 = [0.0] * 30, [0.0] * 30
    u3[0] = 0.2
    step(h3, u3, [0.5] * 30, [0.0625] * 30, dt, n, m, dx, dy, "all")
    assert u3[0] == 0.2
    # The step counts: as added (grid 60) 4 a frame; wave speed 1, height 1, grid 600 -> 57.
    assert steps_for([dict(wave_speed=0.25, height=0.5)], 1 / 60, 1 / 60) == 4
    assert steps_for([dict(wave_speed=1, height=1)], 1 / 600, 1 / 600) == 57
    # The drive: phase 0 rises first, phase 180 dips first, both from 0.
    k = dict(producer_1_amplitude=0.5, producer_1_phase=0)
    assert drive(k, 1, 0) == 0 and drive(k, 1, 0.1) > 0
    k["producer_1_phase"] = 180
    assert abs(drive(k, 1, 0)) < 1e-15 and drive(k, 1, 0.1) < 0


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    hand_step()
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    grey = srgb_to_linear(0.5)
    flat = [[grey, grey, grey, 1.0]] * (W * H)
    # As added, frame 0: flat grey, opaque.
    assert near(c["FX-WW-001"]["0"], flat)
    assert c["FX-WW-001"]["12"] != flat and c["FX-WW-001"]["24"] != flat
    # Rises first in the middle; phase 180 the mirror image about grey 0.5 (no clipping).
    mid = (H // 2) * W + W // 2
    assert c["FX-WW-002"]["3"][mid][0] > grey and c["FX-WW-003"]["3"][mid][0] < grey
    for f in ("3", "12"):
        for p, q in zip(c["FX-WW-002"][f], c["FX-WW-003"][f]):
            assert abs(to_srgb(p[0]) + to_srgb(q[0]) - 1) < 1e-9
    # Keyed amplitude from 0: frame 0 would be flat; it differs from the plain source.
    assert c["FX-WW-008"]["12"] != c["FX-WW-002"]["12"]
    # The reflecting edges differ from the absorbing ones.
    assert c["FX-WW-012"]["40"] != c["FX-WW-013"]["40"]
    # Pre-roll: frame 0 is not flat; the keyed amplitude's pre-roll matches 0.8 at frame 0.
    assert c["FX-WW-015"]["0"] != flat and c["FX-WW-016"]["0"] != flat
    # The in point: flat at it, and 12 frames later equal to FX-WW-002's frame 12.
    assert near(c["FX-WW-017"]["3"], flat) and near(c["FX-WW-017"]["15"], c["FX-WW-002"]["12"])
    # The island drawn level is the brightness inside the disc; transparent, clear there.
    disc = 9 * W + 21
    assert near([c["FX-WW-018"]["30"][disc]], [[grey, grey, grey, 1.0]])
    assert c["FX-WW-019"]["30"][disc] == CLEAR
    # Transparency: clearer over the shallow disc than over the deep corner.
    assert c["FX-WW-021"]["30"][disc][3] < c["FX-WW-021"]["30"][0][3] < 1
    # The ground's rise makes waves only with wave strength.
    assert c["FX-WW-022"]["18"] != flat and near(c["FX-WW-023"]["18"], flat)
    # The late and the missing ground are flat ground.
    assert near(c["FX-WW-024"]["12"], c["FX-WW-002"]["12"])
    assert near(c["FX-WW-024"]["24"], c["FX-WW-002"]["24"])
    assert near(c["FX-WW-025"]["12"], c["FX-WW-002"]["12"])
    # Height 0: the brightness everywhere, or clear.
    assert near(c["FX-WW-029"]["12"], flat)
    assert all(p == CLEAR for p in c["FX-WW-030"]["12"])
    # Moved: the same picture, three right and one down; clear where the layer is not.
    moved, base = c["FX-WW-031"]["12"], c["FX-WW-002"]["12"]
    assert all(moved[(y + 1) * W + x + 3] == base[y * W + x]
               for y in range(H - 1) for x in range(W - 3))
    assert moved[0] == CLEAR
    # Grid 1: every pixel the same.
    for f in ("3", "6"):
        assert all(p == c["FX-WW-033"][f][0] for p in c["FX-WW-033"][f])
    assert c["FX-WW-033"]["3"][0] != c["FX-WW-033"]["6"][0]
    # Brightness keyed: frame 0 is flat at 0.2.
    g2 = srgb_to_linear(0.2)
    assert near(c["FX-WW-034"]["0"], [[g2, g2, g2, 1.0]] * (W * H))
    # Contrast 8 clips somewhere to white and to black.
    vals = [p[0] for p in c["FX-WW-027"]["12"]]
    assert max(vals) == 1.0 and min(vals) == 0.0
    print("checks passed")


if __name__ == "__main__":
    main()
