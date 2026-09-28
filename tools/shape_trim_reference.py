"""Trim paths on shape layers, worked a second way.

D-169 proposes that a shape may carry a `trim` of three properties, each keyed as any number is:
`start` and `end`, from 0 to 100 percent of the way along the path, and `offset`, in degrees,
360 of them once round. The stroke is then drawn along only that stretch of the path, which is
After Effects' Trim Paths and the way a line is drawn on. The fill is not trimmed.

The rule, on D-77's flattened outline, a closed one with its closing side from the last point
back to the first counted as the path's last piece:
  L is the outline's length, the sum of its pieces' straight lengths in order.
  s and e are start and end over 100, taken in the order low then high.
  If e - s is 1 the whole stroke is drawn, as with no trim; if e = s none of it is.
  Otherwise a = s + offset / 360, less its whole turns, so 0 <= a < 1, and b = a + (e - s).
  The stroke covers the path from a L to b L; past L it carries on from the path's beginning,
  on an open path as on a closed one, so a stretch that runs past the end is two pieces.
  A place d along the path is on the first piece of non-zero length whose far end is at least
  d along, at the fraction (d - where the piece starts) / its length.
A path of no length has nowhere to cut, and is stroked as with no trim.
Each stretch is a line of its own through the outline's points that lie strictly inside it,
and its coverage is D-78's, unchanged: the samples within half the stroke's width of it, so the
cut ends are round, as every end is.

**This file never runs the build's code path.** It reuses `shape_reference.py` for everything
D-78 already fixed and adds only the stretch.

    python tools/shape_trim_reference.py
"""

import json
import sys
from copy import deepcopy
from math import floor, hypot
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import W, H, over, prop, fmt  # noqa: E402
from mask_reference import draw_cases, points  # noqa: E402
import shape_reference as sr  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "shape_trims"
PICTURE = ROOT / "verification" / "B-109a proposal" / "trim_cases.png"
KEY_PICTURE = ROOT / "verification" / "B-109a proposal" / "trim_moving_case.png"
KEY_FRAMES = 5
TOLERANCE = 1e-6
ACROSS = points([0, 1], [6, 1])  # open, six pixels long, along the frame's middle


# --- the stretch --------------------------------------------------------------------------------

def value_at(p, frame):
    """A trim number at a frame: document 20's hold and linear, a key's value held outside."""
    keys = p.get("keys")
    if not keys:
        return p["at"]
    if frame <= keys[0][0]:
        return keys[0][1]
    if frame >= keys[-1][0]:
        return keys[-1][1]
    a, b = next((a, b) for a, b in zip(keys, keys[1:]) if a[0] <= frame < b[0])
    if a[2] == "hold":
        return a[1]
    return a[1] + (b[1] - a[1]) * (frame - a[0]) / (b[0] - a[0])


def stretches(poly, closed, trim, frame):
    """The pieces of the outline the stroke is drawn along, or None for the whole of it."""
    s, e = sorted((value_at(trim["start"], frame) / 100, value_at(trim["end"], frame) / 100))
    if e - s >= 1:
        return None
    if e == s:
        return []
    pts = poly + [poly[0]] if closed else poly
    runs = [hypot(q[0] - p[0], q[1] - p[1]) for p, q in zip(pts, pts[1:])]
    along = [0.0]
    for r in runs:
        along.append(along[-1] + r)
    total = along[-1]
    if total == 0:
        return None

    def place(d):
        i = next(i for i, r in enumerate(runs) if r > 0 and along[i + 1] >= d)
        f = (d - along[i]) / runs[i]
        p, q = pts[i], pts[i + 1]
        return (p[0] + (q[0] - p[0]) * f, p[1] + (q[1] - p[1]) * f)

    def piece(u0, u1):
        d0, d1 = u0 * total, u1 * total
        inside = [pts[j] for j in range(1, len(pts) - 1) if d0 < along[j] < d1]
        return [place(d0), *inside, place(d1)]

    a = s + value_at(trim["offset"], frame) / 360
    a -= floor(a)
    b = a + (e - s)
    if b <= 1:
        return [piece(a, b)]
    return [piece(a, 1.0), piece(0.0, b - 1)]


def stretch_field(pieces, width_px):
    radius = width_px / 2
    return [sr.pixel_coverage(lambda sx, sy: min(sr.distance_to_path(p, False, sx, sy)
                                                 for p in pieces) <= radius, x, y)
            if pieces else 0.0
            for y in range(H) for x in range(W)]


