"""D-918: the colour picker's, the eyedropper's and the Info row's two sets of numbers (ADR-023,
step C7), OpenColorIO's own.

Writes Fixtures/colour/c7/expected_readout.json:
  picker  the 24 ColorChecker swatches and nine plain ones as the picker's hex; for each, the
          number the file stores (the sRGB curve's published decode, D-910's linear Rec.709),
          the working numbers in both working spaces, and the display levels of both views from
          both working spaces
  pixels  points of D-915's scene (Fixtures/colour/view/media/hdr_scene.exr, 512 by 256, eight
          bands of 32 rows) with the same numbers, the picture's own 32-bit floats

Every expected number is OpenColorIO's, from PyOpenColorIO and its built-in ACES 2.0 studio
config; nothing here calls the build. Run as tools/view_reference.py says:

    <folder>/venv/Scripts/python -I tools/c7_reference.py
"""

import json
import math
import os
import struct

import PyOpenColorIO as ocio

VERSION = "2.5.1"
CONFIG = "ocio://studio-config-v4.0.0_aces-v2.0_ocio-v2.5"
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "..", "Fixtures", "colour", "c7")
OPT = ocio.OPTIMIZATION_LOSSLESS
DISPLAY = "sRGB - Display"
VIEWS = {"aces-2.0-sdr": "ACES 2.0 - SDR 100 nits (Rec.709)", "un-tone-mapped": "Un-tone-mapped"}
REC709, AP1 = "Linear Rec.709 (sRGB)", "ACEScg"
WORKING = {"acescg": AP1, "linear-srgb": REC709}
WIDTH, BAND = 512, 32

CHECKER = [  # X-Rite's published sRGB values, as tools/view_reference.py has them
    (115, 82, 68), (194, 150, 130), (98, 122, 157), (87, 108, 67), (133, 128, 177),
    (103, 189, 170), (214, 126, 44), (80, 91, 166), (193, 90, 99), (94, 60, 108),
    (157, 188, 64), (224, 163, 46), (56, 61, 150), (70, 148, 73), (175, 54, 60),
    (231, 199, 31), (187, 86, 149), (8, 133, 161), (243, 243, 242), (200, 200, 200),
    (160, 160, 160), (122, 122, 121), (85, 85, 85), (52, 52, 52),
]
PLAIN = [(255, 128, 0), (0, 0, 0), (255, 255, 255), (128, 128, 128), (255, 0, 0), (0, 255, 0),
         (0, 0, 255), (1, 1, 1), (254, 254, 254)]


def g9(x):
    return float("%.9g" % x)


def f32(x):
    return struct.unpack("<f", struct.pack("<f", x))[0]


def level(v):  # color::quantise_u8: clamp, times 255, round half up
    return int(math.floor(min(max(v, 0.0), 1.0) * 255 + 0.5))


def srgb_decode(c):
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4


def convert(src, dst, rgb):
    if src == dst:
        return [g9(v) for v in rgb]
    cpu = config.getProcessor(src, dst).getOptimizedCPUProcessor(OPT)
    return [g9(v) for v in cpu.applyRGB(list(rgb))]


def viewed(working, view, rgb):
    t = ocio.DisplayViewTransform(src=WORKING[working], display=DISPLAY, view=VIEWS[view])
    out = config.getProcessor(t).getOptimizedCPUProcessor(OPT).applyRGB(list(rgb))
    return [level(v) for v in out]


def numbers(stored):
    """Working numbers and display levels of one stored (linear Rec.709) colour."""
    working, display = {}, {}
    for w, space in WORKING.items():
        rgb = convert(REC709, space, stored)
        working[w] = rgb
        display[w] = {v: viewed(w, v, rgb) for v in VIEWS}
    return working, display


