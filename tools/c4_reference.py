"""D-910: colour management's step C4 (ADR-023), the expected pictures.

Writes Fixtures/colour/c4/: four pictures made only of stored colours and display-value rules
(a solid, a Gradient, an Overlay and a luma matte), each as a project working in linear
Rec.709 and in ACEScg with colour management on; two projects colour management refuses
(8 bpc (After Effects) and 32 bpc (After Effects)) and one it opens (Float); and
expected_c4.json.

Every expected number is worked here, never by the build: the published sRGB curve, the
documented rules of D-74 (a solid's colour is linear Rec.709), D-114 (Gradient mixes its two
`#rrggbb` colours as encoded values, then decodes), D-301 (Overlay works on the encoded colours)
and D-293 (a luma matte's cover is the encoded Rec.709 luma), in double precision; and
OpenColorIO 2.5.1's own matrix for linear Rec.709 to ACEScg (PyOpenColorIO, the built-in
config `ocio://studio-config-v4.0.0_aces-v2.0_ocio-v2.5`, `OPTIMIZATION_LOSSLESS`), the same
tool dependency D-906 recorded (BSD-3-Clause, never shipped):

    <venv>/Scripts/python -m pip install opencolorio==2.5.1
    <venv>/Scripts/python -I tools/c4_reference.py
"""

import json
import os

import PyOpenColorIO as ocio

VERSION = "2.5.1"
CONFIG = "ocio://studio-config-v4.0.0_aces-v2.0_ocio-v2.5"
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "..", "Fixtures", "colour", "c4")
OPT = ocio.OPTIMIZATION_LOSSLESS
W, H = 16, 4

assert ocio.__version__ == VERSION, ocio.__version__
config = ocio.Config.CreateFromFile(CONFIG)
TO_AP1 = config.getProcessor("Linear Rec.709 (sRGB)", "ACEScg").getOptimizedCPUProcessor(OPT)


def decode(c):
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4


def encode(c):
    return 12.92 * c if c <= 0.0031308 else 1.055 * c ** (1 / 2.4) - 0.055


def view_byte(x):  # today's view: D-17's sRGB curve, held to 0..1, 8 bits
    return int(min(max(encode(x), 0.0), 1.0) * 255 + 0.5)


def hexa(word):
    return [int(word[i:i + 2], 16) / 255 for i in (1, 3, 5)]


def g12(x):
    return float("%.12g" % x)


# --- projects ------------------------------------------------------------------------------

def still(v):
    return {"base": v, "keyframes": []}


def solid(lid, colour, blend="normal", effects=(), matte=None):
    return {
        "id": lid, "kind": "solid", "name": lid, "enabled": True, "locked": False,
        "in_frame": 0, "out_frame": 1,
        "transform": {"anchor": still([0, 0]), "position": still([0, 0]),
                      "scale": still([100, 100]), "rotation": still(0), "opacity": still(1)},
        "mask": None, "matte": matte, "blend_mode": blend, "effects": list(effects),
        "solid": {"color": colour, "width": W, "height": H},
    }


def gradient(start_color, end_color):
    return {
        "instance_id": "fx-gradient", "type_id": "core.gradient", "enabled": True,
        "parameters": {
            "shape": "linear", "start": [0, 50], "end": [100, 50],
            "start_color": start_color, "end_color": end_color,
            "start_opacity": 100, "end_opacity": 100, "blend": "normal",
        },
    }


def project(pid, layers, working=None, depth=None):
    colours = {"working_space": "linear-srgb", "alpha_mode": "premultiplied"}
    if working:
        colours = {"working_space": working, "color_management": True,
                   "alpha_mode": "premultiplied"}
    comp = {
        "id": "comp-main", "name": "Main", "width": W, "height": H, "pixel_aspect_ratio": 1,
        "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
        "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
        "layer_order": [l["id"] for l in layers], "layers": layers,
    }
    comp.update(depth or {})
    return {"schema_version": 0, "project_id": pid, "color_settings": colours, "assets": [],
            "compositions": [comp]}


