"""Soft Physical Glow, worked a second way.

D-353 adds the shared soft-glow engine (P0-21 in `docs/effects/EFFECTS.md`) and proves it with
pick #1, Soft Physical Glow: a new mode of `core.glow`, chosen by the setting `falloff` written
"physical". A file whose Glow has no `falloff`, or `falloff` "classic", is the Glow of D-89 and
is not touched. The rule is our own reading of `docs/effects/PLUGINS.md` section 2.3 (Deep Glow)
and section 5; no plugin code or presets are used, and the docs never describe the product's
kernel, so the spread below is our own design. Document 21 is the rule in words; this file is
the reference for the numbers document 25 pins against it.

The rule, in the order it runs (PLUGINS.md 2.3, "Order of stages"):

1. **Input.** The layer's colour, already multiplied by its covering (premultiplied, linear), so
   a half-covered pixel counts at half its brightness. Threshold Mode "chroma" tests each of red,
   green and blue on its own; "luminance" tests one Rec. 709 brightness. Saturation Bias b moves
   the tested value towards the pixel's saturation (b > 0) or towards one minus it (b < 0):
   v' = (1 - |b|) v + |b| s, s = (max - min) / max. With Threshold t and Smooth m, a value at or
   over t glows fully; with m > 0 a value between t (1 - m) and t glows in proportion; at
   Threshold 0 everything glows and Smooth and Bias do nothing.
2. **Spread.** S = Radius / 3. Levels k = 0, 1, 2 ... have Gaussian sizes sigma_k = S / 2^k, a
   doubling (SPACING = 2) so the number of levels follows Radius (11 at 2,000). Each level fades
   in as f = clamp(2 sigma - 1, 0, 1), F is their sum, a level weighs f / max(F, 1) and the light
   itself keeps max(0, 1 - F): equal weight per doubling gives the inverse-square tail, the total
   always sums to one, and no level jumps in as Radius is animated.
   Aspect Ratio a (0 to 2) and Angle stretch each level: sigma a along the angle and
   sigma (2 - a) across it (below 1 taller, above 1 wider). The stretched Gaussian is made of two
   one-way passes, document 21's normalised weights out to ceil(3 sigma): first along x (or y)
   with what the second pass leaves, then along a slanted line (mu, 1) (or (1, mu)), reading
   between two cells by straight-line interpolation.
   **Downsampling, fixed (no quality setting):** a level of round size sigma is worked on cells of
   d by d pixels, d the largest power of two with 8 d <= sigma (at least 1), so a level is always
   blurred at 4 to 8 cells (at 2 to 4 cells the step as a level moves to coarser cells was seven
   times larger, 0.0025 against 0.00036 in linear light, and could be seen when Radius is
   animated); cells are plain averages, laid from the layer's own top-left corner,
   and are read back by bilinear interpolation at ((x + 0.5) / d - 0.5).
3. **Output.** The sum times Exposure is the glow. Blend Mode "screen" adds and then holds each
   channel at 1 (PLUGINS.md's Screen, not the usual formula), so it can never pass white; "add"
   adds and is not cut off. Unmult on: the glow's covering is its brightest channel, held at 1;
   off: the glow sits on solid black. Then the untouched layer goes back on top at Source Opacity
   with the same mode.

The layer grows by as far as any level's cells reach, so no glow is cut off at the layer's edge,
and a glowing pixel off the composition's edge still lights the frame (no silent crop).

**This file never runs the build's code path.** It works in double precision with numpy over
whole padded arrays of cells, on a plane with no edges at all, where the build works in single
precision, pixel by pixel, on buffers it sizes itself; the two agree to far inside the tolerance.

Every case is a composition 16 pixels by 10 holding one drawing the same size, unmoved unless
the case says. The drawings go into `Fixtures/soft_glow/media`, the projects into
`Fixtures/soft_glow`, and the expected frames into `Fixtures/soft_glow/expected_soft_glow.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/soft_glow_reference.py
"""

import json
import math
import sys
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear, prop  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402

W, H = 16, 10
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "soft_glow"
TOLERANCE = 2e-5  # document 25's default for a filter
MAX_RADIUS, MAX_EXPOSURE, MAX_ASPECT, MAX_ANGLE = 2000, 100, 2, 3600
SPACING = 2.0


# --- the rule -------------------------------------------------------------------------------

