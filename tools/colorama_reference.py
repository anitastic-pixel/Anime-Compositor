"""Colorama, worked a second way, with the controls D-381 adds.

`core.colorama`, modelled on After Effects' Colorama: every pixel's place round a ring of colours
is read from one of its properties, and the pixel takes the colour at that place. D-316 built
the reduced effect (document 21 has its rule); D-381 adds the rest of the controls After
Effects' manual names: more properties to read the phase from, a separate property and an add
mode for the Add Phase layer, interpolation of the palette on or off, an opacity for each colour
of the ring, what the ring modifies, Modify Alpha and Change Empty Pixels, a matching colour that
picks the pixels changed, a mask layer, and Composite Over Layer. Adobe publishes no formula, so
the rule below is ours. Document 21 is the rule in words; this file is the reference for the
numbers document 25 pins against it.

The rule. D-316's settings, unchanged: `get_phase`, `layer` and `fit`, `phase_shift`,
`cycle_repetitions`, `stops`, `color_1` to `color_5`, `blend_with_original`. D-381's, each one
that a file leaves out meaning what the effect did before it:

- `get_phase`, now also "hue", "lightness", "saturation" (HSL of the encoded straight colour,
  the hue a turn as 0 to 1, a grey's 0), "value" (the largest encoded channel) and "zero" (0).
- `add_phase_from`, the same words, the property read from the Add Phase layer; left out, the
  same as `get_phase`. Times the layer pixel's covering unless it is "alpha".
- `add_mode`, "wrap", "clamp", "average" or "screen", "wrap": with a layer named, the pixel's
  phase P and the layer's Q make P + Q, min(P + Q, 1), (P + Q) / 2 or P + Q - P Q.
- `interpolate`, "on" or "off", "on": off, the colour is the ring colour C_i itself.
- `opacity_1` to `opacity_5`, 0 to 100, 100, round the ring as the colours are.
- `modify`, "all", "hue", "lightness", "saturation", "red", "green", "blue" or "none", "all":
  what of the ring colour m the pixel takes, its encoded straight colour e keeping the rest; hue,
  lightness and saturation as HSL, one channel as itself, "none" e as it is.
- `modify_alpha`, "off" or "on", "off": on, the pixel's new covering is the ring's opacity.
- `change_empty`, "off" or "on", "off": with modify alpha on, the empty pixels are worked too,
  as black with phase 0.
- `matching_mode`, "off", "rgb", "hue" or "chroma", "off"; `matching_color`, "#ffffff";
  `matching_tolerance` and `matching_softness`, 0 to 100, 15 and 0: Change Color's distance
  (D-370) of e from the colour, d, gives the weight 1 within the tolerance, falling straight to
  0 across the softness, 0 beyond. Off, the weight is 1.
- `mask_layer`, a second layer setting, "" none, placed with `fit` as the first is; and
  `masking_mode`, "luminance", "inverse_luminance", "alpha" or "inverse_alpha", "luminance":
  the mask pixel's encoded luminance times its covering, its covering, or one less either. A
  layer not in the composition is no mask, with EFFECT_LAYER_MISSING.
- `composite_over`, "on" or "off", "on".

At a pixel with covering a (worked when a > 0, or always with Change Empty on), b its straight
linear colour (black when empty), e = encoded b held inside 0 to 1: the place
t = (phase) repetitions + shift / 360, u = (t - floor t) n, i = floor u, f = u - i (0 with
interpolate off); m = C_i + f (C_(i+1) - C_i) encoded and the opacity alike; the modified
colour g, back to linear; R = (g a', a') with a' the opacity or a; O = the pixel as it was;
w = the matching weight times the mask's. Composite on X = O + w (R - O), off X = w R; the
output is X + (blend / 100)(O - X), all premultiplied. Every one at its start, this is D-316's
rule exactly.

**This file never runs the build's code path.** It works in double precision on lists, from
the drawings' exact 8-bit values, with Python's colorsys for HSL and HSV, where the build works
on its single-precision buffers. It asserts no pixel's place lies within a ten-thousandth of a
stop where interpolation is off, so the build's rounding cannot pick the other colour.

Every case is Broadcast Safe's drawing (tools/broadcast_safe_reference.py) as the layer `art`,
16 by 10, five frames; the cases with layers add `phase`, a hidden 16 by 10 drawing of red
rising left to right and green top to bottom, its bottom two rows half covered, and `mask`, a
hidden 16 by 10 drawing white on the left, grey in the middle, and on the right white whose
covering is 0 in the top half and a half below. The projects go into `Fixtures/colorama`, the
expected frames into `Fixtures/colorama/expected_colorama.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/colorama_reference.py
"""

