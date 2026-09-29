"""Time stretch, frame blending and the drawing dissolve, worked a second way.

D-216 proposes three things, all of them about which drawing a layer shows and none about where
the layer is. A raster or composition layer may be stretched in time; a stretched layer may mix
the two drawings either side of a time that falls between them (After Effects' Frame Mix, with a
switch on the layer and one on the composition, both needed); and a raster layer may dissolve
each held drawing into the next over the last frames of its hold.

The rule, for a layer active at composition frame `n`, with in point `I`, source offset `O` and
stretch `S` per cent:

    t = (n - I) * 100 / S + O,    f = floor(t),    w = t - f

worked in 64-bit numbers. `P(f)` is the layer's step-1 picture at the whole local frame `f`: the
drawing its exposure holds there (dissolved, below), or its composition drawn at frame `f`. The
layer's step-1 picture at `n` is `P(f)`, unless both switches are on, `w > 0` and `f + 1` is not
past the end of the layer's source; then it is

    P(f) + w * (P(f + 1) - P(f))

each of the four linear premultiplied working numbers apart. The dissolve: with `D` frames, at a
local frame `f` in a span `[s, e)` holding drawing `A`, when a span begins at exactly `e` holding
`B`, `d = min(D, e - s - 1)`, and on the last `d` frames of the hold, `f >= e - d`,

    P(f) = A + ((f - (e - d) + 1) / (d + 1)) * (B - A)

so the hold's first frame is always `A` whole and the next span's first frame is `B` whole.
Masks, effects, transform, matte, opacity and blend then run once on that picture, at `n`.
Property keys stay on their composition frames: a stretch does not move them.

**This file never runs the build's code path.** It reads the very project files it writes and
draws each one-pixel-high frame pixel by pixel from documents 20 and 21.

Four kinds of case. The times cases are `t`, `f` and `w` for a list of frames. The pixel cases are
a composition 8 pixels by 1 holding drawings 8 by 1 of full and empty channels, so every mix is a
share that can be checked by hand. The commands cases are where the Time Stretch command puts the
out point. Then there are the files a build must refuse.

The projects go into `Fixtures/frame_blending`, the expected numbers into
`Fixtures/frame_blending/expected_frame_blending.json`, and the proposal's pictures into
`verification/B-150a proposal/`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/frame_blending_reference.py
"""

import copy
import json
import sys
from math import floor
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from motion_blur_reference import common, finish, key, null, png, prop, solid, value_at  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "frame_blending"
PROPOSAL = ROOT / "verification" / "B-150a proposal"
TOLERANCE = 1e-12  # the times, one rounding from exact
PIXEL_TOLERANCE = 1e-6  # the frames, drawn in 32-bit numbers by the build
W, H = 8, 1
CLEAR = [0.0, 0.0, 0.0, 0.0]
GAP = "MEDIA_SEQUENCE_GAP"

# Three drawings of a bar four pixels wide, each a step further right and a different colour.
# Full and empty channels only, so the sRGB curve returns exactly 1 and 0.
DRAWINGS = {
    "bar_0001": [[(255, 0, 0, 255)] * 4 + [(0, 0, 0, 0)] * 4],
    "bar_0002": [[(0, 0, 0, 0)] * 2 + [(0, 0, 255, 255)] * 4 + [(0, 0, 0, 0)] * 2],
    "bar_0003": [[(0, 0, 0, 0)] * 4 + [(0, 255, 0, 255)] * 4],
}


def decoded(name):
    out = []
    for r, g, b, a in DRAWINGS[name][0]:
        assert {r, g, b, a} <= {0, 255}, "exact channels only"
        out.append([r / 255 * a / 255, g / 255 * a / 255, b / 255 * a / 255, a / 255])
    return out


def mix(a, b, w):
    """`a + w (b - a)`, number by number: equal pictures, or w = 0, give `a` bit for bit."""
    return [[x + w * (y - x) for x, y in zip(p, q)] for p, q in zip(a, b)]


# --- time -----------------------------------------------------------------------------------

def source_time(layer, n):
    t = ((n - layer["in_frame"]) * 100) / layer.get("time_stretch", 100) + layer["source_offset_frames"]
    f = floor(t)
    return t, f, t - f


def comp_of(project, cid):
    return next((c for c in project["compositions"] if c["id"] == cid), None)


def source_end(project, layer):
    """The first local frame past the layer's source, or None when it has no end (a still)."""
    if layer["kind"] == "composition":
        inner = comp_of(project, layer["composition_id"])
        return inner["start_frame"] + inner["duration_frames"]
    asset = next(a for a in project["assets"] if a["id"] == layer["asset_id"])
    if asset["kind"] == "still":
        return None
    return max(s["end_frame_exclusive"] for s in layer["exposure_spans"])


