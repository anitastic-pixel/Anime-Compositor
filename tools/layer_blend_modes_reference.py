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


def mix(mode, b, s):
    """Encoded b beneath, encoded s the layer, both straight; the encoded result."""
    if mode in PER_CHANNEL:
        return [PER_CHANNEL[mode](b[c], s[c]) for c in range(3)]
    return WHOLE[mode](b, s)


def pixel(mode, x, y):
    """The expected pixel, linear and premultiplied."""
    dst = [0.0, 0.0, 0.0, 0.0]
    if x < 8:
        dst = [lin(v) for v in BACK[x // 2]] + [1.0]
    color, opacity = TOP[y // 2]
    a_s = opacity / 100
    cs = [lin(v) for v in color]
    src = [v * a_s for v in cs] + [a_s]
    a_d = dst[3]
    cd = dst[:3]  # opaque or clear, so straight equals premultiplied
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
    layers += [solid(f"top-{j}", c, 0, 2 * j, W, 2, o, mode) for j, (c, o) in enumerate(TOP)]
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
REFUSED = ("FX-LBLEND-009", "color_dodge",
           "A mode After Effects has and this program does not, Color Dodge: the file is refused "
           "with the reason, which names the sixteen modes it knows; nothing is drawn as another "
           "mode (document 28).")


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
    fid, mode, says = REFUSED
    name = "fx_lblend_009.json"
    (OUT / name).write_text(json.dumps(project(mode, fid.lower()), indent=1) + "\n",
                            encoding="utf-8", newline="\n")
    refused = {fid: {"says": says, "mode": mode, "project": name,
                     "reason": "layers/4/blend_mode: expected one of normal, darken, multiply, "
                               "add, lighten, screen, overlay, soft_light, hard_light, "
                               "difference, hue, saturation, color, luminosity, stencil_alpha, "
                               "stencil_luma."}}
    out = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": cases, "refused": refused}
    (OUT / "expected_layer_blend_modes.json").write_text(json.dumps(out, indent=1) + "\n",
                                                          encoding="utf-8", newline="\n")
    print(f"wrote {len(cases)} drawn cases and 1 refused case to {OUT}")


if __name__ == "__main__":
    main()
