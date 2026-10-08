"""Whole-picture statistics and the effects that read them, worked a second way.

D-351 (EFFECTS.md P0-15) adds picture statistics: a count of a layer's whole picture, at the
effect's place in its stack, that an effect reads before it draws. Four effects read them, this
program's own, modelled on After Effects' Auto Levels, Auto Contrast, Auto Color and Equalize:
Stretch Levels (`core.stretch_levels`), Stretch Contrast (`core.stretch_contrast`), Stretch
Color (`core.stretch_color`) and Spread Tones (`core.spread_tones`). Nothing is ported; Adobe
does not publish its methods, so the rules below are ours. Document 21 is the rule in words;
this file is the reference for the numbers document 25 pins against it.

**The statistics.** Over every pixel of the layer's picture as the effects before this one left
it, the whole picture, whatever part of it the composition shows, and whose covering a is above
0: its straight colour through the sRGB curve, e = (eR, eG, eB), each 0 to 1, and its brightness
y = 0.2126 eR + 0.7152 eG + 0.0722 eB. A value v falls in bin floor(255 v + 0.5), 0 to 255.
Four histograms, red, green, blue and brightness, each pixel adding its covering a to its bin;
and for each brightness bin the colour it holds, a e summed. The average colour is the sum of
a e over the sum of a. A picture where nothing shows has no statistics, and the effect changes
nothing.

**Clip points** of a histogram h whose total is T, for clips b and w per cent: the black bin kb
is the lowest whose running total from the bottom is above T b / 100, and the white bin kw the
highest whose running total from the top is above T w / 100. With both 0 they are the lowest
and highest bins anything is in: the picture's own minimum and maximum, to a 255th.

**Stretch Levels**: each channel by its own histogram, e to (e - kb/255) / ((kw - kb)/255),
held inside 0 to 1. A channel whose kw is not above kb is left as it is.
**Stretch Contrast**: the same with one histogram, red, green and blue added, for all three, so
the colours keep their balance.
**Stretch Color**: kb and kw of the brightness histogram; the dark colour D is the colour of
bins 0 to kb over their covering, the light colour L that of bins kw to 255. Each channel goes
e to (e - D) / (L - D), held inside 0 to 1, when L - D is above a millionth; otherwise it is
left. So the darkest pixels become black and the lightest white, and a cast in either is taken
out. With `snap_neutral_midtones` on, the average colour taken through the same stretch, m,
held inside 0 to 1, and t the mean of its three channels: each channel is then raised to
g = ln t / ln m (held inside 0.1 to 10; 1 unless m and t are both between 0 and 1), which takes
the average colour to a grey.
**Spread Tones** (after Equalize): the map of a histogram h, total T, lowest bin c0 in weight,
is M(k) = (running total to k - c0) / (T - c0), held inside 0 to 1; a histogram all in one bin
has none, and nothing changes. `rgb`: each channel e to M_c(bin(e)) by its own histogram;
`photoshop`: by the one histogram of the three added; `brightness`: each channel multiplied by
M_y(bin(y)) / y, a black pixel left. Then e + (amount / 100) (that - e).

Every result, held inside 0 to 1, comes back to linear at the pixel's own covering; a pixel that
does not show is left as it is (document 21's shared colour rule).

**Temporal smoothing** (the three Stretch effects), 0 to 10 seconds: N = floor(seconds x frame
rate) frames each side of frame n. Each frame's statistics are those of the layer's picture at
that frame with the effects before this one; a frame outside the layer, or where nothing shows,
is passed over. With `scene_detect` on, each side is walked outward from n and stops at the
first frame whose brightness differs from the last one counted by more than half: the
histograms in 16 groups of 16 bins, each over its total, and half the sum of the differences.
The frames counted are added, each over its own total, and the clip points, colours and average
read from the sum as from one picture. On an adjustment layer the frames beneath at other times
are not to hand: smoothing is not drawn, each frame is stretched by itself, with
TEMPORAL_SMOOTHING_SKIPPED each frame. `black_clip` and `white_clip`, 0 to 10 per cent, 0.1
when added; `temporal_smoothing` 0; `scene_detect` and `snap_neutral_midtones` "off" or "on",
"off". Spread Tones: `equalize` "rgb", "brightness" or "photoshop", "rgb"; `amount` 0 to 100,
100. After Effects' Blend With Original is the Mix every effect has (D-202): 100 less it.

**This file never runs the build's code path.** It counts with numpy's bincount and finds the
clip points with cumulative sums and argmax, on the drawings' exact 8-bit values in double
precision, where the build counts row by row in its own loop from its single-precision buffers.
It asserts that no value lies within a thousandth of a bin of a bin's edge, no running total
within a millionth of its clip, and no scene difference within a thousandth of a half, so the
build's rounding can't land anything on the other side.

Every case is a project of one composition 16 by 10, eight frames, in `Fixtures/auto_tone/`: the
layer `holder`, a run of drawings flickering in brightness, with a cut to a darker scene at frame
4. The expected pixels are in `Fixtures/auto_tone/expected_auto_tone.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/auto_tone_reference.py
"""

