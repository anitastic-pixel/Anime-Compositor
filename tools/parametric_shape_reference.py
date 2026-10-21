"""D-738 (a), B-609: parametric shapes.

Writes `Fixtures/parametric_shapes/` and prints nothing but "checked" (or, with `--table`,
document 25's table). Written before any code.

A shape may carry `parametric`, `{"type": "rectangle" | "ellipse" | "polygon" | "star", ...}`,
whose settings are property records keyed as any property is (no expression, no spatial):
- rectangle: `size` [w, h], `position` [x, y], `roundness`;
- ellipse: `size`, `position`;
- polygon: `points`, `position`, `rotation` (degrees), `outer_radius`, `outer_roundness` (per cent);
- star: a polygon's, and `inner_radius` and `inner_roundness`.
Its outline at a frame is worked from the settings at that frame, as After Effects draws it (and
as lottie-web, MIT, writes it down):
- a rectangle is centred on `position`. With a roundness of 0 it is four corners, top right first,
  then clockwise on screen. Otherwise it is eight points, the radius `r` the roundness held to half
  the shorter side, starting at (x + w/2, y - h/2 + r) on the right side and going down it; each
  point beside a corner has a handle pointing to that corner, KAPPA x r long;
- an ellipse is four points, top, right, bottom, left, each with handles along its tangent, KAPPA
  times the half width (top and bottom) or the half height (left and right) long, the out handle
  going clockwise;
- a polygon is floor(points) corners on a circle of `outer_radius`, the first straight up from
  `position` turned by `rotation`, then clockwise; a star is twice as many, outer and inner by
  turns. Each corner has handles along the circle's tangent, 2 pi radius / (4 n) x roundness / 100
  long, n being floor(points), the out handle going clockwise.
KAPPA is 4/3 (sqrt 2 - 1). The file's `path` holds the outline at the settings' bases with no
keys, written by the program so that a build without D-738 still draws the shape, standing still;
a build with it reads the settings and works `path` out again.

`host.json` is a 400 by 300 composition with one shape layer of five parametric shapes; the
second has its size keyed (linear, frame 0 to 24); the third keeps a setting its type does not use
and a field this build does not know, both to be saved again unchanged.
`expected_parametric_shapes.json` gives for each case a shape, a frame, and its outline there as
the file writes a path's points, numbers within 1e-9; the saved project's records; and the refused
files, each with the diagnostic and a word its message must contain.

**This file never runs the build's code path.** Fixtures are read-only to implementation work.
"""
import copy
import json
import math
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "parametric_shapes"
KAPPA = 4 / 3 * (math.sqrt(2) - 1)


def prop(base, keys=()):
    return {"base": base, "keyframes": [{"frame": f, "value": v, "interp": "linear"} for f, v in keys]}


def at(record, frame):
    """A property record's value at a frame: linear between keys, held outside them."""
    keys = record["keyframes"]
    if not keys:
        return record["base"]
    if frame <= keys[0]["frame"]:
        return keys[0]["value"]
    for a, b in zip(keys, keys[1:]):
        if frame <= b["frame"]:
            t = (frame - a["frame"]) / (b["frame"] - a["frame"])
            if isinstance(a["value"], list):
                return [x + (y - x) * t for x, y in zip(a["value"], b["value"])]
            return a["value"] + (b["value"] - a["value"]) * t
    return keys[-1]["value"]


def pt(x, y, i=(0, 0), o=(0, 0)):
    return {"point": [x, y], "in": list(i), "out": list(o)}


def outline(p, frame):
    v = {k: at(r, frame) for k, r in p.items() if isinstance(r, dict) and "base" in r}
    x, y = v["position"]
    if p["type"] == "ellipse":
        w, h = v["size"][0] / 2, v["size"][1] / 2
        kw, kh = KAPPA * w, KAPPA * h
        return [pt(x, y - h, (-kw, 0), (kw, 0)), pt(x + w, y, (0, -kh), (0, kh)),
                pt(x, y + h, (kw, 0), (-kw, 0)), pt(x - w, y, (0, kh), (0, -kh))]
    if p["type"] == "rectangle":
        w, h = v["size"][0] / 2, v["size"][1] / 2
        r = min(w, h, v["roundness"])
        if r == 0:
            return [pt(x + w, y - h), pt(x + w, y + h), pt(x - w, y + h), pt(x - w, y - h)]
        k = KAPPA * r
        return [pt(x + w, y - h + r, (0, -k)), pt(x + w, y + h - r, (0, 0), (0, k)),
                pt(x + w - r, y + h, (k, 0)), pt(x - w + r, y + h, (0, 0), (-k, 0)),
                pt(x - w, y + h - r, (0, k)), pt(x - w, y - h + r, (0, 0), (0, -k)),
                pt(x - w + r, y - h, (-k, 0)), pt(x + w - r, y - h, (0, 0), (k, 0))]
    n = math.floor(v["points"])
    star = p["type"] == "star"
    count = 2 * n if star else n
    step = 2 * math.pi / count
    out = []
    for i in range(count):
        outer = not star or i % 2 == 0
        rad = v["outer_radius"] if outer else v["inner_radius"]
        rnd = v["outer_roundness"] if outer else v["inner_roundness"]
        a = -math.pi / 2 + math.radians(v["rotation"]) + i * step
        length = 2 * math.pi * rad / (4 * n) * rnd / 100
        tx, ty = -math.sin(a) * length, math.cos(a) * length
        out.append(pt(x + rad * math.cos(a), y + rad * math.sin(a), (-tx, -ty), (tx, ty)))
    return out


