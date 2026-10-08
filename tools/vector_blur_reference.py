"""CC Vector Blur, worked a second way.

D-336 adds `core.vector_blur`, After Effects' CC Vector Blur, from P-26's tutorial 2: a layer
smeared along a field of directions read from a map. CycoreFX's manual says what each Type does
in a sentence and publishes no formula, so the rule below is this program's own reading of it.

At the holder's composition frame n, with its input O, linear and premultiplied, whose drawing's
own top-left pixel is at (ox, oy):

1. The map is D-189's layer setting, fitted to the drawing's own W by H and lying on it, clear
   outside it; with the name "" (or a layer not in the composition, with the warning) it is O
   itself. F, the height, is at each pixel the map's `property`, as Colorama reads a phase
   (`grade::phase_of`): red, green, blue or luminance, or HSL hue (a turn as 0 to 1), lightness or
   saturation of the straight encoded colour, all times the covering; or the covering itself for
   `alpha`.
2. B is F through document 21's Gaussian at sigma `map_softness / 2`, nothing outside the
   drawing, worked out past its edges as far as the kernel reaches; at softness 0, B is F and
   nothing outside. Its slope g is the central differences `(B(x+1, y) - B(x-1, y)) / 2` and
   `(B(x, y+1) - B(x, y-1)) / 2`, and G its length.
3. Each pixel gets a vector of length L along a unit direction u:
   - `natural`: where G is at least 1/10000, u is g / G turned `angle_offset` degrees clockwise,
     L = amount * 100 G / sqrt((100 G)^2 + ridge^2), `ridge_smoothness` softening the gentle
     slopes (at ridge 0, L = amount); the samples weigh 1 - |t|, fading.
   - `constant`: as natural, L = amount, every sample weighing 1.
   - `perpendicular`: natural turned a further 90 degrees, along the slopes, around hills.
   - `direction_center`: u is document 21's step at `angle_offset + 360 * ridge * B` degrees
     clockwise from up, `ridge_smoothness` counting the revolutions; L = amount everywhere;
     every sample weighing 1.
   - `direction_fading`: as direction_center, but only forward, the samples weighing 1 - t.
   Elsewhere (G under 1/10000 for the first three) there is no vector and the pixel is O.
4. With n = ceil(amount), the samples are at t = k / n, k from -n to n (from 0 to n for
   direction_fading): O read by document 21's bilinear sampler at the pixel's centre plus
   t L u, transparent outside. The output is their weighted sum over the weights' sum. Amount 0
   is O. The layer does not grow.
5. For a draft, `amount` and `map_softness` are distances.

`type` one of the five words, `natural` when added; `amount` 0 to 500, 10; `angle_offset` -3600
to 3600, 0; `ridge_smoothness` 0 to 100, 10; `layer` D-189's word, ""; `fit` `stretch`, `center`
or `tile`, `stretch`; `property` one of red, green, blue, alpha, luminance, lightness, hue,
saturation, `lightness`; `map_softness` 0 to 100, 30. Every number keyable. The values when added
are chosen here; the manual gives none.

**This file never runs the build's code path.** It sums the two-dimensional kernel directly where
the build runs two passes, in double precision from the drawings' 8-bit values.

Every case is a project of one composition 16 by 10, five frames, in `Fixtures/vector_blur/`:
the drawing `holder` at the top with the effect, and under it the map layers, every one switched
off. The expected pixels are in `Fixtures/vector_blur/expected_vector_blur.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/vector_blur_reference.py
"""

import json
import sys
from math import ceil, cos, exp, floor, radians, sin, sqrt
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from hue_saturation_reference import to_hsl  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import smooth_reference as S  # noqa: E402
import effect_layer_reference as L  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "vector_blur"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES = 16, 10, 5
FLAT = 1e-4
NEAREST_FLAT = [1.0]  # how near any slope came to FLAT, as a ratio; checked below

RED, BLUE, CREAM, NONE = (220, 40, 40, 255), (40, 80, 220, 255), (240, 240, 200, 255), (0, 0, 0, 0)


