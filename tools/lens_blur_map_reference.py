"""Lens blur with a blur map, worked a second way.

D-359 gives `core.lens_blur` (D-116, D-121, `tools/lens_blur_reference.py`) a blur map, as After
Effects' Camera Lens Blur has: another layer of the same composition says, pixel by pixel, how
far out of focus the holder is there. The map is D-189's: the named layer's picture fitted to the
holder's drawing, `tools/effect_layer_reference.py`. This file pins what is done with it.

At the holder's composition frame n, its drawing's own top-left pixel at (0, 0):

1. With `layer` "" the rule is D-121's, untouched: `tools/lens_blur_reference.py`. With a layer
   named that is not in the composition, the frame is drawn without the effect: the drawing,
   the layer not grown (D-189's `EFFECT_LAYER_MISSING`).
2. The map M is the named layer's picture fitted to the drawing's W by H by `fit`, `center`
   (After Effects' Center Map) or `stretch` (Stretch Map to Fit). Output pixel (x, y), in the
   drawing's own pixels, reads M at (x, y) held inside M's rectangle, so the ring the layer grows
   by reads M's outer pixels. v is, by `channel`: `luminance`, the sRGB encoding of
   0.2126 r + 0.7152 g + 0.0722 b of M's premultiplied pixel held within 0 and 1, as Compound
   Blur reads a map (transparent is 0); or `alpha`, its covering. With `invert` on, v is 1 - v.
3. d = |v - focal_distance / 255|: the map value `focal_distance` stays sharp, and the further a
   pixel's value is from it the more it is blurred, `radius` d at the most.
4. J = max(1, ceil(radius)) levels, level j's iris D-121's at radius radius (j / J) (the pixel
   alone below 1). s = d J, j = floor(s) held at most J, t = s - j. The output is
   A_j + t (A_(j+1) - A_j), where A_j is the plain mean, all four numbers, over level j's iris
   turned half round about the pixel of the drawing's pixels, each lit by D-121's highlight
   first, read with `edges` as D-121's: transparent outside the drawing, or its column and row
   held. When t is 0 A_(j+1) is not read. Then, with highlight gain above 0, each colour is held
   at or below the covering.
5. The layer grows as D-121's does at the full radius, whatever the map says: with transparent
   edges by the iris's reach rounded up on every side, with repeat not at all.

So a map whose v is 1 everywhere with focal distance 0 is plain D-121 at the full radius, and one
equal to the focal distance everywhere changes nothing but the layer's size.

`layer` D-189's word, "" when added; `fit` `center` when added, or `stretch`; `channel`
`luminance` when added, or `alpha`; `focal_distance` 0 to 255, keyable, 0 when added; `invert`
`off` or `on`. A file without them reads them at those values (D-121's way), so every
FX-LENS-001 to 044 file is the same numbers.

Depth: the After Effects way. A render's depth pass is put on a layer of its own through Pass
Extract (D-348), and that layer is the map; its effects are run for it (D-189). FX-LENS-059 does
so with the depth fixtures' EXR.

**This file never runs the build's code path.** Each mean is summed directly over the iris's
steps, in double precision from the drawings' 8-bit values, where the build sums running totals
in single precision buffers.

Every case is a composition 16 by 10, five frames, in `Fixtures/lens_blur/`: the drawing `holder`
at the top with the effect, and under it the map layers, every one switched off. Expected pixels
in `Fixtures/lens_blur/expected_lens_blur_map.json`. `expected_lens_blur.json` is not touched.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/lens_blur_map_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import smooth_reference as S  # noqa: E402
import directional_blur_reference as D  # noqa: E402
import edges_reference as E  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import shutil  # noqa: E402
from lens_blur_reference import IRIS_START, offsets, growth, lit  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "lens_blur"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 5
CLEAR = [0.0, 0.0, 0.0, 0.0]


# --- the pictures ---------------------------------------------------------------------------

def decode(rows):
    return L.pic(len(rows[0]), len(rows),
                 [[srgb_to_linear(r / 255) * a / 255, srgb_to_linear(g / 255) * a / 255,
                   srgb_to_linear(b / 255) * a / 255, a / 255] for row in rows for r, g, b, a in row])


HOLDERS = {"bars": D.DRAWINGS["bars"], "plate": E.DRAWINGS["plate"]}  # already in media/
DRAWINGS = {
    # Black at the left to white at the right, one step of 17 a column.
    "ramp": [[(17 * x, 17 * x, 17 * x, 255) for x in range(W)] for y in range(H)],
    # Black, fully covering at the left and clear at the right, 17 less a column.
    "fade": [[(0, 0, 0, 255 - 17 * x) for x in range(W)] for y in range(H)],
    # A 4 by 2 checker of white and black, white at its top left.
    "card": [[(255, 255, 255, 255) if (x + y) % 2 == 0 else (0, 0, 0, 255) for x in range(4)]
             for y in range(2)],
}
# The depth fixtures' scene, `tools/depth_channel_reference.py`'s `make_files`, copied byte for
# byte: 8 by 6, its Z 0.5 + 1.25 x + 0.75 y, every value exact in single precision.
DEPTH_FILE = "media/depth.exr"
DEPTH_FROM = OUT.parent / "depth_channel" / "media" / "scene.exr"
DW, DH = 8, 6
NEAR, FAR = 0.5, 13.0  # Pass Extract's Black and White Points on it: nearest black, furthest white


def depth_picture():
    """The map layer `depth`: the EXR's Z through Pass Extract, NEAR to FAR, clamped, opaque."""
    e = lambda z: min(1.0, max(0.0, (z - NEAR) / (FAR - NEAR)))  # noqa: E731
    return L.pic(DW, DH, [[e(0.5 + 1.25 * x + 0.75 * y)] * 3 + [1.0]
                          for y in range(DH) for x in range(DW)])


