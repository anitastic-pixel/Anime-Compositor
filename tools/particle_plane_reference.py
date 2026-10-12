"""Particle Plane, worked a second way.

`core.particle_plane` (D-612), the particle engine's flat emitter (P0-6), modelled on what
CycoreFX's CC Particle Systems II is for: particles born from a producer on the layer, thrown out
by one of eleven animations, pulled down by gravity and slowed by resistance, all in the layer's
own plane, drawn straight onto it with no camera and no depth. Cycore publishes no maths and no
settings when added, so every rule and every number below is ours, written after what each
control is for (document 21, D-612). No text or code was copied. "Particle Plane" is our name.

It runs on Particle Space's engine (D-610, `tools/particle_space_reference.py`, here P): the same
births, life, randomness, steps, closed-form pull and noise field, and the same drawing of each
kind, laid by the same transfer modes. What is the plane's own is said below; everything else is
D-610's, word for word. **No state is carried from frame to frame** (document 20).

The settings are SETTINGS (name, lowest, highest, value when added; all keyable), WORDS (name,
the words, the word when added; exact) and COLOURS (name, `#rrggbb` when added). Distances are
in the layer's widths W, the origin its centre, Y down; sizes in hundredths of W; times in
seconds; Birth Rate in hundreds of particles a second; Direction in degrees, 0 up and 90 to the
right.

- The engine's settings, at each frame's sample: Birth Rate, Longevity, Producer (x, y, 0),
  Radius (x, y, 0), Velocity, Inherit Velocity, Gravity, Resistance, Extra, Extra Angle, Birth
  Size, Death Size, Size Variation, Max Opacity and Random Seed as given; Direction Axis
  (sin d, -cos d, 0), d the Direction; Gravity Vector (0, 1, 0), so Gravity pulls down and Fire
  rises up; no wind, no turbulence (Turbulence 0, Scale 0.25, Evolution 0), no floor, Volume
  Shade 0, the light from 45 degrees at height 45. Between frames these are read straight-line
  as D-610 reads every setting, so a keyed Direction turns its pointing vector, made unit.
- Spawn, read at b_k: the point (Rx rho cos phi, Ry rho sin phi, 0) from the producer, rho =
  sqrt(r0), phi = 2 pi r2 (r1 not used).
- Thrown, read at b_k, A the axis as D-610 finds it and e1 = (-A_y, A_x, 0), alpha = Extra Angle
  / 2, the turn t giving cos t A + sin t e1, s = 1 when r4 < 1/2 else -1:
  Explosive, Viscous, Twirl, Twirly, Vortex, Fractal Omni: (cos 2 pi r3, sin 2 pi r3, 0);
  Direction, Fractal Uni: t = alpha (2 r3 - 1); Cone: t = s alpha; Fire: t = 20 degrees
  (2 r3 - 1); Jet Sideways: s e1. Speed and Inherit Velocity as D-610's.
- Steps as D-610's, except: Twirl, Twirly and Vortex swirl about (0, 0, 1) through the producer
  (Fire keeps its axis A); the turbulence (Fire's 0.5, Fractal Omni's and Uni's 1) pushes by
  (dN/dy, -dN/dx, 0), N the noise field's channel 2 at the same q, each derivative as D-610's.
  Nothing ever leaves the plane: z stays 0.
- Drawing: a particle at (x, y) is at (W/2 + x W, H/2 + y W) on the drawing, one unit W pixels
  across, every particle at the same depth, so they go on by number, oldest first (the newest
  on top). Every pixel starts clear; the input is read only for the Origin colours.
- The opacity: Max Opacity / 100 times the Opacity Map at the age fraction f: Constant 1, Fade
  In f, Fade Out 1 - f, Fade In and Out 1 - |2 f - 1|, Custom the five Custom Opacity settings /
  100 at 0, 1/4, 1/2, 3/4 and 1, straight-line between (Max Opacity and the five read at the
  frame drawn).
- Star, a kind of its own (D-610's INVALID case FX-PSPACE-066 said stars wait for D-612): five
  points up, outer radius R, inner 0.4 R; its outline the ten points STAR (x, y) times R from the
  centre, the j-th 36 j degrees clockwise from up (the table is the rule, as written below).
  With e the distance from the pixel centre to the nearest of the ten edges, the covering is
  clamp(1/2 - e, 0, 1) outside the outline and clamp(1/2 + e, 0, 1) inside (inside by the even-odd
  rule: a ray to the right crossing the edges whose ends lie on either side of the pixel's row,
  the end below it counted as on one side, at x = a_x + (b_x - a_x)(y - a_y) / (b_y - a_y)).
  Small stars as D-610's other kinds (R < 0.7 drawn at 0.7, the opacity times (R / 0.7)^2); never
  lit.

**This file never runs the build's code path.** It works in double precision on lists from the
drawing's 8-bit values; trigonometry is the platform's. The drawing is Broadcast Colors' 16 by
10. The projects go into `Fixtures/particle_plane`, the expected frames into
`Fixtures/particle_plane/expected_particle_plane.json`, each alive particle's numbers for some
cases into `Fixtures/particle_plane/particles_particle_plane.json`. Every case is "not built yet"
until B-492 lands (D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/particle_plane_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import particle_space_reference as P  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H, FPS = P.W, P.H, P.FPS
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "particle_plane"

SETTINGS = [
    ("birth_rate", 0, 100, 2), ("longevity", 0, 30, 1),
    ("producer_x", -10, 10, 0), ("producer_y", -10, 10, 0),
    ("radius_x", 0, 10, 0.025), ("radius_y", 0, 10, 0.025),
    ("velocity", -10, 10, 1), ("inherit_velocity", -10000, 10000, 0),
    ("gravity", -10, 10, 0.5), ("resistance", 0, 100, 0),
    ("direction", -36000, 36000, 0), ("extra", 0, 10, 0.5), ("extra_angle", 0, 360, 360),
    ("birth_size", 0, 100, 0.15), ("death_size", 0, 100, 0.15), ("size_variation", 0, 100, 0),
    ("max_opacity", 0, 100, 75),
    ("custom_opacity_1", 0, 100, 100), ("custom_opacity_2", 0, 100, 100),
    ("custom_opacity_3", 0, 100, 100), ("custom_opacity_4", 0, 100, 100),
    ("custom_opacity_5", 0, 100, 100),
    ("random_seed", 0, 10000, 0),
]
NAMES = [s[0] for s in SETTINGS]
RANGES = {s[0]: (s[1], s[2]) for s in SETTINGS}
ANIMATIONS = ("explosive", "direction", "cone", "viscous", "twirl", "twirly", "vortex", "fire",
              "jet_sideways", "fractal_omni", "fractal_uni")
KINDS = ("line", "faded_sphere", "shaded_sphere", "disc", "bubble", "star")
WORDS = [
    ("animation", ANIMATIONS, "explosive"),
    ("particle_type", KINDS, "line"),
    ("opacity_map", ("constant", "fade_in", "fade_out", "fade_in_and_out", "custom"), "constant"),
    ("color_map", ("birth_to_death", "origin_to_death", "birth_to_origin", "origin_constant",
                   "custom"), "birth_to_death"),
    ("transfer_mode", ("composite", "screen", "add"), "composite"),
]
COLOURS = [(n, v) for n, v in P.COLOURS if n != "depth_cue_color"]

# The star's outline, the rule itself: point j at 36 j degrees clockwise from up, the outer five
# 1 from the centre and the inner five 0.4, as the platform's sine and cosine gave them.
STAR = [(0.0, -1.0), (0.23511410091698925, -0.323606797749979),
        (0.9510565162951535, -0.30901699437494745), (0.3804226065180615, 0.12360679774997894),
        (0.5877852522924732, 0.8090169943749473), (4.898587196589413e-17, 0.4),
        (-0.587785252292473, 0.8090169943749475), (-0.3804226065180614, 0.12360679774997903),
        (-0.9510565162951536, -0.30901699437494723),
        (-0.23511410091698937, -0.32360679774997897)]


# --- the run: the plane's settings as the engine's ------------------------------------------

def settings(c, frame_no):
    out = {}
    for n in NAMES:
        lo, hi = RANGES[n]
        out[n] = P.clamp(value_at(c[n], frame_no), lo, hi)
    return out


def engine(s):
    """One frame's plane settings as Particle Space's."""
    e = {n: d for n, _, _, d in P.SETTINGS}
    for n in ("birth_rate", "longevity", "producer_x", "producer_y", "radius_x", "radius_y",
              "velocity", "inherit_velocity", "gravity", "resistance", "extra", "extra_angle",
              "birth_size", "death_size", "size_variation", "max_opacity", "random_seed"):
        e[n] = s[n]
    d = math.radians(s["direction"])
    e.update(producer_z=0.0, radius_z=0.0, axis_x=math.sin(d), axis_y=-math.cos(d), axis_z=0.0,
             gravity_x=0.0, gravity_y=1.0, gravity_z=0.0, wind_x=0.0, wind_y=0.0, wind_z=0.0,
             turbulence=0.0, turbulence_scale=0.25, turbulence_evolution=0.0, volume_shade=0.0,
             light_direction=45.0, light_height=45.0)
    return e


class Run(P.Run):
    def __init__(self, c, n):
        self.c = c
        self.u = n - c["in_frame"]
        own = [settings(c, c["in_frame"] + i) for i in range(self.u + 2)]
        self.samples = [engine(s) for s in own]
        self.own = own[self.u]
        self.now = self.samples[self.u]
        self.seed = math.floor(self.now["random_seed"])


# --- birth and steps in the plane -----------------------------------------------------------

def born(run, k, b):
    r = lambda s: run.r(k, s)  # noqa: E731
    p = run.vec("producer", b)
    rad = run.vec("radius", b)
    rho = math.sqrt(r(0))
    turn = 2 * math.pi * r(2)
    x = [p[0] + rad[0] * rho * math.cos(turn), p[1] + rad[1] * rho * math.sin(turn), 0.0]
    mode = run.c["animation"]
    a = P.axes(run, b)
    e1 = [-a[1], a[0], 0.0]
    side = 1.0 if r(4) < 0.5 else -1.0

    def turned(t):
        return P.add(P.mul(a, math.cos(t)), P.mul(e1, math.sin(t)))

    alpha = math.radians(run.at("extra_angle", b) / 2)
    if mode in ("direction", "fractal_uni"):
        way = turned(alpha * (2 * r(3) - 1))
    elif mode == "cone":
        way = turned(side * alpha)
    elif mode == "fire":
        way = turned(math.radians(20) * (2 * r(3) - 1))
    elif mode == "jet_sideways":
        way = P.mul(e1, side)
    else:
        t = 2 * math.pi * r(3)
        way = [math.cos(t), math.sin(t), 0.0]
    speed = run.at("velocity", b) * (1 + run.at("extra", b) * (r(5) - 0.5))
    v = P.mul(way, speed)
    iv = run.at("inherit_velocity", b)
    if iv != 0:
        pv = P.mul(P.sub(run.vec("producer", b + 0.125), run.vec("producer", b - 0.125)),
                   FPS / 0.25)
        v = P.add(v, P.mul(pv, iv / 100))
    sv = run.at("size_variation", b) / 100
    return {"k": k, "b": b, "x": x, "v": v, "x0": list(x),
            "sizes": (run.at("birth_size", b), run.at("death_size", b)),
            "size_factor": 1 + sv * (2 * r(6) - 1), "twirl": 2 * r(9)}


def curl2(seed, q):
    e = 1e-3

    def d(axis):
        hi, lo = list(q), list(q)
        hi[axis] += e
        lo[axis] -= e
        return (P.noise(seed, 2, hi) - P.noise(seed, 2, lo)) / (2 * e)

    return [d(1), -d(0), 0.0]


def step(run, p, t0, h):
    mode = run.c["animation"]
    at = lambda n: run.at(n, t0)  # noqa: E731
    x, v = p["x"], p["v"]
    push = [0.0, 0.0, 0.0]
    if mode in ("twirl", "twirly", "vortex", "fire"):
        a = P.axes(run, t0) if mode == "fire" else [0.0, 0.0, 1.0]
        dd = P.sub(x, run.vec("producer", t0))
        dp = P.sub(dd, P.mul(a, P.dot(dd, a)))
        vel = at("velocity")
        if mode == "fire":
            push = P.mul(dp, -4)
        else:
            push = P.mul(P.cross(a, dp), 2 * vel / (P.dot(dp, dp) + 0.01))
            if mode == "twirly":
                push = P.mul(push, p["twirl"])
            if mode == "vortex":
                push = P.add(push, P.mul(dp, -2 * vel))
    strength = at("turbulence") + {"fire": 0.5, "fractal_omni": 1, "fractal_uni": 1}.get(mode, 0)
    if strength > 0:
        evo = at("turbulence_evolution")
        sc = at("turbulence_scale")
        q = [x[0] / sc + evo * 0.31, x[1] / sc + evo * 0.57, x[2] / sc + evo * 0.77]
        push = P.add(push, P.mul(curl2(run.seed, q), strength))
    v = P.add(v, P.mul(push, h))
    g = run.vec("gravity", t0)
    gu = P.unit_or(g, (0, 0, 0))
    a = P.add(P.mul(gu, -at("gravity") if mode == "fire" else at("gravity")), run.vec("wind", t0))
    k = at("resistance") + (3 if mode == "viscous" else 0)
    p["x"], p["v"] = P.closed(x, v, a, k, h)


def moved(run, k, b):
    p = born(run, k, b)
    points = [b] + [j / 4 for j in range(math.floor(4 * b) + 1, 4 * run.u + 1)]
    for t0, t1 in zip(points, points[1:]):
        step(run, p, t0, (t1 - t0) / FPS)
    return p


# --- the drawing ----------------------------------------------------------------------------

def fade(c, own, f):
    m = c["opacity_map"]
    if m == "constant":
        return 1.0
    if m == "fade_in":
        return f
    if m == "fade_out":
        return 1 - f
    if m == "fade_in_and_out":
        return 1 - abs(2 * f - 1)
    stops = [own["custom_opacity_%d" % i] / 100 for i in range(1, 6)]
    i = min(math.floor(f * 4), 3)
    return stops[i] + (stops[i + 1] - stops[i]) * (f * 4 - i)


def flat(x):
    return (W / 2 + x[0] * W, H / 2 + x[1] * W)


def sprites(run, px):
    """Every particle drawn, in the order laid: (k, sprite, particle)."""
    c, now = run.c, run.now
    out = []
    for k, b in P.births(run):
        age = run.u - b
        life = run.at("longevity", b) * FPS
        if not age < life:
            continue
        p = moved(run, k, b)
        f = age / life
        bs, ds = p["sizes"]
        size = (bs + (ds - bs) * f) * p["size_factor"]
        X, Y = flat(p["x"])
        rad = size / 100 * W / 2
        col = P.colour(c, f, P.origin(px, p["x0"]))
        alpha = now["max_opacity"] / 100 * fade(c, run.own, f)
        kind = c["particle_type"]
        sp = {"kind": kind, "X": X, "Y": Y, "col": col}
        if kind == "line":
            X2, Y2 = flat(P.sub(p["x"], P.mul(p["v"], size / 0.15 / FPS)))
            sp.update(X2=X2, Y2=Y2, w=max(1.0, 2 * rad), alpha=alpha)
        else:
            if rad < 0.7:
                alpha *= (rad / 0.7) ** 2
                rad = 0.7
            sp.update(R=rad, alpha=alpha)
        out.append((k, sp, p))
    out.sort(key=lambda e: e[0])
    return out


def star_cover(rad, dx, dy):
    pts = [(sx * rad, sy * rad) for sx, sy in STAR]
    e = math.inf
    inside = False
    for j in range(10):
        (ax, ay), (bx, by) = pts[j], pts[(j + 1) % 10]
        ex, ey = bx - ax, by - ay
        t = P.clamp(((dx - ax) * ex + (dy - ay) * ey) / (ex * ex + ey * ey))
        e = min(e, math.hypot(dx - ax - t * ex, dy - ay - t * ey))
        if (ay > dy) != (by > dy) and dx < ax + (bx - ax) * (dy - ay) / (by - ay):
            inside = not inside
    return P.clamp(0.5 + e if inside else 0.5 - e)


def cover(sp, x, y, lt):
    if sp["kind"] != "star":
        return P.cover(sp, x, y, 0.0, lt)
    cv = star_cover(sp["R"], x - sp["X"], y - sp["Y"])
    if cv <= 0:
        return None
    a = sp["alpha"] * cv
    return [sp["col"][i] * a for i in range(3)] + [a]


def particle_plane(px, c, n):
    run = Run(c, n)
    if run.u < 0:
        return [list(p) for p in px]
    out = [[0.0] * 4 for _ in px]
    lt = P.light(run.now)
    mode = c["transfer_mode"]
    for _, sp, _ in sprites(run, px):
        if sp["kind"] == "line":
            r = sp["w"] / 2 + 1
            xs = (min(sp["X"], sp["X2"]) - r, max(sp["X"], sp["X2"]) + r)
            ys = (min(sp["Y"], sp["Y2"]) - r, max(sp["Y"], sp["Y2"]) + r)
        else:
            r = sp["R"] + 1
            xs, ys = (sp["X"] - r, sp["X"] + r), (sp["Y"] - r, sp["Y"] + r)
        for gy in range(max(0, math.floor(ys[0])), min(H, math.ceil(ys[1]) + 1)):
            for gx in range(max(0, math.floor(xs[0])), min(W, math.ceil(xs[1]) + 1)):
                s = cover(sp, gx + 0.5, gy + 0.5, lt)
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

def case(shift=0, in_frame=0, **given):
    c = {s[0]: s[3] for s in SETTINGS}
    c.update({w[0]: w[2] for w in WORDS})
    c.update({k: v for k, v in COLOURS})
    for k, v in given.items():
        assert k in c, k
        c[k] = v
    c.update(shift=shift, in_frame=in_frame)
    return c


def in_range(c):
    for n, lo, hi, _ in SETTINGS:
        v = c[n]
        for x in ([k[1] for k in v["keys"]] if isinstance(v, dict) else [v]):
            if not lo <= x <= hi:
                return False
    return True


def render(c, frame_no):
    return R.frame(particle_plane(P.working(), c, frame_no), c["shift"])


def plain(c):
    return R.frame(P.working(), c["shift"])


def file_json(fx, c):
    params = {n: setting_json(c[n]) for n in NAMES}
    params.update({w[0]: c[w[0]] for w in WORDS})
    params.update({k: c[k] for k, _ in COLOURS})
    p = B.project_json(fx, "core.particle_plane", params, c["shift"])
    comp = p["compositions"][0]
    frames = max([f for f in FRAMES_OF.get(fx, [0])] + [4]) + 2
    comp["duration_frames"] = frames
    comp["work_area"]["end_frame_exclusive"] = frames
    layer = comp["layers"][0]
    layer["in_frame"] = c["in_frame"]
    layer["out_frame"] = frames
    return p


# One particle, still, in the middle: 1.3 a second, the first born at frame 9.23 and the next at
# 27.7, so frame 20 shows the first alone at age fraction 10.77 / 48. Size 50 is 8 pixels across.
ONE = dict(birth_rate=0.013, longevity=2, velocity=0, gravity=0, radius_x=0, radius_y=0,
           birth_size=50, death_size=50, max_opacity=100, particle_type="disc")
# Origin colours: a dot 1.6 pixels across over the pixel it took its colour from.
ORIGIN = dict(ONE, birth_size=10, death_size=10)
# Two: 2.6 a second, born at frames 4.62 and 13.85, apart.
TWO = dict(ONE, birth_rate=0.026, radius_x=0.2, radius_y=0.1, birth_size=40, death_size=40)
# A spread of still particles.
SPREAD = dict(ONE, birth_rate=0.2, radius_x=0.4, radius_y=0.25, birth_size=20, death_size=20)
# A burst to watch move: a hundred a second, slow, discs 4 pixels across shrinking to 1.6.
FLOW = dict(birth_rate=1, longevity=1.5, velocity=0.4, birth_size=25, death_size=10,
            particle_type="disc", max_opacity=90)
SLOW = dict(FLOW, birth_rate=0.06, longevity=2)

CASES = {
    "FX-PPLANE-001": ("The settings as they start, at frames 0, 12 and 24: nothing at frame 0, "
                      "then 200 a second thrown out every way in the layer's plane as faint "
                      "yellow-to-red streaks, falling a little.", case(), [0, 12, 24]),
    "FX-PPLANE-002": ("One disc, still, in the middle: a round dot 8 pixels across, a fifth of "
                      "the way from yellow to red, fully covering.", case(**ONE), [20]),
    "FX-PPLANE-003": ("The same particle as a Faded Sphere: soft to its edge.",
                      case(**dict(ONE, particle_type="faded_sphere")), [20]),
    "FX-PPLANE-004": ("As a Shaded Sphere: lit from the upper right and in front.",
                      case(**dict(ONE, particle_type="shaded_sphere")), [20]),
    "FX-PPLANE-005": ("As a Bubble: a lit rim, a nearly clear middle.",
                      case(**dict(ONE, particle_type="bubble")), [20]),
    "FX-PPLANE-006": ("As a Star: five points up, 8 pixels from tip to tip across the top.",
                      case(**dict(ONE, particle_type="star")), [20]),
    "FX-PPLANE-007": ("A still Line: no movement, so a round dot as wide as the size, the same as "
                      "FX-PPLANE-002's disc.",
                      case(**dict(ONE, particle_type="line")), [20]),
    "FX-PPLANE-008": ("A tiny star, size 3 (under a pixel's radius): drawn 1.4 pixels across "
                      "and fainter by its true size.",
                      case(**dict(ONE, particle_type="star", birth_size=3, death_size=3)), [20]),
    "FX-PPLANE-009": ("Death Size 10: the disc a fifth of the way from 50 to 10, smaller.",
                      case(**dict(ONE, death_size=10)), [20]),
    "FX-PPLANE-010": ("Size Variation 80 on a spread of still discs: each its own size.",
                      case(**dict(SPREAD, size_variation=80)), [20]),
    "FX-PPLANE-011": ("Opacity Map Fade In: at a fifth of its life, a fifth covering.",
                      case(**dict(ONE, opacity_map="fade_in")), [20]),
    "FX-PPLANE-012": ("Fade Out: four fifths covering.",
                      case(**dict(ONE, opacity_map="fade_out")), [20]),
    "FX-PPLANE-013": ("Fade In and Out: two fifths covering.",
                      case(**dict(ONE, opacity_map="fade_in_and_out")), [20]),
    "FX-PPLANE-014": ("Custom opacity 0, 100, 40, 100, 0: at a fifth of its life, between the "
                      "first two, about 80 per cent.",
                      case(**dict(ONE, opacity_map="custom", custom_opacity_1=0,
                                  custom_opacity_3=40, custom_opacity_5=0)), [20]),
    "FX-PPLANE-015": ("Custom opacity keyed: the second stop from 100 at frame 0 to 20 at frame "
                      "20, read at the frame drawn.",
                      case(**dict(ONE, opacity_map="custom",
                                  custom_opacity_2=keyed((0, 100), (20, 20)))), [20]),
    "FX-PPLANE-016": ("Birth and death colours blue to green.",
                      case(**dict(ONE, birth_color="#2040ff", death_color="#20ff40")), [20]),
    "FX-PPLANE-017": ("Origin to Death over the yellow column: from the drawing's yellow towards "
                      "red.", case(**dict(ORIGIN, color_map="origin_to_death",
                                           producer_x=-0.21875, producer_y=0.03125)), [20]),
    "FX-PPLANE-018": ("Birth to Origin over the blue column.",
                      case(**dict(ORIGIN, color_map="birth_to_origin",
                                  producer_x=0.09375, producer_y=-0.21875)), [20]),
    "FX-PPLANE-019": ("Origin Constant over the clear first column: black.",
                      case(**dict(ORIGIN, color_map="origin_constant",
                                  producer_x=-0.46875, producer_y=0.03125)), [20]),
    "FX-PPLANE-020": ("Custom colours white, yellow, orange, red, near-black: at a fifth of its "
                      "life, between white and yellow.",
                      case(**dict(ONE, color_map="custom", custom_color_1="#ffffff",
                                  custom_color_3="#ff8000", custom_color_5="#330000")), [20]),
    "FX-PPLANE-021": ("Two overlapping discs, Composite: the newer over the older.",
                      case(**TWO), [20]),
    "FX-PPLANE-022": ("The two, Screen.", case(**dict(TWO, transfer_mode="screen")), [20]),
    "FX-PPLANE-023": ("The two, Add.", case(**dict(TWO, transfer_mode="add")), [20]),
    "FX-PPLANE-024": ("Two overlapping stars, Composite.",
                      case(**dict(TWO, particle_type="star", max_opacity=80)), [20]),
    "FX-PPLANE-025": ("A burst of discs falling: Explosive with gravity, at frames 6, 12, 24 and "
                      "40. Frame 40 drawn alone is the same as after frames 0 to 39.",
                      case(**FLOW), [6, 12, 24, 40]),
    "FX-PPLANE-026": ("Resistance 3: the burst slows and stays tighter.",
                      case(**dict(FLOW, resistance=3)), [12, 24]),
    "FX-PPLANE-027": ("Direction, Extra Angle 30: a jet upwards within 15 degrees.",
                      case(**dict(FLOW, animation="direction", extra_angle=30,
                                  producer_y=0.2)), [12, 24]),
    "FX-PPLANE-028": ("Direction 90, Extra Angle 30: the jet to the right.",
                      case(**dict(FLOW, animation="direction", extra_angle=30, direction=90,
                                  producer_x=-0.3, gravity=0)), [12, 24]),
    "FX-PPLANE-029": ("Cone, 60 degrees: two jets, 30 degrees either side of up.",
                      case(**dict(FLOW, animation="cone", extra_angle=60, producer_y=0.2)),
                      [12, 24]),
    "FX-PPLANE-030": ("Viscous: Explosive held back hard.",
                      case(**dict(FLOW, animation="viscous")), [12, 24]),
    "FX-PPLANE-031": ("Twirl: swirling round the producer, clockwise on screen.",
                      case(**dict(FLOW, animation="twirl", gravity=0)), [12, 24]),
    "FX-PPLANE-032": ("Twirly: each particle swirling by its own amount.",
                      case(**dict(FLOW, animation="twirly", gravity=0)), [12, 24]),
    "FX-PPLANE-033": ("Vortex: swirling and pulled in.",
                      case(**dict(FLOW, animation="vortex", gravity=0)), [12, 24]),
    "FX-PPLANE-034": ("Fire: rising against gravity, narrowing and flickering.",
                      case(**dict(SLOW, animation="fire", producer_y=0.2)), [12, 24]),
    "FX-PPLANE-035": ("Jet Sideways: out to the left and right of up.",
                      case(**dict(FLOW, animation="jet_sideways", gravity=0)), [12, 24]),
    "FX-PPLANE-036": ("Fractal Omni: Explosive in the turbulence.",
                      case(**dict(SLOW, animation="fractal_omni")), [12, 24]),
    "FX-PPLANE-037": ("Fractal Uni: Direction in the turbulence.",
                      case(**dict(SLOW, animation="fractal_uni", extra_angle=40,
                                  producer_y=0.2)), [12, 24]),
    "FX-PPLANE-038": ("Inherit Velocity 100 with the producer keyed across: the particles carry "
                      "its motion.",
                      case(**dict(FLOW, velocity=0.1, gravity=0, inherit_velocity=100,
                                  producer_x=keyed((0, -0.4), (24, 0.4)))), [12, 24]),
    "FX-PPLANE-039": ("Direction keyed from 0 to 180 over frames 0 to 24 on Direction, Extra "
                      "Angle 20: the jet turns from up through right to down.",
                      case(**dict(FLOW, animation="direction", extra_angle=20, gravity=0,
                                  direction=keyed((0, 0), (24, 180)))), [12, 24]),
    "FX-PPLANE-040": ("Birth Rate keyed from 0 at frame 0 to 0.5 at frame 20: births start "
                      "slowly and quicken.",
                      case(**dict(FLOW, birth_rate=keyed((0, 0), (20, 0.5)))), [10, 20, 30]),
    "FX-PPLANE-041": ("Gravity keyed 0 to 3 over frames 0 to 24.",
                      case(**dict(FLOW, gravity=keyed((0, 0), (24, 3)))), [12, 24]),
    "FX-PPLANE-042": ("FX-PPLANE-025 with the layer starting at frame 10: nothing before, frame "
                      "22 the same as FX-PPLANE-025's frame 12.",
                      case(in_frame=10, **FLOW), [10, 22]),
    "FX-PPLANE-043": ("FX-PPLANE-025 with the layer moved 3 pixels right.",
                      case(shift=3, **FLOW), [12]),
    "FX-PPLANE-044": ("Random Seed 7: a different burst.", case(**dict(FLOW, random_seed=7)), [12]),
    "FX-PPLANE-045": ("Birth Rate 0: nothing.", case(birth_rate=0), [12]),
    "FX-PPLANE-046": ("Longevity 0: nothing.", case(longevity=0), [12]),
    "FX-PPLANE-047": ("Lines, larger: the streak's length follows the size.",
                      case(**dict(FLOW, particle_type="line", birth_size=4, death_size=4)), [12]),
    "FX-PPLANE-048": ("A burst of stars, faded out over life, added.",
                      case(**dict(FLOW, particle_type="star", opacity_map="fade_out",
                                  transfer_mode="add", birth_size=40, death_size=20)), [12]),
}
FRAMES_OF = {fx: frames for fx, (_, _, frames) in CASES.items()}
assert all(in_range(c) for _, c, _ in CASES.values())

INVALID = {
    "FX-PPLANE-049": ("Birth Rate -1, below 0.", case(birth_rate=-1)),
    "FX-PPLANE-050": ("Longevity 31, above 30.", case(longevity=31)),
    "FX-PPLANE-051": ("Birth Size 101, above 100.", case(birth_size=101)),
    "FX-PPLANE-052": ("Custom Opacity 3 at 120, above 100.", case(custom_opacity_3=120)),
    "FX-PPLANE-053": ("Animation \"direction_axis\": Particle Space's word, not the plane's.",
                      case(animation="direction_axis")),
    "FX-PPLANE-054": ("Animation \"Explosive\": the word is exact.", case(animation="Explosive")),
    "FX-PPLANE-055": ("Particle Type \"textured_square\": not one of the plane's six.",
                      case(particle_type="textured_square")),
    "FX-PPLANE-056": ("Opacity Map \"linear\".", case(opacity_map="linear")),
    "FX-PPLANE-057": ("Birth Color \"#12345\": not a colour.", case(birth_color="#12345")),
    "FX-PPLANE-058": ("Transfer Mode \"multiply\".", case(transfer_mode="multiply")),
}

LISTED = [("FX-PPLANE-025", 12), ("FX-PPLANE-026", 24), ("FX-PPLANE-028", 24),
          ("FX-PPLANE-029", 24), ("FX-PPLANE-031", 12), ("FX-PPLANE-033", 24),
          ("FX-PPLANE-034", 24), ("FX-PPLANE-035", 24), ("FX-PPLANE-037", 24),
          ("FX-PPLANE-038", 24), ("FX-PPLANE-039", 24), ("FX-PPLANE-040", 30)]


def listed():
    out = {"tolerance": 1e-7, "cases": {}}
    for fx, f in LISTED:
        run = Run(CASES[fx][1], f)
        rows = []
        for k, b in P.births(run):
            if not run.u - b < run.at("longevity", b) * FPS:
                continue
            p = moved(run, k, b)
            rows.append([k, b] + p["x"] + p["v"])
        out["cases"][fx] = {"frame": f, "particles": rows}
        print(f"{fx} frame {f}: {len(rows)} particles listed")
    (OUT / "particles_particle_plane.json").write_text(json.dumps(out, indent=1) + "\n",
                                                      encoding="utf-8")


def clear():
    """No number lands so near a cut that the build's rounding could put it on the other side."""
    px = P.working()
    for fx, (_, c, frames) in CASES.items():
        for f in frames:
            run = Run(c, f)
            if run.u < 0:
                continue
            for k, b in P.births(run):
                assert abs(b - run.u) > 1e-6, (fx, f, k)
                assert abs((run.u - b) - run.at("longevity", b) * FPS) > 1e-6, (fx, f, k)
            for _, sp, p in sprites(run, px):
                if c["color_map"] not in ("birth_to_death", "custom"):
                    for v in flat(p["x0"]):
                        assert abs(v - round(v)) > 1e-6, (fx, v)
                if sp["kind"] == "star":
                    # no pixel centre on the star's outline, where in or out turns
                    for gy in range(H):
                        for gx in range(W):
                            dx, dy = gx + 0.5 - sp["X"], gy + 0.5 - sp["Y"]
                            pts = [(sx * sp["R"], sy * sp["R"]) for sx, sy in STAR]
                            for j in range(10):
                                (ax, ay), (bx, by) = pts[j], pts[(j + 1) % 10]
                                assert abs(ay - dy) > 1e-9, (fx, gx, gy)
    print("clear")


