"""Audio Spectrum, worked a second way.

D-420 adds `core.audio_spectrum`, After Effects' Audio Spectrum (Generate): it "display[s] the
audio spectrum of a layer that contains audio", "the magnitude of audio levels at frequencies in
the range that you define using Start Frequency and End Frequency" (Adobe's After Effects user
manual, pages 509 to 510 of the copy on manualsdir.com). Its settings are After Effects': Audio
Layer, Start Point, End Point, Path, Use Polar Path, Start Frequency, End Frequency, Frequency
Bands, Maximum Height, Audio Duration, Audio Offset, Thickness, Softness, Inside Color, Outside
Color, Blend Overlapping Colors, Hue Interpolation, Dynamic Hue Phase, Color Symmetry, Display
Options, Side Options, Duration Averaging and Composite On Original.

Adobe does not publish how it measures the frequencies, nor how tall a level is drawn. The rule
below is this program's own, nothing is ported: a Hann window over the samples Audio Duration
long, one Fourier sum (no transform size, so no rounding to a power of two) at each band's
middle frequency, the bands spread evenly in hertz from Start Frequency to End Frequency.

The sound. The Audio Layer is a sound layer of the composition, by its id ("" none: silence).
At composition frame f the layer's own frame is n = f - in + source offset (silence when f is
outside the layer); its first sample is s = floor(n rate den / num), ADR-018's, and the window
starts at s0 = s + floor(offset rate / 1000 + 1/2), offset the Audio Offset in milliseconds. The
window is N = max(1, floor(duration rate / 1000 + 1/2)) samples, duration the Audio Duration in
milliseconds. A sample is the mean of the file's channels times the layer's gain, 10^(gain/20),
and 0 before the file's first sample or past its last. 8-bit samples are (v - 128) / 128,
16-bit v / 32768, 24-bit v / 8388608, 32-bit v / 2147483648, floating point as written.

The levels. B = floor(Frequency Bands) bands; band b's frequency is
F_b = start + (end - start) (b + 1/2) / B. With the window w_k = 1/2 - 1/2 cos(2 pi (k + 1/2) / N),
the level is a_b = 2 |sum_k x_k w_k e^(-2 pi i F_b k / rate)| / sum_k w_k: a sine of amplitude A
at F_b reads A. Duration Averaging on: the mean of the levels of three windows, starting at
s0 - floor(N / 2), s0 and s0 + floor(N / 2). Band b stands h_b = Maximum Height min(a_b, 1).

Where the bands stand, t_b = (b + 1/2) / B along the way, in the drawing's own pixels:
- Path 0 and Use Polar Path off: on the line from Start Point to End Point (per cent of the
  drawing), at P + t_b (Q - P), facing n = (dy, -dx) for the line's direction (dx, dy), which
  is up for a line drawn left to right ((0, -1) when the two points meet).
- Use Polar Path on, Path 0: every band at Start Point, facing (cos a_b, sin a_b),
  a_b = -pi/2 + 2 pi t_b: from straight up, round clockwise on the screen.
- Path k: along mask k of the layer as document 19 flattens it (closed), at t_b of its length,
  facing the normal of the piece it falls on, as for the line. A mask that is not there, or is
  off, or has fewer than two points: nothing is drawn and EFFECT_PATH_MISSING is said.
Side A is the tip p + h n, Side B the tip p - h n.

The marks, each a straight piece drawn Thickness across:
- Digital: a piece from the band's foot to its tip (Side A, Side B) or from tip to tip (both).
- Analog Lines: one line through the tips of each side, Side A's first; band b's colours on the
  piece from tip b to tip b + 1, and round from the last tip to the first when the way is closed
  (polar, or a mask).
- Analog Dots: a dot (a piece of no length) at each tip, band by band, Side A's first.

Colours. Hue Interpolation H, in degrees, turns band b's Inside and Outside Color round the hue
(HSL, of the colour as written) by H u_b, with u_b = t_b; Dynamic Hue Phase on counts b from the
band with the largest level, u_b = (((b - m) mod B) + 1/2) / B; Color Symmetry on folds it,
u_b = 1 - |2 u_b - 1|, so the first and last match. H 0 leaves the colours as written.

Drawing, at a pixel centre X. A mark's distance d is the least from X to its pieces, its
colours those of the nearest piece (the first of equals). As Beam's (D-207, line_profile):
r = Thickness / 2, sw = max(2 r Softness / 100, 1), the covering
c = clamp((min(d + sw/2, r) - max(d - sw/2, -r)) / sw, 0, 1) and the colour
L = (1 - q) inside + q outside, both linear, q = clamp(d / r, 0, 1) (1 when r = 0).
Blend Overlapping Colors off: the marks laid one over the next in order, (L c, c) over what is
there. On: the covering is 1 - prod(1 - c) and the colour the mean of L weighted by c.
Composite On Original on: that over the layer; off: that alone. The layer never grows.

`audio_layer`, a layer id or ""; `start_point` and `end_point` -1000 to 1000 per cent, keyable,
(10, 50) and (90, 50) when added; `path` 0 to 1000, 0 when added; `use_polar_path` off;
`start_frequency` and `end_frequency` 1 to 20000 hertz, keyable, 20 and 2000; `frequency_bands`
1 to 4096, keyable, 64; `maximum_height` 0 to 10000 pixels, keyable, 200; `audio_duration` 1 to
30000 milliseconds, keyable, 90; `audio_offset` -30000 to 30000 milliseconds, keyable, 0;
`thickness` 0 to 10000 pixels, keyable, 3; `softness` 0 to 100, keyable, 50; `inside_color` white
and `outside_color` #3c8cff, read in small letters; `blend_overlapping_colors` off;
`hue_interpolation` -3600 to 3600 degrees, keyable, 0; `dynamic_hue_phase` and `color_symmetry`
off; `display_options` "digital", "analog_lines" or "analog_dots", digital; `side_options`
"side_a", "side_b" or "side_a_b", side_a_b; `duration_averaging` off; `composite` off. A draft
halves Maximum Height and Thickness, which are distances.

**This file never runs the build's code path.** It works in double precision on lists, straight
from the drawing's 8-bit values and from the samples it writes into the sound files.

Every case is a composition 16 by 10 at 24 frames a second holding Gradient's cel, the same
size, unmoved unless the case says, and a sound layer "sound" over all five frames. The sounds
are written here into `Fixtures/audio_spectrum/media/`, 4800 samples a second, a second long:
`tone.wav` (16-bit, one channel: 0.5 at 300 Hz and 0.25 at 1200 Hz), the same in 24-bit
(`tone24.wav`), 32-bit floating point (`tone_float.wav`) and 8-bit (`tone8.wav`), and
`stereo.wav` (16-bit, 0.6 at 600 Hz on the left and 0.3 at 900 Hz on the right). The expected
frames are in `Fixtures/audio_spectrum/expected_audio_spectrum.json`.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/audio_spectrum_reference.py
"""