def layer_picture(layer, frame_no):
    picture = [[0.0] * 4 for _ in range(W * H)]
    for shape in layer["shapes"]:
        closed = shape.get("closed", True)
        poly = sr.flatten(shape["points"], closed)
        if shape.get("fill"):
            f = shape["fill"]
            picture = sr.paint(picture, sr.fill_field(poly), f["color"], f["opacity"])
        if shape.get("stroke"):
            t = shape["stroke"]
            pieces = stretches(poly, closed, shape["trim"], frame_no) if shape.get("trim") else None
            field = (sr.stroke_field(poly, closed, t["width_px"]) if pieces is None
                     else stretch_field(pieces, t["width_px"]))
            picture = sr.paint(picture, field, t["color"], t["opacity"])
    return picture


def render(case, frame_no=0):
    frame = [[0.0] * 4 for _ in range(W * H)]
    for layer in case["layers"]:
        frame = [over(s, d) for s, d in zip(layer_picture(layer, frame_no), frame)]
    return frame


# --- the project files --------------------------------------------------------------------------

def number_json(p):
    record = prop(p["keys"][0][1] if p.get("keys") else p["at"])
    for frame, value, interp in p.get("keys", []):
        record["keyframes"].append({"frame": frame, "value": value, "interp": interp})
    return record


def project_json(name, case):
    project = sr.project_json(name, {**case, "layers": [
        {**l, "shapes": [{k: v for k, v in s.items() if k != "trim"} for s in l["shapes"]]}
        for l in case["layers"]]})
    for record, layer in zip(project["compositions"][0]["layers"], case["layers"]):
        for shape_record, shape in zip(record["shapes"], layer["shapes"]):
            t = shape.get("trim")
            if t:
                shape_record["trim"] = {k: number_json(t[k]) for k in ("start", "end", "offset")}
    return project


# --- the cases ----------------------------------------------------------------------------------

def trim(start, end, offset=0):
    def number(v):
        return {"keys": v} if isinstance(v, tuple) else {"at": v}
    return {"start": number(start), "end": number(end), "offset": number(offset)}


def line(t, **kw):
    return {"layers": [{"id": "shapes", "kind": "shape", "shapes": [
        sr.shape(ACROSS, closed=False, stroke=sr.stroke(), trim=t, **kw)]}]}


def box(t, **kw):
    return {"layers": [{"id": "shapes", "kind": "shape", "shapes": [
        sr.shape(sr.TALL, stroke=sr.stroke(), trim=t, **kw)]}]}


CASES = {
    "FX-SHP-070": ("A one-pixel stroke along the frame's middle, trimmed 0 to 100 with no "
                   "offset: the whole line, exactly as with no trim.",
                   line(trim(0, 100))),
    "FX-SHP-071": ("Trimmed 0 to 50: the left half, columns 0 to 2, ending in a round cap in "
                   "column 3.",
                   line(trim(0, 50))),
    "FX-SHP-072": ("Trimmed 25 to 75: the middle, from x = 1.5 to 4.5, a round cap at each end.",
                   line(trim(25, 75))),
    "FX-SHP-073": ("Start 75 and end 25, the other way round: the same stretch as 25 to 75.",
                   line(trim(75, 25))),
    "FX-SHP-074": ("Start and end both 40: nothing is drawn.",
                   line(trim(40, 40))),
    "FX-SHP-075": ("0 to 50 with an offset of 180 degrees, half a turn: the right half.",
                   line(trim(0, 50, 180))),
    "FX-SHP-076": ("0 to 50 with an offset of 270 degrees: the stretch runs off the right end "
                   "and carries on from the left, so both ends of the line are drawn and the "
                   "middle is not.",
                   line(trim(0, 50, 270))),
    "FX-SHP-077": ("An offset of -90 degrees is the same as 270.",
                   line(trim(0, 50, -90))),
    "FX-SHP-078": ("The closed rectangle taller than the frame, trimmed 0 to 50: its path starts "
                   "at the top left and runs clockwise, and its first half is its top and its "
                   "right side, of which only the right-hand band is in the frame.",
                   box(trim(0, 50))),
    "FX-SHP-079": ("The same with an offset of 180: the second half, and only the left-hand "
                   "band shows.",
                   box(trim(0, 50, 180))),
    "FX-SHP-080": ("0 to 100 with an offset of 90 on a closed path: still the whole outline, "
                   "the frame of FX-SHP-006.",
                   box(trim(0, 100, 90))),
    "FX-SHP-081": ("A fill and a trimmed stroke: the fill is whole, and only the stroke is "
                   "trimmed.",
                   box(trim(0, 50), fill=sr.fill())),
    "FX-SHP-082": ("The ellipse's outline trimmed 0 to 25: a quarter of its length along the "
                   "curve from its top, the right-hand upper quarter, measured on the flattened "
                   "outline.",
                   {"layers": [{"id": "shapes", "kind": "shape", "shapes": [
                       sr.shape(sr.ELLIPSE, stroke=sr.stroke(), trim=trim(0, 25))]}]}),
    "FX-SHP-083": ("The line drawn on: the end keyed from 0 at frame 0 to 100 at frame 4, "
                   "linear, so each frame draws a further quarter.",
                   {**line(trim(0, ((0, 0, "linear"), (4, 100, "linear")))),
                    "frames": KEY_FRAMES}),
}