import json
import sys
from math import floor, log
from pathlib import Path

import numpy as np

sys.path.insert(0, str(Path(__file__).resolve().parent))
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
from motion_blur_reference import png  # noqa: E402
import effect_layer_reference as L  # noqa: E402
from smooth_reference import linear_to_srgb  # noqa: E402

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "auto_tone"
TOLERANCE = 2e-5  # document 25's default for a filter
W, H, FRAMES, FPS = 16, 10, 8, 24
LUMA = np.array([0.2126, 0.7152, 0.0722])


def srgb_to_linear(c):
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4


# --- the drawings ---------------------------------------------------------------------------

def scene_a(x, y, lift):
    """A dull, warm picture: a grey ramp 70 to 188 with red raised and blue lowered, an orange
    and a blue patch in the bottom rows, column 0 empty and column 15 at half covering."""
    if x == 0:
        return (0, 0, 0, 0)
    g = 70 + 7 * (x - 1) + 3 * y
    rgb = (g + 20, g, g - 25)
    if y >= 8 and 2 <= x <= 5:
        rgb = (180, 90, 60)
    if y >= 8 and 9 <= x <= 12:
        rgb = (60, 110, 150)
    rgb = tuple(max(0, min(255, v + lift)) for v in rgb)
    return rgb + ((128,) if x == 15 else (255,))


def scene_b(x, y, lift):
    """A darker, cold picture: a ramp 20 to 131 with blue raised, column 0 empty."""
    if x == 0:
        return (0, 0, 0, 0)
    g = 20 + 5 * x + 4 * y
    return tuple(max(0, min(255, v + lift)) for v in (g - 10, g, g + 35)) + (255,)


DRAWINGS = {
    "tone_1": [[scene_a(x, y, 0) for x in range(W)] for y in range(H)],
    "tone_2": [[scene_a(x, y, 12) for x in range(W)] for y in range(H)],
    "tone_3": [[scene_a(x, y, -10) for x in range(W)] for y in range(H)],
    "tone_4": [[scene_b(x, y, 0) for x in range(W)] for y in range(H)],
    "tone_5": [[scene_b(x, y, 8) for x in range(W)] for y in range(H)],
}
RUN = [1, 2, 3, 1, 4, 5, 4, 5]  # the drawing at each frame: a cut at frame 4
SPANS = [{"start_frame": f, "end_frame_exclusive": f + 1, "drawing_number": d}
         for f, d in enumerate(RUN)]


# --- the statistics -------------------------------------------------------------------------

def edge_margin(v):
    """How far 255 v is from a bin's edge, in bins."""
    t = np.asarray(v) * 255
    return float(np.min(np.abs(t - np.floor(t) - 0.5))) if t.size else 1.0


