"""Color Grade, worked a third way: the last curves, the HSL Secondary's three wheels and its
key's Denoise and Blur (D-571).

The third unit of `core.color_grade` (D-397, D-398), after Lumetri Color's panel under our own
name. D-398 (`tools/color_grade_2_reference.py`, whose rule this file reuses step for step)
built Curves' RGB curves and hue versus saturation, Color Wheels and HSL Secondary; this unit
adds what EFFECTS.md row 121 listed as missing from them: Lumetri's other four curves (hue
versus hue, hue versus luma, luma versus saturation and saturation versus saturation), the HSL
Secondary's three-way correction wheels, and its key's Denoise and Blur. Adobe does not publish
Lumetri's maths, so every rule below is ours, built from rules this program already has (named
at each step); document 14 records it as a deviation. Nothing is ported. Lumetri's highlight
recovery past white is not here: the grade works on encoded colours held inside 0 to 1
(D-397), so there is nothing past white for it to recover, and any rule would be invented.

The new settings, with where they start (every number keyable; the lists and the word not):

    hue_hue                        0 to 16 points [hue, shift], hue 0 to under 360 rising,
                                   shift -180 to 180 degrees; [] (none)
    hue_luma                       0 to 16 points [hue, luma], hue as above, luma -100 to 100;
                                   [] (none)
    luma_saturation                0 to 16 points [luma, saturation], luma 0 to 100 rising,
                                   saturation 0 to 200; [] (none)
    saturation_saturation          0 to 16 points [saturation, saturation], the first 0 to 100
                                   rising, the second 0 to 200; [] (none)
    secondary_wheels               "one" or "three", "one"
    secondary_shadow_wheel_hue,
    secondary_highlight_wheel_hue  0 to 360, 0        degrees
    secondary_shadow_wheel_amount,
    secondary_highlight_wheel_amount   0 to 100, 0
    secondary_shadow_lightness,
    secondary_highlight_lightness  -100 to 100, 0
    key_denoise                    0 to 100, 0        a median of radius key_denoise / 10 px
    key_blur                       0 to 100, 0        a Gaussian of sigma key_blur / 10 px

A file without them (D-398's) reads each as it starts, and draws what D-398 drew.

The rule. e is the encoded colour as D-397 has it, L its encoded luma, each step that changes
anything a step of its own, its result held inside 0 to 1 and back to linear at the pixel's
covering. After D-398's hue versus saturation (its step 9) and before its colour wheels, in
Lumetri's order:

9a. Hue versus hue, when a point's shift is not 0: with H the HLS hue of e, its HLS lightness
    and saturation kept, the hue turned to H + D(H) round the circle, D the curve. A grey,
    which has no hue, is left as it is.
9b. Hue versus luma, when a point's luma is not 0: e + K(H) (max(e) - min(e)) / 200 on each
    channel, K the curve; the colour's chroma scales it, so a grey is left as it is.
9c. Luma versus saturation, when a point's saturation is not 100: D-398's rule with S(100 L):
    L + (e - L) S / 100.
9d. Saturation versus saturation, when a point's saturation is not 100: the same with S at 100
    times e's HLS saturation (held at 1).

The hue curves run round the hue circle as D-398's (u^2 (3 - 2u) from each point to the next,
the last to the first through 360; one point that value for every hue). The luma and
saturation curves run the same way from each point to the next and are held flat past the
first and the last.

11. HSL Secondary. The key m as D-398's, each pixel's m (0 where the pixel does not show) then,
    when key_denoise is not 0, Median's rule (D-203) over the key with radius key_denoise / 10
    pixels (a disc, places outside the layer not counted, below 1 nothing), and when key_blur is
    not 0, Gaussian Blur's kernel (document 21) of sigma key_blur / 10 pixels across and then
    down, the layer's edge places repeated, as Shadow/Highlight's (D-400). The mask view shows
    that key. The correction as D-398's up to its push; its push, with secondary_wheels "one",
    the secondary wheel and lightness for every tone as D-398's; with "three", Color Balance's
    rule (D-130) from e's luma: the shadow wheel weighted max(0, 1 - 2L), the highlight wheel
    max(0, 2L - 1), the secondary wheel (now the midtones') the rest, each A (C - L(C)) + K over
    200 as D-398's. With "one" the shadow and highlight wheels are not used, as Photo Filter's
    colour is not used but for "custom" (D-384).

**This file never runs the build's code path.** It works in double precision on lists,
straight from the drawing's 8-bit values, with Python's own colorsys for HLS, where the build
works on its single-precision buffers.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py): pure colours, greys,
a skin tone, orange and three warm tones, darker row by row. The projects go into
`Fixtures/color_grade_3`, the expected frames into
`Fixtures/color_grade_3/expected_color_grade_3.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/color_grade_3_reference.py
"""

