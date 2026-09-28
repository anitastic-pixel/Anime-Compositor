"""Gradient fills and strokes on shape layers, worked a second way.

D-168 proposes that a shape's fill or stroke may carry a `gradient` in place of its flat colour:
linear or radial, two to sixty-four stops of colour and opacity, and a start and an end point
that are properties like any other and so may be keyed. The colour at a pixel is worked at the
pixel's centre in the layer's space, which for a shape layer is its composition's; the fill or
stroke's coverage and its own opacity are unchanged from D-78, so the only new thing is that the
colour laid down differs from pixel to pixel.

The rule, for a pixel centre p, the start s and the end e, d = e - s:
  linear  t = ((p - s) . d) / |d|^2
  radial  t = |p - s| / |d|
  t is 1 wherever s and e coincide, and is held within 0 and 1, so the end stops pad outwards.
  Below the first stop's offset the first stop is used, at or past the last stop's the last;
  between stop i and the next, f = (t - offset_i) / (offset_i+1 - offset_i), and the two are
  mixed by f - the colours as sRGB-encoded values, as D-114's Gradient effect and After Effects
  mix them, then taken back to linear; the opacities as they are.
Stops are held in the order of their offsets, and two at the same offset keep the order they
were written in, so a pair of stops at one offset is a hard step.

**This file never runs the build's code path.** It reuses `shape_reference.py` for everything
D-78 already fixed and adds only the colour.

    python tools/shape_gradient_reference.py
"""

import json
import sys
from copy import deepcopy
from math import hypot
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import W, H, over, prop, fmt, srgb_to_linear  # noqa: E402
from mask_reference import draw_cases, linear_to_srgb, points  # noqa: E402
import shape_reference as sr  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "shape_gradients"
PICTURE = ROOT / "verification" / "B-108a proposal" / "gradient_cases.png"
KEY_PICTURE = ROOT / "verification" / "B-108a proposal" / "gradient_moving_case.png"
KEY_FRAMES = 5
TOLERANCE = 1e-6
RED = [0.8, 0.2, 0.1]    # linear; shape_reference's stroke colour
BLUE = [0.2, 0.5, 0.8]   # linear; shape_reference's fill colour
YELLOW = [0.9, 0.7, 0.05]
FULL = points([0, 0], [6, 0], [6, 2], [0, 2])


# --- the colour ---------------------------------------------------------------------------------

def value_at(point, frame):
    """A start or end point at a frame: document 20's hold and linear, a key's value held outside."""
    keys = point.get("keys")
    if not keys:
        return point["at"]
    if frame <= keys[0][0]:
        return keys[0][1]
    if frame >= keys[-1][0]:
        return keys[-1][1]
    a, b = next((a, b) for a, b in zip(keys, keys[1:]) if a[0] <= frame < b[0])
    if a[2] == "hold":
        return a[1]
    f = (frame - a[0]) / (b[0] - a[0])
    return [x + (y - x) * f for x, y in zip(a[1], b[1])]


def ordered(stops):
    return sorted(stops, key=lambda s: s["offset"])  # Python's sort is stable: a tie keeps order


def stop_at(stops, t):
    """The colour (linear) and opacity at t, from stops already in order."""
    if t < stops[0]["offset"]:
        return stops[0]["color"], stops[0]["opacity"]
    i = max(n for n, s in enumerate(stops) if s["offset"] <= t)
    if i == len(stops) - 1:
        return stops[i]["color"], stops[i]["opacity"]
    a, b = stops[i], stops[i + 1]
    f = (t - a["offset"]) / (b["offset"] - a["offset"])
    color = [srgb_to_linear(linear_to_srgb(x) + (linear_to_srgb(y) - linear_to_srgb(x)) * f)
             for x, y in zip(a["color"], b["color"])]
    return color, a["opacity"] + (b["opacity"] - a["opacity"]) * f


def colours(gradient, frame):
    """Every pixel's (colour, opacity) for one gradient at one frame."""
    s = value_at(gradient["start"], frame)
    e = value_at(gradient["end"], frame)
    dx, dy = e[0] - s[0], e[1] - s[1]
    run = dx * dx + dy * dy
    stops = ordered(gradient["stops"])
    out = []
    for y in range(H):
        for x in range(W):
            px, py = x + 0.5 - s[0], y + 0.5 - s[1]
            if run == 0:
                t = 1.0
            elif gradient["type"] == "linear":
                t = (px * dx + py * dy) / run
            else:
                t = hypot(px, py) / run ** 0.5
            out.append(stop_at(stops, max(0.0, min(1.0, t))))
    return out


