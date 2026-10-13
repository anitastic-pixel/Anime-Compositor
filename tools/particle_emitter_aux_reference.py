"""Particle Emitter's second system, worked a second way.

D-711 (B-581) closes the first of EFFECTS.md row 367's gaps on `core.particle_emitter` (D-615,
`tools/particle_emitter_reference.py`, here E): particles thrown by particles, after what
Trapcode Particular's auxiliary system is for (trails behind each particle, sparks where a
particle bounces). Red Giant publishes no formula and no settings when added, so every rule and
number below is ours. No text or code was copied, and no open-source code is used.

The settings added, after E's (all keyable; lengths in the layer's pixels, speeds in pixels a
second, times in seconds, the rest per cent):

- `aux_particles_per_second` 0 to 1000, 30 when added: how many each particle throws a second
  (Continuously).
- `aux_particles_per_bounce` 0 to 1000, 10: how many each bounce throws (At Bounce).
- `aux_life` 0 to 1000 seconds, 1.
- `aux_velocity` -100000 to 100000, 10: how fast they are thrown, every way.
- `aux_inherit_velocity` -1000 to 1000, 0: how much of the thrower's speed they keep.
- `aux_size` 0 to 10000, 2; `aux_size_at_death` 0 to 1000, 100 (per cent of the size).
- `aux_opacity` 0 to 100, 100; `aux_opacity_at_death` 0 to 100, 0 (per cent of the opacity).

Words: `aux_emit` "off", "continuously" or "at_bounce", "off" when added; `aux_particle_type`
one of E's five, "sphere"; `aux_set_color` "from_main" or "own", "from_main". Colour:
`aux_color`, "#ffffff". A file written before D-711 has none of them: each reads as added, so
the second system is off and every FX-PEMIT case draws as before.

The rules, with E's names (u the frame drawn after the in point, b_k particle k's birth, L_k its
life in frames, r(k, s) D-610's randomness):

1. Off: E's drawing, nothing more.
2. The thrower's path. Particle k is stepped as E steps it (from b_k to the next quarter frame,
   then quarter to quarter) while the step starts before its death (t0 < b_k + L_k) and ends by
   u, keeping (t, x, v, bounces) at b_k and at each step's end.
3. Continuously: q = Aux Particles/sec (read at b_k) / fps a frame; aux particle j (0 up) is born
   at c_j = b_k + (j + 1/2) / q, while c_j < b_k + L_k and c_j <= u, at the place and speed
   straight-line between the two kept points round it (t0 < c_j <= t1).
4. At Bounce: for each kept step whose bounce count rises from n0 to n1, and each bounce m from n0
   to n1 - 1, N = floor(Aux Particles/Bounce at t1) aux particles i = 0 to N - 1, numbered
   j = 1000 m + i, born at t1 at the thrower's place and speed then. Only E's Floor Action Bounce
   counts bounces.
5. Aux particle j of k is numbered K = -1 - (2^32 k + j) (taken as a 64-bit number), so it has its
   own randomness r(K, s). It is thrown at Aux Velocity (read at c) / W along the uniform
   direction z = 1 - 2 r(K, 0), turn 2 pi r(K, 1), plus Aux Inherit Velocity / 100 times the
   thrower's speed, and then moves on E's steps from c (gravity, air resistance, wind, turbulence
   and the floor, its bounces by its own randomness). Its life is Aux Life (read at c) fps frames;
   alive at u when u - c is less, f = (u - c) / life its age fraction.
6. Size Aux Size (read at c) times 1 + (Aux Size at Death / 100 - 1) f; opacity Aux Opacity / 100
   (read at c) times 1 + (Aux Opacity at Death / 100 - 1) f (the two at-death levels read at the
   frame drawn). Colour, linear light: From Main the thrower's colour at c by E's Set Color (Over
   Life at min(1, (c - b_k) / L_k)) times its brightness; Own the Aux Color. Drawn as Aux Particle
   Type by E's sprites with E's Sphere Feather, a streak along its own speed.
7. Every thrower that may still have an aux particle alive is worked: E's births are read with
   the longest life widened to the largest Life plus the largest Aux Life plus one frame, over the
   frames sampled. Throwers and aux particles are drawn together, farthest first, equal depths by
   number (aux particles' numbers are below every thrower's), with E's transfer mode.

**This file never runs the build's code path.** It reuses E (and D-610's
`tools/particle_space_reference.py`, here P) in its own process, adding the settings, words and
colour here only; neither file's fixtures are written again. The drawing is Broadcast Colors' 16
by 10. The projects go into `Fixtures/particle_emitter_aux`, the expected frames into
`Fixtures/particle_emitter_aux/expected_particle_emitter_aux.json`, each alive particle's numbers
for some cases into `Fixtures/particle_emitter_aux/particles_particle_emitter_aux.json`. Every
case is "not built yet" until B-581 lands (D-566).

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/particle_emitter_aux_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed  # noqa: E402
import particle_emitter_reference as E  # noqa: E402
import particle_space_reference as P  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H, FPS = E.W, E.H, E.FPS
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "particle_emitter_aux"
KINDS = ("sphere", "glow_sphere", "star", "cloudlet", "streak")

# D-711's settings, words and colour, in this process only.
E.SETTINGS = E.SETTINGS + [
    ("aux_particles_per_second", 0, 1000, 30), ("aux_particles_per_bounce", 0, 1000, 10),
    ("aux_life", 0, 1000, 1), ("aux_velocity", -100000, 100000, 10),
    ("aux_inherit_velocity", -1000, 1000, 0), ("aux_size", 0, 10000, 2),
    ("aux_size_at_death", 0, 1000, 100), ("aux_opacity", 0, 100, 100),
    ("aux_opacity_at_death", 0, 100, 0),
]
E.NAMES = [s[0] for s in E.SETTINGS]
E.RANGES = {s[0]: (s[1], s[2]) for s in E.SETTINGS}
E.WORDS = E.WORDS + [
    ("aux_emit", ("off", "continuously", "at_bounce"), "off"),
    ("aux_particle_type", KINDS, "sphere"),
    ("aux_set_color", ("from_main", "own"), "from_main"),
]
E.COLOURS = E.COLOURS + [("aux_color", "#ffffff")]


class Run(E.Run):
    """E's run, its births widened (rule 7) when the second system is on."""

    def __init__(self, c, n):
        super().__init__(c, n)
        self.aux = c["aux_emit"]
        if self.aux != "off":
            wide = (max(s["life"] for s in self.own) + max(s["aux_life"] for s in self.own)
                    + 1 / FPS)
            for s in self.samples:
                s["longevity"] = wide


