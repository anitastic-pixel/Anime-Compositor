"""Color Grade, worked a second way: Curves, Color Wheels and HSL Secondary (D-398).

The second unit of `core.color_grade` (D-397), modelled on Premiere Pro's and After Effects'
Lumetri Color panel under our own name. D-397 built Basic Correction, Creative and Vignette
(`tools/color_grade_reference.py`, whose rule this file reuses step for step); this unit adds
Lumetri's three middle sections, run in the panel's order between Creative and Vignette:
Curves (the RGB curves and the hue-versus-saturation curve), Color Wheels (shadows, midtones and
highlights, each a colour and a lightness) and HSL Secondary (a key by hue, saturation and
lightness ranges, and a correction inside it). Adobe does not publish Lumetri's maths, so every
rule below is ours, built where it can be from rules this program already has (named at each
step); document 14 records it as a deviation. Nothing is ported.

The new settings, with where they start (every number keyable; the lists and words not):

    curve_master, curve_red,
    curve_green, curve_blue        Curves' points (D-111), [[0, 0], [255, 255]]
    hue_saturation                 0 to 16 points [hue, saturation], hue 0 to under 360 rising,
                                   saturation 0 to 200; [] (none)
    shadow_wheel_hue,
    midtone_wheel_hue,
    highlight_wheel_hue            0 to 360, 0        degrees
    shadow_wheel_amount, ...       0 to 100, 0
    shadow_lightness, ...          -100 to 100, 0
    key_hue                        0 to 360, 0        degrees
    key_hue_range                  0 to 180, 180      degrees each side; 180 takes every hue
    key_hue_softness               0 to 180, 0        degrees
    key_saturation_low, _high      0 to 100, 0 and 100   per cent
    key_lightness_low, _high       0 to 100, 0 and 100   per cent
    key_softness                   0 to 100, 0        per cent, the saturation and lightness edges
    key_invert                     "off" or "on", "off"
    key_view                       "off" or "mask", "off"
    secondary_temperature,
    secondary_tint,
    secondary_contrast             -100 to 100, 0
    secondary_saturation           0 to 200, 100
    secondary_wheel_hue            0 to 360, 0
    secondary_wheel_amount         0 to 100, 0
    secondary_lightness            -100 to 100, 0

A file without them (D-397's) reads each as it starts, and draws what D-397 drew.

The rule. e is the encoded colour as D-397 has it, L its encoded luma, and each step that
changes anything is a step of its own, its result held inside 0 to 1 and back to linear at the
pixel's covering. After D-397's steps 1 to 6 (tone, saturation, look, faded film, vibrance,
tints) and before its vignette:

8. RGB curves, when any of the four is not straight: Curves' rule (D-111), each channel through
   its own curve and then the master.
9. Hue versus saturation, when the list has a point whose saturation is not 100: with H the HLS
   hue of e in degrees (0 for a grey), S(H) the curve, Vibrance's rule (D-140) with vibrance 0
   and saturation S(H) - 100: L + (e - L) S(H) / 100. One point is that saturation for every
   hue; with more, the curve runs round the hue circle, from each point to the next (the last to
   the first, through 360), s_i + (s_{i+1} - s_i) u^2 (3 - 2u), u the fraction of the way.
10. Color wheels, when any amount or lightness is not 0: Color Balance's rule (D-130), luminosity
   not kept, with shadows, midtones and highlights each set to A (C - L(C)) + K, A the wheel's
   amount, C its hue's pure colour (HLS saturation 1, lightness 1/2), K its lightness, the same
   on all three channels.
11. HSL Secondary, when the correction changes anything or the view is the mask. The key m from
   e's HLS hue H (degrees), saturation S and lightness Lh (per cent): the hue's weight is 1 within
   key_hue_range degrees of key_hue round the circle, falling to 0 over key_hue_softness more
   (1 - t^2 (3 - 2t), t the fraction of the softness passed), 0 beyond (180 takes every hue;
   below 180 a grey, which has no hue, is not taken);
   the saturation's weight is 1 from key_saturation_low to key_saturation_high, falling the same
   way to 0 over key_softness outside them, and the lightness's likewise; m their product, or
   1 - m with key_invert on. With key_view "mask" the step gives the grey m, and the vignette is
   not drawn. Otherwise the correction c, from e: the white balance's gains of
   secondary_temperature and secondary_tint (step 1's, exposure 0) in linear light, held inside
   0 to 1; then v += k v (1 - v)(2v - 1) for the contrast k over 100; then L(c) + (c - L(c)) s
   for the saturation s over 100; then c + (A (C - L(C)) + K) / 200 for the wheel and lightness
   as step 10's; and the step gives e + m (c - e).

**This file never runs the build's code path.** It works in double precision on lists,
straight from the drawing's 8-bit values, with Python's own colorsys for HLS, where the build
works on its single-precision buffers.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py): pure colours, greys,
a skin tone, orange and three warm tones, darker row by row. The projects go into
`Fixtures/color_grade_2`, the expected frames into
`Fixtures/color_grade_2/expected_color_grade_2.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/color_grade_2_reference.py
"""