import colorsys
import math
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from adjust_reference import srgb_to_linear  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as R  # noqa: E402
import broadcast_safe_reference as B  # noqa: E402
import effect_layer_reference as L  # noqa: E402

W, H, FRAMES = B.W, B.H, 5
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "colorama"
NUMBERS = {"phase_shift": (-3600, 3600), "cycle_repetitions": (0, 100), "stops": (2, 5),
           "blend_with_original": (0, 100), "matching_tolerance": (0, 100),
           "matching_softness": (0, 100), **{f"opacity_{i}": (0, 100) for i in range(1, 6)}}
OLD = ("get_phase", "layer", "fit", "phase_shift", "cycle_repetitions", "stops", "color_1",
       "color_2", "color_3", "color_4", "color_5", "blend_with_original")


# --- the drawings for the layers ------------------------------------------------------------

def phase_px(x, y):
    return (round(255 * x / 15), round(255 * y / 9), 64, 128 if y >= 8 else 255)


def mask_px(x, y):
    if x < 6:
        return (255, 255, 255, 255)
    if x < 11:
        return (128, 128, 128, 255)
    return (255, 255, 255, 0 if y < 5 else 128)


DRAWINGS = {"phase": [[phase_px(x, y) for x in range(W)] for y in range(H)],
            "mask": [[mask_px(x, y) for x in range(W)] for y in range(H)]}


# --- the rule -------------------------------------------------------------------------------

def hexc(s):
    return [int(s[i:i + 2], 16) / 255 for i in (1, 3, 5)]


def straight(p):
    """A drawing pixel's covering, straight linear colour and encoded colour, from its 8 bits."""
    a = p[3] / 255
    if a == 0:
        return 0.0, [0.0] * 3, [0.0] * 3
    e = [v / 255 for v in p[:3]]
    return a, [srgb_to_linear(v) for v in e], e


def phase(word, p):
    a, b, e = straight(p)
    if word == "alpha":
        return a
    if a == 0 or word == "zero":
        return 0.0
    if word in ("red", "green", "blue"):
        return e[("red", "green", "blue").index(word)]
    if word == "luminance":
        return S.linear_to_srgb(0.2126 * b[0] + 0.7152 * b[1] + 0.0722 * b[2])
    if word == "value":
        return max(e)
    h, l, s = colorsys.rgb_to_hls(*e)
    return {"hue": h, "lightness": l, "saturation": s}.get(word, sum(e) / 3)


def distance(e, c, mode):
    if mode == "rgb":
        return (sum((u - v) ** 2 for u, v in zip(e, c)) / 3) ** 0.5
    if mode == "chroma":
        def cbcr(v):
            y = 0.2126 * v[0] + 0.7152 * v[1] + 0.0722 * v[2]
            return (v[2] - y) / 1.8556, (v[0] - y) / 1.5748
        (p0, p1), (q0, q1) = cbcr(e), cbcr(c)
        return ((p0 - q0) ** 2 + (p1 - q1) ** 2) ** 0.5
    if max(e) == min(e) or max(c) == min(c):
        return 1.0
    d = abs(colorsys.rgb_to_hsv(*e)[0] - colorsys.rgb_to_hsv(*c)[0]) * 360
    return min(d, 360 - d) / 180


def weight(d, t, s):
    if d <= t:
        return 1.0
    if s == 0 or d >= t + s:
        return 0.0
    return 1 - (d - t) / s


