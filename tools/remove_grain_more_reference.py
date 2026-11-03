"""Remove Grain's further After Effects controls, worked a second way (D-966).

D-455 built Remove Grain's Noise Reduction Settings and Unsharp Mask
(`tools/remove_grain_reference.py`, whose rule this file reuses unchanged). D-966 adds the groups
After Effects shows round them, as the playlist's Remove Grain tutorial shows them with their
values as added (wave 12 lane w12d, crops/xa_375.png): Viewing Mode, Preview Region (Center,
Width 200, Height 200, Show Box), Channel Noise Reduction (Red, Green, Blue, 1 each), Fine
Tuning (Chroma Suppression 1, Texture 0.1, Noise Size Bias 0, Clean Solid Areas 0) and Temporal
Filtering (Enable off, Amount 100 %, Motion Sensitivity 0.9). Adobe's user guide (After Effects
CS4, Remove Grain, manualsdir.com/manuals/753846/adobe-after-effects-cs4.html?page=541 and 542)
says what each does and gives no formula:

- Chroma Suppression "suppresses some of the chroma from the noise"; "too high may strip some
  chroma from the image itself"; it "has no effect on grayscale images".
- Texture "controls the amount of low-level noise that passes through to the output. Lower values
  result in a smoother, possibly artificial-looking result."
- Noise Size Bias: "the default value of zero treats all sizes equally"; negative values remove
  small grain harder, positive values large grain.
- Clean Solid Areas: "the extent to which adjacent pixels with low variations in value are
  smoothed out"; too high makes nearly solid areas look artificial.
- Channel Noise Reduction: Red, Green and Blue Noise Reduction, set apart.
- Viewing Mode: Preview "displays the current settings of the applied effect in a 200x200 pixel
  area"; Noise Samples "shows the areas that have been sampled"; Blending Matte "shows the
  current color matte or mask" of the Blend with Original group; Final Output.
- Temporal Filtering, for a sequence of frames: Enable, Amount, and Motion Sensitivity: "if you
  see unwanted streaking or blurs around moving objects, reduce the Motion Sensitivity value".

The rules below are this program's own (nothing is ported), each a small change to D-455's,
written so that the value an older file reads (the "neutral" value, in brackets) leaves D-455's
picture exactly as it was:

1. Channel Noise Reduction r_c, 0 to 5 (1): sigma_c is taken as r_c sigma_c before the spreads
   are made, h_c = 2 n r_c sigma_c (Multichannel h = 2 n sqrt(mean of (r_c sigma_c)^2)).
2. Chroma Suppression s, 0 to 5 (0; 1 as added): in a tap's colour bell the difference d_c is
   split into its mean over the channels, Y, and the rest, C_c = d_c - Y, and the rest is
   divided by 1 + s: Multichannel D^2 = Y^2 + mean(C_c^2) / (1 + s)^2; Single Channel each
   channel's d_c becomes Y + C_c / (1 + s). Colour that differs only in hue weighs less, so
   colour grain is averaged away harder, and hue edges of one brightness soften.
3. Noise Size Bias b, -1 to 1 (0): pass k of P has its colour spreads times 2^(b u_k), u_k =
   2 (k - 1) / (P - 1) - 1 running from -1 at the first (smallest) pass to 1 at the last; one
   pass, u = 0. Negative b, the small passes reach further in colour; positive, the large ones.
4. Clean Solid Areas a, 0 to 3 (0): after the passes, one more pass of the first size (radius 2,
   spread 1) whose colour spreads are a h_c. Skipped at 0.
5. Texture t, 0 to 1 (0; 0.1 as added): the encoded result e becomes e + t (o - e), o the
   encoded colour the effect was given: t of the grain is passed through.
6. Temporal Filtering, "off" or "on" (off), Amount A 0 to 100 (100), Motion Sensitivity m 0 to 1
   (0.9): first, before the passes, each pixel that shows becomes o + A/100 (T - o), T the mean
   of its own colour (weight 1) and the same pixel of the layer as it reaches the effect at the
   frame before and the frame after (where the layer is, the same size; weight its covering
   times a bell on the colour difference, of spread m h, Multichannel one bell on the mean
   squared difference, Single Channel one per channel). m h is how far a colour may differ and
   still count as the same, unmoving thing: lower m, less streaking.
7. Viewing Mode, "final_output", "preview", "noise_samples" or "blending_matte"
   (final_output; After Effects adds it as Preview, ours as Final Output, D-966): Preview draws
   the whole result inside the Preview Region and the layer as given outside it, then, with
   Show Box "on" (on), the region's inside edge in opaque white; the region is the pixels whose
   middles lie less than Width / 2 and Height / 2 (pixels, 1 to 10000, 200) from its Center
   (per cent of the drawing's width and height, -1000 to 1000, 50 50). Noise Samples: ours
   measures the whole layer, so the layer as given with the drawing's inside edge in opaque
   white. Blending Matte: ours has no matte (Blend with Original is the effect's Mix), so the
   matte is the same everywhere: white at each pixel's covering.

Everything else is D-455's: the passes, the measure, then the Unsharp Mask on the whole result.

**This file never runs the build's code path.** Double precision on lists, from the drawings'
exact 8-bit values. Cases FX-REMOVEGRAIN-022 to 040 in `Fixtures/remove_grain/`, expected
pixels in `Fixtures/remove_grain/expected_remove_grain_more.json`. Fixtures are read-only to
implementation work: this file is run when the specification changes, never to make a build
pass. FX-REMOVEGRAIN-001 to 021 and their expected file are not touched.

    python tools/remove_grain_more_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import recolor_reference as R  # noqa: E402
import unsharp_threshold_reference as U  # noqa: E402
import remove_grain_reference as G  # noqa: E402

OUT = G.OUT
TOLERANCE = G.TOLERANCE
W, H, FRAMES = G.W, G.H, G.FRAMES

# Each further setting: its neutral value (an older file's), its range.
MORE = {
    "red_noise_reduction": (1, (0, 5)),
    "green_noise_reduction": (1, (0, 5)),
    "blue_noise_reduction": (1, (0, 5)),
    "chroma_suppression": (0, (0, 5)),
    "texture": (0, (0, 1)),
    "noise_size_bias": (0, (-1, 1)),
    "clean_solid_areas": (0, (0, 3)),
    "temporal_amount": (100, (0, 100)),
    "motion_sensitivity": (0.9, (0, 1)),
    "preview_width": (200, (1, 10000)),
    "preview_height": (200, (1, 10000)),
}
WORDS = {"viewing_mode": "final_output", "show_box": "on", "temporal_filtering": "off"}
VIEWS = ("final_output", "preview", "noise_samples", "blending_matte")
ADDED = {"passes": 3, "chroma_suppression": 1, "texture": 0.1}  # D-965 and D-966, as added


# --- the drawings ---------------------------------------------------------------------------

def moving(f):
    """`grainy` whose grain is drawn again each frame (the step shifted by the frame)."""
    def at(x, y):
        base = (215, 185, 150) if x < 8 else (35, 55, 115)
        g = G.d(x + 3 * f, y + f)
        return (base[0] + 4 * g, base[1] + 3 * g, base[2] + 2 * G.d(y + f, x + 3 * f), 255)
    return [[at(x, y) for x in range(W)] for y in range(H)]


def cut(f):
    """`moving`, but frames 3 and 4 show a green right half: something moved in."""
    rows = moving(f)
    if f >= 3:
        rows = [[(p if x < 8 else (60, 170, 70, 255)) for x, p in enumerate(r)] for r in rows]
    return rows


SEQUENCES = {"seq": moving, "cut": cut}
DRAWINGS = dict(G.DRAWINGS)
for _s, _fn in SEQUENCES.items():
    for _f in range(FRAMES):
        DRAWINGS[f"{_s}_{_f + 1}"] = _fn(_f)


def pixels_of(name, f):
    key = f"{name}_{f + 1}" if name in SEQUENCES else name
    return [R.working(p) for row in DRAWINGS[key] for p in row]


# --- the rule -------------------------------------------------------------------------------

def get(c, k, f):
    if k in WORDS:
        return c.get(k, WORDS[k])
    if k == "preview_center":
        return value_at(c.get(k, (50, 50)), f)
    lo, hi = MORE[k][1] if k in MORE else G.RANGES[k]
    return min(hi, max(lo, value_at(c.get(k, MORE[k][0] if k in MORE else None), f)))


def spreads(sig, n, mode):
    if mode == "multichannel":
        return [2 * n * math.sqrt(sum(s * s for s in sig) / 3)] * 3
    return [2 * n * s for s in sig]


def bells(d, hs, mode, s):
    """The colour bell of a tap whose difference is d (D-455's, with Chroma Suppression s)."""
    if mode == "multichannel":
        if s == 0:
            dd = sum(v * v for v in d) / 3
        else:
            y = sum(d) / 3
            dd = y * y + sum((v - y) ** 2 for v in d) / 3 / (1 + s) ** 2
        return [math.exp(-dd / (2 * hs[0] ** 2))] * 3
    if s != 0:
        y = sum(d) / 3
        d = [y + (v - y) / (1 + s) for v in d]
    return [math.exp(-d[ch] ** 2 / (2 * hs[ch] ** 2)) if hs[ch] > 0 else 0.0 for ch in range(3)]


def bilateral(e, px, k, hs, mode, s):
    r = 2 * k
    taps = [(dx, dy) for dy in range(-r, r + 1) for dx in range(-r, r + 1)
            if dx * dx + dy * dy <= r * r]
    out = []
    for i, own in enumerate(e):
        if own is None:
            out.append(None)
            continue
        x, y = i % W, i // W
        total, weight = [0.0] * 3, [0.0] * 3
        for dx, dy in taps:
            tx, ty = x + dx, y + dy
            if not (0 <= tx < W and 0 <= ty < H) or e[ty * W + tx] is None:
                continue
            t = e[ty * W + tx]
            g = math.exp(-(dx * dx + dy * dy) / (2 * k * k)) * px[ty * W + tx][3]
            wt = [g * b for b in bells([t[ch] - own[ch] for ch in range(3)], hs, mode, s)]
            for ch in range(3):
                total[ch] += wt[ch] * t[ch]
                weight[ch] += wt[ch]
        out.append([total[ch] / weight[ch] if hs[ch] > 0 else own[ch] for ch in range(3)])
    return out


def temporal(e, px, nbs, hs, mode, m, amount):
    mh = [m * h for h in hs]
    if max(mh) == 0 or amount == 0:
        return e
    out = []
    for i, own in enumerate(e):
        if own is None:
            out.append(None)
            continue
        total, weight = list(own), [1.0] * 3
        for ne, npx in nbs:
            t = ne[i]
            if t is None:
                continue
            wt = [npx[i][3] * b for b in bells([t[ch] - own[ch] for ch in range(3)], mh, mode, 0)]
            for ch in range(3):
                total[ch] += wt[ch] * t[ch]
                weight[ch] += wt[ch]
        out.append([own[ch] + amount / 100 * (total[ch] / weight[ch] - own[ch])
                    if mh[ch] > 0 else own[ch] for ch in range(3)])
    return out


def denoise(c, f):
    px = pixels_of(c["drawing"], f)
    n, mode = get(c, "noise_reduction", f), c["mode"]
    passes = math.floor(get(c, "passes", f))
    r = [get(c, k, f) for k in ("red_noise_reduction", "green_noise_reduction",
                                "blue_noise_reduction")]
    sig = [a * b for a, b in zip(r, G.levels(px))]
    hs = spreads(sig, n, mode)
    if max(hs) == 0:
        return [list(p) for p in px]
    o = G.encoded(px)
    e = o
    if get(c, "temporal_filtering", f) == "on":
        nbs = [(G.encoded(q), q) for q in (pixels_of(c["drawing"], g)
                                           for g in (f - 1, f + 1) if 0 <= g < FRAMES)]
        e = temporal(e, px, nbs, hs, mode, get(c, "motion_sensitivity", f),
                     get(c, "temporal_amount", f))
    s, b = get(c, "chroma_suppression", f), get(c, "noise_size_bias", f)
    for k in range(1, passes + 1):
        u = 0 if passes == 1 else 2 * (k - 1) / (passes - 1) - 1
        e = bilateral(e, px, k, [h * 2 ** (b * u) for h in hs], mode, s)
    a = get(c, "clean_solid_areas", f)
    if a > 0:
        e = bilateral(e, px, 1, [a * h for h in hs], mode, s)
    t = get(c, "texture", f)
    if t > 0:
        e = [[q[ch] + t * (oo[ch] - q[ch]) for ch in range(3)] if q is not None else None
             for q, oo in zip(e, o)]
    return [[srgb_to_linear(v) * p[3] for v in q] + [p[3]] if q is not None else list(p)
            for p, q in zip(px, e)]


def region(c, f):
    cx, cy = (v / 100 * size for v, size in zip(get(c, "preview_center", f), (W, H)))
    hw, hh = get(c, "preview_width", f) / 2, get(c, "preview_height", f) / 2
    return lambda x, y: abs(x + 0.5 - cx) < hw and abs(y + 0.5 - cy) < hh


def edged(px, inside):
    """`px` with the inside edge of the pixels `inside` names in opaque white."""
    out = [list(p) for p in px]
    for y in range(H):
        for x in range(W):
            if inside(x, y) and not all(inside(x + dx, y + dy)
                                        for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1))):
                out[y * W + x] = [1.0, 1.0, 1.0, 1.0]
    return out


