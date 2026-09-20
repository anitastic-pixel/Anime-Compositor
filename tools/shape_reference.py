"""Shape layers, worked a second way.

D-78 proposes a layer whose drawing is a list of shapes it carries rather than a file on disk:
each a path of points that may carry handles, open or closed, with an optional fill and an
optional stroke. The path is D-77's path record unchanged, so a shape is drawn with the pen, the
rectangle and the ellipse the mask tools already have, flattened by the same rule and moved
between keys by the same rule. What is new here is the open path, the fill, and the stroke - and
a stroke is every point within half its width of the path, which is what makes its joins and its
caps round without naming a single one of them.

**This file never runs the build's code path.** It renders each tiny frame pixel by pixel from
document 21 and ADR-016, and the outline sampler here is written from the rule rather than copied
from `src/`. It shares the parts of the rule that D-77 already fixed with `mask_reference.py`,
which is that file's second implementation and not the build's.

Every case is a composition 6 pixels by 2, so each frame is twelve pixels and can be printed. A
case whose path moves is five frames long and every one of its frames is pinned. The one drawing
is written into `Fixtures/shapes/media`, the projects into `Fixtures/shapes`, the expected frames
and refusals into `Fixtures/shapes/expected_shapes.json`, a picture of every still case into
`verification/B-25a proposal/shape_cases.png` and of the moving one, frame by frame across the
row, into `verification/B-25a proposal/shape_moving_case.png`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/shape_reference.py
"""

import json
import sys
from copy import deepcopy
from math import ceil, hypot
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import W, H, png, decoded, over, prop, fmt  # noqa: E402
from mask_reference import (  # noqa: E402
    K, SAMPLES_PER_SIDE, coverage, draw_cases, mask_json, point_inside, points, points_at,
)

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "shapes"
ROOT = Path(__file__).resolve().parent.parent
PICTURE = ROOT / "verification" / "B-25a proposal" / "shape_cases.png"
KEY_PICTURE = ROOT / "verification" / "B-25a proposal" / "shape_moving_case.png"
KEY_FRAMES = 5  # a case with a moving path is five frames long
TOLERANCE = 1e-6
BG_PIXELS = [[(255, 0, 0, 255)] * W, [(128, 128, 128, 255)] * W]  # adjust_reference's "bg"
FILL = [0.2, 0.5, 0.8]    # linear; a sky blue on screen, and no channel 0 or 1
STROKE = [0.8, 0.2, 0.1]  # linear; plainly not the fill, in every channel


# --- the outline ------------------------------------------------------------------------------

def flatten(pts, closed):
    """The path as a list of (x, y): D-77's flattening, with the one difference an open path makes.

    A segment whose two handles are both zero stays one straight line. A curved segment is split
    into `n` straight pieces at equal steps of t, n the control polygon's length in pixels halved,
    rounded up, held between 16 and 512, the length summed point, out handle, in handle, point.
    The only new thing is `closed`: an open path has no segment from its last point back to its
    first, so its last point is the end of the line rather than a corner.
    """
    out = []
    n = len(pts)
    for i in range(n if closed else n - 1):
        p0, p1 = pts[i], pts[(i + 1) % n]
        a = tuple(p0["point"])
        b = (a[0] + p0["out"][0], a[1] + p0["out"][1])
        d = tuple(p1["point"])
        c = (d[0] + p1["in"][0], d[1] + p1["in"][1])
        out.append(a)
        if p0["out"] == [0, 0] and p1["in"] == [0, 0]:
            continue
        length = hypot(b[0] - a[0], b[1] - a[1])
        length += hypot(c[0] - b[0], c[1] - b[1])
        length += hypot(d[0] - c[0], d[1] - c[1])
        steps = min(512, max(16, ceil(length / 2)))
        for k in range(1, steps):
            t = k / steps
            u = 1 - t
            out.append(tuple(u ** 3 * a[j] + 3 * u * u * t * b[j] + 3 * u * t * t * c[j]
                             + t ** 3 * d[j] for j in range(2)))
    if not closed:
        out.append(tuple(pts[-1]["point"]))
    return out


