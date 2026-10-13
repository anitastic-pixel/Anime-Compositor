"""Particle Grid, worked a second way.

`core.particle_grid` (D-616), the particle engine's fifth effect (P0-6, D-600), modelled on what
Trapcode Form is for (`docs/effects/PLUGINS.md` 2.2, the parked pick, built after Particle
Emitter as that section asks): particles that do not fly away. They sit in a grid of points or on
layered spheres in 3D, are scattered, twisted and rippled by a noise field that can drift, take
their colour, size and depth from other layers, and are drawn as soft spheres, glowing spheres,
stars or cloudlets through the composition's camera. Red Giant publishes no formula, so every rule
and every number when added below is ours (document 21, D-616). No text or code was copied.
Audio reaction, the spherical field, the kaleidoscope, strings, text and model forms, lights and
shadows are not built in this step (D-616 says why).

**No state is carried from frame to frame** (document 20): every particle's place is worked out
from the settings at the frame drawn, except the drift, which is the sum of the Flow settings over
the frames from the layer's in point (each frame worked from the settings alone, never from the
frame before). The camera, the sprites and the transfer are Particle Emitter's
(`tools/particle_emitter_reference.py`, D-615); the noise is D-610's
(`tools/particle_space_reference.py`).

The settings are SETTINGS (name, lowest, highest, value when added; all keyable), POINTS (two
numbers, per cent of the layer's width and height, each held to -1000 to 1000; keyable), WORDS
(exact), LAYERS (D-189's layer word, "" for none; each map fitted to the layer by `stretch`) and
COLOURS (`#rrggbb`). Lengths are in the layer's pixels, speeds in pixels a second, angles in
degrees. With W and H the layer's size, n the frame drawn, i0 the layer's in point, u = n - i0 and
fps the composition's rate, every setting read at n and held inside its range:

- The points. N = n_x n_y n_z, n_a = floor(Particles In a). Point j (0 up to N - 1) is i = j mod
  n_x, l = floor(j / n_x) mod n_y, m = floor(j / (n_x n_y)); S = (Form Size X, Y, Z). Its base
  place d0, in pixels round the form's middle: Box Grid
  d0 = (S_x ((i + 1/2) / n_x - 1/2), S_y ((l + 1/2) / n_y - 1/2), S_z ((m + 1/2) / n_z - 1/2));
  Sphere Layered, shell m of radius g = (m + 1) / n_z, ring l at theta = pi (l + 1/2) / n_y from
  the top and turn phi = 2 pi i / n_x,
  d0 = g (S_x / 2 sin theta cos phi, -S_y / 2 cos theta, S_z / 2 sin theta sin phi).
- Randomness r(j, s) = D-610's, (U(seed, j, s, 0, 0) + 1) / 2 with seed = floor(Random Seed).
  Streams: 0 to 2 the scatter, 3 the size, 4 the opacity.
- The maps, read at the base place's spot on the form's front: a = d0_x / S_x + 1/2 (1/2 when
  S_x is 0), b = d0_y / S_y + 1/2 (likewise), the map's bilinear sample (document 21's, pixel
  centres at halves, transparent outside) at (a W, b H). L(p) = compound blur's luminance of the
  premultiplied sample, sRGB-coded. With no map named: L = 1 and the sample opaque white.
- The noise: q = (d0 + F) / Fractal Scale + E (0.31, 0.57, 0.77), F = (Flow X, Y, Z) summed and
  E = Evolution + Flow Evolution summed, a sum being the area under the setting's frame-by-frame
  straight lines from i0 to n, in seconds (sum over k from 0 to u - 1 of (v(i0 + k) + v(i0 + k +
  1)) / (2 fps); 0 when u is 0). N_c = D-610's noise(seed, c, q), c = 0 to 4.
- The place: d = d0 + Disperse (2 r(j, 0) - 1, 2 r(j, 1) - 1, 2 r(j, 2) - 1); then d_z minus Map
  Displace L(Displace Map) when one is named (bright comes towards the camera); then turned by
  Ry(Twist t), t = d0_y / S_y (0 when S_y is 0), the top row turned by -Twist / 2 and the bottom
  by Twist / 2; then plus Displace (N_0, N_1, N_2). In the engine's units (W, origin the layer's
  centre, Y down, Z away): x = C + M d / W, C = (Form Center_x / 100 - 1/2, (Form Center_y / 100
  - 1/2) H / W, Form Center Z / W), M = Rx(Form Rotation X) Ry(Form Rotation Y) Rz(Form Rotation
  Z), D-610's.
- Size in pixels: Size (1 + Size Random / 100 (2 r(j, 3) - 1)) max(0, 1 + Affect Size / 100 N_3)
  L(Size Map). Opacity: Opacity / 100 (1 - Opacity Random / 100 r(j, 4)) clamp(1 + Affect Opacity
  / 100 N_4) alpha(Color Map). Colour, linear light: the Color, or with a Color Map its sample
  unpremultiplied (not drawn where the map is clear).
- Drawing: Particle Emitter's camera (the composition's, else straight on from W 50 / 36 in front
  of the layer's centre), its sprites for Sphere, Glow Sphere, Star and Cloudlet with the Sphere
  Feather (its 0.7 smallest radius too), farthest first, equal depths by j, over a clear layer,
  Normal, Add or Screen.

**This file never runs the build's code path.** It works in double precision on lists from the
drawing's 8-bit values; trigonometry is the platform's. The drawing is Broadcast Colors' 16 by
10; the map layers are three 16 by 10 drawings, switched off, below the layer `art`. The projects
go into `Fixtures/particle_grid`, the expected frames into
`Fixtures/particle_grid/expected_particle_grid.json`, each particle's numbers for some cases into
`Fixtures/particle_grid/particles_particle_grid.json`. Every case is "not built yet" until B-496
lands (D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/particle_grid_reference.py
"""

