"""D-917: the export's Output with colour management on (ADR-023, step C6), OpenColorIO's numbers.

Writes Fixtures/colour/output/:
  media/output_chart.exr   a 264 by 4 float picture stored in linear Rec.709, premultiplied:
                           row 0 a white ramp from 1/256 to 64, row 1 a fire ramp to 64, row 2
                           the 24 ColorChecker patches at every stop from 4 under to 6 over,
                           row 3 an orange with alpha from 0 to 1 (premultiplied in the file)
  chart_acescg.json        the picture in a project with colour management on, ACEScg working
  chart_rec709.json        the same, linear Rec.709 working
  chart_off.json           the same, colour management off
  expected_output.json     for both working spaces, every Output the export offers:
                           the picture outputs (sRGB, Rec.709 BT.1886, the ACES 2.0 SDR view) as
                           8-bit and 16-bit levels with straight alpha and 8-bit levels with
                           premultiplied alpha; the EXR outputs (working, ACEScg, ACES2065-1) as
                           float RGBA; and what each file must say of its colour

Every expected number is OpenColorIO's, from PyOpenColorIO and its built-in ACES 2.0 studio
config; nothing here calls the build. The outputs are OpenColorIO's own transforms:
  sRGB       display "sRGB - Display", view "Un-tone-mapped"
  Rec.709    display "Rec.1886 Rec.709 - Display", view "Un-tone-mapped"
  ACES view  display "sRGB - Display", view "ACES 2.0 - SDR 100 nits (Rec.709)", analytic
             (the build uses D-915's baked table, so it is judged by D-915's bar)
  ACEScg     colour space "ACEScg";  ACES2065-1  colour space "ACES2065-1"

The tool needs only PyOpenColorIO, the tool dependency D-906 recorded (BSD-3-Clause, never
shipped):

    python -m venv <folder>/venv
    <folder>/venv/Scripts/python -m pip install opencolorio==2.5.1
    <folder>/venv/Scripts/python -I tools/output_reference.py
"""

import json
import math
import os
import struct

import PyOpenColorIO as ocio

VERSION = "2.5.1"
CONFIG = "ocio://studio-config-v4.0.0_aces-v2.0_ocio-v2.5"
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "..", "Fixtures", "colour", "output")
OPT = ocio.OPTIMIZATION_LOSSLESS
REC709, AP1, AP0 = "Linear Rec.709 (sRGB)", "ACEScg", "ACES2065-1"
WORKING = {"acescg": AP1, "linear-srgb": REC709}
WIDTH = 264

# The protocol word, what it is, and how OpenColorIO makes it.
PICTURE = {
    "default": ("sRGB", ("sRGB - Display", "Un-tone-mapped")),
    "rec709": ("Rec.709 (BT.1886)", ("Rec.1886 Rec.709 - Display", "Un-tone-mapped")),
    "aces-view": ("ACES 2.0 SDR view", ("sRGB - Display", "ACES 2.0 - SDR 100 nits (Rec.709)")),
}
EXR = {"default": ("Working space", None), "acescg": ("ACEScg", AP1), "aces2065-1": ("ACES2065-1", AP0)}

# What each file must say, with colour management on (with it off every file is as before).
# PNG: the sRGB chunk (rendering intent 0, perceptual), or cICP (ITU-T H.273 code points:
# primaries 1 BT.709, transfer 1 BT.709 / 13 IEC 61966-2-1, matrix 0 RGB, full range 1).
# MP4: the `colr` nclx box (primaries, transfer, matrix, full range flag), as FX-FMT-060 reads it;
# with colour management off it stays 1, 1, 1, 0 (D-73).
# EXR: the chromaticities attribute, the published primaries and white of each space.
REC709_CHROMA = [[0.64, 0.33], [0.30, 0.60], [0.15, 0.06], [0.3127, 0.3290]]
AP1_CHROMA = [[0.713, 0.293], [0.165, 0.830], [0.128, 0.044], [0.32168, 0.33767]]
AP0_CHROMA = [[0.7347, 0.2653], [0.0, 1.0], [0.0001, -0.077], [0.32168, 0.33767]]
LABELS = {
    "picture": {
        "default": {"png_chunk": "sRGB 0", "mp4_colr": [1, 13, 1, 0],
                    "colour_tag": "sRGB IEC 61966-2-1"},
        "rec709": {"png_chunk": "cICP 1 1 0 1", "mp4_colr": [1, 1, 1, 0],
                   "colour_tag": "Rec.709, BT.1886 (gamma 2.4)"},
        "aces-view": {"png_chunk": "sRGB 0", "mp4_colr": [1, 13, 1, 0],
                      "colour_tag": "sRGB IEC 61966-2-1, the ACES 2.0 SDR view"},
    },
    "off": {"png_chunk": None, "mp4_colr": [1, 1, 1, 0],
            "colour_tag": "sRGB IEC 61966-2-1"},
    "exr": {
        "acescg": {"default": AP1_CHROMA, "acescg": AP1_CHROMA, "aces2065-1": AP0_CHROMA},
        "linear-srgb": {"default": REC709_CHROMA, "acescg": AP1_CHROMA, "aces2065-1": AP0_CHROMA},
        "off": {"default": REC709_CHROMA},
    },
}


