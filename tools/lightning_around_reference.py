"""Lightning Bolt going round shapes, and a negative Alpha Obstacle, worked a second way.

D-329 (from P-26's tutorial 2) adds to D-324's Advanced Lightning (`tools/lightning_extras_reference.py`)
a word `path`, what a bolt does at an obstacle, and lets Alpha Obstacle go below 0. Nothing is
ported and no After Effects frame was compared: After Effects' manual says only that its bolt
"goes around" shapes and that a negative Alpha Obstacle keeps it inside them. This is this
program's own rule. Document 21 is the rule in words; this file is the reference for the numbers
document 25 pins against it.

- Alpha Obstacle a in -100..100. A pixel of the layer, as it is before the bolt (D-324's), blocks
  when a > 0 and its covering is above 1 - a / 100 (D-324's rule), or when a < 0 and its covering
  is below -a / 100: the mirror, so a negative obstacle keeps the bolt inside what is solid.
  Outside the layer nothing blocks when a > 0 and everything blocks when a < 0. At 0 nothing
  blocks.
- `path` "split" (absent "split"): D-324's bolt, stopped at its first blocking point.
- `path` "around" with a blocking obstacle: before the halving, each of the type's bolts, O to D,
  is routed. The cells are the layer's pixels, a cell free when its pixel does not block. From
  O's cell (each coordinate floored and held inside the layer) a breadth-first search visits the
  free cells, each new cell's neighbours in the order right, left, down, up, down-right,
  up-right, down-left, up-left, a diagonal step only when both cells beside it are free, a cell
  counted on its first visit, until D's cell (held the same way) is visited. The best cell is D's
  when it is visited, otherwise the first visited cell nearest D's in whole-cell squared
  distance. When O's cell blocks, that bolt is not drawn. The way is the parents' chain from O's
  cell to the best cell; its points are O, the middles of the cells between, and D (or the best
  cell's middle). From each point it keeps the furthest later point it can see in a straight
  line, a line clear when none of D-324's walk points along it blocks. Piece p of the result is a
  bolt of its own, (P, Q, w(s_p), w(s_p+1), 0, b_p, 1), s the length along the way, w(s) =
  1 + (k_d - 1) s / total (exactly k_d at the end), b_0 the type's bolt number b and
  b_p = b - 8 p after it. The halving is D-324's, except that a main bolt's (depth 0) pushed
  middle is used only when both halves are clear, and is otherwise the plain middle. Main bolts
  are then never stopped; forks are stopped as D-324's.
- `path` "around" with Alpha Obstacle 0 is the split bolt exactly.

**This file never runs the build's code path.** It works in double precision on lists.

The drawings go into `Fixtures/lightning_around/media`, the projects into
`Fixtures/lightning_around` and the expected frames into
`Fixtures/lightning_around/expected_lightning_around.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/lightning_around_reference.py
"""

import json
import math
import sys
from collections import deque
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
import lightning_bolt_reference as LB  # noqa: E402
import lightning_extras_reference as LX  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = LX.W, LX.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "lightning_around"
TOLERANCE = LX.TOLERANCE
STEPS = ((1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1))


# --- the rule -------------------------------------------------------------------------------

def blocker(pixels, obstacle):
    """D-324's blocking pixel for a > 0, its mirror for a < 0, outside the layer as a < 0."""
    a = obstacle / 100

    def blocked(x, y):
        i, j = math.floor(x), math.floor(y)
        if not (0 <= i < W and 0 <= j < H):
            return a < 0
        alpha = pixels[j * W + i][3] / 255
        return alpha > 1 - a if a > 0 else alpha < -a
    return blocked


def walk(P, Q):
    n = max(1, math.ceil(math.hypot(Q[0] - P[0], Q[1] - P[1])))
    return n, [(P[0] + (Q[0] - P[0]) * (j / n), P[1] + (Q[1] - P[1]) * (j / n))
               for j in range(n + 1)]


