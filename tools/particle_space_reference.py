"""Particle Space, worked a second way.

`core.particle_space` (D-610), the first effect on the particle engine (P0-6), modelled on what
CycoreFX's CC Particle World is for: particles born from a producer, thrown out by one of eleven
animation modes, pulled by gravity, slowed by resistance, stopped or bounced by a floor, and
drawn as 3D points through a camera. Cycore publishes no maths, so every rule below is ours,
written after what each control is for (document 21, D-610). No text or code was copied.

**The engine's rule: no state is carried from frame to frame.** A particle's state at a frame is
a pure function of the seed, its number k and the settings history, so any frame drawn alone
equals the same frame played from the start (document 20).

The settings are SETTINGS (name, lowest, highest, value when added; all keyable), WORDS (name,
the words, the word when added; exact) and COLOURS (name, `#rrggbb` when added). With n the
composition frame, the layer's in point i0 and fps the composition's rate:

- Time. u = n - i0, in frames since the in point; seconds are frames / fps. Nothing is born
  before the in point. The settings are sampled once a frame, S[i] at frame i0 + i, for i = 0 to
  u + 1 (each held inside its range); between two frames a setting is read straight-line
  between its samples, before 0 as at 0 and past the last as the last.
- Randomness. r(k, s) = (unit(seed, k, s, 0, 0) + 1) / 2, a number from 0 up to 1, with unit
  tools/noise_reference.py's (the build's `grade::unit`) and seed the whole part of Random Seed
  at frame n. Streams: 0 spawn radius, 1 and 2 spawn direction, 3 and 4 the direction thrown,
  5 speed, 6 size, 9 twirl, 100 + 2m and 101 + 2m the m-th bounce.
- Births. q_i = 100 Birth Rate (S[i]) / fps particles a frame. B(t), the births up to t, is the
  integral of q read straight-line between frames; particle k (0 up) is born at b_k where
  B(b_k) = k + 1/2. When q is the same at every sample, b_k = (k + 1/2) / q. Otherwise frame by
  frame: B_0 = 0, B_(i+1) = B_i + (q_i + q_(i+1)) / 2, and the particles with
  B_i < k + 1/2 <= B_(i+1) are born at i + x, x = 2 D / (b + sqrt(b^2 + 4 a D)), D = k + 1/2 - B_i,
  a = (q_(i+1) - q_i) / 2, b = q_i (the root under the square root held at 0 or more, x at 1 or
  less).
- Life. Longevity at b_k, in seconds, fixed for the particle; it is alive at u when b_k <= u and
  u - b_k < Longevity fps.
- Spawn, read at b_k. The point rho (Rx dx, Ry dy, Rz dz) from the producer P, rho = cbrt(r0),
  d the unit vector with z = 1 - 2 r1 and turn phi = 2 pi r2 (x = sqrt(1 - z^2) cos phi,
  y = sqrt(1 - z^2) sin phi).
- Thrown, read at b_k. The axis A is Direction Axis made unit, or (0, -1, 0) when it is 0; for
  Fire it is the way up against gravity, minus Gravity Vector made unit, or (0, -1, 0). e1 = (h x
  A) made unit, h = (1, 0, 0) unless |A_x| >= 0.9, then (0, 1, 0); e2 = A x e1. alpha = Extra
  Angle / 2. The direction:
  Explosive, Viscous, Twirl, Twirly, Vortex, Fractal Omni: uniform, z = 1 - 2 r3, turn 2 pi r4;
  Direction Axis, Fractal Uni: within alpha of A, cos t = 1 - r3 (1 - cos alpha), turn 2 pi r4,
  the direction cos t A + sin t (cos phi e1 + sin phi e2); Cone Axis: on the cone, t = alpha;
  Fire: within 20 degrees of A as Direction Axis; Jet Sideways: cos phi e1 + sin phi e2.
  The speed Velocity (1 + Extra (r5 - 1/2)); then Inherit Velocity / 100 times the producer's
  velocity, (P(b + 1/8) - P(b - 1/8)) / (1/4 frame).
- Steps. On one grid of 4 a frame from the in point, the j-th boundary at j / 4: a particle
  steps from b_k to the first boundary after it, then boundary to boundary up to u. In a step
  from t0, h seconds long, with the settings at t0:
  - a push, applied as a change of velocity push x h at once (the Y part left out on ice). With
    D = x - P and Dp = D - (D . A) A its part square to the axis through the producer:
    Twirl 2 V (A x Dp) / (|Dp|^2 + 0.01), V the Velocity setting; Twirly that times 2 r9;
    Vortex Twirl's minus 2 V Dp; Fire minus 4 Dp. Then the turbulence, strength Turbulence
    (Fire 0.5 more, Fractal Omni and Uni 1 more): strength times the curl of the noise field
    below at q = x / Turbulence Scale + Turbulence Evolution (0.31, 0.57, 0.77), each derivative
    by central differences 1/1000 apart in q.
  - then the pull a = G g + Wind (Fire -G g + Wind), g Gravity Vector made unit (0 when it is
    0), with the resistance k = Resistance (Viscous 3 more), in closed form:
    k > 0: v' = a / k + (v - a / k) e^(-k h), x' = x + (a / k) h + (v - a / k)(-expm1(-k h)) / k;
    k = 0: v' = v + a h, x' = x + v h + a h^2 / 2. On ice the Y parts of a are 0.
  - the floor, Y = Floor Position, unless None: a particle that starts the move with y <= F and
    ends it with y > F crossed it. The crossing time from 30 halvings of the move (low 0, high
    the move's length; the middle replaces high when y there is past F, else low; the crossing
    is high). There the particle is put on the floor (y = F) with the closed form's velocity, and
    Glue: stops for good. Ice: the Y velocity 0 and its Y motion gone for life, sliding on for
    the rest of the move. Bounce: the Y velocity times -max(0, Bounciness / 100 + Random
    Bounciness / 100 (2 r(100 + 2m) - 1)), the X and Z velocity turned about Y by Bounce Spread
    / 100 pi (2 r(101 + 2m) - 1), m the particle's bounces so far; then the rest of the move. After
    4 bounces in one move the particle rests on the floor for the rest of it.
- The noise field, channels c = 0, 1, 2: Perlin's gradient noise, at each lattice corner (i, j, l)
  the unit gradient with z = unit(seed, i, j, l, 200 + 2c) and turn pi unit(seed, i, j, l,
  201 + 2c); the eight corners' dot products mixed by t^3 (t (6 t - 15) + 10).
- Look, read at b_k: Birth Size, Death Size and Size Variation. The size at age fraction f =
  age / life is (Birth + (Death - Birth) f)(1 + Size Variation / 100 (2 r6 - 1)), in hundredths of
  the layer's width W (the world's unit is W, its origin the layer's centre, Y down, Z away).
  The colour along the Color Map at f, worked in linear light: Birth to Death, Origin to Death,
  Birth to Origin, Origin Constant, or Custom's five colours at 0, 1/4, 1/2, 3/4 and 1. Origin
  is the input's straight colour under the spawn point, (W/2 + x W, H/2 + y W), whole pixels;
  black where the input is clear or outside.
- The camera, at frame n. With the composition's camera (the file has one): the camera is
  given in the layer's own pixels, a its place and A_l its distance from the layer's plane
  (compose works these out). A particle at x_px = (W/2 + x W, H/2 + y W), z_px = z W is drawn at
  a + (x_px - a) A_l / (A_l + z_px), one unit W A_l / (A_l + z_px) pixels across there, and at
  depth (A_l + z_px) / W; not drawn when that depth is 0.01 or less. Otherwise the effect's own:
  the world turned by Rx(Rotation X) Ry(Rotation Y) Rz(Rotation Z) (Rz first) and moved Distance
  away, c = M x + (0, 0, Distance); drawn at (W/2 + f c_x / c_z, H/2 + f c_y / c_z), f = (W/2) /
  tan(FOV / 2), one unit f / c_z pixels across, at depth c_z; not drawn when c_z <= 0.01.
- Drawing. The radius R is half the size's pixels. Opacity Max Opacity / 100 times the Opacity
  Map at f (D-837): Constant 1; Fade In And Out straight-line between the FADE points below, CC Particle World's
  map as added as two tutorials draw it. A file without the word is Constant. Depth Cue Fade
  times it by clamp(1 - depth / distance, 0, 1), Fog mixes the colour towards its colour by 1
  minus that. Line: from the particle to the point x - v (1 / fps)(size / 0.15), width
  w = max(1, 2 R), covering clamp(w / 2 - e + 1/2, 0, 1), e the pixel centre's distance from that
  segment. The others, when R < 0.7, are drawn with R = 0.7 and the opacity times (R / 0.7)^2;
  with e the distance from the centre: Disc and Shaded Sphere clamp(R - e + 1/2, 0, 1), Faded
  Sphere max(0, 1 - (e / R)^2)^2, Bubble the disc's times 0.25 + 0.75 min(1, e / R)^4. Lit, the
  colour times 1 - v + v (0.2 + 0.8 max(0, n . L)), v = 1 for Shaded Sphere and Bubble and
  Volume Shade / 100 for Disc and Faded Sphere; n = (dx, dy, -sqrt(1 - q)) at
  (dx, dy) = (pixel - centre) / R, q = dx^2 + dy^2 < 1, else (dx, dy, 0) / sqrt(q); towards the
  light L = (cos h sin d, -cos h cos d, -sin h), d Light Direction, h Light Height. Line is
  never lit.
- Order and transfer. Every pixel starts clear, or the input with Composite With Original on;
  the particles go on farthest first (equal depths by number): Composite s + (1 - s_a) d,
  Screen s + d - s d on all four, Add the colours summed and the coverings summed held at 1.

**This file never runs the build's code path.** It works in double precision on lists from the
drawing's 8-bit values; trigonometry is the platform's.

The drawing is Broadcast Colors' (tools/broadcast_safe_reference.py), 16 by 10. The projects go
into `Fixtures/particle_space`, the expected frames into
`Fixtures/particle_space/expected_particle_space.json`, and each alive particle's numbers for
some cases into `Fixtures/particle_space/particles_particle_space.json`. Every case is "not
built yet" until B-490 lands (D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/particle_space_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from noise_reference import u as U  # noqa: E402
from adjust_reference import srgb_to_linear, prop  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402

W, H = B.W, B.H
FPS = 24
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "particle_space"

SETTINGS = [
    ("birth_rate", 0, 100, 2), ("longevity", 0, 30, 1),
    ("producer_x", -10, 10, 0), ("producer_y", -10, 10, 0), ("producer_z", -10, 10, 0),
    ("radius_x", 0, 10, 0.025), ("radius_y", 0, 10, 0.025), ("radius_z", 0, 10, 0.025),
    ("velocity", -10, 10, 1), ("inherit_velocity", -10000, 10000, 0),
    ("gravity", -10, 10, 0.5), ("resistance", 0, 100, 0),
    ("axis_x", -1, 1, 0), ("axis_y", -1, 1, -1), ("axis_z", -1, 1, 0),
    ("extra", 0, 10, 0.5), ("extra_angle", 0, 360, 360),
    ("gravity_x", -1, 1, 0), ("gravity_y", -1, 1, 1), ("gravity_z", -1, 1, 0),
    ("floor_position", -10, 10, 0.5), ("bounciness", 0, 100, 50),
    ("random_bounciness", 0, 100, 0), ("bounce_spread", 0, 100, 0),
    ("wind_x", -10, 10, 0), ("wind_y", -10, 10, 0), ("wind_z", -10, 10, 0),
    ("turbulence", 0, 10, 0), ("turbulence_scale", 0.01, 10, 0.25),
    ("turbulence_evolution", -100000, 100000, 0),
    ("birth_size", 0, 10, 0.15), ("death_size", 0, 10, 0.15), ("size_variation", 0, 100, 0),
    ("max_opacity", 0, 100, 75), ("volume_shade", 0, 100, 0),
    ("camera_distance", 0.1, 100, 2), ("camera_rotation_x", -36000, 36000, 0),
    ("camera_rotation_y", -36000, 36000, 0), ("camera_rotation_z", -36000, 36000, 0),
    ("camera_fov", 1, 170, 50), ("depth_cue_distance", 0.01, 100, 4),
    ("light_direction", -36000, 36000, 45), ("light_height", -90, 90, 45),
    ("random_seed", 0, 10000, 0),
]
NAMES = [s[0] for s in SETTINGS]
RANGES = {s[0]: (s[1], s[2]) for s in SETTINGS}
ANIMATIONS = ("explosive", "direction_axis", "cone_axis", "viscous", "twirl", "twirly", "vortex",
              "fire", "jet_sideways", "fractal_omni", "fractal_uni")
WORDS = [
    ("animation", ANIMATIONS, "explosive"),
    ("floor_action", ("none", "ice", "glue", "bounce"), "none"),
    ("particle_type", ("line", "faded_sphere", "shaded_sphere", "disc", "bubble"), "line"),
    ("color_map", ("birth_to_death", "origin_to_death", "birth_to_origin", "origin_constant",
                   "custom"), "birth_to_death"),
    ("transfer_mode", ("composite", "screen", "add"), "composite"),
    ("depth_cue", ("none", "fade", "fog"), "none"),
    ("composite_on_original", ("off", "on"), "off"),
    # D-837: added as Fade In And Out; a case leaves it Constant, and a file without it is.
    ("opacity_map", ("constant", "fade_in_and_out"), "fade_in_and_out"),
]
FADE = [(0, 0), (0.025, 0.24), (0.05, 0.41), (0.075, 0.55), (0.1, 0.67), (0.15, 0.84), (0.2, 0.94), (0.25, 0.98), (0.3, 1), (0.7, 1), (0.75, 0.98), (0.8, 0.94), (0.85, 0.84), (0.9, 0.67), (0.925, 0.55), (0.95, 0.41), (0.975, 0.24), (1, 0)]


def opacity_map(c, f):
    if c["opacity_map"] == "constant":
        return 1.0
    for (f0, o0), (f1, o1) in zip(FADE, FADE[1:]):
        if f <= f1:
            return o0 + (o1 - o0) * (f - f0) / (f1 - f0)
    return 0.0
COLOURS = [
    ("birth_color", "#ffd933"), ("death_color", "#cc1a00"),
    # Custom's five run from the birth colour to the death colour, a quarter of the 8-bit way
    # each (D-610: "birth .. death").
    ("custom_color_1", "#ffd933"), ("custom_color_2", "#f2a926"), ("custom_color_3", "#e67a1a"),
    ("custom_color_4", "#d94a0d"), ("custom_color_5", "#cc1a00"),
    ("depth_cue_color", "#000000"),
]


# --- small sums -----------------------------------------------------------------------------

def add(a, b):
    return [a[0] + b[0], a[1] + b[1], a[2] + b[2]]


def sub(a, b):
    return [a[0] - b[0], a[1] - b[1], a[2] - b[2]]


def mul(a, s):
    return [a[0] * s, a[1] * s, a[2] * s]


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def unit_or(v, fallback):
    n = math.sqrt(dot(v, v))
    return [c / n for c in v] if n > 0 else list(fallback)


def clamp(v, lo=0.0, hi=1.0):
    return min(hi, max(lo, v))


def linear(hex_colour):
    return [srgb_to_linear(v / 255) for v in R.hex_color(hex_colour)]


# --- the run: one frame's settings history --------------------------------------------------

class Run:
    def __init__(self, c, n):
        self.c = c
        self.u = n - c["in_frame"]
        self.samples = [settings(c, c["in_frame"] + i) for i in range(self.u + 2)]
        now = self.samples[self.u]
        self.seed = math.floor(now["random_seed"])
        self.now = now

    def at(self, name, t):
        s = self.samples
        last = len(s) - 1
        i0 = math.floor(t)
        if i0 < 0:
            return s[0][name]
        if i0 >= last:
            return s[last][name]
        a, b = s[i0][name], s[i0 + 1][name]
        return a + (b - a) * (t - i0)

    def vec(self, prefix, t):
        return [self.at(prefix + a, t) for a in ("_x", "_y", "_z")]

    def r(self, k, stream):
        return (U(self.seed, k, stream, 0, 0) + 1) / 2


def settings(c, frame_no):
    k = {}
    for n in NAMES:
        lo, hi = RANGES[n]
        k[n] = clamp(value_at(c[n], frame_no), lo, hi)
    return k


# --- births ---------------------------------------------------------------------------------

def births(run):
    """Every (k, b_k) with b_k <= u that may be alive, oldest first."""
    q = [100 * s["birth_rate"] / FPS for s in run.samples]
    lmax = max(s["longevity"] for s in run.samples) * FPS
    u = run.u
    out = []
    if all(v == q[0] for v in q):
        if q[0] <= 0:
            return out
        lo = max(0, math.floor(q[0] * (u - lmax) - 0.5))
        hi = math.floor(q[0] * u - 0.5) + 1
        for k in range(lo, hi + 1):
            b = (k + 0.5) / q[0]
            if b <= u:
                out.append((k, b))
        return out
    total = 0.0
    for i in range(u):
        nxt = total + (q[i] + q[i + 1]) / 2
        if i + 1 >= u - lmax:
            a, b = (q[i + 1] - q[i]) / 2, q[i]
            k = math.floor(total - 0.5) + 1
            while k + 0.5 <= nxt:
                d = k + 0.5 - total
                den = b + math.sqrt(max(0.0, b * b + 4 * a * d))
                x = min(1.0, 2 * d / den) if den > 0 else 1.0
                out.append((k, i + x))
                k += 1
        total = nxt
    return out


# --- birth: where, which way, how fast ------------------------------------------------------

def axes(run, t):
    if run.c["animation"] == "fire":
        return [-v for v in unit_or(run.vec("gravity", t), (0, -1, 0))] \
            if any(run.vec("gravity", t)) else [0.0, -1.0, 0.0]
    return unit_or(run.vec("axis", t), (0, -1, 0))


def basis(a):
    h = [0.0, 1.0, 0.0] if abs(a[0]) >= 0.9 else [1.0, 0.0, 0.0]
    e1 = unit_or(cross(h, a), (1, 0, 0))
    return e1, cross(a, e1)


def sphere(z, turn):
    s = math.sqrt(max(0.0, 1 - z * z))
    return [s * math.cos(turn), s * math.sin(turn), z]


def born(run, k, b):
    """The particle's start: place, velocity and its fixed numbers."""
    r = lambda s: run.r(k, s)  # noqa: E731
    p = run.vec("producer", b)
    rad = run.vec("radius", b)
    rho = r(0) ** (1 / 3)
    d = sphere(1 - 2 * r(1), 2 * math.pi * r(2))
    x = [p[i] + rad[i] * rho * d[i] for i in range(3)]
    mode = run.c["animation"]
    a = axes(run, b)
    e1, e2 = basis(a)
    phi = 2 * math.pi * r(4)

    def cap(cos_t):
        sin_t = math.sqrt(max(0.0, 1 - cos_t * cos_t))
        return add(mul(a, cos_t), mul(add(mul(e1, math.cos(phi)), mul(e2, math.sin(phi))), sin_t))

    alpha = math.radians(run.at("extra_angle", b) / 2)
    if mode in ("direction_axis", "fractal_uni"):
        way = cap(1 - r(3) * (1 - math.cos(alpha)))
    elif mode == "cone_axis":
        way = cap(math.cos(alpha))
    elif mode == "fire":
        way = cap(1 - r(3) * (1 - math.cos(math.radians(20))))
    elif mode == "jet_sideways":
        way = add(mul(e1, math.cos(phi)), mul(e2, math.sin(phi)))
    else:
        way = sphere(1 - 2 * r(3), phi)
    speed = run.at("velocity", b) * (1 + run.at("extra", b) * (r(5) - 0.5))
    v = mul(way, speed)
    iv = run.at("inherit_velocity", b)
    if iv != 0:
        pv = mul(sub(run.vec("producer", b + 0.125), run.vec("producer", b - 0.125)), FPS / 0.25)
        v = add(v, mul(pv, iv / 100))
    sv = run.at("size_variation", b) / 100
    return {
        "k": k, "b": b, "x": x, "v": v, "x0": list(x),
        "life": run.at("longevity", b) * FPS,
        "sizes": (run.at("birth_size", b), run.at("death_size", b)),
        "size_factor": 1 + sv * (2 * r(6) - 1),
        "twirl": 2 * r(9),
        "glued": False, "iced": False, "bounces": 0,
    }


