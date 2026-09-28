"""Shape styles over time, and line joins and caps, worked a second way.

D-170 proposes two things for a shape layer's shapes.

Keyed styles. A fill's `color` and `opacity`, and a stroke's `color`, `opacity` and `width_px`,
may each be written as a property's record, `{"base", "keyframes"}`, as a keyed effect setting
is (D-68), instead of the plain value. A colour's keys hold three numbers, which move together.
Each is resolved to the frame by document 20's hold and linear, then drawn exactly as D-78 draws
a plain value. The ranges are D-78's, held by every key.

Joins and caps. A stroke may carry `join`, "miter", "round" or "bevel", `miter_limit`, 1 to 100,
and `cap`, "butt", "round" or "square". Written nowhere they are round and round, and a stroke
with round joins and round caps is D-78's coverage unchanged: the samples within half the width
of the path. Otherwise, with r half the width, a sample is covered if it is inside any of:
  the body of each piece of non-zero length: along it from its start to its end, and at most r
    from the line through it;
  a cap at each end of an open line: a disc of radius r round the end point for round; for
    square, the body of the first and the last piece carried on r beyond the end; for butt,
    nothing;
  a join where two pieces of non-zero length meet, at every point of the flattened outline
    between them, and on a closed line at its first point too: a disc of radius r round the
    point for round; nothing where the two pieces run on in one line or turn straight back;
    otherwise, on the outside of the turn, the triangle of the point and the two pieces' outer
    corners for bevel, and for miter the four-sided piece out to where the two outer edges meet,
    when that point is at most miter_limit half-widths from the corner, the bevel when it is not.
A line with no length at all is drawn as its caps would be at its one place: a disc for round,
a square lined up with the frame for square, nothing for butt.

A trim (D-169) cuts the stroke into open lines, each with its caps. Where a closed path's
stretch runs through the path's first point, it is one line joined there, not two cut ones;
with round joins and caps the two are the same pixels, so D-169's fixtures stand.

**This file never runs the build's code path.** It reuses `shape_reference.py` for everything
D-78 fixed and `shape_trim_reference.py` for D-169's stretch.

    python tools/shape_style_reference.py
"""

import json
import sys
from copy import deepcopy
from math import sqrt
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import W, H, over, prop, fmt  # noqa: E402
from mask_reference import draw_cases, png_any, points  # noqa: E402
import shape_reference as sr  # noqa: E402
import shape_trim_reference as tr  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "shape_styles"
PICTURE = ROOT / "verification" / "B-110a proposal" / "style_cases.png"
KEY_PICTURE = ROOT / "verification" / "B-110a proposal" / "style_moving_cases.png"
ILLUSTRATION = ROOT / "verification" / "B-110a proposal" / "joins_and_caps_enlarged.png"
KEY_FRAMES = 5
TOLERANCE = 1e-6

CHEVRON = points([1, 0], [4, 1], [1, 2])               # open, a point at (4, 1) facing right
DIAMOND = points([1, 1], [3, 0.2], [5, 1], [3, 1.8])   # closed, points at (1, 1) and (5, 1)
BACK = points([1, 1], [4, 1], [2, 1])                  # open, turning straight back at (4, 1)
DOT = points([2, 1], [2, 1])                           # open, two points in one place


# --- a style at a frame -------------------------------------------------------------------------

def value_at(v, frame):
    """A plain value, or `{"keys": [(frame, value, interp)]}` by document 20's hold and linear."""
    if not isinstance(v, dict):
        return v
    keys = v["keys"]
    if frame <= keys[0][0]:
        return keys[0][1]
    if frame >= keys[-1][0]:
        return keys[-1][1]
    a, b = next((a, b) for a, b in zip(keys, keys[1:]) if a[0] <= frame < b[0])
    if a[2] == "hold":
        return a[1]
    u = (frame - a[0]) / (b[0] - a[0])
    if isinstance(a[1], list):
        return [x + (y - x) * u for x, y in zip(a[1], b[1])]
    return a[1] + (b[1] - a[1]) * u


# --- joins and caps -----------------------------------------------------------------------------

def disc(c, r):
    return lambda x, y: (x - c[0]) * (x - c[0]) + (y - c[1]) * (y - c[1]) <= r * r


def body(a, b, r, before, after):
    """A piece's band, carried on `before` and `after` pixels past its ends."""
    dx, dy = b[0] - a[0], b[1] - a[1]
    run2 = dx * dx + dy * dy
    run = sqrt(run2)

    def inside(x, y):
        px, py = x - a[0], y - a[1]
        dot = px * dx + py * dy
        cross = px * dy - py * dx
        return -before * run <= dot <= run2 + after * run and abs(cross) <= r * run
    return inside


