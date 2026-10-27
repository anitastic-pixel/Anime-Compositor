"""D-915: the viewer's colour views (ADR-023, step C5), OpenColorIO's numbers and its table.

Writes:
  assets/colour/aces2_sdr_srgb_97.f32    the ACES 2.0 SDR view the build ships, baked by
                                          OpenColorIO's Baker as a .cube: input ACEScct (the
                                          shaper, the build's own D-906 curve), 97 points a side,
                                          output the sRGB display's encoded values; written as
                                          little-endian 32-bit floats, red, green, blue, red
                                          changing fastest, nothing else (the .cube's numbers
                                          exactly, at 11 MB instead of 25)
  Fixtures/colour/view/expected_view.json OpenColorIO's own (analytic) ACES 2.0 SDR and
                                          Un-tone-mapped views of a synthetic HDR scene, a chart
                                          and a stress set, from both working spaces, as 8-bit
                                          levels; and the table's SHA-256, its recorded check
  Fixtures/colour/view/media/hdr_scene.exr the scene as a picture (linear Rec.709 floats)
  Fixtures/colour/view/*.json             projects that show it with each view

Every expected number is OpenColorIO's, from PyOpenColorIO and its built-in ACES 2.0 studio
config; nothing here calls the build. The table and the expected values come from the same
OpenColorIO, but by different routes (Baker's sampled table against the analytic processor),
so the build's table is judged against the maths it stands in for.

The tool needs only PyOpenColorIO, the tool dependency D-906 recorded (BSD-3-Clause, never
shipped):

    python -m venv <folder>/venv
    <folder>/venv/Scripts/python -m pip install opencolorio==2.5.1
    <folder>/venv/Scripts/python -I tools/view_reference.py
"""

import hashlib
import json
import math
import os
import random
import struct

import PyOpenColorIO as ocio

VERSION = "2.5.1"
CONFIG = "ocio://studio-config-v4.0.0_aces-v2.0_ocio-v2.5"
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "..", "Fixtures", "colour", "view")
TABLE = os.path.join(HERE, "..", "assets", "colour", "aces2_sdr_srgb_97.f32")
OPT = ocio.OPTIMIZATION_LOSSLESS
DISPLAY = "sRGB - Display"
VIEWS = {"aces-2.0-sdr": "ACES 2.0 - SDR 100 nits (Rec.709)", "un-tone-mapped": "Un-tone-mapped"}
REC709, AP1 = "Linear Rec.709 (sRGB)", "ACEScg"
WORKING = {"acescg": AP1, "linear-srgb": REC709}
SIZE = 97  # D-915: 65 missed the bar on the chart, 97 with tetrahedral lookup meets it
WIDTH, BAND = 512, 32


def g9(x):
    return float("%.9g" % x)


def level(v):  # color::quantise_u8: clamp, times 255, round half up
    return int(math.floor(min(max(v, 0.0), 1.0) * 255 + 0.5))


def convert(src, dst, rgb):
    if src == dst:
        return [g9(v) for v in rgb]
    cpu = config.getProcessor(src, dst).getOptimizedCPUProcessor(OPT)
    return [g9(v) for v in cpu.applyRGB(list(rgb))]


