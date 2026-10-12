"""Particle Playground's property mappers, worked a second way.

D-614, the second step of `core.particle_playground` (D-613, P0-6): After Effects' Particle
Playground has a Persistent Property Mapper and an Ephemeral Property Mapper, each reading a
layer's red, green and blue under every particle into one of the particle's properties. Adobe's
page says what each control is for and gives no formula, so every rule and every number when
added below is ours (document 21, D-614). No text or code was copied. The rest of the effect is
D-613's, unchanged: `tools/particle_playground_reference.py`, whose rule this file extends and
whose code it runs.

**No state is carried from frame to frame** (document 20): a property is part of the particle,
worked out along its own steps from its birth like its place and speed.

The settings added, all keyable numbers, words or layers, each as added when a file leaves it
out (a file written before D-614 is the effect it was), for m = `persistent` and `ephemeral`:

- `m_map_layer` (D-189's layer word, "" none; the map read at the frame drawn, fitted by
  `center`, as D-613's maps), the Affects of D-613 for the two new groups g = 4 and 5
  (`m_particles_from`, `m_selection_map`, `m_older_younger_than`, `m_age_feather`; random
  stream 20 + g), `m_map_red_to`, `m_map_green_to`, `m_map_blue_to` (a property or `none`), and
  `m_min_red`, `m_max_red`, `m_min_green`, `m_max_green`, `m_min_blue`, `m_max_blue` (-10000 to
  10000; 0 and 1 when added). The Ephemeral one also has `ephemeral_operator`: `set`, `add`,
  `difference` (|a - v|), `subtract` (a - v), `multiply`, `min` or `max`, the property a and the
  map's value v.
- The properties: `red`, `green`, `blue` (the particle's colour, sRGB-coded 0 to 1, held to 0 to
  1 when drawn), `opacity` (1 when born, held to 0 to 1 when drawn), `scale` (1 when born; at 0 or
  below the particle is not drawn), `angle` (degrees, 0 when born, added to Auto-Orient's turn),
  `x`, `y` (its place in pixels), `x_velocity`, `y_velocity` (pixels a second) and `x_force`,
  `y_force` (pixels a second a second, 0 when born, added to Gravity in every step).
- A mapper gives a particle p at time t, when its map is named, p is affected by its Affects at
  t (D-613's weight against r(p, 20 + g)) and (floor x, floor y) is a pixel of the drawing: for
  each channel c mapped to a property, the value lo + (hi - lo) e, lo and hi its Min and Max at
  t and e the map's pixel's channel c, sRGB-coded from its premultiplied value held to 0 to 1.
  All three are read before any is used; two channels on one property, the later wins.
- Persistent: at every one of p's step points (its birth and each quarter frame up to the frame
  drawn), after the Particle Exploder is asked and before the step, each value given is set as
  the property, and kept.
- Ephemeral: in every step from a point t0, the forces given are the operator on the particle's
  force and the value, and the velocities given are the operator on its velocity and the value;
  the step moves with them (and the Wall turns that move); afterwards the velocity is what the
  step left minus the change the mapper made, so the mapper's change is not kept. At the frame
  drawn, the colour, opacity, scale, angle, x and y given are the operator on the property and
  the value, for drawing only.
- Children of the Particle Exploder are born with their parent's properties.
- Drawing as D-613's, with the properties: X the place (x, y); the turn o plus the angle; a
  dot's radius times the scale and its colour the coded colour linearised, times the parent's
  alpha; a piece's side times the scale, sampled at O + L / scale; a Layer Map's picture sampled
  at (W/2, H/2) + L / scale; each covering times the opacity.

Not built (D-614 says why): the Mass, Kinetic and Static Friction, Angular Velocity, Torque,
Gradient Velocity, separate X and Y Scale and Lifespan properties (nothing in this playground has
mass, friction or spin, or dies), and Character and Font Size (the text particles are not built).

**This file never runs the build's code path.** Every case is a project of one composition 16
by 10 at 24 frames a second in `Fixtures/particle_mapper/`, as D-613's, with a fourth map layer,
`ramp`: red rising from 0 at the left to 255 at the right, green from the top to the bottom, blue
128. Every case is "not built yet" until B-494 lands (D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/particle_mapper_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed  # noqa: E402
from adjust_reference import srgb_to_linear  # noqa: E402
import smooth_reference as S  # noqa: E402
import particle_playground_reference as G  # noqa: E402

P, L = G.P, G.L
W, H, FPS = G.W, G.H, G.FPS
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "particle_mapper"
TOLERANCE = 2e-5
MAPPERS = ("persistent", "ephemeral")
CHANNELS = ("red", "green", "blue")
PROPERTIES = ("none", "red", "green", "blue", "opacity", "scale", "angle", "x", "y",
              "x_velocity", "y_velocity", "x_force", "y_force")
OPERATORS = ("set", "add", "difference", "subtract", "multiply", "min", "max")

MORE_SETTINGS = [(f"{m}_{n}", lo, hi, 0) for m in MAPPERS
                 for n, lo, hi in (("older_younger_than", -1000, 1000), ("age_feather", 0, 1000))] + [
    (f"{m}_{e}_{c}", -10000, 10000, 0 if e == "min" else 1)
    for m in MAPPERS for c in CHANNELS for e in ("min", "max")]
MORE_WORDS = [(f"{m}_particles_from", tuple(G.FROM), "all") for m in MAPPERS] + [
    (f"{m}_map_{c}_to", PROPERTIES, "none") for m in MAPPERS for c in CHANNELS] + [
    ("ephemeral_operator", OPERATORS, "set")]
MORE_LAYERS = [f"{m}_map_layer" for m in MAPPERS] + [f"{m}_selection_map" for m in MAPPERS]


def ramp_drawing():
    return [[(round(255 * x / (W - 1)), round(255 * y / (H - 1)), 128, 255) for x in range(W)]
            for y in range(H)]


# D-614 extends D-613's contract and its run in place.
G.GROUPS = G.GROUPS + MAPPERS
G.SETTINGS = G.SETTINGS + MORE_SETTINGS
G.NAMES = [s[0] for s in G.SETTINGS]
G.RANGES = {s[0]: (s[1], s[2]) for s in G.SETTINGS}
G.WORDS = G.WORDS + MORE_WORDS
G.LAYERS = G.LAYERS + MORE_LAYERS
G.DRAWINGS["ramp"] = ramp_drawing()
G.MAP_LAYERS["ramp"] = ("ramp", 0)
G.OUT = OUT


class Run(G.Run):
    def __init__(self, c, n):
        super().__init__(c, n)
        self.pmaps = {m: G.map_of(c[f"{m}_map_layer"], n) for m in MAPPERS}


G.Run = Run

_particle = G.particle


def particle(*a, **k):
    p = _particle(*a, **k)
    p.update(rgb=None, opacity=1.0, scale=1.0, angle=0.0, force=[0.0, 0.0])
    return p


G.particle = particle
_children = G.children


def children(run, p, t):
    out = _children(run, p, t)
    for ch in out:
        ch.update(rgb=None if p["rgb"] is None else list(p["rgb"]), opacity=p["opacity"],
                  scale=p["scale"], angle=p["angle"], force=list(p["force"]))
    return out


G.children = children


def coded(p):
    """The particle's colour, sRGB-coded and straight."""
    if p["rgb"] is not None:
        return list(p["rgb"])
    col = p["col"]
    return [S.linear_to_srgb(col[i] / col[3]) if col[3] > 0 else 0.0 for i in range(3)]


