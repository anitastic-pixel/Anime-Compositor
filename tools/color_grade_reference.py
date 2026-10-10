"""Color Grade, worked a second way: Basic Correction, Creative and Vignette (D-397).

`core.color_grade`, modelled on Premiere Pro's and After Effects' Lumetri Color panel, under our
own name: one effect holding a whole grade, section after section. This file is the first unit,
D-397: Basic Correction (white balance, exposure, the tone sliders, saturation), Creative (a
look file through Color Lookup's reading, its intensity, faded film, vibrance, saturation, and
shadow and highlight tints) and Vignette. Curves, Color Wheels and HSL Secondary follow as their
own unit. Adobe does not publish Lumetri's maths, so every rule below is ours, built where it
can be from rules this program already has (named at each step); document 14 records it as a
deviation. Nothing is ported, and none of Adobe's bundled looks is copied. Document 21 is the
rule in words; this file is the reference for the numbers document 25 pins against it.

The settings, each keyable but the look, with where they start:

    temperature, tint            -100 to 100, 0       white balance
    exposure                     -5 to 5, 0           stops
    contrast, highlights,
    shadows, whites, blacks      -100 to 100, 0
    saturation                   0 to 200, 100
    look                         a lut asset's id, or "" (none); "" when added
    look_intensity               0 to 200, 100
    faded_film                   0 to 100, 0
    vibrance                     -100 to 100, 0
    creative_saturation          0 to 200, 100
    shadow_tint_hue,
    highlight_tint_hue           0 to 360, 0          degrees
    shadow_tint_amount,
    highlight_tint_amount        0 to 100, 0
    tint_balance                 -100 to 100, 0
    vignette_amount              -5 to 5, 0
    vignette_midpoint            0 to 100, 50
    vignette_roundness           0 to 100, 0
    vignette_feather             0 to 100, 50

The rule. At a pixel with covering a > 0, e is its straight colour through the sRGB curve held
inside 0 to 1 (document 21's shared colour rule). The steps run in Lumetri's order, each one
that changes anything a step of its own: its result held inside 0 to 1 and back to linear at the
pixel's covering before the next step reads it again, as the separate effects it reuses are.
L(c) = 0.2126 c_r + 0.7152 c_g + 0.0722 c_b on encoded values.

1. Basic tone, when any of temperature, tint, exposure, contrast, highlights, shadows, whites or
   blacks is not 0. With t, n the temperature and tint over 100, the white balance's gains are
   (2^(t/2), 2^(-n/2), 2^(-t/2)) for red, green and blue, each divided by their L so white's
   brightness stays, then times 2^exposure. When a gain is not 1, each channel is taken to
   linear light, multiplied by its gain, held inside 0 to 1 and encoded again. Then on each
   channel v, with c, h, s, w, b the contrast, highlights, shadows, whites and blacks over 100,
   one after another:
       v += c v (1 - v) (2v - 1)      toward (or away from) the smooth S-curve 3v^2 - 2v^3
       v += h v^2 (1 - v)             most at v = 2/3
       v += s v (1 - v)^2             most at v = 1/3
       v += w v^2 / 4                 moves white, holds black
       v += b (1 - v)^2 / 4           moves black, holds white
   each term never reversing the order of two values for settings in range.
2. Basic saturation, when saturation is not 100: Vibrance's rule (D-142) with vibrance 0 and
   saturation `saturation - 100`: L + (e - L)(saturation / 100).
3. The look, when `look` names a lut asset whose file reads and the intensity is not 0: Color
   Lookup's reading (D-182) gives l, and the step gives e + I (l - e), I the intensity over 100;
   past 100 it pushes further the same way.
4. Faded film, when not 0: v + f (0.25 (1 - v)^2 - 0.1 v^2) on each channel, f over 100, black
   lifted to a quarter of f and white lowered by a tenth of it (the shape is ours).
5. Vibrance and creative saturation, when either changes anything: Vibrance's rule as it is.
6. Tints, when either amount is not 0: Color Balance's rule (D-127) with luminosity not kept,
   the midtones untouched; the shadows' push is A_s (1 - B) (C_s - L(C_s)) and the highlights'
   A_h (1 + B) (C_h - L(C_h)), where A is the amount, B the balance over 100 and C the hue's
   pure colour (HSL saturation 1, lightness 1/2), its brightness taken out so a tint colours and
   does not brighten.
7. Vignette, when the amount is not 0: Vignette's rule (D-126) with amount |vignette_amount| x
   20 per cent, the colour black for a negative amount and white for a positive one, size
   50 + midpoint, roundness as it is, softness the feather, centred on the layer.

A pixel with a = 0 is left as it is; the covering is never changed; with every setting as added
the layer is left exactly as it is. A `look` naming nothing that is a lut asset of the project
draws the rest of the grade with a warning (EFFECT_PARAMETER_INVALID), as Color Lookup warns.

**This file never runs the build's code path.** It works in double precision on lists,
straight from the drawing's 8-bit values, where the build works on its single-precision buffers.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py): pure colours, greys,
a skin tone, orange and three warm tones, darker row by row. The look file, `luts/look.cube`, is
this file's own: a gentle S-curve, a little less colour, the shadows cooled and the highlights
warmed. The projects go into `Fixtures/color_grade`, the expected frames into
`Fixtures/color_grade/expected_color_grade.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/color_grade_reference.py
"""