def viewed(working, view, rgb):
    t = ocio.DisplayViewTransform(src=WORKING[working], display=DISPLAY, view=VIEWS[view])
    out = config.getProcessor(t).getOptimizedCPUProcessor(OPT).applyRGB(list(rgb))
    return [[level(v) for v in out[3 * i:3 * i + 3]] for i in range(len(rgb) // 3)]


# --- the scene, in linear Rec.709: one row per band, each band 32 rows tall in the picture -----

def srgb_decode(c):
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4


def bands():
    """(name, barred, row of WIDTH linear Rec.709 colours)."""
    out = []
    ramp = [2.0 ** (-8 + 14 * x / (WIDTH - 1)) for x in range(WIDTH)]  # 1/256 up to 64
    out.append(("white ramp to 64", True, [(v, v, v) for v in ramp]))
    fire = []
    for x in range(WIDTH):
        t = x / (WIDTH - 1)
        v = 64 * t ** 3
        fire.append((v, v * t ** 1.5, v * t ** 4))  # black, deep red, orange, yellow, white
    out.append(("fire to 64", True, fire))
    for name, (r, g, b) in [("red", (1, 0, 0)), ("green", (0, 1, 0)), ("blue", (0, 0, 1)),
                            ("cyan", (0, 1, 1)), ("magenta", (1, 0, 1)), ("yellow", (1, 1, 0))]:
        out.append(("pure Rec.709 " + name + " ramp to 64", False,
                    [(r * v, g * v, b * v) for v in ramp]))
    return out


CHECKER = [  # X-Rite's published sRGB values, as tools/interpret_reference.py has them
    (115, 82, 68), (194, 150, 130), (98, 122, 157), (87, 108, 67), (133, 128, 177),
    (103, 189, 170), (214, 126, 44), (80, 91, 166), (193, 90, 99), (94, 60, 108),
    (157, 188, 64), (224, 163, 46), (56, 61, 150), (70, 148, 73), (175, 54, 60),
    (231, 199, 31), (187, 86, 149), (8, 133, 161), (243, 243, 242), (200, 200, 200),
    (160, 160, 160), (122, 122, 121), (85, 85, 85), (52, 52, 52),
]


def chart():
    """The 24 patches at every stop from 4 under to 6 over."""
    out = []
    for stop in range(-4, 7):
        for rgb in CHECKER:
            out.append(tuple(srgb_decode(c / 255) * 2.0 ** stop for c in rgb))
    return out


def stress():
    """4000 colours, magnitudes 1/256 to 64, any hue and saturation; half inside Rec.709."""
    rng = random.Random(915)
    out = []
    to_ap1 = config.getProcessor(REC709, AP1).getDefaultCPUProcessor()
    for k in range(4000):
        size = 2.0 ** rng.uniform(-8, 6)
        c = [rng.random() for _ in range(3)]
        top = max(c) or 1.0
        c = [v / top * size for v in c]
        out.append(("rec709", c) if k % 2 == 0 else ("acescg", c))
    rec = [x for kind, c in out if kind == "rec709" for x in c]
    rec = list(to_ap1.applyRGB(rec))
    ap1, i = [], 0
    for kind, c in out:
        if kind == "rec709":
            ap1.append(rec[3 * i:3 * i + 3])
            i += 1
        else:
            ap1.append(c)
    return [g9(v) for c in ap1 for v in c]


# --- the picture and the projects -----------------------------------------------------------

def write_exr(path, width, height, rgba):
    """An uncompressed scanline OpenEXR file, 32-bit float R, G, B, A, no chromaticities
    (tools/interpret_reference.py's writer)."""
    def attr(name, kind, value):
        return name.encode() + b"\0" + kind.encode() + b"\0" + struct.pack("<i", len(value)) + value

    chlist = b"".join(c.encode() + b"\0" + struct.pack("<iB3xii", 2, 0, 1, 1) for c in "ABGR") + b"\0"
    box = struct.pack("<4i", 0, 0, width - 1, height - 1)
    header = (b"\x76\x2f\x31\x01" + struct.pack("<i", 2)
              + attr("channels", "chlist", chlist)
              + attr("compression", "compression", b"\0")
              + attr("dataWindow", "box2i", box)
              + attr("displayWindow", "box2i", box)
              + attr("lineOrder", "lineOrder", b"\0")
              + attr("pixelAspectRatio", "float", struct.pack("<f", 1.0))
              + attr("screenWindowCenter", "v2f", struct.pack("<2f", 0, 0))
              + attr("screenWindowWidth", "float", struct.pack("<f", 1.0))
              + b"\0")
    line_size = width * 4 * 4
    first = len(header) + 8 * height
    offsets = b"".join(struct.pack("<Q", first + y * (8 + line_size)) for y in range(height))
    blocks = []
    for y in range(height):
        row = rgba[y * width:(y + 1) * width]
        data = b"".join(struct.pack("<%df" % width, *[p[ch] for p in row]) for ch in (3, 2, 1, 0))
        blocks.append(struct.pack("<ii", y, line_size) + data)
    with open(path, "wb") as f:
        f.write(header + offsets + b"".join(blocks))


def project(pid, w, h, managed, view=None):
    still = lambda v: {"base": v, "keyframes": []}
    colours = {"working_space": "acescg" if managed else "linear-srgb"}
    if managed:
        colours["color_management"] = True
    colours["alpha_mode"] = "premultiplied"
    if view is not None:
        # After alpha_mode: the build's key order does not rank `view` (effects use the word too).
        colours["view"] = view
    layer = {
        "id": "layer-scene", "kind": "raster", "name": "scene", "asset_id": "asset-scene",
        "enabled": True, "locked": False, "in_frame": 0, "out_frame": 1,
        "source_offset_frames": 0,
        "transform": {"anchor": still([0, 0]), "position": still([0, 0]),
                      "scale": still([100, 100]), "rotation": still(0), "opacity": still(1)},
        "exposure_spans": [], "mask": None, "matte": None, "blend_mode": "normal", "effects": [],
    }
    return {
        "schema_version": 0, "project_id": pid, "color_settings": colours,
        "assets": [{"id": "asset-scene", "kind": "still", "name": "hdr_scene",
                    "path": "media/hdr_scene.exr",
                    "interpretation": {"color_space": "linear-srgb", "alpha": "premultiplied"}}],
        "compositions": [{
            "id": "comp-main", "name": "Main", "width": w, "height": h, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": ["layer-scene"], "layers": [layer],
        }],
    }


def save(rel, value):
    with open(os.path.join(OUT, rel), "w", encoding="utf-8", newline="\n") as f:
        f.write(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


def bake():
    baker = ocio.Baker()
    baker.setConfig(config)
    baker.setFormat("iridas_cube")
    baker.setInputSpace("ACEScct")
    baker.setDisplayView(DISPLAY, VIEWS["aces-2.0-sdr"])
    baker.setCubeSize(SIZE)
    rows = [line.split() for line in baker.bake().splitlines()]
    assert rows[0] == ["LUT_3D_SIZE", str(SIZE)], rows[0]
    values = [float(v) for row in rows[1:] if row for v in row]
    assert len(values) == 3 * SIZE ** 3, len(values)
    data = struct.pack("<%df" % len(values), *values)
    os.makedirs(os.path.dirname(TABLE), exist_ok=True)
    with open(TABLE, "wb") as f:
        f.write(data)
    return hashlib.sha256(data).hexdigest()


def main():
    os.makedirs(os.path.join(OUT, "media"), exist_ok=True)
    sha = bake()
    scene = bands()
    pixels = []
    for _, _, row in scene:
        for _ in range(BAND):
            pixels += [(r, g, b, 1.0) for r, g, b in row]
    h = BAND * len(scene)
    write_exr(os.path.join(OUT, "media", "hdr_scene.exr"), WIDTH, h, pixels)

    cases = []
    for working in WORKING:
        for name, barred, row in scene:
            rgb = convert(REC709, WORKING[working], [v for c in row for v in c])
            cases.append({"name": name, "working": working, "barred": barred,
                          "pixels": [rgb[3 * i:3 * i + 3] for i in range(len(row))],
                          **{v: viewed(working, v, rgb) for v in VIEWS}})
        lin = [v for c in chart() for v in c]
        rgb = convert(REC709, WORKING[working], lin)
        cases.append({"name": "ColorChecker, 4 stops under to 6 over", "working": working,
                      "barred": True, "pixels": [rgb[3 * i:3 * i + 3] for i in range(len(lin) // 3)],
                      **{v: viewed(working, v, rgb) for v in VIEWS}})
    ap1 = stress()
    cases.append({"name": "stress: 4000 colours, half outside Rec.709", "working": "acescg",
                  "barred": False, "pixels": [ap1[3 * i:3 * i + 3] for i in range(len(ap1) // 3)],
                  **{v: viewed("acescg", v, ap1) for v in VIEWS}})

    for pid, managed, view in [
        ("hdr_scene_aces", True, None),
        ("hdr_scene_un_tone_mapped", True, "un-tone-mapped"),
        ("hdr_scene_standard", True, "standard"),
        ("hdr_scene_unknown_view", True, "aces-1.3-sdr"),
        ("hdr_scene_off", False, None),
        ("hdr_scene_off_with_view", False, "aces-2.0-sdr"),
    ]:
        save(pid + ".json", project("proj-" + pid.replace("_", "-"), WIDTH, h, managed, view))

    doc = {
        "about": "D-915 / B-794: OpenColorIO's own views of a synthetic HDR scene, as 8-bit "
                 "levels. Written by tools/view_reference.py; never edit by hand.",
        "ocio_version": VERSION, "config": CONFIG, "optimization": "OPTIMIZATION_LOSSLESS",
        "display": DISPLAY, "views": VIEWS,
        "table": {"file": "assets/colour/aces2_sdr_srgb_97.f32", "size": SIZE,
                  "input": "ACEScct", "format": "little-endian f32 RGB, red fastest", "lookup": "tetrahedral", "sha256": sha},
        "bar": "every case: |ours - expected| at most 1 level of 255 on every channel, on at "
               "least 99.9% of pixels of the cases marked barred; the rest are reported",
        "picture": {"file": "media/hdr_scene.exr", "width": WIDTH, "height": h, "band": BAND,
                    "stored_in": "linear-srgb", "bands": [n for n, _, _ in scene]},
        "cases": cases,
    }
    with open(os.path.join(OUT, "expected_view.json"), "w", encoding="utf-8", newline="\n") as f:
        f.write("{\n")
        keys = list(doc)
        for i, k in enumerate(keys):
            end = ",\n" if i < len(keys) - 1 else "\n"
            if k == "cases":
                f.write('"cases": [\n')
                f.write(",\n".join(json.dumps(c, separators=(",", ":")) for c in doc[k]))
                f.write("\n]" + end)
            else:
                f.write(json.dumps(k) + ": " + json.dumps(doc[k], separators=(",", ":")) + end)
        f.write("}\n")
    print("wrote", OUT, "and", TABLE, "sha256", sha, "cases", len(cases))


assert ocio.__version__ == VERSION, f"PyOpenColorIO {ocio.__version__}, pinned {VERSION}"
config = ocio.Config.CreateFromFile(CONFIG)
main()
