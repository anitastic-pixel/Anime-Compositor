"""Particle Playground's further mapper properties, worked a second way.

D-713, the third step of `core.particle_playground` (D-613, D-614, P0-6): the property mappers
can also set a particle's separate X and Y Scale, Angular Velocity, Torque, Lifespan and Kinetic
and Static Friction. Adobe's page says what each property is for and gives no formula, so every
rule and every number when added below is ours (document 21, D-713). No text or code was copied.
The rest is D-613's and D-614's, unchanged: `tools/particle_playground_reference.py` and
`tools/particle_mapper_reference.py`, whose rules this file extends and whose code it runs.

**No state is carried from frame to frame** (document 20): each new property is part of the
particle, worked out along its own steps from its birth like its place and speed.

The words added: seven more properties a mapper's channel can be set to, after D-614's, so every
file written before keeps its meaning. Nothing else is added.

- `x_scale`, `y_scale` (1 when born): the particle drawn stretched along its own turned x and y
  as well as by Scale. At 0 or below it is not drawn.
- `angular_velocity` (degrees a second, 0 when born) and `torque` (degrees a second a second, 0
  when born): in every step of h seconds the angle grows by w h + T h^2 / 2 and w by T h, w the
  angular velocity and T the torque, as the place and speed do.
- `lifespan` (seconds, endless when born): at every step point, after the Persistent mapper
  sets its values, a particle whose age (seconds since its birth) is the lifespan or more dies
  and is gone from then on. A child of the Particle Exploder takes its parent's lifespan, counted
  from its own birth. An endless lifespan stays endless under every operator (times 0 is no
  number, and no age reaches it).
- `kinetic_friction` (pixels a second a second, 0 when born): in every step, before the move,
  a moving particle's speed is cut by K h, never below 0, its direction kept. 0 or below does
  nothing.
- `static_friction` (pixels a second a second, 0 when born): after the kinetic cut, a particle
  that is still stays still for the step (and the Wall is not asked) while the pull on it
  (Gravity and its force) is S or less.
- The Ephemeral mapper's angular velocity, torque and frictions are the operator for one step,
  as its speeds and forces are: the change it made to the angular velocity is taken off again
  after the step. Its X and Y Scale are for the drawing only, and so is its Lifespan: a particle
  is drawn only while its age at the frame drawn is less than the operator's lifespan.
- Children are born with their parent's properties, these included.
- Drawing as D-614's, with the turn o and S_x = Scale X Scale, S_y = Scale Y Scale: a Layer Map
  copy sampled at (W/2, H/2) + (L_x / S_x, L_y / S_y); a piece covering clamp(q S_x / 2 - |L_x| +
  1/2) clamp(q S_y / 2 - |L_y| + 1/2), sampled at O + (L_x / S_x, L_y / S_y) held as before; a
  dot of radii r_x = R S_x and r_y = R S_y, each below 0.7 drawn at 0.7 with the colour times
  r / 0.7, covering clamp(r_x - e + 1/2) when the radii are equal (D-613's disc) and otherwise
  clamp(m (1 - |(L_x / r_x, L_y / r_y)|) + 1/2), m the smaller radius, an ellipse turned with the
  particle.

Not built (D-713 says why): Mass (came later by D-715, after the owner's decision on `FX-PMAP-039`), Gradient
Velocity (no published rule for what a map's slope does), Character and Font Size (no text
particles), Repel (the owner's choice).

**This file never runs the build's code path.** Every case is a project of one composition 16
by 10 at 24 frames a second in `Fixtures/particle_mapper_physics/`, as D-614's. Every case is "not
built yet" until B-583 lands (D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/particle_mapper_physics_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed  # noqa: E402
from adjust_reference import srgb_to_linear  # noqa: E402
import particle_mapper_reference as M  # noqa: E402

G, P, L = M.G, M.P, M.L
W, H, FPS = M.W, M.H, M.FPS
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "particle_mapper_physics"
TOLERANCE = 2e-5
NEW = ("x_scale", "y_scale", "angular_velocity", "torque", "lifespan", "kinetic_friction",
       "static_friction")
PROPERTIES = M.PROPERTIES + NEW
G.WORDS = [(n, PROPERTIES if words is M.PROPERTIES else words, d) for n, words, d in G.WORDS]
G.OUT = OUT
KEYS = dict(x_scale="sx", y_scale="sy", angular_velocity="w", torque="tq", lifespan="life",
            kinetic_friction="mk", static_friction="ms")
TIES = []  # (pull - static, age - lifespan) near-ties, for clear()

_particle = G.particle


def particle(*a, **k):
    p = _particle(*a, **k)
    p.update(sx=1.0, sy=1.0, w=0.0, tq=0.0, life=math.inf, mk=0.0, ms=0.0)
    return p


G.particle = particle
_children = G.children


def children(run, p, t):
    out = _children(run, p, t)
    for ch in out:
        ch.update({k: p[k] for k in KEYS.values()})
    return out


G.children = children
_put = M.put


def put(p, prop, v):
    if prop in KEYS:
        p[KEYS[prop]] = v
    else:
        _put(p, prop, v)


M.put = put


def step(run, p, t0, h):
    a = [0.0, 0.0]
    if G.affected(run, 2, p, t0):
        f = run.at("gravity_force", t0) + run.at("gravity_random_spread", t0) * (run.rr(p, 7) - 0.5)
        gx, gy = G.heading(run.at("gravity_direction", t0))
        a = [f * gx, f * gy]
    force, v_base = list(p["force"]), list(p["v"])
    v_eff = list(v_base)
    w_base = p["w"]
    w, tq, mk, ms = w_base, p["tq"], p["mk"], p["ms"]
    op = run.c["ephemeral_operator"]
    for prop, val in M.mapped(run, "ephemeral", p, t0):
        if prop in ("x_force", "y_force"):
            i = prop[0] == "y"
            force[i] = M.operate(op, force[i], val)
        elif prop in ("x_velocity", "y_velocity"):
            i = prop[0] == "y"
            v_eff[i] = M.operate(op, v_eff[i], val)
        elif prop == "angular_velocity":
            w = M.operate(op, w, val)
        elif prop == "torque":
            tq = M.operate(op, tq, val)
        elif prop == "kinetic_friction":
            mk = M.operate(op, mk, val)
        elif prop == "static_friction":
            ms = M.operate(op, ms, val)
    a = [a[0] + force[0], a[1] + force[1]]
    v = list(v_eff)
    speed = math.hypot(v[0], v[1])
    if mk > 0 and speed > 0:
        k = max(0.0, speed - mk * h) / speed
        v = [v[0] * k, v[1] * k]
    p["v"] = v
    p["angle"] = p["angle"] + w * h + tq * h * h / 2
    p["w"] = w + tq * h
    pull = math.hypot(a[0], a[1])
    if v[0] == 0 and v[1] == 0 and ms != 0:
        TIES.append(pull - ms)
    if not (v[0] == 0 and v[1] == 0 and pull <= ms):
        if run.wall is not None and G.affected(run, 3, p, t0):
            G.bounce_move(run, p, a, h)
        else:
            x = p["x"]
            p["x"] = [x[i] + v[i] * h + a[i] * h * h / 2 for i in range(2)]
            p["v"] = [v[i] + a[i] * h for i in range(2)]
    if v_eff != v_base:
        p["v"] = [p["v"][i] - (v_eff[i] - v_base[i]) for i in range(2)]
    if w != w_base:
        p["w"] = p["w"] - (w - w_base)


M.step = G.step = step


def moved(run, p):
    pts = G.points_from(p["born"], run.u)
    explodes = p["code"] <= 3
    for i, t0 in enumerate(pts):
        if explodes and run.at("particle_exploder_radius", t0) > 0 and G.affected(run, 0, p, t0):
            out = []
            for ch in G.children(run, p, t0):
                out += moved(run, ch)
            return out
        for prop, val in M.mapped(run, "persistent", p, t0):
            put(p, prop, val)
        age = (t0 - p["born"]) / FPS
        if p["life"] != math.inf:
            TIES.append(age - p["life"])
        if age >= p["life"]:
            return []
        if i + 1 < len(pts):
            step(run, p, t0, (pts[i + 1] - t0) / FPS)
    return [p]


M.moved = G.moved = moved


def playground(c, n):
    out = [[0.0] * 4 for _ in range(W * H)]
    run = M.Run(c, n)
    if run.u < 0:
        return out
    op = c["ephemeral_operator"]
    for p in G.everyone(run):
        X, Y = p["x"]
        rgb = p["rgb"]
        opacity, scale, angle = p["opacity"], p["scale"], p["angle"]
        sx, sy, life = p["sx"], p["sy"], p["life"]
        for prop, val in M.mapped(run, "ephemeral", p, run.u):
            if prop == "x":
                X = M.operate(op, X, val)
            elif prop == "y":
                Y = M.operate(op, Y, val)
            elif prop in M.CHANNELS:
                rgb = M.coded(p) if rgb is None else list(rgb)
                i = M.CHANNELS.index(prop)
                rgb[i] = M.operate(op, rgb[i], val)
            elif prop == "opacity":
                opacity = M.operate(op, opacity, val)
            elif prop == "scale":
                scale = M.operate(op, scale, val)
            elif prop == "angle":
                angle = M.operate(op, angle, val)
            elif prop == "x_scale":
                sx = M.operate(op, sx, val)
            elif prop == "y_scale":
                sy = M.operate(op, sy, val)
            elif prop == "lifespan":
                life = M.operate(op, life, val)
        k = P.clamp(opacity)
        if scale <= 0 or k <= 0 or sx <= 0 or sy <= 0:
            continue
        age = (run.u - p["born"]) / FPS
        if life == life and life != math.inf:
            TIES.append(age - life)
        if age >= life:
            continue
        SX, SY = scale * sx, scale * sy
        o = G.turn_of(run, p) + math.radians(angle)
        if run.layer_map is not None and G.affected(run, 1, p, run.u):
            m = run.layer_map
            for gy in range(H):
                for gx in range(W):
                    lx, ly = G.local(o, gx + 0.5 - X, gy + 0.5 - Y)
                    s = L.bilinear(m, W / 2 + lx / SX, H / 2 + ly / SY)
                    if s[3] > 0:
                        out[gy * W + gx] = G.lay(out[gy * W + gx], [v * k for v in s])
            continue
        if p["code"] == 3:
            q = p["side"]
            hold = q / 2 - 0.5
            for gy in range(H):
                for gx in range(W):
                    lx, ly = G.local(o, gx + 0.5 - X, gy + 0.5 - Y)
                    cv = P.clamp(q * SX / 2 - abs(lx) + 0.5) * P.clamp(q * SY / 2 - abs(ly) + 0.5)
                    if cv <= 0:
                        continue
                    s = L.bilinear(run.explode, p["origin"][0] + P.clamp(lx / SX, -hold, hold),
                                   p["origin"][1] + P.clamp(ly / SY, -hold, hold))
                    out[gy * W + gx] = G.lay(out[gy * W + gx], [v * cv * k for v in s])
            continue
        col = p["col"]
        if rgb is not None:
            col = [srgb_to_linear(P.clamp(v)) * col[3] for v in rgb] + [col[3]]
        rx, ry, kk = p["rad"] * SX, p["rad"] * SY, 1.0
        if rx < 0.7:
            kk, rx = kk * (rx / 0.7), 0.7
        if ry < 0.7:
            kk, ry = kk * (ry / 0.7), 0.7
        mm = min(rx, ry)
        for gy in range(H):
            for gx in range(W):
                dx, dy = gx + 0.5 - X, gy + 0.5 - Y
                if rx == ry:
                    e = rx - math.hypot(dx, dy)
                else:
                    lx, ly = G.local(o, dx, dy)
                    e = mm * (1 - math.hypot(lx / rx, ly / ry))
                cv = P.clamp(e + 0.5) * kk * k
                if cv > 0:
                    out[gy * W + gx] = G.lay(out[gy * W + gx], [v * cv for v in col])
    return out


M.playground = G.playground = playground


# --- the cases ------------------------------------------------------------------------------

case = G.case
PG, EG, BL, FT, SH, WL, PF, EF = M.PG, M.EG, M.BL, M.FT, M.SH, M.WL, M.PF, M.EF
PB = dict(BL, persistent_map_layer="ramp")
EB = dict(BL, ephemeral_map_layer="ramp")
# The grid falling under a gentle Gravity of 40.
FALL = dict(PG, gravity_force=40)
EFALL = dict(EG, gravity_force=40)


def per(prop, lo, hi, ch="red", m="persistent"):
    return {f"{m}_map_{ch}_to": prop, f"{m}_min_{ch}": lo, f"{m}_max_{ch}": hi}


def eph(prop, lo, hi, ch="red", op="set"):
    return dict(per(prop, lo, hi, ch, "ephemeral"), ephemeral_operator=op)


CASES = {
    "PM-001": ("Persistent, red to X Scale 0.5 to 3: the grid's dots stretched wider from left "
               "to right.", case(**dict(PG, **per("x_scale", 0.5, 3))), [6]),
    "PM-002": ("Persistent, red to Y Scale 2 and green to Angle 0 to 90: tall dots, the lower "
               "row turned further.",
               case(**dict(PG, **per("y_scale", 2, 2), **per("angle", 0, 90, "green"))), [6]),
    "PM-003": ("Persistent, red to Angular Velocity 0 to 360: the exploded holder's pieces spin, "
               "faster to the right.", case(**dict(PB, **per("angular_velocity", 0, 360))), [12]),
    "PM-004": ("Persistent, red to Torque -720 to 720: pieces on the left spin up one way, on "
               "the right the other.", case(**dict(PB, **per("torque", -720, 720))), [12]),
    "PM-005": ("Persistent, red to Lifespan 0.13 to 0.73 seconds: the fountain's dots die young "
               "on the left.", case(**dict(PF, **per("lifespan", 0.13, 0.73))), [24]),
    "PM-006": ("Persistent Lifespan 0.51 seconds everywhere: only the fountain's dots younger "
               "than that are drawn.", case(**dict(PF, **per("lifespan", 0.51, 0.51))), [24]),
    "PM-007": ("Persistent Kinetic Friction 15: the fountain's dots slow and stay lower.",
               case(**dict(PF, **per("kinetic_friction", 15, 15))), [24]),
    "PM-008": ("Persistent Static Friction 50 on the grid falling at 40: it stays where it was "
               "born.", case(**dict(FALL, **per("static_friction", 50, 50))), [6]),
    "PM-009": ("Persistent Static Friction 30 on the grid falling at 40: not enough, it falls "
               "as without it.", case(**dict(FALL, **per("static_friction", 30, 30))), [6]),
    "PM-010": ("The wall with Persistent Kinetic Friction 8: the dots still bounce inside the "
               "box, slowing.",
               case(masks=[G.BOX], **dict(WL, persistent_map_layer="ramp",
                                          **per("kinetic_friction", 8, 8))), [36]),
    "PM-011": ("Persistent, red to X Scale 0.5 to 1.5 on the Layer Map: each grid point the logo, "
               "narrower on the left.",
               case(**dict(PG, particles_across=2, particles_down=1, grid_width=10,
                           grid_position=[50, 55], layer_map="logo",
                           **per("x_scale", 0.5, 1.5))), [3]),
    "PM-012": ("Persistent, green to Y Scale 0.5 to 2 on the exploded holder: pieces squashed "
               "at the top, drawn out at the bottom.",
               case(**dict(PB, **per("y_scale", 0.5, 2, "green"))), [6]),
    "PM-013": ("Fireworks, Persistent Lifespan 0.8 seconds: the shells burst, and each spark "
               "lives 0.8 seconds from its own birth: at frame 44 the first burst is gone and the "
               "second shell still climbing.",
               case(**dict(SH, persistent_map_layer="ramp", **per("lifespan", 0.8, 0.8))), [44]),
    "PM-014": ("Persistent for the Cannon only, red to X Scale 3: the grid's dots stay round, "
               "the fountain's stretched.",
               case(**M.both_grids(persistent_map_layer="ramp", persistent_particles_from="cannon",
                                   **per("x_scale", 3, 3))), [24]),
    "PM-015": ("Persistent, Max Red keyed 0 to 720 over frames 0 to 12, red to Torque: the "
               "pieces spin up harder as time goes on.",
               case(**dict(PB, **per("torque", 0, keyed((0, 0), (12, 720))))), [12]),
    "PM-016": ("Persistent X Scale 2 with Auto-Orient: the fountain's dots stretched along the "
               "way they move.", case(**dict(PF, auto_orient="on", **per("x_scale", 2, 2))),
               [24]),
    "PE-017": ("Ephemeral Set, red to X Scale 0 to 3: drawn stretched by where they are, "
               "narrow on the left.", case(**dict(EG, **eph("x_scale", 0, 3))), [6]),
    "PE-018": ("Ephemeral Multiply, green to Y Scale 1 to 3: the lower row drawn taller.",
               case(**dict(EG, **eph("y_scale", 1, 3, "green", "multiply"))), [6]),
    "PE-019": ("Ephemeral Add, red to Angular Velocity 0 to 360: pieces spin by where they are, "
               "the spin not kept.", case(**dict(EB, **eph("angular_velocity", 0, 360, op="add"))),
               [12]),
    "PE-020": ("Ephemeral Set, red to Torque -720 to 720: the pieces' spin pushed while they "
               "stand there.", case(**dict(EB, **eph("torque", -720, 720))), [12]),
    "PE-021": ("Ephemeral Set, red to Lifespan 0 to 1 second: a dot drawn only while younger "
               "than the ramp under it says.", case(**dict(EF, **eph("lifespan", 0, 1))), [24]),
    "PE-022": ("Ephemeral Multiply, green to Lifespan: an endless lifespan stays endless, the "
               "fountain as without it.",
               case(**dict(EF, **eph("lifespan", 0, 1, "green", "multiply"))), [24]),
    "PE-023": ("Ephemeral Set, red to Kinetic Friction 0 to 40: dots on the right braked, only "
               "while there.", case(**dict(EF, **eph("kinetic_friction", 0, 40))), [24]),
    "PE-024": ("Ephemeral Set, red to Static Friction 0 to 80 on the grid falling at 40: the "
               "right half stays, the left half falls.",
               case(**dict(EFALL, **eph("static_friction", 0, 80))), [6]),
    "PE-025": ("Both: Persistent red to X Scale 2, Ephemeral Set green to Y Scale 0.5 to 2.",
               case(**dict(PG, ephemeral_map_layer="ramp", **per("x_scale", 2, 2),
                           **eph("y_scale", 0.5, 2, "green"))), [6]),
}
FRAMES_OF = {fx: frames for fx, (_, _, frames) in CASES.items()}
assert all(G.in_range(c) for _, c, _ in CASES.values())

WARNED = {}

INVALID = {
    "PM-026": ("Persistent Map Red To \"gradient_velocity\", not built.",
               case(persistent_map_red_to="gradient_velocity")),
    "PE-027": ("Ephemeral Map Green To \"character\", not built.",
               case(ephemeral_map_green_to="character")),
    "PE-028": ("Ephemeral Map Blue To \"font_size\", not built.",
               case(ephemeral_map_blue_to="font_size")),
}

LISTED = [("PM-005", 24), ("PM-007", 24), ("PM-008", 6), ("PM-010", 36), ("PM-013", 44),
          ("PE-023", 24), ("PE-024", 6)]


def fx_id(fx):
    return "FX-PMPH-" + fx.split("-")[1]


def write(fx, c, frames):
    name = f"fx_pmph_{fx.split('-')[1]}.json"
    (OUT / name).write_text(json.dumps(G.project_json(fx_id(fx), c, frames), indent=2) + "\n",
                            encoding="utf-8")
    return name


def listed():
    out = {"tolerance": 1e-7, "cases": {}}
    for fx, f in LISTED:
        run = M.Run(CASES[fx][1], f)
        rows = [[p["code"], p["a"], p["b"], p["born"]] + p["x"] + p["v"] for p in G.everyone(run)]
        out["cases"][fx_id(fx)] = {"frame": f, "particles": rows}
        print(f"{fx_id(fx)} frame {f}: {len(rows)} particles listed")
    (OUT / "particles_particle_mapper_physics.json").write_text(json.dumps(out, indent=1) + "\n",
                                                                encoding="utf-8")


def clear():
    """D-614's checks over these cases, and no static friction or lifespan within a hair of
    the pull or the age it is weighed against."""
    M.CASES, M.WARNED = CASES, WARNED
    TIES.clear()
    M.clear()
    for d in TIES:
        assert abs(d) > 1e-9, d
    print("clear (ties)")


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
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = G.plain(c)
        expected["cases"][fx_id(fx)] = {"says": says, "project": write(fx, c, G.length([4])),
                                        "frames": {"0": before, "4": before},
                                        "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx_id(fx)}: invalid")
    (OUT / "expected_particle_mapper_physics.json").write_text(
        json.dumps(expected, indent=1) + "\n", encoding="utf-8")
    listed()
    check(expected)


def check(expected):
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    g = lambda n, f: c["FX-PMPH-%03d" % n][str(f)]  # noqa: E731
    covered = lambda fr: sum(1 for p in fr if p[3] > 0)  # noqa: E731
    total = lambda fr: sum(p[3] for p in fr)  # noqa: E731
    R = lambda d, f: G.render(case(**d), f)  # noqa: E731
    # every case differs from the same shot without the new property
    for n, d, f in ((1, PG, 6), (2, PG, 6), (3, PB, 12), (4, PB, 12), (5, PF, 24), (6, PF, 24),
                    (7, PF, 24), (8, FALL, 6), (11, None, 3), (12, PB, 6), (13, SH, 44),
                    (15, PB, 12), (16, dict(PF, auto_orient="on"), 24), (17, EG, 6), (18, EG, 6),
                    (19, EB, 12), (20, EB, 12), (21, EF, 24), (23, EF, 24), (24, EFALL, 6),
                    (25, PG, 6)):
        if d is not None:
            assert g(n, f) != R(d, f), n
    assert g(10, 36) != G.render(case(masks=[G.BOX], **dict(WL, persistent_map_layer="ramp")), 36)
    # static friction above the pull holds the grid where it was born; below it, no change
    assert g(8, 6) == R(PG, 6)
    assert g(9, 6) == R(FALL, 6)
    # an endless lifespan times anything stays endless
    assert g(22, 24) == R(EF, 24)
    # lifespans draw fewer dots; stretching draws more
    assert covered(g(5, 24)) < covered(R(PF, 24)) and covered(g(6, 24)) < covered(R(PF, 24))
    assert total(g(1, 6)) > total(R(PG, 6))
    # the Cannon-only stretch leaves the grid's dots round: the grid's own rows unchanged
    base = R(M.both_grids(persistent_map_layer="ramp"), 24)
    assert g(14, 24) != base
    print("checked")


if __name__ == "__main__":
    main()
