"""Change to Color, worked a second way.

D-197 adds `core.change_to_color`. It turns one colour of a drawing into another, with the
shades and lights painted in it, as a costume is recoloured for a second outfit or a night
scene: a red jacket, its shadow tone and its highlight all turn blue, and the skin beside it is
left alone. `from` is the colour to change, `#rrggbb` (#ff0000); `to` the colour it becomes
(#0080ff); `change`, what of `to` is taken: "hue" (as added), "hue_lightness", "hue_saturation"
or "hue_lightness_saturation"; `change_by`, "setting" (as added), each changed part set to
`to`'s, or "transforming", each shifted by how far `to` is from `from`; `hue_tolerance` (5),
`lightness_tolerance` (50) and `saturation_tolerance` (50), 0 to 100, how far from `from` a
colour may be and still change whole; `softness`, 0 to 100 (50), a band past each tolerance,
that part of it wide, over which the change fades out; and `view_matte`, "off" (as added) or
"on", which shows instead how much each pixel is changed, white for whole, black for not at
all. Colours are compared as hue, lightness and saturation (HLS). Pixels that do not show stay
as they are, and every pixel keeps its own covering. It is this program's own method, modelled
on After Effects' Change to Color, its settings kept; nothing is ported. Document 21 is the
rule in words; this file is the reference for the numbers document 25 pins against it.

The rule. The HLS of an encoded colour e: with mx, mn its largest and smallest channel,
lightness l = (mx + mn) / 2; saturation s = 0 when mx == mn, else (mx - mn) / (1 - |2 l - 1|);
hue h, in degrees, D-141's HSV hue, none when mx == mn. `from` and `to` are read from their
8-bit values / 255, giving (h_f, l_f, s_f) and (h_t, l_t, s_t). At a pixel with covering a > 0,
with e = linear_to_srgb(clamp(p.rgb / a, 0, 1)) its encoded straight colour and (h, l, s) its
HLS: d_h = min(|h - h_f|, 360 - |h - h_f|) / 180, or 1 when either hue is none; d_l = |l - l_f|;
d_s = |s - s_f|. With t_x the matching tolerance / 100 and w = softness / 100, each part keeps
k_x = 1 when d_x <= t_x, else 0 when w == 0, t_x == 0 or d_x >= t_x (1 + w), else
1 - (d_x - t_x) / (t_x w); the pixel's k = min(k_h, k_l, k_s). With `view_matte` "on" the output
is (srgb_to_linear(k) * a, three times, a). Otherwise a pixel with k = 0 is its input exactly;
else, with a hue of none read as 0, the changed HLS (h', l', s') starts as (h, l, s) and takes,
for "setting", h' = h_t, and l' = l_t when `change` names lightness, s' = s_t when it names
saturation; for "transforming", h' = (h + h_t - h_f) taken into [0, 360), l' = clamp(l + l_t -
l_f, 0, 1) and s' = clamp(s + s_t - s_f, 0, 1) as named. The colour c of (h', l', s'): C = (1 -
|2 l' - 1|) s', H = h' / 60, X = C (1 - |H mod 2 - 1|), the sector i = min(floor(H), 5) giving
(C, X, 0), (X, C, 0), (0, C, X), (0, X, C), (X, 0, C) or (C, 0, X), each plus l' - C / 2. The
output is (srgb_to_linear(clamp(e + k (c - e), 0, 1)) * a, a). The layer does not grow.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values, where the build works in single precision on its buffers.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says: a red in its shades and lights beside the colours near it, drawn below. The
drawing goes into `Fixtures/change_to_color/media`, the projects into
`Fixtures/change_to_color`, and the expected frames into
`Fixtures/change_to_color/expected_change_to_color.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/change_to_color_reference.py
"""

import json
import sys
from math import floor
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json, OVERSHOOT  # noqa: E402
from ease_reference import ease  # noqa: E402
from color_key_reference import hue  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402

