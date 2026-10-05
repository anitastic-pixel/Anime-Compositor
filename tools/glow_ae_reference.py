"""D-322's Glow in After Effects' own numbers, worked a second way.

The owner asked on 2026-10-04 for After Effects' Glow numbers, checked against After Effects
examples on the internet, since they no longer have After Effects. The rule is from the Creative
COW thread "Glow Effect and transparent background mechanics"
(https://creativecow.net/forums/thread/glow-effect-and-transparent-background-mechanics/), where
After Effects' Glow was measured on white shapes:

- "Glow Radius pretty much creates layer's Gaussian blur and adds it to the layer. This is the
  only parameter which changes alpha of surrounding pixels." So the radius is a Gaussian Blur's
  Blurriness (D-321: sigma 0.3 times it, reaching 6.5 sigmas), and Glow Intensity leaves the
  glow's covering alone.
- The glow's brightness is GI * (GT / 100) + GI * 16 * (1 - GT / 100), GI being Glow Intensity
  and GT Glow Threshold; at GI 3, GT 0 the glow's colour reads about 48, with a low covering.

`core.glow` gains `units`: "classic", what a file without it means, D-89's rule exactly; or
"after_effects", which a Glow added from now on writes. In after_effects:

- what glows is as before (the threshold test, chosen colours, tint);
- it is spread by Gaussian Blur at Blurriness = radius: sigma 0.3 radius, reach ceil(6.5 sigma);
- its colour is multiplied by k = intensity * (t + 16 (1 - t)), t = threshold / 100, and its
  covering is not multiplied at all;
- Add and Screen lay it on as before (Add: colour not cut off, covering stopping at 1; Screen:
  each held inside 0 to 1 first). Intensity 0 still changes nothing.

Every case is glow_reference's 16 by 10 composition and drawings. The projects go into
`Fixtures/glow_ae`, the expected frames into `Fixtures/glow_ae/expected_glow_ae.json`.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, summing the two-dimensional kernel directly at each pixel.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/glow_ae_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
import glow_reference as G  # noqa: E402
import smooth_reference as S  # noqa: E402

W, H = G.W, G.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "glow_ae"
TOLERANCE = 2e-5  # document 25's default for a filter


# --- the rule -------------------------------------------------------------------------------

def strength(intensity, threshold):
    """Creative COW's measured brightness of the glow of white: GI*(GT/100) + GI*16*(1-GT/100)."""
    t = threshold / 100
    return intensity * (t + 16 * (1 - t))


def glow(pixels, c):
    """The glowing drawing, as the composition's W by H frame, in units after_effects."""
    threshold, radius, intensity = c["threshold"], c["radius"], c["intensity"]
    if intensity == 0:
        return [G.working(p) for p in pixels]
    source = []
    for p in pixels:
        if not G.glows(p, c, threshold, c["tolerance"]):
            source.append([0.0] * 4)
        elif c["tint"]:
            a = p[3] / 255
            source.append([srgb_to_linear(v / 255) * a for v in G.hex_color(c["tint"].lower())]
                          + [a])
        else:
            source.append(G.working(p))

    sigma = 0.3 * radius
    reach = math.ceil(6.5 * sigma) if sigma > 0 else 0
    one = [math.exp(-(d * d) / (2 * sigma * sigma)) for d in range(-reach, reach + 1)] \
        if reach else [1.0]
    total = sum(one)
    one = [v / total for v in one]
    k = strength(intensity, threshold)

    out = []
    for y in range(H):
        for x in range(W):
            g = [0.0] * 4
            for j in range(-reach, reach + 1):
                for i in range(-reach, reach + 1):
                    sx, sy = x + i, y + j
                    if 0 <= sx < W and 0 <= sy < H:
                        w = one[i + reach] * one[j + reach]
                        for ch in range(4):
                            g[ch] += source[sy * W + sx][ch] * w
            g = [v * k for v in g[:3]] + [g[3]]
            o = G.working(pixels[y * W + x])
            if c["operation"] == "add":
                out.append([o[ch] + g[ch] for ch in range(3)] + [min(1.0, o[3] + g[3])])
            else:
                g = [min(1.0, max(0.0, v)) for v in g]
                out.append([o[ch] + g[ch] - o[ch] * g[ch] for ch in range(4)])
    return out


# --- the cases ------------------------------------------------------------------------------

def case(units="after_effects", **settings):
    """`units` None: the file does not say. The rest are glow_reference's, its defaults being
    After Effects' own but for radius, which is After Effects' 10 here."""
    c = G.case(**{"radius": 10, **settings})
    c["units"] = units
    return c


def render(c):
    pixels = [p for row in G.DRAWINGS[c["drawing"]] for p in row]
    if c["units"] in (None, "classic"):
        return G.glow(pixels, c, 0, 0)
    return glow(pixels, c)


