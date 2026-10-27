"""Quality Fast on the big blurs of Glow, Bloom, Deband and Halation, worked a second way.

D-914 (wave 11, pick 9, step 4): a Quality setting on Glow, Bloom, Deband and Halation, "exact"
(as they have always been drawn, and the default, never saved) or "fast", which the user chooses
and which exports draw too. Fast blurs a reduced copy of the picture and reads it back, as Nuke's
Blur does at a quality below full. Document 21 is the rule in words; this file is the reference
for the numbers document 25 pins against it (FX-GLOW-034 to 037, FX-BLOOM-029 to 032,
FX-DEBAND-025 to 028, FX-HALO-019 to 022). Everything else of the four effects is their own
reference's, which this file imports and does not change.

**The cell** of a blur at sigma: the smallest power of two d with 8 d above sigma, so the reduced
blur runs at 4 to 8 cells a sigma; below sigma 8, d is 1 and Fast is the exact blur itself. This
is Soft Physical Glow's and Halation's own rule (D-353), which they already use at Exact.

**The reduced blur** of a picture at sigma, cell d above 1: the picture is cut into cells d by d
from its top left corner, each cell holding its pixels' sum over d * d (nothing outside the
picture). The cells are blurred by document 21's Gaussian at sigma_l = sqrt(max(sigma^2 - d^2 / 4,
0)) / d (the cell's average and the read back's tent spread it by about d^2 / 4 between them),
cut at its own reach, nothing outside the cells. Each full pixel (x, y) reads the blurred cells
bilinearly at u = (x + 0.5) / d - 0.5, v = (y + 0.5) / d - 0.5, nothing outside. The full blur's
reach is kept, so the picture grows as at Exact.

**Glow**: the light's blur is the reduced blur at Glow's sigma (both dimensions only; one
dimension stays exact). **Bloom**: each of its four halo blurs at its own sigma and cell; the
streaks are not blurs and stay as they are. **Deband**: its edge-stopping blur reads a tap every d
pixels, tap m weighted by the Gaussian at sigma / d at m, through the product of (1 - E) over
every pixel strictly between, so an edge stops it exactly where it stops Exact. The reduced blur
was tried first and let the colour across an edge bleed up to 30 levels into FX-DEBAND-027's
drawing (cells 4 by 4): a cell holds both sides of an edge. **Halation**:
its levels one cell coarser, the cell of 2 sigma (2 to 4 cells a sigma), from sigma 4.

**This file never runs the build's code path**: Glow's and Bloom's reduced blurs are summed in two
dimensions at once in double precision where the build blurs across and then down in single
precision.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/fast_blur_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import value_at  # noqa: E402
import glow_reference as G  # noqa: E402
import bloom_reference as B  # noqa: E402
import deband_reference as DB  # noqa: E402
import halation_reference as HA  # noqa: E402
import soft_glow_reference as SG  # noqa: E402

TOLERANCE = 2e-5  # document 25's default for a filter


# --- the rule -------------------------------------------------------------------------------

def cell(sigma):
    d = 1
    while 8 * d <= sigma:
        d *= 2
    return d


def low_sigma(sigma, d):
    return math.sqrt(max(sigma * sigma - d * d / 4, 0.0)) / d


def taps(sigma):
    r = math.ceil(3 * sigma) if sigma > 0 else 0
    if not r:
        return 0, [1.0]
    one = [math.exp(-(i * i) / (2 * sigma * sigma)) for i in range(-r, r + 1)]
    t = sum(one)
    return r, [v / t for v in one]


def cells(plane, w, h, d):
    """Each d by d cell's sum over d * d, cells from the top left."""
    lw, lh = -(-w // d), -(-h // d)
    low = []
    for j in range(lh):
        for i in range(lw):
            s = [0.0] * 4
            for y in range(j * d, min((j + 1) * d, h)):
                for x in range(i * d, min((i + 1) * d, w)):
                    s = [a + b for a, b in zip(s, plane[y * w + x])]
            low.append([v / (d * d) for v in s])
    return low, lw, lh


def read_back(at, lw, lh, d):
    """The full pixel (x, y)'s bilinear read of the cells' `at(i, j)`, nothing outside."""
    def sample(x, y):
        u, v = (x + 0.5) / d - 0.5, (y + 0.5) / d - 0.5
        u0, v0 = math.floor(u), math.floor(v)
        fu, fv = u - u0, v - v0
        out = [0.0] * 4
        for dj, wj in ((0, 1 - fv), (1, fv)):
            for di, wi in ((0, 1 - fu), (1, fu)):
                i, j = u0 + di, v0 + dj
                if 0 <= i < lw and 0 <= j < lh and wi * wj:
                    out = [o + wi * wj * p for o, p in zip(out, at(i, j))]
        return out
    return sample


def reduced(plane, w, h, sigma, d):
    """The reduced blur of `plane` (w * h pixels of four numbers), as a function of a full
    pixel (x, y), any whole numbers."""
    low, lw, lh = cells(plane, w, h, d)
    r, k = taps(low_sigma(sigma, d))
    cache = {}

    def blurred(i0, j0):
        if (i0, j0) not in cache:
            out = [0.0] * 4
            for j in range(j0 - r, j0 + r + 1):
                for i in range(i0 - r, i0 + r + 1):
                    if 0 <= i < lw and 0 <= j < lh:
                        wt = k[i - i0 + r] * k[j - j0 + r]
                        out = [o + wt * p for o, p in zip(out, low[j * lw + i])]
            cache[(i0, j0)] = out
        return cache[(i0, j0)]
    return read_back(blurred, lw, lh, d)


# --- Glow -----------------------------------------------------------------------------------

def glow(c, frame_no):
    pixels = [p for row in G.DRAWINGS[c["drawing"]] for p in row]
    radius = min(G.MAX_RADIUS, max(0.0, value_at(c["radius"], frame_no)))
    sigma = radius / 3
    d = cell(sigma)
    if d == 1:
        return G.glow(pixels, c, frame_no, c["shift"])
    threshold = min(G.MAX_THRESHOLD, max(0.0, value_at(c["threshold"], frame_no)))
    intensity = min(G.MAX_INTENSITY, max(0.0, value_at(c["intensity"], frame_no)))
    tolerance = min(G.MAX_TOLERANCE, max(0.0, value_at(c["tolerance"], frame_no)))
    source = []
    for p in pixels:
        if not G.glows(p, c, threshold, tolerance):
            source.append([0.0] * 4)
        elif c["tint"]:
            a = p[3] / 255
            source.append([G.srgb_to_linear(v / 255) * a for v in G.hex_color(c["tint"].lower())]
                          + [a])
        else:
            source.append(G.working(p))
    spread = reduced(source, G.W, G.H, sigma, d)
    out = []
    for y in range(G.H):
        for x in range(G.W):
            dx = x - c["shift"]
            g = [v * intensity for v in spread(dx, y)]
            o = G.working(pixels[y * G.W + dx]) if 0 <= dx < G.W else [0.0] * 4
            if c["operation"] == "add":
                out.append([o[ch] + g[ch] for ch in range(3)] + [min(1.0, o[3] + g[3])])
            else:
                g = [min(1.0, max(0.0, v)) for v in g]
                out.append([o[ch] + g[ch] - o[ch] * g[ch] for ch in range(4)])
    return out


# --- Bloom ----------------------------------------------------------------------------------

def bloom(c, frame_no):
    pixels = [p for row in B.DRAWINGS[c["drawing"]] for p in row]
    W, H, shift = B.W, B.H, c["shift"]
    threshold = min(B.MAX_THRESHOLD, max(0.0, value_at(c["threshold"], frame_no)))
    radius = min(B.MAX_RADIUS, max(0.0, value_at(c["radius"], frame_no)))
    intensity = min(B.MAX_INTENSITY, max(0.0, value_at(c["intensity"], frame_no)))
    length = min(B.MAX_LENGTH, max(0.0, value_at(c["length"], frame_no)))
    angle = min(B.MAX_ANGLE, max(-B.MAX_ANGLE, value_at(c["angle"], frame_no)))
    lines = B.LINES[c["streaks"]]
    bright = {"based_on": "bright"}
    light = [G.working(p) if G.glows(p, bright, threshold, 0) else [0.0] * 4 for p in pixels]
    lit = [(i % W, i // W) for i in range(W * H) if light[i][3] > 0]
    blurs = []
    for s in B.SCALES:
        sigma = radius / 3 * s
        d = cell(sigma)
        blurs.append(("exact", B.gaussian(sigma)) if d == 1
                     else ("reduced", reduced(light, W, H, sigma, d)))
    dirs = [B.along(angle + j * 180 / lines) for j in range(lines)]
    out = []
    for y in range(H):
        for x in range(W):
            dx = x - shift
            halo = [0.0] * 4
            for kind, b in blurs:
                if kind == "reduced":
                    halo = [h + v / len(B.SCALES) for h, v in zip(halo, b(dx, y))]
                    continue
                reach, one = b
                for lx, ly in lit:
                    i, j = lx - dx, ly - y
                    if abs(i) <= reach and abs(j) <= reach:
                        k = one[i + reach] * one[j + reach] / len(B.SCALES)
                        for ch in range(4):
                            halo[ch] += light[ly * W + lx][ch] * k
            streak = [0.0] * 4
            for u in dirs:
                line = B.streak_line(light, u, length, dx, y)
                for ch in range(4):
                    streak[ch] += line[ch] / lines
            g = [intensity * (halo[ch] + streak[ch]) for ch in range(4)]
            o = G.working(pixels[y * W + dx]) if 0 <= dx < W else [0.0] * 4
            out.append([o[ch] + g[ch] for ch in range(3)] + [min(1.0, o[3] + g[3])])
    return out


# --- Deband: its own reference with the blur's gathers taken every d pixels ---------------

EXACT_WEIGHTS, EXACT_GATHER = DB.weights, DB.gather
SIGMA = [0.0]


def weights(sigma):
    SIGMA[0] = sigma
    return EXACT_WEIGHTS(sigma)


def gather(values, E, k, axis):
    """`deband_reference.gather` with its taps every d pixels: tap m at m d, weighted by the
    Gaussian at sigma / d at m, and the product of (1 - E) over every pixel strictly between."""
    sigma = SIGMA[0]
    d = cell(sigma)
    if d == 1:
        return EXACT_GATHER(values, E, k, axis)
    W, H = DB.W, DB.H
    ks = EXACT_WEIGHTS(sigma / d)
    r = len(ks) // 2
    out = []
    for i in range(W * H):
        x, y = i % W, i // W
        at = (lambda q: (x + q, y)) if axis == 0 else (lambda q: (x, y + q))
        total = [ks[r] * v for v in values[i]]
        for sign in (-1, 1):
            for m in range(1, r + 1):
                tx, ty = at(sign * m * d)
                if not (0 <= tx < W and 0 <= ty < H):
                    break
                through = 1.0
                for q in range(1, m * d):
                    px, py = at(sign * q)
                    through *= 1 - E[py * W + px]
                total = [t + ks[r + m] * through * v for t, v in zip(total, values[ty * W + tx])]
        out.append(total)
    return out


def deband(c, frame_no):
    DB.weights, DB.gather = weights, gather
    try:
        return DB.render(c, frame_no)
    finally:
        DB.weights, DB.gather = EXACT_WEIGHTS, EXACT_GATHER


# --- Halation: its levels one cell coarser --------------------------------------------------

EXACT_DOWN = SG.down_factor


def halation(c, frame_no):
    SG.down_factor = lambda sigma: EXACT_DOWN(2 * sigma)
    try:
        return HA.render(c, frame_no)
    finally:
        SG.down_factor = EXACT_DOWN


# --- the cases ------------------------------------------------------------------------------

EFFECTS = {
    # name: (reference module, expected file, rendered at Fast, rendered at Exact, the plain
    # drawing for an invalid case)
    "glow": (G, "expected_glow_fast.json", glow, G.render,
             lambda c: G.render(G.case(c["drawing"], intensity=0), 0)),
    "bloom": (B, "expected_bloom_fast.json", bloom, B.render,
              lambda c: B.render(B.case(c["drawing"], intensity=0), 0)),
    "deband": (DB, "expected_deband_fast.json", deband, DB.render,
               lambda c: DB.R.frame(DB.pixels(c["drawing"]), 0)),
    "halation": (HA, "expected_halation_fast.json", halation, HA.render,
                 lambda c: SG.plain(c["drawing"])),
}

CASES = {
    "glow": {
        "FX-GLOW-034": ("FX-GLOW-001 at radius 21 (sigma 7) at Quality Fast: below sigma 8 Fast "
                        "is the exact blur, so the same frame as at Exact.",
                        G.case(radius=21), [0], "same"),
        "FX-GLOW-035": ("FX-GLOW-001 at radius 30 (sigma 10) at Quality Fast: the light blurred "
                        "on cells 2 by 2 at sigma 4.97 and read back smoothly, close to Exact "
                        "and not the same.", G.case(radius=30), [0], "close"),
        "FX-GLOW-036": ("Radius 60 (sigma 20), Screen, at Quality Fast: cells 4 by 4.",
                        G.case(radius=60, operation="screen"), [0], "close"),
    },
    "bloom": {
        "FX-BLOOM-029": ("FX-BLOOM-010 (radius 20, the widest blur sigma 6.67) at Quality Fast: "
                         "every blur below sigma 8, so the same frame as at Exact.",
                         B.case(radius=20, length=60), [0], "same"),
        "FX-BLOOM-030": ("Radius 30 at Quality Fast: the widest blur (sigma 10) on cells 2 by 2, "
                         "the three narrower exact.", B.case(radius=30), [0], "close"),
        "FX-BLOOM-031": ("Radius 100 with a star of streaks, length 60, at Quality Fast: the "
                         "four blurs on cells 8, 4, 2 and 1 wide; the streaks as at Exact.",
                         B.case(radius=100, length=60, streaks="star"), [0], "close"),
    },
    "deband": {
        "FX-DEBAND-025": ("Blur 16 alone (sigma 5.33) at Quality Fast: below sigma 8, the same "
                          "frame as at Exact.", DB.only(blur_amount=16), (0,), "same"),
        "FX-DEBAND-026": ("Blur 30 alone (sigma 10) at Quality Fast: the edge-stopping blur "
                          "reading every second pixel; the edge to the dark block kept as at "
                          "Exact.", DB.only(blur_amount=30), (0,), "close"),
        "FX-DEBAND-027": ("The settings as added with Blur 60 (sigma 20) at Quality Fast: every "
                          "fourth pixel; frame 1 has fresh dither.", DB.case(blur_amount=60), (0, 1),
                          "close"),
    },
    "halation": {
        "FX-HALO-019": ("Radius 6 (the widest level sigma 2) at Quality Fast: the cell of 2 "
                        "sigma is 1 for every level, the same frame as at Exact.",
                        HA.case(radius=6), [0], "same"),
        "FX-HALO-020": ("FX-HALO-001 (as added, radius 12) at Quality Fast: the widest level "
                        "(sigma 4) on cells 2 by 2 where Exact has 1, close and not the same.",
                        HA.case(), [0], "close"),
        "FX-HALO-021": ("FX-HALO-004 (radius 100) at Quality Fast: every level of sigma 4 and "
                        "up one cell coarser.", HA.case(radius=100), [0], "close"),
    },
}

INVALID = {
    "glow": ("FX-GLOW-037", G.case(radius=30)),
    "bloom": ("FX-BLOOM-032", B.case(radius=30)),
    "deband": ("FX-DEBAND-028", DB.case(blur_amount=30)),
    "halation": ("FX-HALO-022", HA.case()),
}


# --- the project files ----------------------------------------------------------------------

def write(module, fx, c, quality):
    name = module.write(fx, c)
    path = module.OUT / name
    p = json.loads(path.read_text(encoding="utf-8"))
    p["compositions"][0]["layers"][0]["effects"][0]["parameters"]["quality"] = quality
    with open(path, "w", encoding="utf-8", newline="") as f:
        f.write(json.dumps(p, indent=2) + "\n")
    return name


def main():
    results = {}
    for effect, (module, file, fast, exact, plain) in EFFECTS.items():
        expected = {"tolerance": TOLERANCE, "width": module.W, "height": module.H, "cases": {}}
        for fx, (says, c, frames, kind) in CASES[effect].items():
            rendered = {str(f): [[float(v) for v in p] for p in fast(c, f)] for f in frames}
            expected["cases"][fx] = {"says": says, "project": write(module, fx, c, "fast"),
                                     "frames": rendered}
            worst = max(abs(a - b) for f in frames for p, q in zip(rendered[str(f)], exact(c, f))
                        for a, b in zip(p, q))
            print(f"{fx}: {kind}, largest difference from Exact {worst:.6f}")
            results[fx] = (kind, worst)
        fx, c = INVALID[effect]
        says = ("Quality written \"medium\". The file is read, the effect is kept as written and "
                "left out of every frame, with a warning.")
        before = [[float(v) for v in p] for p in plain(c)]
        expected["cases"][fx] = {"says": says, "project": write(module, fx, c, "medium"),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
        with open(module.OUT / file, "w", encoding="utf-8", newline="") as f:
            f.write(json.dumps(expected, indent=1) + "\n")
        check(expected)
    for fx, (kind, worst) in results.items():
        if kind == "same":
            assert worst == 0.0, fx
        else:
            assert 1e-5 < worst < 0.05, (fx, worst)


def check(expected):
    """Every value finite and the covering inside 0 to 1."""
    for fx, v in expected["cases"].items():
        for px in v["frames"].values():
            for p in px:
                assert all(math.isfinite(x) for x in p), fx
                assert -1e-12 <= p[3] <= 1 + 1e-12, fx


if __name__ == "__main__":
    main()