def bins(v):
    assert edge_margin(v) > 1e-3, "a value lies on a bin's edge"
    return np.clip(np.floor(np.asarray(v) * 255 + 0.5).astype(int), 0, 255)


def stats(picture):
    """`picture` is (e, a): straight encoded colours, n by 3, and coverings, all n showing."""
    e, a = picture
    if len(a) == 0:
        return None
    y = e @ LUMA
    by = bins(y)
    hist = [np.bincount(bins(e[:, c]), weights=a, minlength=256) for c in range(3)]
    hist.append(np.bincount(by, weights=a, minlength=256))
    sums = [np.bincount(by, weights=a * e[:, c], minlength=256) for c in range(3)]
    return {"hist": np.array(hist), "sums": np.array(sums)}


def added(many):
    """Frames' statistics, each over its own total, added."""
    many = [s for s in many if s is not None]
    if not many:
        return None
    return {k: sum(s[k] / s["hist"][3].sum() for s in many) for k in ("hist", "sums")}


def difference(s, t):
    g = lambda u: u["hist"][3].reshape(16, 16).sum(axis=1) / u["hist"][3].sum()  # noqa: E731
    d = 0.5 * float(np.abs(g(s) - g(t)).sum())
    assert abs(d - 0.5) > 1e-3, "a scene difference lies on the half"
    return d


def clip_points(h, black, white):
    T = h.sum()
    lo, hi = T * black / 100, T * white / 100
    up, down = np.cumsum(h), np.cumsum(h[::-1])[::-1]
    for run, at in ((up, lo), (down, hi)):
        near = np.abs(run - at) <= 1e-6 * T
        assert not np.any(near & ~((run == 0) & (at == 0))), "a running total lies on its clip"
    kb = int(np.argmax(up > lo))
    kw = 255 - int(np.argmax(down[::-1] > hi))
    return kb, kw


def spread_map(h):
    T = h.sum()
    nz = np.nonzero(h)[0]
    c0 = h[nz[0]]
    if T - c0 <= 0:
        return None
    return np.clip((np.cumsum(h) - c0) / (T - c0), 0, 1)


# --- the effects ----------------------------------------------------------------------------

def stretch_fn(kind, s, black, white, snap):
    """The function e -> e' (three channels at once) the statistics s give."""
    if s is None:
        return lambda e: e
    hist, sums = s["hist"], s["sums"]
    if kind in ("levels", "contrast"):
        maps = []
        pooled = hist[0] + hist[1] + hist[2]
        for c in range(3):
            kb, kw = clip_points(hist[c] if kind == "levels" else pooled, black, white)
            maps.append(None if kw <= kb else (kb / 255, kw / 255))
        return lambda e: np.array([e[c] if m is None else (e[c] - m[0]) / (m[1] - m[0])
                                   for c, m in enumerate(maps)])
    kb, kw = clip_points(hist[3], black, white)
    if kw <= kb:
        return lambda e: e
    dark = sums[:, :kb + 1].sum(axis=1) / hist[3][:kb + 1].sum()
    light = sums[:, kw:].sum(axis=1) / hist[3][kw:].sum()
    on = light - dark > 1e-6
    assert np.all(np.abs(light - dark - 1e-6) > 1e-9)

    def stretch(e):
        return np.array([min(1.0, max(0.0, (e[c] - dark[c]) / (light[c] - dark[c]))) if on[c]
                         else e[c] for c in range(3)])

    if not snap:
        return stretch
    mean = sums.sum(axis=1) / hist[3].sum()
    m = stretch(mean)
    t = m.mean()
    g = [min(10.0, max(0.1, log(t) / log(m[c]))) if 0 < m[c] < 1 and 0 < t < 1 else 1.0
         for c in range(3)]
    return lambda e: np.array([v ** g[c] for c, v in enumerate(np.clip(stretch(e), 0, 1))])


