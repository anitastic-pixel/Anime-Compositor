"""Dashed strokes on shape layers, worked a second way.

D-1050 proposes that a shape may carry `dashes` of three properties, each keyed as any number is:
`dash` and `gap`, in pixels from 0 to 10000, and `offset`, in pixels, any number. The stroke is
then drawn as dashes along its path, which is After Effects' Stroke > Dashes (Dash, Gap, Offset).

The rule, on each line the stroke is drawn along (the whole flattened outline, or D-169's trimmed
stretches as open lines, measured from each line's own start):
  If the gap is 0, or dash + gap is under 1 pixel, the line is drawn whole, as with no dashes.
  L is the line's length; a line of no length is drawn whole.
  P = dash + gap and phase = offset less its whole P's, so 0 <= phase < P.
  For k = 0, 1, 2, ... while s = k P - phase is at most L, the dash from s to s + dash,
  clipped to 0 .. L, is kept if its clipped length is above 0; a dash of length 0 is kept as a
  single place where 0 <= s <= L (s < L on a closed line, whose place L is its place 0).
  On a closed line, if the first kept dash begins at 0 and the last ends at L, they are one line
  through the start; if one dash covers 0 to L the line is drawn whole and closed.
  Each dash is cut as D-169 cuts a stretch, and is an open line, so it takes the stroke's caps.
The coverage is then D-78's and D-170's, unchanged.

**This file never runs the build's code path.** It reuses `shape_style_reference.py` for caps,
joins and trims, and adds only the dashes.

    python tools/shape_dash_reference.py
"""

import json
import sys
from copy import deepcopy
from math import hypot

sys.path.insert(0, str(__import__("pathlib").Path(__file__).resolve().parent))
from adjust_reference import W, H, over, prop, fmt  # noqa: E402
from mask_reference import draw_cases  # noqa: E402
import shape_reference as sr  # noqa: E402
import shape_style_reference as ss  # noqa: E402
import shape_trim_reference as tr  # noqa: E402

ROOT = __import__("pathlib").Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "shape_dashes"
PICTURE = ROOT / "verification" / "D-1050 pictures" / "dash_cases.png"
KEY_PICTURE = ROOT / "verification" / "D-1050 pictures" / "dash_moving_case.png"
KEY_FRAMES = 5
TOLERANCE = 1e-6


# --- the dashes ---------------------------------------------------------------------------------

def dashed(line, closed, dashes, frame):
    """The lines one line of the stroke becomes, each with whether it is closed."""
    dash, gap, offset = (tr.value_at(dashes[k], frame) for k in ("dash", "gap", "offset"))
    if gap <= 0 or dash + gap < 1:
        return [(line, closed)]
    pts = line + [line[0]] if closed else line
    runs = [hypot(q[0] - p[0], q[1] - p[1]) for p, q in zip(pts, pts[1:])]
    along = [0.0]
    for r in runs:
        along.append(along[-1] + r)
    total = along[-1]
    if total == 0:
        return [(line, closed)]

    def place(d):
        i = next(i for i, r in enumerate(runs) if r > 0 and along[i + 1] >= d)
        f = (d - along[i]) / runs[i]
        p, q = pts[i], pts[i + 1]
        return (p[0] + (q[0] - p[0]) * f, p[1] + (q[1] - p[1]) * f)

    def piece(d0, d1):
        inside = [pts[j] for j in range(1, len(pts) - 1) if d0 < along[j] < d1]
        return [place(d0), *inside, place(d1)]

    period = dash + gap
    phase = offset % period
    kept = []
    k = 0
    while k * period - phase <= total:
        s = k * period - phase
        a, b = max(s, 0.0), min(s + dash, total)
        if dash > 0 and b > a:
            kept.append((a, b))
        elif dash == 0 and 0 <= s and (s < total or (s == total and not closed)):
            kept.append((s, s))
        k += 1
    if closed and len(kept) == 1 and kept[0] == (0.0, total):
        return [(line, True)]
    lines = [piece(a, b) for a, b in kept]
    if closed and dash > 0 and len(kept) >= 2 and kept[0][0] == 0 and kept[-1][1] == total:
        lines = [lines[-1] + lines[0][1:]] + lines[1:-1]
    return [(l, False) for l in lines]