# --- the noise field ------------------------------------------------------------------------

def fade(t):
    return t * t * t * (t * (6 * t - 15) + 10)


def noise(seed, c, q):
    i = [math.floor(v) for v in q]
    f = [q[n] - i[n] for n in range(3)]
    w = [fade(v) for v in f]
    total = 0.0
    for dz in (0, 1):
        for dy in (0, 1):
            for dx in (0, 1):
                ci = (i[0] + dx, i[1] + dy, i[2] + dz)
                g = sphere(U(seed, ci[0], ci[1], ci[2], 200 + 2 * c),
                           math.pi * U(seed, ci[0], ci[1], ci[2], 201 + 2 * c))
                wt = ((w[0] if dx else 1 - w[0]) * (w[1] if dy else 1 - w[1])
                      * (w[2] if dz else 1 - w[2]))
                total += wt * dot(g, [f[0] - dx, f[1] - dy, f[2] - dz])
    return total


def curl(seed, q):
    e = 1e-3

    def d(c, axis):
        hi, lo = list(q), list(q)
        hi[axis] += e
        lo[axis] -= e
        return (noise(seed, c, hi) - noise(seed, c, lo)) / (2 * e)

    return [d(2, 1) - d(1, 2), d(0, 2) - d(2, 0), d(1, 0) - d(0, 1)]