import json
import math
import sys
from pathlib import Path
from types import SimpleNamespace

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from noise_reference import u as U  # noqa: E402
from motion_blur_reference import png  # noqa: E402
from adjust_reference import prop  # noqa: E402
import particle_space_reference as P  # noqa: E402
import particle_plane_reference as PL  # noqa: E402
import particle_emitter_reference as PE  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import recolor_reference as R  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import compound_blur_reference as K  # noqa: E402

W, H = B.W, B.H
FPS = P.FPS
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "particle_grid"

SETTINGS = [
    ("form_size_x", 0, 100000, 820), ("form_size_y", 0, 100000, 620),
    ("form_size_z", 0, 100000, 0),
    ("particles_in_x", 1, 1000, 41), ("particles_in_y", 1, 1000, 31),
    ("particles_in_z", 1, 100, 1),
    ("form_center_z", -100000, 100000, 0),
    ("form_rotation_x", -36000, 36000, 0), ("form_rotation_y", -36000, 36000, 0),
    ("form_rotation_z", -36000, 36000, 0),
    ("size", 0, 10000, 4), ("size_random", 0, 100, 0),
    ("opacity", 0, 100, 100), ("opacity_random", 0, 100, 0),
    ("sphere_feather", 0, 100, 50),
    ("disperse", 0, 100000, 0), ("twist", -36000, 36000, 0),
    ("displace", 0, 100000, 0), ("fractal_scale", 1, 100000, 100),
    ("flow_x", -100000, 100000, 0), ("flow_y", -100000, 100000, 0),
    ("flow_z", -100000, 100000, 0), ("flow_evolution", -1000, 1000, 0),
    ("evolution", -100000, 100000, 0),
    ("affect_size", 0, 1000, 0), ("affect_opacity", 0, 100, 0),
    ("map_displace", -100000, 100000, 100),
    ("random_seed", 0, 10000, 0),
]
NAMES = [s[0] for s in SETTINGS]
RANGES = {s[0]: (s[1], s[2]) for s in SETTINGS}
POINTS = [("form_center", [50, 50])]
WORDS = [
    ("base_form", ("box_grid", "sphere_layered"), "box_grid"),
    ("particle_type", ("sphere", "glow_sphere", "star", "cloudlet"), "sphere"),
    ("transfer_mode", ("normal", "add", "screen"), "normal"),
]
LAYERS = ["color_map", "size_map", "displace_map"]
COLOURS = [("color", "#ffffff")]
FLOWS = ["flow_x", "flow_y", "flow_z", "flow_evolution"]


# --- the map layers -------------------------------------------------------------------------

def ramp_drawing():
    """Red rising left to right, green top to bottom, blue half."""
    return [[(round(255 * x / 15), round(255 * y / 9), 128, 255) for x in range(W)]
            for y in range(H)]


def spot_drawing():
    """A grey spot, white in the middle fading to black 6 pixels out."""
    def grey(x, y):
        g = round(255 * max(0.0, 1 - math.hypot(x + 0.5 - 8, y + 0.5 - 5) / 6))
        return (g, g, g, 255)
    return [[grey(x, y) for x in range(W)] for y in range(H)]


