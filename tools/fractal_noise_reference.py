"""Fractal noise, worked a second way.

D-128 adds `core.fractal_noise`, a generator: fog, mist, clouds, drawn inside a layer's covering,
on a solid or on any drawing. At each pixel that shows it works out a smooth, cloudy noise value
between 0 and 1, fixed by the seed and the pixel's place in the drawing's own space, turns it
into a colour between `dark_color` and `light_color`, and mixes that colour into the pixel by
`blend` at `opacity` per cent, as Gradient does (D-114). `size` is how many pixels one cloud cell
spans; `complexity` is how many finer layers of detail are added on top of it, each half the size
and half the strength of the one before; `contrast` stretches the value about its middle and
`brightness` lifts or lowers it. `evolution` moves the clouds through a third direction, one full
turn of 360 degrees moving them one cell, and `speed` adds that many degrees every frame, so with
speed not 0 the clouds change from frame to frame. Every pixel keeps its own covering, and a
pixel that does not show stays as it is. It is this program's own method, modelled on After
Effects' Fractal Noise; nothing is ported. Document 21 is the rule in words; this file is the
reference for the numbers document 25 pins against it.

The noise field, shared with Turbulent Displace (D-127, `tools/turbulent_displace_reference.py`,
which imports it from here). V(seed, ch, x, y, z) is smooth value noise in -1..1: with
i, j, k = floor(x), floor(y), floor(z) and sx, sy, sz = fade of the parts past them,
fade(t) = t^3 (t (6t - 15) + 10), V is the sum over the eight corners (di, dj, dk) in {0, 1}^3 of
wx(di) wy(dj) wz(dk) U(seed, i + di, j + dj, k + dk, ch), with w(0) = 1 - s and w(1) = s, U
Noise's hash (D-119, `tools/noise_reference.py`). F(seed, ch, x, y, z, n) is n octaves of it:
the sum over o = 0..n-1 of 0.5^o V(seed, 8 o + ch, x 2^o, y 2^o, z 2^o), over the sum of 0.5^o.
Pixel (X, Y) of the drawing's own space samples the point ((X + 0.5) / size, (Y + 0.5) / size, z)
with z = (evolution + speed * frame) / 360, frame the composition frame, a hidden value.

The rule. At a pixel with a > 0: v = clamp(0.5 + 0.5 F(seed, 0, point, complexity) contrast /
100 + brightness / 100, 0, 1); G = srgb_to_linear(dark + v (light - dark)) per channel, the
colours' 8-bit values over 255; with b the straight linear colour, f = G (normal), b G
(multiply), 1 - (1 - b)(1 - G) (screen) or b + G (add), not clamped, and the output is
((b + op (f - b)) a, a) with op = opacity / 100.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: Noise's card (`tools/noise_reference.py`). The drawing goes into
`Fixtures/fractal_noise/media`, the projects into `Fixtures/fractal_noise`, and the expected
frames into `Fixtures/fractal_noise/expected_fractal_noise.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/fractal_noise_reference.py
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
import noise_reference as N  # noqa: E402
from gradient_reference import BLENDS  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "fractal_noise"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"size": (1, 1000), "complexity": (1, 8), "contrast": (0, 1000),
          "brightness": (-100, 100), "evolution": (-100000, 100000), "speed": (-360, 360),
          "seed": (0, 100000), "opacity": (0, 100)}
FLOORED = ("complexity", "seed")
NAMES = ("size", "complexity", "contrast", "brightness", "evolution", "speed", "seed",
         "dark_color", "light_color", "opacity", "blend")


# --- the noise field, shared with Turbulent Displace ---------------------------------------

def fade(t):
    return t * t * t * (t * (6 * t - 15) + 10)


def V(seed, ch, x, y, z):
    """Smooth value noise in -1..1: the eight lattice corners' hashes, faded between."""
    i, j, k = math.floor(x), math.floor(y), math.floor(z)
    sx, sy, sz = fade(x - i), fade(y - j), fade(z - k)
    total = 0.0
    for dk, wz in ((0, 1 - sz), (1, sz)):
        for dj, wy in ((0, 1 - sy), (1, sy)):
            for di, wx in ((0, 1 - sx), (1, sx)):
                total += wx * wy * wz * U(seed, i + di, j + dj, k + dk, ch)
    return total


