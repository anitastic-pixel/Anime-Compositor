"""Add Grain, worked a second way.

D-443 adds `core.add_grain`, After Effects' Add Grain (Noise & Grain): it "adds artificial film
grain to footage", and its controls set "the color, tonal range, blending mode, and animation
properties of the grain" (Adobe's help page on the Noise & Grain effects). Its settings are a
subset of After Effects': Intensity, Size, Softness and Aspect Ratio and the three Channel
Intensities (Tweaking); Monochromatic and Saturation (Color); Blending Mode, Shadows, Midtones,
Highlights and Midpoint (Application); Animation Speed, Animate Smoothly and Random Seed
(Animation). Adobe publishes no formula and its starting values were not to be read; the numbers
below are this program's own rule, built on P0-19's value noise (`grade::value`, D-127, D-128,
D-299), and nothing is ported.

The rule, at a pixel (x, y) of the drawing's own pixels (its top-left corner (0, 0), however far
an effect above grew the buffer), straight colour e through the sRGB curve, each channel held in
0 and 1:

1. The grain's place: X = (x + 0.5) / (size aspect_ratio), Y = (y + 0.5) / size, in cells of
   `size` pixels down and `size aspect_ratio` across (a draft halves the size, so the grain keeps
   its size on the picture). Its depth z is frame animation_speed, or that number's whole part
   when Animate Smoothly is off.
2. The grain of channel c, n_c: P0-19's value noise at (X, Y, z) for the seed's whole part and
   channel c (channel 0 for all three when Monochromatic), worked twice, once with blocks (each
   cell one number, the depth still smooth) and once smooth, mixed (1 - softness) block +
   softness smooth. Corners that weigh nothing are not worked; they add nothing.
3. Colour: unless Monochromatic, with m the three's mean, n_c becomes m + saturation (n_c - m).
   Then n_c times the channel's intensity.
4. The tonal weight: L = 0.2126 e_r + 0.7152 e_g + 0.0722 e_b. At L up to the midpoint,
   shadows + (midtones - shadows) L / midpoint; above it, midtones + (highlights - midtones)
   (L - midpoint) / (1 - midpoint). The amount A is 0.1 intensity weight.
5. Blending: Film, e + A n 4 e (1 - e), so the grain is strongest in the middle of each channel
   and nothing at black or white; Add, e + A n; Overlay, the grain as a layer B = 0.5 + 0.5 A n
   held in 0 and 1, laid on in Overlay: 2 e B where e is at most a half, else
   1 - 2 (1 - e)(1 - B). The result is held in 0 and 1 and goes back through the curve.

A pixel that does not show is left; the covering is kept. Intensity 0 changes nothing.

Settings: `intensity` 0 to 10, 1 when added; `size` 0.1 to 100 pixels, 1; `softness` 0 to 1, 0;
`aspect_ratio` 0.25 to 4, 1; `red_intensity`, `green_intensity` and `blue_intensity` 0 to 10, 1;
`monochromatic` `off` (when added) or `on`; `saturation` 0 to 1, 1; `blending_mode` `film` (when
added), `add` or `overlay`; `shadows`, `midtones` and `highlights` 0 to 10, 1; `midpoint` 0.01 to
0.99, 0.5; `animation_speed` 0 to 10, 1; `animate_smoothly` `on` (when added) or `off`;
`random_seed` 0 to 100000, 0, its whole part counted. Every number can be keyed.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values.

Every case is a composition 16 by 10 holding Noise's card, the same size, unmoved unless the case
says. The expected frames are in `Fixtures/addgrain/expected_addgrain.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/addgrain_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from motion_tile_reference import motion_tile  # noqa: E402
from noise_reference import DRAWINGS, u  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "addgrain"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"intensity": (0, 10), "size": (0.1, 100), "softness": (0, 1), "aspect_ratio": (0.25, 4),
          "red_intensity": (0, 10), "green_intensity": (0, 10), "blue_intensity": (0, 10),
          "saturation": (0, 1), "shadows": (0, 10), "midtones": (0, 10), "highlights": (0, 10),
          "midpoint": (0.01, 0.99), "animation_speed": (0, 10), "random_seed": (0, 100000)}
WORDS = ("monochromatic", "blending_mode", "animate_smoothly")
NAMES = ("intensity", "size", "softness", "aspect_ratio", "red_intensity", "green_intensity",
         "blue_intensity", "monochromatic", "saturation", "blending_mode", "shadows", "midtones",
         "highlights", "midpoint", "animation_speed", "animate_smoothly", "random_seed")


# --- the rule -------------------------------------------------------------------------------

def fade(t):
    return t * t * t * (t * (6 * t - 15) + 10)


def value(seed, ch, x, y, z, block):
    """grade::value: the eight corners of the cell round (x, y, z), mixed by the smoothed place
    inside it; with `block` the cell is one number across and down. A corner that weighs 0 adds
    nothing and is skipped."""
    i, j, k = math.floor(x), math.floor(y), math.floor(z)
    s = [0.0, 0.0, fade(z - k)] if block else [fade(x - i), fade(y - j), fade(z - k)]
    v = 0.0
    for corner in range(8):
        d = [corner & 1, (corner >> 1) & 1, corner >> 2]
        w = 1.0
        for a in range(3):
            w *= s[a] if d[a] == 1 else 1 - s[a]
        if w != 0:
            v += w * u(seed, i + d[0], j + d[1], k + d[2], ch)
    return v


def grain(seed, ch, x, y, z, softness):
    if softness == 0:
        return value(seed, ch, x, y, z, True)
    if softness == 1:
        return value(seed, ch, x, y, z, False)
    return (1 - softness) * value(seed, ch, x, y, z, True) + softness * value(seed, ch, x, y, z, False)


def blend(mode, e, a):
    if mode == "film":
        return e + a * 4 * e * (1 - e)
    if mode == "add":
        return e + a
    b = min(1.0, max(0.0, 0.5 + 0.5 * a))
    return 2 * e * b if e <= 0.5 else 1 - 2 * (1 - e) * (1 - b)


def add_grain(layer, n, w, frame_no):
    if n["intensity"] == 0:
        return layer
    mono = w["monochromatic"] == "on"
    seed = math.floor(n["random_seed"])
    phase = frame_no * n["animation_speed"]
    z = phase if w["animate_smoothly"] == "on" else math.floor(phase)
    sx, sy = n["size"] * n["aspect_ratio"], n["size"]
    chan = (n["red_intensity"], n["green_intensity"], n["blue_intensity"])
    mid = n["midpoint"]
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            p = layer["px"][j * layer["w"] + i]
            a = p[3]
            if a <= 0:
                px.append(p)
                continue
            x, y = layer["left"] + i, layer["top"] + j
            X, Y = (x + 0.5) / sx, (y + 0.5) / sy
            if mono:
                g = [grain(seed, 0, X, Y, z, n["softness"])] * 3
            else:
                g = [grain(seed, c, X, Y, z, n["softness"]) for c in range(3)]
                m = (g[0] + g[1] + g[2]) / 3
                g = [m + n["saturation"] * (v - m) for v in g]
            g = [g[c] * chan[c] for c in range(3)]
            e = [S.linear_to_srgb(min(1.0, max(0.0, v / a))) for v in p[:3]]
            lum = 0.2126 * e[0] + 0.7152 * e[1] + 0.0722 * e[2]
            if lum <= mid:
                weight = n["shadows"] + (n["midtones"] - n["shadows"]) * lum / mid
            else:
                weight = n["midtones"] + (n["highlights"] - n["midtones"]) * (lum - mid) / (1 - mid)
            amount = 0.1 * n["intensity"] * weight
            o = [blend(w["blending_mode"], e[c], amount * g[c]) for c in range(3)]
            px.append([srgb_to_linear(min(1.0, max(0.0, v))) * a for v in o] + [a])
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def case(intensity=1, size=1, softness=0, aspect_ratio=1, red_intensity=1, green_intensity=1,
         blue_intensity=1, monochromatic="off", saturation=1, blending_mode="film", shadows=1,
         midtones=1, highlights=1, midpoint=0.5, animation_speed=1, animate_smoothly="on",
         random_seed=0, shift=0, tile=False):
    c = dict(locals())
    c["drawing"] = "card"
    return c


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def layer_of(c):
    layer = {"px": [R.working(p) for row in DRAWINGS[c["drawing"]] for p in row],
             "left": 0, "top": 0, "w": W, "h": H}
    return motion_tile(layer, 300, 300, "on") if c["tile"] else layer


def render(c, frame_no):
    n = {k: held(c, k, frame_no) for k in RANGES}
    return frame(add_grain(layer_of(c), n, {k: c[k] for k in WORDS}, frame_no), c["shift"])


def plain(c):
    return frame(layer_of(c), c["shift"])


ADD = {"blending_mode": "add"}

CASES = {
    "FX-ADDGRAIN-001": ("The settings as they start: intensity 1, size 1, Film, in colour, a new "
                        "grain every frame; every shown pixel moves by up to 0.1 a channel "
                        "through the sRGB curve, most in the middle of each channel, nothing in "
                        "the white and black patches; the empty pixels stay empty and the soft "
                        "edge keeps its half covering.", case(), [0, 2, 4]),
    "FX-ADDGRAIN-002": ("Animation Speed 0: the same grain on frames 0, 2 and 4, FX-ADDGRAIN-001's "
                        "frame 0.", case(animation_speed=0), [0, 2, 4]),
    "FX-ADDGRAIN-003": ("Animation Speed 0.5, smooth: frame 1 is half way between FX-ADDGRAIN-001's "
                        "frames 0 and 1 in the noise, and frame 2 is its frame 1.",
                        case(animation_speed=0.5), [1, 2]),
    "FX-ADDGRAIN-004": ("Animation Speed 0.5, Animate Smoothly off: frame 1 holds frame 0's grain "
                        "and frame 3 is FX-ADDGRAIN-001's frame 1.",
                        case(animation_speed=0.5, animate_smoothly="off"), [0, 1, 3]),
    "FX-ADDGRAIN-005": ("Monochromatic, Add: the three channels move together, a grey grain.",
                        case(monochromatic="on", **ADD), [0]),
    "FX-ADDGRAIN-006": ("Saturation 0, Add: each channel takes the three's mean, so they move "
                        "together, but less than FX-ADDGRAIN-005's.",
                        case(saturation=0, **ADD), [0]),
    "FX-ADDGRAIN-007": ("Saturation 0.5, Add: half way.", case(saturation=0.5, **ADD), [0]),
    "FX-ADDGRAIN-008": ("Size 4, Add: grains of 4 by 4 pixels, each one number.",
                        case(size=4, **ADD), [0]),
    "FX-ADDGRAIN-009": ("Size 4, Softness 1, Add: the grains blend smoothly into each other.",
                        case(size=4, softness=1, **ADD), [0]),
    "FX-ADDGRAIN-010": ("Size 4, Softness 0.5, Add: half way between FX-ADDGRAIN-008 and 009 in "
                        "the noise.", case(size=4, softness=0.5, **ADD), [0]),
    "FX-ADDGRAIN-011": ("Size 2, Aspect Ratio 2, Add: grains 4 across and 2 down.",
                        case(size=2, aspect_ratio=2, **ADD), [0]),
    "FX-ADDGRAIN-012": ("Channel intensities 2, 0 and 0.5, Add: red's grain doubled, green "
                        "untouched, blue's halved.",
                        case(red_intensity=2, green_intensity=0, blue_intensity=0.5, **ADD), [0]),
    "FX-ADDGRAIN-013": ("Shadows 0, Highlights 0, Add: no grain in the black or the white patch, "
                        "and most in the middle tones.", case(shadows=0, highlights=0, **ADD),
                        [0]),
    "FX-ADDGRAIN-014": ("Shadows 3, Midtones 0, Highlights 0.5, Midpoint 0.3, Add: strong grain "
                        "in the dark line, none at brightness 0.3.",
                        case(shadows=3, midtones=0, highlights=0.5, midpoint=0.3, **ADD), [0]),
    "FX-ADDGRAIN-015": ("Add, intensity 3: the white patch can only darken and the black only "
                        "lighten; elsewhere up to 0.3 either way.", case(intensity=3, **ADD), [0]),
    "FX-ADDGRAIN-016": ("Overlay, intensity 2: the grain laid on in Overlay.",
                        case(intensity=2, blending_mode="overlay"), [0]),
    "FX-ADDGRAIN-017": ("Intensity 0: the drawing, untouched.", case(intensity=0), [0, 2]),
    "FX-ADDGRAIN-018": ("Intensity 10, Add: the grain at its strongest.",
                        case(intensity=10, **ADD), [0]),
    "FX-ADDGRAIN-019": ("Random Seed 7: a different grain from FX-ADDGRAIN-001's.",
                        case(random_seed=7), [0]),
    "FX-ADDGRAIN-020": ("Random Seed 3.", case(random_seed=3), [0]),
    "FX-ADDGRAIN-021": ("Random Seed 3.7: its whole part counts, so this is FX-ADDGRAIN-020.",
                        case(random_seed=3.7), [0]),
    "FX-ADDGRAIN-022": ("Intensity keyed from 0 at frame 0 to 4 at frame 4, speed 0, Add: frame 0 "
                        "is the drawing; frame 4's grain is frame 2's, twice as strong.",
                        case(intensity=keyed((0, 0), (4, 4)), animation_speed=0, **ADD),
                        [0, 2, 4]),
    "FX-ADDGRAIN-023": ("Size keyed from 1 at frame 0 to 5 at frame 4, speed 0: frame 2 is size 3.",
                        case(size=keyed((0, 1), (4, 5)), animation_speed=0), [0, 2, 4]),
    "FX-ADDGRAIN-024": ("FX-ADDGRAIN-001 moved three pixels right: the grain is the drawing's "
                        "own, so it moves with it.", case(shift=3), [0, 2]),
    "FX-ADDGRAIN-025": ("After a Motion Tile that grows the layer: the grain is worked in the "
                        "drawing's own pixels, so the frame is FX-ADDGRAIN-001's.",
                        case(tile=True), [0, 2]),
    "FX-ADDGRAIN-026": ("Animation Speed 2: frame 1 is FX-ADDGRAIN-001's frame 2.",
                        case(animation_speed=2), [1, 2]),
    "FX-ADDGRAIN-027": ("Midpoint keyed from 0.2 at frame 0 to 0.8 at frame 4 with Shadows 0 and "
                        "Highlights 2, speed 0, Add: the dark tones gain grain as the midpoint "
                        "rises.", case(midpoint=keyed((0, 0.2), (4, 0.8)), shadows=0, highlights=2,
                                       animation_speed=0, **ADD), [0, 4]),
    "FX-ADDGRAIN-028": ("Monochromatic, Size 3, Softness 0.3, Aspect 0.5, Film, intensity 2, "
                        "speed 0.25 smooth, seed 11: the controls together.",
                        case(monochromatic="on", size=3, softness=0.3, aspect_ratio=0.5,
                             intensity=2, animation_speed=0.25, random_seed=11), [0, 1, 2, 3, 4]),
}

INVALID = {
    "FX-ADDGRAIN-029": ("Intensity 11, above 10.", case(intensity=11)),
    "FX-ADDGRAIN-030": ("Size 0.05, below 0.1.", case(size=0.05)),
    "FX-ADDGRAIN-031": ("Softness 1.5, above 1.", case(softness=1.5)),
    "FX-ADDGRAIN-032": ("Aspect Ratio 5, above 4.", case(aspect_ratio=5)),
    "FX-ADDGRAIN-033": ("Green Intensity -1, below 0.", case(green_intensity=-1)),
    "FX-ADDGRAIN-034": ("Saturation 2, above 1.", case(saturation=2)),
    "FX-ADDGRAIN-035": ("Midpoint 1, above 0.99.", case(midpoint=1)),
    "FX-ADDGRAIN-036": ("Animation Speed 11, above 10.", case(animation_speed=11)),
    "FX-ADDGRAIN-037": ("Random Seed 100001, above 100000.", case(random_seed=100001)),
    "FX-ADDGRAIN-038": ("Blending Mode \"screen\", not one of its three words.",
                        case(blending_mode="screen")),
    "FX-ADDGRAIN-039": ("Monochromatic \"yes\", not off or on.", case(monochromatic="yes")),
    "FX-ADDGRAIN-040": ("Animate Smoothly \"On\", in capitals, kept as written and not the word.",
                        case(animate_smoothly="On")),
    "FX-ADDGRAIN-041": ("Shadows keyed to 20 at frame 4, above 10.",
                        case(shadows=keyed((0, 1), (4, 20)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = S.project_json(fx, {"drawing": c["drawing"], "shift": c["shift"],
                            "softness": 0, "threshold": 0})
    p["assets"][0]["path"] = f"media/{c['drawing']}.png"
    comp = p["compositions"][0]
    comp["width"], comp["height"] = W, H
    t = comp["layers"][0]["transform"]
    t["anchor"] = prop([W / 2, H / 2])
    t["position"] = prop([W / 2 + c["shift"], H / 2])
    effects = []
    if c["tile"]:
        effects.append({"instance_id": "fx-0-0", "type_id": "core.motion_tile", "enabled": True,
                        "parameters": {"output_width": 300, "output_height": 300,
                                       "mirror": "on"}})
    effects.append({"instance_id": f"fx-0-{len(effects)}", "type_id": "core.add_grain",
                    "enabled": True,
                    "parameters": {k: (c[k] if k in WORDS else setting_json(c[k]))
                                   for k in NAMES}})
    comp["layers"][0]["effects"] = effects
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))
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
    (OUT / "expected_addgrain.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    drawn = plain(case())
    shown = [i for i in range(W * H) if drawn[i][3] > 0]
    enc = lambda p: [S.linear_to_srgb(v / p[3]) for v in p[:3]]  # noqa: E731
    step = lambda f, i: [a - b for a, b in zip(enc(f[i]), enc(drawn[i]))]  # noqa: E731
    near = lambda p, q, e=1e-9: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda f, g: all(near(f[i], g[i]) for i in range(W * H))  # noqa: E731
    differ = lambda f, g: sum(not near(f[i], g[i]) for i in shown) > len(shown) * 0.8  # noqa: E731
    free = lambda f, i: [ch for ch in range(3) if 1e-9 < enc(f[i])[ch] < 1 - 1e-9]  # noqa: E731
    grey, soft = at(1, 1), at(15, 4)
    whites = [at(x, y) for x in (5, 6) for y in (3, 4)]
    blacks = [at(x, y) for x in (9, 10) for y in (3, 4)]

    # The rule's pieces: block noise at a whole depth is one corner's number; smooth noise at a
    # cell's corner is that corner's number too.
    assert value(5, 0, 2.3, 7.9, 3.0, True) == u(5, 2, 7, 3, 0)
    assert value(5, 1, 2.0, 7.0, 3.0, False) == u(5, 2, 7, 3, 1)
    assert value(5, 0, 2.3, 7.9, 3.5, True) == 0.5 * u(5, 2, 7, 3, 0) + 0.5 * u(5, 2, 7, 4, 0)

    one = c["FX-ADDGRAIN-001"]
    assert differ(one["0"], one["2"]) and differ(one["2"], one["4"])
    for f in one.values():
        for i in range(W * H):
            if drawn[i][3] == 0:
                assert f[i] == [0.0] * 4
            else:
                assert f[i][3] == drawn[i][3]
                assert all(abs(v) <= 0.1 + 1e-12 for v in step(f, i))
        for i in whites + blacks:
            assert near(f[i], drawn[i], 1e-12)
        assert f[soft][3] == 128 / 255
    two = c["FX-ADDGRAIN-002"]
    assert two["0"] == two["2"] == two["4"] == one["0"]
    three = c["FX-ADDGRAIN-003"]
    one1 = render(case(), 1)
    assert same(three["2"], one1) and not same(three["1"], one["0"]) and not same(three["1"], one1)
    four = c["FX-ADDGRAIN-004"]
    assert four["0"] == four["1"] == one["0"] and four["3"] == one1
    five, six, seven = c["FX-ADDGRAIN-005"]["0"], c["FX-ADDGRAIN-006"]["0"], c["FX-ADDGRAIN-007"]["0"]
    for f in (five, six):
        for i in shown:
            d = step(f, i)
            fr = free(f, i)
            assert near([d[ch] for ch in fr], [d[fr[0]]] * len(fr)) if fr else True, i
    assert sum(abs(step(five, i)[0]) for i in shown) > sum(abs(step(six, i)[0]) for i in shown)
    d7 = step(seven, grey)
    assert abs(d7[0] - d7[1]) > 1e-6
    eight = c["FX-ADDGRAIN-008"]["0"]
    # Columns 1 to 2 and rows 0 to 2 of the grey card lie in the cell (0, 0); (1, 1) to (2, 2) agree.
    assert near(step(eight, at(1, 1)), step(eight, at(2, 2))) and near(step(eight, at(1, 0)), step(eight, at(2, 1)))
    assert not near(step(eight, at(1, 1)), step(eight, at(1, 5)))
    nine, ten = c["FX-ADDGRAIN-009"]["0"], c["FX-ADDGRAIN-010"]["0"]
    assert not near(step(nine, at(1, 1)), step(nine, at(2, 2)))
    for i in shown:
        for ch in free(ten, i):
            if ch in free(eight, i) and ch in free(nine, i):
                assert abs(step(ten, i)[ch] - (step(eight, i)[ch] + step(nine, i)[ch]) / 2) < 1e-9
    eleven = c["FX-ADDGRAIN-011"]["0"]
    assert near(step(eleven, at(1, 0)), step(eleven, at(3, 1)))  # one cell 4 across, 2 down
    assert not near(step(eleven, at(1, 0)), step(eleven, at(1, 2)))
    twelve = c["FX-ADDGRAIN-012"]["0"]
    plain_add = render(case(**ADD), 0)
    for i in shown:
        e0, e1 = step(twelve, i), step(plain_add, i)
        assert abs(e0[1]) < 1e-9
        if free(twelve, i) == [0, 1, 2] and free(plain_add, i) == [0, 1, 2]:
            assert abs(e0[0] - 2 * e1[0]) < 1e-9 and abs(e0[2] - 0.5 * e1[2]) < 1e-9
    thirteen = c["FX-ADDGRAIN-013"]["0"]
    for i in whites + blacks:
        assert near(thirteen[i], drawn[i], 1e-12)
    assert differ(thirteen, drawn)
    fourteen = c["FX-ADDGRAIN-014"]["0"]
    line = at(3, 3)  # the line #1e1a24, dark: strong grain
    assert max(abs(v) for v in step(fourteen, line)) > max(abs(v) for v in step(plain_add, line))
    fifteen = c["FX-ADDGRAIN-015"]["0"]
    for i in whites:
        assert all(v <= drawn[i][3] + 1e-12 for v in fifteen[i][:3])
    assert any(fifteen[i][:3] != drawn[i][:3] for i in whites + blacks)
    assert max(max(abs(v) for v in step(fifteen, i)) for i in shown) > 0.1
    assert differ(c["FX-ADDGRAIN-016"]["0"], drawn)
    assert c["FX-ADDGRAIN-017"]["0"] == drawn == c["FX-ADDGRAIN-017"]["2"]
    assert differ(c["FX-ADDGRAIN-019"]["0"], one["0"])
    assert c["FX-ADDGRAIN-021"]["0"] == c["FX-ADDGRAIN-020"]["0"]
    k22 = c["FX-ADDGRAIN-022"]
    assert k22["0"] == drawn
    for i in shown:
        for ch in range(3):
            if ch in free(k22["4"], i):
                assert abs(step(k22["4"], i)[ch] - 2 * step(k22["2"], i)[ch]) < 1e-9
    assert c["FX-ADDGRAIN-023"]["2"] == render(case(size=3, animation_speed=0), 0)
    moved = c["FX-ADDGRAIN-024"]
    for f in ("0", "2"):
        assert all(moved[f][at(x, y)] == one[f][at(x - 3, y)] for x in range(3, W) for y in range(H))
    tiled = c["FX-ADDGRAIN-025"]
    assert same(tiled["0"], one["0"]) and same(tiled["2"], one["2"])
    assert same(c["FX-ADDGRAIN-026"]["1"], one["2"])
    k27 = c["FX-ADDGRAIN-027"]
    assert not same(k27["0"], k27["4"])
    k28 = c["FX-ADDGRAIN-028"]
    assert k28["0"] != k28["4"] and differ(k28["0"], drawn)
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == drawn
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(0 <= v <= p[3] + 1e-12 for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
