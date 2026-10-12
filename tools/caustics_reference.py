"""Caustics, worked a second way.

D-631 adds Caustics (`core.caustics`), after After Effects' Caustics (Simulation): a bottom seen
through a water surface, bent by the waves, brightened and darkened where the waves gather and
spread the light, under a coloured, lit surface that can mirror a sky. Adobe's CS5 help names the
controls and says what several do (the bottom is the effect's own layer when none is chosen;
Refractive Index 1 bends nothing and water is about 1.2; Water Depth 0 bends nothing; the surface
comes from a layer's brightness, best a Wave World; Intensity is the sky's opacity; Caustics
Strength 0 makes none). It publishes no formula, and few ranges or settings when added, so those
are ours (document 21), and nothing is ported. It carries no time: each frame reads only that
frame's settings and layers (D-610's rule holds trivially).

The settings. Bottom: `bottom_layer` D-189's layer word, "" when added (the effect's own drawing);
`bottom_scaling` 0.1 to 10, 1; `bottom_repeat_mode` "once", "tiled" (when added) or "reflected";
`bottom_fit` "center" or "stretch" (when added), After Effects' If Layer Size Differs;
`bottom_blur` 0 to 50, 0. Water: `water_surface` a layer word, ""; `wave_height` 0 to 1, 0.2;
`smoothing` 0 to 50, 5; `water_depth` 0 to 1, 0.1; `refractive_index` 1 to 2, 1.2;
`surface_color` "#rrggbb", "#2a7fd4"; `surface_opacity` 0 to 1, 0.3; `caustics_strength` 0 to 1,
0. Sky: `sky_layer` a layer word, ""; `sky_scaling` 0.1 to 10, 1; `sky_repeat_mode` as the
bottom's, "tiled"; `sky_fit` as the bottom's, "stretch"; `sky_intensity` 0 to 1, 0.3;
`sky_convergence` 0 to 1, 0.2. Light and material, Card Wipe's (D-580): `light_type` "distant"
(when added) or "point"; `light_intensity` 0 to 10, 1; `light_color` "#rrggbb", "#ffffff";
`light_position` x then y, -1000 to 1000 per cent of the layer, [50, 50]; `light_height` 0 to
10 layer widths, 1; `ambient_light` 0.25, `diffuse_reflection` 0.75, `specular_reflection` 0,
each 0 to 2; `highlight_sharpness` 1 to 100, 5. Every number is keyable and held to its range;
the words are exact.

The rule, on a layer W by H pixels, at its pixel (x, y), its centre p = (x + 1/2, y + 1/2).

1. The pictures. The water surface is the named layer's picture stretched to the layer (D-189);
   the bottom and the sky are their layers' whole pictures, Q, w by h. A layer not begun is a
   clear picture. With no water surface named, or one not in the
   composition (D-189's warning), the water is flat. With no bottom named, the bottom is the
   layer's own drawing; one named but not in the composition is a clear bottom (and the
   warning). With no sky named, or one not there (and the warning), there is no sky.
2. Placing the bottom and the sky. With s the scaling, a = (x + 1/2 - W/2) / s + W/2 and
   b = (y + 1/2 - H/2) / s + H/2, the placed picture B at (x, y) is Q read bilinearly (pixel
   centres at halves) at (a - floor((W - w) / 2), b - floor((H - h) / 2)) for "center", or at
   (a w / W, b h / H) for "stretch"; its pixel indices i outside 0..w - 1 (or 0..h - 1) taken
   by the repeat mode: "once" clear, "tiled" i mod w, "reflected" m = i mod 2w, m or else
   2w - 1 - m. An empty Q places clear.
3. The blurs. G(sigma) is the Gaussian of taps exp(-i^2 / (2 sigma^2)), |i| <= ceil(3 sigma),
   summed to 1, run across and then down with the outer pixels held; sigma 0 leaves a picture
   as it is. The bottom's placed picture is blurred by G(bottom_blur W / 1000), each channel of
   its premultiplied light.
4. The water. L = document 21's luma (clamped to 0..1) of the surface picture at each pixel
   (0.5 everywhere for flat water), blurred by G(smoothing W / 1000); the surface's height
   toward the viewer is z = wave_height (L - 1/2) W / 4. Its slopes gx = (z(x + 1, y) -
   z(x - 1, y)) / 2, gy likewise down, the neighbours held inside the layer. In the layer's
   space (x right, y down, depth away from the viewer) the surface's normal is N = (-gx, -gy,
   -1) / sqrt(gx^2 + gy^2 + 1).
5. Bending. The view looks straight down the depth, I = (0, 0, 1). With e = 1 /
   refractive_index, c = -N_z, k = 1 - e^2 (1 - c^2), the bent ray T = e I + (e c - sqrt(k)) N,
   and the bottom, D = water_depth W below, is read at p + o, o = D (T_x, T_y) / T_z.
6. The caustics. With o's differences taken as z's are, det = (1 + dox/dx)(1 + doy/dy) -
   (dox/dy)(doy/dx), C = 1 / max(|det|, 0.1), and the light on the bottom M = 1 +
   caustics_strength (C - 1).
7. The bottom seen: b = the blurred placed bottom read bilinearly at p + o, the place held
   inside the layer's pixel centres; its colour times M, its alpha as read.
8. The sky. The view's mirror R = I - 2 (I.N) N; the placed sky (not blurred) read as b is at
   p + sky_convergence (W / 2) (R_x, R_y), (r, a) premultiplied, a clear (0, 0) with no sky.
   The surface's colour s = Sc (1 - sky_intensity a) + sky_intensity r, Sc the surface colour's
   linear light.
9. The light, Card Wipe's: P = (lx W / 100 - W / 2, ly H / 100 - H / 2, -light_height W);
   Lv = unit(P) for "distant", unit(P - h) for "point", h = (p_x - W/2, p_y - H/2, 0) (a zero
   step stays zero). V = (0, 0, -1), Hv = Lv + V, the highlight sp = max(N.Hv / |Hv|, 0)^sharpness
   where N.Lv > 0 and |Hv| > 0, else 0. With i the intensity, Lc the light's linear colour and
   A, D, S the ambient, diffuse and specular numbers, the lit surface
       s' = s (A + D i Lc max(N.Lv, 0)) + S i sp Lc.
10. The pixel, with O the surface opacity: (b_rgb M (1 - O) + s' O, b_alpha (1 - O) + O).

The layer never grows; buffer pixels outside it are clear. A draft draws the same picture on
fewer pixels, every length being a share of the layer.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawings' 8-bit values. The rule steps only where the highlight is cut at N.Lv = 0;
every case with a highlight is checked that no pixel lies near it.

Every case is a project of one composition 48 by 27, 9 frames at 24 a second, in
`Fixtures/caustics/`: the map layers, switched off, and on top of them the layer `holder`,
showing the drawing `shot`, with the effect. The expected pixels are in
`Fixtures/caustics/expected_caustics.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/caustics_reference.py
"""