def distance_to_path(poly, closed, x, y):
    """The shortest distance from (x, y) to the path itself.

    An open path is the pieces between its points and no more, which is the whole of why the ends
    of a stroke come out round: the nearest thing to a sample beyond the end of the line is the
    end point, so the band ends in a half circle of the stroke's own radius.
    """
    segments = list(zip(poly, poly[1:]))
    if closed and len(poly) > 1:
        segments.append((poly[-1], poly[0]))
    if not segments:
        return hypot(x - poly[0][0], y - poly[0][1])
    best = float("inf")
    for (x0, y0), (x1, y1) in segments:
        dx, dy = x1 - x0, y1 - y0
        run = dx * dx + dy * dy
        t = 0.0 if run == 0 else max(0.0, min(1.0, ((x - x0) * dx + (y - y0) * dy) / run))
        best = min(best, hypot(x - (x0 + t * dx), y - (y0 + t * dy)))
    return best


def pixel_coverage(inside, x, y):
    """ADR-016, unchanged: a 4x4 ordered grid, coverage the count inside over sixteen."""
    n = SAMPLES_PER_SIDE
    hits = sum(1 for j in range(n) for i in range(n)
               if inside(x + (i + 0.5) / n, y + (j + 0.5) / n))
    return hits / (n * n)


def fill_field(poly):
    """Where the fill is: the even-odd rule, an open path counted as closed for this purpose."""
    return [pixel_coverage(lambda sx, sy: point_inside(poly, sx, sy), x, y)
            for y in range(H) for x in range(W)]


def stroke_field(poly, closed, width_px):
    """Where the stroke is: within half a width of the path, which is a round join and a round cap."""
    radius = width_px / 2
    return [pixel_coverage(lambda sx, sy: distance_to_path(poly, closed, sx, sy) <= radius, x, y)
            for y in range(H) for x in range(W)]


# --- the frame --------------------------------------------------------------------------------

def paint(picture, field, color, opacity):
    """One coverage field of one colour, laid over the layer's own picture by document 21's normal."""
    out = []
    for cov, dst in zip(field, picture):
        a = cov * opacity
        out.append(over([color[0] * a, color[1] * a, color[2] * a, a], dst))
    return out


def takes_part(shape, pts):
    """A shape this build draws: switched on, and with at least two points to draw between."""
    return shape.get("enabled", True) and len(pts) >= 2


def layer_picture(layer, frame_no):
    """Document 21 step 1 for a shape layer, then step 2: the shapes, then the masks.

    Step 1 starts transparent black - the shape layer's space is its composition's - and each
    shape is laid into it in the order it was drawn, first to last, so a later shape covers an
    earlier one. Within one shape the fill goes down first and its stroke over it.
    """
    picture = [[0.0] * 4 for _ in range(W * H)]
    for shape in layer.get("shapes", []):
        pts = points_at(shape, frame_no)
        if not takes_part(shape, pts):
            continue
        poly = flatten(pts, shape.get("closed", True))
        fill = shape.get("fill")
        if fill:
            picture = paint(picture, fill_field(poly), fill["color"], fill["opacity"])
        stroke = shape.get("stroke")
        if stroke:
            picture = paint(picture, stroke_field(poly, shape.get("closed", True),
                                                  stroke["width_px"]),
                            stroke["color"], stroke["opacity"])
    acc = coverage(layer.get("masks", []), frame_no)
    if acc is not None:
        picture = [[v * m for v in p] for p, m in zip(picture, acc)]
    return picture


def render(case, frame_no=0):
    frame = [[0.0] * 4 for _ in range(W * H)]
    for layer in case["layers"]:
        src = decoded("bg") if layer["kind"] == "raster" else layer_picture(layer, frame_no)
        frame = [over(s, d) for s, d in zip(src, frame)]
    return frame


# --- the project files --------------------------------------------------------------------------

def path_json(shape):
    """The path as document 19 writes a property, exactly as a mask's path is written."""
    record = prop({"points": shape["points"]})
    for frame, outline, interp in shape.get("keys", []):
        record["keyframes"].append({"frame": frame, "value": {"points": outline},
                                    "interp": interp})
    return record


def shape_json(shape, n):
    return {
        "name": shape.get("name", f"Shape {n}"),
        "enabled": shape.get("enabled", True),
        "closed": shape.get("closed", True),
        "path": path_json(shape),
        "fill": shape.get("fill"),
        "stroke": shape.get("stroke"),
    }