import colorsys
import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
from cube_lut_reference import read_cube, lookup, table_text  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "color_grade"
SETTINGS = {  # name: (low, high, as added)
    "temperature": (-100, 100, 0), "tint": (-100, 100, 0), "exposure": (-5, 5, 0),
    "contrast": (-100, 100, 0), "highlights": (-100, 100, 0), "shadows": (-100, 100, 0),
    "whites": (-100, 100, 0), "blacks": (-100, 100, 0), "saturation": (0, 200, 100),
    "look_intensity": (0, 200, 100), "faded_film": (0, 100, 0), "vibrance": (-100, 100, 0),
    "creative_saturation": (0, 200, 100), "shadow_tint_hue": (0, 360, 0),
    "shadow_tint_amount": (0, 100, 0), "highlight_tint_hue": (0, 360, 0),
    "highlight_tint_amount": (0, 100, 0), "tint_balance": (-100, 100, 0),
    "vignette_amount": (-5, 5, 0), "vignette_midpoint": (0, 100, 50),
    "vignette_roundness": (0, 100, 0), "vignette_feather": (0, 100, 50),
}


# --- the look file --------------------------------------------------------------------------

def look_colour(r, g, b):
    """This file's own look: a gentle S-curve, 80 per cent of the colour, the shadows cooled and
    the highlights warmed."""
    s = lambda x: 0.7 * x + 0.3 * x * x * (3 - 2 * x)  # noqa: E731
    r, g, b = s(r), s(g), s(b)
    lum = 0.2126 * r + 0.7152 * g + 0.0722 * b
    r, g, b = (lum + 0.8 * (c - lum) for c in (r, g, b))
    return [r + 0.06 * lum * lum, g + 0.01 * lum, b + 0.08 * (1 - lum) ** 2 - 0.03 * lum * lum]


def look_text():
    return "\n".join(["TITLE \"color grade look\"", "# tools/color_grade_reference.py",
                      "LUT_3D_SIZE 9"] + table_text(3, 9, look_colour)) + "\n"


CUBE = read_cube(look_text().encode("utf-8"))


# --- the rule -------------------------------------------------------------------------------

def luma(c):
    return 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2]


def tone(e, g, c, h, s, w, b, f):
    """Steps 1 and 4: the gains in linear light, then the tone terms, each share over 100."""
    if g != [1.0, 1.0, 1.0]:
        e = [S.linear_to_srgb(min(1.0, max(0.0, srgb_to_linear(v) * k))) for v, k in zip(e, g)]
    out = []
    for v in e:
        v += c * v * (1 - v) * (2 * v - 1)
        v += h * v * v * (1 - v)
        v += s * v * (1 - v) ** 2
        v += w * v * v / 4
        v += b * (1 - v) ** 2 / 4
        v += f * (0.25 * (1 - v) ** 2 - 0.1 * v * v)
        out.append(v)
    return out


def vibrance(e, vib, sat):
    """D-142's rule."""
    lum = luma(e)
    m = 1 + sat / 100 + vib / 100 * (1 - (max(e) - min(e)))
    return [lum + (v - lum) * m for v in e]