import colorsys
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from curves_reference import spline  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import color_grade_reference as G  # noqa: E402

W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "color_grade_2"
STRAIGHT = [[0, 0], [255, 255]]
SETTINGS = dict(G.SETTINGS)
SETTINGS.update({
    "shadow_wheel_hue": (0, 360, 0), "shadow_wheel_amount": (0, 100, 0),
    "shadow_lightness": (-100, 100, 0),
    "midtone_wheel_hue": (0, 360, 0), "midtone_wheel_amount": (0, 100, 0),
    "midtone_lightness": (-100, 100, 0),
    "highlight_wheel_hue": (0, 360, 0), "highlight_wheel_amount": (0, 100, 0),
    "highlight_lightness": (-100, 100, 0),
    "key_hue": (0, 360, 0), "key_hue_range": (0, 180, 180), "key_hue_softness": (0, 180, 0),
    "key_saturation_low": (0, 100, 0), "key_saturation_high": (0, 100, 100),
    "key_lightness_low": (0, 100, 0), "key_lightness_high": (0, 100, 100),
    "key_softness": (0, 100, 0),
    "secondary_temperature": (-100, 100, 0), "secondary_tint": (-100, 100, 0),
    "secondary_contrast": (-100, 100, 0), "secondary_saturation": (0, 200, 100),
    "secondary_wheel_hue": (0, 360, 0), "secondary_wheel_amount": (0, 100, 0),
    "secondary_lightness": (-100, 100, 0),
})
NEW = [k for k in SETTINGS if k not in G.SETTINGS]
LISTS = {"curve_master": STRAIGHT, "curve_red": STRAIGHT, "curve_green": STRAIGHT,
         "curve_blue": STRAIGHT, "hue_saturation": []}
WORDS = {"key_invert": "off", "key_view": "off"}


# --- the rule -------------------------------------------------------------------------------

luma = G.luma


def clamp(v, lo=0.0, hi=1.0):
    return min(hi, max(lo, v))


def smooth(t):
    t = clamp(t)
    return t * t * (3 - 2 * t)


def hue_curve(points):
    """The hue-versus-saturation curve as a function of the hue in degrees."""
    if len(points) == 1:
        return lambda h: points[0][1]

    def at(h):
        n = len(points)
        for i in range(n):
            h0, s0 = points[i]
            h1, s1 = points[(i + 1) % n]
            if i == n - 1:
                h1 += 360
            x = h if h >= h0 else h + 360
            if h0 <= x < h1:
                return s0 + (s1 - s0) * smooth((x - h0) / (h1 - h0))
        raise AssertionError(h)
    return at


def wheel(hue, amount, lightness):
    return [p + lightness for p in G.push(hue, amount, 1)]


def balance3(e, sh, mid, hi):
    """D-130's rule with all three ranges, luminosity not kept."""
    lum = luma(e)
    ws = clamp(1 - 2 * lum)
    wh = clamp(2 * lum - 1)
    wm = 1 - ws - wh
    return [v + (ws * a + wm * b + wh * c) / 200 for v, a, b, c in zip(e, sh, mid, hi)]


