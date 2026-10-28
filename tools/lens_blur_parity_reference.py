"""Lens blur's Diffraction Fringe and Highlight Saturation, worked a second way.

D-932 gives `core.lens_blur` (D-116, D-121, `tools/lens_blur_reference.py`; its blur map D-359,
`tools/lens_blur_map_reference.py`) two more of After Effects' Camera Lens Blur settings:

- `diffraction_fringe`, 0 to 100, keyable, 0 when added: the bright ring a real lens draws at the
  edge of each out-of-focus disc. The iris's steps are split in two: its inside, the steps the
  same iris at `radius` - 1 holds (the pixel alone below 1), and its rim, the rest. Each rim step
  counts 1 + `diffraction_fringe` / 25 times in the mean, each inside step once, and the mean is
  the weighted sum over the sum of the weights. So at 100 the rim counts five times, and at 0 the
  mean is D-121's. Under radius 1 the iris is the pixel alone, there is no rim, and nothing
  changes. With a blur map each level's iris is split the same way, at its own radius.
- `highlight_saturation`, 0 to 100, keyable, 0 when added: how much more colourful a highlight
  is. A pixel D-121's highlight lights (its brightest straight channel at or above the threshold,
  the gain above 0) has, after the gain, each colour c moved away from its luminance
  Y = 0.2126 r + 0.7152 g + 0.0722 b by (1 + `highlight_saturation` / 100), held at 0 or above:
  Y + (c - Y)(1 + s / 100). A pixel that is not lit, or a gain of 0, is untouched.

A file without them reads them at 0, so every FX-LENS-001 to 069 file is the same numbers. It is
this program's own method, after Adobe's words for the two settings; nothing is ported.

**This file never runs the build's code path.** Each mean is summed directly over the iris's
steps, in double precision from the drawings' 8-bit values.

The cases are FX-LENS-070 onward, compositions 16 by 10 in `Fixtures/lens_blur/` on the drawings
the other two files wrote there; expected pixels in `expected_lens_blur_parity.json`.
`expected_lens_blur.json` and `expected_lens_blur_map.json` are not touched.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/lens_blur_parity_reference.py
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at  # noqa: E402
import directional_blur_reference as D  # noqa: E402
import lens_blur_reference as R  # noqa: E402
import lens_blur_map_reference as M  # noqa: E402

OUT = R.OUT
W, H = R.W, R.H
TOLERANCE = 2e-5
FRINGE = (0, 100)
SATURATION = (0, 100)
NEW_START = {"diffraction_fringe": 0, "highlight_saturation": 0}


# --- the rule -------------------------------------------------------------------------------

def lit(p, gain, threshold, saturation):
    """D-121's highlight, then this file's saturation of what it lit."""
    q = R.lit(p, gain, threshold)
    if q is p or saturation == 0:
        return q
    y = 0.2126 * q[0] + 0.7152 * q[1] + 0.0722 * q[2]
    k = 1 + saturation / 100
    return [max(0.0, y + (v - y) * k) for v in q[:3]] + [q[3]]


def weights(radius, s):
    """Each step of the iris at `radius` with its weight: 1 inside, 1 + fringe / 25 on the rim."""
    shape = (s["iris"], s["roundness"], s["rotation"], s["aspect"])
    outer = R.offsets(radius, *shape)
    inner = set(R.offsets(radius - 1, *shape)) if radius >= 1 else set(outer)
    rim = 1 + s.get("diffraction_fringe", 0) / 25
    return [(d, 1.0 if d in inner else rim) for d in outer]


def mean(layer, radius, edges, x, y, s, gain, threshold):
    """The weighted mean over the iris turned half round about the drawing's pixel (x, y), of the
    lit pixels, not yet held at white. With the fringe at 0 it is D-121's plain mean."""
    total, n = [0.0] * 4, 0.0
    w, h = layer["w"], layer["h"]
    for (dx, dy), k in weights(radius, s):
        n += k
        sx, sy = x - dx, y - dy
        if edges == "repeat":
            sx, sy = min(max(sx, 0), w - 1), min(max(sy, 0), h - 1)
        elif not (0 <= sx < w and 0 <= sy < h):
            continue
        p = lit(layer["px"][sy * w + sx], gain, threshold, s.get("highlight_saturation", 0))
        for i in range(4):
            total[i] += k * p[i]
    return [v / n for v in total]