def clear(P, Q, blocked):
    return not any(blocked(x, y) for x, y in walk(P, Q)[1])


def route(O, D, blocked):
    """The points the bolt from O towards D goes through, or None when O's cell blocks."""
    cell = lambda p: (min(W - 1, max(0, math.floor(p[0]))),  # noqa: E731
                      min(H - 1, max(0, math.floor(p[1]))))
    free = lambda i, j: 0 <= i < W and 0 <= j < H and not blocked(i + 0.5, j + 0.5)  # noqa: E731
    s, t = cell(O), cell(D)
    if not free(*s):
        return None
    far = lambda c: (c[0] - t[0]) ** 2 + (c[1] - t[1]) ** 2  # noqa: E731
    parent, queue, best = {s: None}, deque([s]), s
    while queue and t not in parent:
        ci, cj = queue.popleft()
        for di, dj in STEPS:
            n = (ci + di, cj + dj)
            if n in parent or not free(*n):
                continue
            if di and dj and not (free(ci + di, cj) and free(ci, cj + dj)):
                continue
            parent[n] = (ci, cj)
            queue.append(n)
            if far(n) < far(best):
                best = n
            if n == t:
                break
    way = []
    c = best
    while c is not None:
        way.append(c)
        c = parent[c]
    way.reverse()
    pts = [O] + [(i + 0.5, j + 0.5) for i, j in way[1:-1]]
    pts.append(D if best == t else (best[0] + 0.5, best[1] + 0.5))
    out, i = [pts[0]], 0
    while i < len(pts) - 1:
        j = i + 1
        while j + 1 < len(pts) and clear(pts[i], pts[j + 1], blocked):
            j += 1
        out.append(pts[j])
        i = j
    return out


def pieces(root, blocked):
    """One root bolt routed round the obstacles: its pieces, each a bolt of its own."""
    O, D, _, kd, _, b, _, _ = root
    pts = route(O, D, blocked)
    if pts is None:
        return []
    lengths = [math.hypot(q[0] - p[0], q[1] - p[1]) for p, q in zip(pts, pts[1:])]
    total = sum(lengths)
    s, out = 0.0, []
    for p, (P, Q) in enumerate(zip(pts, pts[1:])):
        ws = 1.0 if p == 0 else 1 + (kd - 1) * s / total
        s += lengths[p]
        we = kd if p == len(lengths) - 1 else 1 + (kd - 1) * s / total
        bp = b if p == 0 else b - 8 * p
        out.append((P, Q, ws, we, 0, bp, 1, bp))
    return out


def bolt_around(kind, O, D, bottom, jagged, detail, branches, seed, m, turbulence, decay,
                conductivity, blocked, reached=None):
    """LX.bolt with each root routed and the main bolts' pushes kept clear."""
    r = LX.numbers(seed, conductivity)
    segs = [s for root in LX.roots(kind, O, D, bottom, 1 - decay / 100, r, m)
            for s in pieces(root, blocked)]
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
            if depth == 0 and not (clear(P, Mid, blocked) and clear(Mid, Q, blocked)):
                Mid = ((P[0] + Q[0]) / 2, (P[1] + Q[1]) / 2)
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


def stop(segs, blocked, keep_main):
    """LX.stop, the main bolts (depth 0) left whole when keep_main."""
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
            n, pts = walk(P, Q)
            hit = None if keep_main and d == 0 else next(
                (j for j, (x, y) in enumerate(pts) if blocked(x, y)), None)
            mine = kept.setdefault(b, set())
            if hit is None:
                out.append((P, Q, wp, wq, d, b, key, parent))
                mine.update((P, Q))
                continue
            done.add(b)
            if hit > 0:
                X = pts[hit]
                out.append((P, X, wp, wp + hit / n * (wq - wp), d, b, key, parent))
                mine.update((P, X))
    return out


def held(c, k, frame_no):
    if k == "obstacle":
        return min(100, max(-100, LX.value_at(c[k], frame_no)))
    return LX.held(c, k, frame_no)


