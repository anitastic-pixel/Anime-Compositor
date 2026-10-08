"""Pass Extract and Depth Key, worked a second way.

D-348 adds the first effects that read more of an OpenEXR file than its colour: a render's depth
(Z) and surface normals, kept in the same file as extra channels. EFFECTS.md's P0-5. Two effects
use them, after After Effects' 3D Channel effects, under this program's own names:

- `core.pass_extract`, Pass Extract, after 3D Channel Extract: the depth or the normals as a
  picture, the value at the Black Point black and at the White Point white.
- `core.depth_key`, Depth Key, after Depth Matte: what is nearer than a depth taken out.

**Which channels are the passes.** A channel name is split at its last dot into a layer and a part
(`N.x` is layer `N`, part `x`; `Z` is layer "", part `Z`). Case is ignored throughout.

- The depth is the first channel, in the file's own order, whose part is `z` and whose layer is
  "", `depth`, `zdepth` or `z`, or whose layer is "" and whose part is `depth` or `zdepth`.
- The normals are the first layer named `n`, `normal` or `normals` holding all three of the parts
  `x`, `y`, `z`; failing that, all three of `r`, `g`, `b`. Red, green and blue are those three.

A pass is read exactly as the colour is (D-62): the display window is the picture, the data window
copied where it overlaps, 0 outside it. Not-a-number is read as 0 and an infinity as the largest
single-precision number of its sign (3.4028234663852886e38), and each frame that reads such a pass
says so with `MEDIA_EXR_ADJUSTED` (`non_finite: n`). The file itself is never changed, so the
passes survive saving and opening: the project keeps the file, and the file keeps its channels.

**Where the pass comes from.** The layer's own footage file at that frame. A layer whose footage
is not an EXR, a layer with no footage (a solid, a shape, a composition, an adjustment layer), or
an EXR without the pass, has no pass: the effect is skipped, its settings kept, the picture
unchanged, and `EFFECT_CHANNEL_MISSING` is said every frame.

At composition frame n, with the input O, linear and premultiplied, the layer's drawing's own
top-left pixel at (ox, oy) however far earlier effects grew it, and the pass P the drawing's size,
lying there:

**Pass Extract.** `pass`, `depth` or `normals`, `depth` when added; `black_point` and
`white_point`, -1,000,000 to 1,000,000, 0 and 1 when added, keyable; `invert`, `off` or `on`, `off`
when added; `clamp`, `off` or `on`, `on` when added. Each of the pass's values v (the depth three
times, or the normals' three) becomes o = (v - black) / (white - black); where the two points are
equal, o = 1 where v >= white and 0 elsewhere. `invert` takes o to 1 - o, and then `clamp` holds o
in 0 to 1. The output pixel is (o_r, o_g, o_b, 1): an opaque picture of the pass over the whole
drawing, whatever the input there was, so that, as Adobe's page says, the depth used as a luma
matte gives fog. Outside the drawing it is transparent.

**Depth Key.** `depth`, -1,000,000 to 1,000,000, 0 when added; `feather`, 0 to 1,000,000, 0 when
added, both keyable, in the depth's own units; `invert`, `off` or `on`, `off` when added. With z the
depth, k = 1 where z >= depth and 0 elsewhere; with a feather f above 0, k = clamp((z - depth) / f +
1/2, 0, 1). `invert` takes k to 1 - k. The output is O times k, all four numbers; outside the
drawing it is transparent. A pixel exactly at the depth is kept.

Neither grows the picture or has a distance in it, so a draft is the same rule at the draft's size,
the pass stretched to it as D-189 stretches a map.

**This file never runs the build's code path.** It writes the EXR files with OpenEXR's own library
(3.4.15), reads them back with it, and works each expected pixel in double precision.

Every case is a project of one composition 8 by 6 at 24 frames a second, five frames long, in
`Fixtures/depth_channel/`. The expected pixels are in
`Fixtures/depth_channel/expected_depth_channel.json`. `Fixtures/depth_channel/sample/spheres.exr`
is a bigger picture for the playtest: three balls on a floor, with depth and normals.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/depth_channel_reference.py   (with OpenEXR 3.4.15 and numpy)
"""