def convex(poly):
    """Inside or on the edge of a convex polygon, whichever way round it is written."""
    def inside(x, y):
        sides = [(q[0] - p[0]) * (y - p[1]) - (q[1] - p[1]) * (x - p[0])
                 for p, q in zip(poly, poly[1:] + poly[:1])]
        return all(s >= 0 for s in sides) or all(s <= 0 for s in sides)
    return inside


def joint(a, v, c, r, join, limit):
    if join == "round":
        return disc(v, r)
    run1 = sqrt((v[0] - a[0]) ** 2 + (v[1] - a[1]) ** 2)
    run2 = sqrt((c[0] - v[0]) ** 2 + (c[1] - v[1]) ** 2)
    u1 = ((v[0] - a[0]) / run1, (v[1] - a[1]) / run1)
    u2 = ((c[0] - v[0]) / run2, (c[1] - v[1]) / run2)
    turn = u1[0] * u2[1] - u1[1] * u2[0]
    if turn == 0:
        return lambda x, y: False
    s = -1.0 if turn > 0 else 1.0  # the outside of the turn
    n1, n2 = (-u1[1], u1[0]), (-u2[1], u2[0])
    p1 = (v[0] + s * r * n1[0], v[1] + s * r * n1[1])
    p2 = (v[0] + s * r * n2[0], v[1] + s * r * n2[1])
    d = u1[0] * u2[0] + u1[1] * u2[1]
    if join == "miter" and 2 <= limit * limit * (1 + d):
        k = s * r / (1 + d)
        m = (v[0] + k * (n1[0] + n2[0]), v[1] + k * (n1[1] + n2[1]))
        return convex([v, p1, m, p2])
    return convex([v, p1, p2])


def styled_inside(lines, r, join, limit, cap):
    tests = []
    for line, closed in lines:
        pts = list(line) + [line[0]] if closed else list(line)
        segs = [(p, q) for p, q in zip(pts, pts[1:]) if tuple(p) != tuple(q)]
        if not segs:
            c = line[0]
            if cap == "round":
                tests.append(disc(c, r))
            elif cap == "square":
                tests.append(lambda x, y, c=c: abs(x - c[0]) <= r and abs(y - c[1]) <= r)
            continue
        square = not closed and cap == "square"
        for i, (a, b) in enumerate(segs):
            tests.append(body(a, b, r, r if square and i == 0 else 0.0,
                              r if square and i == len(segs) - 1 else 0.0))
        if not closed and cap == "round":
            tests += [disc(segs[0][0], r), disc(segs[-1][1], r)]
        pairs = list(zip(segs, segs[1:])) + ([(segs[-1], segs[0])] if closed else [])
        for (a, v), (_, c) in pairs:
            tests.append(joint(a, v, c, r, join, limit))
    return lambda x, y: any(t(x, y) for t in tests)


def lines_of(shape, poly, closed, frame):
    """The lines the stroke is drawn along: the whole path, or D-169's stretches as open lines."""
    pieces = tr.stretches(poly, closed, shape["trim"], frame) if shape.get("trim") else None
    if pieces is None:
        return [(poly, closed)]
    if closed and len(pieces) == 2:
        pieces = [pieces[0] + pieces[1][1:]]
    return [(p, False) for p in pieces]


def stroke_field(lines, width, join, limit, cap):
    r = width / 2
    if join == "round" and cap == "round":
        # D-78's coverage, unchanged.
        inside = lambda x, y: min(sr.distance_to_path(l, c, x, y) for l, c in lines) <= r  # noqa
    else:
        inside = styled_inside(lines, r, join, limit, cap)
    return [sr.pixel_coverage(inside, x, y) if lines else 0.0
            for y in range(H) for x in range(W)]


def layer_picture(layer, frame):
    picture = [[0.0] * 4 for _ in range(W * H)]
    for shape in layer["shapes"]:
        closed = shape.get("closed", True)
        poly = sr.flatten(shape["points"], closed)
        f = shape.get("fill")
        if f:
            picture = sr.paint(picture, sr.fill_field(poly), value_at(f["color"], frame),
                               value_at(f["opacity"], frame))
        t = shape.get("stroke")
        if t:
            field = stroke_field(lines_of(shape, poly, closed, frame),
                                 value_at(t["width_px"], frame), t.get("join", "round"),
                                 t.get("miter_limit", 4), t.get("cap", "round"))
            picture = sr.paint(picture, field, value_at(t["color"], frame),
                               value_at(t["opacity"], frame))
    return picture