def map_picture(name):
    """What each map layer's picture is, before fitting."""
    if name == "white":
        return L.pic(W, H, [[1.0, 1.0, 1.0, 1.0] for _ in range(W * H)])
    if name == "depth":
        return depth_picture()
    return decode(DRAWINGS[name])


# --- the rule -------------------------------------------------------------------------------

def luma(p):
    return S.linear_to_srgb(min(1.0, max(0.0, 0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2])))


def mean(layer, radius, edges, x, y, iris, gain, threshold):
    """Step 4's A at the drawing's pixel (x, y): the plain mean over the iris of `radius`, turned
    half round, of the lit pixels, not yet held at white."""
    steps = offsets(radius, iris["iris"], iris["roundness"], iris["rotation"], iris["aspect"])
    total = [0.0] * 4
    w, h = layer["w"], layer["h"]
    for dx, dy in steps:
        sx, sy = x - dx, y - dy
        if edges == "repeat":
            sx, sy = min(max(sx, 0), w - 1), min(max(sy, 0), h - 1)
        elif not (0 <= sx < w and 0 <= sy < h):
            continue
        p = lit(layer["px"][sy * w + sx], gain, threshold)
        for i in range(4):
            total[i] += p[i]
    return [v / len(steps) for v in total]


def mapped(layer, m, s):
    """Steps 2 to 5: the output over the drawing grown by the full radius's growth, as a dict of
    its pixels by their place in the drawing's own pixels."""
    radius, edges = s["radius"], s["edges"]
    g = growth(radius, edges, s["aspect"])
    big = max(1, math.ceil(radius))
    focus = s["focal_distance"] / 255
    out = {}
    for y in range(-g, layer["h"] + g):
        for x in range(-g, layer["w"] + g):
            p = L.at(m, min(max(x, 0), m["w"] - 1), min(max(y, 0), m["h"] - 1))
            v = luma(p) if s["channel"] == "luminance" else p[3]
            if s["invert"] == "on":
                v = 1 - v
            sj = abs(v - focus) * big
            j = min(big, math.floor(sj))
            t = sj - j
            args = (edges, x, y, s, s["highlight_gain"], s["highlight_threshold"])
            a = mean(layer, radius * (j / big), *args)
            if t > 0:
                b = mean(layer, radius * ((j + 1) / big), *args)
                a = [a[i] + t * (b[i] - a[i]) for i in range(4)]
            if s["highlight_gain"] > 0:
                a = [min(v, a[3]) for v in a[:3]] + [a[3]]
            out[(x, y)] = a
    return out


# --- the cases ------------------------------------------------------------------------------

MAP_START = {"layer": "", "fit": "center", "channel": "luminance", "focal_distance": 0,
             "invert": "off"}
