"""A layer's Hard Light, Darken, Lighten, Difference, Hue, Saturation, Color and Luminosity blend
modes (D-748), worked a second way.

The rule (document 21, "Blend modes", and D-301): recover the straight colours cs (the layer) and
cd (beneath), encode each to sRGB held to 0..1, mix them with f below, decode, and lay it with

    Co = (1-As) Cd + (1-Ad) Cs + As Ad B,   Ao = As + Ad - As Ad.

f, on encoded b (beneath) and s (the layer), per channel:
- hard_light: 2 b s where s <= 0.5, else 1 - 2 (1-b)(1-s) (Adobe's and the W3C's Hard Light,
  Overlay with the two swapped);
- darken: min(b, s); lighten: max(b, s); difference: |b - s|.
and on the whole colour, the W3C Compositing and Blending Level 1 non-separable modes with
Lum = 0.3 r + 0.59 g + 0.11 b, ClipColor, SetLum, Sat and SetSat (our basis for these four:
After Effects publishes no formula; D-730 took the same for Turbulent Noise):
- hue: SetLum(SetSat(s, Sat(b)), Lum(b)); saturation: SetLum(SetSat(b, Sat(s)), Lum(b));
- color: SetLum(s, Lum(b)); luminosity: SetLum(b, Lum(s)).

D-921 adds fourteen more (FX-LBLEND-010 to 023), on the same encoded colours and the same 0..1 hold:
- color_dodge: 0 where b = 0, else 1 where s = 1, else min(1, b / (1 - s)) (the W3C's; Adobe:
  "if the original layer color is pure black, the result color is the underlying color");
- color_burn: 1 where b = 1, else 0 where s = 0, else 1 - min(1, (1 - b) / s) (the W3C's);
- linear_burn: max(0, b + s - 1); linear_light: b + 2 s - 1 held to 0..1;
- vivid_light: color_burn(b, 2 s) where s <= 0.5, else color_dodge(b, 2 s - 1);
- pin_light: min(b, 2 s) where s <= 0.5, else max(b, 2 s - 1);
- hard_mix: 1 where b + s >= 1, else 0; exclusion: b + s - 2 b s; subtract: max(0, b - s);
- divide: min(1, b / s), and where s = 0, 1 if b > 0 else 0;
- darker_color, lighter_color: the whole colour of whichever of the two has the lower (higher)
  Lum, the backdrop's on a tie.
The two Classic modes (our basis; Adobe says only that they are After Effects 5.0's): the layer's
encoded colour is first scaled by its alpha As, s' = s As, the mode is worked on (b, s') and laid
with full weight where there is a backdrop:

    Co = (1-Ad) Cs + Ad B',   Ao = As + Ad - As Ad,

classic_color_dodge's f the color_dodge above, classic_color_burn's the color_burn above except
that s' = 0 leaves b as it is. So a faded Classic layer dodges or burns as a darker layer rather
than mixing towards the backdrop, and a black layer in Classic Color Burn changes nothing.
The new cases use the colours TOP_NEW, chosen so that no b + s is within 0.01 of 1 (Hard Mix's
edge) and no s is 0 or 0.5, and only the yellow row's red is 1.

Nothing here is read from the program. Writes Fixtures/layer_blend_modes/.

The composition, 10 by 8: four opaque backdrop strips 2 pixels wide in columns 0..7, columns 8
and 9 clear; above them, in the mode under test, four strips 2 pixels tall, the last at 50%
opacity. So each 2 by 2 cell is one backdrop colour under one layer colour, and columns 8 and 9
are the layer over nothing (its own colour).
"""

import json
from pathlib import Path

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "layer_blend_modes"
W, H = 10, 8
TOLERANCE = 2e-5

# Encoded colours. Backdrop by column strip, the layer by row strip (opacity in per cent).
BACK = [(0.25, 0.75, 0.5), (0.9, 0.2, 0.1), (0.1, 0.3, 0.95), (0.5, 0.5, 0.5)]
TOP = [((0.8, 0.3, 0.5), 100), ((0.2, 0.9, 0.6), 100), ((1.0, 0.85, 0.1), 100),
       ((0.35, 0.1, 0.7), 50)]


