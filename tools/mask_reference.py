"""Masks with curves, several to a layer, worked a second way.

D-77 proposes what a mask is from B-24 on: a layer holds a list of them rather than one, each with
a path of points that may carry handles, a mode (Add, Subtract, Intersect, Difference, None),
invert, opacity, an even feather and an expansion. Today's polygon mask is the special case where
every handle is zero, one mask, mode Add, opacity 1, feather 0, expansion 0 - and this file's
first case exists to show that its pixels do not move.

**This file never runs the build's code path.** It renders each tiny frame pixel by pixel from
document 21 and ADR-016, and the outline sampler here is written from the rule rather than copied
from `src/mask.rs`.

B-24d adds the moving path to the same file, from the rule D-77 already states: a path is a
document 19 property, and its value at a frame is document 20's, point by point - the point and
both its handles - between keys that hold the same number of points. A case with keys is five
frames long and every one of its frames is pinned.

Every case is a composition 6 pixels by 2, so each frame is twelve pixels and can be printed. The
one drawing is written into `Fixtures/masks/media`, the projects into `Fixtures/masks`, the
expected frames and refusals into `Fixtures/masks/expected_masks.json`, a picture of the still
cases into `verification/B-24a proposal/mask_cases.png` and of the moving ones into
`verification/B-24d keys/mask_key_cases.png`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/mask_reference.py
"""

import json
import struct
import sys
import zlib
from copy import deepcopy
from math import ceil, exp, hypot
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import W, H, png, decoded, over, prop, fmt  # noqa: E402
from ease_reference import ease, EASE_IN_OUT  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "masks"
ROOT = Path(__file__).resolve().parent.parent
PICTURE = ROOT / "verification" / "B-24a proposal" / "mask_cases.png"
KEY_PICTURE = ROOT / "verification" / "B-24d keys" / "mask_key_cases.png"
KEY_FRAMES = 5  # a case with a moving path is five frames long
TOLERANCE = 1e-6
SAMPLES_PER_SIDE = 4  # ADR-016, unchanged
BG_PIXELS = [[(255, 0, 0, 255)] * W, [(128, 128, 128, 255)] * W]  # adjust_reference's "bg"
MAX_EXPANSION = 8192.0


# --- the outline ------------------------------------------------------------------------------

def flatten(points):
    """The closed outline as a list of (x, y).

    A segment whose two handles are both zero stays one straight line, which is why every mask
    written before D-77 keeps its pixels exactly. A curved segment is split into `n` straight
    pieces at equal steps of t, where n is the control polygon's length in pixels halved, rounded
    up, and held between 16 and 512. The length is summed point, out handle, in handle, point, in
    that order, so the count is the same on every machine.
    """
    out = []
    for i, p0 in enumerate(points):
        p1 = points[(i + 1) % len(points)]
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
        n = min(512, max(16, ceil(length / 2)))
        for k in range(1, n):
            t = k / n
            u = 1 - t
            out.append(tuple(u ** 3 * a[j] + 3 * u * u * t * b[j] + 3 * u * t * t * c[j]
                             + t ** 3 * d[j] for j in range(2)))
    return out


def point_inside(outline, x, y):
    """ADR-016's even-odd rule, unchanged: a ray along +x, half-open in y."""
    inside = False
    for i, (x0, y0) in enumerate(outline):
        x1, y1 = outline[(i + 1) % len(outline)]
        if (y0 <= y) != (y1 <= y):
            t = (y - y0) / (y1 - y0)
            if x < x0 + t * (x1 - x0):
                inside = not inside
    return inside


def distance_to_outline(outline, x, y):
    """The shortest distance from (x, y) to the outline itself, never signed."""
    best = float("inf")
    for i, (x0, y0) in enumerate(outline):
        x1, y1 = outline[(i + 1) % len(outline)]
        dx, dy = x1 - x0, y1 - y0
        run = dx * dx + dy * dy
        t = 0.0 if run == 0 else max(0.0, min(1.0, ((x - x0) * dx + (y - y0) * dy) / run))
        best = min(best, hypot(x - (x0 + t * dx), y - (y0 + t * dy)))
    return best