def render(case, frame=0):
    out = [[0.0] * 4 for _ in range(W * H)]
    for layer in case["layers"]:
        out = [over(s, d) for s, d in zip(layer_picture(layer, frame), out)]
    return out


# --- the project files --------------------------------------------------------------------------

def style_json(v):
    if not isinstance(v, dict):
        return v
    record = prop(v["keys"][0][1])
    for frame, value, interp in v["keys"]:
        record["keyframes"].append({"frame": frame, "value": value, "interp": interp})
    return record


def project_json(name, case):
    project = sr.project_json(name, case)
    for record, layer in zip(project["compositions"][0]["layers"], case["layers"]):
        for shape_record, shape in zip(record["shapes"], layer["shapes"]):
            for paint in ("fill", "stroke"):
                if shape.get(paint):
                    shape_record[paint] = {k: style_json(v) for k, v in shape[paint].items()}
            if shape.get("trim"):
                shape_record["trim"] = {k: tr.number_json(shape["trim"][k])
                                        for k in ("start", "end", "offset")}
    return project


# --- the cases ----------------------------------------------------------------------------------

def stroke(width=1.0, **extra):
    return {**sr.stroke(width=width), **extra}


def one(pts, closed=True, **kw):
    return {"layers": [{"id": "shapes", "kind": "shape", "shapes": [
        sr.shape(pts, closed=closed, **kw)]}]}


def keys(*frames):
    return {"keys": list(frames)}


CASES = {
    "FX-SHP-100": ("A one-pixel line from (1, 1) to (5, 1) with butt caps: it stops square at its "
                   "end points, so it covers x = 1 to 5 and half of each row.",
                   one(sr.LINE, False, stroke=stroke(cap="butt"))),
    "FX-SHP-101": ("The same with square caps: carried on half the width past each end, "
                   "x = 0.5 to 5.5, square.",
                   one(sr.LINE, False, stroke=stroke(cap="square"))),
    "FX-SHP-102": ("The same with round caps written in: exactly D-78's line with nothing "
                   "written, FX-SHP-008.",
                   one(sr.LINE, False, stroke=stroke(cap="round", join="round"))),
    "FX-SHP-103": ("A chevron, (1, 0) to (4, 1) to (1, 2), one pixel wide, with a mitre join and "
                   "the default mitre limit of 4 and butt caps: the corner is 3.16 half-widths "
                   "long, inside the limit, so it comes to a sharp point at x = 5.58.",
                   one(CHEVRON, False, stroke=stroke(join="miter", cap="butt"))),
    "FX-SHP-104": ("The same with a mitre limit of 3: 3.16 is past it, so the corner is bevelled, "
                   "exactly FX-SHP-105.",
                   one(CHEVRON, False, stroke=stroke(join="miter", miter_limit=3, cap="butt"))),
    "FX-SHP-105": ("The chevron with a bevel join: the corner cut off straight.",
                   one(CHEVRON, False, stroke=stroke(join="bevel", cap="butt"))),
    "FX-SHP-106": ("The chevron with a round join: the corner rounded, reaching x = 4.5.",
                   one(CHEVRON, False, stroke=stroke(join="round", cap="butt"))),
    "FX-SHP-107": ("A closed diamond, (1, 1), (3, 0.2), (5, 1), (3, 1.8), mitred: both its side "
                   "points are sharp, the one at its first point, (1, 1), included, because a "
                   "closed path is joined where it closes.",
                   one(DIAMOND, stroke=stroke(join="miter"))),
    "FX-SHP-108": ("The diamond bevelled.",
                   one(DIAMOND, stroke=stroke(join="bevel"))),
    "FX-SHP-109": ("The line trimmed 25 to 75 with butt caps: the cut ends are butt too, "
                   "x = 2 to 4 exactly.",
                   one(sr.LINE, False, stroke=stroke(cap="butt"), trim=tr.trim(25, 75))),
    "FX-SHP-110": ("Trimmed the same with square caps: x = 1.5 to 4.5, square.",
                   one(sr.LINE, False, stroke=stroke(cap="square"), trim=tr.trim(25, 75))),
    "FX-SHP-111": ("The mitred diamond trimmed 0 to 50 with an offset of 270 and butt caps: the "
                   "stretch is its last side and its first, running through its first point, "
                   "(1, 1), where it is one line with its mitre rather than two cut ends.",
                   one(DIAMOND, stroke=stroke(join="miter", cap="butt"),
                       trim=tr.trim(0, 50, 270))),
    "FX-SHP-112": ("A line from (1, 1) to (4, 1) and back to (2, 1), mitred with butt caps: a "
                   "mitre on a line that turns straight back would be endless, so there is none, "
                   "and nothing is drawn past x = 4.",
                   one(BACK, False, stroke=stroke(join="miter", cap="butt"))),
    "FX-SHP-113": ("An open path of two points both at (2, 1), with square caps: a square one "
                   "pixel across, lined up with the frame, x = 1.5 to 2.5.",
                   one(DOT, False, stroke=stroke(cap="square"))),
    "FX-SHP-114": ("The same with butt caps: nothing is drawn.",
                   one(DOT, False, stroke=stroke(cap="butt"))),
    "FX-SHP-115": ("The line with its width keyed from 1 at frame 0 to 2 at frame 4, linear: it "
                   "thickens a quarter of a pixel each frame.",
                   {**one(sr.LINE, False, stroke=stroke(width=keys((0, 1.0, "linear"),
                                                                   (4, 2.0, "linear")))),
                    "frames": KEY_FRAMES}),
    "FX-SHP-116": ("A filled box over the left three columns, its colour keyed from the fill's "
                   "blue at frame 0 to the stroke's red at frame 4 and its opacity from 1 to 0.5, "
                   "both linear.",
                   {**one(sr.LEFT3, fill={"color": keys((0, list(sr.FILL), "linear"),
                                                        (4, list(sr.STROKE), "linear")),
                                          "opacity": keys((0, 1.0, "linear"), (4, 0.5, "linear"))}),
                    "frames": KEY_FRAMES}),
    "FX-SHP-117": ("The line's stroke colour held on red until frame 2, then blue, and its opacity "
                   "keyed from 1 at frame 0 to 0 at frame 4, linear.",
                   {**one(sr.LINE, False, stroke={**sr.stroke(),
                                                  "color": keys((0, list(sr.STROKE), "hold"),
                                                                (2, list(sr.FILL), "hold")),
                                                  "opacity": keys((0, 1.0, "linear"),
                                                                  (4, 0.0, "linear"))}),
                    "frames": KEY_FRAMES}),
}