def lin(c):
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4


def enc(c):
    c = min(1.0, max(0.0, c))
    return c * 12.92 if c <= 0.0031308 else 1.055 * c ** (1 / 2.4) - 0.055


def lum(c):
    return 0.3 * c[0] + 0.59 * c[1] + 0.11 * c[2]


def clip_color(c):
    l = lum(c)
    n, x = min(c), max(c)
    if n < 0 and l > n:
        c = [l + (v - l) * l / (l - n) for v in c]
    if x > 1 and x > l:
        c = [l + (v - l) * (1 - l) / (x - l) for v in c]
    return c


def set_lum(c, l):
    d = l - lum(c)
    return clip_color([v + d for v in c])


def sat(c):
    return max(c) - min(c)


def set_sat(c, s):
    n, x = min(c), max(c)
    return [(v - n) * s / (x - n) if x > n else 0.0 for v in c]


PER_CHANNEL = {
    "hard_light": lambda b, s: 2 * b * s if s <= 0.5 else 1 - 2 * (1 - b) * (1 - s),
    "darken": min,
    "lighten": max,
    "difference": lambda b, s: abs(b - s),
}
WHOLE = {
    "hue": lambda b, s: set_lum(set_sat(s, sat(b)), lum(b)),
    "saturation": lambda b, s: set_lum(set_sat(b, sat(s)), lum(b)),
    "color": lambda b, s: set_lum(s, lum(b)),
    "luminosity": lambda b, s: set_lum(b, lum(s)),
}


def dodge(b, s):
    if b <= 0:
        return 0.0
    if s >= 1:
        return 1.0
    return min(1.0, b / (1 - s))


def burn(b, s):
    if b >= 1:
        return 1.0
    if s <= 0:
        return 0.0
    return 1 - min(1.0, (1 - b) / s)


PER_CHANNEL.update({
    "color_dodge": dodge,
    "color_burn": burn,
    "classic_color_dodge": dodge,
    "classic_color_burn": lambda b, s: b if s <= 0 else burn(b, s),
    "linear_burn": lambda b, s: max(0.0, b + s - 1),
    "linear_light": lambda b, s: min(1.0, max(0.0, b + 2 * s - 1)),
    "vivid_light": lambda b, s: burn(b, 2 * s) if s <= 0.5 else dodge(b, 2 * s - 1),
    "pin_light": lambda b, s: min(b, 2 * s) if s <= 0.5 else max(b, 2 * s - 1),
    "hard_mix": lambda b, s: 1.0 if b + s >= 1 else 0.0,
    "exclusion": lambda b, s: b + s - 2 * b * s,
    "subtract": lambda b, s: max(0.0, b - s),
    "divide": lambda b, s: (1.0 if b > 0 else 0.0) if s <= 0 else min(1.0, b / s),
})
WHOLE.update({
    "darker_color": lambda b, s: list(s) if lum(s) < lum(b) else list(b),
    "lighter_color": lambda b, s: list(s) if lum(s) > lum(b) else list(b),
})
CLASSIC = ("classic_color_dodge", "classic_color_burn")
TOP_NEW = [((0.85, 0.35, 0.6), 100), ((0.15, 0.95, 0.42), 100), ((1.0, 0.62, 0.07), 100),
           ((0.33, 0.12, 0.71), 50)]


def mix(mode, b, s):
    """Encoded b beneath, encoded s the layer, both straight; the encoded result."""
    if mode in PER_CHANNEL:
        return [PER_CHANNEL[mode](b[c], s[c]) for c in range(3)]
    return WHOLE[mode](b, s)


def top(mode):
    return TOP_NEW if mode in NEW_MODES else TOP