def sample_inside(outline, x, y, expansion):
    """Whether one sample point is inside the mask, grown or shrunk by `expansion` pixels.

    An expansion of zero is exactly today's rule and touches no distance. Otherwise the sample is
    inside when its signed distance to the outline, positive within, plus the expansion is zero or
    more; a corner therefore rounds off, as After Effects rounds one.
    """
    inside = point_inside(outline, x, y)
    if expansion == 0:
        return inside
    signed = distance_to_outline(outline, x, y) * (1 if inside else -1)
    return signed + expansion >= 0


def pixel_coverage(outline, x, y, expansion):
    """ADR-016: a 4x4 ordered grid, coverage the count inside over sixteen."""
    n = SAMPLES_PER_SIDE
    hits = 0
    for j in range(n):
        for i in range(n):
            if sample_inside(outline, x + (i + 0.5) / n, y + (j + 0.5) / n, expansion):
                hits += 1
    return hits / (n * n)


def blur_field(field, w, h, margin, sigma):
    """Document 21's Gaussian on a coverage field, separable and normalised, returning the frame.

    The field runs from -margin to W+margin each way, which is why nothing has to be invented
    outside the frame: the coverage there is worked out like any other.
    """
    if sigma == 0:
        return [field[(y + margin) * w + (x + margin)] for y in range(H) for x in range(W)]
    r = ceil(3 * sigma)
    one = [exp(-(d * d) / (2 * sigma * sigma)) for d in range(-r, r + 1)]
    one = [v / sum(one) for v in one]
    wide = []
    for y in range(h):
        for x in range(w):
            acc = 0.0
            for i in range(-r, r + 1):
                sx = min(w - 1, max(0, x + i))  # the field is wider than the blur needs
                acc += field[y * w + sx] * one[i + r]
            wide.append(acc)
    out = []
    for y in range(H):
        for x in range(W):
            acc = 0.0
            for j in range(-r, r + 1):
                sy = min(h - 1, max(0, y + margin + j))
                acc += wide[sy * w + (x + margin)] * one[j + r]
            out.append(acc)
    return out


def points_at(mask, frame):
    """A mask's path at a composition frame: document 20's rules, point by point.

    A path without keys is its base at every frame. With keys, the first key's shape holds before
    it and the last key's after it, a hold segment stays at the left key, and a linear or eased
    segment moves each point and each of its two handles the same fraction of the way - the
    fraction the ease gives on an eased segment, `u` itself on a linear one. Every key holds the
    same number of points as the base, which is why the points can be paired off at all; a file
    where they differ is refused whole (FX-MSK-030).
    """
    keys = mask.get("keys")
    if not keys:
        return mask["points"]
    if frame <= keys[0][0]:
        return keys[0][1]
    if frame >= keys[-1][0]:
        return keys[-1][1]
    a, b = next((a, b) for a, b in zip(keys, keys[1:]) if a[0] <= frame < b[0])
    if a[2] == "hold":
        return a[1]
    f = (frame - a[0]) / (b[0] - a[0])
    if a[2] != "linear":
        f = ease(a[2], f)
    return [{part: [x + (y - x) * f for x, y in zip(p[part], q[part])]
             for part in ("point", "in", "out")} for p, q in zip(a[1], b[1])]


def takes_part(mask, points):
    """A mask this build draws: enabled, at least three points, and its points do not cross.

    The crossing test is on the points alone, as it is today; handles are not tested, and a curve
    that crosses itself is drawn by the even-odd rule, which is defined for it. On a moving path
    it is the shape at the frame that is tested, so a path may be drawn on one frame and
    diagnosed on the next.
    """
    if not mask.get("enabled", True) or len(points) < 3:
        return False
    corners = [tuple(p["point"]) for p in points]
    if len(set(corners)) != len(corners):
        return False
    n = len(corners)
    for i in range(n):
        for j in range(i + 1, n):
            if j == i or (j + 1) % n == i or (i + 1) % n == j:
                continue
            if crosses(corners[i], corners[(i + 1) % n], corners[j], corners[(j + 1) % n]):
                return False
    return True


