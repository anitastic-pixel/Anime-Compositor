"""Motion blur, worked a second way.

D-188 proposes that a layer whose motion-blur switch is on, in a composition whose motion blur is
on, is drawn at several moments inside the frame's shutter and the drawings averaged. For frame
`n`, a shutter angle `A` in degrees, a shutter phase `P` in degrees and `N` samples, the moments
are

    t_k = n + P / 360 + (A / 360) * (k + 1/2) / N,     k = 0 .. N-1

Only where the layer is moves with `t`: its transform, its parents' transforms and the camera are
read at `t_k`, by document 20's rules for a key segment read at a fraction of a frame. **Everything
else holds at the whole frame `n`**: the drawing its exposure shows, its in and out points, its
masks, its effects and its opacity. The `N` pictures are averaged in linear, premultiplied light,
then the layer's matte, opacity and blend are applied once. A layer whose `N` transforms are all
the same is drawn once, so a still layer is the same picture, bit for bit, as with its switch off.

**This file never runs the build's code path.** It reads the very project files it writes and
draws each one-pixel-high frame pixel by pixel from document 21, moving a drawing by resampling
it from pixel centres, where the build folds the transform and the camera into one matrix.

Four kinds of case. The times cases are the moments a frame is drawn at. The values cases are a
property read between two frames. The pixel cases are a composition 20 pixels by 1, where the
layers only slide sideways, and most of them by whole pixels, so every number can be checked by
counting. The point case is where a drawing's corners land at each moment when a turning parent,
an eased key, a curved path and a camera dolly all move at once. Then there are the files a
build must refuse.

The projects go into `Fixtures/motion_blur`, the expected numbers into
`Fixtures/motion_blur/expected_motion_blur.json`, and the picture in the proposal into
`verification/B-124a proposal/motion_blur.png`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/motion_blur_reference.py
"""

import copy
import json
import struct
import sys
import zlib
from math import floor
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from ease_reference import EASY_EASE, ease  # noqa: E402
from parent_reference import forward  # noqa: E402
from path_reference import de_casteljau  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "motion_blur"
PICTURE = ROOT / "verification" / "B-124a proposal" / "motion_blur.png"
TOLERANCE = 1e-12  # the times and values, one or two roundings from exact
POINT_TOLERANCE = 1e-9  # the corners, carried through a chain and a camera in 64 bits
PIXEL_TOLERANCE = 1e-6  # the frames, drawn in 32-bit numbers by the build
W, H = 20, 1
BAR = [1.0, 0.5, 0.25]  # the solid's colour, linear: halves and quarters of it stay exact
OFF = {"enabled": False, "shutter_angle": 180, "shutter_phase": -90, "samples": 16}


# --- time -----------------------------------------------------------------------------------

def sample_times(n, blur):
    """The moments frame `n` is drawn at. One, the frame itself, when the blur is off."""
    if not blur["enabled"] or blur["shutter_angle"] == 0:
        return [n]
    a, p, count = blur["shutter_angle"], blur["shutter_phase"], blur["samples"]
    return [n + p / 360 + (a / 360) * (k + 0.5) / count for k in range(count)]


def thirds(a, b, sign):
    return [sign * (b[i] - a[i]) / 3 for i in range(len(a))]


def value_at(prop, t):
    """Document 20's rules, at a time that may fall between two frames."""
    keys = prop.get("keyframes") or []
    if not keys:
        return prop["base"]
    if t <= keys[0]["frame"]:
        return keys[0]["value"]
    if t >= keys[-1]["frame"]:
        return keys[-1]["value"]
    for a, b in zip(keys, keys[1:]):
        if a["frame"] <= t < b["frame"]:
            if t == a["frame"] or a["interp"] == "hold":
                return a["value"]
            u = (t - a["frame"]) / (b["frame"] - a["frame"])
            e = ease(a["ease"], u) if a["interp"] == "ease" else u
            v0, v1 = a["value"], b["value"]
            if "spatial" in a or "spatial" in b:
                out = a["spatial"][2:] if "spatial" in a else thirds(v0, v1, 1)
                inn = b["spatial"][:2] if "spatial" in b else thirds(v0, v1, -1)
                return de_casteljau(v0, [v0[i] + out[i] for i in range(2)],
                                    [v1[i] + inn[i] for i in range(2)], v1, e)
            if isinstance(v0, list):
                return [x + e * (y - x) for x, y in zip(v0, v1)]
            return v0 + e * (v1 - v0)
    raise AssertionError("unreachable")


# --- where a layer is -----------------------------------------------------------------------

def transform_at(layer, t):
    return {k: value_at(layer["transform"][k], t) for k in ("anchor", "position", "scale", "rotation")}


