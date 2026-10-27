"""D-909: "Interpret as" and the working space (ADR-023, step C3), OpenColorIO's numbers.

Writes Fixtures/colour/interpret/: a synthetic chart stored in ACES2065-1 (an uncompressed
float EXR), an 8-bit ramp PNG, the projects that tag them, and expected_interpretation.json.
Every expected number is OpenColorIO's, from PyOpenColorIO and its built-in ACES 2.0 studio
config; nothing here calls the build. Beside OCIO's numbers for the 8-bit ramp it also writes
the published curve worked in double precision then OCIO's own matrix ("formula"), because
OCIO's sRGB curve and its ACEScc are not the published formula (D-906) and the build follows
the formula; each case says which of the two it is judged against.

Both pictures are written with the standard library only (zlib for the PNG, an uncompressed
scanline EXR by hand), so the tool needs nothing beyond PyOpenColorIO, the same tool
dependency D-906 recorded (BSD-3-Clause, never shipped):

    python -m venv <folder>/venv
    <folder>/venv/Scripts/python -m pip install opencolorio==2.5.1
    <folder>/venv/Scripts/python -I tools/interpret_reference.py
"""

import json
import os
import struct
import zlib

import PyOpenColorIO as ocio

VERSION = "2.5.1"
CONFIG = "ocio://studio-config-v4.0.0_aces-v2.0_ocio-v2.5"
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "..", "Fixtures", "colour", "interpret")
OPT = ocio.OPTIMIZATION_LOSSLESS  # D-906: the default's fast power would hide the 1e-6 bar

# The saved word, OCIO's space, the linear space of its primaries, and the published curve.
def srgb_decode(c):
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4


def power(g):
    return lambda c: c if c < 0 else c ** g


def acescct_decode(c):
    if c <= 0.155251141552511:
        return (c - 0.0729055341958355) / 10.5402377416545
    return 2.0 ** (c * 17.52 - 9.72)


def acescc_decode(c):
    if c <= (9.72 - 15) / 17.52:
        return (2.0 ** (c * 17.52 - 9.72) - 2.0 ** -16) * 2
    return 2.0 ** (c * 17.52 - 9.72)


REC709, AP1, AP0 = "Linear Rec.709 (sRGB)", "ACEScg", "ACES2065-1"
WORDS = {
    "srgb": ("sRGB Encoded Rec.709 (sRGB)", REC709, srgb_decode),
    "linear-srgb": (REC709, REC709, None),
    "acescg": (AP1, AP1, None),
    "aces2065-1": (AP0, AP0, None),
    "acescct": ("ACEScct", AP1, acescct_decode),
    "acescc": ("ACEScc", AP1, acescc_decode),
    "linear-rec2020": ("Linear Rec.2020", "Linear Rec.2020", None),
    "linear-p3-d65": ("Linear P3-D65", "Linear P3-D65", None),
    "bt1886-rec709": ("Gamma 2.4 Encoded Rec.709", REC709, power(2.4)),
    "gamma22-rec709": ("Gamma 2.2 Encoded Rec.709", REC709, power(2.2)),
}
# OCIO leaves the published formula for these two curves (D-906), so they are judged by it.
BY_FORMULA = {"srgb", "acescc"}
WORKING = {"linear-srgb": REC709, "acescg": AP1}


def f32(x):
    return struct.unpack("<f", struct.pack("<f", x))[0]


def g9(x):
    return float("%.9g" % x)


def run(src, dst, flat):
    if src == dst:
        return [g9(v) for v in flat]
    cpu = config.getProcessor(src, dst).getOptimizedCPUProcessor(OPT)
    return [g9(v) for v in cpu.applyRGB(list(flat))]


def matrix(src, dst):
    """OCIO's own linear-to-linear matrix, composed by OCIO in double precision."""
    ops = list(config.getProcessor(src, dst).getOptimizedProcessor(OPT).createGroupTransform())
    if not ops:
        return [[1, 0, 0], [0, 1, 0], [0, 0, 1]]
    assert len(ops) == 1 and isinstance(ops[0], ocio.MatrixTransform), ops
    m = ops[0].getMatrix()
    return [[m[r * 4 + c] for c in range(3)] for r in range(3)]