def side(a, b, p):
    v = (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0])
    return (v > 0) - (v < 0)


def crosses(a, b, c, d):
    return (side(a, b, c) * side(a, b, d) < 0) and (side(c, d, a) * side(c, d, b) < 0)


def mask_field(mask, points):
    """One mask's own coverage over the frame: expansion, then feather, then invert, then opacity."""
    outline = flatten(points)
    sigma = mask.get("feather", 0.0) / 2
    margin = ceil(3 * sigma) if sigma else 0
    w, h = W + 2 * margin, H + 2 * margin
    field = [pixel_coverage(outline, x - margin, y - margin, mask.get("expansion", 0.0))
             for y in range(h) for x in range(w)]
    field = blur_field(field, w, h, margin, sigma)
    if mask.get("inverted"):
        field = [1 - v for v in field]
    opacity = mask.get("opacity", 1.0)
    return [v * opacity for v in field]


MODES = {
    "add": lambda a, m: a + m - a * m,
    "subtract": lambda a, m: a * (1 - m),
    "intersect": lambda a, m: a * m,
    "difference": lambda a, m: a + m - 2 * a * m,
}


def coverage(masks, frame_no):
    """Every mask of a layer, first to last, into one coverage; None when none takes part.

    The accumulation starts at nothing, except that a first mask in Subtract or Intersect starts
    from the whole layer, because there would otherwise be nothing to take away from and the mask
    could only ever give an empty layer.
    """
    acc = None
    for mask in masks:
        points = points_at(mask, frame_no)
        if mask.get("mode", "add") == "none" or not takes_part(mask, points):
            continue
        m = mask_field(mask, points)
        if acc is None:
            start = 1.0 if mask.get("mode", "add") in ("subtract", "intersect") else 0.0
            acc = [start] * (W * H)
        rule = MODES[mask.get("mode", "add")]
        acc = [max(0.0, min(1.0, rule(a, v))) for a, v in zip(acc, m)]
    return acc


# --- the frame --------------------------------------------------------------------------------

def render(case, frame_no=0):
    frame = [[0.0] * 4 for _ in range(W * H)]
    for layer in case["layers"]:
        src = decoded("bg")
        acc = coverage(layer.get("masks", []), frame_no)
        if acc is not None:
            src = [[v * m for v in p] for p, m in zip(src, acc)]
        frame = [over(s, d) for s, d in zip(src, frame)]
    return frame


# --- the project files --------------------------------------------------------------------------

def points(*xy, handles=None):
    """A path from bare coordinates; `handles` gives (in, out) offsets in pixels for each point."""
    handles = handles or [([0, 0], [0, 0])] * len(xy)
    return [{"point": list(p), "in": list(h[0]), "out": list(h[1])} for p, h in zip(xy, handles)]


def path_json(mask):
    """The path as document 19 writes a property: a base, and keys where the path moves.

    The base of a moving path is its first key's shape, as every other keyed property in these
    fixtures writes it, so a build that ignored the keys would draw the first key.
    """
    record = prop({"points": mask["points"]})
    for frame, shape, interp in mask.get("keys", []):
        key = {"frame": frame, "value": {"points": shape},
               "interp": interp if isinstance(interp, str) else "ease"}
        if not isinstance(interp, str):
            key["ease"] = list(interp)
        record["keyframes"].append(key)
    return record


def mask_json(mask, n):
    return {
        "name": mask.get("name", f"Mask {n}"),
        "enabled": mask.get("enabled", True),
        "inverted": mask.get("inverted", False),
        "mode": mask.get("mode", "add"),
        "opacity": mask.get("opacity", 1.0),
        "feather_px": mask.get("feather", 0.0),
        "expansion_px": mask.get("expansion", 0.0),
        "path": path_json(mask),
    }