CASES = {
    "FX-GLOW-AE-001": ("Units after_effects with After Effects' defaults, threshold 60, radius "
                       "10, intensity 1, Add: spread as Gaussian Blur at Blurriness 10 (sigma 3), "
                       "the glow's colour seven times the blurred light (0.6 + 16 x 0.4), its "
                       "covering not multiplied.", case()),
    "FX-GLOW-AE-002": ("The same settings in a file without units: D-89's rule as before, "
                       "FX-GLOW-014 exactly.", case(units=None)),
    "FX-GLOW-AE-003": ("Units written \"classic\": FX-GLOW-AE-002 exactly.",
                       case(units="classic")),
    "FX-GLOW-AE-004": ("Tutorial 2's kind of setting, threshold 0, radius 4, intensity 0.1: every "
                       "pixel that shows glows, its colour 1.6 times the blurred light.",
                       case(threshold=0, radius=4, intensity=0.1)),
    "FX-GLOW-AE-005": ("Intensity 0: the drawing, untouched.", case(intensity=0)),
    "FX-GLOW-AE-006": ("Radius 0: nothing spreads, and each glowing pixel's colour is added onto "
                       "itself seven times.", case(radius=0)),
    "FX-GLOW-AE-007": ("Operation Screen: the strengthened glow is held inside 0 to 1 and "
                       "screened on.", case(operation="screen")),
    "FX-GLOW-AE-008": ("Tint #ff4000, threshold 0, intensity 0.2: the glow is orange, 3.2 times "
                       "the blurred tint.", case(threshold=0, intensity=0.2, tint="#ff4000")),
}

INVALID = {
    "FX-GLOW-AE-009": ("Units \"After_Effects\": the word is exact, so capitals are not it.",
                       case(units="After_Effects")),
    "FX-GLOW-AE-010": ("Units \"ae\", which is not one.", case(units="ae")),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = G.project_json(fx, c)
    if c["units"] is not None:
        p["compositions"][0]["layers"][0]["effects"][0]["parameters"]["units"] = c["units"]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in G.DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c) in CASES.items():
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": {"0": render(c)}}
        print(f"{fx}: {says[:60]}")
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        plain = [G.working(p) for row in G.DRAWINGS[c["drawing"]] for p in row]
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": plain, "4": plain},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")
    (OUT / "expected_glow_ae.json").write_text(json.dumps(expected, indent=1) + "\n",
                                               encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    drawn = [G.working(p) for row in G.DRAWINGS["patches"] for p in row]
    at = lambda x, y: y * W + x  # noqa: E731

    # Creative COW's two readings: GI 3, GT 0 is 48; After Effects' defaults are 7.
    assert strength(3, 0) == 48 and abs(strength(1, 60) - 7) < 1e-12
    # A file without units is D-89's rule, FX-GLOW-014 itself.
    old = json.loads((OUT.parent / "glow" / "expected_glow.json").read_text(encoding="utf-8"))
    assert c["FX-GLOW-AE-002"] == old["cases"]["FX-GLOW-014"]["frames"]["0"]
    assert c["FX-GLOW-AE-003"] == c["FX-GLOW-AE-002"]
    one = c["FX-GLOW-AE-001"]
    assert one != c["FX-GLOW-AE-002"]
    # Brighter than the classic glow on the yellow, past white; tighter round it (sigma 3 not
    # 3.33), yet still lit in the far corner, 11 pixels off, inside 6.5 sigmas.
    assert one[at(3, 4)][0] > c["FX-GLOW-AE-002"][at(3, 4)][0] + 1
    assert one[at(15, 0)][3] > 10 * TOLERANCE
    # In the empty space the covering is the blurred covering alone, whatever the intensity;
    # the colour, straight, is k times the light's straight colour, so can pass 1.
    four = c["FX-GLOW-AE-004"]
    assert four[at(0, 0)][3] > 0 and four[at(0, 0)][3] < 0.1
    assert c["FX-GLOW-AE-005"] == drawn
    # Radius 0: the yellow is itself plus seven times itself, its covering held at 1.
    six = c["FX-GLOW-AE-006"]
    y = drawn[at(3, 4)]
    assert all(abs(six[at(3, 4)][ch] - 8 * y[ch]) < 1e-12 for ch in range(3))
    assert six[at(3, 4)][3] == 1 and six[at(0, 0)] == [0.0] * 4
    for p in c["FX-GLOW-AE-007"]:
        assert all(0 <= v <= 1 for v in p)
    tint = [srgb_to_linear(v / 255) for v in (0xff, 0x40, 0x00)]
    p = c["FX-GLOW-AE-008"][at(0, 0)]
    assert all(abs(p[ch] / p[3] - 3.2 * tint[ch]) < 1e-9 for ch in range(3))
    for name, px in c.items():
        for p in px:
            assert 0 <= p[3] <= 1 and all(v >= 0 for v in p[:3]), (name, p)


if __name__ == "__main__":
    main()