def layer_picture(layer, frame):
    picture = [[0.0] * 4 for _ in range(W * H)]
    for shape in layer["shapes"]:
        closed = shape.get("closed", True)
        poly = sr.flatten(shape["points"], closed)
        f = shape.get("fill")
        if f:
            picture = sr.paint(picture, sr.fill_field(poly), f["color"], f["opacity"])
        t = shape.get("stroke")
        if t:
            lines = ss.lines_of(shape, poly, closed, frame)
            if shape.get("dashes"):
                lines = [d for l, c in lines for d in dashed(l, c, shape["dashes"], frame)]
            field = ss.stroke_field(lines, t["width_px"], t.get("join", "round"),
                                    t.get("miter_limit", 4), t.get("cap", "round"))
            picture = sr.paint(picture, field, t["color"], t["opacity"])
    return picture


def render(case, frame=0):
    out = [[0.0] * 4 for _ in range(W * H)]
    for layer in case["layers"]:
        out = [over(s, d) for s, d in zip(layer_picture(layer, frame), out)]
    return out


# --- the project files --------------------------------------------------------------------------

def project_json(name, case):
    project = ss.project_json(name, case)
    for record, layer in zip(project["compositions"][0]["layers"], case["layers"]):
        for shape_record, shape in zip(record["shapes"], layer["shapes"]):
            if shape.get("dashes"):
                shape_record["dashes"] = {k: tr.number_json(shape["dashes"][k])
                                          for k in ("dash", "gap", "offset")}
    return project


# --- the cases ----------------------------------------------------------------------------------

def dashes(dash, gap, offset=0):
    def number(v):
        return {"keys": v} if isinstance(v, tuple) else {"at": v}
    return {"dash": number(dash), "gap": number(gap), "offset": number(offset)}


BUTT = {"cap": "butt"}


def line(d, cap=None, **kw):
    stroke = ss.stroke(**({"cap": cap} if cap else {}))
    return ss.one(tr.ACROSS, closed=False, stroke=stroke, dashes=d, **kw)


CASES = {
    "FX-SHP-130": ("The one-pixel line along the frame's middle, dash 1 and gap 2, round caps: "
                   "dashes from x = 0 to 1 and 3 to 4, each with a round cap half a pixel past "
                   "its ends.",
                   line(dashes(1, 2))),
    "FX-SHP-131": ("The same with butt caps: columns 0 and 3 exactly, nothing else.",
                   line(dashes(1, 2), "butt")),
    "FX-SHP-132": ("Butt caps, offset 1: the pattern moves one pixel back along the line, so "
                   "columns 2 and 5.",
                   line(dashes(1, 2, 1), "butt")),
    "FX-SHP-133": ("Butt caps, offset -1: the same as offset 2, columns 1 and 4.",
                   line(dashes(1, 2, -1), "butt")),
    "FX-SHP-134": ("A gap of 0: the whole line, exactly as with no dashes.",
                   line(dashes(3, 0))),
    "FX-SHP-135": ("Dash 0, gap 2, round caps: round dots at x = 0, 2, 4 and 6.",
                   line(dashes(0, 2))),
    "FX-SHP-136": ("Dash 0.25 and gap 0.25, under a pixel together: the whole line.",
                   line(dashes(0.25, 0.25))),
    "FX-SHP-137": ("The closed rectangle taller than the frame, dash 2 and gap 1, butt caps: "
                   "its right side is the dash from y = -1 to 1 and from 2 to 3, so the right "
                   "band is drawn in the top row and not the bottom; its left side is the same, "
                   "its top row the last dash, which runs on through the start into the first.",
                   ss.one(sr.TALL, stroke=ss.stroke(cap="butt"), dashes=dashes(2, 1))),
    "FX-SHP-138": ("Trimmed 50 to 100, then dash 1 and gap 1, butt caps: the dashes are measured "
                   "from the trimmed line's start, so columns 3 and 5.",
                   line(dashes(1, 1), "butt", trim=tr.trim(50, 100))),
    "FX-SHP-139": ("A fill and a dashed stroke: the fill is whole; only the stroke is dashed.",
                   ss.one(sr.TALL, stroke=ss.stroke(cap="butt"), fill=sr.fill(),
                          dashes=dashes(2, 1))),
    "FX-SHP-140": ("Butt caps, dash 1 and gap 2, the offset keyed from 0 at frame 0 to 3 at "
                   "frame 3, linear: the dashes march back along the line a pixel a frame, and "
                   "frames 3 and 4 are frame 0 again.",
                   {**line(dashes(1, 2, ((0, 0, "linear"), (3, 3, "linear"))), "butt"),
                    "frames": KEY_FRAMES}),
}

