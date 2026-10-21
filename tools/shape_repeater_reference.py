"""D-738 (c), B-612: repeaters.

Writes `Fixtures/shape_repeaters/` and prints nothing but "checked" (or, with `--table`, document
25's table). Written before any code.

A shape group may carry `repeater`, `{"copies", "offset", "composite", "anchor", "position",
"scale", "rotation", "start_opacity", "end_opacity"}`: `composite` is "below" or "above", the rest
property records keyed as any property is, with no expression: copies 0 to 1000, offset -1000 to
1000, anchor and position two numbers, scale two numbers in per cent (-100000 to 100000), rotation
degrees, the two opacities 0 to 1. It draws everything in its group, the groups inside it
included, again and again, as After Effects' Repeater at the foot of a group's contents does.

The group's contents are drawn ceil(copies) times. Copy i (from 0) is turned, inside the group and
before the group's own transform, by R(i + offset), where for n = k + f, k whole and 0 <= f < 1,

    R(n) = step^k P(f),   step = P(1),
    P(f) = T(f position) A S(1 + (scale / 100 - 1) f) A^-1 A R(f rotation) A^-1,

A = T(anchor), R turning clockwise on screen, step^k for k below 0 being the step undone -k times.
So each copy is turned about the anchor, then scaled about it, then moved, from where the copy
before it was: rotation and position together lay copies along an arc, as After Effects' do, and
a fraction of an offset slides them part of a step. Copy i's opacity is start_opacity +
(end_opacity - start_opacity) i / (count - 1), start_opacity alone for one copy; a fraction of a
copy draws the last copy at that fraction of its opacity. With composite "below" each copy is
drawn under the one before it, so the first is on top; with "above", over it. A repeating group's
shapes are drawn together, copy by copy, where the first of them is in the layer's list; each
copy holds them in the list's order, an inner repeating group's as one block within it.

A shape is then placed as D-738 (b) places it, each group's turn being its transform after its
repeater's turn: its points by the whole turn, handles by its linear part, fill and stroke
opacity multiplied by every group's opacity and every copy's, stroke width by the square root of
the turn's size. A copy whose turn is not finite is not drawn.

`host.json` is a 400 by 300 composition with one shape layer:
- group 0 `Row` at (60, 150), repeating: copies keyed linear 4 at frame 0 to 2 at 24, position
  (70, 0), opacity from 1 down to 0.25, composite below;
- group 1 `Fan` at (300, 150), repeating 6 copies, offset keyed linear -1 at 0 to 1 at 24,
  position (5, 0), scale 90 x 80, rotation 30, composite above, and `blend_mode`, a field this
  build does not know, to be saved again unchanged;
- group 2 `Stack` inside `Row`, repeating 2 copies, position (0, 20), opacity 0.5 to 1, above;
- shape 0 `Dot` in `Row`, a 30 square with a fill and a 2 px stroke; shape 1 `Plain` in no group;
  shape 2 `Pip` in `Stack`, a triangle; shape 3 `Petal` in `Fan`, a 60 by 10 bar.
`expected_shape_repeaters.json` gives, for each frame a case names, every shape drawn in drawing
order, each as `{"shape", "copies", "points", "fill_opacity"?, "stroke_opacity"?,
"stroke_width"?}`, numbers within 1e-9; the saved project's groups; and the refused files.

**This file never runs the build's code path.** Fixtures are read-only to implementation work.
"""
import copy
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import shape_group_reference as sg  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "shape_repeaters"
par, prop, at, mul, apply = sg.par, sg.prop, sg.at, sg.mul, sg.apply
STILL = [1, 0, 0, 1, 0, 0]


def repeater(**over):
    r = {"copies": prop(3), "offset": prop(0), "composite": "below", "anchor": prop([0, 0]),
         "position": prop([100, 0]), "scale": prop([100, 100]), "rotation": prop(0),
         "start_opacity": prop(1), "end_opacity": prop(1)}
    r.update(over)
    return r