def drawing(project, layer, number, n, log):
    asset = next(a for a in project["assets"] if a["id"] == layer["asset_id"])
    path = asset["frames"].get(str(number))
    if path is None:
        log.append({"frame": n, "layer": layer["id"], "code": GAP, "drawing": number})
        return [CLEAR] * W
    return decoded(Path(path).stem)


def local_picture(project, layer, f, n, log):
    """P(f): the layer's step-1 picture at the whole local frame `f`."""
    if layer["kind"] == "composition":
        inner = comp_of(project, layer["composition_id"])
        if not inner["start_frame"] <= f < inner["start_frame"] + inner["duration_frames"]:
            return [CLEAR] * inner["width"]
        return render(project, inner["id"], f, [])
    spans = layer["exposure_spans"]
    here = next((s for s in spans if s["start_frame"] <= f < s["end_frame_exclusive"]), None)
    if here is None:
        return [CLEAR] * W
    a = drawing(project, layer, here["drawing_number"], n, log)
    dissolve = layer.get("drawing_dissolve", 0)
    s, e = here["start_frame"], here["end_frame_exclusive"]
    after = next((x for x in spans if x["start_frame"] == e), None)
    d = min(dissolve, e - s - 1)
    if after is None or d <= 0 or f < e - d:
        return a
    b = drawing(project, layer, after["drawing_number"], n, log)
    return mix(a, b, (f - (e - d) + 1) / (d + 1))


def blending(comp, layer):
    return comp.get("frame_blending", False) and layer.get("frame_blend") == "frame_mix"


def source(project, comp, layer, n, log):
    """The layer's step-1 picture at composition frame `n`."""
    kind = layer["kind"]
    if kind == "solid":
        s = layer["solid"]
        return [s["color"] + [1.0]] * s["width"]
    t, f, w = source_time(layer, n)
    a = local_picture(project, layer, f, n, log)
    end = source_end(project, layer)
    if not blending(comp, layer) or w == 0 or (end is not None and f + 1 >= end):
        return a
    return mix(a, local_picture(project, layer, f + 1, n, log), w)


def placed(pic, dx, width):
    """A slide by whole pixels, clear outside: the only move these cases make."""
    return [pic[i - dx] if 0 <= i - dx < len(pic) else CLEAR for i in range(width)]


def over(s, d):
    return [s[i] + d[i] * (1 - s[3]) for i in range(4)]


def render(project, comp_id, n, log):
    comp = comp_of(project, comp_id)
    layers = {l["id"]: l for l in comp["layers"]}
    frame = [CLEAR] * comp["width"]
    for lid in comp["layer_order"]:
        layer = layers[lid]
        if layer["kind"] == "null" or not layer["in_frame"] <= n < layer["out_frame"]:
            continue
        pic = source(project, comp, layer, n, log)
        x, y = value_at(layer["transform"]["position"], n)
        assert x == int(x) and y == 0, "whole pixels, sideways"
        pic = placed(pic, int(x), comp["width"])
        o = value_at(layer["transform"]["opacity"], n)
        frame = [over([c * o for c in p], d) for p, d in zip(pic, frame)]
    return frame


# --- the project files ----------------------------------------------------------------------

TWOS = [(0, 2, 1), (2, 4, 2), (4, 6, 3)]


def spans(rows):
    return [{"start_frame": s, "end_frame_exclusive": e, "drawing_number": d} for s, e, d in rows]


def raster(id="bar", rows=TWOS, position=(0, 0), stretch=None, mix=False, dissolve=0, **kw):
    record, t = common(id, "raster", list(position) if not isinstance(position, list) else position,
                       out_frame=kw.pop("out_frame", 12), **kw)
    record.update({"asset_id": "asset-bar", "source_offset_frames": kw.get("offset", 0),
                   "exposure_spans": spans(rows)})
    record = finish(record, t, kw)
    extra(record, stretch, mix, dissolve)
    return record


def extra(record, stretch, mix, dissolve):
    if stretch is not None:
        record["time_stretch"] = stretch
    if mix:
        record["frame_blend"] = "frame_mix"
    if dissolve:
        record["drawing_dissolve"] = dissolve


def nested(id="nest", inner="comp-inner", stretch=None, mix=False, **kw):
    record, t = common(id, "composition", [0, 0], out_frame=12, **kw)
    record.update({"composition_id": inner, "source_offset_frames": 0})
    record = finish(record, t, kw)
    extra(record, stretch, mix, 0)
    return record