def to_comp(layers, layer, p, t):
    """A point of the layer carried through its own transform and each parent's, at `t`."""
    while layer is not None:
        p = forward(transform_at(layer, t), p)
        layer = layers.get(layer.get("parent"))
    return p


def to_screen(comp, layers, layer, p, t):
    """D-58's camera, read at `t` as the layer is. Every plane in these cases is at depth 0."""
    q = to_comp(layers, layer, p, t)
    cam = comp.get("camera")
    if cam is None:
        return q
    eye, depth, zoom = (value_at(cam[k], t) for k in ("position", "depth", "zoom"))
    s = zoom / (0 - depth)
    centre = [comp["width"] / 2, comp["height"] / 2]
    return [centre[i] + (q[i] - eye[i]) * s for i in range(2)]


def slide(comp, layers, layer, t):
    """How far right the layer's pixels land at `t`. The pixel cases only ever slide."""
    p0 = to_screen(comp, layers, layer, [0, 0], t)
    p1 = to_screen(comp, layers, layer, [1, 0], t)
    p2 = to_screen(comp, layers, layer, [0, 1], t)
    assert p0[1] == 0 and p1 == [p0[0] + 1, 0] and p2 == [p0[0], 1], "a slide and nothing else"
    return p0[0]


# --- drawings -------------------------------------------------------------------------------

def png(pixels):
    """An 8-bit straight RGBA PNG, rows top to bottom."""
    w, h = len(pixels[0]), len(pixels)
    raw = b"".join(b"\0" + bytes(v for px in row for v in px) for row in pixels)

    def chunk(kind, data):
        return (struct.pack(">I", len(data)) + kind + data
                + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF))

    return (b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 6, 0, 0, 0))
            + chunk(b"IDAT", zlib.compress(raw, 9)) + chunk(b"IEND", b""))


# Two drawings of the same bar, four pixels by one: red, then blue. Full and empty channels only,
# so the sRGB curve returns exactly 1 and 0.
DRAWINGS = {
    "bar_0001": [[(255, 0, 0, 255)] * 4],
    "bar_0002": [[(0, 0, 255, 255)] * 4],
}


def decoded(name):
    out = []
    for r, g, b, a in DRAWINGS[name][0]:
        assert {r, g, b, a} <= {0, 255}, "exact channels only"
        out.append([r / 255, g / 255, b / 255, a / 255])
    return out


def source(project, layer, n):
    """The layer's own picture at the whole frame `n`: drawings, shapes and nested frames hold."""
    kind = layer["kind"]
    if kind == "solid":
        s = layer["solid"]
        assert s["height"] == 1
        return [s["color"] + [1.0]] * s["width"]
    if kind == "raster":
        local = n - layer["in_frame"] + layer["source_offset_frames"]
        span = next(s for s in layer["exposure_spans"]
                    if s["start_frame"] <= local < s["end_frame_exclusive"])
        asset = next(a for a in project["assets"] if a["id"] == layer["asset_id"])
        path = asset["frames"][str(span["drawing_number"])]
        return decoded(Path(path).stem)
    if kind == "shape":
        (shape,) = layer["shapes"]
        xs = [p["point"][0] for p in shape["path"]["base"]["points"]]
        ys = [p["point"][1] for p in shape["path"]["base"]["points"]]
        assert min(ys) == 0 and max(ys) == 1 and all(x == int(x) for x in xs), "whole pixels"
        fill = shape["fill"]["color"] + [1.0]
        return [fill if min(xs) <= x < max(xs) else [0.0] * 4 for x in range(W)]
    if kind == "composition":
        local = n - layer["in_frame"] + layer["source_offset_frames"]
        return render(project, layer["composition_id"], local)
    raise AssertionError(kind)


def shifted(src, dx):
    """Document 21's resampling for a slide: bilinear from pixel centres, clear outside."""
    clear = [0.0] * 4
    out = []
    for i in range(W):
        s = i - dx  # the output pixel's centre, in the source's pixel-centre numbering
        j = floor(s)
        f = s - j
        a = src[j] if 0 <= j < len(src) else clear
        b = src[j + 1] if 0 <= j + 1 < len(src) else clear
        out.append([(1 - f) * p + f * q for p, q in zip(a, b)])
    return out


def picture(project, comp, layers, layer, n, blur):
    """One layer, moved and blurred, before its matte, opacity and blend."""
    if not layer["in_frame"] <= n < layer["out_frame"]:
        return None
    src = source(project, layer, n)
    times = sample_times(n, blur) if layer.get("motion_blur") else [n]
    slides = [slide(comp, layers, layer, t) for t in times]
    if all(s == slides[0] for s in slides):
        return shifted(src, slides[0])  # drawn once
    drawn = [shifted(src, s) for s in slides]
    # Summed in the order of k, then divided once.
    return [[sum(d[i][c] for d in drawn) / len(drawn) for c in range(4)] for i in range(W)]


