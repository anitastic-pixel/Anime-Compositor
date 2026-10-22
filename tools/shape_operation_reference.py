"""D-738 (d), B-613: path operations.

Writes `Fixtures/shape_operations/` and prints nothing but "checked" (or, with `--table`, document
25's table). Written before any code.

A shape group may carry `operations`, a list of at most 8, each `{"type", ...}` with its numbers
as property records keyed as any property is, with no expression, and its choice a word:
- `round_corners`: `radius` 0 to 100000;
- `pucker_bloat`: `amount` -100 to 100 per cent;
- `twist`: `angle` degrees, `center` two numbers;
- `zig_zag`: `size` 0 to 10000, `ridges` 0 to 100 (taken whole, rounded down), `points` "corner"
  or "smooth";
- `wiggle`: `size` 0 to 10000, `detail` 0 to 100 (whole), `wiggles_per_second` 0 to 1000,
  `correlation` 0 to 100 per cent, `seed` 0 to 100000 (whole);
- `offset`: `amount` -10000 to 10000, `join` "miter", "round" or "bevel", `miter_limit` 1 to 100.
They change every path drawn in the group, the paths of the groups inside it included, in the
group's own space, one after another in the list's order, before the group's repeater and its
transform place them; a group inside another is changed by its own operations, placed, then
changed by the outer group's. A path's points are written as a path's are: a point and its in and
out handles, the handles relative to the point.

Round Corners (lottie-web's, MIT, which writes After Effects' down): a point whose two handles are
zero, other than an open path's two ends, becomes two points, one toward the point before it and
one toward the point after, each min(radius, half that side) from it; the first's out handle and
the second's in handle reach toward the corner 0.5519 of the way. A radius of 0 changes nothing.

Pucker & Bloat (lottie-web's): with c the mean of the path's points and a = amount / 100, a point
p goes to p + (c - p) a and each handle's end e (the point plus the handle) to e + (c - e)(-a).

Twist: each piece whose two handles are both zero first gets handles a third of the way along it.
With R the greatest distance from the center to a point, every point and every handle's end at a
distance d from the center is turned about it clockwise by angle max(0, 1 - d / R). An angle of 0
changes nothing.

Zig Zag and Wiggle Paths first give straight pieces handles as Twist does, so that equal steps
of t are equal steps along them. Zig Zag: each piece is split at t = k / (ridges + 1), k = 1 to ridges, giving, in order, each
point of the path followed by its piece's split points. Point j (from 0) moves by size (-1)^j
along the path's tangent there turned 90 degrees clockwise. At a split point the tangent is the
piece's derivative (the chord where that is zero); at a point of the path it is the sum of the
unit directions in and out of it, the out direction the first not zero of b - a, c - a, d - a for
the piece a, b, c, d leaving it and the in direction of d - c, d - b, d - a for the piece
arriving, an open path's end taking the one it has. Corner points have no handles; smooth points
have handles along the tangent, half the straight distance to the point before and after (before
either moved).

Wiggle Paths: each piece is split exactly into detail + 1 pieces of equal t, each new piece's
handles a third of the derivative times its share of t. Point j moves by size (c N(S) + (1 - c)
N(j)) in x and in y, its handles with it, c = correlation / 100, S = 2^32, N the value noise at
tau = frame / fps x wiggles per second: k = floor(tau), f = tau - k, u = f f (3 - 2 f),
N = r(k) (1 - u) + r(k + 1) u, r = splitmix64(seed ^ splitmix64(j ^ splitmix64(axis ^
splitmix64(k)))), taking its top 53 bits to [-1, 1); axis 0 for x and 1 for y, k as two's
complement. A size of 0 changes nothing.

Offset Paths: the path is flattened as a shape's outline is (D-77), each point within 1e-9 of the
one kept before it dropped, a closed path's last too if it is on its first. With s the sign of
the outline's shoelace sum (1 for 0), each edge's normal is s (dy, -dx) / length, outward for a
closed path. Each point of the outline gives, with p1 and p2 the point moved by amount along the
normals of the edges into and out of it: where the edges turn away from the offset (cross
product times s times amount above 0) a join: miter, the meeting of the two moved edges while it
is no further than miter limit times |amount| from the point, else bevel; bevel, p1 and p2;
round, p1, the arc about the point every 10 degrees or less, then p2. Where they turn toward it,
the meeting of the two moved edges. Edges in a straight line give p1. An open path's ends move
along their one edge. The result is corners only, open or closed as the path was. An amount of 0
changes nothing.

An operation on a path of more than 100000 points leaves it as it is, and Zig Zag and Wiggle
Paths split each piece into no more than max(1, 100000 // pieces) pieces.

`host.json` is a 600 by 400 composition at 24 fps holding one shape layer of eleven groups and
thirteen shapes; `expected_shape_operations.json` gives, for each frame a case names, every shape
drawn in drawing order, each as `{"shape", "copies", "points", "fill_opacity"?,
"stroke_opacity"?, "stroke_width"?}`, numbers within 1e-9; the saved project's groups; and the
refused files.

**This file never runs the build's code path.** Fixtures are read-only to implementation work.
"""
import copy
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import shape_group_reference as sg  # noqa: E402
import shape_repeater_reference as sr  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "shape_operations"
par, prop, at, mul, apply = sg.par, sg.prop, sg.at, sg.mul, sg.apply
STILL = [1, 0, 0, 1, 0, 0]
FPS = 24
LIMIT = 100000
ROUND = 0.5519
M64 = (1 << 64) - 1