def srgb_encode(c):
    return 12.92 * c if c <= 0.0031308 else 1.055 * c ** (1 / 2.4) - 0.055


def view_byte(x):  # document 21's sRGB view and 8-bit quantisation, the build's D-17 rule
    c = srgb_encode(x)
    c = min(max(c, 0.0), 1.0)
    return int(c * 255 + 0.5)


# --- pictures, standard library only ----------------------------------------------------

def write_exr(path, width, height, rgba):
    """An uncompressed scanline OpenEXR file, 32-bit float R, G, B, A, no chromaticities."""
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


def write_png(path, width, height, rgba_bytes):
    """An 8-bit RGBA PNG with no colour chunks: read as sRGB, straight, as every PNG is."""
    def chunk(kind, data):
        return (struct.pack(">I", len(data)) + kind + data
                + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF))

    raw = b"".join(b"\0" + bytes(rgba_bytes[y * width * 4:(y + 1) * width * 4]) for y in range(height))
    with open(path, "wb") as f:
        f.write(b"\x89PNG\r\n\x1a\n"
                + chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0))
                + chunk(b"IDAT", zlib.compress(raw, 9))
                + chunk(b"IEND", b""))


# --- projects ------------------------------------------------------------------------------

def layer(asset_id, w, h):
    still = lambda v: {"base": v, "keyframes": []}
    return {
        "id": "layer-" + asset_id[6:], "kind": "raster", "name": asset_id[6:],
        "asset_id": asset_id, "enabled": True, "locked": False,
        "in_frame": 0, "out_frame": 1, "source_offset_frames": 0,
        "transform": {"anchor": still([0, 0]), "position": still([0, 0]),
                      "scale": still([100, 100]), "rotation": still(0), "opacity": still(1)},
        "exposure_spans": [], "mask": None, "matte": None, "blend_mode": "normal", "effects": [],
    }


def project(pid, assets, w, h, working="linear-srgb", managed=False, shown=1):
    colours = {"working_space": working, "alpha_mode": "premultiplied"}
    if managed:
        colours = {"working_space": working, "color_management": True, "alpha_mode": "premultiplied"}
    layers = [layer(a["id"], w, h) for a in assets[:shown]]
    return {
        "schema_version": 0, "project_id": pid, "color_settings": colours, "assets": assets,
        "compositions": [{
            "id": "comp-main", "name": "Main", "width": w, "height": h, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": [l["id"] for l in layers], "layers": layers,
        }],
    }


def asset(aid, name, path, word, alpha):
    return {"id": aid, "kind": "still", "name": name, "path": path,
            "interpretation": {"color_space": word, "alpha": alpha}}