import json
import math
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402
import compound_blur_reference as K  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "caustics"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES, FPS = 48, 27, 9, 24
CLEAR = [0.0, 0.0, 0.0, 0.0]


def shot_drawing():
    """The holder: a checker of 6-pixel squares, yellow and blue, the bottom right 6 by 4 clear."""
    def px(x, y):
        if x >= 42 and y >= 23:
            return (0, 0, 0, 0)
        return (230, 200, 60, 255) if (x // 6 + y // 6) % 2 == 0 else (40, 90, 170, 255)
    return [[px(x, y) for x in range(W)] for y in range(H)]


def ripples_drawing():
    """The water surface: smooth greys, crossing waves."""
    def grey(x, y):
        v = (0.5 + 0.3 * math.sin(2 * math.pi * (x + 0.5) / 16) * math.cos(2 * math.pi * (y + 0.5) / 13)
             + 0.12 * math.sin(2 * math.pi * (x + y + 1) / 9))
        g = round(255 * min(max(v, 0.0), 1.0))
        return (g, g, g, 255)
    return [[grey(x, y) for x in range(W)] for y in range(H)]


def floor_drawing():
    """A bottom smaller than the layer, 20 by 12: red and white stripes 3 high, a green line."""
    def px(x, y):
        if 9 <= x < 11:
            return (40, 200, 70, 255)
        return (220, 50, 40, 255) if (y // 3) % 2 == 0 else (245, 245, 235, 255)
    return [[px(x, y) for x in range(20)] for y in range(12)]


def sky_drawing():
    """A sky 24 by 14: orange on the left to pale blue on the right, darker at the top."""
    def px(x, y):
        f, t = x / 23, 0.6 + 0.4 * y / 13
        return tuple(round(c * t) for c in (255 + (200 - 255) * f, 150 + (225 - 150) * f,
                                            40 + (255 - 40) * f)) + (255,)
    return [[px(x, y) for x in range(24)] for y in range(14)]


DRAWINGS = {"shot": shot_drawing(), "ripples": ripples_drawing(), "floor": floor_drawing(),
            "sky": sky_drawing()}
MAPS = {"ripples": "ripples", "floor": "floor", "sky": "sky", "late": "ripples",
        "holder": "shot"}


def decoded(name):
    rows = DRAWINGS[name]
    return L.pic(len(rows[0]), len(rows),
                 [[srgb_to_linear(r / 255) * a / 255, srgb_to_linear(g / 255) * a / 255,
                   srgb_to_linear(b / 255) * a / 255, a / 255] for row in rows for r, g, b, a in row])


def map_picture(name, fit, frame):
    """Step 1: a named layer's picture, fitted to the layer or `whole`; None for one not in the
    composition."""
    if name not in MAPS:
        return None
    p = L.EMPTY if name == "late" and frame < 4 else decoded(MAPS[name])
    return p if fit == "whole" else L.fit(p, fit, W, H)


# --- the rule -------------------------------------------------------------------------------

def colour(hexa):
    return [srgb_to_linear(int(hexa[i:i + 2], 16) / 255) for i in (1, 3, 5)]


def unit_vec(v):
    n = math.sqrt(sum(q * q for q in v))
    return [0.0, 0.0, 0.0] if n == 0 else [q / n for q in v]


def wrap(i, n, mode):
    if 0 <= i < n:
        return i
    if mode == "tiled":
        return i % n
    if mode == "reflected":
        m = i % (2 * n)
        return m if m < n else 2 * n - 1 - m
    return None


def placed(q, s, mode, fit):
    """Step 2."""
    qw, qh = q["w"], q["h"]
    if qw == 0 or qh == 0:
        return [list(CLEAR) for _ in range(W * H)]
    out = []
    for y in range(H):
        for x in range(W):
            a, b = (x + 0.5 - W / 2) / s + W / 2, (y + 0.5 - H / 2) / s + H / 2
            if fit == "center":
                u, v = a - (W - qw) // 2 - 0.5, b - (H - qh) // 2 - 0.5
            else:
                u, v = a * qw / W - 0.5, b * qh / H - 0.5
            i0, j0 = math.floor(u), math.floor(v)
            fu, fv = u - i0, v - j0
            px = [0.0] * 4
            for dj, wv in ((0, 1 - fv), (1, fv)):
                for di, wu in ((0, 1 - fu), (1, fu)):
                    i, j = wrap(i0 + di, qw, mode), wrap(j0 + dj, qh, mode)
                    if i is None or j is None:
                        continue
                    s_ = L.at(q, i, j)
                    for c in range(4):
                        px[c] += s_[c] * wu * wv
            out.append(px)
    return out


def taps(sigma):
    if sigma <= 0:
        return [1.0]
    r = math.ceil(3 * sigma)
    t = [math.exp(-i * i / (2 * sigma * sigma)) for i in range(-r, r + 1)]
    total = sum(t)
    return [v / total for v in t]


def gaussian(vals, sigma):
    """Step 3, on a list of per-pixel lists, the outer pixels held."""
    t = taps(sigma)
    r = len(t) // 2
    if r == 0:
        return [list(v) for v in vals]
    n = len(vals[0])
    across = [[sum(t[k] * vals[y * W + min(max(x + k - r, 0), W - 1)][c] for k in range(len(t)))
               for c in range(n)] for y in range(H) for x in range(W)]
    return [[sum(t[k] * across[min(max(y + k - r, 0), H - 1) * W + x][c] for k in range(len(t)))
             for c in range(n)] for y in range(H) for x in range(W)]


def held(pic_list, sx, sy):
    """Bilinear at (sx, sy), pixel centres at halves, the place held inside the centres."""
    u = min(max(sx - 0.5, 0.0), W - 1.0)
    v = min(max(sy - 0.5, 0.0), H - 1.0)
    i0, j0 = math.floor(u), math.floor(v)
    i1, j1 = min(i0 + 1, W - 1), min(j0 + 1, H - 1)
    fu, fv = u - i0, v - j0
    a, b = pic_list[j0 * W + i0], pic_list[j0 * W + i1]
    c, d = pic_list[j1 * W + i0], pic_list[j1 * W + i1]
    return [(a[k] * (1 - fu) + b[k] * fu) * (1 - fv) + (c[k] * (1 - fu) + d[k] * fu) * fv
            for k in range(len(a))]


def diffs(v, x, y):
    """Step 4's held central differences of a per-pixel number list, across then down."""
    return ((v[y * W + min(x + 1, W - 1)] - v[y * W + max(x - 1, 0)]) / 2,
            (v[min(y + 1, H - 1) * W + x] - v[max(y - 1, 0) * W + x]) / 2)


def normals(k, surface):
    if surface is None:
        lum = [[0.5]] * (W * H)
    else:
        lum = [[K.luma(L.at(surface, x, y))] for y in range(H) for x in range(W)]
    lum = gaussian(lum, k["smoothing"] * W / 1000)
    z = [k["wave_height"] * (v[0] - 0.5) * W / 4 for v in lum]
    out = []
    for y in range(H):
        for x in range(W):
            gx, gy = diffs(z, x, y)
            n = math.sqrt(gx * gx + gy * gy + 1)
            out.append((-gx / n, -gy / n, -1 / n))
    return out


def bend(k, nn):
    """Step 5's offset per pixel."""
    e = 1 / k["refractive_index"]
    d = k["water_depth"] * W
    ox, oy = [], []
    for n in nn:
        c = -n[2]
        kk = 1 - e * e * (1 - c * c)
        f = e * c - math.sqrt(kk)
        t = (f * n[0], f * n[1], e + f * n[2])
        ox.append(d * t[0] / t[2])
        oy.append(d * t[1] / t[2])
    return ox, oy


def light_at(k, n, x, y):
    """Step 9: N.Lv and the highlight."""
    lp = k["light_position"]
    pl = [lp[0] * W / 100 - W / 2, lp[1] * H / 100 - H / 2, -k["light_height"] * W]
    if k["light_type"] == "point":
        pl = [pl[0] - (x + 0.5 - W / 2), pl[1] - (y + 0.5 - H / 2), pl[2]]
    lv = unit_vec(pl)
    hv = [lv[0], lv[1], lv[2] - 1]
    nl = sum(n[i] * lv[i] for i in range(3))
    hn = math.sqrt(sum(q * q for q in hv))
    sp = 0.0
    if nl > 0 and hn > 0:
        sp = max(sum(n[i] * hv[i] for i in range(3)) / hn, 0.0) ** k["highlight_sharpness"]
    return nl, sp


def caustics(k, drawing, frame):
    """Steps 1 to 10 for the whole layer, `drawing` the layer's own picture."""
    if k["bottom_layer"] == "":
        q = drawing
    else:
        q = map_picture(k["bottom_layer"], "whole", frame)
        if q is None:
            q = L.EMPTY
    bottom = gaussian(placed(q, k["bottom_scaling"], k["bottom_repeat_mode"], k["bottom_fit"]),
                      k["bottom_blur"] * W / 1000)
    sky_q = map_picture(k["sky_layer"], "whole", frame) if k["sky_layer"] else None
    sky = None if sky_q is None else placed(sky_q, k["sky_scaling"], k["sky_repeat_mode"],
                                            k["sky_fit"])
    surface = map_picture(k["water_surface"], "stretch", frame) if k["water_surface"] else None
    nn = normals(k, surface)
    ox, oy = bend(k, nn)
    sc, lc = colour(k["surface_color"]), colour(k["light_color"])
    i_, A, D, S = (k["light_intensity"], k["ambient_light"], k["diffuse_reflection"],
                   k["specular_reflection"])
    si, conv, op, cs = (k["sky_intensity"], k["sky_convergence"], k["surface_opacity"],
                        k["caustics_strength"])
    out = []
    for y in range(H):
        for x in range(W):
            j = y * W + x
            n = nn[j]
            dxx, dxy = diffs(ox, x, y)
            dyx, dyy = diffs(oy, x, y)
            det = (1 + dxx) * (1 + dyy) - dxy * dyx
            m = 1 + cs * (1 / max(abs(det), 0.1) - 1)
            b = held(bottom, x + 0.5 + ox[j], y + 0.5 + oy[j])
            if sky is None:
                r = CLEAR
            else:
                rx, ry = -2 * n[2] * n[0], -2 * n[2] * n[1]
                r = held(sky, x + 0.5 + conv * W / 2 * rx, y + 0.5 + conv * W / 2 * ry)
            s = [sc[c] * (1 - si * r[3]) + si * r[c] for c in range(3)]
            nl, sp = light_at(k, n, x, y)
            lit = [s[c] * (A + D * i_ * lc[c] * max(nl, 0.0)) + S * i_ * sp * lc[c]
                   for c in range(3)]
            out.append([b[c] * m * (1 - op) + lit[c] * op for c in range(3)]
                       + [b[3] * (1 - op) + op])
    return out


# --- the cases ------------------------------------------------------------------------------

NUMBERS = ("bottom_scaling", "bottom_blur", "wave_height", "smoothing", "water_depth",
           "refractive_index", "surface_opacity", "caustics_strength", "sky_scaling",
           "sky_intensity", "sky_convergence", "light_intensity", "light_position",
           "light_height", "ambient_light", "diffuse_reflection", "specular_reflection",
           "highlight_sharpness")
WORDS = ("bottom_layer", "bottom_repeat_mode", "bottom_fit", "water_surface", "surface_color",
         "sky_layer", "sky_repeat_mode", "sky_fit", "light_type", "light_color")
DEFAULTS = {
    "bottom_layer": "", "bottom_scaling": 1, "bottom_repeat_mode": "tiled",
    "bottom_fit": "stretch", "bottom_blur": 0, "water_surface": "", "wave_height": 0.2,
    "smoothing": 5, "water_depth": 0.1, "refractive_index": 1.2, "surface_color": "#2a7fd4",
    "surface_opacity": 0.3, "caustics_strength": 0, "sky_layer": "", "sky_scaling": 1,
    "sky_repeat_mode": "tiled", "sky_fit": "stretch", "sky_intensity": 0.3,
    "sky_convergence": 0.2, "light_type": "distant", "light_intensity": 1,
    "light_color": "#ffffff", "light_position": [50, 50], "light_height": 1,
    "ambient_light": 0.25, "diffuse_reflection": 0.75, "specular_reflection": 0,
    "highlight_sharpness": 5,
}


def case(shift=(0, 0), in_frame=0, **kw):
    c = dict(DEFAULTS, shift=shift, in_frame=in_frame)
    for n, v in kw.items():
        assert n in DEFAULTS, n
        c[n] = v
    return c


def water(**kw):
    """The ripples as the water surface, stronger waves than when added, and a deeper floor."""
    return case(**dict(dict(water_surface="ripples", wave_height=0.4, water_depth=0.2), **kw))


def settings(c, frame):
    k = {n: value_at(c[n], frame) for n in NUMBERS}
    k.update({w: c[w] for w in WORDS})
    return k


def render(c, frame):
    if frame < c["in_frame"]:
        return [list(CLEAR) for _ in range(W * H)]
    out = L.pic(W, H, caustics(settings(c, frame), decoded("shot"), frame))
    dx, dy = c["shift"]
    return [list(L.at(out, x - dx, y - dy)) for y in range(H) for x in range(W)]


def plain(c):
    dx, dy = c["shift"]
    o = decoded("shot")
    return [list(L.at(o, x - dx, y - dy)) for y in range(H) for x in range(W)]


SPOT = dict(light_type="point", light_position=[15, 20], light_height=0.8,
            specular_reflection=1, highlight_sharpness=20)

CASES = {
    "FX-CA-001": ("As added, no water surface: flat water, the checker seen straight through, "
                  "30% of the blue surface over it, lit evenly.", case(), (0,)),
    "FX-CA-002": ("The ripples as the water surface, wave height 0.4, depth 0.2: the checker "
                  "bent by the waves.", water(), (0,)),
    "FX-CA-003": ("FX-CA-002 with wave height 0.8 and depth 0.4: bent much further.",
                  water(wave_height=0.8, water_depth=0.4), (0,)),
    "FX-CA-004": ("Refractive index 1 and surface opacity 0: nothing bends and nothing covers, "
                  "the checker exactly.", water(refractive_index=1, surface_opacity=0), (0,)),
    "FX-CA-005": ("Water depth 0 and surface opacity 0: the checker exactly.",
                  water(water_depth=0, surface_opacity=0), (0,)),
    "FX-CA-006": ("Caustics strength 1: bright and dark patches where the waves gather and "
                  "spread the light.", water(caustics_strength=1), (0,)),
    "FX-CA-007": ("Smoothing 40: the waves smoothed before they bend the checker.",
                  water(smoothing=40), (0,)),
    "FX-CA-008": ("Smoothing 0 and refractive index 2: unsmoothed and bent hard.",
                  water(smoothing=0, refractive_index=2), (0,)),
    "FX-CA-009": ("Bottom blur 25: the checker soft under the waves.", water(bottom_blur=25),
                  (0,)),
    "FX-CA-010": ("The `floor` layer (20 by 12) as the bottom, stretched to the layer: red and "
                  "white stripes and a green line, bent.", water(bottom_layer="floor"), (0,)),
    "FX-CA-011": ("The floor centred, once: the stripes in the middle, only the surface round "
                  "them.", water(bottom_layer="floor", bottom_fit="center",
                                 bottom_repeat_mode="once"), (0,)),
    "FX-CA-012": ("The floor centred, tiled at scaling 0.5: many small copies.",
                  water(bottom_layer="floor", bottom_fit="center", bottom_scaling=0.5), (0,)),
    "FX-CA-013": ("The floor centred, reflected at scaling 0.5: copies mirrored at their "
                  "edges.", water(bottom_layer="floor", bottom_fit="center", bottom_scaling=0.5,
                                  bottom_repeat_mode="reflected"), (0,)),
    "FX-CA-014": ("The checker as the bottom at scaling 2.3, tiled: larger squares.",
                  water(bottom_scaling=2.3), (0,)),
    "FX-CA-015": ("The `sky` layer stretched, intensity 1, surface opacity 0.8: the sky mirrored "
                  "on the water, bent by the waves.",
                  water(sky_layer="sky", sky_intensity=1, surface_opacity=0.8), (0,)),
    "FX-CA-016": ("The sky centred, reflected at scaling 0.7, convergence 0.6, intensity 0.7.",
                  water(sky_layer="sky", sky_fit="center", sky_repeat_mode="reflected",
                        sky_scaling=0.7, sky_convergence=0.6, sky_intensity=0.7,
                        surface_opacity=0.8), (0,)),
    "FX-CA-017": ("Flat water, an orange surface fully opaque, a point light low at the top "
                  "left: brighter near the light.",
                  case(surface_color="#ff8000", surface_opacity=1, light_type="point",
                       light_position=[15, 20], light_height=0.4), (0,)),
    "FX-CA-018": ("The ripples under that point light with a sharp highlight, surface opacity "
                  "0.6.", water(surface_opacity=0.6, **SPOT), (0,)),
    "FX-CA-019": ("A pink light at intensity 2, ambient 0.1, diffuse 1.2, from the left.",
                  water(surface_opacity=0.7, light_color="#ff80c0", light_intensity=2,
                        ambient_light=0.1, diffuse_reflection=1.2, light_position=[-50, 50]),
                  (0,)),
    "FX-CA-020": ("A water surface whose in point is frame 4, `late`: flat before it (a clear "
                  "picture, every pixel the same), the ripples after.",
                  water(water_surface="late"), (2, 6)),
    "FX-CA-021": ("A water surface not in the composition, `gone`: flat water, and the warning.",
                  water(water_surface="gone"), (0,)),
    "FX-CA-022": ("A bottom not in the composition, `gone`: a clear bottom, only the surface, "
                  "and the warning.", water(bottom_layer="gone"), (0,)),
    "FX-CA-023": ("A sky not in the composition, `gone`: no sky, as FX-CA-002, and the warning.",
                  water(sky_layer="gone"), (0,)),
    "FX-CA-024": ("Wave height keyed 0 at frame 0 to 0.8 at frame 8: flat at frame 0, then the "
                  "waves grow.", water(wave_height=keyed((0, 0), (8, 0.8))), (0, 4, 8)),
    "FX-CA-025": ("FX-CA-002 on the holder moved 3 right and 2 down: the same, moved.",
                  water(shift=(3, 2)), (0,)),
    "FX-CA-026": ("The holder's in point at frame 2: nothing at frame 1, FX-CA-002 at frame 3.",
                  water(in_frame=2), (1, 3)),
    "FX-CA-027": ("The holder named as its own bottom: its own drawing, as FX-CA-002.",
                  water(bottom_layer="holder"), (0,)),
    "FX-CA-028": ("Everything at once: caustics, sky, blur and a highlight.",
                  water(caustics_strength=0.7, sky_layer="sky", sky_intensity=0.6,
                        bottom_blur=10, surface_opacity=0.5, **SPOT), (0,)),
}

INVALID = {
    "FX-CA-029": ("Refractive index 0.9, below 1.", water(refractive_index=0.9)),
    "FX-CA-030": ("Bottom scaling 0, below 0.1.", water(bottom_scaling=0)),
    "FX-CA-031": ("Bottom blur 60, above 50.", water(bottom_blur=60)),
    "FX-CA-032": ("Bottom repeat mode \"mirror\".", water(bottom_repeat_mode="mirror")),
    "FX-CA-033": ("Sky fit \"tile\", which Caustics does not offer.", water(sky_fit="tile")),
    "FX-CA-034": ("Surface colour \"blue\".", water(surface_color="blue")),
    "FX-CA-035": ("Light type \"spot\".", water(light_type="spot")),
    "FX-CA-036": ("A water surface written as the number 3, not a word.", water(water_surface=3)),
    "FX-CA-037": ("Light position with one number.", water(light_position=[10])),
    "FX-CA-038": ("Smoothing keyed to 60 at frame 4.", water(smoothing=keyed((0, 5), (4, 60)))),
}


def effect(fid, c):
    params = {w: c[w] for w in WORDS}
    params.update({n: setting_json(c[n]) for n in NUMBERS})
    return {"instance_id": fid, "type_id": "core.caustics", "enabled": True,
            "parameters": params}


def map_layers():
    return [L.raster("ripples", "asset-ripples", enabled=False, out_frame=FRAMES),
            L.raster("floor", "asset-floor", enabled=False, out_frame=FRAMES),
            L.raster("sky", "asset-sky", enabled=False, out_frame=FRAMES),
            L.raster("late", "asset-ripples", enabled=False, in_frame=4, out_frame=FRAMES)]


def project_json(fx, c):
    holder = L.raster("holder", "asset-shot", position=c["shift"], in_frame=c["in_frame"],
                      out_frame=FRAMES)
    holder["effects"] = [effect("fx-1", c)]
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [L.still(n, f"media/{n}.png") for n in DRAWINGS],
            "compositions": [L.composition("comp-main", W, H, FRAMES, map_layers() + [holder])]}


def write(name, p):
    (OUT / name).write_text(json.dumps(p, indent=2) + "\n", encoding="utf-8")
    return name


def highlight_clear(fx, c, frames):
    """No pixel near the highlight's cut, N.Lv = 0, where there is a highlight."""
    for f in frames:
        if f < c["in_frame"]:
            continue
        k = settings(c, f)
        if k["specular_reflection"] == 0:
            continue
        surface = map_picture(k["water_surface"], "stretch", f) if k["water_surface"] else None
        nn = normals(k, surface)
        for y in range(H):
            for x in range(W):
                nl, _ = light_at(k, nn[y * W + x], x, y)
                assert abs(nl) > 1e-3, (fx, f, x, y)


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        highlight_clear(fx, c, frames)
        rendered = {str(f): render(c, f) for f in frames}
        entry = {"says": says, "project": write(f"{fx.lower().replace('-', '_')}.json",
                                                project_json(fx, c)), "frames": rendered}
        if "gone" in (c["water_surface"], c["bottom_layer"], c["sky_layer"]):
            entry["warning"] = "EFFECT_LAYER_MISSING"
        expected["cases"][fx] = entry
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(1 for p, q in zip(px, plain(c)) if max(abs(a - b) for a, b in zip(p, q)) > 1e-4)}"
            " off the drawing" for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        before = plain(c)
        expected["cases"][fx] = {"says": says, "project": write(
            f"{fx.lower().replace('-', '_')}.json", project_json(fx, c)),
            "frames": {"0": before, "4": before}, "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_caustics.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                encoding="utf-8")
    check(expected)


def near(a, b, eps=1e-12):
    return all(abs(x - y) <= eps for p, q in zip(a, b) for x, y in zip(p, q))


def hand_checks():
    """Snell's law and the mirror worked by hand on one slope."""
    k = dict(refractive_index=1.5, water_depth=0.5)
    gx = 0.3
    n = math.sqrt(gx * gx + 1)
    nn = [(-gx / n, 0.0, -1 / n)]
    ox, oy = bend(k, nn)
    # The surface leans by atan(0.3); the ray bends by asin(sin(lean) / 1.5) from the normal,
    # so it leaves the vertical by lean - that, towards the rise.
    lean = math.atan(gx)
    want = W / 2 * math.tan(lean - math.asin(math.sin(lean) / 1.5))
    assert abs(ox[0] - want) < 1e-12 and oy[0] == 0.0 and ox[0] > 0
    # Index 1 bends nothing.
    assert bend(dict(refractive_index=1, water_depth=0.5), nn)[0][0] == 0.0
    # The mirror's lean is twice the surface's.
    rx, rz = -2 * nn[0][2] * nn[0][0], 1 - 2 * nn[0][2] ** 2
    assert abs(math.atan2(rx, -rz) + 2 * lean) < 1e-12
    # The blur's taps sum to 1 and are even.
    t = taps(1.7)
    assert abs(sum(t) - 1) < 1e-15 and len(t) == 13 and abs(t[0] - t[-1]) < 1e-18
    # Repeat modes.
    assert [wrap(i, 4, "tiled") for i in (-1, 4, 9)] == [3, 0, 1]
    assert [wrap(i, 4, "reflected") for i in (-1, 4, 7, 8)] == [0, 3, 0, 0]
    assert wrap(-1, 4, "once") is None


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    hand_checks()
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    shot = plain(case())
    sc = colour("#2a7fd4")
    # As added: the checker, 30% of the surface colour over it.
    flat = [[p[i] * 0.7 + sc[i] * 0.3 for i in range(3)] + [p[3] * 0.7 + 0.3] for p in shot]
    assert near(c["FX-CA-001"]["0"], flat, 1e-12)
    # Nothing bends and nothing covers: the checker exactly.
    assert near(c["FX-CA-004"]["0"], shot, 1e-12) and near(c["FX-CA-005"]["0"], shot, 1e-12)
    # The waves bend; the caustics change the light; more bending differs.
    assert not near(c["FX-CA-002"]["0"], flat, 1e-3)
    assert not near(c["FX-CA-006"]["0"], c["FX-CA-002"]["0"], 1e-3)
    assert not near(c["FX-CA-003"]["0"], c["FX-CA-002"]["0"], 1e-3)
    # The floor centred once: the corner pixel shows only the surface.
    corner = c["FX-CA-011"]["0"][0]
    assert abs(corner[3] - 0.3) < 1e-12
    # Flat water under a point light: brighter near the light than far from it.
    lit = c["FX-CA-017"]["0"]
    assert lit[int(0.2 * H) * W + int(0.15 * W)][0] > lit[(H - 1) * W + W - 1][0]
    # The late surface is flat before its in point, the ripples after (as FX-CA-002).
    late = c["FX-CA-020"]["2"]
    assert near(late, [[p[i] * 0.7 + sc[i] * 0.3 for i in range(3)] + [p[3] * 0.7 + 0.3]
                       for p in shot], 1e-12)
    assert near(c["FX-CA-020"]["6"], c["FX-CA-002"]["0"], 1e-12)
    # A surface not there is flat water, as added.
    assert near(c["FX-CA-021"]["0"], flat, 1e-12)
    # A bottom not there: every pixel only the surface, alpha 0.3.
    assert all(abs(p[3] - 0.3) < 1e-12 for p in c["FX-CA-022"]["0"])
    # A sky not there is no sky.
    assert near(c["FX-CA-023"]["0"], c["FX-CA-002"]["0"], 1e-12)
    # Keyed wave height: frame 0 flat, then growing.
    assert near(c["FX-CA-024"]["0"], flat, 1e-12)
    assert not near(c["FX-CA-024"]["4"], c["FX-CA-024"]["8"], 1e-3)
    # Moved: the same picture three right and two down; clear where the layer is not.
    moved, base = c["FX-CA-025"]["0"], c["FX-CA-002"]["0"]
    assert all(moved[(y + 2) * W + x + 3] == base[y * W + x]
               for y in range(H - 2) for x in range(W - 3))
    assert moved[0] == CLEAR
    # The in point.
    assert all(p == CLEAR for p in c["FX-CA-026"]["1"])
    assert near(c["FX-CA-026"]["3"], c["FX-CA-002"]["0"], 1e-12)
    # The holder as its own bottom.
    assert near(c["FX-CA-027"]["0"], c["FX-CA-002"]["0"], 1e-12)
    print("checks passed")


if __name__ == "__main__":
    main()