KEYED = ["FX-SHP-115", "FX-SHP-116", "FX-SHP-117"]


def refusals():
    """Files a build must refuse whole, as PROJECT_SCHEMA_INVALID."""
    def edit(fx, change):
        p = deepcopy(project_json(fx, CASES[fx][1]))
        change(p["compositions"][0]["layers"][0]["shapes"][0])
        return p

    def stroke_put(key, value):
        return lambda s: s["stroke"].__setitem__(key, value)

    return {
        "FX-SHP-120": ("A join that is not miter, round or bevel.",
                       edit("FX-SHP-103", stroke_put("join", "sharp"))),
        "FX-SHP-121": ("A cap that is not butt, round or square.",
                       edit("FX-SHP-103", stroke_put("cap", "arrow"))),
        "FX-SHP-122": ("A mitre limit below 1.", edit("FX-SHP-103", stroke_put("miter_limit", 0.5))),
        "FX-SHP-123": ("A mitre limit above 100.",
                       edit("FX-SHP-103", stroke_put("miter_limit", 101))),
        "FX-SHP-124": ("A key of the stroke width at 0.",
                       edit("FX-SHP-115", lambda s: s["stroke"]["width_px"]["keyframes"][1]
                            .__setitem__("value", 0))),
        "FX-SHP-125": ("A key of the fill opacity above 1.",
                       edit("FX-SHP-116", lambda s: s["fill"]["opacity"]["keyframes"][1]
                            .__setitem__("value", 1.5))),
        "FX-SHP-126": ("A key of the fill colour of two numbers.",
                       edit("FX-SHP-116", lambda s: s["fill"]["color"]["keyframes"][1]
                            .__setitem__("value", [0.5, 0.5]))),
        "FX-SHP-127": ("An expression on the stroke width.",
                       edit("FX-SHP-115", lambda s: s["stroke"]["width_px"].__setitem__(
                           "expression", {"text": "1", "enabled": True}))),
    }


