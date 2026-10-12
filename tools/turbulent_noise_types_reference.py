"""Turbulent Noise's full type lists, worked a second way.

D-567 gives Turbulent Noise (D-445) and Fractal Noise (D-128, D-299), one engine by D-383 and
D-394, After Effects' whole Fractal Type and Noise Type lists. Adobe publishes no formulas for
them, only their names and how they look, so each is this program's own rule, written to look
the way its name says; nothing is ported. The two types already built keep their numbers to the
bit: "basic", and "turbulent" (After Effects' Turbulent Basic), with "smooth" (this program's
own, quintic) and "block". Document 21 is the rule in words; this file is the reference for the
numbers document 25 pins against it.

The noise. V(seed, ch, x, y, z, noise) in -1..1, from U Noise's hash (D-119) at whole points,
with i, j, k = floor(x), floor(y), floor(z) and tx, ty, tz the parts past them:
- smooth and block are D-128's and D-299's (`tools/turbulent_noise_reference.py`);
- linear mixes the eight corners by the parts themselves, straight lines between the points;
- soft_linear by 3t^2 - 2t^3, softer at the points than linear;
- spline mixes four points across and four down by the Catmull-Rom curve, a smooth wave through
  every point, and the depth by smooth's curve; it is held to -1..1, as the wave can overshoot.

The fractal. Octave o of n is V(seed, 8 o, ...) at strength a = 0.5^o (0.35^o for cloudy) and
fineness 2^o (4 2^o for small_bumps), sampled at (x 2^o, y 2^o, z 2^o) moved as the type says.
"So far" is the sum of the weighted octaves before this one over the sum of their strengths (0
before the first). Each octave's value v is weighed as the type says, and the sum over the sum
of the strengths, B, is the result unless the type says what to make of it:
- basic: v; the result B;
- turbulent: |v|, the result 2B - 1 (creases where v crosses 0);
- turbulent_smooth: v^2, the result 2B - 1 (rounded creases);
- turbulent_sharp: sqrt|v|, the result 2B - 1 (thin, sharp creases);
- dynamic: each octave moved 4 times so far across and 4 times the previous octave's v down
  (0 before the first), in its own cells;
- dynamic_progressive: moved 2 o times so far across and 2 o times the previous v down,
  further each octave;
- dynamic_twist: each octave after the first moved two of its cells in the direction 2 pi so far;
- max: the result r starts at the first octave's v, and each octave after lifts it by its
  strength times how far its v is above r (never lowers it): light clouds;
- smeary: each octave after the first stretched four times across and moved twice so far across;
- swirly: the result sin(3 pi B), bands that follow the clouds;
- rocky: the result B to the nearest half, held to -1..1, flat patches;
- cloudy: strengths 0.35^o, the result (B + 1)^2 / 2 - 1, soft light puffs on dark;
- terrain: (1 - |v|)^2 times the previous octave's (1 - |v|)^2 (1 for the first), the result
  2B - 1, ridges;
- subscale: each octave after the first weighed by (the previous octave's v + 1) / 2, finer
  detail only where the coarser is light;
- small_bumps: v, starting two octaves finer;
- strings: the result 2 max(0, 1 - 6 |B|) - 1, thin lines where the clouds cross the middle;
- threads: 2 max(0, 1 - 6 |v|) - 1 in each octave, threads at every scale.

The rule is Turbulent Noise's (`tools/turbulent_noise_reference.py`) with this field.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: Noise's card. The drawing goes into `Fixtures/turbulent_noise_types/media`, the
projects into `Fixtures/turbulent_noise_types`, and the expected frames into
`Fixtures/turbulent_noise_types/expected_turbulent_noise_types.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/turbulent_noise_types_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
from noise_reference import u as U  # noqa: E402
from gradient_reference import BLENDS  # noqa: E402
import fractal_noise_reference as FN  # noqa: E402
import turbulent_noise_reference as T  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "turbulent_noise_types"
TOLERANCE = 2e-5  # document 25's default for a filter

FRACTAL_TYPES = ("basic", "turbulent", "turbulent_smooth", "turbulent_sharp", "dynamic",
                 "dynamic_progressive", "dynamic_twist", "max", "smeary", "swirly", "rocky",
                 "cloudy", "terrain", "subscale", "small_bumps", "strings", "threads")
NOISE_TYPES = ("smooth", "block", "linear", "soft_linear", "spline")


# --- the noise ------------------------------------------------------------------------------

def catmull(t):
    """The Catmull-Rom weights of the points at -1, 0, 1 and 2 for a place t past 0."""
    return ((-t ** 3 + 2 * t * t - t) / 2, (3 * t ** 3 - 5 * t * t + 2) / 2,
            (-3 * t ** 3 + 4 * t * t + t) / 2, (t ** 3 - t * t) / 2)


CURVES = {"linear": lambda t: t, "soft_linear": lambda t: t * t * (3 - 2 * t)}


def V(seed, ch, x, y, z, noise):
    if noise in ("smooth", "block"):
        return T.V(seed, ch, x, y, z, noise == "block")
    i, j, k = math.floor(x), math.floor(y), math.floor(z)
    if noise == "spline":
        wx, wy, sz = catmull(x - i), catmull(y - j), FN.fade(z - k)
        v = 0.0
        for dk, wz in ((0, 1 - sz), (1, sz)):
            for b in range(4):
                for a in range(4):
                    v += wx[a] * wy[b] * wz * U(seed, i + a - 1, j + b - 1, k + dk, ch)
        return min(1.0, max(-1.0, v))
    c = CURVES[noise]
    sx, sy, sz = c(x - i), c(y - j), c(z - k)
    v = 0.0
    for dk, wz in ((0, 1 - sz), (1, sz)):
        for dj, wy in ((0, 1 - sy), (1, sy)):
            for di, wx in ((0, 1 - sx), (1, sx)):
                v += wx * wy * wz * U(seed, i + di, j + dj, k + dk, ch)
    return v


# --- the fractal ----------------------------------------------------------------------------

def F(seed, x, y, z, n, kind, noise):
    if kind in ("basic", "turbulent") and noise in ("smooth", "block"):
        return T.F(seed, x, y, z, n, kind == "turbulent", noise == "block")
    step = 0.35 if kind == "cloudy" else 0.5
    fine = 4.0 if kind == "small_bumps" else 1.0
    total = weight = 0.0
    amp, so_far, prev, prev_ridge, best = 1.0, 0.0, 0.0, 1.0, -math.inf
    for o in range(n):
        px, py, pz = x * fine, y * fine, z * fine
        if kind == "dynamic":
            px, py = px + 4 * so_far, py + 4 * prev
        elif kind == "dynamic_progressive":
            px, py = px + 2 * o * so_far, py + 2 * o * prev
        elif kind == "dynamic_twist" and o > 0:
            px, py = px + 2 * math.cos(2 * math.pi * so_far), py + 2 * math.sin(2 * math.pi * so_far)
        elif kind == "smeary" and o > 0:
            px = px / 4 + 2 * so_far
        v = V(seed, 8 * o, px, py, pz, noise)
        ridge = (1 - abs(v)) ** 2
        g = {"turbulent": abs(v), "turbulent_smooth": v * v, "turbulent_sharp": math.sqrt(abs(v)),
             "terrain": ridge * prev_ridge, "threads": 2 * max(0.0, 1 - 6 * abs(v)) - 1}.get(kind, v)
        if kind == "subscale" and o > 0:
            g *= (prev + 1) / 2
        total += amp * g
        weight += amp
        so_far = total / weight
        best = v if o == 0 else best + amp * max(0.0, v - best)
        prev, prev_ridge = v, ridge
        amp *= step
        fine *= 2
    b = total / weight
    if kind in ("turbulent", "turbulent_smooth", "turbulent_sharp", "terrain"):
        return 2 * b - 1
    if kind == "max":
        return best
    if kind == "swirly":
        return math.sin(3 * math.pi * b)
    if kind == "rocky":
        return min(1.0, max(-1.0, math.floor(2 * b + 0.5) / 2))
    if kind == "cloudy":
        return (b + 1) ** 2 / 2 - 1
    if kind == "strings":
        return 2 * max(0.0, 1 - 6 * abs(b)) - 1
    return b


# --- the rule -------------------------------------------------------------------------------

def value(c, n, x, y):
    f = F(n["seed"], *T.point(x, y, n, c["offset"]), n["complexity"], c["fractal_type"],
          c["noise_type"])
    if c["invert"] == "on":
        f = -f
    return min(1.0, max(0.0, 0.5 + 0.5 * f * n["contrast"] / 100 + n["brightness"] / 100))


def turbulent_noise(pixels, c, n, width=W):
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


def render(c, frame_no):
    pixels = [p for row in T.DRAWINGS[c["drawing"]] for p in row]
    n = {k: T.held(c, k, frame_no) for k in T.RANGES}
    return R.frame(turbulent_noise(pixels, c, n), c["shift"])


# --- the cases ------------------------------------------------------------------------------

case = T.case
FINE = T.FINE

LOOKS = {
    "turbulent_smooth": "rounded creases, softer than turbulent's",
    "turbulent_sharp": "thin, sharp creases",
    "dynamic": "each octave pushed by the clouds so far, the clouds drawn out",
    "dynamic_progressive": "each octave pushed further than the last",
    "dynamic_twist": "each octave pushed in a direction the clouds turn, like smoke",
    "max": "each octave only ever lightening the clouds, light clouds",
    "smeary": "the finer octaves streaked across",
    "swirly": "bands that follow the clouds, like agate",
    "rocky": "flat patches in five greys, like camouflage",
    "cloudy": "soft light puffs on dark",
    "terrain": "light ridges",
    "subscale": "the fine detail only where the coarser clouds are light",
    "small_bumps": "small bumps, starting two octaves finer",
    "strings": "thin light lines where the clouds cross the middle grey",
    "threads": "thin threads at every scale",
}
CASES = {}
for k, (kind, says) in enumerate(LOOKS.items(), start=1):
    CASES[f"FX-TURBTYPE-{k:03}"] = (f"Size 4, fractal type {kind}: {says}.",
                                    case(fractal_type=kind, **FINE), [0])
CASES.update({
    "FX-TURBTYPE-016": ("Size 4, noise type linear: straight lines between the points, a "
                        "harder look than smooth.", case(noise_type="linear", **FINE), [0]),
    "FX-TURBTYPE-017": ("Size 4, noise type soft_linear: softer at the points than linear.",
                        case(noise_type="soft_linear", **FINE), [0]),
    "FX-TURBTYPE-018": ("Size 4, noise type spline: a smooth wave through every point.",
                        case(noise_type="spline", **FINE), [0]),
    "FX-TURBTYPE-019": ("Size 4, dynamic_twist on spline, inverted, evolution keyed from 0 at "
                        "frame 0 to 720 at frame 4, linear: the smoke moves as it evolves.",
                        case(fractal_type="dynamic_twist", noise_type="spline", invert="on",
                             evolution=keyed((0, 0), (4, 720)), **FINE), [0, 2, 4]),
    "FX-TURBTYPE-020": ("Size 4, rocky on block, contrast 200: square flat patches.",
                        case(fractal_type="rocky", noise_type="block", contrast=200, **FINE),
                        [0]),
    "FX-TURBTYPE-021": ("Size 4, terrain on linear at complexity 20, the most.",
                        case(fractal_type="terrain", noise_type="linear", complexity=20,
                             **FINE), [0]),
    "FX-TURBTYPE-022": ("Size 4, swirly at complexity 1.9, which counts as 1: sin(3 pi v) of "
                        "the one octave.", case(fractal_type="swirly", complexity=1.9, **FINE),
                        [0]),
    "FX-TURBTYPE-023": ("FX-TURBTYPE-006 (max) moved three pixels right: worked in the "
                        "drawing's own space, so it moves with it.",
                        case(fractal_type="max", shift=3, **FINE), [0, 3]),
})

INVALID = {
    "FX-TURBTYPE-024": ("Fractal type \"Turbulent Basic\", the name shown and not the word "
                        "written.", case(fractal_type="Turbulent Basic")),
    "FX-TURBTYPE-025": ("Noise type \"soft linear\", with a space and not the underscore.",
                        case(noise_type="soft linear")),
}


# --- the project files ----------------------------------------------------------------------

def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_bytes((json.dumps(T.project_json(fx, c), indent=2) + "\n").encode())
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in T.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        before = T.plain(c)
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != before[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = T.plain(c)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_turbulent_noise_types.json").write_bytes(
        (json.dumps(expected, indent=1) + "\n").encode())
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    two = T.render(case(**FINE), 0)
    drawn = T.plain(case())
    shown = [i for i in range(W * H) if drawn[i][3] > 0]
    enc = lambda p: [S.linear_to_srgb(min(1.0, max(0.0, v / p[3]))) for v in p[:3]]  # noqa: E731
    grey = lambda f, i: enc(f[i])[0]  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731

    # The two types already built are D-445's to the bit, through this file's field.
    for kind in ("basic", "turbulent"):
        for noise in ("smooth", "block"):
            assert F(3, 1.3, 2.7, 0.2, 4, kind, noise) == T.F(3, 1.3, 2.7, 0.2, 4, kind == "turbulent",
                                                            noise == "block")
    # The noises meet the hash at whole points, and stay in -1..1.
    for noise in NOISE_TYPES:
        assert abs(V(5, 0, 3, -2, 1, noise) - U(5, 3, -2, 1, 0)) < 1e-12, noise
        assert all(-1 <= V(2, 8, x / 5, y / 7, 0.4, noise) <= 1 for x in range(30) for y in range(30))
    # Linear is straight between two points; soft_linear's middle is linear's.
    a, b = U(1, 0, 0, 0, 0), U(1, 1, 0, 0, 0)
    assert abs(V(1, 0, 0.25, 0, 0, "linear") - (0.75 * a + 0.25 * b)) < 1e-12
    assert abs(V(1, 0, 0.5, 0, 0, "soft_linear") - V(1, 0, 0.5, 0, 0, "linear")) < 1e-12
    # Every type's field is in -1..1, and with one octave the dynamic ones are basic.
    for kind in FRACTAL_TYPES:
        for noise in NOISE_TYPES:
            vals = [F(4, x / 3, y / 3, 0.7, 5, kind, noise) for x in range(12) for y in range(12)]
            assert all(-1 - 1e-12 <= v <= 1 + 1e-12 for v in vals), (kind, noise)
    for kind in ("dynamic", "dynamic_progressive", "dynamic_twist", "smeary", "subscale"):
        assert F(4, 1.3, 0.4, 0.2, 1, kind, "smooth") == F(4, 1.3, 0.4, 0.2, 1, "basic", "smooth")
    assert abs(F(4, 1.3, 0.4, 0.2, 1, "small_bumps", "smooth") - FN.V(4, 0, 5.2, 1.6, 0.8)) < 1e-12

    # Every new type and noise draws its own clouds, and none is FX-TURBNOISE-002.
    looks = [str(c[f"FX-TURBTYPE-{k:03}"]["0"]) for k in range(1, 19)]
    assert len(set(looks + [str(two)])) == 19
    # Every case keeps every covering and leaves the empty pixels empty.
    for fx, frames in c.items():
        base = T.plain(case(shift=3 if fx == "FX-TURBTYPE-023" else 0))
        for px in frames.values():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
                assert all(0 <= v <= px[i][3] + 1e-12 for v in px[i][:3]), (fx, i)
    # Rocky takes five greys at most (contrast 100): 0, 0.25, 0.5, 0.75, 1.
    rocky = {round(grey(c["FX-TURBTYPE-009"]["0"], i), 9) for i in shown}
    assert rocky <= {0.0, 0.25, 0.5, 0.75, 1.0} and len(rocky) > 1
    # Swirly with one octave is sin(3 pi v) of the plain value noise.
    for i in shown:
        p = T.point(i % W, i // W, case(**FINE), (0, 0))
        want = 0.5 + 0.5 * math.sin(3 * math.pi * FN.V(0, 0, *p))
        assert abs(grey(c["FX-TURBTYPE-022"]["0"], i) - want) < 1e-9
    nineteen = c["FX-TURBTYPE-019"]
    assert nineteen["0"] != nineteen["2"] != nineteen["4"]
    moved = c["FX-TURBTYPE-023"]
    assert moved["0"] == moved["3"]
    six = c["FX-TURBTYPE-006"]["0"]
    for y in range(H):
        assert moved["0"][at(3, y):at(0, y + 1)] == six[at(0, y):at(W - 3, y)]
    print("checked")


if __name__ == "__main__":
    main()