def operate(op, a, v):
    return {"set": v, "add": a + v, "difference": abs(a - v), "subtract": a - v,
            "multiply": a * v, "min": min(a, v), "max": max(a, v)}[op]


def mapped(run, m, p, t):
    """[(property, value)] mapper m gives p at t, in channel order; [] when it gives none."""
    mp = run.pmaps[m]
    if mp is None or not G.affected(run, G.GROUPS.index(m), p, t):
        return []
    gx, gy = math.floor(p["x"][0]), math.floor(p["x"][1])
    if not (0 <= gx < W and 0 <= gy < H):
        return []
    px = L.at(mp, gx, gy)
    out = []
    for ci, ch in enumerate(CHANNELS):
        prop = run.c[f"{m}_map_{ch}_to"]
        if prop != "none":
            lo, hi = run.at(f"{m}_min_{ch}", t), run.at(f"{m}_max_{ch}", t)
            out.append((prop, lo + (hi - lo) * S.linear_to_srgb(P.clamp(px[ci]))))
    return out


def put(p, prop, v):
    if prop in CHANNELS:
        rgb = coded(p)
        rgb[CHANNELS.index(prop)] = v
        p["rgb"] = rgb
    elif prop in ("opacity", "scale", "angle"):
        p[prop] = v
    else:
        what, axis = {"x": ("x", 0), "y": ("x", 1), "x_velocity": ("v", 0),
                      "y_velocity": ("v", 1), "x_force": ("force", 0), "y_force": ("force", 1)}[prop]
        p[what] = list(p[what])
        p[what][axis] = v