def illustrate():
    """The three joins across and the three caps down, on a chevron 24 pixels wide, drawn big by
    the same rule so the shapes can be judged by eye; its centre line is drawn dark on top."""
    cell_w, cell_h, gap = 170, 110, 6
    line = [(25, 18), (110, 55), (25, 92)]
    joins, caps = ["miter", "bevel", "round"], ["butt", "square", "round"]
    w, h = 3 * cell_w + 4 * gap, 3 * cell_h + 4 * gap
    canvas = [(24, 24, 28, 255)] * (w * h)
    for row, cap in enumerate(caps):
        for col, join in enumerate(joins):
            inside = styled_inside([(line, False)], 12, join, 4, cap)
            centre = styled_inside([(line, False)], 0.8, "round", 4, "round")
            left, top = gap + col * (cell_w + gap), gap + row * (cell_h + gap)
            for y in range(cell_h):
                for x in range(cell_w):
                    sx, sy = x + 0.5, y + 0.5
                    colour = ((40, 40, 48) if centre(sx, sy) else (232, 120, 80) if inside(sx, sy)
                              else (200, 200, 205) if (x // 10 + y // 10) % 2 else (170, 170, 176))
                    canvas[(top + y) * w + left + x] = (*colour, 255)
    ILLUSTRATION.write_bytes(png_any(canvas, w, h))


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

    print("Refused whole, each as `PROJECT_SCHEMA_INVALID`; each is the named case's file with "
          "one change:\n")
    for fx, (says, project) in refusals().items():
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(project, indent=2) + "\n", encoding="utf-8")
        expected["refused"][fx] = {"says": says, "project": name, "code": "PROJECT_SCHEMA_INVALID"}
        print(f"- {fx}: {says}")
    print()

    (OUT / "expected_shape_styles.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    PICTURE.parent.mkdir(parents=True, exist_ok=True)
    draw_cases({fx: [f["0"]] for fx, f in drawn.items() if fx not in KEYED}, PICTURE)
    draw_cases({fx: [drawn[fx][str(n)] for n in range(KEY_FRAMES)] for fx in KEYED}, KEY_PICTURE)
    illustrate()
    print(f"Drawn: {PICTURE}\nDrawn: {KEY_PICTURE}\nDrawn: {ILLUSTRATION}")

    # The claims the cases make, checked on the numbers just worked.
    a = {fx: [round(p[3], 12) for p in v["frames"]["0"]] for fx, v in expected["cases"].items()}
    row = lambda fx, y=0: a[fx][y * W:(y + 1) * W]  # noqa: E731
    for y in (0, 1):
        assert row("FX-SHP-100", y) == [0, 0.5, 0.5, 0.5, 0.5, 0]
        assert row("FX-SHP-101", y) == [0.25, 0.5, 0.5, 0.5, 0.5, 0.25]
        assert row("FX-SHP-109", y) == [0, 0, 0.5, 0.5, 0, 0]
        assert row("FX-SHP-110", y) == [0, 0.25, 0.5, 0.5, 0.25, 0]
        assert row("FX-SHP-112", y) == [0, 0.5, 0.5, 0.5, 0, 0]
        assert row("FX-SHP-113", y) == [0, 0.25, 0.25, 0, 0, 0]
    d78 = [round(p[3], 12) for p in sr.render(sr.shapes(sr.shape(sr.LINE, closed=False,
                                                                  stroke=sr.stroke())))]
    assert a["FX-SHP-102"] == d78
    assert a["FX-SHP-104"] == a["FX-SHP-105"]
    tip = lambda fx: sum(a[fx][4:6]) + sum(a[fx][W + 4:W + 6])  # noqa: E731
    assert tip("FX-SHP-103") > tip("FX-SHP-106") > tip("FX-SHP-105") >= 0 and tip("FX-SHP-103") > 0
    assert a["FX-SHP-107"][0] > a["FX-SHP-108"][0] and a["FX-SHP-107"][5] > a["FX-SHP-108"][5]
    cut = styled_inside([(p, False) for p in tr.stretches(
        sr.flatten(DIAMOND, True), True, tr.trim(0, 50, 270), 0)], 0.5, "miter", 4, "butt")
    two_cut_ends = [sr.pixel_coverage(cut, x, y) for y in range(H) for x in range(W)]
    assert a["FX-SHP-111"][0] > two_cut_ends[0] and a["FX-SHP-111"][5] == 0
    assert all(v == 0 for v in a["FX-SHP-114"])
    widen = expected["cases"]["FX-SHP-115"]["frames"]
    assert [round(p[3], 12) for p in widen["0"]] == d78
    cover = [sum(p[3] for p in widen[str(f)]) for f in range(KEY_FRAMES)]
    assert all(x < y for x, y in zip(cover, cover[1:]))
    box = expected["cases"]["FX-SHP-116"]["frames"]
    assert box["0"][0] == [*sr.FILL, 1.0]
    assert [round(v, 12) for v in box["4"][0]] == [round(c * 0.5, 12) for c in sr.STROKE] + [0.5]
    held = expected["cases"]["FX-SHP-117"]["frames"]
    red = lambda f: held[str(f)][W + 2]  # noqa: E731
    assert red(0)[0] > red(0)[2] and red(1)[0] > red(1)[2] and red(2)[2] > red(2)[0]
    assert red(4)[3] == 0


if __name__ == "__main__":
    main()