KEYED = ["FX-SHP-083"]


def refusals():
    """Files a build must refuse whole, as PROJECT_SCHEMA_INVALID, each a change to FX-SHP-071."""
    def edit(change):
        p = deepcopy(project_json("FX-SHP-071", CASES["FX-SHP-071"][1]))
        change(p["compositions"][0]["layers"][0]["shapes"][0]["trim"])
        return p

    def put(key, value):
        return lambda t: t.__setitem__(key, value)

    return {
        "FX-SHP-090": ("A start below 0.", edit(put("start", prop(-10)))),
        "FX-SHP-091": ("An end above 100.", edit(put("end", prop(150)))),
        "FX-SHP-092": ("A key of the end above 100.",
                       edit(put("end", {"base": 0, "keyframes": [
                           {"frame": 0, "value": 0, "interp": "linear"},
                           {"frame": 2, "value": 101, "interp": "linear"}]}))),
        "FX-SHP-093": ("An offset of two numbers rather than one.",
                       edit(put("offset", prop([0, 0])))),
        "FX-SHP-094": ("An expression on the start.",
                       edit(lambda t: t["start"].__setitem__(
                           "expression", {"text": "0", "enabled": True}))),
        "FX-SHP-095": ("A trim with no end.", edit(lambda t: t.pop("end"))),
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

    print("Refused whole, each as `PROJECT_SCHEMA_INVALID`; each is FX-SHP-071's file with one "
          "change:\n")
    for fx, (says, project) in refusals().items():
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(project, indent=2) + "\n", encoding="utf-8")
        expected["refused"][fx] = {"says": says, "project": name, "code": "PROJECT_SCHEMA_INVALID"}
        print(f"- {fx}: {says}")
    print()

    (OUT / "expected_shape_trims.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    draw_cases({fx: [f["0"]] for fx, f in drawn.items() if fx not in KEYED}, PICTURE)
    draw_cases({fx: [drawn[fx][str(n)] for n in range(KEY_FRAMES)] for fx in KEYED}, KEY_PICTURE)
    print(f"Drawn: {PICTURE}\nDrawn: {KEY_PICTURE}")

    # The claims the cases make, checked on the numbers just worked.
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    alpha = {fx: [round(p[3], 12) for p in f] for fx, f in c.items()}
    whole_line = [round(p[3], 12) for p in render(sr.shapes(
        sr.shape(ACROSS, closed=False, stroke=sr.stroke())))]
    assert alpha["FX-SHP-070"] == whole_line
    left = alpha["FX-SHP-071"]
    assert left[:3] == whole_line[:3] and 0 < left[3] < whole_line[3] and left[4:6] == [0, 0]
    assert alpha["FX-SHP-072"] == alpha["FX-SHP-073"]
    mid = alpha["FX-SHP-072"]
    assert mid[0] == 0 and mid[5] == 0 and mid[2] == mid[3] == whole_line[2]
    assert all(v == 0 for v in alpha["FX-SHP-074"])
    right = alpha["FX-SHP-075"]
    assert right[:W] == left[:W][::-1]
    ends = alpha["FX-SHP-076"]
    assert ends[0] == whole_line[0] and ends[5] == whole_line[5] and ends[2] == ends[3] == 0
    assert alpha["FX-SHP-077"] == ends
    box_whole = [round(p[3], 12) for p in render(sr.shapes(sr.shape(sr.TALL, stroke=sr.stroke())))]
    assert alpha["FX-SHP-078"][:W] == [0, 0, 0.5, 0.5, 0, 0]
    assert alpha["FX-SHP-079"][:W] == [0.5, 0, 0, 0, 0, 0]
    assert alpha["FX-SHP-080"] == box_whole
    assert c["FX-SHP-081"][1] == [*sr.FILL, 1.0] and c["FX-SHP-081"][0] != c["FX-SHP-081"][2]
    quarter = alpha["FX-SHP-082"]
    assert quarter[3] > 0 and quarter[0] == quarter[1] == 0 and sum(quarter[W:]) < sum(quarter[:W])
    on = expected["cases"]["FX-SHP-083"]["frames"]
    drawn_on = [sum(p[3] for p in on[str(f)]) for f in range(KEY_FRAMES)]
    assert drawn_on[0] == 0 and all(a < b for a, b in zip(drawn_on, drawn_on[1:]))
    assert [round(p[3], 12) for p in on["4"]] == whole_line


if __name__ == "__main__":
    main()
