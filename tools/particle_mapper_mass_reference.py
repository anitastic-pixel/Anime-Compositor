"""Particle Playground's Mass property, worked a second way.

D-715, the fourth step of `core.particle_playground` (D-613, D-614, D-713, P0-6), by the owner's
decision of 2026-10-13 (FX-PMAP-039 now refuses "repel" instead of "mass"): the property mappers
can also set a particle's Mass. Adobe's page says Mass is the particle's mass, which works with
Force to set how fast it speeds up, and gives no formula, so the rule and every number below are
ours (document 21, D-715). No text or code was copied. The rest is D-613's, D-614's and D-713's,
unchanged: `tools/particle_playground_reference.py`, `tools/particle_mapper_reference.py` and
`tools/particle_mapper_physics_reference.py`, whose rules this file extends and whose code it
runs.

**No state is carried from frame to frame** (document 20): the mass is part of the particle,
worked out along its own steps from its birth like its place and speed.

The word added: `mass`, one more property a mapper's channel can be set to, after D-713's, so
every file written before keeps its meaning. Nothing else is added.

- `mass` (1 when born, no unit): in every step the particle's force (D-614's X and Y Force, with
  the Ephemeral mapper's operator for the step) is divided by its mass before it is added to
  Gravity's pull. Gravity pulls every mass alike, as weight does. At a mass of 0 or below the
  force does nothing (a particle with no mass is not pushed, rather than pushed without limit).
- Static friction (D-713) weighs the pull after the division: a heavier particle is held by
  a smaller static friction.
- The Ephemeral mapper's mass is the operator for one step, as its forces are; it does nothing to
  the drawing.
- Children of the Particle Exploder are born with their parent's mass.

Not built (D-715 says why): Gradient Velocity (no published rule for what a map's slope does),
Character and Font Size (no text particles), Repel (the owner's gap, 2026-10-13).

**This file never runs the build's code path.** Every case is a project of one composition 16
by 10 at 24 frames a second in `Fixtures/particle_mapper_mass/`, as D-614's. Every case is "not
built yet" until B-585 lands (D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/particle_mapper_mass_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed  # noqa: E402
import particle_mapper_physics_reference as PH  # noqa: E402

M, G = PH.M, PH.G
W, H, FPS = M.W, M.H, M.FPS
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "particle_mapper_mass"
TOLERANCE = 2e-5
PROPERTIES = PH.PROPERTIES + ("mass",)
G.WORDS = [(n, PROPERTIES if words is PH.PROPERTIES else words, d) for n, words, d in G.WORDS]
G.OUT = OUT
PH.KEYS["mass"] = "m"  # put() sets it, children() copies it

_particle = G.particle


def particle(*a, **k):
    p = _particle(*a, **k)
    p["m"] = 1.0
    return p


G.particle = particle


def step(run, p, t0, h):
    """D-713's step, the particle's force divided by its mass."""
    a = [0.0, 0.0]
    if G.affected(run, 2, p, t0):
        f = run.at("gravity_force", t0) + run.at("gravity_random_spread", t0) * (run.rr(p, 7) - 0.5)
        gx, gy = G.heading(run.at("gravity_direction", t0))
        a = [f * gx, f * gy]
    force, v_base = list(p["force"]), list(p["v"])
    v_eff = list(v_base)
    w_base = p["w"]
    w, tq, mk, ms, m = w_base, p["tq"], p["mk"], p["ms"], p["m"]
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
        elif prop == "mass":
            m = M.operate(op, m, val)
    if m > 0:
        a = [a[0] + force[0] / m, a[1] + force[1] / m]
    v = list(v_eff)
    speed = PH.math.hypot(v[0], v[1])
    if mk > 0 and speed > 0:
        k = max(0.0, speed - mk * h) / speed
        v = [v[0] * k, v[1] * k]
    p["v"] = v
    p["angle"] = p["angle"] + w * h + tq * h * h / 2
    p["w"] = w + tq * h
    pull = PH.math.hypot(a[0], a[1])
    if v[0] == 0 and v[1] == 0 and ms != 0:
        PH.TIES.append(pull - ms)
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


PH.step = M.step = G.step = step


# --- the cases ------------------------------------------------------------------------------

case = G.case
PG, EG, PB, PF, SH, WL = M.PG, M.EG, PH.PB, M.PF, M.SH, M.WL
per, eph = PH.per, PH.eph
# A constant push down of 60 on the still grid, from the persistent mapper's green.
DOWN = per("y_force", 60, 60, "green")
# A constant push right of 40, from the persistent mapper's blue.
RIGHT = per("x_force", 40, 40, "blue")

CASES = {
    "MA-001": ("Persistent, red to Mass 0.5 to 4, green to Y Force 60: the grid pushed down, the "
               "light dots on the left furthest.", case(**dict(PG, **per("mass", 0.5, 4), **DOWN)),
               [6]),
    "MA-002": ("Persistent Mass 2 with Y Force 60: the grid falls as with Mass 1 and Y Force 30.",
               case(**dict(PG, **per("mass", 2, 2), **DOWN)), [6]),
    "MA-003": ("Persistent Mass 0 with Y Force 60: the force does nothing, the grid stays.",
               case(**dict(PG, **per("mass", 0, 0), **DOWN)), [6]),
    "MA-004": ("Persistent Mass -1 with Y Force 60: as Mass 0, the grid stays.",
               case(**dict(PG, **per("mass", -1, -1), **DOWN)), [6]),
    "MA-005": ("Persistent Mass 3 under Gravity 40 and no force: Gravity pulls every mass alike, "
               "the grid falls as without it.",
               case(**dict(PG, gravity_force=40, **per("mass", 3, 3))), [6]),
    "MA-006": ("The fountain, red to Mass 1 to 5, blue to X Force 40: blown right, the heavy dots "
               "on the right less.", case(**dict(PF, **per("mass", 1, 5), **RIGHT)), [24]),
    "MA-007": ("Ephemeral Set, red to Mass 0.5 to 4, with Persistent Y Force 60: the push by "
               "where each dot is, step by step.",
               case(**dict(PG, ephemeral_map_layer="ramp", **DOWN, **PH.eph("mass", 0.5, 4))), [6]),
    "MA-008": ("Ephemeral Multiply, green to Mass 1 to 4, with Persistent Y Force 60: the lower "
               "row heavier while there.",
               case(**dict(PG, ephemeral_map_layer="ramp", **DOWN,
                           **PH.eph("mass", 1, 4, "green", "multiply"))), [6]),
    "MA-009": ("Persistent Mass 2, Y Force 50 and Static Friction 30 on the grid: the pull is 25, "
               "so it stays; at Mass 1 it would fall.",
               case(**dict(PG, **per("mass", 2, 2), **per("y_force", 50, 50, "green"),
                           **per("static_friction", 30, 30, "blue"))), [6]),
    "MA-010": ("Fireworks, red to Mass 1 to 4, blue to X Force 40: the sparks keep their shell's "
               "mass and drift right.",
               case(**dict(SH, persistent_map_layer="ramp", **per("mass", 1, 4), **RIGHT)), [30]),
    "MA-011": ("The wall with Persistent Mass 2 and Y Force 60: the dots still bounce inside the "
               "box.",
               case(masks=[G.BOX], **dict(WL, persistent_map_layer="ramp", **per("mass", 2, 2),
                                          **DOWN)), [36]),
    "MA-012": ("Persistent, Max Red keyed 1 to 6 over frames 0 to 6, red to Mass, with Y Force 60: "
               "the push weakens as time goes on.",
               case(**dict(PG, **per("mass", 1, keyed((0, 1), (6, 6))), **DOWN)), [6]),
    "MA-013": ("The exploded holder, red to Mass 1 to 4, green to Y Force 0 to 80.",
               case(**dict(PB, **per("mass", 1, 4), **per("y_force", 0, 80, "green"))), [12]),
    "MA-014": ("Persistent for the Cannon only, Mass 4 and X Force 40: the fountain blown right a "
               "quarter as far, the grid's dots not pushed at all.",
               case(**M.both_grids(persistent_map_layer="ramp", persistent_particles_from="cannon",
                                   **per("mass", 4, 4), **RIGHT)), [24]),
}
FRAMES_OF = {fx: frames for fx, (_, _, frames) in CASES.items()}
assert all(G.in_range(c) for _, c, _ in CASES.values())

WARNED = {}

INVALID = {
    "MA-015": ("Persistent Map Red To \"Mass\", a capital M: the words are lowercase.",
               case(persistent_map_red_to="Mass")),
    "MA-016": ("Ephemeral Map Green To \"weight\", not a property.",
               case(ephemeral_map_green_to="weight")),
}

LISTED = [("MA-001", 6), ("MA-006", 24), ("MA-007", 6), ("MA-010", 30), ("MA-011", 36)]


def fx_id(fx):
    return "FX-PMASS-" + fx.split("-")[1]


def write(fx, c, frames):
    name = f"fx_pmass_{fx.split('-')[1]}.json"
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
    (OUT / "particles_particle_mapper_mass.json").write_text(json.dumps(out, indent=1) + "\n",
                                                             encoding="utf-8")


def clear():
    """D-614's and D-713's checks over these cases."""
    M.CASES, M.WARNED = CASES, WARNED
    PH.TIES.clear()
    M.clear()
    for d in PH.TIES:
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
    (OUT / "expected_particle_mapper_mass.json").write_text(
        json.dumps(expected, indent=1) + "\n", encoding="utf-8")
    listed()
    check(expected)


def check(expected):
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    g = lambda n, f: c["FX-PMASS-%03d" % n][str(f)]  # noqa: E731
    R = lambda d, f: G.render(case(**d), f)  # noqa: E731
    pushed = R(dict(PG, **DOWN), 6)
    # every mass changes the pushed shot, except where the rule says it must not
    for n in (1, 2, 7, 8, 12):
        assert g(n, 6) != pushed, n
    assert g(6, 24) != R(dict(PF, **RIGHT), 24)
    assert g(10, 30) != R(dict(SH, persistent_map_layer="ramp", **RIGHT), 30)
    assert g(11, 36) != R(dict(WL, persistent_map_layer="ramp", masks=[G.BOX], **DOWN), 36)
    assert g(13, 12) != R(dict(PB, **per("y_force", 0, 80, "green")), 12)
    # mass 2 under force 60 is force 30; mass 0 or below, no push; gravity alike for every mass
    assert g(2, 6) == R(dict(PG, **per("y_force", 30, 30, "green")), 6)
    assert g(3, 6) == R(PG, 6) and g(4, 6) == R(PG, 6)
    assert g(5, 6) == R(dict(PG, gravity_force=40), 6)
    # static friction holds the heavier grid; at mass 1 the same shot falls
    assert g(9, 6) == R(PG, 6)
    assert R(dict(PG, **per("y_force", 50, 50, "green"),
                  **per("static_friction", 30, 30, "blue")), 6) != R(PG, 6)
    # mass 4 is force 10 for the Cannon; the grid stays where it is
    assert g(14, 24) == R(M.both_grids(persistent_map_layer="ramp",
                                       persistent_particles_from="cannon",
                                       **per("x_force", 10, 10, "blue")), 24)
    print("checked")


if __name__ == "__main__":
    main()