import colorsys
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import color_grade_reference as G  # noqa: E402
import color_grade_2_reference as C2  # noqa: E402
import median_smart_blur_reference as MS  # noqa: E402
import shadow_highlight_reference as SH  # noqa: E402


W, H = B.W, B.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "color_grade_3"
SETTINGS = dict(C2.SETTINGS)
SETTINGS.update({
    "secondary_shadow_wheel_hue": (0, 360, 0), "secondary_shadow_wheel_amount": (0, 100, 0),
    "secondary_shadow_lightness": (-100, 100, 0),
    "secondary_highlight_wheel_hue": (0, 360, 0),
    "secondary_highlight_wheel_amount": (0, 100, 0),
    "secondary_highlight_lightness": (-100, 100, 0),
    "key_denoise": (0, 100, 0), "key_blur": (0, 100, 0),
})
NEW = [k for k in SETTINGS if k not in C2.SETTINGS]
LISTS = dict(C2.LISTS)
LISTS.update({"hue_hue": [], "hue_luma": [], "luma_saturation": [],
              "saturation_saturation": []})
WORDS = dict(C2.WORDS)
WORDS["secondary_wheels"] = "one"
NEW_KEYS = NEW + ["hue_hue", "hue_luma", "luma_saturation", "saturation_saturation",
                  "secondary_wheels"]

luma = G.luma
clamp = C2.clamp


# --- the rule -------------------------------------------------------------------------------

def line_curve(points):
    """A luma or saturation curve, eased from point to point and flat past its ends."""
    def at(x):
        if x <= points[0][0]:
            return points[0][1]
        if x >= points[-1][0]:
            return points[-1][1]
        for (x0, y0), (x1, y1) in zip(points, points[1:]):
            if x0 <= x < x1:
                return y0 + (y1 - y0) * C2.smooth((x - x0) / (x1 - x0))
        raise AssertionError(x)
    return at


def curve_steps(lists):
    """Steps 9a to 9d, those that change anything."""
    out = []
    hh = lists["hue_hue"]
    if any(p[1] != 0 for p in hh):
        turn = C2.hue_curve(hh)

        def hue_hue(e):
            h, l, s = colorsys.rgb_to_hls(*e)
            if max(e) == min(e):
                return e
            return list(colorsys.hls_to_rgb(((h * 360 + turn(h * 360)) % 360) / 360, l, s))
        out.append(hue_hue)
    hl = lists["hue_luma"]
    if any(p[1] != 0 for p in hl):
        lift = C2.hue_curve(hl)

        def hue_luma(e):
            if max(e) == min(e):
                return e
            k = lift(colorsys.rgb_to_hls(*e)[0] * 360) * (max(e) - min(e)) / 200
            return [x + k for x in e]
        out.append(hue_luma)
    for name, of in (("luma_saturation", lambda e: 100 * luma(e)),
                     ("saturation_saturation",
                      lambda e: 100 * min(1.0, colorsys.rgb_to_hls(*e)[2]))):
        pts = lists[name]
        if any(p[1] != 100 for p in pts):
            def sat(e, curve=line_curve(pts), of=of):
                lum = luma(e)
                k = curve(of(e)) / 100
                return [lum + (x - lum) * k for x in e]
            out.append(sat)
    return out


def three(words):
    return words["secondary_wheels"] == "three"


def correcting(v, words):
    return C2.correcting(v) or (three(words) and any(
        v[f"secondary_{n}_{k}"] != 0 for n in ("shadow", "highlight")
        for k in ("wheel_amount", "lightness")))


def correction(e, v, words):
    """D-398's correction, its push as `secondary_wheels` says."""
    c = C2.correction(e, dict(v, secondary_wheel_amount=0, secondary_lightness=0))
    mid = C2.wheel(v["secondary_wheel_hue"], v["secondary_wheel_amount"],
                   v["secondary_lightness"])
    ws = wh = 0.0
    sh = hi = [0.0] * 3
    if three(words):
        lum = luma(e)
        ws, wh = clamp(1 - 2 * lum), clamp(2 * lum - 1)
        sh, hi = (C2.wheel(v[f"secondary_{n}_wheel_hue"], v[f"secondary_{n}_wheel_amount"],
                           v[f"secondary_{n}_lightness"]) for n in ("shadow", "highlight"))
    wm = 1 - ws - wh
    return [x + (ws * a + wm * b + wh * d) / 200 for x, a, b, d in zip(c, sh, mid, hi)]


