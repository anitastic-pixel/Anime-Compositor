"""Turbulent noise, worked a second way.

D-445 adds `core.turbulent_noise`, after After Effects' Turbulent Noise: Adobe's newer, faster
Fractal Noise with a smaller set of controls and no Cycle Evolution. It is a second name over
this program's Fractal Noise (D-128, D-299), by D-383's "Both names, one engine" and D-394's
"Second name, one engine", as Tritone is over Gradient Map (D-402): Fractal Noise with no speed,
black to white, never cycling. Its settings are After Effects' Fractal Type ("basic" or
"turbulent"), Noise Type ("smooth" or "block"), Invert ("off" or "on"), Contrast, Brightness,
the size of a cloud in pixels with Scale Width and Scale Height in per cent of it, Offset
Turbulence in pixels, Complexity, Evolution, Random Seed, Opacity and Blending Mode. It is this
program's own method; nothing is ported. Document 21 is the rule in words; this file is the
reference for the numbers document 25 pins against it.

The field. V(seed, ch, x, y, z, block) is D-128's smooth value noise in -1..1 (from
`tools/fractal_noise_reference.py`): with i, j, k = floor(x), floor(y), floor(z) and sx, sy, sz
= fade of the parts past them, the sum over the eight corners of their weights times U(seed,
i + di, j + dj, k + dk, ch), U Noise's hash (D-119); with `block` sx = sy = 0, each cell one
number across and down while the depth still mixes (D-299). F is n octaves: octave o is
V(seed, 8 o, x 2^o, y 2^o, z 2^o, block) at strength 0.5^o; basic gives the sum over the sum of
the strengths, turbulent gives 2 (the sum of the strengths times |V|) over the sum, less 1.
Pixel (X, Y) of the drawing's own space samples ((X + 0.5 - offset x) / (size scale_width /
100), (Y + 0.5 - offset y) / (size scale_height / 100), evolution / 360).

The rule. At a pixel with a > 0: n = F, turned over (-n) when inverted; v = clamp(0.5 + 0.5 n
contrast / 100 + brightness / 100, 0, 1); G = srgb_to_linear(v) in each channel; with b the
straight linear colour, f = G (normal), b G (multiply), 1 - (1 - b)(1 - G) (screen) or b + G
(add), not clamped, and the output is ((b + op (f - b)) a, a) with op = opacity / 100.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: Noise's card (`tools/noise_reference.py`). The drawing goes into
`Fixtures/turbulent_noise/media`, the projects into `Fixtures/turbulent_noise`, and the expected
frames into `Fixtures/turbulent_noise/expected_turbulent_noise.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/turbulent_noise_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from noise_reference import u as U  # noqa: E402
from gradient_reference import BLENDS  # noqa: E402
import fractal_noise_reference as FN  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "turbulent_noise"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"contrast": (0, 1000), "brightness": (-1000, 1000), "size": (1, 1000),
          "scale_width": (1, 10000), "scale_height": (1, 10000), "complexity": (1, 20),
          "evolution": (-100000, 100000), "seed": (0, 100000), "opacity": (0, 100)}
FLOORED = ("complexity", "seed")
NAMES = ("fractal_type", "noise_type", "invert", "contrast", "brightness", "size",
         "scale_width", "scale_height", "offset", "complexity", "evolution", "seed", "opacity",
         "blend")


# --- the field ------------------------------------------------------------------------------

def V(seed, ch, x, y, z, block):
    """D-128's value noise; with `block` each cell is one number across and down."""
    if not block:
        return FN.V(seed, ch, x, y, z)
    i, j, k = math.floor(x), math.floor(y), math.floor(z)
    sz = FN.fade(z - k)
    return (1 - sz) * U(seed, i, j, k, ch) + sz * U(seed, i, j, k + 1, ch)


def F(seed, x, y, z, n, turbulent, block):
    total = weight = 0.0
    for o in range(n):
        s = 2 ** o
        v = V(seed, 8 * o, x * s, y * s, z * s, block)
        total += 0.5 ** o * (abs(v) if turbulent else v)
        weight += 0.5 ** o
    return 2 * total / weight - 1 if turbulent else total / weight


def point(x, y, n, offset):
    return ((x + 0.5 - offset[0]) / (n["size"] * n["scale_width"] / 100),
            (y + 0.5 - offset[1]) / (n["size"] * n["scale_height"] / 100),
            n["evolution"] / 360)