def bands():
    """(name, barred, colour at x) of tools/view_reference.py's scene, in linear Rec.709."""
    ramp = lambda x: 2.0 ** (-8 + 14 * x / (WIDTH - 1))
    def fire(x):
        t = x / (WIDTH - 1)
        v = 64 * t ** 3
        return (v, v * t ** 1.5, v * t ** 4)
    out = [("white ramp to 64", True, lambda x: (ramp(x),) * 3), ("fire to 64", True, fire)]
    for name, (r, g, b) in [("red", (1, 0, 0)), ("green", (0, 1, 0)), ("blue", (0, 0, 1)),
                            ("cyan", (0, 1, 1)), ("magenta", (1, 0, 1)), ("yellow", (1, 1, 0))]:
        out.append(("pure Rec.709 " + name + " ramp to 64", False,
                    lambda x, r=r, g=g, b=b: (r * ramp(x), g * ramp(x), b * ramp(x))))
    return out


def main():
    os.makedirs(OUT, exist_ok=True)
    picker = []
    for rgb in CHECKER + PLAIN:
        hex_ = "#" + "".join("%02x" % c for c in rgb)
        stored = [g9(srgb_decode(c / 255)) for c in rgb]
        # The picker's own round trip: the stored number back through the sRGB curve is the hex.
        back = [level(1.055 * s ** (1 / 2.4) - 0.055 if s > 0.0031308 else 12.92 * s) for s in stored]
        assert back == list(rgb), (hex_, back)
        working, display = numbers(stored)
        picker.append({"hex": hex_, "stored": stored, "working": working, "display": display})

    pixels = []
    for b, (name, barred, colour) in enumerate(bands()):
        for x in (0, 64, 128, 200, 255, 300, 400, 511):
            stored = [g9(f32(v)) for v in colour(x)]  # the picture holds 32-bit floats
            working, display = numbers(stored)
            pixels.append({"x": x, "y": BAND * b + BAND // 2, "band": name, "barred": barred,
                           "stored": stored, "working": working, "display": display})

    doc = {
        "about": "D-918 / B-797: the picker's, the eyedropper's and the Info row's numbers with "
                 "colour management on, OpenColorIO's own. Written by tools/c7_reference.py; "
                 "never edit by hand.",
        "ocio_version": VERSION, "config": CONFIG, "optimization": "OPTIMIZATION_LOSSLESS",
        "display": DISPLAY, "views": VIEWS,
        "bars": {
            "working": "|ours - expected| at most 1e-6 times max(1, |expected|), every channel",
            "stored": "a pixel's stored number back from its working numbers within the same bar; "
                      "a picker hex's stored number the sRGB curve's decode within the same bar, and its "
                      "hex back exactly the hex",
            "display": "at most 1 level of 255 from OpenColorIO on every channel of the picker "
                       "cases and the barred bands (the rest are reported), and exactly the "
                       "viewer's own byte at that pixel",
            "off": "colour management off: the window says so and shows nothing new",
        },
        "pixels_of": {"projects": ["../view/hdr_scene_aces.json", "../view/hdr_scene_un_tone_mapped.json"],
                      "working": "acescg", "frame": 0},
        "picker": picker,
        "pixels": pixels,
    }
    with open(os.path.join(OUT, "expected_readout.json"), "w", encoding="utf-8", newline="\n") as f:
        f.write("{\n")
        keys = list(doc)
        for i, k in enumerate(keys):
            end = ",\n" if i < len(keys) - 1 else "\n"
            if k in ("picker", "pixels"):
                f.write(json.dumps(k) + ": [\n")
                f.write(",\n".join(json.dumps(c, separators=(",", ":")) for c in doc[k]))
                f.write("\n]" + end)
            else:
                f.write(json.dumps(k) + ": " + json.dumps(doc[k], separators=(",", ":")) + end)
        f.write("}\n")
    print("wrote", OUT, "picker", len(picker), "pixels", len(pixels))


assert ocio.__version__ == VERSION, f"PyOpenColorIO {ocio.__version__}, pinned {VERSION}"
config = ocio.Config.CreateFromFile(CONFIG)
main()