def composition(id, layers, frames=12, blend=None):
    c = {"id": id, "name": id, "width": W, "height": H, "pixel_aspect_ratio": 1,
         "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
         "duration_frames": frames, "work_area": {"start_frame": 0, "end_frame_exclusive": frames},
         "layer_order": [l["id"] for l in layers], "layers": layers}
    if blend is not None:
        c["frame_blending"] = blend
    return c


def project(name, layers, blend=True, inner=None, missing=()):
    p = {"schema_version": 0, "project_id": "proj-" + name.lower(),
         "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
         "assets": [], "compositions": [composition("comp-main", layers, blend=blend)]}
    rasters = [l for l in layers + [l for c in (inner or []) for l in c["layers"]] if l["kind"] == "raster"]
    if rasters:
        numbers = sorted({s["drawing_number"] for l in rasters for s in l["exposure_spans"]})
        p["assets"].append({"id": "asset-bar", "kind": "image_sequence", "name": "bar",
                            "pattern": "bar_####.png",
                            "frames": {str(d): f"media/bar_{d:04d}.png" for d in numbers if d not in missing},
                            "interpretation": {"color_space": "srgb", "alpha": "straight"}})
    p["compositions"] += inner or []
    return p


MIXED = dict(stretch=200, mix=True)
INNER_BARS = composition("comp-inner", [raster(out_frame=6)], frames=6)
SLIDER = solid(id="dot", position=[key(0, [0, 0]), key(6, [6, 0])], width=2, color=[1, 1, 1],
               out_frame=8)
INNER_SLIDE = composition("comp-slide", [SLIDER], frames=8)
F12 = list(range(12))