def f32(x):
    return struct.unpack("<f", struct.pack("<f", x))[0]


def g9(x):
    return float("%.9g" % x)


def level(v, top):  # color::quantise_u8 / quantise_u16: clamp, times the top, round half up
    v = 0.0 if math.isnan(v) else min(max(v, 0.0), 1.0)
    return int(math.floor(v * top + 0.5))


def srgb_decode(c):
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4


CHECKER = [  # X-Rite's published sRGB values, as tools/interpret_reference.py has them
    (115, 82, 68), (194, 150, 130), (98, 122, 157), (87, 108, 67), (133, 128, 177),
    (103, 189, 170), (214, 126, 44), (80, 91, 166), (193, 90, 99), (94, 60, 108),
    (157, 188, 64), (224, 163, 46), (56, 61, 150), (70, 148, 73), (175, 54, 60),
    (231, 199, 31), (187, 86, 149), (8, 133, 161), (243, 243, 242), (200, 200, 200),
    (160, 160, 160), (122, 122, 121), (85, 85, 85), (52, 52, 52),
]


def rows():
    """(name, [premultiplied linear Rec.709 RGBA, as the file stores them, f32])."""
    out = []
    ramp = [2.0 ** (-8 + 14 * x / (WIDTH - 1)) for x in range(WIDTH)]
    out.append(("white ramp to 64", [(v, v, v, 1.0) for v in ramp]))
    fire = []
    for x in range(WIDTH):
        t = x / (WIDTH - 1)
        v = 64 * t ** 3
        fire.append((v, v * t ** 1.5, v * t ** 4, 1.0))
    out.append(("fire to 64", fire))
    out.append(("ColorChecker, 4 stops under to 6 over",
                [tuple(srgb_decode(c / 255) * 2.0 ** stop for c in rgb) + (1.0,)
                 for stop in range(-4, 7) for rgb in CHECKER]))
    orange = []
    for x in range(WIDTH):
        a = x / (WIDTH - 1)
        orange.append((0.8 * a, 0.3 * a, 0.05 * a, a))
    out.append(("orange, alpha 0 to 1", orange))
    return [(n, [tuple(f32(v) for v in p) for p in r]) for n, r in out]


def processor(src, picture_word=None, exr_space=None):
    if picture_word is not None:
        display, view = PICTURE[picture_word][1]
        t = ocio.DisplayViewTransform(src=src, display=display, view=view)
        return config.getProcessor(t).getOptimizedCPUProcessor(OPT)
    return config.getProcessor(src, exr_space).getOptimizedCPUProcessor(OPT)


def apply(cpu, rgbs):
    flat = list(cpu.applyRGB([v for c in rgbs for v in c]))
    return [flat[3 * i:3 * i + 3] for i in range(len(rgbs))]


def unpremultiply(p):  # composite::unpremultiply
    r, g, b, a = p
    if a == 0.0:
        return (0.0, 0.0, 0.0, a)
    return (f32(r / a), f32(g / a), f32(b / a), a)


def write_exr(path, width, height, rgba):
    """An uncompressed scanline OpenEXR file, 32-bit float R, G, B, A, no chromaticities
    (tools/view_reference.py's writer)."""
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


def project(pid, working, managed):
    still = lambda v: {"base": v, "keyframes": []}
    colours = {"working_space": working}
    if managed:
        colours["color_management"] = True
    colours["alpha_mode"] = "premultiplied"
    layer = {
        "id": "layer-chart", "kind": "raster", "name": "chart", "asset_id": "asset-chart",
        "enabled": True, "locked": False, "in_frame": 0, "out_frame": 2,
        "source_offset_frames": 0,
        "transform": {"anchor": still([0, 0]), "position": still([0, 0]),
                      "scale": still([100, 100]), "rotation": still(0), "opacity": still(1)},
        "exposure_spans": [], "mask": None, "matte": None, "blend_mode": "normal", "effects": [],
    }
    return {
        "schema_version": 0, "project_id": pid, "color_settings": colours,
        "assets": [{"id": "asset-chart", "kind": "still", "name": "output_chart",
                    "path": "media/output_chart.exr",
                    "interpretation": {"color_space": "linear-srgb", "alpha": "premultiplied"}}],
        "compositions": [{
            "id": "comp-main", "name": "Main", "width": WIDTH, "height": 4,
            "pixel_aspect_ratio": 1, "frame_rate": {"numerator": 24, "denominator": 1},
            "start_frame": 0, "duration_frames": 2,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 2},
            "layer_order": ["layer-chart"], "layers": [layer],
        }],
    }