GROUPS = [
    {"name": "Row", "transform": sg.transform(position=prop([60, 150])),
     "repeater": repeater(copies=prop(4, [(0, 4), (24, 2)]), position=prop([70, 0]),
                          end_opacity=prop(0.25))},
    {"name": "Fan", "transform": sg.transform(position=prop([300, 150])),
     "repeater": dict(repeater(copies=prop(6), offset=prop(0, [(0, -1), (24, 1)]), composite="above",
                               position=prop([5, 0]), scale=prop([90, 80]), rotation=prop(30)),
                      blend_mode="multiply")},
    {"name": "Stack", "parent": 0, "transform": sg.transform(),
     "repeater": repeater(copies=prop(2), composite="above", position=prop([0, 20]),
                          start_opacity=prop(0.5))},
]

SHAPES = [
    sg.plain("Dot", [(15, -15), (15, 15), (-15, 15), (-15, -15)], group=0,
             fill={"color": [sg.GREY] * 3, "opacity": 1}, stroke={"color": [1, 1, 1], "opacity": 1, "width_px": 2}),
    sg.plain("Plain", [(170, 270), (230, 270), (200, 230)], fill={"color": [0.1, 0.8, 0.2], "opacity": 0.8}),
    sg.plain("Pip", [(-5, 30), (5, 30), (0, 22)], group=2, fill={"color": [1, 0.2, 0.1], "opacity": 1}),
    sg.plain("Petal", [(0, -5), (60, -5), (60, 5), (0, 5)], group=1,
             fill={"color": [1, 0.6, 0.1], "opacity": 0.9}, stroke={"color": [0, 0, 0], "opacity": 1, "width_px": 1}),
]
HOST = sg.project(SHAPES, GROUPS)
HOST["project_id"] = "proj-shape-repeaters"


def about(a, m):
    """m about the point a: T(a) m T(-a)."""
    return mul([1, 0, 0, 1, a[0], a[1]], mul(m, [1, 0, 0, 1, -a[0], -a[1]]))


def step(v, f):
    """P(f); the whole step is P(1)."""
    (sx, sy), (px, py) = v["scale"], v["position"]
    s = [1 + (sx / 100 - 1) * f, 0, 0, 1 + (sy / 100 - 1) * f, 0, 0]
    m = mul(about(v["anchor"], s), about(v["anchor"], sg.turn(v["rotation"] * f)))
    return mul([1, 0, 0, 1, px * f, py * f], m)


def invert(m):
    det = m[0] * m[3] - m[1] * m[2]
    a, b, c, d = m[3] / det, -m[1] / det, -m[2] / det, m[0] / det
    return [a, b, c, d, -(a * m[4] + c * m[5]), -(b * m[4] + d * m[5])]


def values(r, frame):
    return {k: (v if k == "composite" else at(v, frame)) for k, v in r.items() if k != "blend_mode"}


def count(v):
    return math.ceil(v["copies"])


def copy_turn(v, i):
    """R(i + offset) and copy i's opacity."""
    n = i + v["offset"]
    k = math.floor(n)
    whole = step(v, 1)
    m = STILL
    for _ in range(abs(k)):
        m = mul(m, whole if k > 0 else invert(whole))
    m = mul(m, step(v, n - k))
    total = count(v)
    o = v["start_opacity"] + (v["end_opacity"] - v["start_opacity"]) * (i / (total - 1) if total > 1 else 0)
    if i == total - 1 and v["copies"] != total:
        o *= v["copies"] - math.floor(v["copies"])
    return m, o


def chain(g):
    """The groups from g out to the layer."""
    out = []
    while g is not None:
        out.append(g)
        g = GROUPS[g].get("parent")
    return out