GREY = 0.21586050011389926
FILL = {"color": [GREY] * 3, "opacity": 1}
STROKE = {"color": [1, 1, 1], "opacity": 1, "width_px": 4}


def shape(name, parametric, fill=FILL, stroke=None):
    return {
        "name": name,
        "enabled": True,
        "closed": True,
        "parametric": parametric,
        "path": {"base": {"points": base_outline(parametric)}, "keyframes": []},
        "fill": fill,
        "stroke": stroke,
    }


def base_outline(p):
    bases = {k: ({"base": r["base"], "keyframes": []} if isinstance(r, dict) and "base" in r else r)
             for k, r in p.items()}
    return outline(bases, 0)


SHAPES = [
    shape("Box", {"type": "rectangle", "size": prop([120, 80]), "position": prop([100, 80]),
                  "roundness": prop(0)}),
    shape("Rounded", {"type": "rectangle", "size": prop([100, 60], [(0, [100, 60]), (24, [160, 100])]),
                      "position": prop([300, 80]), "roundness": prop(20)}),
    shape("Oval", {"type": "ellipse", "size": prop([140, 90]), "position": prop([100, 210]),
                   "roundness": prop(12), "bulge": 7}),
    shape("Star", {"type": "star", "points": prop(5), "position": prop([300, 210]), "rotation": prop(0),
                   "inner_radius": prop(30), "outer_radius": prop(60), "inner_roundness": prop(0),
                   "outer_roundness": prop(0)}, fill=None, stroke=STROKE),
    shape("Hex", {"type": "polygon", "points": prop(6.7), "position": prop([200, 150]), "rotation": prop(30),
                  "outer_radius": prop(40), "outer_roundness": prop(50)}, stroke=STROKE),
]

TRANSFORM = {
    "anchor": prop([0, 0]),
    "position": prop([0, 0]),
    "scale": prop([100, 100]),
    "rotation": prop(0),
    "opacity": prop(1),
}


def project(shapes):
    return {
        "schema_version": 0,
        "project_id": "proj-parametric-shapes",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": "comp-main",
            "name": "Main",
            "width": 400,
            "height": 300,
            "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1},
            "start_frame": 0,
            "duration_frames": 25,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 25},
            "layer_order": ["shapes"],
            "layers": [{
                "id": "shapes",
                "kind": "shape",
                "name": "shapes",
                "enabled": True,
                "locked": False,
                "in_frame": 0,
                "out_frame": 25,
                "transform": copy.deepcopy(TRANSFORM),
                "masks": [],
                "matte": None,
                "blend_mode": "normal",
                "effects": [],
                "shapes": shapes,
            }],
        }],
    }


HOST = project(SHAPES)
CASES = {}
FILES = {}


def case(n, says, **c):
    CASES[f"FX-SHPP-{n:03d}"] = {"says": says, **c}


def drawn(n, says, index, frame):
    case(n, says, shape=index, frame=frame, outline=outline(SHAPES[index]["parametric"], frame))


drawn(1, "A rectangle with a roundness of 0: four corners, top right first, clockwise.", 0, 0)
drawn(2, "A rounded rectangle at its first size key: eight points, a handle on each pointing to its corner.", 1, 0)
drawn(3, "Half way between its size keys the rectangle is half way between the two sizes; the roundness stays 20.",
      1, 12)
drawn(4, "At its last size key.", 1, 24)
drawn(5, "An ellipse: four points, top first, handles along the edge; the roundness it keeps draws nothing.", 2, 0)
drawn(6, "A five-pointed star: ten corners, outer and inner by turns, the first straight up; no roundness, no handles.",
      3, 0)