def layer_json(layer, frames):
    record = {"id": layer["id"], "kind": layer["kind"], "name": layer["id"]}
    if layer["kind"] == "raster":
        record.update({"asset_id": "asset-bg"})
    record.update({"enabled": True, "locked": False, "in_frame": 0, "out_frame": frames})
    if layer["kind"] == "raster":
        record["source_offset_frames"] = 0
    else:
        # A shape layer has no asset, no exposures and no source offset, and no size of its own:
        # its space is the composition's.
        record["shapes"] = [shape_json(s, n + 1) for n, s in enumerate(layer.get("shapes", []))]
    record["transform"] = {
        "anchor": prop([W / 2, H / 2]), "position": prop([W / 2, H / 2]),
        "scale": prop([100, 100]), "rotation": prop(0), "opacity": prop(1),
    }
    if layer["kind"] == "raster":
        record["exposure_spans"] = []
    record["masks"] = [mask_json(m, n + 1) for n, m in enumerate(layer.get("masks", []))]
    record.update({"matte": None, "blend_mode": "normal", "effects": []})
    return record


def project_json(name, case):
    frames = case.get("frames", 3)
    raster = any(l["kind"] == "raster" for l in case["layers"])
    return {
        "schema_version": 0,
        "project_id": "proj-" + name.lower(),
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": ([{"id": "asset-bg", "kind": "still", "name": "bg", "path": "media/bg.png",
                     "interpretation": {"color_space": "srgb", "alpha": "straight"}}]
                   if raster else []),
        "compositions": [{
            "id": "comp-main", "name": "Main", "width": W, "height": H,
            "pixel_aspect_ratio": 1, "frame_rate": {"numerator": 24, "denominator": 1},
            "start_frame": 0, "duration_frames": frames,
            "work_area": {"start_frame": 0, "end_frame_exclusive": frames},
            # layer_order is bottom first here, as the cases are written.
            "layer_order": [l["id"] for l in case["layers"]],
            "layers": [layer_json(l, frames) for l in case["layers"]],
        }],
    }


# --- the cases --------------------------------------------------------------------------------

LEFT3 = points([0, 0], [3, 0], [3, 2], [0, 2])            # columns 0 to 2, on pixel boundaries
HALF = points([0, 0], [2.5, 0], [2.5, 2], [0, 2])         # an edge down the middle of column 2
TRIANGLE = points([0, 0], [4, 0], [0, 2])                 # a sloped edge, partial coverage
ELLIPSE = points([3, 0], [4, 1], [3, 2], [2, 1], handles=[
    ([-K, 0], [K, 0]), ([0, -K], [0, K]), ([K, 0], [-K, 0]), ([0, K], [0, -K])])
TALL = points([0, -1], [3, -1], [3, 3], [0, 3])           # the same box, taller than the frame,
#                                                           so a stroke shows its upright edges
#                                                           alone and the frame is not all band
LINE = points([1, 1], [5, 1])                             # an open path: one straight piece
ARCH = points([1, 1.5], [3, 0.5], [5, 1.5])               # open, three points, a shallow vee
BOWTIE = points([0, 0], [6, 2], [6, 0], [0, 2])           # crosses itself in the middle
RIGHT3 = points([3, 0], [6, 0], [6, 2], [3, 2])           # where a moving path arrives


def fill(color=None, opacity=1.0):
    return {"color": list(color or FILL), "opacity": opacity}


def stroke(color=None, opacity=1.0, width=1.0):
    return {"color": list(color or STROKE), "opacity": opacity, "width_px": width}


def shape(pts, **kw):
    return {"points": pts, **kw}


def shapes(*items, masks=None):
    layer = {"id": "shapes", "kind": "shape", "shapes": list(items)}
    if masks:
        layer["masks"] = list(masks)
    return {"layers": [layer]}


BG = {"id": "bg", "kind": "raster"}

