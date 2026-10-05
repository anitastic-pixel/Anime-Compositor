"""Lightning Bolt's Advanced Lightning extras, worked a second way.

D-324 (from D-308, after P-26's tutorial 2) gives D-190's `core.lightning_bolt` five more
settings, After Effects' Advanced Lightning's in name and purpose and this program's own rule:
the lightning type `kind`, `turbulence`, `decay`, `conductivity` (Conductivity State) and
`obstacle` (Alpha Obstacle). Each one's absent value draws today's bolt, number for number.
Nothing is ported, and no After Effects frame was compared: the types are our reading of Adobe's
one-line descriptions. Document 21 is the rule in words; this file is the reference for the
numbers document 25 pins against it.

The rule, on top of D-190's (`tools/lightning_bolt_reference.py`). O and D are the start and end
points in the drawing's own pixels, R = |D - O|, k_d = 1 - decay / 100, and m = floor(f / hold).

- Conductivity c: with n = floor(c), s = 3 t^2 - 2 t^3 for t = c - n, base_0 the seed's hash and
  base_n = mix(base_0 xor n) for n > 0, every number the bolt draws is r = A + s (B - A), A the
  number from base_n and B from base_(n+1); at s = 0 it is A. Moving c moves the bolt smoothly
  through new shapes; c = 0 is D-190's bolt.
- The bolts the halving starts from, (P, Q, wP, wQ, depth 0, bolt, key 1), wQ = k_d:
  direction, strike and breaking: one, O to D, bolt 0. bouncy: O to D (bolt 0), D to O (bolt -1)
  and O to D again (bolt -2). omni: six, from O to O + turn(D - O, 60 i), bolt -i, i = 0 to 5.
  anywhere: one, from O to O + R (r_b + 1) / 2 (cos a, sin a), a = 180 (r_a + 1) degrees, r_a and
  r_b the numbers of key 1, bolt -1, channels 0 and 1: a new place every `hold` frames.
  vertical: one, from O straight down to the drawing's bottom edge, the end point unused.
  two_way: O to the middle of O and D (bolt 0) and D to the middle (bolt -1).
- Halving i (0 first): the push is D-190's times (1 + turbulence / 100 * i / 2), and a segment
  forks when (r_1 + 1) / 2 < branches / 100 * (1 + turbulence / 100).
- A fork is D-190's, except: strike turns it from the way to D, (D - M) / |D - M| (the segment's
  own way when M is D), not from the segment's way; breaking starts it at the full weight wM, not
  half.
- Alpha Obstacle a: when a > 0, a pixel of the layer, as it is before the bolt is drawn and before
  Composite on Original clears it, blocks when its covering is above 1 - a / 100; outside the
  layer nothing blocks. Each bolt and each fork, from its start, is walked segment by segment at
  points P + (Q - P) j / n, n = max(1, ceil(|Q - P|)), j = 0 to n; at the first point in a
  blocking pixel the bolt ends there (the segment shortened to that point, its weight
  interpolated, or dropped when it is the segment's start) and the rest of it, with every fork
  that starts beyond that point, is gone. A fork is kept only if the point it starts from is
  still on its parent.

**This file never runs the build's code path.** It works in double precision on lists.

Every case is a composition 16 pixels by 10 holding one drawing the same size, as D-190's. The
drawings go into `Fixtures/lightning_extras/media`, the projects into `Fixtures/lightning_extras`
and the expected frames into `Fixtures/lightning_extras/expected_lightning_extras.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/lightning_extras_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from noise_reference import mix, M as MASK  # noqa: E402
import lightning_bolt_reference as LB  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = LB.W, LB.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "lightning_extras"
TOLERANCE = LB.TOLERANCE
CLIFF = LB.CLIFF
KINDS = ("direction", "strike", "breaking", "bouncy", "omni", "anywhere", "vertical", "two_way")
RANGES = {**LB.RANGES, "turbulence": (0, 100), "decay": (0, 100), "conductivity": (0, 10000),
          "obstacle": (0, 100)}
EXTRA = ("turbulence", "decay", "conductivity", "obstacle")
EMPTY = LB.EMPTY


# --- the rule -------------------------------------------------------------------------------

def unit(base, x, y, f, ch):
    """Noise's hash from an already mixed base, as the build's `grade::unit`."""
    h = mix(mix(mix(mix(base ^ (x & MASK)) ^ (y & MASK)) ^ (f & MASK)) ^ ch)
    return (h >> 11) / 2 ** 53 * 2 - 1


def numbers(seed, c):
    """The bolt's numbers r(key, bolt, m, j) at conductivity c."""
    base = mix(seed & MASK)
    n = math.floor(c)
    t = c - n
    s = t * t * (3 - 2 * t)
    a = base if n == 0 else mix(base ^ n)
    b = mix(base ^ (n + 1))

    def r(key, bolt, m, j):
        v = unit(a, key, bolt, m, j)
        return v if s == 0 else v + s * (unit(b, key, bolt, m, j) - v)
    return r