def layer_json(layer, frames):
    record = {
        "id": layer["id"], "kind": "raster", "name": layer["id"], "asset_id": "asset-bg",
        "enabled": True, "locked": False, "in_frame": 0, "out_frame": frames,
        "source_offset_frames": 0,
        "transform": {
            "anchor": prop([W / 2, H / 2]), "position": prop([W / 2, H / 2]),
            "scale": prop([100, 100]), "rotation": prop(0), "opacity": prop(1),
        },
        "exposure_spans": [],
    }
    if "old_mask" in layer:
        record["mask"] = layer["old_mask"]
    else:
        record["masks"] = [mask_json(m, n + 1) for n, m in enumerate(layer.get("masks", []))]
    record.update({"matte": None, "blend_mode": "normal", "effects": []})
    return record


def project_json(name, case):
    frames = case.get("frames", 3)
    return {
        "schema_version": 0,
        "project_id": "proj-" + name.lower(),
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-bg", "kind": "still", "name": "bg", "path": "media/bg.png",
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
            "id": "comp-main", "name": "Main", "width": W, "height": H,
            "pixel_aspect_ratio": 1, "frame_rate": {"numerator": 24, "denominator": 1},
            "start_frame": 0, "duration_frames": frames,
            "work_area": {"start_frame": 0, "end_frame_exclusive": frames},
            "layer_order": [l["id"] for l in case["layers"]],
            "layers": [layer_json(l, frames) for l in case["layers"]],
        }],
    }


# --- the cases --------------------------------------------------------------------------------

LEFT3 = points([0, 0], [3, 0], [3, 2], [0, 2])          # today's mask: columns 0 to 2
RIGHT3 = points([3, 0], [6, 0], [6, 2], [3, 2])         # columns 3 to 5
MIDDLE = points([2, 0], [4, 0], [4, 2], [2, 2])         # columns 2 and 3
DIAGONAL = points([0, 0], [4, 0], [0, 2])               # a sloped edge, partial coverage
INNER = points([1.5, 0.25], [4.5, 0.25], [4.5, 1.75], [1.5, 1.75])  # clear of the edges
K = 0.5522847498307936                                   # a circle's handle, four points
CIRCLE = points([3, 0], [4, 1], [3, 2], [2, 1], handles=[
    ([-K, 0], [K, 0]), ([0, -K], [0, K]), ([K, 0], [-K, 0]), ([0, K], [0, -K])])


BULGE = points([0, 0], [3, 0], [3, 2], [0, 2], handles=[  # LEFT3 with its right edge bellied out
    ([0, 0], [0, 0]), ([0, 0], [2, 0]), ([2, 0], [0, 0]), ([0, 0], [0, 0])])


def keyed(*keys):
    """A mask whose path moves: keys of (frame, points) or (frame, points, interp), where interp
    is "hold", "linear" (the default) or the four numbers of an ease."""
    keys = [(k[0], k[1], k[2] if len(k) > 2 else "linear") for k in keys]
    return {"points": keys[0][1], "keys": keys}


def layer(*masks, old=None):
    if old is not None:
        # The conversion D-77 states, written out: one Add mask at full opacity, no feather, no
        # expansion, every handle zero, the two flags as the old record held them.
        return {"layers": [{"id": "cel", "old_mask": old, "masks": [{
            "points": points(*old["vertices"]),
            "enabled": old["enabled"], "inverted": old["inverted"]}]}]}
    return {"layers": [{"id": "cel", "masks": list(masks)}]}


def moving(*masks):
    """A case whose path moves, and which is therefore five frames long rather than three."""
    return {**layer(*masks), "frames": KEY_FRAMES}