def holder_px(x, y):
    """Two-pixel squares of red, blue and cream, the bottom right corner 4 by 3 clear."""
    if x >= 12 and y >= 7:
        return NONE
    return (RED, BLUE, CREAM)[(x // 2 + y // 2) % 3]


def disc_px(x, y):
    """White inside 3.5 pixels of the middle, black around it."""
    return (255, 255, 255, 255) if (x + 0.5 - 8) ** 2 + (y + 0.5 - 5) ** 2 <= 3.5 ** 2 \
        else (0, 0, 0, 255)


DRAWINGS = {
    "holder": [[holder_px(x, y) for x in range(W)] for y in range(H)],
    # Black at the left to white at the right, one step of 17 a column.
    "ramp": [[(17 * x, 17 * x, 17 * x, 255) for x in range(W)] for y in range(H)],
    "disc": [[disc_px(x, y) for x in range(W)] for y in range(H)],
}


def decoded(name):
    rows = DRAWINGS[name]
    return L.pic(len(rows[0]), len(rows),
                 [[srgb_to_linear(r / 255) * a / 255, srgb_to_linear(g / 255) * a / 255,
                   srgb_to_linear(b / 255) * a / 255, a / 255] for row in rows for r, g, b, a in row])


MAPS = {"ramp": decoded("ramp"), "disc": decoded("disc"),
        "black": L.pic(W, H, [[0.0, 0.0, 0.0, 1.0] for _ in range(W * H)])}


# --- the rule -------------------------------------------------------------------------------

def phase(prop, p):
    """Step 1's height of one pixel."""
    a = p[3]
    if prop == "alpha":
        return a
    if a <= 0:
        return 0.0
    lin = [min(max(p[c] / a, 0.0), 1.0) for c in range(3)]
    c = [S.linear_to_srgb(v) for v in lin]
    h, s, l = to_hsl(*c)
    k = {"red": c[0], "green": c[1], "blue": c[2], "hue": h / 360, "saturation": s,
         "lightness": l,
         "luminance": S.linear_to_srgb(0.2126 * lin[0] + 0.7152 * lin[1] + 0.0722 * lin[2])}[prop]
    return k * a


def along(d):
    q = d % 360
    return {0: (0.0, -1.0), 90: (1.0, 0.0), 180: (0.0, 1.0), 270: (-1.0, 0.0)}.get(
        q, (sin(radians(d)), -cos(radians(d))))


def turned(u, d):
    """u turned d degrees clockwise on the screen, y down."""
    c, s = cos(radians(d)), sin(radians(d))
    return (u[0] * c - u[1] * s, u[0] * s + u[1] * c)


def vector_blur(o, m, kind, amount, angle, ridge, prop, softness):
    """Steps 1 to 4 on the picture o, with the map m already fitted to it."""
    if amount == 0:
        return o
    w, h = o["w"], o["h"]
    f = [phase(prop, p) for p in m["px"]]
    sigma = softness / 2
    r = ceil(3 * sigma) if sigma > 0 else 0
    if r:
        one = [exp(-(k * k) / (2 * sigma * sigma)) for k in range(-r, r + 1)]
        one = [v / sum(one) for v in one]

    def b(x, y):
        if r == 0:
            return f[y * w + x] if 0 <= x < w and 0 <= y < h else 0.0
        return sum(f[j * w + i] * one[i - x + r] * one[j - y + r]
                   for j in range(max(0, y - r), min(h, y + r + 1))
                   for i in range(max(0, x - r), min(w, x + r + 1)))

    n = ceil(amount)
    out = []
    for y in range(h):
        for x in range(w):
            if kind.startswith("direction"):
                u, size = along(angle + 360 * ridge * b(x, y)), amount
            else:
                gx, gy = (b(x + 1, y) - b(x - 1, y)) / 2, (b(x, y + 1) - b(x, y - 1)) / 2
                g = sqrt(gx * gx + gy * gy)
                if g > 0:
                    NEAREST_FLAT[0] = min(NEAREST_FLAT[0], abs(g / FLAT - 1))
                if g < FLAT:
                    out.append(L.at(o, x, y))
                    continue
                u = turned((gx / g, gy / g), angle + (90 if kind == "perpendicular" else 0))
                size = amount if kind == "constant" or ridge == 0 else \
                    amount * 100 * g / sqrt((100 * g) ** 2 + ridge * ridge)
            ks = range(0, n + 1) if kind == "direction_fading" else range(-n, n + 1)
            acc, total = [0.0] * 4, 0.0
            for k in ks:
                t = k / n
                wt = 1 - abs(t) if kind in ("natural", "perpendicular", "direction_fading") else 1.0
                if wt == 0:
                    continue
                s = L.bilinear(o, x + 0.5 + t * size * u[0], y + 0.5 + t * size * u[1])
                acc = [acc[c] + wt * s[c] for c in range(4)]
                total += wt
            out.append([v / total for v in acc])
    return L.pic(w, h, out)


# --- the cases ------------------------------------------------------------------------------

def case(kind="natural", amount=10, angle=0, ridge=10, layer="", fit="stretch",
         prop="lightness", softness=30, shift=(0, 0)):
    return {"type": kind, "amount": amount, "angle_offset": angle, "ridge_smoothness": ridge,
            "layer": layer, "fit": fit, "property": prop, "map_softness": softness,
            "shift": shift}


def the_map(c):
    if c["layer"] in MAPS:
        return L.fit(MAPS[c["layer"]], c["fit"], W, H)
    return decoded("holder")  # "" and a layer not there: the layer itself


def render(c, frame):
    o = decoded("holder")
    out = vector_blur(o, the_map(c), c["type"], value_at(c["amount"], frame),
                      value_at(c["angle_offset"], frame), value_at(c["ridge_smoothness"], frame),
                      c["property"], value_at(c["map_softness"], frame))
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = decoded("holder")
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


RAMP3 = dict(layer="ramp", amount=3, softness=0)
CASES = {
    "FX-VBLUR-001": ("As added: Natural, Amount 10, Ridge Smoothness 10, the layer's own "
                     "lightness as the map, Map Softness 30: a soft smear along the slopes of "
                     "its blurred brightness.", case(), (0,)),
    "FX-VBLUR-002": ("Amount 0: nothing moves.", case(amount=0), (0,)),
    "FX-VBLUR-003": ("The ramp as the map, Constant Length, Amount 3, Map Softness 0: its slope "
                     "runs left to right, so rows 1 to 8 are smeared straight across, 3 pixels "
                     "each way, evenly.", case("constant", **RAMP3), (0,)),
    "FX-VBLUR-004": ("The same as Natural, Ridge Smoothness 10: the ramp's gentle slope, 1/15 a "
                     "pixel, makes the smear 3 * 6.67 / sqrt(6.67^2 + 10^2), 1.66 pixels each "
                     "way, fading.", case("natural", **RAMP3), (0,)),
    "FX-VBLUR-005": ("The same as Perpendicular: along the slope's contour, so rows are left "
                     "alone and columns smeared up and down.", case("perpendicular", **RAMP3),
                     (0,)),
    "FX-VBLUR-006": ("Natural with Angle Offset 90: exactly Perpendicular, FX-VBLUR-005.",
                     case("natural", angle=90, **RAMP3), (0,)),
    "FX-VBLUR-007": ("A black solid as the map, Direction Center, Angle Offset 90, Amount 3: the "
                     "height is 0 everywhere, so every pixel is smeared 3 pixels each way "
                     "across, evenly; rows 1 to 8 are FX-VBLUR-003.",
                     case("direction_center", angle=90, layer="black", amount=3, softness=0),
                     (0,)),
    "FX-VBLUR-008": ("The same as Direction Fading: only forward, from the pixels to the right.",
                     case("direction_fading", angle=90, layer="black", amount=3, softness=0),
                     (0,)),
    "FX-VBLUR-009": ("The ramp with Direction Center, Revolutions 1, Amount 2: the direction "
                     "turns once round from black to white, straight up at the left.",
                     case("direction_center", ridge=1, layer="ramp", amount=2, softness=0),
                     (0,)),
    "FX-VBLUR-010": ("FX-VBLUR-004 with Map Softness 4: the ramp's edges soften, so the slope "
                     "bends near them.", case("natural", layer="ramp", amount=3, softness=4),
                     (0,)),
    "FX-VBLUR-011": ("A white disc as the map, Natural, Ridge Smoothness 0, Map Softness 0, "
                     "Amount 2: smeared in and out across its rim, and nowhere else.",
                     case("natural", ridge=0, layer="disc", amount=2, softness=0), (0,)),
    "FX-VBLUR-012": ("The disc as Perpendicular: smeared round its rim.",
                     case("perpendicular", ridge=0, layer="disc", amount=2, softness=0), (0,)),
    "FX-VBLUR-013": ("Property Alpha on the layer itself, Constant Length, Amount 2, Map "
                     "Softness 0: only the pixels on the drawing's edges and round its clear "
                     "corner have a slope.", case("constant", amount=2, prop="alpha", softness=0),
                     (0,)),
    "FX-VBLUR-014": ("Property Hue on the layer itself, Ridge Smoothness 5, Amount 3, Map "
                     "Softness 2.", case(amount=3, ridge=5, prop="hue", softness=2), (0,)),
    "FX-VBLUR-015": ("Property Saturation on the layer itself, Constant Length, Amount 2, Map "
                     "Softness 2.", case("constant", amount=2, prop="saturation", softness=2),
                     (0,)),
    "FX-VBLUR-016": ("FX-VBLUR-003 with Amount keyed from 0 at frame 0 to 6 at frame 4: frame 0 "
                     "untouched and frame 2 is FX-VBLUR-003.",
                     case("constant", layer="ramp", amount=keyed((0, 0), (4, 6)), softness=0),
                     (0, 2, 4)),
    "FX-VBLUR-017": ("A layer that is not in the composition, `gone`: the layer itself is the "
                     "map, as FX-VBLUR-001, and the warning every frame.", case(layer="gone"),
                     (0, 4)),
    "FX-VBLUR-018": ("FX-VBLUR-003 on the holder moved 2 right and 1 down: the same picture "
                     "moved, since the map lies on the layer.",
                     case("constant", shift=(2, 1), **RAMP3), (0,)),
}

INVALID = {
    "FX-VBLUR-019": ("A type written \"Natural\", with a capital.", case("Natural")),
    "FX-VBLUR-020": ("Amount 501, above 500.", case(amount=501)),
    "FX-VBLUR-021": ("Amount -1, below 0.", case(amount=-1)),
    "FX-VBLUR-022": ("Ridge Smoothness 101, above 100.", case(ridge=101)),
    "FX-VBLUR-023": ("Map Softness keyed to -1 at frame 4.", case(softness=keyed((0, 30), (4, -1)))),
    "FX-VBLUR-024": ("A property written \"brightness\".", case(prop="brightness")),
    "FX-VBLUR-025": ("A fit written \"fill\".", case(layer="ramp", fit="fill")),
    "FX-VBLUR-026": ("A layer written as the number 3, not a word.", case(layer=3)),
    "FX-VBLUR-027": ("Angle Offset 3601, past 3600.", case(angle=3601)),
}

NAMES = ("type", "amount", "angle_offset", "ridge_smoothness", "layer", "fit", "property",
         "map_softness")


def effect(fid, c):
    return {"instance_id": fid, "type_id": "core.vector_blur", "enabled": True,
            "parameters": {k: setting_json(c[k]) for k in NAMES}}


def map_layers():
    return [L.raster("ramp", "asset-ramp", position=(3, 2), scale=(50, 50), enabled=False,
                     out_frame=FRAMES),
            L.raster("disc", "asset-disc", enabled=False, out_frame=FRAMES),
            L.solid("black", W, H, [0.0] * 3, enabled=False, out_frame=FRAMES)]


def project_json(fx, c):
    holder = L.raster("holder", "asset-holder", position=c["shift"], out_frame=FRAMES)
    holder["effects"] = [effect("fx-1", c)]
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(n, f"media/{n}.png") for n in DRAWINGS],
            "compositions": [L.composition("comp-main", W, H, FRAMES, [holder] + map_layers())]}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        entry = {"says": says, "project": write(fx, c), "frames": rendered}
        if c["layer"] == "gone":
            entry["warning"] = "EFFECT_LAYER_MISSING"
        expected["cases"][fx] = entry
        before = plain(c)
        print(f"{fx}: " + ", ".join(f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} "
                                    "changed" for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_vector_blur.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    near = lambda a, b, e=1e-12: all(abs(p - q) < e for u, v in zip(a, b)  # noqa: E731
                                     for p, q in zip(u, v))
    rows = lambda px, ys: [px[y * W + x] for y in ys for x in range(W)]  # noqa: E731
    # No slope sits within a hundredth of the flat line, where single precision could tip it.
    assert NEAREST_FLAT[0] > 0.01, NEAREST_FLAT
    assert c["FX-VBLUR-002"]["0"] == drawn and c["FX-VBLUR-016"]["0"] == drawn
    for fx in ("FX-VBLUR-001", "FX-VBLUR-003", "FX-VBLUR-004", "FX-VBLUR-005", "FX-VBLUR-008",
               "FX-VBLUR-009", "FX-VBLUR-010", "FX-VBLUR-011", "FX-VBLUR-012", "FX-VBLUR-013",
               "FX-VBLUR-014", "FX-VBLUR-015"):
        assert c[fx]["0"] != drawn, fx
    # 003: rows 1 to 8 are an even three-pixel smear straight across.
    o = decoded("holder")
    for y in range(1, H - 1):
        for x in range(W):
            want = [sum(L.at(o, x + k, y)[ch] for k in range(-3, 4)) / 7 for ch in range(4)]
            assert near([c["FX-VBLUR-003"]["0"][y * W + x]], [want]), (x, y)
    assert near(c["FX-VBLUR-005"]["0"], c["FX-VBLUR-006"]["0"])
    assert near(rows(c["FX-VBLUR-007"]["0"], range(1, H - 1)),
                rows(c["FX-VBLUR-003"]["0"], range(1, H - 1)))
    # 005: in the ramp's middle columns nothing is taken from beside a pixel, only above and below.
    for y in range(1, H - 1):
        for x in range(1, W - 1):
            want = [sum((1 - abs(k) / 3) * L.bilinear(o, x + 0.5, y + 0.5 + k * 1.664 / 3)[ch]
                        for k in range(-2, 3)) for ch in range(4)]
            got = c["FX-VBLUR-005"]["0"][y * W + x]
            assert all(abs(got[ch] - want[ch] / 3) < 2e-3 for ch in range(4)), (x, y)
    assert c["FX-VBLUR-004"]["0"] != c["FX-VBLUR-003"]["0"]
    assert c["FX-VBLUR-010"]["0"] != c["FX-VBLUR-004"]["0"]
    # 011 and 012: pixels two or more from the disc's rim untouched.
    inside = lambda x, y: (x + 0.5 - 8) ** 2 + (y + 0.5 - 5) ** 2 <= 3.5 ** 2  # noqa: E731
    for y in range(H):
        for x in range(W):
            rim = any(inside(x + i, y + j) != inside(x, y) for i in (-1, 0, 1) for j in (-1, 0, 1))
            if not rim:
                for fx in ("FX-VBLUR-011", "FX-VBLUR-012"):
                    assert c[fx]["0"][y * W + x] == drawn[y * W + x], (fx, x, y)
    # 013: an alpha map on an opaque middle: the pixels away from every edge untouched.
    for y in range(1, H - 1):
        for x in range(1, W - 1):
            if not (x >= 11 and y >= 6):
                assert c["FX-VBLUR-013"]["0"][y * W + x] == drawn[y * W + x], (x, y)
    assert c["FX-VBLUR-016"]["2"] == c["FX-VBLUR-003"]["0"]
    assert c["FX-VBLUR-017"]["0"] == c["FX-VBLUR-001"]["0"] == c["FX-VBLUR-017"]["4"]
    moved = c["FX-VBLUR-018"]["0"]
    assert all(moved[(y + 1) * W + x + 2] == c["FX-VBLUR-003"]["0"][y * W + x]
               for y in range(H - 1) for x in range(W - 2))
    # Blurring moves light and never makes it.
    assert all(-1e-12 <= p[3] <= 1 + 1e-12 for v in c.values() for px in v.values() for p in px)
    print("checks passed")


if __name__ == "__main__":
    main()