def roots(kind, O, D, bottom, kd, r, m):
    """The bolts the halving starts from: (P, Q, wP, wQ, depth, bolt, key, parent)."""
    one = lambda P, Q, b: (P, Q, 1.0, kd, 0, b, 1, b)  # noqa: E731
    if kind == "bouncy":
        return [one(O, D, 0), one(D, O, -1), one(O, D, -2)]
    if kind == "omni":
        v = (D[0] - O[0], D[1] - O[1])
        return [one(O, (O[0] + t[0], O[1] + t[1]), -i)
                for i, t in enumerate(LB.turn(v, 60 * i) for i in range(6))]
    if kind == "anywhere":
        a = math.radians(180 * (r(1, -1, m, 0) + 1))
        d = math.hypot(D[0] - O[0], D[1] - O[1]) * (r(1, -1, m, 1) + 1) / 2
        return [one(O, (O[0] + d * math.cos(a), O[1] + d * math.sin(a)), 0)]
    if kind == "vertical":
        return [one(O, (O[0], bottom), 0)]
    if kind == "two_way":
        mid = ((O[0] + D[0]) / 2, (O[1] + D[1]) / 2)
        return [one(O, mid, 0), one(D, mid, -1)]
    return [one(O, D, 0)]


def bolt(kind, O, D, bottom, jagged, detail, branches, seed, m, turbulence=0, decay=0,
         conductivity=0, reached=None):
    """The segments before any obstacle, (P, Q, wP, wQ, depth, bolt, key, parent)."""
    r = numbers(seed, conductivity)
    segs = roots(kind, O, D, bottom, 1 - decay / 100, r, m)
    for i in range(detail):
        out = []
        for P, Q, wp, wq, depth, b, key, parent in segs:
            dx, dy = Q[0] - P[0], Q[1] - P[1]
            L = math.hypot(dx, dy)
            if L == 0:
                out.append((P, Q, wp, wq, depth, b, key, parent))
                continue
            rr = [r(key, b, m, j) for j in range(5)]
            a = rr[0] * jagged / 100 * L / 2 * (1 + turbulence / 100 * i / 2)
            Mid = ((P[0] + Q[0]) / 2 - dy / L * a, (P[1] + Q[1]) / 2 + dx / L * a)
            wm = (wp + wq) / 2
            out += [(P, Mid, wp, wm, depth, b, 2 * key, parent),
                    (Mid, Q, wm, wq, depth, b, 2 * key + 1, parent)]
            chance = branches / 100 * (1 + turbulence / 100)
            fork = depth < 3 and (rr[1] + 1) / 2 < chance
            if reached is not None and depth < 3:
                reached.append((rr[1], chance, fork, rr[3]))
            if fork:
                way = (dx / L, dy / L)
                if kind == "strike":
                    tx, ty = D[0] - Mid[0], D[1] - Mid[1]
                    tl = math.hypot(tx, ty)
                    if tl > 0:
                        way = (tx / tl, ty / tl)
                v = LB.turn(way, (15 + 15 * (rr[2] + 1)) * (-1 if rr[3] < 0 else 1))
                lb = L * (0.3 + 0.15 * (rr[4] + 1))
                out.append((Mid, (Mid[0] + v[0] * lb, Mid[1] + v[1] * lb),
                            wm if kind == "breaking" else wm / 2, 0.0, depth + 1, 1024 * b + key,
                            1, b))
        segs = out
    return segs