M.mean = mean  # the blur map's levels, each split at its own radius


def blurred(layer, x, y, s):
    """Without a map: the output at (x, y), nothing outside the layer grown by D-121's growth."""
    g = R.growth(s["radius"], s["edges"], s["aspect"])
    if not (-g <= x < layer["w"] + g and -g <= y < layer["h"] + g):
        return [0.0] * 4
    out = mean(layer, s["radius"], s["edges"], x, y, s, s["highlight_gain"],
               s["highlight_threshold"])
    if s["highlight_gain"]:
        out = [min(v, out[3]) for v in out[:3]] + [out[3]]
    return out


# --- the cases ------------------------------------------------------------------------------

RANGES = {**R.RANGES, "diffraction_fringe": FRINGE, "highlight_saturation": SATURATION}


def case(radius=4, edges="transparent", shift=0, drawing="bars", **iris):
    """A case without a map; every D-121 and D-932 setting written to its file."""
    assert set(iris) <= set(R.IRIS_START) | set(NEW_START)
    return {"drawing": drawing, "radius": radius, "edges": edges, "shift": shift,
            "iris": {**R.IRIS_START, **NEW_START, **iris}}


def map_case(layer="ramp", **kw):
    """A case with a blur map, as `tools/lens_blur_map_reference.py` writes one."""
    iris = {k: kw.pop(k) for k in list(kw) if k in R.IRIS_START or k in NEW_START}
    c = M.case(layer, iris={**NEW_START, **iris}, **kw)
    c["mapped"] = True
    return c


def settings(c, n):
    s = {"radius": c["radius"], "edges": c["edges"], **c["iris"], **c.get("map", {})}
    return {k: min(RANGES[k][1], max(RANGES[k][0], value_at(v, n))) if k in RANGES else v
            for k, v in s.items()}


def render(c, n):
    if c.get("mapped"):
        m = dict(c)
        m["iris"] = {k: value_at(v, n) for k, v in c["iris"].items()}
        s = settings(c, n)
        for k in NEW_START:
            m["iris"][k] = s[k]
        return M.render(m, n)
    layer = R.layer_of(c)
    s = settings(c, n)
    return [blurred(layer, x - c["shift"], y, s) for y in range(H) for x in range(W)]


def plain(c):
    return R.plain({"drawing": c["drawing"], "shift": c["shift"]})


CASES = {
    "FX-LENS-070": ("Radius 4 with Diffraction Fringe and Highlight Saturation written at 0: "
                    "FX-LENS-007 exactly.", case(), (0,)),
    "FX-LENS-071": ("Radius 4, Diffraction Fringe 50: the rim of the iris, the steps more than "
                    "three pixels out, counts three times, so the red pixel's patch is a ring, "
                    "brighter at its edge than in its middle.", case(diffraction_fringe=50), (0,)),
    "FX-LENS-072": ("Iris hexagon, radius 4, Diffraction Fringe 100: the hexagon's rim counts "
                    "five times.", case(iris="hexagon", diffraction_fringe=100), (0,)),
    "FX-LENS-073": ("Radius 4, highlight gain 3 at threshold 80, Highlight Saturation 100: the "
                    "lit skin twice as far from its grey before the average, so its haze is "
                    "pinker than FX-LENS-029's.", case(highlight_gain=3, highlight_threshold=80,
                                                       highlight_saturation=100), (0,)),
    "FX-LENS-074": ("Radius 4, highlight gain 0.5 at threshold 50, Highlight Saturation 50: the "
                    "red is lit and made redder, its green and blue held at 0.",
                    case(highlight_gain=0.5, highlight_threshold=50, highlight_saturation=50),
                    (0,)),
    "FX-LENS-075": ("Radius 4, Highlight Saturation 100 with gain 0: nothing is lit, so nothing "
                    "is saturated: FX-LENS-007.", case(highlight_saturation=100), (0,)),
    "FX-LENS-076": ("Radius 4, Diffraction Fringe keyed from 0 at frame 0 to 100 at frame 4: "
                    "frame 0 is FX-LENS-007, frame 2 the rim three times, frame 4 five.",
                    case(diffraction_fringe=keyed((0, 0), (4, 100))), (0, 2, 4)),
    "FX-LENS-077": ("Radius 0.9, Diffraction Fringe 100: the pixel alone has no rim, and the "
                    "drawing is untouched.", case(radius=0.9, diffraction_fringe=100), (0,)),
    "FX-LENS-078": ("Radius 2.5, edges repeat, Diffraction Fringe 100, on the picture that fills "
                    "the layer: every pixel stays fully covered.",
                    case(radius=2.5, edges="repeat", drawing="plate", diffraction_fringe=100),
                    (0,)),
    "FX-LENS-079": ("The ramp as the blur map, Focal Distance 0, Diffraction Fringe 50: each "
                    "level's iris has its own rim, and the white right edge is FX-LENS-071's.",
                    map_case(diffraction_fringe=50), (0,)),
    "FX-LENS-080": ("A white solid read by its Alpha as the blur map, Diffraction Fringe 50: "
                    "every pixel at the full radius, FX-LENS-071.",
                    map_case("white", channel="alpha", diffraction_fringe=50), (0,)),
}