def op(kind, **numbers):
    o = {"type": kind}
    for k, v in numbers.items():
        o[k] = v if isinstance(v, str) else prop(v) if not isinstance(v, dict) else v
    return o


def group(name, x, y, operations, **more):
    g = {"name": name, "transform": sg.transform(position=prop([x, y])), "operations": operations}
    g.update(more)
    return g


GROUPS = [
    group("Corners", 60, 70, [op("round_corners", radius=prop(0, [(0, 0), (24, 30)]))]),
    group("Puff", 170, 70, [op("pucker_bloat", amount=prop(0, [(0, -60), (24, 60)]))],
          repeater=sr.repeater(copies=prop(2), position=prop([90, 0]))),
    group("Swirl", 380, 70, [op("round_corners", radius=6), op("twist", angle=120, center=[0, 0])]),
    group("Zig", 510, 70, [op("zig_zag", size=6, ridges=4, points="corner")]),
    group("Wobble", 60, 200, [op("wiggle", size=6, detail=2, wiggles_per_second=2, correlation=25, seed=7)]),
    group("Grow", 180, 200, [op("offset", amount=8, join="round", miter_limit=4)]),
    dict(group("Spike", 310, 200, [dict(op("offset", amount=6, join="miter", miter_limit=2), note="kept")]),
         blend="multiply"),
    group("Cut", 440, 200, [op("offset", amount=prop(0, [(0, -10), (24, 10)]), join="bevel", miter_limit=4)]),
    group("Waves", 540, 200, [op("zig_zag", size=8, ridges=3, points="smooth")]),
    group("Outer", 300, 320, [op("twist", angle=60, center=[0, 0])]),
    {"name": "Inner", "parent": 9, "transform": sg.transform(scale=prop([150, 150])),
     "operations": [op("round_corners", radius=10)]},
]

WHITE = {"color": [1, 1, 1], "opacity": 1, "width_px": 2}


def fill(r, g, b):
    return {"color": [r, g, b], "opacity": 1}


def pentagon(r):
    return [(r * math.cos(-math.pi / 2 + k * 2 * math.pi / 5), r * math.sin(-math.pi / 2 + k * 2 * math.pi / 5))
            for k in range(5)]


def star(outer, inner):
    return [((outer if k % 2 == 0 else inner) * math.cos(-math.pi / 2 + k * math.pi / 5),
             (outer if k % 2 == 0 else inner) * math.sin(-math.pi / 2 + k * math.pi / 5)) for k in range(10)]


def path(name, points, group=None, closed=True, fill=None, stroke=None):
    s = {"name": name, "enabled": True, "closed": closed,
         "path": {"base": {"points": points}, "keyframes": []}, "fill": fill, "stroke": stroke}
    if group is not None:
        s["group"] = group
    return s


def corners(cs):
    return [par.pt(x, y) for x, y in cs]


def parametric(name, p, group, fill=None, stroke=None):
    s = par.shape(name, p, fill=fill, stroke=stroke)
    s["group"] = group
    return s