def pixel(mode, x, y):
    """The expected pixel, linear and premultiplied."""
    dst = [0.0, 0.0, 0.0, 0.0]
    if x < 8:
        dst = [lin(v) for v in BACK[x // 2]] + [1.0]
    color, opacity = top(mode)[y // 2]
    a_s = opacity / 100
    cs = [lin(v) for v in color]
    src = [v * a_s for v in cs] + [a_s]
    a_d = dst[3]
    cd = dst[:3]  # opaque or clear, so straight equals premultiplied
    if mode in CLASSIC:
        bm = [lin(v) for v in mix(mode, [enc(v) for v in cd], [enc(v) * a_s for v in cs])]
        out = [(1 - a_d) * src[c] + a_d * bm[c] for c in range(3)]
        return out + [a_s + a_d - a_s * a_d]
    bm = [lin(v) for v in mix(mode, [enc(v) for v in cd], [enc(v) for v in cs])]
    out = [(1 - a_s) * dst[c] + (1 - a_d) * src[c] + a_s * a_d * bm[c] for c in range(3)]
    return out + [a_s + a_d - a_s * a_d]


def transform(x, y, w, h):
    t = lambda v: {"base": v, "keyframes": []}
    return {"anchor": t([w / 2, h / 2]), "position": t([x + w / 2, y + h / 2]),
            "scale": t([100, 100]), "rotation": t(0), "opacity": t(100)}


def solid(lid, color, x, y, w, h, opacity, blend):
    tr = transform(x, y, w, h)
    tr["opacity"] = {"base": opacity / 100, "keyframes": []}
    return {"id": lid, "kind": "solid", "name": lid, "enabled": True, "locked": False,
            "in_frame": 0, "out_frame": 1,
            "solid": {"color": [lin(v) for v in color], "width": w, "height": h},
            "transform": tr, "masks": [], "matte": None, "blend_mode": blend, "effects": []}


def project(mode, pid):
    layers = [solid(f"back-{i}", c, 2 * i, 0, 2, H, 100, "normal") for i, c in enumerate(BACK)]
    layers += [solid(f"top-{j}", c, 0, 2 * j, W, 2, o, mode) for j, (c, o) in enumerate(top(mode))]
    return {
        "schema_version": 0, "project_id": pid,
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": "comp-main", "name": "comp-main", "width": W, "height": H,
            "pixel_aspect_ratio": 1, "frame_rate": {"numerator": 24, "denominator": 1},
            "start_frame": 0, "duration_frames": 1,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": [l["id"] for l in layers], "layers": layers,
        }],
    }


CASES = [
    ("hard_light", "Hard Light: Overlay with the two swapped, the layer's light half screens and "
                   "its dark half multiplies; the yellow row lifts red and green and darkens blue."),
    ("darken", "Darken: the darker of the two in each channel."),
    ("lighten", "Lighten: the lighter of the two in each channel."),
    ("difference", "Difference: the two apart in each channel; on the grey column the layer's "
                   "colours turned about 50%."),
    ("hue", "Hue: the layer's hue at the backdrop's saturation and lightness; the grey column "
            "stays grey."),
    ("saturation", "Saturation: the backdrop's hue and lightness at the layer's saturation."),
    ("color", "Color: the layer's hue and saturation at the backdrop's lightness."),
    ("luminosity", "Luminosity: the backdrop's hue and saturation at the layer's lightness."),
]
NEW_CASES = [
    ("color_burn", "Color Burn: the backdrop darkened by the layer, (1 - b) / s taken off white; a "
                   "white layer would leave it as it is."),
    ("classic_color_burn", "Classic Color Burn: Color Burn worked with the layer's colour scaled by "
                           "its opacity, so the 50% row burns as a darker colour at full weight."),
    ("linear_burn", "Linear Burn: b + s - 1, held at 0; darker than Multiply."),
    ("darker_color", "Darker Color: the whole colour of whichever is darker by Lum, never a mix "
                     "of channels."),
    ("color_dodge", "Color Dodge: the backdrop brightened by the layer, b / (1 - s), held at 1; "
                    "the yellow row's full red turns every red to 1."),
    ("classic_color_dodge", "Classic Color Dodge: Color Dodge worked with the layer's colour "
                            "scaled by its opacity, so the 50% row dodges as a darker colour."),
    ("lighter_color", "Lighter Color: the whole colour of whichever is lighter by Lum."),
    ("linear_light", "Linear Light: b + 2 s - 1, held to 0..1."),
    ("vivid_light", "Vivid Light: Color Burn by 2 s below a half, Color Dodge by 2 s - 1 above."),
    ("pin_light", "Pin Light: the darker of b and 2 s below a half, the lighter of b and 2 s - 1 "
                  "above."),
    ("hard_mix", "Hard Mix: each channel 0 or 1, 1 where b + s reaches 1."),
    ("exclusion", "Exclusion: b + s - 2 b s, a softer Difference."),
    ("subtract", "Subtract: b - s, held at 0."),
    ("divide", "Divide: b / s, held at 1; the yellow row's 0.07 blue turns most blues to 1."),
]
NEW_MODES = [m for m, _ in NEW_CASES]
REFUSED = ("FX-LBLEND-009", "dissolve",
           "A mode After Effects has and this program does not, Dissolve (D-920; before D-921 "
           "this case was Color Dodge): the file is refused with the reason, which names the "
           "thirty modes it knows; nothing is drawn as another mode (document 28).")
ALL_MODES = ["normal", "darken", "multiply", "color_burn", "classic_color_burn", "linear_burn",
             "darker_color", "add", "lighten", "screen", "color_dodge", "classic_color_dodge",
             "lighter_color", "overlay", "soft_light", "hard_light", "linear_light",
             "vivid_light", "pin_light", "hard_mix", "difference", "exclusion", "subtract",
             "divide", "hue", "saturation", "color", "luminosity", "stencil_alpha",
             "stencil_luma"]


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    cases = {}
    for n, (mode, says) in enumerate(CASES, 1):
        fid = f"FX-LBLEND-{n:03d}"
        name = f"fx_lblend_{n:03d}.json"
        (OUT / name).write_text(json.dumps(project(mode, fid.lower()), indent=1) + "\n",
                                encoding="utf-8", newline="\n")
        frame = [pixel(mode, x, y) for y in range(H) for x in range(W)]
        cases[fid] = {"says": says, "mode": mode, "project": name, "frames": {"0": frame}}
    for n, (mode, says) in enumerate(NEW_CASES, 10):
        fid = f"FX-LBLEND-{n:03d}"
        name = f"fx_lblend_{n:03d}.json"
        (OUT / name).write_text(json.dumps(project(mode, fid.lower()), indent=1) + "\n",
                                encoding="utf-8", newline="\n")
        frame = [pixel(mode, x, y) for y in range(H) for x in range(W)]
        cases[fid] = {"says": says, "mode": mode, "project": name, "frames": {"0": frame}}
    fid, mode, says = REFUSED
    name = "fx_lblend_009.json"
    (OUT / name).write_text(json.dumps(project(mode, fid.lower()), indent=1) + "\n",
                            encoding="utf-8", newline="\n")
    refused = {fid: {"says": says, "mode": mode, "project": name,
                     "reason": "layers/4/blend_mode: expected one of "
                               + ", ".join(ALL_MODES) + "."}}
    out = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": cases, "refused": refused}
    (OUT / "expected_layer_blend_modes.json").write_text(json.dumps(out, indent=1) + "\n",
                                                          encoding="utf-8", newline="\n")
    print(f"wrote {len(cases)} drawn cases and 1 refused case to {OUT}")
    # Hard Mix's edge: no channel sum of the new colours within 0.01 of 1.
    for color, _ in TOP_NEW:
        for back in BACK:
            assert all(abs(b + s - 1) > 0.01 for b, s in zip(back, color)), (back, color)


if __name__ == "__main__":
    main()