import json
import sys
from math import floor, isfinite, isnan, sqrt
from pathlib import Path

import numpy as np
import OpenEXR

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "depth_channel"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES, FPS = 8, 6, 5, 24
CLEAR = [0.0, 0.0, 0.0, 0.0]
BIG = 3.4028234663852886e38  # the largest single-precision number


# --- the files ------------------------------------------------------------------------------

def box(x0, y0, x1, y1):
    return ((x0, y0), (x1, y1))


def write(rel, channels, data=None):
    """One single-part scanline file, ZIP; `data` is the data window (x0, y0, x1, y1), inclusive,
    inside the 8 by 6 display window."""
    shape = next(iter(channels.values())).shape
    data = data or (0, 0, shape[1] - 1, shape[0] - 1)
    header = {"compression": OpenEXR.ZIP_COMPRESSION, "type": OpenEXR.scanlineimage,
              "dataWindow": box(*data), "displayWindow": box(0, 0, W - 1, H - 1),
              "lineOrder": OpenEXR.INCREASING_Y, "pixelAspectRatio": 1.0}
    path = OUT / rel
    path.parent.mkdir(parents=True, exist_ok=True)
    # OpenEXR writes an array's memory as it lies, so a slice is copied out first.
    OpenEXR.File(header, {k: np.ascontiguousarray(v) for k, v in channels.items()}).write(str(path))


def colour(w=W, h=H):
    """Premultiplied colour, half: red by column, green by row; (0, 0) at half covering and
    (1, 0) clear."""
    y, x = np.mgrid[0:h, 0:w].astype(np.float64)
    a = np.ones((h, w))
    a[0, 0], a[0, 1] = 0.5, 0.0
    return {"R": ((0.1 + 0.1 * x) * a).astype("f2"), "G": ((0.2 + 0.05 * y) * a).astype("f2"),
            "B": (0.5 * a).astype("f2"), "A": a.astype("f2")}


def depth(w=W, h=H):
    y, x = np.mgrid[0:h, 0:w].astype(np.float64)
    return (0.5 + 1.25 * x + 0.75 * y).astype("f4")


def normals(w=W, h=H):
    y, x = np.mgrid[0:h, 0:w].astype(np.float64)
    nx, ny = (x - 3.5) / 4, (2.5 - y) / 3
    nz = np.sqrt(np.maximum(0.0, 1 - nx * nx - ny * ny))
    return nx.astype("f2"), ny.astype("f2"), nz.astype("f2")


def make_files():
    nx, ny, nz = normals()
    write("media/scene.exr", dict(colour(), Z=depth(), **{"N.x": nx, "N.y": ny, "N.z": nz}))
    # Other names for the same passes, and a depth running the other way.
    y, x = np.mgrid[0:H, 0:W].astype(np.float64)
    write("media/names.exr", dict(colour(), **{"depth.Z": (20 - 2 * x - y).astype("f4"),
                                               "normal.R": ny, "normal.G": nz, "normal.B": nx}))
    far = depth().copy()
    far[5, 7], far[5, 6], far[5, 5] = np.inf, -np.inf, np.nan
    write("media/far.exr", dict(colour(), Z=far))
    # The data window is columns 2 to 6 of rows 1 to 4; outside it every channel is 0.
    inner = {k: v[1:5, 2:7] for k, v in dict(colour(), Z=depth(), **{"N.x": nx, "N.y": ny,
                                                                        "N.z": nz}).items()}
    write("media/offset.exr", inner, data=(2, 1, 6, 4))
    write("media/plain.exr", colour())
    L.DRAWINGS["card"] = [[(36 * x, 200, 30 * y, 255) for x in range(W)] for y in range(H)]
    (OUT / "media" / "card.png").write_bytes(png(L.DRAWINGS["card"]))
    spheres()