def working(p):
    """Document 21's PNG reading: sRGB to linear, then premultiplied."""
    a = p[3] / 255
    return [srgb_to_linear(p[c] / 255) * a for c in range(3)] + [a]


def light(px, c):
    """Stage 1: the light that glows, red, green and blue, from the premultiplied pixels
    (an array H x W x 4)."""
    rgb = px[..., :3]
    t = c["threshold"] / 100
    if t == 0:
        return rgb.copy()
    hi, lo = rgb.max(axis=-1), rgb.min(axis=-1)
    sat = np.where(hi > 0, (hi - lo) / np.where(hi > 0, hi, 1), 0.0)[..., None]
    if c["threshold_mode"] == "luminance":
        v = (0.2126 * rgb[..., 0] + 0.7152 * rgb[..., 1] + 0.0722 * rgb[..., 2])[..., None]
    else:
        v = rgb
    b = c["saturation_bias"] / 100
    if b > 0:
        v = (1 - b) * v + b * sat
    elif b < 0:
        v = (1 + b) * v - b * (1 - sat)
    m = c["threshold_smooth"] / 100
    # Every hard step is kept clear of its edge, so single precision decides it the same way.
    for edge in (t, t * (1 - m)):
        assert np.abs(v - edge).min() > 1e-3, edge
    if m == 0:
        w = (v >= t).astype(float)
    else:
        w = np.clip((v - t * (1 - m)) / (t * m), 0.0, 1.0)
    return rgb * w


def taps(var):
    """Document 21's Gaussian for a variance, normalised after it is cut at ceil(3 sigma)."""
    if var <= 0:
        return np.array([1.0]), 0
    sigma = math.sqrt(var)
    r = math.ceil(3 * sigma)
    k = np.exp(-(np.arange(-r, r + 1) ** 2) / (2 * var))
    return k / k.sum(), r


def down_factor(sigma):
    d = 1
    while 8 * d <= sigma:
        d *= 2
    return d


def levels(radius):
    """(sigma, weight) for every level, and the light's own weight."""
    s = radius / 3
    out = []
    k = 0
    while True:
        sigma = s / SPACING ** k
        f = min(1.0, max(0.0, 2 * sigma - 1))
        if f == 0:
            break
        out.append([sigma, f])
        k += 1
    total = sum(f for _, f in out)
    for lv in out:
        lv[1] /= max(total, 1.0)
    return out, max(0.0, 1 - total)


def shifted(a, dy, dx):
    """`a` moved by whole cells, zeros coming in (the padded plane has room)."""
    out = np.zeros_like(a)
    h, w = a.shape[:2]
    ys, yd = (slice(0, h - dy), slice(dy, h)) if dy >= 0 else (slice(-dy, h), slice(0, h + dy))
    xs, xd = (slice(0, w - dx), slice(dx, w)) if dx >= 0 else (slice(-dx, w), slice(0, w + dx))
    out[yd, xd] = a[ys, xs]
    return out


def spread_level(cells, sigma, d, aspect, angle):
    """One level's stretched Gaussian on its cells: two one-way passes."""
    th = math.radians(angle)
    su, sv = sigma * aspect / d, sigma * (2 - aspect) / d
    co, si = math.cos(th), math.sin(th)
    a = su * su * co * co + sv * sv * si * si
    b = (su * su - sv * sv) * co * si
    c = su * su * si * si + sv * sv * co * co
    if c >= a:
        # x first with what is left, then along (mu, 1) with variance c.
        first, f_axis = a - b * b / c, (0, 1)
        second, mu, along_y = c, b / c, True
    else:
        first, f_axis = c - b * b / a, (1, 0)
        second, mu, along_y = a, b / a, False
    k, r = taps(max(0.0, first))
    out = np.zeros_like(cells)
    for i, wt in enumerate(k):
        t = i - r
        out += wt * shifted(cells, t * f_axis[0], t * f_axis[1])
    k, r = taps(second)
    res = np.zeros_like(cells)
    for i, wt in enumerate(k):
        t = i - r
        off = mu * t
        lo = math.floor(off)
        fr = off - lo
        # The value at offset t along the line: one cell t away along the line's whole axis,
        # and between two cells along the other.
        for o, ow in ((lo, 1 - fr), (lo + 1, fr)):
            if ow == 0:
                continue
            if along_y:
                res += wt * ow * shifted(out, -t, -o)
            else:
                res += wt * ow * shifted(out, -o, -t)
    return res