# --- a step ---------------------------------------------------------------------------------

def closed(x, v, a, k, h):
    if k > 0:
        e = math.exp(-k * h)
        m = -math.expm1(-k * h) / k
        return ([x[i] + a[i] / k * h + (v[i] - a[i] / k) * m for i in range(3)],
                [a[i] / k + (v[i] - a[i] / k) * e for i in range(3)])
    return ([x[i] + v[i] * h + a[i] * h * h / 2 for i in range(3)],
            [v[i] + a[i] * h for i in range(3)])


NEAR = []  # how near each move's ends came to the floor without touching it, for clear()


def step(run, p, t0, h):
    if p["glued"]:
        return
    mode = run.c["animation"]
    at = lambda n: run.at(n, t0)  # noqa: E731
    x, v = p["x"], p["v"]
    push = [0.0, 0.0, 0.0]
    if mode in ("twirl", "twirly", "vortex", "fire"):
        a = axes(run, t0)
        dd = sub(x, run.vec("producer", t0))
        dp = sub(dd, mul(a, dot(dd, a)))
        vel = at("velocity")
        if mode == "fire":
            push = mul(dp, -4)
        else:
            push = mul(cross(a, dp), 2 * vel / (dot(dp, dp) + 0.01))
            if mode == "twirly":
                push = mul(push, p["twirl"])
            if mode == "vortex":
                push = add(push, mul(dp, -2 * vel))
    strength = at("turbulence") + {"fire": 0.5, "fractal_omni": 1, "fractal_uni": 1}.get(mode, 0)
    if strength > 0:
        evo = at("turbulence_evolution")
        sc = at("turbulence_scale")
        q = [x[0] / sc + evo * 0.31, x[1] / sc + evo * 0.57, x[2] / sc + evo * 0.77]
        push = add(push, mul(curl(run.seed, q), strength))
    if p["iced"]:
        push[1] = 0.0
    v = add(v, mul(push, h))
    g = run.vec("gravity", t0)
    gu = unit_or(g, (0, 0, 0)) if any(g) else [0.0, 0.0, 0.0]
    a = add(mul(gu, -at("gravity") if mode == "fire" else at("gravity")), run.vec("wind", t0))
    k = at("resistance") + (3 if mode == "viscous" else 0)
    if p["iced"]:
        a[1] = 0.0
    action = run.c["floor_action"]
    floor = at("floor_position")
    rem = h
    hops = 0
    while True:
        x1, v1 = closed(x, v, a, k, rem)
        if action != "none" and x1[1] != floor and x[1] != floor:
            NEAR.append(min(abs(x1[1] - floor), abs(x[1] - floor)))
        if action == "none" or p["iced"] or not (x[1] <= floor < x1[1]):
            p["x"], p["v"] = x1, v1
            return
        lo, hi = 0.0, rem
        for _ in range(30):
            mid = (lo + hi) / 2
            if closed(x, v, a, k, mid)[0][1] > floor:
                hi = mid
            else:
                lo = mid
        xc, vc = closed(x, v, a, k, hi)
        xc[1] = floor
        rem -= hi
        if action == "glue":
            p["x"], p["v"], p["glued"] = xc, [0.0, 0.0, 0.0], True
            return
        if action == "ice":
            vc[1] = 0.0
            a[1] = 0.0
            p["iced"] = True
            x, v = xc, vc
            continue
        m = p["bounces"]
        bounce = max(0.0, at("bounciness") / 100
                     + at("random_bounciness") / 100 * (2 * run.r(p["k"], 100 + 2 * m) - 1))
        turn = at("bounce_spread") / 100 * math.pi * (2 * run.r(p["k"], 101 + 2 * m) - 1)
        cs, sn = math.cos(turn), math.sin(turn)
        vc = [vc[0] * cs - vc[2] * sn, -vc[1] * bounce, vc[0] * sn + vc[2] * cs]
        p["bounces"] += 1
        hops += 1
        x, v = xc, vc
        if hops == 4:
            v[1] = 0.0
            flat = [a[0], 0.0, a[2]]
            p["x"], p["v"] = closed(x, v, flat, k, rem)
            return