def placed(index, choices, frame):
    s = SHAPES[index]
    m, o = STILL, 1
    for g in chain(s.get("group")):
        gm, go = sg.group_turn(GROUPS[g], frame)
        if "repeater" in GROUPS[g]:
            rm, ro = copy_turn(values(GROUPS[g]["repeater"], frame), choices[g])
            gm, go = mul(gm, rm), go * ro
        m, o = mul(gm, m), o * go
    size = math.sqrt(abs(m[0] * m[3] - m[1] * m[2]))
    out = {"shape": s["name"], "copies": {GROUPS[g]["name"]: choices[g] for g in sorted(choices)},
           "points": [{"point": apply(m, *q["point"]), "in": apply(m, *q["in"], linear=True),
                       "out": apply(m, *q["out"], linear=True)} for q in s["path"]["base"]["points"]]}
    if s["fill"]:
        out["fill_opacity"] = s["fill"]["opacity"] * o
    if s["stroke"]:
        out["stroke_opacity"] = s["stroke"]["opacity"] * o
        out["stroke_width"] = s["stroke"]["width_px"] * size
    return out


def drawn(frame):
    """Every shape drawn at a frame, in drawing order, later over earlier."""

    def child(index, scope):
        """What of `scope` (a group, or None for the layer) shape `index` is or sits in."""
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
                v = values(r, frame)
                order = range(count(v))
                for c in (order if v["composite"] == "above" else reversed(order)):
                    out += walk(which, {**choices, which: c})
        return out

    return walk(None, {})


CASES, FILES = {}, {}


def case(n, says, **c):
    CASES[f"FX-SHPR-{n:03d}"] = {"says": says, **c}


case(1, "Frame 0: Row's four squares 70 apart, fading from 1 to 0.25, each under the one before; each "
     "square's Stack draws its Pip twice, 20 apart, the first at half opacity; Plain; then Fan's six bars "
     "an offset of -1 back, each turned 30 degrees, scaled 90 by 80 and moved 5 from the one before, each "
     "over the one before.", frame=0, drawn=drawn(0))
case(2, "Frame 6: Row's copies at 3.5, so four squares with the opacity ramp over four and the last at half "
     "of its; Fan's offset -0.5, half a step back.", frame=6, drawn=drawn(6))
case(3, "Frame 12: Row's copies 3; Fan's offset 0, its first bar where the bar is drawn.", frame=12, drawn=drawn(12))
case(4, "Frame 18: Row's copies 2.5; Fan's offset 0.5, every bar half a step on.", frame=18, drawn=drawn(18))
case(5, "Saved again: each group's repeater as read, `blend_mode` included.", saved={"groups": GROUPS})


def refused(n, says, name, change, word):
    groups = copy.deepcopy(GROUPS)
    change(groups)
    FILES[name] = sg.project(SHAPES, groups)
    case(n, says, file=name, refused="PROJECT_SCHEMA_INVALID", names=word)


refused(20, "Copies below 0.", "refused_copies.json", lambda g: g[0]["repeater"].update(copies=prop(-1)), "copies")
refused(21, "A copies key past 1000, though the base is fine.", "refused_copies_key.json",
        lambda g: g[1]["repeater"].update(copies=prop(6, [(0, 6), (12, 1200)])), "copies")
refused(22, "An offset past 1000.", "refused_offset.json", lambda g: g[1]["repeater"].update(offset=prop(1001)), "offset")
refused(23, "A start opacity above 1.", "refused_opacity.json",
        lambda g: g[0]["repeater"].update(start_opacity=prop(1.5)), "start opacity")
refused(24, "A composite that is neither below nor above.", "refused_composite.json",
        lambda g: g[0]["repeater"].update(composite="sideways"), "composite")
refused(25, "A repeater without its rotation.", "refused_no_rotation.json", lambda g: g[2]["repeater"].pop("rotation"),
        "rotation")
refused(26, "An expression on a repeater's number: keys only.", "refused_expression.json",
        lambda g: g[1]["repeater"]["offset"].update(expression={"text": "time", "enabled": True}), "expression")