drawn(7, "A polygon of 6.7 points has six (floor), turned 30 degrees, with handles at 50% roundness.", 4, 0)

# The project saved again.
saved = []
for i, s in enumerate(SHAPES):
    saved.append({"shape": i, "type": s["parametric"]["type"], "path_base": base_outline(s["parametric"]),
                  "path_keys": 0})
case(8, "Saved again: every shape keeps its type, its path is its outline at the settings' bases with no keys, "
     "and the oval keeps the roundness its type does not use and `bulge`, a field this build does not know.",
     saved=saved, kept={"shape": 2, "roundness": SHAPES[2]["parametric"]["roundness"], "bulge": 7})


def refused(n, says, name, change, word):
    p = copy.deepcopy(SHAPES[0])
    change(p)
    FILES[name] = project([p])
    case(n, says, file=name, refused="PROJECT_SCHEMA_INVALID", names=word)


def keyed_radius(p):
    p["parametric"] = copy.deepcopy(SHAPES[3]["parametric"])
    p["parametric"]["outer_radius"] = prop(60, [(0, 60), (12, -5)])


def no_size(p):
    del p["parametric"]["size"]


def expression(p):
    p["parametric"]["position"]["expression"] = {"text": "wiggle(2, 10)", "enabled": True}


def two_points(p):
    p["parametric"] = copy.deepcopy(SHAPES[4]["parametric"])
    p["parametric"]["points"] = prop(2)


refused(20, "A parametric shape is closed; one written open is refused.", "refused_open.json",
        lambda p: p.update(closed=False), "closed")
refused(21, "A type this build does not have.", "refused_type.json",
        lambda p: p["parametric"].update(type="heart"), "heart")
refused(22, "A rectangle without its size.", "refused_no_size.json", no_size, "size")
refused(23, "A size below 0.", "refused_negative_size.json",
        lambda p: p["parametric"].update(size=prop([-10, 80])), "size")
refused(24, "A polygon of fewer than 3 points.", "refused_two_points.json", two_points, "points")
refused(25, "An expression on a setting: keys only, as a shape's other numbers.", "refused_expression.json",
        expression, "expression")
refused(26, "A key whose radius is below 0, though the base is fine.", "refused_keyed_radius.json",
        keyed_radius, "radius")


def check():
    # A rectangle's corners and an ellipse's ends are where they must be.
    box = outline(SHAPES[0]["parametric"], 0)
    assert [q["point"] for q in box] == [[160, 40], [160, 120], [40, 120], [40, 40]], box
    half = CASES["FX-SHPP-003"]["outline"]
    assert half[0]["point"] == [365, 40 + 20], half[0]
    oval = CASES["FX-SHPP-005"]["outline"]
    assert oval[0]["point"] == [100, 165] and oval[1]["point"] == [170, 210], oval
    star = CASES["FX-SHPP-006"]["outline"]
    assert len(star) == 10 and abs(star[0]["point"][0] - 300) < 1e-9 and abs(star[0]["point"][1] - 150) < 1e-9
    assert all(q["in"] == [0, 0] or max(map(abs, q["in"])) < 1e-12 for q in star)
    hexes = CASES["FX-SHPP-007"]["outline"]
    assert len(hexes) == 6
    # Every corner of the hexagon is 40 from its centre, and each out handle is at right angles
    # to the radius, clockwise.
    for q in hexes:
        dx, dy = q["point"][0] - 200, q["point"][1] - 150
        assert abs(math.hypot(dx, dy) - 40) < 1e-9
        ox, oy = q["out"]
        assert abs(dx * ox + dy * oy) < 1e-9 and dx * oy - dy * ox > 0
    for s in SHAPES:
        assert s["path"]["base"]["points"] == base_outline(s["parametric"])


def write(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n", encoding="utf-8", newline="\n")


def table():
    rows = ["| case | says | the build must give |", "| --- | --- | --- |"]
    for cid, c in CASES.items():
        if "refused" in c:
            want = f"`{c['file']}` refused, `{c['refused']}`, naming {c['names']}"
        elif "outline" in c:
            want = f"shape {c['shape']} at frame {c['frame']}: {len(c['outline'])} points as written"
        else:
            want = "each shape's type and base outline; shape 2's roundness and bulge as read"
        rows.append(f"| {cid} | {c['says']} | {want} |")
    print("\n".join(rows))


if __name__ == "__main__":
    check()
    if "--table" in sys.argv:
        table()
        raise SystemExit
    write(OUT / "host.json", HOST)
    for name, value in FILES.items():
        write(OUT / name, value)
    write(OUT / "expected_parametric_shapes.json", {"cases": CASES})
    print("checked")
