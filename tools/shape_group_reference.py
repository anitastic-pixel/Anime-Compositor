"""D-738 (b), B-610: shape groups.

Writes `Fixtures/shape_groups/` and prints nothing but "checked" (or, with `--table`, document
25's table). Written before any code.

A shape layer may carry `groups`, a list of groups, each `{"name", "parent"?, "transform"}`:
`parent` is the place in the list of the group it sits in (absent: the layer's own contents), and
`transform` holds `anchor` [x, y], `position` [x, y], `scale` [x, y] per cent, `rotation` degrees,
`opacity` 0 to 1, `skew` degrees (-85 to 85) and `skew_axis` degrees, each a property record keyed
as any property is, with no expression. A shape may carry `group`, the place of the group it is
in. A group's transform turns a point p into

    T(position) R(rotation) K(skew, skew_axis) S(scale / 100) T(-anchor) p,

R turning clockwise on screen, and K = R(-axis) H R(axis) with H the shear x' = x - tan(skew) y,
as lottie-web (MIT) writes After Effects' group transform down. A shape in a group in a group is
turned by its own group first and then by each group it is in. A shape's points are placed by the
whole turn and its handles by its linear part; its fill's and stroke's gradient points are placed
as points; its fill's and stroke's opacity are multiplied by every group's opacity; its stroke
width is multiplied by the square root of the linear part's determinant (its size), which keeps a
stroke round where After Effects would stretch it under a scale that is not even.

`host.json` is a 400 by 300 composition with one shape layer of four shapes and three groups:
- group 0 `Body`: at (200, 150), opacity 0.5, rotation keyed linear from 0 at frame 0 to 90 at 24;
- group 1 `Arm`, inside `Body`: at (60, 0), anchor (10, 0), scale 150 x 50, skew 20 along 30;
- group 2 `Drift`: position keyed linear from (0, 0) at frame 0 to (40, 20) at 24, and `blend`, a
  field this build does not know, to be saved again unchanged;
- shape 0 `Torso` in `Body`: an 80 square about (0, 0) with a linear gradient fill from (-40, 0)
  to (40, 0) and a 4 px stroke;
- shape 1 `Hand` in `Arm`: a parametric 30 by 20 ellipse at (0, 0) with a 2 px stroke;
- shape 2 `Free` in no group: a triangle, drawn where its path is;
- shape 3 `Badge` in `Drift`: a parametric five-pointed star at (320, 70).
`expected_shape_groups.json` gives for each drawn case a shape, a frame and the shape as placed
there: its points as the file writes a path's, its fill's opacity and gradient points, its
stroke's opacity and width, numbers within 1e-9; the saved project's groups and shapes; and the
refused files, each with the diagnostic and a word its message must contain.

**This file never runs the build's code path.** Fixtures are read-only to implementation work.
"""
import copy
import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import parametric_shape_reference as par  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "shape_groups"
prop, at, pt = par.prop, par.at, par.pt


def transform(**over):
    t = {"anchor": prop([0, 0]), "position": prop([0, 0]), "scale": prop([100, 100]),
         "rotation": prop(0), "opacity": prop(1), "skew": prop(0), "skew_axis": prop(0)}
    t.update(over)
    return t


GROUPS = [
    {"name": "Body", "transform": transform(position=prop([200, 150]), opacity=prop(0.5),
                                            rotation=prop(0, [(0, 0), (24, 90)]))},
    {"name": "Arm", "parent": 0, "transform": transform(position=prop([60, 0]), anchor=prop([10, 0]),
                                                        scale=prop([150, 50]), skew=prop(20),
                                                        skew_axis=prop(30))},
    {"name": "Drift", "transform": transform(position=prop([0, 0], [(0, [0, 0]), (24, [40, 20])])),
     "blend": "multiply"},
]

GREY = par.GREY


def plain(name, corners, group=None, fill=None, stroke=None):
    s = {"name": name, "enabled": True, "closed": True,
         "path": {"base": {"points": [pt(x, y) for x, y in corners]}, "keyframes": []},
         "fill": fill, "stroke": stroke}
    if group is not None:
        s["group"] = group
    return s


def parametric(name, p, group, fill=None, stroke=None):
    s = par.shape(name, p, fill=fill, stroke=stroke)
    s["group"] = group
    return s


GRADIENT = {"type": "linear", "start": prop([-40, 0]), "end": prop([40, 0]),
            "stops": [{"offset": 0, "color": [1, 0.2, 0.1], "opacity": 1},
                      {"offset": 1, "color": [0.1, 0.3, 1], "opacity": 1}]}