def segments(c, frame_no, reached=None):
    """The case's bolt at a frame, in the drawing's own space, any obstacle applied."""
    h = {k: held(c, k, frame_no) for k in LB.NUMBERS + LX.EXTRA}
    O = (h["start"][0] / 100 * W, h["start"][1] / 100 * H)
    D = (h["end"][0] / 100 * W, h["end"][1] / 100 * H)
    args = (c["kind"], O, D, H, h["jagged"], h["detail"], h["branches"], h["seed"],
            frame_no // h["hold"], h["turbulence"], h["decay"], h["conductivity"])
    if h["obstacle"] == 0:
        return LX.bolt(*args, reached=reached)
    blocked = blocker([p for row in DRAWINGS[c["drawing"]] for p in row], h["obstacle"])
    if c["path"] == "around":
        return stop(bolt_around(*args, blocked, reached), blocked, True)
    return stop(LX.bolt(*args, reached=reached), blocked, False)


def render(c, frame_no):
    """LX.render with this file's segments."""
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    working = [R.working(p) for p in pixels]
    opacity, width, glow = (held(c, k, frame_no) for k in ("opacity", "width", "glow"))
    if opacity == 0 or (width == 0 and glow == 0):
        return working
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

ROCK = LX.GROUND
NONE = S.NONE


def u_shape(x, y):
    arm = (2 <= x <= 4 or 11 <= x <= 13) and 2 <= y <= 8
    return arm or (2 <= x <= 13 and 7 <= y <= 8)


DRAWINGS = {
    "night": LX.DRAWINGS["night"],
    "ground": LX.DRAWINGS["ground"],
    "mist": LX.DRAWINGS["mist"],
    # A block of rock across columns 5 to 10, rows 3 to 6, in an empty sky.
    "block": [[ROCK if 5 <= x <= 10 and 3 <= y <= 6 else NONE for x in range(W)]
              for y in range(H)],
    # A U of rock: arms down columns 2 to 4 and 11 to 13 from row 2, joined along rows 7 and 8.
    "u": [[ROCK if u_shape(x, y) else NONE for x in range(W)] for y in range(H)],
}


# --- the cases ------------------------------------------------------------------------------

def case(path="split", **settings):
    c = LX.case(**settings)
    c["path"] = path
    return c


DOWN = LX.DOWN
U = {"start": (20, 25), "end": (75, 25), "branches": 0, "width": 1, "glow": 0, "hold": 1}
ROW = {"jagged": 30, "branches": 0, "width": 1, "glow": 0, "hold": 1}

CASES = {
    "FX-LIGHTA-001": ("FX-LIGHTX-004 with the new word written at its starting value (path "
                      "split): exactly FX-LIGHTX-004.",
                      case(drawing="ground", obstacle=50, **DOWN), [0, 1]),
    "FX-LIGHTA-002": ("FX-LIGHTX-005 (no obstacle) with path around: exactly the same, a bolt "
                      "with nothing to go round is not changed.",
                      case("around", drawing="ground", **DOWN), [0, 1]),
    "FX-LIGHTA-003": ("The jagged bolt from the top edge's middle to the bottom's, width 1, no "
                      "glow, over a block of rock across columns 5 to 10, rows 3 to 6, Alpha "
                      "Obstacle 50, path split: it stops at the block's top, rows 7 to 9 "
                      "untouched.",
                      case(drawing="block", obstacle=50, **DOWN), [0]),
    "FX-LIGHTA-004": ("FX-LIGHTA-003 with path around: the bolt goes round the block and reaches "
                      "the bottom edge; the block's middle is untouched.",
                      case("around", drawing="block", obstacle=50, **DOWN), [0, 1]),
    "FX-LIGHTA-005": ("FX-LIGHTA-004 with branches 100: the main bolt goes round, its forks stop "
                      "at the block.",
                      case("around", drawing="block", obstacle=50, **{**DOWN, "branches": 100}),
                      [0]),
    "FX-LIGHTA-006": ("FX-LIGHTA-004 with the end point inside the block, (50, 50) per cent: "
                      "there is no way in, so the bolt goes round and ends at the free pixel nearest "
                      "it, just under the block.",
                      case("around", drawing="block", obstacle=50, **{**DOWN, "end": (50, 50)}),
                      [0]),
    "FX-LIGHTA-007": ("FX-LIGHTX-004's ground with path around: no way round the ground, so the "
                      "bolt ends at the free pixel nearest the end point, and row 9 is "
                      "untouched.",
                      case("around", drawing="ground", obstacle=50, **DOWN), [0]),
    "FX-LIGHTA-008": ("A straight line from the block's middle to the right, from (50, 45) to "
                      "(110, 45) per cent, width 1, no glow, Alpha Obstacle -50, path split: it "
                      "stays inside the block and stops at its right edge, columns 11 to 15 "
                      "untouched.",
                      case(drawing="block", obstacle=-50, **{**LX.LINE, "start": (50, 45),
                                                             "end": (110, 45)}), [0]),
    "FX-LIGHTA-009": ("A jagged bolt inside a U of rock, from the left arm, (20, 25) per cent, to "
                      "the right arm, (75, 25), width 1, no glow, Alpha Obstacle -50, path "
                      "split: it stops where it leaves the left arm.",
                      case(drawing="u", obstacle=-50, **U), [0]),
    "FX-LIGHTA-010": ("FX-LIGHTA-009 with path around: the bolt runs down the left arm, along "
                      "the bottom and up the right arm to the end point; the empty sky between "
                      "the arms, columns 6 to 9 above row 6, is untouched.",
                      case("around", drawing="u", obstacle=-50, **U), [0, 1]),
    "FX-LIGHTA-011": ("A jagged bolt along row 8 of half-covering ground, from (5, 85) to (95, "
                      "85) per cent, Alpha Obstacle -40, path around: half covering is enough to "
                      "keep it, and the bolt stays inside rows 7 to 9.",
                      case("around", drawing="mist", obstacle=-40, **{**ROW, "start": (5, 85),
                                                                       "end": (95, 85)}), [0]),
    "FX-LIGHTA-012": ("FX-LIGHTA-011 at Alpha Obstacle -60: half covering is now too thin, the "
                      "start blocks, and no bolt is drawn.",
                      case("around", drawing="mist", obstacle=-60, **{**ROW, "start": (5, 85),
                                                                       "end": (95, 85)}), [0]),
}

INVALID = {
    "FX-LIGHTA-013": ("A path \"sideways\".", case("sideways")),
    "FX-LIGHTA-014": ("Alpha Obstacle -101, below -100.", case(obstacle=-101)),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = LX.project_json(fx, c)
    p["compositions"][0]["layers"][0]["effects"][0]["parameters"]["path"] = c["path"]
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

    (OUT / "expected_lightning_around.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                        encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    lit = lambda px, d, i: px[i] != d[i]  # noqa: E731

    # Split, and around with no obstacle, are D-324's bolt exactly.
    for fx, (_, lc, frames) in LX.CASES.items():
        for f in frames:
            for path in ("split", "around"):
                mine = {**lc, "path": path}
                if lc["obstacle"] == 0 or path == "split":
                    assert segments(mine, f) == LX.segments(lc, f), fx
    lx4 = {str(f): LX.render(LX.CASES["FX-LIGHTX-004"][1], f) for f in (0, 1)}
    lx5 = {str(f): LX.render(LX.CASES["FX-LIGHTX-005"][1], f) for f in (0, 1)}
    assert c["FX-LIGHTA-001"] == lx4 and c["FX-LIGHTA-002"] == lx5
    # No reached segment sits near a fork's cliff, nor a fork near its side's.
    for fx, (_, cs, frames) in CASES.items():
        for f in frames:
            reached = []
            segments(cs, f, reached)
            for r1, chance, fork, r3 in reached:
                assert chance <= 0 or chance >= 1 or abs((r1 + 1) / 2 - chance) > LX.CLIFF, fx
                assert not fork or abs(r3) > LX.CLIFF, fx
    # Around never puts a point of a main bolt in a blocking pixel (forks stop as D-324).
    for fx, (_, cs, frames) in CASES.items():
        if cs["path"] != "around" or cs["obstacle"] == 0:
            continue
        blocked = blocker([p for row in DRAWINGS[cs["drawing"]] for p in row], cs["obstacle"])
        for f in frames:
            for s in segments(cs, f):
                assert s[4] > 0 or clear(s[0], s[1], blocked), fx

    block, u, mist, ground = (plain(case(drawing=d)) for d in ("block", "u", "mist", "ground"))
    # 003 stops at the block's top; 004 goes round, reaching the bottom edge, the middle bare.
    three = c["FX-LIGHTA-003"]["0"]
    assert all(three[at(x, y)] == block[at(x, y)] for x in range(W) for y in range(7, H))
    assert max(s[1][1] for s in segments(CASES["FX-LIGHTA-003"][1], 0)) < 4
    for px in c["FX-LIGHTA-004"].values():
        assert any(lit(px, block, at(x, 9)) for x in range(W))
        assert all(px[at(x, y)] == block[at(x, y)] for x in (7, 8) for y in (4, 5))
    main = [s for s in segments(CASES["FX-LIGHTA-004"][1], 0)]
    assert main[-1][1] == (8.0, 10.0) and {s[5] for s in main} != {0}
    assert c["FX-LIGHTA-004"]["0"] != c["FX-LIGHTA-004"]["1"]
    # 005: forks there, and stopped.
    five = segments(CASES["FX-LIGHTA-005"][1], 0)
    assert any(s[4] > 0 for s in five)
    assert [s[:7] for s in five if s[4] == 0] == [s[:7] for s in main]
    # 006 and 007: no way in, ending beside the obstacle.
    end6 = [s for s in segments(CASES["FX-LIGHTA-006"][1], 0) if s[4] == 0][-1][1]
    assert end6 == (8.5, 7.5), end6
    seven = c["FX-LIGHTA-007"]["0"]
    assert all(seven[at(x, 9)] == ground[at(x, 9)] for x in range(W))
    assert [s for s in segments(CASES["FX-LIGHTA-007"][1], 0)][-1][1] == (8.5, 6.5)
    # 008: inside the block, stopping at its right edge.
    eight = c["FX-LIGHTA-008"]["0"]
    assert all(lit(eight, block, at(x, 4)) for x in range(8, 11))
    assert all(eight[at(x, y)] == block[at(x, y)] for x in range(12, W) for y in range(H))
    # 009 stops leaving the left arm; 010 reaches the right arm, the gap between untouched.
    nine = c["FX-LIGHTA-009"]["0"]
    assert all(nine[at(x, y)] == u[at(x, y)] for x in range(6, W) for y in range(H))
    for px in c["FX-LIGHTA-010"].values():
        assert all(px[at(x, y)] == u[at(x, y)] for x in range(6, 10) for y in range(6))
        assert any(lit(px, u, at(x, y)) for x in (11, 12, 13) for y in range(2, 5))
    ten = segments(CASES["FX-LIGHTA-010"][1], 0)
    assert ten[-1][1] == (12.0, 2.5)
    # 011 stays in rows 7 to 9; 012 draws nothing.
    eleven = c["FX-LIGHTA-011"]["0"]
    assert all(eleven[at(x, y)] == mist[at(x, y)] for x in range(W) for y in range(6))
    assert sum(lit(eleven, mist, at(x, 8)) for x in range(W)) >= 12
    assert c["FX-LIGHTA-012"]["0"] == mist
    print("checked")


if __name__ == "__main__":
    main()