CASES = {
    "FX-MSK-001": ("Today's rectangle written the new way: the left three columns, and every "
                   "number the same as before D-77.",
                   layer({"points": LEFT3})),
    "FX-MSK-002": ("The same file written the old way, with one `mask` key: read as one Add mask, "
                   "the same frame as FX-MSK-001.",
                   layer(old={"vertices": [[0, 0], [3, 0], [3, 2], [0, 2]],
                              "enabled": True, "inverted": False})),
    "FX-MSK-003": ("A sloped edge: coverage in whole sixteenths.",
                   layer({"points": DIAGONAL})),
    "FX-MSK-004": ("Inverted: the three columns the mask keeps are the ones it now cuts.",
                   layer({"points": LEFT3, "inverted": True})),
    "FX-MSK-005": ("At half opacity: the kept columns are half there.",
                   layer({"points": LEFT3, "opacity": 0.5})),
    "FX-MSK-006": ("Two masks, the second Add: both halves, so the whole frame.",
                   layer({"points": LEFT3}, {"points": RIGHT3, "mode": "add"})),
    "FX-MSK-007": ("Two masks, the second Subtract: the left three columns with its middle "
                   "column taken out.",
                   layer({"points": LEFT3}, {"points": MIDDLE, "mode": "subtract"})),
    "FX-MSK-008": ("Two masks, the second Intersect: only where both are, column 2.",
                   layer({"points": LEFT3}, {"points": MIDDLE, "mode": "intersect"})),
    "FX-MSK-009": ("Two masks, the second Difference: where one is but not both.",
                   layer({"points": LEFT3}, {"points": MIDDLE, "mode": "difference"})),
    "FX-MSK-010": ("A second mask in mode None takes no part: the frame of FX-MSK-001.",
                   layer({"points": LEFT3}, {"points": RIGHT3, "mode": "none"})),
    "FX-MSK-011": ("A first mask in Subtract takes from the whole layer: a hole in columns 2 "
                   "and 3.",
                   layer({"points": MIDDLE, "mode": "subtract"})),
    "FX-MSK-012": ("Expanded a quarter of a pixel: the rectangle grows on every side, its "
                   "corners rounded.",
                   layer({"points": INNER, "expansion": 0.25})),
    "FX-MSK-013": ("Shrunk a quarter of a pixel: the same rectangle the other way.",
                   layer({"points": INNER, "expansion": -0.25})),
    "FX-MSK-014": ("Feathered two pixels: the hard edge at column 3 becomes a soft band.",
                   layer({"points": LEFT3, "feather": 2.0})),
    "FX-MSK-015": ("A circle of four points with handles, filling the frame's height.",
                   layer({"points": CIRCLE})),
    "FX-MSK-016": ("The same circle, expanded half a pixel.",
                   layer({"points": CIRCLE, "expansion": 0.5})),
    "FX-MSK-017": ("A mask switched off takes no part: the whole drawing.",
                   layer({"points": LEFT3, "enabled": False})),
    "FX-MSK-018": ("A mask of two points is kept, diagnosed and takes no part: the whole drawing.",
                   layer({"points": points([0, 0], [3, 2])})),
    "FX-MSK-019": ("A mask whose points cross is kept, diagnosed and takes no part.",
                   layer({"points": points([0, 0], [6, 2], [6, 0], [0, 2])})),
    "FX-MSK-031": ("A path keyed from the left three columns at frame 0 to the right three at "
                   "frame 4, linear: the rectangle slides three columns in four frames, three "
                   "quarters of a column a frame.",
                   moving(keyed((0, LEFT3), (4, RIGHT3)))),
    "FX-MSK-032": ("The same two keys, held: the shape does not move until frame 4, when it is "
                   "the right three columns at once.",
                   moving(keyed((0, LEFT3, "hold"), (4, RIGHT3)))),
    "FX-MSK-033": ("The same two keys, easy ease: the same two shapes and the same path, "
                   "reached at a different time - a quarter of the way through, the shape has "
                   "moved 0.15625 of the way.",
                   moving(keyed((0, LEFT3, EASE_IN_OUT), (4, RIGHT3)))),
    "FX-MSK-034": ("Keys at frames 1 and 3 only: frame 0 is the first key's shape and frame 4 "
                   "the last key's, because a path holds outside its keys as any property does.",
                   moving(keyed((1, LEFT3), (3, RIGHT3)))),
    "FX-MSK-035": ("Handles move with their points: the four corners stay where they are and "
                   "only the handles on the right edge grow, from nothing at frame 0 to two "
                   "pixels at frame 4, so the edge bellies further out every frame.",
                   moving(keyed((0, LEFT3), (4, BULGE)))),
}

KEYED = [fx for fx in CASES if fx >= "FX-MSK-031"]