def life_of(run, k, b):
    return run.mine("life", b) * FPS * (1 - run.mine("life_random", b) / 100 * run.r(k, 11))


def path(run, k, b, life):
    """Rule 2: the thrower as E moves it, and the kept points."""
    p = E.born(run, k, b)
    kept = [(b, list(p["x"]), list(p["v"]), 0)]
    t0 = b
    for j in range(math.floor(4 * b) + 1, 4 * run.u + 1):
        if t0 >= b + life:
            break
        t1 = j / 4
        P.step(run, p, t0, (t1 - t0) / FPS)
        kept.append((t1, list(p["x"]), list(p["v"]), p["bounces"]))
        t0 = t1
    return p, kept


def aux_key(k, j):
    return -1 - ((k << 32) + j)


def throws(run, k, b, life, kept):
    """Rules 3 and 4: (j, c, x, v) for each aux particle thrown by c <= u."""
    out = []
    if run.aux == "continuously":
        q = run.mine("aux_particles_per_second", b) / FPS
        if q <= 0:
            return out
        j, i = 0, 1
        while True:
            c = b + (j + 0.5) / q
            if c >= b + life or c > run.u:
                return out
            while kept[i][0] < c:
                i += 1
            (t0, x0, v0, _), (t1, x1, v1, _) = kept[i - 1], kept[i]
            s = (c - t0) / (t1 - t0)
            out.append((j, c, [x0[a] + (x1[a] - x0[a]) * s for a in range(3)],
                        [v0[a] + (v1[a] - v0[a]) * s for a in range(3)]))
            j += 1
    for (_, _, _, n0), (t1, x1, v1, n1) in zip(kept, kept[1:]):
        for m in range(n0, n1):
            n = math.floor(run.mine("aux_particles_per_bounce", t1))
            for i in range(n):
                out.append((1000 * m + i, t1, list(x1), list(v1)))
    return out