def over(s, d):
    return [s[i] + d[i] * (1 - s[3]) for i in range(4)]


def render(project, comp_id, n):
    comp = next(c for c in project["compositions"] if c["id"] == comp_id)
    blur = comp.get("motion_blur", OFF)
    layers = {l["id"]: l for l in comp["layers"]}
    hidden = {l["matte"]["layer_id"] for l in comp["layers"] if l.get("matte") and l["matte"]["matte_only"]}
    frame = [[0.0] * 4 for _ in range(comp["width"])]
    for lid in comp["layer_order"]:
        layer = layers[lid]
        if lid in hidden or layer["kind"] == "null":
            continue
        pic = picture(project, comp, layers, layer, n, blur)
        if pic is None:
            continue
        if layer.get("matte"):
            m = picture(project, comp, layers, layers[layer["matte"]["layer_id"]], n, blur)
            pic = [[c * q[3] for c in p] for p, q in zip(pic, m or [[0.0] * 4] * W)]
        o = value_at(layer["transform"]["opacity"], n)
        frame = [over([c * o for c in p], d) for p, d in zip(pic, frame)]
    return frame[:W]


# --- the project files ----------------------------------------------------------------------

def prop(base, keys=None):
    return {"base": base, "keyframes": keys or []}


def key(frame, value, interp="linear", **extra):
    return {"frame": frame, "value": value, "interp": interp, **extra}


# The bar's travel: 16 pixels a frame, its left edge at 6 on frame 1. Half a frame of shutter in
# four samples puts it at 3, 5, 7 and 9 there, whole pixels, so every sum is a count.
MOVING = [key(0, [-10, 0]), key(2, [22, 0])]
STILL = [6, 0]


def transform(position, opacity=None, anchor=(0, 0)):
    return {"anchor": prop(list(anchor)),
            "position": prop(position) if isinstance(position[0], (int, float)) else prop(position[0]["value"], position),
            "scale": prop([100, 100]), "rotation": prop(0), "opacity": opacity or prop(1)}


def common(id, kind, position, **kw):
    record = {"id": id, "kind": kind, "name": id, "enabled": True, "locked": False,
              "in_frame": kw.get("in_frame", 0), "out_frame": kw.get("out_frame", 3)}
    return record, transform(position, kw.get("opacity"), kw.get("anchor", (0, 0)))


def finish(record, t, kw):
    record["transform"] = t
    record.update({"mask": None, "matte": kw.get("matte"), "blend_mode": "normal", "effects": []})
    if "parent" in kw:
        record["parent"] = kw["parent"]
    if kw.get("blur"):
        record["motion_blur"] = True
    return record


def solid(id="bar", position=MOVING, width=4, color=BAR, **kw):
    record, t = common(id, "solid", position, **kw)
    record["solid"] = {"color": color, "width": width, "height": 1}
    return finish(record, t, kw)


def raster(id="bar", position=MOVING, **kw):
    record, t = common(id, "raster", position, **kw)
    record.update({"asset_id": "asset-bar", "source_offset_frames": 0,
                   "exposure_spans": [{"start_frame": 0, "end_frame_exclusive": 1, "drawing_number": 1},
                                      {"start_frame": 1, "end_frame_exclusive": 3, "drawing_number": 2}]})
    return finish(record, t, kw)


def shape(id="bar", position=MOVING, **kw):
    record, t = common(id, "shape", position, **kw)
    pts = [{"point": p, "in": [0, 0], "out": [0, 0]} for p in ([0, 0], [4, 0], [4, 1], [0, 1])]
    record["shapes"] = [{"name": "Bar", "enabled": True, "closed": True,
                         "path": {"base": {"points": pts}, "keyframes": []},
                         "fill": {"color": BAR, "opacity": 1.0}, "stroke": None}]
    record = finish(record, t, kw)
    del record["mask"]
    record["masks"] = []  # a shape layer keeps a list
    return record


def nested(id="bar", position=MOVING, **kw):
    record, t = common(id, "composition", position, **kw)
    record.update({"composition_id": "comp-bar", "source_offset_frames": 0})
    return finish(record, t, kw)


def null(id="rig", position=None, **kw):
    record, t = common(id, "null", position, anchor=(50, 50), **kw)
    return finish(record, t, kw)


def composition(id, width, layers, blur=None, camera=None):
    c = {"id": id, "name": id, "width": width, "height": H, "pixel_aspect_ratio": 1,
         "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
         "duration_frames": 3, "work_area": {"start_frame": 0, "end_frame_exclusive": 3},
         "layer_order": [l["id"] for l in layers], "layers": layers}
    if blur is not None:
        c["motion_blur"] = blur
    if camera is not None:
        c["camera"] = camera
    return c