def half_drawing():
    """The left half a solid blue, the right half clear."""
    return [[(51, 102, 255, 255) if x < 8 else (0, 0, 0, 0) for x in range(W)]
            for y in range(H)]


DRAWINGS = {"ramp": ramp_drawing(), "spot": spot_drawing(), "half": half_drawing()}


def map_of(name):
    if name == "":
        return None
    rows = DRAWINGS[name]
    p = L.pic(W, H, [R.working(px) for row in rows for px in row])
    return L.fit(p, "stretch", W, H)


# --- the points -----------------------------------------------------------------------------

def sample(c, frame_no):
    s = {}
    for n in NAMES:
        lo, hi = RANGES[n]
        s[n] = P.clamp(value_at(c[n], frame_no), lo, hi)
    for n, _ in POINTS:
        x, y = value_at(c[n], frame_no)
        s[n + "_x"], s[n + "_y"] = P.clamp(x, -1000, 1000), P.clamp(y, -1000, 1000)
    return s


def flowed(c, n):
    """Each Flow setting summed from the in point to n, in seconds."""
    i0 = c["in_frame"]
    vs = [sample(c, i0 + k) for k in range(n - i0 + 1)]
    return {f: sum((a[f] + b[f]) / 2 for a, b in zip(vs, vs[1:])) / FPS for f in FLOWS}


def counts(s):
    return [math.floor(s["particles_in_" + a]) for a in "xyz"]