def step(run, p, t0, h):
    a = [0.0, 0.0]
    if G.affected(run, 2, p, t0):
        f = run.at("gravity_force", t0) + run.at("gravity_random_spread", t0) * (run.rr(p, 7) - 0.5)
        gx, gy = G.heading(run.at("gravity_direction", t0))
        a = [f * gx, f * gy]
    force, v_base = list(p["force"]), list(p["v"])
    v_eff = list(v_base)
    op = run.c["ephemeral_operator"]
    for prop, val in mapped(run, "ephemeral", p, t0):
        if prop in ("x_force", "y_force"):
            i = prop[0] == "y"
            force[i] = operate(op, force[i], val)
        elif prop in ("x_velocity", "y_velocity"):
            i = prop[0] == "y"
            v_eff[i] = operate(op, v_eff[i], val)
    a = [a[0] + force[0], a[1] + force[1]]
    p["v"] = v_eff
    if run.wall is not None and G.affected(run, 3, p, t0):
        G.bounce_move(run, p, a, h)
    else:
        x, v = p["x"], p["v"]
        p["x"] = [x[i] + v[i] * h + a[i] * h * h / 2 for i in range(2)]
        p["v"] = [v[i] + a[i] * h for i in range(2)]
    if v_eff != v_base:
        p["v"] = [p["v"][i] - (v_eff[i] - v_base[i]) for i in range(2)]


G.step = step


def moved(run, p):
    pts = G.points_from(p["born"], run.u)
    explodes = p["code"] <= 3
    for i, t0 in enumerate(pts):
        if explodes and run.at("particle_exploder_radius", t0) > 0 and G.affected(run, 0, p, t0):
            out = []
            for ch in G.children(run, p, t0):
                out += moved(run, ch)
            return out
        for prop, val in mapped(run, "persistent", p, t0):
            put(p, prop, val)
        if i + 1 < len(pts):
            step(run, p, t0, (pts[i + 1] - t0) / FPS)
    return [p]


G.moved = moved