def read(rel):
    """The file as OpenEXR reads it: every channel, the display window's size, 0 outside the
    data window."""
    with OpenEXR.File(str(OUT / rel), separate_channels=True) as f:
        header, channels = f.header(), f.channels()
        (x0, y0), _ = header["dataWindow"]
        out = {}
        for name, c in channels.items():
            plane = np.zeros((H, W), np.float64)
            p = c.pixels.astype(np.float64)
            for j in range(p.shape[0]):
                for i in range(p.shape[1]):
                    if 0 <= x0 + i < W and 0 <= y0 + j < H:
                        plane[y0 + j, x0 + i] = p[j, i]
            out[name] = plane
        return out


def finite(v):
    """Not-a-number read as 0, an infinity as the largest number of its sign."""
    if isnan(v):
        return 0.0
    if not isfinite(v):
        return BIG if v > 0 else -BIG
    return float(v)


def split(name):
    layer, _, part = name.rpartition(".")
    return layer.lower(), part.lower()


def depth_name(names):
    for n in names:
        layer, part = split(n)
        if (part == "z" and layer in ("", "depth", "zdepth", "z")) or \
                (layer == "" and part in ("depth", "zdepth")):
            return n
    return None


def normal_names(names):
    for parts in (("x", "y", "z"), ("r", "g", "b")):
        for n in names:
            layer, _ = split(n)
            if layer not in ("n", "normal", "normals"):
                continue
            found = [next((m for m in names if split(m) == (layer, p)), None) for p in parts]
            if all(found):
                return found
    return None


def picture(rel):
    """The layer's drawing as the build draws an EXR: the colour as stored."""
    c = read(rel)
    return L.pic(W, H, [[c[k][y, x] for k in "RGBA"] for y in range(H) for x in range(W)])


def passes(rel, which):
    """The pass's three values at each pixel, or None where the file has none."""
    c = read(rel)
    names = sorted(c)  # a file keeps its channels in this order
    if which == "depth":
        n = depth_name(names)
        if n is None:
            return None
        return [[finite(c[n][y, x])] * 3 for y in range(H) for x in range(W)]
    ns = normal_names(names)
    if ns is None:
        return None
    return [[finite(c[n][y, x]) for n in ns] for y in range(H) for x in range(W)]


def non_finite(rel, which):
    c = read(rel)
    names = sorted(c)
    chosen = [depth_name(names)] if which == "depth" else (normal_names(names) or [])
    return sum(1 for n in chosen if n for v in c[n].flat if not isfinite(v))


# --- the rule -------------------------------------------------------------------------------

def extract(v, black, white, invert, clamp):
    o = (v - black) / (white - black) if white != black else (1.0 if v >= white else 0.0)
    if invert == "on":
        o = 1 - o
    return min(1.0, max(0.0, o)) if clamp == "on" else o


def keep(z, at, feather, invert):
    k = min(1.0, max(0.0, (z - at) / feather + 0.5)) if feather > 0 else (1.0 if z >= at else 0.0)
    return 1 - k if invert == "on" else k


def drawing(c):
    """The layer's picture before its effects."""
    if c["layer"] == "card":
        return L.decoded("card")
    if c["layer"] == "solid":
        return L.pic(W, H, [SOLID + [1.0] for _ in range(W * H)])
    return picture(c["file"])


def effected(c, n):
    """The layer's picture after its effects at frame n."""
    p = drawing(c)
    if c["before"] is not None:
        p = L.exposure(p, c["before"])
    if not works(c):
        return p
    pv = passes(c["file"], c["pass"])
    if c["effect"] == "extract":
        b, w = value_at(c["black_point"], n), value_at(c["white_point"], n)
        px = [[extract(v, b, w, c["invert"], c["clamp"]) for v in pv[i]] + [1.0]
              for i in range(W * H)]
    else:
        at, f = value_at(c["depth"], n), value_at(c["feather"], n)
        px = []
        for i in range(W * H):
            k = keep(pv[i][0], at, f, c["invert"])
            px.append([v * k for v in p["px"][i]])
    out = L.pic(W, H, px)
    if c["after"] is not None:
        out = L.exposure(out, c["after"])
    return out


