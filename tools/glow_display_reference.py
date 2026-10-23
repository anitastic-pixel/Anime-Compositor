"""D-337's Glow and Solid Composite on display values, worked a second way.

From P-26's tutorial 2 replay, under the owner's request of 2026-10-07 to build what the
tutorial uses and the app lacks. After Effects in 8 bpc, and in its own 32 bpc without a linear
working space (D-330, D-333), works on display values: the straight colour through the sRGB
curve. D-333 moved the blurs there; Glow and Solid Composite stayed in linear light. Tutorial 2
puts its glow layer on black with Solid Composite and glows it three times; worked in linear
light the black lifts the faint blue into a bright cyan pool, where the tutorial shows a dim
gold one, and the glow is dimmer than the tutorial's.

In a composition at 8 bpc or 32 bpc (After Effects), and only there:

- Solid Composite lays the layer on its solid in display values: the layer is taken to display
  values (straight colour held to 0..top, top 1 at 8 bpc and none at 32 bpc, through the sRGB
  curve, premultiplied), the solid's colour is used as written, without the sRGB curve, and
  the result is taken back the same way.
- Glow works in display values too. What glows is tested as before, on the pixel itself; its
  light is the pixel's display value, or the tint as written at the pixel's covering; it is
  spread as before; the light and the drawing are laid together in display values and taken
  back. Classic units multiply both by the intensity, as before. D-331's covering divided by c
  stays for Float, where it matched.
- D-746 (audit row 12, Wave 1's eighteen tutorial cases) replaced D-337's After Effects units
  rule, the colour multiplied by GI * (t + 16 (1 - t)), which lit the edge 2 to 10 times too
  bright. In units after_effects now: based on bright, each of red, green and blue glows on its
  own when its 8-bit value is at least the threshold's share of 255 (the blue of a blue square
  glows, its red and green do not); each glowing channel's light is its display value times
  GI * c / 3, c = t + 16 (1 - t) as before, held at 1, before the spread (a third of Creative
  COW's strength, fitted on Valeri Visuals' disc and Motion Nations's cases, so 2.33 times the
  intensity at After Effects' default threshold 60); chosen colours and the tint glow as before,
  the light held at 1 the same way; the spread is a Gaussian at sigma 0.4 times the radius
  (Valeri Visuals' white disc at radius 400 measures sigma 161); and the blurred light is laid
  on at strength 1, the covering as blurred.
- At 8 bpc the result is then held to 8 bits, as D-330.

Every case is glow_reference's 16 by 10 composition and drawings. The projects go into
`Fixtures/glow_display`, the expected frames into `Fixtures/glow_display/expected_glow_display.json`.

**This file never runs the build's code path.** It works in double precision on lists, from the
drawing's 8-bit values, summing the two-dimensional kernel directly at each pixel.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/glow_display_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
import eight_bpc_reference as E  # noqa: E402
import glow_ae_reference as GA  # noqa: E402
import glow_reference as G  # noqa: E402
import smooth_reference as S  # noqa: E402

W, H = G.W, G.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "glow_display"
TOLERANCE = 2e-5  # document 25's default for a filter


# --- the rule -------------------------------------------------------------------------------

def display(p, top):
    """Linear premultiplied to display premultiplied, the straight colour held to 0..top."""
    a = p[3]
    if a <= 0:
        return [0.0] * 4
    return [S.linear_to_srgb(min(max(p[i] / a, 0.0), top)) * a for i in range(3)] + [a]


def linear(e, top):
    a = e[3]
    if a <= 0:
        return [0.0] * 4
    return [srgb_to_linear(min(max(e[i] / a, 0.0), top)) * a for i in range(3)] + [a]


def eight_bit(p):
    """The pixel as the glow test reads it: its straight colour in 8-bit steps."""
    a = p[3]
    if a <= 0:
        return (0, 0, 0, 0)
    return tuple(round(S.linear_to_srgb(min(max(p[i] / a, 0.0), 1.0)) * 255) for i in range(3)) + (a,)


def glow(px, c, top):
    """The Glow of the layer `px` (linear premultiplied, W by H) in display values. What glows is
    tested on the pixel itself, as before."""
    if c["intensity"] == 0:
        return px
    threshold, radius, intensity = c["threshold"], c["radius"], c["intensity"]
    ae = c["units"] == "after_effects"
    # D-746: in After Effects units each channel's light is held at 1 after the intensity.
    s = intensity * GA.strength(1, threshold) / 3
    held = (lambda v: min(1.0, s * v)) if ae else (lambda v: v)
    source = []
    for p in px:
        q, a = eight_bit(p), p[3]
        if ae and c["based_on"] == "bright" and not c["tint"]:
            # D-746: each channel glows on its own.
            on = [a > 0 and q[i] * 100 >= threshold * 255 for i in range(3)]
            e = display(p, top)
            source.append([held(e[i] / a) * a if on[i] else 0.0 for i in range(3)]
                          + [a if any(on) else 0.0])
        elif not G.glows(q, c, threshold, c["tolerance"]):
            source.append([0.0] * 4)
        elif c["tint"]:
            source.append([held(v / 255) * a for v in G.hex_color(c["tint"].lower())] + [a])
        else:
            e = display(p, top)
            source.append([held(e[i] / a) * a for i in range(3)] + [a])

    sigma = 0.4 * radius if ae else radius / 3
    reach = math.ceil((6.5 if ae else 3) * sigma) if sigma > 0 else 0
    one = [math.exp(-(d * d) / (2 * sigma * sigma)) for d in range(-reach, reach + 1)] \
        if reach else [1.0]
    total = sum(one)
    one = [v / total for v in one]
    k, ka = (1.0, 1.0) if ae else (intensity, intensity)

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
            g = [v * k for v in g[:3]] + [g[3] * ka]
            o = display(px[y * W + x], top)
            if c["operation"] == "add":
                r = [o[ch] + g[ch] for ch in range(3)] + [min(1.0, o[3] + g[3])]
            else:
                g = [min(1.0, max(0.0, v)) for v in g]
                r = [o[ch] + g[ch] - o[ch] * g[ch] for ch in range(4)]
            out.append(linear(r, top))
    return out


def solid(px, color, opacity, top):
    """Solid Composite, Normal, source opacity 100: the layer over the solid, in display values,
    the solid's colour as written."""
    o = opacity / 100
    s = [v / 255 * o for v in G.hex_color(color)] + [o]
    out = []
    for p in px:
        e = display(p, top)
        out.append(linear([e[ch] + s[ch] * (1 - e[3]) for ch in range(4)], top))
    return out


