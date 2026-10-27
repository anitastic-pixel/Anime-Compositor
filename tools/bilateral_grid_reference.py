"""Bilateral Blur's and Cartoon's Fast quality, worked a second way.

D-913 (wave 11, pick 9): a Quality setting on Bilateral Blur and on Cartoon, "exact" (as they
have always been drawn, and the default, never saved) or "fast", which the user chooses and which
exports draw too. Fast works the bilateral filter on a bilateral grid, the way its authors
published it for speed: Paris and Durand, "A Fast Approximation of the Bilateral Filter using a
Signal Processing Approach" (ECCV 2006), and Chen, Paris and Durand, "Real-time Edge-Aware Image
Processing with the Bilateral Grid" (SIGGRAPH 2007). Written from the papers; no code is ported.
Document 21 is the rule in words; this file is the reference for the numbers document 25 pins
against it (FX-BILAT-018 to 023, FX-CARTOON-033 and 034). Everything else of the two effects is
`bilateral_blur_reference.py`'s and `cartoon_reference.py`'s, which this file imports and does not
change.

**The grid**, for one worked value (each of red, green and blue with Colorize on, the luminance
with it off), Radius r and Threshold t, on a layer w by h:

- The cells are ss = r / 2.2 pixels across and sr = 1.1 t levels deep. With the three steps
  below (a tent in, a 1 2 1 blur, a tent out) each spreads by 5/6 of a cell squared, so the
  grid's spread matches the exact rule's: its disc-cut distance bell (spread r / 2, cut at r)
  spreads 0.687 (r / 2)^2 along each axis, and its level bell t^2.
- e is the value's level (0 to 255, through the sRGB curve), and lo and hi the least and most e
  over the pixels that show. The grid is nx = floor((w - 1) / ss) + 2 cells across, ny =
  floor((h - 1) / ss) + 2 down and nz = floor((hi - lo) / sr) + 2 deep.
- Fast is the exact rule, unchanged, where ss is below 2 (r below 4.4: the disc is small and
  the exact rule is quick) or the grid would hold more than 4 w h cells (a Threshold so small
  against the layer's levels that the grid would outgrow the picture). Otherwise:
- In: each pixel that shows, at u = x / ss, v = y / ss, q = (e - lo) / sr, adds a v and a, a its
  covering and v its straight value, to the 8 cells round it, each by the product of
  (1 - the distance) along each axis (trilinear).
- Blur: both sums of every cell become (left + 2 middle + right) / 4 along x, then along y,
  then along z, a cell past the grid's end counting nothing.
- Out: each pixel that shows reads both sums at its own (u, v, q) the same trilinear way, and
  its new straight value is the first over the second. It is premultiplied again by its own
  covering; a pixel that does not show stays transparent, and the layer never grows.

So Fast is a smooth, never-sharper stand-in for the exact rule: the same smoothing of grain and
the same kept edges, to within the grid's cells.

**This file never runs the build's code path.** It works in double precision on lists.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/bilateral_grid_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import bilateral_blur_reference as B  # noqa: E402
import cartoon_reference as C  # noqa: E402

W, H = B.W, B.H
EXACT = B.bilateral_blur  # the exact rule, kept before the Cartoon cases swap it


def cells(w, h, radius, threshold, lo, hi):
    """The grid's cells (ss, sr, nx, ny, nz), or None where Fast is the exact rule."""
    ss, sr = radius / 2.2, 1.1 * threshold
    if ss < 2 or threshold <= 0:
        return None
    nx, ny = math.floor((w - 1) / ss) + 2, math.floor((h - 1) / ss) + 2
    nz = math.floor((hi - lo) / sr) + 2
    return None if nx * ny * nz > 4 * w * h else (ss, sr, nx, ny, nz)


def corners(t):
    i = math.floor(t)
    return ((i, 1 - (t - i)), (i + 1, t - i))