def refine(m, v):
    """The key through Median's disc of radius key_denoise / 10, then the Gaussian of sigma
    key_blur / 10, its edge places repeated."""
    plane = [m[y * W:(y + 1) * W] for y in range(H)]
    taps = MS.disc(v["key_denoise"] / 10)
    if len(taps) > 1:
        plane = [[MS.middle([plane[y + dy][x + dx] for dx, dy in taps
                             if 0 <= x + dx < W and 0 <= y + dy < H])
                  for x in range(W)] for y in range(H)]
    if v["key_blur"] > 0:
        plane = SH.held_blur(plane, v["key_blur"] / 10)
    return [u for row in plane for u in row]


def secondary_frame(v, words):
    invert, mask = words["key_invert"] == "on", words["key_view"] == "mask"

    def step(es):
        m = [C2.key(e, v, invert) if e is not None else 0.0 for e in es]
        if v["key_denoise"] > 0 or v["key_blur"] > 0:
            m = refine(m, v)
        if mask:
            return [None if e is None else [k] * 3 for e, k in zip(es, m)]
        return [None if e is None else [x + k * (y - x) for x, y in zip(e, correction(e, v, words))]
                for e, k in zip(es, m)]
    return step


def steps(v, look, lists, words):
    """Each step a function of the frame's encoded colours (None where a pixel does not show),
    in order."""
    v0 = dict(v, secondary_temperature=0, secondary_tint=0, secondary_contrast=0,
              secondary_saturation=100, secondary_wheel_amount=0, secondary_lightness=0)
    for n in ("shadow", "midtone", "highlight"):
        v0[f"{n}_wheel_amount"] = v0[f"{n}_lightness"] = 0
    early = C2.steps(v0, look, lists, {"key_invert": "off", "key_view": "off"})
    names = ("shadow", "midtone", "highlight")
    late = []
    if any(v[f"{n}_wheel_amount"] != 0 or v[f"{n}_lightness"] != 0 for n in names):
        pushes = [C2.wheel(v[f"{n}_wheel_hue"], v[f"{n}_wheel_amount"], v[f"{n}_lightness"])
                  for n in names]
        late.append(lambda e: C2.balance3(e, *pushes))
    each = [lambda es, f=f: [None if e is None else f(e) for e in es]
            for f in early + curve_steps(lists) + late]
    if words["key_view"] == "mask" or correcting(v, words):
        each.append(secondary_frame(v, words))
    return each