INVALID = {
    "FX-LENS-081": ("Diffraction Fringe 101, above 100.", case(diffraction_fringe=101)),
    "FX-LENS-082": ("Diffraction Fringe -1, below 0.", case(diffraction_fringe=-1)),
    "FX-LENS-083": ("Highlight Saturation 101, above 100.", case(highlight_saturation=101)),
    "FX-LENS-084": ("Highlight Saturation -1, below 0.", case(highlight_saturation=-1)),
    "FX-LENS-085": ("Highlight Saturation keyed to 150 at frame 4.",
                    case(highlight_saturation=keyed((0, 0), (4, 150)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    if c.get("mapped"):
        return M.project_json(fx, {k: v for k, v in c.items() if k != "mapped"})
    return R.project_json(fx, c)


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_lens_blur_parity.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                        encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    old = json.loads((OUT / "expected_lens_blur.json").read_text())["cases"]
    seven, lit29 = old["FX-LENS-007"]["frames"]["0"], old["FX-LENS-029"]["frames"]["0"]
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    at = lambda x, y: y * W + x  # noqa: E731
    drawn = plain(case())
    red = D.working(D.SOFT)

    # The rim's size: the circle of 49 at radius 4 holds the 29 of radius 3, so 20 on its rim.
    rim = [k for _, k in weights(4, {**R.IRIS_START, "diffraction_fringe": 50}) if k > 1]
    assert len(rim) == 20 and rim[0] == 3
    assert all(k == 1 for _, k in weights(4, {**R.IRIS_START}))
    assert near(c["FX-LENS-070"]["0"], seven) and near(c["FX-LENS-075"]["0"], seven)
    # The red pixel at (12, 4): four above it, (12, 0), is on the rim; one above, (12, 3), inside.
    f71 = c["FX-LENS-071"]["0"]
    n = 29 + 3 * 20
    assert near([f71[at(12, 0)]], [[v * 3 / n for v in red]])
    assert f71[at(12, 0)][3] > seven[at(12, 0)][3]
    assert f71[at(14, 1)][3] > 2 * f71[at(14, 2)][3] > 0  # the ring, past the block's haze
    assert c["FX-LENS-072"]["0"] != f71
    # Saturation: redder than D-121's lit numbers, the covering the same.
    f73 = c["FX-LENS-073"]["0"]
    assert all(abs(p[3] - q[3]) < 1e-12 for p, q in zip(f73, lit29)) and f73 != lit29
    assert lit(red, 0.5, 50, 50)[1] == 0.0 or lit(red, 0.5, 50, 50)[1] < red[1]
    k76 = c["FX-LENS-076"]
    assert near(k76["0"], seven) and k76["2"] != k76["4"]
    assert near(k76["4"], render(case(diffraction_fringe=100), 0))
    assert c["FX-LENS-077"]["0"] == drawn
    assert all(abs(p[3] - 1) < 1e-12 for p in c["FX-LENS-078"]["0"])
    col = lambda px, x: [px[y * W + x] for y in range(H)]  # noqa: E731
    assert near(col(c["FX-LENS-079"]["0"], 15), col(f71, 15), 1e-9)
    assert near(c["FX-LENS-080"]["0"], f71, 1e-9)
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