def paint(picture, field, paint_record, frame):
    """D-78's paint, with the colour and a stop opacity per pixel when there is a gradient."""
    g = paint_record.get("gradient")
    if not g:
        return sr.paint(picture, field, paint_record["color"], paint_record["opacity"])
    out = []
    for cov, dst, (color, opacity) in zip(field, picture, colours(g, frame)):
        a = cov * paint_record["opacity"] * opacity
        out.append(over([color[0] * a, color[1] * a, color[2] * a, a], dst))
    return out


def layer_picture(layer, frame_no):
    picture = [[0.0] * 4 for _ in range(W * H)]
    for shape in layer["shapes"]:
        pts = shape["points"]
        closed = shape.get("closed", True)
        poly = sr.flatten(pts, closed)
        if shape.get("fill"):
            picture = paint(picture, sr.fill_field(poly), shape["fill"], frame_no)
        if shape.get("stroke"):
            picture = paint(picture, sr.stroke_field(poly, closed, shape["stroke"]["width_px"]),
                            shape["stroke"], frame_no)
    return picture


def render(case, frame_no=0):
    frame = [[0.0] * 4 for _ in range(W * H)]
    for layer in case["layers"]:
        frame = [over(s, d) for s, d in zip(layer_picture(layer, frame_no), frame)]
    return frame


# --- the project files --------------------------------------------------------------------------

def point_json(point):
    record = prop(point["keys"][0][1] if point.get("keys") else point["at"])
    for frame, value, interp in point.get("keys", []):
        record["keyframes"].append({"frame": frame, "value": value, "interp": interp})
    return record


def paint_json(paint_record):
    if paint_record is None:
        return None
    record = {k: v for k, v in paint_record.items() if k != "gradient"}
    g = paint_record.get("gradient")
    if g:
        record["gradient"] = {"type": g["type"], "start": point_json(g["start"]),
                              "end": point_json(g["end"]), "stops": g["stops"]}
    return record


def project_json(name, case):
    project = sr.project_json(name, {**case, "layers": [
        {**l, "shapes": [{k: v for k, v in s.items() if k not in ("fill", "stroke")}
                         for s in l["shapes"]]} for l in case["layers"]]})
    for record, layer in zip(project["compositions"][0]["layers"], case["layers"]):
        for shape_record, shape in zip(record["shapes"], layer["shapes"]):
            shape_record["fill"] = paint_json(shape.get("fill"))
            shape_record["stroke"] = paint_json(shape.get("stroke"))
    return project


# --- the cases ----------------------------------------------------------------------------------

def stop(offset, color, opacity=1.0):
    return {"offset": offset, "color": list(color), "opacity": opacity}


def gradient(kind, start, end, *stops):
    def point(p):
        return {"keys": p} if isinstance(p[0], tuple) else {"at": list(p)}
    return {"type": kind, "start": point(start), "end": point(end),
            "stops": list(stops) or [stop(0, RED), stop(1, BLUE)]}


def fill(g, opacity=1.0, color=None):
    return {"color": list(color or RED), "opacity": opacity, "gradient": g}


def one(shape, frames=None):
    case = {"layers": [{"id": "shapes", "kind": "shape", "shapes": [shape]}]}
    if frames:
        case["frames"] = frames
    return case


ACROSS = gradient("linear", [0, 1], [6, 1])

