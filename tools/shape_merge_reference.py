"""D-738 (d), B-614: Merge Paths.

Writes `Fixtures/shape_merge/` and prints nothing but "checked" (or, with `--table`, document 25's
table). Written before any code.

A group's `operations` (B-613, `tools/shape_operation_reference.py`) may hold `{"type": "merge",
"mode"}`, `mode` one of "add", "merge", "subtract", "intersect" or "exclude" (After Effects' Add,
Merge, Subtract, Intersect and Exclude Intersections); a new one is "add", as After Effects'.
Where it stands in the list, every path drawn in the group so far, the paths of the groups inside
it and every copy of their repeaters included, each changed by the operations before it, becomes
one shape of several outlines, drawn where the first of them was and painted as the first of them
is (its fill, stroke, opacity and stroke width). The operations after it change each of its
outlines. A group inside another that merges is one shape to the outer group, as any other.

- `merge` keeps every outline as it is, points and handles, open or closed.
- The others work on the area each path fills: each shape's outlines flattened as an outline is
  (D-77), an open one closed by a straight line from its last point to its first, its area by
  the even-odd rule, as its fill is. A point is in the result where it is in any of them (add),
  in the first and none of the rest (subtract), in all of them (intersect) or in an odd number of
  them (exclude). The result is the outlines of that area, corners only, each closed, the area on
  the (-dy, dx) side of each edge (with y down, outer outlines clockwise on screen, holes the
  other way round).

The outlines are worked out so:
1. Every edge of every flattened outline, in order (shape, outline, edge, a closed outline's edge
   from its last point to its first included), an edge whose two ends are the same point left out.
2. For each pair of edges s before t, p = s's start, r = its end less p, q and u likewise for t,
   den = r x u, w = q - p, EPS = 1e-9. Where den is not 0, ts = (w x u) / den and tu = (w x r) /
   den: with both strictly between EPS and 1 - EPS the edges cross at p + ts r, cut into both at
   that one point; with ts so and tu within EPS of 0 or 1, t's end lies on s and cuts s there;
   with tu so and ts within EPS of 0 or 1, s's end cuts t. Where den is 0 and q lies within 1e-9
   of s's line (|w x r| <= 1e-9 |r|), each end of t whose share along s, (e - p).r / r.r, is
   strictly between EPS and 1 - EPS cuts s there, and each end of s cuts t likewise. A pair whose
   boxes are more than 1e-6 apart is not asked.
3. Each edge's cuts, sorted by share (in the order found where two are equal), give its pieces,
   from cut to cut, a piece whose two ends are the same point left out.
4. A piece from a to b, with m its middle, n = (-dy, dx) / |b - a| and D = 1e-6: where m + D n and
   m - D n are both in or both out of the result, it is left out; else it is kept as a to b where
   m + D n is in, b to a where it is not. A piece the same as one kept before it is left out.
5. The kept pieces are joined in order: from the first not used, each next is the first not used
   that starts where the last ended, until the outline is back at its start or no piece goes on;
   an outline of fewer than three points is left out.
A merge over more than 20000 edges in all leaves the paths as they are.

`host.json` is a 600 by 400 composition at 24 fps holding one shape layer of nine groups and
sixteen shapes; `expected_shape_merge.json` gives, for each frame a case names, every shape drawn
in drawing order, each as `{"shape", "copies", "paths": [{"closed", "points"}], "fill_opacity"?,
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
import shape_operation_reference as so  # noqa: E402
import shape_repeater_reference as sr  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "shape_merge"
par, prop, at, mul, apply = sg.par, sg.prop, sg.at, sg.mul, sg.apply
FPS = so.FPS
EPS = 1e-9
D = 1e-6
BOX = 1e-6
MOST_EDGES = 20000
op, group, path, corners, fill, WHITE = so.op, so.group, so.path, so.corners, so.fill, so.WHITE


def ellipse(name, w, h, position, g, fill=None, stroke=None):
    return so.parametric(name, {"type": "ellipse", "size": prop([w, h]), "position": position}, g,
                         fill=fill, stroke=stroke)


def box(x0, y0, x1, y1):
    return corners([(x1, y0), (x1, y1), (x0, y1), (x0, y0)])


GROUPS = [
    group("Union", 90, 90, [op("merge", mode="add")]),
    group("Bite", 250, 90, [op("merge", mode="subtract")]),
    group("Overlap", 390, 90, [op("merge", mode="intersect")]),
    group("Xor", 520, 90, [op("merge", mode="exclude")],
          repeater=sr.repeater(copies=prop(2), position=prop([0, 120]))),
    group("Keep", 90, 250, [op("merge", mode="merge"), op("round_corners", radius=8)]),
    group("Wheel", 250, 260, [op("merge", mode="add"), op("round_corners", radius=3)]),
    {"name": "Spokes", "parent": 5, "transform": sg.transform(),
     "repeater": sr.repeater(copies=prop(6), position=prop([0, 0]), rotation=prop(30))},
    group("Puffed", 400, 260, [op("pucker_bloat", amount=30), op("merge", mode="add")]),
    group("Join", 520, 330, [dict(op("merge", mode="add"), note="kept")], blend="multiply"),
]

SHAPES = [
    ellipse("Sun", 80, 80, prop([0, 0]), 0, fill=fill(1, 0.6, 0.1), stroke=dict(WHITE, width_px=3)),
    ellipse("Moon", 60, 60, prop([30, 0], [(0, [30, 0]), (24, [75, 0])]), 0, fill=fill(0.2, 0.4, 1)),
    path("Slab", box(-50, -50, 50, 50), 1, fill=fill(0.9, 0.2, 0.5), stroke=WHITE),
    so.parametric("Hole", {"type": "ellipse", "size": prop([30, 30], [(0, [30, 30]), (24, [90, 90])]),
                           "position": prop([30, 0])}, 1, fill=fill(0, 0, 0)),
    path("Wide", box(-50, -20, 50, 20), 2, fill=fill(0.3, 0.8, 0.4), stroke=WHITE),
    path("Peak", corners([(0, -55), (45, 40), (-45, 40)]), 2, fill=fill(1, 1, 1)),
    path("Left", box(-45, -30, 15, 30), 3, fill=fill(0.5, 0.3, 1), stroke=WHITE),
    path("Right", box(-15, -20, 45, 40), 3, fill=fill(1, 1, 1)),
    path("A", box(-40, -30, 20, 30), 4, fill=fill(0.2, 0.9, 0.5), stroke=WHITE),
    path("B", box(-20, -20, 40, 40), 4, fill=fill(1, 1, 1), stroke=WHITE),
    path("Spoke", box(-50, -6, 50, 6), 6, fill=fill(1, 0.85, 0.2), stroke={"color": [0, 0, 0], "opacity": 1,
                                                                          "width_px": 1}),
    path("P1", box(-40, -25, 10, 25), 7, fill=fill(0.2, 0.9, 0.9)),
    path("P2", box(-10, -15, 40, 35), 7, fill=fill(1, 1, 1)),
    path("J1", box(-30, -30, 0, 30), 8, fill=fill(1, 0.4, 0.3), stroke=WHITE),
    path("J2", box(0, -30, 30, 30), 8, fill=fill(1, 1, 1)),
    path("Plain", corners([(20, 390), (80, 390), (50, 340)]), fill=fill(0.6, 0.6, 0.6)),
]


def project(shapes, groups):
    p = so.project(shapes, groups)
    p["project_id"] = "proj-shape-merge"
    return p


HOST = project(SHAPES, GROUPS)


# --- the area and its outlines ---


def inside(outlines, x, y):
    k = False
    for c in outlines:
        n = len(c)
        for i in range(n):
            x0, y0 = c[i]
            x1, y1 = c[(i + 1) % n]
            if (y0 <= y) != (y1 <= y):
                t = (y - y0) / (y1 - y0)
                if x < x0 + t * (x1 - x0):
                    k = not k
    return k


def member(regions, mode, x, y):
    ins = [inside(r, x, y) for r in regions]
    if mode == "add":
        return any(ins)
    if mode == "subtract":
        return ins[0] and not any(ins[1:])
    if mode == "intersect":
        return all(ins)
    return sum(ins) % 2 == 1


def edges_of(regions):
    out = []
    for r in regions:
        for c in r:
            n = len(c)
            for i in range(n):
                a, b = tuple(c[i]), tuple(c[(i + 1) % n])
                if a != b:
                    out.append((a, b))
    return out


def outlines(regions, mode):
    segs = edges_of(regions)
    cuts = [[(0.0, a), (1.0, b)] for a, b in segs]
    boxes = [(min(a[0], b[0]), min(a[1], b[1]), max(a[0], b[0]), max(a[1], b[1])) for a, b in segs]
    for s in range(len(segs)):
        (px, py), (bx, by) = segs[s]
        rx, ry = bx - px, by - py
        for t in range(s + 1, len(segs)):
            if (boxes[t][0] > boxes[s][2] + BOX or boxes[t][2] < boxes[s][0] - BOX
                    or boxes[t][1] > boxes[s][3] + BOX or boxes[t][3] < boxes[s][1] - BOX):
                continue
            (qx, qy), (dx, dy) = segs[t]
            ux, uy = dx - qx, dy - qy
            wx, wy = qx - px, qy - py
            den = rx * uy - ry * ux
            if den != 0:
                ts = (wx * uy - wy * ux) / den
                tu = (wx * ry - wy * rx) / den
                ins, inu = EPS < ts < 1 - EPS, EPS < tu < 1 - EPS
                if ins and inu:
                    p = (px + ts * rx, py + ts * ry)
                    cuts[s].append((ts, p))
                    cuts[t].append((tu, p))
                elif ins and (abs(tu) <= EPS or abs(tu - 1) <= EPS):
                    cuts[s].append((ts, segs[t][0] if abs(tu) <= EPS else segs[t][1]))
                elif inu and (abs(ts) <= EPS or abs(ts - 1) <= EPS):
                    cuts[t].append((tu, segs[s][0] if abs(ts) <= EPS else segs[s][1]))
                continue
            rr, uu = rx * rx + ry * ry, ux * ux + uy * uy
            if abs(wx * ry - wy * rx) > 1e-9 * math.sqrt(rr):
                continue
            for e in segs[t]:
                f = ((e[0] - px) * rx + (e[1] - py) * ry) / rr
                if EPS < f < 1 - EPS:
                    cuts[s].append((f, e))
            for e in segs[s]:
                f = ((e[0] - qx) * ux + (e[1] - qy) * uy) / uu
                if EPS < f < 1 - EPS:
                    cuts[t].append((f, e))
    kept, seen = [], set()
    for cs in cuts:
        cs = sorted(cs, key=lambda c: c[0])
        for (_, a), (_, b) in zip(cs, cs[1:]):
            if a == b:
                continue
            dx, dy = b[0] - a[0], b[1] - a[1]
            length = math.hypot(dx, dy)
            nx, ny = -dy / length, dx / length
            mx, my = (a[0] + b[0]) / 2, (a[1] + b[1]) / 2
            left = member(regions, mode, mx + D * nx, my + D * ny)
            right = member(regions, mode, mx - D * nx, my - D * ny)
            if left == right:
                continue
            e = (a, b) if left else (b, a)
            if e in seen:
                continue
            seen.add(e)
            kept.append(e)
    starts = {}
    for i, (a, _) in enumerate(kept):
        starts.setdefault(a, []).append(i)
    used, out = [False] * len(kept), []
    for i, (a, b) in enumerate(kept):
        if used[i]:
            continue
        used[i] = True
        c, end = [a], b
        while end != a:
            nxt = next((j for j in starts.get(end, []) if not used[j]), None)
            c.append(end)
            if nxt is None:
                break
            used[nxt] = True
            end = kept[nxt][1]
        if len(c) >= 3:
            out.append(c)
    return out


def merge(items, mode):
    if not items:
        return items
    if mode == "merge":
        paths = [p for it in items for p in it["paths"]]
    else:
        regions = [[so.flatten(pts, closed) for pts, closed in it["paths"]] for it in items]
        if len(edges_of(regions)) > MOST_EDGES:
            return items
        paths = [[[[[x, y], [0, 0], [0, 0]] for x, y in c], True] for c in outlines(regions, mode)]
    return [dict(items[0], paths=paths)]


# --- drawing: each group's contents, then its operations, then its turn ---


def chain(g):
    out = []
    while g is not None:
        out.append(g)
        g = GROUPS[g].get("parent")
    return out


def copy_turn(g, choices, frame):
    gm, go = sg.group_turn(GROUPS[g], frame)
    if "repeater" in GROUPS[g]:
        rm, ro = sr.copy_turn(sr.values(GROUPS[g]["repeater"], frame), choices[g])
        gm, go = mul(gm, rm), go * ro
    return gm, go


def leaf(index, choices, frame):
    s = SHAPES[index]
    raw = par.outline(s["parametric"], frame) if "parametric" in s else s["path"]["base"]["points"]
    pts = [[list(q["point"]), list(q["in"]), list(q["out"])] for q in raw]
    m, o = so.STILL, 1
    for g in chain(s.get("group")):
        gm, go = copy_turn(g, choices, frame)
        m, o = mul(gm, m), o * go
    out = {"shape": s["name"], "copies": {GROUPS[g]["name"]: choices[g] for g in sorted(choices)},
           "paths": [[pts, s["closed"]]]}
    if s["fill"]:
        out["fill_opacity"] = s["fill"]["opacity"] * o
    if s["stroke"]:
        out["stroke_opacity"] = s["stroke"]["opacity"] * o
        out["stroke_width"] = s["stroke"]["width_px"] * math.sqrt(abs(m[0] * m[3] - m[1] * m[2]))
    return out


def contents(scope, choices, frame):
    def child(index):
        c = chain(SHAPES[index].get("group"))
        if scope is None:
            return ("group", c[-1]) if c else ("shape", index)
        here = c.index(scope)
        return ("group", c[here - 1]) if here > 0 else ("shape", index)

    items, done = [], set()
    for i in range(len(SHAPES)):
        if scope is not None and scope not in chain(SHAPES[i].get("group")):
            continue
        kind, which = child(i)
        if kind == "shape":
            items.append(leaf(i, choices, frame))
        elif which not in done:
            done.add(which)
            r = GROUPS[which].get("repeater")
            if r is None:
                items += contents(which, choices, frame)
                continue
            v = sr.values(r, frame)
            order = range(sr.count(v))
            for c in (order if v["composite"] == "above" else reversed(order)):
                items += contents(which, {**choices, which: c}, frame)
    if scope is None:
        return items
    for o in GROUPS[scope].get("operations", []):
        v = {k: (x if isinstance(x, str) else at(x, frame)) for k, x in o.items() if k not in ("type", "note")}
        if o["type"] == "merge":
            items = merge(items, v["mode"])
            continue
        work = so.OPERATIONS[o["type"]]
        for it in items:
            it["paths"] = [[pts if len(pts) > so.LIMIT else work(pts, closed, v, frame / FPS), closed]
                           for pts, closed in it["paths"]]
    gm, _ = copy_turn(scope, choices, frame)
    for it in items:
        it["paths"] = [[[[apply(gm, *p), apply(gm, *hi, linear=True), apply(gm, *ho, linear=True)]
                         for p, hi, ho in pts], closed] for pts, closed in it["paths"]]
    return items


def drawn(frame):
    """Every shape drawn at a frame, in drawing order, later over earlier."""
    out = []
    for it in contents(None, {}, frame):
        d = {k: v for k, v in it.items() if k != "paths"}
        d["paths"] = [{"closed": closed, "points": [{"point": p, "in": hi, "out": ho} for p, hi, ho in pts]}
                      for pts, closed in it["paths"]]
        out.append(d)
    return out


CASES, FILES = {}, {}


def case(n, says, **c):
    CASES[f"FX-SHPM-{n:03d}"] = {"says": says, **c}


case(1, "Frame 0: Union adds Moon to Sun, one outline round both, stroked white as Sun is; Bite takes a "
     "30-wide Hole out of Slab's middle, a square with a round hole; Overlap keeps where Wide and Peak "
     "overlap; Xor keeps where Left or Right is but not both, twice, 120 apart; Keep keeps A's and B's "
     "outlines, the overlap filled by the even-odd rule, every corner rounded 8; Wheel adds the six "
     "copies of Spoke into one star, its corners rounded 3; Puffed bloats P1 and P2, then adds them; "
     "Join adds two squares sharing a side into one; Plain as it is.", frame=0, drawn=drawn(0))
case(2, "Frame 12: Moon half way out from Sun; Hole 60 wide, through Slab's right side, a bite.",
     frame=12, drawn=drawn(12))
case(3, "Frame 24: Moon clear of Sun, two outlines in one shape; Hole 90 wide, a deeper bite.",
     frame=24, drawn=drawn(24))
case(4, "Saved again: each group's operations as read, Join's `note` and `blend`, fields this build does not "
     "know, included.", saved={"groups": GROUPS})


def refused(n, says, name, change, word):
    groups = copy.deepcopy(GROUPS)
    change(groups)
    FILES[name] = project(SHAPES, groups)
    case(n, says, file=name, refused="PROJECT_SCHEMA_INVALID", names=word)


refused(20, "A merge mode this build does not know.", "refused_mode.json",
        lambda g: g[0]["operations"][0].update(mode="union"), "mode")
refused(21, "A merge without its mode.", "refused_no_mode.json", lambda g: g[1]["operations"][0].pop("mode"),
        "mode")


def by_name(frame_case, name):
    return [d for d in CASES[frame_case]["drawn"] if d["shape"] == name]


def area(c):
    return sum(c[i][0] * c[(i + 1) % len(c)][1] - c[(i + 1) % len(c)][0] * c[i][1] for i in range(len(c))) / 2


def areas(d):
    return [area([p["point"] for p in path["points"]]) for path in d["paths"]]


def check():
    a = CASES["FX-SHPM-001"]["drawn"]
    assert [d["shape"] for d in a] == ["Sun", "Slab", "Wide", "Left", "Left", "A", "Spoke", "P1", "J1",
                                       "Plain"], [d["shape"] for d in a]
    # Union: one outline at frame 0, two at frame 24; painted as Sun.
    assert len(a[0]["paths"]) == 1 and a[0]["stroke_width"] == 3
    assert len(by_name("FX-SHPM-003", "Sun")[0]["paths"]) == 2
    # Every boolean outline has its area to the side of (-dy, dx): with y down an outer outline's
    # shoelace sum is positive, a hole's negative, and each shape's sum is its area.
    for c in CASES.values():
        for d in c.get("drawn", []):
            if d["shape"] not in ("A", "Plain"):
                assert sum(areas(d)) > 0 and areas(d)[0] > 0, (d["shape"], areas(d))
    # Bite at frame 0: the square and its round hole, run the other way round.
    bite = areas(a[1])
    assert len(bite) == 2 and abs(bite[0] - 10000) < 1e-6 and -710 < bite[1] < -700, bite
    # Overlap: Wide and Peak's common area, worked by hand: the band -20 <= y <= 20 of the
    # triangle, whose width at y is 90 (y + 55) / 95, inside |x| <= 50 everywhere there.
    want = sum(90 * ((y0 + y1) / 2 + 55) / 95 * (y1 - y0) for y0, y1 in [(-20, 20)])
    assert abs(sum(areas(a[2])) - want) < 1e-6, (areas(a[2]), want)
    # Xor: 3600 + 3600 less twice the 30 by 50 overlap.
    assert abs(sum(areas(a[3])) - 4200) < 1e-6 and abs(sum(areas(a[4])) - 4200) < 1e-6
    assert a[3]["copies"] == {"Xor": 1} and a[4]["copies"] == {"Xor": 0}
    # Keep: both outlines, each closed square rounded at four corners: 8 points each.
    assert [len(p["points"]) for p in a[5]["paths"]] == [8, 8]
    # Join: one outline round the 60 by 60 square, its shared side gone.
    assert len(a[8]["paths"]) == 1 and abs(areas(a[8])[0] - 3600) < 1e-6, areas(a[8])
    # Wheel: one outline.
    assert len(a[6]["paths"]) == 1
    # Bite at frame 12: Hole reaches past Slab's side, one outline.
    assert len(by_name("FX-SHPM-002", "Slab")[0]["paths"]) == 1


def table():
    rows = ["| case | says | the build must give |", "| --- | --- | --- |"]
    for cid, c in CASES.items():
        if "refused" in c:
            want = f"`{c['file']}` refused, `{c['refused']}`, naming {c['names']}"
        elif "drawn" in c:
            outs = sum(len(d["paths"]) for d in c["drawn"])
            pts = sum(len(p["points"]) for d in c["drawn"] for p in d["paths"])
            want = (f"at frame {c['frame']}: {len(c['drawn'])} shapes drawn, {outs} outlines, {pts} points in "
                    "all, each as written")
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
    par.write(OUT / "expected_shape_merge.json", {"cases": CASES})
    print("checked")