def blur(g, stride, count):
    """(left + 2 middle + right) / 4 along one axis of the flat grid g."""
    out = [0.0] * len(g)
    for k in range(len(g)):
        i = (k // stride) % count
        left = g[k - stride] if i > 0 else 0.0
        right = g[k + stride] if i < count - 1 else 0.0
        out[k] = (left + 2 * g[k] + right) / 4
    return out


def grid_channel(vals, alphas, es, w, h, grid):
    ss, sr, nx, ny, nz = grid
    lo = min(e for e, a in zip(es, alphas) if a > 0)
    at = lambda X, Y, Z: (Z * ny + Y) * nx + X  # noqa: E731
    num, den = [0.0] * (nx * ny * nz), [0.0] * (nx * ny * nz)
    for y in range(h):
        for x in range(w):
            i = y * w + x
            if alphas[i] <= 0:
                continue
            for X, fx in corners(x / ss):
                for Y, fy in corners(y / ss):
                    for Z, fz in corners((es[i] - lo) / sr):
                        k = at(X, Y, Z)
                        num[k] += fx * fy * fz * alphas[i] * vals[i]
                        den[k] += fx * fy * fz * alphas[i]
    for stride, count in ((1, nx), (nx, ny), (nx * ny, nz)):
        num, den = blur(num, stride, count), blur(den, stride, count)
    out = [None] * (w * h)
    for y in range(h):
        for x in range(w):
            i = y * w + x
            if alphas[i] <= 0:
                continue
            n = d = 0.0
            for X, fx in corners(x / ss):
                for Y, fy in corners(y / ss):
                    for Z, fz in corners((es[i] - lo) / sr):
                        n += fx * fy * fz * num[at(X, Y, Z)]
                        d += fx * fy * fz * den[at(X, Y, Z)]
            out[i] = n / d
    return out


def bilateral_fast(layer, radius, threshold, colorize):
    w, h = layer["w"], layer["h"]
    alphas = [p[3] for p in layer["px"]]
    straight = [None if p[3] <= 0 else [p[k] / p[3] for k in range(3)] for p in layer["px"]]
    chans = 3 if colorize == "on" else 1
    vals = [[0.0] * (w * h) for _ in range(chans)]
    for i, c in enumerate(straight):
        if c is not None:
            v = c if colorize == "on" else [sum(k * x for k, x in zip(B.LUMA, c))]
            for k in range(chans):
                vals[k][i] = v[k]
    if len(B.disc(radius)) == 1 or threshold == 0:
        return EXACT(layer, radius, threshold, colorize)
    worked = []
    for k in range(chans):
        es = [B.e(v) for v in vals[k]]
        shown = [e for e, a in zip(es, alphas) if a > 0]
        grid = cells(w, h, radius, threshold, min(shown), max(shown)) if shown else None
        if grid is None:
            return EXACT(layer, radius, threshold, colorize)
        worked.append(grid_channel(vals[k], alphas, es, w, h, grid))
    px = []
    for i, a in enumerate(alphas):
        if a <= 0:
            px.append(B.EMPTY)
            continue
        out = [worked[k][i] for k in range(chans)]
        if chans == 1:
            out = out * 3
        px.append([v * a for v in out] + [a])
    return dict(layer, px=px)


def uses_grid(layer, radius, threshold, colorize):
    """True where Fast works the grid rather than the exact rule, for the checks."""
    if len(B.disc(radius)) == 1 or threshold == 0:
        return False
    shown = [p for p in layer["px"] if p[3] > 0]
    for k in (range(3) if colorize == "on" else [None]):
        es = [B.e(p[k] / p[3] if k is not None else
                  sum(c * p[j] / p[3] for j, c in enumerate(B.LUMA))) for p in shown]
        if cells(layer["w"], layer["h"], radius, threshold, min(es), max(es)) is None:
            return False
    return True


# --- the cases ------------------------------------------------------------------------------

def bilat(radius=5, threshold=20, colorize="on", quality="fast"):
    c = B.case(radius, threshold, colorize)
    c["quality"] = quality
    return c


def render_bilat(c, frame_no):
    p = c["parameters"]
    layer = B.M.drawn_layer(c["drawing"])
    layer = bilateral_fast(layer, B.held(B.value_at(p["radius"], frame_no), B.RADIUS),
                           B.held(B.value_at(p["threshold"], frame_no), B.THRESHOLD),
                           p["colorize"])
    return B.frame(layer, c["shift"])


def render_cartoon(c, frame_no):
    C.bilateral_blur = bilateral_fast
    try:
        return C.render(c, frame_no)
    finally:
        C.bilateral_blur = EXACT


BILAT = {
    "FX-BILAT-018": ("FX-BILAT-001 (Radius 5, Threshold 20, Colorize on) at Quality Fast: the "
                     "grid's cells 2.27 pixels across and 22 levels deep; the grain is smoothed "
                     "and the line and specks kept, close to FX-BILAT-001 and not the same. The "
                     "hole stays a hole and the half-covered column keeps its covering.",
                     bilat()),
    "FX-BILAT-019": ("FX-BILAT-004 (Colorize off) at Quality Fast: the luminance worked on the "
                     "grid, every pixel that shows grey.", bilat(colorize="off")),
    "FX-BILAT-020": ("Radius 4 at Quality Fast: cells under 2 pixels, so the exact rule, the "
                     "same frame as Quality Exact.", bilat(radius=4)),
    "FX-BILAT-021": ("Radius 5, Threshold 1 at Quality Fast: the grid would hold more than "
                     "four cells for each pixel, so the exact rule, the same frame as Quality "
                     "Exact.", bilat(threshold=1)),
    "FX-BILAT-022": ("Radius 10, Threshold 60 at Quality Fast: cells 4.5 pixels across and 66 "
                     "levels deep, a wider smoothing.", bilat(radius=10, threshold=60)),
}
BILAT_INVALID = {
    "FX-BILAT-023": ("Bilateral Blur, quality written \"quick\".", bilat(quality="quick")),
}
CARTOON = {
    "FX-CARTOON-033": ("FX-CARTOON-001 (the settings as they start, Detail Radius 8, Threshold "
                       "10) at Quality Fast: the smoothing worked on the grid, cells 3.6 pixels "
                       "across and 11 levels deep; the rest of Cartoon as it was.",
                       dict(C.case(), quality="fast")),
}
CARTOON_INVALID = {
    "FX-CARTOON-034": ("Cartoon, quality written \"rough\".", dict(C.case(), quality="rough")),
}


def write_bilat(fx, c):
    j = B.project_json(fx, c)
    j["compositions"][0]["layers"][0]["effects"][0]["parameters"]["quality"] = c["quality"]
    name = f"{fx.lower().replace('-', '_')}.json"
    (B.OUT / name).write_text(json.dumps(j, indent=2) + "\n", encoding="utf-8")
    return name


def write_cartoon(fx, c):
    j = C.project_json(fx, c)
    j["compositions"][0]["layers"][0]["effects"][-1]["parameters"]["quality"] = c["quality"]
    name = f"{fx.lower().replace('-', '_')}.json"
    (C.OUT / name).write_text(json.dumps(j, indent=2) + "\n", encoding="utf-8")
    return name


def build(cases, invalid, write, render, plain, tolerance):
    expected = {"tolerance": tolerance, "width": W, "height": H, "cases": {}}
    for fx, (says, c) in cases.items():
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": render(c, 0)}}
        print(fx)
    for fx, (says, c) in invalid.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    return expected


def main():
    bilat_expected = build(BILAT, BILAT_INVALID, write_bilat, render_bilat, B.plain, B.TOLERANCE)
    (B.OUT / "expected_bilateral_blur_fast.json").write_text(
        json.dumps(bilat_expected, indent=1) + "\n", encoding="utf-8")
    cartoon_expected = build(CARTOON, CARTOON_INVALID, write_cartoon, render_cartoon, C.plain,
                             C.TOLERANCE)
    (C.OUT / "expected_cartoon_fast.json").write_text(
        json.dumps(cartoon_expected, indent=1) + "\n", encoding="utf-8")
    check(bilat_expected, cartoon_expected)


def check(bilat_expected, cartoon_expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    b = {fx: v["frames"]["0"] for fx, v in bilat_expected["cases"].items()}
    layer = B.M.drawn_layer("specks")
    exact = lambda r, t, c="on": B.frame(EXACT(layer, r, t, c), 0)  # noqa: E731
    drawn = B.plain(B.case())
    at = lambda x, y: y * W + x  # noqa: E731
    gap = lambda p, q: max(abs(u - v) for a, c in zip(p, q) for u, v in zip(a, c))  # noqa: E731

    assert uses_grid(layer, 5, 20, "on") and uses_grid(layer, 5, 20, "off")
    assert uses_grid(layer, 10, 60, "on") and uses_grid(layer, 8, 10, "on")
    assert not uses_grid(layer, 4, 20, "on") and not uses_grid(layer, 5, 1, "on")

    one, ex = b["FX-BILAT-018"], exact(5, 20)
    assert 1e-4 < gap(one, ex) < 0.2, gap(one, ex)
    assert one[at(12, 1)] == B.EMPTY
    assert all(one[i][3] == drawn[i][3] for i in range(W * H))
    grain = [(x, y) for x in range(8) for y in range(5, 9) if B.M.MAP[y][x] in "+-"]
    skin = B.R.working(B.M.PALETTE["s"])
    spread = lambda px: max(abs(px[at(x, y)][2] - skin[2]) for x, y in grain)  # noqa: E731
    assert spread(one) < spread(drawn) / 2
    lines = [(x, y) for x in (9, 10) for y in range(9)]
    assert all(one[at(x, y)][0] < 0.05 for x, y in lines)  # the line stays dark
    assert one[at(6, 2)][2] > skin[2] + 0.1  # the white speck stays light
    nineteen = b["FX-BILAT-019"]
    for i, p in enumerate(nineteen):
        assert p[3] == drawn[i][3] and p[0] == p[1] == p[2]
    assert 1e-4 < gap(nineteen, exact(5, 20, "off")) < 0.2
    assert b["FX-BILAT-020"] == exact(4, 20)
    assert b["FX-BILAT-021"] == exact(5, 1)
    assert gap(b["FX-BILAT-022"], exact(10, 60)) > 1e-4
    c = cartoon_expected["cases"]["FX-CARTOON-033"]["frames"]["0"]
    assert gap(c, C.render(C.case(), 0)) > 1e-4
    for frames in (b, {"c": c}):
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), p
    print("checked")


if __name__ == "__main__":
    main()