def save(rel, value):
    with open(os.path.join(OUT, rel), "w", encoding="utf-8", newline="\n") as f:
        f.write(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


# --- the chart -----------------------------------------------------------------------------

# X-Rite's published sRGB values for the 24 ColorChecker patches (post-2014 formulation).
CHECKER = [
    ("dark skin", 115, 82, 68), ("light skin", 194, 150, 130), ("blue sky", 98, 122, 157),
    ("foliage", 87, 108, 67), ("blue flower", 133, 128, 177), ("bluish green", 103, 189, 170),
    ("orange", 214, 126, 44), ("purplish blue", 80, 91, 166), ("moderate red", 193, 90, 99),
    ("purple", 94, 60, 108), ("yellow green", 157, 188, 64), ("orange yellow", 224, 163, 46),
    ("blue", 56, 61, 150), ("green", 70, 148, 73), ("red", 175, 54, 60),
    ("yellow", 231, 199, 31), ("magenta", 187, 86, 149), ("cyan", 8, 133, 161),
    ("white 9.5", 243, 243, 242), ("neutral 8", 200, 200, 200), ("neutral 6.5", 160, 160, 160),
    ("neutral 5", 122, 122, 121), ("neutral 3.5", 85, 85, 85), ("black 2", 52, 52, 52),
]
# A sixth row given in ACEScg: brights past 1 and colours outside Rec.709.
EXTRA = [
    ("ACEScg red 0.5", 0.5, 0, 0), ("ACEScg green 0.5", 0, 0.5, 0), ("ACEScg blue 0.5", 0, 0, 0.5),
    ("grey 2", 2, 2, 2), ("grey 8", 8, 8, 8), ("warm bright", 4, 1, 0.25),
]
PATCH, COLS = 24, 6


def main():
    os.makedirs(os.path.join(OUT, "media"), exist_ok=True)
    patches = []
    lin = [srgb_decode(v / 255) for _, r, g, b in CHECKER for v in (r, g, b)]
    ap0 = run(REC709, AP0, [f32(v) for v in lin])
    for i, (name, r, g, b) in enumerate(CHECKER):
        patches.append({"name": name, "made_from": "sRGB bytes %d %d %d" % (r, g, b),
                        "stored": [f32(v) for v in ap0[3 * i:3 * i + 3]]})
    ap0 = run(AP1, AP0, [f32(v) for e in EXTRA for v in e[1:]])
    for i, (name, *_rgb) in enumerate(EXTRA):
        patches.append({"name": name, "made_from": "ACEScg %g %g %g" % tuple(_rgb),
                        "stored": [f32(v) for v in ap0[3 * i:3 * i + 3]]})
    rows = len(patches) // COLS
    w, h = COLS * PATCH, rows * PATCH
    pixels = []
    for y in range(h):
        for x in range(w):
            p = patches[(y // PATCH) * COLS + x // PATCH]["stored"]
            pixels.append((p[0], p[1], p[2], 1.0))
    write_exr(os.path.join(OUT, "media", "aces_chart.exr"), w, h, pixels)

    stored = [v for p in patches for v in p["stored"]]
    chart_cases = []
    for tag, working, pid in [
        ("aces2065-1", "linear-srgb", "chart_tagged_aces2065_1"),
        ("acescg", "linear-srgb", "chart_tagged_acescg"),
        ("linear-srgb", "linear-srgb", "chart_tagged_linear_rec709"),
        ("aces2065-1", "acescg", "chart_managed_acescg"),
        ("linear-srgb", "acescg", "chart_managed_acescg_linear_rec709"),
    ]:
        managed = working == "acescg"
        save(pid + ".json", project("proj-" + pid.replace("_", "-"),
                                    [asset("asset-chart", "aces_chart", "media/aces_chart.exr", tag,
                                           "premultiplied")], w, h, working, managed))
        out = run(WORDS[tag][0], WORKING[working], stored)
        case = {"project": pid + ".json", "tag": tag, "working": working,
                "ocio": [out[3 * i:3 * i + 3] for i in range(len(patches))]}
        if working == "linear-srgb":
            case["view_bytes"] = [[view_byte(v) for v in out[3 * i:3 * i + 3]]
                                  for i in range(len(patches))]
        chart_cases.append(case)

    # --- the 8-bit ramp: every byte as red, mixed greens and blues, alphas from 0 to 255 -----
    rw = rh = 16
    ramp = []
    for i in range(rw * rh):
        a = 255 if i % 4 else (i * 37) % 256
        ramp += [i, (i * 7 + 3) % 256, 255 - i, a]
    write_png(os.path.join(OUT, "media", "ramp.png"), rw, rh, ramp)
    colour = [f32(ramp[4 * i + c] / 255) for i in range(rw * rh) for c in range(3)]
    ramp_cases = []
    tags = [w_ for w_ in WORDS if w_ not in ("srgb", "linear-srgb")]
    for working in WORKING:
        for tag in (tags if working == "linear-srgb" else list(WORDS)):
            space, linear, decode = WORDS[tag]
            m = matrix(linear, WORKING[working])
            lin = [decode(c) if decode else c for c in colour]
            formula = [float("%.12g" % sum(m[r][k] * lin[3 * i + k] for k in range(3)))
                       for i in range(rw * rh) for r in range(3)]
            ramp_cases.append({"tag": tag, "working": working,
                               "judge": "formula" if tag in BY_FORMULA else "ocio",
                               "ocio": run(space, WORKING[working], colour),
                               "formula": formula})
    ramp_assets = [asset("asset-" + t, "ramp " + t, "media/ramp.png", t, "straight") for t in tags]
    save("ramp_tags.json", project("proj-ramp-tags", ramp_assets, rw, rh, shown=len(ramp_assets)))
    all_assets = [asset("asset-" + t, "ramp " + t, "media/ramp.png", t, "straight") for t in WORDS]
    save("ramp_tags_managed_acescg.json",
         project("proj-ramp-managed", all_assets, rw, rh, "acescg", True, shown=len(all_assets)))

    # --- what is kept, and what is refused ------------------------------------------------------
    save("unknown_space.json", project("proj-unknown-space", [
        asset("asset-chart", "aces_chart", "media/aces_chart.exr", "arri-logc4-awg4",
              "premultiplied")], w, h))
    refused = project("proj-acescg-unmanaged", [], w, h, "acescg")
    save("refused_acescg_without_management.json", refused)
    save("refused_unknown_working_space.json",
         project("proj-unknown-working", [], w, h, "linear-rec2020", True))

    doc = {
        "about": "D-909 / B-788: OpenColorIO's numbers for \"Interpret as\" and the working space. "
                 "Written by tools/interpret_reference.py; never edit by hand.",
        "ocio_version": VERSION, "config": CONFIG, "optimization": "OPTIMIZATION_LOSSLESS",
        "bar": "|ours - expected| <= 1e-6 * scale, scale the largest of 1 and the magnitudes of "
               "the colour expected; colour before alpha, which the build then multiplies by "
               "(document 21 step 1)",
        "words": {k: v[0] for k, v in WORDS.items()},
        "working_spaces": WORKING,
        "chart": {"file": "media/aces_chart.exr", "width": w, "height": h, "patch": PATCH,
                  "columns": COLS, "stored_in": "aces2065-1", "patches": patches,
                  "cases": chart_cases},
        "ramp": {"file": "media/ramp.png", "width": rw, "height": rh, "rgba": ramp,
                 "cases": ramp_cases},
    }
    with open(os.path.join(OUT, "expected_interpretation.json"), "w", encoding="utf-8",
              newline="\n") as f:
        f.write("{\n")
        keys = list(doc)
        for i, k in enumerate(keys):
            end = ",\n" if i < len(keys) - 1 else "\n"
            if k in ("chart", "ramp"):
                inner = doc[k]
                f.write(json.dumps(k) + ": {\n")
                ik = list(inner)
                for j, kk in enumerate(ik):
                    e2 = ",\n" if j < len(ik) - 1 else "\n"
                    if isinstance(inner[kk], list) and inner[kk] and isinstance(inner[kk][0], dict):
                        f.write(json.dumps(kk) + ": [\n")
                        f.write(",\n".join(json.dumps(c, separators=(",", ":")) for c in inner[kk]))
                        f.write("\n]" + e2)
                    else:
                        f.write(json.dumps(kk) + ": " + json.dumps(inner[kk], separators=(",", ":")) + e2)
                f.write("}" + end)
            else:
                f.write(json.dumps(k) + ": " + json.dumps(doc[k], separators=(",", ":")) + end)
        f.write("}\n")
    print(f"wrote {OUT}: chart {w}x{h} ({len(patches)} patches, {len(chart_cases)} cases), "
          f"ramp {rw}x{rh} ({len(ramp_cases)} cases)")
    for c in chart_cases:
        if "view_bytes" in c:
            print(c["tag"], "row 1:", c["view_bytes"][:6])


assert ocio.__version__ == VERSION, f"PyOpenColorIO {ocio.__version__}, pinned {VERSION}"
config = ocio.Config.CreateFromFile(CONFIG)
main()