def spread_fn(s, mode, amount):
    if s is None:
        return lambda e: e
    hist = s["hist"]
    A = amount / 100
    if mode == "brightness":
        m = spread_map(hist[3])
        if m is None:
            return lambda e: e

        def f(e):
            y = float(e @ LUMA)
            return e if y <= 0 else e + A * (e * (m[bins(y)] / y) - e)
        return f
    maps = [spread_map(hist[c]) for c in range(3)] if mode == "rgb" else \
        [spread_map(hist[0] + hist[1] + hist[2])] * 3
    return lambda e: np.array([e[c] if maps[c] is None else e[c] + A * (maps[c][bins(e[c])] - e[c])
                               for c in range(3)])


def levels_fn(output_white):
    """The Levels before the effect in some cases: output white lowered, the rest default."""
    return lambda e: np.clip(e * output_white / 255, 0, 1)


# --- the layer's picture --------------------------------------------------------------------

def drawing(frame):
    """The holder's drawing at a frame, as straight encoded colours and coverings, every pixel,
    or None outside the layer."""
    if not 0 <= frame < FRAMES:
        return None
    rows = DRAWINGS[f"tone_{RUN[frame]}"]
    px = [p for row in rows for p in row]
    return (np.array([[v / 255 for v in p[:3]] for p in px]), np.array([p[3] / 255 for p in px]))


def showing(pic):
    e, a = pic
    keep = a > 0
    return e[keep], a[keep]


def before(c, pic):
    """The effects before this one, on each pixel that shows."""
    if c["before"] is None:
        return pic
    e, a = pic
    f = levels_fn(c["before"])
    return np.array([f(v) if w > 0 else v for v, w in zip(e, a)]), a


def frame_stats(c, frame):
    pic = drawing(frame)
    return None if pic is None else stats(showing(before(c, pic)))


def stats_at(c, n):
    """The statistics the effect reads at frame n."""
    seconds = value_at(c["temporal_smoothing"], n)
    own = frame_stats(c, n)
    if c["on"] == "adjust" or seconds == 0:
        return own
    reach = floor(seconds * FPS + 1e-9)
    counted = [own]
    for step in (1, -1):
        last = own
        for k in range(1, reach + 1):
            s = frame_stats(c, n + step * k)
            if s is None:
                continue
            if c["scene_detect"] == "on" and last is not None and difference(s, last) > 0.5:
                break
            counted.append(s)
            last = s
    return added(counted)


def render(c, n):
    """The composition's pixels at frame n."""
    pic = before(c, drawing(n))
    s = stats_at(c, n)
    if c["kind"] == "spread":
        f = spread_fn(s, c["equalize"], value_at(c["amount"], n))
    else:
        f = stretch_fn(c["kind"], s, value_at(c["black_clip"], n), value_at(c["white_clip"], n),
                       c["snap_neutral_midtones"] == "on")
    mix = c["mix"] / 100
    out = []
    e, a = pic
    for v, w in zip(e, a):
        if w <= 0:
            out.append([0.0] * 4)
            continue
        given = [srgb_to_linear(x) * w for x in v] + [w]
        got = [srgb_to_linear(min(1.0, max(0.0, x))) * w for x in f(v)] + [w]
        out.append([p + mix * (q - p) for p, q in zip(given, got)])
    if c["after"] is not None:
        out = [[srgb_to_linear(min(1.0, max(0.0, linear_to_srgb(p[i] / p[3]) * c["after"] / 255)))
                * p[3] for i in range(3)] + [p[3]] if p[3] > 0 else p for p in out]
    dx = c["shift"]
    return [out[y * W + x - dx] if 0 <= x - dx < W else [0.0] * 4
            for y in range(H) for x in range(W)]


def plain(c, n):
    e, a = drawing(n)
    px = [[srgb_to_linear(x) * w for x in v] + [w] if w > 0 else [0.0] * 4 for v, w in zip(e, a)]
    dx = c["shift"]
    return [px[y * W + x - dx] if 0 <= x - dx < W else [0.0] * 4
            for y in range(H) for x in range(W)]