def save(rel, value):
    with open(os.path.join(OUT, rel), "w", encoding="utf-8", newline="\n") as f:
        f.write(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


def main():
    os.makedirs(os.path.join(OUT, "media"), exist_ok=True)
    picture = rows()
    pixels = [p for _, r in picture for p in r]
    write_exr(os.path.join(OUT, "media", "output_chart.exr"), WIDTH, len(picture), pixels)
    for pid, working, managed in [("chart_acescg", "acescg", True),
                                  ("chart_rec709", "linear-srgb", True),
                                  ("chart_off", "linear-srgb", False)]:
        save(pid + ".json", project("proj-" + pid.replace("_", "-"), working, managed))

    cases, exr = [], []
    for working, space in WORKING.items():
        # The working numbers: the file's colours through OpenColorIO's matrix, alpha untouched.
        to_working = config.getProcessor(REC709, space).getOptimizedCPUProcessor(OPT)
        premul = [tuple(c) + (p[3],) for c, p in
                  zip(apply(to_working, [p[:3] for p in pixels]), pixels)]
        premul = [tuple(f32(v) for v in p) for p in premul]
        straight = [unpremultiply(p) for p in premul]
        for word in PICTURE:
            cpu = processor(space, picture_word=word)
            s = apply(cpu, [p[:3] for p in straight])
            m = apply(cpu, [p[:3] for p in premul])
            cases.append({
                "working": working, "output": word,
                "eight_straight": [[level(v, 255) for v in c] + [level(p[3], 255)] for c, p in zip(s, straight)],
                "sixteen_straight": [[level(v, 65535) for v in c] + [level(p[3], 65535)] for c, p in zip(s, straight)],
                "eight_premultiplied": [[level(v, 255) for v in c] + [level(p[3], 255)] for c, p in zip(m, premul)],
            })
        for word, (_, target) in EXR.items():
            cpu = processor(REC709, exr_space=target or space)
            values = apply(cpu, [p[:3] for p in pixels])
            exr.append({"working": working, "output": word,
                        "rgba": [[g9(v) for v in c] + [p[3]] for c, p in zip(values, pixels)]})

    doc = {
        "about": "D-917 / B-796: OpenColorIO's own numbers for every Output an export offers "
                 "with colour management on. Written by tools/output_reference.py; never edit "
                 "by hand.",
        "ocio_version": VERSION, "config": CONFIG, "optimization": "OPTIMIZATION_LOSSLESS",
        "picture_outputs": {w: {"name": n, "display": d, "view": v} for w, (n, (d, v)) in PICTURE.items()},
        "exr_outputs": {w: {"name": n, "space": s or "the working space"} for w, (n, s) in EXR.items()},
        "labels": LABELS,
        "bar": {
            "picture": "sRGB and Rec.709: |ours - expected| at most 1 level on every channel of "
                       "every pixel, at 8 and at 16 bits. ACES view (D-915's baked table): at "
                       "most 1 level of 255 on 99.9% of pixels at 8 bits, and at 16 bits at most "
                       "257 levels (1 of 255) on 99.9%; the rest reported",
            "exr": "|ours - expected| at most 1e-6 times the colour's size (its largest channel, "
                   "at least 1) on every channel of every pixel; alpha exact",
            "off": "with colour management off every byte is what the export wrote before D-917",
        },
        "picture": {"file": "media/output_chart.exr", "width": WIDTH, "height": len(picture),
                    "stored_in": "linear-srgb, premultiplied", "rows": [n for n, _ in picture]},
        "cases": cases,
        "exr": exr,
    }
    with open(os.path.join(OUT, "expected_output.json"), "w", encoding="utf-8", newline="\n") as f:
        f.write("{\n")
        keys = list(doc)
        for i, k in enumerate(keys):
            end = ",\n" if i < len(keys) - 1 else "\n"
            if k in ("cases", "exr"):
                f.write(json.dumps(k) + ": [\n")
                f.write(",\n".join(json.dumps(c, separators=(",", ":")) for c in doc[k]))
                f.write("\n]" + end)
            else:
                f.write(json.dumps(k) + ": " + json.dumps(doc[k], separators=(",", ":")) + end)
        f.write("}\n")
    print("wrote", OUT, "picture cases", len(cases), "exr cases", len(exr))


assert ocio.__version__ == VERSION, f"PyOpenColorIO {ocio.__version__}, pinned {VERSION}"
config = ocio.Config.CreateFromFile(CONFIG)
main()