def main_colour(run, thrower, b, life, c):
    mode = run.own_c["set_color"]
    if mode == "at_birth":
        col = P.linear(run.own_c["color"])
    else:
        col = E.gradient(run.own_c, min(1.0, (c - b) / life) if mode == "over_life"
                         else thrower["pick"])
    return [v * thrower["bright"] for v in col]


def aux_alive(run):
    """(K, particle, f, own) for every aux particle alive at u, by thrower then number."""
    out = []
    if run.aux == "off":
        return out
    for k, b in P.births(run):
        life = life_of(run, k, b)
        thrower = E.born(run, k, b)
        _, kept = path(run, k, b, life)
        for j, c, x, vm in throws(run, k, b, life, kept):
            alife = run.mine("aux_life", c) * FPS
            if not run.u - c < alife:
                continue
            key = aux_key(k, j)
            r = lambda s: run.r(key, s)  # noqa: E731
            d = P.sphere(1 - 2 * r(0), 2 * math.pi * r(1))
            speed = run.mine("aux_velocity", c) / W
            inherit = run.mine("aux_inherit_velocity", c) / 100
            v = [d[a] * speed + vm[a] * inherit for a in range(3)]
            p = {"k": key, "b": c, "x": list(x), "v": v, "x0": list(x), "twirl": 1.0,
                 "glued": False, "iced": False, "bounces": 0}
            points = [c] + [i / 4 for i in range(math.floor(4 * c) + 1, 4 * run.u + 1)]
            for t0, t1 in zip(points, points[1:]):
                P.step(run, p, t0, (t1 - t0) / FPS)
            col = (P.linear(run.own_c["aux_color"]) if run.own_c["aux_set_color"] == "own"
                   else main_colour(run, thrower, b, life, c))
            own = {"size": run.mine("aux_size", c), "alpha": run.mine("aux_opacity", c) / 100,
                   "col": col, "life": alife}
            out.append((key, p, (run.u - c) / alife, own))
    return out


def sprites(run):
    """E's sprites and the aux particles', farthest first, equal depths by number."""
    out = E.sprites(run)
    if run.aux == "off":
        return out
    c, now = run.own_c, run.own_now
    see = P.camera(run, E.view_of(c))
    feather = now["sphere_feather"] / 100
    kind = c["aux_particle_type"]
    for key, p, f, own in aux_alive(run):
        s = see(p["x"])
        if s is None:
            continue
        X, Y, ppu, depth = s
        size = own["size"] * (1 + (now["aux_size_at_death"] / 100 - 1) * f)
        rad = size * ppu / (2 * W)
        alpha = own["alpha"] * (1 + (now["aux_opacity_at_death"] / 100 - 1) * f)
        sp = {"kind": kind, "X": X, "Y": Y, "col": own["col"]}
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
        out.append((depth, key, sp, p))
    out.sort(key=lambda e: (-e[0], e[1]))
    return out


def particle_emitter(px, c, n):
    """E's drawing with the aux particles."""
    run = Run(c, n)
    if run.u < 0:
        return [list(p) for p in px]
    out = [[0.0] * 4 for _ in px]
    mode = E.TRANSFER[c["transfer_mode"]]
    for _, _, sp, _ in sprites(run):
        xs, ys = E.reach(sp)
        for gy in range(max(0, math.floor(ys[0])), min(H, math.ceil(ys[1]) + 1)):
            for gx in range(max(0, math.floor(xs[0])), min(W, math.ceil(xs[1]) + 1)):
                s = E.cover(sp, gx + 0.5, gy + 0.5)
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


def render(c, frame_no):
    out = particle_emitter(P.working(), c, frame_no)
    move = c["shift"]
    if c["camera"] is not None:
        move += int(W / 2 - c["camera"][0])
    return R.frame(out, move)


# --- the cases ------------------------------------------------------------------------------

case = E.case
# One thrower crossing the middle to the right at 6 pixels a second, 1.3 born a second (the first
# at frame 9.23), each living 2 seconds, leaving a trail of still dots, one a frame.
TRAIL = dict(E.ONE, velocity=6, direction="directional", direction_spread=0,
             emitter_rotation_z=90, emitter_position=[30, 50], size=3,
             aux_emit="continuously", aux_particles_per_second=24, aux_velocity=0,
             aux_life=0.5, aux_size=1.5, aux_opacity_at_death=0)