SHAPES = [
    path("Box", [par.pt(40, -40), par.pt(40, 40), par.pt(-40, 40, (15, 0), (0, -15)), par.pt(-40, -40)], 0,
         fill=fill(0.2, 0.5, 1), stroke=WHITE),
    path("Penta", corners(pentagon(40)), 1, fill=fill(1, 0.6, 0.1)),
    path("Bar", corners([(50, -12), (50, 12), (-50, 12), (-50, -12)]), 2, fill=fill(0.9, 0.2, 0.5)),
    path("Line", corners([(-40, 0), (40, 0)]), 3, closed=False, stroke=WHITE),
    path("Card", corners([(40, -30), (40, 30), (-40, 30), (-40, -30)]), 4, fill=fill(0.3, 0.8, 0.4)),
    path("Tri", corners([(-35, 25), (35, 25), (0, -35)]), 5, fill=fill(0.8, 0.8, 0.2), stroke=WHITE),
    path("Star", corners(star(40, 16)), 6, fill=fill(1, 0.3, 0.2)),
    path("Hook", corners([(-30, 40), (0, 50), (30, 40)]), 6, closed=False, stroke=WHITE),
    parametric("Oval", {"type": "ellipse", "size": prop([70, 50]), "position": prop([0, 0])}, 7,
               fill=fill(0.5, 0.3, 1)),
    parametric("Disc", {"type": "ellipse", "size": prop([60, 60]), "position": prop([0, 0])}, 8,
               fill=fill(0.2, 0.9, 0.9)),
    path("Tile", corners([(30, -12), (30, 12), (-30, 12), (-30, -12)]), 10, fill=fill(1, 0.8, 0.3), stroke=WHITE),
    path("Plain", corners([(50, 350), (110, 350), (80, 290)]), fill=fill(0.6, 0.6, 0.6)),
]


def project(shapes, groups):
    p = sg.project(shapes, groups)
    p["project_id"] = "proj-shape-operations"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = 600, 400
    return p


HOST = project(SHAPES, GROUPS)


# --- the six operations, on paths as lists of [point, in, out], handles relative ---


def add(a, b):
    return [a[0] + b[0], a[1] + b[1]]


def sub(a, b):
    return [a[0] - b[0], a[1] - b[1]]


def unit(v):
    n = math.hypot(v[0], v[1])
    return [v[0] / n, v[1] / n] if n else [0, 0]


def piece(pts, i):
    """The cubic a, b, c, d from point i to the next."""
    p0, p1 = pts[i], pts[(i + 1) % len(pts)]
    return p0[0], add(p0[0], p0[2]), add(p1[0], p1[1]), p1[0]


def bez(q, t):
    a, b, c, d = q
    u = 1 - t
    w = (u * u * u, 3 * u * u * t, 3 * u * t * t, t * t * t)
    return [w[0] * a[k] + w[1] * b[k] + w[2] * c[k] + w[3] * d[k] for k in range(2)]


def dbez(q, t):
    a, b, c, d = q
    u = 1 - t
    return [3 * (u * u * (b[k] - a[k]) + 2 * u * t * (c[k] - b[k]) + t * t * (d[k] - c[k])) for k in range(2)]


def first_moving(*vs):
    for v in vs:
        if v != [0, 0]:
            return v
    return [0, 0]


def round_corners(pts, closed, v, seconds):
    r = v["radius"]
    if r == 0:
        return pts
    n, out = len(pts), []
    for i, (p, hi, ho) in enumerate(pts):
        if hi != [0, 0] or ho != [0, 0] or (not closed and i in (0, n - 1)):
            out.append([p, hi, ho])
            continue
        for near, first in ((pts[i - 1][0], True), (pts[(i + 1) % n][0], False)):
            d = math.hypot(p[0] - near[0], p[1] - near[1])
            f = min(d / 2, r) / d if d else 0
            q = [p[0] + (near[0] - p[0]) * f, p[1] + (near[1] - p[1]) * f]
            h = [(p[0] - q[0]) * ROUND, (p[1] - q[1]) * ROUND]
            out.append([q, [0, 0], h] if first else [q, h, [0, 0]])
    return out


def pucker_bloat(pts, closed, v, seconds):
    a = v["amount"] / 100
    n = len(pts)
    if n == 0 or a == 0:
        return pts
    c = [0, 0]
    for p, _, _ in pts:
        c = [c[0] + p[0], c[1] + p[1]]
    c = [c[0] / n, c[1] / n]
    out = []
    for p, hi, ho in pts:
        q = [p[k] + (c[k] - p[k]) * a for k in range(2)]

        def handle(h):
            e = add(p, h)
            return [e[k] + (c[k] - e[k]) * -a - q[k] for k in range(2)]

        out.append([q, handle(hi), handle(ho)])
    return out