# --- the cases ------------------------------------------------------------------------------

def case(kind="levels", temporal_smoothing=0, scene_detect="off", black_clip=0.1, white_clip=0.1,
         snap_neutral_midtones="off", equalize="rgb", amount=100, mix=100, before=None,
         after=None, shift=0, on="holder"):
    return {"kind": kind, "temporal_smoothing": temporal_smoothing, "scene_detect": scene_detect,
            "black_clip": black_clip, "white_clip": white_clip,
            "snap_neutral_midtones": snap_neutral_midtones, "equalize": equalize,
            "amount": amount, "mix": mix, "before": before, "after": after, "shift": shift,
            "on": on}


SMOOTH = 0.1  # 2.4 frames at 24 a second: two each side

CASES = {
    "FX-AUTO-001": ("Stretch Levels as added, clips 0.1 per cent: each channel of the dull, warm "
                    "drawing stretched to its own darkest and lightest, so the darkest grey "
                    "nears black, the lightest white, and the warm cast is lessened.",
                    case(), (0,)),
    "FX-AUTO-002": ("Stretch Levels, clips 5 per cent: the darkest and lightest 5 per cent of each "
                    "channel go to black and white.", case(black_clip=5, white_clip=5), (0,)),
    "FX-AUTO-003": ("Stretch Levels, clips 0: the drawing's own darkest and lightest in each "
                    "channel become exactly 0 and 255.", case(black_clip=0, white_clip=0), (0,)),
    "FX-AUTO-004": ("Stretch Contrast as added: one stretch for the three channels together, so "
                    "the warm cast stays.", case(kind="contrast"), (0,)),
    "FX-AUTO-005": ("Stretch Contrast, clips 5 per cent.",
                    case(kind="contrast", black_clip=5, white_clip=5), (0,)),
    "FX-AUTO-006": ("Stretch Color as added: the darkest pixels' colour goes to black and the "
                    "lightest pixels' to white.", case(kind="color"), (0,)),
    "FX-AUTO-007": ("Stretch Color with Snap Neutral Midtones: the average colour is taken to a "
                    "grey as well.", case(kind="color", snap_neutral_midtones="on"), (0,)),
    "FX-AUTO-008": ("Stretch Color, clips 5 per cent, Snap Neutral Midtones.",
                    case(kind="color", black_clip=5, white_clip=5, snap_neutral_midtones="on"),
                    (0,)),
    "FX-AUTO-009": ("Stretch Levels at Mix 50, Blend With Original 50 per cent: halfway between "
                    "the drawing and FX-AUTO-001.", case(mix=50), (0,)),
    "FX-AUTO-010": ("A Levels lowering output white to 128, then Stretch Levels: the statistics "
                    "are of the darkened picture, so the full range comes back.",
                    case(before=128), (0,)),
    "FX-AUTO-011": ("Stretch Levels, then a Levels lowering output white to 128: the stretched "
                    "picture darkened.", case(after=128), (0,)),
    "FX-AUTO-012": ("Stretch Levels on each frame of the flicker by itself: frames 1 and 2, a "
                    "brighter and a darker copy of frame 0, are each stretched by their own "
                    "statistics.", case(), (1, 2, 4)),
    "FX-AUTO-013": ("Stretch Levels, Temporal Smoothing 0.1 seconds, two frames each side: frame "
                    "0 reads frames 0 to 2 (the two before are outside the layer); frame 3 reads "
                    "1 to 5, across the cut; frame 7 reads 5 to 7.",
                    case(temporal_smoothing=SMOOTH), (0, 3, 7)),
    "FX-AUTO-014": ("FX-AUTO-013 with Scene Detect: frame 3 reads only 1 to 3 and frame 4 only 4 "
                    "to 6, each side of the cut.",
                    case(temporal_smoothing=SMOOTH, scene_detect="on"), (3, 4)),
    "FX-AUTO-015": ("Stretch Color, Temporal Smoothing 0.1, Scene Detect and Snap Neutral "
                    "Midtones.", case(kind="color", temporal_smoothing=SMOOTH, scene_detect="on",
                                      snap_neutral_midtones="on"), (2, 5)),
    "FX-AUTO-016": ("Stretch Contrast, Temporal Smoothing 0.1.",
                    case(kind="contrast", temporal_smoothing=SMOOTH), (3,)),
    "FX-AUTO-017": ("A Levels lowering output white to 128, then Stretch Levels with Temporal "
                    "Smoothing 0.1: every frame read is the darkened picture.",
                    case(before=128, temporal_smoothing=SMOOTH), (2,)),
    "FX-AUTO-018": ("Spread Tones as added, RGB: each channel's values spread evenly over 0 to "
                    "255 by its own histogram.", case(kind="spread"), (0,)),
    "FX-AUTO-019": ("Spread Tones, Photoshop Style: one histogram of the three channels for all.",
                    case(kind="spread", equalize="photoshop"), (0,)),
    "FX-AUTO-020": ("Spread Tones, Brightness: each pixel scaled so its brightness is spread.",
                    case(kind="spread", equalize="brightness"), (0,)),
    "FX-AUTO-021": ("Spread Tones, RGB, amount 50: halfway.",
                    case(kind="spread", amount=50), (0,)),
    "FX-AUTO-022": ("Stretch Levels on an adjustment layer above the holder: the frame beneath is "
                    "the drawing, so FX-AUTO-001.", case(on="adjust"), (0,)),
    "FX-AUTO-023": ("Stretch Levels with Temporal Smoothing 0.1 on an adjustment layer: the frames "
                    "beneath at other times are not to hand, so each frame is stretched by "
                    "itself, with a warning each frame.",
                    case(on="adjust", temporal_smoothing=SMOOTH), (0, 3)),
    "FX-AUTO-024": ("Black clip keyed from 0 at frame 0 to 10 at frame 4: 5 at frame 2.",
                    case(black_clip=keyed((0, 0), (4, 10))), (0, 2, 4)),
    "FX-AUTO-025": ("FX-AUTO-001 with the holder moved 3 pixels right, its last columns off the "
                    "composition: the statistics are of the whole layer, so the same, moved.",
                    case(shift=3), (0,)),
}

