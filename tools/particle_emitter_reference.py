"""Particle Emitter, worked a second way.

`core.particle_emitter` (D-615), the particle engine's fourth effect (P0-6, D-600), modelled on
what Trapcode Particular is for (`docs/effects/PLUGINS.md` 2.1, top-20 pick #20): particles
thrown from a point, a box, a ball or a grid in 3D, each with its own life, size, opacity and
colour changing over that life, moved by gravity, air resistance, wind and turbulence, bounced,
slid or stuck on a floor, and drawn as soft spheres, glowing spheres, stars, cloudlets or streaks
through the composition's camera. Red Giant publishes no formula, so every rule and every number
when added below is ours (document 21, D-615). No text or code was copied. The second system
(particles thrown by particles), the layer, light, text and model emitters, lights, shadows,
motion blur and depth of field are not built in this step (D-615 says why).

**No state is carried from frame to frame** (document 20): this is D-610's engine
(`tools/particle_space_reference.py`), whose births, steps, floor and camera this file runs. The
Emitter's settings are worked into the engine's once a frame; the Emitter's own parts are the
emitter's shape and the directions, the per-particle life, size, opacity and colour, the curves
over life, and two new sprites.

The settings are SETTINGS (name, lowest, highest, value when added; all keyable), POINTS (two
numbers, per cent of the layer's width and height, each held to -1000 to 1000; keyable), WORDS
(exact) and COLOURS (`#rrggbb`). Lengths are in the layer's pixels (a draft divides them as it
divides every distance), speeds in pixels a second, accelerations in pixels a second a second,
times in seconds, angles in degrees. With W and H the layer's size, n the frame drawn, i0 the
layer's in point, u = n - i0 and fps the composition's rate:

- The engine's settings, at every frame sample (D-610: sampled at i0 + i, i = 0 to u + 1, held
  inside their ranges, straight-line between): Birth Rate = Particles Per Second / 100 (so q =
  Particles Per Second / fps a frame), Longevity = Life, Producer = (Emitter Position_x / 100 -
  1/2, (Emitter Position_y / 100 - 1/2) H / W, Emitter Z / W), Gravity = Gravity / W with Gravity
  Vector (0, 1, 0) (down), Resistance = Air Resistance, Wind = (Wind X, Y, Z) / W, Turbulence =
  Turbulence / W, Turbulence Scale = Turbulence Scale / W, Turbulence Evolution as it is, Floor
  Position = (Floor Position / 100 - 1/2) H / W, Bounciness = Bounce, Random Bounciness = Bounce
  Random, Bounce Spread 0, Floor Action none, bounce, ice (Slide) or glue (Stick), Random Seed as
  it is, Animation Explosive (no swirl, no extra turbulence). The engine's unit is W, its origin
  the layer's centre, Y down, Z away.
- Randomness, D-610's r(k, s). Streams: 0 to 2 the place in the emitter, 3 and 4 the direction,
  5 the speed, 6 the size, 10 the side (Bi-Directional), 11 the life, 12 the opacity, 13 the
  place on the gradient, 14 the colour's brightness; 100 + 2m and 101 + 2m the bounces (D-610).
- Births as D-610's: particle k (0 up) at b_k. Read at b_k: the emitter's turn M = Rx(Emitter
  Rotation X) Ry(Emitter Rotation Y) Rz(Emitter Rotation Z) (Rz first, as D-610's own camera),
  the size S = (Emitter Size X, Y, Z) and, for the grid, n = (floor Grid X, floor Grid Y, floor
  Grid Z).
- Place: x = Producer + M d / W, d in pixels by Emitter Type:
  Point (0, 0, 0); Box S (r0 - 1/2, r1 - 1/2, r2 - 1/2); Sphere (S / 2) cbrt(r0) times the unit
  vector with z = 1 - 2 r1 and turn 2 pi r2 (D-610's, component by component); Grid the point j =
  k mod (n_x n_y n_z), i = j mod n_x, l = floor(j / n_x) mod n_y, m = floor(j / (n_x n_y)),
  d = (S_x ((i + 1/2) / n_x - 1/2), S_y ((l + 1/2) / n_y - 1/2), S_z ((m + 1/2) / n_z - 1/2)).
- Direction, with the axis A = M (0, -1, 0) (up before any turn), e1 and e2 D-610's basis of A,
  phi = 2 pi r4, alpha = 180 Direction Spread / 100 degrees, u the uniform unit vector with
  z = 1 - 2 r3 and turn phi, and cap(c) = c A + sqrt(1 - c^2)(cos phi e1 + sin phi e2):
  Uniform u; Directional cap(1 - r3 (1 - cos alpha)); Bi-Directional the Directional way, turned
  back (times -1) when r10 < 1/2; Disc cap(-sin((alpha / 2)(2 r3 - 1))), round the axis and
  tilted up to alpha / 2 off the disc square to it; Outwards o + (Direction Spread / 100) u made
  unit, o = M d made unit (u when d is 0), and o itself when that sum is 0; Inwards the same with
  -o.
- Speed: Velocity / W (1 + Velocity Random / 100 (2 r5 - 1)) along the direction, plus Velocity
  From Motion / 100 times the Producer's velocity (D-610's Inherit Velocity: (P(b + 1/8) -
  P(b - 1/8)) / (1/4 frame)).
- Life, read at b_k: Life fps (1 - Life Random / 100 r11) frames; alive at u when b_k <= u and
  u - b_k is less than that. f = (u - b_k) / life, its age fraction.
- Steps: D-610's, from the birth on the grid of 4 a frame, with the engine's settings above
  (gravity, resistance in closed form, wind, turbulence's curl, the floor).
- Size, in pixels: Size (1 + Size Random / 100 (2 r6 - 1)) (both read at b_k) times the curve
  over life at f / 100: the five Size Over Life levels at f = 0, 1/4, 1/2, 3/4 and 1, straight
  line between (the levels read at the frame drawn).
- Opacity: Opacity / 100 (1 - Opacity Random / 100 r12) (read at b_k) times the five Opacity Over
  Life levels / 100 at f as the size's.
- Colour, linear light: At Birth Color; Over Life the gradient at f; Random From Gradient the
  gradient at r13; the gradient the five Gradient Colors at 0, 1/4, 1/2, 3/4 and 1, straight
  line between, worked in linear light. Then times 1 - Color Random / 100 r14 (read at b_k).
- Camera: the composition's camera when the file has one, as D-610's; otherwise the layer seen
  straight on from W 50 / 36 in front of its centre (After Effects' 50 mm default view), so the
  layer's plane is drawn 1:1. A particle at (W/2 + x W, H/2 + y W), depth z W, is drawn there
  through the camera, one unit p pixels across, at a depth; not drawn when the depth is 0.01 or
  less. R = size p / (2 W).
- Sprites, e the pixel centre's distance from the particle's place. Sphere, Glow Sphere, Star
  and Cloudlet: when R < 0.7 drawn with R = 0.7 and the opacity times (R / 0.7)^2. Sphere: w =
  max(1, Sphere Feather / 100 R), covering clamp((R - e) / w + 1/2). Glow Sphere: a core of half
  the radius, c = clamp((R / 2 - e) / w + 1/2) with w = max(1, Sphere Feather / 100 R / 2), and
  a glow out to R, covering c + (1 - c) max(0, 1 - e / R)^2 / 2. Star: D-612's. Cloudlet: D-610's
  Faded Sphere, max(0, 1 - (e / R)^2)^2. Streak: from the place back along one frame's movement,
  to (W/2, H/2) + (x - v / fps) W through the camera, width max(1, 2 R), D-610's Line covering.
  None is lit (Particle Emitter has no lights yet). The Sphere Feather is read at the frame
  drawn.
- Order and transfer: over a clear layer (the layer's own picture is not shown), farthest first,
  equal depths by number; Normal, Add and Screen as D-610's Composite, Add and Screen.

**This file never runs the build's code path.** It works in double precision on lists from the
drawing's 8-bit values; trigonometry is the platform's. The drawing is Broadcast Colors' 16 by
10. The projects go into `Fixtures/particle_emitter`, the expected frames into
`Fixtures/particle_emitter/expected_particle_emitter.json`, each alive particle's numbers for
some cases into `Fixtures/particle_emitter/particles_particle_emitter.json`. Every case is "not
built yet" until B-495 lands (D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/particle_emitter_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from adjust_reference import srgb_to_linear, prop  # noqa: E402
import particle_space_reference as P  # noqa: E402
import particle_plane_reference as PL  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = B.W, B.H
FPS = P.FPS
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "particle_emitter"

SETTINGS = [
    ("particles_per_second", 0, 100000, 100), ("emitter_z", -100000, 100000, 0),
    ("emitter_size_x", 0, 100000, 50), ("emitter_size_y", 0, 100000, 50),
    ("emitter_size_z", 0, 100000, 50),
    ("emitter_rotation_x", -36000, 36000, 0), ("emitter_rotation_y", -36000, 36000, 0),
    ("emitter_rotation_z", -36000, 36000, 0),
    ("grid_x", 1, 1000, 5), ("grid_y", 1, 1000, 5), ("grid_z", 1, 1000, 1),
    ("velocity", -100000, 100000, 100), ("velocity_random", 0, 100, 20),
    ("velocity_from_motion", -10000, 10000, 20), ("direction_spread", 0, 100, 20),
    ("life", 0, 1000, 3), ("life_random", 0, 100, 0),
    ("size", 0, 10000, 4), ("size_random", 0, 100, 0),
    ("size_over_life_1", 0, 1000, 100), ("size_over_life_2", 0, 1000, 100),
    ("size_over_life_3", 0, 1000, 100), ("size_over_life_4", 0, 1000, 100),
    ("size_over_life_5", 0, 1000, 100),
    ("opacity", 0, 100, 100), ("opacity_random", 0, 100, 0),
    ("opacity_over_life_1", 0, 100, 100), ("opacity_over_life_2", 0, 100, 100),
    ("opacity_over_life_3", 0, 100, 100), ("opacity_over_life_4", 0, 100, 100),
    ("opacity_over_life_5", 0, 100, 100),
    ("color_random", 0, 100, 0), ("sphere_feather", 0, 100, 50),
    ("gravity", -100000, 100000, 0), ("air_resistance", 0, 100, 0),
    ("wind_x", -100000, 100000, 0), ("wind_y", -100000, 100000, 0),
    ("wind_z", -100000, 100000, 0),
    ("turbulence", 0, 100000, 0), ("turbulence_scale", 1, 100000, 100),
    ("turbulence_evolution", -100000, 100000, 0),
    ("floor_position", -1000, 1000, 100), ("bounce", 0, 100, 50), ("bounce_random", 0, 100, 0),
    ("random_seed", 0, 10000, 0),
]
NAMES = [s[0] for s in SETTINGS]
RANGES = {s[0]: (s[1], s[2]) for s in SETTINGS}
POINTS = [("emitter_position", [50, 50])]
WORDS = [
    ("emitter_type", ("point", "box", "sphere", "grid"), "point"),
    ("direction", ("uniform", "directional", "bi_directional", "disc", "outwards", "inwards"),
     "uniform"),
    ("particle_type", ("sphere", "glow_sphere", "star", "cloudlet", "streak"), "sphere"),
    ("set_color", ("at_birth", "over_life", "random_from_gradient"), "at_birth"),
    ("transfer_mode", ("normal", "add", "screen"), "normal"),
    ("floor_action", ("none", "bounce", "slide", "stick"), "none"),
]
COLOURS = [
    ("color", "#ffffff"),
    # ours: white through yellow and orange to red and a dark red, a spark's cooling.
    ("gradient_color_1", "#ffffff"), ("gradient_color_2", "#ffe066"),
    ("gradient_color_3", "#ff9933"), ("gradient_color_4", "#e6331a"),
    ("gradient_color_5", "#4d0d0d"),
]
FLOOR_ACTION = {"none": "none", "bounce": "bounce", "slide": "ice", "stick": "glue"}
TRANSFER = {"normal": "composite", "add": "add", "screen": "screen"}
DEFAULT_VIEW = W * 50 / 36


# --- the run --------------------------------------------------------------------------------

def sample(c, frame_no):
    s = {}
    for n in NAMES:
        lo, hi = RANGES[n]
        s[n] = P.clamp(value_at(c[n], frame_no), lo, hi)
    for n, _ in POINTS:
        x, y = value_at(c[n], frame_no)
        s[n + "_x"], s[n + "_y"] = P.clamp(x, -1000, 1000), P.clamp(y, -1000, 1000)
    return s


def engine(s):
    """One frame's Emitter settings as Particle Space's."""
    e = {n: d for n, _, _, d in P.SETTINGS}
    e.update(
        birth_rate=s["particles_per_second"] / 100, longevity=s["life"],
        producer_x=s["emitter_position_x"] / 100 - 0.5,
        producer_y=(s["emitter_position_y"] / 100 - 0.5) * H / W,
        producer_z=s["emitter_z"] / W,
        gravity=s["gravity"] / W, gravity_x=0.0, gravity_y=1.0, gravity_z=0.0,
        resistance=s["air_resistance"],
        wind_x=s["wind_x"] / W, wind_y=s["wind_y"] / W, wind_z=s["wind_z"] / W,
        turbulence=s["turbulence"] / W, turbulence_scale=s["turbulence_scale"] / W,
        turbulence_evolution=s["turbulence_evolution"],
        floor_position=(s["floor_position"] / 100 - 0.5) * H / W,
        bounciness=s["bounce"], random_bounciness=s["bounce_random"], bounce_spread=0.0,
        random_seed=s["random_seed"])
    return e


class Run(P.Run):
    def __init__(self, c, n):
        self.c = dict(c, animation="explosive", floor_action=FLOOR_ACTION[c["floor_action"]])
        self.own_c = c
        self.u = n - c["in_frame"]
        self.own = [sample(c, c["in_frame"] + i) for i in range(max(0, self.u) + 2)]
        self.samples = [engine(s) for s in self.own]
        self.now = self.samples[max(0, self.u)]
        self.own_now = self.own[max(0, self.u)]
        self.seed = math.floor(self.now["random_seed"])

    def mine(self, name, t):
        s = self.own
        last = len(s) - 1
        i0 = math.floor(t)
        if i0 < 0:
            return s[0][name]
        if i0 >= last:
            return s[last][name]
        a, b = s[i0][name], s[i0 + 1][name]
        return a + (b - a) * (t - i0)


def turn(run, b):
    m = [run.mine("emitter_rotation_" + a, b) for a in "xyz"]
    return P.mat(P.rx(math.radians(m[0])), P.mat(P.ry(math.radians(m[1])), P.rz(math.radians(m[2]))))


def offset(run, k, b):
    """d, the place in the emitter in pixels before the turn."""
    r = lambda s: run.r(k, s)  # noqa: E731
    size = [run.mine("emitter_size_" + a, b) for a in "xyz"]
    kind = run.own_c["emitter_type"]
    if kind == "point":
        return [0.0, 0.0, 0.0]
    if kind == "box":
        return [size[0] * (r(0) - 0.5), size[1] * (r(1) - 0.5), size[2] * (r(2) - 0.5)]
    if kind == "sphere":
        rho = r(0) ** (1 / 3)
        d = P.sphere(1 - 2 * r(1), 2 * math.pi * r(2))
        return [size[i] / 2 * rho * d[i] for i in range(3)]
    n = [math.floor(run.mine("grid_" + a, b)) for a in "xyz"]
    j = k % (n[0] * n[1] * n[2])
    idx = [j % n[0], (j // n[0]) % n[1], j // (n[0] * n[1])]
    return [size[i] * ((idx[i] + 0.5) / n[i] - 0.5) for i in range(3)]


def born(run, k, b):
    r = lambda s: run.r(k, s)  # noqa: E731
    m = turn(run, b)
    d = P.apply(m, offset(run, k, b))
    p = run.vec("producer", b)
    x = [p[i] + d[i] / W for i in range(3)]
    a = P.apply(m, [0.0, -1.0, 0.0])
    e1, e2 = P.basis(a)
    phi = 2 * math.pi * r(4)
    spread = run.mine("direction_spread", b)
    alpha = math.radians(180 * spread / 100)
    u = P.sphere(1 - 2 * r(3), phi)

    def cap(ct):
        st = math.sqrt(max(0.0, 1 - ct * ct))
        return P.add(P.mul(a, ct), P.mul(P.add(P.mul(e1, math.cos(phi)), P.mul(e2, math.sin(phi))), st))

    mode = run.own_c["direction"]
    if mode == "uniform":
        way = u
    elif mode in ("directional", "bi_directional"):
        way = cap(1 - r(3) * (1 - math.cos(alpha)))
        if mode == "bi_directional" and r(10) < 0.5:
            way = P.mul(way, -1)
    elif mode == "disc":
        way = cap(-math.sin(alpha / 2 * (2 * r(3) - 1)))
    else:
        o = P.unit_or(d, u) if any(d) else list(u)
        if mode == "inwards":
            o = P.mul(o, -1)
        way = P.unit_or(P.add(o, P.mul(u, spread / 100)), o)
    speed = run.mine("velocity", b) / W * (1 + run.mine("velocity_random", b) / 100 * (2 * r(5) - 1))
    v = P.mul(way, speed)
    vm = run.mine("velocity_from_motion", b)
    if vm != 0:
        pv = P.mul(P.sub(run.vec("producer", b + 0.125), run.vec("producer", b - 0.125)), FPS / 0.25)
        v = P.add(v, P.mul(pv, vm / 100))
    return {"k": k, "b": b, "x": x, "v": v, "x0": list(x), "twirl": 1.0,
            "glued": False, "iced": False, "bounces": 0,
            "life": run.mine("life", b) * FPS * (1 - run.mine("life_random", b) / 100 * r(11)),
            "size": run.mine("size", b) * (1 + run.mine("size_random", b) / 100 * (2 * r(6) - 1)),
            "alpha": run.mine("opacity", b) / 100 * (1 - run.mine("opacity_random", b) / 100 * r(12)),
            "pick": r(13),
            "bright": 1 - run.mine("color_random", b) / 100 * r(14)}


def moved(run, k, b):
    p = born(run, k, b)
    points = [b] + [j / 4 for j in range(math.floor(4 * b) + 1, 4 * run.u + 1)]
    for t0, t1 in zip(points, points[1:]):
        P.step(run, p, t0, (t1 - t0) / FPS)
    return p


def alive(run):
    """(k, particle, f) for every particle alive at u, oldest first."""
    out = []
    for k, b in P.births(run):
        lr = run.mine("life_random", b) / 100
        life = run.mine("life", b) * FPS * (1 - lr * run.r(k, 11))
        age = run.u - b
        if age < life:
            out.append((k, moved(run, k, b), age / life))
    return out


# --- the drawing ----------------------------------------------------------------------------

def curve(levels, f):
    i = min(math.floor(f * 4), 3)
    return levels[i] + (levels[i + 1] - levels[i]) * (f * 4 - i)


def gradient(c, f):
    stops = [P.linear(c["gradient_color_%d" % i]) for i in range(1, 6)]
    i = min(math.floor(f * 4), 3)
    return P.mix3(stops[i], stops[i + 1], f * 4 - i)


def view_of(c):
    if c["camera"] is None:
        return (W / 2, H / 2, DEFAULT_VIEW)
    return P.view_of(c)


def sprites(run):
    c, now = run.own_c, run.own_now
    see = P.camera(run, view_of(c))
    sizes = [now["size_over_life_%d" % i] / 100 for i in range(1, 6)]
    fades = [now["opacity_over_life_%d" % i] / 100 for i in range(1, 6)]
    feather = now["sphere_feather"] / 100
    out = []
    for k, p, f in alive(run):
        s = see(p["x"])
        if s is None:
            continue
        X, Y, ppu, depth = s
        size = p["size"] * curve(sizes, f)
        rad = size * ppu / (2 * W)
        alpha = p["alpha"] * curve(fades, f)
        mode = c["set_color"]
        col = (P.linear(c["color"]) if mode == "at_birth"
               else gradient(c, f if mode == "over_life" else p["pick"]))
        col = [v * p["bright"] for v in col]
        kind = c["particle_type"]
        sp = {"kind": kind, "X": X, "Y": Y, "col": col}
        if kind == "streak":
            tail = see(P.sub(p["x"], P.mul(p["v"], 1 / FPS)))
            if tail is None:
                continue
            sp.update(X2=tail[0], Y2=tail[1], w=max(1.0, 2 * rad), alpha=alpha)
        else:
            if rad < 0.7:
                alpha *= (rad / 0.7) ** 2
                rad = 0.7
            sp.update(R=rad, alpha=alpha, feather=feather)
        out.append((depth, k, sp, p))
    out.sort(key=lambda e: (-e[0], e[1]))
    return out


def cover(sp, x, y):
    kind = sp["kind"]
    if kind == "streak":
        return P.cover(dict(sp, kind="line"), x, y, 0.0, None)
    if kind == "cloudlet":
        return P.cover(dict(sp, kind="faded_sphere"), x, y, 0.0, None)
    rad = sp["R"]
    e = math.hypot(x - sp["X"], y - sp["Y"])
    if kind == "star":
        cv = PL.star_cover(rad, x - sp["X"], y - sp["Y"])
    elif kind == "sphere":
        w = max(1.0, sp["feather"] * rad)
        cv = P.clamp((rad - e) / w + 0.5)
    else:
        w = max(1.0, sp["feather"] * rad / 2)
        core = P.clamp((rad / 2 - e) / w + 0.5)
        cv = core + (1 - core) * max(0.0, 1 - e / rad) ** 2 / 2
    if cv <= 0:
        return None
    a = sp["alpha"] * cv
    return [sp["col"][i] * a for i in range(3)] + [a]


def reach(sp):
    if sp["kind"] == "streak":
        r = sp["w"] / 2 + 1
        return ((min(sp["X"], sp["X2"]) - r, max(sp["X"], sp["X2"]) + r),
                (min(sp["Y"], sp["Y2"]) - r, max(sp["Y"], sp["Y2"]) + r))
    r = sp["R"] + 1
    if sp["kind"] == "sphere":
        r += max(1.0, sp["feather"] * sp["R"]) / 2
    return (sp["X"] - r, sp["X"] + r), (sp["Y"] - r, sp["Y"] + r)


def particle_emitter(px, c, n):
    run = Run(c, n)
    if run.u < 0:
        return [list(p) for p in px]
    out = [[0.0] * 4 for _ in px]
    mode = TRANSFER[c["transfer_mode"]]
    for _, _, sp, _ in sprites(run):
        xs, ys = reach(sp)
        for gy in range(max(0, math.floor(ys[0])), min(H, math.ceil(ys[1]) + 1)):
            for gx in range(max(0, math.floor(xs[0])), min(W, math.ceil(xs[1]) + 1)):
                s = cover(sp, gx + 0.5, gy + 0.5)
                if s is None:
                    continue
                d = out[gy * W + gx]
                if mode == "composite":
                    d = [s[i] + (1 - s[3]) * d[i] for i in range(4)]
                elif mode == "screen":
                    d = [s[i] + d[i] - s[i] * d[i] for i in range(4)]
                else:
                    d = [s[0] + d[0], s[1] + d[1], s[2] + d[2], min(1.0, s[3] + d[3])]
                out[gy * W + gx] = d
    return out


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, camera=None, in_frame=0, **given):
    c = {s[0]: s[3] for s in SETTINGS}
    c.update({n: list(v) for n, v in POINTS})
    c.update({w[0]: w[2] for w in WORDS})
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
    out = particle_emitter(P.working(), c, frame_no)
    move = c["shift"]
    if c["camera"] is not None:
        assert c["camera"][1] == H / 2 and c["camera"][0] == int(c["camera"][0])
        move += int(W / 2 - c["camera"][0])
    return R.frame(out, move)


def plain(c):
    move = c["shift"]
    if c["camera"] is not None:
        move += int(W / 2 - c["camera"][0])
    return R.frame(P.working(), move)


def file_json(fx, c):
    params = {n: setting_json(c[n]) for n in NAMES}
    params.update({n: setting_json(c[n]) for n, _ in POINTS})
    params.update({w[0]: c[w[0]] for w in WORDS})
    params.update({k: c[k] for k, _ in COLOURS})
    p = B.project_json(fx, "core.particle_emitter", params, c["shift"])
    comp = p["compositions"][0]
    frames = max([f for f in FRAMES_OF.get(fx, [0])] + [4]) + 2
    comp["duration_frames"] = frames
    comp["work_area"]["end_frame_exclusive"] = frames
    layer = comp["layers"][0]
    layer["in_frame"] = c["in_frame"]
    layer["out_frame"] = frames
    if c["camera"] is not None:
        x, y, depth, zoom = c["camera"]
        comp["camera"] = {"position": prop([x, y]), "depth": prop(depth), "zoom": prop(zoom)}
    return p


# One particle, still, in the middle: 1.3 a second, the first born at frame 9.23 and the next at
# 27.7, so frame 20 shows the first alone at age fraction 10.77 / 48. Size 8 is 8 pixels across.
ONE = dict(particles_per_second=1.3, life=2, velocity=0, velocity_from_motion=0, size=8)
# Two: 2.6 a second, born at frames 4.62 and 13.85, apart in a box.
TWO = dict(ONE, particles_per_second=2.6, emitter_type="box", emitter_size_x=6, emitter_size_y=3,
           emitter_size_z=0, size=7, color="#3366cc")
# Still particles spread in a box, two pixels across, 12 a second for 4 seconds.
SPREAD = dict(ONE, particles_per_second=12, emitter_type="box", emitter_size_x=14,
              emitter_size_y=8, emitter_size_z=0, size=2, life=4)
# A burst to watch move: 24 a second, 30 pixels a second, 1.5 pixels across.
FLOW = dict(particles_per_second=24, life=1.5, velocity=30, velocity_random=30,
            velocity_from_motion=0, size=1.5, opacity=90)
# A jet from below the middle.
JET = dict(FLOW, direction="directional", direction_spread=10, emitter_position=[50, 85])

CASES = {
    "FX-PEMIT-001": ("The settings as added, at frames 0, 6 and 12: nothing at frame 0, then 100 "
                     "white soft dots a second thrown every way at about 100 pixels a second, so "
                     "fast most are out of the 16 by 10 layer within frames; the layer's own "
                     "picture is gone.", case(), [0, 6, 12]),
    "FX-PEMIT-002": ("One particle, still, in the middle: a white Sphere 8 pixels across, "
                     "feathered by half its radius.", case(**ONE), [20]),
    "FX-PEMIT-003": ("Sphere Feather 0: a hard-edged disc.", case(**dict(ONE, sphere_feather=0)),
                     [20]),
    "FX-PEMIT-004": ("Sphere Feather 100: soft from the middle out.",
                     case(**dict(ONE, sphere_feather=100)), [20]),
    "FX-PEMIT-005": ("A Glow Sphere: a solid core half as wide and a faint glow round it.",
                     case(**dict(ONE, particle_type="glow_sphere")), [20]),
    "FX-PEMIT-006": ("A Star: five points up.", case(**dict(ONE, particle_type="star")), [20]),
    "FX-PEMIT-007": ("A Cloudlet: soft all the way to its edge.",
                     case(**dict(ONE, particle_type="cloudlet")), [20]),
    "FX-PEMIT-008": ("A still Streak: no movement, so a round dot as wide as the size.",
                     case(**dict(ONE, particle_type="streak")), [20]),
    "FX-PEMIT-009": ("A tiny sphere, size 1: drawn 1.4 pixels across and fainter by its true "
                     "size.", case(**dict(ONE, size=1)), [20]),
    "FX-PEMIT-010": ("Size Over Life 100, 30, 100, 100, 100: at a fifth of its life it is "
                     "shrinking towards the second level, about 37 per cent of its size.",
                     case(**dict(ONE, size_over_life_2=30)), [20]),
    "FX-PEMIT-011": ("Opacity Over Life 0, 100, 100, 100, 100: fading in, about 90 per cent at a "
                     "fifth of its life.", case(**dict(ONE, opacity_over_life_1=0)), [20]),
    "FX-PEMIT-012": ("Opacity 50: half covering.", case(**dict(ONE, opacity=50)), [20]),
    "FX-PEMIT-013": ("Opacity Over Life keyed: the second level from 100 at frame 0 to 10 at "
                     "frame 20, read at the frame drawn.",
                     case(**dict(ONE, opacity_over_life_2=keyed((0, 100), (20, 10)))), [20]),
    "FX-PEMIT-014": ("Opacity Random 80 on a spread of still dots: each its own opacity.",
                     case(**dict(SPREAD, opacity_random=80)), [20]),
    "FX-PEMIT-015": ("Size Random 80 on the spread: each its own size.",
                     case(**dict(SPREAD, size_random=80)), [20]),
    "FX-PEMIT-016": ("Life Random 90 on the spread: some gone early, at frames 20 and 40.",
                     case(**dict(SPREAD, life_random=90, life=2)), [20, 40]),
    "FX-PEMIT-017": ("At Birth colour blue.", case(**dict(ONE, color="#2040ff")), [20]),
    "FX-PEMIT-018": ("Over Life: at a fifth of its life, between white and yellow on the "
                     "gradient.", case(**dict(ONE, set_color="over_life")), [20]),
    "FX-PEMIT-019": ("Random From Gradient on the spread: each dot a colour from white to dark "
                     "red.", case(**dict(SPREAD, set_color="random_from_gradient")), [20]),
    "FX-PEMIT-020": ("Color Random 70 on the spread: each dot its own brightness.",
                     case(**dict(SPREAD, color_random=70, color="#ffcc33")), [20]),
    "FX-PEMIT-021": ("A Sphere emitter 14 wide, 8 high and 30 deep: still dots in a ball, the "
                     "far ones smaller and nearer the middle.",
                     case(**dict(SPREAD, emitter_type="sphere", emitter_size_z=30)), [20]),
    "FX-PEMIT-022": ("A Grid emitter 4 by 2 over 12 by 6 pixels: a dot a frame, filling the grid "
                     "point by point in rows.",
                     case(**dict(ONE, particles_per_second=24, life=10, emitter_type="grid",
                                 grid_x=4, grid_y=2, emitter_size_x=12, emitter_size_y=6,
                                 emitter_size_z=0, size=1.6)), [6, 20]),
    "FX-PEMIT-023": ("The grid turned 30 degrees about Z.",
                     case(**dict(ONE, particles_per_second=24, life=10, emitter_type="grid",
                                 grid_x=4, grid_y=2, emitter_size_x=12, emitter_size_y=6,
                                 emitter_size_z=0, size=1.6, emitter_rotation_z=30)), [20]),
    "FX-PEMIT-024": ("The particle 11 pixels farther away and up the left: smaller, drawn nearer "
                     "the middle.",
                     case(**dict(ONE, emitter_z=11, emitter_position=[22, 31])), [20]),
    "FX-PEMIT-025": ("Two overlapping blue dots, Normal: the newer over the older.",
                     case(**TWO), [20]),
    "FX-PEMIT-026": ("The two, Add: brighter where they overlap.", case(**dict(TWO, transfer_mode="add")), [20]),
    "FX-PEMIT-027": ("The two, Screen.", case(**dict(TWO, transfer_mode="screen")), [20]),
    "FX-PEMIT-028": ("A burst every way, at frames 6, 12, 24 and 40. Frame 40 drawn alone is the "
                     "same as after frames 0 to 39.", case(**FLOW), [6, 12, 24, 40]),
    "FX-PEMIT-029": ("Directional, spread 10: a jet upwards within 18 degrees.", case(**JET),
                     [12, 24]),
    "FX-PEMIT-030": ("The jet turned 90 degrees about Z: to the right.",
                     case(**dict(JET, emitter_rotation_z=90, emitter_position=[15, 50])),
                     [12, 24]),
    "FX-PEMIT-031": ("Bi-Directional, spread 10: up and down at once.",
                     case(**dict(JET, direction="bi_directional", emitter_position=[50, 50])),
                     [12, 24]),
    "FX-PEMIT-032": ("Disc, the emitter turned 90 degrees about X so the axis faces us: a ring "
                     "spreading out on the screen.",
                     case(**dict(FLOW, direction="disc", direction_spread=0,
                                 emitter_rotation_x=90)), [12, 24]),
    "FX-PEMIT-033": ("Outwards from a Sphere emitter 6 across: away from its middle.",
                     case(**dict(FLOW, direction="outwards", direction_spread=0,
                                 emitter_type="sphere", emitter_size_x=6, emitter_size_y=6,
                                 emitter_size_z=6, velocity=15)), [12, 24]),
    "FX-PEMIT-034": ("Inwards into a Sphere emitter 14 across, slowly: towards its middle.",
                     case(**dict(FLOW, direction="inwards", direction_spread=0,
                                 emitter_type="sphere", emitter_size_x=14, emitter_size_y=14,
                                 emitter_size_z=14, velocity=8)), [12, 24]),
    "FX-PEMIT-035": ("Gravity 60 on the jet: it arcs back down.",
                     case(**dict(JET, gravity=60)), [12, 24]),
    "FX-PEMIT-036": ("Air Resistance 3: the burst slows and stays tight.",
                     case(**dict(FLOW, air_resistance=3)), [12, 24]),
    "FX-PEMIT-037": ("Wind X 40 on the jet: it bends to the right.",
                     case(**dict(JET, wind_x=40)), [12, 24]),
    "FX-PEMIT-038": ("Turbulence 400 at scale 4 on the jet: it wanders.",
                     case(**dict(JET, turbulence=400, turbulence_scale=4)), [12, 24]),
    "FX-PEMIT-039": ("Bounce on a floor at 80 per cent: a fountain from the top falls and "
                     "bounces.",
                     case(**dict(FLOW, emitter_position=[50, 20], gravity=120, velocity=10,
                                 floor_action="bounce", floor_position=80)), [12, 24]),
    "FX-PEMIT-040": ("Slide: they land and slide along the floor.",
                     case(**dict(FLOW, emitter_position=[50, 20], gravity=120, velocity=10,
                                 floor_action="slide", floor_position=80)), [24]),
    "FX-PEMIT-041": ("Stick: they land and stay.",
                     case(**dict(FLOW, emitter_position=[50, 20], gravity=120, velocity=10,
                                 floor_action="stick", floor_position=80)), [24]),
    "FX-PEMIT-042": ("Velocity From Motion 100 with the emitter keyed across: the particles "
                     "carry its motion.",
                     case(**dict(FLOW, velocity=3, velocity_from_motion=100,
                                 emitter_position=keyed((0, [10, 50]), (24, [90, 50])))),
                     [12, 24]),
    "FX-PEMIT-043": ("Particles Per Second keyed from 0 at frame 0 to 110 at frame 20: births "
                     "start slowly and quicken.",
                     case(**dict(FLOW, particles_per_second=keyed((0, 0), (20, 110)))),
                     [10, 20, 30]),
    "FX-PEMIT-044": ("The ball of FX-PEMIT-021 through a composition camera 12 pixels in front: "
                     "the far dots much smaller, the near ones larger.",
                     case(camera=(8, 5, -12, 12),
                          **dict(SPREAD, emitter_type="sphere", emitter_size_z=30)), [20]),
    "FX-PEMIT-045": ("FX-PEMIT-028 with the layer starting at frame 10: nothing before, frame 22 "
                     "the same as FX-PEMIT-028's frame 12.", case(in_frame=10, **FLOW), [10, 22]),
    "FX-PEMIT-046": ("FX-PEMIT-028 with the layer moved 3 pixels right.",
                     case(shift=3, **FLOW), [12]),
    "FX-PEMIT-047": ("Random Seed 7: a different burst.", case(**dict(FLOW, random_seed=7)), [12]),
    "FX-PEMIT-048": ("Particles Per Second 0: nothing.", case(particles_per_second=0), [12]),
    "FX-PEMIT-049": ("Life 0: nothing.", case(life=0), [12]),
    "FX-PEMIT-050": ("Streaks on the burst: each a short line back along its last frame.",
                     case(**dict(FLOW, particle_type="streak", size=1.5)), [12]),
    "FX-PEMIT-051": ("A burst of glowing sparks: Over Life colours, shrinking and fading, added.",
                     case(**dict(FLOW, particle_type="glow_sphere", set_color="over_life",
                                 size=5, size_over_life_5=20, opacity_over_life_5=0,
                                 transfer_mode="add")), [12]),
}
FRAMES_OF = {fx: frames for fx, (_, _, frames) in CASES.items()}
assert all(in_range(c) for _, c, _ in CASES.values())

INVALID = {
    "FX-PEMIT-052": ("Particles Per Second -1, below 0.", case(particles_per_second=-1)),
    "FX-PEMIT-053": ("Life 1001, above 1000.", case(life=1001)),
    "FX-PEMIT-054": ("Size Over Life 3 at 1001, above 1000.", case(size_over_life_3=1001)),
    "FX-PEMIT-055": ("Opacity Over Life 2 at 101, above 100.", case(opacity_over_life_2=101)),
    "FX-PEMIT-056": ("Emitter Type \"light\": not built.", case(emitter_type="light")),
    "FX-PEMIT-057": ("Direction \"Uniform\": the word is exact.", case(direction="Uniform")),
    "FX-PEMIT-058": ("Particle Type \"sprite\": not one of the five.", case(particle_type="sprite")),
    "FX-PEMIT-059": ("Set Color \"from_layer\".", case(set_color="from_layer")),
    "FX-PEMIT-060": ("Transfer Mode \"lighten\".", case(transfer_mode="lighten")),
    "FX-PEMIT-061": ("Floor Action \"collide\".", case(floor_action="collide")),
    "FX-PEMIT-062": ("Gradient Color 2 \"#12345\": not a colour.",
                     case(gradient_color_2="#12345")),
    "FX-PEMIT-063": ("Grid X 0, below 1.", case(grid_x=0)),
}

LISTED = [("FX-PEMIT-016", 20), ("FX-PEMIT-021", 20), ("FX-PEMIT-022", 20),
          ("FX-PEMIT-023", 20), ("FX-PEMIT-028", 12), ("FX-PEMIT-029", 24),
          ("FX-PEMIT-030", 24), ("FX-PEMIT-031", 24), ("FX-PEMIT-032", 24),
          ("FX-PEMIT-033", 12), ("FX-PEMIT-034", 24), ("FX-PEMIT-035", 24),
          ("FX-PEMIT-036", 24), ("FX-PEMIT-037", 24), ("FX-PEMIT-038", 24),
          ("FX-PEMIT-039", 24), ("FX-PEMIT-040", 24), ("FX-PEMIT-041", 24),
          ("FX-PEMIT-042", 24), ("FX-PEMIT-043", 30)]


def listed():
    out = {"tolerance": 1e-7, "cases": {}}
    for fx, f in LISTED:
        run = Run(CASES[fx][1], f)
        rows = [[k, p["b"]] + p["x"] + p["v"] for k, p, _ in alive(run)]
        out["cases"][fx] = {"frame": f, "particles": rows}
        print(f"{fx} frame {f}: {len(rows)} particles listed")
    (OUT / "particles_particle_emitter.json").write_text(json.dumps(out, indent=1) + "\n",
                                                         encoding="utf-8")


def clear():
    """No number lands so near a cut that the build's rounding could put it on the other side:
    a birth near the frame drawn, an age near its life, a near miss of the floor, a pixel centre
    on a star's outline, or two overlapping particles at nearly but not quite the same depth."""
    P.NEAR.clear()
    for fx, (_, c, frames) in CASES.items():
        for f in frames:
            run = Run(c, f)
            if run.u < 0:
                continue
            for k, b in P.births(run):
                assert abs(b - run.u) > 1e-6, (fx, f, k)
                life = run.mine("life", b) * FPS * (1 - run.mine("life_random", b) / 100 * run.r(k, 11))
                assert abs((run.u - b) - life) > 1e-6, (fx, f, k)
            sps = sprites(run)
            for i in range(len(sps)):
                for j in range(i + 1, len(sps)):
                    a, b = sps[i][2], sps[j][2]
                    if "R" not in a or "R" not in b:
                        continue
                    near = math.hypot(a["X"] - b["X"], a["Y"] - b["Y"]) < a["R"] + b["R"] + 2
                    gap = abs(sps[i][0] - sps[j][0])
                    assert not (near and 0 < gap < 1e-7), (fx, f, i, j, gap)
            for _, _, sp, _ in sps:
                if sp["kind"] == "star":
                    for gy in range(H):
                        for gx in range(W):
                            dy = gy + 0.5 - sp["Y"]
                            for sx, sy in PL.STAR:
                                assert abs(sy * sp["R"] - dy) > 1e-9, (fx, gx, gy)
    if P.NEAR:
        assert min(P.NEAR) > 1e-9, min(P.NEAR)
        print(f"clear: nearest miss of the floor {min(P.NEAR):.3g}")
    else:
        print("clear")