# A fountain from the top falling on a floor at 80 per cent, bouncing.
FOUNTAIN = dict(E.FLOW, particles_per_second=8, emitter_position=[50, 20], gravity=120, velocity=10,
                floor_action="bounce", floor_position=80, aux_emit="at_bounce",
                aux_particles_per_bounce=4, aux_velocity=15, aux_life=0.4, aux_size=1)

CASES = {
    "FX-PEAUX-001": ("The settings as added, the second system off: FX-PEMIT-001 at frames 0, 6 "
                     "and 12.", case(), [0, 6, 12]),
    "FX-PEAUX-002": ("A trail: one particle crossing to the right leaves a still dot a frame, "
                     "each fading out over half a second, at frames 20 and 30.",
                     case(**TRAIL), [20, 30]),
    "FX-PEAUX-003": ("The trail's dots shrinking to nothing instead of fading (Aux Size at Death "
                     "0, Aux Opacity at Death 100).",
                     case(**dict(TRAIL, aux_size_at_death=0, aux_opacity_at_death=100)), [20]),
    "FX-PEAUX-004": ("Aux Velocity 20: the dots thrown out every way as it goes.",
                     case(**dict(TRAIL, aux_velocity=20)), [20]),
    "FX-PEAUX-005": ("Aux Inherit Velocity 100 and no Aux Velocity: the dots ride along with "
                     "the particle, over it.",
                     case(**dict(TRAIL, aux_inherit_velocity=100)), [20]),
    "FX-PEAUX-006": ("Aux Set Color Own, red: a red trail behind a white particle.",
                     case(**dict(TRAIL, aux_set_color="own", aux_color="#ff3020")), [20]),
    "FX-PEAUX-007": ("The particle coloured Over Life, the trail From Main: each dot the "
                     "particle's colour when it was dropped.",
                     case(**dict(TRAIL, set_color="over_life", life=1)), [20]),
    "FX-PEAUX-008": ("Aux Particle Type Streak, thrown at 20: short lines.",
                     case(**dict(TRAIL, aux_particle_type="streak", aux_velocity=20)), [20]),
    "FX-PEAUX-009": ("Glowing aux spheres 3 across, Add.",
                     case(**dict(TRAIL, aux_particle_type="glow_sphere", aux_size=3,
                                 transfer_mode="add")), [20]),
    "FX-PEAUX-010": ("At Bounce: the fountain's particles throw four sparks at each bounce on "
                     "the floor, at frames 24 and 36.", case(**FOUNTAIN), [24, 36]),
    "FX-PEAUX-011": ("At Bounce with Floor Action None: no bounce, so nothing thrown; "
                     "FX-PEMIT-028's burst.", case(**dict(E.FLOW, aux_emit="at_bounce")), [12]),
    "FX-PEAUX-012": ("The particle lives 0.5 seconds, its trail 1 second: at frame 24 the "
                     "particle is gone and its trail is still fading.",
                     case(**dict(TRAIL, life=0.5, aux_life=1)), [18, 24]),
    "FX-PEAUX-013": ("Gravity 40 and Aux Velocity 10: the sparks fall as they go.",
                     case(**dict(TRAIL, gravity=40, aux_velocity=10)), [20]),
    "FX-PEAUX-014": ("Aux Particles/sec keyed from 0 at frame 0 to 48 at frame 20 on a burst: "
                     "read at each thrower's birth, so the later ones leave thicker trails.",
                     case(**dict(E.FLOW, aux_emit="continuously", aux_velocity=0,
                                 aux_life=0.3, aux_size=1,
                                 aux_particles_per_second=keyed((0, 0), (20, 48)))), [24]),
    "FX-PEAUX-015": ("Aux Opacity at Death keyed from 0 at frame 0 to 100 at frame 20: read at "
                     "the frame drawn.",
                     case(**dict(TRAIL, aux_opacity_at_death=keyed((0, 0), (20, 100)))), [20]),
    "FX-PEAUX-016": ("FX-PEAUX-002 with the layer starting at frame 10: frame 30 the same as "
                     "its frame 20.", case(in_frame=10, **TRAIL), [10, 30]),
    "FX-PEAUX-017": ("FX-PEAUX-004 with the layer moved 3 pixels right.",
                     case(shift=3, **dict(TRAIL, aux_velocity=20)), [20]),
    "FX-PEAUX-018": ("Random Seed 7 on FX-PEAUX-004: different sparks.",
                     case(**dict(TRAIL, aux_velocity=20, random_seed=7)), [20]),
    "FX-PEAUX-019": ("Aux Particles/sec 0: nothing thrown; the particle alone.",
                     case(**dict(TRAIL, aux_particles_per_second=0)), [20]),
    "FX-PEAUX-020": ("Aux Life 0: nothing thrown lives; the particle alone.",
                     case(**dict(TRAIL, aux_life=0)), [20]),
    "FX-PEAUX-021": ("The sparking trail through a composition camera 12 pixels in front, the "
                     "emitter 6 pixels behind the layer: the sparks bigger as they come nearer.",
                     case(camera=(8, 5, -12, 12),
                          **dict(TRAIL, aux_velocity=20, emitter_z=6)), [20]),
    "FX-PEAUX-022": ("A burst every way, each particle trailing short-lived dots, at frames 6, "
                     "12 and 24.",
                     case(**dict(E.FLOW, aux_emit="continuously", aux_particles_per_second=12,
                                 aux_velocity=0, aux_life=0.25, aux_size=1,
                                 aux_opacity=60)), [6, 12, 24]),
}
E.FRAMES_OF.update({fx: frames for fx, (_, _, frames) in CASES.items()})
assert all(E.in_range(c) for _, c, _ in CASES.values())

