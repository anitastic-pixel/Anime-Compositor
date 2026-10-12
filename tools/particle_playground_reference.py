"""Particle Playground, worked a second way.

`core.particle_playground` (D-613), the particle engine's third effect (P0-6), after After
Effects' Particle Playground (Simulation): dots shot from a Cannon, laid out by a Grid, cut from a
layer by the Layer Exploder, split again by the Particle Exploder, shown as a layer by the Layer
Map, pulled by Gravity and kept in by a Wall. Adobe's page says what each control is for and
gives no formula, so every rule and every number when added below is ours (document 21, D-613).
No text or code was copied. The property mappers come in a second step (B-494); Repel, the text
particles and the Layer Map's time offsets are not built (D-613 says why).

**No state is carried from frame to frame** (document 20): a particle's place at a frame is a
pure function of the seed, the particle's own numbers and the settings sampled once a frame from
the in point, stepped on the engine's fixed grid of 4 steps a frame (D-610).

The settings are SETTINGS (name, lowest, highest, value when added; all keyable), POINTS (two
numbers, per cent of the layer's width and height; keyable), WORDS (exact), LAYERS (D-189's
layer word, "" for none) and COLOURS (`#rrggbb`). Lengths are in the layer's pixels (a draft
divides them as it divides every distance), speeds in pixels a second, Gravity in pixels a second
a second, times in seconds, angles in degrees with 0 up and 90 to the right. With n the frame
drawn, i0 the layer's in point, u = n - i0 and fps the composition's rate:

- Samples. As D-610's: each number sampled at frames i0 + i, i = 0 to u + 1, held inside its
  range, read straight-line between samples. Nothing is born before the in point; before it the
  layer is not drawn.
- Randomness. r(p, s) = (unit(seed, a, b, code, s) + 1) / 2, unit noise_reference's and seed the
  whole part of Random Seed at n; a particle p is (code, a, b): the Cannon's k-th (1, k, 0), the
  Grid's j-th of frame i (2, j, i), the Layer Exploder's piece j (3, j, 0); the c-th child of a
  particle of code s keeps its a and b and has code s + 4 (c + 1). Streams: 0 and 1 the barrel,
  3 the direction, 5 the speed, 7 the gravity spread, 20 + g the g-th Affects (g = 0 Particle
  Exploder, 1 Layer Map, 2 Gravity, 3 Wall).
- Cannon. Born as D-610's births at Particles Per Second / fps a frame (Birth Rate's integral,
  straight-line between frames), particle k at b_k, with nothing ever dying of age (After
  Effects' particles live on). At b_k: the place C = (Position_x W / 100, Position_y H / 100),
  then with the Barrel Radius r: r < 0 a round barrel, C + |r| sqrt(r0)(cos 2 pi r1, sin 2 pi r1);
  r > 0 a square one, C + r (2 r0 - 1, 2 r1 - 1). The direction d + s (r3 - 1/2), d Direction and
  s Direction Random Spread; the speed V + S (r5 - 1/2), V Velocity and S Velocity Random
  Spread; the velocity speed (sin t, -cos t) for the direction t. The colour Cannon Color, opaque,
  and the radius Particle Radius at b_k.
- Grid. At every whole frame i = 0 to u a whole grid is born (After Effects' grid makes
  particles continuously): with n_x = floor(Particles Across) and n_y = floor(Particles Down) at
  i, particle j = row n_x + column at (C_x + Width ((column + 1/2) / n_x - 1/2), C_y + Height
  ((row + 1/2) / n_y - 1/2)), C the Grid Position as the Cannon's, still, Grid Color, its Particle
  Radius at i.
- Layer Exploder. The Explode Layer's map (D-189, fitted by `center`), read once at the in point,
  as Wave World reads its ground (D-630), is cut into squares of side q = 2 Radius Of New
  Particles (at the in point), square j = row m + column (m = ceil(W / q) across) from
  (column q, row q). A square is a piece when a pixel whose centre lies in it is not clear. All
  are born at the in point, still where they were cut, then thrown: direction 360 r3 degrees,
  speed Velocity Dispersion r5. Its colour, for its children, the mean of those pixels (as
  premultiplied).
- Steps. On D-610's grid: a particle steps from its birth to the first quarter frame after it,
  then quarter to quarter up to u, each step h = (t1 - t0) / fps seconds with the settings at t0.
  Before each step (and at the birth) the Particle Exploder is asked (below). A step moves
  x' = x + v h + a h^2 / 2, v' = v + a h, a = the Gravity when it affects p at t0: (Force + Force
  Random Spread (r7 - 1/2)) (sin g, -cos g), g Gravity's Direction; else 0. Then the Wall, when
  it affects p at t0: the straight move from x to x' is cut at the first edge of the wall it
  crosses (at fraction f along it, the edge just left not counted); there x is that point, v the
  velocity at f h turned off the edge (v - 2 (v . n) n, n the edge's unit normal), and the rest
  of the step, (1 - f) h, is moved again; after 4 such bounces in one step the particle stays
  where it is for the rest of it.
- The Wall is the holder's mask numbered Wall's Mask (1 the first, its floor taken; 0 none) at
  frame n, switched on and of 3 points or more, flattened as document 19 flattens it (closed).
  A number naming no such mask is no wall, and EFFECT_PATH_MISSING is said every frame.
- Affects, for each of the four: Particles From (which emitters' particles: `all`, one, two, or
  all but one; a child counts as the Particle Exploder's), Selection Map, Older/Younger Than T and
  Age Feather F. The weight is the product of: 1 or 0 by Particles From; by age a (seconds since
  its birth), T = 0 gives 1, T > 0 (older than) and T < 0 (younger than |T|) take d = a - T and
  d = -T - a, F = 0 gives 1 when d >= 0 else 0, F > 0 gives clamp(d / F + 1/2); and with a map
  named, the map's luminance (compound blur's, sRGB-coded) at the pixel under the particle
  (floor x, floor y), 0 outside. The maps are read at the frame drawn. p is affected when its
  weight is above r(p, 20 + g), so a feather picks some particles and then more.
- Particle Exploder, with Radius Of New Particles c > 0: a particle that is not itself a child,
  asked at time t (its birth or a quarter frame) and affected, is gone from t and its
  N = clamp(floor((R / c)^2 + 1/2), 1, 100) children are born there, R its radius (a piece's
  q / 2): each at its place with its velocity plus Velocity Dispersion r5 (sin, -cos)(360 r3
  degrees), radius c, the parent's colour. Children never explode.
- Drawing, onto a clear layer (After Effects' playground hides the layer's own picture), in the
  order born (by birth time, then Cannon, Grid, pieces, children, then the numbers), each laid
  over what is there. A particle the Layer Map affects at n is the map's picture (D-189's,
  `center`, at n), its middle (W/2, H/2) on the particle: the colour at pixel centre P the
  map's bilinear sample at (W/2, H/2) + L, L = Rot(-o)(P - X), X the particle. A piece is its
  square: covering clamp(q / 2 - |L_x| + 1/2) clamp(q / 2 - |L_y| + 1/2), colour the Explode
  map's bilinear sample at O + L, each part of L held within q / 2 - 1/2 (O the square's centre
  where it was cut). A dot is D-610's disc: covering clamp(R - e + 1/2), e the distance from X,
  with R < 0.7 drawn at 0.7 and its colour times (R / 0.7)^2. o is 0 unless Auto-Orient Rotation
  is on, then the way the particle moves, atan2(v_x, -v_y) (0 when still); Rot(o)(x, y) =
  (x cos o - y sin o, x sin o + y cos o), so a picture's top faces the way it moves.

**This file never runs the build's code path.** It works in double precision on lists from the
drawings' 8-bit values; trigonometry is the platform's. Every case is a project of one
composition 16 by 10 at 24 frames a second, in `Fixtures/particle_playground/`: the map layers,
switched off, then the layer `holder`, showing Broadcast Colors' drawing, with the effect. The
expected frames go into `Fixtures/particle_playground/expected_particle_playground.json` and each
particle's numbers for some cases into `particles_particle_playground.json`. Every case is "not
built yet" until B-493 lands (D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/particle_playground_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from noise_reference import u as U  # noqa: E402
from motion_blur_reference import png  # noqa: E402
from mask_reference import points, mask_json  # noqa: E402
import particle_space_reference as P  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import recolor_reference as R  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import compound_blur_reference as K  # noqa: E402

W, H, FPS = B.W, B.H, 24
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "particle_playground"
TOLERANCE = 2e-5
GROUPS = ("particle_exploder", "layer_map", "gravity", "wall")
CODES = {"cannon": 1, "grid": 2, "layer_exploder": 3, "particle_exploder": 4}

SETTINGS = [
    ("barrel_radius", -2500, 2500, 0), ("particles_per_second", 0, 1000, 20),
    ("cannon_direction", -36000, 36000, 0), ("direction_random_spread", 0, 360, 20),
    ("velocity", -10000, 10000, 130), ("velocity_random_spread", 0, 10000, 20),
    ("cannon_particle_radius", 0, 1000, 2),
    ("grid_width", 0, 10000, 100), ("grid_height", 0, 10000, 100),
    ("particles_across", 0, 100, 0), ("particles_down", 0, 100, 0),
    ("grid_particle_radius", 0, 1000, 2),
    ("explode_radius", 0.5, 100, 2), ("explode_dispersion", 0, 10000, 20),
    ("particle_exploder_radius", 0, 1000, 0), ("particle_exploder_dispersion", 0, 10000, 20),
    ("gravity_force", -10000, 10000, 100), ("gravity_random_spread", 0, 10000, 0),
    ("gravity_direction", -36000, 36000, 180),
    ("wall_mask", 0, 1000, 0),
] + [(f"{g}_{n}", lo, hi, 0) for g in GROUPS
     for n, lo, hi in (("older_younger_than", -1000, 1000), ("age_feather", 0, 1000))] + [
    ("random_seed", 0, 10000, 0),
]
NAMES = [s[0] for s in SETTINGS]
RANGES = {s[0]: (s[1], s[2]) for s in SETTINGS}
POINTS = [("cannon_position", [50, 50]), ("grid_position", [50, 50])]
FROM = {
    "all": {1, 2, 3, 4}, "cannon": {1}, "grid": {2}, "layer_exploder": {3},
    "particle_exploder": {4}, "cannon_grid": {1, 2}, "cannon_layer_exploder": {1, 3},
    "cannon_particle_exploder": {1, 4}, "grid_layer_exploder": {2, 3},
    "grid_particle_exploder": {2, 4}, "layer_exploder_particle_exploder": {3, 4},
    "all_but_cannon": {2, 3, 4}, "all_but_grid": {1, 3, 4},
    "all_but_layer_exploder": {1, 2, 4}, "all_but_particle_exploder": {1, 2, 3},
}
WORDS = [(f"{g}_particles_from", tuple(FROM), "all") for g in GROUPS] + [
    ("auto_orient", ("off", "on"), "off")]
LAYERS = ["explode_layer", "layer_map"] + [f"{g}_selection_map" for g in GROUPS]
COLOURS = [("cannon_color", "#ff0000"), ("grid_color", "#ffff00")]


# --- the drawings and the map layers --------------------------------------------------------

def logo_drawing():
    """4 by 4: a white top row, then red and green halves over a blue and yellow row."""
    rows = [[(255, 255, 255, 255)] * 4,
            [(255, 0, 0, 255)] * 2 + [(0, 255, 0, 255)] * 2,
            [(255, 0, 0, 255)] * 2 + [(0, 255, 0, 255)] * 2,
            [(0, 0, 255, 255)] * 2 + [(255, 255, 0, 255)] * 2]
    return rows


def sel_drawing():
    """The selection map: the left half white, the right half black."""
    return [[(255, 255, 255, 255) if x < 8 else (0, 0, 0, 255) for x in range(W)]
            for y in range(H)]


DRAWINGS = {"art": B.DRAWINGS["colours"], "logo": logo_drawing(), "sel": sel_drawing()}
# map layer name -> (drawing, its in point)
MAP_LAYERS = {"logo": ("logo", 0), "sel": ("sel", 0), "late": ("logo", 4)}


def decoded(name):
    rows = DRAWINGS[name]
    return L.pic(len(rows[0]), len(rows), [R.working(p) for row in rows for p in row])


def map_of(name, frame):
    """D-189's map at `frame`, fitted by `center`; None for no map (none named, or not a layer
    of the composition: the latter says EFFECT_LAYER_MISSING)."""
    if name == "holder":
        return decoded("art")
    if name not in MAP_LAYERS:
        return None
    drawing, start = MAP_LAYERS[name]
    p = L.EMPTY if frame < start else decoded(drawing)
    return L.fit(p, "center", W, H)


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


class Run(P.Run):
    def __init__(self, c, n):
        self.c = c
        self.u = n - c["in_frame"]
        self.samples = [sample(c, c["in_frame"] + i) for i in range(max(0, self.u) + 2)]
        self.now = self.samples[max(0, self.u)]
        self.seed = math.floor(self.now["random_seed"])
        self.maps = {g: map_of(c[f"{g}_selection_map"], n) for g in GROUPS}
        self.layer_map = map_of(c["layer_map"], n)
        self.explode = map_of(c["explode_layer"], c["in_frame"])
        self.wall = wall_of(c, n)

    def rr(self, p, s):
        return (U(self.seed, p["a"], p["b"], p["code"], s) + 1) / 2


def wall_of(c, n):
    k = math.floor(sample(c, n)["wall_mask"])
    if k < 1 or k > len(c["masks"]):
        return None
    pts = [q["point"] for q in c["masks"][k - 1]["points"]]
    return pts if len(pts) >= 3 else None


class Rate:
    """The Cannon's rate as D-610's births read it."""

    def __init__(self, run):
        self.u = run.u
        self.samples = [{"birth_rate": s["particles_per_second"] / 100, "longevity": 1e9}
                        for s in run.samples]


def centre(run, name, t):
    return (run.at(name + "_x", t) * W / 100, run.at(name + "_y", t) * H / 100)


def heading(deg):
    t = math.radians(deg)
    return (math.sin(t), -math.cos(t))


def particle(code, a, b, born, x, v, col, rad, **more):
    return dict(code=code, a=a, b=b, born=born, x=list(x), v=list(v), col=col, rad=rad,
                src=code if code <= 3 else 4, **more)


def cannon(run):
    out = []
    for k, b in P.births(Rate(run)):
        p = particle(1, k, 0, b, (0, 0), (0, 0), None, run.at("cannon_particle_radius", b))
        cx, cy = centre(run, "cannon_position", b)
        r = run.at("barrel_radius", b)
        if r < 0:
            rho, turn = -r * math.sqrt(run.rr(p, 0)), 2 * math.pi * run.rr(p, 1)
            x = (cx + rho * math.cos(turn), cy + rho * math.sin(turn))
        else:
            x = (cx + r * (2 * run.rr(p, 0) - 1), cy + r * (2 * run.rr(p, 1) - 1))
        d = run.at("cannon_direction", b) + run.at("direction_random_spread", b) * (
            run.rr(p, 3) - 0.5)
        speed = run.at("velocity", b) + run.at("velocity_random_spread", b) * (run.rr(p, 5) - 0.5)
        hx, hy = heading(d)
        p["x"], p["v"] = list(x), [speed * hx, speed * hy]
        p["col"] = R.working(R.hex_color(run.c["cannon_color"]) + (255,))
        out.append(p)
    return out


def grid(run):
    out = []
    for i in range(run.u + 1):
        s = run.samples[i]
        nx, ny = math.floor(s["particles_across"]), math.floor(s["particles_down"])
        cx, cy = s["grid_position_x"] * W / 100, s["grid_position_y"] * H / 100
        for row in range(ny):
            for col in range(nx):
                x = (cx + s["grid_width"] * ((col + 0.5) / nx - 0.5),
                     cy + s["grid_height"] * ((row + 0.5) / ny - 0.5))
                out.append(particle(2, row * nx + col, i, float(i), x, (0, 0),
                                    R.working(R.hex_color(run.c["grid_color"]) + (255,)),
                                    s["grid_particle_radius"]))
    return out


def pieces(run):
    m = run.explode
    if m is None:
        return []
    q = 2 * run.samples[0]["explode_radius"]
    across, down = math.ceil(W / q), math.ceil(H / q)
    out = []
    for row in range(down):
        for col in range(across):
            inside = [m["px"][y * W + x] for y in range(H) for x in range(W)
                      if col * q <= x + 0.5 < (col + 1) * q and row * q <= y + 0.5 < (row + 1) * q]
            if not any(px[3] > 0 for px in inside):
                continue
            mean = [sum(px[ch] for px in inside) / len(inside) for ch in range(4)]
            o = ((col + 0.5) * q, (row + 0.5) * q)
            p = particle(3, row * across + col, 0, 0.0, o, (0, 0), mean, q / 2, origin=o, side=q)
            speed = run.samples[0]["explode_dispersion"] * run.rr(p, 5)
            hx, hy = heading(360 * run.rr(p, 3))
            p["v"] = [speed * hx, speed * hy]
            out.append(p)
    return out


# --- affects, the exploder and the steps ----------------------------------------------------

def age_weight(t_, f_, age):
    if t_ == 0:
        return 1.0
    d = age - t_ if t_ > 0 else -t_ - age
    if f_ == 0:
        return 1.0 if d >= 0 else 0.0
    return P.clamp(d / f_ + 0.5)


def weight(run, gi, p, t):
    g = GROUPS[gi]
    if p["src"] not in FROM[run.c[f"{g}_particles_from"]]:
        return 0.0
    age = (t - p["born"]) / FPS
    w = age_weight(run.at(f"{g}_older_younger_than", t), run.at(f"{g}_age_feather", t), age)
    m = run.maps[g]
    if m is not None:
        gx, gy = math.floor(p["x"][0]), math.floor(p["x"][1])
        w *= K.luma(L.at(m, gx, gy)) if 0 <= gx < W and 0 <= gy < H else 0.0
    return w


def affected(run, gi, p, t):
    return weight(run, gi, p, t) > run.rr(p, 20 + gi)


def children(run, p, t):
    c = run.at("particle_exploder_radius", t)
    n = min(100, max(1, math.floor((p["rad"] / c) ** 2 + 0.5)))
    out = []
    for i in range(n):
        ch = particle(p["code"] + 4 * (i + 1), p["a"], p["b"], t, p["x"], p["v"], p["col"], c,
                      parent=p["code"])
        speed = run.at("particle_exploder_dispersion", t) * run.rr(ch, 5)
        hx, hy = heading(360 * run.rr(ch, 3))
        ch["v"] = [p["v"][0] + speed * hx, p["v"][1] + speed * hy]
        out.append(ch)
    return out


def cross2(a, b):
    return a[0] * b[1] - a[1] * b[0]


HITS = []  # every wall crossing's (f, s) and near-misses, for clear()


def bounce_move(run, p, a, h):
    x, v = p["x"], p["v"]
    wall = run.wall
    skip = None
    for _ in range(4):
        x1 = [x[i] + v[i] * h + a[i] * h * h / 2 for i in range(2)]
        d = [x1[0] - x[0], x1[1] - x[1]]
        best = None
        for j in range(len(wall)):
            if j == skip:
                continue
            A, Bp = wall[j], wall[(j + 1) % len(wall)]
            e = [Bp[0] - A[0], Bp[1] - A[1]]
            den = cross2(d, e)
            if den == 0:
                continue
            w0 = [A[0] - x[0], A[1] - x[1]]
            f = cross2(w0, e) / den
            s = cross2(w0, d) / den
            HITS.append((f, s))
            if 0 < f <= 1 and 0 <= s <= 1 and (best is None or f < best[0]):
                best = (f, j, e)
        if best is None:
            p["x"], p["v"] = x1, [v[i] + a[i] * h for i in range(2)]
            return
        f, j, e = best
        tau = f * h
        xc = [x[i] + f * d[i] for i in range(2)]
        vc = [v[i] + a[i] * tau for i in range(2)]
        le = math.hypot(e[0], e[1])
        nrm = [-e[1] / le, e[0] / le]
        vn = vc[0] * nrm[0] + vc[1] * nrm[1]
        x, v = xc, [vc[0] - 2 * vn * nrm[0], vc[1] - 2 * vn * nrm[1]]
        h -= tau
        skip = j
    p["x"], p["v"] = x, v


def step(run, p, t0, h):
    a = [0.0, 0.0]
    if affected(run, 2, p, t0):
        f = run.at("gravity_force", t0) + run.at("gravity_random_spread", t0) * (run.rr(p, 7) - 0.5)
        gx, gy = heading(run.at("gravity_direction", t0))
        a = [f * gx, f * gy]
    if run.wall is not None and affected(run, 3, p, t0):
        bounce_move(run, p, a, h)
        return
    x, v = p["x"], p["v"]
    p["x"] = [x[i] + v[i] * h + a[i] * h * h / 2 for i in range(2)]
    p["v"] = [v[i] + a[i] * h for i in range(2)]


def points_from(t, u):
    return [t] + [j / 4 for j in range(math.floor(4 * t) + 1, 4 * u + 1)]


def moved(run, p):
    """Particle p as it is at u: a list of itself, or of its children once it exploded."""
    pts = points_from(p["born"], run.u)
    explodes = p["code"] <= 3
    for i, t0 in enumerate(pts):
        if explodes and run.at("particle_exploder_radius", t0) > 0 and affected(run, 0, p, t0):
            out = []
            for ch in children(run, p, t0):
                out += moved(run, ch)
            return out
        if i + 1 < len(pts):
            step(run, p, t0, (pts[i + 1] - t0) / FPS)
    return [p]


def everyone(run):
    out = []
    for p in cannon(run) + grid(run) + pieces(run):
        out += moved(run, p)
    order = {1: 0, 2: 1, 3: 2}
    out.sort(key=lambda p: (p["born"], order.get(p["code"], 3), p.get("parent", 0), p["a"],
                            p["b"], p["code"]))
    return out


# --- the drawing ----------------------------------------------------------------------------

def turn_of(run, p):
    if run.c["auto_orient"] != "on" or (p["v"][0] == 0 and p["v"][1] == 0):
        return 0.0
    return math.atan2(p["v"][0], -p["v"][1])


def local(o, dx, dy):
    """Rot(-o) of (dx, dy)."""
    cs, sn = math.cos(o), math.sin(o)
    return (dx * cs + dy * sn, -dx * sn + dy * cs)


def lay(d, s):
    return [s[i] + (1 - s[3]) * d[i] for i in range(4)]


def playground(c, n):
    out = [[0.0] * 4 for _ in range(W * H)]
    run = Run(c, n)
    if run.u < 0:
        return out
    for p in everyone(run):
        X, Y = p["x"]
        o = turn_of(run, p)
        if run.layer_map is not None and affected(run, 1, p, run.u):
            m = run.layer_map
            for gy in range(H):
                for gx in range(W):
                    lx, ly = local(o, gx + 0.5 - X, gy + 0.5 - Y)
                    s = L.bilinear(m, W / 2 + lx, H / 2 + ly)
                    if s[3] > 0:
                        out[gy * W + gx] = lay(out[gy * W + gx], s)
            continue
        if p["code"] == 3:
            q = p["side"]
            hold = q / 2 - 0.5
            for gy in range(H):
                for gx in range(W):
                    lx, ly = local(o, gx + 0.5 - X, gy + 0.5 - Y)
                    cv = P.clamp(q / 2 - abs(lx) + 0.5) * P.clamp(q / 2 - abs(ly) + 0.5)
                    if cv <= 0:
                        continue
                    s = L.bilinear(run.explode, p["origin"][0] + P.clamp(lx, -hold, hold),
                                   p["origin"][1] + P.clamp(ly, -hold, hold))
                    out[gy * W + gx] = lay(out[gy * W + gx], [v * cv for v in s])
            continue
        rad, k = p["rad"], 1.0
        if rad < 0.7:
            k, rad = (rad / 0.7) ** 2, 0.7
        for gy in range(H):
            for gx in range(W):
                cv = P.clamp(rad - math.hypot(gx + 0.5 - X, gy + 0.5 - Y) + 0.5) * k
                if cv > 0:
                    out[gy * W + gx] = lay(out[gy * W + gx], [v * cv for v in p["col"]])
    return out


# --- the cases ------------------------------------------------------------------------------

def case(shift=0, in_frame=0, masks=(), **given):
    c = {s[0]: s[3] for s in SETTINGS}
    c.update({n: list(v) for n, v in POINTS})
    c.update({w[0]: w[2] for w in WORDS})
    c.update({n: "" for n in LAYERS})
    c.update(dict(COLOURS))
    for k, v in given.items():
        assert k in c, k
        c[k] = v
    c.update(shift=shift, in_frame=in_frame, masks=list(masks))
    return c


def in_range(c):
    for n, lo, hi, _ in SETTINGS:
        v = c[n]
        for x in ([k[1] for k in v["keys"]] if isinstance(v, dict) else [v]):
            if not lo <= x <= hi:
                return False
    return True


def shifted(px, shift):
    return [list(px[y * W + x - shift]) if 0 <= x - shift < W else [0.0] * 4
            for y in range(H) for x in range(W)]


def render(c, frame_no):
    return shifted(playground(c, frame_no), c["shift"])


def plain(c):
    return shifted(decoded("art")["px"], c["shift"])


def project_json(fx, c, frames):
    params = {n: setting_json(c[n]) for n in NAMES}
    params.update({n: setting_json(c[n]) for n, _ in POINTS})
    params.update({w[0]: c[w[0]] for w in WORDS})
    params.update({n: c[n] for n in LAYERS})
    params.update({n: c[n] for n, _ in COLOURS})
    holder = L.raster("holder", "asset-art", position=(c["shift"], 0), in_frame=c["in_frame"],
                      out_frame=frames)
    holder["effects"] = [{"instance_id": "fx-1", "type_id": "core.particle_playground",
                          "enabled": True, "parameters": params}]
    if c["masks"]:
        items = [(k, v) for k, v in holder.items() if k != "mask"]
        at = [k for k, _ in items].index("matte")
        items.insert(at, ("masks", [mask_json(m, i + 1) for i, m in enumerate(c["masks"])]))
        holder = dict(items)
    maps = [L.raster(name, "asset-" + d, enabled=False, in_frame=start, out_frame=frames)
            for name, (d, start) in MAP_LAYERS.items()]
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(n, f"media/{n}.png") for n in DRAWINGS],
            "compositions": [L.composition("comp-main", W, H, frames, maps + [holder])]}


def none(*xy):
    return {"points": points(*xy), "mode": "none"}


BOX = none([1.5, 1.5], [14.5, 1.5], [14.5, 8.5], [1.5, 8.5])

# One still red dot in the middle: one a second, the first born at frame 12; radius 3.
ONE = dict(particles_per_second=1, velocity=0, velocity_random_spread=0, gravity_force=0,
           cannon_particle_radius=3, direction_random_spread=0)
# A slow fountain: 6 a second from near the bottom, upwards at 20 pixels a second, falling at
# 30 pixels a second a second, dots of radius 0.8.
FOUNTAIN = dict(particles_per_second=6, velocity=20, velocity_random_spread=6,
                direction_random_spread=40, gravity_force=30, cannon_particle_radius=0.8,
                cannon_position=[50, 85])
# A grid of 4 by 2 dots, radius 1, still.
GRID = dict(particles_per_second=0, particles_across=4, particles_down=2, grid_width=12,
            grid_height=6, gravity_force=0, grid_particle_radius=1)
# The holder blown apart: 2-pixel pieces, 4 pixels a second, no gravity.
BLAST = dict(particles_per_second=0, explode_layer="holder", explode_radius=1,
             explode_dispersion=4, gravity_force=0)
# Fireworks: one shell a second shot straight up, bursting into 16 sparks at half a second.
SHELL = dict(particles_per_second=1, velocity=12, velocity_random_spread=0,
             direction_random_spread=0, gravity_force=10, cannon_particle_radius=2,
             cannon_position=[50, 90], particle_exploder_radius=0.5,
             particle_exploder_dispersion=8, particle_exploder_older_younger_than=0.51)
# A box wall of mask 1 and dots thrown about inside it.
WALLED = dict(FOUNTAIN, wall_mask=1, cannon_position=[50, 50], velocity=30,
              direction_random_spread=360, gravity_force=0)

CASES = {
    "PG-001": ("The settings as added, at frames 0, 2 and 6: 20 red dots a second shot upwards "
               "from the middle at about 130 pixels a second, so fast they are out of the 16 "
               "by 10 layer within frames; the layer's own picture is gone.",
               case(), [0, 2, 6]),
    "PG-002": ("One still red dot of radius 3 in the middle, born at frame 12: nothing at frame "
               "11, the dot at frame 13 and the same at frame 30 (it never dies).",
               case(**ONE), [11, 13, 30]),
    "PG-003": ("A tiny dot, radius 0.35: drawn at 0.7 and a quarter as strong.",
               case(**dict(ONE, cannon_particle_radius=0.35)), [13]),
    "PG-004": ("A round barrel of radius 4 (Barrel Radius -4): dots born scattered in a disc "
               "about the middle.",
               case(**dict(ONE, particles_per_second=48, barrel_radius=-4,
                           cannon_particle_radius=0.8)), [10]),
    "PG-005": ("A square barrel of radius 4: scattered in a square.",
               case(**dict(ONE, particles_per_second=48, barrel_radius=4,
                           cannon_particle_radius=0.8)), [10]),
    "PG-006": ("The fountain: shot upwards from near the bottom, spreading 20 degrees either "
               "side, slowing and falling back, at frames 12, 24 and 40. Frame 40 drawn alone is "
               "the same as after frames 0 to 39.", case(**FOUNTAIN), [12, 24, 40]),
    "PG-007": ("Direction 90: the fountain thrown to the right, falling.",
               case(**dict(FOUNTAIN, cannon_direction=90, cannon_position=[20, 40])), [24]),
    "PG-008": ("Direction keyed from -60 to 60 over frames 0 to 24: the jet sweeps from upper "
               "left to upper right.",
               case(**dict(FOUNTAIN, direction_random_spread=0, gravity_force=0,
                           cannon_direction=keyed((0, -60), (24, 60)))), [24]),
    "PG-009": ("Particles Per Second keyed from 0 to 24 over frames 0 to 20.",
               case(**dict(FOUNTAIN, particles_per_second=keyed((0, 0), (20, 24)))), [12, 24]),
    "PG-010": ("Cannon Color blue.", case(**dict(FOUNTAIN, cannon_color="#2040ff")), [24]),
    "PG-011": ("Gravity Direction 90: falling to the right.",
               case(**dict(FOUNTAIN, gravity_direction=90)), [24]),
    "PG-012": ("Gravity Force Random Spread 40: each dot falls at its own rate.",
               case(**dict(FOUNTAIN, gravity_random_spread=40)), [24]),
    "PG-013": ("Gravity keyed from 0 to 60 over frames 0 to 24.",
               case(**dict(FOUNTAIN, gravity_force=keyed((0, 0), (24, 60)))), [24]),
    "PG-014": ("Gravity only on dots older than about half a second (0.51): they rise straight, "
               "then fall.", case(**dict(FOUNTAIN, gravity_older_younger_than=0.51)), [24]),
    "PG-015": ("Gravity only on dots younger than about half a second (0.51).",
               case(**dict(FOUNTAIN, gravity_older_younger_than=-0.51)), [24]),
    "PG-016": ("Gravity older than half a second with an Age Feather of 0.6: some dots start "
               "falling earlier, some later.",
               case(**dict(FOUNTAIN, gravity_older_younger_than=0.5, gravity_age_feather=0.6)),
               [24]),
    "PG-017": ("Gravity through the selection map: only dots over its white left half fall.",
               case(**dict(FOUNTAIN, direction_random_spread=120, gravity_selection_map="sel",
                           cannon_position=[47, 85])),
               [24]),
    "PG-018": ("Gravity from the Grid only: the Cannon's dots fly on unbent.",
               case(**dict(FOUNTAIN, gravity_particles_from="grid")), [24]),
    "PG-019": ("The grid: 4 by 2 still yellow dots of radius 1 on a 12 by 6 grid, a new grid "
               "born on the last every frame, so the dots' soft edges fill in.",
               case(**GRID), [0, 6]),
    "PG-020": ("The grid falling: a full grid born every frame, each falling from where it "
               "was born, streaks of dots under each grid point.",
               case(**dict(GRID, gravity_force=40, grid_particle_radius=0.8)), [6, 12]),
    "PG-021": ("Grid across keyed from 1 to 4 over frames 0 to 6, gravity 40.",
               case(**dict(GRID, gravity_force=40, grid_particle_radius=0.8,
                           particles_across=keyed((0, 1), (6, 4)))), [8]),
    "PG-022": ("The holder exploded into 2-pixel pieces: at frame 0 its own picture exactly, "
               "then flying apart slowly.", case(**BLAST), [0, 6, 12, 18]),
    "PG-023": ("Pieces of 4 pixels, Radius 2.", case(**dict(BLAST, explode_radius=2)), [12]),
    "PG-024": ("Pieces of 3 pixels, Radius 1.5: the right and bottom pieces cut short by the "
               "layer's edge.", case(**dict(BLAST, explode_radius=1.5)), [12]),
    "PG-025": ("The exploded pieces falling, gravity 20.",
               case(**dict(BLAST, gravity_force=20)), [12]),
    "PG-026": ("Exploding the logo layer, 4 by 4 in the middle: only its pieces.",
               case(**dict(BLAST, explode_layer="logo")), [0, 12]),
    "PG-027": ("Exploding a layer that begins at frame 4, after the in point: nothing.",
               case(**dict(BLAST, explode_layer="late")), [12]),
    "PG-028": ("Auto-Orient on: each piece turned to the way it flies.",
               case(**dict(BLAST, auto_orient="on")), [12]),
    "PG-029": ("Fireworks: shells shot up, each bursting into 16 sparks about half a second "
               "(0.51) after it was shot, the sparks falling, at frames 18, 30 and 40.",
               case(**SHELL), [18, 30, 40]),
    "PG-030": ("Fireworks with no age limit on the exploder: every shell bursts at birth.",
               case(**dict(SHELL, particle_exploder_older_younger_than=0)), [18]),
    "PG-031": ("Exploder Radius 1: four sparks a shell.",
               case(**dict(SHELL, particle_exploder_radius=1)), [30]),
    "PG-032": ("The exploded pieces burst again into four sparks each about a quarter second (0.26) "
               "in.",
               case(**dict(BLAST, particle_exploder_radius=0.5, particle_exploder_dispersion=4,
                           particle_exploder_older_younger_than=0.26)), [12]),
    "PG-033": ("Particles From for the exploder set to the Grid: the Cannon's shells fly on.",
               case(**dict(SHELL, particle_exploder_particles_from="grid")), [30]),
    "PG-034": ("The Layer Map: each fountain dot shown as the 4 by 4 logo.",
               case(**dict(FOUNTAIN, particles_per_second=2, layer_map="logo")), [24]),
    "PG-035": ("The Layer Map with Auto-Orient on: each logo turned to the way it flies.",
               case(**dict(FOUNTAIN, particles_per_second=2, layer_map="logo",
                           auto_orient="on")), [24]),
    "PG-036": ("The Layer Map only for dots older than about a second (1.01): young dots stay dots.",
               case(**dict(FOUNTAIN, particles_per_second=2, layer_map="logo",
                           layer_map_older_younger_than=1.01)), [36]),
    "PG-037": ("The Layer Map on the Grid: each still grid point the logo.",
               case(**dict(GRID, particles_across=2, particles_down=1, layer_map="logo")), [3]),
    "PG-038": ("The Layer Map naming the late layer, at frame 2 (not yet begun): the dots "
               "affected are not drawn.",
               case(**dict(GRID, layer_map="late")), [2]),
    "PG-039": ("A wall: dots thrown every way inside the box mask bounce off its sides.",
               case(masks=[BOX], **WALLED), [12, 36]),
    "PG-040": ("The wall with gravity 40: dots bounce on the floor of the box.",
               case(masks=[BOX], **dict(WALLED, gravity_force=40)), [36]),
    "PG-041": ("The wall for the Grid only: the Cannon's dots go through it.",
               case(masks=[BOX], **dict(WALLED, wall_particles_from="grid")), [36]),
    "PG-042": ("Wall Mask 0: no wall even with the mask there.",
               case(masks=[BOX], **dict(WALLED, wall_mask=0)), [36]),
    "PG-043": ("The fountain with the layer starting at frame 10: nothing at frame 9, frame 22 "
               "the same as PG-006's frame 12.", case(in_frame=10, **FOUNTAIN), [9, 22]),
    "PG-044": ("The fountain with the layer moved 3 pixels right.",
               case(shift=3, **FOUNTAIN), [24]),
    "PG-045": ("Random Seed 7: a different fountain.",
               case(**dict(FOUNTAIN, random_seed=7)), [24]),
    "PG-046": ("Particles Per Second 0 and no grid: nothing.",
               case(particles_per_second=0), [12]),
}
FRAMES_OF = {fx: frames for fx, (_, _, frames) in CASES.items()}
assert all(in_range(c) for _, c, _ in CASES.values())

WARNED = {
    "PG-047": ("The Explode Layer naming a layer that is not in the composition: no pieces, "
               "the fountain drawn, and EFFECT_LAYER_MISSING.",
               case(**dict(FOUNTAIN, explode_layer="gone")), [24], "EFFECT_LAYER_MISSING"),
    "PG-048": ("Wall Mask 2 with only one mask: no wall, the dots drawn, and "
               "EFFECT_PATH_MISSING.", case(masks=[BOX], **dict(WALLED, wall_mask=2)), [36],
               "EFFECT_PATH_MISSING"),
}

INVALID = {
    "PG-049": ("Particles Per Second -1, below 0.", case(particles_per_second=-1)),
    "PG-050": ("Direction Random Spread 361, above 360.", case(direction_random_spread=361)),
    "PG-051": ("Particles Across 101, above 100.", case(particles_across=101)),
    "PG-052": ("Explode Radius 0.4, below 0.5.", case(explode_radius=0.4)),
    "PG-053": ("Gravity Older/Younger Than 1001, above 1000.",
               case(gravity_older_younger_than=1001)),
    "PG-054": ("Wall Mask -1, below 0.", case(wall_mask=-1)),
    "PG-055": ("Particles From \"everything\".", case(gravity_particles_from="everything")),
    "PG-056": ("Auto-Orient \"yes\".", case(auto_orient="yes")),
    "PG-057": ("Cannon Color \"#12345\": not a colour.", case(cannon_color="#12345")),
    "PG-058": ("An Explode Layer written as the number 3, not a word.", case(explode_layer=3)),
    "PG-059": ("Cannon Position with one number.", case(cannon_position=[50])),
    "PG-060": ("Velocity keyed to 20000 at frame 4, above 10000.",
               case(velocity=keyed((0, 0), (4, 20000)))),
}

LISTED = [("PG-006", 24), ("PG-008", 24), ("PG-014", 24), ("PG-016", 24), ("PG-017", 24),
          ("PG-022", 18), ("PG-029", 30), ("PG-039", 36), ("PG-040", 36)]


def fx_id(fx):
    return "FX-PPG-" + fx.split("-")[1]


def write(fx, c, frames):
    name = f"fx_ppg_{fx.split('-')[1]}.json"
    (OUT / name).write_text(json.dumps(project_json(fx_id(fx), c, frames), indent=2) + "\n",
                            encoding="utf-8")
    return name


def length(frames):
    return max(list(frames) + [4]) + 2


def listed():
    out = {"tolerance": 1e-7, "cases": {}}
    for fx, f in LISTED:
        run = Run(CASES[fx][1], f)
        rows = [[p["code"], p["a"], p["b"], p["born"]] + p["x"] + p["v"] for p in everyone(run)]
        out["cases"][fx_id(fx)] = {"frame": f, "particles": rows}
        print(f"{fx_id(fx)} frame {f}: {len(rows)} particles listed")
    (OUT / "particles_particle_playground.json").write_text(json.dumps(out, indent=1) + "\n",
                                                            encoding="utf-8")


def clear():
    """No number lands so near a cut that the build's rounding could put it on the other side:
    a birth on the frame drawn, an age on Older/Younger Than, a weight on its random number, a
    particle on a selection map's pixel edge, or a move on a wall's edge or corner."""
    for fx, (_, c, frames) in list(CASES.items()) + [(k, v[:3]) for k, v in WARNED.items()]:
        for f in frames:
            run = Run(c, f)
            if run.u < 0:
                continue
            HITS.clear()
            real_affected = affected

            def watched(run_, gi, p, t, real=real_affected):
                w = weight(run_, gi, p, t)
                r = run_.rr(p, 20 + gi)
                if 0 < w < 1:
                    assert abs(w - r) > 1e-6, (fx, f, gi)
                g = GROUPS[gi]
                tt = run_.at(f"{g}_older_younger_than", t)
                if tt != 0 and run_.at(f"{g}_age_feather", t) == 0:
                    age = (t - p["born"]) / FPS
                    assert abs(abs(tt) - age) > 1e-9, (fx, f, gi, age)
                if run_.maps[g] is not None:
                    for v in p["x"]:
                        assert abs(v - round(v)) > 1e-6, (fx, f, gi, v)
                return real(run_, gi, p, t)

            globals()["affected"] = watched
            try:
                everyone(run)
            finally:
                globals()["affected"] = real_affected
            for fr, s in HITS:
                assert min(abs(fr), abs(fr - 1)) > 1e-9 or not -1e-9 < s < 1 + 1e-9, (fx, f, fr)
                assert min(abs(s), abs(s - 1)) > 1e-9 or not -1e-9 < fr < 1 + 1e-9, (fx, f, s)
            for k, b in P.births(Rate(run)):
                assert abs(b - run.u) > 1e-6, (fx, f, k)
    print("clear")


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, rows in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(rows))
    clear()
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx_id(fx)] = {"says": says, "project": write(fx, c, length(frames)),
                                        "frames": rendered}
        print(f"{fx_id(fx)}: " + ", ".join(
            f"frame {f} {sum(1 for p in px if p[3] > 0)} covered" for f, px in rendered.items()))
    for fx, (says, c, frames, warning) in WARNED.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx_id(fx)] = {"says": says, "project": write(fx, c, length(frames)),
                                        "frames": rendered, "warning": warning}
        print(f"{fx_id(fx)}: warned {warning}")
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx_id(fx)] = {"says": says, "project": write(fx, c, length([4])),
                                        "frames": {"0": before, "4": before},
                                        "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx_id(fx)}: invalid")
    (OUT / "expected_particle_playground.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                           encoding="utf-8")
    listed()
    check(expected)


def check(expected):
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    clear_frame = [[0.0] * 4 for _ in range(W * H)]
    total = lambda fr: sum(p[3] for p in fr)  # noqa: E731
    g = lambda n, f: c["FX-PPG-%03d" % n][str(f)]  # noqa: E731
    assert g(1, 0) == clear_frame
    assert g(2, 11) == clear_frame and g(2, 13) == g(2, 30) and total(g(2, 13)) > 0
    assert g(2, 13)[5 * W + 8][3] == 1.0
    assert total(g(3, 13)) < total(g(2, 13))
    assert g(6, 40) != g(6, 24)
    for n in range(7, 19):
        assert c["FX-PPG-%03d" % n]["24"] != g(6, 24), n
    # a grid born every frame piles up on itself: the edges of the dots fill in
    assert total(g(19, 6)) > total(g(19, 0)) > 0
    # the explosion starts as the picture itself
    art = decoded("art")["px"]
    assert all(abs(a - b) < 1e-12 for p, q in zip(g(22, 0), art) for a, b in zip(p, q))
    assert g(22, 6) != g(22, 0)
    assert total(g(26, 0)) == 16.0
    assert g(27, 12) == clear_frame
    assert g(28, 12) != g(22, 12)
    assert g(29, 30) != g(29, 18)
    assert g(42, 36) != g(39, 36)
    assert g(43, 9) == clear_frame and g(43, 22) == g(6, 12)
    m, base = g(44, 24), g(6, 24)
    for y in range(H):
        assert m[y * W + 3:(y + 1) * W] == base[y * W:y * W + W - 3]
    assert g(45, 24) != base
    assert g(46, 12) == clear_frame
    assert c["FX-PPG-047"]["24"] == base
    assert c["FX-PPG-048"]["36"] == g(42, 36)
    # inside the wall every dot stays inside the box
    run = Run(CASES["PG-039"][1], 36)
    assert all(1.5 < p["x"][0] < 14.5 and 1.5 < p["x"][1] < 8.5 for p in everyone(run))
    # without it some leave
    run = Run(CASES["PG-042"][1], 36)
    assert any(not (1.5 < p["x"][0] < 14.5 and 1.5 < p["x"][1] < 8.5) for p in everyone(run))
    # a shell makes 16 sparks, radius 2 over 0.5
    run = Run(CASES["PG-029"][1], 30)
    sparks = [p for p in everyone(run) if p["code"] > 4]
    assert len(sparks) % 16 == 0 and sparks
    # Direction 90 throws right, gravity 180 pulls down
    run = Run(CASES["PG-007"][1], 24)
    assert all(p["v"][0] > 0 for p in cannon(run))
    print("checked")


if __name__ == "__main__":
    main()