# --- the rule -------------------------------------------------------------------------------

def value(c, n, x, y):
    f = F(n["seed"], *point(x, y, n, c["offset"]), n["complexity"],
          c["fractal_type"] == "turbulent", c["noise_type"] == "block")
    if c["invert"] == "on":
        f = -f
    return min(1.0, max(0.0, 0.5 + 0.5 * f * n["contrast"] / 100 + n["brightness"] / 100))


def turbulent_noise(pixels, c, n, width=W):
    """`pixels` are 8-bit straight RGBA, rows top to bottom; `n` the numbers already held."""
    mix, op = BLENDS[c["blend"]], n["opacity"] / 100
    out = []
    for i, p in enumerate(pixels):
        if p[3] == 0:
            out.append(R.working(p))
            continue
        g = srgb_to_linear(value(c, n, i % width, i // width))
        a = p[3] / 255
        b = [srgb_to_linear(p[ch] / 255) for ch in range(3)]
        out.append([(b[ch] + op * (mix(b[ch], g) - b[ch])) * a for ch in range(3)] + [a])
    return out


# --- the cases ------------------------------------------------------------------------------

DRAWINGS = FN.DRAWINGS  # Noise's card: grey, skin, white, black, a soft edge


def case(fractal_type="basic", noise_type="smooth", invert="off", contrast=100, brightness=0,
         size=100, scale_width=100, scale_height=100, offset=(0, 0), complexity=6, evolution=0,
         seed=0, opacity=100, blend="normal", shift=0):
    return {"drawing": "card", "fractal_type": fractal_type, "noise_type": noise_type,
            "invert": invert, "contrast": contrast, "brightness": brightness, "size": size,
            "scale_width": scale_width, "scale_height": scale_height, "offset": list(offset),
            "complexity": complexity, "evolution": evolution, "seed": seed, "opacity": opacity,
            "blend": blend, "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = min(hi, max(lo, value_at(c[k], frame_no)))
    return math.floor(v) if k in FLOORED else v


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    n = {k: held(c, k, frame_no) for k in RANGES}
    return R.frame(turbulent_noise(pixels, c, n), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(opacity=0, shift=c["shift"]), 0)


FINE = {"size": 4}  # four pixels a cloud, so the clouds vary across the small drawing

CASES = {
    "FX-TURBNOISE-001": ("The settings as they start: basic, smooth, not inverted, contrast 100, "
                         "brightness 0, size 100 at scale 100 by 100, offset 0, complexity 6, "
                         "evolution 0, seed 0, opacity 100, normal. Grey clouds so large that "
                         "the card sees a small part of one: close greys near the middle. Every "
                         "pixel keeps its covering, the empty ones stay empty, and with nothing "
                         "keyed frame 4 is frame 0: Turbulent Noise has no speed of its own.",
                         case(), [0, 4]),
    "FX-TURBNOISE-002": ("Size 4: four pixels a cloud, so the greys vary across the card.",
                         case(**FINE), [0]),
    "FX-TURBNOISE-003": ("Size 4, fractal type turbulent: each octave's distance from the "
                         "middle, so the clouds crease into dark veins.",
                         case(fractal_type="turbulent", **FINE), [0]),
    "FX-TURBNOISE-004": ("Size 4, noise type block: each cell of each octave one grey, square "
                         "steps.", case(noise_type="block", **FINE), [0]),
    "FX-TURBNOISE-005": ("Size 4, turbulent and block together.",
                         case(fractal_type="turbulent", noise_type="block", **FINE), [0]),
    "FX-TURBNOISE-006": ("Size 4, invert on: FX-TURBNOISE-002 turned over about the middle "
                         "grey, light where it was dark.", case(invert="on", **FINE), [0]),
    "FX-TURBNOISE-007": ("Size 4, complexity 1.9, which counts as 1: one octave, the smooth "
                         "value noise itself.", case(complexity=1.9, **FINE), [0]),
    "FX-TURBNOISE-008": ("Size 4, complexity 20, the most: twenty octaves.",
                         case(complexity=20, **FINE), [0]),
    "FX-TURBNOISE-009": ("Size 4, contrast 300: FX-TURBNOISE-002's greys three times as far "
                         "from the middle, held at black and white.",
                         case(contrast=300, **FINE), [0]),
    "FX-TURBNOISE-010": ("Size 4, brightness 30: FX-TURBNOISE-002 lifted by 0.3, held at white.",
                         case(brightness=30, **FINE), [0]),
    "FX-TURBNOISE-011": ("Brightness -100: held at 0 everywhere, every shown pixel black.",
                         case(brightness=-100), [0]),
    "FX-TURBNOISE-012": ("Size 4, evolution 360: one full turn moves the field one cell through "
                         "its third direction, clouds of their own.",
                         case(evolution=360, **FINE), [0]),
    "FX-TURBNOISE-013": ("Size 4, evolution keyed from 0 at frame 0 to 720 at frame 4, linear: "
                         "frame 0 is FX-TURBNOISE-002 and frame 2, at 360, is FX-TURBNOISE-012.",
                         case(evolution=keyed((0, 0), (4, 720)), **FINE), [0, 2, 4]),
    "FX-TURBNOISE-014": ("Size 4, seed 7: clouds of their own.", case(seed=7, **FINE), [0]),
    "FX-TURBNOISE-015": ("Size 4, seed 7.9, which counts as 7: FX-TURBNOISE-014.",
                         case(seed=7.9, **FINE), [0]),
    "FX-TURBNOISE-016": ("Size 4, scale width 200 and scale height 50: clouds twice as wide and "
                         "half as tall.", case(scale_width=200, scale_height=50, **FINE), [0]),
    "FX-TURBNOISE-017": ("Size 4, offset 2.5 right and 3 up: the clouds slide with it.",
                         case(offset=(2.5, -3), **FINE), [0]),
    "FX-TURBNOISE-018": ("Size 4, blend multiply: the card darkened by the clouds; the black "
                         "patch stays black.", case(blend="multiply", **FINE), [0]),
    "FX-TURBNOISE-019": ("Size 4, blend screen, opacity 50: the card lightened, none darkened.",
                         case(blend="screen", opacity=50, **FINE), [0]),
    "FX-TURBNOISE-020": ("Size 4, blend add, opacity 50: added at half strength, not held, so "
                         "the white patch goes past its covering.",
                         case(blend="add", opacity=50, **FINE), [0]),
    "FX-TURBNOISE-021": ("Size 4, opacity keyed from 0 at frame 0 to 100 at frame 4, eased past "
                         "its end: frame 0 is the drawing; frame 2 is held at 100 and is frame "
                         "4, FX-TURBNOISE-002.",
                         case(opacity=keyed((0, 0, OVERSHOOT), (4, 100)), **FINE), [0, 2, 4]),
    "FX-TURBNOISE-022": ("FX-TURBNOISE-002 moved three pixels right: the clouds are worked in "
                         "the drawing's own space, so they move with it.",
                         case(shift=3, **FINE), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-TURBNOISE-023": ("Size 0, below 1.", case(size=0)),
    "FX-TURBNOISE-024": ("Complexity 21, above 20.", case(complexity=21)),
    "FX-TURBNOISE-025": ("Contrast 1001, above 1000.", case(contrast=1001)),
    "FX-TURBNOISE-026": ("Brightness keyed to -1200 at frame 4, below -1000.",
                         case(brightness=keyed((0, 0), (4, -1200)))),
    "FX-TURBNOISE-027": ("Seed -1, below 0.", case(seed=-1)),
    "FX-TURBNOISE-028": ("Scale width 0, below 1.", case(scale_width=0)),
    "FX-TURBNOISE-029": ("Fractal type \"dynamic\", one of After Effects' that is not built.",
                         case(fractal_type="dynamic")),
    "FX-TURBNOISE-030": ("Noise type \"spline\", one of After Effects' that is not built.",
                         case(noise_type="spline")),
    "FX-TURBNOISE-031": ("Invert \"yes\", which is not \"off\" or \"on\".", case(invert="yes")),
    "FX-TURBNOISE-032": ("Blend \"overlay\", which is not a blend here.", case(blend="overlay")),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = FN.project_json(fx, {**FN.case(), "shift": c["shift"]})
    p["compositions"][0]["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.turbulent_noise", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in NAMES}}]
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

    (OUT / "expected_turbulent_noise.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                       encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    shown = [i for i in range(W * H) if drawn[i][3] > 0]
    enc = lambda p: [S.linear_to_srgb(min(1.0, max(0.0, v / p[3]))) for v in p[:3]]  # noqa: E731
    grey = lambda f, i: enc(f[i])[0]  # noqa: E731
    white = [at(x, y) for x in (5, 6) for y in (3, 4)]
    black = [at(x, y) for x in (9, 10) for y in (3, 4)]

    # The field's pieces: smooth V is Fractal Noise's; block V at a whole depth is one corner.
    assert V(3, 0, 1.3, 2.7, 0.2, False) == FN.V(3, 0, 1.3, 2.7, 0.2)
    assert V(3, 0, 1.3, 2.7, 4, True) == U(3, 1, 2, 4, 0) == V(3, 0, 1.9, 2.1, 4, True)
    assert F(3, 1.3, 2.7, 0.2, 4, False, False) == FN.F(3, 0, 1.3, 2.7, 0.2, 4)
    samples = [F(1, x / 7, y / 7, 0.3, 4, True, False) for x in range(40) for y in range(40)]
    assert all(-1 <= s <= 1 for s in samples)

    # Basic and smooth is Fractal Noise with no speed, black to white: the same numbers.
    two = c["FX-TURBNOISE-002"]["0"]
    assert two == FN.render(FN.case(size=4, complexity=6), 0)
    assert c["FX-TURBNOISE-001"]["0"] == FN.render(FN.case(complexity=6), 0)

    # Every case keeps every covering and leaves the empty pixels empty.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-TURBNOISE-022" else 0))
        for px in frames.values():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)

    one = c["FX-TURBNOISE-001"]
    assert one["0"] == one["4"]
    v1 = [grey(one["0"], i) for i in shown]
    v2 = [grey(two, i) for i in shown]
    assert max(v1) - min(v1) < max(v2) - min(v2)
    three, four, five = (c[f"FX-TURBNOISE-00{k}"]["0"] for k in (3, 4, 5))
    assert len({str(two), str(three), str(four), str(five)}) == 4
    six = c["FX-TURBNOISE-006"]["0"]
    for i, v in zip(shown, v2):
        assert abs(grey(six, i) - (1 - v)) < 1e-9  # contrast 100, brightness 0: turned over
    seven = c["FX-TURBNOISE-007"]["0"]
    for i in shown:
        p = point(i % W, i // W, {**case(**FINE)}, (0, 0))
        assert abs(grey(seven, i) - (0.5 + 0.5 * FN.V(0, 0, *p))) < 1e-9
    assert c["FX-TURBNOISE-008"]["0"] != two
    nine, ten = c["FX-TURBNOISE-009"]["0"], c["FX-TURBNOISE-010"]["0"]
    clipped = 0
    for i, v in zip(shown, v2):
        assert abs(grey(nine, i) - min(1, max(0, 0.5 + 3 * (v - 0.5)))) < 1e-9
        assert abs(grey(ten, i) - min(1, v + 0.3)) < 1e-9
        clipped += grey(nine, i) in (0.0, 1.0)
    assert clipped > 0
    assert all(c["FX-TURBNOISE-011"]["0"][i][:3] == [0.0] * 3 for i in shown)
    twelve, thirteen = c["FX-TURBNOISE-012"]["0"], c["FX-TURBNOISE-013"]
    assert twelve != two and thirteen["0"] == two and thirteen["2"] == twelve
    assert c["FX-TURBNOISE-014"]["0"] != two
    assert c["FX-TURBNOISE-015"]["0"] == c["FX-TURBNOISE-014"]["0"]
    assert c["FX-TURBNOISE-016"]["0"] != two and c["FX-TURBNOISE-017"]["0"] != two
    eighteen, nineteen = c["FX-TURBNOISE-018"]["0"], c["FX-TURBNOISE-019"]["0"]
    assert all(eighteen[i][ch] <= drawn[i][ch] + 1e-12 for i in shown for ch in range(3))
    assert all(eighteen[i][:3] == [0.0] * 3 for i in black)
    assert all(nineteen[i][ch] >= drawn[i][ch] - 1e-12 for i in shown for ch in range(3))
    assert all(c["FX-TURBNOISE-020"]["0"][i][0] > 1 for i in white)
    twenty_one = c["FX-TURBNOISE-021"]
    assert twenty_one["0"] == drawn and twenty_one["2"] == twenty_one["4"] == two
    moved = c["FX-TURBNOISE-022"]
    assert moved["0"] == moved["3"]
    for y in range(H):
        assert moved["0"][at(3, y):at(0, y + 1)] == two[at(0, y):at(W - 3, y)]
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(v >= 0 for v in p[:3]), (name, p)
                if name != "FX-TURBNOISE-020":
                    assert all(v <= p[3] + 1e-12 for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