def moved(run, k, b):
    """Particle k, born at b, as it is at u."""
    p = born(run, k, b)
    u = run.u
    points = [b] + [j / 4 for j in range(math.floor(4 * b) + 1, 4 * u + 1)]
    for t0, t1 in zip(points, points[1:]):
        step(run, p, t0, (t1 - t0) / FPS)
    return p


# --- the camera and the drawing -------------------------------------------------------------

def rx(a):
    c, s = math.cos(a), math.sin(a)
    return [[1, 0, 0], [0, c, -s], [0, s, c]]


def ry(a):
    c, s = math.cos(a), math.sin(a)
    return [[c, 0, s], [0, 1, 0], [-s, 0, c]]


def rz(a):
    c, s = math.cos(a), math.sin(a)
    return [[c, -s, 0], [s, c, 0], [0, 0, 1]]


def mat(a, b):
    return [[sum(a[i][k] * b[k][j] for k in range(3)) for j in range(3)] for i in range(3)]


def apply(m, v):
    return [m[i][0] * v[0] + m[i][1] * v[1] + m[i][2] * v[2] for i in range(3)]


def camera(run, view):
    """A function from a world point to (X, Y, pixels a unit, depth), or None when not drawn."""
    now = run.now
    if view is not None:
        ax, ay, al = view

        def through(x):
            zp = x[2] * W
            depth = (al + zp) / W
            if al <= 0 or depth <= 0.01:
                return None
            ratio = al / (al + zp)
            return (ax + (W / 2 + x[0] * W - ax) * ratio, ay + (H / 2 + x[1] * W - ay) * ratio,
                    W * ratio, depth)
        return through
    m = mat(rx(math.radians(now["camera_rotation_x"])),
            mat(ry(math.radians(now["camera_rotation_y"])),
                rz(math.radians(now["camera_rotation_z"]))))
    f = (W / 2) / math.tan(math.radians(now["camera_fov"]) / 2)
    dist = now["camera_distance"]

    def own(x):
        c = apply(m, x)
        cz = c[2] + dist
        if cz <= 0.01:
            return None
        return (W / 2 + f * c[0] / cz, H / 2 + f * c[1] / cz, f / cz, cz)
    return own


def origin(px, x0):
    gx, gy = math.floor(W / 2 + x0[0] * W), math.floor(H / 2 + x0[1] * W)
    if not (0 <= gx < W and 0 <= gy < H):
        return [0.0, 0.0, 0.0]
    p = px[gy * W + gx]
    return [p[i] / p[3] for i in range(3)] if p[3] > 0 else [0.0, 0.0, 0.0]


def mix3(a, b, t):
    return [a[i] + (b[i] - a[i]) * t for i in range(3)]


def colour(c, f, org):
    cm = c["color_map"]
    birth, death = linear(c["birth_color"]), linear(c["death_color"])
    if cm == "birth_to_death":
        return mix3(birth, death, f)
    if cm == "origin_to_death":
        return mix3(org, death, f)
    if cm == "birth_to_origin":
        return mix3(birth, org, f)
    if cm == "origin_constant":
        return list(org)
    stops = [linear(c["custom_color_%d" % i]) for i in range(1, 6)]
    i = min(math.floor(f * 4), 3)
    return mix3(stops[i], stops[i + 1], f * 4 - i)


def sprites(run, px, view):
    """Every particle drawn, farthest first: (depth, k, sprite)."""
    c, now = run.c, run.now
    see = camera(run, view)
    out = []
    for k, b in births(run):
        age = run.u - b
        life = run.at("longevity", b) * FPS
        if not age < life:
            continue
        p = moved(run, k, b)
        f = age / life
        bs, ds = p["sizes"]
        size = (bs + (ds - bs) * f) * p["size_factor"]
        s = see(p["x"])
        if s is None:
            continue
        X, Y, ppu, depth = s
        rad = size / 100 * ppu / 2
        col = colour(c, f, origin(px, p["x0"]))
        alpha = now["max_opacity"] / 100 * opacity_map(c, f)
        if c["depth_cue"] != "none":
            fd = clamp(1 - depth / now["depth_cue_distance"])
            if c["depth_cue"] == "fade":
                alpha *= fd
            else:
                col = mix3(col, linear(c["depth_cue_color"]), 1 - fd)
        kind = c["particle_type"]
        sp = {"kind": kind, "X": X, "Y": Y, "col": col}
        if kind == "line":
            tail = see(sub(p["x"], mul(p["v"], size / 0.15 / FPS)))
            if tail is None:
                continue
            sp.update(X2=tail[0], Y2=tail[1], w=max(1.0, 2 * rad), alpha=alpha)
        else:
            if rad < 0.7:
                alpha *= (rad / 0.7) ** 2
                rad = 0.7
            sp.update(R=rad, alpha=alpha)
        out.append((depth, k, sp, p))
    out.sort(key=lambda e: (-e[0], e[1]))
    return out


def light(now):
    d, h = math.radians(now["light_direction"]), math.radians(now["light_height"])
    return [math.cos(h) * math.sin(d), -math.cos(h) * math.cos(d), -math.sin(h)]