W, H = R.W, R.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "change_to_color"
TOLERANCE = 2e-5  # document 25's default for a filter
RANGES = {"hue_tolerance": (0, 100), "lightness_tolerance": (0, 100),
          "saturation_tolerance": (0, 100), "softness": (0, 100)}
NAMES = ("from", "to", "change", "change_by", "hue_tolerance", "lightness_tolerance",
         "saturation_tolerance", "softness", "view_matte")
CLIFF = 1e-5  # the shared definitions' margin round a yes-or-no choice


# --- the rule -------------------------------------------------------------------------------

def hls(e):
    """Hue (None for a grey), lightness and saturation of an encoded colour."""
    mx, mn = max(e), min(e)
    l = (mx + mn) / 2
    s = 0.0 if mx == mn else (mx - mn) / (1 - abs(2 * l - 1))
    return hue(e), l, s


def rgb(h, l, s):
    """The encoded colour of a hue in [0, 360], a lightness and a saturation."""
    c = (1 - abs(2 * l - 1)) * s
    hh = h / 60
    x = c * (1 - abs(hh % 2 - 1))
    i = min(floor(hh), 5)
    r, g, b = ((c, x, 0), (x, c, 0), (0, c, x), (0, x, c), (x, 0, c), (c, 0, x))[i]
    m = l - c / 2
    return [r + m, g + m, b + m]


def distances(p, f):
    """How far a colour's hue, lightness and saturation are from `from`'s, each 0 to 1."""
    (h, l, s), (hf, lf, sf) = p, f
    dh = 1.0 if h is None or hf is None else min(abs(h - hf), 360 - abs(h - hf)) / 180
    return dh, abs(l - lf), abs(s - sf)


def part(d, t, w):
    """How much of the change one part lets through, 0 to 1."""
    if d <= t:
        return 1.0
    if w == 0 or t == 0 or d >= t * (1 + w):
        return 0.0
    return 1 - (d - t) / (t * w)


def changed(p, f, t, change, by):
    """The HLS a matched colour is turned to."""
    h, l, s = p
    hf, lf, sf = f
    ht, lt, st = t
    h, hf, ht = (v if v is not None else 0.0 for v in (h, hf, ht))
    if by == "setting":
        return (ht, lt if "lightness" in change else l, st if "saturation" in change else s)
    return ((h + ht - hf) % 360,
            min(1.0, max(0.0, l + lt - lf)) if "lightness" in change else l,
            min(1.0, max(0.0, s + st - sf)) if "saturation" in change else s)


def change_to_color(pixels, frm, to, change, by, th, tl, ts, softness, matte):
    """`pixels` are 8-bit straight RGBA."""
    f = hls([v / 255 for v in R.hex_color(frm.lower())])
    t = hls([v / 255 for v in R.hex_color(to.lower())])
    tols, w = (th / 100, tl / 100, ts / 100), softness / 100
    out = [R.working(p) for p in pixels]
    for px in out:
        a = px[3]
        if a <= 0:
            continue
        e = [S.linear_to_srgb(min(1.0, max(0.0, v / a))) for v in px[:3]]
        p = hls(e)
        k = min(part(d, tx, w) for d, tx in zip(distances(p, f), tols))
        if matte == "on":
            px[:3] = [srgb_to_linear(k) * a] * 3
        elif k > 0:
            c = rgb(*changed(p, f, t, change, by))
            px[:3] = [srgb_to_linear(min(1.0, max(0.0, u + k * (v - u)))) * a for u, v in zip(e, c)]
    return out


# --- the drawing ----------------------------------------------------------------------------