def balance(e, shadows, highlights):
    """D-127's rule, luminosity not kept, no midtones."""
    lum = luma(e)
    ws = min(1.0, max(0.0, 1 - 2 * lum))
    wh = min(1.0, max(0.0, 2 * lum - 1))
    return [v + (ws * sh + wh * hi) / 200 for v, sh, hi in zip(e, shadows, highlights)]


def push(hue, amount, scale):
    pure = colorsys.hls_to_rgb(hue / 360, 0.5, 1.0)
    return [amount * scale * (v - luma(pure)) for v in pure]


def gains(t, n, stops):
    raw = [2 ** (t / 200), 2 ** (-n / 200), 2 ** (-t / 200)]
    return [k / luma(raw) * 2 ** stops for k in raw]


def steps(v, look):
    """The colour steps, each a function of an encoded colour, in order; empty when nothing
    changes."""
    out = []
    if any(v[k] != 0 for k in ("temperature", "tint", "exposure", "contrast", "highlights",
                               "shadows", "whites", "blacks")):
        g = gains(v["temperature"], v["tint"], v["exposure"])
        terms = [v[k] / 100 for k in ("contrast", "highlights", "shadows", "whites", "blacks")]
        out.append(lambda e: tone(e, g, *terms, 0))
    if v["saturation"] != 100:
        out.append(lambda e: vibrance(e, 0, v["saturation"] - 100))
    if look and v["look_intensity"] != 0:
        i = v["look_intensity"] / 100
        out.append(lambda e: [x + i * (l - x) for x, l in zip(e, lookup(CUBE, e))])
    if v["faded_film"] != 0:
        out.append(lambda e: tone(e, [1.0] * 3, 0, 0, 0, 0, 0, v["faded_film"] / 100))
    if v["vibrance"] != 0 or v["creative_saturation"] != 100:
        out.append(lambda e: vibrance(e, v["vibrance"], v["creative_saturation"] - 100))
    if v["shadow_tint_amount"] != 0 or v["highlight_tint_amount"] != 0:
        bal = v["tint_balance"] / 100
        sh = push(v["shadow_tint_hue"], v["shadow_tint_amount"], 1 - bal)
        hi = push(v["highlight_tint_hue"], v["highlight_tint_amount"], 1 + bal)
        out.append(lambda e: balance(e, sh, hi))
    return out


def vignette(x, y, v):
    """Step 7's share of the colour at the pixel (x, y) of the 16 by 10 layer, and the colour."""
    m, r = v["vignette_roundness"] / 100, math.sqrt(W * H) / 2
    rx, ry = (1 - m) * W / 2 + m * r, (1 - m) * H / 2 + m * r
    outer = (50 + v["vignette_midpoint"]) / 100
    inner = outer * (1 - v["vignette_feather"] / 100)
    dx, dy = (x + 0.5 - W / 2) / rx, (y + 0.5 - H / 2) / ry
    d = math.hypot(dx, dy) / math.sqrt(2)
    if inner == outer:
        t = 1.0 if d >= outer else 0.0
    else:
        u = min(1.0, max(0.0, (d - inner) / (outer - inner)))
        t = u * u * (3 - 2 * u)
    return t * abs(v["vignette_amount"]) * 20 / 100, 0.0 if v["vignette_amount"] < 0 else 1.0