def cover(sp, x, y, vs, lt):
    """The sprite's premultiplied colour at the pixel centre (x, y), or None."""
    kind = sp["kind"]
    if kind == "line":
        ax, ay, bx, by = sp["X"], sp["Y"], sp["X2"], sp["Y2"]
        dx, dy = bx - ax, by - ay
        ll = dx * dx + dy * dy
        t = clamp(((x - ax) * dx + (y - ay) * dy) / ll) if ll > 0 else 0.0
        e = math.hypot(x - ax - t * dx, y - ay - t * dy)
        cv = clamp(sp["w"] / 2 - e + 0.5)
        shade = 1.0
    else:
        rad = sp["R"]
        e = math.hypot(x - sp["X"], y - sp["Y"])
        if kind == "faded_sphere":
            cv = max(0.0, 1 - (e / rad) ** 2) ** 2
        else:
            cv = clamp(rad - e + 0.5)
            if kind == "bubble":
                cv *= 0.25 + 0.75 * min(1.0, e / rad) ** 4
        v = 1.0 if kind in ("shaded_sphere", "bubble") else vs
        shade = 1.0
        if v > 0:
            dx, dy = (x - sp["X"]) / rad, (y - sp["Y"]) / rad
            q = dx * dx + dy * dy
            n = [dx, dy, -math.sqrt(1 - q)] if q < 1 else [dx / math.sqrt(q), dy / math.sqrt(q), 0]
            shade = 1 - v + v * (0.2 + 0.8 * max(0.0, dot(n, lt)))
    if cv <= 0:
        return None
    a = sp["alpha"] * cv
    return [sp["col"][i] * shade * a for i in range(3)] + [a]


def particle_space(px, c, n, view):
    run = Run(c, n)
    if run.u < 0:
        return [list(p) for p in px]
    sps = sprites(run, px, view)
    out = [list(p) if c["composite_on_original"] == "on" else [0.0] * 4 for p in px]
    vs = run.now["volume_shade"] / 100
    lt = light(run.now)
    mode = c["transfer_mode"]
    for _, _, sp, _ in sps:
        if sp["kind"] == "line":
            r = sp["w"] / 2 + 1
            xs = (min(sp["X"], sp["X2"]) - r, max(sp["X"], sp["X2"]) + r)
            ys = (min(sp["Y"], sp["Y2"]) - r, max(sp["Y"], sp["Y2"]) + r)
        else:
            r = sp["R"] + 1
            xs, ys = (sp["X"] - r, sp["X"] + r), (sp["Y"] - r, sp["Y"] + r)
        for gy in range(max(0, math.floor(ys[0])), min(H, math.ceil(ys[1]) + 1)):
            for gx in range(max(0, math.floor(xs[0])), min(W, math.ceil(xs[1]) + 1)):
                s = cover(sp, gx + 0.5, gy + 0.5, vs, lt)
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


def working():
    return [R.working(p) for p in B.pixels()]


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, camera=None, in_frame=0, **given):
    """Every setting as added, then the case's; `camera` is (x, y, depth, zoom) for a
    composition camera whose plane at depth 0 is drawn 1:1."""
    c = {s[0]: s[3] for s in SETTINGS}
    c.update({w[0]: w[2] for w in WORDS})
    c["opacity_map"] = "constant"
    c.update({k: v for k, v in COLOURS})
    for k, v in given.items():
        assert k in c, k
        c[k] = v
    c.update(shift=shift, camera=camera, in_frame=in_frame)
    return c


def in_range(c):
    """Every number, and every key of a keyed one, inside its range: the build refuses a file
    that is not (the INVALID cases), so a valid case must not lean on the hold."""
    for n, lo, hi, _ in SETTINGS:
        v = c[n]
        for x in ([k[1] for k in v["keys"]] if isinstance(v, dict) else [v]):
            if not lo <= x <= hi:
                return False
    return True


def view_of(c):
    """What compose works out for the composition camera, in the layer's own pixels."""
    if c["camera"] is None:
        return None
    x, y, depth, zoom = c["camera"]
    assert zoom == -depth, "only cameras that draw the layer's plane 1:1"
    return (x - c["shift"], y, 0 - depth)


def render(c, frame_no):
    px = working()
    out = particle_space(px, c, frame_no, view_of(c))
    move = c["shift"]
    if c["camera"] is not None:
        assert c["camera"][1] == H / 2 and c["camera"][0] == int(c["camera"][0])
        move += int(W / 2 - c["camera"][0])
    return R.frame(out, move)


def plain(c):
    move = c["shift"]
    if c["camera"] is not None:
        move += int(W / 2 - c["camera"][0])
    return R.frame(working(), move)


def file_json(fx, c):
    params = {n: setting_json(c[n]) for n in NAMES}
    params.update({w[0]: c[w[0]] for w in WORDS if w[0] != "opacity_map" or c[w[0]] != "constant"})
    params.update({k: c[k] for k, _ in COLOURS})
    p = B.project_json(fx, "core.particle_space", params, c["shift"])
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


# A close look: one particle, still, seen through a 10 degree lens 2 units away, so a size of 10
# is about 4.6 pixels across. 1.3 particles a second: the first born at frame 9.23, the next at
# 27.7, so frame 20 shows the first alone at age fraction 10.77 / 48.
ONE = dict(birth_rate=0.013, longevity=2, velocity=0, gravity=0, radius_x=0, radius_y=0,
           radius_z=0, camera_fov=10, birth_size=10, death_size=10, max_opacity=100)
# Origin colours: seen from 5.715 units so the layer's plane is drawn nearly 1:1 and the dot,
# 1.6 pixels across, sits over the pixel it took its colour from.
ORIGIN = dict(ONE, particle_type="disc", camera_distance=5.715)
# Two: 2.6 a second, born at frames 4.62 and 13.85, apart in x and z.
TWO = dict(ONE, birth_rate=0.026, radius_x=0.05, radius_z=0.05, particle_type="disc")
# A burst to watch move: a hundred a second, slower, larger discs through a 30 degree lens.
FLOW = dict(birth_rate=1, longevity=1.5, velocity=0.6, camera_fov=30, birth_size=8,
            death_size=3, particle_type="disc", max_opacity=90)
# Turbulence is slow to work by hand: six a second.
SLOW = dict(FLOW, birth_rate=0.06, longevity=2, birth_size=5, death_size=5)
CAM = dict(birth_rate=0.2, longevity=2, velocity=0, gravity=0, radius_x=0.2, radius_y=0.15,
           radius_z=0.45, birth_size=10, death_size=10, camera_fov=20,
           particle_type="shaded_sphere", max_opacity=100)