def stop(segs, blocked):
    """Each bolt ended at its first blocking point, and the forks beyond it gone."""
    kept, done, started, out = {}, set(), set(), []
    for depth in range(4):
        for P, Q, wp, wq, d, b, key, parent in segs:
            if d != depth or b in done:
                continue
            if b not in started:
                started.add(b)
                if d > 0 and P not in kept.get(parent, ()):
                    done.add(b)
                    continue
            n = max(1, math.ceil(math.hypot(Q[0] - P[0], Q[1] - P[1])))
            hit = next((j for j in range(n + 1) if blocked(
                P[0] + (Q[0] - P[0]) * (j / n), P[1] + (Q[1] - P[1]) * (j / n))), None)
            mine = kept.setdefault(b, set())
            if hit is None:
                out.append((P, Q, wp, wq, d, b, key, parent))
                mine.update((P, Q))
                continue
            done.add(b)
            if hit > 0:
                X = (P[0] + (Q[0] - P[0]) * (hit / n), P[1] + (Q[1] - P[1]) * (hit / n))
                out.append((P, X, wp, wp + hit / n * (wq - wp), d, b, key, parent))
                mine.update((P, X))
    return out


def blocker(pixels, obstacle):
    edge = 1 - obstacle / 100

    def blocked(x, y):
        i, j = math.floor(x), math.floor(y)
        return 0 <= i < W and 0 <= j < H and pixels[j * W + i][3] / 255 > edge
    return blocked


def segments(c, frame_no, reached=None):
    """The case's bolt at a frame, in the drawing's own space, any obstacle applied."""
    h = {k: held(c, k, frame_no) for k in LB.NUMBERS + EXTRA}
    O = (h["start"][0] / 100 * W, h["start"][1] / 100 * H)
    D = (h["end"][0] / 100 * W, h["end"][1] / 100 * H)
    segs = bolt(c["kind"], O, D, H, h["jagged"], h["detail"], h["branches"],
                h["seed"], frame_no // h["hold"], h["turbulence"], h["decay"], h["conductivity"],
                reached)
    if h["obstacle"] > 0:
        pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
        segs = stop(segs, blocker(pixels, h["obstacle"]))
    return segs


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    working = [R.working(p) for p in pixels]
    opacity, width, glow = (held(c, k, frame_no) for k in ("opacity", "width", "glow"))
    if opacity == 0 or (width == 0 and glow == 0):
        out = working
    else:
        C, G = LB.field([s[:7] for s in segments(c, frame_no)], width, glow, W, H)
        core = [srgb_to_linear(v / 255) for v in R.hex_color(c["color"].lower())]
        halo = [srgb_to_linear(v / 255) for v in R.hex_color(c["glow_color"].lower())]
        s = opacity / 100
        out = []
        for o, k, g in zip(working, C, G):
            lit = [k * core[ch] + (1 - k) * g * halo[ch] for ch in range(3)]
            out.append([o[ch] + s * lit[ch] for ch in range(3)] +
                       [o[3] + s * (k + (1 - k) * g) * (1 - o[3])])
    return out


# --- the drawings ---------------------------------------------------------------------------

GROUND = (90, 60, 30, 255)          # #5a3c1e, earth
MIST = (90, 60, 30, 128)            # the same earth, half covering
NONE = S.NONE

DRAWINGS = {
    "night": LB.DRAWINGS["night"],
    # Rows 7 to 9 the ground, rows 0 to 6 empty.
    "ground": [[GROUND if y >= 7 else NONE for x in range(W)] for y in range(H)],
    # The same, half covering.
    "mist": [[MIST if y >= 7 else NONE for x in range(W)] for y in range(H)],
}


# --- the cases ------------------------------------------------------------------------------

def case(drawing="night", kind="direction", turbulence=0, decay=0, conductivity=0, obstacle=0,
         **bolt_settings):
    c = LB.case(**bolt_settings)
    c.update(drawing=drawing, kind=kind, turbulence=turbulence, decay=decay,
             conductivity=conductivity, obstacle=obstacle)
    return c


def held(c, k, frame_no):
    if k in LB.RANGES:
        return LB.held(c, k, frame_no)
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


