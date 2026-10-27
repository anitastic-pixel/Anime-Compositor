"""D-906: OpenColorIO's own numbers for the build's named colour spaces (ADR-023, step C2).

Writes Fixtures/colour/expected_conversions.json. Every expected number is OpenColorIO's,
computed by PyOpenColorIO from its built-in ACES 2.0 studio config; nothing here calls the
build. Beside each curve's numbers it also writes the published formula worked in double
precision by this script ("formula"), so that where OpenColorIO and the build disagree the
table can say which of the two the formula sides with.

Tool dependency, never shipped (ADR-023, owner's answer 8): PyOpenColorIO 2.5.1 from PyPI
(BSD-3-Clause), installed in its own virtual environment, never in the build:

    python -m venv <folder>/venv
    <folder>/venv/Scripts/python -m pip install opencolorio==2.5.1
    <folder>/venv/Scripts/python -I tools/ocio_reference.py

The config is OCIO's built-in one, read by its URI; nothing is downloaded.
"""

import json
import math
import os
import struct

import PyOpenColorIO as ocio

VERSION = "2.5.1"
CONFIG = "ocio://studio-config-v4.0.0_aces-v2.0_ocio-v2.5"
HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "..", "Fixtures", "colour", "expected_conversions.json")

# OPTIMIZATION_LOSSLESS, not OCIO's default. The default (VERY_GOOD) includes
# FAST_LOG_EXP_POW, an approximate power that is off by up to 2.5e-5 relative on the sRGB curve
# (measured while writing this script), which would hide the 1e-6 bar under OCIO's own shortcut.
OPTIMIZATION = ocio.OPTIMIZATION_LOSSLESS

# The build's linear spaces and OCIO's names for them.
LINEAR = {
    "linear_rec709": "Linear Rec.709 (sRGB)",
    "acescg": "ACEScg",
    "aces2065_1": "ACES2065-1",
    "linear_rec2020": "Linear Rec.2020",
    "linear_p3_d65": "Linear P3-D65",
}

# The build's curves: (encoded space id, OCIO's encoded space, linear space id, the formula).
def srgb_decode(c):  # IEC 61966-2-1, the build's D-17 curve
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4


def srgb_encode(c):
    return 12.92 * c if c <= 0.0031308 else 1.055 * c ** (1 / 2.4) - 0.055


def power(g):  # OCIO's ExponentTransform, style pass_thru: a negative passes unchanged
    return (lambda c: c if c < 0 else c ** g), (lambda c: c if c < 0 else c ** (1 / g))


def acescct_decode(c):  # ACES S-2016-001, without the half-float cap (as OCIO)
    if c <= 0.155251141552511:
        return (c - 0.0729055341958355) / 10.5402377416545
    return 2.0 ** (c * 17.52 - 9.72)


def acescct_encode(c):
    if c <= 0.0078125:
        return 10.5402377416545 * c + 0.0729055341958355
    return (math.log2(c) + 9.72) / 17.52


def acescc_decode(c):  # ACES S-2014-003, without the half-float cap
    if c <= (9.72 - 15) / 17.52:
        return (2.0 ** (c * 17.52 - 9.72) - 2.0 ** -16) * 2
    return 2.0 ** (c * 17.52 - 9.72)


def acescc_encode(c):
    if c <= 0:
        return (-16 + 9.72) / 17.52
    if c < 2.0 ** -15:
        return (math.log2(2.0 ** -16 + c * 0.5) + 9.72) / 17.52
    return (math.log2(c) + 9.72) / 17.52


G24, G22 = power(2.4), power(2.2)
CURVES = [
    ("srgb", "srgb_rec709", "sRGB Encoded Rec.709 (sRGB)", "linear_rec709", srgb_decode, srgb_encode),
    ("bt1886", "bt1886_rec709", "Gamma 2.4 Encoded Rec.709", "linear_rec709", G24[0], G24[1]),
    ("gamma22", "gamma22_rec709", "Gamma 2.2 Encoded Rec.709", "linear_rec709", G22[0], G22[1]),
    ("acescct", "acescct", "ACEScct", "acescg", acescct_decode, acescct_encode),
    ("acescc", "acescc", "ACEScc", "acescg", acescc_decode, acescc_encode),
]


def f32(x):
    return struct.unpack("<f", struct.pack("<f", x))[0]


def lcg(seed):
    while True:
        seed = (seed * 6364136223846793005 + 1442695040888963407) % 2**64
        yield (seed >> 11) / 2**53