# --- the cases ------------------------------------------------------------------------------

def case(depth="ae32", effects=(), drawing="patches"):
    """`depth`: "ae32", "eight" or "float". `effects`: a list of ("glow", settings) |
    ("solid", (colour, opacity))."""
    return {"depth": depth, "effects": list(effects), "drawing": drawing}


def glow_settings(units="after_effects", **settings):
    return GA.case(units=units, **settings)


def render(c):
    pixels = [p for row in G.DRAWINGS[c["drawing"]] for p in row]
    px = [G.working(p) for p in pixels]
    top = 1.0 if c["depth"] == "eight" else math.inf
    for kind, s in c["effects"]:
        if c["depth"] == "float":
            assert kind == "glow" and s["units"] == "after_effects"
            px = GA.glow(pixels, s)
        elif kind == "glow":
            px = glow(px, s, top)
        else:
            px = solid(px, s[0], s[1], top)
        if c["depth"] == "eight":
            px = E.eight(px)
    return px


TUTORIAL = glow_settings(threshold=0, radius=4, intensity=0.1)
BLACK = ("solid", ("#000000", 100))

CASES = {
    "FX-GLDISP-001": ("32 bpc (After Effects), Glow in After Effects units at tutorial 2's kind "
                      "of setting, threshold 0, radius 4, intensity 0.1: worked in display "
                      "values, D-746: each channel's light 0.1 x 16 / 3 times its display value, spread "
                      "at sigma 1.6, laid on at strength 1 with the covering as blurred, so the "
                      "halo covers more than Float's.",
                      case(effects=[("glow", TUTORIAL)])),
    "FX-GLDISP-002": ("32 bpc (After Effects), tutorial 2's glow layer in small: Solid Composite "
                      "on black, then the same Glow. Opaque, the glow lifts the black round the "
                      "patches with their own colours.",
                      case(effects=[BLACK, ("glow", TUTORIAL)])),
    "FX-GLDISP-003": ("32 bpc (After Effects), Glow with tint #ff4000, threshold 0, intensity "
                      "0.2: the tint is used as written, a display value, so read straight in "
                      "display values the halo is #ff4000 times 0.2 x 16 / 3, held at 1 "
                      "(D-746).",
                      case(effects=[("glow", glow_settings(threshold=0, intensity=0.2,
                                                           tint="#ff4000"))])),
    "FX-GLDISP-004": ("32 bpc (After Effects), Glow in classic units, FX-GLOW-001's settings: "
                      "D-89's strength, worked in display values.",
                      case(effects=[("glow", glow_settings(units="classic", radius=4))])),
    "FX-GLDISP-005": ("32 bpc (After Effects), Glow in After Effects units with Screen: held "
                      "inside 0 to 1 and screened, in display values.",
                      case(effects=[("glow", glow_settings(operation="screen"))])),
    "FX-GLDISP-006": ("8 bpc, Glow in After Effects units at After Effects' defaults, threshold "
                      "60, radius 10, intensity 1: in display values by D-746's rule, then held "
                      "to 8 bits, so "
                      "nothing passes white.",
                      case("eight", [("glow", glow_settings())])),
    "FX-GLDISP-007": ("Float, FX-GLDISP-001's Glow: linear light and D-331's rule as before, "
                      "FX-GLOW-AE-004 exactly.", case("float", [("glow", TUTORIAL)])),
    "FX-GLDISP-008": ("32 bpc (After Effects), the half-covering patches with Solid Composite "
                      "#3c286e at opacity 50 and nothing else: laid on in display values, the "
                      "purple used as written.",
                      case(effects=[("solid", ("#3c286e", 50))], drawing="faint")),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c):
    p = G.project_json(fx, G.case(c["drawing"]))
    effects = []
    for i, (kind, s) in enumerate(c["effects"]):
        if kind == "glow":
            e = GA.project_json(fx, s)["compositions"][0]["layers"][0]["effects"][0]
        else:
            e = {"type_id": "core.solid_composite", "enabled": True,
                 "parameters": {"source_opacity": 100, "color": s[0], "opacity": s[1],
                                "blend": "normal"}}
        e["instance_id"] = f"fx-0-{i}"
        effects.append(e)
    comp = p["compositions"][0]
    comp["layers"][0]["effects"] = effects
    if c["depth"] == "eight":
        comp["eight_bpc"] = True
    elif c["depth"] == "ae32":
        comp["float_depth"] = True
        comp["ae_32bpc"] = True
    else:
        comp["float_depth"] = True
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
    (OUT / "expected_glow_display.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                    encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"]["0"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    lum = lambda p: 0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2]  # noqa: E731

    # Float is D-331's rule, FX-GLOW-AE-004 itself.
    old = json.loads((OUT.parent / "glow_ae" / "expected_glow_ae.json").read_text(encoding="utf-8"))
    assert c["FX-GLDISP-007"] == old["cases"]["FX-GLOW-AE-004"]["frames"]["0"]
    # In display values the halo in the empty space covers more (D-746: the covering as blurred).
    one, flt = c["FX-GLDISP-001"], c["FX-GLDISP-007"]
    for x, y in ((0, 0), (6, 1), (15, 8)):
        assert one[at(x, y)][3] > 4 * flt[at(x, y)][3], (x, y, one[at(x, y)], flt[at(x, y)])
    # On black the layer is opaque everywhere, and the black round the patches is lit.
    two = c["FX-GLDISP-002"]
    assert all(p[3] == 1 for p in two) and lum(two[at(6, 1)]) > 0
    # The tint as written: straight, in display values, #ff4000 times 0.2 x 16 / 3 held at 1 in
    # the far corner (D-746, laid on at strength 1).
    p = c["FX-GLDISP-003"][at(15, 0)]
    e = display(p, math.inf)
    assert all(abs(e[ch] / e[3] - min(1, 0.2 * 16 / 3 * v / 255)) < 1e-9
               for ch, v in enumerate((0xff, 0x40, 0)))
    # Screen holds the glow inside 0 to 1, so the covering never passes 1 and the drawing is
    # never darkened.
    plain = [G.working(p) for row in G.DRAWINGS["patches"] for p in row]
    for p, q in zip(c["FX-GLDISP-005"], plain):
        assert p[3] <= 1 and all(p[ch] >= q[ch] - 1e-9 for ch in range(4))
    # D-746: each channel glows on its own. At threshold 60 no patch's blue reaches 153 (the
    # yellow's is 120, the brown's 51), so the halo in the empty space has red and no blue.
    p = c["FX-GLDISP-006"][at(6, 1)]
    assert p[0] > 0 and p[2] == 0, p
    # 8 bpc: nothing past white, every value on an 8-bit step.
    for p in c["FX-GLDISP-006"]:
        if p[3] > 0:
            assert all(p[ch] / p[3] <= 1 + 1e-12 for ch in range(3))
            assert abs(p[3] * 255 - round(p[3] * 255)) < 1e-9
    # The purple, laid under the half-covering patches in display values: on the empty half,
    # the purple at half covering, as written.
    p = display(c["FX-GLDISP-008"][at(0, 0)], math.inf)
    assert abs(p[3] - 0.5) < 1e-12 and all(
        abs(p[ch] - v / 255 * 0.5) < 1e-12 for ch, v in enumerate((0x3c, 0x28, 0x6e)))
    for name, px in c.items():
        for p in px:
            assert 0 <= p[3] <= 1 and all(v >= 0 for v in p[:3]), (name, p)


if __name__ == "__main__":
    main()