def base(c, s, j):
    nx, ny, nz = counts(s)
    i, l, m = j % nx, (j // nx) % ny, j // (nx * ny)
    size = [s["form_size_" + a] for a in "xyz"]
    if c["base_form"] == "box_grid":
        return [size[0] * ((i + 0.5) / nx - 0.5), size[1] * ((l + 0.5) / ny - 0.5),
                size[2] * ((m + 0.5) / nz - 0.5)]
    g = (m + 1) / nz
    th, ph = math.pi * (l + 0.5) / ny, 2 * math.pi * i / nx
    return [g * size[0] / 2 * math.sin(th) * math.cos(ph), -g * size[1] / 2 * math.cos(th),
            g * size[2] / 2 * math.sin(th) * math.sin(ph)]


def on_front(s, d0, a):
    k = "xy".index(a)
    sz = s["form_size_" + a]
    return d0[k] / sz + 0.5 if sz > 0 else 0.5


def points(c, n):
    """(j, x, size in pixels, opacity, linear colour or None when not drawn) for every point."""
    s = sample(c, n)
    fl = flowed(c, n)
    seed = math.floor(s["random_seed"])
    r = lambda j, st: (U(seed, j, st, 0, 0) + 1) / 2  # noqa: E731
    maps = {m: map_of(c[m]) for m in LAYERS}
    m = P.mat(P.rx(math.radians(s["form_rotation_x"])),
              P.mat(P.ry(math.radians(s["form_rotation_y"])),
                    P.rz(math.radians(s["form_rotation_z"]))))
    centre = [s["form_center_x"] / 100 - 0.5, (s["form_center_y"] / 100 - 0.5) * H / W,
              s["form_center_z"] / W]
    off = [fl["flow_x"], fl["flow_y"], fl["flow_z"]]
    evo = s["evolution"] + fl["flow_evolution"]
    sc = s["fractal_scale"]
    nx, ny, nz = counts(s)
    out = []
    for j in range(nx * ny * nz):
        d0 = base(c, s, j)
        a, b = on_front(s, d0, "x"), on_front(s, d0, "y")
        got = {k: (L.bilinear(v, a * W, b * H) if v is not None else None)
               for k, v in maps.items()}
        q = [(d0[i] + off[i]) / sc + evo * w for i, w in enumerate((0.31, 0.57, 0.77))]
        nn = [P.noise(seed, ch, q) for ch in range(5)]
        d = [d0[i] + s["disperse"] * (2 * r(j, i) - 1) for i in range(3)]
        if got["displace_map"] is not None:
            d[2] -= s["map_displace"] * K.luma(got["displace_map"])
        t = d0[1] / s["form_size_y"] if s["form_size_y"] > 0 else 0.0
        d = P.apply(P.ry(math.radians(s["twist"] * t)), d)
        d = [d[i] + s["displace"] * nn[i] for i in range(3)]
        md = P.apply(m, d)
        x = [centre[i] + md[i] / W for i in range(3)]
        size = (s["size"] * (1 + s["size_random"] / 100 * (2 * r(j, 3) - 1))
                * max(0.0, 1 + s["affect_size"] / 100 * nn[3]))
        if got["size_map"] is not None:
            size *= K.luma(got["size_map"])
        alpha = (s["opacity"] / 100 * (1 - s["opacity_random"] / 100 * r(j, 4))
                 * P.clamp(1 + s["affect_opacity"] / 100 * nn[4]))
        cm = got["color_map"]
        if cm is None:
            col = P.linear(c["color"])
        elif cm[3] > 0:
            col = [cm[i] / cm[3] for i in range(3)]
            alpha *= cm[3]
        else:
            col = None
        out.append((j, x, size, alpha, col))
    return out


# --- the drawing ----------------------------------------------------------------------------

def sprites(c, n):
    s = sample(c, n)
    see = P.camera(SimpleNamespace(now=None), PE.view_of(c))
    out = []
    for j, x, size, alpha, col in points(c, n):
        v = see(x)
        if v is None or col is None:
            continue
        X, Y, ppu, depth = v
        rad = size * ppu / (2 * W)
        if rad < 0.7:
            alpha *= (rad / 0.7) ** 2
            rad = 0.7
        out.append((depth, j, {"kind": c["particle_type"], "X": X, "Y": Y, "col": col,
                               "R": rad, "alpha": alpha, "feather": s["sphere_feather"] / 100}))
    out.sort(key=lambda e: (-e[0], e[1]))
    return out


def particle_grid(px, c, n):
    if n < c["in_frame"]:
        return [list(p) for p in px]
    out = [[0.0] * 4 for _ in px]
    mode = PE.TRANSFER[c["transfer_mode"]]
    for _, _, sp in sprites(c, n):
        xs, ys = PE.reach(sp)
        for gy in range(max(0, math.floor(ys[0])), min(H, math.ceil(ys[1]) + 1)):
            for gx in range(max(0, math.floor(xs[0])), min(W, math.ceil(xs[1]) + 1)):
                cv = PE.cover(sp, gx + 0.5, gy + 0.5)
                if cv is None:
                    continue
                d = out[gy * W + gx]
                if mode == "composite":
                    d = [cv[i] + (1 - cv[3]) * d[i] for i in range(4)]
                elif mode == "screen":
                    d = [cv[i] + d[i] - cv[i] * d[i] for i in range(4)]
                else:
                    d = [cv[0] + d[0], cv[1] + d[1], cv[2] + d[2], min(1.0, cv[3] + d[3])]
                out[gy * W + gx] = d
    return out


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, camera=None, in_frame=0, **given):
    c = {s[0]: s[3] for s in SETTINGS}
    c.update({n: list(v) for n, v in POINTS})
    c.update({w[0]: w[2] for w in WORDS})
    c.update({n: "" for n in LAYERS})
    c.update({k: v for k, v in COLOURS})
    for k, v in given.items():
        assert k in c, k
        c[k] = v
    c.update(shift=shift, camera=camera, in_frame=in_frame)
    return c


def in_range(c):
    for n, lo, hi, _ in SETTINGS:
        v = c[n]
        for x in ([k[1] for k in v["keys"]] if isinstance(v, dict) else [v]):
            if not lo <= x <= hi:
                return False
    return True


def render(c, frame_no):
    out = particle_grid(P.working(), c, frame_no)
    move = c["shift"]
    if c["camera"] is not None:
        assert c["camera"][1] == H / 2 and c["camera"][0] == int(c["camera"][0])
        move += int(W / 2 - c["camera"][0])
    return R.frame(out, move)


def plain(c):
    return PE.plain(c)


def file_json(fx, c):
    params = {n: setting_json(c[n]) for n in NAMES}
    params.update({n: setting_json(c[n]) for n, _ in POINTS})
    params.update({w[0]: c[w[0]] for w in WORDS})
    params.update({n: c[n] for n in LAYERS})
    params.update({k: c[k] for k, _ in COLOURS})
    p = B.project_json(fx, "core.particle_grid", params, c["shift"])
    comp = p["compositions"][0]
    frames = max([f for f in FRAMES_OF.get(fx, [0])] + [4]) + 2
    comp["duration_frames"] = frames
    comp["work_area"]["end_frame_exclusive"] = frames
    art = comp["layers"][0]
    art["in_frame"] = c["in_frame"]
    art["out_frame"] = frames
    maps = [L.raster(name, "asset-" + name, enabled=False, in_frame=0, out_frame=frames)
            for name in DRAWINGS]
    comp["layers"] = maps + [art]
    comp["layer_order"] = [m["id"] for m in maps] + ["art"]
    p["assets"] += [L.still(name, f"media/{name}.png") for name in DRAWINGS]
    if c["camera"] is not None:
        x, y, depth, zoom = c["camera"]
        comp["camera"] = {"position": prop([x, y]), "depth": prop(depth), "zoom": prop(zoom)}
    return p