def check(expected):
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    clear_frame = [[0.0] * 4 for _ in range(W * H)]
    total = lambda fr: sum(p[3] for p in fr)  # noqa: E731
    assert c["FX-PEMIT-001"]["0"] == clear_frame
    assert total(c["FX-PEMIT-001"]["6"]) > 0
    one = c["FX-PEMIT-002"]["20"]
    assert one[5 * W + 8][3] == 1.0
    hard, soft = c["FX-PEMIT-003"]["20"], c["FX-PEMIT-004"]["20"]
    assert hard != one != soft and hard[5 * W + 8] == one[5 * W + 8]
    for n in (5, 6, 7, 9, 10, 12):
        assert total(c["FX-PEMIT-%03d" % n]["20"]) < total(one), n
    assert total(c["FX-PEMIT-011"]["20"]) < total(one)
    assert c["FX-PEMIT-013"]["20"] != c["FX-PEMIT-002"]["20"]
    assert c["FX-PEMIT-016"]["40"] != c["FX-PEMIT-016"]["20"]
    blue = c["FX-PEMIT-017"]["20"][5 * W + 8]
    assert blue[2] > blue[0] and blue[2] > blue[1]
    comp, add, scr = (c["FX-PEMIT-%03d" % n]["20"] for n in (25, 26, 27))
    assert comp != add != scr != comp
    assert c["FX-PEMIT-028"]["40"] != c["FX-PEMIT-028"]["24"]
    for n in range(29, 44):
        assert c["FX-PEMIT-%03d" % n] != c["FX-PEMIT-028"], n
    assert c["FX-PEMIT-044"]["20"] != c["FX-PEMIT-021"]["20"]
    assert c["FX-PEMIT-045"]["10"] == clear_frame
    assert c["FX-PEMIT-045"]["22"] == c["FX-PEMIT-028"]["12"]
    moved_, base = c["FX-PEMIT-046"]["12"], c["FX-PEMIT-028"]["12"]
    for y in range(H):
        assert moved_[y * W + 3:(y + 1) * W] == base[y * W:y * W + W - 3]
    assert c["FX-PEMIT-047"]["12"] != base
    assert c["FX-PEMIT-048"]["12"] == clear_frame
    assert c["FX-PEMIT-049"]["12"] == clear_frame
    # the jet goes up; turned 90 about Z it goes right; bi-directional both ways
    run = Run(CASES["FX-PEMIT-029"][1], 24)
    assert all(born(run, k, b)["v"][1] < 0 for k, b in P.births(run))
    run = Run(CASES["FX-PEMIT-030"][1], 24)
    assert all(born(run, k, b)["v"][0] > 0 for k, b in P.births(run))
    run = Run(CASES["FX-PEMIT-031"][1], 24)
    ups = [born(run, k, b)["v"][1] < 0 for k, b in P.births(run)]
    assert any(ups) and not all(ups)
    # the disc turned to face us throws in the screen's plane
    run = Run(CASES["FX-PEMIT-032"][1], 24)
    assert all(abs(born(run, k, b)["v"][2]) < 1e-12 for k, b in P.births(run))
    # outwards moves away from the emitter's middle, inwards towards it
    for fx, sign in (("FX-PEMIT-033", 1), ("FX-PEMIT-034", -1)):
        run = Run(CASES[fx][1], 12)
        for k, b in P.births(run):
            p = born(run, k, b)
            d = P.sub(p["x"], run.vec("producer", b))
            assert sign * P.dot(d, p["v"]) > 0, fx
    # the grid fills point by point: 8 points, 20 dots at frame 20, each drawn two or three times
    run = Run(CASES["FX-PEMIT-022"][1], 20)
    places = {tuple(round(v, 9) for v in p["x"]) for _, p, _ in alive(run)}
    assert len(alive(run)) == 20 and len(places) == 8
    # the stuck ones lie on the floor
    run = Run(CASES["FX-PEMIT-041"][1], 24)
    floor = run.now["floor_position"]
    assert any(p["glued"] and p["x"][1] == floor for _, p, _ in alive(run))
    print("checked")


def main():
    clear()
    expected = B.write_cases(OUT, "particle_emitter", CASES, INVALID, render, plain, file_json)
    listed()
    check(expected)


if __name__ == "__main__":
    main()