def glow_layer(px, c, xs, ys):
    """The finished layer at the layer's own pixels xs (columns) and ys (rows), which may lie
    outside it, in the space the glow grows into."""
    lit = light(px, c)
    lvls, own = levels(c["radius"])
    gx = np.zeros((len(ys), len(xs), 3))
    X, Y = np.array(xs, float), np.array(ys, float)
    inside = lambda i, j: (0 <= i < W) and (0 <= j < H)  # noqa: E731
    if own > 0:
        for jy, y in enumerate(ys):
            for ix, x in enumerate(xs):
                if inside(x, y):
                    gx[jy, ix] += own * lit[y, x]
    for sigma, wt in lvls:
        d = down_factor(sigma)
        nw, nh = -(-W // d), -(-H // d)
        pad = 4 * math.ceil(3 * sigma * 2 / d) + 64
        cells = np.zeros((nh + 2 * pad, nw + 2 * pad, 3))
        for j in range(nh):
            for i in range(nw):
                block = lit[j * d:(j + 1) * d, i * d:(i + 1) * d]
                cells[pad + j, pad + i] = block.sum(axis=(0, 1)) / (d * d)
        blurred = spread_level(cells, sigma, d, c["aspect_ratio"], c["aspect_angle"])
        u, v = (X + 0.5) / d - 0.5, (Y + 0.5) / d - 0.5
        u0, v0 = np.floor(u).astype(int), np.floor(v).astype(int)
        fu, fv = u - u0, v - v0
        for jy in range(len(ys)):
            for ix in range(len(xs)):
                for dj, wj in ((0, 1 - fv[jy]), (1, fv[jy])):
                    for di, wi in ((0, 1 - fu[ix]), (1, fu[ix])):
                        gx[jy, ix] += wt * wj * wi * blurred[pad + v0[jy] + dj, pad + u0[ix] + di]
    g = gx * c["exposure"]
    screen = c["operation"] == "screen"
    if screen:
        g = np.minimum(g, 1.0)
    if c["unmult"] == "on":
        ga = np.minimum(1.0, g.max(axis=-1))
    else:
        ga = np.ones(g.shape[:2])
    o = c["source_opacity"] / 100
    out = np.zeros((len(ys), len(xs), 4))
    for jy, y in enumerate(ys):
        for ix, x in enumerate(xs):
            s = px[y, x] if inside(x, y) else np.zeros(4)
            rgb = g[jy, ix] + o * s[:3]
            if screen:
                rgb = np.minimum(rgb, 1.0)
            out[jy, ix, :3] = rgb
            out[jy, ix, 3] = min(1.0, ga[jy, ix] + o * s[3])
    return out


def settings_at(c, frame_no):
    out = dict(c)
    for k, top, bottom in (("threshold", 100, 0), ("threshold_smooth", 100, 0),
                           ("saturation_bias", 100, -100), ("radius", MAX_RADIUS, 0),
                           ("exposure", MAX_EXPOSURE, 0), ("aspect_ratio", MAX_ASPECT, 0),
                           ("aspect_angle", MAX_ANGLE, -MAX_ANGLE),
                           ("source_opacity", 100, 0)):
        out[k] = min(top, max(bottom, float(value_at(c[k], frame_no))))
    return out


# --- drawings -------------------------------------------------------------------------------

BRIGHT = (250, 220, 120, 255)  # warm yellow, linear 0.955 0.716 0.188
EDGE = (153, 102, 51, 255)     # brown, linear 0.318 0.133 0.033
DARK = (60, 40, 110, 255)      # purple, linear 0.045 0.021 0.155
GREY = (200, 200, 200, 255)    # grey, linear 0.578 each, saturation 0
NONE = S.NONE


def patches(middle=EDGE, cover=255):
    """Three patches in rows 3 to 6 on nothing: yellow in columns 2 to 4, the middle colour in 7
    and 8, purple in 11 to 13."""
    def colour(x, y):
        if not 3 <= y <= 6:
            return NONE
        for first, last, p in ((2, 4, BRIGHT), (7, 8, middle), (11, 13, DARK)):
            if first <= x <= last:
                return p[:3] + (cover,)
        return NONE
    return [[colour(x, y) for x in range(W)] for y in range(H)]


DRAWINGS = {
    "patches": patches(),
    "faint": patches(cover=128),  # the same, half covering
    "grey": patches(middle=GREY),  # the brown swapped for a grey
}


def pixels(name):
    return np.array([[working(p) for p in row] for row in DRAWINGS[name]])


# --- the cases ------------------------------------------------------------------------------

def case(name="patches", threshold_mode="chroma", threshold=0, threshold_smooth=0,
         saturation_bias=0, radius=12, exposure=1, aspect_ratio=1, aspect_angle=0,
         operation="screen", source_opacity=100, unmult="on", falloff="physical", shift=0):
    return {"drawing": name, "falloff": falloff, "threshold_mode": threshold_mode,
            "threshold": threshold, "threshold_smooth": threshold_smooth,
            "saturation_bias": saturation_bias, "radius": radius, "exposure": exposure,
            "aspect_ratio": aspect_ratio, "aspect_angle": aspect_angle, "operation": operation,
            "source_opacity": source_opacity, "unmult": unmult, "shift": shift}


PARAMS = ("falloff", "threshold_mode", "threshold", "threshold_smooth", "saturation_bias",
          "radius", "exposure", "aspect_ratio", "aspect_angle", "operation", "source_opacity",
          "unmult")


def render(c, frame_no):
    """The composition's frame: column x shows the layer's column x - shift."""
    s = settings_at(c, frame_no)
    out = glow_layer(pixels(c["drawing"]), s, [x - c["shift"] for x in range(W)], list(range(H)))
    return [[float(v) for v in out[y, x]] for y in range(H) for x in range(W)]


CASES = {
    "FX-SGLOW-001": ("Radius 12, everything else at the effect's defaults (threshold 0, so "
                     "everything glows; Screen; exposure 1): a soft glow with a bright core and "
                     "a long tail spreads round every patch into the empty space.",
                     case(), [0]),
    "FX-SGLOW-002": ("The effect's own defaults, radius 500: the glow is spread so wide that on "
                     "a drawing this small it is a faint wash; the drawing shows through "
                     "brighter where it doubles on itself.",
                     case(radius=500), [0]),
    "FX-SGLOW-003": ("Radius 0: nothing spreads; the light is laid on itself, so each pixel "
                     "is doubled and held at white.",
                     case(radius=0), [0]),
    "FX-SGLOW-004": ("Radius 1.5: the first level is too small to count yet, so this is still "
                     "FX-SGLOW-003.",
                     case(radius=1.5), [0]),
    "FX-SGLOW-005": ("Radius 2.25: the first level is half in, so the glow is half spread and "
                     "half laid on itself.",
                     case(radius=2.25), [0]),
    "FX-SGLOW-006": ("Radius 100: seven levels, the larger three worked on cells of 8, 4 and 2 "
                     "pixels.",
                     case(radius=100), [0]),
    "FX-SGLOW-007": ("Threshold 30, Chroma: each channel tested on its own: the yellow's red and "
                     "green glow and its blue does not, the brown's red glows, the purple does "
                     "not glow.",
                     case(threshold=30), [0]),
    "FX-SGLOW-008": ("Threshold 30, Luminance: one brightness per pixel: only the yellow glows, "
                     "all three of its channels.",
                     case(threshold=30, threshold_mode="luminance"), [0]),
    "FX-SGLOW-009": ("Threshold 30, Smooth 50: a channel between 15 % and 30 % glows in "
                     "proportion: the yellow's blue, at 19 %, glows at a quarter.",
                     case(threshold=30, threshold_smooth=50), [0]),
    "FX-SGLOW-010": ("Saturation Bias 100, threshold 50, the grey drawing: the test is on "
                     "saturation alone: the yellow and the purple glow, the grey does not.",
                     case("grey", threshold=50, saturation_bias=100), [0]),
    "FX-SGLOW-011": ("Saturation Bias -100, threshold 50: the other way round: only the grey "
                     "glows.",
                     case("grey", threshold=50, saturation_bias=-100), [0]),
    "FX-SGLOW-012": ("Saturation Bias 50, threshold 40: halfway: the colourful patches glow in "
                     "every channel, the grey, bright as it is, does not.",
                     case("grey", threshold=40, saturation_bias=50), [0]),
    "FX-SGLOW-013": ("Aspect Ratio 1.6: the glow is stretched sideways and squeezed "
                     "top to bottom.",
                     case(aspect_ratio=1.6), [0]),
    "FX-SGLOW-014": ("Aspect Ratio 0.5: taller than wide.",
                     case(aspect_ratio=0.5), [0]),
    "FX-SGLOW-015": ("Aspect Ratio 1.5 at Angle 30: the oval leans, its long side 30 degrees "
                     "below the horizontal, going right.",
                     case(aspect_ratio=1.5, aspect_angle=30), [0]),
    "FX-SGLOW-016": ("Aspect Ratio 1.5 at Angle 60: steeper, worked the other way round "
                     "(slanted line along x).",
                     case(aspect_ratio=1.5, aspect_angle=60), [0]),
    "FX-SGLOW-017": ("Aspect Ratio 2: a flat streak, no spread top to bottom except the "
                     "softening that working on cells gives.",
                     case(aspect_ratio=2), [0]),
    "FX-SGLOW-018": ("Blend Mode Add, exposure 3: the light adds up and is not cut off at white.",
                     case(operation="add", exposure=3), [0]),
    "FX-SGLOW-019": ("Blend Mode Screen, exposure 3: the same, held at white.",
                     case(exposure=3), [0]),
    "FX-SGLOW-020": ("Source Opacity 0: the glow alone, the drawing not laid back.",
                     case(source_opacity=0), [0]),
    "FX-SGLOW-021": ("Source Opacity 50: the drawing laid back at half strength.",
                     case(source_opacity=50), [0]),
    "FX-SGLOW-022": ("Unmult off: the glow sits on solid black, every pixel fully covering.",
                     case(unmult="off"), [0]),
    "FX-SGLOW-023": ("Unmult off, Source Opacity 0: the glow alone on black.",
                     case(unmult="off", source_opacity=0), [0]),
    "FX-SGLOW-024": ("The patches half covering, threshold 50: the colour is multiplied by its "
                     "covering first, so the yellow counts at half and nothing glows: the "
                     "drawing, untouched.",
                     case("faint", threshold=50), [0]),
    "FX-SGLOW-025": ("The patches half covering, threshold 0: they glow at half the strength of "
                     "FX-SGLOW-001.",
                     case("faint"), [0]),
    "FX-SGLOW-026": ("FX-SGLOW-001 moved three pixels right: the glow that spread past the "
                     "drawing's left edge shows in columns 0 to 2; the layer grew to hold it.",
                     case(shift=3), [0]),
    "FX-SGLOW-027": ("FX-SGLOW-001 moved six pixels left: the yellow patch is off the "
                     "composition, and its glow still lights the frame's left side.",
                     case(shift=-6), [0]),
    "FX-SGLOW-028": ("Radius keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is "
                     "FX-SGLOW-003, frame 4 is FX-SGLOW-006.",
                     case(radius=keyed((0, 0), (4, 100))), [0, 2, 4]),
    "FX-SGLOW-029": ("Exposure keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the "
                     "drawing, frame 2 is FX-SGLOW-001.",
                     case(exposure=keyed((0, 0), (4, 2))), [0, 2, 4]),
    "FX-SGLOW-030": ("Threshold 100: nothing in the drawing is that bright: the drawing, "
                     "untouched.",
                     case(threshold=100), [0]),
    "FX-SGLOW-031": ("Radius 2,000, the largest: eleven levels; on a drawing this small the "
                     "glow is a very faint wash.",
                     case(radius=2000), [0]),
    "FX-SGLOW-032": ("Threshold 30, Smooth 50, Saturation Bias 50, Luminance, Add, exposure "
                     "1.5, Aspect Ratio 0.7 at Angle -20, Source Opacity 80, radius 30: "
                     "everything at once.",
                     case(threshold=30, threshold_smooth=50, saturation_bias=50,
                          threshold_mode="luminance", operation="add", exposure=1.5,
                          aspect_ratio=0.7, aspect_angle=-20, source_opacity=80, radius=30),
                     [0]),
}

# Outside the contract in a file. D-46: the file is read, the effect is kept as written and left
# out of every frame, with EFFECT_PARAMETER_INVALID.
INVALID = {
    "FX-SGLOW-033": ("Falloff \"gaussian\", which is not classic or physical.",
                     case(falloff="gaussian")),
    "FX-SGLOW-034": ("Radius 2001, above 2000.", case(radius=2001)),
    "FX-SGLOW-035": ("Radius -1, below 0.", case(radius=-1)),
    "FX-SGLOW-036": ("Radius keyed to 2500 at frame 4.", case(radius=keyed((0, 0), (4, 2500)))),
    "FX-SGLOW-037": ("Saturation Bias 101, above 100.", case(saturation_bias=101)),
    "FX-SGLOW-038": ("Threshold Smooth 101, above 100.", case(threshold_smooth=101)),
    "FX-SGLOW-039": ("Threshold -1, below 0.", case(threshold=-1)),
    "FX-SGLOW-040": ("Aspect Ratio 2.1, above 2.", case(aspect_ratio=2.1)),
    "FX-SGLOW-041": ("Exposure -1, below 0.", case(exposure=-1)),
    "FX-SGLOW-042": ("Source Opacity 101, above 100.", case(source_opacity=101)),
    "FX-SGLOW-043": ("Threshold Mode \"rgb\", which is not chroma or luminance.",
                     case(threshold_mode="rgb")),
    "FX-SGLOW-044": ("Blend Mode \"multiply\", which is not add or screen.",
                     case(operation="multiply")),
    "FX-SGLOW-045": ("Unmult \"yes\", which is not on or off.", case(unmult="yes")),
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
        "instance_id": "fx-0-0", "type_id": "core.glow", "enabled": True,
        "parameters": {k: setting_json(c[k]) for k in PARAMS}}]
    return p


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def plain(name, shift=0):
    px = pixels(name)
    return [[float(v) for v in (px[y, x - shift] if 0 <= x - shift < W else np.zeros(4))]
            for y in range(H) for x in range(W)]


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, px in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(S.png(px))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        base = plain(c["drawing"], c["shift"])
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(max(abs(a - b) for a, b in zip(px[i], base[i])) > 1e-9 for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        base = plain(c["drawing"])
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": base, "4": base},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_soft_glow.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases are there to make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    drawn = plain("patches")
    at = lambda x, y: y * W + x  # noqa: E731
    close = lambda p, q, e=1e-12: all(abs(a - b) <= e for a, b in zip(p, q))  # noqa: E731
    same = lambda f, g, e=1e-12: all(close(p, q, e) for p, q in zip(f, g))  # noqa: E731
    one = c["FX-SGLOW-001"]["0"]

    # Levels follow Radius: 11 at 2,000, as PLUGINS.md 2.3 asks.
    assert len(levels(2000)[0]) == 11 and len(levels(12)[0]) == 3
    # Weights always sum to one.
    for r in (0, 1.5, 2.25, 3, 5.9, 6, 6.1, 12, 30, 500, 2000):
        lv, own = levels(r)
        assert abs(sum(w for _, w in lv) + own - 1) < 1e-12, r
    # No jump when a level comes in: radius just under and just over 6, where the third level
    # starts. What moves at all is document 21's cut at ceil(3 sigma) on the other levels
    # (sigma 1 and 2 sit exactly on it), well under a tenth of one 8-bit step.
    a = render(case(radius=6 - 1e-6), 0)
    b = render(case(radius=6 + 1e-6), 0)
    assert same(a, b, 4e-4)
    # The cells' change at sigma 8 (radius 24) is a small step, recorded, not hidden.
    a = render(case(radius=24 - 1e-6), 0)
    b = render(case(radius=24 + 1e-6), 0)
    step = max(abs(p - q) for u, v in zip(a, b) for p, q in zip(u, v))
    print(f"largest step where the first level moves to cells of 2 (radius 24): {step:.5f}")
    assert step < 5e-4
    assert [down_factor(s) for s, _ in levels(100)[0]] == [8, 4, 2, 1, 1, 1, 1]

    # 001: glow everywhere near the patches, the far corner reached too (radius 12).
    for x in (0, 5, 6, 9, 10, 14, 15):
        assert one[at(x, 4)][3] > 0, x
    assert one[at(15, 0)][3] > 0
    for p in one:
        assert all(0 <= v <= 1 for v in p)
    assert same(c["FX-SGLOW-003"]["0"], c["FX-SGLOW-004"]["0"])
    three = c["FX-SGLOW-003"]["0"]
    for i in range(W * H):
        assert close(three[i], [min(1, 2 * v) for v in drawn[i][:3]] + [drawn[i][3]]), i
    five = c["FX-SGLOW-005"]["0"]
    assert not same(five, three) and not same(five, render(case(radius=3), 0))
    # 007: chroma; the yellow's blue does not glow, so at the yellow's centre blue is unchanged
    # apart from light from elsewhere: there is none in blue (no blue passes 30 %).
    seven = c["FX-SGLOW-007"]["0"]
    assert all(seven[i][2] == drawn[i][2] for i in range(W * H))
    assert seven[at(0, 4)][0] > 0 and seven[at(0, 4)][1] > 0
    # 008: luminance; the yellow glows in blue too.
    eight = c["FX-SGLOW-008"]["0"]
    assert eight[at(0, 4)][2] > 0 and eight[at(15, 4)][0] < seven[at(15, 4)][0]
    # 009: smooth brings the yellow's blue in.
    assert c["FX-SGLOW-009"]["0"][at(0, 4)][2] > 0
    # 010-012: bias.
    g = plain("grey")
    ten, eleven, twelve = (c[f"FX-SGLOW-0{n}"]["0"] for n in (10, 11, 12))
    t0 = render(case("grey", threshold=100), 0)
    assert same(t0, g)
    assert ten[at(0, 4)][0] > 0 and ten[at(15, 4)][2] > 0
    assert eleven[at(0, 4)][0] < ten[at(0, 4)][0]
    # the grey glows evenly in 011: at its own pixel all three channels rise alike.
    rise = [eleven[at(7, 4)][ch] - g[at(7, 4)][ch] for ch in range(3)]
    assert rise[0] > 0 and max(rise) - min(rise) < 1e-9
    assert twelve[at(15, 4)][2] > 0
    # 013/014: wider and taller.
    w13, w14 = c["FX-SGLOW-013"]["0"], c["FX-SGLOW-014"]["0"]
    assert w13[at(0, 4)][0] > one[at(0, 4)][0] and w13[at(3, 0)][0] < one[at(3, 0)][0]
    assert w14[at(3, 0)][0] > one[at(3, 0)][0]
    # 015 and 016 lean: not mirror images of each other top to bottom.
    assert not same(c["FX-SGLOW-015"]["0"], c["FX-SGLOW-016"]["0"])
    assert not same(c["FX-SGLOW-015"]["0"], w13)
    # 018: add passes white, 019 is held.
    assert any(v > 1 for p in c["FX-SGLOW-018"]["0"] for v in p[:3])
    assert all(v <= 1 for p in c["FX-SGLOW-019"]["0"] for v in p)
    # 020: glow alone is darker than 001 on the patches; 021 between.
    assert c["FX-SGLOW-020"]["0"][at(3, 4)][0] < c["FX-SGLOW-021"]["0"][at(3, 4)][0]
    # 022/023: solid.
    assert all(p[3] == 1 for p in c["FX-SGLOW-022"]["0"])
    assert all(p[3] == 1 for p in c["FX-SGLOW-023"]["0"])
    # 024: untouched; 025: half.
    assert same(c["FX-SGLOW-024"]["0"], plain("faint"))
    half = c["FX-SGLOW-025"]["0"]
    assert half[at(0, 4)][0] < one[at(0, 4)][0]
    # 026: shifted, the glow left of the drawing shows.
    moved = c["FX-SGLOW-026"]["0"]
    assert all(moved[at(x, 4)][3] > 0 for x in (0, 1, 2))
    for y in range(H):
        for x in range(3, W):
            assert close(moved[at(x, y)], one[at(x - 3, y)], 1e-12)
    # 027: the off-screen yellow still lights the frame: brighter than with no yellow at all.
    left = c["FX-SGLOW-027"]["0"]
    assert left[at(0, 4)][0] > 0
    assert same(c["FX-SGLOW-028"]["0"], three) and same(c["FX-SGLOW-028"]["4"], c["FX-SGLOW-006"]["0"])
    assert same(c["FX-SGLOW-029"]["0"], drawn) and same(c["FX-SGLOW-029"]["2"], one)
    assert same(c["FX-SGLOW-030"]["0"], drawn)
    assert not same(c["FX-SGLOW-031"]["0"], drawn)

    for name, frames in c.items():
        for px in frames.values():
            for p in px:
                assert 0 <= p[3] <= 1 and all(v >= -1e-12 for v in p[:3]), (name, p)


if __name__ == "__main__":
    main()