CASES = {
    "FX-PSPACE-001": ("The settings as they start, at frames 0, 12 and 24: nothing at frame 0 (the "
                      "first particle is born 0.06 of a frame after it), then 200 a second thrown out every way from the middle as "
                      "faint yellow-to-red streaks, falling a little.", case(), [0, 12, 24]),
    "FX-PSPACE-002": ("One disc, still, seen close: a round dot about 4.6 pixels across, its colour "
                      "a fifth of the way from yellow to red, fully covering at the centre (Max "
                      "Opacity 100).",
                      case(**dict(ONE, particle_type="disc")), [20]),
    "FX-PSPACE-003": ("The same particle as a Faded Sphere: soft, fading to nothing at its edge.",
                      case(**dict(ONE, particle_type="faded_sphere")), [20]),
    "FX-PSPACE-004": ("As a Shaded Sphere: lit from the upper right and in front, darker at its "
                      "lower left.", case(**dict(ONE, particle_type="shaded_sphere")), [20]),
    "FX-PSPACE-005": ("As a Bubble: a lit rim with a nearly clear middle.",
                      case(**dict(ONE, particle_type="bubble")), [20]),
    "FX-PSPACE-006": ("A Disc with Volume Shade 60: lit, but less than the Shaded Sphere.",
                      case(**dict(ONE, particle_type="disc", volume_shade=60)), [20]),
    "FX-PSPACE-007": ("A Shaded Sphere lit from 200 degrees and low (height 10): its bright side "
                      "moves to the bottom.",
                      case(**dict(ONE, particle_type="shaded_sphere", light_direction=200,
                                  light_height=10)), [20]),
    "FX-PSPACE-008": ("Death Size 2: the disc a fifth of the way from size 10 to 2, so smaller "
                      "than FX-PSPACE-002.",
                      case(**dict(ONE, particle_type="disc", death_size=2)), [20]),
    "FX-PSPACE-009": ("A tiny disc, size 1 (under a pixel): drawn 1.4 pixels across and fainter "
                      "by its true size.",
                      case(**dict(ONE, particle_type="disc", birth_size=1, death_size=1)), [20]),
    "FX-PSPACE-010": ("Size Variation 80 on a spread of still discs: each its own size.",
                      case(**dict(CAM, particle_type="disc", size_variation=80)), [20]),
    "FX-PSPACE-011": ("Birth and death colours blue to green.",
                      case(**dict(ONE, particle_type="disc", birth_color="#2040ff",
                                  death_color="#20ff40")), [20]),
    "FX-PSPACE-012": ("Origin to Death, the producer over the yellow column: the disc starts "
                      "the drawing's yellow under it and goes towards red.",
                      case(**dict(ORIGIN, color_map="origin_to_death",
                                  producer_x=-0.21875, producer_y=0.03125)), [20]),
    "FX-PSPACE-013": ("Birth to Origin, over the blue column.",
                      case(**dict(ORIGIN, color_map="birth_to_origin",
                                  producer_x=0.09375, producer_y=-0.21875)), [20]),
    "FX-PSPACE-014": ("Origin Constant, over the clear first column: black.",
                      case(**dict(ORIGIN, color_map="origin_constant",
                                  producer_x=-0.46875, producer_y=0.03125)), [20]),
    "FX-PSPACE-015": ("Origin Constant over the half-covered last column: its yellow, not "
                      "darkened by the half covering.",
                      case(**dict(ORIGIN, color_map="origin_constant",
                                  producer_x=0.46875, producer_y=-0.15625)), [20]),
    "FX-PSPACE-016": ("Custom with white, yellow, orange, red and near-black: at a fifth of its "
                      "life, between the first two (white and yellow).",
                      case(**dict(ONE, particle_type="disc", color_map="custom",
                                  custom_color_1="#ffffff", custom_color_3="#ff8000",
                                  custom_color_5="#330000")), [20]),
    "FX-PSPACE-071": ("Custom as added, from the birth colour to the death colour: at a fifth of "
                      "its life, close to FX-PSPACE-002 but not equal (the stops are spaced in "
                      "8-bit steps, the mix in linear light).",
                      case(**dict(ONE, particle_type="disc", color_map="custom")), [20]),
    "FX-PSPACE-072": ("Opacity Map Fade In And Out (D-837), the one disc: at frame 11, 0.037 of its "
                      "life, its centre 0.32 covered; at frame 20, 0.224 of its life, 0.96.",
                      case(**dict(ONE, particle_type="disc", opacity_map="fade_in_and_out")),
                      [11, 20]),
    "FX-PSPACE-073": ("Fade In And Out on the burst: the newest and oldest faint, the middle-aged "
                      "as FX-PSPACE-024 has them.",
                      case(**dict(FLOW, opacity_map="fade_in_and_out")), [12]),
    "FX-PSPACE-017": ("Two overlapping discs, Composite: the nearer one over the farther.",
                      case(**TWO), [20]),
    "FX-PSPACE-018": ("The two, Screen: brighter where they overlap, neither over the other.",
                      case(**dict(TWO, transfer_mode="screen")), [20]),
    "FX-PSPACE-019": ("The two, Add: the overlap summed.",
                      case(**dict(TWO, transfer_mode="add")), [20]),
    "FX-PSPACE-020": ("The two with Composite With Original on: over the drawing.",
                      case(**dict(TWO, composite_on_original="on", max_opacity=70)), [20]),
    "FX-PSPACE-021": ("Depth Cue Fade, distance 3: the particle 2 units away fades to a third.",
                      case(**dict(ONE, particle_type="disc", depth_cue="fade",
                                  depth_cue_distance=3)), [20]),
    "FX-PSPACE-022": ("Depth Cue Fog towards blue, distance 3: two thirds of the way to blue.",
                      case(**dict(ONE, particle_type="disc", depth_cue="fog",
                                  depth_cue_distance=3, depth_cue_color="#3366ff")), [20]),
    "FX-PSPACE-023": ("A spread of still shaded spheres at many depths: nearer ones larger and "
                      "over farther ones.", case(**CAM), [20]),
    "FX-PSPACE-024": ("A burst of discs falling: Explosive with gravity, at frames 6, 12, 24 and "
                      "40. Frame 40 drawn alone is the same as after frames 0 to 39.",
                      case(**FLOW), [6, 12, 24, 40]),
    "FX-PSPACE-025": ("Resistance 3: the burst slows and stays tighter.",
                      case(**dict(FLOW, resistance=3)), [12, 24]),
    "FX-PSPACE-026": ("Direction Axis, Extra Angle 30: a jet upwards within 15 degrees.",
                      case(**dict(FLOW, animation="direction_axis", extra_angle=30,
                                  producer_y=0.3)), [12, 24]),
    "FX-PSPACE-027": ("Cone Axis, 60 degrees: a hollow cone upwards.",
                      case(**dict(FLOW, animation="cone_axis", extra_angle=60,
                                  producer_y=0.3)), [12, 24]),
    "FX-PSPACE-028": ("Viscous: Explosive held back hard.",
                      case(**dict(FLOW, animation="viscous")), [12, 24]),
    "FX-PSPACE-029": ("Twirl: swirling round the upright axis through the producer.",
                      case(**dict(FLOW, animation="twirl", gravity=0)), [12, 24]),
    "FX-PSPACE-030": ("Twirly: each particle swirling by its own amount.",
                      case(**dict(FLOW, animation="twirly", gravity=0)), [12, 24]),
    "FX-PSPACE-031": ("Vortex: swirling and pulled in to the axis.",
                      case(**dict(FLOW, animation="vortex", gravity=0)), [12, 24]),
    "FX-PSPACE-032": ("Fire: rising against gravity, narrowing and flickering.",
                      case(**dict(SLOW, animation="fire", producer_y=0.3)), [12, 24]),
    "FX-PSPACE-033": ("Jet Sideways: out flat round the axis.",
                      case(**dict(FLOW, animation="jet_sideways", gravity=0)), [12, 24]),
    "FX-PSPACE-034": ("Fractal Omni: Explosive in the turbulence.",
                      case(**dict(SLOW, animation="fractal_omni")), [12, 24]),
    "FX-PSPACE-035": ("Fractal Uni: Direction Axis in the turbulence.",
                      case(**dict(SLOW, animation="fractal_uni", extra_angle=40,
                                  producer_y=0.3)), [12, 24]),
    "FX-PSPACE-036": ("Turbulence 3, Scale 0.5, Evolution 2 on Explosive.",
                      case(**dict(SLOW, turbulence=3, turbulence_scale=0.5,
                                  turbulence_evolution=2)), [12, 24]),
    "FX-PSPACE-037": ("Wind 1.5 to the right: the burst drifts right.",
                      case(**dict(FLOW, wind_x=1.5)), [12, 24]),
    "FX-PSPACE-038": ("Gravity Vector to the right: falling sideways.",
                      case(**dict(FLOW, gravity=1, gravity_x=1, gravity_y=0)), [12, 24]),
    "FX-PSPACE-039": ("Inherit Velocity 100 with the producer keyed across: the particles carry "
                      "its motion.",
                      case(**dict(FLOW, velocity=0.2, gravity=0, inherit_velocity=100,
                                  producer_x=keyed((0, -0.4), (24, 0.4)))), [12, 24]),
    "FX-PSPACE-040": ("Floor Bounce at 0.1, Bounciness 60, Random 30, Spread 50: the burst lands "
                      "and hops.",
                      case(**dict(FLOW, gravity=3, floor_action="bounce", floor_position=0.1,
                                  bounciness=60, random_bounciness=30, bounce_spread=50)),
                      [12, 24]),
    "FX-PSPACE-041": ("Floor Ice: the burst lands and slides.",
                      case(**dict(FLOW, gravity=3, floor_action="ice", floor_position=0.1)),
                      [12, 24]),
    "FX-PSPACE-042": ("Floor Glue: the burst lands and stays.",
                      case(**dict(FLOW, gravity=3, floor_action="glue", floor_position=0.1)),
                      [12, 24]),
    "FX-PSPACE-043": ("The effect camera turned 60 degrees about Y and 20 about X: the spread "
                      "seen from the side and above.",
                      case(**dict(CAM, camera_rotation_y=60, camera_rotation_x=20)), [20]),
    "FX-PSPACE-044": ("The effect camera 1 unit away with a 100 degree lens.",
                      case(**dict(CAM, camera_distance=1, camera_fov=100)), [20]),
    "FX-PSPACE-045": ("The composition's own camera, as added: the spread drawn through it, "
                      "nearer particles larger and spread wider.",
                      case(camera=(8, 5, -W * 50 / 36, W * 50 / 36), **CAM), [20]),
    "FX-PSPACE-046": ("The composition camera moved 3 pixels left: the layer moves right on "
                      "screen, and the particles off its plane move by more or less than it.",
                      case(camera=(5, 5, -W * 50 / 36, W * 50 / 36), **CAM), [20]),
    "FX-PSPACE-047": ("The composition camera dollied in with its lens widened to match (depth "
                      "-12, zoom 12): the layer's plane unchanged, the depth stronger.",
                      case(camera=(8, 5, -12, 12), **CAM), [20]),
    "FX-PSPACE-048": ("FX-PSPACE-045 with the layer moved 3 pixels right.",
                      case(camera=(8, 5, -W * 50 / 36, W * 50 / 36), shift=3, **CAM), [20]),
    "FX-PSPACE-049": ("The camera dollied to 2 pixels from the plane: the particles nearer than "
                      "it, behind the camera, are not drawn (4 of the 17).",
                      case(camera=(8, 5, -2, 2), **CAM), [20]),
    "FX-PSPACE-050": ("Birth Rate keyed from 0 at frame 0 to 0.5 at frame 20: births start "
                      "slowly and quicken.",
                      case(**dict(FLOW, birth_rate=keyed((0, 0), (20, 0.5)))), [10, 20, 30]),
    "FX-PSPACE-051": ("Velocity keyed from 0.2 to 1 over frames 0 to 20: each particle keeps the "
                      "speed it was born with.",
                      case(**dict(FLOW, gravity=0, velocity=keyed((0, 0.2), (20, 1)))),
                      [10, 20]),
    "FX-PSPACE-052": ("Gravity keyed 0 to 3 over frames 0 to 24: every particle feels it when "
                      "it changes.",
                      case(**dict(FLOW, gravity=keyed((0, 0), (24, 3)))), [12, 24]),
    "FX-PSPACE-053": ("FX-PSPACE-024 with the layer starting at frame 10: nothing before, and "
                      "frame 22 the same as FX-PSPACE-024's frame 12.",
                      case(in_frame=10, **FLOW), [10, 22]),
    "FX-PSPACE-054": ("FX-PSPACE-024 with the layer moved 3 pixels right.",
                      case(shift=3, **FLOW), [12]),
    "FX-PSPACE-055": ("Random Seed 7: a different burst.", case(**dict(FLOW, random_seed=7)), [12]),
    "FX-PSPACE-056": ("Birth Rate 0: nothing.", case(birth_rate=0), [12]),
    "FX-PSPACE-057": ("Longevity 0: nothing.", case(longevity=0), [12]),
    "FX-PSPACE-058": ("Lines, larger: the streak's length follows Birth Size.",
                      case(**dict(FLOW, particle_type="line", birth_size=0.6, death_size=0.6)),
                      [12]),
}
FRAMES_OF = {fx: frames for fx, (_, _, frames) in CASES.items()}
assert all(in_range(c) for _, c, _ in CASES.values())