def render(c, f):
    view = get(c, "viewing_mode", f)
    given = pixels_of(c["drawing"], f)
    if view == "noise_samples":
        return R.frame(edged(given, lambda x, y: 0 <= x < W and 0 <= y < H), 0)
    if view == "blending_matte":
        return R.frame([[p[3]] * 4 for p in given], 0)
    out = U.unsharp(denoise(c, f), get(c, "unsharp_amount", f), get(c, "unsharp_radius", f),
                    get(c, "unsharp_threshold", f))
    if view == "preview":
        inside = region(c, f)
        out = [out[i] if inside(i % W, i // W) else list(given[i]) for i in range(W * H)]
        if get(c, "show_box", f) == "on":
            out = edged(out, inside)
    return R.frame(out, 0)


# --- the cases ------------------------------------------------------------------------------

def case(drawing="grainy", **more):
    c = {"drawing": drawing, "noise_reduction": 1, "passes": 1, "mode": "multichannel",
         "unsharp_amount": 0, "unsharp_radius": 1, "unsharp_threshold": 0}
    c.update(more)
    return c


PREVIEW = {"viewing_mode": "preview", "preview_center": (25, 50), "preview_width": 8,
           "preview_height": 6}
TEMPORAL = {"temporal_filtering": "on"}

CASES = {
    "FX-REMOVEGRAIN-022": ("As After Effects adds it now (D-965, D-966): three passes, Chroma "
                           "Suppression 1, Texture 0.1, the rest neutral; a little grain passed "
                           "through.", case(**ADDED), (0,)),
    "FX-REMOVEGRAIN-023": ("Chroma Suppression 3 alone: colour grain averaged harder than "
                           "FX-REMOVEGRAIN-001.", case(chroma_suppression=3), (0,)),
    "FX-REMOVEGRAIN-024": ("Texture 0.5: half way between FX-REMOVEGRAIN-001 and the drawing.",
                           case(texture=0.5), (0,)),
    "FX-REMOVEGRAIN-025": ("Texture 1: all the grain passed through, the drawing as it is.",
                           case(texture=1), (0,)),
    "FX-REMOVEGRAIN-026": ("Noise Size Bias -1 over three passes.",
                           case(passes=3, noise_size_bias=-1), (0,)),
    "FX-REMOVEGRAIN-027": ("Noise Size Bias 1 over three passes.",
                           case(passes=3, noise_size_bias=1), (0,)),
    "FX-REMOVEGRAIN-028": ("Clean Solid Areas 2: one more pass, smoother than "
                           "FX-REMOVEGRAIN-001.", case(clean_solid_areas=2), (0,)),
    "FX-REMOVEGRAIN-029": ("Single Channel, Red Noise Reduction 0: red kept exactly, green and "
                           "blue smoothed.", case(mode="single_channel", red_noise_reduction=0),
                           (0,)),
    "FX-REMOVEGRAIN-030": ("Viewing Mode Preview, the region 8 by 6 pixels round 25 % 50 %: "
                           "inside FX-REMOVEGRAIN-001 with its edge white, outside the drawing.",
                           case(**PREVIEW), (0,)),
    "FX-REMOVEGRAIN-031": ("Preview with Show Box off: no white edge.",
                           case(show_box="off", **PREVIEW), (0,)),
    "FX-REMOVEGRAIN-032": ("Viewing Mode Noise Samples: the drawing as given, its edge white.",
                           case(viewing_mode="noise_samples"), (0,)),
    "FX-REMOVEGRAIN-033": ("Viewing Mode Blending Matte on `holes`: white at each pixel's "
                           "covering.", case("holes", viewing_mode="blending_matte"), (0,)),
    "FX-REMOVEGRAIN-034": ("Temporal Filtering on, as added (100 %, 0.9), on `seq`, grain drawn "
                           "again each frame: frames 0 and 4 have one neighbour, frame 2 two.",
                           case("seq", **TEMPORAL), (0, 2, 4)),
    "FX-REMOVEGRAIN-035": ("Temporal Filtering off on `seq`: each frame alone.",
                           case("seq"), (2,)),
    "FX-REMOVEGRAIN-036": ("Temporal Filtering Amount 50, Motion Sensitivity 0.3 on `cut`, "
                           "whose right half turns green at frame 3: the green does not streak "
                           "into frame 2.", case("cut", temporal_amount=50,
                                                 motion_sensitivity=0.3, **TEMPORAL), (2, 3)),
}

INVALID = {
    "FX-REMOVEGRAIN-037": ("Viewing Mode \"wide\", not one of its four words.",
                           case(viewing_mode="wide")),
    "FX-REMOVEGRAIN-038": ("Texture 2, above 1.", case(texture=2)),
    "FX-REMOVEGRAIN-039": ("Temporal Filtering \"maybe\", not \"off\" or \"on\".",
                           case(temporal_filtering="maybe")),
    "FX-REMOVEGRAIN-040": ("Noise Size Bias -2, below -1.", case(noise_size_bias=-2)),
}


# --- the project files ----------------------------------------------------------------------

def asset(name):
    if name in SEQUENCES:
        return {"id": f"asset-{name}", "kind": "image_sequence", "name": name,
                "pattern": f"{name}_####.png",
                "frames": {str(k): f"media/{name}_{k}.png" for k in range(1, FRAMES + 1)},
                "interpretation": {"color_space": "srgb", "alpha": "straight"}}
    return L.still(name, f"media/{name}.png")


def project_json(fx, c):
    spans = ([{"start_frame": f, "end_frame_exclusive": f + 1, "drawing_number": f + 1}
              for f in range(FRAMES)] if c["drawing"] in SEQUENCES else [])
    layer = L.raster("holder", f"asset-{c['drawing']}", spans=spans, out_frame=FRAMES)
    params = {k: (v if isinstance(v, str) else setting_json(v))
              for k, v in c.items() if k != "drawing"}
    layer["effects"] = [{"instance_id": "fx-0-0", "type_id": "core.remove_grain",
                         "enabled": True, "parameters": params}]
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [asset(c["drawing"])],
            "compositions": [L.composition("comp-main", W, H, FRAMES, [layer])]}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    for s in SEQUENCES:
        for f in range(FRAMES):
            (OUT / "media" / f"{s}_{f + 1}.png").write_bytes(png(DRAWINGS[f"{s}_{f + 1}"]))
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): [[float(v) for v in p] for p in render(c, f)] for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        print(fx, ", ".join(
            f"frame {f}: {sum(px[i] != R.frame(pixels_of(c['drawing'], int(f)), 0)[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = R.frame(pixels_of(c["drawing"], 0), 0)
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": before, "4": before},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(fx, "invalid")
    (OUT / "expected_remove_grain_more.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                         encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    old = json.loads((OUT / "expected_remove_grain.json").read_text(encoding="utf-8"))["cases"]
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    same = lambda f, g, e=1e-9: all(near(p, q, e) for p, q in zip(f, g))  # noqa: E731
    plain = lambda name, f=0: R.frame(pixels_of(name, f), 0)  # noqa: E731
    enc = lambda p: [G.S.linear_to_srgb(v / p[3]) for v in p[:3]]  # noqa: E731
    inside = [(x, y) for y in range(H) for x in range(W - 1) if x != 7]

    def rough(f, chans=range(3)):
        return sum((enc(f[at(x + 1, y)])[k] - enc(f[at(x, y)])[k]) ** 2
                   for x, y in inside for k in chans)

    def chroma(f):
        def ch(p):
            e = enc(p)
            m = sum(e) / 3
            return [v - m for v in e]
        return sum((ch(f[at(x + 1, y)])[k] - ch(f[at(x, y)])[k]) ** 2
                   for x, y in inside for k in range(3))

    # Every neutral value leaves D-455's picture: the old cases worked by this rule.
    for fx in ("FX-REMOVEGRAIN-001", "FX-REMOVEGRAIN-006", "FX-REMOVEGRAIN-007",
               "FX-REMOVEGRAIN-010", "FX-REMOVEGRAIN-011"):
        n = int(fx[-3:])
        oc = {**G.CASES[fx][1]}
        mine = case(oc.pop("drawing"), **oc)
        assert same(render(mine, 0), old[fx]["frames"]["0"], 1e-12), fx
    one, g = old["FX-REMOVEGRAIN-001"]["frames"]["0"], plain("grainy")
    added = c["FX-REMOVEGRAIN-022"]["0"]
    assert rough(added) < rough(g) and not same(added, one, 1e-6)
    assert chroma(c["FX-REMOVEGRAIN-023"]["0"]) < chroma(one)
    half = c["FX-REMOVEGRAIN-024"]["0"]
    assert rough(one) < rough(half) < rough(g)
    assert same(c["FX-REMOVEGRAIN-025"]["0"], g, 1e-9)
    assert not same(c["FX-REMOVEGRAIN-026"]["0"], c["FX-REMOVEGRAIN-027"]["0"], 1e-6)
    assert rough(c["FX-REMOVEGRAIN-028"]["0"]) < rough(one)
    r29 = c["FX-REMOVEGRAIN-029"]["0"]
    assert all(abs(r29[i][0] - g[i][0]) < 1e-12 for i in range(W * H))
    assert rough(r29, (1, 2)) < rough(g, (1, 2))
    p30, p31 = c["FX-REMOVEGRAIN-030"]["0"], c["FX-REMOVEGRAIN-031"]["0"]
    box = [at(x, y) for x in range(8) for y in (2, 7)] + [at(x, y) for x in (0, 7)
                                                          for y in range(2, 8)]
    assert all(p30[i] == [1.0] * 4 for i in box)
    assert all(p31[at(x, y)] == g[at(x, y)] for x in range(16) for y in range(10)
               if not (x < 8 and 2 <= y < 8))
    assert p31[at(3, 4)] == one[at(3, 4)] and p31[at(3, 4)] != g[at(3, 4)]
    p32 = c["FX-REMOVEGRAIN-032"]["0"]
    assert p32[at(0, 0)] == [1.0] * 4 and p32[at(5, 5)] == g[at(5, 5)]
    hp = plain("holes")
    assert all(c["FX-REMOVEGRAIN-033"]["0"][i] == [hp[i][3]] * 4 for i in range(W * H))
    t34, t35 = c["FX-REMOVEGRAIN-034"], c["FX-REMOVEGRAIN-035"]
    assert rough(t34["2"]) < rough(t35["2"]) < rough(plain("seq", 2))
    alone = [render(case("seq"), f) for f in (0, 4)]
    assert not same(t34["0"], alone[0], 1e-6) and not same(t34["4"], alone[1], 1e-6)
    # The green that comes in at frame 3 is too far in colour to be blended into frame 2.
    t36 = c["FX-REMOVEGRAIN-036"]["2"]
    assert all(enc(t36[at(x, y)])[1] < 0.35 for x in range(8, 16) for y in range(H))
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"]
    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(-1e-12 <= v <= p[3] + 1e-12 for v in p[:3]), name
    print("checked")


if __name__ == "__main__":
    main()