def samples(lo, hi, extra):
    """A 17^3 cube over 0..1, a grey ramp and one ramp per channel over lo..hi, then 100 fixed
    pseudo-random colours over lo..hi and the `extra` greys."""
    out = []
    for r in range(17):
        for g in range(17):
            for b in range(17):
                out.append([r / 16, g / 16, b / 16])
    out += [[lo + (hi - lo) * i / 64] * 3 for i in range(65)]
    for ch in range(3):
        for i in range(33):
            v = [0.18] * 3
            v[ch] = lo + (hi - lo) * i / 32
            out.append(v)
    rnd = lcg(906)
    out += [[lo + (hi - lo) * next(rnd) for _ in range(3)] for _ in range(100)]
    out += [[x] * 3 for x in extra]
    return [[f32(c) for c in s] for s in out]


# Linear light: past 1 and below 0, greys from 2^-14 to 64. Encoded values: -0.5 to 2, which
# keeps every log curve's answer inside f32.
LINEAR_SAMPLES = samples(-1.0, 64.0, [2.0**k for k in range(-14, 7)] + [-0.5, -0.01])
ENCODED_SAMPLES = samples(-0.5, 2.0, [])


def g9(x):
    return float("%.9g" % x)  # round-trips an f32 exactly


def run(src, dst, pts):
    cpu = config.getProcessor(src, dst).getOptimizedCPUProcessor(OPTIMIZATION)
    flat = [c for p in pts for c in p]
    return [g9(v) for v in cpu.applyRGB(flat)]


assert ocio.__version__ == VERSION, f"PyOpenColorIO {ocio.__version__}, pinned {VERSION}"
config = ocio.Config.CreateFromFile(CONFIG)

# OCIO's own matrix from each linear space to ACES2065-1, composed by OCIO in double precision.
# The build computes its matrices from chromaticities; these show which white adaptation the
# config's matrices were made with (the build matches Bradford, not CAT02, to 1e-15).
matrices = {}
for key, name in LINEAR.items():
    group = config.getProcessor(name, "ACES2065-1").createGroupTransform()
    ops = list(group)
    if key == "aces2065_1":
        assert len(ops) == 0, ops
        m = [1, 0, 0, 0, 1, 0, 0, 0, 1]
    else:
        assert len(ops) == 1 and isinstance(ops[0], ocio.MatrixTransform), ops
        m44 = ops[0].getMatrix()
        assert ops[0].getOffset() == [0, 0, 0, 0]
        m = [m44[r * 4 + c] for r in range(3) for c in range(3)]
    matrices[key] = [float("%.17g" % v) for v in m]

pairs = []
for a in LINEAR:
    for b in LINEAR:
        if a != b:
            pairs.append({"from": a, "to": b, "samples": "linear",
                          "ocio": run(LINEAR[a], LINEAR[b], LINEAR_SAMPLES)})
for curve, enc, ocio_name, lin, decode, encode in CURVES:
    pairs.append({"from": enc, "to": lin, "curve": curve, "samples": "encoded",
                  "ocio": run(ocio_name, LINEAR[lin], ENCODED_SAMPLES),
                  "formula": [float("%.12g" % decode(c)) for p in ENCODED_SAMPLES for c in p]})
    pairs.append({"from": lin, "to": enc, "curve": curve, "samples": "linear",
                  "ocio": run(LINEAR[lin], ocio_name, LINEAR_SAMPLES),
                  "formula": [float("%.12g" % encode(c)) for p in LINEAR_SAMPLES for c in p]})

doc = {
    "about": "D-906 / B-785: OpenColorIO's numbers for the build's named colour spaces. "
             "Written by tools/ocio_reference.py; never edit by hand.",
    "ocio_version": VERSION,
    "config": CONFIG,
    "optimization": "OPTIMIZATION_LOSSLESS",
    "white_adaptation": "Bradford",
    "bar": "|ours - ocio| <= 1e-6 * scale, scale the largest magnitude in the sample's input "
           "and OCIO's answer (per channel for a curve, the whole colour for a matrix)",
    "spaces": {**LINEAR, **{c[1]: c[2] for c in CURVES}},
    "matrices_to_aces2065_1": matrices,
    "samples": {"linear": LINEAR_SAMPLES, "encoded": ENCODED_SAMPLES},
    "pairs": pairs,
}
os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, "w", encoding="utf-8", newline="\n") as f:
    f.write("{\n")
    keys = list(doc)
    for i, k in enumerate(keys):
        end = ",\n" if i < len(keys) - 1 else "\n"
        if k == "pairs":
            f.write('"pairs": [\n')
            f.write(",\n".join(json.dumps(p, separators=(",", ":")) for p in pairs))
            f.write("\n]" + end)
        else:
            f.write(json.dumps(k) + ": " + json.dumps(doc[k], separators=(",", ":")) + end)
    f.write("}\n")
print(f"wrote {OUT}: {len(pairs)} pairs, {len(LINEAR_SAMPLES)} linear and "
      f"{len(ENCODED_SAMPLES)} encoded samples")