CASES = {
    "FX-SHP-001": ("A filled rectangle on the left three columns, its edges on pixel boundaries: "
                   "the same sixteen decisions a mask's edge is made of.",
                   shapes(shape(LEFT3, fill=fill()))),
    "FX-SHP-002": ("The same rectangle ending half way through column 2: that column is exactly "
                   "half covered, which is the edge quantum ADR-016 states.",
                   shapes(shape(HALF, fill=fill()))),
    "FX-SHP-003": ("A sloped edge: coverage in whole sixteenths.",
                   shapes(shape(TRIANGLE, fill=fill()))),
    "FX-SHP-004": ("An ellipse of four curved segments, filling the frame's height.",
                   shapes(shape(ELLIPSE, fill=fill()))),
    "FX-SHP-005": ("The same rectangle at half fill opacity: half there, and nothing else "
                   "changed.",
                   shapes(shape(LEFT3, fill=fill(opacity=0.5)))),
    "FX-SHP-006": ("A stroke and no fill, one pixel wide, on a rectangle taller than the frame: "
                   "a band half a pixel either side of each upright edge, and the middle of the "
                   "rectangle empty, because a stroke is on the line and not within it.",
                   shapes(shape(TALL, stroke=stroke()))),
    "FX-SHP-007": ("A stroke on an open path of two points, one pixel wide: the band runs the "
                   "length of the line and ends in a half circle at each end, because a cap is "
                   "round by definition.",
                   shapes(shape(LINE, closed=False, stroke=stroke()))),
    "FX-SHP-008": ("Fill and stroke together on that rectangle: the stroke is laid over its own "
                   "fill, so column 1 is the fill's colour alone, column 3 the stroke's alone, "
                   "and the columns they share carry the stroke over the fill.",
                   shapes(shape(TALL, fill=fill(), stroke=stroke()))),
    "FX-SHP-009": ("Two shapes, the second covering the first: the list is drawn first to last.",
                   shapes(shape(LEFT3, fill=fill()), shape(ELLIPSE, fill=fill(STROKE)))),
    "FX-SHP-010": ("A shape layer over the drawing: the drawing shows everywhere the shape does "
                   "not, and the shape is opaque where it does.",
                   {"layers": [BG, shapes(shape(LEFT3, fill=fill()))["layers"][0]]}),
    "FX-SHP-011": ("An open path with both a fill and a stroke: the fill closes it with a "
                   "straight line from its last point to its first, while the stroke does not "
                   "run along that line.",
                   shapes(shape(ARCH, closed=False, fill=fill(), stroke=stroke()))),
    "FX-SHP-012": ("A path that crosses itself, filled: the even-odd rule says what is inside, "
                   "and a shape is drawn rather than diagnosed for it, which is where a shape "
                   "and a mask part company.",
                   shapes(shape(BOWTIE, fill=fill()))),
    "FX-SHP-013": ("A mask on the shape layer, keeping its left three columns: a shape layer is "
                   "masked like any other, after its shapes are drawn - the frame of FX-SHP-001.",
                   shapes(shape(points([0, 0], [6, 0], [6, 2], [0, 2]), fill=fill()),
                          masks=[{"points": LEFT3}])),
    "FX-SHP-014": ("A shape with neither a fill nor a stroke: nothing is drawn and nothing is "
                   "said, because a path a person has not decided about yet is not a fault.",
                   shapes(shape(LEFT3))),
    "FX-SHP-015": ("A shape switched off takes no part: the empty frame.",
                   shapes(shape(LEFT3, fill=fill(), enabled=False))),
    "FX-SHP-016": ("A shape of one point is kept, diagnosed and draws nothing: there is nothing "
                   "to fill and nothing to stroke between.",
                   shapes(shape(points([3, 1]), fill=fill(), stroke=stroke()))),
    "FX-SHP-017": ("A path keyed from the left three columns at frame 0 to the right three at "
                   "frame 4, linear: the fill slides three columns in four frames, three quarters "
                   "of a column a frame, by D-77's rule and document 20's.",
                   {**shapes(shape(LEFT3, fill=fill(),
                                   keys=[(0, LEFT3, "linear"), (4, RIGHT3, "linear")])),
                    "frames": KEY_FRAMES}),
}

KEYED = ["FX-SHP-017"]

DIAGNOSED = {"FX-SHP-016": "SHAPE_INVALID_OUTLINE"}