def render(c, frame):
    dx, dy = c["move"]
    p = drawing(c) if c["on"] == "adjust" else effected(c, frame)
    return [list(L.at(p, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c, frame):
    return render(dict(c, works=False, before=None, after=None), frame)


# --- the cases ------------------------------------------------------------------------------

SOLID = [0.25, 0.5, 1.0]


def extract_case(pass_="depth", black_point=0, white_point=1, invert="off", clamp="on", **kw):
    return case("extract", **{"pass": pass_, "black_point": black_point,
                              "white_point": white_point, "invert": invert, "clamp": clamp}, **kw)


def key_case(depth=0, feather=0, invert="off", **kw):
    return case("key", depth=depth, feather=feather, invert=invert, **kw)


def case(effect, file="scene.exr", layer="scene", move=(0, 0), before=None, after=None,
         on="scene", **settings):
    c = {"effect": effect, "file": "media/" + file, "layer": layer, "move": move,
         "before": before, "after": after, "on": on, "pass": "depth"}
    c.update(settings)
    return c


def works(c):
    return c.get("works", True) and c["layer"] == "scene" and c["on"] == "scene"         and passes(c["file"], c["pass"]) is not None


RAMP = dict(black_point=0, white_point=12)
CASES = {
    "FX-DEPTH-001": ("Pass Extract as added: the depth, Black Point 0, White Point 1, Clamp on: "
                     "every depth past 1 is white, the nearest pixel (0.5) mid grey; the picture "
                     "is opaque, its clear corner too.", extract_case(), (0,)),
    "FX-DEPTH-002": ("Black 0, White 12: the depth as a grey ramp, near dark, far light.",
                     extract_case(**RAMP), (0,)),
    "FX-DEPTH-003": ("Black 12, White 0: the other way round, near light.",
                     extract_case(black_point=12, white_point=0), (0,)),
    "FX-DEPTH-004": ("Black 0, White 12, Invert on: FX-DEPTH-002 turned over, as 003.",
                     extract_case(**RAMP, invert="on"), (0,)),
    "FX-DEPTH-005": ("Black 10, White 14, Clamp off, the other names' file (depth 1 to 20): "
                     "values below 0 and above 1 are kept.",
                     extract_case(black_point=10, white_point=14, clamp="off", file="names.exr"),
                     (0,)),
    "FX-DEPTH-006": ("Black and White both 6: a cut, white from depth 6 on, black nearer; the "
                     "pixel exactly at 6 is white.", extract_case(black_point=6, white_point=6),
                     (0,)),
    "FX-DEPTH-007": ("The normals, Black -1, White 1: each direction as a colour, x red, y green, "
                     "z blue, mid grey for 0.",
                     extract_case(pass_="normals", black_point=-1, white_point=1), (0,)),
    "FX-DEPTH-008": ("The normals as added, Black 0, White 1, Clamp on: what points left or down "
                     "is held at 0.", extract_case(pass_="normals"), (0,)),
    "FX-DEPTH-009": ("The data window only columns 2 to 6 of rows 1 to 4: outside it the depth "
                     "is 0, black, as the colour there is clear.",
                     extract_case(**RAMP, file="offset.exr"), (0,)),
    "FX-DEPTH-010": ("Depth named depth.Z: found.", extract_case(**{"black_point": 7,
                                                                       "white_point": 20},
                                                                    file="names.exr"), (0,)),
    "FX-DEPTH-011": ("Normals named normal.R, normal.G, normal.B: found.",
                     extract_case(pass_="normals", black_point=-1, white_point=1,
                                  file="names.exr"), (0,)),
    "FX-DEPTH-012": ("Infinities and not-a-number in the depth: far infinity white, near "
                     "infinity black, not-a-number read as 0, black; the warning every frame.",
                     extract_case(**RAMP, file="far.exr"), (0,)),
    "FX-DEPTH-013": ("A file with no depth: nothing changes, with the warning every frame.",
                     extract_case(**RAMP, file="plain.exr"), (0,)),
    "FX-DEPTH-014": ("Asked for normals from a file with depth only: nothing changes, the "
                     "warning.", extract_case(pass_="normals", file="far.exr"), (0,)),
    "FX-DEPTH-015": ("On a PNG drawing: no passes, nothing changes, the warning.",
                     extract_case(**RAMP, layer="card"), (0,)),
    "FX-DEPTH-016": ("On a solid: no file, nothing changes, the warning.",
                     extract_case(**RAMP, layer="solid"), (0,)),
    "FX-DEPTH-017": ("On an adjustment layer above the EXR: no file of its own, nothing changes, "
                     "the warning.", extract_case(**RAMP, on="adjust"), (0,)),
    "FX-DEPTH-018": ("An Exposure of +1 before it is not seen: FX-DEPTH-002.",
                     extract_case(**RAMP, before=1), (0,)),
    "FX-DEPTH-019": ("An Exposure of -1 after it darkens the ramp.",
                     extract_case(**RAMP, after=-1), (0,)),
    "FX-DEPTH-020": ("The layer moved 2 right and 1 down: FX-DEPTH-002 moved.",
                     extract_case(**RAMP, move=(2, 1)), (0,)),
    "FX-DEPTH-021": ("White Point keyed from 1 at frame 0 to 12 at frame 4: frame 0 is "
                     "FX-DEPTH-001, frame 4 FX-DEPTH-002.",
                     extract_case(white_point=keyed((0, 1), (4, 12))), (0, 4)),
    "FX-DEPTH-022": ("Depth Key as added: Depth 0, no feather: every depth is at least 0, so "
                     "nothing changes.", key_case(), (0,)),
    "FX-DEPTH-023": ("Depth 6: everything nearer than 6 taken out; the pixel exactly at 6 kept.",
                     key_case(depth=6), (0,)),
    "FX-DEPTH-024": ("Depth 6, Invert on: everything from 6 on taken out instead.",
                     key_case(depth=6, invert="on"), (0,)),
    "FX-DEPTH-025": ("Depth 6, Feather 4: a soft edge from 4 to 8.",
                     key_case(depth=6, feather=4), (0,)),
    "FX-DEPTH-026": ("Depth 6, Feather 4, Invert on: the soft edge the other way.",
                     key_case(depth=6, feather=4, invert="on"), (0,)),
    "FX-DEPTH-027": ("Depth 6 on the file with infinities: far infinity kept, near infinity and "
                     "not-a-number (0) out; the warning.", key_case(depth=6, file="far.exr"),
                     (0,)),
    "FX-DEPTH-028": ("Depth 5 on the small data window: inside it the near pixels go; outside it the "
                     "depth is 0, out, but already clear.", key_case(depth=5, file="offset.exr"), (0,)),
    "FX-DEPTH-029": ("An Exposure of +1 before it: the kept pixels are the brighter ones.",
                     key_case(depth=6, before=1), (0,)),
    "FX-DEPTH-030": ("Depth Key on a file with no depth: nothing changes, the warning.",
                     key_case(depth=6, file="plain.exr"), (0,)),
    "FX-DEPTH-031": ("Depth keyed from 0 at frame 0 to 12.5 at frame 4: frame 4 has only the far "
                     "corner left.", key_case(depth=keyed((0, 0), (4, 12.5))), (0, 4)),
}

INVALID = {
    "FX-DEPTH-032": ("Pass Extract's pass written \"uv\".", extract_case(pass_="uv")),
    "FX-DEPTH-033": ("Pass Extract's clamp written \"yes\".", extract_case(clamp="yes")),
    "FX-DEPTH-034": ("Black Point 2,000,000, past 1,000,000.",
                     extract_case(black_point=2000000)),
    "FX-DEPTH-035": ("Depth Key's feather -1, below 0.", key_case(feather=-1)),
    "FX-DEPTH-036": ("Depth Key's invert written \"yes\".", key_case(invert="yes")),
}

EXTRACT = ("pass", "black_point", "white_point", "invert", "clamp")
KEY = ("depth", "feather", "invert")


def effect(fid, c):
    if c["effect"] == "extract":
        return {"instance_id": fid, "type_id": "core.pass_extract", "enabled": True,
                "parameters": {k: setting_json(c[k]) for k in EXTRACT}}
    return {"instance_id": fid, "type_id": "core.depth_key", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in KEY}}


def exposure(fid, stops):
    return {"instance_id": fid, "type_id": "core.exposure", "enabled": True,
            "parameters": {"stops": stops}}


def exr(name, path):
    return {"id": "asset-" + name, "kind": "still", "name": name, "path": path,
            "interpretation": {"color_space": "linear-srgb", "alpha": "premultiplied"}}


def project_json(fx, c):
    asset = {"card": "asset-card", "solid": None}.get(c["layer"], "asset-scene")
    if asset is None:
        holder = L.solid("scene", W, H, SOLID, position=c["move"], out_frame=FRAMES)
    else:
        holder = L.raster("scene", asset, position=c["move"], out_frame=FRAMES)
    layers = [holder]
    stack = ([exposure("fx-0", c["before"])] if c["before"] is not None else []) \
        + [effect("fx-1", c)] \
        + ([exposure("fx-2", c["after"])] if c["after"] is not None else [])
    if c["on"] == "adjust":
        adjust = L.adjustment("adjust", out_frame=FRAMES)
        adjust["effects"] = stack
        layers.insert(0, adjust)
    else:
        holder["effects"] = stack
    comp = {"id": "comp-main", "name": "comp-main", "width": W, "height": H,
            "pixel_aspect_ratio": 1, "frame_rate": {"numerator": FPS, "denominator": 1},
            "start_frame": 0, "duration_frames": FRAMES,
            "work_area": {"start_frame": 0, "end_frame_exclusive": FRAMES},
            "layer_order": [l["id"] for l in layers], "layers": layers}
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [exr("scene", c["file"]), L.still("card", "media/card.png")],
            "compositions": [comp]}