INVALID = {
    "FX-AUTO-026": ("Black clip 11, above 10.", case(black_clip=11)),
    "FX-AUTO-027": ("White clip -1, below 0.", case(white_clip=-1)),
    "FX-AUTO-028": ("Temporal Smoothing 11 seconds, above 10.", case(temporal_smoothing=11)),
    "FX-AUTO-029": ("Scene Detect written \"yes\".", case(scene_detect="yes")),
    "FX-AUTO-030": ("Snap Neutral Midtones written \"maybe\".",
                    case(kind="color", snap_neutral_midtones="maybe")),
    "FX-AUTO-031": ("Spread Tones' equalize written \"hsl\".", case(kind="spread", equalize="hsl")),
    "FX-AUTO-032": ("Spread Tones' amount 101, above 100.", case(kind="spread", amount=101)),
}

TYPE = {"levels": "core.stretch_levels", "contrast": "core.stretch_contrast",
        "color": "core.stretch_color", "spread": "core.spread_tones"}


def effect(fid, c):
    if c["kind"] == "spread":
        keys = ("equalize", "amount")
    else:
        keys = ("temporal_smoothing", "scene_detect", "black_clip", "white_clip") + \
            (("snap_neutral_midtones",) if c["kind"] == "color" else ())
    record = {"instance_id": fid, "type_id": TYPE[c["kind"]], "enabled": True,
              "parameters": {k: setting_json(c[k]) for k in keys}}
    if c["mix"] != 100:
        record["mix"] = c["mix"]
    return record


def levels(fid, output_white):
    return {"instance_id": fid, "type_id": "core.levels", "enabled": True,
            "parameters": {"input_black": 0, "input_white": 255, "gamma": 1, "output_black": 0,
                           "output_white": output_white}}