# A small flat grid: 4 by 2 points over 12 by 6 pixels, at x 3.5, 6.5, 9.5, 12.5 and y 3.5, 6.5.
SMALL = dict(form_size_x=12, form_size_y=6, particles_in_x=4, particles_in_y=2, size=2)
# The same, the dots big enough to overlap their neighbours.
BIG = dict(SMALL, size=4.5, color="#3366cc")
# A finer grid, 5 by 3 over 15 by 7.5 pixels (x 2, 5, 8, 11, 14; y 2.5, 5, 7.5), 1.5 pixels
# across, point 7 in the middle.
FINE = dict(form_size_x=15, form_size_y=7.5, particles_in_x=5, particles_in_y=3, size=1.5)
# A grid three deep, 20 pixels from front to back.
DEEP = dict(FINE, particles_in_z=3, form_size_z=20)
# A ball: 8 points round, 5 rings, 8 pixels across.
BALL = dict(base_form="sphere_layered", form_size_x=8, form_size_y=8, form_size_z=8,
            particles_in_x=8, particles_in_y=5, size=1.5)
# A rippling field on the fine grid.
RIPPLE = dict(FINE, displace=2, fractal_scale=6)

CASES = {
    "FX-PGRID-001": ("The settings as added, at frames 0 and 12: a grid 41 by 31 points, 20 "
                     "pixels apart, so only its middle point lands on this 16 by 10 layer, a "
                     "white soft dot 4 pixels across in the middle; still; the layer's own "
                     "picture is gone.", case(), [0, 12]),
    "FX-PGRID-002": ("A small grid: 4 by 2 white dots, 2 pixels across.", case(**SMALL), [0]),
    "FX-PGRID-003": ("A finer grid: 5 by 3 dots 1.5 pixels across, 3 pixels apart.",
                     case(**FINE), [0]),
    "FX-PGRID-004": ("Sphere Feather 0 on big dots: hard edges.",
                     case(**dict(BIG, sphere_feather=0)), [0]),
    "FX-PGRID-005": ("Glow Spheres on the big dots: solid cores and faint glows.",
                     case(**dict(BIG, particle_type="glow_sphere")), [0]),
    "FX-PGRID-006": ("Stars: 2 by 1 points, 6 pixels across, five points up.",
                     case(**dict(SMALL, particles_in_x=2, particles_in_y=1, size=6,
                                 particle_type="star")), [0]),
    "FX-PGRID-007": ("Cloudlets on the big dots: soft to the edge.",
                     case(**dict(BIG, particle_type="cloudlet")), [0]),
    "FX-PGRID-008": ("Size 1 on the small grid: drawn 1.4 pixels across, fainter by the true "
                     "size.", case(**dict(SMALL, size=1)), [0]),
    "FX-PGRID-009": ("Size Random 80 on the fine grid: each dot its own size.",
                     case(**dict(FINE, size=2, size_random=80)), [0]),
    "FX-PGRID-010": ("Opacity 60: every dot 60 per cent.", case(**dict(SMALL, opacity=60)), [0]),
    "FX-PGRID-011": ("Opacity Random 80 on the fine grid: each dot its own opacity.",
                     case(**dict(FINE, opacity_random=80)), [0]),
    "FX-PGRID-012": ("Color orange.", case(**dict(SMALL, color="#ff8000")), [0]),
    "FX-PGRID-013": ("Big overlapping blue dots, Normal: later points over earlier.",
                     case(**BIG), [0]),
    "FX-PGRID-014": ("The same, Add: brighter where they overlap.",
                     case(**dict(BIG, transfer_mode="add")), [0]),
    "FX-PGRID-015": ("The same, Screen.", case(**dict(BIG, transfer_mode="screen")), [0]),
    "FX-PGRID-016": ("The fine grid turned 30 degrees about Z.",
                     case(**dict(FINE, form_rotation_z=30)), [0]),
    "FX-PGRID-017": ("The fine grid turned 60 degrees about Y: narrower, the right side farther "
                     "and smaller.", case(**dict(FINE, form_rotation_y=60)), [0]),
    "FX-PGRID-018": ("The deep grid tipped 50 degrees about X: its three layers seen from "
                     "above.", case(**dict(DEEP, form_rotation_x=50)), [0]),
    "FX-PGRID-019": ("The deep grid straight on: the far layers smaller and drawn nearer the "
                     "middle, the front layer over them.", case(**DEEP), [0]),
    "FX-PGRID-020": ("The small grid moved up and left, and 10 pixels back.",
                     case(**dict(SMALL, form_center=[30, 40], form_center_z=10)), [0]),
    "FX-PGRID-021": ("Sphere Layered: one shell, 8 points round on 5 rings, 8 pixels across.",
                     case(**BALL), [0]),
    "FX-PGRID-022": ("Two shells: an inner ball half the size inside.",
                     case(**dict(BALL, particles_in_z=2)), [0]),
    "FX-PGRID-023": ("The ball turning: Form Rotation Y keyed from 0 at frame 0 to 80 at frame "
                     "24, at frames 0, 12 and 24.",
                     case(**dict(BALL, form_rotation_y=keyed((0, 0), (24, 80)),
                                 form_rotation_x=20)), [0, 12, 24]),
    "FX-PGRID-024": ("Disperse 1.5: each dot nudged its own way, the grid loosened.",
                     case(**dict(FINE, disperse=1.5)), [0]),
    "FX-PGRID-025": ("Disperse 1.5, Random Seed 7: nudged differently.",
                     case(**dict(FINE, disperse=1.5, random_seed=7)), [0]),
    "FX-PGRID-026": ("Twist 120: the top row turned one way about the up line, the bottom row "
                     "the other, so the rows narrow and widen.",
                     case(**dict(FINE, twist=120)), [0]),
    "FX-PGRID-027": ("Fractal displace 2 at scale 6: the grid rippled.", case(**RIPPLE), [0]),
    "FX-PGRID-028": ("The ripple drifting: Flow X 12 pixels a second, at frames 0, 12 and 24; "
                     "the field slides through the grid.",
                     case(**dict(RIPPLE, flow_x=12)), [0, 12, 24]),
    "FX-PGRID-029": ("The ripple changing: Flow Evolution 1 a second, at frames 0 and 12.",
                     case(**dict(RIPPLE, flow_evolution=1)), [0, 12]),
    "FX-PGRID-030": ("Evolution 3: a different ripple, still.",
                     case(**dict(RIPPLE, evolution=3)), [0]),
    "FX-PGRID-031": ("Affect Size 150 with no displacement: the field grows and shrinks the "
                     "dots in place.", case(**dict(FINE, size=2, fractal_scale=4,
                                                   affect_size=150)), [0]),
    "FX-PGRID-032": ("Affect Opacity 100: the field fades dots in place.",
                     case(**dict(FINE, size=2, fractal_scale=4, affect_opacity=100)), [0]),
    "FX-PGRID-033": ("Flow Y keyed from 0 at frame 0 to 24 at frame 24: the drift speeds up, "
                     "by frame 24 moved 12 pixels' worth, at frames 12 and 24.",
                     case(**dict(RIPPLE, flow_y=keyed((0, 0), (24, 24)))), [12, 24]),
    "FX-PGRID-034": ("Color Map the ramp: the dots red to the right, green lower down.",
                     case(**dict(FINE, size=2, color_map="ramp")), [0]),
    "FX-PGRID-035": ("Color Map half blue, half clear: only the left half's dots, blue.",
                     case(**dict(FINE, size=2, color_map="half")), [0]),
    "FX-PGRID-036": ("Size Map the spot: big dots in the middle, none at the edges.",
                     case(**dict(FINE, size=3, size_map="spot")), [0]),
    "FX-PGRID-037": ("Displace Map the spot, Map Displace 6: the middle pushed towards the "
                     "camera, larger and spread out.",
                     case(**dict(FINE, displace_map="spot", map_displace=6)), [0]),
    "FX-PGRID-038": ("Map Displace -6: the middle pushed away, smaller and drawn in.",
                     case(**dict(FINE, displace_map="spot", map_displace=-6)), [0]),
    "FX-PGRID-039": ("All three maps at once on the tipped grid.",
                     case(**dict(FINE, size=3, color_map="ramp", size_map="spot",
                                 displace_map="spot", map_displace=6, form_rotation_x=30)), [0]),
    "FX-PGRID-040": ("The deep grid through a composition camera 12 pixels in front: the far "
                     "layers much smaller, the near ones larger.",
                     case(camera=(8, 5, -12, 12), **DEEP), [0]),
    "FX-PGRID-041": ("FX-PGRID-028 with the layer starting at frame 10: frame 10 as its frame "
                     "0, frame 22 as its frame 12.", case(in_frame=10, **dict(RIPPLE, flow_x=12)),
                     [10, 22]),
    "FX-PGRID-042": ("FX-PGRID-027 with the layer moved 3 pixels right.",
                     case(shift=3, **RIPPLE), [0]),
    "FX-PGRID-043": ("FX-PGRID-027 with Random Seed 7: a different ripple.",
                     case(**dict(RIPPLE, random_seed=7)), [0]),
    "FX-PGRID-044": ("Particles In X keyed from 2 at frame 0 to 6 at frame 24, whole points "
                     "only: 3 across at frame 9, 5 at frame 21.",
                     case(**dict(FINE, particles_in_x=keyed((0, 2), (24, 6)))), [9, 21]),
    "FX-PGRID-045": ("Size 0: nothing.", case(**dict(SMALL, size=0)), [0]),
    "FX-PGRID-046": ("Opacity 0: nothing.", case(**dict(SMALL, opacity=0)), [0]),
    "FX-PGRID-047": ("Form Size 0: all eight dots on one spot in the middle, piled up.",
                     case(**dict(SMALL, form_size_x=0, form_size_y=0, opacity=30)), [0]),
    "FX-PGRID-048": ("One point: a single dot in the middle.",
                     case(**dict(SMALL, particles_in_x=1, particles_in_y=1, size=5)), [0]),
    "FX-PGRID-049": ("A glowing grid coloured by the ramp, rippling and added.",
                     case(**dict(RIPPLE, size=3, particle_type="glow_sphere", color_map="ramp",
                                 transfer_mode="add", flow_evolution=0.5)), [12]),
}
FRAMES_OF = {fx: frames for fx, (_, _, frames) in CASES.items()}
assert all(in_range(c) for _, c, _ in CASES.values())