CASES = {
    "FX-SHP-040": ("A linear gradient, red at the left edge to blue at the right, filling the "
                   "frame: every column a step further from red to blue, mixed as encoded colour.",
                   one(sr.shape(FULL, fill=fill(ACROSS)))),
    "FX-SHP-041": ("The same stops between x = 1 and x = 5: column 0 is red and column 5 blue "
                   "exactly, because the ends pad outwards.",
                   one(sr.shape(FULL, fill=fill(gradient("linear", [1, 1], [5, 1]))))),
    "FX-SHP-042": ("A radial gradient from the frame's middle (3, 1) out to (6, 1): the two "
                   "middle columns nearly red and the colour turning blue with the distance, the "
                   "same on each side.",
                   one(sr.shape(FULL, fill=fill(gradient("radial", [3, 1], [6, 1]))))),
    "FX-SHP-043": ("Four stops, red and red to 0.5 then blue and blue: two stops at one offset "
                   "are a hard step, so the left three columns are red and the right three blue.",
                   one(sr.shape(FULL, fill=fill(gradient(
                       "linear", [0, 1], [6, 1],
                       stop(0, RED), stop(0.5, RED), stop(0.5, BLUE), stop(1, BLUE)))))),
    "FX-SHP-044": ("Three stops, red, yellow at 0.5, blue: the middle of the frame passes "
                   "through yellow.",
                   one(sr.shape(FULL, fill=fill(gradient(
                       "linear", [0, 1], [6, 1],
                       stop(0, RED), stop(0.5, YELLOW), stop(1, BLUE)))))),
    "FX-SHP-045": ("A stop's opacity: blue fully there at the left, fading to nothing at the "
                   "right.",
                   one(sr.shape(FULL, fill=fill(gradient(
                       "linear", [0, 1], [6, 1], stop(0, BLUE), stop(1, BLUE, 0.0)))))),
    "FX-SHP-046": ("The fill's own opacity at 50% still halves the gradient: FX-SHP-040 at half "
                   "strength.",
                   one(sr.shape(FULL, fill=fill(ACROSS, opacity=0.5)))),
    "FX-SHP-047": ("Start and end at the same point: the whole fill is the last stop's colour, "
                   "the flat blue of a fill with no gradient.",
                   one(sr.shape(FULL, fill=fill(gradient("linear", [3, 1], [3, 1]))))),
    "FX-SHP-048": ("FX-SHP-040's stops written blue first: stops are taken in the order of their "
                   "offsets, so the frame is FX-SHP-040's.",
                   one(sr.shape(FULL, fill=fill(gradient(
                       "linear", [0, 1], [6, 1], stop(1, BLUE), stop(0, RED)))))),
    "FX-SHP-049": ("A diagonal: from the top-left corner (0, 0) to the bottom-right (6, 2), so "
                   "the bottom row is further along than the top.",
                   one(sr.shape(FULL, fill=fill(gradient("linear", [0, 0], [6, 2]))))),
    "FX-SHP-050": ("A gradient stroke, one pixel wide, on a rectangle taller than the frame: "
                   "the stroke's two upright bands take the gradient's colour where they stand, "
                   "red on the left band and nearer blue on the right, with no fill between.",
                   one(sr.shape(sr.TALL, stroke={
                       "color": list(RED), "opacity": 1.0, "width_px": 1.0,
                       "gradient": gradient("linear", [0, 1], [6, 1])}))),
    "FX-SHP-051": ("The start keyed from the left edge to the right and the end from the right "
                   "to the left, over four frames, linear: the gradient turns round, and at frame "
                   "2, where the two meet, the fill is flat blue.",
                   one(sr.shape(FULL, fill=fill(gradient(
                       "linear", ((0, [0, 1], "linear"), (4, [6, 1], "linear")),
                       ((0, [6, 1], "linear"), (4, [0, 1], "linear"))))), KEY_FRAMES)),
}

KEYED = ["FX-SHP-051"]


def refusals():
    """Files a build must refuse whole, as PROJECT_SCHEMA_INVALID, each a change to FX-SHP-040."""
    def edit(change):
        p = deepcopy(project_json("FX-SHP-040", CASES["FX-SHP-040"][1]))
        change(p["compositions"][0]["layers"][0]["shapes"][0]["fill"]["gradient"])
        return p

    def put(key, value):
        return lambda g: g.__setitem__(key, value)

    def put_stop(key, value):
        return lambda g: g["stops"][0].__setitem__(key, value)

    return {
        "FX-SHP-060": ("A gradient of one stop.", edit(lambda g: g["stops"].pop())),
        "FX-SHP-061": ("A gradient of 65 stops, past the limit of 64.",
                       edit(put("stops", [stop(n / 64, RED) for n in range(65)]))),
        "FX-SHP-062": ("A stop offset above 1.", edit(put_stop("offset", 1.5))),
        "FX-SHP-063": ("A stop colour above 1.", edit(put_stop("color", [0.2, 1.5, 0.8]))),
        "FX-SHP-064": ("A stop opacity below 0.", edit(put_stop("opacity", -0.5))),
        "FX-SHP-065": ("A gradient type that is neither linear nor radial.",
                       edit(put("type", "conic"))),
        "FX-SHP-066": ("A start point of one number rather than two.",
                       edit(put("start", prop(3)))),
        "FX-SHP-067": ("An expression on the end point.",
                       edit(lambda g: g["end"].__setitem__(
                           "expression", {"text": "[6, 1]", "enabled": True}))),
        "FX-SHP-068": ("Motion-path handles on a key of the start point.",
                       edit(put("start", {"base": [0, 1], "keyframes": [
                           {"frame": 0, "value": [0, 1], "interp": "linear",
                            "spatial": [1, 0, 1, 0]},
                           {"frame": 2, "value": [3, 1], "interp": "linear"}]}))),
        "FX-SHP-069": ("A gradient with no stops key.", edit(lambda g: g.pop("stops"))),
    }