def color_grade(px, v, look, lists, words):
    ws = [R.working(p) for p in px]
    for step in steps(v, look, lists, words):
        es = [None if p[3] == 0 else [S.linear_to_srgb(clamp(w[c] / w[3])) for c in range(3)]
              for p, w in zip(px, ws)]
        ws = [w if e is None else [srgb_to_linear(clamp(u)) * w[3] for u in e] + [w[3]]
              for w, e in zip(ws, step(es))]
    if v["vignette_amount"] != 0 and words["key_view"] != "mask":
        for i, p in enumerate(px):
            if p[3] == 0:
                continue
            o, g = G.vignette(i % W, i // W, v)
            w = ws[i]
            a = w[3]
            ws[i] = [(w[c] / a + o * (g - w[c] / a)) * a for c in range(3)] + [a]
    return ws


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
        if not (c["old"] and k in NEW_KEYS):
            params[k] = setting_json(c["set"].get(k, start))
    for k, d in list(LISTS.items()) + list(WORDS.items()):
        if not (c["old"] and k in NEW_KEYS):
            params[k] = c["set"].get(k, d)
    p = B.project_json(fx, "core.color_grade", params, c["shift"])
    p["assets"].append({"id": "asset-look", "kind": "lut", "name": "look",
                        "path": "luts/look.cube"})
    return p


REDS = C2.REDS
SKIN_KEY = C2.SKIN_KEY
GREENS_TO_CYAN = [[0, 0], [120, 60], [240, 0]]

CASES = {
    "FX-GRADE3-001": ("Every setting as added, the new ones written: the drawing exactly as "
                      "it is.", case(), [0]),
    "FX-GRADE3-002": ("Hue versus hue, one point shifting 60 degrees: every colour turned a "
                      "sixth of the way round (red to yellow, green to cyan), its lightness and "
                      "saturation kept; greys, black and white untouched.",
                      case(hue_hue=[[0, 60]]), [0]),
    "FX-GRADE3-003": ("Hue versus hue through (0, 0), (120, 60), (240, 0): the greens turned "
                      "toward cyan, pure red and pure blue unchanged.",
                      case(hue_hue=GREENS_TO_CYAN), [0]),
    "FX-GRADE3-004": ("Hue versus hue, every shift 0, and every other new curve flat: no step, "
                      "the drawing exactly as it is.",
                      case(hue_hue=[[0, 0], [180, 0]], hue_luma=[[90, 0]],
                           luma_saturation=[[0, 100], [100, 100]],
                           saturation_saturation=[[50, 100]]), [0]),
    "FX-GRADE3-005": ("Hue versus luma, one point at 50: each colour lighter by a quarter of "
                      "its chroma (pure colours by 0.25), greys untouched.",
                      case(hue_luma=[[0, 50]]), [0]),
    "FX-GRADE3-006": ("Hue versus luma through (0, -60) and (180, 60): reds darker, cyans "
                      "lighter, greys untouched.",
                      case(hue_luma=[[0, -60], [180, 60]]), [0]),
    "FX-GRADE3-007": ("Luma versus saturation, one point at saturation 0: every colour its own "
                      "grey, whatever its luma.", case(luma_saturation=[[50, 0]]), [0]),
    "FX-GRADE3-008": ("Luma versus saturation through (20, 0) and (80, 150): colours darker "
                      "than luma 20 grey, lighter than 80 half as strong again, eased between.",
                      case(luma_saturation=[[20, 0], [80, 150]]), [0]),
    "FX-GRADE3-009": ("Saturation versus saturation through (0, 100), (50, 100), (100, 40): "
                      "the strong colours duller, the dull ones untouched.",
                      case(saturation_saturation=[[0, 100], [50, 100], [100, 40]]), [0]),
    "FX-GRADE3-010": ("Saturation versus saturation through (0, 200) and (40, 100): the dull "
                      "colours stronger, those of saturation 40 and up untouched.",
                      case(saturation_saturation=[[0, 200], [40, 100]]), [0]),
    "FX-GRADE3-011": ("Three secondary wheels, the key taking everything, the shadow wheel hue "
                      "200 at 60: the darks teal, white untouched.",
                      case(secondary_wheels="three", secondary_shadow_wheel_hue=200,
                           secondary_shadow_wheel_amount=60), [0]),
    "FX-GRADE3-012": ("Three secondary wheels: highlight wheel hue 40 at 50 with highlight "
                      "lightness -20, and the secondary (midtone) lightness 20: the lights "
                      "warmer and lower, the middle lighter, black untouched.",
                      case(secondary_wheels="three", secondary_highlight_wheel_hue=40,
                           secondary_highlight_wheel_amount=50,
                           secondary_highlight_lightness=-20, secondary_lightness=20), [0]),
    "FX-GRADE3-013": ("Three secondary wheels with only the secondary wheel, hue 30 at 50: "
                      "now the midtones' wheel, so black and white are untouched where one "
                      "wheel would move them.",
                      case(secondary_wheels="three", secondary_wheel_hue=30,
                           secondary_wheel_amount=50), [0]),
    "FX-GRADE3-014": ("One secondary wheel (as added) with the shadow and highlight wheels "
                      "set: they are not used, so nothing changes.",
                      case(secondary_shadow_wheel_hue=200, secondary_shadow_wheel_amount=60,
                           secondary_highlight_lightness=-40), [0]),
    "FX-GRADE3-015": ("FX-GRADE2-017's red key (softness 30) shown as the mask with denoise 30: "
                      "the key through a median of radius 3: red's lone keyed column, among "
                      "unkeyed neighbours, is outvoted, and the warm tones' key evens out "
                      "across their columns.",
                      case(key_view="mask", key_hue_softness=30, key_denoise=30, **REDS), [0]),
    "FX-GRADE3-016": ("The red key shown as the mask with blur 20: the key softened by a "
                      "Gaussian of sigma 2, red's column spread to its neighbours.",
                      case(key_view="mask", key_blur=20, **REDS), [0]),
    "FX-GRADE3-017": ("The softened red key (softness 30) with denoise 15 and blur 10, "
                      "secondary saturation 0: red's lone column outvoted (a quarter grey once "
                      "the blur brings some key back), the warm tones mostly grey, and blue "
                      "beside them partly grey where the blurred key reaches it.",
                      case(secondary_saturation=0, key_hue_softness=30, key_denoise=15,
                           key_blur=10, **REDS), [0]),
    "FX-GRADE3-018": ("Denoise 5, a radius of half a pixel: below 1, nothing; FX-GRADE2-012's "
                      "picture.", case(secondary_saturation=0, key_denoise=5, **REDS), [0]),
    "FX-GRADE3-019": ("The red key's mask with blur keyed from 0 at frame 0 to 40 at frame 4, "
                      "linear: a hard key at frame 0, sigma 2 at frame 2, sigma 4 at frame 4.",
                      case(key_view="mask", key_blur=keyed((0, 0), (4, 40)), **REDS),
                      [0, 2, 4]),
    "FX-GRADE3-020": ("Hue versus hue and a blurred key moved three pixels right: the same, "
                      "moved.",
                      case(shift=3, hue_hue=[[0, 60]], secondary_saturation=0, key_blur=10,
                           **REDS), [0]),
    "FX-GRADE3-021": ("A whole grade: D-397's and D-398's sections, the four new curves, a "
                      "skin key denoised and blurred with three wheels, and a dark vignette, "
                      "each in its order.",
                      case("asset-look", curve_master=C2.S_CURVE,
                           hue_saturation=[[0, 110], [120, 90], [240, 120]],
                           hue_hue=[[30, -10], [200, 20]], hue_luma=[[0, 10], [240, -20]],
                           luma_saturation=[[10, 80], [90, 110]],
                           saturation_saturation=[[0, 120], [80, 90]],
                           shadow_wheel_hue=200, shadow_wheel_amount=30, midtone_lightness=10,
                           secondary_wheels="three", secondary_saturation=80,
                           secondary_wheel_hue=200, secondary_wheel_amount=30,
                           secondary_shadow_wheel_hue=220, secondary_shadow_wheel_amount=20,
                           secondary_highlight_lightness=-10, key_denoise=12, key_blur=8,
                           **SKIN_KEY, **G.EVERYTHING), [0]),
    "FX-GRADE3-022": ("A D-398 file without the new settings, a red key at secondary "
                      "saturation 0: D-398's own picture.",
                      case(old=True, secondary_saturation=0, **REDS), [0]),
}

INVALID = {
    "FX-GRADE3-023": ("Hue versus hue with a shift of 181, past 180.",
                      case(hue_hue=[[0, 181]])),
    "FX-GRADE3-024": ("Hue versus hue with a hue of 360, a full turn.",
                      case(hue_hue=[[0, 10], [360, 20]])),
    "FX-GRADE3-025": ("Hue versus luma with a luma of -101, below -100.",
                      case(hue_luma=[[90, -101]])),
    "FX-GRADE3-026": ("Hue versus luma with its hues not rising.",
                      case(hue_luma=[[200, 10], [100, 20]])),
    "FX-GRADE3-027": ("Luma versus saturation with a luma of 101, past 100.",
                      case(luma_saturation=[[0, 100], [101, 50]])),
    "FX-GRADE3-028": ("Luma versus saturation with a point of three numbers.",
                      case(luma_saturation=[[0, 100, 5]])),
    "FX-GRADE3-029": ("Saturation versus saturation with a saturation of 201.",
                      case(saturation_saturation=[[50, 201]])),
    "FX-GRADE3-030": ("Saturation versus saturation of 17 points, past 16.",
                      case(saturation_saturation=[[i * 5, 100] for i in range(17)])),
    "FX-GRADE3-031": ("Secondary wheels \"two\", not \"one\" or \"three\".",
                      case(secondary_wheels="two")),
    "FX-GRADE3-032": ("Key denoise 101, above 100.", case(key_denoise=101)),
    "FX-GRADE3-033": ("Key blur -1, below 0.", case(key_blur=-1)),
    "FX-GRADE3-034": ("Secondary highlight lightness 101, above 100.",
                      case(secondary_highlight_lightness=101)),
}


def main():
    (OUT / "luts").mkdir(parents=True, exist_ok=True)
    (OUT / "luts" / "look.cube").write_bytes(G.look_text().encode("utf-8"))
    expected = B.write_cases(OUT, "color_grade_3", CASES, INVALID, render, plain, file_json)
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
    hue = lambda p: colorsys.rgb_to_hls(*enc(p))[0] * 360  # noqa: E731

    assert c["FX-GRADE3-001"]["0"] == drawn and c["FX-GRADE3-004"]["0"] == drawn
    assert c["FX-GRADE3-014"]["0"] == drawn and c["FX-GRADE3-019"]["0"] != drawn
    turned = c["FX-GRADE3-002"]["0"]
    assert near(enc(turned[at(8, 0)]), [1, 1, 0], 1e-6)  # red to yellow
    assert near(enc(turned[at(6, 0)]), [0, 1, 1], 1e-6)  # green to cyan
    assert same(turned, 1, 0) and same(turned, 2, 0) and same(turned, 3, 0)
    g2c = c["FX-GRADE3-003"]["0"]
    assert same(g2c, 8, 0) and same(g2c, 9, 0) and abs(hue(g2c[at(6, 0)]) - 180) < 1e-6
    lift = c["FX-GRADE3-005"]["0"]
    assert near(enc(lift[at(8, 0)]), [1, 0.25, 0.25], 1e-6) and same(lift, 3, 0)
    six = c["FX-GRADE3-006"]["0"]
    assert enc(six[at(8, 4)])[0] < B.encoded(px[at(8, 4)])[0]
    assert enc(six[at(5, 4)])[1] > B.encoded(px[at(5, 4)])[1] and same(six, 3, 4)
    for i, p in enumerate(c["FX-GRADE3-007"]["0"]):
        if p[3] > 0:
            assert grey(p), i
    eight = c["FX-GRADE3-008"]["0"]
    assert grey(eight[at(9, 9)])  # dark blue, luma well under 20
    assert not grey(eight[at(4, 0)])  # yellow, luma 93
    nine = c["FX-GRADE3-009"]["0"]
    assert grey(nine[at(8, 0)]) is False and enc(nine[at(8, 0)])[1] > 0  # red duller
    assert same(nine, 3, 0)
    ten = c["FX-GRADE3-010"]["0"]
    assert same(ten, 8, 0) and same(ten, 14, 0)  # saturations 100 and 57 kept
    assert not same(ten, 13, 0)  # saturation 20 stronger
    teal = c["FX-GRADE3-011"]["0"]
    assert same(teal, 2, 0) and enc(teal[at(12, 0)])[2] > B.encoded(px[at(12, 0)])[2]
    twelve = c["FX-GRADE3-012"]["0"]
    assert same(twelve, 1, 0) and luma(enc(twelve[at(3, 0)])) > luma(B.encoded(px[at(3, 0)]))
    assert enc(twelve[at(2, 0)])[2] < 1
    mid = c["FX-GRADE3-013"]["0"]
    assert same(mid, 1, 0) and same(mid, 2, 0) and not same(mid, 3, 0)
    one = render(case(secondary_wheel_hue=30, secondary_wheel_amount=50), 0)
    assert not near(one[at(1, 0)], R.working(px[at(1, 0)]))
    soft = render(case(key_view="mask", key_hue_softness=30, **REDS), 0)
    assert not like(c["FX-GRADE3-015"]["0"], soft)
    blur = c["FX-GRADE3-016"]["0"]
    assert 0 < enc(blur[at(7, 0)])[0] < 1 and 0 < enc(blur[at(8, 0)])[0] < 1
    assert enc(blur[at(8, 0)])[0] > enc(blur[at(7, 0)])[0]
    seventeen = c["FX-GRADE3-017"]["0"]
    assert not same(seventeen, 9, 0) and not grey(seventeen[at(9, 0)])
    assert not grey(seventeen[at(8, 0)])
    assert like(c["FX-GRADE3-018"]["0"], render(case(secondary_saturation=0, **REDS), 0))
    blurs = c["FX-GRADE3-019"]
    assert like(blurs["0"], render(case(key_view="mask", **REDS), 0))
    assert like(blurs["2"], render(case(key_view="mask", key_blur=20, **REDS), 0))
    moved = c["FX-GRADE3-020"]["0"]
    still = render(case(hue_hue=[[0, 60]], secondary_saturation=0, key_blur=10, **REDS), 0)
    for y in range(H):
        assert moved[at(3, y):at(0, y + 1)] == still[at(0, y):at(W - 3, y)]
    assert like(c["FX-GRADE3-022"]["0"], C2.render(C2.case(secondary_saturation=0, **REDS), 0))
    assert like(c["FX-GRADE3-022"]["0"], render(case(secondary_saturation=0, **REDS), 0))
    for fx, frames in c.items():
        for f in frames.values():
            for i, p in enumerate(f):
                assert p[3] == drawn[i][3] or fx == "FX-GRADE3-020"
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), fx
    print("checked")


if __name__ == "__main__":
    main()