def thirds(pts, closed):
    out = [[p[:], hi[:], ho[:]] for p, hi, ho in pts]
    n = len(out)
    for i in range(n if closed else n - 1):
        a, b = out[i], out[(i + 1) % n]
        if a[2] == [0, 0] and b[1] == [0, 0]:
            a[2] = [(b[0][k] - a[0][k]) / 3 for k in range(2)]
            b[1] = [(a[0][k] - b[0][k]) / 3 for k in range(2)]
    return out


def twist(pts, closed, v, seconds):
    angle, c = v["angle"], v["center"]
    if angle == 0 or not pts:
        return pts
    pts = thirds(pts, closed)
    far = max(math.hypot(p[0] - c[0], p[1] - c[1]) for p, _, _ in pts)
    if far == 0:
        return pts

    def turned(e):
        dx, dy = e[0] - c[0], e[1] - c[1]
        f = max(0, 1 - math.hypot(dx, dy) / far)
        s, co = math.sin(math.radians(angle * f)), math.cos(math.radians(angle * f))
        return [c[0] + co * dx - s * dy, c[1] + s * dx + co * dy]

    out = []
    for p, hi, ho in pts:
        q = turned(p)
        out.append([q, sub(turned(add(p, hi)), q), sub(turned(add(p, ho)), q)])
    return out