LINE = R.LINE                        # #1e1a24, hue 264
RED = R.TRACE                        # #c82828, hue 0, lightness 0.47, saturation 2/3
SHADOW = (140, 30, 40, 255)          # #8c1e28, the red's shadow tone, hue 354.5
LIGHT = (240, 140, 140, 255)         # #f08c8c, the red's highlight, hue 0, lightness 0.75
DARK = (64, 8, 8, 255)               # #400808, a very dark red, lightness 0.14
MUTED = (128, 96, 96, 255)           # #806060, a dull red, saturation 1/7
PINK = (255, 0, 64, 255)             # #ff0040, hue 344.9, 15.1 degrees from red
SHADE = (220, 160, 140, 255)         # #dca08c, the skin's shadow, hue 15
SKIN = R.SKIN                        # #f6d6be, hue 25.7
GREEN = (64, 192, 64, 255)           # #40c040, hue 120
BLUE = (64, 96, 255, 255)            # #4060ff, hue 230
GREY = (128, 128, 128, 255)          # #808080, no hue
SOFT_RED = RED[:3] + (128,)          # the red at half covering, a soft edge
SOFT_BLUE = BLUE[:3] + (128,)        # the blue at half covering
NONE = S.NONE
SWATCHES = {3: RED, 4: SHADOW, 5: LIGHT, 6: DARK, 7: MUTED, 8: PINK, 9: SHADE, 10: SKIN,
            11: GREEN, 12: BLUE, 13: GREY}


def swatches(x, y):
    """A box of line in columns 2 to 14 and rows 1 to 8 holding, in rows 2 to 7, one column
    each of the red, its shadow, its highlight, the dark red, the dull red, the pink, the skin's
    shadow, the skin, green, blue and grey (columns 3 to 13); down its left side, column 1, the
    red at half covering in rows 1 to 4 and the blue at half covering in rows 5 to 8. Column 0,
    column 15 and rows 0 and 9 are empty."""
    if not (1 <= x <= 14 and 1 <= y <= 8):
        return NONE
    if x == 1:
        return SOFT_RED if y <= 4 else SOFT_BLUE
    if x in (2, 14) or y in (1, 8):
        return LINE
    return SWATCHES[x]


DRAWINGS = {"swatches": [[swatches(x, y) for x in range(W)] for y in range(H)]}


# --- the cases ------------------------------------------------------------------------------

def case(frm="#ff0000", to="#0080ff", change="hue", change_by="setting", hue_tolerance=5,
         lightness_tolerance=50, saturation_tolerance=50, softness=50, view_matte="off",
         shift=0):
    return {"drawing": "swatches", "from": frm, "to": to, "change": change,
            "change_by": change_by, "hue_tolerance": hue_tolerance,
            "lightness_tolerance": lightness_tolerance,
            "saturation_tolerance": saturation_tolerance, "softness": softness,
            "view_matte": view_matte, "shift": shift}


def mine(**kw):
    """A case changing the drawing's own red, #c82828."""
    return case(frm="#c82828", **kw)


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    return min(hi, max(lo, value_at(c[k], frame_no)))


def settings(c, frame_no):
    return tuple(c[k] if k not in RANGES else held(c, k, frame_no) for k in NAMES)


def render(c, frame_no):
    pixels = [p for row in DRAWINGS[c["drawing"]] for p in row]
    return R.frame(change_to_color(pixels, *settings(c, frame_no)), c["shift"])


def plain(c):
    """The drawing untouched, moved as the case moves it."""
    return R.frame([R.working(p) for row in DRAWINGS[c["drawing"]] for p in row], c["shift"])