def project(name, layers, blur=None, camera=None):
    p = {"schema_version": 0, "project_id": "proj-" + name.lower(),
         "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
         "assets": [], "compositions": [composition("comp-main", W, layers, blur, camera)]}
    if any(l["kind"] == "raster" for l in layers):
        p["assets"].append({"id": "asset-bar", "kind": "image_sequence", "name": "bar",
                            "pattern": "bar_####.png",
                            "frames": {"1": "media/bar_0001.png", "2": "media/bar_0002.png"},
                            "interpretation": {"color_space": "srgb", "alpha": "straight"}})
    if any(l["kind"] == "composition" for l in layers):
        p["compositions"].append(composition("comp-bar", 4, [solid(position=[0, 0])]))
    return p


def shutter(angle=180, phase=-90, samples=4, enabled=True):
    return {"enabled": enabled, "shutter_angle": angle, "shutter_phase": phase, "samples": samples}


ON = shutter()
ZOOM = W * 50 / 36  # D-58's default lens, standing that far in front of the depth-0 plane
PAN = {"position": prop([10, 0.5], [key(0, [26, 0.5]), key(2, [-6, 0.5])]),
       "depth": prop(-ZOOM), "zoom": prop(ZOOM)}
RIG = [key(0, [-16, 0]), key(2, [16, 0])]  # a null, its anchor at (50, 50), going 16 a frame
HELD = [key(0, [2, 0], "hold"), key(1, [12, 0])]
FADE = prop(0, [key(0, 0), key(2, 1)])
MATTE = {"layer_id": "m", "mode": "alpha", "matte_only": True}

PIXELS = {
    "FX-MB-010": ("A still bar with both switches on and the composition's shutter as added: it "
                  "is drawn once, the same picture, bit for bit, as with either switch off.",
                  project("FX-MB-010", [solid(position=STILL, blur=True)], shutter(samples=16)), [1]),
    "FX-MB-011": ("The bar moving 16 pixels a frame, both switches on, shutter 180, phase -90, 4 "
                  "samples: on frame 1 it is drawn at 3, 5, 7 and 9, and each pixel is the share "
                  "of the four that cover it.",
                  project("FX-MB-011", [solid(blur=True)], ON), [1]),
    "FX-MB-012": ("The same with the composition's switch off: sharp, at 6.",
                  project("FX-MB-012", [solid(blur=True)], shutter(enabled=False)), [1]),
    "FX-MB-013": ("The same with the composition's switch on and the layer's off: sharp.",
                  project("FX-MB-013", [solid()], ON), [1]),
    "FX-MB-014": ("A file written before motion blur, with neither field: sharp, and saved again "
                  "it still has neither.",
                  project("FX-MB-014", [solid()]), [1]),
    "FX-MB-015": ("Phase 0: the shutter opens on the frame, so the bar is drawn at 7, 9, 11 and "
                  "13, all of it ahead of where it is on the frame.",
                  project("FX-MB-015", [solid(blur=True)], shutter(phase=0)), [1]),
    "FX-MB-016": ("Shutter 360, phase -180: a whole frame of travel, drawn at 0, 4, 8 and 12, "
                  "so sixteen pixels are covered a quarter each.",
                  project("FX-MB-016", [solid(blur=True)], shutter(360, -180)), [1]),
    "FX-MB-017": ("Shutter 0 is motion blur off, whatever the phase: sharp, at 6.",
                  project("FX-MB-017", [solid(blur=True)], shutter(0, -90)), [1]),
    "FX-MB-018": ("The bar still and the camera panning 16 pixels a frame the other way: the "
                  "same picture as FX-MB-011, because the camera is read at each moment too.",
                  project("FX-MB-018", [solid(position=STILL, blur=True)], ON, PAN), [1]),
    "FX-MB-019": ("The bar still on a null that moves 16 pixels a frame: the same picture as "
                  "FX-MB-011. The bar's switch decides; a null has none.",
                  project("FX-MB-019", [null(position=RIG), solid(position=[56, 50], parent="rig", blur=True)], ON), [1]),
    "FX-MB-020": ("A held key: the bar sits at 2 and jumps to 12 on frame 1. Frames 0 and 2 are "
                  "sharp; on frame 1 half the moments are before the jump and half after, so it "
                  "is seen twice at half strength.",
                  project("FX-MB-020", [solid(position=HELD, blur=True)], ON), [0, 1, 2]),
    "FX-MB-021": ("The opacity keyed 0 to 1 over frames 0 to 2 is read at the whole frame: "
                  "FX-MB-011's picture at exactly half.",
                  project("FX-MB-021", [solid(opacity=FADE, blur=True)], ON), [1]),
    "FX-MB-022": ("The layer starts on frame 1: on frame 0 it is not there, and on frame 1 all "
                  "four moments are drawn, including the two before its in point.",
                  project("FX-MB-022", [solid(in_frame=1, blur=True)], ON), [0, 1]),
    "FX-MB-023": ("Drawings hold: the red drawing is exposed on frame 0 and the blue from frame "
                  "1. On frame 1 every moment shows the blue, and no red, although two of the "
                  "moments fall in frame 0.",
                  project("FX-MB-023", [raster(blur=True)], ON), [1]),
    "FX-MB-024": ("A still matte over columns 0 to 7 cuts the moving bar after it is blurred.",
                  project("FX-MB-024", [solid(id="m", position=[0, 0], width=8, color=[1, 1, 1]),
                                        solid(matte=MATTE, blur=True)], ON), [1]),
    "FX-MB-025": ("A still bar with its switch on is drawn once; its matte moves with its own "
                  "switch on, so the matte is blurred and the bar is cut by the blurred matte.",
                  project("FX-MB-025", [solid(id="m", position=[key(0, [-14, 0]), key(2, [18, 0])],
                                              width=8, color=[1, 1, 1], blur=True),
                                        solid(position=STILL, matte=MATTE, blur=True)], ON), [1]),
    "FX-MB-026": ("As added: shutter 180, phase -90, 16 samples. The bar is drawn at 2.25, 2.75 "
                  "and on to 9.75, between pixels, so each moment is resampled as document 21 "
                  "says before the sixteen are averaged.",
                  project("FX-MB-026", [solid(blur=True)], shutter(samples=16)), [1]),
    "FX-MB-027": ("A shape layer whose one shape is the bar: the same picture as FX-MB-011.",
                  project("FX-MB-027", [shape(blur=True)], ON), [1]),
    "FX-MB-028": ("A composition layer showing a composition that holds the bar: the same "
                  "picture as FX-MB-011. The composition inside is drawn once, at the whole frame.",
                  project("FX-MB-028", [nested(blur=True)], ON), [1]),
}