RANGES = {"radius": (0, 200), "focal_distance": (0, 255)}


def case(layer="ramp", radius=4, edges="transparent", shift=0, drawing="bars", iris=None, **m):
    """A case; every Blur Map setting is written to its file."""
    return {"drawing": drawing, "radius": radius, "edges": edges, "shift": shift,
            "iris": {**IRIS_START, **(iris or {})}, "map": {**MAP_START, "layer": layer, **m}}


def settings(c, n):
    s = {"radius": c["radius"], "edges": c["edges"], **c["iris"], **c["map"]}
    return {k: min(RANGES[k][1], max(RANGES[k][0], value_at(v, n))) if k in RANGES else v
            for k, v in s.items()}


def the_map(c):
    name = c["map"]["layer"]
    return L.fit(map_picture(name), c["map"]["fit"], W, H) if name in MAPS else None


MAPS = ("ramp", "fade", "card", "white", "depth")


def render(c, n):
    layer = decode(HOLDERS[c["drawing"]])
    s = settings(c, n)
    m = the_map(c)
    if m is None:  # step 1: "" is D-121's rule, which no case here asks for; a missing layer
        out = {(x, y): layer["px"][y * W + x] for y in range(H) for x in range(W)}
    else:
        out = mapped(layer, m, s)
    return [list(out.get((x - c["shift"], y), CLEAR)) for y in range(H) for x in range(W)]


def plain(c):
    layer = decode(HOLDERS[c["drawing"]])
    return [list(L.at(layer, x - c["shift"], y)) for y in range(H) for x in range(W)]


def lens007():
    """FX-LENS-007 as `tools/lens_blur_reference.py` pins it: radius 4, no map."""
    return json.loads((OUT / "expected_lens_blur.json").read_text())["cases"]["FX-LENS-007"]["frames"]["0"]


CASES = {
    "FX-LENS-045": ("Radius 4 with every Blur Map setting written at its start value: no layer, "
                    "Center, Luminance, Focal Distance 0, Invert off: FX-LENS-007 exactly.",
                    case(layer=""), (0,)),
    "FX-LENS-046": ("The ramp as the blur map, Focal Distance 0: black, near, at the left stays "
                    "sharp, and the blur grows column by column to the full radius 4 at the white "
                    "right edge.", case(), (0,)),
    "FX-LENS-047": ("The ramp, Focal Distance 255: the white right edge in focus, the left "
                    "blurred most.", case(focal_distance=255), (0,)),
    "FX-LENS-048": ("The ramp, Focal Distance 128: the middle in focus, both sides blurred about "
                    "half the radius.", case(focal_distance=128), (0,)),
    "FX-LENS-049": ("The ramp inverted, Focal Distance 0: FX-LENS-047.",
                    case(invert="on"), (0,)),
    "FX-LENS-050": ("A white solid read by its Alpha, Focal Distance 0: every pixel at the full "
                    "radius, FX-LENS-007.", case("white", channel="alpha"), (0,)),
    "FX-LENS-051": ("A black picture fading from covered at the left to clear at the right, read "
                    "by its Alpha: blurred most at the left, sharp at the right.",
                    case("fade", channel="alpha"), (0,)),
    "FX-LENS-052": ("The same picture read by its Luminance: black everywhere, so nothing is "
                    "blurred; the layer still grows, and the drawing is as it was.",
                    case("fade"), (0,)),
    "FX-LENS-053": ("A 4 by 2 checker as the map, Center: it covers columns 6 to 9 of rows 4 and 5; "
                    "only its four white pixels are blurred, everything else is sharp.",
                    case("card"), (0,)),
    "FX-LENS-054": ("The checker, Stretch: spread over the whole layer and softened between its "
                    "squares.", case("card", fit="stretch"), (0,)),
    "FX-LENS-055": ("FX-LENS-046 with edges repeat on the picture that fills the layer: the map "
                    "and the picture both held at the edges, every pixel still fully covered.",
                    case(edges="repeat", drawing="plate"), (0,)),
    "FX-LENS-056": ("Focal Distance keyed from 0 at frame 0 to 255 at frame 4: frame 0 is "
                    "FX-LENS-046 and frame 4 FX-LENS-047; the sharp band moves across.",
                    case(focal_distance=keyed((0, 0), (4, 255))), (0, 2, 4)),
    "FX-LENS-057": ("A layer that is not in the composition, `gone`: the frame is drawn without "
                    "the effect, with the warning every frame.", case("gone"), (0, 4)),
    "FX-LENS-058": ("FX-LENS-046 on the bars moved three pixels right: the same picture moved, "
                    "since the map lies on the layer.", case(shift=3), (0,)),
    "FX-LENS-059": ("Depth: the depth fixtures' EXR on a layer of its own with Pass Extract, its "
                    "depth from 0.5 (black) to 13 (white), stretched as the map, Focal Distance "
                    "0: the near top left in focus, the far bottom right blurred most.",
                    case("depth", fit="stretch"), (0,)),
    "FX-LENS-060": ("The ramp with a hexagon iris and highlights, gain 3 at threshold 80: the "
                    "skin lit and held at white after the levels are mixed.",
                    case(iris={"iris": "hexagon", "highlight_gain": 3,
                               "highlight_threshold": 80}), (0,)),
    "FX-LENS-061": ("The ramp at radius 0: the drawing, untouched.", case(radius=0), (0,)),
    "FX-LENS-062": ("The ramp at radius 2.5, Focal Distance 85: three levels, 0, 0.83 and 1.67 "
                    "of the radius, the sharp column the sixth.", case(radius=2.5,
                                                                     focal_distance=85), (0,)),
}