def playground(c, n):
    out = [[0.0] * 4 for _ in range(W * H)]
    run = Run(c, n)
    if run.u < 0:
        return out
    op = c["ephemeral_operator"]
    for p in G.everyone(run):
        X, Y = p["x"]
        rgb = p["rgb"]
        opacity, scale, angle = p["opacity"], p["scale"], p["angle"]
        for prop, val in mapped(run, "ephemeral", p, run.u):
            if prop == "x":
                X = operate(op, X, val)
            elif prop == "y":
                Y = operate(op, Y, val)
            elif prop in CHANNELS:
                rgb = coded(p) if rgb is None else list(rgb)
                i = CHANNELS.index(prop)
                rgb[i] = operate(op, rgb[i], val)
            elif prop == "opacity":
                opacity = operate(op, opacity, val)
            elif prop == "scale":
                scale = operate(op, scale, val)
            elif prop == "angle":
                angle = operate(op, angle, val)
        k = P.clamp(opacity)
        if scale <= 0 or k <= 0:
            continue
        o = G.turn_of(run, p) + math.radians(angle)
        if run.layer_map is not None and G.affected(run, 1, p, run.u):
            m = run.layer_map
            for gy in range(H):
                for gx in range(W):
                    lx, ly = G.local(o, gx + 0.5 - X, gy + 0.5 - Y)
                    s = L.bilinear(m, W / 2 + lx / scale, H / 2 + ly / scale)
                    if s[3] > 0:
                        out[gy * W + gx] = G.lay(out[gy * W + gx], [v * k for v in s])
            continue
        if p["code"] == 3:
            q = p["side"]
            hold = q / 2 - 0.5
            for gy in range(H):
                for gx in range(W):
                    lx, ly = G.local(o, gx + 0.5 - X, gy + 0.5 - Y)
                    cv = P.clamp(q * scale / 2 - abs(lx) + 0.5) * P.clamp(q * scale / 2 - abs(ly) + 0.5)
                    if cv <= 0:
                        continue
                    s = L.bilinear(run.explode, p["origin"][0] + P.clamp(lx / scale, -hold, hold),
                                   p["origin"][1] + P.clamp(ly / scale, -hold, hold))
                    out[gy * W + gx] = G.lay(out[gy * W + gx], [v * cv * k for v in s])
            continue
        col = p["col"]
        if rgb is not None:
            col = [srgb_to_linear(P.clamp(v)) * col[3] for v in rgb] + [col[3]]
        rad, kk = p["rad"] * scale, 1.0
        if rad < 0.7:
            kk, rad = (rad / 0.7) ** 2, 0.7
        for gy in range(H):
            for gx in range(W):
                cv = P.clamp(rad - math.hypot(gx + 0.5 - X, gy + 0.5 - Y) + 0.5) * kk * k
                if cv > 0:
                    out[gy * W + gx] = G.lay(out[gy * W + gx], [v * cv for v in col])
    return out


G.playground = playground


# --- the cases ------------------------------------------------------------------------------

case = G.case
GRID, FOUNTAIN, BLAST, SHELL, WALLED, BOX = G.GRID, G.FOUNTAIN, G.BLAST, G.SHELL, G.WALLED, G.BOX
# The still grid, mapped persistently from the ramp.
PG = dict(GRID, persistent_map_layer="ramp")
EG = dict(GRID, ephemeral_map_layer="ramp")
# The holder cut in 2.5-pixel pieces, none starting on a pixel edge.
BL = dict(BLAST, explode_radius=1.25)
# The fountain, slower and denser, mapped from the ramp.
FT = dict(FOUNTAIN, particles_per_second=12, cannon_position=[53, 85])
# Fireworks and the walled dots, their cannons off the pixel edges.
SH = dict(SHELL, cannon_position=[53, 87])
WL = dict(WALLED, cannon_position=[53, 53])
PF = dict(FT, persistent_map_layer="ramp")
EF = dict(FT, ephemeral_map_layer="ramp")


def both_grids(**more):
    """The grid with the fountain over it, Gravity for the Cannon's dots only."""
    return dict(FT, gravity_particles_from="cannon", particles_across=4, particles_down=2, grid_width=12, grid_height=6,
                grid_particle_radius=1, **more)