INVALID = {
    "FX-PGRID-050": ("Particles In X 0, below 1.", case(particles_in_x=0)),
    "FX-PGRID-051": ("Particles In Z 101, above 100.", case(particles_in_z=101)),
    "FX-PGRID-052": ("Size -1, below 0.", case(size=-1)),
    "FX-PGRID-053": ("Opacity 101, above 100.", case(opacity=101)),
    "FX-PGRID-054": ("Fractal Scale 0, below 1.", case(fractal_scale=0)),
    "FX-PGRID-055": ("Base Form \"text\": not built.", case(base_form="text")),
    "FX-PGRID-056": ("Base Form \"Box_Grid\": the word is exact.", case(base_form="Box_Grid")),
    "FX-PGRID-057": ("Particle Type \"streak\": a grid does not move, so no streaks.",
                     case(particle_type="streak")),
    "FX-PGRID-058": ("Transfer Mode \"lighten\".", case(transfer_mode="lighten")),
    "FX-PGRID-059": ("Color \"#12345\": not a colour.", case(color="#12345")),
    "FX-PGRID-060": ("Flow Evolution 1001, above 1000.", case(flow_evolution=1001)),
}

LISTED = [("FX-PGRID-016", 0), ("FX-PGRID-018", 0), ("FX-PGRID-021", 0), ("FX-PGRID-022", 0),
          ("FX-PGRID-023", 12), ("FX-PGRID-024", 0), ("FX-PGRID-026", 0), ("FX-PGRID-028", 24),
          ("FX-PGRID-029", 12), ("FX-PGRID-031", 0), ("FX-PGRID-032", 0), ("FX-PGRID-033", 24),
          ("FX-PGRID-034", 0), ("FX-PGRID-036", 0), ("FX-PGRID-037", 0), ("FX-PGRID-039", 0),
          ("FX-PGRID-041", 22), ("FX-PGRID-044", 9)]