INVALID = {
    "FX-LENS-063": ("Focal Distance 256, above 255.", case(focal_distance=256)),
    "FX-LENS-064": ("Focal Distance -1, below 0.", case(focal_distance=-1)),
    "FX-LENS-065": ("Focal Distance keyed to 300 at frame 4.",
                    case(focal_distance=keyed((0, 0), (4, 300)))),
    "FX-LENS-066": ("Placement \"tile\", which Lens Blur's map does not offer.", case(fit="tile")),
    "FX-LENS-067": ("Channel \"red\", which is not a channel it reads.", case(channel="red")),
    "FX-LENS-068": ("Invert written \"yes\".", case(invert="yes")),
    "FX-LENS-069": ("A layer written as the number 3, not a word.", case(3)),
}


# --- the project files ----------------------------------------------------------------------

def effect(fid, c):
    params = {"radius": c["radius"], "edges": c["edges"], **c["iris"], **c["map"]}
    return {"instance_id": fid, "type_id": "core.lens_blur", "enabled": True,
            "parameters": {k: setting_json(v) for k, v in params.items()}}


def pass_extract():
    return {"instance_id": "fx-depth-0", "type_id": "core.pass_extract", "enabled": True,
            "parameters": {"pass": "depth", "black_point": NEAR, "white_point": FAR,
                           "invert": "off", "clamp": "on"}}


def map_layers():
    depth = L.raster("depth", "asset-depth", enabled=False, out_frame=FRAMES)
    depth["effects"] = [pass_extract()]
    return [L.raster("ramp", "asset-ramp", position=(3, 2), scale=(50, 50), enabled=False,
                     out_frame=FRAMES),
            L.raster("fade", "asset-fade", enabled=False, out_frame=FRAMES),
            L.raster("card", "asset-card", enabled=False, out_frame=FRAMES),
            L.solid("white", W, H, [1.0] * 3, enabled=False, out_frame=FRAMES),
            depth]