import colorsys
import json
import math
import struct
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from adjust_reference import srgb_to_linear  # noqa: E402
from fxkey_reference import keyed, value_at, setting_json  # noqa: E402
import smooth_reference as S  # noqa: E402
import recolor_reference as RC  # noqa: E402
import checkerboard_reference as K  # noqa: E402
from drop_shadow_reference import frame  # noqa: E402
from gradient_reference import DRAWINGS  # noqa: E402
from mask_reference import flatten, points, mask_json  # noqa: E402

W, H = K.W, K.H
OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "audio_spectrum"
TOLERANCE = 2e-5  # document 25's default for a filter
RATE, FPS = 4800, 24
RANGES = {"start_point": (-1000, 1000), "end_point": (-1000, 1000), "path": (0, 1000),
          "start_frequency": (1, 20000), "end_frequency": (1, 20000),
          "frequency_bands": (1, 4096), "maximum_height": (0, 10000),
          "audio_duration": (1, 30000), "audio_offset": (-30000, 30000),
          "thickness": (0, 10000), "softness": (0, 100), "hue_interpolation": (-3600, 3600)}
WORDS = ("audio_layer", "use_polar_path", "inside_color", "outside_color",
         "blend_overlapping_colors", "dynamic_hue_phase", "color_symmetry", "display_options",
         "side_options", "duration_averaging", "composite")
NAMES = ("audio_layer", "start_point", "end_point", "path", "use_polar_path", "start_frequency",
         "end_frequency", "frequency_bands", "maximum_height", "audio_duration", "audio_offset",
         "thickness", "softness", "inside_color", "outside_color", "blend_overlapping_colors",
         "hue_interpolation", "dynamic_hue_phase", "color_symmetry", "display_options",
         "side_options", "duration_averaging", "composite")
VIOLET, BLUE, ORANGE = K.VIOLET, "#3c8cff", "#ff8000"


# --- the sounds -----------------------------------------------------------------------------

def tone(t):
    return 0.5 * math.sin(2 * math.pi * 300 * t) + 0.25 * math.sin(2 * math.pi * 1200 * t)


def quantized(v, bits):
    """The sample a file of `bits` holds for v, and what it reads back as."""
    if bits == 8:
        q = max(0, min(255, round(v * 127) + 128))
        return q, (q - 128) / 128
    if bits == 32:
        f = struct.unpack("<f", struct.pack("<f", v))[0]
        return f, f
    full = 1 << (bits - 1)
    q = max(-full, min(full - 1, round(v * (full - 1))))
    return q, q / full