PIXELS = {
    "FX-FBLEND-010": ("Stretch 100 with both switches on: every time is a whole frame, so the "
                      "drawings on twos, red, blue, green, exactly as with both switches off.",
                      project("FX-FBLEND-010", [raster(stretch=100, mix=True, out_frame=6)]), list(range(6))),
    "FX-FBLEND-011": ("Stretch 200, the layer's switch off: each drawing is held four frames, "
                      "red, blue, green, green.",
                      project("FX-FBLEND-011", [raster(stretch=200)]), F12),
    "FX-FBLEND-012": ("Stretch 200, both switches on: frames 3 and 7 fall half way between two "
                      "drawings and are half of each; frames 1 and 5 fall between two frames of "
                      "the same drawing and are that drawing, bit for bit; frames 9 and 11 would "
                      "mix with frame 6, past the last drawing, so they are green alone.",
                      project("FX-FBLEND-012", [raster(**MIXED)]), F12),
    "FX-FBLEND-013": ("Stretch 200, the layer's switch on and the composition's off: as FX-FBLEND-011.",
                      project("FX-FBLEND-013", [raster(**MIXED)], blend=False), F12),
    "FX-FBLEND-014": ("A file written before D-216, with none of the four fields: the drawings on "
                      "twos, and saved again it still has none.",
                      project("FX-FBLEND-014", [raster(out_frame=6)], blend=None), list(range(6))),
    "FX-FBLEND-015": ("Stretch 150, both switches on: the times go in thirds, so frames 1, 2, 4, "
                      "5, 7 and 8 are two thirds of one and a third of the next.",
                      project("FX-FBLEND-015", [raster(stretch=150, mix=True, out_frame=9)]), list(range(9))),
    "FX-FBLEND-016": ("Stretch 75, both switches on: sped up, frames 1 and 2 fall a third and "
                      "two thirds of the way on; frame 3 is local frame 4 exactly, and local "
                      "frame 3, stepped over, is never mixed in.",
                      project("FX-FBLEND-016", [raster(stretch=75, mix=True, out_frame=5)]), list(range(5))),
    "FX-FBLEND-017": ("A gap: nothing exposed on local frames 2 and 3. Stretch 200, both on: "
                      "frame 3 is red half mixed with nothing, so red at half covering; frames 4 "
                      "to 7 are empty; frame 7 mixes nothing with green.",
                      project("FX-FBLEND-017", [raster(rows=[(0, 2, 1), (4, 6, 3)], **MIXED)]), F12),
    "FX-FBLEND-018": ("Drawing 2 has no file. Stretch 200, both on: frames 3 and 7 mix with an "
                      "empty picture and say MEDIA_SEQUENCE_GAP, as frames 4 to 6 do; frames that "
                      "do not read drawing 2 say nothing. No other drawing stands in for it.",
                      project("FX-FBLEND-018", [raster(**MIXED)], missing=(2,)), F12),
    "FX-FBLEND-019": ("Source offset 1, stretch 200, both on: every time is half a frame on, "
                      "so frames 0 to 3 are red, half, blue, blue... one frame earlier than 012.",
                      project("FX-FBLEND-019", [raster(offset=1, **MIXED)]), F12),
    "FX-FBLEND-020": ("In point 3, stretch 200, both on: nothing before frame 3, and from there "
                      "FX-FBLEND-012's frames, three frames later. The stretch runs from the in point.",
                      project("FX-FBLEND-020", [raster(in_frame=3, **MIXED)]), F12),
    "FX-FBLEND-021": ("A composition layer showing a composition, 6 frames long, of the three "
                      "drawings on twos, stretched 200 with both switches on: FX-FBLEND-012's "
                      "frames. The composition's end is its source's end.",
                      project("FX-FBLEND-021", [nested(inner="comp-inner", **MIXED)], inner=[INNER_BARS]), F12),
    "FX-FBLEND-022": ("A composition layer showing a white dot 2 pixels wide moving right a pixel "
                      "a frame, stretched 200 with both on: on odd frames the inner composition "
                      "is drawn twice, a frame apart, and the two are half and half, so the dot "
                      "is 3 pixels with half-covered ends.",
                      project("FX-FBLEND-022", [nested(inner="comp-slide", **MIXED)], inner=[INNER_SLIDE]),
                      list(range(8))),
    "FX-FBLEND-023": ("Keys are not stretched: the layer's position is keyed from 0 at frame 0 "
                      "to 4 at frame 4, and it is at x = n on frame n while its drawings play at "
                      "half speed with Frame Mix.",
                      project("FX-FBLEND-023", [raster(position=[key(0, [0, 0]), key(4, [4, 0])], **MIXED)]),
                      list(range(5))),
    "FX-FBLEND-024": ("A file that writes the defaults: stretch 100 and the composition's switch "
                      "false. They read as absent, the drawings on twos, and saved again neither "
                      "field is written.",
                      project("FX-FBLEND-024", [raster(stretch=100, out_frame=6)], blend=False),
                      list(range(6))),
    "FX-FBLEND-030": ("Drawing Dissolve 1 on twos, stretch 100: the second frame of each hold is "
                      "half the drawing and half the next; the last drawing has no next and holds.",
                      project("FX-FBLEND-030", [raster(dissolve=1, out_frame=6)]), list(range(6))),
    "FX-FBLEND-031": ("Drawing Dissolve 2 on threes: a third, then two thirds, of the next.",
                      project("FX-FBLEND-031", [raster(rows=[(0, 3, 1), (3, 6, 2), (6, 9, 3)], dissolve=2,
                                                       out_frame=9)]), list(range(9))),
    "FX-FBLEND-032": ("Drawing Dissolve 5 on twos: a hold of two frames dissolves over one at "
                      "most, so the same frames as FX-FBLEND-030.",
                      project("FX-FBLEND-032", [raster(dissolve=5, out_frame=6)]), list(range(6))),
    "FX-FBLEND-033": ("Drawing Dissolve 3 on ones: every drawing is shown once, whole.",
                      project("FX-FBLEND-033", [raster(rows=[(0, 1, 1), (1, 2, 2), (2, 3, 3)], dissolve=3,
                                                       out_frame=3)]), list(range(3))),
    "FX-FBLEND-034": ("Drawing Dissolve 1 with a frame of nothing between red and blue: a hold "
                      "before a gap does not dissolve.",
                      project("FX-FBLEND-034", [raster(rows=[(0, 2, 1), (3, 5, 2)], dissolve=1,
                                                       out_frame=5)]), list(range(5))),
    "FX-FBLEND-035": ("Drawing Dissolve 1 where the same drawing is exposed twice running: frame 1 "
                      "is red, bit for bit.",
                      project("FX-FBLEND-035", [raster(rows=[(0, 2, 1), (2, 4, 1), (4, 6, 2)], dissolve=1,
                                                       out_frame=6)]), list(range(6))),
    "FX-FBLEND-036": ("Drawing Dissolve 1 into drawing 2, which has no file: frame 1 is red at "
                      "half and says MEDIA_SEQUENCE_GAP, as frames 2 and 3 do.",
                      project("FX-FBLEND-036", [raster(dissolve=1, out_frame=6)], missing=(2,)),
                      list(range(6))),
    "FX-FBLEND-037": ("Drawing Dissolve 1 with the composition's frame blending switch off: the "
                      "dissolve is the layer's own and still happens, as FX-FBLEND-030.",
                      project("FX-FBLEND-037", [raster(dissolve=1, out_frame=6)], blend=False), list(range(6))),
    "FX-FBLEND-038": ("Drawing Dissolve 1, stretch 200 and both switches on: the dissolved frames "
                      "are mixed again, so frame 1 is three quarters red and a quarter blue.",
                      project("FX-FBLEND-038", [raster(dissolve=1, **MIXED)]), F12),
    "FX-FBLEND-039": ("Drawing Dissolve 1 and stretch 200, the layer's switch off: each "
                      "dissolved frame is held two frames.",
                      project("FX-FBLEND-039", [raster(dissolve=1, stretch=200)]), F12),
}