CASES = {
    "FX-CTC-001": ("The settings as they start: from #ff0000 to #0080ff, Hue, by setting, hue "
                   "tolerance 5, lightness and saturation tolerance 50, softness 50. The red, "
                   "its shadow 5.5 degrees round the wheel, its highlight, the dark red and the "
                   "red at half covering take #0080ff's hue, each keeping its own lightness and "
                   "saturation, so the shading stays; the dull red, too far from #ff0000's "
                   "saturation, the pink and the skin's shadow, 15 degrees round, and every other "
                   "colour are untouched.",
                   case(), [0]),
    "FX-CTC-002": ("From #c82828, the drawing's own red: the same, and now the dull red, its "
                   "saturation 0.524 from the red's, just past the tolerance of 0.5, is changed "
                   "nine tenths of the way.",
                   mine(), [0]),
    "FX-CTC-003": ("Change Hue & Lightness: the changed colours also take #0080ff's lightness, "
                   "0.5, so the shadow, the highlight and the dark red lose their light and dark.",
                   mine(change="hue_lightness"), [0]),
    "FX-CTC-004": ("Change Hue & Saturation: the changed colours take #0080ff's saturation, 1, "
                   "each keeping its own lightness.",
                   mine(change="hue_saturation"), [0]),
    "FX-CTC-005": ("Change Hue, Lightness & Saturation: every changed colour becomes #0080ff "
                   "itself; the dull red nine tenths of the way there.",
                   mine(change="hue_lightness_saturation"), [0]),
    "FX-CTC-006": ("Change By transforming, Hue: each changed hue is turned by as far as "
                   "#0080ff's is from #c82828's, 209.9 degrees, so the shadow, 354.5 degrees, "
                   "lands at 204.4, not 209.9.",
                   mine(change_by="transforming"), [0]),
    "FX-CTC-007": ("Change By transforming, Hue, Lightness & Saturation: each changed colour's "
                   "lightness is raised by 0.029 and its saturation by 1/3, held at 1, as "
                   "#0080ff is lighter and fuller than #c82828.",
                   mine(change="hue_lightness_saturation", change_by="transforming"), [0]),
    "FX-CTC-008": ("Hue tolerance 0, softness 0: only a hue exactly the red's changes; the red, "
                   "the highlight, the dark red and the soft red do, the shadow does not, and "
                   "the dull red, at softness 0, is past its saturation tolerance.",
                   mine(hue_tolerance=0, softness=0), [0]),
    "FX-CTC-009": ("Hue tolerance 10, softness 0, 18 degrees each way: the pink and the skin's "
                   "shadow, 15 degrees off, change whole with the reds; the skin, 25.7 off, "
                   "does not.",
                   mine(hue_tolerance=10, softness=0), [0]),
    "FX-CTC-010": ("Softness 0 at the other settings as they start: FX-CTC-002 with the dull "
                   "red untouched.",
                   mine(softness=0), [0]),
    "FX-CTC-011": ("Softness 100: the band past each tolerance is as wide as the tolerance, so "
                   "the pink changes a third of the way, the skin's shadow a third, and the "
                   "dull red 95 hundredths.",
                   mine(softness=100), [0]),
    "FX-CTC-012": ("View Correction Matte on, at FX-CTC-002's settings: every pixel that shows "
                   "is grey, white where the colour changes whole, black where not at all, and "
                   "the dull red's nine tenths between, each at its own covering.",
                   mine(view_matte="on"), [0]),
    "FX-CTC-013": ("To #808080, a grey, with Hue & Saturation: the reds turn grey, each at its "
                   "own lightness.",
                   mine(to="#808080", change="hue_saturation"), [0]),
    "FX-CTC-014": ("From #808080, a grey, has no hue: no colour is near it, and the drawing is "
                   "untouched.",
                   case(frm="#808080"), [0]),
    "FX-CTC-015": ("All three tolerances 100: every colour takes #0080ff's hue, each keeping "
                   "its lightness and saturation; the grey, having no saturation, stays grey.",
                   mine(hue_tolerance=100, lightness_tolerance=100, saturation_tolerance=100),
                   [0]),
    "FX-CTC-016": ("Hue tolerance keyed from 0 at frame 0 to 20 at frame 4, linear: frame 0 "
                   "changes only the hues exactly red's, fading nothing in, since a tolerance "
                   "of 0 has no band; frame 2, at 10, takes in the pink and the skin's shadow "
                   "and a seventh of the skin; frame 4, at 20, the skin whole.",
                   mine(hue_tolerance=keyed((0, 0), (4, 20))), [0, 2, 4]),
    "FX-CTC-017": ("Lightness tolerance keyed from 10 at frame 0 to 100 at frame 4, eased "
                   "past its end: frame 0 changes the red whole, the shadow, 0.137 from its "
                   "lightness, a quarter, and neither the highlight nor the dark red; frame 2 is "
                   "held at 100 and is frame 4.",
                   mine(lightness_tolerance=keyed((0, 10, OVERSHOOT), (4, 100))), [0, 2, 4]),
    "FX-CTC-018": ("FX-CTC-002 moved three pixels right: the same, moved.",
                   mine(shift=3), [0, 3]),
    "FX-CTC-019": ("FX-CTC-002 with its colours written in capitals, #C82828 and #0080FF: the "
                   "same.",
                   case(frm="#C82828", to="#0080FF"), [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-CTC-020": ("Hue tolerance 101, above 100.", mine(hue_tolerance=101)),
    "FX-CTC-021": ("Softness -1, below 0.", mine(softness=-1)),
    "FX-CTC-022": ("Saturation tolerance keyed to 150 at frame 4.",
                   mine(saturation_tolerance=keyed((0, 50), (4, 150)))),
    "FX-CTC-023": ("A from colour written \"#ff00\", two digits short.", case(frm="#ff00")),
    "FX-CTC-024": ("A to colour written \"blue\", a name, not #rrggbb.", mine(to="blue")),
    "FX-CTC-025": ("A change written \"saturation\", not one of the four.",
                   mine(change="saturation")),
    "FX-CTC-026": ("A change by written \"shift\", not \"setting\" or \"transforming\".",
                   mine(change_by="shift")),
    "FX-CTC-027": ("A view matte written \"yes\", not \"off\" or \"on\".", mine(view_matte="yes")),
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
        "instance_id": "fx-0-0", "type_id": "core.change_to_color", "enabled": True,
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

    (OUT / "expected_change_to_color.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                       encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda u, v, e=1e-9: all(abs(p - q) < e for p, q in zip(u, v))  # noqa: E731
    enc8 = lambda q: [v / 255 for v in q[:3]]  # noqa: E731
    enc = lambda f, xy: [S.linear_to_srgb(min(1.0, max(0.0, v / f[at(*xy)][3])))  # noqa: E731
                         for v in f[at(*xy)][:3]]
    red, shadow, light, dark, muted, pink, shade, skin, green, blue, grey = (
        (x, 4) for x in range(3, 14))
    line, soft_red, soft_blue = (2, 4), (1, 2), (1, 6)
    BANDS = {(x, 4): SWATCHES[x] for x in range(3, 14)}
    BANDS.update({line: LINE, soft_red: RED, soft_blue: BLUE})
    reds = (red, shadow, light, dark, soft_red)
    others = (pink, shade, skin, green, blue, grey, line, soft_blue)
    TO = hls(enc8(R.hex_color("#0080ff")))
    MINE = hls(enc8(RED))
    same = lambda f, xys: all(f[at(*xy)] == drawn[at(*xy)] for xy in xys)  # noqa: E731
    got = lambda f, xy: hls(enc(f, xy))  # noqa: E731

    # The rule's own pieces: HLS both ways, the drawing's hues, lightnesses and saturations.
    for q in list(BANDS.values()) + [(0, 128, 255), (255, 0, 0)]:
        h, l, s = hls(enc8(q))
        assert near(rgb(h or 0.0, l, s), enc8(q), 1e-12), q
    assert abs(TO[0] - 60 * (4 - 128 / 255)) < 1e-12 and TO[1:] == (0.5, 1.0)
    assert abs(MINE[1] - 240 / 510) < 1e-12 and abs(MINE[2] - 2 / 3) < 1e-12 and MINE[0] == 0
    assert abs(hls(enc8(SHADOW))[0] - (360 - 60 / 11)) < 1e-12
    assert abs(hls(enc8(MUTED))[2] - 1 / 7) < 1e-12 and hls(enc8(GREY)) == (None, 128 / 255, 0.0)
    assert part(0.1, 0.1, 0) == 1 and part(0.1 + 1e-9, 0.1, 0) == 0 and part(0, 0, 0.5) == 1
    assert part(1e-9, 0, 0.5) == 0 and abs(part(0.15, 0.1, 1) - 0.5) < 1e-12
    assert part(0.2, 0.1, 1) == 0 and abs(part(0.525, 0.5, 0.5) - 0.9) < 1e-12
    assert rgb(360, 0.5, 1) == rgb(0, 0.5, 1) == [1.0, 0.0, 0.0]

    # A pixel that does not show, and the covering, are kept by every case; a pixel at half
    # covering is treated as its colour at full covering.
    for fx, frames in c.items():
        base = plain(case(shift=3 if fx == "FX-CTC-018" else 0))
        for px in frames.values():
            for i in range(W * H):
                assert px[i][3] == base[i][3], (fx, i)
                if base[i][3] == 0:
                    assert px[i] == [0.0] * 4, (fx, i)
                assert all(-1e-12 <= v <= px[i][3] + 1e-12 for v in px[i][:3]), (fx, i)
            if fx != "FX-CTC-018":
                for s, full in ((soft_red, red), (soft_blue, blue)):
                    assert near(enc(px, s), enc(px, full)), (fx, s)

    # The cliffs: where a part has no band, no pixel's distance is within 1e-5 of its
    # tolerance, but for a distance of exactly 0 at a tolerance of 0.
    for fx, (_, cs, frames) in CASES.items():
        for f in frames:
            st = settings(cs, f)
            fr = hls(enc8(R.hex_color(st[0].lower())))
            for q in BANDS.values():
                for i, (d, t) in enumerate(zip(distances(hls(enc8(q)), fr), st[4:7])):
                    if st[7] == 0 or t == 0:
                        # A hue of exactly 0 comes back exactly 0, its green and blue equal.
                        assert (i == 0 and d == t == 0 and q[1] == q[2]) or \
                            abs(d - t / 100) > CLIFF, (fx, q, d, t)

    one = c["FX-CTC-001"]["0"]
    for xy in reds:
        h, l, s = got(one, xy)
        h0, l0, s0 = hls(enc8(BANDS[xy]))
        assert abs(h - TO[0]) < 1e-6 and abs(l - l0) < 1e-9 and abs(s - s0) < 1e-9, xy
    assert same(one, (muted,) + others)
    two = c["FX-CTC-002"]["0"]
    assert all(two[at(*xy)] == one[at(*xy)] for xy in reds + others)
    k = part(abs(1 / 7 - 2 / 3), 0.5, 0.5)
    assert abs(k - (1 - (11 / 21 - 1 / 2) / 0.25)) < 1e-12 and 0.9 < k < 0.91
    want = [u + k * (v - u) for u, v in zip(enc8(MUTED), rgb(TO[0], *hls(enc8(MUTED))[1:]))]
    assert near(enc(two, muted), want)
    three = c["FX-CTC-003"]["0"]
    assert all(abs(got(three, xy)[1] - 0.5) < 1e-9 for xy in reds) and same(three, others)
    four = c["FX-CTC-004"]["0"]
    assert all(abs(got(four, xy)[2] - 1) < 1e-9 for xy in reds) and same(four, others)
    assert all(abs(got(four, xy)[1] - hls(enc8(BANDS[xy]))[1]) < 1e-9 for xy in reds)
    five = c["FX-CTC-005"]["0"]
    assert all(near(enc(five, xy), enc8((0, 128, 255))) for xy in reds) and same(five, others)
    six = c["FX-CTC-006"]["0"]
    assert abs(got(six, shadow)[0] - (360 - 60 / 11 + TO[0] - 360)) < 1e-6
    assert all(six[at(*xy)] == two[at(*xy)] for xy in (red, light, dark, soft_red) + others)
    seven = c["FX-CTC-007"]["0"]
    for xy in reds:
        h0, l0, s0 = hls(enc8(BANDS[xy]))
        assert abs(got(seven, xy)[1] - (l0 + 0.5 - MINE[1])) < 1e-9, xy
        assert abs(got(seven, xy)[2] - min(1, s0 + 1 / 3)) < 1e-9, xy
    eight = c["FX-CTC-008"]["0"]
    assert all(eight[at(*xy)] == two[at(*xy)] for xy in (red, light, dark, soft_red))
    assert same(eight, (shadow, muted) + others)
    nine = c["FX-CTC-009"]["0"]
    assert all(abs(got(nine, xy)[0] - TO[0]) < 1e-6 for xy in reds + (pink, shade))
    assert same(nine, (muted, skin, green, blue, grey, line, soft_blue))
    ten = c["FX-CTC-010"]["0"]
    assert same(ten, (muted,)) and all(ten[at(*xy)] == two[at(*xy)] for xy in reds + others)
    eleven = c["FX-CTC-011"]["0"]
    for xy, want in ((pink, 1 - (60 * 64 / 255 / 180 - 0.05) / 0.05),
                     (shade, 1 - (15 / 180 - 0.05) / 0.05),
                     (muted, 1 - (11 / 21 - 0.5) / 0.5)):
        kk = min(part(d, t, 1) for d, t in zip(distances(hls(enc8(BANDS[xy])), MINE),
                                                (0.05, 0.5, 0.5)))
        assert abs(kk - want) < 1e-12, (xy, kk, want)
    assert same(eleven, (skin, green, blue, grey, line, soft_blue))
    twelve = c["FX-CTC-012"]["0"]
    for xy in reds:
        assert near(twelve[at(*xy)], [drawn[at(*xy)][3]] * 4), xy
    for xy in others:
        assert twelve[at(*xy)][:3] == [0.0] * 3, xy
    assert near(enc(twelve, muted), [k] * 3)
    thirteen = c["FX-CTC-013"]["0"]
    for xy in reds:
        e = enc(thirteen, xy)
        assert max(e) - min(e) < 1e-9 and abs(e[0] - hls(enc8(BANDS[xy]))[1]) < 1e-9, xy
    assert same(thirteen, others)
    assert c["FX-CTC-014"]["0"] == drawn
    fifteen = c["FX-CTC-015"]["0"]
    for xy in BANDS:
        if xy == grey:
            assert near(fifteen[at(*xy)], drawn[at(*xy)], 1e-12)
        else:
            assert abs(got(fifteen, xy)[0] - TO[0]) < 1e-6, xy
    sixteen = c["FX-CTC-016"]
    assert sixteen["0"] == render(mine(hue_tolerance=0), 0)
    assert all(sixteen["0"][at(*xy)] == eight[at(*xy)] for xy in (red, light, dark, soft_red))
    assert same(sixteen["0"], (shadow, pink, shade, skin))
    assert sixteen["2"] == render(mine(hue_tolerance=10), 0)
    assert abs(min(part(d, t, 0.5) for d, t in zip(distances(hls(enc8(SKIN)), MINE),
                                                  (0.1, 0.5, 0.5))) - 1 / 7) < 2e-3
    assert sixteen["4"] == render(mine(hue_tolerance=20), 0)
    assert abs(got(sixteen["4"], skin)[0] - TO[0]) < 1e-6
    seventeen = c["FX-CTC-017"]
    assert ease(OVERSHOOT, 0.5) > 1 and value_at(mine(lightness_tolerance=keyed(
        (0, 10, OVERSHOOT), (4, 100)))["lightness_tolerance"], 2) > 100
    assert all(seventeen["0"][at(*xy)] == two[at(*xy)] for xy in (red, soft_red, muted))
    assert same(seventeen["0"], (light, dark) + others)
    ks = min(part(d, t, 0.5) for d, t in zip(distances(hls(enc8(SHADOW)), MINE),
                                             (0.05, 0.1, 0.5)))
    assert abs(ks - (1 - (240 / 510 - 1 / 3 - 0.1) / 0.05)) < 1e-12 and 0.25 < ks < 0.26
    assert seventeen["2"] == seventeen["4"] == render(mine(lightness_tolerance=100), 0)
    moved = c["FX-CTC-018"]
    assert moved["0"] == moved["3"]
    for y in range(H):
        assert moved["0"][at(3, y):at(0, y + 1)] == two[at(0, y):at(W - 3, y)]
    assert c["FX-CTC-019"]["0"] == two
    print("checked")


if __name__ == "__main__":
    main()