def refusals():
    """Files a build must refuse whole, as PROJECT_SCHEMA_INVALID, each a change to FX-SHP-001."""
    def edit(change):
        # Deep-copied, or one refusal's damage would still be in the next one's file.
        p = deepcopy(project_json("FX-SHP-001", CASES["FX-SHP-001"][1]))
        change(p["compositions"][0]["layers"][0])
        return p

    def put(key, value):
        return lambda l: l.__setitem__(key, value)

    def put_fill(key, value):
        return lambda l: l["shapes"][0]["fill"].__setitem__(key, value)

    return {
        "FX-SHP-020": ("A shape layer that names an asset.", edit(put("asset_id", "asset-bg"))),
        "FX-SHP-021": ("A shape layer with exposures.", edit(put("exposure_spans", []))),
        "FX-SHP-022": ("A shape layer with a source offset.", edit(put("source_offset_frames", 0))),
        "FX-SHP-023": ("A `shapes` key that is not a list.",
                       edit(lambda l: l.__setitem__("shapes", l["shapes"][0]))),
        "FX-SHP-024": ("A shape with no path.", edit(lambda l: l["shapes"][0].pop("path"))),
        "FX-SHP-025": ("A `closed` that is not true or false.",
                       edit(lambda l: l["shapes"][0].__setitem__("closed", "yes"))),
        "FX-SHP-026": ("A fill colour above 1.", edit(put_fill("color", [0.2, 1.5, 0.8]))),
        "FX-SHP-027": ("A fill opacity below 0.", edit(put_fill("opacity", -0.5))),
        "FX-SHP-028": ("A stroke width of 0.",
                       edit(lambda l: l["shapes"][0].__setitem__("stroke", stroke(width=0)))),
        "FX-SHP-029": ("A stroke width past the 8192 pixel limit.",
                       edit(lambda l: l["shapes"][0].__setitem__("stroke", stroke(width=8193)))),
        "FX-SHP-030": ("A raster layer carrying a `shapes` key.",
                       edit(lambda l: [l.update(kind="raster", asset_id="asset-bg",
                                                source_offset_frames=0, exposure_spans=[])])),
    }


# --- running ------------------------------------------------------------------------------------