TIMES = {
    "FX-FBLEND-001": ("Stretch 100, in 2, offset 1: t is n - 2 + 1, whole.", 100, 2, 1, [2, 3, 4, 7]),
    "FX-FBLEND-002": ("Stretch 200: half a frame of source a frame.", 200, 0, 0, [0, 1, 2, 3, 4, 5]),
    "FX-FBLEND-003": ("Stretch 50: two frames of source a frame.", 50, 0, 0, [0, 1, 2, 3]),
    "FX-FBLEND-004": ("Stretch 150: two thirds of a frame a frame.", 150, 0, 0, [0, 1, 2, 3, 4]),
    "FX-FBLEND-005": ("Stretch 300, in 10, offset 2.", 300, 10, 2, [10, 11, 12, 13]),
    "FX-FBLEND-006": ("Stretch 33.3, not a round share: f is 3, 6 and 9.", 33.3, 0, 0, [0, 1, 2, 3]),
    "FX-FBLEND-007": ("Stretch 10000, the most: a hundredth of a frame a frame.", 10000, 0, 0, [0, 1, 2, 99, 100]),
    "FX-FBLEND-008": ("Stretch 1, the least: a hundred frames a frame.", 1, 5, 0, [5, 6, 7]),
}

COMMANDS = {
    "FX-FBLEND-040": ("In 0, out 6, 100 to 200: the out point goes to 12.", 0, 6, 100, 200),
    "FX-FBLEND-041": ("In 10, out 17, 100 to 150: 7 frames become 10.5, rounded half away to 11.", 10, 17, 100, 150),
    "FX-FBLEND-042": ("In 0, out 12, 200 to 50: 12 frames become 3.", 0, 12, 200, 50),
    "FX-FBLEND-043": ("In 4, out 10, 100 to 1: 6 frames become 0.06, and a layer keeps one frame.", 4, 10, 100, 1),
    "FX-FBLEND-044": ("In 0, out 5, 300 to 100: 5 frames become 1.666..., rounded to 2.", 0, 5, 300, 100),
}


def stretched_out(i, o, s0, s1):
    """Where the Time Stretch command puts the out point; the in point stays."""
    x = (o - i) * s1 / s0
    whole = floor(x + 0.5) if x >= 0 else -floor(-x + 0.5)
    return i + max(1, whole)


def refusals():
    """Files a build must refuse whole, as PROJECT_SCHEMA_INVALID, each FX-FBLEND-012's (the last,
    FX-FBLEND-021's) with one change."""
    def edit(change, base="FX-FBLEND-012"):
        p = copy.deepcopy(PIXELS[base][1])
        change(p["compositions"][0])
        return p

    def lay(k, v):
        return lambda c: c["layers"][0].__setitem__(k, v)

    def add(record):
        return lambda c: (c["layers"].append(record), c["layer_order"].append(record["id"]))

    adjustment = solid(id="adj", position=[0, 0])
    adjustment["kind"] = "adjustment"
    del adjustment["solid"]
    adjustment["frame_blend"] = "frame_mix"
    still_solid = solid(id="card", position=[0, 0], width=8)
    still_solid["time_stretch"] = 200
    rig = null(position=[0, 0])
    rig["frame_blend"] = "frame_mix"
    return {
        "FX-FBLEND-050": ("A stretch below 1.", edit(lay("time_stretch", 0.5))),
        "FX-FBLEND-051": ("A stretch above 10000.", edit(lay("time_stretch", 10001))),
        "FX-FBLEND-052": ("A stretch below 0, playing backwards, which is not part of this.",
                          edit(lay("time_stretch", -100))),
        "FX-FBLEND-053": ("A stretch written as a word.", edit(lay("time_stretch", "200"))),
        "FX-FBLEND-054": ("A layer's frame blending that is Pixel Motion, which is not part of this.",
                          edit(lay("frame_blend", "pixel_motion"))),
        "FX-FBLEND-055": ("A layer's frame blending written true rather than the word frame_mix.",
                          edit(lay("frame_blend", True))),
        "FX-FBLEND-056": ("A composition's switch that is not true or false.",
                          edit(lambda c: c.__setitem__("frame_blending", "yes"))),
        "FX-FBLEND-057": ("A drawing dissolve that is not a whole number of frames.",
                          edit(lay("drawing_dissolve", 1.5))),
        "FX-FBLEND-058": ("A drawing dissolve above 100 frames.", edit(lay("drawing_dissolve", 101))),
        "FX-FBLEND-059": ("A drawing dissolve below 0.", edit(lay("drawing_dissolve", -1))),
        "FX-FBLEND-060": ("A stretch on a solid layer, which has no drawings to time.", edit(add(still_solid))),
        "FX-FBLEND-061": ("Frame blending on a null layer.", edit(add(rig))),
        "FX-FBLEND-062": ("Frame blending on an adjustment layer.", edit(add(adjustment))),
        "FX-FBLEND-063": ("A drawing dissolve on a composition layer, which has no exposure of its own.",
                          edit(lay("drawing_dissolve", 1), "FX-FBLEND-021")),
    }