TIMES = {
    "FX-MB-001": ("As added: shutter 180, phase -90, 16 samples, on frame 10.", 10, shutter(samples=16)),
    "FX-MB-002": ("Shutter 360, phase 0, 4 samples, on frame 0: the whole of frame 0.", 0, shutter(360, 0)),
    "FX-MB-003": ("Shutter 720, phase -360, 2 samples, on frame 5: a frame either side.", 5, shutter(720, -360, 2)),
    "FX-MB-004": ("Shutter 90, phase 90, 3 samples, on frame 0: after the frame, in thirds.", 0, shutter(90, 90, 3)),
    "FX-MB-005": ("Shutter 0: off, and the frame itself, whatever the phase and samples.", 7, shutter(0, 45, 8)),
}

EASED = [key(0, 0, "ease", ease=list(EASY_EASE)), key(4, 100)]
VALUES = {
    "FX-MB-006": ("A linear key from 0 at frame 0 to 100 at frame 4.",
                  prop(0, [key(0, 0), key(4, 100)]), [-0.25, 0, 1.25, 2.75, 4, 4.25]),
    "FX-MB-007": ("The same with easy ease: the value is 3u^2 - 2u^3 of the way.",
                  prop(0, EASED), [1, 2, 2.5, 3.875]),
    "FX-MB-008": ("A hold from 10 at frame 2 to 20 at frame 3: it changes at exactly frame 3.",
                  prop(10, [key(2, 10, "hold"), key(3, 20)]), [1.75, 2, 2.9375, 3, 3.0625]),
    "FX-MB-009": ("A position on a curved path, (0, 0) at frame 0 leaving straight down, (100, "
                  "100) at frame 2 arriving from the left.",
                  prop([0, 0], [key(0, [0, 0], spatial=[0, 0, 0, 60]),
                                key(2, [100, 100], spatial=[-60, 0, 0, 0])]), [0.25, 0.5, 1, 1.5]),
}