def check():
    a = CASES["FX-SHPR-001"]["drawn"]
    assert len(a) == 4 * 3 + 1 + 6, len(a)
    # Below: Row's last copy first, at x = 60 + 3 x 70, opacity 0.25; its own Pips over it.
    assert a[0]["shape"] == "Dot" and a[0]["copies"] == {"Row": 3} and a[0]["fill_opacity"] == 0.25, a[0]
    assert a[0]["points"][0]["point"] == [60 + 210 + 15, 150 - 15], a[0]["points"][0]
    assert [d["copies"].get("Stack") for d in a[1:3]] == [0, 1] and a[1]["fill_opacity"] == 0.25 * 0.5
    assert a[2]["points"][2]["point"] == [270, 150 + 22 + 20], a[2]["points"][2]
    # The first square is drawn last of Row's, on top, opacity 1; then Plain; then Fan's.
    assert a[9]["copies"] == {"Row": 0} and a[9]["fill_opacity"] == 1 and a[12]["shape"] == "Plain"
    assert [d["copies"]["Fan"] for d in a[13:]] == [0, 1, 2, 3, 4, 5]
    # Fan at frame 12, offset 0: copy 0 is the bar at (300, 150); copy 1 is turned 30 degrees about
    # (0, 0), scaled 0.9 by 0.8, then moved 5: its first point (0, -5) goes to (0.9 x 5 sin 30 + 5,
    # -0.8 x 5 cos 30), plus (300, 150).
    fan = [d for d in CASES["FX-SHPR-003"]["drawn"] if d["shape"] == "Petal"]
    assert fan[0]["points"][0]["point"] == [300, 145], fan[0]
    p = fan[1]["points"][0]["point"]
    want = [300 + 5 + 0.9 * 5 * math.sin(math.radians(30)), 150 - 0.8 * 5 * math.cos(math.radians(30))]
    assert all(abs(x - y) < 1e-12 for x, y in zip(p, want)), (p, want)
    # Stroke width: 1 x sqrt(0.9 x 0.8) on the second bar.
    assert abs(fan[1]["stroke_width"] - math.sqrt(0.72)) < 1e-12
    # Frame 0's copy 1, offset -1, is frame 12's copy 0; frame 0's copy 0 is the step undone, so one
    # step takes its first point back to (0, -5) in the group.
    assert all(abs(x - y) < 1e-9 for x, y in zip(a[14]["points"][0]["point"], fan[0]["points"][0]["point"]))
    back = [u - v for u, v in zip(a[13]["points"][0]["point"], [300, 150])]
    there = apply(step(values(GROUPS[1]["repeater"], 0), 1), *back)
    assert abs(there[0]) < 1e-9 and abs(there[1] + 5) < 1e-9, there
    # Frame 6: Row's copies 3.5, four squares, drawn last first: 0.25 x 0.5, 0.5, 0.75, 1.
    b = [d["fill_opacity"] for d in CASES["FX-SHPR-002"]["drawn"] if d["shape"] == "Dot"]
    assert b == [0.125, 0.5, 0.75, 1], b
    # Frame 18: copies 2.5, three squares: 0.25 x 0.5, 0.625, 1.
    d = [x["fill_opacity"] for x in CASES["FX-SHPR-004"]["drawn"] if x["shape"] == "Dot"]
    assert d == [0.125, 0.625, 1], d


def table():
    rows = ["| case | says | the build must give |", "| --- | --- | --- |"]
    for cid, c in CASES.items():
        if "refused" in c:
            want = f"`{c['file']}` refused, `{c['refused']}`, naming {c['names']}"
        elif "drawn" in c:
            want = f"at frame {c['frame']}: {len(c['drawn'])} shapes drawn, in order, each placed as written"
        else:
            want = "every group's repeater as read"
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
    par.write(OUT / "expected_shape_repeaters.json", {"cases": CASES})
    print("checked")