def wav(channels, bits, float_=False):
    """A file's bytes and the samples it reads back as, one list per channel."""
    raw, read = bytearray(), [[] for _ in channels]
    for i in range(RATE):
        for ch, f in enumerate(channels):
            q, r = quantized(f(i / RATE), bits)
            read[ch].append(r)
            if float_:
                raw += struct.pack("<f", q)
            elif bits == 8:
                raw += bytes([q])
            else:
                raw += int(q).to_bytes(bits // 8, "little", signed=True)
    n, align = len(channels), len(channels) * bits // 8
    fmt = struct.pack("<HHIIHH", 3 if float_ else 1, n, RATE, RATE * align, align, bits)
    body = b"WAVE" + b"fmt " + struct.pack("<I", 16) + fmt + b"data" + \
        struct.pack("<I", len(raw)) + bytes(raw)
    return b"RIFF" + struct.pack("<I", len(body)) + body, read


SOUNDS = {
    "tone": wav([tone], 16),
    "tone24": wav([tone], 24),
    "tone_float": wav([tone], 32, float_=True),
    "tone8": wav([tone], 8),
    "stereo": wav([lambda t: 0.6 * math.sin(2 * math.pi * 600 * t),
                   lambda t: 0.3 * math.sin(2 * math.pi * 900 * t)], 16),
}


def sample(snd, i, gain, channel=None):
    """The sound's sample i, its channels' mean (or one channel), times the gain."""
    read = SOUNDS[snd["file"]][1]
    if i < 0 or i >= len(read[0]):
        return 0.0
    if channel is None:
        v = sum(ch[i] for ch in read) / len(read)
    else:
        v = read[min(channel, len(read) - 1)][i]
    return v * gain


def window_start(snd, frame_no, offset):
    """The window's first sample at a composition frame, or None for silence."""
    if frame_no < snd["in"] or frame_no >= snd["out"]:
        return None
    n = frame_no - snd["in"]
    return n * RATE // FPS + math.floor(offset * RATE / 1000 + 0.5)


def levels(snd, frame_no, n):
    """a_b for every band, from the settings `n` held at the frame."""
    bands = int(math.floor(n["frequency_bands"]))
    s0 = None if snd is None else window_start(snd, frame_no, n["audio_offset"])
    if s0 is None:
        return [0.0] * bands
    count = max(1, math.floor(n["audio_duration"] * RATE / 1000 + 0.5))
    w = [0.5 - 0.5 * math.cos(2 * math.pi * (k + 0.5) / count) for k in range(count)]
    sw = sum(w)
    gain = 10 ** (snd["gain"] / 20)
    starts = [s0 - count // 2, s0, s0 + count // 2] if n["duration_averaging"] == "on" else [s0]
    out = []
    for b in range(bands):
        f = n["start_frequency"] + (n["end_frequency"] - n["start_frequency"]) * (b + 0.5) / bands
        total = 0.0
        for s in starts:
            re = im = 0.0
            for k in range(count):
                x = sample(snd, s + k, gain) * w[k]
                re += x * math.cos(2 * math.pi * f * k / RATE)
                im -= x * math.sin(2 * math.pi * f * k / RATE)
            total += 2 * math.hypot(re, im) / sw
        out.append(total / len(starts))
    return out


# --- where the bands stand ------------------------------------------------------------------

def per_cent(p):
    return (p[0] / 100 * W, p[1] / 100 * H)


def facing(dx, dy):
    length = math.hypot(dx, dy)
    return (0.0, -1.0) if length == 0 else (dy / length, -dx / length)


def along(outline, u):
    """The point u of the way round a closed outline, and the normal of its piece."""
    pieces = [(outline[i], outline[(i + 1) % len(outline)]) for i in range(len(outline))]
    total = sum(math.hypot(b[0] - a[0], b[1] - a[1]) for a, b in pieces)
    if total == 0:
        return outline[0], (0.0, -1.0)
    s, walked = u * total, 0.0
    for a, b in pieces:
        length = math.hypot(b[0] - a[0], b[1] - a[1])
        if length > 0 and walked + length >= s:
            v = (s - walked) / length
            return ((a[0] + v * (b[0] - a[0]), a[1] + v * (b[1] - a[1])),
                    facing(b[0] - a[0], b[1] - a[1]))
        walked += length
    a, b = next((a, b) for a, b in reversed(pieces) if a != b)
    return b, facing(b[0] - a[0], b[1] - a[1])


def feet(n, bands, outline):
    """Each band's foot and normal, and whether the way is closed."""
    t = [(b + 0.5) / bands for b in range(bands)]
    if outline is not None:
        return [along(outline, u) for u in t], True
    p = per_cent(n["start_point"])
    if n["use_polar_path"] == "on":
        return [(p, (math.cos(-math.pi / 2 + 2 * math.pi * u),
                     math.sin(-math.pi / 2 + 2 * math.pi * u))) for u in t], True
    q = per_cent(n["end_point"])
    normal = facing(q[0] - p[0], q[1] - p[1])
    return [((p[0] + u * (q[0] - p[0]), p[1] + u * (q[1] - p[1])), normal) for u in t], False


def turned(hex_, degrees):
    rgb = [v / 255 for v in RC.hex_color(hex_.lower())]
    if degrees != 0:
        h, l_, s = colorsys.rgb_to_hls(*rgb)
        rgb = colorsys.hls_to_rgb((h + degrees / 360) % 1.0, l_, s)
    return [srgb_to_linear(v) for v in rgb]


def colours(n, a):
    """Each band's inside and outside colours, linear."""
    bands, hue = len(a), n["hue_interpolation"]
    m = a.index(max(a)) if n["dynamic_hue_phase"] == "on" and a else 0
    out = []
    for b in range(bands):
        u = (((b - m) % bands) + 0.5) / bands
        if n["color_symmetry"] == "on":
            u = 1 - abs(2 * u - 1)
        turn = hue * u
        out.append((turned(n["inside_color"], turn), turned(n["outside_color"], turn)))
    return out


def marks(spots, closed, heights, cols, display, side):
    """The marks in drawing order: each a list of pieces ((x0, y0), (x1, y1), inside, outside)."""
    sides = {"side_a": (1,), "side_b": (-1,), "side_a_b": (1, -1)}[side]
    tip = lambda b, s: (spots[b][0][0] + s * heights[b] * spots[b][1][0],  # noqa: E731
                        spots[b][0][1] + s * heights[b] * spots[b][1][1])
    out, bands = [], len(spots)
    if display == "digital":
        for b in range(bands):
            ends = (tip(b, -1), tip(b, 1)) if len(sides) == 2 else (spots[b][0], tip(b, sides[0]))
            out.append([(ends[0], ends[1]) + cols[b]])
    elif display == "analog_lines":
        for s in sides:
            pieces = [(tip(b, s), tip(b + 1, s)) + cols[b] for b in range(bands - 1)]
            if closed and bands > 1:
                pieces.append((tip(bands - 1, s), tip(0, s)) + cols[bands - 1])
            if bands == 1:
                pieces = [(tip(0, s), tip(0, s)) + cols[0]]
            out.append(pieces)
    else:
        for b in range(bands):
            for s in sides:
                out.append([(tip(b, s), tip(b, s)) + cols[b]])
    return out


# --- drawing --------------------------------------------------------------------------------

def to_piece(a, b, x, y):
    dx, dy = b[0] - a[0], b[1] - a[1]
    l2 = dx * dx + dy * dy
    t = 0.0 if l2 == 0 else min(max(((x - a[0]) * dx + (y - a[1]) * dy) / l2, 0.0), 1.0)
    return math.hypot(x - a[0] - t * dx, y - a[1] - t * dy)


def profile(d, r, softness):
    sw = max(2 * r * softness / 100, 1)
    c = min(max((min(d + sw / 2, r) - max(d - sw / 2, -r)) / sw, 0.0), 1.0)
    q = 1.0 if r == 0 else min(d / r, 1.0)
    return c, q


def drawn(ms, x, y, thickness, softness, blend):
    """The marks at (x, y), premultiplied (r, g, b, a)."""
    r = thickness / 2
    acc, total, weight, clear = [0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 0.0, 1.0
    a = 0.0
    for m in ms:
        best = None
        for p0, p1, ins, outs in m:
            d = to_piece(p0, p1, x, y)
            if best is None or d < best[0]:
                best = (d, ins, outs)
        d, ins, outs = best
        c, q = profile(d, r, softness)
        L = [(1 - q) * ins[k] + q * outs[k] for k in range(3)]
        if blend:
            total = [total[k] + L[k] * c for k in range(3)]
            weight += c
            clear *= 1 - c
        else:
            acc = [L[k] * c + acc[k] * (1 - c) for k in range(3)]
            a = c + a * (1 - c)
    if blend:
        a = 1 - clear
        acc = [total[k] / weight * a if weight > 0 else 0.0 for k in range(3)]
    return acc + [a]


def laid(layer, ms, thickness, softness, blend, how):
    """The marks on the layer: "over" it, "alone", or "add"ed to it (Audio Waveform's)."""
    px = []
    for j in range(layer["h"]):
        for i in range(layer["w"]):
            p = layer["px"][j * layer["w"] + i]
            m = drawn(ms, layer["left"] + i + 0.5, layer["top"] + j + 0.5, thickness, softness,
                      blend)
            if how == "alone":
                px.append(m)
            elif how == "add":
                px.append([p[k] + m[k] for k in range(3)] + [min(p[3] + m[3], 1.0)])
            else:
                px.append([m[k] + p[k] * (1 - m[3]) for k in range(4)])
    return dict(layer, px=px)


# --- the cases ------------------------------------------------------------------------------

def sound(file="tone", in_=0, gain=0.0):
    return {"file": file, "in": in_, "out": 5, "gain": gain}


def case(audio_layer="", start_point=(10, 50), end_point=(90, 50), path=0, use_polar_path="off",
         start_frequency=20, end_frequency=2000, frequency_bands=64, maximum_height=200,
         audio_duration=90, audio_offset=0, thickness=3, softness=50, inside_color="#ffffff",
         outside_color=BLUE, blend_overlapping_colors="off", hue_interpolation=0,
         dynamic_hue_phase="off", color_symmetry="off", display_options="digital",
         side_options="side_a_b", duration_averaging="off", composite="off", snd=None,
         masks=(), shift=0, tile=False):
    c = {k: v for k, v in locals().items() if k in NAMES}
    c.update({"drawing": "cel", "snd": snd or sound(), "masks": list(masks), "shift": shift,
              "tile": tile})
    return c


def bars(**kw):
    """The small frame's own spectrum: four bands at 300, 600, 900 and 1200 Hz on a line across
    the frame 8 pixels down, 8 pixels tall at most, Side A, 2 thick and sharp, alone."""
    base = dict(audio_layer="sound", start_point=(10, 80), end_point=(90, 80),
                start_frequency=150, end_frequency=1350, frequency_bands=4, maximum_height=8,
                thickness=2, softness=0, side_options="side_a")
    base.update(kw)
    return case(**base)


BOX = points((2, 2), (14, 2), (14, 8), (2, 8))


def held(c, k, frame_no):
    lo, hi = RANGES[k]
    v = value_at(c[k], frame_no)
    if isinstance(v, (list, tuple)):
        return [min(hi, max(lo, e)) for e in v]
    return min(hi, max(lo, v))


def settings(c, frame_no):
    n = {k: held(c, k, frame_no) for k in RANGES}
    n.update({k: c[k] for k in WORDS})
    return n


def outline_of(c, n):
    """The mask Path names, flattened, or None (Path 0); "missing" when it is not there."""
    k = int(math.floor(n["path"]))
    if k == 0:
        return None
    if k > len(c["masks"]):
        return "missing"
    return flatten(c["masks"][k - 1]["points"])


def heard(c):
    return c["snd"] if c["audio_layer"] == "sound" else None


def render(c, frame_no):
    n = settings(c, frame_no)
    layer = K.layer_of(c)
    outline = outline_of(c, n)
    if outline == "missing" or c["audio_layer"] not in ("", "sound"):
        return frame(layer, c["shift"])
    a = levels(heard(c), frame_no, n)
    heights = [n["maximum_height"] * min(v, 1.0) for v in a]
    spots, closed = feet(n, len(a), outline)
    ms = marks(spots, closed, heights, colours(n, a), c["display_options"], c["side_options"])
    how = "over" if c["composite"] == "on" else "alone"
    out = laid(layer, ms, n["thickness"], n["softness"], c["blend_overlapping_colors"] == "on",
               how)
    return frame(out, c["shift"])


def plain(c):
    return frame(K.layer_of(c), c["shift"])


EIGHT = dict(frequency_bands=8, start_frequency=225, end_frequency=1425)  # 300, 450 .. 1350 Hz
ROOM = dict(start_point=(50, 50), use_polar_path="on", maximum_height=4, **EIGHT)

CASES = {
    "FX-ASPEC-001": ("The settings as they start, no Audio Layer: silence, so 64 dots of Side A "
                     "and Side B together, 3 thick, along the line through the middle, alone.",
                     case(), [0]),
    "FX-ASPEC-002": ("Four bands at 300, 600, 900 and 1200 Hz, 8 tall at most, Side A, 2 thick, "
                     "sharp, alone: the 300 Hz bar half as tall as it can be (4), the 1200 Hz bar "
                     "a quarter (2), the two between all but nothing.", bars(), [0, 2]),
    "FX-ASPEC-003": ("FX-ASPEC-002 with Composite On Original on: over the cel.",
                     bars(composite="on"), [0]),
    "FX-ASPEC-004": ("Side B: the bars hang down.", bars(side_options="side_b"), [0]),
    "FX-ASPEC-005": ("Side A & B: each bar both ways.", bars(side_options="side_a_b"), [0]),
    "FX-ASPEC-006": ("Analog Lines: one line through the bars' tips.",
                     bars(display_options="analog_lines"), [0]),
    "FX-ASPEC-007": ("Analog Dots: a dot at each tip.", bars(display_options="analog_dots"), [0]),
    "FX-ASPEC-008": ("Thickness 3, softness 100.", bars(thickness=3, softness=100), [0]),
    "FX-ASPEC-009": ("Orange inside, violet outside, Hue Interpolation 180, thickness 3: each "
                     "band's colours turned further round the hue.",
                     bars(thickness=3, inside_color=ORANGE, outside_color=VIOLET,
                          hue_interpolation=180), [0]),
    "FX-ASPEC-010": ("FX-ASPEC-009 with Color Symmetry on: the first and last bands match.",
                     bars(thickness=3, inside_color=ORANGE, outside_color=VIOLET,
                          hue_interpolation=180, color_symmetry="on"), [0]),
    "FX-ASPEC-011": ("FX-ASPEC-009 with Dynamic Hue Phase on: the turn starts at the loudest "
                     "band, 300 Hz, the first, so the frame is FX-ASPEC-009's.",
                     bars(thickness=3, inside_color=ORANGE, outside_color=VIOLET,
                          hue_interpolation=180, dynamic_hue_phase="on"), [0]),
    "FX-ASPEC-012": ("Eight bands at 300, 450 .. 1350 Hz, thickness 3, Side A & B, Blend Overlapping Colors on, orange "
                     "inside: neighbouring bars overlap and their colours are blended.",
                     bars(**EIGHT, thickness=3, side_options="side_a_b",
                          inside_color=ORANGE, blend_overlapping_colors="on"), [0]),
    "FX-ASPEC-013": ("FX-ASPEC-012 with Blend Overlapping Colors off: each bar laid over the "
                     "last.", bars(**EIGHT, thickness=3, side_options="side_a_b",
                                   inside_color=ORANGE), [0]),
    "FX-ASPEC-014": ("Duration Averaging on: three windows half a window apart, averaged.",
                     bars(duration_averaging="on"), [0, 2]),
    "FX-ASPEC-015": ("Audio Offset 50 ms: the window starts 240 samples later.",
                     bars(audio_offset=50), [0, 2]),
    "FX-ASPEC-016": ("Audio Duration 20 ms: a window of 96 samples, too short to keep 300 Hz "
                     "from 600 Hz, so the levels spread.", bars(audio_duration=20), [0]),
    "FX-ASPEC-017": ("Use Polar Path on about the middle, eight bands at 300, 450 .. 1350 Hz, 4 tall at most: the bars "
                     "stand out round the centre from straight up, clockwise.",
                     bars(**ROOM), [0]),
    "FX-ASPEC-018": ("FX-ASPEC-017 as Analog Lines: a closed line round the centre.",
                     bars(display_options="analog_lines", **ROOM), [0]),
    "FX-ASPEC-019": ("Path 1, a mask (mode None) round the box from (2, 2) to (14, 8), eight "
                     "bands, 3 tall at most, Side A: the bars stand outward round the box.",
                     bars(path=1, masks=[{"points": BOX, "mode": "none"}], maximum_height=3,
                          **EIGHT), [0]),
    "FX-ASPEC-020": ("Path 2 with only one mask: none to draw along, so nothing is drawn and "
                     "EFFECT_PATH_MISSING is said; the cel as it was.",
                     bars(path=2, masks=[{"points": BOX, "mode": "none"}], composite="on"), [0]),
    "FX-ASPEC-021": ("Audio Layer \"ghost\", not in the composition: EFFECT_LAYER_MISSING is "
                     "said and the cel is as it was.", bars(audio_layer="ghost", composite="on"),
                     [0]),
    "FX-ASPEC-022": ("Audio Layer \"art\", the cel itself, which holds no sound: "
                     "EFFECT_SOUND_MISSING is said and the cel is as it was.",
                     bars(audio_layer="art", composite="on"), [0]),
    "FX-ASPEC-023": ("The stereo file, bands at 600 and 900 Hz among the four: its two channels "
                     "averaged, 0.3 at 600 Hz and 0.15 at 900 Hz.",
                     bars(snd=sound("stereo")), [0]),
    "FX-ASPEC-024": ("The 24-bit file: as FX-ASPEC-002 within the tolerance.",
                     bars(snd=sound("tone24")), [0]),
    "FX-ASPEC-025": ("The 32-bit floating point file.", bars(snd=sound("tone_float")), [0]),
    "FX-ASPEC-026": ("The 8-bit file.", bars(snd=sound("tone8")), [0]),
    "FX-ASPEC-027": ("The sound layer starting at frame 2: frames 0 and 1 silent, frame 4 its "
                     "own frame 2.", bars(snd=sound(in_=2)), [0, 4]),
    "FX-ASPEC-028": ("The sound layer at -6.0206 dB, half as loud: the bars half as tall.",
                     bars(snd=sound(gain=-6.020599913279624)), [0]),
    "FX-ASPEC-029": ("Maximum Height keyed from 0 at frame 0 to 8 at frame 4, linear.",
                     bars(maximum_height=keyed((0, 0), (4, 8))), [0, 2, 4]),
    "FX-ASPEC-030": ("FX-ASPEC-002 moved three pixels right: the bars move with the layer.",
                     bars(shift=3), [0]),
    "FX-ASPEC-031": ("After a Motion Tile that grows the layer: the points are the drawing's "
                     "own, so the frame is FX-ASPEC-002's.", bars(tile=True), [0]),
    "FX-ASPEC-032": ("One band, 150 to 450 Hz: one bar at 300 Hz in the middle of the line.",
                     bars(frequency_bands=1, end_frequency=450), [0]),
    "FX-ASPEC-033": ("Start Frequency 1350, End Frequency 150: the bands in the other order, "
                     "1200 Hz first.", bars(start_frequency=1350, end_frequency=150), [0]),
    "FX-ASPEC-034": ("Thickness 0: nothing drawn, the cel as it was.",
                     bars(thickness=0, composite="on"), [0]),
}

INVALID = {
    "FX-ASPEC-035": ("Frequency Bands 0, below 1.", case(frequency_bands=0)),
    "FX-ASPEC-036": ("Frequency Bands 4097, above 4096.", case(frequency_bands=4097)),
    "FX-ASPEC-037": ("Start Frequency 0, below 1.", case(start_frequency=0)),
    "FX-ASPEC-038": ("End Frequency 20001, above 20000.", case(end_frequency=20001)),
    "FX-ASPEC-039": ("Maximum Height -1, below 0.", case(maximum_height=-1)),
    "FX-ASPEC-040": ("Audio Duration 0, below 1.", case(audio_duration=0)),
    "FX-ASPEC-041": ("Audio Offset 30001, above 30000.", case(audio_offset=30001)),
    "FX-ASPEC-042": ("Thickness -1, below 0.", case(thickness=-1)),
    "FX-ASPEC-043": ("Softness 101, above 100.", case(softness=101)),
    "FX-ASPEC-044": ("Hue Interpolation 3601, above 3600.", case(hue_interpolation=3601)),
    "FX-ASPEC-045": ("Path -1, below 0.", case(path=-1)),
    "FX-ASPEC-046": ("Start Point 1001 per cent across, above 1000.", case(start_point=(1001, 50))),
    "FX-ASPEC-047": ("Display Options \"bars\", not digital, analog_lines or analog_dots.",
                     case(display_options="bars")),
    "FX-ASPEC-048": ("Side Options \"side_c\".", case(side_options="side_c")),
    "FX-ASPEC-049": ("Composite \"yes\", not on or off.", case(composite="yes")),
    "FX-ASPEC-050": ("Inside colour \"#12345\", not six hex digits.", case(inside_color="#12345")),
    "FX-ASPEC-051": ("Use Polar Path \"maybe\".", case(use_polar_path="maybe")),
    "FX-ASPEC-052": ("Audio Layer 5, a number, not a layer's name.", case(audio_layer=5)),
    "FX-ASPEC-053": ("Frequency Bands keyed to 4097 at frame 4, above 4096.",
                     case(frequency_bands=keyed((0, 64), (4, 4097)))),
}


# --- the project files ----------------------------------------------------------------------

def project_json(fx, c, type_id="core.audio_spectrum", names=NAMES, words=WORDS):
    p = K.project_json(fx, dict(K.case(), shift=c["shift"], tile=c["tile"]))
    snd = c["snd"]
    p["assets"].append({"id": "asset-sound", "kind": "audio", "name": "sound",
                        "path": f"media/{snd['file']}.wav"})
    comp = p["compositions"][0]
    layer = comp["layers"][0]
    if c["masks"]:
        items = [(k, v) for k, v in layer.items() if k != "mask"]
        at = [k for k, _ in items].index("matte")
        items.insert(at, ("masks", [mask_json(m, i + 1) for i, m in enumerate(c["masks"])]))
        layer = dict(items)
        comp["layers"][0] = layer
    effects = layer["effects"]
    effects[-1] = {"instance_id": effects[-1]["instance_id"], "type_id": type_id,
                   "enabled": True,
                   "parameters": {k: (c[k] if k in words else setting_json(c[k]))
                                  for k in names}}
    comp["layer_order"] = ["art", "sound"]
    comp["layers"].append({"id": "sound", "kind": "audio", "name": "sound",
                           "asset_id": "asset-sound", "enabled": True, "locked": False,
                           "in_frame": snd["in"], "out_frame": snd["out"],
                           "source_offset_frames": 0, "gain_db": snd["gain"], "transform": {}})
    return p


def write(fx, c, out=OUT, **kw):
    name = f"{fx.lower().replace('-', '_')}.json"
    (out / name).write_text(json.dumps(project_json(fx, c, **kw), indent=2) + "\n",
                            encoding="utf-8")
    return name


WARNINGS = {"FX-ASPEC-020": "EFFECT_PATH_MISSING", "FX-ASPEC-021": "EFFECT_LAYER_MISSING",
            "FX-ASPEC-022": "EFFECT_SOUND_MISSING"}


def media(out):
    (out / "media").mkdir(parents=True, exist_ok=True)
    for name, pixels in DRAWINGS.items():
        (out / "media" / f"{name}.png").write_bytes(S.png(pixels))
    for name, (raw, _) in SOUNDS.items():
        (out / "media" / f"{name}.wav").write_bytes(raw)


def main():
    media(OUT)
    expected = {"tolerance": TOLERANCE, "width": W, "height": H, "cases": {}}
    for fx, (says, c, frames) in CASES.items():
        rendered = {str(f): render(c, f) for f in frames}
        expected["cases"][fx] = {"says": says, "project": write(fx, c), "frames": rendered}
        if fx in WARNINGS:
            expected["cases"][fx]["warning"] = WARNINGS[fx]
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
    (OUT / "expected_audio_spectrum.json").write_text(json.dumps(expected, indent=1) + "\n",
                                                      encoding="utf-8")
    check(expected)


def check(expected):
    """The claims the cases make, checked on the numbers just worked."""
    c = {fx: v["frames"] for fx, v in expected["cases"].items()}
    at = lambda x, y: y * W + x  # noqa: E731
    near = lambda p, q, e=1e-12: all(abs(a - b) < e for a, b in zip(p, q))  # noqa: E731
    clear = [0.0, 0.0, 0.0, 0.0]
    art = plain(case())
    # the levels: a sine of amplitude A at a band's frequency reads about A
    n = settings(bars(), 0)
    a = levels(sound(), 0, n)
    assert abs(a[0] - 0.5) < 2e-3 and abs(a[3] - 0.25) < 2e-3 and a[1] < 2e-3 and a[2] < 2e-3, a
    st = levels(sound("stereo"), 0, settings(bars(), 0))
    assert abs(st[1] - 0.3) < 2e-3 and abs(st[2] - 0.15) < 2e-3, st
    silent = c["FX-ASPEC-001"]["0"]
    assert silent[at(8, 5)][3] > 0.9 and near(silent[at(8, 0)], clear)
    two, over = c["FX-ASPEC-002"]["0"], c["FX-ASPEC-003"]["0"]
    assert all(near(over[i], [two[i][k] + art[i][k] * (1 - two[i][3]) for k in range(4)])
               for i in range(W * H))
    # band 0 stands at x 3.2 from y 8 up to 6: pixel (3, 6) is inside it, (3, 3) above it
    assert two[at(3, 6)][3] > 0.5 and two[at(3, 2)][3] == 0 and two[at(12, 6)][3] > 0.5
    assert two[at(12, 4)][3] == 0 and two[at(12, 5)][3] > 0.5
    down = c["FX-ASPEC-004"]["0"]
    assert down[at(3, 9)][3] > 0.5 and down[at(3, 6)][3] == 0
    both = c["FX-ASPEC-005"]["0"]
    assert both[at(3, 9)][3] > 0.5 and both[at(3, 6)][3] > 0.5
    assert c["FX-ASPEC-006"]["0"] != two and c["FX-ASPEC-007"]["0"] != two
    assert c["FX-ASPEC-011"]["0"] == c["FX-ASPEC-009"]["0"] != c["FX-ASPEC-010"]["0"]
    assert c["FX-ASPEC-012"]["0"] != c["FX-ASPEC-013"]["0"]
    assert c["FX-ASPEC-014"]["0"] != two and c["FX-ASPEC-016"]["0"] != two
    assert c["FX-ASPEC-018"]["0"] != c["FX-ASPEC-017"]["0"]
    polar = c["FX-ASPEC-017"]["0"]
    assert polar[at(8, 3)][3] > 0 and near(polar[at(0, 0)], clear)
    box = c["FX-ASPEC-019"]["0"]
    assert box[at(8, 5)][3] == 0 and any(box[at(x, 1)][3] > 0 for x in range(W))
    for fx in WARNINGS:
        assert c[fx]["0"] == art
    assert near(c["FX-ASPEC-024"]["0"][at(3, 6)], two[at(3, 6)], 1e-3)
    assert near(c["FX-ASPEC-025"]["0"][at(3, 6)], two[at(3, 6)], 1e-3)
    assert near(c["FX-ASPEC-026"]["0"][at(3, 6)], two[at(3, 6)], 2e-2)
    late = c["FX-ASPEC-027"]
    assert late["4"] == c["FX-ASPEC-002"]["2"] and late["0"] != two
    half = levels(sound(gain=-6.020599913279624), 0, n)
    assert all(abs(h - v / 2) < 1e-9 for h, v in zip(half, a))
    grow = c["FX-ASPEC-029"]
    # frame 4's window holds the same tone as frame 0's but for the odd last bit of a sample
    assert all(near(p, q, 1e-5) for p, q in zip(grow["4"], two)) and grow["0"] != two
    moved = c["FX-ASPEC-030"]["0"]
    assert all(moved[at(x, y)] == two[at(x - 3, y)] for x in range(3, W) for y in range(H))
    assert c["FX-ASPEC-031"]["0"] == two
    one = c["FX-ASPEC-032"]["0"]
    assert one[at(8, 6)][3] > 0.5 and near(one[at(3, 6)], clear)
    flip = c["FX-ASPEC-033"]["0"]
    assert flip[at(3, 7)][3] > 0.5 and flip[at(12, 6)][3] > 0.5 and flip[at(3, 4)][3] == 0
    assert c["FX-ASPEC-034"]["0"] == art
    for fx in INVALID:
        assert c[fx]["0"] == c[fx]["4"] == art
    print("checked")


if __name__ == "__main__":
    if sys.argv[1:] == ["--check"]:
        check(json.loads((OUT / "expected_audio_spectrum.json").read_text(encoding="utf-8")))
    else:
        main()