def project(pid, layers):
    assets = [L.still(n, f"media/{n}.png") for n in list(HOLDERS) + list(DRAWINGS)]
    assets.append({"id": "asset-depth", "kind": "still", "name": "depth", "path": DEPTH_FILE,
                   "interpretation": {"color_space": "linear-srgb", "alpha": "premultiplied"}})
    return {"schema_version": 0, "project_id": pid,
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": assets, "compositions": [L.composition("comp-main", W, H, FRAMES, layers)]}


def project_json(fx, c):
    holder = L.raster("holder", "asset-" + c["drawing"], position=(c["shift"], 0),
                      out_frame=FRAMES)
    holder["effects"] = [effect("fx-1", c)]
    return project("proj-" + fx.lower(), [holder] + map_layers())


def write(name, p):
    (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def cycle():
    a = L.raster("a", "asset-bars", out_frame=FRAMES)
    b = L.raster("b", "asset-bars", out_frame=FRAMES)
    a["effects"] = [effect("fx-a", case("b"))]
    b["effects"] = [effect("fx-b", case("a"))]
    return [a, b]


def main():
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))
    shutil.copyfile(DEPTH_FROM, OUT / DEPTH_FILE)

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}, "loads": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        if c["map"]["layer"] == "":
            rendered = {"0": lens007()}  # step 1: D-121's own numbers
        entry = {"says": says, "project": write(f"{fx.lower().replace('-', '_')}.json",
                                                project_json(fx, c)), "frames": rendered}
        if c["map"]["layer"] == "gone":
            entry["warning"] = "EFFECT_LAYER_MISSING"
        expected["cases"][fx] = entry
        before = plain(c)
        print(f"{fx}: " + ", ".join(f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} "
                                    "changed" for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(
            f"{fx.lower().replace('-', '_')}.json", project_json(fx, c)),
            "frames": {"0": before, "4": before}, "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    write("lens_map_cycle.json", project("proj-lens-map-cycle", cycle() + map_layers()))
    expected["loads"]["lens_map_cycle.json"] = {
        "says": "`a`'s Lens Blur reads `b` as its map and `b`'s reads `a`: refused, "
                "`EFFECT_LAYER_CYCLE`.", "refused": "EFFECT_LAYER_CYCLE"}

    (OUT / "expected_lens_blur_map.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    col = lambda px, x: [px[y * W + x] for y in range(H)]  # noqa: E731
    seven = lens007()
    assert c["FX-LENS-045"]["0"] == seven
    # The full radius everywhere is D-121's rule; the map's ramp is the levels between.
    assert near(c["FX-LENS-050"]["0"], seven, 1e-9)
    assert near(c["FX-LENS-049"]["0"], c["FX-LENS-047"]["0"])
    assert col(c["FX-LENS-046"]["0"], 0) == col(drawn, 0)  # black: in focus
    assert near(col(c["FX-LENS-046"]["0"], 15), col(seven, 15), 1e-9)  # white: radius 4
    assert near(col(c["FX-LENS-047"]["0"], 15), col(drawn, 15), 1e-9)
    assert c["FX-LENS-048"]["0"] not in (c["FX-LENS-046"]["0"], c["FX-LENS-047"]["0"])
    assert c["FX-LENS-052"]["0"] == drawn and c["FX-LENS-061"]["0"] == drawn
    assert c["FX-LENS-051"]["0"] != drawn and col(c["FX-LENS-051"]["0"], 15) == col(drawn, 15)
    for y in range(H):  # the centred checker: only its white pixels change
        for x in range(W):
            if (x, y) not in ((6, 4), (8, 4), (7, 5), (9, 5)):
                assert c["FX-LENS-053"]["0"][y * W + x] == drawn[y * W + x], (x, y)
    assert c["FX-LENS-053"]["0"] != c["FX-LENS-054"]["0"] != drawn
    assert all(abs(p[3] - 1) < 1e-12 for p in c["FX-LENS-055"]["0"])
    k = c["FX-LENS-056"]
    assert k["0"] == c["FX-LENS-046"]["0"] and near(k["4"], c["FX-LENS-047"]["0"])
    assert k["2"] not in (k["0"], k["4"])
    assert c["FX-LENS-057"]["0"] == drawn == c["FX-LENS-057"]["4"]
    moved, here = c["FX-LENS-058"]["0"], c["FX-LENS-046"]["0"]
    assert all(moved[y * W + x + 3] == here[y * W + x] for y in range(H) for x in range(W - 3))
    dp = depth_picture()
    assert dp["px"][0][0] == 0.0 and dp["px"][-1][0] == 1.0
    assert c["FX-LENS-059"]["0"] != drawn
    lit60 = c["FX-LENS-060"]["0"]
    assert any(abs(p[0] - p[3]) < 1e-12 and p[3] > 0 for p in lit60)
    assert c["FX-LENS-062"]["0"] not in (drawn, c["FX-LENS-046"]["0"])
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert -1e-12 <= p[3] <= 1 + 1e-12 and all(-1e-12 <= v <= p[3] + 1e-12
                                                          for v in p[:3]), (name, p)
    print("checks passed")


if __name__ == "__main__":
    main()