def color_grade(px, v, look):
    todo = steps(v, look)
    out = []
    for i, p in enumerate(px):
        w = R.working(p)
        if p[3] == 0:
            out.append(w)
            continue
        for step in todo:
            a = w[3]
            e = [S.linear_to_srgb(min(1.0, max(0.0, w[c] / a))) for c in range(3)]
            w = [srgb_to_linear(min(1.0, max(0.0, u))) * a for u in step(e)] + [a]
        if v["vignette_amount"] != 0:
            o, g = vignette(i % W, i // W, v)
            a = w[3]
            w = [(w[c] / a + o * (g - w[c] / a)) * a for c in range(3)] + [a]
        out.append(w)
    return out


# --- the cases ------------------------------------------------------------------------------

def case(look="", shift=0, **given):
    for k in given:
        assert k in SETTINGS, k
    return {"look": look, "shift": shift, "set": given}


def values(c, frame_no):
    v = {}
    for k, (low, high, start) in SETTINGS.items():
        v[k] = min(high, max(low, value_at(c["set"].get(k, start), frame_no)))
    return v


def render(c, frame_no):
    return R.frame(color_grade(B.pixels(), values(c, frame_no), c["look"] == "asset-look"),
                   c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    params = {"look": c["look"]}
    for k, (_, _, start) in SETTINGS.items():
        params[k] = setting_json(c["set"].get(k, start))
    p = B.project_json(fx, "core.color_grade", params, c["shift"])
    p["assets"].append({"id": "asset-look", "kind": "lut", "name": "look",
                        "path": "luts/look.cube"})
    return p


TEAL_ORANGE = dict(shadow_tint_hue=195, shadow_tint_amount=60, highlight_tint_hue=35,
                   highlight_tint_amount=50)
EVERYTHING = dict(temperature=20, tint=-10, exposure=0.3, contrast=30, highlights=-20,
                  shadows=20, whites=10, blacks=-10, saturation=110, look_intensity=80,
                  faded_film=20, vibrance=30, creative_saturation=95, tint_balance=-20,
                  vignette_amount=-1.5, **TEAL_ORANGE)

CASES = {
    "FX-GRADE-001": ("Every setting as added, no look: the drawing exactly as it is.", case(),
                     [0]),
    "FX-GRADE-002": ("Temperature 60: warmer, red up and blue down in linear light, white's "
                     "brightness kept, so white itself turns cream with its red held at 1.",
                     case(temperature=60), [0]),
    "FX-GRADE-003": ("Temperature -60, tint 40: cooler and toward magenta.",
                     case(temperature=-60, tint=40), [0]),
    "FX-GRADE-004": ("Exposure 1: one stop brighter in linear light, the brightest colours "
                     "held at white.", case(exposure=1), [0]),
    "FX-GRADE-005": ("Exposure -1.5: a stop and a half darker.", case(exposure=-1.5), [0]),
    "FX-GRADE-006": ("Contrast 80: darks darker, lights lighter, middle grey and the pure "
                     "colours' 0 and 1 kept.", case(contrast=80), [0]),
    "FX-GRADE-007": ("Contrast -60: flatter, everything toward the middle.",
                     case(contrast=-60), [0]),
    "FX-GRADE-008": ("Highlights -70, shadows 70: the lights brought down and the darks "
                     "opened up, black and white kept.", case(highlights=-70, shadows=70), [0]),
    "FX-GRADE-009": ("Whites 50, blacks -50: white past 1 and held, black below 0 and held, "
                     "more contrast at the ends.", case(whites=50, blacks=-50), [0]),
    "FX-GRADE-010": ("Whites -60, blacks 60: white lowered and black lifted, a faded picture.",
                     case(whites=-60, blacks=60), [0]),
    "FX-GRADE-011": ("Saturation 0: every colour its own grey, L of its encoded colour.",
                     case(saturation=0), [0]),
    "FX-GRADE-012": ("Saturation 160: stronger colours, the pure ones already at the edge "
                     "held there.", case(saturation=160), [0]),
    "FX-GRADE-013": ("The look at intensity 100: the look file's table as Color Lookup reads "
                     "it.", case("asset-look"), [0]),
    "FX-GRADE-014": ("The look at intensity 50: halfway from the drawing to the look.",
                     case("asset-look", look_intensity=50), [0]),
    "FX-GRADE-015": ("The look at intensity 200: the look's change doubled, held inside 0 "
                     "to 1.", case("asset-look", look_intensity=200), [0]),
    "FX-GRADE-016": ("The look named, intensity 0: the drawing exactly as it is.",
                     case("asset-look", look_intensity=0), [0]),
    "FX-GRADE-017": ("Faded film 70: black lifted to about 0.18 and white lowered to 0.93, "
                     "a washed-out print.", case(faded_film=70), [0]),
    "FX-GRADE-018": ("Vibrance 80: the dull colours (skin, the warm tones) strengthened more "
                     "than the strong ones; greys unchanged.", case(vibrance=80), [0]),
    "FX-GRADE-019": ("Vibrance -50 with creative saturation 140.",
                     case(vibrance=-50, creative_saturation=140), [0]),
    "FX-GRADE-020": ("Shadow tint hue 195 at 60, highlight tint hue 35 at 50: teal shadows, "
                     "orange highlights, middle grey untouched.", case(**TEAL_ORANGE), [0]),
    "FX-GRADE-021": ("The same with tint balance 60: the shadows' tint weaker, the "
                     "highlights' stronger.", case(tint_balance=60, **TEAL_ORANGE), [0]),
    "FX-GRADE-022": ("Vignette -3, midpoint 50, roundness 0, feather 50: the corners darkened "
                     "toward black by 60 per cent, the middle untouched.",
                     case(vignette_amount=-3), [0]),
    "FX-GRADE-023": ("Vignette 2, midpoint 20, roundness 100, feather 10: a round, firm white "
                     "edge.", case(vignette_amount=2, vignette_midpoint=20,
                                   vignette_roundness=100, vignette_feather=10), [0]),
    "FX-GRADE-024": ("A whole grade: white balance, exposure, the tone sliders, saturation, "
                     "the look at 80, faded film, vibrance, teal and orange tints and a dark "
                     "vignette, each step in its order.", case("asset-look", **EVERYTHING),
                     [0]),
    "FX-GRADE-025": ("Exposure keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 "
                     "untouched, frame 2 one stop up, frame 4 two.",
                     case(exposure=keyed((0, 0), (4, 2))), [0, 2, 4]),
    "FX-GRADE-026": ("Contrast 50 and vignette -4 moved three pixels right: the same, moved; "
                     "the vignette goes with the layer.",
                     case(contrast=50, vignette_amount=-4, shift=3), [0, 3]),
    "FX-GRADE-027": ("Contrast 50 with the look naming the drawing, which is not a lookup "
                     "file: drawn with the contrast and without the look, with a warning on "
                     "opening and on each frame.", case("asset-colours", contrast=50), [0]),
}

INVALID = {
    "FX-GRADE-028": ("Temperature 101, above 100.", case(temperature=101)),
    "FX-GRADE-029": ("Exposure -5.5, below -5.", case(exposure=-5.5)),
    "FX-GRADE-030": ("Saturation 201, above 200.", case(saturation=201)),
    "FX-GRADE-031": ("Look intensity -1, below 0.", case("asset-look", look_intensity=-1)),
    "FX-GRADE-032": ("Shadow tint hue 361, past a full turn.", case(shadow_tint_hue=361)),
    "FX-GRADE-033": ("Vignette amount 5.5, above 5.", case(vignette_amount=5.5)),
    "FX-GRADE-034": ("Vignette roundness -20: Lumetri's squarer vignette, below 0, which this "
                     "effect does not draw (a gap).", case(vignette_roundness=-20)),
}


def main():
    (OUT / "luts").mkdir(parents=True, exist_ok=True)
    (OUT / "luts" / "look.cube").write_bytes(look_text().encode("utf-8"))
    expected = B.write_cases(OUT, "color_grade", CASES, INVALID, render, plain, file_json)
    expected["cases"]["FX-GRADE-027"]["warning"] = "EFFECT_PARAMETER_INVALID"
    (OUT / "expected_color_grade.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                   encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    px = B.pixels()
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g, e=1e-9: all(near(p, q, e) for p, q in zip(f, g))  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731
    enc = lambda p: [S.linear_to_srgb(u / p[3]) for u in p[:3]]  # noqa: E731

    assert c["FX-GRADE-001"]["0"] == drawn and c["FX-GRADE-016"]["0"] == drawn
    assert c["FX-GRADE-025"]["0"] == drawn
    warm = c["FX-GRADE-002"]["0"]
    assert near(enc(warm[at(2, 0)])[:1], [1]) and enc(warm[at(2, 0)])[2] < 0.97
    assert enc(warm[at(3, 0)])[0] > enc(warm[at(3, 0)])[2]
    cool = c["FX-GRADE-003"]["0"]
    assert enc(cool[at(3, 0)])[2] > enc(cool[at(3, 0)])[1]
    up, down = c["FX-GRADE-004"]["0"], c["FX-GRADE-005"]["0"]
    grey = R.working(px[at(3, 0)])
    assert abs(up[at(3, 0)][0] - 2 * grey[0]) < 1e-9 and abs(down[at(3, 0)][0] - grey[0] / 2 ** 1.5) < 1e-9
    assert near(enc(up[at(2, 0)]), [1, 1, 1]) and near(up[at(1, 0)], [0, 0, 0, 1])
    hard = c["FX-GRADE-006"]["0"]
    for x in (1, 2, 4, 8):  # the ends kept
        assert near(hard[at(x, 0)], R.working(px[at(x, 0)]))
    g3 = B.encoded(px[at(3, 0)])[0]
    assert abs(enc(hard[at(3, 0)])[0] - (g3 + 0.8 * g3 * (1 - g3) * (2 * g3 - 1))) < 1e-9
    assert enc(hard[at(12, 0)])[0] < B.encoded(px[at(12, 0)])[0]  # a dark darker
    assert enc(hard[at(14, 0)])[1] > B.encoded(px[at(14, 0)])[1]  # a light lighter
    for x in (1, 2):
        assert near(c["FX-GRADE-008"]["0"][at(x, 0)], R.working(px[at(x, 0)]))
    assert enc(c["FX-GRADE-008"]["0"][at(12, 0)])[0] > B.encoded(px[at(12, 0)])[0]
    assert near(enc(c["FX-GRADE-010"]["0"][at(1, 0)]), [0.15] * 3)
    assert near(enc(c["FX-GRADE-010"]["0"][at(2, 0)]), [0.85 + 0.6 * 0.15 ** 2 / 4] * 3)
    flat = c["FX-GRADE-011"]["0"]
    for i, p in enumerate(flat):
        if p[3] > 0:
            e = enc(p)
            assert abs(e[0] - e[1]) < 1e-9 and abs(e[1] - e[2]) < 1e-9
            assert abs(e[0] - min(1, luma(B.encoded(px[i])))) < 1e-9
    full = c["FX-GRADE-013"]["0"]
    assert near(enc(full[at(13, 0)]), [min(1, max(0, u)) for u in lookup(CUBE, B.encoded(px[at(13, 0)]))])
    half = c["FX-GRADE-014"]["0"]
    for x in (3, 13):
        e0, e1 = B.encoded(px[at(x, 0)]), enc(full[at(x, 0)])
        assert near(enc(half[at(x, 0)]), [(u + w) / 2 for u, w in zip(e0, e1)])
    faded = c["FX-GRADE-017"]["0"]
    assert near(enc(faded[at(1, 0)]), [0.175] * 3) and near(enc(faded[at(2, 0)]), [0.93] * 3)
    vib = c["FX-GRADE-018"]["0"]
    assert near(vib[at(3, 0)], R.working(px[at(3, 0)]))  # grey kept
    tints = c["FX-GRADE-020"]["0"]
    assert enc(tints[at(12, 0)])[2] > B.encoded(px[at(12, 0)])[2]  # the shadow bluer
    assert enc(tints[at(14, 0)])[2] < B.encoded(px[at(14, 0)])[2]  # the highlight warmer
    g = B.encoded(px[at(3, 0)])
    assert abs(luma(g) - 0.5) < 0.01
    balanced = c["FX-GRADE-021"]["0"]
    for x in (12, 14):  # the shadow's change smaller, the highlight's larger
        e0 = B.encoded(px[at(x, 0)])[2]
        d20, d21 = abs(enc(tints[at(x, 0)])[2] - e0), abs(enc(balanced[at(x, 0)])[2] - e0)
        assert (d21 < d20) == (x == 12), x
    dark = c["FX-GRADE-022"]["0"]
    assert near(dark[at(7, 4)], R.working(px[at(7, 4)]))  # the middle untouched
    assert dark[at(2, 0)][0] < R.working(px[at(2, 0)])[0]  # a corner darker
    two = c["FX-GRADE-025"]
    assert abs(two["2"][at(3, 0)][0] - 2 * grey[0]) < 1e-9
    assert abs(two["4"][at(3, 0)][0] - min(1, 4 * grey[0])) < 1e-9
    moved = c["FX-GRADE-026"]["0"]
    still = R.frame(color_grade(px, values(case(contrast=50, vignette_amount=-4), 0), False), 0)
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == still[at(0, y):at(W - 3, y)]
    no_look = c["FX-GRADE-027"]["0"]
    assert like(no_look, R.frame(color_grade(px, values(case(contrast=50), 0), False), 0))
    for fx, frames in c.items():
        for f in frames.values():
            for i, p in enumerate(f):
                assert p[3] == drawn[i][3] or fx == "FX-GRADE-026"
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
    print("checked")


if __name__ == "__main__":
    main()