def refusals():
    """Files a build must refuse whole, as PROJECT_SCHEMA_INVALID, each FX-MB-011's with one change."""
    def edit(change):
        p = copy.deepcopy(PIXELS["FX-MB-011"][1])
        change(p["compositions"][0])
        return p

    def blur(k, v):
        return lambda c: c["motion_blur"].__setitem__(k, v)

    def drop(k):
        return lambda c: c["motion_blur"].pop(k)

    def layer(record):
        return lambda c: (c["layers"].append(record), c["layer_order"].append(record["id"]))

    adjustment = solid(id="adj", position=[0, 0], blur=True)
    adjustment["kind"] = "adjustment"
    del adjustment["solid"]
    return {
        "FX-MB-030": ("A shutter angle above 720.", edit(blur("shutter_angle", 721))),
        "FX-MB-031": ("A shutter angle below 0.", edit(blur("shutter_angle", -1))),
        "FX-MB-032": ("A shutter phase above 360.", edit(blur("shutter_phase", 361))),
        "FX-MB-033": ("One sample.", edit(blur("samples", 1))),
        "FX-MB-034": ("Sixty-five samples.", edit(blur("samples", 65))),
        "FX-MB-035": ("Samples that are not a whole number.", edit(blur("samples", 2.5))),
        "FX-MB-036": ("A composition's record with no samples.", edit(drop("samples"))),
        "FX-MB-037": ("A composition's record with a field this build does not know.",
                      edit(blur("adaptive_limit", 128))),
        "FX-MB-038": ("A composition's switch that is not true or false.", edit(blur("enabled", "yes"))),
        "FX-MB-039": ("A layer's switch that is not true or false.",
                      edit(lambda c: c["layers"][0].__setitem__("motion_blur", "on"))),
        "FX-MB-040": ("The switch on a null, which draws nothing.",
                      edit(layer(null(position=[50, 50], blur=True)))),
        "FX-MB-041": ("The switch on an adjustment layer.", edit(layer(adjustment))),
    }


# --- the point case -------------------------------------------------------------------------

def point_case():
    """A turning parent, an eased key, a curved path and a camera dolly, in 1920 by 1080."""
    parent = null(id="P", position=[960, 540])
    parent["transform"]["rotation"] = prop(0, [key(0, 0, "ease", ease=list(EASY_EASE)), key(24, 90)])
    child = solid(id="C", position=[key(0, [150, 50], spatial=[0, 0, 0, 100]),
                                    key(24, [250, 50], spatial=[0, 100, 0, 0])],
                  width=100, color=BAR, parent="P", blur=True)
    child["solid"]["height"] = 100
    camera = {"position": prop([960, 540]), "depth": prop(-1920, [key(0, -1920), key(24, -960)]),
              "zoom": prop(1920)}
    p = project("FX-MB-050", [parent, child], ON, camera)
    c = p["compositions"][0]
    c.update({"width": 1920, "height": 1080, "duration_frames": 25,
              "work_area": {"start_frame": 0, "end_frame_exclusive": 25}})
    for l in c["layers"]:
        l["out_frame"] = 25
    layers = {l["id"]: l for l in c["layers"]}
    rows = []
    for n in (0, 12):
        for t in sample_times(n, c["motion_blur"]):
            rows.append({"frame": n, "time": t,
                         "parent rotation": value_at(parent["transform"]["rotation"], t),
                         "child position": value_at(child["transform"]["position"], t),
                         "camera depth": value_at(camera["depth"], t),
                         "(0, 0) on screen": to_screen(c, layers, child, [0, 0], t),
                         "(100, 0) on screen": to_screen(c, layers, child, [100, 0], t)})
    return p, rows


# --- printing -------------------------------------------------------------------------------

def fmt(v):
    if isinstance(v, list):
        return " ".join(fmt(x) for x in v)
    return f"{v:.12g}"