def save(rel, value):
    with open(os.path.join(OUT, rel), "w", encoding="utf-8", newline="\n") as f:
        f.write(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


# --- the four pictures, each a row of 16 pixels repeated down the frame -------------------

def t_at(x):  # D-114: (x + 0.5) along a line from the left edge to the right, centre to centre
    return (x + 0.5) / W


ORANGE = [0.8, 0.3, 0.05]
G0, G1 = "#ff8000", "#2040c0"
BELOW, ABOVE = [0.2, 0.5, 0.05], [0.6, 0.1, 0.4]
RED, M0, M1 = [0.9, 0.1, 0.05], "#000000", "#40c0ff"


def overlay(b, c):  # D-301: grade::mixer's Overlay, the colour beneath first
    return 2 * b * c if b <= 0.5 else 1 - 2 * (1 - b) * (1 - c)


def row(case):
    out = []
    for x in range(W):
        t = t_at(x)
        if case == "solid":
            out.append(list(ORANGE))
        elif case == "gradient":
            a, b = hexa(G0), hexa(G1)
            out.append([decode(a[c] + t * (b[c] - a[c])) for c in range(3)])
        elif case == "overlay":
            out.append([decode(overlay(encode(BELOW[c]), encode(ABOVE[c]))) for c in range(3)])
        elif case == "luma_matte":
            a, b = hexa(M0), hexa(M1)
            m = [decode(a[c] + t * (b[c] - a[c])) for c in range(3)]
            cover = encode(min(max(0.2126 * m[0] + 0.7152 * m[1] + 0.0722 * m[2], 0), 1))
            out.append([RED[c] * cover for c in range(3)])  # over an opaque black solid
    return out


LAYERS = {
    "solid": [solid("orange", ORANGE)],
    "gradient": [solid("ramp", [1.0, 1.0, 1.0], effects=[gradient(G0, G1)])],
    "overlay": [solid("below", BELOW), solid("above", ABOVE, blend="overlay")],
    "luma_matte": [
        solid("black", [0.0, 0.0, 0.0]),
        solid("matte", [1.0, 1.0, 1.0], effects=[gradient(M0, M1)]),
        solid("red", RED, matte={"layer_id": "matte", "mode": "luma", "matte_only": True}),
    ],
}

os.makedirs(OUT, exist_ok=True)
expected = {
    "tool": f"tools/c4_reference.py, PyOpenColorIO {VERSION}, {CONFIG}",
    "bar": "|build - expected| at most 1e-5 in linear light, every pixel; the two working "
           "spaces' pictures the same bytes through today's view",
    "size": [W, H],
    "cases": {},
}
for case, layers in LAYERS.items():
    for working in ("linear-srgb", "acescg"):
        name = f"{case}_{'rec709' if working == 'linear-srgb' else 'acescg'}.json"
        save(name, project(f"proj-c4-{case}-{working}", layers, working))
    rec709 = row(case)
    acescg = [list(TO_AP1.applyRGB(list(px))) for px in rec709]
    expected["cases"][case] = {
        "projects": [f"{case}_rec709.json", f"{case}_acescg.json"],
        "rec709": [[g12(v) for v in px] for px in rec709],
        "acescg": [[g12(v) for v in px] for px in acescg],
        "view": [[view_byte(v) for v in px] for px in rec709],
    }

save("refused_eight_bpc.json", project("proj-c4-eight", LAYERS["solid"], "acescg",
                                       {"eight_bpc": True}))
save("refused_ae_32bpc.json", project("proj-c4-ae32", LAYERS["solid"], "acescg",
                                      {"float_depth": True, "ae_32bpc": True}))
save("float_acescg.json", project("proj-c4-float", LAYERS["solid"], "acescg",
                                  {"float_depth": True}))
expected["refused"] = ["refused_eight_bpc.json", "refused_ae_32bpc.json"]
expected["opens"] = ["float_acescg.json"]
save("expected_c4.json", expected)
print("wrote", OUT)