CASES = {
    "PM-001": ("Persistent, red to Opacity 0 to 1: the still grid fades in from left to right.",
               case(**dict(PG, persistent_map_red_to="opacity")), [0, 6]),
    "PM-002": ("Persistent, red to Scale 0.5 to 2: the grid's dots grow from left to right.",
               case(**dict(PG, persistent_map_red_to="scale", persistent_min_red=0.5,
                           persistent_max_red=2)), [6]),
    "PM-003": ("Persistent, green to Red and blue to Blue: the yellow dots turn mint green, "
               "less red in the top row than the bottom, with the ramp's middle blue.",
               case(**dict(PG, persistent_map_green_to="red", persistent_map_blue_to="blue")), [6]),
    "PM-004": ("Persistent, red to X 2.3 to 13.9: each fountain dot is pulled across to the "
               "place the ramp names under it, at every quarter frame.",
               case(**dict(PF, persistent_map_red_to="x", persistent_min_red=2.3,
                           persistent_max_red=13.9)), [24]),
    "PM-005": ("Persistent, green to Y Velocity -40 to 10: dots low on the ramp shot up, high "
               "ones slowed; the speed kept after they leave.",
               case(**dict(PF, persistent_map_green_to="y_velocity", persistent_min_green=-40,
                           persistent_max_green=10)), [24]),
    "PM-006": ("Persistent, red to X Force -40 to 40: dots on the left pushed left, on the right "
               "pushed right, and kept pushed.",
               case(**dict(PF, persistent_map_red_to="x_force", persistent_min_red=-40,
                           persistent_max_red=40)), [24]),
    "PM-007": ("Persistent for the Cannon only, red to Opacity: the grid stays solid, the "
               "fountain's dots fade by place.",
               case(**both_grids(persistent_map_layer="ramp", persistent_map_red_to="opacity",
                                 persistent_particles_from="cannon")), [24]),
    "PM-008": ("Persistent through the selection map: only the left half's dots take Scale 0.3.",
               case(**dict(PG, persistent_map_red_to="scale", persistent_min_red=0.3,
                           persistent_max_red=0.3, persistent_selection_map="sel")), [6]),
    "PM-009": ("Persistent only for dots older than about half a second (0.51), red to "
               "Opacity: young dots solid, older ones faded by where they were.",
               case(**dict(PF, persistent_map_red_to="opacity",
                           persistent_older_younger_than=0.51)), [24]),
    "PM-010": ("Persistent from the late layer (begins at frame 4) at frame 2: its map is clear, "
               "every channel 0, so Scale is its Min, 0.4, everywhere.",
               case(**dict(GRID, persistent_map_layer="late", persistent_map_red_to="scale",
                           persistent_min_red=0.4)), [2]),
    "PM-011": ("Persistent, red to Angle 0 to 90 on the exploded holder: each piece turned by "
               "where it was.",
               case(**dict(BL, persistent_map_layer="ramp", persistent_map_red_to="angle",
                           persistent_max_red=90)), [6]),
    "PM-012": ("Persistent, red to Scale 0.5 to 1.5 on the Layer Map: each grid point the logo, "
               "smaller on the left.",
               case(**dict(PG, particles_across=2, particles_down=1, grid_width=10,
                           grid_position=[50, 55], layer_map="logo",
                           persistent_map_red_to="scale", persistent_min_red=0.5,
                           persistent_max_red=1.5)), [3]),
    "PM-013": ("Fireworks, the shells coloured persistently by the ramp (green to Green 0 to "
               "1): each burst's sparks keep their shell's colour.",
               case(**dict(SH, persistent_map_layer="ramp", persistent_map_green_to="green")),
               [30]),
    "PM-014": ("Persistent and the same property twice: red to Opacity, then green to Opacity "
               "0.2 to 0.6; the green one wins, the grid fading by row.",
               case(**dict(PG, persistent_map_red_to="opacity", persistent_map_green_to="opacity",
                           persistent_min_green=0.2, persistent_max_green=0.6)), [6]),
    "PM-015": ("Persistent with every channel on None: the fountain as without it.",
               case(**dict(PF)), [24]),
    "PM-016": ("Persistent, Max Red keyed 0 to 1 over frames 0 to 24, red to Opacity: the "
               "fountain's dots fainter the earlier they were mapped.",
               case(**dict(PF, persistent_map_red_to="opacity",
                           persistent_max_red=keyed((0, 0), (24, 1)))), [24]),
    "PE-017": ("Ephemeral Set, red to Opacity: the fountain's dots fade by where they are now.",
               case(**dict(EF, ephemeral_map_red_to="opacity")), [24]),
    "PE-018": ("Ephemeral Add, green to Scale 0 to 1: dots bigger lower down.",
               case(**dict(EF, ephemeral_map_green_to="scale", ephemeral_operator="add")), [24]),
    "PE-019": ("Ephemeral Difference, red to Opacity: |1 - red|, the dots fading to the right.",
               case(**dict(EG, ephemeral_map_red_to="opacity",
                           ephemeral_operator="difference")), [6]),
    "PE-020": ("Ephemeral Subtract, red to Scale 0 to 0.8: dots smaller to the right.",
               case(**dict(EG, ephemeral_map_red_to="scale", ephemeral_max_red=0.8,
                           ephemeral_operator="subtract")), [6]),
    "PE-021": ("Ephemeral Multiply, green to Opacity: the grid's top row fainter than its "
               "bottom.", case(**dict(EG, ephemeral_map_green_to="opacity",
                                      ephemeral_operator="multiply")), [6]),
    "PE-022": ("Ephemeral Min, red to Opacity 0.3 to 1: no dot more opaque than the ramp says.",
               case(**dict(EG, ephemeral_map_red_to="opacity", ephemeral_min_red=0.3,
                           ephemeral_operator="min")), [6]),
    "PE-023": ("Ephemeral Max, red to Scale 0 to 2: dots on the right grown, those on the left "
               "kept at 1.", case(**dict(EG, ephemeral_map_red_to="scale", ephemeral_max_red=2,
                                         ephemeral_operator="max")), [6]),
    "PE-024": ("Ephemeral Add, red to X -2 to 2: each dot drawn moved by the ramp, its path "
               "unchanged.", case(**dict(EF, ephemeral_map_red_to="x", ephemeral_min_red=-2,
                                         ephemeral_max_red=2, ephemeral_operator="add")), [24]),
    "PE-025": ("Ephemeral Add, red to X Velocity -30 to 30: each step pushed by the ramp under "
               "it, the push not kept.",
               case(**dict(EF, ephemeral_map_red_to="x_velocity", ephemeral_min_red=-30,
                           ephemeral_max_red=30, ephemeral_operator="add")), [24]),
    "PE-026": ("Ephemeral Set, green to Y Force -60 to 60: dots high up pulled up, low ones "
               "down, only while there.",
               case(**dict(EF, ephemeral_map_green_to="y_force", ephemeral_min_green=-60,
                           ephemeral_max_green=60)), [24]),
    "PE-027": ("Ephemeral Add, red to Angle 0 to 45 on the exploded holder with Auto-Orient: "
               "each piece turned a little more to the right.",
               case(**dict(BL, auto_orient="on", ephemeral_map_layer="ramp",
                           ephemeral_map_red_to="angle", ephemeral_max_red=45,
                           ephemeral_operator="add")), [12]),
    "PE-028": ("Ephemeral through the selection map, Scale 2: only dots over its white left "
               "half drawn twice the size.",
               case(**dict(EG, ephemeral_map_red_to="scale", ephemeral_min_red=2,
                           ephemeral_max_red=2, ephemeral_selection_map="sel")), [6]),
    "PE-029": ("Ephemeral only for dots younger than about half a second (0.51), blue to Green 1: "
               "the young red dots yellow.",
               case(**dict(EF, ephemeral_map_blue_to="green", ephemeral_min_blue=1,
                           ephemeral_older_younger_than=-0.51)), [24]),
    "PE-030": ("Both: Persistent red to Opacity 0.3 to 1, then Ephemeral Multiply green to "
               "Opacity.",
               case(**dict(PF, ephemeral_map_layer="ramp", persistent_map_red_to="opacity",
                           persistent_min_red=0.3, ephemeral_map_green_to="opacity",
                           ephemeral_operator="multiply")), [24]),
    "PE-031": ("Ephemeral Set Scale 0: nothing drawn.",
               case(**dict(EG, ephemeral_map_red_to="scale", ephemeral_max_red=0)), [6]),
    "PE-032": ("The wall with Ephemeral Add, red to X Velocity -20 to 20: the dots still bounce "
               "inside the box.",
               case(masks=[BOX], **dict(WL, ephemeral_map_layer="ramp",
                                        ephemeral_map_red_to="x_velocity", ephemeral_min_red=-20,
                                        ephemeral_max_red=20, ephemeral_operator="add")), [36]),
    "PE-033": ("Fireworks with Ephemeral Set, red to Red 0 to 1, for the Particle Exploder's "
               "sparks only.",
               case(**dict(SH, ephemeral_map_layer="ramp", ephemeral_map_red_to="red",
                           ephemeral_particles_from="particle_exploder")), [30]),
}
FRAMES_OF = {fx: frames for fx, (_, _, frames) in CASES.items()}