def project_json(fx, c):
    holder = L.raster("holder", "asset-tone", spans=SPANS, position=(c["shift"], 0),
                      out_frame=FRAMES)
    layers = [holder]
    if c["on"] == "adjust":
        adjust = L.adjustment("adjust", out_frame=FRAMES)
        adjust["effects"] = [effect("fx-1", c)]
        layers.append(adjust)
    else:
        holder["effects"] = ([levels("fx-0", c["before"])] if c["before"] is not None else []) \
            + [effect("fx-1", c)] \
            + ([levels("fx-2", c["after"])] if c["after"] is not None else [])
    return {"schema_version": 0, "project_id": "proj-" + fx.lower(),
            "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
            "assets": [{"id": "asset-tone", "kind": "image_sequence", "name": "tone",
                        "pattern": "tone_####.png",
                        "frames": {str(k): f"media/tone_{k}.png" for k in range(1, 6)},
                        "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
            "compositions": [L.composition("comp-main", W, H, FRAMES, layers)]}


def write(fx, c):
    name = f"{fx.lower().replace('-', '_')}.json"
    (OUT / name).write_text(json.dumps(project_json(fx, c), indent=2) + "\n", encoding="utf-8")
    return name


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (OUT / "media" / f"{name}.png").write_bytes(png(pixels))

    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): [[float(v) for v in p] for p in render(c, f)] for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        if c["on"] == "adjust" and c["temporal_smoothing"] != 0:
            expected["cases"][fx]["frame_warning"] = "TEMPORAL_SMOOTHING_SKIPPED"
        print(f"{fx}: " + ", ".join(
            f"frame {f} {sum(px[i] != plain(c, int(f))[i] for i in range(W * H))} changed"
            for f, px in rendered.items()))
    for fx, (says, c) in INVALID.items():
        says += (" The file is read, the effect is kept as written and left out of every frame, "
                 "with a warning.")
        expected["cases"][fx] = {"says": says, "project": write(fx, c),
                                 "frames": {"0": plain(c, 0), "4": plain(c, 4)},
                                 "warning": "EFFECT_PARAMETER_INVALID"}
        print(f"{fx}: invalid")

    (OUT / "expected_auto_tone.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                 encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    near = lambda a, b, tol=1e-12: all(abs(p - q) < tol for u, v in zip(a, b)  # noqa: E731
                                       for p, q in zip(u, v))
    enc = lambda px: [[linear_to_srgb(p[i] / p[3]) * 255 for i in range(3)]  # noqa: E731
                      for p in px if p[3] > 0]

    # Clips 0: every channel's darkest is 0 and lightest 255.
    e3 = np.array(enc(c["FX-AUTO-003"]["0"]))
    assert np.allclose(e3.min(axis=0), 0, atol=1e-9) and np.allclose(e3.max(axis=0), 255,
                                                                      atol=1e-9)
    # Stretch Contrast keeps the balance: blue stays the lowest channel in the ramp.
    e4 = np.array(enc(c["FX-AUTO-004"]["0"]))
    assert e4.min() < 1e-9 and abs(e4.max() - 255) < 1e-9
    assert e4[:, 0].min() > 1 and e4[:, 2].max() < 254
    # Stretch Levels takes the cast out more than Stretch Contrast: the ramp's middle is greyer.
    mid = lambda f: enc([f[5 * W + 7]])[0]  # noqa: E731
    spread_of = lambda v: max(v) - min(v)  # noqa: E731
    assert spread_of(mid(c["FX-AUTO-001"]["0"])) < spread_of(mid(c["FX-AUTO-004"]["0"]))
    # Snap: the average colour of the result is nearer grey than without.
    avg = lambda f: np.mean(enc(f), axis=0)  # noqa: E731
    assert spread_of(avg(c["FX-AUTO-007"]["0"])) < spread_of(avg(c["FX-AUTO-006"]["0"]))
    # Mix 50 halfway.
    p0 = plain(case(), 0)
    assert near(c["FX-AUTO-009"]["0"], [[(a + b) / 2 for a, b in zip(p, q)]
                                        for p, q in zip(p0, c["FX-AUTO-001"]["0"])], 1e-15)
    # Order matters, and stretching after a darkening brings white back.
    e10 = np.array(enc(c["FX-AUTO-010"]["0"]))
    assert e10.max() > 254 and c["FX-AUTO-010"]["0"] != c["FX-AUTO-011"]["0"]
    assert np.array(enc(c["FX-AUTO-011"]["0"])).max() < 129
    # Each frame by itself: frame 0 of 012 is 001.
    assert c["FX-AUTO-012"]["1"] != c["FX-AUTO-001"]["0"]
    # Smoothing differs from each frame alone, and scene detect differs from smoothing alone.
    s13, s14 = case(temporal_smoothing=SMOOTH), case(temporal_smoothing=SMOOTH, scene_detect="on")
    assert c["FX-AUTO-013"]["3"] != render(case(), 3)
    assert c["FX-AUTO-014"]["3"] != c["FX-AUTO-013"]["3"]
    # The frames each counted, as the cases say.
    def same_stats(a, b):
        return np.allclose(a["hist"], b["hist"]) and np.allclose(a["sums"], b["sums"])
    fs = lambda n: frame_stats(case(), n)  # noqa: E731
    assert same_stats(stats_at(s13, 0), added([fs(0), fs(1), fs(2)]))
    assert same_stats(stats_at(s13, 3), added([fs(n) for n in range(1, 6)]))
    assert same_stats(stats_at(s13, 7), added([fs(5), fs(6), fs(7)]))
    assert same_stats(stats_at(s14, 3), added([fs(1), fs(2), fs(3)]))
    assert same_stats(stats_at(s14, 4), added([fs(4), fs(5), fs(6)]))
    assert difference(fs(4), fs(3)) > 0.5 and difference(fs(2), fs(3)) < 0.5
    # Spread: amount 50 halfway in display values, and the three modes differ.
    assert len({json.dumps(c[f"FX-AUTO-0{n}"]["0"]) for n in (18, 19, 20)}) == 3
    e18, e21, ep = (np.array(enc(f)) for f in (c["FX-AUTO-018"]["0"], c["FX-AUTO-021"]["0"],
                                               plain(case(), 0)))
    assert np.allclose(e21, (e18 + ep) / 2, atol=1e-9)
    assert e18.min() < 1e-9 and abs(e18.max() - 255) < 1e-9
    # The adjustment layer: the same as on the holder; with smoothing, each frame by itself.
    assert near(c["FX-AUTO-022"]["0"], c["FX-AUTO-001"]["0"], 1e-15)
    assert near(c["FX-AUTO-023"]["0"], c["FX-AUTO-001"]["0"], 1e-15)
    assert near(c["FX-AUTO-023"]["3"], render(case(), 3), 1e-15)
    # Keyed clip: frame 0 is clips 0 and 0.1.
    assert near(c["FX-AUTO-024"]["0"], render(case(black_clip=0), 0), 1e-15)
    assert near(c["FX-AUTO-024"]["2"], render(case(black_clip=5), 2), 1e-15)
    # Moved: the whole layer's statistics, so FX-AUTO-001 moved.
    moved = c["FX-AUTO-025"]["0"]
    for y in range(H):
        assert moved[y * W + 3:(y + 1) * W] == c["FX-AUTO-001"]["0"][y * W:(y + 1) * W - 3]
    # Every pixel a picture can hold.
    for fx, v in c.items():
        for px in v.values():
            for p in px:
                assert all(-1e-12 <= a <= 1 + 1e-12 for a in p) and max(p[:3]) <= p[3] + 1e-12, fx
    print("checks passed")


if __name__ == "__main__":
    main()