def listed():
    """Each point's j, place (engine units), size in pixels and opacity."""
    out = {"tolerance": 1e-7, "cases": {}}
    for fx, f in LISTED:
        rows = [[j] + x + [size, alpha] for j, x, size, alpha, _ in points(CASES[fx][1], f)]
        out["cases"][fx] = {"frame": f, "particles": rows}
        print(f"{fx} frame {f}: {len(rows)} points listed")
    (OUT / "particles_particle_grid.json").write_text(json.dumps(out, indent=1) + "\n",
                                                      encoding="utf-8")


def clear():
    """No number lands so near a cut that the build's rounding could put it on the other side:
    a point count near a whole number, a depth near the camera's cut, a pixel centre on a star's
    outline, or two overlapping dots at nearly but not quite the same depth."""
    for fx, (_, c, frames) in CASES.items():
        for f in frames:
            if f < c["in_frame"]:
                continue
            s = sample(c, f)
            for a in "xyz":
                v = s["particles_in_" + a]
                assert v == math.floor(v) or abs(v - round(v)) > 1e-6, (fx, f, a)
            sps = sprites(c, f)
            for i in range(len(sps)):
                assert abs(sps[i][0] - 0.01) > 1e-6, (fx, f)
                for j in range(i + 1, len(sps)):
                    a, b = sps[i][2], sps[j][2]
                    near = math.hypot(a["X"] - b["X"], a["Y"] - b["Y"]) < a["R"] + b["R"] + 2
                    gap = abs(sps[i][0] - sps[j][0])
                    assert not (near and 0 < gap < 1e-7), (fx, f, i, j, gap)
            for _, _, sp in sps:
                if sp["kind"] == "star":
                    for gy in range(H):
                        dy = gy + 0.5 - sp["Y"]
                        for sx, sy in PL.STAR:
                            assert abs(sy * sp["R"] - dy) > 1e-9, (fx, gy)
    print("clear")