DIAGNOSED = {"FX-MSK-018": "MASK_INVALID_OUTLINE", "FX-MSK-019": "MASK_INVALID_OUTLINE"}


def refusals():
    """Files a build must refuse whole, as PROJECT_SCHEMA_INVALID, each a change to FX-MSK-001."""
    def edit(change):
        # Deep-copied, or one refusal's damage would still be in the next one's file: the case
        # dictionaries are shared, and a mask edited in place here would be edited for everybody.
        p = deepcopy(project_json("FX-MSK-001", CASES["FX-MSK-001"][1]))
        change(p["compositions"][0]["layers"][0])
        return p

    def put(key, value):
        return lambda l: l["masks"][0].__setitem__(key, value)

    return {
        "FX-MSK-020": ("Both a `mask` and a `masks` key.",
                       edit(lambda l: l.__setitem__("mask", {"vertices": [[0, 0], [3, 0], [3, 2]],
                                                             "enabled": True, "inverted": False}))),
        "FX-MSK-021": ("A mode this build does not know.", edit(put("mode", "lighten"))),
        "FX-MSK-022": ("An opacity above 1.", edit(put("opacity", 1.5))),
        "FX-MSK-023": ("A feather below 0.", edit(put("feather_px", -1))),
        "FX-MSK-024": ("An expansion past the 8192 pixel limit.",
                       edit(put("expansion_px", 8193))),
        "FX-MSK-025": ("A point that is not two numbers.",
                       edit(lambda l: l["masks"][0]["path"]["base"]["points"][0]
                            .__setitem__("point", [0, 0, 0]))),
        "FX-MSK-026": ("A handle that is not two numbers.",
                       edit(lambda l: l["masks"][0]["path"]["base"]["points"][0]
                            .__setitem__("out", "none"))),
        "FX-MSK-027": ("A mask with no path.", edit(lambda l: l["masks"][0].pop("path"))),
        "FX-MSK-028": ("A `masks` key that is not a list.", edit(lambda l: l.__setitem__(
            "masks", l["masks"][0]))),
        "FX-MSK-029": ("Masks on an audio layer.",
                       # Everything else about a picture goes, or the file would be refused for
                       # its transform before anybody looked at its masks.
                       edit(lambda l: [l.update(kind="audio", asset_id="asset-bg"),
                                       [l.pop(k, None) for k in ("transform", "exposure_spans",
                                                                 "matte", "blend_mode",
                                                                 "effects")]])),
        "FX-MSK-030": ("A keyed path whose key holds a different number of points.",
                       edit(lambda l: l["masks"][0]["path"]["keyframes"].append(
                           {"frame": 1, "value": {"points": points([0, 0], [1, 0], [1, 1])[:3]},
                            "interp": "linear"}) or l["masks"][0]["path"]["keyframes"][0]
                           ["value"].__setitem__("points", points([0, 0], [1, 0], [1, 1])))),
    }


# --- the drawn picture ---------------------------------------------------------------------------

SCALE = 24
DIGITS = {
    "0": "111 101 101 101 111", "1": "010 110 010 010 111", "2": "111 001 111 100 111",
    "3": "111 001 111 001 111", "4": "101 101 111 001 001", "5": "111 100 111 001 111",
    "6": "111 100 111 101 111", "7": "111 001 001 001 001", "8": "111 101 111 101 111",
    "9": "111 101 111 001 111", "-": "000 000 111 000 000",
}


def linear_to_srgb(c):
    c = max(0.0, min(1.0, c))
    return c * 12.92 if c <= 0.0031308 else 1.055 * (c ** (1 / 2.4)) - 0.055


def png_any(pixels, w, h):
    raw = b"".join(b"\0" + bytes(v for px in pixels[y * w:(y + 1) * w] for v in px)
                   for y in range(h))

    def chunk(kind, data):
        return (struct.pack(">I", len(data)) + kind + data
                + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF))

    return (b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 6, 0, 0, 0))
            + chunk(b"IDAT", zlib.compress(raw, 9)) + chunk(b"IEND", b""))