LINE = LB.LINE
SMALL = LB.SMALL
# Down the middle onto the ground, thin and glowless, so where it ends is plain.
DOWN = {"start": (50, 0), "end": (50, 100), "branches": 0, "width": 1, "glow": 0, "hold": 1}
# A small star's middle and radius: from the middle of the frame, 30 per cent of its height up.
STAR = {"start": (50, 50), "end": (50, 20), "branches": 0, "width": 1, "glow": 0, "jagged": 20}

CASES = {
    "FX-LIGHTX-001": ("FX-BOLT-001's settings with the five new ones written at their starting "
                      "values (type direction, turbulence 0, decay 0, conductivity 0, obstacle "
                      "0): exactly FX-BOLT-001.",
                      case(), [0, 1, 2]),
    "FX-LIGHTX-002": ("A straight line across row 4 from right to left, from (110, 45) to (-10, "
                      "45) per cent, width 1, no glow, with Alpha Obstacle 50 over the night "
                      "sky's left half: it stops where it meets the sky. Columns 8 to 15 of row 4 "
                      "lit, columns 0 to 6 untouched.",
                      case(**{**LINE, "start": (110, 45), "end": (-10, 45)}, obstacle=50), [0]),
    "FX-LIGHTX-003": ("FX-LIGHTX-002 with Alpha Obstacle 0: the whole row lit, as before.",
                      case(**{**LINE, "start": (110, 45), "end": (-10, 45)}), [0]),
    "FX-LIGHTX-004": ("A jagged bolt from the middle of the top edge to the middle of the "
                      "bottom, width 1, no glow, over a drawing whose rows 7 to 9 are ground, "
                      "with Alpha Obstacle 50: it ends where it reaches the ground, and row 9 is "
                      "untouched. Tutorial 2's bolt ending on the ground.",
                      case(drawing="ground", obstacle=50, **DOWN), [0, 1]),
    "FX-LIGHTX-005": ("FX-LIGHTX-004 with Alpha Obstacle 0: the bolt runs on through the ground "
                      "to the bottom edge.",
                      case(drawing="ground", **DOWN), [0, 1]),
    "FX-LIGHTX-006": ("FX-LIGHTX-004 with branches 100: the forks stop at the ground too.",
                      case(drawing="ground", obstacle=50, **{**DOWN, "branches": 100}), [0]),
    "FX-LIGHTX-007": ("FX-LIGHTX-004 over ground that half covers, Alpha Obstacle 40: half "
                      "covering is not enough to block at 40, so the bolt runs to the bottom, "
                      "as FX-LIGHTX-005 does on that drawing.",
                      case(drawing="mist", obstacle=40, **DOWN), [0]),
    "FX-LIGHTX-008": ("FX-LIGHTX-007 at Alpha Obstacle 60: now half covering blocks, and the "
                      "bolt ends at the ground.",
                      case(drawing="mist", obstacle=60, **DOWN), [0]),
    "FX-LIGHTX-009": ("FX-BOLT-010's bolt (branches 100) as Strike: the main bolt the same, its "
                      "forks turned from the way to the end point instead.",
                      case(kind="strike", **{**SMALL, "branches": 100}), [0]),
    "FX-LIGHTX-010": ("FX-BOLT-010's bolt as Breaking: the main bolt and the forks' paths the "
                      "same, each fork starting as bright and as wide as the bolt where it "
                      "leaves it.",
                      case(kind="breaking", **{**SMALL, "branches": 100}), [0]),
    "FX-LIGHTX-011": ("FX-BOLT-009's bolt as Bouncy: three bolts between the two points, there, "
                      "back and there again, each its own shape.",
                      case(kind="bouncy", **SMALL), [0]),
    "FX-LIGHTX-012": ("Omni from the middle, (50, 50), with the end point 3 pixels above it: six "
                      "bolts 3 pixels long, 60 degrees apart, the first straight up.",
                      case(kind="omni", **STAR), [0]),
    "FX-LIGHTX-013": ("Anywhere from the middle, the end point 3 pixels above it, hold 1: one "
                      "bolt to somewhere within 3 pixels, a new place each frame.",
                      case(kind="anywhere", **{**STAR, "hold": 1}), [0, 1, 2]),
    "FX-LIGHTX-014": ("Vertical from (25, 0), the end point at (90, 30): a bolt from column 4 "
                      "of the top edge straight down to the bottom edge; the end point is not "
                      "used.",
                      case(kind="vertical", **{**DOWN, "start": (25, 0), "end": (90, 30)}), [0]),
    "FX-LIGHTX-015": ("FX-LIGHTX-014 with the end point at (10, 80): the same.",
                      case(kind="vertical", **{**DOWN, "start": (25, 0), "end": (10, 80)}), [0]),
    "FX-LIGHTX-016": ("Two-Way Striking from the top edge's middle to the bottom's: two bolts, "
                      "one down from the top and one up from the bottom, meeting in the middle.",
                      case(kind="two_way", **SMALL), [0]),
    "FX-LIGHTX-017": ("FX-BOLT-001 with turbulence 100: a rougher bolt with more forks.",
                      case(turbulence=100), [0]),
    "FX-LIGHTX-018": ("FX-BOLT-006's line (rows 4 and 5, width 2, left to right) with decay "
                      "100: thick at the left, thinning to nothing at the right.",
                      case(decay=100, **{**LINE, "start": (-10, 50), "end": (110, 50),
                                         "width": 2}), [0]),
    "FX-LIGHTX-019": ("FX-BOLT-009's bolt held for 100 frames, its conductivity keyed from 0 at "
                      "frame 0 to 2 at frame 4: frame 0 is FX-BOLT-009's frame 0; the bolt "
                      "changes shape smoothly through frames 1 to 4 without being redrawn.",
                      case(conductivity=keyed((0, 0), (4, 2)), **{**SMALL, "hold": 100}),
                      [0, 1, 2, 4]),
}