INVALID = {
    "FX-PEAUX-023": ("Aux Emit \"trail\": not one of off, continuously, at_bounce.",
                     case(aux_emit="trail")),
    "FX-PEAUX-024": ("Aux Particle Type \"sprite\".", case(aux_particle_type="sprite")),
    "FX-PEAUX-025": ("Aux Set Color \"gradient\".", case(aux_set_color="gradient")),
    "FX-PEAUX-026": ("Aux Color \"#12345\": not a colour.", case(aux_color="#12345")),
    "FX-PEAUX-027": ("Aux Particles/sec 1001, above 1000.",
                     case(aux_particles_per_second=1001)),
    "FX-PEAUX-028": ("Aux Life -1, below 0.", case(aux_life=-1)),
    "FX-PEAUX-029": ("Aux Inherit Velocity 1001, above 1000.", case(aux_inherit_velocity=1001)),
    "FX-PEAUX-030": ("Aux Opacity at Death 101, above 100.", case(aux_opacity_at_death=101)),
}

LISTED = [("FX-PEAUX-002", 20), ("FX-PEAUX-004", 20), ("FX-PEAUX-010", 24),
          ("FX-PEAUX-012", 24), ("FX-PEAUX-013", 20), ("FX-PEAUX-022", 12)]


def listed():
    out = {"tolerance": 1e-7, "cases": {}}
    for fx, f in LISTED:
        run = Run(CASES[fx][1], f)
        rows = [[k, p["b"]] + p["x"] + p["v"] for k, p, _ in E.alive(run)]
        rows += [[key, p["b"]] + p["x"] + p["v"] for key, p, _, _ in aux_alive(run)]
        out["cases"][fx] = {"frame": f, "particles": rows}
        print(f"{fx} frame {f}: {len(rows)} particles listed")
    (OUT / "particles_particle_emitter_aux.json").write_text(json.dumps(out, indent=1) + "\n",
                                                             encoding="utf-8")