SHAPES = [
    plain("Torso", [(40, -40), (40, 40), (-40, 40), (-40, -40)], group=0,
          fill={"color": [GREY] * 3, "opacity": 1, "gradient": GRADIENT},
          stroke={"color": [1, 1, 1], "opacity": 1, "width_px": 4}),
    parametric("Hand", {"type": "ellipse", "size": prop([30, 20]), "position": prop([0, 0])}, 1,
               stroke={"color": [1, 1, 0.2], "opacity": 1, "width_px": 2}),
    plain("Free", [(30, 260), (90, 260), (60, 210)], fill={"color": [0.1, 0.8, 0.2], "opacity": 0.8}),
    parametric("Badge", {"type": "star", "points": prop(5), "position": prop([320, 70]), "rotation": prop(0),
                         "inner_radius": prop(15), "outer_radius": prop(30), "inner_roundness": prop(0),
                         "outer_roundness": prop(0)}, 2, fill={"color": [1, 0.8, 0.1], "opacity": 1}),
]


def project(shapes, groups):
    p = par.project(shapes)
    p["project_id"] = "proj-shape-groups"
    p["compositions"][0]["layers"][0]["groups"] = groups
    return p


HOST = project(SHAPES, GROUPS)


def mul(a, b):
    """Two turns [a, b, c, d, e, f], x' = a x + c y + e, y' = b x + d y + f: a after b."""
    return [a[0] * b[0] + a[2] * b[1], a[1] * b[0] + a[3] * b[1],
            a[0] * b[2] + a[2] * b[3], a[1] * b[2] + a[3] * b[3],
            a[0] * b[4] + a[2] * b[5] + a[4], a[1] * b[4] + a[3] * b[5] + a[5]]