def print_frames(frames):
    print("| frame | " + " | ".join(f"x = {x}" for x in range(W)) + " |")
    print("| --- " * (W + 1) + "|")
    for f, px in frames.items():
        print(f"| {f} | " + " | ".join(" ".join(f"{v:.9g}" for v in p) for p in px) + " |")
    print()


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))

    expected = {"tolerance": TOLERANCE, "point_tolerance": POINT_TOLERANCE,
                "pixel_tolerance": PIXEL_TOLERANCE, "width": W, "height": H,
                "times": {}, "values": {}, "cases": {}, "points": {}, "refused": {}}

    print("Times. Each row is the moments one frame is drawn at, first to last.\n")
    print("| case | frame | angle | phase | samples | moments |")
    print("| --- | --- | --- | --- | --- | --- |")
    for fx, (says, n, blur) in TIMES.items():
        ts = sample_times(n, blur)
        expected["times"][fx] = {"says": says, "frame": n, "motion_blur": blur, "times": ts}
        print(f"| {fx} | {n} | {blur['shutter_angle']} | {blur['shutter_phase']} | "
              f"{blur['samples']} | {', '.join(fmt(t) for t in ts)} |")
    print()
    for fx, (says, _, _) in TIMES.items():
        print(f"{fx}: {says}")
    print()

    print("Values. A property read between frames.\n")
    for fx, (says, p, ts) in VALUES.items():
        vs = [value_at(p, t) for t in ts]
        expected["values"][fx] = {"says": says, "property": p, "times": ts, "values": vs}
        print(f"{fx}: {says}\n")
        print("| time | " + " | ".join(fmt(t) for t in ts) + " |")
        print("| --- " * (len(ts) + 1) + "|")
        print("| value | " + " | ".join(fmt(v) for v in vs) + " |\n")

    print("Pictures. Each value is a pixel's red, green, blue and covering, linear and "
          "premultiplied.\n")
    for fx, (says, proj, frames) in PIXELS.items():
        rendered = {str(f): render(proj, "comp-main", f) for f in frames}
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(proj, indent=2) + "\n", encoding="utf-8")
        expected["cases"][fx] = {"says": says, "project": name, "frames": rendered}
        print(f"{fx}: {says}\n")
        print_frames(rendered)

    p, rows = point_case()
    (OUT / "fx_mb_050.json").write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    says = ("A 100-pixel square on a parent that turns with easy ease from 0 to 90 degrees over "
            "frames 0 to 24, moving along a curved path in the parent's space, under a camera "
            "that dollies from 1920 to 960 in front of it; shutter 180, phase -90, 4 samples. "
            "Before frame 0 every key holds its first value.")
    expected["points"]["FX-MB-050"] = {"says": says, "project": "fx_mb_050.json", "rows": rows}
    print(f"FX-MB-050: {says}\n")
    cols = list(rows[0].keys())
    print("| " + " | ".join(cols) + " |")
    print("| --- " * len(cols) + "|")
    for r in rows:
        print("| " + " | ".join(fmt(r[k]) for k in cols) + " |")
    print()

    print("Refused whole, each as `PROJECT_SCHEMA_INVALID`; each is FX-MB-011's file with one "
          "change:\n")
    for fx, (says, proj) in refusals().items():
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(proj, indent=2) + "\n", encoding="utf-8")
        expected["refused"][fx] = {"says": says, "project": name, "code": "PROJECT_SCHEMA_INVALID"}
        print(f"- {fx}: {says}")
    print()

    (OUT / "expected_motion_blur.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    checks(expected)
    draw_picture()


def checks(e):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in e["cases"].items()}
    t = {fx: v["times"] for fx, v in e["times"].items()}
    v = {fx: x["values"] for fx, x in e["values"].items()}

    def bar(at, colour=BAR, share=1.0):
        return [[x * share for x in colour + [1.0]] if at <= i < at + 4 else [0.0] * 4 for i in range(W)]

    sharp = bar(6)
    assert c["FX-MB-010"]["1"] == sharp
    blurred = c["FX-MB-011"]["1"]
    counts = [0, 0, 0, 1, 1, 2, 2, 2, 2, 2, 2, 1, 1, 0, 0, 0, 0, 0, 0, 0]
    assert blurred == [[x * n / 4 for x in BAR + [1.0]] for n in counts]
    for fx in ("FX-MB-012", "FX-MB-013", "FX-MB-014", "FX-MB-017"):
        assert c[fx]["1"] == sharp, fx
    for fx in ("FX-MB-018", "FX-MB-019", "FX-MB-027", "FX-MB-028"):
        assert c[fx]["1"] == blurred, fx
    assert [p[3] for p in c["FX-MB-015"]["1"]][7:17] == [0.25, 0.25] + [0.5] * 6 + [0.25, 0.25]
    assert [p[3] for p in c["FX-MB-016"]["1"]] == [0.25] * 16 + [0.0] * 4
    held = c["FX-MB-020"]
    assert held["0"] == bar(2) and held["2"] == bar(12)
    assert held["1"] == [[x * 0.5 for x in p] for p in [a if a[3] else b for a, b in zip(bar(2), bar(12))]]
    assert c["FX-MB-021"]["1"] == [[x / 2 for x in p] for p in blurred]
    assert c["FX-MB-022"]["0"] == [[0.0] * 4] * W and c["FX-MB-022"]["1"] == blurred
    assert all(p[0] == 0 for p in c["FX-MB-023"]["1"]), "red was drawn"
    assert [p[2] for p in c["FX-MB-023"]["1"]] == [p[3] for p in blurred]
    assert c["FX-MB-024"]["1"] == blurred[:8] + [[0.0] * 4] * 12
    assert c["FX-MB-025"]["1"] != sharp and all(p[3] == 0 for p in c["FX-MB-025"]["1"][10:])
    soft = c["FX-MB-026"]["1"]
    assert abs(sum(p[3] for p in soft) - 4) < 1e-12, "the bar's covering is kept"
    assert t["FX-MB-001"][0] == 9.765625 and t["FX-MB-001"][-1] == 10.234375
    assert t["FX-MB-002"] == [0.125, 0.375, 0.625, 0.875]
    assert t["FX-MB-003"] == [4.5, 5.5] and t["FX-MB-005"] == [7]
    assert v["FX-MB-006"] == [0, 0, 31.25, 68.75, 100, 100]
    assert abs(v["FX-MB-007"][0] - 15.625) < 1e-12 and abs(v["FX-MB-007"][1] - 50) < 1e-12
    assert v["FX-MB-008"] == [10, 10, 10, 20, 20]