def clear():
    """No number lands so near a cut that the build's rounding could put it on the other side:
    a birth near the frame drawn, an age near its life, an aux birth near the frame drawn or the
    thrower's death, an aux age near its life, a near miss of the floor, or two overlapping
    particles at nearly but not quite the same depth."""
    P.NEAR.clear()
    for fx, (_, c, frames) in CASES.items():
        for f in frames:
            run = Run(c, f)
            if run.u < 0:
                continue
            for k, b in P.births(run):
                assert abs(b - run.u) > 1e-6, (fx, f, k)
                life = life_of(run, k, b)
                assert abs((run.u - b) - life) > 1e-6, (fx, f, k)
                if run.aux == "off":
                    continue
                _, kept = path(run, k, b, life)
                assert all(abs(t - (b + life)) > 1e-6 for t, *_ in kept), (fx, f, k)
                for j, cj, _, _ in throws(run, k, b, life, kept):
                    assert abs(cj - run.u) > 1e-6 or cj == round(cj * 4) / 4, (fx, f, k, j)
                    assert abs((run.u - cj) - run.mine("aux_life", cj) * FPS) > 1e-6, (fx, f, k, j)
                if run.aux == "continuously":
                    q = run.mine("aux_particles_per_second", b) / FPS
                    if q > 0:
                        for j in range(math.floor(q * (life + 1)) + 2):
                            cj = b + (j + 0.5) / q
                            assert abs(cj - (b + life)) > 1e-6 and abs(cj - run.u) > 1e-6, \
                                (fx, f, k, j)
            sps = sprites(run)
            for i in range(len(sps)):
                for j in range(i + 1, len(sps)):
                    a, b2 = sps[i][2], sps[j][2]
                    if "R" not in a or "R" not in b2:
                        continue
                    near = math.hypot(a["X"] - b2["X"], a["Y"] - b2["Y"]) < a["R"] + b2["R"] + 2
                    gap = abs(sps[i][0] - sps[j][0])
                    assert not (near and 0 < gap < 1e-7), (fx, f, i, j, gap)
    if P.NEAR:
        assert min(P.NEAR) > 1e-9, min(P.NEAR)
        print(f"clear: nearest miss of the floor {min(P.NEAR):.3g}")
    else:
        print("clear")


def check(expected):
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    e = json.loads((E.OUT / "expected_particle_emitter.json").read_text(encoding="utf-8"))
    pemit = {fx: v["frames"] for fx, v in e["cases"].items()}
    total = lambda fr: sum(p[3] for p in fr)  # noqa: E731
    # off, and the cases that throw nothing, draw as D-615 does
    assert c["FX-PEAUX-001"] == pemit["FX-PEMIT-001"]
    assert c["FX-PEAUX-011"]["12"] == pemit["FX-PEMIT-028"]["12"]
    alone = render(case(**dict(TRAIL, aux_emit="off")), 20)
    assert c["FX-PEAUX-019"]["20"] == alone and c["FX-PEAUX-020"]["20"] == alone
    trail = c["FX-PEAUX-002"]["20"]
    assert total(trail) > total(alone)
    for n in range(3, 10):
        assert c["FX-PEAUX-%03d" % n]["20"] != trail, n
    assert c["FX-PEAUX-010"]["24"] != render(case(**dict(FOUNTAIN, aux_emit="off")), 24)
    assert c["FX-PEAUX-016"]["30"] == trail
    assert c["FX-PEAUX-016"]["10"] == [[0.0] * 4 for _ in range(W * H)]
    moved_, base = c["FX-PEAUX-017"]["20"], c["FX-PEAUX-004"]["20"]
    for y in range(H):
        assert moved_[y * W + 3:(y + 1) * W] == base[y * W:y * W + W - 3]
    assert c["FX-PEAUX-018"]["20"] != base
    assert c["FX-PEAUX-021"]["20"] != base
    # the red trail is red where the particle is not
    red = c["FX-PEAUX-006"]["20"]
    assert any(p[0] > 2 * p[1] and p[3] > 0.1 for p in red)
    # the bounce throws: some aux particles at frame 24, all from a floor contact
    run = Run(CASES["FX-PEAUX-010"][1], 24)
    sparks = aux_alive(run)
    assert sparks and all(p["b"] == round(p["b"] * 4) / 4 for _, p, _, _ in sparks)
    # at frame 24 of FX-PEAUX-012 every thrower is dead, the trail is not
    run = Run(CASES["FX-PEAUX-012"][1], 24)
    assert not E.alive(run) and aux_alive(run)
    print("checked")


def main():
    clear()
    expected = B.write_cases(OUT, "particle_emitter_aux", CASES, INVALID, render, E.plain,
                             E.file_json)
    listed()
    check(expected)


if __name__ == "__main__":
    main()