def draw_cases(rendered, where):
    """Every case as a picture: its frames on a checkerboard, left to right, its number beside.

    A still case is one frame wide, a moving one is as wide as it has frames, so B-24d's picture
    shows the shape travelling across the row and can be judged without reading a number.
    """
    label_w, gap = 4 * 4 * 3, 8
    row_h = H * SCALE + gap
    across = max(len(frames) for frames in rendered.values())
    w = label_w + across * (W * SCALE + gap)
    h = row_h * len(rendered)
    canvas = [(24, 24, 28, 255)] * (w * h)
    for n, (fx, frames) in enumerate(rendered.items()):
        top = n * row_h
        for i, ch in enumerate(fx.split("-")[2]):
            for y, row in enumerate(DIGITS[ch].split()):
                for x, on in enumerate(row):
                    if on == "1":
                        for dy in range(4):
                            for dx in range(4):
                                px, py = 8 + i * 16 + x * 4 + dx, top + 12 + y * 4 + dy
                                canvas[py * w + px] = (220, 226, 236, 255)
        for f, frame in enumerate(frames):
            left = label_w + f * (W * SCALE + gap)
            for y in range(H * SCALE):
                for x in range(W * SCALE):
                    p = frame[(y // SCALE) * W + (x // SCALE)]
                    check = 0.35 if ((x // 8) + (y // 8)) % 2 else 0.5
                    rgb = [round(255 * linear_to_srgb(p[c] + (1 - p[3]) * check)) for c in range(3)]
                    canvas[(top + y) * w + left + x] = (*rgb, 255)
    where.parent.mkdir(parents=True, exist_ok=True)
    where.write_bytes(png_any(canvas, w, h))


# --- running ------------------------------------------------------------------------------------

def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    (OUT / "media" / "bg.png").write_bytes(png(BG_PIXELS))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}, "refused": {}}
    drawn = {}
    for fx, (says, case) in CASES.items():
        # A still case is pinned at frame 0, as it has been since B-24a; a case whose path moves
        # is pinned at every one of its five frames, which is the whole of what B-24d claims.
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

    print("Refused whole, each as `PROJECT_SCHEMA_INVALID`; each is FX-MSK-001's file with one "
          "change:\n")
    for fx, (says, project) in refusals().items():
        name = f"{fx.lower().replace('-', '_')}.json"
        (OUT / name).write_text(json.dumps(project, indent=2) + "\n", encoding="utf-8")
        expected["refused"][fx] = {"says": says, "project": name, "code": "PROJECT_SCHEMA_INVALID"}
        print(f"- {fx}: {says}")
    print()

    (OUT / "expected_masks.json").write_text(json.dumps(expected, indent=1) + "\n",
                                             encoding="utf-8")
    draw_cases({fx: [f["0"]] for fx, f in drawn.items() if fx not in KEYED}, PICTURE)
    draw_cases({fx: [drawn[fx][str(n)] for n in range(KEY_FRAMES)] for fx in KEYED}, KEY_PICTURE)
    print(f"Drawn: {PICTURE}\nDrawn: {KEY_PICTURE}")

    # The claims the cases are there to make, checked on the numbers just worked.
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    at = {fx: expected["cases"][fx]["frames"] for fx in KEYED}
    bg = decoded("bg")
    blank = [[0.0] * 4] * (W * H)
    # Nothing moved: the new shape draws today's mask exactly, and the old key reads into it.
    assert c["FX-MSK-001"][:3] == bg[:3] and c["FX-MSK-001"][3:6] == blank[3:6]
    assert c["FX-MSK-001"] == c["FX-MSK-002"] == c["FX-MSK-010"]
    assert c["FX-MSK-017"] == c["FX-MSK-018"] == c["FX-MSK-019"] == bg
    # A mask and its inverse sum back to the drawing, everywhere.
    assert all(abs(a[i] + b[i] - p[i]) < 1e-12
               for a, b, p in zip(c["FX-MSK-001"], c["FX-MSK-004"], bg) for i in range(4))
    assert c["FX-MSK-005"] == [[v * 0.5 for v in p] for p in c["FX-MSK-001"]]
    assert c["FX-MSK-006"] == bg                      # Add: the two halves make the whole
    assert c["FX-MSK-008"][2] == bg[2] and c["FX-MSK-008"][1] == [0.0] * 4   # Intersect: column 2
    assert c["FX-MSK-007"][2] == [0.0] * 4 and c["FX-MSK-007"][1] == bg[1]   # Subtract: the hole
    # Difference: column 2 is in both and goes, column 3 is in one and stays.
    assert c["FX-MSK-009"][2] == [0.0] * 4 and c["FX-MSK-009"][3] == bg[3]
    assert c["FX-MSK-009"][1] == bg[1] and c["FX-MSK-009"][4] == [0.0] * 4
    assert c["FX-MSK-011"][2] == [0.0] * 4 and c["FX-MSK-011"][0] == bg[0]
    # Expansion grows and shrinks, and a sloped edge lands on the sixteenths.
    grown, shrunk = c["FX-MSK-012"], c["FX-MSK-013"]
    assert sum(p[3] for p in grown) > sum(p[3] for p in shrunk)
    assert all(abs(p[3] * 16 - round(p[3] * 16)) < 1e-12 for p in c["FX-MSK-003"])
    # Feather softens the edge without changing what is well inside or well outside.
    soft = [p[3] for p in c["FX-MSK-014"]]
    assert all(0 < v < 1 for v in soft)            # every column is now partly there
    assert soft[1] > soft[2] > soft[3] > soft[4] > soft[5]   # and falls away past the edge
    assert c["FX-MSK-015"][0][3] == 0 and c["FX-MSK-015"][2][3] > 0
    assert sum(p[3] for p in c["FX-MSK-016"]) > sum(p[3] for p in c["FX-MSK-015"])

    # A moving path: the shape at a key is the shape drawn, and between two keys it is worked
    # point by point at the fraction document 20 gives.
    def alphas(frame):
        return [round(frame[x][3], 12) for x in range(W)]   # the top row, which is enough

    slide = at["FX-MSK-031"]
    assert slide["0"] == c["FX-MSK-001"]                        # frame 0 is the first key exactly
    assert alphas(slide["0"]) == [1, 1, 1, 0, 0, 0]
    assert alphas(slide["4"]) == [0, 0, 0, 1, 1, 1]             # and frame 4 the last key
    assert alphas(slide["1"]) == [0.25, 1, 1, 0.75, 0, 0]       # three quarters of a column a frame
    assert alphas(slide["2"]) == [0, 0.5, 1, 1, 0.5, 0]
    assert alphas(slide["3"]) == [0, 0, 0.75, 1, 1, 0.25]
    held = at["FX-MSK-032"]
    assert held["0"] == held["1"] == held["2"] == held["3"] == slide["0"]   # a hold does not move
    assert held["4"] == slide["4"]
    eased = at["FX-MSK-033"]
    assert eased["0"] == slide["0"] and eased["4"] == slide["4"]   # the same two shapes
    assert eased["2"] == slide["2"]                 # easy ease is half way at half the segment
    assert alphas(eased["1"]) == [0.5, 1, 1, 0.5, 0, 0]   # but nearer the start a quarter in
    assert eased["1"] != slide["1"] and eased["3"] != slide["3"]
    outside = at["FX-MSK-034"]
    assert outside["0"] == outside["1"] == slide["0"]   # before the first key, the first key
    assert outside["3"] == outside["4"] == slide["4"]   # after the last key, the last key
    assert outside["2"] == slide["2"]                   # and half way between them, half way
    handles = at["FX-MSK-035"]
    assert handles["0"] == c["FX-MSK-001"]              # no handles at all is the straight edge
    grew = [sum(p[3] for p in handles[str(f)]) for f in range(KEY_FRAMES)]
    assert grew == sorted(grew) and grew[0] < grew[-1]   # the edge bellies further out each frame
    assert alphas(handles["0"])[3] == 0 and alphas(handles["4"])[3] > 0.5


if __name__ == "__main__":
    main()