INVALID = {
    "FX-PSPACE-059": ("Birth Rate -1, below 0.", case(birth_rate=-1)),
    "FX-PSPACE-060": ("Birth Rate 101, above 100.", case(birth_rate=101)),
    "FX-PSPACE-061": ("Longevity 31, above 30.", case(longevity=31)),
    "FX-PSPACE-062": ("Turbulence Scale 0, below 0.01.", case(turbulence_scale=0)),
    "FX-PSPACE-063": ("Camera FOV 171, above 170.", case(camera_fov=171)),
    "FX-PSPACE-064": ("Animation \"spiral\": not one of the eleven.", case(animation="spiral")),
    "FX-PSPACE-065": ("Animation \"Explosive\": the word is exact.", case(animation="Explosive")),
    "FX-PSPACE-066": ("Particle Type \"star\": not one of the five (stars wait for D-612).",
                      case(particle_type="star")),
    "FX-PSPACE-067": ("Floor Action \"slide\".", case(floor_action="slide")),
    "FX-PSPACE-068": ("Birth Color \"#12345\": not a colour.", case(birth_color="#12345")),
    "FX-PSPACE-069": ("Composite With Original \"yes\": only \"off\" or \"on\".",
                      case(composite_on_original="yes")),
    "FX-PSPACE-070": ("Transfer Mode \"multiply\".", case(transfer_mode="multiply")),
}

# The cases whose every alive particle is written out, number by number, for the engine's own
# test: (case, frame).
LISTED = [("FX-PSPACE-024", 12), ("FX-PSPACE-025", 24), ("FX-PSPACE-029", 12),
          ("FX-PSPACE-031", 24), ("FX-PSPACE-032", 24), ("FX-PSPACE-036", 24),
          ("FX-PSPACE-039", 24), ("FX-PSPACE-040", 24), ("FX-PSPACE-041", 24),
          ("FX-PSPACE-042", 24), ("FX-PSPACE-050", 30), ("FX-PSPACE-052", 24)]


def listed():
    out = {"tolerance": 1e-7, "cases": {}}
    px = working()
    for fx, f in LISTED:
        c = CASES[fx][1]
        run = Run(c, f)
        rows = []
        for k, b in births(run):
            if not run.u - b < run.at("longevity", b) * FPS:
                continue
            p = moved(run, k, b)
            rows.append([k, b] + p["x"] + p["v"])
        out["cases"][fx] = {"frame": f, "particles": rows}
        print(f"{fx} frame {f}: {len(rows)} particles listed")
    del px
    (OUT / "particles_particle_space.json").write_text(json.dumps(out, indent=1) + "\n",
                                                      encoding="utf-8")


def main():
    clear()
    expected = B.write_cases(OUT, "particle_space", CASES, INVALID, render, plain, file_json)
    listed()
    check(expected)