def zig_zag(pts, closed, v, seconds):
    size, ridges, smooth = v["size"], math.floor(v["ridges"]), v["points"] == "smooth"
    n = len(pts)
    if n < 2:
        return pts
    pts = thirds(pts, closed)
    pieces = n if closed else n - 1
    m = min(ridges + 1, max(1, LIMIT // pieces))
    along = []
    for i in range(n):
        dirs = []
        if closed or i > 0:
            a, b, c, d = piece(pts, (i - 1) % n)
            dirs.append(unit(first_moving(sub(d, c), sub(d, b), sub(d, a))))
        if closed or i < n - 1:
            a, b, c, d = piece(pts, i)
            dirs.append(unit(first_moving(sub(b, a), sub(c, a), sub(d, a))))
        tangent = unit([sum(x[0] for x in dirs), sum(x[1] for x in dirs)])
        if tangent == [0, 0]:
            tangent = dirs[-1]
        along.append((pts[i][0], tangent))
        if closed or i < n - 1:
            q = piece(pts, i)
            for k in range(1, m):
                t = k / m
                tangent = unit(dbez(q, t))
                if tangent == [0, 0]:
                    tangent = unit(sub(q[3], q[0]))
                along.append((bez(q, t), tangent))
    out, count = [], len(along)
    for j, (q, t) in enumerate(along):
        by = size if j % 2 == 0 else -size
        moved = [q[0] - t[1] * by, q[1] + t[0] * by]
        hi, ho = [0, 0], [0, 0]
        if smooth:
            if closed or j > 0:
                before = along[j - 1][0]
                d = math.hypot(q[0] - before[0], q[1] - before[1]) / 2
                hi = [-t[0] * d, -t[1] * d]
            if closed or j < count - 1:
                after = along[(j + 1) % count][0]
                d = math.hypot(q[0] - after[0], q[1] - after[1]) / 2
                ho = [t[0] * d, t[1] * d]
        out.append([moved, hi, ho])
    return out


def mix(x):
    x = (x + 0x9E3779B97F4A7C15) & M64
    z = ((x ^ (x >> 30)) * 0xBF58476D1CE4E5B9) & M64
    z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & M64
    return z ^ (z >> 31)


def rnd(seed, j, axis, k):
    h = mix(seed ^ mix(j ^ mix(axis ^ mix(k & M64))))
    return (h >> 11) / 2 ** 53 * 2 - 1


def noise(seed, j, axis, tau):
    k = math.floor(tau)
    f = tau - k
    u = f * f * (3 - 2 * f)
    return rnd(seed, j, axis, k) * (1 - u) + rnd(seed, j, axis, k + 1) * u


def split(pts, closed, m):
    """Each piece split exactly into m pieces of equal t, straight pieces evenly."""
    pts = thirds(pts, closed)
    n = len(pts)
    out = []
    ends = []
    for i in range(n if closed else n - 1):
        q = piece(pts, i)
        for k in range(m):
            t = k / m
            dv = dbez(q, t)
            point = pts[i][0] if k == 0 else bez(q, t)
            hin = pts[i][1] if k == 0 else [-x / (3 * m) for x in dv]
            out.append([point, hin, [x / (3 * m) for x in dv]])
        ends.append([-x / (3 * m) for x in dbez(q, 1)])
    if closed:
        out[0][1] = ends[-1]
    else:
        out.append([pts[-1][0], ends[-1], pts[-1][2]])
    # Each original point after the first takes the in handle its piece before it ends with.
    for i in range(1, n if closed else n - 1):
        out[i * m][1] = ends[i - 1]
    return out


SHARED = 1 << 32


def wiggle(pts, closed, v, seconds):
    size, detail = v["size"], math.floor(v["detail"])
    n = len(pts)
    if n == 0 or size == 0:
        return pts
    if n >= 2 and detail > 0:
        pts = split(pts, closed, min(detail + 1, max(1, LIMIT // (n if closed else n - 1))))
    tau = seconds * v["wiggles_per_second"]
    c, seed = v["correlation"] / 100, math.floor(v["seed"])
    out = []
    for j, (p, hi, ho) in enumerate(pts):
        d = [size * (c * noise(seed, SHARED, a, tau) + (1 - c) * noise(seed, j, a, tau)) for a in (0, 1)]
        out.append([add(p, d), hi, ho])
    return out


def flatten(pts, closed):
    """D-77's flattening, as `mask::flatten` does it."""
    out, n = [], len(pts)
    for i in range(n if closed else n - 1):
        p0, p1 = pts[i], pts[(i + 1) % n]
        a, b, c, d = piece(pts, i)
        out.append(a)
        if p0[2] == [0, 0] and p1[1] == [0, 0]:
            continue
        length = math.hypot(b[0] - a[0], b[1] - a[1])
        length += math.hypot(c[0] - b[0], c[1] - b[1])
        length += math.hypot(d[0] - c[0], d[1] - c[1])
        steps = min(512, max(16, math.ceil(length / 2)))
        for k in range(1, steps):
            out.append(bez((a, b, c, d), k / steps))
    if not closed:
        out.append(pts[-1][0])
    return out


def meet(p1, e1, p2, e2):
    den = e1[0] * e2[1] - e1[1] * e2[0]
    if den == 0:
        return None
    t = ((p2[0] - p1[0]) * e2[1] - (p2[1] - p1[1]) * e2[0]) / den
    return [p1[0] + e1[0] * t, p1[1] + e1[1] * t]


def offset(pts, closed, v, seconds):
    amount, join, limit = v["amount"], v["join"], v["miter_limit"]
    if amount == 0:
        return pts
    f = []
    for q in flatten(pts, closed):
        if not f or math.hypot(q[0] - f[-1][0], q[1] - f[-1][1]) > 1e-9:
            f.append(q)
    if closed and len(f) > 1 and math.hypot(f[0][0] - f[-1][0], f[0][1] - f[-1][1]) <= 1e-9:
        f.pop()
    n = len(f)
    if n < 2:
        return pts
    area = 0
    for i in range(n):
        a, b = f[i], f[(i + 1) % n]
        area += a[0] * b[1] - b[0] * a[1]
    s = -1 if area < 0 else 1

    def edge(i):
        a, b = f[i], f[(i + 1) % n]
        e = sub(b, a)
        length = math.hypot(e[0], e[1])
        return e, [s * e[1] / length, -s * e[0] / length], length

    def moved(p, nrm):
        return [p[0] + amount * nrm[0], p[1] + amount * nrm[1]]

    out = []
    for i in range(n):
        p = f[i]
        if not closed and i == 0:
            out.append(moved(p, edge(0)[1]))
            continue
        if not closed and i == n - 1:
            out.append(moved(p, edge(n - 2)[1]))
            continue
        (e1, n1, l1), (e2, n2, l2) = edge((i - 1) % n), edge(i)
        p1, p2 = moved(p, n1), moved(p, n2)
        cross = e1[0] * e2[1] - e1[1] * e2[0]
        if abs(cross) <= 1e-12 * l1 * l2:
            if e1[0] * e2[0] + e1[1] * e2[1] > 0:
                out.append(p1)
                continue
            outer = True
        else:
            outer = cross * s * amount > 0
        if not outer:
            q = meet(p1, e1, p2, e2)
            out.append(q if q else p1)
        elif join == "miter":
            q = meet(p1, e1, p2, e2)
            if q and math.hypot(q[0] - p[0], q[1] - p[1]) <= limit * abs(amount):
                out.append(q)
            else:
                out += [p1, p2]
        elif join == "bevel":
            out += [p1, p2]
        else:
            a1, a2 = math.atan2(n1[1], n1[0]), math.atan2(n2[1], n2[0])
            delta = a2 - a1
            if delta > math.pi:
                delta -= 2 * math.pi
            elif delta <= -math.pi:
                delta += 2 * math.pi
            steps = max(1, math.ceil(abs(delta) / (math.pi / 18)))
            out.append(p1)
            for k in range(1, steps):
                ang = a1 + delta * k / steps
                out.append([p[0] + amount * math.cos(ang), p[1] + amount * math.sin(ang)])
            out.append(p2)
    return [[q, [0, 0], [0, 0]] for q in out]


OPERATIONS = {"round_corners": round_corners, "pucker_bloat": pucker_bloat, "twist": twist,
              "zig_zag": zig_zag, "wiggle": wiggle, "offset": offset}


def operate(pts, closed, operations, frame):
    for o in operations:
        if len(pts) > LIMIT:
            continue
        v = {k: (x if isinstance(x, str) else at(x, frame)) for k, x in o.items() if k not in ("type", "note")}
        pts = OPERATIONS[o["type"]](pts, closed, v, frame / FPS)
    return pts


# --- drawing, as D-738 (c) draws, each group's operations before its turn ---


def chain(g):
    out = []
    while g is not None:
        out.append(g)
        g = GROUPS[g].get("parent")
    return out


def placed(index, choices, frame):
    s = SHAPES[index]
    raw = par.outline(s["parametric"], frame) if "parametric" in s else s["path"]["base"]["points"]
    pts = [[list(q["point"]), list(q["in"]), list(q["out"])] for q in raw]
    m, o = STILL, 1
    for g in chain(s.get("group")):
        pts = operate(pts, s["closed"], GROUPS[g].get("operations", []), frame)
        gm, go = sg.group_turn(GROUPS[g], frame)
        if "repeater" in GROUPS[g]:
            rm, ro = sr.copy_turn(sr.values(GROUPS[g]["repeater"], frame), choices[g])
            gm, go = mul(gm, rm), go * ro
        pts = [[apply(gm, *p), apply(gm, *hi, linear=True), apply(gm, *ho, linear=True)] for p, hi, ho in pts]
        m, o = mul(gm, m), o * go
    size = math.sqrt(abs(m[0] * m[3] - m[1] * m[2]))
    out = {"shape": s["name"], "copies": {GROUPS[g]["name"]: choices[g] for g in sorted(choices)},
           "points": [{"point": p, "in": hi, "out": ho} for p, hi, ho in pts]}
    if s["fill"]:
        out["fill_opacity"] = s["fill"]["opacity"] * o
    if s["stroke"]:
        out["stroke_opacity"] = s["stroke"]["opacity"] * o
        out["stroke_width"] = s["stroke"]["width_px"] * size
    return out


def drawn(frame):
    """Every shape drawn at a frame, in drawing order, later over earlier."""

    def child(index, scope):
        c = chain(SHAPES[index].get("group"))
        if scope is None:
            return ("group", c[-1]) if c else ("shape", index)
        here = c.index(scope)
        return ("group", c[here - 1]) if here > 0 else ("shape", index)

    def members(scope):
        return [i for i in range(len(SHAPES)) if scope is None or scope in chain(SHAPES[i].get("group"))]

    def walk(scope, choices):
        out, done = [], set()
        for i in members(scope):
            kind, which = child(i, scope)
            if kind == "shape":
                out.append(placed(i, choices, frame))
            elif which not in done:
                done.add(which)
                r = GROUPS[which].get("repeater")
                if r is None:
                    out += walk(which, choices)
                    continue
                v = sr.values(r, frame)
                order = range(sr.count(v))
                for c in (order if v["composite"] == "above" else reversed(order)):
                    out += walk(which, {**choices, which: c})
        return out

    return walk(None, {})


CASES, FILES = {}, {}


def case(n, says, **c):
    CASES[f"FX-SHPO-{n:03d}"] = {"says": says, **c}


case(1, "Frame 0: Corners' radius 0 leaves its square as it is; Puff's pentagon puckered 60 per cent, twice; "
     "Swirl's bar rounded 6 and twisted 120 degrees; Zig's line zig-zagged 6 with 4 ridges; Wobble's card "
     "wiggled; Grow's triangle offset 8 with round joins; Spike's star offset 6 with mitres cut to bevels past "
     "2, its open hook along one side; Cut's oval offset 10 inward; Waves' circle zig-zagged smooth; Inner's "
     "tile rounded 10, scaled 150 per cent, then twisted 60 by Outer; Plain as it is.", frame=0, drawn=drawn(0))
case(2, "Frame 6: Corners' radius 7.5; Puff -30; Wobble's points somewhere else; Cut's offset -5.",
     frame=6, drawn=drawn(6))
case(3, "Frame 12: Corners' radius 15; Puff's amount 0 leaves its pentagon as it is; Cut's offset 0 leaves its "
     "oval as it is.", frame=12, drawn=drawn(12))
case(4, "Frame 24: Corners' radius 30, the side with a handled point left alone at that point; Puff bloated 60 "
     "per cent; Cut's oval offset 10 outward.", frame=24, drawn=drawn(24))
case(5, "Saved again: each group's operations as read, Spike's `note` and `blend`, fields this build does not "
     "know, included.", saved={"groups": GROUPS})


def refused(n, says, name, change, word):
    groups = copy.deepcopy(GROUPS)
    change(groups)
    FILES[name] = project(SHAPES, groups)
    case(n, says, file=name, refused="PROJECT_SCHEMA_INVALID", names=word)


refused(20, "An operation of a type this build does not know.", "refused_type.json",
        lambda g: g[0]["operations"].append({"type": "spin", "amount": prop(1)}), "spin")
refused(21, "A radius below 0.", "refused_radius.json",
        lambda g: g[2]["operations"][0].update(radius=prop(-1)), "radius")
refused(22, "A pucker and bloat key past 100, though the base is fine.", "refused_amount_key.json",
        lambda g: g[1]["operations"][0].update(amount=prop(0, [(0, 0), (12, 150)])), "amount")
refused(23, "Ridges past 100.", "refused_ridges.json", lambda g: g[3]["operations"][0].update(ridges=prop(500)),
        "ridges")
refused(24, "Zig zag points that are neither corner nor smooth.", "refused_points.json",
        lambda g: g[3]["operations"][0].update(points="wavy"), "points")
refused(25, "An offset join that is not miter, round or bevel.", "refused_join.json",
        lambda g: g[5]["operations"][0].update(join="square"), "join")
refused(26, "A miter limit below 1.", "refused_miter_limit.json",
        lambda g: g[6]["operations"][0].update(miter_limit=prop(0.5)), "miter_limit")
refused(27, "A wiggle without its seed.", "refused_no_seed.json", lambda g: g[4]["operations"][0].pop("seed"),
        "seed")
refused(28, "A correlation past 100.", "refused_correlation.json",
        lambda g: g[4]["operations"][0].update(correlation=prop(120)), "correlation")
refused(29, "An expression on an operation's number: keys only.", "refused_expression.json",
        lambda g: g[9]["operations"][0]["angle"].update(expression={"text": "time * 90", "enabled": True}),
        "expression")
refused(30, "Nine operations on one group: eight at most.", "refused_nine.json",
        lambda g: g[2].update(operations=[op("twist", angle=10, center=[0, 0])] * 9), "operations")


def by_name(frame_case, name):
    return [d for d in CASES[frame_case]["drawn"] if d["shape"] == name]


def check():
    a = CASES["FX-SHPO-001"]["drawn"]
    assert [d["shape"] for d in a] == ["Box", "Penta", "Penta", "Bar", "Line", "Card", "Tri", "Star", "Hook",
                                       "Oval", "Disc", "Tile", "Plain"], [d["shape"] for d in a]
    # Radius 0: the square as it is, moved to (60, 70).
    box = a[0]["points"]
    assert [p["point"] for p in box] == [[100, 30], [100, 110], [20, 110], [20, 30]], box
    # Radius 30 at frame 24: three corners split in two, the handled point kept: 7 points; the
    # first corner (40, -40) gives (40 - 30, -40) toward (-40, -40), its out handle 30 x 0.5519 back.
    box = by_name("FX-SHPO-004", "Box")[0]["points"]
    assert len(box) == 7 and box[0]["point"] == [70, 30], box[0]
    assert abs(box[0]["out"][0] - 30 * ROUND) < 1e-12 and box[0]["out"][1] == 0, box[0]
    # Puff's amount 0 at frame 12: the pentagon as written; its second copy 90 to the right.
    pen = by_name("FX-SHPO-003", "Penta")
    for d, dx in zip(pen, (90, 0)):
        for q, (x, y) in zip(d["points"], pentagon(40)):
            assert math.hypot(q["point"][0] - (170 + dx + x), q["point"][1] - (70 + y)) < 1e-9, (q, x, y)
    # Puckered at frame 0, a = -0.6: the top point 1.6 times as far from the centre.
    top = by_name("FX-SHPO-001", "Penta")[1]["points"][0]["point"]
    c = [sum(x for x, _ in pentagon(40)) / 5, sum(y for _, y in pentagon(40)) / 5]
    assert abs(top[1] - (70 + c[1] + (-40 - c[1]) * 1.6)) < 1e-9, top
    # Zig: (-40, 0) moved down 6, the first split point (-24, 0) up 6, the end (40, 0) down: 6 points.
    line = a[4]["points"]
    assert len(line) == 6 and line[0]["point"] == [470, 76] and line[1]["point"] == [486, 64], line[:2]
    assert line[5]["point"] == [550, 64], line[5]
    # Grow, round joins: every point is 8 from the triangle.
    tri = [(145, 225), (215, 225), (180, 165)]

    def to_triangle(p):
        best = math.inf
        for (ax, ay), (bx, by) in zip(tri, tri[1:] + tri[:1]):
            t = max(0, min(1, ((p[0] - ax) * (bx - ax) + (p[1] - ay) * (by - ay)) / ((bx - ax) ** 2 + (by - ay) ** 2)))
            best = min(best, math.hypot(p[0] - ax - t * (bx - ax), p[1] - ay - t * (by - ay)))
        return best

    assert all(abs(to_triangle(q["point"]) - 8) < 1e-9 for q in a[6]["points"]), a[6]
    # Spike's star: tips sharper than a mitre of 2 are bevelled (two points), the inner corners
    # meet (one point): 5 x 2 + 5 points.
    assert len(a[7]["points"]) == 15, len(a[7]["points"])
    # The open hook: 3 points, its ends moved along their one edge.
    assert len(a[8]["points"]) == 3
    # Cut's offset 0 at frame 12: the oval's four points with their handles, as written.
    oval = by_name("FX-SHPO-003", "Oval")[0]["points"]
    assert len(oval) == 4 and oval[0]["point"] == [440, 175], oval[0]
    # Wobble moves between frames.
    assert by_name("FX-SHPO-001", "Card") != by_name("FX-SHPO-002", "Card")
    # Twist: Swirl's bar points farthest from the centre stay put; Outer turns Inner's rounded tile.
    # (50, -12) rounded gives (44, -12) then (50, -6), which, as far from the centre as any point,
    # the twist leaves where it is.
    bar = a[3]["points"]
    assert math.hypot(bar[1]["point"][0] - 430, bar[1]["point"][1] - 64) < 1e-9, bar[1]
    assert math.hypot(bar[0]["point"][0] - 424, bar[0]["point"][1] - 58) > 1, bar[0]
    # Splitmix64 of 0, as published.
    assert mix(0) == 0xE220A8397B1DCDAF


def table():
    rows = ["| case | says | the build must give |", "| --- | --- | --- |"]
    for cid, c in CASES.items():
        if "refused" in c:
            want = f"`{c['file']}` refused, `{c['refused']}`, naming {c['names']}"
        elif "drawn" in c:
            pts = sum(len(d["points"]) for d in c["drawn"])
            want = f"at frame {c['frame']}: {len(c['drawn'])} shapes drawn, {pts} points in all, each as written"
        else:
            want = "every group's operations as read"
        rows.append(f"| {cid} | {c['says']} | {want} |")
    print("\n".join(rows))


if __name__ == "__main__":
    check()
    if "--table" in sys.argv:
        table()
        raise SystemExit
    par.write(OUT / "host.json", HOST)
    for name, value in FILES.items():
        par.write(OUT / name, value)
    par.write(OUT / "expected_shape_operations.json", {"cases": CASES})
    print("checked")