def band(x, low, high, soft):
    """1 from low to high, falling smoothly to 0 over soft outside them."""
    d = low - x if x < low else x - high if x > high else 0.0
    if d <= 0:
        return 1.0
    return 0.0 if soft == 0 else 1 - smooth(d / soft)


def key(e, v, invert):
    h, l, s = colorsys.rgb_to_hls(*e)
    d = abs((h * 360 - v["key_hue"] + 180) % 360 - 180)
    rng, soft = v["key_hue_range"], v["key_hue_softness"]
    wh = 1.0 if rng >= 180 else 0.0 if s == 0 else band(d, 0, rng, soft)
    m = (wh * band(s * 100, v["key_saturation_low"], v["key_saturation_high"], v["key_softness"])
         * band(l * 100, v["key_lightness_low"], v["key_lightness_high"], v["key_softness"]))
    return 1 - m if invert else m


def correcting(v):
    return (any(v[k] != 0 for k in ("secondary_temperature", "secondary_tint",
                                    "secondary_contrast", "secondary_wheel_amount",
                                    "secondary_lightness"))
            or v["secondary_saturation"] != 100)


def correction(e, v):
    c = e
    if v["secondary_temperature"] != 0 or v["secondary_tint"] != 0:
        g = G.gains(v["secondary_temperature"], v["secondary_tint"], 0)
        c = [S.linear_to_srgb(clamp(srgb_to_linear(x) * k)) for x, k in zip(c, g)]
    k = v["secondary_contrast"] / 100
    if k != 0:
        c = [x + k * x * (1 - x) * (2 * x - 1) for x in c]
    if v["secondary_saturation"] != 100:
        lum = luma(c)
        c = [lum + (x - lum) * v["secondary_saturation"] / 100 for x in c]
    if v["secondary_wheel_amount"] != 0 or v["secondary_lightness"] != 0:
        p = wheel(v["secondary_wheel_hue"], v["secondary_wheel_amount"], v["secondary_lightness"])
        c = [x + q / 200 for x, q in zip(c, p)]
    return c


def steps(v, look, lists, words):
    out = G.steps(v, look)
    curves = [lists[k] for k in ("curve_red", "curve_green", "curve_blue")]
    master = lists["curve_master"]
    if any(c != STRAIGHT for c in curves + [master]):
        m = spline(master)
        fs = [spline(c) for c in curves]
        out.append(lambda e: [clamp(m(clamp(f(x * 255), 0, 255)), 0, 255) / 255
                              for f, x in zip(fs, e)])
    hs = lists["hue_saturation"]
    if any(p[1] != 100 for p in hs):
        curve = hue_curve(hs)

        def hue_sat(e):
            lum = luma(e)
            k = curve(colorsys.rgb_to_hls(*e)[0] * 360) / 100
            return [lum + (x - lum) * k for x in e]
        out.append(hue_sat)
    names = ("shadow", "midtone", "highlight")
    if any(v[f"{n}_wheel_amount"] != 0 or v[f"{n}_lightness"] != 0 for n in names):
        pushes = [wheel(v[f"{n}_wheel_hue"], v[f"{n}_wheel_amount"], v[f"{n}_lightness"])
                  for n in names]
        out.append(lambda e: balance3(e, *pushes))
    invert, mask = words["key_invert"] == "on", words["key_view"] == "mask"
    if mask:
        out.append(lambda e: [key(e, v, invert)] * 3)
    elif correcting(v):
        def secondary(e):
            m = key(e, v, invert)
            return [x + m * (y - x) for x, y in zip(e, correction(e, v))]
        out.append(secondary)
    return out