def check(expected):
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    clear_frame = [[0.0] * 4 for _ in range(W * H)]
    total = lambda fr: sum(p[3] for p in fr)  # noqa: E731
    assert c["FX-PPLANE-001"]["0"] == clear_frame
    assert total(c["FX-PPLANE-001"]["24"]) > 0
    one = c["FX-PPLANE-002"]["20"]
    assert one[5 * W + 8][3] == 1.0  # the pixel right of the middle, inside the dot
    assert total(c["FX-PPLANE-003"]["20"]) < total(one)
    assert total(c["FX-PPLANE-006"]["20"]) < total(one)
    assert total(c["FX-PPLANE-008"]["20"]) < total(c["FX-PPLANE-006"]["20"])
    assert total(c["FX-PPLANE-009"]["20"]) < total(one)
    for fx, share in (("FX-PPLANE-011", 10.77 / 48), ("FX-PPLANE-012", 1 - 10.77 / 48)):
        assert abs(total(c[fx]["20"]) - share * total(one)) < 0.02 * total(one), fx
    # the star covers its middle and leaves the corners of its box clear
    st = c["FX-PPLANE-006"]["20"]
    assert st[5 * W + 8][3] == 1.0 and st[1 * W + 4][3] == 0 and st[1 * W + 11][3] == 0
    assert c["FX-PPLANE-007"]["20"] == one
    assert c["FX-PPLANE-014"]["20"] != c["FX-PPLANE-015"]["20"]
    dark = c["FX-PPLANE-019"]["20"]
    assert all(p[0] == 0 and p[1] == 0 and p[2] == 0 for p in dark) and total(dark) > 0
    comp, scr, ad = (c["FX-PPLANE-%03d" % n]["20"] for n in (21, 22, 23))
    assert comp != scr != ad != comp
    assert c["FX-PPLANE-025"]["40"] != c["FX-PPLANE-025"]["24"]
    for n in range(26, 42):
        assert c["FX-PPLANE-%03d" % n] != c["FX-PPLANE-025"], n
    assert c["FX-PPLANE-042"]["10"] == clear_frame
    assert c["FX-PPLANE-042"]["22"] == c["FX-PPLANE-025"]["12"]
    moved_ = c["FX-PPLANE-043"]["12"]
    base = c["FX-PPLANE-025"]["12"]
    for y in range(H):
        assert moved_[y * W + 3:(y + 1) * W] == base[y * W:y * W + W - 3]
    assert c["FX-PPLANE-044"]["12"] != base
    assert c["FX-PPLANE-045"]["12"] == clear_frame
    assert c["FX-PPLANE-046"]["12"] == clear_frame
    # nothing ever leaves the plane
    for fx, f in LISTED:
        run = Run(CASES[fx][1], f)
        for k, b in P.births(run):
            p = moved(run, k, b)
            assert p["x"][2] == 0.0 and p["v"][2] == 0.0, fx
    # Direction 90 throws to the right; Direction 0 throws up
    run = Run(CASES["FX-PPLANE-028"][1], 24)
    assert all(born(run, k, b)["v"][0] > 0 for k, b in P.births(run))
    run = Run(CASES["FX-PPLANE-027"][1], 24)
    assert all(born(run, k, b)["v"][1] < 0 for k, b in P.births(run))
    # twirl turns clockwise on screen: a particle right of the producer is pushed down
    run = Run(CASES["FX-PPLANE-031"][1], 12)
    p = {"x": [0.1, 0.0, 0.0], "v": [0.0, 0.0, 0.0], "twirl": 1.0}
    step(run, p, 6.0, 1 / FPS)
    assert p["v"][1] > 0 and abs(p["v"][0]) < 1e-12
    # the star's outline is the rule's: an outer point on the circle, an inner one at 0.4
    assert all(abs(math.hypot(*STAR[j]) - (1 if j % 2 == 0 else 0.4)) < 1e-15 for j in range(10))
    print("checked")


def main():
    clear()
    expected = B.write_cases(OUT, "particle_plane", CASES, INVALID, render, plain, file_json)
    listed()
    check(expected)


if __name__ == "__main__":
    main()