def turn(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return [c, s, -s, c, 0, 0]


def group_turn(g, frame):
    v = {k: at(r, frame) for k, r in g["transform"].items()}
    (ax, ay), (px, py), (sx, sy) = v["anchor"], v["position"], v["scale"]
    shear = [1, 0, -math.tan(math.radians(v["skew"])), 1, 0, 0]
    m = [1, 0, 0, 1, -ax, -ay]
    m = mul([sx / 100, 0, 0, sy / 100, 0, 0], m)
    m = mul(mul(turn(-v["skew_axis"]), mul(shear, turn(v["skew_axis"]))), m)
    m = mul(turn(v["rotation"]), m)
    m = mul([1, 0, 0, 1, px, py], m)
    return m, v["opacity"]


def placing(groups, g, frame):
    """The whole turn and opacity of group g, its parents' included."""
    m, o = [1, 0, 0, 1, 0, 0], 1
    while g is not None:
        gm, go = group_turn(groups[g], frame)
        m, o = mul(gm, m), o * go
        g = groups[g].get("parent")
    return m, o


def apply(m, x, y, linear=False):
    return [m[0] * x + m[2] * y + (0 if linear else m[4]), m[1] * x + m[3] * y + (0 if linear else m[5])]


def placed(index, frame):
    s = SHAPES[index]
    if "parametric" in s:
        points = par.outline(s["parametric"], frame)
    else:
        points = copy.deepcopy(s["path"]["base"]["points"])
    m, o = placing(GROUPS, s.get("group"), frame)
    size = math.sqrt(abs(m[0] * m[3] - m[1] * m[2]))
    out = {"points": [{"point": apply(m, *q["point"]), "in": apply(m, *q["in"], linear=True),
                       "out": apply(m, *q["out"], linear=True)} for q in points]}
    if s["fill"]:
        out["fill_opacity"] = s["fill"]["opacity"] * o
        g = s["fill"].get("gradient")
        if g:
            out["fill_start"] = apply(m, *at(g["start"], frame))
            out["fill_end"] = apply(m, *at(g["end"], frame))
    if s["stroke"]:
        out["stroke_opacity"] = s["stroke"]["opacity"] * o
        out["stroke_width"] = s["stroke"]["width_px"] * size
    return out


CASES = {}
FILES = {}


def case(n, says, **c):
    CASES[f"FX-SHPG-{n:03d}"] = {"says": says, **c}


def drawn(n, says, index, frame):
    case(n, says, shape=index, frame=frame, placed=placed(index, frame))


drawn(1, "A shape in a group: moved to the group's position, its opacity halved, its gradient moved with it.", 0, 0)
drawn(2, "Half way through the group's rotation key, the square is turned 45 degrees clockwise about (200, 150).", 0, 12)
drawn(3, "A shape in a group in a group: scaled, skewed and moved by its own group, then by the outer one; its "
      "stroke's width scaled by the square root of the scale's area.", 1, 0)
drawn(4, "The same at frame 24, the outer group turned 90 degrees.", 1, 24)
drawn(5, "A shape in no group is drawn where its path is, as before.", 2, 0)
drawn(6, "A parametric star in a group whose position is keyed: half way at frame 12.", 3, 12)
case(7, "Saved again: each group as read, `blend` included; each shape keeps its group; every path as read, "
     "not placed.", saved={"groups": GROUPS, "shape_groups": [s.get("group") for s in SHAPES],
                           "paths": [s["path"] for s in SHAPES]})


def refused(n, says, name, change, word):
    shapes, groups = copy.deepcopy(SHAPES), copy.deepcopy(GROUPS)
    change(shapes, groups)
    FILES[name] = project(shapes, groups)
    case(n, says, file=name, refused="PROJECT_SCHEMA_INVALID", names=word)


refused(20, "A shape in a group the layer does not have.", "refused_shape_group.json",
        lambda s, g: s[0].update(group=3), "group")
refused(21, "A group inside a group the layer does not have.", "refused_parent.json",
        lambda s, g: g[1].update(parent=7), "parent")
refused(22, "Groups inside each other in a loop.", "refused_loop.json",
        lambda s, g: g[0].update(parent=1), "parent")
refused(23, "An opacity above 1.", "refused_opacity.json",
        lambda s, g: g[0]["transform"].update(opacity=prop(1.5)), "opacity")
refused(24, "A skew key past 85 degrees, though the base is fine.", "refused_skew.json",
        lambda s, g: g[1]["transform"].update(skew=prop(20, [(0, 20), (12, 90)])), "skew")
refused(25, "A transform without its rotation.", "refused_no_rotation.json",
        lambda s, g: g[2]["transform"].pop("rotation"), "rotation")
refused(26, "An expression on a group's number: keys only, as a shape's other numbers.", "refused_expression.json",
        lambda s, g: g[0]["transform"]["position"].update(expression={"text": "wiggle(2, 10)", "enabled": True}),
        "expression")


def check():
    # Body at frame 0: (40, -40) is (240, 110); at frame 12 the square's corner is turned 45
    # degrees clockwise about (200, 150), so (40, -40) lands straight right of it.
    a = CASES["FX-SHPG-001"]["placed"]
    assert a["points"][0]["point"] == [240, 110] and a["fill_opacity"] == 0.5, a
    b = CASES["FX-SHPG-002"]["placed"]["points"][0]["point"]
    assert abs(b[0] - (200 + 40 * math.sqrt(2))) < 1e-9 and abs(b[1] - 150) < 1e-9, b
    # The hand's stroke: 2 x sqrt(1.5 x 0.5), the skew and turns having no area.
    h = CASES["FX-SHPG-003"]["placed"]
    assert abs(h["stroke_width"] - 2 * math.sqrt(0.75)) < 1e-12 and h["stroke_opacity"] == 0.5, h
    # With no skew, the Arm's anchor (10, 0) lands on its position (60, 0) in Body, (260, 150).
    g = copy.deepcopy(GROUPS[1])
    g["transform"]["skew"] = prop(0)
    m, _ = group_turn(g, 0)
    assert apply(m, 10, 0) == [60, 0], apply(m, 10, 0)
    # Skew 20 along 0 leans the top right: (0, -1) goes to (tan 20, -1).
    g["transform"].update(skew=prop(20), skew_axis=prop(0), anchor=prop([0, 0]), position=prop([0, 0]),
                          scale=prop([100, 100]))
    m, _ = group_turn(g, 0)
    q = apply(m, 0, -1)
    assert abs(q[0] - math.tan(math.radians(20))) < 1e-12 and abs(q[1] + 1) < 1e-12, q
    assert CASES["FX-SHPG-005"]["placed"]["points"][2]["point"] == [60, 210]
    d = CASES["FX-SHPG-006"]["placed"]["points"][0]["point"]
    assert abs(d[0] - 340) < 1e-9 and abs(d[1] - 50) < 1e-9, d


def table():
    rows = ["| case | says | the build must give |", "| --- | --- | --- |"]
    for cid, c in CASES.items():
        if "refused" in c:
            want = f"`{c['file']}` refused, `{c['refused']}`, naming {c['names']}"
        elif "placed" in c:
            want = f"shape {c['shape']} at frame {c['frame']}: {len(c['placed']['points'])} points and its paint as written"
        else:
            want = "the groups, each shape's group and every path as read"
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
    par.write(OUT / "expected_shape_groups.json", {"cases": CASES})
    print("checked")