def write_project(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def warning_at_frame(c):
    """What every frame says, beyond what opening the file says."""
    if c["on"] == "adjust" or c["layer"] != "scene" or passes(c["file"], c["pass"]) is None:
        return "EFFECT_CHANNEL_MISSING"
    if non_finite(c["file"], c["pass"]):
        return "MEDIA_EXR_ADJUSTED"
    return None


def main():
    make_files()
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        entry = {"says": says, "project": write_project(fx, c), "frames": rendered}
        said = warning_at_frame(c)
        if said:
            entry["frame_warning"] = said
        expected["cases"][fx] = entry
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != plain(c, int(f))[i] for i in range(W * H))} changed"
            for f, px in rendered.items()) + (f", {said}" if said else ""))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        expected["cases"][fx] = {"says": says, "project": write_project(fx, c),
                                 "frames": {"0": plain(c, 0), "4": plain(c, 4)},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_depth_channel.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    four = {fx: v["frames"].get("4") for fx, v in expected["cases"].items()}
    scene = plain(extract_case(), 0)
    z = lambda x, y: 0.5 + 1.25 * x + 0.75 * y  # noqa: E731
    near = lambda a, b: all(abs(p - q) < 1e-9 for u, v in zip(a, b)  # noqa: E731
                            for p, q in zip(u, v))
    # 001: white where the depth passes 1, mid grey at the one pixel at 0.5; all opaque.
    assert all(p[3] == 1 for p in c["FX-DEPTH-001"])
    assert c["FX-DEPTH-001"][0][:3] == [0.5] * 3 and c["FX-DEPTH-001"][1][:3] == [1.0] * 3
    for y in range(H):
        for x in range(W):
            assert abs(c["FX-DEPTH-002"][y * W + x][0] - min(1, z(x, y) / 12)) < 1e-12
    assert near(c["FX-DEPTH-003"], c["FX-DEPTH-004"])
    assert min(p[0] for p in c["FX-DEPTH-005"]) < 0 < 1 < max(p[0] for p in c["FX-DEPTH-005"])
    assert c["FX-DEPTH-006"][4 * W + 2][0] == 1.0 and c["FX-DEPTH-006"][4 * W + 1][0] == 0.0
    # 007: mid grey where a direction is 0, red rising to the right, green to the top.
    assert c["FX-DEPTH-007"][0][0] < c["FX-DEPTH-007"][7][0]
    assert c["FX-DEPTH-007"][0][1] > c["FX-DEPTH-007"][5 * W][1]
    assert all(p[0] == 0 for i, p in enumerate(c["FX-DEPTH-008"]) if i % W < 4)
    for y in range(H):
        for x in range(W):
            inside = 2 <= x <= 6 and 1 <= y <= 4
            want = min(1, z(x, y) / 12) if inside else 0.0
            assert abs(c["FX-DEPTH-009"][y * W + x][0] - want) < 1e-6, (x, y)
    assert c["FX-DEPTH-010"][0][0] == 1.0 and c["FX-DEPTH-010"][W * H - 1][0] == 0.0
    far = c["FX-DEPTH-012"]
    assert far[5 * W + 7][:3] == [1.0] * 3 and far[5 * W + 6][:3] == [0.0] * 3
    assert far[5 * W + 5][:3] == [0.0] * 3
    for fx in ("FX-DEPTH-013", "FX-DEPTH-014", "FX-DEPTH-016", "FX-DEPTH-017",
               "FX-DEPTH-022", "FX-DEPTH-030"):
        assert near(c[fx], scene if fx != "FX-DEPTH-016" else c[fx]), fx
    assert c["FX-DEPTH-015"] == plain(extract_case(layer="card"), 0)
    assert c["FX-DEPTH-016"] == [SOLID + [1.0]] * (W * H)
    assert near(c["FX-DEPTH-018"], c["FX-DEPTH-002"])
    assert near([[v * (0.5 if i < 3 else 1) for i, v in enumerate(p)] for p in c["FX-DEPTH-002"]],
                c["FX-DEPTH-019"])
    assert all(c["FX-DEPTH-020"][(y + 1) * W + x + 2] == c["FX-DEPTH-002"][y * W + x]
               for y in range(H - 1) for x in range(W - 2))
    assert near(c["FX-DEPTH-021"], c["FX-DEPTH-001"]) and near(four["FX-DEPTH-021"],
                                                               c["FX-DEPTH-002"])
    for y in range(H):
        for x in range(W):
            i = y * W + x
            kept = z(x, y) >= 6
            assert c["FX-DEPTH-023"][i] == (scene[i] if kept else CLEAR), (x, y)
            assert c["FX-DEPTH-024"][i] == (CLEAR if kept else scene[i]), (x, y)
            k = min(1.0, max(0.0, (z(x, y) - 6) / 4 + 0.5))
            assert near([c["FX-DEPTH-025"][i]], [[v * k for v in scene[i]]])
            assert near([c["FX-DEPTH-026"][i]], [[v * (1 - k) for v in scene[i]]])
    assert c["FX-DEPTH-023"][4 * W + 2] == scene[4 * W + 2]  # exactly at 6
    keyed_far = c["FX-DEPTH-027"]
    assert keyed_far[5 * W + 7] != CLEAR and keyed_far[5 * W + 6] == CLEAR == keyed_far[5 * W + 5]
    assert all(p == CLEAR for i, p in enumerate(c["FX-DEPTH-028"])
               if not (2 <= i % W <= 6 and 1 <= i // W <= 4))
    assert [i for i, p in enumerate(four["FX-DEPTH-031"]) if p != CLEAR] == [W * H - 1]
    print("checks passed")


# --- the playtest's picture -----------------------------------------------------------------

def spheres():
    """Three balls on a floor, seen from a camera 1.5 above it looking slightly down: colour,
    depth (the distance along the camera's view, 1,000 for the clear sky) and the normals as
    the camera sees them (x right, y up, z towards it). 320 by 180."""
    w, h = 320, 180
    balls = [((-1.6, 0.8, 6.0), 0.8, (0.9, 0.25, 0.2)), ((0.3, 1.0, 9.0), 1.0, (0.2, 0.6, 0.9)),
             ((2.4, 1.2, 14.0), 1.2, (0.95, 0.8, 0.2))]
    eye = np.array([0.0, 1.5, 0.0])
    tilt = 0.12
    ca, sa = np.cos(tilt), np.sin(tilt)
    light = np.array([-0.5, 0.8, -0.4])
    light = light / np.linalg.norm(light)
    planes = {k: np.zeros((h, w), np.float64) for k in ("R", "G", "B", "A", "Z", "N.x", "N.y",
                                                         "N.z")}
    for j in range(h):
        for i in range(w):
            u = (i + 0.5 - w / 2) / (h / 2) * 0.6
            v = -(j + 0.5 - h / 2) / (h / 2) * 0.6
            d_cam = np.array([u, v, 1.0])  # camera space: x right, y up, z forward
            d = np.array([d_cam[0], d_cam[1] * ca - d_cam[2] * sa, d_cam[1] * sa + d_cam[2] * ca])
            d = d / np.linalg.norm(d)
            best, hit = 1e9, None
            for centre, r, col in balls:
                oc = eye - np.array(centre)
                b = np.dot(oc, d)
                q = b * b - (np.dot(oc, oc) - r * r)
                if q >= 0:
                    t = -b - sqrt(q)
                    if 0 < t < best:
                        p = eye + t * d
                        best, hit = t, ((p - np.array(centre)) / r, col)
            if d[1] < 0:
                t = -eye[1] / d[1]
                if t < best:
                    p = eye + t * d
                    check_ = (floor(p[0]) + floor(p[2])) % 2
                    best, hit = t, (np.array([0.0, 1.0, 0.0]),
                                    (0.55, 0.55, 0.5) if check_ else (0.3, 0.3, 0.28))
            if hit is None:
                planes["Z"][j, i] = 1000.0  # the sky: clear, and far away
                continue
            n, col = hit
            shade = 0.15 + 0.85 * max(0.0, float(np.dot(n, light)))
            # Camera space: undo the tilt.
            n_cam = np.array([n[0], n[1] * ca + n[2] * sa, -n[1] * sa + n[2] * ca])
            hit_cam_z = best * float(np.dot(d, np.array([0.0, -sa, ca])))
            for k, value in zip("RGB", col):
                planes[k][j, i] = value * shade
            planes["A"][j, i] = 1.0
            planes["Z"][j, i] = hit_cam_z
            planes["N.x"][j, i], planes["N.y"][j, i], planes["N.z"][j, i] = n_cam[0], n_cam[1], \
                -n_cam[2]
    header = {"compression": OpenEXR.ZIP_COMPRESSION, "type": OpenEXR.scanlineimage,
              "dataWindow": box(0, 0, w - 1, h - 1), "displayWindow": box(0, 0, w - 1, h - 1),
              "lineOrder": OpenEXR.INCREASING_Y, "pixelAspectRatio": 1.0}
    channels = {k: v.astype("f4" if k == "Z" else "f2") for k, v in planes.items()}
    (OUT / "sample").mkdir(parents=True, exist_ok=True)
    OpenEXR.File(header, channels).write(str(OUT / "sample" / "spheres.exr"))


if __name__ == "__main__":
    main()