# --- the picture in the proposal ------------------------------------------------------------

def draw_picture():
    """Seven strips, each the same square moving right and turning, drawn by the rule above in
    two dimensions. Not a fixture: it is there so the proposal can be judged by eye."""
    import numpy as np
    from PIL import Image, ImageDraw, ImageFont

    sw, sh, label = 640, 72, 20
    size = 36
    tile = np.zeros((size, size, 4))
    orange = np.array([0.871, 0.352, 0.051, 1.0])  # #f0a040 in linear light
    tile[:, :] = orange
    tile[:, 14:22] = [1.0, 1.0, 1.0, 1.0]  # a white stripe, so the turn shows
    bg = np.array([0.011, 0.016, 0.029])  # #1c2230 in linear light

    def draw(xf_at, times):
        ys, xs = np.mgrid[0:sh, 0:sw] + 0.5
        acc = np.zeros((sh, sw, 4))
        for t in times:
            xf = xf_at(t)
            px, py = _inverse(xf, xs, ys)
            sx, sy = px - 0.5, py - 0.5
            x0, y0 = np.floor(sx).astype(int), np.floor(sy).astype(int)
            fx, fy = sx - x0, sy - y0

            def at(x, y):
                ok = (x >= 0) & (x < size) & (y >= 0) & (y < size)
                out = np.zeros((sh, sw, 4))
                out[ok] = tile[y[ok], x[ok]]
                return out

            acc += ((1 - fx)[..., None] * (1 - fy)[..., None] * at(x0, y0)
                    + fx[..., None] * (1 - fy)[..., None] * at(x0 + 1, y0)
                    + (1 - fx)[..., None] * fy[..., None] * at(x0, y0 + 1)
                    + fx[..., None] * fy[..., None] * at(x0 + 1, y0 + 1))
        acc /= len(times)
        rgb = acc[..., :3] + bg * (1 - acc[..., 3:4])
        srgb = np.where(rgb <= 0.0031308, rgb * 12.92, 1.055 * np.clip(rgb, 0, 1) ** (1 / 2.4) - 0.055)
        return (np.clip(srgb, 0, 1) * 255 + 0.5).astype(np.uint8)

    move = {"anchor": prop([size / 2, size / 2]),
            "position": prop([0, 0], [key(0, [80, 36]), key(8, [560, 36])]),
            "scale": prop([100, 100]), "rotation": prop(0, [key(0, 0), key(8, 160)])}
    jump = {**move, "position": prop([0, 0], [key(0, [200, 36], "hold"), key(4, [440, 36])]),
            "rotation": prop(0, [key(0, 0, "hold"), key(4, 80)])}

    def xf_of(tr):
        return lambda t: {k: value_at(tr[k], t) for k in ("anchor", "position", "scale", "rotation")}

    strips = [
        ("Motion blur off", move, None),
        ("As added: shutter 180, phase -90, 16 samples", move, shutter(samples=16)),
        ("Shutter 360, phase -180: a whole frame of travel", move, shutter(360, -180, 16)),
        ("Shutter 90, phase -45: a quarter frame", move, shutter(90, -45, 16)),
        ("Phase 0: the shutter opens on the frame and the smear runs ahead", move, shutter(180, 0, 16)),
        ("4 samples: too few for this speed, and the steps show", move, shutter(samples=4)),
        ("Held keys: the jump on this frame is seen twice, half strength each", jump, shutter(samples=16)),
    ]
    img = Image.new("RGB", (sw, len(strips) * (sh + label)), (28, 34, 48))
    pen = ImageDraw.Draw(img)
    try:
        font = ImageFont.load_default(size=14)
    except TypeError:
        font = ImageFont.load_default()
    for i, (text, tr, blur) in enumerate(strips):
        times = sample_times(4, blur) if blur else [4]
        img.paste(Image.fromarray(draw(xf_of(tr), times)), (0, i * (sh + label) + label))
        pen.text((8, i * (sh + label) + 3), text, fill=(230, 230, 230), font=font)
    PICTURE.parent.mkdir(parents=True, exist_ok=True)
    img.save(PICTURE)


def _inverse(xf, xs, ys):
    """`parent_reference.backward` on whole arrays of points."""
    import numpy as np
    from math import cos, sin, radians
    c, s = cos(radians(xf["rotation"])), sin(radians(xf["rotation"]))
    x, y = xs - xf["position"][0], ys - xf["position"][1]
    x, y = c * x + s * y, -s * x + c * y
    x, y = x * 100 / xf["scale"][0], y * 100 / xf["scale"][1]
    return x + xf["anchor"][0], y + xf["anchor"][1]


if __name__ == "__main__":
    main()