# --- printing -------------------------------------------------------------------------------

def fmt(v):
    return f"{v:.12g}"


def print_frames(frames, warnings):
    print("| frame | " + " | ".join(f"x = {x}" for x in range(W)) + " | says |")
    print("| --- " * (W + 2) + "|")
    for f, px in frames.items():
        said = ", ".join(sorted({w["code"] for w in warnings if str(w["frame"]) == f})) or "-"
        print(f"| {f} | " + " | ".join(" ".join(f"{v:.9g}" for v in p) for p in px) + f" | {said} |")
    print()


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))

    expected = {"tolerance": TOLERANCE, "pixel_tolerance": PIXEL_TOLERANCE, "width": W, "height": H,
                "times": {}, "cases": {}, "commands": {}, "refused": {}}

    print("Times. For each frame n, the source time t, its whole frame f and the share w of the "
          "next.\n")
    print("| case | stretch | in | offset | n | t | f | w |")
    print("| --- | --- | --- | --- | --- | --- | --- | --- |")
    for fx, (says, s, i, o, ns) in TIMES.items():
        layer = {"in_frame": i, "source_offset_frames": o, "time_stretch": s}
        rows = [dict(zip(("t", "f", "w"), source_time(layer, n)), n=n) for n in ns]
        expected["times"][fx] = {"says": says, "time_stretch": s, "in_frame": i,
                                 "source_offset_frames": o, "rows": rows}
        for r in rows:
            print(f"| {fx} | {fmt(s)} | {i} | {o} | {r['n']} | {fmt(r['t'])} | {r['f']} | {fmt(r['w'])} |")
    print()
    for fx, (says, *_) in TIMES.items():
        print(f"{fx}: {says}")
    print()

    print("Pictures. Each value is a pixel's red, green, blue and covering, linear and "
          "premultiplied; `says` is what the frame reports.\n")
    for fx, (says, proj, frames) in PIXELS.items():
        log = []
        rendered = {str(f): render(proj, "comp-main", f, log) for f in frames}
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(proj, indent=2) + "\n", encoding="utf-8")
        said = sorted({(w["frame"], w["code"]) for w in log})
        expected["cases"][fx] = {"says": says, "project": name, "frames": rendered,
                                 "diagnostics": [{"frame": f, "code": c} for f, c in said]}
        print(f"{fx}: {says}\n")
        print_frames(rendered, log)

    print("Commands. Time Stretch keeps the in point and moves the out point to "
          "in + (out - in) x new / old, rounded half away from zero, never less than one frame.\n")
    print("| case | in | out | from | to | out after |")
    print("| --- | --- | --- | --- | --- | --- |")
    for fx, (says, i, o, s0, s1) in COMMANDS.items():
        after = stretched_out(i, o, s0, s1)
        expected["commands"][fx] = {"says": says, "in_frame": i, "out_frame": o, "from": s0,
                                    "to": s1, "out_after": after}
        print(f"| {fx} | {i} | {o} | {s0} | {s1} | {after} |")
    print()
    for fx, (says, *_) in COMMANDS.items():
        print(f"{fx}: {says}")
    print()

    print("Refused whole, each as `PROJECT_SCHEMA_INVALID`; each is FX-FBLEND-012's file with one "
          "change (FX-FBLEND-063 is FX-FBLEND-021's):\n")
    for fx, (says, proj) in refusals().items():
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(proj, indent=2) + "\n", encoding="utf-8")
        expected["refused"][fx] = {"says": says, "project": name, "code": "PROJECT_SCHEMA_INVALID"}
        print(f"- {fx}: {says}")
    print()

    (OUT / "expected_frame_blending.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                      encoding="utf-8")
    checks(expected)
    draw_proposal()


def checks(e):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in e["cases"].items()}
    d = {fx: v["diagnostics"] for fx, v in e["cases"].items()}
    red, blue, green = (decoded(f"bar_000{i}") for i in (1, 2, 3))
    none = [CLEAR] * W

    def half(a, b, w=0.5):
        return mix(a, b, w)

    def frames(*pics):
        return {str(i): p for i, p in enumerate(pics)}

    twos = frames(red, red, blue, blue, green, green)
    assert c["FX-FBLEND-010"] == twos and c["FX-FBLEND-014"] == twos and c["FX-FBLEND-024"] == twos
    held = frames(*[red] * 4, *[blue] * 4, *[green] * 4)
    assert c["FX-FBLEND-011"] == held and c["FX-FBLEND-013"] == held
    mixed = frames(red, red, red, half(red, blue), blue, blue, blue, half(blue, green), *[green] * 4)
    assert c["FX-FBLEND-012"] == mixed and c["FX-FBLEND-021"] == mixed
    assert c["FX-FBLEND-012"]["3"][2] == [0.5, 0, 0.5, 1.0], "half red, half blue"
    assert c["FX-FBLEND-015"]["1"] == half(red, red, 2 / 3) == red
    assert c["FX-FBLEND-015"]["2"] == half(red, blue, (2 * 100 / 150) % 1)
    assert c["FX-FBLEND-016"]["1"] == half(red, blue, 100 / 75 - 1)
    assert c["FX-FBLEND-017"]["3"][0] == [0.5, 0, 0, 0.5] and c["FX-FBLEND-017"]["5"] == none
    assert [x["frame"] for x in d["FX-FBLEND-018"]] == [3, 4, 5, 6, 7]
    assert c["FX-FBLEND-018"]["3"] == half(red, none) and c["FX-FBLEND-018"]["7"] == half(none, green)
    assert all(not d[fx] for fx in c if fx not in ("FX-FBLEND-018", "FX-FBLEND-036"))
    assert c["FX-FBLEND-019"]["1"] == mixed["3"]
    assert all(c["FX-FBLEND-020"][str(n)] == none for n in range(3))
    assert all(c["FX-FBLEND-020"][str(n + 3)] == mixed[str(n)] for n in range(9))
    dot = c["FX-FBLEND-022"]["1"]
    assert [p[3] for p in dot][:4] == [0.5, 1.0, 0.5, 0.0]
    assert c["FX-FBLEND-023"]["2"] == [CLEAR] * 2 + red[:6]
    diss = frames(red, half(red, blue), blue, half(blue, green), green, green)
    assert c["FX-FBLEND-030"] == diss and c["FX-FBLEND-032"] == diss and c["FX-FBLEND-037"] == diss
    assert c["FX-FBLEND-031"]["1"] == half(red, blue, 1 / 3) and c["FX-FBLEND-031"]["2"] == half(red, blue, 2 / 3)
    assert c["FX-FBLEND-033"] == frames(red, blue, green)
    assert c["FX-FBLEND-034"] == frames(red, red, none, blue, blue)
    assert c["FX-FBLEND-035"]["1"] == red and c["FX-FBLEND-035"]["3"] == half(red, blue)
    assert [x["frame"] for x in d["FX-FBLEND-036"]] == [1, 2, 3]
    assert c["FX-FBLEND-038"]["1"] == half(red, half(red, blue))
    assert c["FX-FBLEND-038"]["1"][2] == [0.75, 0, 0.25, 1.0]
    assert c["FX-FBLEND-039"]["2"] == c["FX-FBLEND-039"]["3"] == half(red, blue)
    t = {fx: v["rows"] for fx, v in e["times"].items()}
    assert [r["t"] for r in t["FX-FBLEND-001"]] == [1, 2, 3, 6]
    assert [r["w"] for r in t["FX-FBLEND-002"]] == [0, 0.5] * 3
    assert [r["f"] for r in t["FX-FBLEND-006"]] == [0, 3, 6, 9]
    assert [r["t"] for r in t["FX-FBLEND-008"]] == [0, 100, 200]
    assert [v["out_after"] for v in e["commands"].values()] == [12, 21, 3, 5, 2]