def check(expected):
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    clear_frame = [[0.0] * 4 for _ in range(W * H)]
    total = lambda fr: sum(p[3] for p in fr)  # noqa: E731
    g = lambda n, f=0: c["FX-PGRID-%03d" % n][str(f)]  # noqa: E731
    assert g(1, 0) == g(1, 12) and total(g(1)) > 0
    assert g(1)[5 * W + 8][3] == 1.0 and g(1)[0][3] == 0.0
    assert len(points(CASES["FX-PGRID-001"][1], 0)) == 41 * 31
    assert total(g(3)) > 0 and total(g(8)) < total(g(2))
    assert g(13) != g(14) != g(15) != g(13)
    for n in (16, 17, 18, 19, 24, 26, 27):
        assert g(n) != g(3), n
    assert g(20) != g(2) and g(21) != g(22)
    assert g(23, 0) != g(23, 12) != g(23, 24)
    assert g(24) != g(25) and g(27) != g(30) and g(27) != g(43)
    assert g(28, 0) == g(27) and g(28, 12) != g(28, 0) != g(28, 24)
    assert g(29, 0) == g(27) and g(29, 12) != g(27)
    assert g(31) != g(32)
    assert g(41, 10) == g(28, 0) and g(41, 22) == g(28, 12)
    m, b = g(42), g(27)
    for y in range(H):
        assert m[y * W + 3:(y + 1) * W] == b[y * W:y * W + W - 3]
    assert g(44, 9) != g(44, 21)
    assert g(45) == clear_frame and g(46) == clear_frame
    # the colour map: the right of the ramp redder, the half map leaves the right side clear
    ramp = g(34)
    assert ramp[5 * W + 14][0] > ramp[5 * W + 2][0]
    assert all(g(35)[y * W + x][3] == 0 for y in range(H) for x in range(9, W))
    # the size map leaves the corner dots out; the displace map spreads the middle outwards
    assert g(36)[1 * W + 1][3] == 0 and g(36)[5 * W + 8][3] > 0
    near, far = points(CASES["FX-PGRID-037"][1], 0), points(CASES["FX-PGRID-038"][1], 0)
    assert near[7][1][2] < 0 < far[7][1][2]
    # the deep grid's front layer is drawn last
    assert sprites(CASES["FX-PGRID-019"][1], 0)[-1][1] < 15
    # the flow sums: Flow Y keyed 0 to 24 over 24 frames moves 12 pixels by frame 24
    assert abs(flowed(CASES["FX-PGRID-033"][1], 24)["flow_y"] - 12) < 1e-12
    print("checked")


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, rows in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(rows))
    clear()
    expected = B.write_cases(OUT, "particle_grid", CASES, INVALID, render, plain, file_json)
    listed()
    check(expected)


if __name__ == "__main__":
    main()