KEYED = ["FX-SHP-140"]


def refusals():
    """Files a build must refuse whole, as PROJECT_SCHEMA_INVALID, each a change to FX-SHP-131."""
    def edit(change):
        p = deepcopy(project_json("FX-SHP-131", CASES["FX-SHP-131"][1]))
        change(p["compositions"][0]["layers"][0]["shapes"][0]["dashes"])
        return p

    def put(key, value):
        return lambda d: d.__setitem__(key, value)

    return {
        "FX-SHP-141": ("A dash below 0.", edit(put("dash", prop(-1)))),
        "FX-SHP-142": ("A gap below 0.", edit(put("gap", prop(-0.5)))),
        "FX-SHP-143": ("A key of the gap above 10000.",
                       edit(put("gap", {"base": 2, "keyframes": [
                           {"frame": 0, "value": 2, "interp": "linear"},
                           {"frame": 2, "value": 20000, "interp": "linear"}]}))),
        "FX-SHP-144": ("An offset of two numbers rather than one.",
                       edit(put("offset", prop([0, 0])))),
        "FX-SHP-145": ("An expression on the dash.",
                       edit(lambda d: d["dash"].__setitem__(
                           "expression", {"text": "1", "enabled": True}))),
        "FX-SHP-146": ("Dashes with no gap.", edit(lambda d: d.pop("gap"))),
    }


# --- running ------------------------------------------------------------------------------------

def main():
    OUT.mkdir(parents=True, exist_ok=True)
    PICTURE.parent.mkdir(parents=True, exist_ok=True)
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

    print("Refused whole, each as `PROJECT_SCHEMA_INVALID`; each is FX-SHP-131's file with one "
          "change:\n")
    for fx, (says, project) in refusals().items():
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(project, indent=2) + "\n", encoding="utf-8")
        expected["refused"][fx] = {"says": says, "project": name, "code": "PROJECT_SCHEMA_INVALID"}
        print(f"- {fx}: {says}")
    print()

    (OUT / "expected_shape_dashes.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    draw_cases({fx: [f["0"]] for fx, f in drawn.items() if fx not in KEYED}, PICTURE)
    draw_cases({fx: [drawn[fx][str(n)] for n in range(KEY_FRAMES)] for fx in KEYED}, KEY_PICTURE)
    print(f"Drawn: {PICTURE}\nDrawn: {KEY_PICTURE}")

    # The claims the cases make, checked on the numbers just worked.
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    a = {fx: [round(p[3], 12) for p in f] for fx, f in c.items()}
    whole = [round(p[3], 12) for p in render(ss.one(tr.ACROSS, closed=False, stroke=ss.stroke()))]

    def columns(fx):
        return [x for x in range(W) if a[fx][x] > 0]

    assert columns("FX-SHP-131") == [0, 3] and a["FX-SHP-131"][0] == 0.5
    assert a["FX-SHP-131"][:W] == a["FX-SHP-131"][W:]
    assert columns("FX-SHP-132") == [2, 5]
    assert columns("FX-SHP-133") == [1, 4]
    assert a["FX-SHP-130"][0] == a["FX-SHP-130"][3] == whole[0] and a["FX-SHP-130"][5] == 0
    assert 0 < a["FX-SHP-130"][1] < whole[1] and 0 < a["FX-SHP-130"][2] < whole[2]
    assert a["FX-SHP-134"] == whole and a["FX-SHP-136"] == whole
    dots = a["FX-SHP-135"]
    assert dots[0] > 0 and dots[2] > 0 and dots[4] > 0 and dots[5] > 0 and dots[0] < whole[0]
    box = a["FX-SHP-137"]
    assert box[2] == box[3] == 0.5 and box[W + 2] == box[W + 3] == 0
    assert box[0] == 0.5 and box[1] == box[W] == 0
    assert columns("FX-SHP-138") == [3, 5]
    assert c["FX-SHP-139"][1] == [*sr.FILL, 1.0] and c["FX-SHP-139"][W + 1] == [*sr.FILL, 1.0]
    moving = expected["cases"]["FX-SHP-140"]["frames"]
    assert moving["0"] == moving["3"] == moving["4"] == c["FX-SHP-131"]
    assert moving["1"] == c["FX-SHP-132"]


if __name__ == "__main__":
    main()