# --- running ------------------------------------------------------------------------------------

def main():
    OUT.mkdir(parents=True, exist_ok=True)
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}, "refused": {}}
    drawn = {}
    for fx, (says, case) in CASES.items():
        wanted = range(KEY_FRAMES) if fx in KEYED else [0]
        frames = {str(f): render(case, f) for f in wanted}
        drawn[fx] = frames
        project = f"{fx.lower().replace('-', '_')}.json"
        (OUT / project).write_text(json.dumps(project_json(fx, case), indent=2) + "\n",
                                   encoding="utf-8")
        expected["cases"][fx] = {"says": says, "project": project, "frames": frames}

        print(f"{fx}: {says}\n")
        print("| frame, row | " + " | ".join(f"x = {x}" for x in range(W)) + " |")
        print("| --- " * (W + 1) + "|")
        for f, frame in frames.items():
            for y in range(H):
                cells = (" ".join(fmt(v) for v in frame[y * W + x]) for x in range(W))
                print(f"| {f}, {y} | " + " | ".join(cells) + " |")
        print()

    print("Refused whole, each as `PROJECT_SCHEMA_INVALID`; each is FX-SHP-040's file with one "
          "change:\n")
    for fx, (says, project) in refusals().items():
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(project, indent=2) + "\n", encoding="utf-8")
        expected["refused"][fx] = {"says": says, "project": name, "code": "PROJECT_SCHEMA_INVALID"}
        print(f"- {fx}: {says}")
    print()

    (OUT / "expected_shape_gradients.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                       encoding="utf-8")
    draw_cases({fx: [f["0"]] for fx, f in drawn.items() if fx not in KEYED}, PICTURE)
    draw_cases({fx: [drawn[fx][str(n)] for n in range(KEY_FRAMES)] for fx in KEYED}, KEY_PICTURE)
    print(f"Drawn: {PICTURE}\nDrawn: {KEY_PICTURE}")

    # The claims the cases make, checked on the numbers just worked.
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}

    def near(a, b):
        return all(abs(x - y) < 1e-12 for p, q in zip(a, b) for x, y in zip(p, q))

    red, blue = [*RED, 1.0], [*BLUE, 1.0]
    forty = c["FX-SHP-040"]
    # Every column further from red than the one before, and both rows alike.
    assert all(forty[x][0] > forty[x + 1][0] and forty[x][2] < forty[x + 1][2] for x in range(5))
    assert forty[:W] == forty[W:]
    # Mixed as encoded colour: the middle of the frame is half way in sRGB, not in linear light.
    mid = srgb_to_linear((linear_to_srgb(RED[0]) + linear_to_srgb(BLUE[0])) / 2)
    left, right = forty[2][0], forty[3][0]
    assert right < mid < left and abs((left + right) / 2 - mid) < 0.02
    assert near([c["FX-SHP-041"][0], c["FX-SHP-041"][5]], [red, blue])
    radial = c["FX-SHP-042"]
    assert radial[2] == radial[3] and radial[0] == radial[5] and radial[1] == radial[4]
    assert radial[2][0] > radial[1][0] > radial[0][0]
    assert near(c["FX-SHP-043"][0:3], [red] * 3) and near(c["FX-SHP-043"][3:6], [blue] * 3)
    yellow_ish = c["FX-SHP-044"]
    assert yellow_ish[2][1] > forty[2][1] and yellow_ish[3][1] > forty[3][1]
    fade = c["FX-SHP-045"]
    assert all(fade[x][3] > fade[x + 1][3] for x in range(5)) and fade[0][3] < 1
    assert near(c["FX-SHP-046"], [[v * 0.5 for v in p] for p in forty])
    assert all(p == blue for p in c["FX-SHP-047"])
    assert c["FX-SHP-048"] == forty
    diag = c["FX-SHP-049"]
    assert all(diag[W + x][2] > diag[x][2] for x in range(W))
    band = c["FX-SHP-050"]
    assert [round(p[3], 12) for p in band[:W]] == [0.5, 0, 0.5, 0.5, 0, 0]
    assert band[0][0] > band[3][0] and band[0][2] < band[3][2]
    at = expected["cases"]["FX-SHP-051"]["frames"]
    assert at["0"] == forty and all(p == blue for p in at["2"])
    assert near(at["4"], [forty[y * W + (W - 1 - x)] for y in range(H) for x in range(W)])


if __name__ == "__main__":
    main()