def modified(e, m, word):
    if word == "all":
        return list(m)
    if word == "none":
        return list(e)
    if word in ("red", "green", "blue"):
        i = ("red", "green", "blue").index(word)
        return [m[c] if c == i else e[c] for c in range(3)]
    he, le, se = colorsys.rgb_to_hls(*e)
    hm, lm, sm = colorsys.rgb_to_hls(*m)
    h, l, s = {"hue": (hm, le, se), "lightness": (he, lm, se), "saturation": (he, le, sm)}[word]
    return list(colorsys.hls_to_rgb(h, l, s))


EDGES = []  # the distance of each place to its nearest stop, where interpolation is off


def colorama(px, s, maps):
    n = int(s["stops"])
    ring = [hexc(s[f"color_{i + 1}"]) for i in range(n)]
    ops = [s[f"opacity_{i + 1}"] / 100 for i in range(n)]
    o = s["blend_with_original"] / 100
    add, mask = maps.get("add"), maps.get("mask")
    out = []
    for k, p in enumerate(px):
        a, b, e = straight(p)
        O = R.working(p)
        empty = s["modify_alpha"] == "on" and s["change_empty"] == "on"
        if a == 0 and not empty:
            out.append(O)
            continue
        ph = phase(s["get_phase"], p)
        if add is not None:
            q = add[k]
            qq = phase(s["add_phase_from"], q) * (1 if s["add_phase_from"] == "alpha" else q[3] / 255)
            ph = {"wrap": ph + qq, "clamp": min(ph + qq, 1.0), "average": (ph + qq) / 2,
                  "screen": ph + qq - ph * qq}[s["add_mode"]]
        t = ph * s["cycle_repetitions"] + s["phase_shift"] / 360
        u = (t - math.floor(t)) * n
        i = int(u) % n
        f = u - int(u)
        if s["interpolate"] == "off":
            EDGES.append(min(f, 1 - f))
            f = 0.0
        j = (i + 1) % n
        m = [ring[i][c] + f * (ring[j][c] - ring[i][c]) for c in range(3)]
        op = ops[i] + f * (ops[j] - ops[i])
        g = [srgb_to_linear(min(1.0, max(0.0, v))) for v in modified(e, m, s["modify"])]
        a2 = op if s["modify_alpha"] == "on" else a
        Rp = [v * a2 for v in g] + [a2]
        w = 1.0
        if s["matching_mode"] != "off":
            d = distance(e, hexc(s["matching_color"]), s["matching_mode"])
            w *= weight(d, s["matching_tolerance"] / 100, s["matching_softness"] / 100)
        if mask is not None:
            q = mask[k]
            lum = phase("luminance", q) * q[3] / 255
            w *= {"luminance": lum, "inverse_luminance": 1 - lum, "alpha": q[3] / 255,
                  "inverse_alpha": 1 - q[3] / 255}[s["masking_mode"]]
        if s["composite_over"] == "on":
            X = [u0 + w * (r - u0) for u0, r in zip(O, Rp)]
        else:
            X = [w * r for r in Rp]
        out.append([x + o * (u0 - x) for x, u0 in zip(X, O)])
    return out


# --- the cases ------------------------------------------------------------------------------

START = {"get_phase": "intensity", "layer": "", "fit": "stretch", "phase_shift": 0,
         "cycle_repetitions": 1, "stops": 5, "color_1": "#ff0000", "color_2": "#ccff00",
         "color_3": "#00ff66", "color_4": "#0066ff", "color_5": "#cc00ff",
         "blend_with_original": 0, "add_phase_from": "intensity", "add_mode": "wrap",
         "interpolate": "on", **{f"opacity_{i}": 100 for i in range(1, 6)}, "modify": "all",
         "modify_alpha": "off", "change_empty": "off", "matching_mode": "off",
         "matching_color": "#ffffff", "matching_tolerance": 15, "matching_softness": 0,
         "mask_layer": "", "masking_mode": "luminance", "composite_over": "on"}


def case(shift=0, omit=(), **kw):
    c = dict(START, **kw)
    c["shift"], c["omit"] = shift, tuple(omit)
    return c