WARNED = {
    "PM-034": ("Persistent Map Layer naming a layer that is not in the composition: no mapping, "
               "the grid drawn as without it, and EFFECT_LAYER_MISSING.",
               case(**dict(GRID, persistent_map_layer="gone", persistent_map_red_to="scale")), [6],
               "EFFECT_LAYER_MISSING"),
    "PE-035": ("Ephemeral Selection Map naming a missing layer: the mapper applies to every dot, "
               "and EFFECT_LAYER_MISSING.",
               case(**dict(EG, ephemeral_map_red_to="opacity", ephemeral_selection_map="gone")),
               [6], "EFFECT_LAYER_MISSING"),
}

INVALID = {
    "PM-036": ("Persistent Min Red -10001, below -10000.", case(persistent_min_red=-10001)),
    "PE-037": ("Ephemeral Max Blue 10001, above 10000.", case(ephemeral_max_blue=10001)),
    "PM-038": ("Persistent Age Feather -1, below 0.", case(persistent_age_feather=-1)),
    "PM-039": ("Persistent Map Red To \"mass\", not built.", case(persistent_map_red_to="mass")),
    "PE-040": ("Ephemeral Operator \"divide\".", case(ephemeral_operator="divide")),
    "PE-041": ("Ephemeral Particles From \"everything\".",
               case(ephemeral_particles_from="everything")),
    "PM-042": ("A Persistent Map Layer written as the number 3, not a word.",
               case(persistent_map_layer=3)),
    "PE-043": ("Ephemeral Max Green keyed to 20000 at frame 4, above 10000.",
               case(ephemeral_max_green=keyed((0, 1), (4, 20000)))),
}