def clear():
    """No number lands so near a cut that the build's rounding could put it on the other side:
    a birth near the frame drawn, an age near its life, a depth near the camera's cut-off, a
    spawn point near a pixel's edge, or two overlapping particles at nearly the same depth."""
    px = working()
    NEAR.clear()
    for fx, (_, c, frames) in CASES.items():
        for f in frames:
            run = Run(c, f)
            if run.u < 0:
                continue
            for k, b in births(run):
                assert abs(b - run.u) > 1e-6, (fx, f, k)
                assert abs((run.u - b) - run.at("longevity", b) * FPS) > 1e-6, (fx, f, k)
            sps = sprites(run, px, view_of(c))
            for d, _, sp, p in sps:
                assert abs(d - 0.01) > 1e-6, fx
                if c["color_map"] != "birth_to_death" and c["color_map"] != "custom":
                    for v in (W / 2 + p["x0"][0] * W, H / 2 + p["x0"][1] * W):
                        assert abs(v - round(v)) > 1e-6, (fx, v)
            for i in range(len(sps)):
                for j in range(i + 1, len(sps)):
                    a, b = sps[i][2], sps[j][2]
                    if a["kind"] == "line":
                        continue
                    near = math.hypot(a["X"] - b["X"], a["Y"] - b["Y"]) < a["R"] + b["R"] + 1
                    assert not (near and abs(sps[i][0] - sps[j][0]) < 1e-7), (fx, f, i, j)
    assert min(NEAR) > 1e-9, min(NEAR)
    print(f"clear: nearest miss of the floor {min(NEAR):.3g}")


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    clear_frame = [[0.0] * 4 for _ in range(W * H)]
    total = lambda fr: sum(p[3] for p in fr)  # noqa: E731
    assert c["FX-PSPACE-001"]["0"] == clear_frame
    assert total(c["FX-PSPACE-001"]["24"]) > 0
    one = c["FX-PSPACE-002"]["20"]
    centre = max(range(W * H), key=lambda i: one[i][3])
    assert abs(one[centre][3] - 1.0) < 1e-12
    assert total(c["FX-PSPACE-003"]["20"]) < total(one)
    assert total(c["FX-PSPACE-005"]["20"]) < total(one)
    assert total(c["FX-PSPACE-008"]["20"]) < total(one)
    assert total(c["FX-PSPACE-009"]["20"]) < total(c["FX-PSPACE-008"]["20"])
    assert c["FX-PSPACE-004"]["20"] != c["FX-PSPACE-007"]["20"]
    assert c["FX-PSPACE-006"]["20"] != one
    for fx in ("FX-PSPACE-012", "FX-PSPACE-013", "FX-PSPACE-014", "FX-PSPACE-015",
               "FX-PSPACE-016", "FX-PSPACE-011", "FX-PSPACE-071"):
        assert c[fx]["20"] != one, fx
    dark = c["FX-PSPACE-014"]["20"]
    assert all(p[0] == 0 and p[1] == 0 and p[2] == 0 for p in dark) and total(dark) > 0
    comp, scr, ad = (c["FX-PSPACE-%03d" % n]["20"] for n in (17, 18, 19))
    assert comp != scr != ad != comp
    assert total(c["FX-PSPACE-021"]["20"]) < total(one) * 0.34
    assert c["FX-PSPACE-024"]["40"] != c["FX-PSPACE-024"]["24"]
    for n in range(25, 44):
        assert c["FX-PSPACE-%03d" % n] != c["FX-PSPACE-024"], n
    assert c["FX-PSPACE-053"]["10"] == clear_frame
    assert c["FX-PSPACE-053"]["22"] == c["FX-PSPACE-024"]["12"]
    moved_ = c["FX-PSPACE-054"]["12"]
    base = c["FX-PSPACE-024"]["12"]
    for y in range(H):
        assert moved_[y * W + 3:(y + 1) * W] == base[y * W:y * W + W - 3]
    assert c["FX-PSPACE-055"]["12"] != base
    assert c["FX-PSPACE-056"]["12"] == clear_frame
    assert c["FX-PSPACE-057"]["12"] == clear_frame
    assert c["FX-PSPACE-045"]["20"] != c["FX-PSPACE-023"]["20"]
    # D-837: the faded disc's centre is the map at its age; the burst is fainter, not moved
    for fr, f in (("11", (11 - 0.5 / 1.3 * 24) / 48), ("20", (20 - 0.5 / 1.3 * 24) / 48)):
        fd = c["FX-PSPACE-072"][fr]
        assert abs(max(p[3] for p in fd) - opacity_map({"opacity_map": "fade_in_and_out"}, f)) < 1e-9, (fr, f)
    assert total(c["FX-PSPACE-073"]["12"]) < total(c["FX-PSPACE-024"]["12"])
    assert c["FX-PSPACE-047"]["20"] != c["FX-PSPACE-045"]["20"]
    # the camera's own plane is 1:1: a particle at depth 0 sits where the layer puts it
    see = camera(Run(CASES["FX-PSPACE-045"][1], 20), view_of(CASES["FX-PSPACE-045"][1]))
    s = see([0.1, -0.2, 0.0])
    assert abs(s[0] - (8 + 1.6)) < 1e-12 and abs(s[1] - (5 - 3.2)) < 1e-12 and s[2] == W
    # a particle behind the dollied camera is left out
    run49 = Run(CASES["FX-PSPACE-049"][1], 20)
    behind = [p for _, _, _, p in sprites(run49, working(), view_of(CASES["FX-PSPACE-049"][1]))]
    alive = [b for k, b in births(run49) if run49.u - b < 2 * FPS]
    assert len(behind) < len(alive)
    # births: constant rate closed form, and the keyed ramp's first births late
    run = Run(case(birth_rate=0.013), 20)
    assert [round(b, 9) for _, b in births(run)] == [round(0.5 / 1.3 * 24, 9)]
    ramp = Run(CASES["FX-PSPACE-050"][1], 30)
    first = births(ramp)[0][1]
    # B(t) = (100 * 0.5 / 24) / 20 * t^2 / 2 at t <= 20 frames, so b_0 = sqrt(2 * 0.5 * 20 / q)
    q = 100 * 0.5 / 24
    assert abs(first - math.sqrt(0.5 * 2 * 20 / q)) < 1e-9, first
    # the closed form: a particle with no forces but gravity lands where the formula says
    flat = case(birth_rate=0.013, longevity=3, velocity=0, radius_x=0, radius_y=0, radius_z=0,
                gravity=0.5, extra=0)
    run = Run(flat, 40)
    k, b = births(run)[0]
    p = moved(run, k, b)
    tt = (40 - b) / FPS
    assert abs(p["x"][1] - 0.25 * tt * tt) < 1e-12, (p["x"], tt)
    drag = case(birth_rate=0.013, longevity=3, velocity=0, radius_x=0, radius_y=0, radius_z=0,
                gravity=0.5, resistance=2, extra=0)
    run = Run(drag, 40)
    k, b = births(run)[0]
    p = moved(run, k, b)
    want = 0.5 / 2 * tt + (0 - 0.5 / 2) * (-math.expm1(-2 * tt)) / 2
    assert abs(p["x"][1] - want) < 1e-12, (p["x"][1], want)
    # glue: everything that landed sits on the floor, still
    run = Run(CASES["FX-PSPACE-042"][1], 24)
    landed = [moved(run, k, b) for k, b in births(run) if run.u - b < 1.5 * FPS]
    stuck = [p for p in landed if p["glued"]]
    assert stuck and all(p["x"][1] == 0.1 and p["v"] == [0.0, 0.0, 0.0] for p in stuck)
    run = Run(CASES["FX-PSPACE-041"][1], 24)
    iced = [p for p in (moved(run, k, b) for k, b in births(run) if run.u - b < 1.5 * FPS)
            if p["iced"]]
    assert iced and all(p["x"][1] == 0.1 and p["v"][1] == 0.0 for p in iced)
    run = Run(CASES["FX-PSPACE-040"][1], 24)
    assert any(moved(run, k, b)["bounces"] > 0 for k, b in births(run))
    print("checked")


if __name__ == "__main__":
    main()