def render(c, frame_no):
    s = dict(c)
    for k, (lo, hi) in NUMBERS.items():
        s[k] = min(hi, max(lo, value_at(c[k], frame_no)))
    s["stops"] = int(min(5, max(2, s["stops"])) // 1)
    if "add_phase_from" in c["omit"]:
        s["add_phase_from"] = s["get_phase"]
    maps = {}
    if c["layer"] in DRAWINGS:
        maps["add"] = [p for row in DRAWINGS[c["layer"]] for p in row]
    if c["mask_layer"] in DRAWINGS:
        maps["mask"] = [p for row in DRAWINGS[c["mask_layer"]] for p in row]
    return R.frame(colorama(B.pixels(), s, maps), c["shift"])


def plain(c):
    return B.plain(c)


def file_json(fx, c):
    params = {k: setting_json(v) if k in NUMBERS else v for k, v in c.items()
              if k not in ("shift", "omit") and k not in c["omit"]}
    p = B.project_json(fx, "core.colorama", params, c["shift"])
    named = [n for n in (c["layer"], c["mask_layer"]) if n in DRAWINGS]
    for n in dict.fromkeys(named):
        p["assets"].append(L.still(n, f"media/{n}.png"))
        comp = p["compositions"][0]
        comp["layers"].append(L.raster(n, "asset-" + n, out_frame=FRAMES, enabled=False))
        comp["layer_order"].append(n)
    return p


NEW = tuple(k for k in START if k not in OLD)
OFFSET = {"phase_shift": 10}  # off the stops, for the cases with interpolation off
ALPHA = {"modify_alpha": "on", "opacity_2": 0, "opacity_4": 30}
PHASE = {"layer": "phase", "add_phase_from": "red"}
MASK = {"mask_layer": "mask"}

CASES = {
    "FX-COLORAMA-001": ("The settings as the effect is added, every D-381 setting written at its "
                        "start: phase from intensity round the five hues, each pixel its colour "
                        "by its brightness, the empty column left empty.", case(), [0]),
    "FX-COLORAMA-002": ("The same written as a file from before D-381, none of its settings "
                        "there: the same picture.", case(omit=NEW), [0]),
    "FX-COLORAMA-003": ("Phase from hue: red and the greys the first colour, the other colours "
                        "round the ring by their hue.", case(get_phase="hue"), [0]),
    "FX-COLORAMA-004": ("Phase from saturation.", case(get_phase="saturation"), [0]),
    "FX-COLORAMA-005": ("Phase from lightness.", case(get_phase="lightness"), [0]),
    "FX-COLORAMA-006": ("Phase from value, the largest channel: every full colour the first "
                        "colour, darker rows further round.", case(get_phase="value"), [0]),
    "FX-COLORAMA-007": ("Phase from zero, shift 120, three colours: every showing pixel the "
                        "second colour.", case(get_phase="zero", phase_shift=120, stops=3), [0]),
    "FX-COLORAMA-008": ("Interpolate off, shift 10: each pixel takes one of the five colours, "
                        "none between.", case(interpolate="off", **OFFSET), [0]),
    "FX-COLORAMA-009": ("Modify hue: each pixel takes the ring colour's hue and keeps its own "
                        "saturation and lightness, so the greys stay grey.",
                        case(modify="hue"), [0]),
    "FX-COLORAMA-010": ("Modify lightness: the ring colour's lightness, the pixel's own hue and "
                        "saturation.", case(modify="lightness"), [0]),
    "FX-COLORAMA-011": ("Modify saturation.", case(modify="saturation"), [0]),
    "FX-COLORAMA-012": ("Modify red: only red from the ring colour, green and blue the pixel's "
                        "own.", case(modify="red"), [0]),
    "FX-COLORAMA-013": ("Modify none, Modify Alpha on, opacities 100, 0, 100, 30, 100: the "
                        "colours kept, the covering the ring's opacity at each pixel's place.",
                        case(modify="none", **ALPHA), [0]),
    "FX-COLORAMA-014": ("Modify Alpha and Change Empty Pixels on: the empty column worked too, "
                        "as black, so it turns the first colour at full covering.",
                        case(change_empty="on", **ALPHA), [0]),
    "FX-COLORAMA-015": ("Phase from alpha, two colours, opacities 100 and 0, Modify none and "
                        "Modify Alpha on: a curve on the covering; the solid pixels stay solid, "
                        "the half-covered column nearly clear.",
                        case(get_phase="alpha", stops=2, modify="none", modify_alpha="on",
                             opacity_2=0), [0]),
    "FX-COLORAMA-016": ("Matching RGB, red, tolerance 30, softness 20: red and the colours "
                        "near it change, fading out with distance; the rest stay.",
                        case(matching_mode="rgb", matching_color="#ff0000",
                             matching_tolerance=30, matching_softness=20), [0]),
    "FX-COLORAMA-017": ("Matching hue, yellow, tolerance 10, softness 10: the yellows and the "
                        "orange in part; greys never match by hue.",
                        case(matching_mode="hue", matching_color="#ffff00",
                             matching_tolerance=10, matching_softness=10), [0]),
    "FX-COLORAMA-018": ("Matching chroma, the skin tone, tolerance 5, softness 15.",
                        case(matching_mode="chroma", matching_color="#f6d6be",
                             matching_tolerance=5, matching_softness=15), [0]),
    "FX-COLORAMA-019": ("FX-COLORAMA-016 with Composite Over Layer off: the pixels not matched "
                        "go clear, the matched ones alone.",
                        case(matching_mode="rgb", matching_color="#ff0000",
                             matching_tolerance=30, matching_softness=20,
                             composite_over="off"), [0]),
    "FX-COLORAMA-020": ("FX-COLORAMA-019 blended 50 with the original.",
                        case(matching_mode="rgb", matching_color="#ff0000",
                             matching_tolerance=30, matching_softness=20,
                             composite_over="off", blend_with_original=50), [0]),
    "FX-COLORAMA-021": ("Matching tolerance keyed from 0 at frame 0 to 100 at frame 4, linear, "
                        "softness 10, matching white: more and more of the layer changes.",
                        case(matching_mode="rgb", matching_tolerance=keyed((0, 0), (4, 100)),
                             matching_softness=10), [0, 2, 4]),
    "FX-COLORAMA-022": ("Opacity 3 keyed from 100 at frame 0 to 0 at frame 4, Modify Alpha on: "
                        "the pixels at the third colour fade.",
                        case(modify_alpha="on", opacity_3=keyed((0, 100), (4, 0))), [0, 4]),
    "FX-COLORAMA-023": ("FX-COLORAMA-009 moved three pixels right: the same, moved.",
                        case(modify="hue", shift=3), [0, 3]),
    "FX-COLORAMA-024": ("Add Phase layer `phase`, from its red, add mode wrap: each pixel's "
                        "place moves on by the layer's red under it, less where it is half "
                        "covered.", case(**PHASE), [0]),
    "FX-COLORAMA-025": ("Add mode clamp: the sum held at 1, the first colour.",
                        case(add_mode="clamp", **PHASE), [0]),
    "FX-COLORAMA-026": ("Add mode average.", case(add_mode="average", **PHASE), [0]),
    "FX-COLORAMA-027": ("Add mode screen.", case(add_mode="screen", **PHASE), [0]),
    "FX-COLORAMA-028": ("An Add Phase layer in a file from before D-381, no add phase from: the "
                        "layer's intensity is added, as get phase reads it.",
                        case(layer="phase", omit=NEW), [0]),
    "FX-COLORAMA-029": ("Mask layer `mask`, by luminance: the left full Colorama, the middle "
                        "part way, the right untouched where the mask is clear and half where it "
                        "is half covered.", case(**MASK), [0]),
    "FX-COLORAMA-030": ("Masking by inverse luminance: the other way round.",
                        case(masking_mode="inverse_luminance", **MASK), [0]),
    "FX-COLORAMA-031": ("Masking by alpha: the grey middle counts as fully there.",
                        case(masking_mode="alpha", **MASK), [0]),
    "FX-COLORAMA-032": ("Masking by inverse alpha with Composite Over Layer off: only the right "
                        "of the layer shows.",
                        case(masking_mode="inverse_alpha", composite_over="off", **MASK), [0]),
    "FX-COLORAMA-033": ("Add Phase and mask layers together, add mode screen, masking by "
                        "luminance.", case(add_mode="screen", **PHASE, **MASK), [0]),
    "FX-COLORAMA-034": ("Mask layer `ghost`, not a layer of the composition: no mask, the whole "
                        "of FX-COLORAMA-001, with EFFECT_LAYER_MISSING each frame.",
                        case(mask_layer="ghost"), [0, 3]),
}

INVALID = {
    "FX-COLORAMA-035": ("Get phase \"Hue\": the word is exact, so a capital is not it.",
                        case(get_phase="Hue")),
    "FX-COLORAMA-036": ("Add phase from \"brightness\", which is not a choice.",
                        case(add_phase_from="brightness")),
    "FX-COLORAMA-037": ("Add mode \"multiply\", which is not a choice.", case(add_mode="multiply")),
    "FX-COLORAMA-038": ("Interpolate \"yes\", which is not a choice.", case(interpolate="yes")),
    "FX-COLORAMA-039": ("Opacity 3 at 101, above 100.", case(opacity_3=101)),
    "FX-COLORAMA-040": ("Modify \"rgb\", which is not a choice.", case(modify="rgb")),
    "FX-COLORAMA-041": ("Change empty \"yes\", which is not a choice.", case(change_empty="yes")),
    "FX-COLORAMA-042": ("Matching mode \"lab\", which is not a choice.",
                        case(matching_mode="lab")),
    "FX-COLORAMA-043": ("Matching colour \"white\", not written as #rrggbb.",
                        case(matching_color="white")),
    "FX-COLORAMA-044": ("Matching tolerance -1, below 0.", case(matching_tolerance=-1)),
    "FX-COLORAMA-045": ("Mask layer 5, a number, not the name of a layer.", case(mask_layer=5)),
    "FX-COLORAMA-046": ("Masking mode \"off\", which is not a choice.", case(masking_mode="off")),
    "FX-COLORAMA-047": ("Composite over \"no\", which is not a choice.",
                        case(composite_over="no")),
}


def main():
    expected = B.write_cases(OUT, "colorama", CASES, INVALID, render, plain, file_json)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(pixels))
    expected["cases"]["FX-COLORAMA-034"]["warning"] = "EFFECT_LAYER_MISSING"  # D-189, on opening too (D-381)
    (OUT / "expected_colorama.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    assert min(EDGES) > 1e-4, min(EDGES)
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain(case())
    px = B.pixels()
    near = lambda p, q, e=1e-9: all(abs(u - v) < e for u, v in zip(p, q))  # noqa: E731
    like = lambda f, g: all(near(p, q) for p, q in zip(f, g))  # noqa: E731
    at = lambda x, y: y * W + x  # noqa: E731
    enc = lambda p: [S.linear_to_srgb(u / p[3]) for u in p[:3]]  # noqa: E731
    one = c["FX-COLORAMA-001"]["0"]

    # D-316's rule, written out once more: the start is today's effect, and so is an old file.
    ring = [hexc(START[f"color_{i}"]) for i in range(1, 6)]
    for i, p in enumerate(px):
        if p[3]:
            u = (sum(p[:3]) / 765 % 1) * 5
            k = int(u)
            m = [ring[k % 5][ch] + (u - k) * (ring[(k + 1) % 5][ch] - ring[k % 5][ch])
                 for ch in range(3)]
            assert near(one[i], [srgb_to_linear(v) * p[3] / 255 for v in m] + [p[3] / 255])
        else:
            assert one[i] == [0.0] * 4
    assert c["FX-COLORAMA-002"]["0"] == one
    assert like(c["FX-COLORAMA-034"]["0"], one) and like(c["FX-COLORAMA-034"]["3"], one)
    # Hue: red and the greys read 0, the first colour.
    three = c["FX-COLORAMA-003"]["0"]
    for x in (1, 2, 3, 8):
        assert near(enc(three[at(x, 0)]), ring[0])
    seven = c["FX-COLORAMA-007"]["0"]
    assert all(near(enc(q), ring[1], 1e-7) for q, p in zip(seven, px) if p[3])
    # Interpolate off: every colour one of the ring's.
    eight = c["FX-COLORAMA-008"]["0"]
    assert all(any(near(enc(q), r) for r in ring) for q, p in zip(eight, px) if p[3])
    # Modify hue keeps grey; modify red keeps green and blue.
    nine, twelve = c["FX-COLORAMA-009"]["0"], c["FX-COLORAMA-012"]["0"]
    for x in (1, 2, 3):
        assert all(near(nine[at(x, y)], drawn[at(x, y)], 1e-7) for y in range(H))
    for i, p in enumerate(px):
        if p[3]:
            assert near(enc(twelve[i])[1:], B.encoded(p)[1:])
            assert abs(enc(twelve[i])[0] - enc(one[i])[0]) < 1e-9
            assert near(enc(c["FX-COLORAMA-013"]["0"][i]), B.encoded(p)) or \
                c["FX-COLORAMA-013"]["0"][i][3] < 1e-9
    # Change empty fills the empty column with the first colour, fully covered.
    fourteen = c["FX-COLORAMA-014"]["0"]
    assert all(near(fourteen[at(0, y)], [srgb_to_linear(v) for v in ring[0]] + [1])
               for y in range(H))
    fifteen = c["FX-COLORAMA-015"]["0"]
    assert near(fifteen[at(4, 0)], drawn[at(4, 0)]) and fifteen[at(15, 0)][3] < 0.02
    # Matching: unmatched pixels kept, or gone with composite off; blend halfway back.
    sixteen, nineteen, twenty = (c[f"FX-COLORAMA-0{n}"]["0"] for n in (16, 19, 20))
    assert near(sixteen[at(8, 0)], one[at(8, 0)]) and sixteen[at(5, 0)] == drawn[at(5, 0)]
    assert nineteen[at(5, 0)] == [0.0] * 4 and near(nineteen[at(8, 0)], one[at(8, 0)])
    assert near(twenty[at(5, 0)], [v / 2 for v in drawn[at(5, 0)]])
    seventeen = c["FX-COLORAMA-017"]["0"]
    assert all(seventeen[at(x, y)] == drawn[at(x, y)] for x in (1, 2, 3) for y in range(H))
    assert near(seventeen[at(4, 0)], one[at(4, 0)])
    twenty_one = c["FX-COLORAMA-021"]
    changed = [sum(f[i] != drawn[i] for i in range(W * H)) for f in twenty_one.values()]
    assert changed[0] < changed[1] < changed[2]
    assert like(twenty_one["4"], one)
    # Moved.
    nine_m = c["FX-COLORAMA-023"]["0"]
    for y in range(H):
        assert nine_m[at(3, y):at(0, y + 1)] == nine[at(0, y):at(W - 3, y)]
    # Add modes differ; clamp at a pixel whose sum passes 1 is the first colour.
    adds = [c[f"FX-COLORAMA-0{n}"]["0"] for n in (24, 25, 26, 27)]
    assert all(not like(adds[i], adds[j]) for i in range(4) for j in range(i + 1, 4))
    assert near(enc(adds[1][at(14, 0)]), ring[0])
    assert not like(c["FX-COLORAMA-028"]["0"], adds[0])
    # Masks: full on the left, none on the clear right; inverse the other way.
    m1, m2 = c["FX-COLORAMA-029"]["0"], c["FX-COLORAMA-030"]["0"]
    assert near(m1[at(4, 0)], one[at(4, 0)]) and m1[at(12, 0)] == drawn[at(12, 0)]
    assert near(m2[at(4, 0)], drawn[at(4, 0)]) and near(m2[at(12, 0)], one[at(12, 0)])
    assert near(c["FX-COLORAMA-031"]["0"][at(8, 0)], one[at(8, 0)])
    assert c["FX-COLORAMA-032"]["0"][at(4, 0)] == [0.0] * 4
    for fx, frames in c.items():
        for f in frames.values():
            for p in f:
                assert all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]) and p[3] <= 1 + 1e-12, fx
    print("checked")


if __name__ == "__main__":
    main()