def F(seed, ch, x, y, z, n):
    """n octaves of V, each twice as fine and half as strong, over the sum of the strengths."""
    total = weight = 0.0
    for o in range(n):
        s = 2 ** o
        total += 0.5 ** o * V(seed, 8 * o + ch, x * s, y * s, z * s)
        weight += 0.5 ** o
    return total / weight


def point(x, y, size, evolution, speed, frame_no):
    """The field's point for pixel (x, y) of the drawing's own space at a composition frame."""
    return (x + 0.5) / size, (y + 0.5) / size, (evolution + speed * frame_no) / 360


# --- the rule -------------------------------------------------------------------------------

def value(seed, complexity, contrast, brightness, p):
    return min(1.0, max(0.0, 0.5 + 0.5 * F(seed, 0, *p, complexity) * contrast / 100
                        + brightness / 100))


def fractal_noise(pixels, size, complexity, contrast, brightness, evolution, speed, seed,
                  dark_color, light_color, opacity, blend, frame_no, width=W):
    """`pixels` are 8-bit straight RGBA, rows top to bottom, `width` wide, the drawing's
    top-left pixel at (0, 0). Numbers are already held; complexity and seed already floored."""
    dark = [v / 255 for v in R.hex_color(dark_color.lower())]
    light = [v / 255 for v in R.hex_color(light_color.lower())]
    mix, op = BLENDS[blend], opacity / 100
    out = []
    for i, p in enumerate(pixels):
        if p[3] == 0:
            out.append(R.working(p))
            continue
        v = value(seed, complexity, contrast, brightness,
                  point(i % width, i // width, size, evolution, speed, frame_no))
        g = [srgb_to_linear(d + v * (e - d)) for d, e in zip(dark, light)]
        a = p[3] / 255
        b = [srgb_to_linear(p[c] / 255) for c in range(3)]
        out.append([(b[c] + op * (mix(b[c], g[c]) - b[c])) * a for c in range(3)] + [a])
    return out


# --- the drawing ----------------------------------------------------------------------------

DRAWINGS = {"card": N.DRAWINGS["card"]}  # Noise's card: grey, skin, white, black, a soft edge


# --- the cases ------------------------------------------------------------------------------

def case(size=100, complexity=4, contrast=100, brightness=0, evolution=0, speed=0, seed=0,
         dark_color="#000000", light_color="#ffffff", opacity=100, blend="normal", shift=0):
    return {"drawing": "card", "size": size, "complexity": complexity, "contrast": contrast,
            "brightness": brightness, "evolution": evolution, "speed": speed, "seed": seed,
            "dark_color": dark_color, "light_color": light_color, "opacity": opacity,
            "blend": blend, "shift": shift}


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = min(hi, max(lo, value_at(c[k], frame_no)))
    return math.floor(v) if k in FLOORED else v


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    n = {k: held(c, k, frame_no) for k in RANGES}
    return R.frame(fractal_noise(pixels, n["size"], n["complexity"], n["contrast"],
                                 n["brightness"], n["evolution"], n["speed"], n["seed"],
                                 c["dark_color"], c["light_color"], n["opacity"], c["blend"],
                                 frame_no), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return render(case(opacity=0, shift=c["shift"]), 0)


FINE = {"size": 4}  # four pixels a cell, so the clouds vary across the small drawing
DUSK, CREAM = "#1e1a24", "#fff0b0"

CASES = {
    "FX-FRACTAL-001": ("The settings as they start: size 100, complexity 4, contrast 100, "
                       "brightness 0, evolution 0, speed 0, seed 0, black to white, opacity 100, "
                       "normal. The card is covered by grey clouds so large that the drawing sees "
                       "a small part of one cell: close greys, near the middle. Every pixel "
                       "keeps its covering, the empty ones stay empty, and with speed 0 frame 4 "
                       "is frame 0.",
                       case(), [0, 4]),
    "FX-FRACTAL-002": ("Size 4: four pixels a cell, so the greys vary across the card, "
                       "lighter and darker patches.",
                       case(**FINE), [0]),
    "FX-FRACTAL-003": ("Size 4, complexity 1.9, which counts as 1: one octave, the smooth value "
                       "noise itself, softer than FX-FRACTAL-002's four.",
                       case(complexity=1.9, **FINE), [0]),
    "FX-FRACTAL-004": ("Size 4, complexity 8: eight octaves, finer detail on top of "
                       "FX-FRACTAL-002's.",
                       case(complexity=8, **FINE), [0]),
    "FX-FRACTAL-005": ("Size 4, contrast 0: every shown pixel is the middle of the ramp, "
                       "encoded exactly 0.5, at its own covering.",
                       case(contrast=0, **FINE), [0]),
    "FX-FRACTAL-006": ("Size 4, contrast 300: FX-FRACTAL-002's greys three times as far from "
                       "the middle, held at black and white.",
                       case(contrast=300, **FINE), [0]),
    "FX-FRACTAL-007": ("Size 4, brightness 30: FX-FRACTAL-002's value lifted by 0.3, held at "
                       "white.",
                       case(brightness=30, **FINE), [0]),
    "FX-FRACTAL-008": ("Brightness -100: the value is held at 0 everywhere, so every shown "
                       "pixel is black at its own covering.",
                       case(brightness=-100), [0]),
    "FX-FRACTAL-009": ("Size 4, speed 90 degrees a frame: the clouds change from frame to "
                       "frame; frame 0 is FX-FRACTAL-002, frame 2 is evolution 180, and frame "
                       "4 is evolution 360, FX-FRACTAL-010.",
                       case(speed=90, **FINE), [0, 2, 4]),
    "FX-FRACTAL-010": ("Size 4, evolution 360: one full turn moves the field one cell through "
                       "its third direction, clouds of their own, not FX-FRACTAL-002's.",
                       case(evolution=360, **FINE), [0]),
    "FX-FRACTAL-011": ("Size 4, evolution keyed from 0 at frame 0 to 720 at frame 4, linear, "
                       "speed 0: frame 0 is FX-FRACTAL-002 and frame 2, at 360, is "
                       "FX-FRACTAL-010.",
                       case(evolution=keyed((0, 0), (4, 720)), **FINE), [0, 2, 4]),
    "FX-FRACTAL-012": ("Size 4, seed 7: clouds of their own.",
                       case(seed=7, **FINE), [0]),
    "FX-FRACTAL-013": ("Size 4, seed 7.9, which counts as 7: FX-FRACTAL-012.",
                       case(seed=7.9, **FINE), [0]),
    "FX-FRACTAL-014": (f"Size 4, dark {DUSK} and light {CREAM}: the clouds run from the dark "
                       "violet to the cream, mixed in encoded values.",
                       case(dark_color=DUSK, light_color=CREAM, **FINE), [0]),
    "FX-FRACTAL-015": ("FX-FRACTAL-014 with its colours written in capitals: the same.",
                       case(dark_color=DUSK.upper(), light_color=CREAM.upper(), **FINE), [0]),
    "FX-FRACTAL-016": ("Size 4, blend multiply: the card darkened by the clouds, its own "
                       "colours showing through; the black patch stays black.",
                       case(blend="multiply", **FINE), [0]),
    "FX-FRACTAL-017": ("Size 4, blend screen, opacity 50: the card lightened by the clouds, "
                       "none darkened; the white patch stays white.",
                       case(blend="screen", opacity=50, **FINE), [0]),
    "FX-FRACTAL-018": ("Size 4, blend add, opacity 50: the clouds added at half strength, not "
                       "held, so the white patch goes past its covering.",
                       case(blend="add", opacity=50, **FINE), [0]),
    "FX-FRACTAL-019": ("Size 4, opacity keyed from 0 at frame 0 to 100 at frame 4, eased past "
                       "its end (132.5 at frame 2): frame 0 is the drawing; frame 2 is held at "
                       "100 and is frame 4, FX-FRACTAL-002.",
                       case(opacity=keyed((0, 0, OVERSHOOT), (4, 100)), **FINE), [0, 2, 4]),
    "FX-FRACTAL-020": ("FX-FRACTAL-002 moved three pixels right: the clouds are worked in the "
                       "drawing's own space, so they move with it.",
                       case(shift=3, **FINE), [0, 3]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-FRACTAL-021": ("Size 0, below 1.", case(size=0)),
    "FX-FRACTAL-022": ("Complexity 9, above 8.", case(complexity=9)),
    "FX-FRACTAL-023": ("Contrast 1001, above 1000.", case(contrast=1001)),
    "FX-FRACTAL-024": ("Brightness -101, below -100.", case(brightness=-101)),
    "FX-FRACTAL-025": ("Seed -1, below 0.", case(seed=-1)),
    "FX-FRACTAL-026": ("Speed keyed to 400 at frame 4, above 360.",
                       case(speed=keyed((0, 0), (4, 400)))),
    "FX-FRACTAL-027": ("Blend \"overlay\", which is not a blend.", case(blend="overlay")),
    "FX-FRACTAL-028": ("A dark colour written \"#12345\", one digit short.",
                       case(dark_color="#12345")),
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
    comp["layers"][0]["effects"] = [{
        "instance_id": "fx-0-0", "type_id": "core.fractal_noise", "enabled": True,
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

    (OUT / "expected_fractal_noise.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                     encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    shown = [i for i in range(W * H) if drawn[i][3] > 0]
    near = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    enc = lambda p: [S.linear_to_srgb(min(1.0, max(0.0, v / p[3]))) for v in p[:3]]  # noqa: E731
    grey = lambda f, i: enc(f[i])[0]  # the value v, where the ramp is black to white  # noqa: E731
    white, black = [at(x, y) for x in (5, 6) for y in (3, 4)], [at(x, y) for x in (9, 10)
                                                                for y in (3, 4)]

    # The field's own pieces.
    assert fade(0) == 0 and fade(1) == 1 and fade(0.5) == 0.5
    assert V(3, 0, 2, 5, 1) == U(3, 2, 5, 1, 0)  # on a lattice corner, the corner's hash
    assert V(3, 0, -2, 5, -1) == U(3, -2, 5, -1, 0)  # negative corners too
    assert abs(V(3, 0, 2 - 1e-9, 5.3, 0.4) - V(3, 0, 2, 5.3, 0.4)) < 1e-6  # continuous
    assert F(3, 0, 1.3, 2.7, 0.2, 1) == V(3, 0, 1.3, 2.7, 0.2)
    samples = [F(1, 0, x / 7, y / 7, 0.3, 4) for x in range(40) for y in range(40)]
    assert all(-1 <= s <= 1 for s in samples) and abs(sum(samples) / len(samples)) < 0.1
    assert F(1, 1, 1.3, 2.7, 0.2, 2) != F(1, 0, 1.3, 2.7, 0.2, 2)  # channels differ

    # Every case keeps every covering and leaves the empty pixels empty.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-FRACTAL-020" else 0))
        for px in frames.values():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)

    def ramp(f):
        """Normal at 100, black to white: each shown pixel is a grey, v, at its covering."""
        for i in shown:
            e = enc(f[i])
            assert abs(e[0] - e[1]) < 1e-9 and abs(e[1] - e[2]) < 1e-9, i
        return [grey(f, i) for i in shown]

    def expect(f, **kw):
        n = {**case(**FINE), **kw}
        for i in shown:
            p = point(i % W, i // W, n["size"], n["evolution"], n["speed"], 0)
            v = value(n["seed"], n["complexity"], n["contrast"], n["brightness"], p)
            assert abs(grey(f, i) - v) < 1e-9, i
        return True

    one = c["FX-FRACTAL-001"]
    assert one["0"] == one["4"]
    v1 = ramp(one["0"])
    assert max(v1) - min(v1) < 0.3 and all(0.1 < v < 0.9 for v in v1)
    assert one["0"][at(15, 4)][3] == 128 / 255
    two = c["FX-FRACTAL-002"]["0"]
    v2 = ramp(two)
    assert max(v2) - min(v2) > max(v1) - min(v1) and expect(two)
    three = c["FX-FRACTAL-003"]["0"]
    for i in shown:
        p = point(i % W, i // W, 4, 0, 0, 0)
        assert abs(grey(three, i) - (0.5 + 0.5 * V(0, 0, *p))) < 1e-9
    four = c["FX-FRACTAL-004"]["0"]
    assert expect(four, complexity=8) and four != two
    for i in shown:
        assert abs(grey(c["FX-FRACTAL-005"]["0"], i) - 0.5) < 1e-9
    six, seven = c["FX-FRACTAL-006"]["0"], c["FX-FRACTAL-007"]["0"]
    clipped = 0
    for i, v in zip(shown, v2):
        assert abs(grey(six, i) - min(1, max(0, 0.5 + 3 * (v - 0.5)))) < 1e-9
        assert abs(grey(seven, i) - min(1, v + 0.3)) < 1e-9
        clipped += grey(six, i) in (0.0, 1.0)
    assert clipped > 0
    assert all(c["FX-FRACTAL-008"]["0"][i][:3] == [0.0] * 3 for i in shown)
    nine = c["FX-FRACTAL-009"]
    assert nine["0"] == two and nine["4"] == c["FX-FRACTAL-010"]["0"] != two
    assert nine["2"] == render(case(evolution=180, **FINE), 0) and nine["2"] != nine["0"]
    assert nine["2"] != nine["4"]
    eleven = c["FX-FRACTAL-011"]
    assert eleven["0"] == two and eleven["2"] == c["FX-FRACTAL-010"]["0"]
    assert c["FX-FRACTAL-012"]["0"] != two and c["FX-FRACTAL-013"]["0"] == c["FX-FRACTAL-012"]["0"]
    fourteen = c["FX-FRACTAL-014"]["0"]
    lo, hi = [v / 255 for v in R.hex_color(DUSK)], [v / 255 for v in R.hex_color(CREAM.lower())]
    for i, v in zip(shown, v2):
        a = drawn[i][3]
        assert near(fourteen[i], [srgb_to_linear(d + v * (e - d)) * a for d, e in zip(lo, hi)]
                    + [a], 1e-9)
    assert c["FX-FRACTAL-015"]["0"] == fourteen
    sixteen, seventeen = c["FX-FRACTAL-016"]["0"], c["FX-FRACTAL-017"]["0"]
    assert all(sixteen[i][ch] <= drawn[i][ch] + 1e-12 for i in shown for ch in range(3))
    assert all(sixteen[i][:3] == [0.0] * 3 for i in black)
    assert all(seventeen[i][ch] >= drawn[i][ch] - 1e-12 for i in shown for ch in range(3))
    assert all(near(seventeen[i], drawn[i]) for i in white)
    eighteen = c["FX-FRACTAL-018"]["0"]
    assert all(eighteen[i][0] > 1 for i in white)
    assert all(eighteen[i][ch] >= drawn[i][ch] for i in shown for ch in range(3))
    nineteen = c["FX-FRACTAL-019"]
    assert nineteen["0"] == drawn and nineteen["2"] == nineteen["4"] == two
    moved = c["FX-FRACTAL-020"]
    assert moved["0"] == moved["3"]
    for y in range(H):
        assert moved["0"][at(3, y):at(0, y + 1)] == two[at(0, y):at(W - 3, y)]
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(v >= 0 for v in p[:3]), (name, p)
                if name != "FX-FRACTAL-018":
                    assert all(v <= p[3] + 1e-12 for v in p[:3]), (name, p)
    print("checked")


if __name__ == "__main__":
    main()