# --- the proposal's pictures ----------------------------------------------------------------

def draw_proposal():
    """A ball bouncing in six drawings on twos, played four ways by the rule above in two
    dimensions: a film and a contact sheet. Not fixtures: they are there to be judged by eye."""
    import numpy as np
    from PIL import Image, ImageDraw, ImageFont

    pw, ph, label = 220, 170, 34
    paper = np.array([0.913, 0.871, 0.776])  # #f4efe4 in linear light, roughly

    def cel(i):
        """Drawing i of 6 (0-based): the ball's height and squash, drawn at 4x and taken down."""
        s = 4
        img = Image.new("RGBA", (pw * s, ph * s), (0, 0, 0, 0))
        pen = ImageDraw.Draw(img)
        heights = [0.0, 0.55, 0.9, 1.0, 0.9, 0.55]
        squash = [0.72, 1.12, 1.0, 1.0, 1.0, 1.12]
        x = 30 + i * 32
        r = 20
        ground = ph - 22
        cy = ground - r - heights[i] * 100
        rx, ry = r / squash[i] ** 0.5, r * squash[i] ** 0.5
        if i == 0:
            cy = ground - ry
        box = [(x - rx) * s, (cy - ry) * s, (x + rx) * s, (cy + ry) * s]
        pen.ellipse(box, fill=(235, 90, 40, 255), outline=(40, 24, 20, 255), width=3 * s)
        hl = [(x - rx * 0.45) * s, (cy - ry * 0.55) * s, (x - rx * 0.05) * s, (cy - ry * 0.2) * s]
        pen.ellipse(hl, fill=(255, 220, 190, 255))
        pen.line([(8 * s, (ground + 2) * s), ((pw - 8) * s, (ground + 2) * s)], fill=(60, 50, 40, 255), width=2 * s)
        img = img.resize((pw, ph), Image.LANCZOS)
        a = np.asarray(img, dtype=np.float64) / 255
        rgb = a[..., :3]
        lin = np.where(rgb <= 0.04045, rgb / 12.92, ((rgb + 0.055) / 1.055) ** 2.4)
        return np.concatenate([lin * a[..., 3:4], a[..., 3:4]], axis=-1)

    cels = [cel(i) for i in range(6)]
    empty = np.zeros_like(cels[0])

    # The ball's exposure: six drawings on twos, over and over.
    ex = [{"start_frame": 2 * k, "end_frame_exclusive": 2 * k + 2, "drawing_number": k % 6 + 1}
          for k in range(40)]

    def layer_pic(stretch, mixing, dissolve, n):
        def P(f):
            here = next((s for s in ex if s["start_frame"] <= f < s["end_frame_exclusive"]), None)
            if here is None:
                return empty
            a = cels[here["drawing_number"] - 1]
            s, e = here["start_frame"], here["end_frame_exclusive"]
            after = next((x for x in ex if x["start_frame"] == e), None)
            dd = min(dissolve, e - s - 1)
            if after is None or dd <= 0 or f < e - dd:
                return a
            return a + ((f - (e - dd) + 1) / (dd + 1)) * (cels[after["drawing_number"] - 1] - a)

        t = n * 100 / stretch
        f = floor(t)
        w = t - f
        a = P(f)
        return a if not mixing or w == 0 else a + w * (P(f + 1) - a)

    def show(pic):
        rgb = pic[..., :3] + paper * (1 - pic[..., 3:4])
        srgb = np.where(rgb <= 0.0031308, rgb * 12.92, 1.055 * np.clip(rgb, 0, 1) ** (1 / 2.4) - 0.055)
        return Image.fromarray((np.clip(srgb, 0, 1) * 255 + 0.5).astype(np.uint8))

    panels = [
        ("As drawn: on twos", (100, False, 0)),
        ("Stretched 200%, no blending", (200, False, 0)),
        ("Stretched 200%, Frame Mix", (200, True, 0)),
        ("Drawing Dissolve 1, at 100%", (100, False, 1)),
    ]
    try:
        font = ImageFont.load_default(size=15)
    except TypeError:
        font = ImageFont.load_default()
    PROPOSAL.mkdir(parents=True, exist_ok=True)

    # The film: 48 frames, the four side by side, about 24 frames a second.
    film = []
    for n in range(48):
        img = Image.new("RGB", (pw * 4 + 30, ph + label), (28, 34, 48))
        pen = ImageDraw.Draw(img)
        for i, (text, (s, m, dv)) in enumerate(panels):
            img.paste(show(layer_pic(s, m, dv, n)), (i * (pw + 10), label))
            pen.text((i * (pw + 10) + 6, 8), text, fill=(230, 230, 230), font=font)
        pen.text((pw * 4 + 30 - 70, 8), f"frame {n:2d}", fill=(150, 160, 180), font=font)
        film.append(img)
    film[0].save(PROPOSAL / "frame_blending.gif", save_all=True, append_images=film[1:],
                 duration=42, loop=0)

    # The contact sheet: frames 0 to 11 of each, one row each, small.
    tw, th = pw // 2, ph // 2
    rows = len(panels)
    sheet = Image.new("RGB", (12 * (tw + 4) + 4, rows * (th + label) + 4), (28, 34, 48))
    pen = ImageDraw.Draw(sheet)
    for r, (text, (s, m, dv)) in enumerate(panels):
        y = r * (th + label) + label
        pen.text((6, y - label + 10), text + " - frames 0 to 11", fill=(230, 230, 230), font=font)
        for n in range(12):
            sheet.paste(show(layer_pic(s, m, dv, n)).resize((tw, th), Image.LANCZOS), (4 + n * (tw + 4), y))
    sheet.save(PROPOSAL / "frame_blending.png")


if __name__ == "__main__":
    main()