def color_grade(px, v, look, lists, words):
    todo = steps(v, look, lists, words)
    out = []
    for i, p in enumerate(px):
        w = R.working(p)
        if p[3] == 0:
            out.append(w)
            continue
        for step in todo:
            a = w[3]
            e = [S.linear_to_srgb(clamp(w[c] / a)) for c in range(3)]
            w = [srgb_to_linear(clamp(u)) * a for u in step(e)] + [a]
        if v["vignette_amount"] != 0 and words["key_view"] != "mask":
            o, g = G.vignette(i % W, i // W, v)
            a = w[3]
            w = [(w[c] / a + o * (g - w[c] / a)) * a for c in range(3)] + [a]
        out.append(w)
    return out


# --- the cases ------------------------------------------------------------------------------

def case(look="", shift=0, old=False, **given):
    for k in given:
        assert k in SETTINGS or k in LISTS or k in WORDS, k
    return {"look": look, "shift": shift, "old": old, "set": given}


def values(c, frame_no):
    v = {}
    for k, (low, high, start) in SETTINGS.items():
        v[k] = min(high, max(low, value_at(c["set"].get(k, start), frame_no)))
    return v


def render(c, frame_no):
    lists = {k: c["set"].get(k, d) for k, d in LISTS.items()}
    words = {k: c["set"].get(k, d) for k, d in WORDS.items()}
    return R.frame(color_grade(B.pixels(), values(c, frame_no), c["look"] == "asset-look",
                               lists, words), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    params = {"look": c["look"]}
    for k, (_, _, start) in SETTINGS.items():
        if not (c["old"] and k in NEW):
            params[k] = setting_json(c["set"].get(k, start))
    if not c["old"]:
        for k, d in LISTS.items():
            params[k] = c["set"].get(k, d)
        for k, d in WORDS.items():
            params[k] = c["set"].get(k, d)
    p = B.project_json(fx, "core.color_grade", params, c["shift"])
    p["assets"].append({"id": "asset-look", "kind": "lut", "name": "look",
                        "path": "luts/look.cube"})
    return p


S_CURVE = [[0, 0], [64, 40], [192, 215], [255, 255]]
REDS = dict(key_hue=0, key_hue_range=20)
SKIN_KEY = dict(key_hue=25, key_hue_range=10, key_hue_softness=20)

CASES = {
    "FX-GRADE2-001": ("Every setting as added, the new ones written: the drawing exactly as "
                      "it is.", case(), [0]),
    "FX-GRADE2-002": ("Master curve, an S through (64, 40) and (192, 215): darks darker, lights "
                      "lighter, black and white kept.", case(curve_master=S_CURVE), [0]),
    "FX-GRADE2-003": ("Red curve lifted through (128, 170), blue lowered through (128, 90): "
                      "warmer midtones, black and white kept.",
                      case(curve_red=[[0, 0], [128, 170], [255, 255]],
                           curve_blue=[[0, 0], [128, 90], [255, 255]]), [0]),
    "FX-GRADE2-004": ("Hue versus saturation, one point at saturation 0: every colour its own "
                      "grey, whatever its hue.", case(hue_saturation=[[0, 0]]), [0]),
    "FX-GRADE2-005": ("Hue versus saturation through (0, 100), (120, 200), (240, 100): the "
                      "greens twice as strong, pure red and pure blue unchanged.",
                      case(hue_saturation=[[0, 100], [120, 200], [240, 100]]), [0]),
    "FX-GRADE2-006": ("Hue versus saturation through (30, 150) and (200, 40): the curve runs "
                      "round through 360 from the second point back to the first; oranges "
                      "stronger, cyans and blues duller.",
                      case(hue_saturation=[[30, 150], [200, 40]]), [0]),
    "FX-GRADE2-007": ("Hue versus saturation, every point at 100: no step, the drawing exactly "
                      "as it is.", case(hue_saturation=[[0, 100], [180, 100]]), [0]),
    "FX-GRADE2-008": ("Shadow wheel hue 195 at 60: the darks teal, the lights untouched.",
                      case(shadow_wheel_hue=195, shadow_wheel_amount=60), [0]),
    "FX-GRADE2-009": ("Midtone wheel hue 30 at 50 with midtone lightness 20: the middle "
                      "warmer and lighter, black and white untouched.",
                      case(midtone_wheel_hue=30, midtone_wheel_amount=50, midtone_lightness=20),
                      [0]),
    "FX-GRADE2-010": ("Highlight wheel hue 220 at 40 with highlight lightness -30: the lights "
                      "cooler and lower, the darks untouched.",
                      case(highlight_wheel_hue=220, highlight_wheel_amount=40,
                           highlight_lightness=-30), [0]),
    "FX-GRADE2-011": ("Shadow lightness 40 and highlight lightness -20, no colour: black "
                      "lifted to 0.2, white lowered to 0.9, greys only.",
                      case(shadow_lightness=40, highlight_lightness=-20), [0]),
    "FX-GRADE2-012": ("Key the reds (hue 0, 20 either side, no softness), secondary saturation "
                      "0: red grey, everything else, the skin and orange too, untouched.", case(secondary_saturation=0, **REDS), [0]),
    "FX-GRADE2-013": ("The same key with hue softness 30: hues between 20 and 50 degrees away "
                      "partly grey, the skin (about 25) and orange (about 33); yellow untouched.",
                      case(secondary_saturation=0, key_hue_softness=30, **REDS), [0]),
    "FX-GRADE2-014": ("Key saturation 50 to 100, softness 10, secondary lightness -40: the "
                      "strong colours darker, greys and the dull tones untouched.",
                      case(key_saturation_low=50, key_softness=10, secondary_lightness=-40),
                      [0]),
    "FX-GRADE2-015": ("Key lightness 55 to 100, secondary temperature -50: the light colours "
                      "cooler, the darks untouched.",
                      case(key_lightness_low=55, secondary_temperature=-50), [0]),
    "FX-GRADE2-016": ("FX-GRADE2-012 inverted: everything but the reds grey.",
                      case(secondary_saturation=0, key_invert="on", **REDS), [0]),
    "FX-GRADE2-017": ("FX-GRADE2-013's key shown as the mask: white where the key takes all, "
                      "black where none, greys between.",
                      case(secondary_saturation=0, key_hue_softness=30, key_view="mask", **REDS),
                      [0]),
    "FX-GRADE2-018": ("The mask view with no correction set, inverted: the mask still shown.",
                      case(key_view="mask", key_invert="on", **REDS), [0]),
    "FX-GRADE2-019": ("Key everything (as added), secondary contrast 60 and tint 30: a second "
                      "correction over the whole drawing.",
                      case(secondary_contrast=60, secondary_tint=30), [0]),
    "FX-GRADE2-020": ("Key the skin (hue 25, 10 either side, softness 20), secondary wheel hue "
                      "200 at 70: the skin and warm tones cooled, the rest untouched.",
                      case(secondary_wheel_hue=200, secondary_wheel_amount=70, **SKIN_KEY), [0]),
    "FX-GRADE2-021": ("Key hue 350, 20 either side: the key wraps through 0 and takes red; "
                      "secondary saturation 0.",
                      case(key_hue=350, key_hue_range=20, secondary_saturation=0), [0]),
    "FX-GRADE2-022": ("A whole grade: D-397's sections, the master S-curve, a hue curve, the "
                      "three wheels, a skin key cooled and a dark vignette, each in its order.",
                      case("asset-look", curve_master=S_CURVE,
                           hue_saturation=[[0, 110], [120, 90], [240, 120]],
                           shadow_wheel_hue=200, shadow_wheel_amount=30, midtone_lightness=10,
                           highlight_wheel_hue=40, highlight_wheel_amount=20,
                           secondary_saturation=80, secondary_wheel_hue=200,
                           secondary_wheel_amount=30, **SKIN_KEY, **G.EVERYTHING), [0]),
    "FX-GRADE2-023": ("Shadow lightness keyed from 0 at frame 0 to 60 at frame 4, linear: "
                      "frame 0 untouched, black lifted to 0.15 at frame 2 and 0.3 at frame 4.",
                      case(shadow_lightness=keyed((0, 0), (4, 60))), [0, 2, 4]),
    "FX-GRADE2-024": ("Key hue keyed from 0 at frame 0 to 120 at frame 4, 20 either side, "
                      "secondary saturation 0: the reds grey at frame 0, the greens at frame 4.",
                      case(key_hue=keyed((0, 0), (4, 120)), key_hue_range=20,
                           secondary_saturation=0), [0, 4]),
    "FX-GRADE2-025": ("Wheels and a key moved three pixels right: the same, moved.",
                      case(shift=3, shadow_wheel_hue=195, shadow_wheel_amount=60,
                           secondary_saturation=0, **REDS), [0]),
    "FX-GRADE2-026": ("The mask view with a dark vignette: the mask shown, the vignette not "
                      "drawn over it.",
                      case(key_view="mask", vignette_amount=-3, **REDS), [0]),
    "FX-GRADE2-027": ("A D-397 file without the new settings, contrast 40: D-397's own picture.",
                      case(old=True, contrast=40), [0]),
}

INVALID = {
    "FX-GRADE2-028": ("Key hue range 181, past 180.", case(key_hue_range=181)),
    "FX-GRADE2-029": ("Key softness 101, above 100.", case(key_softness=101)),
    "FX-GRADE2-030": ("Secondary saturation 201, above 200.", case(secondary_saturation=201)),
    "FX-GRADE2-031": ("Shadow lightness -101, below -100.", case(shadow_lightness=-101)),
    "FX-GRADE2-032": ("Master curve with a point at 256, past 255.",
                      case(curve_master=[[0, 0], [256, 255]])),
    "FX-GRADE2-033": ("Red curve with its ins not rising.",
                      case(curve_red=[[0, 0], [128, 100], [128, 140], [255, 255]])),
    "FX-GRADE2-034": ("Green curve of one point.", case(curve_green=[[0, 0]])),
    "FX-GRADE2-035": ("Hue versus saturation with a hue of 360, a full turn.",
                      case(hue_saturation=[[0, 100], [360, 50]])),
    "FX-GRADE2-036": ("Hue versus saturation with a saturation of 201.",
                      case(hue_saturation=[[90, 201]])),
    "FX-GRADE2-037": ("Hue versus saturation with its hues not rising.",
                      case(hue_saturation=[[200, 50], [100, 150]])),
    "FX-GRADE2-038": ("Hue versus saturation of 17 points, past 16.",
                      case(hue_saturation=[[i * 20, 100] for i in range(17)])),
    "FX-GRADE2-039": ("Key invert \"yes\", not \"on\" or \"off\".", case(key_invert="yes")),
    "FX-GRADE2-040": ("Key view \"grey\", not \"off\" or \"mask\".", case(key_view="grey")),
}


def main():
    (OUT / "luts").mkdir(parents=True, exist_ok=True)
    (OUT / "luts" / "look.cube").write_bytes(G.look_text().encode("utf-8"))
    expected = B.write_cases(OUT, "color_grade_2", CASES, INVALID, render, plain, file_json)
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
    same = lambda f, x, y: near(f[at(x, y)], R.working(px[at(x, y)]))  # noqa: E731
    grey = lambda p: abs(enc(p)[0] - enc(p)[1]) < 1e-9 and abs(enc(p)[1] - enc(p)[2]) < 1e-9  # noqa: E731,E501

    assert c["FX-GRADE2-001"]["0"] == drawn and c["FX-GRADE2-007"]["0"] == drawn
    assert c["FX-GRADE2-023"]["0"] == drawn
    s = c["FX-GRADE2-002"]["0"]
    assert same(s, 1, 0) and same(s, 2, 0)
    assert enc(s[at(12, 0)])[0] < B.encoded(px[at(12, 0)])[0]
    assert enc(s[at(14, 0)])[0] > B.encoded(px[at(14, 0)])[0]
    warm = c["FX-GRADE2-003"]["0"]
    assert same(warm, 1, 0) and same(warm, 2, 0)
    assert enc(warm[at(3, 0)])[0] > enc(warm[at(3, 0)])[2]
    for i, p in enumerate(c["FX-GRADE2-004"]["0"]):
        if p[3] > 0:
            assert grey(p), i
    five = c["FX-GRADE2-005"]["0"]
    assert same(five, 8, 0) and same(five, 9, 0)
    g = enc(five[at(6, 4)])
    assert g[1] > B.encoded(px[at(6, 4)])[1] or g[1] == 1.0
    six = c["FX-GRADE2-006"]["0"]
    o0, o1 = B.encoded(px[at(11, 4)]), enc(six[at(11, 4)])  # orange, hue 33: stronger
    assert o1[0] - o1[2] > o0[0] - o0[2]
    b0, b1 = B.encoded(px[at(9, 3)]), enc(six[at(9, 3)])  # blue, hue 240: duller
    assert b1[2] - b1[0] < b0[2] - b0[0]
    teal = c["FX-GRADE2-008"]["0"]
    assert same(teal, 2, 0) and same(teal, 14, 0)
    assert enc(teal[at(12, 0)])[2] > B.encoded(px[at(12, 0)])[2]
    mid = c["FX-GRADE2-009"]["0"]
    assert same(mid, 1, 0) and same(mid, 2, 0)
    assert luma(enc(mid[at(3, 0)])) > luma(B.encoded(px[at(3, 0)]))
    hi = c["FX-GRADE2-010"]["0"]
    assert same(hi, 1, 0) and enc(hi[at(2, 0)])[0] < 1
    lift = c["FX-GRADE2-011"]["0"]
    assert near(enc(lift[at(1, 0)]), [0.2] * 3) and near(enc(lift[at(2, 0)]), [0.9] * 3)
    reds = c["FX-GRADE2-012"]["0"]
    assert grey(reds[at(8, 0)]) and same(reds, 9, 0) and same(reds, 10, 0) and same(reds, 11, 0)
    soft = c["FX-GRADE2-013"]["0"]
    assert not same(soft, 11, 0) and not grey(soft[at(11, 0)]) and same(soft, 4, 0)
    strong = c["FX-GRADE2-014"]["0"]
    assert same(strong, 3, 0) and enc(strong[at(8, 0)])[0] < 1
    light = c["FX-GRADE2-015"]["0"]
    assert same(light, 12, 0) and not same(light, 14, 0)
    inv = c["FX-GRADE2-016"]["0"]
    assert same(inv, 8, 0) and grey(inv[at(9, 0)])
    mask = c["FX-GRADE2-017"]["0"]
    assert near(enc(mask[at(8, 0)]), [1, 1, 1]) and near(enc(mask[at(9, 0)]), [0, 0, 0])
    assert 0 < enc(mask[at(11, 0)])[0] < 1
    assert near(enc(c["FX-GRADE2-018"]["0"][at(8, 0)]), [0, 0, 0])
    wrap = c["FX-GRADE2-021"]["0"]
    assert grey(wrap[at(8, 0)]) and same(wrap, 9, 0)
    two = c["FX-GRADE2-023"]
    assert near(enc(two["2"][at(1, 0)]), [0.15] * 3) and near(enc(two["4"][at(1, 0)]), [0.3] * 3)
    hue = c["FX-GRADE2-024"]
    assert grey(hue["0"][at(8, 0)]) and same(hue["0"], 6, 0)
    assert grey(hue["4"][at(6, 0)]) and same(hue["4"], 8, 0)
    moved = c["FX-GRADE2-025"]["0"]
    still = render(case(shadow_wheel_hue=195, shadow_wheel_amount=60, secondary_saturation=0,
                        **REDS), 0)
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == still[at(0, y):at(W - 3, y)]
    assert like(c["FX-GRADE2-026"]["0"], render(case(key_view="mask", **REDS), 0))
    old = c["FX-GRADE2-027"]["0"]
    assert like(old, R.frame(G.color_grade(px, G.values(G.case(contrast=40), 0), False), 0))
    for fx, frames in c.items():
        for f in frames.values():
            for i, p in enumerate(f):
                assert p[3] == drawn[i][3] or fx == "FX-GRADE2-025"
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
    print("checked")


if __name__ == "__main__":
    main()