def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    (OUT / "media" / "bg.png").write_bytes(png(BG_PIXELS))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}, "refused": {}}
    drawn = {}
    for fx, (says, case) in CASES.items():
        wanted = range(KEY_FRAMES) if fx in KEYED else [0]
        frames = {str(f): render(case, f) for f in wanted}
        drawn[fx] = frames
        project = f"{fx.lower().replace('-', '_')}.json"
        (OUT / project).write_text(json.dumps(project_json(fx, case), indent=2) + "\n",
                                   encoding="utf-8")
        record = {"says": says, "project": project, "frames": frames}
        if fx in DIAGNOSED:
            record["diagnostic"] = DIAGNOSED[fx]
        expected["cases"][fx] = record

        print(f"{fx}: {says}\n")
        print("| frame, row | " + " | ".join(f"x = {x}" for x in range(W)) + " |")
        print("| --- " * (W + 1) + "|")
        for f, frame in frames.items():
            for y in range(H):
                cells = (" ".join(fmt(v) for v in frame[y * W + x]) for x in range(W))
                print(f"| {f}, {y} | " + " | ".join(cells) + " |")
        print()

    print("Refused whole, each as `PROJECT_SCHEMA_INVALID`; each is FX-SHP-001's file with one "
          "change:\n")
    for fx, (says, project) in refusals().items():
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(project, indent=2) + "\n", encoding="utf-8")
        expected["refused"][fx] = {"says": says, "project": name, "code": "PROJECT_SCHEMA_INVALID"}
        print(f"- {fx}: {says}")
    print()

    (OUT / "expected_shapes.json").write_text(json.dumps(expected, indent=1) + "\n",
                                              encoding="utf-8")
    draw_cases({fx: [f["0"]] for fx, f in drawn.items() if fx not in KEYED}, PICTURE)
    draw_cases({fx: [drawn[fx][str(n)] for n in range(KEY_FRAMES)] for fx in KEYED}, KEY_PICTURE)
    print(f"Drawn: {PICTURE}\nDrawn: {KEY_PICTURE}")

    # The claims the cases are there to make, checked on the numbers just worked.
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    blank = [[0.0] * 4] * (W * H)
    opaque_fill = [*FILL, 1.0]
    # A fill on pixel boundaries is whole pixels of its colour, and nothing outside it.
    assert all(p == opaque_fill for p in c["FX-SHP-001"][:3] + c["FX-SHP-001"][6:9])
    assert c["FX-SHP-001"][3:6] == blank[3:6]
    # The edge quantum: half a pixel of coverage is half the colour, premultiplied.
    assert [round(v, 12) for v in c["FX-SHP-002"][2]] == [v * 0.5 for v in opaque_fill]
    assert all(abs(p[3] * 16 - round(p[3] * 16)) < 1e-12 for p in c["FX-SHP-003"])
    # An ellipse is round: it reaches the middle columns and not the corners.
    assert c["FX-SHP-004"][0][3] == 0 and c["FX-SHP-004"][3][3] > 0
    assert c["FX-SHP-005"] == [[v * 0.5 for v in p] for p in c["FX-SHP-001"]]
    # A stroke is a band on the line: half a pixel either side of each upright edge, which is
    # half of column 0 and half of each of columns 2 and 3, and nothing in between or beyond.
    six = c["FX-SHP-006"]
    assert [round(p[3], 12) for p in six[:W]] == [0.5, 0, 0.5, 0.5, 0, 0]
    assert six[0][:3] == [v * 0.5 for v in STROKE]
    # A round cap reaches half a width past the end of the line, so column 0 is partly covered
    # although the line starts at x = 1.
    seven = c["FX-SHP-007"]
    assert seven[0][3] > 0 and seven[5][3] > 0
    assert seven[1][3] > seven[0][3] and seven[4][3] > seven[5][3]
    # Fill and stroke together: the fill alone in the middle, the stroke alone outside the fill,
    # and the stroke laid over the fill where both are.
    eight = c["FX-SHP-008"]
    assert eight[1] == opaque_fill                                  # the fill, untouched
    assert [round(v, 12) for v in eight[3]] == [v * 0.5 for v in [*STROKE, 1.0]]   # the stroke
    assert eight[0][0] / eight[0][3] > eight[1][0] / eight[1][3]    # redder where both are
    assert all(a[3] >= b[3] for a, b in zip(eight, c["FX-SHP-006"]))   # and it covers no less
    # A later shape covers an earlier one: the ellipse's colour is on top in the middle column.
    assert c["FX-SHP-009"][2][0] > c["FX-SHP-001"][2][0]
    # Over the drawing: the shape where it is, the drawing where it is not.
    bg = decoded("bg")
    assert c["FX-SHP-010"][:3] == c["FX-SHP-001"][:3] and c["FX-SHP-010"][3:6] == bg[3:6]
    # An open path is closed for its fill and open for its stroke: the straight line home is
    # filled under, and not stroked over.
    eleven = c["FX-SHP-011"]
    assert eleven[3][3] > 0                        # the vee holds a filled area
    assert eleven[0][3] == 0 and eleven[5][3] == 0  # and neither reaches the frame's ends
    # A crossing shape is drawn: the even-odd rule leaves the two triangles and not the crossing.
    assert c["FX-SHP-012"][0][3] > 0 and c["FX-SHP-012"][5][3] > 0
    # A mask on a shape layer cuts it exactly as it cuts a drawing.
    assert c["FX-SHP-013"] == c["FX-SHP-001"]
    # Nothing to draw, in three different ways, is the empty frame.
    assert c["FX-SHP-014"] == c["FX-SHP-015"] == c["FX-SHP-016"] == blank

    # A moving path: the shape at a key is the shape drawn, and between two keys it is worked
    # point by point at the fraction document 20 gives - the same numbers a mask's path gives.
    at = expected["cases"]["FX-SHP-017"]["frames"]

    def alphas(frame):
        return [round(frame[x][3], 12) for x in range(W)]   # the top row, which is enough

    assert at["0"] == c["FX-SHP-001"]
    assert alphas(at["0"]) == [1, 1, 1, 0, 0, 0]
    assert alphas(at["1"]) == [0.25, 1, 1, 0.75, 0, 0]
    assert alphas(at["2"]) == [0, 0.5, 1, 1, 0.5, 0]
    assert alphas(at["3"]) == [0, 0, 0.75, 1, 1, 0.25]
    assert alphas(at["4"]) == [0, 0, 0, 1, 1, 1]


if __name__ == "__main__":
    main()