INVALID = {
    "FX-LIGHTX-020": ("A lightning type \"sideways\".", case(kind="sideways")),
    "FX-LIGHTX-021": ("Turbulence 101, above 100.", case(turbulence=101)),
    "FX-LIGHTX-022": ("Decay -1, below 0.", case(decay=-1)),
    "FX-LIGHTX-023": ("Conductivity 10001, above 10000.", case(conductivity=10001)),
    "FX-LIGHTX-024": ("Alpha Obstacle 101, above 100.", case(obstacle=101)),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = LB.project_json(fx, c)
    params = p["compositions"][0]["layers"][0]["effects"][0]["parameters"]
    params["kind"] = c["kind"]
    for k in EXTRA:
        params[k] = setting_json(c[k])
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def plain(c):
    return render(case(drawing=c["drawing"], opacity=0), 0)


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_lightning_extras.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                        encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    lit = lambda px, d, i: px[i] != d[i]  # noqa: E731

    # The absent values are D-190's bolt, number for number, in every one of its cases.
    for fx, (_, lc, frames) in LB.CASES.items():
        for f in frames:
            mine = case(**{k: lc[k] for k in LB.NAMES}, shift=lc["shift"])
            if lc["shift"] == 0:
                assert render(mine, f) == LB.render(lc, f), fx
            assert [s[:7] for s in segments(mine, f)] == LB.segments(lc, f), fx
    assert c["FX-LIGHTX-001"] == {str(f): LB.render(LB.case(), f) for f in (0, 1, 2)}
    # No reached segment sits near a fork's cliff, nor a fork near its side's.
    for fx, (_, cs, frames) in CASES.items():
        for f in frames:
            reached = []
            segments(cs, f, reached)
            for r1, chance, fork, r3 in reached:
                assert chance <= 0 or chance >= 1 or abs((r1 + 1) / 2 - chance) > CLIFF, fx
                assert not fork or abs(r3) > CLIFF, fx

    night, ground, mist = (plain(case(drawing=d)) for d in ("night", "ground", "mist"))
    # 002: stops at the sky; 003: the whole row.
    two, three = c["FX-LIGHTX-002"]["0"], c["FX-LIGHTX-003"]["0"]
    assert all(lit(two, night, at(x, 4)) for x in range(8, W))
    assert all(two[at(x, y)] == night[at(x, y)] for x in range(7) for y in range(H))
    assert all(lit(three, night, at(x, 4)) for x in range(W))
    # 004 to 006: ends at the ground, row 9 untouched, row 7 reached; 005 runs on to the bottom.
    for fx in ("FX-LIGHTX-004", "FX-LIGHTX-006"):
        for px in c[fx].values():
            assert all(px[at(x, 9)] == ground[at(x, 9)] for x in range(W)), fx
            assert any(lit(px, ground, at(x, 7)) for x in range(W)), fx
    for px in c["FX-LIGHTX-005"].values():
        assert any(lit(px, ground, at(x, 9)) for x in range(W))
    assert c["FX-LIGHTX-004"]["0"] != c["FX-LIGHTX-004"]["1"]
    ends = [s for s in segments(CASES["FX-LIGHTX-006"][1], 0)]
    assert max(s[1][1] for s in ends) < 8 and any(s[4] > 0 for s in ends)
    # 007: half covering, 40, not blocked, the bolt as with no obstacle; 008 at 60 blocked.
    free = render(case(drawing="mist", **DOWN), 0)
    assert c["FX-LIGHTX-007"]["0"] == free
    assert all(c["FX-LIGHTX-008"]["0"][at(x, 9)] == mist[at(x, 9)] for x in range(W))
    assert free != c["FX-LIGHTX-008"]["0"]
    # 009 and 010: the main bolt as FX-BOLT-010's; only the forks differ.
    forked = LB.segments(LB.case(**{**SMALL, "branches": 100}), 0)
    for fx in ("FX-LIGHTX-009", "FX-LIGHTX-010"):
        mine = segments(CASES[fx][1], 0)
        assert [s[:7] for s in mine if s[5] == 0] == [s for s in forked if s[5] == 0], fx
        assert [s[:7] for s in mine] != forked and len(mine) == len(forked), fx
    breaking = segments(CASES["FX-LIGHTX-010"][1], 0)
    assert [(s[0], s[1]) for s in breaking] == [(s[0], s[1]) for s in forked]
    # 011: three bolts; 012: six spokes 3 pixels long; 013: a new place each frame.
    assert len({s[5] for s in segments(CASES["FX-LIGHTX-011"][1], 0)}) == 3
    star = segments(CASES["FX-LIGHTX-012"][1], 0)
    tips = [s[1] for s in star if s[6] == 2 ** 7 - 1]
    assert len(tips) == 6
    assert all(abs(math.hypot(x - 8, y - 5) - 3) < 1e-9 for x, y in tips)
    assert min(tips, key=lambda p: p[1]) == tips[0] and abs(tips[0][0] - 8) < 1e-9
    any_ = c["FX-LIGHTX-013"]
    assert any_["0"] != any_["1"] != any_["2"]
    # 014 and 015: the end point unused; straight down column 4 to the bottom.
    assert c["FX-LIGHTX-014"] == c["FX-LIGHTX-015"]
    main = [s for s in segments(CASES["FX-LIGHTX-014"][1], 0) if s[5] == 0]
    assert main[0][0] == (4.0, 0.0) and main[-1][1] == (4.0, 10.0)
    # 016: two bolts meeting at the middle.
    two_way = segments(CASES["FX-LIGHTX-016"][1], 0)
    assert {s[1] for s in two_way if s[6] == 2 ** 7 - 1} == {(8.0, 5.0)}
    # 017: more forks than FX-BOLT-001.
    assert len(segments(CASES["FX-LIGHTX-017"][1], 0)) > len(LB.segments(LB.case(), 0))
    # 018: thinning, the covering along row 4 never growing left to right, the right end bare.
    row = [c["FX-LIGHTX-018"]["0"][at(x, 4)][3] for x in range(8, W)]
    assert all(a >= b for a, b in zip(row, row[1:])) and row[0] > row[-1]
    # 019: frame 0 is FX-BOLT-009's, and the path moves smoothly.
    assert c["FX-LIGHTX-019"]["0"] == LB.render(LB.case(**SMALL), 0)
    k = CASES["FX-LIGHTX-019"][1]
    a = segments(case(**{**SMALL, "hold": 100}, conductivity=1), 0)
    b = segments(case(**{**SMALL, "hold": 100}, conductivity=1.001), 0)
    assert max(abs(p[0][0] - q[0][0]) + abs(p[0][1] - q[0][1]) for p, q in zip(a, b)) < 0.01
    assert segments(k, 2) == a and segments(k, 4) != segments(k, 0)
    print("checked")


if __name__ == "__main__":
    main()