LISTED = [("PM-004", 24), ("PM-005", 24), ("PM-006", 24), ("PM-013", 30), ("PE-025", 24),
          ("PE-026", 24), ("PE-032", 36)]


def fx_id(fx):
    return "FX-PMAP-" + fx.split("-")[1]


def write(fx, c, frames):
    name = f"fx_pmap_{fx.split('-')[1]}.json"
    (OUT / name).write_text(json.dumps(G.project_json(fx_id(fx), c, frames), indent=2) + "\n",
                            encoding="utf-8")
    return name


def listed():
    out = {"tolerance": 1e-7, "cases": {}}
    for fx, f in LISTED:
        run = Run(CASES[fx][1], f)
        rows = [[p["code"], p["a"], p["b"], p["born"]] + p["x"] + p["v"] for p in G.everyone(run)]
        out["cases"][fx_id(fx)] = {"frame": f, "particles": rows}
        print(f"{fx_id(fx)} frame {f}: {len(rows)} particles listed")
    (OUT / "particles_particle_mapper.json").write_text(json.dumps(out, indent=1) + "\n",
                                                        encoding="utf-8")


def clear():
    """As D-613's: no number lands so near a cut that the build's rounding could put it on the
    other side, the mappers' maps' pixel edges and weights included."""
    real = G.weight
    for fx, (_, c, frames) in list(CASES.items()) + [(k, v[:3]) for k, v in WARNED.items()]:
        for f in frames:
            run = Run(c, f)
            if run.u < 0:
                continue
            G.HITS.clear()

            def watched(run_, gi, p, t):
                w = real(run_, gi, p, t)
                r = run_.rr(p, 20 + gi)
                if 0 < w < 1:
                    assert abs(w - r) > 1e-6, (fx, f, gi)
                g = G.GROUPS[gi]
                tt = run_.at(f"{g}_older_younger_than", t)
                if tt != 0 and run_.at(f"{g}_age_feather", t) == 0:
                    age = (t - p["born"]) / FPS
                    assert abs(abs(tt) - age) > 1e-9, (fx, f, gi, age)
                named = run_.maps[g] is not None or (g in MAPPERS and run_.pmaps[g] is not None)
                if named:
                    for v in p["x"]:
                        assert abs(v - round(v)) > 1e-6, (fx, f, gi, v)
                return w

            G.weight = watched
            try:
                G.everyone(run)
                playground(c, f)
            finally:
                G.weight = real
            for fr, s in G.HITS:
                assert min(abs(fr), abs(fr - 1)) > 1e-9 or not -1e-9 < s < 1 + 1e-9, (fx, f, fr)
                assert min(abs(s), abs(s - 1)) > 1e-9 or not -1e-9 < fr < 1 + 1e-9, (fx, f, s)
            for k, b in P.births(G.Rate(run)):
                assert abs(b - run.u) > 1e-6, (fx, f, k)
    print("clear")


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, rows in G.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(G.png(rows))
    clear()
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): G.render(c, f) for f in frames}
        expected["cases"][fx_id(fx)] = {"says": says, "project": write(fx, c, G.length(frames)),
                                        "frames": rendered}
        print(f"{fx_id(fx)}: " + ", ".join(
            f"frame {f} {sum(1 for p in px if p[3] > 0)} covered" for f, px in rendered.items()))
    for fx, (says, c, frames, warning) in WARNED.items():
        rendered = {str(f): G.render(c, f) for f in frames}
        expected["cases"][fx_id(fx)] = {"says": says, "project": write(fx, c, G.length(frames)),
                                        "frames": rendered, "warning": warning}
        print(f"{fx_id(fx)}: warned {warning}")
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = G.plain(c)
        expected["cases"][fx_id(fx)] = {"says": says, "project": write(fx, c, G.length([4])),
                                        "frames": {"0": before, "4": before},
                                        "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx_id(fx)}: invalid")
    (OUT / "expected_particle_mapper.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                       encoding="utf-8")
    listed()
    check(expected)


def check(expected):
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    g = lambda n, f: c["FX-PMAP-%03d" % n][str(f)]  # noqa: E731
    total = lambda fr: sum(p[3] for p in fr)  # noqa: E731
    clear_frame = [[0.0] * 4 for _ in range(W * H)]
    plain_grid = G.render(case(**GRID), 6)
    plain_fountain = G.render(case(**FT), 24)
    # every mapped case differs from the same shot unmapped, and the unmapped one is D-613's
    assert g(15, 24) == plain_fountain
    assert g(34, 6) == plain_grid
    for n in (1, 2, 3, 8, 14, 19, 20, 21, 22, 23, 28):
        assert g(n, 6) != plain_grid, n
    for n in (4, 5, 6, 7, 9, 16, 17, 18, 24, 25, 26, 29, 30):
        assert g(n, 24) != plain_fountain, n
    assert g(31, 6) == clear_frame
    for n, f, base in ((10, 2, case(**GRID)), (11, 6, case(**BL)),
                       (12, 3, case(**dict(GRID, particles_across=2, particles_down=1,
                                           grid_width=10, grid_position=[50, 55],
                                           layer_map="logo"))),
                       (13, 30, case(**SH)), (27, 12, case(**dict(BL, auto_orient="on"))),
                       (32, 36, case(masks=[BOX], **WL)), (33, 30, case(**SH))):
        assert g(n, f) != G.render(base, f), n
    # opacity fading left to right: the left dot fainter than the right
    left, right = g(1, 6)[3 * W + 3][3], g(1, 6)[3 * W + 12][3]
    assert left < right
    # the ephemeral x move leaves the path alone: the listed fountain is D-613's
    run = Run(CASES["PE-024"][1], 24)
    base = Run(case(**FT), 24)
    assert [p["x"] for p in G.everyone(run)] == [p["x"] for p in G.everyone(base)]
    assert total(g(2, 6)) > total(plain_grid)
    print("checked")


if __name__ == "__main__":
    main()
